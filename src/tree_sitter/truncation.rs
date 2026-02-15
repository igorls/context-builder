//! Smart truncation at AST boundaries.

use super::language_support::LanguageSupport;

/// Find a truncation point that ends at a complete AST node boundary.
///
/// Returns the byte position where the source should be truncated.
/// If no suitable boundary is found within max_bytes, returns max_bytes.
pub fn find_truncation_point(
    source: &str,
    max_bytes: usize,
    support: &dyn LanguageSupport,
) -> usize {
    if source.len() <= max_bytes {
        return source.len();
    }

    support.find_truncation_point(source, max_bytes)
}

/// Check if truncation is needed at a UTF-8 boundary.
pub fn ensure_utf8_boundary(source: &str, position: usize) -> usize {
    if position >= source.len() {
        return source.len();
    }

    let mut pos = position;
    while pos > 0 && !source.is_char_boundary(pos) {
        pos -= 1;
    }
    pos
}

/// Add a truncation notice to the output.
pub fn add_truncation_notice(output: &mut String, truncated_count: usize) {
    output.push_str("\n\n---\n\n");
    if truncated_count > 0 {
        output.push_str(&format!(
            "_Output truncated: {} more items omitted._\n",
            truncated_count
        ));
    } else {
        output.push_str("_Output truncated at code boundary._\n");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ensure_utf8_boundary_ascii() {
        let source = "Hello, world!";
        assert_eq!(ensure_utf8_boundary(source, 5), 5);
        assert_eq!(ensure_utf8_boundary(source, 100), 13);
    }

    #[test]
    fn test_ensure_utf8_boundary_unicode() {
        let source = "Hello, 世界!"; // 4 bytes per Chinese char
        // Position 8 is inside the first Chinese character (starts at 7)
        let boundary = ensure_utf8_boundary(source, 8);
        assert_eq!(boundary, 7); // Should fall back to start of char
    }

    #[test]
    fn test_ensure_utf8_boundary_at_zero() {
        let source = "Hello";
        assert_eq!(ensure_utf8_boundary(source, 0), 0);
    }

    #[test]
    fn test_add_truncation_notice_with_count() {
        let mut output = String::new();
        add_truncation_notice(&mut output, 5);
        assert!(output.contains("---"));
        assert!(output.contains("5 more items omitted"));
    }

    #[test]
    fn test_add_truncation_notice_zero_count() {
        let mut output = String::new();
        add_truncation_notice(&mut output, 0);
        assert!(output.contains("---"));
        assert!(output.contains("truncated at code boundary"));
    }

    #[test]
    #[cfg(feature = "tree-sitter-rust")]
    fn test_find_truncation_point_source_within_limit() {
        let lang = super::super::languages::get_language_support("rs").unwrap();
        let source = "fn foo() { }";
        let point = find_truncation_point(source, 1000, lang);
        assert_eq!(point, source.len());
    }

    #[test]
    #[cfg(feature = "tree-sitter-rust")]
    fn test_find_truncation_point_source_exceeds_limit() {
        let lang = super::super::languages::get_language_support("rs").unwrap();
        let source = "fn foo() {\n    let x = 1;\n    let y = 2;\n}\nfn bar() {\n    let z = 3;\n}\n";
        let point = find_truncation_point(source, 20, lang);
        assert!(point <= source.len());
        assert!(point > 0);
    }
}

