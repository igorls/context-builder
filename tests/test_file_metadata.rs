//! Per-file Size/Modified lines are opt-in (v0.11 default change).
//!
//! Default output omits them so a checkout or `touch` does not change the
//! document or its content hash. `--file-metadata` and `file_metadata = true`
//! restore the lines. The auto-diff cache compares file bytes, not mtime.

use serial_test::serial;
use std::fs::{self, File};
use std::path::Path;
use std::time::{Duration, SystemTime};
use tempfile::tempdir;

use context_builder::cli::Args;
use context_builder::config::Config;
use context_builder::config_resolver::{ExplicitCli, resolve_final_config};
use context_builder::{Prompter, run_with_args};

struct TestPrompter;

impl Prompter for TestPrompter {
    fn confirm_overwrite(&self, _file_path: &str) -> std::io::Result<bool> {
        Ok(true)
    }
}

fn sample_args(input: &Path, output: &Path, file_metadata: bool) -> Args {
    Args {
        input: input.to_string_lossy().to_string(),
        output: output.to_string_lossy().to_string(),
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
        file_metadata,
    }
}

fn write_project(dir: &Path) {
    fs::create_dir_all(dir.join("src")).unwrap();
    fs::write(dir.join("src/main.rs"), "fn main() {}\n").unwrap();
    fs::write(dir.join("README.md"), "# hi\n").unwrap();
}

fn content_hash_line(text: &str) -> String {
    text.lines()
        .find(|line| line.starts_with("Content hash:"))
        .expect("document should contain a content hash")
        .to_string()
}

fn touch_later(path: &Path) {
    let before = fs::metadata(path).unwrap().modified().unwrap();
    let later = SystemTime::now() + Duration::from_secs(48 * 3600);
    File::options()
        .write(true)
        .open(path)
        .unwrap()
        .set_modified(later)
        .unwrap();
    let after = fs::metadata(path).unwrap().modified().unwrap();
    assert_ne!(before, after, "mtime should actually move");
}

/// Resolve config the same way `run()` does, then execute.
fn run_resolved(args: Args, config: Config) {
    let resolution = resolve_final_config(args, Some(config.clone()), ExplicitCli::default());
    let resolved = &resolution.config;
    let final_args = sample_args(
        Path::new(&resolved.input),
        Path::new(&resolved.output),
        resolved.file_metadata,
    );
    // sample_args rebuilds from the resolved paths and the resolved flag.
    // Carry the rest of the resolved selection through in case a test sets them.
    let final_args = Args {
        filter: resolved.filter.clone(),
        ignore: resolved.ignore.clone(),
        line_numbers: resolved.line_numbers,
        ..final_args
    };
    let final_config = Config {
        auto_diff: Some(resolved.auto_diff),
        diff_context_lines: Some(resolved.diff_context_lines),
        ..config
    };
    run_with_args(final_args, final_config, &TestPrompter).unwrap();
}

#[test]
#[serial]
fn default_output_omits_size_and_modified() {
    let dir = tempdir().unwrap();
    let project = dir.path().join("proj");
    write_project(&project);
    let output = dir.path().join("out.md");

    run_with_args(
        sample_args(&project, &output, false),
        Config::default(),
        &TestPrompter,
    )
    .unwrap();

    let text = fs::read_to_string(&output).unwrap();
    assert!(text.contains("### File:"));
    assert!(
        !text.contains("- Size:"),
        "default output must not emit per-file Size lines:\n{text}"
    );
    assert!(
        !text.contains("- Modified:"),
        "default output must not emit per-file Modified lines:\n{text}"
    );
    assert!(text.contains("Content hash:"));
}

#[test]
#[serial]
fn file_metadata_flag_restores_size_and_modified() {
    let dir = tempdir().unwrap();
    let project = dir.path().join("proj");
    write_project(&project);
    let output = dir.path().join("out.md");

    run_with_args(
        sample_args(&project, &output, true),
        Config::default(),
        &TestPrompter,
    )
    .unwrap();

    let text = fs::read_to_string(&output).unwrap();
    assert!(text.contains("- Size:"));
    assert!(
        text.contains("- Modified:"),
        "flag should restore Modified lines:\n{text}"
    );
    // Standard path formats the timestamp, not Debug SystemTime.
    assert!(text.contains("UTC"));
}

#[test]
#[serial]
fn file_metadata_config_key_restores_lines_and_cli_wins() {
    let dir = tempdir().unwrap();
    let project = dir.path().join("proj");
    write_project(&project);
    let from_config = dir.path().join("from-config.md");
    let from_cli = dir.path().join("from-cli.md");

    // Config opts in; the flag is omitted (false) so the key applies.
    run_resolved(
        sample_args(&project, &from_config, false),
        Config {
            file_metadata: Some(true),
            ..Config::default()
        },
    );
    let text = fs::read_to_string(&from_config).unwrap();
    assert!(text.contains("- Size:"), "config key should restore Size");
    assert!(
        text.contains("- Modified:"),
        "config key should restore Modified"
    );

    // Explicit CLI flag wins over `file_metadata = false`.
    run_resolved(
        sample_args(&project, &from_cli, true),
        Config {
            file_metadata: Some(false),
            ..Config::default()
        },
    );
    let text = fs::read_to_string(&from_cli).unwrap();
    assert!(text.contains("- Size:"));
    assert!(text.contains("- Modified:"));
}

#[test]
#[serial]
fn mtime_only_change_keeps_output_and_hash_stable() {
    let dir = tempdir().unwrap();
    let project = dir.path().join("proj");
    write_project(&project);
    let first_out = dir.path().join("first.md");
    let second_out = dir.path().join("second.md");
    let readme = project.join("README.md");

    run_with_args(
        sample_args(&project, &first_out, false),
        Config::default(),
        &TestPrompter,
    )
    .unwrap();
    let first = fs::read_to_string(&first_out).unwrap();

    touch_later(&readme);

    run_with_args(
        sample_args(&project, &second_out, false),
        Config::default(),
        &TestPrompter,
    )
    .unwrap();
    let second = fs::read_to_string(&second_out).unwrap();

    assert_eq!(
        content_hash_line(&first),
        content_hash_line(&second),
        "content hash must not change when only mtime changes"
    );
    assert_eq!(
        first, second,
        "default output must be byte-identical across an mtime-only change"
    );
}

#[test]
#[serial]
fn auto_diff_cache_ignores_mtime_when_metadata_off() {
    let dir = tempdir().unwrap();
    let project = dir.path().join("proj");
    write_project(&project);
    let first_out = dir.path().join("first.md");
    let second_out = dir.path().join("second.md");

    let config = Config {
        auto_diff: Some(true),
        // Keep the output path stable (no timestamp suffix) so the two
        // documents can be compared directly.
        timestamped_output: Some(false),
        ..Config::default()
    };

    run_resolved(sample_args(&project, &first_out, false), config.clone());
    let first = fs::read_to_string(&first_out).unwrap();
    assert!(
        !first.contains("- Size:"),
        "auto-diff output omits Size by default"
    );
    assert!(
        !first.contains("SystemTime"),
        "auto-diff output must not print Debug mtimes"
    );

    touch_later(&project.join("src/main.rs"));

    run_resolved(sample_args(&project, &second_out, false), config);
    let second = fs::read_to_string(&second_out).unwrap();

    assert!(
        second.contains("## No Changes Detected"),
        "mtime-only change must not be an auto-diff edit:\n{second}"
    );
    assert!(!second.contains("- Size:"));
    assert!(!second.contains("SystemTime"));

    // File tree and file bodies must match. The second run adds a
    // "No Changes Detected" section and a new `**Generated:**` timestamp;
    // neither is the per-file mtime.
    let from_tree = |text: &str| -> String {
        let i = text
            .find("## File Tree Structure")
            .expect("file tree section");
        text[i..].to_string()
    };
    assert_eq!(
        from_tree(&first),
        from_tree(&second),
        "auto-diff file sections must not change when only mtime changes"
    );
}
