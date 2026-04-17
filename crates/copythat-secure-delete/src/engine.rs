//! `shred_file` — the per-file overwrite engine.
//!
//! Shape:
//!  1. Stat the file, emit `Started` (and `SsdAdvisory` when the probe
//!     reports an SSD).
//!  2. For each pass in the chosen method:
//!     a. Rewind the file to offset 0.
//!     b. Stream the pass pattern across the file length in 1 MiB
//!     chunks.
//!     c. `sync_all` to flush page cache -> medium.
//!     d. If the pass has `verify`, re-read and compare.
//!     e. Emit `PassCompleted`.
//!  3. Truncate the file to zero bytes.
//!  4. Rename to a random hex name in the same directory (scrubs the
//!     filename from the directory entry where the filesystem allows).
//!  5. Unlink.
//!
//! Between every buffer the loop cooperatively checks `CopyControl`
//! for pause / resume / cancel — same protocol as `copythat_core`.

use std::io::SeekFrom;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use copythat_core::CopyControl;
use rand::TryRngCore;
use rand::rngs::OsRng;
use tokio::fs::{File, OpenOptions};
use tokio::io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt};
use tokio::sync::mpsc;

use crate::error::ShredError;
use crate::event::{ShredEvent, ShredReport};
use crate::method::{CHUNK_SIZE, ShredMethod};
use crate::pattern::PassPattern;
use crate::purge;
use crate::ssd;

const PROGRESS_MIN_BYTES: u64 = 16 * 1024;
const PROGRESS_MIN_INTERVAL: Duration = Duration::from_millis(50);

/// Fluent message key emitted alongside [`ShredEvent::SsdAdvisory`].
pub(crate) const SSD_ADVISORY_KEY: &str = "shred-ssd-advisory";

/// Securely delete a single regular file.
///
/// `path` must point at a regular file — directories and symlinks are
/// rejected with [`ShredErrorKind::BadTarget`](crate::error::ShredErrorKind::BadTarget).
/// Use [`shred_tree`](crate::shred_tree) for directories.
pub async fn shred_file(
    path: &Path,
    method: ShredMethod,
    ctrl: CopyControl,
    events: mpsc::Sender<ShredEvent>,
) -> Result<ShredReport, ShredError> {
    let path_buf = path.to_path_buf();
    let started_at = Instant::now();

    match shred_file_inner(&path_buf, method, ctrl, &events, started_at).await {
        Ok(report) => Ok(report),
        Err(err) => {
            let _ = events.send(ShredEvent::Failed { err: err.clone() }).await;
            Err(err)
        }
    }
}

async fn shred_file_inner(
    path: &Path,
    method: ShredMethod,
    ctrl: CopyControl,
    events: &mpsc::Sender<ShredEvent>,
    started_at: Instant,
) -> Result<ShredReport, ShredError> {
    // Nist80088Purge is a special case: the per-file API can't honor
    // it. Refuse early with a message that points at the safe
    // alternative.
    if method == ShredMethod::Nist80088Purge && !purge::hardware_purge_available(path) {
        return Err(ShredError::purge_not_supported(path));
    }

    // Refuse symlinks and directories up front.
    let meta = tokio::fs::symlink_metadata(path)
        .await
        .map_err(|e| ShredError::from_io(path, e))?;
    let file_type = meta.file_type();
    if file_type.is_symlink() {
        return Err(ShredError::bad_target(
            path,
            "shred_file refuses to follow symlinks; pass the target directly or use shred_tree",
        ));
    }
    if file_type.is_dir() {
        return Err(ShredError::bad_target(
            path,
            "shred_file target is a directory; use shred_tree",
        ));
    }
    let file_size = meta.len();

    let passes = method.passes();

    let _ = events
        .send(ShredEvent::Started {
            path: path.to_path_buf(),
            method,
            total_passes: passes.len(),
            file_size,
        })
        .await;

    // SSD advisory — emitted once per operation, before the first pass.
    if let Some(true) = ssd::is_ssd(path) {
        let _ = events
            .send(ShredEvent::SsdAdvisory {
                path: path.to_path_buf(),
                localized_key: SSD_ADVISORY_KEY,
            })
            .await;
    }

    // Open for write, re-used across passes. On Unix we could push
    // O_SYNC into the open flags via `OpenOptionsExt::custom_flags`;
    // we intentionally don't, because `sync_all` after each pass
    // produces the same "this pass hit the medium" guarantee on every
    // platform (Linux, macOS, Windows) with no unsafe and no
    // platform-specific libc constants.
    let mut file = open_for_rewrite(path).await?;

    let mut was_paused = false;

    for (idx, pass) in passes.iter().enumerate() {
        // Cooperative check before rewinding: if cancelled between
        // passes, bail cleanly.
        if ctrl.is_cancelled() {
            return Err(ShredError::cancelled(path));
        }
        if ctrl.is_paused() {
            if !was_paused {
                let _ = events.send(ShredEvent::Paused).await;
                was_paused = true;
            }
            ctrl.wait_while_paused().await;
            if ctrl.is_cancelled() {
                return Err(ShredError::cancelled(path));
            }
            if was_paused {
                let _ = events.send(ShredEvent::Resumed).await;
                was_paused = false;
            }
        }

        let pass_index = idx + 1;
        let _ = events
            .send(ShredEvent::PassStarted {
                pass_index,
                total_passes: passes.len(),
                pattern: describe_pattern(pass),
            })
            .await;
        let pass_started = Instant::now();

        file.seek(SeekFrom::Start(0))
            .await
            .map_err(|e| ShredError::from_io(path, e))?;

        write_pass(
            &mut file,
            pass,
            file_size,
            pass_index,
            passes.len(),
            &ctrl,
            events,
            path,
            &mut was_paused,
        )
        .await?;

        // Flush the pass to the medium so any verify (or the next
        // pass) reads the bytes we just wrote, not cached ones.
        file.flush()
            .await
            .map_err(|e| ShredError::from_io(path, e))?;
        file.sync_all()
            .await
            .map_err(|e| ShredError::from_io(path, e))?;

        let verified = if pass.verify() {
            verify_pass(&mut file, pass, file_size, path, pass_index, &ctrl).await?;
            true
        } else {
            false
        };

        let _ = events
            .send(ShredEvent::PassCompleted {
                pass_index,
                total_passes: passes.len(),
                bytes: file_size,
                duration: pass_started.elapsed(),
                verified,
            })
            .await;
    }

    // Truncate to zero length so the free-list sees a zero-byte file
    // before we rename + unlink.
    file.set_len(0)
        .await
        .map_err(|e| ShredError::from_io(path, e))?;
    file.flush()
        .await
        .map_err(|e| ShredError::from_io(path, e))?;
    // Release the handle before renaming/unlinking. On Windows an open
    // handle with DELETE sharing could, but wouldn't normally, block
    // the later rename; dropping keeps the contract simple.
    drop(file);

    let scrubbed_path = rename_to_scrubbed(path).await?;
    tokio::fs::remove_file(&scrubbed_path)
        .await
        .map_err(|e| ShredError::from_io(&scrubbed_path, e))?;

    let duration = started_at.elapsed();
    let _ = events
        .send(ShredEvent::Completed {
            path: path.to_path_buf(),
            method,
            passes: passes.len(),
            bytes_per_pass: file_size,
            duration,
        })
        .await;

    Ok(ShredReport {
        path: path.to_path_buf(),
        method,
        passes: passes.len(),
        bytes_per_pass: file_size,
        files: 1,
        duration,
    })
}

/// Open `path` read-write without truncating. We keep the existing file
/// size and just overwrite byte-for-byte; `create(false)` so we never
/// accidentally create a new file when the caller passed a stale path.
async fn open_for_rewrite(path: &Path) -> Result<File, ShredError> {
    OpenOptions::new()
        .read(true)
        .write(true)
        .create(false)
        .open(path)
        .await
        .map_err(|e| ShredError::from_io(path, e))
}

#[allow(clippy::too_many_arguments)]
async fn write_pass(
    file: &mut File,
    pass: &PassPattern,
    file_size: u64,
    pass_index: usize,
    total_passes: usize,
    ctrl: &CopyControl,
    events: &mpsc::Sender<ShredEvent>,
    path: &Path,
    was_paused: &mut bool,
) -> Result<(), ShredError> {
    let mut buffer = vec![0u8; CHUNK_SIZE];
    fill_pattern(&mut buffer, pass);

    let mut written: u64 = 0;
    let started_at = Instant::now();
    let mut last_emit_at = started_at;
    let mut last_emit_bytes: u64 = 0;

    while written < file_size {
        if ctrl.is_cancelled() {
            return Err(ShredError::cancelled(path));
        }
        if ctrl.is_paused() {
            if !*was_paused {
                let _ = events.send(ShredEvent::Paused).await;
                *was_paused = true;
            }
            ctrl.wait_while_paused().await;
            if ctrl.is_cancelled() {
                return Err(ShredError::cancelled(path));
            }
            if *was_paused {
                let _ = events.send(ShredEvent::Resumed).await;
                *was_paused = false;
            }
            continue;
        }

        let remaining = file_size - written;
        let n = remaining.min(CHUNK_SIZE as u64) as usize;

        // For Random passes, re-fill the buffer each chunk so we don't
        // tile the same block repeatedly. Fixed / Tiled patterns stay
        // constant for the whole pass.
        if matches!(pass, PassPattern::Random { .. }) {
            OsRng
                .try_fill_bytes(&mut buffer[..n])
                .map_err(|e| ShredError::from_io(path, std::io::Error::other(e)))?;
        }

        file.write_all(&buffer[..n])
            .await
            .map_err(|e| ShredError::from_io(path, e))?;
        written += n as u64;

        let now = Instant::now();
        if written.saturating_sub(last_emit_bytes) >= PROGRESS_MIN_BYTES
            && now.duration_since(last_emit_at) >= PROGRESS_MIN_INTERVAL
        {
            let elapsed = now.duration_since(started_at);
            let rate = rate_bps(written, elapsed);
            let _ = events
                .send(ShredEvent::PassProgress {
                    pass_index,
                    total_passes,
                    bytes: written,
                    total: file_size,
                    rate_bps: rate,
                })
                .await;
            last_emit_at = now;
            last_emit_bytes = written;
        }
    }
    Ok(())
}

/// Re-read the file and confirm it matches the pass pattern.
/// Called only for passes whose `verify()` is true.
async fn verify_pass(
    file: &mut File,
    pass: &PassPattern,
    file_size: u64,
    path: &Path,
    pass_index: usize,
    ctrl: &CopyControl,
) -> Result<(), ShredError> {
    // Random-with-verify is a "best-effort" verify: we can't replay
    // the exact bytes OsRng produced. We settle for verifying the
    // file length and that the stream isn't all-zero (which would
    // indicate the write silently failed). For Fixed / Tiled we can
    // do an exact compare.
    file.seek(SeekFrom::Start(0))
        .await
        .map_err(|e| ShredError::from_io(path, e))?;

    let mut read_buf = vec![0u8; CHUNK_SIZE];
    let mut reference = vec![0u8; CHUNK_SIZE];
    let is_random = matches!(pass, PassPattern::Random { .. });
    if !is_random {
        fill_pattern(&mut reference, pass);
    }

    let mut read_total: u64 = 0;
    let mut saw_nonzero = false;
    while read_total < file_size {
        if ctrl.is_cancelled() {
            return Err(ShredError::cancelled(path));
        }
        let remaining = file_size - read_total;
        let n = remaining.min(CHUNK_SIZE as u64) as usize;
        file.read_exact(&mut read_buf[..n])
            .await
            .map_err(|e| ShredError::from_io(path, e))?;
        if is_random {
            if !saw_nonzero {
                saw_nonzero = read_buf[..n].iter().any(|b| *b != 0);
            }
        } else if read_buf[..n] != reference[..n] {
            return Err(ShredError::verify_failed(path, pass_index));
        }
        read_total += n as u64;
    }
    if is_random && file_size > 0 && !saw_nonzero {
        return Err(ShredError::verify_failed(path, pass_index));
    }
    Ok(())
}

fn fill_pattern(buffer: &mut [u8], pass: &PassPattern) {
    match pass {
        PassPattern::Fixed { byte, .. } => {
            buffer.fill(*byte);
        }
        PassPattern::Random { .. } => {
            // OsRng failing here means the OS refused to give us random
            // bytes — genuinely catastrophic. We'd rather crash loudly
            // than silently write predictable data during a shred.
            OsRng
                .try_fill_bytes(buffer)
                .expect("OsRng::try_fill_bytes failed during shred buffer seed");
        }
        PassPattern::Tiled { pattern, len, .. } => {
            let pat = &pattern[..(*len as usize).min(pattern.len())];
            if pat.is_empty() {
                buffer.fill(0);
                return;
            }
            for (i, slot) in buffer.iter_mut().enumerate() {
                *slot = pat[i % pat.len()];
            }
        }
    }
}

/// Stable short description of a pattern for events / logs.
fn describe_pattern(pass: &PassPattern) -> String {
    match pass {
        PassPattern::Fixed { byte: 0x00, .. } => "zero".to_string(),
        PassPattern::Fixed { byte: 0xFF, .. } => "one".to_string(),
        PassPattern::Fixed { byte, .. } => format!("fixed-0x{byte:02X}"),
        PassPattern::Random { .. } => "random".to_string(),
        PassPattern::Tiled { pattern, len, .. } => {
            let mut s = String::from("tiled-");
            for b in &pattern[..*len as usize] {
                s.push_str(&format!("{b:02X}"));
            }
            s
        }
    }
}

/// Rename the file to a random hex name inside its parent directory,
/// scrubbing the original filename from the directory entry on
/// filesystems where `rename` updates the dirent in-place (ext4, APFS,
/// NTFS — effectively all supported targets).
async fn rename_to_scrubbed(original: &Path) -> Result<PathBuf, ShredError> {
    let parent = original
        .parent()
        .ok_or_else(|| ShredError::bad_target(original, "shred target has no parent directory"))?;

    // Try up to 8 candidate names in case of collision (vanishingly
    // unlikely with 16 random hex chars, but cheap to guard against).
    for _ in 0..8 {
        let mut buf = [0u8; 8];
        OsRng
            .try_fill_bytes(&mut buf)
            .map_err(|e| ShredError::from_io(original, std::io::Error::other(e)))?;
        let name: String = buf.iter().map(|b| format!("{b:02x}")).collect();
        let candidate = parent.join(name);
        match tokio::fs::rename(original, &candidate).await {
            Ok(()) => return Ok(candidate),
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => continue,
            Err(e) => return Err(ShredError::from_io(original, e)),
        }
    }
    Err(ShredError::bad_target(
        original,
        "exhausted rename attempts while scrubbing filename",
    ))
}

fn rate_bps(bytes: u64, elapsed: Duration) -> u64 {
    let secs = elapsed.as_secs_f64();
    if secs <= 0.0 {
        return 0;
    }
    (bytes as f64 / secs) as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn describe_pattern_roundtrip() {
        assert_eq!(
            describe_pattern(&PassPattern::Fixed {
                byte: 0x00,
                verify: false
            }),
            "zero"
        );
        assert_eq!(
            describe_pattern(&PassPattern::Fixed {
                byte: 0xFF,
                verify: false
            }),
            "one"
        );
        assert_eq!(
            describe_pattern(&PassPattern::Fixed {
                byte: 0xAA,
                verify: false
            }),
            "fixed-0xAA"
        );
        assert_eq!(
            describe_pattern(&PassPattern::Random { verify: false }),
            "random"
        );
        assert_eq!(
            describe_pattern(&PassPattern::Tiled {
                pattern: [0x55, 0x55, 0x55, 0, 0, 0, 0, 0],
                len: 3,
                verify: false,
            }),
            "tiled-555555"
        );
    }

    #[test]
    fn fill_pattern_fixed_matches() {
        let mut buf = [0u8; 16];
        fill_pattern(
            &mut buf,
            &PassPattern::Fixed {
                byte: 0x5A,
                verify: false,
            },
        );
        assert!(buf.iter().all(|b| *b == 0x5A));
    }

    #[test]
    fn fill_pattern_tiled_wraps() {
        let mut buf = [0u8; 10];
        fill_pattern(
            &mut buf,
            &PassPattern::Tiled {
                pattern: [0xDE, 0xAD, 0xBE, 0, 0, 0, 0, 0],
                len: 3,
                verify: false,
            },
        );
        assert_eq!(
            &buf[..],
            &[0xDE, 0xAD, 0xBE, 0xDE, 0xAD, 0xBE, 0xDE, 0xAD, 0xBE, 0xDE]
        );
    }

    #[test]
    fn rate_bps_sane() {
        assert_eq!(rate_bps(0, Duration::from_secs(1)), 0);
        assert_eq!(rate_bps(100, Duration::ZERO), 0);
        assert_eq!(rate_bps(1000, Duration::from_secs(2)), 500);
    }
}
