//! Sync engine — the `sync()` entry point, the decision matrix, and
//! the action executor.
//!
//! The engine is strictly serial per pair: one walk pass, one
//! decision pass, one execution pass. A single pair stays within one
//! tokio task so there's no cross-thread coordination to reason
//! about. Parallelism across pairs is the runner's concern.
//!
//! # Winner selection
//!
//! When both sides diverged concurrently from a common ancestor, the
//! engine picks a `winner` by mtime (later wins; ties go to `Left`
//! for determinism). The loser's current content is preserved on its
//! own side as `name.sync-conflict-YYYYMMDD-HHMMSS-<host>.ext`, and
//! the winner's content is propagated to the loser's canonical path.
//! Both sides are still surfaced to the UI so the user can override
//! the automatic choice.

use std::path::Path;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use chrono::{TimeZone, Utc};
use tokio::sync::mpsc;

use copythat_core::{CopyControl, CopyEvent, CopyOptions, copy_file};

use crate::clock::resolve_concurrent;
use crate::control::SyncControl;
use crate::db::{FileRecord, HistoryEntry, HistoryKind, SyncDb};
use crate::error::{Result, SyncError};
use crate::types::{
    Conflict, ConflictKind, CopyHookFactory, Direction, FileMeta, SideState, SyncAction, SyncEvent,
    SyncMode, SyncOptions, SyncPair, SyncReport,
};
use crate::walker::{join_relpath, scan_side};

/// The one public entry point.
///
/// Runs one reconciliation round of `pair` under `mode`. Emits
/// progress events on `events`; returns a [`SyncReport`] summary on
/// completion.
///
/// The pair's DB is opened (or created) at `pair.db_path`. The DB
/// file itself is excluded from the walk, so placing it under the
/// left root by default is safe.
pub async fn sync(
    pair: &SyncPair,
    mode: SyncMode,
    opts: SyncOptions,
    ctrl: SyncControl,
    events: mpsc::Sender<SyncEvent>,
) -> Result<SyncReport> {
    let start = SystemTime::now();
    let db = SyncDb::open(&pair.db_path)?;

    ctrl.wait_while_paused().await?;
    let _ = events
        .send(SyncEvent::WalkStarted {
            side: Direction::LeftToRight,
        })
        .await;
    let skip = [".copythat-sync.db"];
    let left = {
        let root = pair.left.clone();
        let skip_vec: Vec<&'static str> = skip.to_vec();
        tokio::task::spawn_blocking(move || scan_side(&root, &skip_vec))
            .await
            .map_err(|e| SyncError::Io {
                path: pair.left.clone(),
                source: std::io::Error::other(e.to_string()),
            })??
    };
    let _ = events
        .send(SyncEvent::WalkCompleted {
            side: Direction::LeftToRight,
            files_total: left.len() as u64,
        })
        .await;

    ctrl.wait_while_paused().await?;
    let _ = events
        .send(SyncEvent::WalkStarted {
            side: Direction::RightToLeft,
        })
        .await;
    let right = {
        let root = pair.right.clone();
        let skip_vec: Vec<&'static str> = skip.to_vec();
        tokio::task::spawn_blocking(move || scan_side(&root, &skip_vec))
            .await
            .map_err(|e| SyncError::Io {
                path: pair.right.clone(),
                source: std::io::Error::other(e.to_string()),
            })??
    };
    let _ = events
        .send(SyncEvent::WalkCompleted {
            side: Direction::RightToLeft,
            files_total: right.len() as u64,
        })
        .await;

    // Collect the universe of relpaths seen on either side + any
    // relpath with a live baseline row.
    let mut relpaths: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
    relpaths.extend(left.keys().cloned());
    relpaths.extend(right.keys().cloned());
    for (k, _) in db.all_files()? {
        relpaths.insert(k);
    }

    // Decide every action up-front, then execute. The two-pass split
    // makes dry-run trivial (return after decide) and lets the UI
    // render the planned list before any copy fires.
    let mut actions: Vec<SyncAction> = Vec::with_capacity(relpaths.len());
    let device_self = db.device_id();
    for relpath in &relpaths {
        ctrl.wait_while_paused().await?;
        let baseline = db.get_file(relpath)?;
        let l = left.get(relpath).cloned().unwrap_or(SideState {
            relpath: relpath.clone(),
            meta: None,
        });
        let r = right.get(relpath).cloned().unwrap_or(SideState {
            relpath: relpath.clone(),
            meta: None,
        });
        let action = decide_action(relpath, &l, &r, baseline.as_ref(), mode);
        let _ = events
            .send(SyncEvent::ActionPlanned {
                action: action.clone(),
            })
            .await;
        actions.push(action);
    }

    let mut report = SyncReport::default();
    let host_label = pair
        .host_label
        .clone()
        .unwrap_or_else(|| format!("device-{}", short_device(device_self)));

    if opts.dry_run {
        report.elapsed_ms = elapsed_ms(start);
        let _ = events
            .send(SyncEvent::Finished {
                report: report.clone(),
            })
            .await;
        return Ok(report);
    }

    // Execute. Copies and deletes run serially per pair; we could
    // parallelise across relpaths but the payoff is small for the
    // typical home-user pair and the serial layout keeps the
    // conflict-file naming deterministic (timestamp + seq).
    let copy_factory = opts.copy_hook.clone().unwrap_or_else(|| {
        Arc::new(DefaultCopyHookFactory::new(&opts)) as Arc<dyn CopyHookFactory>
    });

    let mut history_seq: u64 = 0;
    for action in &actions {
        ctrl.wait_while_paused().await?;
        if ctrl.is_cancelled() {
            report.cancelled = true;
            break;
        }

        let _ = events
            .send(SyncEvent::ActionStarted {
                action: action.clone(),
            })
            .await;

        match action {
            SyncAction::Noop { relpath } => {
                // When both sides hold identical content, establish
                // or refresh the baseline so future edits are
                // detectable. Without this the second sync round
                // can't tell "nothing changed" from "never seen".
                let left_meta = left.get(relpath).and_then(|s| s.meta.clone());
                let right_meta = right.get(relpath).and_then(|s| s.meta.clone());
                let agreed_meta = match (&left_meta, &right_meta) {
                    (Some(l), Some(r)) if l.blake3 == r.blake3 => Some(l.clone()),
                    _ => None,
                };
                if let Some(meta) = agreed_meta {
                    let baseline = db.get_file(relpath)?;
                    let needs_update = match &baseline {
                        Some(b) => b.blake3 != meta.blake3 || b.size != meta.size,
                        None => true,
                    };
                    if needs_update {
                        let mut vv = baseline.map(|b| b.vv).unwrap_or_default();
                        vv.increment(device_self);
                        db.put_file(
                            relpath,
                            &FileRecord {
                                vv,
                                mtime_ms: meta.mtime_ms,
                                size: meta.size,
                                blake3: meta.blake3,
                            },
                        )?;
                    }
                } else if left_meta.is_none() && right_meta.is_none() {
                    // Both absent but baseline row existed —
                    // prune it so the state store stays compact.
                    if db.get_file(relpath)?.is_some() {
                        db.delete_file(relpath)?;
                    }
                }
            }
            SyncAction::Copy { relpath, direction } => {
                apply_copy(pair, relpath, *direction, copy_factory.as_ref()).await?;
                let side_state = match direction {
                    Direction::LeftToRight => left.get(relpath),
                    Direction::RightToLeft => right.get(relpath),
                }
                .expect("copy direction implies source present");
                let meta = side_state.meta.as_ref().expect("source has meta");
                // Bump vv on the *source* device (that's us on a
                // local-initiated sync). The DB records the merged
                // view so next round sees the propagated vv.
                let baseline = db.get_file(relpath)?;
                let mut vv = baseline.map(|b| b.vv).unwrap_or_default();
                vv.increment(device_self);
                let rec = FileRecord {
                    vv: vv.clone(),
                    mtime_ms: meta.mtime_ms,
                    size: meta.size,
                    blake3: meta.blake3,
                };
                db.put_file(relpath, &rec)?;
                match direction {
                    Direction::LeftToRight => report.applied_right += 1,
                    Direction::RightToLeft => report.applied_left += 1,
                }
                history_seq += 1;
                db.append_history(&HistoryEntry {
                    timestamp_ms: now_ms(),
                    seq: history_seq,
                    relpath: relpath.clone(),
                    kind: HistoryKind::Propagated,
                    direction: Some(*direction),
                    vv_before: None,
                    vv_after: Some(vv),
                })?;
            }
            SyncAction::Delete { relpath, direction } => {
                apply_delete(pair, relpath, *direction).await?;
                db.delete_file(relpath)?;
                match direction {
                    Direction::LeftToRight => report.deleted_right += 1,
                    Direction::RightToLeft => report.deleted_left += 1,
                }
                history_seq += 1;
                db.append_history(&HistoryEntry {
                    timestamp_ms: now_ms(),
                    seq: history_seq,
                    relpath: relpath.clone(),
                    kind: HistoryKind::Deleted,
                    direction: Some(*direction),
                    vv_before: None,
                    vv_after: None,
                })?;
            }
            SyncAction::KeepConflict {
                relpath,
                winner,
                loser,
                kind,
            } => {
                let conflict = apply_conflict(
                    pair,
                    relpath,
                    *winner,
                    *loser,
                    *kind,
                    &host_label,
                    copy_factory.as_ref(),
                )
                .await?;
                // Baseline becomes a merged VV of both sides'
                // current vectors, bumped by self so future rounds
                // see it as descending both ancestors.
                let left_meta = left.get(relpath).and_then(|s| s.meta.clone());
                let right_meta = right.get(relpath).and_then(|s| s.meta.clone());
                let baseline = db.get_file(relpath)?;
                let mut vv_left = baseline.as_ref().map(|b| b.vv.clone()).unwrap_or_default();
                if left_meta.is_some() {
                    vv_left.increment(device_self);
                }
                let mut vv_right = baseline.as_ref().map(|b| b.vv.clone()).unwrap_or_default();
                if right_meta.is_some() {
                    vv_right.increment(device_self);
                }
                let merged = resolve_concurrent(&vv_left, &vv_right, device_self);
                let winner_meta = match winner {
                    Direction::LeftToRight => left_meta.clone(),
                    Direction::RightToLeft => right_meta.clone(),
                };
                if let Some(meta) = winner_meta {
                    db.put_file(
                        relpath,
                        &FileRecord {
                            vv: merged.clone(),
                            mtime_ms: meta.mtime_ms,
                            size: meta.size,
                            blake3: meta.blake3,
                        },
                    )?;
                }
                history_seq += 1;
                db.append_history(&HistoryEntry {
                    timestamp_ms: now_ms(),
                    seq: history_seq,
                    relpath: relpath.clone(),
                    kind: match kind {
                        ConflictKind::CorruptEqual => HistoryKind::Corrupt,
                        _ => HistoryKind::Conflict,
                    },
                    direction: Some(*winner),
                    vv_before: None,
                    vv_after: Some(merged),
                })?;
                report.conflicts.push(conflict.clone());
                let _ = events.send(SyncEvent::Conflict(conflict)).await;
            }
        }

        let _ = events
            .send(SyncEvent::ActionCompleted {
                action: action.clone(),
            })
            .await;
    }

    report.elapsed_ms = elapsed_ms(start);
    let _ = events
        .send(SyncEvent::Finished {
            report: report.clone(),
        })
        .await;
    Ok(report)
}

/// The conflict matrix, pure function for testability.
///
/// `baseline` is the last-seen-synced row; `left` + `right` are the
/// current walk observations (with `meta = None` meaning absent).
/// `mode` controls whether a one-way mirror should consider deletes.
pub fn decide_action(
    relpath: &str,
    left: &SideState,
    right: &SideState,
    baseline: Option<&FileRecord>,
    mode: SyncMode,
) -> SyncAction {
    let lm = left.meta.as_ref();
    let rm = right.meta.as_ref();

    // Both absent → Noop (+ the caller can prune the baseline).
    if lm.is_none() && rm.is_none() {
        return SyncAction::Noop {
            relpath: relpath.to_string(),
        };
    }

    // Both present with identical content → Noop (advance baseline).
    if let (Some(l), Some(r)) = (lm, rm) {
        if l.blake3 == r.blake3 {
            return SyncAction::Noop {
                relpath: relpath.to_string(),
            };
        }
    }

    let l_matches_baseline = matches_baseline(lm, baseline);
    let r_matches_baseline = matches_baseline(rm, baseline);

    match (lm.is_some(), rm.is_some(), baseline.is_some()) {
        // New file on left only.
        (true, false, false) => direction_allowed(mode, Direction::LeftToRight)
            .map(|_| SyncAction::Copy {
                relpath: relpath.to_string(),
                direction: Direction::LeftToRight,
            })
            .unwrap_or_else(|| SyncAction::Noop {
                relpath: relpath.to_string(),
            }),
        // New file on right only.
        (false, true, false) => direction_allowed(mode, Direction::RightToLeft)
            .map(|_| SyncAction::Copy {
                relpath: relpath.to_string(),
                direction: Direction::RightToLeft,
            })
            .unwrap_or_else(|| SyncAction::Noop {
                relpath: relpath.to_string(),
            }),
        // Both added at the same relpath with different content — add/add conflict.
        (true, true, false) => {
            let (winner, loser) = pick_winner_by_mtime(lm.unwrap(), rm.unwrap());
            SyncAction::KeepConflict {
                relpath: relpath.to_string(),
                winner,
                loser,
                kind: ConflictKind::AddAdd,
            }
        }
        // Baseline existed:
        (true, true, true) => {
            let l = lm.unwrap();
            let r = rm.unwrap();
            match (l_matches_baseline, r_matches_baseline) {
                (true, true) => SyncAction::Noop {
                    relpath: relpath.to_string(),
                },
                (true, false) => {
                    // Right changed, propagate R→L.
                    if matches!(mode, SyncMode::ContributeLeftToRight) {
                        // In contribute mode we never apply remote-side changes.
                        SyncAction::Noop {
                            relpath: relpath.to_string(),
                        }
                    } else {
                        SyncAction::Copy {
                            relpath: relpath.to_string(),
                            direction: Direction::RightToLeft,
                        }
                    }
                }
                (false, true) => SyncAction::Copy {
                    relpath: relpath.to_string(),
                    direction: Direction::LeftToRight,
                },
                (false, false) => {
                    // Both changed. If the new contents agree, no-op
                    // and advance baseline; if they disagree, conflict.
                    if l.blake3 == r.blake3 {
                        SyncAction::Noop {
                            relpath: relpath.to_string(),
                        }
                    } else {
                        let (winner, loser) = pick_winner_by_mtime(l, r);
                        SyncAction::KeepConflict {
                            relpath: relpath.to_string(),
                            winner,
                            loser,
                            kind: ConflictKind::ConcurrentWrite,
                        }
                    }
                }
            }
        }
        // Left deleted (absent, had baseline), right still present.
        (false, true, true) => {
            if r_matches_baseline {
                // Right didn't change; left deleted → propagate delete R→L? no,
                // left is the one that deleted. Delete on right.
                if matches!(mode, SyncMode::ContributeLeftToRight) {
                    SyncAction::Noop {
                        relpath: relpath.to_string(),
                    }
                } else {
                    SyncAction::Delete {
                        relpath: relpath.to_string(),
                        direction: Direction::LeftToRight,
                    }
                }
            } else {
                // Right edited after last sync, left deleted → delete-edit conflict.
                SyncAction::KeepConflict {
                    relpath: relpath.to_string(),
                    winner: Direction::RightToLeft,
                    loser: Direction::LeftToRight,
                    kind: ConflictKind::DeleteEdit,
                }
            }
        }
        // Left present, right deleted.
        (true, false, true) => {
            if l_matches_baseline {
                if matches!(mode, SyncMode::ContributeLeftToRight) {
                    // Contribute never deletes; right-side absence is
                    // reinterpreted as "need to re-copy from left".
                    SyncAction::Copy {
                        relpath: relpath.to_string(),
                        direction: Direction::LeftToRight,
                    }
                } else {
                    SyncAction::Delete {
                        relpath: relpath.to_string(),
                        direction: Direction::RightToLeft,
                    }
                }
            } else {
                SyncAction::KeepConflict {
                    relpath: relpath.to_string(),
                    winner: Direction::LeftToRight,
                    loser: Direction::RightToLeft,
                    kind: ConflictKind::DeleteEdit,
                }
            }
        }
        // Both absent but baseline had the relpath → prune.
        (false, false, true) => SyncAction::Noop {
            relpath: relpath.to_string(),
        },
        // Unreachable: the both-absent-no-baseline case is handled by
        // the early return at the top of this function. Kept explicit
        // so the match stays total.
        (false, false, false) => SyncAction::Noop {
            relpath: relpath.to_string(),
        },
    }
}

fn matches_baseline(current: Option<&FileMeta>, baseline: Option<&FileRecord>) -> bool {
    match (current, baseline) {
        (Some(c), Some(b)) => c.blake3 == b.blake3 && c.size == b.size,
        _ => false,
    }
}

fn direction_allowed(mode: SyncMode, direction: Direction) -> Option<()> {
    match (mode, direction) {
        (SyncMode::TwoWay, _) => Some(()),
        (SyncMode::MirrorLeftToRight, Direction::LeftToRight) => Some(()),
        (SyncMode::MirrorRightToLeft, Direction::RightToLeft) => Some(()),
        (SyncMode::ContributeLeftToRight, Direction::LeftToRight) => Some(()),
        _ => None,
    }
}

fn pick_winner_by_mtime(left: &FileMeta, right: &FileMeta) -> (Direction, Direction) {
    // Later mtime wins. Ties → Left for determinism.
    if right.mtime_ms > left.mtime_ms {
        (Direction::RightToLeft, Direction::LeftToRight)
    } else {
        (Direction::LeftToRight, Direction::RightToLeft)
    }
}

async fn apply_copy(
    pair: &SyncPair,
    relpath: &str,
    direction: Direction,
    hook: &dyn CopyHookFactory,
) -> Result<()> {
    let (src_root, dst_root) = match direction {
        Direction::LeftToRight => (&pair.left, &pair.right),
        Direction::RightToLeft => (&pair.right, &pair.left),
    };
    let src = join_relpath(src_root, relpath);
    let dst = join_relpath(dst_root, relpath);

    if let Some(parent) = dst.parent() {
        std::fs::create_dir_all(parent).map_err(|e| SyncError::Io {
            path: parent.to_path_buf(),
            source: e,
        })?;
    }

    run_copy(&src, &dst, hook, relpath).await
}

async fn apply_delete(pair: &SyncPair, relpath: &str, direction: Direction) -> Result<()> {
    let dst_root = match direction {
        Direction::LeftToRight => &pair.right,
        Direction::RightToLeft => &pair.left,
    };
    let dst = join_relpath(dst_root, relpath);
    if dst.exists() {
        std::fs::remove_file(&dst).map_err(|e| SyncError::Io {
            path: dst.clone(),
            source: e,
        })?;
    }
    Ok(())
}

async fn apply_conflict(
    pair: &SyncPair,
    relpath: &str,
    winner: Direction,
    loser: Direction,
    kind: ConflictKind,
    host_label: &str,
    hook: &dyn CopyHookFactory,
) -> Result<Conflict> {
    // Step 1: rename the loser's live file to
    // `name.sync-conflict-YYYYMMDD-HHMMSS-<host>.ext` so the local
    // copy is preserved for the user.
    let loser_root = match loser {
        Direction::LeftToRight => &pair.left,
        Direction::RightToLeft => &pair.right,
    };
    let winner_root = match winner {
        Direction::LeftToRight => &pair.left,
        Direction::RightToLeft => &pair.right,
    };
    let loser_path = join_relpath(loser_root, relpath);
    let winner_path = join_relpath(winner_root, relpath);

    let preservation_name = conflict_preservation_name(relpath, host_label);
    let preservation_path = loser_path
        .parent()
        .unwrap_or(loser_root.as_path())
        .join(&preservation_name);

    // Delete-edit conflict where loser is the deleter: the loser
    // has nothing to preserve. For the other kinds, rename the
    // loser's file to the preservation path.
    let loser_has_content = matches!(
        kind,
        ConflictKind::ConcurrentWrite | ConflictKind::AddAdd | ConflictKind::CorruptEqual
    ) || (kind == ConflictKind::DeleteEdit && loser_path.exists());

    if loser_has_content && loser_path.exists() {
        // Ensure parent exists (it should — we're renaming in place).
        if let Some(parent) = preservation_path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| SyncError::Io {
                path: parent.to_path_buf(),
                source: e,
            })?;
        }
        std::fs::rename(&loser_path, &preservation_path).map_err(|e| SyncError::Io {
            path: loser_path.clone(),
            source: e,
        })?;
    }

    // Step 2: if the winner has content, propagate it to the loser's
    // canonical path. For a delete-edit where the editor won, this
    // copies the winner's edit to the deleter's side. For a delete-edit
    // where the deleter won (rare under mtime wins but possible under
    // ties), the loser's content was already renamed; nothing to copy.
    if winner_path.exists() {
        if let Some(parent) = loser_path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| SyncError::Io {
                path: parent.to_path_buf(),
                source: e,
            })?;
        }
        run_copy(&winner_path, &loser_path, hook, relpath).await?;
    }

    Ok(Conflict {
        relpath: relpath.to_string(),
        kind,
        winner,
        loser,
        loser_preservation_path: preservation_path,
    })
}

async fn run_copy(src: &Path, dst: &Path, hook: &dyn CopyHookFactory, relpath: &str) -> Result<()> {
    let opts = hook.build();
    let ctrl = CopyControl::new();
    let (tx, mut rx) = mpsc::channel::<CopyEvent>(64);
    let drainer = tokio::spawn(async move { while rx.recv().await.is_some() {} });

    let src_owned = src.to_path_buf();
    let dst_owned = dst.to_path_buf();
    let res = copy_file(&src_owned, &dst_owned, opts, ctrl, tx).await;
    // Drop the sender side by dropping tx's owner (happens when
    // copy_file returns). The drainer task will exit once rx sees
    // the closed channel.
    let _ = drainer.await;
    match res {
        Ok(_) => Ok(()),
        Err(e) => Err(SyncError::Propagation {
            relpath: relpath.to_string(),
            inner: Box::new(e),
        }),
    }
}

fn conflict_preservation_name(relpath: &str, host_label: &str) -> String {
    let (stem, ext) = split_last_dot(relpath);
    let ts = Utc
        .timestamp_millis_opt(now_ms() as i64)
        .single()
        .unwrap_or_else(Utc::now)
        .format("%Y%m%d-%H%M%S")
        .to_string();
    // The relpath may be `sub/dir/file.ext` — we only want the final
    // component's stem/ext, since we join against the loser's parent
    // dir. `split_last_dot` already operates on the whole relpath;
    // strip the leading directory segments here.
    let last_slash = stem.rfind('/').map(|i| i + 1).unwrap_or(0);
    let file_stem = &stem[last_slash..];
    if let Some(ext) = ext {
        format!("{file_stem}.sync-conflict-{ts}-{host_label}.{ext}")
    } else {
        format!("{file_stem}.sync-conflict-{ts}-{host_label}")
    }
}

fn split_last_dot(s: &str) -> (&str, Option<&str>) {
    // Split at the LAST dot that is inside the final path segment.
    // `foo.bar/baz` → ("foo.bar/baz", None) because the only dot is
    // before a separator. `foo.tar.gz` → ("foo.tar", Some("gz")).
    let last_slash = s.rfind('/').map(|i| i + 1).unwrap_or(0);
    let last_segment = &s[last_slash..];
    match last_segment.rfind('.') {
        Some(i) if i > 0 && i < last_segment.len() - 1 => {
            let abs_i = last_slash + i;
            (&s[..abs_i], Some(&s[abs_i + 1..]))
        }
        _ => (s, None),
    }
}

fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

fn elapsed_ms(start: SystemTime) -> u64 {
    SystemTime::now()
        .duration_since(start)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

fn short_device(id: uuid::Uuid) -> String {
    id.to_string().chars().take(8).collect()
}

/// Default copy-hook factory that builds a minimal [`CopyOptions`]
/// honouring the sync-level [`SyncOptions`]. The Tauri runner
/// overrides this with a factory that attaches platform hooks.
#[derive(Debug)]
struct DefaultCopyHookFactory {
    buffer_size: usize,
    fsync_on_close: bool,
}

impl DefaultCopyHookFactory {
    fn new(opts: &SyncOptions) -> Self {
        Self {
            buffer_size: opts.copy_buffer_size,
            fsync_on_close: opts.fsync_on_close,
        }
    }
}

impl CopyHookFactory for DefaultCopyHookFactory {
    fn build(&self) -> CopyOptions {
        CopyOptions {
            buffer_size: self.buffer_size,
            fsync_on_close: self.fsync_on_close,
            ..CopyOptions::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::clock::VersionVector;

    fn meta(blake3_byte: u8, mtime_ms: i64) -> FileMeta {
        FileMeta {
            mtime_ms,
            size: 4,
            blake3: [blake3_byte; 32],
        }
    }

    fn side_state(relpath: &str, meta: Option<FileMeta>) -> SideState {
        SideState {
            relpath: relpath.to_string(),
            meta,
        }
    }

    fn baseline(blake3_byte: u8) -> FileRecord {
        FileRecord {
            vv: VersionVector::new(),
            mtime_ms: 0,
            size: 4,
            blake3: [blake3_byte; 32],
        }
    }

    #[test]
    fn both_absent_is_noop() {
        let a = side_state("a", None);
        let b = side_state("a", None);
        let action = decide_action("a", &a, &b, None, SyncMode::TwoWay);
        assert!(matches!(action, SyncAction::Noop { .. }));
    }

    #[test]
    fn both_identical_is_noop() {
        let a = side_state("a", Some(meta(1, 10)));
        let b = side_state("a", Some(meta(1, 20)));
        let action = decide_action("a", &a, &b, None, SyncMode::TwoWay);
        assert!(matches!(action, SyncAction::Noop { .. }));
    }

    #[test]
    fn new_file_on_left_copies_right() {
        let a = side_state("a", Some(meta(1, 10)));
        let b = side_state("a", None);
        let action = decide_action("a", &a, &b, None, SyncMode::TwoWay);
        assert_eq!(
            action,
            SyncAction::Copy {
                relpath: "a".into(),
                direction: Direction::LeftToRight,
            }
        );
    }

    #[test]
    fn new_file_on_right_copies_left() {
        let a = side_state("a", None);
        let b = side_state("a", Some(meta(1, 10)));
        let action = decide_action("a", &a, &b, None, SyncMode::TwoWay);
        assert_eq!(
            action,
            SyncAction::Copy {
                relpath: "a".into(),
                direction: Direction::RightToLeft,
            }
        );
    }

    #[test]
    fn left_unchanged_right_edited_propagates_right_to_left() {
        let a = side_state("a", Some(meta(1, 10)));
        let b = side_state("a", Some(meta(2, 20)));
        let action = decide_action("a", &a, &b, Some(&baseline(1)), SyncMode::TwoWay);
        assert_eq!(
            action,
            SyncAction::Copy {
                relpath: "a".into(),
                direction: Direction::RightToLeft,
            }
        );
    }

    #[test]
    fn left_edited_right_unchanged_propagates_left_to_right() {
        let a = side_state("a", Some(meta(3, 20)));
        let b = side_state("a", Some(meta(1, 10)));
        let action = decide_action("a", &a, &b, Some(&baseline(1)), SyncMode::TwoWay);
        assert_eq!(
            action,
            SyncAction::Copy {
                relpath: "a".into(),
                direction: Direction::LeftToRight,
            }
        );
    }

    #[test]
    fn both_edited_differently_is_conflict() {
        let a = side_state("a", Some(meta(5, 10)));
        let b = side_state("a", Some(meta(7, 20)));
        let action = decide_action("a", &a, &b, Some(&baseline(1)), SyncMode::TwoWay);
        match action {
            SyncAction::KeepConflict { kind, winner, .. } => {
                assert_eq!(kind, ConflictKind::ConcurrentWrite);
                // Right has later mtime, so right wins.
                assert_eq!(winner, Direction::RightToLeft);
            }
            other => panic!("expected KeepConflict, got {other:?}"),
        }
    }

    #[test]
    fn left_deleted_right_unchanged_propagates_delete() {
        let a = side_state("a", None);
        let b = side_state("a", Some(meta(1, 10)));
        let action = decide_action("a", &a, &b, Some(&baseline(1)), SyncMode::TwoWay);
        assert_eq!(
            action,
            SyncAction::Delete {
                relpath: "a".into(),
                direction: Direction::LeftToRight,
            }
        );
    }

    #[test]
    fn delete_edit_is_conflict() {
        let a = side_state("a", None);
        let b = side_state("a", Some(meta(9, 99)));
        let action = decide_action("a", &a, &b, Some(&baseline(1)), SyncMode::TwoWay);
        match action {
            SyncAction::KeepConflict { kind, winner, .. } => {
                assert_eq!(kind, ConflictKind::DeleteEdit);
                assert_eq!(winner, Direction::RightToLeft);
            }
            other => panic!("expected KeepConflict, got {other:?}"),
        }
    }

    #[test]
    fn add_add_different_content_is_conflict() {
        let a = side_state("a", Some(meta(1, 10)));
        let b = side_state("a", Some(meta(2, 20)));
        let action = decide_action("a", &a, &b, None, SyncMode::TwoWay);
        match action {
            SyncAction::KeepConflict { kind, .. } => {
                assert_eq!(kind, ConflictKind::AddAdd);
            }
            other => panic!("expected KeepConflict, got {other:?}"),
        }
    }

    #[test]
    fn contribute_mode_never_deletes_left() {
        let a = side_state("a", Some(meta(1, 10)));
        let b = side_state("a", None);
        // Baseline matches left → left unchanged, right deleted.
        let action = decide_action(
            "a",
            &a,
            &b,
            Some(&baseline(1)),
            SyncMode::ContributeLeftToRight,
        );
        // Contribute reinterprets as "re-copy left to right".
        assert_eq!(
            action,
            SyncAction::Copy {
                relpath: "a".into(),
                direction: Direction::LeftToRight,
            }
        );
    }

    #[test]
    fn contribute_mode_ignores_right_edits() {
        let a = side_state("a", Some(meta(1, 10)));
        let b = side_state("a", Some(meta(5, 50)));
        let action = decide_action(
            "a",
            &a,
            &b,
            Some(&baseline(1)),
            SyncMode::ContributeLeftToRight,
        );
        assert!(matches!(action, SyncAction::Noop { .. }));
    }

    #[test]
    fn split_last_dot_keeps_multi_dot_stem_intact() {
        let (stem, ext) = split_last_dot("foo.tar.gz");
        assert_eq!(stem, "foo.tar");
        assert_eq!(ext, Some("gz"));
    }

    #[test]
    fn split_last_dot_returns_none_for_no_ext() {
        let (stem, ext) = split_last_dot("README");
        assert_eq!(stem, "README");
        assert_eq!(ext, None);
    }

    #[test]
    fn split_last_dot_ignores_dot_before_slash() {
        let (stem, ext) = split_last_dot("foo.bar/baz");
        assert_eq!(stem, "foo.bar/baz");
        assert_eq!(ext, None);
    }

    #[test]
    fn split_last_dot_handles_subdirs() {
        let (stem, ext) = split_last_dot("sub/dir/file.ext");
        assert_eq!(stem, "sub/dir/file");
        assert_eq!(ext, Some("ext"));
    }

    #[test]
    fn conflict_name_has_timestamp_host_and_ext() {
        let name = conflict_preservation_name("sub/file.txt", "A");
        assert!(name.starts_with("file.sync-conflict-"));
        assert!(name.ends_with("-A.txt"));
    }

    #[test]
    fn conflict_name_without_ext() {
        let name = conflict_preservation_name("README", "A");
        assert!(name.starts_with("README.sync-conflict-"));
        assert!(name.ends_with("-A"));
        // No trailing `.<ext>` — the loser's final segment had no
        // `.ext` portion to preserve.
        assert!(!name.rsplit_once('-').unwrap().1.contains('.'));
    }

    #[test]
    fn pick_winner_later_mtime() {
        let a = meta(1, 100);
        let b = meta(2, 200);
        let (w, l) = pick_winner_by_mtime(&a, &b);
        assert_eq!(w, Direction::RightToLeft);
        assert_eq!(l, Direction::LeftToRight);
    }

    #[test]
    fn pick_winner_ties_go_to_left() {
        let a = meta(1, 100);
        let b = meta(2, 100);
        let (w, _) = pick_winner_by_mtime(&a, &b);
        assert_eq!(w, Direction::LeftToRight);
    }
}
