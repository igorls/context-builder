use clap::Parser;

/// CLI tool to aggregate directory contents into a single markdown file optimized for LLM consumption
#[derive(Parser, Debug)]
#[clap(author, version, about, arg_required_else_help = true)]
pub struct Args {
    /// Directory path to process
    #[clap(short = 'd', long, default_value = ".")]
    pub input: String,

    /// Output file path
    #[clap(short, long, default_value = "output.md")]
    pub output: String,

    /// File extensions to include (e.g., --filter rs --filter toml)
    #[clap(short = 'f', long)]
    pub filter: Vec<String>,

    /// Folder or file names to ignore (e.g., --ignore target --ignore lock)
    #[clap(short = 'i', long)]
    pub ignore: Vec<String>,

    /// Preview mode: only print the file tree to console, don't generate the documentation file
    #[clap(long)]
    pub preview: bool,

    /// Add line numbers to code blocks in the output
    #[clap(long)]
    pub line_numbers: bool,
}