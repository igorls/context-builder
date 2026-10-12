//! In-process runs through the size-warning code paths (#41). The
//! binary-spawning tests in `test_size_warnings.rs` assert the stderr text;
//! these run the library directly (warnings enabled: `CB_SILENT` unset) so the
//! paths are also visible to coverage tools that do not follow child
//! processes, and check the documents the runs produce.

use context_builder::cli::Args;
use context_builder::config::Config;
use context_builder::{Prompter, run_with_args};
use serial_test::serial;
use std::fs;
use std::path::Path;
use tempfile::tempdir;

struct TestPrompter;

impl Prompter for TestPrompter {
    fn confirm_overwrite(&self, _file_path: &str) -> std::io::Result<bool> {
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
        // Large enough that the big fixtures are read, not skipped.
        max_file_size: "10M".to_string(),
        hidden: false,
        include_secrets: false,
        include_lockfiles: false,
    }
}

/// A Rust file of roughly `kb` kilobytes made of many small functions.
fn big_rust(kb: usize) -> String {
    let mut s = String::new();
    let mut i = 0;
    while s.len() < kb * 1024 {
        s.push_str(&format!(
            "pub fn function_number_{i}(alpha: u32, beta: u32) -> u32 {{ alpha + beta + {i} }}\n"
        ));
        i += 1;
    }
    s
}

fn project() -> tempfile::TempDir {
    let dir = tempdir().unwrap();
    fs::write(dir.path().join("big.rs"), big_rust(600)).unwrap();
    fs::write(dir.path().join("notes.txt"), "plain text\n").unwrap();
    dir
}

fn run(a: Args) -> String {
    let out = a.output.clone();
    run_with_args(a, Config::default(), &TestPrompter).unwrap();
    fs::read_to_string(out).unwrap()
}

#[test]
#[serial]
fn full_run_over_budget_still_writes_every_body() {
    let dir = project();
    let out = dir.path().join("out.md");
    let doc = run(args(dir.path(), &out));
    assert!(doc.contains("function_number_0("), "full body expected");
    assert!(doc.contains("plain text"));
}

#[test]
#[serial]
fn preview_reports_input_without_writing_a_document() {
    let dir = project();
    let out = dir.path().join("out.md");
    let mut a = args(dir.path(), &out);
    a.preview = true;
    run_with_args(a, Config::default(), &TestPrompter).unwrap();
    assert!(!out.exists(), "--preview writes nothing");
}

#[cfg(feature = "tree-sitter-base")]
#[test]
#[serial]
fn signatures_outline_supported_files_structure_does_not() {
    let dir = project();
    let out = dir.path().join("sig.md");
    let mut a = args(dir.path(), &out);
    a.signatures = true;
    let doc = run(a);
    // The Rust body is replaced by an outline, the text file is not.
    assert!(
        !doc.contains("alpha + beta + 7 }"),
        "body should be outlined"
    );
    assert!(doc.contains("plain text"), "unsupported file stays whole");

    // `--structure` appends a summary but keeps the full body, so it is not
    // an outline and the input-size report must still count the file.
    let out = dir.path().join("structure.md");
    let mut a = args(dir.path(), &out);
    a.structure = true;
    let doc = run(a);
    assert!(doc.contains("alpha + beta + 7 }"), "structure keeps bodies");
}

#[test]
#[serial]
fn token_budget_and_filter_runs_complete() {
    let dir = project();
    let out = dir.path().join("budget.md");
    let mut a = args(dir.path(), &out);
    a.max_tokens = Some(2_000);
    let doc = run(a);
    assert!(doc.len() < 600 * 1024, "budget caps the document");

    let out = dir.path().join("filtered.md");
    let mut a = args(dir.path(), &out);
    a.filter = vec!["rs".to_string()];
    let doc = run(a);
    assert!(!doc.contains("plain text"));
}
