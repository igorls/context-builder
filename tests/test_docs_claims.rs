//! Behaviour the README and `--help` describe (#21 docs/friction).

use std::process::Command;

fn run(dir: &std::path::Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_context-builder"))
        .args(args)
        .current_dir(dir)
        .output()
        .unwrap()
}

#[test]
fn help_documents_stdout_output() {
    let dir = tempfile::tempdir().unwrap();
    let help = String::from_utf8_lossy(&run(dir.path(), &["--help"]).stdout).into_owned();
    assert!(help.contains("`-` writes the document to stdout"), "{help}");
}

#[test]
fn bare_invocation_prints_help_and_writes_nothing() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("a.rs"), "fn a() {}\n").unwrap();
    let out = run(dir.path(), &[]);
    assert!(out.status.success());
    assert!(String::from_utf8_lossy(&out.stdout).contains("Usage:"));
    assert!(!dir.path().join("output.md").exists());
}

#[test]
fn readme_basic_usage_command_processes_the_current_directory() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("a.rs"), "fn a() {}\n").unwrap();
    let out = run(dir.path(), &["-d", "."]);
    assert!(out.status.success());
    let doc = std::fs::read_to_string(dir.path().join("output.md")).unwrap();
    assert!(doc.contains("a.rs"));
}
