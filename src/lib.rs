use clap::Parser;
use log::info;
use std::io;
use std::path::Path;
use std::time::Instant;

pub mod cli;
pub mod file_utils;
pub mod markdown;
pub mod tree;

use cli::Args;
use file_utils::{collect_files, confirm_processing, confirm_overwrite};
use markdown::generate_markdown;
use tree::{build_file_tree, print_tree};


pub fn run() -> io::Result<()> {
    env_logger::init();
    let args = Args::parse();
    let start_time = Instant::now();

    let base_path = Path::new(&args.input);

    // Pre-run checks
    if !base_path.exists() || !base_path.is_dir() {

        // Pretty print error message and exit gracefully
        eprintln!("Error: The specified input directory '{}' does not exist or is not a directory.", args.input);
        return Ok(());
    }

    // Check if an output file already exists
    if Path::new(&args.output).exists() {
        // Ask for user confirmation to overwrite
        if !confirm_overwrite(&args.output)? {
            println!("Operation cancelled.");
            return Ok(());
        }
    }

    // --- 1. Collect files --- //
    info!("Starting file collection...");
    let files = collect_files(base_path, &args.filter, &args.ignore)?;
    info!("Found {} files to process.", files.len());

    // --- 2. Build file tree --- //
    let file_tree = build_file_tree(&files, base_path);

    // --- 3. Handle preview mode --- //
    if args.preview {
        println!("\n# File Tree Structure (Preview)\n");
        print_tree(&file_tree, 0);
        return Ok(());
    }

    // --- 4. Get user confirmation --- //
    if !confirm_processing(files.len())? {
        println!("Operation cancelled.");
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
    println!("Documentation created successfully: {}", args.output);
    println!("Processing time: {:.2?}", duration);

    Ok(())
}
