use clap::Parser;

use crate::content_filter::parse_max_file_size_arg;

/// CLI tool to aggregate directory contents into a single Markdown file optimized for LLM consumption
#[derive(Parser, Debug, Clone)]
#[clap(author, version, about)]
pub struct Args {
    /// Directory path to process
    #[clap(short = 'd', long, default_value = ".")]
    pub input: String,

    /// Output file path
    #[clap(short, long, default_value = "output.md")]
    pub output: String,

    /// File types to include (e.g., --filter rs,toml).
    ///
    /// Values are ripgrep file types, not exact extensions: `toml` also matches
    /// Cargo.lock, and `md` also matches `.markdown` and `.mdx`. A leading `.`
    /// or `*.` is stripped and the value is lowercased, so `.rs`, `*.rs`, and
    /// `RS` all mean `rs`. Dependency lockfiles stay out unless you pass
    /// `--include-lockfiles`, even when a type like `toml` matches them.
    #[clap(short = 'f', long, value_delimiter = ',')]
    pub filter: Vec<String>,

    /// Include dependency lockfiles (Cargo.lock, package-lock.json, uv.lock, …).
    ///
    /// Skipped by default. A filter that matches a lockfile's type, such as
    /// `-f toml` or `-f lock`, does not override this. Set
    /// `include_lockfiles = true` in context-builder.toml for the same opt-in;
    /// this flag wins when both are set.
    #[clap(long, default_value_t = false)]
    pub include_lockfiles: bool,

    /// Paths or gitignore-style globs to ignore (e.g. -i docs,assets, -i '*.lock', -i crates/core)
    #[clap(short = 'i', long, value_delimiter = ',', value_name = "PATTERN")]
    pub ignore: Vec<String>,

    /// Preview mode: only print the file tree to the console, don't generate the documentation file
    #[clap(long)]
    pub preview: bool,

    /// Token count mode: estimate the total token count of the final document
    #[clap(long)]
    pub token_count: bool,

    /// Add line numbers to code blocks in the output
    #[clap(long)]
    pub line_numbers: bool,

    /// Include per-file size and modification time under each file header.
    /// Off by default: those lines cost tokens and change whenever a file's
    /// mtime changes (checkout, copy, `touch`), even if the bytes did not.
    #[clap(long)]
    pub file_metadata: bool,

    /// Overwrite an existing output file without asking. The >100-file confirmation
    /// was removed in v0.11.0; this flag is still accepted and only affects the overwrite prompt
    #[clap(short = 'y', long)]
    pub yes: bool,

    /// Maximum token budget for the output. Files are truncated/skipped when exceeded.
    #[clap(long)]
    pub max_tokens: Option<usize>,

    /// Output only diffs (omit full file contents; requires auto-diff & timestamped output)
    #[clap(long, default_value_t = false)]
    pub diff_only: bool,

    /// Clear the cached project state and exit
    #[clap(long)]
    pub clear_cache: bool,

    /// Initialize a new context-builder.toml config file in the current directory
    #[clap(long)]
    pub init: bool,

    /// Extract function/class signatures only (requires tree-sitter feature)
    #[clap(long)]
    pub signatures: bool,

    /// Extract code structure (imports, exports, symbol counts) - requires tree-sitter feature
    #[clap(long)]
    pub structure: bool,

    /// Truncation mode for max-tokens: "smart" (AST boundaries) or "byte"
    #[clap(long, value_name = "MODE", value_parser = ["smart", "byte"], default_value = "smart")]
    pub truncate: String,

    /// Filter signatures by visibility: "all", "public", or "private"
    #[clap(long, value_parser = ["all", "public", "private"], default_value = "all")]
    pub visibility: String,

    /// Tokenizer encoding used for `--token-count` and `--max-tokens` budgeting.
    /// "o200k_base" matches GPT-4o/o-series (default); "cl100k_base" matches GPT-4/3.5.
    #[clap(long, value_parser = ["o200k_base", "cl100k_base"], default_value = "o200k_base")]
    pub encoding: String,

    /// Skip files larger than SIZE. Examples: `256K` (default), `1M`, `262144` (bytes).
    /// `K`/`M`/`G` are powers of 1024 (`KB` and `KiB` are accepted). `0` disables the limit.
    /// Files strictly larger than SIZE are listed under `## Skipped` as "too large".
    /// An explicit `--filter` of an asset extension does not bypass this limit.
    #[clap(long, value_name = "SIZE", default_value = "256K", value_parser = parse_max_file_size_arg)]
    pub max_file_size: String,

    /// Include hidden files and directories (names starting with `.`).
    ///
    /// Off by default, which is unchanged: `.github/`, `.gitignore`, `.env`, and other
    /// dot paths are omitted. With `--hidden` those paths are included — for example
    /// `.github/workflows/ci.yml`, `.gitignore`, and `.cargo/config.toml`.
    ///
    /// `--hidden` does not follow symlinks, does not override `.gitignore` / `.ignore` /
    /// `--ignore` / the built-in heavy-directory ignores, and does not descend into
    /// version-control metadata (`.git`, `.hg`, `.svn`, `.bzr`). Likely-secret files
    /// stay skipped unless `--include-secrets` is also set. `.env.example` and
    /// `.env.sample` are not secrets, but they are hidden, so they appear only with
    /// `--hidden`.
    #[clap(long)]
    pub hidden: bool,

    /// Include likely-secret files that are skipped by default.
    ///
    /// Skipped names: `id_rsa`, `id_dsa`, `id_ecdsa`, `id_ed25519` (and their `*_sk`
    /// forms), `*.pem`, `*.key`, `*.p12`, `*.pfx`, `*.ppk`, `credentials*.json`,
    /// `.env` and `.env.*` except `.env.example` and `.env.sample`, plus `.npmrc` /
    /// `.pypirc` when they contain a token. Public keys (`id_rsa.pub`) are kept.
    /// Warnings name the path and the category only — never file contents.
    ///
    /// `--hidden` does not imply this flag. Dotfile secrets such as `.env` also need
    /// `--hidden` or they are never visited. Naming a secret extension in `--filter`
    /// (`pem`, `key`, `p12`, `pfx`, `ppk`) includes that extension as well; name-only
    /// secrets are not extensions, so use this flag for `id_rsa`, `credentials.json`,
    /// and `.env`.
    #[clap(long)]
    pub include_secrets: bool,
}

#[cfg(test)]
mod tests {
    use super::Args;
    use clap::{CommandFactory, Parser};

    #[test]
    fn parses_with_no_args() {
        let res = Args::try_parse_from(["context-builder"]);
        assert!(res.is_ok(), "Expected success when no args are provided");
    }

    #[test]
    fn parses_all_flags_and_options() {
        let args = Args::try_parse_from([
            "context-builder",
            "--input",
            "some/dir",
            "--output",
            "ctx.md",
            "--filter",
            "rs",
            "--filter",
            "toml",
            "--ignore",
            "target",
            "--ignore",
            "node_modules",
            "--preview",
            "--token-count",
            "--line-numbers",
            "--diff-only",
            "--clear-cache",
        ])
        .expect("should parse");

        assert_eq!(args.input, "some/dir");
        assert_eq!(args.output, "ctx.md");
        assert_eq!(args.filter, vec!["rs".to_string(), "toml".to_string()]);
        assert_eq!(
            args.ignore,
            vec!["target".to_string(), "node_modules".to_string()]
        );
        assert!(args.preview);
        assert!(args.token_count);
        assert!(args.line_numbers);
        assert!(args.diff_only);
        assert!(args.clear_cache);
    }

    #[test]
    fn short_flags_parse_correctly() {
        let args = Args::try_parse_from([
            "context-builder",
            "-d",
            ".",
            "-o",
            "out.md",
            "-f",
            "md",
            "-f",
            "rs",
            "-i",
            "target",
            "-i",
            ".git",
        ])
        .expect("should parse");

        assert_eq!(args.input, ".");
        assert_eq!(args.output, "out.md");
        assert_eq!(args.filter, vec!["md".to_string(), "rs".to_string()]);
        assert_eq!(args.ignore, vec!["target".to_string(), ".git".to_string()]);
        assert!(!args.preview);
        assert!(!args.line_numbers);
        assert!(!args.file_metadata);
        assert!(!args.clear_cache);
    }

    #[test]
    fn defaults_for_options_when_not_provided() {
        let args = Args::try_parse_from(["context-builder", "-d", "proj"]).expect("should parse");

        assert_eq!(args.input, "proj");
        assert_eq!(args.output, "output.md");
        assert!(args.filter.is_empty());
        assert!(args.ignore.is_empty());
        assert!(!args.preview);
        assert!(!args.line_numbers);
        assert!(!args.file_metadata);
        assert!(!args.diff_only);
        assert!(!args.clear_cache);
        assert!(!args.include_lockfiles);
    }

    #[test]
    fn parses_include_lockfiles_flag() {
        let args = Args::try_parse_from(["context-builder", "--include-lockfiles"])
            .expect("should parse include-lockfiles");
        assert!(args.include_lockfiles);

        let help = Args::command().render_long_help().to_string();
        assert!(help.contains("--include-lockfiles"));
        assert!(help.contains("-f toml") || help.contains("toml"));
    }

    #[test]
    fn parses_file_metadata_flag() {
        let args = Args::try_parse_from(["context-builder", "--file-metadata"])
            .expect("should parse file-metadata flag");
        assert!(args.file_metadata);

        let omitted = Args::try_parse_from(["context-builder", "-d", "."]).expect("should parse");
        assert!(!omitted.file_metadata);
    }

    #[test]
    fn parses_diff_only_flag() {
        let args = Args::try_parse_from(["context-builder", "--diff-only"])
            .expect("should parse diff-only flag");
        assert!(args.diff_only);
        assert!(!args.clear_cache);
    }

    #[test]
    fn parses_clear_cache_flag() {
        let args = Args::try_parse_from(["context-builder", "--clear-cache"])
            .expect("should parse clear-cache flag");
        assert!(args.clear_cache);
        assert!(!args.diff_only);
    }

    #[test]
    fn parses_signatures_flag() {
        let args = Args::try_parse_from(["context-builder", "--signatures"])
            .expect("should parse signatures flag");
        assert!(args.signatures);
    }

    #[test]
    fn parses_structure_flag() {
        let args = Args::try_parse_from(["context-builder", "--structure"])
            .expect("should parse structure flag");
        assert!(args.structure);
    }

    #[test]
    fn parses_truncate_mode() {
        let args = Args::try_parse_from(["context-builder", "--truncate", "byte"])
            .expect("should parse truncate flag");
        assert_eq!(args.truncate, "byte");

        let args_default =
            Args::try_parse_from(["context-builder"]).expect("should parse with default truncate");
        assert_eq!(args_default.truncate, "smart");
    }

    #[test]
    fn parses_visibility_filter() {
        let args = Args::try_parse_from(["context-builder", "--visibility", "public"])
            .expect("should parse visibility flag");
        assert_eq!(args.visibility, "public");

        let args_default = Args::try_parse_from(["context-builder"])
            .expect("should parse with default visibility");
        assert_eq!(args_default.visibility, "all");
    }

    #[test]
    fn parses_encoding_flag_with_default() {
        let args = Args::try_parse_from(["context-builder", "--encoding", "cl100k_base"])
            .expect("should parse encoding flag");
        assert_eq!(args.encoding, "cl100k_base");

        let args_default =
            Args::try_parse_from(["context-builder"]).expect("should parse with default encoding");
        assert_eq!(args_default.encoding, "o200k_base");
    }

    #[test]
    fn output_flag_value_source_distinguishes_explicit_from_default() {
        use clap::CommandFactory;

        // The resolver keys off this id. `-o output.md` must count as explicit
        // even though the string equals the clap default.
        let explicit = Args::command().get_matches_from(["context-builder", "-o", "wanted.md"]);
        assert_eq!(
            explicit.value_source("output"),
            Some(clap::parser::ValueSource::CommandLine)
        );

        let explicit_default =
            Args::command().get_matches_from(["context-builder", "-o", "output.md"]);
        assert_eq!(
            explicit_default.value_source("output"),
            Some(clap::parser::ValueSource::CommandLine)
        );

        let omitted = Args::command().get_matches_from(["context-builder", "-d", "proj"]);
        assert_eq!(
            omitted.value_source("output"),
            Some(clap::parser::ValueSource::DefaultValue)
        );
    }

    #[test]
    fn filter_help_documents_ripgrep_types() {
        use clap::CommandFactory;
        // `--help` renders the long help, which carries the ripgrep-type note.
        let help = Args::command().render_long_help().to_string();
        assert!(
            help.contains("ripgrep"),
            "expected --help to mention ripgrep file types:\n{help}"
        );
        assert!(
            help.contains("Cargo.lock"),
            "expected --help to mention that toml expands beyond *.toml:\n{help}"
        );
    }

    #[test]
    fn ignore_comma_delimiter_splits_and_repeated_flags_append() {
        let comma = Args::try_parse_from(["context-builder", "-i", "docs,assets"]).expect("parse");
        assert_eq!(comma.ignore, vec!["docs".to_string(), "assets".to_string()]);

        let long =
            Args::try_parse_from(["context-builder", "--ignore", "docs,assets"]).expect("parse");
        assert_eq!(long.ignore, vec!["docs".to_string(), "assets".to_string()]);

        // Repeated -i still appends, including after a comma-separated value.
        let repeated = Args::try_parse_from([
            "context-builder",
            "-i",
            "docs,assets",
            "-i",
            "target",
            "--ignore",
            "*.lock",
        ])
        .expect("parse");
        assert_eq!(
            repeated.ignore,
            vec![
                "docs".to_string(),
                "assets".to_string(),
                "target".to_string(),
                "*.lock".to_string()
            ]
        );
    }

    #[test]
    fn ignore_help_documents_globs_paths_and_a_working_example() {
        use clap::CommandFactory;

        let help = Args::command().render_long_help().to_string();
        assert!(
            help.contains("gitignore-style globs"),
            "help should say ignore accepts gitignore-style globs: {help}"
        );
        assert!(
            help.contains("docs,assets"),
            "help should show the comma form: {help}"
        );
        assert!(
            help.contains("*.lock"),
            "help should show a glob example: {help}"
        );
        assert!(
            help.contains("crates/core"),
            "help should show a path example: {help}"
        );
        assert!(
            !help.contains("--ignore lock"),
            "help must not suggest a bare name that matches nothing useful: {help}"
        );
    }

    #[test]
    fn rejects_invalid_enum_values() {
        // value_parser restricts these flags to their allowed sets, so invalid
        // values now error at parse time instead of being silently coerced.
        assert!(Args::try_parse_from(["context-builder", "--truncate", "bogus"]).is_err());
        assert!(Args::try_parse_from(["context-builder", "--visibility", "bogus"]).is_err());
        assert!(Args::try_parse_from(["context-builder", "--encoding", "bogus"]).is_err());
    }

    #[test]
    fn parses_content_filter_flags() {
        let args = Args::try_parse_from([
            "context-builder",
            "--max-file-size",
            "1M",
            "--hidden",
            "--include-secrets",
        ])
        .expect("should parse content-filter flags");
        assert_eq!(args.max_file_size, "1M");
        assert!(args.hidden);
        assert!(args.include_secrets);

        let defaults = Args::try_parse_from(["context-builder"]).expect("defaults");
        assert_eq!(defaults.max_file_size, "256K");
        assert!(!defaults.hidden);
        assert!(!defaults.include_secrets);

        assert!(Args::try_parse_from(["context-builder", "--max-file-size", "nope"]).is_err());
        assert!(Args::try_parse_from(["context-builder", "--max-file-size", "0"]).is_ok());
    }

    #[test]
    fn help_documents_skip_policy() {
        let mut help = Vec::new();
        Args::command()
            .write_long_help(&mut help)
            .expect("help renders");
        let help = String::from_utf8(help).expect("help is utf-8");
        assert!(help.contains("--max-file-size"));
        assert!(help.contains("--hidden"));
        assert!(help.contains("--include-secrets"));
        assert!(help.contains(".env.example"));
        assert!(help.contains(".git"));
    }
}
