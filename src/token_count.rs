use ignore::DirEntry;
use std::collections::BTreeMap;
use std::path::Path;
use std::str::FromStr;
use std::sync::LazyLock;
/// Token counting utilities for estimating LLM token usage
use tiktoken_rs::{CoreBPE, cl100k_base, o200k_base};

/// Tokenizer encoding used for token estimation and budgeting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Encoding {
    /// `o200k_base` — GPT-4o / o-series, and the closest match for current
    /// frontier models. Default, because `cl100k_base` under-counts these.
    #[default]
    O200kBase,
    /// `cl100k_base` — GPT-4 / GPT-3.5-turbo.
    Cl100kBase,
}

impl FromStr for Encoding {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "o200k_base" | "o200k" => Ok(Encoding::O200kBase),
            "cl100k_base" | "cl100k" => Ok(Encoding::Cl100kBase),
            _ => Err(()),
        }
    }
}

// Initialize each tokenizer once and reuse it (std::sync::LazyLock — once_cell
// was dropped in the v0.10 dependency diet; LazyLock is stable since 1.80
// and the crate MSRV is 1.89).
static O200K_BASE: LazyLock<CoreBPE> = LazyLock::new(|| o200k_base().unwrap());
static CL100K_BASE: LazyLock<CoreBPE> = LazyLock::new(|| cl100k_base().unwrap());

impl Encoding {
    fn bpe(self) -> &'static CoreBPE {
        match self {
            Encoding::O200kBase => &O200K_BASE,
            Encoding::Cl100kBase => &CL100K_BASE,
        }
    }
}

/// Estimates the number of tokens in a text string using the given encoding.
pub fn estimate_tokens(encoding: Encoding, text: &str) -> usize {
    encoding.bpe().encode_with_special_tokens(text).len()
}

/// Counts the tokens that would be generated for a file.
///
/// Renders the file through the same `process_file` path the document uses, then
/// tokenizes the result — so the `--token-count` preview matches the produced
/// document, including encoding transcoding and tree-sitter signature/structure
/// enrichment, instead of re-reading raw bytes through a divergent code path (B9).
pub fn count_file_tokens(
    base_path: &Path,
    entry: &DirEntry,
    line_numbers: bool,
    encoding: Encoding,
    encoding_strategy: Option<&str>,
    ts_config: &crate::markdown::TreeSitterConfig,
) -> usize {
    let mut buf: Vec<u8> = Vec::new();
    if crate::markdown::process_file(
        base_path,
        entry.path(),
        &mut buf,
        line_numbers,
        encoding_strategy,
        ts_config,
    )
    .is_err()
    {
        // A file that fails to render contributes nothing to the document.
        return 0;
    }
    estimate_tokens(encoding, &String::from_utf8_lossy(&buf))
}

/// Counts the tokens that would be generated for the entire file tree section
pub fn count_tree_tokens(
    tree: &BTreeMap<String, crate::tree::FileNode>,
    depth: usize,
    encoding: Encoding,
) -> usize {
    let mut token_count = 0;

    // Add tokens for indentation
    let indent = "  ".repeat(depth);

    for (name, node) in tree {
        match node {
            crate::tree::FileNode::File => {
                token_count += estimate_tokens(encoding, &format!("{}- 📄 {}\n", indent, name));
            }
            crate::tree::FileNode::Directory(children) => {
                token_count += estimate_tokens(encoding, &format!("{}- 📁 {}\n", indent, name));
                token_count += count_tree_tokens(children, depth + 1, encoding);
            }
        }
    }

    token_count
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    #[test]
    fn test_estimate_tokens() {
        // Test with a simple string
        let text = "Hello, world!";
        let tokens = estimate_tokens(Encoding::Cl100kBase, text);
        // "Hello, world!" is 4 tokens with cl100k_base
        assert_eq!(tokens, 4);

        // Test with code-like content
        let code_text = "fn main() {\n    println!(\"Hello, world!\");\n}";
        let tokens = estimate_tokens(Encoding::Cl100kBase, code_text);
        // This specific code snippet is 12 tokens with cl100k_base
        assert_eq!(tokens, 12);
    }

    #[test]
    fn test_encoding_default_and_parse() {
        assert_eq!(Encoding::default(), Encoding::O200kBase);
        assert_eq!("o200k_base".parse::<Encoding>(), Ok(Encoding::O200kBase));
        assert_eq!("cl100k_base".parse::<Encoding>(), Ok(Encoding::Cl100kBase));
        assert_eq!("O200K".parse::<Encoding>(), Ok(Encoding::O200kBase));
        assert!("bogus".parse::<Encoding>().is_err());
        // Both encodings tokenize non-empty text to a non-zero count.
        assert!(estimate_tokens(Encoding::O200kBase, "hello world") > 0);
        assert!(estimate_tokens(Encoding::Cl100kBase, "hello world") > 0);
    }

    #[test]
    fn test_count_tree_tokens() {
        // Create a simple tree structure
        let mut tree = BTreeMap::new();
        tree.insert("file1.rs".to_string(), crate::tree::FileNode::File);

        let mut subdir = BTreeMap::new();
        subdir.insert("file2.md".to_string(), crate::tree::FileNode::File);
        tree.insert("src".to_string(), crate::tree::FileNode::Directory(subdir));

        let tokens = count_tree_tokens(&tree, 0, Encoding::Cl100kBase);
        // "- 📄 file1.rs\n" -> 8 tokens
        // "- 📁 src\n" -> 6 tokens
        // "  - 📄 file2.md\n" -> 9 tokens
        // Total should be 23 tokens
        assert_eq!(tokens, 23);
    }

    #[test]
    fn test_token_estimation_format_consistency() {
        use tempfile::tempdir;

        let dir = tempdir().unwrap();
        let test_file = dir.path().join("test.rs");
        std::fs::write(&test_file, "fn main() {}\n").unwrap();

        let entry = ignore::WalkBuilder::new(&test_file)
            .build()
            .next()
            .unwrap()
            .unwrap();

        // Estimate tokens for the file
        let estimated_tokens = count_file_tokens(
            dir.path(),
            &entry,
            false,
            Encoding::Cl100kBase,
            None,
            &crate::markdown::TreeSitterConfig::default(),
        );

        // Generate actual markdown content
        let mut actual_content = Vec::new();
        crate::markdown::process_file(
            dir.path(),
            &test_file,
            &mut actual_content,
            false,
            None,
            &crate::markdown::TreeSitterConfig::default(),
        )
        .unwrap();
        let actual_content_str = String::from_utf8(actual_content).unwrap();

        // Count actual tokens
        let actual_tokens = estimate_tokens(Encoding::Cl100kBase, &actual_content_str);

        // The estimation should be close to actual (within a reasonable margin)
        // Allow for some variance due to timestamp differences and minor formatting
        let difference = actual_tokens.abs_diff(estimated_tokens);

        // Should be within 10% or 20 tokens difference (whichever is larger)
        let max_allowed_difference = std::cmp::max(actual_tokens / 10, 20);

        assert!(
            difference <= max_allowed_difference,
            "Token estimation {} differs too much from actual {} (difference: {})",
            estimated_tokens,
            actual_tokens,
            difference
        );
    }

    #[test]
    fn test_estimate_tokens_empty_string() {
        let tokens = estimate_tokens(Encoding::Cl100kBase, "");
        assert_eq!(tokens, 0);
    }

    #[test]
    fn test_estimate_tokens_whitespace_only() {
        let tokens = estimate_tokens(Encoding::Cl100kBase, "   \n\t  ");
        assert!(tokens > 0); // Whitespace still counts as tokens
    }

    #[test]
    fn test_estimate_tokens_unicode() {
        let tokens = estimate_tokens(Encoding::Cl100kBase, "Hello 世界! 🌍");
        assert!(tokens > 0);
        // Unicode characters may be encoded as multiple tokens
        assert!(tokens >= 4);
    }

    #[test]
    fn test_count_file_tokens_with_line_numbers() {
        use tempfile::tempdir;

        let dir = tempdir().unwrap();
        let test_file = dir.path().join("test.rs");
        std::fs::write(&test_file, "line 1\nline 2\nline 3").unwrap();

        let entry = ignore::WalkBuilder::new(&test_file)
            .build()
            .next()
            .unwrap()
            .unwrap();

        let tokens_without_line_numbers = count_file_tokens(
            dir.path(),
            &entry,
            false,
            Encoding::Cl100kBase,
            None,
            &crate::markdown::TreeSitterConfig::default(),
        );
        let tokens_with_line_numbers = count_file_tokens(
            dir.path(),
            &entry,
            true,
            Encoding::Cl100kBase,
            None,
            &crate::markdown::TreeSitterConfig::default(),
        );

        // With line numbers should have more tokens due to line number prefixes
        assert!(tokens_with_line_numbers > tokens_without_line_numbers);
    }

    #[test]
    fn test_count_file_tokens_unreadable_file() {
        use tempfile::tempdir;

        let dir = tempdir().unwrap();
        let test_file = dir.path().join("nonexistent.txt");

        // Create a mock DirEntry for a file that doesn't exist
        // This simulates what happens when a file is deleted between discovery and processing
        let walker = ignore::WalkBuilder::new(dir.path());
        let mut found_entry = None;

        // Create the file temporarily to get a DirEntry
        std::fs::write(&test_file, "temp").unwrap();
        for entry in walker.build() {
            if let Ok(entry) = entry
                && entry.path() == test_file
            {
                found_entry = Some(entry);
                break;
            }
        }

        // Now delete the file
        std::fs::remove_file(&test_file).unwrap();

        if let Some(entry) = found_entry {
            let tokens = count_file_tokens(
                dir.path(),
                &entry,
                false,
                Encoding::Cl100kBase,
                None,
                &crate::markdown::TreeSitterConfig::default(),
            );
            // A file that cannot be read renders to nothing, so it contributes 0
            // tokens — matching what the produced document would actually contain.
            assert_eq!(tokens, 0);
        }
    }

    #[test]
    fn test_count_tree_tokens_empty_tree() {
        let tree = BTreeMap::new();
        let tokens = count_tree_tokens(&tree, 0, Encoding::Cl100kBase);
        assert_eq!(tokens, 0);
    }

    #[test]
    fn test_count_tree_tokens_nested_directories() {
        let mut tree = BTreeMap::new();

        // Create deeply nested structure
        let mut level3 = BTreeMap::new();
        level3.insert("deep_file.txt".to_string(), crate::tree::FileNode::File);

        let mut level2 = BTreeMap::new();
        level2.insert(
            "level3".to_string(),
            crate::tree::FileNode::Directory(level3),
        );

        let mut level1 = BTreeMap::new();
        level1.insert(
            "level2".to_string(),
            crate::tree::FileNode::Directory(level2),
        );

        tree.insert(
            "level1".to_string(),
            crate::tree::FileNode::Directory(level1),
        );

        let tokens = count_tree_tokens(&tree, 0, Encoding::Cl100kBase);
        assert!(tokens > 0);

        // Should account for indentation at different levels
        let tokens_with_depth = count_tree_tokens(&tree, 2, Encoding::Cl100kBase);
        assert!(tokens_with_depth > tokens); // More indentation = more tokens
    }

    #[test]
    fn test_count_tree_tokens_mixed_content() {
        let mut tree = BTreeMap::new();

        // Add files with various name lengths and characters
        tree.insert("a.txt".to_string(), crate::tree::FileNode::File);
        tree.insert(
            "very_long_filename_with_underscores.rs".to_string(),
            crate::tree::FileNode::File,
        );
        tree.insert("файл.txt".to_string(), crate::tree::FileNode::File); // Unicode filename

        let mut subdir = BTreeMap::new();
        subdir.insert("nested.md".to_string(), crate::tree::FileNode::File);
        tree.insert(
            "directory".to_string(),
            crate::tree::FileNode::Directory(subdir),
        );

        let tokens = count_tree_tokens(&tree, 0, Encoding::Cl100kBase);
        assert!(tokens > 0);

        // Verify it handles unicode filenames without crashing
        assert!(tokens > 20); // Should be substantial given the content
    }
}
