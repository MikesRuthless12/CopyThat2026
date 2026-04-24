//! Phase 32c — local-source → [`CopyTarget`] transfer helper.
//!
//! The Phase 32 engine integration will eventually route cloud
//! destinations through `CopyOptions::cloud_sink` at
//! `copy_file`-level. Until that rewrite lands, this helper gives
//! the Tauri layer a direct path from "user has a local file" to
//! "that file exists at `<backend>/<key>`" without going through the
//! async-agnostic [`crate::CopyThatCloudSink`] adapter — which is
//! useful because the adapter's blocking surface spins up its own
//! tokio runtime, whereas this helper stays fully async so it
//! composes with a tokio runtime the caller already owns.
//!
//! ```no_run
//! # async fn demo() -> Result<(), copythat_cloud::BackendError> {
//! # use std::sync::Arc;
//! # use copythat_cloud::{Backend, BackendConfig, BackendKind, LocalFsConfig, OperatorTarget, make_operator, copy_to_target};
//! let backend = Backend {
//!     name: "local-mirror".into(),
//!     kind: BackendKind::LocalFs,
//!     config: BackendConfig::LocalFs(LocalFsConfig { root: "/srv/mirror".into() }),
//! };
//! let op = make_operator(&backend, None)?;
//! let target: Arc<dyn copythat_cloud::CopyTarget> =
//!     Arc::new(OperatorTarget::new("local-mirror", op));
//! copy_to_target("/data/report.csv".as_ref(), &target, "reports/report.csv").await?;
//! # Ok(()) }
//! ```

use std::path::Path;
use std::sync::Arc;

use bytes::Bytes;

use crate::error::BackendError;
use crate::target::CopyTarget;

/// Transfer a single local file to a configured [`CopyTarget`] at
/// `dst_key`. Returns the number of bytes written on success.
///
/// Reads the source into memory in one shot. Phase 32d swaps this
/// for a streaming implementation that uses `opendal::Operator::
/// writer()` so >100 MiB transfers don't allocate a full-file
/// buffer; the trait surface stays the same.
pub async fn copy_to_target(
    src: &Path,
    target: &Arc<dyn CopyTarget>,
    dst_key: &str,
) -> Result<u64, BackendError> {
    if dst_key.is_empty() {
        return Err(BackendError::InvalidConfig(
            "destination key is required".into(),
        ));
    }
    let bytes = tokio::fs::read(src)
        .await
        .map_err(|e| BackendError::InvalidConfig(format!("read {src:?} failed: {e}")))?;
    let len = bytes.len() as u64;
    target.put(dst_key, Bytes::from(bytes)).await?;
    Ok(len)
}

/// Reverse direction — pull `src_key` from the target into the
/// local path `dst`. Creates parent directories on demand. Phase
/// 32d streams with `opendal::Operator::reader()`.
pub async fn copy_from_target(
    target: &Arc<dyn CopyTarget>,
    src_key: &str,
    dst: &Path,
) -> Result<u64, BackendError> {
    if src_key.is_empty() {
        return Err(BackendError::InvalidConfig("source key is required".into()));
    }
    let bytes = target.get(src_key).await?;
    let len = bytes.len() as u64;
    if let Some(parent) = dst.parent()
        && !parent.as_os_str().is_empty()
    {
        tokio::fs::create_dir_all(parent)
            .await
            .map_err(|e| BackendError::InvalidConfig(format!("mkdir {parent:?} failed: {e}")))?;
    }
    tokio::fs::write(dst, &bytes)
        .await
        .map_err(|e| BackendError::InvalidConfig(format!("write {dst:?} failed: {e}")))?;
    Ok(len)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::backend::{Backend, BackendConfig, BackendKind, LocalFsConfig, make_operator};
    use crate::target::OperatorTarget;

    fn local_target(root: &Path) -> Arc<dyn CopyTarget> {
        let backend = Backend {
            name: "local".into(),
            kind: BackendKind::LocalFs,
            config: BackendConfig::LocalFs(LocalFsConfig {
                root: root.to_string_lossy().into_owned(),
            }),
        };
        let op = make_operator(&backend, None).expect("operator");
        Arc::new(OperatorTarget::new("local", op))
    }

    #[tokio::test]
    async fn copy_to_target_round_trips_bytes() {
        let src_dir = tempfile::tempdir().expect("src dir");
        let remote_root = tempfile::tempdir().expect("remote root");

        let src = src_dir.path().join("payload.bin");
        let payload: Vec<u8> = (0..4096).map(|i| (i % 251) as u8).collect();
        tokio::fs::write(&src, &payload).await.expect("seed");

        let target = local_target(remote_root.path());
        let written = copy_to_target(&src, &target, "sub/payload.bin")
            .await
            .expect("copy");
        assert_eq!(written as usize, payload.len());

        let landed = tokio::fs::read(remote_root.path().join("sub/payload.bin"))
            .await
            .expect("landed");
        assert_eq!(landed, payload);
    }

    #[tokio::test]
    async fn copy_to_target_rejects_empty_key() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let target = local_target(tmp.path());
        let src = tmp.path().join("irrelevant.bin");
        tokio::fs::write(&src, b"x").await.expect("seed");
        let err = copy_to_target(&src, &target, "")
            .await
            .expect_err("must reject");
        assert!(matches!(err, BackendError::InvalidConfig(_)));
    }

    #[tokio::test]
    async fn copy_from_target_creates_parent_dirs() {
        let remote_root = tempfile::tempdir().expect("remote root");
        let dst_dir = tempfile::tempdir().expect("dst dir");

        let target = local_target(remote_root.path());
        target
            .put("nested/file.bin", Bytes::from_static(b"hello-target"))
            .await
            .expect("seed remote");

        let dst = dst_dir.path().join("a/b/c/file.bin");
        let pulled = copy_from_target(&target, "nested/file.bin", &dst)
            .await
            .expect("pull");
        assert_eq!(pulled, 12);
        assert_eq!(
            tokio::fs::read(&dst).await.expect("read dst"),
            b"hello-target"
        );
    }

    #[tokio::test]
    async fn copy_from_target_reports_not_found_via_opendal() {
        let remote_root = tempfile::tempdir().expect("remote root");
        let dst_dir = tempfile::tempdir().expect("dst dir");
        let target = local_target(remote_root.path());
        let err = copy_from_target(&target, "ghost.bin", &dst_dir.path().join("ghost.bin"))
            .await
            .expect_err("missing src must fail");
        // OpenDAL maps missing to its NotFound ErrorKind; our
        // `BackendError::OpenDal` passthrough's fluent_key lands on
        // `cloud-error-not-found`.
        assert_eq!(err.fluent_key(), "cloud-error-not-found");
    }
}
