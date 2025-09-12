use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Once;
use std::time::Duration;

use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use tempfile::tempdir;

use context_builder::cli::Args;
use context_builder::{Prompter, run_with_args};

static INIT: Once = Once::new();

fn init_bench_env() {
    INIT.call_once(|| {
        // Make benches silent by default so console I/O doesn't skew measurements.
        // Wrapped in `unsafe` to satisfy diagnostics in this environment.
        unsafe {
            std::env::set_var("CB_SILENT", "1");
        }
    });
}

/// Prompter that always auto-confirms. Used to avoid interactive pauses during benchmarks.
struct NoPrompt;

impl Prompter for NoPrompt {
    fn confirm_processing(&self, _file_count: usize) -> std::io::Result<bool> {
        Ok(true)
    }
    fn confirm_overwrite(&self, _file_path: &str) -> std::io::Result<bool> {
        Ok(true)
    }
}

/// Specification for generating a synthetic dataset for benchmarking.
#[derive(Clone)]
struct DatasetSpec {
    /// Human-friendly name used in the benchmark ID.
    name: &'static str,
    /// Approximate number of text files to generate.
    text_files: usize,
    /// Generate one binary file every `binary_every` text files (0 disables binary generation).
    binary_every: usize,
    /// Directory tree depth.
    depth: usize,
    /// Number of subdirectories per directory level.
    width: usize,
    /// Size of each text file (in bytes).
    text_file_size: usize,
    /// File extensions to include in benchmark (others should be ignored).
    filters: Vec<String>,
    /// Directory/file names to ignore (by component name).
    ignores: Vec<String>,
}

fn write_text_file(path: &Path, bytes: usize) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    let mut content = String::with_capacity(bytes);
    // Generate deterministic content consisting of multiple lines
    // Approx 40 bytes per line -> repeat to reach desired size
    let line = "let x = 42; // benchmark content line\n";
    while content.len() < bytes {
        content.push_str(line);
    }
    // Trim to exact size
    content.truncate(bytes);
    // Ensure trailing newline for line-numbering path
    if !content.ends_with('\n') {
        content.push('\n');
    }
    fs::write(path, content).unwrap();
}

fn write_binary_file(path: &Path, bytes: usize) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    let mut data = Vec::with_capacity(bytes);
    // Simple reproducible byte pattern
    for i in 0..bytes {
        data.push(((i as u8).wrapping_mul(31)).wrapping_add(7));
    }
    fs::write(path, data).unwrap();
}

/// Generate a synthetic project directory structure under `root`, returning the input directory path.
fn generate_dataset(root: &Path, spec: &DatasetSpec) -> PathBuf {
    let input_dir = root.join("project");
    let src_dir = input_dir.join("src");
    let docs_dir = input_dir.join("docs");
    let assets_dir = input_dir.join("assets");
    let ignored_target = input_dir.join("target"); // will be ignored if configured
    let ignored_node_modules = input_dir.join("node_modules"); // will be ignored if configured

    fs::create_dir_all(&src_dir).unwrap();
    fs::create_dir_all(&docs_dir).unwrap();
    fs::create_dir_all(&assets_dir).unwrap();
    fs::create_dir_all(&ignored_target).unwrap();
    fs::create_dir_all(&ignored_node_modules).unwrap();

    // Generate nested directories
    fn make_nested_dirs(base: &Path, depth: usize, width: usize) -> Vec<PathBuf> {
        let mut dirs = vec![base.to_path_buf()];
        for d in 1..=depth {
            let mut next_level = Vec::new();
            for parent in &dirs {
                for w in 0..width {
                    let child = parent.join(format!("d{}_{}", d, w));
                    fs::create_dir_all(&child).unwrap();
                    next_level.push(child);
                }
            }
            dirs.extend(next_level);
        }
        dirs
    }

    let all_dirs = {
        let mut v = Vec::new();
        v.extend(make_nested_dirs(&src_dir, spec.depth, spec.width));
        v.extend(make_nested_dirs(&docs_dir, spec.depth, spec.width));
        v.extend(make_nested_dirs(&assets_dir, spec.depth, spec.width));
        v
    };

    // Extensions to distribute across text files
    let text_exts = ["rs", "md", "txt", "toml"];

    // Create text files distributed across dirs
    let mut created = 0usize;
    let mut bin_counter = 0usize;

    'outer: for dir in &all_dirs {
        for i in 0..spec.width.max(1) {
            if created >= spec.text_files {
                break 'outer;
            }
            // Round-robin extensions
            let ext = text_exts[created % text_exts.len()];
            let path = dir.join(format!("f{}_{}.{}", created, i, ext));
            write_text_file(&path, spec.text_file_size);
            created += 1;

            if spec.binary_every > 0 {
                bin_counter += 1;
                if bin_counter.is_multiple_of(spec.binary_every) {
                    let bpath = dir.join(format!("bin_{}_{}.bin", created, i));
                    write_binary_file(&bpath, 2048);
                }
            }
        }
    }

    // Populate ignored directories with content that should not be processed
    write_text_file(&ignored_target.join("ignored.rs"), spec.text_file_size);
    write_text_file(
        &ignored_node_modules.join("ignored.js"),
        spec.text_file_size,
    );

    // Add some top-level files
    write_text_file(&input_dir.join("README.md"), spec.text_file_size);
    write_text_file(&input_dir.join("Cargo.toml"), spec.text_file_size);

    input_dir
}

/// Run a single benchmark scenario for a given dataset and line-numbering mode.
fn bench_scenario(c: &mut Criterion, spec: DatasetSpec, line_numbers: bool) {
    let tmp = tempdir().unwrap();
    let root = tmp.path();

    // Prefer local ./samples/<dataset>/project if it exists, else use CB_BENCH_DATASET_DIR, else generate temp dataset
    let samples_default = PathBuf::from("samples").join(spec.name).join("project");
    let input_dir = if samples_default.exists() {
        samples_default
    } else if let Some(dir) = std::env::var_os("CB_BENCH_DATASET_DIR") {
        let path = PathBuf::from(dir).join(spec.name).join("project");

        if !path.exists() {
            panic!(
                "CB_BENCH_DATASET_DIR is set but dataset not found at {}",
                path.display()
            );
        }

        path
    } else {
        generate_dataset(root, &spec)
    };

    let output_path = root.join(format!(
        "output_{}_{}.md",
        spec.name,
        if line_numbers { "ln" } else { "raw" }
    ));

    let args = Args {
        input: input_dir.to_string_lossy().into_owned(),
        output: output_path.to_string_lossy().into_owned(),
        filter: spec.filters.clone(),
        ignore: spec.ignores.clone(),
        preview: false,
        token_count: false,
        line_numbers,
        yes: true,
        diff_only: false,
    };

    let prompter = NoPrompt;

    let mut group = c.benchmark_group("context_builder");

    group.measurement_time(Duration::from_secs(20));
    group.sample_size(20);

    let mode = if cfg!(feature = "parallel") {
        "parallel"
    } else {
        "sequential"
    };
    let ln = if line_numbers {
        "line_numbers"
    } else {
        "no_line_numbers"
    };
    let id = BenchmarkId::new(
        format!(
            "{}-{}files-{}B",
            spec.name, spec.text_files, spec.text_file_size
        ),
        format!("{}-{}", ln, mode),
    );

    group.bench_with_input(id, &args, |b, _| {
        b.iter(|| {
            // Allow repeated overwrites; keep the output path stable to avoid filesystem churn
            let _ = std::hint::black_box(run_with_args(
                Args {
                    input: args.input.clone(),
                    output: args.output.clone(),
                    filter: args.filter.clone(),
                    ignore: args.ignore.clone(),
                    preview: args.preview,
                    token_count: args.token_count,
                    line_numbers: args.line_numbers,
                    yes: true,
                    diff_only: false,
                },
                &prompter,
            ));
        });
    });

    group.finish();
}

/// Benchmarks:
/// - tiny: ~100 files, small size
/// - small: ~1,000 files
/// - medium: ~5,000 files (enabled only if CB_BENCH_MEDIUM=1)
///
/// These datasets are generated in a temporary directory at runtime to keep the
/// benchmark self-contained. Binary files are generated but filtered out by
/// the `filters` configuration so they aren't processed.
///
/// Run:
///   cargo bench --bench context_bench
pub fn context_benchmark(c: &mut Criterion) {
    // Ensure silent-by-default for benchmarks
    init_bench_env();

    // Common filters and ignores: ignore typical heavy dirs; only include text code/docs
    let common_filters = vec!["rs".into(), "md".into(), "txt".into(), "toml".into()];
    let common_ignores = vec!["target".into(), "node_modules".into()];

    // Tiny dataset
    let tiny = DatasetSpec {
        name: "tiny",
        text_files: 100,
        binary_every: 10,
        depth: 2,
        width: 3,
        text_file_size: 256,
        filters: common_filters.clone(),
        ignores: common_ignores.clone(),
    };

    // Small dataset
    let small = DatasetSpec {
        name: "small",
        text_files: 1_000,
        binary_every: 20,
        depth: 3,
        width: 4,
        text_file_size: 512,
        filters: common_filters.clone(),
        ignores: common_ignores.clone(),
    };

    // Medium dataset (can be enabled via env var to avoid heavy runs by default)
    let include_medium = std::env::var("CB_BENCH_MEDIUM").ok().as_deref() == Some("1");
    let medium = DatasetSpec {
        name: "medium",
        text_files: 5_000,
        binary_every: 25,
        depth: 4,
        width: 4,
        text_file_size: 800,
        filters: common_filters.clone(),
        ignores: common_ignores.clone(),
    };

    // For each dataset, run benchmarks with and without line numbers
    for ds in [tiny, small] {
        bench_scenario(c, ds.clone(), false);
        bench_scenario(c, ds, true);
    }

    if include_medium {
        bench_scenario(c, medium.clone(), false);
        bench_scenario(c, medium, true);
    }
}

criterion_group!(benches, context_benchmark);
criterion_main!(benches);
