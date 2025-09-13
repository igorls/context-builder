#![allow(
    clippy::needless_return,
    clippy::extra_unused_lifetimes,
    clippy::doc_overindented_list_items,
    dead_code
)]
//! Dataset generation script for creating synthetic sample directories to benchmark and test
//! the context-builder CLI locally. This is intended to generate a folder that should be ignored
//! by version control (e.g., add `/samples` to your project's .gitignore).
//!
//! Usage examples (Windows PowerShell):
//!   - rustc scripts/generate_samples.rs -O -o generate_samples.exe; .\generate_samples.exe
//!   - .\generate_samples.exe --help
//!
//! Flags:
//!   --out <DIR>             Output directory (default: ./samples)
//!   --presets <list>        Comma-separated presets to generate: tiny,small,medium (default: tiny,small)
//!   --include-large         Also generate the large preset (off by default)
//!   --only <name>           Only generate a single preset (overrides --presets)
//!   --clean                 Remove the output directory before generating
//!   --dry-run               Print the plan without writing files
//!
//! Advanced overrides (apply when using --only):
//!   --files <N>             Number of text files
//!   --binary-every <N>      Create one .bin file every N text files (0 disables)
//!   --depth <D>             Directory tree depth
//!   --width <W>             Subdirectories per level
//!   --size <BYTES>          Approx text file size in bytes
//!   --filters <CSV>         Extensions to include (default: rs,md,txt,toml)
//!   --ignores <CSV>         Directory/file names to ignore (default: target,node_modules)
//!
//! Generated structure per dataset (e.g., samples/small):
//!   - project/
//!       src/, docs/, assets/      -> nested trees with text files
//!       target/, node_modules/    -> ignored directories with noise
//!       README.md, Cargo.toml     -> top-level files
//!       (binary files are sprinkled across trees and should be ignored by the tool)
//!
//! Notes:
//! - Binary files are generated to validate that the CLI ignores them by default filters.
//! - This script uses only the Rust standard library.

use std::env;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

#[derive(Clone, Debug)]
struct DatasetSpec {
    name: String,
    text_files: usize,
    binary_every: usize,
    depth: usize,
    width: usize,
    text_file_size: usize,
    filters: Vec<String>,
    ignores: Vec<String>,
}

impl DatasetSpec {
    fn with_name(name: &str) -> Option<Self> {
        match name {
            "tiny" => Some(Self {
                name: "tiny".into(),
                text_files: 100,
                binary_every: 10,
                depth: 2,
                width: 3,
                text_file_size: 256,
                filters: default_filters(),
                ignores: default_ignores(),
            }),
            "small" => Some(Self {
                name: "small".into(),
                text_files: 1_000,
                binary_every: 20,
                depth: 3,
                width: 4,
                text_file_size: 512,
                filters: default_filters(),
                ignores: default_ignores(),
            }),
            "medium" => Some(Self {
                name: "medium".into(),
                text_files: 5_000,
                binary_every: 25,
                depth: 4,
                width: 4,
                text_file_size: 800,
                filters: default_filters(),
                ignores: default_ignores(),
            }),
            "large" => Some(Self {
                name: "large".into(),
                text_files: 20_000,
                binary_every: 50,
                depth: 5,
                width: 5,
                text_file_size: 1024,
                filters: default_filters(),
                ignores: default_ignores(),
            }),
            _ => None,
        }
    }
}

fn default_filters() -> Vec<String> {
    vec!["rs", "md", "txt", "toml"]
        .into_iter()
        .map(|s| s.to_string())
        .collect()
}

fn default_ignores() -> Vec<String> {
    vec!["target", "node_modules"]
        .into_iter()
        .map(|s| s.to_string())
        .collect()
}

#[derive(Default)]
struct Args {
    out: PathBuf,
    presets: Vec<String>,
    include_large: bool,
    only: Option<String>,
    clean: bool,
    dry_run: bool,
    // overrides for --only
    files: Option<usize>,
    binary_every: Option<usize>,
    depth: Option<usize>,
    width: Option<usize>,
    size: Option<usize>,
    filters: Option<Vec<String>>,
    ignores: Option<Vec<String>>,
}

fn parse_args() -> Args {
    let mut out = PathBuf::from("samples");
    let mut presets: Vec<String> = vec!["tiny".into(), "small".into()];
    let mut include_large = false;
    let mut only: Option<String> = None;
    let mut clean = false;
    let mut dry_run = false;

    let mut files: Option<usize> = None;
    let mut binary_every: Option<usize> = None;
    let mut depth: Option<usize> = None;
    let mut width: Option<usize> = None;
    let mut size: Option<usize> = None;
    let mut filters: Option<Vec<String>> = None;
    let mut ignores: Option<Vec<String>> = None;

    let mut it = env::args().skip(1).peekable();
    while let Some(arg) = it.next() {
        match arg.as_str() {
            "--out" => {
                out = PathBuf::from(expect_value("--out", &mut it));
            }
            "--presets" => {
                presets = parse_csv(expect_value("--presets", &mut it));
            }
            "--include-large" => include_large = true,
            "--only" => {
                only = Some(expect_value("--only", &mut it).to_lowercase());
            }
            "--clean" => clean = true,
            "--dry-run" => dry_run = true,

            // overrides (effective with --only)
            "--files" => files = parse_usize(expect_value("--files", &mut it)),
            "--binary-every" => binary_every = parse_usize(expect_value("--binary-every", &mut it)),
            "--depth" => depth = parse_usize(expect_value("--depth", &mut it)),
            "--width" => width = parse_usize(expect_value("--width", &mut it)),
            "--size" => size = parse_usize(expect_value("--size", &mut it)),
            "--filters" => filters = Some(parse_csv(expect_value("--filters", &mut it))),
            "--ignores" => ignores = Some(parse_csv(expect_value("--ignores", &mut it))),
            "--help" | "-h" => {
                print_help();
                std::process::exit(0);
            }
            other => {
                eprintln!("Unknown argument: {}", other);
                print_help();
                std::process::exit(2);
            }
        }
    }

    if include_large && !presets.iter().any(|p| p == "large") {
        presets.push("large".into());
    }

    Args {
        out,
        presets,
        include_large,
        only,
        clean,
        dry_run,
        files,
        binary_every,
        depth,
        width,
        size,
        filters,
        ignores,
    }
}

fn expect_value<'a, I>(flag: &str, it: &mut I) -> String
where
    I: Iterator<Item = String>,
{
    if let Some(v) = it.next() {
        v
    } else {
        eprintln!("{flag} requires a value");
        std::process::exit(2);
    }
}

fn parse_usize(s: String) -> Option<usize> {
    match s.parse::<usize>() {
        Ok(v) => Some(v),
        Err(_) => {
            eprintln!("Invalid number: {}", s);
            std::process::exit(2);
        }
    }
}

fn parse_csv(s: String) -> Vec<String> {
    s.split(',')
        .map(|x| x.trim().to_string())
        .filter(|x| !x.is_empty())
        .collect()
}

fn print_help() {
    println!(
        r#"generate_samples - generate synthetic datasets for benchmarking

Usage:
  generate_samples [--out DIR] [--presets CSV] [--include-large]
                   [--only NAME] [--clean] [--dry-run]
                   [--files N] [--binary-every N] [--depth D] [--width W]
                   [--size BYTES] [--filters CSV] [--ignores CSV]

Examples:
  # Default (tiny, small) into ./samples
  generate_samples

  # Include medium and large
  generate_samples --presets tiny,small,medium --include-large

  # Only 'small' with custom parameters
  generate_samples --only small --files 5000 --depth 4 --width 4 --size 1024

  # Clean output directory before generating
  generate_samples --clean

  # Dry-run (show plan, don't write)
  generate_samples --dry-run
"#
    );
}

fn write_text_file(path: &Path, bytes: usize) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let mut f = File::create(path)?;
    // Deterministic multi-line content ~40 bytes per line
    let line = b"let x = 42; // benchmark content line\n";
    let mut written = 0usize;
    while written + line.len() <= bytes {
        f.write_all(line)?;
        written += line.len();
    }
    if written < bytes {
        let remaining = &line[..(bytes - written).min(line.len())];
        f.write_all(remaining)?;
        written += remaining.len();
    }
    // Ensure trailing newline for nicer line-numbered output
    if written == 0 || !path.to_string_lossy().ends_with('\n') {
        f.write_all(b"\n")?;
    }
    Ok(())
}

fn write_binary_file(path: &Path, bytes: usize) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let mut f = File::create(path)?;
    // Simple reproducible byte pattern
    for i in 0..bytes {
        let b = ((i as u8).wrapping_mul(31)).wrapping_add(7);
        f.write_all(&[b])?;
    }
    Ok(())
}

fn make_nested_dirs(base: &Path, depth: usize, width: usize) -> io::Result<Vec<PathBuf>> {
    let mut dirs = vec![base.to_path_buf()];
    for d in 1..=depth {
        let mut next = Vec::new();
        for parent in &dirs {
            for w in 0..width.max(1) {
                let child = parent.join(format!("d{}_{}", d, w));
                fs::create_dir_all(&child)?;
                next.push(child);
            }
        }
        dirs.extend(next);
    }
    Ok(dirs)
}

fn write_string(path: &Path, s: &str) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let mut f = File::create(path)?;
    f.write_all(s.as_bytes())
}

fn generate_dataset(root: &Path, spec: &DatasetSpec, dry_run: bool) -> io::Result<()> {
    let dataset_dir = root.join(&spec.name);
    let project_dir = dataset_dir.join("project");
    let src_dir = project_dir.join("src");
    let docs_dir = project_dir.join("docs");
    let assets_dir = project_dir.join("assets");
    let ignored_target = project_dir.join("target");
    let ignored_node_modules = project_dir.join("node_modules");

    println!(
        "- [{}] files={}, bin_every={}, depth={}, width={}, size={}, filters={:?}, ignores={:?}",
        spec.name,
        spec.text_files,
        spec.binary_every,
        spec.depth,
        spec.width,
        spec.text_file_size,
        spec.filters,
        spec.ignores
    );

    if dry_run {
        return Ok(());
    }

    fs::create_dir_all(&src_dir)?;
    fs::create_dir_all(&docs_dir)?;
    fs::create_dir_all(&assets_dir)?;
    fs::create_dir_all(&ignored_target)?;
    fs::create_dir_all(&ignored_node_modules)?;

    // Write dataset README and .gitignore to discourage accidental commits
    write_string(
        &dataset_dir.join("README.txt"),
        &format!(
            "Synthetic dataset '{}'\n\
             - Generated by scripts/generate_samples.rs\n\
             - Intended for local benchmarking and testing\n\
             - May be large; avoid committing this folder\n",
            spec.name
        ),
    )?;
    write_string(
        &dataset_dir.join(".gitignore"),
        "*\n!.gitignore\n!README.txt\n",
    )?;

    let mut all_dirs = Vec::new();
    all_dirs.extend(make_nested_dirs(&src_dir, spec.depth, spec.width)?);
    all_dirs.extend(make_nested_dirs(&docs_dir, spec.depth, spec.width)?);
    all_dirs.extend(make_nested_dirs(&assets_dir, spec.depth, spec.width)?);

    // Distribute text files across dirs with round-robin extensions
    let text_exts = ["rs", "md", "txt", "toml"];
    let mut created = 0usize;
    let mut bin_counter = 0usize;

    'outer: for dir in &all_dirs {
        for i in 0..spec.width.max(1) {
            if created >= spec.text_files {
                break 'outer;
            }
            let ext = text_exts[created % text_exts.len()];
            let path = dir.join(format!("f{}_{}.{}", created, i, ext));
            write_text_file(&path, spec.text_file_size)?;
            created += 1;

            if spec.binary_every > 0 {
                bin_counter += 1;
                if bin_counter.is_multiple_of(spec.binary_every) {
                    let bpath = dir.join(format!("bin_{}_{}.bin", created, i));
                    write_binary_file(&bpath, 2048)?;
                }
            }
        }
    }

    // Populate ignored directories with content that should be skipped by the tool
    write_text_file(&ignored_target.join("ignored.rs"), spec.text_file_size)?;
    write_text_file(
        &ignored_node_modules.join("ignored.js"),
        spec.text_file_size,
    )?;

    // Top-level files
    write_text_file(&project_dir.join("README.md"), spec.text_file_size)?;
    write_text_file(&project_dir.join("Cargo.toml"), spec.text_file_size)?;

    Ok(())
}

fn apply_overrides(spec: &mut DatasetSpec, args: &Args) {
    if let Some(v) = args.files {
        spec.text_files = v;
    }
    if let Some(v) = args.binary_every {
        spec.binary_every = v;
    }
    if let Some(v) = args.depth {
        spec.depth = v;
    }
    if let Some(v) = args.width {
        spec.width = v;
    }
    if let Some(v) = args.size {
        spec.text_file_size = v;
    }
    if let Some(v) = args.filters.clone() {
        spec.filters = v;
    }
    if let Some(v) = args.ignores.clone() {
        spec.ignores = v;
    }
}

fn main() -> io::Result<()> {
    let args = parse_args();

    if args.clean && args.out.exists() && !args.dry_run {
        println!("Cleaning output directory: {}", args.out.display());
        fs::remove_dir_all(&args.out)?;
    }

    println!("Output directory: {}", args.out.display());
    println!("Dry run: {}", args.dry_run);

    let mut specs: Vec<DatasetSpec> = Vec::new();

    if let Some(name) = args.only.clone() {
        let mut spec = DatasetSpec::with_name(&name).unwrap_or_else(|| {
            eprintln!("Unknown preset for --only: {}", name);
            std::process::exit(2);
        });
        apply_overrides(&mut spec, &args);
        specs.push(spec);
    } else {
        for p in &args.presets {
            if let Some(spec) = DatasetSpec::with_name(p) {
                specs.push(spec);
            } else {
                eprintln!("Unknown preset: {}", p);
                std::process::exit(2);
            }
        }
    }

    if args.dry_run {
        println!("Planned datasets:");
        for s in &specs {
            println!(
                "  - {}: files={}, bin_every={}, depth={}, width={}, size={}",
                s.name, s.text_files, s.binary_every, s.depth, s.width, s.text_file_size
            );
        }
        return Ok(());
    }

    fs::create_dir_all(&args.out)?;
    // Guard .gitignore at the root samples folder
    let root_gitignore = args.out.join(".gitignore");
    if !root_gitignore.exists() {
        write_string(&root_gitignore, "*\n!.gitignore\n")?;
    }

    for spec in specs {
        generate_dataset(&args.out, &spec, false)?;
    }

    println!("Done.");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expect_value() {
        let mut args = vec!["--out".to_string(), "samples".to_string()].into_iter();
        let value = expect_value("--out", &mut args);
        assert_eq!(value, "samples");
    }
}
