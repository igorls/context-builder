//! Canonical extension → code-fence language map.
//!
//! Single source of truth for the language tag used in code fences. Before
//! v0.10 there were three independent `match extension` maps (file bodies in
//! `markdown.rs`, signature blocks in `markdown.rs`, and the auto-diff path in
//! `lib.rs`) that had already diverged: `.mjs` bodies got a non-highlightable
//! `` ```mjs `` fence while its signature block said `javascript`, and `.jsx`/
//! `.sh` files rendered through auto-diff got raw-extension fences. All call
//! sites now route through this module so any extension yields the same fence
//! language everywhere.
//!
//! Unknown extensions fall back to the raw extension (preserves pre-v0.10
//! behavior for exotic files).

/// Map a file extension to the code-fence language tag.
///
/// # Examples
///
/// ```
/// assert_eq!(context_builder::languages::language_for_extension("mjs"), "javascript");
/// assert_eq!(context_builder::languages::language_for_extension("jsx"), "jsx");
/// assert_eq!(context_builder::languages::language_for_extension("sh"), "bash");
/// assert_eq!(context_builder::languages::language_for_extension("weird"), "weird");
/// ```
pub fn language_for_extension(ext: &str) -> &str {
    match ext {
        // Rust
        "rs" => "rust",
        // JavaScript family — `mjs`/`cjs` are not highlightable fence tags,
        // so they fold to `javascript`; `jsx` is a distinct linguist language
        // and keeps its own (more precise) fence tag.
        "js" | "mjs" | "cjs" => "javascript",
        "jsx" => "jsx",
        // TypeScript family — same reasoning: `mts`/`cts` fold to
        // `typescript`; `tsx` is a distinct linguist language.
        "ts" | "mts" | "cts" => "typescript",
        "tsx" => "tsx",
        // Python
        "py" | "pyw" => "python",
        // Go
        "go" => "go",
        // Java
        "java" => "java",
        // C / C++
        "c" | "h" => "c",
        "cpp" | "cxx" | "cc" | "hpp" | "hxx" | "hh" => "cpp",
        // Data / markup / config
        "json" => "json",
        "toml" => "toml",
        "md" => "markdown",
        "yaml" | "yml" => "yaml",
        "html" => "html",
        "css" => "css",
        "sql" => "sql",
        "xml" => "xml",
        // Shell — `sh` is not a highlightable tag in most renderers; `bash`
        // is the convention GitHub and most markdown pipelines recognize.
        "sh" | "bash" => "bash",
        "lock" => "toml",
        // Unknown extension: use it verbatim (pre-v0.10 fallback behavior)
        _ => ext,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn js_family_folds_to_javascript_except_jsx() {
        assert_eq!(language_for_extension("js"), "javascript");
        assert_eq!(language_for_extension("mjs"), "javascript");
        assert_eq!(language_for_extension("cjs"), "javascript");
        // `jsx`/`tsx` are distinct linguist languages — keep their own fence
        assert_eq!(language_for_extension("jsx"), "jsx");
        assert_eq!(language_for_extension("tsx"), "tsx");
    }

    #[test]
    fn ts_family_folds_to_typescript() {
        assert_eq!(language_for_extension("ts"), "typescript");
        assert_eq!(language_for_extension("mts"), "typescript");
        assert_eq!(language_for_extension("cts"), "typescript");
    }

    #[test]
    fn shell_folds_to_bash() {
        assert_eq!(language_for_extension("sh"), "bash");
    }

    #[test]
    fn unknown_extension_falls_back_to_raw() {
        assert_eq!(language_for_extension("weird"), "weird");
        assert_eq!(language_for_extension("text"), "text");
    }

    #[test]
    fn core_languages() {
        assert_eq!(language_for_extension("rs"), "rust");
        assert_eq!(language_for_extension("py"), "python");
        assert_eq!(language_for_extension("go"), "go");
        assert_eq!(language_for_extension("java"), "java");
        assert_eq!(language_for_extension("c"), "c");
        assert_eq!(language_for_extension("cpp"), "cpp");
    }
}
