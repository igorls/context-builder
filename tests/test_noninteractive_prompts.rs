//! Prompts and the large-output warning (dogfood B4, then v0.11.0 issue #23 (b)).
//!
//! The >100-file confirmation was removed entirely. These tests spawn the real
//! binary: more than 100 files must exit 0 on null stdin, an empty pipe, an
//! open pipe, and (on Unix) a TTY, with no "Continue?" prompt. The overwrite
//! prompt remains on a TTY and is written to stderr. A timeout fails the test
//! if the process blocks.

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
    // The output already exists and the walk finds more than 100 files.
    // Non-TTY stdin skips the overwrite prompt and never asks about file count.
    assert_noninteractive(StdinMode::OpenPipe, 101, true);
}

#[test]
fn large_output_warning_goes_to_stderr_not_stdout() {
    let dir = tempdir().unwrap();
    let fixture = dir.path().join("repo");
    fs::create_dir_all(&fixture).unwrap();
    // bytes/4 must exceed the 128K warning threshold. 600_000 bytes of text
    // plus the markdown wrapper is well over 512_000 output bytes.
    fs::write(fixture.join("big.txt"), "a".repeat(600_000)).unwrap();

    let output = dir.path().join("out.md");
    let file_run = run_binary(
        &fixture.to_string_lossy(),
        &output.to_string_lossy(),
        StdinMode::Null,
    );
    assert!(
        file_run.status.success(),
        "exit {:?}\nstderr:\n{}",
        file_run.status.code(),
        file_run.stderr
    );
    assert_warning_on_stderr_only(&file_run);

    let pipe_run = run_binary(&fixture.to_string_lossy(), "-", StdinMode::Null);
    assert!(
        pipe_run.status.success(),
        "pipe exit {:?}\nstderr:\n{}",
        pipe_run.status.code(),
        pipe_run.stderr
    );
    assert!(
        pipe_run.stdout.contains("# Directory Structure Report"),
        "piped stdout should be the document"
    );
    assert_warning_on_stderr_only(&pipe_run);
}

fn assert_warning_on_stderr_only(run: &RunOutput) {
    assert!(
        run.stderr.contains("recommended limit is 128K"),
        "large-output warning missing from stderr:\n{}",
        run.stderr
    );
    assert!(
        run.stderr.contains("Estimated tokens:"),
        "token estimate missing from stderr:\n{}",
        run.stderr
    );
    assert!(
        !run.stdout.contains("recommended limit"),
        "warning leaked to stdout:\n{}",
        run.stdout
    );
    assert!(
        !run.stdout.contains("Estimated tokens:"),
        "token estimate leaked to stdout:\n{}",
        run.stdout
    );
    assert!(
        !run.stdout.contains("128K"),
        "warning leaked to stdout:\n{}",
        run.stdout
    );
    assert!(!run.stdout.contains("Continue?"));
    assert!(!run.stderr.contains("Continue?"));
}

/// More than 100 files on a real terminal must not ask to continue.
#[cfg(unix)]
#[test]
fn tty_over_100_files_does_not_prompt() {
    let dir = tempdir().unwrap();
    let fixture = dir.path().join("repo");
    let output = dir.path().join("out.md");
    write_fixture(&fixture, 101);

    let run = run_on_pty(
        &fixture.to_string_lossy(),
        &output.to_string_lossy(),
        &[],
        None,
    );
    assert!(
        run.status.success(),
        "TTY run should not wait for confirmation, exit {:?}\nstderr:\n{}",
        run.status.code(),
        run.stderr
    );
    assert!(!run.stdout.contains("Continue?"));
    assert!(!run.stderr.contains("Continue?"));
    assert!(!run.stderr.contains("might take a while"));
    let body = fs::read_to_string(&output).unwrap();
    assert!(body.contains(MARKER));
}

/// The overwrite prompt is unchanged on a TTY: stderr, default no, `-y` skips it.
#[cfg(unix)]
#[test]
fn tty_overwrite_prompt_unchanged() {
    let dir = tempdir().unwrap();
    let fixture = dir.path().join("repo");
    let output = dir.path().join("out.md");
    write_fixture(&fixture, 3);
    fs::write(&output, SENTINEL).unwrap();

    let declined = run_on_pty(
        &fixture.to_string_lossy(),
        &output.to_string_lossy(),
        &[],
        Some(b"n\n"),
    );
    assert!(
        !declined.status.success(),
        "declining overwrite should cancel, stderr:\n{}",
        declined.stderr
    );
    assert!(
        declined.stderr.contains("Overwrite?"),
        "overwrite prompt should be on stderr:\n{}",
        declined.stderr
    );
    assert!(
        !declined.stdout.contains("Overwrite?"),
        "overwrite prompt leaked to stdout:\n{}",
        declined.stdout
    );
    assert_eq!(fs::read_to_string(&output).unwrap(), SENTINEL);

    fs::write(&output, SENTINEL).unwrap();
    let accepted = run_on_pty(
        &fixture.to_string_lossy(),
        &output.to_string_lossy(),
        &[],
        Some(b"y\n"),
    );
    assert!(
        accepted.status.success(),
        "accepting overwrite should succeed, stderr:\n{}",
        accepted.stderr
    );
    let body = fs::read_to_string(&output).unwrap();
    assert!(body.contains("# Directory Structure Report"));
    assert!(!body.contains(SENTINEL));

    fs::write(&output, SENTINEL).unwrap();
    let yes = run_on_pty(
        &fixture.to_string_lossy(),
        &output.to_string_lossy(),
        &["--yes"],
        None,
    );
    assert!(
        yes.status.success(),
        "`--yes` should overwrite without asking, exit {:?}\nstderr:\n{}",
        yes.status.code(),
        yes.stderr
    );
    assert!(
        !yes.stderr.contains("Overwrite?"),
        "`--yes` still asked:\n{}",
        yes.stderr
    );
    assert!(!yes.stdout.contains("Overwrite?"));
    assert!(fs::read_to_string(&output).unwrap().contains(MARKER));
}

#[cfg(unix)]
fn run_on_pty(input: &str, output: &str, extra: &[&str], answer: Option<&[u8]>) -> RunOutput {
    use std::io::Write;
    use std::os::fd::{FromRawFd, OwnedFd};

    let mut master_fd: libc::c_int = -1;
    let mut slave_fd: libc::c_int = -1;
    let rc = unsafe {
        libc::openpty(
            &mut master_fd,
            &mut slave_fd,
            std::ptr::null_mut(),
            std::ptr::null_mut::<libc::termios>(),
            std::ptr::null_mut::<libc::winsize>(),
        )
    };
    assert_eq!(rc, 0, "openpty failed");
    let slave = unsafe { OwnedFd::from_raw_fd(slave_fd) };
    let mut master = std::fs::File::from(unsafe { OwnedFd::from_raw_fd(master_fd) });

    let mut child = Command::new(bin_path())
        .args(["-d", input, "-o", output])
        .args(extra)
        .env_remove("CB_SILENT")
        .stdin(Stdio::from(slave))
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to spawn context-builder");

    if let Some(bytes) = answer {
        // Queued before the child reads, so a prompt receives the answer
        // without the test racing the question.
        master.write_all(bytes).expect("write pty answer");
        let _ = master.flush();
    }

    let stdout_pipe = child.stdout.take().expect("piped stdout");
    let stderr_pipe = child.stderr.take().expect("piped stderr");
    let stdout_thread = thread::spawn(move || read_pipe(stdout_pipe));
    let stderr_thread = thread::spawn(move || read_pipe(stderr_pipe));

    let waited = wait_timeout(&mut child, TIMEOUT);
    drop(master);

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
