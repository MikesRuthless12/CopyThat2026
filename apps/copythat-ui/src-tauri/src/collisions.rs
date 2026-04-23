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
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::{AtomicU64, Ordering};

use copythat_core::CollisionResolution;
use copythat_settings::{ConflictProfile, ConflictRule, ConflictRuleResolution};
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
    /// Phase 22 — per-job accumulated pattern rules. Seeded from the
    /// active `ConflictProfile` when a job starts; appended to as
    /// the user clicks "Apply to all of this extension" / "Apply to
    /// glob" in the aggregate dialog. First-match-wins semantics
    /// within the job.
    rules: Mutex<HashMap<u64, ConflictProfile>>,
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

    /// Phase 22 — seed a job's per-job rule set from the user's
    /// persisted active profile. Called by the runner right after
    /// `enqueue_jobs` allocates a fresh `JobId`. Idempotent — a
    /// later call overwrites the seed (useful for tests that
    /// re-seed mid-run).
    pub fn seed_rules(&self, job_id: u64, profile: ConflictProfile) {
        self.inner
            .rules
            .lock()
            .expect("collision registry poisoned")
            .insert(job_id, profile);
    }

    /// Append one rule to the end of a job's rule list. The UI
    /// invokes this via the `add_conflict_rule` IPC when the user
    /// clicks "Apply to all of this extension" / "Apply to glob"
    /// on a running job. Creates an empty profile for the job on
    /// first call.
    pub fn append_rule(&self, job_id: u64, rule: ConflictRule) {
        self.inner
            .rules
            .lock()
            .expect("collision registry poisoned")
            .entry(job_id)
            .or_default()
            .rules
            .push(rule);
    }

    /// Consult the job's accumulated rules for `src_path`. Returns
    /// `Some((resolution, matched_pattern))` when a rule or
    /// fallback fires; `None` when the user must still be
    /// prompted. The runner peeks source/dest metadata and calls
    /// [`apply_rule_resolution`] to translate the rule hit into
    /// the engine's four-variant `CollisionResolution`.
    pub fn consult_rules(
        &self,
        job_id: u64,
        src_path: &Path,
        rel_path: Option<&str>,
    ) -> Option<(ConflictRuleResolution, String)> {
        let rules = self.inner.rules.lock().expect("collision registry poisoned");
        let profile = rules.get(&job_id)?;
        if profile.is_empty() {
            return None;
        }
        let basename = src_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");
        let rel = rel_path.unwrap_or(basename);
        let hit = profile.match_basename_or_path(basename, rel)?;
        Some((hit.resolution, hit.pattern.to_string()))
    }

    /// Snapshot of a job's current rule list — UI renders this in
    /// the profile-save dialog so users can see what will be
    /// persisted. `None` when the job has no rules recorded.
    pub fn rules_for(&self, job_id: u64) -> Option<ConflictProfile> {
        self.inner
            .rules
            .lock()
            .expect("collision registry poisoned")
            .get(&job_id)
            .cloned()
    }

    /// Remove all per-job state (pending, apply-all cache, rules)
    /// when a job finishes. Runner calls on Completed / Failed /
    /// Cancelled so a later job with the same id (impossible today
    /// but defensive) doesn't inherit stale rules.
    pub fn clear_job(&self, job_id: u64) {
        self.inner
            .apply_all
            .lock()
            .expect("collision registry poisoned")
            .remove(&job_id);
        self.inner
            .rules
            .lock()
            .expect("collision registry poisoned")
            .remove(&job_id);
    }
}

/// Translate a `ConflictRuleResolution` into the engine's
/// four-variant `CollisionResolution`, using source/destination
/// metadata for the mtime- and size-comparison variants. `dst`
/// is the destination path the engine was about to write — needed
/// because `KeepBoth` generates a free filename relative to it.
///
/// Returns `None` only in the vanishingly-rare case where a
/// `KeepBoth` rule fires but no free suffix was found within 10000
/// attempts — in that case the runner falls through to the
/// interactive prompt rather than silently skipping.
pub fn apply_rule_resolution(
    resolution: ConflictRuleResolution,
    src_size: Option<u64>,
    dst_size: Option<u64>,
    src_modified_ms: Option<i64>,
    dst_modified_ms: Option<i64>,
    dst: &Path,
) -> Option<CollisionResolution> {
    match resolution {
        ConflictRuleResolution::Skip => Some(CollisionResolution::Skip),
        ConflictRuleResolution::Overwrite => Some(CollisionResolution::Overwrite),
        ConflictRuleResolution::OverwriteIfNewer => {
            let s = src_modified_ms.unwrap_or(i64::MIN);
            let d = dst_modified_ms.unwrap_or(i64::MIN);
            Some(if s > d {
                CollisionResolution::Overwrite
            } else {
                CollisionResolution::Skip
            })
        }
        ConflictRuleResolution::OverwriteIfLarger => {
            let s = src_size.unwrap_or(0);
            let d = dst_size.unwrap_or(0);
            Some(if s > d {
                CollisionResolution::Overwrite
            } else {
                CollisionResolution::Skip
            })
        }
        ConflictRuleResolution::KeepBoth => {
            let fresh = next_keep_both_name(dst)?;
            Some(CollisionResolution::Rename(fresh))
        }
    }
}

/// Mirror of the engine's `keep_both_path` naming: `foo.txt` →
/// `foo_2.txt`, then `_3`, `_4`… Returns the bare filename (no
/// directory component) because `CollisionResolution::Rename`
/// takes a filename that lands in the same parent as the
/// original destination. Gives up at 10000.
fn next_keep_both_name(dst: &Path) -> Option<String> {
    let parent = dst.parent()?;
    let stem = dst.file_stem()?.to_string_lossy().into_owned();
    let ext = dst
        .extension()
        .map(|e| e.to_string_lossy().into_owned())
        .unwrap_or_default();
    for n in 2..10_000 {
        let candidate_name = if ext.is_empty() {
            format!("{stem}_{n}")
        } else {
            format!("{stem}_{n}.{ext}")
        };
        let candidate = parent.join(&candidate_name);
        if !candidate.exists() {
            return Some(candidate_name);
        }
    }
    None
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

    #[test]
    fn consult_rules_matches_extension_glob() {
        let reg = CollisionRegistry::new();
        reg.seed_rules(
            99,
            ConflictProfile::with_rules(vec![ConflictRule {
                pattern: "*.txt".into(),
                resolution: ConflictRuleResolution::Skip,
            }]),
        );
        let hit = reg.consult_rules(99, Path::new("/src/notes.txt"), Some("notes.txt"));
        assert!(matches!(hit, Some((ConflictRuleResolution::Skip, _))));
        let miss = reg.consult_rules(99, Path::new("/src/photo.jpg"), Some("photo.jpg"));
        assert!(miss.is_none());
    }

    #[test]
    fn consult_rules_returns_none_for_unseeded_job() {
        let reg = CollisionRegistry::new();
        assert!(
            reg.consult_rules(42, Path::new("/src/a.txt"), Some("a.txt"))
                .is_none()
        );
    }

    #[test]
    fn append_rule_grows_the_job_profile() {
        let reg = CollisionRegistry::new();
        reg.append_rule(
            5,
            ConflictRule {
                pattern: "*.docx".into(),
                resolution: ConflictRuleResolution::OverwriteIfNewer,
            },
        );
        reg.append_rule(
            5,
            ConflictRule {
                pattern: "*.tmp".into(),
                resolution: ConflictRuleResolution::Skip,
            },
        );
        let hit = reg
            .consult_rules(5, Path::new("/src/a.docx"), Some("a.docx"))
            .unwrap();
        assert_eq!(hit.0, ConflictRuleResolution::OverwriteIfNewer);
        assert_eq!(hit.1, "*.docx");
    }

    #[test]
    fn apply_rule_resolution_newer_wins_by_mtime() {
        let out = apply_rule_resolution(
            ConflictRuleResolution::OverwriteIfNewer,
            Some(100),
            Some(100),
            Some(2000),
            Some(1000),
            Path::new("/dst/a.txt"),
        )
        .unwrap();
        assert_eq!(out, CollisionResolution::Overwrite);

        let out = apply_rule_resolution(
            ConflictRuleResolution::OverwriteIfNewer,
            Some(100),
            Some(100),
            Some(500),
            Some(1000),
            Path::new("/dst/a.txt"),
        )
        .unwrap();
        assert_eq!(out, CollisionResolution::Skip);
    }

    #[test]
    fn apply_rule_resolution_larger_wins_by_size() {
        let out = apply_rule_resolution(
            ConflictRuleResolution::OverwriteIfLarger,
            Some(500),
            Some(100),
            None,
            None,
            Path::new("/dst/a.txt"),
        )
        .unwrap();
        assert_eq!(out, CollisionResolution::Overwrite);

        let out = apply_rule_resolution(
            ConflictRuleResolution::OverwriteIfLarger,
            Some(10),
            Some(500),
            None,
            None,
            Path::new("/dst/a.txt"),
        )
        .unwrap();
        assert_eq!(out, CollisionResolution::Skip);
    }

    #[test]
    fn apply_rule_resolution_keep_both_generates_suffix() {
        let tmp = tempfile::tempdir().unwrap();
        let dst = tmp.path().join("report.docx");
        std::fs::write(&dst, b"occupied").unwrap();
        let out = apply_rule_resolution(
            ConflictRuleResolution::KeepBoth,
            Some(1),
            Some(1),
            None,
            None,
            &dst,
        )
        .unwrap();
        match out {
            CollisionResolution::Rename(name) => assert_eq!(name, "report_2.docx"),
            other => panic!("expected Rename(report_2.docx), got {other:?}"),
        }
    }

    #[test]
    fn clear_job_removes_rules_and_apply_all() {
        let reg = CollisionRegistry::new();
        reg.append_rule(
            7,
            ConflictRule {
                pattern: "*".into(),
                resolution: ConflictRuleResolution::Skip,
            },
        );
        // apply-all cache goes via `resolve(..., true)`; drop a fake
        // entry by registering + resolving with apply-to-all.
        let (tx, _rx) = oneshot::channel();
        let id = reg.register(7, PathBuf::from("/s"), PathBuf::from("/d"), tx);
        reg.resolve(id, CollisionResolution::Overwrite, true).unwrap();
        assert!(reg.cached_resolution(7).is_some());
        assert!(reg.rules_for(7).is_some());

        reg.clear_job(7);
        assert!(reg.cached_resolution(7).is_none());
        assert!(reg.rules_for(7).is_none());
    }
}
