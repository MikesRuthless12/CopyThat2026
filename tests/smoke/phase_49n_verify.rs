//! Phase 49n smoke — snapshot verification & repair (quarantine).

use copythat_chunk::{Repository, SnapshotKind, VerifyLevel};

#[test]
fn verify_clean_then_detect_corruption_and_repair() {
    let tmp = tempfile::tempdir().unwrap();
    let repo = Repository::open(tmp.path()).unwrap();
    let data = vec![7u8; 2 * 1024 * 1024];
    let id = repo
        .snapshot_bytes(SnapshotKind::Backup, "s", 1000, &[("/f", &data)])
        .unwrap();

    // A healthy repo verifies clean at both levels.
    assert!(repo.verify(None, VerifyLevel::Metadata).unwrap().is_clean());
    assert!(repo.verify(None, VerifyLevel::ReadData).unwrap().is_clean());

    // Corrupt a byte in the pack → the deep (ReadData) pass detects it.
    let packs_dir = tmp.path().join("packs");
    let pack = std::fs::read_dir(&packs_dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .find(|p| {
            p.extension().is_some_and(|x| x == "pack")
                && std::fs::metadata(p).map(|m| m.len() > 0).unwrap_or(false)
        })
        .expect("a non-empty pack file");
    {
        use std::io::{Read, Seek, SeekFrom, Write};
        let mut f = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open(&pack)
            .unwrap();
        let mut b = [0u8; 1];
        f.seek(SeekFrom::Start(0)).unwrap();
        f.read_exact(&mut b).unwrap();
        b[0] ^= 0xff;
        f.seek(SeekFrom::Start(0)).unwrap();
        f.write_all(&b).unwrap();
    }

    let report = repo.verify(None, VerifyLevel::ReadData).unwrap();
    assert!(!report.is_clean(), "corruption should be detected");
    assert!(report.damage.iter().any(|d| d.snapshot_id == id.0));

    // Metadata-only stays clean (the index entry still exists — no pack read).
    assert!(repo.verify(None, VerifyLevel::Metadata).unwrap().is_clean());

    // Repair dry-run names the damaged snapshot but changes nothing.
    let (would, _gc) = repo.repair_remove_damaged(&report, false).unwrap();
    assert_eq!(would, vec![id]);
    assert_eq!(repo.snapshots().unwrap().len(), 1, "dry run keeps it");

    // Apply removes the un-restorable snapshot.
    let (removed, _gc) = repo.repair_remove_damaged(&report, true).unwrap();
    assert_eq!(removed, vec![id]);
    assert_eq!(repo.snapshots().unwrap().len(), 0);
}
