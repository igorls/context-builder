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