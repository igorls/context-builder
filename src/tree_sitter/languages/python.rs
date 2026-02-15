//! Python language support for tree-sitter.

#[cfg(feature = "tree-sitter-python")]
use tree_sitter::{Parser, Tree};

#[cfg(feature = "tree-sitter-python")]
use crate::tree_sitter::language_support::{
    CodeStructure, LanguageSupport, Signature, SignatureKind, Visibility,
};

pub struct PythonSupport;

#[cfg(feature = "tree-sitter-python")]
impl PythonSupport {
    fn get_language() -> tree_sitter::Language {
        tree_sitter_python::LANGUAGE.into()
    }
}

#[cfg(feature = "tree-sitter-python")]
impl LanguageSupport for PythonSupport {
    fn file_extensions(&self) -> &[&'static str] {
        &["py", "pyw"]
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

#[cfg(feature = "tree-sitter-python")]
impl PythonSupport {
    fn extract_signatures_from_node(
        &self,
        source: &str,
        node: &tree_sitter::Node,
        _visibility: Visibility,
        signatures: &mut Vec<Signature>,
    ) {
        match node.kind() {
            "function_definition" => {
                if let Some(sig) = self.extract_function_signature(source, node) {
                    signatures.push(sig);
                }
            }
            "class_definition" => {
                if let Some(sig) = self.extract_class_signature(source, node) {
                    signatures.push(sig);
                }
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
            "function_definition" => structure.functions += 1,
            "class_definition" => structure.classes += 1,
            "import_statement" | "import_from_statement" => {
                structure.imports.push("import".to_string());
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
        let params = self.find_child_text(node, "parameters", source);

        // Check for decorators (to detect @property, @staticmethod, etc.)
        let is_method = node
            .parent()
            .is_some_and(|p| p.kind() == "class_definition");
        let kind = if is_method {
            SignatureKind::Method
        } else {
            SignatureKind::Function
        };

        let mut full_sig = String::new();
        if let Some(decorators) = self.find_decorators(source, node) {
            full_sig.push_str(&decorators);
            full_sig.push('\n');
        }
        full_sig.push_str("def ");
        full_sig.push_str(&name);
        if let Some(p) = &params {
            full_sig.push_str(p);
        } else {
            full_sig.push_str("()");
        }

        Some(Signature {
            kind,
            name,
            params,
            return_type: None, // Python uses type hints differently
            visibility: Visibility::All,
            line_number: node.start_position().row + 1,
            full_signature: full_sig,
        })
    }

    fn extract_class_signature(&self, source: &str, node: &tree_sitter::Node) -> Option<Signature> {
        let name = self.find_child_text(node, "identifier", source)?;
        let bases = self.find_child_text(node, "argument_list", source);

        let mut full_sig = String::new();
        if let Some(decorators) = self.find_decorators(source, node) {
            full_sig.push_str(&decorators);
            full_sig.push('\n');
        }
        full_sig.push_str("class ");
        full_sig.push_str(&name);
        if let Some(b) = &bases {
            full_sig.push('(');
            full_sig.push_str(b);
            full_sig.push(')');
        }

        Some(Signature {
            kind: SignatureKind::Class,
            name,
            params: bases,
            return_type: None,
            visibility: Visibility::All,
            line_number: node.start_position().row + 1,
            full_signature: full_sig,
        })
    }

    fn find_decorators(&self, source: &str, node: &tree_sitter::Node) -> Option<String> {
        let parent = node.parent()?;
        let mut cursor = parent.walk();
        let mut decorators = Vec::new();

        for sibling in parent.children(&mut cursor) {
            if sibling.kind() == "decorator"
                && sibling.end_position().row == node.start_position().row.saturating_sub(1)
            {
                let text = &source[sibling.start_byte()..sibling.end_byte()];
                decorators.push(text.to_string());
            }
        }

        if decorators.is_empty() {
            None
        } else {
            Some(decorators.join("\n"))
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
                    "function_definition" | "class_definition" | "decorated_definition"
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
def hello(name):
    return f"Hello, {name}!"

def add(a: int, b: int) -> int:
    return a + b
"#;

        let signatures = PythonSupport.extract_signatures(source, Visibility::All);
        assert!(!signatures.is_empty());

        let funcs: Vec<_> = signatures
            .iter()
            .filter(|s| matches!(s.kind, SignatureKind::Function | SignatureKind::Method))
            .collect();
        assert!(funcs.len() >= 2);
        assert_eq!(funcs[0].name, "hello");
    }

    #[test]
    fn test_extract_class_signature() {
        let source = r#"
class User:
    def __init__(self, name):
        self.name = name
"#;

        let signatures = PythonSupport.extract_signatures(source, Visibility::All);
        let classes: Vec<_> = signatures
            .iter()
            .filter(|s| s.kind == SignatureKind::Class)
            .collect();
        assert!(!classes.is_empty());
        assert_eq!(classes[0].name, "User");
    }

    #[test]
    fn test_file_extensions() {
        assert!(PythonSupport.supports_extension("py"));
        assert!(PythonSupport.supports_extension("pyw"));
        assert!(!PythonSupport.supports_extension("rs"));
    }
}
