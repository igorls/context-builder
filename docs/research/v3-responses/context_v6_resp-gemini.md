This is a massive step forward for the codebase. Moving away from brittle string concatenation to AST-driven byte slicing solves the hardest problem of signature extraction: preserving complex language semantics like generics, lifetimes, and modifiers.

However, as is typical when wiring up Tree-Sitter across 8 distinct parsers, the strict nesting rules of the AST have exposed a few logic gaps. Furthermore, some of the new Phase 4 flags were disconnected from the older core mechanics.

Here is the deep-dive analysis of `context-builder` v0.8.0.

---

### Part 1: Verify the Byte-Slicing Refactor

The core slicing mechanic (`&source[node.start_byte()..child.start_byte()].trim_end()`) is mathematically sound. It elegantly handles inline comments, trailing whitespace, and gracefully skips leading doc-comments. However, because it relies on the "body" node being a **direct child** of the passed `node`, it breaks in several languages:

**1. Python: The Decorator Erasure & Fallback Irony**

* **Decorators are dropped:** In Tree-Sitter Python, decorated functions are wrapped in a `decorated_definition` node. Because your code explicitly matches `"function_definition"`, `node.start_byte()` begins strictly at the `def` keyword, bypassing the decorator sibling entirely.
* **The Irony:** You wrote a `find_decorators` helper inside the `unwrap_or_else` fallback block. But because a `function_definition` *always* has a `"block"` child, the slicing succeeds and your fallback is **never executed**.
* **The `is_method` bug:** Your grandparent check looks 2 levels up (`block` -> `class_definition`). But for decorated methods, the parent chain is 3 levels deep (`decorated_definition` -> `block` -> `class_definition`). Thus, decorated methods are misclassified as global functions.

**2. JavaScript / TypeScript: The Arrow Function Body Leak (Critical)**

* **The Bug:** Arrow functions bypass signature stripping, dumping their *entire implementation body* into the output and destroying the token budget.
* **Why:** You pass the `variable_declarator` node to `slice_signature` and tell it to look for a `"statement_block"`. However, `"statement_block"` is a child of the `"arrow_function"`, which is a grandchild of the declarator. Because it isn't a direct child, slicing returns `None`.
* **The Leak:** The fallback triggers: `&source[child.start_byte()..child.end_byte()]`. This blindly captures the entire variable declaration, leaking the full function implementation. Furthermore, because it starts at the declarator, it strips `const` and `export` from the parent `lexical_declaration`.

**3. C / C++: The Header File Blindspot**

* **The Bug:** Header files (`.h`, `.hpp`) appear effectively empty, and C++ classes lose their inheritance.
* **Why:** You only match `"function_definition"` nodes (functions with bodies). Header files contain prototypes, which are parsed as `"declaration"` nodes. Because `"declaration"` is ignored, headers are skipped. For classes, you bypass byte-slicing entirely (`format!("class {}", name)`), permanently dropping `template <typename T>` and `public BaseClass`.

**4. Rust: The Tuple Struct Erasure**

* **The Bug:** Tuple structs (`struct Color(u8, u8);`) lose their fields.
* **Why:** You slice looking for a `"field_declaration_list"`. Tuple structs use an `"ordered_field_declaration_list"`. The slice fails, falling back to `pub struct Color`.

---

### Part 2: Tree-Sitter Integration Health

**1. Feature Gate Consistency**
**Flawless.** The conditional compilation (`#[cfg(feature = "tree-sitter-base")]`) is executed perfectly. The fallback stubs ensure the CLI compiles cleanly when features are omitted, and the runtime degradation warning in `lib.rs` provides excellent UX.

**2. The `is_supported_extension()` Function**
**Missing `.jsx`:** In `tree_sitter/languages/mod.rs`, `JS_SUPPORT` covers `"js" | "mjs" | "cjs"`, but misses `"jsx"`. React files will fail `is_supported_extension`, bypassing the signature extractor and dumping full React source code bodies into the context.

**3. Signature vs Structure Interaction**
**Excellent synergy.** Emitting a high-level `Structure` count (e.g., `5 functions, 2 classes`) followed immediately by the `Signatures` provides me (the LLM) with a perfect, highly-compressible semantic map.

**4. Smart Truncation: The Phantom Feature**
**Dead Code Bug.** As the comment in `markdown.rs:546` admits, the flag `--truncate smart` is parsed, and Tree-Sitter implements `find_truncation_point`, but **it is never actually called.**
If `max_tokens` is hit, `lib.rs:464` executes a brute-force UTF-8 byte truncation on the `final_doc` string, which will happily slice an AST block or JSON payload in half, causing hallucinated completions.

---

### Part 3: Remaining Bugs (Missed in all rounds)

**1. The Auto-Diff + Signatures Collision (Logic Defeat)**
If a user runs `context-builder --auto-diff --signatures`, they are trying to save token space.

* However, `state.rs` caches the **full raw text** of the files.
* `diff.rs` performs a unified diff on the **full raw text**.
* The Markdown file will contain the stripped signatures in the content section, but the `## File Differences` section will output massive unified diffs showing changes to the *full function bodies*. This completely defeats the token-saving purpose of `--signatures`.

**2. Cache Hash Desynchronization (Cache Thrashing)**
In Round 3, `compute_config_hash` was added to `state.rs` to include the new TS flags (`signatures`, `structure`, `truncate`, `visibility`).

* **However, you forgot to update `hash_config()` in `cache.rs`.**
* Toggling `--signatures` generates the *same* physical cache filename (`state_X_Y.json`). The diff state correctly invalidates internally, but it continuously overwrites the same physical file on disk, causing cache thrashing if users swap between config profiles.

**3. Missing JS/TS Export Signatures**
In `javascript.rs` and `typescript.rs`, `extract_export_signatures` only looks for `function_declaration`, `class_declaration`, and `interface_declaration`. It ignores `lexical_declaration`. Thus, **any exported arrow function (`export const myFunc = () => {}`) is completely omitted from the signature output.**

---

### Part 4: Next Priorities

Ranked by **Value to LLM Consumer** × **Implementation Feasibility**.

#### 1. Fix Arrow Function Leaks & Missing Exports (JS/TS)

* **Problem:** React/Node codebases export arrow functions heavily. They are either completely omitted or leak their entire implementation bodies, ruining context token limits.
* **Design:**
1. In `extract_export_signatures`, add a match arm for `"lexical_declaration"` to capture `export const`.
2. For slicing arrow functions, do not pass `variable_declarator`. Use the cursor to navigate down to the `arrow_function` node, locate the `=>` token, and slice the source up to the `end_byte` of the `=>` token. Slice from the parent `lexical_declaration` to preserve `const/export`.


* **Complexity:** **Small** (Massive token savings).

#### 2. Reconcile Auto-Diff with Signatures

* **Problem:** Using auto-diff with signatures creates massive diffs of implementation bodies that the user explicitly asked to hide.
* **Design:** Pass `TreeSitterConfig` into `ProjectState::from_files`. If `signatures` is active, apply the tree-sitter extraction *before* storing the content in `FileState`. Store the formatted signatures string instead of the raw file bytes. `diff.rs` will then automatically (and cheaply) diff exactly what API boundaries changed.
* **Complexity:** **Medium**

#### 3. Wire Up "Smart" AST Truncation

* **Problem:** `--truncate smart` is a placebo. Global byte truncation cuts code blocks in half, resulting in invalid syntax.
* **Design:** Remove the global string truncation in `lib.rs`. Inside the `process_file` loop (in `markdown.rs`), track the remaining token budget. If a file pushes the budget over the limit, invoke `tree_sitter::find_smart_truncation_point` on that specific file's content, slice it cleanly at the AST boundary, append an `[Output Truncated]` notice, and stop processing further files.
* **Complexity:** **Medium**

#### 4. Rescue Python Decorators & C++ Header Prototypes

* **Problem:** I lose critical architectural context when decorators are dropped, decorated methods are misclassified, and C++ header files are skipped entirely.
* **Design:**
* *Python:* Intercept `"decorated_definition"` nodes and slice from there. Update `is_method` to walk up `parent()` iteratively until it hits a `class_definition` or module scope.
* *C/C++:* Add `"declaration"` to the node matcher so header prototypes are processed. Update `extract_class_signature` to use `slice_signature_before_body` with `["field_declaration_list"]`.


* **Complexity:** **Small**

#### 5. Synchronize the Cache Hash & Fix JSX Support

* **Problem:** Cache files continually overwrite each other; React `.jsx` files dump their full bodies.
* **Design:**
1. Delete `hash_config()` in `cache.rs`. Have `CacheManager::new` accept the pre-computed hash directly from `ProjectState::compute_config_hash(config)` for a single source of truth.
2. Add `"jsx"` to the `JavaScriptSupport.file_extensions()` array.


* **Complexity:** **Small**