//! Phase 23 — sparse-file extent introspection and preservation.
//!
//! A sparse file is one whose logical length exceeds the sum of its
//! allocated extents: unwritten regions read back as zeros without
//! consuming disk blocks. The copy engine preserves that layout when
//! [`CopyOptions::preserve_sparseness`](crate::CopyOptions::preserve_sparseness)
//! is `true` and a [`SparseOps`] hook is installed.
//!
//! This module stays `#![forbid(unsafe_code)]`-clean: the platform
//! syscalls (`FSCTL_QUERY_ALLOCATED_RANGES` on Windows,
//! `SEEK_HOLE` / `SEEK_DATA` on Linux / macOS) live in
//! `copythat-platform::sparse` behind the [`SparseOps`] trait object.
//! Tests can plug in an in-memory implementation.

use std::io;
use std::path::Path;

/// A contiguous byte range of allocated data inside a file.
///
/// The union of all [`ByteRange`]s returned by
/// [`SparseOps::detect_extents`] describes the file's on-disk footprint;
/// everything outside those ranges is a hole that reads back as zero.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ByteRange {
    /// Byte offset of the first byte in the range.
    pub offset: u64,
    /// Length of the range in bytes.
    pub len: u64,
}

impl ByteRange {
    /// Construct a new byte range from `offset` and `len`.
    pub const fn new(offset: u64, len: u64) -> Self {
        Self { offset, len }
    }

    /// Byte offset one past the last byte of the range.
    pub const fn end(&self) -> u64 {
        self.offset.saturating_add(self.len)
    }

    /// `true` when the range covers zero bytes.
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }
}

/// Summed allocated bytes across a slice of extents.
///
/// Cheap helper used by the engine to decide whether the source is
/// actually sparse: if `allocated_bytes(extents) == total_len` there
/// are no holes and the sparse-aware copy path can be skipped.
pub fn allocated_bytes(extents: &[ByteRange]) -> u64 {
    extents.iter().map(|r| r.len).sum()
}

/// Platform hook for extent introspection and sparse marking.
///
/// Implemented by `copythat-platform::PlatformSparseOps`. The trait is
/// deliberately narrow so alternate backends (in-memory test doubles,
/// FUSE shims) can be dropped in without bringing in the OS FFI
/// machinery.
pub trait SparseOps: Send + Sync + std::fmt::Debug + 'static {
    /// Enumerate the allocated extents of `path`.
    ///
    /// A fully-dense file returns a single `ByteRange { offset: 0,
    /// len: metadata_len }`. A file consisting entirely of holes
    /// returns an empty vec. Filesystems that do not expose extent
    /// queries (e.g. FAT32 / exFAT) should return a single dense
    /// extent so the engine treats the file as non-sparse and copies
    /// the whole content.
    fn detect_extents(&self, path: &Path) -> io::Result<Vec<ByteRange>>;

    /// Mark `path` as a sparse file (Windows `FSCTL_SET_SPARSE` on
    /// NTFS / ReFS). On Linux / macOS this is a no-op: sparseness
    /// emerges naturally from `set_len` + writes that skip ranges.
    fn make_destination_sparse(&self, path: &Path) -> io::Result<()>;

    /// Returns `true` when the filesystem backing `path` supports
    /// sparse files. Used by the engine's pre-flight check: a `false`
    /// answer raises [`CopyEvent::SparsenessNotSupported`] and
    /// densifies the destination.
    ///
    /// [`CopyEvent::SparsenessNotSupported`]: crate::CopyEvent::SparsenessNotSupported
    fn supports_sparse(&self, path: &Path) -> bool;
}

/// A trivial [`SparseOps`] that treats every file as fully dense and
/// refuses to mark destinations as sparse. Useful as a test stub and as
/// the zero-config default when the caller doesn't wire in the
/// platform hook.
#[derive(Debug, Default, Clone, Copy)]
pub struct DenseOnlySparseOps;

impl SparseOps for DenseOnlySparseOps {
    fn detect_extents(&self, path: &Path) -> io::Result<Vec<ByteRange>> {
        let len = std::fs::metadata(path)?.len();
        Ok(vec![ByteRange::new(0, len)])
    }

    fn make_destination_sparse(&self, _path: &Path) -> io::Result<()> {
        Ok(())
    }

    fn supports_sparse(&self, _path: &Path) -> bool {
        false
    }
}

/// Summary of a sparseness-layout mismatch between source and
/// destination. Attached to `CopyError::SparsenessMismatch` so the UI
/// can show "src has N extents, dst has M" without the engine having
/// to reformat the message in every locale.
#[derive(Debug, Clone)]
pub struct SparsenessMismatch {
    /// Source extent layout observed before the copy.
    pub src_extents: Vec<ByteRange>,
    /// Destination extent layout observed after the copy.
    pub dst_extents: Vec<ByteRange>,
}

impl SparsenessMismatch {
    /// Number of allocated extents observed on the source.
    pub fn src_extent_count(&self) -> usize {
        self.src_extents.len()
    }

    /// Number of allocated extents observed on the destination.
    pub fn dst_extent_count(&self) -> usize {
        self.dst_extents.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn byte_range_end_saturates() {
        let r = ByteRange::new(u64::MAX - 10, 100);
        assert_eq!(r.end(), u64::MAX);
    }

    #[test]
    fn allocated_bytes_sums_extents() {
        let extents = vec![
            ByteRange::new(0, 1024),
            ByteRange::new(4096, 512),
            ByteRange::new(10_000, 256),
        ];
        assert_eq!(allocated_bytes(&extents), 1024 + 512 + 256);
    }

    #[test]
    fn allocated_bytes_empty_is_zero() {
        assert_eq!(allocated_bytes(&[]), 0);
    }

    #[test]
    fn dense_only_ops_returns_single_full_extent() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        std::fs::write(tmp.path(), vec![0u8; 4096]).unwrap();
        let ops = DenseOnlySparseOps;
        let extents = ops.detect_extents(tmp.path()).unwrap();
        assert_eq!(extents, vec![ByteRange::new(0, 4096)]);
        assert!(!ops.supports_sparse(tmp.path()));
    }
}
