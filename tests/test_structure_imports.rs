//! `--structure` lists imports and exports (#22 B15), as `--help` always promised.
#![cfg(feature = "tree-sitter-base")]

use context_builder::tree_sitter::extract_structure_for_file;
use context_builder::tree_sitter::structure::format_structure_as_markdown;
use std::fs;
use std::process::Command;

fn lists(src: &str, ext: &str) -> (Vec<String>, Vec<String>) {
    let s = extract_structure_for_file(src, ext).unwrap();
    (s.imports, s.exports)
}

#[test]
fn rust_use_and_pub_use() {
    let (i, e) = lists(
        "use std::io;\nuse std::{fs, path::Path};\npub use crate::x::Y;\nfn f() { use std::fmt; }\n",
        "rs",
    );
    assert_eq!(i, ["use std::io", "use std::{fs, path::Path}"]);
    assert_eq!(e, ["pub use crate::x::Y"]);
}

#[test]
fn typescript_imports_and_exports() {
    let (i, e) = lists(
        "import { a } from \"./a\";\nexport function f(x: number) { return x; }\nexport default class K {}\nexport { a };\n",
        "ts",
    );
    assert_eq!(i, ["import { a } from \"./a\""]);
    assert_eq!(
        e,
        [
            "export function f(x: number)",
            "export default class K",
            "export { a }"
        ]
    );
}

#[test]
fn python_multiline_import_is_one_entry_and_nested_import_is_skipped() {
    let (i, _) = lists(
        "import os\nfrom typing import (\n    List,\n    Dict,\n)\ndef g():\n    import json\n",
        "py",
    );
    assert_eq!(i, ["import os", "from typing import ( List, Dict, )"]);
}

#[test]
fn go_block_and_single_imports() {
    let (i, _) = lists(
        "package main\n\nimport (\n\t\"fmt\"\n\tstr \"strings\"\n)\n\nimport \"os\"\n",
        "go",
    );
    assert_eq!(i, ["\"fmt\"", "str \"strings\"", "\"os\""]);
}

#[test]
fn c_includes_and_java_imports() {
    let (i, _) = lists("#include <stdio.h>\n#include \"x.h\"\nint main(){}\n", "c");
    assert_eq!(i, ["#include <stdio.h>", "#include \"x.h\""]);
    let (i, _) = lists("import java.util.List;\nclass A {}\n", "java");
    assert_eq!(i, ["import java.util.List"]);
}

#[test]
fn markdown_lists_them_and_caps_long_lists() {
    let mut src = String::new();
    for n in 0..25 {
        src.push_str(&format!("use crate::m{n};\n"));
    }
    let s = extract_structure_for_file(&src, "rs").unwrap();
    let md = format_structure_as_markdown(&s);
    assert!(md.contains("**Imports:**"), "{md}");
    assert!(md.contains("- `use crate::m0`"), "{md}");
    assert!(md.contains("- … and 5 more"), "{md}");
    assert!(!md.contains("**Exports:**"));
}

#[test]
fn a_file_with_only_imports_still_gets_a_structure_block() {
    let s = extract_structure_for_file("use std::io;\n", "rs").unwrap();
    let md = format_structure_as_markdown(&s);
    assert!(md.starts_with("**Structure:**"), "{md}");
    assert!(md.contains("`use std::io`"), "{md}");
}

#[test]
fn binary_prints_imports_and_exports() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(
        dir.path().join("a.ts"),
        "import b from 'b';\nexport const x = 1;\n",
    )
    .unwrap();
    let out = Command::new(env!("CARGO_BIN_EXE_context-builder"))
        .args(["-d", dir.path().to_str().unwrap(), "--structure", "-o", "-"])
        .current_dir(dir.path())
        .output()
        .unwrap();
    let doc = String::from_utf8_lossy(&out.stdout);
    assert!(doc.contains("**Imports:**\n- `import b from 'b'`"), "{doc}");
    assert!(
        doc.contains("**Exports:**\n- `export const x = 1`"),
        "{doc}"
    );
}
