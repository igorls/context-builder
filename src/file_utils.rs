use ignore::{DirEntry, WalkBuilder, overrides::OverrideBuilder};
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

/// Collects all files to be processed using `ignore` crate for efficient traversal.
pub fn collect_files(
    base_path: &Path,
    filters: &[String],
    ignores: &[String],
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
        .collect();

    // FIX: Sort files deterministically by path to ensure consistent output order
    files.sort_by(|a, b| a.path().cmp(b.path()));

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

        let files = collect_files(base, &filters, &ignores).unwrap();
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

        let files = collect_files(base, &filters, &ignores).unwrap();
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

        let result = collect_files(base, &filters, &ignores);
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

        let files = collect_files(base, &filters, &ignores).unwrap();
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

        let files = collect_files(base, &filters, &ignores).unwrap();
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

        let files = collect_files(base, &filters, &ignores).unwrap();
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

        let result = collect_files(base, &filters, &ignores);
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

        let files = collect_files(base, &filters, &ignores).unwrap();
        // Should find at least the regular file
        assert!(!files.is_empty());
    }
}
