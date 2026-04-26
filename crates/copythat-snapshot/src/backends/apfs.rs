//! macOS APFS local snapshots via `tmutil` + `mount_apfs`.
//!
//! `tmutil localsnapshot` asks macOS to take an APFS local snapshot
//! (the same primitive Time Machine uses for hourly backups on the
//! boot volume). The snapshot name is auto-generated with a
//! `com.apple.TimeMachine.<timestamp>.local` prefix; `tmutil
//! listlocalsnapshotdates` surfaces just the timestamps.
//!
//! To make the snapshot readable we mount it read-only at an ephemeral
//! directory with `mount_apfs -o nobrowse -s <timestamp> /System/Volumes/Data /tmp/copythat-<uuid>`.
//!
//! Cleanup: `umount` the mount point and then `tmutil
//! deletelocalsnapshots <timestamp>`. The `mount_apfs` path is
//! best-effort on CI runners (System Integrity Protection + the
//! sealed system volume make local snapshots flaky), so the smoke
//! test gates this backend behind an opt-in env var.

use std::path::{Path, PathBuf};
use std::process::Stdio;

use tokio::process::Command;

use crate::SnapshotHandle;
use crate::error::SnapshotError;
use crate::kind::SnapshotKind;

use super::Cleanup as SuperCleanup;

#[derive(Debug)]
pub(crate) struct Cleanup {
    mount_point: PathBuf,
    snapshot_timestamp: String,
    /// Which underlying volume `mount_apfs` attached; typically
    /// `/System/Volumes/Data` on modern macOS.
    device: String,
}

/// macOS-only probe: we always claim to apply (APFS is the default),
/// but every helper call gracefully degrades to `BackendMissing` when
/// `tmutil` / `mount_apfs` is unavailable.
pub(crate) fn applies_to(_path: &Path) -> bool {
    true
}

pub(crate) async fn create(src_path: &Path) -> Result<SnapshotHandle, SnapshotError> {
    // Step 1: ask tmutil to take a fresh local snapshot. The system
    // snapshots the current root volume (typically `/System/Volumes/Data`).
    let out = Command::new("/usr/bin/tmutil")
        .arg("localsnapshot")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await
        .map_err(map_spawn_err_tmutil)?;
    if !out.status.success() {
        return Err(SnapshotError::BackendFailure {
            kind: SnapshotKind::Apfs,
            message: format!(
                "tmutil localsnapshot failed (code {:?}): {}",
                out.status.code(),
                String::from_utf8_lossy(&out.stderr).trim()
            ),
        });
    }
    let stamp = extract_timestamp(&String::from_utf8_lossy(&out.stdout)).ok_or_else(|| {
        SnapshotError::BackendFailure {
            kind: SnapshotKind::Apfs,
            message: "could not parse timestamp from `tmutil localsnapshot` output".to_string(),
        }
    })?;

    // Step 2: pick the device the path lives on. `df -P <path>` prints
    // the device as the first column of the last line.
    let device = detect_device_for(src_path).unwrap_or_else(|| "/System/Volumes/Data".to_string());

    // Step 3: mount the snapshot read-only at an ephemeral dir.
    let id = uuid::Uuid::new_v4();
    let mount_point = PathBuf::from(format!("/tmp/copythat-snap-{id}"));
    tokio::fs::create_dir_all(&mount_point)
        .await
        .map_err(|e| SnapshotError::BackendFailure {
            kind: SnapshotKind::Apfs,
            message: format!("create mountpoint {}: {e}", mount_point.display()),
        })?;

    let mount_out = Command::new("/sbin/mount_apfs")
        .arg("-o")
        .arg("nobrowse,ro")
        .arg("-s")
        .arg(&stamp)
        .arg(&device)
        .arg(&mount_point)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await
        .map_err(map_spawn_err_mount)?;
    if !mount_out.status.success() {
        // Best-effort roll-back of tmutil snapshot we just created.
        let _ = Command::new("/usr/bin/tmutil")
            .args(["deletelocalsnapshots", &stamp])
            .output()
            .await;
        let _ = tokio::fs::remove_dir(&mount_point).await;
        return Err(SnapshotError::BackendFailure {
            kind: SnapshotKind::Apfs,
            message: format!(
                "mount_apfs failed (code {:?}): {}",
                mount_out.status.code(),
                String::from_utf8_lossy(&mount_out.stderr).trim()
            ),
        });
    }

    Ok(SnapshotHandle {
        kind: SnapshotKind::Apfs,
        mount_path: mount_point.clone(),
        original_root: PathBuf::from(&device),
        cleanup: Some(SuperCleanup::Apfs(Cleanup {
            mount_point,
            snapshot_timestamp: stamp,
            device,
        })),
    })
}

pub(crate) async fn release(c: Cleanup) -> Result<(), SnapshotError> {
    let _ = Command::new("/sbin/umount")
        .arg(&c.mount_point)
        .output()
        .await
        .map_err(map_spawn_err_mount)?;
    let _ = tokio::fs::remove_dir(&c.mount_point).await;
    let out = Command::new("/usr/bin/tmutil")
        .arg("deletelocalsnapshots")
        .arg(&c.snapshot_timestamp)
        .output()
        .await
        .map_err(map_spawn_err_tmutil)?;
    if out.status.success() {
        let _ = &c.device;
        Ok(())
    } else {
        Err(SnapshotError::BackendFailure {
            kind: SnapshotKind::Apfs,
            message: format!(
                "tmutil deletelocalsnapshots failed: {}",
                String::from_utf8_lossy(&out.stderr).trim()
            ),
        })
    }
}

pub(crate) fn release_blocking(c: Cleanup) -> Result<(), SnapshotError> {
    let _ = std::process::Command::new("/sbin/umount")
        .arg(&c.mount_point)
        .output()
        .map_err(map_spawn_err_mount)?;
    let _ = std::fs::remove_dir(&c.mount_point);
    let out = std::process::Command::new("/usr/bin/tmutil")
        .arg("deletelocalsnapshots")
        .arg(&c.snapshot_timestamp)
        .output()
        .map_err(map_spawn_err_tmutil)?;
    if out.status.success() {
        let _ = &c.device;
        Ok(())
    } else {
        Err(SnapshotError::BackendFailure {
            kind: SnapshotKind::Apfs,
            message: format!(
                "tmutil deletelocalsnapshots failed: {}",
                String::from_utf8_lossy(&out.stderr).trim()
            ),
        })
    }
}

fn map_spawn_err_tmutil(e: std::io::Error) -> SnapshotError {
    if e.kind() == std::io::ErrorKind::NotFound {
        SnapshotError::BackendMissing { tool: "tmutil" }
    } else {
        SnapshotError::Io(e)
    }
}

fn map_spawn_err_mount(e: std::io::Error) -> SnapshotError {
    if e.kind() == std::io::ErrorKind::NotFound {
        SnapshotError::BackendMissing { tool: "mount_apfs" }
    } else {
        SnapshotError::Io(e)
    }
}

/// Pull the `com.apple.TimeMachine.<timestamp>.local` stamp out of
/// tmutil's free-form stdout. tmutil's one documented invariant is
/// that it always echoes the new snapshot name on success.
fn extract_timestamp(stdout: &str) -> Option<String> {
    for line in stdout.lines() {
        if let Some(pos) = line.find("com.apple.TimeMachine.") {
            // After the prefix, the next `.` starts `.local` etc.
            let rest = &line[pos + "com.apple.TimeMachine.".len()..];
            if let Some(end) = rest.find('.') {
                return Some(rest[..end].to_string());
            }
        }
    }
    None
}

fn detect_device_for(path: &Path) -> Option<String> {
    let out = std::process::Command::new("/bin/df")
        .arg("-P")
        .arg(path)
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    let text = String::from_utf8(out.stdout).ok()?;
    // Last non-header line; first whitespace-delimited field.
    text.lines()
        .filter(|l| !l.starts_with("Filesystem"))
        .last()
        .and_then(|l| l.split_whitespace().next())
        .map(|s| s.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_timestamp_parses_tmutil_output() {
        let stdout = "Created local snapshot with date: 2026-04-22-010203\n\
                      Created local snapshot name: com.apple.TimeMachine.2026-04-22-010203.local\n";
        assert_eq!(
            extract_timestamp(stdout).as_deref(),
            Some("2026-04-22-010203")
        );
    }

    #[test]
    fn extract_timestamp_returns_none_on_failure_output() {
        assert!(extract_timestamp("tmutil: Local snapshots not supported").is_none());
    }
}
