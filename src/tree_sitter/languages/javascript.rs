//! JavaScript language support for tree-sitter.

use tree_sitter::{Parser, Tree};

use crate::tree_sitter::language_support::{CodeStructure, LanguageSupport, Signature, SignatureKind, Visibility};

pub struct JavaScriptSupport;

impl JavaScriptSupport {
    fn get_language() -> tree_sitter::Language {
        tree_sitter_javascript::language()
    }
}

impl LanguageSupport for JavaScriptSupport {
    fn file_extensions(&self) -> &[&'static str] {
        &["js", "mjs", "cjs"]
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

        if best_end == 0 {
            max_bytes
        } else {
            best_end
        }
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
                self.extract_export_signatures(source, node, signatures);
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

        let full_sig = match &params {
            Some(p) => format!("function {}({})", name, p),
            None => format!("function {}()", name),
        };

        Some(Signature {
            kind: SignatureKind::Function,
            name,
            params,
            return_type: None, // JS doesn't have explicit return types in syntax
            visibility: Visibility::All,
            line_number: node.start_position().row + 1,
            full_signature: full_sig,
        })
    }

    fn extract_class_signature(&self, source: &str, node: &tree_sitter::Node) -> Option<Signature> {
        let name = self.find_child_text(node, "identifier", source)?;

        let full_sig = format!("class {}", name);

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
                if let Some(name) = self.find_child_text(&child, "identifier", source) {
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
            } else if child.kind() == "class_declaration" {
                if let Some(sig) = self.extract_class_signature(source, &child) {
                    signatures.push(sig);
                }
            }
        }
    }

    fn find_child_text<'a>(
        &self,
        node: &tree_sitter::Node,
        kind: &str,
        source: &'a str,
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
