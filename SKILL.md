---
name: context-builder
description: Generate LLM-optimized codebase context from any directory using context-builder CLI
homepage: https://github.com/igorls/context-builder
version: 0.7.1
requires:
  - cargo
  - context-builder
---

# Context Builder — Agentic Skill

Generate a single, structured markdown file from any codebase directory. The output is optimized for LLM consumption with relevance-based file ordering, automatic token budgeting, and smart defaults.

## Installation

```bash
cargo install context-builder
```

Verify: `context-builder --version`

## When to Use

- **Deep code review** — Feed an entire codebase to an LLM for architecture analysis or bug hunting
- **Onboarding** — Generate a project snapshot for understanding unfamiliar codebases
- **Diff-based updates** — After code changes, generate only the diffs to update an LLM's understanding
- **Cross-project research** — Quickly package a dependency's source for analysis

## Core Workflow

### 1. Quick Context (whole project)

```bash
context-builder -d /path/to/project -y -o context.md
```

- `-y` skips confirmation prompts (essential for non-interactive agent use)
- Output includes: header → file tree → files sorted by relevance (config → source → tests → docs)

### 2. Scoped Context (specific file types)

```bash
context-builder -d /path/to/project -f rs,toml -i docs,assets -y -o context.md
```

- `-f rs,toml` includes only Rust and TOML files
- `-i docs,assets` excludes directories by name

### 3. Budget-Constrained Context

```bash
context-builder -d /path/to/project --max-tokens 100000 -y -o context.md
```

- Caps output to ~100K tokens (estimated)
- Files are included in relevance order until budget is exhausted
- Automatically warns if output exceeds 128K tokens

### 4. Token Count Preview

```bash
context-builder -d /path/to/project --token-count
```

- Prints estimated token count without generating output
- Use this first to decide if filtering is needed

### 5. Incremental Diffs

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
| `-d <PATH>` | Input directory | Always use absolute paths for reliability |
| `-o <FILE>` | Output path | Write to a temp or docs directory |
| `-f <EXT>` | Filter by extension | Comma-separated: `-f rs,toml,md` |
| `-i <NAME>` | Ignore dirs/files | Comma-separated: `-i tests,docs,assets` |
| `--max-tokens <N>` | Token budget cap | Use `100000` for most models, `200000` for Gemini |
| `--token-count` | Dry-run token estimate | Run first to check if filtering is needed |
| `-y` | Skip all prompts | **Always use in agent workflows** |
| `--preview` | Show file tree only | Quick exploration without generating output |
| `--diff-only` | Output only diffs | Minimizes tokens for incremental updates |
| `--init` | Create config file | Auto-detects project file types |

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

# If > 128K tokens, scope it down:
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

## Error Handling

- If `context-builder` is not installed, install with `cargo install context-builder`
- If output exceeds token limits, add `--max-tokens` or narrow with `-f` / `-i`
- If the project has no `.git` directory, auto-ignores still protect against dependency flooding
- Use `--clear-cache` if diff output seems stale or incorrect
