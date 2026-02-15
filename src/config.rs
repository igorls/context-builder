use serde::Deserialize;
use std::fs;
use std::path::Path;

/// Global configuration loaded from `context-builder.toml`.
///
/// Any field left as `None` means "use the CLI default / do not override".
/// Command-line arguments always take precedence over values provided here.
///
/// Example `context-builder.toml`:
/// ```toml
/// output = "context.md"
/// output_folder = "docs"
/// timestamped_output = true
/// auto_diff = true
/// diff_only = true         # Emit only change summary + modified file diffs (no full file bodies)
/// filter = ["rs", "toml"]
/// ignore = ["target", ".git"]
/// line_numbers = false
/// diff_context_lines = 5
/// ```
///
#[derive(Deserialize, Debug, Default, Clone)]
pub struct Config {
    /// Output file name (or base name when `timestamped_output = true`)
    pub output: Option<String>,

    /// File extensions to include (no leading dot, e.g. `rs`, `toml`)
    pub filter: Option<Vec<String>>,

    /// File / directory names to ignore (exact name matches)
    pub ignore: Option<Vec<String>>,

    /// Add line numbers to code blocks
    pub line_numbers: Option<bool>,

    /// Preview only the file tree (no file output)
    pub preview: Option<bool>,

    /// Token counting mode
    pub token_count: Option<bool>,

    /// Optional folder to place the generated output file(s) in
    pub output_folder: Option<String>,

    /// If true, append a UTC timestamp to the output file name (before extension)
    pub timestamped_output: Option<bool>,

    /// Assume "yes" for overwrite / processing confirmations
    pub yes: Option<bool>,

    /// Enable automatic diff generation (requires `timestamped_output = true`)
    pub auto_diff: Option<bool>,

    /// Override number of unified diff context lines (falls back to env or default = 3)
    pub diff_context_lines: Option<usize>,

    /// When true, emit ONLY:
    /// - Header + file tree
    /// - Change Summary
    /// - Per-file diffs for modified files
    ///
    /// Excludes full file contents section entirely. Added files appear only in the
    /// change summary (and are marked Added) but their full content is omitted.
    pub diff_only: Option<bool>,

    /// Encoding handling strategy for non-UTF-8 files.
    /// - "detect": Attempt to detect and transcode to UTF-8 (default)
    /// - "strict": Only include valid UTF-8 files, skip others
    /// - "skip": Skip all non-UTF-8 files without transcoding attempts
    pub encoding_strategy: Option<String>,

    /// Maximum token budget for the output. Files are truncated/skipped when exceeded.
    pub max_tokens: Option<usize>,

    /// Extract function/class signatures only (requires tree-sitter feature)
    pub signatures: Option<bool>,

    /// Extract code structure (imports, exports, symbol counts) - requires tree-sitter feature
    pub structure: Option<bool>,

    /// Truncation mode for max-tokens: "smart" (AST boundaries) or "byte"
    pub truncate: Option<String>,

    /// Filter signatures by visibility: "all", "public", or "private"
    pub visibility: Option<String>,
}

/// Load configuration from `context-builder.toml` in the current working directory.
/// Returns `None` if the file does not exist or cannot be parsed.
pub fn load_config() -> Option<Config> {
    let config_path = Path::new("context-builder.toml");
    if config_path.exists() {
        let content = fs::read_to_string(config_path).ok()?;
        match toml::from_str(&content) {
            Ok(config) => Some(config),
            Err(e) => {
                eprintln!(
                    "⚠️  Failed to parse context-builder.toml: {}. Config will be ignored.",
                    e
                );
                None
            }
        }
    } else {
        None
    }
}

/// Load configuration from `context-builder.toml` in the specified project root directory.
/// Returns `None` if the file does not exist or cannot be parsed.
pub fn load_config_from_path(project_root: &Path) -> Option<Config> {
    let config_path = project_root.join("context-builder.toml");
    if config_path.exists() {
        let content = fs::read_to_string(&config_path).ok()?;
        match toml::from_str(&content) {
            Ok(config) => Some(config),
            Err(e) => {
                eprintln!(
                    "⚠️  Failed to parse {}: {}. Config will be ignored.",
                    config_path.display(),
                    e
                );
                None
            }
        }
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn load_config_nonexistent_file() {
        // Test loading config when file doesn't exist by temporarily changing directory
        let temp_dir = tempdir().unwrap();
        let original_dir = std::env::current_dir().unwrap();

        // Change to temp directory where no config file exists
        std::env::set_current_dir(&temp_dir).unwrap();

        let result = load_config();

        // Restore original directory
        std::env::set_current_dir(original_dir).unwrap();

        assert!(result.is_none());
    }

    #[test]
    fn load_config_from_path_nonexistent_file() {
        let dir = tempdir().unwrap();
        let result = load_config_from_path(dir.path());
        assert!(result.is_none());
    }

    #[test]
    fn load_config_from_path_valid_config() {
        let dir = tempdir().unwrap();
        let config_path = dir.path().join("context-builder.toml");

        let config_content = r#"
output = "test-output.md"
filter = ["rs", "toml"]
ignore = ["target", ".git"]
line_numbers = true
preview = false
token_count = true
timestamped_output = true
yes = false
auto_diff = true
diff_context_lines = 5
diff_only = false
encoding_strategy = "detect"
"#;

        fs::write(&config_path, config_content).unwrap();

        let config = load_config_from_path(dir.path()).unwrap();
        assert_eq!(config.output.unwrap(), "test-output.md");
        assert_eq!(config.filter.unwrap(), vec!["rs", "toml"]);
        assert_eq!(config.ignore.unwrap(), vec!["target", ".git"]);
        assert!(config.line_numbers.unwrap());
        assert!(!config.preview.unwrap());
        assert!(config.token_count.unwrap());
        assert!(config.timestamped_output.unwrap());
        assert!(!config.yes.unwrap());
        assert!(config.auto_diff.unwrap());
        assert_eq!(config.diff_context_lines.unwrap(), 5);
        assert!(!config.diff_only.unwrap());
        assert_eq!(config.encoding_strategy.unwrap(), "detect");
    }

    #[test]
    fn load_config_from_path_partial_config() {
        let dir = tempdir().unwrap();
        let config_path = dir.path().join("context-builder.toml");

        let config_content = r#"
output = "minimal.md"
filter = ["py"]
"#;

        fs::write(&config_path, config_content).unwrap();

        let config = load_config_from_path(dir.path()).unwrap();
        assert_eq!(config.output.unwrap(), "minimal.md");
        assert_eq!(config.filter.unwrap(), vec!["py"]);
        assert!(config.ignore.is_none());
        assert!(config.line_numbers.is_none());
        assert!(config.auto_diff.is_none());
    }

    #[test]
    fn load_config_from_path_invalid_toml() {
        let dir = tempdir().unwrap();
        let config_path = dir.path().join("context-builder.toml");

        // Invalid TOML content
        let config_content = r#"
output = "test.md"
invalid_toml [
"#;

        fs::write(&config_path, config_content).unwrap();

        let config = load_config_from_path(dir.path());
        assert!(config.is_none());
    }

    #[test]
    fn load_config_from_path_empty_config() {
        let dir = tempdir().unwrap();
        let config_path = dir.path().join("context-builder.toml");

        fs::write(&config_path, "").unwrap();

        let config = load_config_from_path(dir.path()).unwrap();
        assert!(config.output.is_none());
        assert!(config.filter.is_none());
        assert!(config.ignore.is_none());
    }

    #[test]
    fn config_default_implementation() {
        let config = Config::default();
        assert!(config.output.is_none());
        assert!(config.filter.is_none());
        assert!(config.ignore.is_none());
        assert!(config.line_numbers.is_none());
        assert!(config.preview.is_none());
        assert!(config.token_count.is_none());
        assert!(config.output_folder.is_none());
        assert!(config.timestamped_output.is_none());
        assert!(config.yes.is_none());
        assert!(config.auto_diff.is_none());
        assert!(config.diff_context_lines.is_none());
        assert!(config.diff_only.is_none());
        assert!(config.encoding_strategy.is_none());
        assert!(config.max_tokens.is_none());
        assert!(config.signatures.is_none());
        assert!(config.structure.is_none());
        assert!(config.truncate.is_none());
        assert!(config.visibility.is_none());
    }

    #[test]
    fn load_config_invalid_toml_in_cwd() {
        let temp_dir = tempdir().unwrap();
        let original_dir = std::env::current_dir().unwrap();

        std::env::set_current_dir(&temp_dir).unwrap();

        let config_path = temp_dir.path().join("context-builder.toml");
        let invalid_toml = r#"
output = "test.md"
invalid_toml [
"#;
        fs::write(&config_path, invalid_toml).unwrap();

        let result = load_config();

        std::env::set_current_dir(original_dir).unwrap();

        assert!(result.is_none());
    }

    #[test]
    fn load_config_valid_in_cwd() {
        let temp_dir = tempdir().unwrap();
        let original_dir = std::env::current_dir().unwrap();

        std::env::set_current_dir(&temp_dir).unwrap();

        let config_path = temp_dir.path().join("context-builder.toml");
        let valid_toml = r#"
output = "context.md"
filter = ["rs"]
"#;
        fs::write(&config_path, valid_toml).unwrap();

        let result = load_config();

        std::env::set_current_dir(original_dir).unwrap();

        assert!(result.is_some());
        let config = result.unwrap();
        assert_eq!(config.output, Some("context.md".to_string()));
    }
}
