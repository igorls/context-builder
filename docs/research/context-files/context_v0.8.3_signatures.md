# Directory Structure Report

**Project:** context-builder
**Generated:** 2026-02-16 03:27:52 UTC
**Filters:** rs, toml, sh, yml
**Ignored:** docs, target, .git, node_modules

## File Tree Structure

- 📄 Cargo.toml
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

- Size: 3727 bytes
- Modified: SystemTime { tv_sec: 1771212003, tv_nsec: 847413402 }


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

- Size: 2886 bytes
- Modified: SystemTime { tv_sec: 1771211737, tv_nsec: 871665745 }

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
trap 'rm -rf "$TMP"' EXIT
echo "Downloading ${ARCHIVE}..."
curl -sSL "${BASE_URL}/${ARCHIVE}" -o "$TMP/$ARCHIVE"
curl -sSL "${BASE_URL}/SHA256SUMS" -o "$TMP/SHA256SUMS"

# Verify SHA256 checksum (fail closed — never install unverified binaries)
echo "Verifying checksum..."
EXPECTED="$(grep "$ARCHIVE" "$TMP/SHA256SUMS" | awk '{print $1}')"
if [ -z "$EXPECTED" ]; then
  echo "Error: Could not find checksum for $ARCHIVE in SHA256SUMS"
  echo "Aborting installation. Download the binary manually from:"
  echo "  https://github.com/$REPO/releases/latest"
  exit 1
fi

if command -v sha256sum >/dev/null 2>&1; then
  ACTUAL="$(sha256sum "$TMP/$ARCHIVE" | awk '{print $1}')"
elif command -v shasum >/dev/null 2>&1; then
  ACTUAL="$(shasum -a 256 "$TMP/$ARCHIVE" | awk '{print $1}')"
else
  echo "Error: No SHA256 verification tool found (need sha256sum or shasum)"
  echo "Aborting installation. Install one of these tools or download manually:"
  echo "  https://github.com/$REPO/releases/latest"
  exit 1
fi

if [ "$ACTUAL" != "$EXPECTED" ]; then
  echo "Error: Checksum verification failed!"
  echo "  Expected: $EXPECTED"
  echo "  Got:      $ACTUAL"
  echo "The download may be corrupted or tampered with."
  exit 1
fi
echo "✓ Checksum verified"

# Extract and install
tar xzf "$TMP/$ARCHIVE" -C "$TMP"
$SUDO mv "$TMP/context-builder" "$INSTALL_DIR/context-builder"
$SUDO chmod +x "$INSTALL_DIR/context-builder"
# $TMP is cleaned up automatically by the EXIT trap

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

- Size: 19719 bytes
- Modified: SystemTime { tv_sec: 1771212001, tv_nsec: 733383614 }


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

- Size: 28089 bytes
- Modified: SystemTime { tv_sec: 1771211998, tv_nsec: 21331311 }


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

- Size: 20717 bytes
- Modified: SystemTime { tv_sec: 1771211994, tv_nsec: 340279444 }


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

- Size: 13628 bytes
- Modified: SystemTime { tv_sec: 1771211951, tv_nsec: 498675797 }


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

- Size: 24729 bytes
- Modified: SystemTime { tv_sec: 1771211920, tv_nsec: 504239078 }


**Signatures:**

```rust
// Structs/Classes
pub struct TypeScriptSupport
pub struct TsxSupport

// Implementations
impl TypeScriptSupport

// Functions
fn get_language() -> tree_sitter::Language

// Implementations
impl TsxSupport

// Functions
fn get_language() -> tree_sitter::Language

// Macros
macro_rules! impl_ts_language_support

// Modules
mod tests

// Functions
fn test_extract_function_signature()
fn test_extract_arrow_function()
fn test_extract_class_signature()
fn test_extract_class_with_inheritance()
fn test_extract_interface_signature()
fn test_extract_interface_with_extends()
fn test_no_duplicate_exported_signatures()
fn test_parse_valid_typescript()
fn test_find_truncation_point()
fn test_file_extensions_ts()
fn test_file_extensions_tsx()
fn test_tsx_parses_jsx_syntax()
fn test_ts_parser_rejects_jsx_syntax()
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

