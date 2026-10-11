//! `--token-count` honors `--max-tokens` (#21 B21).

use std::fs;
use std::process::Command;

fn count(dir: &std::path::Path, extra: &[&str]) -> String {
    let out = Command::new(env!("CARGO_BIN_EXE_context-builder"))
        .args(["-d", dir.to_str().unwrap(), "--token-count"])
        .args(extra)
        .current_dir(dir)
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8_lossy(&out.stdout).into_owned()
}

fn total(text: &str) -> usize {
    text.lines()
        .find_map(|l| l.strip_prefix("Estimated total tokens: "))
        .expect("total line")
        .trim()
        .parse()
        .unwrap()
}

#[test]
fn token_count_reports_the_capped_size_and_omitted_files() {
    let dir = tempfile::tempdir().unwrap();
    for i in 0..10 {
        let body = format!(
            "// file {i}\n{}",
            "let value = compute(alpha, beta);\n".repeat(200)
        );
        fs::write(dir.path().join(format!("f{i}.rs")), body).unwrap();
    }

    let full = count(dir.path(), &[]);
    assert!(total(&full) > 10_000, "{full}");
    assert!(!full.contains("Token budget"), "{full}");

    let capped = count(dir.path(), &["--max-tokens", "3000"]);
    assert!(total(&capped) <= 3000, "{capped}");
    assert!(
        total(&capped) > 2000,
        "budget should be nearly used: {capped}"
    );
    let omitted_line = capped
        .lines()
        .find(|l| l.starts_with("Token budget: 3000"))
        .unwrap_or_else(|| panic!("no budget line: {capped}"));
    assert!(
        !omitted_line.ends_with("(files omitted: 0)"),
        "{omitted_line}"
    );
}

/// The preview must agree with the real run about how many files the budget
/// leaves out, including when the crossing file cannot even fit its header.
#[test]
fn preview_omitted_count_matches_the_real_run_at_every_small_budget() {
    let dir = tempfile::tempdir().unwrap();
    for i in 0..6 {
        fs::write(
            dir.path().join(format!("file_{i}.rs")),
            "let value = compute(alpha, beta);\n".repeat(30),
        )
        .unwrap();
    }
    for budget in (120..700).step_by(45) {
        let b = budget.to_string();
        let preview = count(dir.path(), &["--max-tokens", &b]);
        assert!(total(&preview) <= budget, "{budget}: {preview}");
        let preview_omitted: usize = preview
            .lines()
            .find_map(|l| l.split("files omitted: ").nth(1))
            .and_then(|s| s.trim_end_matches(')').parse().ok())
            .expect("omitted");

        let real = Command::new(env!("CARGO_BIN_EXE_context-builder"))
            .args(["-d", dir.path().to_str().unwrap(), "-o", "-", "-y"])
            .args(["--max-tokens", &b])
            .current_dir(dir.path())
            .output()
            .unwrap();
        let doc = String::from_utf8_lossy(&real.stdout);
        let real_omitted: usize = doc
            .split("reached. ")
            .nth(1)
            .and_then(|s| s.split(' ').next())
            .and_then(|n| n.parse().ok())
            .unwrap_or(0);
        assert_eq!(
            preview_omitted, real_omitted,
            "budget {budget}:\n{preview}\n{doc}"
        );
    }
}
