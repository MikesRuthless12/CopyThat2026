//! Phase 49l smoke — Sources dashboard fold (summaries only, by directory).

use copythat_chunk::{Repository, SnapshotKind};

#[test]
fn sources_group_by_directory_prefix() {
    let tmp = tempfile::tempdir().unwrap();
    let repo = Repository::open(tmp.path()).unwrap();

    // Two snapshots under /docs (one multi-file at /docs, one single file at
    // /docs), one under /photos.
    repo.snapshot_bytes(
        SnapshotKind::Backup,
        "docs1",
        1000,
        &[("/docs/a.txt", b"a"), ("/docs/b.txt", b"bb")],
    )
    .unwrap();
    repo.snapshot_bytes(
        SnapshotKind::Version,
        "docs2",
        2000,
        &[("/docs/c.txt", b"ccc")],
    )
    .unwrap();
    repo.snapshot_bytes(
        SnapshotKind::Backup,
        "photos",
        3000,
        &[("/photos/p.jpg", b"jpeg")],
    )
    .unwrap();

    let sources = repo.sources().unwrap();
    assert_eq!(sources.len(), 2, "two distinct source directories");

    // Newest source first (photos at 3000).
    assert_eq!(sources[0].source, "/photos");

    let photos = sources.iter().find(|s| s.source == "/photos").unwrap();
    assert_eq!(photos.snapshot_count, 1);
    assert_eq!(photos.latest_ms, 3000);
    assert_eq!(photos.latest_kind, SnapshotKind::Backup);
    assert_eq!(photos.total_files, 1);

    let docs = sources.iter().find(|s| s.source == "/docs").unwrap();
    assert_eq!(docs.snapshot_count, 2);
    assert_eq!(docs.latest_ms, 2000); // docs2 is newest
    assert_eq!(docs.latest_kind, SnapshotKind::Version);
}

#[test]
fn single_file_source_is_its_parent_dir() {
    let tmp = tempfile::tempdir().unwrap();
    let repo = Repository::open(tmp.path()).unwrap();
    repo.snapshot_bytes(
        SnapshotKind::Backup,
        "one",
        1000,
        &[("/home/me/photo.jpg", b"x")],
    )
    .unwrap();
    let sources = repo.sources().unwrap();
    assert_eq!(sources.len(), 1);
    assert_eq!(sources[0].source, "/home/me");
}
