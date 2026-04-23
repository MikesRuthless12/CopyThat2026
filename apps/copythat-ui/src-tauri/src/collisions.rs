//! Phase 8 — pending-collision registry.
//!
//! Mirrors [`crate::errors::ErrorRegistry`] but for
//! `CopyEvent::Collision` prompts: the runner takes the
//! `oneshot::Sender<CollisionResolution>` out of the event, hands
//! it to this registry, and emits an IPC payload. The Svelte
//! `CollisionModal` invokes `resolve_collision` with the user's
//! choice + an optional `apply_to_all` flag.
//!
//! `apply_to_all` caches the resolution per job. Unlike
//! [`crate::errors::ErrorRegistry`] — which caches by error *kind*
//! — a collision cache is per-job (Overwrite every collision in
//! this operation), because file-by-file collisions are typically
//! the same shape (the user means "yes, overwrite all").

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::{AtomicU64, Ordering};

use copythat_core::CollisionResolution;
use tokio::sync::oneshot;

#[derive(Clone, Default)]
pub struct CollisionRegistry {
    inner: Arc<CollisionRegistryInner>,
}

#[derive(Default)]
struct CollisionRegistryInner {
    pending: Mutex<HashMap<u64, PendingCollision>>,
    /// job_id → cached resolution. Set when a user ticks "Apply to
    /// all" and chooses a non-Abort action.
    apply_all: Mutex<HashMap<u64, CollisionResolution>>,
    next_id: AtomicU64,
}

pub struct PendingCollision {
    pub id: u64,
    pub job_id: u64,
    pub src: PathBuf,
    pub dst: PathBuf,
    pub resolver: oneshot::Sender<CollisionResolution>,
}

impl CollisionRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Stash a prompt. Returns the assigned id. The runner ships
    /// that id in the IPC `collision-raised` event.
    pub fn register(
        &self,
        job_id: u64,
        src: PathBuf,
        dst: PathBuf,
        resolver: oneshot::Sender<CollisionResolution>,
    ) -> u64 {
        let id = self.inner.next_id.fetch_add(1, Ordering::Relaxed) + 1;
        let pending = PendingCollision {
            id,
            job_id,
            src,
            dst,
            resolver,
        };
        self.inner
            .pending
            .lock()
            .expect("collision registry poisoned")
            .insert(id, pending);
        id
    }

    /// Consult the "Apply to all" cache for the given job. The
    /// runner checks this *before* registering a fresh prompt.
    pub fn cached_resolution(&self, job_id: u64) -> Option<CollisionResolution> {
        self.inner
            .apply_all
            .lock()
            .expect("collision registry poisoned")
            .get(&job_id)
            .cloned()
    }

    /// Resolve a pending collision. `apply_to_all` caches the
    /// resolution for every subsequent collision in the same job —
    /// *except* when the resolution is `Abort`, which is terminal
    /// by nature and doesn't want an "apply forever" cache.
    pub fn resolve(
        &self,
        id: u64,
        resolution: CollisionResolution,
        apply_to_all: bool,
    ) -> Result<ResolvedCollision, String> {
        let pending = self
            .inner
            .pending
            .lock()
            .expect("collision registry poisoned")
            .remove(&id)
            .ok_or_else(|| format!("unknown collision id: {id}"))?;

        if apply_to_all && resolution != CollisionResolution::Abort {
            self.inner
                .apply_all
                .lock()
                .expect("collision registry poisoned")
                .insert(pending.job_id, resolution.clone());
        }

        let _ = pending.resolver.send(resolution.clone());

        Ok(ResolvedCollision {
            id: pending.id,
            job_id: pending.job_id,
            resolution,
        })
    }

    pub fn pending_count(&self) -> usize {
        self.inner
            .pending
            .lock()
            .expect("collision registry poisoned")
            .len()
    }
}

#[derive(Debug, Clone)]
pub struct ResolvedCollision {
    pub id: u64,
    pub job_id: u64,
    pub resolution: CollisionResolution,
}

/// Wire name for a `CollisionResolution`. Used in the DTO that goes
/// to the frontend + the CSV exporter for errors that are actually
/// collisions.
pub fn resolution_name(resolution: &CollisionResolution) -> &'static str {
    match resolution {
        CollisionResolution::Skip => "skip",
        CollisionResolution::Overwrite => "overwrite",
        CollisionResolution::Rename(_) => "rename",
        CollisionResolution::Abort => "abort",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn register_then_resolve_fires_oneshot() {
        let reg = CollisionRegistry::new();
        let (tx, rx) = oneshot::channel::<CollisionResolution>();
        let id = reg.register(11, PathBuf::from("/src"), PathBuf::from("/dst"), tx);
        let resolved = reg
            .resolve(id, CollisionResolution::Overwrite, false)
            .unwrap();
        assert_eq!(resolved.id, id);
        assert_eq!(resolved.job_id, 11);
        assert_eq!(rx.await.unwrap(), CollisionResolution::Overwrite);
        assert_eq!(reg.pending_count(), 0);
    }

    #[tokio::test]
    async fn apply_to_all_caches_by_job() {
        let reg = CollisionRegistry::new();
        let (tx, _rx) = oneshot::channel();
        let id = reg.register(11, PathBuf::from("/src"), PathBuf::from("/dst"), tx);
        reg.resolve(id, CollisionResolution::Overwrite, true)
            .unwrap();

        assert_eq!(
            reg.cached_resolution(11),
            Some(CollisionResolution::Overwrite)
        );
        assert_eq!(reg.cached_resolution(12), None);
    }

    #[test]
    fn apply_to_all_refuses_to_cache_abort() {
        let reg = CollisionRegistry::new();
        let (tx, _rx) = oneshot::channel();
        let id = reg.register(11, PathBuf::from("/src"), PathBuf::from("/dst"), tx);
        reg.resolve(id, CollisionResolution::Abort, true).unwrap();

        assert_eq!(reg.cached_resolution(11), None);
    }

    #[test]
    fn resolution_names_are_stable_kebab() {
        assert_eq!(resolution_name(&CollisionResolution::Skip), "skip");
        assert_eq!(
            resolution_name(&CollisionResolution::Overwrite),
            "overwrite"
        );
        assert_eq!(
            resolution_name(&CollisionResolution::Rename("x".into())),
            "rename"
        );
        assert_eq!(resolution_name(&CollisionResolution::Abort), "abort");
    }
}
