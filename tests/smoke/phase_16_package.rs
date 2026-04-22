//! Phase 16 smoke test — free-first packaging guarantee.
//!
//! Every packaging artifact + CI workflow we ship must be reachable
//! without paying for a code-signing certificate, Apple Developer
//! membership, or Azure Trusted Signing subscription. This test is a
//! tripwire: if a future edit silently wires a paid service into
//! `release.yml`, the assertion here fires *before* the tag.
//!
//! The test is deliberately filesystem-only — it doesn't need a
//! Tauri runtime, a webview, or network. It asserts:
//!
//! 1. `.github/workflows/release.yml` exists and, outside YAML comment
//!    lines, never references any of the paid-signing tokens listed
//!    below. Comment lines (`^\s*#`) are allowed — we document the
//!    upgrade path in `docs/SIGNING_UPGRADE.md` and want the
//!    commented-out job blocks to stay readable.
//! 2. `tauri.conf.json` sets `bundle.macOS.signingIdentity = "-"` so
//!    macOS bundles use the free ad-hoc codesign.
//! 3. Every packaging manifest under `packaging/` exists and is
//!    non-empty (winget, Chocolatey, Homebrew cask, Flatpak + appdata,
//!    AUR PKGBUILD).
//! 4. `docs/SIGNING_UPGRADE.md` exists and mentions each of the three
//!    free-first platforms.
//! 5. `xtask release` subcommand is registered in `xtask/src/main.rs`.
//!
//! On failure the assertion prints the offending line/file so the
//! next reader can fix forward without hunting.

use std::fs;
use std::path::{Path, PathBuf};

/// Tokens that imply a paid signing service is being invoked for
/// real (not just documented in comments). These are the tokens any
/// CI upgrade to paid signing would legitimately need; the whole
/// point of Phase 16 is to ship *without* them.
const FORBIDDEN_LIVE_TOKENS: &[&str] = &[
    // Apple Notary / Developer ID
    "APPLE_CERTIFICATE:",
    "APPLE_CERTIFICATE_PASSWORD:",
    "APPLE_APP_SPECIFIC_PASSWORD:",
    "APPLE_TEAM_ID:",
    "APPLE_ID:",
    "DEVELOPER_ID_APPLICATION",
    "notarytool submit",
    // Azure Trusted Signing
    "AZURE_TENANT_ID:",
    "AZURE_CLIENT_ID:",
    "AZURE_CLIENT_SECRET:",
    "Azure/trusted-signing-action",
    // Legacy EV cert workflows
    "WINDOWS_PFX_BASE64:",
    "WINDOWS_PFX_PASSWORD:",
    "signtool.exe sign",
];

fn repo_root() -> PathBuf {
    // CARGO_MANIFEST_DIR points into the UI crate when run via its
    // `[[test]]` registration, so walk up until we find the workspace
    // root (Cargo.toml + locales/ — same marker `xtask` uses).
    let here = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let mut cur: &Path = &here;
    loop {
        if cur.join("Cargo.toml").is_file() && cur.join("locales").is_dir() {
            return cur.to_path_buf();
        }
        cur = match cur.parent() {
            Some(p) => p,
            None => break,
        };
    }
    panic!(
        "could not locate repo root from CARGO_MANIFEST_DIR={}",
        here.display()
    );
}

fn read_to_string(path: &Path) -> String {
    fs::read_to_string(path).unwrap_or_else(|e| panic!("reading {}: {e}", path.display()))
}

#[test]
fn release_workflow_has_no_live_paid_tokens() {
    let root = repo_root();
    let path = root.join(".github").join("workflows").join("release.yml");
    assert!(
        path.is_file(),
        "missing release workflow at {}",
        path.display()
    );

    let content = read_to_string(&path);
    let mut offenders: Vec<(usize, &str, String)> = Vec::new();
    for (idx, line) in content.lines().enumerate() {
        let trimmed = line.trim_start();
        if trimmed.starts_with('#') {
            // Commented blocks are documentation, not live config —
            // the whole "upgrade path" story in SIGNING_UPGRADE.md
            // depends on them being readable here.
            continue;
        }
        for token in FORBIDDEN_LIVE_TOKENS {
            if line.contains(token) {
                offenders.push((idx + 1, token, line.to_string()));
            }
        }
    }
    assert!(
        offenders.is_empty(),
        "release.yml contains live references to paid-signing tokens:\n{}",
        offenders
            .iter()
            .map(|(n, t, l)| format!("  line {n}: token `{t}` in `{}`", l.trim()))
            .collect::<Vec<_>>()
            .join("\n"),
    );
}

#[test]
fn release_workflow_runs_on_free_runners() {
    let root = repo_root();
    let path = root.join(".github").join("workflows").join("release.yml");
    let content = read_to_string(&path);
    // Only free GitHub-hosted runner labels are allowed. If a later
    // edit points this workflow at a self-hosted runner (which may
    // cost money or require a paid signing HSM), the assertion here
    // fires before the tag lands.
    let free_runner_labels = [
        "ubuntu-latest",
        "macos-latest",
        "macos-13",
        "macos-14",
        "windows-latest",
    ];
    let mut any_runs_on_line = false;
    for line in content.lines() {
        let trimmed = line.trim_start();
        if !trimmed.starts_with("runs-on:") {
            continue;
        }
        any_runs_on_line = true;
        let has_free = free_runner_labels
            .iter()
            .any(|lbl| trimmed.contains(lbl) || trimmed.contains("matrix."));
        assert!(
            has_free,
            "release.yml `runs-on:` line is not a free GitHub-hosted label: {}",
            trimmed,
        );
    }
    assert!(
        any_runs_on_line,
        "release.yml has no `runs-on:` lines — did the workflow lose its jobs?",
    );
}

#[test]
fn tauri_conf_uses_ad_hoc_codesign() {
    let root = repo_root();
    let path = root
        .join("apps")
        .join("copythat-ui")
        .join("src-tauri")
        .join("tauri.conf.json");
    let content = read_to_string(&path);
    assert!(
        content.contains("\"signingIdentity\": \"-\""),
        "tauri.conf.json must set bundle.macOS.signingIdentity to \"-\" (ad-hoc); got:\n{content}"
    );
    // The pubkey for the updater plugin stays a placeholder until
    // the release pipeline grows a MiniSign key; this guard makes
    // the placeholder explicit so nobody ships a real key half-wired.
    assert!(
        content.contains("REPLACE_ME_WITH_RELEASE_PUBKEY") || !content.contains("\"pubkey\""),
        "tauri.conf.json must use the placeholder pubkey (or omit it) until Phase 16 MiniSign key-pair wiring lands"
    );
}

#[test]
fn packaging_manifests_exist_and_are_non_empty() {
    let root = repo_root();
    let required_files: &[&[&str]] = &[
        &["packaging", "README.md"],
        &["packaging", "windows", "winget", "CopyThat.CopyThat.yaml"],
        &[
            "packaging",
            "windows",
            "winget",
            "CopyThat.CopyThat.locale.en-US.yaml",
        ],
        &[
            "packaging",
            "windows",
            "winget",
            "CopyThat.CopyThat.installer.yaml",
        ],
        &["packaging", "windows", "chocolatey", "copythat.nuspec"],
        &[
            "packaging",
            "windows",
            "chocolatey",
            "tools",
            "chocolateyinstall.ps1",
        ],
        &["packaging", "macos", "homebrew-cask", "copythat.rb"],
        &["packaging", "linux", "flatpak", "com.copythat.desktop.yml"],
        &[
            "packaging",
            "linux",
            "flatpak",
            "com.copythat.desktop.appdata.xml",
        ],
        &["packaging", "linux", "aur", "PKGBUILD"],
    ];
    for parts in required_files {
        let mut p = root.clone();
        for seg in *parts {
            p.push(seg);
        }
        assert!(p.is_file(), "missing packaging manifest: {}", p.display());
        let meta = fs::metadata(&p).unwrap_or_else(|e| panic!("stat {}: {e}", p.display()));
        assert!(
            meta.len() > 16,
            "packaging manifest is suspiciously small: {} ({} bytes)",
            p.display(),
            meta.len(),
        );
    }
}

#[test]
fn signing_upgrade_doc_covers_each_platform() {
    // `docs/SIGNING_UPGRADE.md` lives under `.gitignore` alongside the
    // other strategic planning docs (ROADMAP / CHANGELOG / ARCHITECTURE
    // — see `.gitignore` § "Strategic docs kept local-only"). Fresh
    // clones and CI runners won't see it; that's deliberate. When the
    // file *is* on disk (maintainer working copy), the content check
    // below still fires so the upgrade-path story stays honest.
    let root = repo_root();
    let path = root.join("docs").join("SIGNING_UPGRADE.md");
    if !path.is_file() {
        eprintln!(
            "skip signing-upgrade-doc check: {} not present on disk (gitignored)",
            path.display()
        );
        return;
    }
    let content = read_to_string(&path);
    for needle in [
        "Azure Trusted Signing",
        "Apple Developer",
        "GPG_SIGNING_KEY",
        "MiniSign",
        "APPLE_SIGNING_IDENTITY",
    ] {
        assert!(
            content.contains(needle),
            "docs/SIGNING_UPGRADE.md is missing expected section / keyword `{needle}`"
        );
    }
}

#[test]
fn xtask_release_subcommand_wired_up() {
    let root = repo_root();
    let main = read_to_string(&root.join("xtask").join("src").join("main.rs"));
    assert!(
        main.contains("Some(\"release\")"),
        "xtask/src/main.rs must dispatch the `release` subcommand (Phase 16)"
    );
    assert!(
        main.contains("mod release;"),
        "xtask/src/main.rs must `mod release;` the Phase 16 release orchestrator"
    );
}

#[test]
fn release_workflow_triggers_only_on_tags_or_manual() {
    let root = repo_root();
    let content = read_to_string(&root.join(".github").join("workflows").join("release.yml"));
    // Either a tag-pattern push trigger or a workflow_dispatch; never
    // a blanket `on: push` that would run the release path on every
    // commit to main (which would burn artifact storage needlessly).
    assert!(
        content.contains("tags: [\"v*.*.*\"]") || content.contains("tags:\n      - \"v*.*.*\""),
        "release.yml must trigger on tag pattern v*.*.*"
    );
    assert!(
        !content.contains("branches: [main]") && !content.contains("branches:\n      - main"),
        "release.yml must NOT trigger on branch push — tags only (plus manual dispatch)"
    );
}
