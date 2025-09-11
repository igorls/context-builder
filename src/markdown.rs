use chrono::Utc;
use ignore::DirEntry;
use log::{error, info, warn};
use std::fs;
use std::io::{self, BufRead, BufReader, Read, Seek, SeekFrom, Write};
use std::path::Path;

use crate::tree::{FileTree, write_tree_to_file};

/// Generates the final markdown file.
#[allow(clippy::too_many_arguments)]
pub fn generate_markdown(
    output_path: &str,
    input_dir: &str,
    filters: &[String],
    ignores: &[String],
    file_tree: &FileTree,
    files: &[DirEntry],
    base_path: &Path,
    line_numbers: bool,
) -> io::Result<()> {
    let mut output = fs::File::create(output_path)?;

    // --- Header --- //
    writeln!(output, "# Directory Structure Report\n")?;

    if !filters.is_empty() {
        writeln!(
            output,
            "This document contains files from the `{}` directory with extensions: {}",
            input_dir,
            filters.join(", ")
        )?;
    } else {
        writeln!(
            output,
            "This document contains all files from the `{}` directory, optimized for LLM consumption.",
            input_dir
        )?;
    }

    if !ignores.is_empty() {
        writeln!(output, "Custom ignored patterns: {}", ignores.join(", "))?;
    }

    writeln!(
        output,
        "Processed at: {}",
        Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
    )?;
    writeln!(output)?;

    // --- File Tree --- //
    writeln!(output, "## File Tree Structure\n")?;
    write_tree_to_file(&mut output, file_tree, 0)?;

    // --- File Contents --- //

    #[cfg(feature = "parallel")]
    {
        use rayon::prelude::*;
        let results: Vec<io::Result<Vec<u8>>> = files
            .par_iter()
            .map(|entry| {
                let mut buf = Vec::new();
                match process_file(base_path, entry.path(), &mut buf, line_numbers) {
                    Ok(()) => Ok(buf),
                    Err(e) => Err(e),
                }
            })
            .collect();

        for chunk in results {
            match chunk {
                Ok(buf) => output.write_all(&buf)?,
                Err(e) => return Err(e),
            }
        }
    }

    #[cfg(not(feature = "parallel"))]
    {
        for entry in files {
            process_file(base_path, entry.path(), &mut output, line_numbers)?;
        }
    }

    Ok(())
}

/// Processes a single file and writes its content to the output.
fn process_file(
    base_path: &Path,

    file_path: &Path,

    output: &mut impl Write,
    line_numbers: bool,
) -> io::Result<()> {
    let relative_path = file_path.strip_prefix(base_path).unwrap_or(file_path);
    info!("Processing file: {}", relative_path.display());

    let metadata = match fs::metadata(file_path) {
        Ok(meta) => meta,
        Err(e) => {
            error!(
                "Failed to get metadata for {}: {}",
                relative_path.display(),
                e
            );
            return Ok(());
        }
    };

    let modified_time = metadata
        .modified()
        .ok()
        .map(|time| {
            let system_time: chrono::DateTime<Utc> = time.into();
            system_time.format("%Y-%m-%d %H:%M:%S UTC").to_string()
        })
        .unwrap_or_else(|| "Unknown".to_string());

    // --- File Header --- //
    writeln!(output)?;
    writeln!(output, "## File: `{}`", relative_path.display())?;
    writeln!(output)?;
    writeln!(output, "- Size: {} bytes", metadata.len())?;
    writeln!(output, "- Modified: {}", modified_time)?;
    writeln!(output)?;

    // --- File Content --- //
    let extension = file_path
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("text");
    let language = match extension {
        "rs" => "rust",
        "js" => "javascript",
        "ts" => "typescript",
        "jsx" => "jsx",
        "tsx" => "tsx",
        "json" => "json",
        "toml" => "toml",
        "md" => "markdown",
        "yaml" | "yml" => "yaml",
        "html" => "html",
        "css" => "css",
        "py" => "python",
        "java" => "java",
        "cpp" => "cpp",
        "c" => "c",
        "h" => "c",
        "hpp" => "cpp",
        "sql" => "sql",
        "sh" => "bash",
        "xml" => "xml",
        "lock" => "toml",
        _ => extension,
    };

    // Stream file content for performance and handle binary files
    // Peek into the file to determine if it's likely text (UTF-8) without loading entire file
    match fs::File::open(file_path) {
        Ok(mut file) => {
            let mut sniff = [0u8; 8192];
            let n = match file.read(&mut sniff) {
                Ok(n) => n,
                Err(e) => {
                    warn!(
                        "Could not read file {}: {}. Skipping content.",
                        relative_path.display(),
                        e
                    );

                    writeln!(output, "```text")?;

                    writeln!(
                        output,
                        "<Could not read file content (e.g., binary file or permission error)>"
                    )?;

                    writeln!(output, "```")?;

                    return Ok(());
                }
            };
            let slice = &sniff[..n];
            let is_text = !slice.contains(&0) && std::str::from_utf8(slice).is_ok();

            if !is_text {
                warn!(
                    "Detected non-text or binary file {}. Skipping content.",
                    relative_path.display()
                );
                writeln!(output, "```text")?;
                writeln!(
                    output,
                    "<Could not read file content (e.g., binary file or permission error)>"
                )?;
                writeln!(output, "```")?;
                return Ok(());
            }

            // Reset cursor and stream the content
            if let Err(e) = file.seek(SeekFrom::Start(0)) {
                warn!(
                    "Could not reset file cursor for {}: {}. Skipping content.",
                    relative_path.display(),
                    e
                );
                writeln!(output, "```text")?;
                writeln!(
                    output,
                    "<Could not read file content (e.g., binary file or permission error)>"
                )?;
                writeln!(output, "```")?;
                return Ok(());
            }

            writeln!(output, "```{}", language)?;
            let mut reader = BufReader::new(file);

            if line_numbers {
                let mut buf = String::new();
                let mut line_no: usize = 1;
                loop {
                    buf.clear();
                    match reader.read_line(&mut buf) {
                        Ok(0) => break,
                        Ok(_) => {
                            // Trim only trailing newline to avoid doubling
                            let line = buf.strip_suffix('\n').unwrap_or(&buf);
                            // Also handle Windows CRLF by trimming trailing '\r'
                            let line = line.strip_suffix('\r').unwrap_or(line);
                            writeln!(output, "{:>4} | {}", line_no, line)?;
                            line_no += 1;
                        }
                        Err(e) => {
                            warn!(
                                "Error while reading {}: {}. Output may be truncated.",
                                relative_path.display(),
                                e
                            );
                            break;
                        }
                    }
                }
            } else {
                // Fast path: stream bytes to output
                if let Err(e) = std::io::copy(&mut reader, output) {
                    warn!(
                        "Error while streaming {}: {}. Output may be truncated.",
                        relative_path.display(),
                        e
                    );
                }
            }
            writeln!(output, "```")?;
        }
        Err(e) => {
            warn!(
                "Could not open file {}: {}. Skipping content.",
                relative_path.display(),
                e
            );
            writeln!(output, "```text")?;
            writeln!(
                output,
                "<Could not read file content (e.g., binary file or permission error)>"
            )?;
            writeln!(output, "```")?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_code_block_formatting() {
        let dir = tempdir().unwrap();
        let base_path = dir.path();
        let file_path = base_path.join("test.rs");
        let output_path = base_path.join("output.md");

        // Create a test Rust file
        fs::write(
            &file_path,
            "fn main() {\n    println!(\"Hello, world!\");\n}",
        )
        .unwrap();

        // Create output file
        let mut output = fs::File::create(&output_path).unwrap();

        // Process the file
        process_file(base_path, &file_path, &mut output, false).unwrap();

        // Read the output
        let content = fs::read_to_string(&output_path).unwrap();

        // Check that code blocks are properly formatted
        assert!(content.contains("```rust"));
        assert!(content.contains("```") && content.matches("```").count() >= 2);
    }

    #[test]
    fn test_markdown_file_formatting() {
        let dir = tempdir().unwrap();
        let base_path = dir.path();
        let file_path = base_path.join("README.md");
        let output_path = base_path.join("output.md");

        // Create a test markdown file
        fs::write(&file_path, "# Test\n\nThis is a test markdown file.").unwrap();

        // Create output file
        let mut output = fs::File::create(&output_path).unwrap();

        // Process the file
        process_file(base_path, &file_path, &mut output, false).unwrap();

        // Read the output
        let content = fs::read_to_string(&output_path).unwrap();

        // Debug print the content
        println!("Generated content:\n{}", content);

        // Check that markdown files use the correct language identifier
        assert!(
            content.contains("```markdown"),
            "Content should contain '```markdown' but was: {}",
            content
        );
        // Count the number of code block markers
        let code_block_markers = content.matches("```").count();

        assert!(
            code_block_markers >= 2,
            "Expected at least 2 code block markers, found {}",
            code_block_markers
        );
    }

    #[test]
    fn test_line_numbered_code_blocks() {
        let dir = tempdir().unwrap();
        let base_path = dir.path();
        let file_path = base_path.join("lib.rs");
        let output_path = base_path.join("out.md");

        // Create a multi-line Rust file
        fs::write(
                    &file_path,
                    "fn add(a: i32, b: i32) -> i32 {\n    a + b\n}\n\nfn main() {\n    println!(\"{}\", add(1, 2));\n}\n",
                )
                .unwrap();

        let mut output = fs::File::create(&output_path).unwrap();
        process_file(base_path, &file_path, &mut output, true).unwrap();

        let content = fs::read_to_string(&output_path).unwrap();

        // Check language and line numbers prefix
        assert!(content.contains("```rust"));
        assert!(content.contains("   1 | "));
        assert!(content.contains("   2 | "));

        // Count lines with " | " prefix equals number of lines in original file
        let numbered_lines = content
            .lines()
            .filter(|l| {
                l.trim_start()
                    .chars()
                    .next()
                    .map(|c| c.is_ascii_digit())
                    .unwrap_or(false)
                    && l.contains(" | ")
            })
            .count();
        let original_line_count = fs::read_to_string(&file_path).unwrap().lines().count();
        assert_eq!(numbered_lines, original_line_count);

        // Ensure code fence closes
        assert!(content.contains("```"));
    }

    #[test]
    fn test_binary_file_handling() {
        let dir = tempdir().unwrap();
        let base_path = dir.path();
        let file_path = base_path.join("image.bin");
        let output_path = base_path.join("out.md");

        // Write some non-UTF8 bytes
        let bytes = vec![0u8, 159, 146, 150, 255, 0, 1, 2];
        fs::write(&file_path, bytes).unwrap();

        let mut output = fs::File::create(&output_path).unwrap();
        process_file(base_path, &file_path, &mut output, false).unwrap();

        let content = fs::read_to_string(&output_path).unwrap();

        // Expect a text block fallback with a helpful message
        assert!(content.contains("```text"));
        assert!(
            content
                .contains("<Could not read file content (e.g., binary file or permission error)>")
        );

        // Ensure the code block is closed
        let fence_count = content.matches("```").count();
        assert!(
            fence_count >= 2,
            "expected at least opening and closing fences, got {}",
            fence_count
        );
    }
}
