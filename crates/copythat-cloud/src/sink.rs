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

use std::sync::Arc;

use bytes::Bytes;
use copythat_core::CloudSink;
use tokio::runtime::{Builder, Runtime};

use crate::target::CopyTarget;

/// `CopyOptions::cloud_sink`-compatible adapter backed by a
/// [`crate::OperatorTarget`] (or any [`CopyTarget`] impl).
pub struct CopyThatCloudSink {
    target: Arc<dyn CopyTarget>,
    runtime: Runtime,
}

impl std::fmt::Debug for CopyThatCloudSink {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CopyThatCloudSink")
            .field("backend_name", &self.target.name())
            .finish()
    }
}

impl CopyThatCloudSink {
    /// Wrap `target` in a sink that drives its async IO through a
    /// dedicated single-thread tokio runtime. Fails only if the
    /// tokio runtime itself can't start (virtually never in practice).
    pub fn new(target: Arc<dyn CopyTarget>) -> std::io::Result<Self> {
        let runtime = Builder::new_current_thread()
            .enable_all()
            .thread_name(format!("copythat-cloud-sink-{}", target.name()))
            .build()?;
        Ok(Self { target, runtime })
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
        self.runtime
            .block_on(async move { target.put(&path, owned).await })
            .map_err(|e| e.to_string())?;
        Ok(len)
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
