//! Regression tests for inclusion bugs B6, B7, and B8.
//!
//! B6: `.gitignore` and `target/` (CACHEDIR.TAG) without a `.git` directory.
//! B7: a previous context-builder report is not re-ingested on a later run.
//! B8: the default output name does not hide a nested `docs/output.md`.

use context_builder::cli::Args;
use context_builder::config::Config;
use context_builder::{Prompter, run_with_args};
use serial_test::serial;
use std::fs;
use std::path::{Path, PathBuf};

use tempfile::tempdir;

struct TestPrompter;

impl Prompter for TestPrompter {
    fn confirm_processing(&self, _file_count: usize) -> std::io::Result<bool> {
        Ok(true)
    }

    fn confirm_overwrite(&self, _file_path: &str) -> std::io::Result<bool> {
        Ok(true)
    }
}

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

fn args(input: impl Into<String>, output: impl Into<String>) -> Args {
    Args {
        input: input.into(),
        output: output.into(),
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
    }
}

fn run(input: impl Into<String>, output: impl Into<String>) -> std::io::Result<()> {
    run_with_args(args(input, output), Config::default(), &TestPrompter)
}

fn file_sections(content: &str) -> Vec<&str> {
    content
        .lines()
        .filter(|line| line.starts_with("### File: "))
        .collect()
}

const CACHEDIR_TAG_BODY: &str = "Signature: 8a477f597d28d172789f06886806bc55\n# This file is a cache directory tag created by cargo.\n";

/// Repro: a built tree copied without `.git` (dogfood B7 / report B6).
/// `.gitignore` must apply, and `target/` must not be walked.
#[test]
fn no_git_directory_respects_gitignore_and_skips_target() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    fs::create_dir_all(root.join("src")).unwrap();
    fs::write(root.join("src/main.rs"), "fn main() { /* KEEP_MAIN */ }\n").unwrap();
    fs::write(root.join("keep.txt"), "KEEP_TXT\n").unwrap();
    // No `.git` directory.
    fs::write(root.join(".gitignore"), "secret.log\ngenerated/\n").unwrap();
    fs::write(root.join("secret.log"), "SECRET_LOG\n").unwrap();
    fs::create_dir_all(root.join("generated")).unwrap();
    fs::write(root.join("generated/junk.txt"), "GENERATED_JUNK\n").unwrap();

    fs::create_dir_all(root.join("target/debug")).unwrap();
    fs::write(root.join("target/CACHEDIR.TAG"), CACHEDIR_TAG_BODY).unwrap();
    fs::write(root.join("target/debug/x.d"), "TARGET_DEP\n").unwrap();

    fs::create_dir_all(root.join("my-cache")).unwrap();
    fs::write(root.join("my-cache/CACHEDIR.TAG"), CACHEDIR_TAG_BODY).unwrap();
    fs::write(root.join("my-cache/blob.txt"), "CACHE_BLOB\n").unwrap();

    let output = root.join("out.md");
    run(root.to_string_lossy(), output.to_string_lossy()).unwrap();
    let content = fs::read_to_string(&output).unwrap();

    assert!(
        content.contains("KEEP_MAIN"),
        "source should be included:\n{content}"
    );
    assert!(content.contains("KEEP_TXT"));
    assert!(
        !content.contains("SECRET_LOG"),
        ".gitignore was ignored without a .git directory:\n{content}"
    );
    assert!(!content.contains("GENERATED_JUNK"));
    assert!(
        !content.contains("TARGET_DEP"),
        "target/ was included:\n{content}"
    );
    assert!(
        !content.contains("CACHE_BLOB"),
        "CACHEDIR.TAG directory was included:\n{content}"
    );
    assert!(!content.contains("x.d"));
}

/// Repro: `context-builder -d . -y` then `context-builder -d . -o other.md`.
#[test]
fn previous_output_is_not_reingested() {
    let dir = tempdir().unwrap();
    let root = dir.path();
    fs::write(root.join("a.txt"), "hello-source-b7\n").unwrap();

    let first = root.join("output.md");
    run(root.to_string_lossy(), first.to_string_lossy()).unwrap();
    let first_body = fs::read_to_string(&first).unwrap();
    assert!(
        first_body.starts_with("# Directory Structure Report\n"),
        "first run should write the report header"
    );
    assert!(
        first_body
            .lines()
            .any(|line| line.starts_with("Content hash: ")),
        "first run should write a content hash line"
    );
    assert!(first_body.contains("hello-source-b7"));

    let second = root.join("other.md");
    run(root.to_string_lossy(), second.to_string_lossy()).unwrap();
    let second_body = fs::read_to_string(&second).unwrap();
    let sections = file_sections(&second_body);

    assert!(
        second_body.contains("hello-source-b7"),
        "source file should still be included"
    );
    assert!(
        sections.contains(&"### File: `a.txt`"),
        "expected a.txt, got {sections:?}"
    );
    assert!(
        !sections.contains(&"### File: `output.md`"),
        "previous output.md was re-ingested: {sections:?}"
    );
    let hash_lines = second_body
        .lines()
        .filter(|line| line.starts_with("Content hash: "))
        .count();
    assert_eq!(
        hash_lines, 1,
        "previous report header was copied into the new output"
    );
}

/// The resolved output is `<project>/output.md`. A nested `docs/output.md`
/// must stay; only that exact output path is auto-ignored.
#[test]
fn nested_output_md_kept_when_output_is_project_root() {
    let dir = tempdir().unwrap();
    let root = dir.path();
    fs::create_dir_all(root.join("docs")).unwrap();
    fs::write(root.join("docs/output.md"), "real doc NESTED_DOC_MARKER\n").unwrap();
    fs::write(root.join("a.txt"), "source-file\n").unwrap();

    let output = root.join("output.md");
    run(root.to_string_lossy(), output.to_string_lossy()).unwrap();
    let content = fs::read_to_string(&output).unwrap();
    let sections = file_sections(&content);

    assert!(
        sections.contains(&"### File: `docs/output.md`"),
        "nested docs/output.md was dropped: {sections:?}"
    );
    assert!(content.contains("NESTED_DOC_MARKER"));
    assert!(sections.contains(&"### File: `a.txt`"));
    assert!(
        !sections.contains(&"### File: `output.md`"),
        "the output file itself should not be part of the report: {sections:?}"
    );

    // Second run: root output.md now exists and matches the report signature.
    run(root.to_string_lossy(), output.to_string_lossy()).unwrap();
    let again = fs::read_to_string(&output).unwrap();
    let again_sections = file_sections(&again);
    assert!(
        again_sections.contains(&"### File: `docs/output.md`"),
        "nested docs/output.md disappeared on rerun: {again_sections:?}"
    );
    assert!(!again_sections.contains(&"### File: `output.md`"));
}

/// Repro: `context-builder -d proj` from the parent, default `-o output.md`.
/// The output file is not inside `proj`, so nothing named `output.md` in the
/// project should be ignored just because of that default name.
#[test]
#[serial]
fn default_output_name_from_parent_keeps_nested_output_md() {
    let dir = tempdir().unwrap();
    let parent = dir.path();
    let proj = parent.join("proj");
    fs::create_dir_all(proj.join("docs")).unwrap();
    fs::write(
        proj.join("docs/output.md"),
        "real doc PARENT_NESTED_MARKER\n",
    )
    .unwrap();
    fs::write(proj.join("a.txt"), "x\n").unwrap();

    let _cwd = CwdGuard::enter(parent);
    run("proj", "output.md").unwrap();
    let content = fs::read_to_string(parent.join("output.md")).unwrap();
    let sections = file_sections(&content);

    assert!(
        sections.contains(&"### File: `docs/output.md`"),
        "default output name hid docs/output.md: {sections:?}\n{content}"
    );
    assert!(content.contains("PARENT_NESTED_MARKER"));
    assert!(sections.contains(&"### File: `a.txt`"));
}
