//! Default content policy: skip bulky assets, oversized files, and likely secrets.
//!
//! This module is intentionally separate from directory walking
//! ([`crate::file_utils`]) and from binary sniffing in [`crate::markdown`]
//! (NUL bytes and encoding detection). It only looks at file names, sizes, and
//! — for `.npmrc` / `.pypirc` — whether a credential assignment is present.
//! File contents are never included in warnings or in the skipped-file list.
//!
//! # What is skipped
//!
//! * **Assets** — extensions in [`ASSET_EXTENSIONS`] (images including SVG,
//!   fonts, audio, video, archives, PDFs and office documents, design files
//!   such as `.ai` / `.psd`, compiled objects, wasm, model weights, and source
//!   maps) plus minified bundles (`*.min.js`, `*.min.mjs`, `*.min.cjs`,
//!   `*.min.css`).
//! * **Too large** — files bigger than [`DEFAULT_MAX_FILE_SIZE_BYTES`]
//!   (256 KiB). `--max-file-size 0` disables the limit.
//! * **Secrets** — private-key filenames (`id_rsa`, `id_ed25519`, and the
//!   other names in [`PRIVATE_KEY_BASENAMES`]), `*.pem`, `*.key`, `*.p12`,
//!   `*.pfx`, `*.ppk`, `credentials*.json`, `.env` and `.env.*` (except
//!   `.env.example` and `.env.sample`), and `.npmrc` / `.pypirc` when they
//!   contain a token. A value that is wholly an environment reference
//!   (`${VAR}`, `$VAR`, `%VAR%`) is a placeholder, not a token.
//!
//! # Escape hatches
//!
//! * An explicit `--filter` / config `filter` token equal to an excluded
//!   **extension** includes that extension (`--filter svg`, `--filter pem`).
//!   `--filter` is still an allow-list: `--filter svg` returns SVG files
//!   rather than "everything, plus SVG". Minified bundles are included when
//!   the filter names their final extension (`js`, `css`, …) or the compound
//!   suffix (`min.js`). The size limit still applies to those files.
//! * `--max-file-size 0` (or any `0` with an optional `K`/`M`/`G` suffix)
//!   disables the size limit. A larger value such as `1M` raises it. A file
//!   is skipped only when it is **strictly larger** than the limit.
//! * `--include-secrets` includes likely-secret files the walk already
//!   collected. Naming a secret extension in `--filter` (`pem`, `key`, `p12`,
//!   `pfx`, `ppk`) does the same for that extension. Name-only secrets
//!   (`id_rsa`, `credentials.json`, `.env`) are not extensions; use
//!   `--include-secrets`. Dotfile secrets are still hidden unless `--hidden`
//!   is set — `--hidden` alone does **not** include secrets.
//!
//! `--hidden` itself is applied by the walker, not here. See that flag's
//! `--help` text for exactly which hidden paths it adds.

use ignore::DirEntry;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;

/// Default maximum file size: 256 KiB. The `K`/`M`/`G` suffixes are powers of 1024.
pub const DEFAULT_MAX_FILE_SIZE_BYTES: u64 = 256 * 1024;

/// CLI/config spelling of [`DEFAULT_MAX_FILE_SIZE_BYTES`].
pub const DEFAULT_MAX_FILE_SIZE_SPEC: &str = "256K";

/// How many leading bytes of `.npmrc` / `.pypirc` are scanned for a token.
const CREDENTIAL_SCAN_BYTES: usize = 64 * 1024;

/// Extensions treated as non-source assets. Lowercase, no leading dot.
///
/// The slice is sorted so lookups can binary-search. Categories:
///
/// * images — `png` `jpg` `jpeg` `gif` `webp` `bmp` `ico` `svg` `avif` …
/// * design / print — `ai` `psd` `eps` `indd` `sketch` `fig` `xd` `pdf`
/// * fonts — `ttf` `otf` `woff` `woff2` `eot`
/// * audio / video — `mp3` `wav` `flac` `mp4` `mov` `webm` …
/// * archives — `zip` `tar` `gz` `7z` `rar` `dmg` `iso` …
/// * office — `doc` `docx` `xls` `xlsx` `ppt` `pptx` `odt` `rtf` …
/// * compiled objects — `o` `obj` `a` `so` `dll` `dylib` `exe` `class` `pyc` …
/// * wasm — `wasm`
/// * model weights — `pt` `pth` `onnx` `safetensors` `gguf` `ckpt` `npy` …
/// * source maps — `map`
/// * 3D / native blobs that dominate context the same way — `glb` `gltf` `fbx`
///
/// Minified bundles are not extensions; see [`minified_suffix`].
pub const ASSET_EXTENSIONS: &[&str] = &[
    "7z",
    "a",
    "aac",
    "ai",
    "aif",
    "aiff",
    "avi",
    "avif",
    "bin",
    "bmp",
    "bz2",
    "cab",
    "ckpt",
    "class",
    "dll",
    "dmg",
    "doc",
    "docx",
    "dylib",
    "eot",
    "eps",
    "exe",
    "exp",
    "fbx",
    "fig",
    "flac",
    "flv",
    "ggml",
    "gguf",
    "gif",
    "glb",
    "gltf",
    "gz",
    "h5",
    "hdf5",
    "heic",
    "heif",
    "icns",
    "ico",
    "ilk",
    "indd",
    "iso",
    "jar",
    "jfif",
    "joblib",
    "jp2",
    "jpe",
    "jpeg",
    "jpg",
    "lib",
    "lz",
    "lzma",
    "m4a",
    "m4v",
    "map",
    "mid",
    "midi",
    "mkv",
    "mov",
    "mp3",
    "mp4",
    "mpeg",
    "mpg",
    "node",
    "npy",
    "npz",
    "o",
    "obj",
    "odp",
    "ods",
    "odt",
    "oga",
    "ogg",
    "ogv",
    "onnx",
    "opus",
    "otf",
    "pb",
    "pdb",
    "pdf",
    "pickle",
    "pkl",
    "png",
    "ppt",
    "pptx",
    "psd",
    "pt",
    "pth",
    "pyc",
    "pyo",
    "rar",
    "rlib",
    "rtf",
    "safetensors",
    "sketch",
    "so",
    "stl",
    "svg",
    "tar",
    "tflite",
    "tgz",
    "tif",
    "tiff",
    "ttc",
    "ttf",
    "tzst",
    "usdz",
    "war",
    "wasm",
    "wav",
    "webm",
    "webp",
    "wma",
    "wmv",
    "woff",
    "woff2",
    "xd",
    "xls",
    "xlsx",
    "xz",
    "zip",
    "zst",
];

/// Private-key basenames skipped even when they have no extension.
/// `id_rsa.pub` is not in this list and is not skipped.
pub const PRIVATE_KEY_BASENAMES: &[&str] = &[
    "id_dsa",
    "id_ecdsa",
    "id_ecdsa_sk",
    "id_ed25519",
    "id_ed25519_sk",
    "id_rsa",
];

/// Secret extensions. Naming one of these in `--filter` includes those files.
const SECRET_EXTENSIONS: &[&str] = &["key", "p12", "pem", "pfx", "ppk"];

/// Why a file was left out of the document.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SkipReason {
    /// Known binary, media, or generated-asset type.
    Asset,
    /// Larger than the active `--max-file-size` limit.
    TooLarge,
    /// Filename (or `.npmrc` / `.pypirc` contents) looks like a credential.
    Secret,
}

impl SkipReason {
    /// Stable label written into the `## Skipped` section.
    pub fn label(self) -> &'static str {
        match self {
            SkipReason::Asset => "asset",
            SkipReason::TooLarge => "too large",
            SkipReason::Secret => "secret",
        }
    }
}

/// One file omitted by this policy. `detail` is a fixed category string for
/// stderr (for example `private key`); it is never file contents.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SkippedFile {
    /// Path relative to the project root, using `/` separators.
    pub path: String,
    pub reason: SkipReason,
    pub detail: &'static str,
}

/// Resolved knobs for one run. Built from CLI args after config merging.
#[derive(Debug, Clone)]
pub struct ContentPolicy {
    /// `None` means the size limit is disabled.
    pub max_file_size: Option<u64>,
    /// Normalized `--filter` tokens (lowercase, leading `.` / `*.` stripped).
    tokens: Vec<String>,
    pub include_secrets: bool,
}

impl ContentPolicy {
    /// `max_file_size_spec` is a size string (`256K`, `0`, `262144`). Invalid
    /// specs fall back to the default limit; callers should warn before that.
    pub fn new(max_file_size_spec: &str, filters: &[String], include_secrets: bool) -> Self {
        let max_file_size =
            parse_file_size(max_file_size_spec).unwrap_or(Some(DEFAULT_MAX_FILE_SIZE_BYTES));
        Self {
            max_file_size,
            tokens: filters.iter().map(|f| normalize_filter_token(f)).collect(),
            include_secrets,
        }
    }
}

/// Parse a file-size specification.
///
/// Accepts a non-negative integer, optionally followed by `K`/`KB`/`KiB`,
/// `M`/`MB`/`MiB`, or `G`/`GB`/`GiB` (1024-based, case-insensitive). A bare
/// number or a trailing `B` is bytes.
///
/// Returns `Ok(None)` when the value is zero (the limit is disabled) and
/// `Ok(Some(bytes))` otherwise.
pub fn parse_file_size(input: &str) -> Result<Option<u64>, String> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(
            "file size is empty; expected a number such as 256K, 1M, or 262144 (0 disables the limit)"
                .to_string(),
        );
    }
    let bytes = trimmed.as_bytes();
    let mut i = 0;
    while i < bytes.len() && bytes[i].is_ascii_digit() {
        i += 1;
    }
    if i == 0 {
        return Err(format!(
            "invalid file size '{trimmed}': expected a number such as 256K, 1M, or 262144"
        ));
    }
    let number: u64 = std::str::from_utf8(&bytes[..i])
        .unwrap_or("")
        .parse()
        .map_err(|_| format!("invalid file size '{trimmed}': number is out of range"))?;
    let suffix = std::str::from_utf8(&bytes[i..])
        .unwrap_or("")
        .trim()
        .to_ascii_lowercase();
    let mult: u64 = match suffix.as_str() {
        "" | "b" => 1,
        "k" | "kb" | "ki" | "kib" => 1024,
        "m" | "mb" | "mi" | "mib" => 1024 * 1024,
        "g" | "gb" | "gi" | "gib" => 1024 * 1024 * 1024,
        _ => {
            return Err(format!(
                "invalid file size '{trimmed}': unknown suffix '{suffix}' (use K, M, G, or bytes)"
            ));
        }
    };
    if number == 0 {
        return Ok(None);
    }
    match number.checked_mul(mult) {
        Some(n) => Ok(Some(n)),
        None => Err(format!("invalid file size '{trimmed}': value overflows")),
    }
}

/// Clap `value_parser`: accept the size and keep the user's spelling.
pub fn parse_max_file_size_arg(input: &str) -> Result<String, String> {
    parse_file_size(input)?;
    Ok(input.trim().to_string())
}

/// Strip `*.` / a single leading `.` and lowercase. `.svg` and `*.svg` become
/// `svg`; `.min.js` becomes `min.js`; `id_rsa` is unchanged.
pub fn normalize_filter_token(raw: &str) -> String {
    let mut token = raw.trim();
    if let Some(rest) = token.strip_prefix("*.") {
        token = rest;
    }
    if let Some(rest) = token.strip_prefix('.') {
        token = rest;
    }
    token.to_ascii_lowercase()
}

/// Split collected files into those to render and those to report as skipped.
///
/// Skipped entries are sorted by relative path so the report is deterministic.
pub fn partition(
    entries: Vec<DirEntry>,
    base: &Path,
    policy: &ContentPolicy,
) -> (Vec<DirEntry>, Vec<SkippedFile>) {
    let mut kept = Vec::with_capacity(entries.len());
    let mut skipped = Vec::new();
    for entry in entries {
        let path = entry.path();
        let size = path.metadata().map(|meta| meta.len()).unwrap_or(0);
        if let Some((reason, detail)) = classify(path, size, policy) {
            skipped.push(SkippedFile {
                path: relative_posix(path, base),
                reason,
                detail,
            });
        } else {
            kept.push(entry);
        }
    }
    skipped.sort_by(|a, b| a.path.cmp(&b.path));
    (kept, skipped)
}

/// Decide whether `path` should be skipped. `size` is the file length in bytes.
///
/// Secrets win over asset and size (a large `id_rsa` is reported as a secret).
/// Assets win over size (a large PNG is reported as an asset) unless the
/// extension was explicitly requested, in which case only the size limit remains.
pub fn classify(
    path: &Path,
    size: u64,
    policy: &ContentPolicy,
) -> Option<(SkipReason, &'static str)> {
    if let Some(detail) = secret_detail(path)
        && !policy.include_secrets
        && !secret_explicitly_included(path, &policy.tokens)
    {
        return Some((SkipReason::Secret, detail));
    }
    let name = file_name_lower(path);
    let ext = extension_lower(path);
    if is_asset_name(&name, &ext) && !asset_explicitly_included(&name, &ext, &policy.tokens) {
        return Some((SkipReason::Asset, "asset"));
    }
    if let Some(limit) = policy.max_file_size
        && size > limit
    {
        return Some((SkipReason::TooLarge, "too large"));
    }
    None
}

/// Stderr lines for a skip list: one warning per secret (path and category
/// only), then a single summary line. Empty when nothing was skipped.
pub fn report_lines(skipped: &[SkippedFile]) -> Vec<String> {
    let mut lines = Vec::new();
    for item in skipped
        .iter()
        .filter(|item| item.reason == SkipReason::Secret)
    {
        lines.push(format!(
            "⚠️  Skipping likely secret `{}` ({}).",
            item.path, item.detail
        ));
    }
    if let Some(summary) = summary_line(skipped) {
        lines.push(summary);
    }
    lines
}

/// Print [`report_lines`] to stderr. Suppressed when `silent` is set (`CB_SILENT`).
pub fn report_skips(skipped: &[SkippedFile], silent: bool) {
    if silent {
        return;
    }
    for line in report_lines(skipped) {
        errln!("{line}");
    }
}

/// `## Skipped` section. Writes nothing when `skipped` is empty.
pub fn write_skipped_section(out: &mut dyn Write, skipped: &[SkippedFile]) -> io::Result<()> {
    if skipped.is_empty() {
        return Ok(());
    }
    writeln!(out, "## Skipped\n")?;
    for item in skipped {
        writeln!(
            out,
            "- {} — {}",
            crate::fences::inline_code(&item.path),
            item.reason.label()
        )?;
    }
    writeln!(out)?;
    Ok(())
}

/// One-line count, e.g. `Skipped 3 files (1 asset, 1 too large, 1 secret).`
pub fn summary_line(skipped: &[SkippedFile]) -> Option<String> {
    if skipped.is_empty() {
        return None;
    }
    let mut assets = 0usize;
    let mut large = 0usize;
    let mut secrets = 0usize;
    for item in skipped {
        match item.reason {
            SkipReason::Asset => assets += 1,
            SkipReason::TooLarge => large += 1,
            SkipReason::Secret => secrets += 1,
        }
    }
    let mut parts = Vec::new();
    if assets > 0 {
        parts.push(count_phrase(assets, "asset", "assets"));
    }
    if large > 0 {
        parts.push(count_phrase(large, "too large", "too large"));
    }
    if secrets > 0 {
        parts.push(count_phrase(secrets, "secret", "secrets"));
    }
    let noun = if skipped.len() == 1 { "file" } else { "files" };
    Some(format!(
        "Skipped {} {} ({}).",
        skipped.len(),
        noun,
        parts.join(", ")
    ))
}

fn count_phrase(n: usize, singular: &str, plural: &str) -> String {
    if n == 1 {
        format!("1 {singular}")
    } else {
        format!("{n} {plural}")
    }
}

fn relative_posix(path: &Path, base: &Path) -> String {
    path.strip_prefix(base)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

fn file_name_lower(path: &Path) -> String {
    path.file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("")
        .to_ascii_lowercase()
}

fn extension_lower(path: &Path) -> String {
    path.extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("")
        .to_ascii_lowercase()
}

fn is_asset_extension(ext: &str) -> bool {
    ASSET_EXTENSIONS.binary_search(&ext).is_ok()
}

fn minified_suffix(file_name: &str) -> Option<&'static str> {
    const SUFFIXES: &[&str] = &[".min.cjs", ".min.css", ".min.js", ".min.mjs"];
    SUFFIXES
        .iter()
        .find(|suffix| file_name.ends_with(*suffix))
        .map(|suffix| suffix.trim_start_matches('.'))
}

fn is_asset_name(file_name: &str, ext: &str) -> bool {
    minified_suffix(file_name).is_some() || is_asset_extension(ext)
}

fn asset_explicitly_included(file_name: &str, ext: &str, tokens: &[String]) -> bool {
    if tokens.is_empty() {
        return false;
    }
    if let Some(suffix) = minified_suffix(file_name) {
        // `--filter js` names the extension; `--filter min.js` names the bundle.
        return tokens.iter().any(|token| token == ext || token == suffix);
    }
    !ext.is_empty() && tokens.iter().any(|token| token == ext)
}

fn secret_explicitly_included(path: &Path, tokens: &[String]) -> bool {
    if tokens.is_empty() {
        return false;
    }
    let name = file_name_lower(path);
    let ext = extension_lower(path);
    if tokens
        .iter()
        .any(|token| token == &name || format!(".{token}") == name)
    {
        return true;
    }
    if SECRET_EXTENSIONS.binary_search(&ext.as_str()).is_ok()
        && tokens.iter().any(|token| token == &ext)
    {
        return true;
    }
    // `--filter env` names the dotenv family (`.env`, `.env.local`, …).
    is_dotenv_name(&name) && tokens.iter().any(|token| token == "env")
}

fn secret_detail(path: &Path) -> Option<&'static str> {
    let name = file_name_lower(path);
    let ext = extension_lower(path);

    if is_dotenv_name(&name) {
        return Some("environment file");
    }
    if is_private_key_name(&name, &ext) {
        return Some("private key");
    }
    if name.starts_with("credentials") && name.ends_with(".json") {
        return Some("credentials file");
    }
    match ext.as_str() {
        "pem" => return Some("PEM file"),
        "key" => return Some("key file"),
        "p12" | "pfx" => return Some("PKCS#12 file"),
        "ppk" => return Some("PuTTY private key"),
        _ => {}
    }
    if name == ".npmrc" {
        return match credential_file_has_secret(path, &["_authtoken", "_password", "_auth"]) {
            Ok(true) => Some("npm credentials"),
            Ok(false) => None,
            Err(_) => Some("unreadable credentials file"),
        };
    }
    if name == ".pypirc" {
        return match credential_file_has_secret(path, &["password", "token"]) {
            Ok(true) => Some("PyPI credentials"),
            Ok(false) => None,
            Err(_) => Some("unreadable credentials file"),
        };
    }
    None
}

/// `.env` and `.env.*`, but not the template names `.env.example` / `.env.sample`.
fn is_dotenv_name(name: &str) -> bool {
    if name == ".env.example" || name == ".env.sample" {
        return false;
    }
    name == ".env" || name.starts_with(".env.")
}

fn is_private_key_name(name: &str, ext: &str) -> bool {
    if PRIVATE_KEY_BASENAMES.binary_search(&name).is_ok() {
        return true;
    }
    // `id_rsa.pub` is a public key. Any other extra suffix (`id_rsa.bak`) still matches.
    if ext == "pub" {
        return false;
    }
    let stem = name.rsplit_once('.').map(|(stem, _)| stem).unwrap_or(name);
    PRIVATE_KEY_BASENAMES.binary_search(&stem).is_ok()
}

/// True when a non-placeholder credential is assigned. The scanned text is
/// never returned to the caller. [`is_placeholder`] treats a value that is
/// wholly an environment reference (`${VAR}`, `$VAR`, `%VAR%`) as a placeholder.
fn credential_file_has_secret(path: &Path, keys: &[&str]) -> io::Result<bool> {
    let mut file = File::open(path)?;
    let mut buf = vec![0u8; CREDENTIAL_SCAN_BYTES];
    let n = file.read(&mut buf)?;
    let text = String::from_utf8_lossy(&buf[..n]);
    Ok(text_has_credential(&text, keys))
}

/// True when any assignment of `keys` has a non-empty value that
/// [`is_placeholder`] does not accept. A value that is wholly `${VAR}`,
/// `$VAR`, or `%VAR%` is a placeholder.
fn text_has_credential(text: &str, keys: &[&str]) -> bool {
    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') || trimmed.starts_with(';') {
            continue;
        }
        let lower = trimmed.to_ascii_lowercase();
        for key in keys {
            let key = key.trim();
            let mut from = 0;
            while let Some(idx) = find_key_from(&lower, key, from) {
                let end = idx + key.len();
                from = end;
                // `token` inside `${PYPI_TOKEN}` is the reference, not an assignment.
                if key_inside_env_reference(trimmed, idx, end) {
                    continue;
                }
                let value = credential_value(&trimmed[end..]);
                if !value.is_empty() && !is_placeholder(value) {
                    return true;
                }
            }
        }
    }
    false
}

fn find_key_from(haystack: &str, key: &str, mut start: usize) -> Option<usize> {
    while let Some(rel) = haystack[start..].find(key) {
        let idx = start + rel;
        let before_ok = idx == 0 || !haystack.as_bytes()[idx - 1].is_ascii_alphanumeric();
        let end = idx + key.len();
        let after_ok = end >= haystack.len() || !haystack.as_bytes()[end].is_ascii_alphanumeric();
        if before_ok && after_ok {
            return Some(idx);
        }
        start = idx + key.len();
    }
    None
}

fn credential_value(after_key: &str) -> &str {
    let after = after_key.trim_start();
    let after = after.trim_start_matches(['=', ':']).trim();
    let token = after.split_whitespace().next().unwrap_or("");
    token.trim_matches(|c| c == '"' || c == '\'' || c == ',')
}

fn is_placeholder(value: &str) -> bool {
    if is_env_reference(value) {
        return true;
    }
    matches!(
        value.to_ascii_lowercase().as_str(),
        "" | "changeme"
            | "change-me"
            | "change_me"
            | "your-token"
            | "your_token"
            | "your-password"
            | "your_password"
            | "your_api_token"
            | "<token>"
            | "<password>"
            | "todo"
            | "xxx"
            | "***"
            | "insert-token"
            | "replace-me"
            | "replace_me"
            | "password"
            | "token"
            | "secret"
            | "none"
            | "null"
            | "undefined"
            | "example"
            | "sample"
    )
}

/// True when `value` is entirely `${VAR}`, `$VAR`, or `%VAR%`.
///
/// `VAR` is an ASCII identifier (letter or `_`, then letters, digits, or `_`).
/// A value that only contains a reference (`prefix${VAR}`, `${VAR}suffix`) is
/// not a placeholder.
fn is_env_reference(value: &str) -> bool {
    if let Some(name) = value
        .strip_prefix("${")
        .and_then(|rest| rest.strip_suffix('}'))
    {
        return is_env_name(name);
    }
    if let Some(name) = value.strip_prefix('$') {
        return is_env_name(name);
    }
    if let Some(name) = value
        .strip_prefix('%')
        .and_then(|rest| rest.strip_suffix('%'))
    {
        return is_env_name(name);
    }
    false
}

fn is_env_name(name: &str) -> bool {
    let mut chars = name.chars();
    match chars.next() {
        Some(c) if c.is_ascii_alphabetic() || c == '_' => {}
        _ => return false,
    }
    chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

/// True when the key span `[start, end)` lies inside `${VAR}`, `$VAR`, or `%VAR%`.
fn key_inside_env_reference(line: &str, start: usize, end: usize) -> bool {
    let mut rest = line;
    let mut offset = 0;
    while !rest.is_empty() {
        if let Some(len) = env_reference_len(rest) {
            if start >= offset && end <= offset + len {
                return true;
            }
            offset += len;
            rest = &rest[len..];
        } else {
            let len = rest.chars().next().map(|c| c.len_utf8()).unwrap_or(1);
            offset += len;
            rest = &rest[len..];
        }
    }
    false
}

/// Byte length of an env reference at the start of `s`, if one is there.
fn env_reference_len(s: &str) -> Option<usize> {
    let bytes = s.as_bytes();
    if bytes.first() == Some(&b'$') && bytes.get(1) == Some(&b'{') {
        let close = s[2..].find('}')?;
        if is_env_name(&s[2..2 + close]) {
            return Some(2 + close + 1);
        }
        return None;
    }
    if bytes.first() == Some(&b'$') {
        let name_len = env_name_prefix_len(&s[1..]);
        if name_len > 0 {
            return Some(1 + name_len);
        }
        return None;
    }
    if bytes.first() == Some(&b'%') {
        let name_len = env_name_prefix_len(&s[1..]);
        if name_len > 0 && bytes.get(1 + name_len) == Some(&b'%') {
            return Some(name_len + 2);
        }
    }
    None
}

fn env_name_prefix_len(s: &str) -> usize {
    let mut len = 0;
    for c in s.chars() {
        let ok = if len == 0 {
            c.is_ascii_alphabetic() || c == '_'
        } else {
            c.is_ascii_alphanumeric() || c == '_'
        };
        if !ok {
            break;
        }
        len += c.len_utf8();
    }
    len
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    fn policy(filters: &[&str], limit: Option<u64>, include_secrets: bool) -> ContentPolicy {
        ContentPolicy {
            max_file_size: limit,
            tokens: filters.iter().map(|f| normalize_filter_token(f)).collect(),
            include_secrets,
        }
    }

    fn reason(path: &str, size: u64, policy: &ContentPolicy) -> Option<SkipReason> {
        classify(Path::new(path), size, policy).map(|(reason, _)| reason)
    }

    #[test]
    fn asset_extensions_are_sorted_and_unique() {
        let mut sorted = ASSET_EXTENSIONS.to_vec();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(ASSET_EXTENSIONS, sorted.as_slice());
    }

    #[test]
    fn private_key_names_are_sorted() {
        let mut sorted = PRIVATE_KEY_BASENAMES.to_vec();
        sorted.sort_unstable();
        assert_eq!(PRIVATE_KEY_BASENAMES, sorted.as_slice());
    }

    #[test]
    fn parses_size_specifications() {
        assert_eq!(parse_file_size("256K").unwrap(), Some(256 * 1024));
        assert_eq!(parse_file_size("256KB").unwrap(), Some(256 * 1024));
        assert_eq!(parse_file_size("256KiB").unwrap(), Some(256 * 1024));
        assert_eq!(parse_file_size("1M").unwrap(), Some(1024 * 1024));
        assert_eq!(parse_file_size("1MB").unwrap(), Some(1024 * 1024));
        assert_eq!(parse_file_size("262144").unwrap(), Some(262144));
        assert_eq!(parse_file_size("10B").unwrap(), Some(10));
        assert_eq!(parse_file_size("0").unwrap(), None);
        assert_eq!(parse_file_size("0K").unwrap(), None);
        assert_eq!(parse_file_size("  1G ").unwrap(), Some(1024 * 1024 * 1024));
        assert!(parse_file_size("abc").is_err());
        assert!(parse_file_size("12XB").is_err());
        assert!(parse_file_size("").is_err());
    }

    #[test]
    fn assets_are_skipped_unless_the_extension_is_filtered() {
        let bare = policy(&[], Some(DEFAULT_MAX_FILE_SIZE_BYTES), false);
        for path in [
            "logo.svg",
            "logo.SVG",
            "a/logo.png",
            "font.woff2",
            "song.mp3",
            "clip.mp4",
            "data.zip",
            "doc.pdf",
            "art.ai",
            "design.psd",
            "obj.o",
            "mod.wasm",
            "model.safetensors",
            "app.min.js",
            "app.min.css",
            "app.js.map",
            "bundle.min.mjs",
        ] {
            assert_eq!(
                reason(path, 10, &bare),
                Some(SkipReason::Asset),
                "{path} should be an asset"
            );
        }
        assert_eq!(reason("src/main.rs", 10, &bare), None);

        let svg = policy(&["svg"], Some(DEFAULT_MAX_FILE_SIZE_BYTES), false);
        assert_eq!(reason("logo.svg", 10, &svg), None);
        let dotted = policy(&[".png", "*.pdf"], Some(DEFAULT_MAX_FILE_SIZE_BYTES), false);
        assert_eq!(reason("logo.png", 10, &dotted), None);
        assert_eq!(reason("doc.pdf", 10, &dotted), None);
        // A filter for one asset does not bring the others back.
        assert_eq!(reason("logo.svg", 10, &dotted), Some(SkipReason::Asset));

        let js = policy(&["js"], Some(DEFAULT_MAX_FILE_SIZE_BYTES), false);
        assert_eq!(reason("app.min.js", 10, &js), None);
        assert_eq!(reason("app.js.map", 10, &js), Some(SkipReason::Asset));
    }

    #[test]
    fn size_limit_skips_and_zero_disables_it() {
        let limited = policy(&[], Some(100), false);
        assert_eq!(reason("big.rs", 101, &limited), Some(SkipReason::TooLarge));
        assert_eq!(reason("exact.rs", 100, &limited), None);
        assert_eq!(reason("small.rs", 99, &limited), None);

        let unlimited = policy(&[], None, false);
        assert_eq!(reason("big.rs", u64::MAX, &unlimited), None);

        // Explicitly included assets still obey the size limit.
        let svg = policy(&["svg"], Some(100), false);
        assert_eq!(reason("logo.svg", 101, &svg), Some(SkipReason::TooLarge));
        assert_eq!(reason("logo.svg", 50, &svg), None);

        // A large secret is reported as a secret, not as too large.
        let bare = policy(&[], Some(10), false);
        assert_eq!(reason("id_rsa", 10_000, &bare), Some(SkipReason::Secret));
    }

    #[test]
    fn each_secret_pattern_is_skipped_with_a_warning_and_templates_are_kept() {
        let dir = tempdir().unwrap();
        let root = dir.path();
        let payload = "SUPER_SECRET_PAYLOAD_do_not_print";
        let files = [
            ("id_rsa", payload),
            ("id_ed25519", payload),
            ("id_dsa", payload),
            ("id_ecdsa", payload),
            ("server.pem", payload),
            ("server.key", payload),
            ("cert.p12", payload),
            ("cert.pfx", payload),
            ("putty.ppk", payload),
            ("credentials.json", payload),
            ("credentials-prod.json", payload),
            (".env", payload),
            (".env.local", payload),
            (".env.example", "EXAMPLE_NOT_SECRET"),
            (".env.sample", "SAMPLE_NOT_SECRET"),
            ("id_rsa.pub", "ssh-ed25519 AAAA public"),
            ("notes.rs", "fn main() {}"),
        ];
        for (name, body) in files {
            fs::write(root.join(name), body).unwrap();
        }
        fs::write(
            root.join(".npmrc"),
            "//registry.npmjs.org/:_authToken=npm_SUPER_SECRET_TOKEN\n",
        )
        .unwrap();
        fs::write(
            root.join(".npmrc.clean"),
            "registry=https://registry.npmjs.org/\n",
        )
        .unwrap();
        // The clean file must be named `.npmrc` to exercise the rule. Use a subdirectory.
        fs::create_dir(root.join("clean-npm")).unwrap();
        fs::write(
            root.join("clean-npm/.npmrc"),
            "registry=https://registry.npmjs.org/\n",
        )
        .unwrap();
        fs::write(
            root.join(".pypirc"),
            "[pypi]\nusername = user\npassword = pypi-secret-password\n",
        )
        .unwrap();
        fs::create_dir(root.join("clean-pypi")).unwrap();
        fs::write(
            root.join("clean-pypi/.pypirc"),
            "[pypi]\nusername = user\npassword = changeme\n",
        )
        .unwrap();

        let bare = policy(&[], Some(DEFAULT_MAX_FILE_SIZE_BYTES), false);
        let expected_secrets = [
            "id_rsa",
            "id_ed25519",
            "id_dsa",
            "id_ecdsa",
            "server.pem",
            "server.key",
            "cert.p12",
            "cert.pfx",
            "putty.ppk",
            "credentials.json",
            "credentials-prod.json",
            ".env",
            ".env.local",
            ".npmrc",
            ".pypirc",
        ];
        let mut skipped = Vec::new();
        for name in expected_secrets {
            let found = classify(&root.join(name), 20, &bare);
            assert_eq!(
                found.map(|(reason, _)| reason),
                Some(SkipReason::Secret),
                "{name} should be a secret"
            );
            let (reason, detail) = found.unwrap();
            skipped.push(SkippedFile {
                path: name.to_string(),
                reason,
                detail,
            });
        }
        assert_eq!(reason_of(&root.join(".env.example"), &bare), None);
        assert_eq!(reason_of(&root.join(".env.sample"), &bare), None);
        assert_eq!(reason_of(&root.join("id_rsa.pub"), &bare), None);
        assert_eq!(reason_of(&root.join("notes.rs"), &bare), None);
        assert_eq!(reason_of(&root.join("clean-npm/.npmrc"), &bare), None);
        assert_eq!(reason_of(&root.join("clean-pypi/.pypirc"), &bare), None);

        let lines = report_lines(&skipped);
        assert_eq!(lines.len(), expected_secrets.len() + 1);
        let joined = lines.join("\n");
        for name in expected_secrets {
            assert!(
                joined.contains(&format!("Skipping likely secret `{name}`")),
                "missing warning for {name}: {joined}"
            );
        }
        assert!(joined.contains("Skipped 15 files (15 secrets)."));
        assert!(!joined.contains(payload));
        assert!(!joined.contains("npm_SUPER_SECRET_TOKEN"));
        assert!(!joined.contains("pypi-secret-password"));

        // --include-secrets lifts every pattern. A filter of `pem` lifts only pem.
        let opted_in = policy(&[], Some(DEFAULT_MAX_FILE_SIZE_BYTES), true);
        assert_eq!(reason_of(&root.join("id_rsa"), &opted_in), None);
        assert_eq!(reason_of(&root.join(".env"), &opted_in), None);
        let pem_filter = policy(&["pem"], Some(DEFAULT_MAX_FILE_SIZE_BYTES), false);
        assert_eq!(reason_of(&root.join("server.pem"), &pem_filter), None);
        assert_eq!(
            reason_of(&root.join("id_rsa"), &pem_filter),
            Some(SkipReason::Secret)
        );
        // `--filter json` does not opt credentials.json back in.
        let json_filter = policy(&["json"], Some(DEFAULT_MAX_FILE_SIZE_BYTES), false);
        assert_eq!(
            reason_of(&root.join("credentials.json"), &json_filter),
            Some(SkipReason::Secret)
        );
    }

    fn reason_of(path: &Path, policy: &ContentPolicy) -> Option<SkipReason> {
        classify(path, 20, policy).map(|(reason, _)| reason)
    }

    #[test]
    fn env_references_are_not_credentials_in_npmrc_or_pypirc() {
        assert!(is_placeholder("${NPM_TOKEN}"));
        assert!(is_placeholder("$NPM_TOKEN"));
        assert!(is_placeholder("%NPM_TOKEN%"));
        assert!(is_placeholder("${_token}"));
        assert!(!is_placeholder("npm_${NPM_TOKEN}"));
        assert!(!is_placeholder("${NPM_TOKEN}x"));
        assert!(!is_placeholder("prefix$NPM_TOKEN"));
        assert!(!is_placeholder("$"));
        assert!(!is_placeholder("${}"));
        assert!(!is_placeholder("%NPM_TOKEN"));
        assert!(!is_placeholder("npm_SUPER_SECRET_TOKEN"));

        let npm_keys = &["_authtoken", "_password", "_auth"][..];
        assert!(!text_has_credential(
            "//registry.npmjs.org/:_authToken=${NPM_TOKEN}\n",
            npm_keys,
        ));
        assert!(!text_has_credential(
            "//registry.npmjs.org/:_authToken=$NPM_TOKEN\n//other/:_authToken=%NPM_TOKEN%\n",
            npm_keys,
        ));
        assert!(!text_has_credential(
            "//registry.npmjs.org/:_authToken=\"${NPM_TOKEN}\"\n",
            npm_keys,
        ));
        assert!(text_has_credential(
            "//registry.npmjs.org/:_authToken=npm_SUPER_SECRET_TOKEN\n",
            npm_keys,
        ));
        assert!(text_has_credential(
            "//registry.npmjs.org/:_authToken=prefix${NPM_TOKEN}\n",
            npm_keys,
        ));

        let pypi_keys = &["password", "token"][..];
        assert!(!text_has_credential(
            "[pypi]\nusername = user\npassword = ${PYPI_TOKEN}\n",
            pypi_keys,
        ));
        assert!(!text_has_credential(
            "password = $PYPI_TOKEN\ntoken = %PYPI_TOKEN%\n",
            pypi_keys,
        ));
        assert!(text_has_credential(
            "[pypi]\nusername = user\npassword = pypi-secret-password\n",
            pypi_keys,
        ));
        assert!(text_has_credential(
            "password = secret${PYPI_TOKEN}\n",
            pypi_keys,
        ));

        let dir = tempdir().unwrap();
        let root = dir.path();
        fs::create_dir(root.join("env-npm")).unwrap();
        fs::write(
            root.join("env-npm/.npmrc"),
            "//registry.npmjs.org/:_authToken=${NPM_TOKEN}\n",
        )
        .unwrap();
        fs::create_dir(root.join("env-pypi")).unwrap();
        fs::write(
            root.join("env-pypi/.pypirc"),
            "[distutils]\nindex-servers = pypi\n[pypi]\nusername = user\npassword = ${PYPI_TOKEN}\n",
        )
        .unwrap();
        fs::write(
            root.join(".npmrc"),
            "//registry.npmjs.org/:_authToken=npm_SUPER_SECRET_TOKEN\n",
        )
        .unwrap();
        fs::write(
            root.join(".pypirc"),
            "[pypi]\npassword = pypi-secret-password\n",
        )
        .unwrap();

        let bare = policy(&[], Some(DEFAULT_MAX_FILE_SIZE_BYTES), false);
        assert_eq!(reason_of(&root.join("env-npm/.npmrc"), &bare), None);
        assert_eq!(reason_of(&root.join("env-pypi/.pypirc"), &bare), None);
        assert_eq!(
            reason_of(&root.join(".npmrc"), &bare),
            Some(SkipReason::Secret)
        );
        assert_eq!(
            reason_of(&root.join(".pypirc"), &bare),
            Some(SkipReason::Secret)
        );
    }

    #[test]
    fn skipped_section_lists_path_and_reason() {
        let skipped = vec![
            SkippedFile {
                path: "logo.svg".into(),
                reason: SkipReason::Asset,
                detail: "asset",
            },
            SkippedFile {
                path: "big.rs".into(),
                reason: SkipReason::TooLarge,
                detail: "too large",
            },
            SkippedFile {
                path: "id_rsa".into(),
                reason: SkipReason::Secret,
                detail: "private key",
            },
        ];
        let mut buf = Vec::new();
        write_skipped_section(&mut buf, &skipped).unwrap();
        let text = String::from_utf8(buf).unwrap();
        assert!(text.starts_with("## Skipped\n"));
        assert!(text.contains("- `logo.svg` — asset\n"));
        assert!(text.contains("- `big.rs` — too large\n"));
        assert!(text.contains("- `id_rsa` — secret\n"));
        assert_eq!(
            summary_line(&skipped).unwrap(),
            "Skipped 3 files (1 asset, 1 too large, 1 secret)."
        );

        let mut empty = Vec::new();
        write_skipped_section(&mut empty, &[]).unwrap();
        assert!(empty.is_empty());
        assert!(summary_line(&[]).is_none());
        assert!(report_lines(&[]).is_empty());
    }

    #[test]
    fn skipped_list_uses_adaptive_inline_code_for_backticks() {
        let skipped = vec![SkippedFile {
            path: "weird`name.svg".into(),
            reason: SkipReason::Asset,
            detail: "image",
        }];
        let mut buf = Vec::new();
        write_skipped_section(&mut buf, &skipped).unwrap();
        let text = String::from_utf8(buf).unwrap();
        assert!(text.contains("- ``weird`name.svg`` — asset\n"), "{text}");
    }
}
