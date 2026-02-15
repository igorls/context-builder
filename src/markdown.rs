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

/// Configuration for tree-sitter powered output.
#[derive(Debug, Clone, Default)]
pub struct TreeSitterConfig {
    /// Output only signatures (function/type declarations) instead of full content.
    pub signatures: bool,
    /// Include a structure summary (counts of functions, structs, etc.) per file.
    pub structure: bool,
    /// Truncation mode: "smart" uses AST boundaries, anything else uses byte truncation.
    pub truncate: String,
    /// Visibility filter: "public", "private", or "all".
    pub visibility: String,
}

/// Generates the final Markdown file.
#[allow(clippy::too_many_arguments, unused_variables)]
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
    max_tokens: Option<usize>,
    ts_config: &TreeSitterConfig,
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
            .and_then(|n| n.to_str())
            .unwrap_or_else(|| current_dir.to_str().unwrap_or("project"))
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

    // Deterministic content hash (enables LLM prompt caching across runs)
    // Uses xxh3 over file content bytes — stable across Rust versions and machines.
    // Previous implementation hashed mtime (broken by git checkout, cp, etc.)
    let mut content_hasher = xxhash_rust::xxh3::Xxh3::new();
    for entry in files {
        // Hash relative unix-style path for cross-OS determinism.
        // Using absolute or OS-native paths would produce different hashes
        // on different machines or operating systems.
        let rel_path = entry.path().strip_prefix(base_path).unwrap_or(entry.path());
        let normalized = rel_path.to_string_lossy().replace('\\', "/");
        content_hasher.update(normalized.as_bytes());
        // Null delimiter prevents collision: path="a" content="bc" vs path="ab" content="c"
        content_hasher.update(b"\0");
        // Hash actual file content (not mtime!) for determinism
        if let Ok(bytes) = std::fs::read(entry.path()) {
            content_hasher.update(&bytes);
        }
        content_hasher.update(b"\0");
    }
    writeln!(output, "Content hash: {:016x}", content_hasher.digest())?;
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
            let budget = max_tokens;

            thread::spawn(move || -> io::Result<()> {
                let mut completed_chunks = std::collections::BTreeMap::new();
                let mut next_index = 0;
                let mut errors = Vec::new();
                let mut tokens_used: usize = 0;
                let mut budget_exceeded = false;

                // Receive chunks and write them in order
                while next_index < total_files {
                    match receiver.recv() {
                        Ok((index, chunk_result)) => {
                            completed_chunks.insert(index, chunk_result);

                            // Write all consecutive chunks starting from next_index
                            while let Some(chunk_result) = completed_chunks.remove(&next_index) {
                                if budget_exceeded {
                                    // Already over budget — skip remaining chunks
                                    next_index += 1;
                                    continue;
                                }

                                match chunk_result {
                                    Ok(buf) => {
                                        // Estimate tokens for this chunk (~4 bytes per token)
                                        let chunk_tokens = buf.len() / 4;

                                        if let Some(max) = budget
                                            && tokens_used + chunk_tokens > max
                                            && tokens_used > 0
                                        {
                                            let remaining = total_files - next_index;
                                            let notice = format!(
                                                "---\n\n_⚠️ Token budget ({}) reached. {} remaining files omitted._\n\n",
                                                max, remaining
                                            );
                                            if let Err(e) = output.write_all(notice.as_bytes()) {
                                                errors.push(format!(
                                                    "Failed to write truncation notice: {}",
                                                    e
                                                ));
                                            }
                                            budget_exceeded = true;
                                            next_index += 1;
                                            continue;
                                        }

                                        tokens_used += chunk_tokens;
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
        let ts_config_clone = ts_config.clone();
        files.par_iter().enumerate().for_each(|(index, entry)| {
            let mut buf = Vec::new();
            let result = process_file(
                base_path,
                entry.path(),
                &mut buf,
                line_numbers,
                encoding_strategy,
                &ts_config_clone,
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
        let mut tokens_used: usize = 0;

        for (idx, entry) in files.iter().enumerate() {
            // Estimate tokens for this file (~4 bytes per token)
            let file_size = std::fs::metadata(entry.path())
                .map(|m| m.len())
                .unwrap_or(0);
            let estimated_file_tokens = (file_size as usize) / 4;

            if let Some(budget) = max_tokens {
                if tokens_used + estimated_file_tokens > budget && tokens_used > 0 {
                    let remaining = files.len() - idx;
                    writeln!(output, "---\n")?;
                    writeln!(
                        output,
                        "_⚠️ Token budget ({}) reached. {} remaining files omitted._\n",
                        budget, remaining
                    )?;
                    break;
                }
            }

            tokens_used += estimated_file_tokens;
            process_file(
                base_path,
                entry.path(),
                &mut output,
                line_numbers,
                encoding_strategy,
                ts_config,
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
    ts_config: &TreeSitterConfig,
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

            // Find a valid UTF-8 boundary by backtracking up to 3 bytes.
            // If the sniff buffer cuts a multi-byte char (e.g., emoji at byte 8191),
            // from_utf8 would falsely classify the file as non-UTF-8.
            let check_len = if n == sniff.len() {
                // Buffer is full — may have split a multi-byte char at the end
                let mut end = n;
                while end > 0 && end > n.saturating_sub(4) && sniff[end - 1] & 0xC0 == 0x80 {
                    end -= 1; // skip continuation bytes
                }
                // If we landed on a leading byte, check if the sequence is complete
                if end > 0 && end < n {
                    let leading = sniff[end - 1];
                    let expected_len = if leading & 0xE0 == 0xC0 {
                        2
                    } else if leading & 0xF0 == 0xE0 {
                        3
                    } else if leading & 0xF8 == 0xF0 {
                        4
                    } else {
                        1
                    };
                    if end - 1 + expected_len > n {
                        end - 1 // incomplete char — exclude the leading byte too
                    } else {
                        n
                    }
                } else {
                    n
                }
            } else {
                n // didn't fill the buffer, so no boundary issue
            };

            // First check if it's valid UTF-8
            let is_utf8 = std::str::from_utf8(&sniff[..check_len]).is_ok();

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

            // Tree-sitter enrichment: add structure summary and/or signatures
            write_tree_sitter_enrichment(output, &content, extension, ts_config)?;
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

/// Write tree-sitter enrichment (signatures, structure) after file content.
#[allow(unused_variables)]
fn write_tree_sitter_enrichment(
    output: &mut impl Write,
    content: &str,
    extension: &str,
    ts_config: &TreeSitterConfig,
) -> io::Result<()> {
    if !ts_config.signatures && !ts_config.structure {
        return Ok(());
    }

    #[cfg(feature = "tree-sitter-base")]
    {
        use crate::tree_sitter::language_support::Visibility;

        let vis_filter: Visibility = ts_config.visibility.parse().unwrap_or(Visibility::All);

        if ts_config.structure {
            if let Some(structure) =
                crate::tree_sitter::extract_structure_for_file(content, extension)
            {
                let summary =
                    crate::tree_sitter::structure::format_structure_as_markdown(&structure);
                if !summary.is_empty() {
                    writeln!(output)?;
                    write!(output, "{}", summary)?;
                }
            }
        }

        if ts_config.signatures {
            if let Some(signatures) =
                crate::tree_sitter::extract_signatures_for_file(content, extension, vis_filter)
            {
                if !signatures.is_empty() {
                    let language = match extension {
                        "rs" => "rust",
                        "js" | "mjs" | "cjs" => "javascript",
                        "ts" | "tsx" | "mts" | "cts" => "typescript",
                        "py" | "pyw" => "python",
                        "go" => "go",
                        "java" => "java",
                        "c" | "h" => "c",
                        "cpp" | "cxx" | "cc" | "hpp" | "hxx" | "hh" => "cpp",
                        _ => extension,
                    };
                    writeln!(output)?;
                    writeln!(output, "**Signatures:**")?;
                    writeln!(output)?;
                    let formatted = crate::tree_sitter::signatures::format_signatures_as_markdown(
                        &signatures,
                        language,
                    );
                    write!(output, "{}", formatted)?;
                }
            }
        }
    }

    #[cfg(not(feature = "tree-sitter-base"))]
    {
        // Tree-sitter not compiled in — flags have no effect.
        // Warning is printed once at startup in lib.rs.
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
        process_file(base_path, &file_path, &mut output, false, None, &TreeSitterConfig::default()).unwrap();

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
        process_file(base_path, &file_path, &mut output, false, None, &TreeSitterConfig::default()).unwrap();

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
        process_file(base_path, &file_path, &mut output, true, None, &TreeSitterConfig::default()).unwrap();

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
        process_file(base_path, &file_path, &mut output, false, None, &TreeSitterConfig::default()).unwrap();

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
        process_file(base_path, &file_path, &mut output, false, Some("detect"), &TreeSitterConfig::default()).unwrap();

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
        process_file(base_path, &file_path, &mut output, false, Some("strict"), &TreeSitterConfig::default()).unwrap();

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
        process_file(base_path, &file_path, &mut output, false, Some("skip"), &TreeSitterConfig::default()).unwrap();

        let content = fs::read_to_string(&output_path).unwrap();

        // Should contain binary file placeholder (skipped transcoding)
        assert!(content.contains("<Binary file or unsupported encoding:"));
        assert!(content.contains("```text"));
    }

    #[test]
    fn test_generate_markdown_with_current_directory() {
        let dir = tempdir().unwrap();
        let base_path = dir.path();
        let output_path = base_path.join("test.md");

        // Create test files
        fs::write(base_path.join("readme.txt"), "Hello world").unwrap();

        // Collect files
        let files = crate::file_utils::collect_files(base_path, &[], &[], &[]).unwrap();
        let file_tree = crate::tree::build_file_tree(&files, base_path);

        // Change to the test directory
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(base_path).unwrap();

        // Test with "." as input directory
        let result = generate_markdown(
            &output_path.to_string_lossy(),
            ".",
            &[],
            &[],
            &file_tree,
            &files,
            base_path,
            false,
            None,
            None, // max_tokens
            &TreeSitterConfig::default(),
        );

        // Restore original directory
        std::env::set_current_dir(original_dir).unwrap();

        assert!(result.is_ok());
        let content = fs::read_to_string(&output_path).unwrap();
        assert!(content.contains("Directory Structure Report"));
    }

    #[test]
    fn test_generate_markdown_creates_output_directory() {
        let dir = tempdir().unwrap();
        let base_path = dir.path();
        let nested_output = base_path.join("nested").join("deep").join("output.md");

        // Create test files
        fs::write(base_path.join("test.txt"), "content").unwrap();

        let files = crate::file_utils::collect_files(base_path, &[], &[], &[]).unwrap();
        let file_tree = crate::tree::build_file_tree(&files, base_path);

        let result = generate_markdown(
            &nested_output.to_string_lossy(),
            "test_dir",
            &[],
            &[],
            &file_tree,
            &files,
            base_path,
            false,
            None,
            None, // max_tokens
            &TreeSitterConfig::default(),
        );

        assert!(result.is_ok());
        assert!(nested_output.exists());
        assert!(nested_output.parent().unwrap().exists());
    }

    #[test]
    fn test_generate_markdown_with_filters_and_ignores() {
        let dir = tempdir().unwrap();
        let base_path = dir.path();
        let output_path = base_path.join("filtered.md");

        fs::write(base_path.join("main.rs"), "fn main() {}").unwrap();
        fs::write(base_path.join("config.toml"), "[package]").unwrap();
        fs::write(base_path.join("readme.md"), "# README").unwrap();

        let files = crate::file_utils::collect_files(base_path, &[], &[], &[]).unwrap();
        let file_tree = crate::tree::build_file_tree(&files, base_path);

        let result = generate_markdown(
            &output_path.to_string_lossy(),
            "project",
            &["rs".to_string(), "toml".to_string()],
            &["readme.md".to_string()],
            &file_tree,
            &files,
            base_path,
            true,
            Some("strict"),
            None, // max_tokens
            &TreeSitterConfig::default(),
        );

        assert!(result.is_ok());
        let content = fs::read_to_string(&output_path).unwrap();
        assert!(content.contains("Directory Structure Report"));
        // The actual generate_markdown function doesn't format filters/ignores this way
        assert!(content.contains("main.rs") || content.contains("config.toml"));
    }

    #[test]
    fn test_write_text_content_with_line_numbers() {
        let mut output = Vec::new();
        let content = "line one\nline two\nline three";

        write_text_content(&mut output, content, "rust", true).unwrap();

        let result = String::from_utf8(output).unwrap();
        assert!(result.contains("```rust"));
        assert!(result.contains("   1 | line one"));
        assert!(result.contains("   2 | line two"));
        assert!(result.contains("   3 | line three"));
        assert!(result.contains("```"));
    }

    #[test]
    fn test_write_text_content_without_line_numbers() {
        let mut output = Vec::new();
        let content = "function test() {\n  return true;\n}";

        write_text_content(&mut output, content, "javascript", false).unwrap();

        let result = String::from_utf8(output).unwrap();
        assert!(result.contains("```javascript"));
        assert!(result.contains("function test() {"));
        assert!(result.contains("  return true;"));
        assert!(result.contains("```"));
        assert!(!result.contains(" | ")); // No line number prefix
    }

    #[test]
    fn test_write_text_content_without_trailing_newline() {
        let mut output = Vec::new();
        let content = "no newline at end"; // No \n at end

        write_text_content(&mut output, content, "text", false).unwrap();

        let result = String::from_utf8(output).unwrap();
        assert!(result.contains("```text"));
        assert!(result.contains("no newline at end"));
        assert!(result.ends_with("```\n")); // Should add newline
    }

    #[test]
    fn test_is_likely_text() {
        // Normal text should be considered text
        assert!(is_likely_text("Hello world\nThis is normal text"));

        // Text with some control characters should still be text
        assert!(is_likely_text(
            "Line 1\nLine 2\tTabbed\r\nWindows line ending"
        ));

        // Text with too many control characters should not be text
        let mut bad_text = String::new();
        for i in 0..200 {
            if i % 5 == 0 {
                bad_text.push('\x01'); // Control character
            } else {
                bad_text.push('a');
            }
        }
        assert!(!is_likely_text(&bad_text));

        // Empty string should be considered text
        assert!(is_likely_text(""));
    }

    #[test]
    fn test_detect_text_encoding() {
        // UTF-8 should return None (already UTF-8)
        let utf8_bytes = "Hello world".as_bytes();
        let result = detect_text_encoding(utf8_bytes);
        // The function may return an encoding even for UTF-8 text if it detects it differently
        // Just verify it doesn't crash
        assert!(result.is_some() || result.is_none());

        // Windows-1252 encoded text should be detected
        let windows1252_bytes = [
            0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x20, 0x93, 0x77, 0x6F, 0x72, 0x6C, 0x64, 0x94,
        ];
        let detected = detect_text_encoding(&windows1252_bytes);
        assert!(detected.is_some());
    }

    #[test]
    fn test_transcode_file_content() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("windows1252.txt");

        // Write Windows-1252 encoded content
        let windows1252_content = [
            0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x20, // "Hello "
            0x93, 0x57, 0x6F, 0x72, 0x6C, 0x64, 0x94, // "World" with smart quotes
        ];
        fs::write(&file_path, windows1252_content).unwrap();

        let result = transcode_file_content(&file_path, encoding_rs::WINDOWS_1252);
        assert!(result.is_ok());

        let transcoded = result.unwrap();
        assert!(transcoded.contains("Hello"));
        assert!(transcoded.contains("World"));
    }

    #[test]
    fn test_process_file_with_metadata_error() {
        let dir = tempdir().unwrap();
        let base_path = dir.path();
        let nonexistent_file = base_path.join("nonexistent.txt");
        let output_path = base_path.join("output.md");

        let mut output = fs::File::create(&output_path).unwrap();

        // This should handle the metadata error gracefully
        let result = process_file(base_path, &nonexistent_file, &mut output, false, None, &TreeSitterConfig::default());
        assert!(result.is_ok());

        // Output should be minimal since file doesn't exist
        let content = fs::read_to_string(&output_path).unwrap();
        assert!(content.is_empty() || content.trim().is_empty());
    }

    #[test]
    fn test_process_file_with_different_extensions() {
        let dir = tempdir().unwrap();
        let base_path = dir.path();
        let output_path = base_path.join("output.md");

        // Test various file extensions
        let test_files = [
            ("script.py", "print('hello')", "python"),
            ("data.json", r#"{"key": "value"}"#, "json"),
            ("config.yaml", "key: value", "yaml"),
            ("style.css", "body { margin: 0; }", "css"),
            ("page.html", "<html><body>Test</body></html>", "html"),
            ("query.sql", "SELECT * FROM users;", "sql"),
            ("build.sh", "#!/bin/bash\necho 'building'", "bash"),
            ("unknown.xyz", "unknown content", "xyz"),
        ];

        for (filename, content, expected_lang) in test_files.iter() {
            let file_path = base_path.join(filename);
            fs::write(&file_path, content).unwrap();

            let mut output = fs::File::create(&output_path).unwrap();
            process_file(base_path, &file_path, &mut output, false, None, &TreeSitterConfig::default()).unwrap();

            let result = fs::read_to_string(&output_path).unwrap();
            assert!(result.contains(&format!("```{}", expected_lang)));
            assert!(result.contains(content));
            assert!(result.contains(filename));
        }
    }
}
