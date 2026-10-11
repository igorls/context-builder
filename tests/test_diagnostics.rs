//! Diagnostics of the built binary: warnings only when relevant, errors once
//! and readable (#21: B13, B17, B18).

use std::fs;
use std::path::Path;
use std::process::{Command, Output};

fn run(dir: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_context-builder"))
        .args(args)
        .current_dir(dir)
        .output()
        .unwrap()
}

fn text(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).into_owned()
}

fn project() -> tempfile::TempDir {
    let dir = tempfile::tempdir().unwrap();
    fs::write(dir.path().join("a.rs"), "fn a() {}\n").unwrap();
    dir
}

#[test]
fn plain_run_does_not_mention_tree_sitter() {
    let dir = project();
    let out = run(dir.path(), &["-o", "-"]);
    assert!(out.status.success());
    let stderr = text(&out.stderr);
    assert!(!stderr.contains("tree-sitter"), "{stderr}");
}

#[cfg(not(feature = "tree-sitter-base"))]
#[test]
fn tree_sitter_warning_only_for_flags_that_need_it_and_suggests_install() {
    let dir = project();
    let signatures = text(&run(dir.path(), &["-o", "-", "--signatures"]).stderr);
    assert!(
        signatures.contains("require tree-sitter support"),
        "{signatures}"
    );
    assert!(
        signatures.contains("cargo install context-builder --features tree-sitter-all"),
        "{signatures}"
    );
    assert!(!signatures.contains("cargo build"), "{signatures}");

    let budget = text(&run(dir.path(), &["-o", "-", "--max-tokens", "5000"]).stderr);
    assert!(budget.contains("require tree-sitter support"), "{budget}");
}

#[cfg(not(feature = "tree-sitter-base"))]
#[test]
fn truncation_notice_reports_the_effective_mode() {
    let dir = tempfile::tempdir().unwrap();
    let body = "pub fn filler() { let _ = 1; }\n".repeat(2_000);
    fs::write(dir.path().join("big.rs"), body).unwrap();
    let out = run(dir.path(), &["-o", "-", "--max-tokens", "2000"]);
    let doc = text(&out.stdout);
    assert!(doc.contains("File content truncated"), "{doc}");
    assert!(doc.contains("(mode: byte)"), "{doc}");
    assert!(!doc.contains("(mode: smart)"), "{doc}");
}

#[test]
fn missing_input_dir_error_is_printed_once_and_readable() {
    let dir = tempfile::tempdir().unwrap();
    let out = run(dir.path(), &["-d", "no-such-dir"]);
    assert_eq!(out.status.code(), Some(1));
    let stderr = text(&out.stderr);
    assert_eq!(stderr.matches("Error:").count(), 1, "{stderr}");
    assert!(stderr.contains("no-such-dir"), "{stderr}");
    assert!(!stderr.contains("Custom {"), "{stderr}");
}

#[test]
fn unwritable_output_error_names_the_path_without_debug_formatting() {
    let dir = project();
    // A regular file used as a directory component cannot be created under.
    fs::write(dir.path().join("blocker"), "x").unwrap();
    let out = run(dir.path(), &["-o", "blocker/out.md"]);
    assert_eq!(out.status.code(), Some(1));
    let stderr = text(&out.stderr);
    assert_eq!(stderr.matches("Error:").count(), 1, "{stderr}");
    assert!(stderr.contains("blocker"), "{stderr}");
    assert!(!stderr.contains("Os {"), "{stderr}");
    assert!(!stderr.contains("Custom {"), "{stderr}");
}

#[cfg(unix)]
#[test]
fn write_failure_after_open_names_the_output_path() {
    let dir = project();
    // `/dev/full` opens fine and fails every write with ENOSPC.
    if !Path::new("/dev/full").exists() {
        return;
    }
    let out = run(dir.path(), &["-o", "/dev/full"]);
    assert_eq!(out.status.code(), Some(1));
    let stderr = text(&out.stderr);
    assert!(stderr.contains("/dev/full"), "{stderr}");
}

#[cfg(not(feature = "tree-sitter-base"))]
#[test]
fn auto_diff_runs_also_get_the_tree_sitter_notice() {
    let dir = project();
    fs::write(
        dir.path().join("context-builder.toml"),
        "output_folder = \"out\"\ntimestamped_output = true\nauto_diff = true\n",
    )
    .unwrap();
    let stderr = text(&run(dir.path(), &["-y", "--signatures"]).stderr);
    assert!(stderr.contains("require tree-sitter support"), "{stderr}");
}
