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

    /// File extensions to include (e.g., --filter rs,toml)
    #[clap(short = 'f', long, value_delimiter = ',')]
    pub filter: Vec<String>,

    /// Folder or file names to ignore (e.g., --ignore target --ignore lock)
    #[clap(short = 'i', long)]
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
    fn rejects_invalid_enum_values() {
        // value_parser restricts these flags to their allowed sets, so invalid
        // values now error at parse time instead of being silently coerced.
        assert!(Args::try_parse_from(["context-builder", "--truncate", "bogus"]).is_err());
        assert!(Args::try_parse_from(["context-builder", "--visibility", "bogus"]).is_err());
        assert!(Args::try_parse_from(["context-builder", "--encoding", "bogus"]).is_err());
    }
}
