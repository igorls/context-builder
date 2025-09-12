use chrono::Utc;
use clap::Parser;
use log::info;
use std::io;
use std::path::Path;
use std::time::Instant;

pub mod cli;
pub mod file_utils;
pub mod markdown;
pub mod token_count;
pub mod tree;

use cli::Args;
use file_utils::{collect_files, confirm_overwrite, confirm_processing};
use markdown::generate_markdown;
use token_count::{count_file_tokens, count_tree_tokens, estimate_tokens};
use tree::{build_file_tree, print_tree};

pub trait Prompter {
    fn confirm_processing(&self, file_count: usize) -> io::Result<bool>;
    fn confirm_overwrite(&self, file_path: &str) -> io::Result<bool>;
}

pub struct DefaultPrompter;

impl Prompter for DefaultPrompter {
    fn confirm_processing(&self, file_count: usize) -> io::Result<bool> {
        confirm_processing(file_count)
    }
    fn confirm_overwrite(&self, file_path: &str) -> io::Result<bool> {
        confirm_overwrite(file_path)
    }
}

pub fn run_with_args(args: Args, prompter: &impl Prompter) -> io::Result<()> {
    let start_time = Instant::now();

    // If CB_SILENT is set to "1" or "true" (case-insensitive), suppress user-facing prints.
    let silent = std::env::var("CB_SILENT")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false);

    let base_path = Path::new(&args.input);

    // Pre-run checks
    if !base_path.exists() || !base_path.is_dir() {
        if !silent {
            eprintln!(
                "Error: The specified input directory '{}' does not exist or is not a directory.",
                args.input
            );
        }
        return Ok(());
    }

    // Check if an output file already exists (skip in preview and token count modes)
    if !args.preview && !args.token_count && Path::new(&args.output).exists() {
        // Ask for user confirmation to overwrite
        if !prompter.confirm_overwrite(&args.output)? {
            if !silent {
                println!("Operation cancelled.");
            }
            return Ok(());
        }
    }

    // --- 1. Collect files --- //
    info!("Starting file collection...");
    let files = collect_files(base_path, &args.filter, &args.ignore)?;
    info!("Found {} files to process.", files.len());

    // --- 2. Build file tree --- //
    let file_tree = build_file_tree(&files, base_path);

    // --- 3. Handle preview and token count modes --- //
    if args.preview {
        if !silent {
            println!("\n# File Tree Structure (Preview)\n");
            print_tree(&file_tree, 0);
        }
        // If only preview mode is set, return early
        if !args.token_count {
            return Ok(());
        }
        // If both preview and token_count are set, continue to token count mode
    }

    if args.token_count {
        if !silent {
            println!("\n# Token Count Estimation\n");

            // Count tokens for the header section
            let mut total_tokens = 0;
            total_tokens += estimate_tokens("# Directory Structure Report\n\n");

            if !args.filter.is_empty() {
                total_tokens += estimate_tokens(&format!(
                    "This document contains files from the `{}` directory with extensions: {}\n",
                    args.input,
                    args.filter.join(", ")
                ));
            } else {
                total_tokens += estimate_tokens(&format!(
                    "This document contains all files from the `{}` directory, optimized for LLM consumption.\n",
                    args.input
                ));
            }

            if !args.ignore.is_empty() {
                total_tokens += estimate_tokens(&format!(
                    "Custom ignored patterns: {}\n",
                    args.ignore.join(", ")
                ));
            }

            total_tokens += estimate_tokens(&format!(
                "Processed at: {}\n\n",
                Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
            ));

            // File tree section (tree already printed earlier if --preview was supplied)
            total_tokens += estimate_tokens("## File Tree Structure\n\n");

            let tree_tokens = count_tree_tokens(&file_tree, 0);
            total_tokens += tree_tokens;

            // Count tokens for all files
            let file_tokens: usize = files
                .iter()
                .map(|entry| count_file_tokens(base_path, entry, args.line_numbers))
                .sum();
            total_tokens += file_tokens;

            println!("Estimated total tokens: {}", total_tokens);
            println!("File tree tokens: {}", tree_tokens);
            println!("File content tokens: {}", file_tokens);
            println!(
                "\nNote: This is an estimation based on character count heuristics.\nActual token counts may vary depending on the specific tokenizer used by your LLM."
            );
        }
        return Ok(());
    }

    // --- 5. Get user confirmation --- //
    if !prompter.confirm_processing(files.len())? {
        if !silent {
            println!("Operation cancelled.");
        }
        return Ok(());
    }

    // --- 5. Generate the markdown file --- //
    generate_markdown(
        &args.output,
        &args.input,
        &args.filter,
        &args.ignore,
        &file_tree,
        &files,
        base_path,
        args.line_numbers,
    )?;

    let duration = start_time.elapsed();

    if !silent {
        println!("Documentation created successfully: {}", args.output);
        println!("Processing time: {:.2?}", duration);
    }

    Ok(())
}

pub fn run() -> io::Result<()> {
    env_logger::init();
    let args = Args::parse();
    run_with_args(args, &DefaultPrompter)
}
