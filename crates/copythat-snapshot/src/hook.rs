//! `copythat-core::SnapshotHook` implementation.
//!
//! This is the glue between the engine's `CopyOptions::snapshot_hook`
//! seat and the per-OS backends in `backends/`. The Tauri runner (and
//! any library-mode user) attaches an instance of
//! [`CopyThatSnapshotHook`] to every `CopyOptions` where
//! `on_locked == Snapshot`.

use std::future::Future;
use std::path::PathBuf;
use std::pin::Pin;

use copythat_core::{CopyError, CopyErrorKind, SnapshotGuard, SnapshotHook, SnapshotLease};

use crate::error::SnapshotError;
use crate::{SnapshotHandle, create_snapshot};

/// Engine-ready implementation of [`SnapshotHook`].
///
/// Stateless — a single instance can be shared across many jobs via
/// `Arc::new(CopyThatSnapshotHook::default())`.
#[derive(Debug, Default)]
pub struct CopyThatSnapshotHook;

impl CopyThatSnapshotHook {
    pub fn new() -> Self {
        Self
    }
}

impl SnapshotHook for CopyThatSnapshotHook {
    fn open_for_read<'a>(
        &'a self,
        src: PathBuf,
    ) -> Pin<Box<dyn Future<Output = Result<SnapshotLease, CopyError>> + Send + 'a>> {
        Box::pin(async move {
            let handle = create_snapshot(&src)
                .await
                .map_err(|e| snapshot_to_copy(&src, e))?;
            let translated = crate::translate_path(&handle, &src).ok_or_else(|| CopyError {
                kind: CopyErrorKind::IoOther,
                src: src.clone(),
                dst: src.clone(),
                raw_os_error: None,
                message: format!(
                    "snapshot did not cover original path (kind={:?}, root={})",
                    handle.kind,
                    handle.original_root.display()
                ),
            })?;
            let kind_wire = handle.kind.as_str();
            let original_root = handle.original_root.clone();
            let mount_root = handle.mount_path.clone();
            Ok(SnapshotLease {
                translated,
                kind_wire,
                original_root,
                mount_root,
                guard: Box::new(HandleGuard {
                    inner: Some(handle),
                }),
            })
        })
    }
}

#[derive(Debug)]
struct HandleGuard {
    inner: Option<SnapshotHandle>,
}

impl SnapshotGuard for HandleGuard {}

impl Drop for HandleGuard {
    fn drop(&mut self) {
        // SnapshotHandle's own Drop does the blocking release — we
        // just let it run. Keeping the Option-wrap matches the
        // "moved out on release()" pattern and stays forward-
        // compatible with an async-release path.
        drop(self.inner.take());
    }
}

fn snapshot_to_copy(src: &std::path::Path, err: SnapshotError) -> CopyError {
    let kind = match &err {
        SnapshotError::Unsupported { .. } => CopyErrorKind::IoOther,
        SnapshotError::NeedsElevation | SnapshotError::UacDenied => CopyErrorKind::PermissionDenied,
        SnapshotError::BackendMissing { .. } => CopyErrorKind::IoOther,
        SnapshotError::BackendFailure { .. } => CopyErrorKind::IoOther,
        SnapshotError::Io(e) => match e.kind() {
            std::io::ErrorKind::PermissionDenied => CopyErrorKind::PermissionDenied,
            std::io::ErrorKind::NotFound => CopyErrorKind::NotFound,
            _ => CopyErrorKind::IoOther,
        },
        SnapshotError::Protocol(_) => CopyErrorKind::IoOther,
    };
    CopyError {
        kind,
        src: src.to_path_buf(),
        dst: src.to_path_buf(),
        raw_os_error: None,
        message: format!("snapshot failed: {err}"),
    }
}
