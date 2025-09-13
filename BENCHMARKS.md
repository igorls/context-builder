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