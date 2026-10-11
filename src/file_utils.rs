use ignore::{DirEntry, WalkBuilder, overrides::OverrideBuilder};
use std::fs::{self, File};
use std::io::{self, IsTerminal, Read, Write};
use std::path::{Path, PathBuf};

use crate::markdown::{CONTENT_HASH_PREFIX, REPORT_TITLE_LINE};

/// Bytes of each file examined when looking for a previous context-builder
/// report. The signature is the header (`REPORT_TITLE_LINE` plus a
/// `CONTENT_HASH_PREFIX` line), which is written before the file tree.
const CONTEXT_OUTPUT_PREFIX_LEN: usize = 8 * 1024;

/// Cargo and other tools drop this file in cache directories (see
/// <https://bford.info/cachedir/>). `target/` contains one.
const CACHEDIR_TAG: &str = "CACHEDIR.TAG";
/// Basenames of generated dependency lockfiles (category 5).
///
/// A lockfile is a resolved dependency snapshot, such as `Cargo.lock`,
/// `package-lock.json`, or `uv.lock`. It is not a project manifest
/// ([`ROOT_MANIFESTS`]). Matched by exact basename at any depth.
pub const LOCKFILES: &[&str] = &[
    "Cargo.lock",
    "package-lock.json",
    "yarn.lock",
    "pnpm-lock.yaml",
    "Gemfile.lock",
    "poetry.lock",
    "composer.lock",
    "go.sum",
    "bun.lockb",
    "flake.lock",
    "uv.lock",
    "Pipfile.lock",
    "bun.lock",
    "deno.lock",
    "mix.lock",
    "pubspec.lock",
    "npm-shrinkwrap.json",
    "Package.resolved",
];

/// Basenames of root project manifests (category 0).
///
/// A manifest defines a project or package: metadata, dependency
/// declarations, and the build entry. Examples: `Cargo.toml`, `package.json`,
/// `pyproject.toml`, `pom.xml`. Not lockfiles ([`LOCKFILES`]), READMEs,
/// changelogs, or tool-only config. Matched by exact basename at any depth.
pub const ROOT_MANIFESTS: &[&str] = &[
    "Cargo.toml",
    "package.json",
    "tsconfig.json",
    "pyproject.toml",
    "setup.py",
    "setup.cfg",
    "go.mod",
    "Gemfile",
    "pom.xml",
    "build.gradle",
    "build.gradle.kts",
    "Package.swift",
    "composer.json",
    "deno.json",
    "mix.exs",
    "pubspec.yaml",
    "flake.nix",
    "requirements.txt",
    "Pipfile",
];

/// Tool config and key project docs. Category 0 only when the file is at the repository root.
const PRIORITY_CONFIG_AND_DOCS: &[&str] = &[
    "context-builder.toml",
    ".gitignore",
    "README.md",
    "README",
    "README.txt",
    "README.rst",
    "AGENTS.md",
    "CLAUDE.md",
    "GEMINI.md",
    "COPILOT.md",
    "CONTRIBUTING.md",
];

/// Changelog and release-history basenames. Category 3 (docs) at any depth,
/// including the repository root.
const HISTORY_DOC_NAMES: &[&str] = &[
    "CHANGELOG.md",
    "CHANGELOG",
    "HISTORY.md",
    "HISTORY",
    "CHANGES.md",
    "NEWS.md",
];

/// README basenames. At the repository root these are category 0 and sort
/// ahead of other root files. Nested READMEs sort with manifests, ahead of
/// the other files in that directory.
const README_NAMES: &[&str] = &["README.md", "README", "README.txt", "README.rst"];

/// Directory names that mark tests or benchmarks, at any depth.
const TEST_DIR_NAMES: &[&str] = &[
    "tests",
    "test",
    "spec",
    "__tests__",
    "benches",
    "benchmarks",
    "testdata",
    "fixtures",
];

/// Source extensions for which a `test_*` basename is a test module.
const SOURCE_EXTENSIONS: &[&str] = &[
    "rs", "go", "py", "ts", "tsx", "js", "jsx", "java", "c", "cpp", "h", "hpp", "rb", "swift",
    "kt", "scala", "ex", "exs", "zig", "hs",
];

/// Build and CI filenames matched by exact basename.
///
/// `CMakeLists.txt` is included even though it has a `.txt` extension, so the
/// extension fallback cannot classify it as prose. `justfile` is matched
/// separately and case-insensitively.
const BUILD_FILE_NAMES: &[&str] = &[
    "Makefile",
    "CMakeLists.txt",
    "Dockerfile",
    "Containerfile",
    "Taskfile",
    "Rakefile",
    "Vagrantfile",
];

/// Test-module filename suffixes, matched against the basename at any depth.
const TEST_FILE_SUFFIXES: &[&str] = &[
    "_test.rs",
    "_test.go",
    "_test.py",
    "_spec.rb",
    ".test.ts",
    ".test.tsx",
    ".test.js",
    ".test.jsx",
    ".spec.ts",
    ".spec.tsx",
    ".spec.js",
    ".spec.jsx",
];

/// Returns a numeric category for file relevance ordering.
/// Lower numbers appear first in output. Categories:
/// 0 = Root-level project config and key docs (root Cargo.toml, root README, …)
/// 1 = Source code (src/, lib/) — manifests and READMEs, then entry points, within each directory
/// 2 = Tests and benchmarks (tests/, benches/, test/, spec/, testdata/, fixtures/)
/// 3 = Documentation, changelogs, scripts, and everything else
/// 4 = Build/CI infrastructure (.github/, .circleci/, Dockerfile, etc.)
/// 5 = Generated/lock files (Cargo.lock, package-lock.json, etc.)
fn file_relevance_category(path: &Path, base_path: &Path) -> u8 {
    let rel = normalized_relative(path, base_path);
    let parts: Vec<&str> = rel.split('/').filter(|part| !part.is_empty()).collect();
    if parts.is_empty() {
        return 3;
    }
    let name = parts[parts.len() - 1];
    let parents = &parts[..parts.len() - 1];
    let first = parts[0];

    // Lockfiles outrank every other basename match, including manifests.
    if LOCKFILES.contains(&name) {
        return 5;
    }
    // Changelogs are docs at every depth, including the repository root.
    if HISTORY_DOC_NAMES.contains(&name) {
        return 3;
    }
    // Category 0 is only the repository root: manifests, README, and the other
    // priority config/docs. Nested copies rank with their directory.
    if parents.is_empty()
        && (ROOT_MANIFESTS.contains(&name) || PRIORITY_CONFIG_AND_DOCS.contains(&name))
    {
        return 0;
    }
    // Test markers win over source-root classification (`src/foo_test.go` is a
    // test, not source) and over the docs/extension fallback.
    if is_test_path(parents, name) {
        return 2;
    }

    match first {
        "src" | "lib" | "crates" | "packages" | "internal" | "cmd" | "pkg" => 1,
        "docs" | "doc" | "examples" | "scripts" | "tools" | "assets" => 3,
        // Build/CI infrastructure — useful context but not core source.
        ".github" | ".circleci" | ".gitlab" | ".buildkite" => 4,
        // A nested README stays with the rest of its directory rather than
        // falling to the Markdown/docs category (`apps/web/README.md` sorts
        // with `apps/web/package.json`, ahead of it via `directory_lead_rank`).
        _ if !parents.is_empty() && is_readme_name(name) => 1,
        _ => category_from_name(name),
    }
}

/// Relative path with `\` folded to `/`, so component checks do not depend on
/// the OS separator. `Path` on Unix does not split on `\`, and
/// `to_string_lossy()` keeps the backslashes Windows paths actually use.
fn normalized_relative(path: &Path, base_path: &Path) -> String {
    path.strip_prefix(base_path)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

fn is_test_path(parents: &[&str], name: &str) -> bool {
    parents.iter().any(|dir| TEST_DIR_NAMES.contains(dir))
        || (parents.is_empty() && TEST_DIR_NAMES.contains(&name))
        || is_test_filename(name)
}

fn is_test_filename(name: &str) -> bool {
    if TEST_FILE_SUFFIXES
        .iter()
        .any(|suffix| name.ends_with(suffix))
    {
        return true;
    }
    // `test_*.py` and the same `test_*` rule for other source extensions.
    // The basename is used so this matches at any depth, not only the repo root.
    name.starts_with("test_")
        && matches!(
            file_extension(name),
            Some(ext) if SOURCE_EXTENSIONS.contains(&ext)
        )
}

fn file_extension(name: &str) -> Option<&str> {
    let (stem, ext) = name.rsplit_once('.')?;
    if stem.is_empty() { None } else { Some(ext) }
}

fn is_build_file(name: &str) -> bool {
    BUILD_FILE_NAMES.contains(&name) || name.eq_ignore_ascii_case("justfile")
}

fn category_from_name(name: &str) -> u8 {
    if is_build_file(name) {
        return 4;
    }
    match file_extension(name) {
        Some("md" | "txt" | "rst" | "adoc") => 3,
        Some(_) => 1,
        None => 3,
    }
}

/// Returns a sub-priority for sorting within the same relevance category.
/// Lower values appear first. Entry points (main, lib, mod) get priority 0,
/// other files get priority 1. This ensures LLMs see architectural entry
/// points before helper modules.
fn file_entry_point_priority(path: &Path) -> u8 {
    if let Some("main" | "lib" | "mod" | "index" | "app" | "__init__") =
        path.file_stem().and_then(|s| s.to_str())
    {
        0
    } else {
        1
    }
}

/// Strip one leading `*.` or `.`, then lowercase.
///
/// `.rs`, `*.rs`, and `RS` all become `rs`. Names that are already ripgrep
/// types (`toml`, `md`, `rust`) are left as those type names so their
/// built-in globs still apply.
fn normalize_filter(filter: &str) -> String {
    let stripped = if let Some(rest) = filter.strip_prefix("*.") {
        rest
    } else if let Some(rest) = filter.strip_prefix('.') {
        rest
    } else {
        filter
    };
    stripped.to_ascii_lowercase()
}

/// Ripgrep type names are non-empty and alphanumeric. `all` is alphanumeric
/// but reserved by `TypesBuilder::add` (it means "every defined type").
fn is_legal_type_name(name: &str) -> bool {
    !name.is_empty() && name != "all" && name.chars().all(|c| c.is_alphanumeric())
}

fn unrecognized_filter_error(original: &str, normalized: &str) -> io::Error {
    let shown = if original == normalized {
        format!("'{original}'")
    } else {
        format!("'{original}' (normalized to '{normalized}')")
    };
    io::Error::new(
        io::ErrorKind::InvalidInput,
        format!(
            "Unrecognized file type filter {shown}. Filters are ripgrep file types \
             (for example, rust, toml, md) or plain extensions (for example, rs). \
             A leading '.' or '*.' is stripped and the value is lowercased; \
             what remains must be letters and digits only."
        ),
    )
}

/// Apply `--filter` values as ripgrep file types.
///
/// Known types keep their built-in globs (`toml` still matches `Cargo.lock`).
/// A name that is not a known type is registered as `*.{name}` when that name
/// is a legal type name. Anything that still cannot be registered is returned
/// as an error — `TypesBuilder::build` is never unwrapped.
fn configure_file_type_filters(walker: &mut WalkBuilder, filters: &[String]) -> io::Result<()> {
    if filters.is_empty() {
        return Ok(());
    }

    let mut type_builder = ignore::types::TypesBuilder::new();
    type_builder.add_defaults();
    for filter in filters {
        let name = normalize_filter(filter);
        // `all` selects every default type. It is not a legal `add` name.
        if name == "all" {
            type_builder.select("all");
            continue;
        }
        if !is_legal_type_name(&name) {
            return Err(unrecognized_filter_error(filter, &name));
        }
        // Appending `*.{name}` extends an existing ripgrep type and creates a
        // custom extension type otherwise. The previous code did this too;
        // the `Result` used to be discarded, which is what made `build` panic.
        let glob = format!("*.{name}");
        type_builder
            .add(&name, &glob)
            .map_err(|_| unrecognized_filter_error(filter, &name))?;
        type_builder.select(&name);
    }

    let types = type_builder
        .build()
        .map_err(|_| unrecognized_filter_error("(combined)", "(combined)"))?;
    walker.types(types);
    Ok(())
}

/// Files selected for one run, after the lockfile default is applied.
pub struct FileCollection {
    pub files: Vec<DirEntry>,
    /// How many [`LOCKFILES`] basenames were dropped. Zero when `include_lockfiles` is set.
    pub skipped_lockfiles: usize,
}

/// Collects all files to be processed using `ignore` crate for efficient traversal.
///
/// `filters` are ripgrep file types or extensions. A leading `.` or `*.` is
/// stripped and the value is lowercased. A filter that still is not a legal
/// type name returns an error instead of panicking.
///
/// `auto_ignores` are runtime-computed exclusion patterns (e.g., the tool's own
/// output file or cache directory). They are processed identically to user ignores
/// but kept separate to avoid polluting user-facing configuration.
///
/// Hidden files and directories are skipped. Use [`collect_files_ext`] with
/// `include_hidden` to opt in (that still prunes VCS metadata directories).
/// Lockfiles ([`LOCKFILES`]) are skipped. Pass [`collect_files_reporting`] with
/// `include_lockfiles` when they should be kept.
pub fn collect_files(
    base_path: &Path,
    filters: &[String],
    ignores: &[String],
    auto_ignores: &[String],
) -> io::Result<Vec<DirEntry>> {
    collect_files_ext(base_path, filters, ignores, auto_ignores, false)
}

/// Like [`collect_files`], with an explicit hidden-file switch.
///
/// When `include_hidden` is true, dotfiles and dot-directories are visited
/// (for example `.github/workflows/ci.yml` and `.gitignore`). Version-control
/// metadata directories (`.git`, `.hg`, `.svn`, `.bzr`) are still pruned so
/// object stores are never pulled in. Symlinks, `.gitignore`, and the default
/// heavy-directory ignores are unchanged. Likely-secret files are not decided
/// here; see `content_filter`.
pub fn collect_files_ext(
    base_path: &Path,
    filters: &[String],
    ignores: &[String],
    auto_ignores: &[String],
    include_hidden: bool,
) -> io::Result<Vec<DirEntry>> {
    Ok(collect_files_reporting(
        base_path,
        filters,
        ignores,
        auto_ignores,
        include_hidden,
        false,
    )?
    .files)
}

/// Same selection as [`collect_files_ext`], plus the lockfile skip count.
///
/// `include_lockfiles` keeps basenames listed in [`LOCKFILES`]. A type filter
/// such as `toml` or `lock` does not include those files on its own.
pub fn collect_files_reporting(
    base_path: &Path,
    filters: &[String],
    ignores: &[String],
    auto_ignores: &[String],
    include_hidden: bool,
    include_lockfiles: bool,
) -> io::Result<FileCollection> {
    let mut walker = WalkBuilder::new(base_path);
    if include_hidden {
        // `hidden(false)` means "do not ignore hidden files".
        walker.hidden(false);
    }
    // A `.git` directory or gitdir file at the walk root or any ancestor is a
    // real checkout. Keep the crate defaults (`require_git(true)`,
    // `parents(true)`): parent ignore files inside that repo apply, and ignore
    // files above the repository do not.
    //
    // With no checkout, `require_git(false)` alone would still read every
    // ancestor `.gitignore` (a `$HOME` dotfiles pattern of `*` and `!*/`
    // then hides every file). `parents(false)` limits `.gitignore` and
    // `.ignore` to files inside the walk root, which is the B6 fix.
    if git_link_in_ancestors(base_path) {
        walker.require_git(true);
        walker.parents(true);
    } else {
        walker.require_git(false);
        walker.parents(false);
    }
    // Skip cache directories (Cargo's `target/` ships a CACHEDIR.TAG) without
    // descending into them. The root itself is never filtered out by the walker.
    // One predicate: `filter_entry` replaces any earlier filter, so the cache-dir
    // skip and the VCS-metadata prune (needed with `--hidden`) must share it.
    walker.filter_entry(|entry| !directory_has_cachedir_tag(entry) && !is_vcs_metadata_dir(entry));

    // Build overrides for custom ignore patterns
    let mut override_builder = OverrideBuilder::new(base_path);

    // Hardcoded auto-ignores for common heavy directories. `.gitignore` is
    // applied even without a `.git` directory, but many trees never list
    // these names. Without the defaults, dependency folders can dominate
    // the output.
    //
    // IMPORTANT: These are added FIRST so that user ignores can override them.
    // The ignore crate uses "last-match-wins" semantics, so a user can whitelist
    // a legitimate "vendor" or "build" dir by passing it as a filter pattern.
    //
    // IMPORTANT: Patterns must NOT contain a slash — the ignore crate anchors
    // slash-containing patterns to the root, so `!dir/**` would only match
    // top-level dirs, missing nested ones like `apps/web/node_modules/`.
    let default_ignores = [
        "node_modules",
        "__pycache__",
        ".venv",
        "venv",
        ".tox",
        ".mypy_cache",
        ".pytest_cache",
        ".ruff_cache",
        "vendor",  // Go, PHP, Ruby
        ".bundle", // Ruby
        "bower_components",
        ".next",       // Next.js build output
        ".nuxt",       // Nuxt build output
        ".svelte-kit", // SvelteKit build output
        ".angular",    // Angular cache
        "dist",        // Common build output
        "build",       // Common build output
        ".gradle",     // Gradle cache
                       // `.cargo` is deliberately not listed: it is hidden (skipped by default),
                       // and with `--hidden` a project's `.cargo/config.toml` is real config.
    ];
    for dir in &default_ignores {
        // No slash in pattern → matches at any depth (not root-anchored)
        let pattern = format!("!{}", dir);
        if let Err(e) = override_builder.add(&pattern) {
            log::warn!("Skipping invalid default-ignore '{}': {}", dir, e);
        }
    }
    // `target` is anchored to the walk root. An unanchored name would hide a
    // real source directory such as `src/target/`. Cargo build output at any
    // depth still carries a CACHEDIR.TAG and is skipped above.
    if let Err(e) = override_builder.add("!/target") {
        log::warn!("Skipping invalid default-ignore '/target': {}", e);
    }

    // User-specified ignore patterns (added AFTER defaults so they can override)
    for pattern in ignores {
        // Attention: Confusing pattern ahead!
        // Add the pattern to the override builder with ! prefix to ignore matching files.
        // In OverrideBuilder, patterns without ! are whitelist (include) patterns,
        // while patterns with ! are ignore patterns.
        let ignore_pattern = format!("!{}", pattern);
        if let Err(e) = override_builder.add(&ignore_pattern) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Invalid ignore pattern '{}': {}", pattern, e),
            ));
        }
    }
    // Apply auto-computed ignore patterns (output file, cache dir, etc.)
    for pattern in auto_ignores {
        let ignore_pattern = format!("!{}", pattern);
        if let Err(e) = override_builder.add(&ignore_pattern) {
            log::warn!("Skipping invalid auto-ignore pattern '{}': {}", pattern, e);
        }
    }
    // Also, always ignore the config file itself
    if let Err(e) = override_builder.add("!context-builder.toml") {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("Failed to add config ignore: {}", e),
        ));
    }

    let overrides = override_builder.build().map_err(|e| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("Failed to build overrides: {}", e),
        )
    })?;
    walker.overrides(overrides);
    configure_file_type_filters(&mut walker, filters)?;

    let mut files: Vec<DirEntry> = walker
        .build()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_some_and(|ft| ft.is_file()))
        .filter(|e| !is_prior_context_output(e.path()))
        .collect();

    let skipped_lockfiles = if include_lockfiles {
        0
    } else {
        let before = files.len();
        files.retain(|entry| !is_lockfile_path(entry.path()));
        before - files.len()
    };

    // Category, then directory, then README/manifests within that directory,
    // then entry points, then path. Separators are normalized so the order
    // does not depend on the OS.
    files.sort_by(|a, b| {
        relevance_sort_key(a.path(), base_path).cmp(&relevance_sort_key(b.path(), base_path))
    });

    Ok(FileCollection {
        files,
        skipped_lockfiles,
    })
}

fn is_lockfile_path(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| LOCKFILES.contains(&name))
}

fn is_readme_name(name: &str) -> bool {
    README_NAMES.contains(&name)
}

/// Lead rank inside one directory. Lower comes first.
/// Root README is ahead of root manifests; nested READMEs share the manifest bucket.
fn directory_lead_rank(name: &str, at_root: bool) -> u8 {
    if at_root && is_readme_name(name) {
        0
    } else if is_readme_name(name) || ROOT_MANIFESTS.contains(&name) {
        1
    } else {
        2
    }
}

fn relevance_sort_key(path: &Path, base_path: &Path) -> (u8, String, u8, u8, String) {
    let rel = normalized_relative(path, base_path);
    let parts: Vec<&str> = rel.split('/').filter(|part| !part.is_empty()).collect();
    let name = parts.last().copied().unwrap_or("");
    let directory = if parts.len() > 1 {
        parts[..parts.len() - 1].join("/")
    } else {
        String::new()
    };
    let at_root = directory.is_empty();
    (
        file_relevance_category(path, base_path),
        directory,
        directory_lead_rank(name, at_root),
        file_entry_point_priority(path),
        rel,
    )
}

/// Stderr notice when lockfiles were left out. `None` when nothing was skipped.
pub fn lockfile_skip_notice(count: usize) -> Option<String> {
    match count {
        0 => None,
        1 => Some("Skipped 1 lockfile (use --include-lockfiles to include)".to_string()),
        n => Some(format!(
            "Skipped {n} lockfiles (use --include-lockfiles to include)"
        )),
    }
}

/// `.git` / `.hg` / `.svn` / `.bzr` stay out of the walk even with `--hidden`.
fn is_vcs_metadata_dir(entry: &DirEntry) -> bool {
    if entry.depth() == 0 {
        return false;
    }
    let is_dir = entry.file_type().is_some_and(|ft| ft.is_dir());
    if !is_dir {
        return false;
    }
    matches!(
        entry.file_name().to_str(),
        Some(".git" | ".hg" | ".svn" | ".bzr")
    )
}

/// True when `entry` is a directory that contains a `CACHEDIR.TAG` file.
fn directory_has_cachedir_tag(entry: &DirEntry) -> bool {
    entry.file_type().is_some_and(|ft| ft.is_dir()) && entry.path().join(CACHEDIR_TAG).is_file()
}

/// True when the walk root or one of its ancestors contains a `.git` directory
/// or file (worktrees and submodules use a gitdir file).
///
/// Relative roots are resolved against the current directory first. A relative
/// `-d .` otherwise has no real ancestors, so a repository above the process
/// cwd would be missed.
fn git_link_in_ancestors(base_path: &Path) -> bool {
    let mut current = absolute_walk_root(base_path);
    loop {
        if is_git_link(&current.join(".git")) {
            return true;
        }
        if !current.pop() {
            return false;
        }
    }
}

fn is_git_link(path: &Path) -> bool {
    fs::metadata(path).is_ok_and(|meta| meta.is_dir() || meta.is_file())
}

fn absolute_walk_root(base_path: &Path) -> PathBuf {
    let joined = if base_path.is_absolute() {
        base_path.to_path_buf()
    } else {
        match std::env::current_dir() {
            Ok(cwd) => cwd.join(base_path),
            Err(_) => base_path.to_path_buf(),
        }
    };
    let normalized = normalize_lexically(&joined);
    if normalized.as_os_str().is_empty() {
        PathBuf::from(".")
    } else {
        normalized
    }
}

/// Collapse `.` and `..` without touching the filesystem.
fn normalize_lexically(path: &Path) -> PathBuf {
    let mut out = PathBuf::new();
    for component in path.components() {
        match component {
            std::path::Component::CurDir => {}
            std::path::Component::ParentDir => {
                out.pop();
            }
            other => out.push(other.as_os_str()),
        }
    }
    out
}

/// True when `path` is a previous context-builder report.
///
/// The tool writes [`REPORT_TITLE_LINE`] as the first line and a
/// `Content hash:` line of 16 lowercase hex digits in the header
/// (see `markdown.rs`). Only [`CONTEXT_OUTPUT_PREFIX_LEN`] bytes are read.
fn is_prior_context_output(path: &Path) -> bool {
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(_) => return false,
    };
    let mut buf = [0u8; CONTEXT_OUTPUT_PREFIX_LEN];
    let n = match file.read(&mut buf) {
        Ok(n) => n,
        Err(_) => return false,
    };
    if !header_is_context_builder_output(&buf[..n]) {
        return false;
    }
    log::debug!("skipping prior report: {}", path.display());
    true
}

fn header_is_context_builder_output(prefix: &[u8]) -> bool {
    let text = String::from_utf8_lossy(prefix);
    let mut lines = text.lines();
    if lines.next() != Some(REPORT_TITLE_LINE) {
        return false;
    }
    let rest: Vec<&str> = lines.collect();
    if rest.iter().any(|l| is_content_hash_line(l)) {
        return true;
    }
    // The auto-diff renderer (lib.rs) writes the title followed directly by
    // `**Project:**` and `**Generated:**` lines and has no content hash.
    let mut meta = rest.iter().filter(|l| !l.is_empty());
    matches!(
        (meta.next(), meta.next()),
        (Some(p), Some(g)) if p.starts_with("**Project:** ") && g.starts_with("**Generated:** ")
    )
}

/// The header line `markdown.rs` writes: `Content hash: ` plus 16 lowercase hex digits.
fn is_content_hash_line(line: &str) -> bool {
    let Some(rest) = line.strip_prefix(CONTENT_HASH_PREFIX) else {
        return false;
    };
    rest.len() == 16 && rest.bytes().all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f'))
}

/// True when a person can answer a `[y/N]` prompt on stdin.
///
/// Pipes, redirects, and `/dev/null` are not terminals (`std::io::IsTerminal`).
/// Non-interactive callers proceed without prompting, the same way `--yes` and
/// `-o -` already do. The check lives here — not in `run_with_args` — so tests
/// that inject their own `Prompter` still control confirmations.
fn stdin_is_terminal() -> bool {
    can_prompt(io::stdin().is_terminal(), io::stderr().is_terminal())
}

/// A prompt needs a person on both ends: stdin to answer and stderr (where
/// prompts are written) to see the question. With stderr redirected, e.g.
/// `2>build.log`, the question would be invisible and the run would hang.
fn can_prompt(stdin_tty: bool, stderr_tty: bool) -> bool {
    stdin_tty && stderr_tty
}

/// Writes `prompt` to stderr and returns whether the answer was `y`/`Y`.
///
/// Prompts must not go to stdout: `-o -` and any caller capturing stdout would
/// otherwise treat the question as document content.
fn prompt_yes(prompt: &str) -> io::Result<bool> {
    err!("{prompt}");
    io::stderr().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().eq_ignore_ascii_case("y"))
}

/// Asks for user confirmation to overwrite an existing file.
///
/// When stdin is not a terminal the prompt is skipped and the file is overwritten.
pub fn confirm_overwrite(file_path: &str) -> io::Result<bool> {
    if !stdin_is_terminal() {
        return Ok(true);
    }
    prompt_yes(&format!(
        "The file '{file_path}' already exists. Overwrite? [y/N] "
    ))
}

pub fn find_latest_file(dir: &Path) -> io::Result<Option<PathBuf>> {
    if !dir.is_dir() {
        return Ok(None);
    }

    let mut latest_file = None;
    let mut latest_time = std::time::SystemTime::UNIX_EPOCH;

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            let metadata = fs::metadata(&path)?;
            let modified = metadata.modified()?;
            if modified > latest_time {
                latest_time = modified;
                latest_file = Some(path);
            }
        }
    }

    Ok(latest_file)
}

#[cfg(test)]
mod tests {
    #[test]
    fn can_prompt_requires_stdin_and_stderr_terminals() {
        assert!(can_prompt(true, true));
        assert!(
            !can_prompt(true, false),
            "stderr redirected: prompt invisible"
        );
        assert!(!can_prompt(false, true));
        assert!(!can_prompt(false, false));
    }

    use super::*;
    use std::fs;
    use std::path::Path;
    use tempfile::tempdir;

    fn to_rel_paths(mut entries: Vec<DirEntry>, base: &Path) -> Vec<String> {
        entries.sort_by_key(|e| e.path().to_path_buf());
        entries
            .iter()
            .map(|e| {
                e.path()
                    .strip_prefix(base)
                    .unwrap()
                    .to_string_lossy()
                    .replace('\\', "/")
            })
            .collect()
    }

    #[test]
    fn collect_files_respects_filters() {
        let dir = tempdir().unwrap();
        let base = dir.path();

        // create files
        fs::create_dir_all(base.join("src")).unwrap();
        fs::create_dir_all(base.join("scripts")).unwrap();
        fs::write(base.join("src").join("main.rs"), "fn main() {}").unwrap();
        fs::write(base.join("Cargo.toml"), "[package]\nname=\"x\"").unwrap();
        fs::write(base.join("README.md"), "# readme").unwrap();
        fs::write(base.join("scripts").join("build.sh"), "#!/bin/sh\n").unwrap();

        let filters = vec!["rs".to_string(), "toml".to_string()];
        let ignores: Vec<String> = vec![];

        let files = collect_files(base, &filters, &ignores, &[]).unwrap();
        let relative_paths = to_rel_paths(files, base);

        assert!(relative_paths.contains(&"src/main.rs".to_string()));
        assert!(relative_paths.contains(&"Cargo.toml".to_string()));
        assert!(!relative_paths.contains(&"README.md".to_string()));
        assert!(!relative_paths.contains(&"scripts/build.sh".to_string()));
    }

    #[test]
    fn collect_files_respects_ignores_for_dirs_and_files() {
        let dir = tempdir().unwrap();
        let base = dir.path();

        fs::create_dir_all(base.join("src")).unwrap();
        fs::create_dir_all(base.join("target")).unwrap();
        fs::create_dir_all(base.join("node_modules")).unwrap();

        fs::write(base.join("src").join("main.rs"), "fn main() {}").unwrap();
        fs::write(base.join("target").join("artifact.txt"), "bin").unwrap();
        fs::write(base.join("node_modules").join("pkg.js"), "console.log();").unwrap();
        fs::write(base.join("README.md"), "# readme").unwrap();

        let filters: Vec<String> = vec![];
        let ignores: Vec<String> = vec!["target".into(), "node_modules".into(), "README.md".into()];

        let files = collect_files(base, &filters, &ignores, &[]).unwrap();
        let relative_paths = to_rel_paths(files, base);

        assert!(relative_paths.contains(&"src/main.rs".to_string()));
        assert!(!relative_paths.contains(&"target/artifact.txt".to_string()));
        assert!(!relative_paths.contains(&"node_modules/pkg.js".to_string()));
        assert!(!relative_paths.contains(&"README.md".to_string()));
    }

    #[test]
    fn collect_files_handles_invalid_ignore_pattern() {
        let dir = tempdir().unwrap();
        let base = dir.path();

        fs::create_dir_all(base.join("src")).unwrap();
        fs::write(base.join("src").join("main.rs"), "fn main() {}").unwrap();

        let filters: Vec<String> = vec![];
        let ignores: Vec<String> = vec!["[".into()]; // Invalid regex pattern

        let result = collect_files(base, &filters, &ignores, &[]);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Invalid ignore pattern")
        );
    }

    #[test]
    fn collect_files_empty_directory() {
        let dir = tempdir().unwrap();
        let base = dir.path();

        let filters: Vec<String> = vec![];
        let ignores: Vec<String> = vec![];

        let files = collect_files(base, &filters, &ignores, &[]).unwrap();
        assert!(files.is_empty());
    }

    #[test]
    fn collect_files_no_matching_filters() {
        let dir = tempdir().unwrap();
        let base = dir.path();

        fs::write(base.join("README.md"), "# readme").unwrap();
        fs::write(base.join("script.py"), "print('hello')").unwrap();

        let filters = vec!["rs".to_string()]; // Only Rust files
        let ignores: Vec<String> = vec![];

        let files = collect_files(base, &filters, &ignores, &[]).unwrap();
        assert!(files.is_empty());
    }

    #[test]
    fn collect_files_normalizes_dotted_glob_and_case_filters() {
        let dir = tempdir().unwrap();
        let base = dir.path();
        fs::create_dir_all(base.join("src")).unwrap();
        fs::write(base.join("src").join("a.rs"), "fn main() {}").unwrap();
        fs::write(base.join("README.md"), "# readme").unwrap();

        for filter in [".rs", "*.rs", "RS", "Rs", ".RS", "*.RS"] {
            let files = collect_files(base, &[filter.to_string()], &[], &[])
                .unwrap_or_else(|e| panic!("filter {filter:?} should not error: {e}"));
            let relative_paths = to_rel_paths(files, base);
            assert!(
                relative_paths.contains(&"src/a.rs".to_string()),
                "filter {filter:?} should include src/a.rs, got {relative_paths:?}"
            );
            assert!(
                !relative_paths.contains(&"README.md".to_string()),
                "filter {filter:?} should exclude README.md, got {relative_paths:?}"
            );
        }
    }

    #[test]
    fn collect_files_unrecognized_filter_is_an_error() {
        let dir = tempdir().unwrap();
        let base = dir.path();
        fs::write(base.join("a.rs"), "fn main() {}").unwrap();

        for filter in ["d.ts", "c++", "tar.gz", "*.d.ts", ".c++"] {
            let err = collect_files(base, &[filter.to_string()], &[], &[])
                .expect_err("unrecognized filter must return an error, not panic");
            let msg = err.to_string();
            assert!(
                msg.contains(filter),
                "error for {filter:?} should name the filter, got {msg}"
            );
            assert!(
                msg.contains("Unrecognized file type filter"),
                "error for {filter:?} should be user-facing, got {msg}"
            );
            assert!(
                !msg.contains("UnrecognizedFileType"),
                "error for {filter:?} should not leak the ignore-crate panic payload, got {msg}"
            );
        }

        // A later bad filter must not be dropped or panic after a valid one.
        let err = collect_files(base, &["rs".to_string(), "tar.gz".to_string()], &[], &[])
            .expect_err("mixed filters should still error");
        assert!(err.to_string().contains("tar.gz"));
    }

    #[test]
    fn collect_files_keeps_ripgrep_type_expansion() {
        let dir = tempdir().unwrap();
        let base = dir.path();
        fs::write(base.join("Cargo.toml"), "[package]\nname=\"x\"\n").unwrap();
        fs::write(base.join("Cargo.lock"), "# lock\n").unwrap();
        fs::write(base.join("notes.mdx"), "# notes\n").unwrap();
        fs::write(base.join("skip.txt"), "nope\n").unwrap();
        fs::write(base.join("notes.unknownext"), "x\n").unwrap();

        for filter in ["toml", "TOML", ".toml", "*.toml"] {
            // Lockfiles are opt-in, so ask for them to observe the type expansion.
            let files = collect_files_reporting(base, &[filter.to_string()], &[], &[], false, true)
                .unwrap_or_else(|e| panic!("filter {filter:?} should keep the toml type: {e}"))
                .files;
            let relative_paths = to_rel_paths(files, base);

            // By default the same filter still leaves the lockfile out.
            let default_paths = to_rel_paths(
                collect_files(base, &[filter.to_string()], &[], &[]).unwrap(),
                base,
            );
            assert!(!default_paths.contains(&"Cargo.lock".to_string()));
            assert!(
                relative_paths.contains(&"Cargo.toml".to_string()),
                "{filter:?}: {relative_paths:?}"
            );
            assert!(
                relative_paths.contains(&"Cargo.lock".to_string()),
                "{filter:?} should still expand to the ripgrep toml type: {relative_paths:?}"
            );
            assert!(!relative_paths.contains(&"skip.txt".to_string()));
        }

        // `all` is ripgrep's "every defined type" name. Lowercasing must not
        // turn it into an unrecognized filter.
        for filter in ["all", "ALL"] {
            let files = collect_files(base, &[filter.to_string()], &[], &[])
                .unwrap_or_else(|e| panic!("filter {filter:?} should select known types: {e}"));
            let relative_paths = to_rel_paths(files, base);
            assert!(
                relative_paths.contains(&"Cargo.toml".to_string()),
                "{filter:?}: {relative_paths:?}"
            );
            assert!(
                relative_paths.contains(&"notes.mdx".to_string()),
                "{filter:?}: {relative_paths:?}"
            );
            assert!(
                !relative_paths.contains(&"notes.unknownext".to_string()),
                "{filter:?} should not include extensions outside the default types: {relative_paths:?}"
            );
        }

        let md_files = collect_files(base, &["md".to_string()], &[], &[]).unwrap();
        let md_paths = to_rel_paths(md_files, base);
        assert!(
            md_paths.contains(&"notes.mdx".to_string()),
            "md should keep the ripgrep markdown globs, got {md_paths:?}"
        );
        assert!(!md_paths.contains(&"Cargo.lock".to_string()));
    }

    #[test]
    fn collect_files_ignores_config_file() {
        let dir = tempdir().unwrap();
        let base = dir.path();

        fs::write(base.join("context-builder.toml"), "[config]").unwrap();
        fs::write(base.join("other.toml"), "[other]").unwrap();

        let filters: Vec<String> = vec![];
        let ignores: Vec<String> = vec![];

        let files = collect_files(base, &filters, &ignores, &[]).unwrap();
        let relative_paths = to_rel_paths(files, base);

        assert!(!relative_paths.contains(&"context-builder.toml".to_string()));
        assert!(relative_paths.contains(&"other.toml".to_string()));
    }

    #[test]
    fn find_latest_file_empty_directory() {
        let dir = tempdir().unwrap();
        let result = find_latest_file(dir.path()).unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn find_latest_file_nonexistent_directory() {
        let dir = tempdir().unwrap();
        let nonexistent = dir.path().join("nonexistent");
        let result = find_latest_file(&nonexistent).unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn find_latest_file_single_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.txt");
        fs::write(&file_path, "content").unwrap();

        let result = find_latest_file(dir.path()).unwrap();
        assert!(result.is_some());
        assert_eq!(result.unwrap(), file_path);
    }

    #[test]
    fn find_latest_file_multiple_files() {
        let dir = tempdir().unwrap();

        let file1 = dir.path().join("old.txt");
        let file2 = dir.path().join("new.txt");

        fs::write(&file1, "old content").unwrap();
        std::thread::sleep(std::time::Duration::from_millis(10));
        fs::write(&file2, "new content").unwrap();

        let result = find_latest_file(dir.path()).unwrap();
        assert!(result.is_some());
        assert_eq!(result.unwrap(), file2);
    }

    #[test]
    fn find_latest_file_ignores_directories() {
        let dir = tempdir().unwrap();
        let subdir = dir.path().join("subdir");
        fs::create_dir(&subdir).unwrap();

        let file_path = dir.path().join("test.txt");
        fs::write(&file_path, "content").unwrap();

        let result = find_latest_file(dir.path()).unwrap();
        assert!(result.is_some());
        assert_eq!(result.unwrap(), file_path);
    }

    #[test]
    fn test_confirm_overwrite_function_exists() {
        // This function requires user interaction
        // We can verify it exists and has the expected signature

        // For testing purposes, we know this function prompts for user input
        // and returns Ok(true) if user types "y" or "Y", Ok(false) otherwise

        // The function signature should be:
        // pub fn confirm_overwrite(file_path: &str) -> io::Result<bool>

        // We can't easily test the interactive behavior without mocking stdin,
        // but we can ensure the function compiles and has the right signature
        let _: fn(&str) -> std::io::Result<bool> = confirm_overwrite;
    }

    #[test]
    fn test_collect_files_handles_permission_errors() {
        // Test what happens when we can't access a directory
        // This is harder to test portably, but we can test with invalid patterns
        let dir = tempdir().unwrap();
        let base = dir.path();

        // Test with a pattern that might cause issues
        let filters: Vec<String> = vec![];
        let ignores: Vec<String> = vec!["[invalid".into()]; // Incomplete bracket

        let result = collect_files(base, &filters, &ignores, &[]);
        assert!(result.is_err());
    }

    #[test]
    fn test_find_latest_file_permission_error() {
        // Test behavior when we can't read directory metadata
        use std::path::Path;

        // Test with a path that doesn't exist
        let nonexistent = Path::new("/this/path/should/not/exist/anywhere");
        let result = find_latest_file(nonexistent);

        // Should return Ok(None) for non-existent directories
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[test]
    fn test_collect_files_with_symlinks() {
        // Test behavior with symbolic links (if supported on platform)
        let dir = tempdir().unwrap();
        let base = dir.path();

        // Create a regular file
        fs::write(base.join("regular.txt"), "content").unwrap();

        // On Unix-like systems, try creating a symlink
        #[cfg(unix)]
        {
            use std::os::unix::fs::symlink;
            let _ = symlink("regular.txt", base.join("link.txt"));
        }

        // On Windows, symlinks require special privileges, so skip this part
        #[cfg(windows)]
        {
            // Just create another regular file to test
            fs::write(base.join("another.txt"), "content2").unwrap();
        }

        let filters: Vec<String> = vec![];
        let ignores: Vec<String> = vec![];

        let files = collect_files(base, &filters, &ignores, &[]).unwrap();
        // Should find at least the regular file
        assert!(!files.is_empty());
    }

    #[test]
    fn test_file_relevance_category_lockfiles() {
        let dir = tempdir().unwrap();
        let base = dir.path();

        let lockfiles = [
            "Cargo.lock",
            "package-lock.json",
            "yarn.lock",
            "pnpm-lock.yaml",
            "Gemfile.lock",
            "poetry.lock",
            "composer.lock",
            "go.sum",
            "bun.lockb",
            "flake.lock",
        ];

        for lockfile in &lockfiles {
            fs::write(base.join(lockfile), "lock content").unwrap();
        }

        let skipped = collect_files(base, &[], &[], &[]).unwrap();
        assert!(
            skipped.is_empty(),
            "lockfiles are excluded unless include_lockfiles is set"
        );

        let (kept, n_skipped) = collected_rels(base, &[], true);
        assert_eq!(n_skipped, 0);
        for lockfile in &lockfiles {
            assert!(
                kept.iter().any(|path| path == lockfile),
                "Expected {lockfile} to be collected when include_lockfiles is set"
            );
        }
    }

    #[test]
    fn test_file_relevance_category_test_files_in_src() {
        let dir = tempdir().unwrap();
        let base = dir.path();

        fs::create_dir_all(base.join("src/tests")).unwrap();
        fs::create_dir_all(base.join("src/test")).unwrap();
        fs::create_dir_all(base.join("src/__tests__")).unwrap();

        fs::write(base.join("src/tests/auth.rs"), "fn test_auth() {}").unwrap();
        fs::write(base.join("src/test/helper.rs"), "fn helper() {}").unwrap();
        fs::write(base.join("src/__tests__/main.rs"), "fn main_test() {}").unwrap();
        fs::write(base.join("src/lib.rs"), "pub fn lib() {}").unwrap();

        let files = collect_files(base, &[], &[], &[]).unwrap();
        assert!(!files.is_empty());
    }

    #[test]
    fn test_file_relevance_category_benchmarks_in_src() {
        let dir = tempdir().unwrap();
        let base = dir.path();

        fs::create_dir_all(base.join("src/benches")).unwrap();
        fs::write(base.join("src/benches/my_bench.rs"), "fn bench() {}").unwrap();
        fs::write(base.join("src/main.rs"), "fn main() {}").unwrap();

        let files = collect_files(base, &[], &[], &[]).unwrap();
        assert_eq!(files.len(), 2);
    }

    #[test]
    fn test_file_relevance_category_test_file_patterns() {
        let dir = tempdir().unwrap();
        let base = dir.path();

        fs::write(base.join("my_test.rs"), "fn test() {}").unwrap();
        fs::write(base.join("test_main.rs"), "fn test_main() {}").unwrap();
        fs::write(base.join("my_test.go"), "func Test() {}").unwrap();
        fs::write(base.join("my_spec.rb"), "describe 'test' do end").unwrap();
        fs::write(base.join("app.test.ts"), "describe('test', () => {});").unwrap();
        fs::write(base.join("app.spec.ts"), "it('works', () => {});").unwrap();
        fs::write(base.join("main.rs"), "fn main() {}").unwrap();

        let files = collect_files(base, &[], &[], &[]).unwrap();
        assert_eq!(files.len(), 7);
    }

    #[test]
    fn test_file_relevance_category_build_files_without_extension() {
        let dir = tempdir().unwrap();
        let base = dir.path();

        fs::write(base.join("Makefile"), "all:\n\techo hello").unwrap();
        fs::write(base.join("Dockerfile"), "FROM alpine").unwrap();
        fs::write(base.join("Justfile"), "default:\n\techo hi").unwrap();

        let files = collect_files(base, &[], &[], &[]).unwrap();
        assert!(!files.is_empty());
    }

    #[test]
    fn test_collect_files_with_auto_ignores() {
        let dir = tempdir().unwrap();
        let base = dir.path();

        fs::write(base.join("test.txt"), "content").unwrap();

        let auto_ignores = vec!["test.txt".to_string()];
        let files = collect_files(base, &[], &[], &auto_ignores).unwrap();

        assert!(files.is_empty());
    }

    #[test]
    fn test_file_relevance_ci_directories() {
        let dir = tempdir().unwrap();
        let base = dir.path();

        fs::create_dir_all(base.join("ci")).unwrap();
        fs::write(base.join("ci/build.sh"), "#!/bin/sh").unwrap();

        let files = collect_files(base, &[], &[], &[]).unwrap();
        assert!(!files.is_empty());
    }

    #[test]
    fn test_file_relevance_docs_scripts_dirs() {
        let dir = tempdir().unwrap();
        let base = dir.path();

        fs::create_dir_all(base.join("docs")).unwrap();
        fs::create_dir_all(base.join("scripts")).unwrap();
        fs::create_dir_all(base.join("examples")).unwrap();
        fs::create_dir_all(base.join("tools")).unwrap();
        fs::create_dir_all(base.join("assets")).unwrap();

        fs::write(base.join("docs/guide.md"), "# Guide").unwrap();
        fs::write(base.join("scripts/build.sh"), "#!/bin/sh").unwrap();
        fs::write(base.join("examples/demo.rs"), "fn demo() {}").unwrap();
        fs::write(base.join("tools/helper.py"), "def helper(): pass").unwrap();
        fs::write(base.join("assets/logo.svg"), "<svg/>").unwrap();

        let files = collect_files(base, &[], &[], &[]).unwrap();
        assert_eq!(files.len(), 5);
    }

    #[test]
    fn test_file_relevance_various_source_extensions() {
        let dir = tempdir().unwrap();
        let base = dir.path();

        fs::write(base.join("main.go"), "package main").unwrap();
        fs::write(base.join("App.java"), "class App {}").unwrap();
        fs::write(base.join("main.py"), "print('hello')").unwrap();
        fs::write(base.join("app.swift"), "import Foundation").unwrap();
        fs::write(base.join("Main.kt"), "fun main() {}").unwrap();
        fs::write(base.join("main.scala"), "object Main {}").unwrap();
        fs::write(base.join("app.ex"), "defmodule App do end").unwrap();
        fs::write(base.join("main.zig"), "pub fn main() void {}").unwrap();
        fs::write(base.join("Lib.hs"), "module Lib where").unwrap();
        fs::write(base.join("main.c"), "int main() { return 0; }").unwrap();
        fs::write(base.join("main.cpp"), "int main() { return 0; }").unwrap();
        fs::write(base.join("main.rb"), "puts 'hello'").unwrap();

        let files = collect_files(base, &[], &[], &[]).unwrap();
        assert_eq!(files.len(), 12);
    }

    #[test]
    fn test_file_entry_point_priority_various_names() {
        let dir = tempdir().unwrap();
        let base = dir.path();

        fs::write(base.join("main.rs"), "fn main() {}").unwrap();
        fs::write(base.join("lib.rs"), "pub fn lib() {}").unwrap();
        fs::write(base.join("mod.rs"), "pub mod sub;").unwrap();
        fs::write(base.join("index.js"), "module.exports = {};").unwrap();
        fs::write(base.join("app.py"), "print('app')").unwrap();
        fs::write(base.join("__init__.py"), "pass").unwrap();
        fs::write(base.join("other.rs"), "fn other() {}").unwrap();

        let files = collect_files(base, &[], &[], &[]).unwrap();
        assert_eq!(files.len(), 7);
    }

    #[test]
    fn test_collect_files_config_files_priority() {
        let dir = tempdir().unwrap();
        let base = dir.path();

        fs::write(base.join("package.json"), "{}").unwrap();
        fs::write(base.join("tsconfig.json"), "{}").unwrap();
        fs::write(base.join("pyproject.toml"), "[project]").unwrap();
        fs::write(base.join("go.mod"), "module test").unwrap();
        fs::write(base.join("Gemfile"), "gem 'rails'").unwrap();
        fs::write(base.join("README.md"), "# Readme").unwrap();
        fs::write(base.join("AGENTS.md"), "# Agents").unwrap();
        fs::write(base.join("CONTRIBUTING.md"), "# Contrib").unwrap();
        fs::write(base.join("CHANGELOG.md"), "# Changes").unwrap();

        let files = collect_files(base, &[], &[], &[]).unwrap();
        assert!(!files.is_empty());
    }

    #[test]
    fn no_git_honors_gitignore_cachedir_tag_and_target() {
        let dir = tempdir().unwrap();
        let base = dir.path();

        fs::create_dir_all(base.join("src")).unwrap();
        fs::write(base.join("src/main.rs"), "fn main() {}").unwrap();
        fs::write(base.join("keep.txt"), "keep").unwrap();
        fs::write(base.join(".gitignore"), "secret.log\ngenerated/\n").unwrap();
        fs::write(base.join("secret.log"), "hidden").unwrap();
        fs::create_dir_all(base.join("generated")).unwrap();
        fs::write(base.join("generated/junk.txt"), "junk").unwrap();

        // Root `target/` with no tag. The default ignore is anchored at the
        // walk root, so this directory is still excluded.
        fs::create_dir_all(base.join("target/debug")).unwrap();
        fs::write(base.join("target/debug/x.d"), "dep").unwrap();

        // Nested source directory named `target`, no tag — must be kept.
        fs::create_dir_all(base.join("src/target")).unwrap();
        fs::write(base.join("src/target/notes.rs"), "fn notes() {}").unwrap();

        // Nested Cargo output: the tag skips it even though the name is not
        // anchored past the walk root.
        fs::create_dir_all(base.join("pkg/target")).unwrap();
        fs::write(
            base.join("pkg/target/CACHEDIR.TAG"),
            "Signature: 8a477f597d28d172789f06886806bc55\n",
        )
        .unwrap();
        fs::write(base.join("pkg/target/out.txt"), "artifact").unwrap();

        // Cache dir that is not named `target` — only the tag should exclude it.
        fs::create_dir_all(base.join("my-cache")).unwrap();
        fs::write(
            base.join("my-cache/CACHEDIR.TAG"),
            "Signature: 8a477f597d28d172789f06886806bc55\n",
        )
        .unwrap();
        fs::write(base.join("my-cache/blob.txt"), "blob").unwrap();

        // Nested `.gitignore` still applies when there is no checkout.
        fs::create_dir_all(base.join("sub")).unwrap();
        fs::write(base.join("sub/.gitignore"), "nested-secret.txt\n").unwrap();
        fs::write(base.join("sub/nested-secret.txt"), "nope").unwrap();
        fs::write(base.join("sub/ok.txt"), "yes").unwrap();

        let rel = to_rel_paths(collect_files(base, &[], &[], &[]).unwrap(), base);
        assert!(rel.contains(&"src/main.rs".to_string()));
        assert!(rel.contains(&"keep.txt".to_string()));
        assert!(rel.contains(&"src/target/notes.rs".to_string()));
        assert!(rel.contains(&"sub/ok.txt".to_string()));
        assert!(!rel.iter().any(|p| p.contains("secret.log")));
        assert!(!rel.iter().any(|p| p.contains("generated/")));
        assert!(
            !rel.iter()
                .any(|p| p == "target" || p.starts_with("target/"))
        );
        assert!(!rel.iter().any(|p| p.contains("pkg/target")));
        assert!(!rel.iter().any(|p| p.contains("my-cache/")));
        assert!(!rel.iter().any(|p| p.contains("nested-secret.txt")));
    }

    #[test]
    fn parent_gitignore_without_git_keeps_tree_files() {
        let dir = tempdir().unwrap();
        fs::write(dir.path().join(".gitignore"), "*\n!*/\n").unwrap();
        let proj = dir.path().join("proj");
        fs::create_dir_all(&proj).unwrap();
        fs::write(proj.join("keep.txt"), "keep").unwrap();
        fs::write(proj.join(".gitignore"), "secret.txt\n").unwrap();
        fs::write(proj.join("secret.txt"), "nope").unwrap();

        let rel = to_rel_paths(collect_files(&proj, &[], &[], &[]).unwrap(), &proj);
        assert!(rel.contains(&"keep.txt".to_string()));
        assert!(!rel.iter().any(|p| p == "secret.txt"));
    }

    #[test]
    fn repo_root_gitignore_applies_inside_nested_project() {
        let dir = tempdir().unwrap();
        // Dotfiles pattern above the repository must not apply.
        fs::write(dir.path().join(".gitignore"), "*\n!*/\n").unwrap();
        let repo = dir.path().join("repo");
        fs::create_dir_all(repo.join(".git")).unwrap();
        fs::write(repo.join(".gitignore"), "secret.txt\n").unwrap();
        let proj = repo.join("proj");
        fs::create_dir_all(&proj).unwrap();
        fs::write(proj.join("keep.txt"), "keep").unwrap();
        fs::write(proj.join("secret.txt"), "nope").unwrap();

        let rel = to_rel_paths(collect_files(&proj, &[], &[], &[]).unwrap(), &proj);
        assert!(
            rel.contains(&"keep.txt".to_string()),
            "parent dotfiles gitignore hid the tree: {rel:?}"
        );
        assert!(
            !rel.iter().any(|p| p == "secret.txt"),
            "repo-root .gitignore was not applied: {rel:?}"
        );
    }

    /// Classify `rel` as a relative path. The base is not a prefix, so the
    /// whole string (including Windows separators) is what the heuristic sees.
    fn category_of(rel: &str) -> u8 {
        file_relevance_category(Path::new(rel), Path::new("NOT_A_PREFIX"))
    }

    #[test]
    fn cmake_lists_txt_is_a_build_manifest() {
        // `.txt` used to take the docs branch before the build-file list ran.
        assert_eq!(category_of("CMakeLists.txt"), 4);
        assert_eq!(category_of("notes.txt"), 3);
        for name in [
            "Makefile",
            "Dockerfile",
            "Containerfile",
            "Taskfile",
            "Rakefile",
            "Vagrantfile",
        ] {
            assert_eq!(category_of(name), 4, "{name}");
        }
    }

    #[test]
    fn justfile_is_matched_case_insensitively() {
        for name in ["justfile", "Justfile", "JUSTFILE", "JustFile"] {
            assert_eq!(category_of(name), 4, "{name}");
        }
    }

    #[test]
    fn windows_style_separators_match_test_directories() {
        let samples = [
            r"src\tests\auth.rs",
            r"src\lib.rs",
            r"internal\x\x_test.go",
            r"pkg\fixtures\sample.json",
            r"src\components\Button.test.tsx",
            r"app\test_models.py",
            r"testdata\input.txt",
        ];
        for windows in samples {
            let unix = windows.replace('\\', "/");
            assert_eq!(
                category_of(windows),
                category_of(&unix),
                "{windows} should classify like {unix}"
            );
        }
        assert_eq!(category_of(r"src\tests\auth.rs"), 2);
        assert_eq!(category_of(r"src\lib.rs"), 1);
        assert_eq!(category_of(r"src\contest.rs"), 1);

        // Base stripping still works for normal OS paths.
        assert_eq!(
            file_relevance_category(Path::new("/repo/src/tests/auth.rs"), Path::new("/repo")),
            2
        );
        assert_eq!(
            file_relevance_category(Path::new("/repo/src/lib.rs"), Path::new("/repo")),
            1
        );
    }

    #[test]
    fn hidden_files_stay_out_unless_requested_and_git_metadata_stays_out() {
        let dir = tempdir().unwrap();
        let base = dir.path();
        fs::create_dir_all(base.join(".github/workflows")).unwrap();
        fs::create_dir_all(base.join(".git/objects")).unwrap();
        fs::write(base.join("README.md"), "# hi").unwrap();
        fs::write(base.join(".gitignore"), "target/\n").unwrap();
        fs::write(base.join(".github/workflows/ci.yml"), "name: ci\n").unwrap();
        fs::write(base.join(".git/config"), "[core]\n").unwrap();
        fs::write(base.join(".git/objects/pack"), "blob").unwrap();

        let visible = to_rel_paths(collect_files(base, &[], &[], &[]).unwrap(), base);
        assert!(visible.contains(&"README.md".to_string()));
        assert!(
            !visible
                .iter()
                .any(|p| p.starts_with('.') || p.contains("/."))
        );

        let hidden = to_rel_paths(collect_files_ext(base, &[], &[], &[], true).unwrap(), base);
        assert!(hidden.contains(&"README.md".to_string()));
        assert!(hidden.contains(&".gitignore".to_string()));
        assert!(hidden.contains(&".github/workflows/ci.yml".to_string()));
        assert!(
            !hidden.iter().any(|p| p == ".git" || p.starts_with(".git/")),
            "VCS metadata must stay out with --hidden: {hidden:?}"
        );
    }

    #[test]
    fn gitdir_file_counts_as_a_repo() {
        let dir = tempdir().unwrap();
        fs::write(dir.path().join(".gitignore"), "*\n!*/\n").unwrap();
        let repo = dir.path().join("repo");
        fs::create_dir_all(&repo).unwrap();
        fs::write(repo.join(".git"), "gitdir: /somewhere\n").unwrap();
        fs::write(repo.join(".gitignore"), "secret.txt\n").unwrap();
        let proj = repo.join("proj");
        fs::create_dir_all(&proj).unwrap();
        fs::write(proj.join("keep.txt"), "keep").unwrap();
        fs::write(proj.join("secret.txt"), "nope").unwrap();

        let rel = to_rel_paths(collect_files(&proj, &[], &[], &[]).unwrap(), &proj);
        assert!(rel.contains(&"keep.txt".to_string()), "{rel:?}");
        assert!(!rel.iter().any(|p| p == "secret.txt"), "{rel:?}");
    }

    #[test]
    fn skips_prior_report_header_but_not_near_misses() {
        let dir = tempdir().unwrap();
        let base = dir.path();
        fs::write(base.join("keep.txt"), "hello").unwrap();
        fs::write(
            base.join("old.md"),
            "# Directory Structure Report\n\nThis document contains all files from the `proj` directory, optimized for LLM consumption.\nContent hash: 0123456789abcdef\n\n## File Tree Structure\n",
        )
        .unwrap();
        // Title without the hash line the tool writes.
        fs::write(
            base.join("notes.md"),
            "# Directory Structure Report\n\nJust a heading.\n",
        )
        .unwrap();
        // Hash line, but not the report title.
        fs::write(base.join("other.md"), "Content hash: 0123456789abcdef\n").unwrap();
        // Title plus a hash line that is not 16 lowercase hex digits.
        fs::write(
            base.join("almost.md"),
            "# Directory Structure Report\n\nContent hash: not-a-real-hash\n",
        )
        .unwrap();

        let rel = to_rel_paths(collect_files(base, &[], &[], &[]).unwrap(), base);
        assert!(rel.contains(&"keep.txt".to_string()));
        assert!(rel.contains(&"notes.md".to_string()));
        assert!(rel.contains(&"other.md".to_string()));
        assert!(rel.contains(&"almost.md".to_string()));
        assert!(!rel.contains(&"old.md".to_string()));
    }

    #[test]
    fn skips_prior_auto_diff_report_but_not_near_misses() {
        let dir = tempdir().unwrap();
        let base = dir.path();
        fs::write(base.join("keep.txt"), "hello").unwrap();
        fs::write(
            base.join("auto.md"),
            "# Directory Structure Report\n\n**Project:** proj\n**Generated:** 2026-01-01 00:00:00 UTC\n\n## File Tree Structure\n",
        )
        .unwrap();
        // Title and Project line, but no Generated line.
        fs::write(
            base.join("near.md"),
            "# Directory Structure Report\n\n**Project:** proj\nSome prose.\n",
        )
        .unwrap();

        let rel = to_rel_paths(collect_files(base, &[], &[], &[]).unwrap(), base);
        assert!(rel.contains(&"keep.txt".to_string()));
        assert!(rel.contains(&"near.md".to_string()));
        assert!(!rel.contains(&"auto.md".to_string()));
    }

    #[test]
    fn anchored_auto_ignore_keeps_nested_same_basename() {
        let dir = tempdir().unwrap();
        let base = dir.path();
        fs::create_dir_all(base.join("docs")).unwrap();
        fs::write(base.join("output.md"), "# user notes at the root\n").unwrap();
        fs::write(base.join("docs/output.md"), "real doc\n").unwrap();
        fs::write(base.join("keep.txt"), "k").unwrap();

        let rel = to_rel_paths(
            collect_files(base, &[], &[], &["/output.md".to_string()]).unwrap(),
            base,
        );
        assert!(!rel.contains(&"output.md".to_string()));
        assert!(rel.contains(&"docs/output.md".to_string()));
        assert!(rel.contains(&"keep.txt".to_string()));
    }

    #[test]
    fn test_suffixes_are_detected_at_any_depth() {
        let tests = [
            "src/foo_test.go",
            "lib/foo_test.go",
            "crates/foo/foo_test.go",
            "internal/x/x_test.go",
            "cmd/tool/foo_test.go",
            "pkg/y/y_test.go",
            "src/components/Button.test.ts",
            "src/components/Button.test.tsx",
            "src/components/Button.test.js",
            "src/components/Button.test.jsx",
            "src/utils.spec.ts",
            "src/utils.spec.tsx",
            "src/utils.spec.js",
            "src/utils.spec.jsx",
            "mypkg/core_test.py",
            "src/test_utils.py",
            "app/test_models.py",
            "packages/web/src/index.test.ts",
            // Existing suffixes stay tests inside source roots too.
            "src/foo_test.rs",
            "src/my_spec.rb",
            "test_main.rs",
            "src/test_Button.tsx",
            "src/test_widget.jsx",
        ];
        for path in tests {
            assert_eq!(category_of(path), 2, "{path}");
        }

        let sources = [
            "src/foo.go",
            "lib/foo.go",
            "crates/foo/foo.go",
            "internal/x/x.go",
            "cmd/tool/main.go",
            "pkg/y/y.go",
            "src/components/Button.tsx",
            "src/components/Button.ts",
            "src/components/Button.js",
            "src/components/Button.jsx",
            "mypkg/core.py",
            "app/models.py",
            "packages/web/src/index.ts",
            "src/lib.rs",
            "contest.rs",
            "src/contest.rs",
            "latest.rs",
        ];
        for path in sources {
            assert_eq!(category_of(path), 1, "{path}");
        }
        // `doc/` is still documentation. Reclassifying that package is out of scope.
        assert_eq!(category_of("doc/api.go"), 3);
    }

    #[test]
    fn testdata_and_fixtures_directories_are_tests() {
        let tests = [
            "testdata/input.txt",
            "fixtures/sample.json",
            "src/testdata/fixture.txt",
            "pkg/fixtures/x.go",
            "lib/fixtures/keep.ts",
            "internal/x/testdata/input.json",
        ];
        for path in tests {
            assert_eq!(category_of(path), 2, "{path}");
        }
        assert_eq!(category_of("src/keep.rs"), 1);
        assert_eq!(category_of("docs/guide.md"), 3);
    }

    #[test]
    fn new_lockfiles_are_category_5() {
        let added = [
            "uv.lock",
            "Pipfile.lock",
            "bun.lock",
            "deno.lock",
            "mix.lock",
            "pubspec.lock",
            "npm-shrinkwrap.json",
            "Package.resolved",
        ];
        for name in added {
            assert!(LOCKFILES.contains(&name), "{name} missing from LOCKFILES");
            assert_eq!(category_of(name), 5, "{name}");
            assert_eq!(category_of(&format!("pkg/{name}")), 5, "nested {name}");
        }
        for name in LOCKFILES {
            assert_eq!(category_of(name), 5, "{name}");
        }
        // Category numbers stay as the code has them: 4 = CI, 5 = lock.
        assert_eq!(category_of(".github/workflows/ci.yml"), 4);
        assert_eq!(category_of("Dockerfile"), 4);
        assert_eq!(category_of("src/lib.rs"), 1);
    }

    #[test]
    fn new_manifests_are_category_0() {
        // An empty path has no components and ranks as a plain file.
        assert_eq!(category_of(""), 3);
        let added = [
            "pom.xml",
            "build.gradle",
            "build.gradle.kts",
            "Package.swift",
            "composer.json",
            "deno.json",
            "mix.exs",
            "pubspec.yaml",
            "flake.nix",
            "requirements.txt",
            "Pipfile",
        ];
        for name in added {
            assert!(
                ROOT_MANIFESTS.contains(&name),
                "{name} missing from ROOT_MANIFESTS"
            );
            assert_eq!(category_of(name), 0, "{name}");
        }
        for name in ROOT_MANIFESTS {
            assert_eq!(category_of(name), 0, "{name}");
            assert!(
                !LOCKFILES.contains(name),
                "{name} is in both ROOT_MANIFESTS and LOCKFILES"
            );
        }
        // Same extensions must not all become manifests.
        assert_eq!(category_of("notes.txt"), 3);
        assert_eq!(category_of("app.kts"), 1);
        assert_eq!(category_of("App.swift"), 1);
        assert_eq!(category_of("app.exs"), 1);
        assert_eq!(category_of("other.xml"), 1);
        // Nested manifests and READMEs are not category 0.
        assert_eq!(category_of("examples/demo/pyproject.toml"), 3);
        assert_eq!(category_of("packages/web/package.json"), 1);
        assert_eq!(category_of("docs/README.md"), 3);
        // Nested READMEs outside docs-like dirs rank with their directory.
        assert_eq!(category_of("apps/web/README.md"), 1);
        assert_eq!(category_of("apps/web/package.json"), 1);
        assert_eq!(category_of("apps/web/index.ts"), 1);
        assert_eq!(category_of("README.md"), 0);
    }

    fn collected_rels(
        base: &Path,
        filters: &[String],
        include_lockfiles: bool,
    ) -> (Vec<String>, usize) {
        let collected =
            collect_files_reporting(base, filters, &[], &[], false, include_lockfiles).unwrap();
        let rels = collected
            .files
            .iter()
            .map(|entry| {
                entry
                    .path()
                    .strip_prefix(base)
                    .unwrap_or(entry.path())
                    .to_string_lossy()
                    .replace('\\', "/")
            })
            .collect();
        (rels, collected.skipped_lockfiles)
    }

    #[test]
    fn lockfiles_are_skipped_unless_opted_in() {
        let dir = tempdir().unwrap();
        let base = dir.path();
        fs::create_dir_all(base.join("src")).unwrap();
        fs::write(base.join("Cargo.toml"), "[package]\nname = \"t\"\n").unwrap();
        fs::write(base.join("Cargo.lock"), "# lock\n").unwrap();
        fs::write(base.join("uv.lock"), "# lock\n").unwrap();
        fs::write(base.join("src/lib.rs"), "pub fn f() {}\n").unwrap();

        let (skipped, n) = collected_rels(base, &[], false);
        assert_eq!(n, 2);
        assert_eq!(
            lockfile_skip_notice(n).as_deref(),
            Some("Skipped 2 lockfiles (use --include-lockfiles to include)")
        );
        assert_eq!(
            lockfile_skip_notice(1).as_deref(),
            Some("Skipped 1 lockfile (use --include-lockfiles to include)")
        );
        assert!(lockfile_skip_notice(0).is_none());
        assert!(!skipped.iter().any(|path| path.ends_with(".lock")));
        assert!(skipped.iter().any(|path| path == "Cargo.toml"));

        let (kept, n_kept) = collected_rels(base, &[], true);
        assert_eq!(n_kept, 0);
        assert!(kept.iter().any(|path| path == "Cargo.lock"));
        assert!(kept.iter().any(|path| path == "uv.lock"));
        // Included lockfiles stay after source.
        let lock_at = kept.iter().position(|path| path == "Cargo.lock").unwrap();
        let src_at = kept.iter().position(|path| path == "src/lib.rs").unwrap();
        assert!(src_at < lock_at);
    }

    #[test]
    fn type_filter_does_not_include_lockfiles() {
        let dir = tempdir().unwrap();
        let base = dir.path();
        fs::write(base.join("Cargo.toml"), "[package]\nname = \"t\"\n").unwrap();
        fs::write(base.join("Cargo.lock"), "# lock\n").unwrap();
        fs::write(base.join("other.lock"), "# not a known lockfile\n").unwrap();

        let toml = vec!["toml".to_string()];
        let (without, skipped) = collected_rels(base, &toml, false);
        assert!(without.iter().any(|path| path == "Cargo.toml"));
        assert!(!without.iter().any(|path| path == "Cargo.lock"));
        assert_eq!(skipped, 1);

        let (with_flag, skipped_flag) = collected_rels(base, &toml, true);
        assert!(with_flag.iter().any(|path| path == "Cargo.lock"));
        assert_eq!(skipped_flag, 0);

        let lock_filter = vec!["lock".to_string()];
        let (lock_only, lock_skipped) = collected_rels(base, &lock_filter, false);
        assert!(!lock_only.iter().any(|path| path == "Cargo.lock"));
        assert!(lock_only.iter().any(|path| path == "other.lock"));
        assert_eq!(lock_skipped, 1);

        let (lock_included, _) = collected_rels(base, &lock_filter, true);
        assert!(lock_included.iter().any(|path| path == "Cargo.lock"));
        assert!(lock_included.iter().any(|path| path == "other.lock"));
    }

    #[test]
    fn root_readme_precedes_changelog_and_changelog_is_docs() {
        for name in [
            "CHANGELOG.md",
            "CHANGELOG",
            "HISTORY.md",
            "HISTORY",
            "CHANGES.md",
            "NEWS.md",
        ] {
            assert_eq!(category_of(name), 3, "{name}");
            assert!(
                HISTORY_DOC_NAMES.contains(&name),
                "{name} should stay a history doc, not a manifest"
            );
        }
        assert_eq!(category_of("README.md"), 0);
        assert_eq!(category_of("packages/web/CHANGELOG.md"), 3);

        let dir = tempdir().unwrap();
        let base = dir.path();
        fs::write(base.join("README.md"), "# Hi\n").unwrap();
        fs::write(base.join("CHANGELOG.md"), "# Changes\n").unwrap();
        fs::write(base.join("Cargo.toml"), "[package]\nname = \"t\"\n").unwrap();
        fs::create_dir_all(base.join("src")).unwrap();
        fs::write(base.join("src/lib.rs"), "pub fn f() {}\n").unwrap();

        let (order, _) = collected_rels(base, &[], false);
        assert_eq!(
            order,
            vec![
                "README.md".to_string(),
                "Cargo.toml".to_string(),
                "src/lib.rs".to_string(),
                "CHANGELOG.md".to_string(),
            ]
        );
    }

    #[test]
    fn nested_package_json_is_grouped_with_its_directory() {
        let dir = tempdir().unwrap();
        let base = dir.path();
        fs::create_dir_all(base.join("packages/foo")).unwrap();
        fs::create_dir_all(base.join("packages/foo/src")).unwrap();
        fs::write(base.join("packages/foo/package.json"), "{}\n").unwrap();
        fs::write(base.join("packages/foo/index.ts"), "export {}\n").unwrap();
        fs::write(base.join("packages/foo/src/lib.ts"), "export {}\n").unwrap();
        fs::write(base.join("README.md"), "# Hi\n").unwrap();

        assert_eq!(category_of("packages/foo/package.json"), 1);
        assert_ne!(category_of("packages/foo/package.json"), 0);

        let (order, _) = collected_rels(base, &[], false);
        let pkg = order
            .iter()
            .position(|p| p == "packages/foo/package.json")
            .unwrap();
        let index = order
            .iter()
            .position(|p| p == "packages/foo/index.ts")
            .unwrap();
        let nested = order
            .iter()
            .position(|p| p == "packages/foo/src/lib.ts")
            .unwrap();
        assert!(pkg < index, "manifest sorts ahead of sibling files");
        assert!(
            index < nested,
            "a directory's files stay ahead of its subdirectories"
        );
        assert_eq!(order[0], "README.md");
    }

    #[test]
    fn pnpm_monorepo_leads_with_root_readme_and_manifest() {
        let dir = tempdir().unwrap();
        let base = dir.path();
        fs::create_dir_all(base.join("app")).unwrap();
        fs::create_dir_all(base.join("packages/web/src")).unwrap();
        fs::create_dir_all(base.join("packages/api")).unwrap();
        fs::write(base.join("README.md"), "# App\n").unwrap();
        fs::write(base.join("package.json"), "{}\n").unwrap();
        fs::write(base.join("Cargo.toml"), "[package]\nname = \"t\"\n").unwrap();
        fs::write(base.join("CHANGELOG.md"), "# Log\n").unwrap();
        fs::write(base.join("pnpm-lock.yaml"), "lock\n").unwrap();
        fs::write(base.join("app/main.ts"), "export {}\n").unwrap();
        fs::write(base.join("packages/web/package.json"), "{}\n").unwrap();
        fs::write(base.join("packages/web/CHANGELOG.md"), "# Web\n").unwrap();
        fs::write(base.join("packages/web/src/index.ts"), "export {}\n").unwrap();
        fs::write(base.join("packages/api/package.json"), "{}\n").unwrap();
        fs::write(base.join("packages/api/README.md"), "# API\n").unwrap();

        let expected = vec![
            "README.md",
            "Cargo.toml",
            "package.json",
            "app/main.ts",
            "packages/api/README.md",
            "packages/api/package.json",
            "packages/web/package.json",
            "packages/web/src/index.ts",
            "CHANGELOG.md",
            "packages/web/CHANGELOG.md",
        ];

        let (first, skipped) = collected_rels(base, &[], false);
        let (second, _) = collected_rels(base, &[], false);
        assert_eq!(first, second, "ranking is deterministic");
        assert_eq!(skipped, 1);
        assert_eq!(
            first,
            expected.iter().map(|s| s.to_string()).collect::<Vec<_>>()
        );
        assert!(!first.iter().any(|path| path == "pnpm-lock.yaml"));
        assert_eq!(category_of("packages/web/package.json"), 1);
        assert_eq!(category_of("CHANGELOG.md"), 3);
    }

    #[test]
    fn hidden_includes_cargo_config() {
        let dir = tempdir().unwrap();
        let base = dir.path();
        fs::create_dir_all(base.join(".cargo")).unwrap();
        fs::write(base.join(".cargo/config.toml"), "[build]\n").unwrap();
        fs::write(base.join("main.rs"), "fn main() {}").unwrap();

        let hidden = to_rel_paths(collect_files_ext(base, &[], &[], &[], true).unwrap(), base);
        assert!(
            hidden.contains(&".cargo/config.toml".to_string()),
            "{hidden:?}"
        );
        let visible = to_rel_paths(collect_files(base, &[], &[], &[]).unwrap(), base);
        assert!(!visible.iter().any(|p| p.starts_with(".cargo")));
    }

    #[test]
    fn hidden_walk_prunes_vcs_metadata_and_cachedir_tag_together() {
        // `filter_entry` replaces earlier filters, so both predicates must live
        // in the single closure: --hidden must not re-admit a tagged cache dir,
        // and the tag skip must not re-admit `.git`.
        let dir = tempdir().unwrap();
        let base = dir.path();
        fs::create_dir_all(base.join(".git/objects")).unwrap();
        fs::write(base.join(".git/config"), "[core]\n").unwrap();
        fs::create_dir_all(base.join(".github")).unwrap();
        fs::write(base.join(".github/ci.yml"), "name: ci\n").unwrap();
        fs::create_dir_all(base.join("pkg/target")).unwrap();
        fs::write(
            base.join("pkg/target/CACHEDIR.TAG"),
            "Signature: 8a477f597d28d172789f06886806bc55\n",
        )
        .unwrap();
        fs::write(base.join("pkg/target/out.txt"), "artifact").unwrap();
        fs::write(base.join("keep.txt"), "keep").unwrap();

        let rel = to_rel_paths(collect_files_ext(base, &[], &[], &[], true).unwrap(), base);
        assert!(rel.contains(&"keep.txt".to_string()), "{rel:?}");
        assert!(rel.contains(&".github/ci.yml".to_string()), "{rel:?}");
        assert!(!rel.iter().any(|p| p.starts_with(".git/")), "{rel:?}");
        assert!(!rel.iter().any(|p| p.contains("pkg/target")), "{rel:?}");
    }
}
