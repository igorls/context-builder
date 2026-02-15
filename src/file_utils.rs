use ignore::{DirEntry, WalkBuilder, overrides::OverrideBuilder};
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

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
    // By default, the "ignore" crate respects .gitignore and hidden files, so we don't need walker.hidden(false)

    // Build overrides for custom ignore patterns
    let mut override_builder = OverrideBuilder::new(base_path);
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

    // Hardcoded auto-ignores for common heavy directories that should NEVER be
    // included, even when there's no .git directory (so .gitignore isn't read).
    // Without these, projects missing .git can produce million-line outputs
    // from dependency trees.
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
        let pattern = format!("!{}/**", dir);
        if let Err(e) = override_builder.add(&pattern) {
            log::warn!("Skipping invalid default-ignore '{}': {}", dir, e);
        }
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
}
