//! The size warnings must match what the run actually writes (#41).

use std::fs;
use std::path::Path;
use std::process::Command;

/// Six ~110 KB Rust files: each over the 100 KB large-file threshold, and the
/// total (~660 KB, ~165K tokens) over the 128K-token advice threshold.
fn big_fixture(dir: &Path) {
    for i in 0..6 {
        let body = format!(
            "// file {i}\n{}",
            "pub fn filler() { let _ = 1; }\n".repeat(3_700)
        );
        fs::write(dir.join(format!("big{i}.rs")), body).unwrap();
    }
}

fn run(dir: &Path, args: &[&str]) -> String {
    let out = Command::new(env!("CARGO_BIN_EXE_context-builder"))
        .args(["-d", dir.to_str().unwrap(), "-o", "-"])
        .args(args)
        .current_dir(dir)
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8_lossy(&out.stderr).into_owned()
}

#[test]
fn default_run_reports_large_files_and_filter_advice() {
    let dir = tempfile::tempdir().unwrap();
    big_fixture(dir.path());
    let stderr = run(dir.path(), &[]);
    assert!(stderr.contains("large file(s) detected"), "{stderr}");
    assert!(stderr.contains("--filter rs"), "{stderr}");
}

#[test]
fn filter_advice_is_omitted_when_a_filter_is_already_applied() {
    let dir = tempfile::tempdir().unwrap();
    big_fixture(dir.path());
    let stderr = run(dir.path(), &["-f", "rs"]);
    assert!(stderr.contains("recommended limit is 128K"), "{stderr}");
    assert!(
        !stderr.contains("Include only these file types"),
        "advice repeats the filter that was already given:\n{stderr}"
    );
    assert!(stderr.contains("--max-tokens 100000"), "{stderr}");
}

#[test]
fn token_budget_run_has_no_large_file_block() {
    let dir = tempfile::tempdir().unwrap();
    big_fixture(dir.path());
    let stderr = run(dir.path(), &["--max-tokens", "20000"]);
    assert!(!stderr.contains("large file(s) detected"), "{stderr}");
    assert!(!stderr.contains("Total context size"), "{stderr}");
}

#[cfg(feature = "tree-sitter-base")]
#[test]
fn signatures_run_has_no_large_file_block() {
    let dir = tempfile::tempdir().unwrap();
    big_fixture(dir.path());
    let stderr = run(dir.path(), &["--signatures"]);
    assert!(!stderr.contains("large file(s) detected"), "{stderr}");
}

#[test]
fn preview_still_reports_the_input_size() {
    let dir = tempfile::tempdir().unwrap();
    big_fixture(dir.path());
    let stderr = run(dir.path(), &["--preview"]);
    assert!(stderr.contains("large file(s) detected"), "{stderr}");
}
