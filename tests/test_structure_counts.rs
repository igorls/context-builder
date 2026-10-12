//! `--structure` reports real code-line counts for every language (#22 B15).
#![cfg(all(
    feature = "tree-sitter-ts",
    feature = "tree-sitter-python",
    feature = "tree-sitter-go"
))]

use std::fs;
use std::process::Command;

#[test]
fn structure_code_counts_are_nonzero_for_typescript_python_go() {
    let dir = tempfile::tempdir().unwrap();
    let base = dir.path();
    fs::write(
        base.join("a.ts"),
        "// header\nexport function f(): number {\n  return 1;\n}\n",
    )
    .unwrap();
    fs::write(
        base.join("b.py"),
        "# header\n\ndef g():\n    return 2\n\nclass C:\n    pass\n",
    )
    .unwrap();
    fs::write(
        base.join("c.go"),
        "package c\n\n// note\nfunc H() int { return 3 }\nfunc J() int { return 4 }\n",
    )
    .unwrap();

    let out = Command::new(env!("CARGO_BIN_EXE_context-builder"))
        .args(["-d", base.to_str().unwrap(), "--structure", "-o", "-"])
        .current_dir(base)
        .output()
        .unwrap();
    assert!(out.status.success());
    let doc = String::from_utf8_lossy(&out.stdout);
    assert!(!doc.contains("(0 code)"), "{doc}");
    assert!(doc.contains("7 lines (4 code)"), "py: {doc}");
    // TS and Go have different totals, so neither can satisfy the other's assertion.
    assert_eq!(doc.matches("4 lines (3 code)").count(), 1, "ts: {doc}");
    assert!(doc.contains("5 lines (3 code)"), "go: {doc}");
}

#[test]
fn pyw_hash_comments_are_not_counted_as_code() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(
        dir.path().join("a.pyw"),
        "# c\n# d\ndef f():\n    return 1\n",
    )
    .unwrap();
    let out = Command::new(env!("CARGO_BIN_EXE_context-builder"))
        .args(["-d", dir.path().to_str().unwrap(), "--structure", "-o", "-"])
        .current_dir(dir.path())
        .output()
        .unwrap();
    let doc = String::from_utf8_lossy(&out.stdout);
    assert!(doc.contains("4 lines (2 code)"), "pyw: {doc}");
}
