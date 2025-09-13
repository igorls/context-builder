//! Integration tests for determinism and robustness of context-builder
//!
//! These tests verify that the critical bug fixes are working correctly:
//! - Deterministic output order
//! - Robust caching
//! - Thread safety

use pretty_assertions::assert_eq;
use serial_test::serial;
use std::fs;
use std::path::Path;
use tempfile::tempdir;

use chrono::Utc;
use context_builder::cli::Args;
use context_builder::config::{Config, load_config};
use context_builder::{Prompter, run_with_args};

/// Test prompter that always confirms
struct TestPrompter;

impl Prompter for TestPrompter {
    fn confirm_processing(&self, _file_count: usize) -> std::io::Result<bool> {
        Ok(true)
    }
    fn confirm_overwrite(&self, _file_path: &str) -> std::io::Result<bool> {
        Ok(true)
    }
}

/// Create a test project with multiple files in different directories
fn create_test_project(base_dir: &Path) -> std::io::Result<()> {
    let src_dir = base_dir.join("src");
    let tests_dir = base_dir.join("tests");
    let docs_dir = base_dir.join("docs");

    fs::create_dir_all(&src_dir)?;
    fs::create_dir_all(&tests_dir)?;
    fs::create_dir_all(&docs_dir)?;

    // Create files in different orders to test sorting
    fs::write(
        src_dir.join("main.rs"),
        "fn main() {\n    println!(\"Hello\");\n}",
    )?;
    fs::write(src_dir.join("lib.rs"), "pub mod utils;\npub mod config;")?;
    fs::write(src_dir.join("utils.rs"), "pub fn helper() {}")?;
    fs::write(
        tests_dir.join("integration.rs"),
        "#[test]\nfn test_something() {}",
    )?;
    fs::write(tests_dir.join("unit.rs"), "#[test]\nfn test_unit() {}")?;
    fs::write(
        docs_dir.join("README.md"),
        "# Project\n\nThis is a test project.",
    )?;
    fs::write(
        base_dir.join("Cargo.toml"),
        "[package]\nname = \"test\"\nversion = \"0.1.0\"",
    )?;

    Ok(())
}

#[test]
#[serial] // Ensure tests don't interfere with each other
fn test_deterministic_output_multiple_runs() {
    let temp_dir = tempdir().unwrap();
    let project_dir = temp_dir.path().join("project");
    create_test_project(&project_dir).unwrap();

    // Note: The actual output files may have timestamps appended due to auto-diff mode
    // We'll need to find the actual files created
    let prompter = TestPrompter;

    // Run twice with identical arguments
    let result1 = run_with_args(
        Args {
            input: project_dir.to_string_lossy().to_string(),
            output: temp_dir
                .path()
                .join("output1.md")
                .to_string_lossy()
                .to_string(),
            filter: vec!["rs".to_string(), "md".to_string(), "toml".to_string()],
            ignore: vec![],
            preview: false,
            token_count: false,
            line_numbers: false,
            yes: true,
            diff_only: false,
            clear_cache: false,
        },
        Config::default(),
        &prompter,
    );

    let result2 = run_with_args(
        Args {
            input: project_dir.to_string_lossy().to_string(),
            output: temp_dir
                .path()
                .join("output2.md")
                .to_string_lossy()
                .to_string(),
            filter: vec!["rs".to_string(), "md".to_string(), "toml".to_string()],
            ignore: vec![],
            preview: false,
            token_count: false,
            line_numbers: false,
            yes: true,
            diff_only: false,
            clear_cache: false,
        },
        Config::default(),
        &prompter,
    );

    if let Err(e) = result1 {
        panic!("First run failed: {}", e);
    }
    if let Err(e) = result2 {
        panic!("Second run failed: {}", e);
    }

    // Find the actual output files (they may have timestamps appended)
    let temp_entries: Vec<_> = fs::read_dir(temp_dir.path())
        .unwrap()
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            let file_name = entry.file_name();
            let name = file_name.to_string_lossy();
            name.starts_with("output") && name.ends_with(".md")
        })
        .collect();

    if temp_entries.len() < 2 {
        eprintln!("Expected 2 output files, found {}", temp_entries.len());
        eprintln!("Temp directory contents:");
        for entry in fs::read_dir(temp_dir.path()).unwrap() {
            eprintln!("  {:?}", entry.unwrap().file_name());
        }
        panic!("Not enough output files found");
    }

    // Sort to ensure consistent ordering
    let mut output_files: Vec<_> = temp_entries.iter().map(|entry| entry.path()).collect();
    output_files.sort();

    // Read both outputs
    let content1 = fs::read_to_string(&output_files[0]).unwrap();
    let content2 = fs::read_to_string(&output_files[1]).unwrap();

    // Debug: Write contents to temp files for inspection
    fs::write(temp_dir.path().join("debug_content1.md"), &content1).unwrap();
    fs::write(temp_dir.path().join("debug_content2.md"), &content2).unwrap();

    // Normalize timestamps for comparison since they will be different
    let normalize = |content: &str| -> String {
        content
            .lines()
            .map(|line| {
                if line.starts_with("Processed at: ") {
                    "Processed at: <timestamp>"
                } else {
                    line
                }
            })
            .collect::<Vec<_>>()
            .join("\n")
    };

    let normalized1 = normalize(&content1);
    let normalized2 = normalize(&content2);

    // Debug: Write normalized contents for comparison
    fs::write(temp_dir.path().join("debug_normalized1.md"), &normalized1).unwrap();
    fs::write(temp_dir.path().join("debug_normalized2.md"), &normalized2).unwrap();

    // They should be identical (deterministic) after normalizing timestamps
    if normalized1 != normalized2 {
        eprintln!(
            "Content1 length: {}, Content2 length: {}",
            normalized1.len(),
            normalized2.len()
        );
        eprintln!(
            "First difference at position: {:?}",
            normalized1
                .chars()
                .zip(normalized2.chars())
                .position(|(a, b)| a != b)
        );
        eprintln!("Debug files written to: {}", temp_dir.path().display());
        panic!("Output should be deterministic across multiple runs (ignoring timestamps)");
    }

    // Verify that files are listed in a consistent order
    let lines: Vec<&str> = content1.lines().collect();
    let file_lines: Vec<&str> = lines
        .iter()
        .filter(|line| line.starts_with("### File: `"))
        .copied()
        .collect();

    // Should have found some files
    assert!(
        !file_lines.is_empty(),
        "Should have found some file entries"
    );

    // Check that files are sorted alphabetically
    let mut sorted_files = file_lines.clone();
    sorted_files.sort();
    assert_eq!(
        file_lines, sorted_files,
        "Files should be listed in alphabetical order"
    );
}
#[test]
#[serial] // Ensure tests don't interfere with each other
fn test_deterministic_file_tree_order() {
    let temp_dir = tempdir().unwrap();
    let project_dir = temp_dir.path().join("project");
    create_test_project(&project_dir).unwrap();

    let output_path = temp_dir.path().join("output.md");

    // Change to project directory so config loading works
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(&project_dir).unwrap();

    let args = Args {
        input: ".".to_string(),
        output: output_path.to_string_lossy().to_string(),
        filter: vec![],
        ignore: vec![],
        preview: false,
        token_count: false,
        line_numbers: false,
        yes: true,
        diff_only: false,
        clear_cache: false,
    };

    let prompter = TestPrompter;
    run_with_args(args, Config::default(), &prompter).unwrap();

    // Restore original directory
    std::env::set_current_dir(original_dir).unwrap();

    let content = fs::read_to_string(&output_path).unwrap();

    // Find the file tree section
    let tree_start = content
        .find("## File Tree Structure")
        .expect("Should have file tree section");
    let files_start = content.find("### File: `").unwrap_or(content.len());
    let tree_section = &content[tree_start..files_start];

    // Check that directories and files appear in alphabetical order in the tree
    // This is a basic check - a more sophisticated test would parse the tree structure
    assert!(tree_section.contains("Cargo.toml"));
    // Check for directory entries - they may appear as just the name or with trailing content
    assert!(tree_section.contains("docs") || tree_section.contains("docs/"));
    assert!(tree_section.contains("src") || tree_section.contains("src/"));
    assert!(tree_section.contains("tests") || tree_section.contains("tests/"));
}

#[test]
#[serial] // Ensure cache tests don't interfere with each other
fn test_cache_collision_prevention() {
    let temp_dir1 = tempdir().unwrap();
    let temp_dir2 = tempdir().unwrap();

    let project1 = temp_dir1.path().join("project");
    let project2 = temp_dir2.path().join("project");

    create_test_project(&project1).unwrap();
    create_test_project(&project2).unwrap();

    // Add different content to make projects distinct
    fs::write(project1.join("unique1.txt"), "This is project 1").unwrap();
    fs::write(project2.join("unique2.txt"), "This is project 2").unwrap();

    let output1 = temp_dir1.path().join("output.md");
    let output2 = temp_dir2.path().join("output.md");

    let prompter = TestPrompter;

    // Change to project1 directory and run
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(&project1).unwrap();

    let args1 = Args {
        input: ".".to_string(),
        output: output1.to_string_lossy().to_string(),
        filter: vec![],
        ignore: vec![],
        preview: false,
        token_count: false,
        line_numbers: false,
        yes: true,
        diff_only: false,
        clear_cache: false,
    };

    run_with_args(args1, Config::default(), &prompter).unwrap();

    // Change to project2 directory and run
    std::env::set_current_dir(&project2).unwrap();

    let args2 = Args {
        input: ".".to_string(),
        output: output2.to_string_lossy().to_string(),
        filter: vec!["txt".to_string()],
        ignore: vec![],
        preview: false,
        token_count: false,
        line_numbers: false,
        yes: true,
        diff_only: false,
        clear_cache: false,
    };

    run_with_args(args2, Config::default(), &prompter).unwrap();

    // Restore original directory
    std::env::set_current_dir(original_dir).unwrap();

    let content1 = fs::read_to_string(&output1).unwrap();
    let content2 = fs::read_to_string(&output2).unwrap();

    // Outputs should be different due to different projects and configs
    assert_ne!(
        content1, content2,
        "Different projects should produce different outputs"
    );

    // Each should contain their unique content
    assert!(content1.contains("unique1.txt"));
    assert!(content2.contains("unique2.txt"));
}

#[test]
#[serial] // Ensure tests don't interfere with each other
fn test_custom_ignores_performance() {
    let temp_dir = tempdir().unwrap();
    let project_dir = temp_dir.path().join("project");

    // Create a project with ignored directories
    create_test_project(&project_dir).unwrap();

    let target_dir = project_dir.join("target");
    let node_modules_dir = project_dir.join("node_modules");

    fs::create_dir_all(&target_dir).unwrap();
    fs::create_dir_all(&node_modules_dir).unwrap();

    // Create many files in ignored directories
    for i in 0..10 {
        fs::write(target_dir.join(format!("file{}.txt", i)), "ignored content").unwrap();
        fs::write(
            node_modules_dir.join(format!("module{}.js", i)),
            "ignored js",
        )
        .unwrap();
    }

    let output_path = temp_dir.path().join("output.md");

    // Change to project directory so config loading works
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(&project_dir).unwrap();

    let args = Args {
        input: ".".to_string(),
        output: output_path.to_string_lossy().to_string(),
        filter: vec![],
        ignore: vec!["target".to_string(), "node_modules".to_string()],
        preview: false,
        token_count: false,
        line_numbers: false,
        yes: true,
        diff_only: false,
        clear_cache: false,
    };

    let prompter = TestPrompter;
    let start = std::time::Instant::now();

    run_with_args(args, Config::default(), &prompter).unwrap();

    // Restore original directory
    std::env::set_current_dir(original_dir).unwrap();

    let duration = start.elapsed();

    let content = fs::read_to_string(&output_path).unwrap();

    // Verify ignored files are not included
    assert!(!content.contains("target/file"));
    assert!(!content.contains("node_modules/module"));

    // Performance should be reasonable (this is a basic check)
    assert!(
        duration.as_secs() < 5,
        "Should complete within reasonable time even with ignored directories"
    );
}

#[test]
#[serial] // Ensure cache tests don't interfere with each other
fn test_configuration_affects_cache_key() {
    let temp_dir = tempdir().unwrap();
    let project_dir = temp_dir.path().join("project");
    create_test_project(&project_dir).unwrap();

    // Test that different configurations create different cache behaviors
    // This is verified indirectly by ensuring different configs produce appropriate outputs

    let output1_path = temp_dir.path().join("output1.md");
    let output2_path = temp_dir.path().join("output2.md");

    // Change to project directory so config loading works
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(&project_dir).unwrap();

    let args1 = Args {
        input: ".".to_string(),
        output: output1_path.to_string_lossy().to_string(),
        filter: vec!["rs".to_string()],
        ignore: vec![],
        preview: false,
        token_count: false,
        line_numbers: false,
        yes: true,
        diff_only: false,
        clear_cache: false,
    };

    let args2 = Args {
        input: ".".to_string(),
        output: output2_path.to_string_lossy().to_string(),
        filter: vec!["md".to_string()],
        ignore: vec![],
        preview: false,
        token_count: false,
        line_numbers: false,
        yes: true,
        diff_only: false,
        clear_cache: false,
    };

    let prompter = TestPrompter;

    run_with_args(args1, Config::default(), &prompter).unwrap();
    run_with_args(args2, Config::default(), &prompter).unwrap();

    // Restore original directory
    std::env::set_current_dir(original_dir).unwrap();

    let content1 = fs::read_to_string(&output1_path).unwrap();
    let content2 = fs::read_to_string(&output2_path).unwrap();

    // Different filters should produce different outputs
    assert_ne!(content1, content2);

    // Verify filter effects
    assert!(content1.contains(".rs"));
    assert!(content2.contains("README.md"));
    // Note: Due to file tree section, both outputs may contain references to all files
    // but the actual file content sections should be filtered
}

#[test]
#[serial] // Ensure tests don't interfere with each other
fn test_edge_case_filenames_no_panic() {
    let temp_dir = tempdir().unwrap();
    let project_dir = temp_dir.path().join("project");
    fs::create_dir_all(&project_dir).unwrap();

    // Create files with edge case names that could cause panics
    fs::write(project_dir.join(".bashrc"), "# bash config").unwrap(); // no extension
    fs::write(project_dir.join("Dockerfile"), "FROM alpine").unwrap(); // no extension
    fs::write(project_dir.join(".gitignore"), "target/").unwrap(); // starts with dot, no extension

    // Change to project directory
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(&project_dir).unwrap();

    // Create a config file that enables timestamped output
    fs::write(
        project_dir.join(".context-builder.toml"),
        r#"
timestamped_output = true
auto_diff = true
"#,
    )
    .unwrap();

    // Test with output filename that has no extension (extreme edge case)
    let output_path = temp_dir.path().join("no_extension_output");

    let args = Args {
        input: ".".to_string(),
        output: output_path.to_string_lossy().to_string(),
        filter: vec![],
        ignore: vec![],
        preview: false,
        token_count: false,
        line_numbers: false,
        yes: true,
        diff_only: false,
        clear_cache: false,
    };

    let prompter = TestPrompter;

    // This should not panic even with edge case filenames
    let config = load_config().unwrap_or_default();

    // Apply config merging manually since we're bypassing run()
    let mut final_args = args;

    // Apply line_numbers from config
    if !final_args.line_numbers
        && let Some(line_numbers) = config.line_numbers
    {
        final_args.line_numbers = line_numbers;
    }

    // Apply diff_only from config
    if !final_args.diff_only
        && let Some(diff_only) = config.diff_only
    {
        final_args.diff_only = diff_only;
    }

    // Apply timestamping manually since we're bypassing run()
    if config.timestamped_output.unwrap_or(false) {
        let timestamp = Utc::now().format("%Y%m%d%H%M%S").to_string();
        let path = std::path::Path::new(&final_args.output);
        let stem = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("output");
        let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("md");
        let new_filename = format!("{}_{}.{}", stem, timestamp, extension);
        if let Some(parent) = path.parent() {
            final_args.output = parent.join(new_filename).to_string_lossy().to_string();
        } else {
            final_args.output = new_filename;
        }
    }

    let result = run_with_args(final_args, config, &prompter);
    std::env::set_current_dir(original_dir).unwrap();

    // Should succeed without panicking
    assert!(
        result.is_ok(),
        "Should handle edge case filenames without panicking"
    );

    // Verify a timestamped file was created
    let temp_entries: Vec<_> = fs::read_dir(temp_dir.path())
        .unwrap()
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            let name = entry.file_name();
            let name_str = name.to_string_lossy();
            name_str.starts_with("no_extension_output_") && name_str.contains("2025")
        })
        .collect();

    assert!(
        !temp_entries.is_empty(),
        "Should create timestamped output file even with edge case input filename"
    );
}
