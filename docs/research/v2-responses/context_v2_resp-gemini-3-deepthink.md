Here is a deep-dive architectural analysis of `context-builder` v0.7.0.

The transition from naive alphabetical concatenation to intelligent, relevance-ordered context aggregation shows a deep understanding of how Large Language Models (LLMs) consume prompt windows. However, diving into the execution of v0.7.0 reveals that while the *ideas* are excellent, there are several critical architectural blind spots—including one that completely neutralizes the `--max-tokens` feature, and another that actively sabotages the LLM prompt caching it was built to enable.

---

### Part 1: Relevance Ordering Impact Assessment

Reading the codebase in relevance order fundamentally improves the ingestion experience, but it currently has some sharp edges.

1. **Comprehension Impact (High but flawed)**:
* **The Good**: Seeing `Cargo.toml` before `src/` is fantastic. It immediately loaded my weights with your dependencies (`tiktoken-rs`, `ignore`, `rayon`), which primed my mental model before I read a single line of Rust.
* **The Bad**: Hitting `Cargo.lock` (39KB of noise) as the *very first file* is brutal. Lockfiles consume massive token budgets and offer near-zero architectural signal.


2. **Category Boundaries (Config → Source → Tests → Docs)**: This is **sub-optimal because Docs are at the bottom**.
* An LLM benefits immensely from reading `README.md` and `AGENTS.md` *before* the source code. These files provide the "system prompt" and architectural intent. Reading docs last forces the LLM to deduce the architecture blindly from code, only to have its deductions confirmed/denied at the very end of the context window.
* *Ideal Order*: `Core Docs (README)` → `Config` → `Source` → `Tests` → `Other Docs`.


3. **Within-Category Alphabetical**: Alphabetical sorting within `src/` is an arbitrary bottleneck. It forces the LLM to read `cache.rs` and `cli.rs` before it reads `lib.rs` and `main.rs`.
* *Ideal Intra-Category*: **Topological / Dependency Graph**. The entry points (`main.rs`, `lib.rs`) should appear first, followed by the modules they declare.


4. **Missing Categories**:
* **Lockfiles**: Must be separated from semantic configs and relegated to the absolute lowest priority (or excluded entirely by default).
* **Build/CI**: `.github/workflows`, `Makefile`, `build.rs` belong in an "Infrastructure" category distinct from business logic.



---

### Part 2: Architecture & Code Review (Delta from v0.6.0)

I reviewed the three new features and verified your previous bug fixes. I found **four critical new bugs**.

#### 🚨 BUG 1: Token Budgeting is a No-Op by Default

In `markdown.rs` (line 135), the `--max-tokens` logic was implemented *exclusively* inside the `#[cfg(not(feature = "parallel"))]` block.
Because the `parallel` feature is **enabled by default** in `Cargo.toml`, **the `--max-tokens` flag does absolutely nothing for 99% of your users.** The Rayon parallel iterator processes all files without any atomic token tracking or short-circuiting. Furthermore, the `auto_diff` path in `lib.rs` completely ignores `max_tokens` as well.

#### 🚨 BUG 2: Relevance Sorting is Destroyed by Auto-Diff

In `file_utils.rs`, you painstakingly sort files by relevance. But look at what happens in `lib.rs` (line 439) when rendering the full file contents in `auto_diff` mode:

```rust
for (path, file_state) in &current_state.files { ... }

```

`current_state.files` is a `BTreeMap<PathBuf, FileState>`. Iterating over a `BTreeMap` is inherently **alphabetical**. If a user uses `auto_diff` without `diff_only`, your relevance sorting is completely wiped out and the output reverts to alphabetical!

#### 🚨 BUG 3: "Content Hash" Defeats LLM Prompt Caching

In `markdown.rs`, you implemented a hash to replace the volatile timestamp to allow LLMs (like Anthropic/OpenAI) to cache the prompt. But look at what you are hashing:

```rust
if let Ok(modified) = meta.modified() {
    modified.hash(&mut hasher); // <--- FATAL FLAW
}

```

By hashing the `modified` metadata, the hash reflects the file's *timestamp* (`mtime`). If a user runs `git clone`, switches branches, or runs `touch`, the timestamps change, the hash changes, and the **entire LLM prompt cache is busted**, even if the code content is 100% identical. You must hash *only* relative paths and file contents.

#### 🚨 BUG 4: `DefaultHasher` is Non-Deterministic Across Environments

The code uses `std::collections::hash_map::DefaultHasher`. The Rust documentation explicitly states: *"The internal algorithm is not specified... hashes of the same value can differ across runs and across Rust releases."* This will silently break caching across different environments. You should use a stable hash crate like `sha2` or `blake3`.

#### Bug Fix Verification (from v0.6.0)

1. **TOCTOU data loss**: ✅ FIXED. `file.lock_exclusive()?` is acquired *before* `file.set_len(0)?` in `cache.rs`.
2. **UTF-8 8KB boundary**: ✅ FIXED. The bitwise backtracking logic (`sniff[end - 1] & 0xC0 == 0x80`) is highly robust.
3. **CLI flags overwritten**: ✅ FIXED. Handled safely in `config_resolver.rs`.
4. **Double file seek**: ✅ FIXED. Removed from `markdown.rs`.
5. **Indentation destruction (`diff_only`)**: ⚠️ *Partially Fixed.* You used `line.strip_prefix('+')` in `lib.rs`. However, unified diffs output additions as `+ code` (with a space). Stripping just the `+` leaves a leading space (` code`), shifting indentation right by 1 space. This is better than `trim_start()`, but `strip_prefix("+ ").or_else(|| strip_prefix("+"))` is required to fix Python/YAML formatting.

---

### Part 3: Strategic Feature Roadmap (Tier 2)

To maximize the "quality of reasoning per token," here are the top 5 features for Tier 2:

#### 1. Code Skeletonization / AST Minification (High Impact)

* **Problem**: Large repositories blow past context windows with implementation details the LLM doesn't need (e.g., a 1,000-line utility file).
* **Design**: Add an `--outline-only` flag for specific directories. Use `tree-sitter` (or lightweight regex) to strip function bodies, retaining only `imports`, `structs`, `traits`, and `fn` signatures.
* **Complexity**: **L** (Requires language-specific parsing).
* **Risk**: Increased binary size; parsing failures on invalid syntax.

#### 2. Topological Dependency Sorting

* **Problem**: Sequential reading forces the LLM to hold unresolved symbols in its KV cache until definitions are found later.
* **Design**: Within the `Source` category, run a lightweight pass over `import` / `use` statements to build a DAG. Output files so that definitions appear before their usages (or entry points first).
* **Complexity**: **M**.
* **Risk**: Cyclic dependencies causing sort panics (needs cycle breaking).

#### 3. XML-Based Output Wrapping

* **Problem**: Markdown `code` blocks break when the source file itself contains Markdown code blocks (the "inception" bug). Furthermore, frontier models are instruction-tuned to attend strongly to XML boundaries.
* **Design**: Wrap the output in semantic XML tags (see Part 4).
* **Complexity**: **S**. Purely formatting changes.
* **Risk**: Low, though it changes the visual layout for humans.

#### 4. Smart Lockfile Summarization

* **Problem**: Passing a 40KB `Cargo.lock` to an LLM wastes tokens and attention span.
* **Design**: If a file is a known lockfile, intercept it and output a flattened, deduplicated list of dependencies and versions instead of the raw TOML/JSON.
* **Complexity**: **M**.
* **Risk**: Strips deep transitive dependency info the user might have actually wanted to debug.

#### 5. Active Git Context Injection

* **Problem**: An LLM knows *what* the code is, but not *why* it is that way or what the developer is currently doing.
* **Design**: Integrate the `git2` crate (or shell out). Inject a `<git_context>` block summarizing the last 5 commits, or inject `git diff --cached` to provide temporal grounding.
* **Complexity**: **M**.
* **Risk**: Fails gracefully if not in a git repository.

---

### Part 4: Output Format v2 Specification

Markdown is optimized for human eyes, but LLMs are syntax-parsing engines. Anthropic and OpenAI explicitly recommend **XML tags** for strict boundary separation in RAG and context-stuffing. XML prevents prompt-injection (e.g., a file containing triple-backticks) and makes attribute parsing trivial for LLM attention heads.

Furthermore, we should drop repetitive noise like `- Size: X bytes` and `- Modified: Unknown` on every single file, compressing them into attributes.

#### Ideal v2 Output Format (First ~100 lines)

```xml
<context_builder_report version="2.0">
<project_metadata>
  <name>context-builder</name>
  <generated_at_utc>2026-02-14T20:08:42</generated_at_utc>
  <stable_content_hash>a1b2c3d4e5f6</stable_content_hash>
  <filters_applied>rs, toml, md</filters_applied>
</project_metadata>

<instructions>
  The following context contains the codebase structure and contents.
  Review the <documentation> first to understand the system architecture.
</instructions>

<directory_tree>
├── README.md
├── Cargo.toml
├── src/
│   ├── lib.rs
│   └── main.rs
└── tests/
</directory_tree>

<codebase>
<documentation>
<document path="README.md" size_bytes="9822">
<![CDATA[
<div align="center">
# Context Builder
A blazing-fast CLI for creating LLM context from your entire codebase.
...
]]>
</document>
</documentation>

<configuration>
<document path="Cargo.toml" size_bytes="1409">
<![CDATA[
[package]
name = "context-builder"
version = "0.7.0"
...
]]>
</document>
</configuration>

<source_code>
<document path="src/main.rs" size_bytes="73" language="rust">
<![CDATA[
use std::io;

fn main() -> io::Result<()> {
    context_builder::run()
}
]]>
</document>

<document path="src/lib.rs" size_bytes="45261" language="rust">
<![CDATA[
use clap::{CommandFactory, Parser};
// ... code ...
]]>
</document>
</source_code>
</codebase>
</context_builder_report>

```

**Why this format maximizes reasoning:**

1. **Zero Cache Busting**: No timestamps or byte sizes in the text bodies. If the code doesn't change, the text is byte-for-byte identical. 100% LLM cache hit rate.
2. **Boundary Safety**: `<document>` and `<![CDATA[ ... ]]>` guarantees that no matter what text is inside the file, the LLM will *never* confuse it for structural instructions.
3. **Token Efficiency**: Consolidating markdown bullet points (`- Size: X`) into XML attributes (`size_bytes="X"`) saves ~10-15 tokens per file, scaling massively across large repositories.