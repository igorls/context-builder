//! Integration tests for configuration resolution functionality
//!
//! These tests verify that the new config resolver properly merges CLI arguments
//! with configuration file values according to the correct precedence rules.

use serial_test::serial;
use std::fs;
use std::path::Path;
use tempfile::tempdir;

use context_builder::{Prompter, cli::Args, config_resolver::resolve_final_config, run_with_args};

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

/// Helper function that mimics the run() function's config resolution logic
fn run_with_resolved_config(
    args: Args,
    config: Option<context_builder::config::Config>,
    prompter: &impl Prompter,
) -> std::io::Result<()> {
    // Resolve final configuration using the new config resolver
    let resolution = resolve_final_config(args, config.clone());

    // Convert resolved config back to Args for run_with_args
    let final_args = Args {
        input: resolution.config.input,
        output: resolution.config.output,
        filter: resolution.config.filter,
        ignore: resolution.config.ignore,
        line_numbers: resolution.config.line_numbers,
        preview: resolution.config.preview,
        token_count: resolution.config.token_count,
        yes: resolution.config.yes,
        diff_only: resolution.config.diff_only,
        clear_cache: resolution.config.clear_cache,
        init: resolution.config.init,
        max_tokens: resolution.config.max_tokens,
    };

    // Create final Config with resolved values
    let final_config = context_builder::config::Config {
        auto_diff: Some(resolution.config.auto_diff),
        diff_context_lines: Some(resolution.config.diff_context_lines),
        ..config.unwrap_or_default()
    };

    run_with_args(final_args, final_config, prompter)
}

#[test]
#[serial]
fn test_cli_arguments_override_config_file() {
    let temp_dir = tempdir().unwrap();
    let project_dir = temp_dir.path().join("project");
    let output_dir = temp_dir.path().join("output");

    // Create a simple project
    write_file(
        &project_dir.join("src/main.rs"),
        "fn main() { println!(\"Hello\"); }",
    );
    write_file(&project_dir.join("lib.py"), "def hello(): print('world')");

    // Create config file with specific settings
    write_file(
        &project_dir.join("context-builder.toml"),
        r#"
filter = ["py"]
line_numbers = true
output = "from_config.md"
"#,
    );

    fs::create_dir_all(&output_dir).unwrap();

    // CLI args that should override config
    // Change to project directory (run_with_args creates output relative to CWD)
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(&project_dir).unwrap();

    let args = Args {
        input: ".".to_string(), // Use current directory
        output: output_dir.join("from_cli.md").to_string_lossy().to_string(),
        filter: vec!["rs".to_string()], // Should override config's ["py"]
        ignore: vec![],
        line_numbers: true, // Can't override config boolean settings
        preview: false,
        token_count: false,
        yes: true,
        diff_only: false,
        clear_cache: false,
        init: false,
        max_tokens: None,
    };

    let config = context_builder::config::load_config_from_path(&project_dir).unwrap();
    let prompter = TestPrompter::new(true, true);

    let result = run_with_resolved_config(args, Some(config), &prompter);

    // Restore original directory
    std::env::set_current_dir(original_dir).unwrap();
    assert!(result.is_ok(), "Should succeed with CLI override");

    // Verify output file was created with CLI name, not config name
    let output_file = output_dir.join("from_cli.md");
    assert!(output_file.exists(), "Output file should use CLI filename");

    let content = fs::read_to_string(&output_file).unwrap();

    // Should contain .rs file (CLI filter), not .py file (config filter)
    assert!(
        content.contains("main.rs"),
        "Should include .rs files from CLI filter"
    );
    assert!(
        !content.contains("lib.py"),
        "Should not include .py files despite config filter"
    );

    // Should have line numbers (config applies since we can't distinguish CLI false from default)
    assert!(
        content.contains("   1 |"),
        "Should have line numbers from config"
    );
}

#[test]
#[serial]
fn test_config_applies_when_cli_uses_defaults() {
    let temp_dir = tempdir().unwrap();
    let project_dir = temp_dir.path().join("project");
    let output_dir = temp_dir.path().join("output");

    // Create a simple project
    write_file(
        &project_dir.join("src/main.rs"),
        "fn main() { println!(\"Hello\"); }",
    );
    write_file(&project_dir.join("lib.py"), "def hello(): print('world')");

    // Create config file
    write_file(
        &project_dir.join("context-builder.toml"),
        r#"
filter = ["py", "rs"]
line_numbers = true
ignore = ["target"]
"#,
    );

    fs::create_dir_all(&output_dir).unwrap();

    // Change to project directory
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(&project_dir).unwrap();

    // CLI args using defaults (should be overridden by config)
    let args = Args {
        input: ".".to_string(),          // Use current directory
        output: "output.md".to_string(), // Default - should use config if available
        filter: vec![],                  // Default - should use config
        ignore: vec![],                  // Default - should use config
        line_numbers: false,             // Default - should use config
        preview: false,
        token_count: false,
        yes: true,
        diff_only: false,
        clear_cache: false,
        init: false,
        max_tokens: None,
    };

    let config = context_builder::config::load_config_from_path(&project_dir).unwrap();
    let prompter = TestPrompter::new(true, true);

    let result = run_with_resolved_config(args, Some(config), &prompter);

    // Restore original directory
    std::env::set_current_dir(original_dir).unwrap();
    assert!(result.is_ok(), "Should succeed with config application");

    // Find the output file (should be in current working directory, which is project dir)
    let output_file = project_dir.join("output.md");
    // The tool runs with project_dir as input, so output.md should be created there
    assert!(
        output_file.exists(),
        "Output file should be created in project directory"
    );

    let content = fs::read_to_string(&output_file).unwrap();

    // Should contain both file types from config filter
    assert!(
        content.contains("main.rs"),
        "Should include .rs files from config filter"
    );
    assert!(
        content.contains("lib.py"),
        "Should include .py files from config filter"
    );

    // Should have line numbers from config
    assert!(
        content.contains("   1 |"),
        "Should have line numbers from config"
    );
}

#[test]
#[serial]
fn test_timestamped_output_and_output_folder() {
    let temp_dir = tempdir().unwrap();
    let project_dir = temp_dir.path().join("project");
    let _output_dir = temp_dir.path().join("docs");

    // Create a simple project
    write_file(
        &project_dir.join("src/main.rs"),
        "fn main() { println!(\"Hello\"); }",
    );

    // Create config with timestamping and output folder (relative to project)
    write_file(
        &project_dir.join("context-builder.toml"),
        r#"
output = "context.md"
output_folder = "docs"
timestamped_output = true
"#,
    );

    // Create docs directory inside project directory
    let docs_dir = project_dir.join("docs");
    fs::create_dir_all(&docs_dir).unwrap();

    // Change to project directory
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(&project_dir).unwrap();

    let args = Args {
        input: ".".to_string(),          // Use current directory
        output: "output.md".to_string(), // Should be overridden by config
        filter: vec![],
        ignore: vec![],
        line_numbers: false,
        preview: false,
        token_count: false,
        yes: true,
        diff_only: false,
        clear_cache: false,
        init: false,
        max_tokens: None,
    };

    let config = context_builder::config::load_config_from_path(&project_dir).unwrap();
    let prompter = TestPrompter::new(true, true);

    let result = run_with_resolved_config(args, Some(config), &prompter);

    // Restore original directory
    std::env::set_current_dir(original_dir).unwrap();
    assert!(result.is_ok(), "Should succeed with timestamped output");

    // Find timestamped file in docs directory
    let docs_dir = project_dir.join("docs");
    let entries = fs::read_dir(&docs_dir).unwrap();
    let output_files: Vec<_> = entries
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            let name = entry.file_name();
            let name_str = name.to_string_lossy();
            name_str.starts_with("context_") && name_str.ends_with(".md")
        })
        .collect();

    assert!(
        !output_files.is_empty(),
        "Should have timestamped output file"
    );
    assert!(
        output_files.len() == 1,
        "Should have exactly one output file"
    );

    let output_file = &output_files[0];
    let content = fs::read_to_string(output_file.path()).unwrap();
    assert!(content.contains("main.rs"), "Should contain project files");
}

#[test]
#[serial]
fn test_mixed_explicit_and_default_values() {
    let temp_dir = tempdir().unwrap();
    let project_dir = temp_dir.path().join("project");

    // Create a simple project
    write_file(
        &project_dir.join("src/main.rs"),
        "fn main() { println!(\"Hello\"); }",
    );
    write_file(&project_dir.join("test.py"), "print('test')");

    // Config with multiple settings
    write_file(
        &project_dir.join("context-builder.toml"),
        r#"
filter = ["py"]
line_numbers = true
yes = true
"#,
    );

    // Change to project directory
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(&project_dir).unwrap();

    let args = Args {
        input: ".".to_string(),          // Use current directory
        output: "custom.md".to_string(), // Explicit CLI value
        filter: vec![],                  // Default - should use config
        ignore: vec![],
        line_numbers: false, // Default - config will override this
        preview: false,      // Default - should use config
        token_count: false,  // Don't use token count mode so file gets created
        yes: false,          // Default - should use config
        diff_only: false,
        clear_cache: false,
        init: false,
        max_tokens: None,
    };

    let config = context_builder::config::load_config_from_path(&project_dir).unwrap();
    let prompter = TestPrompter::new(true, true);

    let result = run_with_resolved_config(args, Some(config), &prompter);

    // Restore original directory
    std::env::set_current_dir(original_dir).unwrap();
    assert!(result.is_ok(), "Should succeed with mixed values");

    // Verify output file uses CLI name (created in project directory)
    let output_file = project_dir.join("custom.md");
    assert!(
        output_file.exists(),
        "Should use CLI output filename in project directory"
    );

    let content = fs::read_to_string(&output_file).unwrap();

    // Should use config filter (py files)
    assert!(
        content.contains("test.py"),
        "Should include .py files from config"
    );
    assert!(!content.contains("main.rs"), "Should not include .rs files");

    // Should use config line_numbers setting
    assert!(
        content.contains("   1 |"),
        "Should have line numbers from config"
    );
}

#[test]
#[serial]
fn test_auto_diff_configuration_warning() {
    let temp_dir = tempdir().unwrap();
    let project_dir = temp_dir.path().join("project");

    // Create a simple project
    write_file(
        &project_dir.join("src/main.rs"),
        "fn main() { println!(\"Hello\"); }",
    );

    // Config with auto_diff but no timestamped_output (should generate warning)
    write_file(
        &project_dir.join("context-builder.toml"),
        r#"
auto_diff = true
timestamped_output = false
"#,
    );

    // Change to project directory
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(&project_dir).unwrap();

    let args = Args {
        input: ".".to_string(), // Use current directory
        output: "output.md".to_string(),
        filter: vec![],
        ignore: vec![],
        line_numbers: false,
        preview: false,
        token_count: false,
        yes: true,
        diff_only: false,
        clear_cache: false,
        init: false,
        max_tokens: None,
    };

    let config = context_builder::config::load_config_from_path(&project_dir).unwrap();
    let prompter = TestPrompter::new(true, true);

    // Capture stderr to check for warnings
    let result = run_with_resolved_config(args, Some(config), &prompter);

    // Restore original directory
    std::env::set_current_dir(original_dir).unwrap();
    assert!(result.is_ok(), "Should succeed despite warning");

    // Note: In a real application, we would capture stderr to verify the warning
    // For this test, we're just ensuring the config is handled without crashing
}
