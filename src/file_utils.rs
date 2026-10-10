use ignore::{DirEntry, WalkBuilder, overrides::OverrideBuilder};
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

use crate::markdown::{CONTENT_HASH_PREFIX, REPORT_TITLE_LINE};

/// Bytes of each file examined when looking for a previous context-builder
/// report. The signature is the header (`REPORT_TITLE_LINE` plus a
/// `CONTENT_HASH_PREFIX` line), which is written before the file tree.
const CONTEXT_OUTPUT_PREFIX_LEN: usize = 8 * 1024;

/// Cargo and other tools drop this file in cache directories (see
/// <https://bford.info/cachedir/>). `target/` contains one.
const CACHEDIR_TAG: &str = "CACHEDIR.TAG";

/// Returns a numeric category for file relevance ordering.
/// Lower numbers appear first in output. Categories:
/// 0 = Project config + key docs (Cargo.toml, README.md, AGENTS.md, etc.)
/// 1 = Source code (src/, lib/) — entry points sorted first within category
/// 2 = Tests and benchmarks (tests/, benches/, test/, spec/)
/// 3 = Documentation, scripts, and everything else
/// 4 = Generated/lock files (Cargo.lock, package-lock.json, etc.)
/// 5 = Build/CI infrastructure (.github/, .circleci/, Dockerfile, etc.)
fn file_relevance_category(path: &Path, base_path: &Path) -> u8 {
    let relative = path.strip_prefix(base_path).unwrap_or(path);
    let rel_str = relative.to_string_lossy();

    // Check filename for lockfiles first — these are lowest priority
    if let Some(name) = relative.file_name().and_then(|n| n.to_str()) {
        let lockfile_names = [
            "Cargo.lock",
            "package-lock.json",
            "yarn.lock",
            "pnpm-lock.yaml",
            "Gemfile.lock",
            "poetry.lock",
            "composer.lock",
            "go.sum",
            "bun.lockb",
            "flake.lock",
        ];
        if lockfile_names.contains(&name) {
            return 5;
        }

        // Check for config/manifest files + key project docs — highest priority
        let config_names = [
            // Package manifests
            "Cargo.toml",
            "package.json",
            "tsconfig.json",
            "pyproject.toml",
            "setup.py",
            "setup.cfg",
            "go.mod",
            "Gemfile",
            // Tool config
            "context-builder.toml",
            ".gitignore",
            // Key project documentation (LLMs need these for context)
            "README.md",
            "README",
            "README.txt",
            "README.rst",
            "AGENTS.md",
            "CLAUDE.md",
            "GEMINI.md",
            "COPILOT.md",
            "CONTRIBUTING.md",
            "CHANGELOG.md",
        ];
        if config_names.contains(&name) {
            return 0;
        }
    }

    // Check path prefix for category
    let first_component = relative
        .components()
        .next()
        .and_then(|c| c.as_os_str().to_str())
        .unwrap_or("");

    match first_component {
        "src" | "lib" | "crates" | "packages" | "internal" | "cmd" | "pkg" => {
            // Check sub-components for test directories within source trees.
            // e.g., src/tests/auth.rs should be cat 2 (tests), not cat 1 (source).
            let sub_path = rel_str.as_ref();
            if sub_path.contains("/tests/")
                || sub_path.contains("/test/")
                || sub_path.contains("/spec/")
                || sub_path.contains("/__tests__/")
                || sub_path.contains("/benches/")
                || sub_path.contains("/benchmarks/")
            {
                2
            } else {
                1
            }
        }
        "tests" | "test" | "spec" | "benches" | "benchmarks" | "__tests__" => 2,
        "docs" | "doc" | "examples" | "scripts" | "tools" | "assets" => 3,
        // Build/CI infrastructure — useful context but not core source
        ".github" | ".circleci" | ".gitlab" | ".buildkite" => 4,
        _ => {
            // Check extensions for additional heuristics
            if let Some(ext) = relative.extension().and_then(|e| e.to_str()) {
                match ext {
                    "rs" | "go" | "py" | "ts" | "js" | "java" | "c" | "cpp" | "h" | "hpp"
                    | "rb" | "swift" | "kt" | "scala" | "ex" | "exs" | "zig" | "hs" => {
                        // Source file not in a recognized dir — check if it's a test
                        // Use path boundaries to avoid false positives (e.g., "contest.rs")
                        if rel_str.contains("/test/")
                            || rel_str.contains("/tests/")
                            || rel_str.contains("/spec/")
                            || rel_str.contains("/__tests__/")
                            || rel_str.ends_with("_test.rs")
                            || rel_str.ends_with("_test.go")
                            || rel_str.ends_with("_spec.rb")
                            || rel_str.ends_with(".test.ts")
                            || rel_str.ends_with(".test.js")
                            || rel_str.ends_with(".spec.ts")
                            || rel_str.starts_with("test_")
                        {
                            2
                        } else {
                            1
                        }
                    }
                    "md" | "txt" | "rst" | "adoc" => 3,
                    _ => 1, // Unknown extension in root — treat as source
                }
            } else {
                // Check for build-related root files without extensions
                if let Some(
                    "Makefile" | "CMakeLists.txt" | "Dockerfile" | "Containerfile" | "Justfile"
                    | "Taskfile" | "Rakefile" | "Vagrantfile",
                ) = relative.file_name().and_then(|n| n.to_str())
                {
                    4
                } else {
                    3 // No extension — docs/other
                }
            }
        }
    }
}

/// Returns a sub-priority for sorting within the same relevance category.
/// Lower values appear first. Entry points (main, lib, mod) get priority 0,
/// other files get priority 1. This ensures LLMs see architectural entry
/// points before helper modules.
fn file_entry_point_priority(path: &Path) -> u8 {
    if let Some("main" | "lib" | "mod" | "index" | "app" | "__init__") =
        path.file_stem().and_then(|s| s.to_str())
    {
        0
    } else {
        1
    }
}

/// Collects all files to be processed using `ignore` crate for efficient traversal.
///
/// `auto_ignores` are runtime-computed exclusion patterns (e.g., the tool's own
/// output file or cache directory). They are processed identically to user ignores
/// but kept separate to avoid polluting user-facing configuration.
pub fn collect_files(
    base_path: &Path,
    filters: &[String],
    ignores: &[String],
    auto_ignores: &[String],
) -> io::Result<Vec<DirEntry>> {
    let mut walker = WalkBuilder::new(base_path);
    // A `.git` directory or gitdir file at the walk root or any ancestor is a
    // real checkout. Keep the crate defaults (`require_git(true)`,
    // `parents(true)`): parent ignore files inside that repo apply, and ignore
    // files above the repository do not.
    //
    // With no checkout, `require_git(false)` alone would still read every
    // ancestor `.gitignore` (a `$HOME` dotfiles pattern of `*` and `!*/`
    // then hides every file). `parents(false)` limits `.gitignore` and
    // `.ignore` to files inside the walk root, which is the B6 fix.
    if git_link_in_ancestors(base_path) {
        walker.require_git(true);
        walker.parents(true);
    } else {
        walker.require_git(false);
        walker.parents(false);
    }
    // Skip cache directories (Cargo's `target/` ships a CACHEDIR.TAG) without
    // descending into them. The root itself is never filtered out by the walker.
    walker.filter_entry(|entry| !directory_has_cachedir_tag(entry));

    // Build overrides for custom ignore patterns
    let mut override_builder = OverrideBuilder::new(base_path);

    // Hardcoded auto-ignores for common heavy directories. `.gitignore` is
    // applied even without a `.git` directory, but many trees never list
    // these names. Without the defaults, dependency folders can dominate
    // the output.
    //
    // IMPORTANT: These are added FIRST so that user ignores can override them.
    // The ignore crate uses "last-match-wins" semantics, so a user can whitelist
    // a legitimate "vendor" or "build" dir by passing it as a filter pattern.
    //
    // IMPORTANT: Patterns must NOT contain a slash — the ignore crate anchors
    // slash-containing patterns to the root, so `!dir/**` would only match
    // top-level dirs, missing nested ones like `apps/web/node_modules/`.
    let default_ignores = [
        "node_modules",
        "__pycache__",
        ".venv",
        "venv",
        ".tox",
        ".mypy_cache",
        ".pytest_cache",
        ".ruff_cache",
        "vendor",  // Go, PHP, Ruby
        ".bundle", // Ruby
        "bower_components",
        ".next",       // Next.js build output
        ".nuxt",       // Nuxt build output
        ".svelte-kit", // SvelteKit build output
        ".angular",    // Angular cache
        "dist",        // Common build output
        "build",       // Common build output
        ".gradle",     // Gradle cache
        ".cargo",      // Cargo registry cache
    ];
    for dir in &default_ignores {
        // No slash in pattern → matches at any depth (not root-anchored)
        let pattern = format!("!{}", dir);
        if let Err(e) = override_builder.add(&pattern) {
            log::warn!("Skipping invalid default-ignore '{}': {}", dir, e);
        }
    }
    // `target` is anchored to the walk root. An unanchored name would hide a
    // real source directory such as `src/target/`. Cargo build output at any
    // depth still carries a CACHEDIR.TAG and is skipped above.
    if let Err(e) = override_builder.add("!/target") {
        log::warn!("Skipping invalid default-ignore '/target': {}", e);
    }

    // User-specified ignore patterns (added AFTER defaults so they can override)
    for pattern in ignores {
        // Attention: Confusing pattern ahead!
        // Add the pattern to the override builder with ! prefix to ignore matching files.
        // In OverrideBuilder, patterns without ! are whitelist (include) patterns,
        // while patterns with ! are ignore patterns.
        let ignore_pattern = format!("!{}", pattern);
        if let Err(e) = override_builder.add(&ignore_pattern) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Invalid ignore pattern '{}': {}", pattern, e),
            ));
        }
    }
    // Apply auto-computed ignore patterns (output file, cache dir, etc.)
    for pattern in auto_ignores {
        let ignore_pattern = format!("!{}", pattern);
        if let Err(e) = override_builder.add(&ignore_pattern) {
            log::warn!("Skipping invalid auto-ignore pattern '{}': {}", pattern, e);
        }
    }
    // Also, always ignore the config file itself
    if let Err(e) = override_builder.add("!context-builder.toml") {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("Failed to add config ignore: {}", e),
        ));
    }

    let overrides = override_builder.build().map_err(|e| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("Failed to build overrides: {}", e),
        )
    })?;
    walker.overrides(overrides);

    if !filters.is_empty() {
        let mut type_builder = ignore::types::TypesBuilder::new();
        type_builder.add_defaults();
        for filter in filters {
            let _ = type_builder.add(filter, &format!("*.{}", filter));
            type_builder.select(filter);
        }
        let types = type_builder.build().unwrap();
        walker.types(types);
    }

    let mut files: Vec<DirEntry> = walker
        .build()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_some_and(|ft| ft.is_file()))
        .filter(|e| !is_prior_context_output(e.path()))
        .collect();

    // Sort files by relevance category, then entry-point priority, then alphabetically.
    // This puts config + docs first, then source code (entry points before helpers),
    // then tests, then docs/other, then build/CI, then lockfiles.
    // LLMs comprehend codebases better when core source appears before test scaffolding.
    files.sort_by(|a, b| {
        let cat_a = file_relevance_category(a.path(), base_path);
        let cat_b = file_relevance_category(b.path(), base_path);
        cat_a
            .cmp(&cat_b)
            .then_with(|| {
                file_entry_point_priority(a.path()).cmp(&file_entry_point_priority(b.path()))
            })
            .then_with(|| a.path().cmp(b.path()))
    });

    Ok(files)
}

/// True when `entry` is a directory that contains a `CACHEDIR.TAG` file.
fn directory_has_cachedir_tag(entry: &DirEntry) -> bool {
    entry.file_type().is_some_and(|ft| ft.is_dir()) && entry.path().join(CACHEDIR_TAG).is_file()
}

/// True when the walk root or one of its ancestors contains a `.git` directory
/// or file (worktrees and submodules use a gitdir file).
///
/// Relative roots are resolved against the current directory first. A relative
/// `-d .` otherwise has no real ancestors, so a repository above the process
/// cwd would be missed.
fn git_link_in_ancestors(base_path: &Path) -> bool {
    let mut current = absolute_walk_root(base_path);
    loop {
        if is_git_link(&current.join(".git")) {
            return true;
        }
        if !current.pop() {
            return false;
        }
    }
}

fn is_git_link(path: &Path) -> bool {
    fs::metadata(path).is_ok_and(|meta| meta.is_dir() || meta.is_file())
}

fn absolute_walk_root(base_path: &Path) -> PathBuf {
    let joined = if base_path.is_absolute() {
        base_path.to_path_buf()
    } else {
        match std::env::current_dir() {
            Ok(cwd) => cwd.join(base_path),
            Err(_) => base_path.to_path_buf(),
        }
    };
    let normalized = normalize_lexically(&joined);
    if normalized.as_os_str().is_empty() {
        PathBuf::from(".")
    } else {
        normalized
    }
}

/// Collapse `.` and `..` without touching the filesystem.
fn normalize_lexically(path: &Path) -> PathBuf {
    let mut out = PathBuf::new();
    for component in path.components() {
        match component {
            std::path::Component::CurDir => {}
            std::path::Component::ParentDir => {
                out.pop();
            }
            other => out.push(other.as_os_str()),
        }
    }
    out
}

/// True when `path` is a previous context-builder report.
///
/// The tool writes [`REPORT_TITLE_LINE`] as the first line and a
/// `Content hash:` line of 16 lowercase hex digits in the header
/// (see `markdown.rs`). Only [`CONTEXT_OUTPUT_PREFIX_LEN`] bytes are read.
fn is_prior_context_output(path: &Path) -> bool {
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(_) => return false,
    };
    let mut buf = [0u8; CONTEXT_OUTPUT_PREFIX_LEN];
    let n = match file.read(&mut buf) {
        Ok(n) => n,
        Err(_) => return false,
    };
    if !header_is_context_builder_output(&buf[..n]) {
        return false;
    }
    log::debug!(
        "skipping previous context-builder output: {}",
        path.display()
    );
    true
}

fn header_is_context_builder_output(prefix: &[u8]) -> bool {
    let text = String::from_utf8_lossy(prefix);
    let mut lines = text.lines();
    if lines.next() != Some(REPORT_TITLE_LINE) {
        return false;
    }
    let rest: Vec<&str> = lines.collect();
    if rest.iter().any(|l| is_content_hash_line(l)) {
        return true;
    }
    // The auto-diff renderer (lib.rs) writes the title followed directly by
    // `**Project:**` and `**Generated:**` lines and has no content hash.
    let mut meta = rest.iter().filter(|l| !l.is_empty());
    matches!(
        (meta.next(), meta.next()),
        (Some(p), Some(g)) if p.starts_with("**Project:** ") && g.starts_with("**Generated:** ")
    )
}

/// The header line `markdown.rs` writes: `Content hash: ` plus 16 lowercase hex digits.
fn is_content_hash_line(line: &str) -> bool {
    let Some(rest) = line.strip_prefix(CONTENT_HASH_PREFIX) else {
        return false;
    };
    rest.len() == 16 && rest.bytes().all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f'))
}

/// Asks for user confirmation if the number of files is large.
pub fn confirm_processing(file_count: usize) -> io::Result<bool> {
    if file_count > 100 {
        print!(
            "Warning: You're about to process {} files. This might take a while. Continue? [y/N] ",
            file_count
        );
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        if !input.trim().eq_ignore_ascii_case("y") {
            return Ok(false);
        }
    }
    Ok(true)
}

/// Asks for user confirmation to overwrite an existing file.
pub fn confirm_overwrite(file_path: &str) -> io::Result<bool> {
    print!("The file '{}' already exists. Overwrite? [y/N] ", file_path);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    if input.trim().eq_ignore_ascii_case("y") {
        Ok(true)
    } else {
        Ok(false)
    }
}

pub fn find_latest_file(dir: &Path) -> io::Result<Option<PathBuf>> {
    if !dir.is_dir() {
        return Ok(None);
    }

    let mut latest_file = None;
    let mut latest_time = std::time::SystemTime::UNIX_EPOCH;

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            let metadata = fs::metadata(&path)?;
            let modified = metadata.modified()?;
            if modified > latest_time {
                latest_time = modified;
                latest_file = Some(path);
            }
        }
    }

    Ok(latest_file)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;
    use tempfile::tempdir;

    fn to_rel_paths(mut entries: Vec<DirEntry>, base: &Path) -> Vec<String> {
        entries.sort_by_key(|e| e.path().to_path_buf());
        entries
            .iter()
            .map(|e| {
                e.path()
                    .strip_prefix(base)
                    .unwrap()
                    .to_string_lossy()
                    .replace('\\', "/")
            })
            .collect()
    }

    #[test]
    fn collect_files_respects_filters() {
        let dir = tempdir().unwrap();
        let base = dir.path();

        // create files
        fs::create_dir_all(base.join("src")).unwrap();
        fs::create_dir_all(base.join("scripts")).unwrap();
        fs::write(base.join("src").join("main.rs"), "fn main() {}").unwrap();
        fs::write(base.join("Cargo.toml"), "[package]\nname=\"x\"").unwrap();
        fs::write(base.join("README.md"), "# readme").unwrap();
        fs::write(base.join("scripts").join("build.sh"), "#!/bin/sh\n").unwrap();

        let filters = vec!["rs".to_string(), "toml".to_string()];
        let ignores: Vec<String> = vec![];

        let files = collect_files(base, &filters, &ignores, &[]).unwrap();
        let relative_paths = to_rel_paths(files, base);

        assert!(relative_paths.contains(&"src/main.rs".to_string()));
        assert!(relative_paths.contains(&"Cargo.toml".to_string()));
        assert!(!relative_paths.contains(&"README.md".to_string()));
        assert!(!relative_paths.contains(&"scripts/build.sh".to_string()));
    }

    #[test]
    fn collect_files_respects_ignores_for_dirs_and_files() {
        let dir = tempdir().unwrap();
        let base = dir.path();

        fs::create_dir_all(base.join("src")).unwrap();
        fs::create_dir_all(base.join("target")).unwrap();
        fs::create_dir_all(base.join("node_modules")).unwrap();

        fs::write(base.join("src").join("main.rs"), "fn main() {}").unwrap();
        fs::write(base.join("target").join("artifact.txt"), "bin").unwrap();
        fs::write(base.join("node_modules").join("pkg.js"), "console.log();").unwrap();
        fs::write(base.join("README.md"), "# readme").unwrap();

        let filters: Vec<String> = vec![];
        let ignores: Vec<String> = vec!["target".into(), "node_modules".into(), "README.md".into()];

        let files = collect_files(base, &filters, &ignores, &[]).unwrap();
        let relative_paths = to_rel_paths(files, base);

        assert!(relative_paths.contains(&"src/main.rs".to_string()));
        assert!(!relative_paths.contains(&"target/artifact.txt".to_string()));
        assert!(!relative_paths.contains(&"node_modules/pkg.js".to_string()));
        assert!(!relative_paths.contains(&"README.md".to_string()));
    }

    #[test]
    fn collect_files_handles_invalid_ignore_pattern() {
        let dir = tempdir().unwrap();
        let base = dir.path();

        fs::create_dir_all(base.join("src")).unwrap();
        fs::write(base.join("src").join("main.rs"), "fn main() {}").unwrap();

        let filters: Vec<String> = vec![];
        let ignores: Vec<String> = vec!["[".into()]; // Invalid regex pattern

        let result = collect_files(base, &filters, &ignores, &[]);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Invalid ignore pattern")
        );
    }

    #[test]
    fn collect_files_empty_directory() {
        let dir = tempdir().unwrap();
        let base = dir.path();

        let filters: Vec<String> = vec![];
        let ignores: Vec<String> = vec![];

        let files = collect_files(base, &filters, &ignores, &[]).unwrap();
        assert!(files.is_empty());
    }

    #[test]
    fn collect_files_no_matching_filters() {
        let dir = tempdir().unwrap();
        let base = dir.path();

        fs::write(base.join("README.md"), "# readme").unwrap();
        fs::write(base.join("script.py"), "print('hello')").unwrap();

        let filters = vec!["rs".to_string()]; // Only Rust files
        let ignores: Vec<String> = vec![];

        let files = collect_files(base, &filters, &ignores, &[]).unwrap();
        assert!(files.is_empty());
    }

    #[test]
    fn collect_files_ignores_config_file() {
        let dir = tempdir().unwrap();
        let base = dir.path();

        fs::write(base.join("context-builder.toml"), "[config]").unwrap();
        fs::write(base.join("other.toml"), "[other]").unwrap();

        let filters: Vec<String> = vec![];
        let ignores: Vec<String> = vec![];

        let files = collect_files(base, &filters, &ignores, &[]).unwrap();
        let relative_paths = to_rel_paths(files, base);

        assert!(!relative_paths.contains(&"context-builder.toml".to_string()));
        assert!(relative_paths.contains(&"other.toml".to_string()));
    }

    #[test]
    fn confirm_processing_small_count() {
        // Test that small file counts don't require confirmation
        let result = confirm_processing(50);
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn find_latest_file_empty_directory() {
        let dir = tempdir().unwrap();
        let result = find_latest_file(dir.path()).unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn find_latest_file_nonexistent_directory() {
        let dir = tempdir().unwrap();
        let nonexistent = dir.path().join("nonexistent");
        let result = find_latest_file(&nonexistent).unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn find_latest_file_single_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.txt");
        fs::write(&file_path, "content").unwrap();

        let result = find_latest_file(dir.path()).unwrap();
        assert!(result.is_some());
        assert_eq!(result.unwrap(), file_path);
    }

    #[test]
    fn find_latest_file_multiple_files() {
        let dir = tempdir().unwrap();

        let file1 = dir.path().join("old.txt");
        let file2 = dir.path().join("new.txt");

        fs::write(&file1, "old content").unwrap();
        std::thread::sleep(std::time::Duration::from_millis(10));
        fs::write(&file2, "new content").unwrap();

        let result = find_latest_file(dir.path()).unwrap();
        assert!(result.is_some());
        assert_eq!(result.unwrap(), file2);
    }

    #[test]
    fn find_latest_file_ignores_directories() {
        let dir = tempdir().unwrap();
        let subdir = dir.path().join("subdir");
        fs::create_dir(&subdir).unwrap();

        let file_path = dir.path().join("test.txt");
        fs::write(&file_path, "content").unwrap();

        let result = find_latest_file(dir.path()).unwrap();
        assert!(result.is_some());
        assert_eq!(result.unwrap(), file_path);
    }

    #[test]
    fn test_confirm_processing_requires_user_interaction() {
        // This test verifies the function signature and basic logic for large file counts
        // The actual user interaction cannot be tested in unit tests

        // For file counts <= 100, should return Ok(true) without prompting
        // This is already tested implicitly by the fact that small counts don't prompt

        // For file counts > 100, the function would prompt user input
        // We can't easily test this without mocking stdin, but we can verify
        // that the function exists and has the expected signature
        use std::io::Cursor;

        // Create a mock stdin that simulates user typing "y"
        let input = b"y\n";
        let _ = Cursor::new(input);

        // We can't easily override stdin in a unit test without complex setup,
        // so we'll just verify the function exists and handles small counts
        let result = confirm_processing(50);
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn test_confirm_overwrite_function_exists() {
        // Similar to confirm_processing, this function requires user interaction
        // We can verify it exists and has the expected signature

        // For testing purposes, we know this function prompts for user input
        // and returns Ok(true) if user types "y" or "Y", Ok(false) otherwise

        // The function signature should be:
        // pub fn confirm_overwrite(file_path: &str) -> io::Result<bool>

        // We can't easily test the interactive behavior without mocking stdin,
        // but we can ensure the function compiles and has the right signature
        let _: fn(&str) -> std::io::Result<bool> = confirm_overwrite;
    }

    #[test]
    fn test_collect_files_handles_permission_errors() {
        // Test what happens when we can't access a directory
        // This is harder to test portably, but we can test with invalid patterns
        let dir = tempdir().unwrap();
        let base = dir.path();

        // Test with a pattern that might cause issues
        let filters: Vec<String> = vec![];
        let ignores: Vec<String> = vec!["[invalid".into()]; // Incomplete bracket

        let result = collect_files(base, &filters, &ignores, &[]);
        assert!(result.is_err());
    }

    #[test]
    fn test_find_latest_file_permission_error() {
        // Test behavior when we can't read directory metadata
        use std::path::Path;

        // Test with a path that doesn't exist
        let nonexistent = Path::new("/this/path/should/not/exist/anywhere");
        let result = find_latest_file(nonexistent);

        // Should return Ok(None) for non-existent directories
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[test]
    fn test_collect_files_with_symlinks() {
        // Test behavior with symbolic links (if supported on platform)
        let dir = tempdir().unwrap();
        let base = dir.path();

        // Create a regular file
        fs::write(base.join("regular.txt"), "content").unwrap();

        // On Unix-like systems, try creating a symlink
        #[cfg(unix)]
        {
            use std::os::unix::fs::symlink;
            let _ = symlink("regular.txt", base.join("link.txt"));
        }

        // On Windows, symlinks require special privileges, so skip this part
        #[cfg(windows)]
        {
            // Just create another regular file to test
            fs::write(base.join("another.txt"), "content2").unwrap();
        }

        let filters: Vec<String> = vec![];
        let ignores: Vec<String> = vec![];

        let files = collect_files(base, &filters, &ignores, &[]).unwrap();
        // Should find at least the regular file
        assert!(!files.is_empty());
    }

    #[test]
    fn test_file_relevance_category_lockfiles() {
        let dir = tempdir().unwrap();
        let base = dir.path();

        let lockfiles = [
            "Cargo.lock",
            "package-lock.json",
            "yarn.lock",
            "pnpm-lock.yaml",
            "Gemfile.lock",
            "poetry.lock",
            "composer.lock",
            "go.sum",
            "bun.lockb",
            "flake.lock",
        ];

        for lockfile in &lockfiles {
            fs::write(base.join(lockfile), "lock content").unwrap();
        }

        let files = collect_files(base, &[], &[], &[]).unwrap();
        let paths: Vec<_> = files
            .iter()
            .map(|e| e.path().file_name().unwrap().to_str().unwrap())
            .collect();

        for lockfile in &lockfiles {
            assert!(
                paths.contains(lockfile),
                "Expected {} to be collected",
                lockfile
            );
        }
    }

    #[test]
    fn test_file_relevance_category_test_files_in_src() {
        let dir = tempdir().unwrap();
        let base = dir.path();

        fs::create_dir_all(base.join("src/tests")).unwrap();
        fs::create_dir_all(base.join("src/test")).unwrap();
        fs::create_dir_all(base.join("src/__tests__")).unwrap();

        fs::write(base.join("src/tests/auth.rs"), "fn test_auth() {}").unwrap();
        fs::write(base.join("src/test/helper.rs"), "fn helper() {}").unwrap();
        fs::write(base.join("src/__tests__/main.rs"), "fn main_test() {}").unwrap();
        fs::write(base.join("src/lib.rs"), "pub fn lib() {}").unwrap();

        let files = collect_files(base, &[], &[], &[]).unwrap();
        assert!(!files.is_empty());
    }

    #[test]
    fn test_file_relevance_category_benchmarks_in_src() {
        let dir = tempdir().unwrap();
        let base = dir.path();

        fs::create_dir_all(base.join("src/benches")).unwrap();
        fs::write(base.join("src/benches/my_bench.rs"), "fn bench() {}").unwrap();
        fs::write(base.join("src/main.rs"), "fn main() {}").unwrap();

        let files = collect_files(base, &[], &[], &[]).unwrap();
        assert_eq!(files.len(), 2);
    }

    #[test]
    fn test_file_relevance_category_test_file_patterns() {
        let dir = tempdir().unwrap();
        let base = dir.path();

        fs::write(base.join("my_test.rs"), "fn test() {}").unwrap();
        fs::write(base.join("test_main.rs"), "fn test_main() {}").unwrap();
        fs::write(base.join("my_test.go"), "func Test() {}").unwrap();
        fs::write(base.join("my_spec.rb"), "describe 'test' do end").unwrap();
        fs::write(base.join("app.test.ts"), "describe('test', () => {});").unwrap();
        fs::write(base.join("app.spec.ts"), "it('works', () => {});").unwrap();
        fs::write(base.join("main.rs"), "fn main() {}").unwrap();

        let files = collect_files(base, &[], &[], &[]).unwrap();
        assert_eq!(files.len(), 7);
    }

    #[test]
    fn test_file_relevance_category_build_files_without_extension() {
        let dir = tempdir().unwrap();
        let base = dir.path();

        fs::write(base.join("Makefile"), "all:\n\techo hello").unwrap();
        fs::write(base.join("Dockerfile"), "FROM alpine").unwrap();
        fs::write(base.join("Justfile"), "default:\n\techo hi").unwrap();

        let files = collect_files(base, &[], &[], &[]).unwrap();
        assert!(!files.is_empty());
    }

    #[test]
    fn test_collect_files_with_auto_ignores() {
        let dir = tempdir().unwrap();
        let base = dir.path();

        fs::write(base.join("test.txt"), "content").unwrap();

        let auto_ignores = vec!["test.txt".to_string()];
        let files = collect_files(base, &[], &[], &auto_ignores).unwrap();

        assert!(files.is_empty());
    }

    #[test]
    fn test_file_relevance_ci_directories() {
        let dir = tempdir().unwrap();
        let base = dir.path();

        fs::create_dir_all(base.join("ci")).unwrap();
        fs::write(base.join("ci/build.sh"), "#!/bin/sh").unwrap();

        let files = collect_files(base, &[], &[], &[]).unwrap();
        assert!(!files.is_empty());
    }

    #[test]
    fn test_file_relevance_docs_scripts_dirs() {
        let dir = tempdir().unwrap();
        let base = dir.path();

        fs::create_dir_all(base.join("docs")).unwrap();
        fs::create_dir_all(base.join("scripts")).unwrap();
        fs::create_dir_all(base.join("examples")).unwrap();
        fs::create_dir_all(base.join("tools")).unwrap();
        fs::create_dir_all(base.join("assets")).unwrap();

        fs::write(base.join("docs/guide.md"), "# Guide").unwrap();
        fs::write(base.join("scripts/build.sh"), "#!/bin/sh").unwrap();
        fs::write(base.join("examples/demo.rs"), "fn demo() {}").unwrap();
        fs::write(base.join("tools/helper.py"), "def helper(): pass").unwrap();
        fs::write(base.join("assets/logo.svg"), "<svg/>").unwrap();

        let files = collect_files(base, &[], &[], &[]).unwrap();
        assert_eq!(files.len(), 5);
    }

    #[test]
    fn test_file_relevance_various_source_extensions() {
        let dir = tempdir().unwrap();
        let base = dir.path();

        fs::write(base.join("main.go"), "package main").unwrap();
        fs::write(base.join("App.java"), "class App {}").unwrap();
        fs::write(base.join("main.py"), "print('hello')").unwrap();
        fs::write(base.join("app.swift"), "import Foundation").unwrap();
        fs::write(base.join("Main.kt"), "fun main() {}").unwrap();
        fs::write(base.join("main.scala"), "object Main {}").unwrap();
        fs::write(base.join("app.ex"), "defmodule App do end").unwrap();
        fs::write(base.join("main.zig"), "pub fn main() void {}").unwrap();
        fs::write(base.join("Lib.hs"), "module Lib where").unwrap();
        fs::write(base.join("main.c"), "int main() { return 0; }").unwrap();
        fs::write(base.join("main.cpp"), "int main() { return 0; }").unwrap();
        fs::write(base.join("main.rb"), "puts 'hello'").unwrap();

        let files = collect_files(base, &[], &[], &[]).unwrap();
        assert_eq!(files.len(), 12);
    }

    #[test]
    fn test_file_entry_point_priority_various_names() {
        let dir = tempdir().unwrap();
        let base = dir.path();

        fs::write(base.join("main.rs"), "fn main() {}").unwrap();
        fs::write(base.join("lib.rs"), "pub fn lib() {}").unwrap();
        fs::write(base.join("mod.rs"), "pub mod sub;").unwrap();
        fs::write(base.join("index.js"), "module.exports = {};").unwrap();
        fs::write(base.join("app.py"), "print('app')").unwrap();
        fs::write(base.join("__init__.py"), "pass").unwrap();
        fs::write(base.join("other.rs"), "fn other() {}").unwrap();

        let files = collect_files(base, &[], &[], &[]).unwrap();
        assert_eq!(files.len(), 7);
    }

    #[test]
    fn test_collect_files_config_files_priority() {
        let dir = tempdir().unwrap();
        let base = dir.path();

        fs::write(base.join("package.json"), "{}").unwrap();
        fs::write(base.join("tsconfig.json"), "{}").unwrap();
        fs::write(base.join("pyproject.toml"), "[project]").unwrap();
        fs::write(base.join("go.mod"), "module test").unwrap();
        fs::write(base.join("Gemfile"), "gem 'rails'").unwrap();
        fs::write(base.join("README.md"), "# Readme").unwrap();
        fs::write(base.join("AGENTS.md"), "# Agents").unwrap();
        fs::write(base.join("CONTRIBUTING.md"), "# Contrib").unwrap();
        fs::write(base.join("CHANGELOG.md"), "# Changes").unwrap();

        let files = collect_files(base, &[], &[], &[]).unwrap();
        assert!(!files.is_empty());
    }

    #[test]
    fn no_git_honors_gitignore_cachedir_tag_and_target() {
        let dir = tempdir().unwrap();
        let base = dir.path();

        fs::create_dir_all(base.join("src")).unwrap();
        fs::write(base.join("src/main.rs"), "fn main() {}").unwrap();
        fs::write(base.join("keep.txt"), "keep").unwrap();
        fs::write(base.join(".gitignore"), "secret.log\ngenerated/\n").unwrap();
        fs::write(base.join("secret.log"), "hidden").unwrap();
        fs::create_dir_all(base.join("generated")).unwrap();
        fs::write(base.join("generated/junk.txt"), "junk").unwrap();

        // Root `target/` with no tag. The default ignore is anchored at the
        // walk root, so this directory is still excluded.
        fs::create_dir_all(base.join("target/debug")).unwrap();
        fs::write(base.join("target/debug/x.d"), "dep").unwrap();

        // Nested source directory named `target`, no tag — must be kept.
        fs::create_dir_all(base.join("src/target")).unwrap();
        fs::write(base.join("src/target/notes.rs"), "fn notes() {}").unwrap();

        // Nested Cargo output: the tag skips it even though the name is not
        // anchored past the walk root.
        fs::create_dir_all(base.join("pkg/target")).unwrap();
        fs::write(
            base.join("pkg/target/CACHEDIR.TAG"),
            "Signature: 8a477f597d28d172789f06886806bc55\n",
        )
        .unwrap();
        fs::write(base.join("pkg/target/out.txt"), "artifact").unwrap();

        // Cache dir that is not named `target` — only the tag should exclude it.
        fs::create_dir_all(base.join("my-cache")).unwrap();
        fs::write(
            base.join("my-cache/CACHEDIR.TAG"),
            "Signature: 8a477f597d28d172789f06886806bc55\n",
        )
        .unwrap();
        fs::write(base.join("my-cache/blob.txt"), "blob").unwrap();

        // Nested `.gitignore` still applies when there is no checkout.
        fs::create_dir_all(base.join("sub")).unwrap();
        fs::write(base.join("sub/.gitignore"), "nested-secret.txt\n").unwrap();
        fs::write(base.join("sub/nested-secret.txt"), "nope").unwrap();
        fs::write(base.join("sub/ok.txt"), "yes").unwrap();

        let rel = to_rel_paths(collect_files(base, &[], &[], &[]).unwrap(), base);
        assert!(rel.contains(&"src/main.rs".to_string()));
        assert!(rel.contains(&"keep.txt".to_string()));
        assert!(rel.contains(&"src/target/notes.rs".to_string()));
        assert!(rel.contains(&"sub/ok.txt".to_string()));
        assert!(!rel.iter().any(|p| p.contains("secret.log")));
        assert!(!rel.iter().any(|p| p.contains("generated/")));
        assert!(
            !rel.iter()
                .any(|p| p == "target" || p.starts_with("target/"))
        );
        assert!(!rel.iter().any(|p| p.contains("pkg/target")));
        assert!(!rel.iter().any(|p| p.contains("my-cache/")));
        assert!(!rel.iter().any(|p| p.contains("nested-secret.txt")));
    }

    #[test]
    fn parent_gitignore_without_git_keeps_tree_files() {
        let dir = tempdir().unwrap();
        fs::write(dir.path().join(".gitignore"), "*\n!*/\n").unwrap();
        let proj = dir.path().join("proj");
        fs::create_dir_all(&proj).unwrap();
        fs::write(proj.join("keep.txt"), "keep").unwrap();
        fs::write(proj.join(".gitignore"), "secret.txt\n").unwrap();
        fs::write(proj.join("secret.txt"), "nope").unwrap();

        let rel = to_rel_paths(collect_files(&proj, &[], &[], &[]).unwrap(), &proj);
        assert!(rel.contains(&"keep.txt".to_string()));
        assert!(!rel.iter().any(|p| p == "secret.txt"));
    }

    #[test]
    fn repo_root_gitignore_applies_inside_nested_project() {
        let dir = tempdir().unwrap();
        // Dotfiles pattern above the repository must not apply.
        fs::write(dir.path().join(".gitignore"), "*\n!*/\n").unwrap();
        let repo = dir.path().join("repo");
        fs::create_dir_all(repo.join(".git")).unwrap();
        fs::write(repo.join(".gitignore"), "secret.txt\n").unwrap();
        let proj = repo.join("proj");
        fs::create_dir_all(&proj).unwrap();
        fs::write(proj.join("keep.txt"), "keep").unwrap();
        fs::write(proj.join("secret.txt"), "nope").unwrap();

        let rel = to_rel_paths(collect_files(&proj, &[], &[], &[]).unwrap(), &proj);
        assert!(
            rel.contains(&"keep.txt".to_string()),
            "parent dotfiles gitignore hid the tree: {rel:?}"
        );
        assert!(
            !rel.iter().any(|p| p == "secret.txt"),
            "repo-root .gitignore was not applied: {rel:?}"
        );
    }

    #[test]
    fn gitdir_file_counts_as_a_repo() {
        let dir = tempdir().unwrap();
        fs::write(dir.path().join(".gitignore"), "*\n!*/\n").unwrap();
        let repo = dir.path().join("repo");
        fs::create_dir_all(&repo).unwrap();
        fs::write(repo.join(".git"), "gitdir: /somewhere\n").unwrap();
        fs::write(repo.join(".gitignore"), "secret.txt\n").unwrap();
        let proj = repo.join("proj");
        fs::create_dir_all(&proj).unwrap();
        fs::write(proj.join("keep.txt"), "keep").unwrap();
        fs::write(proj.join("secret.txt"), "nope").unwrap();

        let rel = to_rel_paths(collect_files(&proj, &[], &[], &[]).unwrap(), &proj);
        assert!(rel.contains(&"keep.txt".to_string()), "{rel:?}");
        assert!(!rel.iter().any(|p| p == "secret.txt"), "{rel:?}");
    }

    #[test]
    fn skips_prior_report_header_but_not_near_misses() {
        let dir = tempdir().unwrap();
        let base = dir.path();
        fs::write(base.join("keep.txt"), "hello").unwrap();
        fs::write(
            base.join("old.md"),
            "# Directory Structure Report\n\nThis document contains all files from the `proj` directory, optimized for LLM consumption.\nContent hash: 0123456789abcdef\n\n## File Tree Structure\n",
        )
        .unwrap();
        // Title without the hash line the tool writes.
        fs::write(
            base.join("notes.md"),
            "# Directory Structure Report\n\nJust a heading.\n",
        )
        .unwrap();
        // Hash line, but not the report title.
        fs::write(base.join("other.md"), "Content hash: 0123456789abcdef\n").unwrap();
        // Title plus a hash line that is not 16 lowercase hex digits.
        fs::write(
            base.join("almost.md"),
            "# Directory Structure Report\n\nContent hash: not-a-real-hash\n",
        )
        .unwrap();

        let rel = to_rel_paths(collect_files(base, &[], &[], &[]).unwrap(), base);
        assert!(rel.contains(&"keep.txt".to_string()));
        assert!(rel.contains(&"notes.md".to_string()));
        assert!(rel.contains(&"other.md".to_string()));
        assert!(rel.contains(&"almost.md".to_string()));
        assert!(!rel.contains(&"old.md".to_string()));
    }

    #[test]
    fn skips_prior_auto_diff_report_but_not_near_misses() {
        let dir = tempdir().unwrap();
        let base = dir.path();
        fs::write(base.join("keep.txt"), "hello").unwrap();
        fs::write(
            base.join("auto.md"),
            "# Directory Structure Report\n\n**Project:** proj\n**Generated:** 2026-01-01 00:00:00 UTC\n\n## File Tree Structure\n",
        )
        .unwrap();
        // Title and Project line, but no Generated line.
        fs::write(
            base.join("near.md"),
            "# Directory Structure Report\n\n**Project:** proj\nSome prose.\n",
        )
        .unwrap();

        let rel = to_rel_paths(collect_files(base, &[], &[], &[]).unwrap(), base);
        assert!(rel.contains(&"keep.txt".to_string()));
        assert!(rel.contains(&"near.md".to_string()));
        assert!(!rel.contains(&"auto.md".to_string()));
    }

    #[test]
    fn anchored_auto_ignore_keeps_nested_same_basename() {
        let dir = tempdir().unwrap();
        let base = dir.path();
        fs::create_dir_all(base.join("docs")).unwrap();
        fs::write(base.join("output.md"), "# user notes at the root\n").unwrap();
        fs::write(base.join("docs/output.md"), "real doc\n").unwrap();
        fs::write(base.join("keep.txt"), "k").unwrap();

        let rel = to_rel_paths(
            collect_files(base, &[], &[], &["/output.md".to_string()]).unwrap(),
            base,
        );
        assert!(!rel.contains(&"output.md".to_string()));
        assert!(rel.contains(&"docs/output.md".to_string()));
        assert!(rel.contains(&"keep.txt".to_string()));
    }
}
