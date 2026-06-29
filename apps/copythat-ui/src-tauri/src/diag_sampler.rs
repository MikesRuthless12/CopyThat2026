//! Phase 47 increment 2b — the 1 Hz diagnostics sampler task.
//!
//! While a copy is running, this samples system resources (CPU via
//! [`copythat_diag::SystemSampler`]) plus the aggregate copy throughput,
//! classifies the dominant bottleneck
//! ([`copythat_diag::classify_snapshot`]), and emits [`EVENT_JOB_DIAG`]
//! so the front-end can annotate the live speed graph (cause emoji on
//! dips) and the throughput badge. It's a standalone task spawned from
//! the setup hook, mirroring the power subscriber.
//!
//! Throughput is derived from the aggregate `bytes_done` delta per tick:
//! the Rust side doesn't carry a live per-job rate (the front-end sums
//! those for the global figure), so the delta is the cheapest server-side
//! throughput signal.

use std::sync::Arc;
use std::time::Duration;

use copythat_diag::{Bottleneck, DiagSnapshot, DiskBusy, SystemSampler, classify_snapshot};
use copythat_platform::DiskBusySampler;
use copythat_power::ThermalProbe;
use serde::Serialize;
use tauri::{AppHandle, Emitter};

use crate::runner::{build_globals_for_state, sole_running_job_paths};
use crate::state::AppState;

/// Tauri event carrying one diagnostics sample. The front-end listens to
/// annotate the speed graph + tooltip the throughput badge.
pub const EVENT_JOB_DIAG: &str = "job-diag";

/// Sampling cadence — the Phase 47 spec's 1 Hz.
const DIAG_INTERVAL: Duration = Duration::from_secs(1);

/// Wire form of one diagnostics sample.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DiagDto {
    /// The live system + throughput snapshot.
    pub snapshot: DiagSnapshot,
    /// Classified dominant bottleneck.
    pub bottleneck: Bottleneck,
    /// Cause glyph the speed graph pins on a dip.
    pub cause_emoji: &'static str,
    /// English cause label (the UI localizes via Fluent).
    pub cause_label: &'static str,
}

/// Aggregate throughput (bytes/s) from a `bytes_done` delta over `secs`.
/// Returns `0.0` when bytes regressed — job churn drops a completed job
/// out of the live `bytes_done` sum, which would otherwise read as a
/// negative delta — or when there's no prior reading / no elapsed time.
fn throughput_from_delta(prev: Option<u64>, now: u64, secs: f64) -> f64 {
    match prev {
        Some(p) if now >= p && secs > 0.0 => (now - p) as f64 / secs,
        _ => 0.0,
    }
}

/// Spawn the diagnostics sampler. The returned handle is kept by the
/// caller so the Tauri runtime keeps the task alive for the app's
/// lifetime (the same pattern as the power poller / subscriber).
pub fn spawn_diag_sampler(
    state: AppState,
    app: AppHandle,
    thermal: Arc<dyn ThermalProbe>,
) -> tauri::async_runtime::JoinHandle<()> {
    tauri::async_runtime::spawn(async move {
        let mut sampler = SystemSampler::new();
        // Per-OS disk-busy sampler (`None` on unsupported targets / when the
        // perf source can't open — the disk fields then stay unset and the
        // classifier falls back to CPU / thermal / network signals).
        let mut disk_sampler = DiskBusySampler::new();
        let mut prev_bytes: Option<u64> = None;
        let mut peak_rate: f64 = 0.0;
        loop {
            tokio::time::sleep(DIAG_INTERVAL).await;
            // Keep the disk-busy rate-counter baseline fresh even while idle,
            // so the first sample of a new copy differences io_ticks / PDH
            // `% Disk Time` over ~1 s rather than the whole idle interval.
            if let Some(ds) = disk_sampler.as_mut() {
                ds.tick();
            }
            let g = build_globals_for_state(&state);
            if g.active_jobs == 0 {
                // No copy running — reset the throughput baseline so the
                // next copy starts clean, and don't emit.
                prev_bytes = None;
                peak_rate = 0.0;
                continue;
            }
            let throughput =
                throughput_from_delta(prev_bytes, g.bytes_done, DIAG_INTERVAL.as_secs_f64());
            prev_bytes = Some(g.bytes_done);
            if throughput > peak_rate {
                peak_rate = throughput;
            }
            // Live thermal signal from the shared Phase 31 probe (the same
            // `Arc<dyn ThermalProbe>` the power poller reads). Sampling it
            // here means a thermal throttle is attributed as the bottleneck
            // even when the power *policy* is disabled — the diagnostic is
            // independent of whether we act on it.
            let throttling = thermal.is_throttling().0;
            // Per-volume disk-busy %, attributed only when exactly ONE job is
            // running — otherwise the global throughput and a single job's
            // disk readings would refer to different work and mislabel the
            // bottleneck. UNC / network paths return `None` (handled via the
            // network signal).
            let disks = match (disk_sampler.as_ref(), sole_running_job_paths(&state)) {
                (Some(ds), Some((src, dst))) => DiskBusy {
                    src_pct: ds.busy_pct_for_path(&src),
                    dst_pct: dst.as_deref().and_then(|d| ds.busy_pct_for_path(d)),
                },
                _ => DiskBusy::default(),
            };
            let snapshot = sampler.snapshot(throughput, throttling, disks);
            let bottleneck = classify_snapshot(&snapshot, peak_rate);
            let dto = DiagDto {
                snapshot,
                bottleneck,
                cause_emoji: bottleneck.cause_emoji(),
                cause_label: bottleneck.label(),
            };
            let _ = app.emit(EVENT_JOB_DIAG, dto);
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn throughput_delta_basics() {
        // 1 MiB moved over 1 s.
        assert_eq!(throughput_from_delta(Some(0), 1_048_576, 1.0), 1_048_576.0);
        // No prior reading → 0.
        assert_eq!(throughput_from_delta(None, 1_000, 1.0), 0.0);
        // Regressed bytes (job churn) → 0, never negative.
        assert_eq!(throughput_from_delta(Some(5_000), 1_000, 1.0), 0.0);
        // Zero elapsed → 0 (no divide-by-zero).
        assert_eq!(throughput_from_delta(Some(0), 1_000, 0.0), 0.0);
    }
}
