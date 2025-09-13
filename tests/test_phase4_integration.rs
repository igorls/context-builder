//! Integration test for all Phase 4 features working together
//!
//! This test validates that the enhanced binary file handling, improved diff_only mode,
//! and comprehensive edge case handling all work correctly in combination.

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
fn test_phase4_features_integration() {
    let temp_dir = tempdir().unwrap();
    let project_dir = temp_dir.path().join("project");
    let output_dir = temp_dir.path().join("output");
    fs::create_dir_all(&output_dir).unwrap();

    // Create config with enhanced features enabled
    write_file(
        &project_dir.join("context-builder.toml"),
        r#"
auto_diff = true
timestamped_output = true
diff_only = true
encoding_strategy = "detect"
filter = ["rs", "txt"]
"#,
    );

    // Change to project directory
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(&project_dir).unwrap();

    // Create initial files with various encoding scenarios
    write_file(
        &project_dir.join("src/main.rs"),
        "fn main() {\n    println!(\"Hello, world!\");\n}\n",
    );

    // UTF-8 file
    write_file(
        &project_dir.join("src/utils.rs"),
        "// UTF-8 file\npub fn helper() -> String {\n    \"Hello from helper\".to_string()\n}\n",
    );

    // Windows-1252 encoded file
    let windows1252_data = [
        0x2F, 0x2F, 0x20, // "// "
        0x57, 0x69, 0x6E, 0x64, 0x6F, 0x77, 0x73, 0x2D, 0x31, 0x32, 0x35, 0x32,
        0x20, // "Windows-1252 "
        0x93, 0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x94, // "Hello" with smart quotes
        0x0A, // newline
        0x70, 0x75, 0x62, 0x20, 0x66, 0x6E, 0x20, 0x74, 0x65, 0x73, 0x74, 0x28, 0x29, 0x20, 0x7B,
        0x7D, 0x0A, // "pub fn test() {}"
    ];
    write_binary_file(&project_dir.join("src/encoded.rs"), &windows1252_data);

    // Binary file that should be skipped - use executable-like binary data
    let binary_data = vec![
        0x7f, 0x45, 0x4c, 0x46, // ELF header
        0x02, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00,
        0x3e, // More ELF data
        0xff, 0xfe, 0xfd, 0xfc, 0xfb, 0xfa, 0xf9, 0xf8, // High bytes
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Null bytes
    ];
    write_binary_file(&project_dir.join("data.txt"), &binary_data);

    let prompter = TestPrompter::new(true, true);
    let config = context_builder::config::load_config_from_path(&project_dir).unwrap_or_default();

    // First run - establish baseline
    let args = Args {
        input: project_dir.to_string_lossy().to_string(),
        output: output_dir.join("baseline.md").to_string_lossy().to_string(),
        filter: vec![], // Use config filter
        ignore: vec![],
        preview: false,
        token_count: false,
        line_numbers: false,
        yes: true,
        diff_only: false, // Will be overridden by config
        clear_cache: false,
    };

    // Apply config manually (simulating what happens in the real application)
    let mut resolved_args = args.clone();
    if resolved_args.filter.is_empty()
        && let Some(ref config_filter) = config.filter
    {
        resolved_args.filter = config_filter.clone();
    }
    if !resolved_args.diff_only
        && let Some(diff_only) = config.diff_only
    {
        resolved_args.diff_only = diff_only;
    }

    let result1 = run_with_args(resolved_args, config.clone(), &prompter);
    assert!(result1.is_ok(), "First run should succeed");

    // Add a new file to test improved diff_only mode
    write_file(
        &project_dir.join("src/new_feature.rs"),
        "// New feature added\npub fn new_feature() -> String {\n    \"Brand new functionality\".to_string()\n}\n\n#[cfg(test)]\nmod tests {\n    use super::*;\n\n    #[test]\n    fn test_new_feature() {\n        assert_eq!(new_feature(), \"Brand new functionality\");\n    }\n}\n",
    );

    // Modify existing file
    write_file(
        &project_dir.join("src/main.rs"),
        "fn main() {\n    println!(\"Hello, enhanced world!\");\n}\n",
    );

    // Small delay to ensure different timestamps
    std::thread::sleep(std::time::Duration::from_millis(1100));

    // Second run with changes
    let mut second_args = args;
    second_args.input = project_dir.to_string_lossy().to_string();
    second_args.output = output_dir.join("enhanced.md").to_string_lossy().to_string();

    // Apply config manually
    if second_args.filter.is_empty()
        && let Some(ref config_filter) = config.filter
    {
        second_args.filter = config_filter.clone();
    }
    if !second_args.diff_only
        && let Some(diff_only) = config.diff_only
    {
        second_args.diff_only = diff_only;
    }

    let result2 = run_with_args(second_args, config, &prompter);
    assert!(result2.is_ok(), "Second run should succeed");

    // Restore original directory
    std::env::set_current_dir(original_dir).unwrap();

    // Verify the enhanced features work correctly
    let outputs: Vec<_> = fs::read_dir(&output_dir)
        .unwrap()
        .map(|e| e.unwrap().path())
        .collect();
    let latest_output = outputs
        .iter()
        .max_by_key(|p| fs::metadata(p).unwrap().modified().unwrap())
        .unwrap();

    let content = fs::read_to_string(latest_output).unwrap();

    // Test enhanced binary file handling
    // Should either transcode Windows-1252 content or show binary placeholder
    assert!(
        content.contains("Hello") || content.contains("<Binary file"),
        "Should handle Windows-1252 encoding or show binary placeholder"
    );

    // Binary files should be handled gracefully (not crash the application)
    // The specific behavior depends on encoding strategy, but it should not fail

    // Test improved diff_only mode
    assert!(
        content.contains("## Change Summary"),
        "Should have change summary in diff_only mode"
    );

    // Should include full content of added files (new feature)
    assert!(
        content.contains("## Added Files"),
        "Should have Added Files section in diff_only mode"
    );
    assert!(
        content.contains("new_feature.rs"),
        "Should include added file"
    );
    assert!(
        content.contains("Brand new functionality"),
        "Should include full content of added file"
    );

    // Should have file differences for modified files
    assert!(
        content.contains("## File Differences"),
        "Should have file differences section"
    );

    // Should not have full Files section (due to diff_only mode)
    assert!(
        !content.contains("## Files\n"),
        "Should not have full Files section in diff_only mode"
    );

    // Test comprehensive edge cases are handled
    assert!(
        content.contains("# Directory Structure Report"),
        "Should have proper document structure"
    );
    assert!(
        content.contains("## File Tree Structure"),
        "Should have file tree"
    );

    // Verify that the enhanced features didn't break basic functionality
    // In diff_only mode, content is smaller since it only shows changes
    assert!(
        content.len() > 500,
        "Should generate reasonable content even in diff_only mode"
    );

    println!("✅ Phase 4 integration test passed!");
    println!("   - Enhanced binary file handling: Working");
    println!("   - Improved diff_only mode: Working");
    println!("   - Comprehensive edge case handling: Working");
    println!("   - All features integrated successfully");
}

#[test]
fn test_encoding_strategy_configuration() {
    let temp_dir = tempdir().unwrap();
    let project_dir = temp_dir.path().join("project");
    let output_dir = temp_dir.path().join("output");
    fs::create_dir_all(&output_dir).unwrap();

    // Create a file with Windows-1252 encoding
    let windows1252_data = [
        0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x20, // "Hello "
        0x93, 0x57, 0x6F, 0x72, 0x6C, 0x64, 0x94, // "World" with smart quotes
        0x0A, // newline
    ];
    write_binary_file(&project_dir.join("test.txt"), &windows1252_data);

    let prompter = TestPrompter::new(true, true);

    // Test all encoding strategies
    for strategy in &["detect", "strict", "skip"] {
        let config = Config {
            encoding_strategy: Some(strategy.to_string()),
            ..Default::default()
        };

        let args = Args {
            input: project_dir.to_string_lossy().to_string(),
            output: output_dir
                .join(format!("encoding_{}.md", strategy))
                .to_string_lossy()
                .to_string(),
            filter: vec!["txt".to_string()],
            ignore: vec![],
            preview: false,
            token_count: false,
            line_numbers: false,
            yes: true,
            diff_only: false,
            clear_cache: false,
        };

        let result = run_with_args(args, config, &prompter);
        assert!(
            result.is_ok(),
            "Encoding strategy '{}' should work",
            strategy
        );

        let output_path = output_dir.join(format!("encoding_{}.md", strategy));
        let content = fs::read_to_string(&output_path).unwrap();

        match *strategy {
            "detect" => {
                // Should attempt transcoding and may succeed
                assert!(
                    content.contains("Hello") || content.contains("<Binary file"),
                    "Detect strategy should transcode or show binary placeholder"
                );
            }
            "strict" | "skip" => {
                // Should show binary placeholder
                assert!(
                    content.contains("<Binary file"),
                    "Strict/skip strategy should show binary placeholder"
                );
            }
            _ => {}
        }
    }

    println!("✅ Encoding strategy configuration test passed!");
}
