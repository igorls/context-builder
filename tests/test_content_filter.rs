//! Default skips for assets, oversized files, and likely secrets (v0.11).

use std::fs;
use std::path::Path;

use tempfile::tempdir;

use context_builder::cli::Args;
use context_builder::config::Config;
use context_builder::config_resolver::{ExplicitCli, resolve_final_config};
use context_builder::content_filter::{self, ContentPolicy, SkipReason};
use context_builder::file_utils::collect_files_ext;
use context_builder::{Prompter, run_with_args};

struct YesPrompter;

impl Prompter for YesPrompter {
    fn confirm_overwrite(&self, _: &str) -> std::io::Result<bool> {
        Ok(true)
    }
}

fn project(dir: &tempfile::TempDir) -> (std::path::PathBuf, std::path::PathBuf) {
    let root = dir.path().join("proj");
    let outs = dir.path().join("outs");
    fs::create_dir_all(&root).unwrap();
    fs::create_dir_all(&outs).unwrap();
    (root, outs)
}

fn args(input: &Path, output: &Path) -> Args {
    Args {
        input: input.to_string_lossy().into_owned(),
        output: output.to_string_lossy().into_owned(),
        filter: vec![],
        ignore: vec![],
        preview: false,
        token_count: false,
        line_numbers: false,
        yes: true,
        diff_only: false,
        clear_cache: false,
        encoding: "o200k_base".into(),
        init: false,
        max_tokens: None,
        signatures: false,
        structure: false,
        truncate: "smart".into(),
        visibility: "all".into(),
        max_file_size: "256K".into(),
        hidden: false,
        include_secrets: false,
        file_metadata: false,
        include_lockfiles: false,
    }
}

fn run(args: Args) -> String {
    let output = args.output.clone();
    run_with_args(args, Config::default(), &YesPrompter).expect("run succeeds");
    fs::read_to_string(output).expect("output exists")
}

#[test]
fn asset_is_skipped_and_listed_unless_its_extension_is_filtered() {
    let dir = tempdir().unwrap();
    let (root, outs) = project(&dir);
    let root = root.as_path();
    fs::write(root.join("keep.rs"), "fn main() {}\n").unwrap();
    fs::write(root.join("logo.svg"), "<svg/>").unwrap();
    fs::write(root.join("app.min.js"), "function f(){}\n").unwrap();
    fs::write(root.join("app.js.map"), "{\"version\":3}").unwrap();

    let doc = run(args(root, &outs.join("out.md")));
    assert!(doc.contains("fn main()"));
    assert!(!doc.contains("<svg/>"), "svg body must not be inlined");
    assert!(doc.contains("## Skipped"));
    assert!(doc.contains("- `logo.svg` — asset"));
    assert!(doc.contains("- `app.min.js` — asset"));
    assert!(doc.contains("- `app.js.map` — asset"));
    assert!(!doc.contains("### File: `logo.svg`"));

    // Explicit --filter svg is an allow-list that opts that extension back in.
    let mut only_svg = args(root, &outs.join("svg.md"));
    only_svg.filter = vec!["svg".into()];
    let svg_doc = run(only_svg);
    assert!(svg_doc.contains("<svg/>"));
    assert!(!svg_doc.contains("- `logo.svg` — asset"));

    // --filter js names the js extension, so a small minified bundle is included.
    // Source maps stay assets (their extension is `map`).
    let mut only_js = args(root, &outs.join("js.md"));
    only_js.filter = vec!["js".into()];
    let js_doc = run(only_js);
    assert!(js_doc.contains("function f()"));
    assert!(!js_doc.contains("- `app.min.js` — asset"));
}

#[test]
fn size_limit_skips_and_zero_or_a_larger_limit_overrides_it() {
    let dir = tempdir().unwrap();
    let (root, outs) = project(&dir);
    let root = root.as_path();
    fs::write(root.join("small.rs"), "fn small() {}\n").unwrap();
    let big = vec![b'a'; 300];
    fs::write(root.join("big.rs"), &big).unwrap();

    let mut limited = args(root, &outs.join("limited.md"));
    limited.max_file_size = "100".into();
    let doc = run(limited);
    assert!(doc.contains("fn small()"));
    assert!(doc.contains("- `big.rs` — too large"));
    assert!(!doc.contains(&String::from_utf8(big.clone()).unwrap()));

    let mut unlimited = args(root, &outs.join("unlimited.md"));
    unlimited.max_file_size = "0".into();
    let doc = run(unlimited);
    assert!(doc.contains(&String::from_utf8(big.clone()).unwrap()));
    assert!(!doc.contains("too large"));

    let mut raised = args(root, &outs.join("raised.md"));
    raised.max_file_size = "1K".into();
    let doc = run(raised);
    assert!(doc.contains(&String::from_utf8(big).unwrap()));
    assert!(!doc.contains("- `big.rs` — too large"));

    // Default limit is 256 KiB: one byte over is skipped, the exact limit is kept.
    let exact = vec![b'b'; 256 * 1024];
    fs::write(root.join("exact.txt"), &exact).unwrap();
    fs::write(root.join("over.txt"), vec![b'c'; 256 * 1024 + 1]).unwrap();
    let doc = run(args(root, &outs.join("default.md")));
    assert!(doc.contains("- `over.txt` — too large"));
    assert!(!doc.contains("- `exact.txt` — too large"));
    assert!(doc.contains("### File: `exact.txt`"));
}

#[test]
fn skipped_section_renders_every_reason() {
    let dir = tempdir().unwrap();
    let (root, outs) = project(&dir);
    let root = root.as_path();
    fs::write(root.join("keep.rs"), "fn kept() {}\n").unwrap();
    fs::write(root.join("logo.png"), [0x89, 0x50, 0x4E, 0x47]).unwrap();
    fs::write(root.join("wide.rs"), vec![b'x'; 200]).unwrap();
    fs::write(root.join("id_rsa"), "PRIVATE KEY MATERIAL").unwrap();

    let mut args = args(root, &outs.join("out.md"));
    args.max_file_size = "100".into();
    let doc = run(args);
    assert!(doc.contains("## Skipped\n"));
    assert!(doc.contains("- `logo.png` — asset"));
    assert!(doc.contains("- `wide.rs` — too large"));
    assert!(doc.contains("- `id_rsa` — secret"));
    assert!(doc.contains("fn kept()"));
    // Reasons are sorted by path: id_rsa, logo.png, wide.rs.
    let skipped = doc.split("## Skipped").nth(1).unwrap();
    let id = skipped.find("id_rsa").unwrap();
    let logo = skipped.find("logo.png").unwrap();
    let wide = skipped.find("wide.rs").unwrap();
    assert!(id < logo && logo < wide);

    let policy = ContentPolicy::new("100", &[], false);
    let entries = collect_files_ext(root, &[], &[], &[], false).unwrap();
    let (_kept, skipped_files) = content_filter::partition(entries, root, &policy);
    let lines = content_filter::report_lines(&skipped_files);
    let joined = lines.join("\n");
    assert!(joined.contains("Skipping likely secret `id_rsa` (private key)."));
    assert!(!joined.contains("PRIVATE KEY MATERIAL"));
    assert_eq!(
        lines.last().map(String::as_str),
        Some("Skipped 3 files (1 asset, 1 too large, 1 secret).")
    );
    assert!(
        skipped_files
            .iter()
            .any(|item| item.reason == SkipReason::Secret)
    );
}

#[test]
fn each_secret_pattern_is_skipped_with_a_warning_and_templates_are_kept() {
    let dir = tempdir().unwrap();
    let (root, outs) = project(&dir);
    let root = root.as_path();
    let payload = "SUPER_SECRET_PAYLOAD_do_not_print";
    let secrets = [
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
    ];
    for name in secrets {
        fs::write(root.join(name), payload).unwrap();
    }
    fs::write(root.join(".env.example"), "EXAMPLE_KEPT=1\n").unwrap();
    fs::write(root.join(".env.sample"), "SAMPLE_KEPT=1\n").unwrap();
    fs::write(root.join("id_rsa.pub"), "ssh-ed25519 AAAA public\n").unwrap();
    fs::write(root.join("keep.rs"), "fn kept() {}\n").unwrap();
    fs::write(
        root.join(".npmrc"),
        "//registry.npmjs.org/:_authToken=npm_SUPER_SECRET_TOKEN\n",
    )
    .unwrap();
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
    fs::create_dir(root.join("env-npm")).unwrap();
    fs::write(
        root.join("env-npm/.npmrc"),
        "registry=https://registry.npmjs.org/\n//registry.npmjs.org/:_authToken=${NPM_TOKEN}\n",
    )
    .unwrap();
    fs::create_dir(root.join("env-pypi")).unwrap();
    fs::write(
        root.join("env-pypi/.pypirc"),
        "[pypi]\nusername = user\npassword = ${PYPI_TOKEN}\ntoken = $PYPI_TOKEN\n",
    )
    .unwrap();

    let mut with_hidden = args(root, &outs.join("out.md"));
    with_hidden.hidden = true;
    let doc = run(with_hidden);

    for name in secrets {
        let display = name.trim_start_matches("./");
        assert!(
            doc.contains(&format!("- `{display}` — secret")),
            "missing skipped entry for {display}\n{doc}"
        );
        assert!(
            !doc.contains(&format!("### File: `{display}`")),
            "{display} was included"
        );
    }
    assert!(doc.contains("- `.npmrc` — secret"));
    assert!(doc.contains("- `.pypirc` — secret"));
    assert!(doc.contains("EXAMPLE_KEPT=1"));
    assert!(doc.contains("SAMPLE_KEPT=1"));
    assert!(doc.contains("ssh-ed25519 AAAA public"));
    assert!(doc.contains("registry=https://registry.npmjs.org/"));
    assert!(doc.contains("password = changeme"));
    assert!(doc.contains("//registry.npmjs.org/:_authToken=${NPM_TOKEN}"));
    assert!(doc.contains("password = ${PYPI_TOKEN}"));
    assert!(doc.contains("token = $PYPI_TOKEN"));
    assert!(!doc.contains("- `env-npm/.npmrc` — secret"));
    assert!(!doc.contains("- `env-pypi/.pypirc` — secret"));
    assert!(doc.contains("fn kept()"));
    assert!(!doc.contains(payload));
    assert!(!doc.contains("npm_SUPER_SECRET_TOKEN"));
    assert!(!doc.contains("pypi-secret-password"));

    let policy = ContentPolicy::new("256K", &[], false);
    let entries = collect_files_ext(root, &[], &[], &[], true).unwrap();
    let (_kept, skipped) = content_filter::partition(entries, root, &policy);
    let warnings = content_filter::report_lines(&skipped);
    let joined = warnings.join("\n");
    for name in secrets {
        assert!(
            joined.contains(&format!("Skipping likely secret `{name}`")),
            "missing warning for {name}"
        );
    }
    assert!(joined.contains("Skipping likely secret `.npmrc` (npm credentials)."));
    assert!(joined.contains("Skipping likely secret `.pypirc` (PyPI credentials)."));
    assert!(!joined.contains("env-npm/.npmrc"));
    assert!(!joined.contains("env-pypi/.pypirc"));
    assert!(joined.contains("Skipping likely secret `.env` (environment file)."));
    assert!(!joined.contains(payload));
    assert!(!joined.contains("npm_SUPER_SECRET_TOKEN"));
    assert!(!joined.contains("pypi-secret-password"));
    assert!(!joined.contains(".env.example"));
    assert!(!joined.contains(".env.sample"));

    // --include-secrets keeps secrets, but dotfiles still need --hidden (already set).
    let mut opted = args(root, &outs.join("secrets.md"));
    opted.hidden = true;
    opted.include_secrets = true;
    let doc = run(opted);
    assert!(doc.contains("### File: `id_rsa`"));
    assert!(doc.contains("### File: `.env`"));
    assert!(!doc.contains("- `id_rsa` — secret"));

    // Naming the secret extension in --filter includes it; other secrets stay out.
    let mut pem_only = args(root, &outs.join("pem.md"));
    pem_only.filter = vec!["pem".into()];
    let doc = run(pem_only);
    assert!(doc.contains("### File: `server.pem`"));
    assert!(!doc.contains("- `server.pem` — secret"));
}

#[test]
fn hidden_includes_dotfiles_but_not_git_metadata_or_secrets() {
    let dir = tempdir().unwrap();
    let (root, outs) = project(&dir);
    let root = root.as_path();
    fs::create_dir_all(root.join(".github/workflows")).unwrap();
    fs::create_dir_all(root.join(".git/objects")).unwrap();
    fs::write(root.join("README.md"), "# readme\n").unwrap();
    fs::write(root.join(".gitignore"), "target/\n").unwrap();
    fs::write(root.join(".github/workflows/ci.yml"), "name: ci\n").unwrap();
    fs::write(root.join(".git/config"), "[core]\n").unwrap();
    fs::write(root.join(".env"), "TOKEN=hidden-secret\n").unwrap();
    fs::write(root.join(".env.example"), "TOKEN=example\n").unwrap();

    let plain = run(args(root, &outs.join("plain.md")));
    assert!(plain.contains("# readme"));
    assert!(!plain.contains("name: ci"));
    assert!(!plain.contains("target/"));
    assert!(!plain.contains("TOKEN=example"));
    assert!(!plain.contains("[core]"));
    assert!(!plain.contains("- `.env` — secret"));

    let mut hidden = args(root, &outs.join("hidden.md"));
    hidden.hidden = true;
    let doc = run(hidden);
    assert!(doc.contains("name: ci"));
    assert!(doc.contains("target/"));
    assert!(doc.contains("TOKEN=example"));
    assert!(!doc.contains("[core]"), ".git must stay out");
    assert!(doc.contains("- `.env` — secret"));
    assert!(!doc.contains("TOKEN=hidden-secret"));
}

#[test]
fn config_keys_apply_and_cli_wins() {
    let dir = tempdir().unwrap();
    let (root, outs) = project(&dir);
    let root = root.as_path();
    fs::write(root.join("small.rs"), "fn small() {}\n").unwrap();
    fs::write(root.join("wide.rs"), vec![b'z'; 80]).unwrap();
    fs::write(root.join(".gitignore"), "*.log\n").unwrap();
    fs::write(root.join("id_rsa"), "secret-key\n").unwrap();
    fs::write(
        root.join("context-builder.toml"),
        "max_file_size = \"50\"\nhidden = true\ninclude_secrets = true\n",
    )
    .unwrap();

    let loaded = context_builder::config::load_config_from_path(root).unwrap();
    let resolution = resolve_final_config(
        args(root, &outs.join("cfg.md")),
        Some(loaded.clone()),
        ExplicitCli::default(),
    );
    assert_eq!(resolution.config.max_file_size, "50");
    assert!(resolution.config.hidden);
    assert!(resolution.config.include_secrets);

    let mut resolved_args = args(root, &outs.join("cfg.md"));
    resolved_args.max_file_size = resolution.config.max_file_size.clone();
    resolved_args.hidden = resolution.config.hidden;
    resolved_args.include_secrets = resolution.config.include_secrets;
    let doc = run(resolved_args);
    assert!(doc.contains("- `wide.rs` — too large"));
    assert!(doc.contains("*.log"));
    assert!(doc.contains("### File: `id_rsa`"));

    // An explicit CLI size beats the config, even at the default spelling.
    let cli = args(root, &outs.join("cli.md"));
    let overridden = resolve_final_config(
        cli,
        Some(loaded),
        ExplicitCli {
            max_file_size: true,
            ..ExplicitCli::default()
        },
    );
    assert_eq!(overridden.config.max_file_size, "256K");
}
