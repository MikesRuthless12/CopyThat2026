//! Phase 32b — `CopyThatCloudSink` adapter.
//!
//! Implements [`copythat_core::CloudSink`] on top of an
//! [`crate::OperatorTarget`] so `CopyOptions::cloud_sink` can be
//! populated without `copythat-core` pulling in OpenDAL.
//!
//! The contract is synchronous / blocking ([`copythat_core::CloudSink::put_blocking`]).
//! The adapter owns a dedicated single-thread tokio runtime so the
//! engine's copy task (itself running on a multi-thread tokio
//! runtime) can call the sink without re-entering its own executor.

use std::path::Path;
use std::sync::Arc;

use bytes::Bytes;
use copythat_core::CloudSink;
use tokio::io::AsyncReadExt;
use tokio::runtime::Builder;

use crate::target::CopyTarget;

/// `CopyOptions::cloud_sink`-compatible adapter backed by a
/// [`crate::OperatorTarget`] (or any [`CopyTarget`] impl).
///
/// **Runtime model (Phase 32f rewrite):** the blocking methods run
/// each call on a fresh OS thread that owns a dedicated
/// `current_thread` tokio runtime. This sidesteps the tokio
/// "cannot block the current thread from within a runtime" error
/// that surfaces when the engine's `spawn_blocking` worker tries to
/// re-enter an outer runtime via a shared inner `block_on`. Each
/// transfer is a whole-file upload, so per-call thread spawn is
/// negligible overhead.
pub struct CopyThatCloudSink {
    target: Arc<dyn CopyTarget>,
}

impl std::fmt::Debug for CopyThatCloudSink {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CopyThatCloudSink")
            .field("backend_name", &self.target.name())
            .finish()
    }
}

impl CopyThatCloudSink {
    /// Wrap `target` in a sink. Always `Ok` — the per-call thread
    /// model needs no setup. Returns `std::io::Result` so callers
    /// keep the same construction shape across Phase 32b/f
    /// revisions.
    pub fn new(target: Arc<dyn CopyTarget>) -> std::io::Result<Self> {
        Ok(Self { target })
    }

    /// Run `work` on a fresh OS thread with its own
    /// `current_thread` runtime. Work takes the live `Runtime` by
    /// reference so it can `block_on` its own async body. Keeps the
    /// caller's tokio context out of the way so a `spawn_blocking`-
    /// hosted call doesn't trip the "nested runtime" guard.
    fn run_on_dedicated_thread<F, T>(thread_name: String, work: F) -> Result<T, String>
    where
        F: FnOnce(&tokio::runtime::Runtime) -> Result<T, String> + Send + 'static,
        T: Send + 'static,
    {
        let builder = std::thread::Builder::new().name(thread_name);
        let handle = builder
            .spawn(move || -> Result<T, String> {
                let rt = Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .map_err(|e| format!("runtime build: {e}"))?;
                work(&rt)
            })
            .map_err(|e| format!("spawn runtime thread: {e}"))?;
        handle
            .join()
            .map_err(|_| "runtime thread panicked".to_owned())?
    }
}

impl CloudSink for CopyThatCloudSink {
    fn backend_name(&self) -> &str {
        self.target.name()
    }

    fn put_blocking(&self, path: &str, bytes: &[u8]) -> Result<u64, String> {
        let owned = Bytes::copy_from_slice(bytes);
        let target = self.target.clone();
        let path = path.to_owned();
        let len = bytes.len() as u64;
        let thread_name = format!("copythat-cloud-sink-{}", target.name());
        Self::run_on_dedicated_thread(thread_name, move |rt| {
            rt.block_on(async move { target.put(&path, owned).await })
                .map_err(|e| e.to_string())
        })?;
        Ok(len)
    }

    /// Phase 32f — streaming override. When the wrapped `CopyTarget`
    /// exposes an `opendal::Operator`, open a streaming `Writer`
    /// and pipe source-file chunks through it without buffering the
    /// full payload. Non-OpenDAL targets fall through to the trait
    /// default which reads-into-Vec then delegates to
    /// `put_blocking`.
    fn put_stream_blocking(
        &self,
        path: &str,
        source_path: &Path,
        buffer_size: usize,
        on_progress: &dyn Fn(u64),
    ) -> Result<u64, String> {
        let Some(operator) = self.target.as_opendal_operator().cloned() else {
            // No operator → fall back to read-into-Vec.
            let bytes = std::fs::read(source_path).map_err(|e| e.to_string())?;
            let n = bytes.len() as u64;
            on_progress(n);
            return self.put_blocking(path, &bytes);
        };

        let path_owned = path.to_owned();
        let source_owned = source_path.to_path_buf();
        let buffer_size = buffer_size.clamp(64 * 1024, 16 * 1024 * 1024);
        let thread_name = format!("copythat-cloud-stream-{}", self.target.name());

        // Cross-thread progress via a channel; the async worker
        // sends byte totals back to the caller's thread, which
        // invokes the callback. Avoids Send-bounding the closure.
        let (progress_tx, progress_rx) = std::sync::mpsc::channel::<u64>();

        // Spawn the worker in a thread, then drain progress on this
        // thread until the worker joins.
        let worker_handle = std::thread::Builder::new()
            .name(thread_name)
            .spawn(move || -> Result<u64, String> {
                let rt = Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .map_err(|e| format!("runtime build: {e}"))?;
                rt.block_on(async move {
                    let mut file = tokio::fs::File::open(&source_owned)
                        .await
                        .map_err(|e| format!("open {}: {e}", source_owned.display()))?;
                    let mut writer = operator
                        .writer(&path_owned)
                        .await
                        .map_err(|e| format!("open writer: {e}"))?;
                    let mut buf = vec![0u8; buffer_size];
                    let mut total: u64 = 0;
                    loop {
                        let n = file
                            .read(&mut buf)
                            .await
                            .map_err(|e| format!("read chunk: {e}"))?;
                        if n == 0 {
                            break;
                        }
                        let chunk = Bytes::copy_from_slice(&buf[..n]);
                        writer
                            .write(chunk)
                            .await
                            .map_err(|e| format!("write chunk: {e}"))?;
                        total += n as u64;
                        let _ = progress_tx.send(total);
                    }
                    writer
                        .close()
                        .await
                        .map_err(|e| format!("close writer: {e}"))?;
                    Ok::<u64, String>(total)
                })
            })
            .map_err(|e| format!("spawn worker: {e}"))?;

        // Drain progress until the worker finishes.
        while let Ok(bytes_done) = progress_rx.recv() {
            on_progress(bytes_done);
        }
        worker_handle
            .join()
            .map_err(|_| "worker thread panicked".to_owned())?
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::backend::{
        Backend, BackendConfig, BackendKind, LocalFsConfig, make_operator,
    };
    use crate::target::OperatorTarget;

    #[test]
    fn sink_puts_bytes_through_local_fs_target() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let backend = Backend {
            name: "cloud-sink-test".into(),
            kind: BackendKind::LocalFs,
            config: BackendConfig::LocalFs(LocalFsConfig {
                root: tmp.path().to_string_lossy().into_owned(),
            }),
        };
        let target = Arc::new(OperatorTarget::new(
            "cloud-sink-test",
            make_operator(&backend, None).expect("operator"),
        ));

        let sink = CopyThatCloudSink::new(target).expect("sink");
        assert_eq!(sink.backend_name(), "cloud-sink-test");

        let payload = b"phase-32b sink payload";
        let written = sink.put_blocking("relpath/file.bin", payload).expect("put");
        assert_eq!(written, payload.len() as u64);

        // Readback through std::fs to confirm the local FS saw the
        // bytes the sink wrote.
        let disk = std::fs::read(tmp.path().join("relpath/file.bin")).expect("disk read");
        assert_eq!(disk, payload);
    }
}
