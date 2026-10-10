//! Regression tests for dogfood bug B4.
//!
//! Non-interactive runs used to break once more than 100 files were found:
//! `context-builder -d <repo> </dev/null` printed "Operation cancelled" and
//! exited 1, and `sleep 10 | context-builder -d <repo>` hung, because the
//! over-100-file prompt (and the overwrite prompt) were written to stdout and
//! blocked on stdin. Both prompts are skipped when stdin is not a terminal.
//!
//! These tests spawn the real binary so they exercise `DefaultPrompter`, not
//! an injected test double. A timeout fails the test if the process blocks.

use std::fs;
use std::io::Read;
use std::path::PathBuf;
use std::process::{Child, Command, ExitStatus, Stdio};
use std::thread;
use std::time::{Duration, Instant};

use tempfile::tempdir;

/// Long enough for a tiny fixture to finish on a slow CI host, short enough
/// that a stdin hang fails the test.
const TIMEOUT: Duration = Duration::from_secs(15);

const MARKER: &str = "MARKER_B4_PAYLOAD";
const SENTINEL: &str = "SENTINEL_B4_OLD_OUTPUT";

enum StdinMode {
    /// Immediate EOF, as in `context-builder ... </dev/null`.
    Null,
    /// A pipe that is closed before the child reads, as in `printf '' | context-builder`.
    Empty,
    /// A pipe left open with no data, as in `sleep 10 | context-builder`.
    OpenPipe,
}

struct RunOutput {
    status: ExitStatus,
    stdout: String,
    stderr: String,
}

fn bin_path() -> &'static str {
    // Binary target name is `context-builder` (hyphen), used as-is.
    env!("CARGO_BIN_EXE_context-builder")
}

fn read_pipe(mut pipe: impl Read) -> String {
    let mut buf = Vec::new();
    let _ = pipe.read_to_end(&mut buf);
    String::from_utf8_lossy(&buf).into_owned()
}

fn wait_timeout(child: &mut Child, timeout: Duration) -> Result<ExitStatus, String> {
    let start = Instant::now();
    loop {
        match child.try_wait() {
            Ok(Some(status)) => return Ok(status),
            Ok(None) if start.elapsed() >= timeout => {
                let _ = child.kill();
                let _ = child.wait();
                return Err(format!(
                    "context-builder still running after {timeout:?} (blocked on a prompt?)"
                ));
            }
            Ok(None) => thread::sleep(Duration::from_millis(20)),
            Err(err) => return Err(format!("failed to wait on context-builder: {err}")),
        }
    }
}

fn run_binary(input: &str, output: &str, mode: StdinMode) -> RunOutput {
    let mut cmd = Command::new(bin_path());
    cmd.args(["-d", input, "-o", output])
        .env_remove("CB_SILENT")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .stdin(match mode {
            StdinMode::Null => Stdio::null(),
            StdinMode::Empty | StdinMode::OpenPipe => Stdio::piped(),
        });

    let mut child = cmd.spawn().expect("failed to spawn context-builder");

    // Empty: close the pipe now so the child sees EOF.
    // OpenPipe: hold the handle so no data and no EOF arrive while it runs.
    let held_stdin = match mode {
        StdinMode::OpenPipe => Some(child.stdin.take().expect("piped stdin")),
        StdinMode::Empty => {
            drop(child.stdin.take());
            None
        }
        StdinMode::Null => None,
    };

    let stdout_pipe = child.stdout.take().expect("piped stdout");
    let stderr_pipe = child.stderr.take().expect("piped stderr");
    let stdout_thread = thread::spawn(move || read_pipe(stdout_pipe));
    let stderr_thread = thread::spawn(move || read_pipe(stderr_pipe));

    let waited = wait_timeout(&mut child, TIMEOUT);
    drop(held_stdin);

    let stdout = stdout_thread.join().expect("stdout reader panicked");
    let stderr = stderr_thread.join().expect("stderr reader panicked");

    match waited {
        Ok(status) => RunOutput {
            status,
            stdout,
            stderr,
        },
        Err(reason) => panic!("{reason}\n--- stdout ---\n{stdout}\n--- stderr ---\n{stderr}"),
    }
}

fn write_fixture(dir: &std::path::Path, count: usize) {
    fs::create_dir_all(dir).unwrap();
    for i in 0..count {
        let body = if i == 0 {
            format!("{MARKER}\n")
        } else {
            format!("file-{i}\n")
        };
        fs::write(dir.join(format!("f{i}.txt")), body).unwrap();
    }
}

fn assert_completed_without_prompt(run: &RunOutput, output: &PathBuf, file_count: usize) {
    assert!(
        run.status.success(),
        "expected exit 0, got {:?}\n--- stdout ---\n{}\n--- stderr ---\n{}",
        run.status.code(),
        run.stdout,
        run.stderr
    );
    for (name, stream) in [("stdout", &run.stdout), ("stderr", &run.stderr)] {
        assert!(
            !stream.contains("Continue?"),
            "the >100-file prompt was written to {name}:\n{stream}"
        );
        assert!(
            !stream.contains("Overwrite?"),
            "the overwrite prompt was written to {name}:\n{stream}"
        );
        assert!(
            !stream.contains("Operation cancelled"),
            "run was cancelled; message on {name}:\n{stream}"
        );
    }
    let body = fs::read_to_string(output)
        .unwrap_or_else(|err| panic!("output {} was not written: {err}", output.display()));
    assert!(
        body.contains("# Directory Structure Report"),
        "output is not a context-builder document:\n{body}"
    );
    assert!(
        body.contains(MARKER),
        "output does not include fixture file contents:\n{body}"
    );
    let sections = body.matches("### File:").count();
    assert!(
        sections >= file_count,
        "expected at least {file_count} file sections, found {sections}"
    );
    assert!(
        !body.contains(SENTINEL),
        "pre-existing output was not replaced:\n{body}"
    );
}

fn assert_noninteractive(mode: StdinMode, file_count: usize, preexisting_output: bool) {
    let dir = tempdir().unwrap();
    let fixture = dir.path().join("repo");
    let output = dir.path().join("out.md");
    write_fixture(&fixture, file_count);
    if preexisting_output {
        fs::write(&output, SENTINEL).unwrap();
    }

    let run = run_binary(&fixture.to_string_lossy(), &output.to_string_lossy(), mode);
    assert_completed_without_prompt(&run, &output, file_count);
}

#[test]
fn null_stdin_over_100_files_writes_output() {
    assert_noninteractive(StdinMode::Null, 101, false);
}

#[test]
fn empty_stdin_over_100_files_writes_output() {
    assert_noninteractive(StdinMode::Empty, 101, false);
}

#[test]
fn open_pipe_stdin_over_100_files_writes_output() {
    assert_noninteractive(StdinMode::OpenPipe, 101, false);
}

#[test]
fn null_stdin_overwrites_existing_output() {
    assert_noninteractive(StdinMode::Null, 3, true);
}

#[test]
fn empty_stdin_overwrites_existing_output() {
    assert_noninteractive(StdinMode::Empty, 3, true);
}

#[test]
fn open_pipe_stdin_overwrites_existing_output() {
    assert_noninteractive(StdinMode::OpenPipe, 3, true);
}

#[test]
fn open_pipe_stdin_over_100_files_also_overwrites() {
    // Both prompts would fire: the output already exists, and the walk finds
    // more than 100 files. An open pipe must not hang on either question.
    assert_noninteractive(StdinMode::OpenPipe, 101, true);
}
