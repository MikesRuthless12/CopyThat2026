//! Phase 38 — aggregate destination dedup + reflink fallback
//! ladder.
//!
//! [`try_dedup`] runs the four-stage ladder per file:
//!
//! 1. **Reflink** — block-clone via the Phase 6 reflink path. Same
//!    volume + reflink-capable filesystem (Btrfs / XFS-with-CoW /
//!    APFS / ReFS Dev Drive). Instant; no bytes copied; future
//!    edits trigger CoW so each side mutates independently.
//! 2. **Hardlink** — `std::fs::hard_link` on same-volume copies.
//!    Two directory entries pointing at the same inode. Fastest
//!    path of the four, but *both names share state* — editing
//!    either side affects the other. Off by default; opt-in via
//!    [`DedupOptions::hardlink_policy`]. Surfaced in the UI with
//!    a yellow warning badge.
//! 3. **Chunk share** — when the Phase 27 chunk store is enabled,
//!    write the file as references into the chunk store so any
//!    repeating chunks dedup transparently. The runtime gate is
//!    in `copythat-chunk`; this crate just signals the intent.
//! 4. **Byte copy** — fallback. Always succeeds (modulo I/O
//!    errors). The engine's existing `copy_file` path handles it.
//!
//! [`DedupOptions::mode`] selects how aggressive the ladder is:
//!
//! - `AutoLadder`: try reflink → hardlink (if opted in) →
//!   chunk-share (if enabled) → copy. Default.
//! - `ReflinkOnly`: try reflink; fall through to copy if it fails.
//!   Doesn't touch hardlink or chunk-share.
//! - `HardlinkAggressive`: try reflink, then hardlink even when
//!   the file is writable. Surfaces extra warnings in the UI;
//!   useful for pure-archive workflows where the user knows the
//!   tree is read-only after the copy.
//! - `None`: skip the ladder entirely. Engine takes the regular
//!   copy path.

use std::fs;
use std::io;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::helpers::{supports_reflink, volume_id};

/// Which leg of the ladder ended up handling the file.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DedupStrategy {
    Reflink,
    Hardlink,
    ChunkShare,
    Copy,
    /// The ladder didn't run at all — caller skipped via
    /// [`DedupMode::None`] or the entry wasn't a regular file.
    Skipped,
}

/// What the runner expects the ladder to attempt.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DedupMode {
    #[default]
    /// Try every leg in order.
    AutoLadder,
    /// Reflink only — fall through to byte copy on failure. Skips
    /// hardlink + chunk-share entirely.
    ReflinkOnly,
    /// Reflink, then hardlink (even on writable files). Skips
    /// chunk-share. Use for read-only archives.
    HardlinkAggressive,
    /// Disable the ladder. Engine runs the standard copy path.
    None,
}

/// When does the dedup ladder allow hardlinking?
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum HardlinkPolicy {
    /// Don't hardlink at all (default — safest, since hardlinks
    /// share state).
    #[default]
    Never,
    /// Hardlink only when the source file is read-only.
    ReadOnlyOnly,
    /// Always hardlink when the volumes match.
    Always,
}

/// Knobs the runner hands to [`try_dedup`].
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DedupOptions {
    pub mode: DedupMode,
    pub hardlink_policy: HardlinkPolicy,
    /// Whether the chunk store is enabled and ready. Caller drives
    /// this from the live `ChunkStoreSettings`. When `false`, the
    /// `ChunkShare` leg is skipped.
    pub chunk_share_enabled: bool,
}

/// Result of one [`try_dedup`] invocation. `bytes_saved` is the
/// number of bytes the destination *would* have consumed under a
/// straight copy that the ladder avoided. `Reflink` saves the full
/// file size at the volume level (the new directory entry shares
/// extents); `Hardlink` saves the same; `ChunkShare` saves the
/// total minus the unique chunks; `Copy` and `Skipped` save 0.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct DedupOutcome {
    pub strategy: DedupStrategy,
    pub bytes_saved: u64,
}

/// Run the dedup ladder for one source/destination pair. Returns
/// the strategy that handled the file. Callers use this in place
/// of the engine's regular `copy_file` for the per-file body of a
/// `dedup_mode`-enabled tree copy.
///
/// On `DedupStrategy::Copy` / `Skipped`, **the caller is responsible
/// for the actual byte-copy** — `try_dedup` only writes the
/// destination when it can take a metadata-level shortcut (reflink
/// or hardlink). Decoupling the byte-copy keeps the ladder
/// testable in isolation; the engine wires the regular `copy_file`
/// path on the fallthrough.
pub fn try_dedup(src: &Path, dst: &Path, opts: &DedupOptions) -> io::Result<DedupOutcome> {
    if opts.mode == DedupMode::None {
        return Ok(DedupOutcome {
            strategy: DedupStrategy::Skipped,
            bytes_saved: 0,
        });
    }

    let metadata = match fs::symlink_metadata(src) {
        Ok(m) => m,
        Err(e) if e.kind() == io::ErrorKind::NotFound => {
            return Err(e);
        }
        Err(e) => return Err(e),
    };
    if !metadata.is_file() {
        return Ok(DedupOutcome {
            strategy: DedupStrategy::Skipped,
            bytes_saved: 0,
        });
    }
    let bytes_total = metadata.len();

    // 1. Reflink leg.
    if matches!(
        opts.mode,
        DedupMode::AutoLadder | DedupMode::ReflinkOnly | DedupMode::HardlinkAggressive
    ) && volumes_match(src, dst)
        && filesystem_supports_reflink(dst)
        && try_reflink(src, dst).is_ok()
    {
        return Ok(DedupOutcome {
            strategy: DedupStrategy::Reflink,
            bytes_saved: bytes_total,
        });
    }

    // 2. Hardlink leg.
    if matches!(
        opts.mode,
        DedupMode::AutoLadder | DedupMode::HardlinkAggressive
    ) && volumes_match(src, dst)
    {
        let allowed = match (opts.mode, opts.hardlink_policy) {
            (DedupMode::HardlinkAggressive, _) => true,
            (DedupMode::AutoLadder, HardlinkPolicy::Never) => false,
            (DedupMode::AutoLadder, HardlinkPolicy::ReadOnlyOnly) => is_read_only(&metadata),
            (DedupMode::AutoLadder, HardlinkPolicy::Always) => true,
            _ => false,
        };
        if allowed {
            // Remove dst if it already exists; std::fs::hard_link
            // refuses to overwrite.
            let _ = fs::remove_file(dst);
            if fs::hard_link(src, dst).is_ok() {
                return Ok(DedupOutcome {
                    strategy: DedupStrategy::Hardlink,
                    bytes_saved: bytes_total,
                });
            }
        }
    }

    // 3. Chunk-share leg — runtime gate. The actual integration
    //    lives in the engine: when the chunk store is enabled the
    //    runner already routes the byte-copy through the chunk
    //    write path, which dedups by content. Here we surface the
    //    strategy so the UI can render the right badge — but ONLY
    //    when the engine has actually wired the chunk-store hook
    //    (`CopyOptions::chunk_store`). Today the engine does not
    //    consult `chunk_store`; reporting `ChunkShare` here would
    //    let the caller skip `copy_file`, leaving the destination
    //    file empty. Until the engine wiring lands, fall through to
    //    the byte-copy fallback so dst still gets the bytes.
    let _chunk_share_intent = opts.mode == DedupMode::AutoLadder && opts.chunk_share_enabled;

    // 4. Byte-copy fallback.
    Ok(DedupOutcome {
        strategy: DedupStrategy::Copy,
        bytes_saved: 0,
    })
}

fn volumes_match(src: &Path, dst: &Path) -> bool {
    match (volume_id(src), dst.parent().and_then(volume_id)) {
        (Some(a), Some(b)) => a == b,
        _ => false,
    }
}

fn filesystem_supports_reflink(dst: &Path) -> bool {
    // Probe the destination's parent — `dst` itself may not exist
    // yet at the moment we call this.
    let probe = dst.parent().unwrap_or(dst);
    matches!(supports_reflink(probe), Some(true))
}

fn try_reflink(src: &Path, dst: &Path) -> io::Result<()> {
    if dst.exists() {
        let _ = fs::remove_file(dst);
    }
    reflink_copy::reflink(src, dst)
}

#[cfg(unix)]
fn is_read_only(meta: &fs::Metadata) -> bool {
    use std::os::unix::fs::PermissionsExt;
    // Read-only by all classes — no write bit set anywhere.
    (meta.permissions().mode() & 0o222) == 0
}

#[cfg(windows)]
fn is_read_only(meta: &fs::Metadata) -> bool {
    meta.permissions().readonly()
}

#[cfg(not(any(unix, windows)))]
fn is_read_only(meta: &fs::Metadata) -> bool {
    meta.permissions().readonly()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn mode_none_skips_unconditionally() {
        let dir = tempdir().unwrap();
        let src = dir.path().join("a.bin");
        let dst = dir.path().join("b.bin");
        fs::write(&src, b"hello").unwrap();
        let opts = DedupOptions {
            mode: DedupMode::None,
            ..DedupOptions::default()
        };
        let outcome = try_dedup(&src, &dst, &opts).unwrap();
        assert_eq!(outcome.strategy, DedupStrategy::Skipped);
        assert_eq!(outcome.bytes_saved, 0);
        // No destination written.
        assert!(!dst.exists());
    }

    #[test]
    fn auto_ladder_falls_through_to_copy_on_unsupported_volume() {
        // tempdir is on the OS temp volume — generally NTFS on
        // Windows / ext4 on Linux / APFS on macOS. Reflink may or
        // may not be supported. The ladder should at minimum
        // surface a Copy outcome rather than panic; if reflink
        // happens (APFS / Btrfs), accept that too.
        let dir = tempdir().unwrap();
        let src = dir.path().join("src.bin");
        let dst = dir.path().join("dst.bin");
        fs::write(&src, vec![0u8; 1024]).unwrap();
        let opts = DedupOptions {
            mode: DedupMode::AutoLadder,
            hardlink_policy: HardlinkPolicy::Never,
            chunk_share_enabled: false,
        };
        let outcome = try_dedup(&src, &dst, &opts).unwrap();
        assert!(matches!(
            outcome.strategy,
            DedupStrategy::Reflink | DedupStrategy::Copy
        ));
    }

    #[test]
    fn hardlink_aggressive_uses_hardlink_on_writable_files() {
        let dir = tempdir().unwrap();
        let src = dir.path().join("src.bin");
        let dst = dir.path().join("dst.bin");
        fs::write(&src, b"hardlink-me").unwrap();
        let opts = DedupOptions {
            mode: DedupMode::HardlinkAggressive,
            hardlink_policy: HardlinkPolicy::Always,
            chunk_share_enabled: false,
        };
        let outcome = try_dedup(&src, &dst, &opts).unwrap();
        // Reflink may engage on capable filesystems; otherwise the
        // hardlink leg fires.
        assert!(matches!(
            outcome.strategy,
            DedupStrategy::Reflink | DedupStrategy::Hardlink
        ));
        // In either case the destination exists.
        assert!(dst.exists());
        let dst_bytes = fs::read(&dst).unwrap();
        assert_eq!(dst_bytes, b"hardlink-me");
    }

    #[test]
    fn dedup_outcome_round_trips_through_serde() {
        let outcome = DedupOutcome {
            strategy: DedupStrategy::Reflink,
            bytes_saved: 1_048_576,
        };
        let s = serde_json::to_string(&outcome).unwrap();
        let back: DedupOutcome = serde_json::from_str(&s).unwrap();
        assert_eq!(outcome, back);
    }

    #[test]
    fn options_round_trip_through_serde() {
        let opts = DedupOptions {
            mode: DedupMode::AutoLadder,
            hardlink_policy: HardlinkPolicy::ReadOnlyOnly,
            chunk_share_enabled: true,
        };
        let s = serde_json::to_string(&opts).unwrap();
        let back: DedupOptions = serde_json::from_str(&s).unwrap();
        assert_eq!(opts, back);
    }
}
