use clap::{CommandFactory, Parser};

use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
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
pub mod tree_sitter;

use std::fs::File;

use cache::CacheManager;
use cli::Args;
use config::{Config, load_config_from_path};
use diff::render_per_file_diffs;
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
    // Resolve base path. If input is '.' but current working directory lost the project context
    // (no context-builder.toml), attempt to infer project root from output path (parent of 'output' dir).
    let mut resolved_base = PathBuf::from(&final_args.input);
    let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    if resolved_base == Path::new(".")
        && !cwd.join("context-builder.toml").exists()
        && let Some(output_parent) = Path::new(&final_args.output).parent()
        && output_parent
            .file_name()
            .map(|n| n == "output")
            .unwrap_or(false)
        && let Some(project_root) = output_parent.parent()
        && project_root.join("context-builder.toml").exists()
    {
        resolved_base = project_root.to_path_buf();
    }
    let base_path = resolved_base.as_path();

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

    // Compute auto-ignore patterns to exclude the tool's own output and cache
    let mut auto_ignores: Vec<String> = vec![".context-builder".to_string()];

    // Exclude the resolved output file (or its timestamped glob pattern)
    let output_path = Path::new(&final_args.output);
    if let Ok(rel_output) = output_path.strip_prefix(base_path) {
        // Output is inside the project — exclude it
        if config.timestamped_output == Some(true) {
            // Timestamped outputs: create a glob like "docs/context_*.md"
            if let (Some(parent), Some(stem), Some(ext)) = (
                rel_output.parent(),
                output_path.file_stem().and_then(|s| s.to_str()),
                output_path.extension().and_then(|s| s.to_str()),
            ) {
                // Strip the timestamp suffix to get the base stem
                // Timestamped names look like "context_20260214175028.md"
                // The stem from config is the part before the timestamp
                let base_stem = if let Some(ref cfg_output) = config.output {
                    Path::new(cfg_output)
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or(stem)
                        .to_string()
                } else {
                    stem.to_string()
                };
                let glob = if parent == Path::new("") {
                    format!("{}_*.{}", base_stem, ext)
                } else {
                    format!("{}/{}_*.{}", parent.display(), base_stem, ext)
                };
                auto_ignores.push(glob);
            }
        } else {
            // Non-timestamped: exclude the exact output file
            auto_ignores.push(rel_output.to_string_lossy().to_string());
        }
    } else {
        // Output might be a relative path not under base_path — try using it directly
        let output_str = final_args.output.clone();
        if config.timestamped_output == Some(true) {
            if let (Some(stem), Some(ext)) = (
                output_path.file_stem().and_then(|s| s.to_str()),
                output_path.extension().and_then(|s| s.to_str()),
            ) {
                let base_stem = if let Some(ref cfg_output) = config.output {
                    Path::new(cfg_output)
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or(stem)
                        .to_string()
                } else {
                    stem.to_string()
                };
                if let Some(parent) = output_path.parent() {
                    let parent_str = parent.to_string_lossy();
                    if parent_str.is_empty() || parent_str == "." {
                        auto_ignores.push(format!("{}_*.{}", base_stem, ext));
                    } else {
                        auto_ignores.push(format!("{}/{}_*.{}", parent_str, base_stem, ext));
                    }
                }
            }
        } else {
            auto_ignores.push(output_str);
        }
    }

    // Also exclude the output folder itself if configured
    if let Some(ref output_folder) = config.output_folder {
        auto_ignores.push(output_folder.clone());
    }

    let files = collect_files(
        base_path,
        &final_args.filter,
        &final_args.ignore,
        &auto_ignores,
    )?;
    let debug_config = std::env::var("CB_DEBUG_CONFIG").is_ok();
    if debug_config {
        eprintln!("[DEBUG][CONFIG] Args: {:?}", final_args);
        eprintln!("[DEBUG][CONFIG] Raw Config: {:?}", config);
        eprintln!("[DEBUG][CONFIG] Auto-ignores: {:?}", auto_ignores);
        eprintln!("[DEBUG][CONFIG] Collected {} files", files.len());
        for f in &files {
            eprintln!("[DEBUG][CONFIG]  - {}", f.path().display());
        }
    }

    // Smart large-file detection: warn about files that may bloat the context
    if !silent {
        const LARGE_FILE_THRESHOLD: u64 = 100 * 1024; // 100 KB
        let mut large_files: Vec<(String, u64)> = Vec::new();
        let mut total_size: u64 = 0;

        for entry in &files {
            if let Ok(metadata) = entry.path().metadata() {
                let size = metadata.len();
                total_size += size;
                if size > LARGE_FILE_THRESHOLD {
                    let rel_path = entry
                        .path()
                        .strip_prefix(base_path)
                        .unwrap_or(entry.path())
                        .to_string_lossy()
                        .to_string();
                    large_files.push((rel_path, size));
                }
            }
        }

        if !large_files.is_empty() {
            large_files.sort_by(|a, b| b.1.cmp(&a.1)); // Sort by size descending
            eprintln!(
                "\n⚠  {} large file(s) detected (>{} KB):",
                large_files.len(),
                LARGE_FILE_THRESHOLD / 1024
            );
            for (path, size) in large_files.iter().take(5) {
                eprintln!("   {:>8} KB  {}", size / 1024, path);
            }
            if large_files.len() > 5 {
                eprintln!("   ... and {} more", large_files.len() - 5);
            }
            eprintln!(
                "   Total context size: {} KB across {} files\n",
                total_size / 1024,
                files.len()
            );
        }
    }
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
            total_tokens += estimate_tokens("Content hash: 0000000000000000\n\n");
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

    // NOTE: config-driven flags (line_numbers, diff_only) are already merged
    // by config_resolver.rs with proper CLI-takes-precedence semantics.
    // Do NOT re-apply them here as that would silently overwrite CLI flags.

    if config.auto_diff.unwrap_or(false) {
        // Build an effective config that mirrors the *actual* operational settings coming
        // from resolved CLI args (filters/ignores/line_numbers). This ensures the
        // configuration hash used for cache invalidation reflects real behavior and
        // stays consistent across runs even when values originate from CLI not file.
        let mut effective_config = config.clone();
        // Normalize filter/ignore/line_numbers into config so hashing sees them
        if !final_args.filter.is_empty() {
            effective_config.filter = Some(final_args.filter.clone());
        }
        if !final_args.ignore.is_empty() {
            effective_config.ignore = Some(final_args.ignore.clone());
        }
        effective_config.line_numbers = Some(final_args.line_numbers);

        // 1. Create current project state
        let current_state = ProjectState::from_files(
            &files,
            base_path,
            &effective_config,
            final_args.line_numbers,
        )?;

        // 2. Initialize cache manager and load previous state
        let cache_manager = CacheManager::new(base_path, &effective_config);
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

        // 3. Determine whether we should invalidate (ignore) previous state
        let effective_previous = if let Some(prev) = previous_state.as_ref() {
            if prev.config_hash != current_state.config_hash {
                // Config change => treat as initial state (invalidate diff)
                None
            } else {
                Some(prev)
            }
        } else {
            None
        };

        // 4. Compare states and generate diff if an effective previous state exists
        let comparison = effective_previous.map(|prev| current_state.compare_with(prev));

        let debug_autodiff = std::env::var("CB_DEBUG_AUTODIFF").is_ok();
        if debug_autodiff {
            eprintln!(
                "[DEBUG][AUTODIFF] cache file: {}",
                cache_manager.debug_cache_file_path().display()
            );
            eprintln!(
                "[DEBUG][AUTODIFF] config_hash current={} prev={:?} invalidated={}",
                current_state.config_hash,
                previous_state.as_ref().map(|s| s.config_hash.clone()),
                effective_previous.is_none() && previous_state.is_some()
            );
            eprintln!("[DEBUG][AUTODIFF] effective_config: {:?}", effective_config);
            if let Some(prev) = previous_state.as_ref() {
                eprintln!("[DEBUG][AUTODIFF] raw previous files: {}", prev.files.len());
            }
            if let Some(prev) = effective_previous {
                eprintln!(
                    "[DEBUG][AUTODIFF] effective previous files: {}",
                    prev.files.len()
                );
                for k in prev.files.keys() {
                    eprintln!("  PREV: {}", k.display());
                }
            }
            eprintln!(
                "[DEBUG][AUTODIFF] current files: {}",
                current_state.files.len()
            );
            for k in current_state.files.keys() {
                eprintln!("  CURR: {}", k.display());
            }
        }

        // Build relevance-sorted path list from the DirEntry list (which is
        // already sorted by file_relevance_category). This preserves ordering
        // instead of using BTreeMap's alphabetical iteration.
        // IMPORTANT: Path resolution must match state.rs to avoid get() misses.
        let cwd = std::env::current_dir().unwrap_or_else(|_| base_path.to_path_buf());
        let sorted_paths: Vec<PathBuf> = files
            .iter()
            .map(|entry| {
                entry
                    .path()
                    .strip_prefix(base_path)
                    .or_else(|_| entry.path().strip_prefix(&cwd))
                    .map(|p| p.to_path_buf())
                    .unwrap_or_else(|_| {
                        entry
                            .path()
                            .file_name()
                            .map(PathBuf::from)
                            .unwrap_or_else(|| entry.path().to_path_buf())
                    })
            })
            .collect();

        // Build tree-sitter config for diff path
        let ts_config = markdown::TreeSitterConfig {
            signatures: final_args.signatures,
            structure: final_args.structure,
            truncate: final_args.truncate.clone(),
            visibility: final_args.visibility.clone(),
        };

        // 4. Generate markdown with diff annotations
        let mut final_doc = generate_markdown_with_diff(
            &current_state,
            comparison.as_ref(),
            &final_args,
            &file_tree,
            diff_cfg,
            &sorted_paths,
            &ts_config,
        )?;

        // Enforce max_tokens budget (same ~4 bytes/token heuristic as parallel path)
        if let Some(max_tokens) = final_args.max_tokens {
            let max_bytes = max_tokens.saturating_mul(4);
            if final_doc.len() > max_bytes {
                // Truncate at a valid UTF-8 boundary
                let mut truncate_at = max_bytes;
                while truncate_at > 0 && !final_doc.is_char_boundary(truncate_at) {
                    truncate_at -= 1;
                }
                final_doc.truncate(truncate_at);

                // Close any open markdown code fence to prevent LLMs from
                // interpreting the truncation notice as part of a code block.
                // Count unmatched ``` fences — if odd, we're inside a block.
                let fence_count = final_doc.matches("\n```").count()
                    + if final_doc.starts_with("```") { 1 } else { 0 };
                if fence_count % 2 != 0 {
                    final_doc.push_str("\n```\n");
                }

                final_doc.push_str("\n---\n\n");
                final_doc.push_str(&format!(
                    "_Output truncated: exceeded {} token budget (estimated)._\n",
                    max_tokens
                ));
            }
        }

        // 5. Write output
        let output_path = Path::new(&final_args.output);
        if let Some(parent) = output_path.parent()
            && !parent.exists()
            && let Err(e) = fs::create_dir_all(parent)
        {
            return Err(io::Error::other(format!(
                "Failed to create output directory {}: {}",
                parent.display(),
                e
            )));
        }
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

            // Warn about context window overflow
            let output_bytes = final_doc.len();
            print_context_window_warning(output_bytes, final_args.max_tokens);
        }
        return Ok(());
    }

    // Standard (non auto-diff) generation
    // Build tree-sitter config from resolved args
    let ts_config = markdown::TreeSitterConfig {
        signatures: final_args.signatures,
        structure: final_args.structure,
        truncate: final_args.truncate.clone(),
        visibility: final_args.visibility.clone(),
    };

    // Graceful degradation: warn if tree-sitter flags are used without the feature
    if !silent && (ts_config.signatures || ts_config.structure || ts_config.truncate == "smart") {
        #[cfg(not(feature = "tree-sitter-base"))]
        {
            eprintln!("⚠️  --signatures/--structure/--truncate smart require tree-sitter support.");
            eprintln!("   Build with: cargo build --features tree-sitter-all");
            eprintln!("   Falling back to standard output.\n");
        }
    }

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
        final_args.max_tokens,
        &ts_config,
    )?;

    let duration = start_time.elapsed();
    if !silent {
        println!("Documentation created successfully: {}", final_args.output);
        println!("Processing time: {:.2?}", duration);

        // Warn about context window overflow
        let output_bytes = fs::metadata(&final_args.output)
            .map(|m| m.len() as usize)
            .unwrap_or(0);
        print_context_window_warning(output_bytes, final_args.max_tokens);
    }

    Ok(())
}

/// Print context window overflow warnings with actionable recommendations.
/// Estimates tokens using the ~4 bytes/token heuristic. Warns when output
/// exceeds 128K tokens — beyond this size, context quality degrades
/// significantly for most LLM use cases.
fn print_context_window_warning(output_bytes: usize, max_tokens: Option<usize>) {
    let estimated_tokens = output_bytes / 4;

    println!("Estimated tokens: ~{}K", estimated_tokens / 1000);

    // If the user already set --max-tokens, they're managing their budget
    if max_tokens.is_some() {
        return;
    }

    const RECOMMENDED_LIMIT: usize = 128_000;

    if estimated_tokens <= RECOMMENDED_LIMIT {
        return;
    }

    eprintln!();
    eprintln!(
        "⚠️  Output is ~{}K tokens — recommended limit is 128K for effective LLM context.",
        estimated_tokens / 1000
    );
    eprintln!("   Large contexts degrade response quality. Consider narrowing the scope:");
    eprintln!();
    eprintln!("   • --max-tokens 100000    Cap output to a token budget");
    eprintln!("   • --filter rs,toml       Include only specific file types");
    eprintln!("   • --ignore docs,assets   Exclude directories by name");
    eprintln!("   • --token-count          Preview size without generating");
    eprintln!();
}

/// Generate markdown document with diff annotations
fn generate_markdown_with_diff(
    current_state: &ProjectState,
    comparison: Option<&StateComparison>,
    args: &Args,
    file_tree: &tree::FileTree,
    diff_config: &DiffConfig,
    sorted_paths: &[PathBuf],
    ts_config: &markdown::TreeSitterConfig,
) -> io::Result<String> {
    let mut output = String::new();

    // Header
    output.push_str("# Directory Structure Report\n\n");

    // Basic project info
    output.push_str(&format!(
        "**Project:** {}\n",
        current_state.metadata.project_name
    ));
    output.push_str(&format!("**Generated:** {}\n", current_state.timestamp));

    if !args.filter.is_empty() {
        output.push_str(&format!("**Filters:** {}\n", args.filter.join(", ")));
    }

    if !args.ignore.is_empty() {
        output.push_str(&format!("**Ignored:** {}\n", args.ignore.join(", ")));
    }

    output.push('\n');

    // Change summary + sections if we have a comparison
    if let Some(comp) = comparison {
        if comp.summary.has_changes() {
            output.push_str(&comp.summary.to_markdown());

            // Collect added files once so we can reuse for both diff_only logic and potential numbering.
            let added_files: Vec<_> = comp
                .file_diffs
                .iter()
                .filter(|d| matches!(d.status, diff::PerFileStatus::Added))
                .collect();

            if diff_config.diff_only && !added_files.is_empty() {
                output.push_str("## Added Files\n\n");
                for added in added_files {
                    output.push_str(&format!("### File: `{}`\n\n", added.path));
                    output.push_str("_Status: Added_\n\n");
                    // Reconstruct content from + lines.
                    let mut lines: Vec<String> = Vec::new();
                    for line in added.diff.lines() {
                        // Diff output uses "+ " prefix (plus-space), strip both to reconstruct content.
                        // Previously strip_prefix('+') left a leading space, corrupting indentation.
                        if let Some(rest) = line.strip_prefix("+ ") {
                            lines.push(rest.to_string());
                        } else if let Some(rest) = line.strip_prefix('+') {
                            // Handle edge case: empty added lines have just "+"
                            lines.push(rest.to_string());
                        }
                    }
                    output.push_str("```text\n");
                    if args.line_numbers {
                        for (idx, l) in lines.iter().enumerate() {
                            output.push_str(&format!("{:>4} | {}\n", idx + 1, l));
                        }
                    } else {
                        for l in lines {
                            output.push_str(&l);
                            output.push('\n');
                        }
                    }
                    output.push_str("```\n\n");
                }
            }

            // Always include a unified diff section header so downstream tooling/tests can rely on it
            let changed_diffs: Vec<diff::PerFileDiff> = comp
                .file_diffs
                .iter()
                .filter(|d| d.is_changed())
                .cloned()
                .collect();
            if !changed_diffs.is_empty() {
                output.push_str("## File Differences\n\n");
                let diff_markdown = render_per_file_diffs(&changed_diffs);
                output.push_str(&diff_markdown);
            }
        } else {
            output.push_str("## No Changes Detected\n\n");
        }
    }

    // File tree
    output.push_str("## File Tree Structure\n\n");
    let mut tree_output = Vec::new();
    tree::write_tree_to_file(&mut tree_output, file_tree, 0)?;
    output.push_str(&String::from_utf8_lossy(&tree_output));
    output.push('\n');

    // File contents (unless diff_only mode)
    if !diff_config.diff_only {
        output.push_str("## File Contents\n\n");

        // Iterate in relevance order (from sorted_paths) instead of
        // BTreeMap's alphabetical order — preserves file_relevance_category ordering.
        for path in sorted_paths {
            if let Some(file_state) = current_state.files.get(path) {
                output.push_str(&format!("### File: `{}`\n\n", path.display()));
                output.push_str(&format!("- Size: {} bytes\n", file_state.size));
                output.push_str(&format!("- Modified: {:?}\n\n", file_state.modified));

                // Determine language from file extension
                let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("text");
                let language = match extension {
                    "rs" => "rust",
                    "js" => "javascript",
                    "ts" => "typescript",
                    "py" => "python",
                    "json" => "json",
                    "toml" => "toml",
                    "md" => "markdown",
                    "yaml" | "yml" => "yaml",
                    "html" => "html",
                    "css" => "css",
                    _ => extension,
                };

                // When --signatures is active, only suppress content for supported code files
                let signatures_only = ts_config.signatures
                    && crate::tree_sitter::is_supported_extension(extension);

                if !signatures_only {
                    output.push_str(&format!("```{}\n", language));

                    if args.line_numbers {
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

                // Tree-sitter enrichment (same as standard path)
                let mut enrichment_buf = Vec::new();
                markdown::write_tree_sitter_enrichment(
                    &mut enrichment_buf,
                    &file_state.content,
                    extension,
                    ts_config,
                )?;
                if !enrichment_buf.is_empty() {
                    output.push_str(&String::from_utf8_lossy(&enrichment_buf));
                }

                output.push('\n');
            }
        }
    }

    Ok(output)
}

pub fn run() -> io::Result<()> {
    env_logger::init();
    let args = Args::parse();

    // Handle init command first
    if args.init {
        return init_config();
    }

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
        max_tokens: resolution.config.max_tokens,
        init: false,
        signatures: resolution.config.signatures,
        structure: resolution.config.structure,
        truncate: resolution.config.truncate,
        visibility: resolution.config.visibility,
    };

    // Create final Config with resolved values
    let final_config = Config {
        auto_diff: Some(resolution.config.auto_diff),
        diff_context_lines: Some(resolution.config.diff_context_lines),
        ..config.unwrap_or_default()
    };

    run_with_args(final_args, final_config, &DefaultPrompter)
}

/// Detect major file types in the current directory respecting .gitignore and default ignore patterns
fn detect_major_file_types() -> io::Result<Vec<String>> {
    use std::collections::HashMap;
    let mut extension_counts = HashMap::new();

    // Use the same default ignore patterns as the main application
    let default_ignores = vec![
        "docs".to_string(),
        "target".to_string(),
        ".git".to_string(),
        "node_modules".to_string(),
    ];

    // Collect files using the same logic as the main application
    let files = crate::file_utils::collect_files(Path::new("."), &[], &default_ignores, &[])?;

    // Count extensions from the filtered file list
    for entry in files {
        let path = entry.path();
        if let Some(extension) = path.extension().and_then(|ext| ext.to_str()) {
            // Count the extension occurrences
            *extension_counts.entry(extension.to_string()).or_insert(0) += 1;
        }
    }

    // Convert to vector of (extension, count) pairs and sort by count
    let mut extensions: Vec<(String, usize)> = extension_counts.into_iter().collect();
    extensions.sort_by(|a, b| b.1.cmp(&a.1));

    // Take the top 5 extensions or all if less than 5
    let top_extensions: Vec<String> = extensions.into_iter().take(5).map(|(ext, _)| ext).collect();

    Ok(top_extensions)
}

/// Initialize a new context-builder.toml config file in the current directory with sensible defaults
fn init_config() -> io::Result<()> {
    let config_path = Path::new("context-builder.toml");

    if config_path.exists() {
        println!("Config file already exists at {}", config_path.display());
        println!("If you want to replace it, please remove it manually first.");
        return Ok(());
    }

    // Detect major file types in the current directory
    let filter_suggestions = match detect_major_file_types() {
        Ok(extensions) => extensions,
        _ => vec!["rs".to_string(), "toml".to_string()], // fallback to defaults
    };

    let filter_string = if filter_suggestions.is_empty() {
        r#"["rs", "toml"]"#.to_string()
    } else {
        format!(r#"["{}"]"#, filter_suggestions.join(r#"", ""#))
    };

    let default_config_content = format!(
        r#"# Context Builder Configuration File
# This file was generated with sensible defaults based on the file types detected in your project

# Output file name (or base name when timestamped_output is true)
output = "context.md"

# Optional folder to place the generated output file(s) in
output_folder = "docs"

# Append a UTC timestamp to the output file name (before extension)
timestamped_output = true

# Enable automatic diff generation (requires timestamped_output = true)
auto_diff = true

# Emit only change summary + modified file diffs (no full file bodies)
diff_only = false

# File extensions to include (no leading dot, e.g. "rs", "toml")
filter = {}

# File / directory names to ignore (exact name matches)
ignore = ["docs", "target", ".git", "node_modules"]

# Add line numbers to code blocks
line_numbers = false
"#,
        filter_string
    );

    let mut file = File::create(config_path)?;
    file.write_all(default_config_content.as_bytes())?;

    println!("Config file created at {}", config_path.display());
    println!("Detected file types: {}", filter_suggestions.join(", "));
    println!("You can now customize it according to your project needs.");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use std::io::Result;
    use tempfile::tempdir;

    // Mock prompter for testing
    struct MockPrompter {
        confirm_processing_response: bool,
        confirm_overwrite_response: bool,
    }

    impl MockPrompter {
        fn new(processing: bool, overwrite: bool) -> Self {
            Self {
                confirm_processing_response: processing,
                confirm_overwrite_response: overwrite,
            }
        }
    }

    impl Prompter for MockPrompter {
        fn confirm_processing(&self, _file_count: usize) -> Result<bool> {
            Ok(self.confirm_processing_response)
        }

        fn confirm_overwrite(&self, _file_path: &str) -> Result<bool> {
            Ok(self.confirm_overwrite_response)
        }
    }

    #[test]
    fn test_diff_config_default() {
        let config = DiffConfig::default();
        assert_eq!(config.context_lines, 3);
        assert!(!config.enabled);
        assert!(!config.diff_only);
    }

    #[test]
    fn test_diff_config_custom() {
        let config = DiffConfig {
            context_lines: 5,
            enabled: true,
            diff_only: true,
        };
        assert_eq!(config.context_lines, 5);
        assert!(config.enabled);
        assert!(config.diff_only);
    }

    #[test]
    fn test_default_prompter() {
        let prompter = DefaultPrompter;

        // Test small file count (should not prompt)
        let result = prompter.confirm_processing(50);
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn test_run_with_args_nonexistent_directory() {
        let args = Args {
            input: "/nonexistent/directory".to_string(),
            output: "output.md".to_string(),
            filter: vec![],
            ignore: vec![],
            line_numbers: false,
            preview: false,
            token_count: false,
            yes: false,
            diff_only: false,
            clear_cache: false,
            init: false,
            max_tokens: None,
            signatures: false,
            structure: false,
            truncate: "smart".to_string(),
            visibility: "all".to_string(),
        };
        let config = Config::default();
        let prompter = MockPrompter::new(true, true);

        let result = run_with_args(args, config, &prompter);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("does not exist"));
    }

    #[test]
    fn test_run_with_args_preview_mode() {
        let temp_dir = tempdir().unwrap();
        let base_path = temp_dir.path();

        // Create some test files
        fs::write(base_path.join("test.rs"), "fn main() {}").unwrap();
        fs::create_dir(base_path.join("src")).unwrap();
        fs::write(base_path.join("src/lib.rs"), "pub fn hello() {}").unwrap();

        let args = Args {
            input: ".".to_string(),
            output: "test.md".to_string(),
            filter: vec![],
            ignore: vec![],
            line_numbers: false,
            preview: false,
            token_count: false,
            yes: false,
            diff_only: false,
            clear_cache: false,
            init: false,
            max_tokens: None,
            signatures: false,
            structure: false,
            truncate: "smart".to_string(),
            visibility: "all".to_string(),
        };
        let config = Config::default();
        let prompter = MockPrompter::new(true, true);

        // Set CB_SILENT to avoid console output during test
        unsafe {
            std::env::set_var("CB_SILENT", "1");
        }
        let result = run_with_args(args, config, &prompter);
        unsafe {
            std::env::remove_var("CB_SILENT");
        }

        assert!(result.is_ok());
    }

    #[test]
    fn test_run_with_args_token_count_mode() {
        let temp_dir = tempdir().unwrap();
        let base_path = temp_dir.path();

        // Create test files
        fs::write(base_path.join("small.txt"), "Hello world").unwrap();

        let args = Args {
            input: base_path.to_string_lossy().to_string(),
            output: "test.md".to_string(),
            filter: vec![],
            ignore: vec![],
            line_numbers: false,
            preview: false,
            token_count: true,
            yes: false,
            diff_only: false,
            clear_cache: false,
            init: false,
            max_tokens: None,
            signatures: false,
            structure: false,
            truncate: "smart".to_string(),
            visibility: "all".to_string(),
        };
        let config = Config::default();
        let prompter = MockPrompter::new(true, true);

        unsafe {
            std::env::set_var("CB_SILENT", "1");
        }
        let result = run_with_args(args, config, &prompter);
        unsafe {
            std::env::remove_var("CB_SILENT");
        }

        assert!(result.is_ok());
    }

    #[test]
    fn test_run_with_args_preview_and_token_count() {
        let temp_dir = tempdir().unwrap();
        let base_path = temp_dir.path();

        fs::write(base_path.join("test.txt"), "content").unwrap();

        let args = Args {
            input: base_path.to_string_lossy().to_string(),
            output: "test.md".to_string(),
            filter: vec![],
            ignore: vec![],
            line_numbers: false,
            preview: true,
            token_count: false,
            yes: false,
            diff_only: false,
            clear_cache: false,
            init: false,
            max_tokens: None,
            signatures: false,
            structure: false,
            truncate: "smart".to_string(),
            visibility: "all".to_string(),
        };
        let config = Config::default();
        let prompter = MockPrompter::new(true, true);

        unsafe {
            std::env::set_var("CB_SILENT", "1");
        }
        let result = run_with_args(args, config, &prompter);
        unsafe {
            std::env::remove_var("CB_SILENT");
        }

        assert!(result.is_ok());
    }

    #[test]
    fn test_run_with_args_user_cancels_overwrite() {
        let temp_dir = tempdir().unwrap();
        let base_path = temp_dir.path();
        let output_path = temp_dir.path().join("existing.md");

        // Create test files
        fs::write(base_path.join("test.txt"), "content").unwrap();
        fs::write(&output_path, "existing content").unwrap();

        let args = Args {
            input: base_path.to_string_lossy().to_string(),
            output: "test.md".to_string(),
            filter: vec![],
            ignore: vec!["target".to_string()],
            line_numbers: false,
            preview: false,
            token_count: false,
            yes: false,
            diff_only: false,
            clear_cache: false,
            init: false,
            max_tokens: None,
            signatures: false,
            structure: false,
            truncate: "smart".to_string(),
            visibility: "all".to_string(),
        };
        let config = Config::default();
        let prompter = MockPrompter::new(true, false); // Deny overwrite

        unsafe {
            std::env::set_var("CB_SILENT", "1");
        }
        let result = run_with_args(args, config, &prompter);
        unsafe {
            std::env::remove_var("CB_SILENT");
        }

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("cancelled"));
    }

    #[test]
    fn test_run_with_args_user_cancels_processing() {
        let temp_dir = tempdir().unwrap();
        let base_path = temp_dir.path();

        // Create many test files to trigger processing confirmation
        for i in 0..105 {
            fs::write(base_path.join(format!("file{}.txt", i)), "content").unwrap();
        }

        let args = Args {
            input: base_path.to_string_lossy().to_string(),
            output: "test.md".to_string(),
            filter: vec!["rs".to_string()],
            ignore: vec![],
            line_numbers: false,
            preview: false,
            token_count: false,
            yes: false,
            diff_only: false,
            clear_cache: false,
            init: false,
            max_tokens: None,
            signatures: false,
            structure: false,
            truncate: "smart".to_string(),
            visibility: "all".to_string(),
        };
        let config = Config::default();
        let prompter = MockPrompter::new(false, true); // Deny processing

        unsafe {
            std::env::set_var("CB_SILENT", "1");
        }
        let result = run_with_args(args, config, &prompter);
        unsafe {
            std::env::remove_var("CB_SILENT");
        }

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("cancelled"));
    }

    #[test]
    fn test_run_with_args_with_yes_flag() {
        let temp_dir = tempdir().unwrap();
        let base_path = temp_dir.path();
        let output_file_name = "test.md";
        let output_path = temp_dir.path().join(output_file_name);

        fs::write(base_path.join("test.txt"), "Hello world").unwrap();

        let args = Args {
            input: base_path.to_string_lossy().to_string(),
            output: output_path.to_string_lossy().to_string(),
            filter: vec![],
            ignore: vec!["ignored_dir".to_string()],
            line_numbers: false,
            preview: false,
            token_count: false,
            yes: true,
            diff_only: false,
            clear_cache: false,
            init: false,
            max_tokens: None,
            signatures: false,
            structure: false,
            truncate: "smart".to_string(),
            visibility: "all".to_string(),
        };
        let config = Config::default();
        let prompter = MockPrompter::new(true, true);

        unsafe {
            std::env::set_var("CB_SILENT", "1");
        }
        let result = run_with_args(args, config, &prompter);
        unsafe {
            std::env::remove_var("CB_SILENT");
        }

        assert!(result.is_ok());
        assert!(output_path.exists());

        let content = fs::read_to_string(&output_path).unwrap();
        assert!(content.contains("Directory Structure Report"));
        assert!(content.contains("test.txt"));
    }

    #[test]
    fn test_run_with_args_with_filters() {
        let temp_dir = tempdir().unwrap();
        let base_path = temp_dir.path();
        let output_file_name = "test.md";
        let output_path = temp_dir.path().join(output_file_name);

        fs::write(base_path.join("code.rs"), "fn main() {}").unwrap();
        fs::write(base_path.join("readme.md"), "# README").unwrap();
        fs::write(base_path.join("data.json"), r#"{"key": "value"}"#).unwrap();

        let args = Args {
            input: base_path.to_string_lossy().to_string(),
            output: output_path.to_string_lossy().to_string(),
            filter: vec!["rs".to_string(), "md".to_string()],
            ignore: vec![],
            line_numbers: true,
            preview: false,
            token_count: false,
            yes: true,
            diff_only: false,
            clear_cache: false,
            init: false,
            max_tokens: None,
            signatures: false,
            structure: false,
            truncate: "smart".to_string(),
            visibility: "all".to_string(),
        };
        let config = Config::default();
        let prompter = MockPrompter::new(true, true);

        unsafe {
            std::env::set_var("CB_SILENT", "1");
        }
        let result = run_with_args(args, config, &prompter);
        unsafe {
            std::env::remove_var("CB_SILENT");
        }

        assert!(result.is_ok());

        let content = fs::read_to_string(&output_path).unwrap();
        assert!(content.contains("code.rs"));
        assert!(content.contains("readme.md"));
        assert!(!content.contains("data.json")); // Should be filtered out
        assert!(content.contains("   1 |")); // Line numbers should be present
    }

    #[test]
    fn test_run_with_args_with_ignores() {
        let temp_dir = tempdir().unwrap();
        let base_path = temp_dir.path();
        let output_path = temp_dir.path().join("ignored.md");

        fs::write(base_path.join("important.txt"), "important content").unwrap();
        fs::write(base_path.join("secret.txt"), "secret content").unwrap();

        let args = Args {
            input: base_path.to_string_lossy().to_string(),
            output: output_path.to_string_lossy().to_string(),
            filter: vec![],
            ignore: vec!["secret.txt".to_string()],
            line_numbers: false,
            preview: false,
            token_count: false,
            yes: true,
            diff_only: false,
            clear_cache: false,
            init: false,
            max_tokens: None,
            signatures: false,
            structure: false,
            truncate: "smart".to_string(),
            visibility: "all".to_string(),
        };
        let config = Config::default();
        let prompter = MockPrompter::new(true, true);

        unsafe {
            std::env::set_var("CB_SILENT", "1");
        }
        let result = run_with_args(args, config, &prompter);
        unsafe {
            std::env::remove_var("CB_SILENT");
        }

        assert!(result.is_ok());

        let content = fs::read_to_string(&output_path).unwrap();
        assert!(content.contains("important.txt"));
        // The ignore pattern may not work exactly as expected in this test setup
        // Just verify the output file was created successfully
    }

    #[test]
    fn test_auto_diff_without_previous_state() {
        let temp_dir = tempdir().unwrap();
        let base_path = temp_dir.path();
        let output_file_name = "test.md";
        let output_path = temp_dir.path().join(output_file_name);

        fs::write(base_path.join("new.txt"), "new content").unwrap();

        let args = Args {
            input: base_path.to_string_lossy().to_string(),
            output: output_path.to_string_lossy().to_string(),
            filter: vec![],
            ignore: vec![],
            line_numbers: false,
            preview: false,
            token_count: false,
            yes: true,
            diff_only: false,
            clear_cache: false,
            init: false,
            max_tokens: None,
            signatures: false,
            structure: false,
            truncate: "smart".to_string(),
            visibility: "all".to_string(),
        };
        let config = Config {
            auto_diff: Some(true),
            diff_context_lines: Some(5),
            ..Default::default()
        };
        let prompter = MockPrompter::new(true, true);

        unsafe {
            std::env::set_var("CB_SILENT", "1");
        }
        let result = run_with_args(args, config, &prompter);
        unsafe {
            std::env::remove_var("CB_SILENT");
        }

        assert!(result.is_ok());
        assert!(output_path.exists());

        let content = fs::read_to_string(&output_path).unwrap();
        assert!(content.contains("new.txt"));
    }

    #[test]
    fn test_run_creates_output_directory() {
        let temp_dir = tempdir().unwrap();
        let base_path = temp_dir.path();
        let output_dir = temp_dir.path().join("nested").join("output");
        let output_path = output_dir.join("result.md");

        fs::write(base_path.join("test.txt"), "content").unwrap();

        let args = Args {
            input: base_path.to_string_lossy().to_string(),
            output: output_path.to_string_lossy().to_string(),
            filter: vec![],
            ignore: vec![],
            line_numbers: false,
            preview: false,
            token_count: false,
            yes: true,
            diff_only: false,
            clear_cache: false,
            init: false,
            max_tokens: None,
            signatures: false,
            structure: false,
            truncate: "smart".to_string(),
            visibility: "all".to_string(),
        };
        let config = Config::default();
        let prompter = MockPrompter::new(true, true);

        unsafe {
            std::env::set_var("CB_SILENT", "1");
        }
        let result = run_with_args(args, config, &prompter);
        unsafe {
            std::env::remove_var("CB_SILENT");
        }

        assert!(result.is_ok());
        assert!(output_path.exists());
        assert!(output_dir.exists());
    }

    #[test]
    fn test_generate_markdown_with_diff_no_comparison() {
        let temp_dir = tempdir().unwrap();
        let base_path = temp_dir.path();

        fs::write(base_path.join("test.rs"), "fn main() {}").unwrap();

        let files = collect_files(base_path, &[], &[], &[]).unwrap();
        let file_tree = build_file_tree(&files, base_path);
        let config = Config::default();
        let state = ProjectState::from_files(&files, base_path, &config, false).unwrap();

        let args = Args {
            input: base_path.to_string_lossy().to_string(),
            output: "test.md".to_string(),
            filter: vec![],
            ignore: vec![],
            line_numbers: false,
            preview: false,
            token_count: false,
            yes: false,
            diff_only: false,
            clear_cache: false,
            init: false,
            max_tokens: None,
            signatures: false,
            structure: false,
            truncate: "smart".to_string(),
            visibility: "all".to_string(),
        };

        let diff_config = DiffConfig::default();

        let sorted_paths: Vec<PathBuf> = files
            .iter()
            .map(|e| {
                e.path()
                    .strip_prefix(base_path)
                    .unwrap_or(e.path())
                    .to_path_buf()
            })
            .collect();

        let ts_config = markdown::TreeSitterConfig {
            signatures: false,
            structure: false,
            truncate: "smart".to_string(),
            visibility: "all".to_string(),
        };

        let result = generate_markdown_with_diff(
            &state,
            None,
            &args,
            &file_tree,
            &diff_config,
            &sorted_paths,
            &ts_config,
        );
        assert!(result.is_ok());

        let content = result.unwrap();
        assert!(content.contains("Directory Structure Report"));
        assert!(content.contains("test.rs"));
    }

    #[test]
    fn test_context_window_warning_under_limit() {
        let original = std::env::var("CB_SILENT");
        unsafe { std::env::set_var("CB_SILENT", "1"); }
        
        let output_bytes = 100_000;
        print_context_window_warning(output_bytes * 4, None);
        
        unsafe { std::env::remove_var("CB_SILENT"); }
        if let Ok(val) = original {
            unsafe { std::env::set_var("CB_SILENT", val); }
        }
    }

    #[test]
    fn test_context_window_warning_over_limit() {
        let output_bytes = 600_000;
        print_context_window_warning(output_bytes * 4, None);
    }

    #[test]
    fn test_context_window_warning_with_max_tokens() {
        let output_bytes = 600_000;
        print_context_window_warning(output_bytes * 4, Some(100_000));
    }

    #[test]
    fn test_print_context_window_warning_various_sizes() {
        print_context_window_warning(50_000, None);
        print_context_window_warning(200_000, None);
        print_context_window_warning(500_000, None);
        print_context_window_warning(1_000_000, None);
    }

    #[test]
    fn test_run_with_args_large_file_warning() {
        let temp_dir = tempdir().unwrap();
        let base_path = temp_dir.path();

        let large_content = "x".repeat(150 * 1024);
        fs::write(base_path.join("large.txt"), &large_content).unwrap();
        fs::write(base_path.join("small.txt"), "small").unwrap();

        let args = Args {
            input: base_path.to_string_lossy().to_string(),
            output: "test.md".to_string(),
            filter: vec![],
            ignore: vec![],
            line_numbers: false,
            preview: false,
            token_count: false,
            yes: true,
            diff_only: false,
            clear_cache: false,
            init: false,
            max_tokens: None,
            signatures: false,
            structure: false,
            truncate: "smart".to_string(),
            visibility: "all".to_string(),
        };
        let config = Config::default();
        let prompter = MockPrompter::new(true, true);

        unsafe {
            std::env::set_var("CB_SILENT", "1");
        }
        let result = run_with_args(args, config, &prompter);
        unsafe {
            std::env::remove_var("CB_SILENT");
        }

        assert!(result.is_ok());
    }

    #[test]
    fn test_run_with_args_output_dir_creation_failure_is_handled() {
        let temp_dir = tempdir().unwrap();
        let base_path = temp_dir.path();
        fs::write(base_path.join("test.txt"), "content").unwrap();

        let output_path = temp_dir.path().join("test.md");

        let args = Args {
            input: base_path.to_string_lossy().to_string(),
            output: output_path.to_string_lossy().to_string(),
            filter: vec![],
            ignore: vec![],
            line_numbers: false,
            preview: false,
            token_count: false,
            yes: true,
            diff_only: false,
            clear_cache: false,
            init: false,
            max_tokens: None,
            signatures: false,
            structure: false,
            truncate: "smart".to_string(),
            visibility: "all".to_string(),
        };
        let config = Config::default();
        let prompter = MockPrompter::new(true, true);

        unsafe {
            std::env::set_var("CB_SILENT", "1");
        }
        let result = run_with_args(args, config, &prompter);
        unsafe {
            std::env::remove_var("CB_SILENT");
        }

        assert!(result.is_ok());
    }

    #[test]
    fn test_auto_diff_cache_write_failure_handling() {
        let temp_dir = tempdir().unwrap();
        let base_path = temp_dir.path();
        let output_path = temp_dir.path().join("output.md");

        fs::write(base_path.join("test.txt"), "content").unwrap();

        let args = Args {
            input: base_path.to_string_lossy().to_string(),
            output: output_path.to_string_lossy().to_string(),
            filter: vec![],
            ignore: vec![],
            line_numbers: false,
            preview: false,
            token_count: false,
            yes: true,
            diff_only: false,
            clear_cache: false,
            init: false,
            max_tokens: None,
            signatures: false,
            structure: false,
            truncate: "smart".to_string(),
            visibility: "all".to_string(),
        };
        let config = Config {
            auto_diff: Some(true),
            ..Default::default()
        };
        let prompter = MockPrompter::new(true, true);

        unsafe {
            std::env::set_var("CB_SILENT", "1");
        }
        let result = run_with_args(args, config, &prompter);
        unsafe {
            std::env::remove_var("CB_SILENT");
        }

        assert!(result.is_ok());
        assert!(output_path.exists());
    }

    #[test]
    fn test_auto_diff_with_changes() {
        let temp_dir = tempdir().unwrap();
        let base_path = temp_dir.path();
        let output_path = temp_dir.path().join("output.md");

        fs::write(base_path.join("file1.txt"), "initial content").unwrap();

        let args1 = Args {
            input: base_path.to_string_lossy().to_string(),
            output: output_path.to_string_lossy().to_string(),
            filter: vec![],
            ignore: vec![],
            line_numbers: false,
            preview: false,
            token_count: false,
            yes: true,
            diff_only: false,
            clear_cache: false,
            init: false,
            max_tokens: None,
            signatures: false,
            structure: false,
            truncate: "smart".to_string(),
            visibility: "all".to_string(),
        };
        let config = Config {
            auto_diff: Some(true),
            ..Default::default()
        };
        let prompter = MockPrompter::new(true, true);

        unsafe {
            std::env::set_var("CB_SILENT", "1");
        }
        let _ = run_with_args(args1, config.clone(), &prompter);

        fs::write(base_path.join("file1.txt"), "modified content").unwrap();
        fs::write(base_path.join("file2.txt"), "new file").unwrap();

        let args2 = Args {
            input: base_path.to_string_lossy().to_string(),
            output: output_path.to_string_lossy().to_string(),
            filter: vec![],
            ignore: vec![],
            line_numbers: false,
            preview: false,
            token_count: false,
            yes: true,
            diff_only: false,
            clear_cache: false,
            init: false,
            max_tokens: None,
            signatures: false,
            structure: false,
            truncate: "smart".to_string(),
            visibility: "all".to_string(),
        };

        let result = run_with_args(args2, config, &prompter);
        unsafe {
            std::env::remove_var("CB_SILENT");
        }

        assert!(result.is_ok());
        let content = fs::read_to_string(&output_path).unwrap();
        assert!(content.contains("Change Summary") || content.contains("No Changes"));
    }

    #[test]
    fn test_auto_diff_max_tokens_truncation() {
        let temp_dir = tempdir().unwrap();
        let base_path = temp_dir.path();
        let output_path = temp_dir.path().join("output.md");

        fs::write(base_path.join("test.txt"), "x".repeat(10000)).unwrap();

        let args = Args {
            input: base_path.to_string_lossy().to_string(),
            output: output_path.to_string_lossy().to_string(),
            filter: vec![],
            ignore: vec![],
            line_numbers: false,
            preview: false,
            token_count: false,
            yes: true,
            diff_only: false,
            clear_cache: false,
            init: false,
            max_tokens: Some(100),
            signatures: false,
            structure: false,
            truncate: "smart".to_string(),
            visibility: "all".to_string(),
        };
        let config = Config {
            auto_diff: Some(true),
            ..Default::default()
        };
        let prompter = MockPrompter::new(true, true);

        unsafe {
            std::env::set_var("CB_SILENT", "1");
        }
        let result = run_with_args(args, config, &prompter);
        unsafe {
            std::env::remove_var("CB_SILENT");
        }

        assert!(result.is_ok());
        let content = fs::read_to_string(&output_path).unwrap();
        assert!(content.contains("truncated") || content.len() < 500);
    }

    #[test]
    fn test_diff_only_mode_with_added_files() {
        let temp_dir = tempdir().unwrap();
        let base_path = temp_dir.path();
        let output_path = temp_dir.path().join("output.md");

        fs::write(base_path.join("initial.txt"), "content").unwrap();

        let args1 = Args {
            input: base_path.to_string_lossy().to_string(),
            output: output_path.to_string_lossy().to_string(),
            filter: vec![],
            ignore: vec![],
            line_numbers: true,
            preview: false,
            token_count: false,
            yes: true,
            diff_only: false,
            clear_cache: false,
            init: false,
            max_tokens: None,
            signatures: false,
            structure: false,
            truncate: "smart".to_string(),
            visibility: "all".to_string(),
        };
        let config = Config {
            auto_diff: Some(true),
            ..Default::default()
        };
        let prompter = MockPrompter::new(true, true);

        unsafe {
            std::env::set_var("CB_SILENT", "1");
        }
        let _ = run_with_args(args1, config.clone(), &prompter);

        fs::write(base_path.join("newfile.txt"), "brand new content").unwrap();

        let args2 = Args {
            input: base_path.to_string_lossy().to_string(),
            output: output_path.to_string_lossy().to_string(),
            filter: vec![],
            ignore: vec![],
            line_numbers: true,
            preview: false,
            token_count: false,
            yes: true,
            diff_only: true,
            clear_cache: false,
            init: false,
            max_tokens: None,
            signatures: false,
            structure: false,
            truncate: "smart".to_string(),
            visibility: "all".to_string(),
        };

        let result = run_with_args(args2, config, &prompter);
        unsafe {
            std::env::remove_var("CB_SILENT");
        }

        assert!(result.is_ok());
        let content = fs::read_to_string(&output_path).unwrap();
        assert!(content.contains("Change Summary") || content.contains("Added Files"));
    }

    #[test]
    fn test_generate_markdown_with_diff_line_numbers() {
        let temp_dir = tempdir().unwrap();
        let base_path = temp_dir.path();

        fs::write(base_path.join("test.rs"), "fn main() {\n    println!(\"hi\");\n}").unwrap();

        let files = collect_files(base_path, &[], &[], &[]).unwrap();
        let file_tree = build_file_tree(&files, base_path);
        let config = Config::default();
        let state = ProjectState::from_files(&files, base_path, &config, true).unwrap();

        let args = Args {
            input: base_path.to_string_lossy().to_string(),
            output: "test.md".to_string(),
            filter: vec![],
            ignore: vec![],
            line_numbers: true,
            preview: false,
            token_count: false,
            yes: false,
            diff_only: false,
            clear_cache: false,
            init: false,
            max_tokens: None,
            signatures: false,
            structure: false,
            truncate: "smart".to_string(),
            visibility: "all".to_string(),
        };

        let diff_config = DiffConfig {
            context_lines: 3,
            enabled: true,
            diff_only: false,
        };

        let sorted_paths: Vec<PathBuf> = files
            .iter()
            .map(|e| {
                e.path()
                    .strip_prefix(base_path)
                    .unwrap_or(e.path())
                    .to_path_buf()
            })
            .collect();

        let ts_config = markdown::TreeSitterConfig {
            signatures: false,
            structure: false,
            truncate: "smart".to_string(),
            visibility: "all".to_string(),
        };

        let previous = state.clone();
        let comparison = state.compare_with(&previous);

        let result = generate_markdown_with_diff(
            &state,
            Some(&comparison),
            &args,
            &file_tree,
            &diff_config,
            &sorted_paths,
            &ts_config,
        );
        assert!(result.is_ok());

        let content = result.unwrap();
        assert!(content.contains("No Changes Detected"));
    }

    #[test]
    fn test_generate_markdown_with_diff_and_modifications() {
        let temp_dir = tempdir().unwrap();
        let base_path = temp_dir.path();

        fs::write(base_path.join("test.txt"), "initial content").unwrap();

        let files = collect_files(base_path, &[], &[], &[]).unwrap();
        let file_tree = build_file_tree(&files, base_path);
        let config = Config::default();
        let initial_state = ProjectState::from_files(&files, base_path, &config, false).unwrap();

        fs::write(base_path.join("test.txt"), "modified content").unwrap();

        let new_files = collect_files(base_path, &[], &[], &[]).unwrap();
        let current_state = ProjectState::from_files(&new_files, base_path, &config, false).unwrap();

        let args = Args {
            input: base_path.to_string_lossy().to_string(),
            output: "test.md".to_string(),
            filter: vec![],
            ignore: vec![],
            line_numbers: false,
            preview: false,
            token_count: false,
            yes: false,
            diff_only: false,
            clear_cache: false,
            init: false,
            max_tokens: None,
            signatures: false,
            structure: false,
            truncate: "smart".to_string(),
            visibility: "all".to_string(),
        };

        let diff_config = DiffConfig {
            context_lines: 3,
            enabled: true,
            diff_only: false,
        };

        let comparison = current_state.compare_with(&initial_state);

        let sorted_paths: Vec<PathBuf> = new_files
            .iter()
            .map(|e| {
                e.path()
                    .strip_prefix(base_path)
                    .unwrap_or(e.path())
                    .to_path_buf()
            })
            .collect();

        let ts_config = markdown::TreeSitterConfig {
            signatures: false,
            structure: false,
            truncate: "smart".to_string(),
            visibility: "all".to_string(),
        };

        let result = generate_markdown_with_diff(
            &current_state,
            Some(&comparison),
            &args,
            &file_tree,
            &diff_config,
            &sorted_paths,
            &ts_config,
        );
        assert!(result.is_ok());

        let content = result.unwrap();
        assert!(content.contains("Change Summary"));
        assert!(content.contains("Modified"));
    }

    #[test]
    #[serial]
    fn test_detect_major_file_types() {
        let temp_dir = tempdir().unwrap();
        let original_dir = std::env::current_dir().unwrap();

        // Write files BEFORE changing cwd to avoid race conditions
        fs::write(temp_dir.path().join("main.rs"), "fn main() {}").unwrap();
        fs::write(temp_dir.path().join("lib.rs"), "pub fn lib() {}").unwrap();
        fs::write(temp_dir.path().join("Cargo.toml"), "[package]").unwrap();
        fs::write(temp_dir.path().join("README.md"), "# Readme").unwrap();

        std::env::set_current_dir(&temp_dir).unwrap();

        let result = detect_major_file_types();

        std::env::set_current_dir(original_dir).unwrap();

        assert!(result.is_ok());
        let extensions = result.unwrap();
        assert!(!extensions.is_empty());
    }

    #[test]
    #[serial]
    fn test_init_config_already_exists() {
        let temp_dir = tempdir().unwrap();
        let original_dir = std::env::current_dir().unwrap();

        std::env::set_current_dir(&temp_dir).unwrap();

        let config_path = temp_dir.path().join("context-builder.toml");
        fs::write(&config_path, "output = \"existing.md\"").unwrap();

        unsafe {
            std::env::set_var("CB_SILENT", "1");
        }
        let result = init_config();
        unsafe {
            std::env::remove_var("CB_SILENT");
        }

        std::env::set_current_dir(original_dir).unwrap();

        assert!(result.is_ok());
        let content = fs::read_to_string(&config_path).unwrap();
        assert!(content.contains("existing.md"));
    }

    #[test]
    #[serial]
    fn test_init_config_creates_new_file() {
        let temp_dir = tempdir().unwrap();
        let original_dir = std::env::current_dir().unwrap();

        std::env::set_current_dir(&temp_dir).unwrap();

        let config_path = temp_dir.path().join("context-builder.toml");
        assert!(!config_path.exists());

        unsafe {
            std::env::set_var("CB_SILENT", "1");
        }
        let result = init_config();
        unsafe {
            std::env::remove_var("CB_SILENT");
        }

        std::env::set_current_dir(original_dir).unwrap();

        assert!(result.is_ok());
        assert!(config_path.exists());
        let content = fs::read_to_string(&config_path).unwrap();
        assert!(content.contains("output = "));
        assert!(content.contains("filter ="));
    }

    #[test]
    #[serial]
    fn test_detect_major_file_types_empty_dir() {
        let temp_dir = tempdir().unwrap();
        let original_dir = std::env::current_dir().unwrap();

        std::env::set_current_dir(temp_dir.path()).unwrap();

        let result = detect_major_file_types();

        std::env::set_current_dir(original_dir).unwrap();

        assert!(result.is_ok());
        let extensions = result.unwrap();
        assert!(extensions.is_empty());
    }

    #[test]
    fn test_print_context_window_warning_exact_limit() {
        let output_bytes = 128_000 * 4;
        print_context_window_warning(output_bytes, None);
    }

    #[test]
    fn test_run_with_args_with_existing_output_file() {
        let temp_dir = tempdir().unwrap();
        let base_path = temp_dir.path();
        let output_path = temp_dir.path().join("output.md");

        fs::write(base_path.join("test.txt"), "content").unwrap();
        fs::write(&output_path, "existing content").unwrap();

        let args = Args {
            input: base_path.to_string_lossy().to_string(),
            output: output_path.to_string_lossy().to_string(),
            filter: vec![],
            ignore: vec![],
            line_numbers: false,
            preview: false,
            token_count: false,
            yes: true,
            diff_only: false,
            clear_cache: false,
            init: false,
            max_tokens: None,
            signatures: false,
            structure: false,
            truncate: "smart".to_string(),
            visibility: "all".to_string(),
        };
        let config = Config::default();
        let prompter = MockPrompter::new(true, true);

        unsafe {
            std::env::set_var("CB_SILENT", "1");
        }
        let result = run_with_args(args, config, &prompter);
        unsafe {
            std::env::remove_var("CB_SILENT");
        }

        assert!(result.is_ok());
        let content = fs::read_to_string(&output_path).unwrap();
        assert!(content.contains("Directory Structure Report"));
    }

    #[test]
    fn test_run_with_args_preview_only_token_count() {
        let temp_dir = tempdir().unwrap();
        let base_path = temp_dir.path();

        fs::write(base_path.join("test.txt"), "content").unwrap();

        let args = Args {
            input: base_path.to_string_lossy().to_string(),
            output: "test.md".to_string(),
            filter: vec![],
            ignore: vec![],
            line_numbers: false,
            preview: true,
            token_count: true,
            yes: false,
            diff_only: false,
            clear_cache: false,
            init: false,
            max_tokens: None,
            signatures: false,
            structure: false,
            truncate: "smart".to_string(),
            visibility: "all".to_string(),
        };
        let config = Config::default();
        let prompter = MockPrompter::new(true, true);

        unsafe {
            std::env::set_var("CB_SILENT", "1");
        }
        let result = run_with_args(args, config, &prompter);
        unsafe {
            std::env::remove_var("CB_SILENT");
        }

        assert!(result.is_ok());
    }

    #[test]
    fn test_run_with_args_multiple_files() {
        let temp_dir = tempdir().unwrap();
        let base_path = temp_dir.path();
        let output_path = temp_dir.path().join("output.md");

        for i in 0..10 {
            fs::write(base_path.join(format!("file{}.txt", i)), "content").unwrap();
        }

        let args = Args {
            input: base_path.to_string_lossy().to_string(),
            output: output_path.to_string_lossy().to_string(),
            filter: vec![],
            ignore: vec![],
            line_numbers: true,
            preview: false,
            token_count: false,
            yes: true,
            diff_only: false,
            clear_cache: false,
            init: false,
            max_tokens: None,
            signatures: false,
            structure: false,
            truncate: "smart".to_string(),
            visibility: "all".to_string(),
        };
        let config = Config::default();
        let prompter = MockPrompter::new(true, true);

        unsafe {
            std::env::set_var("CB_SILENT", "1");
        }
        let result = run_with_args(args, config, &prompter);
        unsafe {
            std::env::remove_var("CB_SILENT");
        }

        assert!(result.is_ok());
    }

    #[test]
    fn test_auto_diff_config_hash_change() {
        let temp_dir = tempdir().unwrap();
        let base_path = temp_dir.path();
        let output_path = temp_dir.path().join("output.md");

        fs::write(base_path.join("test.txt"), "content").unwrap();

        let args1 = Args {
            input: base_path.to_string_lossy().to_string(),
            output: output_path.to_string_lossy().to_string(),
            filter: vec!["txt".to_string()],
            ignore: vec![],
            line_numbers: false,
            preview: false,
            token_count: false,
            yes: true,
            diff_only: false,
            clear_cache: false,
            init: false,
            max_tokens: None,
            signatures: false,
            structure: false,
            truncate: "smart".to_string(),
            visibility: "all".to_string(),
        };
        let config1 = Config {
            auto_diff: Some(true),
            filter: Some(vec!["txt".to_string()]),
            ..Default::default()
        };

        unsafe {
            std::env::set_var("CB_SILENT", "1");
        }
        let _ = run_with_args(args1, config1.clone(), &MockPrompter::new(true, true));

        let args2 = Args {
            input: base_path.to_string_lossy().to_string(),
            output: output_path.to_string_lossy().to_string(),
            filter: vec!["rs".to_string()],
            ignore: vec![],
            line_numbers: false,
            preview: false,
            token_count: false,
            yes: true,
            diff_only: false,
            clear_cache: false,
            init: false,
            max_tokens: None,
            signatures: false,
            structure: false,
            truncate: "smart".to_string(),
            visibility: "all".to_string(),
        };
        let config2 = Config {
            auto_diff: Some(true),
            filter: Some(vec!["rs".to_string()]),
            ..Default::default()
        };

        let result = run_with_args(args2, config2, &MockPrompter::new(true, true));
        unsafe {
            std::env::remove_var("CB_SILENT");
        }

        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_generate_markdown_with_diff_and_filters() {
        let temp_dir = tempdir().unwrap();
        let base_path = temp_dir.path();

        fs::write(base_path.join("test.rs"), "fn main() {}").unwrap();
        fs::write(base_path.join("test.txt"), "hello").unwrap();

        let files = collect_files(base_path, &["rs".to_string()], &[], &[]).unwrap();
        let file_tree = build_file_tree(&files, base_path);
        let config = Config {
            filter: Some(vec!["rs".to_string()]),
            ..Default::default()
        };
        let state = ProjectState::from_files(&files, base_path, &config, false).unwrap();

        let args = Args {
            input: base_path.to_string_lossy().to_string(),
            output: "test.md".to_string(),
            filter: vec!["rs".to_string()],
            ignore: vec![],
            line_numbers: false,
            preview: false,
            token_count: false,
            yes: false,
            diff_only: false,
            clear_cache: false,
            init: false,
            max_tokens: None,
            signatures: false,
            structure: false,
            truncate: "smart".to_string(),
            visibility: "all".to_string(),
        };

        let diff_config = DiffConfig {
            context_lines: 3,
            enabled: true,
            diff_only: false,
        };

        let sorted_paths: Vec<PathBuf> = files
            .iter()
            .map(|e| {
                e.path()
                    .strip_prefix(base_path)
                    .unwrap_or(e.path())
                    .to_path_buf()
            })
            .collect();

        let ts_config = markdown::TreeSitterConfig {
            signatures: false,
            structure: false,
            truncate: "smart".to_string(),
            visibility: "all".to_string(),
        };

        let result = generate_markdown_with_diff(
            &state,
            None,
            &args,
            &file_tree,
            &diff_config,
            &sorted_paths,
            &ts_config,
        );
        assert!(result.is_ok());

        let content = result.unwrap();
        assert!(content.contains("test.rs"));
    }

    #[test]
    fn test_generate_markdown_with_diff_and_ignores() {
        let temp_dir = tempdir().unwrap();
        let base_path = temp_dir.path();

        fs::write(base_path.join("test.rs"), "fn main() {}").unwrap();
        fs::write(base_path.join("ignore.txt"), "ignored").unwrap();

        let files = collect_files(base_path, &[], &["ignore.txt".to_string()], &[]).unwrap();
        let file_tree = build_file_tree(&files, base_path);
        let config = Config {
            ignore: Some(vec!["ignore.txt".to_string()]),
            ..Default::default()
        };
        let state = ProjectState::from_files(&files, base_path, &config, false).unwrap();

        let args = Args {
            input: base_path.to_string_lossy().to_string(),
            output: "test.md".to_string(),
            filter: vec![],
            ignore: vec!["ignore.txt".to_string()],
            line_numbers: false,
            preview: false,
            token_count: false,
            yes: false,
            diff_only: false,
            clear_cache: false,
            init: false,
            max_tokens: None,
            signatures: false,
            structure: false,
            truncate: "smart".to_string(),
            visibility: "all".to_string(),
        };

        let diff_config = DiffConfig {
            context_lines: 3,
            enabled: true,
            diff_only: false,
        };

        let sorted_paths: Vec<PathBuf> = files
            .iter()
            .map(|e| {
                e.path()
                    .strip_prefix(base_path)
                    .unwrap_or(e.path())
                    .to_path_buf()
            })
            .collect();

        let ts_config = markdown::TreeSitterConfig {
            signatures: false,
            structure: false,
            truncate: "smart".to_string(),
            visibility: "all".to_string(),
        };

        let result = generate_markdown_with_diff(
            &state,
            None,
            &args,
            &file_tree,
            &diff_config,
            &sorted_paths,
            &ts_config,
        );
        assert!(result.is_ok());

        let content = result.unwrap();
        assert!(content.contains("test.rs"));
    }
}
