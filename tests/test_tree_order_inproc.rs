//! In-process runs for the tree-order note (#42), including `--token-count`,
//! which must count the note it will print.

use context_builder::cli::Args;
use context_builder::config::Config;
use context_builder::{Prompter, run_with_args};
use serial_test::serial;
use std::fs;
use std::io;
use std::path::Path;
use tempfile::tempdir;

struct TestPrompter;

impl Prompter for TestPrompter {
    fn confirm_overwrite(&self, _file_path: &str) -> io::Result<bool> {
        Ok(true)
    }
}

fn args(input: &Path, output: &Path) -> Args {
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
        file_metadata: false,
        max_file_size: "256K".to_string(),
        hidden: false,
        include_secrets: false,
        include_lockfiles: false,
        stats: None,
    }
}

fn project() -> tempfile::TempDir {
    let dir = tempdir().unwrap();
    fs::write(dir.path().join("b.rs"), "fn b() {}\n").unwrap();
    fs::write(dir.path().join("a.rs"), "fn a() {}\n").unwrap();
    fs::write(dir.path().join("README.md"), "# readme\n").unwrap();
    dir
}

#[test]
#[serial]
fn full_report_states_the_tree_is_alphabetical() {
    let dir = project();
    let out = dir.path().join("out.md");
    run_with_args(args(dir.path(), &out), Config::default(), &TestPrompter).unwrap();
    let doc = fs::read_to_string(&out).unwrap();
    assert!(doc.contains("alphabetical"), "{doc}");
}

#[test]
#[serial]
fn token_count_run_completes_without_writing_a_report() {
    let dir = project();
    let out = dir.path().join("out.md");
    let mut a = args(dir.path(), &out);
    a.token_count = true;
    run_with_args(a, Config::default(), &TestPrompter).unwrap();
    assert!(!out.exists());
}
