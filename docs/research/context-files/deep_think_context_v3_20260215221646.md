# Directory Structure Report

**Project:** context-builder
**Generated:** 2026-02-15 22:16:46 UTC
**Filters:** rs, toml, md, sh, yml
**Ignored:** docs, target, .git, node_modules

## File Tree Structure

- 📄 AGENTS.md
- 📄 BENCHMARKS.md
- 📄 CHANGELOG.md
- 📄 Cargo.toml
- 📄 DEVELOPMENT.md
- 📄 README.md
- 📄 SKILL.md
- 📁 benches
  - 📄 context_bench.rs
- 📄 install.sh
- 📁 scripts
  - 📄 demo.sh
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

### File: `AGENTS.md`

- Size: 6816 bytes
- Modified: SystemTime { tv_sec: 1771053874, tv_nsec: 10700049 }

```markdown
# AGENTS.md - AI Agent Instructions

This file helps AI agents quickly understand and contribute to the Context Builder codebase.

## Project Overview

Context Builder is a **blazing-fast Rust CLI** for aggregating entire codebases into single, LLM-friendly markdown files. Published on [crates.io](https://crates.io/crates/context-builder) under MIT license.

**If this is your first time:** Read this file, then run `cargo run -- --help` to see all options.

---

## Tech Stack

| Technology | Usage |
|---|---|
| **Language** | Rust (Edition 2024) |
| **Build** | Cargo (no npm/bun/node) |
| **CLI** | `clap` (derive) |
| **Parallelism** | `rayon` (optional, default on) + `crossbeam-channel` |
| **Diffing** | `similar` (unified diffs) |
| **File traversal** | `ignore` crate (gitignore-aware) |
| **Token counting** | `tiktoken-rs` (`cl100k_base`) |
| **Caching** | JSON + `fs2` file locking |
| **Config** | TOML (`context-builder.toml`) |
| **Encoding** | `encoding_rs` (transcoding non-UTF-8) |
| **Logging** | `env_logger` |
| **Branch** | `master` (not `main`) |

---

## Project Structure

```
context-builder/
├── src/
│   ├── main.rs              # Entry point — calls lib::run()
│   ├── lib.rs               # Core orchestration, run_with_args(), Prompter trait, --init
│   ├── cli.rs               # Args struct via clap derive
│   ├── config.rs            # Config struct, TOML deserialization
│   ├── config_resolver.rs   # Merges CLI args + TOML config (CLI > config > defaults)
│   ├── file_utils.rs        # .gitignore-aware traversal, OverrideBuilder for custom ignores
│   ├── tree.rs              # BTreeMap file tree (deterministic ordering)
│   ├── state.rs             # ProjectState/FileState structured snapshots
│   ├── markdown.rs          # Streaming file renderer, binary detection, encoding, parallel
│   ├── cache.rs             # JSON-based caching with fs2 locking, old cache migration
│   ├── diff.rs              # Per-file unified diffs via similar
│   └── token_count.rs       # Real tokenization via tiktoken-rs (cl100k_base, lazy init)
├── tests/                   # 10 integration test files
├── benches/                 # Criterion benchmark suite
├── scripts/                 # generate_samples.rs (benchmark dataset generator)
├── context-builder.toml     # Project's own config file
├── Cargo.toml               # Crate metadata, dependencies, features
├── DEVELOPMENT.md           # Contributor guide
├── BENCHMARKS.md            # Performance benchmarking guide
├── CHANGELOG.md             # Release history
└── .github/workflows/ci.yml # CI: fmt, clippy, build, test, security audit (ubuntu/win/macos)
```

---

## Key Commands

```bash
# Build
cargo build

# Run
cargo run -- --help
cargo run -- -d . -o out.md -f rs -f toml
cargo run -- --preview        # File tree only, no output
cargo run -- --init           # Create config file with auto-detected filters

# Test (MUST use single thread — tests share CWD)
cargo test -- --test-threads=1

# Lint (must pass -D warnings)
cargo clippy --all-targets --all-features -- -D warnings

# Format
cargo fmt --all
```

---

## Key Design Patterns

1. **`Prompter` trait** — Abstracts user confirmation (overwrite/processing). Tests use `MockPrompter`/`TestPrompter`. Never add stdin reads in library code.

2. **Streaming writes** — `markdown.rs` processes files line-by-line for low memory. With `parallel` feature, uses crossbeam channels for concurrent processing.

3. **Structured state** — v0.5.0 replaced fragile text-based cache parsing with JSON `ProjectState` snapshots for reliable auto-diff.

4. **Deterministic output** — `BTreeMap` everywhere ensures identical output across runs.

5. **Config precedence** — CLI args > TOML config > defaults, with explicit detection in `config_resolver.rs`.

---

## Feature Flags

| Feature | Default | Purpose |
|---|---|---|
| `parallel` | ✅ | Rayon for parallel file processing |
| `samples-bin` | ❌ | Exposes `generate_samples` binary for benchmarking |

---

## Environment Variables

| Variable | Purpose |
|---|---|
| `CB_SILENT` | `"1"` suppresses user-facing prints (benchmarks set this) |
| `CB_BENCH_MEDIUM` | `"1"` enables heavier benchmark datasets |
| `CB_BENCH_DATASET_DIR` | External benchmark dataset root |
| `RUST_LOG` | Controls `env_logger` verbosity (e.g., `RUST_LOG=info`) |

---

## Code Style Guidelines

1. **Error handling** — Use `io::Result`. Prefer returning errors over panicking. `unwrap()`/`expect()` OK in tests, NOT in library code.
2. **Cross-platform** — Normalize path separators in tests for string comparisons.
3. **New CLI flags** — Add in `cli.rs`, update tests in same file, propagate through `run_with_args`.
4. **Language detection** — Keep simple and deterministic; add mappings in one place.
5. **Binary detection** — Lightweight: NUL byte check + UTF-8 validity.
6. **Logging** — Use `log::{info, warn, error}`. Let `env_logger` control emission.

---

## Test Organization

- **Unit tests**: Inline `#[cfg(test)]` modules in every source file
- **Integration tests** (10 files in `tests/`):
  - `test_auto_diff.rs` — Auto-diff workflow (largest test file)
  - `test_determinism.rs` — Output determinism verification
  - `test_config_resolution.rs` — CLI/config merge behavior
  - `test_cwd_independence.rs` — Path independence
  - `test_comprehensive_edge_cases.rs` — Edge cases
  - `cli_integration.rs` — End-to-end CLI tests
  - `test_binary_file_autodiff.rs`, `test_parallel_memory.rs`, `test_phase4_integration.rs`, `diff_integration.rs`
- **Benchmarks**: Criterion suite at `benches/context_bench.rs`

**Critical:** Tests MUST run with `--test-threads=1` (CI enforces this). Many tests use `set_current_dir()` which is process-global. Use `#[serial]` attribute where order matters.

---

## Known Hazards

- **Year in tests**: Watch for hardcoded year strings in timestamp assertions. Use dynamic `Utc::now().format("%Y")` instead.
- **CWD mutation**: Tests that `set_current_dir()` must restore the original directory in all code paths (including panics).
- **Config from CWD**: `load_config()` reads from CWD. `load_config_from_path()` reads from explicit root. Prefer the latter in tests.
- **Cache collisions**: Cache keys are project-path + config hash. Different configs = different cache files.

---

## Release Process

1. `cargo fmt --all && cargo clippy --all-targets --all-features -- -D warnings && cargo test -- --test-threads=1`
2. Bump `version` in `Cargo.toml`, add entry to `CHANGELOG.md`
3. `git commit -am "chore(release): vX.Y.Z" && git tag vX.Y.Z && git push && git push --tags`
4. `cargo publish`
```

### File: `CHANGELOG.md`

- Size: 12742 bytes
- Modified: SystemTime { tv_sec: 1771187035, tv_nsec: 131259223 }

```markdown
# Changelog

All notable changes to this project will be documented in this file.

## v0.8.2

- **Documentation**
  - Updated SKILL.md for v0.8.1+ with Security & Path Scoping section
  - Documented Tree-Sitter CLI flags (`--signatures`, `--structure`, `--visibility`, `--truncate`)
  - Added AST signatures and API surface review recipes

- **Test Coverage**
  - Extended unit test coverage across `config.rs`, `file_utils.rs`, `state.rs`, `markdown.rs`, and `lib.rs`
  - Added tests for file relevance categories, lock files, various source extensions, encoding handling, auto-diff workflows, and config hash consistency

## v0.8.1

- **Bug Fixes** (identified by Gemini Deep Think v6 code review — 11 confirmed bugs, 0 false positives)
  - Fixed cache hash desync — `cache.rs` was missing 4 fields (`signatures`, `structure`, `truncate`, `visibility`), causing stale cache hits when toggling tree-sitter flags
  - Fixed JavaScript arrow function body leak — `statement_block` is a child of `arrow_function`, not `variable_declarator`, causing full function bodies to leak into signature output
  - Fixed TypeScript arrow function handling — same root cause as JavaScript
  - Fixed Python decorator erasure — intercepting `decorated_definition` nodes now preserves `@decorator` lines in signatures
  - Fixed Python `is_method` for decorated methods — iterative 4-level parent walk replaces fragile 2-level check
  - Fixed Rust tuple struct erasure — added `ordered_field_declaration_list` to body kinds
  - Fixed C/C++ header file prototype extraction — added `declaration` node matching for `.h` files
  - Fixed C++ class inheritance dropped — applied byte-slicing to preserve `template<>` and `: public Base`
  - Fixed JS/TS exported arrow functions invisible — added `lexical_declaration` to export signature extraction
  - Added `.jsx` extension support for JavaScript

- **Dependency Updates**
  - Updated `tree-sitter` core: 0.24 → 0.26
  - Updated `tree-sitter-rust`: 0.23 → 0.24
  - Updated `tree-sitter-javascript`: 0.23 → 0.25
  - Updated `tree-sitter-python`: 0.23 → 0.25
  - Updated `tree-sitter-go`: 0.23 → 0.25
  - Updated `tree-sitter-c`: 0.23 → 0.24

## v0.8.0

- **Tree-Sitter AST Integration** (feature-gated)
  - New `--signatures` flag: Replaces full file content with extracted function/class signatures — dramatically reduces token usage (~4K vs ~15K tokens per file)
  - New `--structure` flag: Appends a structural summary to each file (e.g., "6 functions, 2 structs, 1 impl block")
  - New `--truncate smart` mode: Prefers AST-boundary truncation when content needs truncating
  - Supports 8 languages: Rust, JavaScript, TypeScript, Python, Go, Java, C, C++
  - Install with: `cargo install context-builder --features tree-sitter-all`
  - Individual language features available (e.g., `--features tree-sitter-rust`)

- **Dependency Updates**
  - Updated `tree-sitter` core: 0.22 → 0.24
  - Updated all grammar crates: 0.21 → 0.23
  - Migrated from deprecated `language()` functions to `LANGUAGE` constants API

- **Bug Fixes**
  - Fixed config hash mismatch — cache now includes `auto_diff` and `diff_context_lines` fields, preventing stale cache hits when toggling these options
  - Fixed silent config parse failure — `context-builder.toml` with invalid TOML syntax now prints a warning instead of silently falling back to defaults
  - Fixed smart truncation unconditionally cutting 50% of file content — now only activates with explicit token budget
  - Fixed Windows path separators in determinism test causing CI failure

- **CI & Quality**
  - Added Coveralls code coverage integration via `cargo-tarpaulin`
  - All 188+ tests passing across Ubuntu, macOS, and Windows

## v0.7.1

- **Bug Fixes** (identified by Gemini Deep Think multi-round code review)
  - Fixed content hash using absolute OS paths — now normalized to relative unix-style for cross-platform determinism
  - Fixed hash collision risk — added null byte delimiter between path and content in content hash
  - Fixed `strip_prefix('+')` leaving extra space in diff_only mode, corrupting indentation
  - Fixed auto_diff path bypassing `--max-tokens` budget entirely
  - Fixed `src/tests/` files misclassified as source code instead of tests
  - Fixed `sorted_paths` missing cwd fallback, silently dropping files when cwd ≠ base_path

- **Auto-Ignore Common Directories**
  - 19 heavy directories (node_modules, dist, build, __pycache__, .venv, vendor, etc.) are now excluded by default
  - Prevents million-line outputs when processing projects without a `.git` directory

- **Context Window Warnings**
  - Shows estimated token count after every run
  - Warns when output exceeds 128K tokens with actionable CLI suggestions

## v0.7.0

- **Deterministic Output**
  - Replaced volatile timestamp (`Processed at: <timestamp>`) with a content hash (`Content hash: <hex>`) in the Markdown header
  - Identical project states now produce byte-for-byte identical output files, enabling LLM prompt caching

- **Context Budgeting (`--max-tokens N`)**
  - New CLI argument `--max-tokens` and `context-builder.toml` config option to cap the output token budget
  - Files are processed until the budget is exhausted, with a `<truncated>` marker appended
  - Prevents API errors from excessively large contexts and reduces costs

- **Relevance-Based File Ordering**
  - Files are now sorted by relevance category: config files (0) → source code (1) → tests (2) → docs/other (3)
  - Within each category, files remain alphabetically sorted
  - Helps LLMs prioritize core logic and configuration over supporting files

## v0.6.1

- **Bug Fixes** (identified by Gemini Deep Think code review)
  - Fixed TOCTOU race in cache writes: `File::create` was truncating before acquiring lock, risking data loss for concurrent readers
  - Fixed indentation destruction in `diff_only` mode: `trim_start()` was stripping all leading whitespace from added files, corrupting Python/YAML
  - Fixed UTF-8 boundary corruption: 8KB sniff buffer could split multi-byte characters, misclassifying valid UTF-8 files as binary
  - Fixed CLI flags silently overwritten: config file values were unconditionally overriding CLI arguments post-resolution
  - Removed duplicate file seek block (copy-paste error)

## v0.6.0

- **Smart Defaults**
  - Auto-exclude output files: the tool now automatically excludes its own generated output file, output folder, and `.context-builder/` cache directory from context collection without requiring manual `--ignore` flags
  - Timestamped output glob patterns (e.g., `docs/context_*.md`) are auto-excluded when `timestamped_output` is enabled
  - Large-file detection: warns about files exceeding 100 KB with a sorted top-5 list and total context size summary
  - Improved project name detection: canonicalizes relative paths (like `.`) to resolve the actual directory name instead of showing "unknown"

- **Testing & Stability**
  - Added `#[serial]` annotations to integration tests that mutate CWD, fixing intermittent test failures in parallel execution
  - All 146 tests pass consistently with `--test-threads=1`

- **Dependencies**
  - Updated `criterion` to 0.8.2
  - Updated `tiktoken-rs` to 0.9.1
  - Updated `toml` to 1.0.1

## v0.5.2

- Enhanced `--init` command to detect major file types in the current directory and suggest appropriate filters instead of using generic defaults
- Fixed file type detection to respect .gitignore patterns and common ignore directories (target, node_modules, etc.)

## v0.5.1

- Added `--init` command to create a new `context-builder.toml` configuration file in the current directory with sensible defaults

## v0.5.0

- **BREAKING CHANGES**
  - Cache file locations changed to project-specific paths to prevent collisions

- **Critical Bug Fixes**
  - **Fixed inverted ignore logic**: Corrected critical bug where ignore patterns were being treated as include patterns, causing files/directories meant to be ignored to be explicitly included instead
  - **Fixed cache read panics**: Improved error handling for corrupted cache files to prevent application crashes
  - **Fixed potential panics in path manipulation**: Added safe handling for edge case filenames without extensions or stems

- **Major Improvements**
  - **Deterministic Output**: Files are now sorted consistently, ensuring identical output for the same input across multiple runs
  - **Robust Caching Architecture**: Complete rewrite of caching system with:
    - Project-specific cache keys based on absolute path hash to prevent collisions
    - JSON-based structured caching replacing fragile markdown parsing
    - File locking with `fs2` crate for thread-safe concurrent access
    - Configuration changes now properly invalidate cache
  - **Enhanced Auto-Diff System**:
    - Structured state representation before markdown generation
    - Eliminated fragile text parsing with `extract_file_contents` and `strip_line_number` functions
    - Cache structured data (JSON) instead of markdown for reliability
  - **Thread Safety**: Removed all `unsafe` blocks and explicit configuration passing replaces environment variables

- **Performance Optimizations**
  - **Custom Ignores**: Now uses `ignore::overrides::OverrideBuilder` with glob pattern support for better performance
  - **Parallel Processing**: Improved error handling to collect all errors and continue processing other files
  - **Directory Traversal**: Let `ignore` crate optimize directory traversal instead of custom logic

- **Bug Fixes**
  - Fixed non-deterministic output order that caused inconsistent LLM context generation
  - Removed incorrect triple-backtick filtering in diff logic that was corrupting file content
  - Fixed cache corruption issues in concurrent access scenarios
  - Improved error recovery for partial failures and corrupted cache
  - Fixed inconsistent file tree visualization between auto-diff and standard modes

- **Testing & Quality**
  - Added comprehensive integration test suite with tests covering:
    - Determinism verification
    - Auto-diff workflows
    - Cache collision prevention
    - Configuration change detection
    - Error recovery scenarios
  - Fixed test race conditions by running tests serially in CI (`--test-threads=1`)
  - Added `pretty_assertions` for better test output
  - Fixed all clippy warnings and enforced `-D warnings` in CI

- **Dependencies**
  - Added `fs2` for file locking
  - Added `serde_json` for structured cache format
  - Added `serial_test` for test serialization
  - Added `pretty_assertions` for enhanced test output
  - Added `encoding_rs` for enhanced encoding detection and transcoding

- **Migration**
  - Automatic detection and cleanup of old markdown-based cache files (`last_canonical.md`, etc.)
  - First run after upgrade will clear old cache format to prevent conflicts
  - CLI interface remains fully backward compatible

- **Code Quality & Maintenance**
  - Fixed all clippy warnings including type complexity, collapsible if statements, and redundant closures
  - Updated CI workflow to prevent race conditions in tests
  - Improved binary file detection with better encoding strategy handling
  - Enhanced error handling for edge cases and file system operations

## v0.4.0


- Added

  - Token count mode (`--token-count`) now provides accurate token counts using the `tiktoken-rs` library.

  - Configuration file support (`context-builder.toml`) for project-specific settings.

  - Timestamped output versions.

  - `auto_diff` feature to automatically generate a diff from the latest output.
  - `diff_only` mode (`--diff-only` / `diff_only = true`) to output only the change summary and modified file diffs (no full file bodies) for lower token usage.

- Removed
  - Deprecated, unpublished `standalone_snapshot` option (replaced by `diff_only`).


## v0.3.0

- Changed
  - Parallel processing is now enabled by default via the `parallel` feature (uses `rayon`) for significant speedups on large projects.
    - To build/run sequentially, disable default features:
      - CLI/build: `cargo build --no-default-features` or `cargo run --no-default-features`
      - As a dependency: `default-features = false`
  - Updated Rust edition to 2024.

- Benchmarks
  - Benchmarks run silent by default by setting `CB_SILENT=1` at startup to avoid skewing timings with console I/O.
    - Override with `CB_SILENT=0` if you want to see output during benches.

## v0.2.0

- Added line numbers support
- Improved file tree visualization
- Enhanced error handling
- Better CLI argument validation

## v0.1.0

- Initial release
- Basic directory processing
- File filtering and ignoring
- Markdown output generation
```

### File: `Cargo.toml`

- Size: 2832 bytes
- Modified: SystemTime { tv_sec: 1771187003, tv_nsec: 458908742 }

```toml
[package]
name = "context-builder"
version = "0.8.2"
default-run = "context-builder"
edition = "2024"
authors = ["Igor Lins e Silva"]
description = "CLI tool to aggregate directory contents into a single markdown file optimized for LLM consumption"
readme = "README.md"
homepage = "https://github.com/igorls/context-builder"
repository = "https://github.com/igorls/context-builder"
license = "MIT"
keywords = ["cli", "markdown", "documentation", "llm", "context"]
categories = ["command-line-utilities", "development-tools"]

[dependencies]
clap = { version = "4.5.58", features = ["derive"] }
chrono = { version = "0.4.43", features = ["serde"] }
ignore = "0.4.25"
log = "0.4.29"
env_logger = "0.11.9"
rayon = { version = "1.10", optional = true }
serde = { version = "1.0.228", features = ["derive"] }
toml = "1.0.1"
similar = "2.7.0"
tempfile = "3.25.0"
tiktoken-rs = "0.9.1"
once_cell = "1.21.3"
fs2 = "0.4.3"
serde_json = "1.0.143"
crossbeam-channel = "0.5.15"
num_cpus = "1.17.0"
encoding_rs = "0.8.35"
walkdir = "2.5.0"
xxhash-rust = { version = "0.8", features = ["xxh3"] }

# Tree-sitter dependencies (feature-gated)
tree-sitter = { version = "0.26", optional = true }
tree-sitter-rust = { version = "0.24", optional = true }
tree-sitter-javascript = { version = "0.25", optional = true }
tree-sitter-typescript = { version = "0.23", optional = true }
tree-sitter-python = { version = "0.25", optional = true }
tree-sitter-go = { version = "0.25", optional = true }
tree-sitter-java = { version = "0.23", optional = true }
tree-sitter-c = { version = "0.24", optional = true }
tree-sitter-cpp = { version = "0.23", optional = true }

[features]
default = ["parallel"]
parallel = ["rayon"]
samples-bin = []

# Tree-sitter features - language grammar support
tree-sitter-base = ["dep:tree-sitter"]
tree-sitter-rust = ["tree-sitter-base", "dep:tree-sitter-rust"]
tree-sitter-js = ["tree-sitter-base", "dep:tree-sitter-javascript"]
tree-sitter-ts = ["tree-sitter-base", "dep:tree-sitter-typescript"]
tree-sitter-python = ["tree-sitter-base", "dep:tree-sitter-python"]
tree-sitter-go = ["tree-sitter-base", "dep:tree-sitter-go"]
tree-sitter-java = ["tree-sitter-base", "dep:tree-sitter-java"]
tree-sitter-c = ["tree-sitter-base", "dep:tree-sitter-c"]
tree-sitter-cpp = ["tree-sitter-base", "dep:tree-sitter-cpp"]
tree-sitter-all = [
    "tree-sitter-rust",
    "tree-sitter-js",
    "tree-sitter-ts",
    "tree-sitter-python",
    "tree-sitter-go",
    "tree-sitter-java",
    "tree-sitter-c",
    "tree-sitter-cpp",
]

[dev-dependencies]
tempfile = "3.25.0"
criterion = { version = "0.8.2", features = ["html_reports"] }
pretty_assertions = "1.4.1"
serial_test = "3.0"

[[bench]]
name = "context_bench"
harness = false

[[bin]]
name = "generate_samples"
path = "scripts/generate_samples.rs"
required-features = ["samples-bin"]
```

### File: `README.md`

- Size: 11387 bytes
- Modified: SystemTime { tv_sec: 1771190999, tv_nsec: 364539648 }

```markdown
<div align="center">

# Context Builder

A blazing-fast CLI for creating LLM context from your entire codebase.

[![Crates.io](https://img.shields.io/crates/v/context-builder.svg)](https://crates.io/crates/context-builder)
![Crates.io Size](https://img.shields.io/crates/size/context-builder)
![Deps.rs Crate Dependencies (latest)](https://img.shields.io/deps-rs/context-builder/latest)
![Crates.io Total Downloads](https://img.shields.io/crates/d/context-builder)

</div>

<div align="center">

[![Coverage Status](https://coveralls.io/repos/github/igorls/context-builder/badge.svg?branch=master)](https://coveralls.io/github/igorls/context-builder?branch=master)
[![CI](https://github.com/igorls/context-builder/actions/workflows/ci.yml/badge.svg)](https://github.com/igorls/context-builder/actions/workflows/ci.yml)
![docs.rs](https://img.shields.io/docsrs/context-builder)

</div>

<div align="center">

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/igorls/context-builder/blob/main/LICENSE)

</div>

<br/>

Tired of manually copy-pasting files into your LLM prompts? Context Builder automates this tedious process, creating a single, clean, and context-rich markdown file from any directory.

<div align="center">

![demo](docs/demo.gif)

</div>

---

## Why Context Builder?

Providing broad context to Large Language Models (LLMs) is key to getting high-quality, relevant responses. This tool was built to solve one problem exceptionally well: **packaging your project's source code into a clean, LLM-friendly format with zero fuss.**

It's a command-line utility that recursively processes directories and creates comprehensive markdown documentation, optimized for AI conversations.

## Core Features


- ⚡ **Blazing Fast & Parallel by Default:**
  Processes thousands of files in seconds by leveraging all available CPU cores.

- 🧠 **Smart & Efficient File Discovery:**
  Respects `.gitignore` and custom ignore patterns out-of-the-box using optimized, parallel directory traversal. Automatically excludes common heavy directories (`node_modules`, `dist`, `build`, `__pycache__`, `.venv`, `vendor`, etc.) even without a `.git` directory.

- 📊 **Relevance-Based File Ordering:**
  Files appear in LLM-optimized order: config & project docs first, then source code (entry points before helpers), tests, documentation, build/CI files, and lockfiles last. This helps LLMs build a mental model faster.

- 💰 **Context Budgeting (`--max-tokens`):**
  Cap token output to fit your model's context window. Warns when output exceeds 128K tokens with actionable suggestions.

- 💾 **Memory-Efficient Streaming:**
  Handles massive files with ease by reading and writing line-by-line, keeping memory usage low.

- 🌳 **Clear File Tree Visualization:**
  Generates an easy-to-read directory structure at the top of the output file.

- 🔍 **Powerful Filtering & Preview:**
  Easily include only the file extensions you need and use the instant `--preview` mode to see what will be processed.

 - ⚙️ **Configuration-First:**
  Use a `context-builder.toml` file to store your preferences for consistent, repeatable outputs. Initialize a new config file with `--init`, which will detect the major file types in your project (respecting `.gitignore` patterns) and suggest appropriate filters.

- 🔁 **Automatic Per-File Diffs:**
  When enabled, automatically generates a clean, noise-reduced diff showing what changed between snapshots.

- ✂️ **Diff-Only Mode:**
  Output only the change summary and modified file diffs—no full file bodies—to minimize token usage.

- 🌲 **Tree-Sitter AST Analysis** *(optional)*:
  Extract function/class signatures (`--signatures`), structural summaries (`--structure`), and smart AST-boundary truncation (`--truncate smart`). Supports Rust, JavaScript, TypeScript, Python, Go, Java, C, and C++.

- 🧪 **Accurate Token Counting:**
  Get real tokenizer–based estimates with `--token-count` to plan your prompt budgets.


---

## Installation

### Quick Install (Linux/macOS)

Pre-built binaries include full Tree-Sitter AST support.

```bash
curl -sSL https://raw.githubusercontent.com/igorls/context-builder/master/install.sh | sh
```

### Windows (PowerShell)

```powershell
Invoke-WebRequest -Uri "https://github.com/igorls/context-builder/releases/latest/download/context-builder-x86_64-pc-windows-msvc.zip" -OutFile "$env:TEMP\cb.zip"
Expand-Archive "$env:TEMP\cb.zip" -DestinationPath "$env:LOCALAPPDATA\Programs\context-builder" -Force
$env:PATH += ";$env:LOCALAPPDATA\Programs\context-builder"
```

> Add `%LOCALAPPDATA%\Programs\context-builder` to your PATH permanently via System Settings.

### From crates.io

```bash
cargo install context-builder --features tree-sitter-all
```

### From source

```bash
git clone https://github.com/igorls/context-builder.git
cd context-builder
cargo install --path .
```

---

## Usage

### Basic Usage

```bash
# Initialize a new context-builder.toml config file with automatically detected file types (respecting .gitignore)
context-builder --init

# Process current directory and create output.md
context-builder

# Process a specific directory
context-builder -d /path/to/project

# Specify an output file
context-builder -d /path/to/project -o documentation.md
```

### Advanced Options

```bash
# Filter by file extensions (e.g., only Rust and TOML files)
context-builder -f rs -f toml

# Ignore specific folders/files by name
context-builder -i target -i node_modules -i .git

# Cap output to a token budget (prevents context overflow)
context-builder --max-tokens 100000

# Preview mode (shows the file tree without generating output)
context-builder --preview

# Token count mode (accurately count the total token count of the final document using a real tokenizer.)
context-builder --token-count

# Add line numbers to all code blocks
context-builder --line-numbers

# Skip all confirmation prompts (auto-answer yes)
context-builder --yes

# Output only diffs (requires auto-diff & timestamped output)
context-builder --diff-only


# Clear cached project state (resets auto-diff baseline & removes stored state)

context-builder --clear-cache

# Combine multiple options for a powerful workflow
context-builder -d ./src -f rs -f toml -i tests --line-numbers --max-tokens 100000 -o rust_context.md
```

---

## Configuration

For more complex projects, you can use a `context-builder.toml` file in your project's root directory to store your preferences. This is great for ensuring consistent outputs and avoiding repetitive command-line flags.

### Example `context-builder.toml`

```toml
# Default output file name
output = "context.md"

# Default output folder
output_folder = "docs/context"

# Create timestamped versions of the output file (e.g., context_20250912123000.md)
timestamped_output = true

# Automatically compute per-file diffs against the previous timestamped snapshot
auto_diff = true

# Emit only change summary + modified file diffs (omit full file bodies)
# Set to true to greatly reduce token usage when you just need what's changed.
diff_only = false

# Number of context lines to show around changes in diffs (default: 3)
diff_context_lines = 5

# File extensions to include
filter = ["rs", "toml", "md"]

# Folders or file names to ignore
ignore = ["target", "node_modules", ".git"]

# Add line numbers to code blocks
line_numbers = true

# Preview mode: only show file tree without generating output
preview = false

# Token counting mode
token_count = false


# Automatically answer yes to all prompts

yes = false



# Encoding handling strategy for non-UTF-8 files

# Options: "detect" (default), "strict", "skip"

encoding_strategy = "detect"

```



 You can initialize a new configuration file using the `--init` command. This will create a `context-builder.toml` file in your current directory with sensible defaults based on the file types detected in your project. The filter suggestions will be automatically tailored to your project's most common file extensions while respecting `.gitignore` patterns and common ignore directories like `target`, `node_modules`, etc. This makes it more likely to include the files you actually want to process.



---

## Auto-diff

When using `timestamped_output = true` together with `auto_diff = true`, Context Builder compares the previous canonical snapshot to the newly generated one and produces:

- A Change Summary (Added / Removed / Modified files)
- A File Differences section containing only modified files (added & removed are summarized but not diffed)

If you also set `diff_only = true` (or pass `--diff-only`), the full “## Files” section is omitted to conserve tokens: you get just the header + tree, the Change Summary, and per-file diffs for modified files.

**Note:** Command-line arguments will always override the settings in the configuration file.

### Command Line Options

- `-d, --input <PATH>` - Directory path to process (default: current directory).
- `-o, --output <FILE>` - Output file path (default: `output.md`).
- `-f, --filter <EXT>` - File extensions to include (can be used multiple times).
- `-i, --ignore <NAME>` - Folder or file names to ignore (can be used multiple times).
- `--max-tokens <N>` - Maximum token budget for the output. Files are truncated/skipped when exceeded.
- `--preview` - Preview mode: only show the file tree, don't generate output.
- `--token-count` - Token count mode: accurately count the total token count of the final document using a real tokenizer.
- `--line-numbers` - Add line numbers to code blocks in the output.
- `-y, --yes` - Automatically answer yes to all prompts (skip confirmation dialogs).
- `--diff-only` - With auto-diff + timestamped output, output only change summary + modified file diffs (omit full file bodies).
- `--clear-cache` - Remove stored state used for auto-diff; next run becomes a fresh baseline.
- `--signatures` - Replace full file content with extracted function/class signatures *(requires tree-sitter)*.
- `--structure` - Append structural summary (function/class counts) to each file *(requires tree-sitter)*.
- `--truncate <MODE>` - Truncation strategy: `none` (default) or `smart` (AST-boundary aware) *(requires tree-sitter)*.
- `--init` - Initialize a new `context-builder.toml` config file.
- `-h, --help` - Show help information.
---

## Token Counting

Context Builder uses the `tiktoken-rs` library to provide accurate token counts for OpenAI models. This ensures that the token count is as close as possible to the actual number of tokens that will be used by the model.

---

## Documentation

- **[DEVELOPMENT.md](DEVELOPMENT.md):** For contributors. Covers setup, testing, linting, and release process.
- **[BENCHMARKS.md](BENCHMARKS.md):** For performance enthusiasts. Details on running benchmarks and generating datasets.
- **[CHANGELOG.md](CHANGELOG.md):** A complete history of releases and changes.

## Contributing

Contributions are welcome! Please see **[DEVELOPMENT.md](DEVELOPMENT.md)** for setup instructions and guidelines. For major changes, please open an issue first to discuss what you would like to change.

## Changelog

See **[CHANGELOG.md](CHANGELOG.md)** for a complete history of releases and changes.

## License

This project is licensed under the MIT License. See the **[LICENSE](LICENSE)** file for details.
```

### File: `src/lib.rs`

- Size: 83953 bytes
- Modified: SystemTime { tv_sec: 1771189128, tv_nsec: 576359466 }


**Signatures:**

```rust
// Modules
pub mod cache
pub mod cli
pub mod config
pub mod config_resolver
pub mod diff
pub mod file_utils
pub mod markdown
pub mod state
pub mod token_count
pub mod tree
pub mod tree_sitter

// Structs/Classes
pub struct DiffConfig

// Implementations
impl Default for DiffConfig

// Functions
fn default() -> Self

// Traits/Interfaces
pub trait Prompter

// Structs/Classes
pub struct DefaultPrompter

// Implementations
impl Prompter for DefaultPrompter

// Functions
fn confirm_processing(&self, file_count: usize) -> io::Result<bool>
fn confirm_overwrite(&self, file_path: &str) -> io::Result<bool>
pub fn run_with_args(args: Args, config: Config, prompter: &impl Prompter) -> io::Result<()>

// Constants
const LARGE_FILE_THRESHOLD

// Functions
fn print_context_window_warning(output_bytes: usize, max_tokens: Option<usize>)

// Constants
const RECOMMENDED_LIMIT

// Functions
fn generate_markdown_with_diff(
    current_state: &ProjectState,
    comparison: Option<&StateComparison>,
    args: &Args,
    file_tree: &tree::FileTree,
    diff_config: &DiffConfig,
    sorted_paths: &[PathBuf],
    ts_config: &markdown::TreeSitterConfig,
) -> io::Result<String>
pub fn run() -> io::Result<()>
fn detect_major_file_types() -> io::Result<Vec<String>>
fn init_config() -> io::Result<()>

// Modules
mod tests

// Structs/Classes
struct MockPrompter

// Implementations
impl MockPrompter

// Functions
fn new(processing: bool, overwrite: bool) -> Self

// Implementations
impl Prompter for MockPrompter

// Functions
fn confirm_processing(&self, _file_count: usize) -> Result<bool>
fn confirm_overwrite(&self, _file_path: &str) -> Result<bool>
fn test_diff_config_default()
fn test_diff_config_custom()
fn test_default_prompter()
fn test_run_with_args_nonexistent_directory()
fn test_run_with_args_preview_mode()
fn test_run_with_args_token_count_mode()
fn test_run_with_args_preview_and_token_count()
fn test_run_with_args_user_cancels_overwrite()
fn test_run_with_args_user_cancels_processing()
fn test_run_with_args_with_yes_flag()
fn test_run_with_args_with_filters()
fn test_run_with_args_with_ignores()
fn test_auto_diff_without_previous_state()
fn test_run_creates_output_directory()
fn test_generate_markdown_with_diff_no_comparison()
fn test_context_window_warning_under_limit()
fn test_context_window_warning_over_limit()
fn test_context_window_warning_with_max_tokens()
fn test_print_context_window_warning_various_sizes()
fn test_run_with_args_large_file_warning()
fn test_run_with_args_output_dir_creation_failure_is_handled()
fn test_auto_diff_cache_write_failure_handling()
fn test_auto_diff_with_changes()
fn test_auto_diff_max_tokens_truncation()
fn test_diff_only_mode_with_added_files()
fn test_generate_markdown_with_diff_line_numbers()
fn test_generate_markdown_with_diff_and_modifications()
fn test_detect_major_file_types()
fn test_init_config_already_exists()
fn test_init_config_creates_new_file()
fn test_detect_major_file_types_empty_dir()
fn test_print_context_window_warning_exact_limit()
fn test_run_with_args_with_existing_output_file()
fn test_run_with_args_preview_only_token_count()
fn test_run_with_args_multiple_files()
fn test_auto_diff_config_hash_change()
fn test_generate_markdown_with_diff_and_filters()
fn test_generate_markdown_with_diff_and_ignores()
```

### File: `src/main.rs`

- Size: 73 bytes
- Modified: SystemTime { tv_sec: 1771053288, tv_nsec: 65557165 }


**Signatures:**

```rust
// Functions
fn main() -> io::Result<()>
```

### File: `src/tree_sitter/languages/mod.rs`

- Size: 3447 bytes
- Modified: SystemTime { tv_sec: 1771138025, tv_nsec: 375473793 }


**Signatures:**

```rust
// Modules
mod rust
mod javascript
mod typescript
mod python
mod go
mod java
mod c
mod cpp

// Functions
pub fn get_language_support(ext: &str) -> Option<&'static dyn LanguageSupport>
pub fn get_language_support(_ext: &str) -> Option<()>
pub fn supported_extensions() -> Vec<&'static str>
pub fn supported_extensions() -> Vec<&'static str>
```

### File: `src/tree_sitter/mod.rs`

- Size: 7458 bytes
- Modified: SystemTime { tv_sec: 1771188412, tv_nsec: 85574162 }


**Signatures:**

```rust
// Modules
pub mod language_support
pub mod signatures
pub mod structure
pub mod truncation
pub mod languages

// Functions
pub fn is_supported_extension(ext: &str) -> bool
pub fn is_supported_extension(_ext: &str) -> bool
fn get_extension(path: &Path) -> Option<String>
pub fn get_language_for_path(path: &Path) -> Option<&'static dyn LanguageSupport>
pub fn extract_signatures_for_file(
    source: &str,
    ext: &str,
    visibility_filter: Visibility,
) -> Option<Vec<Signature>>
pub fn extract_structure_for_file(source: &str, ext: &str) -> Option<CodeStructure>
pub fn find_smart_truncation_point(source: &str, max_bytes: usize, ext: &str) -> Option<usize>
pub fn extract_signatures_for_file(
    _source: &str,
    _ext: &str,
    _visibility_filter: (),
) -> Option<()>
pub fn extract_structure_for_file(_source: &str, _ext: &str) -> Option<()>
pub fn find_smart_truncation_point(_source: &str, _max_bytes: usize, _ext: &str) -> Option<usize>
pub fn get_language_for_path(_path: &std::path::Path) -> Option<()>

// Modules
mod tests

// Functions
fn test_is_supported_extension()
fn test_no_tree_sitter_support()
fn test_get_extension()
fn test_extract_signatures_for_file_rust()
fn test_extract_signatures_for_file_unsupported()
fn test_extract_structure_for_file_rust()
fn test_extract_structure_for_file_unsupported()
fn test_find_smart_truncation_point_within_bounds()
fn test_find_smart_truncation_point_truncated()
fn test_find_smart_truncation_point_unsupported()
fn test_get_language_for_path_known()
fn test_get_language_for_path_unknown()
fn test_get_language_for_path_no_extension()
```

### File: `install.sh`

- Size: 2606 bytes
- Modified: SystemTime { tv_sec: 1771193284, tv_nsec: 825237670 }

```sh
#!/bin/sh
# Context Builder Installer
# Usage: curl -sSL https://raw.githubusercontent.com/igorls/context-builder/master/install.sh | sh
set -e

REPO="igorls/context-builder"
INSTALL_DIR="/usr/local/bin"

# Detect OS and architecture
OS="$(uname -s)"
ARCH="$(uname -m)"

case "$OS" in
  Linux)   TARGET_OS="unknown-linux-gnu" ;;
  Darwin)  TARGET_OS="apple-darwin" ;;
  *)       echo "Error: Unsupported OS '$OS'. Please install manually from:"; echo "  https://github.com/$REPO/releases/latest"; exit 1 ;;
esac

case "$ARCH" in
  x86_64|amd64)  TARGET_ARCH="x86_64" ;;
  arm64|aarch64) TARGET_ARCH="aarch64" ;;
  *)             echo "Error: Unsupported architecture '$ARCH'. Please install manually from:"; echo "  https://github.com/$REPO/releases/latest"; exit 1 ;;
esac

TARGET="${TARGET_ARCH}-${TARGET_OS}"
ARCHIVE="context-builder-${TARGET}.tar.gz"
BASE_URL="https://github.com/${REPO}/releases/latest/download"

echo "Installing context-builder for ${TARGET}..."

# Check write permissions
if [ ! -w "$INSTALL_DIR" ]; then
  echo "Note: $INSTALL_DIR requires elevated permissions."
  SUDO="sudo"
else
  SUDO=""
fi

# Download binary and checksums
TMP="$(mktemp -d)"
echo "Downloading ${ARCHIVE}..."
curl -sSL "${BASE_URL}/${ARCHIVE}" -o "$TMP/$ARCHIVE"
curl -sSL "${BASE_URL}/SHA256SUMS" -o "$TMP/SHA256SUMS"

# Verify SHA256 checksum
echo "Verifying checksum..."
EXPECTED="$(grep "$ARCHIVE" "$TMP/SHA256SUMS" | awk '{print $1}')"
if [ -z "$EXPECTED" ]; then
  echo "Warning: Could not find checksum for $ARCHIVE in SHA256SUMS"
  echo "Proceeding without verification..."
else
  if command -v sha256sum >/dev/null 2>&1; then
    ACTUAL="$(sha256sum "$TMP/$ARCHIVE" | awk '{print $1}')"
  elif command -v shasum >/dev/null 2>&1; then
    ACTUAL="$(shasum -a 256 "$TMP/$ARCHIVE" | awk '{print $1}')"
  else
    echo "Warning: No SHA256 tool found, skipping verification"
    ACTUAL="$EXPECTED"
  fi

  if [ "$ACTUAL" != "$EXPECTED" ]; then
    echo "Error: Checksum verification failed!"
    echo "  Expected: $EXPECTED"
    echo "  Got:      $ACTUAL"
    echo "The download may be corrupted or tampered with."
    rm -rf "$TMP"
    exit 1
  fi
  echo "✓ Checksum verified"
fi

# Extract and install
tar xzf "$TMP/$ARCHIVE" -C "$TMP"
$SUDO mv "$TMP/context-builder" "$INSTALL_DIR/context-builder"
$SUDO chmod +x "$INSTALL_DIR/context-builder"
rm -rf "$TMP"

# Verify
VERSION="$(context-builder --version 2>/dev/null || true)"
if [ -n "$VERSION" ]; then
  echo "✓ Installed: $VERSION"
else
  echo "✓ Installed to $INSTALL_DIR/context-builder"
  echo "  Make sure $INSTALL_DIR is in your PATH"
fi
```

### File: `src/cache.rs`

- Size: 19683 bytes
- Modified: SystemTime { tv_sec: 1771188560, tv_nsec: 626385572 }


**Signatures:**

```rust
// Structs/Classes
pub struct CacheManager

// Implementations
impl CacheManager

// Functions
pub fn new(project_path: &Path, config: &Config) -> Self
fn normalize_project_path(path: &Path) -> PathBuf
fn hash_path(path: &Path) -> String
fn normalize_path_format(path: &Path) -> PathBuf
fn hash_config(config: &Config) -> String
fn get_cache_path(&self) -> PathBuf
pub fn debug_cache_file_path(&self) -> PathBuf
fn migrate_old_cache(&self)
pub fn read_cache(&self) -> Result<Option<ProjectState>, Box<dyn std::error::Error>>
pub fn write_cache(&self, state: &ProjectState) -> Result<(), Box<dyn std::error::Error>>

// Modules
mod tests

// Functions
fn test_hash_path()
fn test_hash_config()
fn test_cache_operations()
fn test_old_cache_migration()
fn test_cache_consistency_across_path_representations()
fn test_normalize_path_format()
fn test_cache_read_nonexistent_file()
fn test_cache_read_corrupted_file()
fn test_cache_write_read_roundtrip()
fn test_different_configs_different_cache_files()
fn test_normalize_project_path_absolute()
fn test_normalize_project_path_relative()
fn test_hash_config_same_values()
fn test_migrate_old_cache_preserves_new_files()
```

### File: `src/cli.rs`

- Size: 6553 bytes
- Modified: SystemTime { tv_sec: 1771138255, tv_nsec: 543523125 }


**Signatures:**

```rust
// Structs/Classes
pub struct Args

// Modules
mod tests

// Functions
fn parses_with_no_args()
fn parses_all_flags_and_options()
fn short_flags_parse_correctly()
fn defaults_for_options_when_not_provided()
fn parses_diff_only_flag()
fn parses_clear_cache_flag()
fn parses_signatures_flag()
fn parses_structure_flag()
fn parses_truncate_mode()
fn parses_visibility_filter()
```

### File: `src/config.rs`

- Size: 10227 bytes
- Modified: SystemTime { tv_sec: 1771188530, tv_nsec: 741020763 }


**Signatures:**

```rust
// Structs/Classes
pub struct Config

// Functions
pub fn load_config() -> Option<Config>
pub fn load_config_from_path(project_root: &Path) -> Option<Config>

// Modules
mod tests

// Functions
fn load_config_nonexistent_file()
fn load_config_from_path_nonexistent_file()
fn load_config_from_path_valid_config()
fn load_config_from_path_partial_config()
fn load_config_from_path_invalid_toml()
fn load_config_from_path_empty_config()
fn config_default_implementation()
fn load_config_invalid_toml_in_cwd()
fn load_config_valid_in_cwd()
```

### File: `src/config_resolver.rs`

- Size: 15995 bytes
- Modified: SystemTime { tv_sec: 1771138540, tv_nsec: 524298632 }


**Signatures:**

```rust
// Structs/Classes
pub struct ResolvedConfig
pub struct ConfigResolution

// Functions
pub fn resolve_final_config(mut args: Args, config: Option<Config>) -> ConfigResolution
fn apply_config_to_args(args: &mut Args, config: &Config, warnings: &mut Vec<String>)
fn resolve_output_path(args: &mut Args, config: &Config, warnings: &mut Vec<String>)

// Modules
mod tests

// Functions
fn test_config_precedence_cli_over_config()
fn test_config_applies_when_cli_uses_defaults()
fn test_timestamped_output_resolution()
fn test_output_folder_resolution()
fn test_output_folder_with_timestamping()
fn test_auto_diff_without_timestamping_warning()
fn test_no_config_uses_cli_defaults()
```

### File: `src/diff.rs`

- Size: 21233 bytes
- Modified: SystemTime { tv_sec: 1771131034, tv_nsec: 854121736 }


**Signatures:**

```rust
// Functions
fn resolve_context_lines(explicit: Option<usize>) -> usize
pub fn generate_diff(old_content: &str, new_content: &str) -> String

// Enums
pub enum PerFileStatus

// Structs/Classes
pub struct PerFileDiff

// Implementations
impl PerFileDiff

// Functions
pub fn is_changed(&self) -> bool
fn unified_no_header(old: &str, new: &str, context_lines: usize) -> String
pub fn diff_file_contents(
    previous: &HashMap<String, String>,
    current: &HashMap<String, String>,
    skip_unchanged: bool,
    explicit_context: Option<usize>,
) -> Vec<PerFileDiff>
pub fn render_per_file_diffs(diffs: &[PerFileDiff]) -> String

// Modules
mod tests

// Functions
fn map(pairs: &[(&str, &str)]) -> HashMap<String, String>
fn unchanged_is_skipped()
fn added_file_diff()
fn removed_file_diff()
fn modified_file_diff()
fn include_unchanged_when_requested()
fn render_output_basic()
fn test_empty_files()
fn test_empty_to_content()
fn test_content_to_empty()
fn test_multiline_modifications()
fn test_windows_line_endings()
fn test_per_file_diff_is_changed()
fn test_generate_diff_identical_content()
fn test_generate_diff_with_changes()
fn test_resolve_context_lines_default()
fn test_resolve_context_lines_explicit()
fn test_resolve_context_lines_zero_fallback()
fn test_unicode_content_diff()
fn test_render_per_file_diffs_empty()
fn test_render_per_file_diffs_unchanged()
fn test_render_per_file_diffs_without_trailing_newline()
fn test_generate_diff_with_multiple_groups()
fn test_diff_with_windows_line_endings()
fn test_unified_no_header_with_multiple_groups()
fn test_unified_no_header_with_windows_line_endings()
```

### File: `src/file_utils.rs`

- Size: 31270 bytes
- Modified: SystemTime { tv_sec: 1771189128, tv_nsec: 567359354 }


**Signatures:**

```rust
// Functions
fn file_relevance_category(path: &Path, base_path: &Path) -> u8
fn file_entry_point_priority(path: &Path) -> u8
pub fn collect_files(
    base_path: &Path,
    filters: &[String],
    ignores: &[String],
    auto_ignores: &[String],
) -> io::Result<Vec<DirEntry>>
pub fn confirm_processing(file_count: usize) -> io::Result<bool>
pub fn confirm_overwrite(file_path: &str) -> io::Result<bool>
pub fn find_latest_file(dir: &Path) -> io::Result<Option<PathBuf>>

// Modules
mod tests

// Functions
fn to_rel_paths(mut entries: Vec<DirEntry>, base: &Path) -> Vec<String>
fn collect_files_respects_filters()
fn collect_files_respects_ignores_for_dirs_and_files()
fn collect_files_handles_invalid_ignore_pattern()
fn collect_files_empty_directory()
fn collect_files_no_matching_filters()
fn collect_files_ignores_config_file()
fn confirm_processing_small_count()
fn find_latest_file_empty_directory()
fn find_latest_file_nonexistent_directory()
fn find_latest_file_single_file()
fn find_latest_file_multiple_files()
fn find_latest_file_ignores_directories()
fn test_confirm_processing_requires_user_interaction()
fn test_confirm_overwrite_function_exists()
fn test_collect_files_handles_permission_errors()
fn test_find_latest_file_permission_error()
fn test_collect_files_with_symlinks()
fn test_file_relevance_category_lockfiles()
fn test_file_relevance_category_test_files_in_src()
fn test_file_relevance_category_benchmarks_in_src()
fn test_file_relevance_category_test_file_patterns()
fn test_file_relevance_category_build_files_without_extension()
fn test_collect_files_with_auto_ignores()
fn test_file_relevance_ci_directories()
fn test_file_relevance_docs_scripts_dirs()
fn test_file_relevance_various_source_extensions()
fn test_file_entry_point_priority_various_names()
fn test_collect_files_config_files_priority()
```

### File: `src/markdown.rs`

- Size: 54530 bytes
- Modified: SystemTime { tv_sec: 1771189128, tv_nsec: 583359553 }


**Signatures:**

```rust
// Structs/Classes
pub struct TreeSitterConfig

// Functions
pub fn generate_markdown(
    output_path: &str,
    input_dir: &str,
    filters: &[String],
    ignores: &[String],
    file_tree: &FileTree,
    files: &[DirEntry],
    base_path: &Path,
    line_numbers: bool,
    encoding_strategy: Option<&str>,
    max_tokens: Option<usize>,
    ts_config: &TreeSitterConfig,
) -> io::Result<()>

// Type Aliases
type ChunkResult

// Functions
pub fn process_file(
    base_path: &Path,
    file_path: &Path,
    output: &mut impl Write,
    line_numbers: bool,
    encoding_strategy: Option<&str>,
    ts_config: &TreeSitterConfig,
) -> io::Result<()>
pub fn write_tree_sitter_enrichment(
    output: &mut impl Write,
    content: &str,
    extension: &str,
    ts_config: &TreeSitterConfig,
) -> io::Result<()>
fn detect_text_encoding(bytes: &[u8]) -> Option<&'static Encoding>
fn is_likely_text(content: &str) -> bool
fn transcode_file_content(file_path: &Path, encoding: &'static Encoding) -> io::Result<String>
fn write_text_content(
    output: &mut impl Write,
    content: &str,
    language: &str,
    line_numbers: bool,
) -> io::Result<()>

// Modules
mod tests

// Functions
fn test_code_block_formatting()
fn test_markdown_file_formatting()
fn test_line_numbered_code_blocks()
fn test_binary_file_handling()
fn test_encoding_detection_and_transcoding()
fn test_encoding_strategy_strict()
fn test_encoding_strategy_skip()
fn test_generate_markdown_with_current_directory()
fn test_generate_markdown_creates_output_directory()
fn test_generate_markdown_with_filters_and_ignores()
fn test_write_text_content_with_line_numbers()
fn test_write_text_content_without_line_numbers()
fn test_write_text_content_without_trailing_newline()
fn test_is_likely_text()
fn test_detect_text_encoding()
fn test_transcode_file_content()
fn test_process_file_with_metadata_error()
fn test_process_file_with_different_extensions()
fn test_process_file_with_seek_error_handling()
fn test_process_file_jsx_tsx_extensions()
fn test_process_file_various_lock_extensions()
fn test_process_file_java_cpp_extensions()
fn test_process_file_with_bom()
fn test_detect_text_encoding_utf16()
fn test_detect_text_encoding_shift_jis()
fn test_transcode_file_content_with_errors()
fn test_write_tree_sitter_enrichment_no_feature()
fn test_generate_markdown_max_tokens_budget()
fn test_process_file_empty_file()
fn test_process_file_with_multibyte_utf8()
fn test_generate_markdown_with_ignores_list()
```

### File: `src/state.rs`

- Size: 28053 bytes
- Modified: SystemTime { tv_sec: 1771189128, tv_nsec: 586359591 }


**Signatures:**

```rust
// Structs/Classes
pub struct ProjectState
pub struct FileState
pub struct ProjectMetadata
pub struct StateComparison
pub struct ChangeSummary

// Implementations
impl ProjectState

// Functions
pub fn from_files(
        files: &[DirEntry],
        base_path: &Path,
        config: &Config,
        line_numbers: bool,
    ) -> std::io::Result<Self>
pub fn compare_with(&self, previous: &ProjectState) -> StateComparison
pub fn has_changes(&self, other: &ProjectState) -> bool
fn compute_config_hash(config: &Config) -> String

// Implementations
impl FileState

// Functions
pub fn from_path(path: &Path) -> std::io::Result<Self>

// Implementations
impl ChangeSummary

// Functions
pub fn has_changes(&self) -> bool
pub fn to_markdown(&self) -> String

// Modules
mod tests

// Functions
fn test_file_state_creation()
fn test_project_state_comparison()
fn test_change_summary_markdown()
fn test_binary_file_handling()
fn test_has_changes_identical_states()
fn test_has_changes_different_file_count()
fn test_has_changes_content_different()
fn test_config_hash_generation()
fn test_change_summary_no_changes()
fn test_from_files_with_config()
fn test_file_state_from_path_missing_file()
fn test_project_state_timestamp_format()
fn test_project_metadata_with_filters_and_ignores()
fn test_project_name_extraction_from_path()
fn test_change_summary_empty_outputs_nothing()
fn test_state_comparison_with_removed_files()
fn test_compute_config_hash_with_all_options()
fn test_compute_config_hash_differences()
fn create_mock_dir_entry(path: &std::path::Path) -> ignore::DirEntry
```

### File: `src/token_count.rs`

- Size: 10045 bytes
- Modified: SystemTime { tv_sec: 1771142666, tv_nsec: 596069918 }


**Signatures:**

```rust
// Functions
pub fn estimate_tokens(text: &str) -> usize
pub fn count_file_tokens(base_path: &Path, entry: &DirEntry, line_numbers: bool) -> usize
pub fn count_tree_tokens(tree: &BTreeMap<String, crate::tree::FileNode>, depth: usize) -> usize

// Modules
mod tests

// Functions
fn test_estimate_tokens()
fn test_count_tree_tokens()
fn test_token_estimation_format_consistency()
fn test_estimate_tokens_empty_string()
fn test_estimate_tokens_whitespace_only()
fn test_estimate_tokens_unicode()
fn test_count_file_tokens_with_line_numbers()
fn test_count_file_tokens_unreadable_file()
fn test_count_tree_tokens_empty_tree()
fn test_count_tree_tokens_nested_directories()
fn test_count_tree_tokens_mixed_content()
```

### File: `src/tree.rs`

- Size: 10845 bytes
- Modified: SystemTime { tv_sec: 1771091715, tv_nsec: 380300807 }


**Signatures:**

```rust
// Enums
pub enum FileNode

// Type Aliases
pub type FileTree

// Functions
pub fn build_file_tree(files: &[DirEntry], base_path: &Path) -> FileTree
fn insert_path(tree: &mut FileTree, components: &[std::path::Component])
pub fn print_tree(tree: &FileTree, depth: usize)
pub fn write_tree_to_file(
    output: &mut impl Write,
    tree: &FileTree,
    depth: usize,
) -> io::Result<()>

// Modules
mod tests

// Functions
fn test_build_file_tree_with_collected_files()
fn test_build_file_tree_empty()
fn test_build_file_tree_single_file()
fn test_build_file_tree_nested_directories()
fn test_build_file_tree_unicode_filenames()
fn test_insert_path_empty_components()
fn test_write_tree_to_file()
fn test_write_tree_to_file_with_depth()
fn test_write_tree_to_file_empty_tree()
fn test_file_node_equality()
fn test_build_file_tree_absolute_path_fallback()
fn test_build_file_tree_multiple_files_same_directory()
```

### File: `src/tree_sitter/language_support.rs`

- Size: 4742 bytes
- Modified: SystemTime { tv_sec: 1771153356, tv_nsec: 169962139 }


**Signatures:**

```rust
// Enums
pub enum SignatureKind

// Implementations
impl fmt::Display for SignatureKind

// Functions
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result

// Enums
pub enum Visibility

// Implementations
impl FromStr for Visibility

// Type Aliases
type Err

// Functions
fn from_str(s: &str) -> Result<Self, Self::Err>

// Implementations
impl Visibility

// Functions
pub fn matches_filter(self, filter: Visibility) -> bool

// Structs/Classes
pub struct Signature

// Implementations
impl fmt::Display for Signature

// Functions
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
pub fn slice_signature_before_body(
    source: &str,
    node: &tree_sitter::Node,
    body_kinds: &[&str],
) -> Option<String>

// Structs/Classes
pub struct CodeStructure

// Implementations
impl CodeStructure

// Functions
pub fn total_symbols(&self) -> usize

// Traits/Interfaces
pub trait LanguageSupport: Send + Sync

// Functions
fn supports_extension(&self, ext: &str) -> bool
```

### File: `src/tree_sitter/languages/c.rs`

- Size: 15771 bytes
- Modified: SystemTime { tv_sec: 1771189128, tv_nsec: 590359641 }


**Signatures:**

```rust
// Structs/Classes
pub struct CSupport

// Implementations
impl CSupport

// Functions
fn get_language() -> tree_sitter::Language

// Implementations
impl LanguageSupport for CSupport

// Functions
fn file_extensions(&self) -> &[&'static str]
fn parse(&self, source: &str) -> Option<Tree>
fn extract_signatures(&self, source: &str, visibility: Visibility) -> Vec<Signature>
fn extract_structure(&self, source: &str) -> CodeStructure
fn find_truncation_point(&self, source: &str, max_bytes: usize) -> usize

// Implementations
impl CSupport

// Functions
fn extract_signatures_from_node(
        &self,
        source: &str,
        node: &tree_sitter::Node,
        _visibility: Visibility,
        signatures: &mut Vec<Signature>,
    )
fn extract_structure_from_node(&self, node: &tree_sitter::Node, structure: &mut CodeStructure)
fn extract_function_signature(
        &self,
        source: &str,
        node: &tree_sitter::Node,
    ) -> Option<Signature>
fn extract_declaration_signature(
        &self,
        source: &str,
        node: &tree_sitter::Node,
    ) -> Option<Signature>
fn extract_struct_signature(
        &self,
        source: &str,
        node: &tree_sitter::Node,
    ) -> Option<Signature>
fn extract_enum_signature(&self, source: &str, node: &tree_sitter::Node) -> Option<Signature>
fn extract_typedef_signature(
        &self,
        source: &str,
        node: &tree_sitter::Node,
    ) -> Option<Signature>
fn extract_macro_signature(&self, source: &str, node: &tree_sitter::Node) -> Option<Signature>
fn find_function_name(&self, node: &tree_sitter::Node, source: &str) -> Option<String>
fn find_return_type(&self, node: &tree_sitter::Node, source: &str) -> Option<String>
fn find_child_text(
        &self,
        node: &tree_sitter::Node,
        kind: &str,
        source: &str,
    ) -> Option<String>
fn find_best_boundary(
        &self,
        cursor: &mut tree_sitter::TreeCursor,
        max_bytes: usize,
        best_end: &mut usize,
    )

// Modules
mod tests

// Functions
fn test_extract_function_signature()
fn test_extract_struct_signature()
fn test_extract_enum_signature()
fn test_extract_header_prototype()
fn test_extract_typedef()
fn test_extract_macro_definition()
fn test_extract_structure()
fn test_parse_valid_c()
fn test_find_truncation_point_within_limit()
fn test_file_extensions()
```

### File: `src/tree_sitter/languages/cpp.rs`

- Size: 18525 bytes
- Modified: SystemTime { tv_sec: 1771189128, tv_nsec: 592359666 }


**Signatures:**

```rust
// Structs/Classes
pub struct CppSupport

// Implementations
impl CppSupport

// Functions
fn get_language() -> tree_sitter::Language

// Implementations
impl LanguageSupport for CppSupport

// Functions
fn file_extensions(&self) -> &[&'static str]
fn parse(&self, source: &str) -> Option<Tree>
fn extract_signatures(&self, source: &str, visibility: Visibility) -> Vec<Signature>
fn extract_structure(&self, source: &str) -> CodeStructure
fn find_truncation_point(&self, source: &str, max_bytes: usize) -> usize

// Implementations
impl CppSupport

// Functions
fn extract_signatures_from_node(
        &self,
        source: &str,
        node: &tree_sitter::Node,
        visibility: Visibility,
        signatures: &mut Vec<Signature>,
    )
fn extract_structure_from_node(&self, node: &tree_sitter::Node, structure: &mut CodeStructure)
fn get_visibility(&self, _node: &tree_sitter::Node) -> Visibility
fn extract_function_signature(
        &self,
        source: &str,
        node: &tree_sitter::Node,
        visibility: Visibility,
    ) -> Option<Signature>
fn extract_declaration_signature(
        &self,
        source: &str,
        node: &tree_sitter::Node,
    ) -> Option<Signature>
fn extract_class_signature(
        &self,
        source: &str,
        node: &tree_sitter::Node,
        visibility: Visibility,
    ) -> Option<Signature>
fn extract_struct_signature(
        &self,
        source: &str,
        node: &tree_sitter::Node,
    ) -> Option<Signature>
fn extract_enum_signature(&self, source: &str, node: &tree_sitter::Node) -> Option<Signature>
fn extract_alias_signature(&self, source: &str, node: &tree_sitter::Node) -> Option<Signature>
fn extract_macro_signature(&self, source: &str, node: &tree_sitter::Node) -> Option<Signature>
fn find_function_name(&self, node: &tree_sitter::Node, source: &str) -> Option<String>
fn find_return_type(&self, node: &tree_sitter::Node, source: &str) -> Option<String>
fn find_child_text(
        &self,
        node: &tree_sitter::Node,
        kind: &str,
        source: &str,
    ) -> Option<String>
fn find_best_boundary(
        &self,
        cursor: &mut tree_sitter::TreeCursor,
        max_bytes: usize,
        best_end: &mut usize,
    )

// Modules
mod tests

// Functions
fn test_extract_class_signature()
fn test_extract_function_signature()
fn test_extract_struct_signature()
fn test_extract_enum_signature()
fn test_extract_header_prototype()
fn test_extract_template_class_with_inheritance()
fn test_extract_type_alias()
fn test_extract_macro()
fn test_extract_structure()
fn test_parse_valid_cpp()
fn test_find_truncation_point()
fn test_file_extensions()
```

### File: `src/tree_sitter/languages/go.rs`

- Size: 15675 bytes
- Modified: SystemTime { tv_sec: 1771189128, tv_nsec: 594359690 }


**Signatures:**

```rust
// Structs/Classes
pub struct GoSupport

// Implementations
impl GoSupport

// Functions
fn get_language() -> tree_sitter::Language

// Implementations
impl LanguageSupport for GoSupport

// Functions
fn file_extensions(&self) -> &[&'static str]
fn parse(&self, source: &str) -> Option<Tree>
fn extract_signatures(&self, source: &str, visibility: Visibility) -> Vec<Signature>
fn extract_structure(&self, source: &str) -> CodeStructure
fn find_truncation_point(&self, source: &str, max_bytes: usize) -> usize

// Implementations
impl GoSupport

// Functions
fn extract_signatures_from_node(
        &self,
        source: &str,
        node: &tree_sitter::Node,
        visibility: Visibility,
        signatures: &mut Vec<Signature>,
    )
fn extract_structure_from_node(&self, node: &tree_sitter::Node, structure: &mut CodeStructure)
fn is_exported(&self, name: &str) -> bool
fn extract_function_signature(
        &self,
        source: &str,
        node: &tree_sitter::Node,
        visibility: Visibility,
    ) -> Option<Signature>
fn extract_method_signature(
        &self,
        source: &str,
        node: &tree_sitter::Node,
        visibility: Visibility,
    ) -> Option<Signature>
fn extract_type_signatures(
        &self,
        source: &str,
        node: &tree_sitter::Node,
        visibility: Visibility,
        signatures: &mut Vec<Signature>,
    )
fn get_type_kind(&self, node: &tree_sitter::Node) -> SignatureKind
fn find_child_text(
        &self,
        node: &tree_sitter::Node,
        kind: &str,
        source: &str,
    ) -> Option<String>
fn find_child_text_for_result(&self, node: &tree_sitter::Node, source: &str) -> Option<String>
fn find_method_params(&self, node: &tree_sitter::Node, source: &str) -> Option<String>
fn find_best_boundary(
        &self,
        cursor: &mut tree_sitter::TreeCursor,
        max_bytes: usize,
        best_end: &mut usize,
    )

// Modules
mod tests

// Functions
fn test_extract_function_signature()
fn test_public_only_filter()
fn test_extract_struct_signature()
fn test_extract_interface_signature()
fn test_extract_method_with_receiver()
fn test_extract_type_alias()
fn test_extract_structure()
fn test_parse_valid_go()
fn test_find_truncation_point()
fn test_file_extensions()
```

### File: `src/tree_sitter/languages/java.rs`

- Size: 15822 bytes
- Modified: SystemTime { tv_sec: 1771189128, tv_nsec: 596359715 }


**Signatures:**

```rust
// Structs/Classes
pub struct JavaSupport

// Implementations
impl JavaSupport

// Functions
fn get_language() -> tree_sitter::Language

// Implementations
impl LanguageSupport for JavaSupport

// Functions
fn file_extensions(&self) -> &[&'static str]
fn parse(&self, source: &str) -> Option<Tree>
fn extract_signatures(&self, source: &str, visibility: Visibility) -> Vec<Signature>
fn extract_structure(&self, source: &str) -> CodeStructure
fn find_truncation_point(&self, source: &str, max_bytes: usize) -> usize

// Implementations
impl JavaSupport

// Functions
fn extract_signatures_from_node(
        &self,
        source: &str,
        node: &tree_sitter::Node,
        visibility: Visibility,
        signatures: &mut Vec<Signature>,
    )
fn extract_structure_from_node(&self, node: &tree_sitter::Node, structure: &mut CodeStructure)
fn get_visibility(&self, _node: &tree_sitter::Node) -> Visibility
fn extract_method_signature(
        &self,
        source: &str,
        node: &tree_sitter::Node,
        visibility: Visibility,
    ) -> Option<Signature>
fn extract_class_signature(
        &self,
        source: &str,
        node: &tree_sitter::Node,
        visibility: Visibility,
    ) -> Option<Signature>
fn extract_interface_signature(
        &self,
        source: &str,
        node: &tree_sitter::Node,
        visibility: Visibility,
    ) -> Option<Signature>
fn extract_enum_signature(
        &self,
        source: &str,
        node: &tree_sitter::Node,
        visibility: Visibility,
    ) -> Option<Signature>
fn extract_field_signature(
        &self,
        source: &str,
        node: &tree_sitter::Node,
        visibility: Visibility,
    ) -> Option<Signature>
fn find_child_text(
        &self,
        node: &tree_sitter::Node,
        kind: &str,
        source: &str,
    ) -> Option<String>
fn find_child_text_for_type(&self, node: &tree_sitter::Node, source: &str) -> Option<String>
fn find_best_boundary(
        &self,
        cursor: &mut tree_sitter::TreeCursor,
        max_bytes: usize,
        best_end: &mut usize,
    )

// Modules
mod tests

// Functions
fn test_extract_class_signature()
fn test_extract_method_signature()
fn test_extract_interface_signature()
fn test_extract_enum_signature()
fn test_extract_class_with_inheritance()
fn test_extract_structure()
fn test_parse_valid_java()
fn test_find_truncation_point()
fn test_file_extensions()
```

### File: `src/tree_sitter/languages/javascript.rs`

- Size: 13379 bytes
- Modified: SystemTime { tv_sec: 1771189128, tv_nsec: 598359740 }


**Signatures:**

```rust
// Structs/Classes
pub struct JavaScriptSupport

// Implementations
impl JavaScriptSupport

// Functions
fn get_language() -> tree_sitter::Language

// Implementations
impl LanguageSupport for JavaScriptSupport

// Functions
fn file_extensions(&self) -> &[&'static str]
fn parse(&self, source: &str) -> Option<Tree>
fn extract_signatures(&self, source: &str, visibility: Visibility) -> Vec<Signature>
fn extract_structure(&self, source: &str) -> CodeStructure
fn find_truncation_point(&self, source: &str, max_bytes: usize) -> usize

// Implementations
impl JavaScriptSupport

// Functions
fn extract_signatures_from_node(
        &self,
        source: &str,
        node: &tree_sitter::Node,
        _visibility: Visibility,
        signatures: &mut Vec<Signature>,
    )
fn extract_structure_from_node(&self, node: &tree_sitter::Node, structure: &mut CodeStructure)
fn extract_function_signature(
        &self,
        source: &str,
        node: &tree_sitter::Node,
    ) -> Option<Signature>
fn extract_class_signature(&self, source: &str, node: &tree_sitter::Node) -> Option<Signature>
fn extract_variable_declarations(
        &self,
        source: &str,
        node: &tree_sitter::Node,
        signatures: &mut Vec<Signature>,
    )
fn extract_export_signatures(
        &self,
        source: &str,
        node: &tree_sitter::Node,
        signatures: &mut Vec<Signature>,
    )
fn find_child_text(
        &self,
        node: &tree_sitter::Node,
        kind: &str,
        source: &str,
    ) -> Option<String>
fn find_best_boundary(
        &self,
        cursor: &mut tree_sitter::TreeCursor,
        max_bytes: usize,
        best_end: &mut usize,
    )

// Modules
mod tests

// Functions
fn test_extract_function_signature()
fn test_extract_class_signature()
fn test_file_extensions()
```

### File: `src/tree_sitter/languages/python.rs`

- Size: 11941 bytes
- Modified: SystemTime { tv_sec: 1771189128, tv_nsec: 600359765 }


**Signatures:**

```rust
// Structs/Classes
pub struct PythonSupport

// Implementations
impl PythonSupport

// Functions
fn get_language() -> tree_sitter::Language

// Implementations
impl LanguageSupport for PythonSupport

// Functions
fn file_extensions(&self) -> &[&'static str]
fn parse(&self, source: &str) -> Option<Tree>
fn extract_signatures(&self, source: &str, visibility: Visibility) -> Vec<Signature>
fn extract_structure(&self, source: &str) -> CodeStructure
fn find_truncation_point(&self, source: &str, max_bytes: usize) -> usize

// Implementations
impl PythonSupport

// Functions
fn extract_signatures_from_node(
        &self,
        source: &str,
        node: &tree_sitter::Node,
        _visibility: Visibility,
        signatures: &mut Vec<Signature>,
    )
fn extract_structure_from_node(&self, node: &tree_sitter::Node, structure: &mut CodeStructure)
fn extract_function_signature_with_context(
        &self,
        source: &str,
        node: &tree_sitter::Node,
        context_node: Option<&tree_sitter::Node>,
    ) -> Option<Signature>
fn extract_class_signature(&self, source: &str, node: &tree_sitter::Node) -> Option<Signature>
fn find_decorators(&self, source: &str, node: &tree_sitter::Node) -> Option<String>
fn find_child_text(
        &self,
        node: &tree_sitter::Node,
        kind: &str,
        source: &str,
    ) -> Option<String>
fn find_best_boundary(
        &self,
        cursor: &mut tree_sitter::TreeCursor,
        max_bytes: usize,
        best_end: &mut usize,
    )

// Modules
mod tests

// Functions
fn test_extract_function_signature()
fn test_extract_class_signature()
fn test_file_extensions()
```

### File: `src/tree_sitter/languages/rust.rs`

- Size: 25304 bytes
- Modified: SystemTime { tv_sec: 1771189128, tv_nsec: 603359803 }


**Signatures:**

```rust
// Structs/Classes
pub struct RustSupport

// Implementations
impl RustSupport

// Functions
fn get_language() -> tree_sitter::Language

// Implementations
impl LanguageSupport for RustSupport

// Functions
fn file_extensions(&self) -> &[&'static str]
fn parse(&self, source: &str) -> Option<Tree>
fn extract_signatures(&self, source: &str, visibility: Visibility) -> Vec<Signature>
fn extract_structure(&self, source: &str) -> CodeStructure
fn find_truncation_point(&self, source: &str, max_bytes: usize) -> usize

// Implementations
impl RustSupport

// Functions
fn walk_for_boundary(
        &self,
        cursor: &mut tree_sitter::TreeCursor,
        max_bytes: usize,
        best_end: &mut usize,
    )
fn extract_signatures_from_node(
        &self,
        source: &str,
        node: &tree_sitter::Node,
        visibility: Visibility,
        signatures: &mut Vec<Signature>,
    )
fn extract_structure_from_node(
        &self,
        source: &str,
        node: &tree_sitter::Node,
        structure: &mut CodeStructure,
    )
fn is_public(&self, node: &tree_sitter::Node) -> bool
fn get_visibility(&self, node: &tree_sitter::Node) -> Visibility
fn node_text<'a>(&self, source: &'a str, node: &tree_sitter::Node) -> &'a str
fn extract_function_signature(
        &self,
        source: &str,
        node: &tree_sitter::Node,
        visibility_filter: Visibility,
    ) -> Option<Signature>
fn extract_struct_signature(
        &self,
        source: &str,
        node: &tree_sitter::Node,
        visibility_filter: Visibility,
    ) -> Option<Signature>
fn extract_enum_signature(
        &self,
        source: &str,
        node: &tree_sitter::Node,
        visibility_filter: Visibility,
    ) -> Option<Signature>
fn extract_trait_signature(
        &self,
        source: &str,
        node: &tree_sitter::Node,
        visibility_filter: Visibility,
    ) -> Option<Signature>
fn extract_impl_signature(&self, source: &str, node: &tree_sitter::Node) -> Option<Signature>
fn extract_mod_signature(
        &self,
        source: &str,
        node: &tree_sitter::Node,
        visibility_filter: Visibility,
    ) -> Option<Signature>
fn extract_const_signature(
        &self,
        source: &str,
        node: &tree_sitter::Node,
        visibility_filter: Visibility,
    ) -> Option<Signature>
fn extract_type_alias_signature(
        &self,
        source: &str,
        node: &tree_sitter::Node,
        visibility_filter: Visibility,
    ) -> Option<Signature>
fn extract_macro_signature(
        &self,
        source: &str,
        node: &tree_sitter::Node,
        visibility_filter: Visibility,
    ) -> Option<Signature>
fn find_child_text(
        &self,
        node: &tree_sitter::Node,
        kind: &str,
        source: &str,
    ) -> Option<String>

// Modules
mod tests

// Functions
fn test_extract_function_signature()
fn test_public_only_filter()
fn test_extract_struct_signature()
fn test_extract_tuple_struct_signature()
fn test_extract_enum_signature()
fn test_extract_trait_signature()
fn test_extract_impl_signature()
fn test_extract_trait_impl_signature()
fn test_extract_module_signature()
fn test_extract_const_signature()
fn test_extract_const_public_filter()
fn test_extract_type_alias_signature()
fn test_extract_macro_signature()
fn test_extract_structure()
fn test_extract_structure_comprehensive()
fn test_find_truncation_point()
fn test_parse_valid_rust()
fn test_file_extensions()
```

### File: `src/tree_sitter/languages/typescript.rs`

- Size: 19775 bytes
- Modified: SystemTime { tv_sec: 1771189128, tv_nsec: 605359828 }


**Signatures:**

```rust
// Structs/Classes
pub struct TypeScriptSupport

// Implementations
impl TypeScriptSupport

// Functions
fn get_language() -> tree_sitter::Language

// Implementations
impl LanguageSupport for TypeScriptSupport

// Functions
fn file_extensions(&self) -> &[&'static str]
fn parse(&self, source: &str) -> Option<Tree>
fn extract_signatures(&self, source: &str, visibility: Visibility) -> Vec<Signature>
fn extract_structure(&self, source: &str) -> CodeStructure
fn find_truncation_point(&self, source: &str, max_bytes: usize) -> usize

// Implementations
impl TypeScriptSupport

// Functions
fn extract_signatures_from_node(
        &self,
        source: &str,
        node: &tree_sitter::Node,
        _visibility: Visibility,
        signatures: &mut Vec<Signature>,
    )
fn extract_structure_from_node(&self, node: &tree_sitter::Node, structure: &mut CodeStructure)
fn extract_function_signature(
        &self,
        source: &str,
        node: &tree_sitter::Node,
    ) -> Option<Signature>
fn extract_class_signature(&self, source: &str, node: &tree_sitter::Node) -> Option<Signature>
fn extract_interface_signature(
        &self,
        source: &str,
        node: &tree_sitter::Node,
    ) -> Option<Signature>
fn extract_type_alias_signature(
        &self,
        source: &str,
        node: &tree_sitter::Node,
    ) -> Option<Signature>
fn extract_enum_signature(&self, source: &str, node: &tree_sitter::Node) -> Option<Signature>
fn extract_variable_declarations(
        &self,
        source: &str,
        node: &tree_sitter::Node,
        signatures: &mut Vec<Signature>,
    )
fn extract_export_signatures(
        &self,
        source: &str,
        node: &tree_sitter::Node,
        signatures: &mut Vec<Signature>,
    )
fn find_child_text(
        &self,
        node: &tree_sitter::Node,
        kind: &str,
        source: &str,
    ) -> Option<String>
fn find_best_boundary(
        &self,
        cursor: &mut tree_sitter::TreeCursor,
        max_bytes: usize,
        best_end: &mut usize,
    )

// Modules
mod tests

// Functions
fn test_extract_function_signature()
fn test_extract_arrow_function()
fn test_extract_class_signature()
fn test_extract_interface_signature()
fn test_extract_enum_signature()
fn test_extract_type_alias()
fn test_extract_export_signatures()
fn test_extract_structure()
fn test_parse_valid_typescript()
fn test_find_truncation_point()
fn test_file_extensions()
```

### File: `src/tree_sitter/signatures.rs`

- Size: 7084 bytes
- Modified: SystemTime { tv_sec: 1771189128, tv_nsec: 607359853 }


**Signatures:**

```rust
// Functions
pub fn extract_signatures(
    source: &str,
    support: &dyn LanguageSupport,
    visibility: Visibility,
) -> Vec<Signature>
pub fn format_signatures_as_markdown(signatures: &[Signature], language: &str) -> String

// Modules
mod tests

// Functions
fn make_sig(kind: SignatureKind, name: &str, full_sig: &str) -> Signature
fn test_format_empty_signatures()
fn test_format_single_function()
fn test_format_multiple_same_kind()
fn test_format_mixed_kinds()
fn test_format_method_grouped_with_functions()
fn test_format_class_grouped_with_struct()
fn test_format_interface_grouped_with_trait()
fn test_extract_signatures_delegates()
fn test_extract_signatures_visibility_filter()
```

### File: `src/tree_sitter/structure.rs`

- Size: 2617 bytes
- Modified: SystemTime { tv_sec: 1771137172, tv_nsec: 933081207 }


**Signatures:**

```rust
// Functions
pub fn extract_structure(source: &str, support: &dyn LanguageSupport) -> CodeStructure
pub fn format_structure_as_markdown(structure: &CodeStructure) -> String

// Modules
mod tests

// Functions
fn test_format_empty_structure()
fn test_format_structure_with_symbols()
```

### File: `src/tree_sitter/truncation.rs`

- Size: 3307 bytes
- Modified: SystemTime { tv_sec: 1771189128, tv_nsec: 607359853 }


**Signatures:**

```rust
// Functions
pub fn find_truncation_point(
    source: &str,
    max_bytes: usize,
    support: &dyn LanguageSupport,
) -> usize
pub fn ensure_utf8_boundary(source: &str, position: usize) -> usize
pub fn add_truncation_notice(output: &mut String, truncated_count: usize)

// Modules
mod tests

// Functions
fn test_ensure_utf8_boundary_ascii()
fn test_ensure_utf8_boundary_unicode()
fn test_ensure_utf8_boundary_at_zero()
fn test_add_truncation_notice_with_count()
fn test_add_truncation_notice_zero_count()
fn test_find_truncation_point_source_within_limit()
fn test_find_truncation_point_source_exceeds_limit()
```

### File: `tarpaulin.toml`

- Size: 304 bytes
- Modified: SystemTime { tv_sec: 1771053288, tv_nsec: 65557165 }

```toml
[test_config]
name = "Context Builder"
manifest-path = "./Cargo.toml"
skip-clean = true
all-features = false
exclude-files = [
        "samples/*",
        "benches/*",
        "tests/*",
        "scripts/*",
        "src/main.rs"
    ]
no-fail-fast = true
color = "Auto"

[report]
out = ["Html", "Xml"]
```

### File: `benches/context_bench.rs`

- Size: 11135 bytes
- Modified: SystemTime { tv_sec: 1771138540, tv_nsec: 511298459 }


**Signatures:**

```rust
// Functions
fn init_bench_env()

// Structs/Classes
struct NoPrompt

// Implementations
impl Prompter for NoPrompt

// Functions
fn confirm_processing(&self, _file_count: usize) -> std::io::Result<bool>
fn confirm_overwrite(&self, _file_path: &str) -> std::io::Result<bool>

// Structs/Classes
struct DatasetSpec

// Functions
fn write_text_file(path: &Path, bytes: usize)
fn write_binary_file(path: &Path, bytes: usize)
fn generate_dataset(root: &Path, spec: &DatasetSpec) -> PathBuf
fn make_nested_dirs(base: &Path, depth: usize, width: usize) -> Vec<PathBuf>
fn bench_scenario(c: &mut Criterion, spec: DatasetSpec, line_numbers: bool)
pub fn context_benchmark(c: &mut Criterion)
```

### File: `tests/cli_integration.rs`

- Size: 13986 bytes
- Modified: SystemTime { tv_sec: 1771138540, tv_nsec: 547298937 }


**Signatures:**

```rust
// Structs/Classes
struct TestPrompter

// Implementations
impl TestPrompter

// Functions
fn new(overwrite_response: bool, processing_response: bool) -> Self
fn last_count(&self) -> usize

// Implementations
impl Prompter for TestPrompter

// Functions
fn confirm_processing(&self, file_count: usize) -> std::io::Result<bool>
fn confirm_overwrite(&self, _file_path: &str) -> std::io::Result<bool>
fn write_file(path: &Path, contents: &str)
fn preview_mode_does_not_create_output_file()
fn preview_mode_skips_overwrite_confirmation()
fn token_count_mode_skips_overwrite_confirmation()
fn both_preview_and_token_count_modes_work_together()
fn end_to_end_generates_output_with_filters_ignores_and_line_numbers()
fn overwrite_prompt_is_respected()
fn confirm_processing_receives_large_count()
fn token_count_mode_does_not_create_output_file()
```

### File: `tests/diff_integration.rs`

- Size: 1122 bytes
- Modified: SystemTime { tv_sec: 1771098907, tv_nsec: 779246312 }


**Signatures:**

```rust
// Functions
fn test_diff_with_identical_content()
fn test_diff_with_changes()
```

### File: `tests/test_auto_diff.rs`

- Size: 34489 bytes
- Modified: SystemTime { tv_sec: 1771138540, tv_nsec: 552299003 }


**Signatures:**

```rust
// Structs/Classes
struct TestPrompter

// Implementations
impl Prompter for TestPrompter

// Functions
fn confirm_processing(&self, _file_count: usize) -> std::io::Result<bool>
fn confirm_overwrite(&self, _file_path: &str) -> std::io::Result<bool>
fn create_simple_project(base_dir: &Path) -> std::io::Result<()>
fn test_auto_diff_workflow_basic()
fn test_auto_diff_added_and_removed_files()
fn test_diff_only_mode()
fn test_cache_invalidation_on_config_change()
fn test_concurrent_cache_access()
fn test_corrupted_cache_recovery()
fn test_diff_only_mode_includes_added_files()
```

### File: `tests/test_binary_file_autodiff.rs`

- Size: 8350 bytes
- Modified: SystemTime { tv_sec: 1771138540, tv_nsec: 553299016 }


**Signatures:**

```rust
// Structs/Classes
struct TestPrompter

// Implementations
impl TestPrompter

// Functions
fn new(overwrite_response: bool, processing_response: bool) -> Self

// Implementations
impl Prompter for TestPrompter

// Functions
fn confirm_processing(&self, _file_count: usize) -> std::io::Result<bool>
fn confirm_overwrite(&self, _file_path: &str) -> std::io::Result<bool>
fn write_file(path: &Path, contents: &str)
fn write_binary_file(path: &Path, data: &[u8])
fn test_binary_files_dont_crash_autodiff()
fn test_mixed_text_and_binary_files_autodiff()
fn test_large_binary_file_autodiff()
```

### File: `tests/test_comprehensive_edge_cases.rs`

- Size: 23611 bytes
- Modified: SystemTime { tv_sec: 1771138540, tv_nsec: 556299056 }


**Signatures:**

```rust
// Structs/Classes
struct TestPrompter

// Implementations
impl TestPrompter

// Functions
fn new(overwrite_response: bool, processing_response: bool) -> Self

// Implementations
impl Prompter for TestPrompter

// Functions
fn confirm_processing(&self, _file_count: usize) -> std::io::Result<bool>
fn confirm_overwrite(&self, _file_path: &str) -> std::io::Result<bool>
fn write_file(path: &Path, contents: &str)
fn write_binary_file(path: &Path, data: &[u8])
fn test_comprehensive_binary_file_edge_cases()
fn test_configuration_precedence_edge_cases()
fn test_cache_consistency_edge_cases()
fn test_error_conditions_and_exit_codes()
fn test_memory_usage_under_parallel_processing()
fn test_cwd_independent_operation()
fn test_edge_case_filenames_and_paths()
```

### File: `tests/test_config_resolution.rs`

- Size: 15023 bytes
- Modified: SystemTime { tv_sec: 1771138540, tv_nsec: 558299082 }


**Signatures:**

```rust
// Structs/Classes
struct TestPrompter

// Implementations
impl TestPrompter

// Functions
fn new(overwrite_response: bool, processing_response: bool) -> Self

// Implementations
impl Prompter for TestPrompter

// Functions
fn confirm_processing(&self, _file_count: usize) -> std::io::Result<bool>
fn confirm_overwrite(&self, _file_path: &str) -> std::io::Result<bool>
fn write_file(path: &Path, contents: &str)
fn run_with_resolved_config(
    args: Args,
    config: Option<context_builder::config::Config>,
    prompter: &impl Prompter,
) -> std::io::Result<()>
fn test_cli_arguments_override_config_file()
fn test_config_applies_when_cli_uses_defaults()
fn test_timestamped_output_and_output_folder()
fn test_mixed_explicit_and_default_values()
fn test_auto_diff_configuration_warning()
```

### File: `tests/test_cwd_independence.rs`

- Size: 13739 bytes
- Modified: SystemTime { tv_sec: 1771138540, tv_nsec: 560299109 }


**Signatures:**

```rust
// Structs/Classes
struct TestPrompter

// Implementations
impl TestPrompter

// Functions
fn new(overwrite_response: bool, processing_response: bool) -> Self

// Implementations
impl Prompter for TestPrompter

// Functions
fn confirm_processing(&self, _file_count: usize) -> std::io::Result<bool>
fn confirm_overwrite(&self, _file_path: &str) -> std::io::Result<bool>
fn write_file(path: &Path, contents: &str)
fn test_config_loaded_from_project_root_not_cwd()
fn test_cache_created_in_project_root_not_cwd()
fn test_clear_cache_uses_project_root()
fn test_load_config_from_path_function()
```

### File: `tests/test_determinism.rs`

- Size: 21480 bytes
- Modified: SystemTime { tv_sec: 1771143750, tv_nsec: 651431068 }


**Signatures:**

```rust
// Structs/Classes
struct TestPrompter

// Implementations
impl Prompter for TestPrompter

// Functions
fn confirm_processing(&self, _file_count: usize) -> std::io::Result<bool>
fn confirm_overwrite(&self, _file_path: &str) -> std::io::Result<bool>
fn create_test_project(base_dir: &Path) -> std::io::Result<()>
fn test_deterministic_output_multiple_runs()
fn test_deterministic_file_tree_order()
fn test_cache_collision_prevention()
fn test_custom_ignores_performance()
fn test_configuration_affects_cache_key()
fn test_edge_case_filenames_no_panic()
```

### File: `tests/test_parallel_memory.rs`

- Size: 9136 bytes
- Modified: SystemTime { tv_sec: 1771138540, tv_nsec: 565299175 }


**Signatures:**

```rust
// Structs/Classes
struct TestPrompter

// Implementations
impl TestPrompter

// Functions
fn new(overwrite_response: bool, processing_response: bool) -> Self

// Implementations
impl Prompter for TestPrompter

// Functions
fn confirm_processing(&self, _file_count: usize) -> std::io::Result<bool>
fn confirm_overwrite(&self, _file_path: &str) -> std::io::Result<bool>
fn test_streaming_parallel_processing()
fn test_parallel_error_handling()
fn test_memory_efficiency_with_large_files()
```

### File: `tests/test_phase4_integration.rs`

- Size: 11358 bytes
- Modified: SystemTime { tv_sec: 1771138540, tv_nsec: 566299188 }


**Signatures:**

```rust
// Structs/Classes
struct TestPrompter

// Implementations
impl TestPrompter

// Functions
fn new(overwrite_response: bool, processing_response: bool) -> Self

// Implementations
impl Prompter for TestPrompter

// Functions
fn confirm_processing(&self, _file_count: usize) -> std::io::Result<bool>
fn confirm_overwrite(&self, _file_path: &str) -> std::io::Result<bool>
fn write_file(path: &Path, contents: &str)
fn write_binary_file(path: &Path, data: &[u8])
fn test_phase4_features_integration()
fn test_encoding_strategy_configuration()
```

### File: `BENCHMARKS.md`

- Size: 6024 bytes
- Modified: SystemTime { tv_sec: 1771053288, tv_nsec: 64557151 }

```markdown
# Benchmarks

This document explains how to run the Criterion benchmarks, how datasets are chosen/created, and how to generate persistent sample datasets for reproducible measurements.

The benchmark suite measures:
- Sequential vs parallel processing
- With and without line-numbered code blocks
- Multiple dataset sizes (tiny, small, optionally medium)

By default, runs are silent to avoid skewing timings with console I/O.

---

## Quick start

- Run (parallel by default):
  - Linux/macOS:
    - `cargo bench --bench context_bench`
  - Windows PowerShell:
    - `cargo bench --bench context_bench`

- Include the medium dataset (heavier, disabled by default):
  - Linux/macOS:
    - `CB_BENCH_MEDIUM=1 cargo bench --bench context_bench`
  - Windows PowerShell:
    - `$env:CB_BENCH_MEDIUM=1; cargo bench --bench context_bench`

- HTML reports:
  - Open: `target/criterion/report/index.html`
  - Or per-benchmark: `target/criterion/context_builder/*/report/index.html`

---

## Parallel vs sequential

Parallel processing is enabled by default via the `parallel` feature (rayon).

- Force sequential:
  - `cargo bench --no-default-features --bench context_bench`

- Force parallel (even if defaults change):
  - `cargo bench --features parallel --bench context_bench`

Note: Benchmarks compare both “line_numbers” and “no_line_numbers” modes. Line numbering does additional formatting work and is expected to be slower.

---

## Silence during benchmarks

Benchmarks set `CB_SILENT=1` once at startup so logs and prompts don’t impact timings.

- To see output during benchmarks:
  - Linux/macOS:
    - `CB_SILENT=0 cargo bench --bench context_bench`
  - Windows PowerShell:
    - `$env:CB_SILENT=0; cargo bench --bench context_bench`

Prompts are auto-confirmed inside benches, so runs are fully non-interactive.

---

## Dataset selection

Each scenario picks an input dataset with the following precedence:

1) If `./samples/<dataset>/project` exists, it is used.
2) Else, if `CB_BENCH_DATASET_DIR` is set, `<CB_BENCH_DATASET_DIR>/<dataset>/project` is used.
3) Else, a synthetic dataset is generated in a temporary directory for the run.

Datasets used:
- tiny: ~100 text files (fast sanity checks)
- small: ~1,000 text files (default performance checks)
- medium: ~5,000 text files (only when `CB_BENCH_MEDIUM=1` is set)

Default filters in the benches focus on text/code: `rs`, `md`, `txt`, `toml`. Common ignored directories: `target`, `node_modules`. Binary files are generated but skipped by filters.

---

## Reproducing results

For more stable and reproducible measurements:
- Generate persistent datasets into `./samples/` (see below).
- Keep your machine’s background activity low during runs.
- Run each scenario multiple times and compare Criterion reports.

---

## Generating persistent sample datasets

You have two options to generate datasets into `./samples`:

### Option A: Cargo bin (feature-gated)

The repository provides a generator binary gated behind the `samples-bin` feature.

- Linux/macOS:
  - `cargo run --no-default-features --features samples-bin --bin generate_samples -- --help`
- Windows PowerShell:
  - `cargo run --no-default-features --features samples-bin --bin generate_samples -- --help`

Examples:
- Generate default presets (tiny, small) into `./samples`:
  - `cargo run --no-default-features --features samples-bin --bin generate_samples`
- Include medium and large:
  - `cargo run --no-default-features --features samples-bin --bin generate_samples -- --presets tiny,small,medium --include-large`
- Only one preset with custom parameters:
  - `cargo run --no-default-features --features samples-bin --bin generate_samples -- --only small --files 5000 --depth 4 --width 4 --size 1024`
- Clean output before generating:
  - `cargo run --no-default-features --features samples-bin --bin generate_samples -- --clean`
- Dry run (print plan only):
  - `cargo run --no-default-features --features samples-bin --bin generate_samples -- --dry-run`

### Option B: Standalone compile with rustc

If you prefer not to use the Cargo feature gating, compile the script directly:

- Linux/macOS:
  - `rustc scripts/generate_samples.rs -O -o generate_samples && ./generate_samples --help`
- Windows PowerShell:
  - `rustc scripts/generate_samples.rs -O -o generate_samples.exe; .\generate_samples.exe --help`

Examples mirror Option A; just replace the leading command with `./generate_samples` (or `.\generate_samples.exe` on Windows).

---

## Directory layout of generated samples

The generator produces datasets under `./samples/<preset>/project`, which benches discover automatically.

Each `project` tree contains:
- `src/`, `docs/`, `assets/` with nested subdirectories and text files
- `target/`, `node_modules/` populated with noise (ignored by default)
- Top-level `README.md`, `Cargo.toml`
- Binary `.bin` files sprinkled to validate binary handling

It’s recommended to add `/samples` to `.gitignore` if not already present.

---

## Comparing modes

- Sequential vs Parallel:
  - Sequential (no rayon): `cargo bench --no-default-features --bench context_bench`
  - Parallel (rayon): `cargo bench --features parallel --bench context_bench`

- With vs Without line numbers:
  - Both modes are exercised in each run; consult the per-benchmark report pages for timings.

---

## Troubleshooting

- Benchmarks produce no output:
  - Expected. They run with `CB_SILENT=1`. Set `CB_SILENT=0` to see logs.
- Medium dataset missing:
  - Set the flag explicitly: `CB_BENCH_MEDIUM=1`.
  - Or pre-generate samples so the benches find `./samples/medium/project`.
- Reports are empty or unchanged:
  - Remove previous results and re-run:
    - `rm -rf target/criterion` (Linux/macOS)
    - `Remove-Item -Recurse -Force target\criterion` (Windows PowerShell)
- Sequential vs parallel deltas are small:
  - On tiny datasets, overheads dominate. Use small or medium for more signal.
  - Try enabling/disabling line numbers to observe formatting costs.

---

Happy benchmarking!
```

### File: `DEVELOPMENT.md`

- Size: 7600 bytes
- Modified: SystemTime { tv_sec: 1771053288, tv_nsec: 64557151 }

```markdown
# Development Guide

Welcome! This document is for contributors and maintainers of Context Builder. It covers how to set up a development environment, build, test, lint, benchmark, and release the project.

For user-facing documentation and examples, see README.md. For performance work, see BENCHMARKS.md. For release history, see CHANGELOG.md.

---

## Prerequisites

- Rust toolchain (stable) with support for the 2024 edition.
  - Install via rustup: https://rustup.rs
  - Keep your toolchain up-to-date: `rustup update`
- Git

Optional but recommended:
- IDE with Rust Analyzer
- Just or Make for local task automation (if you prefer)
- Node.js (only if you plan to view Criterion’s HTML reports and serve them locally, not required for development)

---

## Getting the code

```bash
git clone https://github.com/igorls/context-builder.git
cd context-builder
```

---

## Project layout

- Cargo.toml — crate metadata, dependencies, features
- README.md — user-facing documentation
- CHANGELOG.md — release notes
- DEVELOPMENT.md — this file
- BENCHMARKS.md — running and understanding benchmarks
- scripts/
  - generate_samples.rs — synthetic dataset generator for benchmarking
- benches/
  - context_bench.rs — Criterion benchmark suite
- src/
  - main.rs — binary entry point
  - lib.rs — core orchestration and run() implementation
  - cli.rs — clap parser and CLI arguments
  - file_utils.rs — directory traversal, filter/ignore collection, prompts
  - markdown.rs — core rendering logic, streaming, line numbering, binary/text sniffing
  - tree.rs — file tree structure building and printing
- samples/ — optional persistent datasets (ignored in VCS) for benchmarking

---

## Building and running

Build:
```bash
cargo build
```

Run the CLI:
```bash
cargo run -- --help
cargo run -- -d . -o out.md -f rs -f toml -i target --line-numbers
```

Notes:
- By default, parallel processing is enabled via the `parallel` feature (uses rayon).
- Logging uses env_logger; set `RUST_LOG` to control verbosity:
  - Linux/macOS: `RUST_LOG=info cargo run -- ...`
  - Windows PowerShell: `$env:RUST_LOG='info'; cargo run -- ...`

---

## Features

- parallel (enabled by default)
  - Enables parallel file processing in markdown generation via rayon.
  - Disable defaults (sequential run):
    - Build/Run: `cargo run --no-default-features -- ...`
    - As a dependency in another crate: set `default-features = false` in Cargo.toml.

- samples-bin
  - Exposes the dataset generator as a cargo bin (development-only).
  - Usage:
    - Linux/macOS:
      - `cargo run --no-default-features --features samples-bin --bin generate_samples -- --help`
    - Windows PowerShell:
      - `cargo run --no-default-features --features samples-bin --bin generate_samples -- --help`

---

## Testing

Run all tests:
```bash
cargo test
```

Tips:
- Unit tests cover CLI parsing, file filtering/ignoring, markdown formatting (including line numbers and binary handling), and tree building.
- Avoid adding interactive prompts inside tests. The library is structured so that prompts can be injected/mocked (see `Prompter` trait).
- For additional diagnostics during tests:
  - Linux/macOS: `RUST_LOG=info cargo test`
  - Windows PowerShell: `$env:RUST_LOG='info'; cargo test`

---

## Linting and formatting

Format:
```bash
cargo fmt --all
```

Clippy (lints):
```bash
cargo clippy --all-targets --all-features -- -D warnings
```

Please ensure code is formatted and clippy-clean before opening a PR.

---

## Benchmarks

We use Criterion for micro/meso benchmarks and dataset-driven performance checks.

- See BENCHMARKS.md for details, including dataset generation, silent runs, and HTML report navigation.
- Quick start:
  ```bash
  cargo bench --bench context_bench
  ```

---

## Environment variables

- CB_SILENT
  - When set to “1” or “true” (case-insensitive), suppresses user-facing prints in the CLI.
  - The benchmark harness sets this to “1” by default to avoid skewing timings with console I/O.
  - Override locally:
    - Linux/macOS: `CB_SILENT=0 cargo bench --bench context_bench`
    - Windows PowerShell: `$env:CB_SILENT=0; cargo bench --bench context_bench`

- CB_BENCH_MEDIUM
  - When set to “1”, enables the heavier “medium” dataset scenarios during benches.

- CB_BENCH_DATASET_DIR
  - Allows pointing the benchmark harness to an external root containing datasets:
    - `<CB_BENCH_DATASET_DIR>/<preset>/project`
  - If not set and no `./samples/<preset>/project` is present, benches will synthesize datasets in a temp dir.

- RUST_LOG
  - Controls log verbosity (env_logger). Example:
    - Linux/macOS: `RUST_LOG=info cargo run -- ...`
    - Windows PowerShell: `$env:RUST_LOG='info'; cargo run -- ...`

---

## Coding guidelines

- Edition: 2024
- Error handling:
  - Use `io::Result` where appropriate; prefer returning errors over panicking.
  - It’s okay to use `unwrap()` and `expect()` in tests/benches and small setup helpers, but not in core library logic.
- Performance:
  - Prefer streaming reads/writes for large files (see `markdown.rs`).
  - Keep binary detection lightweight (current sniff logic checks for NUL bytes and UTF-8 validity).
  - Keep language detection simple and deterministic; add new mappings in one place.
- Cross-platform:
  - Normalize path separators in tests where string comparisons are used.
- Logging:
  - Use `log::{info, warn, error}`; let `env_logger` control emission.
- CLI:
  - Add new flags in `cli.rs`. Ensure you update tests covering parsing, and propagate options cleanly through `run_with_args`.

---

## Submitting changes

1) Fork and create a feature branch:
   ```bash
   git checkout -b feat/my-improvement
   ```

2) Make changes, add tests, and keep the code formatted and clippy-clean:
   ```bash
   cargo fmt --all
   cargo clippy --all-targets --all-features -- -D warnings
   cargo test
   ```

3) If you modified performance-sensitive code, run benches (see BENCHMARKS.md).

4) Update CHANGELOG.md if the change is user-visible or noteworthy.

5) Open a PR with:
   - A concise title
   - Description of changes and rationale
   - Notes on performance impact (if any)
   - Any relevant screenshots or benchmark snippets

Suggested commit message convention: short, imperative subject; optionally scope (e.g., `feat(cli): add --no-parallel flag`).

---

## Releasing (maintainers)

1) Ensure the tree is green:
   - `cargo fmt --all`
   - `cargo clippy --all-targets --all-features -- -D warnings`
   - `cargo test`
   - Optionally: `cargo bench`

2) Update versions and docs:
   - Bump `version` in `Cargo.toml`.
   - Add a new entry to `CHANGELOG.md`.
   - Verify README.md and DEVELOPMENT.md are up to date.

3) Tag the release:
   ```bash
   git commit -am "chore(release): vX.Y.Z"
   git tag vX.Y.Z
   git push && git push --tags
   ```

4) Publish to crates.io:
   ```bash
   cargo publish --dry-run
   cargo publish
   ```

5) Create a GitHub release, paste changelog highlights, and attach links to benchmarks if relevant.

---

## Tips and pitfalls

- Prompts during runs
  - The library uses a `Prompter` trait for confirmation flows. Inject a test-friendly prompter to avoid interactive I/O in tests and benches.
- Output file overwrites
  - The CLI confirms overwrites by default. In tests/benches, use the injected prompter that auto-confirms.
- Large datasets
  - Prefer parallel builds for performance.
  - Consider dataset size and line-numbering effects when measuring.

---

## Questions?

Open an issue or start a discussion on GitHub. Thanks for contributing!
```

### File: `SKILL.md`

- Size: 10151 bytes
- Modified: SystemTime { tv_sec: 1771190303, tv_nsec: 75900417 }

```markdown
---
name: context-builder
description: Generate LLM-optimized codebase context from any directory using context-builder CLI
homepage: https://github.com/igorls/context-builder
version: 0.8.2
requires:
  - context-builder
---

# Context Builder — Agentic Skill

Generate a single, structured markdown file from any codebase directory. The output is optimized for LLM consumption with relevance-based file ordering, AST-aware code signatures, automatic token budgeting, and smart defaults.

## Installation

### Pre-built Binary (recommended)

Download the latest release binary for your platform:

```bash
# Linux (x86_64)
curl -sSL https://github.com/igorls/context-builder/releases/latest/download/context-builder-x86_64-unknown-linux-gnu.tar.gz | tar xz -C /usr/local/bin

# macOS (Apple Silicon)
curl -sSL https://github.com/igorls/context-builder/releases/latest/download/context-builder-aarch64-apple-darwin.tar.gz | tar xz -C /usr/local/bin

# macOS (Intel)
curl -sSL https://github.com/igorls/context-builder/releases/latest/download/context-builder-x86_64-apple-darwin.tar.gz | tar xz -C /usr/local/bin
```

Windows: download `context-builder-x86_64-pc-windows-msvc.zip` from [GitHub Releases](https://github.com/igorls/context-builder/releases/latest).

### From Source (fallback)

```bash
# Requires Rust toolchain (cargo)
cargo install context-builder --features tree-sitter-all
```

Verify: `context-builder --version` (expected: `0.8.2`)


## Security & Path Scoping

> **IMPORTANT**: This tool reads file contents from the specified directory. Agents MUST follow these rules:

- **Only target project directories** — never point at home directories, system paths, or credential stores (`~/.ssh`, `~/.aws`, `/etc`)
- **Use explicit paths** — always pass the exact project root, not `/` or `~`
- **Prefer scoped filters** — use `-f` to limit to known source extensions (e.g., `-f rs,toml,md`)
- **Output to temp or docs** — write output to the project's `docs/` folder or `/tmp/`, never to shared or public locations
- **Review before sharing** — the output may contain API keys, secrets, or credentials embedded in source files; always review or use `.gitignore` patterns

The tool automatically excludes `.git/`, `node_modules/`, and 19 other heavy/sensitive directories at any depth. It also respects `.gitignore` rules when a `.git` directory is present.

## When to Use

- **Deep code review** — Feed an entire codebase to an LLM for architecture analysis or bug hunting
- **Onboarding** — Generate a project snapshot for understanding unfamiliar codebases
- **Diff-based updates** — After code changes, generate only the diffs to update an LLM's understanding
- **AST signatures** — Extract function/class signatures for token-efficient structural understanding
- **Cross-project research** — Quickly package a dependency's source for analysis

## Core Workflow

### 1. Quick Context (whole project)

```bash
context-builder -d /path/to/project -y -o context.md
```

- `-y` skips confirmation prompts (essential for non-interactive agent use)
- Output includes: header → file tree → files sorted by relevance (config → source → tests → docs)

### 2. Scoped Context (specific file types)

```bash
context-builder -d /path/to/project -f rs,toml -i docs,assets -y -o context.md
```

- `-f rs,toml` includes only Rust and TOML files
- `-i docs,assets` excludes directories by name

### 3. AST Signatures Mode (minimal tokens)

```bash
context-builder -d /path/to/project --signatures -f rs,ts,py -y -o signatures.md
```

- Replaces full file content with extracted function/class signatures (~4K vs ~15K tokens per file)
- Supports 8 languages: Rust, JavaScript (.js/.jsx), TypeScript (.ts/.tsx), Python, Go, Java, C, C++
- Requires `--features tree-sitter-all` at install time

### 4. Signatures with Structural Summary

```bash
context-builder -d /path/to/project --signatures --structure -y -o context.md
```

- `--structure` appends a count summary (e.g., "6 functions, 2 structs, 1 impl block")
- Combine with `--visibility public` to show only public API surface

### 5. Budget-Constrained Context

```bash
context-builder -d /path/to/project --max-tokens 100000 -y -o context.md
```

- Caps output to ~100K tokens (estimated)
- Files are included in relevance order until budget is exhausted
- Automatically warns if output exceeds 128K tokens

### 6. Token Count Preview

```bash
context-builder -d /path/to/project --token-count
```

- Prints estimated token count without generating output
- Use this first to decide if filtering or `--signatures` is needed

### 7. Incremental Diffs

First, ensure `context-builder.toml` exists with:

```toml
timestamped_output = true
auto_diff = true
```

Then run twice:

```bash
# First run: baseline snapshot
context-builder -d /path/to/project -y

# After code changes: generates diff annotations
context-builder -d /path/to/project -y
```

For minimal output (diffs only, no full file bodies):

```bash
context-builder -d /path/to/project -y --diff-only
```

## Smart Defaults

These behaviors require no configuration:

| Feature | Behavior |
|---------|----------|
| **Auto-ignore** | `node_modules`, `dist`, `build`, `__pycache__`, `.venv`, `vendor`, and 12 more heavy dirs are excluded at any depth |
| **Self-exclusion** | Output file, cache dir, and `context-builder.toml` are auto-excluded |
| **.gitignore** | Respected automatically when `.git` directory exists |
| **Binary detection** | Binary files are skipped via UTF-8 sniffing |
| **File ordering** | Config/docs first → source (entry points before helpers) → tests → build/CI → lockfiles |

## CLI Reference (Agent-Relevant Flags)

| Flag | Purpose | Agent Guidance |
|------|---------|----------------|
| `-d <PATH>` | Input directory | **Always use absolute paths** for reliability |
| `-o <FILE>` | Output path | Write to project `docs/` or `/tmp/` |
| `-f <EXT>` | Filter by extension | Comma-separated: `-f rs,toml,md` |
| `-i <NAME>` | Ignore dirs/files | Comma-separated: `-i tests,docs,assets` |
| `--max-tokens <N>` | Token budget cap | Use `100000` for most models, `200000` for Gemini |
| `--token-count` | Dry-run token estimate | Run first to check if filtering is needed |
| `-y` | Skip all prompts | **Always use in agent workflows** |
| `--preview` | Show file tree only | Quick exploration without generating output |
| `--diff-only` | Output only diffs | Minimizes tokens for incremental updates |
| `--signatures` | AST signature extraction | Requires `tree-sitter-all` feature at install |
| `--structure` | Structural summary | Pair with `--signatures` for compact output |
| `--visibility <V>` | Filter by visibility | `all` (default), `public` (public API only) |
| `--truncate <MODE>` | Truncation strategy | `smart` (AST-aware) or `simple` |
| `--init` | Create config file | Auto-detects project file types |
| `--clear-cache` | Reset diff cache | Use if diff output seems stale |

## Recipes

### Recipe: Deep Think Code Review

Generate a scoped context file, then prompt an LLM for deep analysis:

```bash
# Step 1: Generate focused context
context-builder -d /path/to/project -f rs,toml --max-tokens 120000 -y -o docs/deep_think_context.md

# Step 2: Feed to LLM with a review prompt
# Attach docs/deep_think_context.md and ask for:
# - Architecture review
# - Bug hunting
# - Performance analysis
```

### Recipe: API Surface Review (signatures only)

```bash
# Extract only public signatures — typically 80-90% fewer tokens than full source
context-builder -d /path/to/project --signatures --visibility public -f rs -y -o docs/api_surface.md
```

### Recipe: Compare Two Versions

```bash
# Generate context for both versions
context-builder -d ./v1 -f py -y -o /tmp/v1_context.md
context-builder -d ./v2 -f py -y -o /tmp/v2_context.md

# Feed both to an LLM for comparative analysis
```

### Recipe: Monorepo Slice

```bash
# Focus on a specific package within a monorepo
context-builder -d /path/to/monorepo/packages/core -f ts,tsx -i __tests__,__mocks__ -y -o core_context.md
```

### Recipe: Quick Size Check Before Deciding Strategy

```bash
# Check if the project fits in context
context-builder -d /path/to/project --token-count

# If > 128K tokens, try signatures mode first:
context-builder -d /path/to/project --signatures --token-count

# Or scope it down:
context-builder -d /path/to/project -f rs,toml --max-tokens 100000 --token-count
```

## Configuration File (Optional)

Create `context-builder.toml` in the project root for persistent settings:

```toml
output = "docs/context.md"
output_folder = "docs"
filter = ["rs", "toml"]
ignore = ["target", "benches"]
timestamped_output = true
auto_diff = true
max_tokens = 120000
signatures = true
structure = true
visibility = "public"
```

Initialize one automatically with `context-builder --init`.

## Output Format

The generated markdown follows this structure:

    # Directory Structure Report
    [metadata: project name, filters, content hash]

    ## File Tree
    [visual tree of included files]

    ## Files
    ### File: src/main.rs
    [code block with file contents, syntax-highlighted by extension]

    ### File: src/lib.rs
    ...

Files appear in **relevance order** (not alphabetical), prioritizing config and entry points so LLMs build understanding faster.

When `--signatures` is active, file contents are replaced with extracted signatures:

    ### File: src/lib.rs
    ```rust
    pub fn run_with_args(args: Args, config: Config, prompter: &dyn Prompter) -> Result<()>
    pub fn generate_markdown_with_diff(...) -> Result<String>
    ```

## Error Handling

- If `context-builder` is not installed, install with `cargo install context-builder --features tree-sitter-all`
- If `--signatures` shows no output for a file, the language may not be supported or the feature was not enabled at install
- If output exceeds token limits, add `--max-tokens` or narrow with `-f` / `-i`, or use `--signatures`
- If the project has no `.git` directory, auto-ignores still protect against dependency flooding
- Use `--clear-cache` if diff output seems stale or incorrect
```

### File: `scripts/demo.sh`

- Size: 2948 bytes
- Modified: SystemTime { tv_sec: 1771147572, tv_nsec: 49856151 }

```sh
#!/usr/bin/env bash
# Demo script for context-builder v0.8.0 — records a clean asciinema demo
# Usage: asciinema rec --cols 100 --rows 32 --command="bash scripts/demo.sh" docs/demo.cast

set -e

# Simulate typing effect
type_cmd() {
    local cmd="$1"
    local delay="${2:-0.04}"
    printf '\033[1;32m❯\033[0m '
    for ((i=0; i<${#cmd}; i++)); do
        printf '%s' "${cmd:$i:1}"
        sleep "$delay"
    done
    sleep 0.4
    echo ""
}

# Section header
section() {
    echo ""
    printf '\033[1;35m━━━ %s ━━━\033[0m\n' "$1"
    sleep 0.8
}

# Copy our own source to a clean temp dir (avoids config overrides)
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
REPO="$SCRIPT_DIR/.."
DEMO_DIR=$(mktemp -d)
PROJECT="$DEMO_DIR/context-builder"
mkdir -p "$PROJECT"

# Copy the real source code
cp -r "$REPO/src" "$PROJECT/src"
cp -r "$REPO/tests" "$PROJECT/tests"
cp "$REPO/Cargo.toml" "$PROJECT/"
cp "$REPO/Cargo.lock" "$PROJECT/" 2>/dev/null || true

cd "$PROJECT"

clear
echo ""
printf '\033[1;33m  ╔══════════════════════════════════════════════════════╗\033[0m\n'
printf '\033[1;33m  ║  ⚡ \033[1;37mcontext-builder\033[1;33m v0.8.0  — \033[0;36mTree-Sitter Edition\033[1;33m   ║\033[0m\n'
printf '\033[1;33m  ╚══════════════════════════════════════════════════════╝\033[0m\n'
printf '\033[2m    LLM context from your codebase, with AST superpowers\033[0m\n'
echo ""
sleep 1.5

# --- Demo 1: Preview ---
section "1. See what files will be included"
type_cmd "context-builder -f rs --preview -y"
context-builder -f rs --preview -y 2>/dev/null
sleep 1.5

# --- Demo 2: Full context ---
section "2. Generate full LLM context"
type_cmd "context-builder -f rs -o full.md -y"
context-builder -f rs -o full.md -y 2>/dev/null
sleep 1

type_cmd "head -45 full.md"
head -45 full.md
sleep 2.5

# --- Demo 3: Signatures ---
section "3. NEW: Extract signatures only (tree-sitter AST)"
type_cmd "context-builder -f rs --signatures -o sigs.md -y"
context-builder -f rs --signatures -o sigs.md -y 2>/dev/null
sleep 1

type_cmd "head -70 sigs.md"
head -70 sigs.md
sleep 3

# --- Demo 4: Size comparison ---
section "4. Compare: full context vs signatures"
type_cmd "wc -l full.md sigs.md"
wc -l full.md sigs.md
sleep 2.5

# --- Demo 5: Structure ---
section "5. NEW: Structural summary per file"
type_cmd "context-builder -f rs --structure --signatures -o overview.md -y"
context-builder -f rs --structure --signatures -o overview.md -y 2>/dev/null
sleep 1

type_cmd "head -80 overview.md"
head -80 overview.md
sleep 3

echo ""
printf '\033[1;32m✨ Your codebase is now LLM-ready.\033[0m\n'
printf '\033[2m   cargo install context-builder --features tree-sitter-all\033[0m\n'
sleep 3

# Cleanup
rm -rf "$DEMO_DIR"
```

### File: `scripts/generate_samples.rs`

- Size: 16036 bytes
- Modified: SystemTime { tv_sec: 1771053288, tv_nsec: 64557151 }


**Signatures:**

```rust
// Structs/Classes
struct DatasetSpec

// Implementations
impl DatasetSpec

// Functions
fn with_name(name: &str) -> Option<Self>
fn default_filters() -> Vec<String>
fn default_ignores() -> Vec<String>

// Structs/Classes
struct Args

// Functions
fn parse_args() -> Args
fn expect_value<'a, I>(flag: &str, it: &mut I) -> String
where
    I: Iterator<Item = String>,
fn parse_usize(s: String) -> Option<usize>
fn parse_csv(s: String) -> Vec<String>
fn print_help()
fn write_text_file(path: &Path, bytes: usize) -> io::Result<()>
fn write_binary_file(path: &Path, bytes: usize) -> io::Result<()>
fn make_nested_dirs(base: &Path, depth: usize, width: usize) -> io::Result<Vec<PathBuf>>
fn write_string(path: &Path, s: &str) -> io::Result<()>
fn generate_dataset(root: &Path, spec: &DatasetSpec, dry_run: bool) -> io::Result<()>
fn apply_overrides(spec: &mut DatasetSpec, args: &Args)
fn main() -> io::Result<()>

// Modules
mod tests

// Functions
fn test_expect_value()
```

