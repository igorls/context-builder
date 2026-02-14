//! Integration tests for auto-diff functionality
//!
//! These tests verify that the auto-diff feature works correctly and robustly:
//! - Cache management and collision prevention
//! - Diff generation accuracy
//! - Configuration changes affecting cache
//! - Error recovery from corrupted cache

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

fn create_simple_project(base_dir: &Path) -> std::io::Result<()> {
    let src_dir = base_dir.join("src");
    fs::create_dir_all(&src_dir)?;

    fs::write(
        src_dir.join("main.rs"),
        "fn main() {\n    println!(\"Hello, world!\");\n}",
    )?;
    fs::write(
        src_dir.join("lib.rs"),
        "pub fn add(a: i32, b: i32) -> i32 {\n    a + b\n}",
    )?;
    fs::write(
        base_dir.join("README.md"),
        "# Test Project\n\nThis is a test project for auto-diff.",
    )?;

    // Create config file to enable auto-diff
    fs::write(
        base_dir.join("context-builder.toml"),
        r#"
auto_diff = true
timestamped_output = true
"#,
    )?;

    Ok(())
}

#[test]
#[serial]
fn test_auto_diff_workflow_basic() {
    let temp_dir = tempdir().unwrap();
    let project_dir = temp_dir.path().join("project");
    create_simple_project(&project_dir).unwrap();

    let output_dir = temp_dir.path().join("output");
    fs::create_dir_all(&output_dir).unwrap();

    // Change to project directory so config loading works
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(&project_dir).unwrap();

    let args = Args {
        input: ".".to_string(), // Use current directory
        output: output_dir.join("context.md").to_string_lossy().to_string(),
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
    let prompter = TestPrompter;

    // First run - should create initial output without diffs
    let config = load_config().unwrap_or_default();

    // Apply config merging manually since we're bypassing run()
    let mut first_args = args.clone();

    // Apply line_numbers from config (matches run_with_args behavior)
    if let Some(line_numbers) = config.line_numbers {
        first_args.line_numbers = line_numbers;
    }

    // Apply diff_only from config
    if let Some(diff_only) = config.diff_only {
        first_args.diff_only = diff_only;
    }

    // Apply timestamping manually since we're bypassing run()
    if config.timestamped_output.unwrap_or(false) {
        let timestamp = Utc::now().format("%Y%m%d%H%M%S").to_string();
        let path = std::path::Path::new(&first_args.output);
        let stem = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("output");
        let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("md");
        let new_filename = format!("{}_{}.{}", stem, timestamp, extension);
        if let Some(parent) = path.parent() {
            first_args.output = parent.join(new_filename).to_string_lossy().to_string();
        } else {
            first_args.output = new_filename;
        }
    }

    run_with_args(first_args, config.clone(), &prompter).unwrap();

    // Check that output was created
    let first_output = fs::read_dir(&output_dir)
        .unwrap()
        .next()
        .unwrap()
        .unwrap()
        .path();
    let first_content = fs::read_to_string(&first_output).unwrap();

    // Should not contain change summary on first run
    assert!(!first_content.contains("## Change Summary"));
    assert!(!first_content.contains("## File Differences"));

    // Modify a file
    fs::write(
        project_dir.join("src").join("main.rs"),
        "fn main() {\n    println!(\"Hello, Rust!\");\n    println!(\"Modified!\");\n}",
    )
    .unwrap();

    // Small delay to ensure different timestamps
    std::thread::sleep(std::time::Duration::from_millis(1100));

    // Second run - should detect changes
    let config = load_config().unwrap_or_default();

    // Apply config merging manually since we're bypassing run()
    let mut second_args = args;

    // Apply line_numbers from config (matches run_with_args behavior)
    if let Some(line_numbers) = config.line_numbers {
        second_args.line_numbers = line_numbers;
    }

    // Apply diff_only from config
    if let Some(diff_only) = config.diff_only {
        second_args.diff_only = diff_only;
    }

    // Apply timestamping manually since we're bypassing run()
    if config.timestamped_output.unwrap_or(false) {
        let timestamp = Utc::now().format("%Y%m%d%H%M%S").to_string();
        let path = std::path::Path::new(&second_args.output);
        let stem = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("output");
        let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("md");
        let new_filename = format!("{}_{}.{}", stem, timestamp, extension);
        if let Some(parent) = path.parent() {
            second_args.output = parent.join(new_filename).to_string_lossy().to_string();
        } else {
            second_args.output = new_filename;
        }
    }

    run_with_args(second_args, config, &prompter).unwrap();

    // Restore original directory
    std::env::set_current_dir(original_dir).unwrap();

    // Find the second output file (should have different timestamp)
    let outputs: Vec<_> = fs::read_dir(&output_dir)
        .unwrap()
        .map(|e| e.unwrap().path())
        .collect();
    assert_eq!(outputs.len(), 2, "Should have two output files");

    let second_output = outputs.iter().find(|&p| p != &first_output).unwrap();
    let second_content = fs::read_to_string(second_output).unwrap();

    // Should contain change summary
    assert!(second_content.contains("## Change Summary"));
    // Handle both Windows and Unix path separators
    assert!(
        second_content.contains("- Modified: `src/main.rs`")
            || second_content.contains("- Modified: `src\\main.rs`")
    );

    // Should contain file differences
    assert!(second_content.contains("## File Differences"));
    assert!(
        second_content.contains("### Diff: `src/main.rs`")
            || second_content.contains("### Diff: `src\\main.rs`")
    );
    assert!(second_content.contains("Hello, world!"));
    assert!(second_content.contains("Hello, Rust!"));
    assert!(second_content.contains("Modified!"));
}

#[test]
#[serial]
fn test_auto_diff_added_and_removed_files() {
    let temp_dir = tempdir().unwrap();
    let project_dir = temp_dir.path().join("project");
    create_simple_project(&project_dir).unwrap();

    let output_dir = temp_dir.path().join("output");
    fs::create_dir_all(&output_dir).unwrap();

    // Change to project directory so config loading works
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(&project_dir).unwrap();

    let args = Args {
        input: ".".to_string(), // Use current directory
        output: output_dir.join("context.md").to_string_lossy().to_string(),
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

    let prompter = TestPrompter;

    // First run
    let config = load_config().unwrap_or_default();

    // Apply config merging manually since we're bypassing run()
    let mut first_args = args.clone();

    // Apply line_numbers from config
    if !first_args.line_numbers
        && let Some(line_numbers) = config.line_numbers
    {
        first_args.line_numbers = line_numbers;
    }

    // Apply diff_only from config
    if !first_args.diff_only
        && let Some(diff_only) = config.diff_only
    {
        first_args.diff_only = diff_only;
    }

    // Apply timestamping manually since we're bypassing run()
    if config.timestamped_output.unwrap_or(false) {
        let timestamp = Utc::now().format("%Y%m%d%H%M%S").to_string();
        let path = std::path::Path::new(&first_args.output);
        let stem = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("output");
        let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("md");
        let new_filename = format!("{}_{}.{}", stem, timestamp, extension);
        if let Some(parent) = path.parent() {
            first_args.output = parent.join(new_filename).to_string_lossy().to_string();
        } else {
            first_args.output = new_filename;
        }
    }

    run_with_args(first_args, config.clone(), &prompter).unwrap();

    // Add a new file and remove an existing one
    fs::write(
        project_dir.join("src").join("new_module.rs"),
        "pub fn new_function() -> String {\n    \"new\".to_string()\n}",
    )
    .unwrap();

    fs::remove_file(project_dir.join("src").join("lib.rs")).unwrap();

    // Small delay to ensure different timestamps
    std::thread::sleep(std::time::Duration::from_millis(1100));

    // Second run
    let config = load_config().unwrap_or_default();

    // Apply config merging manually since we're bypassing run()
    let mut second_args = args;

    // Apply line_numbers from config
    if !second_args.line_numbers
        && let Some(line_numbers) = config.line_numbers
    {
        second_args.line_numbers = line_numbers;
    }

    // Apply diff_only from config
    if !second_args.diff_only
        && let Some(diff_only) = config.diff_only
    {
        second_args.diff_only = diff_only;
    }

    // Apply timestamping manually since we're bypassing run()
    if config.timestamped_output.unwrap_or(false) {
        let timestamp = Utc::now().format("%Y%m%d%H%M%S").to_string();
        let path = std::path::Path::new(&second_args.output);
        let stem = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("output");
        let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("md");
        let new_filename = format!("{}_{}.{}", stem, timestamp, extension);
        if let Some(parent) = path.parent() {
            second_args.output = parent.join(new_filename).to_string_lossy().to_string();
        } else {
            second_args.output = new_filename;
        }
    }

    run_with_args(second_args, config, &prompter).unwrap();

    // Restore original directory
    std::env::set_current_dir(original_dir).unwrap();

    let outputs: Vec<_> = fs::read_dir(&output_dir)
        .unwrap()
        .map(|e| e.unwrap().path())
        .collect();
    let latest_output = outputs
        .iter()
        .max_by_key(|p| fs::metadata(p).unwrap().modified().unwrap())
        .unwrap();
    let content = fs::read_to_string(latest_output).unwrap();

    // Should show both added and removed files
    // Handle both Windows and Unix path separators
    assert!(
        content.contains("- Added: `src/new_module.rs`")
            || content.contains("- Added: `src\\new_module.rs`")
    );
    // Handle both Windows and Unix path separators
    assert!(
        content.contains("- Removed: `src/lib.rs`") || content.contains("- Removed: `src\\lib.rs`")
    );

    // Added files should be marked in the files section
    assert!(content.contains("_Status: Added_"));
}

#[test]
#[serial]
fn test_diff_only_mode() {
    let temp_dir = tempdir().unwrap();
    let project_dir = temp_dir.path().join("project");
    create_simple_project(&project_dir).unwrap();

    // Update config to enable diff_only
    fs::write(
        project_dir.join("context-builder.toml"),
        r#"
auto_diff = true
timestamped_output = true
diff_only = true
"#,
    )
    .unwrap();

    let output_dir = temp_dir.path().join("output");
    fs::create_dir_all(&output_dir).unwrap();

    // Change to project directory so config loading works
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(&project_dir).unwrap();

    let args = Args {
        input: ".".to_string(), // Use current directory
        output: output_dir.join("context.md").to_string_lossy().to_string(),
        filter: vec![],
        ignore: vec![],
        preview: false,
        token_count: false,
        line_numbers: false,
        yes: true,
        diff_only: false, // Config file should override this
        clear_cache: false,
        init: false,
        max_tokens: None,
    };

    let prompter = TestPrompter;

    // First run
    let config = load_config().unwrap_or_default();

    // Apply config merging manually since we're bypassing run()
    let mut first_args = args.clone();

    // Apply line_numbers from config
    if !first_args.line_numbers
        && let Some(line_numbers) = config.line_numbers
    {
        first_args.line_numbers = line_numbers;
    }

    // Apply diff_only from config
    if !first_args.diff_only
        && let Some(diff_only) = config.diff_only
    {
        first_args.diff_only = diff_only;
    }

    // Apply timestamping manually since we're bypassing run()
    if config.timestamped_output.unwrap_or(false) {
        let timestamp = Utc::now().format("%Y%m%d%H%M%S").to_string();
        let path = std::path::Path::new(&first_args.output);
        let stem = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("output");
        let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("md");
        let new_filename = format!("{}_{}.{}", stem, timestamp, extension);
        if let Some(parent) = path.parent() {
            first_args.output = parent.join(new_filename).to_string_lossy().to_string();
        } else {
            first_args.output = new_filename;
        }
    }

    run_with_args(first_args, config.clone(), &prompter).unwrap();

    // Modify a file
    fs::write(
        project_dir.join("src").join("main.rs"),
        "fn main() {\n    println!(\"Changed!\");\n}",
    )
    .unwrap();

    // Small delay to ensure different timestamps
    std::thread::sleep(std::time::Duration::from_millis(1100));

    // Second run
    let config = load_config().unwrap_or_default();

    // Apply config merging manually since we're bypassing run()
    let mut second_args = args;

    // Apply line_numbers from config
    if !second_args.line_numbers
        && let Some(line_numbers) = config.line_numbers
    {
        second_args.line_numbers = line_numbers;
    }

    // Apply diff_only from config
    if !second_args.diff_only
        && let Some(diff_only) = config.diff_only
    {
        second_args.diff_only = diff_only;
    }

    // Apply timestamping manually since we're bypassing run()
    if config.timestamped_output.unwrap_or(false) {
        let timestamp = Utc::now().format("%Y%m%d%H%M%S").to_string();
        let path = std::path::Path::new(&second_args.output);
        let stem = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("output");
        let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("md");
        let new_filename = format!("{}_{}.{}", stem, timestamp, extension);
        if let Some(parent) = path.parent() {
            second_args.output = parent.join(new_filename).to_string_lossy().to_string();
        } else {
            second_args.output = new_filename;
        }
    }

    run_with_args(second_args, config, &prompter).unwrap();

    // Restore original directory
    std::env::set_current_dir(original_dir).unwrap();

    let outputs: Vec<_> = fs::read_dir(&output_dir)
        .unwrap()
        .map(|e| e.unwrap().path())
        .collect();
    let latest_output = outputs
        .iter()
        .max_by_key(|p| fs::metadata(p).unwrap().modified().unwrap())
        .unwrap();
    let content = fs::read_to_string(latest_output).unwrap();

    // Should have change summary and diffs
    assert!(content.contains("## Change Summary"));
    assert!(content.contains("## File Differences"));

    // Should NOT have full file bodies section
    assert!(!content.contains("## Files"));

    // But should still have the file tree and header
    assert!(content.contains("## File Tree Structure"));
    assert!(content.contains("# Directory Structure Report"));
}

#[test]
#[serial]
fn test_cache_invalidation_on_config_change() {
    let temp_dir = tempdir().unwrap();
    let project_dir = temp_dir.path().join("project");
    create_simple_project(&project_dir).unwrap();

    let output_dir = temp_dir.path().join("output");
    fs::create_dir_all(&output_dir).unwrap();

    // Change to project directory so config loading works
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(&project_dir).unwrap();

    let args_base = Args {
        input: ".".to_string(), // Use current directory
        output: output_dir.join("context.md").to_string_lossy().to_string(),
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

    let prompter = TestPrompter;

    // First run with original config
    let config = load_config().unwrap_or_default();

    // Apply config merging manually since we're bypassing run()
    let mut first_args = args_base.clone();

    // Apply line_numbers from config
    if !first_args.line_numbers
        && let Some(line_numbers) = config.line_numbers
    {
        first_args.line_numbers = line_numbers;
    }

    // Apply diff_only from config
    if !first_args.diff_only
        && let Some(diff_only) = config.diff_only
    {
        first_args.diff_only = diff_only;
    }

    // Apply timestamping manually since we're bypassing run()
    if config.timestamped_output.unwrap_or(false) {
        let timestamp = Utc::now().format("%Y%m%d%H%M%S").to_string();
        let path = std::path::Path::new(&first_args.output);
        let stem = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("output");
        let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("md");
        let new_filename = format!("{}_{}.{}", stem, timestamp, extension);
        if let Some(parent) = path.parent() {
            first_args.output = parent.join(new_filename).to_string_lossy().to_string();
        } else {
            first_args.output = new_filename;
        }
    }

    run_with_args(first_args, config, &prompter).unwrap();

    // Change configuration - add line numbers
    fs::write(
        project_dir.join("context-builder.toml"),
        r#"
auto_diff = true
timestamped_output = true
line_numbers = true
"#,
    )
    .unwrap();

    // Small delay to ensure different timestamps
    std::thread::sleep(std::time::Duration::from_millis(1100));

    // Second run with new config should not show diffs (cache should be invalidated)
    let config = load_config().unwrap_or_default();

    // Apply config merging manually since we're bypassing run()
    let mut second_args = args_base;

    // Apply line_numbers from config (matches run_with_args behavior)
    if let Some(line_numbers) = config.line_numbers {
        second_args.line_numbers = line_numbers;
    }

    // Apply diff_only from config
    if let Some(diff_only) = config.diff_only {
        second_args.diff_only = diff_only;
    }

    // Apply timestamping manually since we're bypassing run()
    if config.timestamped_output.unwrap_or(false) {
        let timestamp = Utc::now().format("%Y%m%d%H%M%S").to_string();
        let path = std::path::Path::new(&second_args.output);
        let stem = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("output");
        let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("md");
        let new_filename = format!("{}_{}.{}", stem, timestamp, extension);
        if let Some(parent) = path.parent() {
            second_args.output = parent.join(new_filename).to_string_lossy().to_string();
        } else {
            second_args.output = new_filename;
        }
    }

    run_with_args(second_args, config, &prompter).unwrap();

    // Restore original directory
    std::env::set_current_dir(original_dir).unwrap();

    let outputs: Vec<_> = fs::read_dir(&output_dir)
        .unwrap()
        .map(|e| e.unwrap().path())
        .collect();
    let latest_output = outputs
        .iter()
        .max_by_key(|p| fs::metadata(p).unwrap().modified().unwrap())
        .unwrap();
    let content = fs::read_to_string(latest_output).unwrap();

    // Should have line numbers (showing new config is active)
    assert!(content.contains("   1 |"));

    // Should not show change summary since cache was invalidated
    assert!(!content.contains("## Change Summary"));
}

#[test]
#[serial]
fn test_concurrent_cache_access() {
    use std::sync::Arc;
    use std::thread;

    let temp_dir = tempdir().unwrap();
    let project_dir = temp_dir.path().join("project");
    create_simple_project(&project_dir).unwrap();

    let output_dir = temp_dir.path().join("output");
    fs::create_dir_all(&output_dir).unwrap();

    let project_dir = Arc::new(project_dir);
    let output_dir = Arc::new(output_dir);

    // Spawn multiple threads that try to run the tool concurrently
    let handles: Vec<_> = (0..3)
        .map(|i| {
            let project_dir = Arc::clone(&project_dir);
            let output_dir = Arc::clone(&output_dir);

            thread::spawn(move || {
                let args = Args {
                    input: project_dir.to_string_lossy().to_string(),
                    output: output_dir
                        .join(format!("context_{}.md", i))
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
                    init: false,
                    max_tokens: None,
                };

                let prompter = TestPrompter;
                run_with_args(args, Config::default(), &prompter)
            })
        })
        .collect();

    // Wait for all threads to complete
    let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();

    // All should succeed (no cache corruption)
    for result in results {
        assert!(
            result.is_ok(),
            "Concurrent access should not cause failures"
        );
    }

    // Check that all outputs were created
    let output_count = fs::read_dir(&*output_dir).unwrap().count();
    assert_eq!(output_count, 3, "All concurrent runs should produce output");
}

#[test]
#[serial]
fn test_corrupted_cache_recovery() {
    let temp_dir = tempdir().unwrap();
    let project_dir = temp_dir.path().join("project");
    create_simple_project(&project_dir).unwrap();

    let output_dir = temp_dir.path().join("output");
    fs::create_dir_all(&output_dir).unwrap();

    // Change to project directory so config loading works
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(&project_dir).unwrap();

    let args = Args {
        input: ".".to_string(), // Use current directory
        output: output_dir.join("context.md").to_string_lossy().to_string(),
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

    let prompter = TestPrompter;

    // First run to create cache
    let config = load_config().unwrap_or_default();

    // Apply config merging manually since we're bypassing run()
    let mut first_args = args.clone();

    // Apply line_numbers from config
    if !first_args.line_numbers
        && let Some(line_numbers) = config.line_numbers
    {
        first_args.line_numbers = line_numbers;
    }

    // Apply diff_only from config
    if !first_args.diff_only
        && let Some(diff_only) = config.diff_only
    {
        first_args.diff_only = diff_only;
    }

    // Apply timestamping manually since we're bypassing run()
    if config.timestamped_output.unwrap_or(false) {
        let timestamp = Utc::now().format("%Y%m%d%H%M%S").to_string();
        let path = std::path::Path::new(&first_args.output);
        let stem = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("output");
        let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("md");
        let new_filename = format!("{}_{}.{}", stem, timestamp, extension);
        if let Some(parent) = path.parent() {
            first_args.output = parent.join(new_filename).to_string_lossy().to_string();
        } else {
            first_args.output = new_filename;
        }
    }

    run_with_args(first_args, config.clone(), &prompter).unwrap();

    // Corrupt the cache by writing invalid JSON
    let cache_dir = project_dir.join(".context-builder").join("cache");
    if cache_dir.exists() {
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
            // Corrupt the first cache file found
            fs::write(cache_files[0].path(), "{ invalid json }").unwrap();
        }
    }

    // Modify a file
    fs::write(
        project_dir.join("src").join("main.rs"),
        "fn main() {\n    println!(\"Recovered!\");\n}",
    )
    .unwrap();

    // Small delay to ensure different timestamps
    std::thread::sleep(std::time::Duration::from_millis(1100));

    // Second run should handle corrupted cache gracefully
    let config = load_config().unwrap_or_default();

    // Apply config merging manually since we're bypassing run()
    let mut second_args = args;

    // Apply line_numbers from config
    if !second_args.line_numbers
        && let Some(line_numbers) = config.line_numbers
    {
        second_args.line_numbers = line_numbers;
    }

    // Apply diff_only from config
    if !second_args.diff_only
        && let Some(diff_only) = config.diff_only
    {
        second_args.diff_only = diff_only;
    }

    // Apply timestamping manually since we're bypassing run()
    if config.timestamped_output.unwrap_or(false) {
        let timestamp = Utc::now().format("%Y%m%d%H%M%S").to_string();
        let path = std::path::Path::new(&second_args.output);
        let stem = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("output");
        let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("md");
        let new_filename = format!("{}_{}.{}", stem, timestamp, extension);
        if let Some(parent) = path.parent() {
            second_args.output = parent.join(new_filename).to_string_lossy().to_string();
        } else {
            second_args.output = new_filename;
        }
    }

    let result = run_with_args(second_args, config, &prompter);
    assert!(result.is_ok(), "Should recover from corrupted cache");

    // Restore original directory
    std::env::set_current_dir(original_dir).unwrap();

    // Should produce output despite cache corruption
    let output_count = fs::read_dir(&output_dir).unwrap().count();
    assert!(
        output_count >= 1,
        "Should produce output even with corrupted cache"
    );
}

#[test]
#[serial]
fn test_diff_only_mode_includes_added_files() {
    let temp_dir = tempdir().unwrap();
    let project_dir = temp_dir.path().join("project");
    create_simple_project(&project_dir).unwrap();

    let output_dir = temp_dir.path().join("output");
    fs::create_dir_all(&output_dir).unwrap();

    // Change to project directory so config loading works
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(&project_dir).unwrap();

    // Create config with auto_diff and diff_only enabled
    fs::write(
        project_dir.join("context-builder.toml"),
        r#"
auto_diff = true
timestamped_output = true
diff_only = true
"#,
    )
    .unwrap();

    let prompter = TestPrompter;

    // First run to establish baseline
    let args = Args {
        input: ".".to_string(),
        output: output_dir.join("context.md").to_string_lossy().to_string(),
        filter: vec!["rs".to_string()],
        ignore: vec![],
        preview: false,
        token_count: false,
        line_numbers: false,
        yes: true,
        diff_only: false, // Will be overridden by config
        clear_cache: false,
        init: false,
        max_tokens: None,
    };

    run_with_args(args.clone(), load_config().unwrap_or_default(), &prompter).unwrap();

    // Add a new file
    fs::write(
        project_dir.join("src").join("new_module.rs"),
        "// New module added\npub fn new_function() -> String {\n    \"Hello from new module\".to_string()\n}\n\n#[cfg(test)]\nmod tests {\n    use super::*;\n\n    #[test]\n    fn test_new_function() {\n        assert_eq!(new_function(), \"Hello from new module\");\n    }\n}\n",
    )
    .unwrap();

    // Small delay to ensure different timestamps
    std::thread::sleep(std::time::Duration::from_millis(1100));

    // Second run with the added file
    let config = load_config().unwrap_or_default();

    // Apply config merging manually since we're bypassing run()
    let mut second_args = args;

    // Apply line_numbers from config
    if !second_args.line_numbers
        && let Some(line_numbers) = config.line_numbers
    {
        second_args.line_numbers = line_numbers;
    }

    // Apply diff_only from config
    if !second_args.diff_only
        && let Some(diff_only) = config.diff_only
    {
        second_args.diff_only = diff_only;
    }

    // Apply timestamping manually since we're bypassing run()
    if config.timestamped_output.unwrap_or(false) {
        let timestamp = Utc::now().format("%Y%m%d%H%M%S").to_string();
        let path = std::path::Path::new(&second_args.output);
        let stem = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("output");
        let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("md");
        let new_filename = format!("{}_{}.{}", stem, timestamp, extension);
        if let Some(parent) = path.parent() {
            second_args.output = parent.join(new_filename).to_string_lossy().to_string();
        } else {
            second_args.output = new_filename;
        }
    }

    run_with_args(second_args, config, &prompter).unwrap();

    // Restore original directory
    std::env::set_current_dir(original_dir).unwrap();

    // Find the latest output file
    let outputs: Vec<_> = fs::read_dir(&output_dir)
        .unwrap()
        .map(|e| e.unwrap().path())
        .collect();
    let latest_output = outputs
        .iter()
        .max_by_key(|p| fs::metadata(p).unwrap().modified().unwrap())
        .unwrap();
    let content = fs::read_to_string(latest_output).unwrap();

    // Should have change summary
    assert!(content.contains("## Change Summary"));

    // Should have added files section (not full Files section)
    assert!(content.contains("## Added Files"));
    assert!(!content.contains("## Files\n"));

    // Should include the full content of the added file (handle Windows path separators)
    assert!(content.contains("### File: `src") && content.contains("new_module.rs`"));
    assert!(content.contains("pub fn new_function() -> String"));
    assert!(content.contains("Hello from new module"));
    assert!(content.contains("_Status: Added_"));

    // Should still have the file tree and header
    assert!(content.contains("## File Tree Structure"));
    assert!(content.contains("# Directory Structure Report"));

    // Should not include full content of existing files (since they're unchanged)
    // The existing main.rs content should not be in the full Files section (handle Windows path separators)
    let main_rs_in_files = content.contains("### File: `src")
        && content.contains("main.rs`")
        && content.contains("Hello, world!");
    assert!(
        !main_rs_in_files,
        "Existing unchanged files should not have full content in diff_only mode"
    );
}
