Here is a comprehensive review of the `context-builder` codebase, evaluating bugs, architecture, and providing actionable improvements.

### Part 1: Bugs

**1. Incomplete UTF-8 Boundary Detection Truncates Valid Files**

* **Location:** `src/markdown.rs`, `process_file()` (lines 240-259)
* **Severity:** High
* **What is wrong:** The fallback logic tries to avoid splitting multi-byte characters at the 8192-byte `sniff` buffer boundary. However, `sniff[end - 1] & 0xC0 == 0x80` only matches *continuation* bytes. If the boundary falls exactly on a *leading* byte (e.g., `0xE2`), the bitwise check evaluates to `0xC0 != 0x80`. The `while` loop doesn't execute, leaving the incomplete byte at the end of the slice, which subsequently causes `std::str::from_utf8` to fail.
* **Why it matters:** Perfectly valid UTF-8 source files are falsely classified as binary and their entire contents are omitted from the LLM context if a multi-byte character aligns perfectly with the buffer edge.
* **How to fix:** Let the standard library resolve the boundary natively:
```rust
let is_utf8 = match std::str::from_utf8(&sniff[..n]) {
    Ok(_) => true,
    Err(e) if e.error_len().is_none() => std::str::from_utf8(&sniff[..e.valid_up_to()]).is_ok(),
    Err(_) => false,
};

```



**2. Auto-Ignore Glob Fails for CLI-Provided Outputs**

* **Location:** `src/lib.rs`, `run_with_args()` (lines 177-187)
* **Severity:** High
* **What is wrong:** To prevent the tool from reading its own past outputs, a glob is created. If `config.output` is `None` (user specified output via CLI), `base_stem` falls back to `stem`. However, `stem` comes from `final_args.output`, which *already* had the current timestamp appended in `config_resolver.rs`.
* **Why it matters:** The generated glob becomes `output_20260216000000_*.md` instead of `output_*.md`. It completely fails to match previous context files (e.g., `output_20260215000000.md`). The tool ingests its own past outputs, exponentially bloating the LLM context size on every run.
* **How to fix:** Strip the trailing timestamp regex `_\d{14}` from `stem`, or pass the original unmodified stem from `config_resolver.rs` via the `ResolvedConfig` struct.

**3. Auto-Ignores Silently Exclude Valid User Directories**

* **Location:** `src/lib.rs` (line 207) & `src/file_utils.rs` (line 129)
* **Severity:** High
* **What is wrong:** `auto_ignores.push(output_folder.clone())` pushes the raw folder name (e.g., `"docs"`). The `ignore` crate prepends a `!` to this, producing `!docs`.
* **Why it matters:** The `ignore` crate interprets `!docs` as a command to ignore *any* directory named `docs` anywhere in the tree. If a user sets their output folder to `"docs"`, all of their actual documentation source files are silently omitted from the context.
* **How to fix:** Delete `auto_ignores.push(output_folder)`. The file-specific glob exclusion calculated just above it (`docs/context_*.md`) is sufficient.

**4. CLI Flags Ignored in Cache Invalidation**

* **Location:** `src/lib.rs`, `run_with_args()` (lines 269-277)
* **Severity:** Medium
* **What is wrong:** To generate the `config_hash` for auto-diff cache invalidation, the code manually merges `final_args` back into `effective_config`. However, it only merges `filter`, `ignore`, and `line_numbers`. It forgets to merge `signatures`, `structure`, `truncate`, `visibility`, and `max_tokens`.
* **Why it matters:** If a user runs `context-builder --auto-diff` and then runs it again with `--signatures`, the `config_hash` is identical. The tool incorrectly hits the cache and skips extracting signatures, rendering the CLI flags dead in auto-diff mode.
* **How to fix:** Merge the missing fields into `effective_config`, or better yet, hash a single `ResolvedConfig` struct directly.

**5. Configured Diff Context Lines are Ignored**

* **Location:** `src/state.rs`, `compare_with()` (line 106)
* **Severity:** Medium
* **What is wrong:** The method calls `diff_file_contents` with `explicit_context: None`.
* **Why it matters:** If a user specifies `diff_context_lines = 5` in `context-builder.toml`, the setting is parsed but completely ignored by the diffing engine. It always falls back to the default of 3 lines.
* **How to fix:** Pass `config.diff_context_lines` down through `ProjectState::compare_with` so it reaches `diff_file_contents`.

**6. Double I/O and File Descriptor Leaks**

* **Location:** `src/markdown.rs`, `process_file()` (lines 352-355)
* **Severity:** Low
* **What is wrong:** The code opens a `File`, reads up to 8KB to sniff encoding, and executes `file.seek(SeekFrom::Start(0))` to reset the cursor. Immediately after, it calls `std::fs::read_to_string(file_path)`.
* **Why it matters:** `fs::read_to_string` opens a completely *new* file descriptor. The `seek` on the original file is pointless, and the original file descriptor is leaked until the end of the block. This doubles OS file handle usage and path traversal overhead.
* **How to fix:** Reuse the open file: `let mut content = String::new(); file.read_to_string(&mut content)?;`.

---

### Part 2: Architecture

**Positives:**

* **Modularity:** The codebase is well-structured. The `tree_sitter` integrations are elegantly encapsulated behind a `LanguageSupport` trait and feature gates, keeping the binary lightweight by default.
* **Concurrency:** `markdown.rs` uses an excellent `rayon` pipeline and bounded `crossbeam_channel` to process massive amounts of files in parallel with low memory overhead.
* **Test Coverage:** Outstanding. The test suite utilizes `tempdir` and `serial_test` to aggressively test configuration permutations, CWD independence, and binary file recovery.

**Design Issues:**

* **Split-Brain Config State:** Logic drift causes bugs (like Bug #4). `config_resolver.rs` correctly merges CLI and TOML into `ResolvedConfig`, but `main.rs` instantly unpacks it back into `Args` and `Config`. `lib.rs` then tries to manually glue them back together.
* **Memory Exhaustion in Auto-Diff:** The `rayon` streaming pipeline is completely abandoned in `--auto-diff` mode. `ProjectState` buffers the raw text of *every file* into RAM. `compare_with()` clones those strings again. `generate_markdown_with_diff()` then buffers the *entire markdown document* in a single `String`. On medium-to-large repositories, this will trigger severe Out-Of-Memory (OOM) crashes.
* **Dead Code:** `src/tree_sitter/truncation.rs` contains over 1,000 lines of parsed Tree-sitter logic across 9 languages for AST-aware "smart" truncation, but the function `find_smart_truncation_point` is never invoked anywhere in the codebase.

---

### Part 3: Top 5 Improvements

Ranked by **Impact × Feasibility**:

**1. Decouple File Contents from Mega-JSON Cache**

* **Problem:** Serializing the full uncompressed text of every file into a monolithic `.context-builder/cache/state.json` destroys disk space and RAM, causing massive overhead on consecutive runs.
* **Design:** Store only file metadata and XXH3 `content_hash`es in `state.json`. During the `compare_with` phase, only read full file contents from the live disk if the hash mismatched the cache. Delete the raw `content` field from `FileState`.
* **Complexity:** M (Files: `src/state.rs`, `src/cache.rs`)

**2. Stream Auto-Diff Output to Prevent OOMs**

* **Problem:** `generate_markdown_with_diff` allocates the entire output document dynamically in RAM (`let mut output = String::new();`), causing huge memory spikes.
* **Design:** Refactor the function to accept a `&mut impl Write` reference. Replace `output.push_str()` with `writeln!(output, ...)` to stream chunks directly to the OS disk buffer, matching the safe architecture used in `markdown.rs`.
* **Complexity:** M (Files: `src/lib.rs`)

**3. Wire Up the Dead "Smart Truncation" Feature**

* **Problem:** Users explicitly asking for `--truncate smart` alongside a `--max-tokens` budget get crude byte-level cutoffs that break code syntax, because the feature is unlinked dead code.
* **Design:** Inside `src/markdown.rs` (`process_file`), evaluate the token budget. If adding the current file breaches the limit and `ts_config.truncate == "smart"`, invoke the existing `find_smart_truncation_point` on the file string, cleanly slice the chunk at the AST boundary, write it, and break the loop.
* **Complexity:** M (Files: `src/markdown.rs`)

**4. Single Source of Truth for Config Resolution**

* **Problem:** Manual configuration un-merging and re-merging leads to bugs where CLI flags are ignored.
* **Design:** Make `ResolvedConfig` the definitive state. Pass it directly to `run_with_args`, `ProjectState`, and `CacheManager`. Delete the redundant `Config` and `Args` objects from internal function signatures and hash `ResolvedConfig` directly for the cache.
* **Complexity:** S (Files: `src/lib.rs`, `src/main.rs`, `src/config_resolver.rs`)

**5. Normalize Windows Paths for Sorting & Ignores**

* **Problem:** On Windows, `file_relevance_category` fails to prioritize tests because `sub_path.contains("/tests/")` doesn't match backslashes.
* **Design:** Call `.replace('\\', "/")` on the relative path string in `file_utils.rs` before performing substring checks.
* **Complexity:** S (Files: `src/file_utils.rs`)