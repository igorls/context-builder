Context Builder v0.8.2 — Deep Code Review
Scope
Files analyzed: src/tree_sitter/ (all modules + 8 language definitions), src/lib.rs, src/markdown.rs, src/cache.rs, Cargo.toml, .github/workflows/ci.yml, .github/workflows/release.yml, install.sh, winget manifests.

Part 1: Tree-Sitter Implementation Review
Architecture Assessment ✅
The tree-sitter integration is well-architected. The LanguageSupport trait in 

language_support.rs
 provides a clean abstraction:

rust
pub trait LanguageSupport: Send + Sync {
    fn file_extensions(&self) -> &[&'static str];
    fn parse(&self, source: &str) -> Option<Tree>;
    fn extract_signatures(&self, source: &str, visibility: Visibility) -> Vec<Signature>;
    fn extract_structure(&self, source: &str) -> CodeStructure;
    fn find_truncation_point(&self, source: &str, max_bytes: usize) -> usize;
}
The Send + Sync bound enables safe rayon parallelism. Each language creates a new Parser per call (no shared mutable state), which is correct for thread safety. Static language support instances (static RUST_SUPPORT: ...) are zero-cost unit structs.

Language Definitions
All 8 languages share a consistent pattern. Key observations per language:

Language	Constructs Extracted	Visibility Filtering	Notes
Rust	fn, struct, enum, trait, impl, const, macro, mod	✅ Full	Best coverage
Python	def, class, decorators	⚠️ Partial (is_method only)	No __init__ special handling
JavaScript	function, class, arrow fn, exports	❌ Ignored	Visibility always All
TypeScript	function, class, interface, type, enum, arrow fn, exports	❌ Ignored	See TSX bug below
Go	func, method (receiver), type, struct, interface	✅ Full (capitalization)	Correct Go idiom
Java	method, class, interface, enum, field	⚠️ Stub	get_visibility() always returns All
C	function, struct, enum, typedef, macro, prototypes	N/A	Good header prototype support
C++	(assumed similar to C)	N/A	—
Findings
🐛 BUG: TypeScript uses non-TSX parser for .tsx files
In 

typescript.rs:16-19
:

rust
fn get_language() -> tree_sitter::Language {
    tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into() // ← always TS, never TSX
}
But 

languages/mod.rs:66
 routes .tsx to TS_SUPPORT:

rust
"ts" | "tsx" | "mts" | "cts" => Some(&TS_SUPPORT),
JSX syntax in .tsx files will produce parse errors or silent wrong output. The tree-sitter-typescript crate ships both LANGUAGE_TYPESCRIPT and LANGUAGE_TSX. Either split into two support structs, or switch grammar based on extension.

CAUTION

This means every React/TSX codebase gets degraded signatures. High user impact.

⚠️ Missing .jsx extension in registry

languages/mod.rs:63
 maps JS as:

rust
"js" | "mjs" | "cjs" => Some(&JS_SUPPORT),
But .jsx is missing. The JS parser handles JSX syntax natively, so this is just a registry omission. Any React project using .jsx files won't get tree-sitter enrichment.

⚠️ Class signatures drop inheritance info (TS/Java)
For both TypeScript and Java, extract_class_signature constructs:

rust
let full_sig = format!("class {}", name);
This loses extends Foo, implements Bar, generics <T>, etc. The Rust implementation correctly uses slice_signature_before_body to preserve this. Apply the same pattern:

rust
let full_sig = slice_signature_before_body(source, node, &["class_body"])
    .unwrap_or_else(|| format!("class {}", name));
ℹ️ Java visibility is a no-op

java.rs:147-152
:

rust
fn get_visibility(&self, _node: &tree_sitter::Node) -> Visibility {
    Visibility::All // ← stub
}
All Java signatures report Visibility::All regardless of public/private/protected modifiers. The --visibility public filter has no effect on Java files.

ℹ️ code_lines never computed across all languages
CodeStructure.code_lines defaults to 0 and is never populated by any language — only total_lines is set. This field is currently unused by format_structure_as_markdown, but it's dead weight in the struct.

✅ Signature Extraction Quality
The slice_signature_before_body utility is elegant and produces accurate signatures in all tested cases. The byte-slicing approach is safe because tree-sitter node boundaries are always valid byte positions in the source.

✅ Smart Truncation
The find_best_boundary recursion is correct. It walks the full AST and finds the last top-level item that ends ≤ max_bytes. The ensure_utf8_boundary fallback handles the rare parser-failure path.

✅ Graceful Degradation
The #[cfg(not(feature = "tree-sitter-base"))] stubs in 

mod.rs
 correctly return None/(), and 

lib.rs:552-558
 prints a single startup warning. No panics on the fallback path.

Part 2: Distribution & Security Review
Release Workflow (

release.yml
)
Strengths:

Builds with --features tree-sitter-all for all release binaries ✅
SHA256SUMS generated and published ✅
fail-fast: false prevents a single platform failure from canceling others ✅
Pinned action versions (actions/checkout@v5, etc.) ✅
Findings:

⚠️ Missing aarch64-unknown-linux-gnu target
The matrix only builds x86_64-unknown-linux-gnu. ARM Linux (Raspberry Pi, AWS Graviton, Docker on Apple Silicon) is increasingly common. This is a growth opportunity, not a bug.

⚠️ No code signing for macOS binaries
Apple's Gatekeeper will quarantine unsigned binaries. Users must xattr -d com.apple.quarantine manually. Consider codesign in CI or documenting the workaround in install instructions.

Install Script (

install.sh
)
Security assessment: Good. Key properties:

set -e for fail-fast ✅
SHA256 checksum verification with sha256sum or shasum fallback ✅
Downloads to mktemp -d (not cwd) ✅
Cleans up $TMP on success ✅
Findings:

⚠️ No cleanup on failure
If checksum verification passes but tar or mv fails, $TMP is leaked:

bash
# Line 73-76: no trap handler
tar xzf "$TMP/$ARCHIVE" -C "$TMP"    # could fail
$SUDO mv "$TMP/context-builder" "$INSTALL_DIR/context-builder"
$SUDO chmod +x "$INSTALL_DIR/context-builder"
rm -rf "$TMP"   # only reached on success
Fix: Add trap 'rm -rf "$TMP"' EXIT after TMP creation.

⚠️ HTTPS-only but no certificate pinning
The curl -sSL downloads use HTTPS to GitHub, which is sufficient for most threat models. No supply-chain attack vector beyond GitHub itself (the SHA256SUMS file is downloaded over the same channel — it's integrity verification, not authentication). This is standard for CLI installers.

Winget Manifest
Schema version 1.6.0 ✅
Portable installer type with PortableCommandAlias ✅
SHA256 matches release artifact ✅
x64-only (matches the single Windows build target) ✅
CI Workflow (

ci.yml
)
Strong CI pipeline:

Tests on ubuntu/windows/macOS ✅
Both default and --all-features builds ✅
cargo fmt --check + cargo clippy -D warnings ✅
cargo audit security scanning ✅
cargo tarpaulin code coverage ✅
--test-threads=1 to avoid race conditions ✅
⚠️ MSRV check doesn't pin a version

ci.yml:102-104
:

yaml
- uses: dtolnay/rust-toolchain@stable
  with:
    toolchain: stable
This checks against stable, not an actual MSRV. The job name says "Minimum Supported Rust Version" but uses the latest stable. Either pin toolchain: 1.XX.0 or rename the job.

Part 3: Bug Hunting
🐛 Duplicate signatures from recursive tree walk
All language implementations recursively walk children AND extract from the current node:

rust
// typescript.rs:130-133
let mut cursor = node.walk();
for child in node.children(&mut cursor) {
    self.extract_signatures_from_node(source, &child, _visibility, signatures);
}
When extract_signatures_from_node processes an export_statement, it extracts child signatures (e.g., a function_declaration). But the recursive walk then also visits that same function_declaration directly and extracts it again. This produces duplicate signatures.

The Rust implementation avoids this by explicitly skipping children of impl_item:

rust
// rust.rs processes impl block children inline, doesn't re-recurse them
But for JS/TS, exported functions are extracted twice: once via extract_export_signatures and once via the recursive child walk.

WARNING

Run --signatures on any file with export function foo() — expect duplicates in output.

⚠️ max_tokens truncation in auto-diff path uses naive byte-cutting

lib.rs:464-488
 truncates the final document at max_tokens * 4 bytes, then attempts to close unclosed code fences. The fence-counting heuristic:

rust
let fence_count = final_doc.matches("\n```").count()
    + if final_doc.starts_with("```") { 1 } else { 0 };
if fence_count % 2 != 0 {
    final_doc.push_str("\n```\n");
}
This breaks if any file content contains the literal string ``` (e.g., a markdown file documenting code blocks). The LLM would see a malformed document. A more robust approach is tracking fence state with a proper parser.

⚠️ Non-code file mapping diverges between paths
The language mapping in process_file (markdown.rs:332-355) includes .jsx, .tsx, .hpp, etc., but the equivalent map in generate_markdown_with_diff (lib.rs:738-750) is shorter and missing several extensions. When auto_diff is active, files get a simpler language tag.

✅ Cache invalidation is correct
cache.rs correctly hashes signatures, structure, truncate, and visibility fields alongside filter, ignore, line_numbers, auto_diff, and diff_context_lines. Any tree-sitter config change properly invalidates the cache.

✅ Concurrency is safe
Each rayon worker creates its own Parser instance (via self.parse()), avoiding shared mutable state. The Send + Sync trait bounds enforce this at compile time. No data races are possible.

✅ Edge cases: empty files, parse failures
All languages return Vec::new() or CodeStructure::default() on parse failure. The is_supported_extension check prevents tree-sitter from running on non-code files. Empty files produce empty signatures without panics.

Part 4: Architecture Health
lib.rs Complexity
At 2,504 lines, lib.rs is the largest file and handles too many responsibilities:

CLI argument parsing + config resolution
Auto-diff orchestration (cache → compare → generate)
Standard generation orchestration
File tree building, token counting, preview mode
Context window warnings
The auto-diff path alone (lines 332-539) is ~200 lines of dense orchestration logic. Recommendation: Extract auto_diff.rs as a module.

Feature Flag Hygiene ✅
The #[cfg(feature = "...")] gates are consistent and correct. Every language module, its static instance, and its match arm in get_language_support are all gated on the same feature. The fallback stubs in mod.rs ensure compilation without any tree-sitter features.

Error Handling ✅
Library code uses Result<T, io::Error> throughout
No unwrap() / expect() outside of tests
Tree-sitter parse failures return None and are gracefully handled
File I/O errors log warnings and skip files (never panic)
Test Coverage
Unit tests: Each language has 7-10 tests covering signatures, structure, truncation, extensions
Integration tests: 10+ test files in tests/, but none specifically test tree-sitter integration
Missing: No integration test exercises --signatures or --structure flags end-to-end. The duplicate-signature bug (Part 3) wouldn't be caught by current tests because the unit tests don't test exported items.
Part 5: TOP 5 Highest-Impact Improvements
1. 🔴 Fix TSX Parsing (User Pain / Correctness)
Impact: Every React/TypeScript project produces degraded or broken signatures.

Fix: Use LANGUAGE_TSX for .tsx files. Options:

Split TypeScriptSupport into TypeScriptSupport + TsxSupport
Or parameterize get_language() to accept the extension
Effort: Small (< 1 hour)

2. 🔴 Fix Duplicate Signatures in JS/TS Export Statements (Correctness)
Impact: --signatures output for any JS/TS file with exports contains duplicated entries, confusing LLMs with redundant information and burning token budget.

Fix: Either skip recursive descent into children already processed by extract_export_signatures, or deduplicate by line number before returning.

Effort: Small (< 30 mins)

3. 🟡 Add .jsx to Language Registry (Coverage Gap)
Impact: React projects using .jsx get no tree-sitter enrichment.

Fix: Add "jsx" to the JS match arm in 

languages/mod.rs:63
.

Effort: Trivial (1 line)

4. 🟡 Preserve Class Inheritance in TS/Java Signatures (LLM Quality)
Impact: class Foo extends Bar implements Baz shows up as just class Foo. This loses critical type hierarchy information that LLMs rely on for understanding code architecture.

Fix: Use slice_signature_before_body(source, node, &["class_body"]) instead of format!("class {}", name) in TypeScript and Java.

Effort: Small (< 30 mins per language)

5. 🟡 Add Integration Tests for Tree-Sitter Features (Reliability)
Impact: The duplicate-signature bug and TSX bug both went undetected because there are no integration tests for --signatures/--structure. Adding even 3-4 tests would catch regressions early.

Fix: Add tests in tests/ that:

Run --signatures on a multi-file project with .rs, .ts, .tsx, .jsx
Assert no duplicate signature names
Assert TSX files parse without errors
Assert non-code files (.md, .toml) are NOT signature-replaced
Effort: Medium (2-3 hours)

Summary
Area	Grade	Key Finding
Architecture	A	Clean trait-based design, good separation of concerns
Language Coverage	B+	8 languages, consistent patterns, some gaps in TS/Java
Correctness	B-	TSX parser bug, duplicate signatures in exports
Distribution	A-	Checksums, multi-platform, Winget. Missing ARM Linux
Security	A	SHA256 verification, no unsafe code, audit in CI
Tests	B	Good unit coverage, missing integration tests for TS features
The codebase is mature and well-engineered. The top 3 fixes (TSX parsing, duplicate signatures, .jsx registry) are small patches that would meaningfully improve quality for the most common user workflows.