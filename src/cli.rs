use clap::Parser;

/// CLI tool to aggregate directory contents into a single Markdown file optimized for LLM consumption
#[derive(Parser, Debug)]
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

    /// Output only diffs (omit full file contents; requires auto-diff & timestamped output)

    /// Defaults to false.
    #[clap(long, default_value_t = false)]
    pub diff_only: bool,
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
    }

    #[test]
    fn parses_diff_only_flag() {
        let args = Args::try_parse_from(["context-builder", "--diff-only"])
            .expect("should parse diff-only flag");
        assert!(args.diff_only);
    }
}
