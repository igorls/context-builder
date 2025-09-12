[package]
name = "context-builder"
version = "0.4.0"
default-run = "context-builder"
edition = "2024"
authors = ["Igor Lins e Silva"]
description = "CLI tool to aggregate directory contents into a single markdown file optimized for LLM consumption"
readme = "README.md"
homepage = "https://github.com/igorls/context-builder"
repository = "https://github.com/igorls/context-builder"
license = "MIT"
keywords = ["cli", "markdown", "documentation", "llm", "context"]
categories = ["command-line-utilities", "development-tools"]

[dependencies]
clap = { version = "4.5.47", features = ["derive"] }
chrono = { version = "0.4.42", features = ["serde"] }
ignore = "0.4.23"
log = "0.4.28"
env_logger = "0.11.8"
rayon = { version = "1.10", optional = true }
serde = { version = "1.0.219", features = ["derive"] }
toml = "0.8.23"
dissimilar = "1.0.10"
tempfile = "3.22.0"
tiktoken-rs = "0.7.0"
once_cell = "1.19.0"

[features]
default = ["parallel"]
parallel = ["rayon"]
samples-bin = []

[dev-dependencies]
tempfile = "3.22.0"
criterion = { version = "0.5", features = ["html_reports"] }

[[bench]]
name = "context_bench"
harness = false

[[bin]]
name = "generate_samples"
path = "scripts/generate_samples.rs"
required-features = ["samples-bin"]
```
"#;

    let new_content = old_content.clone(); // Identical content

    let diff_result = generate_diff(old_content, new_content);

    println!("Diff result length: {}", diff_result.len());
    println!("Diff result is empty: {}", diff_result.is_empty());

    if diff_result.is_empty() {
        println!("SUCCESS: No diff generated for identical files");
    } else {
        println!("ISSUE: Diff was generated for identical files");
        println!("Diff content:\n{}", diff_result);
    }
}
