//! `--config <path>` and `--no-config` (#21).

use std::fs;
use std::process::{Command, Output};

fn run(cwd: &std::path::Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_context-builder"))
        .args(args)
        .current_dir(cwd)
        .output()
        .unwrap()
}

fn project() -> tempfile::TempDir {
    let dir = tempfile::tempdir().unwrap();
    fs::write(dir.path().join("a.rs"), "fn a() {}\n").unwrap();
    fs::write(dir.path().join("b.py"), "x = 1\n").unwrap();
    // The project's own config only includes Python.
    fs::write(
        dir.path().join("context-builder.toml"),
        "filter = [\"py\"]\n",
    )
    .unwrap();
    dir
}

fn doc(out: &Output) -> String {
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8_lossy(&out.stdout).into_owned()
}

#[test]
fn project_config_applies_by_default() {
    let dir = project();
    let d = doc(&run(dir.path(), &["-o", "-"]));
    assert!(d.contains("b.py") && !d.contains("a.rs"), "{d}");
}

#[test]
fn no_config_ignores_the_project_config() {
    let dir = project();
    let d = doc(&run(dir.path(), &["-o", "-", "--no-config"]));
    assert!(d.contains("b.py") && d.contains("a.rs"), "{d}");
}

#[test]
fn config_flag_uses_the_named_file() {
    let dir = project();
    let elsewhere = tempfile::tempdir().unwrap();
    let alt = elsewhere.path().join("rust-only.toml");
    fs::write(&alt, "filter = [\"rs\"]\n").unwrap();
    let d = doc(&run(
        dir.path(),
        &["-o", "-", "--config", alt.to_str().unwrap()],
    ));
    assert!(d.contains("a.rs") && !d.contains("b.py"), "{d}");
}

#[test]
fn bad_explicit_config_is_an_error_and_flags_conflict() {
    let dir = project();
    let missing = run(dir.path(), &["-o", "-", "--config", "nope.toml"]);
    assert_eq!(missing.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&missing.stderr).contains("nope.toml"));

    fs::write(dir.path().join("broken.toml"), "filter = [\"rs\"\n").unwrap();
    let broken = run(dir.path(), &["-o", "-", "--config", "broken.toml"]);
    assert_eq!(broken.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&broken.stderr).contains("invalid config"));

    let both = run(dir.path(), &["--config", "broken.toml", "--no-config"]);
    assert!(!both.status.success());
}

#[test]
fn help_lists_both_flags() {
    let dir = project();
    let help = String::from_utf8_lossy(&run(dir.path(), &["--help"]).stdout).into_owned();
    assert!(
        help.contains("--config") && help.contains("--no-config"),
        "{help}"
    );
}
