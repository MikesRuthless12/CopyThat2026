//! Phase 49k smoke — repository create/connect lifecycle + passphrase gate.

use copythat_chunk::{ChunkStoreError, Repository, SnapshotKind};

#[test]
fn create_open_passphrase_and_isolation() {
    let tmp = tempfile::tempdir().unwrap();

    // --- Plain repo: create, then open_existing round-trips with its data.
    let a = tmp.path().join("a");
    {
        let repo = Repository::create(&a, None).unwrap();
        repo.snapshot_bytes(SnapshotKind::Backup, "s", 1000, &[("/x", b"hello")])
            .unwrap();
    }
    assert!(!Repository::requires_passphrase(&a));
    let repo_a = Repository::open_existing(&a, None).unwrap();
    assert_eq!(repo_a.snapshots().unwrap().len(), 1);

    // --- create-twice → AlreadyExists (file-existence check, no lock taken).
    assert!(matches!(
        Repository::create(&a, None),
        Err(ChunkStoreError::AlreadyExists(_))
    ));

    // --- open a missing repo → NotInitialised.
    let missing = tmp.path().join("nope");
    assert!(matches!(
        Repository::open_existing(&missing, None),
        Err(ChunkStoreError::NotInitialised(_))
    ));

    // --- Passphrase-gated repo.
    let b = tmp.path().join("b");
    drop(Repository::create(&b, Some("hunter2")).unwrap());
    assert!(Repository::requires_passphrase(&b));
    // no passphrase → Locked.
    assert!(matches!(
        Repository::open_existing(&b, None),
        Err(ChunkStoreError::Locked)
    ));
    // wrong → BadPassphrase.
    assert!(matches!(
        Repository::open_existing(&b, Some("wrong")),
        Err(ChunkStoreError::BadPassphrase)
    ));
    // right → ok; rotate; old fails, new works.
    let repo_b = Repository::open_existing(&b, Some("hunter2")).unwrap();
    repo_b.change_password(Some("hunter2"), "newpass").unwrap();
    drop(repo_b);
    assert!(matches!(
        Repository::open_existing(&b, Some("hunter2")),
        Err(ChunkStoreError::BadPassphrase)
    ));
    let repo_b = Repository::open_existing(&b, Some("newpass")).unwrap();

    // --- Two repos isolate: a's snapshot is absent from b.
    assert_eq!(repo_b.snapshots().unwrap().len(), 0);
    assert_eq!(repo_a.snapshots().unwrap().len(), 1);
}
