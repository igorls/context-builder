//! `--stats`: per-file token breakdown without writing a document (#21 P7), and
//! the post-run token line using the real tokenizer instead of bytes/4.

use std::fs;
use std::path::Path;
use std::process::{Command, Output};
use tempfile::tempdir;

fn project() -> tempfile::TempDir {
    let dir = tempdir().unwrap();
    fs::write(dir.path().join("small.txt"), "tiny\n").unwrap();
    fs::write(dir.path().join("big.rs"), "let x = 1;\n".repeat(400)).unwrap();
    fs::write(dir.path().join("mid.md"), "word ".repeat(120)).unwrap();
    dir
}

fn run(dir: &Path, extra: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_context-builder"))
        .args(["-d", "."])
        .args(extra)
        .current_dir(dir)
        .output()
        .unwrap()
}

#[test]
fn stats_lists_files_largest_first_and_writes_no_document() {
    let dir = project();
    let out = run(dir.path(), &["--stats"]);
    assert!(out.status.success());
    let text = String::from_utf8_lossy(&out.stdout);
    assert!(text.contains("# Token Stats"), "{text}");
    let big = text.find("big.rs").unwrap();
    let mid = text.find("mid.md").unwrap();
    let small = text.find("small.txt").unwrap();
    assert!(big < mid && mid < small, "{text}");
    assert!(!dir.path().join("output.md").exists());
}

#[test]
fn stats_n_limits_the_list() {
    let dir = project();
    let text = String::from_utf8_lossy(&run(dir.path(), &["--stats=1"]).stdout).to_string();
    assert!(text.contains("Top 1 of 3 files"), "{text}");
    assert!(
        text.contains("big.rs") && !text.contains("small.txt"),
        "{text}"
    );
    assert!(text.contains("… 2 more files"), "{text}");
}

#[test]
fn stats_with_a_budget_shows_what_would_be_cut() {
    let dir = project();
    let text =
        String::from_utf8_lossy(&run(dir.path(), &["--stats=0", "--max-tokens", "600"]).stdout)
            .to_string();
    assert!(text.contains("Budget: 600 tokens"), "{text}");
    assert!(text.contains("CUT") || text.contains("omitted"), "{text}");
}

#[test]
fn post_run_token_line_is_the_real_count_not_bytes_over_four() {
    // One token per byte-ish: bytes/4 would report a quarter of the truth.
    let dir = tempdir().unwrap();
    fs::write(dir.path().join("rep.txt"), "a\n".repeat(20_000)).unwrap();
    let out = run(dir.path(), &["-y", "-o", "doc.md"]);
    let err = String::from_utf8_lossy(&out.stderr);
    let line = err
        .lines()
        .find(|l| l.starts_with("Estimated tokens:"))
        .unwrap();
    assert!(!line.contains('~'), "exact count expected, got {line}");
    let n: usize = line
        .trim_start_matches("Estimated tokens:")
        .trim()
        .replace(',', "")
        .parse()
        .unwrap();
    let doc = fs::read_to_string(dir.path().join("doc.md")).unwrap();
    let real = context_builder::token_count::estimate_tokens(
        context_builder::token_count::Encoding::O200kBase,
        &doc,
    );
    assert_eq!(n, real);
    assert!(
        n > doc.len() / 4 * 3 / 2,
        "{n} should clearly exceed bytes/4"
    );
}

#[test]
fn piped_output_keeps_the_estimate() {
    let dir = project();
    let out = run(dir.path(), &["-y", "-o", "-"]);
    let err = String::from_utf8_lossy(&out.stderr);
    assert!(err.contains("Estimated tokens: ~"), "{err}");
}
