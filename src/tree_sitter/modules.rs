//! Imports and exports for `--structure`: what a file pulls in and what it
//! exposes, as the declarations appear in the source.

use super::language_support::{CodeStructure, LanguageSupport};

const MAX_IMPORT_CHARS: usize = 160;
const MAX_EXPORT_CHARS: usize = 120;

/// Node kinds that are imports, exports, and the containers worth entering
/// (an `import ( … )` block in Go holds its specs one level down).
struct Kinds {
    imports: &'static [&'static str],
    exports: &'static [&'static str],
    containers: &'static [&'static str],
}

fn kinds_for(ext: &str) -> Option<Kinds> {
    Some(match ext {
        "rs" => Kinds {
            imports: &["use_declaration"],
            exports: &[],
            containers: &[],
        },
        "py" | "pyi" => Kinds {
            imports: &["import_statement", "import_from_statement"],
            exports: &[],
            containers: &[],
        },
        "js" | "jsx" | "mjs" | "cjs" | "ts" | "tsx" | "mts" | "cts" => Kinds {
            imports: &["import_statement"],
            exports: &["export_statement"],
            containers: &[],
        },
        "go" => Kinds {
            imports: &["import_spec"],
            exports: &[],
            containers: &["import_declaration", "import_spec_list"],
        },
        "java" => Kinds {
            imports: &["import_declaration"],
            exports: &[],
            containers: &[],
        },
        "c" | "h" | "cpp" | "cc" | "cxx" | "hpp" | "hh" | "hxx" => Kinds {
            imports: &["preproc_include"],
            exports: &[],
            containers: &[],
        },
        _ => return None,
    })
}

/// Fill `structure.imports` / `structure.exports` with the declarations'
/// text. Languages without a rule keep whatever the extractor produced.
pub fn fill(structure: &mut CodeStructure, source: &str, ext: &str, support: &dyn LanguageSupport) {
    let Some(kinds) = kinds_for(ext) else {
        return;
    };
    let Some(tree) = support.parse(source) else {
        return;
    };
    let mut imports = Vec::new();
    let mut exports = Vec::new();
    collect(
        &tree.root_node(),
        source,
        ext,
        &kinds,
        &mut imports,
        &mut exports,
    );
    structure.imports = imports;
    structure.exports = exports;
}

fn collect(
    node: &tree_sitter::Node,
    source: &str,
    ext: &str,
    kinds: &Kinds,
    imports: &mut Vec<String>,
    exports: &mut Vec<String>,
) {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        let kind = child.kind();
        let text = &source[child.start_byte()..child.end_byte()];
        if kinds.imports.contains(&kind) {
            // `pub use` re-exports are the Rust module's public surface.
            if ext == "rs" && text.trim_start().starts_with("pub") {
                exports.push(one_line(text, MAX_EXPORT_CHARS));
            } else {
                imports.push(one_line(text, MAX_IMPORT_CHARS));
            }
        } else if kinds.exports.contains(&kind) {
            exports.push(export_header(text));
        } else if kinds.containers.contains(&kind) {
            collect(&child, source, ext, kinds, imports, exports);
        }
    }
}

/// Whitespace collapsed to single spaces, no trailing `;`, capped.
fn one_line(text: &str, max: usize) -> String {
    let joined = text.split_whitespace().collect::<Vec<_>>().join(" ");
    let trimmed = joined.trim_end_matches(';').trim_end();
    cap(trimmed, max)
}

/// The first line of an export, without the opening brace of a body.
fn export_header(text: &str) -> String {
    let first = text.lines().next().unwrap_or("").trim();
    // `export { a, b }` / `export * from "x"` are the whole statement; for a
    // declaration, stop before its body (or object literal).
    let list_form = first.starts_with("export {") || first.starts_with("export *");
    let head = if list_form {
        first
    } else {
        first.split('{').next().unwrap_or(first)
    };
    let head = head.trim_end_matches(';').trim_end();
    let head = head.trim_end_matches('=').trim_end();
    cap(head, MAX_EXPORT_CHARS)
}

fn cap(text: &str, max: usize) -> String {
    if text.chars().count() <= max {
        text.to_string()
    } else {
        let cut: String = text.chars().take(max).collect();
        format!("{cut}…")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_line_collapses_whitespace_and_semicolons() {
        assert_eq!(
            one_line("from a import (\n    b,\n    c,\n)", 100),
            "from a import ( b, c, )"
        );
        assert_eq!(one_line("use std::io;", 100), "use std::io");
    }

    #[test]
    fn export_header_stops_before_the_body() {
        assert_eq!(
            export_header("export function f(a: number) {\n  return a;\n}"),
            "export function f(a: number)"
        );
        assert_eq!(export_header("export { a, b };"), "export { a, b }");
        assert_eq!(
            export_header("export function f(x: number) { return x; }"),
            "export function f(x: number)"
        );
        assert_eq!(
            export_header("export default class K {}"),
            "export default class K"
        );
        assert_eq!(
            export_header("export const cfg = { a: 1 };"),
            "export const cfg"
        );
        assert_eq!(
            export_header("export * from \"./x\";"),
            "export * from \"./x\""
        );
    }

    #[test]
    fn cap_marks_truncation_on_char_boundaries() {
        assert_eq!(cap("abc", 5), "abc");
        assert_eq!(cap("ééééé", 3), "ééé…");
    }

    #[test]
    fn unknown_extension_has_no_rules() {
        assert!(kinds_for("zzz").is_none());
        assert!(kinds_for("rs").is_some());
    }
}
