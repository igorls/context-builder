use chrono::Utc;
use ignore::DirEntry;
use log::{error, info, warn};
use std::fs;
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::path::Path;

use crate::tree::{FileTree, write_tree_to_file};
use encoding_rs::{Encoding, UTF_8};

#[cfg(feature = "parallel")]
use crossbeam_channel::{Receiver, Sender, bounded};
#[cfg(feature = "parallel")]
use std::thread;

/// Generates the final Markdown file.
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
    encoding_strategy: Option<&str>,
) -> io::Result<()> {
    if let Some(parent) = Path::new(output_path).parent()
        && !parent.exists()
    {
        fs::create_dir_all(parent)?;
    }

    let mut output = fs::File::create(output_path)?;

    let input_dir_name = if input_dir == "." {
        let current_dir = std::env::current_dir()?;
        current_dir
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
    } else {
        input_dir.to_string()
    };

    // --- Header --- //
    writeln!(output, "# Directory Structure Report\n")?;

    if !filters.is_empty() {
        writeln!(
            output,
            "This document contains files from the `{}` directory with extensions: {}",
            input_dir_name,
            filters.join(", ")
        )?;
    } else {
        writeln!(
            output,
            "This document contains all files from the `{}` directory, optimized for LLM consumption.",
            input_dir_name
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

    writeln!(output)?;

    // (No '## Files' heading here; it will be injected later only once during final composition)
    // (Diff section will be conditionally inserted later by the auto_diff logic in lib.rs)

    #[cfg(feature = "parallel")]
    {
        use rayon::prelude::*;

        // Create a bounded channel for ordered chunks
        type ChunkResult = (usize, io::Result<Vec<u8>>);
        let (sender, receiver): (Sender<ChunkResult>, Receiver<ChunkResult>) =
            bounded(num_cpus::get() * 2); // Buffer size based on CPU count

        let writer_handle = {
            let mut output = output;
            let total_files = files.len();

            thread::spawn(move || -> io::Result<()> {
                let mut completed_chunks = std::collections::BTreeMap::new();
                let mut next_index = 0;
                let mut errors = Vec::new();

                // Receive chunks and write them in order
                while next_index < total_files {
                    match receiver.recv() {
                        Ok((index, chunk_result)) => {
                            completed_chunks.insert(index, chunk_result);

                            // Write all consecutive chunks starting from next_index
                            while let Some(chunk_result) = completed_chunks.remove(&next_index) {
                                match chunk_result {
                                    Ok(buf) => {
                                        if let Err(e) = output.write_all(&buf) {
                                            errors.push(format!(
                                                "Failed to write output for file index {}: {}",
                                                next_index, e
                                            ));
                                        }
                                    }
                                    Err(e) => {
                                        errors.push(format!(
                                            "Failed to process file index {}: {}",
                                            next_index, e
                                        ));
                                    }
                                }
                                next_index += 1;
                            }
                        }
                        Err(_) => break, // Channel closed
                    }
                }

                if !errors.is_empty() {
                    error!(
                        "Encountered {} errors during parallel processing:",
                        errors.len()
                    );
                    for err in &errors {
                        error!("  {}", err);
                    }
                    return Err(std::io::Error::other(format!(
                        "Failed to process {} files: {}",
                        errors.len(),
                        errors.join("; ")
                    )));
                }

                Ok(())
            })
        };

        // Process files in parallel and send results to writer
        files.par_iter().enumerate().for_each(|(index, entry)| {
            let mut buf = Vec::new();
            let result = process_file(
                base_path,
                entry.path(),
                &mut buf,
                line_numbers,
                encoding_strategy,
            )
            .map(|_| buf);

            // Send result to writer thread (ignore send errors - channel might be closed)
            let _ = sender.send((index, result));
        });

        // Close the sender to signal completion
        drop(sender);

        // Wait for writer thread to complete and propagate any errors
        writer_handle
            .join()
            .map_err(|_| std::io::Error::other("Writer thread panicked"))??;
    }

    #[cfg(not(feature = "parallel"))]
    {
        for entry in files {
            process_file(
                base_path,
                entry.path(),
                &mut output,
                line_numbers,
                encoding_strategy,
            )?;
        }
    }

    Ok(())
}

/// Processes a single file and writes its content to the output.
pub fn process_file(
    base_path: &Path,

    file_path: &Path,

    output: &mut impl Write,
    line_numbers: bool,
    encoding_strategy: Option<&str>,
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

    writeln!(output)?;
    writeln!(output, "### File: `{}`", relative_path.display())?;

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

    // Enhanced binary file handling with encoding detection and transcoding
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

            // First check if it's valid UTF-8
            let is_utf8 = std::str::from_utf8(slice).is_ok();

            if is_utf8 && !slice.contains(&0) {
                // Valid UTF-8 text file - proceed normally
            } else {
                // Try encoding detection for non-UTF-8 files
                // If it's not UTF-8, try to detect the encoding
                let (encoding, _consumed) =
                    encoding_rs::Encoding::for_bom(slice).unwrap_or((encoding_rs::UTF_8, 0));

                // If it's not UTF-8, try to detect the encoding
                let detected_encoding = if encoding == UTF_8 {
                    // Use chardet-like detection for common encodings
                    detect_text_encoding(slice)
                } else {
                    Some(encoding)
                };

                match detected_encoding {
                    Some(enc) if enc != UTF_8 => {
                        let strategy = encoding_strategy.unwrap_or("detect");
                        match strategy {
                            "strict" | "skip" => {
                                // Skip files with non-UTF-8 encoding
                                warn!(
                                    "Skipping non-UTF-8 file {} (encoding: {}, strategy: {})",
                                    relative_path.display(),
                                    enc.name(),
                                    strategy
                                );
                            }
                            _ => {
                                // Default "detect" strategy: attempt to transcode
                                match transcode_file_content(file_path, enc) {
                                    Ok(transcoded_content) => {
                                        info!(
                                            "Successfully transcoded {} from {} to UTF-8",
                                            relative_path.display(),
                                            enc.name()
                                        );
                                        write_text_content(
                                            output,
                                            &transcoded_content,
                                            language,
                                            line_numbers,
                                        )?;
                                        return Ok(());
                                    }
                                    Err(e) => {
                                        warn!(
                                            "Failed to transcode {} from {}: {}. Treating as binary.",
                                            relative_path.display(),
                                            enc.name(),
                                            e
                                        );
                                    }
                                }
                            }
                        }
                    }
                    _ => {
                        // Check if it's likely binary (contains null bytes)
                        if slice.contains(&0) {
                            warn!(
                                "Detected binary file {} (contains null bytes). Skipping content.",
                                relative_path.display()
                            );
                        } else {
                            warn!(
                                "Could not determine encoding for {}. Treating as binary.",
                                relative_path.display()
                            );
                        }
                    }
                }

                // Fallback to binary file placeholder
                writeln!(output, "```text")?;
                writeln!(
                    output,
                    "<Binary file or unsupported encoding: {} bytes>",
                    metadata.len()
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

            // Stream UTF-8 content
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

            let content = match std::fs::read_to_string(file_path) {
                Ok(content) => content,
                Err(e) => {
                    warn!(
                        "Error reading file {}: {}. Output may be truncated.",
                        relative_path.display(),
                        e
                    );
                    writeln!(output, "```text")?;
                    writeln!(output, "<Error reading file content>")?;
                    writeln!(output, "```")?;
                    return Ok(());
                }
            };

            write_text_content(output, &content, language, line_numbers)?;
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

/// Detect text encoding using heuristics for common encodings
fn detect_text_encoding(bytes: &[u8]) -> Option<&'static Encoding> {
    // Try common encodings
    let encodings = [
        encoding_rs::WINDOWS_1252,
        encoding_rs::UTF_16LE,
        encoding_rs::UTF_16BE,
        encoding_rs::SHIFT_JIS,
    ];

    for encoding in &encodings {
        let (decoded, _, had_errors) = encoding.decode(bytes);
        if !had_errors && is_likely_text(&decoded) {
            return Some(encoding);
        }
    }

    None
}

/// Check if decoded content looks like text (no control characters except common ones)
fn is_likely_text(content: &str) -> bool {
    let mut control_chars = 0;
    let mut total_chars = 0;

    for ch in content.chars() {
        total_chars += 1;
        if ch.is_control() && ch != '\n' && ch != '\r' && ch != '\t' {
            control_chars += 1;
        }

        // If more than 5% control characters, probably not text
        if total_chars > 100 && control_chars * 20 > total_chars {
            return false;
        }
    }

    // Allow up to 5% control characters in small files
    if total_chars > 0 {
        control_chars * 20 <= total_chars
    } else {
        true
    }
}

/// Transcode file content from detected encoding to UTF-8
fn transcode_file_content(file_path: &Path, encoding: &'static Encoding) -> io::Result<String> {
    let bytes = std::fs::read(file_path)?;
    let (decoded, _, had_errors) = encoding.decode(&bytes);

    if had_errors {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Failed to decode file with encoding {}", encoding.name()),
        ));
    }

    Ok(decoded.into_owned())
}

/// Write text content with optional line numbers
fn write_text_content(
    output: &mut impl Write,
    content: &str,
    language: &str,
    line_numbers: bool,
) -> io::Result<()> {
    writeln!(output, "```{}", language)?;

    if line_numbers {
        for (i, line) in content.lines().enumerate() {
            writeln!(output, "{:>4} | {}", i + 1, line)?;
        }
    } else {
        output.write_all(content.as_bytes())?;
        if !content.ends_with('\n') {
            writeln!(output)?;
        }
    }

    writeln!(output, "```")?;
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

        // Create an output file
        let mut output = fs::File::create(&output_path).unwrap();

        // Process the file
        process_file(base_path, &file_path, &mut output, false, None).unwrap();

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

        // Create a test Markdown file
        fs::write(&file_path, "# Test\n\nThis is a test markdown file.").unwrap();

        // Create an output file
        let mut output = fs::File::create(&output_path).unwrap();

        // Process the file
        process_file(base_path, &file_path, &mut output, false, None).unwrap();

        // Read the output
        let content = fs::read_to_string(&output_path).unwrap();

        // Debug prints the content
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
        process_file(base_path, &file_path, &mut output, true, None).unwrap();

        let content = fs::read_to_string(&output_path).unwrap();

        // Check language and line numbers prefix
        assert!(content.contains("```rust"));
        assert!(content.contains("   1 | "));
        assert!(content.contains("   2 | "));

        // Count lines with "|" prefix equals number of lines in an original file
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

        // Write truly binary data that won't be decoded by encoding detection
        let bytes = vec![
            0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, // PNG header
            0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44, 0x52, // PNG chunk
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, // More binary data
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Null bytes
        ];
        fs::write(&file_path, bytes).unwrap();

        let mut output = fs::File::create(&output_path).unwrap();
        process_file(base_path, &file_path, &mut output, false, None).unwrap();

        let content = fs::read_to_string(&output_path).unwrap();

        // Expect a text block to fall back with a helpful message
        assert!(content.contains("```text"));
        assert!(content.contains("<Binary file or unsupported encoding:"));

        // Ensure the code block is closed
        let fence_count = content.matches("```").count();
        assert!(
            fence_count >= 2,
            "expected at least opening and closing fences, got {}",
            fence_count
        );
    }

    #[test]
    fn test_encoding_detection_and_transcoding() {
        let dir = tempdir().unwrap();
        let base_path = dir.path();
        let output_path = base_path.join("out.md");

        // Test Windows-1252 encoded file (common in Windows)
        let windows1252_content = [
            0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x20, // "Hello "
            0x93, 0x57, 0x6F, 0x72, 0x6C, 0x64, 0x94, // "World" with smart quotes
            0x0A, // newline
        ];
        let file_path = base_path.join("windows1252.txt");
        fs::write(&file_path, windows1252_content).unwrap();

        let mut output = fs::File::create(&output_path).unwrap();
        process_file(base_path, &file_path, &mut output, false, Some("detect")).unwrap();

        let content = fs::read_to_string(&output_path).unwrap();

        // Should contain transcoded content with UTF-8 equivalents
        assert!(content.contains("Hello"));
        assert!(content.contains("World"));
        // Should use text language
        assert!(content.contains("```txt"));

        // Ensure the code block is closed
        let fence_count = content.matches("```").count();
        assert!(
            fence_count >= 2,
            "expected at least opening and closing fences, got {}",
            fence_count
        );
    }

    #[test]
    fn test_encoding_strategy_strict() {
        let dir = tempdir().unwrap();
        let base_path = dir.path();
        let output_path = base_path.join("out.md");

        // Create a file with non-UTF-8 content
        let non_utf8_content = [0xFF, 0xFE, 0x41, 0x00]; // UTF-16 LE BOM + "A"
        let file_path = base_path.join("utf16.txt");
        fs::write(&file_path, non_utf8_content).unwrap();

        let mut output = fs::File::create(&output_path).unwrap();
        process_file(base_path, &file_path, &mut output, false, Some("strict")).unwrap();

        let content = fs::read_to_string(&output_path).unwrap();

        // Should contain binary file placeholder
        assert!(content.contains("<Binary file or unsupported encoding:"));
        assert!(content.contains("```text"));

        // Ensure the code block is closed
        let fence_count = content.matches("```").count();
        assert!(
            fence_count >= 2,
            "expected at least opening and closing fences, got {}",
            fence_count
        );
    }

    #[test]
    fn test_encoding_strategy_skip() {
        let dir = tempdir().unwrap();
        let base_path = dir.path();
        let output_path = base_path.join("out.md");

        // Create a file with UTF-16 content
        let utf16_content = [0xFF, 0xFE, 0x48, 0x00, 0x69, 0x00]; // UTF-16 LE "Hi"
        let file_path = base_path.join("utf16.txt");
        fs::write(&file_path, utf16_content).unwrap();

        let mut output = fs::File::create(&output_path).unwrap();
        process_file(base_path, &file_path, &mut output, false, Some("skip")).unwrap();

        let content = fs::read_to_string(&output_path).unwrap();

        // Should contain binary file placeholder (skipped transcoding)
        assert!(content.contains("<Binary file or unsupported encoding:"));
        assert!(content.contains("```text"));
    }
}
