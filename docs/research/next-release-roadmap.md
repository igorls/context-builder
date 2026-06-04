# context-builder — Next Release Roadmap

> **Generated:** 2026-06-04 via multi-agent codebase analysis (subsystem mapping, prior-research mining, competitive refresh to mid-2026, adversarially-verified bug hunt, dependency + DX audit). Bug findings were adversarially verified (20/22 confirmed); the four headline "broken feature" claims were independently re-verified against live source.

_Current: v0.8.3 (Feb 2026), ~4 months dormant. All file:line references verified against live source unless marked "per corpus."_

---

## 1. Executive summary

context-builder is a healthy, conservatively-engineered Rust CLI with a **genuinely differentiated core**: relevance-ordered traversal, deterministic content-hash output for prompt caching, snapshot auto-diff/`--diff-only`, and feature-gated tree-sitter signature/structure extraction across 8 languages. The dependency stack is clean (no open advisories; crossbeam-channel already on the patched 0.5.15), the test surface is large (~232 unit + ~51 integration tests), and CI runs a 3-OS matrix. This is not a project in trouble.

It is, however, a project that has **stopped moving while the category accelerated**. Repomix shipped a release nearly every month of 2026 (v1.14.1 on 2026-05-27), and the Rust rival yek shipped v0.25.3 on 2026-06-02 — the most recent release of any tool reviewed — while context-builder's last release was Feb 2026. More importantly, the analysis surfaced a cluster of issues that are individually small but collectively corrosive to the project's *one core promise — accurate, trustworthy, LLM-optimized output*:

- **Two flagship features are partially or wholly broken.** `--truncate smart` is dead code (`find_smart_truncation_point` has zero production callers — verified). `--visibility public` is completely non-functional for Java (and C++): `get_visibility` returns `Visibility::All` unconditionally (java.rs:151 — verified), so `--visibility public` drops *every* Java symbol and `--visibility private` leaks *all* of them.
- **The token budget — the headline value prop — is inaccurate and non-deterministic.** `--max-tokens` is gated on a `buf.len()/4` byte heuristic (markdown.rs:156), the first file always bypasses the budget (`tokens_used > 0` guard, markdown.rs:160 — verified), parallel vs serial paths estimate differently (breaking the deterministic-hash guarantee across builds), and the tokenizer is hardcoded to `cl100k_base` (token_count.rs:10 — verified), which now *under-counts* every modern OpenAI model. The `--token-count` preview compounds this: it reads files via `read_to_string` and ignores tree-sitter enrichment and `--max-tokens` entirely, so even the *previewed* number won't match the produced document.
- **Output is Markdown-only** while every serious competitor offers XML (Anthropic's recommended format for Claude) and stdout piping — the single most conspicuous competitive gap.

### Recommended theme: **"Trustworthy output: accurate tokens, honest features, and pipe-friendly formats."**

This theme is *focused* (not a grab-bag), plays directly to the project's existing identity as the precision/correctness tool, and is **shippable**. It bundles (a) the highest-consensus verified bugs that make advertised features lie, (b) the lowest-effort/highest-credibility table-stakes wins (o200k_base, XML, stdout), and (c) a tokenizer/budget unification that retires a whole family of bugs at once. It deliberately **defers** the big strategic bets (MCP, remote repos) to the *following* release, because shipping a credible, momentum-restoring 0.9.0 in weeks beats shipping a sprawling one in months — and because MCP/remote should be built on top of a *trustworthy* core and a clean library API, not on top of the current split-brain pipeline.

A note on leverage: XML+stdout are the **table-stakes/competitive** king — they unblock the dominant 2026 pipe-to-LLM workflow and match every rival. They are not the *per-token-reasoning* king; by the corpus's own cross-model consensus, the dependency/module graph and doc-comment extraction are the highest output-quality wins. Those are deliberately deferred (graph to P2/v0.10, doc-comments to P2) so v0.9.0 stays focused and shippable.

---

## 2. Recommended next release — **v0.9.0 "Trustworthy Output"** (concrete, shippable scope)

Effort: S (≤½ day), M (1–3 days), L (~1 week).

### Core feature set
| # | Item | Effort | Rationale |
|---|------|--------|-----------|
| F1 | **Selectable tokenizer `--encoding {cl100k_base\|o200k_base}`, default `o200k_base`** | S | `o200k_base` already exists in the pinned tiktoken-rs 0.9.1, so this needs **no dep bump to function**; cl100k_base now under-counts every current OpenAI model. Lowest-effort, highest-credibility fix; directly upgrades `--token-count`/`--max-tokens`. (token_count.rs:10) |
| F2 | **`--stdout` / `-o -` output target** | M | The dominant 2026 usage pattern is `context-builder \| llm`; impossible today (always writes a file). The renderer already streams to `impl Write` — route it to stdout and gate chatter to stderr. |
| F3 | **`--format {markdown\|xml}` with an output-renderer trait** | L | XML (`<files>`/`<file path=…>`) is Anthropic's recommended Claude structure and Repomix's default. Introduce an `OutputFormat`/renderer trait so the streaming pipeline is parameterized. **Scope note:** format logic is currently hardwired in `process_file`/`write_text_content` AND there is a near-duplicate language map at markdown.rs:332 and :607 — the trait must dedup that map too, or formats will drift. |
| F4 | **Unify token budgeting on the real tokenizer + fix first-file bypass + debit header/tree + mirror counter** | M | Replace both `buf.len()/4` gates (markdown.rs:156; serial path) with `estimate_tokens()`, apply the budget to file index 0, **debit the document header + file-tree tokens before the per-file loop**, and **make `--token-count` share one rendering fn with the renderer** so the preview stops lying (folds B9). |
| F5 | **Wire `--truncate smart` into the budget path** | M | `find_smart_truncation_point` is fully built and tested but has **zero callers**. **Hard prerequisite: B19** (fallback returns non-char-boundary `max_bytes` → panic on slice). |
| F6 | **Implement Java + C++ visibility filtering (+ Java field-kind reclassification)** | M | `--visibility public` is a no-op (worse: actively wrong) for Java/C++. While in java.rs, also reclassify Java `field` (currently mis-kinded as `SignatureKind::Constant`). (B12) |
| F7 | **Constrain `--truncate`/`--visibility`/`--encoding` to clap `ValueEnum`** | S | Free validation + auto-generated `[possible values]` in `--help` + makes `--help` the source of truth, killing the doc-drift bug class (B3, B4). Does *not* cover B5 (`encoding_strategy` is config-only) — that's a separate config-load validation fix. |
| F8 | **Tree-sitter integration test through `run_with_args`** | M | The tree-sitter signature path has **zero integration-level coverage** today, yet F5/F6/B12–B18 rewrite it heavily. An end-to-end test driving `signatures`/`--visibility`/`--truncate smart` through `run_with_args` is a *dependency* of trusting those fixes. |

### Sequencing (strict chains)
- **F1 → F4 → B19 → F5.** F1 must land before F4 rewrites the budget gate onto `estimate_tokens()`. F4 establishes the per-file budget; F5 consumes it. B19 (char-boundary clamp) is a **blocking gate** on F5.
- **B13 → B14.** Both touch `find_function_name` in c.rs/cpp.rs. B14 only works *after* B13 makes the walker descend through `pointer_declarator`.
- **B6+B7 are one work item** (unified config-hash), not two.

---

## 3. Verified bug backlog (ordered by severity — quick wins)

### HIGH
| ID | Bug | File:line | Fix |
|----|-----|-----------|-----|
| B1 | **Token budget bypass: oversized first file always emitted in full** — `tokens_used > 0` guard short-circuits on index 0; parallel uses `buf.len()/4`, serial uses raw `metadata().len()/4`. | markdown.rs:158-178 (parallel), :257-268 (serial) | Apply budget to first file too; unify both paths on `estimate_tokens()`. (= F4) |
| B12 | **Java visibility filter non-functional both directions** — `get_visibility` returns `Visibility::All` unconditionally. Same no-op in cpp.rs. Bundle the Java `field`→`Constant` mis-kinding fix. | java.rs:147-152 (applied :162,213,248,283,318); cpp.rs:211 | Scan the `modifiers` node for public/private/protected. (= F6) |
| B13 | **C/C++ pointer/reference-return functions silently dropped** — `find_function_name` only checks direct `function_declarator` children. **Fix before B14.** | c.rs:302-315; cpp.rs:384-400 | Recurse through `pointer_declarator`/`reference_declarator`/`parenthesized_declarator`. |

### MEDIUM
| ID | Bug | File:line | Fix |
|----|-----|-----------|-----|
| B2 | **Content hash covers files that truncation omits — and the auto-diff path emits no content hash at all** (uses volatile `**Generated:**` timestamp). | markdown.rs:88-104; auto-diff path in lib.rs | Hash bytes actually written; give the auto-diff path a real content hash. (pairs with F4) |
| B6+B7 | **Cache/auto-diff config-hash omits output-affecting fields — fix as ONE unified-hash pass** — both omit `encoding_strategy`, `diff_only`, `output_folder`, `timestamped_output`; separately `effective_config` (lib.rs:338-346) stops after line_numbers so resolved `--signatures/--structure/--truncate/--visibility` never reach `final_config`. | cache.rs:94-119; state.rs:225-250; lib.rs:338-346, 862-866 | Collapse both hashers into one shared canonical fingerprint fn; add the four omitted fields; propagate resolved fields into `final_config`. |
| B8 | **`--diff-only` silently ignored when `auto_diff` off** — full content emitted, no warning. | lib.rs:333, 544-575 | Warn/error when `diff_only && !auto_diff`. |
| B14 | **C++ qualified return type misread as function name** — `std::string s(...)` → name="std::string". **Sequence after B13.** | cpp.rs:384-400 | Resolve name strictly from inside `function_declarator`. |
| B15 | **Rust bodiless trait methods (`function_signature_item`) dropped** — trait views, where signatures matter most, are incomplete. | rust.rs:138-184, :200 | Add `"function_signature_item"` arm; add to structure counter and `walk_for_boundary`. |

### LOW (cheap, bundle opportunistically)
| ID | Bug | File:line | Fix |
|----|-----|-----------|-----|
| B3 | Invalid `--visibility` values silently coerced to `all`. | cli.rs:67-69 | clap `ValueEnum`. (= F7) |
| B4 | Invalid `--truncate` values silently accepted; flag inert. | cli.rs:63-65 | `ValueEnum {Smart,Byte}`. (= F5 + F7) |
| B5 | Invalid `encoding_strategy` in config silently falls back to `detect`. **Config-only — NOT covered by F7.** | config.rs:71; markdown.rs:438-450 | Validate on config load; warn on others. |
| B9 | Token-count estimate diverges from output for non-UTF-8/binary; ignores tree-sitter enrichment and `--max-tokens`. **Folded into F4.** | token_count.rs:34-46 | Share one rendering fn between counter and renderer. |
| B16 | Python class base list double-parenthesized: `class User((Base))`. | python.rs:227, 236-240 | Strip parens before re-wrapping. |
| B17 | C/C++ struct/enum/alias drop inheritance/base types/aliased targets (`format!`-based). TS class inheritance loss is the same bug. | cpp.rs:316-366; c.rs:230-284; typescript.rs | Byte-slice up to body node / trailing `;`. |
| B18 | Rust `pub(crate)`/`pub(super)` reported as fully public. | rust.rs:221-237 | Inspect modifier text; treat restricted as a non-public tier. |
| B19 | `find_smart_truncation_point` fallback can return a non-char-boundary offset → panic when wired up. **Blocking gate on F5.** | truncation.rs:9-19 + per-lang fallback | Clamp with `ensure_utf8_boundary` before returning. |
| B20 | fs2 lock not atomic on crash (`set_len(0)` then write); truncated cache → silently dropped baseline. | cache.rs:167-204 | Write temp file + `fs::rename`. (pairs with fs2→fs4) |

---

## 4. Feature roadmap (prioritized)

| Priority | Feature | Effort | Impact | Rationale |
|----------|---------|--------|--------|-----------|
| **P0** | Model-accurate tokenizers (`--encoding`, o200k_base default) | S | High | cl100k_base under-counts every current OpenAI model; o200k_base already in pinned 0.9.1 |
| **P0** | stdout/pipe output | M | High | Unblocks `context-builder \| llm`; renderer already writes to `impl Write` |
| **P0** | Multi-format output (XML for Claude; later JSON) | L | High | Repomix *defaults* to XML; Anthropic recommends XML tags. Most-cited competitive gap |
| **P0** | Wire up `--truncate smart` (verified dead code) | M | High | Headline feature that does nothing today |
| **P1** | MCP server (`--mcp`, rmcp, stdio) | M-L | High | Both top Rust competitors ship it; agents consume context via MCP. Build on a library API first. Defer to v0.10 |
| **P1** | Secret scanning/redaction (`--redact`) | M | Med-High | Piping a repo to an LLM leaks keys; pure-Rust fits single-binary ethos |
| **P1** | Glob include patterns (`--include 'src/**/*.rs'`) | M | Medium | We only filter by extension; `file_utils.rs` already uses `OverrideBuilder` |
| **P1** | Remote-repo ingestion (`--remote <url>`, shallow clone) | M | Medium | Repomix/gitingest/codefetch all do it; removes first-use friction |
| **P1** | Git-aware diff baselines (`--diff-against <ref>`) | L | High | Turns auto-diff into "vs main" — highest-value evolution of the diff moat |
| **P2** | Doc-comment extraction into signatures | M | High (reasoning) | High cross-model consensus as a per-token-reasoning win |
| **P2** | Dependency/module graph + symbol map | L | High (reasoning) | Highest-consensus unshipped feature (9/10 models) |
| **P2** | More tree-sitter languages (C#, Ruby, PHP, Kotlin, Swift, Bash) | L | Medium | 8 langs misses popular ecosystems; pair with query-migration refactor |
| **P2** | Make tree-sitter a default feature | S | Medium | `cargo install` silently degrades AST features today |
| **P2** | Library/crate API surface | M | Medium | Foundation an MCP server should build on |
| **P3** | Watch mode, template engine, web playground/VSCode ext | M-L | Low-Med | Nice-to-haves / heavy distribution lifts; later |

---

## 5. Dependency & maintenance

Stack health is **good** — no open advisory affects pinned versions; crossbeam-channel already on patched 0.5.15.

**Bump (feature-relevant):** `tiktoken-rs` 0.9.1 → 0.12.0 (adds o-series/gpt-5/o200k_harmony mappings — an *enhancement* to F1, not a hard blocker since o200k_base already exists in 0.9.1).

**Dependency diet (one mechanical pass, edition-2024 MSRV makes std swaps free):**
- **Drop `walkdir`** — declared direct dep but *never imported* (traversal is all via `ignore`).
- **`fs2` → `fs4`** — fs2 unreleased since 2017 (RustSec unmaintained profile). Single lock site (cache.rs:7). Do alongside B20.
- **`num_cpus` → `std::thread::available_parallelism()`** — one site (markdown.rs:125).
- **`once_cell::Lazy` → `std::sync::LazyLock`** — one site (token_count.rs:10).

**Routine patch bumps** (manifest is source of truth): clap, chrono, tempfile (and de-dupe — listed in both `[dependencies]` and `[dev-dependencies]`), serde_json, toml, rayon floor, env_logger; **tree-sitter core + grammars together** (ABI pairing).

**CI/tooling gap:** No `cargo audit`/`cargo deny` and no dependabot/renovate. Add both (parallel infra track, not on the v0.9.0 critical path).

---

## 6. DX, docs & distribution

**Critical-path doc fixes (ship in v0.9.0):**
- **README.md:270** — `--truncate` documents `none (default) or smart`. Both wrong: clap default is `smart` (cli.rs:64), and `none` is fabricated. Fix to `smart (AST-aware, default) or byte`.
- **SKILL.md:160** — documents `smart … or simple`. `simple` isn't a real mode. Standardize on `smart`/`byte` everywhere.
- **README/SKILL** advertise "smart AST-boundary truncation" as working (README:80) — it's dead code. Either ship F5 or soften the claim.
- **README.md:257-272** — omits `--visibility` entirely (a real shipping flag) and never documents the `private` value.
- **README.md:95** still has `curl … | bash` despite commit 87657bf removing it from SKILL.md for a VirusTotal flag. Make this a release-gate checklist item.

**Critical-path distribution (ship in v0.9.0):**
1. **`[package.metadata.binstall]`** (S) — prebuilt archives already exist but `cargo binstall context-builder` can't find them.
2. **aarch64-unknown-linux-gnu release target** (M, **HIGH**) — install.sh advertises ARM Linux but release.yml never builds it → `curl|install.sh` 404s on Raspberry Pi/ARM cloud/WSL-ARM today.
3. **`examples/` dir + sample output.md** (S) — no before/after sample showcases the differentiators.

**Parallel infra track (does NOT gate the feature release):**
- AGENTS.md release process covers only crates.io; omits the tag-triggered binary release and winget bump. AGENTS.md/DEVELOPMENT.md layout omits `src/tree_sitter/`.
- CI MSRV job is fake — pins `stable`, no `rust-version` in Cargo.toml. Add `rust-version = "1.85"`.
- Homebrew tap (L), Winget automation (manifest stuck at 0.8.2), CHANGELOG-driven release notes (S), `cargo audit` + dependabot.

---

## 7. Deferred / stretch (later releases)

- **v0.10 strategic bets:** MCP server, remote-repo ingestion, git-aware `--diff-against <ref>`, library/crate API. Ship the clean **library API first**, then build MCP on top of it.
- **Architectural refactor (high internal leverage):** unify on `ResolvedConfig` end-to-end + stream auto-diff through a `Writer` + store hashes-not-content in the cache — retires an entire cluster of bugs (B6/B7, BTreeMap-ordering family, auto-diff OOM, encoding-bypass). Worth doing **before** building MCP/library API. Pairs with extracting `auto_diff.rs` out of the 2505-line lib.rs. (F4's diff-path point-patches are knowingly throwaway once this lands.)
- **Output-quality bets:** dependency/module graph + symbol map, doc-comment extraction, per-file token statistics, smart lockfile summarization, configurable relevance rules in TOML.
- **tree-sitter depth:** migrate hand-rolled walkers to `.scm` queries (de-risks every future language), add C#/Ruby/PHP/Kotlin/Swift/Bash, TS class inheritance/generics preservation (B17's TS parallel).
- **Distribution/ecosystem (heavy):** web playground, VSCode/browser extensions, Docker image + GHCR, GitHub Action.

**Bottom line:** Ship v0.9.0 "Trustworthy Output" soon — accurate tokenizers, honest features (truncate/visibility actually work, verified by a new tree-sitter integration test), and XML+stdout — bundled with the verified bug fixes and the dependency diet. Keep the infra overhaul on a parallel track. It restores momentum, plays to the project's precision identity, and lays clean groundwork (library API, unified config, streamed diff) for the MCP/remote-repo bets in v0.10.
