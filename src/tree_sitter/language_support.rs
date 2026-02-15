//! Core types and traits for language support.

use std::fmt;
use std::str::FromStr;

/// The kind of signature extracted from source code.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SignatureKind {
    Function,
    Method,
    Struct,
    Enum,
    Trait,
    Interface,
    Class,
    Impl,
    Module,
    Constant,
    TypeAlias,
    Macro,
}

impl fmt::Display for SignatureKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SignatureKind::Function => write!(f, "function"),
            SignatureKind::Method => write!(f, "method"),
            SignatureKind::Struct => write!(f, "struct"),
            SignatureKind::Enum => write!(f, "enum"),
            SignatureKind::Trait => write!(f, "trait"),
            SignatureKind::Interface => write!(f, "interface"),
            SignatureKind::Class => write!(f, "class"),
            SignatureKind::Impl => write!(f, "impl"),
            SignatureKind::Module => write!(f, "module"),
            SignatureKind::Constant => write!(f, "constant"),
            SignatureKind::TypeAlias => write!(f, "type"),
            SignatureKind::Macro => write!(f, "macro"),
        }
    }
}

/// Visibility level of a signature.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Visibility {
    #[default]
    All,
    Public,
    Private,
}

impl FromStr for Visibility {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_lowercase().as_str() {
            "public" => Visibility::Public,
            "private" => Visibility::Private,
            _ => Visibility::All,
        })
    }
}

impl Visibility {
    /// Check if a symbol's visibility passes the filter.
    /// Returns `true` if the symbol should be included.
    pub fn matches_filter(self, filter: Visibility) -> bool {
        match filter {
            Visibility::All => true,
            Visibility::Public => self == Visibility::Public,
            Visibility::Private => self == Visibility::Private,
        }
    }
}

/// A signature extracted from source code (function, class, etc.).
#[derive(Debug, Clone)]
pub struct Signature {
    pub kind: SignatureKind,
    pub name: String,
    pub params: Option<String>,
    pub return_type: Option<String>,
    pub visibility: Visibility,
    pub line_number: usize,
    pub full_signature: String,
}

impl fmt::Display for Signature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.full_signature)
    }
}

/// Slice the source text from a node's start to its body's start, producing
/// a perfect signature that preserves all modifiers, generics, params, and return types.
///
/// `body_kinds` is a list of node kinds that represent the function/class body
/// (e.g., `"block"`, `"compound_statement"`, `"statement_block"`).
///
/// Returns `None` if no body node is found (e.g., forward declarations).
pub fn slice_signature_before_body(
    source: &str,
    node: &tree_sitter::Node,
    body_kinds: &[&str],
) -> Option<String> {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if body_kinds.contains(&child.kind()) {
            let sig = &source[node.start_byte()..child.start_byte()];
            return Some(sig.trim_end().to_string());
        }
    }
    None
}

/// Structure information extracted from a source file.
#[derive(Debug, Clone, Default)]
pub struct CodeStructure {
    pub imports: Vec<String>,
    pub exports: Vec<String>,
    pub functions: usize,
    pub structs: usize,
    pub enums: usize,
    pub traits: usize,
    pub classes: usize,
    pub interfaces: usize,
    pub constants: usize,
    pub type_aliases: usize,
    pub macros: usize,
    pub total_lines: usize,
    pub code_lines: usize,
}

impl CodeStructure {
    pub fn total_symbols(&self) -> usize {
        self.functions
            + self.structs
            + self.enums
            + self.traits
            + self.classes
            + self.interfaces
            + self.constants
            + self.type_aliases
            + self.macros
    }
}

/// Trait for language-specific parsing support.
pub trait LanguageSupport: Send + Sync {
    fn file_extensions(&self) -> &[&'static str];

    fn supports_extension(&self, ext: &str) -> bool {
        self.file_extensions()
            .iter()
            .any(|&e| e.eq_ignore_ascii_case(ext))
    }

    fn parse(&self, source: &str) -> Option<tree_sitter::Tree>;

    fn extract_signatures(&self, source: &str, visibility: Visibility) -> Vec<Signature>;

    fn extract_structure(&self, source: &str) -> CodeStructure;

    fn find_truncation_point(&self, source: &str, max_bytes: usize) -> usize;
}
