//! Phase 33 — mount Copy That's chunk store + history archive as a
//! read-only filesystem.
//!
//! # Phase 33a scope (this crate, today)
//!
//! - Pure-Rust virtual-filesystem tree builder. Given a
//!   [`copythat_history::History`] handle + a
//!   [`copythat_chunk::ChunkStore`] handle, [`MountTree::build`]
//!   composes the canonical mount layout (`by-date/` / `by-source/`
//!   / `by-job-id/`) as a node graph. No FUSE / WinFsp IO here — the
//!   tree is the data structure that any of the platform backends
//!   feed their callbacks from.
//! - [`MountHandle`] + [`MountError`] + [`MountBackend`] trait. The
//!   handle's [`Drop`] unmounts.
//! - Test-only [`backends::NoopBackend`] exercising the trait
//!   surface without touching the kernel.
//!
//! Phase 33b (deferred, feature-gated via `fuse` / `winfsp`) wires
//! real `fuser` + `winfsp` backends and adds the History tab
//! context-menu "Mount this job's snapshot" flow + the Settings →
//! Advanced → "Mount latest on launch" toggle.

#![forbid(unsafe_code)]

pub mod backends;
pub mod error;
pub mod handle;
pub mod tree;

pub use backends::{MountBackend, MountSession, NoopBackend};
pub use error::MountError;
pub use handle::MountHandle;
pub use tree::{MountLayout, MountNode, MountTree, NodeKind};
