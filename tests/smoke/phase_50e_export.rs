//! Phase 50e smoke — restic/borg EXPORT round-trips through the importers.
//!
//! No external binary: build a CDR repo, `export` it to the target format,
//! feed it back through `migrate` (the importer), and assert byte-identical
//! file restores — the same self-validating pattern as the 50b/50c/50d
//! importer smokes.

use copythat_chunk::{MigrateError, RepoFormat, Repository, SnapshotKind, export, migrate};

fn build_src(dir: &std::path::Path) -> (Vec<u8>, Vec<u8>) {
    // `a` is shared across both snapshots (tests dedup survives export);
    // `b` is pseudo-random (multi-chunk, non-trivial content).
    let a = vec![0xABu8; 100_000];
    let mut b = vec![0u8; 250_000];
    let mut s = 12_345u64;
    for x in &mut b {
        s = s.wrapping_mul(6364136223846793005).wrapping_add(1);
        *x = (s >> 33) as u8;
    }
    let repo = Repository::open(dir).unwrap();
    repo.snapshot_bytes(
        SnapshotKind::Backup,
        "s1",
        1000,
        &[("/docs/a.bin", &a), ("/docs/b.bin", &b)],
    )
    .unwrap();
    repo.snapshot_bytes(SnapshotKind::Backup, "s2", 2000, &[("/docs/a.bin", &a)])
        .unwrap();
    (a, b)
}

fn assert_roundtrip(back: &std::path::Path, a: &[u8], b: &[u8], tmp: &std::path::Path) {
    let repo = Repository::open(back).unwrap();
    assert!(
        !repo.snapshots().unwrap().is_empty(),
        "re-import produced snapshots"
    );
    let out = tmp.join("out.bin");
    let fa = repo.snapshot_at("/docs/a.bin", 9_999).unwrap().unwrap();
    repo.restore(&fa, &out).unwrap();
    assert_eq!(std::fs::read(&out).unwrap(), a, "a.bin round-trips");
    let fb = repo.snapshot_at("/docs/b.bin", 9_999).unwrap().unwrap();
    repo.restore(&fb, &out).unwrap();
    assert_eq!(std::fs::read(&out).unwrap(), b, "b.bin round-trips");
}

#[test]
fn restic_export_round_trips() {
    let tmp = tempfile::tempdir().unwrap();
    let src = tmp.path().join("src");
    let (a, b) = build_src(&src);
    let restic_dir = tmp.path().join("restic-out");
    export(RepoFormat::Restic, &src, &restic_dir, Some("pw")).unwrap();
    let back = tmp.path().join("back");
    migrate(RepoFormat::Restic, &restic_dir, &back, Some("pw")).unwrap();
    assert_roundtrip(&back, &a, &b, tmp.path());
}

#[test]
fn borg_export_round_trips() {
    let tmp = tempfile::tempdir().unwrap();
    let src = tmp.path().join("src");
    let (a, b) = build_src(&src);
    let borg_dir = tmp.path().join("borg-out");
    export(RepoFormat::Borg, &src, &borg_dir, Some("pw")).unwrap();
    let back = tmp.path().join("back");
    migrate(RepoFormat::Borg, &borg_dir, &back, Some("pw")).unwrap();
    assert_roundtrip(&back, &a, &b, tmp.path());
}

#[test]
fn kopia_export_is_unsupported() {
    let tmp = tempfile::tempdir().unwrap();
    let src = tmp.path().join("src");
    build_src(&src);
    let out = tmp.path().join("kopia-out");
    assert!(matches!(
        export(RepoFormat::Kopia, &src, &out, Some("pw")),
        Err(MigrateError::SourceUnsupported { .. })
    ));
}
