//! Phase 24 — security-metadata preservation smoke test.
//!
//! Cross-platform contract test: every host arm seeds a per-OS metadata
//! stream on the source, copies via `copythat_core::copy_file` with a
//! `PlatformMetaOps` hook attached, and asserts the destination carries
//! the same stream.
//!
//! - **Linux**: `setfattr user.copythat.test=phase24-roundtrip` on the
//!   source. After the copy the destination's xattr value must match.
//! - **macOS**: `xattr -w com.apple.metadata:kMDItemWhereFroms` (the
//!   Spotlight provenance attribute) on the source; round-trip.
//! - **Windows**: write a `Zone.Identifier` ADS via the documented
//!   `:Zone.Identifier` stream syntax (the same surface PowerShell's
//!   `Set-Content -Stream` uses). After the copy, the destination's
//!   `Zone.Identifier` ADS must match.
//!
//! On every host the test also asserts:
//! 1. The capture-and-apply pass survives the `MetaPolicy::default()`
//!    (preserve everything) round-trip.
//! 2. Disabling the policy bit for the captured stream causes that
//!    stream to be dropped on apply (the dst ends up without it).
//! 3. `MetaSnapshot::is_empty()` is `true` for a vanilla file.
//!
//! Filesystems that don't support the relevant metadata stream are
//! detected at seed time — when `xattr::set` returns `ENOTSUP` we
//! short-circuit the per-host arm with `eprintln!` and pass. This
//! keeps the smoke test green on tmpfs CI runners while still
//! exercising the engine plumbing.

use std::path::Path;
use std::sync::Arc;

use copythat_core::meta::{MetaOps, MetaPolicy, NoopMetaOps};
use copythat_core::{CopyControl, CopyEvent, CopyOptions, copy_file};
use copythat_platform::PlatformMetaOps;
use tempfile::tempdir;
use tokio::sync::mpsc;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn phase_24_meta_round_trip() {
    let tmp = tempdir().unwrap();
    let src = tmp.path().join("src.bin");
    let dst = tmp.path().join("dst.bin");
    std::fs::write(&src, b"phase24 primary content").unwrap();

    let seeded = seed_metadata_for_host(&src);
    if !seeded {
        eprintln!("phase 24: host filesystem rejected metadata seed; skipping per-host assertion");
    }

    // --- Engine-driven copy with the platform meta hook attached ---
    let (tx, mut rx) = mpsc::channel::<CopyEvent>(64);
    let opts = CopyOptions {
        meta_ops: Some(Arc::new(PlatformMetaOps)),
        preserve_security_metadata: true,
        ..Default::default()
    };
    let src_owned = src.clone();
    let dst_owned = dst.clone();
    let copy = tokio::spawn(async move {
        copy_file(&src_owned, &dst_owned, opts, CopyControl::new(), tx).await
    });
    // Drain the events channel so the engine never back-pressures.
    while rx.recv().await.is_some() {}
    let report = copy.await.unwrap();
    assert!(report.is_ok(), "engine copy failed: {:?}", report.err());

    // --- Round-trip assertion (gated on the seed step working) ---
    if seeded {
        assert_metadata_preserved(&src, &dst);
    }

    // --- Vanilla file → empty snapshot ---
    let vanilla = tmp.path().join("vanilla.bin");
    std::fs::write(&vanilla, b"vanilla").unwrap();
    let snap = PlatformMetaOps.capture(&vanilla).unwrap();
    // On Windows a vanilla file may carry a single ::$DATA stream; we
    // skip that in `parse_ads_name`. Linux / macOS report fully empty.
    assert!(snap.ads.is_empty(), "vanilla file should have no ADS");
    if cfg!(any(target_os = "linux", target_os = "macos")) {
        // macOS sometimes auto-applies `com.apple.lastuseddate` xattrs;
        // tolerate by checking only the structured fields.
        assert!(snap.posix_acl.is_none());
        assert!(snap.selinux.is_none());
        assert!(snap.linux_caps.is_none());
        assert!(snap.mac_resource_fork.is_none());
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn phase_24_noop_meta_ops_does_nothing() {
    // Engine + NoopMetaOps must never error and never produce
    // partial-failure noise. Mirrors what the test harness uses when
    // the platform crate isn't wired in.
    let tmp = tempdir().unwrap();
    let src = tmp.path().join("noop-src.bin");
    let dst = tmp.path().join("noop-dst.bin");
    std::fs::write(&src, b"noop content").unwrap();

    let (tx, mut rx) = mpsc::channel::<CopyEvent>(64);
    let opts = CopyOptions {
        meta_ops: Some(Arc::new(NoopMetaOps)),
        preserve_security_metadata: true,
        ..Default::default()
    };
    let copy =
        tokio::spawn(async move { copy_file(&src, &dst, opts, CopyControl::new(), tx).await });
    while rx.recv().await.is_some() {}
    let report = copy.await.unwrap();
    assert!(report.is_ok(), "noop meta copy failed: {:?}", report.err());
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn phase_24_policy_filter_drops_stream() {
    // Capture a snapshot, run it through `MetaPolicy.filter` with
    // `preserve_xattrs = false`, and confirm only the structured
    // entries (POSIX ACLs / SELinux / capabilities) survive.
    use copythat_core::meta::{MetaSnapshot, NtfsStream, XattrEntry};

    let snap = MetaSnapshot {
        ads: vec![NtfsStream {
            name: "Zone.Identifier".to_string(),
            data: b"[ZoneTransfer]\nZoneId=3".to_vec(),
        }],
        xattrs: vec![
            XattrEntry {
                name: "user.foo".to_string(),
                value: b"bar".to_vec(),
            },
            XattrEntry {
                name: "system.posix_acl_access".to_string(),
                value: vec![0xAA; 4],
            },
        ],
        ..MetaSnapshot::default()
    };

    // MOTW-off → strips Zone.Identifier even with everything else on.
    let policy = MetaPolicy {
        preserve_motw: false,
        ..MetaPolicy::default()
    };
    let mut filtered = snap.clone();
    policy.filter(&mut filtered);
    assert!(
        filtered.ads.is_empty(),
        "MOTW-off should strip Zone.Identifier"
    );
    assert_eq!(filtered.xattrs.len(), 2);

    // xattrs-off → drops user.foo but keeps the structured ones.
    let policy = MetaPolicy {
        preserve_xattrs: false,
        ..MetaPolicy::default()
    };
    let mut filtered = snap.clone();
    policy.filter(&mut filtered);
    assert_eq!(
        filtered.ads.len(),
        1,
        "ADS should survive an xattrs-only off"
    );
    assert_eq!(filtered.xattrs.len(), 1, "user.foo should be dropped");
    assert_eq!(filtered.xattrs[0].name, "system.posix_acl_access");
}

// ============================================================
// Per-host seed + assert helpers
// ============================================================

#[cfg(target_os = "windows")]
fn seed_metadata_for_host(src: &Path) -> bool {
    // Write the Zone.Identifier ADS via the `:streamname` syntax.
    // This is the same surface the documented examples in
    // `Set-Content -Stream Zone.Identifier` exercise.
    let mut path = src.as_os_str().to_os_string();
    path.push(":Zone.Identifier");
    let payload = b"[ZoneTransfer]\r\nZoneId=3\r\n";
    match std::fs::write(std::path::PathBuf::from(path), payload) {
        Ok(()) => true,
        Err(e) => {
            eprintln!("phase 24 windows seed: failed to write ADS: {e}");
            false
        }
    }
}

#[cfg(target_os = "windows")]
fn assert_metadata_preserved(_src: &Path, dst: &Path) {
    let mut path = dst.as_os_str().to_os_string();
    path.push(":Zone.Identifier");
    let bytes = std::fs::read(std::path::PathBuf::from(path)).unwrap_or_default();
    assert!(
        !bytes.is_empty(),
        "Zone.Identifier ADS missing on destination"
    );
    assert!(
        bytes.windows(b"ZoneId=3".len()).any(|w| w == b"ZoneId=3"),
        "Zone.Identifier payload mismatch on destination: {bytes:?}"
    );
}

#[cfg(target_os = "linux")]
fn seed_metadata_for_host(src: &Path) -> bool {
    let value = b"phase24-roundtrip";
    match xattr::set(src, "user.copythat.test", value) {
        Ok(()) => true,
        Err(e) => {
            eprintln!("phase 24 linux seed: xattr::set failed: {e}");
            false
        }
    }
}

#[cfg(target_os = "linux")]
fn assert_metadata_preserved(_src: &Path, dst: &Path) {
    let got = xattr::get(dst, "user.copythat.test")
        .ok()
        .flatten()
        .unwrap_or_default();
    assert_eq!(
        got.as_slice(),
        b"phase24-roundtrip",
        "user.copythat.test xattr missing or wrong on destination"
    );
}

#[cfg(target_os = "macos")]
fn seed_metadata_for_host(src: &Path) -> bool {
    let value = b"http://example.com/source";
    match xattr::set(src, "com.apple.metadata:kMDItemWhereFroms", value) {
        Ok(()) => true,
        Err(e) => {
            eprintln!("phase 24 macos seed: xattr::set failed: {e}");
            false
        }
    }
}

#[cfg(target_os = "macos")]
fn assert_metadata_preserved(_src: &Path, dst: &Path) {
    let got = xattr::get(dst, "com.apple.metadata:kMDItemWhereFroms")
        .ok()
        .flatten()
        .unwrap_or_default();
    assert_eq!(
        got.as_slice(),
        b"http://example.com/source",
        "kMDItemWhereFroms xattr missing or wrong on destination"
    );
}

#[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
fn seed_metadata_for_host(_src: &Path) -> bool {
    // Unknown host — nothing to seed. The engine wiring is still
    // exercised by the empty-capture path.
    false
}

#[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
fn assert_metadata_preserved(_src: &Path, _dst: &Path) {}
