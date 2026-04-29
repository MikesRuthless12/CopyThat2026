//! Phase 41 smoke — pre-execution dry-run / tree-diff plan.
//!
//! The prompt's scenario translated into Rust:
//!
//! - Source carries 10 files: 5 are duplicated on the destination
//!   (3 newer in src, 2 newer in dst), 5 are source-only.
//! - Plus 3 destination-only files the source never sees.
//! - Plus 1 "kind mismatch" pair (src is a file, dst is a directory).
//!
//! Assertions:
//!
//! 1. Six cases / six PASS — additions count, replacements count
//!    (yellow `SourceNewer`), skips count (`DestinationNewer`),
//!    conflicts count (kind mismatch), unchanged count, and the
//!    bytes_to_transfer running total.
//! 2. Force-overwrite mode flips the dst-newer skips into orange
//!    `ForceOverwriteOlder` replacements.
//! 3. All 14 Phase 41 Fluent keys (`preview-*`) appear in every one
//!    of the 18 locale files.

use std::fs;
use std::path::Path;
use std::time::{Duration, SystemTime};

use copythat_core::dryrun::{
    ConflictKind, DryRunOptions, ReasonForReplacement, ReasonForSkip, compute_tree_diff,
};

const PHASE_41_KEYS: &[&str] = &[
    "preview-modal-title",
    "preview-summary-header",
    "preview-category-additions",
    "preview-category-replacements",
    "preview-category-skips",
    "preview-category-conflicts",
    "preview-category-unchanged",
    "preview-bytes-to-transfer",
    "preview-reason-source-newer",
    "preview-reason-dest-newer",
    "preview-reason-content-different",
    "preview-reason-identical",
    "preview-button-run",
    "preview-button-reduce",
];

const LOCALES: &[&str] = &[
    "en", "es", "zh-CN", "hi", "ar", "pt-BR", "ru", "ja", "de", "fr", "ko", "it", "tr", "vi", "pl",
    "nl", "id", "uk",
];

fn touch(path: &Path, body: &[u8]) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(path, body).unwrap();
}

fn set_mtime(path: &Path, when: SystemTime) {
    filetime::set_file_mtime(path, filetime::FileTime::from_system_time(when)).unwrap();
}

#[test]
fn case01_default_classification() {
    let src = tempfile::tempdir().unwrap();
    let dst = tempfile::tempdir().unwrap();
    let now = SystemTime::now();
    let earlier = now - Duration::from_secs(120);
    let later = now + Duration::from_secs(120);

    // src + dst share these — src wins (yellow):
    for name in ["shared-newer-src/a.bin", "shared-newer-src/b.bin", "c.bin"] {
        touch(&src.path().join(name), b"src-content");
        touch(&dst.path().join(name), b"old");
        set_mtime(&dst.path().join(name), earlier);
        set_mtime(&src.path().join(name), now);
    }
    // src + dst share these — dst is newer (skip by default):
    for name in ["dst-newer/a.bin", "dst-newer/b.bin"] {
        touch(&src.path().join(name), b"src");
        touch(&dst.path().join(name), b"dst-newer");
        set_mtime(&src.path().join(name), earlier);
        set_mtime(&dst.path().join(name), later);
    }
    // src-only (additions):
    for name in [
        "only-src/a.bin",
        "only-src/b.bin",
        "only-src/c.bin",
        "only-src/d.bin",
        "only-src/e.bin",
    ] {
        touch(&src.path().join(name), b"only-src-content");
    }
    // dst-only (we don't see these — TreeDiff is one-way; the user
    // would need a sync-mode walk for those).
    for name in ["only-dst/a.bin", "only-dst/b.bin", "only-dst/c.bin"] {
        touch(&dst.path().join(name), b"only-dst");
    }
    // kind mismatch (file vs directory):
    let mismatch = "kind-clash";
    touch(&src.path().join(mismatch), b"file");
    fs::create_dir_all(dst.path().join(mismatch)).unwrap();

    let diff = compute_tree_diff(src.path(), dst.path(), &DryRunOptions::default()).unwrap();

    assert_eq!(diff.additions.len(), 5, "5 source-only files");
    assert_eq!(diff.replacements.len(), 3, "3 src-newer pairs");
    for (_, reason) in &diff.replacements {
        assert_eq!(*reason, ReasonForReplacement::SourceNewer);
    }
    assert_eq!(diff.skips.len(), 2, "2 dst-newer skips");
    for (_, reason) in &diff.skips {
        assert_eq!(*reason, ReasonForSkip::DestinationNewer);
    }
    assert_eq!(diff.conflicts.len(), 1, "1 kind-mismatch conflict");
    assert_eq!(diff.conflicts[0].1, ConflictKind::KindMismatch);
    assert!(diff.has_blocking_conflicts());
    assert_eq!(diff.unchanged.len(), 0);

    let expected_to_transfer = 5 * 16  // additions × "only-src-content" len
        + 3 * 11; // src-newer replacements × "src-content" len
    assert_eq!(diff.bytes_to_transfer, expected_to_transfer);
}

#[test]
fn case02_force_overwrite_flips_dst_newer_to_replacement() {
    let src = tempfile::tempdir().unwrap();
    let dst = tempfile::tempdir().unwrap();
    let now = SystemTime::now();
    let earlier = now - Duration::from_secs(120);
    let later = now + Duration::from_secs(120);

    let s = src.path().join("a.bin");
    let d = dst.path().join("a.bin");
    touch(&s, b"src");
    touch(&d, b"dst-newer-and-bigger");
    set_mtime(&s, earlier);
    set_mtime(&d, later);

    let opts = DryRunOptions {
        force_overwrite: true,
        ..DryRunOptions::default()
    };
    let diff = compute_tree_diff(src.path(), dst.path(), &opts).unwrap();
    assert_eq!(diff.replacements.len(), 1);
    assert_eq!(
        diff.replacements[0].1,
        ReasonForReplacement::ForceOverwriteOlder
    );
    assert!(diff.skips.is_empty());
}

#[test]
fn case03_identical_size_mtime_classes_unchanged() {
    let src = tempfile::tempdir().unwrap();
    let dst = tempfile::tempdir().unwrap();
    let s = src.path().join("a.bin");
    let d = dst.path().join("a.bin");
    touch(&s, b"hello");
    touch(&d, b"hello");
    let mtime = fs::metadata(&s).unwrap().modified().unwrap();
    set_mtime(&d, mtime);
    let opts = DryRunOptions {
        trust_size_mtime: true,
        ..DryRunOptions::default()
    };
    let diff = compute_tree_diff(src.path(), dst.path(), &opts).unwrap();
    assert_eq!(diff.unchanged.len(), 1);
    assert_eq!(diff.bytes_to_transfer, 0);
}

#[test]
fn case04_empty_diff_when_only_destination_has_content() {
    let src = tempfile::tempdir().unwrap();
    let dst = tempfile::tempdir().unwrap();
    touch(&dst.path().join("only-dst.bin"), b"dst");
    let diff = compute_tree_diff(src.path(), dst.path(), &DryRunOptions::default()).unwrap();
    // TreeDiff is one-way — destination-only files aren't visible to
    // a copy plan. The plan is empty.
    assert!(diff.is_empty());
}

#[test]
fn case05_total_files_aggregates_every_category() {
    let src = tempfile::tempdir().unwrap();
    let dst = tempfile::tempdir().unwrap();
    let now = SystemTime::now();
    let earlier = now - Duration::from_secs(60);
    let later = now + Duration::from_secs(60);

    // 1 addition.
    touch(&src.path().join("add.bin"), b"x");
    // 1 src-newer replacement.
    touch(&src.path().join("repl.bin"), b"new");
    touch(&dst.path().join("repl.bin"), b"old");
    set_mtime(&src.path().join("repl.bin"), now);
    set_mtime(&dst.path().join("repl.bin"), earlier);
    // 1 dst-newer skip.
    touch(&src.path().join("skip.bin"), b"old");
    touch(&dst.path().join("skip.bin"), b"newer");
    set_mtime(&src.path().join("skip.bin"), earlier);
    set_mtime(&dst.path().join("skip.bin"), later);
    // 1 conflict.
    touch(&src.path().join("conflict.bin"), b"a");
    fs::create_dir_all(dst.path().join("conflict.bin")).unwrap();

    let diff = compute_tree_diff(src.path(), dst.path(), &DryRunOptions::default()).unwrap();
    assert_eq!(diff.total_files(), 4);
    assert_eq!(diff.additions.len(), 1);
    assert_eq!(diff.replacements.len(), 1);
    assert_eq!(diff.skips.len(), 1);
    assert_eq!(diff.conflicts.len(), 1);
}

#[test]
fn case06_all_phase_41_keys_present_in_every_locale() {
    let workspace_root = workspace_root();
    let mut missing: Vec<String> = Vec::new();
    for locale in LOCALES {
        let path = workspace_root
            .join("locales")
            .join(locale)
            .join("copythat.ftl");
        let body = std::fs::read_to_string(&path)
            .unwrap_or_else(|_| panic!("could not read {}", path.display()));
        for key in PHASE_41_KEYS {
            let needle = format!("{key} =");
            if !body.contains(&needle) {
                missing.push(format!("{locale}/{key}"));
            }
        }
    }
    assert!(missing.is_empty(), "missing Phase 41 keys: {missing:?}");
}

fn workspace_root() -> std::path::PathBuf {
    let crate_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    crate_dir
        .parent()
        .and_then(|p| p.parent())
        .expect("workspace has crates/<name>/Cargo.toml layout")
        .to_path_buf()
}
