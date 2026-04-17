//! Per-copy configuration.

use crate::verify::Verifier;

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
    /// Optional post-copy verification.
    ///
    /// When `Some(verifier)`, the engine hashes the source stream
    /// during the normal read pass (no re-read) and hashes the
    /// destination via a dedicated post-pass. On mismatch it emits
    /// `CopyEvent::VerifyFailed` and fails the copy with
    /// `CopyErrorKind::VerifyFailed`. `copythat-hash` provides the
    /// standard set of algorithms via
    /// `HashAlgorithm::verifier()`.
    pub verify: Option<Verifier>,
    /// Automatically enable `fsync_on_close` when `verify` is `Some`.
    /// The destination post-pass reads the file immediately after the
    /// write loop, and on some filesystems (notably NFS and several
    /// network-backed shares) the post-pass can race page-cache state.
    /// Defaults to `true` — callers who know their filesystem can set
    /// it off.
    pub fsync_before_verify: bool,
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
            verify: None,
            fsync_before_verify: true,
        }
    }
}

impl CopyOptions {
    pub fn clamped_buffer_size(&self) -> usize {
        self.buffer_size.clamp(MIN_BUFFER_SIZE, MAX_BUFFER_SIZE)
    }
}

/// Behaviour knobs for `move_file` / `move_tree`.
///
/// Move is modelled as "rename if possible, otherwise copy-then-delete".
/// The copy phase reuses [`CopyOptions`]; these extra knobs govern the
/// *move* layer.
#[derive(Debug, Clone)]
pub struct MoveOptions {
    /// Settings passed through to the internal `copy_file` / `copy_tree`
    /// call on the cross-device fallback path.
    pub copy: CopyOptions,
    /// If true, when the same-volume `rename` fails with anything other
    /// than `CrossesDevices`, surface the error instead of falling back
    /// to copy-then-delete. Defaults to false.
    pub strict_rename: bool,
}

impl Default for MoveOptions {
    fn default() -> Self {
        Self {
            copy: CopyOptions {
                // fsync the destination on the move fallback — we
                // unlink the source afterwards, so the cost of an
                // extra sync is justified by not losing data on a
                // crash between flush and unlink.
                fsync_on_close: true,
                ..CopyOptions::default()
            },
            strict_rename: false,
        }
    }
}

/// Default concurrency for `copy_tree` / `move_tree`. Deliberately
/// conservative — Phase 6 will pick this from per-volume SSD / HDD
/// detection.
pub const DEFAULT_TREE_CONCURRENCY: usize = 4;

/// Behaviour knobs for `copy_tree` / `move_tree`.
#[derive(Debug, Clone)]
pub struct TreeOptions {
    /// Per-file copy behaviour. Applied uniformly to every file in the
    /// tree.
    pub file: CopyOptions,
    /// How to resolve an existing destination. Default: `Skip`.
    pub collision: crate::collision::CollisionPolicy,
    /// Maximum concurrent file copies. Clamped to `[1, 64]`.
    pub concurrency: usize,
    /// If true, follow symlinks found *inside* the source tree and
    /// descend into the target. If false (default), reproduce them as
    /// symlinks at the destination — matches the intuitive "copy this
    /// folder, do not chase shortcuts" behaviour and prevents cycles.
    pub follow_symlinks_in_tree: bool,
    /// If true, preserve mtime / atime on every *directory* in
    /// addition to every file. Defaults to true.
    pub preserve_directory_times: bool,
}

impl Default for TreeOptions {
    fn default() -> Self {
        Self {
            file: CopyOptions::default(),
            collision: crate::collision::CollisionPolicy::Skip,
            concurrency: DEFAULT_TREE_CONCURRENCY,
            follow_symlinks_in_tree: false,
            preserve_directory_times: true,
        }
    }
}

impl TreeOptions {
    pub(crate) fn clamped_concurrency(&self) -> usize {
        self.concurrency.clamp(1, 64)
    }
}
