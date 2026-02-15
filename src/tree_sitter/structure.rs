//! Code structure extraction utilities.

use super::language_support::{CodeStructure, LanguageSupport};

/// Extract structure information from source code.
pub fn extract_structure(source: &str, support: &dyn LanguageSupport) -> CodeStructure {
    support.extract_structure(source)
}

/// Format structure as markdown summary.
pub fn format_structure_as_markdown(structure: &CodeStructure) -> String {
    if structure.total_symbols() == 0 {
        return String::new();
    }

    let mut output = String::new();
    output.push_str("**Structure:**\n");

    let mut parts = Vec::new();

    if structure.functions > 0 {
        parts.push(format!("{} functions", structure.functions));
    }
    if structure.structs > 0 {
        parts.push(format!("{} structs", structure.structs));
    }
    if structure.classes > 0 {
        parts.push(format!("{} classes", structure.classes));
    }
    if structure.enums > 0 {
        parts.push(format!("{} enums", structure.enums));
    }
    if structure.traits > 0 {
        parts.push(format!("{} traits", structure.traits));
    }
    if structure.interfaces > 0 {
        parts.push(format!("{} interfaces", structure.interfaces));
    }
    if structure.constants > 0 {
        parts.push(format!("{} constants", structure.constants));
    }
    if structure.type_aliases > 0 {
        parts.push(format!("{} types", structure.type_aliases));
    }
    if structure.macros > 0 {
        parts.push(format!("{} macros", structure.macros));
    }

    output.push_str("- ");
    output.push_str(&parts.join(", "));
    output.push('\n');

    if structure.total_lines > 0 {
        output.push_str(&format!(
            "- {} lines ({} code)\n",
            structure.total_lines, structure.code_lines
        ));
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_empty_structure() {
        let structure = CodeStructure::default();
        let output = format_structure_as_markdown(&structure);
        assert!(output.is_empty());
    }

    #[test]
    fn test_format_structure_with_symbols() {
        let structure = CodeStructure {
            functions: 5,
            structs: 2,
            enums: 1,
            total_lines: 100,
            code_lines: 80,
            ..Default::default()
        };

        let output = format_structure_as_markdown(&structure);
        assert!(output.contains("5 functions"));
        assert!(output.contains("2 structs"));
        assert!(output.contains("1 enums"));
        assert!(output.contains("100 lines"));
        assert!(output.contains("80 code"));
    }
}
