//! Rust `--signatures` list methods under their `impl`/`trait` (#22 B16),
//! so several `pub fn new` are no longer ambiguous.
#![cfg(feature = "tree-sitter-rust")]

use context_builder::tree_sitter::extract_signatures_for_file;
use context_builder::tree_sitter::language_support::Visibility;
use std::fs;
use std::process::Command;

const SRC: &str = "pub struct A;
pub struct B;

impl A {
    pub fn new() -> A { A }
    pub fn go(&self) {}
    fn hidden(&self) {}
}

impl B {
    pub fn new() -> B { B }
}

impl Default for B {
    fn default() -> B { B }
}

pub trait Shape {
    fn area(&self) -> f64;
    fn name(&self) -> String { String::new() }
}

pub fn free() {}
";

fn texts(v: Visibility) -> Vec<String> {
    extract_signatures_for_file(SRC, "rs", v)
        .unwrap()
        .into_iter()
        .map(|s| s.full_signature)
        .collect()
}

#[test]
fn each_new_sits_under_its_own_impl() {
    let all = texts(Visibility::All);
    assert!(
        all.contains(
            &"impl A {\n    pub fn new() -> A\n    pub fn go(&self)\n    fn hidden(&self)\n}"
                .to_string()
        ),
        "{all:?}"
    );
    assert!(
        all.contains(&"impl B {\n    pub fn new() -> B\n}".to_string()),
        "{all:?}"
    );
    assert!(
        all.contains(&"impl Default for B {\n    fn default() -> B\n}".to_string()),
        "{all:?}"
    );
}

#[test]
fn trait_members_are_grouped_and_methods_are_not_repeated_flat() {
    let all = texts(Visibility::All);
    assert!(
        all.contains(
            &"pub trait Shape {\n    fn area(&self) -> f64\n    fn name(&self) -> String\n}"
                .to_string()
        ),
        "{all:?}"
    );
    assert!(
        !all.iter()
            .any(|s| s.starts_with("pub fn new") || s == "pub fn go(&self)"),
        "{all:?}"
    );
    assert!(all.contains(&"pub fn free()".to_string()));
}

#[test]
fn public_filter_hides_private_members_and_empty_impls() {
    let public = texts(Visibility::Public);
    assert!(
        public.contains(&"impl A {\n    pub fn new() -> A\n    pub fn go(&self)\n}".to_string()),
        "{public:?}"
    );
    // `impl Default for B` has only a private-looking `fn default`: nothing to show.
    assert!(!public.iter().any(|s| s.contains("Default")), "{public:?}");
}

#[test]
fn binary_groups_methods() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(dir.path().join("lib.rs"), SRC).unwrap();
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
    assert!(doc.contains("impl A {\n    pub fn new() -> A"), "{doc}");
    assert!(doc.contains("impl B {\n    pub fn new() -> B\n}"), "{doc}");
}

#[test]
fn associated_consts_and_types_stay_with_their_trait_and_impl() {
    let src = "pub trait T {\n    const N: usize;\n    type Out;\n    fn f(&self) -> Self::Out;\n}\nimpl T for S {\n    const N: usize = 3;\n    type Out = u8;\n    fn f(&self) -> u8 { 0 }\n}\nimpl S {\n    pub const MAX: u32 = 9;\n    const HIDDEN: u32 = 1;\n}\n";
    let all: Vec<String> = extract_signatures_for_file(src, "rs", Visibility::All)
        .unwrap()
        .into_iter()
        .map(|s| s.full_signature)
        .collect();
    assert!(
        all.contains(
            &"pub trait T {\n    const N: usize\n    type Out\n    fn f(&self) -> Self::Out\n}"
                .to_string()
        ),
        "{all:?}"
    );
    assert!(
        all.contains(
            &"impl T for S {\n    const N: usize = 3\n    type Out = u8\n    fn f(&self) -> u8\n}"
                .to_string()
        ),
        "{all:?}"
    );
    let public: Vec<String> = extract_signatures_for_file(src, "rs", Visibility::Public)
        .unwrap()
        .into_iter()
        .map(|s| s.full_signature)
        .collect();
    assert!(
        public.contains(&"impl S {\n    pub const MAX: u32 = 9\n}".to_string()),
        "{public:?}"
    );
    let private: Vec<String> = extract_signatures_for_file(src, "rs", Visibility::Private)
        .unwrap()
        .into_iter()
        .map(|s| s.full_signature)
        .collect();
    assert!(
        private
            .iter()
            .any(|s| s.contains("const HIDDEN") && !s.contains("MAX")),
        "{private:?}"
    );
}
