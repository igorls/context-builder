//! Integration tests for configuration resolution functionality
//!
//! These tests verify that the new config resolver properly merges CLI arguments
//! with configuration file values according to the correct precedence rules.

use serial_test::serial;
use std::fs;
use std::path::Path;
use tempfile::tempdir;

use context_builder::{
    Prompter,
    cli::Args,
    config_resolver::{ExplicitCli, resolve_final_config},
    run_with_args,
};

struct TestPrompter {
    overwrite_response: bool,
}

impl TestPrompter {
    fn new(overwrite_response: bool) -> Self {
        Self { overwrite_response }
    }
}

impl Prompter for TestPrompter {
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
    run_with_resolved_config_explicit(args, config, ExplicitCli::default(), prompter)
}

/// Same as [`run_with_resolved_config`], but with an explicit-flag signal so
/// tests can simulate a user-typed `-o` (including `-o output.md`).
fn run_with_resolved_config_explicit(
    args: Args,
    config: Option<context_builder::config::Config>,
    explicit: ExplicitCli,
    prompter: &impl Prompter,
) -> std::io::Result<()> {
    // Resolve final configuration using the new config resolver
    let resolution = resolve_final_config(args, config.clone(), explicit);

    // Convert resolved config back to Args for run_with_args
    let final_args = Args {
        input: resolution.config.input,
        output: resolution.config.output,
        filter: resolution.config.filter,
        ignore: resolution.config.ignore,
        line_numbers: resolution.config.line_numbers,
        file_metadata: resolution.config.file_metadata,
        preview: resolution.config.preview,
        token_count: resolution.config.token_count,
        yes: resolution.config.yes,
        diff_only: resolution.config.diff_only,
        clear_cache: resolution.config.clear_cache,
        init: resolution.config.init,
        max_tokens: resolution.config.max_tokens,
        signatures: resolution.config.signatures,
        structure: resolution.config.structure,
        truncate: resolution.config.truncate,
        visibility: resolution.config.visibility,
        encoding: resolution.config.encoding,
        max_file_size: resolution.config.max_file_size,
        hidden: resolution.config.hidden,
        include_secrets: resolution.config.include_secrets,
        include_lockfiles: resolution.config.include_lockfiles,
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
        encoding: "o200k_base".to_string(),
        init: false,
        max_tokens: None,
        signatures: false,
        structure: false,
        truncate: "smart".to_string(),
        visibility: "all".to_string(),
        max_file_size: "256K".to_string(),
        hidden: false,
        include_secrets: false,
        file_metadata: false,
        include_lockfiles: false,
    };

    let config = context_builder::config::load_config_from_path(&project_dir).unwrap();
    let prompter = TestPrompter::new(true);

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
        encoding: "o200k_base".to_string(),
        init: false,
        max_tokens: None,
        signatures: false,
        structure: false,
        truncate: "smart".to_string(),
        visibility: "all".to_string(),
        max_file_size: "256K".to_string(),
        hidden: false,
        include_secrets: false,
        file_metadata: false,
        include_lockfiles: false,
    };

    let config = context_builder::config::load_config_from_path(&project_dir).unwrap();
    let prompter = TestPrompter::new(true);

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
        encoding: "o200k_base".to_string(),
        init: false,
        max_tokens: None,
        signatures: false,
        structure: false,
        truncate: "smart".to_string(),
        visibility: "all".to_string(),
        max_file_size: "256K".to_string(),
        hidden: false,
        include_secrets: false,
        file_metadata: false,
        include_lockfiles: false,
    };

    let config = context_builder::config::load_config_from_path(&project_dir).unwrap();
    let prompter = TestPrompter::new(true);

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
        encoding: "o200k_base".to_string(),
        init: false,
        max_tokens: None,
        signatures: false,
        structure: false,
        truncate: "smart".to_string(),
        visibility: "all".to_string(),
        max_file_size: "256K".to_string(),
        hidden: false,
        include_secrets: false,
        file_metadata: false,
        include_lockfiles: false,
    };

    let config = context_builder::config::load_config_from_path(&project_dir).unwrap();
    let prompter = TestPrompter::new(true);

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
        encoding: "o200k_base".to_string(),
        init: false,
        max_tokens: None,
        signatures: false,
        structure: false,
        truncate: "smart".to_string(),
        visibility: "all".to_string(),
        max_file_size: "256K".to_string(),
        hidden: false,
        include_secrets: false,
        file_metadata: false,
        include_lockfiles: false,
    };

    let config = context_builder::config::load_config_from_path(&project_dir).unwrap();
    let prompter = TestPrompter::new(true);

    // Capture stderr to check for warnings
    let result = run_with_resolved_config(args, Some(config), &prompter);

    // Restore original directory
    std::env::set_current_dir(original_dir).unwrap();
    assert!(result.is_ok(), "Should succeed despite warning");

    // Note: In a real application, we would capture stderr to verify the warning
    // For this test, we're just ensuring the config is handled without crashing
}

/// Restores the process working directory even if the test panics.
struct CwdGuard(std::path::PathBuf);

impl Drop for CwdGuard {
    fn drop(&mut self) {
        let _ = std::env::set_current_dir(&self.0);
    }
}

fn args_for(input: &str, output: &str) -> Args {
    Args {
        input: input.to_string(),
        output: output.to_string(),
        filter: vec![],
        ignore: vec![],
        line_numbers: false,
        preview: false,
        token_count: false,
        yes: true,
        diff_only: false,
        clear_cache: false,
        encoding: "o200k_base".to_string(),
        init: false,
        max_tokens: None,
        signatures: false,
        structure: false,
        truncate: "smart".to_string(),
        visibility: "all".to_string(),
        file_metadata: false,
        max_file_size: "256K".to_string(),
        hidden: false,
        include_secrets: false,
        include_lockfiles: false,
    }
}

fn markdown_files(dir: &Path) -> Vec<std::path::PathBuf> {
    if !dir.is_dir() {
        return Vec::new();
    }
    let mut files: Vec<_> = fs::read_dir(dir)
        .unwrap()
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| path.extension().and_then(|ext| ext.to_str()) == Some("md"))
        .collect();
    files.sort();
    files
}

/// B3 repro: config sets `output_folder` + `timestamped_output`, then the tool
/// is invoked from another directory with an explicit `-o`.
#[test]
#[serial]
fn explicit_output_overrides_folder_and_timestamp_from_other_cwd() {
    let temp = tempdir().unwrap();
    let project = temp.path().join("proj");
    let elsewhere = temp.path().join("elsewhere");
    write_file(&project.join("a.rs"), "fn main() {}\n");
    write_file(
        &project.join("context-builder.toml"),
        "output_folder = \"docs\"\ntimestamped_output = true\n",
    );
    fs::create_dir_all(&elsewhere).unwrap();

    let original = std::env::current_dir().unwrap();
    let _guard = CwdGuard(original);
    std::env::set_current_dir(&elsewhere).unwrap();

    let config = context_builder::config::load_config_from_path(&project).unwrap();
    let prompter = TestPrompter::new(true);
    let explicit = ExplicitCli {
        output: true,
        ..ExplicitCli::default()
    };
    let project_arg = project.to_string_lossy().into_owned();

    // Absolute `-o /…/wanted.md` must be created at that path, not
    // `./docs/wanted_<ts>.md` relative to cwd.
    let wanted = temp.path().join("wanted.md");
    let result = run_with_resolved_config_explicit(
        args_for(&project_arg, &wanted.to_string_lossy()),
        Some(config.clone()),
        explicit,
        &prompter,
    );
    assert!(result.is_ok(), "absolute -o should succeed: {result:?}");
    assert!(
        wanted.is_file(),
        "explicit absolute -o was not created at {}",
        wanted.display()
    );
    let body = fs::read_to_string(&wanted).unwrap();
    assert!(
        body.contains("fn main"),
        "output should contain the project"
    );
    assert!(
        !elsewhere.join("docs").exists(),
        "must not write cwd/docs when -o is explicit"
    );
    assert!(
        markdown_files(&project.join("docs")).is_empty(),
        "must not write project/docs when -o is explicit"
    );

    // Relative `-o rel-wanted.md` is used verbatim (created in cwd), not
    // rewritten to `docs/rel-wanted_<ts>.md`.
    let result = run_with_resolved_config_explicit(
        args_for(&project_arg, "rel-wanted.md"),
        Some(config.clone()),
        explicit,
        &prompter,
    );
    assert!(result.is_ok(), "relative -o should succeed: {result:?}");
    let relative = elsewhere.join("rel-wanted.md");
    assert!(
        relative.is_file(),
        "explicit relative -o was not created at {}",
        relative.display()
    );
    assert!(
        !elsewhere.join("docs").exists(),
        "relative -o must not be placed in cwd/docs"
    );
    assert!(markdown_files(&project.join("docs")).is_empty());

    // `-o -` stays stdout: no file named `-`, and still no docs folder.
    let result = run_with_resolved_config_explicit(
        args_for(&project_arg, "-"),
        Some(config),
        explicit,
        &prompter,
    );
    assert!(result.is_ok(), "stdout -o should succeed: {result:?}");
    assert!(!elsewhere.join("-").exists());
    assert!(!Path::new("-").exists());
    assert!(!elsewhere.join("docs").exists());
}

/// B20: `output_folder = "docs"` is relative to the project (`-d`), not cwd.
#[test]
#[serial]
fn output_folder_resolves_against_project_root_not_cwd() {
    let temp = tempdir().unwrap();
    let project = temp.path().join("repo");
    let elsewhere = temp.path().join("elsewhere");
    write_file(&project.join("a.rs"), "fn main() {}\n");
    write_file(
        &project.join("context-builder.toml"),
        "output_folder = \"docs\"\ntimestamped_output = true\n",
    );
    fs::create_dir_all(&elsewhere).unwrap();

    let original = std::env::current_dir().unwrap();
    let _guard = CwdGuard(original);
    std::env::set_current_dir(&elsewhere).unwrap();

    let config = context_builder::config::load_config_from_path(&project).unwrap();
    let prompter = TestPrompter::new(true);

    // No `-o`: clap default `output.md`, ExplicitCli::default().
    let result = run_with_resolved_config(
        args_for(&project.to_string_lossy(), "output.md"),
        Some(config),
        &prompter,
    );
    assert!(result.is_ok(), "default output should succeed: {result:?}");

    let written = markdown_files(&project.join("docs"));
    assert_eq!(
        written.len(),
        1,
        "expected one timestamped file under the project docs dir, found {written:?}"
    );
    let name = written[0].file_name().unwrap().to_string_lossy();
    assert!(
        name.starts_with("output_") && name.ends_with(".md"),
        "timestamped name under project root, got {name}"
    );
    let body = fs::read_to_string(&written[0]).unwrap();
    assert!(body.contains("fn main"));
    assert!(
        !elsewhere.join("docs").exists(),
        "output_folder must not be created relative to cwd ({})",
        elsewhere.join("docs").display()
    );
}
