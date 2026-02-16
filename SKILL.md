---
name: context-builder
description: Generate LLM-optimized codebase context from any directory using context-builder CLI
homepage: https://github.com/igorls/context-builder
version: 0.8.3
requires:
  - context-builder
---

# Context Builder — Agentic Skill

Generate a single, structured markdown file from any codebase directory. The output is optimized for LLM consumption with relevance-based file ordering, AST-aware code signatures, automatic token budgeting, and smart defaults.

## Installation

```bash
# Requires Rust toolchain. Builds from source with cryptographic verification via crates.io.
cargo install context-builder --features tree-sitter-all
```

Pre-built binaries with SHA256 checksums are also available for manual download from [GitHub Releases](https://github.com/igorls/context-builder/releases/latest).

Verify: `context-builder --version` (expected: `0.8.3`)


## Security & Path Scoping

> **IMPORTANT**: This tool reads file contents from the specified directory. Agents MUST follow these rules:

- **Only target explicit project directories** — always pass the exact project root (e.g., `/home/user/projects/myapp`). Never point at home directories, system paths, or credential stores (`~/.ssh`, `~/.aws`, `/etc`, `~`, `/`)
- **Use scoped filters** — use `-f` to limit to known source extensions (e.g., `-f rs,toml,md`), reducing exposure surface
- **Output to project-local paths** — write output to the project's `docs/` folder or `/tmp/`, never to shared or public locations
- **Review before sharing** — the output may contain API keys, secrets, or credentials embedded in source files; always review or use `.gitignore` patterns to exclude sensitive files

**Built-in protections** (always active, no configuration needed):
- Excludes `.git/`, `node_modules/`, and 19 other heavy/sensitive directories at any depth
- Respects `.gitignore` rules when a `.git` directory is present
- Binary files are auto-detected and skipped via UTF-8 sniffing
- Output file and cache directory are auto-excluded to prevent self-ingestion

## When to Use

- **Deep code review** — Feed an entire codebase to an LLM for architecture analysis or bug hunting
- **Onboarding** — Generate a project snapshot for understanding unfamiliar codebases
- **Diff-based updates** — After code changes, generate only the diffs to update an LLM's understanding
- **AST signatures** — Extract function/class signatures for token-efficient structural understanding
- **Cross-project research** — Quickly package a dependency's source for analysis

## Core Workflow

### 1. Quick Context (whole project)

```bash
context-builder -d /path/to/project -y -o context.md
```

- `-y` skips confirmation prompts (recommended for agent workflows when path is explicitly scoped)
- Output includes: header → file tree → files sorted by relevance (config → source → tests → docs)

### 2. Scoped Context (specific file types)

```bash
context-builder -d /path/to/project -f rs,toml -i docs,assets -y -o context.md
```

- `-f rs,toml` includes only Rust and TOML files
- `-i docs,assets` excludes directories by name

### 3. AST Signatures Mode (minimal tokens)

```bash
context-builder -d /path/to/project --signatures -f rs,ts,py -y -o signatures.md
```

- Replaces full file content with extracted function/class signatures (~4K vs ~15K tokens per file)
- Supports 8 languages: Rust, JavaScript (.js/.jsx), TypeScript (.ts/.tsx), Python, Go, Java, C, C++
- Requires `--features tree-sitter-all` at install time

### 4. Signatures with Structural Summary

```bash
context-builder -d /path/to/project --signatures --structure -y -o context.md
```

- `--structure` appends a count summary (e.g., "6 functions, 2 structs, 1 impl block")
- Combine with `--visibility public` to show only public API surface

### 5. Budget-Constrained Context

```bash
context-builder -d /path/to/project --max-tokens 100000 -y -o context.md
```

- Caps output to ~100K tokens (estimated)
- Files are included in relevance order until budget is exhausted
- Automatically warns if output exceeds 128K tokens

### 6. Token Count Preview

```bash
context-builder -d /path/to/project --token-count
```

- Prints estimated token count without generating output
- Use this first to decide if filtering or `--signatures` is needed

### 7. Incremental Diffs

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
| `-d <PATH>` | Input directory | **Always use absolute paths** for reliability |
| `-o <FILE>` | Output path | Write to project `docs/` or `/tmp/` |
| `-f <EXT>` | Filter by extension | Comma-separated: `-f rs,toml,md` |
| `-i <NAME>` | Ignore dirs/files | Comma-separated: `-i tests,docs,assets` |
| `--max-tokens <N>` | Token budget cap | Use `100000` for most models, `200000` for Gemini |
| `--token-count` | Dry-run token estimate | Run first to check if filtering is needed |
| `-y` | Skip all prompts | **Use only with explicit, scoped project paths** |
| `--preview` | Show file tree only | Quick exploration without generating output |
| `--diff-only` | Output only diffs | Minimizes tokens for incremental updates |
| `--signatures` | AST signature extraction | Requires `tree-sitter-all` feature at install |
| `--structure` | Structural summary | Pair with `--signatures` for compact output |
| `--visibility <V>` | Filter by visibility | `all` (default), `public` (public API only) |
| `--truncate <MODE>` | Truncation strategy | `smart` (AST-aware) or `simple` |
| `--init` | Create config file | Auto-detects project file types |
| `--clear-cache` | Reset diff cache | Use if diff output seems stale |

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

### Recipe: API Surface Review (signatures only)

```bash
# Extract only public signatures — typically 80-90% fewer tokens than full source
context-builder -d /path/to/project --signatures --visibility public -f rs -y -o docs/api_surface.md
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

# If > 128K tokens, try signatures mode first:
context-builder -d /path/to/project --signatures --token-count

# Or scope it down:
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
signatures = true
structure = true
visibility = "public"
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

When `--signatures` is active, file contents are replaced with extracted signatures:

    ### File: src/lib.rs
    ```rust
    pub fn run_with_args(args: Args, config: Config, prompter: &dyn Prompter) -> Result<()>
    pub fn generate_markdown_with_diff(...) -> Result<String>
    ```

## Error Handling

- If `context-builder` is not installed, install with `cargo install context-builder --features tree-sitter-all`
- If `--signatures` shows no output for a file, the language may not be supported or the feature was not enabled at install
- If output exceeds token limits, add `--max-tokens` or narrow with `-f` / `-i`, or use `--signatures`
- If the project has no `.git` directory, auto-ignores still protect against dependency flooding
- Use `--clear-cache` if diff output seems stale or incorrect
