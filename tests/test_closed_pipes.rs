//! End-to-end checks of the built binary when the reader of stderr or stdout
//! goes away (`context-builder … 2>&1 | head -3`, `context-builder -o - | head`).
//! Previously a failed `eprintln!` panicked and the run ended with exit code 101
//! before the document was finished.

use std::fs;
use std::io::pipe;
use std::path::Path;
use std::process::{Command, Stdio};

fn fixture(dir: &Path) {
    fs::create_dir_all(dir.join("src")).unwrap();
    for i in 0..40 {
        fs::write(
            dir.join("src").join(format!("f{i}.rs")),
            format!("pub fn f{i}() {{}}\n"),
        )
        .unwrap();
    }
    // These make the run write notices to stderr (lockfile and asset skips).
    fs::write(dir.join("Cargo.lock"), "# lock\n").unwrap();
    fs::write(dir.join("logo.png"), b"\x89PNG\r\n\x1a\n\0\0").unwrap();
}

fn bin() -> Command {
    Command::new(env!("CARGO_BIN_EXE_context-builder"))
}

#[test]
fn closed_stderr_pipe_does_not_abort_the_run() {
    let dir = tempfile::tempdir().unwrap();
    fixture(dir.path());
    let out = dir.path().join("out.md");

    // A pipe whose read end is already gone: every stderr write fails with EPIPE.
    let (reader, writer) = pipe().unwrap();
    drop(reader);
    let status = bin()
        .args([
            "-d",
            dir.path().to_str().unwrap(),
            "-o",
            out.to_str().unwrap(),
        ])
        .current_dir(dir.path())
        .stderr(writer)
        .stdout(Stdio::null())
        .status()
        .unwrap();

    assert!(status.success(), "exit status was {status:?}");
    let doc = fs::read_to_string(&out).expect("output file must be written");
    assert!(doc.contains("### File: `src/f39.rs`"), "document truncated");
    assert!(doc.contains("## Skipped"), "skip section missing");
}

#[test]
fn closed_stdout_pipe_ends_quietly() {
    let dir = tempfile::tempdir().unwrap();
    fixture(dir.path());

    let (reader, writer) = pipe().unwrap();
    drop(reader);
    let output = bin()
        .args(["-d", dir.path().to_str().unwrap(), "-o", "-"])
        .current_dir(dir.path())
        .stdout(writer)
        .stderr(Stdio::piped())
        .output()
        .unwrap();

    assert!(
        output.status.success(),
        "exit status was {:?}",
        output.status
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(!stderr.contains("panicked"), "{stderr}");
    assert!(!stderr.contains("Failed to write output"), "{stderr}");
}

#[test]
fn error_exit_code_is_unchanged_for_real_failures() {
    let output = bin()
        .args(["-d", "/definitely/not/here/cb-test"])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(1));
}
