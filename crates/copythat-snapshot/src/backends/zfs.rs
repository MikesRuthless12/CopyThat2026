//! ZFS snapshot backend.
//!
//! `zfs snapshot <dataset>@<name>` creates a point-in-time snapshot.
//! ZFS automatically exposes every snapshot at
//! `<dataset-mountpoint>/.zfs/snapshot/<name>`, so we don't need a
//! separate mount call. Cleanup destroys the snapshot dataset.
//!
//! Compiled in on Linux and FreeBSD — macOS ships a userspace ZFS-on-
//! FUSE port that is out of scope for Phase 19b.

use std::path::{Path, PathBuf};
use std::process::Stdio;

use tokio::process::Command;

use crate::SnapshotHandle;
use crate::error::SnapshotError;
use crate::kind::SnapshotKind;

use super::Cleanup as SuperCleanup;

#[derive(Debug)]
pub(crate) struct Cleanup {
    /// The `dataset@snap` name passed to `zfs destroy`.
    dataset_snapshot: String,
}

pub(crate) fn applies_to(path: &Path) -> bool {
    // On Linux prefer mountinfo; on FreeBSD fall back to asking `zfs`
    // directly (the mountinfo helper short-circuits to None there).
    if let Some(fs) = super::unix_mount::fs_type(path) {
        return fs == "zfs";
    }
    find_zfs_dataset(path).is_some()
}

pub(crate) async fn create(src_path: &Path) -> Result<SnapshotHandle, SnapshotError> {
    let (dataset, mount_point) =
        find_zfs_dataset(src_path).ok_or_else(|| SnapshotError::BackendFailure {
            kind: SnapshotKind::Zfs,
            message: format!("could not locate zfs dataset for {}", src_path.display()),
        })?;

    let name = format!("copythat-{}", uuid::Uuid::new_v4());
    let snap_spec = format!("{dataset}@{name}");

    let out = Command::new("/sbin/zfs")
        .arg("snapshot")
        .arg("--")
        .arg(&snap_spec)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await
        .map_err(map_spawn_err)?;

    if !out.status.success() {
        return Err(SnapshotError::BackendFailure {
            kind: SnapshotKind::Zfs,
            message: format!(
                "zfs snapshot failed (code {:?}): {}",
                out.status.code(),
                String::from_utf8_lossy(&out.stderr).trim()
            ),
        });
    }

    let snapshot_dir: PathBuf = mount_point.join(".zfs").join("snapshot").join(&name);

    Ok(SnapshotHandle {
        kind: SnapshotKind::Zfs,
        mount_path: snapshot_dir,
        original_root: mount_point,
        cleanup: Some(SuperCleanup::Zfs(Cleanup {
            dataset_snapshot: snap_spec,
        })),
    })
}

pub(crate) async fn release(c: Cleanup) -> Result<(), SnapshotError> {
    // Use absolute /sbin/zfs (avoids `$PATH` hijacks in user
    // shells) and insert `--` so a dataset spec accidentally
    // beginning with `-` (or a hostile `zfs list` returning a
    // crafted name from a fake `zfs` binary on PATH) cannot be
    // interpreted as flags — `-r`/`-f` would otherwise turn this
    // into a recursive force-destroy.
    let out = Command::new("/sbin/zfs")
        .arg("destroy")
        .arg("--")
        .arg(&c.dataset_snapshot)
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
            kind: SnapshotKind::Zfs,
            message: format!(
                "zfs destroy failed (code {:?}): {}",
                out.status.code(),
                String::from_utf8_lossy(&out.stderr).trim()
            ),
        })
    }
}

pub(crate) fn release_blocking(c: Cleanup) -> Result<(), SnapshotError> {
    let out = std::process::Command::new("zfs")
        .arg("destroy")
        .arg(&c.dataset_snapshot)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .map_err(map_spawn_err)?;
    if out.status.success() {
        Ok(())
    } else {
        Err(SnapshotError::BackendFailure {
            kind: SnapshotKind::Zfs,
            message: format!(
                "zfs destroy failed (code {:?}): {}",
                out.status.code(),
                String::from_utf8_lossy(&out.stderr).trim()
            ),
        })
    }
}

fn map_spawn_err(e: std::io::Error) -> SnapshotError {
    if e.kind() == std::io::ErrorKind::NotFound {
        SnapshotError::BackendMissing { tool: "zfs" }
    } else {
        SnapshotError::Io(e)
    }
}

/// Returns `(dataset_name, mount_point)` for the ZFS dataset that
/// contains `path`. Calls `zfs list -H -o name,mountpoint` and picks
/// the longest mountpoint prefix match. `None` when `zfs` is not
/// available or no dataset covers the path.
fn find_zfs_dataset(path: &Path) -> Option<(String, PathBuf)> {
    let canon = path.canonicalize().ok()?;
    // Absolute path defends against a `~/.local/bin/zfs` shim that
    // could otherwise return a crafted `<name>\t<mp>` line whose
    // dataset name begins with `-` and gets interpreted as a flag
    // by downstream `zfs destroy`.
    let output = std::process::Command::new("/sbin/zfs")
        .arg("list")
        .arg("-H")
        .arg("-o")
        .arg("name,mountpoint")
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let text = String::from_utf8(output.stdout).ok()?;
    let mut best: Option<(usize, String, PathBuf)> = None;
    for line in text.lines() {
        let mut parts = line.split('\t');
        let name = parts.next()?;
        let mp = parts.next()?;
        if mp == "none" || mp == "legacy" || mp == "-" {
            continue;
        }
        let mp_path = PathBuf::from(mp);
        if canon.starts_with(&mp_path) {
            let score = mp_path.as_os_str().len();
            if best.as_ref().map(|(s, _, _)| score > *s).unwrap_or(true) {
                best = Some((score, name.to_string(), mp_path));
            }
        }
    }
    best.map(|(_, n, p)| (n, p))
}
