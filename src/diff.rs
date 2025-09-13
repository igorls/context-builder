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
}
