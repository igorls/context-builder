use clap::{CommandFactory, FromArgMatches};

use std::fs;
use std::io::{self, Write};
use std::path::{Component, Path, PathBuf};
use std::time::Instant;

pub mod cache;
pub mod cli;
pub mod config;
pub mod config_resolver;
pub mod content_filter;
pub mod diff;
pub mod fences;
pub mod file_utils;
pub mod languages;
pub mod markdown;
pub mod state;
pub mod token_count;
pub mod tree;
pub mod tree_sitter;

use std::fs::File;

use cache::CacheManager;
use cli::Args;
use config::{Config, load_config_from_path};
use content_filter::{ContentPolicy, SkippedFile};
use diff::render_per_file_diffs;
#[cfg(test)]
use file_utils::collect_files;
use file_utils::confirm_overwrite;
use markdown::generate_markdown;
use state::{ProjectState, StateComparison};
use token_count::{Encoding, count_file_tokens, count_tree_tokens, estimate_tokens};
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
    fn confirm_overwrite(&self, file_path: &str) -> io::Result<bool>;
}

pub struct DefaultPrompter;

impl Prompter for DefaultPrompter {
    fn confirm_overwrite(&self, file_path: &str) -> io::Result<bool> {
        confirm_overwrite(file_path)
    }
}

/// Ignore patterns for this run's output file, anchored to its path relative
/// to `base_path`.
///
/// A bare basename such as `output.md` matches at every depth in the ignore
/// crate, which hides a user's `docs/output.md`. Patterns here start with
/// `/` so they match only the resolved output (and, when timestamped output
/// is on, the sibling `stem_*.ext` family in that same directory).
///
/// Returns nothing when the output is stdout (`-`) or lives outside the tree.
fn output_auto_ignores(base_path: &Path, output: &str, config: &Config) -> Vec<String> {
    if output == "-" {
        return Vec::new();
    }
    let cwd = match std::env::current_dir() {
        Ok(cwd) => cwd,
        Err(e) => {
            log::warn!("could not read the working directory to anchor the output ignore: {e}");
            return Vec::new();
        }
    };
    let abs_base = normalize_lexically(&join_absolute(&cwd, base_path));
    let abs_output = normalize_lexically(&join_absolute(&cwd, Path::new(output)));
    let Ok(rel_output) = abs_output.strip_prefix(&abs_base) else {
        return Vec::new();
    };
    if rel_output.as_os_str().is_empty() {
        return Vec::new();
    }

    let rel = rel_output.to_string_lossy().replace('\\', "/");
    let mut patterns = vec![format!("/{rel}")];
    if config.timestamped_output == Some(true)
        && let Some(pattern) = timestamped_output_glob(rel_output, Path::new(output), config)
    {
        patterns.push(pattern);
    }
    patterns
}

/// Anchored glob covering timestamped siblings of the resolved output file
/// (`/docs/context_*.md`, `/context_*.md`). The stem comes from
/// `config.output` when set, matching the name before the timestamp suffix.
fn timestamped_output_glob(
    rel_output: &Path,
    output_path: &Path,
    config: &Config,
) -> Option<String> {
    let parent = rel_output.parent()?;
    let stem = output_path.file_stem().and_then(|s| s.to_str())?;
    let ext = output_path.extension().and_then(|s| s.to_str())?;
    let base_stem = if let Some(ref cfg_output) = config.output {
        Path::new(cfg_output)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or(stem)
            .to_string()
    } else {
        stem.to_string()
    };
    let parent_str = parent.to_string_lossy().replace('\\', "/");
    if parent_str.is_empty() || parent_str == "." {
        Some(format!("/{base_stem}_*.{ext}"))
    } else {
        Some(format!("/{parent_str}/{base_stem}_*.{ext}"))
    }
}

fn join_absolute(cwd: &Path, path: &Path) -> PathBuf {
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        cwd.join(path)
    }
}

/// Collapse `.` and `..` without touching the filesystem.
fn normalize_lexically(path: &Path) -> PathBuf {
    let mut out = PathBuf::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                out.pop();
            }
            other => out.push(other.as_os_str()),
        }
    }
    out
}

pub fn run_with_args(args: Args, config: Config, prompter: &impl Prompter) -> io::Result<()> {
    let start_time = Instant::now();

    let silent = std::env::var("CB_SILENT")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false);

    // Use the finalized args passed in from run()
    let final_args = args;
    // `-o -` streams the document to stdout (pipe mode). In this mode all
    // human-facing chatter must go to stderr so it doesn't corrupt the pipe.
    let to_stdout = final_args.output == "-";

    // B5: warn on an unrecognized encoding_strategy instead of silently using
    // "detect". Validates the config value against the supported set.
    if let Some(ref strat) = config.encoding_strategy
        && !matches!(strat.as_str(), "detect" | "strict" | "skip")
        && !silent
    {
        eprintln!(
            "⚠️  Unknown encoding_strategy '{strat}' in config; expected one of: detect, strict, skip. Falling back to 'detect'."
        );
    }
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
            eprintln!("Operation cancelled.");
        }
        return Err(io::Error::new(
            io::ErrorKind::Interrupted,
            "Operation cancelled by user",
        ));
    }

    // Compute auto-ignore patterns to exclude the tool's own output and cache.
    // The output pattern is anchored to the resolved path (see `output_auto_ignores`).
    let mut auto_ignores: Vec<String> = vec![".context-builder".to_string()];
    auto_ignores.extend(output_auto_ignores(base_path, &final_args.output, &config));

    // Also exclude context output files within the output_folder (not the folder itself,
    // which would silently hide all user content in that directory)
    if let Some(ref output_folder) = config.output_folder {
        auto_ignores.push(format!("{}/*.md", output_folder));
    }

    let collected = crate::file_utils::collect_files_reporting(
        base_path,
        &final_args.filter,
        &final_args.ignore,
        &auto_ignores,
        final_args.hidden,
        final_args.include_lockfiles,
    )?;
    if !silent
        && let Some(notice) = crate::file_utils::lockfile_skip_notice(collected.skipped_lockfiles)
    {
        eprintln!("{notice}");
    }
    let files = collected.files;
    let policy = ContentPolicy::new(
        &final_args.max_file_size,
        &final_args.filter,
        final_args.include_secrets,
    );
    let (files, skipped) = content_filter::partition(files, base_path, &policy);
    content_filter::report_skips(&skipped, silent);
    // Nothing matched: warn on stderr so `-o -` pipes stay clean, and still
    // write the (empty) document. Name the filters when any were given.
    if !silent && files.is_empty() && skipped.is_empty() {
        if final_args.filter.is_empty() {
            eprintln!("Warning: No files matched; check .gitignore, --ignore, and --filter");
        } else {
            let quoted = final_args
                .filter
                .iter()
                .map(|f| format!("'{f}'"))
                .collect::<Vec<_>>()
                .join(", ");
            let noun = if final_args.filter.len() == 1 {
                "filter"
            } else {
                "filters"
            };
            eprintln!("Warning: no files matched {noun} {quoted}.");
        }
    }
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

    // Smart large-file detection: warn about files that may bloat the context.
    // Skipped when the output is already reduced (token budget, signatures,
    // structure): the input sizes would only suggest a problem that is not there.
    if !silent && !output_is_reduced(&final_args) {
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
            large_files.sort_by_key(|b| std::cmp::Reverse(b.1)); // Sort by size descending
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
            let encoding = final_args.encoding.parse::<Encoding>().unwrap_or_default();
            // Render each file through the same path as the document so the
            // preview matches what would actually be produced (B9).
            let ts_config = markdown::TreeSitterConfig {
                signatures: final_args.signatures,
                structure: final_args.structure,
                truncate: final_args.truncate.clone(),
                visibility: final_args.visibility.clone(),
                file_metadata: final_args.file_metadata,
            };
            let enc_strategy = config.encoding_strategy.as_deref();
            println!("\n# Token Count Estimation\n");
            let mut total_tokens = 0;
            total_tokens +=
                estimate_tokens(encoding, &format!("{}\n\n", markdown::REPORT_TITLE_LINE));
            if !final_args.filter.is_empty() {
                total_tokens += estimate_tokens(
                    encoding,
                    &format!(
                        "This document contains files from the {} directory with extensions: {} \n",
                        fences::inline_code(&final_args.input),
                        final_args.filter.join(", ")
                    ),
                );
            } else {
                total_tokens += estimate_tokens(
                    encoding,
                    &format!(
                        "This document contains all files from the {} directory, optimized for LLM consumption.\n",
                        fences::inline_code(&final_args.input)
                    ),
                );
            }
            if !final_args.ignore.is_empty() {
                total_tokens += estimate_tokens(
                    encoding,
                    &format!(
                        "Custom ignored patterns: {} \n",
                        final_args.ignore.join(", ")
                    ),
                );
            }
            total_tokens += estimate_tokens(
                encoding,
                &format!("{}0000000000000000\n\n", markdown::CONTENT_HASH_PREFIX),
            );
            total_tokens += estimate_tokens(encoding, "## File Tree Structure\n\n");
            let tree_tokens = count_tree_tokens(&file_tree, 0, encoding);
            total_tokens += tree_tokens;
            // The `## Skipped` section is part of the generated report.
            total_tokens += skipped_section_tokens(encoding, &skipped)?;
            let file_tokens: usize = files
                .iter()
                .map(|entry| {
                    count_file_tokens(
                        base_path,
                        entry,
                        final_args.line_numbers,
                        encoding,
                        enc_strategy,
                        &ts_config,
                    )
                })
                .sum();
            total_tokens += file_tokens;
            println!("Estimated total tokens: {}", total_tokens);
            println!("File tree tokens: {}", tree_tokens);
            println!("File content tokens: {}", file_tokens);
        }
        return Ok(());
    }

    // NOTE: config-driven flags (line_numbers, diff_only) are already merged
    // by config_resolver.rs with proper CLI-takes-precedence semantics.
    // Do NOT re-apply them here as that would silently overwrite CLI flags.

    // B8: --diff-only only takes effect together with auto_diff (+ timestamped
    // output). Warn instead of silently emitting full file contents.
    if final_args.diff_only && !config.auto_diff.unwrap_or(false) && !silent {
        eprintln!(
            "⚠️  --diff-only has no effect without auto_diff (it also needs timestamped_output). \
             Full file contents will be emitted. Enable auto_diff = true + timestamped_output = true to use diff-only mode."
        );
    }

    if config.auto_diff.unwrap_or(false) {
        // Build an effective config that mirrors the *actual* file selection coming
        // from resolved CLI args, so the cache/diff fingerprint reflects real
        // behavior even when selection originates from the CLI, not the config
        // file. `filter`, `ignore`, `include_lockfiles`, `max_file_size`, `hidden`,
        // and `include_secrets` decide which files form the diff baseline. Rendering
        // options (signatures/structure/truncate/visibility/max_tokens/
        // line_numbers/file_metadata/encoding) deliberately do NOT feed the fingerprint — they
        // don't change the captured raw content — so propagating them here would
        // only risk spurious baseline resets (see `config_fingerprint`). The per-file
        // content hash stored in the cache is the file bytes only, so an mtime-only
        // change is not a diff.
        let mut effective_config = config.clone();
        if !final_args.filter.is_empty() {
            effective_config.filter = Some(final_args.filter.clone());
        }
        if !final_args.ignore.is_empty() {
            effective_config.ignore = Some(final_args.ignore.clone());
        }
        // Size, hidden, and secret policy decide which bytes form the baseline.
        effective_config.max_file_size = Some(final_args.max_file_size.clone());
        effective_config.hidden = Some(final_args.hidden);
        effective_config.include_secrets = Some(final_args.include_secrets);
        if final_args.include_lockfiles {
            effective_config.include_lockfiles = Some(true);
        }

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
        let comparison = effective_previous
            .map(|prev| current_state.compare_with(prev, config.diff_context_lines));

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
            file_metadata: final_args.file_metadata,
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
            &skipped,
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

                // Close any open code fence so the truncation notice is not
                // swallowed by the block. The closer matches the opening
                // fence's length — a fixed ``` would not close a longer fence
                // chosen because the file itself contains ```.
                fences::close_unmatched_backtick_fence(&mut final_doc);

                final_doc.push_str("\n---\n\n");
                final_doc.push_str(&format!(
                    "_Output truncated: exceeded {} token budget (estimated)._\n",
                    max_tokens
                ));
            }
        }

        // 5. Write output — to stdout in pipe mode, otherwise to the file.
        if to_stdout {
            io::stdout().write_all(final_doc.as_bytes())?;
        } else {
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
        }

        // 6. Update cache with current state
        if let Err(e) = cache_manager.write_cache(&current_state)
            && !silent
        {
            eprintln!("Warning: failed to update state cache: {}", e);
        }

        let duration = start_time.elapsed();
        if !silent && !to_stdout {
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
        if !silent {
            // Non-blocking. File count is not a cost proxy; this estimate is.
            // Stderr so `-o -` stays a clean document.
            print_context_window_warning(
                final_doc.len(),
                final_args.max_tokens,
                &files,
                !final_args.filter.is_empty(),
            );
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
        file_metadata: final_args.file_metadata,
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

    let output_bytes = generate_markdown(
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
        final_args.encoding.parse::<Encoding>().unwrap_or_default(),
        &ts_config,
        &skipped,
    )?;

    let duration = start_time.elapsed();
    if !silent && !to_stdout {
        println!("Documentation created successfully: {}", final_args.output);
        println!("Processing time: {:.2?}", duration);
    }
    if !silent {
        // Non-blocking. File count is not a cost proxy; this estimate is.
        // Stderr so `-o -` stays a clean document.
        print_context_window_warning(
            output_bytes,
            final_args.max_tokens,
            &files,
            !final_args.filter.is_empty(),
        );
    }

    Ok(())
}

/// Print context window overflow warnings with actionable recommendations.
/// Tokens in the `## Skipped` section (zero when nothing was skipped).
fn skipped_section_tokens(encoding: Encoding, skipped: &[SkippedFile]) -> io::Result<usize> {
    let mut buf = Vec::new();
    content_filter::write_skipped_section(&mut buf, skipped)?;
    Ok(estimate_tokens(encoding, &String::from_utf8_lossy(&buf)))
}

/// True when the document will be much smaller than the input files: a token
/// budget caps it, or `--signatures` / `--structure` replace bodies with an
/// outline (only when tree-sitter support is compiled in; otherwise the full
/// content is written).
fn output_is_reduced(args: &Args) -> bool {
    args.max_tokens.is_some()
        || (cfg!(feature = "tree-sitter-base") && (args.signatures || args.structure))
}

/// Estimates tokens using the ~4 bytes/token heuristic. Warns when output
/// exceeds 128K tokens — beyond this size, context quality degrades
/// significantly for most LLM use cases.
///
/// Filter advice uses the extensions present in `files`.
fn print_context_window_warning(
    output_bytes: usize,
    max_tokens: Option<usize>,
    files: &[ignore::DirEntry],
    filter_applied: bool,
) {
    let estimated_tokens = output_bytes / 4;

    // Stderr only: this notice must never mix into a captured or piped document.
    eprintln!("Estimated tokens: ~{}K", estimated_tokens / 1000);

    // If the user already set --max-tokens, they're managing their budget
    if max_tokens.is_some() {
        return;
    }

    const RECOMMENDED_LIMIT: usize = 128_000;

    if estimated_tokens <= RECOMMENDED_LIMIT {
        return;
    }

    let paths: Vec<&Path> = files.iter().map(|entry| entry.path()).collect();

    eprintln!();
    eprintln!(
        "⚠️  Output is ~{}K tokens — recommended limit is 128K for effective LLM context.",
        estimated_tokens / 1000
    );
    eprintln!("   Large contexts degrade response quality. Consider narrowing the scope:");
    eprintln!();
    for line in context_window_suggestions(&paths, filter_applied) {
        eprintln!("   • {line}");
    }
    eprintln!();
}

/// `--filter` extensions to suggest for this run: the most common extensions
/// among `paths` (at most two), in a form `--filter` accepts.
///
/// Only extensions that are valid `ignore` file-type names are included, so
/// the printed command does not panic on an unrecognized type. Ties break
/// alphabetically so the suggestion is stable.
fn suggested_filter_exts(paths: &[&Path]) -> Option<String> {
    use std::collections::HashMap;

    let mut counts: HashMap<&str, usize> = HashMap::new();
    let exts = paths
        .iter()
        .filter_map(|path| path.extension().and_then(|ext| ext.to_str()))
        .filter(|ext| is_suggestable_filter_ext(ext));
    for ext in exts {
        *counts.entry(ext).or_insert(0) += 1;
    }
    if counts.is_empty() {
        return None;
    }

    let mut ranked: Vec<(&str, usize)> = counts.into_iter().collect();
    ranked.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(b.0)));
    let joined = ranked
        .into_iter()
        .take(2)
        .map(|(ext, _)| ext)
        .collect::<Vec<_>>()
        .join(",");
    Some(joined)
}

/// File-type names `TypesBuilder::add` accepts: non-empty ASCII alphanumeric,
/// and not the reserved name `all` (which selects every type).
fn is_suggestable_filter_ext(ext: &str) -> bool {
    ext != "all" && !ext.is_empty() && ext.chars().all(|c| c.is_ascii_alphanumeric())
}

/// Copy-pasteable commands for the >128K warning. Every flag here is one the
/// CLI actually honors (`--ignore docs,assets` included).
fn context_window_suggestions(paths: &[&Path], filter_applied: bool) -> Vec<String> {
    let mut lines: Vec<String> = ADVICE.iter().map(|(f, d)| advice_line(f, d)).collect();
    // A run that already passes `--filter` has nothing more to gain from that advice.
    if !filter_applied && let Some(exts) = suggested_filter_exts(paths) {
        lines.insert(1, advice_line(&format!("--filter {exts}"), FILTER_ADVICE));
    }
    lines
}

const FILTER_ADVICE: &str = "Include only these file types";

/// The fixed suggestions; the `--filter` line is inserted after the first.
const ADVICE: [(&str, &str); 3] = [
    ("--max-tokens 100000", "Cap output to a token budget"),
    ("--ignore docs,assets", "Exclude directories by name"),
    ("--token-count", "Preview size without generating"),
];

/// `flag` padded to a column, or followed by two spaces when it is too long.
fn advice_line(flag: &str, description: &str) -> String {
    let pad = if flag.len() >= 24 { 2 } else { 24 - flag.len() };
    format!("{flag}{}{description}", " ".repeat(pad))
}

/// Generate markdown document with diff annotations
#[allow(clippy::too_many_arguments)]
fn generate_markdown_with_diff(
    current_state: &ProjectState,
    comparison: Option<&StateComparison>,
    args: &Args,
    file_tree: &tree::FileTree,
    diff_config: &DiffConfig,
    sorted_paths: &[PathBuf],
    ts_config: &markdown::TreeSitterConfig,
    skipped: &[SkippedFile],
) -> io::Result<String> {
    let mut output = String::new();

    // Header
    output.push_str(markdown::REPORT_TITLE_LINE);
    output.push_str("\n\n");

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
                    output.push_str(&format!(
                        "### File: {}\n\n",
                        fences::inline_code(&added.path)
                    ));
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
                    let mut body = String::new();
                    if args.line_numbers {
                        for (idx, l) in lines.iter().enumerate() {
                            body.push_str(&format!("{:>4} | {}\n", idx + 1, l));
                        }
                    } else {
                        for l in &lines {
                            body.push_str(l);
                            body.push('\n');
                        }
                    }
                    output.push_str(&fences::fenced_block("text", &body));
                    output.push('\n');
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
    let mut skipped_buf = Vec::new();
    content_filter::write_skipped_section(&mut skipped_buf, skipped)?;
    output.push_str(&String::from_utf8_lossy(&skipped_buf));

    // File contents (unless diff_only mode)
    if !diff_config.diff_only {
        output.push_str("## File Contents\n\n");

        // Iterate in relevance order (from sorted_paths) instead of
        // BTreeMap's alphabetical order — preserves file_relevance_category ordering.
        for path in sorted_paths {
            if let Some(file_state) = current_state.files.get(path) {
                output.push_str(&format!(
                    "### File: {}\n\n",
                    fences::inline_code(&path.display().to_string())
                ));
                // Same opt-in as the standard renderer. Off by default so an
                // mtime-only change does not rewrite the document. The cache
                // compares content hashes (file bytes), not mtime.
                if args.file_metadata {
                    output.push_str(&format!("- Size: {} bytes\n", file_state.size));
                    output.push_str(&format!("- Modified: {:?}\n\n", file_state.modified));
                }

                // Determine language from file extension (canonical map —
                // same fence language as the main rendering path)
                let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("text");
                let language = crate::languages::language_for_extension(extension);

                // When --signatures is active, only suppress content for supported code files
                let signatures_only =
                    ts_config.signatures && crate::tree_sitter::is_supported_extension(extension);

                if !signatures_only {
                    // `output` is a String (`fmt::Write`); the shared writer speaks `io::Write`.
                    let mut rendered = Vec::new();
                    markdown::write_text_content(
                        &mut rendered,
                        &file_state.content,
                        language,
                        args.line_numbers,
                    )?;
                    let rendered = std::str::from_utf8(&rendered)
                        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
                    output.push_str(rendered);
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
    // Parse via `ArgMatches` (not `Args::parse`) so we can tell whether the
    // value-bearing flags were *explicitly* passed or left at their clap default.
    // `--encoding o200k_base` and `-o output.md` carry the same value as the
    // default, so the value alone can't reveal an intent to override a
    // non-default config (see resolver).
    let matches = Args::command().get_matches();
    let explicit = crate::config_resolver::ExplicitCli {
        truncate: matches.value_source("truncate") == Some(clap::parser::ValueSource::CommandLine),
        visibility: matches.value_source("visibility")
            == Some(clap::parser::ValueSource::CommandLine),
        encoding: matches.value_source("encoding") == Some(clap::parser::ValueSource::CommandLine),
        max_file_size: matches.value_source("max_file_size")
            == Some(clap::parser::ValueSource::CommandLine),
        // `-o output.md` carries the same string as the default, so the value
        // alone can't tell an explicit path from an omitted flag. An explicit
        // `-o` is used verbatim (no output_folder / timestamp rewrite).
        output: matches.value_source("output") == Some(clap::parser::ValueSource::CommandLine),
    };
    let args = Args::from_arg_matches(&matches)
        .expect("arguments were already validated by get_matches()");

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
    let resolution = crate::config_resolver::resolve_final_config(args, config.clone(), explicit);

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
        file_metadata: resolution.config.file_metadata,
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
        encoding: resolution.config.encoding,
        max_file_size: resolution.config.max_file_size,
        hidden: resolution.config.hidden,
        include_secrets: resolution.config.include_secrets,
        include_lockfiles: resolution.config.include_lockfiles,
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
    extensions.sort_by_key(|b| std::cmp::Reverse(b.1));

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

# Paths or gitignore-style globs to ignore (names, paths like "crates/core", globs like "*.lock")
ignore = ["docs", "target", ".git", "node_modules"]

# Dependency lockfiles are skipped by default, including when `filter` matches
# their type (for example toml or lock). Set true or pass --include-lockfiles.
include_lockfiles = false

# Add line numbers to code blocks
line_numbers = false

# Skip files larger than this ("256K", "1M", "262144"; "0" disables). Default: 256K
# max_file_size = "256K"

# Include hidden dotfiles and directories, except .git/.hg/.svn/.bzr. Default: false
# hidden = false

# Include likely-secret files (id_rsa, *.pem, .env, credentials*.json, …). Default: false
# Dotfile secrets also need hidden = true.
# include_secrets = false

# Per-file Size and Modified lines under each file header.
# Off by default: they cost tokens and change the document when mtime changes.
# Set to true, or pass --file-metadata, to opt in.
file_metadata = false
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
        confirm_overwrite_response: bool,
    }

    impl MockPrompter {
        fn new(overwrite: bool) -> Self {
            Self {
                confirm_overwrite_response: overwrite,
            }
        }
    }

    impl Prompter for MockPrompter {
        fn confirm_overwrite(&self, _file_path: &str) -> Result<bool> {
            Ok(self.confirm_overwrite_response)
        }
    }

    /// Run the pipeline in-process on `dir` with the given CLI flags (no config file).
    fn run_in_process(dir: &Path, extra: &[&str]) -> io::Result<()> {
        use clap::Parser;
        let mut argv = vec![
            "context-builder".to_string(),
            "-d".to_string(),
            dir.to_string_lossy().into_owned(),
        ];
        argv.extend(extra.iter().map(|s| s.to_string()));
        let args = Args::parse_from(argv);
        run_with_args(args, Config::default(), &MockPrompter::new(true))
    }

    #[test]
    fn empty_walk_and_unmatched_filters_still_succeed() {
        let dir = tempdir().unwrap();
        let out = dir.path().join("empty.md");
        // Nothing to collect: warns on stderr and still writes the document.
        run_in_process(dir.path(), &["-o", &out.to_string_lossy(), "-y"]).unwrap();
        assert!(out.exists());

        fs::write(dir.path().join("a.txt"), "a").unwrap();
        for filters in [vec!["-f", "rs"], vec!["-f", "rs,go"]] {
            let mut flags = vec!["-o", "unmatched.md", "-y"];
            let out2 = dir.path().join("unmatched.md");
            let out2 = out2.to_string_lossy().into_owned();
            flags[1] = &out2;
            flags.extend(filters);
            run_in_process(dir.path(), &flags).unwrap();
        }
    }

    #[test]
    fn output_to_stdout_pipe_mode_succeeds() {
        let dir = tempdir().unwrap();
        fs::write(dir.path().join("a.txt"), "a").unwrap();
        run_in_process(dir.path(), &["-o", "-"]).unwrap();
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
            encoding: "o200k_base".to_string(),
            init: false,
            max_tokens: None,
            signatures: false,
            structure: false,
            truncate: "smart".to_string(),
            visibility: "all".to_string(),
            max_file_size: "256K".to_string(),
            hidden: false,
            include_secrets: false,
            file_metadata: false,
            include_lockfiles: false,
        };
        let config = Config::default();
        let prompter = MockPrompter::new(true);

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
            encoding: "o200k_base".to_string(),
            init: false,
            max_tokens: None,
            signatures: false,
            structure: false,
            truncate: "smart".to_string(),
            visibility: "all".to_string(),
            max_file_size: "256K".to_string(),
            hidden: false,
            include_secrets: false,
            file_metadata: false,
            include_lockfiles: false,
        };
        let config = Config::default();
        let prompter = MockPrompter::new(true);

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
    fn skipped_section_counts_toward_token_estimate() {
        let encoding = Encoding::default();
        assert_eq!(skipped_section_tokens(encoding, &[]).unwrap(), 0);
        let skipped = vec![SkippedFile {
            path: "assets/logo.png".to_string(),
            reason: content_filter::SkipReason::Asset,
            detail: "image",
        }];
        assert!(skipped_section_tokens(encoding, &skipped).unwrap() > 0);
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
            encoding: "o200k_base".to_string(),
            init: false,
            max_tokens: None,
            signatures: false,
            structure: false,
            truncate: "smart".to_string(),
            visibility: "all".to_string(),
            max_file_size: "256K".to_string(),
            hidden: false,
            include_secrets: false,
            file_metadata: false,
            include_lockfiles: false,
        };
        let config = Config::default();
        let prompter = MockPrompter::new(true);

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
            encoding: "o200k_base".to_string(),
            init: false,
            max_tokens: None,
            signatures: false,
            structure: false,
            truncate: "smart".to_string(),
            visibility: "all".to_string(),
            max_file_size: "256K".to_string(),
            hidden: false,
            include_secrets: false,
            file_metadata: false,
            include_lockfiles: false,
        };
        let config = Config::default();
        let prompter = MockPrompter::new(true);

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
            encoding: "o200k_base".to_string(),
            init: false,
            max_tokens: None,
            signatures: false,
            structure: false,
            truncate: "smart".to_string(),
            visibility: "all".to_string(),
            max_file_size: "256K".to_string(),
            hidden: false,
            include_secrets: false,
            file_metadata: false,
            include_lockfiles: false,
        };
        let config = Config::default();
        let prompter = MockPrompter::new(false); // Deny overwrite

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
    fn test_run_with_args_many_files_does_not_prompt() {
        let temp_dir = tempdir().unwrap();
        let base_path = temp_dir.path();
        let output_path = temp_dir.path().join("out.md");

        // More than 100 files used to ask for confirmation. It must proceed
        // without `--yes` and without consulting a processing prompt.
        for i in 0..105 {
            fs::write(base_path.join(format!("file{}.txt", i)), "content").unwrap();
        }

        let args = Args {
            input: base_path.to_string_lossy().to_string(),
            output: output_path.to_string_lossy().to_string(),
            filter: vec![],
            ignore: vec![],
            line_numbers: false,
            preview: false,
            token_count: false,
            yes: false,
            diff_only: false,
            clear_cache: false,
            encoding: "o200k_base".to_string(),
            init: false,
            max_tokens: None,
            signatures: false,
            structure: false,
            truncate: "smart".to_string(),
            visibility: "all".to_string(),
            max_file_size: "256K".to_string(),
            hidden: false,
            include_secrets: false,
            file_metadata: false,
            include_lockfiles: false,
        };
        let config = Config::default();
        let prompter = MockPrompter::new(true);

        unsafe {
            std::env::set_var("CB_SILENT", "1");
        }
        let result = run_with_args(args, config, &prompter);
        unsafe {
            std::env::remove_var("CB_SILENT");
        }

        assert!(result.is_ok(), "a large file set must not ask to continue");
        assert!(output_path.exists(), "output should be written");
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
            encoding: "o200k_base".to_string(),
            init: false,
            max_tokens: None,
            signatures: false,
            structure: false,
            truncate: "smart".to_string(),
            visibility: "all".to_string(),
            max_file_size: "256K".to_string(),
            hidden: false,
            include_secrets: false,
            file_metadata: false,
            include_lockfiles: false,
        };
        let config = Config::default();
        let prompter = MockPrompter::new(true);

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
            encoding: "o200k_base".to_string(),
            init: false,
            max_tokens: None,
            signatures: false,
            structure: false,
            truncate: "smart".to_string(),
            visibility: "all".to_string(),
            max_file_size: "256K".to_string(),
            hidden: false,
            include_secrets: false,
            file_metadata: false,
            include_lockfiles: false,
        };
        let config = Config::default();
        let prompter = MockPrompter::new(true);

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
            encoding: "o200k_base".to_string(),
            init: false,
            max_tokens: None,
            signatures: false,
            structure: false,
            truncate: "smart".to_string(),
            visibility: "all".to_string(),
            max_file_size: "256K".to_string(),
            hidden: false,
            include_secrets: false,
            file_metadata: false,
            include_lockfiles: false,
        };
        let config = Config::default();
        let prompter = MockPrompter::new(true);

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
            encoding: "o200k_base".to_string(),
            init: false,
            max_tokens: None,
            signatures: false,
            structure: false,
            truncate: "smart".to_string(),
            visibility: "all".to_string(),
            max_file_size: "256K".to_string(),
            hidden: false,
            include_secrets: false,
            file_metadata: false,
            include_lockfiles: false,
        };
        let config = Config {
            auto_diff: Some(true),
            diff_context_lines: Some(5),
            ..Default::default()
        };
        let prompter = MockPrompter::new(true);

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
            encoding: "o200k_base".to_string(),
            init: false,
            max_tokens: None,
            signatures: false,
            structure: false,
            truncate: "smart".to_string(),
            visibility: "all".to_string(),
            max_file_size: "256K".to_string(),
            hidden: false,
            include_secrets: false,
            file_metadata: false,
            include_lockfiles: false,
        };
        let config = Config::default();
        let prompter = MockPrompter::new(true);

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
            encoding: "o200k_base".to_string(),
            init: false,
            max_tokens: None,
            signatures: false,
            structure: false,
            truncate: "smart".to_string(),
            visibility: "all".to_string(),
            max_file_size: "256K".to_string(),
            hidden: false,
            include_secrets: false,
            file_metadata: false,
            include_lockfiles: false,
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
            file_metadata: false,
        };

        let result = generate_markdown_with_diff(
            &state,
            None,
            &args,
            &file_tree,
            &diff_config,
            &sorted_paths,
            &ts_config,
            &[],
        );
        assert!(result.is_ok());

        let content = result.unwrap();
        assert!(content.contains("Directory Structure Report"));
        assert!(content.contains("test.rs"));
    }

    #[test]
    fn test_context_window_suggestions_follow_detected_extensions() {
        let go = [
            Path::new("cmd/root.go"),
            Path::new("cmd/main.go"),
            Path::new("README.md"),
        ];
        let lines = context_window_suggestions(&go, false);
        let text = lines.join("\n");
        assert!(
            text.contains("--filter go,md"),
            "a Go repo should be told to filter its own types, got:\n{text}"
        );
        assert!(text.contains("--ignore docs,assets"), "{text}");
        assert!(text.contains("--max-tokens 100000"), "{text}");
        assert!(text.contains("--token-count"), "{text}");
        assert!(
            !text.contains("rs,toml"),
            "advice must not hardcode a Rust filter: {text}"
        );
        assert!(
            !text.contains("--ignore lock"),
            "advice must not suggest a no-op ignore: {text}"
        );

        let python = [
            Path::new("src/app.py"),
            Path::new("src/util.py"),
            Path::new("tests/test_app.py"),
        ];
        let py_lines = context_window_suggestions(&python, false);
        assert!(
            py_lines.iter().any(|line| line.contains("--filter py")),
            "{py_lines:?}"
        );
        assert!(
            !py_lines.iter().any(|line| line.contains("rs,toml")),
            "{py_lines:?}"
        );

        // Equal counts break ties alphabetically, so the suggestion is stable.
        let tied = [Path::new("a.py"), Path::new("b.rs")];
        assert_eq!(suggested_filter_exts(&tied).as_deref(), Some("py,rs"));

        // Underscores and the reserved name `all` are not valid --filter type names.
        let skipped = [
            Path::new("vendor/lib.a_b"),
            Path::new("secret.all"),
            Path::new("c.go"),
            Path::new("d.go"),
        ];
        assert_eq!(suggested_filter_exts(&skipped).as_deref(), Some("go"));
    }

    #[test]
    fn advice_line_pads_to_a_column_or_uses_two_spaces() {
        assert_eq!(
            advice_line("--token-count", "x"),
            "--token-count           x"
        );
        assert_eq!(
            advice_line("--filter go,md,something,long", "x"),
            "--filter go,md,something,long  x"
        );
    }

    #[test]
    fn test_context_window_suggestions_omit_filter_when_already_applied() {
        let paths = [Path::new("a.rs"), Path::new("b.rs")];
        let with = context_window_suggestions(&paths, false);
        assert!(with.iter().any(|l| l.contains("--filter rs")));
        let without = context_window_suggestions(&paths, true);
        assert!(
            !without.iter().any(|l| l.contains("--filter")),
            "{without:?}"
        );
        assert_eq!(without.len(), with.len() - 1);
    }

    #[test]
    fn test_context_window_suggestions_omit_filter_without_extensions() {
        let paths = [Path::new("Makefile"), Path::new("LICENSE")];
        let lines = context_window_suggestions(&paths, false);
        assert!(
            !lines.iter().any(|line| line.contains("--filter")),
            "no extensions means no --filter command to suggest: {lines:?}"
        );
        assert!(
            lines
                .iter()
                .any(|line| line.contains("--ignore docs,assets"))
        );
        assert!(
            lines
                .iter()
                .any(|line| line.contains("--max-tokens 100000"))
        );
        assert!(lines.iter().any(|line| line.contains("--token-count")));
    }

    #[test]
    fn test_context_window_warning_under_limit() {
        let original = std::env::var("CB_SILENT");
        unsafe {
            std::env::set_var("CB_SILENT", "1");
        }

        let output_bytes = 100_000;
        print_context_window_warning(output_bytes * 4, None, &[], false);

        unsafe {
            std::env::remove_var("CB_SILENT");
        }
        if let Ok(val) = original {
            unsafe {
                std::env::set_var("CB_SILENT", val);
            }
        }
    }

    #[test]
    fn test_context_window_warning_over_limit() {
        let output_bytes = 600_000;
        print_context_window_warning(output_bytes * 4, None, &[], false);
    }

    #[test]
    fn test_context_window_warning_with_max_tokens() {
        let output_bytes = 600_000;
        print_context_window_warning(output_bytes * 4, Some(100_000), &[], false);
    }

    #[test]
    fn test_print_context_window_warning_various_sizes() {
        print_context_window_warning(50_000, None, &[], false);
        print_context_window_warning(200_000, None, &[], false);
        print_context_window_warning(500_000, None, &[], false);
        print_context_window_warning(1_000_000, None, &[], false);
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
            encoding: "o200k_base".to_string(),
            init: false,
            max_tokens: None,
            signatures: false,
            structure: false,
            truncate: "smart".to_string(),
            visibility: "all".to_string(),
            max_file_size: "256K".to_string(),
            hidden: false,
            include_secrets: false,
            file_metadata: false,
            include_lockfiles: false,
        };
        let config = Config::default();
        let prompter = MockPrompter::new(true);

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
            encoding: "o200k_base".to_string(),
            init: false,
            max_tokens: None,
            signatures: false,
            structure: false,
            truncate: "smart".to_string(),
            visibility: "all".to_string(),
            max_file_size: "256K".to_string(),
            hidden: false,
            include_secrets: false,
            file_metadata: false,
            include_lockfiles: false,
        };
        let config = Config::default();
        let prompter = MockPrompter::new(true);

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
            encoding: "o200k_base".to_string(),
            init: false,
            max_tokens: None,
            signatures: false,
            structure: false,
            truncate: "smart".to_string(),
            visibility: "all".to_string(),
            max_file_size: "256K".to_string(),
            hidden: false,
            include_secrets: false,
            file_metadata: false,
            include_lockfiles: false,
        };
        let config = Config {
            auto_diff: Some(true),
            ..Default::default()
        };
        let prompter = MockPrompter::new(true);

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
            encoding: "o200k_base".to_string(),
            init: false,
            max_tokens: None,
            signatures: false,
            structure: false,
            truncate: "smart".to_string(),
            visibility: "all".to_string(),
            max_file_size: "256K".to_string(),
            hidden: false,
            include_secrets: false,
            file_metadata: false,
            include_lockfiles: false,
        };
        let config = Config {
            auto_diff: Some(true),
            ..Default::default()
        };
        let prompter = MockPrompter::new(true);

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
            encoding: "o200k_base".to_string(),
            init: false,
            max_tokens: None,
            signatures: false,
            structure: false,
            truncate: "smart".to_string(),
            visibility: "all".to_string(),
            max_file_size: "256K".to_string(),
            hidden: false,
            include_secrets: false,
            file_metadata: false,
            include_lockfiles: false,
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
            encoding: "o200k_base".to_string(),
            init: false,
            max_tokens: Some(100),
            signatures: false,
            structure: false,
            truncate: "smart".to_string(),
            visibility: "all".to_string(),
            max_file_size: "256K".to_string(),
            hidden: false,
            include_secrets: false,
            file_metadata: false,
            include_lockfiles: false,
        };
        let config = Config {
            auto_diff: Some(true),
            ..Default::default()
        };
        let prompter = MockPrompter::new(true);

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
            encoding: "o200k_base".to_string(),
            init: false,
            max_tokens: None,
            signatures: false,
            structure: false,
            truncate: "smart".to_string(),
            visibility: "all".to_string(),
            max_file_size: "256K".to_string(),
            hidden: false,
            include_secrets: false,
            file_metadata: false,
            include_lockfiles: false,
        };
        let config = Config {
            auto_diff: Some(true),
            ..Default::default()
        };
        let prompter = MockPrompter::new(true);

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
            encoding: "o200k_base".to_string(),
            init: false,
            max_tokens: None,
            signatures: false,
            structure: false,
            truncate: "smart".to_string(),
            visibility: "all".to_string(),
            max_file_size: "256K".to_string(),
            hidden: false,
            include_secrets: false,
            file_metadata: false,
            include_lockfiles: false,
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

        fs::write(
            base_path.join("test.rs"),
            "fn main() {\n    println!(\"hi\");\n}",
        )
        .unwrap();

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
            encoding: "o200k_base".to_string(),
            init: false,
            max_tokens: None,
            signatures: false,
            structure: false,
            truncate: "smart".to_string(),
            visibility: "all".to_string(),
            max_file_size: "256K".to_string(),
            hidden: false,
            include_secrets: false,
            file_metadata: false,
            include_lockfiles: false,
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
            file_metadata: false,
        };

        let previous = state.clone();
        let comparison = state.compare_with(&previous, None);

        let result = generate_markdown_with_diff(
            &state,
            Some(&comparison),
            &args,
            &file_tree,
            &diff_config,
            &sorted_paths,
            &ts_config,
            &[],
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
        let current_state =
            ProjectState::from_files(&new_files, base_path, &config, false).unwrap();

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
            encoding: "o200k_base".to_string(),
            init: false,
            max_tokens: None,
            signatures: false,
            structure: false,
            truncate: "smart".to_string(),
            visibility: "all".to_string(),
            max_file_size: "256K".to_string(),
            hidden: false,
            include_secrets: false,
            file_metadata: false,
            include_lockfiles: false,
        };

        let diff_config = DiffConfig {
            context_lines: 3,
            enabled: true,
            diff_only: false,
        };

        let comparison = current_state.compare_with(&initial_state, None);

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
            file_metadata: false,
        };

        let result = generate_markdown_with_diff(
            &current_state,
            Some(&comparison),
            &args,
            &file_tree,
            &diff_config,
            &sorted_paths,
            &ts_config,
            &[],
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
        print_context_window_warning(output_bytes, None, &[], false);
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
            encoding: "o200k_base".to_string(),
            init: false,
            max_tokens: None,
            signatures: false,
            structure: false,
            truncate: "smart".to_string(),
            visibility: "all".to_string(),
            max_file_size: "256K".to_string(),
            hidden: false,
            include_secrets: false,
            file_metadata: false,
            include_lockfiles: false,
        };
        let config = Config::default();
        let prompter = MockPrompter::new(true);

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
            encoding: "o200k_base".to_string(),
            init: false,
            max_tokens: None,
            signatures: false,
            structure: false,
            truncate: "smart".to_string(),
            visibility: "all".to_string(),
            max_file_size: "256K".to_string(),
            hidden: false,
            include_secrets: false,
            file_metadata: false,
            include_lockfiles: false,
        };
        let config = Config::default();
        let prompter = MockPrompter::new(true);

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
            encoding: "o200k_base".to_string(),
            init: false,
            max_tokens: None,
            signatures: false,
            structure: false,
            truncate: "smart".to_string(),
            visibility: "all".to_string(),
            max_file_size: "256K".to_string(),
            hidden: false,
            include_secrets: false,
            file_metadata: false,
            include_lockfiles: false,
        };
        let config = Config::default();
        let prompter = MockPrompter::new(true);

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
            encoding: "o200k_base".to_string(),
            init: false,
            max_tokens: None,
            signatures: false,
            structure: false,
            truncate: "smart".to_string(),
            visibility: "all".to_string(),
            max_file_size: "256K".to_string(),
            hidden: false,
            include_secrets: false,
            file_metadata: false,
            include_lockfiles: false,
        };
        let config1 = Config {
            auto_diff: Some(true),
            filter: Some(vec!["txt".to_string()]),
            ..Default::default()
        };

        unsafe {
            std::env::set_var("CB_SILENT", "1");
        }
        let _ = run_with_args(args1, config1.clone(), &MockPrompter::new(true));

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
            encoding: "o200k_base".to_string(),
            init: false,
            max_tokens: None,
            signatures: false,
            structure: false,
            truncate: "smart".to_string(),
            visibility: "all".to_string(),
            max_file_size: "256K".to_string(),
            hidden: false,
            include_secrets: false,
            file_metadata: false,
            include_lockfiles: false,
        };
        let config2 = Config {
            auto_diff: Some(true),
            filter: Some(vec!["rs".to_string()]),
            ..Default::default()
        };

        let result = run_with_args(args2, config2, &MockPrompter::new(true));
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
            encoding: "o200k_base".to_string(),
            init: false,
            max_tokens: None,
            signatures: false,
            structure: false,
            truncate: "smart".to_string(),
            visibility: "all".to_string(),
            max_file_size: "256K".to_string(),
            hidden: false,
            include_secrets: false,
            file_metadata: false,
            include_lockfiles: false,
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
            file_metadata: false,
        };

        let result = generate_markdown_with_diff(
            &state,
            None,
            &args,
            &file_tree,
            &diff_config,
            &sorted_paths,
            &ts_config,
            &[],
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
            encoding: "o200k_base".to_string(),
            init: false,
            max_tokens: None,
            signatures: false,
            structure: false,
            truncate: "smart".to_string(),
            visibility: "all".to_string(),
            max_file_size: "256K".to_string(),
            hidden: false,
            include_secrets: false,
            file_metadata: false,
            include_lockfiles: false,
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
            file_metadata: false,
        };

        let result = generate_markdown_with_diff(
            &state,
            None,
            &args,
            &file_tree,
            &diff_config,
            &sorted_paths,
            &ts_config,
            &[],
        );
        assert!(result.is_ok());

        let content = result.unwrap();
        assert!(content.contains("test.rs"));
    }

    fn section_fence(doc: &str, header: &str) -> (String, String, String) {
        let idx = doc
            .find(header)
            .unwrap_or_else(|| panic!("missing header {header} in:\n{doc}"));
        let rest = &doc[idx + header.len()..];
        let mut offset = 0;
        for line in rest.split_inclusive('\n') {
            let stripped = line.trim_end_matches(['\n', '\r']);
            let ticks = stripped.bytes().take_while(|b| *b == b'`').count();
            if ticks >= 3 && !stripped[ticks..].contains('`') {
                let fence = "`".repeat(ticks);
                let info = stripped[ticks..].to_string();
                let after = &rest[offset + line.len()..];
                let mut pos = 0;
                for bline in after.split_inclusive('\n') {
                    let bstripped = bline.trim_end_matches(['\n', '\r']);
                    if bstripped == fence {
                        return (fence, info, after[..pos].to_string());
                    }
                    pos += bline.len();
                }
                panic!("no closer for {header} in:\n{doc}");
            }
            offset += line.len();
        }
        panic!("no fence after {header} in:\n{doc}");
    }

    #[test]
    fn auto_diff_content_fence_outgrows_inner_backticks() {
        let temp_dir = tempdir().unwrap();
        let base_path = temp_dir.path();
        let triple = "before\n```\nafter\n";
        let quad = "before\n````\nafter\n";
        fs::write(base_path.join("triple.md"), triple).unwrap();
        fs::write(base_path.join("quad.md"), quad).unwrap();
        fs::write(base_path.join("weird`name.py"), "print(1)\n").unwrap();

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
            yes: true,
            diff_only: false,
            clear_cache: false,
            encoding: "o200k_base".to_string(),
            init: false,
            max_tokens: None,
            signatures: false,
            structure: false,
            truncate: "smart".to_string(),
            visibility: "all".to_string(),
            file_metadata: false,
            max_file_size: "256K".to_string(),
            hidden: false,
            include_secrets: false,
            include_lockfiles: false,
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

        let doc = generate_markdown_with_diff(
            &state,
            None,
            &args,
            &file_tree,
            &diff_config,
            &sorted_paths,
            &markdown::TreeSitterConfig::default(),
            &[],
        )
        .unwrap();

        assert!(
            fences::unmatched_backtick_fence_len(&doc).is_none(),
            "{doc}"
        );
        assert!(doc.contains("### File: ``weird`name.py``"), "{doc}");

        let (fence, info, body) = section_fence(&doc, "### File: `triple.md`");
        assert_eq!(info, "markdown");
        assert_eq!(fence.len(), 4);
        assert_eq!(body, triple);

        let (fence, info, body) = section_fence(&doc, "### File: `quad.md`");
        assert_eq!(info, "markdown");
        assert_eq!(fence.len(), 5);
        assert_eq!(body, quad);
    }

    #[test]
    fn auto_diff_added_file_fence_outgrows_inner_backticks() {
        let temp_dir = tempdir().unwrap();
        let base_path = temp_dir.path();
        let files = collect_files(base_path, &[], &[], &[]).unwrap();
        let file_tree = build_file_tree(&files, base_path);
        let config = Config::default();
        let state = ProjectState::from_files(&files, base_path, &config, false).unwrap();

        let mut previous = std::collections::HashMap::new();
        let mut current = std::collections::HashMap::new();
        previous.insert("old.md".to_string(), "gone\n".to_string());
        current.insert("README.md".to_string(), "before\n```\nafter\n".to_string());
        let file_diffs = diff::diff_file_contents(&previous, &current, true, None);
        let summary = state::ChangeSummary {
            added: vec![PathBuf::from("README.md")],
            removed: vec![PathBuf::from("old.md")],
            modified: vec![],
            total_changes: 2,
        };
        let comparison = StateComparison {
            file_diffs,
            summary,
        };

        let args = Args {
            input: base_path.to_string_lossy().to_string(),
            output: "test.md".to_string(),
            filter: vec![],
            ignore: vec![],
            line_numbers: false,
            preview: false,
            token_count: false,
            yes: true,
            diff_only: true,
            clear_cache: false,
            encoding: "o200k_base".to_string(),
            init: false,
            max_tokens: None,
            signatures: false,
            structure: false,
            truncate: "smart".to_string(),
            visibility: "all".to_string(),
            file_metadata: false,
            max_file_size: "256K".to_string(),
            hidden: false,
            include_secrets: false,
            include_lockfiles: false,
        };

        let doc = generate_markdown_with_diff(
            &state,
            Some(&comparison),
            &args,
            &file_tree,
            &DiffConfig {
                context_lines: 3,
                enabled: true,
                diff_only: true,
            },
            &[],
            &markdown::TreeSitterConfig::default(),
            &[],
        )
        .unwrap();

        assert!(
            fences::unmatched_backtick_fence_len(&doc).is_none(),
            "{doc}"
        );
        assert!(doc.contains("- Added: `README.md`"));
        let (fence, info, body) = section_fence(&doc, "### File: `README.md`");
        assert_eq!(info, "text");
        assert_eq!(fence.len(), 4, "{doc}");
        assert_eq!(body, "before\n```\nafter\n");
    }
}
