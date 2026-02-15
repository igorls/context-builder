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

    #[test]
    fn test_format_empty_signatures() {
        let output = format_signatures_as_markdown(&[], "rust");
        assert!(output.is_empty());
    }
}
