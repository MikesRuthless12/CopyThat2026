//! Events emitted during a copy / move, plus the final reports.
//!
//! `CopyEvent` covers both single-file (Phase 1) and tree / collision
//! (Phase 2) flows. Per-file `Started` / `Progress` / `Completed`
//! events continue to fire inside a tree copy; tree-level aggregates
//! are layered on top so a UI can paint both an overall progress bar
//! and a "current item" row without doing its own accounting.

use std::path::PathBuf;
use std::time::Duration;

use tokio::sync::oneshot;

use crate::error::CopyError;
use crate::options::ErrorAction;

/// A single event emitted on the `events` channel during a copy or
/// move. Dropped sends are tolerated: if the receiver disappears the
/// engine keeps working and stops reporting. Progress is advisory,
/// not load-bearing.
///
/// Marked `#[non_exhaustive]` so future phases can add variants
/// without breaking downstream `match` arms.
#[derive(Debug)]
#[non_exhaustive]
pub enum CopyEvent {
    // ---------- single file (Phase 1) ----------
    Started {
        src: PathBuf,
        dst: PathBuf,
        total_bytes: u64,
    },
    Progress {
        bytes: u64,
        total: u64,
        rate_bps: u64,
    },
    Paused,
    Resumed,
    Completed {
        bytes: u64,
        duration: Duration,
        rate_bps: u64,
    },
    Failed {
        err: CopyError,
    },
    // ---------- verify pipeline (Phase 3) ----------
    VerifyStarted {
        /// Algorithm short name (e.g. `sha256`, `blake3`).
        algorithm: &'static str,
        /// Destination total byte count the verify pass will re-read.
        total_bytes: u64,
    },
    VerifyProgress {
        bytes: u64,
        total: u64,
        rate_bps: u64,
    },
    VerifyCompleted {
        algorithm: &'static str,
        src_hex: String,
        dst_hex: String,
        duration: Duration,
    },
    VerifyFailed {
        algorithm: &'static str,
        src_hex: String,
        dst_hex: String,
    },
    // ---------- tree-level aggregates (Phase 2) ----------
    //
    // Phase 16: emit once the tree walk has started but *before*
    // the walker has returned. Fires periodically with running
    // totals so a whole-drive enumeration shows live progress
    // instead of a silent several-minute wait. `TreeStarted` still
    // fires afterwards with the final counts.
    TreeEnumerating {
        files_so_far: u64,
        bytes_so_far: u64,
    },
    TreeStarted {
        root_src: PathBuf,
        root_dst: PathBuf,
        total_files: u64,
        total_bytes: u64,
    },
    TreeProgress {
        files_done: u64,
        files_total: u64,
        bytes_done: u64,
        bytes_total: u64,
        rate_bps: u64,
    },
    TreeCompleted {
        files: u64,
        bytes: u64,
        duration: Duration,
        rate_bps: u64,
    },
    // ---------- collision (Phase 2) ----------
    Collision(Collision),
    // ---------- error UX (Phase 8) ----------
    /// A non-fatal per-file error inside a tree copy. Emitted when
    /// `TreeOptions::on_error` is `Skip` or when `RetryN` exhausts
    /// its attempts; the tree continues. The `errored` counter on
    /// `TreeReport` ticks for each of these.
    FileError {
        err: CopyError,
    },
    /// Interactive error prompt. Emitted when `TreeOptions::on_error`
    /// is `Ask`. Consumers reply on the enclosed oneshot with
    /// `Retry` / `Skip` / `Abort`. If the channel drops without a
    /// reply the engine treats it as `Skip` (mirrors the Collision
    /// fallback).
    ErrorPrompt(ErrorPrompt),
    /// Phase 19b — the engine hit an `ERROR_SHARING_VIOLATION` /
    /// `EBUSY` / `NSFileLockingError` reading `original`, the
    /// `on_locked` policy was `Snapshot`, and the hook minted a
    /// filesystem snapshot. The UI renders a "📷 Reading from <kind>
    /// snapshot of <original_root>" badge on the active row until
    /// the file finishes.
    SnapshotCreated {
        /// Wire string: `"vss"` / `"zfs"` / `"btrfs"` / `"apfs"`.
        kind: &'static str,
        /// The original live-source path the engine was trying to
        /// open before it fell through to the snapshot.
        original: PathBuf,
        /// Root of the snapshot mount — e.g.
        /// `\\?\GLOBALROOT\Device\HarddiskVolumeShadowCopy5` for VSS
        /// or `/mnt/pool/.zfs/snapshot/<name>` for ZFS.
        snap_mount: PathBuf,
    },
    /// Phase 20 — the engine asked the journal for a `ResumePlan`,
    /// the journal said `Resume { offset, src_hash_at_offset }`, but
    /// the prefix re-hash of the existing destination did not match.
    /// Engine falls back to a full restart (truncate + rewrite from
    /// byte 0); the UI surfaces the reason in a row toast so the
    /// user knows why a "resumable" copy went all the way back.
    ResumeAborted {
        /// Stable wire string: `"prefix-hash-mismatch"` /
        /// `"dst-shrunk"` / `"checkpoint-corrupt"`. New variants
        /// land here as additional resume paths discover their own
        /// abort modes.
        reason: &'static str,
        /// Best-effort offset where the mismatch was first observed.
        /// `0` if the mismatch was outside a per-byte compare (e.g.
        /// the dst length was already shorter than the journal
        /// expected).
        offset: u64,
    },
    /// Phase 23 — the destination filesystem doesn't support sparse
    /// files. Engine emits this once per sparse source and falls back
    /// to a dense copy (every hole is written out as zeros). `dst_fs`
    /// is the short name returned by
    /// `copythat-platform::filesystem_name` (`"exFAT"`, `"FAT32"`,
    /// `"HFS+"`, `"unknown"`). The UI surfaces a one-shot toast so
    /// the user knows the destination will be larger than the source.
    SparsenessNotSupported {
        dst_fs: &'static str,
    },
    /// Phase 24 — the destination filesystem couldn't accept some of
    /// the source's foreign metadata streams (e.g. macOS resource
    /// fork landing on a Linux ext4 share, Windows ADS landing on a
    /// FAT32 USB stick). The engine fell through to an
    /// `._<filename>` AppleDouble sidecar so the data survives the
    /// trip and can be re-applied when the file lands back on a
    /// capable FS. `ext` is the source file's extension
    /// (lowercase, no leading dot — `"docx"`, `"jpg"`) so the UI
    /// can render a per-row info badge.
    MetaTranslatedToAppleDouble {
        ext: String,
    },
    /// Phase 27 — content-defined chunk store reported dedup savings
    /// for this copy.
    ///
    /// Emitted once after the copy's chunk-ingest pass completes.
    /// `savings_bytes` is the byte count that dedup'd against chunks
    /// the store already had (either from an earlier file in the
    /// same job or from a prior job). The UI / history sums these
    /// into the "Saved N.NN GiB via chunk dedup" badge.
    ChunkStoreSavings {
        savings_bytes: u64,
    },
    /// Phase 30 — the cross-platform path translator applied Unicode
    /// NFC/NFD normalization and the destination filename byte
    /// sequence differs from the source's. `from` is the
    /// pre-normalization filename (as an absolute path with the
    /// source root intact); `to` is the post-normalization
    /// destination path. UI surfaces a one-shot toast so the user
    /// understands why a round-trip re-copy may show a different
    /// byte sequence.
    UnicodeRenormalized {
        from: PathBuf,
        to: PathBuf,
    },
}

impl Clone for CopyEvent {
    fn clone(&self) -> Self {
        match self {
            CopyEvent::Started {
                src,
                dst,
                total_bytes,
            } => CopyEvent::Started {
                src: src.clone(),
                dst: dst.clone(),
                total_bytes: *total_bytes,
            },
            CopyEvent::Progress {
                bytes,
                total,
                rate_bps,
            } => CopyEvent::Progress {
                bytes: *bytes,
                total: *total,
                rate_bps: *rate_bps,
            },
            CopyEvent::Paused => CopyEvent::Paused,
            CopyEvent::Resumed => CopyEvent::Resumed,
            CopyEvent::Completed {
                bytes,
                duration,
                rate_bps,
            } => CopyEvent::Completed {
                bytes: *bytes,
                duration: *duration,
                rate_bps: *rate_bps,
            },
            CopyEvent::Failed { err } => CopyEvent::Failed { err: err.clone() },
            CopyEvent::VerifyStarted {
                algorithm,
                total_bytes,
            } => CopyEvent::VerifyStarted {
                algorithm,
                total_bytes: *total_bytes,
            },
            CopyEvent::VerifyProgress {
                bytes,
                total,
                rate_bps,
            } => CopyEvent::VerifyProgress {
                bytes: *bytes,
                total: *total,
                rate_bps: *rate_bps,
            },
            CopyEvent::VerifyCompleted {
                algorithm,
                src_hex,
                dst_hex,
                duration,
            } => CopyEvent::VerifyCompleted {
                algorithm,
                src_hex: src_hex.clone(),
                dst_hex: dst_hex.clone(),
                duration: *duration,
            },
            CopyEvent::VerifyFailed {
                algorithm,
                src_hex,
                dst_hex,
            } => CopyEvent::VerifyFailed {
                algorithm,
                src_hex: src_hex.clone(),
                dst_hex: dst_hex.clone(),
            },
            CopyEvent::TreeEnumerating {
                files_so_far,
                bytes_so_far,
            } => CopyEvent::TreeEnumerating {
                files_so_far: *files_so_far,
                bytes_so_far: *bytes_so_far,
            },
            CopyEvent::TreeStarted {
                root_src,
                root_dst,
                total_files,
                total_bytes,
            } => CopyEvent::TreeStarted {
                root_src: root_src.clone(),
                root_dst: root_dst.clone(),
                total_files: *total_files,
                total_bytes: *total_bytes,
            },
            CopyEvent::TreeProgress {
                files_done,
                files_total,
                bytes_done,
                bytes_total,
                rate_bps,
            } => CopyEvent::TreeProgress {
                files_done: *files_done,
                files_total: *files_total,
                bytes_done: *bytes_done,
                bytes_total: *bytes_total,
                rate_bps: *rate_bps,
            },
            CopyEvent::TreeCompleted {
                files,
                bytes,
                duration,
                rate_bps,
            } => CopyEvent::TreeCompleted {
                files: *files,
                bytes: *bytes,
                duration: *duration,
                rate_bps: *rate_bps,
            },
            // Collision carries a oneshot sender; it can't be cloned.
            // Broadcast subscribers only see a placeholder.
            CopyEvent::Collision(_) => CopyEvent::Collision(Collision::placeholder_for_clone()),
            CopyEvent::FileError { err } => CopyEvent::FileError { err: err.clone() },
            // ErrorPrompt carries a oneshot sender; same pattern as
            // Collision — broadcast subscribers see a placeholder.
            CopyEvent::ErrorPrompt(_) => {
                CopyEvent::ErrorPrompt(ErrorPrompt::placeholder_for_clone())
            }
            CopyEvent::SnapshotCreated {
                kind,
                original,
                snap_mount,
            } => CopyEvent::SnapshotCreated {
                kind,
                original: original.clone(),
                snap_mount: snap_mount.clone(),
            },
            CopyEvent::ResumeAborted { reason, offset } => CopyEvent::ResumeAborted {
                reason,
                offset: *offset,
            },
            CopyEvent::SparsenessNotSupported { dst_fs } => {
                CopyEvent::SparsenessNotSupported { dst_fs }
            }
            CopyEvent::MetaTranslatedToAppleDouble { ext } => {
                CopyEvent::MetaTranslatedToAppleDouble { ext: ext.clone() }
            }
            CopyEvent::ChunkStoreSavings { savings_bytes } => CopyEvent::ChunkStoreSavings {
                savings_bytes: *savings_bytes,
            },
            CopyEvent::UnicodeRenormalized { from, to } => CopyEvent::UnicodeRenormalized {
                from: from.clone(),
                to: to.clone(),
            },
        }
    }
}

/// Final success record returned by `copy_file` and `move_file`.
#[derive(Debug, Clone)]
pub struct CopyReport {
    pub src: PathBuf,
    pub dst: PathBuf,
    pub bytes: u64,
    pub duration: Duration,
    pub rate_bps: u64,
}

/// Final success record returned by `copy_tree` and `move_tree`.
#[derive(Debug, Clone)]
pub struct TreeReport {
    pub root_src: PathBuf,
    pub root_dst: PathBuf,
    pub files: u64,
    pub bytes: u64,
    pub duration: Duration,
    pub rate_bps: u64,
    /// Files the caller asked us to skip (via collision policy).
    pub skipped: u64,
    /// Files that failed their per-file copy and were recorded via
    /// `FileError` (policy = `Skip` / exhausted `RetryN`). Zero when
    /// `on_error` is `Abort` — that path bails before the counter
    /// can tick.
    pub errored: u64,
}

/// Destination-already-exists prompt. Consumers reply on the enclosed
/// oneshot to resolve. If the sender is dropped without replying, the
/// engine treats the collision as a Skip.
#[derive(Debug)]
pub struct Collision {
    pub src: PathBuf,
    pub dst: PathBuf,
    /// Reply channel. `None` only on cloned placeholders — cloned
    /// events can't drive the engine forward, so a subscriber that
    /// saw the clone must also be attached to the original mpsc to
    /// actually resolve the collision.
    pub resolver: Option<oneshot::Sender<CollisionResolution>>,
}

impl Collision {
    pub(crate) fn new(
        src: PathBuf,
        dst: PathBuf,
        resolver: oneshot::Sender<CollisionResolution>,
    ) -> Self {
        Self {
            src,
            dst,
            resolver: Some(resolver),
        }
    }

    fn placeholder_for_clone() -> Self {
        Self {
            src: PathBuf::new(),
            dst: PathBuf::new(),
            resolver: None,
        }
    }

    /// Resolve the collision. Consumes the oneshot. No-op on a cloned
    /// placeholder.
    pub fn resolve(mut self, resolution: CollisionResolution) {
        if let Some(tx) = self.resolver.take() {
            let _ = tx.send(resolution);
        }
    }
}

/// Decision returned by the collision prompter.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CollisionResolution {
    Skip,
    Overwrite,
    /// Use this final filename instead (no directory component; stays
    /// in the same parent as the original destination).
    Rename(String),
    /// Abort the whole tree operation.
    Abort,
}

/// Interactive error prompt (Phase 8). Emitted when
/// `TreeOptions::on_error == Ask` and a per-file copy fails.
/// Consumers reply on the `resolver` oneshot; dropping it without
/// a reply is equivalent to `ErrorAction::Skip`.
#[derive(Debug)]
pub struct ErrorPrompt {
    pub err: CopyError,
    /// Reply channel. `None` only on cloned placeholders — broadcast
    /// subscribers can't drive the engine forward.
    pub resolver: Option<oneshot::Sender<ErrorAction>>,
}

impl ErrorPrompt {
    pub(crate) fn new(err: CopyError, resolver: oneshot::Sender<ErrorAction>) -> Self {
        Self {
            err,
            resolver: Some(resolver),
        }
    }

    fn placeholder_for_clone() -> Self {
        // A placeholder carries a zeroed error; consumers that see a
        // cloned ErrorPrompt can't act on it anyway (no resolver).
        Self {
            err: CopyError {
                kind: crate::error::CopyErrorKind::IoOther,
                src: PathBuf::new(),
                dst: PathBuf::new(),
                raw_os_error: None,
                message: String::new(),
            },
            resolver: None,
        }
    }

    /// Resolve the prompt. Consumes the oneshot. No-op on a cloned
    /// placeholder.
    pub fn resolve(mut self, action: ErrorAction) {
        if let Some(tx) = self.resolver.take() {
            let _ = tx.send(action);
        }
    }
}
