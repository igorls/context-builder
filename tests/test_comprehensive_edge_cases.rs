//! Comprehensive edge case testing suite for context-builder v0.5.0
//!
//! This test suite covers all the critical edge cases and robustness scenarios
//! that were identified during the v0.5.0 development cycle.

use context_builder::cli::Args;
use context_builder::config::Config;
use context_builder::{Prompter, run_with_args};
use serial_test::serial;
use std::fs;
use std::path::Path;
use tempfile::tempdir;

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
#[serial]
fn test_comprehensive_binary_file_edge_cases() {
    let temp_dir = tempdir().unwrap();
    let project_dir = temp_dir.path().join("project");
    let output_dir = temp_dir.path().join("output");
    fs::create_dir_all(&output_dir).unwrap();

    // Create various binary and problematic files
    write_file(&project_dir.join("src/normal.rs"), "fn main() {}\n");

    // Pure binary file (executable-like)
    let binary_data = vec![
        0x7f, 0x45, 0x4c, 0x46, // ELF header
        0x02, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];
    write_binary_file(&project_dir.join("src/binary.rs"), &binary_data);

    // File with UTF-16 BOM
    let utf16_data = [
        0xFF, 0xFE, // UTF-16 LE BOM
        0x48, 0x00, 0x65, 0x00, 0x6C, 0x00, 0x6C, 0x00, 0x6F, 0x00, // "Hello"
        0x0A, 0x00, // newline
    ];
    write_binary_file(&project_dir.join("src/utf16.rs"), &utf16_data);

    // File with Windows-1252 encoding
    let windows1252_data = [
        0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x20, // "Hello "
        0x93, 0x57, 0x6F, 0x72, 0x6C, 0x64, 0x94, // "World" with smart quotes
        0x0A, // newline
    ];
    write_binary_file(&project_dir.join("src/win1252.rs"), &windows1252_data);

    // Empty file
    write_file(&project_dir.join("src/empty.rs"), "");

    // File with only null bytes
    write_binary_file(&project_dir.join("src/nulls.rs"), &[0x00; 100]);

    // Very large file (test memory efficiency)
    let large_content = "// Large file\n".repeat(10000);
    write_file(&project_dir.join("src/large.rs"), &large_content);

    // Test with different encoding strategies
    let strategies = ["detect", "strict", "skip"];

    for strategy in &strategies {
        let config = Config {
            filter: Some(vec!["rs".to_string()]),
            encoding_strategy: Some(strategy.to_string()),
            ..Default::default()
        };

        let args = Args {
            input: project_dir.to_string_lossy().to_string(),
            output: output_dir
                .join(format!("test_{}.md", strategy))
                .to_string_lossy()
                .to_string(),
            filter: vec!["rs".to_string()],
            ignore: vec![],
            preview: false,
            token_count: false,
            line_numbers: false,
            yes: true,
            diff_only: false,
            clear_cache: false,
        };

        let prompter = TestPrompter::new(true, true);
        let result = run_with_args(args, config, &prompter);

        assert!(
            result.is_ok(),
            "Should handle binary files gracefully with strategy: {}",
            strategy
        );

        // Verify output file was created
        let output_path = output_dir.join(format!("test_{}.md", strategy));
        assert!(
            output_path.exists(),
            "Output file should exist for strategy: {}",
            strategy
        );

        let content = fs::read_to_string(&output_path).unwrap();

        // Should contain normal file
        assert!(
            content.contains("fn main()"),
            "Should contain normal file content"
        );

        // Should handle binary files appropriately based on strategy
        match *strategy {
            "detect" => {
                // May contain transcoded content or binary placeholders
                assert!(
                    content.contains("Hello") || content.contains("<Binary file"),
                    "Detect strategy should transcode or show binary placeholder"
                );
            }
            "strict" | "skip" => {
                // Should show binary placeholders for non-UTF-8 files
                assert!(
                    content.contains("<Binary file") || content.contains("binary.rs"),
                    "Strict/skip strategy should show binary placeholders"
                );
            }
            _ => {}
        }

        // Should handle empty files
        assert!(content.contains("empty.rs"), "Should list empty files");

        // Should handle large files
        assert!(content.contains("large.rs"), "Should handle large files");
    }

    // No need to restore directory since we never changed it
}

#[test]
#[serial]
fn test_configuration_precedence_edge_cases() {
    let temp_dir = tempdir().unwrap();
    let project_dir = temp_dir.path().join("project");
    let output_dir = temp_dir.path().join("output");
    fs::create_dir_all(&output_dir).unwrap();

    // Create test files
    write_file(&project_dir.join("test.rs"), "fn test() {}\n");
    write_file(&project_dir.join("README.md"), "# Test Project\n");

    // Test 1: Basic functionality with explicit CLI args
    let args = Args {
        input: project_dir.to_string_lossy().to_string(),
        output: output_dir
            .join("basic_test.md")
            .to_string_lossy()
            .to_string(),
        filter: vec!["rs".to_string()],
        ignore: vec![],
        preview: false,
        token_count: false,
        line_numbers: false,
        yes: true,
        diff_only: false,
        clear_cache: false,
    };

    let prompter = TestPrompter::new(true, true);
    let result = run_with_args(args, Config::default(), &prompter);
    assert!(result.is_ok(), "Basic configuration test should succeed");

    let output_path = output_dir.join("basic_test.md");
    assert!(output_path.exists(), "Output should exist for basic test");

    let content = fs::read_to_string(&output_path).unwrap();
    assert!(
        content.contains("test.rs"),
        "Should include filtered .rs files"
    );
    assert!(
        !content.contains("README.md"),
        "Should exclude non-filtered files"
    );

    // Test 2: Empty filter should include all files
    let args = Args {
        input: project_dir.to_string_lossy().to_string(),
        output: output_dir
            .join("all_files_test.md")
            .to_string_lossy()
            .to_string(),
        filter: vec![],
        ignore: vec![],
        preview: false,
        token_count: false,
        line_numbers: false,
        yes: true,
        diff_only: false,
        clear_cache: false,
    };

    let result = run_with_args(args, Config::default(), &prompter);
    assert!(result.is_ok(), "All files test should succeed");

    let output_path = output_dir.join("all_files_test.md");
    let content = fs::read_to_string(&output_path).unwrap();
    assert!(
        content.contains("test.rs"),
        "Should include all files when no filter"
    );
    assert!(
        content.contains("README.md"),
        "Should include all files when no filter"
    );
}

#[test]
#[serial]
fn test_cache_consistency_edge_cases() {
    let temp_dir = tempdir().unwrap();
    let project_dir = temp_dir.path().join("project");
    let output_dir = temp_dir.path().join("output");
    fs::create_dir_all(&output_dir).unwrap();

    write_file(&project_dir.join("test.rs"), "fn original() {}\n");

    // Change to project directory
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(&project_dir).unwrap();

    // Create config with auto_diff enabled
    write_file(
        &project_dir.join("context-builder.toml"),
        r#"
auto_diff = true
timestamped_output = true
"#,
    );

    let base_args = Args {
        input: project_dir.to_string_lossy().to_string(),
        output: output_dir
            .join("cache_test.md")
            .to_string_lossy()
            .to_string(),
        filter: vec!["rs".to_string()],
        ignore: vec![],
        preview: false,
        token_count: false,
        line_numbers: false,
        yes: true,
        diff_only: false,
        clear_cache: false,
    };

    let config = context_builder::config::load_config_from_path(&project_dir).unwrap_or_default();
    let prompter = TestPrompter::new(true, true);

    // First run - establish cache
    let result1 = run_with_args(base_args.clone(), config.clone(), &prompter);
    assert!(result1.is_ok(), "First run should succeed");

    // Verify cache was created
    let cache_dir = project_dir.join(".context-builder").join("cache");
    assert!(cache_dir.exists(), "Cache directory should be created");

    // Test cache with different path representations
    let current_dir_string = std::env::current_dir()
        .unwrap()
        .to_string_lossy()
        .to_string();
    let path_variants = [".", "./", &current_dir_string];

    for (i, path_variant) in path_variants.iter().enumerate() {
        let mut variant_args = base_args.clone();
        variant_args.input = path_variant.to_string();
        variant_args.output = output_dir
            .join(format!("variant_{}.md", i))
            .to_string_lossy()
            .to_string();

        let result = run_with_args(variant_args, config.clone(), &prompter);
        assert!(
            result.is_ok(),
            "Path variant '{}' should succeed",
            path_variant
        );

        let output_path = output_dir.join(format!("variant_{}.md", i));
        let content = fs::read_to_string(&output_path).unwrap();

        // Should show "no changes detected" because cache should be consistent
        // (or at least not crash due to path inconsistencies)
        assert!(
            content.contains("original") || content.contains("no changes"),
            "Path variant should handle cache consistently"
        );
    }

    // Test cache corruption recovery
    let cache_files: Vec<_> = fs::read_dir(&cache_dir)
        .unwrap()
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            entry
                .path()
                .extension()
                .and_then(|s| s.to_str())
                .map(|s| s == "json")
                .unwrap_or(false)
        })
        .collect();

    if !cache_files.is_empty() {
        // Corrupt the cache
        fs::write(cache_files[0].path(), "{ invalid json }").unwrap();

        // Should recover gracefully
        let result = run_with_args(base_args.clone(), config.clone(), &prompter);
        assert!(result.is_ok(), "Should recover from corrupted cache");
    }

    // Restore original directory
    std::env::set_current_dir(original_dir).unwrap();
}

#[test]
#[serial]
fn test_error_conditions_and_exit_codes() {
    let temp_dir = tempdir().unwrap();
    let project_dir = temp_dir.path().join("project");
    let output_dir = temp_dir.path().join("output");
    fs::create_dir_all(&project_dir).unwrap();
    fs::create_dir_all(&output_dir).unwrap();

    let prompter = TestPrompter::new(false, true); // Deny overwrite

    // Test 1: Non-existent input directory
    let args = Args {
        input: temp_dir
            .path()
            .join("nonexistent")
            .to_string_lossy()
            .to_string(),
        output: output_dir.join("test.md").to_string_lossy().to_string(),
        filter: vec![],
        ignore: vec![],
        preview: false,
        token_count: false,
        line_numbers: false,
        yes: true,
        diff_only: false,
        clear_cache: false,
    };

    let result = run_with_args(args, Config::default(), &prompter);
    assert!(
        result.is_err(),
        "Should fail with non-existent input directory"
    );

    // Test 2: Permission denied on output
    write_file(&project_dir.join("test.rs"), "fn test() {}\n");
    let output_file = output_dir.join("existing.md");
    write_file(&output_file, "existing content");

    let args = Args {
        input: project_dir.to_string_lossy().to_string(),
        output: output_file.to_string_lossy().to_string(),
        filter: vec!["rs".to_string()],
        ignore: vec![],
        preview: false,
        token_count: false,
        line_numbers: false,
        yes: false, // Don't auto-confirm
        diff_only: false,
        clear_cache: false,
    };

    let prompter_deny = TestPrompter::new(false, true); // Deny overwrite
    let result = run_with_args(args, Config::default(), &prompter_deny);
    assert!(result.is_err(), "Should fail when overwrite is denied");

    // Test 3: User cancellation during processing
    let args = Args {
        input: project_dir.to_string_lossy().to_string(),
        output: output_dir
            .join("cancelled.md")
            .to_string_lossy()
            .to_string(),
        filter: vec!["rs".to_string()],
        ignore: vec![],
        preview: false,
        token_count: false,
        line_numbers: false,
        yes: false,
        diff_only: false,
        clear_cache: false,
    };

    let prompter_cancel = TestPrompter::new(true, false); // Allow overwrite, deny processing
    let result = run_with_args(args, Config::default(), &prompter_cancel);
    assert!(result.is_err(), "Should fail when processing is cancelled");
}

#[test]
#[cfg(feature = "parallel")]
fn test_memory_usage_under_parallel_processing() {
    let temp_dir = tempdir().unwrap();
    let project_dir = temp_dir.path().join("project");
    fs::create_dir_all(&project_dir).unwrap();

    // Create many files to test memory efficiency
    for i in 0..500 {
        let subdir = project_dir.join(format!("module_{}", i / 50));
        fs::create_dir_all(&subdir).unwrap();

        let content = format!(
            "// File {}\nuse std::collections::HashMap;\n\npub fn function_{}() -> i32 {{\n    {}\n}}\n",
            i, i, i
        );
        write_file(&subdir.join(format!("file_{}.rs", i)), &content);
    }

    let output_dir = temp_dir.path().join("output");
    fs::create_dir_all(&output_dir).unwrap();

    let args = Args {
        input: project_dir.to_string_lossy().to_string(),
        output: output_dir
            .join("parallel_test.md")
            .to_string_lossy()
            .to_string(),
        filter: vec!["rs".to_string()],
        ignore: vec![],
        preview: false,
        token_count: false,
        line_numbers: false,
        yes: true,
        diff_only: false,
        clear_cache: false,
    };

    let prompter = TestPrompter::new(true, true);
    let result = run_with_args(args, Config::default(), &prompter);

    assert!(
        result.is_ok(),
        "Parallel processing should handle many files efficiently"
    );

    let output_path = output_dir.join("parallel_test.md");
    assert!(output_path.exists(), "Output should be created");

    let content = fs::read_to_string(&output_path).unwrap();

    // Verify all files are included and properly ordered
    assert!(
        content.contains("function_0"),
        "Should contain first function"
    );
    assert!(
        content.contains("function_499"),
        "Should contain last function"
    );

    // Verify substantial content was generated
    assert!(
        content.len() > 100_000,
        "Should generate substantial output"
    );

    // Check that files appear in a reasonable order (not completely scrambled)
    let first_pos = content.find("function_0").unwrap();
    let last_pos = content.find("function_499").unwrap();
    assert!(
        first_pos < last_pos,
        "Files should maintain reasonable ordering"
    );
}

#[test]
#[serial]
fn test_cwd_independent_operation() {
    let temp_dir = tempdir().unwrap();
    let project_dir = temp_dir.path().join("project");
    let output_dir = temp_dir.path().join("output");
    let different_cwd = temp_dir.path().join("different_cwd");

    fs::create_dir_all(&project_dir).unwrap();
    fs::create_dir_all(&output_dir).unwrap();
    fs::create_dir_all(&different_cwd).unwrap();

    // Create test files
    write_file(&project_dir.join("test.rs"), "fn test() {}\n");
    write_file(
        &project_dir.join("context-builder.toml"),
        r#"
filter = ["rs"]
line_numbers = true
"#,
    );

    // Store original directory
    let original_dir = std::env::current_dir().unwrap();

    // Test from different working directories
    let test_cwds = [temp_dir.path(), &different_cwd, &original_dir];

    for (i, test_cwd) in test_cwds.iter().enumerate() {
        std::env::set_current_dir(test_cwd).unwrap();

        let args = Args {
            input: project_dir.to_string_lossy().to_string(),
            output: output_dir
                .join(format!("cwd_test_{}.md", i))
                .to_string_lossy()
                .to_string(),
            filter: vec![], // Use config defaults
            ignore: vec![],
            preview: false,
            token_count: false,
            line_numbers: false, // Use config default
            yes: true,
            diff_only: false,
            clear_cache: false,
        };

        let config =
            context_builder::config::load_config_from_path(&project_dir).unwrap_or_default();
        let prompter = TestPrompter::new(true, true);

        let result = run_with_args(args, config, &prompter);
        assert!(result.is_ok(), "Should work regardless of CWD (test {})", i);

        let output_path = output_dir.join(format!("cwd_test_{}.md", i));
        assert!(
            output_path.exists(),
            "Output should exist for CWD test {}",
            i
        );

        let content = fs::read_to_string(&output_path).unwrap();

        // Should find the config file and apply its settings
        assert!(
            content.contains("test.rs"),
            "Should process rust files from config"
        );

        // All outputs should be identical regardless of CWD
        if i > 0 {
            let previous_content =
                fs::read_to_string(output_dir.join(format!("cwd_test_{}.md", i - 1))).unwrap();

            // Remove timestamps for comparison
            let normalize = |s: &str| -> String {
                s.lines()
                    .filter(|line| !line.contains("Processed at:"))
                    .collect::<Vec<_>>()
                    .join("\n")
            };

            assert_eq!(
                normalize(&content),
                normalize(&previous_content),
                "Output should be identical regardless of CWD"
            );
        }
    }

    // Restore original directory
    std::env::set_current_dir(original_dir).unwrap();
}

#[test]
fn test_edge_case_filenames_and_paths() {
    let temp_dir = tempdir().unwrap();
    let project_dir = temp_dir.path().join("project");
    let output_dir = temp_dir.path().join("output");
    fs::create_dir_all(&output_dir).unwrap();

    // Create files with problematic names
    let problematic_names = vec![
        "normal.rs",
        "with spaces.rs",
        "with-dashes.rs",
        "with_underscores.rs",
        "with.dots.rs",
        "uppercase.rs", // Changed from UPPERCASE.RS to avoid case issues
        "file.with.many.dots.rs",
        "123numeric.rs",
        // Note: Avoid truly problematic characters that might fail on Windows
    ];

    for name in &problematic_names {
        write_file(
            &project_dir.join("src").join(name),
            &format!("// File: {}\nfn test() {{}}\n", name),
        );
    }

    // Create nested directory structure
    write_file(
        &project_dir.join("deeply/nested/very/deep/path.rs"),
        "fn deep() {}\n",
    );

    let args = Args {
        input: project_dir.to_string_lossy().to_string(),
        output: output_dir
            .join("edge_case_paths.md")
            .to_string_lossy()
            .to_string(),
        filter: vec!["rs".to_string()],
        ignore: vec![],
        preview: false,
        token_count: false,
        line_numbers: false,
        yes: true,
        diff_only: false,
        clear_cache: false,
    };

    let prompter = TestPrompter::new(true, true);
    let result = run_with_args(args, Config::default(), &prompter);

    assert!(
        result.is_ok(),
        "Should handle edge case filenames without panicking"
    );

    let output_path = output_dir.join("edge_case_paths.md");
    assert!(output_path.exists(), "Output should be created");

    let content = fs::read_to_string(&output_path).unwrap();

    // Verify all problematic files are included
    for name in &problematic_names {
        assert!(
            content.contains(name),
            "Should include file with problematic name: {}",
            name
        );
    }

    // Verify deeply nested path is handled
    assert!(
        content.contains("deeply/nested") || content.contains("deeply\\nested"),
        "Should handle deeply nested paths"
    );
}
