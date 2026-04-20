//! Per-copy configuration.

use std::future::Future;
use std::path::PathBuf;
use std::pin::Pin;
use std::sync::Arc;

use tokio::sync::mpsc;

use crate::control::CopyControl;
use crate::error::CopyError;
use crate::event::{CopyEvent, CopyReport};
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
    /// Which copy strategy the engine should attempt. See [`CopyStrategy`].
    /// Default is [`CopyStrategy::Auto`]. The strategy is only consulted
    /// when [`fast_copy_hook`](Self::fast_copy_hook) is also set;
    /// otherwise the engine always runs the async loop regardless of
    /// strategy.
    pub strategy: CopyStrategy,
    /// Optional bridge to the OS-native fast paths.
    ///
    /// When `Some`, `copy_file` consults the hook before opening files
    /// for the standard read/write loop. The hook is responsible for
    /// reflink, `CopyFileExW`, `copyfile(3)`, `copy_file_range(2)`, and
    /// any other syscall-level acceleration. Returning
    /// [`FastCopyHookOutcome::NotSupported`] tells the engine to fall
    /// through to its async loop. The bridge implementation lives in
    /// `copythat-platform` to keep this crate `#![forbid(unsafe_code)]`-clean.
    ///
    /// The hook is bypassed entirely when [`verify`](Self::verify) is
    /// `Some`, because the verify pipeline relies on hashing the source
    /// bytes during the write loop — fast paths don't expose the
    /// bytes, so verifying through them would require a third-pass
    /// re-read of both files and lose the integration's perf win.
    pub fast_copy_hook: Option<Arc<dyn FastCopyHook>>,
}

/// User-selectable copy strategy.
///
/// Controls which acceleration paths `copy_file` attempts when a
/// [`FastCopyHook`] is installed. With no hook installed, the engine
/// always uses the async byte-by-byte loop regardless of strategy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CopyStrategy {
    /// Try reflink → OS-native → async fallback. The default.
    #[default]
    Auto,
    /// Skip every fast path; always use the async byte-by-byte engine.
    /// Useful for benchmarks and for filesystems where reflink / native
    /// shortcuts have known correctness issues.
    AlwaysAsync,
    /// Try reflink → OS-native; if neither is available, surface an
    /// `IoOther` error rather than silently falling through to the
    /// async engine. Useful for tests that need to assert a specific
    /// fast path actually fired.
    AlwaysFast,
    /// Skip the reflink attempt; OS-native and async fallback still apply.
    /// Useful when the user has observed reflink overhead on a particular
    /// filesystem (rare, but documented for parity with TeraCopy).
    NoReflink,
}

/// Outcome a [`FastCopyHook`] reports back to the engine.
#[derive(Debug)]
pub enum FastCopyHookOutcome {
    /// Hook handled the copy. The included [`CopyReport`] is the truth
    /// the engine returns to its caller.
    Done(CopyReport),
    /// Hook tried every applicable strategy and none was supported on
    /// this src / dst pair. The engine should fall through to its async
    /// loop (unless [`CopyStrategy::AlwaysFast`] was requested, in
    /// which case the engine surfaces an error instead).
    NotSupported,
}

/// Bridge contract for the OS-native fast paths.
///
/// Implemented by `copythat-platform::PlatformFastCopyHook`. Kept in
/// this crate so [`CopyOptions`] can hold a trait object without a
/// dependency cycle.
///
/// The hook receives a *clone* of the active [`CopyOptions`] including
/// itself; implementations must not recursively call back into
/// [`crate::copy_file`] with the same options or they will infinite-loop.
/// Real implementations dispatch to the relevant syscall directly.
pub trait FastCopyHook: Send + Sync + std::fmt::Debug {
    /// Try to copy `src` to `dst` using a fast path. Emits Started /
    /// Progress / Completed events on `events` exactly like the async
    /// engine would. Honours `ctrl` for pause / cancel where the
    /// underlying syscall supports it (most do).
    fn try_copy<'a>(
        &'a self,
        src: PathBuf,
        dst: PathBuf,
        opts: CopyOptions,
        ctrl: CopyControl,
        events: mpsc::Sender<CopyEvent>,
    ) -> Pin<Box<dyn Future<Output = Result<FastCopyHookOutcome, CopyError>> + Send + 'a>>;
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
            strategy: CopyStrategy::Auto,
            fast_copy_hook: None,
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

/// What the tree engine should do when a per-file copy fails.
///
/// Separate from `CollisionPolicy` — that resolves "destination already
/// exists" *before* the copy starts; this governs "the copy started
/// and the filesystem said no" (permission denied, disk full,
/// interrupted, …).
///
/// Added in Phase 8 so the UI can surface a retry / skip dialog. The
/// default is `Abort` to preserve the pre-Phase-8 tree semantics
/// (one failure aborts the whole tree); the Tauri runner opts into
/// `Ask` when the user has not overridden the Settings policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ErrorPolicy {
    /// Emit `CopyEvent::ErrorPrompt` and block the failing file's
    /// task until the consumer replies via a one-shot. The consumer
    /// picks `ErrorAction::Retry` / `Skip` / `Abort`.
    Ask,
    /// Record the error as a `CopyEvent::FileError` event + `errored`
    /// counter increment, then continue the tree.
    Skip,
    /// Retry the failing copy up to `max_attempts` times with a
    /// fixed backoff, then fall through to `Skip` on exhaustion.
    RetryN {
        /// Maximum re-tries (does NOT count the initial attempt).
        /// Clamped to `[0, 10]` by the engine.
        max_attempts: u8,
        /// Sleep between retries, in milliseconds.
        /// Clamped to `[0, 10_000]` by the engine.
        backoff_ms: u64,
    },
    /// Cancel the whole tree on first per-file failure. Pre-Phase-8
    /// behaviour; kept as the default so existing callers see no
    /// behaviour change until they opt in.
    #[default]
    Abort,
}

/// Consumer response to a `CopyEvent::ErrorPrompt`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorAction {
    /// Re-run `copy_file` for the failing entry. If the retry also
    /// fails, the engine emits another `ErrorPrompt` (so a loop of
    /// "keep retrying" is up to the consumer).
    Retry,
    /// Record the error as a `FileError` event + `errored` counter;
    /// continue the rest of the tree.
    Skip,
    /// Cancel the tree. Same outcome as `ErrorPolicy::Abort`.
    Abort,
}

/// Behaviour knobs for `copy_tree` / `move_tree`.
#[derive(Debug, Clone)]
pub struct TreeOptions {
    /// Per-file copy behaviour. Applied uniformly to every file in the
    /// tree.
    pub file: CopyOptions,
    /// How to resolve an existing destination. Default: `Skip`.
    pub collision: crate::collision::CollisionPolicy,
    /// What to do when a per-file copy *fails* (as opposed to "dst
    /// already exists"). Default: `Abort` — pre-Phase-8 behaviour.
    pub on_error: ErrorPolicy,
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
            on_error: ErrorPolicy::default(),
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

    /// Clamp the RetryN knobs so a pathological config (e.g.
    /// `max_attempts: 255, backoff_ms: u64::MAX`) can't freeze the
    /// tree. Non-`RetryN` policies pass through unchanged.
    pub(crate) fn clamped_on_error(&self) -> ErrorPolicy {
        match self.on_error {
            ErrorPolicy::RetryN {
                max_attempts,
                backoff_ms,
            } => ErrorPolicy::RetryN {
                max_attempts: max_attempts.min(10),
                backoff_ms: backoff_ms.min(10_000),
            },
            other => other,
        }
    }
}
