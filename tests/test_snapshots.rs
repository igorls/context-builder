//! Timestamped snapshots never collide, and the auto-diff renderer prints
//! readable times (#20: B10, B9).

use std::fs;
use std::process::Command;

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
fn rapid_timestamped_runs_keep_every_snapshot_and_print_utc_times() {
    let dir = tempfile::tempdir().unwrap();
    let base = dir.path();
    fs::write(
        base.join("context-builder.toml"),
        "output_folder = \"out\"\ntimestamped_output = true\nauto_diff = true\nfile_metadata = true\n",
    )
    .unwrap();
    fs::write(base.join("a.txt"), "one\n").unwrap();

    for i in 0..4 {
        fs::write(base.join("a.txt"), format!("version {i}\n")).unwrap();
        run(base);
    }

    let reports: Vec<_> = fs::read_dir(base.join("out"))
        .unwrap()
        .flatten()
        .map(|e| e.path())
        .collect();
    assert_eq!(reports.len(), 4, "a snapshot was overwritten: {reports:?}");

    let mut saw_modified = false;
    for report in reports {
        let doc = fs::read_to_string(&report).unwrap();
        assert!(!doc.contains("SystemTime"), "{}", report.display());
        for line in doc.lines().filter(|l| l.starts_with("- Modified:") && !l.contains('`')) {
            saw_modified = true;
            assert!(line.ends_with(" UTC"), "{line}");
        }
    }
    assert!(saw_modified, "no Modified line was rendered");
}
