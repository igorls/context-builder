use chrono::Utc;
use clap::{CommandFactory, Parser};

use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::time::Instant;

pub mod cache;
pub mod cli;
pub mod config;
pub mod config_resolver;
pub mod diff;
pub mod file_utils;
pub mod markdown;
pub mod state;
pub mod token_count;
pub mod tree;

use cache::CacheManager;
use cli::Args;
use config::{Config, load_config_from_path};
use diff::{PerFileStatus, render_per_file_diffs};
use file_utils::{collect_files, confirm_overwrite, confirm_processing};
use markdown::generate_markdown;
use state::{ProjectState, StateComparison};
use token_count::{count_file_tokens, count_tree_tokens, estimate_tokens};
use tree::{build_file_tree, print_tree};

/// Configuration for diff operations
#[derive(Debug, Clone)]
pub struct DiffConfig {
    pub context_lines: usize,
    pub enabled: bool,
    pub diff_only: bool,
}

impl Default for DiffConfig {
    fn default() -> Self {
        Self {
            context_lines: 3,
            enabled: false,
            diff_only: false,
        }
    }
}

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
pub fn run_with_args(args: Args, config: Config, prompter: &impl Prompter) -> io::Result<()> {
    let start_time = Instant::now();

    let silent = std::env::var("CB_SILENT")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false);

    // Use the finalized args passed in from run()
    let final_args = args;
    let base_path = Path::new(&final_args.input);

    if !base_path.exists() || !base_path.is_dir() {
        if !silent {
            eprintln!(
                "Error: The specified input directory '{}' does not exist or is not a directory.",
                final_args.input
            );
        }
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!(
                "Input directory '{}' does not exist or is not a directory",
                final_args.input
            ),
        ));
    }

    // Create diff configuration from config
    let diff_config = if config.auto_diff.unwrap_or(false) {
        Some(DiffConfig {
            context_lines: config.diff_context_lines.unwrap_or(3),
            enabled: true,
            diff_only: final_args.diff_only,
        })
    } else {
        None
    };

    if !final_args.preview
        && !final_args.token_count
        && Path::new(&final_args.output).exists()
        && !final_args.yes
        && !prompter.confirm_overwrite(&final_args.output)?
    {
        if !silent {
            println!("Operation cancelled.");
        }
        return Err(io::Error::new(
            io::ErrorKind::Interrupted,
            "Operation cancelled by user",
        ));
    }

    let files = collect_files(base_path, &final_args.filter, &final_args.ignore)?;
    let file_tree = build_file_tree(&files, base_path);

    if final_args.preview {
        if !silent {
            println!("\n# File Tree Structure (Preview)\n");
            print_tree(&file_tree, 0);
        }
        if !final_args.token_count {
            return Ok(());
        }
    }

    if final_args.token_count {
        if !silent {
            println!("\n# Token Count Estimation\n");
            let mut total_tokens = 0;
            total_tokens += estimate_tokens("# Directory Structure Report\n\n");
            if !final_args.filter.is_empty() {
                total_tokens += estimate_tokens(&format!(
                    "This document contains files from the `{}` directory with extensions: {} \n",
                    final_args.input,
                    final_args.filter.join(", ")
                ));
            } else {
                total_tokens += estimate_tokens(&format!(
                    "This document contains all files from the `{}` directory, optimized for LLM consumption.\n",
                    final_args.input
                ));
            }
            if !final_args.ignore.is_empty() {
                total_tokens += estimate_tokens(&format!(
                    "Custom ignored patterns: {} \n",
                    final_args.ignore.join(", ")
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
                .map(|entry| count_file_tokens(base_path, entry, final_args.line_numbers))
                .sum();
            total_tokens += file_tokens;
            println!("Estimated total tokens: {}", total_tokens);
            println!("File tree tokens: {}", tree_tokens);
            println!("File content tokens: {}", file_tokens);
        }
        return Ok(());
    }

    if !final_args.yes && !prompter.confirm_processing(files.len())? {
        if !silent {
            println!("Operation cancelled.");
        }
        return Err(io::Error::new(
            io::ErrorKind::Interrupted,
            "Operation cancelled by user",
        ));
    }

    if config.auto_diff.unwrap_or(false) {
        // 1. Create current project state
        let current_state =
            ProjectState::from_files(&files, base_path, &config, final_args.line_numbers)?;

        // 2. Initialize cache manager and load previous state
        let cache_manager = CacheManager::new(base_path, &config);
        let previous_state = match cache_manager.read_cache() {
            Ok(state) => state,
            Err(e) => {
                if !silent {
                    eprintln!(
                        "Warning: Failed to read cache (proceeding without diff): {}",
                        e
                    );
                }
                None
            }
        };

        let diff_cfg = diff_config.as_ref().unwrap();

        // 3. Compare states and generate diff if previous state exists
        let comparison = previous_state
            .as_ref()
            .map(|prev_state| current_state.compare_with(prev_state));

        // 4. Generate markdown with diff annotations
        let final_doc = generate_markdown_with_diff(
            &current_state,
            comparison.as_ref(),
            &final_args,
            &file_tree,
            diff_cfg,
        )?;

        // 5. Write output
        let output_path = Path::new(&final_args.output);
        let mut final_output = fs::File::create(output_path)?;
        final_output.write_all(final_doc.as_bytes())?;

        // 6. Update cache with current state
        if let Err(e) = cache_manager.write_cache(&current_state)
            && !silent
        {
            eprintln!("Warning: failed to update state cache: {}", e);
        }

        let duration = start_time.elapsed();
        if !silent {
            if let Some(comp) = &comparison {
                if comp.summary.has_changes() {
                    println!(
                        "Documentation created successfully with {} changes: {}",
                        comp.summary.total_changes, final_args.output
                    );
                } else {
                    println!(
                        "Documentation created successfully (no changes detected): {}",
                        final_args.output
                    );
                }
            } else {
                println!(
                    "Documentation created successfully (initial state): {}",
                    final_args.output
                );
            }
            println!("Processing time: {:.2?}", duration);
        }
        return Ok(());
    }

    // Standard (non auto-diff) generation
    generate_markdown(
        &final_args.output,
        &final_args.input,
        &final_args.filter,
        &final_args.ignore,
        &file_tree,
        &files,
        base_path,
        final_args.line_numbers,
        config.encoding_strategy.as_deref(),
    )?;

    let duration = start_time.elapsed();
    if !silent {
        println!("Documentation created successfully: {}", final_args.output);
        println!("Processing time: {:.2?}", duration);
    }

    Ok(())
}

/// Generate markdown document with diff annotations
fn generate_markdown_with_diff(
    current_state: &ProjectState,
    comparison: Option<&StateComparison>,
    _args: &Args,
    file_tree: &crate::tree::FileTree,
    diff_config: &DiffConfig,
) -> io::Result<String> {
    let mut output = String::new();

    // Header
    output.push_str("# Directory Structure Report\n\n");

    if !current_state.metadata.filters.is_empty() {
        output.push_str(&format!(
            "This document contains files from the `{}` directory with extensions: {}\n",
            current_state.metadata.project_name,
            current_state.metadata.filters.join(", ")
        ));
    } else {
        output.push_str(&format!(
            "This document contains all files from the `{}` directory, optimized for LLM consumption.\n",
            current_state.metadata.project_name
        ));
    }

    if !current_state.metadata.ignores.is_empty() {
        output.push_str(&format!(
            "Custom ignored patterns: {}\n",
            current_state.metadata.ignores.join(", ")
        ));
    }

    output.push_str(&format!("Processed at: {}\n\n", current_state.timestamp));

    // File Tree
    output.push_str("## File Tree Structure\n\n");
    output.push_str(&format_file_tree(file_tree));
    output.push('\n');

    // Change Summary and Diffs (if comparison available)
    match comparison {
        Some(comp) if comp.summary.has_changes() => {
            // Change Summary
            output.push_str(&comp.summary.to_markdown());

            // File Differences: only modified files
            let modified_diffs: Vec<_> = comp
                .file_diffs
                .iter()
                .filter(|d| matches!(d.status, PerFileStatus::Modified))
                .collect();

            if !modified_diffs.is_empty() {
                output.push_str("## File Differences\n\n");
                output.push_str(&render_per_file_diffs(
                    &modified_diffs
                        .iter()
                        .map(|d| (*d).clone())
                        .collect::<Vec<_>>(),
                ));
                output.push('\n');
            }
        }
        _ => {}
    }

    // Files section: in diff_only mode, only include added files (LLM needs full context for new files)
    let files_to_include: Vec<_> = if diff_config.diff_only {
        // In diff_only mode, only include added files
        comparison
            .map(|comp| {
                current_state
                    .files
                    .iter()
                    .filter(|(path, _)| comp.summary.added.contains(path))
                    .collect()
            })
            .unwrap_or_default()
    } else {
        // In normal mode, include all files
        current_state.files.iter().collect()
    };

    if !files_to_include.is_empty() {
        if diff_config.diff_only {
            output.push_str("## Added Files\n\n");
        } else {
            output.push_str("## Files\n\n");
        }

        for (path, file_state) in files_to_include {
            output.push_str(&format!("\n### File: `{}`\n\n", path.display()));

            // Check if this file was added
            let is_added = comparison
                .as_ref()
                .map(|c| c.summary.added.contains(path))
                .unwrap_or(false);

            if is_added {
                output.push_str("_Status: Added_\n\n");
            }

            output.push_str(&format!("- Size: {} bytes\n", file_state.size));
            let modified_time = file_state
                .modified
                .duration_since(std::time::UNIX_EPOCH)
                .ok()
                .and_then(|d| chrono::DateTime::from_timestamp(d.as_secs() as i64, 0))
                .map(|dt| dt.format("%Y-%m-%d %H:%M:%S UTC").to_string())
                .unwrap_or_else(|| "Unknown".to_string());
            output.push_str(&format!("- Modified: {}\n\n", modified_time));

            // Determine language for syntax highlighting
            let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("text");
            let language = match extension {
                "rs" => "rust",
                "js" => "javascript",
                "ts" => "typescript",
                "jsx" => "jsx",
                "tsx" => "tsx",
                "json" => "json",
                "toml" => "toml",
                "md" => "markdown",
                "yaml" | "yml" => "yaml",
                "html" => "html",
                "css" => "css",
                "py" => "python",
                "java" => "java",
                "cpp" => "cpp",
                "c" => "c",
                "h" => "c",
                "hpp" => "cpp",
                "sql" => "sql",
                "sh" => "bash",
                "xml" => "xml",
                "lock" => "toml",
                _ => extension,
            };

            output.push_str(&format!("```{}\n", language));

            if current_state.metadata.line_numbers {
                for (i, line) in file_state.content.lines().enumerate() {
                    output.push_str(&format!("{:>4} | {}\n", i + 1, line));
                }
            } else {
                output.push_str(&file_state.content);
                if !file_state.content.ends_with('\n') {
                    output.push('\n');
                }
            }

            output.push_str("```\n");
        }
    }

    Ok(output)
}

fn format_file_tree(tree: &crate::tree::FileTree) -> String {
    let mut output = Vec::new();
    crate::tree::write_tree_to_file(&mut output, tree, 0).unwrap();
    String::from_utf8(output).unwrap()
}
pub fn run() -> io::Result<()> {
    env_logger::init();
    let args = Args::parse();

    // Determine project root first
    let project_root = Path::new(&args.input);
    let config = load_config_from_path(project_root);

    // Handle early clear-cache request (runs even if no config or other args)
    if args.clear_cache {
        let cache_path = project_root.join(".context-builder").join("cache");
        if cache_path.exists() {
            match fs::remove_dir_all(&cache_path) {
                Ok(()) => println!("Cache cleared: {}", cache_path.display()),
                Err(e) => eprintln!("Failed to clear cache ({}): {}", cache_path.display(), e),
            }
        } else {
            println!("No cache directory found at {}", cache_path.display());
        }
        return Ok(());
    }

    if std::env::args().len() == 1 && config.is_none() {
        Args::command().print_help()?;
        return Ok(());
    }

    // Resolve final configuration using the new config resolver
    let resolution = crate::config_resolver::resolve_final_config(args, config.clone());

    // Print warnings if any
    let silent = std::env::var("CB_SILENT")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false);

    if !silent {
        for warning in &resolution.warnings {
            eprintln!("Warning: {}", warning);
        }
    }

    // Convert resolved config back to Args for run_with_args
    let final_args = Args {
        input: resolution.config.input,
        output: resolution.config.output,
        filter: resolution.config.filter,
        ignore: resolution.config.ignore,
        line_numbers: resolution.config.line_numbers,
        preview: resolution.config.preview,
        token_count: resolution.config.token_count,
        yes: resolution.config.yes,
        diff_only: resolution.config.diff_only,
        clear_cache: resolution.config.clear_cache,
    };

    // Create final Config with resolved values
    let final_config = Config {
        auto_diff: Some(resolution.config.auto_diff),
        diff_context_lines: Some(resolution.config.diff_context_lines),
        ..config.unwrap_or_default()
    };

    run_with_args(final_args, final_config, &DefaultPrompter)
}
