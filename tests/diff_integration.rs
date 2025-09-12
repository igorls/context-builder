use context_builder::diff::generate_diff;

#[test]
fn test_diff_with_identical_content() {
    let content = r#"# Test Document

This is a test document with some content.

## Section 1

Some text here.

## Section 2

More text here.
"#;

    let diff = generate_diff(content, content);

    // When content is identical, diff should be empty
    assert!(diff.is_empty());
}

#[test]
fn test_diff_with_changes() {
    let old_content = r#"# Test Document

This is a test document with some content.

## Section 1

Some text here.

## Section 2

More text here.
"#;

    let new_content = r#"# Test Document

This is a test document with some content.

## Section 1

Some different text here.

## Section 2

More text here.
"#;

    let diff = generate_diff(old_content, new_content);

    // When content has differences, diff should not be empty
    assert!(!diff.is_empty());
    assert!(diff.contains("## File Differences"));

    // Print the diff for debugging
    println!("Actual diff output:\n{}", diff);

    assert!(diff.contains("- Some text here"));
    assert!(diff.contains("+ Some different text here"));
}
