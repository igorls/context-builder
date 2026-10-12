//! `extract_structure_for_file` fills `code_lines` for every language, not
//! just Rust (#22 B15). Exercises the library directly.
#![cfg(feature = "tree-sitter-base")]

use context_builder::tree_sitter::extract_structure_for_file;

#[test]
fn python_code_lines_skip_blanks_and_hash_comments() {
    let src = "# header\n\nimport os\n\n# note\ndef f():\n    return 1\n";
    let s = extract_structure_for_file(src, "py").unwrap();
    assert_eq!(s.code_lines, 3, "import, def, return");
}

#[test]
fn slash_style_languages_skip_blanks_and_block_comments() {
    let src = "// line comment\n/* block\n * body\n */\n\nfunction f() {\n  return 1;\n}\n";
    let s = extract_structure_for_file(src, "js").unwrap();
    assert_eq!(s.code_lines, 3, "function, return, closing brace");
}

#[test]
fn rust_keeps_its_own_counter() {
    let s = extract_structure_for_file("// c\nfn a() {}\n", "rs").unwrap();
    assert!(s.code_lines >= 1);
}

#[test]
fn unsupported_extension_is_none() {
    assert!(extract_structure_for_file("x", "unknownext").is_none());
}
