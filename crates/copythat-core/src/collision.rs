//! Destination-already-exists policy.
//!
//! `copy_tree` / `move_tree` consult a `CollisionPolicy` before each
//! file. All decisions except `Prompt` are local and synchronous.
//! `Prompt` emits a `CopyEvent::Collision` and parks the per-file
//! task on a oneshot reply.

use std::path::{Path, PathBuf};

use tokio::sync::{mpsc, oneshot};

use crate::event::{Collision, CollisionResolution, CopyEvent};

/// What to do when `dst` already exists.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum CollisionPolicy {
    /// Leave `dst` alone; do not copy this file. The pre-Phase-8
    /// default and the one `Default` resolves to.
    #[default]
    Skip,
    /// Truncate `dst` and overwrite.
    Overwrite,
    /// Overwrite only if `src`'s mtime is strictly newer than `dst`'s.
    OverwriteIfNewer,
    /// Insert a unique suffix (e.g. `foo.txt` → `foo (1).txt`) so both
    /// files exist at the destination.
    KeepBoth,
    /// Use this filename instead of the original. No directory part —
    /// the file still lands in the same destination directory.
    Rename(String),
    /// Ask the caller via a `CopyEvent::Collision` event and wait on
    /// the returned oneshot.
    Prompt,
}

/// The engine's internal decision after consulting the policy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Decision {
    /// Skip this file. Count it in `skipped`; keep going.
    Skip,
    /// Copy to this destination path, truncating if it exists.
    Write(PathBuf),
    /// Abort the whole tree walk.
    Abort,
}

pub(crate) async fn resolve(
    policy: &CollisionPolicy,
    src: &Path,
    dst: &Path,
    events: &mpsc::Sender<CopyEvent>,
) -> Decision {
    if !path_exists(dst).await {
        return Decision::Write(dst.to_path_buf());
    }
    match policy {
        CollisionPolicy::Skip => Decision::Skip,
        CollisionPolicy::Overwrite => Decision::Write(dst.to_path_buf()),
        CollisionPolicy::OverwriteIfNewer => match source_is_newer(src, dst).await {
            Some(true) => Decision::Write(dst.to_path_buf()),
            _ => Decision::Skip,
        },
        CollisionPolicy::KeepBoth => match keep_both_path(dst).await {
            Some(fresh) => Decision::Write(fresh),
            None => Decision::Skip,
        },
        CollisionPolicy::Rename(name) => {
            let fresh = dst
                .parent()
                .map(|p| p.join(name))
                .unwrap_or_else(|| PathBuf::from(name));
            Decision::Write(fresh)
        }
        CollisionPolicy::Prompt => prompt(src, dst, events).await,
    }
}

async fn path_exists(p: &Path) -> bool {
    tokio::fs::symlink_metadata(p).await.is_ok()
}

async fn source_is_newer(src: &Path, dst: &Path) -> Option<bool> {
    let (sm, dm) = tokio::try_join!(tokio::fs::metadata(src), tokio::fs::metadata(dst)).ok()?;
    let st = sm.modified().ok()?;
    let dt = dm.modified().ok()?;
    Some(st > dt)
}

/// Produce a fresh destination path by appending " (1)", " (2)", ...
/// before the extension until a free slot is found. Gives up at 10_000.
async fn keep_both_path(dst: &Path) -> Option<PathBuf> {
    let parent = dst.parent()?;
    let stem = dst.file_stem()?.to_string_lossy().into_owned();
    let ext = dst
        .extension()
        .map(|e| e.to_string_lossy().into_owned())
        .unwrap_or_default();
    for n in 1..10_000 {
        let name = if ext.is_empty() {
            format!("{stem} ({n})")
        } else {
            format!("{stem} ({n}).{ext}")
        };
        let candidate = parent.join(&name);
        if !path_exists(&candidate).await {
            return Some(candidate);
        }
    }
    None
}

async fn prompt(src: &Path, dst: &Path, events: &mpsc::Sender<CopyEvent>) -> Decision {
    let (tx, rx) = oneshot::channel();
    let collision = Collision::new(src.to_path_buf(), dst.to_path_buf(), tx);
    if events.send(CopyEvent::Collision(collision)).await.is_err() {
        // No consumer — act like Skip rather than hang forever.
        return Decision::Skip;
    }
    match rx.await {
        Ok(CollisionResolution::Overwrite) => Decision::Write(dst.to_path_buf()),
        Ok(CollisionResolution::Rename(name)) => {
            let fresh = dst
                .parent()
                .map(|p| p.join(name))
                .unwrap_or_else(|| PathBuf::from("renamed"));
            Decision::Write(fresh)
        }
        Ok(CollisionResolution::Skip) | Err(_) => Decision::Skip,
        Ok(CollisionResolution::Abort) => Decision::Abort,
    }
}
