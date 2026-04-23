//! Steering handle — pause / resume / cancel.
//!
//! Mirrors `copythat_core::CopyControl`: a cheap cloneable handle the
//! runner holds on one side and the engine's inner loop consults on
//! the other. We intentionally keep this separate from `CopyControl`
//! so a pause on the overall sync run doesn't freeze an in-flight
//! `copy_file` invocation (each spawned copy has its own
//! `CopyControl` the engine manages internally).

use std::sync::Arc;
use std::sync::atomic::{AtomicU8, Ordering};

use tokio::sync::Notify;

use crate::error::{Result, SyncError};

/// Steering state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyncState {
    Running,
    Paused,
    Cancelled,
}

impl From<u8> for SyncState {
    fn from(v: u8) -> Self {
        match v {
            0 => Self::Running,
            1 => Self::Paused,
            _ => Self::Cancelled,
        }
    }
}

impl From<SyncState> for u8 {
    fn from(s: SyncState) -> Self {
        match s {
            SyncState::Running => 0,
            SyncState::Paused => 1,
            SyncState::Cancelled => 2,
        }
    }
}

#[derive(Debug)]
struct Inner {
    state: AtomicU8,
    unpause: Notify,
}

#[derive(Debug, Clone)]
pub struct SyncControl {
    inner: Arc<Inner>,
}

impl SyncControl {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Inner {
                state: AtomicU8::new(SyncState::Running.into()),
                unpause: Notify::new(),
            }),
        }
    }

    /// Current state.
    pub fn state(&self) -> SyncState {
        SyncState::from(self.inner.state.load(Ordering::Acquire))
    }

    /// Flip to `Paused`. No-op if already cancelled.
    pub fn pause(&self) {
        let _ = self.inner.state.compare_exchange(
            SyncState::Running.into(),
            SyncState::Paused.into(),
            Ordering::AcqRel,
            Ordering::Acquire,
        );
    }

    /// Flip back to `Running`. No-op if already cancelled.
    pub fn resume(&self) {
        if self
            .inner
            .state
            .compare_exchange(
                SyncState::Paused.into(),
                SyncState::Running.into(),
                Ordering::AcqRel,
                Ordering::Acquire,
            )
            .is_ok()
        {
            self.inner.unpause.notify_waiters();
        }
    }

    /// Flip to `Cancelled`. Wakes any pause waiter so it can observe
    /// the state change and return `Err(Cancelled)`.
    pub fn cancel(&self) {
        self.inner
            .state
            .store(SyncState::Cancelled.into(), Ordering::Release);
        self.inner.unpause.notify_waiters();
    }

    /// Called at every yield-point by the engine.
    ///
    /// `Running` → returns immediately. `Paused` → awaits the
    /// `unpause` notify. `Cancelled` → returns `Err(Cancelled)`
    /// and the engine bubbles up a typed abort.
    pub async fn wait_while_paused(&self) -> Result<()> {
        loop {
            match self.state() {
                SyncState::Running => return Ok(()),
                SyncState::Cancelled => return Err(SyncError::Cancelled),
                SyncState::Paused => {
                    self.inner.unpause.notified().await;
                }
            }
        }
    }

    /// Cheap check — the engine loops test this between cheap work
    /// items without paying for the async path.
    pub fn is_cancelled(&self) -> bool {
        self.state() == SyncState::Cancelled
    }
}

impl Default for SyncControl {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn running_returns_immediately() {
        let c = SyncControl::new();
        c.wait_while_paused().await.unwrap();
    }

    #[tokio::test]
    async fn cancel_surfaces_typed_error() {
        let c = SyncControl::new();
        c.cancel();
        let err = c.wait_while_paused().await.unwrap_err();
        assert!(matches!(err, SyncError::Cancelled));
    }

    #[tokio::test]
    async fn pause_then_resume_unblocks() {
        let c = SyncControl::new();
        c.pause();
        let c2 = c.clone();
        let wait = tokio::spawn(async move { c2.wait_while_paused().await });
        // Give the waiter a tick to park.
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        c.resume();
        wait.await.unwrap().unwrap();
    }
}
