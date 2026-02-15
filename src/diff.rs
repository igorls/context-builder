use similar::{ChangeTag, TextDiff};
use std::collections::HashMap;

/// Line based diff utilities.
///
/// This module previously exposed `generate_diff` which produced a single
/// "## File Differences" section for an entire markdown document. That
/// approach made it easy for volatile sections (timestamps, file tree
/// structure, etc.) to create noisy diffs. To address this the new
/// per‑file API lets the caller diff only the normalized *file content*
/// blocks that appear under each `### File: `path`` heading in the
/// canonical output, completely ignoring the global header or the file
/// tree portion. Each file receives an isolated unified style diff.
///
/// High level additions:
/// * `PerFileStatus` – classification of the change.
/// * `PerFileDiff` – structured diff result for a single file.
/// * `diff_file_contents` – core engine producing diffs per file without any
///   global "## File Differences" header.
/// * `render_per_file_diffs` – helper to render the per file diffs into
///   markdown (still omits a global header so the caller can choose).
///
/// Backwards compatibility: the existing `generate_diff` function (full
/// document diff) is retained for now. New code should prefer the
/// per‑file functions.
/// Determine number of context lines either from explicit argument or env.
fn resolve_context_lines(explicit: Option<usize>) -> usize {
    explicit
        .filter(|v| *v > 0)
        .or_else(|| {
            std::env::var("CB_DIFF_CONTEXT_LINES")
                .ok()
                .and_then(|v| v.parse().ok())
                .filter(|v: &usize| *v > 0)
        })
        .unwrap_or(3)
}

/// Original API: produce a single markdown section headed by "## File Differences".
/// (Kept unchanged for compatibility.)
pub fn generate_diff(old_content: &str, new_content: &str) -> String {
    let diff = TextDiff::from_lines(old_content, new_content);
    if diff.ratio() == 1.0 {
        return String::new();
    }
    let context_lines = resolve_context_lines(None);
    let grouped = diff.grouped_ops(context_lines);
    let mut out = String::new();
    out.push_str("## File Differences\n\n");
    out.push_str("```diff\n");
    for (group_index, group) in grouped.iter().enumerate() {
        if group_index > 0 {
            out.push_str("  ...\n");
        }
        // Emit standard unified diff hunk header for positional context
        if let (Some(first), Some(last)) = (group.first(), group.last()) {
            let old_start = first.old_range().start + 1;
            let old_len = last.old_range().end - first.old_range().start;
            let new_start = first.new_range().start + 1;
            let new_len = last.new_range().end - first.new_range().start;
            out.push_str(&format!(
                "@@ -{},{} +{},{} @@\n",
                old_start, old_len, new_start, new_len
            ));
        }
        for op in group {
            for change in diff.iter_changes(op) {
                let tag = change.tag();
                let mut line = change.to_string();
                if line.ends_with('\n') {
                    line.pop();
                    if line.ends_with('\r') {
                        line.pop();
                    }
                }

                match tag {
                    ChangeTag::Delete => {
                        out.push_str("- ");
                        out.push_str(&line);
                        out.push('\n');
                    }
                    ChangeTag::Insert => {
                        out.push_str("+ ");
                        out.push_str(&line);
                        out.push('\n');
                    }
                    ChangeTag::Equal => {
                        out.push_str("  ");
                        out.push_str(&line);
                        out.push('\n');
                    }
                }
            }
        }
    }
    out.push_str("```\n\n");
    out
}

/// Classification of how a file changed between two snapshots.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PerFileStatus {
    Added,
    Removed,
    Modified,
    Unchanged,
}

/// Structured diff result for a single file.
#[derive(Debug, Clone)]
pub struct PerFileDiff {
    pub path: String,
    pub status: PerFileStatus,
    /// Unified diff fenced in ```diff (omitted when status == Unchanged and skip_unchanged=true)
    pub diff: String,
}

impl PerFileDiff {
    pub fn is_changed(&self) -> bool {
        self.status != PerFileStatus::Unchanged
    }
}

/// Produce a unified style diff for two text blobs WITHOUT adding any global
/// section header. Returns empty string if contents are identical.
fn unified_no_header(old: &str, new: &str, context_lines: usize) -> String {
    let diff = TextDiff::from_lines(old, new);
    if diff.ratio() == 1.0 {
        return String::new();
    }
    let grouped = diff.grouped_ops(context_lines);
    let mut out = String::new();
    out.push_str("```diff\n");
    for (group_index, group) in grouped.iter().enumerate() {
        if group_index > 0 {
            out.push_str("  ...\n");
        }
        // Emit standard unified diff hunk header for positional context
        if let (Some(first), Some(last)) = (group.first(), group.last()) {
            let old_start = first.old_range().start + 1;
            let old_len = last.old_range().end - first.old_range().start;
            let new_start = first.new_range().start + 1;
            let new_len = last.new_range().end - first.new_range().start;
            out.push_str(&format!(
                "@@ -{},{} +{},{} @@\n",
                old_start, old_len, new_start, new_len
            ));
        }
        for op in group {
            for change in diff.iter_changes(op) {
                let tag = change.tag();
                let mut line = change.to_string();
                if line.ends_with('\n') {
                    line.pop();
                    if line.ends_with('\r') {
                        line.pop();
                    }
                }

                match tag {
                    ChangeTag::Delete => {
                        out.push_str("- ");
                        out.push_str(&line);
                        out.push('\n');
                    }
                    ChangeTag::Insert => {
                        out.push_str("+ ");
                        out.push_str(&line);
                        out.push('\n');
                    }
                    ChangeTag::Equal => {
                        out.push_str("  ");
                        out.push_str(&line);
                        out.push('\n');
                    }
                }
            }
        }
    }
    out.push_str("```\n");
    out
}

/// Diff per file content sets.
///
/// Inputs are maps keyed by file path (relative or absolute – caller decides)
/// with values being the raw file content EXACTLY as you wish it to be diffed
/// (e.g. already stripped of volatile metadata, no size/modified lines, only
/// the real file body). This keeps higher level logic (parsing the markdown
/// document) out of the diff layer.
///
/// Returns a vector of `PerFileDiff` for every file that is Added, Removed,
/// or Modified. Unchanged files are omitted by default (`skip_unchanged=true`)
/// to reduce noise, but you can opt to include them.
pub fn diff_file_contents(
    previous: &HashMap<String, String>,
    current: &HashMap<String, String>,
    skip_unchanged: bool,
    explicit_context: Option<usize>,
) -> Vec<PerFileDiff> {
    let mut all_paths: Vec<String> = previous.keys().chain(current.keys()).cloned().collect();
    all_paths.sort();
    all_paths.dedup();

    let context_lines = resolve_context_lines(explicit_context);
    let mut results = Vec::new();

    for path in all_paths {
        let old_opt = previous.get(&path);
        let new_opt = current.get(&path);
        match (old_opt, new_opt) {
            (None, Some(new_content)) => {
                // Added file: present only in current snapshot
                let mut diff = String::new();
                diff.push_str("```diff\n");
                for line in new_content.lines() {
                    diff.push_str("+ ");
                    diff.push_str(line);
                    diff.push('\n');
                }
                diff.push_str("```\n");
                results.push(PerFileDiff {
                    path,
                    status: PerFileStatus::Added,
                    diff,
                });
            }
            (Some(_old_content), None) => {
                // Removed file
                let old_content = previous.get(&path).unwrap();
                let mut diff = String::new();
                diff.push_str("```diff\n");
                for line in old_content.lines() {
                    diff.push_str("- ");
                    diff.push_str(line);
                    diff.push('\n');
                }
                diff.push_str("```\n");
                results.push(PerFileDiff {
                    path,
                    status: PerFileStatus::Removed,
                    diff,
                });
            }
            (Some(old_content), Some(new_content)) => {
                if old_content == new_content {
                    if !skip_unchanged {
                        results.push(PerFileDiff {
                            path,
                            status: PerFileStatus::Unchanged,
                            diff: String::new(),
                        });
                    }
                } else {
                    let diff = unified_no_header(old_content, new_content, context_lines);
                    results.push(PerFileDiff {
                        path,
                        status: PerFileStatus::Modified,
                        diff,
                    });
                }
            }
            (None, None) => unreachable!(),
        }
    }

    results
}

/// Render a collection of per file diffs into markdown WITHOUT a global
/// "## File Differences" header. Each file begins with a "### Diff: `<path>`"
/// heading so that it can be appended near the changed files summary.
pub fn render_per_file_diffs(diffs: &[PerFileDiff]) -> String {
    let mut out = String::new();
    for d in diffs {
        out.push_str(&format!("### Diff: `{}`\n\n", d.path));
        match d.status {
            PerFileStatus::Added => out.push_str("_Status: Added_\n\n"),
            PerFileStatus::Removed => out.push_str("_Status: Removed_\n\n"),
            PerFileStatus::Modified => out.push_str("_Status: Modified_\n\n"),
            PerFileStatus::Unchanged => {
                out.push_str("_Status: Unchanged_\n\n");
            }
        }
        if !d.diff.is_empty() {
            out.push_str(&d.diff);
            if !d.diff.ends_with('\n') {
                out.push('\n');
            }
        }
        out.push('\n');
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn map(pairs: &[(&str, &str)]) -> HashMap<String, String> {
        pairs
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect()
    }

    #[test]
    fn unchanged_is_skipped() {
        let prev = map(&[("a.txt", "one\n")]);
        let curr = map(&[("a.txt", "one\n")]);
        let diffs = diff_file_contents(&prev, &curr, true, Some(2));
        assert!(diffs.is_empty());
    }

    #[test]
    fn added_file_diff() {
        let prev = map(&[]);
        let curr = map(&[("new.rs", "fn main() {}\n")]);
        let diffs = diff_file_contents(&prev, &curr, true, Some(2));
        assert_eq!(diffs.len(), 1);
        let d = &diffs[0];
        assert_eq!(d.status, PerFileStatus::Added);
        assert!(d.diff.contains("+ fn main() {}"));
    }

    #[test]
    fn removed_file_diff() {
        let prev = map(&[("old.rs", "fn old() {}\n")]);
        let curr = map(&[]);
        let diffs = diff_file_contents(&prev, &curr, true, None);
        assert_eq!(diffs.len(), 1);
        let d = &diffs[0];
        assert_eq!(d.status, PerFileStatus::Removed);
        assert!(d.diff.contains("- fn old() {}"));
    }

    #[test]
    fn modified_file_diff() {
        let prev = map(&[("lib.rs", "fn add(a:i32,b:i32)->i32{a+b}\n")]);
        let curr = map(&[("lib.rs", "fn add(a: i32, b: i32) -> i32 { a + b }\n")]);
        let diffs = diff_file_contents(&prev, &curr, true, Some(1));
        assert_eq!(diffs.len(), 1);
        let d = &diffs[0];
        assert_eq!(d.status, PerFileStatus::Modified);
        assert!(d.diff.contains("- fn add(a:i32,b:i32)->i32{a+b}"));
        assert!(d.diff.contains("+ fn add(a: i32, b: i32) -> i32 { a + b }"));
    }

    #[test]
    fn include_unchanged_when_requested() {
        let prev = map(&[("a.txt", "same\n")]);
        let curr = map(&[("a.txt", "same\n")]);
        let diffs = diff_file_contents(&prev, &curr, false, None);
        assert_eq!(diffs.len(), 1);
        assert_eq!(diffs[0].status, PerFileStatus::Unchanged);
    }

    #[test]
    fn render_output_basic() {
        let prev = map(&[("a.txt", "one\n"), ("b.txt", "line1\nline2\n")]);
        let curr = map(&[
            ("a.txt", "two\n"),
            ("b.txt", "line1\nline2\n"),
            ("c.txt", "new file\n"),
        ]);
        let diffs = diff_file_contents(&prev, &curr, true, Some(1));
        let out = render_per_file_diffs(&diffs);
        assert!(out.contains("### Diff: `a.txt`"));
        assert!(out.contains("_Status: Modified_"));
        assert!(out.contains("+ two"));
        assert!(out.contains("### Diff: `c.txt`"));
        assert!(out.contains("_Status: Added_"));
        assert!(out.contains("+ new file"));
    }

    #[test]
    fn test_empty_files() {
        let prev = map(&[("empty.txt", "")]);
        let curr = map(&[("empty.txt", "")]);
        let diffs = diff_file_contents(&prev, &curr, true, None);
        assert!(diffs.is_empty());
    }

    #[test]
    fn test_empty_to_content() {
        let prev = map(&[("file.txt", "")]);
        let curr = map(&[("file.txt", "new content\n")]);
        let diffs = diff_file_contents(&prev, &curr, true, None);
        assert_eq!(diffs.len(), 1);
        assert_eq!(diffs[0].status, PerFileStatus::Modified);
        assert!(diffs[0].diff.contains("+ new content"));
    }

    #[test]
    fn test_content_to_empty() {
        let prev = map(&[("file.txt", "old content\n")]);
        let curr = map(&[("file.txt", "")]);
        let diffs = diff_file_contents(&prev, &curr, true, None);
        assert_eq!(diffs.len(), 1);
        assert_eq!(diffs[0].status, PerFileStatus::Modified);
        assert!(diffs[0].diff.contains("- old content"));
    }

    #[test]
    fn test_multiline_modifications() {
        let prev = map(&[("file.txt", "line1\nline2\nline3\nline4\n")]);
        let curr = map(&[("file.txt", "line1\nmodified2\nline3\nline4\n")]);
        let diffs = diff_file_contents(&prev, &curr, true, Some(2));
        assert_eq!(diffs.len(), 1);
        assert_eq!(diffs[0].status, PerFileStatus::Modified);
        assert!(diffs[0].diff.contains("- line2"));
        assert!(diffs[0].diff.contains("+ modified2"));
    }

    #[test]
    fn test_windows_line_endings() {
        let prev = map(&[("file.txt", "line1\r\nline2\r\n")]);
        let curr = map(&[("file.txt", "line1\r\nmodified2\r\n")]);
        let diffs = diff_file_contents(&prev, &curr, true, None);
        assert_eq!(diffs.len(), 1);
        assert_eq!(diffs[0].status, PerFileStatus::Modified);
        assert!(diffs[0].diff.contains("- line2"));
        assert!(diffs[0].diff.contains("+ modified2"));
    }

    #[test]
    fn test_per_file_diff_is_changed() {
        let added = PerFileDiff {
            path: "test.txt".to_string(),
            status: PerFileStatus::Added,
            diff: "test".to_string(),
        };
        assert!(added.is_changed());

        let removed = PerFileDiff {
            path: "test.txt".to_string(),
            status: PerFileStatus::Removed,
            diff: "test".to_string(),
        };
        assert!(removed.is_changed());

        let modified = PerFileDiff {
            path: "test.txt".to_string(),
            status: PerFileStatus::Modified,
            diff: "test".to_string(),
        };
        assert!(modified.is_changed());

        let unchanged = PerFileDiff {
            path: "test.txt".to_string(),
            status: PerFileStatus::Unchanged,
            diff: String::new(),
        };
        assert!(!unchanged.is_changed());
    }

    #[test]
    fn test_generate_diff_identical_content() {
        let content = "line1\nline2\nline3\n";
        let diff = generate_diff(content, content);
        assert!(diff.is_empty());
    }

    #[test]
    fn test_generate_diff_with_changes() {
        let old = "line1\nline2\nline3\n";
        let new = "line1\nmodified2\nline3\n";
        let diff = generate_diff(old, new);
        assert!(diff.contains("## File Differences"));
        assert!(diff.contains("```diff"));
        assert!(diff.contains("- line2"));
        assert!(diff.contains("+ modified2"));
    }

    #[test]
    fn test_resolve_context_lines_default() {
        let context = resolve_context_lines(None);
        assert_eq!(context, 3);
    }

    #[test]
    fn test_resolve_context_lines_explicit() {
        let context = resolve_context_lines(Some(5));
        assert_eq!(context, 5);
    }

    #[test]
    fn test_resolve_context_lines_zero_fallback() {
        let context = resolve_context_lines(Some(0));
        assert_eq!(context, 3); // Should fallback to default
    }

    #[test]
    fn test_unicode_content_diff() {
        let prev = map(&[("unicode.txt", "Hello 世界\n")]);
        let curr = map(&[("unicode.txt", "Hello 世界! 🌍\n")]);
        let diffs = diff_file_contents(&prev, &curr, true, None);
        assert_eq!(diffs.len(), 1);
        assert_eq!(diffs[0].status, PerFileStatus::Modified);
        assert!(diffs[0].diff.contains("Hello 世界"));
        assert!(diffs[0].diff.contains("🌍"));
    }

    #[test]
    fn test_render_per_file_diffs_empty() {
        let diffs = vec![];
        let output = render_per_file_diffs(&diffs);
        assert!(output.is_empty());
    }

    #[test]
    fn test_render_per_file_diffs_unchanged() {
        let diffs = vec![PerFileDiff {
            path: "unchanged.txt".to_string(),
            status: PerFileStatus::Unchanged,
            diff: String::new(),
        }];
        let output = render_per_file_diffs(&diffs);
        assert!(output.contains("### Diff: `unchanged.txt`"));
        assert!(output.contains("_Status: Unchanged_"));
    }

    #[test]
    fn test_render_per_file_diffs_without_trailing_newline() {
        let diffs = vec![PerFileDiff {
            path: "test.txt".to_string(),
            status: PerFileStatus::Modified,
            diff: "```diff\n+ line\n```".to_string(), // No trailing newline
        }];
        let output = render_per_file_diffs(&diffs);
        assert!(output.contains("### Diff: `test.txt`"));
        assert!(output.contains("_Status: Modified_"));
        assert!(output.ends_with("\n\n")); // Should add newlines
    }

    #[test]
    fn test_generate_diff_with_multiple_groups() {
        // Create content that will result in multiple diff groups to trigger "..." separator
        let old_content = "line1\nline2\nline3\nline4\nline5\nline6\nline7\nline8\nline9\nline10";
        let new_content = "line1_modified\nline2\nline3\nline4\nline5\nline6\nline7\nline8\nline9_modified\nline10";

        let diff = generate_diff(old_content, new_content);
        assert!(diff.contains("```diff"));
        assert!(diff.contains("## File Differences"));
        // With sufficient distance between changes and small context, should create groups with "..." separator
        println!("Generated diff: {}", diff);
    }

    #[test]
    fn test_diff_with_windows_line_endings() {
        let old_content = "line1\r\nline2\r\n";
        let new_content = "line1_modified\r\nline2\r\n";

        let diff = generate_diff(old_content, new_content);
        assert!(diff.contains("```diff"));
        assert!(diff.contains("line1_modified"));
        assert!(!diff.is_empty());
    }

    #[test]
    fn test_unified_no_header_with_multiple_groups() {
        // Create content that will result in multiple diff groups
        let old_content = "start\n\n\n\n\n\n\n\n\n\nmiddle\n\n\n\n\n\n\n\n\n\nend";
        let new_content =
            "start_modified\n\n\n\n\n\n\n\n\n\nmiddle\n\n\n\n\n\n\n\n\n\nend_modified";

        let diff = unified_no_header(old_content, new_content, 2);
        assert!(diff.contains("```diff"));
        // Should contain "..." separator between groups when changes are far apart
        println!("Unified diff: {}", diff);
    }

    #[test]
    fn test_unified_no_header_with_windows_line_endings() {
        let old_content = "line1\r\nline2\r\n";
        let new_content = "line1_modified\r\nline2\r\n";

        let diff = unified_no_header(old_content, new_content, 3);
        assert!(diff.contains("```diff"));
        assert!(diff.contains("line1_modified"));
        assert!(!diff.is_empty());
    }
}
