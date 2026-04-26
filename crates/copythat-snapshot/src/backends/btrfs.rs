//! Btrfs read-only subvolume snapshots.
//!
//! `btrfs subvolume snapshot -r <subvol> <snap-path>` creates a
//! read-only snapshot as a regular subvolume. We mount it in-place —
//! Btrfs exposes subvolumes as directories under the mount point of
//! the containing filesystem, so no extra mount call is needed. We put
//! the snapshot next to the source subvolume under a hidden
//! `.copythat-snapshots/` directory, clean it up via
//! `btrfs subvolume delete` on release.

use std::path::{Path, PathBuf};
use std::process::Stdio;

use tokio::process::Command;

use crate::SnapshotHandle;
use crate::error::SnapshotError;
use crate::kind::SnapshotKind;

use super::Cleanup as SuperCleanup;

#[derive(Debug)]
pub(crate) struct Cleanup {
    snap_path: PathBuf,
}

/// Cheap probe: is `path` on a Btrfs filesystem? Reads
/// `/proc/self/mountinfo` once and matches the longest mount-point
/// prefix of `path`.
pub(crate) fn applies_to(path: &Path) -> bool {
    match super::unix_mount::fs_type(path) {
        Some(fs) => fs == "btrfs",
        None => false,
    }
}

pub(crate) async fn create(src_path: &Path) -> Result<SnapshotHandle, SnapshotError> {
    let subvol_root =
        find_subvolume_root(src_path).ok_or_else(|| SnapshotError::BackendFailure {
            kind: SnapshotKind::Btrfs,
            message: format!(
                "could not locate btrfs subvolume containing {}",
                src_path.display()
            ),
        })?;

    let snap_dir = subvol_root.join(".copythat-snapshots");
    tokio::fs::create_dir_all(&snap_dir)
        .await
        .map_err(|e| SnapshotError::BackendFailure {
            kind: SnapshotKind::Btrfs,
            message: format!("create snapshot dir {}: {e}", snap_dir.display()),
        })?;

    let name = format!("copythat-{}", uuid::Uuid::new_v4());
    let snap_path = snap_dir.join(&name);

    let out = Command::new("/sbin/btrfs")
        .arg("subvolume")
        .arg("snapshot")
        .arg("-r")
        .arg(&subvol_root)
        .arg(&snap_path)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await
        .map_err(map_spawn_err)?;

    if !out.status.success() {
        return Err(SnapshotError::BackendFailure {
            kind: SnapshotKind::Btrfs,
            message: format!(
                "btrfs subvolume snapshot failed (code {:?}): {}",
                out.status.code(),
                String::from_utf8_lossy(&out.stderr).trim()
            ),
        });
    }

    Ok(SnapshotHandle {
        kind: SnapshotKind::Btrfs,
        mount_path: snap_path.clone(),
        original_root: subvol_root,
        cleanup: Some(SuperCleanup::Btrfs(Cleanup { snap_path })),
    })
}

pub(crate) async fn release(c: Cleanup) -> Result<(), SnapshotError> {
    let out = Command::new("/sbin/btrfs")
        .arg("subvolume")
        .arg("delete")
        .arg(&c.snap_path)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await
        .map_err(map_spawn_err)?;
    if out.status.success() {
        Ok(())
    } else {
        Err(SnapshotError::BackendFailure {
            kind: SnapshotKind::Btrfs,
            message: format!(
                "btrfs subvolume delete failed (code {:?}): {}",
                out.status.code(),
                String::from_utf8_lossy(&out.stderr).trim()
            ),
        })
    }
}

pub(crate) fn release_blocking(c: Cleanup) -> Result<(), SnapshotError> {
    let out = std::process::Command::new("/sbin/btrfs")
        .arg("subvolume")
        .arg("delete")
        .arg(&c.snap_path)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .map_err(map_spawn_err)?;
    if out.status.success() {
        Ok(())
    } else {
        Err(SnapshotError::BackendFailure {
            kind: SnapshotKind::Btrfs,
            message: format!(
                "btrfs subvolume delete failed (code {:?}): {}",
                out.status.code(),
                String::from_utf8_lossy(&out.stderr).trim()
            ),
        })
    }
}

fn map_spawn_err(e: std::io::Error) -> SnapshotError {
    if e.kind() == std::io::ErrorKind::NotFound {
        SnapshotError::BackendMissing { tool: "btrfs" }
    } else {
        SnapshotError::Io(e)
    }
}

/// Walk upward from `path` until we find a directory that is itself a
/// Btrfs subvolume. The cheap heuristic: the longest mount-point from
/// `/proc/self/mountinfo` whose fs_type is `btrfs`. For the smoke
/// test's loopback this is the loop-mount itself.
fn find_subvolume_root(path: &Path) -> Option<PathBuf> {
    let data = std::fs::read_to_string("/proc/self/mountinfo").ok()?;
    let canon = path.canonicalize().ok()?;
    let mut best: Option<(usize, PathBuf)> = None;
    for line in data.lines() {
        let fields: Vec<&str> = line.split_whitespace().collect();
        if fields.len() < 5 {
            continue;
        }
        let mount_point = PathBuf::from(fields[4]);
        // Check fs_type for btrfs.
        let mut saw_dash = false;
        let mut fs_type = None;
        for p in &fields[5..] {
            if *p == "-" {
                saw_dash = true;
                continue;
            }
            if saw_dash {
                fs_type = Some(*p);
                break;
            }
        }
        if fs_type != Some("btrfs") {
            continue;
        }
        if canon.starts_with(&mount_point) {
            let score = mount_point.as_os_str().len();
            if best.as_ref().map(|(s, _)| score > *s).unwrap_or(true) {
                best = Some((score, mount_point));
            }
        }
    }
    best.map(|(_, p)| p)
}
