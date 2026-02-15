//! C language support for tree-sitter.

#[cfg(feature = "tree-sitter-c")]
use tree_sitter::{Parser, Tree};

#[cfg(feature = "tree-sitter-c")]
use crate::tree_sitter::language_support::{
    CodeStructure, LanguageSupport, Signature, SignatureKind, Visibility,
    slice_signature_before_body,
};

pub struct CSupport;

#[cfg(feature = "tree-sitter-c")]
impl CSupport {
    fn get_language() -> tree_sitter::Language {
        tree_sitter_c::LANGUAGE.into()
    }
}

#[cfg(feature = "tree-sitter-c")]
impl LanguageSupport for CSupport {
    fn file_extensions(&self) -> &[&'static str] {
        &["c", "h"]
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

#[cfg(feature = "tree-sitter-c")]
impl CSupport {
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
            "declaration" => {
                // Header file prototypes: `int foo(int x, int y);`
                if let Some(sig) = self.extract_declaration_signature(source, node) {
                    signatures.push(sig);
                }
            }
            "struct_specifier" => {
                if let Some(sig) = self.extract_struct_signature(source, node) {
                    signatures.push(sig);
                }
            }
            "enum_specifier" => {
                if let Some(sig) = self.extract_enum_signature(source, node) {
                    signatures.push(sig);
                }
            }
            "type_definition" => {
                if let Some(sig) = self.extract_typedef_signature(source, node) {
                    signatures.push(sig);
                }
            }
            "preproc_function_def" => {
                if let Some(sig) = self.extract_macro_signature(source, node) {
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
            "struct_specifier" => structure.structs += 1,
            "enum_specifier" => structure.enums += 1,
            "preproc_include" => {
                structure.imports.push("include".to_string());
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
        let name = self.find_function_name(node, source)?;
        let return_type = self.find_return_type(node, source);
        let params = self.find_child_text(node, "parameter_list", source);

        // Use byte-slicing to preserve complete function signatures including parameters
        let full_sig = slice_signature_before_body(source, node, &["compound_statement"])
            .unwrap_or_else(|| {
                let mut sig = String::new();
                if let Some(r) = &return_type {
                    sig.push_str(r);
                    sig.push(' ');
                }
                sig.push_str(&name);
                if let Some(p) = &params {
                    sig.push_str(p);
                } else {
                    sig.push_str("()");
                }
                sig
            });

        Some(Signature {
            kind: SignatureKind::Function,
            name,
            params,
            return_type,
            visibility: Visibility::All, // C has no visibility
            line_number: node.start_position().row + 1,
            full_signature: full_sig,
        })
    }

    /// Extract function prototype signatures from `declaration` nodes (header files).
    /// e.g., `int foo(int x, int y);`
    fn extract_declaration_signature(
        &self,
        source: &str,
        node: &tree_sitter::Node,
    ) -> Option<Signature> {
        // Only capture declarations that look like function prototypes
        // (have a function_declarator child with parameters)
        let mut cursor = node.walk();
        let has_function_declarator = node.children(&mut cursor).any(|c| {
            if c.kind() == "function_declarator" {
                return true;
            }
            // Check one level deeper for pointer declarations
            let mut inner = c.walk();
            c.children(&mut inner)
                .any(|gc| gc.kind() == "function_declarator")
        });

        if !has_function_declarator {
            return None;
        }

        let name = self.find_function_name(node, source)?;
        // For prototypes, the full node text IS the signature (no body to strip)
        let text = source[node.start_byte()..node.end_byte()].trim_end();
        // Remove trailing semicolon for cleaner output
        let full_sig = text.trim_end_matches(';').trim_end().to_string();

        Some(Signature {
            kind: SignatureKind::Function,
            name,
            params: None, // Captured in full_sig
            return_type: None,
            visibility: Visibility::All,
            line_number: node.start_position().row + 1,
            full_signature: full_sig,
        })
    }

    fn extract_struct_signature(
        &self,
        source: &str,
        node: &tree_sitter::Node,
    ) -> Option<Signature> {
        let name = self.find_child_text(node, "type_identifier", source)?;

        let full_sig = format!("struct {}", name);

        Some(Signature {
            kind: SignatureKind::Struct,
            name,
            params: None,
            return_type: None,
            visibility: Visibility::All,
            line_number: node.start_position().row + 1,
            full_signature: full_sig,
        })
    }

    fn extract_enum_signature(&self, source: &str, node: &tree_sitter::Node) -> Option<Signature> {
        let name = self.find_child_text(node, "type_identifier", source)?;

        let full_sig = format!("enum {}", name);

        Some(Signature {
            kind: SignatureKind::Enum,
            name,
            params: None,
            return_type: None,
            visibility: Visibility::All,
            line_number: node.start_position().row + 1,
            full_signature: full_sig,
        })
    }

    fn extract_typedef_signature(
        &self,
        source: &str,
        node: &tree_sitter::Node,
    ) -> Option<Signature> {
        let name = self.find_child_text(node, "type_identifier", source)?;

        let full_sig = format!("typedef {}", name);

        Some(Signature {
            kind: SignatureKind::TypeAlias,
            name,
            params: None,
            return_type: None,
            visibility: Visibility::All,
            line_number: node.start_position().row + 1,
            full_signature: full_sig,
        })
    }

    fn extract_macro_signature(&self, source: &str, node: &tree_sitter::Node) -> Option<Signature> {
        let name = self.find_child_text(node, "identifier", source)?;

        let full_sig = format!("#define {}", name);

        Some(Signature {
            kind: SignatureKind::Macro,
            name,
            params: None,
            return_type: None,
            visibility: Visibility::All,
            line_number: node.start_position().row + 1,
            full_signature: full_sig,
        })
    }

    fn find_function_name(&self, node: &tree_sitter::Node, source: &str) -> Option<String> {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if child.kind() == "function_declarator" {
                let mut inner_cursor = child.walk();
                for inner in child.children(&mut inner_cursor) {
                    if inner.kind() == "identifier" {
                        return Some(source[inner.start_byte()..inner.end_byte()].to_string());
                    }
                }
            }
        }
        None
    }

    fn find_return_type(&self, node: &tree_sitter::Node, source: &str) -> Option<String> {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if child.kind() == "primitive_type" || child.kind() == "type_identifier" {
                return Some(source[child.start_byte()..child.end_byte()].to_string());
            }
        }
        None
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
                    "function_definition"
                        | "struct_specifier"
                        | "enum_specifier"
                        | "type_definition"
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
int main() {
    return 0;
}

void hello(const char* name) {
    printf("Hello, %s\n", name);
}
"#;

        let signatures = CSupport.extract_signatures(source, Visibility::All);
        assert!(!signatures.is_empty());

        let funcs: Vec<_> = signatures
            .iter()
            .filter(|s| s.kind == SignatureKind::Function)
            .collect();
        assert!(funcs.len() >= 2);
    }

    #[test]
    fn test_extract_struct_signature() {
        let source = r#"
struct Point {
    int x;
    int y;
};
"#;

        let signatures = CSupport.extract_signatures(source, Visibility::All);
        let structs: Vec<_> = signatures
            .iter()
            .filter(|s| s.kind == SignatureKind::Struct)
            .collect();
        assert!(!structs.is_empty());
        assert_eq!(structs[0].name, "Point");
    }

    #[test]
    fn test_extract_enum_signature() {
        let source = r#"
enum Color {
    RED,
    GREEN,
    BLUE
};
"#;

        let signatures = CSupport.extract_signatures(source, Visibility::All);
        let enums: Vec<_> = signatures
            .iter()
            .filter(|s| s.kind == SignatureKind::Enum)
            .collect();
        assert!(!enums.is_empty());
        assert_eq!(enums[0].name, "Color");
    }

    #[test]
    fn test_extract_header_prototype() {
        let source = r#"
int add(int a, int b);
void greet(const char* name);
"#;

        let signatures = CSupport.extract_signatures(source, Visibility::All);
        let funcs: Vec<_> = signatures
            .iter()
            .filter(|s| s.kind == SignatureKind::Function)
            .collect();
        assert!(funcs.len() >= 2);
        // Declarations should not end with semicolons
        for f in &funcs {
            assert!(!f.full_signature.ends_with(';'));
        }
    }

    #[test]
    fn test_extract_typedef() {
        let source = r#"
typedef unsigned int uint;
typedef struct { int x; int y; } Point;
"#;

        let signatures = CSupport.extract_signatures(source, Visibility::All);
        let aliases: Vec<_> = signatures
            .iter()
            .filter(|s| s.kind == SignatureKind::TypeAlias)
            .collect();
        assert!(!aliases.is_empty());
    }

    #[test]
    fn test_extract_macro_definition() {
        let source = r#"
#define MAX(a, b) ((a) > (b) ? (a) : (b))
"#;

        let signatures = CSupport.extract_signatures(source, Visibility::All);
        let macros: Vec<_> = signatures
            .iter()
            .filter(|s| s.kind == SignatureKind::Macro)
            .collect();
        assert!(!macros.is_empty());
        assert_eq!(macros[0].name, "MAX");
    }

    #[test]
    fn test_extract_structure() {
        let source = r#"
#include <stdio.h>
#include <stdlib.h>

struct Point { int x; int y; };
enum Color { RED, GREEN };

int main() {
    return 0;
}

void helper() {}
"#;

        let structure = CSupport.extract_structure(source);
        assert!(structure.functions >= 2);
        assert!(structure.structs >= 1);
        assert!(structure.enums >= 1);
        assert!(structure.imports.len() >= 2);
    }

    #[test]
    fn test_parse_valid_c() {
        let source = "int main() { return 0; }";
        let tree = CSupport.parse(source);
        assert!(tree.is_some());
    }

    #[test]
    fn test_find_truncation_point_within_limit() {
        let source = "int main() { return 0; }";
        let point = CSupport.find_truncation_point(source, 1000);
        assert_eq!(point, source.len());
    }

    #[test]
    fn test_file_extensions() {
        assert!(CSupport.supports_extension("c"));
        assert!(CSupport.supports_extension("h"));
        assert!(!CSupport.supports_extension("cpp"));
    }
}
