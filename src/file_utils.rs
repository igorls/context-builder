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
    if let Err(e) = override_builder.add("!.context-builder.toml") {
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
}
