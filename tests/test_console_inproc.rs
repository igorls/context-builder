//! In-process runs through the console output paths and the closed-pipe
//! handling (#40). `test_closed_pipes.rs` spawns the real binary; these call
//! the library directly so coverage tools that do not follow child processes
//! see the same code.

use context_builder::cli::Args;
use context_builder::config::Config;
use context_builder::{Prompter, finish, run_with_args};
use serial_test::serial;
use std::fs;
use std::io::{self, ErrorKind};
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
        max_file_size: "10M".to_string(),
        hidden: false,
        include_secrets: false,
        include_lockfiles: false,
    }
}

fn big_project() -> tempfile::TempDir {
    let dir = tempdir().unwrap();
    for i in 0..3 {
        let mut body = String::new();
        let mut n = 0;
        while body.len() < 200 * 1024 {
            body.push_str(&format!(
                "pub fn function_{i}_{n}(a: u32) -> u32 {{ a + {n} }}\n"
            ));
            n += 1;
        }
        fs::write(dir.path().join(format!("big{i}.rs")), body).unwrap();
    }
    dir
}

#[test]
fn finish_treats_a_closed_pipe_as_success_and_keeps_other_errors() {
    assert!(finish(Ok(())).is_ok());
    assert!(finish(Err(io::Error::from(ErrorKind::BrokenPipe))).is_ok());
    let err = finish(Err(io::Error::from(ErrorKind::NotFound))).unwrap_err();
    assert_eq!(err.kind(), ErrorKind::NotFound);
}

#[test]
#[serial]
fn full_run_reports_progress_and_the_context_window_advice() {
    let dir = big_project();
    let out = dir.path().join("out.md");
    run_with_args(args(dir.path(), &out), Config::default(), &TestPrompter).unwrap();
    assert!(fs::read_to_string(&out).unwrap().contains("function_0_0"));
}

#[test]
#[serial]
fn token_count_mode_prints_totals_and_writes_nothing() {
    let dir = big_project();
    let out = dir.path().join("out.md");
    let mut a = args(dir.path(), &out);
    a.token_count = true;
    run_with_args(a, Config::default(), &TestPrompter).unwrap();
    assert!(!out.exists());
}

#[test]
#[serial]
fn token_budget_run_stops_at_the_budget() {
    let dir = big_project();
    let out = dir.path().join("out.md");
    let mut a = args(dir.path(), &out);
    a.max_tokens = Some(1_500);
    run_with_args(a, Config::default(), &TestPrompter).unwrap();
    let doc = fs::read_to_string(&out).unwrap();
    assert!(doc.contains("Token budget (1500) reached"), "{doc}");
}

#[test]
#[serial]
fn stdout_output_mode_streams_the_document() {
    let dir = big_project();
    let mut a = args(dir.path(), Path::new("-"));
    a.output = "-".to_string();
    run_with_args(a, Config::default(), &TestPrompter).unwrap();
}
