use std::fs;
use std::path::Path;

use clap::Parser;
use tempfile::tempdir;

use context_builder::config::Config;
use context_builder::{Prompter, cli::Args, run_with_args};

struct TestPrompter {
    overwrite_response: bool,
}

impl TestPrompter {
    fn new(overwrite_response: bool) -> Self {
        Self { overwrite_response }
    }
}

impl Prompter for TestPrompter {
    fn confirm_overwrite(&self, _file_path: &str) -> std::io::Result<bool> {
        Ok(self.overwrite_response)
    }
}

fn write_file(path: &Path, contents: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(path, contents).unwrap();
}

#[test]
fn preview_mode_does_not_create_output_file() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    // Create a small project structure
    write_file(&root.join("src/main.rs"), "fn main() { println!(\"hi\"); }");
    write_file(&root.join("README.md"), "# Readme");

    let args = Args {
        input: root.to_string_lossy().into_owned(),
        output: root.join("output.md").to_string_lossy().into_owned(),
        filter: vec![],
        ignore: vec![],
        preview: true,
        token_count: false,
        line_numbers: false,
        yes: false,
        diff_only: false,
        clear_cache: false,
        encoding: "o200k_base".to_string(),
        init: false,
        max_tokens: None,
        signatures: false,
        structure: false,
        truncate: "smart".to_string(),
        visibility: "all".to_string(),
    };

    let prompter = TestPrompter::new(true);

    // Run in preview mode
    let res = run_with_args(args, Config::default(), &prompter);
    assert!(res.is_ok(), "preview mode should succeed");

    // No output file created
    assert!(
        !root.join("output.md").exists(),
        "output file should not be created in preview mode"
    );
}

#[test]
fn preview_mode_skips_overwrite_confirmation() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    // Create an existing output file
    let output_path = root.join("output.md");
    write_file(&output_path, "existing content");

    // Create a small project structure
    write_file(&root.join("src/main.rs"), "fn main() { println!(\"hi\"); }");
    write_file(&root.join("README.md"), "# Readme");

    let args = Args {
        input: root.to_string_lossy().into_owned(),
        output: output_path.to_string_lossy().into_owned(),
        filter: vec![],
        ignore: vec![],
        preview: true,
        token_count: false,
        line_numbers: false,
        yes: false,
        diff_only: false,
        clear_cache: false,
        encoding: "o200k_base".to_string(),
        init: false,
        max_tokens: None,
        signatures: false,
        structure: false,
        truncate: "smart".to_string(),
        visibility: "all".to_string(),
    };

    // Use false for overwrite response to verify it's not called
    let prompter = TestPrompter::new(false);

    // Run in preview mode - should succeed even with overwrite denied
    let res = run_with_args(args, Config::default(), &prompter);
    assert!(
        res.is_ok(),
        "preview mode should succeed without overwrite confirmation"
    );

    // Output file should remain unchanged
    let content = fs::read_to_string(&output_path).unwrap();
    assert_eq!(
        content, "existing content",
        "output file should not be modified in preview mode"
    );
}

#[test]
fn token_count_mode_skips_overwrite_confirmation() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    // Create an existing output file
    let output_path = root.join("output.md");
    write_file(&output_path, "existing content");

    // Create a small project structure
    write_file(&root.join("src/main.rs"), "fn main() { println!(\"hi\"); }");
    write_file(&root.join("README.md"), "# Readme");

    let args = Args {
        input: root.to_string_lossy().into_owned(),
        output: output_path.to_string_lossy().into_owned(),
        filter: vec![],
        ignore: vec![],
        preview: false,
        token_count: true,
        line_numbers: false,
        yes: false,
        diff_only: false,
        clear_cache: false,
        encoding: "o200k_base".to_string(),
        init: false,
        max_tokens: None,
        signatures: false,
        structure: false,
        truncate: "smart".to_string(),
        visibility: "all".to_string(),
    };

    // Use false for overwrite response to verify it's not called
    let prompter = TestPrompter::new(false);

    // Run in token count mode - should succeed even with overwrite denied
    let res = run_with_args(args, Config::default(), &prompter);
    assert!(
        res.is_ok(),
        "token count mode should succeed without overwrite confirmation"
    );

    // Output file should remain unchanged
    let content = fs::read_to_string(&output_path).unwrap();
    assert_eq!(
        content, "existing content",
        "output file should not be modified in token count mode"
    );
}

#[test]

fn both_preview_and_token_count_modes_work_together() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    // Create a small project structure
    write_file(&root.join("src/main.rs"), "fn main() { println!(\"hi\"); }");
    write_file(&root.join("README.md"), "# Readme");

    let args = Args {
        input: root.to_string_lossy().into_owned(),
        output: root.join("output.md").to_string_lossy().into_owned(),
        filter: vec![],
        ignore: vec![],
        preview: true,
        token_count: true,
        line_numbers: false,
        yes: false,
        diff_only: false,
        clear_cache: false,
        encoding: "o200k_base".to_string(),
        init: false,
        max_tokens: None,
        signatures: false,
        structure: false,
        truncate: "smart".to_string(),
        visibility: "all".to_string(),
    };

    let prompter = TestPrompter::new(false); // false for overwrite since it should be skipped

    // Run with both modes
    let res = run_with_args(args, Config::default(), &prompter);
    assert!(res.is_ok(), "both modes should work together");

    // No output file created
    assert!(
        !root.join("output.md").exists(),
        "output file should not be created when both modes are active"
    );
}

#[test]
fn end_to_end_generates_output_with_filters_ignores_and_line_numbers() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    // Files that should be included by filters
    write_file(
        &root.join("src/main.rs"),
        "fn main() {\n    println!(\"hi\");\n}\n",
    );
    write_file(&root.join("README.md"), "# Top-level readme\n\nSome text");

    // Ignored directories/files
    write_file(
        &root.join("node_modules/pkg/index.js"),
        "console.log('ignore');",
    );
    write_file(&root.join("target/artifact.txt"), "binary");

    // A large file to exercise streaming and performance
    let mut large = String::with_capacity(4000 * 25);
    for i in 0..4000 {
        large.push_str(&format!("// line {}\n", i + 1));
    }
    write_file(&root.join("src/large.rs"), &large);

    let output_path = root.join("ctx.md");

    let args = Args {
        input: root.to_string_lossy().into_owned(),
        output: output_path.to_string_lossy().into_owned(),
        filter: vec!["rs".into(), "md".into()],
        ignore: vec!["node_modules".into(), "target".into()],
        preview: false,
        token_count: false,
        line_numbers: true,
        yes: false,
        diff_only: false,
        clear_cache: false,
        encoding: "o200k_base".to_string(),
        init: false,
        max_tokens: None,
        signatures: false,
        structure: false,
        truncate: "smart".to_string(),
        visibility: "all".to_string(),
    };

    // Always proceed without interactive prompts
    let prompter = TestPrompter::new(true);

    let res = run_with_args(args, Config::default(), &prompter);
    assert!(res.is_ok(), "end-to-end generation should succeed");

    // Find the actual output file (may have timestamp appended)
    let actual_output_path = if output_path.exists() {
        output_path
    } else {
        // Look for timestamped version
        let parent = output_path.parent().unwrap();
        let stem = output_path.file_stem().unwrap().to_string_lossy();
        let ext = output_path.extension().unwrap().to_string_lossy();

        let mut found_file = None;
        if let Ok(entries) = fs::read_dir(parent) {
            for entry in entries.flatten() {
                let file_name = entry.file_name();
                let name = file_name.to_string_lossy();
                if name.starts_with(&format!("{}_", stem)) && name.ends_with(&format!(".{}", ext)) {
                    found_file = Some(entry.path());
                    break;
                }
            }
        }

        found_file.unwrap_or_else(|| {
            panic!(
                "No output file found. Expected {} or timestamped version",
                output_path.display()
            )
        })
    };

    // Basic content checks
    let out = fs::read_to_string(&actual_output_path).unwrap();

    // Has file tree section
    assert!(
        out.contains("## File Tree Structure"),
        "output should contain a 'File Tree Structure' section"
    );

    // Has at least one rust code block with line numbers (looking for ' | ' marker)
    assert!(
        out.contains("```rust"),
        "output should contain a rust code block"
    );
    assert!(
        out.contains("   1 | "),
        "output should contain line-numbered code blocks"
    );

    // Should not include ignored directory entries' content (not a strict check, but indicative)
    assert!(
        !out.contains("console.log('ignore');"),
        "output should not include content from ignored directories"
    );
}

#[test]
fn overwrite_prompt_is_respected() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    // Prepare an existing output file with sentinel content
    let output_path = root.join("out.md");
    write_file(&output_path, "SENTINEL");

    // Put a file to process
    write_file(&root.join("src/lib.rs"), "pub fn f() {}");

    let args = Args {
        input: root.to_string_lossy().into_owned(),
        output: output_path.to_string_lossy().into_owned(),
        filter: vec!["rs".into()],
        ignore: vec![],
        preview: false,
        token_count: false,
        line_numbers: false,
        yes: false,
        diff_only: false,
        clear_cache: false,
        encoding: "o200k_base".to_string(),
        init: false,
        max_tokens: None,
        signatures: false,
        structure: false,
        truncate: "smart".to_string(),
        visibility: "all".to_string(),
    };

    // Deny overwrite
    let prompter = TestPrompter::new(false);

    let res = run_with_args(args, Config::default(), &prompter);
    assert!(
        res.is_err(),
        "run should return error when overwrite denied"
    );

    // Ensure file is unchanged
    let out = fs::read_to_string(&output_path).unwrap();
    assert_eq!(out, "SENTINEL", "existing output should not be overwritten");
}

#[test]
fn many_files_proceed_without_a_processing_prompt() {
    // The >100-file confirmation was removed. A large tree must succeed
    // without `--yes`.
    let dir = tempdir().unwrap();
    let root = dir.path();

    fs::create_dir_all(root.join("data")).unwrap();
    for i in 0..150 {
        write_file(&root.join("data").join(format!("f{}.txt", i)), "x");
    }

    let output = root.join("out.md");
    let args = Args {
        input: root.to_string_lossy().into_owned(),
        output: output.to_string_lossy().into_owned(),
        filter: vec!["txt".into()],
        ignore: vec![],
        preview: false,
        token_count: false,
        line_numbers: false,
        yes: false,
        diff_only: false,
        clear_cache: false,
        encoding: "o200k_base".to_string(),
        init: false,
        max_tokens: None,
        signatures: false,
        structure: false,
        truncate: "smart".to_string(),
        visibility: "all".to_string(),
    };

    let prompter = TestPrompter::new(true);

    let res = run_with_args(args, Config::default(), &prompter);
    assert!(res.is_ok(), "run should succeed with many files");
    assert!(output.exists(), "output should be written");
}

#[test]
fn pipe_mode_many_files_does_not_block() {
    // `-o -` with >100 files and without `--yes` must succeed. There is no
    // processing prompt to answer, so the pipe is not blocked on stdin.
    let dir = tempdir().unwrap();
    let root = dir.path();

    fs::create_dir_all(root.join("data")).unwrap();
    for i in 0..150 {
        write_file(&root.join("data").join(format!("f{}.txt", i)), "x");
    }

    let args = Args {
        input: root.to_string_lossy().into_owned(),
        output: "-".to_string(),
        filter: vec!["txt".into()],
        ignore: vec![],
        preview: false,
        token_count: false,
        line_numbers: false,
        yes: false,
        diff_only: false,
        clear_cache: false,
        encoding: "o200k_base".to_string(),
        init: false,
        max_tokens: None,
        signatures: false,
        structure: false,
        truncate: "smart".to_string(),
        visibility: "all".to_string(),
    };

    let prompter = TestPrompter::new(true);

    let res = run_with_args(args, Config::default(), &prompter);
    assert!(
        res.is_ok(),
        "pipe mode must proceed without a file-count confirmation"
    );
}

#[test]
fn token_count_mode_does_not_create_output_file() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    // Create a small project structure
    write_file(&root.join("src/main.rs"), "fn main() { println!(\"hi\"); }");
    write_file(&root.join("README.md"), "# Readme");

    let args = Args {
        input: root.to_string_lossy().into_owned(),
        output: root.join("output.md").to_string_lossy().into_owned(),
        filter: vec![],
        ignore: vec![],
        preview: false,
        token_count: true,
        line_numbers: false,
        yes: false,
        diff_only: false,
        clear_cache: false,
        encoding: "o200k_base".to_string(),
        init: false,
        max_tokens: None,
        signatures: false,
        structure: false,
        truncate: "smart".to_string(),
        visibility: "all".to_string(),
    };

    let prompter = TestPrompter::new(true);

    // Run in token count mode
    let res = run_with_args(args, Config::default(), &prompter);
    assert!(res.is_ok(), "token count mode should succeed");

    // No output file created
    assert!(
        !root.join("output.md").exists(),
        "output file should not be created in token count mode"
    );
}

/// Parse ignore flags the way the CLI does, then collect files with those patterns.
fn collected_with_ignore_args(root: &Path, ignore_args: &[&str]) -> Vec<String> {
    let mut argv = vec!["context-builder".to_string()];
    for arg in ignore_args {
        argv.push((*arg).to_string());
    }
    let args = Args::try_parse_from(&argv).unwrap_or_else(|err| panic!("parse failed: {err}"));
    let files = context_builder::file_utils::collect_files(root, &[], &args.ignore, &[]).unwrap();
    let mut paths: Vec<String> = files
        .iter()
        .map(|entry| {
            entry
                .path()
                .strip_prefix(root)
                .unwrap()
                .to_string_lossy()
                .replace('\\', "/")
        })
        .collect();
    paths.sort();
    paths
}

#[test]
fn comma_separated_ignore_excludes_both_directories() {
    let dir = tempdir().unwrap();
    let root = dir.path();
    write_file(&root.join("src/main.rs"), "fn main() {}");
    write_file(&root.join("docs/guide.md"), "# guide");
    write_file(&root.join("assets/logo.svg"), "<svg></svg>");
    write_file(&root.join("keep.txt"), "keep");

    let paths = collected_with_ignore_args(root, &["-i", "docs,assets"]);
    assert!(
        paths.iter().any(|path| path == "src/main.rs"),
        "source should remain, got {paths:?}"
    );
    assert!(
        paths.iter().any(|path| path == "keep.txt"),
        "unrelated files should remain, got {paths:?}"
    );
    assert!(
        !paths.iter().any(|path| path.starts_with("docs")),
        "docs/ should be ignored, got {paths:?}"
    );
    assert!(
        !paths.iter().any(|path| path.starts_with("assets")),
        "assets/ should be ignored, got {paths:?}"
    );
}

#[test]
fn repeated_ignore_flags_still_exclude_each_name() {
    let dir = tempdir().unwrap();
    let root = dir.path();
    write_file(&root.join("src/main.rs"), "fn main() {}");
    write_file(&root.join("docs/guide.md"), "# guide");
    write_file(&root.join("assets/logo.svg"), "<svg></svg>");
    write_file(&root.join("keep.txt"), "keep");

    let paths = collected_with_ignore_args(root, &["-i", "docs", "-i", "assets"]);
    assert!(
        paths.iter().any(|path| path == "src/main.rs"),
        "source should remain, got {paths:?}"
    );
    assert!(
        paths.iter().any(|path| path == "keep.txt"),
        "unrelated files should remain, got {paths:?}"
    );
    assert!(
        !paths.iter().any(|path| path.starts_with("docs")),
        "repeated -i docs should ignore docs/, got {paths:?}"
    );
    assert!(
        !paths.iter().any(|path| path.starts_with("assets")),
        "repeated -i assets should ignore assets/, got {paths:?}"
    );
}

#[test]
fn glob_ignore_excludes_matching_files() {
    let dir = tempdir().unwrap();
    let root = dir.path();
    write_file(&root.join("src/lib.rs"), "fn lib() {}");
    write_file(&root.join("Cargo.lock"), "# lock");
    write_file(&root.join("nested/pkg.lock"), "# nested lock");
    write_file(&root.join("keep.txt"), "keep");

    let paths = collected_with_ignore_args(root, &["-i", "*.lock"]);
    assert!(
        paths.iter().any(|path| path == "src/lib.rs"),
        "non-matching files should remain, got {paths:?}"
    );
    assert!(
        paths.iter().any(|path| path == "keep.txt"),
        "non-matching files should remain, got {paths:?}"
    );
    assert!(
        !paths.iter().any(|path| path.ends_with(".lock")),
        "gitignore glob *.lock should exclude lockfiles, got {paths:?}"
    );
}

#[test]
fn path_ignore_excludes_that_directory() {
    let dir = tempdir().unwrap();
    let root = dir.path();
    write_file(&root.join("src/main.rs"), "fn main() {}");
    write_file(&root.join("crates/core/lib.rs"), "fn core() {}");
    write_file(&root.join("crates/cli/main.rs"), "fn cli() {}");

    let paths = collected_with_ignore_args(root, &["-i", "crates/core"]);
    assert!(
        paths.iter().any(|path| path == "src/main.rs"),
        "unrelated source should remain, got {paths:?}"
    );
    assert!(
        paths.iter().any(|path| path == "crates/cli/main.rs"),
        "sibling paths should remain, got {paths:?}"
    );
    assert!(
        !paths.iter().any(|path| path.starts_with("crates/core")),
        "path pattern crates/core should be ignored, got {paths:?}"
    );
}
