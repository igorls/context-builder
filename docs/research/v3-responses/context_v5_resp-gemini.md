This is a massive and ambitious release. Integrating Tree-Sitter brings `context-builder` into a new tier of LLM tooling. The module scaffolding, thread-safe parser initialization, and feature gating are structurally excellent.

However, bolting a syntax-aware engine onto an existing text-processing pipeline has exposed several critical **integration seams**. Furthermore, the decision to manually traverse ASTs and reconstruct strings rather than using byte-slicing has resulted in brittle extraction logic that actually *degrades* the context LLMs rely on.

Here is the deep-dive review of the v0.8.0 implementation.

---

### Part 1: Tree-Sitter Implementation Review

**1. Architecture & "Graceful" Degradation (P0 Bug)**
The isolation of language logic behind the `LanguageSupport` trait is clean, but **graceful degradation is fatally broken**.
If a user compiles the tool without `tree-sitter-base` but passes `--signatures`, `lib.rs` prints a warning and proceeds. In `markdown.rs`, `signatures_only` evaluates to `true`, causing the standard `write_text_content()` to be skipped. Because the tree-sitter feature is disabled, the fallback enrichment block does nothing.
**Result:** Every single source file is printed to the markdown output with a header, but **zero bytes of body content**.

**2. Language Definitions: The Manual String Concatenation Anti-Pattern**
Instead of leveraging the original source text, the code manually crawls children looking for specific `node.kind()` strings and glues them together (`format!("pub fn {}()", name)`). This is highly brittle and causes severe accuracy drops:

* **C/C++ Drops Parameters:** In `c.rs` and `cpp.rs`, you search for the function name but never extract the `parameter_list`. Every function signature simply appends a hardcoded `()`. A function `int foo(char* b)` becomes `int foo()`, destroying vital LLM context.
* **Rust Drops Return Types & Generics:** In `rust.rs`, you look for a child with kind `"type"`. In `tree-sitter-rust`, the node is called `"return_type"`, so this always returns `None`. Additionally, `"type_parameters"` (generics) are ignored. `pub fn foo<T>() -> i32` becomes `pub fn foo()`.
* **Python Misclassifies Methods:** You determine `is_method` by checking if the parent is `"class_definition"`. In Python's AST, a method's direct parent is a `"block"`. Therefore, `is_method` is always `false` and all methods are classified as standalone functions.
* **JS/TS Anonymous Functions Dropped:** Arrow functions assigned to variables (`const add = (a) => a;`) are extracted via `variable_declarator` and blindly emitted as `const add`, dropping the parameters and function identity entirely.

**3. Signature Extraction Formatting**
In `signatures.rs`, the logic prints category headers (e.g., `// Functions`, `// Structs/Classes`). However, `signatures` are sorted by `line_number` to preserve file order. If a file alternates between functions and structs, the output will repetitively spam headers before *every single item*.

**4. Smart Truncation: A Phantom Feature**
The release notes advertise `--truncate smart` as "AST-aware truncation." **This is entirely dead code.**
The AST boundary-finding logic in `truncation.rs` is beautiful, but it is *never called* anywhere in the pipeline. `markdown.rs` explicitly comments: `// Without a per-file max_tokens budget, no truncation is applied.` Meanwhile, the global budget tracker in `lib.rs` falls back to blind byte-slicing (`final_doc.truncate(truncate_at)`), which will cut a JSON block or markdown fence in half, polluting the LLM's context.

---

### Part 2: Remaining Bug Hunt

Four rounds of review missed these major integration blind spots:

**1. Auto-Diff Silently Ignores Tree-Sitter (P0)**
If `auto_diff = true` (which is standard since `init_config` defaults to it), `lib.rs` routes execution to `generate_markdown_with_diff()`. This function *does not accept* `ts_config`, does not call `write_tree_sitter_enrichment`, and ignores `--signatures`. It unconditionally prints the full raw file strings. **Using `--signatures` with auto-diff does nothing.**

**2. `--signatures` Destroys Non-Code Files (P0)**
Even with tree-sitter enabled, if you pass `--signatures`, `signatures_only` evaluates to `true`, which skips writing standard file bodies. For unsupported extensions (`README.md`, `Cargo.toml`, `.yaml`), `extract_signatures` returns empty. **Result:** All vital project configuration files and documentation are rendered completely blank.

**3. `--signatures` Breaks `--max-tokens` Budgeting (P1)**
In the non-parallel fallback and in `--token-count`, the token budget is estimated using raw file metadata (`std::fs::metadata().len() / 4`). If a user passes `--signatures`, the actual written output is tiny. However, the budget tracker subtracts the *entire raw file size* from the budget. This will falsely trigger the budget limit and prematurely drop the rest of the codebase.

**4. Stale Caches for New Features (P1)**
`ProjectState::compute_config_hash` and `CacheManager::hash_config` were not updated to include the new fields. If a user runs with `--signatures` and then runs without it, the hash is identical, the cache hits, and it outputs stale diffs.

**5. Rayon Worker Threads Waste CPU/Memory (P2)**
In the parallel processing path, if the `max_tokens` budget is exceeded, the writer thread logs a truncation notice and skips writing. However, **the Rayon worker threads are never stopped**. They will continue parsing ASTs, reading disk, and formatting strings for the remaining thousands of files in a monorepo, only to send them to a channel that discards them.

---

### Part 3: Architecture Health Assessment

1. **`lib.rs` Monolith:** At 1,500 lines, `lib.rs` has become a bottleneck. It orchestrates CLI parsing, config merging, cache coordination, *and* manual markdown generation (`generate_markdown_with_diff`). The fact that standard markdown and auto-diff markdown are two completely divergent code paths is the exact reason the Tree-Sitter features were bypassed in diff mode.
2. **Feature flag hygiene:** Good inside `src/tree_sitter/`, but the fallback stubs in `tree_sitter/mod.rs` use weird dummy types (`_visibility_filter: ()`) and are actually dead code, since `markdown.rs` uses `#[cfg]` macros to skip calling them entirely.
3. **Error handling:** Excellent. Tree-sitter's AST parsing is highly fault-tolerant. Syntax errors produce `ERROR` nodes, which your manual traversals naturally ignore.
4. **Testing:** The testing gap is the root cause of this release's issues. There are zero integration tests in `cli_integration.rs` or `test_auto_diff.rs` that use `--signatures`, `--structure`, or `--truncate smart` alongside filters, auto-diff, or non-code files.

---

### Part 4: Next Priorities

Ranked by (Value × Feasibility) to immediately improve LLM codebase comprehension:

#### 1. Fix `--signatures` Wiping Non-Code Files (P0)

* **Problem:** I lose `Cargo.toml`, `package.json`, and `README.md` entirely when `--signatures` is active, which are the most important files for understanding project architecture.
* **Design:** In `markdown.rs`, strictly evaluate the skip condition: `let signatures_only = ts_config.signatures && crate::tree_sitter::is_supported_extension(extension) && cfg!(feature = "tree-sitter-base");`. If a language isn't supported or the feature isn't compiled, fallback to the full text content.
* **Complexity:** Small

#### 2. Native AST Slicing for Signatures (P0)

* **Problem:** Dropping generics, return types, JS arrow functions, and C parameters causes LLMs to hallucinate incorrect types.
* **Design:** Delete the manual string concatenation in the `languages/*.rs` files. Instead, find the function/class `body` node (the `{}` block), and slice the original string natively: `&source[node.start_byte()..body.start_byte()]`. This perfectly preserves all modifiers, generics, and formatting instantly.
* **Complexity:** Medium

#### 3. Wire up Tree-Sitter to Auto-Diff (P1)

* **Problem:** If a repository uses auto-diff caching, users are entirely blocked from using the new v0.8.0 signature features.
* **Design:** Update `FileState::from_path` in `state.rs` to accept `ts_config`. If signatures/structure are requested, run the extraction *before* storing the content and hashing it. The diff engine will then naturally output **API-only diffs**, which is incredibly powerful for LLM context.
* **Complexity:** Medium

#### 4. Fulfill the "Smart Truncation" Promise (P1)

* **Problem:** `--truncate smart` is a documented feature but is dead code. Global max-tokens slices mid-word, breaking markdown code fences.
* **Design:** When the global budget is nearing exhaustion in `lib.rs` (if `ts_config.truncate == "smart"`), pass the remaining budget to `find_truncation_point` to retreat to the nearest safe AST boundary instead of doing a hard UTF-8 byte slice.
* **Complexity:** Medium

#### 5. Fix Token Budgeting & Cache Hashing (P2)

* **Problem:** Token budgets estimate using raw disk size, breaking `--signatures`. Caching ignores the new CLI flags.
* **Design:**
* Append `config.signatures`, `config.structure`, `config.visibility`, and `config.truncate` to the `config_str` used for `xxhash`.
* In the sequential mode of `markdown.rs`, if `signatures_only` is true, calculate tokens based on the extracted signature string length, rather than falling back to `fs::metadata`.


* **Complexity:** Small