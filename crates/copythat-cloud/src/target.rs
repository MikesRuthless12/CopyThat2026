//! Phase 32 — `CopyTarget` async trait + the OpenDAL operator
//! adapter.
//!
//! The trait surface is intentionally small (`put` / `get` / `list` /
//! `stat` / `delete`) so the Phase 32b engine integration can drive
//! local + remote endpoints through one `dyn CopyTarget`.

use bytes::Bytes;
use serde::{Deserialize, Serialize};

use crate::error::BackendError;

/// Listing entry returned by [`CopyTarget::list`] /
/// [`CopyTarget::stat`]. The shape mirrors what
/// `copythat-history::Item` will eventually want to record for cloud
/// objects.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct EntryMeta {
    /// Path relative to the operator's root.
    pub path: String,
    /// `true` for directories / common prefixes; `false` for blobs.
    pub is_dir: bool,
    /// Object size in bytes. `None` when the backend doesn't report
    /// it (some WebDAV servers omit `getcontentlength`).
    pub size: Option<u64>,
    /// Last-modified RFC 3339 timestamp. `None` when the backend
    /// doesn't expose mtime (FTP without `MDTM`, some SFTP servers).
    pub last_modified: Option<String>,
    /// Phase 32g — server-reported ETag. Populated on S3-class +
    /// Azure Blob backends; `None` elsewhere. The `verify` fast-path
    /// uses this (when present) to skip a round-trip GET + re-hash.
    /// The value is the raw ETag string as the backend returned it —
    /// typically a hex-encoded MD5 on S3's default single-part
    /// upload, a multipart-MD5-of-MD5s on MPU, or an opaque version
    /// token for conditional requests.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    /// Phase 32g — server-reported content-MD5 header (hex
    /// encoded). Populated on Azure Blob with MD5 enabled, some S3
    /// setups. `None` elsewhere.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_md5: Option<String>,
}

/// Async target the engine writes to or reads from. Phase 32a
/// implements this for `opendal::Operator` via [`OperatorTarget`];
/// Phase 32b adds engine integration via `CopySource` / `CopySink`.
#[async_trait::async_trait]
pub trait CopyTarget: Send + Sync {
    /// Stable display name — surfaces in the runner's progress
    /// events.
    fn name(&self) -> &str;

    /// Upload `data` to `path`, overwriting any existing object.
    async fn put(&self, path: &str, data: Bytes) -> Result<(), BackendError>;

    /// Read the object at `path` into memory. Phase 32b adds a
    /// streaming variant for >100 MiB transfers.
    async fn get(&self, path: &str) -> Result<Bytes, BackendError>;

    /// List entries under `prefix`. Recursive — the caller filters.
    async fn list(&self, prefix: &str) -> Result<Vec<EntryMeta>, BackendError>;

    /// Stat a single object. `Ok(None)` when the object is absent;
    /// every other failure propagates as a typed error.
    async fn stat(&self, path: &str) -> Result<Option<EntryMeta>, BackendError>;

    /// Remove the object at `path`. No-op when the object doesn't
    /// exist (same shape as `Credentials::delete`).
    async fn delete(&self, path: &str) -> Result<(), BackendError>;

    /// Phase 32f — expose the underlying OpenDAL operator when the
    /// impl has one. Lets the `CopyThatCloudSink` adapter bypass
    /// the `put` one-shot path and drive streaming uploads through
    /// `opendal::Operator::writer()`. Default `None` — non-OpenDAL
    /// impls (test fakes, custom backends) opt in when they have
    /// an operator to share.
    fn as_opendal_operator(&self) -> Option<&opendal::Operator> {
        None
    }
}

/// `CopyTarget` impl wrapping a configured [`opendal::Operator`].
pub struct OperatorTarget {
    name: String,
    operator: opendal::Operator,
}

impl OperatorTarget {
    pub fn new(name: impl Into<String>, operator: opendal::Operator) -> Self {
        Self {
            name: name.into(),
            operator,
        }
    }

    pub fn operator(&self) -> &opendal::Operator {
        &self.operator
    }
}

#[async_trait::async_trait]
impl CopyTarget for OperatorTarget {
    fn name(&self) -> &str {
        &self.name
    }

    fn as_opendal_operator(&self) -> Option<&opendal::Operator> {
        Some(&self.operator)
    }

    async fn put(&self, path: &str, data: Bytes) -> Result<(), BackendError> {
        // opendal 0.54 takes a `Buffer`; converting Bytes is zero-copy.
        let buffer: opendal::Buffer = data.into();
        self.operator.write(path, buffer).await?;
        Ok(())
    }

    async fn get(&self, path: &str) -> Result<Bytes, BackendError> {
        let buffer = self.operator.read(path).await?;
        // `Buffer::to_bytes` is a defragmenting copy; acceptable for
        // small reads. Phase 32b's streaming path uses `reader()`
        // instead.
        Ok(buffer.to_bytes())
    }

    async fn list(&self, prefix: &str) -> Result<Vec<EntryMeta>, BackendError> {
        let entries = self.operator.list_with(prefix).recursive(true).await?;
        let mut out = Vec::with_capacity(entries.len());
        for e in entries {
            let meta = e.metadata();
            out.push(EntryMeta {
                path: e.path().to_owned(),
                is_dir: meta.mode().is_dir(),
                size: Some(meta.content_length()),
                last_modified: meta.last_modified().map(|t| t.to_rfc3339()),
                etag: meta.etag().map(|s| s.trim_matches('"').to_owned()),
                content_md5: meta.content_md5().map(|s| s.to_owned()),
            });
        }
        Ok(out)
    }

    async fn stat(&self, path: &str) -> Result<Option<EntryMeta>, BackendError> {
        match self.operator.stat(path).await {
            Ok(meta) => Ok(Some(EntryMeta {
                path: path.to_owned(),
                is_dir: meta.mode().is_dir(),
                size: Some(meta.content_length()),
                last_modified: meta.last_modified().map(|t| t.to_rfc3339()),
                // Phase 32g — expose server-side checksum headers.
                // ETag comes back with S3's canonical quoting
                // (`"<md5>"`); strip before comparing.
                etag: meta.etag().map(|s| s.trim_matches('"').to_owned()),
                content_md5: meta.content_md5().map(|s| s.to_owned()),
            })),
            Err(e) if e.kind() == opendal::ErrorKind::NotFound => Ok(None),
            Err(e) => Err(BackendError::OpenDal(e)),
        }
    }

    async fn delete(&self, path: &str) -> Result<(), BackendError> {
        match self.operator.delete(path).await {
            Ok(()) => Ok(()),
            Err(e) if e.kind() == opendal::ErrorKind::NotFound => Ok(()),
            Err(e) => Err(BackendError::OpenDal(e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::backend::{Backend, BackendConfig, BackendKind, LocalFsConfig, make_operator};

    fn local_target() -> (tempfile::TempDir, OperatorTarget) {
        let tmp = tempfile::tempdir().expect("tempdir");
        let backend = Backend {
            name: "test".into(),
            kind: BackendKind::LocalFs,
            config: BackendConfig::LocalFs(LocalFsConfig {
                root: tmp.path().to_string_lossy().into_owned(),
            }),
        };
        let op = make_operator(&backend, None).expect("operator");
        (tmp, OperatorTarget::new("test", op))
    }

    #[tokio::test]
    async fn put_get_round_trip() {
        let (_guard, target) = local_target();
        let payload = Bytes::from_static(b"phase 32 hello");
        target.put("hello.txt", payload.clone()).await.expect("put");
        let back = target.get("hello.txt").await.expect("get");
        assert_eq!(back, payload);
    }

    #[tokio::test]
    async fn stat_returns_none_for_missing() {
        let (_guard, target) = local_target();
        let r = target.stat("missing.bin").await.expect("stat");
        assert!(r.is_none());
    }

    #[tokio::test]
    async fn delete_is_idempotent() {
        let (_guard, target) = local_target();
        target
            .delete("never-existed.txt")
            .await
            .expect("delete-missing");
    }

    #[tokio::test]
    async fn list_after_put() {
        let (_guard, target) = local_target();
        target
            .put("a.bin", Bytes::from_static(b"1"))
            .await
            .expect("put a");
        target
            .put("b.bin", Bytes::from_static(b"22"))
            .await
            .expect("put b");
        let entries = target.list("/").await.expect("list");
        let names: Vec<_> = entries.iter().map(|e| e.path.as_str()).collect();
        assert!(names.iter().any(|p| p.ends_with("a.bin")));
        assert!(names.iter().any(|p| p.ends_with("b.bin")));
    }

    #[tokio::test]
    async fn delete_actually_removes() {
        let (_guard, target) = local_target();
        target
            .put("byebye.txt", Bytes::from_static(b"x"))
            .await
            .expect("put");
        target.delete("byebye.txt").await.expect("delete");
        assert!(target.stat("byebye.txt").await.expect("stat").is_none());
    }
}
