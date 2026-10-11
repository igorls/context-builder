//! Built-in directory ignores can be turned off per name (#16).

use std::fs;
use std::process::Command;

fn doc(dir: &std::path::Path, extra: &[&str]) -> String {
    let out = Command::new(env!("CARGO_BIN_EXE_context-builder"))
        .args(["-d", dir.to_str().unwrap(), "-o", "-"])
        .args(extra)
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
fn negated_ignore_keeps_a_default_ignored_directory() {
    let dir = tempfile::tempdir().unwrap();
    let base = dir.path();
    fs::create_dir_all(base.join("src/build")).unwrap();
    fs::create_dir_all(base.join("vendor/lib")).unwrap();
    fs::write(base.join("src/main.rs"), "fn main() {}\n").unwrap();
    fs::write(base.join("src/build/mod.rs"), "pub mod b;\n").unwrap();
    fs::write(base.join("vendor/lib/k.go"), "package k\n").unwrap();

    let default = doc(base, &[]);
    assert!(default.contains("src/main.rs"));
    assert!(
        !default.contains("src/build/mod.rs"),
        "build is ignored by default"
    );
    assert!(!default.contains("vendor/lib/k.go"));

    let kept = doc(base, &["-i", "!build"]);
    assert!(kept.contains("src/build/mod.rs"), "{kept}");
    assert!(!kept.contains("vendor/lib/k.go"), "only build was opted in");
    assert!(kept.contains("src/main.rs"), "other files must stay");

    let both = doc(base, &["-i", "!build,!vendor", "-i", "nothing"]);
    assert!(both.contains("vendor/lib/k.go") && both.contains("src/build/mod.rs"));
}
