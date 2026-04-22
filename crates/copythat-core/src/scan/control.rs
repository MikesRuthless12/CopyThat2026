//! Phase 19a — pause / resume / cancel primitive for a running scan.
//!
//! Mirrors [`crate::control::CopyControl`] exactly: `Arc<AtomicU8>`
//! flag + a `Notify` so parked workers unblock on state change. Kept
//! separate from `CopyControl` so a scan can be paused / cancelled
//! independently of the copy that reads from its cursor.

use std::sync::Arc;
use std::sync::atomic::{AtomicU8, Ordering};

use tokio::sync::Notify;

pub(crate) const RUNNING: u8 = 0;
pub(crate) const PAUSED: u8 = 1;
pub(crate) const CANCELLED: u8 = 2;

struct Shared {
    flag: AtomicU8,
    notify: Notify,
}

/// Cloneable steering handle. `Clone` + `Send` + `Sync`: the UI holds
/// one, the Scanner task holds one, the hash workers hold one.
#[derive(Clone)]
pub struct ScanControl {
    shared: Arc<Shared>,
}

impl ScanControl {
    pub fn new() -> Self {
        Self {
            shared: Arc::new(Shared {
                flag: AtomicU8::new(RUNNING),
                notify: Notify::new(),
            }),
        }
    }

    /// Request a pause. No-op if already paused or cancelled.
    pub fn pause(&self) {
        let _ =
            self.shared
                .flag
                .compare_exchange(RUNNING, PAUSED, Ordering::AcqRel, Ordering::Acquire);
    }

    /// Resume from pause. No-op if not paused (cancel wins).
    pub fn resume(&self) {
        if self
            .shared
            .flag
            .compare_exchange(PAUSED, RUNNING, Ordering::AcqRel, Ordering::Acquire)
            .is_ok()
        {
            self.shared.notify.notify_waiters();
        }
    }

    /// Terminal: cancel the scan. Once cancelled, stays cancelled.
    pub fn cancel(&self) {
        self.shared.flag.store(CANCELLED, Ordering::Release);
        self.shared.notify.notify_waiters();
    }

    pub fn is_paused(&self) -> bool {
        self.shared.flag.load(Ordering::Acquire) == PAUSED
    }

    pub fn is_cancelled(&self) -> bool {
        self.shared.flag.load(Ordering::Acquire) == CANCELLED
    }

    pub(crate) fn state(&self) -> u8 {
        self.shared.flag.load(Ordering::Acquire)
    }

    /// Park the caller while the flag is `PAUSED`. Returns as soon as
    /// it becomes anything else (running OR cancelled) so the caller
    /// can re-check `is_cancelled` on the next line.
    pub async fn wait_while_paused(&self) {
        loop {
            let notified = self.shared.notify.notified();
            tokio::pin!(notified);
            // Arm the waiter BEFORE re-reading the flag, per the
            // pattern documented in `CopyControl::wait_while_paused`.
            notified.as_mut().enable();
            if self.shared.flag.load(Ordering::Acquire) != PAUSED {
                return;
            }
            notified.await;
        }
    }
}

impl Default for ScanControl {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Debug for ScanControl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self.state() {
            RUNNING => "Running",
            PAUSED => "Paused",
            CANCELLED => "Cancelled",
            _ => "?",
        };
        f.debug_struct("ScanControl").field("state", &s).finish()
    }
}
