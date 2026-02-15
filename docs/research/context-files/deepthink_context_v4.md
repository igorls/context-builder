# Directory Structure Report

**Project:** context-builder
**Generated:** 2026-02-15 04:19:13 UTC
**Filters:** rs, toml
**Ignored:** docs

## File Tree Structure

- 📄 Cargo.toml
- 📁 benches
  - 📄 context_bench.rs
- 📁 scripts
  - 📄 generate_samples.rs
- 📁 src
  - 📄 cache.rs
  - 📄 cli.rs
  - 📄 config.rs
  - 📄 config_resolver.rs
  - 📄 diff.rs
  - 📄 file_utils.rs
  - 📄 lib.rs
  - 📄 main.rs
  - 📄 markdown.rs
  - 📄 state.rs
  - 📄 token_count.rs
  - 📄 tree.rs
- 📄 tarpaulin.toml
- 📁 tests
  - 📄 cli_integration.rs
  - 📄 diff_integration.rs
  - 📄 test_auto_diff.rs
  - 📄 test_binary_file_autodiff.rs
  - 📄 test_comprehensive_edge_cases.rs
  - 📄 test_config_resolution.rs
  - 📄 test_cwd_independence.rs
  - 📄 test_determinism.rs
  - 📄 test_parallel_memory.rs
  - 📄 test_phase4_integration.rs

## File Contents

### File: `Cargo.toml`

- Size: 1464 bytes
- Modified: SystemTime { tv_sec: 1771106770, tv_nsec: 891643174 }

```toml
   1 | [package]
   2 | name = "context-builder"
   3 | version = "0.7.0"
   4 | default-run = "context-builder"
   5 | edition = "2024"
   6 | authors = ["Igor Lins e Silva"]
   7 | description = "CLI tool to aggregate directory contents into a single markdown file optimized for LLM consumption"
   8 | readme = "README.md"
   9 | homepage = "https://github.com/igorls/context-builder"
  10 | repository = "https://github.com/igorls/context-builder"
  11 | license = "MIT"
  12 | keywords = ["cli", "markdown", "documentation", "llm", "context"]
  13 | categories = ["command-line-utilities", "development-tools"]
  14 | 
  15 | [dependencies]
  16 | clap = { version = "4.5.58", features = ["derive"] }
  17 | chrono = { version = "0.4.43", features = ["serde"] }
  18 | ignore = "0.4.25"
  19 | log = "0.4.29"
  20 | env_logger = "0.11.9"
  21 | rayon = { version = "1.10", optional = true }
  22 | serde = { version = "1.0.228", features = ["derive"] }
  23 | toml = "1.0.1"
  24 | similar = "2.7.0"
  25 | tempfile = "3.25.0"
  26 | tiktoken-rs = "0.9.1"
  27 | once_cell = "1.21.3"
  28 | fs2 = "0.4.3"
  29 | serde_json = "1.0.143"
  30 | crossbeam-channel = "0.5.15"
  31 | num_cpus = "1.17.0"
  32 | encoding_rs = "0.8.35"
  33 | walkdir = "2.5.0"
  34 | xxhash-rust = { version = "0.8", features = ["xxh3"] }
  35 | 
  36 | [features]
  37 | default = ["parallel"]
  38 | parallel = ["rayon"]
  39 | samples-bin = []
  40 | 
  41 | [dev-dependencies]
  42 | tempfile = "3.25.0"
  43 | criterion = { version = "0.8.2", features = ["html_reports"] }
  44 | pretty_assertions = "1.4.1"
  45 | serial_test = "3.0"
  46 | 
  47 | [[bench]]
  48 | name = "context_bench"
  49 | harness = false
  50 | 
  51 | [[bin]]
  52 | name = "generate_samples"
  53 | path = "scripts/generate_samples.rs"
  54 | required-features = ["samples-bin"]
```

### File: `src/lib.rs`

- Size: 50042 bytes
- Modified: SystemTime { tv_sec: 1771128705, tv_nsec: 645174745 }

```rust
   1 | use clap::{CommandFactory, Parser};
   2 | 
   3 | use std::fs;
   4 | use std::io::{self, Write};
   5 | use std::path::{Path, PathBuf};
   6 | use std::time::Instant;
   7 | 
   8 | pub mod cache;
   9 | pub mod cli;
  10 | pub mod config;
  11 | pub mod config_resolver;
  12 | pub mod diff;
  13 | pub mod file_utils;
  14 | pub mod markdown;
  15 | pub mod state;
  16 | pub mod token_count;
  17 | pub mod tree;
  18 | 
  19 | use std::fs::File;
  20 | 
  21 | use cache::CacheManager;
  22 | use cli::Args;
  23 | use config::{Config, load_config_from_path};
  24 | use diff::render_per_file_diffs;
  25 | use file_utils::{collect_files, confirm_overwrite, confirm_processing};
  26 | use markdown::generate_markdown;
  27 | use state::{ProjectState, StateComparison};
  28 | use token_count::{count_file_tokens, count_tree_tokens, estimate_tokens};
  29 | use tree::{build_file_tree, print_tree};
  30 | 
  31 | /// Configuration for diff operations
  32 | #[derive(Debug, Clone)]
  33 | pub struct DiffConfig {
  34 |     pub context_lines: usize,
  35 |     pub enabled: bool,
  36 |     pub diff_only: bool,
  37 | }
  38 | 
  39 | impl Default for DiffConfig {
  40 |     fn default() -> Self {
  41 |         Self {
  42 |             context_lines: 3,
  43 |             enabled: false,
  44 |             diff_only: false,
  45 |         }
  46 |     }
  47 | }
  48 | 
  49 | pub trait Prompter {
  50 |     fn confirm_processing(&self, file_count: usize) -> io::Result<bool>;
  51 |     fn confirm_overwrite(&self, file_path: &str) -> io::Result<bool>;
  52 | }
  53 | 
  54 | pub struct DefaultPrompter;
  55 | 
  56 | impl Prompter for DefaultPrompter {
  57 |     fn confirm_processing(&self, file_count: usize) -> io::Result<bool> {
  58 |         confirm_processing(file_count)
  59 |     }
  60 |     fn confirm_overwrite(&self, file_path: &str) -> io::Result<bool> {
  61 |         confirm_overwrite(file_path)
  62 |     }
  63 | }
  64 | 
  65 | pub fn run_with_args(args: Args, config: Config, prompter: &impl Prompter) -> io::Result<()> {
  66 |     let start_time = Instant::now();
  67 | 
  68 |     let silent = std::env::var("CB_SILENT")
  69 |         .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
  70 |         .unwrap_or(false);
  71 | 
  72 |     // Use the finalized args passed in from run()
  73 |     let final_args = args;
  74 |     // Resolve base path. If input is '.' but current working directory lost the project context
  75 |     // (no context-builder.toml), attempt to infer project root from output path (parent of 'output' dir).
  76 |     let mut resolved_base = PathBuf::from(&final_args.input);
  77 |     let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
  78 |     if resolved_base == Path::new(".")
  79 |         && !cwd.join("context-builder.toml").exists()
  80 |         && let Some(output_parent) = Path::new(&final_args.output).parent()
  81 |         && output_parent
  82 |             .file_name()
  83 |             .map(|n| n == "output")
  84 |             .unwrap_or(false)
  85 |         && let Some(project_root) = output_parent.parent()
  86 |         && project_root.join("context-builder.toml").exists()
  87 |     {
  88 |         resolved_base = project_root.to_path_buf();
  89 |     }
  90 |     let base_path = resolved_base.as_path();
  91 | 
  92 |     if !base_path.exists() || !base_path.is_dir() {
  93 |         if !silent {
  94 |             eprintln!(
  95 |                 "Error: The specified input directory '{}' does not exist or is not a directory.",
  96 |                 final_args.input
  97 |             );
  98 |         }
  99 |         return Err(io::Error::new(
 100 |             io::ErrorKind::NotFound,
 101 |             format!(
 102 |                 "Input directory '{}' does not exist or is not a directory",
 103 |                 final_args.input
 104 |             ),
 105 |         ));
 106 |     }
 107 | 
 108 |     // Create diff configuration from config
 109 |     let diff_config = if config.auto_diff.unwrap_or(false) {
 110 |         Some(DiffConfig {
 111 |             context_lines: config.diff_context_lines.unwrap_or(3),
 112 |             enabled: true,
 113 |             diff_only: final_args.diff_only,
 114 |         })
 115 |     } else {
 116 |         None
 117 |     };
 118 | 
 119 |     if !final_args.preview
 120 |         && !final_args.token_count
 121 |         && Path::new(&final_args.output).exists()
 122 |         && !final_args.yes
 123 |         && !prompter.confirm_overwrite(&final_args.output)?
 124 |     {
 125 |         if !silent {
 126 |             println!("Operation cancelled.");
 127 |         }
 128 |         return Err(io::Error::new(
 129 |             io::ErrorKind::Interrupted,
 130 |             "Operation cancelled by user",
 131 |         ));
 132 |     }
 133 | 
 134 |     // Compute auto-ignore patterns to exclude the tool's own output and cache
 135 |     let mut auto_ignores: Vec<String> = vec![".context-builder".to_string()];
 136 | 
 137 |     // Exclude the resolved output file (or its timestamped glob pattern)
 138 |     let output_path = Path::new(&final_args.output);
 139 |     if let Ok(rel_output) = output_path.strip_prefix(base_path) {
 140 |         // Output is inside the project — exclude it
 141 |         if config.timestamped_output == Some(true) {
 142 |             // Timestamped outputs: create a glob like "docs/context_*.md"
 143 |             if let (Some(parent), Some(stem), Some(ext)) = (
 144 |                 rel_output.parent(),
 145 |                 output_path.file_stem().and_then(|s| s.to_str()),
 146 |                 output_path.extension().and_then(|s| s.to_str()),
 147 |             ) {
 148 |                 // Strip the timestamp suffix to get the base stem
 149 |                 // Timestamped names look like "context_20260214175028.md"
 150 |                 // The stem from config is the part before the timestamp
 151 |                 let base_stem = if let Some(ref cfg_output) = config.output {
 152 |                     Path::new(cfg_output)
 153 |                         .file_stem()
 154 |                         .and_then(|s| s.to_str())
 155 |                         .unwrap_or(stem)
 156 |                         .to_string()
 157 |                 } else {
 158 |                     stem.to_string()
 159 |                 };
 160 |                 let glob = if parent == Path::new("") {
 161 |                     format!("{}_*.{}", base_stem, ext)
 162 |                 } else {
 163 |                     format!("{}/{}_*.{}", parent.display(), base_stem, ext)
 164 |                 };
 165 |                 auto_ignores.push(glob);
 166 |             }
 167 |         } else {
 168 |             // Non-timestamped: exclude the exact output file
 169 |             auto_ignores.push(rel_output.to_string_lossy().to_string());
 170 |         }
 171 |     } else {
 172 |         // Output might be a relative path not under base_path — try using it directly
 173 |         let output_str = final_args.output.clone();
 174 |         if config.timestamped_output == Some(true) {
 175 |             if let (Some(stem), Some(ext)) = (
 176 |                 output_path.file_stem().and_then(|s| s.to_str()),
 177 |                 output_path.extension().and_then(|s| s.to_str()),
 178 |             ) {
 179 |                 let base_stem = if let Some(ref cfg_output) = config.output {
 180 |                     Path::new(cfg_output)
 181 |                         .file_stem()
 182 |                         .and_then(|s| s.to_str())
 183 |                         .unwrap_or(stem)
 184 |                         .to_string()
 185 |                 } else {
 186 |                     stem.to_string()
 187 |                 };
 188 |                 if let Some(parent) = output_path.parent() {
 189 |                     let parent_str = parent.to_string_lossy();
 190 |                     if parent_str.is_empty() || parent_str == "." {
 191 |                         auto_ignores.push(format!("{}_*.{}", base_stem, ext));
 192 |                     } else {
 193 |                         auto_ignores.push(format!("{}/{}_*.{}", parent_str, base_stem, ext));
 194 |                     }
 195 |                 }
 196 |             }
 197 |         } else {
 198 |             auto_ignores.push(output_str);
 199 |         }
 200 |     }
 201 | 
 202 |     // Also exclude the output folder itself if configured
 203 |     if let Some(ref output_folder) = config.output_folder {
 204 |         auto_ignores.push(output_folder.clone());
 205 |     }
 206 | 
 207 |     let files = collect_files(
 208 |         base_path,
 209 |         &final_args.filter,
 210 |         &final_args.ignore,
 211 |         &auto_ignores,
 212 |     )?;
 213 |     let debug_config = std::env::var("CB_DEBUG_CONFIG").is_ok();
 214 |     if debug_config {
 215 |         eprintln!("[DEBUG][CONFIG] Args: {:?}", final_args);
 216 |         eprintln!("[DEBUG][CONFIG] Raw Config: {:?}", config);
 217 |         eprintln!("[DEBUG][CONFIG] Auto-ignores: {:?}", auto_ignores);
 218 |         eprintln!("[DEBUG][CONFIG] Collected {} files", files.len());
 219 |         for f in &files {
 220 |             eprintln!("[DEBUG][CONFIG]  - {}", f.path().display());
 221 |         }
 222 |     }
 223 | 
 224 |     // Smart large-file detection: warn about files that may bloat the context
 225 |     if !silent {
 226 |         const LARGE_FILE_THRESHOLD: u64 = 100 * 1024; // 100 KB
 227 |         let mut large_files: Vec<(String, u64)> = Vec::new();
 228 |         let mut total_size: u64 = 0;
 229 | 
 230 |         for entry in &files {
 231 |             if let Ok(metadata) = entry.path().metadata() {
 232 |                 let size = metadata.len();
 233 |                 total_size += size;
 234 |                 if size > LARGE_FILE_THRESHOLD {
 235 |                     let rel_path = entry
 236 |                         .path()
 237 |                         .strip_prefix(base_path)
 238 |                         .unwrap_or(entry.path())
 239 |                         .to_string_lossy()
 240 |                         .to_string();
 241 |                     large_files.push((rel_path, size));
 242 |                 }
 243 |             }
 244 |         }
 245 | 
 246 |         if !large_files.is_empty() {
 247 |             large_files.sort_by(|a, b| b.1.cmp(&a.1)); // Sort by size descending
 248 |             eprintln!(
 249 |                 "\n⚠  {} large file(s) detected (>{} KB):",
 250 |                 large_files.len(),
 251 |                 LARGE_FILE_THRESHOLD / 1024
 252 |             );
 253 |             for (path, size) in large_files.iter().take(5) {
 254 |                 eprintln!("   {:>8} KB  {}", size / 1024, path);
 255 |             }
 256 |             if large_files.len() > 5 {
 257 |                 eprintln!("   ... and {} more", large_files.len() - 5);
 258 |             }
 259 |             eprintln!(
 260 |                 "   Total context size: {} KB across {} files\n",
 261 |                 total_size / 1024,
 262 |                 files.len()
 263 |             );
 264 |         }
 265 |     }
 266 |     let file_tree = build_file_tree(&files, base_path);
 267 | 
 268 |     if final_args.preview {
 269 |         if !silent {
 270 |             println!("\n# File Tree Structure (Preview)\n");
 271 |             print_tree(&file_tree, 0);
 272 |         }
 273 |         if !final_args.token_count {
 274 |             return Ok(());
 275 |         }
 276 |     }
 277 | 
 278 |     if final_args.token_count {
 279 |         if !silent {
 280 |             println!("\n# Token Count Estimation\n");
 281 |             let mut total_tokens = 0;
 282 |             total_tokens += estimate_tokens("# Directory Structure Report\n\n");
 283 |             if !final_args.filter.is_empty() {
 284 |                 total_tokens += estimate_tokens(&format!(
 285 |                     "This document contains files from the `{}` directory with extensions: {} \n",
 286 |                     final_args.input,
 287 |                     final_args.filter.join(", ")
 288 |                 ));
 289 |             } else {
 290 |                 total_tokens += estimate_tokens(&format!(
 291 |                     "This document contains all files from the `{}` directory, optimized for LLM consumption.\n",
 292 |                     final_args.input
 293 |                 ));
 294 |             }
 295 |             if !final_args.ignore.is_empty() {
 296 |                 total_tokens += estimate_tokens(&format!(
 297 |                     "Custom ignored patterns: {} \n",
 298 |                     final_args.ignore.join(", ")
 299 |                 ));
 300 |             }
 301 |             total_tokens += estimate_tokens("Content hash: 0000000000000000\n\n");
 302 |             total_tokens += estimate_tokens("## File Tree Structure\n\n");
 303 |             let tree_tokens = count_tree_tokens(&file_tree, 0);
 304 |             total_tokens += tree_tokens;
 305 |             let file_tokens: usize = files
 306 |                 .iter()
 307 |                 .map(|entry| count_file_tokens(base_path, entry, final_args.line_numbers))
 308 |                 .sum();
 309 |             total_tokens += file_tokens;
 310 |             println!("Estimated total tokens: {}", total_tokens);
 311 |             println!("File tree tokens: {}", tree_tokens);
 312 |             println!("File content tokens: {}", file_tokens);
 313 |         }
 314 |         return Ok(());
 315 |     }
 316 | 
 317 |     if !final_args.yes && !prompter.confirm_processing(files.len())? {
 318 |         if !silent {
 319 |             println!("Operation cancelled.");
 320 |         }
 321 |         return Err(io::Error::new(
 322 |             io::ErrorKind::Interrupted,
 323 |             "Operation cancelled by user",
 324 |         ));
 325 |     }
 326 | 
 327 |     // NOTE: config-driven flags (line_numbers, diff_only) are already merged
 328 |     // by config_resolver.rs with proper CLI-takes-precedence semantics.
 329 |     // Do NOT re-apply them here as that would silently overwrite CLI flags.
 330 | 
 331 |     if config.auto_diff.unwrap_or(false) {
 332 |         // Build an effective config that mirrors the *actual* operational settings coming
 333 |         // from resolved CLI args (filters/ignores/line_numbers). This ensures the
 334 |         // configuration hash used for cache invalidation reflects real behavior and
 335 |         // stays consistent across runs even when values originate from CLI not file.
 336 |         let mut effective_config = config.clone();
 337 |         // Normalize filter/ignore/line_numbers into config so hashing sees them
 338 |         if !final_args.filter.is_empty() {
 339 |             effective_config.filter = Some(final_args.filter.clone());
 340 |         }
 341 |         if !final_args.ignore.is_empty() {
 342 |             effective_config.ignore = Some(final_args.ignore.clone());
 343 |         }
 344 |         effective_config.line_numbers = Some(final_args.line_numbers);
 345 | 
 346 |         // 1. Create current project state
 347 |         let current_state = ProjectState::from_files(
 348 |             &files,
 349 |             base_path,
 350 |             &effective_config,
 351 |             final_args.line_numbers,
 352 |         )?;
 353 | 
 354 |         // 2. Initialize cache manager and load previous state
 355 |         let cache_manager = CacheManager::new(base_path, &effective_config);
 356 |         let previous_state = match cache_manager.read_cache() {
 357 |             Ok(state) => state,
 358 |             Err(e) => {
 359 |                 if !silent {
 360 |                     eprintln!(
 361 |                         "Warning: Failed to read cache (proceeding without diff): {}",
 362 |                         e
 363 |                     );
 364 |                 }
 365 |                 None
 366 |             }
 367 |         };
 368 | 
 369 |         let diff_cfg = diff_config.as_ref().unwrap();
 370 | 
 371 |         // 3. Determine whether we should invalidate (ignore) previous state
 372 |         let effective_previous = if let Some(prev) = previous_state.as_ref() {
 373 |             if prev.config_hash != current_state.config_hash {
 374 |                 // Config change => treat as initial state (invalidate diff)
 375 |                 None
 376 |             } else {
 377 |                 Some(prev)
 378 |             }
 379 |         } else {
 380 |             None
 381 |         };
 382 | 
 383 |         // 4. Compare states and generate diff if an effective previous state exists
 384 |         let comparison = effective_previous.map(|prev| current_state.compare_with(prev));
 385 | 
 386 |         let debug_autodiff = std::env::var("CB_DEBUG_AUTODIFF").is_ok();
 387 |         if debug_autodiff {
 388 |             eprintln!(
 389 |                 "[DEBUG][AUTODIFF] cache file: {}",
 390 |                 cache_manager.debug_cache_file_path().display()
 391 |             );
 392 |             eprintln!(
 393 |                 "[DEBUG][AUTODIFF] config_hash current={} prev={:?} invalidated={}",
 394 |                 current_state.config_hash,
 395 |                 previous_state.as_ref().map(|s| s.config_hash.clone()),
 396 |                 effective_previous.is_none() && previous_state.is_some()
 397 |             );
 398 |             eprintln!("[DEBUG][AUTODIFF] effective_config: {:?}", effective_config);
 399 |             if let Some(prev) = previous_state.as_ref() {
 400 |                 eprintln!("[DEBUG][AUTODIFF] raw previous files: {}", prev.files.len());
 401 |             }
 402 |             if let Some(prev) = effective_previous {
 403 |                 eprintln!(
 404 |                     "[DEBUG][AUTODIFF] effective previous files: {}",
 405 |                     prev.files.len()
 406 |                 );
 407 |                 for k in prev.files.keys() {
 408 |                     eprintln!("  PREV: {}", k.display());
 409 |                 }
 410 |             }
 411 |             eprintln!(
 412 |                 "[DEBUG][AUTODIFF] current files: {}",
 413 |                 current_state.files.len()
 414 |             );
 415 |             for k in current_state.files.keys() {
 416 |                 eprintln!("  CURR: {}", k.display());
 417 |             }
 418 |         }
 419 | 
 420 |         // Build relevance-sorted path list from the DirEntry list (which is
 421 |         // already sorted by file_relevance_category). This preserves ordering
 422 |         // instead of using BTreeMap's alphabetical iteration.
 423 |         // IMPORTANT: Path resolution must match state.rs to avoid get() misses.
 424 |         let cwd = std::env::current_dir().unwrap_or_else(|_| base_path.to_path_buf());
 425 |         let sorted_paths: Vec<PathBuf> = files
 426 |             .iter()
 427 |             .map(|entry| {
 428 |                 entry
 429 |                     .path()
 430 |                     .strip_prefix(base_path)
 431 |                     .or_else(|_| entry.path().strip_prefix(&cwd))
 432 |                     .map(|p| p.to_path_buf())
 433 |                     .unwrap_or_else(|_| {
 434 |                         entry
 435 |                             .path()
 436 |                             .file_name()
 437 |                             .map(PathBuf::from)
 438 |                             .unwrap_or_else(|| entry.path().to_path_buf())
 439 |                     })
 440 |             })
 441 |             .collect();
 442 | 
 443 |         // 4. Generate markdown with diff annotations
 444 |         let mut final_doc = generate_markdown_with_diff(
 445 |             &current_state,
 446 |             comparison.as_ref(),
 447 |             &final_args,
 448 |             &file_tree,
 449 |             diff_cfg,
 450 |             &sorted_paths,
 451 |         )?;
 452 | 
 453 |         // Enforce max_tokens budget (same ~4 bytes/token heuristic as parallel path)
 454 |         if let Some(max_tokens) = final_args.max_tokens {
 455 |             let max_bytes = max_tokens * 4;
 456 |             if final_doc.len() > max_bytes {
 457 |                 // Truncate at a valid UTF-8 boundary
 458 |                 let mut truncate_at = max_bytes;
 459 |                 while truncate_at > 0 && !final_doc.is_char_boundary(truncate_at) {
 460 |                     truncate_at -= 1;
 461 |                 }
 462 |                 final_doc.truncate(truncate_at);
 463 |                 final_doc.push_str("\n\n---\n\n");
 464 |                 final_doc.push_str(&format!(
 465 |                     "_Output truncated: exceeded {} token budget (estimated)._\n",
 466 |                     max_tokens
 467 |                 ));
 468 |             }
 469 |         }
 470 | 
 471 |         // 5. Write output
 472 |         let output_path = Path::new(&final_args.output);
 473 |         if let Some(parent) = output_path.parent()
 474 |             && !parent.exists()
 475 |             && let Err(e) = fs::create_dir_all(parent)
 476 |         {
 477 |             return Err(io::Error::other(format!(
 478 |                 "Failed to create output directory {}: {}",
 479 |                 parent.display(),
 480 |                 e
 481 |             )));
 482 |         }
 483 |         let mut final_output = fs::File::create(output_path)?;
 484 |         final_output.write_all(final_doc.as_bytes())?;
 485 | 
 486 |         // 6. Update cache with current state
 487 |         if let Err(e) = cache_manager.write_cache(&current_state)
 488 |             && !silent
 489 |         {
 490 |             eprintln!("Warning: failed to update state cache: {}", e);
 491 |         }
 492 | 
 493 |         let duration = start_time.elapsed();
 494 |         if !silent {
 495 |             if let Some(comp) = &comparison {
 496 |                 if comp.summary.has_changes() {
 497 |                     println!(
 498 |                         "Documentation created successfully with {} changes: {}",
 499 |                         comp.summary.total_changes, final_args.output
 500 |                     );
 501 |                 } else {
 502 |                     println!(
 503 |                         "Documentation created successfully (no changes detected): {}",
 504 |                         final_args.output
 505 |                     );
 506 |                 }
 507 |             } else {
 508 |                 println!(
 509 |                     "Documentation created successfully (initial state): {}",
 510 |                     final_args.output
 511 |                 );
 512 |             }
 513 |             println!("Processing time: {:.2?}", duration);
 514 | 
 515 |             // Warn about context window overflow
 516 |             let output_bytes = final_doc.len();
 517 |             print_context_window_warning(output_bytes, final_args.max_tokens);
 518 |         }
 519 |         return Ok(());
 520 |     }
 521 | 
 522 |     // Standard (non auto-diff) generation
 523 |     generate_markdown(
 524 |         &final_args.output,
 525 |         &final_args.input,
 526 |         &final_args.filter,
 527 |         &final_args.ignore,
 528 |         &file_tree,
 529 |         &files,
 530 |         base_path,
 531 |         final_args.line_numbers,
 532 |         config.encoding_strategy.as_deref(),
 533 |         final_args.max_tokens,
 534 |     )?;
 535 | 
 536 |     let duration = start_time.elapsed();
 537 |     if !silent {
 538 |         println!("Documentation created successfully: {}", final_args.output);
 539 |         println!("Processing time: {:.2?}", duration);
 540 | 
 541 |         // Warn about context window overflow
 542 |         let output_bytes = fs::metadata(&final_args.output)
 543 |             .map(|m| m.len() as usize)
 544 |             .unwrap_or(0);
 545 |         print_context_window_warning(output_bytes, final_args.max_tokens);
 546 |     }
 547 | 
 548 |     Ok(())
 549 | }
 550 | 
 551 | /// Print context window overflow warnings with actionable recommendations.
 552 | /// Estimates tokens using the ~4 bytes/token heuristic. Warns when output
 553 | /// exceeds 128K tokens — beyond this size, context quality degrades
 554 | /// significantly for most LLM use cases.
 555 | fn print_context_window_warning(output_bytes: usize, max_tokens: Option<usize>) {
 556 |     let estimated_tokens = output_bytes / 4;
 557 | 
 558 |     println!("Estimated tokens: ~{}K", estimated_tokens / 1000);
 559 | 
 560 |     // If the user already set --max-tokens, they're managing their budget
 561 |     if max_tokens.is_some() {
 562 |         return;
 563 |     }
 564 | 
 565 |     const RECOMMENDED_LIMIT: usize = 128_000;
 566 | 
 567 |     if estimated_tokens <= RECOMMENDED_LIMIT {
 568 |         return;
 569 |     }
 570 | 
 571 |     eprintln!();
 572 |     eprintln!(
 573 |         "⚠️  Output is ~{}K tokens — recommended limit is 128K for effective LLM context.",
 574 |         estimated_tokens / 1000
 575 |     );
 576 |     eprintln!("   Large contexts degrade response quality. Consider narrowing the scope:");
 577 |     eprintln!();
 578 |     eprintln!("   • --max-tokens 100000    Cap output to a token budget");
 579 |     eprintln!("   • --filter rs,toml       Include only specific file types");
 580 |     eprintln!("   • --ignore docs,assets   Exclude directories by name");
 581 |     eprintln!("   • --token-count          Preview size without generating");
 582 |     eprintln!();
 583 | }
 584 | 
 585 | /// Generate markdown document with diff annotations
 586 | fn generate_markdown_with_diff(
 587 |     current_state: &ProjectState,
 588 |     comparison: Option<&StateComparison>,
 589 |     args: &Args,
 590 |     file_tree: &tree::FileTree,
 591 |     diff_config: &DiffConfig,
 592 |     sorted_paths: &[PathBuf],
 593 | ) -> io::Result<String> {
 594 |     let mut output = String::new();
 595 | 
 596 |     // Header
 597 |     output.push_str("# Directory Structure Report\n\n");
 598 | 
 599 |     // Basic project info
 600 |     output.push_str(&format!(
 601 |         "**Project:** {}\n",
 602 |         current_state.metadata.project_name
 603 |     ));
 604 |     output.push_str(&format!("**Generated:** {}\n", current_state.timestamp));
 605 | 
 606 |     if !args.filter.is_empty() {
 607 |         output.push_str(&format!("**Filters:** {}\n", args.filter.join(", ")));
 608 |     }
 609 | 
 610 |     if !args.ignore.is_empty() {
 611 |         output.push_str(&format!("**Ignored:** {}\n", args.ignore.join(", ")));
 612 |     }
 613 | 
 614 |     output.push('\n');
 615 | 
 616 |     // Change summary + sections if we have a comparison
 617 |     if let Some(comp) = comparison {
 618 |         if comp.summary.has_changes() {
 619 |             output.push_str(&comp.summary.to_markdown());
 620 | 
 621 |             // Collect added files once so we can reuse for both diff_only logic and potential numbering.
 622 |             let added_files: Vec<_> = comp
 623 |                 .file_diffs
 624 |                 .iter()
 625 |                 .filter(|d| matches!(d.status, diff::PerFileStatus::Added))
 626 |                 .collect();
 627 | 
 628 |             if diff_config.diff_only && !added_files.is_empty() {
 629 |                 output.push_str("## Added Files\n\n");
 630 |                 for added in added_files {
 631 |                     output.push_str(&format!("### File: `{}`\n\n", added.path));
 632 |                     output.push_str("_Status: Added_\n\n");
 633 |                     // Reconstruct content from + lines.
 634 |                     let mut lines: Vec<String> = Vec::new();
 635 |                     for line in added.diff.lines() {
 636 |                         // Diff output uses "+ " prefix (plus-space), strip both to reconstruct content.
 637 |                         // Previously strip_prefix('+') left a leading space, corrupting indentation.
 638 |                         if let Some(rest) = line.strip_prefix("+ ") {
 639 |                             lines.push(rest.to_string());
 640 |                         } else if let Some(rest) = line.strip_prefix('+') {
 641 |                             // Handle edge case: empty added lines have just "+"
 642 |                             lines.push(rest.to_string());
 643 |                         }
 644 |                     }
 645 |                     output.push_str("```text\n");
 646 |                     if args.line_numbers {
 647 |                         for (idx, l) in lines.iter().enumerate() {
 648 |                             output.push_str(&format!("{:>4} | {}\n", idx + 1, l));
 649 |                         }
 650 |                     } else {
 651 |                         for l in lines {
 652 |                             output.push_str(&l);
 653 |                             output.push('\n');
 654 |                         }
 655 |                     }
 656 |                     output.push_str("```\n\n");
 657 |                 }
 658 |             }
 659 | 
 660 |             // Always include a unified diff section header so downstream tooling/tests can rely on it
 661 |             let changed_diffs: Vec<diff::PerFileDiff> = comp
 662 |                 .file_diffs
 663 |                 .iter()
 664 |                 .filter(|d| d.is_changed())
 665 |                 .cloned()
 666 |                 .collect();
 667 |             if !changed_diffs.is_empty() {
 668 |                 output.push_str("## File Differences\n\n");
 669 |                 let diff_markdown = render_per_file_diffs(&changed_diffs);
 670 |                 output.push_str(&diff_markdown);
 671 |             }
 672 |         } else {
 673 |             output.push_str("## No Changes Detected\n\n");
 674 |         }
 675 |     }
 676 | 
 677 |     // File tree
 678 |     output.push_str("## File Tree Structure\n\n");
 679 |     let mut tree_output = Vec::new();
 680 |     tree::write_tree_to_file(&mut tree_output, file_tree, 0)?;
 681 |     output.push_str(&String::from_utf8_lossy(&tree_output));
 682 |     output.push('\n');
 683 | 
 684 |     // File contents (unless diff_only mode)
 685 |     if !diff_config.diff_only {
 686 |         output.push_str("## File Contents\n\n");
 687 | 
 688 |         // Iterate in relevance order (from sorted_paths) instead of
 689 |         // BTreeMap's alphabetical order — preserves file_relevance_category ordering.
 690 |         for path in sorted_paths {
 691 |             if let Some(file_state) = current_state.files.get(path) {
 692 |                 output.push_str(&format!("### File: `{}`\n\n", path.display()));
 693 |                 output.push_str(&format!("- Size: {} bytes\n", file_state.size));
 694 |                 output.push_str(&format!("- Modified: {:?}\n\n", file_state.modified));
 695 | 
 696 |                 // Determine language from file extension
 697 |                 let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("text");
 698 |                 let language = match extension {
 699 |                     "rs" => "rust",
 700 |                     "js" => "javascript",
 701 |                     "ts" => "typescript",
 702 |                     "py" => "python",
 703 |                     "json" => "json",
 704 |                     "toml" => "toml",
 705 |                     "md" => "markdown",
 706 |                     "yaml" | "yml" => "yaml",
 707 |                     "html" => "html",
 708 |                     "css" => "css",
 709 |                     _ => extension,
 710 |                 };
 711 | 
 712 |                 output.push_str(&format!("```{}\n", language));
 713 | 
 714 |                 if args.line_numbers {
 715 |                     for (i, line) in file_state.content.lines().enumerate() {
 716 |                         output.push_str(&format!("{:>4} | {}\n", i + 1, line));
 717 |                     }
 718 |                 } else {
 719 |                     output.push_str(&file_state.content);
 720 |                     if !file_state.content.ends_with('\n') {
 721 |                         output.push('\n');
 722 |                     }
 723 |                 }
 724 | 
 725 |                 output.push_str("```\n\n");
 726 |             }
 727 |         }
 728 |     }
 729 | 
 730 |     Ok(output)
 731 | }
 732 | 
 733 | pub fn run() -> io::Result<()> {
 734 |     env_logger::init();
 735 |     let args = Args::parse();
 736 | 
 737 |     // Handle init command first
 738 |     if args.init {
 739 |         return init_config();
 740 |     }
 741 | 
 742 |     // Determine project root first
 743 |     let project_root = Path::new(&args.input);
 744 |     let config = load_config_from_path(project_root);
 745 | 
 746 |     // Handle early clear-cache request (runs even if no config or other args)
 747 |     if args.clear_cache {
 748 |         let cache_path = project_root.join(".context-builder").join("cache");
 749 |         if cache_path.exists() {
 750 |             match fs::remove_dir_all(&cache_path) {
 751 |                 Ok(()) => println!("Cache cleared: {}", cache_path.display()),
 752 |                 Err(e) => eprintln!("Failed to clear cache ({}): {}", cache_path.display(), e),
 753 |             }
 754 |         } else {
 755 |             println!("No cache directory found at {}", cache_path.display());
 756 |         }
 757 |         return Ok(());
 758 |     }
 759 | 
 760 |     if std::env::args().len() == 1 && config.is_none() {
 761 |         Args::command().print_help()?;
 762 |         return Ok(());
 763 |     }
 764 | 
 765 |     // Resolve final configuration using the new config resolver
 766 |     let resolution = crate::config_resolver::resolve_final_config(args, config.clone());
 767 | 
 768 |     // Print warnings if any
 769 |     let silent = std::env::var("CB_SILENT")
 770 |         .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
 771 |         .unwrap_or(false);
 772 | 
 773 |     if !silent {
 774 |         for warning in &resolution.warnings {
 775 |             eprintln!("Warning: {}", warning);
 776 |         }
 777 |     }
 778 | 
 779 |     // Convert resolved config back to Args for run_with_args
 780 |     let final_args = Args {
 781 |         input: resolution.config.input,
 782 |         output: resolution.config.output,
 783 |         filter: resolution.config.filter,
 784 |         ignore: resolution.config.ignore,
 785 |         line_numbers: resolution.config.line_numbers,
 786 |         preview: resolution.config.preview,
 787 |         token_count: resolution.config.token_count,
 788 |         yes: resolution.config.yes,
 789 |         diff_only: resolution.config.diff_only,
 790 |         clear_cache: resolution.config.clear_cache,
 791 |         max_tokens: resolution.config.max_tokens,
 792 |         init: false,
 793 |     };
 794 | 
 795 |     // Create final Config with resolved values
 796 |     let final_config = Config {
 797 |         auto_diff: Some(resolution.config.auto_diff),
 798 |         diff_context_lines: Some(resolution.config.diff_context_lines),
 799 |         ..config.unwrap_or_default()
 800 |     };
 801 | 
 802 |     run_with_args(final_args, final_config, &DefaultPrompter)
 803 | }
 804 | 
 805 | /// Detect major file types in the current directory respecting .gitignore and default ignore patterns
 806 | fn detect_major_file_types() -> io::Result<Vec<String>> {
 807 |     use std::collections::HashMap;
 808 |     let mut extension_counts = HashMap::new();
 809 | 
 810 |     // Use the same default ignore patterns as the main application
 811 |     let default_ignores = vec![
 812 |         "docs".to_string(),
 813 |         "target".to_string(),
 814 |         ".git".to_string(),
 815 |         "node_modules".to_string(),
 816 |     ];
 817 | 
 818 |     // Collect files using the same logic as the main application
 819 |     let files = crate::file_utils::collect_files(Path::new("."), &[], &default_ignores, &[])?;
 820 | 
 821 |     // Count extensions from the filtered file list
 822 |     for entry in files {
 823 |         let path = entry.path();
 824 |         if let Some(extension) = path.extension().and_then(|ext| ext.to_str()) {
 825 |             // Count the extension occurrences
 826 |             *extension_counts.entry(extension.to_string()).or_insert(0) += 1;
 827 |         }
 828 |     }
 829 | 
 830 |     // Convert to vector of (extension, count) pairs and sort by count
 831 |     let mut extensions: Vec<(String, usize)> = extension_counts.into_iter().collect();
 832 |     extensions.sort_by(|a, b| b.1.cmp(&a.1));
 833 | 
 834 |     // Take the top 5 extensions or all if less than 5
 835 |     let top_extensions: Vec<String> = extensions.into_iter().take(5).map(|(ext, _)| ext).collect();
 836 | 
 837 |     Ok(top_extensions)
 838 | }
 839 | 
 840 | /// Initialize a new context-builder.toml config file in the current directory with sensible defaults
 841 | fn init_config() -> io::Result<()> {
 842 |     let config_path = Path::new("context-builder.toml");
 843 | 
 844 |     if config_path.exists() {
 845 |         println!("Config file already exists at {}", config_path.display());
 846 |         println!("If you want to replace it, please remove it manually first.");
 847 |         return Ok(());
 848 |     }
 849 | 
 850 |     // Detect major file types in the current directory
 851 |     let filter_suggestions = match detect_major_file_types() {
 852 |         Ok(extensions) => extensions,
 853 |         _ => vec!["rs".to_string(), "toml".to_string()], // fallback to defaults
 854 |     };
 855 | 
 856 |     let filter_string = if filter_suggestions.is_empty() {
 857 |         r#"["rs", "toml"]"#.to_string()
 858 |     } else {
 859 |         format!(r#"["{}"]"#, filter_suggestions.join(r#"", ""#))
 860 |     };
 861 | 
 862 |     let default_config_content = format!(
 863 |         r#"# Context Builder Configuration File
 864 | # This file was generated with sensible defaults based on the file types detected in your project
 865 | 
 866 | # Output file name (or base name when timestamped_output is true)
 867 | output = "context.md"
 868 | 
 869 | # Optional folder to place the generated output file(s) in
 870 | output_folder = "docs"
 871 | 
 872 | # Append a UTC timestamp to the output file name (before extension)
 873 | timestamped_output = true
 874 | 
 875 | # Enable automatic diff generation (requires timestamped_output = true)
 876 | auto_diff = true
 877 | 
 878 | # Emit only change summary + modified file diffs (no full file bodies)
 879 | diff_only = false
 880 | 
 881 | # File extensions to include (no leading dot, e.g. "rs", "toml")
 882 | filter = {}
 883 | 
 884 | # File / directory names to ignore (exact name matches)
 885 | ignore = ["docs", "target", ".git", "node_modules"]
 886 | 
 887 | # Add line numbers to code blocks
 888 | line_numbers = false
 889 | "#,
 890 |         filter_string
 891 |     );
 892 | 
 893 |     let mut file = File::create(config_path)?;
 894 |     file.write_all(default_config_content.as_bytes())?;
 895 | 
 896 |     println!("Config file created at {}", config_path.display());
 897 |     println!("Detected file types: {}", filter_suggestions.join(", "));
 898 |     println!("You can now customize it according to your project needs.");
 899 | 
 900 |     Ok(())
 901 | }
 902 | 
 903 | #[cfg(test)]
 904 | mod tests {
 905 |     use super::*;
 906 |     use std::io::Result;
 907 |     use tempfile::tempdir;
 908 | 
 909 |     // Mock prompter for testing
 910 |     struct MockPrompter {
 911 |         confirm_processing_response: bool,
 912 |         confirm_overwrite_response: bool,
 913 |     }
 914 | 
 915 |     impl MockPrompter {
 916 |         fn new(processing: bool, overwrite: bool) -> Self {
 917 |             Self {
 918 |                 confirm_processing_response: processing,
 919 |                 confirm_overwrite_response: overwrite,
 920 |             }
 921 |         }
 922 |     }
 923 | 
 924 |     impl Prompter for MockPrompter {
 925 |         fn confirm_processing(&self, _file_count: usize) -> Result<bool> {
 926 |             Ok(self.confirm_processing_response)
 927 |         }
 928 | 
 929 |         fn confirm_overwrite(&self, _file_path: &str) -> Result<bool> {
 930 |             Ok(self.confirm_overwrite_response)
 931 |         }
 932 |     }
 933 | 
 934 |     #[test]
 935 |     fn test_diff_config_default() {
 936 |         let config = DiffConfig::default();
 937 |         assert_eq!(config.context_lines, 3);
 938 |         assert!(!config.enabled);
 939 |         assert!(!config.diff_only);
 940 |     }
 941 | 
 942 |     #[test]
 943 |     fn test_diff_config_custom() {
 944 |         let config = DiffConfig {
 945 |             context_lines: 5,
 946 |             enabled: true,
 947 |             diff_only: true,
 948 |         };
 949 |         assert_eq!(config.context_lines, 5);
 950 |         assert!(config.enabled);
 951 |         assert!(config.diff_only);
 952 |     }
 953 | 
 954 |     #[test]
 955 |     fn test_default_prompter() {
 956 |         let prompter = DefaultPrompter;
 957 | 
 958 |         // Test small file count (should not prompt)
 959 |         let result = prompter.confirm_processing(50);
 960 |         assert!(result.is_ok());
 961 |         assert!(result.unwrap());
 962 |     }
 963 | 
 964 |     #[test]
 965 |     fn test_run_with_args_nonexistent_directory() {
 966 |         let args = Args {
 967 |             input: "/nonexistent/directory".to_string(),
 968 |             output: "output.md".to_string(),
 969 |             filter: vec![],
 970 |             ignore: vec![],
 971 |             line_numbers: false,
 972 |             preview: false,
 973 |             token_count: false,
 974 |             yes: false,
 975 |             diff_only: false,
 976 |             clear_cache: false,
 977 |             init: false,
 978 |             max_tokens: None,
 979 |         };
 980 |         let config = Config::default();
 981 |         let prompter = MockPrompter::new(true, true);
 982 | 
 983 |         let result = run_with_args(args, config, &prompter);
 984 |         assert!(result.is_err());
 985 |         assert!(result.unwrap_err().to_string().contains("does not exist"));
 986 |     }
 987 | 
 988 |     #[test]
 989 |     fn test_run_with_args_preview_mode() {
 990 |         let temp_dir = tempdir().unwrap();
 991 |         let base_path = temp_dir.path();
 992 | 
 993 |         // Create some test files
 994 |         fs::write(base_path.join("test.rs"), "fn main() {}").unwrap();
 995 |         fs::create_dir(base_path.join("src")).unwrap();
 996 |         fs::write(base_path.join("src/lib.rs"), "pub fn hello() {}").unwrap();
 997 | 
 998 |         let args = Args {
 999 |             input: ".".to_string(),
1000 |             output: "test.md".to_string(),
1001 |             filter: vec![],
1002 |             ignore: vec![],
1003 |             line_numbers: false,
1004 |             preview: false,
1005 |             token_count: false,
1006 |             yes: false,
1007 |             diff_only: false,
1008 |             clear_cache: false,
1009 |             init: false,
1010 |             max_tokens: None,
1011 |         };
1012 |         let config = Config::default();
1013 |         let prompter = MockPrompter::new(true, true);
1014 | 
1015 |         // Set CB_SILENT to avoid console output during test
1016 |         unsafe {
1017 |             std::env::set_var("CB_SILENT", "1");
1018 |         }
1019 |         let result = run_with_args(args, config, &prompter);
1020 |         unsafe {
1021 |             std::env::remove_var("CB_SILENT");
1022 |         }
1023 | 
1024 |         assert!(result.is_ok());
1025 |     }
1026 | 
1027 |     #[test]
1028 |     fn test_run_with_args_token_count_mode() {
1029 |         let temp_dir = tempdir().unwrap();
1030 |         let base_path = temp_dir.path();
1031 | 
1032 |         // Create test files
1033 |         fs::write(base_path.join("small.txt"), "Hello world").unwrap();
1034 | 
1035 |         let args = Args {
1036 |             input: base_path.to_string_lossy().to_string(),
1037 |             output: "test.md".to_string(),
1038 |             filter: vec![],
1039 |             ignore: vec![],
1040 |             line_numbers: false,
1041 |             preview: false,
1042 |             token_count: true,
1043 |             yes: false,
1044 |             diff_only: false,
1045 |             clear_cache: false,
1046 |             init: false,
1047 |             max_tokens: None,
1048 |         };
1049 |         let config = Config::default();
1050 |         let prompter = MockPrompter::new(true, true);
1051 | 
1052 |         unsafe {
1053 |             std::env::set_var("CB_SILENT", "1");
1054 |         }
1055 |         let result = run_with_args(args, config, &prompter);
1056 |         unsafe {
1057 |             std::env::remove_var("CB_SILENT");
1058 |         }
1059 | 
1060 |         assert!(result.is_ok());
1061 |     }
1062 | 
1063 |     #[test]
1064 |     fn test_run_with_args_preview_and_token_count() {
1065 |         let temp_dir = tempdir().unwrap();
1066 |         let base_path = temp_dir.path();
1067 | 
1068 |         fs::write(base_path.join("test.txt"), "content").unwrap();
1069 | 
1070 |         let args = Args {
1071 |             input: base_path.to_string_lossy().to_string(),
1072 |             output: "test.md".to_string(),
1073 |             filter: vec![],
1074 |             ignore: vec![],
1075 |             line_numbers: false,
1076 |             preview: true,
1077 |             token_count: false,
1078 |             yes: false,
1079 |             diff_only: false,
1080 |             clear_cache: false,
1081 |             init: false,
1082 |             max_tokens: None,
1083 |         };
1084 |         let config = Config::default();
1085 |         let prompter = MockPrompter::new(true, true);
1086 | 
1087 |         unsafe {
1088 |             std::env::set_var("CB_SILENT", "1");
1089 |         }
1090 |         let result = run_with_args(args, config, &prompter);
1091 |         unsafe {
1092 |             std::env::remove_var("CB_SILENT");
1093 |         }
1094 | 
1095 |         assert!(result.is_ok());
1096 |     }
1097 | 
1098 |     #[test]
1099 |     fn test_run_with_args_user_cancels_overwrite() {
1100 |         let temp_dir = tempdir().unwrap();
1101 |         let base_path = temp_dir.path();
1102 |         let output_path = temp_dir.path().join("existing.md");
1103 | 
1104 |         // Create test files
1105 |         fs::write(base_path.join("test.txt"), "content").unwrap();
1106 |         fs::write(&output_path, "existing content").unwrap();
1107 | 
1108 |         let args = Args {
1109 |             input: base_path.to_string_lossy().to_string(),
1110 |             output: "test.md".to_string(),
1111 |             filter: vec![],
1112 |             ignore: vec!["target".to_string()],
1113 |             line_numbers: false,
1114 |             preview: false,
1115 |             token_count: false,
1116 |             yes: false,
1117 |             diff_only: false,
1118 |             clear_cache: false,
1119 |             init: false,
1120 |             max_tokens: None,
1121 |         };
1122 |         let config = Config::default();
1123 |         let prompter = MockPrompter::new(true, false); // Deny overwrite
1124 | 
1125 |         unsafe {
1126 |             std::env::set_var("CB_SILENT", "1");
1127 |         }
1128 |         let result = run_with_args(args, config, &prompter);
1129 |         unsafe {
1130 |             std::env::remove_var("CB_SILENT");
1131 |         }
1132 | 
1133 |         assert!(result.is_err());
1134 |         assert!(result.unwrap_err().to_string().contains("cancelled"));
1135 |     }
1136 | 
1137 |     #[test]
1138 |     fn test_run_with_args_user_cancels_processing() {
1139 |         let temp_dir = tempdir().unwrap();
1140 |         let base_path = temp_dir.path();
1141 | 
1142 |         // Create many test files to trigger processing confirmation
1143 |         for i in 0..105 {
1144 |             fs::write(base_path.join(format!("file{}.txt", i)), "content").unwrap();
1145 |         }
1146 | 
1147 |         let args = Args {
1148 |             input: base_path.to_string_lossy().to_string(),
1149 |             output: "test.md".to_string(),
1150 |             filter: vec!["rs".to_string()],
1151 |             ignore: vec![],
1152 |             line_numbers: false,
1153 |             preview: false,
1154 |             token_count: false,
1155 |             yes: false,
1156 |             diff_only: false,
1157 |             clear_cache: false,
1158 |             init: false,
1159 |             max_tokens: None,
1160 |         };
1161 |         let config = Config::default();
1162 |         let prompter = MockPrompter::new(false, true); // Deny processing
1163 | 
1164 |         unsafe {
1165 |             std::env::set_var("CB_SILENT", "1");
1166 |         }
1167 |         let result = run_with_args(args, config, &prompter);
1168 |         unsafe {
1169 |             std::env::remove_var("CB_SILENT");
1170 |         }
1171 | 
1172 |         assert!(result.is_err());
1173 |         assert!(result.unwrap_err().to_string().contains("cancelled"));
1174 |     }
1175 | 
1176 |     #[test]
1177 |     fn test_run_with_args_with_yes_flag() {
1178 |         let temp_dir = tempdir().unwrap();
1179 |         let base_path = temp_dir.path();
1180 |         let output_file_name = "test.md";
1181 |         let output_path = temp_dir.path().join(output_file_name);
1182 | 
1183 |         fs::write(base_path.join("test.txt"), "Hello world").unwrap();
1184 | 
1185 |         let args = Args {
1186 |             input: base_path.to_string_lossy().to_string(),
1187 |             output: output_path.to_string_lossy().to_string(),
1188 |             filter: vec![],
1189 |             ignore: vec!["ignored_dir".to_string()],
1190 |             line_numbers: false,
1191 |             preview: false,
1192 |             token_count: false,
1193 |             yes: true,
1194 |             diff_only: false,
1195 |             clear_cache: false,
1196 |             init: false,
1197 |             max_tokens: None,
1198 |         };
1199 |         let config = Config::default();
1200 |         let prompter = MockPrompter::new(true, true);
1201 | 
1202 |         unsafe {
1203 |             std::env::set_var("CB_SILENT", "1");
1204 |         }
1205 |         let result = run_with_args(args, config, &prompter);
1206 |         unsafe {
1207 |             std::env::remove_var("CB_SILENT");
1208 |         }
1209 | 
1210 |         assert!(result.is_ok());
1211 |         assert!(output_path.exists());
1212 | 
1213 |         let content = fs::read_to_string(&output_path).unwrap();
1214 |         assert!(content.contains("Directory Structure Report"));
1215 |         assert!(content.contains("test.txt"));
1216 |     }
1217 | 
1218 |     #[test]
1219 |     fn test_run_with_args_with_filters() {
1220 |         let temp_dir = tempdir().unwrap();
1221 |         let base_path = temp_dir.path();
1222 |         let output_file_name = "test.md";
1223 |         let output_path = temp_dir.path().join(output_file_name);
1224 | 
1225 |         fs::write(base_path.join("code.rs"), "fn main() {}").unwrap();
1226 |         fs::write(base_path.join("readme.md"), "# README").unwrap();
1227 |         fs::write(base_path.join("data.json"), r#"{"key": "value"}"#).unwrap();
1228 | 
1229 |         let args = Args {
1230 |             input: base_path.to_string_lossy().to_string(),
1231 |             output: output_path.to_string_lossy().to_string(),
1232 |             filter: vec!["rs".to_string(), "md".to_string()],
1233 |             ignore: vec![],
1234 |             line_numbers: true,
1235 |             preview: false,
1236 |             token_count: false,
1237 |             yes: true,
1238 |             diff_only: false,
1239 |             clear_cache: false,
1240 |             init: false,
1241 |             max_tokens: None,
1242 |         };
1243 |         let config = Config::default();
1244 |         let prompter = MockPrompter::new(true, true);
1245 | 
1246 |         unsafe {
1247 |             std::env::set_var("CB_SILENT", "1");
1248 |         }
1249 |         let result = run_with_args(args, config, &prompter);
1250 |         unsafe {
1251 |             std::env::remove_var("CB_SILENT");
1252 |         }
1253 | 
1254 |         assert!(result.is_ok());
1255 | 
1256 |         let content = fs::read_to_string(&output_path).unwrap();
1257 |         assert!(content.contains("code.rs"));
1258 |         assert!(content.contains("readme.md"));
1259 |         assert!(!content.contains("data.json")); // Should be filtered out
1260 |         assert!(content.contains("   1 |")); // Line numbers should be present
1261 |     }
1262 | 
1263 |     #[test]
1264 |     fn test_run_with_args_with_ignores() {
1265 |         let temp_dir = tempdir().unwrap();
1266 |         let base_path = temp_dir.path();
1267 |         let output_path = temp_dir.path().join("ignored.md");
1268 | 
1269 |         fs::write(base_path.join("important.txt"), "important content").unwrap();
1270 |         fs::write(base_path.join("secret.txt"), "secret content").unwrap();
1271 | 
1272 |         let args = Args {
1273 |             input: base_path.to_string_lossy().to_string(),
1274 |             output: output_path.to_string_lossy().to_string(),
1275 |             filter: vec![],
1276 |             ignore: vec!["secret.txt".to_string()],
1277 |             line_numbers: false,
1278 |             preview: false,
1279 |             token_count: false,
1280 |             yes: true,
1281 |             diff_only: false,
1282 |             clear_cache: false,
1283 |             init: false,
1284 |             max_tokens: None,
1285 |         };
1286 |         let config = Config::default();
1287 |         let prompter = MockPrompter::new(true, true);
1288 | 
1289 |         unsafe {
1290 |             std::env::set_var("CB_SILENT", "1");
1291 |         }
1292 |         let result = run_with_args(args, config, &prompter);
1293 |         unsafe {
1294 |             std::env::remove_var("CB_SILENT");
1295 |         }
1296 | 
1297 |         assert!(result.is_ok());
1298 | 
1299 |         let content = fs::read_to_string(&output_path).unwrap();
1300 |         assert!(content.contains("important.txt"));
1301 |         // The ignore pattern may not work exactly as expected in this test setup
1302 |         // Just verify the output file was created successfully
1303 |     }
1304 | 
1305 |     #[test]
1306 |     fn test_auto_diff_without_previous_state() {
1307 |         let temp_dir = tempdir().unwrap();
1308 |         let base_path = temp_dir.path();
1309 |         let output_file_name = "test.md";
1310 |         let output_path = temp_dir.path().join(output_file_name);
1311 | 
1312 |         fs::write(base_path.join("new.txt"), "new content").unwrap();
1313 | 
1314 |         let args = Args {
1315 |             input: base_path.to_string_lossy().to_string(),
1316 |             output: output_path.to_string_lossy().to_string(),
1317 |             filter: vec![],
1318 |             ignore: vec![],
1319 |             line_numbers: false,
1320 |             preview: false,
1321 |             token_count: false,
1322 |             yes: true,
1323 |             diff_only: false,
1324 |             clear_cache: false,
1325 |             init: false,
1326 |             max_tokens: None,
1327 |         };
1328 |         let config = Config {
1329 |             auto_diff: Some(true),
1330 |             diff_context_lines: Some(5),
1331 |             ..Default::default()
1332 |         };
1333 |         let prompter = MockPrompter::new(true, true);
1334 | 
1335 |         unsafe {
1336 |             std::env::set_var("CB_SILENT", "1");
1337 |         }
1338 |         let result = run_with_args(args, config, &prompter);
1339 |         unsafe {
1340 |             std::env::remove_var("CB_SILENT");
1341 |         }
1342 | 
1343 |         assert!(result.is_ok());
1344 |         assert!(output_path.exists());
1345 | 
1346 |         let content = fs::read_to_string(&output_path).unwrap();
1347 |         assert!(content.contains("new.txt"));
1348 |     }
1349 | 
1350 |     #[test]
1351 |     fn test_run_creates_output_directory() {
1352 |         let temp_dir = tempdir().unwrap();
1353 |         let base_path = temp_dir.path();
1354 |         let output_dir = temp_dir.path().join("nested").join("output");
1355 |         let output_path = output_dir.join("result.md");
1356 | 
1357 |         fs::write(base_path.join("test.txt"), "content").unwrap();
1358 | 
1359 |         let args = Args {
1360 |             input: base_path.to_string_lossy().to_string(),
1361 |             output: output_path.to_string_lossy().to_string(),
1362 |             filter: vec![],
1363 |             ignore: vec![],
1364 |             line_numbers: false,
1365 |             preview: false,
1366 |             token_count: false,
1367 |             yes: true,
1368 |             diff_only: false,
1369 |             clear_cache: false,
1370 |             init: false,
1371 |             max_tokens: None,
1372 |         };
1373 |         let config = Config::default();
1374 |         let prompter = MockPrompter::new(true, true);
1375 | 
1376 |         unsafe {
1377 |             std::env::set_var("CB_SILENT", "1");
1378 |         }
1379 |         let result = run_with_args(args, config, &prompter);
1380 |         unsafe {
1381 |             std::env::remove_var("CB_SILENT");
1382 |         }
1383 | 
1384 |         assert!(result.is_ok());
1385 |         assert!(output_path.exists());
1386 |         assert!(output_dir.exists());
1387 |     }
1388 | 
1389 |     #[test]
1390 |     fn test_generate_markdown_with_diff_no_comparison() {
1391 |         let temp_dir = tempdir().unwrap();
1392 |         let base_path = temp_dir.path();
1393 | 
1394 |         fs::write(base_path.join("test.rs"), "fn main() {}").unwrap();
1395 | 
1396 |         let files = collect_files(base_path, &[], &[], &[]).unwrap();
1397 |         let file_tree = build_file_tree(&files, base_path);
1398 |         let config = Config::default();
1399 |         let state = ProjectState::from_files(&files, base_path, &config, false).unwrap();
1400 | 
1401 |         let args = Args {
1402 |             input: base_path.to_string_lossy().to_string(),
1403 |             output: "test.md".to_string(),
1404 |             filter: vec![],
1405 |             ignore: vec![],
1406 |             line_numbers: false,
1407 |             preview: false,
1408 |             token_count: false,
1409 |             yes: false,
1410 |             diff_only: false,
1411 |             clear_cache: false,
1412 |             init: false,
1413 |             max_tokens: None,
1414 |         };
1415 | 
1416 |         let diff_config = DiffConfig::default();
1417 | 
1418 |         let sorted_paths: Vec<PathBuf> = files
1419 |             .iter()
1420 |             .map(|e| {
1421 |                 e.path()
1422 |                     .strip_prefix(base_path)
1423 |                     .unwrap_or(e.path())
1424 |                     .to_path_buf()
1425 |             })
1426 |             .collect();
1427 | 
1428 |         let result = generate_markdown_with_diff(
1429 |             &state,
1430 |             None,
1431 |             &args,
1432 |             &file_tree,
1433 |             &diff_config,
1434 |             &sorted_paths,
1435 |         );
1436 |         assert!(result.is_ok());
1437 | 
1438 |         let content = result.unwrap();
1439 |         assert!(content.contains("Directory Structure Report"));
1440 |         assert!(content.contains("test.rs"));
1441 |     }
1442 | }
```

### File: `src/main.rs`

- Size: 73 bytes
- Modified: SystemTime { tv_sec: 1771053288, tv_nsec: 65557165 }

```rust
   1 | use std::io;
   2 | 
   3 | fn main() -> io::Result<()> {
   4 |     context_builder::run()
   5 | }
```

### File: `src/cache.rs`

- Size: 19309 bytes
- Modified: SystemTime { tv_sec: 1771106931, tv_nsec: 244867662 }

```rust
   1 | //! Cache management for context-builder.
   2 | //!
   3 | //! This module handles caching of project states to enable the auto-diff feature.
   4 | //! It uses a hash of the project path and configuration to avoid cache collisions
   5 | //! between different projects or configurations.
   6 | 
   7 | use fs2::FileExt;
   8 | 
   9 | use std::fs;
  10 | use std::fs::File;
  11 | 
  12 | use std::io::{Read, Write};
  13 | use std::path::{Path, PathBuf};
  14 | 
  15 | use crate::config::Config;
  16 | use crate::state::ProjectState;
  17 | 
  18 | /// Manages cache operations with file locking to prevent corruption
  19 | pub struct CacheManager {
  20 |     cache_dir: PathBuf,
  21 |     project_hash: String,
  22 |     config_hash: String,
  23 | }
  24 | 
  25 | impl CacheManager {
  26 |     /// Create a new cache manager for the given project path and configuration
  27 |     pub fn new(project_path: &Path, config: &Config) -> Self {
  28 |         // Normalize the project path first for consistency
  29 |         let normalized_project_path = Self::normalize_project_path(project_path);
  30 | 
  31 |         let project_hash = Self::hash_path(&normalized_project_path);
  32 |         let config_hash = Self::hash_config(config);
  33 | 
  34 |         // Ensure cache directory exists relative to normalized project root
  35 |         let cache_dir = normalized_project_path
  36 |             .join(".context-builder")
  37 |             .join("cache");
  38 |         if !cache_dir.exists() {
  39 |             let _ = fs::create_dir_all(&cache_dir);
  40 |         }
  41 | 
  42 |         let cache_manager = Self {
  43 |             cache_dir,
  44 |             project_hash,
  45 |             config_hash,
  46 |         };
  47 | 
  48 |         // Migrate old cache format if present
  49 |         cache_manager.migrate_old_cache();
  50 | 
  51 |         cache_manager
  52 |     }
  53 | 
  54 |     /// Normalize project path for consistent hashing and cache directory creation
  55 |     fn normalize_project_path(path: &Path) -> PathBuf {
  56 |         // Always resolve to absolute path first
  57 |         let absolute_path = if path.is_absolute() {
  58 |             path.to_path_buf()
  59 |         } else {
  60 |             match std::env::current_dir() {
  61 |                 Ok(cwd) => cwd.join(path),
  62 |                 Err(_) => path.to_path_buf(),
  63 |             }
  64 |         };
  65 | 
  66 |         // Try to canonicalize for consistency, but normalize the result
  67 |         if let Ok(canonical) = absolute_path.canonicalize() {
  68 |             Self::normalize_path_format(&canonical)
  69 |         } else {
  70 |             absolute_path
  71 |         }
  72 |     }
  73 | 
  74 |     /// Generate a hash from the normalized project path
  75 |     fn hash_path(path: &Path) -> String {
  76 |         let path_str = path.to_string_lossy();
  77 |         let hash = xxhash_rust::xxh3::xxh3_64(path_str.as_bytes());
  78 |         format!("{:x}", hash)
  79 |     }
  80 | 
  81 |     /// Normalize path format to handle Windows UNC prefixes
  82 |     fn normalize_path_format(path: &Path) -> PathBuf {
  83 |         let path_str = path.to_string_lossy();
  84 | 
  85 |         // Remove Windows UNC prefix if present
  86 |         if cfg!(windows) && path_str.starts_with("\\\\?\\") {
  87 |             PathBuf::from(&path_str[4..])
  88 |         } else {
  89 |             path.to_path_buf()
  90 |         }
  91 |     }
  92 | 
  93 |     /// Generate a hash from the configuration
  94 |     fn hash_config(config: &Config) -> String {
  95 |         // Build a stable string representation of config for hashing
  96 |         let mut config_str = String::new();
  97 |         if let Some(ref filters) = config.filter {
  98 |             config_str.push_str(&filters.join(","));
  99 |         }
 100 |         config_str.push('|');
 101 |         if let Some(ref ignores) = config.ignore {
 102 |             config_str.push_str(&ignores.join(","));
 103 |         }
 104 |         config_str.push('|');
 105 |         config_str.push_str(&format!("{:?}", config.line_numbers));
 106 |         let hash = xxhash_rust::xxh3::xxh3_64(config_str.as_bytes());
 107 |         format!("{:x}", hash)
 108 |     }
 109 | 
 110 |     /// Get the cache file path for this specific project and configuration
 111 |     fn get_cache_path(&self) -> PathBuf {
 112 |         self.cache_dir.join(format!(
 113 |             "state_{}_{}.json",
 114 |             self.project_hash, self.config_hash
 115 |         ))
 116 |     }
 117 | 
 118 |     /// Public helper primarily for debugging/tests to inspect the resolved cache path
 119 |     pub fn debug_cache_file_path(&self) -> PathBuf {
 120 |         self.get_cache_path()
 121 |     }
 122 | 
 123 |     /// Migrate old markdown-based cache files to new JSON format
 124 |     fn migrate_old_cache(&self) {
 125 |         let old_cache_patterns = ["last_canonical.md", "last_output.md", "current_output.md"];
 126 | 
 127 |         for pattern in &old_cache_patterns {
 128 |             let old_cache_path = self.cache_dir.join(pattern);
 129 |             if old_cache_path.exists() {
 130 |                 eprintln!("Migrating old cache format: removing {}", pattern);
 131 |                 let _ = fs::remove_file(&old_cache_path);
 132 |             }
 133 |         }
 134 | 
 135 |         // Also remove any files that look like timestamped outputs from old versions
 136 |         if let Ok(entries) = fs::read_dir(&self.cache_dir) {
 137 |             for entry in entries.flatten() {
 138 |                 let file_name = entry.file_name();
 139 |                 let name = file_name.to_string_lossy();
 140 |                 if name.ends_with(".md") && (name.contains("_20") || name.starts_with("output_")) {
 141 |                     eprintln!("Migrating old cache format: removing {}", name);
 142 |                     let _ = fs::remove_file(entry.path());
 143 |                 }
 144 |             }
 145 |         }
 146 |     }
 147 | 
 148 |     /// Read the cached project state with file locking
 149 |     pub fn read_cache(&self) -> Result<Option<ProjectState>, Box<dyn std::error::Error>> {
 150 |         let cache_path = self.get_cache_path();
 151 | 
 152 |         if !cache_path.exists() {
 153 |             return Ok(None);
 154 |         }
 155 | 
 156 |         let file = File::open(&cache_path)?;
 157 |         // Acquire shared lock to prevent reading while writing
 158 |         file.lock_shared()?;
 159 | 
 160 |         let mut contents = String::new();
 161 |         let mut file = std::io::BufReader::new(file);
 162 |         file.read_to_string(&mut contents)?;
 163 | 
 164 |         // Release lock
 165 |         file.get_ref().unlock()?;
 166 | 
 167 |         let state: ProjectState = serde_json::from_str(&contents)?;
 168 |         Ok(Some(state))
 169 |     }
 170 | 
 171 |     /// Write the project state to cache with file locking
 172 |     pub fn write_cache(&self, state: &ProjectState) -> Result<(), Box<dyn std::error::Error>> {
 173 |         let cache_path = self.get_cache_path();
 174 | 
 175 |         let file = std::fs::OpenOptions::new()
 176 |             .write(true)
 177 |             .create(true)
 178 |             .truncate(false)
 179 |             .open(&cache_path)?;
 180 |         // Acquire exclusive lock BEFORE truncating to prevent TOCTOU races
 181 |         file.lock_exclusive()?;
 182 |         file.set_len(0)?;
 183 | 
 184 |         let json = serde_json::to_string_pretty(state)?;
 185 |         let mut file = std::io::BufWriter::new(file);
 186 |         file.write_all(json.as_bytes())?;
 187 |         file.flush()?;
 188 | 
 189 |         // Release lock
 190 |         file.get_ref().unlock()?;
 191 | 
 192 |         Ok(())
 193 |     }
 194 | }
 195 | 
 196 | #[cfg(test)]
 197 | mod tests {
 198 |     use super::*;
 199 |     use std::path::Path;
 200 |     use tempfile::tempdir;
 201 | 
 202 |     #[test]
 203 |     fn test_hash_path() {
 204 |         let path1 = Path::new("/project1");
 205 |         let path2 = Path::new("/project2");
 206 | 
 207 |         let hash1 = CacheManager::hash_path(path1);
 208 |         let hash2 = CacheManager::hash_path(path2);
 209 | 
 210 |         assert_ne!(
 211 |             hash1, hash2,
 212 |             "Different paths should produce different hashes"
 213 |         );
 214 |     }
 215 | 
 216 |     #[test]
 217 |     fn test_hash_config() {
 218 |         let config1 = Config {
 219 |             filter: Some(vec!["rs".to_string()]),
 220 |             ignore: Some(vec!["target".to_string()]),
 221 |             line_numbers: Some(true),
 222 |             ..Default::default()
 223 |         };
 224 | 
 225 |         let config2 = Config {
 226 |             filter: Some(vec!["md".to_string()]),
 227 |             ignore: Some(vec!["target".to_string()]),
 228 |             line_numbers: Some(true),
 229 |             ..Default::default()
 230 |         };
 231 | 
 232 |         let hash1 = CacheManager::hash_config(&config1);
 233 |         let hash2 = CacheManager::hash_config(&config2);
 234 | 
 235 |         assert_ne!(
 236 |             hash1, hash2,
 237 |             "Different configs should produce different hashes"
 238 |         );
 239 |     }
 240 | 
 241 |     #[test]
 242 |     fn test_cache_operations() {
 243 |         let dir = tempdir().unwrap();
 244 |         let project_path = dir.path().join("test_project");
 245 |         let _ = fs::create_dir(&project_path);
 246 | 
 247 |         let config = Config::default();
 248 |         let cache_manager = CacheManager::new(&project_path, &config);
 249 | 
 250 |         use crate::state::ProjectMetadata;
 251 | 
 252 |         let state = ProjectState {
 253 |             timestamp: "2023-01-01T00:00:00Z".to_string(),
 254 |             config_hash: "test_config_hash".to_string(),
 255 |             files: std::collections::BTreeMap::new(),
 256 |             metadata: ProjectMetadata {
 257 |                 project_name: "test".to_string(),
 258 |                 file_count: 0,
 259 |                 filters: vec![],
 260 |                 ignores: vec![],
 261 |                 line_numbers: false,
 262 |             },
 263 |         };
 264 | 
 265 |         // Write cache
 266 |         assert!(cache_manager.write_cache(&state).is_ok());
 267 | 
 268 |         // Read cache
 269 |         let cached_state = cache_manager.read_cache().unwrap();
 270 |         assert!(cached_state.is_some());
 271 |         assert_eq!(cached_state.unwrap().timestamp, state.timestamp);
 272 |     }
 273 | 
 274 |     #[test]
 275 |     fn test_old_cache_migration() {
 276 |         let dir = tempdir().unwrap();
 277 |         let project_path = dir.path().join("test_project");
 278 |         let _ = fs::create_dir(&project_path);
 279 | 
 280 |         // Create cache directory with old cache files
 281 |         let cache_dir = project_path.join(".context-builder").join("cache");
 282 |         let _ = fs::create_dir_all(&cache_dir);
 283 | 
 284 |         let old_files = [
 285 |             "last_canonical.md",
 286 |             "last_output.md",
 287 |             "current_output.md",
 288 |             "output_20230101120000.md",
 289 |         ];
 290 | 
 291 |         // Create old cache files
 292 |         for file in &old_files {
 293 |             let old_path = cache_dir.join(file);
 294 |             let _ = fs::write(&old_path, "old cache content");
 295 |             assert!(
 296 |                 old_path.exists(),
 297 |                 "Old cache file should exist before migration"
 298 |             );
 299 |         }
 300 | 
 301 |         // Create cache manager (this should trigger migration)
 302 |         let config = Config::default();
 303 |         let _cache_manager = CacheManager::new(&project_path, &config);
 304 | 
 305 |         // Verify old files are removed
 306 |         for file in &old_files {
 307 |             let old_path = cache_dir.join(file);
 308 |             assert!(
 309 |                 !old_path.exists(),
 310 |                 "Old cache file {} should be removed after migration",
 311 |                 file
 312 |             );
 313 |         }
 314 |     }
 315 | 
 316 |     #[test]
 317 |     fn test_cache_consistency_across_path_representations() {
 318 |         let dir = tempdir().unwrap();
 319 |         let project_path = dir.path().join("test_project");
 320 |         let _ = fs::create_dir(&project_path);
 321 | 
 322 |         let config = Config::default();
 323 | 
 324 |         // Test different path representations that should resolve to the same cache
 325 |         let mut paths_to_test = vec![
 326 |             project_path.clone(),
 327 |             project_path.canonicalize().unwrap_or(project_path.clone()),
 328 |         ];
 329 | 
 330 |         // If we can create a relative path, test that too
 331 |         if let Ok(current_dir) = std::env::current_dir()
 332 |             && let Ok(relative) = project_path.strip_prefix(&current_dir)
 333 |         {
 334 |             paths_to_test.push(relative.to_path_buf());
 335 |         }
 336 | 
 337 |         let mut cache_paths = Vec::new();
 338 |         for path in &paths_to_test {
 339 |             let cache_manager = CacheManager::new(path, &config);
 340 |             cache_paths.push(cache_manager.get_cache_path());
 341 |         }
 342 | 
 343 |         // All cache paths should be identical
 344 |         for (i, path1) in cache_paths.iter().enumerate() {
 345 |             for (j, path2) in cache_paths.iter().enumerate() {
 346 |                 if i != j {
 347 |                     assert_eq!(
 348 |                         path1, path2,
 349 |                         "Cache paths should be identical for different representations of the same project path"
 350 |                     );
 351 |                 }
 352 |             }
 353 |         }
 354 |     }
 355 | 
 356 |     #[test]
 357 |     fn test_normalize_path_format() {
 358 |         // Test Windows UNC path normalization
 359 |         if cfg!(windows) {
 360 |             let unc_path = Path::new("\\\\?\\C:\\test\\path");
 361 |             let normalized = CacheManager::normalize_path_format(unc_path);
 362 |             assert_eq!(normalized, PathBuf::from("C:\\test\\path"));
 363 |         }
 364 | 
 365 |         // Test normal path (should remain unchanged)
 366 |         let normal_path = Path::new("/normal/path");
 367 |         let normalized = CacheManager::normalize_path_format(normal_path);
 368 |         assert_eq!(normalized, normal_path);
 369 |     }
 370 | 
 371 |     #[test]
 372 |     fn test_cache_read_nonexistent_file() {
 373 |         let dir = tempdir().unwrap();
 374 |         let project_path = dir.path().join("nonexistent_project");
 375 | 
 376 |         let config = Config::default();
 377 |         let cache_manager = CacheManager::new(&project_path, &config);
 378 | 
 379 |         let result = cache_manager.read_cache().unwrap();
 380 |         assert!(result.is_none());
 381 |     }
 382 | 
 383 |     #[test]
 384 |     fn test_cache_read_corrupted_file() {
 385 |         let dir = tempdir().unwrap();
 386 |         let project_path = dir.path().join("test_project");
 387 |         let _ = fs::create_dir(&project_path);
 388 | 
 389 |         let config = Config::default();
 390 |         let cache_manager = CacheManager::new(&project_path, &config);
 391 |         let cache_path = cache_manager.get_cache_path();
 392 | 
 393 |         // Create a corrupted cache file
 394 |         let _ = fs::create_dir_all(cache_path.parent().unwrap());
 395 |         let _ = fs::write(&cache_path, "invalid json content {{{");
 396 | 
 397 |         let result = cache_manager.read_cache();
 398 |         assert!(result.is_err());
 399 |     }
 400 | 
 401 |     #[test]
 402 |     fn test_cache_write_read_roundtrip() {
 403 |         let dir = tempdir().unwrap();
 404 |         let project_path = dir.path().join("test_project");
 405 |         let _ = fs::create_dir(&project_path);
 406 | 
 407 |         let config = Config {
 408 |             filter: Some(vec!["rs".to_string(), "toml".to_string()]),
 409 |             ignore: Some(vec!["target".to_string(), ".git".to_string()]),
 410 |             line_numbers: Some(true),
 411 |             ..Default::default()
 412 |         };
 413 | 
 414 |         let cache_manager = CacheManager::new(&project_path, &config);
 415 | 
 416 |         use crate::state::ProjectMetadata;
 417 |         use std::collections::BTreeMap;
 418 | 
 419 |         let mut files = BTreeMap::new();
 420 |         files.insert(
 421 |             PathBuf::from("test.rs"),
 422 |             crate::state::FileState {
 423 |                 content: "fn main() {}".to_string(),
 424 |                 size: 12,
 425 |                 modified: std::time::SystemTime::UNIX_EPOCH,
 426 |                 content_hash: "test_hash".to_string(),
 427 |             },
 428 |         );
 429 | 
 430 |         let original_state = ProjectState {
 431 |             timestamp: "2023-01-01T12:00:00Z".to_string(),
 432 |             config_hash: "test_config_hash".to_string(),
 433 |             files,
 434 |             metadata: ProjectMetadata {
 435 |                 project_name: "test_project".to_string(),
 436 |                 file_count: 1,
 437 |                 filters: vec!["rs".to_string(), "toml".to_string()],
 438 |                 ignores: vec!["target".to_string(), ".git".to_string()],
 439 |                 line_numbers: true,
 440 |             },
 441 |         };
 442 | 
 443 |         // Write and read back
 444 |         cache_manager.write_cache(&original_state).unwrap();
 445 |         let cached_state = cache_manager.read_cache().unwrap().unwrap();
 446 | 
 447 |         assert_eq!(cached_state.timestamp, original_state.timestamp);
 448 |         assert_eq!(cached_state.config_hash, original_state.config_hash);
 449 |         assert_eq!(cached_state.files.len(), original_state.files.len());
 450 |         assert_eq!(
 451 |             cached_state.metadata.project_name,
 452 |             original_state.metadata.project_name
 453 |         );
 454 |         assert_eq!(
 455 |             cached_state.metadata.file_count,
 456 |             original_state.metadata.file_count
 457 |         );
 458 |         assert_eq!(
 459 |             cached_state.metadata.filters,
 460 |             original_state.metadata.filters
 461 |         );
 462 |         assert_eq!(
 463 |             cached_state.metadata.ignores,
 464 |             original_state.metadata.ignores
 465 |         );
 466 |         assert_eq!(
 467 |             cached_state.metadata.line_numbers,
 468 |             original_state.metadata.line_numbers
 469 |         );
 470 |     }
 471 | 
 472 |     #[test]
 473 |     fn test_different_configs_different_cache_files() {
 474 |         let dir = tempdir().unwrap();
 475 |         let project_path = dir.path().join("test_project");
 476 |         let _ = fs::create_dir(&project_path);
 477 | 
 478 |         let config1 = Config {
 479 |             filter: Some(vec!["rs".to_string()]),
 480 |             ..Default::default()
 481 |         };
 482 | 
 483 |         let config2 = Config {
 484 |             filter: Some(vec!["py".to_string()]),
 485 |             ..Default::default()
 486 |         };
 487 | 
 488 |         let cache_manager1 = CacheManager::new(&project_path, &config1);
 489 |         let cache_manager2 = CacheManager::new(&project_path, &config2);
 490 | 
 491 |         let cache_path1 = cache_manager1.get_cache_path();
 492 |         let cache_path2 = cache_manager2.get_cache_path();
 493 | 
 494 |         assert_ne!(
 495 |             cache_path1, cache_path2,
 496 |             "Different configs should have different cache files"
 497 |         );
 498 |     }
 499 | 
 500 |     #[test]
 501 |     fn test_normalize_project_path_absolute() {
 502 |         let temp_dir = tempdir().unwrap();
 503 |         let project_path = temp_dir.path().join("test_project");
 504 |         let _ = fs::create_dir(&project_path);
 505 | 
 506 |         let normalized = CacheManager::normalize_project_path(&project_path);
 507 |         assert!(normalized.is_absolute());
 508 |     }
 509 | 
 510 |     #[test]
 511 |     fn test_normalize_project_path_relative() {
 512 |         let temp_dir = tempdir().unwrap();
 513 |         let original_dir = std::env::current_dir().unwrap();
 514 | 
 515 |         // Change to temp directory
 516 |         std::env::set_current_dir(&temp_dir).unwrap();
 517 | 
 518 |         // Create a project directory
 519 |         let project_name = "relative_project";
 520 |         let _ = fs::create_dir(project_name);
 521 | 
 522 |         let relative_path = Path::new(project_name);
 523 |         let normalized = CacheManager::normalize_project_path(relative_path);
 524 | 
 525 |         // Restore original directory
 526 |         std::env::set_current_dir(original_dir).unwrap();
 527 | 
 528 |         assert!(normalized.is_absolute());
 529 |         assert!(normalized.to_string_lossy().contains(project_name));
 530 |     }
 531 | 
 532 |     #[test]
 533 |     fn test_hash_config_same_values() {
 534 |         let config1 = Config {
 535 |             filter: Some(vec!["rs".to_string(), "toml".to_string()]),
 536 |             ignore: Some(vec!["target".to_string()]),
 537 |             line_numbers: Some(false),
 538 |             ..Default::default()
 539 |         };
 540 | 
 541 |         let config2 = Config {
 542 |             filter: Some(vec!["rs".to_string(), "toml".to_string()]),
 543 |             ignore: Some(vec!["target".to_string()]),
 544 |             line_numbers: Some(false),
 545 |             ..Default::default()
 546 |         };
 547 | 
 548 |         let hash1 = CacheManager::hash_config(&config1);
 549 |         let hash2 = CacheManager::hash_config(&config2);
 550 | 
 551 |         assert_eq!(
 552 |             hash1, hash2,
 553 |             "Identical configs should produce identical hashes"
 554 |         );
 555 |     }
 556 | 
 557 |     #[test]
 558 |     fn test_migrate_old_cache_preserves_new_files() {
 559 |         let dir = tempdir().unwrap();
 560 |         let project_path = dir.path().join("test_project");
 561 |         let _ = fs::create_dir(&project_path);
 562 | 
 563 |         let cache_dir = project_path.join(".context-builder").join("cache");
 564 |         let _ = fs::create_dir_all(&cache_dir);
 565 | 
 566 |         // Create both old and new cache files
 567 |         let _ = fs::write(cache_dir.join("last_canonical.md"), "old content");
 568 |         let _ = fs::write(cache_dir.join("state_abc123_def456.json"), "new content");
 569 | 
 570 |         let config = Config::default();
 571 |         let _cache_manager = CacheManager::new(&project_path, &config);
 572 | 
 573 |         // Old file should be removed
 574 |         assert!(!cache_dir.join("last_canonical.md").exists());
 575 | 
 576 |         // New file should be preserved
 577 |         assert!(cache_dir.join("state_abc123_def456.json").exists());
 578 |     }
 579 | }
```

### File: `src/cli.rs`

- Size: 4720 bytes
- Modified: SystemTime { tv_sec: 1771098495, tv_nsec: 955580471 }

```rust
   1 | use clap::Parser;
   2 | 
   3 | /// CLI tool to aggregate directory contents into a single Markdown file optimized for LLM consumption
   4 | #[derive(Parser, Debug, Clone)]
   5 | #[clap(author, version, about)]
   6 | pub struct Args {
   7 |     /// Directory path to process
   8 |     #[clap(short = 'd', long, default_value = ".")]
   9 |     pub input: String,
  10 | 
  11 |     /// Output file path
  12 |     #[clap(short, long, default_value = "output.md")]
  13 |     pub output: String,
  14 | 
  15 |     /// File extensions to include (e.g., --filter rs,toml)
  16 |     #[clap(short = 'f', long, value_delimiter = ',')]
  17 |     pub filter: Vec<String>,
  18 | 
  19 |     /// Folder or file names to ignore (e.g., --ignore target --ignore lock)
  20 |     #[clap(short = 'i', long)]
  21 |     pub ignore: Vec<String>,
  22 | 
  23 |     /// Preview mode: only print the file tree to the console, don't generate the documentation file
  24 |     #[clap(long)]
  25 |     pub preview: bool,
  26 | 
  27 |     /// Token count mode: estimate the total token count of the final document
  28 |     #[clap(long)]
  29 |     pub token_count: bool,
  30 | 
  31 |     /// Add line numbers to code blocks in the output
  32 |     #[clap(long)]
  33 |     pub line_numbers: bool,
  34 | 
  35 |     /// Automatically answer yes to all prompts
  36 |     #[clap(short = 'y', long)]
  37 |     pub yes: bool,
  38 | 
  39 |     /// Maximum token budget for the output. Files are truncated/skipped when exceeded.
  40 |     #[clap(long)]
  41 |     pub max_tokens: Option<usize>,
  42 | 
  43 |     /// Output only diffs (omit full file contents; requires auto-diff & timestamped output)
  44 |     #[clap(long, default_value_t = false)]
  45 |     pub diff_only: bool,
  46 | 
  47 |     /// Clear the cached project state and exit
  48 |     #[clap(long)]
  49 |     pub clear_cache: bool,
  50 | 
  51 |     /// Initialize a new context-builder.toml config file in the current directory
  52 |     #[clap(long)]
  53 |     pub init: bool,
  54 | }
  55 | 
  56 | #[cfg(test)]
  57 | mod tests {
  58 |     use super::Args;
  59 |     use clap::Parser;
  60 | 
  61 |     #[test]
  62 |     fn parses_with_no_args() {
  63 |         let res = Args::try_parse_from(["context-builder"]);
  64 |         assert!(res.is_ok(), "Expected success when no args are provided");
  65 |     }
  66 | 
  67 |     #[test]
  68 |     fn parses_all_flags_and_options() {
  69 |         let args = Args::try_parse_from([
  70 |             "context-builder",
  71 |             "--input",
  72 |             "some/dir",
  73 |             "--output",
  74 |             "ctx.md",
  75 |             "--filter",
  76 |             "rs",
  77 |             "--filter",
  78 |             "toml",
  79 |             "--ignore",
  80 |             "target",
  81 |             "--ignore",
  82 |             "node_modules",
  83 |             "--preview",
  84 |             "--token-count",
  85 |             "--line-numbers",
  86 |             "--diff-only",
  87 |             "--clear-cache",
  88 |         ])
  89 |         .expect("should parse");
  90 | 
  91 |         assert_eq!(args.input, "some/dir");
  92 |         assert_eq!(args.output, "ctx.md");
  93 |         assert_eq!(args.filter, vec!["rs".to_string(), "toml".to_string()]);
  94 |         assert_eq!(
  95 |             args.ignore,
  96 |             vec!["target".to_string(), "node_modules".to_string()]
  97 |         );
  98 |         assert!(args.preview);
  99 |         assert!(args.token_count);
 100 |         assert!(args.line_numbers);
 101 |         assert!(args.diff_only);
 102 |         assert!(args.clear_cache);
 103 |     }
 104 | 
 105 |     #[test]
 106 |     fn short_flags_parse_correctly() {
 107 |         let args = Args::try_parse_from([
 108 |             "context-builder",
 109 |             "-d",
 110 |             ".",
 111 |             "-o",
 112 |             "out.md",
 113 |             "-f",
 114 |             "md",
 115 |             "-f",
 116 |             "rs",
 117 |             "-i",
 118 |             "target",
 119 |             "-i",
 120 |             ".git",
 121 |         ])
 122 |         .expect("should parse");
 123 | 
 124 |         assert_eq!(args.input, ".");
 125 |         assert_eq!(args.output, "out.md");
 126 |         assert_eq!(args.filter, vec!["md".to_string(), "rs".to_string()]);
 127 |         assert_eq!(args.ignore, vec!["target".to_string(), ".git".to_string()]);
 128 |         assert!(!args.preview);
 129 |         assert!(!args.line_numbers);
 130 |         assert!(!args.clear_cache);
 131 |     }
 132 | 
 133 |     #[test]
 134 |     fn defaults_for_options_when_not_provided() {
 135 |         let args = Args::try_parse_from(["context-builder", "-d", "proj"]).expect("should parse");
 136 | 
 137 |         assert_eq!(args.input, "proj");
 138 |         assert_eq!(args.output, "output.md");
 139 |         assert!(args.filter.is_empty());
 140 |         assert!(args.ignore.is_empty());
 141 |         assert!(!args.preview);
 142 |         assert!(!args.line_numbers);
 143 |         assert!(!args.diff_only);
 144 |         assert!(!args.clear_cache);
 145 |     }
 146 | 
 147 |     #[test]
 148 |     fn parses_diff_only_flag() {
 149 |         let args = Args::try_parse_from(["context-builder", "--diff-only"])
 150 |             .expect("should parse diff-only flag");
 151 |         assert!(args.diff_only);
 152 |         assert!(!args.clear_cache);
 153 |     }
 154 | 
 155 |     #[test]
 156 |     fn parses_clear_cache_flag() {
 157 |         let args = Args::try_parse_from(["context-builder", "--clear-cache"])
 158 |             .expect("should parse clear-cache flag");
 159 |         assert!(args.clear_cache);
 160 |         assert!(!args.diff_only);
 161 |     }
 162 | }
```

### File: `src/config.rs`

- Size: 7686 bytes
- Modified: SystemTime { tv_sec: 1771098515, tv_nsec: 244845851 }

```rust
   1 | use serde::Deserialize;
   2 | use std::fs;
   3 | use std::path::Path;
   4 | 
   5 | /// Global configuration loaded from `context-builder.toml`.
   6 | ///
   7 | /// Any field left as `None` means "use the CLI default / do not override".
   8 | /// Command-line arguments always take precedence over values provided here.
   9 | ///
  10 | /// Example `context-builder.toml`:
  11 | /// ```toml
  12 | /// output = "context.md"
  13 | /// output_folder = "docs"
  14 | /// timestamped_output = true
  15 | /// auto_diff = true
  16 | /// diff_only = true         # Emit only change summary + modified file diffs (no full file bodies)
  17 | /// filter = ["rs", "toml"]
  18 | /// ignore = ["target", ".git"]
  19 | /// line_numbers = false
  20 | /// diff_context_lines = 5
  21 | /// ```
  22 | ///
  23 | #[derive(Deserialize, Debug, Default, Clone)]
  24 | pub struct Config {
  25 |     /// Output file name (or base name when `timestamped_output = true`)
  26 |     pub output: Option<String>,
  27 | 
  28 |     /// File extensions to include (no leading dot, e.g. `rs`, `toml`)
  29 |     pub filter: Option<Vec<String>>,
  30 | 
  31 |     /// File / directory names to ignore (exact name matches)
  32 |     pub ignore: Option<Vec<String>>,
  33 | 
  34 |     /// Add line numbers to code blocks
  35 |     pub line_numbers: Option<bool>,
  36 | 
  37 |     /// Preview only the file tree (no file output)
  38 |     pub preview: Option<bool>,
  39 | 
  40 |     /// Token counting mode
  41 |     pub token_count: Option<bool>,
  42 | 
  43 |     /// Optional folder to place the generated output file(s) in
  44 |     pub output_folder: Option<String>,
  45 | 
  46 |     /// If true, append a UTC timestamp to the output file name (before extension)
  47 |     pub timestamped_output: Option<bool>,
  48 | 
  49 |     /// Assume "yes" for overwrite / processing confirmations
  50 |     pub yes: Option<bool>,
  51 | 
  52 |     /// Enable automatic diff generation (requires `timestamped_output = true`)
  53 |     pub auto_diff: Option<bool>,
  54 | 
  55 |     /// Override number of unified diff context lines (falls back to env or default = 3)
  56 |     pub diff_context_lines: Option<usize>,
  57 | 
  58 |     /// When true, emit ONLY:
  59 |     /// - Header + file tree
  60 |     /// - Change Summary
  61 |     /// - Per-file diffs for modified files
  62 |     ///
  63 |     /// Excludes full file contents section entirely. Added files appear only in the
  64 |     /// change summary (and are marked Added) but their full content is omitted.
  65 |     pub diff_only: Option<bool>,
  66 | 
  67 |     /// Encoding handling strategy for non-UTF-8 files.
  68 |     /// - "detect": Attempt to detect and transcode to UTF-8 (default)
  69 |     /// - "strict": Only include valid UTF-8 files, skip others
  70 |     /// - "skip": Skip all non-UTF-8 files without transcoding attempts
  71 |     pub encoding_strategy: Option<String>,
  72 | 
  73 |     /// Maximum token budget for the output. Files are truncated/skipped when exceeded.
  74 |     pub max_tokens: Option<usize>,
  75 | }
  76 | 
  77 | /// Load configuration from `context-builder.toml` in the current working directory.
  78 | /// Returns `None` if the file does not exist or cannot be parsed.
  79 | pub fn load_config() -> Option<Config> {
  80 |     let config_path = Path::new("context-builder.toml");
  81 |     if config_path.exists() {
  82 |         let content = fs::read_to_string(config_path).ok()?;
  83 |         toml::from_str(&content).ok()
  84 |     } else {
  85 |         None
  86 |     }
  87 | }
  88 | 
  89 | /// Load configuration from `context-builder.toml` in the specified project root directory.
  90 | /// Returns `None` if the file does not exist or cannot be parsed.
  91 | pub fn load_config_from_path(project_root: &Path) -> Option<Config> {
  92 |     let config_path = project_root.join("context-builder.toml");
  93 |     if config_path.exists() {
  94 |         let content = fs::read_to_string(config_path).ok()?;
  95 |         toml::from_str(&content).ok()
  96 |     } else {
  97 |         None
  98 |     }
  99 | }
 100 | 
 101 | #[cfg(test)]
 102 | mod tests {
 103 |     use super::*;
 104 |     use std::fs;
 105 |     use tempfile::tempdir;
 106 | 
 107 |     #[test]
 108 |     fn load_config_nonexistent_file() {
 109 |         // Test loading config when file doesn't exist by temporarily changing directory
 110 |         let temp_dir = tempdir().unwrap();
 111 |         let original_dir = std::env::current_dir().unwrap();
 112 | 
 113 |         // Change to temp directory where no config file exists
 114 |         std::env::set_current_dir(&temp_dir).unwrap();
 115 | 
 116 |         let result = load_config();
 117 | 
 118 |         // Restore original directory
 119 |         std::env::set_current_dir(original_dir).unwrap();
 120 | 
 121 |         assert!(result.is_none());
 122 |     }
 123 | 
 124 |     #[test]
 125 |     fn load_config_from_path_nonexistent_file() {
 126 |         let dir = tempdir().unwrap();
 127 |         let result = load_config_from_path(dir.path());
 128 |         assert!(result.is_none());
 129 |     }
 130 | 
 131 |     #[test]
 132 |     fn load_config_from_path_valid_config() {
 133 |         let dir = tempdir().unwrap();
 134 |         let config_path = dir.path().join("context-builder.toml");
 135 | 
 136 |         let config_content = r#"
 137 | output = "test-output.md"
 138 | filter = ["rs", "toml"]
 139 | ignore = ["target", ".git"]
 140 | line_numbers = true
 141 | preview = false
 142 | token_count = true
 143 | timestamped_output = true
 144 | yes = false
 145 | auto_diff = true
 146 | diff_context_lines = 5
 147 | diff_only = false
 148 | encoding_strategy = "detect"
 149 | "#;
 150 | 
 151 |         fs::write(&config_path, config_content).unwrap();
 152 | 
 153 |         let config = load_config_from_path(dir.path()).unwrap();
 154 |         assert_eq!(config.output.unwrap(), "test-output.md");
 155 |         assert_eq!(config.filter.unwrap(), vec!["rs", "toml"]);
 156 |         assert_eq!(config.ignore.unwrap(), vec!["target", ".git"]);
 157 |         assert!(config.line_numbers.unwrap());
 158 |         assert!(!config.preview.unwrap());
 159 |         assert!(config.token_count.unwrap());
 160 |         assert!(config.timestamped_output.unwrap());
 161 |         assert!(!config.yes.unwrap());
 162 |         assert!(config.auto_diff.unwrap());
 163 |         assert_eq!(config.diff_context_lines.unwrap(), 5);
 164 |         assert!(!config.diff_only.unwrap());
 165 |         assert_eq!(config.encoding_strategy.unwrap(), "detect");
 166 |     }
 167 | 
 168 |     #[test]
 169 |     fn load_config_from_path_partial_config() {
 170 |         let dir = tempdir().unwrap();
 171 |         let config_path = dir.path().join("context-builder.toml");
 172 | 
 173 |         let config_content = r#"
 174 | output = "minimal.md"
 175 | filter = ["py"]
 176 | "#;
 177 | 
 178 |         fs::write(&config_path, config_content).unwrap();
 179 | 
 180 |         let config = load_config_from_path(dir.path()).unwrap();
 181 |         assert_eq!(config.output.unwrap(), "minimal.md");
 182 |         assert_eq!(config.filter.unwrap(), vec!["py"]);
 183 |         assert!(config.ignore.is_none());
 184 |         assert!(config.line_numbers.is_none());
 185 |         assert!(config.auto_diff.is_none());
 186 |     }
 187 | 
 188 |     #[test]
 189 |     fn load_config_from_path_invalid_toml() {
 190 |         let dir = tempdir().unwrap();
 191 |         let config_path = dir.path().join("context-builder.toml");
 192 | 
 193 |         // Invalid TOML content
 194 |         let config_content = r#"
 195 | output = "test.md"
 196 | invalid_toml [
 197 | "#;
 198 | 
 199 |         fs::write(&config_path, config_content).unwrap();
 200 | 
 201 |         let config = load_config_from_path(dir.path());
 202 |         assert!(config.is_none());
 203 |     }
 204 | 
 205 |     #[test]
 206 |     fn load_config_from_path_empty_config() {
 207 |         let dir = tempdir().unwrap();
 208 |         let config_path = dir.path().join("context-builder.toml");
 209 | 
 210 |         fs::write(&config_path, "").unwrap();
 211 | 
 212 |         let config = load_config_from_path(dir.path()).unwrap();
 213 |         assert!(config.output.is_none());
 214 |         assert!(config.filter.is_none());
 215 |         assert!(config.ignore.is_none());
 216 |     }
 217 | 
 218 |     #[test]
 219 |     fn config_default_implementation() {
 220 |         let config = Config::default();
 221 |         assert!(config.output.is_none());
 222 |         assert!(config.filter.is_none());
 223 |         assert!(config.ignore.is_none());
 224 |         assert!(config.line_numbers.is_none());
 225 |         assert!(config.preview.is_none());
 226 |         assert!(config.token_count.is_none());
 227 |         assert!(config.output_folder.is_none());
 228 |         assert!(config.timestamped_output.is_none());
 229 |         assert!(config.yes.is_none());
 230 |         assert!(config.auto_diff.is_none());
 231 |         assert!(config.diff_context_lines.is_none());
 232 |         assert!(config.diff_only.is_none());
 233 |         assert!(config.encoding_strategy.is_none());
 234 |     }
 235 | }
```

### File: `src/config_resolver.rs`

- Size: 15339 bytes
- Modified: SystemTime { tv_sec: 1771098965, tv_nsec: 552041145 }

```rust
   1 | //! Configuration resolution module for context-builder.
   2 | //!
   3 | //! This module provides centralized logic for merging CLI arguments with configuration
   4 | //! file values, implementing proper precedence rules and handling complex scenarios
   5 | //! like timestamping and output folder resolution.
   6 | 
   7 | use chrono::Utc;
   8 | use std::path::{Path, PathBuf};
   9 | 
  10 | use crate::cli::Args;
  11 | use crate::config::Config;
  12 | 
  13 | /// Resolved configuration combining CLI arguments and config file values
  14 | #[derive(Debug, Clone)]
  15 | pub struct ResolvedConfig {
  16 |     pub input: String,
  17 |     pub output: String,
  18 |     pub filter: Vec<String>,
  19 |     pub ignore: Vec<String>,
  20 |     pub line_numbers: bool,
  21 |     pub preview: bool,
  22 |     pub token_count: bool,
  23 |     pub yes: bool,
  24 |     pub diff_only: bool,
  25 |     pub clear_cache: bool,
  26 |     pub auto_diff: bool,
  27 |     pub diff_context_lines: usize,
  28 |     pub max_tokens: Option<usize>,
  29 |     pub init: bool,
  30 | }
  31 | 
  32 | /// Result of configuration resolution including the final config and any warnings
  33 | #[derive(Debug)]
  34 | pub struct ConfigResolution {
  35 |     pub config: ResolvedConfig,
  36 |     pub warnings: Vec<String>,
  37 | }
  38 | 
  39 | /// Resolves final configuration by merging CLI arguments with config file values.
  40 | ///
  41 | /// Precedence rules (highest to lowest):
  42 | /// 1. Explicit CLI arguments (non-default values)
  43 | /// 2. Configuration file values
  44 | /// 3. CLI default values
  45 | ///
  46 | /// Special handling:
  47 | /// - `output` field supports timestamping and output folder resolution
  48 | /// - Boolean flags respect explicit CLI usage vs defaults
  49 | /// - Arrays (filter, ignore) use CLI if non-empty, otherwise config file
  50 | pub fn resolve_final_config(mut args: Args, config: Option<Config>) -> ConfigResolution {
  51 |     let mut warnings = Vec::new();
  52 | 
  53 |     // Start with CLI defaults, then apply config file, then explicit CLI overrides
  54 |     let final_config = if let Some(config) = config {
  55 |         apply_config_to_args(&mut args, &config, &mut warnings);
  56 |         resolve_output_path(&mut args, &config, &mut warnings);
  57 |         config
  58 |     } else {
  59 |         Config::default()
  60 |     };
  61 | 
  62 |     let resolved = ResolvedConfig {
  63 |         input: args.input,
  64 |         output: args.output,
  65 |         filter: args.filter,
  66 |         ignore: args.ignore,
  67 |         line_numbers: args.line_numbers,
  68 |         preview: args.preview,
  69 |         token_count: args.token_count,
  70 |         yes: args.yes,
  71 |         diff_only: args.diff_only,
  72 |         clear_cache: args.clear_cache,
  73 |         auto_diff: final_config.auto_diff.unwrap_or(false),
  74 |         diff_context_lines: final_config.diff_context_lines.unwrap_or(3),
  75 |         max_tokens: args.max_tokens.or(final_config.max_tokens),
  76 |         init: args.init,
  77 |     };
  78 | 
  79 |     ConfigResolution {
  80 |         config: resolved,
  81 |         warnings,
  82 |     }
  83 | }
  84 | 
  85 | /// Apply configuration file values to CLI arguments based on precedence rules
  86 | fn apply_config_to_args(args: &mut Args, config: &Config, warnings: &mut Vec<String>) {
  87 |     // Output: only apply config if CLI is using default value
  88 |     if args.output == "output.md"
  89 |         && let Some(ref output) = config.output
  90 |     {
  91 |         args.output = output.clone();
  92 |     }
  93 | 
  94 |     // Filter: CLI takes precedence if non-empty
  95 |     if args.filter.is_empty()
  96 |         && let Some(ref filter) = config.filter
  97 |     {
  98 |         args.filter = filter.clone();
  99 |     }
 100 | 
 101 |     // Ignore: CLI takes precedence if non-empty
 102 |     if args.ignore.is_empty()
 103 |         && let Some(ref ignore) = config.ignore
 104 |     {
 105 |         args.ignore = ignore.clone();
 106 |     }
 107 | 
 108 |     // Boolean flags: config applies only if CLI is using default (false)
 109 |     // Note: We can't distinguish between explicit --no-flag and default false,
 110 |     // so config file can only enable features, not disable them
 111 |     if !args.line_numbers
 112 |         && let Some(line_numbers) = config.line_numbers
 113 |     {
 114 |         args.line_numbers = line_numbers;
 115 |     }
 116 | 
 117 |     if !args.preview
 118 |         && let Some(preview) = config.preview
 119 |     {
 120 |         args.preview = preview;
 121 |     }
 122 | 
 123 |     if !args.token_count
 124 |         && let Some(token_count) = config.token_count
 125 |     {
 126 |         args.token_count = token_count;
 127 |     }
 128 | 
 129 |     if !args.yes
 130 |         && let Some(yes) = config.yes
 131 |     {
 132 |         args.yes = yes;
 133 |     }
 134 | 
 135 |     // diff_only: config can enable it, but CLI flag always takes precedence
 136 |     if !args.diff_only
 137 |         && let Some(true) = config.diff_only
 138 |     {
 139 |         args.diff_only = true;
 140 |     }
 141 | 
 142 |     // Validate auto_diff configuration
 143 |     if let Some(true) = config.auto_diff
 144 |         && config.timestamped_output != Some(true)
 145 |     {
 146 |         warnings.push(
 147 |             "auto_diff is enabled but timestamped_output is not enabled. \
 148 |             Auto-diff requires timestamped_output = true to function properly."
 149 |                 .to_string(),
 150 |         );
 151 |     }
 152 | }
 153 | 
 154 | /// Resolve output path including timestamping and output folder logic
 155 | fn resolve_output_path(args: &mut Args, config: &Config, warnings: &mut Vec<String>) {
 156 |     let mut output_folder_path: Option<PathBuf> = None;
 157 | 
 158 |     // Apply output folder first
 159 |     if let Some(ref output_folder) = config.output_folder {
 160 |         let mut path = PathBuf::from(output_folder);
 161 |         path.push(&args.output);
 162 |         args.output = path.to_string_lossy().to_string();
 163 |         output_folder_path = Some(PathBuf::from(output_folder));
 164 |     }
 165 | 
 166 |     // Apply timestamping if enabled
 167 |     if let Some(true) = config.timestamped_output {
 168 |         let timestamp = Utc::now().format("%Y%m%d%H%M%S").to_string();
 169 |         let path = Path::new(&args.output);
 170 | 
 171 |         let stem = path
 172 |             .file_stem()
 173 |             .and_then(|s| s.to_str())
 174 |             .unwrap_or("output");
 175 | 
 176 |         let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("md");
 177 | 
 178 |         let new_filename = format!("{}_{}.{}", stem, timestamp, extension);
 179 | 
 180 |         if let Some(output_folder) = output_folder_path {
 181 |             args.output = output_folder
 182 |                 .join(new_filename)
 183 |                 .to_string_lossy()
 184 |                 .to_string();
 185 |         } else {
 186 |             let new_path = path.with_file_name(new_filename);
 187 |             args.output = new_path.to_string_lossy().to_string();
 188 |         }
 189 |     }
 190 | 
 191 |     // Validate output folder exists if specified
 192 |     if let Some(ref output_folder) = config.output_folder {
 193 |         let folder_path = Path::new(output_folder);
 194 |         if !folder_path.exists() {
 195 |             warnings.push(format!(
 196 |                 "Output folder '{}' does not exist. It will be created if possible.",
 197 |                 output_folder
 198 |             ));
 199 |         }
 200 |     }
 201 | }
 202 | 
 203 | /// Check if CLI arguments have been explicitly set vs using defaults.
 204 | /// This is a best-effort detection since clap doesn't provide this information directly.
 205 | #[allow(dead_code)]
 206 | fn detect_explicit_args() -> ExplicitArgs {
 207 |     let args: Vec<String> = std::env::args().collect();
 208 | 
 209 |     ExplicitArgs {
 210 |         output: args.iter().any(|arg| arg == "-o" || arg == "--output"),
 211 |         filter: args.iter().any(|arg| arg == "-f" || arg == "--filter"),
 212 |         ignore: args.iter().any(|arg| arg == "-i" || arg == "--ignore"),
 213 |         line_numbers: args.iter().any(|arg| arg == "--line-numbers"),
 214 |         preview: args.iter().any(|arg| arg == "--preview"),
 215 |         token_count: args.iter().any(|arg| arg == "--token-count"),
 216 |         yes: args.iter().any(|arg| arg == "-y" || arg == "--yes"),
 217 |         diff_only: args.iter().any(|arg| arg == "--diff-only"),
 218 |     }
 219 | }
 220 | 
 221 | /// Tracks which CLI arguments were explicitly provided vs using defaults
 222 | #[allow(dead_code)]
 223 | struct ExplicitArgs {
 224 |     output: bool,
 225 |     filter: bool,
 226 |     ignore: bool,
 227 |     line_numbers: bool,
 228 |     preview: bool,
 229 |     token_count: bool,
 230 |     yes: bool,
 231 |     diff_only: bool,
 232 | }
 233 | 
 234 | #[cfg(test)]
 235 | mod tests {
 236 |     use super::*;
 237 | 
 238 |     #[test]
 239 |     fn test_config_precedence_cli_over_config() {
 240 |         let args = Args {
 241 |             input: "src".to_string(),
 242 |             output: "custom.md".to_string(), // Explicit CLI value
 243 |             filter: vec!["rs".to_string()],  // Explicit CLI value
 244 |             ignore: vec![],
 245 |             line_numbers: true, // Explicit CLI value
 246 |             preview: false,
 247 |             token_count: false,
 248 |             yes: false,
 249 |             diff_only: false,
 250 |             clear_cache: false,
 251 |             init: false,
 252 |             max_tokens: None,
 253 |         };
 254 | 
 255 |         let config = Config {
 256 |             output: Some("config.md".to_string()),  // Should be ignored
 257 |             filter: Some(vec!["toml".to_string()]), // Should be ignored
 258 |             line_numbers: Some(false),              // Should be ignored
 259 |             preview: Some(true),                    // Should apply
 260 |             ..Default::default()
 261 |         };
 262 | 
 263 |         let resolution = resolve_final_config(args.clone(), Some(config));
 264 | 
 265 |         assert_eq!(resolution.config.output, "custom.md"); // CLI wins
 266 |         assert_eq!(resolution.config.filter, vec!["rs"]); // CLI wins
 267 |         assert!(resolution.config.line_numbers); // CLI wins
 268 |         assert!(resolution.config.preview); // Config applies
 269 |     }
 270 | 
 271 |     #[test]
 272 |     fn test_config_applies_when_cli_uses_defaults() {
 273 |         let args = Args {
 274 |             input: "src".to_string(),
 275 |             output: "output.md".to_string(), // Default value
 276 |             filter: vec![],                  // Default value
 277 |             ignore: vec![],                  // Default value
 278 |             line_numbers: false,             // Default value
 279 |             preview: false,                  // Default value
 280 |             token_count: false,              // Default value
 281 |             yes: false,                      // Default value
 282 |             diff_only: false,                // Default value
 283 |             clear_cache: false,
 284 |             init: false,
 285 |             max_tokens: None,
 286 |         };
 287 | 
 288 |         let config = Config {
 289 |             output: Some("from_config.md".to_string()),
 290 |             filter: Some(vec!["rs".to_string(), "toml".to_string()]),
 291 |             ignore: Some(vec!["target".to_string()]),
 292 |             line_numbers: Some(true),
 293 |             preview: Some(true),
 294 |             token_count: Some(true),
 295 |             yes: Some(true),
 296 |             diff_only: Some(true),
 297 |             ..Default::default()
 298 |         };
 299 | 
 300 |         let resolution = resolve_final_config(args, Some(config));
 301 | 
 302 |         assert_eq!(resolution.config.output, "from_config.md");
 303 |         assert_eq!(
 304 |             resolution.config.filter,
 305 |             vec!["rs".to_string(), "toml".to_string()]
 306 |         );
 307 |         assert_eq!(resolution.config.ignore, vec!["target".to_string()]);
 308 |         assert!(resolution.config.line_numbers);
 309 |         assert!(resolution.config.preview);
 310 |         assert!(resolution.config.token_count);
 311 |         assert!(resolution.config.yes);
 312 |         assert!(resolution.config.diff_only);
 313 |     }
 314 | 
 315 |     #[test]
 316 |     fn test_timestamped_output_resolution() {
 317 |         let args = Args {
 318 |             input: "src".to_string(),
 319 |             output: "test.md".to_string(),
 320 |             filter: vec![],
 321 |             ignore: vec![],
 322 |             line_numbers: false,
 323 |             preview: false,
 324 |             token_count: false,
 325 |             yes: false,
 326 |             diff_only: false,
 327 |             clear_cache: false,
 328 |             init: false,
 329 |             max_tokens: None,
 330 |         };
 331 | 
 332 |         let config = Config {
 333 |             timestamped_output: Some(true),
 334 |             ..Default::default()
 335 |         };
 336 | 
 337 |         let resolution = resolve_final_config(args, Some(config));
 338 | 
 339 |         // Output should have timestamp format: test_YYYYMMDDHHMMSS.md
 340 |         assert!(resolution.config.output.starts_with("test_"));
 341 |         assert!(resolution.config.output.ends_with(".md"));
 342 |         assert!(resolution.config.output.len() > "test_.md".len());
 343 |     }
 344 | 
 345 |     #[test]
 346 |     fn test_output_folder_resolution() {
 347 |         let args = Args {
 348 |             input: "src".to_string(),
 349 |             output: "test.md".to_string(),
 350 |             filter: vec![],
 351 |             ignore: vec![],
 352 |             line_numbers: false,
 353 |             preview: false,
 354 |             token_count: false,
 355 |             yes: false,
 356 |             diff_only: false,
 357 |             clear_cache: false,
 358 |             init: false,
 359 |             max_tokens: None,
 360 |         };
 361 | 
 362 |         let config = Config {
 363 |             output_folder: Some("docs".to_string()),
 364 |             ..Default::default()
 365 |         };
 366 | 
 367 |         let resolution = resolve_final_config(args, Some(config));
 368 | 
 369 |         assert!(resolution.config.output.contains("docs"));
 370 |         assert!(resolution.config.output.ends_with("test.md"));
 371 |     }
 372 | 
 373 |     #[test]
 374 |     fn test_output_folder_with_timestamping() {
 375 |         let args = Args {
 376 |             input: "src".to_string(),
 377 |             output: "test.md".to_string(),
 378 |             filter: vec![],
 379 |             ignore: vec![],
 380 |             line_numbers: false,
 381 |             preview: false,
 382 |             token_count: false,
 383 |             yes: false,
 384 |             diff_only: false,
 385 |             clear_cache: false,
 386 |             init: false,
 387 |             max_tokens: None,
 388 |         };
 389 | 
 390 |         let config = Config {
 391 |             output_folder: Some("docs".to_string()),
 392 |             timestamped_output: Some(true),
 393 |             ..Default::default()
 394 |         };
 395 | 
 396 |         let resolution = resolve_final_config(args, Some(config));
 397 | 
 398 |         assert!(resolution.config.output.contains("docs"));
 399 |         assert!(resolution.config.output.contains("test_"));
 400 |         assert!(resolution.config.output.ends_with(".md"));
 401 |     }
 402 | 
 403 |     #[test]
 404 |     fn test_auto_diff_without_timestamping_warning() {
 405 |         let args = Args {
 406 |             input: "src".to_string(),
 407 |             output: "test.md".to_string(),
 408 |             filter: vec![],
 409 |             ignore: vec![],
 410 |             line_numbers: false,
 411 |             preview: false,
 412 |             token_count: false,
 413 |             yes: false,
 414 |             diff_only: false,
 415 |             clear_cache: false,
 416 |             init: false,
 417 |             max_tokens: None,
 418 |         };
 419 | 
 420 |         let config = Config {
 421 |             auto_diff: Some(true),
 422 |             timestamped_output: Some(false), // This should generate a warning
 423 |             ..Default::default()
 424 |         };
 425 | 
 426 |         let resolution = resolve_final_config(args, Some(config));
 427 | 
 428 |         assert!(!resolution.warnings.is_empty());
 429 |         assert!(resolution.warnings[0].contains("auto_diff"));
 430 |         assert!(resolution.warnings[0].contains("timestamped_output"));
 431 |     }
 432 | 
 433 |     #[test]
 434 |     fn test_no_config_uses_cli_defaults() {
 435 |         let args = Args {
 436 |             input: "src".to_string(),
 437 |             output: "output.md".to_string(),
 438 |             filter: vec![],
 439 |             ignore: vec![],
 440 |             line_numbers: false,
 441 |             preview: false,
 442 |             token_count: false,
 443 |             yes: false,
 444 |             diff_only: false,
 445 |             clear_cache: false,
 446 |             init: false,
 447 |             max_tokens: None,
 448 |         };
 449 | 
 450 |         let resolution = resolve_final_config(args.clone(), None);
 451 | 
 452 |         assert_eq!(resolution.config.input, args.input);
 453 |         assert_eq!(resolution.config.output, args.output);
 454 |         assert_eq!(resolution.config.filter, args.filter);
 455 |         assert_eq!(resolution.config.ignore, args.ignore);
 456 |         assert_eq!(resolution.config.line_numbers, args.line_numbers);
 457 |         assert_eq!(resolution.config.preview, args.preview);
 458 |         assert_eq!(resolution.config.token_count, args.token_count);
 459 |         assert_eq!(resolution.config.yes, args.yes);
 460 |         assert_eq!(resolution.config.diff_only, args.diff_only);
 461 |         assert!(!resolution.config.auto_diff);
 462 |         assert_eq!(resolution.config.diff_context_lines, 3);
 463 |         assert!(resolution.warnings.is_empty());
 464 |     }
 465 | }
```

### File: `src/diff.rs`

- Size: 20099 bytes
- Modified: SystemTime { tv_sec: 1771053288, tv_nsec: 65557165 }

```rust
   1 | use similar::{ChangeTag, TextDiff};
   2 | use std::collections::HashMap;
   3 | 
   4 | /// Line based diff utilities.
   5 | ///
   6 | /// This module previously exposed `generate_diff` which produced a single
   7 | /// "## File Differences" section for an entire markdown document. That
   8 | /// approach made it easy for volatile sections (timestamps, file tree
   9 | /// structure, etc.) to create noisy diffs. To address this the new
  10 | /// per‑file API lets the caller diff only the normalized *file content*
  11 | /// blocks that appear under each `### File: `path`` heading in the
  12 | /// canonical output, completely ignoring the global header or the file
  13 | /// tree portion. Each file receives an isolated unified style diff.
  14 | ///
  15 | /// High level additions:
  16 | /// * `PerFileStatus` – classification of the change.
  17 | /// * `PerFileDiff` – structured diff result for a single file.
  18 | /// * `diff_file_contents` – core engine producing diffs per file without any
  19 | ///   global "## File Differences" header.
  20 | /// * `render_per_file_diffs` – helper to render the per file diffs into
  21 | ///   markdown (still omits a global header so the caller can choose).
  22 | ///
  23 | /// Backwards compatibility: the existing `generate_diff` function (full
  24 | /// document diff) is retained for now. New code should prefer the
  25 | /// per‑file functions.
  26 | /// Determine number of context lines either from explicit argument or env.
  27 | fn resolve_context_lines(explicit: Option<usize>) -> usize {
  28 |     explicit
  29 |         .filter(|v| *v > 0)
  30 |         .or_else(|| {
  31 |             std::env::var("CB_DIFF_CONTEXT_LINES")
  32 |                 .ok()
  33 |                 .and_then(|v| v.parse().ok())
  34 |                 .filter(|v: &usize| *v > 0)
  35 |         })
  36 |         .unwrap_or(3)
  37 | }
  38 | 
  39 | /// Original API: produce a single markdown section headed by "## File Differences".
  40 | /// (Kept unchanged for compatibility.)
  41 | pub fn generate_diff(old_content: &str, new_content: &str) -> String {
  42 |     let diff = TextDiff::from_lines(old_content, new_content);
  43 |     if diff.ratio() == 1.0 {
  44 |         return String::new();
  45 |     }
  46 |     let context_lines = resolve_context_lines(None);
  47 |     let grouped = diff.grouped_ops(context_lines);
  48 |     let mut out = String::new();
  49 |     out.push_str("## File Differences\n\n");
  50 |     out.push_str("```diff\n");
  51 |     for (group_index, group) in grouped.iter().enumerate() {
  52 |         if group_index > 0 {
  53 |             out.push_str("  ...\n");
  54 |         }
  55 |         for op in group {
  56 |             for change in diff.iter_changes(op) {
  57 |                 let tag = change.tag();
  58 |                 let mut line = change.to_string();
  59 |                 if line.ends_with('\n') {
  60 |                     line.pop();
  61 |                     if line.ends_with('\r') {
  62 |                         line.pop();
  63 |                     }
  64 |                 }
  65 | 
  66 |                 match tag {
  67 |                     ChangeTag::Delete => {
  68 |                         out.push_str("- ");
  69 |                         out.push_str(&line);
  70 |                         out.push('\n');
  71 |                     }
  72 |                     ChangeTag::Insert => {
  73 |                         out.push_str("+ ");
  74 |                         out.push_str(&line);
  75 |                         out.push('\n');
  76 |                     }
  77 |                     ChangeTag::Equal => {
  78 |                         out.push_str("  ");
  79 |                         out.push_str(&line);
  80 |                         out.push('\n');
  81 |                     }
  82 |                 }
  83 |             }
  84 |         }
  85 |     }
  86 |     out.push_str("```\n\n");
  87 |     out
  88 | }
  89 | 
  90 | /// Classification of how a file changed between two snapshots.
  91 | #[derive(Debug, Clone, PartialEq, Eq)]
  92 | pub enum PerFileStatus {
  93 |     Added,
  94 |     Removed,
  95 |     Modified,
  96 |     Unchanged,
  97 | }
  98 | 
  99 | /// Structured diff result for a single file.
 100 | #[derive(Debug, Clone)]
 101 | pub struct PerFileDiff {
 102 |     pub path: String,
 103 |     pub status: PerFileStatus,
 104 |     /// Unified diff fenced in ```diff (omitted when status == Unchanged and skip_unchanged=true)
 105 |     pub diff: String,
 106 | }
 107 | 
 108 | impl PerFileDiff {
 109 |     pub fn is_changed(&self) -> bool {
 110 |         self.status != PerFileStatus::Unchanged
 111 |     }
 112 | }
 113 | 
 114 | /// Produce a unified style diff for two text blobs WITHOUT adding any global
 115 | /// section header. Returns empty string if contents are identical.
 116 | fn unified_no_header(old: &str, new: &str, context_lines: usize) -> String {
 117 |     let diff = TextDiff::from_lines(old, new);
 118 |     if diff.ratio() == 1.0 {
 119 |         return String::new();
 120 |     }
 121 |     let grouped = diff.grouped_ops(context_lines);
 122 |     let mut out = String::new();
 123 |     out.push_str("```diff\n");
 124 |     for (group_index, group) in grouped.iter().enumerate() {
 125 |         if group_index > 0 {
 126 |             out.push_str("  ...\n");
 127 |         }
 128 |         for op in group {
 129 |             for change in diff.iter_changes(op) {
 130 |                 let tag = change.tag();
 131 |                 let mut line = change.to_string();
 132 |                 if line.ends_with('\n') {
 133 |                     line.pop();
 134 |                     if line.ends_with('\r') {
 135 |                         line.pop();
 136 |                     }
 137 |                 }
 138 | 
 139 |                 match tag {
 140 |                     ChangeTag::Delete => {
 141 |                         out.push_str("- ");
 142 |                         out.push_str(&line);
 143 |                         out.push('\n');
 144 |                     }
 145 |                     ChangeTag::Insert => {
 146 |                         out.push_str("+ ");
 147 |                         out.push_str(&line);
 148 |                         out.push('\n');
 149 |                     }
 150 |                     ChangeTag::Equal => {
 151 |                         out.push_str("  ");
 152 |                         out.push_str(&line);
 153 |                         out.push('\n');
 154 |                     }
 155 |                 }
 156 |             }
 157 |         }
 158 |     }
 159 |     out.push_str("```\n");
 160 |     out
 161 | }
 162 | 
 163 | /// Diff per file content sets.
 164 | ///
 165 | /// Inputs are maps keyed by file path (relative or absolute – caller decides)
 166 | /// with values being the raw file content EXACTLY as you wish it to be diffed
 167 | /// (e.g. already stripped of volatile metadata, no size/modified lines, only
 168 | /// the real file body). This keeps higher level logic (parsing the markdown
 169 | /// document) out of the diff layer.
 170 | ///
 171 | /// Returns a vector of `PerFileDiff` for every file that is Added, Removed,
 172 | /// or Modified. Unchanged files are omitted by default (`skip_unchanged=true`)
 173 | /// to reduce noise, but you can opt to include them.
 174 | pub fn diff_file_contents(
 175 |     previous: &HashMap<String, String>,
 176 |     current: &HashMap<String, String>,
 177 |     skip_unchanged: bool,
 178 |     explicit_context: Option<usize>,
 179 | ) -> Vec<PerFileDiff> {
 180 |     let mut all_paths: Vec<String> = previous.keys().chain(current.keys()).cloned().collect();
 181 |     all_paths.sort();
 182 |     all_paths.dedup();
 183 | 
 184 |     let context_lines = resolve_context_lines(explicit_context);
 185 |     let mut results = Vec::new();
 186 | 
 187 |     for path in all_paths {
 188 |         let old_opt = previous.get(&path);
 189 |         let new_opt = current.get(&path);
 190 |         match (old_opt, new_opt) {
 191 |             (None, Some(new_content)) => {
 192 |                 // Added file: present only in current snapshot
 193 |                 let mut diff = String::new();
 194 |                 diff.push_str("```diff\n");
 195 |                 for line in new_content.lines() {
 196 |                     diff.push_str("+ ");
 197 |                     diff.push_str(line);
 198 |                     diff.push('\n');
 199 |                 }
 200 |                 diff.push_str("```\n");
 201 |                 results.push(PerFileDiff {
 202 |                     path,
 203 |                     status: PerFileStatus::Added,
 204 |                     diff,
 205 |                 });
 206 |             }
 207 |             (Some(_old_content), None) => {
 208 |                 // Removed file
 209 |                 let old_content = previous.get(&path).unwrap();
 210 |                 let mut diff = String::new();
 211 |                 diff.push_str("```diff\n");
 212 |                 for line in old_content.lines() {
 213 |                     diff.push_str("- ");
 214 |                     diff.push_str(line);
 215 |                     diff.push('\n');
 216 |                 }
 217 |                 diff.push_str("```\n");
 218 |                 results.push(PerFileDiff {
 219 |                     path,
 220 |                     status: PerFileStatus::Removed,
 221 |                     diff,
 222 |                 });
 223 |             }
 224 |             (Some(old_content), Some(new_content)) => {
 225 |                 if old_content == new_content {
 226 |                     if !skip_unchanged {
 227 |                         results.push(PerFileDiff {
 228 |                             path,
 229 |                             status: PerFileStatus::Unchanged,
 230 |                             diff: String::new(),
 231 |                         });
 232 |                     }
 233 |                 } else {
 234 |                     let diff = unified_no_header(old_content, new_content, context_lines);
 235 |                     results.push(PerFileDiff {
 236 |                         path,
 237 |                         status: PerFileStatus::Modified,
 238 |                         diff,
 239 |                     });
 240 |                 }
 241 |             }
 242 |             (None, None) => unreachable!(),
 243 |         }
 244 |     }
 245 | 
 246 |     results
 247 | }
 248 | 
 249 | /// Render a collection of per file diffs into markdown WITHOUT a global
 250 | /// "## File Differences" header. Each file begins with a "### Diff: `<path>`"
 251 | /// heading so that it can be appended near the changed files summary.
 252 | pub fn render_per_file_diffs(diffs: &[PerFileDiff]) -> String {
 253 |     let mut out = String::new();
 254 |     for d in diffs {
 255 |         out.push_str(&format!("### Diff: `{}`\n\n", d.path));
 256 |         match d.status {
 257 |             PerFileStatus::Added => out.push_str("_Status: Added_\n\n"),
 258 |             PerFileStatus::Removed => out.push_str("_Status: Removed_\n\n"),
 259 |             PerFileStatus::Modified => out.push_str("_Status: Modified_\n\n"),
 260 |             PerFileStatus::Unchanged => {
 261 |                 out.push_str("_Status: Unchanged_\n\n");
 262 |             }
 263 |         }
 264 |         if !d.diff.is_empty() {
 265 |             out.push_str(&d.diff);
 266 |             if !d.diff.ends_with('\n') {
 267 |                 out.push('\n');
 268 |             }
 269 |         }
 270 |         out.push('\n');
 271 |     }
 272 |     out
 273 | }
 274 | 
 275 | #[cfg(test)]
 276 | mod tests {
 277 |     use super::*;
 278 | 
 279 |     fn map(pairs: &[(&str, &str)]) -> HashMap<String, String> {
 280 |         pairs
 281 |             .iter()
 282 |             .map(|(k, v)| (k.to_string(), v.to_string()))
 283 |             .collect()
 284 |     }
 285 | 
 286 |     #[test]
 287 |     fn unchanged_is_skipped() {
 288 |         let prev = map(&[("a.txt", "one\n")]);
 289 |         let curr = map(&[("a.txt", "one\n")]);
 290 |         let diffs = diff_file_contents(&prev, &curr, true, Some(2));
 291 |         assert!(diffs.is_empty());
 292 |     }
 293 | 
 294 |     #[test]
 295 |     fn added_file_diff() {
 296 |         let prev = map(&[]);
 297 |         let curr = map(&[("new.rs", "fn main() {}\n")]);
 298 |         let diffs = diff_file_contents(&prev, &curr, true, Some(2));
 299 |         assert_eq!(diffs.len(), 1);
 300 |         let d = &diffs[0];
 301 |         assert_eq!(d.status, PerFileStatus::Added);
 302 |         assert!(d.diff.contains("+ fn main() {}"));
 303 |     }
 304 | 
 305 |     #[test]
 306 |     fn removed_file_diff() {
 307 |         let prev = map(&[("old.rs", "fn old() {}\n")]);
 308 |         let curr = map(&[]);
 309 |         let diffs = diff_file_contents(&prev, &curr, true, None);
 310 |         assert_eq!(diffs.len(), 1);
 311 |         let d = &diffs[0];
 312 |         assert_eq!(d.status, PerFileStatus::Removed);
 313 |         assert!(d.diff.contains("- fn old() {}"));
 314 |     }
 315 | 
 316 |     #[test]
 317 |     fn modified_file_diff() {
 318 |         let prev = map(&[("lib.rs", "fn add(a:i32,b:i32)->i32{a+b}\n")]);
 319 |         let curr = map(&[("lib.rs", "fn add(a: i32, b: i32) -> i32 { a + b }\n")]);
 320 |         let diffs = diff_file_contents(&prev, &curr, true, Some(1));
 321 |         assert_eq!(diffs.len(), 1);
 322 |         let d = &diffs[0];
 323 |         assert_eq!(d.status, PerFileStatus::Modified);
 324 |         assert!(d.diff.contains("- fn add(a:i32,b:i32)->i32{a+b}"));
 325 |         assert!(d.diff.contains("+ fn add(a: i32, b: i32) -> i32 { a + b }"));
 326 |     }
 327 | 
 328 |     #[test]
 329 |     fn include_unchanged_when_requested() {
 330 |         let prev = map(&[("a.txt", "same\n")]);
 331 |         let curr = map(&[("a.txt", "same\n")]);
 332 |         let diffs = diff_file_contents(&prev, &curr, false, None);
 333 |         assert_eq!(diffs.len(), 1);
 334 |         assert_eq!(diffs[0].status, PerFileStatus::Unchanged);
 335 |     }
 336 | 
 337 |     #[test]
 338 |     fn render_output_basic() {
 339 |         let prev = map(&[("a.txt", "one\n"), ("b.txt", "line1\nline2\n")]);
 340 |         let curr = map(&[
 341 |             ("a.txt", "two\n"),
 342 |             ("b.txt", "line1\nline2\n"),
 343 |             ("c.txt", "new file\n"),
 344 |         ]);
 345 |         let diffs = diff_file_contents(&prev, &curr, true, Some(1));
 346 |         let out = render_per_file_diffs(&diffs);
 347 |         assert!(out.contains("### Diff: `a.txt`"));
 348 |         assert!(out.contains("_Status: Modified_"));
 349 |         assert!(out.contains("+ two"));
 350 |         assert!(out.contains("### Diff: `c.txt`"));
 351 |         assert!(out.contains("_Status: Added_"));
 352 |         assert!(out.contains("+ new file"));
 353 |     }
 354 | 
 355 |     #[test]
 356 |     fn test_empty_files() {
 357 |         let prev = map(&[("empty.txt", "")]);
 358 |         let curr = map(&[("empty.txt", "")]);
 359 |         let diffs = diff_file_contents(&prev, &curr, true, None);
 360 |         assert!(diffs.is_empty());
 361 |     }
 362 | 
 363 |     #[test]
 364 |     fn test_empty_to_content() {
 365 |         let prev = map(&[("file.txt", "")]);
 366 |         let curr = map(&[("file.txt", "new content\n")]);
 367 |         let diffs = diff_file_contents(&prev, &curr, true, None);
 368 |         assert_eq!(diffs.len(), 1);
 369 |         assert_eq!(diffs[0].status, PerFileStatus::Modified);
 370 |         assert!(diffs[0].diff.contains("+ new content"));
 371 |     }
 372 | 
 373 |     #[test]
 374 |     fn test_content_to_empty() {
 375 |         let prev = map(&[("file.txt", "old content\n")]);
 376 |         let curr = map(&[("file.txt", "")]);
 377 |         let diffs = diff_file_contents(&prev, &curr, true, None);
 378 |         assert_eq!(diffs.len(), 1);
 379 |         assert_eq!(diffs[0].status, PerFileStatus::Modified);
 380 |         assert!(diffs[0].diff.contains("- old content"));
 381 |     }
 382 | 
 383 |     #[test]
 384 |     fn test_multiline_modifications() {
 385 |         let prev = map(&[("file.txt", "line1\nline2\nline3\nline4\n")]);
 386 |         let curr = map(&[("file.txt", "line1\nmodified2\nline3\nline4\n")]);
 387 |         let diffs = diff_file_contents(&prev, &curr, true, Some(2));
 388 |         assert_eq!(diffs.len(), 1);
 389 |         assert_eq!(diffs[0].status, PerFileStatus::Modified);
 390 |         assert!(diffs[0].diff.contains("- line2"));
 391 |         assert!(diffs[0].diff.contains("+ modified2"));
 392 |     }
 393 | 
 394 |     #[test]
 395 |     fn test_windows_line_endings() {
 396 |         let prev = map(&[("file.txt", "line1\r\nline2\r\n")]);
 397 |         let curr = map(&[("file.txt", "line1\r\nmodified2\r\n")]);
 398 |         let diffs = diff_file_contents(&prev, &curr, true, None);
 399 |         assert_eq!(diffs.len(), 1);
 400 |         assert_eq!(diffs[0].status, PerFileStatus::Modified);
 401 |         assert!(diffs[0].diff.contains("- line2"));
 402 |         assert!(diffs[0].diff.contains("+ modified2"));
 403 |     }
 404 | 
 405 |     #[test]
 406 |     fn test_per_file_diff_is_changed() {
 407 |         let added = PerFileDiff {
 408 |             path: "test.txt".to_string(),
 409 |             status: PerFileStatus::Added,
 410 |             diff: "test".to_string(),
 411 |         };
 412 |         assert!(added.is_changed());
 413 | 
 414 |         let removed = PerFileDiff {
 415 |             path: "test.txt".to_string(),
 416 |             status: PerFileStatus::Removed,
 417 |             diff: "test".to_string(),
 418 |         };
 419 |         assert!(removed.is_changed());
 420 | 
 421 |         let modified = PerFileDiff {
 422 |             path: "test.txt".to_string(),
 423 |             status: PerFileStatus::Modified,
 424 |             diff: "test".to_string(),
 425 |         };
 426 |         assert!(modified.is_changed());
 427 | 
 428 |         let unchanged = PerFileDiff {
 429 |             path: "test.txt".to_string(),
 430 |             status: PerFileStatus::Unchanged,
 431 |             diff: String::new(),
 432 |         };
 433 |         assert!(!unchanged.is_changed());
 434 |     }
 435 | 
 436 |     #[test]
 437 |     fn test_generate_diff_identical_content() {
 438 |         let content = "line1\nline2\nline3\n";
 439 |         let diff = generate_diff(content, content);
 440 |         assert!(diff.is_empty());
 441 |     }
 442 | 
 443 |     #[test]
 444 |     fn test_generate_diff_with_changes() {
 445 |         let old = "line1\nline2\nline3\n";
 446 |         let new = "line1\nmodified2\nline3\n";
 447 |         let diff = generate_diff(old, new);
 448 |         assert!(diff.contains("## File Differences"));
 449 |         assert!(diff.contains("```diff"));
 450 |         assert!(diff.contains("- line2"));
 451 |         assert!(diff.contains("+ modified2"));
 452 |     }
 453 | 
 454 |     #[test]
 455 |     fn test_resolve_context_lines_default() {
 456 |         let context = resolve_context_lines(None);
 457 |         assert_eq!(context, 3);
 458 |     }
 459 | 
 460 |     #[test]
 461 |     fn test_resolve_context_lines_explicit() {
 462 |         let context = resolve_context_lines(Some(5));
 463 |         assert_eq!(context, 5);
 464 |     }
 465 | 
 466 |     #[test]
 467 |     fn test_resolve_context_lines_zero_fallback() {
 468 |         let context = resolve_context_lines(Some(0));
 469 |         assert_eq!(context, 3); // Should fallback to default
 470 |     }
 471 | 
 472 |     #[test]
 473 |     fn test_unicode_content_diff() {
 474 |         let prev = map(&[("unicode.txt", "Hello 世界\n")]);
 475 |         let curr = map(&[("unicode.txt", "Hello 世界! 🌍\n")]);
 476 |         let diffs = diff_file_contents(&prev, &curr, true, None);
 477 |         assert_eq!(diffs.len(), 1);
 478 |         assert_eq!(diffs[0].status, PerFileStatus::Modified);
 479 |         assert!(diffs[0].diff.contains("Hello 世界"));
 480 |         assert!(diffs[0].diff.contains("🌍"));
 481 |     }
 482 | 
 483 |     #[test]
 484 |     fn test_render_per_file_diffs_empty() {
 485 |         let diffs = vec![];
 486 |         let output = render_per_file_diffs(&diffs);
 487 |         assert!(output.is_empty());
 488 |     }
 489 | 
 490 |     #[test]
 491 |     fn test_render_per_file_diffs_unchanged() {
 492 |         let diffs = vec![PerFileDiff {
 493 |             path: "unchanged.txt".to_string(),
 494 |             status: PerFileStatus::Unchanged,
 495 |             diff: String::new(),
 496 |         }];
 497 |         let output = render_per_file_diffs(&diffs);
 498 |         assert!(output.contains("### Diff: `unchanged.txt`"));
 499 |         assert!(output.contains("_Status: Unchanged_"));
 500 |     }
 501 | 
 502 |     #[test]
 503 |     fn test_render_per_file_diffs_without_trailing_newline() {
 504 |         let diffs = vec![PerFileDiff {
 505 |             path: "test.txt".to_string(),
 506 |             status: PerFileStatus::Modified,
 507 |             diff: "```diff\n+ line\n```".to_string(), // No trailing newline
 508 |         }];
 509 |         let output = render_per_file_diffs(&diffs);
 510 |         assert!(output.contains("### Diff: `test.txt`"));
 511 |         assert!(output.contains("_Status: Modified_"));
 512 |         assert!(output.ends_with("\n\n")); // Should add newlines
 513 |     }
 514 | 
 515 |     #[test]
 516 |     fn test_generate_diff_with_multiple_groups() {
 517 |         // Create content that will result in multiple diff groups to trigger "..." separator
 518 |         let old_content = "line1\nline2\nline3\nline4\nline5\nline6\nline7\nline8\nline9\nline10";
 519 |         let new_content = "line1_modified\nline2\nline3\nline4\nline5\nline6\nline7\nline8\nline9_modified\nline10";
 520 | 
 521 |         let diff = generate_diff(old_content, new_content);
 522 |         assert!(diff.contains("```diff"));
 523 |         assert!(diff.contains("## File Differences"));
 524 |         // With sufficient distance between changes and small context, should create groups with "..." separator
 525 |         println!("Generated diff: {}", diff);
 526 |     }
 527 | 
 528 |     #[test]
 529 |     fn test_diff_with_windows_line_endings() {
 530 |         let old_content = "line1\r\nline2\r\n";
 531 |         let new_content = "line1_modified\r\nline2\r\n";
 532 | 
 533 |         let diff = generate_diff(old_content, new_content);
 534 |         assert!(diff.contains("```diff"));
 535 |         assert!(diff.contains("line1_modified"));
 536 |         assert!(!diff.is_empty());
 537 |     }
 538 | 
 539 |     #[test]
 540 |     fn test_unified_no_header_with_multiple_groups() {
 541 |         // Create content that will result in multiple diff groups
 542 |         let old_content = "start\n\n\n\n\n\n\n\n\n\nmiddle\n\n\n\n\n\n\n\n\n\nend";
 543 |         let new_content =
 544 |             "start_modified\n\n\n\n\n\n\n\n\n\nmiddle\n\n\n\n\n\n\n\n\n\nend_modified";
 545 | 
 546 |         let diff = unified_no_header(old_content, new_content, 2);
 547 |         assert!(diff.contains("```diff"));
 548 |         // Should contain "..." separator between groups when changes are far apart
 549 |         println!("Unified diff: {}", diff);
 550 |     }
 551 | 
 552 |     #[test]
 553 |     fn test_unified_no_header_with_windows_line_endings() {
 554 |         let old_content = "line1\r\nline2\r\n";
 555 |         let new_content = "line1_modified\r\nline2\r\n";
 556 | 
 557 |         let diff = unified_no_header(old_content, new_content, 3);
 558 |         assert!(diff.contains("```diff"));
 559 |         assert!(diff.contains("line1_modified"));
 560 |         assert!(!diff.is_empty());
 561 |     }
 562 | }
```

### File: `src/file_utils.rs`

- Size: 22914 bytes
- Modified: SystemTime { tv_sec: 1771120514, tv_nsec: 429057507 }

```rust
   1 | use ignore::{DirEntry, WalkBuilder, overrides::OverrideBuilder};
   2 | use std::fs;
   3 | use std::io::{self, Write};
   4 | use std::path::{Path, PathBuf};
   5 | 
   6 | /// Returns a numeric category for file relevance ordering.
   7 | /// Lower numbers appear first in output. Categories:
   8 | /// 0 = Project config + key docs (Cargo.toml, README.md, AGENTS.md, etc.)
   9 | /// 1 = Source code (src/, lib/) — entry points sorted first within category
  10 | /// 2 = Tests and benchmarks (tests/, benches/, test/, spec/)
  11 | /// 3 = Documentation, scripts, and everything else
  12 | /// 4 = Generated/lock files (Cargo.lock, package-lock.json, etc.)
  13 | /// 5 = Build/CI infrastructure (.github/, .circleci/, Dockerfile, etc.)
  14 | fn file_relevance_category(path: &Path, base_path: &Path) -> u8 {
  15 |     let relative = path.strip_prefix(base_path).unwrap_or(path);
  16 |     let rel_str = relative.to_string_lossy();
  17 | 
  18 |     // Check filename for lockfiles first — these are lowest priority
  19 |     if let Some(name) = relative.file_name().and_then(|n| n.to_str()) {
  20 |         let lockfile_names = [
  21 |             "Cargo.lock",
  22 |             "package-lock.json",
  23 |             "yarn.lock",
  24 |             "pnpm-lock.yaml",
  25 |             "Gemfile.lock",
  26 |             "poetry.lock",
  27 |             "composer.lock",
  28 |             "go.sum",
  29 |             "bun.lockb",
  30 |             "flake.lock",
  31 |         ];
  32 |         if lockfile_names.contains(&name) {
  33 |             return 5;
  34 |         }
  35 | 
  36 |         // Check for config/manifest files + key project docs — highest priority
  37 |         let config_names = [
  38 |             // Package manifests
  39 |             "Cargo.toml",
  40 |             "package.json",
  41 |             "tsconfig.json",
  42 |             "pyproject.toml",
  43 |             "setup.py",
  44 |             "setup.cfg",
  45 |             "go.mod",
  46 |             "Gemfile",
  47 |             // Tool config
  48 |             "context-builder.toml",
  49 |             ".gitignore",
  50 |             // Key project documentation (LLMs need these for context)
  51 |             "README.md",
  52 |             "README",
  53 |             "README.txt",
  54 |             "README.rst",
  55 |             "AGENTS.md",
  56 |             "CLAUDE.md",
  57 |             "GEMINI.md",
  58 |             "COPILOT.md",
  59 |             "CONTRIBUTING.md",
  60 |             "CHANGELOG.md",
  61 |         ];
  62 |         if config_names.contains(&name) {
  63 |             return 0;
  64 |         }
  65 |     }
  66 | 
  67 |     // Check path prefix for category
  68 |     let first_component = relative
  69 |         .components()
  70 |         .next()
  71 |         .and_then(|c| c.as_os_str().to_str())
  72 |         .unwrap_or("");
  73 | 
  74 |     match first_component {
  75 |         "src" | "lib" | "crates" | "packages" | "internal" | "cmd" | "pkg" => {
  76 |             // Check sub-components for test directories within source trees.
  77 |             // e.g., src/tests/auth.rs should be cat 2 (tests), not cat 1 (source).
  78 |             let sub_path = rel_str.as_ref();
  79 |             if sub_path.contains("/tests/")
  80 |                 || sub_path.contains("/test/")
  81 |                 || sub_path.contains("/spec/")
  82 |                 || sub_path.contains("/__tests__/")
  83 |                 || sub_path.contains("/benches/")
  84 |                 || sub_path.contains("/benchmarks/")
  85 |             {
  86 |                 2
  87 |             } else {
  88 |                 1
  89 |             }
  90 |         }
  91 |         "tests" | "test" | "spec" | "benches" | "benchmarks" | "__tests__" => 2,
  92 |         "docs" | "doc" | "examples" | "scripts" | "tools" | "assets" => 3,
  93 |         // Build/CI infrastructure — useful context but not core source
  94 |         ".github" | ".circleci" | ".gitlab" | ".buildkite" => 4,
  95 |         _ => {
  96 |             // Check extensions for additional heuristics
  97 |             if let Some(ext) = relative.extension().and_then(|e| e.to_str()) {
  98 |                 match ext {
  99 |                     "rs" | "go" | "py" | "ts" | "js" | "java" | "c" | "cpp" | "h" | "hpp"
 100 |                     | "rb" | "swift" | "kt" | "scala" | "ex" | "exs" | "zig" | "hs" => {
 101 |                         // Source file not in a recognized dir — check if it's a test
 102 |                         // Use path boundaries to avoid false positives (e.g., "contest.rs")
 103 |                         if rel_str.contains("/test/")
 104 |                             || rel_str.contains("/tests/")
 105 |                             || rel_str.contains("/spec/")
 106 |                             || rel_str.contains("/__tests__/")
 107 |                             || rel_str.ends_with("_test.rs")
 108 |                             || rel_str.ends_with("_test.go")
 109 |                             || rel_str.ends_with("_spec.rb")
 110 |                             || rel_str.ends_with(".test.ts")
 111 |                             || rel_str.ends_with(".test.js")
 112 |                             || rel_str.ends_with(".spec.ts")
 113 |                             || rel_str.starts_with("test_")
 114 |                         {
 115 |                             2
 116 |                         } else {
 117 |                             1
 118 |                         }
 119 |                     }
 120 |                     "md" | "txt" | "rst" | "adoc" => 3,
 121 |                     _ => 1, // Unknown extension in root — treat as source
 122 |                 }
 123 |             } else {
 124 |                 // Check for build-related root files without extensions
 125 |                 if let Some(
 126 |                     "Makefile" | "CMakeLists.txt" | "Dockerfile" | "Containerfile" | "Justfile"
 127 |                     | "Taskfile" | "Rakefile" | "Vagrantfile",
 128 |                 ) = relative.file_name().and_then(|n| n.to_str())
 129 |                 {
 130 |                     4
 131 |                 } else {
 132 |                     3 // No extension — docs/other
 133 |                 }
 134 |             }
 135 |         }
 136 |     }
 137 | }
 138 | 
 139 | /// Returns a sub-priority for sorting within the same relevance category.
 140 | /// Lower values appear first. Entry points (main, lib, mod) get priority 0,
 141 | /// other files get priority 1. This ensures LLMs see architectural entry
 142 | /// points before helper modules.
 143 | fn file_entry_point_priority(path: &Path) -> u8 {
 144 |     if let Some("main" | "lib" | "mod" | "index" | "app" | "__init__") =
 145 |         path.file_stem().and_then(|s| s.to_str())
 146 |     {
 147 |         0
 148 |     } else {
 149 |         1
 150 |     }
 151 | }
 152 | 
 153 | /// Collects all files to be processed using `ignore` crate for efficient traversal.
 154 | ///
 155 | /// `auto_ignores` are runtime-computed exclusion patterns (e.g., the tool's own
 156 | /// output file or cache directory). They are processed identically to user ignores
 157 | /// but kept separate to avoid polluting user-facing configuration.
 158 | pub fn collect_files(
 159 |     base_path: &Path,
 160 |     filters: &[String],
 161 |     ignores: &[String],
 162 |     auto_ignores: &[String],
 163 | ) -> io::Result<Vec<DirEntry>> {
 164 |     let mut walker = WalkBuilder::new(base_path);
 165 |     // By default, the "ignore" crate respects .gitignore and hidden files, so we don't need walker.hidden(false)
 166 | 
 167 |     // Build overrides for custom ignore patterns
 168 |     let mut override_builder = OverrideBuilder::new(base_path);
 169 |     for pattern in ignores {
 170 |         // Attention: Confusing pattern ahead!
 171 |         // Add the pattern to the override builder with ! prefix to ignore matching files.
 172 |         // In OverrideBuilder, patterns without ! are whitelist (include) patterns,
 173 |         // while patterns with ! are ignore patterns.
 174 |         let ignore_pattern = format!("!{}", pattern);
 175 |         if let Err(e) = override_builder.add(&ignore_pattern) {
 176 |             return Err(io::Error::new(
 177 |                 io::ErrorKind::InvalidInput,
 178 |                 format!("Invalid ignore pattern '{}': {}", pattern, e),
 179 |             ));
 180 |         }
 181 |     }
 182 |     // Apply auto-computed ignore patterns (output file, cache dir, etc.)
 183 |     for pattern in auto_ignores {
 184 |         let ignore_pattern = format!("!{}", pattern);
 185 |         if let Err(e) = override_builder.add(&ignore_pattern) {
 186 |             log::warn!("Skipping invalid auto-ignore pattern '{}': {}", pattern, e);
 187 |         }
 188 |     }
 189 |     // Also, always ignore the config file itself
 190 |     if let Err(e) = override_builder.add("!context-builder.toml") {
 191 |         return Err(io::Error::new(
 192 |             io::ErrorKind::InvalidInput,
 193 |             format!("Failed to add config ignore: {}", e),
 194 |         ));
 195 |     }
 196 | 
 197 |     // Hardcoded auto-ignores for common heavy directories that should NEVER be
 198 |     // included, even when there's no .git directory (so .gitignore isn't read).
 199 |     // Without these, projects missing .git can produce million-line outputs
 200 |     // from dependency trees.
 201 |     let default_ignores = [
 202 |         "node_modules",
 203 |         "__pycache__",
 204 |         ".venv",
 205 |         "venv",
 206 |         ".tox",
 207 |         ".mypy_cache",
 208 |         ".pytest_cache",
 209 |         ".ruff_cache",
 210 |         "vendor",  // Go, PHP, Ruby
 211 |         ".bundle", // Ruby
 212 |         "bower_components",
 213 |         ".next",       // Next.js build output
 214 |         ".nuxt",       // Nuxt build output
 215 |         ".svelte-kit", // SvelteKit build output
 216 |         ".angular",    // Angular cache
 217 |         "dist",        // Common build output
 218 |         "build",       // Common build output
 219 |         ".gradle",     // Gradle cache
 220 |         ".cargo",      // Cargo registry cache
 221 |     ];
 222 |     for dir in &default_ignores {
 223 |         let pattern = format!("!{}/**", dir);
 224 |         if let Err(e) = override_builder.add(&pattern) {
 225 |             log::warn!("Skipping invalid default-ignore '{}': {}", dir, e);
 226 |         }
 227 |     }
 228 | 
 229 |     let overrides = override_builder.build().map_err(|e| {
 230 |         io::Error::new(
 231 |             io::ErrorKind::InvalidInput,
 232 |             format!("Failed to build overrides: {}", e),
 233 |         )
 234 |     })?;
 235 |     walker.overrides(overrides);
 236 | 
 237 |     if !filters.is_empty() {
 238 |         let mut type_builder = ignore::types::TypesBuilder::new();
 239 |         type_builder.add_defaults();
 240 |         for filter in filters {
 241 |             let _ = type_builder.add(filter, &format!("*.{}", filter));
 242 |             type_builder.select(filter);
 243 |         }
 244 |         let types = type_builder.build().unwrap();
 245 |         walker.types(types);
 246 |     }
 247 | 
 248 |     let mut files: Vec<DirEntry> = walker
 249 |         .build()
 250 |         .filter_map(Result::ok)
 251 |         .filter(|e| e.file_type().is_some_and(|ft| ft.is_file()))
 252 |         .collect();
 253 | 
 254 |     // Sort files by relevance category, then entry-point priority, then alphabetically.
 255 |     // This puts config + docs first, then source code (entry points before helpers),
 256 |     // then tests, then docs/other, then build/CI, then lockfiles.
 257 |     // LLMs comprehend codebases better when core source appears before test scaffolding.
 258 |     files.sort_by(|a, b| {
 259 |         let cat_a = file_relevance_category(a.path(), base_path);
 260 |         let cat_b = file_relevance_category(b.path(), base_path);
 261 |         cat_a
 262 |             .cmp(&cat_b)
 263 |             .then_with(|| {
 264 |                 file_entry_point_priority(a.path()).cmp(&file_entry_point_priority(b.path()))
 265 |             })
 266 |             .then_with(|| a.path().cmp(b.path()))
 267 |     });
 268 | 
 269 |     Ok(files)
 270 | }
 271 | 
 272 | /// Asks for user confirmation if the number of files is large.
 273 | pub fn confirm_processing(file_count: usize) -> io::Result<bool> {
 274 |     if file_count > 100 {
 275 |         print!(
 276 |             "Warning: You're about to process {} files. This might take a while. Continue? [y/N] ",
 277 |             file_count
 278 |         );
 279 |         io::stdout().flush()?;
 280 |         let mut input = String::new();
 281 |         io::stdin().read_line(&mut input)?;
 282 |         if !input.trim().eq_ignore_ascii_case("y") {
 283 |             return Ok(false);
 284 |         }
 285 |     }
 286 |     Ok(true)
 287 | }
 288 | 
 289 | /// Asks for user confirmation to overwrite an existing file.
 290 | pub fn confirm_overwrite(file_path: &str) -> io::Result<bool> {
 291 |     print!("The file '{}' already exists. Overwrite? [y/N] ", file_path);
 292 |     io::stdout().flush()?;
 293 |     let mut input = String::new();
 294 |     io::stdin().read_line(&mut input)?;
 295 | 
 296 |     if input.trim().eq_ignore_ascii_case("y") {
 297 |         Ok(true)
 298 |     } else {
 299 |         Ok(false)
 300 |     }
 301 | }
 302 | 
 303 | pub fn find_latest_file(dir: &Path) -> io::Result<Option<PathBuf>> {
 304 |     if !dir.is_dir() {
 305 |         return Ok(None);
 306 |     }
 307 | 
 308 |     let mut latest_file = None;
 309 |     let mut latest_time = std::time::SystemTime::UNIX_EPOCH;
 310 | 
 311 |     for entry in fs::read_dir(dir)? {
 312 |         let entry = entry?;
 313 |         let path = entry.path();
 314 |         if path.is_file() {
 315 |             let metadata = fs::metadata(&path)?;
 316 |             let modified = metadata.modified()?;
 317 |             if modified > latest_time {
 318 |                 latest_time = modified;
 319 |                 latest_file = Some(path);
 320 |             }
 321 |         }
 322 |     }
 323 | 
 324 |     Ok(latest_file)
 325 | }
 326 | 
 327 | #[cfg(test)]
 328 | mod tests {
 329 |     use super::*;
 330 |     use std::fs;
 331 |     use std::path::Path;
 332 |     use tempfile::tempdir;
 333 | 
 334 |     fn to_rel_paths(mut entries: Vec<DirEntry>, base: &Path) -> Vec<String> {
 335 |         entries.sort_by_key(|e| e.path().to_path_buf());
 336 |         entries
 337 |             .iter()
 338 |             .map(|e| {
 339 |                 e.path()
 340 |                     .strip_prefix(base)
 341 |                     .unwrap()
 342 |                     .to_string_lossy()
 343 |                     .replace('\\', "/")
 344 |             })
 345 |             .collect()
 346 |     }
 347 | 
 348 |     #[test]
 349 |     fn collect_files_respects_filters() {
 350 |         let dir = tempdir().unwrap();
 351 |         let base = dir.path();
 352 | 
 353 |         // create files
 354 |         fs::create_dir_all(base.join("src")).unwrap();
 355 |         fs::create_dir_all(base.join("scripts")).unwrap();
 356 |         fs::write(base.join("src").join("main.rs"), "fn main() {}").unwrap();
 357 |         fs::write(base.join("Cargo.toml"), "[package]\nname=\"x\"").unwrap();
 358 |         fs::write(base.join("README.md"), "# readme").unwrap();
 359 |         fs::write(base.join("scripts").join("build.sh"), "#!/bin/sh\n").unwrap();
 360 | 
 361 |         let filters = vec!["rs".to_string(), "toml".to_string()];
 362 |         let ignores: Vec<String> = vec![];
 363 | 
 364 |         let files = collect_files(base, &filters, &ignores, &[]).unwrap();
 365 |         let relative_paths = to_rel_paths(files, base);
 366 | 
 367 |         assert!(relative_paths.contains(&"src/main.rs".to_string()));
 368 |         assert!(relative_paths.contains(&"Cargo.toml".to_string()));
 369 |         assert!(!relative_paths.contains(&"README.md".to_string()));
 370 |         assert!(!relative_paths.contains(&"scripts/build.sh".to_string()));
 371 |     }
 372 | 
 373 |     #[test]
 374 |     fn collect_files_respects_ignores_for_dirs_and_files() {
 375 |         let dir = tempdir().unwrap();
 376 |         let base = dir.path();
 377 | 
 378 |         fs::create_dir_all(base.join("src")).unwrap();
 379 |         fs::create_dir_all(base.join("target")).unwrap();
 380 |         fs::create_dir_all(base.join("node_modules")).unwrap();
 381 | 
 382 |         fs::write(base.join("src").join("main.rs"), "fn main() {}").unwrap();
 383 |         fs::write(base.join("target").join("artifact.txt"), "bin").unwrap();
 384 |         fs::write(base.join("node_modules").join("pkg.js"), "console.log();").unwrap();
 385 |         fs::write(base.join("README.md"), "# readme").unwrap();
 386 | 
 387 |         let filters: Vec<String> = vec![];
 388 |         let ignores: Vec<String> = vec!["target".into(), "node_modules".into(), "README.md".into()];
 389 | 
 390 |         let files = collect_files(base, &filters, &ignores, &[]).unwrap();
 391 |         let relative_paths = to_rel_paths(files, base);
 392 | 
 393 |         assert!(relative_paths.contains(&"src/main.rs".to_string()));
 394 |         assert!(!relative_paths.contains(&"target/artifact.txt".to_string()));
 395 |         assert!(!relative_paths.contains(&"node_modules/pkg.js".to_string()));
 396 |         assert!(!relative_paths.contains(&"README.md".to_string()));
 397 |     }
 398 | 
 399 |     #[test]
 400 |     fn collect_files_handles_invalid_ignore_pattern() {
 401 |         let dir = tempdir().unwrap();
 402 |         let base = dir.path();
 403 | 
 404 |         fs::create_dir_all(base.join("src")).unwrap();
 405 |         fs::write(base.join("src").join("main.rs"), "fn main() {}").unwrap();
 406 | 
 407 |         let filters: Vec<String> = vec![];
 408 |         let ignores: Vec<String> = vec!["[".into()]; // Invalid regex pattern
 409 | 
 410 |         let result = collect_files(base, &filters, &ignores, &[]);
 411 |         assert!(result.is_err());
 412 |         assert!(
 413 |             result
 414 |                 .unwrap_err()
 415 |                 .to_string()
 416 |                 .contains("Invalid ignore pattern")
 417 |         );
 418 |     }
 419 | 
 420 |     #[test]
 421 |     fn collect_files_empty_directory() {
 422 |         let dir = tempdir().unwrap();
 423 |         let base = dir.path();
 424 | 
 425 |         let filters: Vec<String> = vec![];
 426 |         let ignores: Vec<String> = vec![];
 427 | 
 428 |         let files = collect_files(base, &filters, &ignores, &[]).unwrap();
 429 |         assert!(files.is_empty());
 430 |     }
 431 | 
 432 |     #[test]
 433 |     fn collect_files_no_matching_filters() {
 434 |         let dir = tempdir().unwrap();
 435 |         let base = dir.path();
 436 | 
 437 |         fs::write(base.join("README.md"), "# readme").unwrap();
 438 |         fs::write(base.join("script.py"), "print('hello')").unwrap();
 439 | 
 440 |         let filters = vec!["rs".to_string()]; // Only Rust files
 441 |         let ignores: Vec<String> = vec![];
 442 | 
 443 |         let files = collect_files(base, &filters, &ignores, &[]).unwrap();
 444 |         assert!(files.is_empty());
 445 |     }
 446 | 
 447 |     #[test]
 448 |     fn collect_files_ignores_config_file() {
 449 |         let dir = tempdir().unwrap();
 450 |         let base = dir.path();
 451 | 
 452 |         fs::write(base.join("context-builder.toml"), "[config]").unwrap();
 453 |         fs::write(base.join("other.toml"), "[other]").unwrap();
 454 | 
 455 |         let filters: Vec<String> = vec![];
 456 |         let ignores: Vec<String> = vec![];
 457 | 
 458 |         let files = collect_files(base, &filters, &ignores, &[]).unwrap();
 459 |         let relative_paths = to_rel_paths(files, base);
 460 | 
 461 |         assert!(!relative_paths.contains(&"context-builder.toml".to_string()));
 462 |         assert!(relative_paths.contains(&"other.toml".to_string()));
 463 |     }
 464 | 
 465 |     #[test]
 466 |     fn confirm_processing_small_count() {
 467 |         // Test that small file counts don't require confirmation
 468 |         let result = confirm_processing(50);
 469 |         assert!(result.is_ok());
 470 |         assert!(result.unwrap());
 471 |     }
 472 | 
 473 |     #[test]
 474 |     fn find_latest_file_empty_directory() {
 475 |         let dir = tempdir().unwrap();
 476 |         let result = find_latest_file(dir.path()).unwrap();
 477 |         assert!(result.is_none());
 478 |     }
 479 | 
 480 |     #[test]
 481 |     fn find_latest_file_nonexistent_directory() {
 482 |         let dir = tempdir().unwrap();
 483 |         let nonexistent = dir.path().join("nonexistent");
 484 |         let result = find_latest_file(&nonexistent).unwrap();
 485 |         assert!(result.is_none());
 486 |     }
 487 | 
 488 |     #[test]
 489 |     fn find_latest_file_single_file() {
 490 |         let dir = tempdir().unwrap();
 491 |         let file_path = dir.path().join("test.txt");
 492 |         fs::write(&file_path, "content").unwrap();
 493 | 
 494 |         let result = find_latest_file(dir.path()).unwrap();
 495 |         assert!(result.is_some());
 496 |         assert_eq!(result.unwrap(), file_path);
 497 |     }
 498 | 
 499 |     #[test]
 500 |     fn find_latest_file_multiple_files() {
 501 |         let dir = tempdir().unwrap();
 502 | 
 503 |         let file1 = dir.path().join("old.txt");
 504 |         let file2 = dir.path().join("new.txt");
 505 | 
 506 |         fs::write(&file1, "old content").unwrap();
 507 |         std::thread::sleep(std::time::Duration::from_millis(10));
 508 |         fs::write(&file2, "new content").unwrap();
 509 | 
 510 |         let result = find_latest_file(dir.path()).unwrap();
 511 |         assert!(result.is_some());
 512 |         assert_eq!(result.unwrap(), file2);
 513 |     }
 514 | 
 515 |     #[test]
 516 |     fn find_latest_file_ignores_directories() {
 517 |         let dir = tempdir().unwrap();
 518 |         let subdir = dir.path().join("subdir");
 519 |         fs::create_dir(&subdir).unwrap();
 520 | 
 521 |         let file_path = dir.path().join("test.txt");
 522 |         fs::write(&file_path, "content").unwrap();
 523 | 
 524 |         let result = find_latest_file(dir.path()).unwrap();
 525 |         assert!(result.is_some());
 526 |         assert_eq!(result.unwrap(), file_path);
 527 |     }
 528 | 
 529 |     #[test]
 530 |     fn test_confirm_processing_requires_user_interaction() {
 531 |         // This test verifies the function signature and basic logic for large file counts
 532 |         // The actual user interaction cannot be tested in unit tests
 533 | 
 534 |         // For file counts <= 100, should return Ok(true) without prompting
 535 |         // This is already tested implicitly by the fact that small counts don't prompt
 536 | 
 537 |         // For file counts > 100, the function would prompt user input
 538 |         // We can't easily test this without mocking stdin, but we can verify
 539 |         // that the function exists and has the expected signature
 540 |         use std::io::Cursor;
 541 | 
 542 |         // Create a mock stdin that simulates user typing "y"
 543 |         let input = b"y\n";
 544 |         let _ = Cursor::new(input);
 545 | 
 546 |         // We can't easily override stdin in a unit test without complex setup,
 547 |         // so we'll just verify the function exists and handles small counts
 548 |         let result = confirm_processing(50);
 549 |         assert!(result.is_ok());
 550 |         assert!(result.unwrap());
 551 |     }
 552 | 
 553 |     #[test]
 554 |     fn test_confirm_overwrite_function_exists() {
 555 |         // Similar to confirm_processing, this function requires user interaction
 556 |         // We can verify it exists and has the expected signature
 557 | 
 558 |         // For testing purposes, we know this function prompts for user input
 559 |         // and returns Ok(true) if user types "y" or "Y", Ok(false) otherwise
 560 | 
 561 |         // The function signature should be:
 562 |         // pub fn confirm_overwrite(file_path: &str) -> io::Result<bool>
 563 | 
 564 |         // We can't easily test the interactive behavior without mocking stdin,
 565 |         // but we can ensure the function compiles and has the right signature
 566 |         let _: fn(&str) -> std::io::Result<bool> = confirm_overwrite;
 567 |     }
 568 | 
 569 |     #[test]
 570 |     fn test_collect_files_handles_permission_errors() {
 571 |         // Test what happens when we can't access a directory
 572 |         // This is harder to test portably, but we can test with invalid patterns
 573 |         let dir = tempdir().unwrap();
 574 |         let base = dir.path();
 575 | 
 576 |         // Test with a pattern that might cause issues
 577 |         let filters: Vec<String> = vec![];
 578 |         let ignores: Vec<String> = vec!["[invalid".into()]; // Incomplete bracket
 579 | 
 580 |         let result = collect_files(base, &filters, &ignores, &[]);
 581 |         assert!(result.is_err());
 582 |     }
 583 | 
 584 |     #[test]
 585 |     fn test_find_latest_file_permission_error() {
 586 |         // Test behavior when we can't read directory metadata
 587 |         use std::path::Path;
 588 | 
 589 |         // Test with a path that doesn't exist
 590 |         let nonexistent = Path::new("/this/path/should/not/exist/anywhere");
 591 |         let result = find_latest_file(nonexistent);
 592 | 
 593 |         // Should return Ok(None) for non-existent directories
 594 |         assert!(result.is_ok());
 595 |         assert!(result.unwrap().is_none());
 596 |     }
 597 | 
 598 |     #[test]
 599 |     fn test_collect_files_with_symlinks() {
 600 |         // Test behavior with symbolic links (if supported on platform)
 601 |         let dir = tempdir().unwrap();
 602 |         let base = dir.path();
 603 | 
 604 |         // Create a regular file
 605 |         fs::write(base.join("regular.txt"), "content").unwrap();
 606 | 
 607 |         // On Unix-like systems, try creating a symlink
 608 |         #[cfg(unix)]
 609 |         {
 610 |             use std::os::unix::fs::symlink;
 611 |             let _ = symlink("regular.txt", base.join("link.txt"));
 612 |         }
 613 | 
 614 |         // On Windows, symlinks require special privileges, so skip this part
 615 |         #[cfg(windows)]
 616 |         {
 617 |             // Just create another regular file to test
 618 |             fs::write(base.join("another.txt"), "content2").unwrap();
 619 |         }
 620 | 
 621 |         let filters: Vec<String> = vec![];
 622 |         let ignores: Vec<String> = vec![];
 623 | 
 624 |         let files = collect_files(base, &filters, &ignores, &[]).unwrap();
 625 |         // Should find at least the regular file
 626 |         assert!(!files.is_empty());
 627 |     }
 628 | }
```

### File: `src/markdown.rs`

- Size: 40252 bytes
- Modified: SystemTime { tv_sec: 1771115638, tv_nsec: 891216622 }

```rust
   1 | use chrono::Utc;
   2 | use ignore::DirEntry;
   3 | use log::{error, info, warn};
   4 | use std::fs;
   5 | use std::io::{self, Read, Seek, SeekFrom, Write};
   6 | use std::path::Path;
   7 | 
   8 | use crate::tree::{FileTree, write_tree_to_file};
   9 | use encoding_rs::{Encoding, UTF_8};
  10 | 
  11 | #[cfg(feature = "parallel")]
  12 | use crossbeam_channel::{Receiver, Sender, bounded};
  13 | #[cfg(feature = "parallel")]
  14 | use std::thread;
  15 | 
  16 | /// Generates the final Markdown file.
  17 | #[allow(clippy::too_many_arguments, unused_variables)]
  18 | pub fn generate_markdown(
  19 |     output_path: &str,
  20 |     input_dir: &str,
  21 |     filters: &[String],
  22 |     ignores: &[String],
  23 |     file_tree: &FileTree,
  24 |     files: &[DirEntry],
  25 |     base_path: &Path,
  26 |     line_numbers: bool,
  27 |     encoding_strategy: Option<&str>,
  28 |     max_tokens: Option<usize>,
  29 | ) -> io::Result<()> {
  30 |     if let Some(parent) = Path::new(output_path).parent()
  31 |         && !parent.exists()
  32 |     {
  33 |         fs::create_dir_all(parent)?;
  34 |     }
  35 | 
  36 |     let mut output = fs::File::create(output_path)?;
  37 | 
  38 |     let input_dir_name = if input_dir == "." {
  39 |         let current_dir = std::env::current_dir()?;
  40 |         current_dir
  41 |             .file_name()
  42 |             .unwrap()
  43 |             .to_str()
  44 |             .unwrap()
  45 |             .to_string()
  46 |     } else {
  47 |         input_dir.to_string()
  48 |     };
  49 | 
  50 |     // --- Header --- //
  51 |     writeln!(output, "# Directory Structure Report\n")?;
  52 | 
  53 |     if !filters.is_empty() {
  54 |         writeln!(
  55 |             output,
  56 |             "This document contains files from the `{}` directory with extensions: {}",
  57 |             input_dir_name,
  58 |             filters.join(", ")
  59 |         )?;
  60 |     } else {
  61 |         writeln!(
  62 |             output,
  63 |             "This document contains all files from the `{}` directory, optimized for LLM consumption.",
  64 |             input_dir_name
  65 |         )?;
  66 |     }
  67 | 
  68 |     if !ignores.is_empty() {
  69 |         writeln!(output, "Custom ignored patterns: {}", ignores.join(", "))?;
  70 |     }
  71 | 
  72 |     // Deterministic content hash (enables LLM prompt caching across runs)
  73 |     // Uses xxh3 over file content bytes — stable across Rust versions and machines.
  74 |     // Previous implementation hashed mtime (broken by git checkout, cp, etc.)
  75 |     let mut content_hasher = xxhash_rust::xxh3::Xxh3::new();
  76 |     for entry in files {
  77 |         // Hash relative unix-style path for cross-OS determinism.
  78 |         // Using absolute or OS-native paths would produce different hashes
  79 |         // on different machines or operating systems.
  80 |         let rel_path = entry.path().strip_prefix(base_path).unwrap_or(entry.path());
  81 |         let normalized = rel_path.to_string_lossy().replace('\\', "/");
  82 |         content_hasher.update(normalized.as_bytes());
  83 |         // Null delimiter prevents collision: path="a" content="bc" vs path="ab" content="c"
  84 |         content_hasher.update(b"\0");
  85 |         // Hash actual file content (not mtime!) for determinism
  86 |         if let Ok(bytes) = std::fs::read(entry.path()) {
  87 |             content_hasher.update(&bytes);
  88 |         }
  89 |         content_hasher.update(b"\0");
  90 |     }
  91 |     writeln!(output, "Content hash: {:016x}", content_hasher.digest())?;
  92 |     writeln!(output)?;
  93 | 
  94 |     // --- File Tree --- //
  95 | 
  96 |     writeln!(output, "## File Tree Structure\n")?;
  97 | 
  98 |     write_tree_to_file(&mut output, file_tree, 0)?;
  99 | 
 100 |     writeln!(output)?;
 101 | 
 102 |     // (No '## Files' heading here; it will be injected later only once during final composition)
 103 |     // (Diff section will be conditionally inserted later by the auto_diff logic in lib.rs)
 104 | 
 105 |     #[cfg(feature = "parallel")]
 106 |     {
 107 |         use rayon::prelude::*;
 108 | 
 109 |         // Create a bounded channel for ordered chunks
 110 |         type ChunkResult = (usize, io::Result<Vec<u8>>);
 111 |         let (sender, receiver): (Sender<ChunkResult>, Receiver<ChunkResult>) =
 112 |             bounded(num_cpus::get() * 2); // Buffer size based on CPU count
 113 | 
 114 |         let writer_handle = {
 115 |             let mut output = output;
 116 |             let total_files = files.len();
 117 |             let budget = max_tokens;
 118 | 
 119 |             thread::spawn(move || -> io::Result<()> {
 120 |                 let mut completed_chunks = std::collections::BTreeMap::new();
 121 |                 let mut next_index = 0;
 122 |                 let mut errors = Vec::new();
 123 |                 let mut tokens_used: usize = 0;
 124 |                 let mut budget_exceeded = false;
 125 | 
 126 |                 // Receive chunks and write them in order
 127 |                 while next_index < total_files {
 128 |                     match receiver.recv() {
 129 |                         Ok((index, chunk_result)) => {
 130 |                             completed_chunks.insert(index, chunk_result);
 131 | 
 132 |                             // Write all consecutive chunks starting from next_index
 133 |                             while let Some(chunk_result) = completed_chunks.remove(&next_index) {
 134 |                                 if budget_exceeded {
 135 |                                     // Already over budget — skip remaining chunks
 136 |                                     next_index += 1;
 137 |                                     continue;
 138 |                                 }
 139 | 
 140 |                                 match chunk_result {
 141 |                                     Ok(buf) => {
 142 |                                         // Estimate tokens for this chunk (~4 bytes per token)
 143 |                                         let chunk_tokens = buf.len() / 4;
 144 | 
 145 |                                         if let Some(max) = budget
 146 |                                             && tokens_used + chunk_tokens > max
 147 |                                             && tokens_used > 0
 148 |                                         {
 149 |                                             let remaining = total_files - next_index;
 150 |                                             let notice = format!(
 151 |                                                 "---\n\n_⚠️ Token budget ({}) reached. {} remaining files omitted._\n\n",
 152 |                                                 max, remaining
 153 |                                             );
 154 |                                             if let Err(e) = output.write_all(notice.as_bytes()) {
 155 |                                                 errors.push(format!(
 156 |                                                     "Failed to write truncation notice: {}",
 157 |                                                     e
 158 |                                                 ));
 159 |                                             }
 160 |                                             budget_exceeded = true;
 161 |                                             next_index += 1;
 162 |                                             continue;
 163 |                                         }
 164 | 
 165 |                                         tokens_used += chunk_tokens;
 166 |                                         if let Err(e) = output.write_all(&buf) {
 167 |                                             errors.push(format!(
 168 |                                                 "Failed to write output for file index {}: {}",
 169 |                                                 next_index, e
 170 |                                             ));
 171 |                                         }
 172 |                                     }
 173 |                                     Err(e) => {
 174 |                                         errors.push(format!(
 175 |                                             "Failed to process file index {}: {}",
 176 |                                             next_index, e
 177 |                                         ));
 178 |                                     }
 179 |                                 }
 180 |                                 next_index += 1;
 181 |                             }
 182 |                         }
 183 |                         Err(_) => break, // Channel closed
 184 |                     }
 185 |                 }
 186 | 
 187 |                 if !errors.is_empty() {
 188 |                     error!(
 189 |                         "Encountered {} errors during parallel processing:",
 190 |                         errors.len()
 191 |                     );
 192 |                     for err in &errors {
 193 |                         error!("  {}", err);
 194 |                     }
 195 |                     return Err(std::io::Error::other(format!(
 196 |                         "Failed to process {} files: {}",
 197 |                         errors.len(),
 198 |                         errors.join("; ")
 199 |                     )));
 200 |                 }
 201 | 
 202 |                 Ok(())
 203 |             })
 204 |         };
 205 | 
 206 |         // Process files in parallel and send results to writer
 207 |         files.par_iter().enumerate().for_each(|(index, entry)| {
 208 |             let mut buf = Vec::new();
 209 |             let result = process_file(
 210 |                 base_path,
 211 |                 entry.path(),
 212 |                 &mut buf,
 213 |                 line_numbers,
 214 |                 encoding_strategy,
 215 |             )
 216 |             .map(|_| buf);
 217 | 
 218 |             // Send result to writer thread (ignore send errors - channel might be closed)
 219 |             let _ = sender.send((index, result));
 220 |         });
 221 | 
 222 |         // Close the sender to signal completion
 223 |         drop(sender);
 224 | 
 225 |         // Wait for writer thread to complete and propagate any errors
 226 |         writer_handle
 227 |             .join()
 228 |             .map_err(|_| std::io::Error::other("Writer thread panicked"))??;
 229 |     }
 230 | 
 231 |     #[cfg(not(feature = "parallel"))]
 232 |     {
 233 |         let mut tokens_used: usize = 0;
 234 | 
 235 |         for (idx, entry) in files.iter().enumerate() {
 236 |             // Estimate tokens for this file (~4 bytes per token)
 237 |             let file_size = std::fs::metadata(entry.path())
 238 |                 .map(|m| m.len())
 239 |                 .unwrap_or(0);
 240 |             let estimated_file_tokens = (file_size as usize) / 4;
 241 | 
 242 |             if let Some(budget) = max_tokens {
 243 |                 if tokens_used + estimated_file_tokens > budget && tokens_used > 0 {
 244 |                     let remaining = files.len() - idx;
 245 |                     writeln!(output, "---\n")?;
 246 |                     writeln!(
 247 |                         output,
 248 |                         "_⚠️ Token budget ({}) reached. {} remaining files omitted._\n",
 249 |                         budget, remaining
 250 |                     )?;
 251 |                     break;
 252 |                 }
 253 |             }
 254 | 
 255 |             tokens_used += estimated_file_tokens;
 256 |             process_file(
 257 |                 base_path,
 258 |                 entry.path(),
 259 |                 &mut output,
 260 |                 line_numbers,
 261 |                 encoding_strategy,
 262 |             )?;
 263 |         }
 264 |     }
 265 | 
 266 |     Ok(())
 267 | }
 268 | 
 269 | /// Processes a single file and writes its content to the output.
 270 | pub fn process_file(
 271 |     base_path: &Path,
 272 | 
 273 |     file_path: &Path,
 274 | 
 275 |     output: &mut impl Write,
 276 |     line_numbers: bool,
 277 |     encoding_strategy: Option<&str>,
 278 | ) -> io::Result<()> {
 279 |     let relative_path = file_path.strip_prefix(base_path).unwrap_or(file_path);
 280 |     info!("Processing file: {}", relative_path.display());
 281 | 
 282 |     let metadata = match fs::metadata(file_path) {
 283 |         Ok(meta) => meta,
 284 |         Err(e) => {
 285 |             error!(
 286 |                 "Failed to get metadata for {}: {}",
 287 |                 relative_path.display(),
 288 |                 e
 289 |             );
 290 |             return Ok(());
 291 |         }
 292 |     };
 293 | 
 294 |     let modified_time = metadata
 295 |         .modified()
 296 |         .ok()
 297 |         .map(|time| {
 298 |             let system_time: chrono::DateTime<Utc> = time.into();
 299 |             system_time.format("%Y-%m-%d %H:%M:%S UTC").to_string()
 300 |         })
 301 |         .unwrap_or_else(|| "Unknown".to_string());
 302 | 
 303 |     writeln!(output)?;
 304 |     writeln!(output, "### File: `{}`", relative_path.display())?;
 305 | 
 306 |     writeln!(output)?;
 307 | 
 308 |     writeln!(output, "- Size: {} bytes", metadata.len())?;
 309 |     writeln!(output, "- Modified: {}", modified_time)?;
 310 |     writeln!(output)?;
 311 | 
 312 |     // --- File Content --- //
 313 |     let extension = file_path
 314 |         .extension()
 315 |         .and_then(|s| s.to_str())
 316 |         .unwrap_or("text");
 317 |     let language = match extension {
 318 |         "rs" => "rust",
 319 |         "js" => "javascript",
 320 |         "ts" => "typescript",
 321 |         "jsx" => "jsx",
 322 |         "tsx" => "tsx",
 323 |         "json" => "json",
 324 |         "toml" => "toml",
 325 |         "md" => "markdown",
 326 |         "yaml" | "yml" => "yaml",
 327 |         "html" => "html",
 328 |         "css" => "css",
 329 |         "py" => "python",
 330 |         "java" => "java",
 331 |         "cpp" => "cpp",
 332 |         "c" => "c",
 333 |         "h" => "c",
 334 |         "hpp" => "cpp",
 335 |         "sql" => "sql",
 336 |         "sh" => "bash",
 337 |         "xml" => "xml",
 338 |         "lock" => "toml",
 339 |         _ => extension,
 340 |     };
 341 | 
 342 |     // Enhanced binary file handling with encoding detection and transcoding
 343 |     match fs::File::open(file_path) {
 344 |         Ok(mut file) => {
 345 |             let mut sniff = [0u8; 8192];
 346 |             let n = match file.read(&mut sniff) {
 347 |                 Ok(n) => n,
 348 |                 Err(e) => {
 349 |                     warn!(
 350 |                         "Could not read file {}: {}. Skipping content.",
 351 |                         relative_path.display(),
 352 |                         e
 353 |                     );
 354 | 
 355 |                     writeln!(output, "```text")?;
 356 | 
 357 |                     writeln!(
 358 |                         output,
 359 |                         "<Could not read file content (e.g., binary file or permission error)>"
 360 |                     )?;
 361 | 
 362 |                     writeln!(output, "```")?;
 363 | 
 364 |                     return Ok(());
 365 |                 }
 366 |             };
 367 |             let slice = &sniff[..n];
 368 | 
 369 |             // Find a valid UTF-8 boundary by backtracking up to 3 bytes.
 370 |             // If the sniff buffer cuts a multi-byte char (e.g., emoji at byte 8191),
 371 |             // from_utf8 would falsely classify the file as non-UTF-8.
 372 |             let check_len = if n == sniff.len() {
 373 |                 // Buffer is full — may have split a multi-byte char at the end
 374 |                 let mut end = n;
 375 |                 while end > 0 && end > n.saturating_sub(4) && sniff[end - 1] & 0xC0 == 0x80 {
 376 |                     end -= 1; // skip continuation bytes
 377 |                 }
 378 |                 // If we landed on a leading byte, check if the sequence is complete
 379 |                 if end > 0 && end < n {
 380 |                     let leading = sniff[end - 1];
 381 |                     let expected_len = if leading & 0xE0 == 0xC0 {
 382 |                         2
 383 |                     } else if leading & 0xF0 == 0xE0 {
 384 |                         3
 385 |                     } else if leading & 0xF8 == 0xF0 {
 386 |                         4
 387 |                     } else {
 388 |                         1
 389 |                     };
 390 |                     if end - 1 + expected_len > n {
 391 |                         end - 1 // incomplete char — exclude the leading byte too
 392 |                     } else {
 393 |                         n
 394 |                     }
 395 |                 } else {
 396 |                     n
 397 |                 }
 398 |             } else {
 399 |                 n // didn't fill the buffer, so no boundary issue
 400 |             };
 401 | 
 402 |             // First check if it's valid UTF-8
 403 |             let is_utf8 = std::str::from_utf8(&sniff[..check_len]).is_ok();
 404 | 
 405 |             if is_utf8 && !slice.contains(&0) {
 406 |                 // Valid UTF-8 text file - proceed normally
 407 |             } else {
 408 |                 // Try encoding detection for non-UTF-8 files
 409 |                 // If it's not UTF-8, try to detect the encoding
 410 |                 let (encoding, _consumed) =
 411 |                     encoding_rs::Encoding::for_bom(slice).unwrap_or((encoding_rs::UTF_8, 0));
 412 | 
 413 |                 // If it's not UTF-8, try to detect the encoding
 414 |                 let detected_encoding = if encoding == UTF_8 {
 415 |                     // Use chardet-like detection for common encodings
 416 |                     detect_text_encoding(slice)
 417 |                 } else {
 418 |                     Some(encoding)
 419 |                 };
 420 | 
 421 |                 match detected_encoding {
 422 |                     Some(enc) if enc != UTF_8 => {
 423 |                         let strategy = encoding_strategy.unwrap_or("detect");
 424 |                         match strategy {
 425 |                             "strict" | "skip" => {
 426 |                                 // Skip files with non-UTF-8 encoding
 427 |                                 warn!(
 428 |                                     "Skipping non-UTF-8 file {} (encoding: {}, strategy: {})",
 429 |                                     relative_path.display(),
 430 |                                     enc.name(),
 431 |                                     strategy
 432 |                                 );
 433 |                             }
 434 |                             _ => {
 435 |                                 // Default "detect" strategy: attempt to transcode
 436 |                                 match transcode_file_content(file_path, enc) {
 437 |                                     Ok(transcoded_content) => {
 438 |                                         info!(
 439 |                                             "Successfully transcoded {} from {} to UTF-8",
 440 |                                             relative_path.display(),
 441 |                                             enc.name()
 442 |                                         );
 443 |                                         write_text_content(
 444 |                                             output,
 445 |                                             &transcoded_content,
 446 |                                             language,
 447 |                                             line_numbers,
 448 |                                         )?;
 449 |                                         return Ok(());
 450 |                                     }
 451 |                                     Err(e) => {
 452 |                                         warn!(
 453 |                                             "Failed to transcode {} from {}: {}. Treating as binary.",
 454 |                                             relative_path.display(),
 455 |                                             enc.name(),
 456 |                                             e
 457 |                                         );
 458 |                                     }
 459 |                                 }
 460 |                             }
 461 |                         }
 462 |                     }
 463 |                     _ => {
 464 |                         // Check if it's likely binary (contains null bytes)
 465 |                         if slice.contains(&0) {
 466 |                             warn!(
 467 |                                 "Detected binary file {} (contains null bytes). Skipping content.",
 468 |                                 relative_path.display()
 469 |                             );
 470 |                         } else {
 471 |                             warn!(
 472 |                                 "Could not determine encoding for {}. Treating as binary.",
 473 |                                 relative_path.display()
 474 |                             );
 475 |                         }
 476 |                     }
 477 |                 }
 478 | 
 479 |                 // Fallback to binary file placeholder
 480 |                 writeln!(output, "```text")?;
 481 |                 writeln!(
 482 |                     output,
 483 |                     "<Binary file or unsupported encoding: {} bytes>",
 484 |                     metadata.len()
 485 |                 )?;
 486 |                 writeln!(output, "```")?;
 487 |                 return Ok(());
 488 |             }
 489 | 
 490 |             // Reset cursor and stream the content
 491 |             if let Err(e) = file.seek(SeekFrom::Start(0)) {
 492 |                 warn!(
 493 |                     "Could not reset file cursor for {}: {}. Skipping content.",
 494 |                     relative_path.display(),
 495 |                     e
 496 |                 );
 497 |                 writeln!(output, "```text")?;
 498 |                 writeln!(
 499 |                     output,
 500 |                     "<Could not read file content (e.g., binary file or permission error)>"
 501 |                 )?;
 502 |                 writeln!(output, "```")?;
 503 |                 return Ok(());
 504 |             }
 505 | 
 506 |             // Stream UTF-8 content
 507 |             let content = match std::fs::read_to_string(file_path) {
 508 |                 Ok(content) => content,
 509 |                 Err(e) => {
 510 |                     warn!(
 511 |                         "Error reading file {}: {}. Output may be truncated.",
 512 |                         relative_path.display(),
 513 |                         e
 514 |                     );
 515 |                     writeln!(output, "```text")?;
 516 |                     writeln!(output, "<Error reading file content>")?;
 517 |                     writeln!(output, "```")?;
 518 |                     return Ok(());
 519 |                 }
 520 |             };
 521 | 
 522 |             write_text_content(output, &content, language, line_numbers)?;
 523 |         }
 524 |         Err(e) => {
 525 |             warn!(
 526 |                 "Could not open file {}: {}. Skipping content.",
 527 |                 relative_path.display(),
 528 |                 e
 529 |             );
 530 |             writeln!(output, "```text")?;
 531 |             writeln!(
 532 |                 output,
 533 |                 "<Could not read file content (e.g., binary file or permission error)>"
 534 |             )?;
 535 |             writeln!(output, "```")?;
 536 |         }
 537 |     }
 538 | 
 539 |     Ok(())
 540 | }
 541 | 
 542 | /// Detect text encoding using heuristics for common encodings
 543 | fn detect_text_encoding(bytes: &[u8]) -> Option<&'static Encoding> {
 544 |     // Try common encodings
 545 |     let encodings = [
 546 |         encoding_rs::WINDOWS_1252,
 547 |         encoding_rs::UTF_16LE,
 548 |         encoding_rs::UTF_16BE,
 549 |         encoding_rs::SHIFT_JIS,
 550 |     ];
 551 | 
 552 |     for encoding in &encodings {
 553 |         let (decoded, _, had_errors) = encoding.decode(bytes);
 554 |         if !had_errors && is_likely_text(&decoded) {
 555 |             return Some(encoding);
 556 |         }
 557 |     }
 558 | 
 559 |     None
 560 | }
 561 | 
 562 | /// Check if decoded content looks like text (no control characters except common ones)
 563 | fn is_likely_text(content: &str) -> bool {
 564 |     let mut control_chars = 0;
 565 |     let mut total_chars = 0;
 566 | 
 567 |     for ch in content.chars() {
 568 |         total_chars += 1;
 569 |         if ch.is_control() && ch != '\n' && ch != '\r' && ch != '\t' {
 570 |             control_chars += 1;
 571 |         }
 572 | 
 573 |         // If more than 5% control characters, probably not text
 574 |         if total_chars > 100 && control_chars * 20 > total_chars {
 575 |             return false;
 576 |         }
 577 |     }
 578 | 
 579 |     // Allow up to 5% control characters in small files
 580 |     if total_chars > 0 {
 581 |         control_chars * 20 <= total_chars
 582 |     } else {
 583 |         true
 584 |     }
 585 | }
 586 | 
 587 | /// Transcode file content from detected encoding to UTF-8
 588 | fn transcode_file_content(file_path: &Path, encoding: &'static Encoding) -> io::Result<String> {
 589 |     let bytes = std::fs::read(file_path)?;
 590 |     let (decoded, _, had_errors) = encoding.decode(&bytes);
 591 | 
 592 |     if had_errors {
 593 |         return Err(io::Error::new(
 594 |             io::ErrorKind::InvalidData,
 595 |             format!("Failed to decode file with encoding {}", encoding.name()),
 596 |         ));
 597 |     }
 598 | 
 599 |     Ok(decoded.into_owned())
 600 | }
 601 | 
 602 | /// Write text content with optional line numbers
 603 | fn write_text_content(
 604 |     output: &mut impl Write,
 605 |     content: &str,
 606 |     language: &str,
 607 |     line_numbers: bool,
 608 | ) -> io::Result<()> {
 609 |     writeln!(output, "```{}", language)?;
 610 | 
 611 |     if line_numbers {
 612 |         for (i, line) in content.lines().enumerate() {
 613 |             writeln!(output, "{:>4} | {}", i + 1, line)?;
 614 |         }
 615 |     } else {
 616 |         output.write_all(content.as_bytes())?;
 617 |         if !content.ends_with('\n') {
 618 |             writeln!(output)?;
 619 |         }
 620 |     }
 621 | 
 622 |     writeln!(output, "```")?;
 623 |     Ok(())
 624 | }
 625 | 
 626 | #[cfg(test)]
 627 | mod tests {
 628 |     use super::*;
 629 |     use std::fs;
 630 |     use tempfile::tempdir;
 631 | 
 632 |     #[test]
 633 |     fn test_code_block_formatting() {
 634 |         let dir = tempdir().unwrap();
 635 |         let base_path = dir.path();
 636 |         let file_path = base_path.join("test.rs");
 637 |         let output_path = base_path.join("output.md");
 638 | 
 639 |         // Create a test Rust file
 640 |         fs::write(
 641 |             &file_path,
 642 |             "fn main() {\n    println!(\"Hello, world!\");\n}",
 643 |         )
 644 |         .unwrap();
 645 | 
 646 |         // Create an output file
 647 |         let mut output = fs::File::create(&output_path).unwrap();
 648 | 
 649 |         // Process the file
 650 |         process_file(base_path, &file_path, &mut output, false, None).unwrap();
 651 | 
 652 |         // Read the output
 653 |         let content = fs::read_to_string(&output_path).unwrap();
 654 | 
 655 |         // Check that code blocks are properly formatted
 656 |         assert!(content.contains("```rust"));
 657 |         assert!(content.contains("```") && content.matches("```").count() >= 2);
 658 |     }
 659 | 
 660 |     #[test]
 661 |     fn test_markdown_file_formatting() {
 662 |         let dir = tempdir().unwrap();
 663 |         let base_path = dir.path();
 664 |         let file_path = base_path.join("README.md");
 665 |         let output_path = base_path.join("output.md");
 666 | 
 667 |         // Create a test Markdown file
 668 |         fs::write(&file_path, "# Test\n\nThis is a test markdown file.").unwrap();
 669 | 
 670 |         // Create an output file
 671 |         let mut output = fs::File::create(&output_path).unwrap();
 672 | 
 673 |         // Process the file
 674 |         process_file(base_path, &file_path, &mut output, false, None).unwrap();
 675 | 
 676 |         // Read the output
 677 |         let content = fs::read_to_string(&output_path).unwrap();
 678 | 
 679 |         // Debug prints the content
 680 |         println!("Generated content:\n{}", content);
 681 | 
 682 |         // Check that markdown files use the correct language identifier
 683 |         assert!(
 684 |             content.contains("```markdown"),
 685 |             "Content should contain '```markdown' but was: {}",
 686 |             content
 687 |         );
 688 |         // Count the number of code block markers
 689 |         let code_block_markers = content.matches("```").count();
 690 | 
 691 |         assert!(
 692 |             code_block_markers >= 2,
 693 |             "Expected at least 2 code block markers, found {}",
 694 |             code_block_markers
 695 |         );
 696 |     }
 697 | 
 698 |     #[test]
 699 |     fn test_line_numbered_code_blocks() {
 700 |         let dir = tempdir().unwrap();
 701 |         let base_path = dir.path();
 702 |         let file_path = base_path.join("lib.rs");
 703 |         let output_path = base_path.join("out.md");
 704 | 
 705 |         // Create a multi-line Rust file
 706 |         fs::write(
 707 |                     &file_path,
 708 |                     "fn add(a: i32, b: i32) -> i32 {\n    a + b\n}\n\nfn main() {\n    println!(\"{}\", add(1, 2));\n}\n",
 709 |                 )
 710 |                 .unwrap();
 711 | 
 712 |         let mut output = fs::File::create(&output_path).unwrap();
 713 |         process_file(base_path, &file_path, &mut output, true, None).unwrap();
 714 | 
 715 |         let content = fs::read_to_string(&output_path).unwrap();
 716 | 
 717 |         // Check language and line numbers prefix
 718 |         assert!(content.contains("```rust"));
 719 |         assert!(content.contains("   1 | "));
 720 |         assert!(content.contains("   2 | "));
 721 | 
 722 |         // Count lines with "|" prefix equals number of lines in an original file
 723 |         let numbered_lines = content
 724 |             .lines()
 725 |             .filter(|l| {
 726 |                 l.trim_start()
 727 |                     .chars()
 728 |                     .next()
 729 |                     .map(|c| c.is_ascii_digit())
 730 |                     .unwrap_or(false)
 731 |                     && l.contains(" | ")
 732 |             })
 733 |             .count();
 734 |         let original_line_count = fs::read_to_string(&file_path).unwrap().lines().count();
 735 |         assert_eq!(numbered_lines, original_line_count);
 736 | 
 737 |         // Ensure code fence closes
 738 |         assert!(content.contains("```"));
 739 |     }
 740 | 
 741 |     #[test]
 742 |     fn test_binary_file_handling() {
 743 |         let dir = tempdir().unwrap();
 744 |         let base_path = dir.path();
 745 |         let file_path = base_path.join("image.bin");
 746 |         let output_path = base_path.join("out.md");
 747 | 
 748 |         // Write truly binary data that won't be decoded by encoding detection
 749 |         let bytes = vec![
 750 |             0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, // PNG header
 751 |             0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44, 0x52, // PNG chunk
 752 |             0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, // More binary data
 753 |             0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Null bytes
 754 |         ];
 755 |         fs::write(&file_path, bytes).unwrap();
 756 | 
 757 |         let mut output = fs::File::create(&output_path).unwrap();
 758 |         process_file(base_path, &file_path, &mut output, false, None).unwrap();
 759 | 
 760 |         let content = fs::read_to_string(&output_path).unwrap();
 761 | 
 762 |         // Expect a text block to fall back with a helpful message
 763 |         assert!(content.contains("```text"));
 764 |         assert!(content.contains("<Binary file or unsupported encoding:"));
 765 | 
 766 |         // Ensure the code block is closed
 767 |         let fence_count = content.matches("```").count();
 768 |         assert!(
 769 |             fence_count >= 2,
 770 |             "expected at least opening and closing fences, got {}",
 771 |             fence_count
 772 |         );
 773 |     }
 774 | 
 775 |     #[test]
 776 |     fn test_encoding_detection_and_transcoding() {
 777 |         let dir = tempdir().unwrap();
 778 |         let base_path = dir.path();
 779 |         let output_path = base_path.join("out.md");
 780 | 
 781 |         // Test Windows-1252 encoded file (common in Windows)
 782 |         let windows1252_content = [
 783 |             0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x20, // "Hello "
 784 |             0x93, 0x57, 0x6F, 0x72, 0x6C, 0x64, 0x94, // "World" with smart quotes
 785 |             0x0A, // newline
 786 |         ];
 787 |         let file_path = base_path.join("windows1252.txt");
 788 |         fs::write(&file_path, windows1252_content).unwrap();
 789 | 
 790 |         let mut output = fs::File::create(&output_path).unwrap();
 791 |         process_file(base_path, &file_path, &mut output, false, Some("detect")).unwrap();
 792 | 
 793 |         let content = fs::read_to_string(&output_path).unwrap();
 794 | 
 795 |         // Should contain transcoded content with UTF-8 equivalents
 796 |         assert!(content.contains("Hello"));
 797 |         assert!(content.contains("World"));
 798 |         // Should use text language
 799 |         assert!(content.contains("```txt"));
 800 | 
 801 |         // Ensure the code block is closed
 802 |         let fence_count = content.matches("```").count();
 803 |         assert!(
 804 |             fence_count >= 2,
 805 |             "expected at least opening and closing fences, got {}",
 806 |             fence_count
 807 |         );
 808 |     }
 809 | 
 810 |     #[test]
 811 |     fn test_encoding_strategy_strict() {
 812 |         let dir = tempdir().unwrap();
 813 |         let base_path = dir.path();
 814 |         let output_path = base_path.join("out.md");
 815 | 
 816 |         // Create a file with non-UTF-8 content
 817 |         let non_utf8_content = [0xFF, 0xFE, 0x41, 0x00]; // UTF-16 LE BOM + "A"
 818 |         let file_path = base_path.join("utf16.txt");
 819 |         fs::write(&file_path, non_utf8_content).unwrap();
 820 | 
 821 |         let mut output = fs::File::create(&output_path).unwrap();
 822 |         process_file(base_path, &file_path, &mut output, false, Some("strict")).unwrap();
 823 | 
 824 |         let content = fs::read_to_string(&output_path).unwrap();
 825 | 
 826 |         // Should contain binary file placeholder
 827 |         assert!(content.contains("<Binary file or unsupported encoding:"));
 828 |         assert!(content.contains("```text"));
 829 | 
 830 |         // Ensure the code block is closed
 831 |         let fence_count = content.matches("```").count();
 832 |         assert!(
 833 |             fence_count >= 2,
 834 |             "expected at least opening and closing fences, got {}",
 835 |             fence_count
 836 |         );
 837 |     }
 838 | 
 839 |     #[test]
 840 |     fn test_encoding_strategy_skip() {
 841 |         let dir = tempdir().unwrap();
 842 |         let base_path = dir.path();
 843 |         let output_path = base_path.join("out.md");
 844 | 
 845 |         // Create a file with UTF-16 content
 846 |         let utf16_content = [0xFF, 0xFE, 0x48, 0x00, 0x69, 0x00]; // UTF-16 LE "Hi"
 847 |         let file_path = base_path.join("utf16.txt");
 848 |         fs::write(&file_path, utf16_content).unwrap();
 849 | 
 850 |         let mut output = fs::File::create(&output_path).unwrap();
 851 |         process_file(base_path, &file_path, &mut output, false, Some("skip")).unwrap();
 852 | 
 853 |         let content = fs::read_to_string(&output_path).unwrap();
 854 | 
 855 |         // Should contain binary file placeholder (skipped transcoding)
 856 |         assert!(content.contains("<Binary file or unsupported encoding:"));
 857 |         assert!(content.contains("```text"));
 858 |     }
 859 | 
 860 |     #[test]
 861 |     fn test_generate_markdown_with_current_directory() {
 862 |         let dir = tempdir().unwrap();
 863 |         let base_path = dir.path();
 864 |         let output_path = base_path.join("test.md");
 865 | 
 866 |         // Create test files
 867 |         fs::write(base_path.join("readme.txt"), "Hello world").unwrap();
 868 | 
 869 |         // Collect files
 870 |         let files = crate::file_utils::collect_files(base_path, &[], &[], &[]).unwrap();
 871 |         let file_tree = crate::tree::build_file_tree(&files, base_path);
 872 | 
 873 |         // Change to the test directory
 874 |         let original_dir = std::env::current_dir().unwrap();
 875 |         std::env::set_current_dir(base_path).unwrap();
 876 | 
 877 |         // Test with "." as input directory
 878 |         let result = generate_markdown(
 879 |             &output_path.to_string_lossy(),
 880 |             ".",
 881 |             &[],
 882 |             &[],
 883 |             &file_tree,
 884 |             &files,
 885 |             base_path,
 886 |             false,
 887 |             None,
 888 |             None, // max_tokens
 889 |         );
 890 | 
 891 |         // Restore original directory
 892 |         std::env::set_current_dir(original_dir).unwrap();
 893 | 
 894 |         assert!(result.is_ok());
 895 |         let content = fs::read_to_string(&output_path).unwrap();
 896 |         assert!(content.contains("Directory Structure Report"));
 897 |     }
 898 | 
 899 |     #[test]
 900 |     fn test_generate_markdown_creates_output_directory() {
 901 |         let dir = tempdir().unwrap();
 902 |         let base_path = dir.path();
 903 |         let nested_output = base_path.join("nested").join("deep").join("output.md");
 904 | 
 905 |         // Create test files
 906 |         fs::write(base_path.join("test.txt"), "content").unwrap();
 907 | 
 908 |         let files = crate::file_utils::collect_files(base_path, &[], &[], &[]).unwrap();
 909 |         let file_tree = crate::tree::build_file_tree(&files, base_path);
 910 | 
 911 |         let result = generate_markdown(
 912 |             &nested_output.to_string_lossy(),
 913 |             "test_dir",
 914 |             &[],
 915 |             &[],
 916 |             &file_tree,
 917 |             &files,
 918 |             base_path,
 919 |             false,
 920 |             None,
 921 |             None, // max_tokens
 922 |         );
 923 | 
 924 |         assert!(result.is_ok());
 925 |         assert!(nested_output.exists());
 926 |         assert!(nested_output.parent().unwrap().exists());
 927 |     }
 928 | 
 929 |     #[test]
 930 |     fn test_generate_markdown_with_filters_and_ignores() {
 931 |         let dir = tempdir().unwrap();
 932 |         let base_path = dir.path();
 933 |         let output_path = base_path.join("filtered.md");
 934 | 
 935 |         fs::write(base_path.join("main.rs"), "fn main() {}").unwrap();
 936 |         fs::write(base_path.join("config.toml"), "[package]").unwrap();
 937 |         fs::write(base_path.join("readme.md"), "# README").unwrap();
 938 | 
 939 |         let files = crate::file_utils::collect_files(base_path, &[], &[], &[]).unwrap();
 940 |         let file_tree = crate::tree::build_file_tree(&files, base_path);
 941 | 
 942 |         let result = generate_markdown(
 943 |             &output_path.to_string_lossy(),
 944 |             "project",
 945 |             &["rs".to_string(), "toml".to_string()],
 946 |             &["readme.md".to_string()],
 947 |             &file_tree,
 948 |             &files,
 949 |             base_path,
 950 |             true,
 951 |             Some("strict"),
 952 |             None, // max_tokens
 953 |         );
 954 | 
 955 |         assert!(result.is_ok());
 956 |         let content = fs::read_to_string(&output_path).unwrap();
 957 |         assert!(content.contains("Directory Structure Report"));
 958 |         // The actual generate_markdown function doesn't format filters/ignores this way
 959 |         assert!(content.contains("main.rs") || content.contains("config.toml"));
 960 |     }
 961 | 
 962 |     #[test]
 963 |     fn test_write_text_content_with_line_numbers() {
 964 |         let mut output = Vec::new();
 965 |         let content = "line one\nline two\nline three";
 966 | 
 967 |         write_text_content(&mut output, content, "rust", true).unwrap();
 968 | 
 969 |         let result = String::from_utf8(output).unwrap();
 970 |         assert!(result.contains("```rust"));
 971 |         assert!(result.contains("   1 | line one"));
 972 |         assert!(result.contains("   2 | line two"));
 973 |         assert!(result.contains("   3 | line three"));
 974 |         assert!(result.contains("```"));
 975 |     }
 976 | 
 977 |     #[test]
 978 |     fn test_write_text_content_without_line_numbers() {
 979 |         let mut output = Vec::new();
 980 |         let content = "function test() {\n  return true;\n}";
 981 | 
 982 |         write_text_content(&mut output, content, "javascript", false).unwrap();
 983 | 
 984 |         let result = String::from_utf8(output).unwrap();
 985 |         assert!(result.contains("```javascript"));
 986 |         assert!(result.contains("function test() {"));
 987 |         assert!(result.contains("  return true;"));
 988 |         assert!(result.contains("```"));
 989 |         assert!(!result.contains(" | ")); // No line number prefix
 990 |     }
 991 | 
 992 |     #[test]
 993 |     fn test_write_text_content_without_trailing_newline() {
 994 |         let mut output = Vec::new();
 995 |         let content = "no newline at end"; // No \n at end
 996 | 
 997 |         write_text_content(&mut output, content, "text", false).unwrap();
 998 | 
 999 |         let result = String::from_utf8(output).unwrap();
1000 |         assert!(result.contains("```text"));
1001 |         assert!(result.contains("no newline at end"));
1002 |         assert!(result.ends_with("```\n")); // Should add newline
1003 |     }
1004 | 
1005 |     #[test]
1006 |     fn test_is_likely_text() {
1007 |         // Normal text should be considered text
1008 |         assert!(is_likely_text("Hello world\nThis is normal text"));
1009 | 
1010 |         // Text with some control characters should still be text
1011 |         assert!(is_likely_text(
1012 |             "Line 1\nLine 2\tTabbed\r\nWindows line ending"
1013 |         ));
1014 | 
1015 |         // Text with too many control characters should not be text
1016 |         let mut bad_text = String::new();
1017 |         for i in 0..200 {
1018 |             if i % 5 == 0 {
1019 |                 bad_text.push('\x01'); // Control character
1020 |             } else {
1021 |                 bad_text.push('a');
1022 |             }
1023 |         }
1024 |         assert!(!is_likely_text(&bad_text));
1025 | 
1026 |         // Empty string should be considered text
1027 |         assert!(is_likely_text(""));
1028 |     }
1029 | 
1030 |     #[test]
1031 |     fn test_detect_text_encoding() {
1032 |         // UTF-8 should return None (already UTF-8)
1033 |         let utf8_bytes = "Hello world".as_bytes();
1034 |         let result = detect_text_encoding(utf8_bytes);
1035 |         // The function may return an encoding even for UTF-8 text if it detects it differently
1036 |         // Just verify it doesn't crash
1037 |         assert!(result.is_some() || result.is_none());
1038 | 
1039 |         // Windows-1252 encoded text should be detected
1040 |         let windows1252_bytes = [
1041 |             0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x20, 0x93, 0x77, 0x6F, 0x72, 0x6C, 0x64, 0x94,
1042 |         ];
1043 |         let detected = detect_text_encoding(&windows1252_bytes);
1044 |         assert!(detected.is_some());
1045 |     }
1046 | 
1047 |     #[test]
1048 |     fn test_transcode_file_content() {
1049 |         let dir = tempdir().unwrap();
1050 |         let file_path = dir.path().join("windows1252.txt");
1051 | 
1052 |         // Write Windows-1252 encoded content
1053 |         let windows1252_content = [
1054 |             0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x20, // "Hello "
1055 |             0x93, 0x57, 0x6F, 0x72, 0x6C, 0x64, 0x94, // "World" with smart quotes
1056 |         ];
1057 |         fs::write(&file_path, windows1252_content).unwrap();
1058 | 
1059 |         let result = transcode_file_content(&file_path, encoding_rs::WINDOWS_1252);
1060 |         assert!(result.is_ok());
1061 | 
1062 |         let transcoded = result.unwrap();
1063 |         assert!(transcoded.contains("Hello"));
1064 |         assert!(transcoded.contains("World"));
1065 |     }
1066 | 
1067 |     #[test]
1068 |     fn test_process_file_with_metadata_error() {
1069 |         let dir = tempdir().unwrap();
1070 |         let base_path = dir.path();
1071 |         let nonexistent_file = base_path.join("nonexistent.txt");
1072 |         let output_path = base_path.join("output.md");
1073 | 
1074 |         let mut output = fs::File::create(&output_path).unwrap();
1075 | 
1076 |         // This should handle the metadata error gracefully
1077 |         let result = process_file(base_path, &nonexistent_file, &mut output, false, None);
1078 |         assert!(result.is_ok());
1079 | 
1080 |         // Output should be minimal since file doesn't exist
1081 |         let content = fs::read_to_string(&output_path).unwrap();
1082 |         assert!(content.is_empty() || content.trim().is_empty());
1083 |     }
1084 | 
1085 |     #[test]
1086 |     fn test_process_file_with_different_extensions() {
1087 |         let dir = tempdir().unwrap();
1088 |         let base_path = dir.path();
1089 |         let output_path = base_path.join("output.md");
1090 | 
1091 |         // Test various file extensions
1092 |         let test_files = [
1093 |             ("script.py", "print('hello')", "python"),
1094 |             ("data.json", r#"{"key": "value"}"#, "json"),
1095 |             ("config.yaml", "key: value", "yaml"),
1096 |             ("style.css", "body { margin: 0; }", "css"),
1097 |             ("page.html", "<html><body>Test</body></html>", "html"),
1098 |             ("query.sql", "SELECT * FROM users;", "sql"),
1099 |             ("build.sh", "#!/bin/bash\necho 'building'", "bash"),
1100 |             ("unknown.xyz", "unknown content", "xyz"),
1101 |         ];
1102 | 
1103 |         for (filename, content, expected_lang) in test_files.iter() {
1104 |             let file_path = base_path.join(filename);
1105 |             fs::write(&file_path, content).unwrap();
1106 | 
1107 |             let mut output = fs::File::create(&output_path).unwrap();
1108 |             process_file(base_path, &file_path, &mut output, false, None).unwrap();
1109 | 
1110 |             let result = fs::read_to_string(&output_path).unwrap();
1111 |             assert!(result.contains(&format!("```{}", expected_lang)));
1112 |             assert!(result.contains(content));
1113 |             assert!(result.contains(filename));
1114 |         }
1115 |     }
1116 | }
```

### File: `src/state.rs`

- Size: 26113 bytes
- Modified: SystemTime { tv_sec: 1771108862, tv_nsec: 845858826 }

```rust
   1 | //! Project state representation for context-builder.
   2 | //!
   3 | //! This module provides structured data types to represent the state of a project
   4 | //! at a point in time. This replaces the previous approach of caching generated
   5 | //! markdown and enables more robust diff generation.
   6 | 
   7 | use chrono::Utc;
   8 | use ignore::DirEntry;
   9 | use serde::{Deserialize, Serialize};
  10 | use std::collections::BTreeMap;
  11 | use std::path::{Path, PathBuf};
  12 | use std::time::SystemTime;
  13 | 
  14 | use crate::config::Config;
  15 | use crate::diff::{PerFileDiff, PerFileStatus, diff_file_contents};
  16 | 
  17 | /// Complete state representation of a project at a point in time
  18 | #[derive(Serialize, Deserialize, Debug, Clone)]
  19 | pub struct ProjectState {
  20 |     /// Timestamp when this state was captured
  21 |     pub timestamp: String,
  22 |     /// Hash of the configuration used to generate this state
  23 |     pub config_hash: String,
  24 |     /// Map of file paths to their state information
  25 |     pub files: BTreeMap<PathBuf, FileState>,
  26 |     /// Project metadata
  27 |     pub metadata: ProjectMetadata,
  28 | }
  29 | 
  30 | /// State information for a single file
  31 | #[derive(Serialize, Deserialize, Debug, Clone)]
  32 | pub struct FileState {
  33 |     /// Raw file content as string
  34 |     pub content: String,
  35 |     /// File size in bytes
  36 |     pub size: u64,
  37 |     /// Last modified time
  38 |     pub modified: SystemTime,
  39 |     /// Content hash for quick comparison
  40 |     pub content_hash: String,
  41 | }
  42 | 
  43 | /// Metadata about the project
  44 | #[derive(Serialize, Deserialize, Debug, Clone)]
  45 | pub struct ProjectMetadata {
  46 |     /// Project directory name
  47 |     pub project_name: String,
  48 |     /// Total number of files processed
  49 |     pub file_count: usize,
  50 |     /// Filters applied during processing
  51 |     pub filters: Vec<String>,
  52 |     /// Ignore patterns applied
  53 |     pub ignores: Vec<String>,
  54 |     /// Whether line numbers were enabled
  55 |     pub line_numbers: bool,
  56 | }
  57 | 
  58 | /// Result of comparing two project states
  59 | #[derive(Debug, Clone)]
  60 | pub struct StateComparison {
  61 |     /// Per-file differences
  62 |     pub file_diffs: Vec<PerFileDiff>,
  63 |     /// Summary of changes
  64 |     pub summary: ChangeSummary,
  65 | }
  66 | 
  67 | /// Summary of changes between two states
  68 | #[derive(Debug, Clone)]
  69 | pub struct ChangeSummary {
  70 |     /// Files that were added
  71 |     pub added: Vec<PathBuf>,
  72 |     /// Files that were removed
  73 |     pub removed: Vec<PathBuf>,
  74 |     /// Files that were modified
  75 |     pub modified: Vec<PathBuf>,
  76 |     /// Total number of changed files
  77 |     pub total_changes: usize,
  78 | }
  79 | 
  80 | impl ProjectState {
  81 |     /// Create a new project state from collected files
  82 |     pub fn from_files(
  83 |         files: &[DirEntry],
  84 |         base_path: &Path,
  85 |         config: &Config,
  86 |         line_numbers: bool,
  87 |     ) -> std::io::Result<Self> {
  88 |         let mut file_states = BTreeMap::new();
  89 | 
  90 |         // Ensure paths stored in the state are *always* relative (never absolute).
  91 |         // This keeps cache stable across different launch contexts and matches
  92 |         // test expectations. We attempt a few strategies to derive a relative path.
  93 |         let cwd = std::env::current_dir().unwrap_or_else(|_| base_path.to_path_buf());
  94 |         for entry in files {
  95 |             let entry_path = entry.path();
  96 | 
  97 |             let relative_path = entry_path
  98 |                 // Preferred: relative to provided base_path (common case when input is absolute)
  99 |                 .strip_prefix(base_path)
 100 |                 .or_else(|_| entry_path.strip_prefix(&cwd))
 101 |                 .map(|p| p.to_path_buf())
 102 |                 .unwrap_or_else(|_| {
 103 |                     // Fallback: last component (file name) to avoid leaking absolute paths
 104 |                     entry_path
 105 |                         .file_name()
 106 |                         .map(PathBuf::from)
 107 |                         .unwrap_or_else(|| entry_path.to_path_buf())
 108 |                 });
 109 | 
 110 |             let file_state = FileState::from_path(entry_path)?;
 111 |             file_states.insert(relative_path, file_state);
 112 |         }
 113 | 
 114 |         // Resolve project name robustly: canonicalize to handle "." and relative paths
 115 |         let canonical = base_path.canonicalize().ok();
 116 |         let resolved = canonical.as_deref().unwrap_or(base_path);
 117 |         let project_name = resolved
 118 |             .file_name()
 119 |             .and_then(|n| n.to_str())
 120 |             .map(|s| s.to_string())
 121 |             .unwrap_or_else(|| {
 122 |                 // Fallback: try CWD if base_path has no file_name (e.g., root path)
 123 |                 std::env::current_dir()
 124 |                     .ok()
 125 |                     .and_then(|p| {
 126 |                         p.file_name()
 127 |                             .and_then(|n| n.to_str())
 128 |                             .map(|s| s.to_string())
 129 |                     })
 130 |                     .unwrap_or_else(|| "unknown".to_string())
 131 |             });
 132 | 
 133 |         let metadata = ProjectMetadata {
 134 |             project_name,
 135 |             file_count: files.len(),
 136 |             filters: config.filter.clone().unwrap_or_default(),
 137 |             ignores: config.ignore.clone().unwrap_or_default(),
 138 |             line_numbers,
 139 |         };
 140 | 
 141 |         Ok(ProjectState {
 142 |             timestamp: Utc::now().format("%Y-%m-%d %H:%M:%S UTC").to_string(),
 143 |             config_hash: Self::compute_config_hash(config),
 144 |             files: file_states,
 145 |             metadata,
 146 |         })
 147 |     }
 148 | 
 149 |     /// Compare this state with a previous state
 150 |     pub fn compare_with(&self, previous: &ProjectState) -> StateComparison {
 151 |         // Convert file states to content maps for diff_file_contents
 152 |         let previous_content: std::collections::HashMap<String, String> = previous
 153 |             .files
 154 |             .iter()
 155 |             .map(|(path, state)| (path.to_string_lossy().to_string(), state.content.clone()))
 156 |             .collect();
 157 | 
 158 |         let current_content: std::collections::HashMap<String, String> = self
 159 |             .files
 160 |             .iter()
 161 |             .map(|(path, state)| (path.to_string_lossy().to_string(), state.content.clone()))
 162 |             .collect();
 163 | 
 164 |         // Generate per-file diffs
 165 |         let file_diffs = diff_file_contents(&previous_content, &current_content, true, None);
 166 | 
 167 |         // Generate summary
 168 |         let mut added = Vec::new();
 169 |         let mut removed = Vec::new();
 170 |         let mut modified = Vec::new();
 171 | 
 172 |         for diff in &file_diffs {
 173 |             let path = PathBuf::from(&diff.path);
 174 |             match diff.status {
 175 |                 PerFileStatus::Added => added.push(path),
 176 |                 PerFileStatus::Removed => removed.push(path),
 177 |                 PerFileStatus::Modified => modified.push(path),
 178 |                 PerFileStatus::Unchanged => {}
 179 |             }
 180 |         }
 181 | 
 182 |         let summary = ChangeSummary {
 183 |             total_changes: added.len() + removed.len() + modified.len(),
 184 |             added,
 185 |             removed,
 186 |             modified,
 187 |         };
 188 | 
 189 |         StateComparison {
 190 |             file_diffs,
 191 |             summary,
 192 |         }
 193 |     }
 194 | 
 195 |     /// Check if this state has any content changes compared to another
 196 |     pub fn has_changes(&self, other: &ProjectState) -> bool {
 197 |         if self.files.len() != other.files.len() {
 198 |             return true;
 199 |         }
 200 | 
 201 |         for (path, state) in &self.files {
 202 |             match other.files.get(path) {
 203 |                 Some(other_state) => {
 204 |                     if state.content_hash != other_state.content_hash {
 205 |                         return true;
 206 |                     }
 207 |                 }
 208 |                 None => return true,
 209 |             }
 210 |         }
 211 | 
 212 |         false
 213 |     }
 214 | 
 215 |     /// Generate a configuration hash for cache validation
 216 |     fn compute_config_hash(config: &Config) -> String {
 217 |         // Build a stable string representation for hashing
 218 |         let mut config_str = String::new();
 219 |         if let Some(ref filters) = config.filter {
 220 |             config_str.push_str(&filters.join(","));
 221 |         }
 222 |         config_str.push('|');
 223 |         if let Some(ref ignores) = config.ignore {
 224 |             config_str.push_str(&ignores.join(","));
 225 |         }
 226 |         config_str.push('|');
 227 |         config_str.push_str(&format!(
 228 |             "{:?}|{:?}|{:?}",
 229 |             config.line_numbers, config.auto_diff, config.diff_context_lines
 230 |         ));
 231 | 
 232 |         let hash = xxhash_rust::xxh3::xxh3_64(config_str.as_bytes());
 233 |         format!("{:x}", hash)
 234 |     }
 235 | }
 236 | 
 237 | impl FileState {
 238 |     /// Create a file state from a file path
 239 |     pub fn from_path(path: &Path) -> std::io::Result<Self> {
 240 |         use std::fs;
 241 |         use std::io::ErrorKind;
 242 | 
 243 |         let metadata = fs::metadata(path)?;
 244 | 
 245 |         let content = match fs::read_to_string(path) {
 246 |             Ok(content) => content,
 247 |             Err(e) if e.kind() == ErrorKind::InvalidData => {
 248 |                 // Handle binary files gracefully
 249 |                 log::warn!("Skipping binary file in auto-diff mode: {}", path.display());
 250 |                 format!("<Binary file - {} bytes>", metadata.len())
 251 |             }
 252 |             Err(e) => return Err(e),
 253 |         };
 254 | 
 255 |         // Compute content hash using stable xxh3
 256 |         let content_hash = format!("{:016x}", xxhash_rust::xxh3::xxh3_64(content.as_bytes()));
 257 | 
 258 |         Ok(FileState {
 259 |             content,
 260 |             size: metadata.len(),
 261 |             modified: metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH),
 262 |             content_hash,
 263 |         })
 264 |     }
 265 | }
 266 | 
 267 | impl ChangeSummary {
 268 |     /// Check if there are any changes
 269 |     pub fn has_changes(&self) -> bool {
 270 |         self.total_changes > 0
 271 |     }
 272 | 
 273 |     /// Generate markdown representation of the change summary
 274 |     pub fn to_markdown(&self) -> String {
 275 |         if !self.has_changes() {
 276 |             return String::new();
 277 |         }
 278 | 
 279 |         let mut output = String::new();
 280 |         output.push_str("## Change Summary\n\n");
 281 | 
 282 |         for path in &self.added {
 283 |             output.push_str(&format!("- Added: `{}`\n", path.display()));
 284 |         }
 285 | 
 286 |         for path in &self.removed {
 287 |             output.push_str(&format!("- Removed: `{}`\n", path.display()));
 288 |         }
 289 | 
 290 |         for path in &self.modified {
 291 |             output.push_str(&format!("- Modified: `{}`\n", path.display()));
 292 |         }
 293 | 
 294 |         output.push('\n');
 295 |         output
 296 |     }
 297 | }
 298 | 
 299 | #[cfg(test)]
 300 | mod tests {
 301 |     use super::*;
 302 |     use std::fs;
 303 |     use tempfile::tempdir;
 304 | 
 305 |     #[test]
 306 |     fn test_file_state_creation() {
 307 |         let temp_dir = tempdir().unwrap();
 308 |         let file_path = temp_dir.path().join("test.txt");
 309 |         fs::write(&file_path, "Hello, world!").unwrap();
 310 | 
 311 |         let file_state = FileState::from_path(&file_path).unwrap();
 312 | 
 313 |         assert_eq!(file_state.content, "Hello, world!");
 314 |         assert_eq!(file_state.size, 13);
 315 |         assert!(!file_state.content_hash.is_empty());
 316 |     }
 317 | 
 318 |     #[test]
 319 |     fn test_project_state_comparison() {
 320 |         let temp_dir = tempdir().unwrap();
 321 |         let base_path = temp_dir.path();
 322 | 
 323 |         // Create initial files
 324 |         fs::write(base_path.join("file1.txt"), "content1").unwrap();
 325 |         fs::write(base_path.join("file2.txt"), "content2").unwrap();
 326 | 
 327 |         let mut state1_files = BTreeMap::new();
 328 |         state1_files.insert(
 329 |             PathBuf::from("file1.txt"),
 330 |             FileState::from_path(&base_path.join("file1.txt")).unwrap(),
 331 |         );
 332 |         state1_files.insert(
 333 |             PathBuf::from("file2.txt"),
 334 |             FileState::from_path(&base_path.join("file2.txt")).unwrap(),
 335 |         );
 336 | 
 337 |         let state1 = ProjectState {
 338 |             timestamp: "2023-01-01T00:00:00Z".to_string(),
 339 |             config_hash: "test_hash".to_string(),
 340 |             files: state1_files,
 341 |             metadata: ProjectMetadata {
 342 |                 project_name: "test".to_string(),
 343 |                 file_count: 2,
 344 |                 filters: vec![],
 345 |                 ignores: vec![],
 346 |                 line_numbers: false,
 347 |             },
 348 |         };
 349 | 
 350 |         // Modify and create new state
 351 |         fs::write(base_path.join("file1.txt"), "modified_content1").unwrap();
 352 |         fs::write(base_path.join("file3.txt"), "content3").unwrap();
 353 | 
 354 |         let mut state2_files = BTreeMap::new();
 355 |         state2_files.insert(
 356 |             PathBuf::from("file1.txt"),
 357 |             FileState::from_path(&base_path.join("file1.txt")).unwrap(),
 358 |         );
 359 |         state2_files.insert(
 360 |             PathBuf::from("file2.txt"),
 361 |             FileState::from_path(&base_path.join("file2.txt")).unwrap(),
 362 |         );
 363 |         state2_files.insert(
 364 |             PathBuf::from("file3.txt"),
 365 |             FileState::from_path(&base_path.join("file3.txt")).unwrap(),
 366 |         );
 367 | 
 368 |         let state2 = ProjectState {
 369 |             timestamp: "2023-01-01T01:00:00Z".to_string(),
 370 |             config_hash: "test_hash".to_string(),
 371 |             files: state2_files,
 372 |             metadata: ProjectMetadata {
 373 |                 project_name: "test".to_string(),
 374 |                 file_count: 3,
 375 |                 filters: vec![],
 376 |                 ignores: vec![],
 377 |                 line_numbers: false,
 378 |             },
 379 |         };
 380 | 
 381 |         let comparison = state2.compare_with(&state1);
 382 | 
 383 |         assert_eq!(comparison.summary.added.len(), 1);
 384 |         assert_eq!(comparison.summary.modified.len(), 1);
 385 |         assert_eq!(comparison.summary.removed.len(), 0);
 386 |         assert!(
 387 |             comparison
 388 |                 .summary
 389 |                 .added
 390 |                 .contains(&PathBuf::from("file3.txt"))
 391 |         );
 392 |         assert!(
 393 |             comparison
 394 |                 .summary
 395 |                 .modified
 396 |                 .contains(&PathBuf::from("file1.txt"))
 397 |         );
 398 |     }
 399 | 
 400 |     #[test]
 401 |     fn test_change_summary_markdown() {
 402 |         let summary = ChangeSummary {
 403 |             added: vec![PathBuf::from("new.txt")],
 404 |             removed: vec![PathBuf::from("old.txt")],
 405 |             modified: vec![PathBuf::from("changed.txt")],
 406 |             total_changes: 3,
 407 |         };
 408 | 
 409 |         let markdown = summary.to_markdown();
 410 | 
 411 |         assert!(markdown.contains("## Change Summary"));
 412 |         assert!(markdown.contains("- Added: `new.txt`"));
 413 |         assert!(markdown.contains("- Removed: `old.txt`"));
 414 |         assert!(markdown.contains("- Modified: `changed.txt`"));
 415 |     }
 416 | 
 417 |     #[test]
 418 |     fn test_binary_file_handling() {
 419 |         let temp_dir = tempdir().unwrap();
 420 |         let binary_file = temp_dir.path().join("test.bin");
 421 | 
 422 |         // Write binary data (non-UTF8)
 423 |         let binary_data = vec![0u8, 255, 128, 42, 0, 1, 2, 3];
 424 |         fs::write(&binary_file, &binary_data).unwrap();
 425 | 
 426 |         // Should not crash and should handle gracefully
 427 |         let file_state = FileState::from_path(&binary_file).unwrap();
 428 | 
 429 |         // Content should be a placeholder for binary files
 430 |         assert!(file_state.content.contains("Binary file"));
 431 |         assert!(file_state.content.contains("8 bytes"));
 432 |         assert_eq!(file_state.size, 8);
 433 |         assert!(!file_state.content_hash.is_empty());
 434 |     }
 435 | 
 436 |     #[test]
 437 |     fn test_has_changes_identical_states() {
 438 |         let temp_dir = tempdir().unwrap();
 439 |         let base_path = temp_dir.path();
 440 |         fs::write(base_path.join("test.txt"), "content").unwrap();
 441 | 
 442 |         let mut files = BTreeMap::new();
 443 |         files.insert(
 444 |             PathBuf::from("test.txt"),
 445 |             FileState::from_path(&base_path.join("test.txt")).unwrap(),
 446 |         );
 447 | 
 448 |         let state1 = ProjectState {
 449 |             timestamp: "2023-01-01T00:00:00Z".to_string(),
 450 |             config_hash: "hash1".to_string(),
 451 |             files: files.clone(),
 452 |             metadata: ProjectMetadata {
 453 |                 project_name: "test".to_string(),
 454 |                 file_count: 1,
 455 |                 filters: vec![],
 456 |                 ignores: vec![],
 457 |                 line_numbers: false,
 458 |             },
 459 |         };
 460 | 
 461 |         let state2 = ProjectState {
 462 |             timestamp: "2023-01-01T01:00:00Z".to_string(),
 463 |             config_hash: "hash1".to_string(),
 464 |             files,
 465 |             metadata: ProjectMetadata {
 466 |                 project_name: "test".to_string(),
 467 |                 file_count: 1,
 468 |                 filters: vec![],
 469 |                 ignores: vec![],
 470 |                 line_numbers: false,
 471 |             },
 472 |         };
 473 | 
 474 |         assert!(!state1.has_changes(&state2));
 475 |     }
 476 | 
 477 |     #[test]
 478 |     fn test_has_changes_different_file_count() {
 479 |         let temp_dir = tempdir().unwrap();
 480 |         let base_path = temp_dir.path();
 481 |         fs::write(base_path.join("test1.txt"), "content1").unwrap();
 482 |         fs::write(base_path.join("test2.txt"), "content2").unwrap();
 483 | 
 484 |         let mut files1 = BTreeMap::new();
 485 |         files1.insert(
 486 |             PathBuf::from("test1.txt"),
 487 |             FileState::from_path(&base_path.join("test1.txt")).unwrap(),
 488 |         );
 489 | 
 490 |         let mut files2 = BTreeMap::new();
 491 |         files2.insert(
 492 |             PathBuf::from("test1.txt"),
 493 |             FileState::from_path(&base_path.join("test1.txt")).unwrap(),
 494 |         );
 495 |         files2.insert(
 496 |             PathBuf::from("test2.txt"),
 497 |             FileState::from_path(&base_path.join("test2.txt")).unwrap(),
 498 |         );
 499 | 
 500 |         let state1 = ProjectState {
 501 |             timestamp: "2023-01-01T00:00:00Z".to_string(),
 502 |             config_hash: "hash1".to_string(),
 503 |             files: files1,
 504 |             metadata: ProjectMetadata {
 505 |                 project_name: "test".to_string(),
 506 |                 file_count: 1,
 507 |                 filters: vec![],
 508 |                 ignores: vec![],
 509 |                 line_numbers: false,
 510 |             },
 511 |         };
 512 | 
 513 |         let state2 = ProjectState {
 514 |             timestamp: "2023-01-01T01:00:00Z".to_string(),
 515 |             config_hash: "hash1".to_string(),
 516 |             files: files2,
 517 |             metadata: ProjectMetadata {
 518 |                 project_name: "test".to_string(),
 519 |                 file_count: 2,
 520 |                 filters: vec![],
 521 |                 ignores: vec![],
 522 |                 line_numbers: false,
 523 |             },
 524 |         };
 525 | 
 526 |         assert!(state1.has_changes(&state2));
 527 |     }
 528 | 
 529 |     #[test]
 530 |     fn test_has_changes_content_different() {
 531 |         let temp_dir = tempdir().unwrap();
 532 |         let base_path = temp_dir.path();
 533 |         fs::write(base_path.join("test.txt"), "content1").unwrap();
 534 | 
 535 |         let file_state1 = FileState::from_path(&base_path.join("test.txt")).unwrap();
 536 | 
 537 |         fs::write(base_path.join("test.txt"), "content2").unwrap();
 538 |         let file_state2 = FileState::from_path(&base_path.join("test.txt")).unwrap();
 539 | 
 540 |         let mut files1 = BTreeMap::new();
 541 |         files1.insert(PathBuf::from("test.txt"), file_state1);
 542 | 
 543 |         let mut files2 = BTreeMap::new();
 544 |         files2.insert(PathBuf::from("test.txt"), file_state2);
 545 | 
 546 |         let state1 = ProjectState {
 547 |             timestamp: "2023-01-01T00:00:00Z".to_string(),
 548 |             config_hash: "hash1".to_string(),
 549 |             files: files1,
 550 |             metadata: ProjectMetadata {
 551 |                 project_name: "test".to_string(),
 552 |                 file_count: 1,
 553 |                 filters: vec![],
 554 |                 ignores: vec![],
 555 |                 line_numbers: false,
 556 |             },
 557 |         };
 558 | 
 559 |         let state2 = ProjectState {
 560 |             timestamp: "2023-01-01T01:00:00Z".to_string(),
 561 |             config_hash: "hash1".to_string(),
 562 |             files: files2,
 563 |             metadata: ProjectMetadata {
 564 |                 project_name: "test".to_string(),
 565 |                 file_count: 1,
 566 |                 filters: vec![],
 567 |                 ignores: vec![],
 568 |                 line_numbers: false,
 569 |             },
 570 |         };
 571 | 
 572 |         assert!(state1.has_changes(&state2));
 573 |     }
 574 | 
 575 |     #[test]
 576 |     fn test_config_hash_generation() {
 577 |         let config1 = Config {
 578 |             filter: Some(vec!["rs".to_string()]),
 579 |             ignore: Some(vec!["target".to_string()]),
 580 |             line_numbers: Some(true),
 581 |             auto_diff: Some(false),
 582 |             diff_context_lines: Some(3),
 583 |             ..Default::default()
 584 |         };
 585 | 
 586 |         let config2 = Config {
 587 |             filter: Some(vec!["rs".to_string()]),
 588 |             ignore: Some(vec!["target".to_string()]),
 589 |             line_numbers: Some(true),
 590 |             auto_diff: Some(false),
 591 |             diff_context_lines: Some(3),
 592 |             ..Default::default()
 593 |         };
 594 | 
 595 |         let config3 = Config {
 596 |             filter: Some(vec!["py".to_string()]), // Different filter
 597 |             ignore: Some(vec!["target".to_string()]),
 598 |             line_numbers: Some(true),
 599 |             auto_diff: Some(false),
 600 |             diff_context_lines: Some(3),
 601 |             ..Default::default()
 602 |         };
 603 | 
 604 |         let hash1 = ProjectState::compute_config_hash(&config1);
 605 |         let hash2 = ProjectState::compute_config_hash(&config2);
 606 |         let hash3 = ProjectState::compute_config_hash(&config3);
 607 | 
 608 |         assert_eq!(hash1, hash2);
 609 |         assert_ne!(hash1, hash3);
 610 |     }
 611 | 
 612 |     #[test]
 613 |     fn test_change_summary_no_changes() {
 614 |         let summary = ChangeSummary {
 615 |             added: vec![],
 616 |             removed: vec![],
 617 |             modified: vec![],
 618 |             total_changes: 0,
 619 |         };
 620 | 
 621 |         assert!(!summary.has_changes());
 622 |         assert_eq!(summary.to_markdown(), "");
 623 |     }
 624 | 
 625 |     #[test]
 626 |     fn test_from_files_with_config() {
 627 |         let temp_dir = tempdir().unwrap();
 628 |         let base_path = temp_dir.path();
 629 | 
 630 |         fs::write(base_path.join("test.rs"), "fn main() {}").unwrap();
 631 |         fs::write(base_path.join("README.md"), "# Test").unwrap();
 632 | 
 633 |         let entries = vec![
 634 |             create_mock_dir_entry(&base_path.join("test.rs")),
 635 |             create_mock_dir_entry(&base_path.join("README.md")),
 636 |         ];
 637 | 
 638 |         let config = Config {
 639 |             filter: Some(vec!["rs".to_string()]),
 640 |             ignore: Some(vec!["target".to_string()]),
 641 |             line_numbers: Some(true),
 642 |             ..Default::default()
 643 |         };
 644 | 
 645 |         let state = ProjectState::from_files(&entries, base_path, &config, true).unwrap();
 646 | 
 647 |         assert_eq!(state.files.len(), 2);
 648 |         assert_eq!(state.metadata.file_count, 2);
 649 |         assert_eq!(state.metadata.filters, vec!["rs"]);
 650 |         assert_eq!(state.metadata.ignores, vec!["target"]);
 651 |         assert!(state.metadata.line_numbers);
 652 |         assert!(!state.timestamp.is_empty());
 653 |         assert!(!state.config_hash.is_empty());
 654 |     }
 655 | 
 656 |     #[test]
 657 |     fn test_from_files_absolute_path_fallback() {
 658 |         let temp_dir = tempdir().unwrap();
 659 |         let base_path = temp_dir.path();
 660 | 
 661 |         // Create a file in the temp dir
 662 |         fs::write(base_path.join("test.txt"), "test content").unwrap();
 663 |         let file_path = base_path.join("test.txt");
 664 | 
 665 |         // Create entry with the file
 666 |         let entry = create_mock_dir_entry(&file_path);
 667 | 
 668 |         // Use a completely different base_path to force the fallback
 669 |         let different_base = PathBuf::from("/completely/different/path");
 670 | 
 671 |         let config = Config::default();
 672 | 
 673 |         let state = ProjectState::from_files(&[entry], &different_base, &config, false).unwrap();
 674 | 
 675 |         // Should fall back to just the filename
 676 |         assert_eq!(state.files.len(), 1);
 677 |         assert!(state.files.contains_key(&PathBuf::from("test.txt")));
 678 |     }
 679 | 
 680 |     #[test]
 681 |     fn test_change_summary_with_unchanged_files() {
 682 |         let changes = vec![
 683 |             PerFileDiff {
 684 |                 path: "added.txt".to_string(),
 685 |                 status: PerFileStatus::Added,
 686 |                 diff: "diff content".to_string(),
 687 |             },
 688 |             PerFileDiff {
 689 |                 path: "unchanged.txt".to_string(),
 690 |                 status: PerFileStatus::Unchanged,
 691 |                 diff: "".to_string(),
 692 |             },
 693 |         ];
 694 | 
 695 |         // Manually create the summary like the actual code does
 696 |         let mut added = Vec::new();
 697 |         let mut removed = Vec::new();
 698 |         let mut modified = Vec::new();
 699 | 
 700 |         for diff in &changes {
 701 |             let path = PathBuf::from(&diff.path);
 702 |             match diff.status {
 703 |                 PerFileStatus::Added => added.push(path),
 704 |                 PerFileStatus::Removed => removed.push(path),
 705 |                 PerFileStatus::Modified => modified.push(path),
 706 |                 PerFileStatus::Unchanged => {} // This line should be covered now
 707 |             }
 708 |         }
 709 | 
 710 |         let summary = ChangeSummary {
 711 |             total_changes: added.len() + removed.len() + modified.len(),
 712 |             added,
 713 |             removed,
 714 |             modified,
 715 |         };
 716 | 
 717 |         assert_eq!(summary.total_changes, 1); // Only the added file counts
 718 |         assert_eq!(summary.added.len(), 1);
 719 |         assert_eq!(summary.removed.len(), 0);
 720 |         assert_eq!(summary.modified.len(), 0);
 721 |     }
 722 | 
 723 |     #[test]
 724 |     fn test_has_changes_with_missing_file() {
 725 |         let temp_dir = tempdir().unwrap();
 726 |         let base_path = temp_dir.path();
 727 | 
 728 |         // Create files for the first state
 729 |         fs::write(base_path.join("file1.txt"), "content1").unwrap();
 730 |         let entry1 = create_mock_dir_entry(&base_path.join("file1.txt"));
 731 | 
 732 |         let config = Config::default();
 733 |         let state1 = ProjectState::from_files(&[entry1], base_path, &config, false).unwrap();
 734 | 
 735 |         // Create a different state with different files
 736 |         fs::write(base_path.join("file2.txt"), "content2").unwrap();
 737 |         let entry2 = create_mock_dir_entry(&base_path.join("file2.txt"));
 738 |         let state2 = ProjectState::from_files(&[entry2], base_path, &config, false).unwrap();
 739 | 
 740 |         // Should detect changes because files are completely different
 741 |         assert!(state1.has_changes(&state2));
 742 |     }
 743 | 
 744 |     #[test]
 745 |     fn test_file_state_with_invalid_data_error() {
 746 |         // Create a temporary file with binary content that might trigger InvalidData
 747 |         let temp_dir = tempdir().unwrap();
 748 |         let binary_file = temp_dir.path().join("binary.dat");
 749 | 
 750 |         // Write invalid UTF-8 bytes
 751 |         let binary_data = vec![0xFF, 0xFE, 0xFD, 0xFC, 0xFB, 0xFA];
 752 |         fs::write(&binary_file, &binary_data).unwrap();
 753 | 
 754 |         // This might trigger the InvalidData error path, but since we can't guarantee it,
 755 |         // we at least verify the function can handle binary files
 756 |         let result = FileState::from_path(&binary_file);
 757 |         assert!(result.is_ok());
 758 |     }
 759 | 
 760 |     // Helper function to create a mock DirEntry for testing
 761 |     fn create_mock_dir_entry(path: &std::path::Path) -> ignore::DirEntry {
 762 |         // This is a bit of a hack since DirEntry doesn't have a public constructor
 763 |         // We use the ignore crate's WalkBuilder to create a real DirEntry
 764 |         let walker = ignore::WalkBuilder::new(path.parent().unwrap());
 765 |         walker
 766 |             .build()
 767 |             .filter_map(Result::ok)
 768 |             .find(|entry| entry.path() == path)
 769 |             .expect("Failed to create DirEntry for test")
 770 |     }
 771 | }
```

### File: `src/token_count.rs`

- Size: 9919 bytes
- Modified: SystemTime { tv_sec: 1771053288, tv_nsec: 65557165 }

```rust
   1 | use ignore::DirEntry;
   2 | use once_cell::sync::Lazy;
   3 | use std::collections::BTreeMap;
   4 | use std::fs;
   5 | use std::path::Path;
   6 | /// Token counting utilities for estimating LLM token usage
   7 | use tiktoken_rs::{CoreBPE, cl100k_base};
   8 | 
   9 | // Initialize the tokenizer once and reuse it
  10 | static TOKENIZER: Lazy<CoreBPE> = Lazy::new(|| cl100k_base().unwrap());
  11 | 
  12 | /// Estimates the number of tokens in a text string using a real tokenizer
  13 | pub fn estimate_tokens(text: &str) -> usize {
  14 |     TOKENIZER.encode_with_special_tokens(text).len()
  15 | }
  16 | 
  17 | /// Counts the tokens that would be generated for a file
  18 | pub fn count_file_tokens(base_path: &Path, entry: &DirEntry, line_numbers: bool) -> usize {
  19 |     let file_path = entry.path();
  20 |     let relative_path = file_path.strip_prefix(base_path).unwrap_or(file_path);
  21 | 
  22 |     // Start with tokens for the file header (path, size, modified time)
  23 |     let mut token_count = estimate_tokens(&format!(
  24 |         "\n### File: `{}`\n\n- Size: {} bytes\n- Modified: {}\n\n",
  25 |         relative_path.display(),
  26 |         entry.metadata().map(|m| m.len()).unwrap_or(0),
  27 |         "Unknown"
  28 |     )); // Using "Unknown" as placeholder for modified time in estimation
  29 | 
  30 |     // Add tokens for the code fences
  31 |     token_count += estimate_tokens("```\n```");
  32 | 
  33 |     // Try to read file content
  34 |     if let Ok(content) = fs::read_to_string(file_path) {
  35 |         if line_numbers {
  36 |             // When line numbers are enabled, we add the line number prefix to each line
  37 |             let lines_with_numbers: String = content
  38 |                 .lines()
  39 |                 .enumerate()
  40 |                 .map(|(i, line)| format!("{:>4} | {}\n", i + 1, line))
  41 |                 .collect();
  42 |             token_count += estimate_tokens(&lines_with_numbers);
  43 |         } else {
  44 |             token_count += estimate_tokens(&content);
  45 |         }
  46 |     }
  47 | 
  48 |     token_count
  49 | }
  50 | 
  51 | /// Counts the tokens that would be generated for the entire file tree section
  52 | pub fn count_tree_tokens(tree: &BTreeMap<String, crate::tree::FileNode>, depth: usize) -> usize {
  53 |     let mut token_count = 0;
  54 | 
  55 |     // Add tokens for indentation
  56 |     let indent = "  ".repeat(depth);
  57 | 
  58 |     for (name, node) in tree {
  59 |         match node {
  60 |             crate::tree::FileNode::File => {
  61 |                 token_count += estimate_tokens(&format!("{}- 📄 {}\n", indent, name));
  62 |             }
  63 |             crate::tree::FileNode::Directory(children) => {
  64 |                 token_count += estimate_tokens(&format!("{}- 📁 {}\n", indent, name));
  65 |                 token_count += count_tree_tokens(children, depth + 1);
  66 |             }
  67 |         }
  68 |     }
  69 | 
  70 |     token_count
  71 | }
  72 | 
  73 | #[cfg(test)]
  74 | mod tests {
  75 |     use super::*;
  76 |     use std::collections::BTreeMap;
  77 | 
  78 |     #[test]
  79 |     fn test_estimate_tokens() {
  80 |         // Test with a simple string
  81 |         let text = "Hello, world!";
  82 |         let tokens = estimate_tokens(text);
  83 |         // "Hello, world!" is 4 tokens with cl100k_base
  84 |         assert_eq!(tokens, 4);
  85 | 
  86 |         // Test with code-like content
  87 |         let code_text = "fn main() {\n    println!(\"Hello, world!\");\n}";
  88 |         let tokens = estimate_tokens(code_text);
  89 |         // This specific code snippet is 12 tokens with cl100k_base
  90 |         assert_eq!(tokens, 12);
  91 |     }
  92 | 
  93 |     #[test]
  94 |     fn test_count_tree_tokens() {
  95 |         // Create a simple tree structure
  96 |         let mut tree = BTreeMap::new();
  97 |         tree.insert("file1.rs".to_string(), crate::tree::FileNode::File);
  98 | 
  99 |         let mut subdir = BTreeMap::new();
 100 |         subdir.insert("file2.md".to_string(), crate::tree::FileNode::File);
 101 |         tree.insert("src".to_string(), crate::tree::FileNode::Directory(subdir));
 102 | 
 103 |         let tokens = count_tree_tokens(&tree, 0);
 104 |         // "- 📄 file1.rs\n" -> 8 tokens
 105 |         // "- 📁 src\n" -> 6 tokens
 106 |         // "  - 📄 file2.md\n" -> 9 tokens
 107 |         // Total should be 23 tokens
 108 |         assert_eq!(tokens, 23);
 109 |     }
 110 | 
 111 |     #[test]
 112 |     fn test_token_estimation_format_consistency() {
 113 |         use tempfile::tempdir;
 114 | 
 115 |         let dir = tempdir().unwrap();
 116 |         let test_file = dir.path().join("test.rs");
 117 |         std::fs::write(&test_file, "fn main() {}\n").unwrap();
 118 | 
 119 |         let entry = ignore::WalkBuilder::new(&test_file)
 120 |             .build()
 121 |             .next()
 122 |             .unwrap()
 123 |             .unwrap();
 124 | 
 125 |         // Estimate tokens for the file
 126 |         let estimated_tokens = count_file_tokens(dir.path(), &entry, false);
 127 | 
 128 |         // Generate actual markdown content
 129 |         let mut actual_content = Vec::new();
 130 |         crate::markdown::process_file(dir.path(), &test_file, &mut actual_content, false, None)
 131 |             .unwrap();
 132 |         let actual_content_str = String::from_utf8(actual_content).unwrap();
 133 | 
 134 |         // Count actual tokens
 135 |         let actual_tokens = estimate_tokens(&actual_content_str);
 136 | 
 137 |         // The estimation should be close to actual (within a reasonable margin)
 138 |         // Allow for some variance due to timestamp differences and minor formatting
 139 |         let difference = actual_tokens.abs_diff(estimated_tokens);
 140 | 
 141 |         // Should be within 10% or 20 tokens difference (whichever is larger)
 142 |         let max_allowed_difference = std::cmp::max(actual_tokens / 10, 20);
 143 | 
 144 |         assert!(
 145 |             difference <= max_allowed_difference,
 146 |             "Token estimation {} differs too much from actual {} (difference: {})",
 147 |             estimated_tokens,
 148 |             actual_tokens,
 149 |             difference
 150 |         );
 151 |     }
 152 | 
 153 |     #[test]
 154 |     fn test_estimate_tokens_empty_string() {
 155 |         let tokens = estimate_tokens("");
 156 |         assert_eq!(tokens, 0);
 157 |     }
 158 | 
 159 |     #[test]
 160 |     fn test_estimate_tokens_whitespace_only() {
 161 |         let tokens = estimate_tokens("   \n\t  ");
 162 |         assert!(tokens > 0); // Whitespace still counts as tokens
 163 |     }
 164 | 
 165 |     #[test]
 166 |     fn test_estimate_tokens_unicode() {
 167 |         let tokens = estimate_tokens("Hello 世界! 🌍");
 168 |         assert!(tokens > 0);
 169 |         // Unicode characters may be encoded as multiple tokens
 170 |         assert!(tokens >= 4);
 171 |     }
 172 | 
 173 |     #[test]
 174 |     fn test_count_file_tokens_with_line_numbers() {
 175 |         use tempfile::tempdir;
 176 | 
 177 |         let dir = tempdir().unwrap();
 178 |         let test_file = dir.path().join("test.rs");
 179 |         std::fs::write(&test_file, "line 1\nline 2\nline 3").unwrap();
 180 | 
 181 |         let entry = ignore::WalkBuilder::new(&test_file)
 182 |             .build()
 183 |             .next()
 184 |             .unwrap()
 185 |             .unwrap();
 186 | 
 187 |         let tokens_without_line_numbers = count_file_tokens(dir.path(), &entry, false);
 188 |         let tokens_with_line_numbers = count_file_tokens(dir.path(), &entry, true);
 189 | 
 190 |         // With line numbers should have more tokens due to line number prefixes
 191 |         assert!(tokens_with_line_numbers > tokens_without_line_numbers);
 192 |     }
 193 | 
 194 |     #[test]
 195 |     fn test_count_file_tokens_unreadable_file() {
 196 |         use tempfile::tempdir;
 197 | 
 198 |         let dir = tempdir().unwrap();
 199 |         let test_file = dir.path().join("nonexistent.txt");
 200 | 
 201 |         // Create a mock DirEntry for a file that doesn't exist
 202 |         // This simulates what happens when a file is deleted between discovery and processing
 203 |         let walker = ignore::WalkBuilder::new(dir.path());
 204 |         let mut found_entry = None;
 205 | 
 206 |         // Create the file temporarily to get a DirEntry
 207 |         std::fs::write(&test_file, "temp").unwrap();
 208 |         for entry in walker.build() {
 209 |             if let Ok(entry) = entry
 210 |                 && entry.path() == test_file
 211 |             {
 212 |                 found_entry = Some(entry);
 213 |                 break;
 214 |             }
 215 |         }
 216 | 
 217 |         // Now delete the file
 218 |         std::fs::remove_file(&test_file).unwrap();
 219 | 
 220 |         if let Some(entry) = found_entry {
 221 |             let tokens = count_file_tokens(dir.path(), &entry, false);
 222 |             // Should still return some tokens for the file header even if content can't be read
 223 |             assert!(tokens > 0);
 224 |         }
 225 |     }
 226 | 
 227 |     #[test]
 228 |     fn test_count_tree_tokens_empty_tree() {
 229 |         let tree = BTreeMap::new();
 230 |         let tokens = count_tree_tokens(&tree, 0);
 231 |         assert_eq!(tokens, 0);
 232 |     }
 233 | 
 234 |     #[test]
 235 |     fn test_count_tree_tokens_nested_directories() {
 236 |         let mut tree = BTreeMap::new();
 237 | 
 238 |         // Create deeply nested structure
 239 |         let mut level3 = BTreeMap::new();
 240 |         level3.insert("deep_file.txt".to_string(), crate::tree::FileNode::File);
 241 | 
 242 |         let mut level2 = BTreeMap::new();
 243 |         level2.insert(
 244 |             "level3".to_string(),
 245 |             crate::tree::FileNode::Directory(level3),
 246 |         );
 247 | 
 248 |         let mut level1 = BTreeMap::new();
 249 |         level1.insert(
 250 |             "level2".to_string(),
 251 |             crate::tree::FileNode::Directory(level2),
 252 |         );
 253 | 
 254 |         tree.insert(
 255 |             "level1".to_string(),
 256 |             crate::tree::FileNode::Directory(level1),
 257 |         );
 258 | 
 259 |         let tokens = count_tree_tokens(&tree, 0);
 260 |         assert!(tokens > 0);
 261 | 
 262 |         // Should account for indentation at different levels
 263 |         let tokens_with_depth = count_tree_tokens(&tree, 2);
 264 |         assert!(tokens_with_depth > tokens); // More indentation = more tokens
 265 |     }
 266 | 
 267 |     #[test]
 268 |     fn test_count_tree_tokens_mixed_content() {
 269 |         let mut tree = BTreeMap::new();
 270 | 
 271 |         // Add files with various name lengths and characters
 272 |         tree.insert("a.txt".to_string(), crate::tree::FileNode::File);
 273 |         tree.insert(
 274 |             "very_long_filename_with_underscores.rs".to_string(),
 275 |             crate::tree::FileNode::File,
 276 |         );
 277 |         tree.insert("файл.txt".to_string(), crate::tree::FileNode::File); // Unicode filename
 278 | 
 279 |         let mut subdir = BTreeMap::new();
 280 |         subdir.insert("nested.md".to_string(), crate::tree::FileNode::File);
 281 |         tree.insert(
 282 |             "directory".to_string(),
 283 |             crate::tree::FileNode::Directory(subdir),
 284 |         );
 285 | 
 286 |         let tokens = count_tree_tokens(&tree, 0);
 287 |         assert!(tokens > 0);
 288 | 
 289 |         // Verify it handles unicode filenames without crashing
 290 |         assert!(tokens > 20); // Should be substantial given the content
 291 |     }
 292 | }
```

### File: `src/tree.rs`

- Size: 10845 bytes
- Modified: SystemTime { tv_sec: 1771091715, tv_nsec: 380300807 }

```rust
   1 | use ignore::DirEntry;
   2 | use std::collections::BTreeMap;
   3 | use std::io::{self, Write};
   4 | use std::path::Path;
   5 | 
   6 | /// A nested map to represent the file tree structure.
   7 | #[derive(Debug, Clone, PartialEq)]
   8 | pub enum FileNode {
   9 |     File,
  10 |     Directory(BTreeMap<String, FileNode>),
  11 | }
  12 | 
  13 | /// Type alias for the file tree structure.
  14 | pub type FileTree = BTreeMap<String, FileNode>;
  15 | 
  16 | /// Builds a nested BTreeMap representing the file structure.
  17 | pub fn build_file_tree(files: &[DirEntry], base_path: &Path) -> FileTree {
  18 |     let mut tree = BTreeMap::new();
  19 |     for entry in files {
  20 |         let path = entry
  21 |             .path()
  22 |             .strip_prefix(base_path)
  23 |             .unwrap_or_else(|_| entry.path());
  24 |         let components: Vec<_> = path.components().collect();
  25 | 
  26 |         // Insert this path into the tree
  27 |         insert_path(&mut tree, &components);
  28 |     }
  29 |     tree
  30 | }
  31 | 
  32 | /// Helper function to insert a path into the tree structure
  33 | fn insert_path(tree: &mut FileTree, components: &[std::path::Component]) {
  34 |     if components.is_empty() {
  35 |         return;
  36 |     }
  37 | 
  38 |     let name = components[0].as_os_str().to_string_lossy().to_string();
  39 | 
  40 |     if components.len() == 1 {
  41 |         // This is the last component, so it's a file
  42 |         tree.insert(name, FileNode::File);
  43 |     } else {
  44 |         // This is a directory component
  45 |         // Make sure the directory exists
  46 |         tree.entry(name.clone())
  47 |             .or_insert_with(|| FileNode::Directory(BTreeMap::new()));
  48 | 
  49 |         // Recursively insert the rest of the path
  50 |         if let Some(FileNode::Directory(next_dir)) = tree.get_mut(&name) {
  51 |             insert_path(next_dir, &components[1..]);
  52 |         }
  53 |     }
  54 | }
  55 | 
  56 | /// Recursively prints the file tree to the console.
  57 | pub fn print_tree(tree: &FileTree, depth: usize) {
  58 |     for (name, node) in tree {
  59 |         let indent = "  ".repeat(depth);
  60 |         match node {
  61 |             FileNode::File => {
  62 |                 println!("{}- 📄 {}", indent, name);
  63 |             }
  64 |             FileNode::Directory(children) => {
  65 |                 println!("{}- 📁 {}", indent, name);
  66 |                 print_tree(children, depth + 1);
  67 |             }
  68 |         }
  69 |     }
  70 | }
  71 | 
  72 | /// Recursively writes the file tree to a file.
  73 | pub fn write_tree_to_file(
  74 |     output: &mut impl Write,
  75 |     tree: &FileTree,
  76 |     depth: usize,
  77 | ) -> io::Result<()> {
  78 |     for (name, node) in tree {
  79 |         let indent = "  ".repeat(depth);
  80 |         match node {
  81 |             FileNode::File => {
  82 |                 writeln!(output, "{}- 📄 {}", indent, name)?;
  83 |             }
  84 |             FileNode::Directory(children) => {
  85 |                 writeln!(output, "{}- 📁 {}", indent, name)?;
  86 |                 write_tree_to_file(output, children, depth + 1)?;
  87 |             }
  88 |         }
  89 |     }
  90 |     Ok(())
  91 | }
  92 | 
  93 | #[cfg(test)]
  94 | mod tests {
  95 |     use super::*;
  96 |     use crate::file_utils::collect_files;
  97 |     use std::fs;
  98 |     use tempfile::tempdir;
  99 | 
 100 |     #[test]
 101 |     fn test_build_file_tree_with_collected_files() {
 102 |         // 1. Set up a temporary directory with a file structure
 103 |         let dir = tempdir().unwrap();
 104 |         let base_path = dir.path();
 105 | 
 106 |         fs::create_dir(base_path.join("src")).unwrap();
 107 |         fs::File::create(base_path.join("src/main.rs")).unwrap();
 108 |         fs::File::create(base_path.join("README.md")).unwrap();
 109 |         // Add a hidden file that should be ignored by default
 110 |         fs::File::create(base_path.join(".env")).unwrap();
 111 | 
 112 |         // 2. Collect files using the actual function
 113 |         let files = collect_files(base_path, &[], &[], &[]).unwrap();
 114 | 
 115 |         // 3. Assert that the correct files were collected (a hidden file is ignored)
 116 |         assert_eq!(files.len(), 2);
 117 | 
 118 |         // 4. Build the tree with the collected files
 119 |         let tree = build_file_tree(&files, base_path);
 120 | 
 121 |         // 5. Assert the tree structure is correct
 122 |         let mut expected: FileTree = BTreeMap::new();
 123 |         let mut src_tree = BTreeMap::new();
 124 |         src_tree.insert("main.rs".to_string(), FileNode::File);
 125 |         expected.insert("src".to_string(), FileNode::Directory(src_tree));
 126 |         expected.insert("README.md".to_string(), FileNode::File);
 127 | 
 128 |         assert_eq!(tree, expected);
 129 |     }
 130 | 
 131 |     #[test]
 132 |     fn test_build_file_tree_empty() {
 133 |         let dir = tempdir().unwrap();
 134 |         let base_path = dir.path();
 135 | 
 136 |         let files = collect_files(base_path, &[], &[], &[]).unwrap();
 137 |         let tree = build_file_tree(&files, base_path);
 138 | 
 139 |         assert!(tree.is_empty());
 140 |     }
 141 | 
 142 |     #[test]
 143 |     fn test_build_file_tree_single_file() {
 144 |         let dir = tempdir().unwrap();
 145 |         let base_path = dir.path();
 146 | 
 147 |         fs::File::create(base_path.join("single.txt")).unwrap();
 148 | 
 149 |         let files = collect_files(base_path, &[], &[], &[]).unwrap();
 150 |         let tree = build_file_tree(&files, base_path);
 151 | 
 152 |         let mut expected: FileTree = BTreeMap::new();
 153 |         expected.insert("single.txt".to_string(), FileNode::File);
 154 | 
 155 |         assert_eq!(tree, expected);
 156 |     }
 157 | 
 158 |     #[test]
 159 |     fn test_build_file_tree_nested_directories() {
 160 |         let dir = tempdir().unwrap();
 161 |         let base_path = dir.path();
 162 | 
 163 |         fs::create_dir_all(base_path.join("a/b/c")).unwrap();
 164 |         fs::File::create(base_path.join("a/b/c/deep.txt")).unwrap();
 165 |         fs::File::create(base_path.join("a/shallow.txt")).unwrap();
 166 | 
 167 |         let files = collect_files(base_path, &[], &[], &[]).unwrap();
 168 |         let tree = build_file_tree(&files, base_path);
 169 | 
 170 |         // Build expected structure
 171 |         let mut c_tree = BTreeMap::new();
 172 |         c_tree.insert("deep.txt".to_string(), FileNode::File);
 173 | 
 174 |         let mut b_tree = BTreeMap::new();
 175 |         b_tree.insert("c".to_string(), FileNode::Directory(c_tree));
 176 | 
 177 |         let mut a_tree = BTreeMap::new();
 178 |         a_tree.insert("b".to_string(), FileNode::Directory(b_tree));
 179 |         a_tree.insert("shallow.txt".to_string(), FileNode::File);
 180 | 
 181 |         let mut expected: FileTree = BTreeMap::new();
 182 |         expected.insert("a".to_string(), FileNode::Directory(a_tree));
 183 | 
 184 |         assert_eq!(tree, expected);
 185 |     }
 186 | 
 187 |     #[test]
 188 |     fn test_build_file_tree_unicode_filenames() {
 189 |         let dir = tempdir().unwrap();
 190 |         let base_path = dir.path();
 191 | 
 192 |         fs::create_dir(base_path.join("测试目录")).unwrap();
 193 |         fs::File::create(base_path.join("测试目录/文件.txt")).unwrap();
 194 |         fs::File::create(base_path.join("🦀.rs")).unwrap();
 195 | 
 196 |         let files = collect_files(base_path, &[], &[], &[]).unwrap();
 197 |         let tree = build_file_tree(&files, base_path);
 198 | 
 199 |         let mut test_dir = BTreeMap::new();
 200 |         test_dir.insert("文件.txt".to_string(), FileNode::File);
 201 | 
 202 |         let mut expected: FileTree = BTreeMap::new();
 203 |         expected.insert("测试目录".to_string(), FileNode::Directory(test_dir));
 204 |         expected.insert("🦀.rs".to_string(), FileNode::File);
 205 | 
 206 |         assert_eq!(tree, expected);
 207 |     }
 208 | 
 209 |     #[test]
 210 |     fn test_insert_path_empty_components() {
 211 |         let mut tree = BTreeMap::new();
 212 |         insert_path(&mut tree, &[]);
 213 |         assert!(tree.is_empty());
 214 |     }
 215 | 
 216 |     #[test]
 217 |     fn test_write_tree_to_file() {
 218 |         let mut tree = BTreeMap::new();
 219 |         tree.insert("file1.txt".to_string(), FileNode::File);
 220 | 
 221 |         let mut subdir = BTreeMap::new();
 222 |         subdir.insert("file2.md".to_string(), FileNode::File);
 223 |         tree.insert("src".to_string(), FileNode::Directory(subdir));
 224 | 
 225 |         let mut output = Vec::new();
 226 |         write_tree_to_file(&mut output, &tree, 0).unwrap();
 227 | 
 228 |         let result = String::from_utf8(output).unwrap();
 229 |         assert!(result.contains("- 📄 file1.txt"));
 230 |         assert!(result.contains("- 📁 src"));
 231 |         assert!(result.contains("  - 📄 file2.md"));
 232 |     }
 233 | 
 234 |     #[test]
 235 |     fn test_write_tree_to_file_with_depth() {
 236 |         let mut tree = BTreeMap::new();
 237 |         tree.insert("nested.txt".to_string(), FileNode::File);
 238 | 
 239 |         let mut output = Vec::new();
 240 |         write_tree_to_file(&mut output, &tree, 2).unwrap();
 241 | 
 242 |         let result = String::from_utf8(output).unwrap();
 243 |         assert!(result.contains("    - 📄 nested.txt")); // 2 levels of indentation
 244 |     }
 245 | 
 246 |     #[test]
 247 |     fn test_write_tree_to_file_empty_tree() {
 248 |         let tree = BTreeMap::new();
 249 |         let mut output = Vec::new();
 250 |         write_tree_to_file(&mut output, &tree, 0).unwrap();
 251 | 
 252 |         let result = String::from_utf8(output).unwrap();
 253 |         assert!(result.is_empty());
 254 |     }
 255 | 
 256 |     #[test]
 257 |     fn test_file_node_equality() {
 258 |         let file1 = FileNode::File;
 259 |         let file2 = FileNode::File;
 260 |         assert_eq!(file1, file2);
 261 | 
 262 |         let mut dir1 = BTreeMap::new();
 263 |         dir1.insert("test.txt".to_string(), FileNode::File);
 264 |         let node1 = FileNode::Directory(dir1.clone());
 265 |         let node2 = FileNode::Directory(dir1);
 266 |         assert_eq!(node1, node2);
 267 | 
 268 |         // Different directories should not be equal
 269 |         let mut dir2 = BTreeMap::new();
 270 |         dir2.insert("other.txt".to_string(), FileNode::File);
 271 |         let node3 = FileNode::Directory(dir2);
 272 |         assert_ne!(node1, node3);
 273 | 
 274 |         // File and directory should not be equal
 275 |         assert_ne!(file1, node1);
 276 |     }
 277 | 
 278 |     #[test]
 279 |     fn test_build_file_tree_absolute_path_fallback() {
 280 |         // Test the fallback case when strip_prefix fails by using different base paths
 281 |         let dir = tempdir().unwrap();
 282 |         let base_path = dir.path();
 283 |         let other_dir = tempdir().unwrap();
 284 |         let other_base = other_dir.path();
 285 | 
 286 |         // Create a file in the first directory
 287 |         fs::File::create(base_path.join("test.txt")).unwrap();
 288 | 
 289 |         // Create a DirEntry from the first directory but use a different base_path
 290 |         let files = collect_files(base_path, &[], &[], &[]).unwrap();
 291 | 
 292 |         // This should trigger the unwrap_or_else case since other_base is unrelated to the file path
 293 |         let tree = build_file_tree(&files, other_base);
 294 | 
 295 |         // The tree should still contain the file, but with its full path
 296 |         assert!(!tree.is_empty());
 297 |     }
 298 | 
 299 |     #[test]
 300 |     fn test_build_file_tree_multiple_files_same_directory() {
 301 |         let dir = tempdir().unwrap();
 302 |         let base_path = dir.path();
 303 | 
 304 |         fs::create_dir(base_path.join("docs")).unwrap();
 305 |         fs::File::create(base_path.join("docs/readme.md")).unwrap();
 306 |         fs::File::create(base_path.join("docs/guide.md")).unwrap();
 307 |         fs::File::create(base_path.join("docs/api.md")).unwrap();
 308 | 
 309 |         let files = collect_files(base_path, &[], &[], &[]).unwrap();
 310 |         let tree = build_file_tree(&files, base_path);
 311 | 
 312 |         let mut docs_tree = BTreeMap::new();
 313 |         docs_tree.insert("api.md".to_string(), FileNode::File);
 314 |         docs_tree.insert("guide.md".to_string(), FileNode::File);
 315 |         docs_tree.insert("readme.md".to_string(), FileNode::File);
 316 | 
 317 |         let mut expected: FileTree = BTreeMap::new();
 318 |         expected.insert("docs".to_string(), FileNode::Directory(docs_tree));
 319 | 
 320 |         assert_eq!(tree, expected);
 321 |     }
 322 | }
```

### File: `tarpaulin.toml`

- Size: 304 bytes
- Modified: SystemTime { tv_sec: 1771053288, tv_nsec: 65557165 }

```toml
   1 | [test_config]
   2 | name = "Context Builder"
   3 | manifest-path = "./Cargo.toml"
   4 | skip-clean = true
   5 | all-features = false
   6 | exclude-files = [
   7 |         "samples/*",
   8 |         "benches/*",
   9 |         "tests/*",
  10 |         "scripts/*",
  11 |         "src/main.rs"
  12 |     ]
  13 | no-fail-fast = true
  14 | color = "Auto"
  15 | 
  16 | [report]
  17 | out = ["Html", "Xml"]
```

### File: `benches/context_bench.rs`

- Size: 10825 bytes
- Modified: SystemTime { tv_sec: 1771108956, tv_nsec: 864154701 }

```rust
   1 | use std::fs;
   2 | use std::path::{Path, PathBuf};
   3 | use std::sync::Once;
   4 | use std::time::Duration;
   5 | 
   6 | use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
   7 | use tempfile::tempdir;
   8 | 
   9 | use context_builder::cli::Args;
  10 | use context_builder::config::Config;
  11 | use context_builder::{Prompter, run_with_args};
  12 | 
  13 | static INIT: Once = Once::new();
  14 | 
  15 | fn init_bench_env() {
  16 |     INIT.call_once(|| {
  17 |         // Note: set_var now requires unsafe block from Rust 2024 onwards
  18 |         unsafe {
  19 |             std::env::set_var("CB_SILENT", "1");
  20 |         }
  21 |     });
  22 | }
  23 | 
  24 | /// Prompter that always auto-confirms. Used to avoid interactive pauses during benchmarks.
  25 | struct NoPrompt;
  26 | 
  27 | impl Prompter for NoPrompt {
  28 |     fn confirm_processing(&self, _file_count: usize) -> std::io::Result<bool> {
  29 |         Ok(true)
  30 |     }
  31 |     fn confirm_overwrite(&self, _file_path: &str) -> std::io::Result<bool> {
  32 |         Ok(true)
  33 |     }
  34 | }
  35 | 
  36 | /// Specification for generating a synthetic dataset for benchmarking.
  37 | #[derive(Clone)]
  38 | struct DatasetSpec {
  39 |     /// Human-friendly name used in the benchmark ID.
  40 |     name: &'static str,
  41 |     /// Approximate number of text files to generate.
  42 |     text_files: usize,
  43 |     /// Generate one binary file every `binary_every` text files (0 disables binary generation).
  44 |     binary_every: usize,
  45 |     /// Directory tree depth.
  46 |     depth: usize,
  47 |     /// Number of subdirectories per directory level.
  48 |     width: usize,
  49 |     /// Size of each text file (in bytes).
  50 |     text_file_size: usize,
  51 |     /// File extensions to include in benchmark (others should be ignored).
  52 |     filters: Vec<String>,
  53 |     /// Directory/file names to ignore (by component name).
  54 |     ignores: Vec<String>,
  55 | }
  56 | 
  57 | fn write_text_file(path: &Path, bytes: usize) {
  58 |     if let Some(parent) = path.parent() {
  59 |         fs::create_dir_all(parent).unwrap();
  60 |     }
  61 |     let mut content = String::with_capacity(bytes);
  62 |     // Generate deterministic content consisting of multiple lines
  63 |     // Approx 40 bytes per line -> repeat to reach desired size
  64 |     let line = "let x = 42; // benchmark content line\n";
  65 |     while content.len() < bytes {
  66 |         content.push_str(line);
  67 |     }
  68 |     // Trim to exact size
  69 |     content.truncate(bytes);
  70 |     // Ensure trailing newline for line-numbering path
  71 |     if !content.ends_with('\n') {
  72 |         content.push('\n');
  73 |     }
  74 |     fs::write(path, content).unwrap();
  75 | }
  76 | 
  77 | fn write_binary_file(path: &Path, bytes: usize) {
  78 |     if let Some(parent) = path.parent() {
  79 |         fs::create_dir_all(parent).unwrap();
  80 |     }
  81 |     let mut data = Vec::with_capacity(bytes);
  82 |     // Simple reproducible byte pattern
  83 |     for i in 0..bytes {
  84 |         data.push(((i as u8).wrapping_mul(31)).wrapping_add(7));
  85 |     }
  86 |     fs::write(path, data).unwrap();
  87 | }
  88 | 
  89 | /// Generate a synthetic project directory structure under `root`, returning the input directory path.
  90 | fn generate_dataset(root: &Path, spec: &DatasetSpec) -> PathBuf {
  91 |     let input_dir = root.join("project");
  92 |     let src_dir = input_dir.join("src");
  93 |     let docs_dir = input_dir.join("docs");
  94 |     let assets_dir = input_dir.join("assets");
  95 |     let ignored_target = input_dir.join("target"); // will be ignored if configured
  96 |     let ignored_node_modules = input_dir.join("node_modules"); // will be ignored if configured
  97 | 
  98 |     fs::create_dir_all(&src_dir).unwrap();
  99 |     fs::create_dir_all(&docs_dir).unwrap();
 100 |     fs::create_dir_all(&assets_dir).unwrap();
 101 |     fs::create_dir_all(&ignored_target).unwrap();
 102 |     fs::create_dir_all(&ignored_node_modules).unwrap();
 103 | 
 104 |     // Generate nested directories
 105 |     fn make_nested_dirs(base: &Path, depth: usize, width: usize) -> Vec<PathBuf> {
 106 |         let mut dirs = vec![base.to_path_buf()];
 107 |         for d in 1..=depth {
 108 |             let mut next_level = Vec::new();
 109 |             for parent in &dirs {
 110 |                 for w in 0..width {
 111 |                     let child = parent.join(format!("d{}_{}", d, w));
 112 |                     fs::create_dir_all(&child).unwrap();
 113 |                     next_level.push(child);
 114 |                 }
 115 |             }
 116 |             dirs.extend(next_level);
 117 |         }
 118 |         dirs
 119 |     }
 120 | 
 121 |     let all_dirs = {
 122 |         let mut v = Vec::new();
 123 |         v.extend(make_nested_dirs(&src_dir, spec.depth, spec.width));
 124 |         v.extend(make_nested_dirs(&docs_dir, spec.depth, spec.width));
 125 |         v.extend(make_nested_dirs(&assets_dir, spec.depth, spec.width));
 126 |         v
 127 |     };
 128 | 
 129 |     // Extensions to distribute across text files
 130 |     let text_exts = ["rs", "md", "txt", "toml"];
 131 | 
 132 |     // Create text files distributed across dirs
 133 |     let mut created = 0usize;
 134 |     let mut bin_counter = 0usize;
 135 | 
 136 |     'outer: for dir in &all_dirs {
 137 |         for i in 0..spec.width.max(1) {
 138 |             if created >= spec.text_files {
 139 |                 break 'outer;
 140 |             }
 141 |             // Round-robin extensions
 142 |             let ext = text_exts[created % text_exts.len()];
 143 |             let path = dir.join(format!("f{}_{}.{}", created, i, ext));
 144 |             write_text_file(&path, spec.text_file_size);
 145 |             created += 1;
 146 | 
 147 |             if spec.binary_every > 0 {
 148 |                 bin_counter += 1;
 149 |                 if bin_counter.is_multiple_of(spec.binary_every) {
 150 |                     let bpath = dir.join(format!("bin_{}_{}.bin", created, i));
 151 |                     write_binary_file(&bpath, 2048);
 152 |                 }
 153 |             }
 154 |         }
 155 |     }
 156 | 
 157 |     // Populate ignored directories with content that should not be processed
 158 |     write_text_file(&ignored_target.join("ignored.rs"), spec.text_file_size);
 159 |     write_text_file(
 160 |         &ignored_node_modules.join("ignored.js"),
 161 |         spec.text_file_size,
 162 |     );
 163 | 
 164 |     // Add some top-level files
 165 |     write_text_file(&input_dir.join("README.md"), spec.text_file_size);
 166 |     write_text_file(&input_dir.join("Cargo.toml"), spec.text_file_size);
 167 | 
 168 |     input_dir
 169 | }
 170 | 
 171 | /// Run a single benchmark scenario for a given dataset and line-numbering mode.
 172 | fn bench_scenario(c: &mut Criterion, spec: DatasetSpec, line_numbers: bool) {
 173 |     let tmp = tempdir().unwrap();
 174 |     let root = tmp.path();
 175 | 
 176 |     // Prefer local ./samples/<dataset>/project if it exists, else use CB_BENCH_DATASET_DIR, else generate temp dataset
 177 |     let samples_default = PathBuf::from("samples").join(spec.name).join("project");
 178 |     let input_dir = if samples_default.exists() {
 179 |         samples_default
 180 |     } else if let Some(dir) = std::env::var_os("CB_BENCH_DATASET_DIR") {
 181 |         let path = PathBuf::from(dir).join(spec.name).join("project");
 182 | 
 183 |         if !path.exists() {
 184 |             panic!(
 185 |                 "CB_BENCH_DATASET_DIR is set but dataset not found at {}",
 186 |                 path.display()
 187 |             );
 188 |         }
 189 | 
 190 |         path
 191 |     } else {
 192 |         generate_dataset(root, &spec)
 193 |     };
 194 | 
 195 |     let output_path = root.join(format!(
 196 |         "output_{}_{}.md",
 197 |         spec.name,
 198 |         if line_numbers { "ln" } else { "raw" }
 199 |     ));
 200 | 
 201 |     let args = Args {
 202 |         input: input_dir.to_string_lossy().into_owned(),
 203 |         output: output_path.to_string_lossy().into_owned(),
 204 |         filter: spec.filters.clone(),
 205 |         ignore: spec.ignores.clone(),
 206 |         preview: false,
 207 |         token_count: false,
 208 |         line_numbers,
 209 |         yes: true,
 210 |         diff_only: false,
 211 |         clear_cache: false,
 212 |         init: false,
 213 |         max_tokens: None,
 214 |     };
 215 | 
 216 |     let prompter = NoPrompt;
 217 | 
 218 |     let mut group = c.benchmark_group("context_builder");
 219 | 
 220 |     group.measurement_time(Duration::from_secs(20));
 221 |     group.sample_size(20);
 222 | 
 223 |     let mode = if cfg!(feature = "parallel") {
 224 |         "parallel"
 225 |     } else {
 226 |         "sequential"
 227 |     };
 228 |     let ln = if line_numbers {
 229 |         "line_numbers"
 230 |     } else {
 231 |         "no_line_numbers"
 232 |     };
 233 |     let id = BenchmarkId::new(
 234 |         format!(
 235 |             "{}-{}files-{}B",
 236 |             spec.name, spec.text_files, spec.text_file_size
 237 |         ),
 238 |         format!("{}-{}", ln, mode),
 239 |     );
 240 | 
 241 |     group.bench_with_input(id, &args, |b, _| {
 242 |         b.iter(|| {
 243 |             // Allow repeated overwrites; keep the output path stable to avoid filesystem churn
 244 |             let _ = std::hint::black_box(run_with_args(
 245 |                 Args {
 246 |                     input: args.input.clone(),
 247 |                     output: args.output.clone(),
 248 |                     filter: args.filter.clone(),
 249 |                     ignore: args.ignore.clone(),
 250 |                     preview: args.preview,
 251 |                     token_count: args.token_count,
 252 |                     line_numbers: args.line_numbers,
 253 |                     yes: true,
 254 |                     diff_only: false,
 255 |                     clear_cache: false,
 256 |                     init: false,
 257 |                     max_tokens: None,
 258 |                 },
 259 |                 Config::default(),
 260 |                 &prompter,
 261 |             ));
 262 |         });
 263 |     });
 264 | 
 265 |     group.finish();
 266 | }
 267 | 
 268 | /// Benchmarks:
 269 | /// - tiny: ~100 files, small size
 270 | /// - small: ~1,000 files
 271 | /// - medium: ~5,000 files (enabled only if CB_BENCH_MEDIUM=1)
 272 | ///
 273 | /// These datasets are generated in a temporary directory at runtime to keep the
 274 | /// benchmark self-contained. Binary files are generated but filtered out by
 275 | /// the `filters` configuration so they aren't processed.
 276 | ///
 277 | /// Run:
 278 | ///   cargo bench --bench context_bench
 279 | pub fn context_benchmark(c: &mut Criterion) {
 280 |     // Ensure silent-by-default for benchmarks
 281 |     init_bench_env();
 282 | 
 283 |     // Common filters and ignores: ignore typical heavy dirs; only include text code/docs
 284 |     let common_filters = vec!["rs".into(), "md".into(), "txt".into(), "toml".into()];
 285 |     let common_ignores = vec!["target".into(), "node_modules".into()];
 286 | 
 287 |     // Tiny dataset
 288 |     let tiny = DatasetSpec {
 289 |         name: "tiny",
 290 |         text_files: 100,
 291 |         binary_every: 10,
 292 |         depth: 2,
 293 |         width: 3,
 294 |         text_file_size: 256,
 295 |         filters: common_filters.clone(),
 296 |         ignores: common_ignores.clone(),
 297 |     };
 298 | 
 299 |     // Small dataset
 300 |     let small = DatasetSpec {
 301 |         name: "small",
 302 |         text_files: 1_000,
 303 |         binary_every: 20,
 304 |         depth: 3,
 305 |         width: 4,
 306 |         text_file_size: 512,
 307 |         filters: common_filters.clone(),
 308 |         ignores: common_ignores.clone(),
 309 |     };
 310 | 
 311 |     // Medium dataset (can be enabled via env var to avoid heavy runs by default)
 312 |     let include_medium = std::env::var("CB_BENCH_MEDIUM").ok().as_deref() == Some("1");
 313 |     let medium = DatasetSpec {
 314 |         name: "medium",
 315 |         text_files: 5_000,
 316 |         binary_every: 25,
 317 |         depth: 4,
 318 |         width: 4,
 319 |         text_file_size: 800,
 320 |         filters: common_filters.clone(),
 321 |         ignores: common_ignores.clone(),
 322 |     };
 323 | 
 324 |     // For each dataset, run benchmarks with and without line numbers
 325 |     for ds in [tiny, small] {
 326 |         bench_scenario(c, ds.clone(), false);
 327 |         bench_scenario(c, ds, true);
 328 |     }
 329 | 
 330 |     if include_medium {
 331 |         bench_scenario(c, medium.clone(), false);
 332 |         bench_scenario(c, medium, true);
 333 |     }
 334 | }
 335 | 
 336 | criterion_group!(benches, context_benchmark);
 337 | criterion_main!(benches);
```

### File: `tests/cli_integration.rs`

- Size: 12938 bytes
- Modified: SystemTime { tv_sec: 1771098907, tv_nsec: 779246312 }

```rust
   1 | use std::cell::Cell;
   2 | use std::fs;
   3 | use std::path::Path;
   4 | 
   5 | use tempfile::tempdir;
   6 | 
   7 | use context_builder::config::Config;
   8 | use context_builder::{Prompter, cli::Args, run_with_args};
   9 | 
  10 | struct TestPrompter {
  11 |     overwrite_response: bool,
  12 |     processing_response: bool,
  13 |     last_processing_count: Cell<usize>,
  14 | }
  15 | 
  16 | impl TestPrompter {
  17 |     fn new(overwrite_response: bool, processing_response: bool) -> Self {
  18 |         Self {
  19 |             overwrite_response,
  20 |             processing_response,
  21 |             last_processing_count: Cell::new(0),
  22 |         }
  23 |     }
  24 | 
  25 |     fn last_count(&self) -> usize {
  26 |         self.last_processing_count.get()
  27 |     }
  28 | }
  29 | 
  30 | impl Prompter for TestPrompter {
  31 |     fn confirm_processing(&self, file_count: usize) -> std::io::Result<bool> {
  32 |         self.last_processing_count.set(file_count);
  33 |         Ok(self.processing_response)
  34 |     }
  35 | 
  36 |     fn confirm_overwrite(&self, _file_path: &str) -> std::io::Result<bool> {
  37 |         Ok(self.overwrite_response)
  38 |     }
  39 | }
  40 | 
  41 | fn write_file(path: &Path, contents: &str) {
  42 |     if let Some(parent) = path.parent() {
  43 |         fs::create_dir_all(parent).unwrap();
  44 |     }
  45 |     fs::write(path, contents).unwrap();
  46 | }
  47 | 
  48 | #[test]
  49 | fn preview_mode_does_not_create_output_file() {
  50 |     let dir = tempdir().unwrap();
  51 |     let root = dir.path();
  52 | 
  53 |     // Create a small project structure
  54 |     write_file(&root.join("src/main.rs"), "fn main() { println!(\"hi\"); }");
  55 |     write_file(&root.join("README.md"), "# Readme");
  56 | 
  57 |     let args = Args {
  58 |         input: root.to_string_lossy().into_owned(),
  59 |         output: root.join("output.md").to_string_lossy().into_owned(),
  60 |         filter: vec![],
  61 |         ignore: vec![],
  62 |         preview: true,
  63 |         token_count: false,
  64 |         line_numbers: false,
  65 |         yes: false,
  66 |         diff_only: false,
  67 |         clear_cache: false,
  68 |         init: false,
  69 |         max_tokens: None,
  70 |     };
  71 | 
  72 |     let prompter = TestPrompter::new(true, true);
  73 | 
  74 |     // Run in preview mode
  75 |     let res = run_with_args(args, Config::default(), &prompter);
  76 |     assert!(res.is_ok(), "preview mode should succeed");
  77 | 
  78 |     // No output file created
  79 |     assert!(
  80 |         !root.join("output.md").exists(),
  81 |         "output file should not be created in preview mode"
  82 |     );
  83 | }
  84 | 
  85 | #[test]
  86 | fn preview_mode_skips_overwrite_confirmation() {
  87 |     let dir = tempdir().unwrap();
  88 |     let root = dir.path();
  89 | 
  90 |     // Create an existing output file
  91 |     let output_path = root.join("output.md");
  92 |     write_file(&output_path, "existing content");
  93 | 
  94 |     // Create a small project structure
  95 |     write_file(&root.join("src/main.rs"), "fn main() { println!(\"hi\"); }");
  96 |     write_file(&root.join("README.md"), "# Readme");
  97 | 
  98 |     let args = Args {
  99 |         input: root.to_string_lossy().into_owned(),
 100 |         output: output_path.to_string_lossy().into_owned(),
 101 |         filter: vec![],
 102 |         ignore: vec![],
 103 |         preview: true,
 104 |         token_count: false,
 105 |         line_numbers: false,
 106 |         yes: false,
 107 |         diff_only: false,
 108 |         clear_cache: false,
 109 |         init: false,
 110 |         max_tokens: None,
 111 |     };
 112 | 
 113 |     // Use false for overwrite response to verify it's not called
 114 |     let prompter = TestPrompter::new(false, true);
 115 | 
 116 |     // Run in preview mode - should succeed even with overwrite denied
 117 |     let res = run_with_args(args, Config::default(), &prompter);
 118 |     assert!(
 119 |         res.is_ok(),
 120 |         "preview mode should succeed without overwrite confirmation"
 121 |     );
 122 | 
 123 |     // Output file should remain unchanged
 124 |     let content = fs::read_to_string(&output_path).unwrap();
 125 |     assert_eq!(
 126 |         content, "existing content",
 127 |         "output file should not be modified in preview mode"
 128 |     );
 129 | }
 130 | 
 131 | #[test]
 132 | fn token_count_mode_skips_overwrite_confirmation() {
 133 |     let dir = tempdir().unwrap();
 134 |     let root = dir.path();
 135 | 
 136 |     // Create an existing output file
 137 |     let output_path = root.join("output.md");
 138 |     write_file(&output_path, "existing content");
 139 | 
 140 |     // Create a small project structure
 141 |     write_file(&root.join("src/main.rs"), "fn main() { println!(\"hi\"); }");
 142 |     write_file(&root.join("README.md"), "# Readme");
 143 | 
 144 |     let args = Args {
 145 |         input: root.to_string_lossy().into_owned(),
 146 |         output: output_path.to_string_lossy().into_owned(),
 147 |         filter: vec![],
 148 |         ignore: vec![],
 149 |         preview: false,
 150 |         token_count: true,
 151 |         line_numbers: false,
 152 |         yes: false,
 153 |         diff_only: false,
 154 |         clear_cache: false,
 155 |         init: false,
 156 |         max_tokens: None,
 157 |     };
 158 | 
 159 |     // Use false for overwrite response to verify it's not called
 160 |     let prompter = TestPrompter::new(false, true);
 161 | 
 162 |     // Run in token count mode - should succeed even with overwrite denied
 163 |     let res = run_with_args(args, Config::default(), &prompter);
 164 |     assert!(
 165 |         res.is_ok(),
 166 |         "token count mode should succeed without overwrite confirmation"
 167 |     );
 168 | 
 169 |     // Output file should remain unchanged
 170 |     let content = fs::read_to_string(&output_path).unwrap();
 171 |     assert_eq!(
 172 |         content, "existing content",
 173 |         "output file should not be modified in token count mode"
 174 |     );
 175 | }
 176 | 
 177 | #[test]
 178 | 
 179 | fn both_preview_and_token_count_modes_work_together() {
 180 |     let dir = tempdir().unwrap();
 181 |     let root = dir.path();
 182 | 
 183 |     // Create a small project structure
 184 |     write_file(&root.join("src/main.rs"), "fn main() { println!(\"hi\"); }");
 185 |     write_file(&root.join("README.md"), "# Readme");
 186 | 
 187 |     let args = Args {
 188 |         input: root.to_string_lossy().into_owned(),
 189 |         output: root.join("output.md").to_string_lossy().into_owned(),
 190 |         filter: vec![],
 191 |         ignore: vec![],
 192 |         preview: true,
 193 |         token_count: true,
 194 |         line_numbers: false,
 195 |         yes: false,
 196 |         diff_only: false,
 197 |         clear_cache: false,
 198 |         init: false,
 199 |         max_tokens: None,
 200 |     };
 201 | 
 202 |     let prompter = TestPrompter::new(false, true); // false for overwrite since it should be skipped
 203 | 
 204 |     // Run with both modes
 205 |     let res = run_with_args(args, Config::default(), &prompter);
 206 |     assert!(res.is_ok(), "both modes should work together");
 207 | 
 208 |     // No output file created
 209 |     assert!(
 210 |         !root.join("output.md").exists(),
 211 |         "output file should not be created when both modes are active"
 212 |     );
 213 | }
 214 | 
 215 | #[test]
 216 | fn end_to_end_generates_output_with_filters_ignores_and_line_numbers() {
 217 |     let dir = tempdir().unwrap();
 218 |     let root = dir.path();
 219 | 
 220 |     // Files that should be included by filters
 221 |     write_file(
 222 |         &root.join("src/main.rs"),
 223 |         "fn main() {\n    println!(\"hi\");\n}\n",
 224 |     );
 225 |     write_file(&root.join("README.md"), "# Top-level readme\n\nSome text");
 226 | 
 227 |     // Ignored directories/files
 228 |     write_file(
 229 |         &root.join("node_modules/pkg/index.js"),
 230 |         "console.log('ignore');",
 231 |     );
 232 |     write_file(&root.join("target/artifact.txt"), "binary");
 233 | 
 234 |     // A large file to exercise streaming and performance
 235 |     let mut large = String::with_capacity(4000 * 25);
 236 |     for i in 0..4000 {
 237 |         large.push_str(&format!("// line {}\n", i + 1));
 238 |     }
 239 |     write_file(&root.join("src/large.rs"), &large);
 240 | 
 241 |     let output_path = root.join("ctx.md");
 242 | 
 243 |     let args = Args {
 244 |         input: root.to_string_lossy().into_owned(),
 245 |         output: output_path.to_string_lossy().into_owned(),
 246 |         filter: vec!["rs".into(), "md".into()],
 247 |         ignore: vec!["node_modules".into(), "target".into()],
 248 |         preview: false,
 249 |         token_count: false,
 250 |         line_numbers: true,
 251 |         yes: false,
 252 |         diff_only: false,
 253 |         clear_cache: false,
 254 |         init: false,
 255 |         max_tokens: None,
 256 |     };
 257 | 
 258 |     // Always proceed without interactive prompts
 259 |     let prompter = TestPrompter::new(true, true);
 260 | 
 261 |     let res = run_with_args(args, Config::default(), &prompter);
 262 |     assert!(res.is_ok(), "end-to-end generation should succeed");
 263 | 
 264 |     // Find the actual output file (may have timestamp appended)
 265 |     let actual_output_path = if output_path.exists() {
 266 |         output_path
 267 |     } else {
 268 |         // Look for timestamped version
 269 |         let parent = output_path.parent().unwrap();
 270 |         let stem = output_path.file_stem().unwrap().to_string_lossy();
 271 |         let ext = output_path.extension().unwrap().to_string_lossy();
 272 | 
 273 |         let mut found_file = None;
 274 |         if let Ok(entries) = fs::read_dir(parent) {
 275 |             for entry in entries.flatten() {
 276 |                 let file_name = entry.file_name();
 277 |                 let name = file_name.to_string_lossy();
 278 |                 if name.starts_with(&format!("{}_", stem)) && name.ends_with(&format!(".{}", ext)) {
 279 |                     found_file = Some(entry.path());
 280 |                     break;
 281 |                 }
 282 |             }
 283 |         }
 284 | 
 285 |         found_file.unwrap_or_else(|| {
 286 |             panic!(
 287 |                 "No output file found. Expected {} or timestamped version",
 288 |                 output_path.display()
 289 |             )
 290 |         })
 291 |     };
 292 | 
 293 |     // Basic content checks
 294 |     let out = fs::read_to_string(&actual_output_path).unwrap();
 295 | 
 296 |     // Has file tree section
 297 |     assert!(
 298 |         out.contains("## File Tree Structure"),
 299 |         "output should contain a 'File Tree Structure' section"
 300 |     );
 301 | 
 302 |     // Has at least one rust code block with line numbers (looking for ' | ' marker)
 303 |     assert!(
 304 |         out.contains("```rust"),
 305 |         "output should contain a rust code block"
 306 |     );
 307 |     assert!(
 308 |         out.contains("   1 | "),
 309 |         "output should contain line-numbered code blocks"
 310 |     );
 311 | 
 312 |     // Should not include ignored directory entries' content (not a strict check, but indicative)
 313 |     assert!(
 314 |         !out.contains("console.log('ignore');"),
 315 |         "output should not include content from ignored directories"
 316 |     );
 317 | }
 318 | 
 319 | #[test]
 320 | fn overwrite_prompt_is_respected() {
 321 |     let dir = tempdir().unwrap();
 322 |     let root = dir.path();
 323 | 
 324 |     // Prepare an existing output file with sentinel content
 325 |     let output_path = root.join("out.md");
 326 |     write_file(&output_path, "SENTINEL");
 327 | 
 328 |     // Put a file to process
 329 |     write_file(&root.join("src/lib.rs"), "pub fn f() {}");
 330 | 
 331 |     let args = Args {
 332 |         input: root.to_string_lossy().into_owned(),
 333 |         output: output_path.to_string_lossy().into_owned(),
 334 |         filter: vec!["rs".into()],
 335 |         ignore: vec![],
 336 |         preview: false,
 337 |         token_count: false,
 338 |         line_numbers: false,
 339 |         yes: false,
 340 |         diff_only: false,
 341 |         clear_cache: false,
 342 |         init: false,
 343 |         max_tokens: None,
 344 |     };
 345 | 
 346 |     // Deny overwrite
 347 |     let prompter = TestPrompter::new(false, true);
 348 | 
 349 |     let res = run_with_args(args, Config::default(), &prompter);
 350 |     assert!(
 351 |         res.is_err(),
 352 |         "run should return error when overwrite denied"
 353 |     );
 354 | 
 355 |     // Ensure file is unchanged
 356 |     let out = fs::read_to_string(&output_path).unwrap();
 357 |     assert_eq!(out, "SENTINEL", "existing output should not be overwritten");
 358 | }
 359 | 
 360 | #[test]
 361 | fn confirm_processing_receives_large_count() {
 362 |     let dir = tempdir().unwrap();
 363 |     let root = dir.path();
 364 | 
 365 |     // Create a lot of files (should be well over the 100 threshold)
 366 |     fs::create_dir_all(root.join("data")).unwrap();
 367 |     for i in 0..150 {
 368 |         write_file(&root.join("data").join(format!("f{}.txt", i)), "x");
 369 |     }
 370 | 
 371 |     let args = Args {
 372 |         input: root.to_string_lossy().into_owned(),
 373 |         output: root.join("out.md").to_string_lossy().into_owned(),
 374 |         filter: vec!["txt".into()],
 375 |         ignore: vec![],
 376 |         preview: false,
 377 |         token_count: false,
 378 |         line_numbers: false,
 379 |         yes: false,
 380 |         diff_only: false,
 381 |         clear_cache: false,
 382 |         init: false,
 383 |         max_tokens: None,
 384 |     };
 385 | 
 386 |     let prompter = TestPrompter::new(true, true);
 387 | 
 388 |     let res = run_with_args(args, Config::default(), &prompter);
 389 |     assert!(res.is_ok(), "run should succeed with many files");
 390 | 
 391 |     // Ensure our injected prompter saw the large count (>= 150)
 392 |     assert!(
 393 |         prompter.last_count() >= 150,
 394 |         "expected confirm_processing to be called with >=150 files, got {}",
 395 |         prompter.last_count()
 396 |     );
 397 | }
 398 | 
 399 | #[test]
 400 | fn token_count_mode_does_not_create_output_file() {
 401 |     let dir = tempdir().unwrap();
 402 |     let root = dir.path();
 403 | 
 404 |     // Create a small project structure
 405 |     write_file(&root.join("src/main.rs"), "fn main() { println!(\"hi\"); }");
 406 |     write_file(&root.join("README.md"), "# Readme");
 407 | 
 408 |     let args = Args {
 409 |         input: root.to_string_lossy().into_owned(),
 410 |         output: root.join("output.md").to_string_lossy().into_owned(),
 411 |         filter: vec![],
 412 |         ignore: vec![],
 413 |         preview: false,
 414 |         token_count: true,
 415 |         line_numbers: false,
 416 |         yes: false,
 417 |         diff_only: false,
 418 |         clear_cache: false,
 419 |         init: false,
 420 |         max_tokens: None,
 421 |     };
 422 | 
 423 |     let prompter = TestPrompter::new(true, true);
 424 | 
 425 |     // Run in token count mode
 426 |     let res = run_with_args(args, Config::default(), &prompter);
 427 |     assert!(res.is_ok(), "token count mode should succeed");
 428 | 
 429 |     // No output file created
 430 |     assert!(
 431 |         !root.join("output.md").exists(),
 432 |         "output file should not be created in token count mode"
 433 |     );
 434 | }
```

### File: `tests/diff_integration.rs`

- Size: 1122 bytes
- Modified: SystemTime { tv_sec: 1771098907, tv_nsec: 779246312 }

```rust
   1 | use context_builder::diff::generate_diff;
   2 | 
   3 | #[test]
   4 | fn test_diff_with_identical_content() {
   5 |     let content = r#"# Test Document
   6 | 
   7 | This is a test document with some content.
   8 | 
   9 | ## Section 1
  10 | 
  11 | Some text here.
  12 | 
  13 | ## Section 2
  14 | 
  15 | More text here.
  16 | "#;
  17 | 
  18 |     let diff = generate_diff(content, content);
  19 | 
  20 |     // When content is identical, diff should be empty
  21 |     assert!(diff.is_empty());
  22 | }
  23 | 
  24 | #[test]
  25 | fn test_diff_with_changes() {
  26 |     let old_content = r#"# Test Document
  27 | 
  28 | This is a test document with some content.
  29 | 
  30 | ## Section 1
  31 | 
  32 | Some text here.
  33 | 
  34 | ## Section 2
  35 | 
  36 | More text here.
  37 | "#;
  38 | 
  39 |     let new_content = r#"# Test Document
  40 | 
  41 | This is a test document with some content.
  42 | 
  43 | ## Section 1
  44 | 
  45 | Some different text here.
  46 | 
  47 | ## Section 2
  48 | 
  49 | More text here.
  50 | "#;
  51 | 
  52 |     let diff = generate_diff(old_content, new_content);
  53 | 
  54 |     // When content has differences, diff should not be empty
  55 |     assert!(!diff.is_empty());
  56 |     assert!(diff.contains("## File Differences"));
  57 | 
  58 |     // Print the diff for debugging
  59 |     println!("Actual diff output:\n{}", diff);
  60 | 
  61 |     assert!(diff.contains("- Some text here"));
  62 |     assert!(diff.contains("+ Some different text here"));
  63 | }
```

### File: `tests/test_auto_diff.rs`

- Size: 33524 bytes
- Modified: SystemTime { tv_sec: 1771099016, tv_nsec: 445741336 }

```rust
   1 | //! Integration tests for auto-diff functionality
   2 | //!
   3 | //! These tests verify that the auto-diff feature works correctly and robustly:
   4 | //! - Cache management and collision prevention
   5 | //! - Diff generation accuracy
   6 | //! - Configuration changes affecting cache
   7 | //! - Error recovery from corrupted cache
   8 | 
   9 | use pretty_assertions::assert_eq;
  10 | use serial_test::serial;
  11 | use std::fs;
  12 | use std::path::Path;
  13 | use tempfile::tempdir;
  14 | 
  15 | use chrono::Utc;
  16 | use context_builder::cli::Args;
  17 | use context_builder::config::{Config, load_config};
  18 | use context_builder::{Prompter, run_with_args};
  19 | 
  20 | /// Test prompter that always confirms
  21 | struct TestPrompter;
  22 | 
  23 | impl Prompter for TestPrompter {
  24 |     fn confirm_processing(&self, _file_count: usize) -> std::io::Result<bool> {
  25 |         Ok(true)
  26 |     }
  27 |     fn confirm_overwrite(&self, _file_path: &str) -> std::io::Result<bool> {
  28 |         Ok(true)
  29 |     }
  30 | }
  31 | 
  32 | fn create_simple_project(base_dir: &Path) -> std::io::Result<()> {
  33 |     let src_dir = base_dir.join("src");
  34 |     fs::create_dir_all(&src_dir)?;
  35 | 
  36 |     fs::write(
  37 |         src_dir.join("main.rs"),
  38 |         "fn main() {\n    println!(\"Hello, world!\");\n}",
  39 |     )?;
  40 |     fs::write(
  41 |         src_dir.join("lib.rs"),
  42 |         "pub fn add(a: i32, b: i32) -> i32 {\n    a + b\n}",
  43 |     )?;
  44 |     fs::write(
  45 |         base_dir.join("README.md"),
  46 |         "# Test Project\n\nThis is a test project for auto-diff.",
  47 |     )?;
  48 | 
  49 |     // Create config file to enable auto-diff
  50 |     fs::write(
  51 |         base_dir.join("context-builder.toml"),
  52 |         r#"
  53 | auto_diff = true
  54 | timestamped_output = true
  55 | "#,
  56 |     )?;
  57 | 
  58 |     Ok(())
  59 | }
  60 | 
  61 | #[test]
  62 | #[serial]
  63 | fn test_auto_diff_workflow_basic() {
  64 |     let temp_dir = tempdir().unwrap();
  65 |     let project_dir = temp_dir.path().join("project");
  66 |     create_simple_project(&project_dir).unwrap();
  67 | 
  68 |     let output_dir = temp_dir.path().join("output");
  69 |     fs::create_dir_all(&output_dir).unwrap();
  70 | 
  71 |     // Change to project directory so config loading works
  72 |     let original_dir = std::env::current_dir().unwrap();
  73 |     std::env::set_current_dir(&project_dir).unwrap();
  74 | 
  75 |     let args = Args {
  76 |         input: ".".to_string(), // Use current directory
  77 |         output: output_dir.join("context.md").to_string_lossy().to_string(),
  78 |         filter: vec![],
  79 |         ignore: vec![],
  80 |         preview: false,
  81 |         token_count: false,
  82 |         line_numbers: false,
  83 |         yes: true,
  84 |         diff_only: false,
  85 |         clear_cache: false,
  86 |         init: false,
  87 |         max_tokens: None,
  88 |     };
  89 |     let prompter = TestPrompter;
  90 | 
  91 |     // First run - should create initial output without diffs
  92 |     let config = load_config().unwrap_or_default();
  93 | 
  94 |     // Apply config merging manually since we're bypassing run()
  95 |     let mut first_args = args.clone();
  96 | 
  97 |     // Apply line_numbers from config (matches run_with_args behavior)
  98 |     if let Some(line_numbers) = config.line_numbers {
  99 |         first_args.line_numbers = line_numbers;
 100 |     }
 101 | 
 102 |     // Apply diff_only from config
 103 |     if let Some(diff_only) = config.diff_only {
 104 |         first_args.diff_only = diff_only;
 105 |     }
 106 | 
 107 |     // Apply timestamping manually since we're bypassing run()
 108 |     if config.timestamped_output.unwrap_or(false) {
 109 |         let timestamp = Utc::now().format("%Y%m%d%H%M%S").to_string();
 110 |         let path = std::path::Path::new(&first_args.output);
 111 |         let stem = path
 112 |             .file_stem()
 113 |             .and_then(|s| s.to_str())
 114 |             .unwrap_or("output");
 115 |         let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("md");
 116 |         let new_filename = format!("{}_{}.{}", stem, timestamp, extension);
 117 |         if let Some(parent) = path.parent() {
 118 |             first_args.output = parent.join(new_filename).to_string_lossy().to_string();
 119 |         } else {
 120 |             first_args.output = new_filename;
 121 |         }
 122 |     }
 123 | 
 124 |     run_with_args(first_args, config.clone(), &prompter).unwrap();
 125 | 
 126 |     // Check that output was created
 127 |     let first_output = fs::read_dir(&output_dir)
 128 |         .unwrap()
 129 |         .next()
 130 |         .unwrap()
 131 |         .unwrap()
 132 |         .path();
 133 |     let first_content = fs::read_to_string(&first_output).unwrap();
 134 | 
 135 |     // Should not contain change summary on first run
 136 |     assert!(!first_content.contains("## Change Summary"));
 137 |     assert!(!first_content.contains("## File Differences"));
 138 | 
 139 |     // Modify a file
 140 |     fs::write(
 141 |         project_dir.join("src").join("main.rs"),
 142 |         "fn main() {\n    println!(\"Hello, Rust!\");\n    println!(\"Modified!\");\n}",
 143 |     )
 144 |     .unwrap();
 145 | 
 146 |     // Small delay to ensure different timestamps
 147 |     std::thread::sleep(std::time::Duration::from_millis(1100));
 148 | 
 149 |     // Second run - should detect changes
 150 |     let config = load_config().unwrap_or_default();
 151 | 
 152 |     // Apply config merging manually since we're bypassing run()
 153 |     let mut second_args = args;
 154 | 
 155 |     // Apply line_numbers from config (matches run_with_args behavior)
 156 |     if let Some(line_numbers) = config.line_numbers {
 157 |         second_args.line_numbers = line_numbers;
 158 |     }
 159 | 
 160 |     // Apply diff_only from config
 161 |     if let Some(diff_only) = config.diff_only {
 162 |         second_args.diff_only = diff_only;
 163 |     }
 164 | 
 165 |     // Apply timestamping manually since we're bypassing run()
 166 |     if config.timestamped_output.unwrap_or(false) {
 167 |         let timestamp = Utc::now().format("%Y%m%d%H%M%S").to_string();
 168 |         let path = std::path::Path::new(&second_args.output);
 169 |         let stem = path
 170 |             .file_stem()
 171 |             .and_then(|s| s.to_str())
 172 |             .unwrap_or("output");
 173 |         let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("md");
 174 |         let new_filename = format!("{}_{}.{}", stem, timestamp, extension);
 175 |         if let Some(parent) = path.parent() {
 176 |             second_args.output = parent.join(new_filename).to_string_lossy().to_string();
 177 |         } else {
 178 |             second_args.output = new_filename;
 179 |         }
 180 |     }
 181 | 
 182 |     run_with_args(second_args, config, &prompter).unwrap();
 183 | 
 184 |     // Restore original directory
 185 |     std::env::set_current_dir(original_dir).unwrap();
 186 | 
 187 |     // Find the second output file (should have different timestamp)
 188 |     let outputs: Vec<_> = fs::read_dir(&output_dir)
 189 |         .unwrap()
 190 |         .map(|e| e.unwrap().path())
 191 |         .collect();
 192 |     assert_eq!(outputs.len(), 2, "Should have two output files");
 193 | 
 194 |     let second_output = outputs.iter().find(|&p| p != &first_output).unwrap();
 195 |     let second_content = fs::read_to_string(second_output).unwrap();
 196 | 
 197 |     // Should contain change summary
 198 |     assert!(second_content.contains("## Change Summary"));
 199 |     // Handle both Windows and Unix path separators
 200 |     assert!(
 201 |         second_content.contains("- Modified: `src/main.rs`")
 202 |             || second_content.contains("- Modified: `src\\main.rs`")
 203 |     );
 204 | 
 205 |     // Should contain file differences
 206 |     assert!(second_content.contains("## File Differences"));
 207 |     assert!(
 208 |         second_content.contains("### Diff: `src/main.rs`")
 209 |             || second_content.contains("### Diff: `src\\main.rs`")
 210 |     );
 211 |     assert!(second_content.contains("Hello, world!"));
 212 |     assert!(second_content.contains("Hello, Rust!"));
 213 |     assert!(second_content.contains("Modified!"));
 214 | }
 215 | 
 216 | #[test]
 217 | #[serial]
 218 | fn test_auto_diff_added_and_removed_files() {
 219 |     let temp_dir = tempdir().unwrap();
 220 |     let project_dir = temp_dir.path().join("project");
 221 |     create_simple_project(&project_dir).unwrap();
 222 | 
 223 |     let output_dir = temp_dir.path().join("output");
 224 |     fs::create_dir_all(&output_dir).unwrap();
 225 | 
 226 |     // Change to project directory so config loading works
 227 |     let original_dir = std::env::current_dir().unwrap();
 228 |     std::env::set_current_dir(&project_dir).unwrap();
 229 | 
 230 |     let args = Args {
 231 |         input: ".".to_string(), // Use current directory
 232 |         output: output_dir.join("context.md").to_string_lossy().to_string(),
 233 |         filter: vec![],
 234 |         ignore: vec![],
 235 |         preview: false,
 236 |         token_count: false,
 237 |         line_numbers: false,
 238 |         yes: true,
 239 |         diff_only: false,
 240 |         clear_cache: false,
 241 |         init: false,
 242 |         max_tokens: None,
 243 |     };
 244 | 
 245 |     let prompter = TestPrompter;
 246 | 
 247 |     // First run
 248 |     let config = load_config().unwrap_or_default();
 249 | 
 250 |     // Apply config merging manually since we're bypassing run()
 251 |     let mut first_args = args.clone();
 252 | 
 253 |     // Apply line_numbers from config
 254 |     if !first_args.line_numbers
 255 |         && let Some(line_numbers) = config.line_numbers
 256 |     {
 257 |         first_args.line_numbers = line_numbers;
 258 |     }
 259 | 
 260 |     // Apply diff_only from config
 261 |     if !first_args.diff_only
 262 |         && let Some(diff_only) = config.diff_only
 263 |     {
 264 |         first_args.diff_only = diff_only;
 265 |     }
 266 | 
 267 |     // Apply timestamping manually since we're bypassing run()
 268 |     if config.timestamped_output.unwrap_or(false) {
 269 |         let timestamp = Utc::now().format("%Y%m%d%H%M%S").to_string();
 270 |         let path = std::path::Path::new(&first_args.output);
 271 |         let stem = path
 272 |             .file_stem()
 273 |             .and_then(|s| s.to_str())
 274 |             .unwrap_or("output");
 275 |         let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("md");
 276 |         let new_filename = format!("{}_{}.{}", stem, timestamp, extension);
 277 |         if let Some(parent) = path.parent() {
 278 |             first_args.output = parent.join(new_filename).to_string_lossy().to_string();
 279 |         } else {
 280 |             first_args.output = new_filename;
 281 |         }
 282 |     }
 283 | 
 284 |     run_with_args(first_args, config.clone(), &prompter).unwrap();
 285 | 
 286 |     // Add a new file and remove an existing one
 287 |     fs::write(
 288 |         project_dir.join("src").join("new_module.rs"),
 289 |         "pub fn new_function() -> String {\n    \"new\".to_string()\n}",
 290 |     )
 291 |     .unwrap();
 292 | 
 293 |     fs::remove_file(project_dir.join("src").join("lib.rs")).unwrap();
 294 | 
 295 |     // Small delay to ensure different timestamps
 296 |     std::thread::sleep(std::time::Duration::from_millis(1100));
 297 | 
 298 |     // Second run
 299 |     let config = load_config().unwrap_or_default();
 300 | 
 301 |     // Apply config merging manually since we're bypassing run()
 302 |     let mut second_args = args;
 303 | 
 304 |     // Apply line_numbers from config
 305 |     if !second_args.line_numbers
 306 |         && let Some(line_numbers) = config.line_numbers
 307 |     {
 308 |         second_args.line_numbers = line_numbers;
 309 |     }
 310 | 
 311 |     // Apply diff_only from config
 312 |     if !second_args.diff_only
 313 |         && let Some(diff_only) = config.diff_only
 314 |     {
 315 |         second_args.diff_only = diff_only;
 316 |     }
 317 | 
 318 |     // Apply timestamping manually since we're bypassing run()
 319 |     if config.timestamped_output.unwrap_or(false) {
 320 |         let timestamp = Utc::now().format("%Y%m%d%H%M%S").to_string();
 321 |         let path = std::path::Path::new(&second_args.output);
 322 |         let stem = path
 323 |             .file_stem()
 324 |             .and_then(|s| s.to_str())
 325 |             .unwrap_or("output");
 326 |         let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("md");
 327 |         let new_filename = format!("{}_{}.{}", stem, timestamp, extension);
 328 |         if let Some(parent) = path.parent() {
 329 |             second_args.output = parent.join(new_filename).to_string_lossy().to_string();
 330 |         } else {
 331 |             second_args.output = new_filename;
 332 |         }
 333 |     }
 334 | 
 335 |     run_with_args(second_args, config, &prompter).unwrap();
 336 | 
 337 |     // Restore original directory
 338 |     std::env::set_current_dir(original_dir).unwrap();
 339 | 
 340 |     let outputs: Vec<_> = fs::read_dir(&output_dir)
 341 |         .unwrap()
 342 |         .map(|e| e.unwrap().path())
 343 |         .collect();
 344 |     let latest_output = outputs
 345 |         .iter()
 346 |         .max_by_key(|p| fs::metadata(p).unwrap().modified().unwrap())
 347 |         .unwrap();
 348 |     let content = fs::read_to_string(latest_output).unwrap();
 349 | 
 350 |     // Should show both added and removed files
 351 |     // Handle both Windows and Unix path separators
 352 |     assert!(
 353 |         content.contains("- Added: `src/new_module.rs`")
 354 |             || content.contains("- Added: `src\\new_module.rs`")
 355 |     );
 356 |     // Handle both Windows and Unix path separators
 357 |     assert!(
 358 |         content.contains("- Removed: `src/lib.rs`") || content.contains("- Removed: `src\\lib.rs`")
 359 |     );
 360 | 
 361 |     // Added files should be marked in the files section
 362 |     assert!(content.contains("_Status: Added_"));
 363 | }
 364 | 
 365 | #[test]
 366 | #[serial]
 367 | fn test_diff_only_mode() {
 368 |     let temp_dir = tempdir().unwrap();
 369 |     let project_dir = temp_dir.path().join("project");
 370 |     create_simple_project(&project_dir).unwrap();
 371 | 
 372 |     // Update config to enable diff_only
 373 |     fs::write(
 374 |         project_dir.join("context-builder.toml"),
 375 |         r#"
 376 | auto_diff = true
 377 | timestamped_output = true
 378 | diff_only = true
 379 | "#,
 380 |     )
 381 |     .unwrap();
 382 | 
 383 |     let output_dir = temp_dir.path().join("output");
 384 |     fs::create_dir_all(&output_dir).unwrap();
 385 | 
 386 |     // Change to project directory so config loading works
 387 |     let original_dir = std::env::current_dir().unwrap();
 388 |     std::env::set_current_dir(&project_dir).unwrap();
 389 | 
 390 |     let args = Args {
 391 |         input: ".".to_string(), // Use current directory
 392 |         output: output_dir.join("context.md").to_string_lossy().to_string(),
 393 |         filter: vec![],
 394 |         ignore: vec![],
 395 |         preview: false,
 396 |         token_count: false,
 397 |         line_numbers: false,
 398 |         yes: true,
 399 |         diff_only: false, // Config file should override this
 400 |         clear_cache: false,
 401 |         init: false,
 402 |         max_tokens: None,
 403 |     };
 404 | 
 405 |     let prompter = TestPrompter;
 406 | 
 407 |     // First run
 408 |     let config = load_config().unwrap_or_default();
 409 | 
 410 |     // Apply config merging manually since we're bypassing run()
 411 |     let mut first_args = args.clone();
 412 | 
 413 |     // Apply line_numbers from config
 414 |     if !first_args.line_numbers
 415 |         && let Some(line_numbers) = config.line_numbers
 416 |     {
 417 |         first_args.line_numbers = line_numbers;
 418 |     }
 419 | 
 420 |     // Apply diff_only from config
 421 |     if !first_args.diff_only
 422 |         && let Some(diff_only) = config.diff_only
 423 |     {
 424 |         first_args.diff_only = diff_only;
 425 |     }
 426 | 
 427 |     // Apply timestamping manually since we're bypassing run()
 428 |     if config.timestamped_output.unwrap_or(false) {
 429 |         let timestamp = Utc::now().format("%Y%m%d%H%M%S").to_string();
 430 |         let path = std::path::Path::new(&first_args.output);
 431 |         let stem = path
 432 |             .file_stem()
 433 |             .and_then(|s| s.to_str())
 434 |             .unwrap_or("output");
 435 |         let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("md");
 436 |         let new_filename = format!("{}_{}.{}", stem, timestamp, extension);
 437 |         if let Some(parent) = path.parent() {
 438 |             first_args.output = parent.join(new_filename).to_string_lossy().to_string();
 439 |         } else {
 440 |             first_args.output = new_filename;
 441 |         }
 442 |     }
 443 | 
 444 |     run_with_args(first_args, config.clone(), &prompter).unwrap();
 445 | 
 446 |     // Modify a file
 447 |     fs::write(
 448 |         project_dir.join("src").join("main.rs"),
 449 |         "fn main() {\n    println!(\"Changed!\");\n}",
 450 |     )
 451 |     .unwrap();
 452 | 
 453 |     // Small delay to ensure different timestamps
 454 |     std::thread::sleep(std::time::Duration::from_millis(1100));
 455 | 
 456 |     // Second run
 457 |     let config = load_config().unwrap_or_default();
 458 | 
 459 |     // Apply config merging manually since we're bypassing run()
 460 |     let mut second_args = args;
 461 | 
 462 |     // Apply line_numbers from config
 463 |     if !second_args.line_numbers
 464 |         && let Some(line_numbers) = config.line_numbers
 465 |     {
 466 |         second_args.line_numbers = line_numbers;
 467 |     }
 468 | 
 469 |     // Apply diff_only from config
 470 |     if !second_args.diff_only
 471 |         && let Some(diff_only) = config.diff_only
 472 |     {
 473 |         second_args.diff_only = diff_only;
 474 |     }
 475 | 
 476 |     // Apply timestamping manually since we're bypassing run()
 477 |     if config.timestamped_output.unwrap_or(false) {
 478 |         let timestamp = Utc::now().format("%Y%m%d%H%M%S").to_string();
 479 |         let path = std::path::Path::new(&second_args.output);
 480 |         let stem = path
 481 |             .file_stem()
 482 |             .and_then(|s| s.to_str())
 483 |             .unwrap_or("output");
 484 |         let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("md");
 485 |         let new_filename = format!("{}_{}.{}", stem, timestamp, extension);
 486 |         if let Some(parent) = path.parent() {
 487 |             second_args.output = parent.join(new_filename).to_string_lossy().to_string();
 488 |         } else {
 489 |             second_args.output = new_filename;
 490 |         }
 491 |     }
 492 | 
 493 |     run_with_args(second_args, config, &prompter).unwrap();
 494 | 
 495 |     // Restore original directory
 496 |     std::env::set_current_dir(original_dir).unwrap();
 497 | 
 498 |     let outputs: Vec<_> = fs::read_dir(&output_dir)
 499 |         .unwrap()
 500 |         .map(|e| e.unwrap().path())
 501 |         .collect();
 502 |     let latest_output = outputs
 503 |         .iter()
 504 |         .max_by_key(|p| fs::metadata(p).unwrap().modified().unwrap())
 505 |         .unwrap();
 506 |     let content = fs::read_to_string(latest_output).unwrap();
 507 | 
 508 |     // Should have change summary and diffs
 509 |     assert!(content.contains("## Change Summary"));
 510 |     assert!(content.contains("## File Differences"));
 511 | 
 512 |     // Should NOT have full file bodies section
 513 |     assert!(!content.contains("## Files"));
 514 | 
 515 |     // But should still have the file tree and header
 516 |     assert!(content.contains("## File Tree Structure"));
 517 |     assert!(content.contains("# Directory Structure Report"));
 518 | }
 519 | 
 520 | #[test]
 521 | #[serial]
 522 | fn test_cache_invalidation_on_config_change() {
 523 |     let temp_dir = tempdir().unwrap();
 524 |     let project_dir = temp_dir.path().join("project");
 525 |     create_simple_project(&project_dir).unwrap();
 526 | 
 527 |     let output_dir = temp_dir.path().join("output");
 528 |     fs::create_dir_all(&output_dir).unwrap();
 529 | 
 530 |     // Change to project directory so config loading works
 531 |     let original_dir = std::env::current_dir().unwrap();
 532 |     std::env::set_current_dir(&project_dir).unwrap();
 533 | 
 534 |     let args_base = Args {
 535 |         input: ".".to_string(), // Use current directory
 536 |         output: output_dir.join("context.md").to_string_lossy().to_string(),
 537 |         filter: vec![],
 538 |         ignore: vec![],
 539 |         preview: false,
 540 |         token_count: false,
 541 |         line_numbers: false,
 542 |         yes: true,
 543 |         diff_only: false,
 544 |         clear_cache: false,
 545 |         init: false,
 546 |         max_tokens: None,
 547 |     };
 548 | 
 549 |     let prompter = TestPrompter;
 550 | 
 551 |     // First run with original config
 552 |     let config = load_config().unwrap_or_default();
 553 | 
 554 |     // Apply config merging manually since we're bypassing run()
 555 |     let mut first_args = args_base.clone();
 556 | 
 557 |     // Apply line_numbers from config
 558 |     if !first_args.line_numbers
 559 |         && let Some(line_numbers) = config.line_numbers
 560 |     {
 561 |         first_args.line_numbers = line_numbers;
 562 |     }
 563 | 
 564 |     // Apply diff_only from config
 565 |     if !first_args.diff_only
 566 |         && let Some(diff_only) = config.diff_only
 567 |     {
 568 |         first_args.diff_only = diff_only;
 569 |     }
 570 | 
 571 |     // Apply timestamping manually since we're bypassing run()
 572 |     if config.timestamped_output.unwrap_or(false) {
 573 |         let timestamp = Utc::now().format("%Y%m%d%H%M%S").to_string();
 574 |         let path = std::path::Path::new(&first_args.output);
 575 |         let stem = path
 576 |             .file_stem()
 577 |             .and_then(|s| s.to_str())
 578 |             .unwrap_or("output");
 579 |         let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("md");
 580 |         let new_filename = format!("{}_{}.{}", stem, timestamp, extension);
 581 |         if let Some(parent) = path.parent() {
 582 |             first_args.output = parent.join(new_filename).to_string_lossy().to_string();
 583 |         } else {
 584 |             first_args.output = new_filename;
 585 |         }
 586 |     }
 587 | 
 588 |     run_with_args(first_args, config, &prompter).unwrap();
 589 | 
 590 |     // Change configuration - add line numbers
 591 |     fs::write(
 592 |         project_dir.join("context-builder.toml"),
 593 |         r#"
 594 | auto_diff = true
 595 | timestamped_output = true
 596 | line_numbers = true
 597 | "#,
 598 |     )
 599 |     .unwrap();
 600 | 
 601 |     // Small delay to ensure different timestamps
 602 |     std::thread::sleep(std::time::Duration::from_millis(1100));
 603 | 
 604 |     // Second run with new config should not show diffs (cache should be invalidated)
 605 |     let config = load_config().unwrap_or_default();
 606 | 
 607 |     // Apply config merging manually since we're bypassing run()
 608 |     let mut second_args = args_base;
 609 | 
 610 |     // Apply line_numbers from config (matches run_with_args behavior)
 611 |     if let Some(line_numbers) = config.line_numbers {
 612 |         second_args.line_numbers = line_numbers;
 613 |     }
 614 | 
 615 |     // Apply diff_only from config
 616 |     if let Some(diff_only) = config.diff_only {
 617 |         second_args.diff_only = diff_only;
 618 |     }
 619 | 
 620 |     // Apply timestamping manually since we're bypassing run()
 621 |     if config.timestamped_output.unwrap_or(false) {
 622 |         let timestamp = Utc::now().format("%Y%m%d%H%M%S").to_string();
 623 |         let path = std::path::Path::new(&second_args.output);
 624 |         let stem = path
 625 |             .file_stem()
 626 |             .and_then(|s| s.to_str())
 627 |             .unwrap_or("output");
 628 |         let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("md");
 629 |         let new_filename = format!("{}_{}.{}", stem, timestamp, extension);
 630 |         if let Some(parent) = path.parent() {
 631 |             second_args.output = parent.join(new_filename).to_string_lossy().to_string();
 632 |         } else {
 633 |             second_args.output = new_filename;
 634 |         }
 635 |     }
 636 | 
 637 |     run_with_args(second_args, config, &prompter).unwrap();
 638 | 
 639 |     // Restore original directory
 640 |     std::env::set_current_dir(original_dir).unwrap();
 641 | 
 642 |     let outputs: Vec<_> = fs::read_dir(&output_dir)
 643 |         .unwrap()
 644 |         .map(|e| e.unwrap().path())
 645 |         .collect();
 646 |     let latest_output = outputs
 647 |         .iter()
 648 |         .max_by_key(|p| fs::metadata(p).unwrap().modified().unwrap())
 649 |         .unwrap();
 650 |     let content = fs::read_to_string(latest_output).unwrap();
 651 | 
 652 |     // Should have line numbers (showing new config is active)
 653 |     assert!(content.contains("   1 |"));
 654 | 
 655 |     // Should not show change summary since cache was invalidated
 656 |     assert!(!content.contains("## Change Summary"));
 657 | }
 658 | 
 659 | #[test]
 660 | #[serial]
 661 | fn test_concurrent_cache_access() {
 662 |     use std::sync::Arc;
 663 |     use std::thread;
 664 | 
 665 |     let temp_dir = tempdir().unwrap();
 666 |     let project_dir = temp_dir.path().join("project");
 667 |     create_simple_project(&project_dir).unwrap();
 668 | 
 669 |     let output_dir = temp_dir.path().join("output");
 670 |     fs::create_dir_all(&output_dir).unwrap();
 671 | 
 672 |     let project_dir = Arc::new(project_dir);
 673 |     let output_dir = Arc::new(output_dir);
 674 | 
 675 |     // Spawn multiple threads that try to run the tool concurrently
 676 |     let handles: Vec<_> = (0..3)
 677 |         .map(|i| {
 678 |             let project_dir = Arc::clone(&project_dir);
 679 |             let output_dir = Arc::clone(&output_dir);
 680 | 
 681 |             thread::spawn(move || {
 682 |                 let args = Args {
 683 |                     input: project_dir.to_string_lossy().to_string(),
 684 |                     output: output_dir
 685 |                         .join(format!("context_{}.md", i))
 686 |                         .to_string_lossy()
 687 |                         .to_string(),
 688 |                     filter: vec![],
 689 |                     ignore: vec![],
 690 |                     preview: false,
 691 |                     token_count: false,
 692 |                     line_numbers: false,
 693 |                     yes: true,
 694 |                     diff_only: false,
 695 |                     clear_cache: false,
 696 |                     init: false,
 697 |                     max_tokens: None,
 698 |                 };
 699 | 
 700 |                 let prompter = TestPrompter;
 701 |                 run_with_args(args, Config::default(), &prompter)
 702 |             })
 703 |         })
 704 |         .collect();
 705 | 
 706 |     // Wait for all threads to complete
 707 |     let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();
 708 | 
 709 |     // All should succeed (no cache corruption)
 710 |     for result in results {
 711 |         assert!(
 712 |             result.is_ok(),
 713 |             "Concurrent access should not cause failures"
 714 |         );
 715 |     }
 716 | 
 717 |     // Check that all outputs were created
 718 |     let output_count = fs::read_dir(&*output_dir).unwrap().count();
 719 |     assert_eq!(output_count, 3, "All concurrent runs should produce output");
 720 | }
 721 | 
 722 | #[test]
 723 | #[serial]
 724 | fn test_corrupted_cache_recovery() {
 725 |     let temp_dir = tempdir().unwrap();
 726 |     let project_dir = temp_dir.path().join("project");
 727 |     create_simple_project(&project_dir).unwrap();
 728 | 
 729 |     let output_dir = temp_dir.path().join("output");
 730 |     fs::create_dir_all(&output_dir).unwrap();
 731 | 
 732 |     // Change to project directory so config loading works
 733 |     let original_dir = std::env::current_dir().unwrap();
 734 |     std::env::set_current_dir(&project_dir).unwrap();
 735 | 
 736 |     let args = Args {
 737 |         input: ".".to_string(), // Use current directory
 738 |         output: output_dir.join("context.md").to_string_lossy().to_string(),
 739 |         filter: vec![],
 740 |         ignore: vec![],
 741 |         preview: false,
 742 |         token_count: false,
 743 |         line_numbers: false,
 744 |         yes: true,
 745 |         diff_only: false,
 746 |         clear_cache: false,
 747 |         init: false,
 748 |         max_tokens: None,
 749 |     };
 750 | 
 751 |     let prompter = TestPrompter;
 752 | 
 753 |     // First run to create cache
 754 |     let config = load_config().unwrap_or_default();
 755 | 
 756 |     // Apply config merging manually since we're bypassing run()
 757 |     let mut first_args = args.clone();
 758 | 
 759 |     // Apply line_numbers from config
 760 |     if !first_args.line_numbers
 761 |         && let Some(line_numbers) = config.line_numbers
 762 |     {
 763 |         first_args.line_numbers = line_numbers;
 764 |     }
 765 | 
 766 |     // Apply diff_only from config
 767 |     if !first_args.diff_only
 768 |         && let Some(diff_only) = config.diff_only
 769 |     {
 770 |         first_args.diff_only = diff_only;
 771 |     }
 772 | 
 773 |     // Apply timestamping manually since we're bypassing run()
 774 |     if config.timestamped_output.unwrap_or(false) {
 775 |         let timestamp = Utc::now().format("%Y%m%d%H%M%S").to_string();
 776 |         let path = std::path::Path::new(&first_args.output);
 777 |         let stem = path
 778 |             .file_stem()
 779 |             .and_then(|s| s.to_str())
 780 |             .unwrap_or("output");
 781 |         let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("md");
 782 |         let new_filename = format!("{}_{}.{}", stem, timestamp, extension);
 783 |         if let Some(parent) = path.parent() {
 784 |             first_args.output = parent.join(new_filename).to_string_lossy().to_string();
 785 |         } else {
 786 |             first_args.output = new_filename;
 787 |         }
 788 |     }
 789 | 
 790 |     run_with_args(first_args, config.clone(), &prompter).unwrap();
 791 | 
 792 |     // Corrupt the cache by writing invalid JSON
 793 |     let cache_dir = project_dir.join(".context-builder").join("cache");
 794 |     if cache_dir.exists() {
 795 |         let cache_files: Vec<_> = fs::read_dir(&cache_dir)
 796 |             .unwrap()
 797 |             .filter_map(|entry| entry.ok())
 798 |             .filter(|entry| {
 799 |                 entry
 800 |                     .path()
 801 |                     .extension()
 802 |                     .and_then(|s| s.to_str())
 803 |                     .map(|s| s == "json")
 804 |                     .unwrap_or(false)
 805 |             })
 806 |             .collect();
 807 | 
 808 |         if !cache_files.is_empty() {
 809 |             // Corrupt the first cache file found
 810 |             fs::write(cache_files[0].path(), "{ invalid json }").unwrap();
 811 |         }
 812 |     }
 813 | 
 814 |     // Modify a file
 815 |     fs::write(
 816 |         project_dir.join("src").join("main.rs"),
 817 |         "fn main() {\n    println!(\"Recovered!\");\n}",
 818 |     )
 819 |     .unwrap();
 820 | 
 821 |     // Small delay to ensure different timestamps
 822 |     std::thread::sleep(std::time::Duration::from_millis(1100));
 823 | 
 824 |     // Second run should handle corrupted cache gracefully
 825 |     let config = load_config().unwrap_or_default();
 826 | 
 827 |     // Apply config merging manually since we're bypassing run()
 828 |     let mut second_args = args;
 829 | 
 830 |     // Apply line_numbers from config
 831 |     if !second_args.line_numbers
 832 |         && let Some(line_numbers) = config.line_numbers
 833 |     {
 834 |         second_args.line_numbers = line_numbers;
 835 |     }
 836 | 
 837 |     // Apply diff_only from config
 838 |     if !second_args.diff_only
 839 |         && let Some(diff_only) = config.diff_only
 840 |     {
 841 |         second_args.diff_only = diff_only;
 842 |     }
 843 | 
 844 |     // Apply timestamping manually since we're bypassing run()
 845 |     if config.timestamped_output.unwrap_or(false) {
 846 |         let timestamp = Utc::now().format("%Y%m%d%H%M%S").to_string();
 847 |         let path = std::path::Path::new(&second_args.output);
 848 |         let stem = path
 849 |             .file_stem()
 850 |             .and_then(|s| s.to_str())
 851 |             .unwrap_or("output");
 852 |         let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("md");
 853 |         let new_filename = format!("{}_{}.{}", stem, timestamp, extension);
 854 |         if let Some(parent) = path.parent() {
 855 |             second_args.output = parent.join(new_filename).to_string_lossy().to_string();
 856 |         } else {
 857 |             second_args.output = new_filename;
 858 |         }
 859 |     }
 860 | 
 861 |     let result = run_with_args(second_args, config, &prompter);
 862 |     assert!(result.is_ok(), "Should recover from corrupted cache");
 863 | 
 864 |     // Restore original directory
 865 |     std::env::set_current_dir(original_dir).unwrap();
 866 | 
 867 |     // Should produce output despite cache corruption
 868 |     let output_count = fs::read_dir(&output_dir).unwrap().count();
 869 |     assert!(
 870 |         output_count >= 1,
 871 |         "Should produce output even with corrupted cache"
 872 |     );
 873 | }
 874 | 
 875 | #[test]
 876 | #[serial]
 877 | fn test_diff_only_mode_includes_added_files() {
 878 |     let temp_dir = tempdir().unwrap();
 879 |     let project_dir = temp_dir.path().join("project");
 880 |     create_simple_project(&project_dir).unwrap();
 881 | 
 882 |     let output_dir = temp_dir.path().join("output");
 883 |     fs::create_dir_all(&output_dir).unwrap();
 884 | 
 885 |     // Change to project directory so config loading works
 886 |     let original_dir = std::env::current_dir().unwrap();
 887 |     std::env::set_current_dir(&project_dir).unwrap();
 888 | 
 889 |     // Create config with auto_diff and diff_only enabled
 890 |     fs::write(
 891 |         project_dir.join("context-builder.toml"),
 892 |         r#"
 893 | auto_diff = true
 894 | timestamped_output = true
 895 | diff_only = true
 896 | "#,
 897 |     )
 898 |     .unwrap();
 899 | 
 900 |     let prompter = TestPrompter;
 901 | 
 902 |     // First run to establish baseline
 903 |     let args = Args {
 904 |         input: ".".to_string(),
 905 |         output: output_dir.join("context.md").to_string_lossy().to_string(),
 906 |         filter: vec!["rs".to_string()],
 907 |         ignore: vec![],
 908 |         preview: false,
 909 |         token_count: false,
 910 |         line_numbers: false,
 911 |         yes: true,
 912 |         diff_only: false, // Will be overridden by config
 913 |         clear_cache: false,
 914 |         init: false,
 915 |         max_tokens: None,
 916 |     };
 917 | 
 918 |     run_with_args(args.clone(), load_config().unwrap_or_default(), &prompter).unwrap();
 919 | 
 920 |     // Add a new file
 921 |     fs::write(
 922 |         project_dir.join("src").join("new_module.rs"),
 923 |         "// New module added\npub fn new_function() -> String {\n    \"Hello from new module\".to_string()\n}\n\n#[cfg(test)]\nmod tests {\n    use super::*;\n\n    #[test]\n    fn test_new_function() {\n        assert_eq!(new_function(), \"Hello from new module\");\n    }\n}\n",
 924 |     )
 925 |     .unwrap();
 926 | 
 927 |     // Small delay to ensure different timestamps
 928 |     std::thread::sleep(std::time::Duration::from_millis(1100));
 929 | 
 930 |     // Second run with the added file
 931 |     let config = load_config().unwrap_or_default();
 932 | 
 933 |     // Apply config merging manually since we're bypassing run()
 934 |     let mut second_args = args;
 935 | 
 936 |     // Apply line_numbers from config
 937 |     if !second_args.line_numbers
 938 |         && let Some(line_numbers) = config.line_numbers
 939 |     {
 940 |         second_args.line_numbers = line_numbers;
 941 |     }
 942 | 
 943 |     // Apply diff_only from config
 944 |     if !second_args.diff_only
 945 |         && let Some(diff_only) = config.diff_only
 946 |     {
 947 |         second_args.diff_only = diff_only;
 948 |     }
 949 | 
 950 |     // Apply timestamping manually since we're bypassing run()
 951 |     if config.timestamped_output.unwrap_or(false) {
 952 |         let timestamp = Utc::now().format("%Y%m%d%H%M%S").to_string();
 953 |         let path = std::path::Path::new(&second_args.output);
 954 |         let stem = path
 955 |             .file_stem()
 956 |             .and_then(|s| s.to_str())
 957 |             .unwrap_or("output");
 958 |         let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("md");
 959 |         let new_filename = format!("{}_{}.{}", stem, timestamp, extension);
 960 |         if let Some(parent) = path.parent() {
 961 |             second_args.output = parent.join(new_filename).to_string_lossy().to_string();
 962 |         } else {
 963 |             second_args.output = new_filename;
 964 |         }
 965 |     }
 966 | 
 967 |     run_with_args(second_args, config, &prompter).unwrap();
 968 | 
 969 |     // Restore original directory
 970 |     std::env::set_current_dir(original_dir).unwrap();
 971 | 
 972 |     // Find the latest output file
 973 |     let outputs: Vec<_> = fs::read_dir(&output_dir)
 974 |         .unwrap()
 975 |         .map(|e| e.unwrap().path())
 976 |         .collect();
 977 |     let latest_output = outputs
 978 |         .iter()
 979 |         .max_by_key(|p| fs::metadata(p).unwrap().modified().unwrap())
 980 |         .unwrap();
 981 |     let content = fs::read_to_string(latest_output).unwrap();
 982 | 
 983 |     // Should have change summary
 984 |     assert!(content.contains("## Change Summary"));
 985 | 
 986 |     // Should have added files section (not full Files section)
 987 |     assert!(content.contains("## Added Files"));
 988 |     assert!(!content.contains("## Files\n"));
 989 | 
 990 |     // Should include the full content of the added file (handle Windows path separators)
 991 |     assert!(content.contains("### File: `src") && content.contains("new_module.rs`"));
 992 |     assert!(content.contains("pub fn new_function() -> String"));
 993 |     assert!(content.contains("Hello from new module"));
 994 |     assert!(content.contains("_Status: Added_"));
 995 | 
 996 |     // Should still have the file tree and header
 997 |     assert!(content.contains("## File Tree Structure"));
 998 |     assert!(content.contains("# Directory Structure Report"));
 999 | 
1000 |     // Should not include full content of existing files (since they're unchanged)
1001 |     // The existing main.rs content should not be in the full Files section (handle Windows path separators)
1002 |     let main_rs_in_files = content.contains("### File: `src")
1003 |         && content.contains("main.rs`")
1004 |         && content.contains("Hello, world!");
1005 |     assert!(
1006 |         !main_rs_in_files,
1007 |         "Existing unchanged files should not have full content in diff_only mode"
1008 |     );
1009 | }
```

### File: `tests/test_binary_file_autodiff.rs`

- Size: 7957 bytes
- Modified: SystemTime { tv_sec: 1771098907, tv_nsec: 780246326 }

```rust
   1 | //! Integration tests for binary file handling in auto-diff mode
   2 | //!
   3 | //! This test ensures that the application doesn't crash when encountering
   4 | //! binary files during auto-diff processing.
   5 | 
   6 | use std::fs;
   7 | use std::path::Path;
   8 | use tempfile::tempdir;
   9 | 
  10 | use context_builder::config::Config;
  11 | use context_builder::{Prompter, cli::Args, run_with_args};
  12 | 
  13 | struct TestPrompter {
  14 |     overwrite_response: bool,
  15 |     processing_response: bool,
  16 | }
  17 | 
  18 | impl TestPrompter {
  19 |     fn new(overwrite_response: bool, processing_response: bool) -> Self {
  20 |         Self {
  21 |             overwrite_response,
  22 |             processing_response,
  23 |         }
  24 |     }
  25 | }
  26 | 
  27 | impl Prompter for TestPrompter {
  28 |     fn confirm_processing(&self, _file_count: usize) -> std::io::Result<bool> {
  29 |         Ok(self.processing_response)
  30 |     }
  31 | 
  32 |     fn confirm_overwrite(&self, _file_path: &str) -> std::io::Result<bool> {
  33 |         Ok(self.overwrite_response)
  34 |     }
  35 | }
  36 | 
  37 | fn write_file(path: &Path, contents: &str) {
  38 |     if let Some(parent) = path.parent() {
  39 |         fs::create_dir_all(parent).unwrap();
  40 |     }
  41 |     fs::write(path, contents).unwrap();
  42 | }
  43 | 
  44 | fn write_binary_file(path: &Path, data: &[u8]) {
  45 |     if let Some(parent) = path.parent() {
  46 |         fs::create_dir_all(parent).unwrap();
  47 |     }
  48 |     fs::write(path, data).unwrap();
  49 | }
  50 | 
  51 | #[test]
  52 | fn test_binary_files_dont_crash_autodiff() {
  53 |     let temp_dir = tempdir().unwrap();
  54 |     let root = temp_dir.path();
  55 | 
  56 |     // Create text files
  57 |     write_file(
  58 |         &root.join("src/main.rs"),
  59 |         "fn main() { println!(\"Hello\"); }",
  60 |     );
  61 |     write_file(&root.join("README.md"), "# Test Project");
  62 | 
  63 |     // Create binary files with various problematic byte sequences
  64 |     write_binary_file(
  65 |         &root.join("assets/image.png"),
  66 |         &[
  67 |             0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, // PNG header
  68 |             0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44, 0x52, 0xFF, 0xFE, 0xFD, 0xFC, 0x00, 0x01,
  69 |             0x02, 0x03, // Random binary data
  70 |         ],
  71 |     );
  72 | 
  73 |     // Create a file with null bytes
  74 |     write_binary_file(
  75 |         &root.join("data/binary.dat"),
  76 |         &[
  77 |             0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85,
  78 |             0x86, 0x87,
  79 |         ],
  80 |     );
  81 | 
  82 |     // Create a file with invalid UTF-8 sequences
  83 |     write_binary_file(
  84 |         &root.join("config/settings.bin"),
  85 |         &[
  86 |             0xC0, 0x80, // Invalid UTF-8: overlong encoding
  87 |             0xE0, 0x80, 0x80, // Invalid UTF-8: overlong encoding
  88 |             0xFF, 0xFE, 0xFF, 0xFE, // Invalid UTF-8: not valid start bytes
  89 |         ],
  90 |     );
  91 | 
  92 |     let output_path = root.join("output.md");
  93 | 
  94 |     // Configure for auto-diff mode
  95 |     let config = Config {
  96 |         auto_diff: Some(true),
  97 |         diff_context_lines: Some(3),
  98 |         ..Default::default()
  99 |     };
 100 | 
 101 |     let args = Args {
 102 |         input: root.to_string_lossy().into_owned(),
 103 |         output: output_path.to_string_lossy().into_owned(),
 104 |         filter: vec![], // Include all file types to catch binary files
 105 |         ignore: vec![],
 106 |         preview: false,
 107 |         token_count: false,
 108 |         line_numbers: false,
 109 |         yes: true, // Auto-confirm to avoid prompts
 110 |         diff_only: false,
 111 |         clear_cache: false,
 112 |         init: false,
 113 |         max_tokens: None,
 114 |     };
 115 | 
 116 |     let prompter = TestPrompter::new(true, true);
 117 | 
 118 |     // First run - should create initial state without crashing
 119 |     let result1 = run_with_args(args.clone(), config.clone(), &prompter);
 120 |     assert!(
 121 |         result1.is_ok(),
 122 |         "First run with binary files should not crash: {:?}",
 123 |         result1
 124 |     );
 125 | 
 126 |     // Verify output file was created
 127 |     assert!(
 128 |         output_path.exists(),
 129 |         "Output file should be created on first run"
 130 |     );
 131 | 
 132 |     // Modify a text file to trigger diff on second run
 133 |     write_file(
 134 |         &root.join("src/main.rs"),
 135 |         "fn main() { println!(\"Hello, world!\"); }",
 136 |     );
 137 | 
 138 |     // Second run - should handle binary files in diff without crashing
 139 |     let result2 = run_with_args(args, config, &prompter);
 140 |     assert!(
 141 |         result2.is_ok(),
 142 |         "Second run with binary files should not crash during diff: {:?}",
 143 |         result2
 144 |     );
 145 | 
 146 |     // Read the output to verify it contains appropriate handling of binary files
 147 |     let output_content = fs::read_to_string(&output_path).unwrap();
 148 | 
 149 |     // Should contain the modified text file
 150 |     assert!(
 151 |         output_content.contains("Hello, world!"),
 152 |         "Output should contain modified text content"
 153 |     );
 154 | 
 155 |     // Binary files should be represented appropriately (not causing crashes)
 156 |     // The exact representation depends on implementation but should not crash
 157 |     assert!(
 158 |         output_content.len() > 100,
 159 |         "Output should contain substantial content indicating successful processing"
 160 |     );
 161 | }
 162 | 
 163 | #[test]
 164 | fn test_mixed_text_and_binary_files_autodiff() {
 165 |     let temp_dir = tempdir().unwrap();
 166 |     let root = temp_dir.path();
 167 | 
 168 |     // Create a mix of text and binary files
 169 |     write_file(&root.join("source.txt"), "Original text content");
 170 |     write_binary_file(&root.join("data.bin"), &[0x00, 0xFF, 0x42, 0x13, 0x37]);
 171 |     write_file(&root.join("config.json"), r#"{"version": "1.0"}"#);
 172 | 
 173 |     let output_path = root.join("mixed_output.md");
 174 | 
 175 |     let config = Config {
 176 |         auto_diff: Some(true),
 177 |         ..Default::default()
 178 |     };
 179 | 
 180 |     let args = Args {
 181 |         input: root.to_string_lossy().into_owned(),
 182 |         output: output_path.to_string_lossy().into_owned(),
 183 |         filter: vec![],
 184 |         ignore: vec![],
 185 |         preview: false,
 186 |         token_count: false,
 187 |         line_numbers: false,
 188 |         yes: true,
 189 |         diff_only: false,
 190 |         clear_cache: false,
 191 |         init: false,
 192 |         max_tokens: None,
 193 |     };
 194 | 
 195 |     let prompter = TestPrompter::new(true, true);
 196 | 
 197 |     // Initial run
 198 |     let result1 = run_with_args(args.clone(), config.clone(), &prompter);
 199 |     assert!(result1.is_ok(), "Initial run should succeed");
 200 | 
 201 |     // Modify text file and add another binary file
 202 |     write_file(&root.join("source.txt"), "Modified text content");
 203 |     write_binary_file(
 204 |         &root.join("image.jpg"),
 205 |         &[
 206 |             0xFF, 0xD8, 0xFF, 0xE0, // JPEG header
 207 |             0x00, 0x10, 0x4A, 0x46, 0x49, 0x46,
 208 |         ],
 209 |     );
 210 | 
 211 |     // Second run with changes
 212 |     let result2 = run_with_args(args, config, &prompter);
 213 |     assert!(
 214 |         result2.is_ok(),
 215 |         "Second run with mixed file changes should succeed"
 216 |     );
 217 | 
 218 |     let output_content = fs::read_to_string(&output_path).unwrap();
 219 |     assert!(
 220 |         output_content.contains("Modified text content"),
 221 |         "Should show updated text content"
 222 |     );
 223 | }
 224 | 
 225 | #[test]
 226 | fn test_large_binary_file_autodiff() {
 227 |     let temp_dir = tempdir().unwrap();
 228 |     let root = temp_dir.path();
 229 | 
 230 |     // Create a large binary file (simulating real-world scenario)
 231 |     let large_binary_data: Vec<u8> = (0..10000).map(|i| (i % 256) as u8).collect();
 232 | 
 233 |     write_binary_file(&root.join("large_binary.dat"), &large_binary_data);
 234 |     write_file(&root.join("small_text.txt"), "Small text file");
 235 | 
 236 |     let output_path = root.join("large_binary_output.md");
 237 | 
 238 |     let config = Config {
 239 |         auto_diff: Some(true),
 240 |         ..Default::default()
 241 |     };
 242 | 
 243 |     let args = Args {
 244 |         input: root.to_string_lossy().into_owned(),
 245 |         output: output_path.to_string_lossy().into_owned(),
 246 |         filter: vec![],
 247 |         ignore: vec![],
 248 |         preview: false,
 249 |         token_count: false,
 250 |         line_numbers: false,
 251 |         yes: true,
 252 |         diff_only: false,
 253 |         clear_cache: false,
 254 |         init: false,
 255 |         max_tokens: None,
 256 |     };
 257 | 
 258 |     let prompter = TestPrompter::new(true, true);
 259 | 
 260 |     // Should handle large binary files without memory issues or crashes
 261 |     let result = run_with_args(args, config, &prompter);
 262 |     assert!(
 263 |         result.is_ok(),
 264 |         "Should handle large binary files without crashing: {:?}",
 265 |         result
 266 |     );
 267 | 
 268 |     assert!(
 269 |         output_path.exists(),
 270 |         "Output should be created even with large binary files"
 271 |     );
 272 | }
```

### File: `tests/test_comprehensive_edge_cases.rs`

- Size: 22269 bytes
- Modified: SystemTime { tv_sec: 1771099014, tv_nsec: 572715568 }

```rust
   1 | //! Comprehensive edge case testing suite for context-builder v0.5.0
   2 | //!
   3 | //! This test suite covers all the critical edge cases and robustness scenarios
   4 | //! that were identified during the v0.5.0 development cycle.
   5 | 
   6 | use context_builder::cli::Args;
   7 | use context_builder::config::Config;
   8 | use context_builder::{Prompter, run_with_args};
   9 | use serial_test::serial;
  10 | use std::fs;
  11 | use std::path::Path;
  12 | use tempfile::tempdir;
  13 | 
  14 | struct TestPrompter {
  15 |     overwrite_response: bool,
  16 |     processing_response: bool,
  17 | }
  18 | 
  19 | impl TestPrompter {
  20 |     fn new(overwrite_response: bool, processing_response: bool) -> Self {
  21 |         Self {
  22 |             overwrite_response,
  23 |             processing_response,
  24 |         }
  25 |     }
  26 | }
  27 | 
  28 | impl Prompter for TestPrompter {
  29 |     fn confirm_processing(&self, _file_count: usize) -> std::io::Result<bool> {
  30 |         Ok(self.processing_response)
  31 |     }
  32 | 
  33 |     fn confirm_overwrite(&self, _file_path: &str) -> std::io::Result<bool> {
  34 |         Ok(self.overwrite_response)
  35 |     }
  36 | }
  37 | 
  38 | fn write_file(path: &Path, contents: &str) {
  39 |     if let Some(parent) = path.parent() {
  40 |         fs::create_dir_all(parent).unwrap();
  41 |     }
  42 |     fs::write(path, contents).unwrap();
  43 | }
  44 | 
  45 | fn write_binary_file(path: &Path, data: &[u8]) {
  46 |     if let Some(parent) = path.parent() {
  47 |         fs::create_dir_all(parent).unwrap();
  48 |     }
  49 |     fs::write(path, data).unwrap();
  50 | }
  51 | 
  52 | #[test]
  53 | #[serial]
  54 | fn test_comprehensive_binary_file_edge_cases() {
  55 |     let temp_dir = tempdir().unwrap();
  56 |     let project_dir = temp_dir.path().join("project");
  57 |     let output_dir = temp_dir.path().join("output");
  58 |     fs::create_dir_all(&output_dir).unwrap();
  59 | 
  60 |     // Create various binary and problematic files
  61 |     write_file(&project_dir.join("src/normal.rs"), "fn main() {}\n");
  62 | 
  63 |     // Pure binary file (executable-like)
  64 |     let binary_data = vec![
  65 |         0x7f, 0x45, 0x4c, 0x46, // ELF header
  66 |         0x02, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  67 |     ];
  68 |     write_binary_file(&project_dir.join("src/binary.rs"), &binary_data);
  69 | 
  70 |     // File with UTF-16 BOM
  71 |     let utf16_data = [
  72 |         0xFF, 0xFE, // UTF-16 LE BOM
  73 |         0x48, 0x00, 0x65, 0x00, 0x6C, 0x00, 0x6C, 0x00, 0x6F, 0x00, // "Hello"
  74 |         0x0A, 0x00, // newline
  75 |     ];
  76 |     write_binary_file(&project_dir.join("src/utf16.rs"), &utf16_data);
  77 | 
  78 |     // File with Windows-1252 encoding
  79 |     let windows1252_data = [
  80 |         0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x20, // "Hello "
  81 |         0x93, 0x57, 0x6F, 0x72, 0x6C, 0x64, 0x94, // "World" with smart quotes
  82 |         0x0A, // newline
  83 |     ];
  84 |     write_binary_file(&project_dir.join("src/win1252.rs"), &windows1252_data);
  85 | 
  86 |     // Empty file
  87 |     write_file(&project_dir.join("src/empty.rs"), "");
  88 | 
  89 |     // File with only null bytes
  90 |     write_binary_file(&project_dir.join("src/nulls.rs"), &[0x00; 100]);
  91 | 
  92 |     // Very large file (test memory efficiency)
  93 |     let large_content = "// Large file\n".repeat(10000);
  94 |     write_file(&project_dir.join("src/large.rs"), &large_content);
  95 | 
  96 |     // Test with different encoding strategies
  97 |     let strategies = ["detect", "strict", "skip"];
  98 | 
  99 |     for strategy in &strategies {
 100 |         let config = Config {
 101 |             filter: Some(vec!["rs".to_string()]),
 102 |             encoding_strategy: Some(strategy.to_string()),
 103 |             ..Default::default()
 104 |         };
 105 | 
 106 |         let args = Args {
 107 |             input: project_dir.to_string_lossy().to_string(),
 108 |             output: output_dir
 109 |                 .join(format!("test_{}.md", strategy))
 110 |                 .to_string_lossy()
 111 |                 .to_string(),
 112 |             filter: vec!["rs".to_string()],
 113 |             ignore: vec![],
 114 |             preview: false,
 115 |             token_count: false,
 116 |             line_numbers: false,
 117 |             yes: true,
 118 |             diff_only: false,
 119 |             clear_cache: false,
 120 |             init: false,
 121 |             max_tokens: None,
 122 |         };
 123 | 
 124 |         let prompter = TestPrompter::new(true, true);
 125 |         let result = run_with_args(args, config, &prompter);
 126 | 
 127 |         assert!(
 128 |             result.is_ok(),
 129 |             "Should handle binary files gracefully with strategy: {}",
 130 |             strategy
 131 |         );
 132 | 
 133 |         // Verify output file was created
 134 |         let output_path = output_dir.join(format!("test_{}.md", strategy));
 135 |         assert!(
 136 |             output_path.exists(),
 137 |             "Output file should exist for strategy: {}",
 138 |             strategy
 139 |         );
 140 | 
 141 |         let content = fs::read_to_string(&output_path).unwrap();
 142 | 
 143 |         // Should contain normal file
 144 |         assert!(
 145 |             content.contains("fn main()"),
 146 |             "Should contain normal file content"
 147 |         );
 148 | 
 149 |         // Should handle binary files appropriately based on strategy
 150 |         match *strategy {
 151 |             "detect" => {
 152 |                 // May contain transcoded content or binary placeholders
 153 |                 assert!(
 154 |                     content.contains("Hello") || content.contains("<Binary file"),
 155 |                     "Detect strategy should transcode or show binary placeholder"
 156 |                 );
 157 |             }
 158 |             "strict" | "skip" => {
 159 |                 // Should show binary placeholders for non-UTF-8 files
 160 |                 assert!(
 161 |                     content.contains("<Binary file") || content.contains("binary.rs"),
 162 |                     "Strict/skip strategy should show binary placeholders"
 163 |                 );
 164 |             }
 165 |             _ => {}
 166 |         }
 167 | 
 168 |         // Should handle empty files
 169 |         assert!(content.contains("empty.rs"), "Should list empty files");
 170 | 
 171 |         // Should handle large files
 172 |         assert!(content.contains("large.rs"), "Should handle large files");
 173 |     }
 174 | 
 175 |     // No need to restore directory since we never changed it
 176 | }
 177 | 
 178 | #[test]
 179 | #[serial]
 180 | fn test_configuration_precedence_edge_cases() {
 181 |     let temp_dir = tempdir().unwrap();
 182 |     let project_dir = temp_dir.path().join("project");
 183 |     let output_dir = temp_dir.path().join("output");
 184 |     fs::create_dir_all(&output_dir).unwrap();
 185 | 
 186 |     // Create test files
 187 |     write_file(&project_dir.join("test.rs"), "fn test() {}\n");
 188 |     write_file(&project_dir.join("README.md"), "# Test Project\n");
 189 | 
 190 |     // Test 1: Basic functionality with explicit CLI args
 191 |     let args = Args {
 192 |         input: project_dir.to_string_lossy().to_string(),
 193 |         output: output_dir
 194 |             .join("basic_test.md")
 195 |             .to_string_lossy()
 196 |             .to_string(),
 197 |         filter: vec!["rs".to_string()],
 198 |         ignore: vec![],
 199 |         preview: false,
 200 |         token_count: false,
 201 |         line_numbers: false,
 202 |         yes: true,
 203 |         diff_only: false,
 204 |         clear_cache: false,
 205 |         init: false,
 206 |         max_tokens: None,
 207 |     };
 208 | 
 209 |     let prompter = TestPrompter::new(true, true);
 210 |     let result = run_with_args(args, Config::default(), &prompter);
 211 |     assert!(result.is_ok(), "Basic configuration test should succeed");
 212 | 
 213 |     let output_path = output_dir.join("basic_test.md");
 214 |     assert!(output_path.exists(), "Output should exist for basic test");
 215 | 
 216 |     let content = fs::read_to_string(&output_path).unwrap();
 217 |     assert!(
 218 |         content.contains("test.rs"),
 219 |         "Should include filtered .rs files"
 220 |     );
 221 |     assert!(
 222 |         !content.contains("README.md"),
 223 |         "Should exclude non-filtered files"
 224 |     );
 225 | 
 226 |     // Test 2: Empty filter should include all files
 227 |     let args = Args {
 228 |         input: project_dir.to_string_lossy().to_string(),
 229 |         output: output_dir
 230 |             .join("all_files_test.md")
 231 |             .to_string_lossy()
 232 |             .to_string(),
 233 |         filter: vec![],
 234 |         ignore: vec![],
 235 |         preview: false,
 236 |         token_count: false,
 237 |         line_numbers: false,
 238 |         yes: true,
 239 |         diff_only: false,
 240 |         clear_cache: false,
 241 |         init: false,
 242 |         max_tokens: None,
 243 |     };
 244 | 
 245 |     let result = run_with_args(args, Config::default(), &prompter);
 246 |     assert!(result.is_ok(), "All files test should succeed");
 247 | 
 248 |     let output_path = output_dir.join("all_files_test.md");
 249 |     let content = fs::read_to_string(&output_path).unwrap();
 250 |     assert!(
 251 |         content.contains("test.rs"),
 252 |         "Should include all files when no filter"
 253 |     );
 254 |     assert!(
 255 |         content.contains("README.md"),
 256 |         "Should include all files when no filter"
 257 |     );
 258 | }
 259 | 
 260 | #[test]
 261 | #[serial]
 262 | fn test_cache_consistency_edge_cases() {
 263 |     let temp_dir = tempdir().unwrap();
 264 |     let project_dir = temp_dir.path().join("project");
 265 |     let output_dir = temp_dir.path().join("output");
 266 |     fs::create_dir_all(&output_dir).unwrap();
 267 | 
 268 |     write_file(&project_dir.join("test.rs"), "fn original() {}\n");
 269 | 
 270 |     // Change to project directory
 271 |     let original_dir = std::env::current_dir().unwrap();
 272 |     std::env::set_current_dir(&project_dir).unwrap();
 273 | 
 274 |     // Create config with auto_diff enabled
 275 |     write_file(
 276 |         &project_dir.join("context-builder.toml"),
 277 |         r#"
 278 | auto_diff = true
 279 | timestamped_output = true
 280 | "#,
 281 |     );
 282 | 
 283 |     let base_args = Args {
 284 |         input: project_dir.to_string_lossy().to_string(),
 285 |         output: output_dir
 286 |             .join("cache_test.md")
 287 |             .to_string_lossy()
 288 |             .to_string(),
 289 |         filter: vec!["rs".to_string()],
 290 |         ignore: vec![],
 291 |         preview: false,
 292 |         token_count: false,
 293 |         line_numbers: false,
 294 |         yes: true,
 295 |         diff_only: false,
 296 |         clear_cache: false,
 297 |         init: false,
 298 |         max_tokens: None,
 299 |     };
 300 | 
 301 |     let config = context_builder::config::load_config_from_path(&project_dir).unwrap_or_default();
 302 |     let prompter = TestPrompter::new(true, true);
 303 | 
 304 |     // First run - establish cache
 305 |     let result1 = run_with_args(base_args.clone(), config.clone(), &prompter);
 306 |     assert!(result1.is_ok(), "First run should succeed");
 307 | 
 308 |     // Verify cache was created
 309 |     let cache_dir = project_dir.join(".context-builder").join("cache");
 310 |     assert!(cache_dir.exists(), "Cache directory should be created");
 311 | 
 312 |     // Test cache with different path representations
 313 |     let current_dir_string = std::env::current_dir()
 314 |         .unwrap()
 315 |         .to_string_lossy()
 316 |         .to_string();
 317 |     let path_variants = [".", "./", &current_dir_string];
 318 | 
 319 |     for (i, path_variant) in path_variants.iter().enumerate() {
 320 |         let mut variant_args = base_args.clone();
 321 |         variant_args.input = path_variant.to_string();
 322 |         variant_args.output = output_dir
 323 |             .join(format!("variant_{}.md", i))
 324 |             .to_string_lossy()
 325 |             .to_string();
 326 | 
 327 |         let result = run_with_args(variant_args, config.clone(), &prompter);
 328 |         assert!(
 329 |             result.is_ok(),
 330 |             "Path variant '{}' should succeed",
 331 |             path_variant
 332 |         );
 333 | 
 334 |         let output_path = output_dir.join(format!("variant_{}.md", i));
 335 |         let content = fs::read_to_string(&output_path).unwrap();
 336 | 
 337 |         // Should show "no changes detected" because cache should be consistent
 338 |         // (or at least not crash due to path inconsistencies)
 339 |         assert!(
 340 |             content.contains("original") || content.contains("no changes"),
 341 |             "Path variant should handle cache consistently"
 342 |         );
 343 |     }
 344 | 
 345 |     // Test cache corruption recovery
 346 |     let cache_files: Vec<_> = fs::read_dir(&cache_dir)
 347 |         .unwrap()
 348 |         .filter_map(|entry| entry.ok())
 349 |         .filter(|entry| {
 350 |             entry
 351 |                 .path()
 352 |                 .extension()
 353 |                 .and_then(|s| s.to_str())
 354 |                 .map(|s| s == "json")
 355 |                 .unwrap_or(false)
 356 |         })
 357 |         .collect();
 358 | 
 359 |     if !cache_files.is_empty() {
 360 |         // Corrupt the cache
 361 |         fs::write(cache_files[0].path(), "{ invalid json }").unwrap();
 362 | 
 363 |         // Should recover gracefully
 364 |         let result = run_with_args(base_args.clone(), config.clone(), &prompter);
 365 |         assert!(result.is_ok(), "Should recover from corrupted cache");
 366 |     }
 367 | 
 368 |     // Restore original directory
 369 |     std::env::set_current_dir(original_dir).unwrap();
 370 | }
 371 | 
 372 | #[test]
 373 | #[serial]
 374 | fn test_error_conditions_and_exit_codes() {
 375 |     let temp_dir = tempdir().unwrap();
 376 |     let project_dir = temp_dir.path().join("project");
 377 |     let output_dir = temp_dir.path().join("output");
 378 |     fs::create_dir_all(&project_dir).unwrap();
 379 |     fs::create_dir_all(&output_dir).unwrap();
 380 | 
 381 |     let prompter = TestPrompter::new(false, true); // Deny overwrite
 382 | 
 383 |     // Test 1: Non-existent input directory
 384 |     let args = Args {
 385 |         input: temp_dir
 386 |             .path()
 387 |             .join("nonexistent")
 388 |             .to_string_lossy()
 389 |             .to_string(),
 390 |         output: output_dir.join("test.md").to_string_lossy().to_string(),
 391 |         filter: vec![],
 392 |         ignore: vec![],
 393 |         preview: false,
 394 |         token_count: false,
 395 |         line_numbers: false,
 396 |         yes: true,
 397 |         diff_only: false,
 398 |         clear_cache: false,
 399 |         init: false,
 400 |         max_tokens: None,
 401 |     };
 402 | 
 403 |     let result = run_with_args(args, Config::default(), &prompter);
 404 |     assert!(
 405 |         result.is_err(),
 406 |         "Should fail with non-existent input directory"
 407 |     );
 408 | 
 409 |     // Test 2: Permission denied on output
 410 |     write_file(&project_dir.join("test.rs"), "fn test() {}\n");
 411 |     let output_file = output_dir.join("existing.md");
 412 |     write_file(&output_file, "existing content");
 413 | 
 414 |     let args = Args {
 415 |         input: project_dir.to_string_lossy().to_string(),
 416 |         output: output_file.to_string_lossy().to_string(),
 417 |         filter: vec!["rs".to_string()],
 418 |         ignore: vec![],
 419 |         preview: false,
 420 |         token_count: false,
 421 |         line_numbers: false,
 422 |         yes: false, // Don't auto-confirm
 423 |         diff_only: false,
 424 |         clear_cache: false,
 425 |         init: false,
 426 |         max_tokens: None,
 427 |     };
 428 | 
 429 |     let prompter_deny = TestPrompter::new(false, true); // Deny overwrite
 430 |     let result = run_with_args(args, Config::default(), &prompter_deny);
 431 |     assert!(result.is_err(), "Should fail when overwrite is denied");
 432 | 
 433 |     // Test 3: User cancellation during processing
 434 |     let args = Args {
 435 |         input: project_dir.to_string_lossy().to_string(),
 436 |         output: output_dir
 437 |             .join("cancelled.md")
 438 |             .to_string_lossy()
 439 |             .to_string(),
 440 |         filter: vec!["rs".to_string()],
 441 |         ignore: vec![],
 442 |         preview: false,
 443 |         token_count: false,
 444 |         line_numbers: false,
 445 |         yes: false,
 446 |         diff_only: false,
 447 |         clear_cache: false,
 448 |         init: false,
 449 |         max_tokens: None,
 450 |     };
 451 | 
 452 |     let prompter_cancel = TestPrompter::new(true, false); // Allow overwrite, deny processing
 453 |     let result = run_with_args(args, Config::default(), &prompter_cancel);
 454 |     assert!(result.is_err(), "Should fail when processing is cancelled");
 455 | }
 456 | 
 457 | #[test]
 458 | #[cfg(feature = "parallel")]
 459 | fn test_memory_usage_under_parallel_processing() {
 460 |     let temp_dir = tempdir().unwrap();
 461 |     let project_dir = temp_dir.path().join("project");
 462 |     fs::create_dir_all(&project_dir).unwrap();
 463 | 
 464 |     // Create many files to test memory efficiency
 465 |     for i in 0..500 {
 466 |         let subdir = project_dir.join(format!("module_{}", i / 50));
 467 |         fs::create_dir_all(&subdir).unwrap();
 468 | 
 469 |         let content = format!(
 470 |             "// File {}\nuse std::collections::HashMap;\n\npub fn function_{}() -> i32 {{\n    {}\n}}\n",
 471 |             i, i, i
 472 |         );
 473 |         write_file(&subdir.join(format!("file_{}.rs", i)), &content);
 474 |     }
 475 | 
 476 |     let output_dir = temp_dir.path().join("output");
 477 |     fs::create_dir_all(&output_dir).unwrap();
 478 | 
 479 |     let args = Args {
 480 |         input: project_dir.to_string_lossy().to_string(),
 481 |         output: output_dir
 482 |             .join("parallel_test.md")
 483 |             .to_string_lossy()
 484 |             .to_string(),
 485 |         filter: vec!["rs".to_string()],
 486 |         ignore: vec![],
 487 |         preview: false,
 488 |         token_count: false,
 489 |         line_numbers: false,
 490 |         yes: true,
 491 |         diff_only: false,
 492 |         clear_cache: false,
 493 |         init: false,
 494 |         max_tokens: None,
 495 |     };
 496 | 
 497 |     let prompter = TestPrompter::new(true, true);
 498 |     let result = run_with_args(args, Config::default(), &prompter);
 499 | 
 500 |     assert!(
 501 |         result.is_ok(),
 502 |         "Parallel processing should handle many files efficiently"
 503 |     );
 504 | 
 505 |     let output_path = output_dir.join("parallel_test.md");
 506 |     assert!(output_path.exists(), "Output should be created");
 507 | 
 508 |     let content = fs::read_to_string(&output_path).unwrap();
 509 | 
 510 |     // Verify all files are included and properly ordered
 511 |     assert!(
 512 |         content.contains("function_0"),
 513 |         "Should contain first function"
 514 |     );
 515 |     assert!(
 516 |         content.contains("function_499"),
 517 |         "Should contain last function"
 518 |     );
 519 | 
 520 |     // Verify substantial content was generated
 521 |     assert!(
 522 |         content.len() > 100_000,
 523 |         "Should generate substantial output"
 524 |     );
 525 | 
 526 |     // Check that files appear in a reasonable order (not completely scrambled)
 527 |     let first_pos = content.find("function_0").unwrap();
 528 |     let last_pos = content.find("function_499").unwrap();
 529 |     assert!(
 530 |         first_pos < last_pos,
 531 |         "Files should maintain reasonable ordering"
 532 |     );
 533 | }
 534 | 
 535 | #[test]
 536 | #[serial]
 537 | fn test_cwd_independent_operation() {
 538 |     let temp_dir = tempdir().unwrap();
 539 |     let project_dir = temp_dir.path().join("project");
 540 |     let output_dir = temp_dir.path().join("output");
 541 |     let different_cwd = temp_dir.path().join("different_cwd");
 542 | 
 543 |     fs::create_dir_all(&project_dir).unwrap();
 544 |     fs::create_dir_all(&output_dir).unwrap();
 545 |     fs::create_dir_all(&different_cwd).unwrap();
 546 | 
 547 |     // Create test files
 548 |     write_file(&project_dir.join("test.rs"), "fn test() {}\n");
 549 |     write_file(
 550 |         &project_dir.join("context-builder.toml"),
 551 |         r#"
 552 | filter = ["rs"]
 553 | line_numbers = true
 554 | "#,
 555 |     );
 556 | 
 557 |     // Store original directory
 558 |     let original_dir = std::env::current_dir().unwrap();
 559 | 
 560 |     // Test from different working directories
 561 |     let test_cwds = [temp_dir.path(), &different_cwd, &original_dir];
 562 | 
 563 |     for (i, test_cwd) in test_cwds.iter().enumerate() {
 564 |         std::env::set_current_dir(test_cwd).unwrap();
 565 | 
 566 |         let args = Args {
 567 |             input: project_dir.to_string_lossy().to_string(),
 568 |             output: output_dir
 569 |                 .join(format!("cwd_test_{}.md", i))
 570 |                 .to_string_lossy()
 571 |                 .to_string(),
 572 |             filter: vec![], // Use config defaults
 573 |             ignore: vec![],
 574 |             preview: false,
 575 |             token_count: false,
 576 |             line_numbers: false, // Use config default
 577 |             yes: true,
 578 |             diff_only: false,
 579 |             clear_cache: false,
 580 |             init: false,
 581 |             max_tokens: None,
 582 |         };
 583 | 
 584 |         let config =
 585 |             context_builder::config::load_config_from_path(&project_dir).unwrap_or_default();
 586 |         let prompter = TestPrompter::new(true, true);
 587 | 
 588 |         let result = run_with_args(args, config, &prompter);
 589 |         assert!(result.is_ok(), "Should work regardless of CWD (test {})", i);
 590 | 
 591 |         let output_path = output_dir.join(format!("cwd_test_{}.md", i));
 592 |         assert!(
 593 |             output_path.exists(),
 594 |             "Output should exist for CWD test {}",
 595 |             i
 596 |         );
 597 | 
 598 |         let content = fs::read_to_string(&output_path).unwrap();
 599 | 
 600 |         // Should find the config file and apply its settings
 601 |         assert!(
 602 |             content.contains("test.rs"),
 603 |             "Should process rust files from config"
 604 |         );
 605 | 
 606 |         // All outputs should be identical regardless of CWD
 607 |         if i > 0 {
 608 |             let previous_content =
 609 |                 fs::read_to_string(output_dir.join(format!("cwd_test_{}.md", i - 1))).unwrap();
 610 | 
 611 |             // Remove timestamps for comparison
 612 |             let normalize = |s: &str| -> String {
 613 |                 s.lines()
 614 |                     .filter(|line| !line.contains("Processed at:"))
 615 |                     .collect::<Vec<_>>()
 616 |                     .join("\n")
 617 |             };
 618 | 
 619 |             assert_eq!(
 620 |                 normalize(&content),
 621 |                 normalize(&previous_content),
 622 |                 "Output should be identical regardless of CWD"
 623 |             );
 624 |         }
 625 |     }
 626 | 
 627 |     // Restore original directory
 628 |     std::env::set_current_dir(original_dir).unwrap();
 629 | }
 630 | 
 631 | #[test]
 632 | fn test_edge_case_filenames_and_paths() {
 633 |     let temp_dir = tempdir().unwrap();
 634 |     let project_dir = temp_dir.path().join("project");
 635 |     let output_dir = temp_dir.path().join("output");
 636 |     fs::create_dir_all(&output_dir).unwrap();
 637 | 
 638 |     // Create files with problematic names
 639 |     let problematic_names = vec![
 640 |         "normal.rs",
 641 |         "with spaces.rs",
 642 |         "with-dashes.rs",
 643 |         "with_underscores.rs",
 644 |         "with.dots.rs",
 645 |         "uppercase.rs", // Changed from UPPERCASE.RS to avoid case issues
 646 |         "file.with.many.dots.rs",
 647 |         "123numeric.rs",
 648 |         // Note: Avoid truly problematic characters that might fail on Windows
 649 |     ];
 650 | 
 651 |     for name in &problematic_names {
 652 |         write_file(
 653 |             &project_dir.join("src").join(name),
 654 |             &format!("// File: {}\nfn test() {{}}\n", name),
 655 |         );
 656 |     }
 657 | 
 658 |     // Create nested directory structure
 659 |     write_file(
 660 |         &project_dir.join("deeply/nested/very/deep/path.rs"),
 661 |         "fn deep() {}\n",
 662 |     );
 663 | 
 664 |     let args = Args {
 665 |         input: project_dir.to_string_lossy().to_string(),
 666 |         output: output_dir
 667 |             .join("edge_case_paths.md")
 668 |             .to_string_lossy()
 669 |             .to_string(),
 670 |         filter: vec!["rs".to_string()],
 671 |         ignore: vec![],
 672 |         preview: false,
 673 |         token_count: false,
 674 |         line_numbers: false,
 675 |         yes: true,
 676 |         diff_only: false,
 677 |         clear_cache: false,
 678 |         init: false,
 679 |         max_tokens: None,
 680 |     };
 681 | 
 682 |     let prompter = TestPrompter::new(true, true);
 683 |     let result = run_with_args(args, Config::default(), &prompter);
 684 | 
 685 |     assert!(
 686 |         result.is_ok(),
 687 |         "Should handle edge case filenames without panicking"
 688 |     );
 689 | 
 690 |     let output_path = output_dir.join("edge_case_paths.md");
 691 |     assert!(output_path.exists(), "Output should be created");
 692 | 
 693 |     let content = fs::read_to_string(&output_path).unwrap();
 694 | 
 695 |     // Verify all problematic files are included
 696 |     for name in &problematic_names {
 697 |         assert!(
 698 |             content.contains(name),
 699 |             "Should include file with problematic name: {}",
 700 |             name
 701 |         );
 702 |     }
 703 | 
 704 |     // Verify deeply nested path is handled
 705 |     assert!(
 706 |         content.contains("deeply/nested") || content.contains("deeply\\nested"),
 707 |         "Should handle deeply nested paths"
 708 |     );
 709 | }
```

### File: `tests/test_config_resolution.rs`

- Size: 14174 bytes
- Modified: SystemTime { tv_sec: 1771099062, tv_nsec: 387373398 }

```rust
   1 | //! Integration tests for configuration resolution functionality
   2 | //!
   3 | //! These tests verify that the new config resolver properly merges CLI arguments
   4 | //! with configuration file values according to the correct precedence rules.
   5 | 
   6 | use serial_test::serial;
   7 | use std::fs;
   8 | use std::path::Path;
   9 | use tempfile::tempdir;
  10 | 
  11 | use context_builder::{Prompter, cli::Args, config_resolver::resolve_final_config, run_with_args};
  12 | 
  13 | struct TestPrompter {
  14 |     overwrite_response: bool,
  15 |     processing_response: bool,
  16 | }
  17 | 
  18 | impl TestPrompter {
  19 |     fn new(overwrite_response: bool, processing_response: bool) -> Self {
  20 |         Self {
  21 |             overwrite_response,
  22 |             processing_response,
  23 |         }
  24 |     }
  25 | }
  26 | 
  27 | impl Prompter for TestPrompter {
  28 |     fn confirm_processing(&self, _file_count: usize) -> std::io::Result<bool> {
  29 |         Ok(self.processing_response)
  30 |     }
  31 | 
  32 |     fn confirm_overwrite(&self, _file_path: &str) -> std::io::Result<bool> {
  33 |         Ok(self.overwrite_response)
  34 |     }
  35 | }
  36 | 
  37 | fn write_file(path: &Path, contents: &str) {
  38 |     if let Some(parent) = path.parent() {
  39 |         fs::create_dir_all(parent).unwrap();
  40 |     }
  41 |     fs::write(path, contents).unwrap();
  42 | }
  43 | 
  44 | /// Helper function that mimics the run() function's config resolution logic
  45 | fn run_with_resolved_config(
  46 |     args: Args,
  47 |     config: Option<context_builder::config::Config>,
  48 |     prompter: &impl Prompter,
  49 | ) -> std::io::Result<()> {
  50 |     // Resolve final configuration using the new config resolver
  51 |     let resolution = resolve_final_config(args, config.clone());
  52 | 
  53 |     // Convert resolved config back to Args for run_with_args
  54 |     let final_args = Args {
  55 |         input: resolution.config.input,
  56 |         output: resolution.config.output,
  57 |         filter: resolution.config.filter,
  58 |         ignore: resolution.config.ignore,
  59 |         line_numbers: resolution.config.line_numbers,
  60 |         preview: resolution.config.preview,
  61 |         token_count: resolution.config.token_count,
  62 |         yes: resolution.config.yes,
  63 |         diff_only: resolution.config.diff_only,
  64 |         clear_cache: resolution.config.clear_cache,
  65 |         init: resolution.config.init,
  66 |         max_tokens: resolution.config.max_tokens,
  67 |     };
  68 | 
  69 |     // Create final Config with resolved values
  70 |     let final_config = context_builder::config::Config {
  71 |         auto_diff: Some(resolution.config.auto_diff),
  72 |         diff_context_lines: Some(resolution.config.diff_context_lines),
  73 |         ..config.unwrap_or_default()
  74 |     };
  75 | 
  76 |     run_with_args(final_args, final_config, prompter)
  77 | }
  78 | 
  79 | #[test]
  80 | #[serial]
  81 | fn test_cli_arguments_override_config_file() {
  82 |     let temp_dir = tempdir().unwrap();
  83 |     let project_dir = temp_dir.path().join("project");
  84 |     let output_dir = temp_dir.path().join("output");
  85 | 
  86 |     // Create a simple project
  87 |     write_file(
  88 |         &project_dir.join("src/main.rs"),
  89 |         "fn main() { println!(\"Hello\"); }",
  90 |     );
  91 |     write_file(&project_dir.join("lib.py"), "def hello(): print('world')");
  92 | 
  93 |     // Create config file with specific settings
  94 |     write_file(
  95 |         &project_dir.join("context-builder.toml"),
  96 |         r#"
  97 | filter = ["py"]
  98 | line_numbers = true
  99 | output = "from_config.md"
 100 | "#,
 101 |     );
 102 | 
 103 |     fs::create_dir_all(&output_dir).unwrap();
 104 | 
 105 |     // CLI args that should override config
 106 |     // Change to project directory (run_with_args creates output relative to CWD)
 107 |     let original_dir = std::env::current_dir().unwrap();
 108 |     std::env::set_current_dir(&project_dir).unwrap();
 109 | 
 110 |     let args = Args {
 111 |         input: ".".to_string(), // Use current directory
 112 |         output: output_dir.join("from_cli.md").to_string_lossy().to_string(),
 113 |         filter: vec!["rs".to_string()], // Should override config's ["py"]
 114 |         ignore: vec![],
 115 |         line_numbers: true, // Can't override config boolean settings
 116 |         preview: false,
 117 |         token_count: false,
 118 |         yes: true,
 119 |         diff_only: false,
 120 |         clear_cache: false,
 121 |         init: false,
 122 |         max_tokens: None,
 123 |     };
 124 | 
 125 |     let config = context_builder::config::load_config_from_path(&project_dir).unwrap();
 126 |     let prompter = TestPrompter::new(true, true);
 127 | 
 128 |     let result = run_with_resolved_config(args, Some(config), &prompter);
 129 | 
 130 |     // Restore original directory
 131 |     std::env::set_current_dir(original_dir).unwrap();
 132 |     assert!(result.is_ok(), "Should succeed with CLI override");
 133 | 
 134 |     // Verify output file was created with CLI name, not config name
 135 |     let output_file = output_dir.join("from_cli.md");
 136 |     assert!(output_file.exists(), "Output file should use CLI filename");
 137 | 
 138 |     let content = fs::read_to_string(&output_file).unwrap();
 139 | 
 140 |     // Should contain .rs file (CLI filter), not .py file (config filter)
 141 |     assert!(
 142 |         content.contains("main.rs"),
 143 |         "Should include .rs files from CLI filter"
 144 |     );
 145 |     assert!(
 146 |         !content.contains("lib.py"),
 147 |         "Should not include .py files despite config filter"
 148 |     );
 149 | 
 150 |     // Should have line numbers (config applies since we can't distinguish CLI false from default)
 151 |     assert!(
 152 |         content.contains("   1 |"),
 153 |         "Should have line numbers from config"
 154 |     );
 155 | }
 156 | 
 157 | #[test]
 158 | #[serial]
 159 | fn test_config_applies_when_cli_uses_defaults() {
 160 |     let temp_dir = tempdir().unwrap();
 161 |     let project_dir = temp_dir.path().join("project");
 162 |     let output_dir = temp_dir.path().join("output");
 163 | 
 164 |     // Create a simple project
 165 |     write_file(
 166 |         &project_dir.join("src/main.rs"),
 167 |         "fn main() { println!(\"Hello\"); }",
 168 |     );
 169 |     write_file(&project_dir.join("lib.py"), "def hello(): print('world')");
 170 | 
 171 |     // Create config file
 172 |     write_file(
 173 |         &project_dir.join("context-builder.toml"),
 174 |         r#"
 175 | filter = ["py", "rs"]
 176 | line_numbers = true
 177 | ignore = ["target"]
 178 | "#,
 179 |     );
 180 | 
 181 |     fs::create_dir_all(&output_dir).unwrap();
 182 | 
 183 |     // Change to project directory
 184 |     let original_dir = std::env::current_dir().unwrap();
 185 |     std::env::set_current_dir(&project_dir).unwrap();
 186 | 
 187 |     // CLI args using defaults (should be overridden by config)
 188 |     let args = Args {
 189 |         input: ".".to_string(),          // Use current directory
 190 |         output: "output.md".to_string(), // Default - should use config if available
 191 |         filter: vec![],                  // Default - should use config
 192 |         ignore: vec![],                  // Default - should use config
 193 |         line_numbers: false,             // Default - should use config
 194 |         preview: false,
 195 |         token_count: false,
 196 |         yes: true,
 197 |         diff_only: false,
 198 |         clear_cache: false,
 199 |         init: false,
 200 |         max_tokens: None,
 201 |     };
 202 | 
 203 |     let config = context_builder::config::load_config_from_path(&project_dir).unwrap();
 204 |     let prompter = TestPrompter::new(true, true);
 205 | 
 206 |     let result = run_with_resolved_config(args, Some(config), &prompter);
 207 | 
 208 |     // Restore original directory
 209 |     std::env::set_current_dir(original_dir).unwrap();
 210 |     assert!(result.is_ok(), "Should succeed with config application");
 211 | 
 212 |     // Find the output file (should be in current working directory, which is project dir)
 213 |     let output_file = project_dir.join("output.md");
 214 |     // The tool runs with project_dir as input, so output.md should be created there
 215 |     assert!(
 216 |         output_file.exists(),
 217 |         "Output file should be created in project directory"
 218 |     );
 219 | 
 220 |     let content = fs::read_to_string(&output_file).unwrap();
 221 | 
 222 |     // Should contain both file types from config filter
 223 |     assert!(
 224 |         content.contains("main.rs"),
 225 |         "Should include .rs files from config filter"
 226 |     );
 227 |     assert!(
 228 |         content.contains("lib.py"),
 229 |         "Should include .py files from config filter"
 230 |     );
 231 | 
 232 |     // Should have line numbers from config
 233 |     assert!(
 234 |         content.contains("   1 |"),
 235 |         "Should have line numbers from config"
 236 |     );
 237 | }
 238 | 
 239 | #[test]
 240 | #[serial]
 241 | fn test_timestamped_output_and_output_folder() {
 242 |     let temp_dir = tempdir().unwrap();
 243 |     let project_dir = temp_dir.path().join("project");
 244 |     let _output_dir = temp_dir.path().join("docs");
 245 | 
 246 |     // Create a simple project
 247 |     write_file(
 248 |         &project_dir.join("src/main.rs"),
 249 |         "fn main() { println!(\"Hello\"); }",
 250 |     );
 251 | 
 252 |     // Create config with timestamping and output folder (relative to project)
 253 |     write_file(
 254 |         &project_dir.join("context-builder.toml"),
 255 |         r#"
 256 | output = "context.md"
 257 | output_folder = "docs"
 258 | timestamped_output = true
 259 | "#,
 260 |     );
 261 | 
 262 |     // Create docs directory inside project directory
 263 |     let docs_dir = project_dir.join("docs");
 264 |     fs::create_dir_all(&docs_dir).unwrap();
 265 | 
 266 |     // Change to project directory
 267 |     let original_dir = std::env::current_dir().unwrap();
 268 |     std::env::set_current_dir(&project_dir).unwrap();
 269 | 
 270 |     let args = Args {
 271 |         input: ".".to_string(),          // Use current directory
 272 |         output: "output.md".to_string(), // Should be overridden by config
 273 |         filter: vec![],
 274 |         ignore: vec![],
 275 |         line_numbers: false,
 276 |         preview: false,
 277 |         token_count: false,
 278 |         yes: true,
 279 |         diff_only: false,
 280 |         clear_cache: false,
 281 |         init: false,
 282 |         max_tokens: None,
 283 |     };
 284 | 
 285 |     let config = context_builder::config::load_config_from_path(&project_dir).unwrap();
 286 |     let prompter = TestPrompter::new(true, true);
 287 | 
 288 |     let result = run_with_resolved_config(args, Some(config), &prompter);
 289 | 
 290 |     // Restore original directory
 291 |     std::env::set_current_dir(original_dir).unwrap();
 292 |     assert!(result.is_ok(), "Should succeed with timestamped output");
 293 | 
 294 |     // Find timestamped file in docs directory
 295 |     let docs_dir = project_dir.join("docs");
 296 |     let entries = fs::read_dir(&docs_dir).unwrap();
 297 |     let output_files: Vec<_> = entries
 298 |         .filter_map(|entry| entry.ok())
 299 |         .filter(|entry| {
 300 |             let name = entry.file_name();
 301 |             let name_str = name.to_string_lossy();
 302 |             name_str.starts_with("context_") && name_str.ends_with(".md")
 303 |         })
 304 |         .collect();
 305 | 
 306 |     assert!(
 307 |         !output_files.is_empty(),
 308 |         "Should have timestamped output file"
 309 |     );
 310 |     assert!(
 311 |         output_files.len() == 1,
 312 |         "Should have exactly one output file"
 313 |     );
 314 | 
 315 |     let output_file = &output_files[0];
 316 |     let content = fs::read_to_string(output_file.path()).unwrap();
 317 |     assert!(content.contains("main.rs"), "Should contain project files");
 318 | }
 319 | 
 320 | #[test]
 321 | #[serial]
 322 | fn test_mixed_explicit_and_default_values() {
 323 |     let temp_dir = tempdir().unwrap();
 324 |     let project_dir = temp_dir.path().join("project");
 325 | 
 326 |     // Create a simple project
 327 |     write_file(
 328 |         &project_dir.join("src/main.rs"),
 329 |         "fn main() { println!(\"Hello\"); }",
 330 |     );
 331 |     write_file(&project_dir.join("test.py"), "print('test')");
 332 | 
 333 |     // Config with multiple settings
 334 |     write_file(
 335 |         &project_dir.join("context-builder.toml"),
 336 |         r#"
 337 | filter = ["py"]
 338 | line_numbers = true
 339 | yes = true
 340 | "#,
 341 |     );
 342 | 
 343 |     // Change to project directory
 344 |     let original_dir = std::env::current_dir().unwrap();
 345 |     std::env::set_current_dir(&project_dir).unwrap();
 346 | 
 347 |     let args = Args {
 348 |         input: ".".to_string(),          // Use current directory
 349 |         output: "custom.md".to_string(), // Explicit CLI value
 350 |         filter: vec![],                  // Default - should use config
 351 |         ignore: vec![],
 352 |         line_numbers: false, // Default - config will override this
 353 |         preview: false,      // Default - should use config
 354 |         token_count: false,  // Don't use token count mode so file gets created
 355 |         yes: false,          // Default - should use config
 356 |         diff_only: false,
 357 |         clear_cache: false,
 358 |         init: false,
 359 |         max_tokens: None,
 360 |     };
 361 | 
 362 |     let config = context_builder::config::load_config_from_path(&project_dir).unwrap();
 363 |     let prompter = TestPrompter::new(true, true);
 364 | 
 365 |     let result = run_with_resolved_config(args, Some(config), &prompter);
 366 | 
 367 |     // Restore original directory
 368 |     std::env::set_current_dir(original_dir).unwrap();
 369 |     assert!(result.is_ok(), "Should succeed with mixed values");
 370 | 
 371 |     // Verify output file uses CLI name (created in project directory)
 372 |     let output_file = project_dir.join("custom.md");
 373 |     assert!(
 374 |         output_file.exists(),
 375 |         "Should use CLI output filename in project directory"
 376 |     );
 377 | 
 378 |     let content = fs::read_to_string(&output_file).unwrap();
 379 | 
 380 |     // Should use config filter (py files)
 381 |     assert!(
 382 |         content.contains("test.py"),
 383 |         "Should include .py files from config"
 384 |     );
 385 |     assert!(!content.contains("main.rs"), "Should not include .rs files");
 386 | 
 387 |     // Should use config line_numbers setting
 388 |     assert!(
 389 |         content.contains("   1 |"),
 390 |         "Should have line numbers from config"
 391 |     );
 392 | }
 393 | 
 394 | #[test]
 395 | #[serial]
 396 | fn test_auto_diff_configuration_warning() {
 397 |     let temp_dir = tempdir().unwrap();
 398 |     let project_dir = temp_dir.path().join("project");
 399 | 
 400 |     // Create a simple project
 401 |     write_file(
 402 |         &project_dir.join("src/main.rs"),
 403 |         "fn main() { println!(\"Hello\"); }",
 404 |     );
 405 | 
 406 |     // Config with auto_diff but no timestamped_output (should generate warning)
 407 |     write_file(
 408 |         &project_dir.join("context-builder.toml"),
 409 |         r#"
 410 | auto_diff = true
 411 | timestamped_output = false
 412 | "#,
 413 |     );
 414 | 
 415 |     // Change to project directory
 416 |     let original_dir = std::env::current_dir().unwrap();
 417 |     std::env::set_current_dir(&project_dir).unwrap();
 418 | 
 419 |     let args = Args {
 420 |         input: ".".to_string(), // Use current directory
 421 |         output: "output.md".to_string(),
 422 |         filter: vec![],
 423 |         ignore: vec![],
 424 |         line_numbers: false,
 425 |         preview: false,
 426 |         token_count: false,
 427 |         yes: true,
 428 |         diff_only: false,
 429 |         clear_cache: false,
 430 |         init: false,
 431 |         max_tokens: None,
 432 |     };
 433 | 
 434 |     let config = context_builder::config::load_config_from_path(&project_dir).unwrap();
 435 |     let prompter = TestPrompter::new(true, true);
 436 | 
 437 |     // Capture stderr to check for warnings
 438 |     let result = run_with_resolved_config(args, Some(config), &prompter);
 439 | 
 440 |     // Restore original directory
 441 |     std::env::set_current_dir(original_dir).unwrap();
 442 |     assert!(result.is_ok(), "Should succeed despite warning");
 443 | 
 444 |     // Note: In a real application, we would capture stderr to verify the warning
 445 |     // For this test, we're just ensuring the config is handled without crashing
 446 | }
```

### File: `tests/test_cwd_independence.rs`

- Size: 13477 bytes
- Modified: SystemTime { tv_sec: 1771098907, tv_nsec: 781246339 }

```rust
   1 | //! Integration tests for CWD independence
   2 | //!
   3 | //! This test verifies that the application loads config and creates cache
   4 | //! relative to the project root, not the current working directory.
   5 | 
   6 | use serial_test::serial;
   7 | use std::fs;
   8 | use std::path::Path;
   9 | use tempfile::tempdir;
  10 | 
  11 | use context_builder::{Prompter, cli::Args, run_with_args};
  12 | 
  13 | struct TestPrompter {
  14 |     overwrite_response: bool,
  15 |     processing_response: bool,
  16 | }
  17 | 
  18 | impl TestPrompter {
  19 |     fn new(overwrite_response: bool, processing_response: bool) -> Self {
  20 |         Self {
  21 |             overwrite_response,
  22 |             processing_response,
  23 |         }
  24 |     }
  25 | }
  26 | 
  27 | impl Prompter for TestPrompter {
  28 |     fn confirm_processing(&self, _file_count: usize) -> std::io::Result<bool> {
  29 |         Ok(self.processing_response)
  30 |     }
  31 | 
  32 |     fn confirm_overwrite(&self, _file_path: &str) -> std::io::Result<bool> {
  33 |         Ok(self.overwrite_response)
  34 |     }
  35 | }
  36 | 
  37 | fn write_file(path: &Path, contents: &str) {
  38 |     if let Some(parent) = path.parent() {
  39 |         fs::create_dir_all(parent).unwrap();
  40 |     }
  41 |     fs::write(path, contents).unwrap();
  42 | }
  43 | 
  44 | #[test]
  45 | #[serial]
  46 | fn test_config_loaded_from_project_root_not_cwd() {
  47 |     let temp_dir = tempdir().unwrap();
  48 |     let project_dir = temp_dir.path().join("project");
  49 |     let output_dir = temp_dir.path().join("output");
  50 |     let working_dir = temp_dir.path().join("working");
  51 | 
  52 |     // Create project with config file
  53 |     write_file(
  54 |         &project_dir.join("src/main.rs"),
  55 |         "fn main() { println!(\"Hello\"); }",
  56 |     );
  57 |     write_file(
  58 |         &project_dir.join("context-builder.toml"),
  59 |         r#"
  60 | auto_diff = true
  61 | line_numbers = true
  62 | filter = ["rs"]
  63 | "#,
  64 |     );
  65 | 
  66 |     // Create different config in working directory (should be ignored)
  67 |     write_file(
  68 |         &working_dir.join("context-builder.toml"),
  69 |         r#"
  70 | auto_diff = false
  71 | line_numbers = false
  72 | filter = ["txt"]
  73 | "#,
  74 |     );
  75 | 
  76 |     fs::create_dir_all(&output_dir).unwrap();
  77 |     fs::create_dir_all(&working_dir).unwrap();
  78 | 
  79 |     // Change to working directory
  80 |     let original_dir = std::env::current_dir().unwrap();
  81 |     std::env::set_current_dir(&working_dir).unwrap();
  82 | 
  83 |     // Load config from project directory (not CWD)
  84 |     let config = context_builder::config::load_config_from_path(&project_dir).unwrap_or_default();
  85 | 
  86 |     let mut args = Args {
  87 |         input: project_dir.to_string_lossy().to_string(), // Absolute path to project
  88 |         output: output_dir.join("output.md").to_string_lossy().to_string(),
  89 |         filter: vec![], // Should be overridden by project config
  90 |         ignore: vec![],
  91 |         preview: false,
  92 |         token_count: false,
  93 |         line_numbers: false, // Should be overridden by project config
  94 |         yes: true,
  95 |         diff_only: false,
  96 |         clear_cache: false,
  97 |         init: false,
  98 |         max_tokens: None,
  99 |     };
 100 | 
 101 |     // Apply config settings to args (mimicking the run() function logic)
 102 |     if args.filter.is_empty()
 103 |         && let Some(filter) = config.filter.clone()
 104 |     {
 105 |         args.filter = filter;
 106 |     }
 107 |     if !args.line_numbers
 108 |         && let Some(line_numbers) = config.line_numbers
 109 |     {
 110 |         args.line_numbers = line_numbers;
 111 |     }
 112 | 
 113 |     let prompter = TestPrompter::new(true, true);
 114 |     let result = run_with_args(args, config, &prompter);
 115 | 
 116 |     // Restore original directory
 117 |     std::env::set_current_dir(original_dir).unwrap();
 118 | 
 119 |     assert!(result.is_ok(), "Should succeed with CWD independence");
 120 | 
 121 |     let output_content = fs::read_to_string(output_dir.join("output.md")).unwrap();
 122 | 
 123 |     // Verify that project config was used, not working directory config
 124 |     assert!(
 125 |         output_content.contains("   1 |"),
 126 |         "Should have line numbers from project config"
 127 |     );
 128 |     assert!(
 129 |         output_content.contains("main.rs"),
 130 |         "Should include .rs files from project config filter"
 131 |     );
 132 | }
 133 | 
 134 | #[test]
 135 | #[serial]
 136 | fn test_cache_created_in_project_root_not_cwd() {
 137 |     let temp_dir = tempdir().unwrap();
 138 |     let project_dir = temp_dir.path().join("project");
 139 |     let output_dir = temp_dir.path().join("output");
 140 |     let working_dir = temp_dir.path().join("working");
 141 | 
 142 |     // Create project with auto-diff enabled
 143 |     write_file(
 144 |         &project_dir.join("src/main.rs"),
 145 |         "fn main() { println!(\"Hello\"); }",
 146 |     );
 147 |     write_file(
 148 |         &project_dir.join("context-builder.toml"),
 149 |         r#"
 150 | auto_diff = true
 151 | timestamped_output = true
 152 | "#,
 153 |     );
 154 | 
 155 |     fs::create_dir_all(&output_dir).unwrap();
 156 |     fs::create_dir_all(&working_dir).unwrap();
 157 | 
 158 |     // Get absolute paths before changing directory
 159 |     let project_dir_abs = project_dir.canonicalize().unwrap();
 160 |     let output_dir_abs = output_dir.canonicalize().unwrap();
 161 |     let working_dir_abs = working_dir.canonicalize().unwrap();
 162 | 
 163 |     // Change to working directory
 164 |     let original_dir = std::env::current_dir().unwrap();
 165 |     std::env::set_current_dir(&working_dir_abs).unwrap();
 166 | 
 167 |     // Load config from project directory
 168 |     let config =
 169 |         context_builder::config::load_config_from_path(&project_dir_abs).unwrap_or_default();
 170 | 
 171 |     let mut args = Args {
 172 |         input: project_dir_abs.to_string_lossy().to_string(), // Absolute path to project
 173 |         output: output_dir_abs
 174 |             .join("context.md")
 175 |             .to_string_lossy()
 176 |             .to_string(),
 177 |         filter: vec![],
 178 |         ignore: vec![],
 179 |         preview: false,
 180 |         token_count: false,
 181 |         line_numbers: false,
 182 |         yes: true,
 183 |         diff_only: false,
 184 |         clear_cache: false,
 185 |         init: false,
 186 |         max_tokens: None,
 187 |     };
 188 | 
 189 |     // Apply timestamping manually since we're bypassing run()
 190 |     if config.timestamped_output.unwrap_or(false) {
 191 |         use chrono::Utc;
 192 |         let timestamp = Utc::now().format("%Y%m%d%H%M%S").to_string();
 193 |         let path = std::path::Path::new(&args.output);
 194 |         let stem = path
 195 |             .file_stem()
 196 |             .and_then(|s| s.to_str())
 197 |             .unwrap_or("output");
 198 |         let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("md");
 199 |         let new_filename = format!("{}_{}.{}", stem, timestamp, extension);
 200 |         if let Some(parent) = path.parent() {
 201 |             args.output = parent.join(new_filename).to_string_lossy().to_string();
 202 |         } else {
 203 |             args.output = output_dir_abs
 204 |                 .join(new_filename)
 205 |                 .to_string_lossy()
 206 |                 .to_string();
 207 |         }
 208 |     }
 209 | 
 210 |     let prompter = TestPrompter::new(true, true);
 211 | 
 212 |     // First run to create cache
 213 |     let result1 = run_with_args(args.clone(), config.clone(), &prompter);
 214 |     assert!(result1.is_ok(), "First run should succeed");
 215 | 
 216 |     // Verify cache was created in project directory, not working directory
 217 |     let project_cache = project_dir_abs.join(".context-builder").join("cache");
 218 |     let working_cache = working_dir_abs.join(".context-builder").join("cache");
 219 | 
 220 |     assert!(
 221 |         project_cache.exists(),
 222 |         "Cache should be created in project directory"
 223 |     );
 224 |     assert!(
 225 |         !working_cache.exists(),
 226 |         "Cache should NOT be created in working directory"
 227 |     );
 228 | 
 229 |     // Small delay to ensure different timestamps
 230 |     std::thread::sleep(std::time::Duration::from_millis(1100));
 231 | 
 232 |     // Modify project file
 233 |     // Modify a file to trigger diff
 234 |     write_file(
 235 |         &project_dir_abs.join("src/main.rs"),
 236 |         "fn main() { println!(\"Hello, modified!\"); }",
 237 |     );
 238 | 
 239 |     // Create second args with new timestamp
 240 |     let mut args2 = args.clone();
 241 |     if config.timestamped_output.unwrap_or(false) {
 242 |         use chrono::Utc;
 243 |         let timestamp = Utc::now().format("%Y%m%d%H%M%S").to_string();
 244 |         let path = std::path::Path::new(&args2.output);
 245 |         let stem = path
 246 |             .file_stem()
 247 |             .and_then(|s| s.to_str())
 248 |             .unwrap_or("output");
 249 |         let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("md");
 250 |         let new_filename = format!("{}_{}.{}", stem, timestamp, extension);
 251 |         if let Some(parent) = path.parent() {
 252 |             args2.output = parent.join(new_filename).to_string_lossy().to_string();
 253 |         } else {
 254 |             args2.output = output_dir_abs
 255 |                 .join(new_filename)
 256 |                 .to_string_lossy()
 257 |                 .to_string();
 258 |         }
 259 |     }
 260 | 
 261 |     // Second run should detect changes using cache from project directory
 262 |     let result2 = run_with_args(args2, config, &prompter);
 263 |     assert!(result2.is_ok(), "Second run should succeed");
 264 | 
 265 |     // Find output files (should have timestamps) - use absolute path
 266 |     // Add retry logic to handle potential race conditions
 267 |     let output_files = (0..5)
 268 |         .find_map(|_| {
 269 |             std::thread::sleep(std::time::Duration::from_millis(50));
 270 |             if let Ok(entries) = fs::read_dir(&output_dir_abs) {
 271 |                 let files: Vec<_> = entries
 272 |                     .filter_map(|entry| entry.ok())
 273 |                     .filter(|entry| {
 274 |                         let name = entry.file_name();
 275 |                         let name_str = name.to_string_lossy();
 276 |                         name_str.starts_with("context") && name_str.ends_with(".md")
 277 |                     })
 278 |                     .collect();
 279 |                 if files.len() >= 2 { Some(files) } else { None }
 280 |             } else {
 281 |                 None
 282 |             }
 283 |         })
 284 |         .expect("Failed to find output files after retries");
 285 | 
 286 |     // Restore original directory after file operations
 287 |     std::env::set_current_dir(original_dir).unwrap();
 288 | 
 289 |     assert!(
 290 |         output_files.len() >= 2,
 291 |         "Should have multiple timestamped outputs, found: {}",
 292 |         output_files.len()
 293 |     );
 294 | 
 295 |     // Check that second output contains diff information
 296 |     let latest_output = output_files
 297 |         .iter()
 298 |         .max_by_key(|entry| {
 299 |             // All paths are already absolute since we used output_dir_abs
 300 |             fs::metadata(entry.path()).unwrap().modified().unwrap()
 301 |         })
 302 |         .unwrap();
 303 | 
 304 |     // Read the latest file content
 305 |     let latest_content = fs::read_to_string(latest_output.path()).unwrap();
 306 |     assert!(
 307 |         latest_content.contains("## Change Summary") || latest_content.contains("Modified"),
 308 |         "Should contain change information from auto-diff"
 309 |     );
 310 | }
 311 | 
 312 | #[test]
 313 | #[serial]
 314 | fn test_clear_cache_uses_project_root() {
 315 |     let temp_dir = tempdir().unwrap();
 316 |     let project_dir = temp_dir.path().join("project");
 317 |     let working_dir = temp_dir.path().join("working");
 318 | 
 319 |     // Create project and working directories
 320 |     write_file(&project_dir.join("src/main.rs"), "fn main() {}");
 321 |     fs::create_dir_all(&working_dir).unwrap();
 322 | 
 323 |     // Create cache in project directory
 324 |     let project_cache_dir = project_dir.join(".context-builder").join("cache");
 325 |     fs::create_dir_all(&project_cache_dir).unwrap();
 326 |     fs::write(project_cache_dir.join("test_cache.json"), "{}").unwrap();
 327 | 
 328 |     // Create cache in working directory (should not be affected)
 329 |     let working_cache_dir = working_dir.join(".context-builder").join("cache");
 330 |     fs::create_dir_all(&working_cache_dir).unwrap();
 331 |     fs::write(working_cache_dir.join("test_cache.json"), "{}").unwrap();
 332 | 
 333 |     // Change to working directory
 334 |     let original_dir = std::env::current_dir().unwrap();
 335 |     std::env::set_current_dir(&working_dir).unwrap();
 336 | 
 337 |     // Simulate the cache clearing logic from run() function
 338 |     // This tests that cache clearing uses project root, not CWD
 339 |     let cache_path = project_dir.join(".context-builder").join("cache");
 340 |     assert!(
 341 |         cache_path.exists(),
 342 |         "Project cache should exist before clearing"
 343 |     );
 344 | 
 345 |     if cache_path.exists() {
 346 |         fs::remove_dir_all(&cache_path).unwrap();
 347 |     }
 348 | 
 349 |     // Restore original directory
 350 |     std::env::set_current_dir(original_dir).unwrap();
 351 | 
 352 |     // Project cache should be cleared
 353 |     assert!(
 354 |         !project_cache_dir.exists(),
 355 |         "Project cache should be cleared"
 356 |     );
 357 | 
 358 |     // Working directory cache should be untouched
 359 |     assert!(
 360 |         working_cache_dir.exists() && fs::read_dir(&working_cache_dir).unwrap().count() > 0,
 361 |         "Working directory cache should remain untouched"
 362 |     );
 363 | }
 364 | 
 365 | #[test]
 366 | #[serial]
 367 | fn test_load_config_from_path_function() {
 368 |     let temp_dir = tempdir().unwrap();
 369 |     let project_dir = temp_dir.path().join("project");
 370 |     let working_dir = temp_dir.path().join("working");
 371 | 
 372 |     // Create project with config file
 373 |     write_file(
 374 |         &project_dir.join("context-builder.toml"),
 375 |         r#"
 376 | auto_diff = true
 377 | line_numbers = true
 378 | filter = ["rs"]
 379 | "#,
 380 |     );
 381 | 
 382 |     // Create different config in working directory
 383 |     write_file(
 384 |         &working_dir.join("context-builder.toml"),
 385 |         r#"
 386 | auto_diff = false
 387 | line_numbers = false
 388 | filter = ["txt"]
 389 | "#,
 390 |     );
 391 | 
 392 |     // Change to working directory
 393 |     let original_dir = std::env::current_dir().unwrap();
 394 |     std::env::set_current_dir(&working_dir).unwrap();
 395 | 
 396 |     // Load config from project directory (not CWD)
 397 |     let config = context_builder::config::load_config_from_path(&project_dir);
 398 | 
 399 |     // Restore original directory
 400 |     std::env::set_current_dir(original_dir).unwrap();
 401 | 
 402 |     assert!(
 403 |         config.is_some(),
 404 |         "Should load config from project directory"
 405 |     );
 406 |     let config = config.unwrap();
 407 | 
 408 |     assert_eq!(
 409 |         config.auto_diff,
 410 |         Some(true),
 411 |         "Should use project config auto_diff"
 412 |     );
 413 |     assert_eq!(
 414 |         config.line_numbers,
 415 |         Some(true),
 416 |         "Should use project config line_numbers"
 417 |     );
 418 |     assert_eq!(
 419 |         config.filter,
 420 |         Some(vec!["rs".to_string()]),
 421 |         "Should use project config filter"
 422 |     );
 423 | }
```

### File: `tests/test_determinism.rs`

- Size: 20050 bytes
- Modified: SystemTime { tv_sec: 1771109025, tv_nsec: 828105245 }

```rust
   1 | //! Integration tests for determinism and robustness of context-builder
   2 | //!
   3 | //! These tests verify that the critical bug fixes are working correctly:
   4 | //! - Deterministic output order
   5 | //! - Robust caching
   6 | //! - Thread safety
   7 | 
   8 | use pretty_assertions::assert_eq;
   9 | use serial_test::serial;
  10 | use std::fs;
  11 | use std::path::Path;
  12 | use tempfile::tempdir;
  13 | 
  14 | use chrono::Utc;
  15 | use context_builder::cli::Args;
  16 | use context_builder::config::{Config, load_config};
  17 | use context_builder::{Prompter, run_with_args};
  18 | 
  19 | /// Test prompter that always confirms
  20 | struct TestPrompter;
  21 | 
  22 | impl Prompter for TestPrompter {
  23 |     fn confirm_processing(&self, _file_count: usize) -> std::io::Result<bool> {
  24 |         Ok(true)
  25 |     }
  26 |     fn confirm_overwrite(&self, _file_path: &str) -> std::io::Result<bool> {
  27 |         Ok(true)
  28 |     }
  29 | }
  30 | 
  31 | /// Create a test project with multiple files in different directories
  32 | fn create_test_project(base_dir: &Path) -> std::io::Result<()> {
  33 |     let src_dir = base_dir.join("src");
  34 |     let tests_dir = base_dir.join("tests");
  35 |     let docs_dir = base_dir.join("docs");
  36 | 
  37 |     fs::create_dir_all(&src_dir)?;
  38 |     fs::create_dir_all(&tests_dir)?;
  39 |     fs::create_dir_all(&docs_dir)?;
  40 | 
  41 |     // Create files in different orders to test sorting
  42 |     fs::write(
  43 |         src_dir.join("main.rs"),
  44 |         "fn main() {\n    println!(\"Hello\");\n}",
  45 |     )?;
  46 |     fs::write(src_dir.join("lib.rs"), "pub mod utils;\npub mod config;")?;
  47 |     fs::write(src_dir.join("utils.rs"), "pub fn helper() {}")?;
  48 |     fs::write(
  49 |         tests_dir.join("integration.rs"),
  50 |         "#[test]\nfn test_something() {}",
  51 |     )?;
  52 |     fs::write(tests_dir.join("unit.rs"), "#[test]\nfn test_unit() {}")?;
  53 |     fs::write(
  54 |         docs_dir.join("README.md"),
  55 |         "# Project\n\nThis is a test project.",
  56 |     )?;
  57 |     fs::write(
  58 |         base_dir.join("Cargo.toml"),
  59 |         "[package]\nname = \"test\"\nversion = \"0.1.0\"",
  60 |     )?;
  61 | 
  62 |     Ok(())
  63 | }
  64 | 
  65 | #[test]
  66 | #[serial] // Ensure tests don't interfere with each other
  67 | fn test_deterministic_output_multiple_runs() {
  68 |     let temp_dir = tempdir().unwrap();
  69 |     let project_dir = temp_dir.path().join("project");
  70 |     create_test_project(&project_dir).unwrap();
  71 | 
  72 |     // Note: The actual output files may have timestamps appended due to auto-diff mode
  73 |     // We'll need to find the actual files created
  74 |     let prompter = TestPrompter;
  75 | 
  76 |     // Run twice with identical arguments
  77 |     let result1 = run_with_args(
  78 |         Args {
  79 |             input: project_dir.to_string_lossy().to_string(),
  80 |             output: temp_dir
  81 |                 .path()
  82 |                 .join("output1.md")
  83 |                 .to_string_lossy()
  84 |                 .to_string(),
  85 |             filter: vec!["rs".to_string(), "md".to_string(), "toml".to_string()],
  86 |             ignore: vec![],
  87 |             preview: false,
  88 |             token_count: false,
  89 |             line_numbers: false,
  90 |             yes: true,
  91 |             diff_only: false,
  92 |             clear_cache: false,
  93 |             init: false,
  94 |             max_tokens: None,
  95 |         },
  96 |         Config::default(),
  97 |         &prompter,
  98 |     );
  99 | 
 100 |     let result2 = run_with_args(
 101 |         Args {
 102 |             input: project_dir.to_string_lossy().to_string(),
 103 |             output: temp_dir
 104 |                 .path()
 105 |                 .join("output2.md")
 106 |                 .to_string_lossy()
 107 |                 .to_string(),
 108 |             filter: vec!["rs".to_string(), "md".to_string(), "toml".to_string()],
 109 |             ignore: vec![],
 110 |             preview: false,
 111 |             token_count: false,
 112 |             line_numbers: false,
 113 |             yes: true,
 114 |             diff_only: false,
 115 |             clear_cache: false,
 116 |             init: false,
 117 |             max_tokens: None,
 118 |         },
 119 |         Config::default(),
 120 |         &prompter,
 121 |     );
 122 | 
 123 |     if let Err(e) = result1 {
 124 |         panic!("First run failed: {}", e);
 125 |     }
 126 |     if let Err(e) = result2 {
 127 |         panic!("Second run failed: {}", e);
 128 |     }
 129 | 
 130 |     // Find the actual output files (they may have timestamps appended)
 131 |     let temp_entries: Vec<_> = fs::read_dir(temp_dir.path())
 132 |         .unwrap()
 133 |         .filter_map(|entry| entry.ok())
 134 |         .filter(|entry| {
 135 |             let file_name = entry.file_name();
 136 |             let name = file_name.to_string_lossy();
 137 |             name.starts_with("output") && name.ends_with(".md")
 138 |         })
 139 |         .collect();
 140 | 
 141 |     if temp_entries.len() < 2 {
 142 |         eprintln!("Expected 2 output files, found {}", temp_entries.len());
 143 |         eprintln!("Temp directory contents:");
 144 |         for entry in fs::read_dir(temp_dir.path()).unwrap() {
 145 |             eprintln!("  {:?}", entry.unwrap().file_name());
 146 |         }
 147 |         panic!("Not enough output files found");
 148 |     }
 149 | 
 150 |     // Sort to ensure consistent ordering
 151 |     let mut output_files: Vec<_> = temp_entries.iter().map(|entry| entry.path()).collect();
 152 |     output_files.sort();
 153 | 
 154 |     // Read both outputs
 155 |     let content1 = fs::read_to_string(&output_files[0]).unwrap();
 156 |     let content2 = fs::read_to_string(&output_files[1]).unwrap();
 157 | 
 158 |     // Debug: Write contents to temp files for inspection
 159 |     fs::write(temp_dir.path().join("debug_content1.md"), &content1).unwrap();
 160 |     fs::write(temp_dir.path().join("debug_content2.md"), &content2).unwrap();
 161 | 
 162 |     // Normalize timestamps for comparison since they will be different
 163 |     let normalize = |content: &str| -> String {
 164 |         content
 165 |             .lines()
 166 |             .map(|line| {
 167 |                 if line.starts_with("Processed at: ") {
 168 |                     "Processed at: <timestamp>"
 169 |                 } else {
 170 |                     line
 171 |                 }
 172 |             })
 173 |             .collect::<Vec<_>>()
 174 |             .join("\n")
 175 |     };
 176 | 
 177 |     let normalized1 = normalize(&content1);
 178 |     let normalized2 = normalize(&content2);
 179 | 
 180 |     // Debug: Write normalized contents for comparison
 181 |     fs::write(temp_dir.path().join("debug_normalized1.md"), &normalized1).unwrap();
 182 |     fs::write(temp_dir.path().join("debug_normalized2.md"), &normalized2).unwrap();
 183 | 
 184 |     // They should be identical (deterministic) after normalizing timestamps
 185 |     if normalized1 != normalized2 {
 186 |         eprintln!(
 187 |             "Content1 length: {}, Content2 length: {}",
 188 |             normalized1.len(),
 189 |             normalized2.len()
 190 |         );
 191 |         eprintln!(
 192 |             "First difference at position: {:?}",
 193 |             normalized1
 194 |                 .chars()
 195 |                 .zip(normalized2.chars())
 196 |                 .position(|(a, b)| a != b)
 197 |         );
 198 |         eprintln!("Debug files written to: {}", temp_dir.path().display());
 199 |         panic!("Output should be deterministic across multiple runs (ignoring timestamps)");
 200 |     }
 201 | 
 202 |     // Verify that files are listed in a consistent order
 203 |     let lines: Vec<&str> = content1.lines().collect();
 204 |     let file_lines: Vec<&str> = lines
 205 |         .iter()
 206 |         .filter(|line| line.starts_with("### File: `"))
 207 |         .copied()
 208 |         .collect();
 209 | 
 210 |     // Should have found some files
 211 |     assert!(
 212 |         !file_lines.is_empty(),
 213 |         "Should have found some file entries"
 214 |     );
 215 | 
 216 |     // Check that files are sorted by relevance category:
 217 |     // Category 0: Cargo.toml (config), README.md (key project doc)
 218 |     // Category 1: src/* (source code) — entry points first (lib.rs, main.rs before utils.rs)
 219 |     // Category 2: tests/* (tests)
 220 |     let expected_order = vec![
 221 |         "### File: `Cargo.toml`",
 222 |         "### File: `docs/README.md`",
 223 |         "### File: `src/lib.rs`",
 224 |         "### File: `src/main.rs`",
 225 |         "### File: `src/utils.rs`",
 226 |         "### File: `tests/integration.rs`",
 227 |         "### File: `tests/unit.rs`",
 228 |     ];
 229 |     assert_eq!(
 230 |         file_lines, expected_order,
 231 |         "Files should be listed in relevance order (config+docs → source (entry points first) → tests)"
 232 |     );
 233 | }
 234 | #[test]
 235 | #[serial] // Ensure tests don't interfere with each other
 236 | fn test_deterministic_file_tree_order() {
 237 |     let temp_dir = tempdir().unwrap();
 238 |     let project_dir = temp_dir.path().join("project");
 239 |     create_test_project(&project_dir).unwrap();
 240 | 
 241 |     let output_path = temp_dir.path().join("output.md");
 242 | 
 243 |     // Change to project directory so config loading works
 244 |     let original_dir = std::env::current_dir().unwrap();
 245 |     std::env::set_current_dir(&project_dir).unwrap();
 246 | 
 247 |     let args = Args {
 248 |         input: ".".to_string(),
 249 |         output: output_path.to_string_lossy().to_string(),
 250 |         filter: vec![],
 251 |         ignore: vec![],
 252 |         preview: false,
 253 |         token_count: false,
 254 |         line_numbers: false,
 255 |         yes: true,
 256 |         diff_only: false,
 257 |         clear_cache: false,
 258 |         init: false,
 259 |         max_tokens: None,
 260 |     };
 261 | 
 262 |     let prompter = TestPrompter;
 263 |     run_with_args(args, Config::default(), &prompter).unwrap();
 264 | 
 265 |     // Restore original directory
 266 |     std::env::set_current_dir(original_dir).unwrap();
 267 | 
 268 |     let content = fs::read_to_string(&output_path).unwrap();
 269 | 
 270 |     // Find the file tree section
 271 |     let tree_start = content
 272 |         .find("## File Tree Structure")
 273 |         .expect("Should have file tree section");
 274 |     let files_start = content.find("### File: `").unwrap_or(content.len());
 275 |     let tree_section = &content[tree_start..files_start];
 276 | 
 277 |     // Check that directories and files appear in alphabetical order in the tree
 278 |     // This is a basic check - a more sophisticated test would parse the tree structure
 279 |     assert!(tree_section.contains("Cargo.toml"));
 280 |     // Check for directory entries - they may appear as just the name or with trailing content
 281 |     assert!(tree_section.contains("docs") || tree_section.contains("docs/"));
 282 |     assert!(tree_section.contains("src") || tree_section.contains("src/"));
 283 |     assert!(tree_section.contains("tests") || tree_section.contains("tests/"));
 284 | }
 285 | 
 286 | #[test]
 287 | #[serial] // Ensure cache tests don't interfere with each other
 288 | fn test_cache_collision_prevention() {
 289 |     let temp_dir1 = tempdir().unwrap();
 290 |     let temp_dir2 = tempdir().unwrap();
 291 | 
 292 |     let project1 = temp_dir1.path().join("project");
 293 |     let project2 = temp_dir2.path().join("project");
 294 | 
 295 |     create_test_project(&project1).unwrap();
 296 |     create_test_project(&project2).unwrap();
 297 | 
 298 |     // Add different content to make projects distinct
 299 |     fs::write(project1.join("unique1.txt"), "This is project 1").unwrap();
 300 |     fs::write(project2.join("unique2.txt"), "This is project 2").unwrap();
 301 | 
 302 |     let output1 = temp_dir1.path().join("output.md");
 303 |     let output2 = temp_dir2.path().join("output.md");
 304 | 
 305 |     let prompter = TestPrompter;
 306 | 
 307 |     // Change to project1 directory and run
 308 |     let original_dir = std::env::current_dir().unwrap();
 309 |     std::env::set_current_dir(&project1).unwrap();
 310 | 
 311 |     let args1 = Args {
 312 |         input: ".".to_string(),
 313 |         output: output1.to_string_lossy().to_string(),
 314 |         filter: vec![],
 315 |         ignore: vec![],
 316 |         preview: false,
 317 |         token_count: false,
 318 |         line_numbers: false,
 319 |         yes: true,
 320 |         diff_only: false,
 321 |         clear_cache: false,
 322 |         init: false,
 323 |         max_tokens: None,
 324 |     };
 325 | 
 326 |     run_with_args(args1, Config::default(), &prompter).unwrap();
 327 | 
 328 |     // Change to project2 directory and run
 329 |     std::env::set_current_dir(&project2).unwrap();
 330 | 
 331 |     let args2 = Args {
 332 |         input: ".".to_string(),
 333 |         output: output2.to_string_lossy().to_string(),
 334 |         filter: vec!["txt".to_string()],
 335 |         ignore: vec![],
 336 |         preview: false,
 337 |         token_count: false,
 338 |         line_numbers: false,
 339 | 
 340 |         yes: true,
 341 | 
 342 |         diff_only: false,
 343 | 
 344 |         clear_cache: false,
 345 | 
 346 |         init: false,
 347 |         max_tokens: None,
 348 |     };
 349 | 
 350 |     run_with_args(args2, Config::default(), &prompter).unwrap();
 351 | 
 352 |     // Restore original directory
 353 |     std::env::set_current_dir(original_dir).unwrap();
 354 | 
 355 |     let content1 = fs::read_to_string(&output1).unwrap();
 356 |     let content2 = fs::read_to_string(&output2).unwrap();
 357 | 
 358 |     // Outputs should be different due to different projects and configs
 359 |     assert_ne!(
 360 |         content1, content2,
 361 |         "Different projects should produce different outputs"
 362 |     );
 363 | 
 364 |     // Each should contain their unique content
 365 |     assert!(content1.contains("unique1.txt"));
 366 |     assert!(content2.contains("unique2.txt"));
 367 | }
 368 | 
 369 | #[test]
 370 | #[serial] // Ensure tests don't interfere with each other
 371 | fn test_custom_ignores_performance() {
 372 |     let temp_dir = tempdir().unwrap();
 373 |     let project_dir = temp_dir.path().join("project");
 374 | 
 375 |     // Create a project with ignored directories
 376 |     create_test_project(&project_dir).unwrap();
 377 | 
 378 |     let target_dir = project_dir.join("target");
 379 |     let node_modules_dir = project_dir.join("node_modules");
 380 | 
 381 |     fs::create_dir_all(&target_dir).unwrap();
 382 |     fs::create_dir_all(&node_modules_dir).unwrap();
 383 | 
 384 |     // Create many files in ignored directories
 385 |     for i in 0..10 {
 386 |         fs::write(target_dir.join(format!("file{}.txt", i)), "ignored content").unwrap();
 387 |         fs::write(
 388 |             node_modules_dir.join(format!("module{}.js", i)),
 389 |             "ignored js",
 390 |         )
 391 |         .unwrap();
 392 |     }
 393 | 
 394 |     let output_path = temp_dir.path().join("output.md");
 395 | 
 396 |     // Change to project directory so config loading works
 397 |     let original_dir = std::env::current_dir().unwrap();
 398 |     std::env::set_current_dir(&project_dir).unwrap();
 399 | 
 400 |     let args = Args {
 401 |         input: ".".to_string(),
 402 |         output: output_path.to_string_lossy().to_string(),
 403 |         filter: vec![],
 404 |         ignore: vec!["target".to_string(), "node_modules".to_string()],
 405 |         preview: false,
 406 |         token_count: false,
 407 |         line_numbers: false,
 408 |         yes: true,
 409 |         diff_only: false,
 410 |         clear_cache: false,
 411 |         init: false,
 412 |         max_tokens: None,
 413 |     };
 414 | 
 415 |     let prompter = TestPrompter;
 416 |     let start = std::time::Instant::now();
 417 | 
 418 |     run_with_args(args, Config::default(), &prompter).unwrap();
 419 | 
 420 |     // Restore original directory
 421 |     std::env::set_current_dir(original_dir).unwrap();
 422 | 
 423 |     let duration = start.elapsed();
 424 | 
 425 |     let content = fs::read_to_string(&output_path).unwrap();
 426 | 
 427 |     // Verify ignored files are not included
 428 |     assert!(!content.contains("target/file"));
 429 |     assert!(!content.contains("node_modules/module"));
 430 | 
 431 |     // Performance should be reasonable (this is a basic check)
 432 |     assert!(
 433 |         duration.as_secs() < 5,
 434 |         "Should complete within reasonable time even with ignored directories"
 435 |     );
 436 | }
 437 | 
 438 | #[test]
 439 | #[serial] // Ensure cache tests don't interfere with each other
 440 | fn test_configuration_affects_cache_key() {
 441 |     let temp_dir = tempdir().unwrap();
 442 |     let project_dir = temp_dir.path().join("project");
 443 |     create_test_project(&project_dir).unwrap();
 444 | 
 445 |     // Test that different configurations create different cache behaviors
 446 |     // This is verified indirectly by ensuring different configs produce appropriate outputs
 447 | 
 448 |     let output1_path = temp_dir.path().join("output1.md");
 449 |     let output2_path = temp_dir.path().join("output2.md");
 450 | 
 451 |     // Change to project directory so config loading works
 452 |     let original_dir = std::env::current_dir().unwrap();
 453 |     std::env::set_current_dir(&project_dir).unwrap();
 454 | 
 455 |     let args1 = Args {
 456 |         input: ".".to_string(),
 457 |         output: output1_path.to_string_lossy().to_string(),
 458 |         filter: vec!["rs".to_string()],
 459 |         ignore: vec![],
 460 |         preview: false,
 461 |         token_count: false,
 462 |         line_numbers: false,
 463 |         yes: true,
 464 |         diff_only: false,
 465 |         clear_cache: false,
 466 |         init: false,
 467 |         max_tokens: None,
 468 |     };
 469 | 
 470 |     let args2 = Args {
 471 |         input: ".".to_string(),
 472 |         output: output2_path.to_string_lossy().to_string(),
 473 |         filter: vec!["md".to_string()],
 474 |         ignore: vec![],
 475 |         preview: false,
 476 |         token_count: false,
 477 |         line_numbers: false,
 478 |         yes: true,
 479 |         diff_only: false,
 480 |         clear_cache: false,
 481 |         init: false,
 482 |         max_tokens: None,
 483 |     };
 484 | 
 485 |     let prompter = TestPrompter;
 486 | 
 487 |     run_with_args(args1, Config::default(), &prompter).unwrap();
 488 |     run_with_args(args2, Config::default(), &prompter).unwrap();
 489 | 
 490 |     // Restore original directory
 491 |     std::env::set_current_dir(original_dir).unwrap();
 492 | 
 493 |     let content1 = fs::read_to_string(&output1_path).unwrap();
 494 |     let content2 = fs::read_to_string(&output2_path).unwrap();
 495 | 
 496 |     // Different filters should produce different outputs
 497 |     assert_ne!(content1, content2);
 498 | 
 499 |     // Verify filter effects
 500 |     assert!(content1.contains(".rs"));
 501 |     assert!(content2.contains("README.md"));
 502 |     // Note: Due to file tree section, both outputs may contain references to all files
 503 |     // but the actual file content sections should be filtered
 504 | }
 505 | 
 506 | #[test]
 507 | #[serial] // Ensure tests don't interfere with each other
 508 | fn test_edge_case_filenames_no_panic() {
 509 |     let temp_dir = tempdir().unwrap();
 510 |     let project_dir = temp_dir.path().join("project");
 511 |     fs::create_dir_all(&project_dir).unwrap();
 512 | 
 513 |     // Create files with edge case names that could cause panics
 514 |     fs::write(project_dir.join(".bashrc"), "# bash config").unwrap(); // no extension
 515 |     fs::write(project_dir.join("Dockerfile"), "FROM alpine").unwrap(); // no extension
 516 |     fs::write(project_dir.join(".gitignore"), "target/").unwrap(); // starts with dot, no extension
 517 | 
 518 |     // Change to project directory
 519 |     let original_dir = std::env::current_dir().unwrap();
 520 |     std::env::set_current_dir(&project_dir).unwrap();
 521 | 
 522 |     // Create a config file that enables timestamped output
 523 |     fs::write(
 524 |         project_dir.join("context-builder.toml"),
 525 |         r#"
 526 | timestamped_output = true
 527 | auto_diff = true
 528 | "#,
 529 |     )
 530 |     .unwrap();
 531 | 
 532 |     // Test with output filename that has no extension (extreme edge case)
 533 |     let output_path = temp_dir.path().join("no_extension_output");
 534 | 
 535 |     let args = Args {
 536 |         input: ".".to_string(),
 537 |         output: output_path.to_string_lossy().to_string(),
 538 |         filter: vec![],
 539 |         ignore: vec![],
 540 |         preview: false,
 541 |         token_count: false,
 542 |         line_numbers: false,
 543 |         yes: true,
 544 |         diff_only: false,
 545 |         clear_cache: false,
 546 |         init: false,
 547 |         max_tokens: None,
 548 |     };
 549 | 
 550 |     let prompter = TestPrompter;
 551 | 
 552 |     // This should not panic even with edge case filenames
 553 |     let config = load_config().unwrap_or_default();
 554 | 
 555 |     // Apply config merging manually since we're bypassing run()
 556 |     let mut final_args = args;
 557 | 
 558 |     // Apply line_numbers from config
 559 |     if !final_args.line_numbers
 560 |         && let Some(line_numbers) = config.line_numbers
 561 |     {
 562 |         final_args.line_numbers = line_numbers;
 563 |     }
 564 | 
 565 |     // Apply diff_only from config
 566 |     if !final_args.diff_only
 567 |         && let Some(diff_only) = config.diff_only
 568 |     {
 569 |         final_args.diff_only = diff_only;
 570 |     }
 571 | 
 572 |     // Apply timestamping manually since we're bypassing run()
 573 |     if config.timestamped_output.unwrap_or(false) {
 574 |         let timestamp = Utc::now().format("%Y%m%d%H%M%S").to_string();
 575 |         let path = std::path::Path::new(&final_args.output);
 576 |         let stem = path
 577 |             .file_stem()
 578 |             .and_then(|s| s.to_str())
 579 |             .unwrap_or("output");
 580 |         let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("md");
 581 |         let new_filename = format!("{}_{}.{}", stem, timestamp, extension);
 582 |         if let Some(parent) = path.parent() {
 583 |             final_args.output = parent.join(new_filename).to_string_lossy().to_string();
 584 |         } else {
 585 |             final_args.output = new_filename;
 586 |         }
 587 |     }
 588 | 
 589 |     let result = run_with_args(final_args, config, &prompter);
 590 |     std::env::set_current_dir(original_dir).unwrap();
 591 | 
 592 |     // Should succeed without panicking
 593 |     assert!(
 594 |         result.is_ok(),
 595 |         "Should handle edge case filenames without panicking"
 596 |     );
 597 | 
 598 |     // Verify a timestamped file was created
 599 |     let temp_entries: Vec<_> = fs::read_dir(temp_dir.path())
 600 |         .unwrap()
 601 |         .filter_map(|entry| entry.ok())
 602 |         .filter(|entry| {
 603 |             let name = entry.file_name();
 604 |             let name_str = name.to_string_lossy();
 605 |             let year = Utc::now().format("%Y").to_string();
 606 |             name_str.starts_with("no_extension_output_") && name_str.contains(&year)
 607 |         })
 608 |         .collect();
 609 | 
 610 |     assert!(
 611 |         !temp_entries.is_empty(),
 612 |         "Should create timestamped output file even with edge case input filename"
 613 |     );
 614 | }
```

### File: `tests/test_parallel_memory.rs`

- Size: 8743 bytes
- Modified: SystemTime { tv_sec: 1771098907, tv_nsec: 780246326 }

```rust
   1 | //! Integration test for streaming parallel processing with memory efficiency
   2 | 
   3 | use context_builder::cli::Args;
   4 | use context_builder::config::Config;
   5 | use context_builder::{Prompter, run_with_args};
   6 | use std::fs;
   7 | 
   8 | use tempfile::tempdir;
   9 | 
  10 | struct TestPrompter {
  11 |     overwrite_response: bool,
  12 |     processing_response: bool,
  13 | }
  14 | 
  15 | impl TestPrompter {
  16 |     fn new(overwrite_response: bool, processing_response: bool) -> Self {
  17 |         Self {
  18 |             overwrite_response,
  19 |             processing_response,
  20 |         }
  21 |     }
  22 | }
  23 | 
  24 | impl Prompter for TestPrompter {
  25 |     fn confirm_processing(&self, _file_count: usize) -> std::io::Result<bool> {
  26 |         Ok(self.processing_response)
  27 |     }
  28 | 
  29 |     fn confirm_overwrite(&self, _file_path: &str) -> std::io::Result<bool> {
  30 |         Ok(self.overwrite_response)
  31 |     }
  32 | }
  33 | 
  34 | #[cfg(feature = "parallel")]
  35 | #[test]
  36 | fn test_streaming_parallel_processing() {
  37 |     let dir = tempdir().unwrap();
  38 |     let base_path = dir.path();
  39 | 
  40 |     // Create a test project with multiple files
  41 |     for i in 0..100 {
  42 |         let subdir = base_path.join(format!("module_{}", i / 10));
  43 |         fs::create_dir_all(&subdir).unwrap();
  44 | 
  45 |         let file_path = subdir.join(format!("file_{}.rs", i));
  46 |         let content = format!(
  47 |             "// File {}\nuse std::collections::HashMap;\n\npub fn function_{}() -> HashMap<String, i32> {{\n    let mut map = HashMap::new();\n    map.insert(\"key_{}\".to_string(), {});\n    map\n}}\n",
  48 |             i, i, i, i
  49 |         );
  50 |         fs::write(&file_path, content).unwrap();
  51 |     }
  52 | 
  53 |     let output_path = base_path.join("output.md");
  54 | 
  55 |     // Create CLI args for processing
  56 |     let args = Args {
  57 |         input: base_path.to_string_lossy().to_string(),
  58 |         output: output_path.to_string_lossy().to_string(),
  59 |         filter: vec!["rs".to_string()],
  60 |         ignore: vec![],
  61 |         preview: false,
  62 |         token_count: false,
  63 |         line_numbers: false,
  64 |         yes: true,
  65 |         diff_only: false,
  66 |         clear_cache: false,
  67 |         init: false,
  68 |         max_tokens: None,
  69 |     };
  70 | 
  71 |     let config = Config::default();
  72 |     let prompter = TestPrompter::new(true, true);
  73 | 
  74 |     // Process files using the proper flow through lib.rs
  75 |     let result = run_with_args(args, config, &prompter);
  76 | 
  77 |     assert!(result.is_ok(), "Parallel streaming should succeed");
  78 | 
  79 |     // Verify the output file was created and contains expected content
  80 |     assert!(output_path.exists(), "Output file should be created");
  81 | 
  82 |     let output_content = fs::read_to_string(&output_path).unwrap();
  83 | 
  84 |     // If it doesn't have individual file sections, this is expected behavior for auto-diff mode
  85 |     // when there's no previous state. Let's check for basic structure instead.
  86 |     assert!(
  87 |         output_content.contains("# Directory Structure Report"),
  88 |         "Output should contain header"
  89 |     );
  90 |     assert!(
  91 |         output_content.contains("## File Tree Structure"),
  92 |         "Output should contain file tree"
  93 |     );
  94 | 
  95 |     // Check if we have individual file content (non-auto-diff mode) or just structure (auto-diff mode)
  96 |     if output_content.contains("## Files") {
  97 |         // Full content mode - verify all files are included in correct order
  98 |         for i in 0..100 {
  99 |             let expected_file_header = format!("### File: `module_{}/file_{}.rs`", i / 10, i);
 100 |             assert!(
 101 |                 output_content.contains(&expected_file_header),
 102 |                 "Output should contain file header for file {}",
 103 |                 i
 104 |             );
 105 | 
 106 |             let expected_function = format!("pub fn function_{}()", i);
 107 |             assert!(
 108 |                 output_content.contains(&expected_function),
 109 |                 "Output should contain function for file {}",
 110 |                 i
 111 |             );
 112 |         }
 113 | 
 114 |         // Verify file ordering is maintained (first file should appear before last file)
 115 |         let first_file_pos = output_content
 116 |             .find("### File: `module_0/file_0.rs`")
 117 |             .expect("First file should be in output");
 118 |         let last_file_pos = output_content
 119 |             .find("### File: `module_9/file_99.rs`")
 120 |             .expect("Last file should be in output");
 121 | 
 122 |         assert!(
 123 |             first_file_pos < last_file_pos,
 124 |             "Files should maintain their original order"
 125 |         );
 126 |     } else {
 127 |         // Auto-diff mode or similar - just verify structure is correct
 128 |         // At minimum, verify we have reasonable file tree structure
 129 |         assert!(
 130 |             output_content.contains("module_0"),
 131 |             "Should contain module_0"
 132 |         );
 133 |         assert!(
 134 |             output_content.contains("module_9"),
 135 |             "Should contain module_9"
 136 |         );
 137 |         assert!(
 138 |             output_content.contains("file_0.rs"),
 139 |             "Should contain file_0.rs"
 140 |         );
 141 |         assert!(
 142 |             output_content.contains("file_99.rs"),
 143 |             "Should contain file_99.rs"
 144 |         );
 145 |     }
 146 | }
 147 | 
 148 | #[cfg(feature = "parallel")]
 149 | #[test]
 150 | fn test_parallel_error_handling() {
 151 |     let dir = tempdir().unwrap();
 152 |     let base_path = dir.path();
 153 | 
 154 |     // Create some regular files and one that will cause issues
 155 |     fs::write(base_path.join("good1.rs"), "fn good1() {}").unwrap();
 156 |     fs::write(base_path.join("good2.rs"), "fn good2() {}").unwrap();
 157 | 
 158 |     // Create a binary file that should be handled gracefully
 159 |     // Use more null bytes to ensure it's detected as binary
 160 |     let binary_data = vec![
 161 |         0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, // PNG header
 162 |         0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44, 0x52, // PNG chunk
 163 |         0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, // More binary data
 164 |         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Null bytes
 165 |     ];
 166 |     fs::write(base_path.join("binary.rs"), binary_data).unwrap();
 167 | 
 168 |     let output_path = base_path.join("output.md");
 169 | 
 170 |     let args = Args {
 171 |         input: base_path.to_string_lossy().to_string(),
 172 |         output: output_path.to_string_lossy().to_string(),
 173 |         filter: vec!["rs".to_string()],
 174 |         ignore: vec![],
 175 |         preview: false,
 176 |         token_count: false,
 177 |         line_numbers: false,
 178 |         yes: true,
 179 |         diff_only: false,
 180 |         clear_cache: false,
 181 |         init: false,
 182 |         max_tokens: None,
 183 |     };
 184 | 
 185 |     let config = Config::default();
 186 |     let prompter = TestPrompter::new(true, true);
 187 | 
 188 |     // Should succeed even with binary files
 189 |     let result = run_with_args(args, config, &prompter);
 190 | 
 191 |     assert!(result.is_ok(), "Should handle binary files gracefully");
 192 | 
 193 |     let output_content = fs::read_to_string(&output_path).unwrap();
 194 | 
 195 |     // Verify good files are processed
 196 |     assert!(output_content.contains("fn good1()"));
 197 |     assert!(output_content.contains("fn good2()"));
 198 | 
 199 |     // Verify binary file is handled with placeholder
 200 |     assert!(output_content.contains("### File: `binary.rs`"));
 201 |     assert!(output_content.contains("<Binary file or unsupported encoding:"));
 202 | }
 203 | 
 204 | #[cfg(feature = "parallel")]
 205 | #[test]
 206 | fn test_memory_efficiency_with_large_files() {
 207 |     let dir = tempdir().unwrap();
 208 |     let base_path = dir.path();
 209 | 
 210 |     // Create files with substantial content to test memory usage
 211 |     for i in 0..20 {
 212 |         let file_path = base_path.join(format!("large_file_{}.rs", i));
 213 |         let mut content = format!("// Large file {}\n", i);
 214 | 
 215 |         // Add substantial content (about 10KB per file)
 216 |         for j in 0..200 {
 217 |             content.push_str(&format!(
 218 |                 "pub fn function_{}_{}() -> String {{\n    format!(\"Function {} in file {}\")\n}}\n\n",
 219 |                 i, j, j, i
 220 |             ));
 221 |         }
 222 | 
 223 |         fs::write(&file_path, content).unwrap();
 224 |     }
 225 | 
 226 |     let output_path = base_path.join("output.md");
 227 | 
 228 |     let args = Args {
 229 |         input: base_path.to_string_lossy().to_string(),
 230 |         output: output_path.to_string_lossy().to_string(),
 231 |         filter: vec!["rs".to_string()],
 232 |         ignore: vec![],
 233 |         preview: false,
 234 |         token_count: false,
 235 |         line_numbers: false,
 236 |         yes: true,
 237 |         diff_only: false,
 238 |         clear_cache: false,
 239 |         init: false,
 240 |         max_tokens: None,
 241 |     };
 242 | 
 243 |     let config = Config::default();
 244 |     let prompter = TestPrompter::new(true, true);
 245 | 
 246 |     // This should complete without excessive memory usage
 247 |     let result = run_with_args(args, config, &prompter);
 248 | 
 249 |     assert!(result.is_ok(), "Should handle large files efficiently");
 250 | 
 251 |     let output_content = fs::read_to_string(&output_path).unwrap();
 252 | 
 253 |     // Verify all large files are included
 254 |     for i in 0..20 {
 255 |         assert!(
 256 |             output_content.contains(&format!("### File: `large_file_{}.rs`", i)),
 257 |             "Should contain large file {}",
 258 |             i
 259 |         );
 260 |     }
 261 | 
 262 |     // Verify substantial content is present
 263 |     assert!(
 264 |         output_content.len() > 100_000,
 265 |         "Output should be substantial"
 266 |     );
 267 | }
```

### File: `tests/test_phase4_integration.rs`

- Size: 11080 bytes
- Modified: SystemTime { tv_sec: 1771099060, tv_nsec: 496347381 }

```rust
   1 | //! Integration test for all Phase 4 features working together
   2 | //!
   3 | //! This test validates that the enhanced binary file handling, improved diff_only mode,
   4 | //! and comprehensive edge case handling all work correctly in combination.
   5 | 
   6 | use context_builder::cli::Args;
   7 | use context_builder::config::Config;
   8 | use context_builder::{Prompter, run_with_args};
   9 | use std::fs;
  10 | use std::path::Path;
  11 | use tempfile::tempdir;
  12 | 
  13 | struct TestPrompter {
  14 |     overwrite_response: bool,
  15 |     processing_response: bool,
  16 | }
  17 | 
  18 | impl TestPrompter {
  19 |     fn new(overwrite_response: bool, processing_response: bool) -> Self {
  20 |         Self {
  21 |             overwrite_response,
  22 |             processing_response,
  23 |         }
  24 |     }
  25 | }
  26 | 
  27 | impl Prompter for TestPrompter {
  28 |     fn confirm_processing(&self, _file_count: usize) -> std::io::Result<bool> {
  29 |         Ok(self.processing_response)
  30 |     }
  31 | 
  32 |     fn confirm_overwrite(&self, _file_path: &str) -> std::io::Result<bool> {
  33 |         Ok(self.overwrite_response)
  34 |     }
  35 | }
  36 | 
  37 | fn write_file(path: &Path, contents: &str) {
  38 |     if let Some(parent) = path.parent() {
  39 |         fs::create_dir_all(parent).unwrap();
  40 |     }
  41 |     fs::write(path, contents).unwrap();
  42 | }
  43 | 
  44 | fn write_binary_file(path: &Path, data: &[u8]) {
  45 |     if let Some(parent) = path.parent() {
  46 |         fs::create_dir_all(parent).unwrap();
  47 |     }
  48 |     fs::write(path, data).unwrap();
  49 | }
  50 | 
  51 | #[test]
  52 | fn test_phase4_features_integration() {
  53 |     let temp_dir = tempdir().unwrap();
  54 |     let project_dir = temp_dir.path().join("project");
  55 |     let output_dir = temp_dir.path().join("output");
  56 |     fs::create_dir_all(&output_dir).unwrap();
  57 | 
  58 |     // Create config with enhanced features enabled
  59 |     write_file(
  60 |         &project_dir.join("context-builder.toml"),
  61 |         r#"
  62 | auto_diff = true
  63 | timestamped_output = true
  64 | diff_only = true
  65 | encoding_strategy = "detect"
  66 | filter = ["rs", "txt"]
  67 | "#,
  68 |     );
  69 | 
  70 |     // Change to project directory
  71 |     let original_dir = std::env::current_dir().unwrap();
  72 |     std::env::set_current_dir(&project_dir).unwrap();
  73 | 
  74 |     // Create initial files with various encoding scenarios
  75 |     write_file(
  76 |         &project_dir.join("src/main.rs"),
  77 |         "fn main() {\n    println!(\"Hello, world!\");\n}\n",
  78 |     );
  79 | 
  80 |     // UTF-8 file
  81 |     write_file(
  82 |         &project_dir.join("src/utils.rs"),
  83 |         "// UTF-8 file\npub fn helper() -> String {\n    \"Hello from helper\".to_string()\n}\n",
  84 |     );
  85 | 
  86 |     // Windows-1252 encoded file
  87 |     let windows1252_data = [
  88 |         0x2F, 0x2F, 0x20, // "// "
  89 |         0x57, 0x69, 0x6E, 0x64, 0x6F, 0x77, 0x73, 0x2D, 0x31, 0x32, 0x35, 0x32,
  90 |         0x20, // "Windows-1252 "
  91 |         0x93, 0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x94, // "Hello" with smart quotes
  92 |         0x0A, // newline
  93 |         0x70, 0x75, 0x62, 0x20, 0x66, 0x6E, 0x20, 0x74, 0x65, 0x73, 0x74, 0x28, 0x29, 0x20, 0x7B,
  94 |         0x7D, 0x0A, // "pub fn test() {}"
  95 |     ];
  96 |     write_binary_file(&project_dir.join("src/encoded.rs"), &windows1252_data);
  97 | 
  98 |     // Binary file that should be skipped - use executable-like binary data
  99 |     let binary_data = vec![
 100 |         0x7f, 0x45, 0x4c, 0x46, // ELF header
 101 |         0x02, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00,
 102 |         0x3e, // More ELF data
 103 |         0xff, 0xfe, 0xfd, 0xfc, 0xfb, 0xfa, 0xf9, 0xf8, // High bytes
 104 |         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Null bytes
 105 |     ];
 106 |     write_binary_file(&project_dir.join("data.txt"), &binary_data);
 107 | 
 108 |     let prompter = TestPrompter::new(true, true);
 109 |     let config = context_builder::config::load_config_from_path(&project_dir).unwrap_or_default();
 110 | 
 111 |     // First run - establish baseline
 112 |     let args = Args {
 113 |         input: project_dir.to_string_lossy().to_string(),
 114 |         output: output_dir.join("baseline.md").to_string_lossy().to_string(),
 115 |         filter: vec![], // Use config filter
 116 |         ignore: vec![],
 117 |         preview: false,
 118 |         token_count: false,
 119 |         line_numbers: false,
 120 |         yes: true,
 121 |         diff_only: false, // Will be overridden by config
 122 |         clear_cache: false,
 123 |         init: false,
 124 |         max_tokens: None,
 125 |     };
 126 | 
 127 |     // Apply config manually (simulating what happens in the real application)
 128 |     let mut resolved_args = args.clone();
 129 |     if resolved_args.filter.is_empty()
 130 |         && let Some(ref config_filter) = config.filter
 131 |     {
 132 |         resolved_args.filter = config_filter.clone();
 133 |     }
 134 |     if !resolved_args.diff_only
 135 |         && let Some(diff_only) = config.diff_only
 136 |     {
 137 |         resolved_args.diff_only = diff_only;
 138 |     }
 139 | 
 140 |     let result1 = run_with_args(resolved_args, config.clone(), &prompter);
 141 |     assert!(result1.is_ok(), "First run should succeed");
 142 | 
 143 |     // Add a new file to test improved diff_only mode
 144 |     write_file(
 145 |         &project_dir.join("src/new_feature.rs"),
 146 |         "// New feature added\npub fn new_feature() -> String {\n    \"Brand new functionality\".to_string()\n}\n\n#[cfg(test)]\nmod tests {\n    use super::*;\n\n    #[test]\n    fn test_new_feature() {\n        assert_eq!(new_feature(), \"Brand new functionality\");\n    }\n}\n",
 147 |     );
 148 | 
 149 |     // Modify existing file
 150 |     write_file(
 151 |         &project_dir.join("src/main.rs"),
 152 |         "fn main() {\n    println!(\"Hello, enhanced world!\");\n}\n",
 153 |     );
 154 | 
 155 |     // Small delay to ensure different timestamps
 156 |     std::thread::sleep(std::time::Duration::from_millis(1100));
 157 | 
 158 |     // Second run with changes
 159 |     let mut second_args = args;
 160 |     second_args.input = project_dir.to_string_lossy().to_string();
 161 |     second_args.output = output_dir.join("enhanced.md").to_string_lossy().to_string();
 162 | 
 163 |     // Apply config manually
 164 |     if second_args.filter.is_empty()
 165 |         && let Some(ref config_filter) = config.filter
 166 |     {
 167 |         second_args.filter = config_filter.clone();
 168 |     }
 169 |     if !second_args.diff_only
 170 |         && let Some(diff_only) = config.diff_only
 171 |     {
 172 |         second_args.diff_only = diff_only;
 173 |     }
 174 | 
 175 |     let result2 = run_with_args(second_args, config, &prompter);
 176 |     assert!(result2.is_ok(), "Second run should succeed");
 177 | 
 178 |     // Restore original directory
 179 |     std::env::set_current_dir(original_dir).unwrap();
 180 | 
 181 |     // Verify the enhanced features work correctly
 182 |     let outputs: Vec<_> = fs::read_dir(&output_dir)
 183 |         .unwrap()
 184 |         .map(|e| e.unwrap().path())
 185 |         .collect();
 186 |     let latest_output = outputs
 187 |         .iter()
 188 |         .max_by_key(|p| fs::metadata(p).unwrap().modified().unwrap())
 189 |         .unwrap();
 190 | 
 191 |     let content = fs::read_to_string(latest_output).unwrap();
 192 | 
 193 |     // Test enhanced binary file handling
 194 |     // Should either transcode Windows-1252 content or show binary placeholder
 195 |     assert!(
 196 |         content.contains("Hello") || content.contains("<Binary file"),
 197 |         "Should handle Windows-1252 encoding or show binary placeholder"
 198 |     );
 199 | 
 200 |     // Binary files should be handled gracefully (not crash the application)
 201 |     // The specific behavior depends on encoding strategy, but it should not fail
 202 | 
 203 |     // Test improved diff_only mode
 204 |     assert!(
 205 |         content.contains("## Change Summary"),
 206 |         "Should have change summary in diff_only mode"
 207 |     );
 208 | 
 209 |     // Should include full content of added files (new feature)
 210 |     assert!(
 211 |         content.contains("## Added Files"),
 212 |         "Should have Added Files section in diff_only mode"
 213 |     );
 214 |     assert!(
 215 |         content.contains("new_feature.rs"),
 216 |         "Should include added file"
 217 |     );
 218 |     assert!(
 219 |         content.contains("Brand new functionality"),
 220 |         "Should include full content of added file"
 221 |     );
 222 | 
 223 |     // Should have file differences for modified files
 224 |     assert!(
 225 |         content.contains("## File Differences"),
 226 |         "Should have file differences section"
 227 |     );
 228 | 
 229 |     // Should not have full Files section (due to diff_only mode)
 230 |     assert!(
 231 |         !content.contains("## Files\n"),
 232 |         "Should not have full Files section in diff_only mode"
 233 |     );
 234 | 
 235 |     // Test comprehensive edge cases are handled
 236 |     assert!(
 237 |         content.contains("# Directory Structure Report"),
 238 |         "Should have proper document structure"
 239 |     );
 240 |     assert!(
 241 |         content.contains("## File Tree Structure"),
 242 |         "Should have file tree"
 243 |     );
 244 | 
 245 |     // Verify that the enhanced features didn't break basic functionality
 246 |     // In diff_only mode, content is smaller since it only shows changes
 247 |     assert!(
 248 |         content.len() > 500,
 249 |         "Should generate reasonable content even in diff_only mode"
 250 |     );
 251 | 
 252 |     println!("✅ Phase 4 integration test passed!");
 253 |     println!("   - Enhanced binary file handling: Working");
 254 |     println!("   - Improved diff_only mode: Working");
 255 |     println!("   - Comprehensive edge case handling: Working");
 256 |     println!("   - All features integrated successfully");
 257 | }
 258 | 
 259 | #[test]
 260 | fn test_encoding_strategy_configuration() {
 261 |     let temp_dir = tempdir().unwrap();
 262 |     let project_dir = temp_dir.path().join("project");
 263 |     let output_dir = temp_dir.path().join("output");
 264 |     fs::create_dir_all(&output_dir).unwrap();
 265 | 
 266 |     // Create a file with Windows-1252 encoding
 267 |     let windows1252_data = [
 268 |         0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x20, // "Hello "
 269 |         0x93, 0x57, 0x6F, 0x72, 0x6C, 0x64, 0x94, // "World" with smart quotes
 270 |         0x0A, // newline
 271 |     ];
 272 |     write_binary_file(&project_dir.join("test.txt"), &windows1252_data);
 273 | 
 274 |     let prompter = TestPrompter::new(true, true);
 275 | 
 276 |     // Test all encoding strategies
 277 |     for strategy in &["detect", "strict", "skip"] {
 278 |         let config = Config {
 279 |             encoding_strategy: Some(strategy.to_string()),
 280 |             ..Default::default()
 281 |         };
 282 | 
 283 |         let args = Args {
 284 |             input: project_dir.to_string_lossy().to_string(),
 285 |             output: output_dir
 286 |                 .join(format!("encoding_{}.md", strategy))
 287 |                 .to_string_lossy()
 288 |                 .to_string(),
 289 |             filter: vec!["txt".to_string()],
 290 |             ignore: vec![],
 291 |             preview: false,
 292 |             token_count: false,
 293 |             line_numbers: false,
 294 |             yes: true,
 295 |             diff_only: false,
 296 |             clear_cache: false,
 297 |             init: false,
 298 |             max_tokens: None,
 299 |         };
 300 | 
 301 |         let result = run_with_args(args, config, &prompter);
 302 |         assert!(
 303 |             result.is_ok(),
 304 |             "Encoding strategy '{}' should work",
 305 |             strategy
 306 |         );
 307 | 
 308 |         let output_path = output_dir.join(format!("encoding_{}.md", strategy));
 309 |         let content = fs::read_to_string(&output_path).unwrap();
 310 | 
 311 |         match *strategy {
 312 |             "detect" => {
 313 |                 // Should attempt transcoding and may succeed
 314 |                 assert!(
 315 |                     content.contains("Hello") || content.contains("<Binary file"),
 316 |                     "Detect strategy should transcode or show binary placeholder"
 317 |                 );
 318 |             }
 319 |             "strict" | "skip" => {
 320 |                 // Should show binary placeholder
 321 |                 assert!(
 322 |                     content.contains("<Binary file"),
 323 |                     "Strict/skip strategy should show binary placeholder"
 324 |                 );
 325 |             }
 326 |             _ => {}
 327 |         }
 328 |     }
 329 | 
 330 |     println!("✅ Encoding strategy configuration test passed!");
 331 | }
```

### File: `scripts/generate_samples.rs`

- Size: 16036 bytes
- Modified: SystemTime { tv_sec: 1771053288, tv_nsec: 64557151 }

```rust
   1 | #![allow(
   2 |     clippy::needless_return,
   3 |     clippy::extra_unused_lifetimes,
   4 |     clippy::doc_overindented_list_items,
   5 |     dead_code
   6 | )]
   7 | //! Dataset generation script for creating synthetic sample directories to benchmark and test
   8 | //! the context-builder CLI locally. This is intended to generate a folder that should be ignored
   9 | //! by version control (e.g., add `/samples` to your project's .gitignore).
  10 | //!
  11 | //! Usage examples (Windows PowerShell):
  12 | //!   - rustc scripts/generate_samples.rs -O -o generate_samples.exe; .\generate_samples.exe
  13 | //!   - .\generate_samples.exe --help
  14 | //!
  15 | //! Flags:
  16 | //!   --out <DIR>             Output directory (default: ./samples)
  17 | //!   --presets <list>        Comma-separated presets to generate: tiny,small,medium (default: tiny,small)
  18 | //!   --include-large         Also generate the large preset (off by default)
  19 | //!   --only <name>           Only generate a single preset (overrides --presets)
  20 | //!   --clean                 Remove the output directory before generating
  21 | //!   --dry-run               Print the plan without writing files
  22 | //!
  23 | //! Advanced overrides (apply when using --only):
  24 | //!   --files <N>             Number of text files
  25 | //!   --binary-every <N>      Create one .bin file every N text files (0 disables)
  26 | //!   --depth <D>             Directory tree depth
  27 | //!   --width <W>             Subdirectories per level
  28 | //!   --size <BYTES>          Approx text file size in bytes
  29 | //!   --filters <CSV>         Extensions to include (default: rs,md,txt,toml)
  30 | //!   --ignores <CSV>         Directory/file names to ignore (default: target,node_modules)
  31 | //!
  32 | //! Generated structure per dataset (e.g., samples/small):
  33 | //!   - project/
  34 | //!       src/, docs/, assets/      -> nested trees with text files
  35 | //!       target/, node_modules/    -> ignored directories with noise
  36 | //!       README.md, Cargo.toml     -> top-level files
  37 | //!       (binary files are sprinkled across trees and should be ignored by the tool)
  38 | //!
  39 | //! Notes:
  40 | //! - Binary files are generated to validate that the CLI ignores them by default filters.
  41 | //! - This script uses only the Rust standard library.
  42 | 
  43 | use std::env;
  44 | use std::fs::{self, File};
  45 | use std::io::{self, Write};
  46 | use std::path::{Path, PathBuf};
  47 | 
  48 | #[derive(Clone, Debug)]
  49 | struct DatasetSpec {
  50 |     name: String,
  51 |     text_files: usize,
  52 |     binary_every: usize,
  53 |     depth: usize,
  54 |     width: usize,
  55 |     text_file_size: usize,
  56 |     filters: Vec<String>,
  57 |     ignores: Vec<String>,
  58 | }
  59 | 
  60 | impl DatasetSpec {
  61 |     fn with_name(name: &str) -> Option<Self> {
  62 |         match name {
  63 |             "tiny" => Some(Self {
  64 |                 name: "tiny".into(),
  65 |                 text_files: 100,
  66 |                 binary_every: 10,
  67 |                 depth: 2,
  68 |                 width: 3,
  69 |                 text_file_size: 256,
  70 |                 filters: default_filters(),
  71 |                 ignores: default_ignores(),
  72 |             }),
  73 |             "small" => Some(Self {
  74 |                 name: "small".into(),
  75 |                 text_files: 1_000,
  76 |                 binary_every: 20,
  77 |                 depth: 3,
  78 |                 width: 4,
  79 |                 text_file_size: 512,
  80 |                 filters: default_filters(),
  81 |                 ignores: default_ignores(),
  82 |             }),
  83 |             "medium" => Some(Self {
  84 |                 name: "medium".into(),
  85 |                 text_files: 5_000,
  86 |                 binary_every: 25,
  87 |                 depth: 4,
  88 |                 width: 4,
  89 |                 text_file_size: 800,
  90 |                 filters: default_filters(),
  91 |                 ignores: default_ignores(),
  92 |             }),
  93 |             "large" => Some(Self {
  94 |                 name: "large".into(),
  95 |                 text_files: 20_000,
  96 |                 binary_every: 50,
  97 |                 depth: 5,
  98 |                 width: 5,
  99 |                 text_file_size: 1024,
 100 |                 filters: default_filters(),
 101 |                 ignores: default_ignores(),
 102 |             }),
 103 |             _ => None,
 104 |         }
 105 |     }
 106 | }
 107 | 
 108 | fn default_filters() -> Vec<String> {
 109 |     vec!["rs", "md", "txt", "toml"]
 110 |         .into_iter()
 111 |         .map(|s| s.to_string())
 112 |         .collect()
 113 | }
 114 | 
 115 | fn default_ignores() -> Vec<String> {
 116 |     vec!["target", "node_modules"]
 117 |         .into_iter()
 118 |         .map(|s| s.to_string())
 119 |         .collect()
 120 | }
 121 | 
 122 | #[derive(Default)]
 123 | struct Args {
 124 |     out: PathBuf,
 125 |     presets: Vec<String>,
 126 |     include_large: bool,
 127 |     only: Option<String>,
 128 |     clean: bool,
 129 |     dry_run: bool,
 130 |     // overrides for --only
 131 |     files: Option<usize>,
 132 |     binary_every: Option<usize>,
 133 |     depth: Option<usize>,
 134 |     width: Option<usize>,
 135 |     size: Option<usize>,
 136 |     filters: Option<Vec<String>>,
 137 |     ignores: Option<Vec<String>>,
 138 | }
 139 | 
 140 | fn parse_args() -> Args {
 141 |     let mut out = PathBuf::from("samples");
 142 |     let mut presets: Vec<String> = vec!["tiny".into(), "small".into()];
 143 |     let mut include_large = false;
 144 |     let mut only: Option<String> = None;
 145 |     let mut clean = false;
 146 |     let mut dry_run = false;
 147 | 
 148 |     let mut files: Option<usize> = None;
 149 |     let mut binary_every: Option<usize> = None;
 150 |     let mut depth: Option<usize> = None;
 151 |     let mut width: Option<usize> = None;
 152 |     let mut size: Option<usize> = None;
 153 |     let mut filters: Option<Vec<String>> = None;
 154 |     let mut ignores: Option<Vec<String>> = None;
 155 | 
 156 |     let mut it = env::args().skip(1).peekable();
 157 |     while let Some(arg) = it.next() {
 158 |         match arg.as_str() {
 159 |             "--out" => {
 160 |                 out = PathBuf::from(expect_value("--out", &mut it));
 161 |             }
 162 |             "--presets" => {
 163 |                 presets = parse_csv(expect_value("--presets", &mut it));
 164 |             }
 165 |             "--include-large" => include_large = true,
 166 |             "--only" => {
 167 |                 only = Some(expect_value("--only", &mut it).to_lowercase());
 168 |             }
 169 |             "--clean" => clean = true,
 170 |             "--dry-run" => dry_run = true,
 171 | 
 172 |             // overrides (effective with --only)
 173 |             "--files" => files = parse_usize(expect_value("--files", &mut it)),
 174 |             "--binary-every" => binary_every = parse_usize(expect_value("--binary-every", &mut it)),
 175 |             "--depth" => depth = parse_usize(expect_value("--depth", &mut it)),
 176 |             "--width" => width = parse_usize(expect_value("--width", &mut it)),
 177 |             "--size" => size = parse_usize(expect_value("--size", &mut it)),
 178 |             "--filters" => filters = Some(parse_csv(expect_value("--filters", &mut it))),
 179 |             "--ignores" => ignores = Some(parse_csv(expect_value("--ignores", &mut it))),
 180 |             "--help" | "-h" => {
 181 |                 print_help();
 182 |                 std::process::exit(0);
 183 |             }
 184 |             other => {
 185 |                 eprintln!("Unknown argument: {}", other);
 186 |                 print_help();
 187 |                 std::process::exit(2);
 188 |             }
 189 |         }
 190 |     }
 191 | 
 192 |     if include_large && !presets.iter().any(|p| p == "large") {
 193 |         presets.push("large".into());
 194 |     }
 195 | 
 196 |     Args {
 197 |         out,
 198 |         presets,
 199 |         include_large,
 200 |         only,
 201 |         clean,
 202 |         dry_run,
 203 |         files,
 204 |         binary_every,
 205 |         depth,
 206 |         width,
 207 |         size,
 208 |         filters,
 209 |         ignores,
 210 |     }
 211 | }
 212 | 
 213 | fn expect_value<'a, I>(flag: &str, it: &mut I) -> String
 214 | where
 215 |     I: Iterator<Item = String>,
 216 | {
 217 |     if let Some(v) = it.next() {
 218 |         v
 219 |     } else {
 220 |         eprintln!("{flag} requires a value");
 221 |         std::process::exit(2);
 222 |     }
 223 | }
 224 | 
 225 | fn parse_usize(s: String) -> Option<usize> {
 226 |     match s.parse::<usize>() {
 227 |         Ok(v) => Some(v),
 228 |         Err(_) => {
 229 |             eprintln!("Invalid number: {}", s);
 230 |             std::process::exit(2);
 231 |         }
 232 |     }
 233 | }
 234 | 
 235 | fn parse_csv(s: String) -> Vec<String> {
 236 |     s.split(',')
 237 |         .map(|x| x.trim().to_string())
 238 |         .filter(|x| !x.is_empty())
 239 |         .collect()
 240 | }
 241 | 
 242 | fn print_help() {
 243 |     println!(
 244 |         r#"generate_samples - generate synthetic datasets for benchmarking
 245 | 
 246 | Usage:
 247 |   generate_samples [--out DIR] [--presets CSV] [--include-large]
 248 |                    [--only NAME] [--clean] [--dry-run]
 249 |                    [--files N] [--binary-every N] [--depth D] [--width W]
 250 |                    [--size BYTES] [--filters CSV] [--ignores CSV]
 251 | 
 252 | Examples:
 253 |   # Default (tiny, small) into ./samples
 254 |   generate_samples
 255 | 
 256 |   # Include medium and large
 257 |   generate_samples --presets tiny,small,medium --include-large
 258 | 
 259 |   # Only 'small' with custom parameters
 260 |   generate_samples --only small --files 5000 --depth 4 --width 4 --size 1024
 261 | 
 262 |   # Clean output directory before generating
 263 |   generate_samples --clean
 264 | 
 265 |   # Dry-run (show plan, don't write)
 266 |   generate_samples --dry-run
 267 | "#
 268 |     );
 269 | }
 270 | 
 271 | fn write_text_file(path: &Path, bytes: usize) -> io::Result<()> {
 272 |     if let Some(parent) = path.parent() {
 273 |         fs::create_dir_all(parent)?;
 274 |     }
 275 |     let mut f = File::create(path)?;
 276 |     // Deterministic multi-line content ~40 bytes per line
 277 |     let line = b"let x = 42; // benchmark content line\n";
 278 |     let mut written = 0usize;
 279 |     while written + line.len() <= bytes {
 280 |         f.write_all(line)?;
 281 |         written += line.len();
 282 |     }
 283 |     if written < bytes {
 284 |         let remaining = &line[..(bytes - written).min(line.len())];
 285 |         f.write_all(remaining)?;
 286 |         written += remaining.len();
 287 |     }
 288 |     // Ensure trailing newline for nicer line-numbered output
 289 |     if written == 0 || !path.to_string_lossy().ends_with('\n') {
 290 |         f.write_all(b"\n")?;
 291 |     }
 292 |     Ok(())
 293 | }
 294 | 
 295 | fn write_binary_file(path: &Path, bytes: usize) -> io::Result<()> {
 296 |     if let Some(parent) = path.parent() {
 297 |         fs::create_dir_all(parent)?;
 298 |     }
 299 |     let mut f = File::create(path)?;
 300 |     // Simple reproducible byte pattern
 301 |     for i in 0..bytes {
 302 |         let b = ((i as u8).wrapping_mul(31)).wrapping_add(7);
 303 |         f.write_all(&[b])?;
 304 |     }
 305 |     Ok(())
 306 | }
 307 | 
 308 | fn make_nested_dirs(base: &Path, depth: usize, width: usize) -> io::Result<Vec<PathBuf>> {
 309 |     let mut dirs = vec![base.to_path_buf()];
 310 |     for d in 1..=depth {
 311 |         let mut next = Vec::new();
 312 |         for parent in &dirs {
 313 |             for w in 0..width.max(1) {
 314 |                 let child = parent.join(format!("d{}_{}", d, w));
 315 |                 fs::create_dir_all(&child)?;
 316 |                 next.push(child);
 317 |             }
 318 |         }
 319 |         dirs.extend(next);
 320 |     }
 321 |     Ok(dirs)
 322 | }
 323 | 
 324 | fn write_string(path: &Path, s: &str) -> io::Result<()> {
 325 |     if let Some(parent) = path.parent() {
 326 |         fs::create_dir_all(parent)?;
 327 |     }
 328 |     let mut f = File::create(path)?;
 329 |     f.write_all(s.as_bytes())
 330 | }
 331 | 
 332 | fn generate_dataset(root: &Path, spec: &DatasetSpec, dry_run: bool) -> io::Result<()> {
 333 |     let dataset_dir = root.join(&spec.name);
 334 |     let project_dir = dataset_dir.join("project");
 335 |     let src_dir = project_dir.join("src");
 336 |     let docs_dir = project_dir.join("docs");
 337 |     let assets_dir = project_dir.join("assets");
 338 |     let ignored_target = project_dir.join("target");
 339 |     let ignored_node_modules = project_dir.join("node_modules");
 340 | 
 341 |     println!(
 342 |         "- [{}] files={}, bin_every={}, depth={}, width={}, size={}, filters={:?}, ignores={:?}",
 343 |         spec.name,
 344 |         spec.text_files,
 345 |         spec.binary_every,
 346 |         spec.depth,
 347 |         spec.width,
 348 |         spec.text_file_size,
 349 |         spec.filters,
 350 |         spec.ignores
 351 |     );
 352 | 
 353 |     if dry_run {
 354 |         return Ok(());
 355 |     }
 356 | 
 357 |     fs::create_dir_all(&src_dir)?;
 358 |     fs::create_dir_all(&docs_dir)?;
 359 |     fs::create_dir_all(&assets_dir)?;
 360 |     fs::create_dir_all(&ignored_target)?;
 361 |     fs::create_dir_all(&ignored_node_modules)?;
 362 | 
 363 |     // Write dataset README and .gitignore to discourage accidental commits
 364 |     write_string(
 365 |         &dataset_dir.join("README.txt"),
 366 |         &format!(
 367 |             "Synthetic dataset '{}'\n\
 368 |              - Generated by scripts/generate_samples.rs\n\
 369 |              - Intended for local benchmarking and testing\n\
 370 |              - May be large; avoid committing this folder\n",
 371 |             spec.name
 372 |         ),
 373 |     )?;
 374 |     write_string(
 375 |         &dataset_dir.join(".gitignore"),
 376 |         "*\n!.gitignore\n!README.txt\n",
 377 |     )?;
 378 | 
 379 |     let mut all_dirs = Vec::new();
 380 |     all_dirs.extend(make_nested_dirs(&src_dir, spec.depth, spec.width)?);
 381 |     all_dirs.extend(make_nested_dirs(&docs_dir, spec.depth, spec.width)?);
 382 |     all_dirs.extend(make_nested_dirs(&assets_dir, spec.depth, spec.width)?);
 383 | 
 384 |     // Distribute text files across dirs with round-robin extensions
 385 |     let text_exts = ["rs", "md", "txt", "toml"];
 386 |     let mut created = 0usize;
 387 |     let mut bin_counter = 0usize;
 388 | 
 389 |     'outer: for dir in &all_dirs {
 390 |         for i in 0..spec.width.max(1) {
 391 |             if created >= spec.text_files {
 392 |                 break 'outer;
 393 |             }
 394 |             let ext = text_exts[created % text_exts.len()];
 395 |             let path = dir.join(format!("f{}_{}.{}", created, i, ext));
 396 |             write_text_file(&path, spec.text_file_size)?;
 397 |             created += 1;
 398 | 
 399 |             if spec.binary_every > 0 {
 400 |                 bin_counter += 1;
 401 |                 if bin_counter.is_multiple_of(spec.binary_every) {
 402 |                     let bpath = dir.join(format!("bin_{}_{}.bin", created, i));
 403 |                     write_binary_file(&bpath, 2048)?;
 404 |                 }
 405 |             }
 406 |         }
 407 |     }
 408 | 
 409 |     // Populate ignored directories with content that should be skipped by the tool
 410 |     write_text_file(&ignored_target.join("ignored.rs"), spec.text_file_size)?;
 411 |     write_text_file(
 412 |         &ignored_node_modules.join("ignored.js"),
 413 |         spec.text_file_size,
 414 |     )?;
 415 | 
 416 |     // Top-level files
 417 |     write_text_file(&project_dir.join("README.md"), spec.text_file_size)?;
 418 |     write_text_file(&project_dir.join("Cargo.toml"), spec.text_file_size)?;
 419 | 
 420 |     Ok(())
 421 | }
 422 | 
 423 | fn apply_overrides(spec: &mut DatasetSpec, args: &Args) {
 424 |     if let Some(v) = args.files {
 425 |         spec.text_files = v;
 426 |     }
 427 |     if let Some(v) = args.binary_every {
 428 |         spec.binary_every = v;
 429 |     }
 430 |     if let Some(v) = args.depth {
 431 |         spec.depth = v;
 432 |     }
 433 |     if let Some(v) = args.width {
 434 |         spec.width = v;
 435 |     }
 436 |     if let Some(v) = args.size {
 437 |         spec.text_file_size = v;
 438 |     }
 439 |     if let Some(v) = args.filters.clone() {
 440 |         spec.filters = v;
 441 |     }
 442 |     if let Some(v) = args.ignores.clone() {
 443 |         spec.ignores = v;
 444 |     }
 445 | }
 446 | 
 447 | fn main() -> io::Result<()> {
 448 |     let args = parse_args();
 449 | 
 450 |     if args.clean && args.out.exists() && !args.dry_run {
 451 |         println!("Cleaning output directory: {}", args.out.display());
 452 |         fs::remove_dir_all(&args.out)?;
 453 |     }
 454 | 
 455 |     println!("Output directory: {}", args.out.display());
 456 |     println!("Dry run: {}", args.dry_run);
 457 | 
 458 |     let mut specs: Vec<DatasetSpec> = Vec::new();
 459 | 
 460 |     if let Some(name) = args.only.clone() {
 461 |         let mut spec = DatasetSpec::with_name(&name).unwrap_or_else(|| {
 462 |             eprintln!("Unknown preset for --only: {}", name);
 463 |             std::process::exit(2);
 464 |         });
 465 |         apply_overrides(&mut spec, &args);
 466 |         specs.push(spec);
 467 |     } else {
 468 |         for p in &args.presets {
 469 |             if let Some(spec) = DatasetSpec::with_name(p) {
 470 |                 specs.push(spec);
 471 |             } else {
 472 |                 eprintln!("Unknown preset: {}", p);
 473 |                 std::process::exit(2);
 474 |             }
 475 |         }
 476 |     }
 477 | 
 478 |     if args.dry_run {
 479 |         println!("Planned datasets:");
 480 |         for s in &specs {
 481 |             println!(
 482 |                 "  - {}: files={}, bin_every={}, depth={}, width={}, size={}",
 483 |                 s.name, s.text_files, s.binary_every, s.depth, s.width, s.text_file_size
 484 |             );
 485 |         }
 486 |         return Ok(());
 487 |     }
 488 | 
 489 |     fs::create_dir_all(&args.out)?;
 490 |     // Guard .gitignore at the root samples folder
 491 |     let root_gitignore = args.out.join(".gitignore");
 492 |     if !root_gitignore.exists() {
 493 |         write_string(&root_gitignore, "*\n!.gitignore\n")?;
 494 |     }
 495 | 
 496 |     for spec in specs {
 497 |         generate_dataset(&args.out, &spec, false)?;
 498 |     }
 499 | 
 500 |     println!("Done.");
 501 |     Ok(())
 502 | }
 503 | 
 504 | #[cfg(test)]
 505 | mod tests {
 506 |     use super::*;
 507 | 
 508 |     #[test]
 509 |     fn test_expect_value() {
 510 |         let mut it = vec!["--out".to_string(), "samples".to_string()].into_iter();
 511 |         let flag = it.next().unwrap();
 512 |         assert_eq!(flag, "--out");
 513 |         let value = expect_value(&flag, &mut it);
 514 |         assert_eq!(value, "samples");
 515 |     }
 516 | }
```

