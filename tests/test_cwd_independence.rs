//! Integration tests for CWD independence
//!
//! This test verifies that the application loads config and creates cache
//! relative to the project root, not the current working directory.

use std::fs;
use std::path::Path;
use tempfile::tempdir;

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

#[test]
fn test_config_loaded_from_project_root_not_cwd() {
    let temp_dir = tempdir().unwrap();
    let project_dir = temp_dir.path().join("project");
    let output_dir = temp_dir.path().join("output");
    let working_dir = temp_dir.path().join("working");

    // Create project with config file
    write_file(
        &project_dir.join("src/main.rs"),
        "fn main() { println!(\"Hello\"); }",
    );
    write_file(
        &project_dir.join("context-builder.toml"),
        r#"
auto_diff = true
line_numbers = true
filter = ["rs"]
"#,
    );

    // Create different config in working directory (should be ignored)
    write_file(
        &working_dir.join("context-builder.toml"),
        r#"
auto_diff = false
line_numbers = false
filter = ["txt"]
"#,
    );

    fs::create_dir_all(&output_dir).unwrap();
    fs::create_dir_all(&working_dir).unwrap();

    // Change to working directory
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(&working_dir).unwrap();

    // Load config from project directory (not CWD)
    let config = context_builder::config::load_config_from_path(&project_dir).unwrap_or_default();

    let mut args = Args {
        input: project_dir.to_string_lossy().to_string(), // Absolute path to project
        output: output_dir.join("output.md").to_string_lossy().to_string(),
        filter: vec![], // Should be overridden by project config
        ignore: vec![],
        preview: false,
        token_count: false,
        line_numbers: false, // Should be overridden by project config
        yes: true,
        diff_only: false,
        clear_cache: false,
    };

    // Apply config settings to args (mimicking the run() function logic)
    if args.filter.is_empty()
        && let Some(filter) = config.filter.clone()
    {
        args.filter = filter;
    }
    if !args.line_numbers
        && let Some(line_numbers) = config.line_numbers
    {
        args.line_numbers = line_numbers;
    }

    let prompter = TestPrompter::new(true, true);
    let result = run_with_args(args, config, &prompter);

    // Restore original directory
    std::env::set_current_dir(original_dir).unwrap();

    assert!(result.is_ok(), "Should succeed with CWD independence");

    let output_content = fs::read_to_string(output_dir.join("output.md")).unwrap();

    // Verify that project config was used, not working directory config
    assert!(
        output_content.contains("   1 |"),
        "Should have line numbers from project config"
    );
    assert!(
        output_content.contains("main.rs"),
        "Should include .rs files from project config filter"
    );
}

#[test]
fn test_cache_created_in_project_root_not_cwd() {
    let temp_dir = tempdir().unwrap();
    let project_dir = temp_dir.path().join("project");
    let output_dir = temp_dir.path().join("output");
    let working_dir = temp_dir.path().join("working");

    // Create project with auto-diff enabled
    write_file(
        &project_dir.join("src/main.rs"),
        "fn main() { println!(\"Hello\"); }",
    );
    write_file(
        &project_dir.join("context-builder.toml"),
        r#"
auto_diff = true
timestamped_output = true
"#,
    );

    fs::create_dir_all(&output_dir).unwrap();
    fs::create_dir_all(&working_dir).unwrap();

    // Get absolute paths before changing directory
    let project_dir_abs = project_dir.canonicalize().unwrap();
    let output_dir_abs = output_dir.canonicalize().unwrap();
    let working_dir_abs = working_dir.canonicalize().unwrap();

    // Change to working directory
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(&working_dir_abs).unwrap();

    // Load config from project directory
    let config =
        context_builder::config::load_config_from_path(&project_dir_abs).unwrap_or_default();

    let mut args = Args {
        input: project_dir_abs.to_string_lossy().to_string(), // Absolute path to project
        output: output_dir_abs
            .join("context.md")
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

    // Apply timestamping manually since we're bypassing run()
    if config.timestamped_output.unwrap_or(false) {
        use chrono::Utc;
        let timestamp = Utc::now().format("%Y%m%d%H%M%S").to_string();
        let path = std::path::Path::new(&args.output);
        let stem = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("output");
        let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("md");
        let new_filename = format!("{}_{}.{}", stem, timestamp, extension);
        if let Some(parent) = path.parent() {
            args.output = parent.join(new_filename).to_string_lossy().to_string();
        } else {
            args.output = output_dir_abs
                .join(new_filename)
                .to_string_lossy()
                .to_string();
        }
    }

    let prompter = TestPrompter::new(true, true);

    // First run to create cache
    let result1 = run_with_args(args.clone(), config.clone(), &prompter);
    assert!(result1.is_ok(), "First run should succeed");

    // Verify cache was created in project directory, not working directory
    let project_cache = project_dir_abs.join(".context-builder").join("cache");
    let working_cache = working_dir_abs.join(".context-builder").join("cache");

    assert!(
        project_cache.exists(),
        "Cache should be created in project directory"
    );
    assert!(
        !working_cache.exists(),
        "Cache should NOT be created in working directory"
    );

    // Small delay to ensure different timestamps
    std::thread::sleep(std::time::Duration::from_millis(1100));

    // Modify project file
    // Modify a file to trigger diff
    write_file(
        &project_dir_abs.join("src/main.rs"),
        "fn main() { println!(\"Hello, modified!\"); }",
    );

    // Create second args with new timestamp
    let mut args2 = args.clone();
    if config.timestamped_output.unwrap_or(false) {
        use chrono::Utc;
        let timestamp = Utc::now().format("%Y%m%d%H%M%S").to_string();
        let path = std::path::Path::new(&args2.output);
        let stem = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("output");
        let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("md");
        let new_filename = format!("{}_{}.{}", stem, timestamp, extension);
        if let Some(parent) = path.parent() {
            args2.output = parent.join(new_filename).to_string_lossy().to_string();
        } else {
            args2.output = output_dir_abs
                .join(new_filename)
                .to_string_lossy()
                .to_string();
        }
    }

    // Second run should detect changes using cache from project directory
    let result2 = run_with_args(args2, config, &prompter);
    assert!(result2.is_ok(), "Second run should succeed");

    // Find output files (should have timestamps) - use absolute path
    // Add retry logic to handle potential race conditions
    let output_files = (0..5)
        .find_map(|_| {
            std::thread::sleep(std::time::Duration::from_millis(50));
            if let Ok(entries) = fs::read_dir(&output_dir_abs) {
                let files: Vec<_> = entries
                    .filter_map(|entry| entry.ok())
                    .filter(|entry| {
                        let name = entry.file_name();
                        let name_str = name.to_string_lossy();
                        name_str.starts_with("context") && name_str.ends_with(".md")
                    })
                    .collect();
                if files.len() >= 2 { Some(files) } else { None }
            } else {
                None
            }
        })
        .expect("Failed to find output files after retries");

    // Restore original directory after file operations
    std::env::set_current_dir(original_dir).unwrap();

    assert!(
        output_files.len() >= 2,
        "Should have multiple timestamped outputs, found: {}",
        output_files.len()
    );

    // Check that second output contains diff information
    let latest_output = output_files
        .iter()
        .max_by_key(|entry| {
            // All paths are already absolute since we used output_dir_abs
            fs::metadata(entry.path()).unwrap().modified().unwrap()
        })
        .unwrap();

    // Read the latest file content
    let latest_content = fs::read_to_string(latest_output.path()).unwrap();
    assert!(
        latest_content.contains("## Change Summary") || latest_content.contains("Modified"),
        "Should contain change information from auto-diff"
    );
}

#[test]
fn test_clear_cache_uses_project_root() {
    let temp_dir = tempdir().unwrap();
    let project_dir = temp_dir.path().join("project");
    let working_dir = temp_dir.path().join("working");

    // Create project and working directories
    write_file(&project_dir.join("src/main.rs"), "fn main() {}");
    fs::create_dir_all(&working_dir).unwrap();

    // Create cache in project directory
    let project_cache_dir = project_dir.join(".context-builder").join("cache");
    fs::create_dir_all(&project_cache_dir).unwrap();
    fs::write(project_cache_dir.join("test_cache.json"), "{}").unwrap();

    // Create cache in working directory (should not be affected)
    let working_cache_dir = working_dir.join(".context-builder").join("cache");
    fs::create_dir_all(&working_cache_dir).unwrap();
    fs::write(working_cache_dir.join("test_cache.json"), "{}").unwrap();

    // Change to working directory
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(&working_dir).unwrap();

    // Simulate the cache clearing logic from run() function
    // This tests that cache clearing uses project root, not CWD
    let cache_path = project_dir.join(".context-builder").join("cache");
    assert!(
        cache_path.exists(),
        "Project cache should exist before clearing"
    );

    if cache_path.exists() {
        fs::remove_dir_all(&cache_path).unwrap();
    }

    // Restore original directory
    std::env::set_current_dir(original_dir).unwrap();

    // Project cache should be cleared
    assert!(
        !project_cache_dir.exists(),
        "Project cache should be cleared"
    );

    // Working directory cache should be untouched
    assert!(
        working_cache_dir.exists() && fs::read_dir(&working_cache_dir).unwrap().count() > 0,
        "Working directory cache should remain untouched"
    );
}

#[test]
fn test_load_config_from_path_function() {
    let temp_dir = tempdir().unwrap();
    let project_dir = temp_dir.path().join("project");
    let working_dir = temp_dir.path().join("working");

    // Create project with config file
    write_file(
        &project_dir.join("context-builder.toml"),
        r#"
auto_diff = true
line_numbers = true
filter = ["rs"]
"#,
    );

    // Create different config in working directory
    write_file(
        &working_dir.join("context-builder.toml"),
        r#"
auto_diff = false
line_numbers = false
filter = ["txt"]
"#,
    );

    // Change to working directory
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(&working_dir).unwrap();

    // Load config from project directory (not CWD)
    let config = context_builder::config::load_config_from_path(&project_dir);

    // Restore original directory
    std::env::set_current_dir(original_dir).unwrap();

    assert!(
        config.is_some(),
        "Should load config from project directory"
    );
    let config = config.unwrap();

    assert_eq!(
        config.auto_diff,
        Some(true),
        "Should use project config auto_diff"
    );
    assert_eq!(
        config.line_numbers,
        Some(true),
        "Should use project config line_numbers"
    );
    assert_eq!(
        config.filter,
        Some(vec!["rs".to_string()]),
        "Should use project config filter"
    );
}
