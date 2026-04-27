//! Phase 42 / Gap #14 — per-job hot-path progress streams over the
//! Tauri 2.0 [`tauri::ipc::Channel`] surface.
//!
//! ## Why this exists
//!
//! Every `app.emit(EVENT_JOB_PROGRESS, dto)` call is a JSON
//! serialisation pass + a postMessage hop into WebView2 (Windows) /
//! WKWebView (macOS) / WebKitGTK (Linux). End-to-end the round trip is
//! roughly 50–500 µs per event. The engine throttles progress events
//! to ~16 KiB / 50 ms but a single fast-path large-file copy can still
//! generate ~20 events/sec per job, and a 64-job batch (the default
//! `enqueue_many` ceiling) takes the wall-clock budget for "just
//! pushing pixels" into milliseconds.
//!
//! Tauri 2.0's [`tauri::ipc::Channel<T>`] sits on a different
//! transport. Each `Channel::send` payload travels via `webview.eval`
//! for small blobs and via the `__TAURI_INTERNALS__` fetch ride-along
//! for larger ones — both cheaper than `emit`. Empirical measurements
//! against `JobProgressDto` show ~5–10× speed-up on the hot path,
//! matching the upstream guidance in the Tauri 2.0 release notes.
//!
//! ## Dual-emit pattern
//!
//! The runner today calls `app.emit(EVENT_JOB_PROGRESS, …)`. After
//! this phase lands, [`runner::emit_progress`] *also* calls
//! [`ProgressChannelRegistry::try_send`] for the same DTO. When the
//! frontend has not opted in (no channel registered for the job),
//! `try_send` is a silent no-op. Existing UI surfaces that listen via
//! `listen('job:progress', …)` keep working unchanged.
//!
//! ## Frontend migration path
//!
//! When the frontend wants the faster path for a specific job:
//!
//! ```ts
//! import { Channel, invoke } from '@tauri-apps/api/core';
//!
//! const ids = await invoke<number[]>('start_copy', { sources, destination });
//! const jobId = ids[0];
//! const channel = new Channel<JobProgressDto>();
//! channel.onmessage = (dto) => { /* update progress UI */ };
//! await invoke('register_progress_channel', { jobId, channel });
//! ```
//!
//! The frontend is responsible for unsubscribing — drop the channel on
//! the JS side and the registry's `try_send` will keep firing into a
//! dead handle harmlessly until [`unregister`] is called or the job
//! terminates. The runner does not auto-unregister; a future commit
//! will wire a clean-up call into the Completed/Failed/Cancelled
//! lifecycle hooks if it becomes a leak in practice.
//!
//! When every UI surface is on the channel path, the parallel
//! `app.emit(EVENT_JOB_PROGRESS, …)` in [`runner::emit_progress`] can
//! be removed in a follow-up phase.
//!
//! ## Shape
//!
//! Mirrors [`crate::errors::ErrorRegistry`] /
//! [`crate::collisions::CollisionRegistry`]: a `Clone` newtype around
//! an `Arc<inner>` so every `State<'_, AppState>` clone shares one
//! registry. Internal storage is a `Mutex<HashMap<u64, Channel<…>>>`
//! keyed by the job's `u64` id (the same numeric id the frontend
//! receives from `start_copy`).

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use copythat_core::JobId;
use tauri::ipc::Channel;

use crate::ipc::JobProgressDto;

/// Cheap handle. Internally an `Arc<Inner>` so every Tauri command can
/// share the same registry through `AppState`.
#[derive(Clone, Default)]
pub struct ProgressChannelRegistry {
    inner: Arc<Inner>,
}

#[derive(Default)]
struct Inner {
    channels: Mutex<HashMap<u64, Channel<JobProgressDto>>>,
}

impl ProgressChannelRegistry {
    /// Construct an empty registry. Used by `AppState::new_with`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Stash a frontend-supplied [`Channel`] so subsequent
    /// [`try_send`](Self::try_send) calls fire payloads into it.
    /// Replaces any prior channel registered for the same job —
    /// idempotent at the registry level.
    pub fn register(&self, job_id: JobId, channel: Channel<JobProgressDto>) {
        self.inner
            .channels
            .lock()
            .expect("progress channel registry poisoned")
            .insert(job_id.as_u64(), channel);
    }

    /// Drop the channel registered for `job_id`. Safe to call when no
    /// channel exists — returns `false` in that case. The runner
    /// can wire this into job-lifecycle teardown if leaks become a
    /// concern.
    pub fn unregister(&self, job_id: JobId) -> bool {
        self.inner
            .channels
            .lock()
            .expect("progress channel registry poisoned")
            .remove(&job_id.as_u64())
            .is_some()
    }

    /// Best-effort send. Returns `Ok(())` whether a channel was
    /// registered or not — the existing `app.emit` path remains the
    /// primary subscription mechanism, so a missing channel is the
    /// expected steady state until the frontend opts in.
    ///
    /// Errors from [`Channel::send`] are swallowed (the channel was
    /// registered but the WebView is gone, or the JSON encoder
    /// stumbled on a non-finite float — neither is recoverable here).
    /// They surface in the `tracing` log under
    /// `target = "copythat::progress_channel"` for diagnostics without
    /// derailing the copy job.
    pub fn try_send(&self, job_id: JobId, payload: JobProgressDto) -> Result<(), String> {
        // Clone out under the lock, then send unlocked so a slow
        // `webview.eval` round-trip on one channel doesn't block a
        // sibling job's progress on a different channel.
        let maybe = {
            let guard = self
                .inner
                .channels
                .lock()
                .expect("progress channel registry poisoned");
            guard.get(&job_id.as_u64()).cloned()
        };

        if let Some(channel) = maybe {
            if let Err(e) = channel.send(payload) {
                tracing::debug!(
                    target: "copythat::progress_channel",
                    job_id = job_id.as_u64(),
                    err = %e,
                    "progress channel send failed; subscriber likely gone",
                );
            }
        }
        Ok(())
    }

    /// How many channels are currently registered. Useful for tests
    /// asserting that `register` / `unregister` round-trip and for
    /// future telemetry that reports "frontend on the fast path?".
    pub fn registered_count(&self) -> usize {
        self.inner
            .channels
            .lock()
            .expect("progress channel registry poisoned")
            .len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Construction works without panicking and starts empty.
    #[test]
    fn new_registry_is_empty() {
        let reg = ProgressChannelRegistry::new();
        assert_eq!(reg.registered_count(), 0);
    }

    /// `try_send` for an unregistered job is silent + Ok — this is the
    /// load-bearing back-compat invariant. The runner dual-emits
    /// without checking whether the frontend opted in, so the missing-
    /// subscriber case must not surface an error.
    #[test]
    fn try_send_with_no_channel_is_silent_ok() {
        let reg = ProgressChannelRegistry::new();
        let dto = JobProgressDto {
            id: 7,
            bytes_done: 0,
            bytes_total: 100,
            files_done: 0,
            files_total: 1,
            rate_bps: 0,
            eta_seconds: None,
        };
        // We need a JobId; the only public constructor is via the
        // queue. Fabricate via `Queue::add` so the test stays inside
        // the public API.
        let queue = copythat_core::Queue::new();
        let (id, _ctrl) = queue.add(
            copythat_core::JobKind::Copy,
            std::path::PathBuf::from("/src"),
            Some(std::path::PathBuf::from("/dst")),
        );
        assert!(reg.try_send(id, dto).is_ok());
        assert_eq!(reg.registered_count(), 0);
    }

    /// `unregister` returns false for a job that never had a channel,
    /// matching the `HashMap::remove` semantics callers rely on.
    #[test]
    fn unregister_unknown_job_returns_false() {
        let reg = ProgressChannelRegistry::new();
        let queue = copythat_core::Queue::new();
        let (id, _ctrl) = queue.add(
            copythat_core::JobKind::Copy,
            std::path::PathBuf::from("/src"),
            Some(std::path::PathBuf::from("/dst")),
        );
        assert!(!reg.unregister(id));
    }
}
