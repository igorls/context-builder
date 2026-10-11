//! The whole command-line path (`run_from`) in-process: argument parsing,
//! config resolution, `--init`, `--clear-cache`, help, and a real run. The
//! spawn-the-binary tests cover the same behavior from the outside; these keep
//! it visible to coverage tools that do not follow child processes.

use context_builder::run_from;
use serial_test::serial;
use std::fs;
use std::path::{Path, PathBuf};
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

fn project() -> tempfile::TempDir {
    let dir = tempdir().unwrap();
    fs::write(dir.path().join("a.txt"), "alpha\n").unwrap();
    dir
}

#[test]
#[serial]
fn explicit_run_writes_the_document() {
    let dir = project();
    let out = dir.path().join("out.md");
    let _cwd = CwdGuard::enter(dir.path());
    run_from([
        "context-builder",
        "-d",
        dir.path().to_str().unwrap(),
        "-o",
        out.to_str().unwrap(),
        "-y",
        "--encoding",
        "cl100k_base",
        "--truncate",
        "byte",
        "--visibility",
        "public",
        "--max-file-size",
        "1M",
    ])
    .unwrap();
    assert!(fs::read_to_string(out).unwrap().contains("alpha"));
}

#[test]
#[serial]
fn bare_invocation_without_a_config_prints_help_and_succeeds() {
    let dir = project();
    let _cwd = CwdGuard::enter(dir.path());
    run_from(["context-builder"]).unwrap();
    assert!(!dir.path().join("output.md").exists());
}

#[test]
#[serial]
fn bad_command_line_is_an_invalid_input_error() {
    let err = run_from(["context-builder", "--no-such-flag"]).unwrap_err();
    assert_eq!(err.kind(), std::io::ErrorKind::InvalidInput);
}

#[test]
#[serial]
fn init_writes_a_config_and_refuses_to_replace_one() {
    let dir = project();
    let _cwd = CwdGuard::enter(dir.path());
    run_from(["context-builder", "--init"]).unwrap();
    let cfg = dir.path().join("context-builder.toml");
    assert!(cfg.exists());
    let first = fs::read_to_string(&cfg).unwrap();
    run_from(["context-builder", "--init"]).unwrap();
    assert_eq!(fs::read_to_string(&cfg).unwrap(), first, "not replaced");
}

#[test]
#[serial]
fn clear_cache_removes_the_cache_directory_or_says_there_is_none() {
    let dir = project();
    let _cwd = CwdGuard::enter(dir.path());
    let d = dir.path().to_str().unwrap();
    run_from(["context-builder", "-d", d, "--clear-cache"]).unwrap();

    let cache = dir.path().join(".context-builder").join("cache");
    fs::create_dir_all(&cache).unwrap();
    fs::write(cache.join("state.json"), "{}").unwrap();
    run_from(["context-builder", "-d", d, "--clear-cache"]).unwrap();
    assert!(!cache.exists());
}

#[test]
#[serial]
fn project_config_with_warnings_still_runs() {
    let dir = project();
    fs::write(
        dir.path().join("context-builder.toml"),
        "line_numbers = true\ndiff_only = true\n",
    )
    .unwrap();
    let out = dir.path().join("out.md");
    let _cwd = CwdGuard::enter(dir.path());
    run_from([
        "context-builder",
        "-d",
        dir.path().to_str().unwrap(),
        "-o",
        out.to_str().unwrap(),
        "-y",
    ])
    .unwrap();
    assert!(out.exists());
}

#[test]
fn help_and_version_succeed_instead_of_being_invalid_input() {
    run_from(["context-builder", "--help"]).unwrap();
    run_from(["context-builder", "--version"]).unwrap();
    let err = run_from(["context-builder", "--definitely-not-a-flag"]).unwrap_err();
    assert_eq!(err.kind(), std::io::ErrorKind::InvalidInput);
}
