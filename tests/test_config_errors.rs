//! A malformed `context-builder.toml` is a hard error; unknown keys warn (#21 B19).
//! Each case runs in-process (`run_from`) and through the real binary.

use context_builder::run_from;
use serial_test::serial;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use tempfile::tempdir;

struct CwdGuard(PathBuf);

impl CwdGuard {
    fn enter(path: &Path) -> Self {
        let prev = std::env::current_dir().unwrap();
        std::env::set_current_dir(path).unwrap();
        Self(prev)
    }
}

impl Drop for CwdGuard {
    fn drop(&mut self) {
        let _ = std::env::set_current_dir(&self.0);
    }
}

fn project(config: &str) -> tempfile::TempDir {
    let dir = tempdir().unwrap();
    fs::write(dir.path().join("a.txt"), "alpha\n").unwrap();
    fs::write(dir.path().join("context-builder.toml"), config).unwrap();
    dir
}

fn bin(dir: &Path) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_context-builder"))
        .args(["-d", ".", "-o", "-", "-y"])
        .current_dir(dir)
        .output()
        .unwrap()
}

#[test]
#[serial]
fn unclosed_bracket_fails_in_process() {
    let dir = project("filter = [\"rs\"\n");
    let _cwd = CwdGuard::enter(dir.path());
    let err = run_from(["context-builder", "-d", ".", "-y"]).unwrap_err();
    assert!(err.to_string().contains("invalid config"), "{err}");
    assert!(!dir.path().join("output.md").exists());
}

#[test]
fn unclosed_bracket_fails_the_binary_once_without_output() {
    let dir = project("filter = [\"rs\"\n");
    let out = bin(dir.path());
    assert!(!out.status.success());
    assert!(out.stdout.is_empty(), "no document on a bad config");
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert_eq!(stderr.matches("invalid config").count(), 1, "{stderr}");
    assert!(stderr.contains("context-builder.toml"), "{stderr}");
}

#[test]
#[serial]
fn unknown_key_runs_in_process() {
    let dir = project("filters = [\"rs\"]\n");
    let _cwd = CwdGuard::enter(dir.path());
    run_from(["context-builder", "-d", ".", "-y"]).unwrap();
    assert!(dir.path().join("output.md").exists());
}

#[test]
fn unknown_key_warns_with_a_suggestion_and_still_succeeds() {
    let dir = project("filters = [\"rs\"]\n");
    let out = bin(dir.path());
    assert!(out.status.success());
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.contains("unknown key 'filters'"), "{stderr}");
    assert!(stderr.contains("did you mean 'filter'?"), "{stderr}");
    assert!(String::from_utf8_lossy(&out.stdout).contains("alpha"));
}

#[test]
fn valid_config_is_silent() {
    let dir = project("line_numbers = true\n");
    let out = bin(dir.path());
    assert!(out.status.success());
    assert!(!String::from_utf8_lossy(&out.stderr).contains("unknown key"));
}
