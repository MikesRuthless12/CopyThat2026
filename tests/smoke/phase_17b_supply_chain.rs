//! Phase 17b smoke test — supply-chain audit gates.
//!
//! Phase 17b's deliverable is **wiring**: cargo-audit + cargo-vet on
//! every push, vet trust imports configured, and `docs/SECURITY.md`
//! documenting the policy. This smoke is the tripwire — if a future
//! edit silently removes the gate, the assertion fires before the
//! commit lands.
//!
//! The check is filesystem-only; it never executes cargo-audit or
//! cargo-vet. That keeps the test cheap on every push and avoids a
//! transitive network dependency on a runner that may be offline.
//! The actual audit runs in CI (`.github/workflows/ci.yml`) where it
//! belongs.

use std::fs;
use std::path::{Path, PathBuf};

fn repo_root() -> PathBuf {
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
fn ci_workflow_runs_cargo_audit() {
    let ci = read_to_string(&repo_root().join(".github/workflows/ci.yml"));
    assert!(
        ci.contains("cargo-audit:") && ci.contains("cargo audit"),
        "ci.yml must declare a cargo-audit job that invokes `cargo audit`",
    );
    assert!(
        ci.contains("--ignore RUSTSEC-"),
        "ci.yml must mirror the deny.toml ignore list via `--ignore` flags",
    );
}

#[test]
fn ci_workflow_runs_cargo_vet() {
    let ci = read_to_string(&repo_root().join(".github/workflows/ci.yml"));
    assert!(
        ci.contains("cargo-vet:") && ci.contains("cargo vet"),
        "ci.yml must declare a cargo-vet job that invokes `cargo vet`",
    );
}

#[test]
fn supply_chain_config_present() {
    let root = repo_root();
    let cfg = root.join("supply-chain/config.toml");
    let aud = root.join("supply-chain/audits.toml");
    let lock = root.join("supply-chain/imports.lock");
    assert!(cfg.is_file(), "missing supply-chain/config.toml");
    assert!(aud.is_file(), "missing supply-chain/audits.toml");
    assert!(lock.is_file(), "missing supply-chain/imports.lock");

    let body = read_to_string(&cfg);
    // Trust imports must include at least the four widely-used feeds we
    // documented in the file. New feeds can be added freely; removing one
    // requires a deliberate edit + a new audit row backfilling the gap.
    for required in [
        "imports.bytecode-alliance",
        "imports.embark-studios",
        "imports.google",
        "imports.mozilla",
    ] {
        assert!(
            body.contains(required),
            "supply-chain/config.toml is missing trust import {required}",
        );
    }
}

#[test]
fn security_md_documents_audit_and_vet_gates() {
    let body = read_to_string(&repo_root().join("docs/SECURITY.md"));
    assert!(
        body.contains("`cargo audit` runs on every push"),
        "docs/SECURITY.md must call out the cargo audit gate as live (Phase 17b)",
    );
    assert!(
        body.contains("`cargo vet` runs on every push"),
        "docs/SECURITY.md must call out the cargo vet gate as live (Phase 17b)",
    );
}

#[test]
fn deny_toml_ignore_block_matches_audit_ignore_flags() {
    let root = repo_root();
    let deny = read_to_string(&root.join("deny.toml"));
    let ci = read_to_string(&root.join(".github/workflows/ci.yml"));

    // Pull every RUSTSEC-* id mentioned in deny.toml's ignore section
    // and confirm cargo-audit ignores the same set. Drift between the
    // two surfaces would either mask new findings (deny ignores, audit
    // catches and fails the build) or block CI on something deny is
    // silently allowing — both bad.
    let mut ids: Vec<&str> = deny
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            // Lines look like `"RUSTSEC-2023-0071", # ...`
            if line.starts_with('"') && line[1..].starts_with("RUSTSEC-") {
                let close = line[1..].find('"').map(|i| i + 1)?;
                Some(&line[1..close])
            } else {
                None
            }
        })
        .collect();
    ids.sort();
    ids.dedup();
    assert!(
        !ids.is_empty(),
        "deny.toml has no RUSTSEC ignores; expected the Phase 36 set",
    );
    for id in ids {
        assert!(
            ci.contains(id),
            "ci.yml cargo-audit step is missing ignore for {id} (drift vs deny.toml)",
        );
    }
}
