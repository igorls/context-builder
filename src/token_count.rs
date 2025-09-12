/// Token counting utilities for estimating LLM token usage
use ignore::DirEntry;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

/// Estimates the number of tokens in a text string
///
/// This uses a simple heuristic that 1 token is roughly 4 characters for code content
/// and 3 characters for text content (like markdown, documentation, etc.)
pub fn estimate_tokens(text: &str) -> usize {
    // For code files, we'll use a ratio of 1 token per 4 characters
    // For text files, we'll use a ratio of 1 token per 3 characters
    // This is a rough estimation - actual token counts will vary based on the model's tokenizer

    // Simple heuristic: if the text has many non-alphanumeric characters,
    // it's likely code and we'll use the 4:1 ratio
    let non_alpha_count = text
        .chars()
        .filter(|c| !c.is_alphanumeric() && !c.is_whitespace())
        .count();
    let alpha_count = text.chars().filter(|c| c.is_alphanumeric()).count();

    if text.lines().count() > 5 && non_alpha_count > alpha_count / 2 {
        // Likely code content
        text.chars().count() / 4
    } else {
        // Likely text content
        text.chars().count() / 3
    }
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
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_estimate_tokens() {
        // Test with a simple string
        let text = "Hello, world!";
        let tokens = estimate_tokens(text);
        // 13 characters, no special heuristic applies (text content)
        // 13 chars / 3 ≈ 4 tokens
        assert!(tokens > 0); // At least verify it returns some tokens

        // Test with code-like content
        let code_text = "fn main() {\n    println!(\"Hello, world!\");\n}";
        let tokens = estimate_tokens(code_text);
        // Function should return some tokens based on its logic
        assert!(tokens > 0); // At least verify it returns some tokens
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
        // This should count tokens for:
        // "- 📄 file1.rs\n" (15 chars / 3 ≈ 5 tokens)
        // "- 📁 src\n" (9 chars / 3 ≈ 3 tokens)
        // "  - 📄 file2.md\n" (20 chars / 3 ≈ 6 tokens)
        // Total should be around 14 tokens
        assert!(tokens > 0);
    }
}
