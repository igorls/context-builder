//! JavaScript language support for tree-sitter.

use tree_sitter::{Parser, Tree};

use crate::tree_sitter::language_support::{
    CodeStructure, LanguageSupport, Signature, SignatureKind, Visibility,
    slice_signature_before_body,
};

pub struct JavaScriptSupport;

impl JavaScriptSupport {
    fn get_language() -> tree_sitter::Language {
        tree_sitter_javascript::LANGUAGE.into()
    }
}

impl LanguageSupport for JavaScriptSupport {
    fn file_extensions(&self) -> &[&'static str] {
        &["js", "mjs", "cjs", "jsx"]
    }

    fn parse(&self, source: &str) -> Option<Tree> {
        let mut parser = Parser::new();
        parser.set_language(&Self::get_language()).ok()?;
        parser.parse(source, None)
    }

    fn extract_signatures(&self, source: &str, visibility: Visibility) -> Vec<Signature> {
        let tree = match self.parse(source) {
            Some(t) => t,
            None => return Vec::new(),
        };

        let root = tree.root_node();
        let mut signatures = Vec::new();

        self.extract_signatures_from_node(source, &root, visibility, &mut signatures);

        signatures.sort_by_key(|s| s.line_number);
        signatures
    }

    fn extract_structure(&self, source: &str) -> CodeStructure {
        let tree = match self.parse(source) {
            Some(t) => t,
            None => return CodeStructure::default(),
        };

        let root = tree.root_node();
        let mut structure = CodeStructure {
            total_lines: source.lines().count(),
            ..Default::default()
        };

        self.extract_structure_from_node(&root, &mut structure);
        structure
    }

    fn find_truncation_point(&self, source: &str, max_bytes: usize) -> usize {
        if source.len() <= max_bytes {
            return source.len();
        }

        let tree = match self.parse(source) {
            Some(t) => t,
            None => return max_bytes,
        };

        let root = tree.root_node();
        let mut best_end = 0;

        let mut cursor = root.walk();
        self.find_best_boundary(&mut cursor, max_bytes, &mut best_end);
        drop(cursor);

        if best_end == 0 { max_bytes } else { best_end }
    }
}

impl JavaScriptSupport {
    fn extract_signatures_from_node(
        &self,
        source: &str,
        node: &tree_sitter::Node,
        _visibility: Visibility,
        signatures: &mut Vec<Signature>,
    ) {
        match node.kind() {
            "function_declaration" => {
                if let Some(sig) = self.extract_function_signature(source, node) {
                    signatures.push(sig);
                }
            }
            "class_declaration" => {
                if let Some(sig) = self.extract_class_signature(source, node) {
                    signatures.push(sig);
                }
            }
            "variable_declaration" | "lexical_declaration" => {
                self.extract_variable_declarations(source, node, signatures);
            }
            "export_statement" => {
                // Extract signatures from exported declarations.
                // Return early — do NOT recurse into children of export_statement,
                // because extract_export_signatures already walks them.
                self.extract_export_signatures(source, node, signatures);
                return;
            }
            _ => {}
        }

        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            self.extract_signatures_from_node(source, &child, _visibility, signatures);
        }
    }

    fn extract_structure_from_node(&self, node: &tree_sitter::Node, structure: &mut CodeStructure) {
        match node.kind() {
            "function_declaration" | "generator_function_declaration" | "function_expression" => {
                structure.functions += 1;
            }
            "class_declaration" | "class_expression" => {
                structure.classes += 1;
            }
            "import_statement" => {
                structure.imports.push("import".to_string());
            }
            "export_statement" => {
                structure.exports.push("export".to_string());
            }
            _ => {}
        }

        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            self.extract_structure_from_node(&child, structure);
        }
    }

    fn extract_function_signature(
        &self,
        source: &str,
        node: &tree_sitter::Node,
    ) -> Option<Signature> {
        let name = self.find_child_text(node, "identifier", source)?;
        let params = self.find_child_text(node, "formal_parameters", source);

        // Use byte-slicing to preserve async, generator*, and complete parameter lists
        let full_sig = slice_signature_before_body(source, node, &["statement_block"])
            .unwrap_or_else(|| match &params {
                Some(p) => format!("function {}({})", name, p),
                None => format!("function {}()", name),
            });

        Some(Signature {
            kind: SignatureKind::Function,
            name,
            params,
            return_type: None,
            visibility: Visibility::All,
            line_number: node.start_position().row + 1,
            full_signature: full_sig,
        })
    }

    fn extract_class_signature(&self, source: &str, node: &tree_sitter::Node) -> Option<Signature> {
        let name = self.find_child_text(node, "identifier", source)?;

        // Use byte-slicing to preserve `extends` and other modifiers
        let full_sig = slice_signature_before_body(source, node, &["class_body"])
            .unwrap_or_else(|| format!("class {}", name));

        Some(Signature {
            kind: SignatureKind::Class,
            name,
            params: None,
            return_type: None,
            visibility: Visibility::All,
            line_number: node.start_position().row + 1,
            full_signature: full_sig,
        })
    }

    fn extract_variable_declarations(
        &self,
        source: &str,
        node: &tree_sitter::Node,
        signatures: &mut Vec<Signature>,
    ) {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if child.kind() == "variable_declarator" {
                // Check if this is an arrow function or regular function assignment
                let mut inner_cursor = child.walk();
                let fn_node = child
                    .children(&mut inner_cursor)
                    .find(|c| c.kind() == "arrow_function" || c.kind() == "function");

                if let Some(fn_child) = fn_node {
                    // Navigate INTO the arrow_function/function to find its body.
                    // statement_block is a child of arrow_function, NOT of variable_declarator.
                    let body_start = {
                        let mut fn_cursor = fn_child.walk();
                        fn_child
                            .children(&mut fn_cursor)
                            .find(|c| c.kind() == "statement_block")
                            .map(|body| body.start_byte())
                    };

                    let full_signature = if let Some(body_start) = body_start {
                        // Slice from the parent node (lexical_declaration) to preserve `const`/`export`,
                        // down to the body start
                        source[node.start_byte()..body_start].trim_end().to_string()
                    } else {
                        // Expression-body arrow: `const add = (a, b) => a + b`
                        // Slice from parent through the `=>` token
                        let mut fn_cursor2 = fn_child.walk();
                        let arrow_end = fn_child
                            .children(&mut fn_cursor2)
                            .find(|c| c.kind() == "=>")
                            .map(|arrow| arrow.end_byte());

                        if let Some(end) = arrow_end {
                            source[node.start_byte()..end].trim_end().to_string()
                        } else {
                            // Last resort: use declarator text only (name = params)
                            source[child.start_byte()..fn_child.start_byte()]
                                .trim_end()
                                .to_string()
                        }
                    };

                    let name = self
                        .find_child_text(&child, "identifier", source)
                        .unwrap_or_default();

                    signatures.push(Signature {
                        kind: SignatureKind::Function,
                        name,
                        params: None, // Captured via byte-slicing
                        return_type: None,
                        visibility: Visibility::All,
                        line_number: child.start_position().row + 1,
                        full_signature,
                    });
                } else if let Some(name) = self.find_child_text(&child, "identifier", source) {
                    let full_signature = format!("const {}", &name);
                    signatures.push(Signature {
                        kind: SignatureKind::Constant,
                        name,
                        params: None,
                        return_type: None,
                        visibility: Visibility::All,
                        line_number: child.start_position().row + 1,
                        full_signature,
                    });
                }
            }
        }
    }

    fn extract_export_signatures(
        &self,
        source: &str,
        node: &tree_sitter::Node,
        signatures: &mut Vec<Signature>,
    ) {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if child.kind() == "function_declaration" {
                if let Some(sig) = self.extract_function_signature(source, &child) {
                    signatures.push(sig);
                }
            } else if child.kind() == "class_declaration"
                && let Some(sig) = self.extract_class_signature(source, &child)
            {
                signatures.push(sig);
            } else if child.kind() == "lexical_declaration"
                || child.kind() == "variable_declaration"
            {
                // Capture exported arrow functions: export const foo = () => {}
                self.extract_variable_declarations(source, &child, signatures);
            }
        }
    }

    fn find_child_text(
        &self,
        node: &tree_sitter::Node,
        kind: &str,
        source: &str,
    ) -> Option<String> {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if child.kind() == kind {
                return Some(source[child.start_byte()..child.end_byte()].to_string());
            }
            let mut nested_cursor = child.walk();
            for nested in child.children(&mut nested_cursor) {
                if nested.kind() == kind {
                    return Some(source[nested.start_byte()..nested.end_byte()].to_string());
                }
            }
        }
        None
    }

    fn find_best_boundary(
        &self,
        cursor: &mut tree_sitter::TreeCursor,
        max_bytes: usize,
        best_end: &mut usize,
    ) {
        loop {
            let node = cursor.node();
            let end_byte = node.end_byte();

            if end_byte <= max_bytes && end_byte > *best_end {
                let is_item = matches!(
                    node.kind(),
                    "function_declaration"
                        | "class_declaration"
                        | "method_definition"
                        | "export_statement"
                        | "variable_declaration"
                        | "lexical_declaration"
                );
                if is_item {
                    *best_end = end_byte;
                }
            }

            if cursor.goto_first_child() {
                self.find_best_boundary(cursor, max_bytes, best_end);
                cursor.goto_parent();
            }

            if !cursor.goto_next_sibling() {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_function_signature() {
        let source = r#"
function hello(name) {
    return `Hello, ${name}!`;
}

const add = (a, b) => a + b;
"#;

        let signatures = JavaScriptSupport.extract_signatures(source, Visibility::All);
        assert!(!signatures.is_empty());

        let funcs: Vec<_> = signatures
            .iter()
            .filter(|s| s.kind == SignatureKind::Function)
            .collect();
        assert!(!funcs.is_empty());
        assert_eq!(funcs[0].name, "hello");
    }

    #[test]
    fn test_extract_class_signature() {
        let source = r#"
class User {
    constructor(name) {
        this.name = name;
    }
}
}
"#;

        let signatures = JavaScriptSupport.extract_signatures(source, Visibility::All);
        let classes: Vec<_> = signatures
            .iter()
            .filter(|s| s.kind == SignatureKind::Class)
            .collect();
        assert!(!classes.is_empty());
        assert_eq!(classes[0].name, "User");
    }

    #[test]
    fn test_file_extensions() {
        assert!(JavaScriptSupport.supports_extension("js"));
        assert!(JavaScriptSupport.supports_extension("mjs"));
        assert!(!JavaScriptSupport.supports_extension("ts"));
    }
}
