//! Per-copy configuration.

pub const DEFAULT_BUFFER_SIZE: usize = 1024 * 1024; // 1 MiB
pub const MIN_BUFFER_SIZE: usize = 64 * 1024; // 64 KiB
pub const MAX_BUFFER_SIZE: usize = 16 * 1024 * 1024; // 16 MiB

/// Behaviour knobs for a single `copy_file` invocation.
#[derive(Debug, Clone)]
pub struct CopyOptions {
    /// Requested buffer size in bytes. Clamped to `[MIN_BUFFER_SIZE,
    /// MAX_BUFFER_SIZE]` by the engine; callers don't need to round.
    pub buffer_size: usize,
    /// If true, call `sync_all` on the destination before returning.
    /// Noticeably slower on spinning media; off by default.
    pub fsync_on_close: bool,
    /// If true, follow a symlinked source and copy the *target*. If
    /// false, clone the symlink itself at `dst`.
    pub follow_symlinks: bool,
    /// If true, copy mtime / atime from source to destination.
    pub preserve_times: bool,
    /// If true, copy the permission bits (mode on Unix, readonly bit on
    /// Windows) from source to destination.
    pub preserve_permissions: bool,
    /// If true, do NOT delete a partially-written destination when the
    /// copy fails or is cancelled. Leave it for the caller to inspect.
    pub keep_partial: bool,
    /// If true, refuse to overwrite an existing destination file and
    /// return `PermissionDenied`/`AlreadyExists`-flavoured error. The
    /// default (false) truncates and rewrites.
    pub fail_if_exists: bool,
}

impl Default for CopyOptions {
    fn default() -> Self {
        Self {
            buffer_size: DEFAULT_BUFFER_SIZE,
            fsync_on_close: false,
            follow_symlinks: true,
            preserve_times: true,
            preserve_permissions: true,
            keep_partial: false,
            fail_if_exists: false,
        }
    }
}

impl CopyOptions {
    pub fn clamped_buffer_size(&self) -> usize {
        self.buffer_size.clamp(MIN_BUFFER_SIZE, MAX_BUFFER_SIZE)
    }
}
