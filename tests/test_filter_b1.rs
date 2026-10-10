//! Regression tests for dogfood bug B1: `--filter` spellings such as `.rs`,
//! `*.rs`, and `RS` used to panic in `TypesBuilder::build`, and a filter that
//! matched nothing wrote an empty document with no warning.
//!
//! Repro (from the dogfood report): `context-builder -d . -f .rs`.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use tempfile::tempdir;

fn bin() -> PathBuf {
    // Integration tests live in target/<profile>/deps/. The context-builder
    // binary is built next to that `deps` directory.
    let mut path = std::env::current_exe().expect("current test executable");
    path.pop();
    path.pop();
    path.push("context-builder");
    if cfg!(windows) {
        path.set_extension("exe");
    }
    assert!(
        path.is_file(),
        "context-builder binary not found at {}",
        path.display()
    );
    path
}

fn run(dir: &Path, args: &[&str]) -> std::process::Output {
    Command::new(bin())
        .current_dir(dir)
        .args(args)
        .env_remove("CB_SILENT")
        .output()
        .expect("failed to spawn context-builder")
}

fn stderr_of(output: &std::process::Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

#[test]
fn cli_dotted_glob_and_case_filters_include_rust_files() {
    let dir = tempdir().unwrap();
    let root = dir.path();
    let proj = root.join("proj");
    fs::create_dir_all(proj.join("src")).unwrap();
    fs::write(proj.join("src/a.rs"), "fn main() {}\n").unwrap();
    fs::write(proj.join("README.md"), "# readme\n").unwrap();

    for filter in [".rs", "*.rs", "RS"] {
        let out = root.join(format!("out-{}.md", filter.trim_start_matches(['.', '*'])));
        let output = run(
            root,
            &[
                "-d",
                proj.to_str().unwrap(),
                "-f",
                filter,
                "-o",
                out.to_str().unwrap(),
                "-y",
            ],
        );
        let stderr = stderr_of(&output);
        assert!(
            output.status.success(),
            "filter {filter:?} should succeed, not panic.\nstderr:\n{stderr}"
        );
        assert!(
            !stderr.contains("panicked at"),
            "filter {filter:?} panicked:\n{stderr}"
        );
        assert!(
            !stderr.contains("no files matched"),
            "filter {filter:?} matched a file and should not warn:\n{stderr}"
        );
        let doc = fs::read_to_string(&out).unwrap_or_else(|e| panic!("read {out:?}: {e}"));
        assert!(
            doc.contains("a.rs"),
            "filter {filter:?} should include a.rs, got:\n{doc}"
        );
        assert!(
            !doc.contains("README.md"),
            "filter {filter:?} should exclude README.md, got:\n{doc}"
        );
    }
}

#[test]
fn cli_unrecognized_filter_is_a_clean_error() {
    let dir = tempdir().unwrap();
    let root = dir.path();
    fs::write(root.join("a.rs"), "fn main() {}\n").unwrap();

    for filter in ["d.ts", "c++", "tar.gz"] {
        let out = root.join("should-not-matter.md");
        let output = run(
            root,
            &[
                "-d",
                root.to_str().unwrap(),
                "-f",
                filter,
                "-o",
                out.to_str().unwrap(),
                "-y",
            ],
        );
        let stderr = stderr_of(&output);
        assert!(
            !output.status.success(),
            "filter {filter:?} should fail, stderr:\n{stderr}"
        );
        assert_ne!(
            output.status.code(),
            Some(101),
            "filter {filter:?} panicked (exit 101):\n{stderr}"
        );
        assert!(
            !stderr.contains("panicked at"),
            "filter {filter:?} panicked:\n{stderr}"
        );
        assert!(
            stderr.contains("Unrecognized file type filter"),
            "filter {filter:?} should print a clear error, stderr:\n{stderr}"
        );
        assert!(
            stderr.contains(filter),
            "filter {filter:?} should name the bad filter, stderr:\n{stderr}"
        );
    }
}

#[test]
fn cli_zero_match_filter_warns_on_stderr() {
    let dir = tempdir().unwrap();
    let root = dir.path();
    let proj = root.join("proj");
    fs::create_dir(&proj).unwrap();
    fs::write(proj.join("readme.md"), "# hi\n").unwrap();
    let out = root.join("empty.md");

    let output = run(
        root,
        &[
            "-d",
            proj.to_str().unwrap(),
            "-f",
            "rs",
            "-o",
            out.to_str().unwrap(),
            "-y",
        ],
    );
    let stderr = stderr_of(&output);
    assert!(
        output.status.success(),
        "a zero-match filter should warn, not fail.\nstderr:\n{stderr}"
    );
    assert!(
        stderr.contains("no files matched") && stderr.contains("'rs'"),
        "expected a zero-match warning on stderr, got:\n{stderr}"
    );
    let doc = fs::read_to_string(&out).expect("empty document should still be written");
    assert!(
        doc.contains("Directory Structure Report"),
        "expected the usual document header, got:\n{doc}"
    );
    assert!(
        !doc.contains("readme.md"),
        "the rs filter should not include readme.md, got:\n{doc}"
    );
}
