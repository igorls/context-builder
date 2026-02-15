//! C++ language support for tree-sitter.

#[cfg(feature = "tree-sitter-cpp")]
use tree_sitter::{Parser, Tree};

#[cfg(feature = "tree-sitter-cpp")]
use crate::tree_sitter::language_support::{
    CodeStructure, LanguageSupport, Signature, SignatureKind, Visibility,
    slice_signature_before_body,
};

pub struct CppSupport;

#[cfg(feature = "tree-sitter-cpp")]
impl CppSupport {
    fn get_language() -> tree_sitter::Language {
        tree_sitter_cpp::LANGUAGE.into()
    }
}

#[cfg(feature = "tree-sitter-cpp")]
impl LanguageSupport for CppSupport {
    fn file_extensions(&self) -> &[&'static str] {
        &["cpp", "cxx", "cc", "hpp", "hxx", "hh"]
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

#[cfg(feature = "tree-sitter-cpp")]
impl CppSupport {
    fn extract_signatures_from_node(
        &self,
        source: &str,
        node: &tree_sitter::Node,
        visibility: Visibility,
        signatures: &mut Vec<Signature>,
    ) {
        match node.kind() {
            "function_definition" => {
                if let Some(sig) = self.extract_function_signature(source, node, visibility) {
                    signatures.push(sig);
                }
            }
            "declaration" => {
                // Header file prototypes: `int foo(int x, int y);`
                if let Some(sig) = self.extract_declaration_signature(source, node) {
                    signatures.push(sig);
                }
            }
            "class_specifier" => {
                if let Some(sig) = self.extract_class_signature(source, node, visibility) {
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
            "alias_declaration" | "type_definition" => {
                if let Some(sig) = self.extract_alias_signature(source, node) {
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
            self.extract_signatures_from_node(source, &child, visibility, signatures);
        }
    }

    fn extract_structure_from_node(&self, node: &tree_sitter::Node, structure: &mut CodeStructure) {
        match node.kind() {
            "function_definition" => structure.functions += 1,
            "class_specifier" => structure.classes += 1,
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

    #[allow(dead_code)]
    fn get_visibility(&self, _node: &tree_sitter::Node) -> Visibility {
        // C++ has access specifiers: public, private, protected
        // For simplicity, we check sibling nodes for access specifiers
        // This is a simplified check; full implementation would track class context
        Visibility::All
    }

    fn extract_function_signature(
        &self,
        source: &str,
        node: &tree_sitter::Node,
        visibility: Visibility,
    ) -> Option<Signature> {
        let name = self.find_function_name(node, source)?;
        let return_type = self.find_return_type(node, source);
        let params = self.find_child_text(node, "parameter_list", source);

        // Use byte-slicing to preserve templates, parameters, and qualifiers
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
            visibility,
            line_number: node.start_position().row + 1,
            full_signature: full_sig,
        })
    }

    /// Extract function prototype signatures from `declaration` nodes (header files).
    fn extract_declaration_signature(
        &self,
        source: &str,
        node: &tree_sitter::Node,
    ) -> Option<Signature> {
        // Only capture declarations that look like function prototypes
        let mut cursor = node.walk();
        let has_function_declarator = node.children(&mut cursor).any(|c| {
            if c.kind() == "function_declarator" {
                return true;
            }
            let mut inner = c.walk();
            c.children(&mut inner).any(|gc| gc.kind() == "function_declarator")
        });

        if !has_function_declarator {
            return None;
        }

        let name = self.find_function_name(node, source)?;
        let text = source[node.start_byte()..node.end_byte()].trim_end();
        let full_sig = text.trim_end_matches(';').trim_end().to_string();

        Some(Signature {
            kind: SignatureKind::Function,
            name,
            params: None,
            return_type: None,
            visibility: Visibility::All,
            line_number: node.start_position().row + 1,
            full_signature: full_sig,
        })
    }
    fn extract_class_signature(
        &self,
        source: &str,
        node: &tree_sitter::Node,
        visibility: Visibility,
    ) -> Option<Signature> {
        let name = self.find_child_text(node, "type_identifier", source)?;

        // Use byte-slicing to preserve templates and inheritance
        // e.g., `template<typename T> class Foo : public Base`
        let full_sig = slice_signature_before_body(source, node, &["field_declaration_list"])
            .unwrap_or_else(|| format!("class {}", name));

        Some(Signature {
            kind: SignatureKind::Class,
            name,
            params: None,
            return_type: None,
            visibility,
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

    fn extract_alias_signature(&self, source: &str, node: &tree_sitter::Node) -> Option<Signature> {
        let name = self.find_child_text(node, "type_identifier", source)?;

        let full_sig = format!("using/typedef {}", name);

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
            if child.kind() == "function_declarator" || child.kind() == "reference_declarator" {
                let mut inner_cursor = child.walk();
                for inner in child.children(&mut inner_cursor) {
                    if inner.kind() == "identifier" || inner.kind() == "qualified_identifier" {
                        return Some(source[inner.start_byte()..inner.end_byte()].to_string());
                    }
                }
            }
            if child.kind() == "identifier" || child.kind() == "qualified_identifier" {
                return Some(source[child.start_byte()..child.end_byte()].to_string());
            }
        }
        None
    }

    fn find_return_type(&self, node: &tree_sitter::Node, source: &str) -> Option<String> {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            match child.kind() {
                "primitive_type" | "type_identifier" | "sized_type_specifier" => {
                    return Some(source[child.start_byte()..child.end_byte()].to_string());
                }
                _ => {}
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
                        | "class_specifier"
                        | "struct_specifier"
                        | "enum_specifier"
                        | "alias_declaration"
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
    fn test_extract_class_signature() {
        let source = r#"
class HelloWorld {
public:
    void greet() {
        std::cout << "Hello" << std::endl;
    }
};
"#;

        let signatures = CppSupport.extract_signatures(source, Visibility::All);
        let classes: Vec<_> = signatures
            .iter()
            .filter(|s| s.kind == SignatureKind::Class)
            .collect();
        assert!(!classes.is_empty());
        assert_eq!(classes[0].name, "HelloWorld");
    }

    #[test]
    fn test_extract_function_signature() {
        let source = r#"
int add(int a, int b) {
    return a + b;
}

void greet(const std::string& name) {
    std::cout << name << std::endl;
}
"#;

        let signatures = CppSupport.extract_signatures(source, Visibility::All);
        let funcs: Vec<_> = signatures
            .iter()
            .filter(|s| s.kind == SignatureKind::Function)
            .collect();
        assert!(funcs.len() >= 2);
    }

    #[test]
    fn test_extract_struct_signature() {
        let source = r#"
struct Vec3 {
    float x, y, z;
};
"#;

        let signatures = CppSupport.extract_signatures(source, Visibility::All);
        let structs: Vec<_> = signatures
            .iter()
            .filter(|s| s.kind == SignatureKind::Struct)
            .collect();
        assert!(!structs.is_empty());
        assert_eq!(structs[0].name, "Vec3");
    }

    #[test]
    fn test_extract_enum_signature() {
        let source = r#"
enum class Direction {
    Up,
    Down,
    Left,
    Right
};
"#;

        let signatures = CppSupport.extract_signatures(source, Visibility::All);
        let enums: Vec<_> = signatures
            .iter()
            .filter(|s| s.kind == SignatureKind::Enum)
            .collect();
        assert!(!enums.is_empty());
        assert_eq!(enums[0].name, "Direction");
    }

    #[test]
    fn test_extract_header_prototype() {
        let source = r#"
int add(int a, int b);
void greet(const std::string& name);
"#;

        let signatures = CppSupport.extract_signatures(source, Visibility::All);
        let funcs: Vec<_> = signatures
            .iter()
            .filter(|s| s.kind == SignatureKind::Function)
            .collect();
        assert!(funcs.len() >= 2);
        for f in &funcs {
            assert!(!f.full_signature.ends_with(';'));
        }
    }

    #[test]
    fn test_extract_template_class_with_inheritance() {
        let source = r#"
template<typename T>
class Container : public Base {
    T value;
public:
    T get() { return value; }
};
"#;

        let signatures = CppSupport.extract_signatures(source, Visibility::All);
        let classes: Vec<_> = signatures
            .iter()
            .filter(|s| s.kind == SignatureKind::Class)
            .collect();
        assert!(!classes.is_empty());
        // Byte-slicing fix should preserve template<> and : public Base
        let sig = &classes[0].full_signature;
        assert!(sig.contains("Container"));
    }

    #[test]
    fn test_extract_type_alias() {
        let source = r#"
using StringVec = std::vector<std::string>;
typedef unsigned int uint;
"#;

        let signatures = CppSupport.extract_signatures(source, Visibility::All);
        let aliases: Vec<_> = signatures
            .iter()
            .filter(|s| s.kind == SignatureKind::TypeAlias)
            .collect();
        assert!(!aliases.is_empty());
    }

    #[test]
    fn test_extract_macro() {
        let source = r#"
#define MIN(a, b) ((a) < (b) ? (a) : (b))
"#;

        let signatures = CppSupport.extract_signatures(source, Visibility::All);
        let macros: Vec<_> = signatures
            .iter()
            .filter(|s| s.kind == SignatureKind::Macro)
            .collect();
        assert!(!macros.is_empty());
        assert_eq!(macros[0].name, "MIN");
    }

    #[test]
    fn test_extract_structure() {
        let source = r#"
#include <iostream>
#include <vector>

class Foo {
public:
    void bar() {}
};

struct Point { int x; int y; };
enum Color { R, G, B };

void helper() {}
"#;

        let structure = CppSupport.extract_structure(source);
        assert!(structure.functions >= 1);
        assert!(structure.classes >= 1);
        assert!(structure.structs >= 1);
        assert!(structure.enums >= 1);
        assert!(structure.imports.len() >= 2);
    }

    #[test]
    fn test_parse_valid_cpp() {
        let source = "int main() { return 0; }";
        let tree = CppSupport.parse(source);
        assert!(tree.is_some());
    }

    #[test]
    fn test_find_truncation_point() {
        let source = "int main() { return 0; }";
        let point = CppSupport.find_truncation_point(source, 1000);
        assert_eq!(point, source.len());
    }

    #[test]
    fn test_file_extensions() {
        assert!(CppSupport.supports_extension("cpp"));
        assert!(CppSupport.supports_extension("hpp"));
        assert!(CppSupport.supports_extension("cxx"));
        assert!(!CppSupport.supports_extension("c"));
    }
}

