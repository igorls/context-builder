//! Python `--signatures` blocks must not contain `//` comments (#22 B16),
//! and `--help` must not promise imports/exports that `--structure` lacks.
#![cfg(feature = "tree-sitter-base")]

use std::fs;
use std::process::Command;

#[test]
fn python_signature_block_uses_hash_comment_headings() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(
        dir.path().join("m.py"),
        "def greet(name):\n    return name\n",
    )
    .unwrap();
    let out = Command::new(env!("CARGO_BIN_EXE_context-builder"))
        .args([
            "-d",
            dir.path().to_str().unwrap(),
            "--signatures",
            "-o",
            "-",
        ])
        .current_dir(dir.path())
        .output()
        .unwrap();
    assert!(out.status.success());
    let doc = String::from_utf8_lossy(&out.stdout);
    assert!(doc.contains("# Functions"), "{doc}");
    assert!(!doc.contains("// Functions"), "{doc}");
}

#[test]
fn help_does_not_promise_imports_or_exports() {
    let out = Command::new(env!("CARGO_BIN_EXE_context-builder"))
        .arg("--help")
        .output()
        .unwrap();
    let help = String::from_utf8_lossy(&out.stdout).to_lowercase();
    assert!(!help.contains("imports, exports"), "{help}");
}
