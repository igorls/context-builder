//! The alphabetical tree and the relevance-ordered bodies differ; the output
//! must say so (#42).

use std::fs;
use std::process::Command;

fn fixture() -> tempfile::TempDir {
    let dir = tempfile::tempdir().unwrap();
    fs::create_dir(dir.path().join("src")).unwrap();
    fs::write(dir.path().join("Cargo.toml"), "[package]\nname = \"x\"\n").unwrap();
    fs::write(dir.path().join("README.md"), "# x\n").unwrap();
    fs::write(dir.path().join("src/lib.rs"), "pub fn f() {}\n").unwrap();
    dir
}

fn run(dir: &std::path::Path, args: &[&str]) -> String {
    let out = Command::new(env!("CARGO_BIN_EXE_context-builder"))
        .args(["-d", dir.to_str().unwrap()])
        .args(args)
        .current_dir(dir)
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8_lossy(&out.stdout).into_owned()
}

#[test]
fn document_says_tree_is_alphabetical_and_bodies_are_ranked() {
    let dir = fixture();
    let doc = run(dir.path(), &["-o", "-"]);

    let heading = doc.find("## File Tree Structure").expect("tree heading");
    let note = doc.find("The tree is alphabetical").expect("order note");
    let first_entry = doc.find("- 📄 Cargo.toml").expect("tree entry");
    assert!(
        heading < note && note < first_entry,
        "note misplaced:\n{doc}"
    );

    // The two orders really do differ, which is why the note is needed.
    let readme_body = doc.find("### File: `README.md`").unwrap();
    let cargo_body = doc.find("### File: `Cargo.toml`").unwrap();
    assert!(readme_body < cargo_body);
}

#[test]
fn preview_carries_the_same_note() {
    let dir = fixture();
    let out = run(dir.path(), &["--preview"]);
    assert!(out.contains("The tree is alphabetical"), "{out}");
}
