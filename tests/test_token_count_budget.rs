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
