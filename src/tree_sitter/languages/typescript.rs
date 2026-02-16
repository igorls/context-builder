//! TypeScript and TSX language support for tree-sitter.

#[cfg(feature = "tree-sitter-ts")]
use tree_sitter::{Parser, Tree};

#[cfg(feature = "tree-sitter-ts")]
use crate::tree_sitter::language_support::{
    CodeStructure, LanguageSupport, Signature, SignatureKind, Visibility,
    slice_signature_before_body,
};

pub struct TypeScriptSupport;
pub struct TsxSupport;

#[cfg(feature = "tree-sitter-ts")]
impl TypeScriptSupport {
    fn get_language() -> tree_sitter::Language {
        tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into()
    }
}

#[cfg(feature = "tree-sitter-ts")]
impl TsxSupport {
    fn get_language() -> tree_sitter::Language {
        tree_sitter_typescript::LANGUAGE_TSX.into()
    }
}

/// Macro to implement LanguageSupport and shared helper methods for both
/// TypeScriptSupport and TsxSupport. The AST node types are identical between
/// TS and TSX grammars — only the parser grammar differs.
#[cfg(feature = "tree-sitter-ts")]
macro_rules! impl_ts_language_support {
    ($struct_name:ident, $extensions:expr) => {
        impl LanguageSupport for $struct_name {
            fn file_extensions(&self) -> &[&'static str] {
                $extensions
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

        impl $struct_name {
            fn extract_signatures_from_node(
                &self,
                source: &str,
                node: &tree_sitter::Node,
                _visibility: Visibility,
                signatures: &mut Vec<Signature>,
            ) {
                match node.kind() {
                    "function_declaration" | "generator_function_declaration" => {
                        if let Some(sig) = self.extract_function_signature(source, node) {
                            signatures.push(sig);
                        }
                    }
                    "class_declaration" => {
                        if let Some(sig) = self.extract_class_signature(source, node) {
                            signatures.push(sig);
                        }
                    }
                    "interface_declaration" => {
                        if let Some(sig) = self.extract_interface_signature(source, node) {
                            signatures.push(sig);
                        }
                    }
                    "type_alias_declaration" => {
                        if let Some(sig) = self.extract_type_alias_signature(source, node) {
                            signatures.push(sig);
                        }
                    }
                    "enum_declaration" => {
                        if let Some(sig) = self.extract_enum_signature(source, node) {
                            signatures.push(sig);
                        }
                    }
                    "lexical_declaration" => {
                        self.extract_variable_declarations(source, node, signatures);
                    }
                    "export_statement" => {
                        // Extract signatures from exported declarations.
                        // Return early — do NOT recurse into children of export_statement,
                        // because extract_export_signatures already walks them.
                        // Without this guard, exported declarations get extracted twice.
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
                    "function_declaration" | "function_expression" | "arrow_function" => {
                        structure.functions += 1;
                    }
                    "class_declaration" | "class_expression" => {
                        structure.classes += 1;
                    }
                    "interface_declaration" => {
                        structure.interfaces += 1;
                    }
                    "type_alias_declaration" => {
                        structure.type_aliases += 1;
                    }
                    "enum_declaration" => {
                        structure.enums += 1;
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
                let return_type = self.find_child_text(node, "type_annotation", source);

                // Use byte-slicing to preserve type params, access modifiers, and return types
                let full_sig = slice_signature_before_body(source, node, &["statement_block"])
                    .unwrap_or_else(|| match (params.as_ref(), return_type.as_ref()) {
                        (Some(p), Some(r)) => format!("function {}{} {}", name, p, r),
                        (Some(p), None) => format!("function {}{}", name, p),
                        (None, Some(r)) => format!("function {}() {}", name, r),
                        (None, None) => format!("function {}()", name),
                    });

                Some(Signature {
                    kind: SignatureKind::Function,
                    name,
                    params,
                    return_type,
                    visibility: Visibility::All,
                    line_number: node.start_position().row + 1,
                    full_signature: full_sig,
                })
            }

            fn extract_class_signature(&self, source: &str, node: &tree_sitter::Node) -> Option<Signature> {
                let name = self
                    .find_child_text(node, "type_identifier", source)
                    .or_else(|| self.find_child_text(node, "identifier", source))?;

                // Use byte-slicing to preserve extends/implements and generics
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

            fn extract_interface_signature(
                &self,
                source: &str,
                node: &tree_sitter::Node,
            ) -> Option<Signature> {
                let name = self.find_child_text(node, "type_identifier", source)?;

                // Use byte-slicing to preserve extends and generics
                let full_sig = slice_signature_before_body(source, node, &["object_type", "interface_body"])
                    .unwrap_or_else(|| format!("interface {}", name));

                Some(Signature {
                    kind: SignatureKind::Interface,
                    name,
                    params: None,
                    return_type: None,
                    visibility: Visibility::All,
                    line_number: node.start_position().row + 1,
                    full_signature: full_sig,
                })
            }

            fn extract_type_alias_signature(
                &self,
                source: &str,
                node: &tree_sitter::Node,
            ) -> Option<Signature> {
                let name = self.find_child_text(node, "type_identifier", source)?;

                let full_sig = format!("type {}", name);

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

            fn extract_enum_signature(&self, source: &str, node: &tree_sitter::Node) -> Option<Signature> {
                let name = self
                    .find_child_text(node, "identifier", source)
                    .or_else(|| self.find_child_text(node, "type_identifier", source))?;

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

            fn extract_variable_declarations(
                &self,
                source: &str,
                node: &tree_sitter::Node,
                signatures: &mut Vec<Signature>,
            ) {
                let mut cursor = node.walk();
                for child in node.children(&mut cursor) {
                    if child.kind() == "variable_declarator"
                        && let Some(name) = self.find_child_text(&child, "identifier", source)
                    {
                        // Check for arrow function or function assignment
                        let mut inner_cursor = child.walk();
                        let fn_node = child
                            .children(&mut inner_cursor)
                            .find(|c| c.kind() == "arrow_function" || c.kind() == "function");

                        if let Some(fn_child) = fn_node {
                            // Navigate INTO the arrow_function to find its body
                            let body_start = {
                                let mut fn_cursor = fn_child.walk();
                                fn_child
                                    .children(&mut fn_cursor)
                                    .find(|c| c.kind() == "statement_block")
                                    .map(|body| body.start_byte())
                            };

                            let full_signature = if let Some(body_start) = body_start {
                                source[node.start_byte()..body_start].trim_end().to_string()
                            } else {
                                // Expression-body arrow
                                let mut fn_cursor2 = fn_child.walk();
                                let arrow_end = fn_child
                                    .children(&mut fn_cursor2)
                                    .find(|c| c.kind() == "=>")
                                    .map(|arrow| arrow.end_byte());

                                if let Some(end) = arrow_end {
                                    source[node.start_byte()..end].trim_end().to_string()
                                } else {
                                    source[child.start_byte()..fn_child.start_byte()]
                                        .trim_end()
                                        .to_string()
                                }
                            };

                            signatures.push(Signature {
                                kind: SignatureKind::Function,
                                name,
                                params: None,
                                return_type: None,
                                visibility: Visibility::All,
                                line_number: child.start_position().row + 1,
                                full_signature,
                            });
                        } else {
                            let type_ann = self.find_child_text(&child, "type_annotation", source);
                            let full_sig = match &type_ann {
                                Some(t) => format!("const {} {}", name, t),
                                None => format!("const {}", name),
                            };
                            signatures.push(Signature {
                                kind: SignatureKind::Constant,
                                name,
                                params: None,
                                return_type: type_ann,
                                visibility: Visibility::All,
                                line_number: child.start_position().row + 1,
                                full_signature: full_sig,
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
                    match child.kind() {
                        "function_declaration" => {
                            if let Some(sig) = self.extract_function_signature(source, &child) {
                                signatures.push(sig);
                            }
                        }
                        "class_declaration" => {
                            if let Some(sig) = self.extract_class_signature(source, &child) {
                                signatures.push(sig);
                            }
                        }
                        "interface_declaration" => {
                            if let Some(sig) = self.extract_interface_signature(source, &child) {
                                signatures.push(sig);
                            }
                        }
                        "lexical_declaration" | "variable_declaration" => {
                            // Capture exported arrow functions: export const foo = () => {}
                            self.extract_variable_declarations(source, &child, signatures);
                        }
                        _ => {}
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
                                | "interface_declaration"
                                | "type_alias_declaration"
                                | "enum_declaration"
                                | "export_statement"
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
    };
}

// TypeScript: .ts, .mts, .cts
#[cfg(feature = "tree-sitter-ts")]
impl_ts_language_support!(TypeScriptSupport, &["ts", "mts", "cts"]);

// TSX: .tsx (uses LANGUAGE_TSX grammar for JSX syntax support)
#[cfg(feature = "tree-sitter-ts")]
impl_ts_language_support!(TsxSupport, &["tsx"]);

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "tree-sitter-ts")]
    use crate::tree_sitter::language_support::LanguageSupport;

    #[test]
    #[cfg(feature = "tree-sitter-ts")]
    fn test_extract_function_signature() {
        let source = "function greet(name: string): void { console.log(name); }";
        let sigs = TypeScriptSupport.extract_signatures(source, Visibility::All);
        assert!(!sigs.is_empty());
        assert_eq!(sigs[0].name, "greet");
    }

    #[test]
    #[cfg(feature = "tree-sitter-ts")]
    fn test_extract_arrow_function() {
        let source = "const greet = (name: string): void => { console.log(name); }";
        let sigs = TypeScriptSupport.extract_signatures(source, Visibility::All);
        assert!(!sigs.is_empty());
        assert_eq!(sigs[0].name, "greet");
        assert_eq!(sigs[0].kind, SignatureKind::Function);
    }

    #[test]
    #[cfg(feature = "tree-sitter-ts")]
    fn test_extract_class_signature() {
        let source = "class MyComponent { render() { return null; } }";
        let sigs = TypeScriptSupport.extract_signatures(source, Visibility::All);
        assert!(!sigs.is_empty());
        assert_eq!(sigs[0].name, "MyComponent");
    }

    #[test]
    #[cfg(feature = "tree-sitter-ts")]
    fn test_extract_class_with_inheritance() {
        let source = "class MyComponent extends React.Component<Props> { render() {} }";
        let sigs = TypeScriptSupport.extract_signatures(source, Visibility::All);
        assert!(!sigs.is_empty());
        let class_sig = &sigs[0];
        assert_eq!(class_sig.name, "MyComponent");
        assert!(
            class_sig.full_signature.contains("extends"),
            "Class signature should preserve extends clause: {}",
            class_sig.full_signature
        );
    }

    #[test]
    #[cfg(feature = "tree-sitter-ts")]
    fn test_extract_interface_signature() {
        let source = "interface User { name: string; age: number; }";
        let sigs = TypeScriptSupport.extract_signatures(source, Visibility::All);
        assert!(!sigs.is_empty());
        assert_eq!(sigs[0].name, "User");
    }

    #[test]
    #[cfg(feature = "tree-sitter-ts")]
    fn test_extract_interface_with_extends() {
        let source = "interface Admin extends User { role: string; }";
        let sigs = TypeScriptSupport.extract_signatures(source, Visibility::All);
        assert!(!sigs.is_empty());
        let iface_sig = &sigs[0];
        assert_eq!(iface_sig.name, "Admin");
        assert!(
            iface_sig.full_signature.contains("extends"),
            "Interface signature should preserve extends clause: {}",
            iface_sig.full_signature
        );
    }

    #[test]
    #[cfg(feature = "tree-sitter-ts")]
    fn test_no_duplicate_exported_signatures() {
        let source = "export function foo(): void { }\nexport class Bar { }";
        let sigs = TypeScriptSupport.extract_signatures(source, Visibility::All);
        let names: Vec<&str> = sigs.iter().map(|s| s.name.as_str()).collect();
        // Each should appear exactly once, not duplicated
        assert_eq!(names.iter().filter(|&&n| n == "foo").count(), 1, "foo should appear once, got: {:?}", names);
        assert_eq!(names.iter().filter(|&&n| n == "Bar").count(), 1, "Bar should appear once, got: {:?}", names);
    }

    #[test]
    #[cfg(feature = "tree-sitter-ts")]
    fn test_parse_valid_typescript() {
        let source = "function foo(): void { }";
        let tree = TypeScriptSupport.parse(source);
        assert!(tree.is_some());
    }

    #[test]
    #[cfg(feature = "tree-sitter-ts")]
    fn test_find_truncation_point() {
        let source = "function foo(): void { }\nfunction bar(): void { }";
        let point = TypeScriptSupport.find_truncation_point(source, 1000);
        assert_eq!(point, source.len());
    }

    #[test]
    #[cfg(feature = "tree-sitter-ts")]
    fn test_file_extensions_ts() {
        assert!(TypeScriptSupport.supports_extension("ts"));
        assert!(!TypeScriptSupport.supports_extension("tsx")); // TSX handled separately
        assert!(!TypeScriptSupport.supports_extension("js"));
    }

    #[test]
    #[cfg(feature = "tree-sitter-ts")]
    fn test_file_extensions_tsx() {
        assert!(TsxSupport.supports_extension("tsx"));
        assert!(!TsxSupport.supports_extension("ts"));
    }

    #[test]
    #[cfg(feature = "tree-sitter-ts")]
    fn test_tsx_parses_jsx_syntax() {
        let source = "const App = () => { return <div className=\"app\">Hello</div>; }";
        let tree = TsxSupport.parse(source);
        assert!(tree.is_some(), "TSX parser should handle JSX syntax");
    }

    #[test]
    #[cfg(feature = "tree-sitter-ts")]
    fn test_ts_parser_rejects_jsx_syntax() {
        // The TS parser should still parse but may produce errors for JSX
        let source = "const App = () => { return <div>Hello</div>; }";
        let tree = TypeScriptSupport.parse(source);
        // It may parse but with errors — the important thing is TsxSupport handles it correctly
        if let Some(t) = tree {
            // TS parser should flag JSX as an error
            assert!(t.root_node().has_error(), "TS parser should flag JSX as error");
        }
    }
}
