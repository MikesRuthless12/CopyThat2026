//! Pause / resume / cancel primitive shared between the caller and the
//! running copy task.
//!
//! Cooperative only: the engine checks the state between buffer-sized
//! reads. A very large individual `fill_buf` could still delay a cancel
//! by up to one buffer's worth of I/O — that's intentional; aborting
//! mid-`write_all` would leak partial writes to disk anyway.

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

/// Handle for steering a running copy. `Clone` + `Send` + `Sync`: stash
/// one in the UI, another in the engine task, they share the same state.
#[derive(Clone)]
pub struct CopyControl {
    shared: Arc<Shared>,
}

impl CopyControl {
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

    /// Resume from pause. No-op if not paused (cancel wins over resume).
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

    /// Terminal: cancel the copy. Once cancelled, stays cancelled.
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

    /// Park the caller while the flag is `PAUSED`. Returns as soon as the
    /// flag is anything else (running OR cancelled) so the engine can
    /// re-check `is_cancelled` on the next line.
    ///
    /// Also used by sibling pipelines (e.g. `copythat-hash`, and the
    /// Phase 4 shredder) that share a `CopyControl` with the copy
    /// engine — if we kept this crate-private they'd have to busy-poll
    /// the pause flag instead.
    pub async fn wait_while_paused(&self) {
        loop {
            let notified = self.shared.notify.notified();
            tokio::pin!(notified);
            // Arm the waiter BEFORE re-reading the flag; otherwise a
            // concurrent `resume`/`cancel` between the read and the arm
            // could wake a notifier we hadn't registered for, causing a
            // permanent park.
            notified.as_mut().enable();
            if self.shared.flag.load(Ordering::Acquire) != PAUSED {
                return;
            }
            notified.await;
        }
    }
}

impl Default for CopyControl {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Debug for CopyControl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self.state() {
            RUNNING => "Running",
            PAUSED => "Paused",
            CANCELLED => "Cancelled",
            _ => "?",
        };
        f.debug_struct("CopyControl").field("state", &s).finish()
    }
}
