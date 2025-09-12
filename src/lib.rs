use chrono::Utc;
use clap::{CommandFactory, Parser};

use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::time::Instant;
use tempfile::NamedTempFile;

pub mod cli;
pub mod config;
pub mod diff;
pub mod file_utils;
pub mod markdown;
pub mod token_count;
pub mod tree;

use cli::Args;
use config::load_config;
use diff::{PerFileStatus, diff_file_contents, render_per_file_diffs};
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

    let silent = std::env::var("CB_SILENT")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false);

    let base_path = Path::new(&args.input);

    if !base_path.exists() || !base_path.is_dir() {
        if !silent {
            eprintln!(
                "Error: The specified input directory '{}' does not exist or is not a directory.",
                args.input
            );
        }
        return Ok(());
    }

    let config = load_config().unwrap_or_default();
    // Expose configured diff context lines (if provided) to the diff generator through env
    if let Some(diff_ctx) = config.diff_context_lines {
        if std::env::var("CB_DIFF_CONTEXT_LINES").is_err() {
            unsafe {
                std::env::set_var("CB_DIFF_CONTEXT_LINES", diff_ctx.to_string());
            }
        }
    }

    if !args.preview && !args.token_count && Path::new(&args.output).exists() {
        if !args.yes && !prompter.confirm_overwrite(&args.output)? {
            if !silent {
                println!("Operation cancelled.");
            }
            return Ok(());
        }
    }

    let files = collect_files(base_path, &args.filter, &args.ignore)?;
    let file_tree = build_file_tree(&files, base_path);

    if args.preview {
        if !silent {
            println!("\n# File Tree Structure (Preview)\n");
            print_tree(&file_tree, 0);
        }
        if !args.token_count {
            return Ok(());
        }
    }

    if args.token_count {
        if !silent {
            println!("\n# Token Count Estimation\n");
            let mut total_tokens = 0;
            total_tokens += estimate_tokens("# Directory Structure Report\n\n");
            if !args.filter.is_empty() {
                total_tokens += estimate_tokens(&format!(
                    "This document contains files from the `{}` directory with extensions: {} \n",
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
                    "Custom ignored patterns: {} \n",
                    args.ignore.join(", ")
                ));
            }
            total_tokens += estimate_tokens(&format!(
                "Processed at: {}\n\n",
                Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
            ));
            total_tokens += estimate_tokens("## File Tree Structure\n\n");
            let tree_tokens = count_tree_tokens(&file_tree, 0);
            total_tokens += tree_tokens;
            let file_tokens: usize = files
                .iter()
                .map(|entry| count_file_tokens(base_path, entry, args.line_numbers))
                .sum();
            total_tokens += file_tokens;
            println!("Estimated total tokens: {}", total_tokens);
            println!("File tree tokens: {}", tree_tokens);
            println!("File content tokens: {}", file_tokens);
        }
        return Ok(());
    }

    if !args.yes && !prompter.confirm_processing(files.len())? {
        if !silent {
            println!("Operation cancelled.");
        }
        return Ok(());
    }

    if config.auto_diff.unwrap_or(false) && config.timestamped_output.unwrap_or(false) {
        // 1. Generate current canonical (no diff) into temp file
        let output_path = Path::new(&args.output);
        let temp_file = NamedTempFile::new()?;

        generate_markdown(
            temp_file.path().to_str().unwrap(),
            &args.input,
            &args.filter,
            &args.ignore,
            &file_tree,
            &files,
            base_path,
            args.line_numbers,
        )?;

        // 2. Load previous canonical (if any)
        let cache_dir = Path::new(".context-builder").join("cache");
        if !cache_dir.exists() {
            let _ = fs::create_dir_all(&cache_dir);
        }
        let cache_file = cache_dir.join("last_canonical.md");
        let previous_canonical = fs::read_to_string(&cache_file).unwrap_or_default();
        let new_canonical = fs::read_to_string(temp_file.path())?;

        // 3. Extract per-file pure contents (only code blocks) for both versions
        fn extract_file_contents(text: &str) -> (String, String, HashMap<String, String>) {
            let mut prefix_end = text.len();
            if let Some(idx) = text.find("\n### File: `") {
                prefix_end = idx;
            }
            let (prefix, rest) = text.split_at(prefix_end);
            let mut files_map: HashMap<String, String> = HashMap::new();
            let files_raw = rest.trim_start().to_string();

            let mut current_path: Option<String> = None;
            let mut in_code = false;
            let mut current_lines: Vec<String> = Vec::new();

            fn strip_line_number(line: &str) -> &str {
                let trimmed = line.trim_start();
                if let Some(pipe_idx) = trimmed.find('|') {
                    let (left, right) = trimmed.split_at(pipe_idx);
                    if left.trim().chars().all(|c| c.is_ascii_digit()) {
                        return right.trim_start_matches('|').trim_start();
                    }
                }
                line
            }

            for line in rest.lines() {
                if line.starts_with("### File: `") {
                    if let Some(p) = current_path.take() {
                        files_map.insert(p, current_lines.join("\n"));
                        current_lines.clear();
                    }
                    if let Some(after) = line.strip_prefix("### File: `") {
                        if let Some(end) = after.find('`') {
                            current_path = Some(after[..end].to_string());
                        }
                    }
                    in_code = false;
                    continue;
                }

                if line.starts_with("```") {
                    in_code = !in_code;
                    continue;
                }

                if in_code {
                    current_lines.push(strip_line_number(line).to_string());
                }
            }

            if let Some(p) = current_path.take() {
                files_map.insert(p, current_lines.join("\n"));
            }

            (prefix.trim_end().to_string(), files_raw, files_map)
        }

        let (_prev_prefix, _prev_files_raw, prev_map) = extract_file_contents(&previous_canonical);
        let (new_prefix, new_files_raw, new_map) = extract_file_contents(&new_canonical);

        // 4. Compute per-file diffs (skip unchanged)
        let per_file_diffs = diff_file_contents(&prev_map, &new_map, true, None);

        // 5. Partition changes
        let mut added_paths: HashSet<&str> = HashSet::new();
        let mut removed_paths: HashSet<&str> = HashSet::new();
        let mut modified_paths: HashSet<&str> = HashSet::new();

        for d in &per_file_diffs {
            match d.status {
                PerFileStatus::Added => {
                    added_paths.insert(d.path.as_str());
                }
                PerFileStatus::Removed => {
                    removed_paths.insert(d.path.as_str());
                }
                PerFileStatus::Modified => {
                    modified_paths.insert(d.path.as_str());
                }
                PerFileStatus::Unchanged => {}
            }
        }

        // 6. Prepare Files section with annotations for added files
        // We only annotate the display; added files produce no diff section.
        let mut files_section = new_files_raw.trim_start().to_string();
        if !added_paths.is_empty() {
            // For safety do replacements on a line-by-line rebuild to avoid nested replacements.
            let mut rebuilt = String::new();
            let mut lines = files_section.lines().peekable();
            while let Some(line) = lines.next() {
                if let Some(after) = line.strip_prefix("### File: `") {
                    if let Some(end) = after.find('`') {
                        let path = &after[..end];
                        rebuilt.push_str(line);
                        rebuilt.push('\n');
                        // The original generator emits a blank line after heading; we add status before metadata
                        if added_paths.contains(path) {
                            rebuilt.push('\n');
                            rebuilt.push_str("_Status: Added_\n");
                        }
                        continue;
                    }
                }
                rebuilt.push_str(line);
                rebuilt.push('\n');
            }
            files_section = rebuilt;
        }

        // 7. Build final document
        let mut final_doc = String::new();
        final_doc.push_str(&new_prefix);
        final_doc.push_str("\n\n");

        // Change Summary always shows additions/removals/modifications if any
        if !(added_paths.is_empty() && removed_paths.is_empty() && modified_paths.is_empty()) {
            final_doc.push_str("## Change Summary\n\n");
            for p in added_paths.iter().copied().collect::<Vec<_>>() {
                final_doc.push_str(&format!("- Added: `{}`\n", p));
            }
            for p in removed_paths.iter().copied().collect::<Vec<_>>() {
                final_doc.push_str(&format!("- Removed: `{}`\n", p));
            }
            for p in modified_paths.iter().copied().collect::<Vec<_>>() {
                final_doc.push_str(&format!("- Modified: `{}`\n", p));
            }
            final_doc.push_str("\n");
        }

        // File Differences: ONLY modified files (no added / removed)
        let modified_diffs: Vec<_> = per_file_diffs
            .iter()
            .filter(|d| matches!(d.status, PerFileStatus::Modified))
            .collect();

        if !modified_diffs.is_empty() {
            final_doc.push_str("## File Differences\n\n");
            // Render only modified diffs
            final_doc.push_str(&render_per_file_diffs(
                &modified_diffs
                    .iter()
                    .map(|d| (*d).clone())
                    .collect::<Vec<_>>(),
            ));
            final_doc.push('\n');
        }

        // Only include full file bodies when not in diff-only mode
        if !args.diff_only && !files_section.is_empty() {
            final_doc.push_str("## Files\n\n");

            final_doc.push_str(&files_section);

            if !final_doc.ends_with('\n') {
                final_doc.push('\n');
            }
        }

        // 8. Write output
        let mut final_output = fs::File::create(output_path)?;
        final_output.write_all(final_doc.as_bytes())?;

        // 9. Update canonical cache
        if let Err(e) = fs::write(&cache_file, &new_canonical) {
            if !silent {
                eprintln!("Warning: failed to update canonical cache: {e}");
            }
        }

        let duration = start_time.elapsed();
        if !silent {
            if modified_diffs.is_empty() {
                println!(
                    "Documentation created successfully (no modified file content): {}",
                    args.output
                );
            } else {
                println!(
                    "Documentation created successfully with modified file diffs: {}",
                    args.output
                );
            }
            println!("Processing time: {:.2?}", duration);
        }
        return Ok(());
    }

    // Standard (non auto-diff) generation
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
    let mut args = Args::parse();
    let config = load_config();

    if std::env::args().len() == 1 && config.is_none() {
        Args::command().print_help()?;
        return Ok(());
    }

    if let Some(config) = config {
        if args.output == "output.md" {
            if let Some(output) = config.output {
                args.output = output;
            }
        }
        if args.filter.is_empty() {
            if let Some(filter) = config.filter {
                args.filter = filter;
            }
        }
        if args.ignore.is_empty() {
            if let Some(ignore) = config.ignore {
                args.ignore = ignore;
            }
        }
        if !args.line_numbers {
            if let Some(line_numbers) = config.line_numbers {
                args.line_numbers = line_numbers;
            }
        }
        if !args.preview {
            if let Some(preview) = config.preview {
                args.preview = preview;
            }
        }
        if !args.token_count {
            if let Some(token_count) = config.token_count {
                args.token_count = token_count;
            }
        }
        if !args.yes {
            if let Some(yes) = config.yes {
                args.yes = yes;
            }
        }

        let mut output_folder_path: Option<PathBuf> = None;
        if let Some(output_folder) = config.output_folder {
            let mut path = PathBuf::from(output_folder.clone());
            path.push(&args.output);
            args.output = path.to_str().unwrap().to_string();
            output_folder_path = Some(PathBuf::from(output_folder));
        }

        if let Some(true) = config.timestamped_output {
            let timestamp = Utc::now().format("%Y%m%d%H%M%S").to_string();

            let path = Path::new(&args.output);

            let stem = path.file_stem().unwrap().to_str().unwrap();

            let extension = path.extension().unwrap().to_str().unwrap();

            let new_filename = format!("{}_{}.{}", stem, timestamp, extension);

            if let Some(output_folder) = output_folder_path {
                args.output = output_folder
                    .join(new_filename)
                    .to_str()
                    .unwrap()
                    .to_string();
            } else {
                let new_path = path.with_file_name(new_filename);

                args.output = new_path.to_str().unwrap().to_string();
            }
        }

        // Apply diff_only from config (CLI flag still has precedence if user supplied --diff-only)
        if let Some(true) = config.diff_only {
            args.diff_only = true;
        }
    }
    run_with_args(args, &DefaultPrompter)
}
