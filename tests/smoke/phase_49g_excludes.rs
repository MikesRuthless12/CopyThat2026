//! Phase 49g smoke — `Repository::snapshot_source` filtered streaming walk.
//!
//! Builds a real temp tree and asserts the gitignore-style filters do what
//! the Library "Sources" exclude/include controls promise: directory
//! pruning (`**/node_modules`, `**/.git`), per-file gating (`*.tmp`), an
//! include whitelist, byte-for-byte restore of a kept file, and that an
//! invalid glob is refused at compile time. Skip-hidden has OS-specific
//! semantics (dotfile name vs. Windows HIDDEN attribute), so that case is
//! asserted on Unix only.

use std::fs;

use copythat_chunk::{FilterError, FilterSet, Repository, SnapshotKind};

fn lcg_bytes(seed: u64, len: usize) -> Vec<u8> {
    let mut out = vec![0u8; len];
    let mut s = seed;
    for b in &mut out {
        s = s
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        *b = (s >> 33) as u8;
    }
    out
}

/// Build the shared fixture tree under `root`:
/// `keep/a.txt`, `keep/big.bin`, `node_modules/x.js`, `.git/HEAD`,
/// `junk.tmp`. Returns the `big.bin` contents for the restore check.
fn build_tree(root: &std::path::Path) -> Vec<u8> {
    fs::create_dir_all(root.join("keep")).unwrap();
    fs::create_dir_all(root.join("node_modules")).unwrap();
    fs::create_dir_all(root.join(".git")).unwrap();
    fs::write(root.join("keep/a.txt"), b"hello").unwrap();
    let big = lcg_bytes(5, 200_000);
    fs::write(root.join("keep/big.bin"), &big).unwrap();
    fs::write(root.join("node_modules/x.js"), b"console.log(1)").unwrap();
    fs::write(root.join(".git/HEAD"), b"ref: refs/heads/main").unwrap();
    fs::write(root.join("junk.tmp"), b"scratch").unwrap();
    big
}

#[test]
fn excludes_prune_dirs_gate_files_and_restore() {
    let tmp = tempfile::tempdir().unwrap();
    let src = tmp.path().join("src");
    let big = build_tree(&src);
    let repo_dir = tmp.path().join("repo");
    let repo = Repository::open(&repo_dir).unwrap();

    let filters = FilterSet {
        exclude_globs: vec!["**/node_modules".into(), "**/.git".into(), "*.tmp".into()],
        ..Default::default()
    }
    .compile()
    .unwrap();

    let summary = repo
        .snapshot_source(
            SnapshotKind::Backup,
            "Docs",
            Some("src-A"),
            1000,
            &src,
            &filters,
        )
        .unwrap();

    let paths: Vec<String> = repo
        .snapshot_tree(summary.id)
        .unwrap()
        .into_iter()
        .map(|e| e.path)
        .collect();
    assert!(paths.contains(&"keep/a.txt".to_string()));
    assert!(paths.contains(&"keep/big.bin".to_string()));
    assert!(
        !paths.iter().any(|p| p.contains("node_modules")),
        "node_modules subtree must be pruned: {paths:?}"
    );
    assert!(!paths.iter().any(|p| p.contains(".git")), "{paths:?}");
    assert!(!paths.iter().any(|p| p.ends_with(".tmp")), "{paths:?}");
    assert_eq!(summary.files, 2);

    // A kept file restores byte-for-byte.
    let fs_entry = repo.snapshot_at("keep/big.bin", 2000).unwrap().unwrap();
    let dst = tmp.path().join("restored.bin");
    repo.restore(&fs_entry, &dst).unwrap();
    assert_eq!(fs::read(&dst).unwrap(), big);
}

#[test]
fn include_whitelist_keeps_only_matches() {
    let tmp = tempfile::tempdir().unwrap();
    let src = tmp.path().join("src");
    build_tree(&src);
    let repo = Repository::open(&tmp.path().join("repo")).unwrap();

    let filters = FilterSet {
        include_globs: vec!["**/*.txt".into()],
        ..Default::default()
    }
    .compile()
    .unwrap();

    let summary = repo
        .snapshot_source(SnapshotKind::Backup, "Docs", None, 1000, &src, &filters)
        .unwrap();
    let paths: Vec<String> = repo
        .snapshot_tree(summary.id)
        .unwrap()
        .into_iter()
        .map(|e| e.path)
        .collect();
    assert_eq!(paths, vec!["keep/a.txt".to_string()]);
}

#[cfg(unix)]
#[test]
fn skip_hidden_drops_dotfiles_on_unix() {
    let tmp = tempfile::tempdir().unwrap();
    let src = tmp.path().join("src");
    fs::create_dir_all(&src).unwrap();
    fs::write(src.join("visible.txt"), b"v").unwrap();
    fs::write(src.join(".secret"), b"s").unwrap();
    let repo = Repository::open(&tmp.path().join("repo")).unwrap();

    let filters = FilterSet {
        skip_hidden: true,
        ..Default::default()
    }
    .compile()
    .unwrap();
    let summary = repo
        .snapshot_source(SnapshotKind::Backup, "Docs", None, 1000, &src, &filters)
        .unwrap();
    let paths: Vec<String> = repo
        .snapshot_tree(summary.id)
        .unwrap()
        .into_iter()
        .map(|e| e.path)
        .collect();
    assert_eq!(paths, vec!["visible.txt".to_string()]);
}

#[test]
fn invalid_glob_is_rejected_at_compile() {
    let err = FilterSet {
        exclude_globs: vec!["[unterminated".into()],
        ..Default::default()
    }
    .compile()
    .unwrap_err();
    assert!(matches!(err, FilterError::InvalidGlob { .. }));
}
