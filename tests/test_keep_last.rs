//! `keep_last` prunes old timestamped snapshots (#20 P16).

use std::fs;
use std::process::Command;

fn names(dir: &std::path::Path) -> Vec<String> {
    let mut v: Vec<String> = fs::read_dir(dir)
        .unwrap()
        .flatten()
        .map(|e| e.file_name().into_string().unwrap())
        .collect();
    v.sort();
    v
}

fn run(dir: &std::path::Path) {
    let out = Command::new(env!("CARGO_BIN_EXE_context-builder"))
        .arg("-y")
        .current_dir(dir)
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
}

#[test]
fn keep_last_prunes_only_matching_old_snapshots() {
    let dir = tempfile::tempdir().unwrap();
    let base = dir.path();
    fs::write(base.join("a.rs"), "fn a() {}\n").unwrap();
    fs::write(
        base.join("context-builder.toml"),
        "output_folder = \"out\"\ntimestamped_output = true\nkeep_last = 2\n",
    )
    .unwrap();
    fs::create_dir(base.join("out")).unwrap();
    for day in 1..=5 {
        fs::write(
            base.join(format!("out/output_2020010{day}000000.md")),
            "old",
        )
        .unwrap();
    }
    // Look-alikes that must survive.
    fs::write(base.join("out/output_notes.md"), "keep").unwrap();
    fs::write(base.join("out/other_20200101000000.md"), "keep").unwrap();
    fs::write(base.join("out/output_20200101000000.txt"), "keep").unwrap();

    run(base);

    let left = names(&base.join("out"));
    let snapshots: Vec<_> = left
        .iter()
        .filter(|n| n.starts_with("output_2") && n.ends_with(".md"))
        .collect();
    assert_eq!(snapshots.len(), 2, "{left:?}");
    assert!(
        snapshots
            .iter()
            .any(|n| n.as_str() == "output_20200105000000.md"),
        "{left:?}"
    );
    assert!(
        !snapshots
            .iter()
            .any(|n| n.as_str() == "output_20200101000000.md"),
        "{left:?}"
    );
    for keep in [
        "output_notes.md",
        "other_20200101000000.md",
        "output_20200101000000.txt",
    ] {
        assert!(
            left.iter().any(|n| n == keep),
            "{keep} was deleted: {left:?}"
        );
    }
}

#[test]
fn without_keep_last_everything_is_kept() {
    let dir = tempfile::tempdir().unwrap();
    let base = dir.path();
    fs::write(base.join("a.rs"), "fn a() {}\n").unwrap();
    fs::write(
        base.join("context-builder.toml"),
        "output_folder = \"out\"\ntimestamped_output = true\n",
    )
    .unwrap();
    fs::create_dir(base.join("out")).unwrap();
    for day in 1..=3 {
        fs::write(
            base.join(format!("out/output_2020010{day}000000.md")),
            "old",
        )
        .unwrap();
    }
    run(base);
    assert_eq!(names(&base.join("out")).len(), 4);
}
