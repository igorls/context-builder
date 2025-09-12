# Changelog

All notable changes to this project will be documented in this file.

## v0.4.0


- Added

  - Token count mode (`--token-count`) now provides accurate token counts using the `tiktoken-rs` library.

  - Configuration file support (`.context-builder.toml`) for project-specific settings.

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