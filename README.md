# Context Builder

A CLI tool to aggregate directory contents into a single markdown file optimized for LLM consumption.

## Overview

Context Builder is a powerful command-line utility that recursively processes directories and creates comprehensive markdown documentation. It's specifically designed to generate context files that are optimized for Large Language Models (LLMs), making it easier to provide complete project context in AI conversations.

## Features

- 📁 **Recursive directory processing** - Scans entire directory trees
- 🔍 **Smart filtering** - Include only specific file extensions
- 🚫 **Flexible ignoring** - Exclude folders and files by name
- 🌳 **File tree visualization** - Generates clear directory structure
- 👀 **Preview mode** - See what will be processed before generating
- 📝 **Line numbers** - Optional line numbering for code blocks
- ⚡ **Fast processing** - Efficient file handling with detailed timing
- 🛡️ **Safe operations** - Confirms before overwriting existing files

## Installation

### From crates.io

```bash
cargo install context-builder
```

### From source

```bash
git clone https://github.com/yourusername/context-builder.git
cd context-builder
cargo install --path .
```

## Usage

### Benchmarks (quick)
- Run:
  - Linux/macOS: `cargo bench --bench context_bench`
  - Windows PowerShell: `cargo bench --bench context_bench`
- Include medium dataset (heavier):
  - Linux/macOS: `CB_BENCH_MEDIUM=1 cargo bench --bench context_bench`
  - Windows PowerShell: `$env:CB_BENCH_MEDIUM=1; cargo bench --bench context_bench`
- HTML report:
  - `target/criterion/report/index.html`

### Basic Usage

```bash
# Process current directory and create output.md
context-builder

# Process specific directory
context-builder -d /path/to/project

# Specify output file
context-builder -d /path/to/project -o documentation.md
```

### Advanced Options

```bash
# Filter by file extensions
context-builder -f rs -f toml -f md

# Ignore specific folders/files
context-builder -i target -i node_modules -i .git

# Preview mode (show file tree without generating output)
context-builder --preview

# Add line numbers to code blocks
context-builder --line-numbers

# Combine multiple options
context-builder -d ./src -f rs -f toml -i tests --line-numbers -o rust_context.md
```

### Command Line Options

- `-d, --input <PATH>` - Directory path to process (default: current directory)
- `-o, --output <FILE>` - Output file path (default: output.md)
- `-f, --filter <EXT>` - File extensions to include (can be used multiple times)
- `-i, --ignore <NAME>` - Folder or file names to ignore (can be used multiple times)
- `--preview` - Preview mode: only show file tree, don't generate output
- `--line-numbers` - Add line numbers to code blocks in the output
- `-h, --help` - Show help information
- `-V, --version` - Show version information

## Output Format

Context Builder generates markdown files with the following structure:

1. **Header** - Project name and generation timestamp
2. **Processing Summary** - File counts and filtering information
3. **File Tree** - Visual directory structure
4. **File Contents** - Each file's content in fenced code blocks

## Use Cases

- **LLM Context Generation** - Create comprehensive project context for AI assistants
- **Code Reviews** - Generate complete snapshots of codebases
- **Documentation** - Create unified documentation from multiple files
- **Project Analysis** - Get overview of project structure and contents
- **Backup/Archival** - Create readable snapshots of project states

## Examples

### Rust Project

```bash
context-builder -d ./my-rust-project -f rs -f toml -i target -o rust_context.md
```

### JavaScript Project

```bash
context-builder -d ./my-js-project -f js -f ts -f json -i node_modules -i dist
```

### Documentation Project

```bash
context-builder -d ./docs -f md -f txt --line-numbers
```

## Performance

Context Builder is designed for efficiency:

- Processes thousands of files in seconds
- Memory-efficient streaming for large files
- Parallel directory traversal
- Smart filtering to avoid unnecessary processing

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.


### Development Setup



```bash

git clone https://github.com/yourusername/context-builder.git

cd context-builder

cargo build

cargo test

```

## Benchmarks

Run Criterion benchmarks to evaluate performance at different scales.

- Quick run:
  - Linux/macOS:
    ```bash
    cargo bench --bench context_bench
    ```
  - Windows PowerShell:
    ```powershell
    cargo bench --bench context_bench
    ```

- Include the medium dataset (disabled by default to keep runs lighter):
  - Linux/macOS:
    ```bash
    CB_BENCH_MEDIUM=1 cargo bench --bench context_bench
    ```
  - Windows PowerShell:
    ```powershell
    $env:CB_BENCH_MEDIUM=1; cargo bench --bench context_bench
    ```

- HTML reports:
  - Open the Criterion report at:
    - target/criterion/report/index.html
    - or per-benchmark reports under: target/criterion/context_builder/*/report/index.html

Notes:
- Benchmarks run both with and without line numbers to compare overhead.
- Datasets are generated in a temporary directory during the run.
- Binary files are generated but ignored by the default filters.
- Ignores: node_modules, target; Filters: rs, md, txt, toml.

## Generating sample datasets

You can generate persistent sample datasets (ignored from version control) for local testing and manual performance checks.

- Add this to your .gitignore if not already present:
  ```
  /samples
  ```

- Compile and run the generator script:
  - Linux/macOS:
    ```bash
    rustc scripts/generate_samples.rs -O -o generate_samples && ./generate_samples --help
    ```
  - Windows PowerShell:
    ```powershell
    rustc scripts/generate_samples.rs -O -o generate_samples.exe; .\generate_samples.exe --help
    ```

Common usages:
- Default presets (tiny, small) into ./samples:
  - `generate_samples`
- Include medium and large:
  - `generate_samples --presets tiny,small,medium --include-large`
- Only one preset with custom parameters:
  - `generate_samples --only small --files 5000 --depth 4 --width 4 --size 1024`
- Clean output before generating:
  - `generate_samples --clean`
- Dry run (print plan only):
  - `generate_samples --dry-run`

Generated structure per dataset:
- project/
  - src/, docs/, assets/ nested trees with text files
  - target/, node_modules/ with ignored noise
  - README.md, Cargo.toml at root
  - .bin files sprinkled across trees to validate binary handling

Binary files are intentionally ignored for now. In the future, specialized parsers can be added to support additional document types (e.g., PDF, DOC).


## Changelog

### v0.2.0
- Added line numbers support
- Improved file tree visualization
- Enhanced error handling
- Better CLI argument validation

### v0.1.0
- Initial release
- Basic directory processing
- File filtering and ignoring
- Markdown output generation

## License

MIT License. See `LICENSE` file for details.
