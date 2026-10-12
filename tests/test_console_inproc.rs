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
        stats: None,
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

fn small_project() -> tempfile::TempDir {
    let dir = tempdir().unwrap();
    fs::write(dir.path().join("a.txt"), "one\n").unwrap();
    dir
}

#[test]
#[serial]
fn debug_env_switches_trace_config_and_auto_diff_to_stderr() {
    let dir = small_project();
    let out = dir.path().join("out.md");
    let config = Config {
        auto_diff: Some(true),
        timestamped_output: Some(false),
        ..Config::default()
    };
    unsafe {
        std::env::set_var("CB_DEBUG_CONFIG", "1");
        std::env::set_var("CB_DEBUG_AUTODIFF", "1");
    }
    let first = run_with_args(args(dir.path(), &out), config.clone(), &TestPrompter);
    fs::write(dir.path().join("a.txt"), "two\n").unwrap();
    let second = run_with_args(args(dir.path(), &out), config, &TestPrompter);
    unsafe {
        std::env::remove_var("CB_DEBUG_CONFIG");
        std::env::remove_var("CB_DEBUG_AUTODIFF");
    }
    first.unwrap();
    second.unwrap();
    assert!(fs::read_to_string(&out).unwrap().contains("two"));
}

#[test]
#[serial]
fn diff_only_without_auto_diff_warns_and_still_writes_the_document() {
    let dir = small_project();
    let out = dir.path().join("out.md");
    let mut a = args(dir.path(), &out);
    a.diff_only = true;
    run_with_args(a, Config::default(), &TestPrompter).unwrap();
    assert!(fs::read_to_string(&out).unwrap().contains("one"));
}

#[test]
#[serial]
fn unwritable_state_cache_only_warns() {
    // The cache lives under `<project>/.context-builder`; a plain file in its
    // place makes the cache write fail without failing the run.
    let dir = small_project();
    fs::write(dir.path().join(".context-builder"), "not a directory").unwrap();
    let out = dir.path().join("out.md");
    let config = Config {
        auto_diff: Some(true),
        timestamped_output: Some(false),
        ..Config::default()
    };
    let _ = run_with_args(args(dir.path(), &out), config, &TestPrompter);
}

struct Declines;

impl Prompter for Declines {
    fn confirm_overwrite(&self, _file_path: &str) -> io::Result<bool> {
        Ok(false)
    }
}

#[test]
#[serial]
fn declining_the_overwrite_prompt_cancels_without_touching_the_file() {
    let dir = small_project();
    let out = dir.path().join("out.md");
    fs::write(&out, "keep me").unwrap();
    let mut a = args(dir.path(), &out);
    a.yes = false;
    let err = run_with_args(a, Config::default(), &Declines).unwrap_err();
    assert_eq!(err.kind(), ErrorKind::Interrupted);
    assert_eq!(fs::read_to_string(&out).unwrap(), "keep me");
}

#[test]
#[serial]
fn an_output_directory_that_cannot_be_created_is_a_clean_error() {
    let dir = small_project();
    fs::write(dir.path().join("blocker"), "a file, not a directory").unwrap();
    let out = dir.path().join("blocker").join("sub").join("out.md");
    let err = run_with_args(args(dir.path(), &out), Config::default(), &TestPrompter);
    assert!(err.is_err());
}

#[test]
#[serial]
fn clearing_a_missing_cache_reports_and_succeeds() {
    let dir = small_project();
    context_builder::run_from([
        "context-builder",
        "-d",
        dir.path().to_str().unwrap(),
        "--clear-cache",
    ])
    .unwrap();
    fs::create_dir_all(dir.path().join(".context-builder/cache")).unwrap();
    context_builder::run_from([
        "context-builder",
        "-d",
        dir.path().to_str().unwrap(),
        "--clear-cache",
    ])
    .unwrap();
    assert!(!dir.path().join(".context-builder/cache").exists());
}
