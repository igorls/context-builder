fn main() {
 use context_builder::diff::generate_diff;
 let old_content = r#
Some
text
here.#;
 let new_content = r#
Some
different
text
here.#;
 let diff = generate_diff(old_content, new_content);
 println!(
Diff
output:);
 println!(

, diff);
 let identical_content = r#
Same
content#;
 let diff_identical = generate_diff(identical_content, identical_content);
 println!(
\nDiff
for
identical
content:);
 println!(
Empty:

, diff_identical.is_empty());
 println!(
Content:
\{}\', diff_identical);
