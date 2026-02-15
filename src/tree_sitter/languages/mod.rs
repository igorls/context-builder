//! Language support registry.
//!
//! This module provides access to language-specific parsers based on file extensions.

#[cfg(feature = "tree-sitter-base")]
use super::language_support::LanguageSupport;

#[cfg(feature = "tree-sitter-rust")]
mod rust;

#[cfg(feature = "tree-sitter-js")]
mod javascript;

#[cfg(feature = "tree-sitter-ts")]
mod typescript;

#[cfg(feature = "tree-sitter-python")]
mod python;

#[cfg(feature = "tree-sitter-go")]
mod go;

#[cfg(feature = "tree-sitter-java")]
mod java;

#[cfg(feature = "tree-sitter-c")]
mod c;

#[cfg(feature = "tree-sitter-cpp")]
mod cpp;

#[cfg(feature = "tree-sitter-rust")]
static RUST_SUPPORT: rust::RustSupport = rust::RustSupport;

#[cfg(feature = "tree-sitter-js")]
static JS_SUPPORT: javascript::JavaScriptSupport = javascript::JavaScriptSupport;

#[cfg(feature = "tree-sitter-ts")]
static TS_SUPPORT: typescript::TypeScriptSupport = typescript::TypeScriptSupport;

#[cfg(feature = "tree-sitter-python")]
static PYTHON_SUPPORT: python::PythonSupport = python::PythonSupport;

#[cfg(feature = "tree-sitter-go")]
static GO_SUPPORT: go::GoSupport = go::GoSupport;

#[cfg(feature = "tree-sitter-java")]
static JAVA_SUPPORT: java::JavaSupport = java::JavaSupport;

#[cfg(feature = "tree-sitter-c")]
static C_SUPPORT: c::CSupport = c::CSupport;

#[cfg(feature = "tree-sitter-cpp")]
static CPP_SUPPORT: cpp::CppSupport = cpp::CppSupport;

#[cfg(feature = "tree-sitter-base")]
pub fn get_language_support(ext: &str) -> Option<&'static dyn LanguageSupport> {
    match ext.to_lowercase().as_str() {
        #[cfg(feature = "tree-sitter-rust")]
        "rs" => Some(&RUST_SUPPORT),

        #[cfg(feature = "tree-sitter-js")]
        "js" | "mjs" | "cjs" => Some(&JS_SUPPORT),

        #[cfg(feature = "tree-sitter-ts")]
        "ts" | "tsx" | "mts" | "cts" => Some(&TS_SUPPORT),

        #[cfg(feature = "tree-sitter-python")]
        "py" | "pyw" => Some(&PYTHON_SUPPORT),

        #[cfg(feature = "tree-sitter-go")]
        "go" => Some(&GO_SUPPORT),

        #[cfg(feature = "tree-sitter-java")]
        "java" => Some(&JAVA_SUPPORT),

        #[cfg(feature = "tree-sitter-c")]
        "c" | "h" => Some(&C_SUPPORT),

        #[cfg(feature = "tree-sitter-cpp")]
        "cpp" | "cxx" | "cc" | "hpp" | "hxx" | "hh" => Some(&CPP_SUPPORT),

        _ => None,
    }
}

#[cfg(not(feature = "tree-sitter-base"))]
pub fn get_language_support(_ext: &str) -> Option<()> {
    None
}

#[cfg(feature = "tree-sitter-base")]
pub fn supported_extensions() -> Vec<&'static str> {
    let mut extensions = Vec::new();

    #[cfg(feature = "tree-sitter-rust")]
    extensions.extend(RUST_SUPPORT.file_extensions());

    #[cfg(feature = "tree-sitter-js")]
    extensions.extend(JS_SUPPORT.file_extensions());

    #[cfg(feature = "tree-sitter-ts")]
    extensions.extend(TS_SUPPORT.file_extensions());

    #[cfg(feature = "tree-sitter-python")]
    extensions.extend(PYTHON_SUPPORT.file_extensions());

    #[cfg(feature = "tree-sitter-go")]
    extensions.extend(GO_SUPPORT.file_extensions());

    #[cfg(feature = "tree-sitter-java")]
    extensions.extend(JAVA_SUPPORT.file_extensions());

    #[cfg(feature = "tree-sitter-c")]
    extensions.extend(C_SUPPORT.file_extensions());

    #[cfg(feature = "tree-sitter-cpp")]
    extensions.extend(CPP_SUPPORT.file_extensions());

    extensions
}

#[cfg(not(feature = "tree-sitter-base"))]
pub fn supported_extensions() -> Vec<&'static str> {
    Vec::new()
}
