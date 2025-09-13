use serde::Deserialize;
use std::fs;
use std::path::Path;

/// Global configuration loaded from `.context-builder.toml`.
///
/// Any field left as `None` means "use the CLI default / do not override".
/// Command-line arguments always take precedence over values provided here.
///
/// Example `.context-builder.toml`:
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
}

/// Load configuration from `.context-builder.toml` in the current working directory.
/// Returns `None` if the file does not exist or cannot be parsed.
pub fn load_config() -> Option<Config> {
    let config_path = Path::new(".context-builder.toml");
    if config_path.exists() {
        let content = fs::read_to_string(config_path).ok()?;
        toml::from_str(&content).ok()
    } else {
        None
    }
}

/// Load configuration from `.context-builder.toml` in the specified project root directory.
/// Returns `None` if the file does not exist or cannot be parsed.
pub fn load_config_from_path(project_root: &Path) -> Option<Config> {
    let config_path = project_root.join(".context-builder.toml");
    if config_path.exists() {
        let content = fs::read_to_string(config_path).ok()?;
        toml::from_str(&content).ok()
    } else {
        None
    }
}
