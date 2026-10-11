//! Go files in a `doc/` package rank as source, not documentation (#19).

use std::fs;
use std::process::Command;

#[test]
fn doc_go_package_is_ranked_with_source_but_doc_markdown_stays_docs() {
    let dir = tempfile::tempdir().unwrap();
    let base = dir.path();
    fs::create_dir_all(base.join("doc")).unwrap();
    fs::create_dir_all(base.join("cmd")).unwrap();
    fs::write(base.join("go.mod"), "module x\n").unwrap();
    fs::write(base.join("doc/man_docs.go"), "package doc\n").unwrap();
    fs::write(base.join("doc/guide.md"), "# guide\n").unwrap();
    fs::write(base.join("cmd/main.go"), "package main\n").unwrap();

    let out = Command::new(env!("CARGO_BIN_EXE_context-builder"))
        .args(["-d", base.to_str().unwrap(), "-o", "-"])
        .current_dir(base)
        .output()
        .unwrap();
    assert!(out.status.success());
    let doc = String::from_utf8_lossy(&out.stdout);
    let pos = |needle: &str| {
        doc.find(needle)
            .unwrap_or_else(|| panic!("{needle} missing"))
    };
    let go = pos("### File: `doc/man_docs.go`");
    let md = pos("### File: `doc/guide.md`");
    assert!(go < md, "doc/*.go should come before doc markdown");
    assert!(pos("### File: `cmd/main.go`") < md, "source before docs");
}
