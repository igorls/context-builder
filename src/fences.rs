//! CommonMark backtick fences and inline code spans.
//!
//! File bodies are wrapped in a fence strictly longer than any backtick run
//! inside them (minimum 3). A README or doc-comment that contains ` ``` `
//! therefore cannot close the outer block early. Paths shown as inline code
//! use a delimiter longer than any backtick run in the path, with the space
//! padding CommonMark requires so the span round-trips.

/// Longest consecutive run of U+0060 GRAVE ACCENT in `text`.
pub(crate) fn longest_backtick_run(text: &str) -> usize {
    let mut max = 0usize;
    let mut cur = 0usize;
    for b in text.bytes() {
        if b == b'`' {
            cur += 1;
            if cur > max {
                max = cur;
            }
        } else {
            cur = 0;
        }
    }
    max
}

/// Backtick fence marker: at least 3, and one longer than any run in `content`.
pub(crate) fn backtick_fence(content: &str) -> String {
    let len = longest_backtick_run(content).saturating_add(1).max(3);
    "`".repeat(len)
}

/// Render `text` as a CommonMark inline code span.
///
/// The delimiter is one backtick longer than the longest run in `text`.
/// When `text` begins or ends with a backtick or a space, the span is padded
/// with spaces: a leading or trailing backtick would otherwise be absorbed
/// into the delimiter, and a span that both begins and ends with a space has
/// one space stripped from each end on parse. An empty span or a span of only
/// spaces is not padded — the strip rule does not apply to an all-space span,
/// so extra padding would be kept.
pub(crate) fn inline_code(text: &str) -> String {
    let delim = "`".repeat(longest_backtick_run(text) + 1);
    if needs_inline_code_padding(text) {
        format!("{delim} {text} {delim}")
    } else {
        format!("{delim}{text}{delim}")
    }
}

fn needs_inline_code_padding(text: &str) -> bool {
    if text.is_empty() || text.bytes().all(|b| b == b' ') {
        return false;
    }
    let bytes = text.as_bytes();
    let first = bytes[0];
    let last = bytes[bytes.len() - 1];
    first == b'`' || last == b'`' || first == b' ' || last == b' '
}

/// Wrap `body` in a backtick fence tagged with `info` (for example `rust` or `diff`).
///
/// `info` must not contain backticks. An empty body produces an empty block
/// with no extra blank line. A non-empty body that does not end in a newline
/// gets one, so the closer is always on its own line.
pub(crate) fn fenced_block(info: &str, body: &str) -> String {
    debug_assert!(
        !info.contains('`'),
        "CommonMark backtick fences cannot have backticks in the info string"
    );
    let fence = backtick_fence(body);
    let mut out = String::with_capacity(fence.len() * 2 + info.len() + body.len() + 3);
    out.push_str(&fence);
    out.push_str(info);
    out.push('\n');
    out.push_str(body);
    if !body.is_empty() && !body.ends_with('\n') {
        out.push('\n');
    }
    out.push_str(&fence);
    out.push('\n');
    out
}

/// If `doc` ends inside a backtick code fence, append a closer of the same length.
///
/// Tracks CommonMark backtick fences (0–3 spaces of indentation, info string
/// on the opener, closer at least as long as the opener and otherwise blank).
/// A shorter run inside an open fence — the usual ` ``` ` line in a README —
/// does not toggle state, so truncation can still close the real wrapper.
pub(crate) fn close_unmatched_backtick_fence(doc: &mut String) {
    if let Some(len) = unmatched_backtick_fence_len(doc) {
        if !doc.ends_with('\n') {
            doc.push('\n');
        }
        doc.push_str(&"`".repeat(len));
        doc.push('\n');
    }
}

/// Length of the backtick fence still open at the end of `doc`, if any.
pub(crate) fn unmatched_backtick_fence_len(doc: &str) -> Option<usize> {
    let mut open: Option<usize> = None;
    for line in doc.lines() {
        if let Some(open_len) = open {
            if is_closing_backtick_fence(line, open_len) {
                open = None;
            }
        } else if let Some(len) = opening_backtick_fence_len(line) {
            open = Some(len);
        }
    }
    open
}

fn opening_backtick_fence_len(line: &str) -> Option<usize> {
    let indent = line.bytes().take_while(|b| *b == b' ').count();
    if indent > 3 {
        return None;
    }
    let rest = &line[indent..];
    let ticks = rest.bytes().take_while(|b| *b == b'`').count();
    if ticks < 3 {
        return None;
    }
    let info = &rest[ticks..];
    if info.contains('`') {
        return None;
    }
    Some(ticks)
}

fn is_closing_backtick_fence(line: &str, open_len: usize) -> bool {
    let indent = line.bytes().take_while(|b| *b == b' ').count();
    if indent > 3 {
        return false;
    }
    let rest = &line[indent..];
    let ticks = rest.bytes().take_while(|b| *b == b'`').count();
    if ticks < open_len {
        return false;
    }
    rest[ticks..].bytes().all(|b| b == b' ' || b == b'\t')
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Parse one inline code span the way CommonMark does (no line endings).
    fn parse_code_span(span: &str) -> String {
        let ticks = span.bytes().take_while(|b| *b == b'`').count();
        assert!(ticks >= 1, "missing opening delimiter in {span:?}");
        let closing = &span[span.len() - ticks..];
        assert_eq!(closing, "`".repeat(ticks), "closer mismatch in {span:?}");
        assert!(
            !span[ticks..span.len() - ticks].contains(&"`".repeat(ticks)),
            "delimiter occurs inside span {span:?}"
        );
        let inner = &span[ticks..span.len() - ticks];
        if inner.len() >= 2
            && inner.starts_with(' ')
            && inner.ends_with(' ')
            && !inner.bytes().all(|b| b == b' ')
        {
            inner[1..inner.len() - 1].to_string()
        } else {
            inner.to_string()
        }
    }

    #[test]
    fn fence_is_at_least_three_and_longer_than_any_run() {
        assert_eq!(backtick_fence(""), "```");
        assert_eq!(backtick_fence("no ticks"), "```");
        assert_eq!(backtick_fence("let x = `a`;"), "```");
        assert_eq!(backtick_fence("``"), "```");
        assert_eq!(backtick_fence("```"), "````");
        assert_eq!(backtick_fence("before\n````\nafter"), "`````");
        assert_eq!(longest_backtick_run("`` ` ``` ````"), 4);
    }

    #[test]
    fn inline_code_round_trips_commonmark() {
        let samples = [
            "src/lib.rs",
            "weird`name.py",
            "`lead",
            "trail`",
            "`both`",
            "a``b",
            " foo",
            "foo ",
            " foo ",
            "  ",
            "`",
            "```",
        ];
        for sample in samples {
            let rendered = inline_code(sample);
            assert_eq!(parse_code_span(&rendered), sample, "span {rendered:?}");
        }
        // Ordinary paths keep a single-backtick span so existing headings stay stable.
        assert_eq!(inline_code("src/lib.rs"), "`src/lib.rs`");
        assert_eq!(inline_code("weird`name.py"), "``weird`name.py``");
        assert_eq!(inline_code("`lead"), "`` `lead ``");
    }

    #[test]
    fn fenced_block_preserves_body_and_grows() {
        let body = "before\n```\nafter\n";
        let block = fenced_block("markdown", body);
        assert_eq!(block, "````markdown\nbefore\n```\nafter\n````\n");
        assert!(unmatched_backtick_fence_len(&block).is_none());

        let empty = fenced_block("diff", "");
        assert_eq!(empty, "```diff\n```\n");
    }

    #[test]
    fn closer_uses_the_open_fence_length() {
        let mut open = String::from("````markdown\nsee\n```\nstill inside");
        close_unmatched_backtick_fence(&mut open);
        assert!(open.ends_with("\n````\n"));
        assert!(unmatched_backtick_fence_len(&open).is_none());

        let mut balanced = String::from("```rust\nfn main() {}\n```\n");
        let before = balanced.clone();
        close_unmatched_backtick_fence(&mut balanced);
        assert_eq!(balanced, before);

        let nested = "````md\n```\ncode\n```\n````\n";
        assert!(unmatched_backtick_fence_len(nested).is_none());

        let mut mid = String::from("```rust\nfn mai");
        close_unmatched_backtick_fence(&mut mid);
        assert_eq!(mid, "```rust\nfn mai\n```\n");
    }

    #[test]
    fn indented_shorter_run_does_not_close_a_longer_fence() {
        // Two spaces of diff context plus ``` is a legal closer for a fence of 3.
        let closed_by_context = "```diff\n  ```\n";
        assert!(unmatched_backtick_fence_len(closed_by_context).is_none());
        // The same line does not close a fence of 4, so the real closer still matches.
        let held_open = "````diff\n  ```\n";
        assert_eq!(unmatched_backtick_fence_len(held_open), Some(4));
        let closed = "````diff\n  ```\n````\n";
        assert!(unmatched_backtick_fence_len(closed).is_none());
    }
}
