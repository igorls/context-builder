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
        file_metadata: false,
    }
}

fn run(input: impl Into<String>, output: impl Into<String>) -> std::io::Result<()> {
    run_with_args(args(input, output), Config::default(), &TestPrompter)
}

/// `### File: ` headers, with Windows `\\` separators shown as `/` so the
/// assertions are platform-independent.
fn file_sections(content: &str) -> Vec<String> {
    content
        .lines()
        .filter(|line| line.starts_with("### File: "))
        .map(|line| line.replace('\\', "/"))
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
        sections.iter().any(|s| s == "### File: `a.txt`"),
        "expected a.txt, got {sections:?}"
    );
    assert!(
        !sections.iter().any(|s| s == "### File: `output.md`"),
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
        sections.iter().any(|s| s == "### File: `docs/output.md`"),
        "nested docs/output.md was dropped: {sections:?}"
    );
    assert!(content.contains("NESTED_DOC_MARKER"));
    assert!(sections.iter().any(|s| s == "### File: `a.txt`"));
    assert!(
        !sections.iter().any(|s| s == "### File: `output.md`"),
        "the output file itself should not be part of the report: {sections:?}"
    );

    // Second run: root output.md now exists and matches the report signature.
    run(root.to_string_lossy(), output.to_string_lossy()).unwrap();
    let again = fs::read_to_string(&output).unwrap();
    let again_sections = file_sections(&again);
    assert!(
        again_sections
            .iter()
            .any(|s| s == "### File: `docs/output.md`"),
        "nested docs/output.md disappeared on rerun: {again_sections:?}"
    );
    assert!(!again_sections.iter().any(|s| s == "### File: `output.md`"));
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
        sections.iter().any(|s| s == "### File: `docs/output.md`"),
        "default output name hid docs/output.md: {sections:?}\n{content}"
    );
    assert!(content.contains("PARENT_NESTED_MARKER"));
    assert!(sections.iter().any(|s| s == "### File: `a.txt`"));
}

fn run_cli(current_dir: &Path, args: &[&str]) -> std::process::Output {
    std::process::Command::new(env!("CARGO_BIN_EXE_context-builder"))
        .current_dir(current_dir)
        .args(args)
        .env_remove("CB_SILENT")
        .output()
        .expect("failed to spawn context-builder")
}

/// Repro: `$HOME`-style `.gitignore` (`*` and `!*/`) above a project that has
/// no `.git`. Those parent rules must not apply. `-d .` is relative, so the
/// ancestor search has to resolve the working directory.
#[test]
fn parent_gitignore_without_repo_keeps_project_files() {
    let dir = tempdir().unwrap();
    let parent = dir.path();
    fs::write(parent.join(".gitignore"), "*\n!*/\n").unwrap();
    let proj = parent.join("proj");
    fs::create_dir_all(&proj).unwrap();
    fs::write(proj.join("keep.txt"), "PARENT_IGNORE_KEEP\n").unwrap();
    fs::write(proj.join(".gitignore"), "secret.txt\n").unwrap();
    fs::write(proj.join("secret.txt"), "PARENT_SECRET\n").unwrap();

    let result = run_cli(&proj, &["-d", ".", "-o", "out.md", "-y"]);
    let stderr = String::from_utf8_lossy(&result.stderr);
    let stdout = String::from_utf8_lossy(&result.stdout);
    assert!(
        result.status.success(),
        "status: {:?}\nstdout: {stdout}\nstderr: {stderr}",
        result.status
    );
    assert!(
        !stderr.contains("No files matched"),
        "non-empty walk warned: {stderr}"
    );
    assert!(stdout.contains("Documentation created successfully"));

    let content = fs::read_to_string(proj.join("out.md")).unwrap();
    let sections = file_sections(&content);
    assert!(
        sections.iter().any(|s| s == "### File: `keep.txt`"),
        "parent .gitignore hid the tree: {sections:?}\n{content}"
    );
    assert!(content.contains("PARENT_IGNORE_KEEP"));
    assert!(
        !content.contains("PARENT_SECRET"),
        "in-tree .gitignore was ignored: {content}"
    );
}

/// A project nested inside a repository still honors the repo-root
/// `.gitignore`, including when `-d` is relative. A `*` pattern above the
/// repository must not apply.
#[test]
fn nested_project_inside_git_repo_honors_repo_gitignore() {
    let dir = tempdir().unwrap();
    let home = dir.path();
    fs::write(home.join(".gitignore"), "*\n!*/\n").unwrap();
    let repo = home.join("repo");
    fs::create_dir_all(repo.join(".git")).unwrap();
    fs::write(repo.join(".gitignore"), "secret.txt\n").unwrap();
    let proj = repo.join("proj");
    fs::create_dir_all(&proj).unwrap();
    fs::write(proj.join("keep.txt"), "REPO_KEEP\n").unwrap();
    fs::write(proj.join("secret.txt"), "REPO_SECRET\n").unwrap();

    let result = run_cli(&proj, &["-d", ".", "-o", "out.md", "-y"]);
    let stderr = String::from_utf8_lossy(&result.stderr);
    let stdout = String::from_utf8_lossy(&result.stdout);
    assert!(
        result.status.success(),
        "status: {:?}\nstdout: {stdout}\nstderr: {stderr}",
        result.status
    );

    let content = fs::read_to_string(proj.join("out.md")).unwrap();
    assert!(
        content.contains("REPO_KEEP"),
        "repo parent rules or the dotfiles pattern hid keep.txt:\n{content}"
    );
    assert!(
        !content.contains("REPO_SECRET"),
        "repo-root .gitignore was not applied:\n{content}"
    );
}

/// An in-tree `.gitignore` that matches every file still exits 0 and writes
/// the document, and prints one stderr warning.
#[test]
fn empty_walk_warns_once_and_exits_zero() {
    let dir = tempdir().unwrap();
    let root = dir.path();
    fs::write(root.join(".gitignore"), "*\n!*/\n").unwrap();
    fs::write(root.join("a.txt"), "hidden-by-gitignore\n").unwrap();

    let result = run_cli(root, &["-d", ".", "-o", "out.md", "-y"]);
    let stderr = String::from_utf8_lossy(&result.stderr);
    let stdout = String::from_utf8_lossy(&result.stdout);
    assert!(
        result.status.success(),
        "status: {:?}\nstdout: {stdout}\nstderr: {stderr}",
        result.status
    );
    assert_eq!(
        stderr.matches("No files matched").count(),
        1,
        "expected one warning, stderr was: {stderr}"
    );
    assert!(stderr.contains("check .gitignore, --ignore, and --filter"));
    assert!(stdout.contains("Documentation created successfully"));

    let content = fs::read_to_string(root.join("out.md")).unwrap();
    assert!(content.contains("# Directory Structure Report"));
    assert!(
        file_sections(&content).is_empty(),
        "expected no file sections: {content}"
    );
    assert!(!content.contains("hidden-by-gitignore"));
}
