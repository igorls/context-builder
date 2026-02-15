//! Signature extraction utilities.

use super::language_support::{LanguageSupport, Signature, Visibility};

/// Extract all signatures from source code.
pub fn extract_signatures(
    source: &str,
    support: &dyn LanguageSupport,
    visibility: Visibility,
) -> Vec<Signature> {
    support.extract_signatures(source, visibility)
}

/// Format signatures as markdown.
pub fn format_signatures_as_markdown(signatures: &[Signature], language: &str) -> String {
    if signatures.is_empty() {
        return String::new();
    }

    let mut output = String::new();
    output.push_str("```");
    output.push_str(language);
    output.push('\n');

    let mut current_kind: Option<&str> = None;

    for sig in signatures {
        let kind_str = match sig.kind {
            super::language_support::SignatureKind::Function
            | super::language_support::SignatureKind::Method => "Functions",
            super::language_support::SignatureKind::Struct
            | super::language_support::SignatureKind::Class => "Structs/Classes",
            super::language_support::SignatureKind::Enum => "Enums",
            super::language_support::SignatureKind::Trait
            | super::language_support::SignatureKind::Interface => "Traits/Interfaces",
            super::language_support::SignatureKind::Impl => "Implementations",
            super::language_support::SignatureKind::Module => "Modules",
            super::language_support::SignatureKind::Constant => "Constants",
            super::language_support::SignatureKind::TypeAlias => "Type Aliases",
            super::language_support::SignatureKind::Macro => "Macros",
        };

        if current_kind != Some(kind_str) {
            if current_kind.is_some() {
                output.push('\n');
            }
            output.push_str("// ");
            output.push_str(kind_str);
            output.push('\n');
            current_kind = Some(kind_str);
        }

        output.push_str(&sig.full_signature);
        output.push('\n');
    }

    output.push_str("```\n");
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::language_support::SignatureKind;

    fn make_sig(kind: SignatureKind, name: &str, full_sig: &str) -> Signature {
        Signature {
            kind,
            name: name.to_string(),
            params: None,
            return_type: None,
            visibility: Visibility::All,
            line_number: 1,
            full_signature: full_sig.to_string(),
        }
    }

    #[test]
    fn test_format_empty_signatures() {
        let output = format_signatures_as_markdown(&[], "rust");
        assert!(output.is_empty());
    }

    #[test]
    fn test_format_single_function() {
        let sigs = vec![make_sig(
            SignatureKind::Function,
            "foo",
            "fn foo(x: i32) -> i32",
        )];
        let output = format_signatures_as_markdown(&sigs, "rust");
        assert!(output.starts_with("```rust\n"));
        assert!(output.contains("// Functions\n"));
        assert!(output.contains("fn foo(x: i32) -> i32\n"));
        assert!(output.ends_with("```\n"));
    }

    #[test]
    fn test_format_multiple_same_kind() {
        let sigs = vec![
            make_sig(SignatureKind::Function, "foo", "fn foo()"),
            make_sig(SignatureKind::Function, "bar", "fn bar()"),
        ];
        let output = format_signatures_as_markdown(&sigs, "rust");
        // "Functions" header should appear only once
        assert_eq!(output.matches("// Functions").count(), 1);
        assert!(output.contains("fn foo()"));
        assert!(output.contains("fn bar()"));
    }

    #[test]
    fn test_format_mixed_kinds() {
        let sigs = vec![
            make_sig(SignatureKind::Function, "foo", "fn foo()"),
            make_sig(SignatureKind::Struct, "Bar", "struct Bar"),
            make_sig(SignatureKind::Enum, "Baz", "enum Baz"),
            make_sig(SignatureKind::Trait, "Qux", "trait Qux"),
            make_sig(SignatureKind::Impl, "ImplBlock", "impl Foo"),
            make_sig(SignatureKind::Module, "mymod", "mod mymod"),
            make_sig(SignatureKind::Constant, "MAX", "const MAX: usize = 100"),
            make_sig(SignatureKind::TypeAlias, "Id", "type Id = u64"),
            make_sig(SignatureKind::Macro, "mymacro", "macro_rules! mymacro"),
        ];
        let output = format_signatures_as_markdown(&sigs, "rust");
        assert!(output.contains("// Functions\n"));
        assert!(output.contains("// Structs/Classes\n"));
        assert!(output.contains("// Enums\n"));
        assert!(output.contains("// Traits/Interfaces\n"));
        assert!(output.contains("// Implementations\n"));
        assert!(output.contains("// Modules\n"));
        assert!(output.contains("// Constants\n"));
        assert!(output.contains("// Type Aliases\n"));
        assert!(output.contains("// Macros\n"));
    }

    #[test]
    fn test_format_method_grouped_with_functions() {
        let sigs = vec![
            make_sig(SignatureKind::Method, "get", "fn get(&self)"),
            make_sig(SignatureKind::Function, "free_func", "fn free_func()"),
        ];
        let output = format_signatures_as_markdown(&sigs, "rust");
        // Both Method and Function map to "Functions"
        assert_eq!(output.matches("// Functions").count(), 1);
    }

    #[test]
    fn test_format_class_grouped_with_struct() {
        let sigs = vec![
            make_sig(SignatureKind::Class, "MyClass", "class MyClass"),
            make_sig(SignatureKind::Struct, "MyStruct", "struct MyStruct"),
        ];
        let output = format_signatures_as_markdown(&sigs, "python");
        // Both Class and Struct map to "Structs/Classes"
        assert_eq!(output.matches("// Structs/Classes").count(), 1);
    }

    #[test]
    fn test_format_interface_grouped_with_trait() {
        let sigs = vec![
            make_sig(SignatureKind::Interface, "IFoo", "interface IFoo"),
            make_sig(SignatureKind::Trait, "Bar", "trait Bar"),
        ];
        let output = format_signatures_as_markdown(&sigs, "typescript");
        assert_eq!(output.matches("// Traits/Interfaces").count(), 1);
    }

    #[test]
    #[cfg(feature = "tree-sitter-rust")]
    fn test_extract_signatures_delegates() {
        let lang = super::super::languages::get_language_support("rs").unwrap();
        let source = "pub fn hello() { }";
        let sigs = extract_signatures(source, lang, Visibility::All);
        assert!(!sigs.is_empty());
        assert_eq!(sigs[0].name, "hello");
    }

    #[test]
    #[cfg(feature = "tree-sitter-rust")]
    fn test_extract_signatures_visibility_filter() {
        let lang = super::super::languages::get_language_support("rs").unwrap();
        let source = "pub fn visible() { }\nfn hidden() { }";
        let sigs = extract_signatures(source, lang, Visibility::Public);
        // Only pub functions should be included
        assert!(sigs.iter().all(|s| s.name == "visible" || s.visibility == Visibility::Public));
    }
}
