//! Go language support for tree-sitter.

#[cfg(feature = "tree-sitter-go")]
use tree_sitter::{Parser, Tree};

#[cfg(feature = "tree-sitter-go")]
use crate::tree_sitter::language_support::{
    CodeStructure, LanguageSupport, Signature, SignatureKind, Visibility,
    slice_signature_before_body,
};

pub struct GoSupport;

#[cfg(feature = "tree-sitter-go")]
impl GoSupport {
    fn get_language() -> tree_sitter::Language {
        tree_sitter_go::LANGUAGE.into()
    }
}

#[cfg(feature = "tree-sitter-go")]
impl LanguageSupport for GoSupport {
    fn file_extensions(&self) -> &[&'static str] {
        &["go"]
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

#[cfg(feature = "tree-sitter-go")]
impl GoSupport {
    fn extract_signatures_from_node(
        &self,
        source: &str,
        node: &tree_sitter::Node,
        visibility: Visibility,
        signatures: &mut Vec<Signature>,
    ) {
        match node.kind() {
            "function_declaration" => {
                if let Some(sig) = self.extract_function_signature(source, node, visibility) {
                    signatures.push(sig);
                }
            }
            "method_declaration" => {
                if let Some(sig) = self.extract_method_signature(source, node, visibility) {
                    signatures.push(sig);
                }
            }
            "type_declaration" => {
                self.extract_type_signatures(source, node, visibility, signatures);
            }
            _ => {}
        }

        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            self.extract_signatures_from_node(source, &child, visibility, signatures);
        }
    }

    fn extract_structure_from_node(&self, node: &tree_sitter::Node, structure: &mut CodeStructure) {
        match node.kind() {
            "function_declaration" | "method_declaration" => structure.functions += 1,
            "type_spec" => {
                // Check what type it is
                if let Some(parent) = node.parent()
                    && parent.kind() == "type_declaration"
                {
                    // Could be struct, interface, or type alias
                    let mut cursor = node.walk();
                    for child in node.children(&mut cursor) {
                        match child.kind() {
                            "struct_type" => structure.structs += 1,
                            "interface_type" => structure.interfaces += 1,
                            "type_identifier" => structure.type_aliases += 1,
                            _ => {}
                        }
                    }
                }
            }
            "import_declaration" => {
                structure.imports.push("import".to_string());
            }
            _ => {}
        }

        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            self.extract_structure_from_node(&child, structure);
        }
    }

    fn is_exported(&self, name: &str) -> bool {
        name.chars().next().is_some_and(|c| c.is_uppercase())
    }

    fn extract_function_signature(
        &self,
        source: &str,
        node: &tree_sitter::Node,
        visibility: Visibility,
    ) -> Option<Signature> {
        let name = self.find_child_text(node, "identifier", source)?;
        let is_exported = self.is_exported(&name);

        if visibility == Visibility::Public && !is_exported {
            return None;
        }
        if visibility == Visibility::Private && is_exported {
            return None;
        }

        let params = self.find_child_text(node, "parameter_list", source);
        let result = self
            .find_child_text(node, "type_identifier", source)
            .or_else(|| self.find_child_text_for_result(node, source));

        // Use byte-slicing to preserve receivers, multiple return values, and named results
        let full_sig = slice_signature_before_body(source, node, &["block"])
            .unwrap_or_else(|| {
                let mut sig = String::new();
                sig.push_str("func ");
                sig.push_str(&name);
                if let Some(p) = &params {
                    sig.push_str(p);
                } else {
                    sig.push_str("()");
                }
                if let Some(r) = &result {
                    sig.push(' ');
                    sig.push_str(r);
                }
                sig
            });

        Some(Signature {
            kind: SignatureKind::Function,
            name,
            params,
            return_type: result,
            visibility: if is_exported {
                Visibility::Public
            } else {
                Visibility::Private
            },
            line_number: node.start_position().row + 1,
            full_signature: full_sig,
        })
    }

    fn extract_method_signature(
        &self,
        source: &str,
        node: &tree_sitter::Node,
        visibility: Visibility,
    ) -> Option<Signature> {
        let name = self
            .find_child_text(node, "field_identifier", source)
            .or_else(|| self.find_child_text(node, "identifier", source))?;
        let is_exported = self.is_exported(&name);

        if visibility == Visibility::Public && !is_exported {
            return None;
        }
        if visibility == Visibility::Private && is_exported {
            return None;
        }

        let receiver = self.find_child_text(node, "parameter_list", source);
        let params = self.find_method_params(node, source);
        let result = self.find_child_text_for_result(node, source);

        let mut full_sig = String::new();
        full_sig.push_str("func ");
        if let Some(r) = &receiver {
            full_sig.push_str(r);
            full_sig.push(' ');
        }
        full_sig.push_str(&name);
        if let Some(p) = &params {
            full_sig.push_str(p);
        } else {
            full_sig.push_str("()");
        }
        if let Some(r) = &result {
            full_sig.push(' ');
            full_sig.push_str(r);
        }

        Some(Signature {
            kind: SignatureKind::Method,
            name,
            params,
            return_type: result,
            visibility: if is_exported {
                Visibility::Public
            } else {
                Visibility::Private
            },
            line_number: node.start_position().row + 1,
            full_signature: full_sig,
        })
    }

    fn extract_type_signatures(
        &self,
        source: &str,
        node: &tree_sitter::Node,
        visibility: Visibility,
        signatures: &mut Vec<Signature>,
    ) {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if child.kind() == "type_spec"
                && let Some(name) = self.find_child_text(&child, "type_identifier", source)
            {
                let is_exported = self.is_exported(&name);

                if visibility == Visibility::Public && !is_exported {
                    continue;
                }
                if visibility == Visibility::Private && is_exported {
                    continue;
                }

                let kind = self.get_type_kind(&child);
                let full_sig = format!("type {} {}", name, kind);

                signatures.push(Signature {
                    kind,
                    name,
                    params: None,
                    return_type: None,
                    visibility: if is_exported {
                        Visibility::Public
                    } else {
                        Visibility::Private
                    },
                    line_number: child.start_position().row + 1,
                    full_signature: full_sig,
                });
            }
        }
    }

    fn get_type_kind(&self, node: &tree_sitter::Node) -> SignatureKind {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            match child.kind() {
                "struct_type" => return SignatureKind::Struct,
                "interface_type" => return SignatureKind::Interface,
                _ => {}
            }
        }
        SignatureKind::TypeAlias
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

    fn find_child_text_for_result(&self, node: &tree_sitter::Node, source: &str) -> Option<String> {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if child.kind() == "func_result" {
                return Some(source[child.start_byte()..child.end_byte()].to_string());
            }
        }
        None
    }

    fn find_method_params(&self, node: &tree_sitter::Node, source: &str) -> Option<String> {
        let mut cursor = node.walk();
        let mut found_receiver = false;
        for child in node.children(&mut cursor) {
            if child.kind() == "parameter_list" {
                if found_receiver {
                    return Some(source[child.start_byte()..child.end_byte()].to_string());
                }
                found_receiver = true;
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
                    "function_declaration" | "method_declaration" | "type_declaration"
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
package main

func Hello(name string) string {
    return "Hello, " + name
}

func internal() int {
    return 42
}
"#;

        let signatures = GoSupport.extract_signatures(source, Visibility::All);
        assert!(!signatures.is_empty());

        let funcs: Vec<_> = signatures
            .iter()
            .filter(|s| s.kind == SignatureKind::Function)
            .collect();
        assert!(funcs.len() >= 2);
    }

    #[test]
    fn test_public_only_filter() {
        let source = r#"
func PublicFunc() {}
func privateFunc() {}
"#;

        let signatures = GoSupport.extract_signatures(source, Visibility::Public);
        assert_eq!(signatures.len(), 1);
        assert_eq!(signatures[0].name, "PublicFunc");
    }

    #[test]
    fn test_extract_struct_signature() {
        let source = r#"
type User struct {
    Name string
    Age  int
}
"#;

        let signatures = GoSupport.extract_signatures(source, Visibility::All);
        let structs: Vec<_> = signatures
            .iter()
            .filter(|s| s.kind == SignatureKind::Struct)
            .collect();
        assert!(!structs.is_empty());
        assert_eq!(structs[0].name, "User");
    }

    #[test]
    fn test_file_extensions() {
        assert!(GoSupport.supports_extension("go"));
        assert!(!GoSupport.supports_extension("rs"));
    }
}
