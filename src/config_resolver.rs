//! Configuration resolution module for context-builder.
//!
//! This module provides centralized logic for merging CLI arguments with configuration
//! file values, implementing proper precedence rules and handling complex scenarios
//! like timestamping and output folder resolution.

use chrono::Utc;
use std::path::{Path, PathBuf};

use crate::cli::Args;
use crate::config::Config;
use crate::content_filter::{self, DEFAULT_MAX_FILE_SIZE_SPEC};

/// Resolved configuration combining CLI arguments and config file values
#[derive(Debug, Clone)]
pub struct ResolvedConfig {
    pub input: String,
    pub output: String,
    pub filter: Vec<String>,
    pub ignore: Vec<String>,
    pub include_lockfiles: bool,
    pub line_numbers: bool,
    pub file_metadata: bool,
    pub preview: bool,
    pub token_count: bool,
    pub yes: bool,
    pub diff_only: bool,
    pub clear_cache: bool,
    pub auto_diff: bool,
    pub diff_context_lines: usize,
    pub max_tokens: Option<usize>,
    pub init: bool,
    pub signatures: bool,
    pub structure: bool,
    pub truncate: String,
    pub visibility: String,
    pub encoding: String,
    /// Resolved size specification (`256K`, `1M`, `0`, …).
    pub max_file_size: String,
    /// Include hidden dotfiles and dot-directories.
    pub hidden: bool,
    /// Include likely-secret files the walk collected.
    pub include_secrets: bool,
}

/// Result of configuration resolution including the final config and any warnings
#[derive(Debug)]
pub struct ConfigResolution {
    pub config: ResolvedConfig,
    pub warnings: Vec<String>,
}

/// Which value-bearing CLI flags the user explicitly passed (as opposed to
/// leaving at their clap default). These flags carry a default *value*, so the
/// value alone can't tell us whether the user typed e.g. `--encoding o200k_base`
/// or `-o output.md` to override a non-default config or simply omitted the
/// flag. `run()` fills this from clap's `ValueSource`; `Default` (all `false`)
/// means "treat the value as a default", which preserves the value-based
/// precedence for callers (e.g. tests) that build `Args` directly.
#[derive(Debug, Clone, Copy, Default)]
pub struct ExplicitCli {
    pub truncate: bool,
    pub visibility: bool,
    pub encoding: bool,
    pub max_file_size: bool,
    /// `-o` / `--output` was present on the command line.
    pub output: bool,
}

/// Resolves final configuration by merging CLI arguments with config file values.
///
/// Precedence rules (highest to lowest):
/// 1. Explicit CLI arguments — a non-default value, or (for the value-bearing
///    flags in `ExplicitCli`) a flag the user passed even at its default value
/// 2. Configuration file values
/// 3. CLI default values
///
/// Special handling:
/// - An explicit `-o` / `--output` (including `-` for stdout) is used verbatim.
///   `output_folder` and `timestamped_output` apply only when `-o` was omitted.
/// - A relative `output_folder` is resolved against the project root (`-d`),
///   not the process working directory. An absolute `output_folder` is kept.
/// - Boolean flags respect explicit CLI usage vs defaults
/// - Arrays (filter, ignore) use CLI if non-empty, otherwise config file
pub fn resolve_final_config(
    mut args: Args,
    config: Option<Config>,
    explicit: ExplicitCli,
) -> ConfigResolution {
    let mut warnings = Vec::new();

    // Start with CLI defaults, then apply config file, then explicit CLI overrides
    let final_config = if let Some(config) = config {
        apply_config_to_args(&mut args, &config, explicit, &mut warnings);
        resolve_output_path(&mut args, &config, explicit, &mut warnings);
        config
    } else {
        Config::default()
    };

    let max_file_size = resolve_max_file_size(&args, &final_config, explicit, &mut warnings);
    let hidden = args.hidden || final_config.hidden.unwrap_or(false);
    let include_secrets = args.include_secrets || final_config.include_secrets.unwrap_or(false);

    let resolved = ResolvedConfig {
        input: args.input,
        output: args.output,
        filter: args.filter,
        ignore: args.ignore,
        include_lockfiles: args.include_lockfiles,
        line_numbers: args.line_numbers,
        file_metadata: args.file_metadata,
        preview: args.preview,
        token_count: args.token_count,
        yes: args.yes,
        diff_only: args.diff_only,
        clear_cache: args.clear_cache,
        auto_diff: final_config.auto_diff.unwrap_or(false),
        diff_context_lines: final_config.diff_context_lines.unwrap_or(3),
        max_tokens: args.max_tokens.or(final_config.max_tokens),
        init: args.init,
        signatures: args.signatures || final_config.signatures.unwrap_or(false),
        structure: args.structure || final_config.structure.unwrap_or(false),
        // CLI explicit (even when the value equals the default) > config > default.
        // The `|| value != default` keeps value-based precedence for callers that
        // build Args directly without an explicitness signal (e.g. tests).
        truncate: if explicit.truncate || args.truncate != "smart" {
            args.truncate.clone()
        } else {
            final_config
                .truncate
                .clone()
                .unwrap_or_else(|| args.truncate.clone())
        },
        visibility: if explicit.visibility || args.visibility != "all" {
            args.visibility.clone()
        } else {
            final_config
                .visibility
                .clone()
                .unwrap_or_else(|| args.visibility.clone())
        },
        encoding: if explicit.encoding || args.encoding != "o200k_base" {
            args.encoding.clone()
        } else {
            final_config
                .encoding
                .clone()
                .unwrap_or_else(|| args.encoding.clone())
        },
        max_file_size,
        hidden,
        include_secrets,
    };

    ConfigResolution {
        config: resolved,
        warnings,
    }
}

/// Pick the size limit: an explicit CLI value (even the default `256K`) wins,
/// otherwise a valid config value, otherwise the CLI default.
fn resolve_max_file_size(
    args: &Args,
    config: &Config,
    explicit: ExplicitCli,
    warnings: &mut Vec<String>,
) -> String {
    if explicit.max_file_size || args.max_file_size != DEFAULT_MAX_FILE_SIZE_SPEC {
        return args.max_file_size.clone();
    }
    if let Some(ref spec) = config.max_file_size {
        match content_filter::parse_file_size(spec) {
            Ok(_) => return spec.clone(),
            Err(err) => warnings.push(format!(
                "Invalid max_file_size '{spec}' in config ({err}). Using {DEFAULT_MAX_FILE_SIZE_SPEC}."
            )),
        }
    }
    args.max_file_size.clone()
}

/// Apply configuration file values to CLI arguments based on precedence rules
fn apply_config_to_args(
    args: &mut Args,
    config: &Config,
    explicit: ExplicitCli,
    warnings: &mut Vec<String>,
) {
    // Output name: config applies only when `-o` was omitted and the value is
    // still the clap default. An explicit `-o output.md` stays `output.md`.
    if !explicit.output
        && args.output == "output.md"
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

    // Lockfiles are off unless the CLI flag or the config key turns them on.
    // An explicit `--include-lockfiles` (args already true) wins over config.
    if !args.include_lockfiles
        && let Some(include_lockfiles) = config.include_lockfiles
    {
        args.include_lockfiles = include_lockfiles;
    }

    // Boolean flags: config applies only if CLI is using default (false)
    // Note: We can't distinguish between explicit --no-flag and default false,
    // so config file can only enable features, not disable them
    if !args.line_numbers
        && let Some(line_numbers) = config.line_numbers
    {
        args.line_numbers = line_numbers;
    }

    // file_metadata: same boolean rule as line_numbers. `--file-metadata`
    // (true) always wins; a config value applies only when the flag is omitted
    // (the clap default is false, so it cannot express an explicit "off").
    if !args.file_metadata
        && let Some(file_metadata) = config.file_metadata
    {
        args.file_metadata = file_metadata;
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

/// Resolve output path including timestamping and output folder logic.
///
/// An explicit `-o` (and stdout `-`) is returned unchanged. `output_folder` and
/// `timestamped_output` run only for the default output path. A relative
/// `output_folder` is anchored at the project root (`args.input`, the `-d`
/// directory the config was loaded from), so `cd /tmp && context-builder -d
/// /repo` writes `/repo/docs/…` rather than `/tmp/docs/…`.
fn resolve_output_path(
    args: &mut Args,
    config: &Config,
    explicit: ExplicitCli,
    warnings: &mut Vec<String>,
) {
    // `-` means stdout. An explicit `-o` path is used verbatim — do not fold in
    // an output folder or a timestamp, and do not warn about a folder we are
    // not going to use.
    if args.output == "-" || explicit.output {
        return;
    }

    let project_root = Path::new(&args.input);
    let anchored_folder = config
        .output_folder
        .as_ref()
        .map(|folder| anchor_output_folder(folder, project_root));

    if let Some(ref folder) = anchored_folder {
        args.output = folder.join(&args.output).to_string_lossy().to_string();
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

        let new_output = if let Some(ref folder) = anchored_folder {
            folder.join(new_filename).to_string_lossy().to_string()
        } else {
            path.with_file_name(new_filename)
                .to_string_lossy()
                .to_string()
        };
        args.output = new_output;
    }

    // Validate output folder exists if specified (check the path we will write to)
    if let Some(ref folder) = anchored_folder
        && !folder.exists()
    {
        warnings.push(format!(
            "Output folder '{}' does not exist. It will be created if possible.",
            folder.display()
        ));
    }
}

/// Resolve `output_folder` against the project root.
///
/// Absolute folders are kept. Relative folders are joined onto `project_root`
/// (the `-d` directory), not left for `File::create` to interpret against cwd.
fn anchor_output_folder(output_folder: &str, project_root: &Path) -> PathBuf {
    let folder = PathBuf::from(output_folder);
    if folder.is_absolute() {
        folder
    } else {
        project_root.join(folder)
    }
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
            encoding: "o200k_base".to_string(),
            init: false,
            max_tokens: None,
            signatures: false,
            structure: false,
            truncate: "smart".to_string(),
            visibility: "all".to_string(),
            max_file_size: "256K".to_string(),
            hidden: false,
            include_secrets: false,
            file_metadata: false,
            include_lockfiles: false,
        };

        let config = Config {
            output: Some("config.md".to_string()),  // Should be ignored
            filter: Some(vec!["toml".to_string()]), // Should be ignored
            line_numbers: Some(false),              // Should be ignored
            preview: Some(true),                    // Should apply
            ..Default::default()
        };

        let resolution = resolve_final_config(args.clone(), Some(config), ExplicitCli::default());

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
            encoding: "o200k_base".to_string(),
            init: false,
            max_tokens: None,
            signatures: false,
            structure: false,
            truncate: "smart".to_string(),
            visibility: "all".to_string(),
            max_file_size: "256K".to_string(),
            hidden: false,
            include_secrets: false,
            file_metadata: false,
            include_lockfiles: false,
        };

        let config = Config {
            output: Some("from_config.md".to_string()),
            filter: Some(vec!["rs".to_string(), "toml".to_string()]),
            ignore: Some(vec!["target".to_string()]),
            line_numbers: Some(true),
            file_metadata: Some(true),
            preview: Some(true),
            token_count: Some(true),
            yes: Some(true),
            diff_only: Some(true),
            ..Default::default()
        };

        let resolution = resolve_final_config(args, Some(config), ExplicitCli::default());

        assert_eq!(resolution.config.output, "from_config.md");
        assert_eq!(
            resolution.config.filter,
            vec!["rs".to_string(), "toml".to_string()]
        );
        assert_eq!(resolution.config.ignore, vec!["target".to_string()]);
        assert!(resolution.config.line_numbers);
        assert!(resolution.config.file_metadata);
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
            encoding: "o200k_base".to_string(),
            init: false,
            max_tokens: None,
            signatures: false,
            structure: false,
            truncate: "smart".to_string(),
            visibility: "all".to_string(),
            max_file_size: "256K".to_string(),
            hidden: false,
            include_secrets: false,
            file_metadata: false,
            include_lockfiles: false,
        };

        let config = Config {
            timestamped_output: Some(true),
            ..Default::default()
        };

        let resolution = resolve_final_config(args, Some(config), ExplicitCli::default());

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
            encoding: "o200k_base".to_string(),
            init: false,
            max_tokens: None,
            signatures: false,
            structure: false,
            truncate: "smart".to_string(),
            visibility: "all".to_string(),
            max_file_size: "256K".to_string(),
            hidden: false,
            include_secrets: false,
            file_metadata: false,
            include_lockfiles: false,
        };

        let config = Config {
            output_folder: Some("docs".to_string()),
            ..Default::default()
        };

        let resolution = resolve_final_config(args, Some(config), ExplicitCli::default());

        // Relative output_folder is anchored at the project root (`-d` / input),
        // not left as a cwd-relative `docs/test.md`.
        assert_eq!(
            PathBuf::from(&resolution.config.output),
            Path::new("src").join("docs").join("test.md")
        );
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
            encoding: "o200k_base".to_string(),
            init: false,
            max_tokens: None,
            signatures: false,
            structure: false,
            truncate: "smart".to_string(),
            visibility: "all".to_string(),
            max_file_size: "256K".to_string(),
            hidden: false,
            include_secrets: false,
            file_metadata: false,
            include_lockfiles: false,
        };

        let config = Config {
            output_folder: Some("docs".to_string()),
            timestamped_output: Some(true),
            ..Default::default()
        };

        let resolution = resolve_final_config(args, Some(config), ExplicitCli::default());

        let out = PathBuf::from(&resolution.config.output);
        let expected_dir = Path::new("src").join("docs");
        assert_eq!(out.parent(), Some(expected_dir.as_path()));
        let name = out.file_name().and_then(|n| n.to_str()).unwrap_or("");
        assert!(name.starts_with("test_"), "{name}");
        assert!(name.ends_with(".md"), "{name}");
    }

    #[test]
    fn stdout_output_bypasses_folder_and_timestamp() {
        let args = Args {
            input: "src".to_string(),
            output: "-".to_string(), // stdout
            filter: vec![],
            ignore: vec![],
            line_numbers: false,
            preview: false,
            token_count: false,
            yes: false,
            diff_only: false,
            clear_cache: false,
            encoding: "o200k_base".to_string(),
            init: false,
            max_tokens: None,
            signatures: false,
            structure: false,
            truncate: "smart".to_string(),
            visibility: "all".to_string(),
            max_file_size: "256K".to_string(),
            hidden: false,
            include_secrets: false,
            file_metadata: false,
            include_lockfiles: false,
        };

        let config = Config {
            output_folder: Some("docs".to_string()),
            timestamped_output: Some(true),
            ..Default::default()
        };

        let resolution = resolve_final_config(args, Some(config), ExplicitCli::default());
        // `-` must survive untouched — not folded into docs/ nor timestamped.
        assert_eq!(resolution.config.output, "-");
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
            encoding: "o200k_base".to_string(),
            init: false,
            max_tokens: None,
            signatures: false,
            structure: false,
            truncate: "smart".to_string(),
            visibility: "all".to_string(),
            max_file_size: "256K".to_string(),
            hidden: false,
            include_secrets: false,
            file_metadata: false,
            include_lockfiles: false,
        };

        let config = Config {
            auto_diff: Some(true),
            timestamped_output: Some(false), // This should generate a warning
            ..Default::default()
        };

        let resolution = resolve_final_config(args, Some(config), ExplicitCli::default());

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
            encoding: "o200k_base".to_string(),
            init: false,
            max_tokens: None,
            signatures: false,
            structure: false,
            truncate: "smart".to_string(),
            visibility: "all".to_string(),
            max_file_size: "256K".to_string(),
            hidden: false,
            include_secrets: false,
            file_metadata: false,
            include_lockfiles: false,
        };

        let resolution = resolve_final_config(args.clone(), None, ExplicitCli::default());

        assert_eq!(resolution.config.input, args.input);
        assert_eq!(resolution.config.output, args.output);
        assert_eq!(resolution.config.filter, args.filter);
        assert_eq!(resolution.config.ignore, args.ignore);
        assert_eq!(resolution.config.line_numbers, args.line_numbers);
        assert_eq!(resolution.config.file_metadata, args.file_metadata);
        assert!(!resolution.config.file_metadata);
        assert_eq!(resolution.config.preview, args.preview);
        assert_eq!(resolution.config.token_count, args.token_count);
        assert_eq!(resolution.config.yes, args.yes);
        assert_eq!(resolution.config.diff_only, args.diff_only);
        assert!(!resolution.config.auto_diff);
        assert_eq!(resolution.config.diff_context_lines, 3);
        assert!(resolution.warnings.is_empty());
    }

    #[test]
    fn explicit_cli_default_value_overrides_config() {
        // Regression: `--encoding o200k_base` (the clap default value) must still
        // override a non-default config encoding. The value looks like the default,
        // so resolution must rely on the explicit-flag signal, not the value.
        // Same for `--truncate smart` and `--visibility all`.
        let make_args = || Args {
            input: ".".to_string(),
            output: "output.md".to_string(),
            filter: vec![],
            ignore: vec![],
            line_numbers: false,
            preview: false,
            token_count: false,
            yes: false,
            diff_only: false,
            clear_cache: false,
            encoding: "o200k_base".to_string(),
            init: false,
            max_tokens: None,
            signatures: false,
            structure: false,
            truncate: "smart".to_string(),
            visibility: "all".to_string(),
            max_file_size: "256K".to_string(),
            hidden: false,
            include_secrets: false,
            file_metadata: false,
            include_lockfiles: false,
        };
        let config = Config {
            encoding: Some("cl100k_base".to_string()),
            truncate: Some("byte".to_string()),
            visibility: Some("public".to_string()),
            ..Default::default()
        };

        // Flags NOT explicitly passed → config wins (the value equals the default).
        let implicit =
            resolve_final_config(make_args(), Some(config.clone()), ExplicitCli::default());
        assert_eq!(implicit.config.encoding, "cl100k_base");
        assert_eq!(implicit.config.truncate, "byte");
        assert_eq!(implicit.config.visibility, "public");

        // Flags explicitly passed at their default value → CLI wins over config.
        let explicit = resolve_final_config(
            make_args(),
            Some(config),
            ExplicitCli {
                truncate: true,
                visibility: true,
                encoding: true,
                max_file_size: false,
                output: false,
            },
        );
        assert_eq!(explicit.config.encoding, "o200k_base");
        assert_eq!(explicit.config.truncate, "smart");
        assert_eq!(explicit.config.visibility, "all");
    }

    fn bare_args(input: &str, output: &str) -> Args {
        Args {
            input: input.to_string(),
            output: output.to_string(),
            filter: vec![],
            ignore: vec![],
            line_numbers: false,
            preview: false,
            token_count: false,
            yes: false,
            diff_only: false,
            clear_cache: false,
            encoding: "o200k_base".to_string(),
            init: false,
            max_tokens: None,
            signatures: false,
            structure: false,
            truncate: "smart".to_string(),
            visibility: "all".to_string(),
            max_file_size: "256K".to_string(),
            hidden: false,
            include_secrets: false,
            file_metadata: false,
            include_lockfiles: false,
        }
    }

    #[test]
    fn include_lockfiles_cli_wins_over_config() {
        let mut args = bare_args(".", "output.md");
        let from_config = resolve_final_config(
            args.clone(),
            Some(Config {
                include_lockfiles: Some(true),
                ..Default::default()
            }),
            ExplicitCli::default(),
        );
        assert!(from_config.config.include_lockfiles);

        args.include_lockfiles = true;
        let cli_on = resolve_final_config(
            args,
            Some(Config {
                include_lockfiles: Some(false),
                ..Default::default()
            }),
            ExplicitCli::default(),
        );
        assert!(cli_on.config.include_lockfiles);
    }

    fn folder_and_timestamp_config() -> Config {
        Config {
            output: Some("context.md".to_string()),
            output_folder: Some("docs".to_string()),
            timestamped_output: Some(true),
            ..Default::default()
        }
    }

    #[test]
    fn explicit_output_is_verbatim_absolute_and_relative() {
        // B3: an explicit `-o` is not folded into output_folder or timestamped.
        let project = std::env::temp_dir().join("cb-b3-project");
        let wanted = std::env::temp_dir().join("wanted.md");
        let config = folder_and_timestamp_config();

        let absolute = resolve_final_config(
            bare_args(&project.to_string_lossy(), &wanted.to_string_lossy()),
            Some(config.clone()),
            ExplicitCli {
                output: true,
                ..ExplicitCli::default()
            },
        );
        assert_eq!(
            PathBuf::from(&absolute.config.output),
            wanted,
            "absolute -o must be kept verbatim"
        );
        assert!(
            absolute.warnings.is_empty(),
            "unused output_folder must not warn: {:?}",
            absolute.warnings
        );

        let relative = resolve_final_config(
            bare_args(&project.to_string_lossy(), "rel-wanted.md"),
            Some(config),
            ExplicitCli {
                output: true,
                ..ExplicitCli::default()
            },
        );
        assert_eq!(relative.config.output, "rel-wanted.md");
        assert!(relative.warnings.is_empty());
    }

    #[test]
    fn explicit_default_output_name_is_not_rewritten() {
        // `-o output.md` carries the clap default string, but was typed by the
        // user. Config output name, folder, and timestamp must not apply.
        let resolution = resolve_final_config(
            bare_args("proj", "output.md"),
            Some(folder_and_timestamp_config()),
            ExplicitCli {
                output: true,
                ..ExplicitCli::default()
            },
        );
        assert_eq!(resolution.config.output, "output.md");
        assert!(resolution.warnings.is_empty());
    }

    #[test]
    fn explicit_stdout_stays_stdout_with_folder_and_timestamp() {
        let resolution = resolve_final_config(
            bare_args("proj", "-"),
            Some(folder_and_timestamp_config()),
            ExplicitCli {
                output: true,
                ..ExplicitCli::default()
            },
        );
        assert_eq!(resolution.config.output, "-");
    }

    #[test]
    fn omitted_output_anchors_folder_at_project_root() {
        // B20: with no `-o`, relative output_folder is <project>/docs, and
        // timestamped_output still renames the config output stem.
        let project = std::env::temp_dir().join("cb-b20-project");
        let resolution = resolve_final_config(
            bare_args(&project.to_string_lossy(), "output.md"),
            Some(folder_and_timestamp_config()),
            ExplicitCli::default(),
        );
        let out = PathBuf::from(&resolution.config.output);
        let expected_dir = project.join("docs");
        assert_eq!(out.parent(), Some(expected_dir.as_path()));
        let name = out.file_name().and_then(|n| n.to_str()).unwrap_or("");
        assert!(
            name.starts_with("context_") && name.ends_with(".md"),
            "{name}"
        );
        assert!(!name.contains("output.md"));
    }

    #[test]
    fn absolute_output_folder_is_not_rerooted() {
        let project = std::env::temp_dir().join("cb-b20-project");
        let folder = std::env::temp_dir().join("cb-abs-out");
        let config = Config {
            output_folder: Some(folder.to_string_lossy().into_owned()),
            ..Default::default()
        };
        let resolution = resolve_final_config(
            bare_args(&project.to_string_lossy(), "output.md"),
            Some(config),
            ExplicitCli::default(),
        );
        assert_eq!(
            PathBuf::from(&resolution.config.output),
            folder.join("output.md")
        );
    }

    #[test]
    fn relative_project_root_anchors_output_folder() {
        // `cd elsewhere && context-builder -d ../proj` (no `-o`).
        let resolution = resolve_final_config(
            bare_args("../proj", "output.md"),
            Some(Config {
                output_folder: Some("docs".to_string()),
                ..Default::default()
            }),
            ExplicitCli::default(),
        );
        assert_eq!(
            PathBuf::from(&resolution.config.output),
            Path::new("../proj").join("docs").join("output.md")
        );
    }

    #[test]
    fn file_metadata_cli_overrides_config_false() {
        // `--file-metadata` is an opt-in bool (default false). Passing it wins
        // over `file_metadata = false`. Omitting it lets the config key apply.
        let mut args = bare_args(".", "output.md");
        args.file_metadata = true;
        let config_off = Config {
            file_metadata: Some(false),
            ..Default::default()
        };
        let on = resolve_final_config(args.clone(), Some(config_off), ExplicitCli::default());
        assert!(on.config.file_metadata);

        args.file_metadata = false;
        let config_on = Config {
            file_metadata: Some(true),
            ..Default::default()
        };
        let from_config = resolve_final_config(args, Some(config_on), ExplicitCli::default());
        assert!(from_config.config.file_metadata);
    }

    #[test]
    fn content_filter_config_precedence() {
        let make_args = |size: &str, hidden: bool, secrets: bool| {
            let mut args = bare_args(".", "output.md");
            args.max_file_size = size.to_string();
            args.hidden = hidden;
            args.include_secrets = secrets;
            args
        };
        let config = Config {
            max_file_size: Some("1M".to_string()),
            hidden: Some(true),
            include_secrets: Some(true),
            ..Default::default()
        };

        // Omitted CLI flags: config wins, including a non-default size.
        let from_config = resolve_final_config(
            make_args("256K", false, false),
            Some(config.clone()),
            ExplicitCli::default(),
        );
        assert_eq!(from_config.config.max_file_size, "1M");
        assert!(from_config.config.hidden);
        assert!(from_config.config.include_secrets);

        // Explicit `--max-file-size 256K` beats config `1M`. A non-default CLI
        // size wins even without the explicit-flag bit (tests build Args directly).
        let explicit_default = resolve_final_config(
            make_args("256K", false, false),
            Some(config.clone()),
            ExplicitCli {
                max_file_size: true,
                ..ExplicitCli::default()
            },
        );
        assert_eq!(explicit_default.config.max_file_size, "256K");

        let raised = resolve_final_config(
            make_args("0", true, false),
            Some(config.clone()),
            ExplicitCli::default(),
        );
        assert_eq!(raised.config.max_file_size, "0");
        assert!(raised.config.hidden);
        assert!(raised.config.include_secrets); // config still enables it

        let invalid = Config {
            max_file_size: Some("lots".to_string()),
            ..Default::default()
        };
        let warned = resolve_final_config(
            make_args("256K", false, false),
            Some(invalid),
            ExplicitCli::default(),
        );
        assert_eq!(warned.config.max_file_size, "256K");
        assert!(warned.warnings.iter().any(|w| w.contains("max_file_size")));

        // Integer byte counts from TOML arrive as decimal strings.
        let numeric = Config {
            max_file_size: Some("100".to_string()),
            ..Default::default()
        };
        let parsed = resolve_final_config(
            make_args("256K", false, false),
            Some(numeric),
            ExplicitCli::default(),
        );
        assert_eq!(parsed.config.max_file_size, "100");
    }
}
