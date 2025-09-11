use ignore::{WalkBuilder, DirEntry};
use std::io::{self, Write};
use std::path::Path;

/// Collects all files to be processed using `ignore` crate for efficient traversal.
pub fn collect_files(base_path: &Path, filters: &[String], ignores: &[String]) -> io::Result<Vec<DirEntry>> {
    let mut walker = WalkBuilder::new(base_path);
    // By default, the ignore crate respects .gitignore and hidden files, so we don't need walker.hidden(false)

    for ignore_pattern in ignores {
        walker.add_ignore(ignore_pattern);
    }

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

    Ok(walker.build().filter_map(Result::ok).filter(|e| e.file_type().map_or(false, |ft| ft.is_file())).collect())
}

/// Asks for user confirmation if the number of files is large.
pub fn confirm_processing(file_count: usize) -> io::Result<bool> {
    if file_count > 100 {
        print!("Warning: You're about to process {} files. This might take a while. Continue? [y/N] ", file_count);
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