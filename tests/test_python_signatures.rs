//! Python `--signatures`: no nested functions, overloads keep their return
//! types and decorators (#22 B16).
#![cfg(feature = "tree-sitter-python")]

use context_builder::tree_sitter::extract_signatures_for_file;
use context_builder::tree_sitter::language_support::Visibility;
use std::fs;
use std::process::Command;

const SRC: &str = "from typing import overload

def outer(a: int) -> int:
    def inner(b):
        return b
    class Local:
        def hidden(self): ...
    return inner(a)

class K:
    @overload
    def f(self, x: int) -> int: ...
    @overload
    def f(self, x: str) -> str: ...
    def f(self, x):
        return x
    def g(self) -> None:
        def nested(): pass

async def fetch(url: str) -> bytes:
    return b''
";

fn names() -> Vec<String> {
    extract_signatures_for_file(SRC, "py", Visibility::All)
        .unwrap()
        .into_iter()
        .map(|s| s.name)
        .collect()
}

fn full() -> Vec<String> {
    extract_signatures_for_file(SRC, "py", Visibility::All)
        .unwrap()
        .into_iter()
        .map(|s| s.full_signature)
        .collect()
}

#[test]
fn functions_nested_in_functions_are_not_listed() {
    let names = names();
    for hidden in ["inner", "nested", "hidden", "Local"] {
        assert!(
            !names.contains(&hidden.to_string()),
            "{hidden} in {names:?}"
        );
    }
    for shown in ["outer", "K", "f", "g", "fetch"] {
        assert!(
            names.contains(&shown.to_string()),
            "{shown} missing from {names:?}"
        );
    }
}

#[test]
fn overloads_keep_return_types_and_decorators_without_stray_indentation() {
    let full = full();
    assert!(
        full.contains(&"@overload\ndef f(self, x: int) -> int:".to_string()),
        "{full:?}"
    );
    assert!(
        full.contains(&"@overload\ndef f(self, x: str) -> str:".to_string()),
        "{full:?}"
    );
    assert!(full.contains(&"def f(self, x):".to_string()), "{full:?}");
    assert!(full.iter().all(|s| !s.contains("\n    ")), "{full:?}");
}

#[test]
fn async_and_annotated_functions_keep_their_signature() {
    assert!(
        full().contains(&"async def fetch(url: str) -> bytes:".to_string()),
        "{:?}",
        full()
    );
}

#[test]
fn binary_lists_overloads_with_returns_and_skips_nested() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(dir.path().join("m.py"), SRC).unwrap();
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
    assert!(doc.contains("def f(self, x: int) -> int:"), "{doc}");
    assert!(!doc.contains("def inner"), "{doc}");
    assert!(!doc.contains("def nested"), "{doc}");
}

#[test]
fn tab_indented_decorated_methods_are_dedented_too() {
    let src = "class K:\n\t@overload\n\tdef f(self, x: int) -> int: ...\n\t@overload\n\tdef f(self, x: str) -> str: ...\n";
    let full: Vec<String> = extract_signatures_for_file(src, "py", Visibility::All)
        .unwrap()
        .into_iter()
        .map(|s| s.full_signature)
        .collect();
    assert!(
        full.contains(&"@overload\ndef f(self, x: int) -> int:".to_string()),
        "{full:?}"
    );
    assert!(full.iter().all(|s| !s.contains('\t')), "{full:?}");
}
