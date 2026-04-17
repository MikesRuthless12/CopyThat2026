//! `shred_tree` — depth-first walk + per-file shred + directory removal.
//!
//! Two-phase, same shape as `copy_tree`:
//!   1. `spawn_blocking(walkdir)` to enumerate files (for totals) and
//!      directories (for bottom-up removal).
//!   2. Shred files one at a time (sequential — the device head is
//!      already under pressure from the multi-pass pattern, so
//!      parallelism would thrash seeks without improving throughput
//!      on any realistic medium), then remove now-empty directories
//!      in reverse-depth order.
//!
//! Symlinks are **unlinked, not followed** — shredding the target
//! file would scrub bytes the user didn't point us at. Broken symlinks
//! are unlinked too.

use std::path::{Path, PathBuf};
use std::time::Instant;

use copythat_core::CopyControl;
use tokio::sync::mpsc;

use crate::engine::shred_file;
use crate::error::{ShredError, ShredErrorKind};
use crate::event::{ShredEvent, ShredReport};
use crate::method::ShredMethod;

/// Securely shred every regular file under `root` and then remove the
/// directories (bottom-up). `root` itself is removed last.
pub async fn shred_tree(
    root: &Path,
    method: ShredMethod,
    ctrl: CopyControl,
    events: mpsc::Sender<ShredEvent>,
) -> Result<ShredReport, ShredError> {
    let root_buf = root.to_path_buf();
    let started_at = Instant::now();

    match shred_tree_inner(&root_buf, method, ctrl, &events, started_at).await {
        Ok(report) => Ok(report),
        Err(err) => {
            let _ = events.send(ShredEvent::Failed { err: err.clone() }).await;
            Err(err)
        }
    }
}

async fn shred_tree_inner(
    root: &Path,
    method: ShredMethod,
    ctrl: CopyControl,
    events: &mpsc::Sender<ShredEvent>,
    started_at: Instant,
) -> Result<ShredReport, ShredError> {
    let meta = tokio::fs::symlink_metadata(root)
        .await
        .map_err(|e| ShredError::from_io(root, e))?;

    // Caller passed a plain file or a symlink — delegate / reject.
    if meta.file_type().is_symlink() {
        return Err(ShredError::bad_target(
            root,
            "shred_tree refuses to follow a symlink root; shred the target or the link \
             directly via shred_file",
        ));
    }
    if meta.is_file() {
        // Convenience: route single-file trees through the simple path.
        return shred_file(root, method, ctrl, events.clone()).await;
    }
    if !meta.is_dir() {
        return Err(ShredError::bad_target(
            root,
            "shred_tree target is neither a regular file nor a directory",
        ));
    }

    // Enumerate — depth-first, files first (so totals match), dirs
    // captured separately in reverse-depth order for cleanup.
    let (files, dirs, total_bytes) = {
        let root_for_walk = root.to_path_buf();
        tokio::task::spawn_blocking(move || enumerate(&root_for_walk))
            .await
            .map_err(|_| {
                ShredError::bad_target(
                    &PathBuf::new(),
                    "walkdir task panicked while enumerating shred tree",
                )
            })?
    };

    let total_files = files.len() as u64;
    let _ = events
        .send(ShredEvent::TreeStarted {
            root: root.to_path_buf(),
            total_files,
            total_bytes,
        })
        .await;

    // Per-file shred, sequential. Aggregate progress is reported in
    // bytes-of-payload × 1 (we don't multiply by pass count — the
    // UI cares about "how much of the user's data have we wiped",
    // not raw I/O volume).
    let mut files_done: u64 = 0;
    let mut bytes_done: u64 = 0;
    let passes = method.passes().len();
    let mut per_file_bytes: u64 = 0;

    for file_entry in &files {
        if ctrl.is_cancelled() {
            return Err(ShredError::cancelled(root));
        }

        // Symlinks inside the tree get unlinked, never followed.
        if file_entry.is_symlink {
            if let Err(e) = tokio::fs::remove_file(&file_entry.path).await {
                if e.kind() != std::io::ErrorKind::NotFound {
                    return Err(ShredError::from_io(&file_entry.path, e));
                }
            }
            files_done += 1;
            let _ = events
                .send(ShredEvent::TreeProgress {
                    files_done,
                    files_total: total_files,
                    bytes_done,
                    bytes_total: total_bytes,
                })
                .await;
            continue;
        }

        match shred_file(&file_entry.path, method, ctrl.clone(), events.clone()).await {
            Ok(report) => {
                files_done += 1;
                bytes_done = bytes_done.saturating_add(report.bytes_per_pass);
                per_file_bytes = per_file_bytes.saturating_add(report.bytes_per_pass);
            }
            Err(err) if err.kind == ShredErrorKind::NotFound => {
                // Race: file disappeared between walk and shred. Skip.
                files_done += 1;
            }
            Err(err) => return Err(err),
        }

        let _ = events
            .send(ShredEvent::TreeProgress {
                files_done,
                files_total: total_files,
                bytes_done,
                bytes_total: total_bytes,
            })
            .await;
    }

    // Directories in deepest-first order so children disappear before
    // parents. `walkdir(contents_first=true)` already gives us that
    // order, so we iterate `dirs` as-is. Swallow NotFound — a racing
    // external writer isn't our problem to guarantee away; shred is a
    // security operation, not a transactional delete.
    for dir in &dirs {
        if ctrl.is_cancelled() {
            return Err(ShredError::cancelled(root));
        }
        match tokio::fs::remove_dir(dir).await {
            Ok(()) => {}
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
            Err(e) => return Err(ShredError::from_io(dir, e)),
        }
    }

    // Finally remove the root.
    match tokio::fs::remove_dir(root).await {
        Ok(()) => {}
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
        Err(e) => return Err(ShredError::from_io(root, e)),
    }

    let duration = started_at.elapsed();
    let _ = events
        .send(ShredEvent::TreeCompleted {
            root: root.to_path_buf(),
            files: total_files,
            bytes: bytes_done,
            duration,
        })
        .await;

    Ok(ShredReport {
        path: root.to_path_buf(),
        method,
        passes,
        bytes_per_pass: per_file_bytes,
        files: total_files,
        duration,
    })
}

#[derive(Debug)]
struct FileEntry {
    path: PathBuf,
    is_symlink: bool,
}

/// Depth-first (post-order) walk of `root`. Returns every regular file
/// (and symlink) in discovery order plus every directory in
/// deepest-first order so the caller can remove them one-by-one.
/// Total byte count is the sum of regular file sizes; symlinks
/// contribute 0.
fn enumerate(root: &Path) -> (Vec<FileEntry>, Vec<PathBuf>, u64) {
    let mut files: Vec<FileEntry> = Vec::new();
    let mut dirs: Vec<PathBuf> = Vec::new();
    let mut total_bytes: u64 = 0;

    for entry in walkdir::WalkDir::new(root)
        .follow_links(false)
        .contents_first(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let meta = match entry.metadata() {
            Ok(m) => m,
            Err(_) => continue,
        };
        let file_type = meta.file_type();
        if file_type.is_dir() {
            // Skip the root itself in this loop; the outer function
            // removes it at the end.
            if entry.path() != root {
                dirs.push(entry.path().to_path_buf());
            }
        } else if file_type.is_file() {
            total_bytes = total_bytes.saturating_add(meta.len());
            files.push(FileEntry {
                path: entry.path().to_path_buf(),
                is_symlink: false,
            });
        } else if file_type.is_symlink() {
            files.push(FileEntry {
                path: entry.path().to_path_buf(),
                is_symlink: true,
            });
        }
        // Other types (devices, FIFOs, sockets) are silently skipped.
    }

    // `contents_first` yields leaves before their parents, so `dirs`
    // is already in deepest-first order — exactly what the outer
    // caller's removal sweep wants.
    (files, dirs, total_bytes)
}
