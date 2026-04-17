//! Shared helpers for copythat-core integration tests.
//!
//! Each integration test file compiles as its own crate, so functions
//! here look "unused" from any single compilation unit even when they
//! are used from the others. The `allow(dead_code)` silences that.

#![allow(dead_code)]

use std::path::Path;
use std::time::Duration;

use copythat_core::{CopyControl, CopyEvent, CopyOptions, copy_file};
use tokio::sync::mpsc;

pub fn write_random(path: &Path, size: usize, seed: u64) -> Vec<u8> {
    use rand::{RngCore, SeedableRng};
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
    let mut buf = vec![0u8; size];
    rng.fill_bytes(&mut buf);
    std::fs::write(path, &buf).unwrap();
    buf
}

pub fn read_all(path: &Path) -> Vec<u8> {
    std::fs::read(path).unwrap()
}

/// Run a copy to completion, returning `(report, events)`.
pub async fn run_copy(
    src: &Path,
    dst: &Path,
    opts: CopyOptions,
) -> (
    Result<copythat_core::CopyReport, copythat_core::CopyError>,
    Vec<CopyEvent>,
) {
    let (tx, rx) = mpsc::channel::<CopyEvent>(256);
    let ctrl = CopyControl::new();
    let src = src.to_path_buf();
    let dst = dst.to_path_buf();
    let task = tokio::spawn(async move { copy_file(&src, &dst, opts, ctrl, tx).await });
    let events = drain(rx).await;
    let report = task.await.unwrap();
    (report, events)
}

pub async fn drain(mut rx: mpsc::Receiver<CopyEvent>) -> Vec<CopyEvent> {
    let mut out = Vec::new();
    while let Some(e) = rx.recv().await {
        out.push(e);
    }
    out
}

pub fn has_variant(events: &[CopyEvent], pred: impl Fn(&CopyEvent) -> bool) -> bool {
    events.iter().any(pred)
}

/// Sleep that still advances under `#[tokio::test(flavor = "current_thread")]`.
pub async fn sleep(d: Duration) {
    tokio::time::sleep(d).await;
}
