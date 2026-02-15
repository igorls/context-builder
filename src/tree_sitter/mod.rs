//! Tree-sitter integration for intelligent code parsing.
//!
//! This module provides:
//! - Signature extraction (function/class signatures without bodies)
//! - Smart truncation (truncate at AST boundaries)
//! - Structure extraction (imports, exports, symbol counts)
//!
//! Feature-gated: Only compiled when one of the tree-sitter-* features is enabled.

#[cfg(feature = "tree-sitter-base")]
pub mod language_support;

#[cfg(feature = "tree-sitter-base")]
pub mod signatures;

#[cfg(feature = "tree-sitter-base")]
pub mod structure;

#[cfg(feature = "tree-sitter-base")]
pub mod truncation;

#[cfg(feature = "tree-sitter-base")]
pub mod languages;

#[cfg(feature = "tree-sitter-base")]
use std::path::Path;

#[cfg(feature = "tree-sitter-base")]
pub use language_support::{CodeStructure, LanguageSupport, Signature, SignatureKind, Visibility};

#[cfg(feature = "tree-sitter-base")]
pub use signatures::extract_signatures;

#[cfg(feature = "tree-sitter-base")]
pub use structure::extract_structure;

#[cfg(feature = "tree-sitter-base")]
pub use truncation::find_truncation_point;

/// Check if tree-sitter is available for a given file extension.
#[cfg(feature = "tree-sitter-base")]
pub fn is_supported_extension(ext: &str) -> bool {
    languages::get_language_support(ext).is_some()
}

#[cfg(not(feature = "tree-sitter-base"))]
pub fn is_supported_extension(_ext: &str) -> bool {
    false
}

/// Extract file extension from a path.
#[cfg(feature = "tree-sitter-base")]
fn get_extension(path: &Path) -> Option<String> {
    path.extension()
        .and_then(|e| e.to_str())
        .map(|s| s.to_lowercase())
}

/// Get language support for a file path.
#[cfg(feature = "tree-sitter-base")]
pub fn get_language_for_path(path: &Path) -> Option<&'static dyn LanguageSupport> {
    let ext = get_extension(path)?;
    languages::get_language_support(&ext)
}

/// Extract signatures from source code for a given file extension.
#[cfg(feature = "tree-sitter-base")]
pub fn extract_signatures_for_file(
    source: &str,
    ext: &str,
    visibility_filter: Visibility,
) -> Option<Vec<Signature>> {
    let support = languages::get_language_support(ext)?;
    Some(extract_signatures(source, support, visibility_filter))
}

/// Extract structure from source code for a given file extension.
#[cfg(feature = "tree-sitter-base")]
pub fn extract_structure_for_file(source: &str, ext: &str) -> Option<CodeStructure> {
    let support = languages::get_language_support(ext)?;
    Some(extract_structure(source, support))
}

/// Find a smart truncation point for a given file extension.
#[cfg(feature = "tree-sitter-base")]
pub fn find_smart_truncation_point(source: &str, max_bytes: usize, ext: &str) -> Option<usize> {
    let support = languages::get_language_support(ext)?;
    Some(find_truncation_point(source, max_bytes, support))
}

#[cfg(not(feature = "tree-sitter-base"))]
pub fn extract_signatures_for_file(
    _source: &str,
    _ext: &str,
    _visibility_filter: (),
) -> Option<()> {
    None
}

#[cfg(not(feature = "tree-sitter-base"))]
pub fn extract_structure_for_file(_source: &str, _ext: &str) -> Option<()> {
    None
}

#[cfg(not(feature = "tree-sitter-base"))]
pub fn find_smart_truncation_point(_source: &str, _max_bytes: usize, _ext: &str) -> Option<usize> {
    None
}

#[cfg(not(feature = "tree-sitter-base"))]
pub fn get_language_for_path(_path: &std::path::Path) -> Option<()> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "tree-sitter-base")]
    fn test_is_supported_extension() {
        #[cfg(feature = "tree-sitter-rust")]
        assert!(is_supported_extension("rs"));
        #[cfg(feature = "tree-sitter-python")]
        assert!(is_supported_extension("py"));
        #[cfg(feature = "tree-sitter-js")]
        assert!(is_supported_extension("js"));
        assert!(!is_supported_extension("xyz"));
    }

    #[test]
    #[cfg(not(feature = "tree-sitter-base"))]
    fn test_no_tree_sitter_support() {
        assert!(!is_supported_extension("rs"));
        assert!(!is_supported_extension("py"));
    }

    #[test]
    #[cfg(feature = "tree-sitter-base")]
    fn test_get_extension() {
        assert_eq!(get_extension(Path::new("foo.rs")), Some("rs".to_string()));
        assert_eq!(get_extension(Path::new("foo.RS")), Some("rs".to_string()));
        assert_eq!(get_extension(Path::new("foo.PY")), Some("py".to_string()));
        assert_eq!(get_extension(Path::new("foo")), None);
        assert_eq!(get_extension(Path::new(".gitignore")), None);
    }

    #[test]
    #[cfg(feature = "tree-sitter-rust")]
    fn test_extract_signatures_for_file_rust() {
        let source = "pub fn hello() { }\nfn world() { }";
        let sigs = extract_signatures_for_file(source, "rs", Visibility::All);
        assert!(sigs.is_some());
        let sigs = sigs.unwrap();
        assert!(sigs.len() >= 2);
    }

    #[test]
    #[cfg(feature = "tree-sitter-base")]
    fn test_extract_signatures_for_file_unsupported() {
        let sigs = extract_signatures_for_file("anything", "xyz", Visibility::All);
        assert!(sigs.is_none());
    }

    #[test]
    #[cfg(feature = "tree-sitter-rust")]
    fn test_extract_structure_for_file_rust() {
        let source = "use std::io;\nfn foo() { }\nstruct Bar { }\nenum Baz { A, B }";
        let structure = extract_structure_for_file(source, "rs");
        assert!(structure.is_some());
        let s = structure.unwrap();
        assert!(s.functions >= 1);
        assert!(s.structs >= 1);
        assert!(s.enums >= 1);
    }

    #[test]
    #[cfg(feature = "tree-sitter-base")]
    fn test_extract_structure_for_file_unsupported() {
        let structure = extract_structure_for_file("anything", "xyz");
        assert!(structure.is_none());
    }

    #[test]
    #[cfg(feature = "tree-sitter-rust")]
    fn test_find_smart_truncation_point_within_bounds() {
        let source = "fn foo() { }\nfn bar() { }\nfn baz() { }";
        // Max bytes larger than source — should return source length
        let point = find_smart_truncation_point(source, 1000, "rs");
        assert!(point.is_some());
        assert_eq!(point.unwrap(), source.len());
    }

    #[test]
    #[cfg(feature = "tree-sitter-rust")]
    fn test_find_smart_truncation_point_truncated() {
        let source = "fn foo() {\n    let x = 1;\n}\nfn bar() {\n    let y = 2;\n}";
        let point = find_smart_truncation_point(source, 15, "rs");
        assert!(point.is_some());
        // Should truncate at an AST boundary, not mid-token
        assert!(point.unwrap() <= source.len());
    }

    #[test]
    #[cfg(feature = "tree-sitter-base")]
    fn test_find_smart_truncation_point_unsupported() {
        let point = find_smart_truncation_point("anything", 100, "xyz");
        assert!(point.is_none());
    }

    #[test]
    #[cfg(feature = "tree-sitter-rust")]
    fn test_get_language_for_path_known() {
        let support = get_language_for_path(Path::new("src/main.rs"));
        assert!(support.is_some());
    }

    #[test]
    #[cfg(feature = "tree-sitter-base")]
    fn test_get_language_for_path_unknown() {
        let support = get_language_for_path(Path::new("README.md"));
        assert!(support.is_none());
    }

    #[test]
    #[cfg(feature = "tree-sitter-base")]
    fn test_get_language_for_path_no_extension() {
        let support = get_language_for_path(Path::new("Makefile"));
        assert!(support.is_none());
    }
}
