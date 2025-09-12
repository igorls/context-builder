use ignore::DirEntry;
use once_cell::sync::Lazy;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
/// Token counting utilities for estimating LLM token usage
use tiktoken_rs::{CoreBPE, cl100k_base};

// Initialize the tokenizer once and reuse it
static TOKENIZER: Lazy<CoreBPE> = Lazy::new(|| cl100k_base().unwrap());

/// Estimates the number of tokens in a text string using a real tokenizer
pub fn estimate_tokens(text: &str) -> usize {
    TOKENIZER.encode_with_special_tokens(text).len()
}

/// Counts the tokens that would be generated for a file
pub fn count_file_tokens(base_path: &Path, entry: &DirEntry, line_numbers: bool) -> usize {
    let file_path = entry.path();
    let relative_path = file_path.strip_prefix(base_path).unwrap_or(file_path);

    // Start with tokens for the file header (path, size, modified time)
    let mut token_count = estimate_tokens(&format!(
        "\n## File: `{}`\n\n- Size: {} bytes\n- Modified: {}\n\n",
        relative_path.display(),
        entry.metadata().map(|m| m.len()).unwrap_or(0),
        "Unknown"
    )); // Using "Unknown" as placeholder for modified time in estimation

    // Add tokens for the code fences
    token_count += estimate_tokens("```\n```");

    // Try to read file content
    if let Ok(content) = fs::read_to_string(file_path) {
        if line_numbers {
            // When line numbers are enabled, we add the line number prefix to each line
            let lines_with_numbers: String = content
                .lines()
                .enumerate()
                .map(|(i, line)| format!("{:>4} | {}\n", i + 1, line))
                .collect();
            token_count += estimate_tokens(&lines_with_numbers);
        } else {
            token_count += estimate_tokens(&content);
        }
    }

    token_count
}

/// Counts the tokens that would be generated for the entire file tree section
pub fn count_tree_tokens(tree: &BTreeMap<String, crate::tree::FileNode>, depth: usize) -> usize {
    let mut token_count = 0;

    // Add tokens for indentation
    let indent = "  ".repeat(depth);

    for (name, node) in tree {
        match node {
            crate::tree::FileNode::File => {
                token_count += estimate_tokens(&format!("{}- 📄 {}\n", indent, name));
            }
            crate::tree::FileNode::Directory(children) => {
                token_count += estimate_tokens(&format!("{}- 📁 {}\n", indent, name));
                token_count += count_tree_tokens(children, depth + 1);
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
        let tokens = estimate_tokens(text);
        // "Hello, world!" is 4 tokens with cl100k_base
        assert_eq!(tokens, 4);

        // Test with code-like content
        let code_text = "fn main() {\n    println!(\"Hello, world!\");\n}";
        let tokens = estimate_tokens(code_text);
        // This specific code snippet is 12 tokens with cl100k_base
        assert_eq!(tokens, 12);
    }

    #[test]
    fn test_count_tree_tokens() {
        // Create a simple tree structure
        let mut tree = BTreeMap::new();
        tree.insert("file1.rs".to_string(), crate::tree::FileNode::File);

        let mut subdir = BTreeMap::new();
        subdir.insert("file2.md".to_string(), crate::tree::FileNode::File);
        tree.insert("src".to_string(), crate::tree::FileNode::Directory(subdir));

        let tokens = count_tree_tokens(&tree, 0);
        // "- 📄 file1.rs\n" -> 8 tokens
        // "- 📁 src\n" -> 6 tokens
        // "  - 📄 file2.md\n" -> 9 tokens
        // Total should be 23 tokens
        assert_eq!(tokens, 23);
    }
}
