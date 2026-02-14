//! Integration tests for binary file handling in auto-diff mode
//!
//! This test ensures that the application doesn't crash when encountering
//! binary files during auto-diff processing.

use std::fs;
use std::path::Path;
use tempfile::tempdir;

use context_builder::config::Config;
use context_builder::{Prompter, cli::Args, run_with_args};

struct TestPrompter {
    overwrite_response: bool,
    processing_response: bool,
}

impl TestPrompter {
    fn new(overwrite_response: bool, processing_response: bool) -> Self {
        Self {
            overwrite_response,
            processing_response,
        }
    }
}

impl Prompter for TestPrompter {
    fn confirm_processing(&self, _file_count: usize) -> std::io::Result<bool> {
        Ok(self.processing_response)
    }

    fn confirm_overwrite(&self, _file_path: &str) -> std::io::Result<bool> {
        Ok(self.overwrite_response)
    }
}

fn write_file(path: &Path, contents: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(path, contents).unwrap();
}

fn write_binary_file(path: &Path, data: &[u8]) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(path, data).unwrap();
}

#[test]
fn test_binary_files_dont_crash_autodiff() {
    let temp_dir = tempdir().unwrap();
    let root = temp_dir.path();

    // Create text files
    write_file(
        &root.join("src/main.rs"),
        "fn main() { println!(\"Hello\"); }",
    );
    write_file(&root.join("README.md"), "# Test Project");

    // Create binary files with various problematic byte sequences
    write_binary_file(
        &root.join("assets/image.png"),
        &[
            0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, // PNG header
            0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44, 0x52, 0xFF, 0xFE, 0xFD, 0xFC, 0x00, 0x01,
            0x02, 0x03, // Random binary data
        ],
    );

    // Create a file with null bytes
    write_binary_file(
        &root.join("data/binary.dat"),
        &[
            0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85,
            0x86, 0x87,
        ],
    );

    // Create a file with invalid UTF-8 sequences
    write_binary_file(
        &root.join("config/settings.bin"),
        &[
            0xC0, 0x80, // Invalid UTF-8: overlong encoding
            0xE0, 0x80, 0x80, // Invalid UTF-8: overlong encoding
            0xFF, 0xFE, 0xFF, 0xFE, // Invalid UTF-8: not valid start bytes
        ],
    );

    let output_path = root.join("output.md");

    // Configure for auto-diff mode
    let config = Config {
        auto_diff: Some(true),
        diff_context_lines: Some(3),
        ..Default::default()
    };

    let args = Args {
        input: root.to_string_lossy().into_owned(),
        output: output_path.to_string_lossy().into_owned(),
        filter: vec![], // Include all file types to catch binary files
        ignore: vec![],
        preview: false,
        token_count: false,
        line_numbers: false,
        yes: true, // Auto-confirm to avoid prompts
        diff_only: false,
        clear_cache: false,
        init: false,
        max_tokens: None,
    };

    let prompter = TestPrompter::new(true, true);

    // First run - should create initial state without crashing
    let result1 = run_with_args(args.clone(), config.clone(), &prompter);
    assert!(
        result1.is_ok(),
        "First run with binary files should not crash: {:?}",
        result1
    );

    // Verify output file was created
    assert!(
        output_path.exists(),
        "Output file should be created on first run"
    );

    // Modify a text file to trigger diff on second run
    write_file(
        &root.join("src/main.rs"),
        "fn main() { println!(\"Hello, world!\"); }",
    );

    // Second run - should handle binary files in diff without crashing
    let result2 = run_with_args(args, config, &prompter);
    assert!(
        result2.is_ok(),
        "Second run with binary files should not crash during diff: {:?}",
        result2
    );

    // Read the output to verify it contains appropriate handling of binary files
    let output_content = fs::read_to_string(&output_path).unwrap();

    // Should contain the modified text file
    assert!(
        output_content.contains("Hello, world!"),
        "Output should contain modified text content"
    );

    // Binary files should be represented appropriately (not causing crashes)
    // The exact representation depends on implementation but should not crash
    assert!(
        output_content.len() > 100,
        "Output should contain substantial content indicating successful processing"
    );
}

#[test]
fn test_mixed_text_and_binary_files_autodiff() {
    let temp_dir = tempdir().unwrap();
    let root = temp_dir.path();

    // Create a mix of text and binary files
    write_file(&root.join("source.txt"), "Original text content");
    write_binary_file(&root.join("data.bin"), &[0x00, 0xFF, 0x42, 0x13, 0x37]);
    write_file(&root.join("config.json"), r#"{"version": "1.0"}"#);

    let output_path = root.join("mixed_output.md");

    let config = Config {
        auto_diff: Some(true),
        ..Default::default()
    };

    let args = Args {
        input: root.to_string_lossy().into_owned(),
        output: output_path.to_string_lossy().into_owned(),
        filter: vec![],
        ignore: vec![],
        preview: false,
        token_count: false,
        line_numbers: false,
        yes: true,
        diff_only: false,
        clear_cache: false,
        init: false,
        max_tokens: None,
    };

    let prompter = TestPrompter::new(true, true);

    // Initial run
    let result1 = run_with_args(args.clone(), config.clone(), &prompter);
    assert!(result1.is_ok(), "Initial run should succeed");

    // Modify text file and add another binary file
    write_file(&root.join("source.txt"), "Modified text content");
    write_binary_file(
        &root.join("image.jpg"),
        &[
            0xFF, 0xD8, 0xFF, 0xE0, // JPEG header
            0x00, 0x10, 0x4A, 0x46, 0x49, 0x46,
        ],
    );

    // Second run with changes
    let result2 = run_with_args(args, config, &prompter);
    assert!(
        result2.is_ok(),
        "Second run with mixed file changes should succeed"
    );

    let output_content = fs::read_to_string(&output_path).unwrap();
    assert!(
        output_content.contains("Modified text content"),
        "Should show updated text content"
    );
}

#[test]
fn test_large_binary_file_autodiff() {
    let temp_dir = tempdir().unwrap();
    let root = temp_dir.path();

    // Create a large binary file (simulating real-world scenario)
    let large_binary_data: Vec<u8> = (0..10000).map(|i| (i % 256) as u8).collect();

    write_binary_file(&root.join("large_binary.dat"), &large_binary_data);
    write_file(&root.join("small_text.txt"), "Small text file");

    let output_path = root.join("large_binary_output.md");

    let config = Config {
        auto_diff: Some(true),
        ..Default::default()
    };

    let args = Args {
        input: root.to_string_lossy().into_owned(),
        output: output_path.to_string_lossy().into_owned(),
        filter: vec![],
        ignore: vec![],
        preview: false,
        token_count: false,
        line_numbers: false,
        yes: true,
        diff_only: false,
        clear_cache: false,
        init: false,
        max_tokens: None,
    };

    let prompter = TestPrompter::new(true, true);

    // Should handle large binary files without memory issues or crashes
    let result = run_with_args(args, config, &prompter);
    assert!(
        result.is_ok(),
        "Should handle large binary files without crashing: {:?}",
        result
    );

    assert!(
        output_path.exists(),
        "Output should be created even with large binary files"
    );
}
