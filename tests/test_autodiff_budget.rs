//! `--max-tokens` with auto-diff budgets file by file with the real tokenizer,
//! like the standard renderer (#20 B14): no cut mid-fence, an overrunning file
//! is clamped and says so, omitted files are counted.

use context_builder::cli::Args;
use context_builder::config::Config;
use context_builder::token_count::{Encoding, estimate_tokens};
use context_builder::{Prompter, run_with_args};
use serial_test::serial;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use tempfile::tempdir;

/// The auto-diff cache lives under the current directory, so each test runs
/// inside its own project (and serially).
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

struct Yes;

impl Prompter for Yes {
    fn confirm_overwrite(&self, _file_path: &str) -> std::io::Result<bool> {
        Ok(true)
    }
}

fn args(input: &Path, output: &Path, max_tokens: Option<usize>, encoding: &str) -> Args {
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
        encoding: encoding.to_string(),
        init: false,
        max_tokens,
        signatures: false,
        structure: false,
        truncate: "smart".to_string(),
        visibility: "all".to_string(),
        file_metadata: false,
        max_file_size: "256K".to_string(),
        hidden: false,
        include_secrets: false,
        include_lockfiles: false,
    }
}

fn project() -> tempfile::TempDir {
    let dir = tempdir().unwrap();
    for i in 1..=4 {
        let mut body = format!("fn f{i}() {{\n");
        for j in 0..300 {
            body.push_str(&format!("    let v{i}_{j} = {j};\n"));
        }
        body.push_str("}\n");
        fs::write(dir.path().join(format!("f{i}.rs")), body).unwrap();
    }
    dir
}

fn auto_diff() -> Config {
    Config {
        auto_diff: Some(true),
        ..Default::default()
    }
}

fn fence_lines(doc: &str) -> usize {
    doc.lines().filter(|l| l.starts_with("```")).count()
}

#[test]
#[serial]
fn budget_is_met_by_the_real_tokenizer_and_fences_stay_closed() {
    let dir = project();
    let _cwd = CwdGuard::enter(dir.path());
    let out = dir.path().join("out.md");
    run_with_args(
        args(dir.path(), &out, Some(3000), "o200k_base"),
        auto_diff(),
        &Yes,
    )
    .unwrap();
    let doc = fs::read_to_string(&out).unwrap();

    let tokens = estimate_tokens(Encoding::O200kBase, &doc);
    assert!(tokens <= 3000, "{tokens} tokens exceed the 3000 budget");
    assert!(tokens > 1500, "budget should be used, got {tokens}");
    assert_eq!(fence_lines(&doc) % 2, 0, "unbalanced fences:\n{doc}");
    assert!(doc.contains("Token budget (3000) reached"), "{doc}");
}

#[test]
#[serial]
fn crossing_file_is_clamped_and_the_rest_counted() {
    let dir = project();
    let _cwd = CwdGuard::enter(dir.path());
    let out = dir.path().join("out.md");
    run_with_args(
        args(dir.path(), &out, Some(5000), "o200k_base"),
        auto_diff(),
        &Yes,
    )
    .unwrap();
    let doc = fs::read_to_string(&out).unwrap();

    assert!(doc.contains("### File: `f1.rs`"), "first file kept");
    assert!(
        doc.contains("File content truncated"),
        "clamped file says so:\n{doc}"
    );
    // f1 fits, f2 is clamped; f3 and f4 are omitted and reported as such.
    assert!(doc.contains("2 remaining files omitted"), "{doc}");
    assert!(!doc.contains("### File: `f3.rs`") && !doc.contains("### File: `f4.rs`"));
}

#[test]
#[serial]
fn a_large_budget_changes_nothing() {
    // Separate projects: a second run in one would diff against the first.
    let (a, b) = (project(), project());
    let full = a.path().join("full.md");
    let capped = b.path().join("capped.md");
    run_with_args(args(a.path(), &full, None, "o200k_base"), auto_diff(), &Yes).unwrap();
    run_with_args(
        args(b.path(), &capped, Some(1_000_000), "o200k_base"),
        auto_diff(),
        &Yes,
    )
    .unwrap();
    let strip = |p: &Path| {
        fs::read_to_string(p)
            .unwrap()
            .lines()
            .filter(|l| {
                !l.starts_with("**Generated:**")
                    && !l.starts_with("**Project:**")
                    && !l.starts_with("Content hash")
            })
            .collect::<Vec<_>>()
            .join("\n")
    };
    assert_eq!(strip(&full), strip(&capped));
}

#[test]
#[serial]
fn encoding_is_honored_for_the_budget() {
    let dir = project();
    let _cwd = CwdGuard::enter(dir.path());
    let out = dir.path().join("out.md");
    run_with_args(
        args(dir.path(), &out, Some(3000), "cl100k_base"),
        auto_diff(),
        &Yes,
    )
    .unwrap();
    let doc = fs::read_to_string(&out).unwrap();
    let tokens = estimate_tokens(Encoding::Cl100kBase, &doc);
    assert!(tokens <= 3000, "{tokens} cl100k tokens exceed the budget");
}

#[test]
#[serial]
fn header_larger_than_the_budget_still_ends_cleanly() {
    let dir = project();
    let _cwd = CwdGuard::enter(dir.path());
    let out = dir.path().join("out.md");
    run_with_args(
        args(dir.path(), &out, Some(40), "o200k_base"),
        auto_diff(),
        &Yes,
    )
    .unwrap();
    let doc = fs::read_to_string(&out).unwrap();
    assert!(
        doc.contains("Output truncated: exceeded 40 token budget"),
        "{doc}"
    );
    assert_eq!(fence_lines(&doc) % 2, 0);
}

#[test]
fn binary_with_auto_diff_config_respects_the_budget() {
    let dir = project();
    fs::write(
        dir.path().join("context-builder.toml"),
        "auto_diff = true\n",
    )
    .unwrap();
    let out = Command::new(env!("CARGO_BIN_EXE_context-builder"))
        .args(["-d", ".", "-y", "-o", "-", "--max-tokens", "3000"])
        .current_dir(dir.path())
        .output()
        .unwrap();
    assert!(out.status.success());
    let doc = String::from_utf8_lossy(&out.stdout);
    let tokens = estimate_tokens(Encoding::O200kBase, &doc);
    assert!(tokens <= 3000, "{tokens}");
    assert!(doc.contains("remaining files omitted"), "{doc}");
}

#[test]
#[serial]
fn file_metadata_is_rendered_in_budgeted_sections() {
    let dir = project();
    let _cwd = CwdGuard::enter(dir.path());
    let out = dir.path().join("out.md");
    let mut a = args(dir.path(), &out, Some(100_000), "o200k_base");
    a.file_metadata = true;
    run_with_args(a, auto_diff(), &Yes).unwrap();
    let doc = fs::read_to_string(&out).unwrap();
    assert!(doc.contains("- Size: "), "{doc}");
    assert!(doc.contains("- Modified: "), "{doc}");
    assert!(!doc.contains("Token budget"), "plenty of room, nothing cut");
}
