//! Events emitted by `hash_file_async`.
//!
//! The shape deliberately mirrors `copythat_core::CopyEvent` so a UI
//! can route both streams through the same progress UI without
//! special-casing. Hashing rarely runs standalone — most of the time
//! it's invoked by the verify hook in the copy pipeline — but the
//! events are useful on their own for `.sha256` sidecar generation,
//! "Verify against sidecar" flows, etc.

use std::path::PathBuf;
use std::time::Duration;

use crate::algorithm::HashAlgorithm;
use crate::error::HashError;

/// A single event emitted on the `events` channel during a hash run.
/// Dropped sends are tolerated: if the receiver disappears the pipeline
/// keeps computing and stops reporting.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum HashEvent {
    Started {
        path: PathBuf,
        algorithm: HashAlgorithm,
        total_bytes: u64,
    },
    Progress {
        bytes: u64,
        total: u64,
        rate_bps: u64,
    },
    Paused,
    Resumed,
    Completed {
        digest: Vec<u8>,
        bytes: u64,
        duration: Duration,
        rate_bps: u64,
    },
    Failed {
        err: HashError,
    },
}

/// Final success record returned by `hash_file_async`. Mirrors
/// `copythat_core::CopyReport`.
#[derive(Debug, Clone)]
pub struct HashReport {
    pub path: PathBuf,
    pub algorithm: HashAlgorithm,
    pub digest: Vec<u8>,
    pub bytes: u64,
    pub duration: Duration,
    pub rate_bps: u64,
}

impl HashReport {
    /// Hex-encoded lowercase digest.
    pub fn hex(&self) -> String {
        hex::encode(&self.digest)
    }
}
