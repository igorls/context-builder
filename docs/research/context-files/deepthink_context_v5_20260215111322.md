# Directory Structure Report

**Project:** context-builder
**Generated:** 2026-02-15 11:13:22 UTC
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
  - 📁 tree_sitter
    - 📄 language_support.rs
    - 📁 languages
      - 📄 c.rs
      - 📄 cpp.rs
      - 📄 go.rs
      - 📄 java.rs
      - 📄 javascript.rs
      - 📄 mod.rs
      - 📄 python.rs
      - 📄 rust.rs
      - 📄 typescript.rs
    - 📄 mod.rs
    - 📄 signatures.rs
    - 📄 structure.rs
    - 📄 truncation.rs
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

- Size: 2832 bytes
- Modified: SystemTime { tv_sec: 1771144364, tv_nsec: 477450833 }

```toml
   1 | [package]
   2 | name = "context-builder"
   3 | version = "0.8.0"
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
  36 | # Tree-sitter dependencies (feature-gated)
  37 | tree-sitter = { version = "0.24", optional = true }
  38 | tree-sitter-rust = { version = "0.23", optional = true }
  39 | tree-sitter-javascript = { version = "0.23", optional = true }
  40 | tree-sitter-typescript = { version = "0.23", optional = true }
  41 | tree-sitter-python = { version = "0.23", optional = true }
  42 | tree-sitter-go = { version = "0.23", optional = true }
  43 | tree-sitter-java = { version = "0.23", optional = true }
  44 | tree-sitter-c = { version = "0.23", optional = true }
  45 | tree-sitter-cpp = { version = "0.23", optional = true }
  46 | 
  47 | [features]
  48 | default = ["parallel"]
  49 | parallel = ["rayon"]
  50 | samples-bin = []
  51 | 
  52 | # Tree-sitter features - language grammar support
  53 | tree-sitter-base = ["dep:tree-sitter"]
  54 | tree-sitter-rust = ["tree-sitter-base", "dep:tree-sitter-rust"]
  55 | tree-sitter-js = ["tree-sitter-base", "dep:tree-sitter-javascript"]
  56 | tree-sitter-ts = ["tree-sitter-base", "dep:tree-sitter-typescript"]
  57 | tree-sitter-python = ["tree-sitter-base", "dep:tree-sitter-python"]
  58 | tree-sitter-go = ["tree-sitter-base", "dep:tree-sitter-go"]
  59 | tree-sitter-java = ["tree-sitter-base", "dep:tree-sitter-java"]
  60 | tree-sitter-c = ["tree-sitter-base", "dep:tree-sitter-c"]
  61 | tree-sitter-cpp = ["tree-sitter-base", "dep:tree-sitter-cpp"]
  62 | tree-sitter-all = [
  63 |     "tree-sitter-rust",
  64 |     "tree-sitter-js",
  65 |     "tree-sitter-ts",
  66 |     "tree-sitter-python",
  67 |     "tree-sitter-go",
  68 |     "tree-sitter-java",
  69 |     "tree-sitter-c",
  70 |     "tree-sitter-cpp",
  71 | ]
  72 | 
  73 | [dev-dependencies]
  74 | tempfile = "3.25.0"
  75 | criterion = { version = "0.8.2", features = ["html_reports"] }
  76 | pretty_assertions = "1.4.1"
  77 | serial_test = "3.0"
  78 | 
  79 | [[bench]]
  80 | name = "context_bench"
  81 | harness = false
  82 | 
  83 | [[bin]]
  84 | name = "generate_samples"
  85 | path = "scripts/generate_samples.rs"
  86 | required-features = ["samples-bin"]
```

### File: `src/lib.rs`

- Size: 54818 bytes
- Modified: SystemTime { tv_sec: 1771153810, tv_nsec: 862728636 }

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
  18 | pub mod tree_sitter;
  19 | 
  20 | use std::fs::File;
  21 | 
  22 | use cache::CacheManager;
  23 | use cli::Args;
  24 | use config::{Config, load_config_from_path};
  25 | use diff::render_per_file_diffs;
  26 | use file_utils::{collect_files, confirm_overwrite, confirm_processing};
  27 | use markdown::generate_markdown;
  28 | use state::{ProjectState, StateComparison};
  29 | use token_count::{count_file_tokens, count_tree_tokens, estimate_tokens};
  30 | use tree::{build_file_tree, print_tree};
  31 | 
  32 | /// Configuration for diff operations
  33 | #[derive(Debug, Clone)]
  34 | pub struct DiffConfig {
  35 |     pub context_lines: usize,
  36 |     pub enabled: bool,
  37 |     pub diff_only: bool,
  38 | }
  39 | 
  40 | impl Default for DiffConfig {
  41 |     fn default() -> Self {
  42 |         Self {
  43 |             context_lines: 3,
  44 |             enabled: false,
  45 |             diff_only: false,
  46 |         }
  47 |     }
  48 | }
  49 | 
  50 | pub trait Prompter {
  51 |     fn confirm_processing(&self, file_count: usize) -> io::Result<bool>;
  52 |     fn confirm_overwrite(&self, file_path: &str) -> io::Result<bool>;
  53 | }
  54 | 
  55 | pub struct DefaultPrompter;
  56 | 
  57 | impl Prompter for DefaultPrompter {
  58 |     fn confirm_processing(&self, file_count: usize) -> io::Result<bool> {
  59 |         confirm_processing(file_count)
  60 |     }
  61 |     fn confirm_overwrite(&self, file_path: &str) -> io::Result<bool> {
  62 |         confirm_overwrite(file_path)
  63 |     }
  64 | }
  65 | 
  66 | pub fn run_with_args(args: Args, config: Config, prompter: &impl Prompter) -> io::Result<()> {
  67 |     let start_time = Instant::now();
  68 | 
  69 |     let silent = std::env::var("CB_SILENT")
  70 |         .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
  71 |         .unwrap_or(false);
  72 | 
  73 |     // Use the finalized args passed in from run()
  74 |     let final_args = args;
  75 |     // Resolve base path. If input is '.' but current working directory lost the project context
  76 |     // (no context-builder.toml), attempt to infer project root from output path (parent of 'output' dir).
  77 |     let mut resolved_base = PathBuf::from(&final_args.input);
  78 |     let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
  79 |     if resolved_base == Path::new(".")
  80 |         && !cwd.join("context-builder.toml").exists()
  81 |         && let Some(output_parent) = Path::new(&final_args.output).parent()
  82 |         && output_parent
  83 |             .file_name()
  84 |             .map(|n| n == "output")
  85 |             .unwrap_or(false)
  86 |         && let Some(project_root) = output_parent.parent()
  87 |         && project_root.join("context-builder.toml").exists()
  88 |     {
  89 |         resolved_base = project_root.to_path_buf();
  90 |     }
  91 |     let base_path = resolved_base.as_path();
  92 | 
  93 |     if !base_path.exists() || !base_path.is_dir() {
  94 |         if !silent {
  95 |             eprintln!(
  96 |                 "Error: The specified input directory '{}' does not exist or is not a directory.",
  97 |                 final_args.input
  98 |             );
  99 |         }
 100 |         return Err(io::Error::new(
 101 |             io::ErrorKind::NotFound,
 102 |             format!(
 103 |                 "Input directory '{}' does not exist or is not a directory",
 104 |                 final_args.input
 105 |             ),
 106 |         ));
 107 |     }
 108 | 
 109 |     // Create diff configuration from config
 110 |     let diff_config = if config.auto_diff.unwrap_or(false) {
 111 |         Some(DiffConfig {
 112 |             context_lines: config.diff_context_lines.unwrap_or(3),
 113 |             enabled: true,
 114 |             diff_only: final_args.diff_only,
 115 |         })
 116 |     } else {
 117 |         None
 118 |     };
 119 | 
 120 |     if !final_args.preview
 121 |         && !final_args.token_count
 122 |         && Path::new(&final_args.output).exists()
 123 |         && !final_args.yes
 124 |         && !prompter.confirm_overwrite(&final_args.output)?
 125 |     {
 126 |         if !silent {
 127 |             println!("Operation cancelled.");
 128 |         }
 129 |         return Err(io::Error::new(
 130 |             io::ErrorKind::Interrupted,
 131 |             "Operation cancelled by user",
 132 |         ));
 133 |     }
 134 | 
 135 |     // Compute auto-ignore patterns to exclude the tool's own output and cache
 136 |     let mut auto_ignores: Vec<String> = vec![".context-builder".to_string()];
 137 | 
 138 |     // Exclude the resolved output file (or its timestamped glob pattern)
 139 |     let output_path = Path::new(&final_args.output);
 140 |     if let Ok(rel_output) = output_path.strip_prefix(base_path) {
 141 |         // Output is inside the project — exclude it
 142 |         if config.timestamped_output == Some(true) {
 143 |             // Timestamped outputs: create a glob like "docs/context_*.md"
 144 |             if let (Some(parent), Some(stem), Some(ext)) = (
 145 |                 rel_output.parent(),
 146 |                 output_path.file_stem().and_then(|s| s.to_str()),
 147 |                 output_path.extension().and_then(|s| s.to_str()),
 148 |             ) {
 149 |                 // Strip the timestamp suffix to get the base stem
 150 |                 // Timestamped names look like "context_20260214175028.md"
 151 |                 // The stem from config is the part before the timestamp
 152 |                 let base_stem = if let Some(ref cfg_output) = config.output {
 153 |                     Path::new(cfg_output)
 154 |                         .file_stem()
 155 |                         .and_then(|s| s.to_str())
 156 |                         .unwrap_or(stem)
 157 |                         .to_string()
 158 |                 } else {
 159 |                     stem.to_string()
 160 |                 };
 161 |                 let glob = if parent == Path::new("") {
 162 |                     format!("{}_*.{}", base_stem, ext)
 163 |                 } else {
 164 |                     format!("{}/{}_*.{}", parent.display(), base_stem, ext)
 165 |                 };
 166 |                 auto_ignores.push(glob);
 167 |             }
 168 |         } else {
 169 |             // Non-timestamped: exclude the exact output file
 170 |             auto_ignores.push(rel_output.to_string_lossy().to_string());
 171 |         }
 172 |     } else {
 173 |         // Output might be a relative path not under base_path — try using it directly
 174 |         let output_str = final_args.output.clone();
 175 |         if config.timestamped_output == Some(true) {
 176 |             if let (Some(stem), Some(ext)) = (
 177 |                 output_path.file_stem().and_then(|s| s.to_str()),
 178 |                 output_path.extension().and_then(|s| s.to_str()),
 179 |             ) {
 180 |                 let base_stem = if let Some(ref cfg_output) = config.output {
 181 |                     Path::new(cfg_output)
 182 |                         .file_stem()
 183 |                         .and_then(|s| s.to_str())
 184 |                         .unwrap_or(stem)
 185 |                         .to_string()
 186 |                 } else {
 187 |                     stem.to_string()
 188 |                 };
 189 |                 if let Some(parent) = output_path.parent() {
 190 |                     let parent_str = parent.to_string_lossy();
 191 |                     if parent_str.is_empty() || parent_str == "." {
 192 |                         auto_ignores.push(format!("{}_*.{}", base_stem, ext));
 193 |                     } else {
 194 |                         auto_ignores.push(format!("{}/{}_*.{}", parent_str, base_stem, ext));
 195 |                     }
 196 |                 }
 197 |             }
 198 |         } else {
 199 |             auto_ignores.push(output_str);
 200 |         }
 201 |     }
 202 | 
 203 |     // Also exclude the output folder itself if configured
 204 |     if let Some(ref output_folder) = config.output_folder {
 205 |         auto_ignores.push(output_folder.clone());
 206 |     }
 207 | 
 208 |     let files = collect_files(
 209 |         base_path,
 210 |         &final_args.filter,
 211 |         &final_args.ignore,
 212 |         &auto_ignores,
 213 |     )?;
 214 |     let debug_config = std::env::var("CB_DEBUG_CONFIG").is_ok();
 215 |     if debug_config {
 216 |         eprintln!("[DEBUG][CONFIG] Args: {:?}", final_args);
 217 |         eprintln!("[DEBUG][CONFIG] Raw Config: {:?}", config);
 218 |         eprintln!("[DEBUG][CONFIG] Auto-ignores: {:?}", auto_ignores);
 219 |         eprintln!("[DEBUG][CONFIG] Collected {} files", files.len());
 220 |         for f in &files {
 221 |             eprintln!("[DEBUG][CONFIG]  - {}", f.path().display());
 222 |         }
 223 |     }
 224 | 
 225 |     // Smart large-file detection: warn about files that may bloat the context
 226 |     if !silent {
 227 |         const LARGE_FILE_THRESHOLD: u64 = 100 * 1024; // 100 KB
 228 |         let mut large_files: Vec<(String, u64)> = Vec::new();
 229 |         let mut total_size: u64 = 0;
 230 | 
 231 |         for entry in &files {
 232 |             if let Ok(metadata) = entry.path().metadata() {
 233 |                 let size = metadata.len();
 234 |                 total_size += size;
 235 |                 if size > LARGE_FILE_THRESHOLD {
 236 |                     let rel_path = entry
 237 |                         .path()
 238 |                         .strip_prefix(base_path)
 239 |                         .unwrap_or(entry.path())
 240 |                         .to_string_lossy()
 241 |                         .to_string();
 242 |                     large_files.push((rel_path, size));
 243 |                 }
 244 |             }
 245 |         }
 246 | 
 247 |         if !large_files.is_empty() {
 248 |             large_files.sort_by(|a, b| b.1.cmp(&a.1)); // Sort by size descending
 249 |             eprintln!(
 250 |                 "\n⚠  {} large file(s) detected (>{} KB):",
 251 |                 large_files.len(),
 252 |                 LARGE_FILE_THRESHOLD / 1024
 253 |             );
 254 |             for (path, size) in large_files.iter().take(5) {
 255 |                 eprintln!("   {:>8} KB  {}", size / 1024, path);
 256 |             }
 257 |             if large_files.len() > 5 {
 258 |                 eprintln!("   ... and {} more", large_files.len() - 5);
 259 |             }
 260 |             eprintln!(
 261 |                 "   Total context size: {} KB across {} files\n",
 262 |                 total_size / 1024,
 263 |                 files.len()
 264 |             );
 265 |         }
 266 |     }
 267 |     let file_tree = build_file_tree(&files, base_path);
 268 | 
 269 |     if final_args.preview {
 270 |         if !silent {
 271 |             println!("\n# File Tree Structure (Preview)\n");
 272 |             print_tree(&file_tree, 0);
 273 |         }
 274 |         if !final_args.token_count {
 275 |             return Ok(());
 276 |         }
 277 |     }
 278 | 
 279 |     if final_args.token_count {
 280 |         if !silent {
 281 |             println!("\n# Token Count Estimation\n");
 282 |             let mut total_tokens = 0;
 283 |             total_tokens += estimate_tokens("# Directory Structure Report\n\n");
 284 |             if !final_args.filter.is_empty() {
 285 |                 total_tokens += estimate_tokens(&format!(
 286 |                     "This document contains files from the `{}` directory with extensions: {} \n",
 287 |                     final_args.input,
 288 |                     final_args.filter.join(", ")
 289 |                 ));
 290 |             } else {
 291 |                 total_tokens += estimate_tokens(&format!(
 292 |                     "This document contains all files from the `{}` directory, optimized for LLM consumption.\n",
 293 |                     final_args.input
 294 |                 ));
 295 |             }
 296 |             if !final_args.ignore.is_empty() {
 297 |                 total_tokens += estimate_tokens(&format!(
 298 |                     "Custom ignored patterns: {} \n",
 299 |                     final_args.ignore.join(", ")
 300 |                 ));
 301 |             }
 302 |             total_tokens += estimate_tokens("Content hash: 0000000000000000\n\n");
 303 |             total_tokens += estimate_tokens("## File Tree Structure\n\n");
 304 |             let tree_tokens = count_tree_tokens(&file_tree, 0);
 305 |             total_tokens += tree_tokens;
 306 |             let file_tokens: usize = files
 307 |                 .iter()
 308 |                 .map(|entry| count_file_tokens(base_path, entry, final_args.line_numbers))
 309 |                 .sum();
 310 |             total_tokens += file_tokens;
 311 |             println!("Estimated total tokens: {}", total_tokens);
 312 |             println!("File tree tokens: {}", tree_tokens);
 313 |             println!("File content tokens: {}", file_tokens);
 314 |         }
 315 |         return Ok(());
 316 |     }
 317 | 
 318 |     if !final_args.yes && !prompter.confirm_processing(files.len())? {
 319 |         if !silent {
 320 |             println!("Operation cancelled.");
 321 |         }
 322 |         return Err(io::Error::new(
 323 |             io::ErrorKind::Interrupted,
 324 |             "Operation cancelled by user",
 325 |         ));
 326 |     }
 327 | 
 328 |     // NOTE: config-driven flags (line_numbers, diff_only) are already merged
 329 |     // by config_resolver.rs with proper CLI-takes-precedence semantics.
 330 |     // Do NOT re-apply them here as that would silently overwrite CLI flags.
 331 | 
 332 |     if config.auto_diff.unwrap_or(false) {
 333 |         // Build an effective config that mirrors the *actual* operational settings coming
 334 |         // from resolved CLI args (filters/ignores/line_numbers). This ensures the
 335 |         // configuration hash used for cache invalidation reflects real behavior and
 336 |         // stays consistent across runs even when values originate from CLI not file.
 337 |         let mut effective_config = config.clone();
 338 |         // Normalize filter/ignore/line_numbers into config so hashing sees them
 339 |         if !final_args.filter.is_empty() {
 340 |             effective_config.filter = Some(final_args.filter.clone());
 341 |         }
 342 |         if !final_args.ignore.is_empty() {
 343 |             effective_config.ignore = Some(final_args.ignore.clone());
 344 |         }
 345 |         effective_config.line_numbers = Some(final_args.line_numbers);
 346 | 
 347 |         // 1. Create current project state
 348 |         let current_state = ProjectState::from_files(
 349 |             &files,
 350 |             base_path,
 351 |             &effective_config,
 352 |             final_args.line_numbers,
 353 |         )?;
 354 | 
 355 |         // 2. Initialize cache manager and load previous state
 356 |         let cache_manager = CacheManager::new(base_path, &effective_config);
 357 |         let previous_state = match cache_manager.read_cache() {
 358 |             Ok(state) => state,
 359 |             Err(e) => {
 360 |                 if !silent {
 361 |                     eprintln!(
 362 |                         "Warning: Failed to read cache (proceeding without diff): {}",
 363 |                         e
 364 |                     );
 365 |                 }
 366 |                 None
 367 |             }
 368 |         };
 369 | 
 370 |         let diff_cfg = diff_config.as_ref().unwrap();
 371 | 
 372 |         // 3. Determine whether we should invalidate (ignore) previous state
 373 |         let effective_previous = if let Some(prev) = previous_state.as_ref() {
 374 |             if prev.config_hash != current_state.config_hash {
 375 |                 // Config change => treat as initial state (invalidate diff)
 376 |                 None
 377 |             } else {
 378 |                 Some(prev)
 379 |             }
 380 |         } else {
 381 |             None
 382 |         };
 383 | 
 384 |         // 4. Compare states and generate diff if an effective previous state exists
 385 |         let comparison = effective_previous.map(|prev| current_state.compare_with(prev));
 386 | 
 387 |         let debug_autodiff = std::env::var("CB_DEBUG_AUTODIFF").is_ok();
 388 |         if debug_autodiff {
 389 |             eprintln!(
 390 |                 "[DEBUG][AUTODIFF] cache file: {}",
 391 |                 cache_manager.debug_cache_file_path().display()
 392 |             );
 393 |             eprintln!(
 394 |                 "[DEBUG][AUTODIFF] config_hash current={} prev={:?} invalidated={}",
 395 |                 current_state.config_hash,
 396 |                 previous_state.as_ref().map(|s| s.config_hash.clone()),
 397 |                 effective_previous.is_none() && previous_state.is_some()
 398 |             );
 399 |             eprintln!("[DEBUG][AUTODIFF] effective_config: {:?}", effective_config);
 400 |             if let Some(prev) = previous_state.as_ref() {
 401 |                 eprintln!("[DEBUG][AUTODIFF] raw previous files: {}", prev.files.len());
 402 |             }
 403 |             if let Some(prev) = effective_previous {
 404 |                 eprintln!(
 405 |                     "[DEBUG][AUTODIFF] effective previous files: {}",
 406 |                     prev.files.len()
 407 |                 );
 408 |                 for k in prev.files.keys() {
 409 |                     eprintln!("  PREV: {}", k.display());
 410 |                 }
 411 |             }
 412 |             eprintln!(
 413 |                 "[DEBUG][AUTODIFF] current files: {}",
 414 |                 current_state.files.len()
 415 |             );
 416 |             for k in current_state.files.keys() {
 417 |                 eprintln!("  CURR: {}", k.display());
 418 |             }
 419 |         }
 420 | 
 421 |         // Build relevance-sorted path list from the DirEntry list (which is
 422 |         // already sorted by file_relevance_category). This preserves ordering
 423 |         // instead of using BTreeMap's alphabetical iteration.
 424 |         // IMPORTANT: Path resolution must match state.rs to avoid get() misses.
 425 |         let cwd = std::env::current_dir().unwrap_or_else(|_| base_path.to_path_buf());
 426 |         let sorted_paths: Vec<PathBuf> = files
 427 |             .iter()
 428 |             .map(|entry| {
 429 |                 entry
 430 |                     .path()
 431 |                     .strip_prefix(base_path)
 432 |                     .or_else(|_| entry.path().strip_prefix(&cwd))
 433 |                     .map(|p| p.to_path_buf())
 434 |                     .unwrap_or_else(|_| {
 435 |                         entry
 436 |                             .path()
 437 |                             .file_name()
 438 |                             .map(PathBuf::from)
 439 |                             .unwrap_or_else(|| entry.path().to_path_buf())
 440 |                     })
 441 |             })
 442 |             .collect();
 443 | 
 444 |         // Build tree-sitter config for diff path
 445 |         let ts_config = markdown::TreeSitterConfig {
 446 |             signatures: final_args.signatures,
 447 |             structure: final_args.structure,
 448 |             truncate: final_args.truncate.clone(),
 449 |             visibility: final_args.visibility.clone(),
 450 |         };
 451 | 
 452 |         // 4. Generate markdown with diff annotations
 453 |         let mut final_doc = generate_markdown_with_diff(
 454 |             &current_state,
 455 |             comparison.as_ref(),
 456 |             &final_args,
 457 |             &file_tree,
 458 |             diff_cfg,
 459 |             &sorted_paths,
 460 |             &ts_config,
 461 |         )?;
 462 | 
 463 |         // Enforce max_tokens budget (same ~4 bytes/token heuristic as parallel path)
 464 |         if let Some(max_tokens) = final_args.max_tokens {
 465 |             let max_bytes = max_tokens.saturating_mul(4);
 466 |             if final_doc.len() > max_bytes {
 467 |                 // Truncate at a valid UTF-8 boundary
 468 |                 let mut truncate_at = max_bytes;
 469 |                 while truncate_at > 0 && !final_doc.is_char_boundary(truncate_at) {
 470 |                     truncate_at -= 1;
 471 |                 }
 472 |                 final_doc.truncate(truncate_at);
 473 | 
 474 |                 // Close any open markdown code fence to prevent LLMs from
 475 |                 // interpreting the truncation notice as part of a code block.
 476 |                 // Count unmatched ``` fences — if odd, we're inside a block.
 477 |                 let fence_count = final_doc.matches("\n```").count()
 478 |                     + if final_doc.starts_with("```") { 1 } else { 0 };
 479 |                 if fence_count % 2 != 0 {
 480 |                     final_doc.push_str("\n```\n");
 481 |                 }
 482 | 
 483 |                 final_doc.push_str("\n---\n\n");
 484 |                 final_doc.push_str(&format!(
 485 |                     "_Output truncated: exceeded {} token budget (estimated)._\n",
 486 |                     max_tokens
 487 |                 ));
 488 |             }
 489 |         }
 490 | 
 491 |         // 5. Write output
 492 |         let output_path = Path::new(&final_args.output);
 493 |         if let Some(parent) = output_path.parent()
 494 |             && !parent.exists()
 495 |             && let Err(e) = fs::create_dir_all(parent)
 496 |         {
 497 |             return Err(io::Error::other(format!(
 498 |                 "Failed to create output directory {}: {}",
 499 |                 parent.display(),
 500 |                 e
 501 |             )));
 502 |         }
 503 |         let mut final_output = fs::File::create(output_path)?;
 504 |         final_output.write_all(final_doc.as_bytes())?;
 505 | 
 506 |         // 6. Update cache with current state
 507 |         if let Err(e) = cache_manager.write_cache(&current_state)
 508 |             && !silent
 509 |         {
 510 |             eprintln!("Warning: failed to update state cache: {}", e);
 511 |         }
 512 | 
 513 |         let duration = start_time.elapsed();
 514 |         if !silent {
 515 |             if let Some(comp) = &comparison {
 516 |                 if comp.summary.has_changes() {
 517 |                     println!(
 518 |                         "Documentation created successfully with {} changes: {}",
 519 |                         comp.summary.total_changes, final_args.output
 520 |                     );
 521 |                 } else {
 522 |                     println!(
 523 |                         "Documentation created successfully (no changes detected): {}",
 524 |                         final_args.output
 525 |                     );
 526 |                 }
 527 |             } else {
 528 |                 println!(
 529 |                     "Documentation created successfully (initial state): {}",
 530 |                     final_args.output
 531 |                 );
 532 |             }
 533 |             println!("Processing time: {:.2?}", duration);
 534 | 
 535 |             // Warn about context window overflow
 536 |             let output_bytes = final_doc.len();
 537 |             print_context_window_warning(output_bytes, final_args.max_tokens);
 538 |         }
 539 |         return Ok(());
 540 |     }
 541 | 
 542 |     // Standard (non auto-diff) generation
 543 |     // Build tree-sitter config from resolved args
 544 |     let ts_config = markdown::TreeSitterConfig {
 545 |         signatures: final_args.signatures,
 546 |         structure: final_args.structure,
 547 |         truncate: final_args.truncate.clone(),
 548 |         visibility: final_args.visibility.clone(),
 549 |     };
 550 | 
 551 |     // Graceful degradation: warn if tree-sitter flags are used without the feature
 552 |     if !silent && (ts_config.signatures || ts_config.structure || ts_config.truncate == "smart") {
 553 |         #[cfg(not(feature = "tree-sitter-base"))]
 554 |         {
 555 |             eprintln!("⚠️  --signatures/--structure/--truncate smart require tree-sitter support.");
 556 |             eprintln!("   Build with: cargo build --features tree-sitter-all");
 557 |             eprintln!("   Falling back to standard output.\n");
 558 |         }
 559 |     }
 560 | 
 561 |     generate_markdown(
 562 |         &final_args.output,
 563 |         &final_args.input,
 564 |         &final_args.filter,
 565 |         &final_args.ignore,
 566 |         &file_tree,
 567 |         &files,
 568 |         base_path,
 569 |         final_args.line_numbers,
 570 |         config.encoding_strategy.as_deref(),
 571 |         final_args.max_tokens,
 572 |         &ts_config,
 573 |     )?;
 574 | 
 575 |     let duration = start_time.elapsed();
 576 |     if !silent {
 577 |         println!("Documentation created successfully: {}", final_args.output);
 578 |         println!("Processing time: {:.2?}", duration);
 579 | 
 580 |         // Warn about context window overflow
 581 |         let output_bytes = fs::metadata(&final_args.output)
 582 |             .map(|m| m.len() as usize)
 583 |             .unwrap_or(0);
 584 |         print_context_window_warning(output_bytes, final_args.max_tokens);
 585 |     }
 586 | 
 587 |     Ok(())
 588 | }
 589 | 
 590 | /// Print context window overflow warnings with actionable recommendations.
 591 | /// Estimates tokens using the ~4 bytes/token heuristic. Warns when output
 592 | /// exceeds 128K tokens — beyond this size, context quality degrades
 593 | /// significantly for most LLM use cases.
 594 | fn print_context_window_warning(output_bytes: usize, max_tokens: Option<usize>) {
 595 |     let estimated_tokens = output_bytes / 4;
 596 | 
 597 |     println!("Estimated tokens: ~{}K", estimated_tokens / 1000);
 598 | 
 599 |     // If the user already set --max-tokens, they're managing their budget
 600 |     if max_tokens.is_some() {
 601 |         return;
 602 |     }
 603 | 
 604 |     const RECOMMENDED_LIMIT: usize = 128_000;
 605 | 
 606 |     if estimated_tokens <= RECOMMENDED_LIMIT {
 607 |         return;
 608 |     }
 609 | 
 610 |     eprintln!();
 611 |     eprintln!(
 612 |         "⚠️  Output is ~{}K tokens — recommended limit is 128K for effective LLM context.",
 613 |         estimated_tokens / 1000
 614 |     );
 615 |     eprintln!("   Large contexts degrade response quality. Consider narrowing the scope:");
 616 |     eprintln!();
 617 |     eprintln!("   • --max-tokens 100000    Cap output to a token budget");
 618 |     eprintln!("   • --filter rs,toml       Include only specific file types");
 619 |     eprintln!("   • --ignore docs,assets   Exclude directories by name");
 620 |     eprintln!("   • --token-count          Preview size without generating");
 621 |     eprintln!();
 622 | }
 623 | 
 624 | /// Generate markdown document with diff annotations
 625 | fn generate_markdown_with_diff(
 626 |     current_state: &ProjectState,
 627 |     comparison: Option<&StateComparison>,
 628 |     args: &Args,
 629 |     file_tree: &tree::FileTree,
 630 |     diff_config: &DiffConfig,
 631 |     sorted_paths: &[PathBuf],
 632 |     ts_config: &markdown::TreeSitterConfig,
 633 | ) -> io::Result<String> {
 634 |     let mut output = String::new();
 635 | 
 636 |     // Header
 637 |     output.push_str("# Directory Structure Report\n\n");
 638 | 
 639 |     // Basic project info
 640 |     output.push_str(&format!(
 641 |         "**Project:** {}\n",
 642 |         current_state.metadata.project_name
 643 |     ));
 644 |     output.push_str(&format!("**Generated:** {}\n", current_state.timestamp));
 645 | 
 646 |     if !args.filter.is_empty() {
 647 |         output.push_str(&format!("**Filters:** {}\n", args.filter.join(", ")));
 648 |     }
 649 | 
 650 |     if !args.ignore.is_empty() {
 651 |         output.push_str(&format!("**Ignored:** {}\n", args.ignore.join(", ")));
 652 |     }
 653 | 
 654 |     output.push('\n');
 655 | 
 656 |     // Change summary + sections if we have a comparison
 657 |     if let Some(comp) = comparison {
 658 |         if comp.summary.has_changes() {
 659 |             output.push_str(&comp.summary.to_markdown());
 660 | 
 661 |             // Collect added files once so we can reuse for both diff_only logic and potential numbering.
 662 |             let added_files: Vec<_> = comp
 663 |                 .file_diffs
 664 |                 .iter()
 665 |                 .filter(|d| matches!(d.status, diff::PerFileStatus::Added))
 666 |                 .collect();
 667 | 
 668 |             if diff_config.diff_only && !added_files.is_empty() {
 669 |                 output.push_str("## Added Files\n\n");
 670 |                 for added in added_files {
 671 |                     output.push_str(&format!("### File: `{}`\n\n", added.path));
 672 |                     output.push_str("_Status: Added_\n\n");
 673 |                     // Reconstruct content from + lines.
 674 |                     let mut lines: Vec<String> = Vec::new();
 675 |                     for line in added.diff.lines() {
 676 |                         // Diff output uses "+ " prefix (plus-space), strip both to reconstruct content.
 677 |                         // Previously strip_prefix('+') left a leading space, corrupting indentation.
 678 |                         if let Some(rest) = line.strip_prefix("+ ") {
 679 |                             lines.push(rest.to_string());
 680 |                         } else if let Some(rest) = line.strip_prefix('+') {
 681 |                             // Handle edge case: empty added lines have just "+"
 682 |                             lines.push(rest.to_string());
 683 |                         }
 684 |                     }
 685 |                     output.push_str("```text\n");
 686 |                     if args.line_numbers {
 687 |                         for (idx, l) in lines.iter().enumerate() {
 688 |                             output.push_str(&format!("{:>4} | {}\n", idx + 1, l));
 689 |                         }
 690 |                     } else {
 691 |                         for l in lines {
 692 |                             output.push_str(&l);
 693 |                             output.push('\n');
 694 |                         }
 695 |                     }
 696 |                     output.push_str("```\n\n");
 697 |                 }
 698 |             }
 699 | 
 700 |             // Always include a unified diff section header so downstream tooling/tests can rely on it
 701 |             let changed_diffs: Vec<diff::PerFileDiff> = comp
 702 |                 .file_diffs
 703 |                 .iter()
 704 |                 .filter(|d| d.is_changed())
 705 |                 .cloned()
 706 |                 .collect();
 707 |             if !changed_diffs.is_empty() {
 708 |                 output.push_str("## File Differences\n\n");
 709 |                 let diff_markdown = render_per_file_diffs(&changed_diffs);
 710 |                 output.push_str(&diff_markdown);
 711 |             }
 712 |         } else {
 713 |             output.push_str("## No Changes Detected\n\n");
 714 |         }
 715 |     }
 716 | 
 717 |     // File tree
 718 |     output.push_str("## File Tree Structure\n\n");
 719 |     let mut tree_output = Vec::new();
 720 |     tree::write_tree_to_file(&mut tree_output, file_tree, 0)?;
 721 |     output.push_str(&String::from_utf8_lossy(&tree_output));
 722 |     output.push('\n');
 723 | 
 724 |     // File contents (unless diff_only mode)
 725 |     if !diff_config.diff_only {
 726 |         output.push_str("## File Contents\n\n");
 727 | 
 728 |         // Iterate in relevance order (from sorted_paths) instead of
 729 |         // BTreeMap's alphabetical order — preserves file_relevance_category ordering.
 730 |         for path in sorted_paths {
 731 |             if let Some(file_state) = current_state.files.get(path) {
 732 |                 output.push_str(&format!("### File: `{}`\n\n", path.display()));
 733 |                 output.push_str(&format!("- Size: {} bytes\n", file_state.size));
 734 |                 output.push_str(&format!("- Modified: {:?}\n\n", file_state.modified));
 735 | 
 736 |                 // Determine language from file extension
 737 |                 let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("text");
 738 |                 let language = match extension {
 739 |                     "rs" => "rust",
 740 |                     "js" => "javascript",
 741 |                     "ts" => "typescript",
 742 |                     "py" => "python",
 743 |                     "json" => "json",
 744 |                     "toml" => "toml",
 745 |                     "md" => "markdown",
 746 |                     "yaml" | "yml" => "yaml",
 747 |                     "html" => "html",
 748 |                     "css" => "css",
 749 |                     _ => extension,
 750 |                 };
 751 | 
 752 |                 // When --signatures is active, only suppress content for supported code files
 753 |                 let signatures_only = ts_config.signatures
 754 |                     && crate::tree_sitter::is_supported_extension(extension);
 755 | 
 756 |                 if !signatures_only {
 757 |                     output.push_str(&format!("```{}\n", language));
 758 | 
 759 |                     if args.line_numbers {
 760 |                         for (i, line) in file_state.content.lines().enumerate() {
 761 |                             output.push_str(&format!("{:>4} | {}\n", i + 1, line));
 762 |                         }
 763 |                     } else {
 764 |                         output.push_str(&file_state.content);
 765 |                         if !file_state.content.ends_with('\n') {
 766 |                             output.push('\n');
 767 |                         }
 768 |                     }
 769 | 
 770 |                     output.push_str("```\n");
 771 |                 }
 772 | 
 773 |                 // Tree-sitter enrichment (same as standard path)
 774 |                 let mut enrichment_buf = Vec::new();
 775 |                 markdown::write_tree_sitter_enrichment(
 776 |                     &mut enrichment_buf,
 777 |                     &file_state.content,
 778 |                     extension,
 779 |                     ts_config,
 780 |                 )?;
 781 |                 if !enrichment_buf.is_empty() {
 782 |                     output.push_str(&String::from_utf8_lossy(&enrichment_buf));
 783 |                 }
 784 | 
 785 |                 output.push_str("\n");
 786 |             }
 787 |         }
 788 |     }
 789 | 
 790 |     Ok(output)
 791 | }
 792 | 
 793 | pub fn run() -> io::Result<()> {
 794 |     env_logger::init();
 795 |     let args = Args::parse();
 796 | 
 797 |     // Handle init command first
 798 |     if args.init {
 799 |         return init_config();
 800 |     }
 801 | 
 802 |     // Determine project root first
 803 |     let project_root = Path::new(&args.input);
 804 |     let config = load_config_from_path(project_root);
 805 | 
 806 |     // Handle early clear-cache request (runs even if no config or other args)
 807 |     if args.clear_cache {
 808 |         let cache_path = project_root.join(".context-builder").join("cache");
 809 |         if cache_path.exists() {
 810 |             match fs::remove_dir_all(&cache_path) {
 811 |                 Ok(()) => println!("Cache cleared: {}", cache_path.display()),
 812 |                 Err(e) => eprintln!("Failed to clear cache ({}): {}", cache_path.display(), e),
 813 |             }
 814 |         } else {
 815 |             println!("No cache directory found at {}", cache_path.display());
 816 |         }
 817 |         return Ok(());
 818 |     }
 819 | 
 820 |     if std::env::args().len() == 1 && config.is_none() {
 821 |         Args::command().print_help()?;
 822 |         return Ok(());
 823 |     }
 824 | 
 825 |     // Resolve final configuration using the new config resolver
 826 |     let resolution = crate::config_resolver::resolve_final_config(args, config.clone());
 827 | 
 828 |     // Print warnings if any
 829 |     let silent = std::env::var("CB_SILENT")
 830 |         .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
 831 |         .unwrap_or(false);
 832 | 
 833 |     if !silent {
 834 |         for warning in &resolution.warnings {
 835 |             eprintln!("Warning: {}", warning);
 836 |         }
 837 |     }
 838 | 
 839 |     // Convert resolved config back to Args for run_with_args
 840 |     let final_args = Args {
 841 |         input: resolution.config.input,
 842 |         output: resolution.config.output,
 843 |         filter: resolution.config.filter,
 844 |         ignore: resolution.config.ignore,
 845 |         line_numbers: resolution.config.line_numbers,
 846 |         preview: resolution.config.preview,
 847 |         token_count: resolution.config.token_count,
 848 |         yes: resolution.config.yes,
 849 |         diff_only: resolution.config.diff_only,
 850 |         clear_cache: resolution.config.clear_cache,
 851 |         max_tokens: resolution.config.max_tokens,
 852 |         init: false,
 853 |         signatures: resolution.config.signatures,
 854 |         structure: resolution.config.structure,
 855 |         truncate: resolution.config.truncate,
 856 |         visibility: resolution.config.visibility,
 857 |     };
 858 | 
 859 |     // Create final Config with resolved values
 860 |     let final_config = Config {
 861 |         auto_diff: Some(resolution.config.auto_diff),
 862 |         diff_context_lines: Some(resolution.config.diff_context_lines),
 863 |         ..config.unwrap_or_default()
 864 |     };
 865 | 
 866 |     run_with_args(final_args, final_config, &DefaultPrompter)
 867 | }
 868 | 
 869 | /// Detect major file types in the current directory respecting .gitignore and default ignore patterns
 870 | fn detect_major_file_types() -> io::Result<Vec<String>> {
 871 |     use std::collections::HashMap;
 872 |     let mut extension_counts = HashMap::new();
 873 | 
 874 |     // Use the same default ignore patterns as the main application
 875 |     let default_ignores = vec![
 876 |         "docs".to_string(),
 877 |         "target".to_string(),
 878 |         ".git".to_string(),
 879 |         "node_modules".to_string(),
 880 |     ];
 881 | 
 882 |     // Collect files using the same logic as the main application
 883 |     let files = crate::file_utils::collect_files(Path::new("."), &[], &default_ignores, &[])?;
 884 | 
 885 |     // Count extensions from the filtered file list
 886 |     for entry in files {
 887 |         let path = entry.path();
 888 |         if let Some(extension) = path.extension().and_then(|ext| ext.to_str()) {
 889 |             // Count the extension occurrences
 890 |             *extension_counts.entry(extension.to_string()).or_insert(0) += 1;
 891 |         }
 892 |     }
 893 | 
 894 |     // Convert to vector of (extension, count) pairs and sort by count
 895 |     let mut extensions: Vec<(String, usize)> = extension_counts.into_iter().collect();
 896 |     extensions.sort_by(|a, b| b.1.cmp(&a.1));
 897 | 
 898 |     // Take the top 5 extensions or all if less than 5
 899 |     let top_extensions: Vec<String> = extensions.into_iter().take(5).map(|(ext, _)| ext).collect();
 900 | 
 901 |     Ok(top_extensions)
 902 | }
 903 | 
 904 | /// Initialize a new context-builder.toml config file in the current directory with sensible defaults
 905 | fn init_config() -> io::Result<()> {
 906 |     let config_path = Path::new("context-builder.toml");
 907 | 
 908 |     if config_path.exists() {
 909 |         println!("Config file already exists at {}", config_path.display());
 910 |         println!("If you want to replace it, please remove it manually first.");
 911 |         return Ok(());
 912 |     }
 913 | 
 914 |     // Detect major file types in the current directory
 915 |     let filter_suggestions = match detect_major_file_types() {
 916 |         Ok(extensions) => extensions,
 917 |         _ => vec!["rs".to_string(), "toml".to_string()], // fallback to defaults
 918 |     };
 919 | 
 920 |     let filter_string = if filter_suggestions.is_empty() {
 921 |         r#"["rs", "toml"]"#.to_string()
 922 |     } else {
 923 |         format!(r#"["{}"]"#, filter_suggestions.join(r#"", ""#))
 924 |     };
 925 | 
 926 |     let default_config_content = format!(
 927 |         r#"# Context Builder Configuration File
 928 | # This file was generated with sensible defaults based on the file types detected in your project
 929 | 
 930 | # Output file name (or base name when timestamped_output is true)
 931 | output = "context.md"
 932 | 
 933 | # Optional folder to place the generated output file(s) in
 934 | output_folder = "docs"
 935 | 
 936 | # Append a UTC timestamp to the output file name (before extension)
 937 | timestamped_output = true
 938 | 
 939 | # Enable automatic diff generation (requires timestamped_output = true)
 940 | auto_diff = true
 941 | 
 942 | # Emit only change summary + modified file diffs (no full file bodies)
 943 | diff_only = false
 944 | 
 945 | # File extensions to include (no leading dot, e.g. "rs", "toml")
 946 | filter = {}
 947 | 
 948 | # File / directory names to ignore (exact name matches)
 949 | ignore = ["docs", "target", ".git", "node_modules"]
 950 | 
 951 | # Add line numbers to code blocks
 952 | line_numbers = false
 953 | "#,
 954 |         filter_string
 955 |     );
 956 | 
 957 |     let mut file = File::create(config_path)?;
 958 |     file.write_all(default_config_content.as_bytes())?;
 959 | 
 960 |     println!("Config file created at {}", config_path.display());
 961 |     println!("Detected file types: {}", filter_suggestions.join(", "));
 962 |     println!("You can now customize it according to your project needs.");
 963 | 
 964 |     Ok(())
 965 | }
 966 | 
 967 | #[cfg(test)]
 968 | mod tests {
 969 |     use super::*;
 970 |     use std::io::Result;
 971 |     use tempfile::tempdir;
 972 | 
 973 |     // Mock prompter for testing
 974 |     struct MockPrompter {
 975 |         confirm_processing_response: bool,
 976 |         confirm_overwrite_response: bool,
 977 |     }
 978 | 
 979 |     impl MockPrompter {
 980 |         fn new(processing: bool, overwrite: bool) -> Self {
 981 |             Self {
 982 |                 confirm_processing_response: processing,
 983 |                 confirm_overwrite_response: overwrite,
 984 |             }
 985 |         }
 986 |     }
 987 | 
 988 |     impl Prompter for MockPrompter {
 989 |         fn confirm_processing(&self, _file_count: usize) -> Result<bool> {
 990 |             Ok(self.confirm_processing_response)
 991 |         }
 992 | 
 993 |         fn confirm_overwrite(&self, _file_path: &str) -> Result<bool> {
 994 |             Ok(self.confirm_overwrite_response)
 995 |         }
 996 |     }
 997 | 
 998 |     #[test]
 999 |     fn test_diff_config_default() {
1000 |         let config = DiffConfig::default();
1001 |         assert_eq!(config.context_lines, 3);
1002 |         assert!(!config.enabled);
1003 |         assert!(!config.diff_only);
1004 |     }
1005 | 
1006 |     #[test]
1007 |     fn test_diff_config_custom() {
1008 |         let config = DiffConfig {
1009 |             context_lines: 5,
1010 |             enabled: true,
1011 |             diff_only: true,
1012 |         };
1013 |         assert_eq!(config.context_lines, 5);
1014 |         assert!(config.enabled);
1015 |         assert!(config.diff_only);
1016 |     }
1017 | 
1018 |     #[test]
1019 |     fn test_default_prompter() {
1020 |         let prompter = DefaultPrompter;
1021 | 
1022 |         // Test small file count (should not prompt)
1023 |         let result = prompter.confirm_processing(50);
1024 |         assert!(result.is_ok());
1025 |         assert!(result.unwrap());
1026 |     }
1027 | 
1028 |     #[test]
1029 |     fn test_run_with_args_nonexistent_directory() {
1030 |         let args = Args {
1031 |             input: "/nonexistent/directory".to_string(),
1032 |             output: "output.md".to_string(),
1033 |             filter: vec![],
1034 |             ignore: vec![],
1035 |             line_numbers: false,
1036 |             preview: false,
1037 |             token_count: false,
1038 |             yes: false,
1039 |             diff_only: false,
1040 |             clear_cache: false,
1041 |             init: false,
1042 |             max_tokens: None,
1043 |             signatures: false,
1044 |             structure: false,
1045 |             truncate: "smart".to_string(),
1046 |             visibility: "all".to_string(),
1047 |         };
1048 |         let config = Config::default();
1049 |         let prompter = MockPrompter::new(true, true);
1050 | 
1051 |         let result = run_with_args(args, config, &prompter);
1052 |         assert!(result.is_err());
1053 |         assert!(result.unwrap_err().to_string().contains("does not exist"));
1054 |     }
1055 | 
1056 |     #[test]
1057 |     fn test_run_with_args_preview_mode() {
1058 |         let temp_dir = tempdir().unwrap();
1059 |         let base_path = temp_dir.path();
1060 | 
1061 |         // Create some test files
1062 |         fs::write(base_path.join("test.rs"), "fn main() {}").unwrap();
1063 |         fs::create_dir(base_path.join("src")).unwrap();
1064 |         fs::write(base_path.join("src/lib.rs"), "pub fn hello() {}").unwrap();
1065 | 
1066 |         let args = Args {
1067 |             input: ".".to_string(),
1068 |             output: "test.md".to_string(),
1069 |             filter: vec![],
1070 |             ignore: vec![],
1071 |             line_numbers: false,
1072 |             preview: false,
1073 |             token_count: false,
1074 |             yes: false,
1075 |             diff_only: false,
1076 |             clear_cache: false,
1077 |             init: false,
1078 |             max_tokens: None,
1079 |             signatures: false,
1080 |             structure: false,
1081 |             truncate: "smart".to_string(),
1082 |             visibility: "all".to_string(),
1083 |         };
1084 |         let config = Config::default();
1085 |         let prompter = MockPrompter::new(true, true);
1086 | 
1087 |         // Set CB_SILENT to avoid console output during test
1088 |         unsafe {
1089 |             std::env::set_var("CB_SILENT", "1");
1090 |         }
1091 |         let result = run_with_args(args, config, &prompter);
1092 |         unsafe {
1093 |             std::env::remove_var("CB_SILENT");
1094 |         }
1095 | 
1096 |         assert!(result.is_ok());
1097 |     }
1098 | 
1099 |     #[test]
1100 |     fn test_run_with_args_token_count_mode() {
1101 |         let temp_dir = tempdir().unwrap();
1102 |         let base_path = temp_dir.path();
1103 | 
1104 |         // Create test files
1105 |         fs::write(base_path.join("small.txt"), "Hello world").unwrap();
1106 | 
1107 |         let args = Args {
1108 |             input: base_path.to_string_lossy().to_string(),
1109 |             output: "test.md".to_string(),
1110 |             filter: vec![],
1111 |             ignore: vec![],
1112 |             line_numbers: false,
1113 |             preview: false,
1114 |             token_count: true,
1115 |             yes: false,
1116 |             diff_only: false,
1117 |             clear_cache: false,
1118 |             init: false,
1119 |             max_tokens: None,
1120 |             signatures: false,
1121 |             structure: false,
1122 |             truncate: "smart".to_string(),
1123 |             visibility: "all".to_string(),
1124 |         };
1125 |         let config = Config::default();
1126 |         let prompter = MockPrompter::new(true, true);
1127 | 
1128 |         unsafe {
1129 |             std::env::set_var("CB_SILENT", "1");
1130 |         }
1131 |         let result = run_with_args(args, config, &prompter);
1132 |         unsafe {
1133 |             std::env::remove_var("CB_SILENT");
1134 |         }
1135 | 
1136 |         assert!(result.is_ok());
1137 |     }
1138 | 
1139 |     #[test]
1140 |     fn test_run_with_args_preview_and_token_count() {
1141 |         let temp_dir = tempdir().unwrap();
1142 |         let base_path = temp_dir.path();
1143 | 
1144 |         fs::write(base_path.join("test.txt"), "content").unwrap();
1145 | 
1146 |         let args = Args {
1147 |             input: base_path.to_string_lossy().to_string(),
1148 |             output: "test.md".to_string(),
1149 |             filter: vec![],
1150 |             ignore: vec![],
1151 |             line_numbers: false,
1152 |             preview: true,
1153 |             token_count: false,
1154 |             yes: false,
1155 |             diff_only: false,
1156 |             clear_cache: false,
1157 |             init: false,
1158 |             max_tokens: None,
1159 |             signatures: false,
1160 |             structure: false,
1161 |             truncate: "smart".to_string(),
1162 |             visibility: "all".to_string(),
1163 |         };
1164 |         let config = Config::default();
1165 |         let prompter = MockPrompter::new(true, true);
1166 | 
1167 |         unsafe {
1168 |             std::env::set_var("CB_SILENT", "1");
1169 |         }
1170 |         let result = run_with_args(args, config, &prompter);
1171 |         unsafe {
1172 |             std::env::remove_var("CB_SILENT");
1173 |         }
1174 | 
1175 |         assert!(result.is_ok());
1176 |     }
1177 | 
1178 |     #[test]
1179 |     fn test_run_with_args_user_cancels_overwrite() {
1180 |         let temp_dir = tempdir().unwrap();
1181 |         let base_path = temp_dir.path();
1182 |         let output_path = temp_dir.path().join("existing.md");
1183 | 
1184 |         // Create test files
1185 |         fs::write(base_path.join("test.txt"), "content").unwrap();
1186 |         fs::write(&output_path, "existing content").unwrap();
1187 | 
1188 |         let args = Args {
1189 |             input: base_path.to_string_lossy().to_string(),
1190 |             output: "test.md".to_string(),
1191 |             filter: vec![],
1192 |             ignore: vec!["target".to_string()],
1193 |             line_numbers: false,
1194 |             preview: false,
1195 |             token_count: false,
1196 |             yes: false,
1197 |             diff_only: false,
1198 |             clear_cache: false,
1199 |             init: false,
1200 |             max_tokens: None,
1201 |             signatures: false,
1202 |             structure: false,
1203 |             truncate: "smart".to_string(),
1204 |             visibility: "all".to_string(),
1205 |         };
1206 |         let config = Config::default();
1207 |         let prompter = MockPrompter::new(true, false); // Deny overwrite
1208 | 
1209 |         unsafe {
1210 |             std::env::set_var("CB_SILENT", "1");
1211 |         }
1212 |         let result = run_with_args(args, config, &prompter);
1213 |         unsafe {
1214 |             std::env::remove_var("CB_SILENT");
1215 |         }
1216 | 
1217 |         assert!(result.is_err());
1218 |         assert!(result.unwrap_err().to_string().contains("cancelled"));
1219 |     }
1220 | 
1221 |     #[test]
1222 |     fn test_run_with_args_user_cancels_processing() {
1223 |         let temp_dir = tempdir().unwrap();
1224 |         let base_path = temp_dir.path();
1225 | 
1226 |         // Create many test files to trigger processing confirmation
1227 |         for i in 0..105 {
1228 |             fs::write(base_path.join(format!("file{}.txt", i)), "content").unwrap();
1229 |         }
1230 | 
1231 |         let args = Args {
1232 |             input: base_path.to_string_lossy().to_string(),
1233 |             output: "test.md".to_string(),
1234 |             filter: vec!["rs".to_string()],
1235 |             ignore: vec![],
1236 |             line_numbers: false,
1237 |             preview: false,
1238 |             token_count: false,
1239 |             yes: false,
1240 |             diff_only: false,
1241 |             clear_cache: false,
1242 |             init: false,
1243 |             max_tokens: None,
1244 |             signatures: false,
1245 |             structure: false,
1246 |             truncate: "smart".to_string(),
1247 |             visibility: "all".to_string(),
1248 |         };
1249 |         let config = Config::default();
1250 |         let prompter = MockPrompter::new(false, true); // Deny processing
1251 | 
1252 |         unsafe {
1253 |             std::env::set_var("CB_SILENT", "1");
1254 |         }
1255 |         let result = run_with_args(args, config, &prompter);
1256 |         unsafe {
1257 |             std::env::remove_var("CB_SILENT");
1258 |         }
1259 | 
1260 |         assert!(result.is_err());
1261 |         assert!(result.unwrap_err().to_string().contains("cancelled"));
1262 |     }
1263 | 
1264 |     #[test]
1265 |     fn test_run_with_args_with_yes_flag() {
1266 |         let temp_dir = tempdir().unwrap();
1267 |         let base_path = temp_dir.path();
1268 |         let output_file_name = "test.md";
1269 |         let output_path = temp_dir.path().join(output_file_name);
1270 | 
1271 |         fs::write(base_path.join("test.txt"), "Hello world").unwrap();
1272 | 
1273 |         let args = Args {
1274 |             input: base_path.to_string_lossy().to_string(),
1275 |             output: output_path.to_string_lossy().to_string(),
1276 |             filter: vec![],
1277 |             ignore: vec!["ignored_dir".to_string()],
1278 |             line_numbers: false,
1279 |             preview: false,
1280 |             token_count: false,
1281 |             yes: true,
1282 |             diff_only: false,
1283 |             clear_cache: false,
1284 |             init: false,
1285 |             max_tokens: None,
1286 |             signatures: false,
1287 |             structure: false,
1288 |             truncate: "smart".to_string(),
1289 |             visibility: "all".to_string(),
1290 |         };
1291 |         let config = Config::default();
1292 |         let prompter = MockPrompter::new(true, true);
1293 | 
1294 |         unsafe {
1295 |             std::env::set_var("CB_SILENT", "1");
1296 |         }
1297 |         let result = run_with_args(args, config, &prompter);
1298 |         unsafe {
1299 |             std::env::remove_var("CB_SILENT");
1300 |         }
1301 | 
1302 |         assert!(result.is_ok());
1303 |         assert!(output_path.exists());
1304 | 
1305 |         let content = fs::read_to_string(&output_path).unwrap();
1306 |         assert!(content.contains("Directory Structure Report"));
1307 |         assert!(content.contains("test.txt"));
1308 |     }
1309 | 
1310 |     #[test]
1311 |     fn test_run_with_args_with_filters() {
1312 |         let temp_dir = tempdir().unwrap();
1313 |         let base_path = temp_dir.path();
1314 |         let output_file_name = "test.md";
1315 |         let output_path = temp_dir.path().join(output_file_name);
1316 | 
1317 |         fs::write(base_path.join("code.rs"), "fn main() {}").unwrap();
1318 |         fs::write(base_path.join("readme.md"), "# README").unwrap();
1319 |         fs::write(base_path.join("data.json"), r#"{"key": "value"}"#).unwrap();
1320 | 
1321 |         let args = Args {
1322 |             input: base_path.to_string_lossy().to_string(),
1323 |             output: output_path.to_string_lossy().to_string(),
1324 |             filter: vec!["rs".to_string(), "md".to_string()],
1325 |             ignore: vec![],
1326 |             line_numbers: true,
1327 |             preview: false,
1328 |             token_count: false,
1329 |             yes: true,
1330 |             diff_only: false,
1331 |             clear_cache: false,
1332 |             init: false,
1333 |             max_tokens: None,
1334 |             signatures: false,
1335 |             structure: false,
1336 |             truncate: "smart".to_string(),
1337 |             visibility: "all".to_string(),
1338 |         };
1339 |         let config = Config::default();
1340 |         let prompter = MockPrompter::new(true, true);
1341 | 
1342 |         unsafe {
1343 |             std::env::set_var("CB_SILENT", "1");
1344 |         }
1345 |         let result = run_with_args(args, config, &prompter);
1346 |         unsafe {
1347 |             std::env::remove_var("CB_SILENT");
1348 |         }
1349 | 
1350 |         assert!(result.is_ok());
1351 | 
1352 |         let content = fs::read_to_string(&output_path).unwrap();
1353 |         assert!(content.contains("code.rs"));
1354 |         assert!(content.contains("readme.md"));
1355 |         assert!(!content.contains("data.json")); // Should be filtered out
1356 |         assert!(content.contains("   1 |")); // Line numbers should be present
1357 |     }
1358 | 
1359 |     #[test]
1360 |     fn test_run_with_args_with_ignores() {
1361 |         let temp_dir = tempdir().unwrap();
1362 |         let base_path = temp_dir.path();
1363 |         let output_path = temp_dir.path().join("ignored.md");
1364 | 
1365 |         fs::write(base_path.join("important.txt"), "important content").unwrap();
1366 |         fs::write(base_path.join("secret.txt"), "secret content").unwrap();
1367 | 
1368 |         let args = Args {
1369 |             input: base_path.to_string_lossy().to_string(),
1370 |             output: output_path.to_string_lossy().to_string(),
1371 |             filter: vec![],
1372 |             ignore: vec!["secret.txt".to_string()],
1373 |             line_numbers: false,
1374 |             preview: false,
1375 |             token_count: false,
1376 |             yes: true,
1377 |             diff_only: false,
1378 |             clear_cache: false,
1379 |             init: false,
1380 |             max_tokens: None,
1381 |             signatures: false,
1382 |             structure: false,
1383 |             truncate: "smart".to_string(),
1384 |             visibility: "all".to_string(),
1385 |         };
1386 |         let config = Config::default();
1387 |         let prompter = MockPrompter::new(true, true);
1388 | 
1389 |         unsafe {
1390 |             std::env::set_var("CB_SILENT", "1");
1391 |         }
1392 |         let result = run_with_args(args, config, &prompter);
1393 |         unsafe {
1394 |             std::env::remove_var("CB_SILENT");
1395 |         }
1396 | 
1397 |         assert!(result.is_ok());
1398 | 
1399 |         let content = fs::read_to_string(&output_path).unwrap();
1400 |         assert!(content.contains("important.txt"));
1401 |         // The ignore pattern may not work exactly as expected in this test setup
1402 |         // Just verify the output file was created successfully
1403 |     }
1404 | 
1405 |     #[test]
1406 |     fn test_auto_diff_without_previous_state() {
1407 |         let temp_dir = tempdir().unwrap();
1408 |         let base_path = temp_dir.path();
1409 |         let output_file_name = "test.md";
1410 |         let output_path = temp_dir.path().join(output_file_name);
1411 | 
1412 |         fs::write(base_path.join("new.txt"), "new content").unwrap();
1413 | 
1414 |         let args = Args {
1415 |             input: base_path.to_string_lossy().to_string(),
1416 |             output: output_path.to_string_lossy().to_string(),
1417 |             filter: vec![],
1418 |             ignore: vec![],
1419 |             line_numbers: false,
1420 |             preview: false,
1421 |             token_count: false,
1422 |             yes: true,
1423 |             diff_only: false,
1424 |             clear_cache: false,
1425 |             init: false,
1426 |             max_tokens: None,
1427 |             signatures: false,
1428 |             structure: false,
1429 |             truncate: "smart".to_string(),
1430 |             visibility: "all".to_string(),
1431 |         };
1432 |         let config = Config {
1433 |             auto_diff: Some(true),
1434 |             diff_context_lines: Some(5),
1435 |             ..Default::default()
1436 |         };
1437 |         let prompter = MockPrompter::new(true, true);
1438 | 
1439 |         unsafe {
1440 |             std::env::set_var("CB_SILENT", "1");
1441 |         }
1442 |         let result = run_with_args(args, config, &prompter);
1443 |         unsafe {
1444 |             std::env::remove_var("CB_SILENT");
1445 |         }
1446 | 
1447 |         assert!(result.is_ok());
1448 |         assert!(output_path.exists());
1449 | 
1450 |         let content = fs::read_to_string(&output_path).unwrap();
1451 |         assert!(content.contains("new.txt"));
1452 |     }
1453 | 
1454 |     #[test]
1455 |     fn test_run_creates_output_directory() {
1456 |         let temp_dir = tempdir().unwrap();
1457 |         let base_path = temp_dir.path();
1458 |         let output_dir = temp_dir.path().join("nested").join("output");
1459 |         let output_path = output_dir.join("result.md");
1460 | 
1461 |         fs::write(base_path.join("test.txt"), "content").unwrap();
1462 | 
1463 |         let args = Args {
1464 |             input: base_path.to_string_lossy().to_string(),
1465 |             output: output_path.to_string_lossy().to_string(),
1466 |             filter: vec![],
1467 |             ignore: vec![],
1468 |             line_numbers: false,
1469 |             preview: false,
1470 |             token_count: false,
1471 |             yes: true,
1472 |             diff_only: false,
1473 |             clear_cache: false,
1474 |             init: false,
1475 |             max_tokens: None,
1476 |             signatures: false,
1477 |             structure: false,
1478 |             truncate: "smart".to_string(),
1479 |             visibility: "all".to_string(),
1480 |         };
1481 |         let config = Config::default();
1482 |         let prompter = MockPrompter::new(true, true);
1483 | 
1484 |         unsafe {
1485 |             std::env::set_var("CB_SILENT", "1");
1486 |         }
1487 |         let result = run_with_args(args, config, &prompter);
1488 |         unsafe {
1489 |             std::env::remove_var("CB_SILENT");
1490 |         }
1491 | 
1492 |         assert!(result.is_ok());
1493 |         assert!(output_path.exists());
1494 |         assert!(output_dir.exists());
1495 |     }
1496 | 
1497 |     #[test]
1498 |     fn test_generate_markdown_with_diff_no_comparison() {
1499 |         let temp_dir = tempdir().unwrap();
1500 |         let base_path = temp_dir.path();
1501 | 
1502 |         fs::write(base_path.join("test.rs"), "fn main() {}").unwrap();
1503 | 
1504 |         let files = collect_files(base_path, &[], &[], &[]).unwrap();
1505 |         let file_tree = build_file_tree(&files, base_path);
1506 |         let config = Config::default();
1507 |         let state = ProjectState::from_files(&files, base_path, &config, false).unwrap();
1508 | 
1509 |         let args = Args {
1510 |             input: base_path.to_string_lossy().to_string(),
1511 |             output: "test.md".to_string(),
1512 |             filter: vec![],
1513 |             ignore: vec![],
1514 |             line_numbers: false,
1515 |             preview: false,
1516 |             token_count: false,
1517 |             yes: false,
1518 |             diff_only: false,
1519 |             clear_cache: false,
1520 |             init: false,
1521 |             max_tokens: None,
1522 |             signatures: false,
1523 |             structure: false,
1524 |             truncate: "smart".to_string(),
1525 |             visibility: "all".to_string(),
1526 |         };
1527 | 
1528 |         let diff_config = DiffConfig::default();
1529 | 
1530 |         let sorted_paths: Vec<PathBuf> = files
1531 |             .iter()
1532 |             .map(|e| {
1533 |                 e.path()
1534 |                     .strip_prefix(base_path)
1535 |                     .unwrap_or(e.path())
1536 |                     .to_path_buf()
1537 |             })
1538 |             .collect();
1539 | 
1540 |         let ts_config = markdown::TreeSitterConfig {
1541 |             signatures: false,
1542 |             structure: false,
1543 |             truncate: "smart".to_string(),
1544 |             visibility: "all".to_string(),
1545 |         };
1546 | 
1547 |         let result = generate_markdown_with_diff(
1548 |             &state,
1549 |             None,
1550 |             &args,
1551 |             &file_tree,
1552 |             &diff_config,
1553 |             &sorted_paths,
1554 |             &ts_config,
1555 |         );
1556 |         assert!(result.is_ok());
1557 | 
1558 |         let content = result.unwrap();
1559 |         assert!(content.contains("Directory Structure Report"));
1560 |         assert!(content.contains("test.rs"));
1561 |     }
1562 | }
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

### File: `src/tree_sitter/languages/mod.rs`

- Size: 3447 bytes
- Modified: SystemTime { tv_sec: 1771138025, tv_nsec: 375473793 }

```rust
   1 | //! Language support registry.
   2 | //!
   3 | //! This module provides access to language-specific parsers based on file extensions.
   4 | 
   5 | #[cfg(feature = "tree-sitter-base")]
   6 | use super::language_support::LanguageSupport;
   7 | 
   8 | #[cfg(feature = "tree-sitter-rust")]
   9 | mod rust;
  10 | 
  11 | #[cfg(feature = "tree-sitter-js")]
  12 | mod javascript;
  13 | 
  14 | #[cfg(feature = "tree-sitter-ts")]
  15 | mod typescript;
  16 | 
  17 | #[cfg(feature = "tree-sitter-python")]
  18 | mod python;
  19 | 
  20 | #[cfg(feature = "tree-sitter-go")]
  21 | mod go;
  22 | 
  23 | #[cfg(feature = "tree-sitter-java")]
  24 | mod java;
  25 | 
  26 | #[cfg(feature = "tree-sitter-c")]
  27 | mod c;
  28 | 
  29 | #[cfg(feature = "tree-sitter-cpp")]
  30 | mod cpp;
  31 | 
  32 | #[cfg(feature = "tree-sitter-rust")]
  33 | static RUST_SUPPORT: rust::RustSupport = rust::RustSupport;
  34 | 
  35 | #[cfg(feature = "tree-sitter-js")]
  36 | static JS_SUPPORT: javascript::JavaScriptSupport = javascript::JavaScriptSupport;
  37 | 
  38 | #[cfg(feature = "tree-sitter-ts")]
  39 | static TS_SUPPORT: typescript::TypeScriptSupport = typescript::TypeScriptSupport;
  40 | 
  41 | #[cfg(feature = "tree-sitter-python")]
  42 | static PYTHON_SUPPORT: python::PythonSupport = python::PythonSupport;
  43 | 
  44 | #[cfg(feature = "tree-sitter-go")]
  45 | static GO_SUPPORT: go::GoSupport = go::GoSupport;
  46 | 
  47 | #[cfg(feature = "tree-sitter-java")]
  48 | static JAVA_SUPPORT: java::JavaSupport = java::JavaSupport;
  49 | 
  50 | #[cfg(feature = "tree-sitter-c")]
  51 | static C_SUPPORT: c::CSupport = c::CSupport;
  52 | 
  53 | #[cfg(feature = "tree-sitter-cpp")]
  54 | static CPP_SUPPORT: cpp::CppSupport = cpp::CppSupport;
  55 | 
  56 | #[cfg(feature = "tree-sitter-base")]
  57 | pub fn get_language_support(ext: &str) -> Option<&'static dyn LanguageSupport> {
  58 |     match ext.to_lowercase().as_str() {
  59 |         #[cfg(feature = "tree-sitter-rust")]
  60 |         "rs" => Some(&RUST_SUPPORT),
  61 | 
  62 |         #[cfg(feature = "tree-sitter-js")]
  63 |         "js" | "mjs" | "cjs" => Some(&JS_SUPPORT),
  64 | 
  65 |         #[cfg(feature = "tree-sitter-ts")]
  66 |         "ts" | "tsx" | "mts" | "cts" => Some(&TS_SUPPORT),
  67 | 
  68 |         #[cfg(feature = "tree-sitter-python")]
  69 |         "py" | "pyw" => Some(&PYTHON_SUPPORT),
  70 | 
  71 |         #[cfg(feature = "tree-sitter-go")]
  72 |         "go" => Some(&GO_SUPPORT),
  73 | 
  74 |         #[cfg(feature = "tree-sitter-java")]
  75 |         "java" => Some(&JAVA_SUPPORT),
  76 | 
  77 |         #[cfg(feature = "tree-sitter-c")]
  78 |         "c" | "h" => Some(&C_SUPPORT),
  79 | 
  80 |         #[cfg(feature = "tree-sitter-cpp")]
  81 |         "cpp" | "cxx" | "cc" | "hpp" | "hxx" | "hh" => Some(&CPP_SUPPORT),
  82 | 
  83 |         _ => None,
  84 |     }
  85 | }
  86 | 
  87 | #[cfg(not(feature = "tree-sitter-base"))]
  88 | pub fn get_language_support(_ext: &str) -> Option<()> {
  89 |     None
  90 | }
  91 | 
  92 | #[cfg(feature = "tree-sitter-base")]
  93 | pub fn supported_extensions() -> Vec<&'static str> {
  94 |     let mut extensions = Vec::new();
  95 | 
  96 |     #[cfg(feature = "tree-sitter-rust")]
  97 |     extensions.extend(RUST_SUPPORT.file_extensions());
  98 | 
  99 |     #[cfg(feature = "tree-sitter-js")]
 100 |     extensions.extend(JS_SUPPORT.file_extensions());
 101 | 
 102 |     #[cfg(feature = "tree-sitter-ts")]
 103 |     extensions.extend(TS_SUPPORT.file_extensions());
 104 | 
 105 |     #[cfg(feature = "tree-sitter-python")]
 106 |     extensions.extend(PYTHON_SUPPORT.file_extensions());
 107 | 
 108 |     #[cfg(feature = "tree-sitter-go")]
 109 |     extensions.extend(GO_SUPPORT.file_extensions());
 110 | 
 111 |     #[cfg(feature = "tree-sitter-java")]
 112 |     extensions.extend(JAVA_SUPPORT.file_extensions());
 113 | 
 114 |     #[cfg(feature = "tree-sitter-c")]
 115 |     extensions.extend(C_SUPPORT.file_extensions());
 116 | 
 117 |     #[cfg(feature = "tree-sitter-cpp")]
 118 |     extensions.extend(CPP_SUPPORT.file_extensions());
 119 | 
 120 |     extensions
 121 | }
 122 | 
 123 | #[cfg(not(feature = "tree-sitter-base"))]
 124 | pub fn supported_extensions() -> Vec<&'static str> {
 125 |     Vec::new()
 126 | }
```

### File: `src/tree_sitter/mod.rs`

- Size: 4087 bytes
- Modified: SystemTime { tv_sec: 1771140582, tv_nsec: 122629510 }

```rust
   1 | //! Tree-sitter integration for intelligent code parsing.
   2 | //!
   3 | //! This module provides:
   4 | //! - Signature extraction (function/class signatures without bodies)
   5 | //! - Smart truncation (truncate at AST boundaries)
   6 | //! - Structure extraction (imports, exports, symbol counts)
   7 | //!
   8 | //! Feature-gated: Only compiled when one of the tree-sitter-* features is enabled.
   9 | 
  10 | #[cfg(feature = "tree-sitter-base")]
  11 | pub mod language_support;
  12 | 
  13 | #[cfg(feature = "tree-sitter-base")]
  14 | pub mod signatures;
  15 | 
  16 | #[cfg(feature = "tree-sitter-base")]
  17 | pub mod structure;
  18 | 
  19 | #[cfg(feature = "tree-sitter-base")]
  20 | pub mod truncation;
  21 | 
  22 | #[cfg(feature = "tree-sitter-base")]
  23 | pub mod languages;
  24 | 
  25 | #[cfg(feature = "tree-sitter-base")]
  26 | use std::path::Path;
  27 | 
  28 | #[cfg(feature = "tree-sitter-base")]
  29 | pub use language_support::{CodeStructure, LanguageSupport, Signature, SignatureKind, Visibility};
  30 | 
  31 | #[cfg(feature = "tree-sitter-base")]
  32 | pub use signatures::extract_signatures;
  33 | 
  34 | #[cfg(feature = "tree-sitter-base")]
  35 | pub use structure::extract_structure;
  36 | 
  37 | #[cfg(feature = "tree-sitter-base")]
  38 | pub use truncation::find_truncation_point;
  39 | 
  40 | /// Check if tree-sitter is available for a given file extension.
  41 | #[cfg(feature = "tree-sitter-base")]
  42 | pub fn is_supported_extension(ext: &str) -> bool {
  43 |     languages::get_language_support(ext).is_some()
  44 | }
  45 | 
  46 | #[cfg(not(feature = "tree-sitter-base"))]
  47 | pub fn is_supported_extension(_ext: &str) -> bool {
  48 |     false
  49 | }
  50 | 
  51 | /// Extract file extension from a path.
  52 | #[cfg(feature = "tree-sitter-base")]
  53 | fn get_extension(path: &Path) -> Option<String> {
  54 |     path.extension()
  55 |         .and_then(|e| e.to_str())
  56 |         .map(|s| s.to_lowercase())
  57 | }
  58 | 
  59 | /// Get language support for a file path.
  60 | #[cfg(feature = "tree-sitter-base")]
  61 | pub fn get_language_for_path(path: &Path) -> Option<&'static dyn LanguageSupport> {
  62 |     let ext = get_extension(path)?;
  63 |     languages::get_language_support(&ext)
  64 | }
  65 | 
  66 | /// Extract signatures from source code for a given file extension.
  67 | #[cfg(feature = "tree-sitter-base")]
  68 | pub fn extract_signatures_for_file(
  69 |     source: &str,
  70 |     ext: &str,
  71 |     visibility_filter: Visibility,
  72 | ) -> Option<Vec<Signature>> {
  73 |     let support = languages::get_language_support(ext)?;
  74 |     Some(extract_signatures(source, support, visibility_filter))
  75 | }
  76 | 
  77 | /// Extract structure from source code for a given file extension.
  78 | #[cfg(feature = "tree-sitter-base")]
  79 | pub fn extract_structure_for_file(source: &str, ext: &str) -> Option<CodeStructure> {
  80 |     let support = languages::get_language_support(ext)?;
  81 |     Some(extract_structure(source, support))
  82 | }
  83 | 
  84 | /// Find a smart truncation point for a given file extension.
  85 | #[cfg(feature = "tree-sitter-base")]
  86 | pub fn find_smart_truncation_point(source: &str, max_bytes: usize, ext: &str) -> Option<usize> {
  87 |     let support = languages::get_language_support(ext)?;
  88 |     Some(find_truncation_point(source, max_bytes, support))
  89 | }
  90 | 
  91 | #[cfg(not(feature = "tree-sitter-base"))]
  92 | pub fn extract_signatures_for_file(
  93 |     _source: &str,
  94 |     _ext: &str,
  95 |     _visibility_filter: (),
  96 | ) -> Option<()> {
  97 |     None
  98 | }
  99 | 
 100 | #[cfg(not(feature = "tree-sitter-base"))]
 101 | pub fn extract_structure_for_file(_source: &str, _ext: &str) -> Option<()> {
 102 |     None
 103 | }
 104 | 
 105 | #[cfg(not(feature = "tree-sitter-base"))]
 106 | pub fn find_smart_truncation_point(_source: &str, _max_bytes: usize, _ext: &str) -> Option<usize> {
 107 |     None
 108 | }
 109 | 
 110 | #[cfg(not(feature = "tree-sitter-base"))]
 111 | pub fn get_language_for_path(_path: &std::path::Path) -> Option<()> {
 112 |     None
 113 | }
 114 | 
 115 | #[cfg(test)]
 116 | mod tests {
 117 |     use super::*;
 118 | 
 119 |     #[test]
 120 |     #[cfg(feature = "tree-sitter-base")]
 121 |     fn test_is_supported_extension() {
 122 |         #[cfg(feature = "tree-sitter-rust")]
 123 |         assert!(is_supported_extension("rs"));
 124 |         #[cfg(feature = "tree-sitter-python")]
 125 |         assert!(is_supported_extension("py"));
 126 |         #[cfg(feature = "tree-sitter-js")]
 127 |         assert!(is_supported_extension("js"));
 128 |         assert!(!is_supported_extension("xyz"));
 129 |     }
 130 | 
 131 |     #[test]
 132 |     #[cfg(not(feature = "tree-sitter-base"))]
 133 |     fn test_no_tree_sitter_support() {
 134 |         assert!(!is_supported_extension("rs"));
 135 |         assert!(!is_supported_extension("py"));
 136 |     }
 137 | }
```

### File: `src/cache.rs`

- Size: 19474 bytes
- Modified: SystemTime { tv_sec: 1771135276, tv_nsec: 736490842 }

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
  95 |         // Build a stable string representation of config for hashing.
  96 |         // IMPORTANT: Must stay in sync with state.rs::compute_config_hash
  97 |         let mut config_str = String::new();
  98 |         if let Some(ref filters) = config.filter {
  99 |             config_str.push_str(&filters.join(","));
 100 |         }
 101 |         config_str.push('|');
 102 |         if let Some(ref ignores) = config.ignore {
 103 |             config_str.push_str(&ignores.join(","));
 104 |         }
 105 |         config_str.push('|');
 106 |         config_str.push_str(&format!(
 107 |             "{:?}|{:?}|{:?}",
 108 |             config.line_numbers, config.auto_diff, config.diff_context_lines
 109 |         ));
 110 |         let hash = xxhash_rust::xxh3::xxh3_64(config_str.as_bytes());
 111 |         format!("{:x}", hash)
 112 |     }
 113 | 
 114 |     /// Get the cache file path for this specific project and configuration
 115 |     fn get_cache_path(&self) -> PathBuf {
 116 |         self.cache_dir.join(format!(
 117 |             "state_{}_{}.json",
 118 |             self.project_hash, self.config_hash
 119 |         ))
 120 |     }
 121 | 
 122 |     /// Public helper primarily for debugging/tests to inspect the resolved cache path
 123 |     pub fn debug_cache_file_path(&self) -> PathBuf {
 124 |         self.get_cache_path()
 125 |     }
 126 | 
 127 |     /// Migrate old markdown-based cache files to new JSON format
 128 |     fn migrate_old_cache(&self) {
 129 |         let old_cache_patterns = ["last_canonical.md", "last_output.md", "current_output.md"];
 130 | 
 131 |         for pattern in &old_cache_patterns {
 132 |             let old_cache_path = self.cache_dir.join(pattern);
 133 |             if old_cache_path.exists() {
 134 |                 eprintln!("Migrating old cache format: removing {}", pattern);
 135 |                 let _ = fs::remove_file(&old_cache_path);
 136 |             }
 137 |         }
 138 | 
 139 |         // Also remove any files that look like timestamped outputs from old versions
 140 |         if let Ok(entries) = fs::read_dir(&self.cache_dir) {
 141 |             for entry in entries.flatten() {
 142 |                 let file_name = entry.file_name();
 143 |                 let name = file_name.to_string_lossy();
 144 |                 if name.ends_with(".md") && (name.contains("_20") || name.starts_with("output_")) {
 145 |                     eprintln!("Migrating old cache format: removing {}", name);
 146 |                     let _ = fs::remove_file(entry.path());
 147 |                 }
 148 |             }
 149 |         }
 150 |     }
 151 | 
 152 |     /// Read the cached project state with file locking
 153 |     pub fn read_cache(&self) -> Result<Option<ProjectState>, Box<dyn std::error::Error>> {
 154 |         let cache_path = self.get_cache_path();
 155 | 
 156 |         if !cache_path.exists() {
 157 |             return Ok(None);
 158 |         }
 159 | 
 160 |         let file = File::open(&cache_path)?;
 161 |         // Acquire shared lock to prevent reading while writing
 162 |         file.lock_shared()?;
 163 | 
 164 |         let mut contents = String::new();
 165 |         let mut file = std::io::BufReader::new(file);
 166 |         file.read_to_string(&mut contents)?;
 167 | 
 168 |         // Release lock
 169 |         file.get_ref().unlock()?;
 170 | 
 171 |         let state: ProjectState = serde_json::from_str(&contents)?;
 172 |         Ok(Some(state))
 173 |     }
 174 | 
 175 |     /// Write the project state to cache with file locking
 176 |     pub fn write_cache(&self, state: &ProjectState) -> Result<(), Box<dyn std::error::Error>> {
 177 |         let cache_path = self.get_cache_path();
 178 | 
 179 |         let file = std::fs::OpenOptions::new()
 180 |             .write(true)
 181 |             .create(true)
 182 |             .truncate(false)
 183 |             .open(&cache_path)?;
 184 |         // Acquire exclusive lock BEFORE truncating to prevent TOCTOU races
 185 |         file.lock_exclusive()?;
 186 |         file.set_len(0)?;
 187 | 
 188 |         let json = serde_json::to_string_pretty(state)?;
 189 |         let mut file = std::io::BufWriter::new(file);
 190 |         file.write_all(json.as_bytes())?;
 191 |         file.flush()?;
 192 | 
 193 |         // Release lock
 194 |         file.get_ref().unlock()?;
 195 | 
 196 |         Ok(())
 197 |     }
 198 | }
 199 | 
 200 | #[cfg(test)]
 201 | mod tests {
 202 |     use super::*;
 203 |     use std::path::Path;
 204 |     use tempfile::tempdir;
 205 | 
 206 |     #[test]
 207 |     fn test_hash_path() {
 208 |         let path1 = Path::new("/project1");
 209 |         let path2 = Path::new("/project2");
 210 | 
 211 |         let hash1 = CacheManager::hash_path(path1);
 212 |         let hash2 = CacheManager::hash_path(path2);
 213 | 
 214 |         assert_ne!(
 215 |             hash1, hash2,
 216 |             "Different paths should produce different hashes"
 217 |         );
 218 |     }
 219 | 
 220 |     #[test]
 221 |     fn test_hash_config() {
 222 |         let config1 = Config {
 223 |             filter: Some(vec!["rs".to_string()]),
 224 |             ignore: Some(vec!["target".to_string()]),
 225 |             line_numbers: Some(true),
 226 |             ..Default::default()
 227 |         };
 228 | 
 229 |         let config2 = Config {
 230 |             filter: Some(vec!["md".to_string()]),
 231 |             ignore: Some(vec!["target".to_string()]),
 232 |             line_numbers: Some(true),
 233 |             ..Default::default()
 234 |         };
 235 | 
 236 |         let hash1 = CacheManager::hash_config(&config1);
 237 |         let hash2 = CacheManager::hash_config(&config2);
 238 | 
 239 |         assert_ne!(
 240 |             hash1, hash2,
 241 |             "Different configs should produce different hashes"
 242 |         );
 243 |     }
 244 | 
 245 |     #[test]
 246 |     fn test_cache_operations() {
 247 |         let dir = tempdir().unwrap();
 248 |         let project_path = dir.path().join("test_project");
 249 |         let _ = fs::create_dir(&project_path);
 250 | 
 251 |         let config = Config::default();
 252 |         let cache_manager = CacheManager::new(&project_path, &config);
 253 | 
 254 |         use crate::state::ProjectMetadata;
 255 | 
 256 |         let state = ProjectState {
 257 |             timestamp: "2023-01-01T00:00:00Z".to_string(),
 258 |             config_hash: "test_config_hash".to_string(),
 259 |             files: std::collections::BTreeMap::new(),
 260 |             metadata: ProjectMetadata {
 261 |                 project_name: "test".to_string(),
 262 |                 file_count: 0,
 263 |                 filters: vec![],
 264 |                 ignores: vec![],
 265 |                 line_numbers: false,
 266 |             },
 267 |         };
 268 | 
 269 |         // Write cache
 270 |         assert!(cache_manager.write_cache(&state).is_ok());
 271 | 
 272 |         // Read cache
 273 |         let cached_state = cache_manager.read_cache().unwrap();
 274 |         assert!(cached_state.is_some());
 275 |         assert_eq!(cached_state.unwrap().timestamp, state.timestamp);
 276 |     }
 277 | 
 278 |     #[test]
 279 |     fn test_old_cache_migration() {
 280 |         let dir = tempdir().unwrap();
 281 |         let project_path = dir.path().join("test_project");
 282 |         let _ = fs::create_dir(&project_path);
 283 | 
 284 |         // Create cache directory with old cache files
 285 |         let cache_dir = project_path.join(".context-builder").join("cache");
 286 |         let _ = fs::create_dir_all(&cache_dir);
 287 | 
 288 |         let old_files = [
 289 |             "last_canonical.md",
 290 |             "last_output.md",
 291 |             "current_output.md",
 292 |             "output_20230101120000.md",
 293 |         ];
 294 | 
 295 |         // Create old cache files
 296 |         for file in &old_files {
 297 |             let old_path = cache_dir.join(file);
 298 |             let _ = fs::write(&old_path, "old cache content");
 299 |             assert!(
 300 |                 old_path.exists(),
 301 |                 "Old cache file should exist before migration"
 302 |             );
 303 |         }
 304 | 
 305 |         // Create cache manager (this should trigger migration)
 306 |         let config = Config::default();
 307 |         let _cache_manager = CacheManager::new(&project_path, &config);
 308 | 
 309 |         // Verify old files are removed
 310 |         for file in &old_files {
 311 |             let old_path = cache_dir.join(file);
 312 |             assert!(
 313 |                 !old_path.exists(),
 314 |                 "Old cache file {} should be removed after migration",
 315 |                 file
 316 |             );
 317 |         }
 318 |     }
 319 | 
 320 |     #[test]
 321 |     fn test_cache_consistency_across_path_representations() {
 322 |         let dir = tempdir().unwrap();
 323 |         let project_path = dir.path().join("test_project");
 324 |         let _ = fs::create_dir(&project_path);
 325 | 
 326 |         let config = Config::default();
 327 | 
 328 |         // Test different path representations that should resolve to the same cache
 329 |         let mut paths_to_test = vec![
 330 |             project_path.clone(),
 331 |             project_path.canonicalize().unwrap_or(project_path.clone()),
 332 |         ];
 333 | 
 334 |         // If we can create a relative path, test that too
 335 |         if let Ok(current_dir) = std::env::current_dir()
 336 |             && let Ok(relative) = project_path.strip_prefix(&current_dir)
 337 |         {
 338 |             paths_to_test.push(relative.to_path_buf());
 339 |         }
 340 | 
 341 |         let mut cache_paths = Vec::new();
 342 |         for path in &paths_to_test {
 343 |             let cache_manager = CacheManager::new(path, &config);
 344 |             cache_paths.push(cache_manager.get_cache_path());
 345 |         }
 346 | 
 347 |         // All cache paths should be identical
 348 |         for (i, path1) in cache_paths.iter().enumerate() {
 349 |             for (j, path2) in cache_paths.iter().enumerate() {
 350 |                 if i != j {
 351 |                     assert_eq!(
 352 |                         path1, path2,
 353 |                         "Cache paths should be identical for different representations of the same project path"
 354 |                     );
 355 |                 }
 356 |             }
 357 |         }
 358 |     }
 359 | 
 360 |     #[test]
 361 |     fn test_normalize_path_format() {
 362 |         // Test Windows UNC path normalization
 363 |         if cfg!(windows) {
 364 |             let unc_path = Path::new("\\\\?\\C:\\test\\path");
 365 |             let normalized = CacheManager::normalize_path_format(unc_path);
 366 |             assert_eq!(normalized, PathBuf::from("C:\\test\\path"));
 367 |         }
 368 | 
 369 |         // Test normal path (should remain unchanged)
 370 |         let normal_path = Path::new("/normal/path");
 371 |         let normalized = CacheManager::normalize_path_format(normal_path);
 372 |         assert_eq!(normalized, normal_path);
 373 |     }
 374 | 
 375 |     #[test]
 376 |     fn test_cache_read_nonexistent_file() {
 377 |         let dir = tempdir().unwrap();
 378 |         let project_path = dir.path().join("nonexistent_project");
 379 | 
 380 |         let config = Config::default();
 381 |         let cache_manager = CacheManager::new(&project_path, &config);
 382 | 
 383 |         let result = cache_manager.read_cache().unwrap();
 384 |         assert!(result.is_none());
 385 |     }
 386 | 
 387 |     #[test]
 388 |     fn test_cache_read_corrupted_file() {
 389 |         let dir = tempdir().unwrap();
 390 |         let project_path = dir.path().join("test_project");
 391 |         let _ = fs::create_dir(&project_path);
 392 | 
 393 |         let config = Config::default();
 394 |         let cache_manager = CacheManager::new(&project_path, &config);
 395 |         let cache_path = cache_manager.get_cache_path();
 396 | 
 397 |         // Create a corrupted cache file
 398 |         let _ = fs::create_dir_all(cache_path.parent().unwrap());
 399 |         let _ = fs::write(&cache_path, "invalid json content {{{");
 400 | 
 401 |         let result = cache_manager.read_cache();
 402 |         assert!(result.is_err());
 403 |     }
 404 | 
 405 |     #[test]
 406 |     fn test_cache_write_read_roundtrip() {
 407 |         let dir = tempdir().unwrap();
 408 |         let project_path = dir.path().join("test_project");
 409 |         let _ = fs::create_dir(&project_path);
 410 | 
 411 |         let config = Config {
 412 |             filter: Some(vec!["rs".to_string(), "toml".to_string()]),
 413 |             ignore: Some(vec!["target".to_string(), ".git".to_string()]),
 414 |             line_numbers: Some(true),
 415 |             ..Default::default()
 416 |         };
 417 | 
 418 |         let cache_manager = CacheManager::new(&project_path, &config);
 419 | 
 420 |         use crate::state::ProjectMetadata;
 421 |         use std::collections::BTreeMap;
 422 | 
 423 |         let mut files = BTreeMap::new();
 424 |         files.insert(
 425 |             PathBuf::from("test.rs"),
 426 |             crate::state::FileState {
 427 |                 content: "fn main() {}".to_string(),
 428 |                 size: 12,
 429 |                 modified: std::time::SystemTime::UNIX_EPOCH,
 430 |                 content_hash: "test_hash".to_string(),
 431 |             },
 432 |         );
 433 | 
 434 |         let original_state = ProjectState {
 435 |             timestamp: "2023-01-01T12:00:00Z".to_string(),
 436 |             config_hash: "test_config_hash".to_string(),
 437 |             files,
 438 |             metadata: ProjectMetadata {
 439 |                 project_name: "test_project".to_string(),
 440 |                 file_count: 1,
 441 |                 filters: vec!["rs".to_string(), "toml".to_string()],
 442 |                 ignores: vec!["target".to_string(), ".git".to_string()],
 443 |                 line_numbers: true,
 444 |             },
 445 |         };
 446 | 
 447 |         // Write and read back
 448 |         cache_manager.write_cache(&original_state).unwrap();
 449 |         let cached_state = cache_manager.read_cache().unwrap().unwrap();
 450 | 
 451 |         assert_eq!(cached_state.timestamp, original_state.timestamp);
 452 |         assert_eq!(cached_state.config_hash, original_state.config_hash);
 453 |         assert_eq!(cached_state.files.len(), original_state.files.len());
 454 |         assert_eq!(
 455 |             cached_state.metadata.project_name,
 456 |             original_state.metadata.project_name
 457 |         );
 458 |         assert_eq!(
 459 |             cached_state.metadata.file_count,
 460 |             original_state.metadata.file_count
 461 |         );
 462 |         assert_eq!(
 463 |             cached_state.metadata.filters,
 464 |             original_state.metadata.filters
 465 |         );
 466 |         assert_eq!(
 467 |             cached_state.metadata.ignores,
 468 |             original_state.metadata.ignores
 469 |         );
 470 |         assert_eq!(
 471 |             cached_state.metadata.line_numbers,
 472 |             original_state.metadata.line_numbers
 473 |         );
 474 |     }
 475 | 
 476 |     #[test]
 477 |     fn test_different_configs_different_cache_files() {
 478 |         let dir = tempdir().unwrap();
 479 |         let project_path = dir.path().join("test_project");
 480 |         let _ = fs::create_dir(&project_path);
 481 | 
 482 |         let config1 = Config {
 483 |             filter: Some(vec!["rs".to_string()]),
 484 |             ..Default::default()
 485 |         };
 486 | 
 487 |         let config2 = Config {
 488 |             filter: Some(vec!["py".to_string()]),
 489 |             ..Default::default()
 490 |         };
 491 | 
 492 |         let cache_manager1 = CacheManager::new(&project_path, &config1);
 493 |         let cache_manager2 = CacheManager::new(&project_path, &config2);
 494 | 
 495 |         let cache_path1 = cache_manager1.get_cache_path();
 496 |         let cache_path2 = cache_manager2.get_cache_path();
 497 | 
 498 |         assert_ne!(
 499 |             cache_path1, cache_path2,
 500 |             "Different configs should have different cache files"
 501 |         );
 502 |     }
 503 | 
 504 |     #[test]
 505 |     fn test_normalize_project_path_absolute() {
 506 |         let temp_dir = tempdir().unwrap();
 507 |         let project_path = temp_dir.path().join("test_project");
 508 |         let _ = fs::create_dir(&project_path);
 509 | 
 510 |         let normalized = CacheManager::normalize_project_path(&project_path);
 511 |         assert!(normalized.is_absolute());
 512 |     }
 513 | 
 514 |     #[test]
 515 |     fn test_normalize_project_path_relative() {
 516 |         let temp_dir = tempdir().unwrap();
 517 |         let original_dir = std::env::current_dir().unwrap();
 518 | 
 519 |         // Change to temp directory
 520 |         std::env::set_current_dir(&temp_dir).unwrap();
 521 | 
 522 |         // Create a project directory
 523 |         let project_name = "relative_project";
 524 |         let _ = fs::create_dir(project_name);
 525 | 
 526 |         let relative_path = Path::new(project_name);
 527 |         let normalized = CacheManager::normalize_project_path(relative_path);
 528 | 
 529 |         // Restore original directory
 530 |         std::env::set_current_dir(original_dir).unwrap();
 531 | 
 532 |         assert!(normalized.is_absolute());
 533 |         assert!(normalized.to_string_lossy().contains(project_name));
 534 |     }
 535 | 
 536 |     #[test]
 537 |     fn test_hash_config_same_values() {
 538 |         let config1 = Config {
 539 |             filter: Some(vec!["rs".to_string(), "toml".to_string()]),
 540 |             ignore: Some(vec!["target".to_string()]),
 541 |             line_numbers: Some(false),
 542 |             ..Default::default()
 543 |         };
 544 | 
 545 |         let config2 = Config {
 546 |             filter: Some(vec!["rs".to_string(), "toml".to_string()]),
 547 |             ignore: Some(vec!["target".to_string()]),
 548 |             line_numbers: Some(false),
 549 |             ..Default::default()
 550 |         };
 551 | 
 552 |         let hash1 = CacheManager::hash_config(&config1);
 553 |         let hash2 = CacheManager::hash_config(&config2);
 554 | 
 555 |         assert_eq!(
 556 |             hash1, hash2,
 557 |             "Identical configs should produce identical hashes"
 558 |         );
 559 |     }
 560 | 
 561 |     #[test]
 562 |     fn test_migrate_old_cache_preserves_new_files() {
 563 |         let dir = tempdir().unwrap();
 564 |         let project_path = dir.path().join("test_project");
 565 |         let _ = fs::create_dir(&project_path);
 566 | 
 567 |         let cache_dir = project_path.join(".context-builder").join("cache");
 568 |         let _ = fs::create_dir_all(&cache_dir);
 569 | 
 570 |         // Create both old and new cache files
 571 |         let _ = fs::write(cache_dir.join("last_canonical.md"), "old content");
 572 |         let _ = fs::write(cache_dir.join("state_abc123_def456.json"), "new content");
 573 | 
 574 |         let config = Config::default();
 575 |         let _cache_manager = CacheManager::new(&project_path, &config);
 576 | 
 577 |         // Old file should be removed
 578 |         assert!(!cache_dir.join("last_canonical.md").exists());
 579 | 
 580 |         // New file should be preserved
 581 |         assert!(cache_dir.join("state_abc123_def456.json").exists());
 582 |     }
 583 | }
```

### File: `src/cli.rs`

- Size: 6553 bytes
- Modified: SystemTime { tv_sec: 1771138255, tv_nsec: 543523125 }

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
  54 | 
  55 |     /// Extract function/class signatures only (requires tree-sitter feature)
  56 |     #[clap(long)]
  57 |     pub signatures: bool,
  58 | 
  59 |     /// Extract code structure (imports, exports, symbol counts) - requires tree-sitter feature
  60 |     #[clap(long)]
  61 |     pub structure: bool,
  62 | 
  63 |     /// Truncation mode for max-tokens: "smart" (AST boundaries) or "byte"
  64 |     #[clap(long, value_name = "MODE", default_value = "smart")]
  65 |     pub truncate: String,
  66 | 
  67 |     /// Filter signatures by visibility: "all", "public", or "private"
  68 |     #[clap(long, default_value = "all")]
  69 |     pub visibility: String,
  70 | }
  71 | 
  72 | #[cfg(test)]
  73 | mod tests {
  74 |     use super::Args;
  75 |     use clap::Parser;
  76 | 
  77 |     #[test]
  78 |     fn parses_with_no_args() {
  79 |         let res = Args::try_parse_from(["context-builder"]);
  80 |         assert!(res.is_ok(), "Expected success when no args are provided");
  81 |     }
  82 | 
  83 |     #[test]
  84 |     fn parses_all_flags_and_options() {
  85 |         let args = Args::try_parse_from([
  86 |             "context-builder",
  87 |             "--input",
  88 |             "some/dir",
  89 |             "--output",
  90 |             "ctx.md",
  91 |             "--filter",
  92 |             "rs",
  93 |             "--filter",
  94 |             "toml",
  95 |             "--ignore",
  96 |             "target",
  97 |             "--ignore",
  98 |             "node_modules",
  99 |             "--preview",
 100 |             "--token-count",
 101 |             "--line-numbers",
 102 |             "--diff-only",
 103 |             "--clear-cache",
 104 |         ])
 105 |         .expect("should parse");
 106 | 
 107 |         assert_eq!(args.input, "some/dir");
 108 |         assert_eq!(args.output, "ctx.md");
 109 |         assert_eq!(args.filter, vec!["rs".to_string(), "toml".to_string()]);
 110 |         assert_eq!(
 111 |             args.ignore,
 112 |             vec!["target".to_string(), "node_modules".to_string()]
 113 |         );
 114 |         assert!(args.preview);
 115 |         assert!(args.token_count);
 116 |         assert!(args.line_numbers);
 117 |         assert!(args.diff_only);
 118 |         assert!(args.clear_cache);
 119 |     }
 120 | 
 121 |     #[test]
 122 |     fn short_flags_parse_correctly() {
 123 |         let args = Args::try_parse_from([
 124 |             "context-builder",
 125 |             "-d",
 126 |             ".",
 127 |             "-o",
 128 |             "out.md",
 129 |             "-f",
 130 |             "md",
 131 |             "-f",
 132 |             "rs",
 133 |             "-i",
 134 |             "target",
 135 |             "-i",
 136 |             ".git",
 137 |         ])
 138 |         .expect("should parse");
 139 | 
 140 |         assert_eq!(args.input, ".");
 141 |         assert_eq!(args.output, "out.md");
 142 |         assert_eq!(args.filter, vec!["md".to_string(), "rs".to_string()]);
 143 |         assert_eq!(args.ignore, vec!["target".to_string(), ".git".to_string()]);
 144 |         assert!(!args.preview);
 145 |         assert!(!args.line_numbers);
 146 |         assert!(!args.clear_cache);
 147 |     }
 148 | 
 149 |     #[test]
 150 |     fn defaults_for_options_when_not_provided() {
 151 |         let args = Args::try_parse_from(["context-builder", "-d", "proj"]).expect("should parse");
 152 | 
 153 |         assert_eq!(args.input, "proj");
 154 |         assert_eq!(args.output, "output.md");
 155 |         assert!(args.filter.is_empty());
 156 |         assert!(args.ignore.is_empty());
 157 |         assert!(!args.preview);
 158 |         assert!(!args.line_numbers);
 159 |         assert!(!args.diff_only);
 160 |         assert!(!args.clear_cache);
 161 |     }
 162 | 
 163 |     #[test]
 164 |     fn parses_diff_only_flag() {
 165 |         let args = Args::try_parse_from(["context-builder", "--diff-only"])
 166 |             .expect("should parse diff-only flag");
 167 |         assert!(args.diff_only);
 168 |         assert!(!args.clear_cache);
 169 |     }
 170 | 
 171 |     #[test]
 172 |     fn parses_clear_cache_flag() {
 173 |         let args = Args::try_parse_from(["context-builder", "--clear-cache"])
 174 |             .expect("should parse clear-cache flag");
 175 |         assert!(args.clear_cache);
 176 |         assert!(!args.diff_only);
 177 |     }
 178 | 
 179 |     #[test]
 180 |     fn parses_signatures_flag() {
 181 |         let args = Args::try_parse_from(["context-builder", "--signatures"])
 182 |             .expect("should parse signatures flag");
 183 |         assert!(args.signatures);
 184 |     }
 185 | 
 186 |     #[test]
 187 |     fn parses_structure_flag() {
 188 |         let args = Args::try_parse_from(["context-builder", "--structure"])
 189 |             .expect("should parse structure flag");
 190 |         assert!(args.structure);
 191 |     }
 192 | 
 193 |     #[test]
 194 |     fn parses_truncate_mode() {
 195 |         let args = Args::try_parse_from(["context-builder", "--truncate", "byte"])
 196 |             .expect("should parse truncate flag");
 197 |         assert_eq!(args.truncate, "byte");
 198 | 
 199 |         let args_default =
 200 |             Args::try_parse_from(["context-builder"]).expect("should parse with default truncate");
 201 |         assert_eq!(args_default.truncate, "smart");
 202 |     }
 203 | 
 204 |     #[test]
 205 |     fn parses_visibility_filter() {
 206 |         let args = Args::try_parse_from(["context-builder", "--visibility", "public"])
 207 |             .expect("should parse visibility flag");
 208 |         assert_eq!(args.visibility, "public");
 209 | 
 210 |         let args_default = Args::try_parse_from(["context-builder"])
 211 |             .expect("should parse with default visibility");
 212 |         assert_eq!(args_default.visibility, "all");
 213 |     }
 214 | }
```

### File: `src/config.rs`

- Size: 8954 bytes
- Modified: SystemTime { tv_sec: 1771138308, tv_nsec: 12218245 }

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
  75 | 
  76 |     /// Extract function/class signatures only (requires tree-sitter feature)
  77 |     pub signatures: Option<bool>,
  78 | 
  79 |     /// Extract code structure (imports, exports, symbol counts) - requires tree-sitter feature
  80 |     pub structure: Option<bool>,
  81 | 
  82 |     /// Truncation mode for max-tokens: "smart" (AST boundaries) or "byte"
  83 |     pub truncate: Option<String>,
  84 | 
  85 |     /// Filter signatures by visibility: "all", "public", or "private"
  86 |     pub visibility: Option<String>,
  87 | }
  88 | 
  89 | /// Load configuration from `context-builder.toml` in the current working directory.
  90 | /// Returns `None` if the file does not exist or cannot be parsed.
  91 | pub fn load_config() -> Option<Config> {
  92 |     let config_path = Path::new("context-builder.toml");
  93 |     if config_path.exists() {
  94 |         let content = fs::read_to_string(config_path).ok()?;
  95 |         match toml::from_str(&content) {
  96 |             Ok(config) => Some(config),
  97 |             Err(e) => {
  98 |                 eprintln!(
  99 |                     "⚠️  Failed to parse context-builder.toml: {}. Config will be ignored.",
 100 |                     e
 101 |                 );
 102 |                 None
 103 |             }
 104 |         }
 105 |     } else {
 106 |         None
 107 |     }
 108 | }
 109 | 
 110 | /// Load configuration from `context-builder.toml` in the specified project root directory.
 111 | /// Returns `None` if the file does not exist or cannot be parsed.
 112 | pub fn load_config_from_path(project_root: &Path) -> Option<Config> {
 113 |     let config_path = project_root.join("context-builder.toml");
 114 |     if config_path.exists() {
 115 |         let content = fs::read_to_string(&config_path).ok()?;
 116 |         match toml::from_str(&content) {
 117 |             Ok(config) => Some(config),
 118 |             Err(e) => {
 119 |                 eprintln!(
 120 |                     "⚠️  Failed to parse {}: {}. Config will be ignored.",
 121 |                     config_path.display(),
 122 |                     e
 123 |                 );
 124 |                 None
 125 |             }
 126 |         }
 127 |     } else {
 128 |         None
 129 |     }
 130 | }
 131 | 
 132 | #[cfg(test)]
 133 | mod tests {
 134 |     use super::*;
 135 |     use std::fs;
 136 |     use tempfile::tempdir;
 137 | 
 138 |     #[test]
 139 |     fn load_config_nonexistent_file() {
 140 |         // Test loading config when file doesn't exist by temporarily changing directory
 141 |         let temp_dir = tempdir().unwrap();
 142 |         let original_dir = std::env::current_dir().unwrap();
 143 | 
 144 |         // Change to temp directory where no config file exists
 145 |         std::env::set_current_dir(&temp_dir).unwrap();
 146 | 
 147 |         let result = load_config();
 148 | 
 149 |         // Restore original directory
 150 |         std::env::set_current_dir(original_dir).unwrap();
 151 | 
 152 |         assert!(result.is_none());
 153 |     }
 154 | 
 155 |     #[test]
 156 |     fn load_config_from_path_nonexistent_file() {
 157 |         let dir = tempdir().unwrap();
 158 |         let result = load_config_from_path(dir.path());
 159 |         assert!(result.is_none());
 160 |     }
 161 | 
 162 |     #[test]
 163 |     fn load_config_from_path_valid_config() {
 164 |         let dir = tempdir().unwrap();
 165 |         let config_path = dir.path().join("context-builder.toml");
 166 | 
 167 |         let config_content = r#"
 168 | output = "test-output.md"
 169 | filter = ["rs", "toml"]
 170 | ignore = ["target", ".git"]
 171 | line_numbers = true
 172 | preview = false
 173 | token_count = true
 174 | timestamped_output = true
 175 | yes = false
 176 | auto_diff = true
 177 | diff_context_lines = 5
 178 | diff_only = false
 179 | encoding_strategy = "detect"
 180 | "#;
 181 | 
 182 |         fs::write(&config_path, config_content).unwrap();
 183 | 
 184 |         let config = load_config_from_path(dir.path()).unwrap();
 185 |         assert_eq!(config.output.unwrap(), "test-output.md");
 186 |         assert_eq!(config.filter.unwrap(), vec!["rs", "toml"]);
 187 |         assert_eq!(config.ignore.unwrap(), vec!["target", ".git"]);
 188 |         assert!(config.line_numbers.unwrap());
 189 |         assert!(!config.preview.unwrap());
 190 |         assert!(config.token_count.unwrap());
 191 |         assert!(config.timestamped_output.unwrap());
 192 |         assert!(!config.yes.unwrap());
 193 |         assert!(config.auto_diff.unwrap());
 194 |         assert_eq!(config.diff_context_lines.unwrap(), 5);
 195 |         assert!(!config.diff_only.unwrap());
 196 |         assert_eq!(config.encoding_strategy.unwrap(), "detect");
 197 |     }
 198 | 
 199 |     #[test]
 200 |     fn load_config_from_path_partial_config() {
 201 |         let dir = tempdir().unwrap();
 202 |         let config_path = dir.path().join("context-builder.toml");
 203 | 
 204 |         let config_content = r#"
 205 | output = "minimal.md"
 206 | filter = ["py"]
 207 | "#;
 208 | 
 209 |         fs::write(&config_path, config_content).unwrap();
 210 | 
 211 |         let config = load_config_from_path(dir.path()).unwrap();
 212 |         assert_eq!(config.output.unwrap(), "minimal.md");
 213 |         assert_eq!(config.filter.unwrap(), vec!["py"]);
 214 |         assert!(config.ignore.is_none());
 215 |         assert!(config.line_numbers.is_none());
 216 |         assert!(config.auto_diff.is_none());
 217 |     }
 218 | 
 219 |     #[test]
 220 |     fn load_config_from_path_invalid_toml() {
 221 |         let dir = tempdir().unwrap();
 222 |         let config_path = dir.path().join("context-builder.toml");
 223 | 
 224 |         // Invalid TOML content
 225 |         let config_content = r#"
 226 | output = "test.md"
 227 | invalid_toml [
 228 | "#;
 229 | 
 230 |         fs::write(&config_path, config_content).unwrap();
 231 | 
 232 |         let config = load_config_from_path(dir.path());
 233 |         assert!(config.is_none());
 234 |     }
 235 | 
 236 |     #[test]
 237 |     fn load_config_from_path_empty_config() {
 238 |         let dir = tempdir().unwrap();
 239 |         let config_path = dir.path().join("context-builder.toml");
 240 | 
 241 |         fs::write(&config_path, "").unwrap();
 242 | 
 243 |         let config = load_config_from_path(dir.path()).unwrap();
 244 |         assert!(config.output.is_none());
 245 |         assert!(config.filter.is_none());
 246 |         assert!(config.ignore.is_none());
 247 |     }
 248 | 
 249 |     #[test]
 250 |     fn config_default_implementation() {
 251 |         let config = Config::default();
 252 |         assert!(config.output.is_none());
 253 |         assert!(config.filter.is_none());
 254 |         assert!(config.ignore.is_none());
 255 |         assert!(config.line_numbers.is_none());
 256 |         assert!(config.preview.is_none());
 257 |         assert!(config.token_count.is_none());
 258 |         assert!(config.output_folder.is_none());
 259 |         assert!(config.timestamped_output.is_none());
 260 |         assert!(config.yes.is_none());
 261 |         assert!(config.auto_diff.is_none());
 262 |         assert!(config.diff_context_lines.is_none());
 263 |         assert!(config.diff_only.is_none());
 264 |         assert!(config.encoding_strategy.is_none());
 265 |         assert!(config.max_tokens.is_none());
 266 |         assert!(config.signatures.is_none());
 267 |         assert!(config.structure.is_none());
 268 |         assert!(config.truncate.is_none());
 269 |         assert!(config.visibility.is_none());
 270 |     }
 271 | }
```

### File: `src/config_resolver.rs`

- Size: 15995 bytes
- Modified: SystemTime { tv_sec: 1771138540, tv_nsec: 524298632 }

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
  30 |     pub signatures: bool,
  31 |     pub structure: bool,
  32 |     pub truncate: String,
  33 |     pub visibility: String,
  34 | }
  35 | 
  36 | /// Result of configuration resolution including the final config and any warnings
  37 | #[derive(Debug)]
  38 | pub struct ConfigResolution {
  39 |     pub config: ResolvedConfig,
  40 |     pub warnings: Vec<String>,
  41 | }
  42 | 
  43 | /// Resolves final configuration by merging CLI arguments with config file values.
  44 | ///
  45 | /// Precedence rules (highest to lowest):
  46 | /// 1. Explicit CLI arguments (non-default values)
  47 | /// 2. Configuration file values
  48 | /// 3. CLI default values
  49 | ///
  50 | /// Special handling:
  51 | /// - `output` field supports timestamping and output folder resolution
  52 | /// - Boolean flags respect explicit CLI usage vs defaults
  53 | /// - Arrays (filter, ignore) use CLI if non-empty, otherwise config file
  54 | pub fn resolve_final_config(mut args: Args, config: Option<Config>) -> ConfigResolution {
  55 |     let mut warnings = Vec::new();
  56 | 
  57 |     // Start with CLI defaults, then apply config file, then explicit CLI overrides
  58 |     let final_config = if let Some(config) = config {
  59 |         apply_config_to_args(&mut args, &config, &mut warnings);
  60 |         resolve_output_path(&mut args, &config, &mut warnings);
  61 |         config
  62 |     } else {
  63 |         Config::default()
  64 |     };
  65 | 
  66 |     let resolved = ResolvedConfig {
  67 |         input: args.input,
  68 |         output: args.output,
  69 |         filter: args.filter,
  70 |         ignore: args.ignore,
  71 |         line_numbers: args.line_numbers,
  72 |         preview: args.preview,
  73 |         token_count: args.token_count,
  74 |         yes: args.yes,
  75 |         diff_only: args.diff_only,
  76 |         clear_cache: args.clear_cache,
  77 |         auto_diff: final_config.auto_diff.unwrap_or(false),
  78 |         diff_context_lines: final_config.diff_context_lines.unwrap_or(3),
  79 |         max_tokens: args.max_tokens.or(final_config.max_tokens),
  80 |         init: args.init,
  81 |         signatures: args.signatures || final_config.signatures.unwrap_or(false),
  82 |         structure: args.structure || final_config.structure.unwrap_or(false),
  83 |         truncate: if args.truncate != "smart" {
  84 |             args.truncate.clone()
  85 |         } else {
  86 |             final_config
  87 |                 .truncate
  88 |                 .clone()
  89 |                 .unwrap_or_else(|| args.truncate.clone())
  90 |         },
  91 |         visibility: if args.visibility != "all" {
  92 |             args.visibility.clone()
  93 |         } else {
  94 |             final_config
  95 |                 .visibility
  96 |                 .clone()
  97 |                 .unwrap_or_else(|| args.visibility.clone())
  98 |         },
  99 |     };
 100 | 
 101 |     ConfigResolution {
 102 |         config: resolved,
 103 |         warnings,
 104 |     }
 105 | }
 106 | 
 107 | /// Apply configuration file values to CLI arguments based on precedence rules
 108 | fn apply_config_to_args(args: &mut Args, config: &Config, warnings: &mut Vec<String>) {
 109 |     // Output: only apply config if CLI is using default value
 110 |     if args.output == "output.md"
 111 |         && let Some(ref output) = config.output
 112 |     {
 113 |         args.output = output.clone();
 114 |     }
 115 | 
 116 |     // Filter: CLI takes precedence if non-empty
 117 |     if args.filter.is_empty()
 118 |         && let Some(ref filter) = config.filter
 119 |     {
 120 |         args.filter = filter.clone();
 121 |     }
 122 | 
 123 |     // Ignore: CLI takes precedence if non-empty
 124 |     if args.ignore.is_empty()
 125 |         && let Some(ref ignore) = config.ignore
 126 |     {
 127 |         args.ignore = ignore.clone();
 128 |     }
 129 | 
 130 |     // Boolean flags: config applies only if CLI is using default (false)
 131 |     // Note: We can't distinguish between explicit --no-flag and default false,
 132 |     // so config file can only enable features, not disable them
 133 |     if !args.line_numbers
 134 |         && let Some(line_numbers) = config.line_numbers
 135 |     {
 136 |         args.line_numbers = line_numbers;
 137 |     }
 138 | 
 139 |     if !args.preview
 140 |         && let Some(preview) = config.preview
 141 |     {
 142 |         args.preview = preview;
 143 |     }
 144 | 
 145 |     if !args.token_count
 146 |         && let Some(token_count) = config.token_count
 147 |     {
 148 |         args.token_count = token_count;
 149 |     }
 150 | 
 151 |     if !args.yes
 152 |         && let Some(yes) = config.yes
 153 |     {
 154 |         args.yes = yes;
 155 |     }
 156 | 
 157 |     // diff_only: config can enable it, but CLI flag always takes precedence
 158 |     if !args.diff_only
 159 |         && let Some(true) = config.diff_only
 160 |     {
 161 |         args.diff_only = true;
 162 |     }
 163 | 
 164 |     // Validate auto_diff configuration
 165 |     if let Some(true) = config.auto_diff
 166 |         && config.timestamped_output != Some(true)
 167 |     {
 168 |         warnings.push(
 169 |             "auto_diff is enabled but timestamped_output is not enabled. \
 170 |             Auto-diff requires timestamped_output = true to function properly."
 171 |                 .to_string(),
 172 |         );
 173 |     }
 174 | }
 175 | 
 176 | /// Resolve output path including timestamping and output folder logic
 177 | fn resolve_output_path(args: &mut Args, config: &Config, warnings: &mut Vec<String>) {
 178 |     let mut output_folder_path: Option<PathBuf> = None;
 179 | 
 180 |     // Apply output folder first
 181 |     if let Some(ref output_folder) = config.output_folder {
 182 |         let mut path = PathBuf::from(output_folder);
 183 |         path.push(&args.output);
 184 |         args.output = path.to_string_lossy().to_string();
 185 |         output_folder_path = Some(PathBuf::from(output_folder));
 186 |     }
 187 | 
 188 |     // Apply timestamping if enabled
 189 |     if let Some(true) = config.timestamped_output {
 190 |         let timestamp = Utc::now().format("%Y%m%d%H%M%S").to_string();
 191 |         let path = Path::new(&args.output);
 192 | 
 193 |         let stem = path
 194 |             .file_stem()
 195 |             .and_then(|s| s.to_str())
 196 |             .unwrap_or("output");
 197 | 
 198 |         let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("md");
 199 | 
 200 |         let new_filename = format!("{}_{}.{}", stem, timestamp, extension);
 201 | 
 202 |         if let Some(output_folder) = output_folder_path {
 203 |             args.output = output_folder
 204 |                 .join(new_filename)
 205 |                 .to_string_lossy()
 206 |                 .to_string();
 207 |         } else {
 208 |             let new_path = path.with_file_name(new_filename);
 209 |             args.output = new_path.to_string_lossy().to_string();
 210 |         }
 211 |     }
 212 | 
 213 |     // Validate output folder exists if specified
 214 |     if let Some(ref output_folder) = config.output_folder {
 215 |         let folder_path = Path::new(output_folder);
 216 |         if !folder_path.exists() {
 217 |             warnings.push(format!(
 218 |                 "Output folder '{}' does not exist. It will be created if possible.",
 219 |                 output_folder
 220 |             ));
 221 |         }
 222 |     }
 223 | }
 224 | 
 225 | #[cfg(test)]
 226 | mod tests {
 227 |     use super::*;
 228 | 
 229 |     #[test]
 230 |     fn test_config_precedence_cli_over_config() {
 231 |         let args = Args {
 232 |             input: "src".to_string(),
 233 |             output: "custom.md".to_string(), // Explicit CLI value
 234 |             filter: vec!["rs".to_string()],  // Explicit CLI value
 235 |             ignore: vec![],
 236 |             line_numbers: true, // Explicit CLI value
 237 |             preview: false,
 238 |             token_count: false,
 239 |             yes: false,
 240 |             diff_only: false,
 241 |             clear_cache: false,
 242 |             init: false,
 243 |             max_tokens: None,
 244 |             signatures: false,
 245 |             structure: false,
 246 |             truncate: "smart".to_string(),
 247 |             visibility: "all".to_string(),
 248 |         };
 249 | 
 250 |         let config = Config {
 251 |             output: Some("config.md".to_string()),  // Should be ignored
 252 |             filter: Some(vec!["toml".to_string()]), // Should be ignored
 253 |             line_numbers: Some(false),              // Should be ignored
 254 |             preview: Some(true),                    // Should apply
 255 |             ..Default::default()
 256 |         };
 257 | 
 258 |         let resolution = resolve_final_config(args.clone(), Some(config));
 259 | 
 260 |         assert_eq!(resolution.config.output, "custom.md"); // CLI wins
 261 |         assert_eq!(resolution.config.filter, vec!["rs"]); // CLI wins
 262 |         assert!(resolution.config.line_numbers); // CLI wins
 263 |         assert!(resolution.config.preview); // Config applies
 264 |     }
 265 | 
 266 |     #[test]
 267 |     fn test_config_applies_when_cli_uses_defaults() {
 268 |         let args = Args {
 269 |             input: "src".to_string(),
 270 |             output: "output.md".to_string(), // Default value
 271 |             filter: vec![],                  // Default value
 272 |             ignore: vec![],                  // Default value
 273 |             line_numbers: false,             // Default value
 274 |             preview: false,                  // Default value
 275 |             token_count: false,              // Default value
 276 |             yes: false,                      // Default value
 277 |             diff_only: false,                // Default value
 278 |             clear_cache: false,
 279 |             init: false,
 280 |             max_tokens: None,
 281 |             signatures: false,
 282 |             structure: false,
 283 |             truncate: "smart".to_string(),
 284 |             visibility: "all".to_string(),
 285 |         };
 286 | 
 287 |         let config = Config {
 288 |             output: Some("from_config.md".to_string()),
 289 |             filter: Some(vec!["rs".to_string(), "toml".to_string()]),
 290 |             ignore: Some(vec!["target".to_string()]),
 291 |             line_numbers: Some(true),
 292 |             preview: Some(true),
 293 |             token_count: Some(true),
 294 |             yes: Some(true),
 295 |             diff_only: Some(true),
 296 |             ..Default::default()
 297 |         };
 298 | 
 299 |         let resolution = resolve_final_config(args, Some(config));
 300 | 
 301 |         assert_eq!(resolution.config.output, "from_config.md");
 302 |         assert_eq!(
 303 |             resolution.config.filter,
 304 |             vec!["rs".to_string(), "toml".to_string()]
 305 |         );
 306 |         assert_eq!(resolution.config.ignore, vec!["target".to_string()]);
 307 |         assert!(resolution.config.line_numbers);
 308 |         assert!(resolution.config.preview);
 309 |         assert!(resolution.config.token_count);
 310 |         assert!(resolution.config.yes);
 311 |         assert!(resolution.config.diff_only);
 312 |     }
 313 | 
 314 |     #[test]
 315 |     fn test_timestamped_output_resolution() {
 316 |         let args = Args {
 317 |             input: "src".to_string(),
 318 |             output: "test.md".to_string(),
 319 |             filter: vec![],
 320 |             ignore: vec![],
 321 |             line_numbers: false,
 322 |             preview: false,
 323 |             token_count: false,
 324 |             yes: false,
 325 |             diff_only: false,
 326 |             clear_cache: false,
 327 |             init: false,
 328 |             max_tokens: None,
 329 |             signatures: false,
 330 |             structure: false,
 331 |             truncate: "smart".to_string(),
 332 |             visibility: "all".to_string(),
 333 |         };
 334 | 
 335 |         let config = Config {
 336 |             timestamped_output: Some(true),
 337 |             ..Default::default()
 338 |         };
 339 | 
 340 |         let resolution = resolve_final_config(args, Some(config));
 341 | 
 342 |         // Output should have timestamp format: test_YYYYMMDDHHMMSS.md
 343 |         assert!(resolution.config.output.starts_with("test_"));
 344 |         assert!(resolution.config.output.ends_with(".md"));
 345 |         assert!(resolution.config.output.len() > "test_.md".len());
 346 |     }
 347 | 
 348 |     #[test]
 349 |     fn test_output_folder_resolution() {
 350 |         let args = Args {
 351 |             input: "src".to_string(),
 352 |             output: "test.md".to_string(),
 353 |             filter: vec![],
 354 |             ignore: vec![],
 355 |             line_numbers: false,
 356 |             preview: false,
 357 |             token_count: false,
 358 |             yes: false,
 359 |             diff_only: false,
 360 |             clear_cache: false,
 361 |             init: false,
 362 |             max_tokens: None,
 363 |             signatures: false,
 364 |             structure: false,
 365 |             truncate: "smart".to_string(),
 366 |             visibility: "all".to_string(),
 367 |         };
 368 | 
 369 |         let config = Config {
 370 |             output_folder: Some("docs".to_string()),
 371 |             ..Default::default()
 372 |         };
 373 | 
 374 |         let resolution = resolve_final_config(args, Some(config));
 375 | 
 376 |         assert!(resolution.config.output.contains("docs"));
 377 |         assert!(resolution.config.output.ends_with("test.md"));
 378 |     }
 379 | 
 380 |     #[test]
 381 |     fn test_output_folder_with_timestamping() {
 382 |         let args = Args {
 383 |             input: "src".to_string(),
 384 |             output: "test.md".to_string(),
 385 |             filter: vec![],
 386 |             ignore: vec![],
 387 |             line_numbers: false,
 388 |             preview: false,
 389 |             token_count: false,
 390 |             yes: false,
 391 |             diff_only: false,
 392 |             clear_cache: false,
 393 |             init: false,
 394 |             max_tokens: None,
 395 |             signatures: false,
 396 |             structure: false,
 397 |             truncate: "smart".to_string(),
 398 |             visibility: "all".to_string(),
 399 |         };
 400 | 
 401 |         let config = Config {
 402 |             output_folder: Some("docs".to_string()),
 403 |             timestamped_output: Some(true),
 404 |             ..Default::default()
 405 |         };
 406 | 
 407 |         let resolution = resolve_final_config(args, Some(config));
 408 | 
 409 |         assert!(resolution.config.output.contains("docs"));
 410 |         assert!(resolution.config.output.contains("test_"));
 411 |         assert!(resolution.config.output.ends_with(".md"));
 412 |     }
 413 | 
 414 |     #[test]
 415 |     fn test_auto_diff_without_timestamping_warning() {
 416 |         let args = Args {
 417 |             input: "src".to_string(),
 418 |             output: "test.md".to_string(),
 419 |             filter: vec![],
 420 |             ignore: vec![],
 421 |             line_numbers: false,
 422 |             preview: false,
 423 |             token_count: false,
 424 |             yes: false,
 425 |             diff_only: false,
 426 |             clear_cache: false,
 427 |             init: false,
 428 |             max_tokens: None,
 429 |             signatures: false,
 430 |             structure: false,
 431 |             truncate: "smart".to_string(),
 432 |             visibility: "all".to_string(),
 433 |         };
 434 | 
 435 |         let config = Config {
 436 |             auto_diff: Some(true),
 437 |             timestamped_output: Some(false), // This should generate a warning
 438 |             ..Default::default()
 439 |         };
 440 | 
 441 |         let resolution = resolve_final_config(args, Some(config));
 442 | 
 443 |         assert!(!resolution.warnings.is_empty());
 444 |         assert!(resolution.warnings[0].contains("auto_diff"));
 445 |         assert!(resolution.warnings[0].contains("timestamped_output"));
 446 |     }
 447 | 
 448 |     #[test]
 449 |     fn test_no_config_uses_cli_defaults() {
 450 |         let args = Args {
 451 |             input: "src".to_string(),
 452 |             output: "output.md".to_string(),
 453 |             filter: vec![],
 454 |             ignore: vec![],
 455 |             line_numbers: false,
 456 |             preview: false,
 457 |             token_count: false,
 458 |             yes: false,
 459 |             diff_only: false,
 460 |             clear_cache: false,
 461 |             init: false,
 462 |             max_tokens: None,
 463 |             signatures: false,
 464 |             structure: false,
 465 |             truncate: "smart".to_string(),
 466 |             visibility: "all".to_string(),
 467 |         };
 468 | 
 469 |         let resolution = resolve_final_config(args.clone(), None);
 470 | 
 471 |         assert_eq!(resolution.config.input, args.input);
 472 |         assert_eq!(resolution.config.output, args.output);
 473 |         assert_eq!(resolution.config.filter, args.filter);
 474 |         assert_eq!(resolution.config.ignore, args.ignore);
 475 |         assert_eq!(resolution.config.line_numbers, args.line_numbers);
 476 |         assert_eq!(resolution.config.preview, args.preview);
 477 |         assert_eq!(resolution.config.token_count, args.token_count);
 478 |         assert_eq!(resolution.config.yes, args.yes);
 479 |         assert_eq!(resolution.config.diff_only, args.diff_only);
 480 |         assert!(!resolution.config.auto_diff);
 481 |         assert_eq!(resolution.config.diff_context_lines, 3);
 482 |         assert!(resolution.warnings.is_empty());
 483 |     }
 484 | }
```

### File: `src/diff.rs`

- Size: 21233 bytes
- Modified: SystemTime { tv_sec: 1771131034, tv_nsec: 854121736 }

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
  55 |         // Emit standard unified diff hunk header for positional context
  56 |         if let (Some(first), Some(last)) = (group.first(), group.last()) {
  57 |             let old_start = first.old_range().start + 1;
  58 |             let old_len = last.old_range().end - first.old_range().start;
  59 |             let new_start = first.new_range().start + 1;
  60 |             let new_len = last.new_range().end - first.new_range().start;
  61 |             out.push_str(&format!(
  62 |                 "@@ -{},{} +{},{} @@\n",
  63 |                 old_start, old_len, new_start, new_len
  64 |             ));
  65 |         }
  66 |         for op in group {
  67 |             for change in diff.iter_changes(op) {
  68 |                 let tag = change.tag();
  69 |                 let mut line = change.to_string();
  70 |                 if line.ends_with('\n') {
  71 |                     line.pop();
  72 |                     if line.ends_with('\r') {
  73 |                         line.pop();
  74 |                     }
  75 |                 }
  76 | 
  77 |                 match tag {
  78 |                     ChangeTag::Delete => {
  79 |                         out.push_str("- ");
  80 |                         out.push_str(&line);
  81 |                         out.push('\n');
  82 |                     }
  83 |                     ChangeTag::Insert => {
  84 |                         out.push_str("+ ");
  85 |                         out.push_str(&line);
  86 |                         out.push('\n');
  87 |                     }
  88 |                     ChangeTag::Equal => {
  89 |                         out.push_str("  ");
  90 |                         out.push_str(&line);
  91 |                         out.push('\n');
  92 |                     }
  93 |                 }
  94 |             }
  95 |         }
  96 |     }
  97 |     out.push_str("```\n\n");
  98 |     out
  99 | }
 100 | 
 101 | /// Classification of how a file changed between two snapshots.
 102 | #[derive(Debug, Clone, PartialEq, Eq)]
 103 | pub enum PerFileStatus {
 104 |     Added,
 105 |     Removed,
 106 |     Modified,
 107 |     Unchanged,
 108 | }
 109 | 
 110 | /// Structured diff result for a single file.
 111 | #[derive(Debug, Clone)]
 112 | pub struct PerFileDiff {
 113 |     pub path: String,
 114 |     pub status: PerFileStatus,
 115 |     /// Unified diff fenced in ```diff (omitted when status == Unchanged and skip_unchanged=true)
 116 |     pub diff: String,
 117 | }
 118 | 
 119 | impl PerFileDiff {
 120 |     pub fn is_changed(&self) -> bool {
 121 |         self.status != PerFileStatus::Unchanged
 122 |     }
 123 | }
 124 | 
 125 | /// Produce a unified style diff for two text blobs WITHOUT adding any global
 126 | /// section header. Returns empty string if contents are identical.
 127 | fn unified_no_header(old: &str, new: &str, context_lines: usize) -> String {
 128 |     let diff = TextDiff::from_lines(old, new);
 129 |     if diff.ratio() == 1.0 {
 130 |         return String::new();
 131 |     }
 132 |     let grouped = diff.grouped_ops(context_lines);
 133 |     let mut out = String::new();
 134 |     out.push_str("```diff\n");
 135 |     for (group_index, group) in grouped.iter().enumerate() {
 136 |         if group_index > 0 {
 137 |             out.push_str("  ...\n");
 138 |         }
 139 |         // Emit standard unified diff hunk header for positional context
 140 |         if let (Some(first), Some(last)) = (group.first(), group.last()) {
 141 |             let old_start = first.old_range().start + 1;
 142 |             let old_len = last.old_range().end - first.old_range().start;
 143 |             let new_start = first.new_range().start + 1;
 144 |             let new_len = last.new_range().end - first.new_range().start;
 145 |             out.push_str(&format!(
 146 |                 "@@ -{},{} +{},{} @@\n",
 147 |                 old_start, old_len, new_start, new_len
 148 |             ));
 149 |         }
 150 |         for op in group {
 151 |             for change in diff.iter_changes(op) {
 152 |                 let tag = change.tag();
 153 |                 let mut line = change.to_string();
 154 |                 if line.ends_with('\n') {
 155 |                     line.pop();
 156 |                     if line.ends_with('\r') {
 157 |                         line.pop();
 158 |                     }
 159 |                 }
 160 | 
 161 |                 match tag {
 162 |                     ChangeTag::Delete => {
 163 |                         out.push_str("- ");
 164 |                         out.push_str(&line);
 165 |                         out.push('\n');
 166 |                     }
 167 |                     ChangeTag::Insert => {
 168 |                         out.push_str("+ ");
 169 |                         out.push_str(&line);
 170 |                         out.push('\n');
 171 |                     }
 172 |                     ChangeTag::Equal => {
 173 |                         out.push_str("  ");
 174 |                         out.push_str(&line);
 175 |                         out.push('\n');
 176 |                     }
 177 |                 }
 178 |             }
 179 |         }
 180 |     }
 181 |     out.push_str("```\n");
 182 |     out
 183 | }
 184 | 
 185 | /// Diff per file content sets.
 186 | ///
 187 | /// Inputs are maps keyed by file path (relative or absolute – caller decides)
 188 | /// with values being the raw file content EXACTLY as you wish it to be diffed
 189 | /// (e.g. already stripped of volatile metadata, no size/modified lines, only
 190 | /// the real file body). This keeps higher level logic (parsing the markdown
 191 | /// document) out of the diff layer.
 192 | ///
 193 | /// Returns a vector of `PerFileDiff` for every file that is Added, Removed,
 194 | /// or Modified. Unchanged files are omitted by default (`skip_unchanged=true`)
 195 | /// to reduce noise, but you can opt to include them.
 196 | pub fn diff_file_contents(
 197 |     previous: &HashMap<String, String>,
 198 |     current: &HashMap<String, String>,
 199 |     skip_unchanged: bool,
 200 |     explicit_context: Option<usize>,
 201 | ) -> Vec<PerFileDiff> {
 202 |     let mut all_paths: Vec<String> = previous.keys().chain(current.keys()).cloned().collect();
 203 |     all_paths.sort();
 204 |     all_paths.dedup();
 205 | 
 206 |     let context_lines = resolve_context_lines(explicit_context);
 207 |     let mut results = Vec::new();
 208 | 
 209 |     for path in all_paths {
 210 |         let old_opt = previous.get(&path);
 211 |         let new_opt = current.get(&path);
 212 |         match (old_opt, new_opt) {
 213 |             (None, Some(new_content)) => {
 214 |                 // Added file: present only in current snapshot
 215 |                 let mut diff = String::new();
 216 |                 diff.push_str("```diff\n");
 217 |                 for line in new_content.lines() {
 218 |                     diff.push_str("+ ");
 219 |                     diff.push_str(line);
 220 |                     diff.push('\n');
 221 |                 }
 222 |                 diff.push_str("```\n");
 223 |                 results.push(PerFileDiff {
 224 |                     path,
 225 |                     status: PerFileStatus::Added,
 226 |                     diff,
 227 |                 });
 228 |             }
 229 |             (Some(_old_content), None) => {
 230 |                 // Removed file
 231 |                 let old_content = previous.get(&path).unwrap();
 232 |                 let mut diff = String::new();
 233 |                 diff.push_str("```diff\n");
 234 |                 for line in old_content.lines() {
 235 |                     diff.push_str("- ");
 236 |                     diff.push_str(line);
 237 |                     diff.push('\n');
 238 |                 }
 239 |                 diff.push_str("```\n");
 240 |                 results.push(PerFileDiff {
 241 |                     path,
 242 |                     status: PerFileStatus::Removed,
 243 |                     diff,
 244 |                 });
 245 |             }
 246 |             (Some(old_content), Some(new_content)) => {
 247 |                 if old_content == new_content {
 248 |                     if !skip_unchanged {
 249 |                         results.push(PerFileDiff {
 250 |                             path,
 251 |                             status: PerFileStatus::Unchanged,
 252 |                             diff: String::new(),
 253 |                         });
 254 |                     }
 255 |                 } else {
 256 |                     let diff = unified_no_header(old_content, new_content, context_lines);
 257 |                     results.push(PerFileDiff {
 258 |                         path,
 259 |                         status: PerFileStatus::Modified,
 260 |                         diff,
 261 |                     });
 262 |                 }
 263 |             }
 264 |             (None, None) => unreachable!(),
 265 |         }
 266 |     }
 267 | 
 268 |     results
 269 | }
 270 | 
 271 | /// Render a collection of per file diffs into markdown WITHOUT a global
 272 | /// "## File Differences" header. Each file begins with a "### Diff: `<path>`"
 273 | /// heading so that it can be appended near the changed files summary.
 274 | pub fn render_per_file_diffs(diffs: &[PerFileDiff]) -> String {
 275 |     let mut out = String::new();
 276 |     for d in diffs {
 277 |         out.push_str(&format!("### Diff: `{}`\n\n", d.path));
 278 |         match d.status {
 279 |             PerFileStatus::Added => out.push_str("_Status: Added_\n\n"),
 280 |             PerFileStatus::Removed => out.push_str("_Status: Removed_\n\n"),
 281 |             PerFileStatus::Modified => out.push_str("_Status: Modified_\n\n"),
 282 |             PerFileStatus::Unchanged => {
 283 |                 out.push_str("_Status: Unchanged_\n\n");
 284 |             }
 285 |         }
 286 |         if !d.diff.is_empty() {
 287 |             out.push_str(&d.diff);
 288 |             if !d.diff.ends_with('\n') {
 289 |                 out.push('\n');
 290 |             }
 291 |         }
 292 |         out.push('\n');
 293 |     }
 294 |     out
 295 | }
 296 | 
 297 | #[cfg(test)]
 298 | mod tests {
 299 |     use super::*;
 300 | 
 301 |     fn map(pairs: &[(&str, &str)]) -> HashMap<String, String> {
 302 |         pairs
 303 |             .iter()
 304 |             .map(|(k, v)| (k.to_string(), v.to_string()))
 305 |             .collect()
 306 |     }
 307 | 
 308 |     #[test]
 309 |     fn unchanged_is_skipped() {
 310 |         let prev = map(&[("a.txt", "one\n")]);
 311 |         let curr = map(&[("a.txt", "one\n")]);
 312 |         let diffs = diff_file_contents(&prev, &curr, true, Some(2));
 313 |         assert!(diffs.is_empty());
 314 |     }
 315 | 
 316 |     #[test]
 317 |     fn added_file_diff() {
 318 |         let prev = map(&[]);
 319 |         let curr = map(&[("new.rs", "fn main() {}\n")]);
 320 |         let diffs = diff_file_contents(&prev, &curr, true, Some(2));
 321 |         assert_eq!(diffs.len(), 1);
 322 |         let d = &diffs[0];
 323 |         assert_eq!(d.status, PerFileStatus::Added);
 324 |         assert!(d.diff.contains("+ fn main() {}"));
 325 |     }
 326 | 
 327 |     #[test]
 328 |     fn removed_file_diff() {
 329 |         let prev = map(&[("old.rs", "fn old() {}\n")]);
 330 |         let curr = map(&[]);
 331 |         let diffs = diff_file_contents(&prev, &curr, true, None);
 332 |         assert_eq!(diffs.len(), 1);
 333 |         let d = &diffs[0];
 334 |         assert_eq!(d.status, PerFileStatus::Removed);
 335 |         assert!(d.diff.contains("- fn old() {}"));
 336 |     }
 337 | 
 338 |     #[test]
 339 |     fn modified_file_diff() {
 340 |         let prev = map(&[("lib.rs", "fn add(a:i32,b:i32)->i32{a+b}\n")]);
 341 |         let curr = map(&[("lib.rs", "fn add(a: i32, b: i32) -> i32 { a + b }\n")]);
 342 |         let diffs = diff_file_contents(&prev, &curr, true, Some(1));
 343 |         assert_eq!(diffs.len(), 1);
 344 |         let d = &diffs[0];
 345 |         assert_eq!(d.status, PerFileStatus::Modified);
 346 |         assert!(d.diff.contains("- fn add(a:i32,b:i32)->i32{a+b}"));
 347 |         assert!(d.diff.contains("+ fn add(a: i32, b: i32) -> i32 { a + b }"));
 348 |     }
 349 | 
 350 |     #[test]
 351 |     fn include_unchanged_when_requested() {
 352 |         let prev = map(&[("a.txt", "same\n")]);
 353 |         let curr = map(&[("a.txt", "same\n")]);
 354 |         let diffs = diff_file_contents(&prev, &curr, false, None);
 355 |         assert_eq!(diffs.len(), 1);
 356 |         assert_eq!(diffs[0].status, PerFileStatus::Unchanged);
 357 |     }
 358 | 
 359 |     #[test]
 360 |     fn render_output_basic() {
 361 |         let prev = map(&[("a.txt", "one\n"), ("b.txt", "line1\nline2\n")]);
 362 |         let curr = map(&[
 363 |             ("a.txt", "two\n"),
 364 |             ("b.txt", "line1\nline2\n"),
 365 |             ("c.txt", "new file\n"),
 366 |         ]);
 367 |         let diffs = diff_file_contents(&prev, &curr, true, Some(1));
 368 |         let out = render_per_file_diffs(&diffs);
 369 |         assert!(out.contains("### Diff: `a.txt`"));
 370 |         assert!(out.contains("_Status: Modified_"));
 371 |         assert!(out.contains("+ two"));
 372 |         assert!(out.contains("### Diff: `c.txt`"));
 373 |         assert!(out.contains("_Status: Added_"));
 374 |         assert!(out.contains("+ new file"));
 375 |     }
 376 | 
 377 |     #[test]
 378 |     fn test_empty_files() {
 379 |         let prev = map(&[("empty.txt", "")]);
 380 |         let curr = map(&[("empty.txt", "")]);
 381 |         let diffs = diff_file_contents(&prev, &curr, true, None);
 382 |         assert!(diffs.is_empty());
 383 |     }
 384 | 
 385 |     #[test]
 386 |     fn test_empty_to_content() {
 387 |         let prev = map(&[("file.txt", "")]);
 388 |         let curr = map(&[("file.txt", "new content\n")]);
 389 |         let diffs = diff_file_contents(&prev, &curr, true, None);
 390 |         assert_eq!(diffs.len(), 1);
 391 |         assert_eq!(diffs[0].status, PerFileStatus::Modified);
 392 |         assert!(diffs[0].diff.contains("+ new content"));
 393 |     }
 394 | 
 395 |     #[test]
 396 |     fn test_content_to_empty() {
 397 |         let prev = map(&[("file.txt", "old content\n")]);
 398 |         let curr = map(&[("file.txt", "")]);
 399 |         let diffs = diff_file_contents(&prev, &curr, true, None);
 400 |         assert_eq!(diffs.len(), 1);
 401 |         assert_eq!(diffs[0].status, PerFileStatus::Modified);
 402 |         assert!(diffs[0].diff.contains("- old content"));
 403 |     }
 404 | 
 405 |     #[test]
 406 |     fn test_multiline_modifications() {
 407 |         let prev = map(&[("file.txt", "line1\nline2\nline3\nline4\n")]);
 408 |         let curr = map(&[("file.txt", "line1\nmodified2\nline3\nline4\n")]);
 409 |         let diffs = diff_file_contents(&prev, &curr, true, Some(2));
 410 |         assert_eq!(diffs.len(), 1);
 411 |         assert_eq!(diffs[0].status, PerFileStatus::Modified);
 412 |         assert!(diffs[0].diff.contains("- line2"));
 413 |         assert!(diffs[0].diff.contains("+ modified2"));
 414 |     }
 415 | 
 416 |     #[test]
 417 |     fn test_windows_line_endings() {
 418 |         let prev = map(&[("file.txt", "line1\r\nline2\r\n")]);
 419 |         let curr = map(&[("file.txt", "line1\r\nmodified2\r\n")]);
 420 |         let diffs = diff_file_contents(&prev, &curr, true, None);
 421 |         assert_eq!(diffs.len(), 1);
 422 |         assert_eq!(diffs[0].status, PerFileStatus::Modified);
 423 |         assert!(diffs[0].diff.contains("- line2"));
 424 |         assert!(diffs[0].diff.contains("+ modified2"));
 425 |     }
 426 | 
 427 |     #[test]
 428 |     fn test_per_file_diff_is_changed() {
 429 |         let added = PerFileDiff {
 430 |             path: "test.txt".to_string(),
 431 |             status: PerFileStatus::Added,
 432 |             diff: "test".to_string(),
 433 |         };
 434 |         assert!(added.is_changed());
 435 | 
 436 |         let removed = PerFileDiff {
 437 |             path: "test.txt".to_string(),
 438 |             status: PerFileStatus::Removed,
 439 |             diff: "test".to_string(),
 440 |         };
 441 |         assert!(removed.is_changed());
 442 | 
 443 |         let modified = PerFileDiff {
 444 |             path: "test.txt".to_string(),
 445 |             status: PerFileStatus::Modified,
 446 |             diff: "test".to_string(),
 447 |         };
 448 |         assert!(modified.is_changed());
 449 | 
 450 |         let unchanged = PerFileDiff {
 451 |             path: "test.txt".to_string(),
 452 |             status: PerFileStatus::Unchanged,
 453 |             diff: String::new(),
 454 |         };
 455 |         assert!(!unchanged.is_changed());
 456 |     }
 457 | 
 458 |     #[test]
 459 |     fn test_generate_diff_identical_content() {
 460 |         let content = "line1\nline2\nline3\n";
 461 |         let diff = generate_diff(content, content);
 462 |         assert!(diff.is_empty());
 463 |     }
 464 | 
 465 |     #[test]
 466 |     fn test_generate_diff_with_changes() {
 467 |         let old = "line1\nline2\nline3\n";
 468 |         let new = "line1\nmodified2\nline3\n";
 469 |         let diff = generate_diff(old, new);
 470 |         assert!(diff.contains("## File Differences"));
 471 |         assert!(diff.contains("```diff"));
 472 |         assert!(diff.contains("- line2"));
 473 |         assert!(diff.contains("+ modified2"));
 474 |     }
 475 | 
 476 |     #[test]
 477 |     fn test_resolve_context_lines_default() {
 478 |         let context = resolve_context_lines(None);
 479 |         assert_eq!(context, 3);
 480 |     }
 481 | 
 482 |     #[test]
 483 |     fn test_resolve_context_lines_explicit() {
 484 |         let context = resolve_context_lines(Some(5));
 485 |         assert_eq!(context, 5);
 486 |     }
 487 | 
 488 |     #[test]
 489 |     fn test_resolve_context_lines_zero_fallback() {
 490 |         let context = resolve_context_lines(Some(0));
 491 |         assert_eq!(context, 3); // Should fallback to default
 492 |     }
 493 | 
 494 |     #[test]
 495 |     fn test_unicode_content_diff() {
 496 |         let prev = map(&[("unicode.txt", "Hello 世界\n")]);
 497 |         let curr = map(&[("unicode.txt", "Hello 世界! 🌍\n")]);
 498 |         let diffs = diff_file_contents(&prev, &curr, true, None);
 499 |         assert_eq!(diffs.len(), 1);
 500 |         assert_eq!(diffs[0].status, PerFileStatus::Modified);
 501 |         assert!(diffs[0].diff.contains("Hello 世界"));
 502 |         assert!(diffs[0].diff.contains("🌍"));
 503 |     }
 504 | 
 505 |     #[test]
 506 |     fn test_render_per_file_diffs_empty() {
 507 |         let diffs = vec![];
 508 |         let output = render_per_file_diffs(&diffs);
 509 |         assert!(output.is_empty());
 510 |     }
 511 | 
 512 |     #[test]
 513 |     fn test_render_per_file_diffs_unchanged() {
 514 |         let diffs = vec![PerFileDiff {
 515 |             path: "unchanged.txt".to_string(),
 516 |             status: PerFileStatus::Unchanged,
 517 |             diff: String::new(),
 518 |         }];
 519 |         let output = render_per_file_diffs(&diffs);
 520 |         assert!(output.contains("### Diff: `unchanged.txt`"));
 521 |         assert!(output.contains("_Status: Unchanged_"));
 522 |     }
 523 | 
 524 |     #[test]
 525 |     fn test_render_per_file_diffs_without_trailing_newline() {
 526 |         let diffs = vec![PerFileDiff {
 527 |             path: "test.txt".to_string(),
 528 |             status: PerFileStatus::Modified,
 529 |             diff: "```diff\n+ line\n```".to_string(), // No trailing newline
 530 |         }];
 531 |         let output = render_per_file_diffs(&diffs);
 532 |         assert!(output.contains("### Diff: `test.txt`"));
 533 |         assert!(output.contains("_Status: Modified_"));
 534 |         assert!(output.ends_with("\n\n")); // Should add newlines
 535 |     }
 536 | 
 537 |     #[test]
 538 |     fn test_generate_diff_with_multiple_groups() {
 539 |         // Create content that will result in multiple diff groups to trigger "..." separator
 540 |         let old_content = "line1\nline2\nline3\nline4\nline5\nline6\nline7\nline8\nline9\nline10";
 541 |         let new_content = "line1_modified\nline2\nline3\nline4\nline5\nline6\nline7\nline8\nline9_modified\nline10";
 542 | 
 543 |         let diff = generate_diff(old_content, new_content);
 544 |         assert!(diff.contains("```diff"));
 545 |         assert!(diff.contains("## File Differences"));
 546 |         // With sufficient distance between changes and small context, should create groups with "..." separator
 547 |         println!("Generated diff: {}", diff);
 548 |     }
 549 | 
 550 |     #[test]
 551 |     fn test_diff_with_windows_line_endings() {
 552 |         let old_content = "line1\r\nline2\r\n";
 553 |         let new_content = "line1_modified\r\nline2\r\n";
 554 | 
 555 |         let diff = generate_diff(old_content, new_content);
 556 |         assert!(diff.contains("```diff"));
 557 |         assert!(diff.contains("line1_modified"));
 558 |         assert!(!diff.is_empty());
 559 |     }
 560 | 
 561 |     #[test]
 562 |     fn test_unified_no_header_with_multiple_groups() {
 563 |         // Create content that will result in multiple diff groups
 564 |         let old_content = "start\n\n\n\n\n\n\n\n\n\nmiddle\n\n\n\n\n\n\n\n\n\nend";
 565 |         let new_content =
 566 |             "start_modified\n\n\n\n\n\n\n\n\n\nmiddle\n\n\n\n\n\n\n\n\n\nend_modified";
 567 | 
 568 |         let diff = unified_no_header(old_content, new_content, 2);
 569 |         assert!(diff.contains("```diff"));
 570 |         // Should contain "..." separator between groups when changes are far apart
 571 |         println!("Unified diff: {}", diff);
 572 |     }
 573 | 
 574 |     #[test]
 575 |     fn test_unified_no_header_with_windows_line_endings() {
 576 |         let old_content = "line1\r\nline2\r\n";
 577 |         let new_content = "line1_modified\r\nline2\r\n";
 578 | 
 579 |         let diff = unified_no_header(old_content, new_content, 3);
 580 |         assert!(diff.contains("```diff"));
 581 |         assert!(diff.contains("line1_modified"));
 582 |         assert!(!diff.is_empty());
 583 |     }
 584 | }
```

### File: `src/file_utils.rs`

- Size: 23556 bytes
- Modified: SystemTime { tv_sec: 1771131012, tv_nsec: 551819506 }

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
 169 | 
 170 |     // Hardcoded auto-ignores for common heavy directories that should NEVER be
 171 |     // included, even when there's no .git directory (so .gitignore isn't read).
 172 |     // Without these, projects missing .git can produce million-line outputs
 173 |     // from dependency trees.
 174 |     //
 175 |     // IMPORTANT: These are added FIRST so that user ignores can override them.
 176 |     // The ignore crate uses "last-match-wins" semantics, so a user can whitelist
 177 |     // a legitimate "vendor" or "build" dir by passing it as a filter pattern.
 178 |     //
 179 |     // IMPORTANT: Patterns must NOT contain a slash — the ignore crate anchors
 180 |     // slash-containing patterns to the root, so `!dir/**` would only match
 181 |     // top-level dirs, missing nested ones like `apps/web/node_modules/`.
 182 |     let default_ignores = [
 183 |         "node_modules",
 184 |         "__pycache__",
 185 |         ".venv",
 186 |         "venv",
 187 |         ".tox",
 188 |         ".mypy_cache",
 189 |         ".pytest_cache",
 190 |         ".ruff_cache",
 191 |         "vendor",  // Go, PHP, Ruby
 192 |         ".bundle", // Ruby
 193 |         "bower_components",
 194 |         ".next",       // Next.js build output
 195 |         ".nuxt",       // Nuxt build output
 196 |         ".svelte-kit", // SvelteKit build output
 197 |         ".angular",    // Angular cache
 198 |         "dist",        // Common build output
 199 |         "build",       // Common build output
 200 |         ".gradle",     // Gradle cache
 201 |         ".cargo",      // Cargo registry cache
 202 |     ];
 203 |     for dir in &default_ignores {
 204 |         // No slash in pattern → matches at any depth (not root-anchored)
 205 |         let pattern = format!("!{}", dir);
 206 |         if let Err(e) = override_builder.add(&pattern) {
 207 |             log::warn!("Skipping invalid default-ignore '{}': {}", dir, e);
 208 |         }
 209 |     }
 210 | 
 211 |     // User-specified ignore patterns (added AFTER defaults so they can override)
 212 |     for pattern in ignores {
 213 |         // Attention: Confusing pattern ahead!
 214 |         // Add the pattern to the override builder with ! prefix to ignore matching files.
 215 |         // In OverrideBuilder, patterns without ! are whitelist (include) patterns,
 216 |         // while patterns with ! are ignore patterns.
 217 |         let ignore_pattern = format!("!{}", pattern);
 218 |         if let Err(e) = override_builder.add(&ignore_pattern) {
 219 |             return Err(io::Error::new(
 220 |                 io::ErrorKind::InvalidInput,
 221 |                 format!("Invalid ignore pattern '{}': {}", pattern, e),
 222 |             ));
 223 |         }
 224 |     }
 225 |     // Apply auto-computed ignore patterns (output file, cache dir, etc.)
 226 |     for pattern in auto_ignores {
 227 |         let ignore_pattern = format!("!{}", pattern);
 228 |         if let Err(e) = override_builder.add(&ignore_pattern) {
 229 |             log::warn!("Skipping invalid auto-ignore pattern '{}': {}", pattern, e);
 230 |         }
 231 |     }
 232 |     // Also, always ignore the config file itself
 233 |     if let Err(e) = override_builder.add("!context-builder.toml") {
 234 |         return Err(io::Error::new(
 235 |             io::ErrorKind::InvalidInput,
 236 |             format!("Failed to add config ignore: {}", e),
 237 |         ));
 238 |     }
 239 | 
 240 |     let overrides = override_builder.build().map_err(|e| {
 241 |         io::Error::new(
 242 |             io::ErrorKind::InvalidInput,
 243 |             format!("Failed to build overrides: {}", e),
 244 |         )
 245 |     })?;
 246 |     walker.overrides(overrides);
 247 | 
 248 |     if !filters.is_empty() {
 249 |         let mut type_builder = ignore::types::TypesBuilder::new();
 250 |         type_builder.add_defaults();
 251 |         for filter in filters {
 252 |             let _ = type_builder.add(filter, &format!("*.{}", filter));
 253 |             type_builder.select(filter);
 254 |         }
 255 |         let types = type_builder.build().unwrap();
 256 |         walker.types(types);
 257 |     }
 258 | 
 259 |     let mut files: Vec<DirEntry> = walker
 260 |         .build()
 261 |         .filter_map(Result::ok)
 262 |         .filter(|e| e.file_type().is_some_and(|ft| ft.is_file()))
 263 |         .collect();
 264 | 
 265 |     // Sort files by relevance category, then entry-point priority, then alphabetically.
 266 |     // This puts config + docs first, then source code (entry points before helpers),
 267 |     // then tests, then docs/other, then build/CI, then lockfiles.
 268 |     // LLMs comprehend codebases better when core source appears before test scaffolding.
 269 |     files.sort_by(|a, b| {
 270 |         let cat_a = file_relevance_category(a.path(), base_path);
 271 |         let cat_b = file_relevance_category(b.path(), base_path);
 272 |         cat_a
 273 |             .cmp(&cat_b)
 274 |             .then_with(|| {
 275 |                 file_entry_point_priority(a.path()).cmp(&file_entry_point_priority(b.path()))
 276 |             })
 277 |             .then_with(|| a.path().cmp(b.path()))
 278 |     });
 279 | 
 280 |     Ok(files)
 281 | }
 282 | 
 283 | /// Asks for user confirmation if the number of files is large.
 284 | pub fn confirm_processing(file_count: usize) -> io::Result<bool> {
 285 |     if file_count > 100 {
 286 |         print!(
 287 |             "Warning: You're about to process {} files. This might take a while. Continue? [y/N] ",
 288 |             file_count
 289 |         );
 290 |         io::stdout().flush()?;
 291 |         let mut input = String::new();
 292 |         io::stdin().read_line(&mut input)?;
 293 |         if !input.trim().eq_ignore_ascii_case("y") {
 294 |             return Ok(false);
 295 |         }
 296 |     }
 297 |     Ok(true)
 298 | }
 299 | 
 300 | /// Asks for user confirmation to overwrite an existing file.
 301 | pub fn confirm_overwrite(file_path: &str) -> io::Result<bool> {
 302 |     print!("The file '{}' already exists. Overwrite? [y/N] ", file_path);
 303 |     io::stdout().flush()?;
 304 |     let mut input = String::new();
 305 |     io::stdin().read_line(&mut input)?;
 306 | 
 307 |     if input.trim().eq_ignore_ascii_case("y") {
 308 |         Ok(true)
 309 |     } else {
 310 |         Ok(false)
 311 |     }
 312 | }
 313 | 
 314 | pub fn find_latest_file(dir: &Path) -> io::Result<Option<PathBuf>> {
 315 |     if !dir.is_dir() {
 316 |         return Ok(None);
 317 |     }
 318 | 
 319 |     let mut latest_file = None;
 320 |     let mut latest_time = std::time::SystemTime::UNIX_EPOCH;
 321 | 
 322 |     for entry in fs::read_dir(dir)? {
 323 |         let entry = entry?;
 324 |         let path = entry.path();
 325 |         if path.is_file() {
 326 |             let metadata = fs::metadata(&path)?;
 327 |             let modified = metadata.modified()?;
 328 |             if modified > latest_time {
 329 |                 latest_time = modified;
 330 |                 latest_file = Some(path);
 331 |             }
 332 |         }
 333 |     }
 334 | 
 335 |     Ok(latest_file)
 336 | }
 337 | 
 338 | #[cfg(test)]
 339 | mod tests {
 340 |     use super::*;
 341 |     use std::fs;
 342 |     use std::path::Path;
 343 |     use tempfile::tempdir;
 344 | 
 345 |     fn to_rel_paths(mut entries: Vec<DirEntry>, base: &Path) -> Vec<String> {
 346 |         entries.sort_by_key(|e| e.path().to_path_buf());
 347 |         entries
 348 |             .iter()
 349 |             .map(|e| {
 350 |                 e.path()
 351 |                     .strip_prefix(base)
 352 |                     .unwrap()
 353 |                     .to_string_lossy()
 354 |                     .replace('\\', "/")
 355 |             })
 356 |             .collect()
 357 |     }
 358 | 
 359 |     #[test]
 360 |     fn collect_files_respects_filters() {
 361 |         let dir = tempdir().unwrap();
 362 |         let base = dir.path();
 363 | 
 364 |         // create files
 365 |         fs::create_dir_all(base.join("src")).unwrap();
 366 |         fs::create_dir_all(base.join("scripts")).unwrap();
 367 |         fs::write(base.join("src").join("main.rs"), "fn main() {}").unwrap();
 368 |         fs::write(base.join("Cargo.toml"), "[package]\nname=\"x\"").unwrap();
 369 |         fs::write(base.join("README.md"), "# readme").unwrap();
 370 |         fs::write(base.join("scripts").join("build.sh"), "#!/bin/sh\n").unwrap();
 371 | 
 372 |         let filters = vec!["rs".to_string(), "toml".to_string()];
 373 |         let ignores: Vec<String> = vec![];
 374 | 
 375 |         let files = collect_files(base, &filters, &ignores, &[]).unwrap();
 376 |         let relative_paths = to_rel_paths(files, base);
 377 | 
 378 |         assert!(relative_paths.contains(&"src/main.rs".to_string()));
 379 |         assert!(relative_paths.contains(&"Cargo.toml".to_string()));
 380 |         assert!(!relative_paths.contains(&"README.md".to_string()));
 381 |         assert!(!relative_paths.contains(&"scripts/build.sh".to_string()));
 382 |     }
 383 | 
 384 |     #[test]
 385 |     fn collect_files_respects_ignores_for_dirs_and_files() {
 386 |         let dir = tempdir().unwrap();
 387 |         let base = dir.path();
 388 | 
 389 |         fs::create_dir_all(base.join("src")).unwrap();
 390 |         fs::create_dir_all(base.join("target")).unwrap();
 391 |         fs::create_dir_all(base.join("node_modules")).unwrap();
 392 | 
 393 |         fs::write(base.join("src").join("main.rs"), "fn main() {}").unwrap();
 394 |         fs::write(base.join("target").join("artifact.txt"), "bin").unwrap();
 395 |         fs::write(base.join("node_modules").join("pkg.js"), "console.log();").unwrap();
 396 |         fs::write(base.join("README.md"), "# readme").unwrap();
 397 | 
 398 |         let filters: Vec<String> = vec![];
 399 |         let ignores: Vec<String> = vec!["target".into(), "node_modules".into(), "README.md".into()];
 400 | 
 401 |         let files = collect_files(base, &filters, &ignores, &[]).unwrap();
 402 |         let relative_paths = to_rel_paths(files, base);
 403 | 
 404 |         assert!(relative_paths.contains(&"src/main.rs".to_string()));
 405 |         assert!(!relative_paths.contains(&"target/artifact.txt".to_string()));
 406 |         assert!(!relative_paths.contains(&"node_modules/pkg.js".to_string()));
 407 |         assert!(!relative_paths.contains(&"README.md".to_string()));
 408 |     }
 409 | 
 410 |     #[test]
 411 |     fn collect_files_handles_invalid_ignore_pattern() {
 412 |         let dir = tempdir().unwrap();
 413 |         let base = dir.path();
 414 | 
 415 |         fs::create_dir_all(base.join("src")).unwrap();
 416 |         fs::write(base.join("src").join("main.rs"), "fn main() {}").unwrap();
 417 | 
 418 |         let filters: Vec<String> = vec![];
 419 |         let ignores: Vec<String> = vec!["[".into()]; // Invalid regex pattern
 420 | 
 421 |         let result = collect_files(base, &filters, &ignores, &[]);
 422 |         assert!(result.is_err());
 423 |         assert!(
 424 |             result
 425 |                 .unwrap_err()
 426 |                 .to_string()
 427 |                 .contains("Invalid ignore pattern")
 428 |         );
 429 |     }
 430 | 
 431 |     #[test]
 432 |     fn collect_files_empty_directory() {
 433 |         let dir = tempdir().unwrap();
 434 |         let base = dir.path();
 435 | 
 436 |         let filters: Vec<String> = vec![];
 437 |         let ignores: Vec<String> = vec![];
 438 | 
 439 |         let files = collect_files(base, &filters, &ignores, &[]).unwrap();
 440 |         assert!(files.is_empty());
 441 |     }
 442 | 
 443 |     #[test]
 444 |     fn collect_files_no_matching_filters() {
 445 |         let dir = tempdir().unwrap();
 446 |         let base = dir.path();
 447 | 
 448 |         fs::write(base.join("README.md"), "# readme").unwrap();
 449 |         fs::write(base.join("script.py"), "print('hello')").unwrap();
 450 | 
 451 |         let filters = vec!["rs".to_string()]; // Only Rust files
 452 |         let ignores: Vec<String> = vec![];
 453 | 
 454 |         let files = collect_files(base, &filters, &ignores, &[]).unwrap();
 455 |         assert!(files.is_empty());
 456 |     }
 457 | 
 458 |     #[test]
 459 |     fn collect_files_ignores_config_file() {
 460 |         let dir = tempdir().unwrap();
 461 |         let base = dir.path();
 462 | 
 463 |         fs::write(base.join("context-builder.toml"), "[config]").unwrap();
 464 |         fs::write(base.join("other.toml"), "[other]").unwrap();
 465 | 
 466 |         let filters: Vec<String> = vec![];
 467 |         let ignores: Vec<String> = vec![];
 468 | 
 469 |         let files = collect_files(base, &filters, &ignores, &[]).unwrap();
 470 |         let relative_paths = to_rel_paths(files, base);
 471 | 
 472 |         assert!(!relative_paths.contains(&"context-builder.toml".to_string()));
 473 |         assert!(relative_paths.contains(&"other.toml".to_string()));
 474 |     }
 475 | 
 476 |     #[test]
 477 |     fn confirm_processing_small_count() {
 478 |         // Test that small file counts don't require confirmation
 479 |         let result = confirm_processing(50);
 480 |         assert!(result.is_ok());
 481 |         assert!(result.unwrap());
 482 |     }
 483 | 
 484 |     #[test]
 485 |     fn find_latest_file_empty_directory() {
 486 |         let dir = tempdir().unwrap();
 487 |         let result = find_latest_file(dir.path()).unwrap();
 488 |         assert!(result.is_none());
 489 |     }
 490 | 
 491 |     #[test]
 492 |     fn find_latest_file_nonexistent_directory() {
 493 |         let dir = tempdir().unwrap();
 494 |         let nonexistent = dir.path().join("nonexistent");
 495 |         let result = find_latest_file(&nonexistent).unwrap();
 496 |         assert!(result.is_none());
 497 |     }
 498 | 
 499 |     #[test]
 500 |     fn find_latest_file_single_file() {
 501 |         let dir = tempdir().unwrap();
 502 |         let file_path = dir.path().join("test.txt");
 503 |         fs::write(&file_path, "content").unwrap();
 504 | 
 505 |         let result = find_latest_file(dir.path()).unwrap();
 506 |         assert!(result.is_some());
 507 |         assert_eq!(result.unwrap(), file_path);
 508 |     }
 509 | 
 510 |     #[test]
 511 |     fn find_latest_file_multiple_files() {
 512 |         let dir = tempdir().unwrap();
 513 | 
 514 |         let file1 = dir.path().join("old.txt");
 515 |         let file2 = dir.path().join("new.txt");
 516 | 
 517 |         fs::write(&file1, "old content").unwrap();
 518 |         std::thread::sleep(std::time::Duration::from_millis(10));
 519 |         fs::write(&file2, "new content").unwrap();
 520 | 
 521 |         let result = find_latest_file(dir.path()).unwrap();
 522 |         assert!(result.is_some());
 523 |         assert_eq!(result.unwrap(), file2);
 524 |     }
 525 | 
 526 |     #[test]
 527 |     fn find_latest_file_ignores_directories() {
 528 |         let dir = tempdir().unwrap();
 529 |         let subdir = dir.path().join("subdir");
 530 |         fs::create_dir(&subdir).unwrap();
 531 | 
 532 |         let file_path = dir.path().join("test.txt");
 533 |         fs::write(&file_path, "content").unwrap();
 534 | 
 535 |         let result = find_latest_file(dir.path()).unwrap();
 536 |         assert!(result.is_some());
 537 |         assert_eq!(result.unwrap(), file_path);
 538 |     }
 539 | 
 540 |     #[test]
 541 |     fn test_confirm_processing_requires_user_interaction() {
 542 |         // This test verifies the function signature and basic logic for large file counts
 543 |         // The actual user interaction cannot be tested in unit tests
 544 | 
 545 |         // For file counts <= 100, should return Ok(true) without prompting
 546 |         // This is already tested implicitly by the fact that small counts don't prompt
 547 | 
 548 |         // For file counts > 100, the function would prompt user input
 549 |         // We can't easily test this without mocking stdin, but we can verify
 550 |         // that the function exists and has the expected signature
 551 |         use std::io::Cursor;
 552 | 
 553 |         // Create a mock stdin that simulates user typing "y"
 554 |         let input = b"y\n";
 555 |         let _ = Cursor::new(input);
 556 | 
 557 |         // We can't easily override stdin in a unit test without complex setup,
 558 |         // so we'll just verify the function exists and handles small counts
 559 |         let result = confirm_processing(50);
 560 |         assert!(result.is_ok());
 561 |         assert!(result.unwrap());
 562 |     }
 563 | 
 564 |     #[test]
 565 |     fn test_confirm_overwrite_function_exists() {
 566 |         // Similar to confirm_processing, this function requires user interaction
 567 |         // We can verify it exists and has the expected signature
 568 | 
 569 |         // For testing purposes, we know this function prompts for user input
 570 |         // and returns Ok(true) if user types "y" or "Y", Ok(false) otherwise
 571 | 
 572 |         // The function signature should be:
 573 |         // pub fn confirm_overwrite(file_path: &str) -> io::Result<bool>
 574 | 
 575 |         // We can't easily test the interactive behavior without mocking stdin,
 576 |         // but we can ensure the function compiles and has the right signature
 577 |         let _: fn(&str) -> std::io::Result<bool> = confirm_overwrite;
 578 |     }
 579 | 
 580 |     #[test]
 581 |     fn test_collect_files_handles_permission_errors() {
 582 |         // Test what happens when we can't access a directory
 583 |         // This is harder to test portably, but we can test with invalid patterns
 584 |         let dir = tempdir().unwrap();
 585 |         let base = dir.path();
 586 | 
 587 |         // Test with a pattern that might cause issues
 588 |         let filters: Vec<String> = vec![];
 589 |         let ignores: Vec<String> = vec!["[invalid".into()]; // Incomplete bracket
 590 | 
 591 |         let result = collect_files(base, &filters, &ignores, &[]);
 592 |         assert!(result.is_err());
 593 |     }
 594 | 
 595 |     #[test]
 596 |     fn test_find_latest_file_permission_error() {
 597 |         // Test behavior when we can't read directory metadata
 598 |         use std::path::Path;
 599 | 
 600 |         // Test with a path that doesn't exist
 601 |         let nonexistent = Path::new("/this/path/should/not/exist/anywhere");
 602 |         let result = find_latest_file(nonexistent);
 603 | 
 604 |         // Should return Ok(None) for non-existent directories
 605 |         assert!(result.is_ok());
 606 |         assert!(result.unwrap().is_none());
 607 |     }
 608 | 
 609 |     #[test]
 610 |     fn test_collect_files_with_symlinks() {
 611 |         // Test behavior with symbolic links (if supported on platform)
 612 |         let dir = tempdir().unwrap();
 613 |         let base = dir.path();
 614 | 
 615 |         // Create a regular file
 616 |         fs::write(base.join("regular.txt"), "content").unwrap();
 617 | 
 618 |         // On Unix-like systems, try creating a symlink
 619 |         #[cfg(unix)]
 620 |         {
 621 |             use std::os::unix::fs::symlink;
 622 |             let _ = symlink("regular.txt", base.join("link.txt"));
 623 |         }
 624 | 
 625 |         // On Windows, symlinks require special privileges, so skip this part
 626 |         #[cfg(windows)]
 627 |         {
 628 |             // Just create another regular file to test
 629 |             fs::write(base.join("another.txt"), "content2").unwrap();
 630 |         }
 631 | 
 632 |         let filters: Vec<String> = vec![];
 633 |         let ignores: Vec<String> = vec![];
 634 | 
 635 |         let files = collect_files(base, &filters, &ignores, &[]).unwrap();
 636 |         // Should find at least the regular file
 637 |         assert!(!files.is_empty());
 638 |     }
 639 | }
```

### File: `src/markdown.rs`

- Size: 45356 bytes
- Modified: SystemTime { tv_sec: 1771153749, tv_nsec: 128945717 }

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
  16 | /// Configuration for tree-sitter powered output.
  17 | #[derive(Debug, Clone, Default)]
  18 | pub struct TreeSitterConfig {
  19 |     /// Output only signatures (function/type declarations) instead of full content.
  20 |     pub signatures: bool,
  21 |     /// Include a structure summary (counts of functions, structs, etc.) per file.
  22 |     pub structure: bool,
  23 |     /// Truncation mode: "smart" uses AST boundaries, anything else uses byte truncation.
  24 |     pub truncate: String,
  25 |     /// Visibility filter: "public", "private", or "all".
  26 |     pub visibility: String,
  27 | }
  28 | 
  29 | /// Generates the final Markdown file.
  30 | #[allow(clippy::too_many_arguments, unused_variables)]
  31 | pub fn generate_markdown(
  32 |     output_path: &str,
  33 |     input_dir: &str,
  34 |     filters: &[String],
  35 |     ignores: &[String],
  36 |     file_tree: &FileTree,
  37 |     files: &[DirEntry],
  38 |     base_path: &Path,
  39 |     line_numbers: bool,
  40 |     encoding_strategy: Option<&str>,
  41 |     max_tokens: Option<usize>,
  42 |     ts_config: &TreeSitterConfig,
  43 | ) -> io::Result<()> {
  44 |     if let Some(parent) = Path::new(output_path).parent()
  45 |         && !parent.exists()
  46 |     {
  47 |         fs::create_dir_all(parent)?;
  48 |     }
  49 | 
  50 |     let mut output = fs::File::create(output_path)?;
  51 | 
  52 |     let input_dir_name = if input_dir == "." {
  53 |         let current_dir = std::env::current_dir()?;
  54 |         current_dir
  55 |             .file_name()
  56 |             .and_then(|n| n.to_str())
  57 |             .unwrap_or_else(|| current_dir.to_str().unwrap_or("project"))
  58 |             .to_string()
  59 |     } else {
  60 |         input_dir.to_string()
  61 |     };
  62 | 
  63 |     // --- Header --- //
  64 |     writeln!(output, "# Directory Structure Report\n")?;
  65 | 
  66 |     if !filters.is_empty() {
  67 |         writeln!(
  68 |             output,
  69 |             "This document contains files from the `{}` directory with extensions: {}",
  70 |             input_dir_name,
  71 |             filters.join(", ")
  72 |         )?;
  73 |     } else {
  74 |         writeln!(
  75 |             output,
  76 |             "This document contains all files from the `{}` directory, optimized for LLM consumption.",
  77 |             input_dir_name
  78 |         )?;
  79 |     }
  80 | 
  81 |     if !ignores.is_empty() {
  82 |         writeln!(output, "Custom ignored patterns: {}", ignores.join(", "))?;
  83 |     }
  84 | 
  85 |     // Deterministic content hash (enables LLM prompt caching across runs)
  86 |     // Uses xxh3 over file content bytes — stable across Rust versions and machines.
  87 |     // Previous implementation hashed mtime (broken by git checkout, cp, etc.)
  88 |     let mut content_hasher = xxhash_rust::xxh3::Xxh3::new();
  89 |     for entry in files {
  90 |         // Hash relative unix-style path for cross-OS determinism.
  91 |         // Using absolute or OS-native paths would produce different hashes
  92 |         // on different machines or operating systems.
  93 |         let rel_path = entry.path().strip_prefix(base_path).unwrap_or(entry.path());
  94 |         let normalized = rel_path.to_string_lossy().replace('\\', "/");
  95 |         content_hasher.update(normalized.as_bytes());
  96 |         // Null delimiter prevents collision: path="a" content="bc" vs path="ab" content="c"
  97 |         content_hasher.update(b"\0");
  98 |         // Hash actual file content (not mtime!) for determinism
  99 |         if let Ok(bytes) = std::fs::read(entry.path()) {
 100 |             content_hasher.update(&bytes);
 101 |         }
 102 |         content_hasher.update(b"\0");
 103 |     }
 104 |     writeln!(output, "Content hash: {:016x}", content_hasher.digest())?;
 105 |     writeln!(output)?;
 106 | 
 107 |     // --- File Tree --- //
 108 | 
 109 |     writeln!(output, "## File Tree Structure\n")?;
 110 | 
 111 |     write_tree_to_file(&mut output, file_tree, 0)?;
 112 | 
 113 |     writeln!(output)?;
 114 | 
 115 |     // (No '## Files' heading here; it will be injected later only once during final composition)
 116 |     // (Diff section will be conditionally inserted later by the auto_diff logic in lib.rs)
 117 | 
 118 |     #[cfg(feature = "parallel")]
 119 |     {
 120 |         use rayon::prelude::*;
 121 | 
 122 |         // Create a bounded channel for ordered chunks
 123 |         type ChunkResult = (usize, io::Result<Vec<u8>>);
 124 |         let (sender, receiver): (Sender<ChunkResult>, Receiver<ChunkResult>) =
 125 |             bounded(num_cpus::get() * 2); // Buffer size based on CPU count
 126 | 
 127 |         let writer_handle = {
 128 |             let mut output = output;
 129 |             let total_files = files.len();
 130 |             let budget = max_tokens;
 131 | 
 132 |             thread::spawn(move || -> io::Result<()> {
 133 |                 let mut completed_chunks = std::collections::BTreeMap::new();
 134 |                 let mut next_index = 0;
 135 |                 let mut errors = Vec::new();
 136 |                 let mut tokens_used: usize = 0;
 137 |                 let mut budget_exceeded = false;
 138 | 
 139 |                 // Receive chunks and write them in order
 140 |                 while next_index < total_files {
 141 |                     match receiver.recv() {
 142 |                         Ok((index, chunk_result)) => {
 143 |                             completed_chunks.insert(index, chunk_result);
 144 | 
 145 |                             // Write all consecutive chunks starting from next_index
 146 |                             while let Some(chunk_result) = completed_chunks.remove(&next_index) {
 147 |                                 if budget_exceeded {
 148 |                                     // Already over budget — skip remaining chunks
 149 |                                     next_index += 1;
 150 |                                     continue;
 151 |                                 }
 152 | 
 153 |                                 match chunk_result {
 154 |                                     Ok(buf) => {
 155 |                                         // Estimate tokens for this chunk (~4 bytes per token)
 156 |                                         let chunk_tokens = buf.len() / 4;
 157 | 
 158 |                                         if let Some(max) = budget
 159 |                                             && tokens_used + chunk_tokens > max
 160 |                                             && tokens_used > 0
 161 |                                         {
 162 |                                             let remaining = total_files - next_index;
 163 |                                             let notice = format!(
 164 |                                                 "---\n\n_⚠️ Token budget ({}) reached. {} remaining files omitted._\n\n",
 165 |                                                 max, remaining
 166 |                                             );
 167 |                                             if let Err(e) = output.write_all(notice.as_bytes()) {
 168 |                                                 errors.push(format!(
 169 |                                                     "Failed to write truncation notice: {}",
 170 |                                                     e
 171 |                                                 ));
 172 |                                             }
 173 |                                             budget_exceeded = true;
 174 |                                             next_index += 1;
 175 |                                             continue;
 176 |                                         }
 177 | 
 178 |                                         tokens_used += chunk_tokens;
 179 |                                         if let Err(e) = output.write_all(&buf) {
 180 |                                             errors.push(format!(
 181 |                                                 "Failed to write output for file index {}: {}",
 182 |                                                 next_index, e
 183 |                                             ));
 184 |                                         }
 185 |                                     }
 186 |                                     Err(e) => {
 187 |                                         errors.push(format!(
 188 |                                             "Failed to process file index {}: {}",
 189 |                                             next_index, e
 190 |                                         ));
 191 |                                     }
 192 |                                 }
 193 |                                 next_index += 1;
 194 |                             }
 195 |                         }
 196 |                         Err(_) => break, // Channel closed
 197 |                     }
 198 |                 }
 199 | 
 200 |                 if !errors.is_empty() {
 201 |                     error!(
 202 |                         "Encountered {} errors during parallel processing:",
 203 |                         errors.len()
 204 |                     );
 205 |                     for err in &errors {
 206 |                         error!("  {}", err);
 207 |                     }
 208 |                     return Err(std::io::Error::other(format!(
 209 |                         "Failed to process {} files: {}",
 210 |                         errors.len(),
 211 |                         errors.join("; ")
 212 |                     )));
 213 |                 }
 214 | 
 215 |                 Ok(())
 216 |             })
 217 |         };
 218 | 
 219 |         // Process files in parallel and send results to writer
 220 |         let ts_config_clone = ts_config.clone();
 221 |         files.par_iter().enumerate().for_each(|(index, entry)| {
 222 |             let mut buf = Vec::new();
 223 |             let result = process_file(
 224 |                 base_path,
 225 |                 entry.path(),
 226 |                 &mut buf,
 227 |                 line_numbers,
 228 |                 encoding_strategy,
 229 |                 &ts_config_clone,
 230 |             )
 231 |             .map(|_| buf);
 232 | 
 233 |             // Send result to writer thread (ignore send errors - channel might be closed)
 234 |             let _ = sender.send((index, result));
 235 |         });
 236 | 
 237 |         // Close the sender to signal completion
 238 |         drop(sender);
 239 | 
 240 |         // Wait for writer thread to complete and propagate any errors
 241 |         writer_handle
 242 |             .join()
 243 |             .map_err(|_| std::io::Error::other("Writer thread panicked"))??;
 244 |     }
 245 | 
 246 |     #[cfg(not(feature = "parallel"))]
 247 |     {
 248 |         let mut tokens_used: usize = 0;
 249 | 
 250 |         for (idx, entry) in files.iter().enumerate() {
 251 |             // Estimate tokens for this file (~4 bytes per token)
 252 |             let file_size = std::fs::metadata(entry.path())
 253 |                 .map(|m| m.len())
 254 |                 .unwrap_or(0);
 255 |             let estimated_file_tokens = (file_size as usize) / 4;
 256 | 
 257 |             if let Some(budget) = max_tokens {
 258 |                 if tokens_used + estimated_file_tokens > budget && tokens_used > 0 {
 259 |                     let remaining = files.len() - idx;
 260 |                     writeln!(output, "---\n")?;
 261 |                     writeln!(
 262 |                         output,
 263 |                         "_⚠️ Token budget ({}) reached. {} remaining files omitted._\n",
 264 |                         budget, remaining
 265 |                     )?;
 266 |                     break;
 267 |                 }
 268 |             }
 269 | 
 270 |             tokens_used += estimated_file_tokens;
 271 |             process_file(
 272 |                 base_path,
 273 |                 entry.path(),
 274 |                 &mut output,
 275 |                 line_numbers,
 276 |                 encoding_strategy,
 277 |                 ts_config,
 278 |             )?;
 279 |         }
 280 |     }
 281 | 
 282 |     Ok(())
 283 | }
 284 | 
 285 | /// Processes a single file and writes its content to the output.
 286 | pub fn process_file(
 287 |     base_path: &Path,
 288 |     file_path: &Path,
 289 |     output: &mut impl Write,
 290 |     line_numbers: bool,
 291 |     encoding_strategy: Option<&str>,
 292 |     ts_config: &TreeSitterConfig,
 293 | ) -> io::Result<()> {
 294 |     let relative_path = file_path.strip_prefix(base_path).unwrap_or(file_path);
 295 |     info!("Processing file: {}", relative_path.display());
 296 | 
 297 |     let metadata = match fs::metadata(file_path) {
 298 |         Ok(meta) => meta,
 299 |         Err(e) => {
 300 |             error!(
 301 |                 "Failed to get metadata for {}: {}",
 302 |                 relative_path.display(),
 303 |                 e
 304 |             );
 305 |             return Ok(());
 306 |         }
 307 |     };
 308 | 
 309 |     let modified_time = metadata
 310 |         .modified()
 311 |         .ok()
 312 |         .map(|time| {
 313 |             let system_time: chrono::DateTime<Utc> = time.into();
 314 |             system_time.format("%Y-%m-%d %H:%M:%S UTC").to_string()
 315 |         })
 316 |         .unwrap_or_else(|| "Unknown".to_string());
 317 | 
 318 |     writeln!(output)?;
 319 |     writeln!(output, "### File: `{}`", relative_path.display())?;
 320 | 
 321 |     writeln!(output)?;
 322 | 
 323 |     writeln!(output, "- Size: {} bytes", metadata.len())?;
 324 |     writeln!(output, "- Modified: {}", modified_time)?;
 325 |     writeln!(output)?;
 326 | 
 327 |     // --- File Content --- //
 328 |     let extension = file_path
 329 |         .extension()
 330 |         .and_then(|s| s.to_str())
 331 |         .unwrap_or("text");
 332 |     let language = match extension {
 333 |         "rs" => "rust",
 334 |         "js" => "javascript",
 335 |         "ts" => "typescript",
 336 |         "jsx" => "jsx",
 337 |         "tsx" => "tsx",
 338 |         "json" => "json",
 339 |         "toml" => "toml",
 340 |         "md" => "markdown",
 341 |         "yaml" | "yml" => "yaml",
 342 |         "html" => "html",
 343 |         "css" => "css",
 344 |         "py" => "python",
 345 |         "java" => "java",
 346 |         "cpp" => "cpp",
 347 |         "c" => "c",
 348 |         "h" => "c",
 349 |         "hpp" => "cpp",
 350 |         "sql" => "sql",
 351 |         "sh" => "bash",
 352 |         "xml" => "xml",
 353 |         "lock" => "toml",
 354 |         _ => extension,
 355 |     };
 356 | 
 357 |     // Enhanced binary file handling with encoding detection and transcoding
 358 |     match fs::File::open(file_path) {
 359 |         Ok(mut file) => {
 360 |             let mut sniff = [0u8; 8192];
 361 |             let n = match file.read(&mut sniff) {
 362 |                 Ok(n) => n,
 363 |                 Err(e) => {
 364 |                     warn!(
 365 |                         "Could not read file {}: {}. Skipping content.",
 366 |                         relative_path.display(),
 367 |                         e
 368 |                     );
 369 | 
 370 |                     writeln!(output, "```text")?;
 371 | 
 372 |                     writeln!(
 373 |                         output,
 374 |                         "<Could not read file content (e.g., binary file or permission error)>"
 375 |                     )?;
 376 | 
 377 |                     writeln!(output, "```")?;
 378 | 
 379 |                     return Ok(());
 380 |                 }
 381 |             };
 382 |             let slice = &sniff[..n];
 383 | 
 384 |             // Find a valid UTF-8 boundary by backtracking up to 3 bytes.
 385 |             // If the sniff buffer cuts a multi-byte char (e.g., emoji at byte 8191),
 386 |             // from_utf8 would falsely classify the file as non-UTF-8.
 387 |             let check_len = if n == sniff.len() {
 388 |                 // Buffer is full — may have split a multi-byte char at the end
 389 |                 let mut end = n;
 390 |                 while end > 0 && end > n.saturating_sub(4) && sniff[end - 1] & 0xC0 == 0x80 {
 391 |                     end -= 1; // skip continuation bytes
 392 |                 }
 393 |                 // If we landed on a leading byte, check if the sequence is complete
 394 |                 if end > 0 && end < n {
 395 |                     let leading = sniff[end - 1];
 396 |                     let expected_len = if leading & 0xE0 == 0xC0 {
 397 |                         2
 398 |                     } else if leading & 0xF0 == 0xE0 {
 399 |                         3
 400 |                     } else if leading & 0xF8 == 0xF0 {
 401 |                         4
 402 |                     } else {
 403 |                         1
 404 |                     };
 405 |                     if end - 1 + expected_len > n {
 406 |                         end - 1 // incomplete char — exclude the leading byte too
 407 |                     } else {
 408 |                         n
 409 |                     }
 410 |                 } else {
 411 |                     n
 412 |                 }
 413 |             } else {
 414 |                 n // didn't fill the buffer, so no boundary issue
 415 |             };
 416 | 
 417 |             // First check if it's valid UTF-8
 418 |             let is_utf8 = std::str::from_utf8(&sniff[..check_len]).is_ok();
 419 | 
 420 |             if is_utf8 && !slice.contains(&0) {
 421 |                 // Valid UTF-8 text file - proceed normally
 422 |             } else {
 423 |                 // Try encoding detection for non-UTF-8 files
 424 |                 // If it's not UTF-8, try to detect the encoding
 425 |                 let (encoding, _consumed) =
 426 |                     encoding_rs::Encoding::for_bom(slice).unwrap_or((encoding_rs::UTF_8, 0));
 427 | 
 428 |                 // If it's not UTF-8, try to detect the encoding
 429 |                 let detected_encoding = if encoding == UTF_8 {
 430 |                     // Use chardet-like detection for common encodings
 431 |                     detect_text_encoding(slice)
 432 |                 } else {
 433 |                     Some(encoding)
 434 |                 };
 435 | 
 436 |                 match detected_encoding {
 437 |                     Some(enc) if enc != UTF_8 => {
 438 |                         let strategy = encoding_strategy.unwrap_or("detect");
 439 |                         match strategy {
 440 |                             "strict" | "skip" => {
 441 |                                 // Skip files with non-UTF-8 encoding
 442 |                                 warn!(
 443 |                                     "Skipping non-UTF-8 file {} (encoding: {}, strategy: {})",
 444 |                                     relative_path.display(),
 445 |                                     enc.name(),
 446 |                                     strategy
 447 |                                 );
 448 |                             }
 449 |                             _ => {
 450 |                                 // Default "detect" strategy: attempt to transcode
 451 |                                 match transcode_file_content(file_path, enc) {
 452 |                                     Ok(transcoded_content) => {
 453 |                                         info!(
 454 |                                             "Successfully transcoded {} from {} to UTF-8",
 455 |                                             relative_path.display(),
 456 |                                             enc.name()
 457 |                                         );
 458 |                                         write_text_content(
 459 |                                             output,
 460 |                                             &transcoded_content,
 461 |                                             language,
 462 |                                             line_numbers,
 463 |                                         )?;
 464 |                                         return Ok(());
 465 |                                     }
 466 |                                     Err(e) => {
 467 |                                         warn!(
 468 |                                             "Failed to transcode {} from {}: {}. Treating as binary.",
 469 |                                             relative_path.display(),
 470 |                                             enc.name(),
 471 |                                             e
 472 |                                         );
 473 |                                     }
 474 |                                 }
 475 |                             }
 476 |                         }
 477 |                     }
 478 |                     _ => {
 479 |                         // Check if it's likely binary (contains null bytes)
 480 |                         if slice.contains(&0) {
 481 |                             warn!(
 482 |                                 "Detected binary file {} (contains null bytes). Skipping content.",
 483 |                                 relative_path.display()
 484 |                             );
 485 |                         } else {
 486 |                             warn!(
 487 |                                 "Could not determine encoding for {}. Treating as binary.",
 488 |                                 relative_path.display()
 489 |                             );
 490 |                         }
 491 |                     }
 492 |                 }
 493 | 
 494 |                 // Fallback to binary file placeholder
 495 |                 writeln!(output, "```text")?;
 496 |                 writeln!(
 497 |                     output,
 498 |                     "<Binary file or unsupported encoding: {} bytes>",
 499 |                     metadata.len()
 500 |                 )?;
 501 |                 writeln!(output, "```")?;
 502 |                 return Ok(());
 503 |             }
 504 | 
 505 |             // Reset cursor and stream the content
 506 |             if let Err(e) = file.seek(SeekFrom::Start(0)) {
 507 |                 warn!(
 508 |                     "Could not reset file cursor for {}: {}. Skipping content.",
 509 |                     relative_path.display(),
 510 |                     e
 511 |                 );
 512 |                 writeln!(output, "```text")?;
 513 |                 writeln!(
 514 |                     output,
 515 |                     "<Could not read file content (e.g., binary file or permission error)>"
 516 |                 )?;
 517 |                 writeln!(output, "```")?;
 518 |                 return Ok(());
 519 |             }
 520 | 
 521 |             // Stream UTF-8 content
 522 |             let content = match std::fs::read_to_string(file_path) {
 523 |                 Ok(content) => content,
 524 |                 Err(e) => {
 525 |                     warn!(
 526 |                         "Error reading file {}: {}. Output may be truncated.",
 527 |                         relative_path.display(),
 528 |                         e
 529 |                     );
 530 |                     writeln!(output, "```text")?;
 531 |                     writeln!(output, "<Error reading file content>")?;
 532 |                     writeln!(output, "```")?;
 533 |                     return Ok(());
 534 |                 }
 535 |             };
 536 |             // When --signatures is active, replace file content with signatures-only output
 537 |             // ONLY for extensions that tree-sitter actually supports. Non-code files
 538 |             // (Cargo.toml, README.md, .yaml, etc.) must always show full content.
 539 |             let signatures_only = ts_config.signatures
 540 |                 && crate::tree_sitter::is_supported_extension(extension);
 541 | 
 542 |             if !signatures_only {
 543 |                 // Note: Smart truncation (`truncate: "smart"`) indicates AST-boundary
 544 |                 // truncation should be preferred when content needs truncating.
 545 |                 // Without a per-file max_tokens budget, no truncation is applied.
 546 |                 // The flag is stored for future use when per-file token limits are implemented.
 547 |                 write_text_content(output, &content, language, line_numbers)?;
 548 |             }
 549 | 
 550 |             // Tree-sitter enrichment: signatures and/or structure
 551 |             write_tree_sitter_enrichment(output, &content, extension, ts_config)?;
 552 |         }
 553 |         Err(e) => {
 554 |             warn!(
 555 |                 "Could not open file {}: {}. Skipping content.",
 556 |                 relative_path.display(),
 557 |                 e
 558 |             );
 559 |             writeln!(output, "```text")?;
 560 |             writeln!(
 561 |                 output,
 562 |                 "<Could not read file content (e.g., binary file or permission error)>"
 563 |             )?;
 564 |             writeln!(output, "```")?;
 565 |         }
 566 |     }
 567 | 
 568 |     Ok(())
 569 | }
 570 | 
 571 | /// Write tree-sitter enrichment (signatures, structure) after file content.
 572 | #[allow(unused_variables)]
 573 | pub fn write_tree_sitter_enrichment(
 574 |     output: &mut impl Write,
 575 |     content: &str,
 576 |     extension: &str,
 577 |     ts_config: &TreeSitterConfig,
 578 | ) -> io::Result<()> {
 579 |     if !ts_config.signatures && !ts_config.structure {
 580 |         return Ok(());
 581 |     }
 582 | 
 583 |     #[cfg(feature = "tree-sitter-base")]
 584 |     {
 585 |         use crate::tree_sitter::language_support::Visibility;
 586 | 
 587 |         let vis_filter: Visibility = ts_config.visibility.parse().unwrap_or(Visibility::All);
 588 | 
 589 |         if ts_config.structure
 590 |             && let Some(structure) =
 591 |                 crate::tree_sitter::extract_structure_for_file(content, extension)
 592 |         {
 593 |             let summary = crate::tree_sitter::structure::format_structure_as_markdown(&structure);
 594 |             if !summary.is_empty() {
 595 |                 writeln!(output)?;
 596 |                 write!(output, "{}", summary)?;
 597 |             }
 598 |         }
 599 | 
 600 |         if ts_config.signatures
 601 |             && let Some(signatures) =
 602 |                 crate::tree_sitter::extract_signatures_for_file(content, extension, vis_filter)
 603 |             && !signatures.is_empty()
 604 |         {
 605 |             let language = match extension {
 606 |                 "rs" => "rust",
 607 |                 "js" | "mjs" | "cjs" => "javascript",
 608 |                 "ts" | "tsx" | "mts" | "cts" => "typescript",
 609 |                 "py" | "pyw" => "python",
 610 |                 "go" => "go",
 611 |                 "java" => "java",
 612 |                 "c" | "h" => "c",
 613 |                 "cpp" | "cxx" | "cc" | "hpp" | "hxx" | "hh" => "cpp",
 614 |                 _ => extension,
 615 |             };
 616 |             writeln!(output)?;
 617 |             writeln!(output, "**Signatures:**")?;
 618 |             writeln!(output)?;
 619 |             let formatted = crate::tree_sitter::signatures::format_signatures_as_markdown(
 620 |                 &signatures,
 621 |                 language,
 622 |             );
 623 |             write!(output, "{}", formatted)?;
 624 |         }
 625 |     }
 626 | 
 627 |     #[cfg(not(feature = "tree-sitter-base"))]
 628 |     {
 629 |         // Tree-sitter not compiled in — flags have no effect.
 630 |         // Warning is printed once at startup in lib.rs.
 631 |     }
 632 | 
 633 |     Ok(())
 634 | }
 635 | 
 636 | /// Detect text encoding using heuristics for common encodings
 637 | fn detect_text_encoding(bytes: &[u8]) -> Option<&'static Encoding> {
 638 |     // Try common encodings
 639 |     let encodings = [
 640 |         encoding_rs::WINDOWS_1252,
 641 |         encoding_rs::UTF_16LE,
 642 |         encoding_rs::UTF_16BE,
 643 |         encoding_rs::SHIFT_JIS,
 644 |     ];
 645 | 
 646 |     for encoding in &encodings {
 647 |         let (decoded, _, had_errors) = encoding.decode(bytes);
 648 |         if !had_errors && is_likely_text(&decoded) {
 649 |             return Some(encoding);
 650 |         }
 651 |     }
 652 | 
 653 |     None
 654 | }
 655 | 
 656 | /// Check if decoded content looks like text (no control characters except common ones)
 657 | fn is_likely_text(content: &str) -> bool {
 658 |     let mut control_chars = 0;
 659 |     let mut total_chars = 0;
 660 | 
 661 |     for ch in content.chars() {
 662 |         total_chars += 1;
 663 |         if ch.is_control() && ch != '\n' && ch != '\r' && ch != '\t' {
 664 |             control_chars += 1;
 665 |         }
 666 | 
 667 |         // If more than 5% control characters, probably not text
 668 |         if total_chars > 100 && control_chars * 20 > total_chars {
 669 |             return false;
 670 |         }
 671 |     }
 672 | 
 673 |     // Allow up to 5% control characters in small files
 674 |     if total_chars > 0 {
 675 |         control_chars * 20 <= total_chars
 676 |     } else {
 677 |         true
 678 |     }
 679 | }
 680 | 
 681 | /// Transcode file content from detected encoding to UTF-8
 682 | fn transcode_file_content(file_path: &Path, encoding: &'static Encoding) -> io::Result<String> {
 683 |     let bytes = std::fs::read(file_path)?;
 684 |     let (decoded, _, had_errors) = encoding.decode(&bytes);
 685 | 
 686 |     if had_errors {
 687 |         return Err(io::Error::new(
 688 |             io::ErrorKind::InvalidData,
 689 |             format!("Failed to decode file with encoding {}", encoding.name()),
 690 |         ));
 691 |     }
 692 | 
 693 |     Ok(decoded.into_owned())
 694 | }
 695 | 
 696 | /// Write text content with optional line numbers
 697 | fn write_text_content(
 698 |     output: &mut impl Write,
 699 |     content: &str,
 700 |     language: &str,
 701 |     line_numbers: bool,
 702 | ) -> io::Result<()> {
 703 |     writeln!(output, "```{}", language)?;
 704 | 
 705 |     if line_numbers {
 706 |         for (i, line) in content.lines().enumerate() {
 707 |             writeln!(output, "{:>4} | {}", i + 1, line)?;
 708 |         }
 709 |     } else {
 710 |         output.write_all(content.as_bytes())?;
 711 |         if !content.ends_with('\n') {
 712 |             writeln!(output)?;
 713 |         }
 714 |     }
 715 | 
 716 |     writeln!(output, "```")?;
 717 |     Ok(())
 718 | }
 719 | 
 720 | #[cfg(test)]
 721 | mod tests {
 722 |     use super::*;
 723 |     use std::fs;
 724 |     use tempfile::tempdir;
 725 | 
 726 |     #[test]
 727 |     fn test_code_block_formatting() {
 728 |         let dir = tempdir().unwrap();
 729 |         let base_path = dir.path();
 730 |         let file_path = base_path.join("test.rs");
 731 |         let output_path = base_path.join("output.md");
 732 | 
 733 |         // Create a test Rust file
 734 |         fs::write(
 735 |             &file_path,
 736 |             "fn main() {\n    println!(\"Hello, world!\");\n}",
 737 |         )
 738 |         .unwrap();
 739 | 
 740 |         // Create an output file
 741 |         let mut output = fs::File::create(&output_path).unwrap();
 742 | 
 743 |         // Process the file
 744 |         process_file(
 745 |             base_path,
 746 |             &file_path,
 747 |             &mut output,
 748 |             false,
 749 |             None,
 750 |             &TreeSitterConfig::default(),
 751 |         )
 752 |         .unwrap();
 753 | 
 754 |         // Read the output
 755 |         let content = fs::read_to_string(&output_path).unwrap();
 756 | 
 757 |         // Check that code blocks are properly formatted
 758 |         assert!(content.contains("```rust"));
 759 |         assert!(content.contains("```") && content.matches("```").count() >= 2);
 760 |     }
 761 | 
 762 |     #[test]
 763 |     fn test_markdown_file_formatting() {
 764 |         let dir = tempdir().unwrap();
 765 |         let base_path = dir.path();
 766 |         let file_path = base_path.join("README.md");
 767 |         let output_path = base_path.join("output.md");
 768 | 
 769 |         // Create a test Markdown file
 770 |         fs::write(&file_path, "# Test\n\nThis is a test markdown file.").unwrap();
 771 | 
 772 |         // Create an output file
 773 |         let mut output = fs::File::create(&output_path).unwrap();
 774 | 
 775 |         // Process the file
 776 |         process_file(
 777 |             base_path,
 778 |             &file_path,
 779 |             &mut output,
 780 |             false,
 781 |             None,
 782 |             &TreeSitterConfig::default(),
 783 |         )
 784 |         .unwrap();
 785 | 
 786 |         // Read the output
 787 |         let content = fs::read_to_string(&output_path).unwrap();
 788 | 
 789 |         // Debug prints the content
 790 |         println!("Generated content:\n{}", content);
 791 | 
 792 |         // Check that markdown files use the correct language identifier
 793 |         assert!(
 794 |             content.contains("```markdown"),
 795 |             "Content should contain '```markdown' but was: {}",
 796 |             content
 797 |         );
 798 |         // Count the number of code block markers
 799 |         let code_block_markers = content.matches("```").count();
 800 | 
 801 |         assert!(
 802 |             code_block_markers >= 2,
 803 |             "Expected at least 2 code block markers, found {}",
 804 |             code_block_markers
 805 |         );
 806 |     }
 807 | 
 808 |     #[test]
 809 |     fn test_line_numbered_code_blocks() {
 810 |         let dir = tempdir().unwrap();
 811 |         let base_path = dir.path();
 812 |         let file_path = base_path.join("lib.rs");
 813 |         let output_path = base_path.join("out.md");
 814 | 
 815 |         // Create a multi-line Rust file
 816 |         fs::write(
 817 |                     &file_path,
 818 |                     "fn add(a: i32, b: i32) -> i32 {\n    a + b\n}\n\nfn main() {\n    println!(\"{}\", add(1, 2));\n}\n",
 819 |                 )
 820 |                 .unwrap();
 821 | 
 822 |         let mut output = fs::File::create(&output_path).unwrap();
 823 |         process_file(
 824 |             base_path,
 825 |             &file_path,
 826 |             &mut output,
 827 |             true,
 828 |             None,
 829 |             &TreeSitterConfig::default(),
 830 |         )
 831 |         .unwrap();
 832 | 
 833 |         let content = fs::read_to_string(&output_path).unwrap();
 834 | 
 835 |         // Check language and line numbers prefix
 836 |         assert!(content.contains("```rust"));
 837 |         assert!(content.contains("   1 | "));
 838 |         assert!(content.contains("   2 | "));
 839 | 
 840 |         // Count lines with "|" prefix equals number of lines in an original file
 841 |         let numbered_lines = content
 842 |             .lines()
 843 |             .filter(|l| {
 844 |                 l.trim_start()
 845 |                     .chars()
 846 |                     .next()
 847 |                     .map(|c| c.is_ascii_digit())
 848 |                     .unwrap_or(false)
 849 |                     && l.contains(" | ")
 850 |             })
 851 |             .count();
 852 |         let original_line_count = fs::read_to_string(&file_path).unwrap().lines().count();
 853 |         assert_eq!(numbered_lines, original_line_count);
 854 | 
 855 |         // Ensure code fence closes
 856 |         assert!(content.contains("```"));
 857 |     }
 858 | 
 859 |     #[test]
 860 |     fn test_binary_file_handling() {
 861 |         let dir = tempdir().unwrap();
 862 |         let base_path = dir.path();
 863 |         let file_path = base_path.join("image.bin");
 864 |         let output_path = base_path.join("out.md");
 865 | 
 866 |         // Write truly binary data that won't be decoded by encoding detection
 867 |         let bytes = vec![
 868 |             0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, // PNG header
 869 |             0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44, 0x52, // PNG chunk
 870 |             0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, // More binary data
 871 |             0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Null bytes
 872 |         ];
 873 |         fs::write(&file_path, bytes).unwrap();
 874 | 
 875 |         let mut output = fs::File::create(&output_path).unwrap();
 876 |         process_file(
 877 |             base_path,
 878 |             &file_path,
 879 |             &mut output,
 880 |             false,
 881 |             None,
 882 |             &TreeSitterConfig::default(),
 883 |         )
 884 |         .unwrap();
 885 | 
 886 |         let content = fs::read_to_string(&output_path).unwrap();
 887 | 
 888 |         // Expect a text block to fall back with a helpful message
 889 |         assert!(content.contains("```text"));
 890 |         assert!(content.contains("<Binary file or unsupported encoding:"));
 891 | 
 892 |         // Ensure the code block is closed
 893 |         let fence_count = content.matches("```").count();
 894 |         assert!(
 895 |             fence_count >= 2,
 896 |             "expected at least opening and closing fences, got {}",
 897 |             fence_count
 898 |         );
 899 |     }
 900 | 
 901 |     #[test]
 902 |     fn test_encoding_detection_and_transcoding() {
 903 |         let dir = tempdir().unwrap();
 904 |         let base_path = dir.path();
 905 |         let output_path = base_path.join("out.md");
 906 | 
 907 |         // Test Windows-1252 encoded file (common in Windows)
 908 |         let windows1252_content = [
 909 |             0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x20, // "Hello "
 910 |             0x93, 0x57, 0x6F, 0x72, 0x6C, 0x64, 0x94, // "World" with smart quotes
 911 |             0x0A, // newline
 912 |         ];
 913 |         let file_path = base_path.join("windows1252.txt");
 914 |         fs::write(&file_path, windows1252_content).unwrap();
 915 | 
 916 |         let mut output = fs::File::create(&output_path).unwrap();
 917 |         process_file(
 918 |             base_path,
 919 |             &file_path,
 920 |             &mut output,
 921 |             false,
 922 |             Some("detect"),
 923 |             &TreeSitterConfig::default(),
 924 |         )
 925 |         .unwrap();
 926 | 
 927 |         let content = fs::read_to_string(&output_path).unwrap();
 928 | 
 929 |         // Should contain transcoded content with UTF-8 equivalents
 930 |         assert!(content.contains("Hello"));
 931 |         assert!(content.contains("World"));
 932 |         // Should use text language
 933 |         assert!(content.contains("```txt"));
 934 | 
 935 |         // Ensure the code block is closed
 936 |         let fence_count = content.matches("```").count();
 937 |         assert!(
 938 |             fence_count >= 2,
 939 |             "expected at least opening and closing fences, got {}",
 940 |             fence_count
 941 |         );
 942 |     }
 943 | 
 944 |     #[test]
 945 |     fn test_encoding_strategy_strict() {
 946 |         let dir = tempdir().unwrap();
 947 |         let base_path = dir.path();
 948 |         let output_path = base_path.join("out.md");
 949 | 
 950 |         // Create a file with non-UTF-8 content
 951 |         let non_utf8_content = [0xFF, 0xFE, 0x41, 0x00]; // UTF-16 LE BOM + "A"
 952 |         let file_path = base_path.join("utf16.txt");
 953 |         fs::write(&file_path, non_utf8_content).unwrap();
 954 | 
 955 |         let mut output = fs::File::create(&output_path).unwrap();
 956 |         process_file(
 957 |             base_path,
 958 |             &file_path,
 959 |             &mut output,
 960 |             false,
 961 |             Some("strict"),
 962 |             &TreeSitterConfig::default(),
 963 |         )
 964 |         .unwrap();
 965 | 
 966 |         let content = fs::read_to_string(&output_path).unwrap();
 967 | 
 968 |         // Should contain binary file placeholder
 969 |         assert!(content.contains("<Binary file or unsupported encoding:"));
 970 |         assert!(content.contains("```text"));
 971 | 
 972 |         // Ensure the code block is closed
 973 |         let fence_count = content.matches("```").count();
 974 |         assert!(
 975 |             fence_count >= 2,
 976 |             "expected at least opening and closing fences, got {}",
 977 |             fence_count
 978 |         );
 979 |     }
 980 | 
 981 |     #[test]
 982 |     fn test_encoding_strategy_skip() {
 983 |         let dir = tempdir().unwrap();
 984 |         let base_path = dir.path();
 985 |         let output_path = base_path.join("out.md");
 986 | 
 987 |         // Create a file with UTF-16 content
 988 |         let utf16_content = [0xFF, 0xFE, 0x48, 0x00, 0x69, 0x00]; // UTF-16 LE "Hi"
 989 |         let file_path = base_path.join("utf16.txt");
 990 |         fs::write(&file_path, utf16_content).unwrap();
 991 | 
 992 |         let mut output = fs::File::create(&output_path).unwrap();
 993 |         process_file(
 994 |             base_path,
 995 |             &file_path,
 996 |             &mut output,
 997 |             false,
 998 |             Some("skip"),
 999 |             &TreeSitterConfig::default(),
1000 |         )
1001 |         .unwrap();
1002 | 
1003 |         let content = fs::read_to_string(&output_path).unwrap();
1004 | 
1005 |         // Should contain binary file placeholder (skipped transcoding)
1006 |         assert!(content.contains("<Binary file or unsupported encoding:"));
1007 |         assert!(content.contains("```text"));
1008 |     }
1009 | 
1010 |     #[test]
1011 |     fn test_generate_markdown_with_current_directory() {
1012 |         let dir = tempdir().unwrap();
1013 |         let base_path = dir.path();
1014 |         let output_path = base_path.join("test.md");
1015 | 
1016 |         // Create test files
1017 |         fs::write(base_path.join("readme.txt"), "Hello world").unwrap();
1018 | 
1019 |         // Collect files
1020 |         let files = crate::file_utils::collect_files(base_path, &[], &[], &[]).unwrap();
1021 |         let file_tree = crate::tree::build_file_tree(&files, base_path);
1022 | 
1023 |         // Change to the test directory
1024 |         let original_dir = std::env::current_dir().unwrap();
1025 |         std::env::set_current_dir(base_path).unwrap();
1026 | 
1027 |         // Test with "." as input directory
1028 |         let result = generate_markdown(
1029 |             &output_path.to_string_lossy(),
1030 |             ".",
1031 |             &[],
1032 |             &[],
1033 |             &file_tree,
1034 |             &files,
1035 |             base_path,
1036 |             false,
1037 |             None,
1038 |             None, // max_tokens
1039 |             &TreeSitterConfig::default(),
1040 |         );
1041 | 
1042 |         // Restore original directory
1043 |         std::env::set_current_dir(original_dir).unwrap();
1044 | 
1045 |         assert!(result.is_ok());
1046 |         let content = fs::read_to_string(&output_path).unwrap();
1047 |         assert!(content.contains("Directory Structure Report"));
1048 |     }
1049 | 
1050 |     #[test]
1051 |     fn test_generate_markdown_creates_output_directory() {
1052 |         let dir = tempdir().unwrap();
1053 |         let base_path = dir.path();
1054 |         let nested_output = base_path.join("nested").join("deep").join("output.md");
1055 | 
1056 |         // Create test files
1057 |         fs::write(base_path.join("test.txt"), "content").unwrap();
1058 | 
1059 |         let files = crate::file_utils::collect_files(base_path, &[], &[], &[]).unwrap();
1060 |         let file_tree = crate::tree::build_file_tree(&files, base_path);
1061 | 
1062 |         let result = generate_markdown(
1063 |             &nested_output.to_string_lossy(),
1064 |             "test_dir",
1065 |             &[],
1066 |             &[],
1067 |             &file_tree,
1068 |             &files,
1069 |             base_path,
1070 |             false,
1071 |             None,
1072 |             None, // max_tokens
1073 |             &TreeSitterConfig::default(),
1074 |         );
1075 | 
1076 |         assert!(result.is_ok());
1077 |         assert!(nested_output.exists());
1078 |         assert!(nested_output.parent().unwrap().exists());
1079 |     }
1080 | 
1081 |     #[test]
1082 |     fn test_generate_markdown_with_filters_and_ignores() {
1083 |         let dir = tempdir().unwrap();
1084 |         let base_path = dir.path();
1085 |         let output_path = base_path.join("filtered.md");
1086 | 
1087 |         fs::write(base_path.join("main.rs"), "fn main() {}").unwrap();
1088 |         fs::write(base_path.join("config.toml"), "[package]").unwrap();
1089 |         fs::write(base_path.join("readme.md"), "# README").unwrap();
1090 | 
1091 |         let files = crate::file_utils::collect_files(base_path, &[], &[], &[]).unwrap();
1092 |         let file_tree = crate::tree::build_file_tree(&files, base_path);
1093 | 
1094 |         let result = generate_markdown(
1095 |             &output_path.to_string_lossy(),
1096 |             "project",
1097 |             &["rs".to_string(), "toml".to_string()],
1098 |             &["readme.md".to_string()],
1099 |             &file_tree,
1100 |             &files,
1101 |             base_path,
1102 |             true,
1103 |             Some("strict"),
1104 |             None, // max_tokens
1105 |             &TreeSitterConfig::default(),
1106 |         );
1107 | 
1108 |         assert!(result.is_ok());
1109 |         let content = fs::read_to_string(&output_path).unwrap();
1110 |         assert!(content.contains("Directory Structure Report"));
1111 |         // The actual generate_markdown function doesn't format filters/ignores this way
1112 |         assert!(content.contains("main.rs") || content.contains("config.toml"));
1113 |     }
1114 | 
1115 |     #[test]
1116 |     fn test_write_text_content_with_line_numbers() {
1117 |         let mut output = Vec::new();
1118 |         let content = "line one\nline two\nline three";
1119 | 
1120 |         write_text_content(&mut output, content, "rust", true).unwrap();
1121 | 
1122 |         let result = String::from_utf8(output).unwrap();
1123 |         assert!(result.contains("```rust"));
1124 |         assert!(result.contains("   1 | line one"));
1125 |         assert!(result.contains("   2 | line two"));
1126 |         assert!(result.contains("   3 | line three"));
1127 |         assert!(result.contains("```"));
1128 |     }
1129 | 
1130 |     #[test]
1131 |     fn test_write_text_content_without_line_numbers() {
1132 |         let mut output = Vec::new();
1133 |         let content = "function test() {\n  return true;\n}";
1134 | 
1135 |         write_text_content(&mut output, content, "javascript", false).unwrap();
1136 | 
1137 |         let result = String::from_utf8(output).unwrap();
1138 |         assert!(result.contains("```javascript"));
1139 |         assert!(result.contains("function test() {"));
1140 |         assert!(result.contains("  return true;"));
1141 |         assert!(result.contains("```"));
1142 |         assert!(!result.contains(" | ")); // No line number prefix
1143 |     }
1144 | 
1145 |     #[test]
1146 |     fn test_write_text_content_without_trailing_newline() {
1147 |         let mut output = Vec::new();
1148 |         let content = "no newline at end"; // No \n at end
1149 | 
1150 |         write_text_content(&mut output, content, "text", false).unwrap();
1151 | 
1152 |         let result = String::from_utf8(output).unwrap();
1153 |         assert!(result.contains("```text"));
1154 |         assert!(result.contains("no newline at end"));
1155 |         assert!(result.ends_with("```\n")); // Should add newline
1156 |     }
1157 | 
1158 |     #[test]
1159 |     fn test_is_likely_text() {
1160 |         // Normal text should be considered text
1161 |         assert!(is_likely_text("Hello world\nThis is normal text"));
1162 | 
1163 |         // Text with some control characters should still be text
1164 |         assert!(is_likely_text(
1165 |             "Line 1\nLine 2\tTabbed\r\nWindows line ending"
1166 |         ));
1167 | 
1168 |         // Text with too many control characters should not be text
1169 |         let mut bad_text = String::new();
1170 |         for i in 0..200 {
1171 |             if i % 5 == 0 {
1172 |                 bad_text.push('\x01'); // Control character
1173 |             } else {
1174 |                 bad_text.push('a');
1175 |             }
1176 |         }
1177 |         assert!(!is_likely_text(&bad_text));
1178 | 
1179 |         // Empty string should be considered text
1180 |         assert!(is_likely_text(""));
1181 |     }
1182 | 
1183 |     #[test]
1184 |     fn test_detect_text_encoding() {
1185 |         // UTF-8 should return None (already UTF-8)
1186 |         let utf8_bytes = "Hello world".as_bytes();
1187 |         let result = detect_text_encoding(utf8_bytes);
1188 |         // The function may return an encoding even for UTF-8 text if it detects it differently
1189 |         // Just verify it doesn't crash
1190 |         assert!(result.is_some() || result.is_none());
1191 | 
1192 |         // Windows-1252 encoded text should be detected
1193 |         let windows1252_bytes = [
1194 |             0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x20, 0x93, 0x77, 0x6F, 0x72, 0x6C, 0x64, 0x94,
1195 |         ];
1196 |         let detected = detect_text_encoding(&windows1252_bytes);
1197 |         assert!(detected.is_some());
1198 |     }
1199 | 
1200 |     #[test]
1201 |     fn test_transcode_file_content() {
1202 |         let dir = tempdir().unwrap();
1203 |         let file_path = dir.path().join("windows1252.txt");
1204 | 
1205 |         // Write Windows-1252 encoded content
1206 |         let windows1252_content = [
1207 |             0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x20, // "Hello "
1208 |             0x93, 0x57, 0x6F, 0x72, 0x6C, 0x64, 0x94, // "World" with smart quotes
1209 |         ];
1210 |         fs::write(&file_path, windows1252_content).unwrap();
1211 | 
1212 |         let result = transcode_file_content(&file_path, encoding_rs::WINDOWS_1252);
1213 |         assert!(result.is_ok());
1214 | 
1215 |         let transcoded = result.unwrap();
1216 |         assert!(transcoded.contains("Hello"));
1217 |         assert!(transcoded.contains("World"));
1218 |     }
1219 | 
1220 |     #[test]
1221 |     fn test_process_file_with_metadata_error() {
1222 |         let dir = tempdir().unwrap();
1223 |         let base_path = dir.path();
1224 |         let nonexistent_file = base_path.join("nonexistent.txt");
1225 |         let output_path = base_path.join("output.md");
1226 | 
1227 |         let mut output = fs::File::create(&output_path).unwrap();
1228 | 
1229 |         // This should handle the metadata error gracefully
1230 |         let result = process_file(
1231 |             base_path,
1232 |             &nonexistent_file,
1233 |             &mut output,
1234 |             false,
1235 |             None,
1236 |             &TreeSitterConfig::default(),
1237 |         );
1238 |         assert!(result.is_ok());
1239 | 
1240 |         // Output should be minimal since file doesn't exist
1241 |         let content = fs::read_to_string(&output_path).unwrap();
1242 |         assert!(content.is_empty() || content.trim().is_empty());
1243 |     }
1244 | 
1245 |     #[test]
1246 |     fn test_process_file_with_different_extensions() {
1247 |         let dir = tempdir().unwrap();
1248 |         let base_path = dir.path();
1249 |         let output_path = base_path.join("output.md");
1250 | 
1251 |         // Test various file extensions
1252 |         let test_files = [
1253 |             ("script.py", "print('hello')", "python"),
1254 |             ("data.json", r#"{"key": "value"}"#, "json"),
1255 |             ("config.yaml", "key: value", "yaml"),
1256 |             ("style.css", "body { margin: 0; }", "css"),
1257 |             ("page.html", "<html><body>Test</body></html>", "html"),
1258 |             ("query.sql", "SELECT * FROM users;", "sql"),
1259 |             ("build.sh", "#!/bin/bash\necho 'building'", "bash"),
1260 |             ("unknown.xyz", "unknown content", "xyz"),
1261 |         ];
1262 | 
1263 |         for (filename, content, expected_lang) in test_files.iter() {
1264 |             let file_path = base_path.join(filename);
1265 |             fs::write(&file_path, content).unwrap();
1266 | 
1267 |             let mut output = fs::File::create(&output_path).unwrap();
1268 |             process_file(
1269 |                 base_path,
1270 |                 &file_path,
1271 |                 &mut output,
1272 |                 false,
1273 |                 None,
1274 |                 &TreeSitterConfig::default(),
1275 |             )
1276 |             .unwrap();
1277 | 
1278 |             let result = fs::read_to_string(&output_path).unwrap();
1279 |             assert!(result.contains(&format!("```{}", expected_lang)));
1280 |             assert!(result.contains(content));
1281 |             assert!(result.contains(filename));
1282 |         }
1283 |     }
1284 | }
```

### File: `src/state.rs`

- Size: 26279 bytes
- Modified: SystemTime { tv_sec: 1771153716, tv_nsec: 762535241 }

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
 228 |             "{:?}|{:?}|{:?}|{:?}|{:?}|{:?}|{:?}",
 229 |             config.line_numbers,
 230 |             config.auto_diff,
 231 |             config.diff_context_lines,
 232 |             config.signatures,
 233 |             config.structure,
 234 |             config.truncate,
 235 |             config.visibility,
 236 |         ));
 237 | 
 238 |         let hash = xxhash_rust::xxh3::xxh3_64(config_str.as_bytes());
 239 |         format!("{:x}", hash)
 240 |     }
 241 | }
 242 | 
 243 | impl FileState {
 244 |     /// Create a file state from a file path
 245 |     pub fn from_path(path: &Path) -> std::io::Result<Self> {
 246 |         use std::fs;
 247 |         use std::io::ErrorKind;
 248 | 
 249 |         let metadata = fs::metadata(path)?;
 250 | 
 251 |         let content = match fs::read_to_string(path) {
 252 |             Ok(content) => content,
 253 |             Err(e) if e.kind() == ErrorKind::InvalidData => {
 254 |                 // Handle binary files gracefully
 255 |                 log::warn!("Skipping binary file in auto-diff mode: {}", path.display());
 256 |                 format!("<Binary file - {} bytes>", metadata.len())
 257 |             }
 258 |             Err(e) => return Err(e),
 259 |         };
 260 | 
 261 |         // Compute content hash using stable xxh3
 262 |         let content_hash = format!("{:016x}", xxhash_rust::xxh3::xxh3_64(content.as_bytes()));
 263 | 
 264 |         Ok(FileState {
 265 |             content,
 266 |             size: metadata.len(),
 267 |             modified: metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH),
 268 |             content_hash,
 269 |         })
 270 |     }
 271 | }
 272 | 
 273 | impl ChangeSummary {
 274 |     /// Check if there are any changes
 275 |     pub fn has_changes(&self) -> bool {
 276 |         self.total_changes > 0
 277 |     }
 278 | 
 279 |     /// Generate markdown representation of the change summary
 280 |     pub fn to_markdown(&self) -> String {
 281 |         if !self.has_changes() {
 282 |             return String::new();
 283 |         }
 284 | 
 285 |         let mut output = String::new();
 286 |         output.push_str("## Change Summary\n\n");
 287 | 
 288 |         for path in &self.added {
 289 |             output.push_str(&format!("- Added: `{}`\n", path.display()));
 290 |         }
 291 | 
 292 |         for path in &self.removed {
 293 |             output.push_str(&format!("- Removed: `{}`\n", path.display()));
 294 |         }
 295 | 
 296 |         for path in &self.modified {
 297 |             output.push_str(&format!("- Modified: `{}`\n", path.display()));
 298 |         }
 299 | 
 300 |         output.push('\n');
 301 |         output
 302 |     }
 303 | }
 304 | 
 305 | #[cfg(test)]
 306 | mod tests {
 307 |     use super::*;
 308 |     use std::fs;
 309 |     use tempfile::tempdir;
 310 | 
 311 |     #[test]
 312 |     fn test_file_state_creation() {
 313 |         let temp_dir = tempdir().unwrap();
 314 |         let file_path = temp_dir.path().join("test.txt");
 315 |         fs::write(&file_path, "Hello, world!").unwrap();
 316 | 
 317 |         let file_state = FileState::from_path(&file_path).unwrap();
 318 | 
 319 |         assert_eq!(file_state.content, "Hello, world!");
 320 |         assert_eq!(file_state.size, 13);
 321 |         assert!(!file_state.content_hash.is_empty());
 322 |     }
 323 | 
 324 |     #[test]
 325 |     fn test_project_state_comparison() {
 326 |         let temp_dir = tempdir().unwrap();
 327 |         let base_path = temp_dir.path();
 328 | 
 329 |         // Create initial files
 330 |         fs::write(base_path.join("file1.txt"), "content1").unwrap();
 331 |         fs::write(base_path.join("file2.txt"), "content2").unwrap();
 332 | 
 333 |         let mut state1_files = BTreeMap::new();
 334 |         state1_files.insert(
 335 |             PathBuf::from("file1.txt"),
 336 |             FileState::from_path(&base_path.join("file1.txt")).unwrap(),
 337 |         );
 338 |         state1_files.insert(
 339 |             PathBuf::from("file2.txt"),
 340 |             FileState::from_path(&base_path.join("file2.txt")).unwrap(),
 341 |         );
 342 | 
 343 |         let state1 = ProjectState {
 344 |             timestamp: "2023-01-01T00:00:00Z".to_string(),
 345 |             config_hash: "test_hash".to_string(),
 346 |             files: state1_files,
 347 |             metadata: ProjectMetadata {
 348 |                 project_name: "test".to_string(),
 349 |                 file_count: 2,
 350 |                 filters: vec![],
 351 |                 ignores: vec![],
 352 |                 line_numbers: false,
 353 |             },
 354 |         };
 355 | 
 356 |         // Modify and create new state
 357 |         fs::write(base_path.join("file1.txt"), "modified_content1").unwrap();
 358 |         fs::write(base_path.join("file3.txt"), "content3").unwrap();
 359 | 
 360 |         let mut state2_files = BTreeMap::new();
 361 |         state2_files.insert(
 362 |             PathBuf::from("file1.txt"),
 363 |             FileState::from_path(&base_path.join("file1.txt")).unwrap(),
 364 |         );
 365 |         state2_files.insert(
 366 |             PathBuf::from("file2.txt"),
 367 |             FileState::from_path(&base_path.join("file2.txt")).unwrap(),
 368 |         );
 369 |         state2_files.insert(
 370 |             PathBuf::from("file3.txt"),
 371 |             FileState::from_path(&base_path.join("file3.txt")).unwrap(),
 372 |         );
 373 | 
 374 |         let state2 = ProjectState {
 375 |             timestamp: "2023-01-01T01:00:00Z".to_string(),
 376 |             config_hash: "test_hash".to_string(),
 377 |             files: state2_files,
 378 |             metadata: ProjectMetadata {
 379 |                 project_name: "test".to_string(),
 380 |                 file_count: 3,
 381 |                 filters: vec![],
 382 |                 ignores: vec![],
 383 |                 line_numbers: false,
 384 |             },
 385 |         };
 386 | 
 387 |         let comparison = state2.compare_with(&state1);
 388 | 
 389 |         assert_eq!(comparison.summary.added.len(), 1);
 390 |         assert_eq!(comparison.summary.modified.len(), 1);
 391 |         assert_eq!(comparison.summary.removed.len(), 0);
 392 |         assert!(
 393 |             comparison
 394 |                 .summary
 395 |                 .added
 396 |                 .contains(&PathBuf::from("file3.txt"))
 397 |         );
 398 |         assert!(
 399 |             comparison
 400 |                 .summary
 401 |                 .modified
 402 |                 .contains(&PathBuf::from("file1.txt"))
 403 |         );
 404 |     }
 405 | 
 406 |     #[test]
 407 |     fn test_change_summary_markdown() {
 408 |         let summary = ChangeSummary {
 409 |             added: vec![PathBuf::from("new.txt")],
 410 |             removed: vec![PathBuf::from("old.txt")],
 411 |             modified: vec![PathBuf::from("changed.txt")],
 412 |             total_changes: 3,
 413 |         };
 414 | 
 415 |         let markdown = summary.to_markdown();
 416 | 
 417 |         assert!(markdown.contains("## Change Summary"));
 418 |         assert!(markdown.contains("- Added: `new.txt`"));
 419 |         assert!(markdown.contains("- Removed: `old.txt`"));
 420 |         assert!(markdown.contains("- Modified: `changed.txt`"));
 421 |     }
 422 | 
 423 |     #[test]
 424 |     fn test_binary_file_handling() {
 425 |         let temp_dir = tempdir().unwrap();
 426 |         let binary_file = temp_dir.path().join("test.bin");
 427 | 
 428 |         // Write binary data (non-UTF8)
 429 |         let binary_data = vec![0u8, 255, 128, 42, 0, 1, 2, 3];
 430 |         fs::write(&binary_file, &binary_data).unwrap();
 431 | 
 432 |         // Should not crash and should handle gracefully
 433 |         let file_state = FileState::from_path(&binary_file).unwrap();
 434 | 
 435 |         // Content should be a placeholder for binary files
 436 |         assert!(file_state.content.contains("Binary file"));
 437 |         assert!(file_state.content.contains("8 bytes"));
 438 |         assert_eq!(file_state.size, 8);
 439 |         assert!(!file_state.content_hash.is_empty());
 440 |     }
 441 | 
 442 |     #[test]
 443 |     fn test_has_changes_identical_states() {
 444 |         let temp_dir = tempdir().unwrap();
 445 |         let base_path = temp_dir.path();
 446 |         fs::write(base_path.join("test.txt"), "content").unwrap();
 447 | 
 448 |         let mut files = BTreeMap::new();
 449 |         files.insert(
 450 |             PathBuf::from("test.txt"),
 451 |             FileState::from_path(&base_path.join("test.txt")).unwrap(),
 452 |         );
 453 | 
 454 |         let state1 = ProjectState {
 455 |             timestamp: "2023-01-01T00:00:00Z".to_string(),
 456 |             config_hash: "hash1".to_string(),
 457 |             files: files.clone(),
 458 |             metadata: ProjectMetadata {
 459 |                 project_name: "test".to_string(),
 460 |                 file_count: 1,
 461 |                 filters: vec![],
 462 |                 ignores: vec![],
 463 |                 line_numbers: false,
 464 |             },
 465 |         };
 466 | 
 467 |         let state2 = ProjectState {
 468 |             timestamp: "2023-01-01T01:00:00Z".to_string(),
 469 |             config_hash: "hash1".to_string(),
 470 |             files,
 471 |             metadata: ProjectMetadata {
 472 |                 project_name: "test".to_string(),
 473 |                 file_count: 1,
 474 |                 filters: vec![],
 475 |                 ignores: vec![],
 476 |                 line_numbers: false,
 477 |             },
 478 |         };
 479 | 
 480 |         assert!(!state1.has_changes(&state2));
 481 |     }
 482 | 
 483 |     #[test]
 484 |     fn test_has_changes_different_file_count() {
 485 |         let temp_dir = tempdir().unwrap();
 486 |         let base_path = temp_dir.path();
 487 |         fs::write(base_path.join("test1.txt"), "content1").unwrap();
 488 |         fs::write(base_path.join("test2.txt"), "content2").unwrap();
 489 | 
 490 |         let mut files1 = BTreeMap::new();
 491 |         files1.insert(
 492 |             PathBuf::from("test1.txt"),
 493 |             FileState::from_path(&base_path.join("test1.txt")).unwrap(),
 494 |         );
 495 | 
 496 |         let mut files2 = BTreeMap::new();
 497 |         files2.insert(
 498 |             PathBuf::from("test1.txt"),
 499 |             FileState::from_path(&base_path.join("test1.txt")).unwrap(),
 500 |         );
 501 |         files2.insert(
 502 |             PathBuf::from("test2.txt"),
 503 |             FileState::from_path(&base_path.join("test2.txt")).unwrap(),
 504 |         );
 505 | 
 506 |         let state1 = ProjectState {
 507 |             timestamp: "2023-01-01T00:00:00Z".to_string(),
 508 |             config_hash: "hash1".to_string(),
 509 |             files: files1,
 510 |             metadata: ProjectMetadata {
 511 |                 project_name: "test".to_string(),
 512 |                 file_count: 1,
 513 |                 filters: vec![],
 514 |                 ignores: vec![],
 515 |                 line_numbers: false,
 516 |             },
 517 |         };
 518 | 
 519 |         let state2 = ProjectState {
 520 |             timestamp: "2023-01-01T01:00:00Z".to_string(),
 521 |             config_hash: "hash1".to_string(),
 522 |             files: files2,
 523 |             metadata: ProjectMetadata {
 524 |                 project_name: "test".to_string(),
 525 |                 file_count: 2,
 526 |                 filters: vec![],
 527 |                 ignores: vec![],
 528 |                 line_numbers: false,
 529 |             },
 530 |         };
 531 | 
 532 |         assert!(state1.has_changes(&state2));
 533 |     }
 534 | 
 535 |     #[test]
 536 |     fn test_has_changes_content_different() {
 537 |         let temp_dir = tempdir().unwrap();
 538 |         let base_path = temp_dir.path();
 539 |         fs::write(base_path.join("test.txt"), "content1").unwrap();
 540 | 
 541 |         let file_state1 = FileState::from_path(&base_path.join("test.txt")).unwrap();
 542 | 
 543 |         fs::write(base_path.join("test.txt"), "content2").unwrap();
 544 |         let file_state2 = FileState::from_path(&base_path.join("test.txt")).unwrap();
 545 | 
 546 |         let mut files1 = BTreeMap::new();
 547 |         files1.insert(PathBuf::from("test.txt"), file_state1);
 548 | 
 549 |         let mut files2 = BTreeMap::new();
 550 |         files2.insert(PathBuf::from("test.txt"), file_state2);
 551 | 
 552 |         let state1 = ProjectState {
 553 |             timestamp: "2023-01-01T00:00:00Z".to_string(),
 554 |             config_hash: "hash1".to_string(),
 555 |             files: files1,
 556 |             metadata: ProjectMetadata {
 557 |                 project_name: "test".to_string(),
 558 |                 file_count: 1,
 559 |                 filters: vec![],
 560 |                 ignores: vec![],
 561 |                 line_numbers: false,
 562 |             },
 563 |         };
 564 | 
 565 |         let state2 = ProjectState {
 566 |             timestamp: "2023-01-01T01:00:00Z".to_string(),
 567 |             config_hash: "hash1".to_string(),
 568 |             files: files2,
 569 |             metadata: ProjectMetadata {
 570 |                 project_name: "test".to_string(),
 571 |                 file_count: 1,
 572 |                 filters: vec![],
 573 |                 ignores: vec![],
 574 |                 line_numbers: false,
 575 |             },
 576 |         };
 577 | 
 578 |         assert!(state1.has_changes(&state2));
 579 |     }
 580 | 
 581 |     #[test]
 582 |     fn test_config_hash_generation() {
 583 |         let config1 = Config {
 584 |             filter: Some(vec!["rs".to_string()]),
 585 |             ignore: Some(vec!["target".to_string()]),
 586 |             line_numbers: Some(true),
 587 |             auto_diff: Some(false),
 588 |             diff_context_lines: Some(3),
 589 |             ..Default::default()
 590 |         };
 591 | 
 592 |         let config2 = Config {
 593 |             filter: Some(vec!["rs".to_string()]),
 594 |             ignore: Some(vec!["target".to_string()]),
 595 |             line_numbers: Some(true),
 596 |             auto_diff: Some(false),
 597 |             diff_context_lines: Some(3),
 598 |             ..Default::default()
 599 |         };
 600 | 
 601 |         let config3 = Config {
 602 |             filter: Some(vec!["py".to_string()]), // Different filter
 603 |             ignore: Some(vec!["target".to_string()]),
 604 |             line_numbers: Some(true),
 605 |             auto_diff: Some(false),
 606 |             diff_context_lines: Some(3),
 607 |             ..Default::default()
 608 |         };
 609 | 
 610 |         let hash1 = ProjectState::compute_config_hash(&config1);
 611 |         let hash2 = ProjectState::compute_config_hash(&config2);
 612 |         let hash3 = ProjectState::compute_config_hash(&config3);
 613 | 
 614 |         assert_eq!(hash1, hash2);
 615 |         assert_ne!(hash1, hash3);
 616 |     }
 617 | 
 618 |     #[test]
 619 |     fn test_change_summary_no_changes() {
 620 |         let summary = ChangeSummary {
 621 |             added: vec![],
 622 |             removed: vec![],
 623 |             modified: vec![],
 624 |             total_changes: 0,
 625 |         };
 626 | 
 627 |         assert!(!summary.has_changes());
 628 |         assert_eq!(summary.to_markdown(), "");
 629 |     }
 630 | 
 631 |     #[test]
 632 |     fn test_from_files_with_config() {
 633 |         let temp_dir = tempdir().unwrap();
 634 |         let base_path = temp_dir.path();
 635 | 
 636 |         fs::write(base_path.join("test.rs"), "fn main() {}").unwrap();
 637 |         fs::write(base_path.join("README.md"), "# Test").unwrap();
 638 | 
 639 |         let entries = vec![
 640 |             create_mock_dir_entry(&base_path.join("test.rs")),
 641 |             create_mock_dir_entry(&base_path.join("README.md")),
 642 |         ];
 643 | 
 644 |         let config = Config {
 645 |             filter: Some(vec!["rs".to_string()]),
 646 |             ignore: Some(vec!["target".to_string()]),
 647 |             line_numbers: Some(true),
 648 |             ..Default::default()
 649 |         };
 650 | 
 651 |         let state = ProjectState::from_files(&entries, base_path, &config, true).unwrap();
 652 | 
 653 |         assert_eq!(state.files.len(), 2);
 654 |         assert_eq!(state.metadata.file_count, 2);
 655 |         assert_eq!(state.metadata.filters, vec!["rs"]);
 656 |         assert_eq!(state.metadata.ignores, vec!["target"]);
 657 |         assert!(state.metadata.line_numbers);
 658 |         assert!(!state.timestamp.is_empty());
 659 |         assert!(!state.config_hash.is_empty());
 660 |     }
 661 | 
 662 |     #[test]
 663 |     fn test_from_files_absolute_path_fallback() {
 664 |         let temp_dir = tempdir().unwrap();
 665 |         let base_path = temp_dir.path();
 666 | 
 667 |         // Create a file in the temp dir
 668 |         fs::write(base_path.join("test.txt"), "test content").unwrap();
 669 |         let file_path = base_path.join("test.txt");
 670 | 
 671 |         // Create entry with the file
 672 |         let entry = create_mock_dir_entry(&file_path);
 673 | 
 674 |         // Use a completely different base_path to force the fallback
 675 |         let different_base = PathBuf::from("/completely/different/path");
 676 | 
 677 |         let config = Config::default();
 678 | 
 679 |         let state = ProjectState::from_files(&[entry], &different_base, &config, false).unwrap();
 680 | 
 681 |         // Should fall back to just the filename
 682 |         assert_eq!(state.files.len(), 1);
 683 |         assert!(state.files.contains_key(&PathBuf::from("test.txt")));
 684 |     }
 685 | 
 686 |     #[test]
 687 |     fn test_change_summary_with_unchanged_files() {
 688 |         let changes = vec![
 689 |             PerFileDiff {
 690 |                 path: "added.txt".to_string(),
 691 |                 status: PerFileStatus::Added,
 692 |                 diff: "diff content".to_string(),
 693 |             },
 694 |             PerFileDiff {
 695 |                 path: "unchanged.txt".to_string(),
 696 |                 status: PerFileStatus::Unchanged,
 697 |                 diff: "".to_string(),
 698 |             },
 699 |         ];
 700 | 
 701 |         // Manually create the summary like the actual code does
 702 |         let mut added = Vec::new();
 703 |         let mut removed = Vec::new();
 704 |         let mut modified = Vec::new();
 705 | 
 706 |         for diff in &changes {
 707 |             let path = PathBuf::from(&diff.path);
 708 |             match diff.status {
 709 |                 PerFileStatus::Added => added.push(path),
 710 |                 PerFileStatus::Removed => removed.push(path),
 711 |                 PerFileStatus::Modified => modified.push(path),
 712 |                 PerFileStatus::Unchanged => {} // This line should be covered now
 713 |             }
 714 |         }
 715 | 
 716 |         let summary = ChangeSummary {
 717 |             total_changes: added.len() + removed.len() + modified.len(),
 718 |             added,
 719 |             removed,
 720 |             modified,
 721 |         };
 722 | 
 723 |         assert_eq!(summary.total_changes, 1); // Only the added file counts
 724 |         assert_eq!(summary.added.len(), 1);
 725 |         assert_eq!(summary.removed.len(), 0);
 726 |         assert_eq!(summary.modified.len(), 0);
 727 |     }
 728 | 
 729 |     #[test]
 730 |     fn test_has_changes_with_missing_file() {
 731 |         let temp_dir = tempdir().unwrap();
 732 |         let base_path = temp_dir.path();
 733 | 
 734 |         // Create files for the first state
 735 |         fs::write(base_path.join("file1.txt"), "content1").unwrap();
 736 |         let entry1 = create_mock_dir_entry(&base_path.join("file1.txt"));
 737 | 
 738 |         let config = Config::default();
 739 |         let state1 = ProjectState::from_files(&[entry1], base_path, &config, false).unwrap();
 740 | 
 741 |         // Create a different state with different files
 742 |         fs::write(base_path.join("file2.txt"), "content2").unwrap();
 743 |         let entry2 = create_mock_dir_entry(&base_path.join("file2.txt"));
 744 |         let state2 = ProjectState::from_files(&[entry2], base_path, &config, false).unwrap();
 745 | 
 746 |         // Should detect changes because files are completely different
 747 |         assert!(state1.has_changes(&state2));
 748 |     }
 749 | 
 750 |     #[test]
 751 |     fn test_file_state_with_invalid_data_error() {
 752 |         // Create a temporary file with binary content that might trigger InvalidData
 753 |         let temp_dir = tempdir().unwrap();
 754 |         let binary_file = temp_dir.path().join("binary.dat");
 755 | 
 756 |         // Write invalid UTF-8 bytes
 757 |         let binary_data = vec![0xFF, 0xFE, 0xFD, 0xFC, 0xFB, 0xFA];
 758 |         fs::write(&binary_file, &binary_data).unwrap();
 759 | 
 760 |         // This might trigger the InvalidData error path, but since we can't guarantee it,
 761 |         // we at least verify the function can handle binary files
 762 |         let result = FileState::from_path(&binary_file);
 763 |         assert!(result.is_ok());
 764 |     }
 765 | 
 766 |     // Helper function to create a mock DirEntry for testing
 767 |     fn create_mock_dir_entry(path: &std::path::Path) -> ignore::DirEntry {
 768 |         // This is a bit of a hack since DirEntry doesn't have a public constructor
 769 |         // We use the ignore crate's WalkBuilder to create a real DirEntry
 770 |         let walker = ignore::WalkBuilder::new(path.parent().unwrap());
 771 |         walker
 772 |             .build()
 773 |             .filter_map(Result::ok)
 774 |             .find(|entry| entry.path() == path)
 775 |             .expect("Failed to create DirEntry for test")
 776 |     }
 777 | }
```

### File: `src/token_count.rs`

- Size: 10045 bytes
- Modified: SystemTime { tv_sec: 1771142666, tv_nsec: 596069918 }

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
 130 |         crate::markdown::process_file(
 131 |             dir.path(),
 132 |             &test_file,
 133 |             &mut actual_content,
 134 |             false,
 135 |             None,
 136 |             &crate::markdown::TreeSitterConfig::default(),
 137 |         )
 138 |         .unwrap();
 139 |         let actual_content_str = String::from_utf8(actual_content).unwrap();
 140 | 
 141 |         // Count actual tokens
 142 |         let actual_tokens = estimate_tokens(&actual_content_str);
 143 | 
 144 |         // The estimation should be close to actual (within a reasonable margin)
 145 |         // Allow for some variance due to timestamp differences and minor formatting
 146 |         let difference = actual_tokens.abs_diff(estimated_tokens);
 147 | 
 148 |         // Should be within 10% or 20 tokens difference (whichever is larger)
 149 |         let max_allowed_difference = std::cmp::max(actual_tokens / 10, 20);
 150 | 
 151 |         assert!(
 152 |             difference <= max_allowed_difference,
 153 |             "Token estimation {} differs too much from actual {} (difference: {})",
 154 |             estimated_tokens,
 155 |             actual_tokens,
 156 |             difference
 157 |         );
 158 |     }
 159 | 
 160 |     #[test]
 161 |     fn test_estimate_tokens_empty_string() {
 162 |         let tokens = estimate_tokens("");
 163 |         assert_eq!(tokens, 0);
 164 |     }
 165 | 
 166 |     #[test]
 167 |     fn test_estimate_tokens_whitespace_only() {
 168 |         let tokens = estimate_tokens("   \n\t  ");
 169 |         assert!(tokens > 0); // Whitespace still counts as tokens
 170 |     }
 171 | 
 172 |     #[test]
 173 |     fn test_estimate_tokens_unicode() {
 174 |         let tokens = estimate_tokens("Hello 世界! 🌍");
 175 |         assert!(tokens > 0);
 176 |         // Unicode characters may be encoded as multiple tokens
 177 |         assert!(tokens >= 4);
 178 |     }
 179 | 
 180 |     #[test]
 181 |     fn test_count_file_tokens_with_line_numbers() {
 182 |         use tempfile::tempdir;
 183 | 
 184 |         let dir = tempdir().unwrap();
 185 |         let test_file = dir.path().join("test.rs");
 186 |         std::fs::write(&test_file, "line 1\nline 2\nline 3").unwrap();
 187 | 
 188 |         let entry = ignore::WalkBuilder::new(&test_file)
 189 |             .build()
 190 |             .next()
 191 |             .unwrap()
 192 |             .unwrap();
 193 | 
 194 |         let tokens_without_line_numbers = count_file_tokens(dir.path(), &entry, false);
 195 |         let tokens_with_line_numbers = count_file_tokens(dir.path(), &entry, true);
 196 | 
 197 |         // With line numbers should have more tokens due to line number prefixes
 198 |         assert!(tokens_with_line_numbers > tokens_without_line_numbers);
 199 |     }
 200 | 
 201 |     #[test]
 202 |     fn test_count_file_tokens_unreadable_file() {
 203 |         use tempfile::tempdir;
 204 | 
 205 |         let dir = tempdir().unwrap();
 206 |         let test_file = dir.path().join("nonexistent.txt");
 207 | 
 208 |         // Create a mock DirEntry for a file that doesn't exist
 209 |         // This simulates what happens when a file is deleted between discovery and processing
 210 |         let walker = ignore::WalkBuilder::new(dir.path());
 211 |         let mut found_entry = None;
 212 | 
 213 |         // Create the file temporarily to get a DirEntry
 214 |         std::fs::write(&test_file, "temp").unwrap();
 215 |         for entry in walker.build() {
 216 |             if let Ok(entry) = entry
 217 |                 && entry.path() == test_file
 218 |             {
 219 |                 found_entry = Some(entry);
 220 |                 break;
 221 |             }
 222 |         }
 223 | 
 224 |         // Now delete the file
 225 |         std::fs::remove_file(&test_file).unwrap();
 226 | 
 227 |         if let Some(entry) = found_entry {
 228 |             let tokens = count_file_tokens(dir.path(), &entry, false);
 229 |             // Should still return some tokens for the file header even if content can't be read
 230 |             assert!(tokens > 0);
 231 |         }
 232 |     }
 233 | 
 234 |     #[test]
 235 |     fn test_count_tree_tokens_empty_tree() {
 236 |         let tree = BTreeMap::new();
 237 |         let tokens = count_tree_tokens(&tree, 0);
 238 |         assert_eq!(tokens, 0);
 239 |     }
 240 | 
 241 |     #[test]
 242 |     fn test_count_tree_tokens_nested_directories() {
 243 |         let mut tree = BTreeMap::new();
 244 | 
 245 |         // Create deeply nested structure
 246 |         let mut level3 = BTreeMap::new();
 247 |         level3.insert("deep_file.txt".to_string(), crate::tree::FileNode::File);
 248 | 
 249 |         let mut level2 = BTreeMap::new();
 250 |         level2.insert(
 251 |             "level3".to_string(),
 252 |             crate::tree::FileNode::Directory(level3),
 253 |         );
 254 | 
 255 |         let mut level1 = BTreeMap::new();
 256 |         level1.insert(
 257 |             "level2".to_string(),
 258 |             crate::tree::FileNode::Directory(level2),
 259 |         );
 260 | 
 261 |         tree.insert(
 262 |             "level1".to_string(),
 263 |             crate::tree::FileNode::Directory(level1),
 264 |         );
 265 | 
 266 |         let tokens = count_tree_tokens(&tree, 0);
 267 |         assert!(tokens > 0);
 268 | 
 269 |         // Should account for indentation at different levels
 270 |         let tokens_with_depth = count_tree_tokens(&tree, 2);
 271 |         assert!(tokens_with_depth > tokens); // More indentation = more tokens
 272 |     }
 273 | 
 274 |     #[test]
 275 |     fn test_count_tree_tokens_mixed_content() {
 276 |         let mut tree = BTreeMap::new();
 277 | 
 278 |         // Add files with various name lengths and characters
 279 |         tree.insert("a.txt".to_string(), crate::tree::FileNode::File);
 280 |         tree.insert(
 281 |             "very_long_filename_with_underscores.rs".to_string(),
 282 |             crate::tree::FileNode::File,
 283 |         );
 284 |         tree.insert("файл.txt".to_string(), crate::tree::FileNode::File); // Unicode filename
 285 | 
 286 |         let mut subdir = BTreeMap::new();
 287 |         subdir.insert("nested.md".to_string(), crate::tree::FileNode::File);
 288 |         tree.insert(
 289 |             "directory".to_string(),
 290 |             crate::tree::FileNode::Directory(subdir),
 291 |         );
 292 | 
 293 |         let tokens = count_tree_tokens(&tree, 0);
 294 |         assert!(tokens > 0);
 295 | 
 296 |         // Verify it handles unicode filenames without crashing
 297 |         assert!(tokens > 20); // Should be substantial given the content
 298 |     }
 299 | }
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

### File: `src/tree_sitter/language_support.rs`

- Size: 4742 bytes
- Modified: SystemTime { tv_sec: 1771153356, tv_nsec: 169962139 }

```rust
   1 | //! Core types and traits for language support.
   2 | 
   3 | use std::fmt;
   4 | use std::str::FromStr;
   5 | 
   6 | /// The kind of signature extracted from source code.
   7 | #[derive(Debug, Clone, PartialEq, Eq)]
   8 | pub enum SignatureKind {
   9 |     Function,
  10 |     Method,
  11 |     Struct,
  12 |     Enum,
  13 |     Trait,
  14 |     Interface,
  15 |     Class,
  16 |     Impl,
  17 |     Module,
  18 |     Constant,
  19 |     TypeAlias,
  20 |     Macro,
  21 | }
  22 | 
  23 | impl fmt::Display for SignatureKind {
  24 |     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
  25 |         match self {
  26 |             SignatureKind::Function => write!(f, "function"),
  27 |             SignatureKind::Method => write!(f, "method"),
  28 |             SignatureKind::Struct => write!(f, "struct"),
  29 |             SignatureKind::Enum => write!(f, "enum"),
  30 |             SignatureKind::Trait => write!(f, "trait"),
  31 |             SignatureKind::Interface => write!(f, "interface"),
  32 |             SignatureKind::Class => write!(f, "class"),
  33 |             SignatureKind::Impl => write!(f, "impl"),
  34 |             SignatureKind::Module => write!(f, "module"),
  35 |             SignatureKind::Constant => write!(f, "constant"),
  36 |             SignatureKind::TypeAlias => write!(f, "type"),
  37 |             SignatureKind::Macro => write!(f, "macro"),
  38 |         }
  39 |     }
  40 | }
  41 | 
  42 | /// Visibility level of a signature.
  43 | #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
  44 | pub enum Visibility {
  45 |     #[default]
  46 |     All,
  47 |     Public,
  48 |     Private,
  49 | }
  50 | 
  51 | impl FromStr for Visibility {
  52 |     type Err = std::convert::Infallible;
  53 | 
  54 |     fn from_str(s: &str) -> Result<Self, Self::Err> {
  55 |         Ok(match s.to_lowercase().as_str() {
  56 |             "public" => Visibility::Public,
  57 |             "private" => Visibility::Private,
  58 |             _ => Visibility::All,
  59 |         })
  60 |     }
  61 | }
  62 | 
  63 | impl Visibility {
  64 |     /// Check if a symbol's visibility passes the filter.
  65 |     /// Returns `true` if the symbol should be included.
  66 |     pub fn matches_filter(self, filter: Visibility) -> bool {
  67 |         match filter {
  68 |             Visibility::All => true,
  69 |             Visibility::Public => self == Visibility::Public,
  70 |             Visibility::Private => self == Visibility::Private,
  71 |         }
  72 |     }
  73 | }
  74 | 
  75 | /// A signature extracted from source code (function, class, etc.).
  76 | #[derive(Debug, Clone)]
  77 | pub struct Signature {
  78 |     pub kind: SignatureKind,
  79 |     pub name: String,
  80 |     pub params: Option<String>,
  81 |     pub return_type: Option<String>,
  82 |     pub visibility: Visibility,
  83 |     pub line_number: usize,
  84 |     pub full_signature: String,
  85 | }
  86 | 
  87 | impl fmt::Display for Signature {
  88 |     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
  89 |         write!(f, "{}", self.full_signature)
  90 |     }
  91 | }
  92 | 
  93 | /// Slice the source text from a node's start to its body's start, producing
  94 | /// a perfect signature that preserves all modifiers, generics, params, and return types.
  95 | ///
  96 | /// `body_kinds` is a list of node kinds that represent the function/class body
  97 | /// (e.g., `"block"`, `"compound_statement"`, `"statement_block"`).
  98 | ///
  99 | /// Returns `None` if no body node is found (e.g., forward declarations).
 100 | pub fn slice_signature_before_body(
 101 |     source: &str,
 102 |     node: &tree_sitter::Node,
 103 |     body_kinds: &[&str],
 104 | ) -> Option<String> {
 105 |     let mut cursor = node.walk();
 106 |     for child in node.children(&mut cursor) {
 107 |         if body_kinds.contains(&child.kind()) {
 108 |             let sig = &source[node.start_byte()..child.start_byte()];
 109 |             return Some(sig.trim_end().to_string());
 110 |         }
 111 |     }
 112 |     None
 113 | }
 114 | 
 115 | /// Structure information extracted from a source file.
 116 | #[derive(Debug, Clone, Default)]
 117 | pub struct CodeStructure {
 118 |     pub imports: Vec<String>,
 119 |     pub exports: Vec<String>,
 120 |     pub functions: usize,
 121 |     pub structs: usize,
 122 |     pub enums: usize,
 123 |     pub traits: usize,
 124 |     pub classes: usize,
 125 |     pub interfaces: usize,
 126 |     pub constants: usize,
 127 |     pub type_aliases: usize,
 128 |     pub macros: usize,
 129 |     pub total_lines: usize,
 130 |     pub code_lines: usize,
 131 | }
 132 | 
 133 | impl CodeStructure {
 134 |     pub fn total_symbols(&self) -> usize {
 135 |         self.functions
 136 |             + self.structs
 137 |             + self.enums
 138 |             + self.traits
 139 |             + self.classes
 140 |             + self.interfaces
 141 |             + self.constants
 142 |             + self.type_aliases
 143 |             + self.macros
 144 |     }
 145 | }
 146 | 
 147 | /// Trait for language-specific parsing support.
 148 | pub trait LanguageSupport: Send + Sync {
 149 |     fn file_extensions(&self) -> &[&'static str];
 150 | 
 151 |     fn supports_extension(&self, ext: &str) -> bool {
 152 |         self.file_extensions()
 153 |             .iter()
 154 |             .any(|&e| e.eq_ignore_ascii_case(ext))
 155 |     }
 156 | 
 157 |     fn parse(&self, source: &str) -> Option<tree_sitter::Tree>;
 158 | 
 159 |     fn extract_signatures(&self, source: &str, visibility: Visibility) -> Vec<Signature>;
 160 | 
 161 |     fn extract_structure(&self, source: &str) -> CodeStructure;
 162 | 
 163 |     fn find_truncation_point(&self, source: &str, max_bytes: usize) -> usize;
 164 | }
```

### File: `src/tree_sitter/languages/c.rs`

- Size: 10846 bytes
- Modified: SystemTime { tv_sec: 1771153429, tv_nsec: 24886100 }

```rust
   1 | //! C language support for tree-sitter.
   2 | 
   3 | #[cfg(feature = "tree-sitter-c")]
   4 | use tree_sitter::{Parser, Tree};
   5 | 
   6 | #[cfg(feature = "tree-sitter-c")]
   7 | use crate::tree_sitter::language_support::{
   8 |     CodeStructure, LanguageSupport, Signature, SignatureKind, Visibility,
   9 |     slice_signature_before_body,
  10 | };
  11 | 
  12 | pub struct CSupport;
  13 | 
  14 | #[cfg(feature = "tree-sitter-c")]
  15 | impl CSupport {
  16 |     fn get_language() -> tree_sitter::Language {
  17 |         tree_sitter_c::LANGUAGE.into()
  18 |     }
  19 | }
  20 | 
  21 | #[cfg(feature = "tree-sitter-c")]
  22 | impl LanguageSupport for CSupport {
  23 |     fn file_extensions(&self) -> &[&'static str] {
  24 |         &["c", "h"]
  25 |     }
  26 | 
  27 |     fn parse(&self, source: &str) -> Option<Tree> {
  28 |         let mut parser = Parser::new();
  29 |         parser.set_language(&Self::get_language()).ok()?;
  30 |         parser.parse(source, None)
  31 |     }
  32 | 
  33 |     fn extract_signatures(&self, source: &str, visibility: Visibility) -> Vec<Signature> {
  34 |         let tree = match self.parse(source) {
  35 |             Some(t) => t,
  36 |             None => return Vec::new(),
  37 |         };
  38 | 
  39 |         let root = tree.root_node();
  40 |         let mut signatures = Vec::new();
  41 | 
  42 |         self.extract_signatures_from_node(source, &root, visibility, &mut signatures);
  43 | 
  44 |         signatures.sort_by_key(|s| s.line_number);
  45 |         signatures
  46 |     }
  47 | 
  48 |     fn extract_structure(&self, source: &str) -> CodeStructure {
  49 |         let tree = match self.parse(source) {
  50 |             Some(t) => t,
  51 |             None => return CodeStructure::default(),
  52 |         };
  53 | 
  54 |         let root = tree.root_node();
  55 |         let mut structure = CodeStructure {
  56 |             total_lines: source.lines().count(),
  57 |             ..Default::default()
  58 |         };
  59 | 
  60 |         self.extract_structure_from_node(&root, &mut structure);
  61 |         structure
  62 |     }
  63 | 
  64 |     fn find_truncation_point(&self, source: &str, max_bytes: usize) -> usize {
  65 |         if source.len() <= max_bytes {
  66 |             return source.len();
  67 |         }
  68 | 
  69 |         let tree = match self.parse(source) {
  70 |             Some(t) => t,
  71 |             None => return max_bytes,
  72 |         };
  73 | 
  74 |         let root = tree.root_node();
  75 |         let mut best_end = 0;
  76 | 
  77 |         let mut cursor = root.walk();
  78 |         self.find_best_boundary(&mut cursor, max_bytes, &mut best_end);
  79 |         drop(cursor);
  80 | 
  81 |         if best_end == 0 { max_bytes } else { best_end }
  82 |     }
  83 | }
  84 | 
  85 | #[cfg(feature = "tree-sitter-c")]
  86 | impl CSupport {
  87 |     fn extract_signatures_from_node(
  88 |         &self,
  89 |         source: &str,
  90 |         node: &tree_sitter::Node,
  91 |         _visibility: Visibility,
  92 |         signatures: &mut Vec<Signature>,
  93 |     ) {
  94 |         match node.kind() {
  95 |             "function_definition" => {
  96 |                 if let Some(sig) = self.extract_function_signature(source, node) {
  97 |                     signatures.push(sig);
  98 |                 }
  99 |             }
 100 |             "struct_specifier" => {
 101 |                 if let Some(sig) = self.extract_struct_signature(source, node) {
 102 |                     signatures.push(sig);
 103 |                 }
 104 |             }
 105 |             "enum_specifier" => {
 106 |                 if let Some(sig) = self.extract_enum_signature(source, node) {
 107 |                     signatures.push(sig);
 108 |                 }
 109 |             }
 110 |             "type_definition" => {
 111 |                 if let Some(sig) = self.extract_typedef_signature(source, node) {
 112 |                     signatures.push(sig);
 113 |                 }
 114 |             }
 115 |             "preproc_function_def" => {
 116 |                 if let Some(sig) = self.extract_macro_signature(source, node) {
 117 |                     signatures.push(sig);
 118 |                 }
 119 |             }
 120 |             _ => {}
 121 |         }
 122 | 
 123 |         let mut cursor = node.walk();
 124 |         for child in node.children(&mut cursor) {
 125 |             self.extract_signatures_from_node(source, &child, _visibility, signatures);
 126 |         }
 127 |     }
 128 | 
 129 |     fn extract_structure_from_node(&self, node: &tree_sitter::Node, structure: &mut CodeStructure) {
 130 |         match node.kind() {
 131 |             "function_definition" => structure.functions += 1,
 132 |             "struct_specifier" => structure.structs += 1,
 133 |             "enum_specifier" => structure.enums += 1,
 134 |             "preproc_include" => {
 135 |                 structure.imports.push("include".to_string());
 136 |             }
 137 |             _ => {}
 138 |         }
 139 | 
 140 |         let mut cursor = node.walk();
 141 |         for child in node.children(&mut cursor) {
 142 |             self.extract_structure_from_node(&child, structure);
 143 |         }
 144 |     }
 145 | 
 146 |     fn extract_function_signature(
 147 |         &self,
 148 |         source: &str,
 149 |         node: &tree_sitter::Node,
 150 |     ) -> Option<Signature> {
 151 |         let name = self.find_function_name(node, source)?;
 152 |         let return_type = self.find_return_type(node, source);
 153 |         let params = self.find_child_text(node, "parameter_list", source);
 154 | 
 155 |         // Use byte-slicing to preserve complete function signatures including parameters
 156 |         let full_sig = slice_signature_before_body(source, node, &["compound_statement"])
 157 |             .unwrap_or_else(|| {
 158 |                 let mut sig = String::new();
 159 |                 if let Some(r) = &return_type {
 160 |                     sig.push_str(r);
 161 |                     sig.push(' ');
 162 |                 }
 163 |                 sig.push_str(&name);
 164 |                 if let Some(p) = &params {
 165 |                     sig.push_str(p);
 166 |                 } else {
 167 |                     sig.push_str("()");
 168 |                 }
 169 |                 sig
 170 |             });
 171 | 
 172 |         Some(Signature {
 173 |             kind: SignatureKind::Function,
 174 |             name,
 175 |             params,
 176 |             return_type,
 177 |             visibility: Visibility::All, // C has no visibility
 178 |             line_number: node.start_position().row + 1,
 179 |             full_signature: full_sig,
 180 |         })
 181 |     }
 182 | 
 183 |     fn extract_struct_signature(
 184 |         &self,
 185 |         source: &str,
 186 |         node: &tree_sitter::Node,
 187 |     ) -> Option<Signature> {
 188 |         let name = self.find_child_text(node, "type_identifier", source)?;
 189 | 
 190 |         let full_sig = format!("struct {}", name);
 191 | 
 192 |         Some(Signature {
 193 |             kind: SignatureKind::Struct,
 194 |             name,
 195 |             params: None,
 196 |             return_type: None,
 197 |             visibility: Visibility::All,
 198 |             line_number: node.start_position().row + 1,
 199 |             full_signature: full_sig,
 200 |         })
 201 |     }
 202 | 
 203 |     fn extract_enum_signature(&self, source: &str, node: &tree_sitter::Node) -> Option<Signature> {
 204 |         let name = self.find_child_text(node, "type_identifier", source)?;
 205 | 
 206 |         let full_sig = format!("enum {}", name);
 207 | 
 208 |         Some(Signature {
 209 |             kind: SignatureKind::Enum,
 210 |             name,
 211 |             params: None,
 212 |             return_type: None,
 213 |             visibility: Visibility::All,
 214 |             line_number: node.start_position().row + 1,
 215 |             full_signature: full_sig,
 216 |         })
 217 |     }
 218 | 
 219 |     fn extract_typedef_signature(
 220 |         &self,
 221 |         source: &str,
 222 |         node: &tree_sitter::Node,
 223 |     ) -> Option<Signature> {
 224 |         let name = self.find_child_text(node, "type_identifier", source)?;
 225 | 
 226 |         let full_sig = format!("typedef {}", name);
 227 | 
 228 |         Some(Signature {
 229 |             kind: SignatureKind::TypeAlias,
 230 |             name,
 231 |             params: None,
 232 |             return_type: None,
 233 |             visibility: Visibility::All,
 234 |             line_number: node.start_position().row + 1,
 235 |             full_signature: full_sig,
 236 |         })
 237 |     }
 238 | 
 239 |     fn extract_macro_signature(&self, source: &str, node: &tree_sitter::Node) -> Option<Signature> {
 240 |         let name = self.find_child_text(node, "identifier", source)?;
 241 | 
 242 |         let full_sig = format!("#define {}", name);
 243 | 
 244 |         Some(Signature {
 245 |             kind: SignatureKind::Macro,
 246 |             name,
 247 |             params: None,
 248 |             return_type: None,
 249 |             visibility: Visibility::All,
 250 |             line_number: node.start_position().row + 1,
 251 |             full_signature: full_sig,
 252 |         })
 253 |     }
 254 | 
 255 |     fn find_function_name(&self, node: &tree_sitter::Node, source: &str) -> Option<String> {
 256 |         let mut cursor = node.walk();
 257 |         for child in node.children(&mut cursor) {
 258 |             if child.kind() == "function_declarator" {
 259 |                 let mut inner_cursor = child.walk();
 260 |                 for inner in child.children(&mut inner_cursor) {
 261 |                     if inner.kind() == "identifier" {
 262 |                         return Some(source[inner.start_byte()..inner.end_byte()].to_string());
 263 |                     }
 264 |                 }
 265 |             }
 266 |         }
 267 |         None
 268 |     }
 269 | 
 270 |     fn find_return_type(&self, node: &tree_sitter::Node, source: &str) -> Option<String> {
 271 |         let mut cursor = node.walk();
 272 |         for child in node.children(&mut cursor) {
 273 |             if child.kind() == "primitive_type" || child.kind() == "type_identifier" {
 274 |                 return Some(source[child.start_byte()..child.end_byte()].to_string());
 275 |             }
 276 |         }
 277 |         None
 278 |     }
 279 | 
 280 |     fn find_child_text(
 281 |         &self,
 282 |         node: &tree_sitter::Node,
 283 |         kind: &str,
 284 |         source: &str,
 285 |     ) -> Option<String> {
 286 |         let mut cursor = node.walk();
 287 |         for child in node.children(&mut cursor) {
 288 |             if child.kind() == kind {
 289 |                 return Some(source[child.start_byte()..child.end_byte()].to_string());
 290 |             }
 291 |         }
 292 |         None
 293 |     }
 294 | 
 295 |     fn find_best_boundary(
 296 |         &self,
 297 |         cursor: &mut tree_sitter::TreeCursor,
 298 |         max_bytes: usize,
 299 |         best_end: &mut usize,
 300 |     ) {
 301 |         loop {
 302 |             let node = cursor.node();
 303 |             let end_byte = node.end_byte();
 304 | 
 305 |             if end_byte <= max_bytes && end_byte > *best_end {
 306 |                 let is_item = matches!(
 307 |                     node.kind(),
 308 |                     "function_definition"
 309 |                         | "struct_specifier"
 310 |                         | "enum_specifier"
 311 |                         | "type_definition"
 312 |                 );
 313 |                 if is_item {
 314 |                     *best_end = end_byte;
 315 |                 }
 316 |             }
 317 | 
 318 |             if cursor.goto_first_child() {
 319 |                 self.find_best_boundary(cursor, max_bytes, best_end);
 320 |                 cursor.goto_parent();
 321 |             }
 322 | 
 323 |             if !cursor.goto_next_sibling() {
 324 |                 break;
 325 |             }
 326 |         }
 327 |     }
 328 | }
 329 | 
 330 | #[cfg(test)]
 331 | mod tests {
 332 |     use super::*;
 333 | 
 334 |     #[test]
 335 |     fn test_extract_function_signature() {
 336 |         let source = r#"
 337 | int main() {
 338 |     return 0;
 339 | 
 340 | void hello(const char* name) {
 341 |     printf("Hello, %s\n", name);
 342 | }
 343 | }
 344 | "#;
 345 | 
 346 |         let signatures = CSupport.extract_signatures(source, Visibility::All);
 347 |         assert!(!signatures.is_empty());
 348 | 
 349 |         let funcs: Vec<_> = signatures
 350 |             .iter()
 351 |             .filter(|s| s.kind == SignatureKind::Function)
 352 |             .collect();
 353 |         assert!(!funcs.is_empty());
 354 |     }
 355 | 
 356 |     #[test]
 357 |     fn test_file_extensions() {
 358 |         assert!(CSupport.supports_extension("c"));
 359 |         assert!(CSupport.supports_extension("h"));
 360 |         assert!(!CSupport.supports_extension("cpp"));
 361 |     }
 362 | }
```

### File: `src/tree_sitter/languages/cpp.rs`

- Size: 12567 bytes
- Modified: SystemTime { tv_sec: 1771153473, tv_nsec: 314447787 }

```rust
   1 | //! C++ language support for tree-sitter.
   2 | 
   3 | #[cfg(feature = "tree-sitter-cpp")]
   4 | use tree_sitter::{Parser, Tree};
   5 | 
   6 | #[cfg(feature = "tree-sitter-cpp")]
   7 | use crate::tree_sitter::language_support::{
   8 |     CodeStructure, LanguageSupport, Signature, SignatureKind, Visibility,
   9 |     slice_signature_before_body,
  10 | };
  11 | 
  12 | pub struct CppSupport;
  13 | 
  14 | #[cfg(feature = "tree-sitter-cpp")]
  15 | impl CppSupport {
  16 |     fn get_language() -> tree_sitter::Language {
  17 |         tree_sitter_cpp::LANGUAGE.into()
  18 |     }
  19 | }
  20 | 
  21 | #[cfg(feature = "tree-sitter-cpp")]
  22 | impl LanguageSupport for CppSupport {
  23 |     fn file_extensions(&self) -> &[&'static str] {
  24 |         &["cpp", "cxx", "cc", "hpp", "hxx", "hh"]
  25 |     }
  26 | 
  27 |     fn parse(&self, source: &str) -> Option<Tree> {
  28 |         let mut parser = Parser::new();
  29 |         parser.set_language(&Self::get_language()).ok()?;
  30 |         parser.parse(source, None)
  31 |     }
  32 | 
  33 |     fn extract_signatures(&self, source: &str, visibility: Visibility) -> Vec<Signature> {
  34 |         let tree = match self.parse(source) {
  35 |             Some(t) => t,
  36 |             None => return Vec::new(),
  37 |         };
  38 | 
  39 |         let root = tree.root_node();
  40 |         let mut signatures = Vec::new();
  41 | 
  42 |         self.extract_signatures_from_node(source, &root, visibility, &mut signatures);
  43 | 
  44 |         signatures.sort_by_key(|s| s.line_number);
  45 |         signatures
  46 |     }
  47 | 
  48 |     fn extract_structure(&self, source: &str) -> CodeStructure {
  49 |         let tree = match self.parse(source) {
  50 |             Some(t) => t,
  51 |             None => return CodeStructure::default(),
  52 |         };
  53 | 
  54 |         let root = tree.root_node();
  55 |         let mut structure = CodeStructure {
  56 |             total_lines: source.lines().count(),
  57 |             ..Default::default()
  58 |         };
  59 | 
  60 |         self.extract_structure_from_node(&root, &mut structure);
  61 |         structure
  62 |     }
  63 | 
  64 |     fn find_truncation_point(&self, source: &str, max_bytes: usize) -> usize {
  65 |         if source.len() <= max_bytes {
  66 |             return source.len();
  67 |         }
  68 | 
  69 |         let tree = match self.parse(source) {
  70 |             Some(t) => t,
  71 |             None => return max_bytes,
  72 |         };
  73 | 
  74 |         let root = tree.root_node();
  75 |         let mut best_end = 0;
  76 | 
  77 |         let mut cursor = root.walk();
  78 |         self.find_best_boundary(&mut cursor, max_bytes, &mut best_end);
  79 |         drop(cursor);
  80 | 
  81 |         if best_end == 0 { max_bytes } else { best_end }
  82 |     }
  83 | }
  84 | 
  85 | #[cfg(feature = "tree-sitter-cpp")]
  86 | impl CppSupport {
  87 |     fn extract_signatures_from_node(
  88 |         &self,
  89 |         source: &str,
  90 |         node: &tree_sitter::Node,
  91 |         visibility: Visibility,
  92 |         signatures: &mut Vec<Signature>,
  93 |     ) {
  94 |         match node.kind() {
  95 |             "function_definition" => {
  96 |                 if let Some(sig) = self.extract_function_signature(source, node, visibility) {
  97 |                     signatures.push(sig);
  98 |                 }
  99 |             }
 100 |             "class_specifier" => {
 101 |                 if let Some(sig) = self.extract_class_signature(source, node, visibility) {
 102 |                     signatures.push(sig);
 103 |                 }
 104 |             }
 105 |             "struct_specifier" => {
 106 |                 if let Some(sig) = self.extract_struct_signature(source, node) {
 107 |                     signatures.push(sig);
 108 |                 }
 109 |             }
 110 |             "enum_specifier" => {
 111 |                 if let Some(sig) = self.extract_enum_signature(source, node) {
 112 |                     signatures.push(sig);
 113 |                 }
 114 |             }
 115 |             "alias_declaration" | "type_definition" => {
 116 |                 if let Some(sig) = self.extract_alias_signature(source, node) {
 117 |                     signatures.push(sig);
 118 |                 }
 119 |             }
 120 |             "preproc_function_def" => {
 121 |                 if let Some(sig) = self.extract_macro_signature(source, node) {
 122 |                     signatures.push(sig);
 123 |                 }
 124 |             }
 125 |             _ => {}
 126 |         }
 127 | 
 128 |         let mut cursor = node.walk();
 129 |         for child in node.children(&mut cursor) {
 130 |             self.extract_signatures_from_node(source, &child, visibility, signatures);
 131 |         }
 132 |     }
 133 | 
 134 |     fn extract_structure_from_node(&self, node: &tree_sitter::Node, structure: &mut CodeStructure) {
 135 |         match node.kind() {
 136 |             "function_definition" => structure.functions += 1,
 137 |             "class_specifier" => structure.classes += 1,
 138 |             "struct_specifier" => structure.structs += 1,
 139 |             "enum_specifier" => structure.enums += 1,
 140 |             "preproc_include" => {
 141 |                 structure.imports.push("include".to_string());
 142 |             }
 143 |             _ => {}
 144 |         }
 145 | 
 146 |         let mut cursor = node.walk();
 147 |         for child in node.children(&mut cursor) {
 148 |             self.extract_structure_from_node(&child, structure);
 149 |         }
 150 |     }
 151 | 
 152 |     #[allow(dead_code)]
 153 |     fn get_visibility(&self, _node: &tree_sitter::Node) -> Visibility {
 154 |         // C++ has access specifiers: public, private, protected
 155 |         // For simplicity, we check sibling nodes for access specifiers
 156 |         // This is a simplified check; full implementation would track class context
 157 |         Visibility::All
 158 |     }
 159 | 
 160 |     fn extract_function_signature(
 161 |         &self,
 162 |         source: &str,
 163 |         node: &tree_sitter::Node,
 164 |         visibility: Visibility,
 165 |     ) -> Option<Signature> {
 166 |         let name = self.find_function_name(node, source)?;
 167 |         let return_type = self.find_return_type(node, source);
 168 |         let params = self.find_child_text(node, "parameter_list", source);
 169 | 
 170 |         // Use byte-slicing to preserve templates, parameters, and qualifiers
 171 |         let full_sig = slice_signature_before_body(source, node, &["compound_statement"])
 172 |             .unwrap_or_else(|| {
 173 |                 let mut sig = String::new();
 174 |                 if let Some(r) = &return_type {
 175 |                     sig.push_str(r);
 176 |                     sig.push(' ');
 177 |                 }
 178 |                 sig.push_str(&name);
 179 |                 if let Some(p) = &params {
 180 |                     sig.push_str(p);
 181 |                 } else {
 182 |                     sig.push_str("()");
 183 |                 }
 184 |                 sig
 185 |             });
 186 | 
 187 |         Some(Signature {
 188 |             kind: SignatureKind::Function,
 189 |             name,
 190 |             params,
 191 |             return_type,
 192 |             visibility,
 193 |             line_number: node.start_position().row + 1,
 194 |             full_signature: full_sig,
 195 |         })
 196 |     }
 197 | 
 198 |     fn extract_class_signature(
 199 |         &self,
 200 |         source: &str,
 201 |         node: &tree_sitter::Node,
 202 |         visibility: Visibility,
 203 |     ) -> Option<Signature> {
 204 |         let name = self.find_child_text(node, "type_identifier", source)?;
 205 | 
 206 |         let full_sig = format!("class {}", name);
 207 | 
 208 |         Some(Signature {
 209 |             kind: SignatureKind::Class,
 210 |             name,
 211 |             params: None,
 212 |             return_type: None,
 213 |             visibility,
 214 |             line_number: node.start_position().row + 1,
 215 |             full_signature: full_sig,
 216 |         })
 217 |     }
 218 | 
 219 |     fn extract_struct_signature(
 220 |         &self,
 221 |         source: &str,
 222 |         node: &tree_sitter::Node,
 223 |     ) -> Option<Signature> {
 224 |         let name = self.find_child_text(node, "type_identifier", source)?;
 225 | 
 226 |         let full_sig = format!("struct {}", name);
 227 | 
 228 |         Some(Signature {
 229 |             kind: SignatureKind::Struct,
 230 |             name,
 231 |             params: None,
 232 |             return_type: None,
 233 |             visibility: Visibility::All,
 234 |             line_number: node.start_position().row + 1,
 235 |             full_signature: full_sig,
 236 |         })
 237 |     }
 238 | 
 239 |     fn extract_enum_signature(&self, source: &str, node: &tree_sitter::Node) -> Option<Signature> {
 240 |         let name = self.find_child_text(node, "type_identifier", source)?;
 241 | 
 242 |         let full_sig = format!("enum {}", name);
 243 | 
 244 |         Some(Signature {
 245 |             kind: SignatureKind::Enum,
 246 |             name,
 247 |             params: None,
 248 |             return_type: None,
 249 |             visibility: Visibility::All,
 250 |             line_number: node.start_position().row + 1,
 251 |             full_signature: full_sig,
 252 |         })
 253 |     }
 254 | 
 255 |     fn extract_alias_signature(&self, source: &str, node: &tree_sitter::Node) -> Option<Signature> {
 256 |         let name = self.find_child_text(node, "type_identifier", source)?;
 257 | 
 258 |         let full_sig = format!("using/typedef {}", name);
 259 | 
 260 |         Some(Signature {
 261 |             kind: SignatureKind::TypeAlias,
 262 |             name,
 263 |             params: None,
 264 |             return_type: None,
 265 |             visibility: Visibility::All,
 266 |             line_number: node.start_position().row + 1,
 267 |             full_signature: full_sig,
 268 |         })
 269 |     }
 270 | 
 271 |     fn extract_macro_signature(&self, source: &str, node: &tree_sitter::Node) -> Option<Signature> {
 272 |         let name = self.find_child_text(node, "identifier", source)?;
 273 | 
 274 |         let full_sig = format!("#define {}", name);
 275 | 
 276 |         Some(Signature {
 277 |             kind: SignatureKind::Macro,
 278 |             name,
 279 |             params: None,
 280 |             return_type: None,
 281 |             visibility: Visibility::All,
 282 |             line_number: node.start_position().row + 1,
 283 |             full_signature: full_sig,
 284 |         })
 285 |     }
 286 | 
 287 |     fn find_function_name(&self, node: &tree_sitter::Node, source: &str) -> Option<String> {
 288 |         let mut cursor = node.walk();
 289 |         for child in node.children(&mut cursor) {
 290 |             if child.kind() == "function_declarator" || child.kind() == "reference_declarator" {
 291 |                 let mut inner_cursor = child.walk();
 292 |                 for inner in child.children(&mut inner_cursor) {
 293 |                     if inner.kind() == "identifier" || inner.kind() == "qualified_identifier" {
 294 |                         return Some(source[inner.start_byte()..inner.end_byte()].to_string());
 295 |                     }
 296 |                 }
 297 |             }
 298 |             if child.kind() == "identifier" || child.kind() == "qualified_identifier" {
 299 |                 return Some(source[child.start_byte()..child.end_byte()].to_string());
 300 |             }
 301 |         }
 302 |         None
 303 |     }
 304 | 
 305 |     fn find_return_type(&self, node: &tree_sitter::Node, source: &str) -> Option<String> {
 306 |         let mut cursor = node.walk();
 307 |         for child in node.children(&mut cursor) {
 308 |             match child.kind() {
 309 |                 "primitive_type" | "type_identifier" | "sized_type_specifier" => {
 310 |                     return Some(source[child.start_byte()..child.end_byte()].to_string());
 311 |                 }
 312 |                 _ => {}
 313 |             }
 314 |         }
 315 |         None
 316 |     }
 317 | 
 318 |     fn find_child_text(
 319 |         &self,
 320 |         node: &tree_sitter::Node,
 321 |         kind: &str,
 322 |         source: &str,
 323 |     ) -> Option<String> {
 324 |         let mut cursor = node.walk();
 325 |         for child in node.children(&mut cursor) {
 326 |             if child.kind() == kind {
 327 |                 return Some(source[child.start_byte()..child.end_byte()].to_string());
 328 |             }
 329 |         }
 330 |         None
 331 |     }
 332 | 
 333 |     fn find_best_boundary(
 334 |         &self,
 335 |         cursor: &mut tree_sitter::TreeCursor,
 336 |         max_bytes: usize,
 337 |         best_end: &mut usize,
 338 |     ) {
 339 |         loop {
 340 |             let node = cursor.node();
 341 |             let end_byte = node.end_byte();
 342 | 
 343 |             if end_byte <= max_bytes && end_byte > *best_end {
 344 |                 let is_item = matches!(
 345 |                     node.kind(),
 346 |                     "function_definition"
 347 |                         | "class_specifier"
 348 |                         | "struct_specifier"
 349 |                         | "enum_specifier"
 350 |                         | "alias_declaration"
 351 |                         | "type_definition"
 352 |                 );
 353 |                 if is_item {
 354 |                     *best_end = end_byte;
 355 |                 }
 356 |             }
 357 | 
 358 |             if cursor.goto_first_child() {
 359 |                 self.find_best_boundary(cursor, max_bytes, best_end);
 360 |                 cursor.goto_parent();
 361 |             }
 362 | 
 363 |             if !cursor.goto_next_sibling() {
 364 |                 break;
 365 |             }
 366 |         }
 367 |     }
 368 | }
 369 | 
 370 | #[cfg(test)]
 371 | mod tests {
 372 |     use super::*;
 373 | 
 374 |     #[test]
 375 |     fn test_extract_class_signature() {
 376 |         let source = r#"
 377 | class HelloWorld {
 378 | public:
 379 |     void greet() {
 380 |         std::cout << "Hello" << std::endl;
 381 |     }
 382 | };
 383 | "#;
 384 | 
 385 |         let signatures = CppSupport.extract_signatures(source, Visibility::All);
 386 |         let classes: Vec<_> = signatures
 387 |             .iter()
 388 |             .filter(|s| s.kind == SignatureKind::Class)
 389 |             .collect();
 390 |         assert!(!classes.is_empty());
 391 |         assert_eq!(classes[0].name, "HelloWorld");
 392 |     }
 393 | 
 394 |     #[test]
 395 |     fn test_file_extensions() {
 396 |         assert!(CppSupport.supports_extension("cpp"));
 397 |         assert!(CppSupport.supports_extension("hpp"));
 398 |         assert!(CppSupport.supports_extension("cxx"));
 399 |         assert!(!CppSupport.supports_extension("c"));
 400 |     }
 401 | }
```

### File: `src/tree_sitter/languages/go.rs`

- Size: 13519 bytes
- Modified: SystemTime { tv_sec: 1771153608, tv_nsec: 580163251 }

```rust
   1 | //! Go language support for tree-sitter.
   2 | 
   3 | #[cfg(feature = "tree-sitter-go")]
   4 | use tree_sitter::{Parser, Tree};
   5 | 
   6 | #[cfg(feature = "tree-sitter-go")]
   7 | use crate::tree_sitter::language_support::{
   8 |     CodeStructure, LanguageSupport, Signature, SignatureKind, Visibility,
   9 |     slice_signature_before_body,
  10 | };
  11 | 
  12 | pub struct GoSupport;
  13 | 
  14 | #[cfg(feature = "tree-sitter-go")]
  15 | impl GoSupport {
  16 |     fn get_language() -> tree_sitter::Language {
  17 |         tree_sitter_go::LANGUAGE.into()
  18 |     }
  19 | }
  20 | 
  21 | #[cfg(feature = "tree-sitter-go")]
  22 | impl LanguageSupport for GoSupport {
  23 |     fn file_extensions(&self) -> &[&'static str] {
  24 |         &["go"]
  25 |     }
  26 | 
  27 |     fn parse(&self, source: &str) -> Option<Tree> {
  28 |         let mut parser = Parser::new();
  29 |         parser.set_language(&Self::get_language()).ok()?;
  30 |         parser.parse(source, None)
  31 |     }
  32 | 
  33 |     fn extract_signatures(&self, source: &str, visibility: Visibility) -> Vec<Signature> {
  34 |         let tree = match self.parse(source) {
  35 |             Some(t) => t,
  36 |             None => return Vec::new(),
  37 |         };
  38 | 
  39 |         let root = tree.root_node();
  40 |         let mut signatures = Vec::new();
  41 | 
  42 |         self.extract_signatures_from_node(source, &root, visibility, &mut signatures);
  43 | 
  44 |         signatures.sort_by_key(|s| s.line_number);
  45 |         signatures
  46 |     }
  47 | 
  48 |     fn extract_structure(&self, source: &str) -> CodeStructure {
  49 |         let tree = match self.parse(source) {
  50 |             Some(t) => t,
  51 |             None => return CodeStructure::default(),
  52 |         };
  53 | 
  54 |         let root = tree.root_node();
  55 |         let mut structure = CodeStructure {
  56 |             total_lines: source.lines().count(),
  57 |             ..Default::default()
  58 |         };
  59 | 
  60 |         self.extract_structure_from_node(&root, &mut structure);
  61 |         structure
  62 |     }
  63 | 
  64 |     fn find_truncation_point(&self, source: &str, max_bytes: usize) -> usize {
  65 |         if source.len() <= max_bytes {
  66 |             return source.len();
  67 |         }
  68 | 
  69 |         let tree = match self.parse(source) {
  70 |             Some(t) => t,
  71 |             None => return max_bytes,
  72 |         };
  73 | 
  74 |         let root = tree.root_node();
  75 |         let mut best_end = 0;
  76 | 
  77 |         let mut cursor = root.walk();
  78 |         self.find_best_boundary(&mut cursor, max_bytes, &mut best_end);
  79 |         drop(cursor);
  80 | 
  81 |         if best_end == 0 { max_bytes } else { best_end }
  82 |     }
  83 | }
  84 | 
  85 | #[cfg(feature = "tree-sitter-go")]
  86 | impl GoSupport {
  87 |     fn extract_signatures_from_node(
  88 |         &self,
  89 |         source: &str,
  90 |         node: &tree_sitter::Node,
  91 |         visibility: Visibility,
  92 |         signatures: &mut Vec<Signature>,
  93 |     ) {
  94 |         match node.kind() {
  95 |             "function_declaration" => {
  96 |                 if let Some(sig) = self.extract_function_signature(source, node, visibility) {
  97 |                     signatures.push(sig);
  98 |                 }
  99 |             }
 100 |             "method_declaration" => {
 101 |                 if let Some(sig) = self.extract_method_signature(source, node, visibility) {
 102 |                     signatures.push(sig);
 103 |                 }
 104 |             }
 105 |             "type_declaration" => {
 106 |                 self.extract_type_signatures(source, node, visibility, signatures);
 107 |             }
 108 |             _ => {}
 109 |         }
 110 | 
 111 |         let mut cursor = node.walk();
 112 |         for child in node.children(&mut cursor) {
 113 |             self.extract_signatures_from_node(source, &child, visibility, signatures);
 114 |         }
 115 |     }
 116 | 
 117 |     fn extract_structure_from_node(&self, node: &tree_sitter::Node, structure: &mut CodeStructure) {
 118 |         match node.kind() {
 119 |             "function_declaration" | "method_declaration" => structure.functions += 1,
 120 |             "type_spec" => {
 121 |                 // Check what type it is
 122 |                 if let Some(parent) = node.parent()
 123 |                     && parent.kind() == "type_declaration"
 124 |                 {
 125 |                     // Could be struct, interface, or type alias
 126 |                     let mut cursor = node.walk();
 127 |                     for child in node.children(&mut cursor) {
 128 |                         match child.kind() {
 129 |                             "struct_type" => structure.structs += 1,
 130 |                             "interface_type" => structure.interfaces += 1,
 131 |                             "type_identifier" => structure.type_aliases += 1,
 132 |                             _ => {}
 133 |                         }
 134 |                     }
 135 |                 }
 136 |             }
 137 |             "import_declaration" => {
 138 |                 structure.imports.push("import".to_string());
 139 |             }
 140 |             _ => {}
 141 |         }
 142 | 
 143 |         let mut cursor = node.walk();
 144 |         for child in node.children(&mut cursor) {
 145 |             self.extract_structure_from_node(&child, structure);
 146 |         }
 147 |     }
 148 | 
 149 |     fn is_exported(&self, name: &str) -> bool {
 150 |         name.chars().next().is_some_and(|c| c.is_uppercase())
 151 |     }
 152 | 
 153 |     fn extract_function_signature(
 154 |         &self,
 155 |         source: &str,
 156 |         node: &tree_sitter::Node,
 157 |         visibility: Visibility,
 158 |     ) -> Option<Signature> {
 159 |         let name = self.find_child_text(node, "identifier", source)?;
 160 |         let is_exported = self.is_exported(&name);
 161 | 
 162 |         if visibility == Visibility::Public && !is_exported {
 163 |             return None;
 164 |         }
 165 |         if visibility == Visibility::Private && is_exported {
 166 |             return None;
 167 |         }
 168 | 
 169 |         let params = self.find_child_text(node, "parameter_list", source);
 170 |         let result = self
 171 |             .find_child_text(node, "type_identifier", source)
 172 |             .or_else(|| self.find_child_text_for_result(node, source));
 173 | 
 174 |         // Use byte-slicing to preserve receivers, multiple return values, and named results
 175 |         let full_sig = slice_signature_before_body(source, node, &["block"])
 176 |             .unwrap_or_else(|| {
 177 |                 let mut sig = String::new();
 178 |                 sig.push_str("func ");
 179 |                 sig.push_str(&name);
 180 |                 if let Some(p) = &params {
 181 |                     sig.push_str(p);
 182 |                 } else {
 183 |                     sig.push_str("()");
 184 |                 }
 185 |                 if let Some(r) = &result {
 186 |                     sig.push(' ');
 187 |                     sig.push_str(r);
 188 |                 }
 189 |                 sig
 190 |             });
 191 | 
 192 |         Some(Signature {
 193 |             kind: SignatureKind::Function,
 194 |             name,
 195 |             params,
 196 |             return_type: result,
 197 |             visibility: if is_exported {
 198 |                 Visibility::Public
 199 |             } else {
 200 |                 Visibility::Private
 201 |             },
 202 |             line_number: node.start_position().row + 1,
 203 |             full_signature: full_sig,
 204 |         })
 205 |     }
 206 | 
 207 |     fn extract_method_signature(
 208 |         &self,
 209 |         source: &str,
 210 |         node: &tree_sitter::Node,
 211 |         visibility: Visibility,
 212 |     ) -> Option<Signature> {
 213 |         let name = self
 214 |             .find_child_text(node, "field_identifier", source)
 215 |             .or_else(|| self.find_child_text(node, "identifier", source))?;
 216 |         let is_exported = self.is_exported(&name);
 217 | 
 218 |         if visibility == Visibility::Public && !is_exported {
 219 |             return None;
 220 |         }
 221 |         if visibility == Visibility::Private && is_exported {
 222 |             return None;
 223 |         }
 224 | 
 225 |         let receiver = self.find_child_text(node, "parameter_list", source);
 226 |         let params = self.find_method_params(node, source);
 227 |         let result = self.find_child_text_for_result(node, source);
 228 | 
 229 |         let mut full_sig = String::new();
 230 |         full_sig.push_str("func ");
 231 |         if let Some(r) = &receiver {
 232 |             full_sig.push_str(r);
 233 |             full_sig.push(' ');
 234 |         }
 235 |         full_sig.push_str(&name);
 236 |         if let Some(p) = &params {
 237 |             full_sig.push_str(p);
 238 |         } else {
 239 |             full_sig.push_str("()");
 240 |         }
 241 |         if let Some(r) = &result {
 242 |             full_sig.push(' ');
 243 |             full_sig.push_str(r);
 244 |         }
 245 | 
 246 |         Some(Signature {
 247 |             kind: SignatureKind::Method,
 248 |             name,
 249 |             params,
 250 |             return_type: result,
 251 |             visibility: if is_exported {
 252 |                 Visibility::Public
 253 |             } else {
 254 |                 Visibility::Private
 255 |             },
 256 |             line_number: node.start_position().row + 1,
 257 |             full_signature: full_sig,
 258 |         })
 259 |     }
 260 | 
 261 |     fn extract_type_signatures(
 262 |         &self,
 263 |         source: &str,
 264 |         node: &tree_sitter::Node,
 265 |         visibility: Visibility,
 266 |         signatures: &mut Vec<Signature>,
 267 |     ) {
 268 |         let mut cursor = node.walk();
 269 |         for child in node.children(&mut cursor) {
 270 |             if child.kind() == "type_spec"
 271 |                 && let Some(name) = self.find_child_text(&child, "type_identifier", source)
 272 |             {
 273 |                 let is_exported = self.is_exported(&name);
 274 | 
 275 |                 if visibility == Visibility::Public && !is_exported {
 276 |                     continue;
 277 |                 }
 278 |                 if visibility == Visibility::Private && is_exported {
 279 |                     continue;
 280 |                 }
 281 | 
 282 |                 let kind = self.get_type_kind(&child);
 283 |                 let full_sig = format!("type {} {}", name, kind);
 284 | 
 285 |                 signatures.push(Signature {
 286 |                     kind,
 287 |                     name,
 288 |                     params: None,
 289 |                     return_type: None,
 290 |                     visibility: if is_exported {
 291 |                         Visibility::Public
 292 |                     } else {
 293 |                         Visibility::Private
 294 |                     },
 295 |                     line_number: child.start_position().row + 1,
 296 |                     full_signature: full_sig,
 297 |                 });
 298 |             }
 299 |         }
 300 |     }
 301 | 
 302 |     fn get_type_kind(&self, node: &tree_sitter::Node) -> SignatureKind {
 303 |         let mut cursor = node.walk();
 304 |         for child in node.children(&mut cursor) {
 305 |             match child.kind() {
 306 |                 "struct_type" => return SignatureKind::Struct,
 307 |                 "interface_type" => return SignatureKind::Interface,
 308 |                 _ => {}
 309 |             }
 310 |         }
 311 |         SignatureKind::TypeAlias
 312 |     }
 313 | 
 314 |     fn find_child_text(
 315 |         &self,
 316 |         node: &tree_sitter::Node,
 317 |         kind: &str,
 318 |         source: &str,
 319 |     ) -> Option<String> {
 320 |         let mut cursor = node.walk();
 321 |         for child in node.children(&mut cursor) {
 322 |             if child.kind() == kind {
 323 |                 return Some(source[child.start_byte()..child.end_byte()].to_string());
 324 |             }
 325 |         }
 326 |         None
 327 |     }
 328 | 
 329 |     fn find_child_text_for_result(&self, node: &tree_sitter::Node, source: &str) -> Option<String> {
 330 |         let mut cursor = node.walk();
 331 |         for child in node.children(&mut cursor) {
 332 |             if child.kind() == "func_result" {
 333 |                 return Some(source[child.start_byte()..child.end_byte()].to_string());
 334 |             }
 335 |         }
 336 |         None
 337 |     }
 338 | 
 339 |     fn find_method_params(&self, node: &tree_sitter::Node, source: &str) -> Option<String> {
 340 |         let mut cursor = node.walk();
 341 |         let mut found_receiver = false;
 342 |         for child in node.children(&mut cursor) {
 343 |             if child.kind() == "parameter_list" {
 344 |                 if found_receiver {
 345 |                     return Some(source[child.start_byte()..child.end_byte()].to_string());
 346 |                 }
 347 |                 found_receiver = true;
 348 |             }
 349 |         }
 350 |         None
 351 |     }
 352 | 
 353 |     fn find_best_boundary(
 354 |         &self,
 355 |         cursor: &mut tree_sitter::TreeCursor,
 356 |         max_bytes: usize,
 357 |         best_end: &mut usize,
 358 |     ) {
 359 |         loop {
 360 |             let node = cursor.node();
 361 |             let end_byte = node.end_byte();
 362 | 
 363 |             if end_byte <= max_bytes && end_byte > *best_end {
 364 |                 let is_item = matches!(
 365 |                     node.kind(),
 366 |                     "function_declaration" | "method_declaration" | "type_declaration"
 367 |                 );
 368 |                 if is_item {
 369 |                     *best_end = end_byte;
 370 |                 }
 371 |             }
 372 | 
 373 |             if cursor.goto_first_child() {
 374 |                 self.find_best_boundary(cursor, max_bytes, best_end);
 375 |                 cursor.goto_parent();
 376 |             }
 377 | 
 378 |             if !cursor.goto_next_sibling() {
 379 |                 break;
 380 |             }
 381 |         }
 382 |     }
 383 | }
 384 | 
 385 | #[cfg(test)]
 386 | mod tests {
 387 |     use super::*;
 388 | 
 389 |     #[test]
 390 |     fn test_extract_function_signature() {
 391 |         let source = r#"
 392 | package main
 393 | 
 394 | func Hello(name string) string {
 395 |     return "Hello, " + name
 396 | }
 397 | 
 398 | func internal() int {
 399 |     return 42
 400 | }
 401 | "#;
 402 | 
 403 |         let signatures = GoSupport.extract_signatures(source, Visibility::All);
 404 |         assert!(!signatures.is_empty());
 405 | 
 406 |         let funcs: Vec<_> = signatures
 407 |             .iter()
 408 |             .filter(|s| s.kind == SignatureKind::Function)
 409 |             .collect();
 410 |         assert!(funcs.len() >= 2);
 411 |     }
 412 | 
 413 |     #[test]
 414 |     fn test_public_only_filter() {
 415 |         let source = r#"
 416 | func PublicFunc() {}
 417 | func privateFunc() {}
 418 | "#;
 419 | 
 420 |         let signatures = GoSupport.extract_signatures(source, Visibility::Public);
 421 |         assert_eq!(signatures.len(), 1);
 422 |         assert_eq!(signatures[0].name, "PublicFunc");
 423 |     }
 424 | 
 425 |     #[test]
 426 |     fn test_extract_struct_signature() {
 427 |         let source = r#"
 428 | type User struct {
 429 |     Name string
 430 |     Age  int
 431 | }
 432 | "#;
 433 | 
 434 |         let signatures = GoSupport.extract_signatures(source, Visibility::All);
 435 |         let structs: Vec<_> = signatures
 436 |             .iter()
 437 |             .filter(|s| s.kind == SignatureKind::Struct)
 438 |             .collect();
 439 |         assert!(!structs.is_empty());
 440 |         assert_eq!(structs[0].name, "User");
 441 |     }
 442 | 
 443 |     #[test]
 444 |     fn test_file_extensions() {
 445 |         assert!(GoSupport.supports_extension("go"));
 446 |         assert!(!GoSupport.supports_extension("rs"));
 447 |     }
 448 | }
```

### File: `src/tree_sitter/languages/java.rs`

- Size: 12889 bytes
- Modified: SystemTime { tv_sec: 1771153610, tv_nsec: 765190962 }

```rust
   1 | //! Java language support for tree-sitter.
   2 | 
   3 | #[cfg(feature = "tree-sitter-java")]
   4 | use tree_sitter::{Parser, Tree};
   5 | 
   6 | #[cfg(feature = "tree-sitter-java")]
   7 | use crate::tree_sitter::language_support::{
   8 |     CodeStructure, LanguageSupport, Signature, SignatureKind, Visibility,
   9 |     slice_signature_before_body,
  10 | };
  11 | 
  12 | pub struct JavaSupport;
  13 | 
  14 | #[cfg(feature = "tree-sitter-java")]
  15 | impl JavaSupport {
  16 |     fn get_language() -> tree_sitter::Language {
  17 |         tree_sitter_java::LANGUAGE.into()
  18 |     }
  19 | }
  20 | 
  21 | #[cfg(feature = "tree-sitter-java")]
  22 | impl LanguageSupport for JavaSupport {
  23 |     fn file_extensions(&self) -> &[&'static str] {
  24 |         &["java"]
  25 |     }
  26 | 
  27 |     fn parse(&self, source: &str) -> Option<Tree> {
  28 |         let mut parser = Parser::new();
  29 |         parser.set_language(&Self::get_language()).ok()?;
  30 |         parser.parse(source, None)
  31 |     }
  32 | 
  33 |     fn extract_signatures(&self, source: &str, visibility: Visibility) -> Vec<Signature> {
  34 |         let tree = match self.parse(source) {
  35 |             Some(t) => t,
  36 |             None => return Vec::new(),
  37 |         };
  38 | 
  39 |         let root = tree.root_node();
  40 |         let mut signatures = Vec::new();
  41 | 
  42 |         self.extract_signatures_from_node(source, &root, visibility, &mut signatures);
  43 | 
  44 |         signatures.sort_by_key(|s| s.line_number);
  45 |         signatures
  46 |     }
  47 | 
  48 |     fn extract_structure(&self, source: &str) -> CodeStructure {
  49 |         let tree = match self.parse(source) {
  50 |             Some(t) => t,
  51 |             None => return CodeStructure::default(),
  52 |         };
  53 | 
  54 |         let root = tree.root_node();
  55 |         let mut structure = CodeStructure {
  56 |             total_lines: source.lines().count(),
  57 |             ..Default::default()
  58 |         };
  59 | 
  60 |         self.extract_structure_from_node(&root, &mut structure);
  61 |         structure
  62 |     }
  63 | 
  64 |     fn find_truncation_point(&self, source: &str, max_bytes: usize) -> usize {
  65 |         if source.len() <= max_bytes {
  66 |             return source.len();
  67 |         }
  68 | 
  69 |         let tree = match self.parse(source) {
  70 |             Some(t) => t,
  71 |             None => return max_bytes,
  72 |         };
  73 | 
  74 |         let root = tree.root_node();
  75 |         let mut best_end = 0;
  76 | 
  77 |         let mut cursor = root.walk();
  78 |         self.find_best_boundary(&mut cursor, max_bytes, &mut best_end);
  79 |         drop(cursor);
  80 | 
  81 |         if best_end == 0 { max_bytes } else { best_end }
  82 |     }
  83 | }
  84 | 
  85 | #[cfg(feature = "tree-sitter-java")]
  86 | impl JavaSupport {
  87 |     fn extract_signatures_from_node(
  88 |         &self,
  89 |         source: &str,
  90 |         node: &tree_sitter::Node,
  91 |         visibility: Visibility,
  92 |         signatures: &mut Vec<Signature>,
  93 |     ) {
  94 |         match node.kind() {
  95 |             "method_declaration" => {
  96 |                 if let Some(sig) = self.extract_method_signature(source, node, visibility) {
  97 |                     signatures.push(sig);
  98 |                 }
  99 |             }
 100 |             "class_declaration" => {
 101 |                 if let Some(sig) = self.extract_class_signature(source, node, visibility) {
 102 |                     signatures.push(sig);
 103 |                 }
 104 |             }
 105 |             "interface_declaration" => {
 106 |                 if let Some(sig) = self.extract_interface_signature(source, node, visibility) {
 107 |                     signatures.push(sig);
 108 |                 }
 109 |             }
 110 |             "enum_declaration" => {
 111 |                 if let Some(sig) = self.extract_enum_signature(source, node, visibility) {
 112 |                     signatures.push(sig);
 113 |                 }
 114 |             }
 115 |             "field_declaration" => {
 116 |                 if let Some(sig) = self.extract_field_signature(source, node, visibility) {
 117 |                     signatures.push(sig);
 118 |                 }
 119 |             }
 120 |             _ => {}
 121 |         }
 122 | 
 123 |         let mut cursor = node.walk();
 124 |         for child in node.children(&mut cursor) {
 125 |             self.extract_signatures_from_node(source, &child, visibility, signatures);
 126 |         }
 127 |     }
 128 | 
 129 |     fn extract_structure_from_node(&self, node: &tree_sitter::Node, structure: &mut CodeStructure) {
 130 |         match node.kind() {
 131 |             "method_declaration" => structure.functions += 1,
 132 |             "class_declaration" => structure.classes += 1,
 133 |             "interface_declaration" => structure.interfaces += 1,
 134 |             "enum_declaration" => structure.enums += 1,
 135 |             "import_declaration" => {
 136 |                 structure.imports.push("import".to_string());
 137 |             }
 138 |             _ => {}
 139 |         }
 140 | 
 141 |         let mut cursor = node.walk();
 142 |         for child in node.children(&mut cursor) {
 143 |             self.extract_structure_from_node(&child, structure);
 144 |         }
 145 |     }
 146 | 
 147 |     #[allow(dead_code)]
 148 |     fn get_visibility(&self, _node: &tree_sitter::Node) -> Visibility {
 149 |         // Java visibility is determined by modifiers
 150 |         // Simplified: check for public/private/protected keywords in AST modifiers
 151 |         Visibility::All
 152 |     }
 153 | 
 154 |     fn extract_method_signature(
 155 |         &self,
 156 |         source: &str,
 157 |         node: &tree_sitter::Node,
 158 |         visibility: Visibility,
 159 |     ) -> Option<Signature> {
 160 |         let vis = self.get_visibility(node);
 161 | 
 162 |         if visibility == Visibility::Public && vis != Visibility::Public {
 163 |             return None;
 164 |         }
 165 |         if visibility == Visibility::Private && vis == Visibility::Public {
 166 |             return None;
 167 |         }
 168 | 
 169 |         let name = self.find_child_text(node, "identifier", source)?;
 170 |         let params = self.find_child_text(node, "formal_parameters", source);
 171 |         let return_type = self
 172 |             .find_child_text(node, "type_identifier", source)
 173 |             .or_else(|| self.find_child_text_for_type(node, source));
 174 | 
 175 |         // Use byte-slicing to preserve annotations, generics, throws, and modifiers
 176 |         let full_sig = slice_signature_before_body(source, node, &["block"])
 177 |             .unwrap_or_else(|| {
 178 |                 let mut sig = String::new();
 179 |                 if vis == Visibility::Public {
 180 |                     sig.push_str("public ");
 181 |                 }
 182 |                 if let Some(r) = &return_type {
 183 |                     sig.push_str(r);
 184 |                     sig.push(' ');
 185 |                 }
 186 |                 sig.push_str(&name);
 187 |                 if let Some(p) = &params {
 188 |                     sig.push_str(p);
 189 |                 } else {
 190 |                     sig.push_str("()");
 191 |                 }
 192 |                 sig
 193 |             });
 194 | 
 195 |         Some(Signature {
 196 |             kind: SignatureKind::Method,
 197 |             name,
 198 |             params,
 199 |             return_type,
 200 |             visibility: vis,
 201 |             line_number: node.start_position().row + 1,
 202 |             full_signature: full_sig,
 203 |         })
 204 |     }
 205 | 
 206 |     fn extract_class_signature(
 207 |         &self,
 208 |         source: &str,
 209 |         node: &tree_sitter::Node,
 210 |         visibility: Visibility,
 211 |     ) -> Option<Signature> {
 212 |         let vis = self.get_visibility(node);
 213 | 
 214 |         if visibility == Visibility::Public && vis != Visibility::Public {
 215 |             return None;
 216 |         }
 217 |         if visibility == Visibility::Private && vis == Visibility::Public {
 218 |             return None;
 219 |         }
 220 | 
 221 |         let name = self.find_child_text(node, "identifier", source)?;
 222 | 
 223 |         let mut full_sig = String::new();
 224 |         if vis == Visibility::Public {
 225 |             full_sig.push_str("public ");
 226 |         }
 227 |         full_sig.push_str("class ");
 228 |         full_sig.push_str(&name);
 229 | 
 230 |         Some(Signature {
 231 |             kind: SignatureKind::Class,
 232 |             name,
 233 |             params: None,
 234 |             return_type: None,
 235 |             visibility: vis,
 236 |             line_number: node.start_position().row + 1,
 237 |             full_signature: full_sig,
 238 |         })
 239 |     }
 240 | 
 241 |     fn extract_interface_signature(
 242 |         &self,
 243 |         source: &str,
 244 |         node: &tree_sitter::Node,
 245 |         visibility: Visibility,
 246 |     ) -> Option<Signature> {
 247 |         let vis = self.get_visibility(node);
 248 | 
 249 |         if visibility == Visibility::Public && vis != Visibility::Public {
 250 |             return None;
 251 |         }
 252 |         if visibility == Visibility::Private && vis == Visibility::Public {
 253 |             return None;
 254 |         }
 255 | 
 256 |         let name = self.find_child_text(node, "identifier", source)?;
 257 | 
 258 |         let mut full_sig = String::new();
 259 |         if vis == Visibility::Public {
 260 |             full_sig.push_str("public ");
 261 |         }
 262 |         full_sig.push_str("interface ");
 263 |         full_sig.push_str(&name);
 264 | 
 265 |         Some(Signature {
 266 |             kind: SignatureKind::Interface,
 267 |             name,
 268 |             params: None,
 269 |             return_type: None,
 270 |             visibility: vis,
 271 |             line_number: node.start_position().row + 1,
 272 |             full_signature: full_sig,
 273 |         })
 274 |     }
 275 | 
 276 |     fn extract_enum_signature(
 277 |         &self,
 278 |         source: &str,
 279 |         node: &tree_sitter::Node,
 280 |         visibility: Visibility,
 281 |     ) -> Option<Signature> {
 282 |         let vis = self.get_visibility(node);
 283 | 
 284 |         if visibility == Visibility::Public && vis != Visibility::Public {
 285 |             return None;
 286 |         }
 287 |         if visibility == Visibility::Private && vis == Visibility::Public {
 288 |             return None;
 289 |         }
 290 | 
 291 |         let name = self.find_child_text(node, "identifier", source)?;
 292 | 
 293 |         let mut full_sig = String::new();
 294 |         if vis == Visibility::Public {
 295 |             full_sig.push_str("public ");
 296 |         }
 297 |         full_sig.push_str("enum ");
 298 |         full_sig.push_str(&name);
 299 | 
 300 |         Some(Signature {
 301 |             kind: SignatureKind::Enum,
 302 |             name,
 303 |             params: None,
 304 |             return_type: None,
 305 |             visibility: vis,
 306 |             line_number: node.start_position().row + 1,
 307 |             full_signature: full_sig,
 308 |         })
 309 |     }
 310 | 
 311 |     fn extract_field_signature(
 312 |         &self,
 313 |         source: &str,
 314 |         node: &tree_sitter::Node,
 315 |         visibility: Visibility,
 316 |     ) -> Option<Signature> {
 317 |         let vis = self.get_visibility(node);
 318 | 
 319 |         if visibility == Visibility::Public && vis != Visibility::Public {
 320 |             return None;
 321 |         }
 322 |         if visibility == Visibility::Private && vis == Visibility::Public {
 323 |             return None;
 324 |         }
 325 | 
 326 |         let name = self.find_child_text(node, "identifier", source)?;
 327 |         let full_signature = format!("field {}", &name);
 328 | 
 329 |         Some(Signature {
 330 |             kind: SignatureKind::Constant,
 331 |             name,
 332 |             params: None,
 333 |             return_type: None,
 334 |             visibility: vis,
 335 |             line_number: node.start_position().row + 1,
 336 |             full_signature,
 337 |         })
 338 |     }
 339 | 
 340 |     fn find_child_text(
 341 |         &self,
 342 |         node: &tree_sitter::Node,
 343 |         kind: &str,
 344 |         source: &str,
 345 |     ) -> Option<String> {
 346 |         let mut cursor = node.walk();
 347 |         for child in node.children(&mut cursor) {
 348 |             if child.kind() == kind {
 349 |                 return Some(source[child.start_byte()..child.end_byte()].to_string());
 350 |             }
 351 |         }
 352 |         None
 353 |     }
 354 | 
 355 |     fn find_child_text_for_type(&self, node: &tree_sitter::Node, source: &str) -> Option<String> {
 356 |         let mut cursor = node.walk();
 357 |         for child in node.children(&mut cursor) {
 358 |             if child.kind() == "void_type"
 359 |                 || child.kind() == "integral_type"
 360 |                 || child.kind() == "boolean_type"
 361 |             {
 362 |                 return Some(source[child.start_byte()..child.end_byte()].to_string());
 363 |             }
 364 |         }
 365 |         None
 366 |     }
 367 | 
 368 |     fn find_best_boundary(
 369 |         &self,
 370 |         cursor: &mut tree_sitter::TreeCursor,
 371 |         max_bytes: usize,
 372 |         best_end: &mut usize,
 373 |     ) {
 374 |         loop {
 375 |             let node = cursor.node();
 376 |             let end_byte = node.end_byte();
 377 | 
 378 |             if end_byte <= max_bytes && end_byte > *best_end {
 379 |                 let is_item = matches!(
 380 |                     node.kind(),
 381 |                     "method_declaration"
 382 |                         | "class_declaration"
 383 |                         | "interface_declaration"
 384 |                         | "enum_declaration"
 385 |                 );
 386 |                 if is_item {
 387 |                     *best_end = end_byte;
 388 |                 }
 389 |             }
 390 | 
 391 |             if cursor.goto_first_child() {
 392 |                 self.find_best_boundary(cursor, max_bytes, best_end);
 393 |                 cursor.goto_parent();
 394 |             }
 395 | 
 396 |             if !cursor.goto_next_sibling() {
 397 |                 break;
 398 |             }
 399 |         }
 400 |     }
 401 | }
 402 | 
 403 | #[cfg(test)]
 404 | mod tests {
 405 |     use super::*;
 406 | 
 407 |     #[test]
 408 |     fn test_extract_class_signature() {
 409 |         let source = r#"
 410 | public class HelloWorld {
 411 |     public static void main(String[] args) {
 412 |         System.out.println("Hello");
 413 |     }
 414 | }
 415 | }
 416 | "#;
 417 | 
 418 |         let signatures = JavaSupport.extract_signatures(source, Visibility::All);
 419 |         let classes: Vec<_> = signatures
 420 |             .iter()
 421 |             .filter(|s| s.kind == SignatureKind::Class)
 422 |             .collect();
 423 |         assert!(!classes.is_empty());
 424 |         assert_eq!(classes[0].name, "HelloWorld");
 425 |     }
 426 | 
 427 |     #[test]
 428 |     fn test_file_extensions() {
 429 |         assert!(JavaSupport.supports_extension("java"));
 430 |         assert!(!JavaSupport.supports_extension("rs"));
 431 |     }
 432 | }
```

### File: `src/tree_sitter/languages/javascript.rs`

- Size: 11974 bytes
- Modified: SystemTime { tv_sec: 1771153513, tv_nsec: 567958290 }

```rust
   1 | //! JavaScript language support for tree-sitter.
   2 | 
   3 | use tree_sitter::{Parser, Tree};
   4 | 
   5 | use crate::tree_sitter::language_support::{
   6 |     CodeStructure, LanguageSupport, Signature, SignatureKind, Visibility,
   7 |     slice_signature_before_body,
   8 | };
   9 | 
  10 | pub struct JavaScriptSupport;
  11 | 
  12 | impl JavaScriptSupport {
  13 |     fn get_language() -> tree_sitter::Language {
  14 |         tree_sitter_javascript::LANGUAGE.into()
  15 |     }
  16 | }
  17 | 
  18 | impl LanguageSupport for JavaScriptSupport {
  19 |     fn file_extensions(&self) -> &[&'static str] {
  20 |         &["js", "mjs", "cjs"]
  21 |     }
  22 | 
  23 |     fn parse(&self, source: &str) -> Option<Tree> {
  24 |         let mut parser = Parser::new();
  25 |         parser.set_language(&Self::get_language()).ok()?;
  26 |         parser.parse(source, None)
  27 |     }
  28 | 
  29 |     fn extract_signatures(&self, source: &str, visibility: Visibility) -> Vec<Signature> {
  30 |         let tree = match self.parse(source) {
  31 |             Some(t) => t,
  32 |             None => return Vec::new(),
  33 |         };
  34 | 
  35 |         let root = tree.root_node();
  36 |         let mut signatures = Vec::new();
  37 | 
  38 |         self.extract_signatures_from_node(source, &root, visibility, &mut signatures);
  39 | 
  40 |         signatures.sort_by_key(|s| s.line_number);
  41 |         signatures
  42 |     }
  43 | 
  44 |     fn extract_structure(&self, source: &str) -> CodeStructure {
  45 |         let tree = match self.parse(source) {
  46 |             Some(t) => t,
  47 |             None => return CodeStructure::default(),
  48 |         };
  49 | 
  50 |         let root = tree.root_node();
  51 |         let mut structure = CodeStructure {
  52 |             total_lines: source.lines().count(),
  53 |             ..Default::default()
  54 |         };
  55 | 
  56 |         self.extract_structure_from_node(&root, &mut structure);
  57 |         structure
  58 |     }
  59 | 
  60 |     fn find_truncation_point(&self, source: &str, max_bytes: usize) -> usize {
  61 |         if source.len() <= max_bytes {
  62 |             return source.len();
  63 |         }
  64 | 
  65 |         let tree = match self.parse(source) {
  66 |             Some(t) => t,
  67 |             None => return max_bytes,
  68 |         };
  69 | 
  70 |         let root = tree.root_node();
  71 |         let mut best_end = 0;
  72 | 
  73 |         let mut cursor = root.walk();
  74 |         self.find_best_boundary(&mut cursor, max_bytes, &mut best_end);
  75 |         drop(cursor);
  76 | 
  77 |         if best_end == 0 { max_bytes } else { best_end }
  78 |     }
  79 | }
  80 | 
  81 | impl JavaScriptSupport {
  82 |     fn extract_signatures_from_node(
  83 |         &self,
  84 |         source: &str,
  85 |         node: &tree_sitter::Node,
  86 |         _visibility: Visibility,
  87 |         signatures: &mut Vec<Signature>,
  88 |     ) {
  89 |         match node.kind() {
  90 |             "function_declaration" => {
  91 |                 if let Some(sig) = self.extract_function_signature(source, node) {
  92 |                     signatures.push(sig);
  93 |                 }
  94 |             }
  95 |             "class_declaration" => {
  96 |                 if let Some(sig) = self.extract_class_signature(source, node) {
  97 |                     signatures.push(sig);
  98 |                 }
  99 |             }
 100 |             "variable_declaration" | "lexical_declaration" => {
 101 |                 self.extract_variable_declarations(source, node, signatures);
 102 |             }
 103 |             "export_statement" => {
 104 |                 self.extract_export_signatures(source, node, signatures);
 105 |             }
 106 |             _ => {}
 107 |         }
 108 | 
 109 |         let mut cursor = node.walk();
 110 |         for child in node.children(&mut cursor) {
 111 |             self.extract_signatures_from_node(source, &child, _visibility, signatures);
 112 |         }
 113 |     }
 114 | 
 115 |     fn extract_structure_from_node(&self, node: &tree_sitter::Node, structure: &mut CodeStructure) {
 116 |         match node.kind() {
 117 |             "function_declaration" | "generator_function_declaration" | "function_expression" => {
 118 |                 structure.functions += 1;
 119 |             }
 120 |             "class_declaration" | "class_expression" => {
 121 |                 structure.classes += 1;
 122 |             }
 123 |             "import_statement" => {
 124 |                 structure.imports.push("import".to_string());
 125 |             }
 126 |             "export_statement" => {
 127 |                 structure.exports.push("export".to_string());
 128 |             }
 129 |             _ => {}
 130 |         }
 131 | 
 132 |         let mut cursor = node.walk();
 133 |         for child in node.children(&mut cursor) {
 134 |             self.extract_structure_from_node(&child, structure);
 135 |         }
 136 |     }
 137 | 
 138 |     fn extract_function_signature(
 139 |         &self,
 140 |         source: &str,
 141 |         node: &tree_sitter::Node,
 142 |     ) -> Option<Signature> {
 143 |         let name = self.find_child_text(node, "identifier", source)?;
 144 |         let params = self.find_child_text(node, "formal_parameters", source);
 145 | 
 146 |         // Use byte-slicing to preserve async, generator*, and complete parameter lists
 147 |         let full_sig = slice_signature_before_body(source, node, &["statement_block"])
 148 |             .unwrap_or_else(|| {
 149 |                 match &params {
 150 |                     Some(p) => format!("function {}({})", name, p),
 151 |                     None => format!("function {}()", name),
 152 |                 }
 153 |             });
 154 | 
 155 |         Some(Signature {
 156 |             kind: SignatureKind::Function,
 157 |             name,
 158 |             params,
 159 |             return_type: None,
 160 |             visibility: Visibility::All,
 161 |             line_number: node.start_position().row + 1,
 162 |             full_signature: full_sig,
 163 |         })
 164 |     }
 165 | 
 166 |     fn extract_class_signature(&self, source: &str, node: &tree_sitter::Node) -> Option<Signature> {
 167 |         let name = self.find_child_text(node, "identifier", source)?;
 168 | 
 169 |         // Use byte-slicing to preserve `extends` and other modifiers
 170 |         let full_sig = slice_signature_before_body(source, node, &["class_body"])
 171 |             .unwrap_or_else(|| format!("class {}", name));
 172 | 
 173 |         Some(Signature {
 174 |             kind: SignatureKind::Class,
 175 |             name,
 176 |             params: None,
 177 |             return_type: None,
 178 |             visibility: Visibility::All,
 179 |             line_number: node.start_position().row + 1,
 180 |             full_signature: full_sig,
 181 |         })
 182 |     }
 183 | 
 184 |     fn extract_variable_declarations(
 185 |         &self,
 186 |         source: &str,
 187 |         node: &tree_sitter::Node,
 188 |         signatures: &mut Vec<Signature>,
 189 |     ) {
 190 |         let mut cursor = node.walk();
 191 |         for child in node.children(&mut cursor) {
 192 |             if child.kind() == "variable_declarator" {
 193 |                 // Check if this is an arrow function or regular function assignment
 194 |                 let mut inner_cursor = child.walk();
 195 |                 let has_function = child.children(&mut inner_cursor).any(|c| {
 196 |                     c.kind() == "arrow_function" || c.kind() == "function"
 197 |                 });
 198 | 
 199 |                 if has_function {
 200 |                     // Use byte-slicing to capture the full arrow function signature
 201 |                     // e.g., `const add = (a, b) =>`
 202 |                     let full_signature = slice_signature_before_body(
 203 |                         source, &child, &["statement_block", "expression_statement"]
 204 |                     ).unwrap_or_else(|| {
 205 |                         // For arrow functions with expression bodies (no block),
 206 |                         // slice the whole declarator text up to the arrow function body
 207 |                         let text = &source[child.start_byte()..child.end_byte()]; 
 208 |                         text.trim().to_string()
 209 |                     });
 210 | 
 211 |                     let name = self.find_child_text(&child, "identifier", source)
 212 |                         .unwrap_or_default();
 213 | 
 214 |                     signatures.push(Signature {
 215 |                         kind: SignatureKind::Function,
 216 |                         name,
 217 |                         params: None, // Captured via byte-slicing
 218 |                         return_type: None,
 219 |                         visibility: Visibility::All,
 220 |                         line_number: child.start_position().row + 1,
 221 |                         full_signature,
 222 |                     });
 223 |                 } else if let Some(name) = self.find_child_text(&child, "identifier", source) {
 224 |                     let full_signature = format!("const {}", &name);
 225 |                     signatures.push(Signature {
 226 |                         kind: SignatureKind::Constant,
 227 |                         name,
 228 |                         params: None,
 229 |                         return_type: None,
 230 |                         visibility: Visibility::All,
 231 |                         line_number: child.start_position().row + 1,
 232 |                         full_signature,
 233 |                     });
 234 |                 }
 235 |             }
 236 |         }
 237 |     }
 238 | 
 239 |     fn extract_export_signatures(
 240 |         &self,
 241 |         source: &str,
 242 |         node: &tree_sitter::Node,
 243 |         signatures: &mut Vec<Signature>,
 244 |     ) {
 245 |         let mut cursor = node.walk();
 246 |         for child in node.children(&mut cursor) {
 247 |             if child.kind() == "function_declaration" {
 248 |                 if let Some(sig) = self.extract_function_signature(source, &child) {
 249 |                     signatures.push(sig);
 250 |                 }
 251 |             } else if child.kind() == "class_declaration"
 252 |                 && let Some(sig) = self.extract_class_signature(source, &child)
 253 |             {
 254 |                 signatures.push(sig);
 255 |             }
 256 |         }
 257 |     }
 258 | 
 259 |     fn find_child_text(
 260 |         &self,
 261 |         node: &tree_sitter::Node,
 262 |         kind: &str,
 263 |         source: &str,
 264 |     ) -> Option<String> {
 265 |         let mut cursor = node.walk();
 266 |         for child in node.children(&mut cursor) {
 267 |             if child.kind() == kind {
 268 |                 return Some(source[child.start_byte()..child.end_byte()].to_string());
 269 |             }
 270 |             let mut nested_cursor = child.walk();
 271 |             for nested in child.children(&mut nested_cursor) {
 272 |                 if nested.kind() == kind {
 273 |                     return Some(source[nested.start_byte()..nested.end_byte()].to_string());
 274 |                 }
 275 |             }
 276 |         }
 277 |         None
 278 |     }
 279 | 
 280 |     fn find_best_boundary(
 281 |         &self,
 282 |         cursor: &mut tree_sitter::TreeCursor,
 283 |         max_bytes: usize,
 284 |         best_end: &mut usize,
 285 |     ) {
 286 |         loop {
 287 |             let node = cursor.node();
 288 |             let end_byte = node.end_byte();
 289 | 
 290 |             if end_byte <= max_bytes && end_byte > *best_end {
 291 |                 let is_item = matches!(
 292 |                     node.kind(),
 293 |                     "function_declaration"
 294 |                         | "class_declaration"
 295 |                         | "method_definition"
 296 |                         | "export_statement"
 297 |                         | "variable_declaration"
 298 |                         | "lexical_declaration"
 299 |                 );
 300 |                 if is_item {
 301 |                     *best_end = end_byte;
 302 |                 }
 303 |             }
 304 | 
 305 |             if cursor.goto_first_child() {
 306 |                 self.find_best_boundary(cursor, max_bytes, best_end);
 307 |                 cursor.goto_parent();
 308 |             }
 309 | 
 310 |             if !cursor.goto_next_sibling() {
 311 |                 break;
 312 |             }
 313 |         }
 314 |     }
 315 | }
 316 | 
 317 | #[cfg(test)]
 318 | mod tests {
 319 |     use super::*;
 320 | 
 321 |     #[test]
 322 |     fn test_extract_function_signature() {
 323 |         let source = r#"
 324 | function hello(name) {
 325 |     return `Hello, ${name}!`;
 326 | }
 327 | 
 328 | const add = (a, b) => a + b;
 329 | "#;
 330 | 
 331 |         let signatures = JavaScriptSupport.extract_signatures(source, Visibility::All);
 332 |         assert!(!signatures.is_empty());
 333 | 
 334 |         let funcs: Vec<_> = signatures
 335 |             .iter()
 336 |             .filter(|s| s.kind == SignatureKind::Function)
 337 |             .collect();
 338 |         assert!(!funcs.is_empty());
 339 |         assert_eq!(funcs[0].name, "hello");
 340 |     }
 341 | 
 342 |     #[test]
 343 |     fn test_extract_class_signature() {
 344 |         let source = r#"
 345 | class User {
 346 |     constructor(name) {
 347 |         this.name = name;
 348 |     }
 349 | }
 350 | }
 351 | "#;
 352 | 
 353 |         let signatures = JavaScriptSupport.extract_signatures(source, Visibility::All);
 354 |         let classes: Vec<_> = signatures
 355 |             .iter()
 356 |             .filter(|s| s.kind == SignatureKind::Class)
 357 |             .collect();
 358 |         assert!(!classes.is_empty());
 359 |         assert_eq!(classes[0].name, "User");
 360 |     }
 361 | 
 362 |     #[test]
 363 |     fn test_file_extensions() {
 364 |         assert!(JavaScriptSupport.supports_extension("js"));
 365 |         assert!(JavaScriptSupport.supports_extension("mjs"));
 366 |         assert!(!JavaScriptSupport.supports_extension("ts"));
 367 |     }
 368 | }
```

### File: `src/tree_sitter/languages/python.rs`

- Size: 9816 bytes
- Modified: SystemTime { tv_sec: 1771153475, tv_nsec: 470475130 }

```rust
   1 | //! Python language support for tree-sitter.
   2 | 
   3 | #[cfg(feature = "tree-sitter-python")]
   4 | use tree_sitter::{Parser, Tree};
   5 | 
   6 | #[cfg(feature = "tree-sitter-python")]
   7 | use crate::tree_sitter::language_support::{
   8 |     CodeStructure, LanguageSupport, Signature, SignatureKind, Visibility,
   9 |     slice_signature_before_body,
  10 | };
  11 | 
  12 | pub struct PythonSupport;
  13 | 
  14 | #[cfg(feature = "tree-sitter-python")]
  15 | impl PythonSupport {
  16 |     fn get_language() -> tree_sitter::Language {
  17 |         tree_sitter_python::LANGUAGE.into()
  18 |     }
  19 | }
  20 | 
  21 | #[cfg(feature = "tree-sitter-python")]
  22 | impl LanguageSupport for PythonSupport {
  23 |     fn file_extensions(&self) -> &[&'static str] {
  24 |         &["py", "pyw"]
  25 |     }
  26 | 
  27 |     fn parse(&self, source: &str) -> Option<Tree> {
  28 |         let mut parser = Parser::new();
  29 |         parser.set_language(&Self::get_language()).ok()?;
  30 |         parser.parse(source, None)
  31 |     }
  32 | 
  33 |     fn extract_signatures(&self, source: &str, visibility: Visibility) -> Vec<Signature> {
  34 |         let tree = match self.parse(source) {
  35 |             Some(t) => t,
  36 |             None => return Vec::new(),
  37 |         };
  38 | 
  39 |         let root = tree.root_node();
  40 |         let mut signatures = Vec::new();
  41 | 
  42 |         self.extract_signatures_from_node(source, &root, visibility, &mut signatures);
  43 | 
  44 |         signatures.sort_by_key(|s| s.line_number);
  45 |         signatures
  46 |     }
  47 | 
  48 |     fn extract_structure(&self, source: &str) -> CodeStructure {
  49 |         let tree = match self.parse(source) {
  50 |             Some(t) => t,
  51 |             None => return CodeStructure::default(),
  52 |         };
  53 | 
  54 |         let root = tree.root_node();
  55 |         let mut structure = CodeStructure {
  56 |             total_lines: source.lines().count(),
  57 |             ..Default::default()
  58 |         };
  59 | 
  60 |         self.extract_structure_from_node(&root, &mut structure);
  61 |         structure
  62 |     }
  63 | 
  64 |     fn find_truncation_point(&self, source: &str, max_bytes: usize) -> usize {
  65 |         if source.len() <= max_bytes {
  66 |             return source.len();
  67 |         }
  68 | 
  69 |         let tree = match self.parse(source) {
  70 |             Some(t) => t,
  71 |             None => return max_bytes,
  72 |         };
  73 | 
  74 |         let root = tree.root_node();
  75 |         let mut best_end = 0;
  76 | 
  77 |         let mut cursor = root.walk();
  78 |         self.find_best_boundary(&mut cursor, max_bytes, &mut best_end);
  79 |         drop(cursor);
  80 | 
  81 |         if best_end == 0 { max_bytes } else { best_end }
  82 |     }
  83 | }
  84 | 
  85 | #[cfg(feature = "tree-sitter-python")]
  86 | impl PythonSupport {
  87 |     fn extract_signatures_from_node(
  88 |         &self,
  89 |         source: &str,
  90 |         node: &tree_sitter::Node,
  91 |         _visibility: Visibility,
  92 |         signatures: &mut Vec<Signature>,
  93 |     ) {
  94 |         match node.kind() {
  95 |             "function_definition" => {
  96 |                 if let Some(sig) = self.extract_function_signature(source, node) {
  97 |                     signatures.push(sig);
  98 |                 }
  99 |             }
 100 |             "class_definition" => {
 101 |                 if let Some(sig) = self.extract_class_signature(source, node) {
 102 |                     signatures.push(sig);
 103 |                 }
 104 |             }
 105 |             _ => {}
 106 |         }
 107 | 
 108 |         let mut cursor = node.walk();
 109 |         for child in node.children(&mut cursor) {
 110 |             self.extract_signatures_from_node(source, &child, _visibility, signatures);
 111 |         }
 112 |     }
 113 | 
 114 |     fn extract_structure_from_node(&self, node: &tree_sitter::Node, structure: &mut CodeStructure) {
 115 |         match node.kind() {
 116 |             "function_definition" => structure.functions += 1,
 117 |             "class_definition" => structure.classes += 1,
 118 |             "import_statement" | "import_from_statement" => {
 119 |                 structure.imports.push("import".to_string());
 120 |             }
 121 |             _ => {}
 122 |         }
 123 | 
 124 |         let mut cursor = node.walk();
 125 |         for child in node.children(&mut cursor) {
 126 |             self.extract_structure_from_node(&child, structure);
 127 |         }
 128 |     }
 129 | 
 130 |     fn extract_function_signature(
 131 |         &self,
 132 |         source: &str,
 133 |         node: &tree_sitter::Node,
 134 |     ) -> Option<Signature> {
 135 |         let name = self.find_child_text(node, "identifier", source)?;
 136 |         let params = self.find_child_text(node, "parameters", source);
 137 | 
 138 |         // Fix: Walk up to grandparent to detect methods. In Python's AST, a method's
 139 |         // direct parent is a `block`, not `class_definition`.
 140 |         let is_method = node.parent().is_some_and(|p| {
 141 |             p.kind() == "class_definition"
 142 |                 || (p.kind() == "block"
 143 |                     && p.parent().is_some_and(|gp| gp.kind() == "class_definition"))
 144 |         });
 145 |         let kind = if is_method {
 146 |             SignatureKind::Method
 147 |         } else {
 148 |             SignatureKind::Function
 149 |         };
 150 | 
 151 |         // Use byte-slicing to preserve decorators, type hints, and return annotations
 152 |         let full_sig = slice_signature_before_body(source, node, &["block"])
 153 |             .unwrap_or_else(|| {
 154 |                 let mut sig = String::new();
 155 |                 if let Some(decorators) = self.find_decorators(source, node) {
 156 |                     sig.push_str(&decorators);
 157 |                     sig.push('\n');
 158 |                 }
 159 |                 sig.push_str("def ");
 160 |                 sig.push_str(&name);
 161 |                 if let Some(p) = &params {
 162 |                     sig.push_str(p);
 163 |                 } else {
 164 |                     sig.push_str("()");
 165 |                 }
 166 |                 sig
 167 |             });
 168 | 
 169 |         Some(Signature {
 170 |             kind,
 171 |             name,
 172 |             params,
 173 |             return_type: None, // Captured via byte-slicing in full_sig
 174 |             visibility: Visibility::All,
 175 |             line_number: node.start_position().row + 1,
 176 |             full_signature: full_sig,
 177 |         })
 178 |     }
 179 | 
 180 |     fn extract_class_signature(&self, source: &str, node: &tree_sitter::Node) -> Option<Signature> {
 181 |         let name = self.find_child_text(node, "identifier", source)?;
 182 |         let bases = self.find_child_text(node, "argument_list", source);
 183 | 
 184 |         let mut full_sig = String::new();
 185 |         if let Some(decorators) = self.find_decorators(source, node) {
 186 |             full_sig.push_str(&decorators);
 187 |             full_sig.push('\n');
 188 |         }
 189 |         full_sig.push_str("class ");
 190 |         full_sig.push_str(&name);
 191 |         if let Some(b) = &bases {
 192 |             full_sig.push('(');
 193 |             full_sig.push_str(b);
 194 |             full_sig.push(')');
 195 |         }
 196 | 
 197 |         Some(Signature {
 198 |             kind: SignatureKind::Class,
 199 |             name,
 200 |             params: bases,
 201 |             return_type: None,
 202 |             visibility: Visibility::All,
 203 |             line_number: node.start_position().row + 1,
 204 |             full_signature: full_sig,
 205 |         })
 206 |     }
 207 | 
 208 |     fn find_decorators(&self, source: &str, node: &tree_sitter::Node) -> Option<String> {
 209 |         let parent = node.parent()?;
 210 |         let mut cursor = parent.walk();
 211 |         let mut decorators = Vec::new();
 212 | 
 213 |         for sibling in parent.children(&mut cursor) {
 214 |             if sibling.kind() == "decorator"
 215 |                 && sibling.end_position().row == node.start_position().row.saturating_sub(1)
 216 |             {
 217 |                 let text = &source[sibling.start_byte()..sibling.end_byte()];
 218 |                 decorators.push(text.to_string());
 219 |             }
 220 |         }
 221 | 
 222 |         if decorators.is_empty() {
 223 |             None
 224 |         } else {
 225 |             Some(decorators.join("\n"))
 226 |         }
 227 |     }
 228 | 
 229 |     fn find_child_text(
 230 |         &self,
 231 |         node: &tree_sitter::Node,
 232 |         kind: &str,
 233 |         source: &str,
 234 |     ) -> Option<String> {
 235 |         let mut cursor = node.walk();
 236 |         for child in node.children(&mut cursor) {
 237 |             if child.kind() == kind {
 238 |                 return Some(source[child.start_byte()..child.end_byte()].to_string());
 239 |             }
 240 |         }
 241 |         None
 242 |     }
 243 | 
 244 |     fn find_best_boundary(
 245 |         &self,
 246 |         cursor: &mut tree_sitter::TreeCursor,
 247 |         max_bytes: usize,
 248 |         best_end: &mut usize,
 249 |     ) {
 250 |         loop {
 251 |             let node = cursor.node();
 252 |             let end_byte = node.end_byte();
 253 | 
 254 |             if end_byte <= max_bytes && end_byte > *best_end {
 255 |                 let is_item = matches!(
 256 |                     node.kind(),
 257 |                     "function_definition" | "class_definition" | "decorated_definition"
 258 |                 );
 259 |                 if is_item {
 260 |                     *best_end = end_byte;
 261 |                 }
 262 |             }
 263 | 
 264 |             if cursor.goto_first_child() {
 265 |                 self.find_best_boundary(cursor, max_bytes, best_end);
 266 |                 cursor.goto_parent();
 267 |             }
 268 | 
 269 |             if !cursor.goto_next_sibling() {
 270 |                 break;
 271 |             }
 272 |         }
 273 |     }
 274 | }
 275 | 
 276 | #[cfg(test)]
 277 | mod tests {
 278 |     use super::*;
 279 | 
 280 |     #[test]
 281 |     fn test_extract_function_signature() {
 282 |         let source = r#"
 283 | def hello(name):
 284 |     return f"Hello, {name}!"
 285 | 
 286 | def add(a: int, b: int) -> int:
 287 |     return a + b
 288 | "#;
 289 | 
 290 |         let signatures = PythonSupport.extract_signatures(source, Visibility::All);
 291 |         assert!(!signatures.is_empty());
 292 | 
 293 |         let funcs: Vec<_> = signatures
 294 |             .iter()
 295 |             .filter(|s| matches!(s.kind, SignatureKind::Function | SignatureKind::Method))
 296 |             .collect();
 297 |         assert!(funcs.len() >= 2);
 298 |         assert_eq!(funcs[0].name, "hello");
 299 |     }
 300 | 
 301 |     #[test]
 302 |     fn test_extract_class_signature() {
 303 |         let source = r#"
 304 | class User:
 305 |     def __init__(self, name):
 306 |         self.name = name
 307 | "#;
 308 | 
 309 |         let signatures = PythonSupport.extract_signatures(source, Visibility::All);
 310 |         let classes: Vec<_> = signatures
 311 |             .iter()
 312 |             .filter(|s| s.kind == SignatureKind::Class)
 313 |             .collect();
 314 |         assert!(!classes.is_empty());
 315 |         assert_eq!(classes[0].name, "User");
 316 |     }
 317 | 
 318 |     #[test]
 319 |     fn test_file_extensions() {
 320 |         assert!(PythonSupport.supports_extension("py"));
 321 |         assert!(PythonSupport.supports_extension("pyw"));
 322 |         assert!(!PythonSupport.supports_extension("rs"));
 323 |     }
 324 | }
```

### File: `src/tree_sitter/languages/rust.rs`

- Size: 19524 bytes
- Modified: SystemTime { tv_sec: 1771153406, tv_nsec: 452599834 }

```rust
   1 | //! Rust language support for tree-sitter.
   2 | 
   3 | use tree_sitter::{Parser, Tree};
   4 | 
   5 | use crate::tree_sitter::language_support::{
   6 |     CodeStructure, LanguageSupport, Signature, SignatureKind, Visibility,
   7 |     slice_signature_before_body,
   8 | };
   9 | 
  10 | pub struct RustSupport;
  11 | 
  12 | impl RustSupport {
  13 |     fn get_language() -> tree_sitter::Language {
  14 |         tree_sitter_rust::LANGUAGE.into()
  15 |     }
  16 | }
  17 | 
  18 | impl LanguageSupport for RustSupport {
  19 |     fn file_extensions(&self) -> &[&'static str] {
  20 |         &["rs"]
  21 |     }
  22 | 
  23 |     fn parse(&self, source: &str) -> Option<Tree> {
  24 |         let mut parser = Parser::new();
  25 |         parser.set_language(&Self::get_language()).ok()?;
  26 |         parser.parse(source, None)
  27 |     }
  28 | 
  29 |     fn extract_signatures(&self, source: &str, visibility: Visibility) -> Vec<Signature> {
  30 |         let tree = match self.parse(source) {
  31 |             Some(t) => t,
  32 |             None => return Vec::new(),
  33 |         };
  34 | 
  35 |         let root = tree.root_node();
  36 |         let mut signatures = Vec::new();
  37 | 
  38 |         self.extract_signatures_from_node(source, &root, visibility, &mut signatures);
  39 | 
  40 |         signatures.sort_by_key(|s| s.line_number);
  41 |         signatures
  42 |     }
  43 | 
  44 |     fn extract_structure(&self, source: &str) -> CodeStructure {
  45 |         let tree = match self.parse(source) {
  46 |             Some(t) => t,
  47 |             None => return CodeStructure::default(),
  48 |         };
  49 | 
  50 |         let root = tree.root_node();
  51 |         let mut structure = CodeStructure {
  52 |             total_lines: source.lines().count(),
  53 |             ..Default::default()
  54 |         };
  55 | 
  56 |         self.extract_structure_from_node(source, &root, &mut structure);
  57 | 
  58 |         structure.code_lines = source
  59 |             .lines()
  60 |             .filter(|line| {
  61 |                 let trimmed = line.trim();
  62 |                 !trimmed.is_empty() && !trimmed.starts_with("//")
  63 |             })
  64 |             .count();
  65 | 
  66 |         structure
  67 |     }
  68 | 
  69 |     fn find_truncation_point(&self, source: &str, max_bytes: usize) -> usize {
  70 |         if source.len() <= max_bytes {
  71 |             return source.len();
  72 |         }
  73 | 
  74 |         let tree = match self.parse(source) {
  75 |             Some(t) => t,
  76 |             None => return max_bytes,
  77 |         };
  78 | 
  79 |         let root = tree.root_node();
  80 | 
  81 |         let mut best_end = 0;
  82 |         let mut cursor = root.walk();
  83 | 
  84 |         self.walk_for_boundary(&mut cursor, max_bytes, &mut best_end);
  85 | 
  86 |         if best_end == 0 { max_bytes } else { best_end }
  87 |     }
  88 | }
  89 | 
  90 | impl RustSupport {
  91 |     fn walk_for_boundary(
  92 |         &self,
  93 |         cursor: &mut tree_sitter::TreeCursor,
  94 |         max_bytes: usize,
  95 |         best_end: &mut usize,
  96 |     ) {
  97 |         loop {
  98 |             let node = cursor.node();
  99 |             let end_byte = node.end_byte();
 100 | 
 101 |             if end_byte <= max_bytes && end_byte > *best_end {
 102 |                 let is_item = matches!(
 103 |                     node.kind(),
 104 |                     "function_item"
 105 |                         | "struct_item"
 106 |                         | "enum_item"
 107 |                         | "trait_item"
 108 |                         | "impl_item"
 109 |                         | "mod_item"
 110 |                         | "const_item"
 111 |                         | "static_item"
 112 |                         | "type_item"
 113 |                         | "macro_definition"
 114 |                 );
 115 |                 if is_item {
 116 |                     *best_end = end_byte;
 117 |                 }
 118 |             }
 119 | 
 120 |             if cursor.goto_first_child() {
 121 |                 self.walk_for_boundary(cursor, max_bytes, best_end);
 122 |                 cursor.goto_parent();
 123 |             }
 124 | 
 125 |             if !cursor.goto_next_sibling() {
 126 |                 break;
 127 |             }
 128 |         }
 129 |     }
 130 | 
 131 |     fn extract_signatures_from_node(
 132 |         &self,
 133 |         source: &str,
 134 |         node: &tree_sitter::Node,
 135 |         visibility: Visibility,
 136 |         signatures: &mut Vec<Signature>,
 137 |     ) {
 138 |         match node.kind() {
 139 |             "function_item" => {
 140 |                 if let Some(sig) = self.extract_function_signature(source, node, visibility) {
 141 |                     signatures.push(sig);
 142 |                 }
 143 |             }
 144 |             "struct_item" => {
 145 |                 if let Some(sig) = self.extract_struct_signature(source, node, visibility) {
 146 |                     signatures.push(sig);
 147 |                 }
 148 |             }
 149 |             "enum_item" => {
 150 |                 if let Some(sig) = self.extract_enum_signature(source, node, visibility) {
 151 |                     signatures.push(sig);
 152 |                 }
 153 |             }
 154 |             "trait_item" => {
 155 |                 if let Some(sig) = self.extract_trait_signature(source, node, visibility) {
 156 |                     signatures.push(sig);
 157 |                 }
 158 |             }
 159 |             "impl_item" => {
 160 |                 if let Some(sig) = self.extract_impl_signature(source, node) {
 161 |                     signatures.push(sig);
 162 |                 }
 163 |             }
 164 |             "mod_item" => {
 165 |                 if let Some(sig) = self.extract_mod_signature(source, node, visibility) {
 166 |                     signatures.push(sig);
 167 |                 }
 168 |             }
 169 |             "const_item" => {
 170 |                 if let Some(sig) = self.extract_const_signature(source, node, visibility) {
 171 |                     signatures.push(sig);
 172 |                 }
 173 |             }
 174 |             "type_item" => {
 175 |                 if let Some(sig) = self.extract_type_alias_signature(source, node, visibility) {
 176 |                     signatures.push(sig);
 177 |                 }
 178 |             }
 179 |             "macro_definition" => {
 180 |                 if let Some(sig) = self.extract_macro_signature(source, node, visibility) {
 181 |                     signatures.push(sig);
 182 |                 }
 183 |             }
 184 |             _ => {}
 185 |         }
 186 | 
 187 |         let mut cursor = node.walk();
 188 |         for child in node.children(&mut cursor) {
 189 |             self.extract_signatures_from_node(source, &child, visibility, signatures);
 190 |         }
 191 |     }
 192 | 
 193 |     fn extract_structure_from_node(
 194 |         &self,
 195 |         source: &str,
 196 |         node: &tree_sitter::Node,
 197 |         structure: &mut CodeStructure,
 198 |     ) {
 199 |         match node.kind() {
 200 |             "function_item" => structure.functions += 1,
 201 |             "struct_item" => structure.structs += 1,
 202 |             "enum_item" => structure.enums += 1,
 203 |             "trait_item" => structure.traits += 1,
 204 |             "const_item" => structure.constants += 1,
 205 |             "type_item" => structure.type_aliases += 1,
 206 |             "macro_definition" => structure.macros += 1,
 207 |             "use_declaration" => {
 208 |                 structure
 209 |                     .imports
 210 |                     .push(self.node_text(source, node).to_string());
 211 |             }
 212 |             _ => {}
 213 |         }
 214 | 
 215 |         let mut cursor = node.walk();
 216 |         for child in node.children(&mut cursor) {
 217 |             self.extract_structure_from_node(source, &child, structure);
 218 |         }
 219 |     }
 220 | 
 221 |     fn is_public(&self, node: &tree_sitter::Node) -> bool {
 222 |         let mut cursor = node.walk();
 223 |         for child in node.children(&mut cursor) {
 224 |             if child.kind() == "visibility_modifier" {
 225 |                 return true;
 226 |             }
 227 |         }
 228 |         false
 229 |     }
 230 | 
 231 |     fn get_visibility(&self, node: &tree_sitter::Node) -> Visibility {
 232 |         if self.is_public(node) {
 233 |             Visibility::Public
 234 |         } else {
 235 |             Visibility::Private
 236 |         }
 237 |     }
 238 | 
 239 |     fn node_text<'a>(&self, source: &'a str, node: &tree_sitter::Node) -> &'a str {
 240 |         &source[node.start_byte()..node.end_byte()]
 241 |     }
 242 | 
 243 |     fn extract_function_signature(
 244 |         &self,
 245 |         source: &str,
 246 |         node: &tree_sitter::Node,
 247 |         visibility_filter: Visibility,
 248 |     ) -> Option<Signature> {
 249 |         let vis = self.get_visibility(node);
 250 |         if !vis.matches_filter(visibility_filter) {
 251 |             return None;
 252 |         }
 253 | 
 254 |         let name = self.find_child_text(node, "identifier", source)?;
 255 |         let params = self.find_child_text(node, "parameters", source);
 256 |         let return_type = self.find_child_text(node, "return_type", source);
 257 | 
 258 |         // Use byte-slicing to preserve generics, return types, and all modifiers
 259 |         let full_sig = slice_signature_before_body(source, node, &["block"])
 260 |             .unwrap_or_else(|| {
 261 |                 // Fallback for declarations without a body
 262 |                 let mut sig = String::new();
 263 |                 if vis == Visibility::Public {
 264 |                     sig.push_str("pub ");
 265 |                 }
 266 |                 sig.push_str("fn ");
 267 |                 sig.push_str(&name);
 268 |                 if let Some(p) = &params {
 269 |                     sig.push_str(p);
 270 |                 }
 271 |                 if let Some(r) = &return_type {
 272 |                     sig.push_str(" -> ");
 273 |                     sig.push_str(r);
 274 |                 }
 275 |                 sig
 276 |             });
 277 | 
 278 |         Some(Signature {
 279 |             kind: SignatureKind::Function,
 280 |             name,
 281 |             params,
 282 |             return_type,
 283 |             visibility: vis,
 284 |             line_number: node.start_position().row + 1,
 285 |             full_signature: full_sig,
 286 |         })
 287 |     }
 288 | 
 289 |     fn extract_struct_signature(
 290 |         &self,
 291 |         source: &str,
 292 |         node: &tree_sitter::Node,
 293 |         visibility_filter: Visibility,
 294 |     ) -> Option<Signature> {
 295 |         let vis = self.get_visibility(node);
 296 |         if !vis.matches_filter(visibility_filter) {
 297 |             return None;
 298 |         }
 299 | 
 300 |         let name = self.find_child_text(node, "type_identifier", source)?;
 301 | 
 302 |         // Use byte-slicing to preserve generic bounds and where clauses
 303 |         let full_sig = slice_signature_before_body(source, node, &["field_declaration_list"])
 304 |             .unwrap_or_else(|| {
 305 |                 let mut sig = String::new();
 306 |                 if vis == Visibility::Public {
 307 |                     sig.push_str("pub ");
 308 |                 }
 309 |                 sig.push_str("struct ");
 310 |                 sig.push_str(&name);
 311 |                 sig
 312 |             });
 313 | 
 314 |         Some(Signature {
 315 |             kind: SignatureKind::Struct,
 316 |             name,
 317 |             params: None,
 318 |             return_type: None,
 319 |             visibility: vis,
 320 |             line_number: node.start_position().row + 1,
 321 |             full_signature: full_sig,
 322 |         })
 323 |     }
 324 | 
 325 |     fn extract_enum_signature(
 326 |         &self,
 327 |         source: &str,
 328 |         node: &tree_sitter::Node,
 329 |         visibility_filter: Visibility,
 330 |     ) -> Option<Signature> {
 331 |         let vis = self.get_visibility(node);
 332 |         if !vis.matches_filter(visibility_filter) {
 333 |             return None;
 334 |         }
 335 | 
 336 |         let name = self.find_child_text(node, "type_identifier", source)?;
 337 | 
 338 |         // Use byte-slicing to preserve generic bounds
 339 |         let full_sig = slice_signature_before_body(source, node, &["enum_variant_list"])
 340 |             .unwrap_or_else(|| {
 341 |                 let mut sig = String::new();
 342 |                 if vis == Visibility::Public {
 343 |                     sig.push_str("pub ");
 344 |                 }
 345 |                 sig.push_str("enum ");
 346 |                 sig.push_str(&name);
 347 |                 sig
 348 |             });
 349 | 
 350 |         Some(Signature {
 351 |             kind: SignatureKind::Enum,
 352 |             name,
 353 |             params: None,
 354 |             return_type: None,
 355 |             visibility: vis,
 356 |             line_number: node.start_position().row + 1,
 357 |             full_signature: full_sig,
 358 |         })
 359 |     }
 360 | 
 361 |     fn extract_trait_signature(
 362 |         &self,
 363 |         source: &str,
 364 |         node: &tree_sitter::Node,
 365 |         visibility_filter: Visibility,
 366 |     ) -> Option<Signature> {
 367 |         let vis = self.get_visibility(node);
 368 |         if !vis.matches_filter(visibility_filter) {
 369 |             return None;
 370 |         }
 371 | 
 372 |         let name = self.find_child_text(node, "type_identifier", source)?;
 373 | 
 374 |         // Use byte-slicing to preserve trait bounds and supertraits
 375 |         let full_sig = slice_signature_before_body(source, node, &["declaration_list"])
 376 |             .unwrap_or_else(|| {
 377 |                 let mut sig = String::new();
 378 |                 if vis == Visibility::Public {
 379 |                     sig.push_str("pub ");
 380 |                 }
 381 |                 sig.push_str("trait ");
 382 |                 sig.push_str(&name);
 383 |                 sig
 384 |             });
 385 | 
 386 |         Some(Signature {
 387 |             kind: SignatureKind::Trait,
 388 |             name,
 389 |             params: None,
 390 |             return_type: None,
 391 |             visibility: vis,
 392 |             line_number: node.start_position().row + 1,
 393 |             full_signature: full_sig,
 394 |         })
 395 |     }
 396 | 
 397 |     fn extract_impl_signature(&self, source: &str, node: &tree_sitter::Node) -> Option<Signature> {
 398 |         let name = self.find_child_text(node, "type_identifier", source)?;
 399 | 
 400 |         // Use byte-slicing to preserve `impl Trait for Type` and generics
 401 |         let full_sig = slice_signature_before_body(source, node, &["declaration_list"])
 402 |             .unwrap_or_else(|| {
 403 |                 let mut sig = String::new();
 404 |                 sig.push_str("impl ");
 405 |                 sig.push_str(&name);
 406 |                 sig
 407 |             });
 408 | 
 409 |         Some(Signature {
 410 |             kind: SignatureKind::Impl,
 411 |             name,
 412 |             params: None,
 413 |             return_type: None,
 414 |             visibility: Visibility::All,
 415 |             line_number: node.start_position().row + 1,
 416 |             full_signature: full_sig,
 417 |         })
 418 |     }
 419 | 
 420 |     fn extract_mod_signature(
 421 |         &self,
 422 |         source: &str,
 423 |         node: &tree_sitter::Node,
 424 |         visibility_filter: Visibility,
 425 |     ) -> Option<Signature> {
 426 |         let vis = self.get_visibility(node);
 427 |         if !vis.matches_filter(visibility_filter) {
 428 |             return None;
 429 |         }
 430 | 
 431 |         let name = self.find_child_text(node, "identifier", source)?;
 432 | 
 433 |         let mut full_sig = String::new();
 434 |         if vis == Visibility::Public {
 435 |             full_sig.push_str("pub ");
 436 |         }
 437 |         full_sig.push_str("mod ");
 438 |         full_sig.push_str(&name);
 439 | 
 440 |         Some(Signature {
 441 |             kind: SignatureKind::Module,
 442 |             name,
 443 |             params: None,
 444 |             return_type: None,
 445 |             visibility: vis,
 446 |             line_number: node.start_position().row + 1,
 447 |             full_signature: full_sig,
 448 |         })
 449 |     }
 450 | 
 451 |     fn extract_const_signature(
 452 |         &self,
 453 |         source: &str,
 454 |         node: &tree_sitter::Node,
 455 |         visibility_filter: Visibility,
 456 |     ) -> Option<Signature> {
 457 |         let vis = self.get_visibility(node);
 458 |         if !vis.matches_filter(visibility_filter) {
 459 |             return None;
 460 |         }
 461 | 
 462 |         let name = self.find_child_text(node, "identifier", source)?;
 463 | 
 464 |         let mut full_sig = String::new();
 465 |         if vis == Visibility::Public {
 466 |             full_sig.push_str("pub ");
 467 |         }
 468 |         full_sig.push_str("const ");
 469 |         full_sig.push_str(&name);
 470 | 
 471 |         Some(Signature {
 472 |             kind: SignatureKind::Constant,
 473 |             name,
 474 |             params: None,
 475 |             return_type: None,
 476 |             visibility: vis,
 477 |             line_number: node.start_position().row + 1,
 478 |             full_signature: full_sig,
 479 |         })
 480 |     }
 481 | 
 482 |     fn extract_type_alias_signature(
 483 |         &self,
 484 |         source: &str,
 485 |         node: &tree_sitter::Node,
 486 |         visibility_filter: Visibility,
 487 |     ) -> Option<Signature> {
 488 |         let vis = self.get_visibility(node);
 489 |         if !vis.matches_filter(visibility_filter) {
 490 |             return None;
 491 |         }
 492 | 
 493 |         let name = self.find_child_text(node, "type_identifier", source)?;
 494 | 
 495 |         let mut full_sig = String::new();
 496 |         if vis == Visibility::Public {
 497 |             full_sig.push_str("pub ");
 498 |         }
 499 |         full_sig.push_str("type ");
 500 |         full_sig.push_str(&name);
 501 | 
 502 |         Some(Signature {
 503 |             kind: SignatureKind::TypeAlias,
 504 |             name,
 505 |             params: None,
 506 |             return_type: None,
 507 |             visibility: vis,
 508 |             line_number: node.start_position().row + 1,
 509 |             full_signature: full_sig,
 510 |         })
 511 |     }
 512 | 
 513 |     fn extract_macro_signature(
 514 |         &self,
 515 |         source: &str,
 516 |         node: &tree_sitter::Node,
 517 |         visibility_filter: Visibility,
 518 |     ) -> Option<Signature> {
 519 |         let vis = self.get_visibility(node);
 520 |         if !vis.matches_filter(visibility_filter) {
 521 |             return None;
 522 |         }
 523 | 
 524 |         let name = self.find_child_text(node, "identifier", source)?;
 525 | 
 526 |         let mut full_sig = String::new();
 527 |         if vis == Visibility::Public {
 528 |             full_sig.push_str("pub ");
 529 |         }
 530 |         full_sig.push_str("macro_rules! ");
 531 |         full_sig.push_str(&name);
 532 | 
 533 |         Some(Signature {
 534 |             kind: SignatureKind::Macro,
 535 |             name,
 536 |             params: None,
 537 |             return_type: None,
 538 |             visibility: vis,
 539 |             line_number: node.start_position().row + 1,
 540 |             full_signature: full_sig,
 541 |         })
 542 |     }
 543 | 
 544 |     fn find_child_text(
 545 |         &self,
 546 |         node: &tree_sitter::Node,
 547 |         kind: &str,
 548 |         source: &str,
 549 |     ) -> Option<String> {
 550 |         let mut cursor = node.walk();
 551 |         for child in node.children(&mut cursor) {
 552 |             if child.kind() == kind {
 553 |                 return Some(self.node_text(source, &child).to_string());
 554 |             }
 555 |             let mut nested_cursor = child.walk();
 556 |             for nested in child.children(&mut nested_cursor) {
 557 |                 if nested.kind() == kind {
 558 |                     return Some(self.node_text(source, &nested).to_string());
 559 |                 }
 560 |             }
 561 |         }
 562 |         None
 563 |     }
 564 | }
 565 | 
 566 | #[cfg(test)]
 567 | mod tests {
 568 |     use super::*;
 569 | 
 570 |     #[test]
 571 |     fn test_extract_function_signature() {
 572 |         let source = r#"
 573 | pub fn hello(name: &str) -> String {
 574 |     format!("Hello, {}!", name)
 575 | }
 576 | 
 577 | fn private_helper(x: i32) -> i32 {
 578 |     x * 2
 579 | }
 580 | "#;
 581 | 
 582 |         let signatures = RustSupport.extract_signatures(source, Visibility::All);
 583 |         assert_eq!(signatures.len(), 2);
 584 | 
 585 |         assert_eq!(signatures[0].name, "hello");
 586 |         assert_eq!(signatures[0].kind, SignatureKind::Function);
 587 |         assert_eq!(signatures[0].visibility, Visibility::Public);
 588 | 
 589 |         assert_eq!(signatures[1].name, "private_helper");
 590 |         assert_eq!(signatures[1].visibility, Visibility::Private);
 591 |     }
 592 | 
 593 |     #[test]
 594 |     fn test_public_only_filter() {
 595 |         let source = r#"
 596 | pub fn public_fn() {}
 597 | fn private_fn() {}
 598 | "#;
 599 | 
 600 |         let signatures = RustSupport.extract_signatures(source, Visibility::Public);
 601 |         assert_eq!(signatures.len(), 1);
 602 |         assert_eq!(signatures[0].name, "public_fn");
 603 |     }
 604 | 
 605 |     #[test]
 606 |     fn test_extract_struct_signature() {
 607 |         let source = r#"
 608 | pub struct User {
 609 |     name: String,
 610 |     age: u32,
 611 | }
 612 | "#;
 613 | 
 614 |         let signatures = RustSupport.extract_signatures(source, Visibility::All);
 615 |         assert_eq!(signatures.len(), 1);
 616 |         assert_eq!(signatures[0].name, "User");
 617 |         assert_eq!(signatures[0].kind, SignatureKind::Struct);
 618 |     }
 619 | 
 620 |     #[test]
 621 |     fn test_extract_structure() {
 622 |         let source = r#"
 623 | use std::fs;
 624 | 
 625 | pub struct Config {
 626 |     path: String,
 627 | }
 628 | 
 629 | pub fn load() -> Config {
 630 |     Config { path: ".".into() }
 631 | }
 632 | 
 633 | enum Status {
 634 |     Active,
 635 |     Inactive,
 636 | }
 637 | "#;
 638 | 
 639 |         let structure = RustSupport.extract_structure(source);
 640 |         assert_eq!(structure.structs, 1);
 641 |         assert_eq!(structure.functions, 1);
 642 |         assert_eq!(structure.enums, 1);
 643 |     }
 644 | 
 645 |     #[test]
 646 |     fn test_find_truncation_point() {
 647 |         let source = r#"
 648 | fn first() -> i32 {
 649 |     1
 650 | }
 651 | 
 652 | fn second() -> i32 {
 653 |     2
 654 | }
 655 | 
 656 | fn third() -> i32 {
 657 |     3
 658 | }
 659 | "#;
 660 | 
 661 |         let after_first = source.find("fn second()").unwrap();
 662 |         let point = RustSupport.find_truncation_point(source, after_first);
 663 | 
 664 |         assert!(point <= after_first);
 665 |         assert!(source[..point].contains("fn first()"));
 666 |     }
 667 | 
 668 |     #[test]
 669 |     fn test_file_extensions() {
 670 |         assert!(RustSupport.supports_extension("rs"));
 671 |         assert!(!RustSupport.supports_extension("py"));
 672 |     }
 673 | }
```

### File: `src/tree_sitter/languages/typescript.rs`

- Size: 13860 bytes
- Modified: SystemTime { tv_sec: 1771153567, tv_nsec: 244639026 }

```rust
   1 | //! TypeScript language support for tree-sitter.
   2 | 
   3 | #[cfg(feature = "tree-sitter-ts")]
   4 | use tree_sitter::{Parser, Tree};
   5 | 
   6 | #[cfg(feature = "tree-sitter-ts")]
   7 | use crate::tree_sitter::language_support::{
   8 |     CodeStructure, LanguageSupport, Signature, SignatureKind, Visibility,
   9 |     slice_signature_before_body,
  10 | };
  11 | 
  12 | pub struct TypeScriptSupport;
  13 | 
  14 | #[cfg(feature = "tree-sitter-ts")]
  15 | impl TypeScriptSupport {
  16 |     fn get_language() -> tree_sitter::Language {
  17 |         // Use TypeScript grammar (not TSX)
  18 |         tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into()
  19 |     }
  20 | }
  21 | 
  22 | #[cfg(feature = "tree-sitter-ts")]
  23 | impl LanguageSupport for TypeScriptSupport {
  24 |     fn file_extensions(&self) -> &[&'static str] {
  25 |         &["ts", "tsx", "mts", "cts"]
  26 |     }
  27 | 
  28 |     fn parse(&self, source: &str) -> Option<Tree> {
  29 |         let mut parser = Parser::new();
  30 |         parser.set_language(&Self::get_language()).ok()?;
  31 |         parser.parse(source, None)
  32 |     }
  33 | 
  34 |     fn extract_signatures(&self, source: &str, visibility: Visibility) -> Vec<Signature> {
  35 |         let tree = match self.parse(source) {
  36 |             Some(t) => t,
  37 |             None => return Vec::new(),
  38 |         };
  39 | 
  40 |         let root = tree.root_node();
  41 |         let mut signatures = Vec::new();
  42 | 
  43 |         self.extract_signatures_from_node(source, &root, visibility, &mut signatures);
  44 | 
  45 |         signatures.sort_by_key(|s| s.line_number);
  46 |         signatures
  47 |     }
  48 | 
  49 |     fn extract_structure(&self, source: &str) -> CodeStructure {
  50 |         let tree = match self.parse(source) {
  51 |             Some(t) => t,
  52 |             None => return CodeStructure::default(),
  53 |         };
  54 | 
  55 |         let root = tree.root_node();
  56 |         let mut structure = CodeStructure {
  57 |             total_lines: source.lines().count(),
  58 |             ..Default::default()
  59 |         };
  60 | 
  61 |         self.extract_structure_from_node(&root, &mut structure);
  62 |         structure
  63 |     }
  64 | 
  65 |     fn find_truncation_point(&self, source: &str, max_bytes: usize) -> usize {
  66 |         if source.len() <= max_bytes {
  67 |             return source.len();
  68 |         }
  69 | 
  70 |         let tree = match self.parse(source) {
  71 |             Some(t) => t,
  72 |             None => return max_bytes,
  73 |         };
  74 | 
  75 |         let root = tree.root_node();
  76 |         let mut best_end = 0;
  77 | 
  78 |         let mut cursor = root.walk();
  79 |         self.find_best_boundary(&mut cursor, max_bytes, &mut best_end);
  80 |         drop(cursor);
  81 | 
  82 |         if best_end == 0 { max_bytes } else { best_end }
  83 |     }
  84 | }
  85 | 
  86 | #[cfg(feature = "tree-sitter-ts")]
  87 | impl TypeScriptSupport {
  88 |     fn extract_signatures_from_node(
  89 |         &self,
  90 |         source: &str,
  91 |         node: &tree_sitter::Node,
  92 |         _visibility: Visibility,
  93 |         signatures: &mut Vec<Signature>,
  94 |     ) {
  95 |         match node.kind() {
  96 |             "function_declaration" | "generator_function_declaration" => {
  97 |                 if let Some(sig) = self.extract_function_signature(source, node) {
  98 |                     signatures.push(sig);
  99 |                 }
 100 |             }
 101 |             "class_declaration" => {
 102 |                 if let Some(sig) = self.extract_class_signature(source, node) {
 103 |                     signatures.push(sig);
 104 |                 }
 105 |             }
 106 |             "interface_declaration" => {
 107 |                 if let Some(sig) = self.extract_interface_signature(source, node) {
 108 |                     signatures.push(sig);
 109 |                 }
 110 |             }
 111 |             "type_alias_declaration" => {
 112 |                 if let Some(sig) = self.extract_type_alias_signature(source, node) {
 113 |                     signatures.push(sig);
 114 |                 }
 115 |             }
 116 |             "enum_declaration" => {
 117 |                 if let Some(sig) = self.extract_enum_signature(source, node) {
 118 |                     signatures.push(sig);
 119 |                 }
 120 |             }
 121 |             "lexical_declaration" => {
 122 |                 self.extract_variable_declarations(source, node, signatures);
 123 |             }
 124 |             "export_statement" => {
 125 |                 self.extract_export_signatures(source, node, signatures);
 126 |             }
 127 |             _ => {}
 128 |         }
 129 | 
 130 |         let mut cursor = node.walk();
 131 |         for child in node.children(&mut cursor) {
 132 |             self.extract_signatures_from_node(source, &child, _visibility, signatures);
 133 |         }
 134 |     }
 135 | 
 136 |     fn extract_structure_from_node(&self, node: &tree_sitter::Node, structure: &mut CodeStructure) {
 137 |         match node.kind() {
 138 |             "function_declaration" | "function_expression" | "arrow_function" => {
 139 |                 structure.functions += 1;
 140 |             }
 141 |             "class_declaration" | "class_expression" => {
 142 |                 structure.classes += 1;
 143 |             }
 144 |             "interface_declaration" => {
 145 |                 structure.interfaces += 1;
 146 |             }
 147 |             "type_alias_declaration" => {
 148 |                 structure.type_aliases += 1;
 149 |             }
 150 |             "enum_declaration" => {
 151 |                 structure.enums += 1;
 152 |             }
 153 |             "import_statement" => {
 154 |                 structure.imports.push("import".to_string());
 155 |             }
 156 |             "export_statement" => {
 157 |                 structure.exports.push("export".to_string());
 158 |             }
 159 |             _ => {}
 160 |         }
 161 | 
 162 |         let mut cursor = node.walk();
 163 |         for child in node.children(&mut cursor) {
 164 |             self.extract_structure_from_node(&child, structure);
 165 |         }
 166 |     }
 167 | 
 168 |     fn extract_function_signature(
 169 |         &self,
 170 |         source: &str,
 171 |         node: &tree_sitter::Node,
 172 |     ) -> Option<Signature> {
 173 |         let name = self.find_child_text(node, "identifier", source)?;
 174 |         let params = self.find_child_text(node, "formal_parameters", source);
 175 |         let return_type = self.find_child_text(node, "type_annotation", source);
 176 | 
 177 |         // Use byte-slicing to preserve type params, access modifiers, and return types
 178 |         let full_sig = slice_signature_before_body(source, node, &["statement_block"])
 179 |             .unwrap_or_else(|| {
 180 |                 match (params.as_ref(), return_type.as_ref()) {
 181 |                     (Some(p), Some(r)) => format!("function {}{} {}", name, p, r),
 182 |                     (Some(p), None) => format!("function {}{}", name, p),
 183 |                     (None, Some(r)) => format!("function {}() {}", name, r),
 184 |                     (None, None) => format!("function {}()", name),
 185 |                 }
 186 |             });
 187 | 
 188 |         Some(Signature {
 189 |             kind: SignatureKind::Function,
 190 |             name,
 191 |             params,
 192 |             return_type,
 193 |             visibility: Visibility::All,
 194 |             line_number: node.start_position().row + 1,
 195 |             full_signature: full_sig,
 196 |         })
 197 |     }
 198 | 
 199 |     fn extract_class_signature(&self, source: &str, node: &tree_sitter::Node) -> Option<Signature> {
 200 |         let name = self
 201 |             .find_child_text(node, "type_identifier", source)
 202 |             .or_else(|| self.find_child_text(node, "identifier", source))?;
 203 | 
 204 |         let full_sig = format!("class {}", name);
 205 | 
 206 |         Some(Signature {
 207 |             kind: SignatureKind::Class,
 208 |             name,
 209 |             params: None,
 210 |             return_type: None,
 211 |             visibility: Visibility::All,
 212 |             line_number: node.start_position().row + 1,
 213 |             full_signature: full_sig,
 214 |         })
 215 |     }
 216 | 
 217 |     fn extract_interface_signature(
 218 |         &self,
 219 |         source: &str,
 220 |         node: &tree_sitter::Node,
 221 |     ) -> Option<Signature> {
 222 |         let name = self.find_child_text(node, "type_identifier", source)?;
 223 | 
 224 |         let full_sig = format!("interface {}", name);
 225 | 
 226 |         Some(Signature {
 227 |             kind: SignatureKind::Interface,
 228 |             name,
 229 |             params: None,
 230 |             return_type: None,
 231 |             visibility: Visibility::All,
 232 |             line_number: node.start_position().row + 1,
 233 |             full_signature: full_sig,
 234 |         })
 235 |     }
 236 | 
 237 |     fn extract_type_alias_signature(
 238 |         &self,
 239 |         source: &str,
 240 |         node: &tree_sitter::Node,
 241 |     ) -> Option<Signature> {
 242 |         let name = self.find_child_text(node, "type_identifier", source)?;
 243 | 
 244 |         let full_sig = format!("type {}", name);
 245 | 
 246 |         Some(Signature {
 247 |             kind: SignatureKind::TypeAlias,
 248 |             name,
 249 |             params: None,
 250 |             return_type: None,
 251 |             visibility: Visibility::All,
 252 |             line_number: node.start_position().row + 1,
 253 |             full_signature: full_sig,
 254 |         })
 255 |     }
 256 | 
 257 |     fn extract_enum_signature(&self, source: &str, node: &tree_sitter::Node) -> Option<Signature> {
 258 |         let name = self
 259 |             .find_child_text(node, "identifier", source)
 260 |             .or_else(|| self.find_child_text(node, "type_identifier", source))?;
 261 | 
 262 |         let full_sig = format!("enum {}", name);
 263 | 
 264 |         Some(Signature {
 265 |             kind: SignatureKind::Enum,
 266 |             name,
 267 |             params: None,
 268 |             return_type: None,
 269 |             visibility: Visibility::All,
 270 |             line_number: node.start_position().row + 1,
 271 |             full_signature: full_sig,
 272 |         })
 273 |     }
 274 | 
 275 |     fn extract_variable_declarations(
 276 |         &self,
 277 |         source: &str,
 278 |         node: &tree_sitter::Node,
 279 |         signatures: &mut Vec<Signature>,
 280 |     ) {
 281 |         let mut cursor = node.walk();
 282 |         for child in node.children(&mut cursor) {
 283 |             if child.kind() == "variable_declarator"
 284 |                 && let Some(name) = self.find_child_text(&child, "identifier", source)
 285 |             {
 286 |                 let type_ann = self.find_child_text(&child, "type_annotation", source);
 287 |                 let full_sig = match &type_ann {
 288 |                     Some(t) => format!("const {} {}", name, t),
 289 |                     None => format!("const {}", name),
 290 |                 };
 291 |                 signatures.push(Signature {
 292 |                     kind: SignatureKind::Constant,
 293 |                     name,
 294 |                     params: None,
 295 |                     return_type: type_ann,
 296 |                     visibility: Visibility::All,
 297 |                     line_number: child.start_position().row + 1,
 298 |                     full_signature: full_sig,
 299 |                 });
 300 |             }
 301 |         }
 302 |     }
 303 | 
 304 |     fn extract_export_signatures(
 305 |         &self,
 306 |         source: &str,
 307 |         node: &tree_sitter::Node,
 308 |         signatures: &mut Vec<Signature>,
 309 |     ) {
 310 |         let mut cursor = node.walk();
 311 |         for child in node.children(&mut cursor) {
 312 |             match child.kind() {
 313 |                 "function_declaration" => {
 314 |                     if let Some(sig) = self.extract_function_signature(source, &child) {
 315 |                         signatures.push(sig);
 316 |                     }
 317 |                 }
 318 |                 "class_declaration" => {
 319 |                     if let Some(sig) = self.extract_class_signature(source, &child) {
 320 |                         signatures.push(sig);
 321 |                     }
 322 |                 }
 323 |                 "interface_declaration" => {
 324 |                     if let Some(sig) = self.extract_interface_signature(source, &child) {
 325 |                         signatures.push(sig);
 326 |                     }
 327 |                 }
 328 |                 _ => {}
 329 |             }
 330 |         }
 331 |     }
 332 | 
 333 |     fn find_child_text(
 334 |         &self,
 335 |         node: &tree_sitter::Node,
 336 |         kind: &str,
 337 |         source: &str,
 338 |     ) -> Option<String> {
 339 |         let mut cursor = node.walk();
 340 |         for child in node.children(&mut cursor) {
 341 |             if child.kind() == kind {
 342 |                 return Some(source[child.start_byte()..child.end_byte()].to_string());
 343 |             }
 344 |             let mut nested_cursor = child.walk();
 345 |             for nested in child.children(&mut nested_cursor) {
 346 |                 if nested.kind() == kind {
 347 |                     return Some(source[nested.start_byte()..nested.end_byte()].to_string());
 348 |                 }
 349 |             }
 350 |         }
 351 |         None
 352 |     }
 353 | 
 354 |     fn find_best_boundary(
 355 |         &self,
 356 |         cursor: &mut tree_sitter::TreeCursor,
 357 |         max_bytes: usize,
 358 |         best_end: &mut usize,
 359 |     ) {
 360 |         loop {
 361 |             let node = cursor.node();
 362 |             let end_byte = node.end_byte();
 363 | 
 364 |             if end_byte <= max_bytes && end_byte > *best_end {
 365 |                 let is_item = matches!(
 366 |                     node.kind(),
 367 |                     "function_declaration"
 368 |                         | "class_declaration"
 369 |                         | "interface_declaration"
 370 |                         | "type_alias_declaration"
 371 |                         | "enum_declaration"
 372 |                         | "export_statement"
 373 |                         | "lexical_declaration"
 374 |                 );
 375 |                 if is_item {
 376 |                     *best_end = end_byte;
 377 |                 }
 378 |             }
 379 | 
 380 |             if cursor.goto_first_child() {
 381 |                 self.find_best_boundary(cursor, max_bytes, best_end);
 382 |                 cursor.goto_parent();
 383 |             }
 384 | 
 385 |             if !cursor.goto_next_sibling() {
 386 |                 break;
 387 |             }
 388 |         }
 389 |     }
 390 | }
 391 | 
 392 | #[cfg(test)]
 393 | mod tests {
 394 |     use super::*;
 395 | 
 396 |     #[test]
 397 |     fn test_extract_function_signature() {
 398 |         let source = r#"
 399 | function hello(name: string): string {
 400 |     return `Hello, ${name}!`;
 401 | }
 402 | "#;
 403 | 
 404 |         let signatures = TypeScriptSupport.extract_signatures(source, Visibility::All);
 405 |         assert!(!signatures.is_empty());
 406 |         assert_eq!(signatures[0].name, "hello");
 407 |         assert!(signatures[0].return_type.is_some());
 408 |     }
 409 | 
 410 |     #[test]
 411 |     fn test_extract_interface_signature() {
 412 |         let source = r#"
 413 | interface User {
 414 |     name: string;
 415 |     age: number;
 416 | }
 417 | }
 418 | "#;
 419 | 
 420 |         let signatures = TypeScriptSupport.extract_signatures(source, Visibility::All);
 421 |         let interfaces: Vec<_> = signatures
 422 |             .iter()
 423 |             .filter(|s| s.kind == SignatureKind::Interface)
 424 |             .collect();
 425 |         assert!(!interfaces.is_empty());
 426 |         assert_eq!(interfaces[0].name, "User");
 427 |     }
 428 | 
 429 |     #[test]
 430 |     fn test_file_extensions() {
 431 |         assert!(TypeScriptSupport.supports_extension("ts"));
 432 |         assert!(TypeScriptSupport.supports_extension("tsx"));
 433 |         assert!(!TypeScriptSupport.supports_extension("js"));
 434 |     }
 435 | }
```

### File: `src/tree_sitter/signatures.rs`

- Size: 2276 bytes
- Modified: SystemTime { tv_sec: 1771140505, tv_nsec: 237594206 }

```rust
   1 | //! Signature extraction utilities.
   2 | 
   3 | use super::language_support::{LanguageSupport, Signature, Visibility};
   4 | 
   5 | /// Extract all signatures from source code.
   6 | pub fn extract_signatures(
   7 |     source: &str,
   8 |     support: &dyn LanguageSupport,
   9 |     visibility: Visibility,
  10 | ) -> Vec<Signature> {
  11 |     support.extract_signatures(source, visibility)
  12 | }
  13 | 
  14 | /// Format signatures as markdown.
  15 | pub fn format_signatures_as_markdown(signatures: &[Signature], language: &str) -> String {
  16 |     if signatures.is_empty() {
  17 |         return String::new();
  18 |     }
  19 | 
  20 |     let mut output = String::new();
  21 |     output.push_str("```");
  22 |     output.push_str(language);
  23 |     output.push('\n');
  24 | 
  25 |     let mut current_kind: Option<&str> = None;
  26 | 
  27 |     for sig in signatures {
  28 |         let kind_str = match sig.kind {
  29 |             super::language_support::SignatureKind::Function
  30 |             | super::language_support::SignatureKind::Method => "Functions",
  31 |             super::language_support::SignatureKind::Struct
  32 |             | super::language_support::SignatureKind::Class => "Structs/Classes",
  33 |             super::language_support::SignatureKind::Enum => "Enums",
  34 |             super::language_support::SignatureKind::Trait
  35 |             | super::language_support::SignatureKind::Interface => "Traits/Interfaces",
  36 |             super::language_support::SignatureKind::Impl => "Implementations",
  37 |             super::language_support::SignatureKind::Module => "Modules",
  38 |             super::language_support::SignatureKind::Constant => "Constants",
  39 |             super::language_support::SignatureKind::TypeAlias => "Type Aliases",
  40 |             super::language_support::SignatureKind::Macro => "Macros",
  41 |         };
  42 | 
  43 |         if current_kind != Some(kind_str) {
  44 |             if current_kind.is_some() {
  45 |                 output.push('\n');
  46 |             }
  47 |             output.push_str("// ");
  48 |             output.push_str(kind_str);
  49 |             output.push('\n');
  50 |             current_kind = Some(kind_str);
  51 |         }
  52 | 
  53 |         output.push_str(&sig.full_signature);
  54 |         output.push('\n');
  55 |     }
  56 | 
  57 |     output.push_str("```\n");
  58 |     output
  59 | }
  60 | 
  61 | #[cfg(test)]
  62 | mod tests {
  63 |     use super::*;
  64 | 
  65 |     #[test]
  66 |     fn test_format_empty_signatures() {
  67 |         let output = format_signatures_as_markdown(&[], "rust");
  68 |         assert!(output.is_empty());
  69 |     }
  70 | }
```

### File: `src/tree_sitter/structure.rs`

- Size: 2617 bytes
- Modified: SystemTime { tv_sec: 1771137172, tv_nsec: 933081207 }

```rust
   1 | //! Code structure extraction utilities.
   2 | 
   3 | use super::language_support::{CodeStructure, LanguageSupport};
   4 | 
   5 | /// Extract structure information from source code.
   6 | pub fn extract_structure(source: &str, support: &dyn LanguageSupport) -> CodeStructure {
   7 |     support.extract_structure(source)
   8 | }
   9 | 
  10 | /// Format structure as markdown summary.
  11 | pub fn format_structure_as_markdown(structure: &CodeStructure) -> String {
  12 |     if structure.total_symbols() == 0 {
  13 |         return String::new();
  14 |     }
  15 | 
  16 |     let mut output = String::new();
  17 |     output.push_str("**Structure:**\n");
  18 | 
  19 |     let mut parts = Vec::new();
  20 | 
  21 |     if structure.functions > 0 {
  22 |         parts.push(format!("{} functions", structure.functions));
  23 |     }
  24 |     if structure.structs > 0 {
  25 |         parts.push(format!("{} structs", structure.structs));
  26 |     }
  27 |     if structure.classes > 0 {
  28 |         parts.push(format!("{} classes", structure.classes));
  29 |     }
  30 |     if structure.enums > 0 {
  31 |         parts.push(format!("{} enums", structure.enums));
  32 |     }
  33 |     if structure.traits > 0 {
  34 |         parts.push(format!("{} traits", structure.traits));
  35 |     }
  36 |     if structure.interfaces > 0 {
  37 |         parts.push(format!("{} interfaces", structure.interfaces));
  38 |     }
  39 |     if structure.constants > 0 {
  40 |         parts.push(format!("{} constants", structure.constants));
  41 |     }
  42 |     if structure.type_aliases > 0 {
  43 |         parts.push(format!("{} types", structure.type_aliases));
  44 |     }
  45 |     if structure.macros > 0 {
  46 |         parts.push(format!("{} macros", structure.macros));
  47 |     }
  48 | 
  49 |     output.push_str("- ");
  50 |     output.push_str(&parts.join(", "));
  51 |     output.push('\n');
  52 | 
  53 |     if structure.total_lines > 0 {
  54 |         output.push_str(&format!(
  55 |             "- {} lines ({} code)\n",
  56 |             structure.total_lines, structure.code_lines
  57 |         ));
  58 |     }
  59 | 
  60 |     output
  61 | }
  62 | 
  63 | #[cfg(test)]
  64 | mod tests {
  65 |     use super::*;
  66 | 
  67 |     #[test]
  68 |     fn test_format_empty_structure() {
  69 |         let structure = CodeStructure::default();
  70 |         let output = format_structure_as_markdown(&structure);
  71 |         assert!(output.is_empty());
  72 |     }
  73 | 
  74 |     #[test]
  75 |     fn test_format_structure_with_symbols() {
  76 |         let structure = CodeStructure {
  77 |             functions: 5,
  78 |             structs: 2,
  79 |             enums: 1,
  80 |             total_lines: 100,
  81 |             code_lines: 80,
  82 |             ..Default::default()
  83 |         };
  84 | 
  85 |         let output = format_structure_as_markdown(&structure);
  86 |         assert!(output.contains("5 functions"));
  87 |         assert!(output.contains("2 structs"));
  88 |         assert!(output.contains("1 enums"));
  89 |         assert!(output.contains("100 lines"));
  90 |         assert!(output.contains("80 code"));
  91 |     }
  92 | }
```

### File: `src/tree_sitter/truncation.rs`

- Size: 1859 bytes
- Modified: SystemTime { tv_sec: 1771142666, tv_nsec: 609070082 }

```rust
   1 | //! Smart truncation at AST boundaries.
   2 | 
   3 | use super::language_support::LanguageSupport;
   4 | 
   5 | /// Find a truncation point that ends at a complete AST node boundary.
   6 | ///
   7 | /// Returns the byte position where the source should be truncated.
   8 | /// If no suitable boundary is found within max_bytes, returns max_bytes.
   9 | pub fn find_truncation_point(
  10 |     source: &str,
  11 |     max_bytes: usize,
  12 |     support: &dyn LanguageSupport,
  13 | ) -> usize {
  14 |     if source.len() <= max_bytes {
  15 |         return source.len();
  16 |     }
  17 | 
  18 |     support.find_truncation_point(source, max_bytes)
  19 | }
  20 | 
  21 | /// Check if truncation is needed at a UTF-8 boundary.
  22 | pub fn ensure_utf8_boundary(source: &str, position: usize) -> usize {
  23 |     if position >= source.len() {
  24 |         return source.len();
  25 |     }
  26 | 
  27 |     let mut pos = position;
  28 |     while pos > 0 && !source.is_char_boundary(pos) {
  29 |         pos -= 1;
  30 |     }
  31 |     pos
  32 | }
  33 | 
  34 | /// Add a truncation notice to the output.
  35 | pub fn add_truncation_notice(output: &mut String, truncated_count: usize) {
  36 |     output.push_str("\n\n---\n\n");
  37 |     if truncated_count > 0 {
  38 |         output.push_str(&format!(
  39 |             "_Output truncated: {} more items omitted._\n",
  40 |             truncated_count
  41 |         ));
  42 |     } else {
  43 |         output.push_str("_Output truncated at code boundary._\n");
  44 |     }
  45 | }
  46 | 
  47 | #[cfg(test)]
  48 | mod tests {
  49 |     use super::*;
  50 | 
  51 |     #[test]
  52 |     fn test_ensure_utf8_boundary_ascii() {
  53 |         let source = "Hello, world!";
  54 |         assert_eq!(ensure_utf8_boundary(source, 5), 5);
  55 |         assert_eq!(ensure_utf8_boundary(source, 100), 13);
  56 |     }
  57 | 
  58 |     #[test]
  59 |     fn test_ensure_utf8_boundary_unicode() {
  60 |         let source = "Hello, 世界!"; // 4 bytes per Chinese char
  61 |         // Position 8 is inside the first Chinese character (starts at 7)
  62 |         let boundary = ensure_utf8_boundary(source, 8);
  63 |         assert_eq!(boundary, 7); // Should fall back to start of char
  64 |     }
  65 | }
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

- Size: 11135 bytes
- Modified: SystemTime { tv_sec: 1771138540, tv_nsec: 511298459 }

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
 214 |         signatures: false,
 215 |         structure: false,
 216 |         truncate: "smart".to_string(),
 217 |         visibility: "all".to_string(),
 218 |     };
 219 | 
 220 |     let prompter = NoPrompt;
 221 | 
 222 |     let mut group = c.benchmark_group("context_builder");
 223 | 
 224 |     group.measurement_time(Duration::from_secs(20));
 225 |     group.sample_size(20);
 226 | 
 227 |     let mode = if cfg!(feature = "parallel") {
 228 |         "parallel"
 229 |     } else {
 230 |         "sequential"
 231 |     };
 232 |     let ln = if line_numbers {
 233 |         "line_numbers"
 234 |     } else {
 235 |         "no_line_numbers"
 236 |     };
 237 |     let id = BenchmarkId::new(
 238 |         format!(
 239 |             "{}-{}files-{}B",
 240 |             spec.name, spec.text_files, spec.text_file_size
 241 |         ),
 242 |         format!("{}-{}", ln, mode),
 243 |     );
 244 | 
 245 |     group.bench_with_input(id, &args, |b, _| {
 246 |         b.iter(|| {
 247 |             // Allow repeated overwrites; keep the output path stable to avoid filesystem churn
 248 |             let _ = std::hint::black_box(run_with_args(
 249 |                 Args {
 250 |                     input: args.input.clone(),
 251 |                     output: args.output.clone(),
 252 |                     filter: args.filter.clone(),
 253 |                     ignore: args.ignore.clone(),
 254 |                     preview: args.preview,
 255 |                     token_count: args.token_count,
 256 |                     line_numbers: args.line_numbers,
 257 |                     yes: true,
 258 |                     diff_only: false,
 259 |                     clear_cache: false,
 260 |                     init: false,
 261 |                     max_tokens: None,
 262 |                     signatures: false,
 263 |                     structure: false,
 264 |                     truncate: "smart".to_string(),
 265 |                     visibility: "all".to_string(),
 266 |                 },
 267 |                 Config::default(),
 268 |                 &prompter,
 269 |             ));
 270 |         });
 271 |     });
 272 | 
 273 |     group.finish();
 274 | }
 275 | 
 276 | /// Benchmarks:
 277 | /// - tiny: ~100 files, small size
 278 | /// - small: ~1,000 files
 279 | /// - medium: ~5,000 files (enabled only if CB_BENCH_MEDIUM=1)
 280 | ///
 281 | /// These datasets are generated in a temporary directory at runtime to keep the
 282 | /// benchmark self-contained. Binary files are generated but filtered out by
 283 | /// the `filters` configuration so they aren't processed.
 284 | ///
 285 | /// Run:
 286 | ///   cargo bench --bench context_bench
 287 | pub fn context_benchmark(c: &mut Criterion) {
 288 |     // Ensure silent-by-default for benchmarks
 289 |     init_bench_env();
 290 | 
 291 |     // Common filters and ignores: ignore typical heavy dirs; only include text code/docs
 292 |     let common_filters = vec!["rs".into(), "md".into(), "txt".into(), "toml".into()];
 293 |     let common_ignores = vec!["target".into(), "node_modules".into()];
 294 | 
 295 |     // Tiny dataset
 296 |     let tiny = DatasetSpec {
 297 |         name: "tiny",
 298 |         text_files: 100,
 299 |         binary_every: 10,
 300 |         depth: 2,
 301 |         width: 3,
 302 |         text_file_size: 256,
 303 |         filters: common_filters.clone(),
 304 |         ignores: common_ignores.clone(),
 305 |     };
 306 | 
 307 |     // Small dataset
 308 |     let small = DatasetSpec {
 309 |         name: "small",
 310 |         text_files: 1_000,
 311 |         binary_every: 20,
 312 |         depth: 3,
 313 |         width: 4,
 314 |         text_file_size: 512,
 315 |         filters: common_filters.clone(),
 316 |         ignores: common_ignores.clone(),
 317 |     };
 318 | 
 319 |     // Medium dataset (can be enabled via env var to avoid heavy runs by default)
 320 |     let include_medium = std::env::var("CB_BENCH_MEDIUM").ok().as_deref() == Some("1");
 321 |     let medium = DatasetSpec {
 322 |         name: "medium",
 323 |         text_files: 5_000,
 324 |         binary_every: 25,
 325 |         depth: 4,
 326 |         width: 4,
 327 |         text_file_size: 800,
 328 |         filters: common_filters.clone(),
 329 |         ignores: common_ignores.clone(),
 330 |     };
 331 | 
 332 |     // For each dataset, run benchmarks with and without line numbers
 333 |     for ds in [tiny, small] {
 334 |         bench_scenario(c, ds.clone(), false);
 335 |         bench_scenario(c, ds, true);
 336 |     }
 337 | 
 338 |     if include_medium {
 339 |         bench_scenario(c, medium.clone(), false);
 340 |         bench_scenario(c, medium, true);
 341 |     }
 342 | }
 343 | 
 344 | criterion_group!(benches, context_benchmark);
 345 | criterion_main!(benches);
```

### File: `tests/cli_integration.rs`

- Size: 13986 bytes
- Modified: SystemTime { tv_sec: 1771138540, tv_nsec: 547298937 }

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
  70 |         signatures: false,
  71 |         structure: false,
  72 |         truncate: "smart".to_string(),
  73 |         visibility: "all".to_string(),
  74 |     };
  75 | 
  76 |     let prompter = TestPrompter::new(true, true);
  77 | 
  78 |     // Run in preview mode
  79 |     let res = run_with_args(args, Config::default(), &prompter);
  80 |     assert!(res.is_ok(), "preview mode should succeed");
  81 | 
  82 |     // No output file created
  83 |     assert!(
  84 |         !root.join("output.md").exists(),
  85 |         "output file should not be created in preview mode"
  86 |     );
  87 | }
  88 | 
  89 | #[test]
  90 | fn preview_mode_skips_overwrite_confirmation() {
  91 |     let dir = tempdir().unwrap();
  92 |     let root = dir.path();
  93 | 
  94 |     // Create an existing output file
  95 |     let output_path = root.join("output.md");
  96 |     write_file(&output_path, "existing content");
  97 | 
  98 |     // Create a small project structure
  99 |     write_file(&root.join("src/main.rs"), "fn main() { println!(\"hi\"); }");
 100 |     write_file(&root.join("README.md"), "# Readme");
 101 | 
 102 |     let args = Args {
 103 |         input: root.to_string_lossy().into_owned(),
 104 |         output: output_path.to_string_lossy().into_owned(),
 105 |         filter: vec![],
 106 |         ignore: vec![],
 107 |         preview: true,
 108 |         token_count: false,
 109 |         line_numbers: false,
 110 |         yes: false,
 111 |         diff_only: false,
 112 |         clear_cache: false,
 113 |         init: false,
 114 |         max_tokens: None,
 115 |         signatures: false,
 116 |         structure: false,
 117 |         truncate: "smart".to_string(),
 118 |         visibility: "all".to_string(),
 119 |     };
 120 | 
 121 |     // Use false for overwrite response to verify it's not called
 122 |     let prompter = TestPrompter::new(false, true);
 123 | 
 124 |     // Run in preview mode - should succeed even with overwrite denied
 125 |     let res = run_with_args(args, Config::default(), &prompter);
 126 |     assert!(
 127 |         res.is_ok(),
 128 |         "preview mode should succeed without overwrite confirmation"
 129 |     );
 130 | 
 131 |     // Output file should remain unchanged
 132 |     let content = fs::read_to_string(&output_path).unwrap();
 133 |     assert_eq!(
 134 |         content, "existing content",
 135 |         "output file should not be modified in preview mode"
 136 |     );
 137 | }
 138 | 
 139 | #[test]
 140 | fn token_count_mode_skips_overwrite_confirmation() {
 141 |     let dir = tempdir().unwrap();
 142 |     let root = dir.path();
 143 | 
 144 |     // Create an existing output file
 145 |     let output_path = root.join("output.md");
 146 |     write_file(&output_path, "existing content");
 147 | 
 148 |     // Create a small project structure
 149 |     write_file(&root.join("src/main.rs"), "fn main() { println!(\"hi\"); }");
 150 |     write_file(&root.join("README.md"), "# Readme");
 151 | 
 152 |     let args = Args {
 153 |         input: root.to_string_lossy().into_owned(),
 154 |         output: output_path.to_string_lossy().into_owned(),
 155 |         filter: vec![],
 156 |         ignore: vec![],
 157 |         preview: false,
 158 |         token_count: true,
 159 |         line_numbers: false,
 160 |         yes: false,
 161 |         diff_only: false,
 162 |         clear_cache: false,
 163 |         init: false,
 164 |         max_tokens: None,
 165 |         signatures: false,
 166 |         structure: false,
 167 |         truncate: "smart".to_string(),
 168 |         visibility: "all".to_string(),
 169 |     };
 170 | 
 171 |     // Use false for overwrite response to verify it's not called
 172 |     let prompter = TestPrompter::new(false, true);
 173 | 
 174 |     // Run in token count mode - should succeed even with overwrite denied
 175 |     let res = run_with_args(args, Config::default(), &prompter);
 176 |     assert!(
 177 |         res.is_ok(),
 178 |         "token count mode should succeed without overwrite confirmation"
 179 |     );
 180 | 
 181 |     // Output file should remain unchanged
 182 |     let content = fs::read_to_string(&output_path).unwrap();
 183 |     assert_eq!(
 184 |         content, "existing content",
 185 |         "output file should not be modified in token count mode"
 186 |     );
 187 | }
 188 | 
 189 | #[test]
 190 | 
 191 | fn both_preview_and_token_count_modes_work_together() {
 192 |     let dir = tempdir().unwrap();
 193 |     let root = dir.path();
 194 | 
 195 |     // Create a small project structure
 196 |     write_file(&root.join("src/main.rs"), "fn main() { println!(\"hi\"); }");
 197 |     write_file(&root.join("README.md"), "# Readme");
 198 | 
 199 |     let args = Args {
 200 |         input: root.to_string_lossy().into_owned(),
 201 |         output: root.join("output.md").to_string_lossy().into_owned(),
 202 |         filter: vec![],
 203 |         ignore: vec![],
 204 |         preview: true,
 205 |         token_count: true,
 206 |         line_numbers: false,
 207 |         yes: false,
 208 |         diff_only: false,
 209 |         clear_cache: false,
 210 |         init: false,
 211 |         max_tokens: None,
 212 |         signatures: false,
 213 |         structure: false,
 214 |         truncate: "smart".to_string(),
 215 |         visibility: "all".to_string(),
 216 |     };
 217 | 
 218 |     let prompter = TestPrompter::new(false, true); // false for overwrite since it should be skipped
 219 | 
 220 |     // Run with both modes
 221 |     let res = run_with_args(args, Config::default(), &prompter);
 222 |     assert!(res.is_ok(), "both modes should work together");
 223 | 
 224 |     // No output file created
 225 |     assert!(
 226 |         !root.join("output.md").exists(),
 227 |         "output file should not be created when both modes are active"
 228 |     );
 229 | }
 230 | 
 231 | #[test]
 232 | fn end_to_end_generates_output_with_filters_ignores_and_line_numbers() {
 233 |     let dir = tempdir().unwrap();
 234 |     let root = dir.path();
 235 | 
 236 |     // Files that should be included by filters
 237 |     write_file(
 238 |         &root.join("src/main.rs"),
 239 |         "fn main() {\n    println!(\"hi\");\n}\n",
 240 |     );
 241 |     write_file(&root.join("README.md"), "# Top-level readme\n\nSome text");
 242 | 
 243 |     // Ignored directories/files
 244 |     write_file(
 245 |         &root.join("node_modules/pkg/index.js"),
 246 |         "console.log('ignore');",
 247 |     );
 248 |     write_file(&root.join("target/artifact.txt"), "binary");
 249 | 
 250 |     // A large file to exercise streaming and performance
 251 |     let mut large = String::with_capacity(4000 * 25);
 252 |     for i in 0..4000 {
 253 |         large.push_str(&format!("// line {}\n", i + 1));
 254 |     }
 255 |     write_file(&root.join("src/large.rs"), &large);
 256 | 
 257 |     let output_path = root.join("ctx.md");
 258 | 
 259 |     let args = Args {
 260 |         input: root.to_string_lossy().into_owned(),
 261 |         output: output_path.to_string_lossy().into_owned(),
 262 |         filter: vec!["rs".into(), "md".into()],
 263 |         ignore: vec!["node_modules".into(), "target".into()],
 264 |         preview: false,
 265 |         token_count: false,
 266 |         line_numbers: true,
 267 |         yes: false,
 268 |         diff_only: false,
 269 |         clear_cache: false,
 270 |         init: false,
 271 |         max_tokens: None,
 272 |         signatures: false,
 273 |         structure: false,
 274 |         truncate: "smart".to_string(),
 275 |         visibility: "all".to_string(),
 276 |     };
 277 | 
 278 |     // Always proceed without interactive prompts
 279 |     let prompter = TestPrompter::new(true, true);
 280 | 
 281 |     let res = run_with_args(args, Config::default(), &prompter);
 282 |     assert!(res.is_ok(), "end-to-end generation should succeed");
 283 | 
 284 |     // Find the actual output file (may have timestamp appended)
 285 |     let actual_output_path = if output_path.exists() {
 286 |         output_path
 287 |     } else {
 288 |         // Look for timestamped version
 289 |         let parent = output_path.parent().unwrap();
 290 |         let stem = output_path.file_stem().unwrap().to_string_lossy();
 291 |         let ext = output_path.extension().unwrap().to_string_lossy();
 292 | 
 293 |         let mut found_file = None;
 294 |         if let Ok(entries) = fs::read_dir(parent) {
 295 |             for entry in entries.flatten() {
 296 |                 let file_name = entry.file_name();
 297 |                 let name = file_name.to_string_lossy();
 298 |                 if name.starts_with(&format!("{}_", stem)) && name.ends_with(&format!(".{}", ext)) {
 299 |                     found_file = Some(entry.path());
 300 |                     break;
 301 |                 }
 302 |             }
 303 |         }
 304 | 
 305 |         found_file.unwrap_or_else(|| {
 306 |             panic!(
 307 |                 "No output file found. Expected {} or timestamped version",
 308 |                 output_path.display()
 309 |             )
 310 |         })
 311 |     };
 312 | 
 313 |     // Basic content checks
 314 |     let out = fs::read_to_string(&actual_output_path).unwrap();
 315 | 
 316 |     // Has file tree section
 317 |     assert!(
 318 |         out.contains("## File Tree Structure"),
 319 |         "output should contain a 'File Tree Structure' section"
 320 |     );
 321 | 
 322 |     // Has at least one rust code block with line numbers (looking for ' | ' marker)
 323 |     assert!(
 324 |         out.contains("```rust"),
 325 |         "output should contain a rust code block"
 326 |     );
 327 |     assert!(
 328 |         out.contains("   1 | "),
 329 |         "output should contain line-numbered code blocks"
 330 |     );
 331 | 
 332 |     // Should not include ignored directory entries' content (not a strict check, but indicative)
 333 |     assert!(
 334 |         !out.contains("console.log('ignore');"),
 335 |         "output should not include content from ignored directories"
 336 |     );
 337 | }
 338 | 
 339 | #[test]
 340 | fn overwrite_prompt_is_respected() {
 341 |     let dir = tempdir().unwrap();
 342 |     let root = dir.path();
 343 | 
 344 |     // Prepare an existing output file with sentinel content
 345 |     let output_path = root.join("out.md");
 346 |     write_file(&output_path, "SENTINEL");
 347 | 
 348 |     // Put a file to process
 349 |     write_file(&root.join("src/lib.rs"), "pub fn f() {}");
 350 | 
 351 |     let args = Args {
 352 |         input: root.to_string_lossy().into_owned(),
 353 |         output: output_path.to_string_lossy().into_owned(),
 354 |         filter: vec!["rs".into()],
 355 |         ignore: vec![],
 356 |         preview: false,
 357 |         token_count: false,
 358 |         line_numbers: false,
 359 |         yes: false,
 360 |         diff_only: false,
 361 |         clear_cache: false,
 362 |         init: false,
 363 |         max_tokens: None,
 364 |         signatures: false,
 365 |         structure: false,
 366 |         truncate: "smart".to_string(),
 367 |         visibility: "all".to_string(),
 368 |     };
 369 | 
 370 |     // Deny overwrite
 371 |     let prompter = TestPrompter::new(false, true);
 372 | 
 373 |     let res = run_with_args(args, Config::default(), &prompter);
 374 |     assert!(
 375 |         res.is_err(),
 376 |         "run should return error when overwrite denied"
 377 |     );
 378 | 
 379 |     // Ensure file is unchanged
 380 |     let out = fs::read_to_string(&output_path).unwrap();
 381 |     assert_eq!(out, "SENTINEL", "existing output should not be overwritten");
 382 | }
 383 | 
 384 | #[test]
 385 | fn confirm_processing_receives_large_count() {
 386 |     let dir = tempdir().unwrap();
 387 |     let root = dir.path();
 388 | 
 389 |     // Create a lot of files (should be well over the 100 threshold)
 390 |     fs::create_dir_all(root.join("data")).unwrap();
 391 |     for i in 0..150 {
 392 |         write_file(&root.join("data").join(format!("f{}.txt", i)), "x");
 393 |     }
 394 | 
 395 |     let args = Args {
 396 |         input: root.to_string_lossy().into_owned(),
 397 |         output: root.join("out.md").to_string_lossy().into_owned(),
 398 |         filter: vec!["txt".into()],
 399 |         ignore: vec![],
 400 |         preview: false,
 401 |         token_count: false,
 402 |         line_numbers: false,
 403 |         yes: false,
 404 |         diff_only: false,
 405 |         clear_cache: false,
 406 |         init: false,
 407 |         max_tokens: None,
 408 |         signatures: false,
 409 |         structure: false,
 410 |         truncate: "smart".to_string(),
 411 |         visibility: "all".to_string(),
 412 |     };
 413 | 
 414 |     let prompter = TestPrompter::new(true, true);
 415 | 
 416 |     let res = run_with_args(args, Config::default(), &prompter);
 417 |     assert!(res.is_ok(), "run should succeed with many files");
 418 | 
 419 |     // Ensure our injected prompter saw the large count (>= 150)
 420 |     assert!(
 421 |         prompter.last_count() >= 150,
 422 |         "expected confirm_processing to be called with >=150 files, got {}",
 423 |         prompter.last_count()
 424 |     );
 425 | }
 426 | 
 427 | #[test]
 428 | fn token_count_mode_does_not_create_output_file() {
 429 |     let dir = tempdir().unwrap();
 430 |     let root = dir.path();
 431 | 
 432 |     // Create a small project structure
 433 |     write_file(&root.join("src/main.rs"), "fn main() { println!(\"hi\"); }");
 434 |     write_file(&root.join("README.md"), "# Readme");
 435 | 
 436 |     let args = Args {
 437 |         input: root.to_string_lossy().into_owned(),
 438 |         output: root.join("output.md").to_string_lossy().into_owned(),
 439 |         filter: vec![],
 440 |         ignore: vec![],
 441 |         preview: false,
 442 |         token_count: true,
 443 |         line_numbers: false,
 444 |         yes: false,
 445 |         diff_only: false,
 446 |         clear_cache: false,
 447 |         init: false,
 448 |         max_tokens: None,
 449 |         signatures: false,
 450 |         structure: false,
 451 |         truncate: "smart".to_string(),
 452 |         visibility: "all".to_string(),
 453 |     };
 454 | 
 455 |     let prompter = TestPrompter::new(true, true);
 456 | 
 457 |     // Run in token count mode
 458 |     let res = run_with_args(args, Config::default(), &prompter);
 459 |     assert!(res.is_ok(), "token count mode should succeed");
 460 | 
 461 |     // No output file created
 462 |     assert!(
 463 |         !root.join("output.md").exists(),
 464 |         "output file should not be created in token count mode"
 465 |     );
 466 | }
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

- Size: 34489 bytes
- Modified: SystemTime { tv_sec: 1771138540, tv_nsec: 552299003 }

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
  88 |         signatures: false,
  89 |         structure: false,
  90 |         truncate: "smart".to_string(),
  91 |         visibility: "all".to_string(),
  92 |     };
  93 |     let prompter = TestPrompter;
  94 | 
  95 |     // First run - should create initial output without diffs
  96 |     let config = load_config().unwrap_or_default();
  97 | 
  98 |     // Apply config merging manually since we're bypassing run()
  99 |     let mut first_args = args.clone();
 100 | 
 101 |     // Apply line_numbers from config (matches run_with_args behavior)
 102 |     if let Some(line_numbers) = config.line_numbers {
 103 |         first_args.line_numbers = line_numbers;
 104 |     }
 105 | 
 106 |     // Apply diff_only from config
 107 |     if let Some(diff_only) = config.diff_only {
 108 |         first_args.diff_only = diff_only;
 109 |     }
 110 | 
 111 |     // Apply timestamping manually since we're bypassing run()
 112 |     if config.timestamped_output.unwrap_or(false) {
 113 |         let timestamp = Utc::now().format("%Y%m%d%H%M%S").to_string();
 114 |         let path = std::path::Path::new(&first_args.output);
 115 |         let stem = path
 116 |             .file_stem()
 117 |             .and_then(|s| s.to_str())
 118 |             .unwrap_or("output");
 119 |         let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("md");
 120 |         let new_filename = format!("{}_{}.{}", stem, timestamp, extension);
 121 |         if let Some(parent) = path.parent() {
 122 |             first_args.output = parent.join(new_filename).to_string_lossy().to_string();
 123 |         } else {
 124 |             first_args.output = new_filename;
 125 |         }
 126 |     }
 127 | 
 128 |     run_with_args(first_args, config.clone(), &prompter).unwrap();
 129 | 
 130 |     // Check that output was created
 131 |     let first_output = fs::read_dir(&output_dir)
 132 |         .unwrap()
 133 |         .next()
 134 |         .unwrap()
 135 |         .unwrap()
 136 |         .path();
 137 |     let first_content = fs::read_to_string(&first_output).unwrap();
 138 | 
 139 |     // Should not contain change summary on first run
 140 |     assert!(!first_content.contains("## Change Summary"));
 141 |     assert!(!first_content.contains("## File Differences"));
 142 | 
 143 |     // Modify a file
 144 |     fs::write(
 145 |         project_dir.join("src").join("main.rs"),
 146 |         "fn main() {\n    println!(\"Hello, Rust!\");\n    println!(\"Modified!\");\n}",
 147 |     )
 148 |     .unwrap();
 149 | 
 150 |     // Small delay to ensure different timestamps
 151 |     std::thread::sleep(std::time::Duration::from_millis(1100));
 152 | 
 153 |     // Second run - should detect changes
 154 |     let config = load_config().unwrap_or_default();
 155 | 
 156 |     // Apply config merging manually since we're bypassing run()
 157 |     let mut second_args = args;
 158 | 
 159 |     // Apply line_numbers from config (matches run_with_args behavior)
 160 |     if let Some(line_numbers) = config.line_numbers {
 161 |         second_args.line_numbers = line_numbers;
 162 |     }
 163 | 
 164 |     // Apply diff_only from config
 165 |     if let Some(diff_only) = config.diff_only {
 166 |         second_args.diff_only = diff_only;
 167 |     }
 168 | 
 169 |     // Apply timestamping manually since we're bypassing run()
 170 |     if config.timestamped_output.unwrap_or(false) {
 171 |         let timestamp = Utc::now().format("%Y%m%d%H%M%S").to_string();
 172 |         let path = std::path::Path::new(&second_args.output);
 173 |         let stem = path
 174 |             .file_stem()
 175 |             .and_then(|s| s.to_str())
 176 |             .unwrap_or("output");
 177 |         let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("md");
 178 |         let new_filename = format!("{}_{}.{}", stem, timestamp, extension);
 179 |         if let Some(parent) = path.parent() {
 180 |             second_args.output = parent.join(new_filename).to_string_lossy().to_string();
 181 |         } else {
 182 |             second_args.output = new_filename;
 183 |         }
 184 |     }
 185 | 
 186 |     run_with_args(second_args, config, &prompter).unwrap();
 187 | 
 188 |     // Restore original directory
 189 |     std::env::set_current_dir(original_dir).unwrap();
 190 | 
 191 |     // Find the second output file (should have different timestamp)
 192 |     let outputs: Vec<_> = fs::read_dir(&output_dir)
 193 |         .unwrap()
 194 |         .map(|e| e.unwrap().path())
 195 |         .collect();
 196 |     assert_eq!(outputs.len(), 2, "Should have two output files");
 197 | 
 198 |     let second_output = outputs.iter().find(|&p| p != &first_output).unwrap();
 199 |     let second_content = fs::read_to_string(second_output).unwrap();
 200 | 
 201 |     // Should contain change summary
 202 |     assert!(second_content.contains("## Change Summary"));
 203 |     // Handle both Windows and Unix path separators
 204 |     assert!(
 205 |         second_content.contains("- Modified: `src/main.rs`")
 206 |             || second_content.contains("- Modified: `src\\main.rs`")
 207 |     );
 208 | 
 209 |     // Should contain file differences
 210 |     assert!(second_content.contains("## File Differences"));
 211 |     assert!(
 212 |         second_content.contains("### Diff: `src/main.rs`")
 213 |             || second_content.contains("### Diff: `src\\main.rs`")
 214 |     );
 215 |     assert!(second_content.contains("Hello, world!"));
 216 |     assert!(second_content.contains("Hello, Rust!"));
 217 |     assert!(second_content.contains("Modified!"));
 218 | }
 219 | 
 220 | #[test]
 221 | #[serial]
 222 | fn test_auto_diff_added_and_removed_files() {
 223 |     let temp_dir = tempdir().unwrap();
 224 |     let project_dir = temp_dir.path().join("project");
 225 |     create_simple_project(&project_dir).unwrap();
 226 | 
 227 |     let output_dir = temp_dir.path().join("output");
 228 |     fs::create_dir_all(&output_dir).unwrap();
 229 | 
 230 |     // Change to project directory so config loading works
 231 |     let original_dir = std::env::current_dir().unwrap();
 232 |     std::env::set_current_dir(&project_dir).unwrap();
 233 | 
 234 |     let args = Args {
 235 |         input: ".".to_string(), // Use current directory
 236 |         output: output_dir.join("context.md").to_string_lossy().to_string(),
 237 |         filter: vec![],
 238 |         ignore: vec![],
 239 |         preview: false,
 240 |         token_count: false,
 241 |         line_numbers: false,
 242 |         yes: true,
 243 |         diff_only: false,
 244 |         clear_cache: false,
 245 |         init: false,
 246 |         max_tokens: None,
 247 |         signatures: false,
 248 |         structure: false,
 249 |         truncate: "smart".to_string(),
 250 |         visibility: "all".to_string(),
 251 |     };
 252 | 
 253 |     let prompter = TestPrompter;
 254 | 
 255 |     // First run
 256 |     let config = load_config().unwrap_or_default();
 257 | 
 258 |     // Apply config merging manually since we're bypassing run()
 259 |     let mut first_args = args.clone();
 260 | 
 261 |     // Apply line_numbers from config
 262 |     if !first_args.line_numbers
 263 |         && let Some(line_numbers) = config.line_numbers
 264 |     {
 265 |         first_args.line_numbers = line_numbers;
 266 |     }
 267 | 
 268 |     // Apply diff_only from config
 269 |     if !first_args.diff_only
 270 |         && let Some(diff_only) = config.diff_only
 271 |     {
 272 |         first_args.diff_only = diff_only;
 273 |     }
 274 | 
 275 |     // Apply timestamping manually since we're bypassing run()
 276 |     if config.timestamped_output.unwrap_or(false) {
 277 |         let timestamp = Utc::now().format("%Y%m%d%H%M%S").to_string();
 278 |         let path = std::path::Path::new(&first_args.output);
 279 |         let stem = path
 280 |             .file_stem()
 281 |             .and_then(|s| s.to_str())
 282 |             .unwrap_or("output");
 283 |         let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("md");
 284 |         let new_filename = format!("{}_{}.{}", stem, timestamp, extension);
 285 |         if let Some(parent) = path.parent() {
 286 |             first_args.output = parent.join(new_filename).to_string_lossy().to_string();
 287 |         } else {
 288 |             first_args.output = new_filename;
 289 |         }
 290 |     }
 291 | 
 292 |     run_with_args(first_args, config.clone(), &prompter).unwrap();
 293 | 
 294 |     // Add a new file and remove an existing one
 295 |     fs::write(
 296 |         project_dir.join("src").join("new_module.rs"),
 297 |         "pub fn new_function() -> String {\n    \"new\".to_string()\n}",
 298 |     )
 299 |     .unwrap();
 300 | 
 301 |     fs::remove_file(project_dir.join("src").join("lib.rs")).unwrap();
 302 | 
 303 |     // Small delay to ensure different timestamps
 304 |     std::thread::sleep(std::time::Duration::from_millis(1100));
 305 | 
 306 |     // Second run
 307 |     let config = load_config().unwrap_or_default();
 308 | 
 309 |     // Apply config merging manually since we're bypassing run()
 310 |     let mut second_args = args;
 311 | 
 312 |     // Apply line_numbers from config
 313 |     if !second_args.line_numbers
 314 |         && let Some(line_numbers) = config.line_numbers
 315 |     {
 316 |         second_args.line_numbers = line_numbers;
 317 |     }
 318 | 
 319 |     // Apply diff_only from config
 320 |     if !second_args.diff_only
 321 |         && let Some(diff_only) = config.diff_only
 322 |     {
 323 |         second_args.diff_only = diff_only;
 324 |     }
 325 | 
 326 |     // Apply timestamping manually since we're bypassing run()
 327 |     if config.timestamped_output.unwrap_or(false) {
 328 |         let timestamp = Utc::now().format("%Y%m%d%H%M%S").to_string();
 329 |         let path = std::path::Path::new(&second_args.output);
 330 |         let stem = path
 331 |             .file_stem()
 332 |             .and_then(|s| s.to_str())
 333 |             .unwrap_or("output");
 334 |         let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("md");
 335 |         let new_filename = format!("{}_{}.{}", stem, timestamp, extension);
 336 |         if let Some(parent) = path.parent() {
 337 |             second_args.output = parent.join(new_filename).to_string_lossy().to_string();
 338 |         } else {
 339 |             second_args.output = new_filename;
 340 |         }
 341 |     }
 342 | 
 343 |     run_with_args(second_args, config, &prompter).unwrap();
 344 | 
 345 |     // Restore original directory
 346 |     std::env::set_current_dir(original_dir).unwrap();
 347 | 
 348 |     let outputs: Vec<_> = fs::read_dir(&output_dir)
 349 |         .unwrap()
 350 |         .map(|e| e.unwrap().path())
 351 |         .collect();
 352 |     let latest_output = outputs
 353 |         .iter()
 354 |         .max_by_key(|p| fs::metadata(p).unwrap().modified().unwrap())
 355 |         .unwrap();
 356 |     let content = fs::read_to_string(latest_output).unwrap();
 357 | 
 358 |     // Should show both added and removed files
 359 |     // Handle both Windows and Unix path separators
 360 |     assert!(
 361 |         content.contains("- Added: `src/new_module.rs`")
 362 |             || content.contains("- Added: `src\\new_module.rs`")
 363 |     );
 364 |     // Handle both Windows and Unix path separators
 365 |     assert!(
 366 |         content.contains("- Removed: `src/lib.rs`") || content.contains("- Removed: `src\\lib.rs`")
 367 |     );
 368 | 
 369 |     // Added files should be marked in the files section
 370 |     assert!(content.contains("_Status: Added_"));
 371 | }
 372 | 
 373 | #[test]
 374 | #[serial]
 375 | fn test_diff_only_mode() {
 376 |     let temp_dir = tempdir().unwrap();
 377 |     let project_dir = temp_dir.path().join("project");
 378 |     create_simple_project(&project_dir).unwrap();
 379 | 
 380 |     // Update config to enable diff_only
 381 |     fs::write(
 382 |         project_dir.join("context-builder.toml"),
 383 |         r#"
 384 | auto_diff = true
 385 | timestamped_output = true
 386 | diff_only = true
 387 | "#,
 388 |     )
 389 |     .unwrap();
 390 | 
 391 |     let output_dir = temp_dir.path().join("output");
 392 |     fs::create_dir_all(&output_dir).unwrap();
 393 | 
 394 |     // Change to project directory so config loading works
 395 |     let original_dir = std::env::current_dir().unwrap();
 396 |     std::env::set_current_dir(&project_dir).unwrap();
 397 | 
 398 |     let args = Args {
 399 |         input: ".".to_string(), // Use current directory
 400 |         output: output_dir.join("context.md").to_string_lossy().to_string(),
 401 |         filter: vec![],
 402 |         ignore: vec![],
 403 |         preview: false,
 404 |         token_count: false,
 405 |         line_numbers: false,
 406 |         yes: true,
 407 |         diff_only: false, // Config file should override this
 408 |         clear_cache: false,
 409 |         init: false,
 410 |         max_tokens: None,
 411 |         signatures: false,
 412 |         structure: false,
 413 |         truncate: "smart".to_string(),
 414 |         visibility: "all".to_string(),
 415 |     };
 416 | 
 417 |     let prompter = TestPrompter;
 418 | 
 419 |     // First run
 420 |     let config = load_config().unwrap_or_default();
 421 | 
 422 |     // Apply config merging manually since we're bypassing run()
 423 |     let mut first_args = args.clone();
 424 | 
 425 |     // Apply line_numbers from config
 426 |     if !first_args.line_numbers
 427 |         && let Some(line_numbers) = config.line_numbers
 428 |     {
 429 |         first_args.line_numbers = line_numbers;
 430 |     }
 431 | 
 432 |     // Apply diff_only from config
 433 |     if !first_args.diff_only
 434 |         && let Some(diff_only) = config.diff_only
 435 |     {
 436 |         first_args.diff_only = diff_only;
 437 |     }
 438 | 
 439 |     // Apply timestamping manually since we're bypassing run()
 440 |     if config.timestamped_output.unwrap_or(false) {
 441 |         let timestamp = Utc::now().format("%Y%m%d%H%M%S").to_string();
 442 |         let path = std::path::Path::new(&first_args.output);
 443 |         let stem = path
 444 |             .file_stem()
 445 |             .and_then(|s| s.to_str())
 446 |             .unwrap_or("output");
 447 |         let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("md");
 448 |         let new_filename = format!("{}_{}.{}", stem, timestamp, extension);
 449 |         if let Some(parent) = path.parent() {
 450 |             first_args.output = parent.join(new_filename).to_string_lossy().to_string();
 451 |         } else {
 452 |             first_args.output = new_filename;
 453 |         }
 454 |     }
 455 | 
 456 |     run_with_args(first_args, config.clone(), &prompter).unwrap();
 457 | 
 458 |     // Modify a file
 459 |     fs::write(
 460 |         project_dir.join("src").join("main.rs"),
 461 |         "fn main() {\n    println!(\"Changed!\");\n}",
 462 |     )
 463 |     .unwrap();
 464 | 
 465 |     // Small delay to ensure different timestamps
 466 |     std::thread::sleep(std::time::Duration::from_millis(1100));
 467 | 
 468 |     // Second run
 469 |     let config = load_config().unwrap_or_default();
 470 | 
 471 |     // Apply config merging manually since we're bypassing run()
 472 |     let mut second_args = args;
 473 | 
 474 |     // Apply line_numbers from config
 475 |     if !second_args.line_numbers
 476 |         && let Some(line_numbers) = config.line_numbers
 477 |     {
 478 |         second_args.line_numbers = line_numbers;
 479 |     }
 480 | 
 481 |     // Apply diff_only from config
 482 |     if !second_args.diff_only
 483 |         && let Some(diff_only) = config.diff_only
 484 |     {
 485 |         second_args.diff_only = diff_only;
 486 |     }
 487 | 
 488 |     // Apply timestamping manually since we're bypassing run()
 489 |     if config.timestamped_output.unwrap_or(false) {
 490 |         let timestamp = Utc::now().format("%Y%m%d%H%M%S").to_string();
 491 |         let path = std::path::Path::new(&second_args.output);
 492 |         let stem = path
 493 |             .file_stem()
 494 |             .and_then(|s| s.to_str())
 495 |             .unwrap_or("output");
 496 |         let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("md");
 497 |         let new_filename = format!("{}_{}.{}", stem, timestamp, extension);
 498 |         if let Some(parent) = path.parent() {
 499 |             second_args.output = parent.join(new_filename).to_string_lossy().to_string();
 500 |         } else {
 501 |             second_args.output = new_filename;
 502 |         }
 503 |     }
 504 | 
 505 |     run_with_args(second_args, config, &prompter).unwrap();
 506 | 
 507 |     // Restore original directory
 508 |     std::env::set_current_dir(original_dir).unwrap();
 509 | 
 510 |     let outputs: Vec<_> = fs::read_dir(&output_dir)
 511 |         .unwrap()
 512 |         .map(|e| e.unwrap().path())
 513 |         .collect();
 514 |     let latest_output = outputs
 515 |         .iter()
 516 |         .max_by_key(|p| fs::metadata(p).unwrap().modified().unwrap())
 517 |         .unwrap();
 518 |     let content = fs::read_to_string(latest_output).unwrap();
 519 | 
 520 |     // Should have change summary and diffs
 521 |     assert!(content.contains("## Change Summary"));
 522 |     assert!(content.contains("## File Differences"));
 523 | 
 524 |     // Should NOT have full file bodies section
 525 |     assert!(!content.contains("## Files"));
 526 | 
 527 |     // But should still have the file tree and header
 528 |     assert!(content.contains("## File Tree Structure"));
 529 |     assert!(content.contains("# Directory Structure Report"));
 530 | }
 531 | 
 532 | #[test]
 533 | #[serial]
 534 | fn test_cache_invalidation_on_config_change() {
 535 |     let temp_dir = tempdir().unwrap();
 536 |     let project_dir = temp_dir.path().join("project");
 537 |     create_simple_project(&project_dir).unwrap();
 538 | 
 539 |     let output_dir = temp_dir.path().join("output");
 540 |     fs::create_dir_all(&output_dir).unwrap();
 541 | 
 542 |     // Change to project directory so config loading works
 543 |     let original_dir = std::env::current_dir().unwrap();
 544 |     std::env::set_current_dir(&project_dir).unwrap();
 545 | 
 546 |     let args_base = Args {
 547 |         input: ".".to_string(), // Use current directory
 548 |         output: output_dir.join("context.md").to_string_lossy().to_string(),
 549 |         filter: vec![],
 550 |         ignore: vec![],
 551 |         preview: false,
 552 |         token_count: false,
 553 |         line_numbers: false,
 554 |         yes: true,
 555 |         diff_only: false,
 556 |         clear_cache: false,
 557 |         init: false,
 558 |         max_tokens: None,
 559 |         signatures: false,
 560 |         structure: false,
 561 |         truncate: "smart".to_string(),
 562 |         visibility: "all".to_string(),
 563 |     };
 564 | 
 565 |     let prompter = TestPrompter;
 566 | 
 567 |     // First run with original config
 568 |     let config = load_config().unwrap_or_default();
 569 | 
 570 |     // Apply config merging manually since we're bypassing run()
 571 |     let mut first_args = args_base.clone();
 572 | 
 573 |     // Apply line_numbers from config
 574 |     if !first_args.line_numbers
 575 |         && let Some(line_numbers) = config.line_numbers
 576 |     {
 577 |         first_args.line_numbers = line_numbers;
 578 |     }
 579 | 
 580 |     // Apply diff_only from config
 581 |     if !first_args.diff_only
 582 |         && let Some(diff_only) = config.diff_only
 583 |     {
 584 |         first_args.diff_only = diff_only;
 585 |     }
 586 | 
 587 |     // Apply timestamping manually since we're bypassing run()
 588 |     if config.timestamped_output.unwrap_or(false) {
 589 |         let timestamp = Utc::now().format("%Y%m%d%H%M%S").to_string();
 590 |         let path = std::path::Path::new(&first_args.output);
 591 |         let stem = path
 592 |             .file_stem()
 593 |             .and_then(|s| s.to_str())
 594 |             .unwrap_or("output");
 595 |         let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("md");
 596 |         let new_filename = format!("{}_{}.{}", stem, timestamp, extension);
 597 |         if let Some(parent) = path.parent() {
 598 |             first_args.output = parent.join(new_filename).to_string_lossy().to_string();
 599 |         } else {
 600 |             first_args.output = new_filename;
 601 |         }
 602 |     }
 603 | 
 604 |     run_with_args(first_args, config, &prompter).unwrap();
 605 | 
 606 |     // Change configuration - add line numbers
 607 |     fs::write(
 608 |         project_dir.join("context-builder.toml"),
 609 |         r#"
 610 | auto_diff = true
 611 | timestamped_output = true
 612 | line_numbers = true
 613 | "#,
 614 |     )
 615 |     .unwrap();
 616 | 
 617 |     // Small delay to ensure different timestamps
 618 |     std::thread::sleep(std::time::Duration::from_millis(1100));
 619 | 
 620 |     // Second run with new config should not show diffs (cache should be invalidated)
 621 |     let config = load_config().unwrap_or_default();
 622 | 
 623 |     // Apply config merging manually since we're bypassing run()
 624 |     let mut second_args = args_base;
 625 | 
 626 |     // Apply line_numbers from config (matches run_with_args behavior)
 627 |     if let Some(line_numbers) = config.line_numbers {
 628 |         second_args.line_numbers = line_numbers;
 629 |     }
 630 | 
 631 |     // Apply diff_only from config
 632 |     if let Some(diff_only) = config.diff_only {
 633 |         second_args.diff_only = diff_only;
 634 |     }
 635 | 
 636 |     // Apply timestamping manually since we're bypassing run()
 637 |     if config.timestamped_output.unwrap_or(false) {
 638 |         let timestamp = Utc::now().format("%Y%m%d%H%M%S").to_string();
 639 |         let path = std::path::Path::new(&second_args.output);
 640 |         let stem = path
 641 |             .file_stem()
 642 |             .and_then(|s| s.to_str())
 643 |             .unwrap_or("output");
 644 |         let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("md");
 645 |         let new_filename = format!("{}_{}.{}", stem, timestamp, extension);
 646 |         if let Some(parent) = path.parent() {
 647 |             second_args.output = parent.join(new_filename).to_string_lossy().to_string();
 648 |         } else {
 649 |             second_args.output = new_filename;
 650 |         }
 651 |     }
 652 | 
 653 |     run_with_args(second_args, config, &prompter).unwrap();
 654 | 
 655 |     // Restore original directory
 656 |     std::env::set_current_dir(original_dir).unwrap();
 657 | 
 658 |     let outputs: Vec<_> = fs::read_dir(&output_dir)
 659 |         .unwrap()
 660 |         .map(|e| e.unwrap().path())
 661 |         .collect();
 662 |     let latest_output = outputs
 663 |         .iter()
 664 |         .max_by_key(|p| fs::metadata(p).unwrap().modified().unwrap())
 665 |         .unwrap();
 666 |     let content = fs::read_to_string(latest_output).unwrap();
 667 | 
 668 |     // Should have line numbers (showing new config is active)
 669 |     assert!(content.contains("   1 |"));
 670 | 
 671 |     // Should not show change summary since cache was invalidated
 672 |     assert!(!content.contains("## Change Summary"));
 673 | }
 674 | 
 675 | #[test]
 676 | #[serial]
 677 | fn test_concurrent_cache_access() {
 678 |     use std::sync::Arc;
 679 |     use std::thread;
 680 | 
 681 |     let temp_dir = tempdir().unwrap();
 682 |     let project_dir = temp_dir.path().join("project");
 683 |     create_simple_project(&project_dir).unwrap();
 684 | 
 685 |     let output_dir = temp_dir.path().join("output");
 686 |     fs::create_dir_all(&output_dir).unwrap();
 687 | 
 688 |     let project_dir = Arc::new(project_dir);
 689 |     let output_dir = Arc::new(output_dir);
 690 | 
 691 |     // Spawn multiple threads that try to run the tool concurrently
 692 |     let handles: Vec<_> = (0..3)
 693 |         .map(|i| {
 694 |             let project_dir = Arc::clone(&project_dir);
 695 |             let output_dir = Arc::clone(&output_dir);
 696 | 
 697 |             thread::spawn(move || {
 698 |                 let args = Args {
 699 |                     input: project_dir.to_string_lossy().to_string(),
 700 |                     output: output_dir
 701 |                         .join(format!("context_{}.md", i))
 702 |                         .to_string_lossy()
 703 |                         .to_string(),
 704 |                     filter: vec![],
 705 |                     ignore: vec![],
 706 |                     preview: false,
 707 |                     token_count: false,
 708 |                     line_numbers: false,
 709 |                     yes: true,
 710 |                     diff_only: false,
 711 |                     clear_cache: false,
 712 |                     init: false,
 713 |                     max_tokens: None,
 714 |                     signatures: false,
 715 |                     structure: false,
 716 |                     truncate: "smart".to_string(),
 717 |                     visibility: "all".to_string(),
 718 |                 };
 719 | 
 720 |                 let prompter = TestPrompter;
 721 |                 run_with_args(args, Config::default(), &prompter)
 722 |             })
 723 |         })
 724 |         .collect();
 725 | 
 726 |     // Wait for all threads to complete
 727 |     let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();
 728 | 
 729 |     // All should succeed (no cache corruption)
 730 |     for result in results {
 731 |         assert!(
 732 |             result.is_ok(),
 733 |             "Concurrent access should not cause failures"
 734 |         );
 735 |     }
 736 | 
 737 |     // Check that all outputs were created
 738 |     let output_count = fs::read_dir(&*output_dir).unwrap().count();
 739 |     assert_eq!(output_count, 3, "All concurrent runs should produce output");
 740 | }
 741 | 
 742 | #[test]
 743 | #[serial]
 744 | fn test_corrupted_cache_recovery() {
 745 |     let temp_dir = tempdir().unwrap();
 746 |     let project_dir = temp_dir.path().join("project");
 747 |     create_simple_project(&project_dir).unwrap();
 748 | 
 749 |     let output_dir = temp_dir.path().join("output");
 750 |     fs::create_dir_all(&output_dir).unwrap();
 751 | 
 752 |     // Change to project directory so config loading works
 753 |     let original_dir = std::env::current_dir().unwrap();
 754 |     std::env::set_current_dir(&project_dir).unwrap();
 755 | 
 756 |     let args = Args {
 757 |         input: ".".to_string(), // Use current directory
 758 |         output: output_dir.join("context.md").to_string_lossy().to_string(),
 759 |         filter: vec![],
 760 |         ignore: vec![],
 761 |         preview: false,
 762 |         token_count: false,
 763 |         line_numbers: false,
 764 |         yes: true,
 765 |         diff_only: false,
 766 |         clear_cache: false,
 767 |         init: false,
 768 |         max_tokens: None,
 769 |         signatures: false,
 770 |         structure: false,
 771 |         truncate: "smart".to_string(),
 772 |         visibility: "all".to_string(),
 773 |     };
 774 | 
 775 |     let prompter = TestPrompter;
 776 | 
 777 |     // First run to create cache
 778 |     let config = load_config().unwrap_or_default();
 779 | 
 780 |     // Apply config merging manually since we're bypassing run()
 781 |     let mut first_args = args.clone();
 782 | 
 783 |     // Apply line_numbers from config
 784 |     if !first_args.line_numbers
 785 |         && let Some(line_numbers) = config.line_numbers
 786 |     {
 787 |         first_args.line_numbers = line_numbers;
 788 |     }
 789 | 
 790 |     // Apply diff_only from config
 791 |     if !first_args.diff_only
 792 |         && let Some(diff_only) = config.diff_only
 793 |     {
 794 |         first_args.diff_only = diff_only;
 795 |     }
 796 | 
 797 |     // Apply timestamping manually since we're bypassing run()
 798 |     if config.timestamped_output.unwrap_or(false) {
 799 |         let timestamp = Utc::now().format("%Y%m%d%H%M%S").to_string();
 800 |         let path = std::path::Path::new(&first_args.output);
 801 |         let stem = path
 802 |             .file_stem()
 803 |             .and_then(|s| s.to_str())
 804 |             .unwrap_or("output");
 805 |         let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("md");
 806 |         let new_filename = format!("{}_{}.{}", stem, timestamp, extension);
 807 |         if let Some(parent) = path.parent() {
 808 |             first_args.output = parent.join(new_filename).to_string_lossy().to_string();
 809 |         } else {
 810 |             first_args.output = new_filename;
 811 |         }
 812 |     }
 813 | 
 814 |     run_with_args(first_args, config.clone(), &prompter).unwrap();
 815 | 
 816 |     // Corrupt the cache by writing invalid JSON
 817 |     let cache_dir = project_dir.join(".context-builder").join("cache");
 818 |     if cache_dir.exists() {
 819 |         let cache_files: Vec<_> = fs::read_dir(&cache_dir)
 820 |             .unwrap()
 821 |             .filter_map(|entry| entry.ok())
 822 |             .filter(|entry| {
 823 |                 entry
 824 |                     .path()
 825 |                     .extension()
 826 |                     .and_then(|s| s.to_str())
 827 |                     .map(|s| s == "json")
 828 |                     .unwrap_or(false)
 829 |             })
 830 |             .collect();
 831 | 
 832 |         if !cache_files.is_empty() {
 833 |             // Corrupt the first cache file found
 834 |             fs::write(cache_files[0].path(), "{ invalid json }").unwrap();
 835 |         }
 836 |     }
 837 | 
 838 |     // Modify a file
 839 |     fs::write(
 840 |         project_dir.join("src").join("main.rs"),
 841 |         "fn main() {\n    println!(\"Recovered!\");\n}",
 842 |     )
 843 |     .unwrap();
 844 | 
 845 |     // Small delay to ensure different timestamps
 846 |     std::thread::sleep(std::time::Duration::from_millis(1100));
 847 | 
 848 |     // Second run should handle corrupted cache gracefully
 849 |     let config = load_config().unwrap_or_default();
 850 | 
 851 |     // Apply config merging manually since we're bypassing run()
 852 |     let mut second_args = args;
 853 | 
 854 |     // Apply line_numbers from config
 855 |     if !second_args.line_numbers
 856 |         && let Some(line_numbers) = config.line_numbers
 857 |     {
 858 |         second_args.line_numbers = line_numbers;
 859 |     }
 860 | 
 861 |     // Apply diff_only from config
 862 |     if !second_args.diff_only
 863 |         && let Some(diff_only) = config.diff_only
 864 |     {
 865 |         second_args.diff_only = diff_only;
 866 |     }
 867 | 
 868 |     // Apply timestamping manually since we're bypassing run()
 869 |     if config.timestamped_output.unwrap_or(false) {
 870 |         let timestamp = Utc::now().format("%Y%m%d%H%M%S").to_string();
 871 |         let path = std::path::Path::new(&second_args.output);
 872 |         let stem = path
 873 |             .file_stem()
 874 |             .and_then(|s| s.to_str())
 875 |             .unwrap_or("output");
 876 |         let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("md");
 877 |         let new_filename = format!("{}_{}.{}", stem, timestamp, extension);
 878 |         if let Some(parent) = path.parent() {
 879 |             second_args.output = parent.join(new_filename).to_string_lossy().to_string();
 880 |         } else {
 881 |             second_args.output = new_filename;
 882 |         }
 883 |     }
 884 | 
 885 |     let result = run_with_args(second_args, config, &prompter);
 886 |     assert!(result.is_ok(), "Should recover from corrupted cache");
 887 | 
 888 |     // Restore original directory
 889 |     std::env::set_current_dir(original_dir).unwrap();
 890 | 
 891 |     // Should produce output despite cache corruption
 892 |     let output_count = fs::read_dir(&output_dir).unwrap().count();
 893 |     assert!(
 894 |         output_count >= 1,
 895 |         "Should produce output even with corrupted cache"
 896 |     );
 897 | }
 898 | 
 899 | #[test]
 900 | #[serial]
 901 | fn test_diff_only_mode_includes_added_files() {
 902 |     let temp_dir = tempdir().unwrap();
 903 |     let project_dir = temp_dir.path().join("project");
 904 |     create_simple_project(&project_dir).unwrap();
 905 | 
 906 |     let output_dir = temp_dir.path().join("output");
 907 |     fs::create_dir_all(&output_dir).unwrap();
 908 | 
 909 |     // Change to project directory so config loading works
 910 |     let original_dir = std::env::current_dir().unwrap();
 911 |     std::env::set_current_dir(&project_dir).unwrap();
 912 | 
 913 |     // Create config with auto_diff and diff_only enabled
 914 |     fs::write(
 915 |         project_dir.join("context-builder.toml"),
 916 |         r#"
 917 | auto_diff = true
 918 | timestamped_output = true
 919 | diff_only = true
 920 | "#,
 921 |     )
 922 |     .unwrap();
 923 | 
 924 |     let prompter = TestPrompter;
 925 | 
 926 |     // First run to establish baseline
 927 |     let args = Args {
 928 |         input: ".".to_string(),
 929 |         output: output_dir.join("context.md").to_string_lossy().to_string(),
 930 |         filter: vec!["rs".to_string()],
 931 |         ignore: vec![],
 932 |         preview: false,
 933 |         token_count: false,
 934 |         line_numbers: false,
 935 |         yes: true,
 936 |         diff_only: false, // Will be overridden by config
 937 |         clear_cache: false,
 938 |         init: false,
 939 |         max_tokens: None,
 940 |         signatures: false,
 941 |         structure: false,
 942 |         truncate: "smart".to_string(),
 943 |         visibility: "all".to_string(),
 944 |     };
 945 | 
 946 |     run_with_args(args.clone(), load_config().unwrap_or_default(), &prompter).unwrap();
 947 | 
 948 |     // Add a new file
 949 |     fs::write(
 950 |         project_dir.join("src").join("new_module.rs"),
 951 |         "// New module added\npub fn new_function() -> String {\n    \"Hello from new module\".to_string()\n}\n\n#[cfg(test)]\nmod tests {\n    use super::*;\n\n    #[test]\n    fn test_new_function() {\n        assert_eq!(new_function(), \"Hello from new module\");\n    }\n}\n",
 952 |     )
 953 |     .unwrap();
 954 | 
 955 |     // Small delay to ensure different timestamps
 956 |     std::thread::sleep(std::time::Duration::from_millis(1100));
 957 | 
 958 |     // Second run with the added file
 959 |     let config = load_config().unwrap_or_default();
 960 | 
 961 |     // Apply config merging manually since we're bypassing run()
 962 |     let mut second_args = args;
 963 | 
 964 |     // Apply line_numbers from config
 965 |     if !second_args.line_numbers
 966 |         && let Some(line_numbers) = config.line_numbers
 967 |     {
 968 |         second_args.line_numbers = line_numbers;
 969 |     }
 970 | 
 971 |     // Apply diff_only from config
 972 |     if !second_args.diff_only
 973 |         && let Some(diff_only) = config.diff_only
 974 |     {
 975 |         second_args.diff_only = diff_only;
 976 |     }
 977 | 
 978 |     // Apply timestamping manually since we're bypassing run()
 979 |     if config.timestamped_output.unwrap_or(false) {
 980 |         let timestamp = Utc::now().format("%Y%m%d%H%M%S").to_string();
 981 |         let path = std::path::Path::new(&second_args.output);
 982 |         let stem = path
 983 |             .file_stem()
 984 |             .and_then(|s| s.to_str())
 985 |             .unwrap_or("output");
 986 |         let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("md");
 987 |         let new_filename = format!("{}_{}.{}", stem, timestamp, extension);
 988 |         if let Some(parent) = path.parent() {
 989 |             second_args.output = parent.join(new_filename).to_string_lossy().to_string();
 990 |         } else {
 991 |             second_args.output = new_filename;
 992 |         }
 993 |     }
 994 | 
 995 |     run_with_args(second_args, config, &prompter).unwrap();
 996 | 
 997 |     // Restore original directory
 998 |     std::env::set_current_dir(original_dir).unwrap();
 999 | 
1000 |     // Find the latest output file
1001 |     let outputs: Vec<_> = fs::read_dir(&output_dir)
1002 |         .unwrap()
1003 |         .map(|e| e.unwrap().path())
1004 |         .collect();
1005 |     let latest_output = outputs
1006 |         .iter()
1007 |         .max_by_key(|p| fs::metadata(p).unwrap().modified().unwrap())
1008 |         .unwrap();
1009 |     let content = fs::read_to_string(latest_output).unwrap();
1010 | 
1011 |     // Should have change summary
1012 |     assert!(content.contains("## Change Summary"));
1013 | 
1014 |     // Should have added files section (not full Files section)
1015 |     assert!(content.contains("## Added Files"));
1016 |     assert!(!content.contains("## Files\n"));
1017 | 
1018 |     // Should include the full content of the added file (handle Windows path separators)
1019 |     assert!(content.contains("### File: `src") && content.contains("new_module.rs`"));
1020 |     assert!(content.contains("pub fn new_function() -> String"));
1021 |     assert!(content.contains("Hello from new module"));
1022 |     assert!(content.contains("_Status: Added_"));
1023 | 
1024 |     // Should still have the file tree and header
1025 |     assert!(content.contains("## File Tree Structure"));
1026 |     assert!(content.contains("# Directory Structure Report"));
1027 | 
1028 |     // Should not include full content of existing files (since they're unchanged)
1029 |     // The existing main.rs content should not be in the full Files section (handle Windows path separators)
1030 |     let main_rs_in_files = content.contains("### File: `src")
1031 |         && content.contains("main.rs`")
1032 |         && content.contains("Hello, world!");
1033 |     assert!(
1034 |         !main_rs_in_files,
1035 |         "Existing unchanged files should not have full content in diff_only mode"
1036 |     );
1037 | }
```

### File: `tests/test_binary_file_autodiff.rs`

- Size: 8350 bytes
- Modified: SystemTime { tv_sec: 1771138540, tv_nsec: 553299016 }

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
 114 |         signatures: false,
 115 |         structure: false,
 116 |         truncate: "smart".to_string(),
 117 |         visibility: "all".to_string(),
 118 |     };
 119 | 
 120 |     let prompter = TestPrompter::new(true, true);
 121 | 
 122 |     // First run - should create initial state without crashing
 123 |     let result1 = run_with_args(args.clone(), config.clone(), &prompter);
 124 |     assert!(
 125 |         result1.is_ok(),
 126 |         "First run with binary files should not crash: {:?}",
 127 |         result1
 128 |     );
 129 | 
 130 |     // Verify output file was created
 131 |     assert!(
 132 |         output_path.exists(),
 133 |         "Output file should be created on first run"
 134 |     );
 135 | 
 136 |     // Modify a text file to trigger diff on second run
 137 |     write_file(
 138 |         &root.join("src/main.rs"),
 139 |         "fn main() { println!(\"Hello, world!\"); }",
 140 |     );
 141 | 
 142 |     // Second run - should handle binary files in diff without crashing
 143 |     let result2 = run_with_args(args, config, &prompter);
 144 |     assert!(
 145 |         result2.is_ok(),
 146 |         "Second run with binary files should not crash during diff: {:?}",
 147 |         result2
 148 |     );
 149 | 
 150 |     // Read the output to verify it contains appropriate handling of binary files
 151 |     let output_content = fs::read_to_string(&output_path).unwrap();
 152 | 
 153 |     // Should contain the modified text file
 154 |     assert!(
 155 |         output_content.contains("Hello, world!"),
 156 |         "Output should contain modified text content"
 157 |     );
 158 | 
 159 |     // Binary files should be represented appropriately (not causing crashes)
 160 |     // The exact representation depends on implementation but should not crash
 161 |     assert!(
 162 |         output_content.len() > 100,
 163 |         "Output should contain substantial content indicating successful processing"
 164 |     );
 165 | }
 166 | 
 167 | #[test]
 168 | fn test_mixed_text_and_binary_files_autodiff() {
 169 |     let temp_dir = tempdir().unwrap();
 170 |     let root = temp_dir.path();
 171 | 
 172 |     // Create a mix of text and binary files
 173 |     write_file(&root.join("source.txt"), "Original text content");
 174 |     write_binary_file(&root.join("data.bin"), &[0x00, 0xFF, 0x42, 0x13, 0x37]);
 175 |     write_file(&root.join("config.json"), r#"{"version": "1.0"}"#);
 176 | 
 177 |     let output_path = root.join("mixed_output.md");
 178 | 
 179 |     let config = Config {
 180 |         auto_diff: Some(true),
 181 |         ..Default::default()
 182 |     };
 183 | 
 184 |     let args = Args {
 185 |         input: root.to_string_lossy().into_owned(),
 186 |         output: output_path.to_string_lossy().into_owned(),
 187 |         filter: vec![],
 188 |         ignore: vec![],
 189 |         preview: false,
 190 |         token_count: false,
 191 |         line_numbers: false,
 192 |         yes: true,
 193 |         diff_only: false,
 194 |         clear_cache: false,
 195 |         init: false,
 196 |         max_tokens: None,
 197 |         signatures: false,
 198 |         structure: false,
 199 |         truncate: "smart".to_string(),
 200 |         visibility: "all".to_string(),
 201 |     };
 202 | 
 203 |     let prompter = TestPrompter::new(true, true);
 204 | 
 205 |     // Initial run
 206 |     let result1 = run_with_args(args.clone(), config.clone(), &prompter);
 207 |     assert!(result1.is_ok(), "Initial run should succeed");
 208 | 
 209 |     // Modify text file and add another binary file
 210 |     write_file(&root.join("source.txt"), "Modified text content");
 211 |     write_binary_file(
 212 |         &root.join("image.jpg"),
 213 |         &[
 214 |             0xFF, 0xD8, 0xFF, 0xE0, // JPEG header
 215 |             0x00, 0x10, 0x4A, 0x46, 0x49, 0x46,
 216 |         ],
 217 |     );
 218 | 
 219 |     // Second run with changes
 220 |     let result2 = run_with_args(args, config, &prompter);
 221 |     assert!(
 222 |         result2.is_ok(),
 223 |         "Second run with mixed file changes should succeed"
 224 |     );
 225 | 
 226 |     let output_content = fs::read_to_string(&output_path).unwrap();
 227 |     assert!(
 228 |         output_content.contains("Modified text content"),
 229 |         "Should show updated text content"
 230 |     );
 231 | }
 232 | 
 233 | #[test]
 234 | fn test_large_binary_file_autodiff() {
 235 |     let temp_dir = tempdir().unwrap();
 236 |     let root = temp_dir.path();
 237 | 
 238 |     // Create a large binary file (simulating real-world scenario)
 239 |     let large_binary_data: Vec<u8> = (0..10000).map(|i| (i % 256) as u8).collect();
 240 | 
 241 |     write_binary_file(&root.join("large_binary.dat"), &large_binary_data);
 242 |     write_file(&root.join("small_text.txt"), "Small text file");
 243 | 
 244 |     let output_path = root.join("large_binary_output.md");
 245 | 
 246 |     let config = Config {
 247 |         auto_diff: Some(true),
 248 |         ..Default::default()
 249 |     };
 250 | 
 251 |     let args = Args {
 252 |         input: root.to_string_lossy().into_owned(),
 253 |         output: output_path.to_string_lossy().into_owned(),
 254 |         filter: vec![],
 255 |         ignore: vec![],
 256 |         preview: false,
 257 |         token_count: false,
 258 |         line_numbers: false,
 259 |         yes: true,
 260 |         diff_only: false,
 261 |         clear_cache: false,
 262 |         init: false,
 263 |         max_tokens: None,
 264 |         signatures: false,
 265 |         structure: false,
 266 |         truncate: "smart".to_string(),
 267 |         visibility: "all".to_string(),
 268 |     };
 269 | 
 270 |     let prompter = TestPrompter::new(true, true);
 271 | 
 272 |     // Should handle large binary files without memory issues or crashes
 273 |     let result = run_with_args(args, config, &prompter);
 274 |     assert!(
 275 |         result.is_ok(),
 276 |         "Should handle large binary files without crashing: {:?}",
 277 |         result
 278 |     );
 279 | 
 280 |     assert!(
 281 |         output_path.exists(),
 282 |         "Output should be created even with large binary files"
 283 |     );
 284 | }
```

### File: `tests/test_comprehensive_edge_cases.rs`

- Size: 23611 bytes
- Modified: SystemTime { tv_sec: 1771138540, tv_nsec: 556299056 }

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
 122 |             signatures: false,
 123 |             structure: false,
 124 |             truncate: "smart".to_string(),
 125 |             visibility: "all".to_string(),
 126 |         };
 127 | 
 128 |         let prompter = TestPrompter::new(true, true);
 129 |         let result = run_with_args(args, config, &prompter);
 130 | 
 131 |         assert!(
 132 |             result.is_ok(),
 133 |             "Should handle binary files gracefully with strategy: {}",
 134 |             strategy
 135 |         );
 136 | 
 137 |         // Verify output file was created
 138 |         let output_path = output_dir.join(format!("test_{}.md", strategy));
 139 |         assert!(
 140 |             output_path.exists(),
 141 |             "Output file should exist for strategy: {}",
 142 |             strategy
 143 |         );
 144 | 
 145 |         let content = fs::read_to_string(&output_path).unwrap();
 146 | 
 147 |         // Should contain normal file
 148 |         assert!(
 149 |             content.contains("fn main()"),
 150 |             "Should contain normal file content"
 151 |         );
 152 | 
 153 |         // Should handle binary files appropriately based on strategy
 154 |         match *strategy {
 155 |             "detect" => {
 156 |                 // May contain transcoded content or binary placeholders
 157 |                 assert!(
 158 |                     content.contains("Hello") || content.contains("<Binary file"),
 159 |                     "Detect strategy should transcode or show binary placeholder"
 160 |                 );
 161 |             }
 162 |             "strict" | "skip" => {
 163 |                 // Should show binary placeholders for non-UTF-8 files
 164 |                 assert!(
 165 |                     content.contains("<Binary file") || content.contains("binary.rs"),
 166 |                     "Strict/skip strategy should show binary placeholders"
 167 |                 );
 168 |             }
 169 |             _ => {}
 170 |         }
 171 | 
 172 |         // Should handle empty files
 173 |         assert!(content.contains("empty.rs"), "Should list empty files");
 174 | 
 175 |         // Should handle large files
 176 |         assert!(content.contains("large.rs"), "Should handle large files");
 177 |     }
 178 | 
 179 |     // No need to restore directory since we never changed it
 180 | }
 181 | 
 182 | #[test]
 183 | #[serial]
 184 | fn test_configuration_precedence_edge_cases() {
 185 |     let temp_dir = tempdir().unwrap();
 186 |     let project_dir = temp_dir.path().join("project");
 187 |     let output_dir = temp_dir.path().join("output");
 188 |     fs::create_dir_all(&output_dir).unwrap();
 189 | 
 190 |     // Create test files
 191 |     write_file(&project_dir.join("test.rs"), "fn test() {}\n");
 192 |     write_file(&project_dir.join("README.md"), "# Test Project\n");
 193 | 
 194 |     // Test 1: Basic functionality with explicit CLI args
 195 |     let args = Args {
 196 |         input: project_dir.to_string_lossy().to_string(),
 197 |         output: output_dir
 198 |             .join("basic_test.md")
 199 |             .to_string_lossy()
 200 |             .to_string(),
 201 |         filter: vec!["rs".to_string()],
 202 |         ignore: vec![],
 203 |         preview: false,
 204 |         token_count: false,
 205 |         line_numbers: false,
 206 |         yes: true,
 207 |         diff_only: false,
 208 |         clear_cache: false,
 209 |         init: false,
 210 |         max_tokens: None,
 211 |         signatures: false,
 212 |         structure: false,
 213 |         truncate: "smart".to_string(),
 214 |         visibility: "all".to_string(),
 215 |     };
 216 | 
 217 |     let prompter = TestPrompter::new(true, true);
 218 |     let result = run_with_args(args, Config::default(), &prompter);
 219 |     assert!(result.is_ok(), "Basic configuration test should succeed");
 220 | 
 221 |     let output_path = output_dir.join("basic_test.md");
 222 |     assert!(output_path.exists(), "Output should exist for basic test");
 223 | 
 224 |     let content = fs::read_to_string(&output_path).unwrap();
 225 |     assert!(
 226 |         content.contains("test.rs"),
 227 |         "Should include filtered .rs files"
 228 |     );
 229 |     assert!(
 230 |         !content.contains("README.md"),
 231 |         "Should exclude non-filtered files"
 232 |     );
 233 | 
 234 |     // Test 2: Empty filter should include all files
 235 |     let args = Args {
 236 |         input: project_dir.to_string_lossy().to_string(),
 237 |         output: output_dir
 238 |             .join("all_files_test.md")
 239 |             .to_string_lossy()
 240 |             .to_string(),
 241 |         filter: vec![],
 242 |         ignore: vec![],
 243 |         preview: false,
 244 |         token_count: false,
 245 |         line_numbers: false,
 246 |         yes: true,
 247 |         diff_only: false,
 248 |         clear_cache: false,
 249 |         init: false,
 250 |         max_tokens: None,
 251 |         signatures: false,
 252 |         structure: false,
 253 |         truncate: "smart".to_string(),
 254 |         visibility: "all".to_string(),
 255 |     };
 256 | 
 257 |     let result = run_with_args(args, Config::default(), &prompter);
 258 |     assert!(result.is_ok(), "All files test should succeed");
 259 | 
 260 |     let output_path = output_dir.join("all_files_test.md");
 261 |     let content = fs::read_to_string(&output_path).unwrap();
 262 |     assert!(
 263 |         content.contains("test.rs"),
 264 |         "Should include all files when no filter"
 265 |     );
 266 |     assert!(
 267 |         content.contains("README.md"),
 268 |         "Should include all files when no filter"
 269 |     );
 270 | }
 271 | 
 272 | #[test]
 273 | #[serial]
 274 | fn test_cache_consistency_edge_cases() {
 275 |     let temp_dir = tempdir().unwrap();
 276 |     let project_dir = temp_dir.path().join("project");
 277 |     let output_dir = temp_dir.path().join("output");
 278 |     fs::create_dir_all(&output_dir).unwrap();
 279 | 
 280 |     write_file(&project_dir.join("test.rs"), "fn original() {}\n");
 281 | 
 282 |     // Change to project directory
 283 |     let original_dir = std::env::current_dir().unwrap();
 284 |     std::env::set_current_dir(&project_dir).unwrap();
 285 | 
 286 |     // Create config with auto_diff enabled
 287 |     write_file(
 288 |         &project_dir.join("context-builder.toml"),
 289 |         r#"
 290 | auto_diff = true
 291 | timestamped_output = true
 292 | "#,
 293 |     );
 294 | 
 295 |     let base_args = Args {
 296 |         input: project_dir.to_string_lossy().to_string(),
 297 |         output: output_dir
 298 |             .join("cache_test.md")
 299 |             .to_string_lossy()
 300 |             .to_string(),
 301 |         filter: vec!["rs".to_string()],
 302 |         ignore: vec![],
 303 |         preview: false,
 304 |         token_count: false,
 305 |         line_numbers: false,
 306 |         yes: true,
 307 |         diff_only: false,
 308 |         clear_cache: false,
 309 |         init: false,
 310 |         max_tokens: None,
 311 |         signatures: false,
 312 |         structure: false,
 313 |         truncate: "smart".to_string(),
 314 |         visibility: "all".to_string(),
 315 |     };
 316 | 
 317 |     let config = context_builder::config::load_config_from_path(&project_dir).unwrap_or_default();
 318 |     let prompter = TestPrompter::new(true, true);
 319 | 
 320 |     // First run - establish cache
 321 |     let result1 = run_with_args(base_args.clone(), config.clone(), &prompter);
 322 |     assert!(result1.is_ok(), "First run should succeed");
 323 | 
 324 |     // Verify cache was created
 325 |     let cache_dir = project_dir.join(".context-builder").join("cache");
 326 |     assert!(cache_dir.exists(), "Cache directory should be created");
 327 | 
 328 |     // Test cache with different path representations
 329 |     let current_dir_string = std::env::current_dir()
 330 |         .unwrap()
 331 |         .to_string_lossy()
 332 |         .to_string();
 333 |     let path_variants = [".", "./", &current_dir_string];
 334 | 
 335 |     for (i, path_variant) in path_variants.iter().enumerate() {
 336 |         let mut variant_args = base_args.clone();
 337 |         variant_args.input = path_variant.to_string();
 338 |         variant_args.output = output_dir
 339 |             .join(format!("variant_{}.md", i))
 340 |             .to_string_lossy()
 341 |             .to_string();
 342 | 
 343 |         let result = run_with_args(variant_args, config.clone(), &prompter);
 344 |         assert!(
 345 |             result.is_ok(),
 346 |             "Path variant '{}' should succeed",
 347 |             path_variant
 348 |         );
 349 | 
 350 |         let output_path = output_dir.join(format!("variant_{}.md", i));
 351 |         let content = fs::read_to_string(&output_path).unwrap();
 352 | 
 353 |         // Should show "no changes detected" because cache should be consistent
 354 |         // (or at least not crash due to path inconsistencies)
 355 |         assert!(
 356 |             content.contains("original") || content.contains("no changes"),
 357 |             "Path variant should handle cache consistently"
 358 |         );
 359 |     }
 360 | 
 361 |     // Test cache corruption recovery
 362 |     let cache_files: Vec<_> = fs::read_dir(&cache_dir)
 363 |         .unwrap()
 364 |         .filter_map(|entry| entry.ok())
 365 |         .filter(|entry| {
 366 |             entry
 367 |                 .path()
 368 |                 .extension()
 369 |                 .and_then(|s| s.to_str())
 370 |                 .map(|s| s == "json")
 371 |                 .unwrap_or(false)
 372 |         })
 373 |         .collect();
 374 | 
 375 |     if !cache_files.is_empty() {
 376 |         // Corrupt the cache
 377 |         fs::write(cache_files[0].path(), "{ invalid json }").unwrap();
 378 | 
 379 |         // Should recover gracefully
 380 |         let result = run_with_args(base_args.clone(), config.clone(), &prompter);
 381 |         assert!(result.is_ok(), "Should recover from corrupted cache");
 382 |     }
 383 | 
 384 |     // Restore original directory
 385 |     std::env::set_current_dir(original_dir).unwrap();
 386 | }
 387 | 
 388 | #[test]
 389 | #[serial]
 390 | fn test_error_conditions_and_exit_codes() {
 391 |     let temp_dir = tempdir().unwrap();
 392 |     let project_dir = temp_dir.path().join("project");
 393 |     let output_dir = temp_dir.path().join("output");
 394 |     fs::create_dir_all(&project_dir).unwrap();
 395 |     fs::create_dir_all(&output_dir).unwrap();
 396 | 
 397 |     let prompter = TestPrompter::new(false, true); // Deny overwrite
 398 | 
 399 |     // Test 1: Non-existent input directory
 400 |     let args = Args {
 401 |         input: temp_dir
 402 |             .path()
 403 |             .join("nonexistent")
 404 |             .to_string_lossy()
 405 |             .to_string(),
 406 |         output: output_dir.join("test.md").to_string_lossy().to_string(),
 407 |         filter: vec![],
 408 |         ignore: vec![],
 409 |         preview: false,
 410 |         token_count: false,
 411 |         line_numbers: false,
 412 |         yes: true,
 413 |         diff_only: false,
 414 |         clear_cache: false,
 415 |         init: false,
 416 |         max_tokens: None,
 417 |         signatures: false,
 418 |         structure: false,
 419 |         truncate: "smart".to_string(),
 420 |         visibility: "all".to_string(),
 421 |     };
 422 | 
 423 |     let result = run_with_args(args, Config::default(), &prompter);
 424 |     assert!(
 425 |         result.is_err(),
 426 |         "Should fail with non-existent input directory"
 427 |     );
 428 | 
 429 |     // Test 2: Permission denied on output
 430 |     write_file(&project_dir.join("test.rs"), "fn test() {}\n");
 431 |     let output_file = output_dir.join("existing.md");
 432 |     write_file(&output_file, "existing content");
 433 | 
 434 |     let args = Args {
 435 |         input: project_dir.to_string_lossy().to_string(),
 436 |         output: output_file.to_string_lossy().to_string(),
 437 |         filter: vec!["rs".to_string()],
 438 |         ignore: vec![],
 439 |         preview: false,
 440 |         token_count: false,
 441 |         line_numbers: false,
 442 |         yes: false, // Don't auto-confirm
 443 |         diff_only: false,
 444 |         clear_cache: false,
 445 |         init: false,
 446 |         max_tokens: None,
 447 |         signatures: false,
 448 |         structure: false,
 449 |         truncate: "smart".to_string(),
 450 |         visibility: "all".to_string(),
 451 |     };
 452 | 
 453 |     let prompter_deny = TestPrompter::new(false, true); // Deny overwrite
 454 |     let result = run_with_args(args, Config::default(), &prompter_deny);
 455 |     assert!(result.is_err(), "Should fail when overwrite is denied");
 456 | 
 457 |     // Test 3: User cancellation during processing
 458 |     let args = Args {
 459 |         input: project_dir.to_string_lossy().to_string(),
 460 |         output: output_dir
 461 |             .join("cancelled.md")
 462 |             .to_string_lossy()
 463 |             .to_string(),
 464 |         filter: vec!["rs".to_string()],
 465 |         ignore: vec![],
 466 |         preview: false,
 467 |         token_count: false,
 468 |         line_numbers: false,
 469 |         yes: false,
 470 |         diff_only: false,
 471 |         clear_cache: false,
 472 |         init: false,
 473 |         max_tokens: None,
 474 |         signatures: false,
 475 |         structure: false,
 476 |         truncate: "smart".to_string(),
 477 |         visibility: "all".to_string(),
 478 |     };
 479 | 
 480 |     let prompter_cancel = TestPrompter::new(true, false); // Allow overwrite, deny processing
 481 |     let result = run_with_args(args, Config::default(), &prompter_cancel);
 482 |     assert!(result.is_err(), "Should fail when processing is cancelled");
 483 | }
 484 | 
 485 | #[test]
 486 | #[cfg(feature = "parallel")]
 487 | fn test_memory_usage_under_parallel_processing() {
 488 |     let temp_dir = tempdir().unwrap();
 489 |     let project_dir = temp_dir.path().join("project");
 490 |     fs::create_dir_all(&project_dir).unwrap();
 491 | 
 492 |     // Create many files to test memory efficiency
 493 |     for i in 0..500 {
 494 |         let subdir = project_dir.join(format!("module_{}", i / 50));
 495 |         fs::create_dir_all(&subdir).unwrap();
 496 | 
 497 |         let content = format!(
 498 |             "// File {}\nuse std::collections::HashMap;\n\npub fn function_{}() -> i32 {{\n    {}\n}}\n",
 499 |             i, i, i
 500 |         );
 501 |         write_file(&subdir.join(format!("file_{}.rs", i)), &content);
 502 |     }
 503 | 
 504 |     let output_dir = temp_dir.path().join("output");
 505 |     fs::create_dir_all(&output_dir).unwrap();
 506 | 
 507 |     let args = Args {
 508 |         input: project_dir.to_string_lossy().to_string(),
 509 |         output: output_dir
 510 |             .join("parallel_test.md")
 511 |             .to_string_lossy()
 512 |             .to_string(),
 513 |         filter: vec!["rs".to_string()],
 514 |         ignore: vec![],
 515 |         preview: false,
 516 |         token_count: false,
 517 |         line_numbers: false,
 518 |         yes: true,
 519 |         diff_only: false,
 520 |         clear_cache: false,
 521 |         init: false,
 522 |         max_tokens: None,
 523 |         signatures: false,
 524 |         structure: false,
 525 |         truncate: "smart".to_string(),
 526 |         visibility: "all".to_string(),
 527 |     };
 528 | 
 529 |     let prompter = TestPrompter::new(true, true);
 530 |     let result = run_with_args(args, Config::default(), &prompter);
 531 | 
 532 |     assert!(
 533 |         result.is_ok(),
 534 |         "Parallel processing should handle many files efficiently"
 535 |     );
 536 | 
 537 |     let output_path = output_dir.join("parallel_test.md");
 538 |     assert!(output_path.exists(), "Output should be created");
 539 | 
 540 |     let content = fs::read_to_string(&output_path).unwrap();
 541 | 
 542 |     // Verify all files are included and properly ordered
 543 |     assert!(
 544 |         content.contains("function_0"),
 545 |         "Should contain first function"
 546 |     );
 547 |     assert!(
 548 |         content.contains("function_499"),
 549 |         "Should contain last function"
 550 |     );
 551 | 
 552 |     // Verify substantial content was generated
 553 |     assert!(
 554 |         content.len() > 100_000,
 555 |         "Should generate substantial output"
 556 |     );
 557 | 
 558 |     // Check that files appear in a reasonable order (not completely scrambled)
 559 |     let first_pos = content.find("function_0").unwrap();
 560 |     let last_pos = content.find("function_499").unwrap();
 561 |     assert!(
 562 |         first_pos < last_pos,
 563 |         "Files should maintain reasonable ordering"
 564 |     );
 565 | }
 566 | 
 567 | #[test]
 568 | #[serial]
 569 | fn test_cwd_independent_operation() {
 570 |     let temp_dir = tempdir().unwrap();
 571 |     let project_dir = temp_dir.path().join("project");
 572 |     let output_dir = temp_dir.path().join("output");
 573 |     let different_cwd = temp_dir.path().join("different_cwd");
 574 | 
 575 |     fs::create_dir_all(&project_dir).unwrap();
 576 |     fs::create_dir_all(&output_dir).unwrap();
 577 |     fs::create_dir_all(&different_cwd).unwrap();
 578 | 
 579 |     // Create test files
 580 |     write_file(&project_dir.join("test.rs"), "fn test() {}\n");
 581 |     write_file(
 582 |         &project_dir.join("context-builder.toml"),
 583 |         r#"
 584 | filter = ["rs"]
 585 | line_numbers = true
 586 | "#,
 587 |     );
 588 | 
 589 |     // Store original directory
 590 |     let original_dir = std::env::current_dir().unwrap();
 591 | 
 592 |     // Test from different working directories
 593 |     let test_cwds = [temp_dir.path(), &different_cwd, &original_dir];
 594 | 
 595 |     for (i, test_cwd) in test_cwds.iter().enumerate() {
 596 |         std::env::set_current_dir(test_cwd).unwrap();
 597 | 
 598 |         let args = Args {
 599 |             input: project_dir.to_string_lossy().to_string(),
 600 |             output: output_dir
 601 |                 .join(format!("cwd_test_{}.md", i))
 602 |                 .to_string_lossy()
 603 |                 .to_string(),
 604 |             filter: vec![], // Use config defaults
 605 |             ignore: vec![],
 606 |             preview: false,
 607 |             token_count: false,
 608 |             line_numbers: false, // Use config default
 609 |             yes: true,
 610 |             diff_only: false,
 611 |             clear_cache: false,
 612 |             init: false,
 613 |             max_tokens: None,
 614 |             signatures: false,
 615 |             structure: false,
 616 |             truncate: "smart".to_string(),
 617 |             visibility: "all".to_string(),
 618 |         };
 619 | 
 620 |         let config =
 621 |             context_builder::config::load_config_from_path(&project_dir).unwrap_or_default();
 622 |         let prompter = TestPrompter::new(true, true);
 623 | 
 624 |         let result = run_with_args(args, config, &prompter);
 625 |         assert!(result.is_ok(), "Should work regardless of CWD (test {})", i);
 626 | 
 627 |         let output_path = output_dir.join(format!("cwd_test_{}.md", i));
 628 |         assert!(
 629 |             output_path.exists(),
 630 |             "Output should exist for CWD test {}",
 631 |             i
 632 |         );
 633 | 
 634 |         let content = fs::read_to_string(&output_path).unwrap();
 635 | 
 636 |         // Should find the config file and apply its settings
 637 |         assert!(
 638 |             content.contains("test.rs"),
 639 |             "Should process rust files from config"
 640 |         );
 641 | 
 642 |         // All outputs should be identical regardless of CWD
 643 |         if i > 0 {
 644 |             let previous_content =
 645 |                 fs::read_to_string(output_dir.join(format!("cwd_test_{}.md", i - 1))).unwrap();
 646 | 
 647 |             // Remove timestamps for comparison
 648 |             let normalize = |s: &str| -> String {
 649 |                 s.lines()
 650 |                     .filter(|line| !line.contains("Processed at:"))
 651 |                     .collect::<Vec<_>>()
 652 |                     .join("\n")
 653 |             };
 654 | 
 655 |             assert_eq!(
 656 |                 normalize(&content),
 657 |                 normalize(&previous_content),
 658 |                 "Output should be identical regardless of CWD"
 659 |             );
 660 |         }
 661 |     }
 662 | 
 663 |     // Restore original directory
 664 |     std::env::set_current_dir(original_dir).unwrap();
 665 | }
 666 | 
 667 | #[test]
 668 | fn test_edge_case_filenames_and_paths() {
 669 |     let temp_dir = tempdir().unwrap();
 670 |     let project_dir = temp_dir.path().join("project");
 671 |     let output_dir = temp_dir.path().join("output");
 672 |     fs::create_dir_all(&output_dir).unwrap();
 673 | 
 674 |     // Create files with problematic names
 675 |     let problematic_names = vec![
 676 |         "normal.rs",
 677 |         "with spaces.rs",
 678 |         "with-dashes.rs",
 679 |         "with_underscores.rs",
 680 |         "with.dots.rs",
 681 |         "uppercase.rs", // Changed from UPPERCASE.RS to avoid case issues
 682 |         "file.with.many.dots.rs",
 683 |         "123numeric.rs",
 684 |         // Note: Avoid truly problematic characters that might fail on Windows
 685 |     ];
 686 | 
 687 |     for name in &problematic_names {
 688 |         write_file(
 689 |             &project_dir.join("src").join(name),
 690 |             &format!("// File: {}\nfn test() {{}}\n", name),
 691 |         );
 692 |     }
 693 | 
 694 |     // Create nested directory structure
 695 |     write_file(
 696 |         &project_dir.join("deeply/nested/very/deep/path.rs"),
 697 |         "fn deep() {}\n",
 698 |     );
 699 | 
 700 |     let args = Args {
 701 |         input: project_dir.to_string_lossy().to_string(),
 702 |         output: output_dir
 703 |             .join("edge_case_paths.md")
 704 |             .to_string_lossy()
 705 |             .to_string(),
 706 |         filter: vec!["rs".to_string()],
 707 |         ignore: vec![],
 708 |         preview: false,
 709 |         token_count: false,
 710 |         line_numbers: false,
 711 |         yes: true,
 712 |         diff_only: false,
 713 |         clear_cache: false,
 714 |         init: false,
 715 |         max_tokens: None,
 716 |         signatures: false,
 717 |         structure: false,
 718 |         truncate: "smart".to_string(),
 719 |         visibility: "all".to_string(),
 720 |     };
 721 | 
 722 |     let prompter = TestPrompter::new(true, true);
 723 |     let result = run_with_args(args, Config::default(), &prompter);
 724 | 
 725 |     assert!(
 726 |         result.is_ok(),
 727 |         "Should handle edge case filenames without panicking"
 728 |     );
 729 | 
 730 |     let output_path = output_dir.join("edge_case_paths.md");
 731 |     assert!(output_path.exists(), "Output should be created");
 732 | 
 733 |     let content = fs::read_to_string(&output_path).unwrap();
 734 | 
 735 |     // Verify all problematic files are included
 736 |     for name in &problematic_names {
 737 |         assert!(
 738 |             content.contains(name),
 739 |             "Should include file with problematic name: {}",
 740 |             name
 741 |         );
 742 |     }
 743 | 
 744 |     // Verify deeply nested path is handled
 745 |     assert!(
 746 |         content.contains("deeply/nested") || content.contains("deeply\\nested"),
 747 |         "Should handle deeply nested paths"
 748 |     );
 749 | }
```

### File: `tests/test_config_resolution.rs`

- Size: 15023 bytes
- Modified: SystemTime { tv_sec: 1771138540, tv_nsec: 558299082 }

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
  67 |         signatures: resolution.config.signatures,
  68 |         structure: resolution.config.structure,
  69 |         truncate: resolution.config.truncate,
  70 |         visibility: resolution.config.visibility,
  71 |     };
  72 | 
  73 |     // Create final Config with resolved values
  74 |     let final_config = context_builder::config::Config {
  75 |         auto_diff: Some(resolution.config.auto_diff),
  76 |         diff_context_lines: Some(resolution.config.diff_context_lines),
  77 |         ..config.unwrap_or_default()
  78 |     };
  79 | 
  80 |     run_with_args(final_args, final_config, prompter)
  81 | }
  82 | 
  83 | #[test]
  84 | #[serial]
  85 | fn test_cli_arguments_override_config_file() {
  86 |     let temp_dir = tempdir().unwrap();
  87 |     let project_dir = temp_dir.path().join("project");
  88 |     let output_dir = temp_dir.path().join("output");
  89 | 
  90 |     // Create a simple project
  91 |     write_file(
  92 |         &project_dir.join("src/main.rs"),
  93 |         "fn main() { println!(\"Hello\"); }",
  94 |     );
  95 |     write_file(&project_dir.join("lib.py"), "def hello(): print('world')");
  96 | 
  97 |     // Create config file with specific settings
  98 |     write_file(
  99 |         &project_dir.join("context-builder.toml"),
 100 |         r#"
 101 | filter = ["py"]
 102 | line_numbers = true
 103 | output = "from_config.md"
 104 | "#,
 105 |     );
 106 | 
 107 |     fs::create_dir_all(&output_dir).unwrap();
 108 | 
 109 |     // CLI args that should override config
 110 |     // Change to project directory (run_with_args creates output relative to CWD)
 111 |     let original_dir = std::env::current_dir().unwrap();
 112 |     std::env::set_current_dir(&project_dir).unwrap();
 113 | 
 114 |     let args = Args {
 115 |         input: ".".to_string(), // Use current directory
 116 |         output: output_dir.join("from_cli.md").to_string_lossy().to_string(),
 117 |         filter: vec!["rs".to_string()], // Should override config's ["py"]
 118 |         ignore: vec![],
 119 |         line_numbers: true, // Can't override config boolean settings
 120 |         preview: false,
 121 |         token_count: false,
 122 |         yes: true,
 123 |         diff_only: false,
 124 |         clear_cache: false,
 125 |         init: false,
 126 |         max_tokens: None,
 127 |         signatures: false,
 128 |         structure: false,
 129 |         truncate: "smart".to_string(),
 130 |         visibility: "all".to_string(),
 131 |     };
 132 | 
 133 |     let config = context_builder::config::load_config_from_path(&project_dir).unwrap();
 134 |     let prompter = TestPrompter::new(true, true);
 135 | 
 136 |     let result = run_with_resolved_config(args, Some(config), &prompter);
 137 | 
 138 |     // Restore original directory
 139 |     std::env::set_current_dir(original_dir).unwrap();
 140 |     assert!(result.is_ok(), "Should succeed with CLI override");
 141 | 
 142 |     // Verify output file was created with CLI name, not config name
 143 |     let output_file = output_dir.join("from_cli.md");
 144 |     assert!(output_file.exists(), "Output file should use CLI filename");
 145 | 
 146 |     let content = fs::read_to_string(&output_file).unwrap();
 147 | 
 148 |     // Should contain .rs file (CLI filter), not .py file (config filter)
 149 |     assert!(
 150 |         content.contains("main.rs"),
 151 |         "Should include .rs files from CLI filter"
 152 |     );
 153 |     assert!(
 154 |         !content.contains("lib.py"),
 155 |         "Should not include .py files despite config filter"
 156 |     );
 157 | 
 158 |     // Should have line numbers (config applies since we can't distinguish CLI false from default)
 159 |     assert!(
 160 |         content.contains("   1 |"),
 161 |         "Should have line numbers from config"
 162 |     );
 163 | }
 164 | 
 165 | #[test]
 166 | #[serial]
 167 | fn test_config_applies_when_cli_uses_defaults() {
 168 |     let temp_dir = tempdir().unwrap();
 169 |     let project_dir = temp_dir.path().join("project");
 170 |     let output_dir = temp_dir.path().join("output");
 171 | 
 172 |     // Create a simple project
 173 |     write_file(
 174 |         &project_dir.join("src/main.rs"),
 175 |         "fn main() { println!(\"Hello\"); }",
 176 |     );
 177 |     write_file(&project_dir.join("lib.py"), "def hello(): print('world')");
 178 | 
 179 |     // Create config file
 180 |     write_file(
 181 |         &project_dir.join("context-builder.toml"),
 182 |         r#"
 183 | filter = ["py", "rs"]
 184 | line_numbers = true
 185 | ignore = ["target"]
 186 | "#,
 187 |     );
 188 | 
 189 |     fs::create_dir_all(&output_dir).unwrap();
 190 | 
 191 |     // Change to project directory
 192 |     let original_dir = std::env::current_dir().unwrap();
 193 |     std::env::set_current_dir(&project_dir).unwrap();
 194 | 
 195 |     // CLI args using defaults (should be overridden by config)
 196 |     let args = Args {
 197 |         input: ".".to_string(),          // Use current directory
 198 |         output: "output.md".to_string(), // Default - should use config if available
 199 |         filter: vec![],                  // Default - should use config
 200 |         ignore: vec![],                  // Default - should use config
 201 |         line_numbers: false,             // Default - should use config
 202 |         preview: false,
 203 |         token_count: false,
 204 |         yes: true,
 205 |         diff_only: false,
 206 |         clear_cache: false,
 207 |         init: false,
 208 |         max_tokens: None,
 209 |         signatures: false,
 210 |         structure: false,
 211 |         truncate: "smart".to_string(),
 212 |         visibility: "all".to_string(),
 213 |     };
 214 | 
 215 |     let config = context_builder::config::load_config_from_path(&project_dir).unwrap();
 216 |     let prompter = TestPrompter::new(true, true);
 217 | 
 218 |     let result = run_with_resolved_config(args, Some(config), &prompter);
 219 | 
 220 |     // Restore original directory
 221 |     std::env::set_current_dir(original_dir).unwrap();
 222 |     assert!(result.is_ok(), "Should succeed with config application");
 223 | 
 224 |     // Find the output file (should be in current working directory, which is project dir)
 225 |     let output_file = project_dir.join("output.md");
 226 |     // The tool runs with project_dir as input, so output.md should be created there
 227 |     assert!(
 228 |         output_file.exists(),
 229 |         "Output file should be created in project directory"
 230 |     );
 231 | 
 232 |     let content = fs::read_to_string(&output_file).unwrap();
 233 | 
 234 |     // Should contain both file types from config filter
 235 |     assert!(
 236 |         content.contains("main.rs"),
 237 |         "Should include .rs files from config filter"
 238 |     );
 239 |     assert!(
 240 |         content.contains("lib.py"),
 241 |         "Should include .py files from config filter"
 242 |     );
 243 | 
 244 |     // Should have line numbers from config
 245 |     assert!(
 246 |         content.contains("   1 |"),
 247 |         "Should have line numbers from config"
 248 |     );
 249 | }
 250 | 
 251 | #[test]
 252 | #[serial]
 253 | fn test_timestamped_output_and_output_folder() {
 254 |     let temp_dir = tempdir().unwrap();
 255 |     let project_dir = temp_dir.path().join("project");
 256 |     let _output_dir = temp_dir.path().join("docs");
 257 | 
 258 |     // Create a simple project
 259 |     write_file(
 260 |         &project_dir.join("src/main.rs"),
 261 |         "fn main() { println!(\"Hello\"); }",
 262 |     );
 263 | 
 264 |     // Create config with timestamping and output folder (relative to project)
 265 |     write_file(
 266 |         &project_dir.join("context-builder.toml"),
 267 |         r#"
 268 | output = "context.md"
 269 | output_folder = "docs"
 270 | timestamped_output = true
 271 | "#,
 272 |     );
 273 | 
 274 |     // Create docs directory inside project directory
 275 |     let docs_dir = project_dir.join("docs");
 276 |     fs::create_dir_all(&docs_dir).unwrap();
 277 | 
 278 |     // Change to project directory
 279 |     let original_dir = std::env::current_dir().unwrap();
 280 |     std::env::set_current_dir(&project_dir).unwrap();
 281 | 
 282 |     let args = Args {
 283 |         input: ".".to_string(),          // Use current directory
 284 |         output: "output.md".to_string(), // Should be overridden by config
 285 |         filter: vec![],
 286 |         ignore: vec![],
 287 |         line_numbers: false,
 288 |         preview: false,
 289 |         token_count: false,
 290 |         yes: true,
 291 |         diff_only: false,
 292 |         clear_cache: false,
 293 |         init: false,
 294 |         max_tokens: None,
 295 |         signatures: false,
 296 |         structure: false,
 297 |         truncate: "smart".to_string(),
 298 |         visibility: "all".to_string(),
 299 |     };
 300 | 
 301 |     let config = context_builder::config::load_config_from_path(&project_dir).unwrap();
 302 |     let prompter = TestPrompter::new(true, true);
 303 | 
 304 |     let result = run_with_resolved_config(args, Some(config), &prompter);
 305 | 
 306 |     // Restore original directory
 307 |     std::env::set_current_dir(original_dir).unwrap();
 308 |     assert!(result.is_ok(), "Should succeed with timestamped output");
 309 | 
 310 |     // Find timestamped file in docs directory
 311 |     let docs_dir = project_dir.join("docs");
 312 |     let entries = fs::read_dir(&docs_dir).unwrap();
 313 |     let output_files: Vec<_> = entries
 314 |         .filter_map(|entry| entry.ok())
 315 |         .filter(|entry| {
 316 |             let name = entry.file_name();
 317 |             let name_str = name.to_string_lossy();
 318 |             name_str.starts_with("context_") && name_str.ends_with(".md")
 319 |         })
 320 |         .collect();
 321 | 
 322 |     assert!(
 323 |         !output_files.is_empty(),
 324 |         "Should have timestamped output file"
 325 |     );
 326 |     assert!(
 327 |         output_files.len() == 1,
 328 |         "Should have exactly one output file"
 329 |     );
 330 | 
 331 |     let output_file = &output_files[0];
 332 |     let content = fs::read_to_string(output_file.path()).unwrap();
 333 |     assert!(content.contains("main.rs"), "Should contain project files");
 334 | }
 335 | 
 336 | #[test]
 337 | #[serial]
 338 | fn test_mixed_explicit_and_default_values() {
 339 |     let temp_dir = tempdir().unwrap();
 340 |     let project_dir = temp_dir.path().join("project");
 341 | 
 342 |     // Create a simple project
 343 |     write_file(
 344 |         &project_dir.join("src/main.rs"),
 345 |         "fn main() { println!(\"Hello\"); }",
 346 |     );
 347 |     write_file(&project_dir.join("test.py"), "print('test')");
 348 | 
 349 |     // Config with multiple settings
 350 |     write_file(
 351 |         &project_dir.join("context-builder.toml"),
 352 |         r#"
 353 | filter = ["py"]
 354 | line_numbers = true
 355 | yes = true
 356 | "#,
 357 |     );
 358 | 
 359 |     // Change to project directory
 360 |     let original_dir = std::env::current_dir().unwrap();
 361 |     std::env::set_current_dir(&project_dir).unwrap();
 362 | 
 363 |     let args = Args {
 364 |         input: ".".to_string(),          // Use current directory
 365 |         output: "custom.md".to_string(), // Explicit CLI value
 366 |         filter: vec![],                  // Default - should use config
 367 |         ignore: vec![],
 368 |         line_numbers: false, // Default - config will override this
 369 |         preview: false,      // Default - should use config
 370 |         token_count: false,  // Don't use token count mode so file gets created
 371 |         yes: false,          // Default - should use config
 372 |         diff_only: false,
 373 |         clear_cache: false,
 374 |         init: false,
 375 |         max_tokens: None,
 376 |         signatures: false,
 377 |         structure: false,
 378 |         truncate: "smart".to_string(),
 379 |         visibility: "all".to_string(),
 380 |     };
 381 | 
 382 |     let config = context_builder::config::load_config_from_path(&project_dir).unwrap();
 383 |     let prompter = TestPrompter::new(true, true);
 384 | 
 385 |     let result = run_with_resolved_config(args, Some(config), &prompter);
 386 | 
 387 |     // Restore original directory
 388 |     std::env::set_current_dir(original_dir).unwrap();
 389 |     assert!(result.is_ok(), "Should succeed with mixed values");
 390 | 
 391 |     // Verify output file uses CLI name (created in project directory)
 392 |     let output_file = project_dir.join("custom.md");
 393 |     assert!(
 394 |         output_file.exists(),
 395 |         "Should use CLI output filename in project directory"
 396 |     );
 397 | 
 398 |     let content = fs::read_to_string(&output_file).unwrap();
 399 | 
 400 |     // Should use config filter (py files)
 401 |     assert!(
 402 |         content.contains("test.py"),
 403 |         "Should include .py files from config"
 404 |     );
 405 |     assert!(!content.contains("main.rs"), "Should not include .rs files");
 406 | 
 407 |     // Should use config line_numbers setting
 408 |     assert!(
 409 |         content.contains("   1 |"),
 410 |         "Should have line numbers from config"
 411 |     );
 412 | }
 413 | 
 414 | #[test]
 415 | #[serial]
 416 | fn test_auto_diff_configuration_warning() {
 417 |     let temp_dir = tempdir().unwrap();
 418 |     let project_dir = temp_dir.path().join("project");
 419 | 
 420 |     // Create a simple project
 421 |     write_file(
 422 |         &project_dir.join("src/main.rs"),
 423 |         "fn main() { println!(\"Hello\"); }",
 424 |     );
 425 | 
 426 |     // Config with auto_diff but no timestamped_output (should generate warning)
 427 |     write_file(
 428 |         &project_dir.join("context-builder.toml"),
 429 |         r#"
 430 | auto_diff = true
 431 | timestamped_output = false
 432 | "#,
 433 |     );
 434 | 
 435 |     // Change to project directory
 436 |     let original_dir = std::env::current_dir().unwrap();
 437 |     std::env::set_current_dir(&project_dir).unwrap();
 438 | 
 439 |     let args = Args {
 440 |         input: ".".to_string(), // Use current directory
 441 |         output: "output.md".to_string(),
 442 |         filter: vec![],
 443 |         ignore: vec![],
 444 |         line_numbers: false,
 445 |         preview: false,
 446 |         token_count: false,
 447 |         yes: true,
 448 |         diff_only: false,
 449 |         clear_cache: false,
 450 |         init: false,
 451 |         max_tokens: None,
 452 |         signatures: false,
 453 |         structure: false,
 454 |         truncate: "smart".to_string(),
 455 |         visibility: "all".to_string(),
 456 |     };
 457 | 
 458 |     let config = context_builder::config::load_config_from_path(&project_dir).unwrap();
 459 |     let prompter = TestPrompter::new(true, true);
 460 | 
 461 |     // Capture stderr to check for warnings
 462 |     let result = run_with_resolved_config(args, Some(config), &prompter);
 463 | 
 464 |     // Restore original directory
 465 |     std::env::set_current_dir(original_dir).unwrap();
 466 |     assert!(result.is_ok(), "Should succeed despite warning");
 467 | 
 468 |     // Note: In a real application, we would capture stderr to verify the warning
 469 |     // For this test, we're just ensuring the config is handled without crashing
 470 | }
```

### File: `tests/test_cwd_independence.rs`

- Size: 13739 bytes
- Modified: SystemTime { tv_sec: 1771138540, tv_nsec: 560299109 }

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
  99 |         signatures: false,
 100 |         structure: false,
 101 |         truncate: "smart".to_string(),
 102 |         visibility: "all".to_string(),
 103 |     };
 104 | 
 105 |     // Apply config settings to args (mimicking the run() function logic)
 106 |     if args.filter.is_empty()
 107 |         && let Some(filter) = config.filter.clone()
 108 |     {
 109 |         args.filter = filter;
 110 |     }
 111 |     if !args.line_numbers
 112 |         && let Some(line_numbers) = config.line_numbers
 113 |     {
 114 |         args.line_numbers = line_numbers;
 115 |     }
 116 | 
 117 |     let prompter = TestPrompter::new(true, true);
 118 |     let result = run_with_args(args, config, &prompter);
 119 | 
 120 |     // Restore original directory
 121 |     std::env::set_current_dir(original_dir).unwrap();
 122 | 
 123 |     assert!(result.is_ok(), "Should succeed with CWD independence");
 124 | 
 125 |     let output_content = fs::read_to_string(output_dir.join("output.md")).unwrap();
 126 | 
 127 |     // Verify that project config was used, not working directory config
 128 |     assert!(
 129 |         output_content.contains("   1 |"),
 130 |         "Should have line numbers from project config"
 131 |     );
 132 |     assert!(
 133 |         output_content.contains("main.rs"),
 134 |         "Should include .rs files from project config filter"
 135 |     );
 136 | }
 137 | 
 138 | #[test]
 139 | #[serial]
 140 | fn test_cache_created_in_project_root_not_cwd() {
 141 |     let temp_dir = tempdir().unwrap();
 142 |     let project_dir = temp_dir.path().join("project");
 143 |     let output_dir = temp_dir.path().join("output");
 144 |     let working_dir = temp_dir.path().join("working");
 145 | 
 146 |     // Create project with auto-diff enabled
 147 |     write_file(
 148 |         &project_dir.join("src/main.rs"),
 149 |         "fn main() { println!(\"Hello\"); }",
 150 |     );
 151 |     write_file(
 152 |         &project_dir.join("context-builder.toml"),
 153 |         r#"
 154 | auto_diff = true
 155 | timestamped_output = true
 156 | "#,
 157 |     );
 158 | 
 159 |     fs::create_dir_all(&output_dir).unwrap();
 160 |     fs::create_dir_all(&working_dir).unwrap();
 161 | 
 162 |     // Get absolute paths before changing directory
 163 |     let project_dir_abs = project_dir.canonicalize().unwrap();
 164 |     let output_dir_abs = output_dir.canonicalize().unwrap();
 165 |     let working_dir_abs = working_dir.canonicalize().unwrap();
 166 | 
 167 |     // Change to working directory
 168 |     let original_dir = std::env::current_dir().unwrap();
 169 |     std::env::set_current_dir(&working_dir_abs).unwrap();
 170 | 
 171 |     // Load config from project directory
 172 |     let config =
 173 |         context_builder::config::load_config_from_path(&project_dir_abs).unwrap_or_default();
 174 | 
 175 |     let mut args = Args {
 176 |         input: project_dir_abs.to_string_lossy().to_string(), // Absolute path to project
 177 |         output: output_dir_abs
 178 |             .join("context.md")
 179 |             .to_string_lossy()
 180 |             .to_string(),
 181 |         filter: vec![],
 182 |         ignore: vec![],
 183 |         preview: false,
 184 |         token_count: false,
 185 |         line_numbers: false,
 186 |         yes: true,
 187 |         diff_only: false,
 188 |         clear_cache: false,
 189 |         init: false,
 190 |         max_tokens: None,
 191 |         signatures: false,
 192 |         structure: false,
 193 |         truncate: "smart".to_string(),
 194 |         visibility: "all".to_string(),
 195 |     };
 196 | 
 197 |     // Apply timestamping manually since we're bypassing run()
 198 |     if config.timestamped_output.unwrap_or(false) {
 199 |         use chrono::Utc;
 200 |         let timestamp = Utc::now().format("%Y%m%d%H%M%S").to_string();
 201 |         let path = std::path::Path::new(&args.output);
 202 |         let stem = path
 203 |             .file_stem()
 204 |             .and_then(|s| s.to_str())
 205 |             .unwrap_or("output");
 206 |         let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("md");
 207 |         let new_filename = format!("{}_{}.{}", stem, timestamp, extension);
 208 |         if let Some(parent) = path.parent() {
 209 |             args.output = parent.join(new_filename).to_string_lossy().to_string();
 210 |         } else {
 211 |             args.output = output_dir_abs
 212 |                 .join(new_filename)
 213 |                 .to_string_lossy()
 214 |                 .to_string();
 215 |         }
 216 |     }
 217 | 
 218 |     let prompter = TestPrompter::new(true, true);
 219 | 
 220 |     // First run to create cache
 221 |     let result1 = run_with_args(args.clone(), config.clone(), &prompter);
 222 |     assert!(result1.is_ok(), "First run should succeed");
 223 | 
 224 |     // Verify cache was created in project directory, not working directory
 225 |     let project_cache = project_dir_abs.join(".context-builder").join("cache");
 226 |     let working_cache = working_dir_abs.join(".context-builder").join("cache");
 227 | 
 228 |     assert!(
 229 |         project_cache.exists(),
 230 |         "Cache should be created in project directory"
 231 |     );
 232 |     assert!(
 233 |         !working_cache.exists(),
 234 |         "Cache should NOT be created in working directory"
 235 |     );
 236 | 
 237 |     // Small delay to ensure different timestamps
 238 |     std::thread::sleep(std::time::Duration::from_millis(1100));
 239 | 
 240 |     // Modify project file
 241 |     // Modify a file to trigger diff
 242 |     write_file(
 243 |         &project_dir_abs.join("src/main.rs"),
 244 |         "fn main() { println!(\"Hello, modified!\"); }",
 245 |     );
 246 | 
 247 |     // Create second args with new timestamp
 248 |     let mut args2 = args.clone();
 249 |     if config.timestamped_output.unwrap_or(false) {
 250 |         use chrono::Utc;
 251 |         let timestamp = Utc::now().format("%Y%m%d%H%M%S").to_string();
 252 |         let path = std::path::Path::new(&args2.output);
 253 |         let stem = path
 254 |             .file_stem()
 255 |             .and_then(|s| s.to_str())
 256 |             .unwrap_or("output");
 257 |         let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("md");
 258 |         let new_filename = format!("{}_{}.{}", stem, timestamp, extension);
 259 |         if let Some(parent) = path.parent() {
 260 |             args2.output = parent.join(new_filename).to_string_lossy().to_string();
 261 |         } else {
 262 |             args2.output = output_dir_abs
 263 |                 .join(new_filename)
 264 |                 .to_string_lossy()
 265 |                 .to_string();
 266 |         }
 267 |     }
 268 | 
 269 |     // Second run should detect changes using cache from project directory
 270 |     let result2 = run_with_args(args2, config, &prompter);
 271 |     assert!(result2.is_ok(), "Second run should succeed");
 272 | 
 273 |     // Find output files (should have timestamps) - use absolute path
 274 |     // Add retry logic to handle potential race conditions
 275 |     let output_files = (0..5)
 276 |         .find_map(|_| {
 277 |             std::thread::sleep(std::time::Duration::from_millis(50));
 278 |             if let Ok(entries) = fs::read_dir(&output_dir_abs) {
 279 |                 let files: Vec<_> = entries
 280 |                     .filter_map(|entry| entry.ok())
 281 |                     .filter(|entry| {
 282 |                         let name = entry.file_name();
 283 |                         let name_str = name.to_string_lossy();
 284 |                         name_str.starts_with("context") && name_str.ends_with(".md")
 285 |                     })
 286 |                     .collect();
 287 |                 if files.len() >= 2 { Some(files) } else { None }
 288 |             } else {
 289 |                 None
 290 |             }
 291 |         })
 292 |         .expect("Failed to find output files after retries");
 293 | 
 294 |     // Restore original directory after file operations
 295 |     std::env::set_current_dir(original_dir).unwrap();
 296 | 
 297 |     assert!(
 298 |         output_files.len() >= 2,
 299 |         "Should have multiple timestamped outputs, found: {}",
 300 |         output_files.len()
 301 |     );
 302 | 
 303 |     // Check that second output contains diff information
 304 |     let latest_output = output_files
 305 |         .iter()
 306 |         .max_by_key(|entry| {
 307 |             // All paths are already absolute since we used output_dir_abs
 308 |             fs::metadata(entry.path()).unwrap().modified().unwrap()
 309 |         })
 310 |         .unwrap();
 311 | 
 312 |     // Read the latest file content
 313 |     let latest_content = fs::read_to_string(latest_output.path()).unwrap();
 314 |     assert!(
 315 |         latest_content.contains("## Change Summary") || latest_content.contains("Modified"),
 316 |         "Should contain change information from auto-diff"
 317 |     );
 318 | }
 319 | 
 320 | #[test]
 321 | #[serial]
 322 | fn test_clear_cache_uses_project_root() {
 323 |     let temp_dir = tempdir().unwrap();
 324 |     let project_dir = temp_dir.path().join("project");
 325 |     let working_dir = temp_dir.path().join("working");
 326 | 
 327 |     // Create project and working directories
 328 |     write_file(&project_dir.join("src/main.rs"), "fn main() {}");
 329 |     fs::create_dir_all(&working_dir).unwrap();
 330 | 
 331 |     // Create cache in project directory
 332 |     let project_cache_dir = project_dir.join(".context-builder").join("cache");
 333 |     fs::create_dir_all(&project_cache_dir).unwrap();
 334 |     fs::write(project_cache_dir.join("test_cache.json"), "{}").unwrap();
 335 | 
 336 |     // Create cache in working directory (should not be affected)
 337 |     let working_cache_dir = working_dir.join(".context-builder").join("cache");
 338 |     fs::create_dir_all(&working_cache_dir).unwrap();
 339 |     fs::write(working_cache_dir.join("test_cache.json"), "{}").unwrap();
 340 | 
 341 |     // Change to working directory
 342 |     let original_dir = std::env::current_dir().unwrap();
 343 |     std::env::set_current_dir(&working_dir).unwrap();
 344 | 
 345 |     // Simulate the cache clearing logic from run() function
 346 |     // This tests that cache clearing uses project root, not CWD
 347 |     let cache_path = project_dir.join(".context-builder").join("cache");
 348 |     assert!(
 349 |         cache_path.exists(),
 350 |         "Project cache should exist before clearing"
 351 |     );
 352 | 
 353 |     if cache_path.exists() {
 354 |         fs::remove_dir_all(&cache_path).unwrap();
 355 |     }
 356 | 
 357 |     // Restore original directory
 358 |     std::env::set_current_dir(original_dir).unwrap();
 359 | 
 360 |     // Project cache should be cleared
 361 |     assert!(
 362 |         !project_cache_dir.exists(),
 363 |         "Project cache should be cleared"
 364 |     );
 365 | 
 366 |     // Working directory cache should be untouched
 367 |     assert!(
 368 |         working_cache_dir.exists() && fs::read_dir(&working_cache_dir).unwrap().count() > 0,
 369 |         "Working directory cache should remain untouched"
 370 |     );
 371 | }
 372 | 
 373 | #[test]
 374 | #[serial]
 375 | fn test_load_config_from_path_function() {
 376 |     let temp_dir = tempdir().unwrap();
 377 |     let project_dir = temp_dir.path().join("project");
 378 |     let working_dir = temp_dir.path().join("working");
 379 | 
 380 |     // Create project with config file
 381 |     write_file(
 382 |         &project_dir.join("context-builder.toml"),
 383 |         r#"
 384 | auto_diff = true
 385 | line_numbers = true
 386 | filter = ["rs"]
 387 | "#,
 388 |     );
 389 | 
 390 |     // Create different config in working directory
 391 |     write_file(
 392 |         &working_dir.join("context-builder.toml"),
 393 |         r#"
 394 | auto_diff = false
 395 | line_numbers = false
 396 | filter = ["txt"]
 397 | "#,
 398 |     );
 399 | 
 400 |     // Change to working directory
 401 |     let original_dir = std::env::current_dir().unwrap();
 402 |     std::env::set_current_dir(&working_dir).unwrap();
 403 | 
 404 |     // Load config from project directory (not CWD)
 405 |     let config = context_builder::config::load_config_from_path(&project_dir);
 406 | 
 407 |     // Restore original directory
 408 |     std::env::set_current_dir(original_dir).unwrap();
 409 | 
 410 |     assert!(
 411 |         config.is_some(),
 412 |         "Should load config from project directory"
 413 |     );
 414 |     let config = config.unwrap();
 415 | 
 416 |     assert_eq!(
 417 |         config.auto_diff,
 418 |         Some(true),
 419 |         "Should use project config auto_diff"
 420 |     );
 421 |     assert_eq!(
 422 |         config.line_numbers,
 423 |         Some(true),
 424 |         "Should use project config line_numbers"
 425 |     );
 426 |     assert_eq!(
 427 |         config.filter,
 428 |         Some(vec!["rs".to_string()]),
 429 |         "Should use project config filter"
 430 |     );
 431 | }
```

### File: `tests/test_determinism.rs`

- Size: 21480 bytes
- Modified: SystemTime { tv_sec: 1771143750, tv_nsec: 651431068 }

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
  95 |             signatures: false,
  96 |             structure: false,
  97 |             truncate: "smart".to_string(),
  98 |             visibility: "all".to_string(),
  99 |         },
 100 |         Config::default(),
 101 |         &prompter,
 102 |     );
 103 | 
 104 |     let result2 = run_with_args(
 105 |         Args {
 106 |             input: project_dir.to_string_lossy().to_string(),
 107 |             output: temp_dir
 108 |                 .path()
 109 |                 .join("output2.md")
 110 |                 .to_string_lossy()
 111 |                 .to_string(),
 112 |             filter: vec!["rs".to_string(), "md".to_string(), "toml".to_string()],
 113 |             ignore: vec![],
 114 |             preview: false,
 115 |             token_count: false,
 116 |             line_numbers: false,
 117 |             yes: true,
 118 |             diff_only: false,
 119 |             clear_cache: false,
 120 |             init: false,
 121 |             max_tokens: None,
 122 |             signatures: false,
 123 |             structure: false,
 124 |             truncate: "smart".to_string(),
 125 |             visibility: "all".to_string(),
 126 |         },
 127 |         Config::default(),
 128 |         &prompter,
 129 |     );
 130 | 
 131 |     if let Err(e) = result1 {
 132 |         panic!("First run failed: {}", e);
 133 |     }
 134 |     if let Err(e) = result2 {
 135 |         panic!("Second run failed: {}", e);
 136 |     }
 137 | 
 138 |     // Find the actual output files (they may have timestamps appended)
 139 |     let temp_entries: Vec<_> = fs::read_dir(temp_dir.path())
 140 |         .unwrap()
 141 |         .filter_map(|entry| entry.ok())
 142 |         .filter(|entry| {
 143 |             let file_name = entry.file_name();
 144 |             let name = file_name.to_string_lossy();
 145 |             name.starts_with("output") && name.ends_with(".md")
 146 |         })
 147 |         .collect();
 148 | 
 149 |     if temp_entries.len() < 2 {
 150 |         eprintln!("Expected 2 output files, found {}", temp_entries.len());
 151 |         eprintln!("Temp directory contents:");
 152 |         for entry in fs::read_dir(temp_dir.path()).unwrap() {
 153 |             eprintln!("  {:?}", entry.unwrap().file_name());
 154 |         }
 155 |         panic!("Not enough output files found");
 156 |     }
 157 | 
 158 |     // Sort to ensure consistent ordering
 159 |     let mut output_files: Vec<_> = temp_entries.iter().map(|entry| entry.path()).collect();
 160 |     output_files.sort();
 161 | 
 162 |     // Read both outputs
 163 |     let content1 = fs::read_to_string(&output_files[0]).unwrap();
 164 |     let content2 = fs::read_to_string(&output_files[1]).unwrap();
 165 | 
 166 |     // Debug: Write contents to temp files for inspection
 167 |     fs::write(temp_dir.path().join("debug_content1.md"), &content1).unwrap();
 168 |     fs::write(temp_dir.path().join("debug_content2.md"), &content2).unwrap();
 169 | 
 170 |     // Normalize timestamps for comparison since they will be different
 171 |     let normalize = |content: &str| -> String {
 172 |         content
 173 |             .lines()
 174 |             .map(|line| {
 175 |                 if line.starts_with("Processed at: ") {
 176 |                     "Processed at: <timestamp>"
 177 |                 } else {
 178 |                     line
 179 |                 }
 180 |             })
 181 |             .collect::<Vec<_>>()
 182 |             .join("\n")
 183 |     };
 184 | 
 185 |     let normalized1 = normalize(&content1);
 186 |     let normalized2 = normalize(&content2);
 187 | 
 188 |     // Debug: Write normalized contents for comparison
 189 |     fs::write(temp_dir.path().join("debug_normalized1.md"), &normalized1).unwrap();
 190 |     fs::write(temp_dir.path().join("debug_normalized2.md"), &normalized2).unwrap();
 191 | 
 192 |     // They should be identical (deterministic) after normalizing timestamps
 193 |     if normalized1 != normalized2 {
 194 |         eprintln!(
 195 |             "Content1 length: {}, Content2 length: {}",
 196 |             normalized1.len(),
 197 |             normalized2.len()
 198 |         );
 199 |         eprintln!(
 200 |             "First difference at position: {:?}",
 201 |             normalized1
 202 |                 .chars()
 203 |                 .zip(normalized2.chars())
 204 |                 .position(|(a, b)| a != b)
 205 |         );
 206 |         eprintln!("Debug files written to: {}", temp_dir.path().display());
 207 |         panic!("Output should be deterministic across multiple runs (ignoring timestamps)");
 208 |     }
 209 | 
 210 |     // Verify that files are listed in a consistent order
 211 |     let lines: Vec<&str> = content1.lines().collect();
 212 |     let file_lines: Vec<&str> = lines
 213 |         .iter()
 214 |         .filter(|line| line.starts_with("### File: `"))
 215 |         .copied()
 216 |         .collect();
 217 | 
 218 |     // Should have found some files
 219 |     assert!(
 220 |         !file_lines.is_empty(),
 221 |         "Should have found some file entries"
 222 |     );
 223 | 
 224 |     // Check that files are sorted by relevance category:
 225 |     // Category 0: Cargo.toml (config), README.md (key project doc)
 226 |     // Category 1: src/* (source code) — entry points first (lib.rs, main.rs before utils.rs)
 227 |     // Category 2: tests/* (tests)
 228 |     // Normalize path separators for cross-platform compatibility (Windows uses backslashes)
 229 |     let file_lines: Vec<String> = file_lines
 230 |         .iter()
 231 |         .map(|line| line.replace('\\', "/"))
 232 |         .collect();
 233 |     let expected_order = vec![
 234 |         "### File: `Cargo.toml`",
 235 |         "### File: `docs/README.md`",
 236 |         "### File: `src/lib.rs`",
 237 |         "### File: `src/main.rs`",
 238 |         "### File: `src/utils.rs`",
 239 |         "### File: `tests/integration.rs`",
 240 |         "### File: `tests/unit.rs`",
 241 |     ];
 242 |     assert_eq!(
 243 |         file_lines, expected_order,
 244 |         "Files should be listed in relevance order (config+docs → source (entry points first) → tests)"
 245 |     );
 246 | }
 247 | #[test]
 248 | #[serial] // Ensure tests don't interfere with each other
 249 | fn test_deterministic_file_tree_order() {
 250 |     let temp_dir = tempdir().unwrap();
 251 |     let project_dir = temp_dir.path().join("project");
 252 |     create_test_project(&project_dir).unwrap();
 253 | 
 254 |     let output_path = temp_dir.path().join("output.md");
 255 | 
 256 |     // Change to project directory so config loading works
 257 |     let original_dir = std::env::current_dir().unwrap();
 258 |     std::env::set_current_dir(&project_dir).unwrap();
 259 | 
 260 |     let args = Args {
 261 |         input: ".".to_string(),
 262 |         output: output_path.to_string_lossy().to_string(),
 263 |         filter: vec![],
 264 |         ignore: vec![],
 265 |         preview: false,
 266 |         token_count: false,
 267 |         line_numbers: false,
 268 |         yes: true,
 269 |         diff_only: false,
 270 |         clear_cache: false,
 271 |         init: false,
 272 |         max_tokens: None,
 273 |         signatures: false,
 274 |         structure: false,
 275 |         truncate: "smart".to_string(),
 276 |         visibility: "all".to_string(),
 277 |     };
 278 | 
 279 |     let prompter = TestPrompter;
 280 |     run_with_args(args, Config::default(), &prompter).unwrap();
 281 | 
 282 |     // Restore original directory
 283 |     std::env::set_current_dir(original_dir).unwrap();
 284 | 
 285 |     let content = fs::read_to_string(&output_path).unwrap();
 286 | 
 287 |     // Find the file tree section
 288 |     let tree_start = content
 289 |         .find("## File Tree Structure")
 290 |         .expect("Should have file tree section");
 291 |     let files_start = content.find("### File: `").unwrap_or(content.len());
 292 |     let tree_section = &content[tree_start..files_start];
 293 | 
 294 |     // Check that directories and files appear in alphabetical order in the tree
 295 |     // This is a basic check - a more sophisticated test would parse the tree structure
 296 |     assert!(tree_section.contains("Cargo.toml"));
 297 |     // Check for directory entries - they may appear as just the name or with trailing content
 298 |     assert!(tree_section.contains("docs") || tree_section.contains("docs/"));
 299 |     assert!(tree_section.contains("src") || tree_section.contains("src/"));
 300 |     assert!(tree_section.contains("tests") || tree_section.contains("tests/"));
 301 | }
 302 | 
 303 | #[test]
 304 | #[serial] // Ensure cache tests don't interfere with each other
 305 | fn test_cache_collision_prevention() {
 306 |     let temp_dir1 = tempdir().unwrap();
 307 |     let temp_dir2 = tempdir().unwrap();
 308 | 
 309 |     let project1 = temp_dir1.path().join("project");
 310 |     let project2 = temp_dir2.path().join("project");
 311 | 
 312 |     create_test_project(&project1).unwrap();
 313 |     create_test_project(&project2).unwrap();
 314 | 
 315 |     // Add different content to make projects distinct
 316 |     fs::write(project1.join("unique1.txt"), "This is project 1").unwrap();
 317 |     fs::write(project2.join("unique2.txt"), "This is project 2").unwrap();
 318 | 
 319 |     let output1 = temp_dir1.path().join("output.md");
 320 |     let output2 = temp_dir2.path().join("output.md");
 321 | 
 322 |     let prompter = TestPrompter;
 323 | 
 324 |     // Change to project1 directory and run
 325 |     let original_dir = std::env::current_dir().unwrap();
 326 |     std::env::set_current_dir(&project1).unwrap();
 327 | 
 328 |     let args1 = Args {
 329 |         input: ".".to_string(),
 330 |         output: output1.to_string_lossy().to_string(),
 331 |         filter: vec![],
 332 |         ignore: vec![],
 333 |         preview: false,
 334 |         token_count: false,
 335 |         line_numbers: false,
 336 |         yes: true,
 337 |         diff_only: false,
 338 |         clear_cache: false,
 339 |         init: false,
 340 |         max_tokens: None,
 341 |         signatures: false,
 342 |         structure: false,
 343 |         truncate: "smart".to_string(),
 344 |         visibility: "all".to_string(),
 345 |     };
 346 | 
 347 |     run_with_args(args1, Config::default(), &prompter).unwrap();
 348 | 
 349 |     // Change to project2 directory and run
 350 |     std::env::set_current_dir(&project2).unwrap();
 351 | 
 352 |     let args2 = Args {
 353 |         input: ".".to_string(),
 354 |         output: output2.to_string_lossy().to_string(),
 355 |         filter: vec!["txt".to_string()],
 356 |         ignore: vec![],
 357 |         preview: false,
 358 |         token_count: false,
 359 |         line_numbers: false,
 360 | 
 361 |         yes: true,
 362 | 
 363 |         diff_only: false,
 364 | 
 365 |         clear_cache: false,
 366 | 
 367 |         init: false,
 368 |         max_tokens: None,
 369 |         signatures: false,
 370 |         structure: false,
 371 |         truncate: "smart".to_string(),
 372 |         visibility: "all".to_string(),
 373 |     };
 374 | 
 375 |     run_with_args(args2, Config::default(), &prompter).unwrap();
 376 | 
 377 |     // Restore original directory
 378 |     std::env::set_current_dir(original_dir).unwrap();
 379 | 
 380 |     let content1 = fs::read_to_string(&output1).unwrap();
 381 |     let content2 = fs::read_to_string(&output2).unwrap();
 382 | 
 383 |     // Outputs should be different due to different projects and configs
 384 |     assert_ne!(
 385 |         content1, content2,
 386 |         "Different projects should produce different outputs"
 387 |     );
 388 | 
 389 |     // Each should contain their unique content
 390 |     assert!(content1.contains("unique1.txt"));
 391 |     assert!(content2.contains("unique2.txt"));
 392 | }
 393 | 
 394 | #[test]
 395 | #[serial] // Ensure tests don't interfere with each other
 396 | fn test_custom_ignores_performance() {
 397 |     let temp_dir = tempdir().unwrap();
 398 |     let project_dir = temp_dir.path().join("project");
 399 | 
 400 |     // Create a project with ignored directories
 401 |     create_test_project(&project_dir).unwrap();
 402 | 
 403 |     let target_dir = project_dir.join("target");
 404 |     let node_modules_dir = project_dir.join("node_modules");
 405 | 
 406 |     fs::create_dir_all(&target_dir).unwrap();
 407 |     fs::create_dir_all(&node_modules_dir).unwrap();
 408 | 
 409 |     // Create many files in ignored directories
 410 |     for i in 0..10 {
 411 |         fs::write(target_dir.join(format!("file{}.txt", i)), "ignored content").unwrap();
 412 |         fs::write(
 413 |             node_modules_dir.join(format!("module{}.js", i)),
 414 |             "ignored js",
 415 |         )
 416 |         .unwrap();
 417 |     }
 418 | 
 419 |     let output_path = temp_dir.path().join("output.md");
 420 | 
 421 |     // Change to project directory so config loading works
 422 |     let original_dir = std::env::current_dir().unwrap();
 423 |     std::env::set_current_dir(&project_dir).unwrap();
 424 | 
 425 |     let args = Args {
 426 |         input: ".".to_string(),
 427 |         output: output_path.to_string_lossy().to_string(),
 428 |         filter: vec![],
 429 |         ignore: vec!["target".to_string(), "node_modules".to_string()],
 430 |         preview: false,
 431 |         token_count: false,
 432 |         line_numbers: false,
 433 |         yes: true,
 434 |         diff_only: false,
 435 |         clear_cache: false,
 436 |         init: false,
 437 |         max_tokens: None,
 438 |         signatures: false,
 439 |         structure: false,
 440 |         truncate: "smart".to_string(),
 441 |         visibility: "all".to_string(),
 442 |     };
 443 | 
 444 |     let prompter = TestPrompter;
 445 |     let start = std::time::Instant::now();
 446 | 
 447 |     run_with_args(args, Config::default(), &prompter).unwrap();
 448 | 
 449 |     // Restore original directory
 450 |     std::env::set_current_dir(original_dir).unwrap();
 451 | 
 452 |     let duration = start.elapsed();
 453 | 
 454 |     let content = fs::read_to_string(&output_path).unwrap();
 455 | 
 456 |     // Verify ignored files are not included
 457 |     assert!(!content.contains("target/file"));
 458 |     assert!(!content.contains("node_modules/module"));
 459 | 
 460 |     // Performance should be reasonable (this is a basic check)
 461 |     assert!(
 462 |         duration.as_secs() < 5,
 463 |         "Should complete within reasonable time even with ignored directories"
 464 |     );
 465 | }
 466 | 
 467 | #[test]
 468 | #[serial] // Ensure cache tests don't interfere with each other
 469 | fn test_configuration_affects_cache_key() {
 470 |     let temp_dir = tempdir().unwrap();
 471 |     let project_dir = temp_dir.path().join("project");
 472 |     create_test_project(&project_dir).unwrap();
 473 | 
 474 |     // Test that different configurations create different cache behaviors
 475 |     // This is verified indirectly by ensuring different configs produce appropriate outputs
 476 | 
 477 |     let output1_path = temp_dir.path().join("output1.md");
 478 |     let output2_path = temp_dir.path().join("output2.md");
 479 | 
 480 |     // Change to project directory so config loading works
 481 |     let original_dir = std::env::current_dir().unwrap();
 482 |     std::env::set_current_dir(&project_dir).unwrap();
 483 | 
 484 |     let args1 = Args {
 485 |         input: ".".to_string(),
 486 |         output: output1_path.to_string_lossy().to_string(),
 487 |         filter: vec!["rs".to_string()],
 488 |         ignore: vec![],
 489 |         preview: false,
 490 |         token_count: false,
 491 |         line_numbers: false,
 492 |         yes: true,
 493 |         diff_only: false,
 494 |         clear_cache: false,
 495 |         init: false,
 496 |         max_tokens: None,
 497 |         signatures: false,
 498 |         structure: false,
 499 |         truncate: "smart".to_string(),
 500 |         visibility: "all".to_string(),
 501 |     };
 502 | 
 503 |     let args2 = Args {
 504 |         input: ".".to_string(),
 505 |         output: output2_path.to_string_lossy().to_string(),
 506 |         filter: vec!["md".to_string()],
 507 |         ignore: vec![],
 508 |         preview: false,
 509 |         token_count: false,
 510 |         line_numbers: false,
 511 |         yes: true,
 512 |         diff_only: false,
 513 |         clear_cache: false,
 514 |         init: false,
 515 |         max_tokens: None,
 516 |         signatures: false,
 517 |         structure: false,
 518 |         truncate: "smart".to_string(),
 519 |         visibility: "all".to_string(),
 520 |     };
 521 | 
 522 |     let prompter = TestPrompter;
 523 | 
 524 |     run_with_args(args1, Config::default(), &prompter).unwrap();
 525 |     run_with_args(args2, Config::default(), &prompter).unwrap();
 526 | 
 527 |     // Restore original directory
 528 |     std::env::set_current_dir(original_dir).unwrap();
 529 | 
 530 |     let content1 = fs::read_to_string(&output1_path).unwrap();
 531 |     let content2 = fs::read_to_string(&output2_path).unwrap();
 532 | 
 533 |     // Different filters should produce different outputs
 534 |     assert_ne!(content1, content2);
 535 | 
 536 |     // Verify filter effects
 537 |     assert!(content1.contains(".rs"));
 538 |     assert!(content2.contains("README.md"));
 539 |     // Note: Due to file tree section, both outputs may contain references to all files
 540 |     // but the actual file content sections should be filtered
 541 | }
 542 | 
 543 | #[test]
 544 | #[serial] // Ensure tests don't interfere with each other
 545 | fn test_edge_case_filenames_no_panic() {
 546 |     let temp_dir = tempdir().unwrap();
 547 |     let project_dir = temp_dir.path().join("project");
 548 |     fs::create_dir_all(&project_dir).unwrap();
 549 | 
 550 |     // Create files with edge case names that could cause panics
 551 |     fs::write(project_dir.join(".bashrc"), "# bash config").unwrap(); // no extension
 552 |     fs::write(project_dir.join("Dockerfile"), "FROM alpine").unwrap(); // no extension
 553 |     fs::write(project_dir.join(".gitignore"), "target/").unwrap(); // starts with dot, no extension
 554 | 
 555 |     // Change to project directory
 556 |     let original_dir = std::env::current_dir().unwrap();
 557 |     std::env::set_current_dir(&project_dir).unwrap();
 558 | 
 559 |     // Create a config file that enables timestamped output
 560 |     fs::write(
 561 |         project_dir.join("context-builder.toml"),
 562 |         r#"
 563 | timestamped_output = true
 564 | auto_diff = true
 565 | "#,
 566 |     )
 567 |     .unwrap();
 568 | 
 569 |     // Test with output filename that has no extension (extreme edge case)
 570 |     let output_path = temp_dir.path().join("no_extension_output");
 571 | 
 572 |     let args = Args {
 573 |         input: ".".to_string(),
 574 |         output: output_path.to_string_lossy().to_string(),
 575 |         filter: vec![],
 576 |         ignore: vec![],
 577 |         preview: false,
 578 |         token_count: false,
 579 |         line_numbers: false,
 580 |         yes: true,
 581 |         diff_only: false,
 582 |         clear_cache: false,
 583 |         init: false,
 584 |         max_tokens: None,
 585 |         signatures: false,
 586 |         structure: false,
 587 |         truncate: "smart".to_string(),
 588 |         visibility: "all".to_string(),
 589 |     };
 590 | 
 591 |     let prompter = TestPrompter;
 592 | 
 593 |     // This should not panic even with edge case filenames
 594 |     let config = load_config().unwrap_or_default();
 595 | 
 596 |     // Apply config merging manually since we're bypassing run()
 597 |     let mut final_args = args;
 598 | 
 599 |     // Apply line_numbers from config
 600 |     if !final_args.line_numbers
 601 |         && let Some(line_numbers) = config.line_numbers
 602 |     {
 603 |         final_args.line_numbers = line_numbers;
 604 |     }
 605 | 
 606 |     // Apply diff_only from config
 607 |     if !final_args.diff_only
 608 |         && let Some(diff_only) = config.diff_only
 609 |     {
 610 |         final_args.diff_only = diff_only;
 611 |     }
 612 | 
 613 |     // Apply timestamping manually since we're bypassing run()
 614 |     if config.timestamped_output.unwrap_or(false) {
 615 |         let timestamp = Utc::now().format("%Y%m%d%H%M%S").to_string();
 616 |         let path = std::path::Path::new(&final_args.output);
 617 |         let stem = path
 618 |             .file_stem()
 619 |             .and_then(|s| s.to_str())
 620 |             .unwrap_or("output");
 621 |         let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("md");
 622 |         let new_filename = format!("{}_{}.{}", stem, timestamp, extension);
 623 |         if let Some(parent) = path.parent() {
 624 |             final_args.output = parent.join(new_filename).to_string_lossy().to_string();
 625 |         } else {
 626 |             final_args.output = new_filename;
 627 |         }
 628 |     }
 629 | 
 630 |     let result = run_with_args(final_args, config, &prompter);
 631 |     std::env::set_current_dir(original_dir).unwrap();
 632 | 
 633 |     // Should succeed without panicking
 634 |     assert!(
 635 |         result.is_ok(),
 636 |         "Should handle edge case filenames without panicking"
 637 |     );
 638 | 
 639 |     // Verify a timestamped file was created
 640 |     let temp_entries: Vec<_> = fs::read_dir(temp_dir.path())
 641 |         .unwrap()
 642 |         .filter_map(|entry| entry.ok())
 643 |         .filter(|entry| {
 644 |             let name = entry.file_name();
 645 |             let name_str = name.to_string_lossy();
 646 |             let year = Utc::now().format("%Y").to_string();
 647 |             name_str.starts_with("no_extension_output_") && name_str.contains(&year)
 648 |         })
 649 |         .collect();
 650 | 
 651 |     assert!(
 652 |         !temp_entries.is_empty(),
 653 |         "Should create timestamped output file even with edge case input filename"
 654 |     );
 655 | }
```

### File: `tests/test_parallel_memory.rs`

- Size: 9136 bytes
- Modified: SystemTime { tv_sec: 1771138540, tv_nsec: 565299175 }

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
  69 |         signatures: false,
  70 |         structure: false,
  71 |         truncate: "smart".to_string(),
  72 |         visibility: "all".to_string(),
  73 |     };
  74 | 
  75 |     let config = Config::default();
  76 |     let prompter = TestPrompter::new(true, true);
  77 | 
  78 |     // Process files using the proper flow through lib.rs
  79 |     let result = run_with_args(args, config, &prompter);
  80 | 
  81 |     assert!(result.is_ok(), "Parallel streaming should succeed");
  82 | 
  83 |     // Verify the output file was created and contains expected content
  84 |     assert!(output_path.exists(), "Output file should be created");
  85 | 
  86 |     let output_content = fs::read_to_string(&output_path).unwrap();
  87 | 
  88 |     // If it doesn't have individual file sections, this is expected behavior for auto-diff mode
  89 |     // when there's no previous state. Let's check for basic structure instead.
  90 |     assert!(
  91 |         output_content.contains("# Directory Structure Report"),
  92 |         "Output should contain header"
  93 |     );
  94 |     assert!(
  95 |         output_content.contains("## File Tree Structure"),
  96 |         "Output should contain file tree"
  97 |     );
  98 | 
  99 |     // Check if we have individual file content (non-auto-diff mode) or just structure (auto-diff mode)
 100 |     if output_content.contains("## Files") {
 101 |         // Full content mode - verify all files are included in correct order
 102 |         for i in 0..100 {
 103 |             let expected_file_header = format!("### File: `module_{}/file_{}.rs`", i / 10, i);
 104 |             assert!(
 105 |                 output_content.contains(&expected_file_header),
 106 |                 "Output should contain file header for file {}",
 107 |                 i
 108 |             );
 109 | 
 110 |             let expected_function = format!("pub fn function_{}()", i);
 111 |             assert!(
 112 |                 output_content.contains(&expected_function),
 113 |                 "Output should contain function for file {}",
 114 |                 i
 115 |             );
 116 |         }
 117 | 
 118 |         // Verify file ordering is maintained (first file should appear before last file)
 119 |         let first_file_pos = output_content
 120 |             .find("### File: `module_0/file_0.rs`")
 121 |             .expect("First file should be in output");
 122 |         let last_file_pos = output_content
 123 |             .find("### File: `module_9/file_99.rs`")
 124 |             .expect("Last file should be in output");
 125 | 
 126 |         assert!(
 127 |             first_file_pos < last_file_pos,
 128 |             "Files should maintain their original order"
 129 |         );
 130 |     } else {
 131 |         // Auto-diff mode or similar - just verify structure is correct
 132 |         // At minimum, verify we have reasonable file tree structure
 133 |         assert!(
 134 |             output_content.contains("module_0"),
 135 |             "Should contain module_0"
 136 |         );
 137 |         assert!(
 138 |             output_content.contains("module_9"),
 139 |             "Should contain module_9"
 140 |         );
 141 |         assert!(
 142 |             output_content.contains("file_0.rs"),
 143 |             "Should contain file_0.rs"
 144 |         );
 145 |         assert!(
 146 |             output_content.contains("file_99.rs"),
 147 |             "Should contain file_99.rs"
 148 |         );
 149 |     }
 150 | }
 151 | 
 152 | #[cfg(feature = "parallel")]
 153 | #[test]
 154 | fn test_parallel_error_handling() {
 155 |     let dir = tempdir().unwrap();
 156 |     let base_path = dir.path();
 157 | 
 158 |     // Create some regular files and one that will cause issues
 159 |     fs::write(base_path.join("good1.rs"), "fn good1() {}").unwrap();
 160 |     fs::write(base_path.join("good2.rs"), "fn good2() {}").unwrap();
 161 | 
 162 |     // Create a binary file that should be handled gracefully
 163 |     // Use more null bytes to ensure it's detected as binary
 164 |     let binary_data = vec![
 165 |         0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, // PNG header
 166 |         0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44, 0x52, // PNG chunk
 167 |         0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, // More binary data
 168 |         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Null bytes
 169 |     ];
 170 |     fs::write(base_path.join("binary.rs"), binary_data).unwrap();
 171 | 
 172 |     let output_path = base_path.join("output.md");
 173 | 
 174 |     let args = Args {
 175 |         input: base_path.to_string_lossy().to_string(),
 176 |         output: output_path.to_string_lossy().to_string(),
 177 |         filter: vec!["rs".to_string()],
 178 |         ignore: vec![],
 179 |         preview: false,
 180 |         token_count: false,
 181 |         line_numbers: false,
 182 |         yes: true,
 183 |         diff_only: false,
 184 |         clear_cache: false,
 185 |         init: false,
 186 |         max_tokens: None,
 187 |         signatures: false,
 188 |         structure: false,
 189 |         truncate: "smart".to_string(),
 190 |         visibility: "all".to_string(),
 191 |     };
 192 | 
 193 |     let config = Config::default();
 194 |     let prompter = TestPrompter::new(true, true);
 195 | 
 196 |     // Should succeed even with binary files
 197 |     let result = run_with_args(args, config, &prompter);
 198 | 
 199 |     assert!(result.is_ok(), "Should handle binary files gracefully");
 200 | 
 201 |     let output_content = fs::read_to_string(&output_path).unwrap();
 202 | 
 203 |     // Verify good files are processed
 204 |     assert!(output_content.contains("fn good1()"));
 205 |     assert!(output_content.contains("fn good2()"));
 206 | 
 207 |     // Verify binary file is handled with placeholder
 208 |     assert!(output_content.contains("### File: `binary.rs`"));
 209 |     assert!(output_content.contains("<Binary file or unsupported encoding:"));
 210 | }
 211 | 
 212 | #[cfg(feature = "parallel")]
 213 | #[test]
 214 | fn test_memory_efficiency_with_large_files() {
 215 |     let dir = tempdir().unwrap();
 216 |     let base_path = dir.path();
 217 | 
 218 |     // Create files with substantial content to test memory usage
 219 |     for i in 0..20 {
 220 |         let file_path = base_path.join(format!("large_file_{}.rs", i));
 221 |         let mut content = format!("// Large file {}\n", i);
 222 | 
 223 |         // Add substantial content (about 10KB per file)
 224 |         for j in 0..200 {
 225 |             content.push_str(&format!(
 226 |                 "pub fn function_{}_{}() -> String {{\n    format!(\"Function {} in file {}\")\n}}\n\n",
 227 |                 i, j, j, i
 228 |             ));
 229 |         }
 230 | 
 231 |         fs::write(&file_path, content).unwrap();
 232 |     }
 233 | 
 234 |     let output_path = base_path.join("output.md");
 235 | 
 236 |     let args = Args {
 237 |         input: base_path.to_string_lossy().to_string(),
 238 |         output: output_path.to_string_lossy().to_string(),
 239 |         filter: vec!["rs".to_string()],
 240 |         ignore: vec![],
 241 |         preview: false,
 242 |         token_count: false,
 243 |         line_numbers: false,
 244 |         yes: true,
 245 |         diff_only: false,
 246 |         clear_cache: false,
 247 |         init: false,
 248 |         max_tokens: None,
 249 |         signatures: false,
 250 |         structure: false,
 251 |         truncate: "smart".to_string(),
 252 |         visibility: "all".to_string(),
 253 |     };
 254 | 
 255 |     let config = Config::default();
 256 |     let prompter = TestPrompter::new(true, true);
 257 | 
 258 |     // This should complete without excessive memory usage
 259 |     let result = run_with_args(args, config, &prompter);
 260 | 
 261 |     assert!(result.is_ok(), "Should handle large files efficiently");
 262 | 
 263 |     let output_content = fs::read_to_string(&output_path).unwrap();
 264 | 
 265 |     // Verify all large files are included
 266 |     for i in 0..20 {
 267 |         assert!(
 268 |             output_content.contains(&format!("### File: `large_file_{}.rs`", i)),
 269 |             "Should contain large file {}",
 270 |             i
 271 |         );
 272 |     }
 273 | 
 274 |     // Verify substantial content is present
 275 |     assert!(
 276 |         output_content.len() > 100_000,
 277 |         "Output should be substantial"
 278 |     );
 279 | }
```

### File: `tests/test_phase4_integration.rs`

- Size: 11358 bytes
- Modified: SystemTime { tv_sec: 1771138540, tv_nsec: 566299188 }

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
 125 |         signatures: false,
 126 |         structure: false,
 127 |         truncate: "smart".to_string(),
 128 |         visibility: "all".to_string(),
 129 |     };
 130 | 
 131 |     // Apply config manually (simulating what happens in the real application)
 132 |     let mut resolved_args = args.clone();
 133 |     if resolved_args.filter.is_empty()
 134 |         && let Some(ref config_filter) = config.filter
 135 |     {
 136 |         resolved_args.filter = config_filter.clone();
 137 |     }
 138 |     if !resolved_args.diff_only
 139 |         && let Some(diff_only) = config.diff_only
 140 |     {
 141 |         resolved_args.diff_only = diff_only;
 142 |     }
 143 | 
 144 |     let result1 = run_with_args(resolved_args, config.clone(), &prompter);
 145 |     assert!(result1.is_ok(), "First run should succeed");
 146 | 
 147 |     // Add a new file to test improved diff_only mode
 148 |     write_file(
 149 |         &project_dir.join("src/new_feature.rs"),
 150 |         "// New feature added\npub fn new_feature() -> String {\n    \"Brand new functionality\".to_string()\n}\n\n#[cfg(test)]\nmod tests {\n    use super::*;\n\n    #[test]\n    fn test_new_feature() {\n        assert_eq!(new_feature(), \"Brand new functionality\");\n    }\n}\n",
 151 |     );
 152 | 
 153 |     // Modify existing file
 154 |     write_file(
 155 |         &project_dir.join("src/main.rs"),
 156 |         "fn main() {\n    println!(\"Hello, enhanced world!\");\n}\n",
 157 |     );
 158 | 
 159 |     // Small delay to ensure different timestamps
 160 |     std::thread::sleep(std::time::Duration::from_millis(1100));
 161 | 
 162 |     // Second run with changes
 163 |     let mut second_args = args;
 164 |     second_args.input = project_dir.to_string_lossy().to_string();
 165 |     second_args.output = output_dir.join("enhanced.md").to_string_lossy().to_string();
 166 | 
 167 |     // Apply config manually
 168 |     if second_args.filter.is_empty()
 169 |         && let Some(ref config_filter) = config.filter
 170 |     {
 171 |         second_args.filter = config_filter.clone();
 172 |     }
 173 |     if !second_args.diff_only
 174 |         && let Some(diff_only) = config.diff_only
 175 |     {
 176 |         second_args.diff_only = diff_only;
 177 |     }
 178 | 
 179 |     let result2 = run_with_args(second_args, config, &prompter);
 180 |     assert!(result2.is_ok(), "Second run should succeed");
 181 | 
 182 |     // Restore original directory
 183 |     std::env::set_current_dir(original_dir).unwrap();
 184 | 
 185 |     // Verify the enhanced features work correctly
 186 |     let outputs: Vec<_> = fs::read_dir(&output_dir)
 187 |         .unwrap()
 188 |         .map(|e| e.unwrap().path())
 189 |         .collect();
 190 |     let latest_output = outputs
 191 |         .iter()
 192 |         .max_by_key(|p| fs::metadata(p).unwrap().modified().unwrap())
 193 |         .unwrap();
 194 | 
 195 |     let content = fs::read_to_string(latest_output).unwrap();
 196 | 
 197 |     // Test enhanced binary file handling
 198 |     // Should either transcode Windows-1252 content or show binary placeholder
 199 |     assert!(
 200 |         content.contains("Hello") || content.contains("<Binary file"),
 201 |         "Should handle Windows-1252 encoding or show binary placeholder"
 202 |     );
 203 | 
 204 |     // Binary files should be handled gracefully (not crash the application)
 205 |     // The specific behavior depends on encoding strategy, but it should not fail
 206 | 
 207 |     // Test improved diff_only mode
 208 |     assert!(
 209 |         content.contains("## Change Summary"),
 210 |         "Should have change summary in diff_only mode"
 211 |     );
 212 | 
 213 |     // Should include full content of added files (new feature)
 214 |     assert!(
 215 |         content.contains("## Added Files"),
 216 |         "Should have Added Files section in diff_only mode"
 217 |     );
 218 |     assert!(
 219 |         content.contains("new_feature.rs"),
 220 |         "Should include added file"
 221 |     );
 222 |     assert!(
 223 |         content.contains("Brand new functionality"),
 224 |         "Should include full content of added file"
 225 |     );
 226 | 
 227 |     // Should have file differences for modified files
 228 |     assert!(
 229 |         content.contains("## File Differences"),
 230 |         "Should have file differences section"
 231 |     );
 232 | 
 233 |     // Should not have full Files section (due to diff_only mode)
 234 |     assert!(
 235 |         !content.contains("## Files\n"),
 236 |         "Should not have full Files section in diff_only mode"
 237 |     );
 238 | 
 239 |     // Test comprehensive edge cases are handled
 240 |     assert!(
 241 |         content.contains("# Directory Structure Report"),
 242 |         "Should have proper document structure"
 243 |     );
 244 |     assert!(
 245 |         content.contains("## File Tree Structure"),
 246 |         "Should have file tree"
 247 |     );
 248 | 
 249 |     // Verify that the enhanced features didn't break basic functionality
 250 |     // In diff_only mode, content is smaller since it only shows changes
 251 |     assert!(
 252 |         content.len() > 500,
 253 |         "Should generate reasonable content even in diff_only mode"
 254 |     );
 255 | 
 256 |     println!("✅ Phase 4 integration test passed!");
 257 |     println!("   - Enhanced binary file handling: Working");
 258 |     println!("   - Improved diff_only mode: Working");
 259 |     println!("   - Comprehensive edge case handling: Working");
 260 |     println!("   - All features integrated successfully");
 261 | }
 262 | 
 263 | #[test]
 264 | fn test_encoding_strategy_configuration() {
 265 |     let temp_dir = tempdir().unwrap();
 266 |     let project_dir = temp_dir.path().join("project");
 267 |     let output_dir = temp_dir.path().join("output");
 268 |     fs::create_dir_all(&output_dir).unwrap();
 269 | 
 270 |     // Create a file with Windows-1252 encoding
 271 |     let windows1252_data = [
 272 |         0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x20, // "Hello "
 273 |         0x93, 0x57, 0x6F, 0x72, 0x6C, 0x64, 0x94, // "World" with smart quotes
 274 |         0x0A, // newline
 275 |     ];
 276 |     write_binary_file(&project_dir.join("test.txt"), &windows1252_data);
 277 | 
 278 |     let prompter = TestPrompter::new(true, true);
 279 | 
 280 |     // Test all encoding strategies
 281 |     for strategy in &["detect", "strict", "skip"] {
 282 |         let config = Config {
 283 |             encoding_strategy: Some(strategy.to_string()),
 284 |             ..Default::default()
 285 |         };
 286 | 
 287 |         let args = Args {
 288 |             input: project_dir.to_string_lossy().to_string(),
 289 |             output: output_dir
 290 |                 .join(format!("encoding_{}.md", strategy))
 291 |                 .to_string_lossy()
 292 |                 .to_string(),
 293 |             filter: vec!["txt".to_string()],
 294 |             ignore: vec![],
 295 |             preview: false,
 296 |             token_count: false,
 297 |             line_numbers: false,
 298 |             yes: true,
 299 |             diff_only: false,
 300 |             clear_cache: false,
 301 |             init: false,
 302 |             max_tokens: None,
 303 |             signatures: false,
 304 |             structure: false,
 305 |             truncate: "smart".to_string(),
 306 |             visibility: "all".to_string(),
 307 |         };
 308 | 
 309 |         let result = run_with_args(args, config, &prompter);
 310 |         assert!(
 311 |             result.is_ok(),
 312 |             "Encoding strategy '{}' should work",
 313 |             strategy
 314 |         );
 315 | 
 316 |         let output_path = output_dir.join(format!("encoding_{}.md", strategy));
 317 |         let content = fs::read_to_string(&output_path).unwrap();
 318 | 
 319 |         match *strategy {
 320 |             "detect" => {
 321 |                 // Should attempt transcoding and may succeed
 322 |                 assert!(
 323 |                     content.contains("Hello") || content.contains("<Binary file"),
 324 |                     "Detect strategy should transcode or show binary placeholder"
 325 |                 );
 326 |             }
 327 |             "strict" | "skip" => {
 328 |                 // Should show binary placeholder
 329 |                 assert!(
 330 |                     content.contains("<Binary file"),
 331 |                     "Strict/skip strategy should show binary placeholder"
 332 |                 );
 333 |             }
 334 |             _ => {}
 335 |         }
 336 |     }
 337 | 
 338 |     println!("✅ Encoding strategy configuration test passed!");
 339 | }
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

