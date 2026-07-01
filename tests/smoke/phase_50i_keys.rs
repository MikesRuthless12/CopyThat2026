//! Phase 50i smoke — multiple key slots + recovery key on the repository
//! access gate. Proves: several passphrases each unlock the repo, a generated
//! recovery key unlocks it, per-slot rotation leaves other slots intact,
//! removing a slot revokes just that credential, and the last slot cannot be
//! removed (which would silently unlock the repo).
//!
//! Each `open_existing` takes redb's exclusive lock, so every handle is
//! dropped before the next open. The key-slot mutations act on the on-disk
//! `repo-key.json`, independent of the open database.

use copythat_chunk::Repository;

/// `true` if `secret` opens the repository at `root`.
fn unlocks(root: &std::path::Path, secret: &str) -> bool {
    match Repository::open_existing(root, Some(secret)) {
        Ok(repo) => {
            drop(repo); // release the redb lock before the next attempt
            true
        }
        Err(_) => false,
    }
}

#[test]
fn multiple_slots_recovery_and_revocation() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();

    // Create with a primary passphrase.
    drop(Repository::create(root, Some("alpha")).unwrap());
    assert!(Repository::requires_passphrase(root));

    // Wrong / missing secret is refused; the primary opens it.
    assert!(!unlocks(root, "wrong"));
    assert!(
        Repository::open_existing(root, None).is_err(),
        "locked without a secret"
    );
    assert!(unlocks(root, "alpha"));

    // Add a second password slot and mint a recovery key.
    let recovery;
    {
        let repo = Repository::open_existing(root, Some("alpha")).unwrap();
        repo.add_key(Some("alpha"), "bravo", "laptop").unwrap();
        recovery = repo.generate_recovery_key(Some("alpha")).unwrap();
        let keys = repo.list_keys().unwrap();
        assert_eq!(keys.len(), 3, "primary + laptop + recovery");
        assert!(
            keys.iter()
                .any(|k| k.label == "primary" && k.kind == "password")
        );
        assert!(
            keys.iter()
                .any(|k| k.label == "laptop" && k.kind == "password")
        );
        assert!(keys.iter().any(|k| k.kind == "recovery"));
        // A duplicate label is refused (reuse this authenticated handle).
        assert!(
            repo.add_key(Some("alpha"), "x", "laptop").is_err(),
            "duplicate label"
        );
    }
    assert!(recovery.len() >= 32, "recovery key is high-entropy");

    // All three credentials unlock the repo.
    assert!(unlocks(root, "alpha"));
    assert!(unlocks(root, "bravo"));
    assert!(unlocks(root, &recovery));

    // Rotate ONLY the primary (alpha → alpha2); the laptop slot is untouched.
    {
        let repo = Repository::open_existing(root, Some("alpha")).unwrap();
        repo.change_password(Some("alpha"), "alpha2").unwrap();
    }
    assert!(!unlocks(root, "alpha"), "old primary revoked by rotation");
    assert!(unlocks(root, "alpha2"), "rotated primary works");
    assert!(unlocks(root, "bravo"), "other slot unaffected by rotation");

    // Revoke the primary slot; alpha2 stops working, the others remain.
    {
        let repo = Repository::open_existing(root, Some("bravo")).unwrap();
        assert!(repo.remove_key("primary").unwrap());
        assert!(!repo.remove_key("primary").unwrap(), "already gone");
        assert_eq!(repo.list_keys().unwrap().len(), 2);
    }
    assert!(!unlocks(root, "alpha2"), "revoked primary no longer opens");
    assert!(
        unlocks(root, &recovery),
        "recovery still opens after primary revoked"
    );

    // Remove the recovery slot, then refuse to remove the final slot.
    {
        let repo = Repository::open_existing(root, Some("bravo")).unwrap();
        assert!(repo.remove_key("recovery").unwrap());
        assert_eq!(repo.list_keys().unwrap().len(), 1);
        assert!(
            repo.remove_key("laptop").is_err(),
            "must refuse to remove the last key slot"
        );
    }
    assert!(unlocks(root, "bravo"), "sole remaining slot still opens");
}
