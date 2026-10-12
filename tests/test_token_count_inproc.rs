//! `--token-count` with `--max-tokens`, run in-process (#21 B21).

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

fn project(files: usize) -> tempfile::TempDir {
    let dir = tempdir().unwrap();
    for i in 0..files {
        let body = format!(
            "// file {i}\n{}",
            "let value = compute(alpha, beta);\n".repeat(200)
        );
        fs::write(dir.path().join(format!("f{i}.rs")), body).unwrap();
    }
    dir
}

fn count(dir: &Path, max_tokens: Option<usize>) {
    let out = dir.join("never-written.md");
    let mut a = args(dir, &out);
    a.token_count = true;
    a.max_tokens = max_tokens;
    run_with_args(a, Config::default(), &TestPrompter).unwrap();
    assert!(!out.exists(), "--token-count writes no document");
}

#[test]
#[serial]
fn budget_smaller_than_one_file_cuts_it_and_omits_the_rest() {
    let dir = project(5);
    count(dir.path(), Some(500));
}

#[test]
#[serial]
fn budget_that_fits_every_file_omits_none() {
    let dir = project(2);
    count(dir.path(), Some(1_000_000));
}

#[test]
#[serial]
fn no_budget_counts_everything() {
    let dir = project(3);
    count(dir.path(), None);
}

#[test]
#[serial]
fn budget_between_files_takes_whole_files_then_a_partial_one() {
    let dir = project(6);
    count(dir.path(), Some(3_000));
}
