//! Shared verification abstraction.
//!
//! The `Hasher` trait and `Verifier` factory type live here so the copy
//! engine can drive verify-on-copy without depending on `copythat-hash`.
//! `copythat-hash` supplies the actual algorithm implementations and a
//! convenience `HashAlgorithm::verifier()` factory.
//!
//! # Lifecycle
//!
//! - On the source side the engine reuses the bytes it's already
//!   reading (`hasher.update(buf)` inside the `fill_buf` / `write_all`
//!   loop) so there's no re-read.
//! - On the destination side the engine opens the freshly-written file
//!   in a post-pass and hashes it independently. Two separate hasher
//!   instances, one comparison at the end.
//!
//! A mismatch produces a `CopyEvent::VerifyFailed` and fails the copy
//! with `CopyErrorKind::VerifyFailed`. Partial-destination cleanup
//! follows the same rules as every other failure path.

use std::sync::Arc;

/// Minimal streaming hash interface.
///
/// Implementations live in `copythat-hash` (one per algorithm). The
/// engine only touches this trait.
pub trait Hasher: Send {
    /// Stable algorithm name — mirrors `HashAlgorithm::name()` in
    /// `copythat-hash`. Used in events and error messages.
    fn name(&self) -> &'static str;
    /// Feed more bytes into the running digest.
    fn update(&mut self, bytes: &[u8]);
    /// Consume the hasher and return the final digest bytes.
    fn finalize(self: Box<Self>) -> Vec<u8>;
}

type Factory = dyn Fn() -> Box<dyn Hasher> + Send + Sync + 'static;

/// Factory for `Hasher` instances. Cloneable, carries a stable
/// algorithm name, and produces a fresh hasher each call — the engine
/// uses one for the source stream and one for the destination
/// post-pass.
#[derive(Clone)]
pub struct Verifier {
    factory: Arc<Factory>,
    name: &'static str,
}

impl Verifier {
    /// Build a verifier from a factory closure. `name` must be stable
    /// (it ends up in events and sidecar filenames) — callers should
    /// pass the value of `HashAlgorithm::name()`.
    pub fn new<F>(name: &'static str, factory: F) -> Self
    where
        F: Fn() -> Box<dyn Hasher> + Send + Sync + 'static,
    {
        Self {
            factory: Arc::new(factory),
            name,
        }
    }

    /// Short algorithm name (e.g. `sha256`, `blake3`).
    pub fn name(&self) -> &'static str {
        self.name
    }

    /// Produce a fresh, zeroed hasher.
    pub fn make(&self) -> Box<dyn Hasher> {
        (self.factory)()
    }
}

impl std::fmt::Debug for Verifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Verifier")
            .field("name", &self.name)
            .finish_non_exhaustive()
    }
}
