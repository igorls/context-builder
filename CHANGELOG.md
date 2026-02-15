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
