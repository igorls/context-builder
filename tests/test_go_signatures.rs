//! Go `--signatures` keep return types and show what a type alias aliases (#22 B16).
#![cfg(feature = "tree-sitter-go")]

use context_builder::tree_sitter::extract_signatures_for_file;
use context_builder::tree_sitter::language_support::Visibility;
use std::fs;
use std::process::Command;

const SRC: &str = r#"package cobra

import "context"

type FParseErrWhitelist flag.ParseErrorsWhitelist

type Command struct{ name string }

func (c *Command) Context() context.Context { return c.ctx }

func (c *Command) Name() (string, error) { return "", nil }

func (c *Command) Reset() { }

func New(a int, b string) *Command { return nil }

type Runner interface{ Run() error }
"#;

fn sigs() -> Vec<String> {
    extract_signatures_for_file(SRC, "go", Visibility::All)
        .unwrap()
        .into_iter()
        .map(|s| s.full_signature)
        .collect()
}

#[test]
fn methods_keep_single_and_multiple_return_types() {
    let sigs = sigs();
    assert!(
        sigs.contains(&"func (c *Command) Context() context.Context".to_string()),
        "{sigs:?}"
    );
    assert!(
        sigs.contains(&"func (c *Command) Name() (string, error)".to_string()),
        "{sigs:?}"
    );
    assert!(
        sigs.contains(&"func (c *Command) Reset()".to_string()),
        "{sigs:?}"
    );
}

#[test]
fn functions_keep_return_types() {
    assert!(sigs().contains(&"func New(a int, b string) *Command".to_string()));
}

#[test]
fn type_alias_shows_its_underlying_type() {
    let sigs = sigs();
    assert!(
        sigs.contains(&"type FParseErrWhitelist flag.ParseErrorsWhitelist".to_string()),
        "{sigs:?}"
    );
    assert!(
        !sigs.iter().any(|s| s == "type FParseErrWhitelist type"),
        "{sigs:?}"
    );
    assert!(sigs.contains(&"type Command struct".to_string()));
    assert!(sigs.contains(&"type Runner interface".to_string()));
}

#[test]
fn binary_prints_return_types() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(dir.path().join("c.go"), SRC).unwrap();
    let out = Command::new(env!("CARGO_BIN_EXE_context-builder"))
        .args([
            "-d",
            dir.path().to_str().unwrap(),
            "--signatures",
            "-o",
            "-",
        ])
        .current_dir(dir.path())
        .output()
        .unwrap();
    let doc = String::from_utf8_lossy(&out.stdout);
    assert!(
        doc.contains("func (c *Command) Context() context.Context"),
        "{doc}"
    );
    assert!(
        doc.contains("type FParseErrWhitelist flag.ParseErrorsWhitelist"),
        "{doc}"
    );
}
