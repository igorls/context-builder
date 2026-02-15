//! Rust language support for tree-sitter.

use tree_sitter::{Parser, Tree};

use crate::tree_sitter::language_support::{
    CodeStructure, LanguageSupport, Signature, SignatureKind, Visibility,
    slice_signature_before_body,
};

pub struct RustSupport;

impl RustSupport {
    fn get_language() -> tree_sitter::Language {
        tree_sitter_rust::LANGUAGE.into()
    }
}

impl LanguageSupport for RustSupport {
    fn file_extensions(&self) -> &[&'static str] {
        &["rs"]
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

        self.extract_structure_from_node(source, &root, &mut structure);

        structure.code_lines = source
            .lines()
            .filter(|line| {
                let trimmed = line.trim();
                !trimmed.is_empty() && !trimmed.starts_with("//")
            })
            .count();

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

        self.walk_for_boundary(&mut cursor, max_bytes, &mut best_end);

        if best_end == 0 { max_bytes } else { best_end }
    }
}

impl RustSupport {
    fn walk_for_boundary(
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
                    "function_item"
                        | "struct_item"
                        | "enum_item"
                        | "trait_item"
                        | "impl_item"
                        | "mod_item"
                        | "const_item"
                        | "static_item"
                        | "type_item"
                        | "macro_definition"
                );
                if is_item {
                    *best_end = end_byte;
                }
            }

            if cursor.goto_first_child() {
                self.walk_for_boundary(cursor, max_bytes, best_end);
                cursor.goto_parent();
            }

            if !cursor.goto_next_sibling() {
                break;
            }
        }
    }

    fn extract_signatures_from_node(
        &self,
        source: &str,
        node: &tree_sitter::Node,
        visibility: Visibility,
        signatures: &mut Vec<Signature>,
    ) {
        match node.kind() {
            "function_item" => {
                if let Some(sig) = self.extract_function_signature(source, node, visibility) {
                    signatures.push(sig);
                }
            }
            "struct_item" => {
                if let Some(sig) = self.extract_struct_signature(source, node, visibility) {
                    signatures.push(sig);
                }
            }
            "enum_item" => {
                if let Some(sig) = self.extract_enum_signature(source, node, visibility) {
                    signatures.push(sig);
                }
            }
            "trait_item" => {
                if let Some(sig) = self.extract_trait_signature(source, node, visibility) {
                    signatures.push(sig);
                }
            }
            "impl_item" => {
                if let Some(sig) = self.extract_impl_signature(source, node) {
                    signatures.push(sig);
                }
            }
            "mod_item" => {
                if let Some(sig) = self.extract_mod_signature(source, node, visibility) {
                    signatures.push(sig);
                }
            }
            "const_item" => {
                if let Some(sig) = self.extract_const_signature(source, node, visibility) {
                    signatures.push(sig);
                }
            }
            "type_item" => {
                if let Some(sig) = self.extract_type_alias_signature(source, node, visibility) {
                    signatures.push(sig);
                }
            }
            "macro_definition" => {
                if let Some(sig) = self.extract_macro_signature(source, node, visibility) {
                    signatures.push(sig);
                }
            }
            _ => {}
        }

        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            self.extract_signatures_from_node(source, &child, visibility, signatures);
        }
    }

    fn extract_structure_from_node(
        &self,
        source: &str,
        node: &tree_sitter::Node,
        structure: &mut CodeStructure,
    ) {
        match node.kind() {
            "function_item" => structure.functions += 1,
            "struct_item" => structure.structs += 1,
            "enum_item" => structure.enums += 1,
            "trait_item" => structure.traits += 1,
            "const_item" => structure.constants += 1,
            "type_item" => structure.type_aliases += 1,
            "macro_definition" => structure.macros += 1,
            "use_declaration" => {
                structure
                    .imports
                    .push(self.node_text(source, node).to_string());
            }
            _ => {}
        }

        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            self.extract_structure_from_node(source, &child, structure);
        }
    }

    fn is_public(&self, node: &tree_sitter::Node) -> bool {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if child.kind() == "visibility_modifier" {
                return true;
            }
        }
        false
    }

    fn get_visibility(&self, node: &tree_sitter::Node) -> Visibility {
        if self.is_public(node) {
            Visibility::Public
        } else {
            Visibility::Private
        }
    }

    fn node_text<'a>(&self, source: &'a str, node: &tree_sitter::Node) -> &'a str {
        &source[node.start_byte()..node.end_byte()]
    }

    fn extract_function_signature(
        &self,
        source: &str,
        node: &tree_sitter::Node,
        visibility_filter: Visibility,
    ) -> Option<Signature> {
        let vis = self.get_visibility(node);
        if !vis.matches_filter(visibility_filter) {
            return None;
        }

        let name = self.find_child_text(node, "identifier", source)?;
        let params = self.find_child_text(node, "parameters", source);
        let return_type = self.find_child_text(node, "return_type", source);

        // Use byte-slicing to preserve generics, return types, and all modifiers
        let full_sig = slice_signature_before_body(source, node, &["block"])
            .unwrap_or_else(|| {
                // Fallback for declarations without a body
                let mut sig = String::new();
                if vis == Visibility::Public {
                    sig.push_str("pub ");
                }
                sig.push_str("fn ");
                sig.push_str(&name);
                if let Some(p) = &params {
                    sig.push_str(p);
                }
                if let Some(r) = &return_type {
                    sig.push_str(" -> ");
                    sig.push_str(r);
                }
                sig
            });

        Some(Signature {
            kind: SignatureKind::Function,
            name,
            params,
            return_type,
            visibility: vis,
            line_number: node.start_position().row + 1,
            full_signature: full_sig,
        })
    }

    fn extract_struct_signature(
        &self,
        source: &str,
        node: &tree_sitter::Node,
        visibility_filter: Visibility,
    ) -> Option<Signature> {
        let vis = self.get_visibility(node);
        if !vis.matches_filter(visibility_filter) {
            return None;
        }

        let name = self.find_child_text(node, "type_identifier", source)?;

        // Use byte-slicing to preserve generic bounds and where clauses
        // Include both `field_declaration_list` (regular structs) and
        // `ordered_field_declaration_list` (tuple structs like `struct Color(u8, u8, u8)`)
        let full_sig = slice_signature_before_body(source, node, &["field_declaration_list", "ordered_field_declaration_list"])
            .unwrap_or_else(|| {
                let mut sig = String::new();
                if vis == Visibility::Public {
                    sig.push_str("pub ");
                }
                sig.push_str("struct ");
                sig.push_str(&name);
                sig
            });

        Some(Signature {
            kind: SignatureKind::Struct,
            name,
            params: None,
            return_type: None,
            visibility: vis,
            line_number: node.start_position().row + 1,
            full_signature: full_sig,
        })
    }

    fn extract_enum_signature(
        &self,
        source: &str,
        node: &tree_sitter::Node,
        visibility_filter: Visibility,
    ) -> Option<Signature> {
        let vis = self.get_visibility(node);
        if !vis.matches_filter(visibility_filter) {
            return None;
        }

        let name = self.find_child_text(node, "type_identifier", source)?;

        // Use byte-slicing to preserve generic bounds
        let full_sig = slice_signature_before_body(source, node, &["enum_variant_list"])
            .unwrap_or_else(|| {
                let mut sig = String::new();
                if vis == Visibility::Public {
                    sig.push_str("pub ");
                }
                sig.push_str("enum ");
                sig.push_str(&name);
                sig
            });

        Some(Signature {
            kind: SignatureKind::Enum,
            name,
            params: None,
            return_type: None,
            visibility: vis,
            line_number: node.start_position().row + 1,
            full_signature: full_sig,
        })
    }

    fn extract_trait_signature(
        &self,
        source: &str,
        node: &tree_sitter::Node,
        visibility_filter: Visibility,
    ) -> Option<Signature> {
        let vis = self.get_visibility(node);
        if !vis.matches_filter(visibility_filter) {
            return None;
        }

        let name = self.find_child_text(node, "type_identifier", source)?;

        // Use byte-slicing to preserve trait bounds and supertraits
        let full_sig = slice_signature_before_body(source, node, &["declaration_list"])
            .unwrap_or_else(|| {
                let mut sig = String::new();
                if vis == Visibility::Public {
                    sig.push_str("pub ");
                }
                sig.push_str("trait ");
                sig.push_str(&name);
                sig
            });

        Some(Signature {
            kind: SignatureKind::Trait,
            name,
            params: None,
            return_type: None,
            visibility: vis,
            line_number: node.start_position().row + 1,
            full_signature: full_sig,
        })
    }

    fn extract_impl_signature(&self, source: &str, node: &tree_sitter::Node) -> Option<Signature> {
        let name = self.find_child_text(node, "type_identifier", source)?;

        // Use byte-slicing to preserve `impl Trait for Type` and generics
        let full_sig = slice_signature_before_body(source, node, &["declaration_list"])
            .unwrap_or_else(|| {
                let mut sig = String::new();
                sig.push_str("impl ");
                sig.push_str(&name);
                sig
            });

        Some(Signature {
            kind: SignatureKind::Impl,
            name,
            params: None,
            return_type: None,
            visibility: Visibility::All,
            line_number: node.start_position().row + 1,
            full_signature: full_sig,
        })
    }

    fn extract_mod_signature(
        &self,
        source: &str,
        node: &tree_sitter::Node,
        visibility_filter: Visibility,
    ) -> Option<Signature> {
        let vis = self.get_visibility(node);
        if !vis.matches_filter(visibility_filter) {
            return None;
        }

        let name = self.find_child_text(node, "identifier", source)?;

        let mut full_sig = String::new();
        if vis == Visibility::Public {
            full_sig.push_str("pub ");
        }
        full_sig.push_str("mod ");
        full_sig.push_str(&name);

        Some(Signature {
            kind: SignatureKind::Module,
            name,
            params: None,
            return_type: None,
            visibility: vis,
            line_number: node.start_position().row + 1,
            full_signature: full_sig,
        })
    }

    fn extract_const_signature(
        &self,
        source: &str,
        node: &tree_sitter::Node,
        visibility_filter: Visibility,
    ) -> Option<Signature> {
        let vis = self.get_visibility(node);
        if !vis.matches_filter(visibility_filter) {
            return None;
        }

        let name = self.find_child_text(node, "identifier", source)?;

        let mut full_sig = String::new();
        if vis == Visibility::Public {
            full_sig.push_str("pub ");
        }
        full_sig.push_str("const ");
        full_sig.push_str(&name);

        Some(Signature {
            kind: SignatureKind::Constant,
            name,
            params: None,
            return_type: None,
            visibility: vis,
            line_number: node.start_position().row + 1,
            full_signature: full_sig,
        })
    }

    fn extract_type_alias_signature(
        &self,
        source: &str,
        node: &tree_sitter::Node,
        visibility_filter: Visibility,
    ) -> Option<Signature> {
        let vis = self.get_visibility(node);
        if !vis.matches_filter(visibility_filter) {
            return None;
        }

        let name = self.find_child_text(node, "type_identifier", source)?;

        let mut full_sig = String::new();
        if vis == Visibility::Public {
            full_sig.push_str("pub ");
        }
        full_sig.push_str("type ");
        full_sig.push_str(&name);

        Some(Signature {
            kind: SignatureKind::TypeAlias,
            name,
            params: None,
            return_type: None,
            visibility: vis,
            line_number: node.start_position().row + 1,
            full_signature: full_sig,
        })
    }

    fn extract_macro_signature(
        &self,
        source: &str,
        node: &tree_sitter::Node,
        visibility_filter: Visibility,
    ) -> Option<Signature> {
        let vis = self.get_visibility(node);
        if !vis.matches_filter(visibility_filter) {
            return None;
        }

        let name = self.find_child_text(node, "identifier", source)?;

        let mut full_sig = String::new();
        if vis == Visibility::Public {
            full_sig.push_str("pub ");
        }
        full_sig.push_str("macro_rules! ");
        full_sig.push_str(&name);

        Some(Signature {
            kind: SignatureKind::Macro,
            name,
            params: None,
            return_type: None,
            visibility: vis,
            line_number: node.start_position().row + 1,
            full_signature: full_sig,
        })
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
                return Some(self.node_text(source, &child).to_string());
            }
            let mut nested_cursor = child.walk();
            for nested in child.children(&mut nested_cursor) {
                if nested.kind() == kind {
                    return Some(self.node_text(source, &nested).to_string());
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_function_signature() {
        let source = r#"
pub fn hello(name: &str) -> String {
    format!("Hello, {}!", name)
}

fn private_helper(x: i32) -> i32 {
    x * 2
}
"#;

        let signatures = RustSupport.extract_signatures(source, Visibility::All);
        assert_eq!(signatures.len(), 2);

        assert_eq!(signatures[0].name, "hello");
        assert_eq!(signatures[0].kind, SignatureKind::Function);
        assert_eq!(signatures[0].visibility, Visibility::Public);

        assert_eq!(signatures[1].name, "private_helper");
        assert_eq!(signatures[1].visibility, Visibility::Private);
    }

    #[test]
    fn test_public_only_filter() {
        let source = r#"
pub fn public_fn() {}
fn private_fn() {}
"#;

        let signatures = RustSupport.extract_signatures(source, Visibility::Public);
        assert_eq!(signatures.len(), 1);
        assert_eq!(signatures[0].name, "public_fn");
    }

    #[test]
    fn test_extract_struct_signature() {
        let source = r#"
pub struct User {
    name: String,
    age: u32,
}
"#;

        let signatures = RustSupport.extract_signatures(source, Visibility::All);
        assert_eq!(signatures.len(), 1);
        assert_eq!(signatures[0].name, "User");
        assert_eq!(signatures[0].kind, SignatureKind::Struct);
    }

    #[test]
    fn test_extract_structure() {
        let source = r#"
use std::fs;

pub struct Config {
    path: String,
}

pub fn load() -> Config {
    Config { path: ".".into() }
}

enum Status {
    Active,
    Inactive,
}
"#;

        let structure = RustSupport.extract_structure(source);
        assert_eq!(structure.structs, 1);
        assert_eq!(structure.functions, 1);
        assert_eq!(structure.enums, 1);
    }

    #[test]
    fn test_find_truncation_point() {
        let source = r#"
fn first() -> i32 {
    1
}

fn second() -> i32 {
    2
}

fn third() -> i32 {
    3
}
"#;

        let after_first = source.find("fn second()").unwrap();
        let point = RustSupport.find_truncation_point(source, after_first);

        assert!(point <= after_first);
        assert!(source[..point].contains("fn first()"));
    }

    #[test]
    fn test_file_extensions() {
        assert!(RustSupport.supports_extension("rs"));
        assert!(!RustSupport.supports_extension("py"));
    }
}
