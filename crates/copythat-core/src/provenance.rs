//! Phase 43 — forensic chain-of-custody contract.
//!
//! `copythat-provenance` (sibling crate) owns the manifest format,
//! signing, RFC 3161 timestamping, and the verify command. This module
//! ships the engine-side contract: the [`OutboardEncoder`] trait that
//! the engine feeds source bytes into during the read pass, and the
//! [`ProvenanceSink`] trait that accumulates per-file [`FileRecord`]s
//! once the bytes are finalized.
//!
//! The engine reuses the bytes it's already reading for the verify
//! hasher — provenance is "free" in the same way; one extra
//! `encoder.update(buf)` per buffered read.
//!
//! # Why split engine / sink
//!
//! Single-file `copy_file` doesn't know about jobs. The manifest is
//! per-job (multi-file). Splitting the trait surface into "feed
//! per-file" (engine) and "accumulate + finalize" (sink — owned by the
//! caller) keeps the engine's hot path job-agnostic and lets the
//! tree-walker / job runner own manifest formation.
//!
//! # Privacy
//!
//! The manifest publishes file paths, sizes, and BLAKE3 digests. It
//! does NOT publish file contents. The signing key never leaves the
//! caller's keyring; the engine never sees it.

use std::path::Path;
use std::sync::Arc;

/// Streaming encoder that consumes source bytes and finalizes to a
/// (BLAKE3 root, Bao outboard) pair.
///
/// The engine creates one of these per file via
/// [`ProvenanceSink::make_encoder`], feeds it from the same
/// `fill_buf` loop that drives the verify hasher, and hands the
/// finalized result back to the sink via
/// [`ProvenanceSink::record_file`].
///
/// Implementations live in `copythat-provenance`. The default
/// concrete implementation wraps `bao::encode::Encoder` for
/// verified-streaming outboards; a stub implementation ships in this
/// crate's tests (BLAKE3 root only, empty outboard) so engine wiring
/// can be exercised without pulling the bao crate into the core
/// build.
pub trait OutboardEncoder: Send {
    /// Feed more source bytes into the encoder. Cheap; expected on
    /// the copy hot path.
    fn update(&mut self, bytes: &[u8]);

    /// Consume the encoder and return the (32-byte BLAKE3 root,
    /// Bao verified-streaming outboard bytes) pair. The outboard may
    /// be empty when the implementation skips Bao tree construction
    /// (e.g. the test stub or a future "root-only" mode).
    fn finalize(self: Box<Self>) -> ([u8; 32], Vec<u8>);
}

/// Bridge contract for the per-job provenance manifest aggregator.
///
/// Implemented by `copythat_provenance::CopyThatProvenanceSink`. Kept
/// in `copythat-core` so [`crate::CopyOptions`] can hold a trait
/// object without pulling `copythat-provenance` (and its `bao` /
/// `ed25519-dalek` / `ciborium` deps) into every consumer of the
/// engine's public surface.
///
/// Lifecycle:
/// 1. Caller constructs the sink (with src_root, dst_root, signing
///    key, TSA URL, etc.) and stashes it in
///    [`crate::CopyOptions::provenance`].
/// 2. The engine, per file, calls [`ProvenanceSink::make_encoder`],
///    feeds the encoder during the source-read pass, then calls
///    [`ProvenanceSink::record_file`] with the finalized root +
///    outboard.
/// 3. After the job (tree-walk) completes, the caller invokes the
///    sink's own `finalize_to_path` (not part of this trait — the
///    caller already holds the concrete `Arc`) to build the canonical
///    CBOR manifest, sign it, optionally request a timestamp, and
///    write it to disk.
///
/// All methods take `&self` so the sink can use interior mutability
/// (a Mutex) and remain `Send + Sync`.
pub trait ProvenanceSink: Send + Sync + std::fmt::Debug {
    /// Make a fresh encoder for one file. The engine consumes the
    /// returned `Box<dyn OutboardEncoder>` to completion via
    /// `update` + `finalize` and hands the result back to
    /// [`Self::record_file`].
    fn make_encoder(&self) -> Box<dyn OutboardEncoder>;

    /// Record one file's contribution to the manifest. Called exactly
    /// once per successfully-copied file, AFTER the byte copy + verify
    /// pass complete (so the recorded digest matches what's on disk).
    ///
    /// Implementations are expected to derive the manifest's
    /// `rel_path` from the source path against the job-level src_root
    /// the sink was constructed with. Best-effort: a relative-path
    /// failure should still record a usable rel_path (e.g. the file
    /// name) rather than abort.
    fn record_file(
        &self,
        src: &Path,
        dst: &Path,
        size: u64,
        blake3_root: [u8; 32],
        bao_outboard: Vec<u8>,
    );
}

/// Per-job provenance configuration.
///
/// Stashed in [`crate::CopyOptions::provenance`]. When `Some`, the
/// engine drives the contract above — when `None`, the engine skips
/// every provenance code path.
#[derive(Clone)]
pub struct ProvenancePolicy {
    /// Per-job aggregator + encoder factory. Concrete implementation
    /// lives in `copythat-provenance`.
    pub sink: Arc<dyn ProvenanceSink>,
}

impl std::fmt::Debug for ProvenancePolicy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ProvenancePolicy")
            .field("sink", &self.sink)
            .finish()
    }
}
