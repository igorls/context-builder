//! `--init` must not suggest filters that undo the default skips (#39).

use std::fs;
use std::process::Command;

fn bin() -> Command {
    Command::new(env!("CARGO_BIN_EXE_context-builder"))
}

#[test]
fn init_does_not_suggest_asset_secret_or_oversized_types() {
    let dir = tempfile::tempdir().unwrap();
    let base = dir.path();
    fs::write(base.join("a.rs"), "fn a() {}\n").unwrap();
    fs::write(base.join("Cargo.toml"), "[package]\nname = \"x\"\n").unwrap();
    // More of these than of any real type, so they would rank first.
    for i in 0..6 {
        fs::write(base.join(format!("img{i}.png")), b"\x89PNG\r\n\x1a\n\0\0").unwrap();
        fs::write(base.join(format!("bundle{i}.min.js")), "var a=1;\n").unwrap();
        fs::write(
            base.join(format!("key{i}.pem")),
            "-----BEGIN CERTIFICATE-----\n",
        )
        .unwrap();
    }
    fs::write(base.join("dump.json"), "x".repeat(300 * 1024)).unwrap();

    let out = bin().arg("--init").current_dir(base).output().unwrap();
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );

    let toml = fs::read_to_string(base.join("context-builder.toml")).unwrap();
    let filter_line = toml
        .lines()
        .find(|l| l.starts_with("filter ="))
        .expect("filter line");
    for bad in ["png", "js", "pem", "json"] {
        assert!(
            !filter_line.contains(&format!("\"{bad}\"")),
            "filter must not contain {bad}: {filter_line}"
        );
    }
    assert!(filter_line.contains("\"rs\""), "{filter_line}");
    assert!(filter_line.contains("\"toml\""), "{filter_line}");

    // And the generated config really keeps the assets out of a default run.
    let preview = bin().arg("--preview").current_dir(base).output().unwrap();
    let stdout = String::from_utf8_lossy(&preview.stdout);
    assert!(stdout.contains("a.rs"), "{stdout}");
    assert!(!stdout.contains("img0.png"), "{stdout}");
}
