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
}
