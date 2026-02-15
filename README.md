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

- 🌲 **Tree-Sitter AST Analysis** *(optional)*:
  Extract function/class signatures (`--signatures`), structural summaries (`--structure`), and smart AST-boundary truncation (`--truncate smart`). Supports Rust, JavaScript, TypeScript, Python, Go, Java, C, and C++.

- 🧪 **Accurate Token Counting:**
  Get real tokenizer–based estimates with `--token-count` to plan your prompt budgets.


---

## Installation

### From crates.io (Recommended)

```bash
cargo install context-builder
```

With tree-sitter AST support (signatures, structure analysis):
```bash
cargo install context-builder --features tree-sitter-all
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
