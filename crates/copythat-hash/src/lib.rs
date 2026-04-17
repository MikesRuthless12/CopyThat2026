//! `copythat-hash` — verification hashing pipeline.
//!
//! # What's here (Phase 3)
//!
//! - A small streaming `Hasher` trait: `update(&mut self, &[u8])` +
//!   `finalize(self: Box<Self>) -> Vec<u8>`.
//! - `HashAlgorithm` enum with Crc32 / Md5 / Sha1 / Sha256 / Sha512 /
//!   XxHash3_64 / XxHash3_128 / Blake3 variants, each with a stable
//!   short `name()`, a `digest_len()` in bytes, and an optional
//!   `sidecar_extension()` for TeraCopy-compatible hash files.
//! - `hash_file_async(path, algo, ctrl, events)` — async streaming
//!   hasher built around `BufReader::fill_buf` with a 1 MiB buffer
//!   (matches the copy engine). Honours a shared `CopyControl` for
//!   pause / resume / cancel and emits `HashEvent`s on an mpsc channel.
//! - `write_sidecar` / `read_sidecar` for the TeraCopy-compatible
//!   `<hex>  <relpath>` one-line-per-file format, and a batched
//!   `write_tree_sidecar` for dumping a whole tree into a single
//!   `.sha256` / `.md5` / `.blake3` file alongside the destination.
//! - A small `verify::verify_pair` helper that hashes two files with
//!   the same algorithm in parallel and returns `VerifyOutcome`.
//!
//! The verify hook in `copythat-core` (`CopyOptions::verify`) calls
//! into this crate: source bytes are hashed *inside* the copy loop
//! (no re-read), destination bytes are hashed with a separate
//! post-pass. On mismatch the engine emits a `VerifyFailed` event and
//! the overall copy is marked failed — exactly TeraCopy's model.
//!
//! # Example — standalone hash
//!
//! ```no_run
//! use copythat_core::CopyControl;
//! use copythat_hash::{hash_file_async, HashAlgorithm, HashEvent};
//! use tokio::sync::mpsc;
//!
//! # async fn demo() -> Result<(), copythat_hash::HashError> {
//! let (tx, mut rx) = mpsc::channel::<HashEvent>(64);
//! let ctrl = CopyControl::new();
//! let task = tokio::spawn(async move {
//!     hash_file_async(
//!         std::path::Path::new("big.iso"),
//!         HashAlgorithm::Blake3,
//!         ctrl,
//!         tx,
//!     )
//!     .await
//! });
//! while let Some(evt) = rx.recv().await {
//!     if let HashEvent::Progress { bytes, total, .. } = evt {
//!         println!("{bytes}/{total}");
//!     }
//! }
//! let report = task.await.unwrap()?;
//! println!("{} {}", report.algorithm, report.hex());
//! # Ok(())
//! # }
//! ```

#![forbid(unsafe_code)]

mod algorithm;
mod error;
mod event;
mod impls;
pub mod sidecar;
mod streaming;
pub mod verify;

pub use algorithm::{HashAlgorithm, Hasher, UnknownAlgorithm};
pub use error::{HashError, HashErrorKind};
pub use event::{HashEvent, HashReport};
pub use streaming::{DEFAULT_HASH_BUFFER, hash_file_async, hash_file_async_with_buffer};
pub use verify::{VerifyOutcome, verify_pair};
