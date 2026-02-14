//! Configuration resolution module for context-builder.
//!
//! This module provides centralized logic for merging CLI arguments with configuration
//! file values, implementing proper precedence rules and handling complex scenarios
//! like timestamping and output folder resolution.

use chrono::Utc;
use std::path::{Path, PathBuf};

use crate::cli::Args;
use crate::config::Config;

/// Resolved configuration combining CLI arguments and config file values
#[derive(Debug, Clone)]
pub struct ResolvedConfig {
    pub input: String,
    pub output: String,
    pub filter: Vec<String>,
    pub ignore: Vec<String>,
    pub line_numbers: bool,
    pub preview: bool,
    pub token_count: bool,
    pub yes: bool,
    pub diff_only: bool,
    pub clear_cache: bool,
    pub auto_diff: bool,
    pub diff_context_lines: usize,
    pub max_tokens: Option<usize>,
    pub init: bool,
}

/// Result of configuration resolution including the final config and any warnings
#[derive(Debug)]
pub struct ConfigResolution {
    pub config: ResolvedConfig,
    pub warnings: Vec<String>,
}

/// Resolves final configuration by merging CLI arguments with config file values.
///
/// Precedence rules (highest to lowest):
/// 1. Explicit CLI arguments (non-default values)
/// 2. Configuration file values
/// 3. CLI default values
///
/// Special handling:
/// - `output` field supports timestamping and output folder resolution
/// - Boolean flags respect explicit CLI usage vs defaults
/// - Arrays (filter, ignore) use CLI if non-empty, otherwise config file
pub fn resolve_final_config(mut args: Args, config: Option<Config>) -> ConfigResolution {
    let mut warnings = Vec::new();

    // Start with CLI defaults, then apply config file, then explicit CLI overrides
    let final_config = if let Some(config) = config {
        apply_config_to_args(&mut args, &config, &mut warnings);
        resolve_output_path(&mut args, &config, &mut warnings);
        config
    } else {
        Config::default()
    };

    let resolved = ResolvedConfig {
        input: args.input,
        output: args.output,
        filter: args.filter,
        ignore: args.ignore,
        line_numbers: args.line_numbers,
        preview: args.preview,
        token_count: args.token_count,
        yes: args.yes,
        diff_only: args.diff_only,
        clear_cache: args.clear_cache,
        auto_diff: final_config.auto_diff.unwrap_or(false),
        diff_context_lines: final_config.diff_context_lines.unwrap_or(3),
        max_tokens: args.max_tokens.or(final_config.max_tokens),
        init: args.init,
    };

    ConfigResolution {
        config: resolved,
        warnings,
    }
}

/// Apply configuration file values to CLI arguments based on precedence rules
fn apply_config_to_args(args: &mut Args, config: &Config, warnings: &mut Vec<String>) {
    // Output: only apply config if CLI is using default value
    if args.output == "output.md"
        && let Some(ref output) = config.output
    {
        args.output = output.clone();
    }

    // Filter: CLI takes precedence if non-empty
    if args.filter.is_empty()
        && let Some(ref filter) = config.filter
    {
        args.filter = filter.clone();
    }

    // Ignore: CLI takes precedence if non-empty
    if args.ignore.is_empty()
        && let Some(ref ignore) = config.ignore
    {
        args.ignore = ignore.clone();
    }

    // Boolean flags: config applies only if CLI is using default (false)
    // Note: We can't distinguish between explicit --no-flag and default false,
    // so config file can only enable features, not disable them
    if !args.line_numbers
        && let Some(line_numbers) = config.line_numbers
    {
        args.line_numbers = line_numbers;
    }

    if !args.preview
        && let Some(preview) = config.preview
    {
        args.preview = preview;
    }

    if !args.token_count
        && let Some(token_count) = config.token_count
    {
        args.token_count = token_count;
    }

    if !args.yes
        && let Some(yes) = config.yes
    {
        args.yes = yes;
    }

    // diff_only: config can enable it, but CLI flag always takes precedence
    if !args.diff_only
        && let Some(true) = config.diff_only
    {
        args.diff_only = true;
    }

    // Validate auto_diff configuration
    if let Some(true) = config.auto_diff
        && config.timestamped_output != Some(true)
    {
        warnings.push(
            "auto_diff is enabled but timestamped_output is not enabled. \
            Auto-diff requires timestamped_output = true to function properly."
                .to_string(),
        );
    }
}

/// Resolve output path including timestamping and output folder logic
fn resolve_output_path(args: &mut Args, config: &Config, warnings: &mut Vec<String>) {
    let mut output_folder_path: Option<PathBuf> = None;

    // Apply output folder first
    if let Some(ref output_folder) = config.output_folder {
        let mut path = PathBuf::from(output_folder);
        path.push(&args.output);
        args.output = path.to_string_lossy().to_string();
        output_folder_path = Some(PathBuf::from(output_folder));
    }

    // Apply timestamping if enabled
    if let Some(true) = config.timestamped_output {
        let timestamp = Utc::now().format("%Y%m%d%H%M%S").to_string();
        let path = Path::new(&args.output);

        let stem = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("output");

        let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("md");

        let new_filename = format!("{}_{}.{}", stem, timestamp, extension);

        if let Some(output_folder) = output_folder_path {
            args.output = output_folder
                .join(new_filename)
                .to_string_lossy()
                .to_string();
        } else {
            let new_path = path.with_file_name(new_filename);
            args.output = new_path.to_string_lossy().to_string();
        }
    }

    // Validate output folder exists if specified
    if let Some(ref output_folder) = config.output_folder {
        let folder_path = Path::new(output_folder);
        if !folder_path.exists() {
            warnings.push(format!(
                "Output folder '{}' does not exist. It will be created if possible.",
                output_folder
            ));
        }
    }
}

/// Check if CLI arguments have been explicitly set vs using defaults.
/// This is a best-effort detection since clap doesn't provide this information directly.
#[allow(dead_code)]
fn detect_explicit_args() -> ExplicitArgs {
    let args: Vec<String> = std::env::args().collect();

    ExplicitArgs {
        output: args.iter().any(|arg| arg == "-o" || arg == "--output"),
        filter: args.iter().any(|arg| arg == "-f" || arg == "--filter"),
        ignore: args.iter().any(|arg| arg == "-i" || arg == "--ignore"),
        line_numbers: args.iter().any(|arg| arg == "--line-numbers"),
        preview: args.iter().any(|arg| arg == "--preview"),
        token_count: args.iter().any(|arg| arg == "--token-count"),
        yes: args.iter().any(|arg| arg == "-y" || arg == "--yes"),
        diff_only: args.iter().any(|arg| arg == "--diff-only"),
    }
}

/// Tracks which CLI arguments were explicitly provided vs using defaults
#[allow(dead_code)]
struct ExplicitArgs {
    output: bool,
    filter: bool,
    ignore: bool,
    line_numbers: bool,
    preview: bool,
    token_count: bool,
    yes: bool,
    diff_only: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_precedence_cli_over_config() {
        let args = Args {
            input: "src".to_string(),
            output: "custom.md".to_string(), // Explicit CLI value
            filter: vec!["rs".to_string()],  // Explicit CLI value
            ignore: vec![],
            line_numbers: true, // Explicit CLI value
            preview: false,
            token_count: false,
            yes: false,
            diff_only: false,
            clear_cache: false,
            init: false,
            max_tokens: None,
        };

        let config = Config {
            output: Some("config.md".to_string()),  // Should be ignored
            filter: Some(vec!["toml".to_string()]), // Should be ignored
            line_numbers: Some(false),              // Should be ignored
            preview: Some(true),                    // Should apply
            ..Default::default()
        };

        let resolution = resolve_final_config(args.clone(), Some(config));

        assert_eq!(resolution.config.output, "custom.md"); // CLI wins
        assert_eq!(resolution.config.filter, vec!["rs"]); // CLI wins
        assert!(resolution.config.line_numbers); // CLI wins
        assert!(resolution.config.preview); // Config applies
    }

    #[test]
    fn test_config_applies_when_cli_uses_defaults() {
        let args = Args {
            input: "src".to_string(),
            output: "output.md".to_string(), // Default value
            filter: vec![],                  // Default value
            ignore: vec![],                  // Default value
            line_numbers: false,             // Default value
            preview: false,                  // Default value
            token_count: false,              // Default value
            yes: false,                      // Default value
            diff_only: false,                // Default value
            clear_cache: false,
            init: false,
            max_tokens: None,
        };

        let config = Config {
            output: Some("from_config.md".to_string()),
            filter: Some(vec!["rs".to_string(), "toml".to_string()]),
            ignore: Some(vec!["target".to_string()]),
            line_numbers: Some(true),
            preview: Some(true),
            token_count: Some(true),
            yes: Some(true),
            diff_only: Some(true),
            ..Default::default()
        };

        let resolution = resolve_final_config(args, Some(config));

        assert_eq!(resolution.config.output, "from_config.md");
        assert_eq!(
            resolution.config.filter,
            vec!["rs".to_string(), "toml".to_string()]
        );
        assert_eq!(resolution.config.ignore, vec!["target".to_string()]);
        assert!(resolution.config.line_numbers);
        assert!(resolution.config.preview);
        assert!(resolution.config.token_count);
        assert!(resolution.config.yes);
        assert!(resolution.config.diff_only);
    }

    #[test]
    fn test_timestamped_output_resolution() {
        let args = Args {
            input: "src".to_string(),
            output: "test.md".to_string(),
            filter: vec![],
            ignore: vec![],
            line_numbers: false,
            preview: false,
            token_count: false,
            yes: false,
            diff_only: false,
            clear_cache: false,
            init: false,
            max_tokens: None,
        };

        let config = Config {
            timestamped_output: Some(true),
            ..Default::default()
        };

        let resolution = resolve_final_config(args, Some(config));

        // Output should have timestamp format: test_YYYYMMDDHHMMSS.md
        assert!(resolution.config.output.starts_with("test_"));
        assert!(resolution.config.output.ends_with(".md"));
        assert!(resolution.config.output.len() > "test_.md".len());
    }

    #[test]
    fn test_output_folder_resolution() {
        let args = Args {
            input: "src".to_string(),
            output: "test.md".to_string(),
            filter: vec![],
            ignore: vec![],
            line_numbers: false,
            preview: false,
            token_count: false,
            yes: false,
            diff_only: false,
            clear_cache: false,
            init: false,
            max_tokens: None,
        };

        let config = Config {
            output_folder: Some("docs".to_string()),
            ..Default::default()
        };

        let resolution = resolve_final_config(args, Some(config));

        assert!(resolution.config.output.contains("docs"));
        assert!(resolution.config.output.ends_with("test.md"));
    }

    #[test]
    fn test_output_folder_with_timestamping() {
        let args = Args {
            input: "src".to_string(),
            output: "test.md".to_string(),
            filter: vec![],
            ignore: vec![],
            line_numbers: false,
            preview: false,
            token_count: false,
            yes: false,
            diff_only: false,
            clear_cache: false,
            init: false,
            max_tokens: None,
        };

        let config = Config {
            output_folder: Some("docs".to_string()),
            timestamped_output: Some(true),
            ..Default::default()
        };

        let resolution = resolve_final_config(args, Some(config));

        assert!(resolution.config.output.contains("docs"));
        assert!(resolution.config.output.contains("test_"));
        assert!(resolution.config.output.ends_with(".md"));
    }

    #[test]
    fn test_auto_diff_without_timestamping_warning() {
        let args = Args {
            input: "src".to_string(),
            output: "test.md".to_string(),
            filter: vec![],
            ignore: vec![],
            line_numbers: false,
            preview: false,
            token_count: false,
            yes: false,
            diff_only: false,
            clear_cache: false,
            init: false,
            max_tokens: None,
        };

        let config = Config {
            auto_diff: Some(true),
            timestamped_output: Some(false), // This should generate a warning
            ..Default::default()
        };

        let resolution = resolve_final_config(args, Some(config));

        assert!(!resolution.warnings.is_empty());
        assert!(resolution.warnings[0].contains("auto_diff"));
        assert!(resolution.warnings[0].contains("timestamped_output"));
    }

    #[test]
    fn test_no_config_uses_cli_defaults() {
        let args = Args {
            input: "src".to_string(),
            output: "output.md".to_string(),
            filter: vec![],
            ignore: vec![],
            line_numbers: false,
            preview: false,
            token_count: false,
            yes: false,
            diff_only: false,
            clear_cache: false,
            init: false,
            max_tokens: None,
        };

        let resolution = resolve_final_config(args.clone(), None);

        assert_eq!(resolution.config.input, args.input);
        assert_eq!(resolution.config.output, args.output);
        assert_eq!(resolution.config.filter, args.filter);
        assert_eq!(resolution.config.ignore, args.ignore);
        assert_eq!(resolution.config.line_numbers, args.line_numbers);
        assert_eq!(resolution.config.preview, args.preview);
        assert_eq!(resolution.config.token_count, args.token_count);
        assert_eq!(resolution.config.yes, args.yes);
        assert_eq!(resolution.config.diff_only, args.diff_only);
        assert!(!resolution.config.auto_diff);
        assert_eq!(resolution.config.diff_context_lines, 3);
        assert!(resolution.warnings.is_empty());
    }
}
