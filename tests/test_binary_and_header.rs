//! Binary files are reported, not dumped as placeholder blocks, and the header
//! names the directory instead of leaking an absolute path (#21 P11).

use context_builder::cli::Args;
use context_builder::config::Config;
use context_builder::content_filter::{ContentPolicy, SkipReason, classify};
use context_builder::{Prompter, run_with_args};
use std::fs;
use std::path::Path;
use std::process::Command;

struct Yes;

impl Prompter for Yes {
    fn confirm_overwrite(&self, _file_path: &str) -> std::io::Result<bool> {
        Ok(true)
    }
}

fn args(input: &Path, output: &Path) -> Args {
    Args {
        input: input.to_string_lossy().to_string(),
        output: output.to_string_lossy().to_string(),
        filter: vec![],
        ignore: vec![],
        line_numbers: false,
        preview: false,
        token_count: false,
        yes: true,
        diff_only: false,
        clear_cache: false,
        encoding: "o200k_base".to_string(),
        init: false,
        max_tokens: None,
        signatures: false,
        structure: false,
        truncate: "smart".to_string(),
        visibility: "all".to_string(),
        file_metadata: false,
        max_file_size: "256K".to_string(),
        hidden: false,
        include_secrets: false,
        include_lockfiles: false,
    }
}

fn project() -> tempfile::TempDir {
    let dir = tempfile::Builder::new()
        .prefix("my proj ")
        .tempdir()
        .unwrap();
    fs::write(dir.path().join("a.txt"), "hello\n").unwrap();
    // Not a known asset extension, but clearly binary.
    fs::write(dir.path().join("blob.dat"), [0u8, 1, 2, 3, 0, 0, 255, 254]).unwrap();
    // NUL-padded UTF-16 with a BOM is text.
    let mut utf16 = vec![0xFF, 0xFE];
    for c in "wide text\n".encode_utf16() {
        utf16.extend_from_slice(&c.to_le_bytes());
    }
    fs::write(dir.path().join("wide.txt"), utf16).unwrap();
    dir
}

#[test]
fn classify_marks_nul_content_as_binary_but_not_utf16_or_plain_text() {
    let dir = project();
    let policy = ContentPolicy::new("256K", &[], false);
    let verdict = |name: &str| {
        let path = dir.path().join(name);
        classify(&path, fs::metadata(&path).unwrap().len(), &policy).map(|(r, _)| r)
    };
    assert_eq!(verdict("blob.dat"), Some(SkipReason::Binary));
    assert_eq!(verdict("a.txt"), None);
    assert_eq!(verdict("wide.txt"), None);
    // A path that cannot be opened is left to the renderer.
    assert_eq!(classify(&dir.path().join("missing.dat"), 10, &policy), None);
}

#[test]
fn document_skips_binary_in_the_body_and_lists_it() {
    let dir = project();
    let out = dir.path().join("out.md");
    run_with_args(args(dir.path(), &out), Config::default(), &Yes).unwrap();
    let doc = fs::read_to_string(&out).unwrap();
    assert!(doc.contains("- `blob.dat` — binary"), "{doc}");
    assert!(!doc.contains("### File: `blob.dat`"), "{doc}");
    assert!(!doc.contains("<Binary file"), "{doc}");
    assert!(doc.contains("### File: `a.txt`"));
    assert!(doc.contains("### File: `wide.txt`"), "UTF-16 text stays");
}

#[test]
fn header_names_the_directory_not_its_absolute_path() {
    let dir = project();
    let out = dir.path().join("out.md");
    run_with_args(args(dir.path(), &out), Config::default(), &Yes).unwrap();
    let doc = fs::read_to_string(&out).unwrap();
    let name = dir
        .path()
        .file_name()
        .unwrap()
        .to_string_lossy()
        .to_string();
    let parent = dir.path().parent().unwrap().to_string_lossy().to_string();
    assert!(
        doc.contains(&format!("from the `{name}` directory")),
        "{doc}"
    );
    assert!(!doc.contains(&parent), "absolute path leaked:\n{doc}");
}

#[test]
fn binary_prints_summary_with_binary_count_and_clean_header() {
    let dir = project();
    let out = Command::new(env!("CARGO_BIN_EXE_context-builder"))
        .args(["-d", dir.path().to_str().unwrap(), "-o", "-", "-y"])
        .current_dir(std::env::temp_dir())
        .output()
        .unwrap();
    assert!(out.status.success());
    let doc = String::from_utf8_lossy(&out.stdout);
    let err = String::from_utf8_lossy(&out.stderr);
    assert!(err.contains("1 binary"), "{err}");
    let parent = dir.path().parent().unwrap().to_string_lossy().to_string();
    assert!(!doc.contains(&parent), "{doc}");
    assert!(!doc.contains("<Binary file"));
}

#[test]
fn header_keeps_a_symlink_name_and_never_falls_back_to_a_path() {
    let dir = project();
    let cwd = dir.path().join("sub");
    fs::create_dir(&cwd).unwrap();
    fs::write(cwd.join("x.txt"), "x").unwrap();
    // `..` resolves to the parent's name; `/` has no name and must not print `/`.
    let out = dir.path().join("out.md");
    let mut a = args(&cwd.join(".."), &out);
    a.input = format!("{}/..", cwd.display());
    run_with_args(a, Config::default(), &Yes).unwrap();
    let doc = fs::read_to_string(&out).unwrap();
    let name = dir
        .path()
        .file_name()
        .unwrap()
        .to_string_lossy()
        .to_string();
    assert!(
        doc.contains(&format!("from the `{name}` directory")),
        "{doc}"
    );

    #[cfg(unix)]
    {
        let link = dir.path().join("alias-name");
        std::os::unix::fs::symlink(&cwd, &link).unwrap();
        let out2 = dir.path().join("out2.md");
        run_with_args(args(&link, &out2), Config::default(), &Yes).unwrap();
        let doc2 = fs::read_to_string(&out2).unwrap();
        assert!(doc2.contains("from the `alias-name` directory"), "{doc2}");
    }
}

#[test]
fn several_binaries_are_summarized_in_the_plural() {
    let dir = project();
    fs::write(dir.path().join("two.dat"), [0u8, 9, 9]).unwrap();
    let out = Command::new(env!("CARGO_BIN_EXE_context-builder"))
        .args(["-d", dir.path().to_str().unwrap(), "-o", "-", "-y"])
        .current_dir(std::env::temp_dir())
        .output()
        .unwrap();
    assert!(String::from_utf8_lossy(&out.stderr).contains("(2 binaries)"));
}
