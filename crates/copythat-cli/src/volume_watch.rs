//! Phase 14f — queue-while-locked / volume-arrival watcher.
//!
//! Some users queue copy jobs against a volume that isn't currently
//! mounted (an external SSD that lives in a drawer; a network share
//! that's offline overnight). The traditional path is to keep the
//! job blocked until the user manually re-fires `copythat apply`
//! once the volume comes back. Phase 14f adds a polling watcher
//! that detects when a previously-absent destination root becomes
//! reachable and surfaces the transition through a typed event.
//!
//! The watcher itself is a sync, blocking loop — designed for a
//! `tokio::task::spawn_blocking` worker (the CLI's case) or the
//! GUI's runner thread. Polling cadence defaults to 5 s; the
//! consumer adjusts via [`VolumeWatchOptions::poll_interval`]. We
//! intentionally avoid filesystem-event APIs (inotify, FSEvents,
//! ReadDirectoryChangesW) here — those don't fire on
//! mount/unmount transitions on every platform, and a 5 s poll
//! against `Path::exists` is cheap.
//!
//! ## Threat model
//!
//! - The watcher only reads filesystem state — it never writes.
//!   Consumers receive a `VolumeArrival` event and choose how to
//!   react (typically: re-fire the queued `copythat apply`).
//! - **Phase 17a path-safety bar** — every root path passes through
//!   the lexical traversal guard before the watcher takes ownership.
//!   A `..`-laden root surfaces as `VolumeWatchError::PathRejected`.

use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

use copythat_core::{PathSafetyError, validate_path_no_traversal};
use thiserror::Error;

/// What the watcher emits when a previously-absent root becomes
/// reachable. Plain enum so consumers can match without parsing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VolumeEvent {
    /// `root` was unreachable on a prior tick and is reachable now.
    /// `Path::exists` is the probe, so this fires for both
    /// "external drive plugged in" and "network share came online"
    /// transitions.
    Arrival { root: PathBuf },
    /// `root` was reachable on a prior tick and is unreachable
    /// now. Watchers can use this to re-park a job that was about
    /// to fire.
    Departure { root: PathBuf },
}

/// Configuration knob set for the watcher.
#[derive(Debug, Clone)]
pub struct VolumeWatchOptions {
    /// Cadence between probes. Defaults to 5 seconds.
    pub poll_interval: Duration,
    /// Stop the watcher after this elapsed wall-clock time. `None`
    /// runs forever (until `cancel` is set). Tests use a short
    /// budget so the smoke doesn't hang.
    pub max_run_time: Option<Duration>,
}

impl Default for VolumeWatchOptions {
    fn default() -> Self {
        Self {
            poll_interval: Duration::from_secs(5),
            max_run_time: None,
        }
    }
}

#[derive(Debug, Error)]
pub enum VolumeWatchError {
    #[error("path rejected by Phase 17a safety guard ({offending}): {reason}")]
    PathRejected {
        offending: PathBuf,
        reason: PathSafetyError,
    },
    #[error("the watcher's roots list is empty")]
    NoRoots,
}

impl PartialEq for VolumeWatchError {
    fn eq(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}

/// Cancellation handle. Cloning is intentional — the watcher loop
/// reads `is_cancelled` on every tick, so any consumer can flip it
/// from outside the worker thread.
#[derive(Debug, Default, Clone)]
pub struct VolumeWatchCancel {
    flag: Arc<AtomicBool>,
}

impl VolumeWatchCancel {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn cancel(&self) {
        self.flag.store(true, Ordering::Relaxed);
    }
    pub fn is_cancelled(&self) -> bool {
        self.flag.load(Ordering::Relaxed)
    }
}

/// Probe interface — defaulted to `std::fs::metadata` so production
/// callers get a real check, but the test suite injects a
/// deterministic stub.
pub trait VolumeProbe: Send + Sync {
    /// True iff `root` exists and is a directory at the time of the
    /// call. The watcher tolerates transient errors as `false`
    /// (treats them as "not reachable yet") — that matches the
    /// network-share unmount case.
    fn is_reachable(&self, root: &Path) -> bool;
}

/// Real probe used by the CLI. Crate-public so the GUI runner can
/// re-use it.
pub struct StdProbe;

impl VolumeProbe for StdProbe {
    fn is_reachable(&self, root: &Path) -> bool {
        match std::fs::metadata(root) {
            Ok(md) => md.is_dir(),
            Err(_) => false,
        }
    }
}

/// Run the watcher synchronously. Loops forever (or until
/// `options.max_run_time` elapses, whichever is first), invoking
/// `on_event` for every transition. Returns once the loop exits;
/// the consumer typically pumps this on a `spawn_blocking` worker.
pub fn watch_volumes<P, F>(
    roots: &[PathBuf],
    options: VolumeWatchOptions,
    cancel: VolumeWatchCancel,
    probe: P,
    mut on_event: F,
) -> Result<(), VolumeWatchError>
where
    P: VolumeProbe,
    F: FnMut(VolumeEvent),
{
    if roots.is_empty() {
        return Err(VolumeWatchError::NoRoots);
    }
    for root in roots {
        validate_path_no_traversal(root).map_err(|e| VolumeWatchError::PathRejected {
            offending: root.clone(),
            reason: e,
        })?;
    }

    // Sample once up front so we have a baseline; transitions are
    // edge-detected against the previous tick.
    let mut last_state: Vec<bool> = roots.iter().map(|r| probe.is_reachable(r)).collect();

    let started_at = Instant::now();
    // Break the inter-poll wait into short slices so cancellation
    // latency stays bounded (the §4.11g requirement is ≤ 2 s, and a
    // typical poll_interval is 5 s — sleeping the whole interval
    // would miss the bound). 100 ms ticks give effectively-immediate
    // cancellation while keeping the per-second wakeup count to ~10.
    const CANCEL_TICK: Duration = Duration::from_millis(100);
    'outer: loop {
        if cancel.is_cancelled() {
            break;
        }
        if let Some(budget) = options.max_run_time {
            if started_at.elapsed() >= budget {
                break;
            }
        }
        let mut waited = Duration::ZERO;
        while waited < options.poll_interval {
            if cancel.is_cancelled() {
                break 'outer;
            }
            if let Some(budget) = options.max_run_time {
                if started_at.elapsed() >= budget {
                    break 'outer;
                }
            }
            let slice = (options.poll_interval - waited).min(CANCEL_TICK);
            std::thread::sleep(slice);
            waited += slice;
        }
        if cancel.is_cancelled() {
            break;
        }

        for (i, root) in roots.iter().enumerate() {
            let now = probe.is_reachable(root);
            let prev = last_state[i];
            if now == prev {
                continue;
            }
            if now {
                on_event(VolumeEvent::Arrival { root: root.clone() });
            } else {
                on_event(VolumeEvent::Departure { root: root.clone() });
            }
            last_state[i] = now;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;
    use tempfile::TempDir;

    /// Test probe — flips arms based on which paths exist in the
    /// `present` set, so the test owns the transition timeline
    /// without racing the real filesystem. Wraps `present` in a
    /// `Mutex` so the test can mutate the set between ticks.
    struct ScriptedProbe {
        present: Arc<Mutex<Vec<PathBuf>>>,
    }
    impl VolumeProbe for ScriptedProbe {
        fn is_reachable(&self, root: &Path) -> bool {
            let p = self.present.lock().unwrap();
            p.iter().any(|x| x.as_path() == root)
        }
    }

    #[test]
    fn empty_roots_is_an_error() {
        let err = watch_volumes(
            &[],
            VolumeWatchOptions::default(),
            VolumeWatchCancel::new(),
            StdProbe,
            |_| {},
        )
        .unwrap_err();
        assert!(matches!(err, VolumeWatchError::NoRoots));
    }

    #[test]
    fn rejects_traversal_root() {
        let err = watch_volumes(
            &[PathBuf::from("foo/../etc/passwd")],
            VolumeWatchOptions::default(),
            VolumeWatchCancel::new(),
            StdProbe,
            |_| {},
        )
        .unwrap_err();
        assert!(matches!(err, VolumeWatchError::PathRejected { .. }));
    }

    #[test]
    fn detects_arrival_then_departure() {
        let dir = TempDir::new().unwrap();
        let root = dir.path().to_path_buf();

        let present = Arc::new(Mutex::new(Vec::<PathBuf>::new()));
        let probe = ScriptedProbe {
            present: present.clone(),
        };
        let cancel = VolumeWatchCancel::new();
        let observed: Arc<Mutex<Vec<VolumeEvent>>> = Arc::new(Mutex::new(Vec::new()));
        let observed_for_loop = observed.clone();

        // Drive the loop on a background thread so the test can
        // mutate `present` between ticks.
        let cancel_for_loop = cancel.clone();
        let roots_for_loop = vec![root.clone()];
        let handle = std::thread::spawn(move || {
            let opts = VolumeWatchOptions {
                poll_interval: Duration::from_millis(20),
                max_run_time: Some(Duration::from_secs(2)),
            };
            watch_volumes(&roots_for_loop, opts, cancel_for_loop, probe, move |evt| {
                observed_for_loop.lock().unwrap().push(evt);
            })
        });

        // Tick 0: empty -> no event.
        std::thread::sleep(Duration::from_millis(60));
        // Make the root "appear" and wait long enough for two
        // poll intervals to fire.
        present.lock().unwrap().push(root.clone());
        std::thread::sleep(Duration::from_millis(120));
        // Make it "disappear".
        present.lock().unwrap().clear();
        std::thread::sleep(Duration::from_millis(120));
        cancel.cancel();

        let result = handle.join().unwrap();
        assert!(result.is_ok(), "watcher exited with {result:?}");

        let events = observed.lock().unwrap().clone();
        // Expect at least an Arrival followed by a Departure.
        let arrival_idx = events
            .iter()
            .position(|e| matches!(e, VolumeEvent::Arrival { .. }));
        let departure_idx = events
            .iter()
            .position(|e| matches!(e, VolumeEvent::Departure { .. }));
        assert!(
            arrival_idx.is_some(),
            "no Arrival event observed: {events:?}",
        );
        assert!(
            departure_idx.is_some(),
            "no Departure event observed: {events:?}",
        );
        assert!(
            arrival_idx.unwrap() < departure_idx.unwrap(),
            "Arrival must precede Departure: {events:?}",
        );
    }

    #[test]
    fn cancellation_terminates_the_loop_promptly() {
        let dir = TempDir::new().unwrap();
        let root = dir.path().to_path_buf();
        let present = Arc::new(Mutex::new(vec![root.clone()]));
        let probe = ScriptedProbe {
            present: present.clone(),
        };
        let cancel = VolumeWatchCancel::new();
        let cancel_for_loop = cancel.clone();
        let roots_for_loop = vec![root];
        let started = Instant::now();
        let handle = std::thread::spawn(move || {
            watch_volumes(
                &roots_for_loop,
                VolumeWatchOptions {
                    poll_interval: Duration::from_millis(50),
                    max_run_time: Some(Duration::from_secs(10)),
                },
                cancel_for_loop,
                probe,
                |_| {},
            )
        });
        std::thread::sleep(Duration::from_millis(80));
        cancel.cancel();
        let result = handle.join().unwrap();
        assert!(result.is_ok());
        // Should exit well under the 10 s budget.
        assert!(started.elapsed() < Duration::from_secs(2));
    }
}
