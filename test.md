# Directory Structure Report

This document contains all files from the `context-builder` directory, optimized for LLM consumption.
Content hash: bafd944d08e879fc

## File Tree Structure

- 📄 AGENTS.md
- 📄 BENCHMARKS.md
- 📄 CHANGELOG.md
- 📄 Cargo.toml
- 📄 DEVELOPMENT.md
- 📄 LICENSE
- 📄 README.md
- 📄 SKILL.md
- 📁 benches
  - 📄 context_bench.rs
- 📁 docs
  - 📁 research
    - 📄 multi-model-code-review-analysis.md
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


### File: `AGENTS.md`

- Size: 6816 bytes
- Modified: 2026-02-14 07:24:34 UTC

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

- Size: 9078 bytes
- Modified: 2026-02-15 04:21:58 UTC

```markdown
# Changelog

All notable changes to this project will be documented in this file.

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

- Size: 2905 bytes
- Modified: 2026-02-15 06:44:58 UTC

```toml
[package]
name = "context-builder"
version = "0.7.1"
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
# Note: tree-sitter 0.22.x requires grammars compiled for ABI version 14
tree-sitter = { version = "0.22", optional = true }
tree-sitter-rust = { version = "0.21", optional = true }
tree-sitter-javascript = { version = "0.21", optional = true }
tree-sitter-typescript = { version = "0.21", optional = true }
tree-sitter-python = { version = "0.21", optional = true }
tree-sitter-go = { version = "0.21", optional = true }
tree-sitter-java = { version = "0.21", optional = true }
tree-sitter-c = { version = "0.21", optional = true }
tree-sitter-cpp = { version = "0.21", optional = true }

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

- Size: 10641 bytes
- Modified: 2026-02-15 04:23:13 UTC

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

- 🧪 **Accurate Token Counting:**
  Get real tokenizer–based estimates with `--token-count` to plan your prompt budgets.


---

## Installation

### From crates.io (Recommended)

```bash
cargo install context-builder
```


### If you don't have Rust installed

Context Builder is distributed via crates.io. We do not ship pre-built binaries yet, so you need a Rust toolchain.


#### Quick install (Linux/macOS):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
Follow the prompt, then restart your shell

#### Windows: https://www.rust-lang.org/tools/install

After installation, ensure Cargo is on your PATH:

```bash
cargo --version
```

Then install Context Builder:
```bash
cargo install context-builder
```

Update later with:
```bash
cargo install context-builder --force
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

- Size: 53430 bytes
- Modified: 2026-02-15 07:30:07 UTC

```rust
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

        // 4. Generate markdown with diff annotations
        let mut final_doc = generate_markdown_with_diff(
            &current_state,
            comparison.as_ref(),
            &final_args,
            &file_tree,
            diff_cfg,
            &sorted_paths,
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
            eprintln!(
                "⚠️  --signatures/--structure/--truncate smart require tree-sitter support."
            );
            eprintln!(
                "   Build with: cargo build --features tree-sitter-all"
            );
            eprintln!(
                "   Falling back to standard output.\n"
            );
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
```

### File: `src/main.rs`

- Size: 73 bytes
- Modified: 2026-02-14 07:14:48 UTC

```rust
use std::io;

fn main() -> io::Resul
```

### File: `src/tree_sitter/languages/mod.rs`

- Size: 3447 bytes
- Modified: 2026-02-15 06:47:05 UTC

```rust
//! Language support registry.
//!
//! This module provides access to language-specific parsers based on file extensions.

#[cfg(feature = "tree-sitter-base")]
use super::language_support::LanguageSupport;

#[cfg(feature = "tree-sitter-rust")]
mod rust;

#[cfg(feature = "tree-sitter-js")]
mod javascript;

#[cfg(feature = "tree-sitter-ts")]
mod typescript;

#[cfg(feature = "tree-sitter-python")]
mod python;

#[cfg(feature = "tree-sitter-go")]
mod go;

#[cfg(feature = "tree-sitter-java")]
mod java;

#[cfg(feature = "tree-sitter-c")]
mod c;

#[cfg(feature = "tree-sitter-cpp")]
mod cpp;

#[cfg(feature = "tree-sitter-rust")]
static RUST_SUPPORT: rust::RustSupport = rust::RustSupport;

#[cfg(feature = "tree-sitter-js")]
static JS_SUPPORT: javascript::JavaScriptSupport = javascript::JavaScriptSupport;

#[cfg(feature = "tree-sitter-ts")]
static TS_SUPPORT: typescript::TypeScriptSupport = typescript::TypeScriptSupport;

#[cfg(feature = "tree-sitter-python")]
static PYTHON_SUPPORT: python::PythonSupport = python::PythonSupport;

#[cfg(feature = "tree-sitter-go")]
static GO_SUPPORT: go::GoSupport = go::GoSupport;

#[cfg(feature = "tree-sitter-java")]
static JAVA_SUPPORT: java::JavaSupport = java::JavaSupport;

#[cfg(feature = "tree-sitter-c")]
static C_SUPPORT: c::CSupport = c::CSupport;

#[cfg(feature = "tree-sitter-cpp")]
static CPP_SUPPORT: cpp::CppSupport = cpp::CppSupport;
```

### File: `src/tree_sitter/mod.rs`

- Size: 4087 bytes
- Modified: 2026-02-15 07:29:42 UTC

```rust
//! Tree-sitter integration for intelligent code parsing.
//!
//! This module provides:
//! - Signature extraction (function/class signatures without bodies)
//! - Smart truncation (truncate at AST boundaries)
//! - Structure extraction (imports, exports, symbol counts)
//!
//! Feature-gated: Only compiled when one of the tree-sitter-* features is enabled.

#[cfg(feature = "tree-sitter-base")]
pub mod language_support;

#[cfg(feature = "tree-sitter-base")]
pub mod signatures;

#[cfg(feature = "tree-sitter-base")]
pub mod structure;

#[cfg(feature = "tree-sitter-base")]
pub mod truncation;

#[cfg(feature = "tree-sitter-base")]
pub mod languages;

#[cfg(feature = "tree-sitter-base")]
use std::path::Path;

#[cfg(feature = "tree-sitter-base")]
pub use language_support::{CodeStructure, LanguageSupport, Signature, SignatureKind, Visibility};

#[cfg(feature = "tree-sitter-base")]
pub use signatures::extract_signatures;

#[cfg(feature = "tree-sitter-base")]
pub use structure::extract_structure;

#[cfg(feature = "tree-sitter-base")]
pub use truncation::find_truncation_point;

/// Check if tree-sitter is available for a given file extension.
#[cfg(feature = "tree-sitter-base")]
pub fn is_supported_extension(ext: &str) -> bool {
    languages::get_language_support(ext).is_some()
}

#[cfg(not(feature = "tree-sitter-base"))]
pub fn is_supported_extension(_ext: &str) -> bool {
    false
}

/// Extract file extension from a path.
#[cfg(feature = "tree-sitter-base")]
fn get_extension(path: &Path) -> Option<String> {
    path.extension()
        .and_then(|e| e.to_str())
        .map(|s| s.to_lowercase())
}

/// Get language support for a file path.
#[cfg(feature = "tree-sitter-base")]
pub fn get_language_for_path(path: &Path) -> Option<&'static dyn LanguageSupport> {
    let ext = get_extension(path)?;
    languages::get_language_support(&ext)
}
```

### File: `src/cache.rs`

- Size: 19474 bytes
- Modified: 2026-02-15 06:01:16 UTC

```rust
//! Cache management for context-builder.
//!
//! This module handles caching of project states to enable the auto-diff feature.
//! It uses a hash of the project path and configuration to avoid cache collisions
//! between different projects or configurations.

use fs2::FileExt;

use std::fs;
use std::fs::File;

use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use crate::config::Config;
use crate::state::ProjectState;

/// Manages cache operations with file locking to prevent corruption
pub struct CacheManager {
    cache_dir: PathBuf,
    project_hash: String,
    config_hash: String,
}

impl CacheManager {
    /// Create a new cache manager for the given project path and configuration
    pub fn new(project_path: &Path, config: &Config) -> Self {
        // Normalize the project path first for consistency
        let normalized_project_path = Self::normalize_project_path(project_path);

        let project_hash = Self::hash_path(&normalized_project_path);
        let config_hash = Self::hash_config(config);

        // Ensure cache directory exists relative to normalized project root
        let cache_dir = normalized_project_path
            .join(".context-builder")
            .join("cache");
        if !cache_dir.exists() {
            let _ = fs::create_dir_all(&cache_dir);
        }

        let cache_manager = Self {
            cache_dir,
            project_hash,
            config_hash,
        };

        // Migrate old cache format if present
        cache_manager.migrate_old_cache();

        cache_manager
    }

    /// Normalize project path for consistent hashing and cache directory creation
    fn normalize_project_path(path: &Path) -> PathBuf {
        // Always resolve to absolute path first
        let absolute_path = if path.is_absolute() {
            path.to_path_buf()
        } else {
            match std::env::current_dir() {
                Ok(cwd) => cwd.join(path),
                Err(_) => path.to_path_buf(),
            }
        };

        // Try to canonicalize for consistency, but normalize the result
        if let Ok(canonical) = absolute_path.canonicalize() {
            Self::normalize_path_format(&canonical)
        } else {
            absolute_path
        }
    }

    /// Generate a hash from the normalized project path
    fn hash_path(path: &Path) -> String {
        let path_str = path.to_string_lossy();
        let hash = xxhash_rust::xxh3::xxh3_64(path_str.as_bytes());
        format!("{:x}", hash)
    }

    /// Normalize path format to handle Windows UNC prefixes
    fn normalize_path_format(path: &Path) -> PathBuf {
        let path_str = path.to_string_lossy();

        // Remove Windows UNC prefix if present
        if cfg!(windows) && path_str.starts_with("\\\\?\\") {
            PathBuf::from(&path_str[4..])
        } else {
            path.to_path_buf()
        }
    }

    /// Generate a hash from the configuration
    fn hash_config(config: &Config) -> String {
        // Build a stable string representation of config for hashing.
        // IMPORTANT: Must stay in sync with state.rs::compute_config_hash
        let mut config_str = String::new();
        if let Some(ref filters) = config.filter {
            config_str.push_str(&filters.join(","));
        }
        config_str.push('|');
        if let Some(ref ignores) = config.ignore {
            config_str.push_str(&ignores.join(","));
        }
        config_str.push('|');
        config_str.push_str(&format!(
            "{:?}|{:?}|{:?}",
            config.line_numbers, config.auto_diff, config.diff_context_lines
        ));
        let hash = xxhash_rust::xxh3::xxh3_64(config_str.as_bytes());
        format!("{:x}", hash)
    }

    /// Get the cache file path for this specific project and configuration
    fn get_cache_path(&self) -> PathBuf {
        self.cache_dir.join(format!(
            "state_{}_{}.json",
            self.project_hash, self.config_hash
        ))
    }

    /// Public helper primarily for debugging/tests to inspect the resolved cache path
    pub fn debug_cache_file_path(&self) -> PathBuf {
        self.get_cache_path()
    }

    /// Migrate old markdown-based cache files to new JSON format
    fn migrate_old_cache(&self) {
        let old_cache_patterns = ["last_canonical.md", "last_output.md", "current_output.md"];

        for pattern in &old_cache_patterns {
            let old_cache_path = self.cache_dir.join(pattern);
            if old_cache_path.exists() {
                eprintln!("Migrating old cache format: removing {}", pattern);
                let _ = fs::remove_file(&old_cache_path);
            }
        }

        // Also remove any files that look like timestamped outputs from old versions
        if let Ok(entries) = fs::read_dir(&self.cache_dir) {
            for entry in entries.flatten() {
                let file_name = entry.file_name();
                let name = file_name.to_string_lossy();
                if name.ends_with(".md") && (name.contains("_20") || name.starts_with("output_")) {
                    eprintln!("Migrating old cache format: removing {}", name);
                    let _ = fs::remove_file(entry.path());
                }
            }
        }
    }

    /// Read the cached project state with file locking
    pub fn read_cache(&self) -> Result<Option<ProjectState>, Box<dyn std::error::Error>> {
        let cache_path = self.get_cache_path();

        if !cache_path.exists() {
            return Ok(None);
        }

        let file = File::open(&cache_path)?;
        // Acquire shared lock to prevent reading while writing
        file.lock_shared()?;

        let mut contents = String::new();
        let mut file = std::io::BufReader::new(file);
        file.read_to_string(&mut contents)?;

        // Release lock
        file.get_ref().unlock()?;

        let state: ProjectState = serde_json::from_str(&contents)?;
        Ok(Some(state))
    }

    /// Write the project state to cache with file locking
    pub fn write_cache(&self, state: &ProjectState) -> Result<(), Box<dyn std::error::Error>> {
        let cache_path = self.get_cache_path();

        let file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(false)
            .open(&cache_path)?;
        // Acquire exclusive lock BEFORE truncating to prevent TOCTOU races
        file.lock_exclusive()?;
        file.set_len(0)?;

        let json = serde_json::to_string_pretty(state)?;
        let mut file = std::io::BufWriter::new(file);
        file.write_all(json.as_bytes())?;
        file.flush()?;

        // Release lock
        file.get_ref().unlock()?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    use tempfile::tempdir;

    #[test]
    fn test_hash_path() {
        let path1 = Path::new("/project1");
        let path2 = Path::new("/project2");

        let hash1 = CacheManager::hash_path(path1);
        let hash2 = CacheManager::hash_path(path2);

        assert_ne!(
            hash1, hash2,
            "Different paths should produce different hashes"
        );
    }

    #[test]
    fn test_hash_config() {
        let config1 = Config {
            filter: Some(vec!["rs".to_string()]),
            ignore: Some(vec!["target".to_string()]),
            line_numbers: Some(true),
            ..Default::default()
        };

        let config2 = Config {
            filter: Some(vec!["md".to_string()]),
            ignore: Some(vec!["target".to_string()]),
            line_numbers: Some(true),
            ..Default::default()
        };

        let hash1 = CacheManager::hash_config(&config1);
        let hash2 = CacheManager::hash_config(&config2);

        assert_ne!(
            hash1, hash2,
            "Different configs should produce different hashes"
        );
    }

    #[test]
    fn test_cache_operations() {
        let dir = tempdir().unwrap();
        let project_path = dir.path().join("test_project");
        let _ = fs::create_dir(&project_path);

        let config = Config::default();
        let cache_manager = CacheManager::new(&project_path, &config);

        use crate::state::ProjectMetadata;

        let state = ProjectState {
            timestamp: "2023-01-01T00:00:00Z".to_string(),
            config_hash: "test_config_hash".to_string(),
            files: std::collections::BTreeMap::new(),
            metadata: ProjectMetadata {
                project_name: "test".to_string(),
                file_count: 0,
                filters: vec![],
                ignores: vec![],
                line_numbers: false,
            },
        };

        // Write cache
        assert!(cache_manager.write_cache(&state).is_ok());

        // Read cache
        let cached_state = cache_manager.read_cache().unwrap();
        assert!(cached_state.is_some());
        assert_eq!(cached_state.unwrap().timestamp, state.timestamp);
    }
```

### File: `src/cli.rs`

- Size: 6553 bytes
- Modified: 2026-02-15 06:50:55 UTC

```rust
use clap::Parser;

/// CLI tool to aggregate directory contents into a single Markdown file optimized for LLM consumption
#[derive(Parser, Debug, Clone)]
#[clap(author, version, about)]
pub struct Args {
    /// Directory path to process
    #[clap(short = 'd', long, default_value = ".")]
    pub input: String,

    /// Output file path
    #[clap(short, long, default_value = "output.md")]
    pub output: String,

    /// File extensions to include (e.g., --filter rs,toml)
    #[clap(short = 'f', long, value_delimiter = ',')]
    pub filter: Vec<String>,

    /// Folder or file names to ignore (e.g., --ignore target --ignore lock)
    #[clap(short = 'i', long)]
    pub ignore: Vec<String>,

    /// Preview mode: only print the file tree to the console, don't generate the documentation file
    #[clap(long)]
    pub preview: bool,

    /// Token count mode: estimate the total token count of the final document
    #[clap(long)]
    pub token_count: bool,

    /// Add line numbers to code blocks in the output
    #[clap(long)]
    pub line_numbers: bool,

    /// Automatically answer yes to all prompts
    #[clap(short = 'y', long)]
    pub yes: bool,

    /// Maximum token budget for the output. Files are truncated/skipped when exceeded.
    #[clap(long)]
    pub max_tokens: Option<usize>,

    /// Output only diffs (omit full file contents; requires auto-diff & timestamped output)
    #[clap(long, default_value_t = false)]
    pub diff_only: bool,

    /// Clear the cached project state and exit
    #[clap(long)]
    pub clear_cache: bool,

    /// Initialize a new context-builder.toml config file in the current directory
    #[clap(long)]
    pub init: bool,

    /// Extract function/class signatures only (requires tree-sitter feature)
    #[clap(long)]
    pub signatures: bool,

    /// Extract code structure (imports, exports, symbol counts) - requires tree-sitter feature
    #[clap(long)]
    pub structure: bool,

    /// Truncation mode for max-tokens: "smart" (AST boundaries) or "byte"
    #[clap(long, value_name = "MODE", default_value = "smart")]
    pub truncate: String,

    /// Filter signatures by visibility: "all", "public", or "private"
    #[clap(long, default_value = "all")]
    pub visibility: String,
}

#[cfg(test)]
mod tests {
    use super::Args;
    use clap::Parser;

    #[test]
    fn parses_with_no_args() {
        let res = Args::try_parse_from(["context-builder"]);
        assert!(res.is_ok(), "Expected success when no args are provided");
    }
```

### File: `src/config.rs`

- Size: 8954 bytes
- Modified: 2026-02-15 06:51:48 UTC

```rust
use serde::Deserialize;
use std::fs;
use std::path::Path;

/// Global configuration loaded from `context-builder.toml`.
///
/// Any field left as `None` means "use the CLI default / do not override".
/// Command-line arguments always take precedence over values provided here.
///
/// Example `context-builder.toml`:
/// ```toml
/// output = "context.md"
/// output_folder = "docs"
/// timestamped_output = true
/// auto_diff = true
/// diff_only = true         # Emit only change summary + modified file diffs (no full file bodies)
/// filter = ["rs", "toml"]
/// ignore = ["target", ".git"]
/// line_numbers = false
/// diff_context_lines = 5
/// ```
///
#[derive(Deserialize, Debug, Default, Clone)]
pub struct Config {
    /// Output file name (or base name when `timestamped_output = true`)
    pub output: Option<String>,

    /// File extensions to include (no leading dot, e.g. `rs`, `toml`)
    pub filter: Option<Vec<String>>,

    /// File / directory names to ignore (exact name matches)
    pub ignore: Option<Vec<String>>,

    /// Add line numbers to code blocks
    pub line_numbers: Option<bool>,

    /// Preview only the file tree (no file output)
    pub preview: Option<bool>,

    /// Token counting mode
    pub token_count: Option<bool>,

    /// Optional folder to place the generated output file(s) in
    pub output_folder: Option<String>,

    /// If true, append a UTC timestamp to the output file name (before extension)
    pub timestamped_output: Option<bool>,

    /// Assume "yes" for overwrite / processing confirmations
    pub yes: Option<bool>,

    /// Enable automatic diff generation (requires `timestamped_output = true`)
    pub auto_diff: Option<bool>,

    /// Override number of unified diff context lines (falls back to env or default = 3)
    pub diff_context_lines: Option<usize>,

    /// When true, emit ONLY:
    /// - Header + file tree
    /// - Change Summary
    /// - Per-file diffs for modified files
    ///
    /// Excludes full file contents section entirely. Added files appear only in the
    /// change summary (and are marked Added) but their full content is omitted.
    pub diff_only: Option<bool>,

    /// Encoding handling strategy for non-UTF-8 files.
    /// - "detect": Attempt to detect and transcode to UTF-8 (default)
    /// - "strict": Only include valid UTF-8 files, skip others
    /// - "skip": Skip all non-UTF-8 files without transcoding attempts
    pub encoding_strategy: Option<String>,

    /// Maximum token budget for the output. Files are truncated/skipped when exceeded.
    pub max_tokens: Option<usize>,

    /// Extract function/class signatures only (requires tree-sitter feature)
    pub signatures: Option<bool>,

    /// Extract code structure (imports, exports, symbol counts) - requires tree-sitter feature
    pub structure: Option<bool>,

    /// Truncation mode for max-tokens: "smart" (AST boundaries) or "byte"
    pub truncate: Option<String>,

    /// Filter signatures by visibility: "all", "public", or "private"
    pub visibility: Option<String>,
}

/// Load configuration from `context-builder.toml` in the current working directory.
/// Returns `None` if the file does not exist or cannot be parsed.
pub fn load_config() -> Option<Config> {
    let config_path = Path::new("context-builder.toml");
    if config_path.exists() {
        let content = fs::read_to_string(config_path).ok()?;
        match toml::from_str(&content) {
            Ok(config) => Some(config),
            Err(e) => {
                eprintln!(
                    "⚠️  Failed to parse context-builder.toml: {}. Config will be ignored.",
                    e
                );
                None
            }
        }
    } else {
        None
    }
}
```

### File: `src/config_resolver.rs`

- Size: 15995 bytes
- Modified: 2026-02-15 06:55:40 UTC

```rust
//! Configuration resolution module for context-builder.
//!
//! This module provides centralized logic for merging CLI arguments with configuration
//! file values, implementing proper precedence rules and handling complex scenarios
//! like timestamping and output folder resolution.

use chrono::Utc;
use std::path::{Path, PathBuf};

use crate::cli::Args;
use crate::config::Config;

/// Resolved configuration combining CLI arguments and config file values
#[derive(Debug, Clone)]
pub struct ResolvedConfig {
    pub input: String,
    pub output: String,
    pub filter: Vec<String>,
    pub ignore: Vec<String>,
    pub line_numbers: bool,
    pub preview: bool,
    pub token_count: bool,
    pub yes: bool,
    pub diff_only: bool,
    pub clear_cache: bool,
    pub auto_diff: bool,
    pub diff_context_lines: usize,
    pub max_tokens: Option<usize>,
    pub init: bool,
    pub signatures: bool,
    pub structure: bool,
    pub truncate: String,
    pub visibility: String,
}

/// Result of configuration resolution including the final config and any warnings
#[derive(Debug)]
pub struct ConfigResolution {
    pub config: ResolvedConfig,
    pub warnings: Vec<String>,
}

/// Resolves final configuration by merging CLI arguments with config file values.
///
/// Precedence rules (highest to lowest):
/// 1. Explicit CLI arguments (non-default values)
/// 2. Configuration file values
/// 3. CLI default values
///
/// Special handling:
/// - `output` field supports timestamping and output folder resolution
/// - Boolean flags respect explicit CLI usage vs defaults
/// - Arrays (filter, ignore) use CLI if non-empty, otherwise config file
pub fn resolve_final_config(mut args: Args, config: Option<Config>) -> ConfigResolution {
    let mut warnings = Vec::new();

    // Start with CLI defaults, then apply config file, then explicit CLI overrides
    let final_config = if let Some(config) = config {
        apply_config_to_args(&mut args, &config, &mut warnings);
        resolve_output_path(&mut args, &config, &mut warnings);
        config
    } else {
        Config::default()
    };

    let resolved = ResolvedConfig {
        input: args.input,
        output: args.output,
        filter: args.filter,
        ignore: args.ignore,
        line_numbers: args.line_numbers,
        preview: args.preview,
        token_count: args.token_count,
        yes: args.yes,
        diff_only: args.diff_only,
        clear_cache: args.clear_cache,
        auto_diff: final_config.auto_diff.unwrap_or(false),
        diff_context_lines: final_config.diff_context_lines.unwrap_or(3),
        max_tokens: args.max_tokens.or(final_config.max_tokens),
        init: args.init,
        signatures: args.signatures || final_config.signatures.unwrap_or(false),
        structure: args.structure || final_config.structure.unwrap_or(false),
        truncate: if args.truncate != "smart" {
            args.truncate.clone()
        } else {
            final_config
                .truncate
                .clone()
                .unwrap_or_else(|| args.truncate.clone())
        },
        visibility: if args.visibility != "all" {
            args.visibility.clone()
        } else {
            final_config
                .visibility
                .clone()
                .unwrap_or_else(|| args.visibility.clone())
        },
    };

    ConfigResolution {
        config: resolved,
        warnings,
    }
}

/// Apply configuration file values to CLI arguments based on precedence rules
fn apply_config_to_args(args: &mut Args, config: &Config, warnings: &mut Vec<String>) {
    // Output: only apply config if CLI is using default value
    if args.output == "output.md"
        && let Some(ref output) = config.output
    {
        args.output = output.clone();
    }

    // Filter: CLI takes precedence if non-empty
    if args.filter.is_empty()
        && let Some(ref filter) = config.filter
    {
        args.filter = filter.clone();
    }

    // Ignore: CLI takes precedence if non-empty
    if args.ignore.is_empty()
        && let Some(ref ignore) = config.ignore
    {
        args.ignore = ignore.clone();
    }

    // Boolean flags: config applies only if CLI is using default (false)
    // Note: We can't distinguish between explicit --no-flag and default false,
    // so config file can only enable features, not disable them
    if !args.line_numbers
        && let Some(line_numbers) = config.line_numbers
    {
        args.line_numbers = line_numbers;
    }

    if !args.preview
        && let Some(preview) = config.preview
    {
        args.preview = preview;
    }

    if !args.token_count
        && let Some(token_count) = config.token_count
    {
        args.token_count = token_count;
    }

    if !args.yes
        && let Some(yes) = config.yes
    {
        args.yes = yes;
    }

    // diff_only: config can enable it, but CLI flag always takes precedence
    if !args.diff_only
        && let Some(true) = config.diff_only
    {
        args.diff_only = true;
    }

    // Validate auto_diff configuration
    if let Some(true) = config.auto_diff
        && config.timestamped_output != Some(true)
    {
        warnings.push(
            "auto_diff is enabled but timestamped_output is not enabled. \
            Auto-diff requires timestamped_output = true to function properly."
                .to_string(),
        );
    }
}

/// Resolve output path including timestamping and output folder logic
fn resolve_output_path(args: &mut Args, config: &Config, warnings: &mut Vec<String>) {
    let mut output_folder_path: Option<PathBuf> = None;

    // Apply output folder first
    if let Some(ref output_folder) = config.output_folder {
        let mut path = PathBuf::from(output_folder);
        path.push(&args.output);
        args.output = path.to_string_lossy().to_string();
        output_folder_path = Some(PathBuf::from(output_folder));
    }

    // Apply timestamping if enabled
    if let Some(true) = config.timestamped_output {
        let timestamp = Utc::now().format("%Y%m%d%H%M%S").to_string();
        let path = Path::new(&args.output);

        let stem = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("output");

        let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("md");

        let new_filename = format!("{}_{}.{}", stem, timestamp, extension);

        if let Some(output_folder) = output_folder_path {
            args.output = output_folder
                .join(new_filename)
                .to_string_lossy()
                .to_string();
        } else {
            let new_path = path.with_file_name(new_filename);
            args.output = new_path.to_string_lossy().to_string();
        }
    }

    // Validate output folder exists if specified
    if let Some(ref output_folder) = config.output_folder {
        let folder_path = Path::new(output_folder);
        if !folder_path.exists() {
            warnings.push(format!(
                "Output folder '{}' does not exist. It will be created if possible.",
                output_folder
            ));
        }
    }
}
```

### File: `src/diff.rs`

- Size: 21233 bytes
- Modified: 2026-02-15 04:50:34 UTC

```rust
use similar::{ChangeTag, TextDiff};
use std::collections::HashMap;

/// Line based diff utilities.
///
/// This module previously exposed `generate_diff` which produced a single
/// "## File Differences" section for an entire markdown document. That
/// approach made it easy for volatile sections (timestamps, file tree
/// structure, etc.) to create noisy diffs. To address this the new
/// per‑file API lets the caller diff only the normalized *file content*
/// blocks that appear under each `### File: `path`` heading in the
/// canonical output, completely ignoring the global header or the file
/// tree portion. Each file receives an isolated unified style diff.
///
/// High level additions:
/// * `PerFileStatus` – classification of the change.
/// * `PerFileDiff` – structured diff result for a single file.
/// * `diff_file_contents` – core engine producing diffs per file without any
///   global "## File Differences" header.
/// * `render_per_file_diffs` – helper to render the per file diffs into
///   markdown (still omits a global header so the caller can choose).
///
/// Backwards compatibility: the existing `generate_diff` function (full
/// document diff) is retained for now. New code should prefer the
/// per‑file functions.
/// Determine number of context lines either from explicit argument or env.
fn resolve_context_lines(explicit: Option<usize>) -> usize {
    explicit
        .filter(|v| *v > 0)
        .or_else(|| {
            std::env::var("CB_DIFF_CONTEXT_LINES")
                .ok()
                .and_then(|v| v.parse().ok())
                .filter(|v: &usize| *v > 0)
        })
        .unwrap_or(3)
}

/// Original API: produce a single markdown section headed by "## File Differences".
/// (Kept unchanged for compatibility.)
pub fn generate_diff(old_content: &str, new_content: &str) -> String {
    let diff = TextDiff::from_lines(old_content, new_content);
    if diff.ratio() == 1.0 {
        return String::new();
    }
    let context_lines = resolve_context_lines(None);
    let grouped = diff.grouped_ops(context_lines);
    let mut out = String::new();
    out.push_str("## File Differences\n\n");
    out.push_str("```diff\n");
    for (group_index, group) in grouped.iter().enumerate() {
        if group_index > 0 {
            out.push_str("  ...\n");
        }
        // Emit standard unified diff hunk header for positional context
        if let (Some(first), Some(last)) = (group.first(), group.last()) {
            let old_start = first.old_range().start + 1;
            let old_len = last.old_range().end - first.old_range().start;
            let new_start = first.new_range().start + 1;
            let new_len = last.new_range().end - first.new_range().start;
            out.push_str(&format!(
                "@@ -{},{} +{},{} @@\n",
                old_start, old_len, new_start, new_len
            ));
        }
        for op in group {
            for change in diff.iter_changes(op) {
                let tag = change.tag();
                let mut line = change.to_string();
                if line.ends_with('\n') {
                    line.pop();
                    if line.ends_with('\r') {
                        line.pop();
                    }
                }

                match tag {
                    ChangeTag::Delete => {
                        out.push_str("- ");
                        out.push_str(&line);
                        out.push('\n');
                    }
                    ChangeTag::Insert => {
                        out.push_str("+ ");
                        out.push_str(&line);
                        out.push('\n');
                    }
                    ChangeTag::Equal => {
                        out.push_str("  ");
                        out.push_str(&line);
                        out.push('\n');
                    }
                }
            }
        }
    }
    out.push_str("```\n\n");
    out
}

/// Classification of how a file changed between two snapshots.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PerFileStatus {
    Added,
    Removed,
    Modified,
    Unchanged,
}

/// Structured diff result for a single file.
#[derive(Debug, Clone)]
pub struct PerFileDiff {
    pub path: String,
    pub status: PerFileStatus,
    /// Unified diff fenced in ```diff (omitted when status == Unchanged and skip_unchanged=true)
    pub diff: String,
}

impl PerFileDiff {
    pub fn is_changed(&self) -> bool {
        self.status != PerFileStatus::Unchanged
    }
}

/// Produce a unified style diff for two text blobs WITHOUT adding any global
/// section header. Returns empty string if contents are identical.
fn unified_no_header(old: &str, new: &str, context_lines: usize) -> String {
    let diff = TextDiff::from_lines(old, new);
    if diff.ratio() == 1.0 {
        return String::new();
    }
    let grouped = diff.grouped_ops(context_lines);
    let mut out = String::new();
    out.push_str("```diff\n");
    for (group_index, group) in grouped.iter().enumerate() {
        if group_index > 0 {
            out.push_str("  ...\n");
        }
        // Emit standard unified diff hunk header for positional context
        if let (Some(first), Some(last)) = (group.first(), group.last()) {
            let old_start = first.old_range().start + 1;
            let old_len = last.old_range().end - first.old_range().start;
            let new_start = first.new_range().start + 1;
            let new_len = last.new_range().end - first.new_range().start;
            out.push_str(&format!(
                "@@ -{},{} +{},{} @@\n",
                old_start, old_len, new_start, new_len
            ));
        }
        for op in group {
            for change in diff.iter_changes(op) {
                let tag = change.tag();
                let mut line = change.to_string();
                if line.ends_with('\n') {
                    line.pop();
                    if line.ends_with('\r') {
                        line.pop();
                    }
                }

                match tag {
                    ChangeTag::Delete => {
                        out.push_str("- ");
                        out.push_str(&line);
                        out.push('\n');
                    }
                    ChangeTag::Insert => {
                        out.push_str("+ ");
                        out.push_str(&line);
                        out.push('\n');
                    }
                    ChangeTag::Equal => {
                        out.push_str("  ");
                        out.push_str(&line);
                        out.push('\n');
                    }
                }
            }
        }
    }
    out.push_str("```\n");
    out
}

/// Diff per file content sets.
///
/// Inputs are maps keyed by file path (relative or absolute – caller decides)
/// with values being the raw file content EXACTLY as you wish it to be diffed
/// (e.g. already stripped of volatile metadata, no size/modified lines, only
/// the real file body). This keeps higher level logic (parsing the markdown
/// document) out of the diff layer.
///
/// Returns a vector of `PerFileDiff` for every file that is Added, Removed,
/// or Modified. Unchanged files are omitted by default (`skip_unchanged=true`)
/// to reduce noise, but you can opt to include them.
pub fn diff_file_contents(
    previous: &HashMap<String, String>,
    current: &HashMap<String, String>,
    skip_unchanged: bool,
    explicit_context: Option<usize>,
) -> Vec<PerFileDiff> {
    let mut all_paths: Vec<String> = previous.keys().chain(current.keys()).cloned().collect();
    all_paths.sort();
    all_paths.dedup();

    let context_lines = resolve_context_lines(explicit_context);
    let mut results = Vec::new();

    for path in all_paths {
        let old_opt = previous.get(&path);
        let new_opt = current.get(&path);
        match (old_opt, new_opt) {
            (None, Some(new_content)) => {
                // Added file: present only in current snapshot
                let mut diff = String::new();
                diff.push_str("```diff\n");
                for line in new_content.lines() {
                    diff.push_str("+ ");
                    diff.push_str(line);
                    diff.push('\n');
                }
                diff.push_str("```\n");
                results.push(PerFileDiff {
                    path,
                    status: PerFileStatus::Added,
                    diff,
                });
            }
            (Some(_old_content), None) => {
                // Removed file
                let old_content = previous.get(&path).unwrap();
                let mut diff = String::new();
                diff.push_str("```diff\n");
                for line in old_content.lines() {
                    diff.push_str("- ");
                    diff.push_str(line);
                    diff.push('\n');
                }
                diff.push_str("```\n");
                results.push(PerFileDiff {
                    path,
                    status: PerFileStatus::Removed,
                    diff,
                });
            }
            (Some(old_content), Some(new_content)) => {
                if old_content == new_content {
                    if !skip_unchanged {
                        results.push(PerFileDiff {
                            path,
                            status: PerFileStatus::Unchanged,
                            diff: String::new(),
                        });
                    }
                } else {
                    let diff = unified_no_header(old_content, new_content, context_lines);
                    results.push(PerFileDiff {
                        path,
                        status: PerFileStatus::Modified,
                        diff,
                    });
                }
            }
            (None, None) => unreachable!(),
        }
    }

    results
}
```

### File: `src/file_utils.rs`

- Size: 23556 bytes
- Modified: 2026-02-15 04:50:12 UTC

```rust
use ignore::{DirEntry, WalkBuilder, overrides::OverrideBuilder};
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

/// Returns a numeric category for file relevance ordering.
/// Lower numbers appear first in output. Categories:
/// 0 = Project config + key docs (Cargo.toml, README.md, AGENTS.md, etc.)
/// 1 = Source code (src/, lib/) — entry points sorted first within category
/// 2 = Tests and benchmarks (tests/, benches/, test/, spec/)
/// 3 = Documentation, scripts, and everything else
/// 4 = Generated/lock files (Cargo.lock, package-lock.json, etc.)
/// 5 = Build/CI infrastructure (.github/, .circleci/, Dockerfile, etc.)
fn file_relevance_category(path: &Path, base_path: &Path) -> u8 {
    let relative = path.strip_prefix(base_path).unwrap_or(path);
    let rel_str = relative.to_string_lossy();

    // Check filename for lockfiles first — these are lowest priority
    if let Some(name) = relative.file_name().and_then(|n| n.to_str()) {
        let lockfile_names = [
            "Cargo.lock",
            "package-lock.json",
            "yarn.lock",
            "pnpm-lock.yaml",
            "Gemfile.lock",
            "poetry.lock",
            "composer.lock",
            "go.sum",
            "bun.lockb",
            "flake.lock",
        ];
        if lockfile_names.contains(&name) {
            return 5;
        }

        // Check for config/manifest files + key project docs — highest priority
        let config_names = [
            // Package manifests
            "Cargo.toml",
            "package.json",
            "tsconfig.json",
            "pyproject.toml",
            "setup.py",
            "setup.cfg",
            "go.mod",
            "Gemfile",
            // Tool config
            "context-builder.toml",
            ".gitignore",
            // Key project documentation (LLMs need these for context)
            "README.md",
            "README",
            "README.txt",
            "README.rst",
            "AGENTS.md",
            "CLAUDE.md",
            "GEMINI.md",
            "COPILOT.md",
            "CONTRIBUTING.md",
            "CHANGELOG.md",
        ];
        if config_names.contains(&name) {
            return 0;
        }
    }

    // Check path prefix for category
    let first_component = relative
        .components()
        .next()
        .and_then(|c| c.as_os_str().to_str())
        .unwrap_or("");

    match first_component {
        "src" | "lib" | "crates" | "packages" | "internal" | "cmd" | "pkg" => {
            // Check sub-components for test directories within source trees.
            // e.g., src/tests/auth.rs should be cat 2 (tests), not cat 1 (source).
            let sub_path = rel_str.as_ref();
            if sub_path.contains("/tests/")
                || sub_path.contains("/test/")
                || sub_path.contains("/spec/")
                || sub_path.contains("/__tests__/")
                || sub_path.contains("/benches/")
                || sub_path.contains("/benchmarks/")
            {
                2
            } else {
                1
            }
        }
        "tests" | "test" | "spec" | "benches" | "benchmarks" | "__tests__" => 2,
        "docs" | "doc" | "examples" | "scripts" | "tools" | "assets" => 3,
        // Build/CI infrastructure — useful context but not core source
        ".github" | ".circleci" | ".gitlab" | ".buildkite" => 4,
        _ => {
            // Check extensions for additional heuristics
            if let Some(ext) = relative.extension().and_then(|e| e.to_str()) {
                match ext {
                    "rs" | "go" | "py" | "ts" | "js" | "java" | "c" | "cpp" | "h" | "hpp"
                    | "rb" | "swift" | "kt" | "scala" | "ex" | "exs" | "zig" | "hs" => {
                        // Source file not in a recognized dir — check if it's a test
                        // Use path boundaries to avoid false positives (e.g., "contest.rs")
                        if rel_str.contains("/test/")
                            || rel_str.contains("/tests/")
                            || rel_str.contains("/spec/")
                            || rel_str.contains("/__tests__/")
                            || rel_str.ends_with("_test.rs")
                            || rel_str.ends_with("_test.go")
                            || rel_str.ends_with("_spec.rb")
                            || rel_str.ends_with(".test.ts")
                            || rel_str.ends_with(".test.js")
                            || rel_str.ends_with(".spec.ts")
                            || rel_str.starts_with("test_")
                        {
                            2
                        } else {
                            1
                        }
                    }
                    "md" | "txt" | "rst" | "adoc" => 3,
                    _ => 1, // Unknown extension in root — treat as source
                }
            } else {
                // Check for build-related root files without extensions
                if let Some(
                    "Makefile" | "CMakeLists.txt" | "Dockerfile" | "Containerfile" | "Justfile"
                    | "Taskfile" | "Rakefile" | "Vagrantfile",
                ) = relative.file_name().and_then(|n| n.to_str())
                {
                    4
                } else {
                    3 // No extension — docs/other
                }
            }
        }
    }
}

/// Returns a sub-priority for sorting within the same relevance category.
/// Lower values appear first. Entry points (main, lib, mod) get priority 0,
/// other files get priority 1. This ensures LLMs see architectural entry
/// points before helper modules.
fn file_entry_point_priority(path: &Path) -> u8 {
    if let Some("main" | "lib" | "mod" | "index" | "app" | "__init__") =
        path.file_stem().and_then(|s| s.to_str())
    {
        0
    } else {
        1
    }
}

/// Collects all files to be processed using `ignore` crate for efficient traversal.
///
/// `auto_ignores` are runtime-computed exclusion patterns (e.g., the tool's own
/// output file or cache directory). They are processed identically to user ignores
/// but kept separate to avoid polluting user-facing configuration.
pub fn collect_files(
    base_path: &Path,
    filters: &[String],
    ignores: &[String],
    auto_ignores: &[String],
) -> io::Result<Vec<DirEntry>> {
    let mut walker = WalkBuilder::new(base_path);
    // By default, the "ignore" crate respects .gitignore and hidden files, so we don't need walker.hidden(false)

    // Build overrides for custom ignore patterns
    let mut override_builder = OverrideBuilder::new(base_path);

    // Hardcoded auto-ignores for common heavy directories that should NEVER be
    // included, even when there's no .git directory (so .gitignore isn't read).
    // Without these, projects missing .git can produce million-line outputs
    // from dependency trees.
    //
    // IMPORTANT: These are added FIRST so that user ignores can override them.
    // The ignore crate uses "last-match-wins" semantics, so a user can whitelist
    // a legitimate "vendor" or "build" dir by passing it as a filter pattern.
    //
    // IMPORTANT: Patterns must NOT contain a slash — the ignore crate anchors
    // slash-containing patterns to the root, so `!dir/**` would only match
    // top-level dirs, missing nested ones like `apps/web/node_modules/`.
    let default_ignores = [
        "node_modules",
        "__pycache__",
        ".venv",
        "venv",
        ".tox",
        ".mypy_cache",
        ".pytest_cache",
        ".ruff_cache",
        "vendor",  // Go, PHP, Ruby
        ".bundle", // Ruby
        "bower_components",
        ".next",       // Next.js build output
        ".nuxt",       // Nuxt build output
        ".svelte-kit", // SvelteKit build output
        ".angular",    // Angular cache
        "dist",        // Common build output
        "build",       // Common build output
        ".gradle",     // Gradle cache
        ".cargo",      // Cargo registry cache
    ];
    for dir in &default_ignores {
        // No slash in pattern → matches at any depth (not root-anchored)
        let pattern = format!("!{}", dir);
        if let Err(e) = override_builder.add(&pattern) {
            log::warn!("Skipping invalid default-ignore '{}': {}", dir, e);
        }
    }

    // User-specified ignore patterns (added AFTER defaults so they can override)
    for pattern in ignores {
        // Attention: Confusing pattern ahead!
        // Add the pattern to the override builder with ! prefix to ignore matching files.
        // In OverrideBuilder, patterns without ! are whitelist (include) patterns,
        // while patterns with ! are ignore patterns.
        let ignore_pattern = format!("!{}", pattern);
        if let Err(e) = override_builder.add(&ignore_pattern) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Invalid ignore pattern '{}': {}", pattern, e),
            ));
        }
    }
    // Apply auto-computed ignore patterns (output file, cache dir, etc.)
    for pattern in auto_ignores {
        let ignore_pattern = format!("!{}", pattern);
        if let Err(e) = override_builder.add(&ignore_pattern) {
            log::warn!("Skipping invalid auto-ignore pattern '{}': {}", pattern, e);
        }
    }
    // Also, always ignore the config file itself
    if let Err(e) = override_builder.add("!context-builder.toml") {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("Failed to add config ignore: {}", e),
        ));
    }

    let overrides = override_builder.build().map_err(|e| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("Failed to build overrides: {}", e),
        )
    })?;
    walker.overrides(overrides);

    if !filters.is_empty() {
        let mut type_builder = ignore::types::TypesBuilder::new();
        type_builder.add_defaults();
        for filter in filters {
            let _ = type_builder.add(filter, &format!("*.{}", filter));
            type_builder.select(filter);
        }
        let types = type_builder.build().unwrap();
        walker.types(types);
    }

    let mut files: Vec<DirEntry> = walker
        .build()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_some_and(|ft| ft.is_file()))
        .collect();

    // Sort files by relevance category, then entry-point priority, then alphabetically.
    // This puts config + docs first, then source code (entry points before helpers),
    // then tests, then docs/other, then build/CI, then lockfiles.
    // LLMs comprehend codebases better when core source appears before test scaffolding.
    files.sort_by(|a, b| {
        let cat_a = file_relevance_category(a.path(), base_path);
        let cat_b = file_relevance_category(b.path(), base_path);
        cat_a
            .cmp(&cat_b)
            .then_with(|| {
                file_entry_point_priority(a.path()).cmp(&file_entry_point_priority(b.path()))
            })
            .then_with(|| a.path().cmp(b.path()))
    });

    Ok(files)
}

/// Asks for user confirmation if the number of files is large.
pub fn confirm_processing(file_count: usize) -> io::Result<bool> {
    if file_count > 100 {
        print!(
            "Warning: You're about to process {} files. This might take a while. Continue? [y/N] ",
            file_count
        );
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        if !input.trim().eq_ignore_ascii_case("y") {
            return Ok(false);
        }
    }
    Ok(true)
}
```

### File: `src/markdown.rs`

- Size: 45629 bytes
- Modified: 2026-02-15 07:54:09 UTC

```rust
use chrono::Utc;
use ignore::DirEntry;
use log::{error, info, warn};
use std::fs;
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::path::Path;

use crate::tree::{FileTree, write_tree_to_file};
use encoding_rs::{Encoding, UTF_8};

#[cfg(feature = "parallel")]
use crossbeam_channel::{Receiver, Sender, bounded};
#[cfg(feature = "parallel")]
use std::thread;

/// Configuration for tree-sitter powered output.
#[derive(Debug, Clone, Default)]
pub struct TreeSitterConfig {
    /// Output only signatures (function/type declarations) instead of full content.
    pub signatures: bool,
    /// Include a structure summary (counts of functions, structs, etc.) per file.
    pub structure: bool,
    /// Truncation mode: "smart" uses AST boundaries, anything else uses byte truncation.
    pub truncate: String,
    /// Visibility filter: "public", "private", or "all".
    pub visibility: String,
}

/// Generates the final Markdown file.
#[allow(clippy::too_many_arguments, unused_variables)]
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
) -> io::Result<()> {
    if let Some(parent) = Path::new(output_path).parent()
        && !parent.exists()
    {
        fs::create_dir_all(parent)?;
    }

    let mut output = fs::File::create(output_path)?;

    let input_dir_name = if input_dir == "." {
        let current_dir = std::env::current_dir()?;
        current_dir
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or_else(|| current_dir.to_str().unwrap_or("project"))
            .to_string()
    } else {
        input_dir.to_string()
    };

    // --- Header --- //
    writeln!(output, "# Directory Structure Report\n")?;

    if !filters.is_empty() {
        writeln!(
            output,
            "This document contains files from the `{}` directory with extensions: {}",
            input_dir_name,
            filters.join(", ")
        )?;
    } else {
        writeln!(
            output,
            "This document contains all files from the `{}` directory, optimized for LLM consumption.",
            input_dir_name
        )?;
    }

    if !ignores.is_empty() {
        writeln!(output, "Custom ignored patterns: {}", ignores.join(", "))?;
    }

    // Deterministic content hash (enables LLM prompt caching across runs)
    // Uses xxh3 over file content bytes — stable across Rust versions and machines.
    // Previous implementation hashed mtime (broken by git checkout, cp, etc.)
    let mut content_hasher = xxhash_rust::xxh3::Xxh3::new();
    for entry in files {
        // Hash relative unix-style path for cross-OS determinism.
        // Using absolute or OS-native paths would produce different hashes
        // on different machines or operating systems.
        let rel_path = entry.path().strip_prefix(base_path).unwrap_or(entry.path());
        let normalized = rel_path.to_string_lossy().replace('\\', "/");
        content_hasher.update(normalized.as_bytes());
        // Null delimiter prevents collision: path="a" content="bc" vs path="ab" content="c"
        content_hasher.update(b"\0");
        // Hash actual file content (not mtime!) for determinism
        if let Ok(bytes) = std::fs::read(entry.path()) {
            content_hasher.update(&bytes);
        }
        content_hasher.update(b"\0");
    }
    writeln!(output, "Content hash: {:016x}", content_hasher.digest())?;
    writeln!(output)?;

    // --- File Tree --- //

    writeln!(output, "## File Tree Structure\n")?;

    write_tree_to_file(&mut output, file_tree, 0)?;

    writeln!(output)?;

    // (No '## Files' heading here; it will be injected later only once during final composition)
    // (Diff section will be conditionally inserted later by the auto_diff logic in lib.rs)

    #[cfg(feature = "parallel")]
    {
        use rayon::prelude::*;

        // Create a bounded channel for ordered chunks
        type ChunkResult = (usize, io::Result<Vec<u8>>);
        let (sender, receiver): (Sender<ChunkResult>, Receiver<ChunkResult>) =
            bounded(num_cpus::get() * 2); // Buffer size based on CPU count

        let writer_handle = {
            let mut output = output;
            let total_files = files.len();
            let budget = max_tokens;

            thread::spawn(move || -> io::Result<()> {
                let mut completed_chunks = std::collections::BTreeMap::new();
                let mut next_index = 0;
                let mut errors = Vec::new();
                let mut tokens_used: usize = 0;
                let mut budget_exceeded = false;

                // Receive chunks and write them in order
                while next_index < total_files {
                    match receiver.recv() {
                        Ok((index, chunk_result)) => {
                            completed_chunks.insert(index, chunk_result);

                            // Write all consecutive chunks starting from next_index
                            while let Some(chunk_result) = completed_chunks.remove(&next_index) {
                                if budget_exceeded {
                                    // Already over budget — skip remaining chunks
                                    next_index += 1;
                                    continue;
                                }

                                match chunk_result {
                                    Ok(buf) => {
                                        // Estimate tokens for this chunk (~4 bytes per token)
                                        let chunk_tokens = buf.len() / 4;

                                        if let Some(max) = budget
                                            && tokens_used + chunk_tokens > max
                                            && tokens_used > 0
                                        {
                                            let remaining = total_files - next_index;
                                            let notice = format!(
                                                "---\n\n_⚠️ Token budget ({}) reached. {} remaining files omitted._\n\n",
                                                max, remaining
                                            );
                                            if let Err(e) = output.write_all(notice.as_bytes()) {
                                                errors.push(format!(
                                                    "Failed to write truncation notice: {}",
                                                    e
                                                ));
                                            }
                                            budget_exceeded = true;
                                            next_index += 1;
                                            continue;
                                        }

                                        tokens_used += chunk_tokens;
                                        if let Err(e) = output.write_all(&buf) {
                                            errors.push(format!(
                                                "Failed to write output for file index {}: {}",
                                                next_index, e
                                            ));
                                        }
                                    }
                                    Err(e) => {
                                        errors.push(format!(
                                            "Failed to process file index {}: {}",
                                            next_index, e
                                        ));
                                    }
                                }
                                next_index += 1;
                            }
                        }
                        Err(_) => break, // Channel closed
                    }
                }

                if !errors.is_empty() {
                    error!(
                        "Encountered {} errors during parallel processing:",
                        errors.len()
                    );
                    for err in &errors {
                        error!("  {}", err);
                    }
                    return Err(std::io::Error::other(format!(
                        "Failed to process {} files: {}",
                        errors.len(),
                        errors.join("; ")
                    )));
                }

                Ok(())
            })
        };

        // Process files in parallel and send results to writer
        let ts_config_clone = ts_config.clone();
        files.par_iter().enumerate().for_each(|(index, entry)| {
            let mut buf = Vec::new();
            let result = process_file(
                base_path,
                entry.path(),
                &mut buf,
                line_numbers,
                encoding_strategy,
                &ts_config_clone,
            )
            .map(|_| buf);

            // Send result to writer thread (ignore send errors - channel might be closed)
            let _ = sender.send((index, result));
        });

        // Close the sender to signal completion
        drop(sender);

        // Wait for writer thread to complete and propagate any errors
        writer_handle
            .join()
            .map_err(|_| std::io::Error::other("Writer thread panicked"))??;
    }

    #[cfg(not(feature = "parallel"))]
    {
        let mut tokens_used: usize = 0;

        for (idx, entry) in files.iter().enumerate() {
            // Estimate tokens for this file (~4 bytes per token)
            let file_size = std::fs::metadata(entry.path())
                .map(|m| m.len())
                .unwrap_or(0);
            let estimated_file_tokens = (file_size as usize) / 4;

            if let Some(budget) = max_tokens {
                if tokens_used + estimated_file_tokens > budget && tokens_used > 0 {
                    let remaining = files.len() - idx;
                    writeln!(output, "---\n")?;
                    writeln!(
                        output,
                        "_⚠️ Token budget ({}) reached. {} remaining files omitted._\n",
                        budget, remaining
                    )?;
                    break;
                }
            }

            tokens_used += estimated_file_tokens;
            process_file(
                base_path,
                entry.path(),
                &mut output,
                line_numbers,
                encoding_strategy,
                ts_config,
            )?;
        }
    }

    Ok(())
}
```

### File: `src/state.rs`

- Size: 26113 bytes
- Modified: 2026-02-14 22:41:02 UTC

```rust
//! Project state representation for context-builder.
//!
//! This module provides structured data types to represent the state of a project
//! at a point in time. This replaces the previous approach of caching generated
//! markdown and enables more robust diff generation.

use chrono::Utc;
use ignore::DirEntry;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use crate::config::Config;
use crate::diff::{PerFileDiff, PerFileStatus, diff_file_contents};

/// Complete state representation of a project at a point in time
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProjectState {
    /// Timestamp when this state was captured
    pub timestamp: String,
    /// Hash of the configuration used to generate this state
    pub config_hash: String,
    /// Map of file paths to their state information
    pub files: BTreeMap<PathBuf, FileState>,
    /// Project metadata
    pub metadata: ProjectMetadata,
}

/// State information for a single file
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileState {
    /// Raw file content as string
    pub content: String,
    /// File size in bytes
    pub size: u64,
    /// Last modified time
    pub modified: SystemTime,
    /// Content hash for quick comparison
    pub content_hash: String,
}

/// Metadata about the project
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProjectMetadata {
    /// Project directory name
    pub project_name: String,
    /// Total number of files processed
    pub file_count: usize,
    /// Filters applied during processing
    pub filters: Vec<String>,
    /// Ignore patterns applied
    pub ignores: Vec<String>,
    /// Whether line numbers were enabled
    pub line_numbers: bool,
}

/// Result of comparing two project states
#[derive(Debug, Clone)]
pub struct StateComparison {
    /// Per-file differences
    pub file_diffs: Vec<PerFileDiff>,
    /// Summary of changes
    pub summary: ChangeSummary,
}

/// Summary of changes between two states
#[derive(Debug, Clone)]
pub struct ChangeSummary {
    /// Files that were added
    pub added: Vec<PathBuf>,
    /// Files that were removed
    pub removed: Vec<PathBuf>,
    /// Files that were modified
    pub modified: Vec<PathBuf>,
    /// Total number of changed files
    pub total_changes: usize,
}

impl ProjectState {
    /// Create a new project state from collected files
    pub fn from_files(
        files: &[DirEntry],
        base_path: &Path,
        config: &Config,
        line_numbers: bool,
    ) -> std::io::Result<Self> {
        let mut file_states = BTreeMap::new();

        // Ensure paths stored in the state are *always* relative (never absolute).
        // This keeps cache stable across different launch contexts and matches
        // test expectations. We attempt a few strategies to derive a relative path.
        let cwd = std::env::current_dir().unwrap_or_else(|_| base_path.to_path_buf());
        for entry in files {
            let entry_path = entry.path();

            let relative_path = entry_path
                // Preferred: relative to provided base_path (common case when input is absolute)
                .strip_prefix(base_path)
                .or_else(|_| entry_path.strip_prefix(&cwd))
                .map(|p| p.to_path_buf())
                .unwrap_or_else(|_| {
                    // Fallback: last component (file name) to avoid leaking absolute paths
                    entry_path
                        .file_name()
                        .map(PathBuf::from)
                        .unwrap_or_else(|| entry_path.to_path_buf())
                });

            let file_state = FileState::from_path(entry_path)?;
            file_states.insert(relative_path, file_state);
        }

        // Resolve project name robustly: canonicalize to handle "." and relative paths
        let canonical = base_path.canonicalize().ok();
        let resolved = canonical.as_deref().unwrap_or(base_path);
        let project_name = resolved
            .file_name()
            .and_then(|n| n.to_str())
            .map(|s| s.to_string())
            .unwrap_or_else(|| {
                // Fallback: try CWD if base_path has no file_name (e.g., root path)
                std::env::current_dir()
                    .ok()
                    .and_then(|p| {
                        p.file_name()
                            .and_then(|n| n.to_str())
                            .map(|s| s.to_string())
                    })
                    .unwrap_or_else(|| "unknown".to_string())
            });

        let metadata = ProjectMetadata {
            project_name,
            file_count: files.len(),
            filters: config.filter.clone().unwrap_or_default(),
            ignores: config.ignore.clone().unwrap_or_default(),
            line_numbers,
        };

        Ok(ProjectState {
            timestamp: Utc::now().format("%Y-%m-%d %H:%M:%S UTC").to_string(),
            config_hash: Self::compute_config_hash(config),
            files: file_states,
            metadata,
        })
    }

    /// Compare this state with a previous state
    pub fn compare_with(&self, previous: &ProjectState) -> StateComparison {
        // Convert file states to content maps for diff_file_contents
        let previous_content: std::collections::HashMap<String, String> = previous
            .files
            .iter()
            .map(|(path, state)| (path.to_string_lossy().to_string(), state.content.clone()))
            .collect();

        let current_content: std::collections::HashMap<String, String> = self
            .files
            .iter()
            .map(|(path, state)| (path.to_string_lossy().to_string(), state.content.clone()))
            .collect();

        // Generate per-file diffs
        let file_diffs = diff_file_contents(&previous_content, &current_content, true, None);

        // Generate summary
        let mut added = Vec::new();
        let mut removed = Vec::new();
        let mut modified = Vec::new();

        for diff in &file_diffs {
            let path = PathBuf::from(&diff.path);
            match diff.status {
                PerFileStatus::Added => added.push(path),
                PerFileStatus::Removed => removed.push(path),
                PerFileStatus::Modified => modified.push(path),
                PerFileStatus::Unchanged => {}
            }
        }

        let summary = ChangeSummary {
            total_changes: added.len() + removed.len() + modified.len(),
            added,
            removed,
            modified,
        };

        StateComparison {
            file_diffs,
            summary,
        }
    }

    /// Check if this state has any content changes compared to another
    pub fn has_changes(&self, other: &ProjectState) -> bool {
        if self.files.len() != other.files.len() {
            return true;
        }

        for (path, state) in &self.files {
            match other.files.get(path) {
                Some(other_state) => {
                    if state.content_hash != other_state.content_hash {
                        return true;
                    }
                }
                None => return true,
            }
        }

        false
    }

    /// Generate a configuration hash for cache validation
    fn compute_config_hash(config: &Config) -> String {
        // Build a stable string representation for hashing
        let mut config_str = String::new();
        if let Some(ref filters) = config.filter {
            config_str.push_str(&filters.join(","));
        }
        config_str.push('|');
        if let Some(ref ignores) = config.ignore {
            config_str.push_str(&ignores.join(","));
        }
        config_str.push('|');
        config_str.push_str(&format!(
            "{:?}|{:?}|{:?}",
            config.line_numbers, config.auto_diff, config.diff_context_lines
        ));

        let hash = xxhash_rust::xxh3::xxh3_64(config_str.as_bytes());
        format!("{:x}", hash)
    }
}

impl FileState {
    /// Create a file state from a file path
    pub fn from_path(path: &Path) -> std::io::Result<Self> {
        use std::fs;
        use std::io::ErrorKind;

        let metadata = fs::metadata(path)?;

        let content = match fs::read_to_string(path) {
            Ok(content) => content,
            Err(e) if e.kind() == ErrorKind::InvalidData => {
                // Handle binary files gracefully
                log::warn!("Skipping binary file in auto-diff mode: {}", path.display());
                format!("<Binary file - {} bytes>", metadata.len())
            }
            Err(e) => return Err(e),
        };

        // Compute content hash using stable xxh3
        let content_hash = format!("{:016x}", xxhash_rust::xxh3::xxh3_64(content.as_bytes()));

        Ok(FileState {
            content,
            size: metadata.len(),
            modified: metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH),
            content_hash,
        })
    }
}

impl ChangeSummary {
    /// Check if there are any changes
    pub fn has_changes(&self) -> bool {
        self.total_changes > 0
    }

    /// Generate markdown representation of the change summary
    pub fn to_markdown(&self) -> String {
        if !self.has_changes() {
            return String::new();
        }

        let mut output = String::new();
        output.push_str("## Change Summary\n\n");

        for path in &self.added {
            output.push_str(&format!("- Added: `{}`\n", path.display()));
        }

        for path in &self.removed {
            output.push_str(&format!("- Removed: `{}`\n", path.display()));
        }

        for path in &self.modified {
            output.push_str(&format!("- Modified: `{}`\n", path.display()));
        }

        output.push('\n');
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_file_state_creation() {
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        fs::write(&file_path, "Hello, world!").unwrap();

        let file_state = FileState::from_path(&file_path).unwrap();

        assert_eq!(file_state.content, "Hello, world!");
        assert_eq!(file_state.size, 13);
        assert!(!file_state.content_hash.is_empty());
    }
```

### File: `src/token_count.rs`

- Size: 9966 bytes
- Modified: 2026-02-15 07:55:43 UTC

```rust
use ignore::DirEntry;
use once_cell::sync::Lazy;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
/// Token counting utilities for estimating LLM token usage
use tiktoken_rs::{CoreBPE, cl100k_base};

// Initialize the tokenizer once and reuse it
static TOKENIZER: Lazy<CoreBPE> = Lazy::new(|| cl100k_base().unwrap());

/// Estimates the number of tokens in a text string using a real tokenizer
pub fn estimate_tokens(text: &str) -> usize {
    TOKENIZER.encode_with_special_tokens(text).len()
}

/// Counts the tokens that would be generated for a file
pub fn count_file_tokens(base_path: &Path, entry: &DirEntry, line_numbers: bool) -> usize {
    let file_path = entry.path();
    let relative_path = file_path.strip_prefix(base_path).unwrap_or(file_path);

    // Start with tokens for the file header (path, size, modified time)
    let mut token_count = estimate_tokens(&format!(
        "\n### File: `{}`\n\n- Size: {} bytes\n- Modified: {}\n\n",
        relative_path.display(),
        entry.metadata().map(|m| m.len()).unwrap_or(0),
        "Unknown"
    )); // Using "Unknown" as placeholder for modified time in estimation

    // Add tokens for the code fences
    token_count += estimate_tokens("```\n```");

    // Try to read file content
    if let Ok(content) = fs::read_to_string(file_path) {
        if line_numbers {
            // When line numbers are enabled, we add the line number prefix to each line
            let lines_with_numbers: String = content
                .lines()
                .enumerate()
                .map(|(i, line)| format!("{:>4} | {}\n", i + 1, line))
                .collect();
            token_count += estimate_tokens(&lines_with_numbers);
        } else {
            token_count += estimate_tokens(&content);
        }
    }

    token_count
}

/// Counts the tokens that would be generated for the entire file tree section
pub fn count_tree_tokens(tree: &BTreeMap<String, crate::tree::FileNode>, depth: usize) -> usize {
    let mut token_count = 0;

    // Add tokens for indentation
    let indent = "  ".repeat(depth);

    for (name, node) in tree {
        match node {
            crate::tree::FileNode::File => {
                token_count += estimate_tokens(&format!("{}- 📄 {}\n", indent, name));
            }
            crate::tree::FileNode::Directory(children) => {
                token_count += estimate_tokens(&format!("{}- 📁 {}\n", indent, name));
                token_count += count_tree_tokens(children, depth + 1);
            }
        }
    }

    token_count
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    #[test]
    fn test_estimate_tokens() {
        // Test with a simple string
        let text = "Hello, world!";
        let tokens = estimate_tokens(text);
        // "Hello, world!" is 4 tokens with cl100k_base
        assert_eq!(tokens, 4);

        // Test with code-like content
        let code_text = "fn main() {\n    println!(\"Hello, world!\");\n}";
        let tokens = estimate_tokens(code_text);
        // This specific code snippet is 12 tokens with cl100k_base
        assert_eq!(tokens, 12);
    }

    #[test]
    fn test_count_tree_tokens() {
        // Create a simple tree structure
        let mut tree = BTreeMap::new();
        tree.insert("file1.rs".to_string(), crate::tree::FileNode::File);

        let mut subdir = BTreeMap::new();
        subdir.insert("file2.md".to_string(), crate::tree::FileNode::File);
        tree.insert("src".to_string(), crate::tree::FileNode::Directory(subdir));

        let tokens = count_tree_tokens(&tree, 0);
        // "- 📄 file1.rs\n" -> 8 tokens
        // "- 📁 src\n" -> 6 tokens
        // "  - 📄 file2.md\n" -> 9 tokens
        // Total should be 23 tokens
        assert_eq!(tokens, 23);
    }
```

### File: `src/tree.rs`

- Size: 10845 bytes
- Modified: 2026-02-14 17:55:15 UTC

```rust
use ignore::DirEntry;
use std::collections::BTreeMap;
use std::io::{self, Write};
use std::path::Path;

/// A nested map to represent the file tree structure.
#[derive(Debug, Clone, PartialEq)]
pub enum FileNode {
    File,
    Directory(BTreeMap<String, FileNode>),
}

/// Type alias for the file tree structure.
pub type FileTree = BTreeMap<String, FileNode>;

/// Builds a nested BTreeMap representing the file structure.
pub fn build_file_tree(files: &[DirEntry], base_path: &Path) -> FileTree {
    let mut tree = BTreeMap::new();
    for entry in files {
        let path = entry
            .path()
            .strip_prefix(base_path)
            .unwrap_or_else(|_| entry.path());
        let components: Vec<_> = path.components().collect();

        // Insert this path into the tree
        insert_path(&mut tree, &components);
    }
    tree
}

/// Helper function to insert a path into the tree structure
fn insert_path(tree: &mut FileTree, components: &[std::path::Component]) {
    if components.is_empty() {
        return;
    }

    let name = components[0].as_os_str().to_string_lossy().to_string();

    if components.len() == 1 {
        // This is the last component, so it's a file
        tree.insert(name, FileNode::File);
    } else {
        // This is a directory component
        // Make sure the directory exists
        tree.entry(name.clone())
            .or_insert_with(|| FileNode::Directory(BTreeMap::new()));

        // Recursively insert the rest of the path
        if let Some(FileNode::Directory(next_dir)) = tree.get_mut(&name) {
            insert_path(next_dir, &components[1..]);
        }
    }
}

/// Recursively prints the file tree to the console.
pub fn print_tree(tree: &FileTree, depth: usize) {
    for (name, node) in tree {
        let indent = "  ".repeat(depth);
        match node {
            FileNode::File => {
                println!("{}- 📄 {}", indent, name);
            }
            FileNode::Directory(children) => {
                println!("{}- 📁 {}", indent, name);
                print_tree(children, depth + 1);
            }
        }
    }
}

/// Recursively writes the file tree to a file.
pub fn write_tree_to_file(
    output: &mut impl Write,
    tree: &FileTree,
    depth: usize,
) -> io::Result<()> {
    for (name, node) in tree {
        let indent = "  ".repeat(depth);
        match node {
            FileNode::File => {
                writeln!(output, "{}- 📄 {}", indent, name)?;
            }
            FileNode::Directory(children) => {
                writeln!(output, "{}- 📁 {}", indent, name)?;
                write_tree_to_file(output, children, depth + 1)?;
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::file_utils::collect_files;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_build_file_tree_with_collected_files() {
        // 1. Set up a temporary directory with a file structure
        let dir = tempdir().unwrap();
        let base_path = dir.path();

        fs::create_dir(base_path.join("src")).unwrap();
        fs::File::create(base_path.join("src/main.rs")).unwrap();
        fs::File::create(base_path.join("README.md")).unwrap();
        // Add a hidden file that should be ignored by default
        fs::File::create(base_path.join(".env")).unwrap();

        // 2. Collect files using the actual function
        let files = collect_files(base_path, &[], &[], &[]).unwrap();

        // 3. Assert that the correct files were collected (a hidden file is ignored)
        assert_eq!(files.len(), 2);

        // 4. Build the tree with the collected files
        let tree = build_file_tree(&files, base_path);

        // 5. Assert the tree structure is correct
        let mut expected: FileTree = BTreeMap::new();
        let mut src_tree = BTreeMap::new();
        src_tree.insert("main.rs".to_string(), FileNode::File);
        expected.insert("src".to_string(), FileNode::Directory(src_tree));
        expected.insert("README.md".to_string(), FileNode::File);

        assert_eq!(tree, expected);
    }

    #[test]
    fn test_build_file_tree_empty() {
        let dir = tempdir().unwrap();
        let base_path = dir.path();

        let files = collect_files(base_path, &[], &[], &[]).unwrap();
        let tree = build_file_tree(&files, base_path);

        assert!(tree.is_empty());
    }

    #[test]
    fn test_build_file_tree_single_file() {
        let dir = tempdir().unwrap();
        let base_path = dir.path();

        fs::File::create(base_path.join("single.txt")).unwrap();

        let files = collect_files(base_path, &[], &[], &[]).unwrap();
        let tree = build_file_tree(&files, base_path);

        let mut expected: FileTree = BTreeMap::new();
        expected.insert("single.txt".to_string(), FileNode::File);

        assert_eq!(tree, expected);
    }
```

### File: `src/tree_sitter/language_support.rs`

- Size: 3935 bytes
- Modified: 2026-02-15 07:25:48 UTC

```rust
//! Core types and traits for language support.

use std::fmt;
use std::str::FromStr;

/// The kind of signature extracted from source code.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SignatureKind {
    Function,
    Method,
    Struct,
    Enum,
    Trait,
    Interface,
    Class,
    Impl,
    Module,
    Constant,
    TypeAlias,
    Macro,
}

impl fmt::Display for SignatureKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SignatureKind::Function => write!(f, "function"),
            SignatureKind::Method => write!(f, "method"),
            SignatureKind::Struct => write!(f, "struct"),
            SignatureKind::Enum => write!(f, "enum"),
            SignatureKind::Trait => write!(f, "trait"),
            SignatureKind::Interface => write!(f, "interface"),
            SignatureKind::Class => write!(f, "class"),
            SignatureKind::Impl => write!(f, "impl"),
            SignatureKind::Module => write!(f, "module"),
            SignatureKind::Constant => write!(f, "constant"),
            SignatureKind::TypeAlias => write!(f, "type"),
            SignatureKind::Macro => write!(f, "macro"),
        }
    }
}

/// Visibility level of a signature.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Visibility {
    #[default]
    All,
    Public,
    Private,
}

impl FromStr for Visibility {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_lowercase().as_str() {
            "public" => Visibility::Public,
            "private" => Visibility::Private,
            _ => Visibility::All,
        })
    }
}
```

### File: `src/tree_sitter/languages/c.rs`

- Size: 10386 bytes
- Modified: 2026-02-15 07:34:19 UTC

```rust
//! C language support for tree-sitter.

#[cfg(feature = "tree-sitter-c")]
use tree_sitter::{Parser, Tree};

#[cfg(feature = "tree-sitter-c")]
use crate::tree_sitter::language_support::{CodeStructure, LanguageSupport, Signature, SignatureKind, Visibility};

pub struct CSupport;

#[cfg(feature = "tree-sitter-c")]
impl CSupport {
    fn get_language() -> tree_sitter::Language {
        tree_sitter_c::language()
    }
}

#[cfg(feature = "tree-sitter-c")]
impl LanguageSupport for CSupport {
    fn file_extensions(&self) -> &[&'static str] {
        &["c", "h"]
    }

    fn parse(&self, source: &str) -> Option<Tree> {
        let mut parser = Parser::new();
        parser.set_language(&Self::get_language()).ok()?;
        parser.parse(source, None)
    }

    fn extract_signatures(&self, source: &str, visibility: Visibility) -> Vec<Signature> {
        let tree = match self.parse(source) {
            Some(t) => t,
            None => return Vec::new(),
        };

        let root = tree.root_node();
        let mut signatures = Vec::new();

        self.extract_signatures_from_node(source, &root, visibility, &mut signatures);

        signatures.sort_by_key(|s| s.line_number);
        signatures
    }

    fn extract_structure(&self, source: &str) -> CodeStructure {
        let tree = match self.parse(source) {
            Some(t) => t,
            None => return CodeStructure::default(),
        };

        let root = tree.root_node();
        let mut structure = CodeStructure {
            total_lines: source.lines().count(),
            ..Default::default()
        };

        self.extract_structure_from_node(&root, &mut structure);
        structure
    }

    fn find_truncation_point(&self, source: &str, max_bytes: usize) -> usize {
        if source.len() <= max_bytes {
            return source.len();
        }

        let tree = match self.parse(source) {
            Some(t) => t,
            None => return max_bytes,
        };

        let root = tree.root_node();
        let mut best_end = 0;

        let mut cursor = root.walk();
        self.find_best_boundary(&mut cursor, max_bytes, &mut best_end);
        drop(cursor);

        if best_end == 0 {
            max_bytes
        } else {
            best_end
        }
    }
}

#[cfg(feature = "tree-sitter-c")]
impl CSupport {
    fn extract_signatures_from_node(
        &self,
        source: &str,
        node: &tree_sitter::Node,
        _visibility: Visibility,
        signatures: &mut Vec<Signature>,
    ) {
        match node.kind() {
            "function_definition" => {
                if let Some(sig) = self.extract_function_signature(source, node) {
                    signatures.push(sig);
                }
            }
            "struct_specifier" => {
                if let Some(sig) = self.extract_struct_signature(source, node) {
                    signatures.push(sig);
                }
            }
            "enum_specifier" => {
                if let Some(sig) = self.extract_enum_signature(source, node) {
                    signatures.push(sig);
                }
            }
            "type_definition" => {
                if let Some(sig) = self.extract_typedef_signature(source, node) {
                    signatures.push(sig);
                }
            }
            "preproc_function_def" => {
                if let Some(sig) = self.extract_macro_signature(source, node) {
                    signatures.push(sig);
                }
            }
            _ => {}
        }

        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            self.extract_signatures_from_node(source, &child, _visibility, signatures);
        }
    }

    fn extract_structure_from_node(&self, node: &tree_sitter::Node, structure: &mut CodeStructure) {
        match node.kind() {
            "function_definition" => structure.functions += 1,
            "struct_specifier" => structure.structs += 1,
            "enum_specifier" => structure.enums += 1,
            "preproc_include" => {
                structure.imports.push("include".to_string());
            }
            _ => {}
        }

        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            self.extract_structure_from_node(&child, structure);
        }
    }

    fn extract_function_signature(
        &self,
        source: &str,
        node: &tree_sitter::Node,
    ) -> Option<Signature> {
        let name = self.find_function_name(node, source)?;
        let return_type = self.find_return_type(node, source);

        let mut full_sig = String::new();
        if let Some(r) = &return_type {
            full_sig.push_str(r);
            full_sig.push(' ');
        }
        full_sig.push_str(&name);
        full_sig.push_str("()");

        Some(Signature {
            kind: SignatureKind::Function,
            name,
            params: None,
            return_type,
            visibility: Visibility::All, // C has no visibility
            line_number: node.start_position().row + 1,
            full_signature: full_sig,
        })
    }
```

### File: `src/tree_sitter/languages/cpp.rs`

- Size: 12095 bytes
- Modified: 2026-02-15 07:44:25 UTC

```rust
//! C++ language support for tree-sitter.

#[cfg(feature = "tree-sitter-cpp")]
use tree_sitter::{Parser, Tree};

#[cfg(feature = "tree-sitter-cpp")]
use crate::tree_sitter::language_support::{CodeStructure, LanguageSupport, Signature, SignatureKind, Visibility};

pub struct CppSupport;

#[cfg(feature = "tree-sitter-cpp")]
impl CppSupport {
    fn get_language() -> tree_sitter::Language {
        tree_sitter_cpp::language()
    }
}

#[cfg(feature = "tree-sitter-cpp")]
impl LanguageSupport for CppSupport {
    fn file_extensions(&self) -> &[&'static str] {
        &["cpp", "cxx", "cc", "hpp", "hxx", "hh"]
    }

    fn parse(&self, source: &str) -> Option<Tree> {
        let mut parser = Parser::new();
        parser.set_language(&Self::get_language()).ok()?;
        parser.parse(source, None)
    }

    fn extract_signatures(&self, source: &str, visibility: Visibility) -> Vec<Signature> {
        let tree = match self.parse(source) {
            Some(t) => t,
            None => return Vec::new(),
        };

        let root = tree.root_node();
        let mut signatures = Vec::new();

        self.extract_signatures_from_node(source, &root, visibility, &mut signatures);

        signatures.sort_by_key(|s| s.line_number);
        signatures
    }

    fn extract_structure(&self, source: &str) -> CodeStructure {
        let tree = match self.parse(source) {
            Some(t) => t,
            None => return CodeStructure::default(),
        };

        let root = tree.root_node();
        let mut structure = CodeStructure {
            total_lines: source.lines().count(),
            ..Default::default()
        };

        self.extract_structure_from_node(&root, &mut structure);
        structure
    }

    fn find_truncation_point(&self, source: &str, max_bytes: usize) -> usize {
        if source.len() <= max_bytes {
            return source.len();
        }

        let tree = match self.parse(source) {
            Some(t) => t,
            None => return max_bytes,
        };

        let root = tree.root_node();
        let mut best_end = 0;

        let mut cursor = root.walk();
        self.find_best_boundary(&mut cursor, max_bytes, &mut best_end);
        drop(cursor);

        if best_end == 0 {
            max_bytes
        } else {
            best_end
        }
    }
}

#[cfg(feature = "tree-sitter-cpp")]
impl CppSupport {
    fn extract_signatures_from_node(
        &self,
        source: &str,
        node: &tree_sitter::Node,
        visibility: Visibility,
        signatures: &mut Vec<Signature>,
    ) {
        match node.kind() {
            "function_definition" => {
                if let Some(sig) = self.extract_function_signature(source, node, visibility) {
                    signatures.push(sig);
                }
            }
            "class_specifier" => {
                if let Some(sig) = self.extract_class_signature(source, node, visibility) {
                    signatures.push(sig);
                }
            }
            "struct_specifier" => {
                if let Some(sig) = self.extract_struct_signature(source, node) {
                    signatures.push(sig);
                }
            }
            "enum_specifier" => {
                if let Some(sig) = self.extract_enum_signature(source, node) {
                    signatures.push(sig);
                }
            }
            "alias_declaration" | "type_definition" => {
                if let Some(sig) = self.extract_alias_signature(source, node) {
                    signatures.push(sig);
                }
            }
            "preproc_function_def" => {
                if let Some(sig) = self.extract_macro_signature(source, node) {
                    signatures.push(sig);
                }
            }
            _ => {}
        }

        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            self.extract_signatures_from_node(source, &child, visibility, signatures);
        }
    }

    fn extract_structure_from_node(&self, node: &tree_sitter::Node, structure: &mut CodeStructure) {
        match node.kind() {
            "function_definition" => structure.functions += 1,
            "class_specifier" => structure.classes += 1,
            "struct_specifier" => structure.structs += 1,
            "enum_specifier" => structure.enums += 1,
            "preproc_include" => {
                structure.imports.push("include".to_string());
            }
            _ => {}
        }

        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            self.extract_structure_from_node(&child, structure);
        }
    }

    fn get_visibility(&self, _node: &tree_sitter::Node) -> Visibility {
        // C++ has access specifiers: public, private, protected
        // For simplicity, we check sibling nodes for access specifiers
        // This is a simplified check; full implementation would track class context
        Visibility::All
    }

    fn extract_function_signature(
        &self,
        source: &str,
        node: &tree_sitter::Node,
        visibility: Visibility,
    ) -> Option<Signature> {
        let name = self.find_function_name(node, source)?;
        let return_type = self.find_return_type(node, source);

        let mut full_sig = String::new();
        if let Some(r) = &return_type {
            full_sig.push_str(r);
            full_sig.push(' ');
        }
        full_sig.push_str(&name);
        full_sig.push_str("()");

        Some(Signature {
            kind: SignatureKind::Function,
            name,
            params: None,
            return_type,
            visibility,
            line_number: node.start_position().row + 1,
            full_signature: full_sig,
        })
    }
```

### File: `src/tree_sitter/languages/go.rs`

- Size: 13412 bytes
- Modified: 2026-02-15 07:56:34 UTC

```rust
//! Go language support for tree-sitter.

#[cfg(feature = "tree-sitter-go")]
use tree_sitter::{Parser, Tree};

#[cfg(feature = "tree-sitter-go")]
use crate::tree_sitter::language_support::{CodeStructure, LanguageSupport, Signature, SignatureKind, Visibility};

pub struct GoSupport;

#[cfg(feature = "tree-sitter-go")]
impl GoSupport {
    fn get_language() -> tree_sitter::Language {
        tree_sitter_go::language()
    }
}

#[cfg(feature = "tree-sitter-go")]
impl LanguageSupport for GoSupport {
    fn file_extensions(&self) -> &[&'static str] {
        &["go"]
    }

    fn parse(&self, source: &str) -> Option<Tree> {
        let mut parser = Parser::new();
        parser.set_language(&Self::get_language()).ok()?;
        parser.parse(source, None)
    }

    fn extract_signatures(&self, source: &str, visibility: Visibility) -> Vec<Signature> {
        let tree = match self.parse(source) {
            Some(t) => t,
            None => return Vec::new(),
        };

        let root = tree.root_node();
        let mut signatures = Vec::new();

        self.extract_signatures_from_node(source, &root, visibility, &mut signatures);

        signatures.sort_by_key(|s| s.line_number);
        signatures
    }

    fn extract_structure(&self, source: &str) -> CodeStructure {
        let tree = match self.parse(source) {
            Some(t) => t,
            None => return CodeStructure::default(),
        };

        let root = tree.root_node();
        let mut structure = CodeStructure {
            total_lines: source.lines().count(),
            ..Default::default()
        };

        self.extract_structure_from_node(&root, &mut structure);
        structure
    }

    fn find_truncation_point(&self, source: &str, max_bytes: usize) -> usize {
        if source.len() <= max_bytes {
            return source.len();
        }

        let tree = match self.parse(source) {
            Some(t) => t,
            None => return max_bytes,
        };

        let root = tree.root_node();
        let mut best_end = 0;

        let mut cursor = root.walk();
        self.find_best_boundary(&mut cursor, max_bytes, &mut best_end);
        drop(cursor);

        if best_end == 0 {
            max_bytes
        } else {
            best_end
        }
    }
}

#[cfg(feature = "tree-sitter-go")]
impl GoSupport {
    fn extract_signatures_from_node(
        &self,
        source: &str,
        node: &tree_sitter::Node,
        visibility: Visibility,
        signatures: &mut Vec<Signature>,
    ) {
        match node.kind() {
            "function_declaration" => {
                if let Some(sig) = self.extract_function_signature(source, node, visibility) {
                    signatures.push(sig);
                }
            }
            "method_declaration" => {
                if let Some(sig) = self.extract_method_signature(source, node, visibility) {
                    signatures.push(sig);
                }
            }
            "type_declaration" => {
                self.extract_type_signatures(source, node, visibility, signatures);
            }
            _ => {}
        }

        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            self.extract_signatures_from_node(source, &child, visibility, signatures);
        }
    }

    fn extract_structure_from_node(&self, node: &tree_sitter::Node, structure: &mut CodeStructure) {
        match node.kind() {
            "function_declaration" | "method_declaration" => structure.functions += 1,
            "type_spec" => {
                // Check what type it is
                if let Some(parent) = node.parent() {
                    if parent.kind() == "type_declaration" {
                        // Could be struct, interface, or type alias
                        let mut cursor = node.walk();
                        for child in node.children(&mut cursor) {
                            match child.kind() {
                                "struct_type" => structure.structs += 1,
                                "interface_type" => structure.interfaces += 1,
                                "type_identifier" => structure.type_aliases += 1,
                                _ => {}
                            }
                        }
                    }
                }
            }
            "import_declaration" => {
                structure.imports.push("import".to_string());
            }
            _ => {}
        }

        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            self.extract_structure_from_node(&child, structure);
        }
    }

    fn is_exported(&self, name: &str) -> bool {
        name.chars().next().map_or(false, |c| c.is_uppercase())
    }

    fn extract_function_signature(
        &self,
        source: &str,
        node: &tree_sitter::Node,
        visibility: Visibility,
    ) -> Option<Signature> {
        let name = self.find_child_text(node, "identifier", source)?;
        let is_exported = self.is_exported(&name);

        if visibility == Visibility::Public && !is_exported {
            return None;
        }
        if visibility == Visibility::Private && is_exported {
            return None;
        }

        let params = self.find_child_text(node, "parameter_list", source);
        let result = self
            .find_child_text(node, "type_identifier", source)
            .or_else(|| self.find_child_text_for_result(node, source));

        let mut full_sig = String::new();
        full_sig.push_str("func ");
        full_sig.push_str(&name);
        if let Some(p) = &params {
            full_sig.push_str(p);
        } else {
            full_sig.push_str("()");
        }
        if let Some(r) = &result {
            full_sig.push(' ');
            full_sig.push_str(r);
        }

        Some(Signature {
            kind: SignatureKind::Function,
            name,
            params,
            return_type: result,
            visibility: if is_exported {
                Visibility::Public
            } else {
                Visibility::Private
            },
            line_number: node.start_position().row + 1,
            full_signature: full_sig,
        })
    }
```

### File: `src/tree_sitter/languages/java.rs`

- Size: 12597 bytes
- Modified: 2026-02-15 07:44:23 UTC

```rust
//! Java language support for tree-sitter.

#[cfg(feature = "tree-sitter-java")]
use tree_sitter::{Parser, Tree};

#[cfg(feature = "tree-sitter-java")]
use crate::tree_sitter::language_support::{CodeStructure, LanguageSupport, Signature, SignatureKind, Visibility};

pub struct JavaSupport;

#[cfg(feature = "tree-sitter-java")]
impl JavaSupport {
    fn get_language() -> tree_sitter::Language {
        tree_sitter_java::language()
    }
}

#[cfg(feature = "tree-sitter-java")]
impl LanguageSupport for JavaSupport {
    fn file_extensions(&self) -> &[&'static str] {
        &["java"]
    }

    fn parse(&self, source: &str) -> Option<Tree> {
        let mut parser = Parser::new();
        parser.set_language(&Self::get_language()).ok()?;
        parser.parse(source, None)
    }

    fn extract_signatures(&self, source: &str, visibility: Visibility) -> Vec<Signature> {
        let tree = match self.parse(source) {
            Some(t) => t,
            None => return Vec::new(),
        };

        let root = tree.root_node();
        let mut signatures = Vec::new();

        self.extract_signatures_from_node(source, &root, visibility, &mut signatures);

        signatures.sort_by_key(|s| s.line_number);
        signatures
    }

    fn extract_structure(&self, source: &str) -> CodeStructure {
        let tree = match self.parse(source) {
            Some(t) => t,
            None => return CodeStructure::default(),
        };

        let root = tree.root_node();
        let mut structure = CodeStructure {
            total_lines: source.lines().count(),
            ..Default::default()
        };

        self.extract_structure_from_node(&root, &mut structure);
        structure
    }

    fn find_truncation_point(&self, source: &str, max_bytes: usize) -> usize {
        if source.len() <= max_bytes {
            return source.len();
        }

        let tree = match self.parse(source) {
            Some(t) => t,
            None => return max_bytes,
        };

        let root = tree.root_node();
        let mut best_end = 0;

        let mut cursor = root.walk();
        self.find_best_boundary(&mut cursor, max_bytes, &mut best_end);
        drop(cursor);

        if best_end == 0 {
            max_bytes
        } else {
            best_end
        }
    }
}

#[cfg(feature = "tree-sitter-java")]
impl JavaSupport {
    fn extract_signatures_from_node(
        &self,
        source: &str,
        node: &tree_sitter::Node,
        visibility: Visibility,
        signatures: &mut Vec<Signature>,
    ) {
        match node.kind() {
            "method_declaration" => {
                if let Some(sig) = self.extract_method_signature(source, node, visibility) {
                    signatures.push(sig);
                }
            }
            "class_declaration" => {
                if let Some(sig) = self.extract_class_signature(source, node, visibility) {
                    signatures.push(sig);
                }
            }
            "interface_declaration" => {
                if let Some(sig) = self.extract_interface_signature(source, node, visibility) {
                    signatures.push(sig);
                }
            }
            "enum_declaration" => {
                if let Some(sig) = self.extract_enum_signature(source, node, visibility) {
                    signatures.push(sig);
                }
            }
            "field_declaration" => {
                if let Some(sig) = self.extract_field_signature(source, node, visibility) {
                    signatures.push(sig);
                }
            }
            _ => {}
        }

        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            self.extract_signatures_from_node(source, &child, visibility, signatures);
        }
    }

    fn extract_structure_from_node(&self, node: &tree_sitter::Node, structure: &mut CodeStructure) {
        match node.kind() {
            "method_declaration" => structure.functions += 1,
            "class_declaration" => structure.classes += 1,
            "interface_declaration" => structure.interfaces += 1,
            "enum_declaration" => structure.enums += 1,
            "import_declaration" => {
                structure.imports.push("import".to_string());
            }
            _ => {}
        }

        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            self.extract_structure_from_node(&child, structure);
        }
    }

    fn get_visibility(&self, _node: &tree_sitter::Node) -> Visibility {
        // Java visibility is determined by modifiers
        // Simplified: check for public/private/protected keywords in AST modifiers
        Visibility::All
    }

    fn extract_method_signature(
        &self,
        source: &str,
        node: &tree_sitter::Node,
        visibility: Visibility,
    ) -> Option<Signature> {
        let vis = self.get_visibility(node);

        if visibility == Visibility::Public && vis != Visibility::Public {
            return None;
        }
        if visibility == Visibility::Private && vis == Visibility::Public {
            return None;
        }

        let name = self.find_child_text(node, "identifier", source)?;
        let params = self.find_child_text(node, "formal_parameters", source);
        let return_type = self
            .find_child_text(node, "type_identifier", source)
            .or_else(|| self.find_child_text_for_type(node, source));

        let mut full_sig = String::new();
        if vis == Visibility::Public {
            full_sig.push_str("public ");
        }
        if let Some(r) = &return_type {
            full_sig.push_str(r);
            full_sig.push(' ');
        }
        full_sig.push_str(&name);
        if let Some(p) = &params {
            full_sig.push_str(p);
        } else {
            full_sig.push_str("()");
        }

        Some(Signature {
            kind: SignatureKind::Method,
            name,
            params,
            return_type,
            visibility: vis,
            line_number: node.start_position().row + 1,
            full_signature: full_sig,
        })
    }
```

### File: `src/tree_sitter/languages/javascript.rs`

- Size: 10064 bytes
- Modified: 2026-02-15 07:56:31 UTC

```rust
//! JavaScript language support for tree-sitter.

use tree_sitter::{Parser, Tree};

use crate::tree_sitter::language_support::{CodeStructure, LanguageSupport, Signature, SignatureKind, Visibility};

pub struct JavaScriptSupport;

impl JavaScriptSupport {
    fn get_language() -> tree_sitter::Language {
        tree_sitter_javascript::language()
    }
}

impl LanguageSupport for JavaScriptSupport {
    fn file_extensions(&self) -> &[&'static str] {
        &["js", "mjs", "cjs"]
    }

    fn parse(&self, source: &str) -> Option<Tree> {
        let mut parser = Parser::new();
        parser.set_language(&Self::get_language()).ok()?;
        parser.parse(source, None)
    }

    fn extract_signatures(&self, source: &str, visibility: Visibility) -> Vec<Signature> {
        let tree = match self.parse(source) {
            Some(t) => t,
            None => return Vec::new(),
        };

        let root = tree.root_node();
        let mut signatures = Vec::new();

        self.extract_signatures_from_node(source, &root, visibility, &mut signatures);

        signatures.sort_by_key(|s| s.line_number);
        signatures
    }

    fn extract_structure(&self, source: &str) -> CodeStructure {
        let tree = match self.parse(source) {
            Some(t) => t,
            None => return CodeStructure::default(),
        };

        let root = tree.root_node();
        let mut structure = CodeStructure {
            total_lines: source.lines().count(),
            ..Default::default()
        };

        self.extract_structure_from_node(&root, &mut structure);
        structure
    }

    fn find_truncation_point(&self, source: &str, max_bytes: usize) -> usize {
        if source.len() <= max_bytes {
            return source.len();
        }

        let tree = match self.parse(source) {
            Some(t) => t,
            None => return max_bytes,
        };

        let root = tree.root_node();
        let mut best_end = 0;

        let mut cursor = root.walk();
        self.find_best_boundary(&mut cursor, max_bytes, &mut best_end);
        drop(cursor);

        if best_end == 0 {
            max_bytes
        } else {
            best_end
        }
    }
}

impl JavaScriptSupport {
    fn extract_signatures_from_node(
        &self,
        source: &str,
        node: &tree_sitter::Node,
        _visibility: Visibility,
        signatures: &mut Vec<Signature>,
    ) {
        match node.kind() {
            "function_declaration" => {
                if let Some(sig) = self.extract_function_signature(source, node) {
                    signatures.push(sig);
                }
            }
            "class_declaration" => {
                if let Some(sig) = self.extract_class_signature(source, node) {
                    signatures.push(sig);
                }
            }
            "variable_declaration" | "lexical_declaration" => {
                self.extract_variable_declarations(source, node, signatures);
            }
            "export_statement" => {
                self.extract_export_signatures(source, node, signatures);
            }
            _ => {}
        }

        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            self.extract_signatures_from_node(source, &child, _visibility, signatures);
        }
    }

    fn extract_structure_from_node(&self, node: &tree_sitter::Node, structure: &mut CodeStructure) {
        match node.kind() {
            "function_declaration" | "generator_function_declaration" | "function_expression" => {
                structure.functions += 1;
            }
            "class_declaration" | "class_expression" => {
                structure.classes += 1;
            }
            "import_statement" => {
                structure.imports.push("import".to_string());
            }
            "export_statement" => {
                structure.exports.push("export".to_string());
            }
            _ => {}
        }

        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            self.extract_structure_from_node(&child, structure);
        }
    }

    fn extract_function_signature(
        &self,
        source: &str,
        node: &tree_sitter::Node,
    ) -> Option<Signature> {
        let name = self.find_child_text(node, "identifier", source)?;
        let params = self.find_child_text(node, "formal_parameters", source);

        let full_sig = match &params {
            Some(p) => format!("function {}({})", name, p),
            None => format!("function {}()", name),
        };

        Some(Signature {
            kind: SignatureKind::Function,
            name,
            params,
            return_type: None, // JS doesn't have explicit return types in syntax
            visibility: Visibility::All,
            line_number: node.start_position().row + 1,
            full_signature: full_sig,
        })
    }
```

### File: `src/tree_sitter/languages/python.rs`

- Size: 9326 bytes
- Modified: 2026-02-15 07:40:59 UTC

```rust
//! Python language support for tree-sitter.

#[cfg(feature = "tree-sitter-python")]
use tree_sitter::{Parser, Tree};

#[cfg(feature = "tree-sitter-python")]
use crate::tree_sitter::language_support::{CodeStructure, LanguageSupport, Signature, SignatureKind, Visibility};

pub struct PythonSupport;

#[cfg(feature = "tree-sitter-python")]
impl PythonSupport {
    fn get_language() -> tree_sitter::Language {
        tree_sitter_python::language()
    }
}

#[cfg(feature = "tree-sitter-python")]
impl LanguageSupport for PythonSupport {
    fn file_extensions(&self) -> &[&'static str] {
        &["py", "pyw"]
    }

    fn parse(&self, source: &str) -> Option<Tree> {
        let mut parser = Parser::new();
        parser.set_language(&Self::get_language()).ok()?;
        parser.parse(source, None)
    }

    fn extract_signatures(&self, source: &str, visibility: Visibility) -> Vec<Signature> {
        let tree = match self.parse(source) {
            Some(t) => t,
            None => return Vec::new(),
        };

        let root = tree.root_node();
        let mut signatures = Vec::new();

        self.extract_signatures_from_node(source, &root, visibility, &mut signatures);

        signatures.sort_by_key(|s| s.line_number);
        signatures
    }

    fn extract_structure(&self, source: &str) -> CodeStructure {
        let tree = match self.parse(source) {
            Some(t) => t,
            None => return CodeStructure::default(),
        };

        let root = tree.root_node();
        let mut structure = CodeStructure {
            total_lines: source.lines().count(),
            ..Default::default()
        };

        self.extract_structure_from_node(&root, &mut structure);
        structure
    }

    fn find_truncation_point(&self, source: &str, max_bytes: usize) -> usize {
        if source.len() <= max_bytes {
            return source.len();
        }

        let tree = match self.parse(source) {
            Some(t) => t,
            None => return max_bytes,
        };

        let root = tree.root_node();
        let mut best_end = 0;

        let mut cursor = root.walk();
        self.find_best_boundary(&mut cursor, max_bytes, &mut best_end);
        drop(cursor);

        if best_end == 0 {
            max_bytes
        } else {
            best_end
        }
    }
}

#[cfg(feature = "tree-sitter-python")]
impl PythonSupport {
    fn extract_signatures_from_node(
        &self,
        source: &str,
        node: &tree_sitter::Node,
        _visibility: Visibility,
        signatures: &mut Vec<Signature>,
    ) {
        match node.kind() {
            "function_definition" => {
                if let Some(sig) = self.extract_function_signature(source, node) {
                    signatures.push(sig);
                }
            }
            "class_definition" => {
                if let Some(sig) = self.extract_class_signature(source, node) {
                    signatures.push(sig);
                }
            }
            _ => {}
        }

        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            self.extract_signatures_from_node(source, &child, _visibility, signatures);
        }
    }

    fn extract_structure_from_node(&self, node: &tree_sitter::Node, structure: &mut CodeStructure) {
        match node.kind() {
            "function_definition" => structure.functions += 1,
            "class_definition" => structure.classes += 1,
            "import_statement" | "import_from_statement" => {
                structure.imports.push("import".to_string());
            }
            _ => {}
        }

        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            self.extract_structure_from_node(&child, structure);
        }
    }
```

### File: `src/tree_sitter/languages/rust.rs`

- Size: 18127 bytes
- Modified: 2026-02-15 07:26:38 UTC

```rust
//! Rust language support for tree-sitter.

use tree_sitter::{Parser, Tree};

use crate::tree_sitter::language_support::{
    CodeStructure, LanguageSupport, Signature, SignatureKind, Visibility,
};

pub struct RustSupport;

impl RustSupport {
    fn get_language() -> tree_sitter::Language {
        tree_sitter_rust::language()
    }
}

impl LanguageSupport for RustSupport {
    fn file_extensions(&self) -> &[&'static str] {
        &["rs"]
    }

    fn parse(&self, source: &str) -> Option<Tree> {
        let mut parser = Parser::new();
        parser.set_language(&Self::get_language()).ok()?;
        parser.parse(source, None)
    }

    fn extract_signatures(&self, source: &str, visibility: Visibility) -> Vec<Signature> {
        let tree = match self.parse(source) {
            Some(t) => t,
            None => return Vec::new(),
        };

        let root = tree.root_node();
        let mut signatures = Vec::new();

        self.extract_signatures_from_node(source, &root, visibility, &mut signatures);

        signatures.sort_by_key(|s| s.line_number);
        signatures
    }

    fn extract_structure(&self, source: &str) -> CodeStructure {
        let tree = match self.parse(source) {
            Some(t) => t,
            None => return CodeStructure::default(),
        };

        let root = tree.root_node();
        let mut structure = CodeStructure {
            total_lines: source.lines().count(),
            ..Default::default()
        };

        self.extract_structure_from_node(source, &root, &mut structure);

        structure.code_lines = source
            .lines()
            .filter(|line| {
                let trimmed = line.trim();
                !trimmed.is_empty() && !trimmed.starts_with("//")
            })
            .count();

        structure
    }

    fn find_truncation_point(&self, source: &str, max_bytes: usize) -> usize {
        if source.len() <= max_bytes {
            return source.len();
        }

        let tree = match self.parse(source) {
            Some(t) => t,
            None => return max_bytes,
        };

        let root = tree.root_node();

        let mut best_end = 0;
        let mut cursor = root.walk();

        self.walk_for_boundary(&mut cursor, max_bytes, &mut best_end);

        if best_end == 0 {
            max_bytes
        } else {
            best_end
        }
    }
}

impl RustSupport {
    fn walk_for_boundary(
        &self,
        cursor: &mut tree_sitter::TreeCursor,
        max_bytes: usize,
        best_end: &mut usize,
    ) {
        loop {
            let node = cursor.node();
            let end_byte = node.end_byte();

            if end_byte <= max_bytes && end_byte > *best_end {
                let is_item = matches!(
                    node.kind(),
                    "function_item"
                        | "struct_item"
                        | "enum_item"
                        | "trait_item"
                        | "impl_item"
                        | "mod_item"
                        | "const_item"
                        | "static_item"
                        | "type_item"
                        | "macro_definition"
                );
                if is_item {
                    *best_end = end_byte;
                }
            }

            if cursor.goto_first_child() {
                self.walk_for_boundary(cursor, max_bytes, best_end);
                cursor.goto_parent();
            }

            if !cursor.goto_next_sibling() {
                break;
            }
        }
    }

    fn extract_signatures_from_node(
        &self,
        source: &str,
        node: &tree_sitter::Node,
        visibility: Visibility,
        signatures: &mut Vec<Signature>,
    ) {
        match node.kind() {
            "function_item" => {
                if let Some(sig) = self.extract_function_signature(source, node, visibility) {
                    signatures.push(sig);
                }
            }
            "struct_item" => {
                if let Some(sig) = self.extract_struct_signature(source, node, visibility) {
                    signatures.push(sig);
                }
            }
            "enum_item" => {
                if let Some(sig) = self.extract_enum_signature(source, node, visibility) {
                    signatures.push(sig);
                }
            }
            "trait_item" => {
                if let Some(sig) = self.extract_trait_signature(source, node, visibility) {
                    signatures.push(sig);
                }
            }
            "impl_item" => {
                if let Some(sig) = self.extract_impl_signature(source, node) {
                    signatures.push(sig);
                }
            }
            "mod_item" => {
                if let Some(sig) = self.extract_mod_signature(source, node, visibility) {
                    signatures.push(sig);
                }
            }
            "const_item" => {
                if let Some(sig) = self.extract_const_signature(source, node, visibility) {
                    signatures.push(sig);
                }
            }
            "type_item" => {
                if let Some(sig) = self.extract_type_alias_signature(source, node, visibility) {
                    signatures.push(sig);
                }
            }
            "macro_definition" => {
                if let Some(sig) = self.extract_macro_signature(source, node, visibility) {
                    signatures.push(sig);
                }
            }
            _ => {}
        }

        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            self.extract_signatures_from_node(source, &child, visibility, signatures);
        }
    }

    fn extract_structure_from_node(
        &self,
        source: &str,
        node: &tree_sitter::Node,
        structure: &mut CodeStructure,
    ) {
        match node.kind() {
            "function_item" => structure.functions += 1,
            "struct_item" => structure.structs += 1,
            "enum_item" => structure.enums += 1,
            "trait_item" => structure.traits += 1,
            "const_item" => structure.constants += 1,
            "type_item" => structure.type_aliases += 1,
            "macro_definition" => structure.macros += 1,
            "use_declaration" => {
                structure.imports.push(self.node_text(source, node).to_string());
            }
            _ => {}
        }

        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            self.extract_structure_from_node(source, &child, structure);
        }
    }

    fn is_public(&self, node: &tree_sitter::Node) -> bool {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if child.kind() == "visibility_modifier" {
                return true;
            }
        }
        false
    }

    fn get_visibility(&self, node: &tree_sitter::Node) -> Visibility {
        if self.is_public(node) {
            Visibility::Public
        } else {
            Visibility::Private
        }
    }

    fn node_text<'a>(&self, source: &'a str, node: &tree_sitter::Node) -> &'a str {
        &source[node.start_byte()..node.end_byte()]
    }

    fn extract_function_signature(
        &self,
        source: &str,
        node: &tree_sitter::Node,
        visibility_filter: Visibility,
    ) -> Option<Signature> {
        let vis = self.get_visibility(node);
        if !vis.matches_filter(visibility_filter) {
            return None;
        }

        let name = self.find_child_text(node, "identifier", source)?;
        let params = self.find_child_text(node, "parameters", source);
        let return_type = self.find_child_text(node, "type", source);

        let mut full_sig = String::new();
        if vis == Visibility::Public {
            full_sig.push_str("pub ");
        }
        full_sig.push_str("fn ");
        full_sig.push_str(&name);
        if let Some(p) = &params {
            full_sig.push_str(p);
        }
        if let Some(r) = &return_type {
            full_sig.push_str(" -> ");
            full_sig.push_str(r);
        }

        Some(Signature {
            kind: SignatureKind::Function,
            name,
            params,
            return_type,
            visibility: vis,
            line_number: node.start_position().row + 1,
            full_signature: full_sig,
        })
    }
```

### File: `src/tree_sitter/languages/typescript.rs`

- Size: 13681 bytes
- Modified: 2026-02-15 07:56:36 UTC

```rust
//! TypeScript language support for tree-sitter.

#[cfg(feature = "tree-sitter-ts")]
use tree_sitter::{Parser, Tree};

#[cfg(feature = "tree-sitter-ts")]
use crate::tree_sitter::language_support::{CodeStructure, LanguageSupport, Signature, SignatureKind, Visibility};

pub struct TypeScriptSupport;

#[cfg(feature = "tree-sitter-ts")]
impl TypeScriptSupport {
    fn get_language() -> tree_sitter::Language {
        // Use TypeScript grammar (not TSX)
        unsafe { tree_sitter_typescript::language_typescript() }
    }
}

#[cfg(feature = "tree-sitter-ts")]
impl LanguageSupport for TypeScriptSupport {
    fn file_extensions(&self) -> &[&'static str] {
        &["ts", "tsx", "mts", "cts"]
    }

    fn parse(&self, source: &str) -> Option<Tree> {
        let mut parser = Parser::new();
        parser.set_language(&Self::get_language()).ok()?;
        parser.parse(source, None)
    }

    fn extract_signatures(&self, source: &str, visibility: Visibility) -> Vec<Signature> {
        let tree = match self.parse(source) {
            Some(t) => t,
            None => return Vec::new(),
        };

        let root = tree.root_node();
        let mut signatures = Vec::new();

        self.extract_signatures_from_node(source, &root, visibility, &mut signatures);

        signatures.sort_by_key(|s| s.line_number);
        signatures
    }

    fn extract_structure(&self, source: &str) -> CodeStructure {
        let tree = match self.parse(source) {
            Some(t) => t,
            None => return CodeStructure::default(),
        };

        let root = tree.root_node();
        let mut structure = CodeStructure {
            total_lines: source.lines().count(),
            ..Default::default()
        };

        self.extract_structure_from_node(&root, &mut structure);
        structure
    }

    fn find_truncation_point(&self, source: &str, max_bytes: usize) -> usize {
        if source.len() <= max_bytes {
            return source.len();
        }

        let tree = match self.parse(source) {
            Some(t) => t,
            None => return max_bytes,
        };

        let root = tree.root_node();
        let mut best_end = 0;

        let mut cursor = root.walk();
        self.find_best_boundary(&mut cursor, max_bytes, &mut best_end);
        drop(cursor);

        if best_end == 0 {
            max_bytes
        } else {
            best_end
        }
    }
}

#[cfg(feature = "tree-sitter-ts")]
impl TypeScriptSupport {
    fn extract_signatures_from_node(
        &self,
        source: &str,
        node: &tree_sitter::Node,
        _visibility: Visibility,
        signatures: &mut Vec<Signature>,
    ) {
        match node.kind() {
            "function_declaration" | "generator_function_declaration" => {
                if let Some(sig) = self.extract_function_signature(source, node) {
                    signatures.push(sig);
                }
            }
            "class_declaration" => {
                if let Some(sig) = self.extract_class_signature(source, node) {
                    signatures.push(sig);
                }
            }
            "interface_declaration" => {
                if let Some(sig) = self.extract_interface_signature(source, node) {
                    signatures.push(sig);
                }
            }
            "type_alias_declaration" => {
                if let Some(sig) = self.extract_type_alias_signature(source, node) {
                    signatures.push(sig);
                }
            }
            "enum_declaration" => {
                if let Some(sig) = self.extract_enum_signature(source, node) {
                    signatures.push(sig);
                }
            }
            "lexical_declaration" => {
                self.extract_variable_declarations(source, node, signatures);
            }
            "export_statement" => {
                self.extract_export_signatures(source, node, signatures);
            }
            _ => {}
        }

        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            self.extract_signatures_from_node(source, &child, _visibility, signatures);
        }
    }

    fn extract_structure_from_node(&self, node: &tree_sitter::Node, structure: &mut CodeStructure) {
        match node.kind() {
            "function_declaration" | "function_expression" | "arrow_function" => {
                structure.functions += 1;
            }
            "class_declaration" | "class_expression" => {
                structure.classes += 1;
            }
            "interface_declaration" => {
                structure.interfaces += 1;
            }
            "type_alias_declaration" => {
                structure.type_aliases += 1;
            }
            "enum_declaration" => {
                structure.enums += 1;
            }
            "import_statement" => {
                structure.imports.push("import".to_string());
            }
            "export_statement" => {
                structure.exports.push("export".to_string());
            }
            _ => {}
        }

        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            self.extract_structure_from_node(&child, structure);
        }
    }

    fn extract_function_signature(
        &self,
        source: &str,
        node: &tree_sitter::Node,
    ) -> Option<Signature> {
        let name = self.find_child_text(node, "identifier", source)?;
        let params = self.find_child_text(node, "formal_parameters", source);
        let return_type = self.find_child_text(node, "type_annotation", source);

        let full_sig = match (params.as_ref(), return_type.as_ref()) {
            (Some(p), Some(r)) => format!("function {}{} {}", name, p, r),
            (Some(p), None) => format!("function {}{}", name, p),
            (None, Some(r)) => format!("function {}() {}", name, r),
            (None, None) => format!("function {}()", name),
        };

        Some(Signature {
            kind: SignatureKind::Function,
            name,
            params,
            return_type,
            visibility: Visibility::All,
            line_number: node.start_position().row + 1,
            full_signature: full_sig,
        })
    }
```

### File: `src/tree_sitter/signatures.rs`

- Size: 2276 bytes
- Modified: 2026-02-15 07:28:25 UTC

```rust
//! Signature extraction utilities.

use super::language_support::{LanguageSupport, Signature, Visibility};

/// Extract all signatures from source code.
pub fn extract_signatures(
    source: &str,
    support: &dyn LanguageSupport,
    visibility: Visibility,
) -> Vec<Signature> {
    support.extract_signatures(source, visibility)
}
```

### File: `src/tree_sitter/structure.rs`

- Size: 2617 bytes
- Modified: 2026-02-15 06:32:52 UTC

```rust
//! Code structure extraction utilities.

use super::language_support::{CodeStructure, LanguageSupport};

/// Extract structure information from source code.
pub fn extract_structure(source: &str, support: &dyn LanguageSupport) -> CodeStructure {
    support.extract_structure(source)
}
```

### File: `src/tree_sitter/truncation.rs`

- Size: 1888 bytes
- Modified: 2026-02-15 06:32:58 UTC

```rust
//! Smart truncation at AST boundaries.

use super::language_support::LanguageSupport;

/// Find a truncation point that ends at a complete AST node boundary.
///
/// Returns the byte position where the source should be truncated.
/// If no suitable boundary is found within max_bytes, returns max_bytes.
pub fn find_truncation_point(
    source: &str,
    max_bytes: usize,
    support: &dyn LanguageSupport,
) -> usize {
    if source.len() <= max_bytes {
        return source.len();
    }

    support.find_truncation_point(source, max_bytes)
}

/// Check if truncation is needed at a UTF-8 boundary.
pub fn ensure_utf8_boundary(source: &str, position: usize) -> usize {
    if position >= source.len() {
        return source.len();
    }

    let mut pos = position;
    while pos > 0 && !source.is_char_boundary(pos) {
        pos -= 1;
    }
    pos
}
```

### File: `tarpaulin.toml`

- Size: 304 bytes
- Modified: 2026-02-14 07:14:48 UTC

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
- Modified: 2026-02-15 06:55:40 UTC

```rust
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Once;
use std::time::Duration;

use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use tempfile::tempdir;

use context_builder::cli::Args;
use context_builder::config::Config;
use context_builder::{Prompter, run_with_args};

static INIT: Once = Once::new();

fn init_bench_env() {
    INIT.call_once(|| {
        // Note: set_var now requires unsafe block from Rust 2024 onwards
        unsafe {
            std::env::set_var("CB_SILENT", "1");
        }
    });
}

/// Prompter that always auto-confirms. Used to avoid interactive pauses during benchmarks.
struct NoPrompt;

impl Prompter for NoPrompt {
    fn confirm_processing(&self, _file_count: usize) -> std::io::Result<bool> {
        Ok(true)
    }
    fn confirm_overwrite(&self, _file_path: &str) -> std::io::Result<bool> {
        Ok(true)
    }
}

/// Specification for generating a synthetic dataset for benchmarking.
#[derive(Clone)]
struct DatasetSpec {
    /// Human-friendly name used in the benchmark ID.
    name: &'static str,
    /// Approximate number of text files to generate.
    text_files: usize,
    /// Generate one binary file every `binary_every` text files (0 disables binary generation).
    binary_every: usize,
    /// Directory tree depth.
    depth: usize,
    /// Number of subdirectories per directory level.
    width: usize,
    /// Size of each text file (in bytes).
    text_file_size: usize,
    /// File extensions to include in benchmark (others should be ignored).
    filters: Vec<String>,
    /// Directory/file names to ignore (by component name).
    ignores: Vec<String>,
}

fn write_text_file(path: &Path, bytes: usize) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    let mut content = String::with_capacity(bytes);
    // Generate deterministic content consisting of multiple lines
    // Approx 40 bytes per line -> repeat to reach desired size
    let line = "let x = 42; // benchmark content line\n";
    while content.len() < bytes {
        content.push_str(line);
    }
    // Trim to exact size
    content.truncate(bytes);
    // Ensure trailing newline for line-numbering path
    if !content.ends_with('\n') {
        content.push('\n');
    }
    fs::write(path, content).unwrap();
}

fn write_binary_file(path: &Path, bytes: usize) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    let mut data = Vec::with_capacity(bytes);
    // Simple reproducible byte pattern
    for i in 0..bytes {
        data.push(((i as u8).wrapping_mul(31)).wrapping_add(7));
    }
    fs::write(path, data).unwrap();
}

/// Generate a synthetic project directory structure under `root`, returning the input directory path.
fn generate_dataset(root: &Path, spec: &DatasetSpec) -> PathBuf {
    let input_dir = root.join("project");
    let src_dir = input_dir.join("src");
    let docs_dir = input_dir.join("docs");
    let assets_dir = input_dir.join("assets");
    let ignored_target = input_dir.join("target"); // will be ignored if configured
    let ignored_node_modules = input_dir.join("node_modules"); // will be ignored if configured

    fs::create_dir_all(&src_dir).unwrap();
    fs::create_dir_all(&docs_dir).unwrap();
    fs::create_dir_all(&assets_dir).unwrap();
    fs::create_dir_all(&ignored_target).unwrap();
    fs::create_dir_all(&ignored_node_modules).unwrap();

    // Generate nested directories
    fn make_nested_dirs(base: &Path, depth: usize, width: usize) -> Vec<PathBuf> {
        let mut dirs = vec![base.to_path_buf()];
        for d in 1..=depth {
            let mut next_level = Vec::new();
            for parent in &dirs {
                for w in 0..width {
                    let child = parent.join(format!("d{}_{}", d, w));
                    fs::create_dir_all(&child).unwrap();
                    next_level.push(child);
                }
            }
            dirs.extend(next_level);
        }
        dirs
    }
```

### File: `tests/cli_integration.rs`

- Size: 13986 bytes
- Modified: 2026-02-15 06:55:40 UTC

```rust
use std::cell::Cell;
use std::fs;
use std::path::Path;

use tempfile::tempdir;

use context_builder::config::Config;
use context_builder::{Prompter, cli::Args, run_with_args};

struct TestPrompter {
    overwrite_response: bool,
    processing_response: bool,
    last_processing_count: Cell<usize>,
}

impl TestPrompter {
    fn new(overwrite_response: bool, processing_response: bool) -> Self {
        Self {
            overwrite_response,
            processing_response,
            last_processing_count: Cell::new(0),
        }
    }

    fn last_count(&self) -> usize {
        self.last_processing_count.get()
    }
}

impl Prompter for TestPrompter {
    fn confirm_processing(&self, file_count: usize) -> std::io::Result<bool> {
        self.last_processing_count.set(file_count);
        Ok(self.processing_response)
    }

    fn confirm_overwrite(&self, _file_path: &str) -> std::io::Result<bool> {
        Ok(self.overwrite_response)
    }
}

fn write_file(path: &Path, contents: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(path, contents).unwrap();
}

#[test]
fn preview_mode_does_not_create_output_file() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    // Create a small project structure
    write_file(&root.join("src/main.rs"), "fn main() { println!(\"hi\"); }");
    write_file(&root.join("README.md"), "# Readme");

    let args = Args {
        input: root.to_string_lossy().into_owned(),
        output: root.join("output.md").to_string_lossy().into_owned(),
        filter: vec![],
        ignore: vec![],
        preview: true,
        token_count: false,
        line_numbers: false,
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

    let prompter = TestPrompter::new(true, true);

    // Run in preview mode
    let res = run_with_args(args, Config::default(), &prompter);
    assert!(res.is_ok(), "preview mode should succeed");

    // No output file created
    assert!(
        !root.join("output.md").exists(),
        "output file should not be created in preview mode"
    );
}

#[test]
fn preview_mode_skips_overwrite_confirmation() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    // Create an existing output file
    let output_path = root.join("output.md");
    write_file(&output_path, "existing content");

    // Create a small project structure
    write_file(&root.join("src/main.rs"), "fn main() { println!(\"hi\"); }");
    write_file(&root.join("README.md"), "# Readme");

    let args = Args {
        input: root.to_string_lossy().into_owned(),
        output: output_path.to_string_lossy().into_owned(),
        filter: vec![],
        ignore: vec![],
        preview: true,
        token_count: false,
        line_numbers: false,
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

    // Use false for overwrite response to verify it's not called
    let prompter = TestPrompter::new(false, true);

    // Run in preview mode - should succeed even with overwrite denied
    let res = run_with_args(args, Config::default(), &prompter);
    assert!(
        res.is_ok(),
        "preview mode should succeed without overwrite confirmation"
    );

    // Output file should remain unchanged
    let content = fs::read_to_string(&output_path).unwrap();
    assert_eq!(
        content, "existing content",
        "output file should not be modified in preview mode"
    );
}

#[test]
fn token_count_mode_skips_overwrite_confirmation() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    // Create an existing output file
    let output_path = root.join("output.md");
    write_file(&output_path, "existing content");

    // Create a small project structure
    write_file(&root.join("src/main.rs"), "fn main() { println!(\"hi\"); }");
    write_file(&root.join("README.md"), "# Readme");

    let args = Args {
        input: root.to_string_lossy().into_owned(),
        output: output_path.to_string_lossy().into_owned(),
        filter: vec![],
        ignore: vec![],
        preview: false,
        token_count: true,
        line_numbers: false,
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

    // Use false for overwrite response to verify it's not called
    let prompter = TestPrompter::new(false, true);

    // Run in token count mode - should succeed even with overwrite denied
    let res = run_with_args(args, Config::default(), &prompter);
    assert!(
        res.is_ok(),
        "token count mode should succeed without overwrite confirmation"
    );

    // Output file should remain unchanged
    let content = fs::read_to_string(&output_path).unwrap();
    assert_eq!(
        content, "existing content",
        "output file should not be modified in token count mode"
    );
}

#[test]

fn both_preview_and_token_count_modes_work_together() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    // Create a small project structure
    write_file(&root.join("src/main.rs"), "fn main() { println!(\"hi\"); }");
    write_file(&root.join("README.md"), "# Readme");

    let args = Args {
        input: root.to_string_lossy().into_owned(),
        output: root.join("output.md").to_string_lossy().into_owned(),
        filter: vec![],
        ignore: vec![],
        preview: true,
        token_count: true,
        line_numbers: false,
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

    let prompter = TestPrompter::new(false, true); // false for overwrite since it should be skipped

    // Run with both modes
    let res = run_with_args(args, Config::default(), &prompter);
    assert!(res.is_ok(), "both modes should work together");

    // No output file created
    assert!(
        !root.join("output.md").exists(),
        "output file should not be created when both modes are active"
    );
}
```

### File: `tests/diff_integration.rs`

- Size: 1122 bytes
- Modified: 2026-02-14 19:55:07 UTC

```rust
use context_builder::diff::generate_diff;

#[test]
fn test_diff_with_identical_content() {
    let content = r#"# Test Document

This is a test document with some content.

## Section 1

Some text here.

## Section 2

More text here.
"#;

    let diff = generate_diff(content, content);

    // When content is identical, diff should be empty
    assert!(diff.is_empty());
}
```

### File: `tests/test_auto_diff.rs`

- Size: 34489 bytes
- Modified: 2026-02-15 06:55:40 UTC

```rust
//! Integration tests for auto-diff functionality
//!
//! These tests verify that the auto-diff feature works correctly and robustly:
//! - Cache management and collision prevention
//! - Diff generation accuracy
//! - Configuration changes affecting cache
//! - Error recovery from corrupted cache

use pretty_assertions::assert_eq;
use serial_test::serial;
use std::fs;
use std::path::Path;
use tempfile::tempdir;

use chrono::Utc;
use context_builder::cli::Args;
use context_builder::config::{Config, load_config};
use context_builder::{Prompter, run_with_args};

/// Test prompter that always confirms
struct TestPrompter;

impl Prompter for TestPrompter {
    fn confirm_processing(&self, _file_count: usize) -> std::io::Result<bool> {
        Ok(true)
    }
    fn confirm_overwrite(&self, _file_path: &str) -> std::io::Result<bool> {
        Ok(true)
    }
}

fn create_simple_project(base_dir: &Path) -> std::io::Result<()> {
    let src_dir = base_dir.join("src");
    fs::create_dir_all(&src_dir)?;

    fs::write(
        src_dir.join("main.rs"),
        "fn main() {\n    println!(\"Hello, world!\");\n}",
    )?;
    fs::write(
        src_dir.join("lib.rs"),
        "pub fn add(a: i32, b: i32) -> i32 {\n    a + b\n}",
    )?;
    fs::write(
        base_dir.join("README.md"),
        "# Test Project\n\nThis is a test project for auto-diff.",
    )?;

    // Create config file to enable auto-diff
    fs::write(
        base_dir.join("context-builder.toml"),
        r#"
auto_diff = true
timestamped_output = true
"#,
    )?;

    Ok(())
}

#[test]
#[serial]
fn test_auto_diff_workflow_basic() {
    let temp_dir = tempdir().unwrap();
    let project_dir = temp_dir.path().join("project");
    create_simple_project(&project_dir).unwrap();

    let output_dir = temp_dir.path().join("output");
    fs::create_dir_all(&output_dir).unwrap();

    // Change to project directory so config loading works
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(&project_dir).unwrap();

    let args = Args {
        input: ".".to_string(), // Use current directory
        output: output_dir.join("context.md").to_string_lossy().to_string(),
        filter: vec![],
        ignore: vec![],
        preview: false,
        token_count: false,
        line_numbers: false,
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
    let prompter = TestPrompter;

    // First run - should create initial output without diffs
    let config = load_config().unwrap_or_default();

    // Apply config merging manually since we're bypassing run()
    let mut first_args = args.clone();

    // Apply line_numbers from config (matches run_with_args behavior)
    if let Some(line_numbers) = config.line_numbers {
        first_args.line_numbers = line_numbers;
    }

    // Apply diff_only from config
    if let Some(diff_only) = config.diff_only {
        first_args.diff_only = diff_only;
    }

    // Apply timestamping manually since we're bypassing run()
    if config.timestamped_output.unwrap_or(false) {
        let timestamp = Utc::now().format("%Y%m%d%H%M%S").to_string();
        let path = std::path::Path::new(&first_args.output);
        let stem = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("output");
        let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("md");
        let new_filename = format!("{}_{}.{}", stem, timestamp, extension);
        if let Some(parent) = path.parent() {
            first_args.output = parent.join(new_filename).to_string_lossy().to_string();
        } else {
            first_args.output = new_filename;
        }
    }

    run_with_args(first_args, config.clone(), &prompter).unwrap();

    // Check that output was created
    let first_output = fs::read_dir(&output_dir)
        .unwrap()
        .next()
        .unwrap()
        .unwrap()
        .path();
    let first_content = fs::read_to_string(&first_output).unwrap();

    // Should not contain change summary on first run
    assert!(!first_content.contains("## Change Summary"));
    assert!(!first_content.contains("## File Differences"));

    // Modify a file
    fs::write(
        project_dir.join("src").join("main.rs"),
        "fn main() {\n    println!(\"Hello, Rust!\");\n    println!(\"Modified!\");\n}",
    )
    .unwrap();

    // Small delay to ensure different timestamps
    std::thread::sleep(std::time::Duration::from_millis(1100));

    // Second run - should detect changes
    let config = load_config().unwrap_or_default();

    // Apply config merging manually since we're bypassing run()
    let mut second_args = args;

    // Apply line_numbers from config (matches run_with_args behavior)
    if let Some(line_numbers) = config.line_numbers {
        second_args.line_numbers = line_numbers;
    }

    // Apply diff_only from config
    if let Some(diff_only) = config.diff_only {
        second_args.diff_only = diff_only;
    }

    // Apply timestamping manually since we're bypassing run()
    if config.timestamped_output.unwrap_or(false) {
        let timestamp = Utc::now().format("%Y%m%d%H%M%S").to_string();
        let path = std::path::Path::new(&second_args.output);
        let stem = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("output");
        let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("md");
        let new_filename = format!("{}_{}.{}", stem, timestamp, extension);
        if let Some(parent) = path.parent() {
            second_args.output = parent.join(new_filename).to_string_lossy().to_string();
        } else {
            second_args.output = new_filename;
        }
    }

    run_with_args(second_args, config, &prompter).unwrap();

    // Restore original directory
    std::env::set_current_dir(original_dir).unwrap();

    // Find the second output file (should have different timestamp)
    let outputs: Vec<_> = fs::read_dir(&output_dir)
        .unwrap()
        .map(|e| e.unwrap().path())
        .collect();
    assert_eq!(outputs.len(), 2, "Should have two output files");

    let second_output = outputs.iter().find(|&p| p != &first_output).unwrap();
    let second_content = fs::read_to_string(second_output).unwrap();

    // Should contain change summary
    assert!(second_content.contains("## Change Summary"));
    // Handle both Windows and Unix path separators
    assert!(
        second_content.contains("- Modified: `src/main.rs`")
            || second_content.contains("- Modified: `src\\main.rs`")
    );

    // Should contain file differences
    assert!(second_content.contains("## File Differences"));
    assert!(
        second_content.contains("### Diff: `src/main.rs`")
            || second_content.contains("### Diff: `src\\main.rs`")
    );
    assert!(second_content.contains("Hello, world!"));
    assert!(second_content.contains("Hello, Rust!"));
    assert!(second_content.contains("Modified!"));
}

#[test]
#[serial]
fn test_auto_diff_added_and_removed_files() {
    let temp_dir = tempdir().unwrap();
    let project_dir = temp_dir.path().join("project");
    create_simple_project(&project_dir).unwrap();

    let output_dir = temp_dir.path().join("output");
    fs::create_dir_all(&output_dir).unwrap();

    // Change to project directory so config loading works
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(&project_dir).unwrap();

    let args = Args {
        input: ".".to_string(), // Use current directory
        output: output_dir.join("context.md").to_string_lossy().to_string(),
        filter: vec![],
        ignore: vec![],
        preview: false,
        token_count: false,
        line_numbers: false,
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

    let prompter = TestPrompter;

    // First run
    let config = load_config().unwrap_or_default();

    // Apply config merging manually since we're bypassing run()
    let mut first_args = args.clone();

    // Apply line_numbers from config
    if !first_args.line_numbers
        && let Some(line_numbers) = config.line_numbers
    {
        first_args.line_numbers = line_numbers;
    }

    // Apply diff_only from config
    if !first_args.diff_only
        && let Some(diff_only) = config.diff_only
    {
        first_args.diff_only = diff_only;
    }

    // Apply timestamping manually since we're bypassing run()
    if config.timestamped_output.unwrap_or(false) {
        let timestamp = Utc::now().format("%Y%m%d%H%M%S").to_string();
        let path = std::path::Path::new(&first_args.output);
        let stem = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("output");
        let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("md");
        let new_filename = format!("{}_{}.{}", stem, timestamp, extension);
        if let Some(parent) = path.parent() {
            first_args.output = parent.join(new_filename).to_string_lossy().to_string();
        } else {
            first_args.output = new_filename;
        }
    }

    run_with_args(first_args, config.clone(), &prompter).unwrap();

    // Add a new file and remove an existing one
    fs::write(
        project_dir.join("src").join("new_module.rs"),
        "pub fn new_function() -> String {\n    \"new\".to_string()\n}",
    )
    .unwrap();

    fs::remove_file(project_dir.join("src").join("lib.rs")).unwrap();

    // Small delay to ensure different timestamps
    std::thread::sleep(std::time::Duration::from_millis(1100));

    // Second run
    let config = load_config().unwrap_or_default();

    // Apply config merging manually since we're bypassing run()
    let mut second_args = args;

    // Apply line_numbers from config
    if !second_args.line_numbers
        && let Some(line_numbers) = config.line_numbers
    {
        second_args.line_numbers = line_numbers;
    }

    // Apply diff_only from config
    if !second_args.diff_only
        && let Some(diff_only) = config.diff_only
    {
        second_args.diff_only = diff_only;
    }

    // Apply timestamping manually since we're bypassing run()
    if config.timestamped_output.unwrap_or(false) {
        let timestamp = Utc::now().format("%Y%m%d%H%M%S").to_string();
        let path = std::path::Path::new(&second_args.output);
        let stem = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("output");
        let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("md");
        let new_filename = format!("{}_{}.{}", stem, timestamp, extension);
        if let Some(parent) = path.parent() {
            second_args.output = parent.join(new_filename).to_string_lossy().to_string();
        } else {
            second_args.output = new_filename;
        }
    }

    run_with_args(second_args, config, &prompter).unwrap();

    // Restore original directory
    std::env::set_current_dir(original_dir).unwrap();

    let outputs: Vec<_> = fs::read_dir(&output_dir)
        .unwrap()
        .map(|e| e.unwrap().path())
        .collect();
    let latest_output = outputs
        .iter()
        .max_by_key(|p| fs::metadata(p).unwrap().modified().unwrap())
        .unwrap();
    let content = fs::read_to_string(latest_output).unwrap();

    // Should show both added and removed files
    // Handle both Windows and Unix path separators
    assert!(
        content.contains("- Added: `src/new_module.rs`")
            || content.contains("- Added: `src\\new_module.rs`")
    );
    // Handle both Windows and Unix path separators
    assert!(
        content.contains("- Removed: `src/lib.rs`") || content.contains("- Removed: `src\\lib.rs`")
    );

    // Added files should be marked in the files section
    assert!(content.contains("_Status: Added_"));
}
```

### File: `tests/test_binary_file_autodiff.rs`

- Size: 8350 bytes
- Modified: 2026-02-15 06:55:40 UTC

```rust
//! Integration tests for binary file handling in auto-diff mode
//!
//! This test ensures that the application doesn't crash when encountering
//! binary files during auto-diff processing.

use std::fs;
use std::path::Path;
use tempfile::tempdir;

use context_builder::config::Config;
use context_builder::{Prompter, cli::Args, run_with_args};

struct TestPrompter {
    overwrite_response: bool,
    processing_response: bool,
}

impl TestPrompter {
    fn new(overwrite_response: bool, processing_response: bool) -> Self {
        Self {
            overwrite_response,
            processing_response,
        }
    }
}

impl Prompter for TestPrompter {
    fn confirm_processing(&self, _file_count: usize) -> std::io::Result<bool> {
        Ok(self.processing_response)
    }

    fn confirm_overwrite(&self, _file_path: &str) -> std::io::Result<bool> {
        Ok(self.overwrite_response)
    }
}

fn write_file(path: &Path, contents: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(path, contents).unwrap();
}

fn write_binary_file(path: &Path, data: &[u8]) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(path, data).unwrap();
}
```

### File: `tests/test_comprehensive_edge_cases.rs`

- Size: 23611 bytes
- Modified: 2026-02-15 06:55:40 UTC

```rust
//! Comprehensive edge case testing suite for context-builder v0.5.0
//!
//! This test suite covers all the critical edge cases and robustness scenarios
//! that were identified during the v0.5.0 development cycle.

use context_builder::cli::Args;
use context_builder::config::Config;
use context_builder::{Prompter, run_with_args};
use serial_test::serial;
use std::fs;
use std::path::Path;
use tempfile::tempdir;

struct TestPrompter {
    overwrite_response: bool,
    processing_response: bool,
}

impl TestPrompter {
    fn new(overwrite_response: bool, processing_response: bool) -> Self {
        Self {
            overwrite_response,
            processing_response,
        }
    }
}

impl Prompter for TestPrompter {
    fn confirm_processing(&self, _file_count: usize) -> std::io::Result<bool> {
        Ok(self.processing_response)
    }

    fn confirm_overwrite(&self, _file_path: &str) -> std::io::Result<bool> {
        Ok(self.overwrite_response)
    }
}

fn write_file(path: &Path, contents: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(path, contents).unwrap();
}

fn write_binary_file(path: &Path, data: &[u8]) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(path, data).unwrap();
}

#[test]
#[serial]
fn test_comprehensive_binary_file_edge_cases() {
    let temp_dir = tempdir().unwrap();
    let project_dir = temp_dir.path().join("project");
    let output_dir = temp_dir.path().join("output");
    fs::create_dir_all(&output_dir).unwrap();

    // Create various binary and problematic files
    write_file(&project_dir.join("src/normal.rs"), "fn main() {}\n");

    // Pure binary file (executable-like)
    let binary_data = vec![
        0x7f, 0x45, 0x4c, 0x46, // ELF header
        0x02, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];
    write_binary_file(&project_dir.join("src/binary.rs"), &binary_data);

    // File with UTF-16 BOM
    let utf16_data = [
        0xFF, 0xFE, // UTF-16 LE BOM
        0x48, 0x00, 0x65, 0x00, 0x6C, 0x00, 0x6C, 0x00, 0x6F, 0x00, // "Hello"
        0x0A, 0x00, // newline
    ];
    write_binary_file(&project_dir.join("src/utf16.rs"), &utf16_data);

    // File with Windows-1252 encoding
    let windows1252_data = [
        0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x20, // "Hello "
        0x93, 0x57, 0x6F, 0x72, 0x6C, 0x64, 0x94, // "World" with smart quotes
        0x0A, // newline
    ];
    write_binary_file(&project_dir.join("src/win1252.rs"), &windows1252_data);

    // Empty file
    write_file(&project_dir.join("src/empty.rs"), "");

    // File with only null bytes
    write_binary_file(&project_dir.join("src/nulls.rs"), &[0x00; 100]);

    // Very large file (test memory efficiency)
    let large_content = "// Large file\n".repeat(10000);
    write_file(&project_dir.join("src/large.rs"), &large_content);

    // Test with different encoding strategies
    let strategies = ["detect", "strict", "skip"];

    for strategy in &strategies {
        let config = Config {
            filter: Some(vec!["rs".to_string()]),
            encoding_strategy: Some(strategy.to_string()),
            ..Default::default()
        };

        let args = Args {
            input: project_dir.to_string_lossy().to_string(),
            output: output_dir
                .join(format!("test_{}.md", strategy))
                .to_string_lossy()
                .to_string(),
            filter: vec!["rs".to_string()],
            ignore: vec![],
            preview: false,
            token_count: false,
            line_numbers: false,
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

        let prompter = TestPrompter::new(true, true);
        let result = run_with_args(args, config, &prompter);

        assert!(
            result.is_ok(),
            "Should handle binary files gracefully with strategy: {}",
            strategy
        );

        // Verify output file was created
        let output_path = output_dir.join(format!("test_{}.md", strategy));
        assert!(
            output_path.exists(),
            "Output file should exist for strategy: {}",
            strategy
        );

        let content = fs::read_to_string(&output_path).unwrap();

        // Should contain normal file
        assert!(
            content.contains("fn main()"),
            "Should contain normal file content"
        );

        // Should handle binary files appropriately based on strategy
        match *strategy {
            "detect" => {
                // May contain transcoded content or binary placeholders
                assert!(
                    content.contains("Hello") || content.contains("<Binary file"),
                    "Detect strategy should transcode or show binary placeholder"
                );
            }
            "strict" | "skip" => {
                // Should show binary placeholders for non-UTF-8 files
                assert!(
                    content.contains("<Binary file") || content.contains("binary.rs"),
                    "Strict/skip strategy should show binary placeholders"
                );
            }
            _ => {}
        }

        // Should handle empty files
        assert!(content.contains("empty.rs"), "Should list empty files");

        // Should handle large files
        assert!(content.contains("large.rs"), "Should handle large files");
    }

    // No need to restore directory since we never changed it
}

#[test]
#[serial]
fn test_configuration_precedence_edge_cases() {
    let temp_dir = tempdir().unwrap();
    let project_dir = temp_dir.path().join("project");
    let output_dir = temp_dir.path().join("output");
    fs::create_dir_all(&output_dir).unwrap();

    // Create test files
    write_file(&project_dir.join("test.rs"), "fn test() {}\n");
    write_file(&project_dir.join("README.md"), "# Test Project\n");

    // Test 1: Basic functionality with explicit CLI args
    let args = Args {
        input: project_dir.to_string_lossy().to_string(),
        output: output_dir
            .join("basic_test.md")
            .to_string_lossy()
            .to_string(),
        filter: vec!["rs".to_string()],
        ignore: vec![],
        preview: false,
        token_count: false,
        line_numbers: false,
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

    let prompter = TestPrompter::new(true, true);
    let result = run_with_args(args, Config::default(), &prompter);
    assert!(result.is_ok(), "Basic configuration test should succeed");

    let output_path = output_dir.join("basic_test.md");
    assert!(output_path.exists(), "Output should exist for basic test");

    let content = fs::read_to_string(&output_path).unwrap();
    assert!(
        content.contains("test.rs"),
        "Should include filtered .rs files"
    );
    assert!(
        !content.contains("README.md"),
        "Should exclude non-filtered files"
    );

    // Test 2: Empty filter should include all files
    let args = Args {
        input: project_dir.to_string_lossy().to_string(),
        output: output_dir
            .join("all_files_test.md")
            .to_string_lossy()
            .to_string(),
        filter: vec![],
        ignore: vec![],
        preview: false,
        token_count: false,
        line_numbers: false,
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

    let result = run_with_args(args, Config::default(), &prompter);
    assert!(result.is_ok(), "All files test should succeed");

    let output_path = output_dir.join("all_files_test.md");
    let content = fs::read_to_string(&output_path).unwrap();
    assert!(
        content.contains("test.rs"),
        "Should include all files when no filter"
    );
    assert!(
        content.contains("README.md"),
        "Should include all files when no filter"
    );
}
```

### File: `tests/test_config_resolution.rs`

- Size: 15023 bytes
- Modified: 2026-02-15 06:55:40 UTC

```rust
//! Integration tests for configuration resolution functionality
//!
//! These tests verify that the new config resolver properly merges CLI arguments
//! with configuration file values according to the correct precedence rules.

use serial_test::serial;
use std::fs;
use std::path::Path;
use tempfile::tempdir;

use context_builder::{Prompter, cli::Args, config_resolver::resolve_final_config, run_with_args};

struct TestPrompter {
    overwrite_response: bool,
    processing_response: bool,
}

impl TestPrompter {
    fn new(overwrite_response: bool, processing_response: bool) -> Self {
        Self {
            overwrite_response,
            processing_response,
        }
    }
}

impl Prompter for TestPrompter {
    fn confirm_processing(&self, _file_count: usize) -> std::io::Result<bool> {
        Ok(self.processing_response)
    }

    fn confirm_overwrite(&self, _file_path: &str) -> std::io::Result<bool> {
        Ok(self.overwrite_response)
    }
}

fn write_file(path: &Path, contents: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(path, contents).unwrap();
}

/// Helper function that mimics the run() function's config resolution logic
fn run_with_resolved_config(
    args: Args,
    config: Option<context_builder::config::Config>,
    prompter: &impl Prompter,
) -> std::io::Result<()> {
    // Resolve final configuration using the new config resolver
    let resolution = resolve_final_config(args, config.clone());

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
        init: resolution.config.init,
        max_tokens: resolution.config.max_tokens,
        signatures: resolution.config.signatures,
        structure: resolution.config.structure,
        truncate: resolution.config.truncate,
        visibility: resolution.config.visibility,
    };

    // Create final Config with resolved values
    let final_config = context_builder::config::Config {
        auto_diff: Some(resolution.config.auto_diff),
        diff_context_lines: Some(resolution.config.diff_context_lines),
        ..config.unwrap_or_default()
    };

    run_with_args(final_args, final_config, prompter)
}

#[test]
#[serial]
fn test_cli_arguments_override_config_file() {
    let temp_dir = tempdir().unwrap();
    let project_dir = temp_dir.path().join("project");
    let output_dir = temp_dir.path().join("output");

    // Create a simple project
    write_file(
        &project_dir.join("src/main.rs"),
        "fn main() { println!(\"Hello\"); }",
    );
    write_file(&project_dir.join("lib.py"), "def hello(): print('world')");

    // Create config file with specific settings
    write_file(
        &project_dir.join("context-builder.toml"),
        r#"
filter = ["py"]
line_numbers = true
output = "from_config.md"
"#,
    );

    fs::create_dir_all(&output_dir).unwrap();

    // CLI args that should override config
    // Change to project directory (run_with_args creates output relative to CWD)
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(&project_dir).unwrap();

    let args = Args {
        input: ".".to_string(), // Use current directory
        output: output_dir.join("from_cli.md").to_string_lossy().to_string(),
        filter: vec!["rs".to_string()], // Should override config's ["py"]
        ignore: vec![],
        line_numbers: true, // Can't override config boolean settings
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

    let config = context_builder::config::load_config_from_path(&project_dir).unwrap();
    let prompter = TestPrompter::new(true, true);

    let result = run_with_resolved_config(args, Some(config), &prompter);

    // Restore original directory
    std::env::set_current_dir(original_dir).unwrap();
    assert!(result.is_ok(), "Should succeed with CLI override");

    // Verify output file was created with CLI name, not config name
    let output_file = output_dir.join("from_cli.md");
    assert!(output_file.exists(), "Output file should use CLI filename");

    let content = fs::read_to_string(&output_file).unwrap();

    // Should contain .rs file (CLI filter), not .py file (config filter)
    assert!(
        content.contains("main.rs"),
        "Should include .rs files from CLI filter"
    );
    assert!(
        !content.contains("lib.py"),
        "Should not include .py files despite config filter"
    );

    // Should have line numbers (config applies since we can't distinguish CLI false from default)
    assert!(
        content.contains("   1 |"),
        "Should have line numbers from config"
    );
}
```

### File: `tests/test_cwd_independence.rs`

- Size: 13739 bytes
- Modified: 2026-02-15 06:55:40 UTC

```rust
//! Integration tests for CWD independence
//!
//! This test verifies that the application loads config and creates cache
//! relative to the project root, not the current working directory.

use serial_test::serial;
use std::fs;
use std::path::Path;
use tempfile::tempdir;

use context_builder::{Prompter, cli::Args, run_with_args};

struct TestPrompter {
    overwrite_response: bool,
    processing_response: bool,
}

impl TestPrompter {
    fn new(overwrite_response: bool, processing_response: bool) -> Self {
        Self {
            overwrite_response,
            processing_response,
        }
    }
}

impl Prompter for TestPrompter {
    fn confirm_processing(&self, _file_count: usize) -> std::io::Result<bool> {
        Ok(self.processing_response)
    }

    fn confirm_overwrite(&self, _file_path: &str) -> std::io::Result<bool> {
        Ok(self.overwrite_response)
    }
}

fn write_file(path: &Path, contents: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(path, contents).unwrap();
}

#[test]
#[serial]
fn test_config_loaded_from_project_root_not_cwd() {
    let temp_dir = tempdir().unwrap();
    let project_dir = temp_dir.path().join("project");
    let output_dir = temp_dir.path().join("output");
    let working_dir = temp_dir.path().join("working");

    // Create project with config file
    write_file(
        &project_dir.join("src/main.rs"),
        "fn main() { println!(\"Hello\"); }",
    );
    write_file(
        &project_dir.join("context-builder.toml"),
        r#"
auto_diff = true
line_numbers = true
filter = ["rs"]
"#,
    );

    // Create different config in working directory (should be ignored)
    write_file(
        &working_dir.join("context-builder.toml"),
        r#"
auto_diff = false
line_numbers = false
filter = ["txt"]
"#,
    );

    fs::create_dir_all(&output_dir).unwrap();
    fs::create_dir_all(&working_dir).unwrap();

    // Change to working directory
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(&working_dir).unwrap();

    // Load config from project directory (not CWD)
    let config = context_builder::config::load_config_from_path(&project_dir).unwrap_or_default();

    let mut args = Args {
        input: project_dir.to_string_lossy().to_string(), // Absolute path to project
        output: output_dir.join("output.md").to_string_lossy().to_string(),
        filter: vec![], // Should be overridden by project config
        ignore: vec![],
        preview: false,
        token_count: false,
        line_numbers: false, // Should be overridden by project config
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

    // Apply config settings to args (mimicking the run() function logic)
    if args.filter.is_empty()
        && let Some(filter) = config.filter.clone()
    {
        args.filter = filter;
    }
    if !args.line_numbers
        && let Some(line_numbers) = config.line_numbers
    {
        args.line_numbers = line_numbers;
    }

    let prompter = TestPrompter::new(true, true);
    let result = run_with_args(args, config, &prompter);

    // Restore original directory
    std::env::set_current_dir(original_dir).unwrap();

    assert!(result.is_ok(), "Should succeed with CWD independence");

    let output_content = fs::read_to_string(output_dir.join("output.md")).unwrap();

    // Verify that project config was used, not working directory config
    assert!(
        output_content.contains("   1 |"),
        "Should have line numbers from project config"
    );
    assert!(
        output_content.contains("main.rs"),
        "Should include .rs files from project config filter"
    );
}
```

### File: `tests/test_determinism.rs`

- Size: 21261 bytes
- Modified: 2026-02-15 06:55:40 UTC

```rust
//! Integration tests for determinism and robustness of context-builder
//!
//! These tests verify that the critical bug fixes are working correctly:
//! - Deterministic output order
//! - Robust caching
//! - Thread safety

use pretty_assertions::assert_eq;
use serial_test::serial;
use std::fs;
use std::path::Path;
use tempfile::tempdir;

use chrono::Utc;
use context_builder::cli::Args;
use context_builder::config::{Config, load_config};
use context_builder::{Prompter, run_with_args};

/// Test prompter that always confirms
struct TestPrompter;

impl Prompter for TestPrompter {
    fn confirm_processing(&self, _file_count: usize) -> std::io::Result<bool> {
        Ok(true)
    }
    fn confirm_overwrite(&self, _file_path: &str) -> std::io::Result<bool> {
        Ok(true)
    }
}

/// Create a test project with multiple files in different directories
fn create_test_project(base_dir: &Path) -> std::io::Result<()> {
    let src_dir = base_dir.join("src");
    let tests_dir = base_dir.join("tests");
    let docs_dir = base_dir.join("docs");

    fs::create_dir_all(&src_dir)?;
    fs::create_dir_all(&tests_dir)?;
    fs::create_dir_all(&docs_dir)?;

    // Create files in different orders to test sorting
    fs::write(
        src_dir.join("main.rs"),
        "fn main() {\n    println!(\"Hello\");\n}",
    )?;
    fs::write(src_dir.join("lib.rs"), "pub mod utils;\npub mod config;")?;
    fs::write(src_dir.join("utils.rs"), "pub fn helper() {}")?;
    fs::write(
        tests_dir.join("integration.rs"),
        "#[test]\nfn test_something() {}",
    )?;
    fs::write(tests_dir.join("unit.rs"), "#[test]\nfn test_unit() {}")?;
    fs::write(
        docs_dir.join("README.md"),
        "# Project\n\nThis is a test project.",
    )?;
    fs::write(
        base_dir.join("Cargo.toml"),
        "[package]\nname = \"test\"\nversion = \"0.1.0\"",
    )?;

    Ok(())
}

#[test]
#[serial] // Ensure tests don't interfere with each other
fn test_deterministic_output_multiple_runs() {
    let temp_dir = tempdir().unwrap();
    let project_dir = temp_dir.path().join("project");
    create_test_project(&project_dir).unwrap();

    // Note: The actual output files may have timestamps appended due to auto-diff mode
    // We'll need to find the actual files created
    let prompter = TestPrompter;

    // Run twice with identical arguments
    let result1 = run_with_args(
        Args {
            input: project_dir.to_string_lossy().to_string(),
            output: temp_dir
                .path()
                .join("output1.md")
                .to_string_lossy()
                .to_string(),
            filter: vec!["rs".to_string(), "md".to_string(), "toml".to_string()],
            ignore: vec![],
            preview: false,
            token_count: false,
            line_numbers: false,
            yes: true,
            diff_only: false,
            clear_cache: false,
            init: false,
            max_tokens: None,
            signatures: false,
            structure: false,
            truncate: "smart".to_string(),
            visibility: "all".to_string(),
        },
        Config::default(),
        &prompter,
    );

    let result2 = run_with_args(
        Args {
            input: project_dir.to_string_lossy().to_string(),
            output: temp_dir
                .path()
                .join("output2.md")
                .to_string_lossy()
                .to_string(),
            filter: vec!["rs".to_string(), "md".to_string(), "toml".to_string()],
            ignore: vec![],
            preview: false,
            token_count: false,
            line_numbers: false,
            yes: true,
            diff_only: false,
            clear_cache: false,
            init: false,
            max_tokens: None,
            signatures: false,
            structure: false,
            truncate: "smart".to_string(),
            visibility: "all".to_string(),
        },
        Config::default(),
        &prompter,
    );

    if let Err(e) = result1 {
        panic!("First run failed: {}", e);
    }
    if let Err(e) = result2 {
        panic!("Second run failed: {}", e);
    }

    // Find the actual output files (they may have timestamps appended)
    let temp_entries: Vec<_> = fs::read_dir(temp_dir.path())
        .unwrap()
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            let file_name = entry.file_name();
            let name = file_name.to_string_lossy();
            name.starts_with("output") && name.ends_with(".md")
        })
        .collect();

    if temp_entries.len() < 2 {
        eprintln!("Expected 2 output files, found {}", temp_entries.len());
        eprintln!("Temp directory contents:");
        for entry in fs::read_dir(temp_dir.path()).unwrap() {
            eprintln!("  {:?}", entry.unwrap().file_name());
        }
        panic!("Not enough output files found");
    }

    // Sort to ensure consistent ordering
    let mut output_files: Vec<_> = temp_entries.iter().map(|entry| entry.path()).collect();
    output_files.sort();

    // Read both outputs
    let content1 = fs::read_to_string(&output_files[0]).unwrap();
    let content2 = fs::read_to_string(&output_files[1]).unwrap();

    // Debug: Write contents to temp files for inspection
    fs::write(temp_dir.path().join("debug_content1.md"), &content1).unwrap();
    fs::write(temp_dir.path().join("debug_content2.md"), &content2).unwrap();

    // Normalize timestamps for comparison since they will be different
    let normalize = |content: &str| -> String {
        content
            .lines()
            .map(|line| {
                if line.starts_with("Processed at: ") {
                    "Processed at: <timestamp>"
                } else {
                    line
                }
            })
            .collect::<Vec<_>>()
            .join("\n")
    };

    let normalized1 = normalize(&content1);
    let normalized2 = normalize(&content2);

    // Debug: Write normalized contents for comparison
    fs::write(temp_dir.path().join("debug_normalized1.md"), &normalized1).unwrap();
    fs::write(temp_dir.path().join("debug_normalized2.md"), &normalized2).unwrap();

    // They should be identical (deterministic) after normalizing timestamps
    if normalized1 != normalized2 {
        eprintln!(
            "Content1 length: {}, Content2 length: {}",
            normalized1.len(),
            normalized2.len()
        );
        eprintln!(
            "First difference at position: {:?}",
            normalized1
                .chars()
                .zip(normalized2.chars())
                .position(|(a, b)| a != b)
        );
        eprintln!("Debug files written to: {}", temp_dir.path().display());
        panic!("Output should be deterministic across multiple runs (ignoring timestamps)");
    }

    // Verify that files are listed in a consistent order
    let lines: Vec<&str> = content1.lines().collect();
    let file_lines: Vec<&str> = lines
        .iter()
        .filter(|line| line.starts_with("### File: `"))
        .copied()
        .collect();

    // Should have found some files
    assert!(
        !file_lines.is_empty(),
        "Should have found some file entries"
    );

    // Check that files are sorted by relevance category:
    // Category 0: Cargo.toml (config), README.md (key project doc)
    // Category 1: src/* (source code) — entry points first (lib.rs, main.rs before utils.rs)
    // Category 2: tests/* (tests)
    let expected_order = vec![
        "### File: `Cargo.toml`",
        "### File: `docs/README.md`",
        "### File: `src/lib.rs`",
        "### File: `src/main.rs`",
        "### File: `src/utils.rs`",
        "### File: `tests/integration.rs`",
        "### File: `tests/unit.rs`",
    ];
    assert_eq!(
        file_lines, expected_order,
        "Files should be listed in relevance order (config+docs → source (entry points first) → tests)"
    );
}
#[test]
#[serial] // Ensure tests don't interfere with each other
fn test_deterministic_file_tree_order() {
    let temp_dir = tempdir().unwrap();
    let project_dir = temp_dir.path().join("project");
    create_test_project(&project_dir).unwrap();

    let output_path = temp_dir.path().join("output.md");

    // Change to project directory so config loading works
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(&project_dir).unwrap();

    let args = Args {
        input: ".".to_string(),
        output: output_path.to_string_lossy().to_string(),
        filter: vec![],
        ignore: vec![],
        preview: false,
        token_count: false,
        line_numbers: false,
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

    let prompter = TestPrompter;
    run_with_args(args, Config::default(), &prompter).unwrap();

    // Restore original directory
    std::env::set_current_dir(original_dir).unwrap();

    let content = fs::read_to_string(&output_path).unwrap();

    // Find the file tree section
    let tree_start = content
        .find("## File Tree Structure")
        .expect("Should have file tree section");
    let files_start = content.find("### File: `").unwrap_or(content.len());
    let tree_section = &content[tree_start..files_start];

    // Check that directories and files appear in alphabetical order in the tree
    // This is a basic check - a more sophisticated test would parse the tree structure
    assert!(tree_section.contains("Cargo.toml"));
    // Check for directory entries - they may appear as just the name or with trailing content
    assert!(tree_section.contains("docs") || tree_section.contains("docs/"));
    assert!(tree_section.contains("src") || tree_section.contains("src/"));
    assert!(tree_section.contains("tests") || tree_section.contains("tests/"));
}
```

### File: `tests/test_parallel_memory.rs`

- Size: 9136 bytes
- Modified: 2026-02-15 06:55:40 UTC

```rust
//! Integration test for streaming parallel processing with memory efficiency

use context_builder::cli::Args;
use context_builder::config::Config;
use context_builder::{Prompter, run_with_args};
use std::fs;

use tempfile::tempdir;

struct TestPrompter {
    overwrite_response: bool,
    processing_response: bool,
}

impl TestPrompter {
    fn new(overwrite_response: bool, processing_response: bool) -> Self {
        Self {
            overwrite_response,
            processing_response,
        }
    }
}

impl Prompter for TestPrompter {
    fn confirm_processing(&self, _file_count: usize) -> std::io::Result<bool> {
        Ok(self.processing_response)
    }

    fn confirm_overwrite(&self, _file_path: &str) -> std::io::Result<bool> {
        Ok(self.overwrite_response)
    }
}
```

### File: `tests/test_phase4_integration.rs`

- Size: 11358 bytes
- Modified: 2026-02-15 06:55:40 UTC

```rust
//! Integration test for all Phase 4 features working together
//!
//! This test validates that the enhanced binary file handling, improved diff_only mode,
//! and comprehensive edge case handling all work correctly in combination.

use context_builder::cli::Args;
use context_builder::config::Config;
use context_builder::{Prompter, run_with_args};
use std::fs;
use std::path::Path;
use tempfile::tempdir;

struct TestPrompter {
    overwrite_response: bool,
    processing_response: bool,
}

impl TestPrompter {
    fn new(overwrite_response: bool, processing_response: bool) -> Self {
        Self {
            overwrite_response,
            processing_response,
        }
    }
}

impl Prompter for TestPrompter {
    fn confirm_processing(&self, _file_count: usize) -> std::io::Result<bool> {
        Ok(self.processing_response)
    }

    fn confirm_overwrite(&self, _file_path: &str) -> std::io::Result<bool> {
        Ok(self.overwrite_response)
    }
}

fn write_file(path: &Path, contents: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(path, contents).unwrap();
}

fn write_binary_file(path: &Path, data: &[u8]) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(path, data).unwrap();
}
```

### File: `BENCHMARKS.md`

- Size: 6024 bytes
- Modified: 2026-02-14 07:14:48 UTC

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
- Modified: 2026-02-14 07:14:48 UTC

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

### File: `LICENSE`

- Size: 1078 bytes
- Modified: 2026-02-14 07:14:48 UTC

```text
The MIT License

Copyright (c) 2025 Igor Lins e Silva

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
THE SOFTWARE.
```

### File: `SKILL.md`

- Size: 6238 bytes
- Modified: 2026-02-15 05:22:34 UTC

```markdown
---
name: context-builder
description: Generate LLM-optimized codebase context from any directory using context-builder CLI
homepage: https://github.com/igorls/context-builder
version: 0.7.1
requires:
  - cargo
  - context-builder
---

# Context Builder — Agentic Skill

Generate a single, structured markdown file from any codebase directory. The output is optimized for LLM consumption with relevance-based file ordering, automatic token budgeting, and smart defaults.

## Installation

```bash
cargo install context-builder
```

Verify: `context-builder --version`

## When to Use

- **Deep code review** — Feed an entire codebase to an LLM for architecture analysis or bug hunting
- **Onboarding** — Generate a project snapshot for understanding unfamiliar codebases
- **Diff-based updates** — After code changes, generate only the diffs to update an LLM's understanding
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

### 3. Budget-Constrained Context

```bash
context-builder -d /path/to/project --max-tokens 100000 -y -o context.md
```

- Caps output to ~100K tokens (estimated)
- Files are included in relevance order until budget is exhausted
- Automatically warns if output exceeds 128K tokens

### 4. Token Count Preview

```bash
context-builder -d /path/to/project --token-count
```

- Prints estimated token count without generating output
- Use this first to decide if filtering is needed

### 5. Incremental Diffs

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
| `-d <PATH>` | Input directory | Always use absolute paths for reliability |
| `-o <FILE>` | Output path | Write to a temp or docs directory |
| `-f <EXT>` | Filter by extension | Comma-separated: `-f rs,toml,md` |
| `-i <NAME>` | Ignore dirs/files | Comma-separated: `-i tests,docs,assets` |
| `--max-tokens <N>` | Token budget cap | Use `100000` for most models, `200000` for Gemini |
| `--token-count` | Dry-run token estimate | Run first to check if filtering is needed |
| `-y` | Skip all prompts | **Always use in agent workflows** |
| `--preview` | Show file tree only | Quick exploration without generating output |
| `--diff-only` | Output only diffs | Minimizes tokens for incremental updates |
| `--init` | Create config file | Auto-detects project file types |

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

# If > 128K tokens, scope it down:
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

## Error Handling

- If `context-builder` is not installed, install with `cargo install context-builder`
- If output exceeds token limits, add `--max-tokens` or narrow with `-f` / `-i`
- If the project has no `.git` directory, auto-ignores still protect against dependency flooding
- Use `--clear-cache` if diff output seems stale or incorrect
```

### File: `docs/research/multi-model-code-review-analysis.md`

- Size: 19787 bytes
- Modified: 2026-02-14 21:36:34 UTC

```markdown
# Multi-Model AI Code Review: 10 LLMs Analyze context-builder v0.7.0

> **Date**: February 14, 2026  
> **Project**: [context-builder](https://github.com/igorls/context-builder) v0.7.0  
> **Prompt**: Deep Think v2 — code review + relevance ordering evaluation  
> **Context file**: ~460KB, generated by context-builder itself (self-referential review)

## Abstract

We submitted context-builder v0.7.0's full source code to 10 AI models using a structured code review prompt. The context was generated by context-builder itself, creating a self-referential test: the tool's output quality directly affected the models' ability to review it. This exposed real bugs (e.g., lockfile starvation) that only manifested *because* the models consumed the tool's own output.

The models collectively identified **10 unique bugs**, achieved **universal consensus** on 4 architectural improvements, and proposed a clear roadmap for v2 output format. This document captures the full comparative analysis.

---

## 1. Methodology

### 1.1 Prompt Design

All models received the same structured prompt ("Deep Think v2") containing:
1. **Role**: Senior Rust developer and CLI tool architect
2. **Task**: Review context-builder v0.7.0 with focus on correctness, architecture, and a new relevance ordering feature
3. **Context**: The full output of `context-builder -d . --filter rs,toml,md` run against its own repository
4. **Specific asks**: Verify 5 bug fixes from v0.6.1→v0.7.0, evaluate relevance ordering, propose v2 output format

### 1.2 Model Access Methods

| Method | Models | Description |
|--------|--------|-------------|
| 🧠 **One-shot** | Gemini Deep Think, Gemini Pro, Grok 4.1, Qwen-3-Max, GLM-5 (×2), ChatGPT 5.2, Kimi K2.5 | Single prompt + context file, no tool access |
| 🤖 **Agentic** | Claude Opus 4.6, MiniMax Agent 2.5 | Multi-step sessions with file search, workspace tools, iterative reasoning |

> **Why this matters**: One-shot models that found deep bugs did so purely through reasoning over 460KB of context in a single pass. Agentic models could iteratively search, grep, and verify — giving them a structural advantage for precision but not necessarily for discovery depth.

### 1.3 Lockfile Fix (Mid-Experiment)

GLM-5's first run (pre-fix) was truncated because `Cargo.lock` (39KB) was classified as config (category 0), consuming the entire context window before source code appeared. We fixed this bug (moving lockfiles to category 4) and re-ran GLM-5, creating a natural A/B test of the fix's effectiveness.

Post-fix models (ChatGPT 5.2, Kimi K2.5, Claude Opus 4.6, MiniMax Agent, GLM-5 run 2) reviewed the corrected ordering.

---

## 2. Models Tested

| # | Model | Method | Context | Lockfile Fix | Quality |
|---|-------|--------|---------|-------------|---------|
| 1 | **Gemini 3 Deep Think** | 🧠 One-shot | Full | Pre-fix | 🟢 Excellent |
| 2 | **Gemini 3 Pro** | 🧠 One-shot | Full | Pre-fix | 🟢 Good |
| 3 | **Grok 4.1** | 🧠 One-shot | Full | Pre-fix | 🟡 Average |
| 4 | **Qwen-3-Max** | 🧠 One-shot | Full | Pre-fix | 🟢 Good |
| 5 | **GLM-5** (run 1, pre-fix) | 🧠 One-shot | Truncated | Pre-fix | 🔴 Truncated |
| 6 | **GLM-5** (run 2, post-fix) | 🧠 One-shot | Partial | Post-fix | 🟡 Partial |
| 7 | **ChatGPT 5.2** | 🧠 One-shot | Full | Post-fix | 🟢 Excellent |
| 8 | **Kimi K2.5** | 🧠 One-shot | Full | Post-fix | 🟢 Good |
| 9 | **Claude Opus 4.6** | 🤖 Agentic | Full | Post-fix | 🟢 Excellent |
| 10 | **MiniMax Agent 2.5** | 🤖 Agentic | Full | Post-fix | 🟢 Good |

> **Response files**: Each model's raw response is archived in [`docs/`](../):
> `context_v2_resp-gemini-3-deepthink.md`, `context_v2_resp-gemini-3-pro.md`, `context_v2_resp-grok-4.1.md`, `context_v2_resp-qwen-3-max.md`, `context_v2_resp-glm5.md`, `context_v2_resp-glm5-run2.md`, `context_v2_resp-chat-gpt-5.2.md`, `context_v2_resp-kimi-k2.5.md`, `context_v2_resp-claude-opus-4.6.md`, `context_v2_resp-minimax-agent.md`

---

## 3. Results: Model Rankings

### 3.1 Overall (Raw Output Quality)

| Rank | Model | Method | Unique Bugs | Novel Ideas | v0.6.1 Fix Verification |
|------|-------|--------|------------|-------------|------------------------|
| 🥇 | **Claude Opus 4.6** | 🤖 Agentic | 2 | Signatures-first format, `[category:N]` tags | 5/5 ✅ |
| 🥈 | **Gemini Deep Think** | 🧠 One-shot | 2 | XML CDATA format, BTreeMap ordering bug | 5/5 ✅ |
| 🥉 | **ChatGPT 5.2** | 🧠 One-shot | 1 | Tests-before-source ordering | 5/5 ✅ |
| 4th | **Qwen-3-Max** | 🧠 One-shot | 1 | Progressive disclosure | 5/5 ✅ |
| 5th | **MiniMax Agent 2.5** | 🤖 Agentic | 0 | XML w/ purpose summaries | 5/5 ✅ |
| 6th | **Kimi K2.5** | 🧠 One-shot | 0 | `--docs-first` flag, heat map | 5/5 ✅ |
| 7th | **Gemini Pro** | 🧠 One-shot | 0 | — | 5/5 ✅ |
| 8th | **GLM-5** (run 2) | 🧠 One-shot | 1 | Reliability column concept | 2/5 |
| 9th | **Grok 4.1** | 🧠 One-shot | 0 | — | 5/5 ✅ |
| 10th | **GLM-5** (run 1) | 🧠 One-shot | 1 (accidental) | — | 0/5 (truncated) |

### 3.2 Adjusted (Normalized for Methodology)

When accounting for the agentic advantage (tool access, iterative search, workspace), the efficiency picture changes:

| Rank | Model | Reasoning |
|------|-------|-----------|
| 🥇 | **Gemini Deep Think** 🧠 | Found 2 unique *architectural* bugs (BTreeMap ordering, DefaultHasher non-determinism) in a single reasoning pass — no tools, no iteration. The auto-diff BTreeMap bug was the deepest finding across all 10 models, found by no other. |
| 🥈 | **ChatGPT 5.2** 🧠 | Most original thinker. Proposed tests-before-source ordering that no other model considered. One unique bug. Entirely one-shot. |
| 🥉 | **Claude Opus 4.6** 🤖 | Highest raw quality, but had agentic advantage. Line-level citations and signatures-first v2 format are best-in-class, but required multi-step tool-assisted processing. |
| 4th | **Qwen-3-Max** 🧠 | Strong one-shot. Caught parallel `max_tokens` and header token omission bugs independently. |
| 5th | **Kimi K2.5** 🧠 | Solid one-shot with practical, implementable suggestions (`--docs-first` flag). |

**Key insight**: Deep Think matched Claude Opus in bug *discovery depth* with zero tool access — pure one-shot reasoning over 460KB of context. This suggests extended reasoning traces are competitive with agentic tool-use for code review tasks. The agentic advantage primarily manifests in citation precision (exact line numbers), not bug discovery.

---

## 4. Bug Matrix

### 4.1 All Bugs Found (10 unique)

| # | Bug | Severity | DT | GP | GR | QW | G5 | G5² | GPT | K2 | CO | MM | Consensus |
|---|-----|----------|----|----|----|----|----|----|-----|----|----|----|----|
| 1 | `max_tokens` ignored in parallel mode | 🔴 Critical | ✅ | | | ✅ | | | | ✅ | ✅ | | 4/10 |
| 2 | mtime hash ≠ content hash (breaks determinism) | 🔴 Critical | ✅ | ✅ | | ✅ | | | ✅ | | ✅ | ✅ | 6/10 |
| 3 | `DefaultHasher` non-deterministic across Rust versions | 🟡 High | ✅ | | | | | | ✅ | | ✅ | ✅ | 4/10 |
| 4 | Auto-diff `BTreeMap` destroys relevance ordering | 🔴 Critical | ✅ | | | | | | | | | | 1/10 |
| 5 | Header/tree tokens not counted in budget | 🟡 High | | | | ✅ | | | | | ✅ | | 2/10 |
| 6 | `contains("test")` substring false positives | 🟡 Medium | | | | ✅ | | | | ✅ | ✅ | | 3/10 |
| 7 | `strip_prefix('+')` incomplete for diff indentation | 🟢 Low | ✅ | | | | | | | | | | 1/10 |
| 8 | 4-byte/token estimate ~20% off for code | 🟡 High | | | | | | | ✅ | ✅ | ✅ | ✅ | 4/10 |
| 9 | Binary file content stored as `String` in cache | 🟢 Low | | | | | | ✅ | | | | | 1/10 |
| 10 | `starts_with("test_")` matches root-level helpers | 🟡 Medium | | | | | | | | | ✅ | | 1/10 |

**Legend**: DT=Deep Think, GP=Gemini Pro, GR=Grok, QW=Qwen, G5=GLM-5 run 1, G5²=GLM-5 run 2, GPT=ChatGPT 5.2, K2=Kimi K2.5, CO=Claude Opus, MM=MiniMax

### 4.2 Bug Descriptions

**#1 — `max_tokens` ignored in parallel mode** (4/10): The `--max-tokens` flag is only enforced in the sequential code path. When parallel processing is used, files are concatenated without budget enforcement, potentially exceeding the token limit.

**#2 — mtime hash ≠ content hash** (6/10): The cache uses `file.modified()` timestamp for hashing. A `git checkout` or `cp` changes mtime without changing content → different hash → broken prompt caching across machines, CI environments, or even consecutive runs.

**#3 — `DefaultHasher` non-deterministic** (4/10): Rust's `std::hash::DefaultHasher` explicitly does not guarantee stable output across compiler versions or architectures. This silently breaks cache invalidation when the binary is compiled with different toolchains.

**#4 — Auto-diff BTreeMap destroys relevance** (1/10, Deep Think only): When `auto_diff = true` and full file content is rendered, the code iterates over `BTreeMap<PathBuf, FileState>` — which is alphabetically ordered by path, completely overriding the relevance-based sort from `file_utils.rs`. Since `auto_diff = true` is the recommended config, **relevance ordering doesn't work in the primary usage path**.

**#5 — Header/tree tokens not in budget** (2/10): The file tree header and section headers consume tokens but aren't deducted from `max_tokens`, meaning the actual file content budget is smaller than specified.

**#6 — Test substring false positives** (3/10): `rel_str.contains("test")` matches files like `latest_results.rs` or `contest_entry.rs`. Fixed during this experiment by switching to path boundary matching.

**#7 — `strip_prefix('+')` incomplete** (1/10, Deep Think only): Unified diff format uses `+ code` (plus, space, code). `strip_prefix('+')` removes the `+` but leaves the leading space, causing indentation mismatch.

**#8 — 4-byte/token estimate inaccurate** (4/10): The hardcoded 4 bytes/token ratio is an average across natural language. Code (which has more symbols, shorter identifiers) typically runs closer to 3.2 bytes/token, making estimates ~20% off.

**#9 — Binary content as String** (1/10, GLM-5 run 2 only): Binary file content passed through `String` type in the cache layer, potentially causing encoding issues or silent corruption.

**#10 — `starts_with("test_")` matches root helpers** (1/10, Claude Opus only): Root-level test helper files (e.g., `test_utils.rs`) in the project root would be classified as test files even if they're shared helpers used by production code.

---

## 5. Consensus: Relevance Ordering

### 5.1 Universal Agreement

All 10 models agreed on these points:
- ✅ Config-first ordering is correct (10/10)
- ✅ Lockfiles should be last or excluded (10/10)
- ✅ Entry points (`main.rs`, `lib.rs`) should be elevated within source (9/10)
- ✅ Alphabetical ordering within categories is suboptimal (9/10)

### 5.2 Where Should Docs Go?

| Position | Models | Count |
|----------|--------|-------|
| Core docs FIRST (README before source) | Deep Think, ChatGPT 5.2, GLM-5², Claude Opus | 4 |
| Docs after config, before source | Qwen, Kimi K2.5, MiniMax | 3 |
| Docs LAST (current behavior) | Grok, Gemini Pro | 2 |
| Configurable via flag | Kimi K2.5 (`--docs-first`) | 1 |

**Winner (7/10)**: Core docs (README, AGENTS.md) should appear before source code.

### 5.3 Proposed Category System

Based on consensus across all models:

```
0 — Core Docs      (README.md, AGENTS.md, ARCHITECTURE.md)
1 — Config/Manifest (Cargo.toml, package.json, pyproject.toml)
2 — Build/CI        (.github/, Dockerfile, build.rs, Makefile)
3 — Source          (src/*, entry points elevated)
4 — Tests/Benches   (tests/*, benches/*)
5 — Other Docs      (CHANGELOG, DEVELOPMENT, etc.)
6 — Generated/Lock  (Cargo.lock, package-lock.json — or excluded)
```

### 5.4 Intra-Category Ordering

| Approach | Models Proposing | Feasibility |
|----------|-----------------|-------------|
| Entry points first (`main.rs`, `lib.rs`) | DT, GPT, K2, CO, G5², QW | Simple |
| Dependency/topological sort | DT, GPT, K2, CO, G5², MM | Medium |
| File size ascending | CO | Simple |
| Centrality score (most-imported) | CO, GPT | Medium-Large |

### 5.5 Novel Proposal: Tests Before Source

ChatGPT 5.2 proposed a radical reordering unique among all models:

```
config → public API source → TESTS → internal source → docs
```

**Rationale**: "LLM reasoning improves when expectation is known *before* implementation." Tests encode invariants, usage intent, and hidden contracts — reading them first allows deductive (not inductive) code comprehension.

---

## 6. Consensus: Tier 2 Features

| Feature | Models Proposing | Total |
|---------|-----------------|-------|
| **Dependency graph / module map** | DT, GP, GR, QW, G5, G5², GPT, K2, CO | **9/10** |
| **Signature-only / skeleton mode** | DT, GP, QW, G5, G5², K2, CO, MM | **8/10** |
| **Git-aware context / change heatmap** | DT, GP, GR, G5, G5², K2, MM | **7/10** |
| **Semantic chunking (AST-aware)** | QW, G5, K2, CO, MM | **5/10** |
| **Structured output (XML/JSON)** | DT, G5, MM | **3/10** |
| **Smart diff (move/rename detection)** | GR, CO, MM | **3/10** |
| **Cross-reference annotations** | QW, GPT, CO | **3/10** |
| **Interactive query mode** | DT, MM | **2/10** |

**Top 3 for next implementation cycle**:
1. 🥇 Dependency graph / module map (9/10 — near-universal consensus)
2. 🥈 Signature-only mode (8/10 — critical for token budget management)
3. 🥉 Git-aware context (7/10 — recent changes as relevance signal)

---

## 7. Consensus: Output Format v2

### 7.1 Format Preference

| Format | Models | Argument |
|--------|--------|----------|
| Enhanced Markdown | GPT, K2, CO, G5² | Human-readable AND LLM-friendly |
| XML with CDATA | DT, MM | Prevents code block inception, machine-parseable |
| Markdown default + `--format` flag | K2, MM | Backward compatible |

### 7.2 Structural Consensus

Every model independently proposed a v2 format following this general structure:

```
1. Project metadata (name, version, hash, token count)
2. Architecture overview (natural language + dependency graph)
3. File manifest table (path, category, size, tokens, purpose)
4. Optional: Public API / signatures section
5. Full file contents (with per-file metadata headers)
6. Truncation notice if budget exceeded
```

### 7.3 Most Innovative Proposals

| Innovation | Model | Impact |
|-----------|-------|--------|
| Signatures-first progressive disclosure | Claude Opus | ~40% token reduction for large files |
| `<![CDATA[]]>` for code content | Gemini Deep Think | Eliminates markdown-in-markdown inception |
| Per-file `[category:N]` tags | Claude Opus | Machine-parseable relevance metadata |
| Commit heatmap (🔥) | Kimi K2.5 | Visual frequency signal |
| Reliability column | GLM-5 run 2 | Inferred trust score per module |
| Centrality scores | Kimi K2.5 | Quantitative module importance |
| Tests-before-source | ChatGPT 5.2 | Deductive comprehension flow |

---

## 8. Methodology Analysis: Agentic vs One-Shot

### 8.1 Comparison

| Aspect | 🤖 Agentic (Opus, MiniMax) | 🧠 One-Shot (8 others) |
|--------|---------------------------|------------------------|
| Bug verification | Can search/grep for exact lines | Must infer from context window |
| Line citations | Exact (tool-verified) | Approximate or absent |
| Missing context | Can request more files | Must work with what's given |
| False positives | Lower (can verify claims) | Higher (inferring from memory) |
| Cost per review | Higher (many API calls) | Lower (single inference) |
| Latency | Minutes (multi-step) | Seconds to minutes |

### 8.2 Conclusion

For code review of context-builder's pre-packaged context files, **one-shot reasoning was surprisingly competitive with agentic sessions**. The key differentiator was not *finding* bugs (Deep Think matched Opus in depth with 2 unique architectural bugs each) but *citing* them precisely (Opus could grep for exact line numbers).

This has implications for context-builder itself: **if the context file is well-structured with good relevance ordering, one-shot models can perform at near-agentic quality** — which validates the tool's core value proposition.

---

## 9. Model Personality Profiles

| Model | Method | Style | Strength | Weakness |
|-------|--------|-------|----------|----------|
| **Claude Opus 4.6** | 🤖 | Surgical, evidence-based | Line-level citations, exact locations | Agentic overhead; verbose |
| **Gemini Deep Think** | 🧠 | Adversarial auditor | Deepest bugs in one shot (BTreeMap, DefaultHasher) | Slightly speculative on edge cases |
| **ChatGPT 5.2** | 🧠 | Systems thinker | Most original ideas (deductive ordering) | Brief on verification details |
| **Qwen-3-Max** | 🧠 | Technically precise | Thorough on new bugs, token math | Less creative on format proposals |
| **Kimi K2.5** | 🧠 | Pragmatic engineer | Balanced analysis, implementable suggestions | Less aggressive on bug hunting |
| **MiniMax Agent** | 🤖 | Academic reviewer | Thorough, systematic structure | Conservative despite tool access |
| **GLM-5** (run 2) | 🧠 | Context-limited analyst | Unique binary cache bug | Missing core source files |
| **Gemini Pro** | 🧠 | Balanced reviewer | Good verification of known fixes | Fewer unique insights |
| **Grok 4.1** | 🧠 | Conservative reviewer | Safe, accurate | Fewest bugs, least novel proposals |
| **GLM-5** (run 1) | 🧠 | Truncation victim | Accidentally exposed lockfile starvation bug | Couldn't review actual code |

---

## 10. Immediate Action Items (Prioritized)

| Priority | Fix | Models | Effort |
|----------|-----|--------|--------|
| 🔴 P0 | Implement `max_tokens` enforcement in parallel path | 4/10 | Medium |
| 🔴 P0 | Replace mtime hash with content hash | 6/10 | Small-Medium |
| 🔴 P0 | Replace `DefaultHasher` with stable hasher (blake3/xxhash) | 4/10 | Small |
| 🔴 P0 | Fix auto-diff BTreeMap ordering (use relevance-sorted Vec) | 1/10 | Small |
| 🟡 P1 | Elevate README/AGENTS.md to category 0 | 7/10 | Small |
| 🟡 P1 | Elevate entry points (main.rs, lib.rs) within source | 9/10 | Small |
| 🟡 P1 | Account for header/tree tokens in budget | 2/10 | Small |
| ✅ Done | Fix lockfile starvation (Cargo.lock → category 4) | 10/10 | Done |
| ✅ Done | Fix test substring false positives (path boundaries) | 3/10 | Done |

---

## Appendix A: Raw Response Files

| Model | Response File |
|-------|--------------|
| Gemini 3 Deep Think | [`context_v2_resp-gemini-3-deepthink.md`](../context_v2_resp-gemini-3-deepthink.md) |
| Gemini 3 Pro | [`context_v2_resp-gemini-3-pro.md`](../context_v2_resp-gemini-3-pro.md) |
| Grok 4.1 | [`context_v2_resp-grok-4.1.md`](../context_v2_resp-grok-4.1.md) |
| Qwen-3-Max | [`context_v2_resp-qwen-3-max.md`](../context_v2_resp-qwen-3-max.md) |
| GLM-5 (run 1) | [`context_v2_resp-glm5.md`](../context_v2_resp-glm5.md) |
| GLM-5 (run 2) | [`context_v2_resp-glm5-run2.md`](../context_v2_resp-glm5-run2.md) |
| ChatGPT 5.2 | [`context_v2_resp-chat-gpt-5.2.md`](../context_v2_resp-chat-gpt-5.2.md) |
| Kimi K2.5 | [`context_v2_resp-kimi-k2.5.md`](../context_v2_resp-kimi-k2.5.md) |
| Claude Opus 4.6 | [`context_v2_resp-claude-opus-4.6.md`](../context_v2_resp-claude-opus-4.6.md) |
| MiniMax Agent 2.5 | [`context_v2_resp-minimax-agent.md`](../context_v2_resp-minimax-agent.md) |

## Appendix B: Context File Used

- **Pre-fix context**: [`deepthink_context_v2.md`](../deepthink_context_v2.md) (original, Cargo.lock at position 0)
- **Post-fix context**: [`deepthink_context_v2_fixed.md`](../deepthink_context_v2_fixed.md) (lockfile fix applied, Cargo.lock at position last)
- **Prompt template**: [`deep_think_prompt_v2.md`](../deep_think_prompt_v2.md)
```

### File: `scripts/generate_samples.rs`

- Size: 16036 bytes
- Modified: 2026-02-14 07:14:48 UTC

```rust
#![allow(
    clippy::needless_return,
    clippy::extra_unused_lifetimes,
    clippy::doc_overindented_list_items,
    dead_code
)]
//! Dataset generation script for creating synthetic sample directories to benchmark and test
//! the context-builder CLI locally. This is intended to generate a folder that should be ignored
//! by version control (e.g., add `/samples` to your project's .gitignore).
//!
//! Usage examples (Windows PowerShell):
//!   - rustc scripts/generate_samples.rs -O -o generate_samples.exe; .\generate_samples.exe
//!   - .\generate_samples.exe --help
//!
//! Flags:
//!   --out <DIR>             Output directory (default: ./samples)
//!   --presets <list>        Comma-separated presets to generate: tiny,small,medium (default: tiny,small)
//!   --include-large         Also generate the large preset (off by default)
//!   --only <name>           Only generate a single preset (overrides --presets)
//!   --clean                 Remove the output directory before generating
//!   --dry-run               Print the plan without writing files
//!
//! Advanced overrides (apply when using --only):
//!   --files <N>             Number of text files
//!   --binary-every <N>      Create one .bin file every N text files (0 disables)
//!   --depth <D>             Directory tree depth
//!   --width <W>             Subdirectories per level
//!   --size <BYTES>          Approx text file size in bytes
//!   --filters <CSV>         Extensions to include (default: rs,md,txt,toml)
//!   --ignores <CSV>         Directory/file names to ignore (default: target,node_modules)
//!
//! Generated structure per dataset (e.g., samples/small):
//!   - project/
//!       src/, docs/, assets/      -> nested trees with text files
//!       target/, node_modules/    -> ignored directories with noise
//!       README.md, Cargo.toml     -> top-level files
//!       (binary files are sprinkled across trees and should be ignored by the tool)
//!
//! Notes:
//! - Binary files are generated to validate that the CLI ignores them by default filters.
//! - This script uses only the Rust standard library.

use std::env;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

#[derive(Clone, Debug)]
struct DatasetSpec {
    name: String,
    text_files: usize,
    binary_every: usize,
    depth: usize,
    width: usize,
    text_file_size: usize,
    filters: Vec<String>,
    ignores: Vec<String>,
}

impl DatasetSpec {
    fn with_name(name: &str) -> Option<Self> {
        match name {
            "tiny" => Some(Self {
                name: "tiny".into(),
                text_files: 100,
                binary_every: 10,
                depth: 2,
                width: 3,
                text_file_size: 256,
                filters: default_filters(),
                ignores: default_ignores(),
            }),
            "small" => Some(Self {
                name: "small".into(),
                text_files: 1_000,
                binary_every: 20,
                depth: 3,
                width: 4,
                text_file_size: 512,
                filters: default_filters(),
                ignores: default_ignores(),
            }),
            "medium" => Some(Self {
                name: "medium".into(),
                text_files: 5_000,
                binary_every: 25,
                depth: 4,
                width: 4,
                text_file_size: 800,
                filters: default_filters(),
                ignores: default_ignores(),
            }),
            "large" => Some(Self {
                name: "large".into(),
                text_files: 20_000,
                binary_every: 50,
                depth: 5,
                width: 5,
                text_file_size: 1024,
                filters: default_filters(),
                ignores: default_ignores(),
            }),
            _ => None,
        }
    }
}

fn default_filters() -> Vec<String> {
    vec!["rs", "md", "txt", "toml"]
        .into_iter()
        .map(|s| s.to_string())
        .collect()
}

fn default_ignores() -> Vec<String> {
    vec!["target", "node_modules"]
        .into_iter()
        .map(|s| s.to_string())
        .collect()
}

#[derive(Default)]
struct Args {
    out: PathBuf,
    presets: Vec<String>,
    include_large: bool,
    only: Option<String>,
    clean: bool,
    dry_run: bool,
    // overrides for --only
    files: Option<usize>,
    binary_every: Option<usize>,
    depth: Option<usize>,
    width: Option<usize>,
    size: Option<usize>,
    filters: Option<Vec<String>>,
    ignores: Option<Vec<String>>,
}

fn parse_args() -> Args {
    let mut out = PathBuf::from("samples");
    let mut presets: Vec<String> = vec!["tiny".into(), "small".into()];
    let mut include_large = false;
    let mut only: Option<String> = None;
    let mut clean = false;
    let mut dry_run = false;

    let mut files: Option<usize> = None;
    let mut binary_every: Option<usize> = None;
    let mut depth: Option<usize> = None;
    let mut width: Option<usize> = None;
    let mut size: Option<usize> = None;
    let mut filters: Option<Vec<String>> = None;
    let mut ignores: Option<Vec<String>> = None;

    let mut it = env::args().skip(1).peekable();
    while let Some(arg) = it.next() {
        match arg.as_str() {
            "--out" => {
                out = PathBuf::from(expect_value("--out", &mut it));
            }
            "--presets" => {
                presets = parse_csv(expect_value("--presets", &mut it));
            }
            "--include-large" => include_large = true,
            "--only" => {
                only = Some(expect_value("--only", &mut it).to_lowercase());
            }
            "--clean" => clean = true,
            "--dry-run" => dry_run = true,

            // overrides (effective with --only)
            "--files" => files = parse_usize(expect_value("--files", &mut it)),
            "--binary-every" => binary_every = parse_usize(expect_value("--binary-every", &mut it)),
            "--depth" => depth = parse_usize(expect_value("--depth", &mut it)),
            "--width" => width = parse_usize(expect_value("--width", &mut it)),
            "--size" => size = parse_usize(expect_value("--size", &mut it)),
            "--filters" => filters = Some(parse_csv(expect_value("--filters", &mut it))),
            "--ignores" => ignores = Some(parse_csv(expect_value("--ignores", &mut it))),
            "--help" | "-h" => {
                print_help();
                std::process::exit(0);
            }
            other => {
                eprintln!("Unknown argument: {}", other);
                print_help();
                std::process::exit(2);
            }
        }
    }

    if include_large && !presets.iter().any(|p| p == "large") {
        presets.push("large".into());
    }

    Args {
        out,
        presets,
        include_large,
        only,
        clean,
        dry_run,
        files,
        binary_every,
        depth,
        width,
        size,
        filters,
        ignores,
    }
}

fn expect_value<'a, I>(flag: &str, it: &mut I) -> String
where
    I: Iterator<Item = String>,
{
    if let Some(v) = it.next() {
        v
    } else {
        eprintln!("{flag} requires a value");
        std::process::exit(2);
    }
}

fn parse_usize(s: String) -> Option<usize> {
    match s.parse::<usize>() {
        Ok(v) => Some(v),
        Err(_) => {
            eprintln!("Invalid number: {}", s);
            std::process::exit(2);
        }
    }
}

fn parse_csv(s: String) -> Vec<String> {
    s.split(',')
        .map(|x| x.trim().to_string())
        .filter(|x| !x.is_empty())
        .collect()
}
```
