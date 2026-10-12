//! In-process check of the auto-diff renderer's opt-in `Size`/`Modified`
//! lines (#20 B9). The binary-spawning test in `test_snapshots.rs` covers the
//! CLI path; this one runs the library directly so coverage tools that do not
//! follow child processes see the renderer too.

use context_builder::cli::Args;
use context_builder::config::Config;
use context_builder::{Prompter, run_with_args};
use serial_test::serial;
use std::fs;
use std::path::Path;
use tempfile::tempdir;

struct TestPrompter;

impl Prompter for TestPrompter {
    fn confirm_overwrite(&self, _file_path: &str) -> std::io::Result<bool> {
        Ok(true)
    }
}

fn args(input: &Path, output: &Path, file_metadata: bool) -> Args {
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
        max_file_size: "256K".to_string(),
        hidden: false,
        include_secrets: false,
        include_lockfiles: false,
        stats: None,
    }
}

fn config() -> Config {
    Config {
        auto_diff: Some(true),
        timestamped_output: Some(false),
        ..Config::default()
    }
}

#[test]
#[serial]
fn auto_diff_document_prints_size_and_utc_modified_when_enabled() {
    let dir = tempdir().unwrap();
    let project = dir.path().join("proj");
    fs::create_dir_all(&project).unwrap();
    fs::write(project.join("a.txt"), "one\n").unwrap();
    let out = dir.path().join("out.md");

    run_with_args(args(&project, &out, true), config(), &TestPrompter).unwrap();
    fs::write(project.join("a.txt"), "two\n").unwrap();
    run_with_args(args(&project, &out, true), config(), &TestPrompter).unwrap();

    let doc = fs::read_to_string(&out).unwrap();
    assert!(doc.contains("- Size: 4 bytes"), "{doc}");
    let modified: Vec<&str> = doc
        .lines()
        .filter(|l| l.starts_with("- Modified: ") && !l.contains('`'))
        .collect();
    assert!(!modified.is_empty(), "{doc}");
    assert!(modified.iter().all(|l| l.ends_with(" UTC")), "{modified:?}");
    assert!(!doc.contains("SystemTime"), "{doc}");
}

#[test]
#[serial]
fn auto_diff_document_omits_metadata_by_default() {
    let dir = tempdir().unwrap();
    let project = dir.path().join("proj");
    fs::create_dir_all(&project).unwrap();
    fs::write(project.join("a.txt"), "one\n").unwrap();
    let out = dir.path().join("out.md");

    run_with_args(args(&project, &out, false), config(), &TestPrompter).unwrap();

    let doc = fs::read_to_string(&out).unwrap();
    assert!(!doc.contains("- Size:"), "{doc}");
    assert!(!doc.contains("- Modified:"), "{doc}");
}
