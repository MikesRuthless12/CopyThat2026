//! Linux native fast path.
//!
//! Strategy:
//!
//! 1. `copy_file_range(2)` in a loop — the kernel's internal accelerated
//!    path. On reflink-capable filesystems this collapses to a clone;
//!    elsewhere it pumps bytes through page cache without crossing into
//!    userspace.
//! 2. `sendfile(2)` fallback when `copy_file_range` returns
//!    `EXDEV` / `EINVAL` and the source is <2 GiB. `sendfile` has the
//!    same in-kernel speed advantage but predates `copy_file_range`
//!    and has a 2 GiB single-call cap on 32-bit `off_t` systems.
//! 3. Anything else falls through to the async engine via
//!    [`super::NativeOutcome::Unsupported`].
//!
//! Pause / resume / cancel: checked between syscall iterations. A
//! cancelled copy unlinks the partial destination via the dispatcher's
//! standard path.

use std::fs::OpenOptions;
use std::io;
use std::os::fd::{AsRawFd, RawFd};
use std::path::{Path, PathBuf};
use std::time::Instant;

use copythat_core::{CopyControl, CopyEvent};
use tokio::sync::mpsc;

use super::NativeOutcome;
use crate::outcome::ChosenStrategy;

const SENDFILE_MAX_FILE_SIZE: u64 = 2 * 1024 * 1024 * 1024; // 2 GiB
const COPY_CHUNK: usize = 16 * 1024 * 1024; // 16 MiB per syscall iteration
const PROGRESS_MIN_BYTES: u64 = 16 * 1024;
const PROGRESS_MIN_INTERVAL: std::time::Duration = std::time::Duration::from_millis(50);

#[allow(clippy::too_many_arguments)]
pub(crate) async fn try_native_copy(
    src: PathBuf,
    dst: PathBuf,
    total: u64,
    ctrl: CopyControl,
    events: mpsc::Sender<CopyEvent>,
) -> NativeOutcome {
    super::emit_started(&src, &dst, total, &events).await;

    // Both files opened on a blocking thread — open(2) on a network
    // filesystem can stall and we don't want to park the runtime.
    let pair = tokio::task::spawn_blocking(move || open_pair(&src, &dst)).await;
    let (src_file, dst_file) = match pair {
        Ok(Ok(pair)) => pair,
        Ok(Err(e)) => return NativeOutcome::Io(e),
        Err(join) => {
            return NativeOutcome::Io(io::Error::other(format!("open spawn panicked: {join}")));
        }
    };

    let src_fd = src_file.as_raw_fd();
    let dst_fd = dst_file.as_raw_fd();

    // Try copy_file_range first. We loop until either total bytes are
    // moved, an iteration says "0 bytes" (EOF), or an error fires.
    let started = Instant::now();
    match drive_copy_file_range(src_fd, dst_fd, total, &ctrl, &events, started).await {
        DriveOutcome::Done(bytes) => {
            // Keep the files alive until after the syscall returns.
            drop(src_file);
            drop(dst_file);
            return NativeOutcome::Done {
                strategy: ChosenStrategy::CopyFileRange,
                bytes,
            };
        }
        DriveOutcome::Cancelled => {
            drop(src_file);
            drop(dst_file);
            return NativeOutcome::Cancelled;
        }
        DriveOutcome::FallThroughToSendfile => {
            // Reset destination to zero — copy_file_range may have
            // written some bytes before EXDEV / EINVAL surfaced.
            if let Err(e) = ftruncate_zero(dst_fd) {
                drop(src_file);
                drop(dst_file);
                return NativeOutcome::Io(e);
            }
            if let Err(e) = lseek_start(src_fd) {
                drop(src_file);
                drop(dst_file);
                return NativeOutcome::Io(e);
            }
        }
        DriveOutcome::Unsupported => {
            drop(src_file);
            drop(dst_file);
            return NativeOutcome::Unsupported;
        }
        DriveOutcome::Io(e) => {
            drop(src_file);
            drop(dst_file);
            return NativeOutcome::Io(e);
        }
    }

    // sendfile(2) is the second-tier in-kernel option. Cap to 2 GiB
    // because sendfile's offset argument is `off_t` and some libc
    // configurations still wrap to 32 bits.
    if total > SENDFILE_MAX_FILE_SIZE {
        drop(src_file);
        drop(dst_file);
        return NativeOutcome::Unsupported;
    }
    let started_sendfile = Instant::now();
    let outcome = drive_sendfile(src_fd, dst_fd, total, &ctrl, &events, started_sendfile).await;
    drop(src_file);
    drop(dst_file);
    outcome
}

enum DriveOutcome {
    Done(u64),
    Cancelled,
    /// copy_file_range failed with EXDEV / EINVAL on the very first
    /// iteration; try sendfile.
    FallThroughToSendfile,
    /// copy_file_range is genuinely not supported by this kernel
    /// (ENOSYS) — let the dispatcher fall through to the async loop.
    Unsupported,
    Io(io::Error),
}

async fn drive_copy_file_range(
    src_fd: RawFd,
    dst_fd: RawFd,
    total: u64,
    ctrl: &CopyControl,
    events: &mpsc::Sender<CopyEvent>,
    started: Instant,
) -> DriveOutcome {
    let mut moved: u64 = 0;
    let mut last_emit_at = started;
    let mut last_emit_bytes: u64 = 0;
    let mut first_call = true;

    loop {
        if ctrl.is_cancelled() {
            return DriveOutcome::Cancelled;
        }
        if ctrl.is_paused() {
            ctrl.wait_while_paused().await;
            if ctrl.is_cancelled() {
                return DriveOutcome::Cancelled;
            }
        }

        let remaining = total.saturating_sub(moved) as usize;
        if remaining == 0 {
            // Source had `total` bytes; everything moved. (For sparse
            // files copy_file_range still reports the logical length.)
            return DriveOutcome::Done(moved);
        }
        let chunk = remaining.min(COPY_CHUNK);

        // SAFETY: passing valid raw fds (kept alive by the caller) and
        // null offset pointers means the kernel uses the file's
        // current position for both. `len` is bounded above by 16 MiB.
        let ret = unsafe {
            libc::copy_file_range(
                src_fd,
                std::ptr::null_mut(),
                dst_fd,
                std::ptr::null_mut(),
                chunk,
                0,
            )
        };

        if ret == -1 {
            let err = io::Error::last_os_error();
            let raw = err.raw_os_error().unwrap_or(0);
            if first_call && (raw == libc::EXDEV || raw == libc::EINVAL) {
                return DriveOutcome::FallThroughToSendfile;
            }
            if raw == libc::ENOSYS {
                return DriveOutcome::Unsupported;
            }
            return DriveOutcome::Io(err);
        }
        first_call = false;
        if ret == 0 {
            // EOF before total — happens when the source was truncated
            // mid-copy, or when the file system reports a different
            // logical length than the metadata. Treat as success but
            // report what we actually moved.
            return DriveOutcome::Done(moved);
        }

        moved += ret as u64;

        let now = Instant::now();
        if moved.saturating_sub(last_emit_bytes) >= PROGRESS_MIN_BYTES
            && now.duration_since(last_emit_at) >= PROGRESS_MIN_INTERVAL
        {
            let elapsed = now.duration_since(started);
            let rate = super::fast_rate_bps(moved, elapsed);
            let _ = events
                .send(CopyEvent::Progress {
                    bytes: moved,
                    total,
                    rate_bps: rate,
                })
                .await;
            last_emit_at = now;
            last_emit_bytes = moved;
        }
    }
}

async fn drive_sendfile(
    src_fd: RawFd,
    dst_fd: RawFd,
    total: u64,
    ctrl: &CopyControl,
    events: &mpsc::Sender<CopyEvent>,
    started: Instant,
) -> NativeOutcome {
    let mut moved: u64 = 0;
    let mut last_emit_at = started;
    let mut last_emit_bytes: u64 = 0;

    loop {
        if ctrl.is_cancelled() {
            return NativeOutcome::Cancelled;
        }
        if ctrl.is_paused() {
            ctrl.wait_while_paused().await;
            if ctrl.is_cancelled() {
                return NativeOutcome::Cancelled;
            }
        }

        let remaining = total.saturating_sub(moved) as usize;
        if remaining == 0 {
            return NativeOutcome::Done {
                strategy: ChosenStrategy::Sendfile,
                bytes: moved,
            };
        }
        let chunk = remaining.min(COPY_CHUNK);

        // SAFETY: src_fd/dst_fd are valid (held alive by caller). A
        // null offset pointer makes sendfile read from the current src
        // file position, matching the copy_file_range semantics.
        let ret = unsafe { libc::sendfile(dst_fd, src_fd, std::ptr::null_mut(), chunk) };

        if ret == -1 {
            let err = io::Error::last_os_error();
            let raw = err.raw_os_error().unwrap_or(0);
            if raw == libc::ENOSYS || raw == libc::EINVAL {
                return NativeOutcome::Unsupported;
            }
            return NativeOutcome::Io(err);
        }
        if ret == 0 {
            return NativeOutcome::Done {
                strategy: ChosenStrategy::Sendfile,
                bytes: moved,
            };
        }
        moved += ret as u64;

        let now = Instant::now();
        if moved.saturating_sub(last_emit_bytes) >= PROGRESS_MIN_BYTES
            && now.duration_since(last_emit_at) >= PROGRESS_MIN_INTERVAL
        {
            let elapsed = now.duration_since(started);
            let rate = super::fast_rate_bps(moved, elapsed);
            let _ = events
                .send(CopyEvent::Progress {
                    bytes: moved,
                    total,
                    rate_bps: rate,
                })
                .await;
            last_emit_at = now;
            last_emit_bytes = moved;
        }
    }
}

fn open_pair(src: &Path, dst: &Path) -> io::Result<(std::fs::File, std::fs::File)> {
    let src_file = std::fs::File::open(src)?;
    let dst_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(dst)?;
    Ok((src_file, dst_file))
}

fn ftruncate_zero(fd: RawFd) -> io::Result<()> {
    // SAFETY: `fd` is the destination file owned by the caller.
    let ret = unsafe { libc::ftruncate(fd, 0) };
    if ret == -1 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}

fn lseek_start(fd: RawFd) -> io::Result<()> {
    // SAFETY: `fd` is the source file owned by the caller. Offset 0 is
    // unconditionally valid.
    let ret = unsafe { libc::lseek(fd, 0, libc::SEEK_SET) };
    if ret == -1 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}

// ---------------------------------------------------------------------
// Helpers: SSD probe + filesystem name
// ---------------------------------------------------------------------

pub(crate) fn is_ssd(path: &Path) -> Option<bool> {
    let probe_target = if path.exists() {
        path.to_path_buf()
    } else {
        path.parent()?.to_path_buf()
    };
    let out = std::process::Command::new("findmnt")
        .arg("-nro")
        .arg("SOURCE")
        .arg("-T")
        .arg(&probe_target)
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    let device = String::from_utf8(out.stdout).ok()?.trim().to_string();
    if device.is_empty() {
        return None;
    }
    let dev_name = Path::new(&device)
        .file_name()?
        .to_string_lossy()
        .into_owned();
    let parent_dev = strip_partition_suffix(&dev_name);
    let sys_path = Path::new("/sys/block")
        .join(&parent_dev)
        .join("queue/rotational");
    let flag = std::fs::read_to_string(&sys_path).ok()?;
    match flag.trim() {
        "0" => Some(true),
        "1" => Some(false),
        _ => None,
    }
}

fn strip_partition_suffix(name: &str) -> String {
    if name.starts_with("nvme") || name.starts_with("mmcblk") || name.starts_with("loop") {
        if let Some(pos) = name.rfind('p') {
            let (head, tail) = name.split_at(pos);
            // Real partition suffixes look like `nvme0n1p3` or `mmcblk0p2`:
            // the char immediately before `p` is always a digit (the device
            // index). Plain `loop0` has its `p` inside the device name, so
            // the char before is `o` and we must not strip.
            let head_ends_in_digit = head.chars().next_back().is_some_and(|c| c.is_ascii_digit());
            if head_ends_in_digit
                && tail.len() > 1
                && tail[1..].chars().all(|c| c.is_ascii_digit())
            {
                return head.to_string();
            }
        }
        return name.to_string();
    }
    let trimmed: String = name
        .chars()
        .rev()
        .skip_while(|c| c.is_ascii_digit())
        .collect::<String>()
        .chars()
        .rev()
        .collect();
    if trimmed.is_empty() {
        name.to_string()
    } else {
        trimmed
    }
}

pub(crate) fn filesystem_name(path: &Path) -> Option<String> {
    let probe_target = if path.exists() {
        path.to_path_buf()
    } else {
        path.parent()?.to_path_buf()
    };
    let out = std::process::Command::new("findmnt")
        .arg("-nro")
        .arg("FSTYPE")
        .arg("-T")
        .arg(&probe_target)
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    let name = String::from_utf8(out.stdout)
        .ok()?
        .trim()
        .to_ascii_lowercase();
    if name.is_empty() { None } else { Some(name) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partition_suffix_stripping() {
        assert_eq!(strip_partition_suffix("sda1"), "sda");
        assert_eq!(strip_partition_suffix("nvme0n1p3"), "nvme0n1");
        assert_eq!(strip_partition_suffix("mmcblk0p2"), "mmcblk0");
        assert_eq!(strip_partition_suffix("loop0"), "loop0");
    }
}
