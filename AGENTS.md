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
