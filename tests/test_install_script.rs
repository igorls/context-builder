//! `install.sh` against a fake release (a stub `curl` serves local files), so
//! no network is needed. Linux only: the script maps `uname` to a target.
#![cfg(target_os = "linux")]

use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::process::Command;

fn script() -> String {
    format!("{}/install.sh", env!("CARGO_MANIFEST_DIR"))
}

fn sha256(path: &Path) -> String {
    let out = Command::new("sha256sum").arg(path).output().unwrap();
    String::from_utf8_lossy(&out.stdout)
        .split_whitespace()
        .next()
        .unwrap()
        .to_string()
}

/// A release directory holding archives for `targets`, each containing a
/// `context-builder` that prints `marker`, plus SHA256SUMS.
fn fake_release(dir: &Path, targets: &[&str], marker: &str) {
    for target in targets {
        let stage = dir.join(format!("stage-{target}"));
        fs::create_dir_all(&stage).unwrap();
        let bin = stage.join("context-builder");
        fs::write(
            &bin,
            format!("#!/bin/sh\necho 'context-builder {marker}'\n"),
        )
        .unwrap();
        fs::set_permissions(&bin, fs::Permissions::from_mode(0o755)).unwrap();
        let archive = dir.join(format!("context-builder-{target}.tar.gz"));
        let status = Command::new("tar")
            .args([
                "czf",
                archive.to_str().unwrap(),
                "-C",
                stage.to_str().unwrap(),
                "context-builder",
            ])
            .status()
            .unwrap();
        assert!(status.success());
    }
    let mut sums = String::new();
    for target in targets {
        let archive = dir.join(format!("context-builder-{target}.tar.gz"));
        sums.push_str(&format!(
            "{}  context-builder-{target}.tar.gz\n",
            sha256(&archive)
        ));
    }
    fs::write(dir.join("SHA256SUMS"), sums).unwrap();
}

/// A `curl` that copies `<release>/<basename of URL>` to the `-o` target and
/// records the requested URLs.
fn fake_curl(bin_dir: &Path, release: &Path) {
    fs::create_dir_all(bin_dir).unwrap();
    let curl = bin_dir.join("curl");
    fs::write(
        &curl,
        format!(
            "#!/bin/sh\nurl=''; out=''\nwhile [ $# -gt 0 ]; do\n  case \"$1\" in\n    -o) out=\"$2\"; shift ;;\n    http*) url=\"$1\" ;;\n  esac\n  shift\ndone\necho \"$url\" >> '{rel}/requests.log'\ncp '{rel}'/\"$(basename \"$url\")\" \"$out\"\n",
            rel = release.display()
        ),
    )
    .unwrap();
    fs::set_permissions(&curl, fs::Permissions::from_mode(0o755)).unwrap();
}

fn run_install(
    libc: &str,
    arch_target: &str,
    marker: &str,
    path_extra: Option<&Path>,
) -> (std::process::Output, String, tempfile::TempDir) {
    let tmp = tempfile::tempdir().unwrap();
    let release = tmp.path().join("release");
    let bins = tmp.path().join("bin");
    let install = tmp.path().join("install");
    fs::create_dir_all(&release).unwrap();
    fake_release(&release, &[arch_target], marker);
    fake_curl(&bins, &release);
    let mut path = format!("{}:/usr/bin:/bin", bins.display());
    if let Some(extra) = path_extra {
        path = format!("{}:{path}", extra.display());
    }
    let out = Command::new("sh")
        .arg(script())
        .env("PATH", path)
        .env("CONTEXT_BUILDER_INSTALL_DIR", &install)
        .env("CONTEXT_BUILDER_LIBC", libc)
        .env("HOME", tmp.path())
        .output()
        .unwrap();
    let requests = fs::read_to_string(release.join("requests.log")).unwrap_or_default();
    (out, requests, tmp)
}

fn host_arch() -> &'static str {
    if cfg!(target_arch = "aarch64") {
        "aarch64"
    } else {
        "x86_64"
    }
}

#[test]
fn musl_override_downloads_the_musl_archive() {
    let target = format!("{}-unknown-linux-musl", host_arch());
    let (out, requests, tmp) = run_install("musl", &target, "from-install", None);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        out.status.success(),
        "{stdout}{}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(
        requests.contains(&format!("context-builder-{target}.tar.gz")),
        "{requests}"
    );
    assert!(tmp.path().join("install/context-builder").exists());
}

#[test]
fn gnu_override_downloads_the_gnu_archive() {
    let target = format!("{}-unknown-linux-gnu", host_arch());
    let (out, requests, _tmp) = run_install("gnu", &target, "from-install", None);
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(requests.contains("-unknown-linux-gnu.tar.gz"), "{requests}");
}

#[test]
fn verification_runs_the_installed_binary_not_the_one_on_path() {
    let target = format!("{}-unknown-linux-gnu", host_arch());
    let shadow = tempfile::tempdir().unwrap();
    let shadow_bin = shadow.path().join("context-builder");
    fs::write(
        &shadow_bin,
        "#!/bin/sh\necho 'context-builder shadow-copy'\n",
    )
    .unwrap();
    fs::set_permissions(&shadow_bin, fs::Permissions::from_mode(0o755)).unwrap();

    let (out, _requests, _tmp) = run_install("gnu", &target, "fresh-install", Some(shadow.path()));
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(out.status.success(), "{stdout}");
    assert!(
        stdout.contains("Installed: context-builder fresh-install"),
        "{stdout}"
    );
    assert!(
        !stdout.contains("Installed: context-builder shadow-copy"),
        "{stdout}"
    );
    assert!(stdout.contains("shadows this install"), "{stdout}");
}

#[test]
fn a_directory_not_on_path_is_called_out() {
    let target = format!("{}-unknown-linux-gnu", host_arch());
    let (out, _r, _tmp) = run_install("gnu", &target, "x", None);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("to your PATH"), "{stdout}");
}

#[test]
fn a_tampered_archive_is_rejected() {
    let target = format!("{}-unknown-linux-gnu", host_arch());
    let tmp = tempfile::tempdir().unwrap();
    let release = tmp.path().join("release");
    fs::create_dir_all(&release).unwrap();
    fake_release(&release, &[&target], "x");
    fs::write(
        release.join(format!("context-builder-{target}.tar.gz")),
        "tampered",
    )
    .unwrap();
    let bins = tmp.path().join("bin");
    fake_curl(&bins, &release);
    let out = Command::new("sh")
        .arg(script())
        .env("PATH", format!("{}:/usr/bin:/bin", bins.display()))
        .env("CONTEXT_BUILDER_INSTALL_DIR", tmp.path().join("install"))
        .env("CONTEXT_BUILDER_LIBC", "gnu")
        .env("HOME", tmp.path())
        .output()
        .unwrap();
    assert!(!out.status.success());
    assert!(String::from_utf8_lossy(&out.stdout).contains("Checksum verification failed"));
    assert!(!tmp.path().join("install/context-builder").exists());
}

fn requested_with_fake_ldd(ldd_output: &str) -> String {
    let tmp = tempfile::tempdir().unwrap();
    let release = tmp.path().join("release");
    let bins = tmp.path().join("bin");
    fs::create_dir_all(&release).unwrap();
    let arch = host_arch();
    let targets = [
        format!("{arch}-unknown-linux-gnu"),
        format!("{arch}-unknown-linux-musl"),
    ];
    let refs: Vec<&str> = targets.iter().map(String::as_str).collect();
    fake_release(&release, &refs, "x");
    fake_curl(&bins, &release);
    let ldd = bins.join("ldd");
    fs::write(&ldd, format!("#!/bin/sh\ncat <<'EOF'\n{ldd_output}\nEOF\n")).unwrap();
    fs::set_permissions(&ldd, fs::Permissions::from_mode(0o755)).unwrap();
    let out = Command::new("sh")
        .arg(script())
        .env("PATH", format!("{}:/usr/bin:/bin", bins.display()))
        .env("CONTEXT_BUILDER_INSTALL_DIR", tmp.path().join("install"))
        .env_remove("CONTEXT_BUILDER_LIBC")
        .env("HOME", tmp.path())
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stdout)
    );
    fs::read_to_string(release.join("requests.log")).unwrap()
}

#[test]
fn old_glibc_and_musl_systems_get_the_musl_build_new_glibc_gets_gnu() {
    assert!(requested_with_fake_ldd("ldd (GNU libc) 2.31").contains("linux-musl.tar.gz"));
    assert!(
        requested_with_fake_ldd("musl libc (x86_64)\nVersion 1.2.4").contains("linux-musl.tar.gz")
    );
    assert!(
        requested_with_fake_ldd("ldd (Ubuntu GLIBC 2.39-0ubuntu8) 2.39")
            .contains("linux-gnu.tar.gz")
    );
    assert!(requested_with_fake_ldd("ldd (GNU libc) 2.34").contains("linux-gnu.tar.gz"));
}

#[test]
fn release_without_a_musl_archive_stops_instead_of_installing_an_unusable_gnu_binary() {
    // Releases cut before the musl build existed only list the gnu archive.
    let target = format!("{}-unknown-linux-gnu", host_arch());
    let (out, requests, tmp) = run_install("musl", &target, "from-install", None);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(!out.status.success(), "{stdout}");
    assert!(stdout.contains("no musl build"), "{stdout}");
    assert!(stdout.contains("cargo install context-builder"), "{stdout}");
    assert!(
        !requests.contains(".tar.gz"),
        "no archive downloaded: {requests}"
    );
    assert!(!tmp.path().join("install/context-builder").exists());
}

#[test]
fn forcing_gnu_installs_the_gnu_build_even_where_musl_was_detected() {
    let target = format!("{}-unknown-linux-gnu", host_arch());
    let (out, requests, tmp) = run_install("gnu", &target, "forced", None);
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stdout)
    );
    assert!(requests.contains("linux-gnu.tar.gz"), "{requests}");
    assert!(tmp.path().join("install/context-builder").exists());
}
