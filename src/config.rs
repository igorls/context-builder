use serde::Deserialize;
use std::fs;
use std::path::Path;

use crate::content_filter::{self, DEFAULT_MAX_FILE_SIZE_BYTES};

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
/// # Lockfiles stay out unless this is true. `--include-lockfiles` overrides it.
/// # `-f toml` / `-f lock` do not include them on their own.
/// include_lockfiles = false
/// line_numbers = false
/// file_metadata = false   # Per-file Size/Modified lines (off by default)
/// diff_context_lines = 5
/// # max_file_size = "256K"   # "0" disables; also accepts an integer byte count
/// # hidden = false           # include dotfiles (not .git); secrets stay skipped
/// # include_secrets = false  # opt in to id_rsa, *.pem, .env, credentials*.json, …
/// ```
///
#[derive(Deserialize, Debug, Default, Clone)]
pub struct Config {
    /// Output file name (or base name when `timestamped_output = true`)
    pub output: Option<String>,

    /// File extensions to include (no leading dot, e.g. `rs`, `toml`)
    pub filter: Option<Vec<String>>,

    /// Paths or gitignore-style globs to ignore (names like `docs`, paths like
    /// `crates/core`, globs like `*.lock`). Same patterns as `--ignore`.
    pub ignore: Option<Vec<String>>,

    /// Include dependency lockfiles (`Cargo.lock`, `uv.lock`, …).
    /// Default is to skip them. CLI `--include-lockfiles` overrides this.
    /// A `filter` value that matches a lockfile's type does not.
    pub include_lockfiles: Option<bool>,

    /// Add line numbers to code blocks
    pub line_numbers: Option<bool>,

    /// Emit per-file `- Size:` and `- Modified:` lines under each file header.
    /// Off by default (`None` / `false`). `--file-metadata` on the CLI overrides
    /// a config value of `false`; when the flag is omitted, this key applies.
    pub file_metadata: Option<bool>,

    /// Preview only the file tree (no file output)
    pub preview: Option<bool>,

    /// Token counting mode
    pub token_count: Option<bool>,

    /// Optional folder to place the generated output file(s) in
    pub output_folder: Option<String>,

    /// If true, append a UTC timestamp to the output file name (before extension)
    pub timestamped_output: Option<bool>,

    /// Assume "yes" for the overwrite prompt.
    ///
    /// Still accepted after the >100-file confirmation was removed in v0.11.0.
    /// It does not change processing; there is no processing prompt to skip.
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

    /// Tokenizer encoding for token counting/budgeting.
    /// - "o200k_base": GPT-4o / o-series (default)
    /// - "cl100k_base": GPT-4 / GPT-3.5
    pub encoding: Option<String>,

    /// Maximum file size to include. A string (`"256K"`, `"1M"`, `"0"`) or an
    /// integer byte count. `"0"` disables the limit. Unset means 256 KiB.
    /// Command-line `--max-file-size` wins, including when passed explicitly
    /// as the default `256K`.
    #[serde(default, deserialize_with = "deserialize_optional_size")]
    pub max_file_size: Option<String>,

    /// When true, include hidden dotfiles and dot-directories (except `.git`,
    /// `.hg`, `.svn`, and `.bzr`). Does not include likely-secret files.
    pub hidden: Option<bool>,

    /// When true, include likely-secret files the walk collected. Dotfile
    /// secrets such as `.env` still need `hidden = true`.
    pub include_secrets: Option<bool>,
}

/// Accept `max_file_size` as either a string (`"256K"`) or an integer byte count.
fn deserialize_optional_size<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value = toml::Value::deserialize(deserializer)?;
    match value {
        toml::Value::String(s) => Ok(Some(s)),
        toml::Value::Integer(i) if i >= 0 => Ok(Some(i.to_string())),
        toml::Value::Integer(_) => Err(serde::de::Error::custom(
            "max_file_size must be >= 0 (0 disables the limit)",
        )),
        other => Err(serde::de::Error::custom(format!(
            "max_file_size must be a string (\"256K\") or an integer byte count, got {other}"
        ))),
    }
}

/// Stable fingerprint of the configuration that determines the auto-diff
/// *baseline* — i.e. which files are captured and compared between runs. Shared
/// by the cache key (`cache.rs`) and the project-state config hash (`state.rs`)
/// so the two can never drift out of sync.
///
/// The baseline is the **raw content** of the selected files (`ProjectState`
/// stores each file's bytes via `read_to_string`; the diff compares those). The
/// inputs that change that baseline are the file-selection options: `filter`,
/// `ignore`, `include_lockfiles`, `max_file_size`, `hidden`, and
/// `include_secrets`. Everything else is pure *rendering* — `line_numbers`,
/// `file_metadata`, `signatures`, `structure`, `truncate`, `visibility`,
/// `max_tokens`, `encoding`/`encoding_strategy`, `diff_context_lines`,
/// `diff_only`, `timestamped_output`, `output_folder` — and does **not** affect
/// the captured content. Such options are deliberately EXCLUDED: including them
/// would reset the diff baseline whenever a user toggles one (e.g. adding
/// `--signatures`), silently hiding real content changes on that run. (The
/// project *path* is keyed separately in `cache.rs`, so it isn't part of this
/// fingerprint.)
pub(crate) fn config_fingerprint(config: &Config) -> String {
    let mut s = String::new();
    if let Some(ref filters) = config.filter {
        s.push_str(&filters.join(","));
    }
    s.push('|');
    if let Some(ref ignores) = config.ignore {
        s.push_str(&ignores.join(","));
    }
    s.push('|');
    // Normalize so unset and an explicit "256K" hash the same, and so "1M"
    // and "1024K" hash the same. Invalid specs stay distinct from the default.
    s.push_str(&size_fingerprint(config.max_file_size.as_deref()));
    s.push('|');
    s.push(if config.hidden == Some(true) {
        '1'
    } else {
        '0'
    });
    s.push('|');
    s.push(if config.include_secrets == Some(true) {
        '1'
    } else {
        '0'
    });
    // Selection version: bumped when the default file selection changes
    // (v0.11: lockfiles skipped by default), so baselines cached under the
    // previous default are not diffed against the new selection.
    s.push_str("|sel2");
    // The opt-in adds lockfiles to the file set; the default (skip) adds nothing.
    if config.include_lockfiles == Some(true) {
        s.push_str("|lockfiles");
    }
    let hash = xxhash_rust::xxh3::xxh3_64(s.as_bytes());
    format!("{:x}", hash)
}

fn size_fingerprint(spec: Option<&str>) -> String {
    match spec {
        None => DEFAULT_MAX_FILE_SIZE_BYTES.to_string(),
        Some(spec) => match content_filter::parse_file_size(spec) {
            Ok(None) => "unlimited".to_string(),
            Ok(Some(bytes)) => bytes.to_string(),
            Err(_) => format!("invalid:{spec}"),
        },
    }
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
    use serial_test::serial;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    #[serial]
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
    fn load_config_accepts_size_string_or_integer() {
        let dir = tempdir().unwrap();
        let config_path = dir.path().join("context-builder.toml");
        fs::write(
            &config_path,
            "max_file_size = \"1M\"\nhidden = true\ninclude_secrets = false\n",
        )
        .unwrap();
        let config = load_config_from_path(dir.path()).unwrap();
        assert_eq!(config.max_file_size.as_deref(), Some("1M"));
        assert_eq!(config.hidden, Some(true));
        assert_eq!(config.include_secrets, Some(false));

        fs::write(&config_path, "max_file_size = 262144\n").unwrap();
        let config = load_config_from_path(dir.path()).unwrap();
        assert_eq!(config.max_file_size.as_deref(), Some("262144"));
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
        assert!(config.file_metadata.is_none());
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
        assert!(config.encoding.is_none());
        assert!(config.max_file_size.is_none());
        assert!(config.hidden.is_none());
        assert!(config.include_secrets.is_none());
        assert!(config.include_lockfiles.is_none());
    }

    #[test]
    fn config_fingerprint_sensitivity() {
        // The cache/diff fingerprint must change ONLY for the file-selection
        // options (filter, ignore, max_file_size, hidden, include_secrets) that
        // determine which files form the comparable baseline. Every pure
        // output-rendering option must leave it untouched, so toggling one
        // against an existing baseline never discards the diff.
        let base = Config::default();
        let base_h = config_fingerprint(&base);

        // --- File selection: MUST change the fingerprint ---
        let mut c = base.clone();
        c.filter = Some(vec!["rs".to_string()]);
        assert_ne!(
            config_fingerprint(&c),
            base_h,
            "filter changes which files are captured, so it must change the fingerprint"
        );

        let mut c = base.clone();
        c.ignore = Some(vec!["target".to_string()]);
        assert_ne!(
            config_fingerprint(&c),
            base_h,
            "ignore changes which files are captured, so it must change the fingerprint"
        );

        let mut c = base.clone();
        c.max_file_size = Some("1M".to_string());
        assert_ne!(
            config_fingerprint(&c),
            base_h,
            "max_file_size changes which files are captured"
        );
        // Unset and the default spelling are the same selection.
        c.max_file_size = Some("256K".to_string());
        assert_eq!(config_fingerprint(&c), base_h);
        c.max_file_size = Some("0".to_string());
        assert_ne!(config_fingerprint(&c), base_h);

        let mut c = base.clone();
        c.hidden = Some(true);
        assert_ne!(
            config_fingerprint(&c),
            base_h,
            "hidden changes which files are captured"
        );
        c.hidden = Some(false);
        assert_eq!(config_fingerprint(&c), base_h);

        let mut c = base.clone();
        c.include_secrets = Some(true);
        assert_ne!(
            config_fingerprint(&c),
            base_h,
            "include_secrets changes which files are captured"
        );

        let mut c = base.clone();
        c.include_lockfiles = Some(true);
        assert_ne!(
            config_fingerprint(&c),
            base_h,
            "include_lockfiles changes which files are captured, so it must change the fingerprint"
        );
        // The selection-version marker must make the default differ from the
        // pre-v0.11 fingerprint (which was the hash of "|" for an empty config).
        let legacy = format!("{:x}", xxhash_rust::xxh3::xxh3_64(b"|"));
        assert_ne!(
            base_h, legacy,
            "default selection changed; baseline must reset"
        );
        let mut c = base.clone();
        c.include_lockfiles = Some(false);
        assert_eq!(
            config_fingerprint(&c),
            base_h,
            "the default skip must keep the existing fingerprint"
        );

        // --- Rendering options: MUST NOT change the fingerprint ---
        // (none of these affect the raw content captured into the diff baseline)
        type Mutate = fn(&mut Config);
        let render_only: Vec<(&str, Mutate)> = vec![
            ("line_numbers", |c| c.line_numbers = Some(true)),
            ("file_metadata", |c| c.file_metadata = Some(true)),
            ("signatures", |c| c.signatures = Some(true)),
            ("structure", |c| c.structure = Some(true)),
            ("truncate", |c| c.truncate = Some("byte".to_string())),
            ("visibility", |c| c.visibility = Some("public".to_string())),
            ("max_tokens", |c| c.max_tokens = Some(1000)),
            ("encoding", |c| c.encoding = Some("cl100k_base".to_string())),
            ("encoding_strategy", |c| {
                c.encoding_strategy = Some("strict".to_string())
            }),
            ("diff_only", |c| c.diff_only = Some(true)),
            ("diff_context_lines", |c| c.diff_context_lines = Some(9)),
        ];
        for (name, mutate) in render_only {
            let mut c = base.clone();
            mutate(&mut c);
            assert_eq!(
                config_fingerprint(&c),
                base_h,
                "{name} is a pure rendering option and must NOT invalidate the diff baseline"
            );
        }
    }

    #[test]
    #[serial]
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
    #[serial]
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
