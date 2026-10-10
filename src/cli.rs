use clap::Parser;

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
    /// `RS` all mean `rs`.
    #[clap(short = 'f', long, value_delimiter = ',')]
    pub filter: Vec<String>,

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

    /// Automatically answer yes to all prompts
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
}

#[cfg(test)]
mod tests {
    use super::Args;
    use clap::Parser;

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
        assert!(!args.diff_only);
        assert!(!args.clear_cache);
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
}
