//! `--init` honors `-d` and does not generate a config that drops docs (#17 B11).

use std::fs;
use std::process::Command;

#[test]
fn init_targets_the_directory_given_with_d() {
    let proj = tempfile::tempdir().unwrap();
    let elsewhere = tempfile::tempdir().unwrap();
    fs::create_dir_all(proj.path().join("src")).unwrap();
    fs::create_dir_all(proj.path().join("docs")).unwrap();
    fs::write(proj.path().join("src/a.py"), "x = 1\n").unwrap();
    fs::write(proj.path().join("src/b.py"), "y = 1\n").unwrap();
    // Markdown in the filter makes a re-ingested `.md` report detectable below.
    fs::write(proj.path().join("README.md"), "# proj\n").unwrap();
    fs::write(proj.path().join("docs/index.rst"), "Title\n=====\n").unwrap();

    let out = Command::new(env!("CARGO_BIN_EXE_context-builder"))
        .args(["-d", proj.path().to_str().unwrap(), "--init"])
        .current_dir(elsewhere.path())
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );

    assert!(
        !elsewhere.path().join("context-builder.toml").exists(),
        "config must not be written to the cwd"
    );
    let toml = fs::read_to_string(proj.path().join("context-builder.toml")).unwrap();
    let line = |key: &str| {
        toml.lines()
            .find(|l| l.starts_with(key))
            .unwrap()
            .to_string()
    };
    assert!(line("filter =").contains("\"py\""), "{toml}");
    assert!(line("filter =").contains("\"rst\""), "{toml}");
    assert!(line("filter =").contains("\"md\""), "{toml}");
    assert!(!line("ignore =").contains("docs"), "{toml}");
    assert!(!line("output_folder =").contains("\"docs\""), "{toml}");

    // The generated config keeps the docs in a real run and its output out of the next one.
    let run = || {
        Command::new(env!("CARGO_BIN_EXE_context-builder"))
            .args(["-d", proj.path().to_str().unwrap(), "-y"])
            .current_dir(elsewhere.path())
            .output()
            .unwrap()
    };
    assert!(run().status.success());
    let second = run();
    assert!(second.status.success());
    let out_dir = proj.path().join(".context-builder/out");
    let mut reports: Vec<_> = fs::read_dir(&out_dir).unwrap().flatten().collect();
    assert!(!reports.is_empty(), "no report in {out_dir:?}");
    reports.sort_by_key(|e| e.file_name());
    let doc = fs::read_to_string(reports.last().unwrap().path())
        .unwrap()
        .replace('\\', "/");
    assert!(doc.contains("docs/index.rst"), "docs dropped");
    assert_eq!(
        doc.matches("# Directory Structure Report").count(),
        1,
        "output re-ingested"
    );
}

/// Same as the binary test above, in-process so coverage tools see it.
#[test]
fn init_with_d_writes_into_that_directory_in_process() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("a.rs"), "fn a() {}\n").unwrap();
    std::fs::write(dir.path().join("logo.png"), [0u8, 1, 2]).unwrap();
    context_builder::run_from([
        "context-builder",
        "-d",
        dir.path().to_str().unwrap(),
        "--init",
    ])
    .unwrap();
    let cfg = std::fs::read_to_string(dir.path().join("context-builder.toml")).unwrap();
    assert!(cfg.contains("\"rs\""), "{cfg}");
    assert!(!cfg.contains("png"), "{cfg}");
    assert!(
        cfg.contains("output_folder = \".context-builder/out\""),
        "{cfg}"
    );
}
