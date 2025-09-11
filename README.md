# Context Builder

A blazing-fast CLI for creating LLM context from your entire codebase.

[![Crates.io](https://img.shields.io/crates/v/context-builder.svg)](https://crates.io/crates/context-builder)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/igorls/context-builder/blob/main/LICENSE)
[![CI](https://github.com/igorls/context-builder/actions/workflows/ci.yml/badge.svg)](https://github.com/igorls/context-builder/actions/workflows/ci.yml)

Tired of manually copy-pasting files into your LLM prompts? Context Builder automates this tedious process, creating a single, clean, and context-rich markdown file from any directory.

---

## Why Context Builder?

Providing broad context to Large Language Models (LLMs) is key to getting high-quality, relevant responses. This tool was built to solve one problem exceptionally well: **packaging your project's source code into a clean, LLM-friendly format with zero fuss.**

It's a command-line utility that recursively processes directories and creates comprehensive markdown documentation, optimized for AI conversations.

## Core Features

- ⚡ **Blazing Fast & Parallel by Default:** Processes thousands of files in seconds. It uses `rayon` to leverage all available CPU cores for maximum throughput.
- 🧠 **Smart & Efficient File Discovery:** Built on the excellent `ignore` crate, it respects `.gitignore` rules and custom ignore patterns out-of-the-box using optimized, parallel directory traversal.
- 💾 **Memory-Efficient Streaming:** Handles massive files with ease by reading and writing line-by-line, keeping memory usage low no matter the project size.
- 🛡️ **Robust & Safe by Design:** Confirms before overwriting files and gracefully skips binary content. Built with Rust's guarantees of memory safety, it's a tool you can trust.
- 🌳 **Clear File Tree Visualization:** Generates an easy-to-read directory structure at the top of the output file so you can see the project layout at a glance.
- 🔍 **Powerful Filtering & Preview:** Easily include only the file extensions you need and use the instant `--preview` mode to see what will be processed before generating the full output.
- 📝 **Optional Line-Numbered Code Blocks:** Add line numbers to all code blocks for easy reference with a simple `--line-numbers` flag.

---

## Installation

### From crates.io (Recommended)

```bash
cargo install context-builder
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

# Preview mode (shows the file tree without generating output)
context-builder --preview

# Add line numbers to all code blocks
context-builder --line-numbers

# Combine multiple options for a powerful workflow
context-builder -d ./src -f rs -f toml -i tests --line-numbers -o rust_context.md
```

### Command Line Options

- `-d, --input <PATH>` - Directory path to process (default: current directory).
- `-o, --output <FILE>` - Output file path (default: `output.md`).
- `-f, --filter <EXT>` - File extensions to include (can be used multiple times).
- `-i, --ignore <NAME>` - Folder or file names to ignore (can be used multiple times).
- `--preview` - Preview mode: only show the file tree, don't generate output.
- `--line-numbers` - Add line numbers to code blocks in the output.
- `-h, --help` - Show help information.
- `-V, --version` - Show version information.

---

## Documentation

- **[DEVELOPMENT.md](DEVELOPMENT.md):** For contributors. Covers setup, testing, linting, and release process.
- **[BENCHMARKS.md](BENCHMARKS.md):** For performance enthusiasts. Details on running benchmarks and generating datasets.
- **[CHANGELOG.md](CHANGELOG.md):** A complete history of releases and changes.

## Contributing

Contributions are welcome! Please see **[DEVELOPMENT.md](DEVELOPMENT.md)** for setup instructions and guidelines. For major changes, please open an issue first to discuss what you would like to change.

## License

This project is licensed under the MIT License. See the **[LICENSE](LICENSE)** file for details.