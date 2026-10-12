//! Error and non-interactive paths of `file_utils`, exercised in-process.

use context_builder::file_utils::{collect_files, confirm_overwrite};
use std::fs;
use std::io::{ErrorKind, IsTerminal};
use tempfile::tempdir;

#[test]
fn invalid_user_ignore_pattern_is_an_error_naming_the_pattern() {
    let dir = tempdir().unwrap();
    fs::write(dir.path().join("a.txt"), "x").unwrap();

    let err = collect_files(dir.path(), &[], &["[".to_string()], &[]).unwrap_err();
    assert_eq!(err.kind(), ErrorKind::InvalidInput);
    assert!(
        err.to_string().contains("Invalid ignore pattern '['"),
        "{err}"
    );
}

#[test]
fn invalid_auto_ignore_pattern_is_skipped_not_fatal() {
    let dir = tempdir().unwrap();
    fs::write(dir.path().join("a.txt"), "x").unwrap();

    let files = collect_files(dir.path(), &[], &[], &["[".to_string()]).unwrap();
    assert_eq!(files.len(), 1);
}

#[test]
fn overwrite_is_confirmed_without_prompting_when_not_a_terminal() {
    if std::io::stdin().is_terminal() && std::io::stderr().is_terminal() {
        return; // would prompt; only the non-interactive path is under test
    }
    assert!(confirm_overwrite("whatever.md").unwrap());
}
