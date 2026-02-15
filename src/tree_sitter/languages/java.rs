//! Java language support for tree-sitter.

#[cfg(feature = "tree-sitter-java")]
use tree_sitter::{Parser, Tree};

#[cfg(feature = "tree-sitter-java")]
use crate::tree_sitter::language_support::{
    CodeStructure, LanguageSupport, Signature, SignatureKind, Visibility,
    slice_signature_before_body,
};

pub struct JavaSupport;

#[cfg(feature = "tree-sitter-java")]
impl JavaSupport {
    fn get_language() -> tree_sitter::Language {
        tree_sitter_java::LANGUAGE.into()
    }
}

#[cfg(feature = "tree-sitter-java")]
impl LanguageSupport for JavaSupport {
    fn file_extensions(&self) -> &[&'static str] {
        &["java"]
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

#[cfg(feature = "tree-sitter-java")]
impl JavaSupport {
    fn extract_signatures_from_node(
        &self,
        source: &str,
        node: &tree_sitter::Node,
        visibility: Visibility,
        signatures: &mut Vec<Signature>,
    ) {
        match node.kind() {
            "method_declaration" => {
                if let Some(sig) = self.extract_method_signature(source, node, visibility) {
                    signatures.push(sig);
                }
            }
            "class_declaration" => {
                if let Some(sig) = self.extract_class_signature(source, node, visibility) {
                    signatures.push(sig);
                }
            }
            "interface_declaration" => {
                if let Some(sig) = self.extract_interface_signature(source, node, visibility) {
                    signatures.push(sig);
                }
            }
            "enum_declaration" => {
                if let Some(sig) = self.extract_enum_signature(source, node, visibility) {
                    signatures.push(sig);
                }
            }
            "field_declaration" => {
                if let Some(sig) = self.extract_field_signature(source, node, visibility) {
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
            "method_declaration" => structure.functions += 1,
            "class_declaration" => structure.classes += 1,
            "interface_declaration" => structure.interfaces += 1,
            "enum_declaration" => structure.enums += 1,
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

    #[allow(dead_code)]
    fn get_visibility(&self, _node: &tree_sitter::Node) -> Visibility {
        // Java visibility is determined by modifiers
        // Simplified: check for public/private/protected keywords in AST modifiers
        Visibility::All
    }

    fn extract_method_signature(
        &self,
        source: &str,
        node: &tree_sitter::Node,
        visibility: Visibility,
    ) -> Option<Signature> {
        let vis = self.get_visibility(node);

        if visibility == Visibility::Public && vis != Visibility::Public {
            return None;
        }
        if visibility == Visibility::Private && vis == Visibility::Public {
            return None;
        }

        let name = self.find_child_text(node, "identifier", source)?;
        let params = self.find_child_text(node, "formal_parameters", source);
        let return_type = self
            .find_child_text(node, "type_identifier", source)
            .or_else(|| self.find_child_text_for_type(node, source));

        // Use byte-slicing to preserve annotations, generics, throws, and modifiers
        let full_sig = slice_signature_before_body(source, node, &["block"])
            .unwrap_or_else(|| {
                let mut sig = String::new();
                if vis == Visibility::Public {
                    sig.push_str("public ");
                }
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
            kind: SignatureKind::Method,
            name,
            params,
            return_type,
            visibility: vis,
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
        let vis = self.get_visibility(node);

        if visibility == Visibility::Public && vis != Visibility::Public {
            return None;
        }
        if visibility == Visibility::Private && vis == Visibility::Public {
            return None;
        }

        let name = self.find_child_text(node, "identifier", source)?;

        let mut full_sig = String::new();
        if vis == Visibility::Public {
            full_sig.push_str("public ");
        }
        full_sig.push_str("class ");
        full_sig.push_str(&name);

        Some(Signature {
            kind: SignatureKind::Class,
            name,
            params: None,
            return_type: None,
            visibility: vis,
            line_number: node.start_position().row + 1,
            full_signature: full_sig,
        })
    }

    fn extract_interface_signature(
        &self,
        source: &str,
        node: &tree_sitter::Node,
        visibility: Visibility,
    ) -> Option<Signature> {
        let vis = self.get_visibility(node);

        if visibility == Visibility::Public && vis != Visibility::Public {
            return None;
        }
        if visibility == Visibility::Private && vis == Visibility::Public {
            return None;
        }

        let name = self.find_child_text(node, "identifier", source)?;

        let mut full_sig = String::new();
        if vis == Visibility::Public {
            full_sig.push_str("public ");
        }
        full_sig.push_str("interface ");
        full_sig.push_str(&name);

        Some(Signature {
            kind: SignatureKind::Interface,
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
        visibility: Visibility,
    ) -> Option<Signature> {
        let vis = self.get_visibility(node);

        if visibility == Visibility::Public && vis != Visibility::Public {
            return None;
        }
        if visibility == Visibility::Private && vis == Visibility::Public {
            return None;
        }

        let name = self.find_child_text(node, "identifier", source)?;

        let mut full_sig = String::new();
        if vis == Visibility::Public {
            full_sig.push_str("public ");
        }
        full_sig.push_str("enum ");
        full_sig.push_str(&name);

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

    fn extract_field_signature(
        &self,
        source: &str,
        node: &tree_sitter::Node,
        visibility: Visibility,
    ) -> Option<Signature> {
        let vis = self.get_visibility(node);

        if visibility == Visibility::Public && vis != Visibility::Public {
            return None;
        }
        if visibility == Visibility::Private && vis == Visibility::Public {
            return None;
        }

        let name = self.find_child_text(node, "identifier", source)?;
        let full_signature = format!("field {}", &name);

        Some(Signature {
            kind: SignatureKind::Constant,
            name,
            params: None,
            return_type: None,
            visibility: vis,
            line_number: node.start_position().row + 1,
            full_signature,
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
                return Some(source[child.start_byte()..child.end_byte()].to_string());
            }
        }
        None
    }

    fn find_child_text_for_type(&self, node: &tree_sitter::Node, source: &str) -> Option<String> {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if child.kind() == "void_type"
                || child.kind() == "integral_type"
                || child.kind() == "boolean_type"
            {
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
                    "method_declaration"
                        | "class_declaration"
                        | "interface_declaration"
                        | "enum_declaration"
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
public class HelloWorld {
    public static void main(String[] args) {
        System.out.println("Hello");
    }
}
}
"#;

        let signatures = JavaSupport.extract_signatures(source, Visibility::All);
        let classes: Vec<_> = signatures
            .iter()
            .filter(|s| s.kind == SignatureKind::Class)
            .collect();
        assert!(!classes.is_empty());
        assert_eq!(classes[0].name, "HelloWorld");
    }

    #[test]
    fn test_extract_method_signature() {
        let source = r#"
public class Calculator {
    public int add(int a, int b) {
        return a + b;
    }

    private double multiply(double x, double y) {
        return x * y;
    }
}
"#;

        let signatures = JavaSupport.extract_signatures(source, Visibility::All);
        let methods: Vec<_> = signatures
            .iter()
            .filter(|s| s.kind == SignatureKind::Method || s.kind == SignatureKind::Function)
            .collect();
        assert!(methods.len() >= 2);
    }

    #[test]
    fn test_extract_interface_signature() {
        let source = r#"
public interface Printable {
    void print();
    String format(String template);
}
"#;

        let signatures = JavaSupport.extract_signatures(source, Visibility::All);
        let interfaces: Vec<_> = signatures
            .iter()
            .filter(|s| s.kind == SignatureKind::Interface)
            .collect();
        assert!(!interfaces.is_empty());
        assert_eq!(interfaces[0].name, "Printable");
    }

    #[test]
    fn test_extract_enum_signature() {
        let source = r#"
public enum Color {
    RED, GREEN, BLUE;
}
"#;

        let signatures = JavaSupport.extract_signatures(source, Visibility::All);
        let enums: Vec<_> = signatures
            .iter()
            .filter(|s| s.kind == SignatureKind::Enum)
            .collect();
        assert!(!enums.is_empty());
        assert_eq!(enums[0].name, "Color");
    }

    #[test]
    fn test_extract_class_with_inheritance() {
        let source = r#"
public class Dog extends Animal implements Runnable {
    public void run() {}
}
"#;

        let signatures = JavaSupport.extract_signatures(source, Visibility::All);
        let classes: Vec<_> = signatures
            .iter()
            .filter(|s| s.kind == SignatureKind::Class)
            .collect();
        assert!(!classes.is_empty());
        assert_eq!(classes[0].name, "Dog");
    }

    #[test]
    fn test_extract_structure() {
        let source = r#"
import java.util.List;
import java.util.Map;

public class App {
    public void doStuff() {}
    private void helper() {}
}

interface Printable {
    void print();
}

enum Status { ACTIVE, INACTIVE }
"#;

        let structure = JavaSupport.extract_structure(source);
        assert!(structure.functions >= 2);
        assert!(structure.classes >= 1);
        assert!(structure.interfaces >= 1);
        assert!(structure.enums >= 1);
        assert!(structure.imports.len() >= 2);
    }

    #[test]
    fn test_parse_valid_java() {
        let source = "public class Main { public static void main(String[] args) {} }";
        let tree = JavaSupport.parse(source);
        assert!(tree.is_some());
    }

    #[test]
    fn test_find_truncation_point() {
        let source = "public class Main { public static void main(String[] args) {} }";
        let point = JavaSupport.find_truncation_point(source, 1000);
        assert_eq!(point, source.len());
    }

    #[test]
    fn test_file_extensions() {
        assert!(JavaSupport.supports_extension("java"));
        assert!(!JavaSupport.supports_extension("rs"));
    }
}

