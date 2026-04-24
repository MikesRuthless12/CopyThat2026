//! Phase 33 — virtual-filesystem tree builder.
//!
//! Given a [`copythat_history::History`] handle + a
//! [`copythat_chunk::ChunkStore`] handle, [`MountTree::build`] emits
//! the canonical layout the FUSE / WinFsp backends expose at
//! mount time:
//!
//! ```text
//! <mountpoint>/
//!   by-date/
//!     2026-04-21/
//!       14-32-15/                   # job start (UTC HH-MM-SS)
//!         <job-label>/
//!           <recovered-tree>/…
//!   by-source/
//!     C--Users-miken-Desktop/       # escaped original src_root
//!       2026-04-21--14-32-15/
//!         …
//!   by-job-id/
//!     9be3fa…/
//!       …
//! ```
//!
//! Phase 33a emits the three top-level virtual directories +
//! per-job containers. The per-item subtree below a job container
//! (the actual files the job copied) is stubbed: the tree carries
//! the `ItemRow` metadata so a Phase 33b FUSE callback can
//! materialise chunks on demand, but no chunk reads happen here.
//!
//! The builder is deterministic: identical inputs produce an
//! identical tree. Node children are sorted by name so the mounted
//! output matches a naive `ls -la`.

use std::collections::BTreeMap;
#[cfg(test)]
use std::path::PathBuf;

use copythat_chunk::ChunkStore;
use copythat_history::JobSummary;

use crate::error::MountError;

/// Top-level layout the mount exposes. Phase 33a always emits all
/// three views; Phase 33b's Settings entry will let a power user
/// disable views they don't use for a smaller directory listing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct MountLayout {
    pub by_date: bool,
    pub by_source: bool,
    pub by_job_id: bool,
}

impl MountLayout {
    /// All three views on — the default the History context menu's
    /// "Mount snapshot" action uses.
    pub const fn all() -> Self {
        Self {
            by_date: true,
            by_source: true,
            by_job_id: true,
        }
    }
}

/// One node in the virtual filesystem. Children are keyed on their
/// display name so duplicates (two jobs sharing a job-label on the
/// same UTC second) collapse via `entry().or_insert_with(...)`.
#[derive(Debug, Clone, PartialEq)]
pub struct MountNode {
    pub name: String,
    pub kind: NodeKind,
    pub children: BTreeMap<String, MountNode>,
}

impl MountNode {
    fn directory(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            kind: NodeKind::Directory,
            children: BTreeMap::new(),
        }
    }

    fn job_placeholder(name: impl Into<String>, job_row_id: i64) -> Self {
        Self {
            name: name.into(),
            kind: NodeKind::JobPlaceholder { job_row_id },
            children: BTreeMap::new(),
        }
    }

    /// Recursive child count, not including self.
    pub fn descendant_count(&self) -> usize {
        self.children
            .values()
            .map(|c| 1 + c.descendant_count())
            .sum()
    }

    /// Insert `child` at the path `segments`, creating intermediate
    /// directories as needed. Silently overwrites a same-named leaf.
    fn insert_at(&mut self, segments: &[&str], leaf: MountNode) {
        if segments.is_empty() {
            self.children.insert(leaf.name.clone(), leaf);
            return;
        }
        let (head, rest) = segments.split_first().expect("non-empty");
        let entry = self
            .children
            .entry((*head).to_string())
            .or_insert_with(|| MountNode::directory(*head));
        entry.insert_at(rest, leaf);
    }
}

/// What a node represents at mount time. FUSE / WinFsp callbacks
/// switch on this.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NodeKind {
    /// Virtual directory — listed but has no backing storage.
    Directory,
    /// Per-job placeholder — Phase 33b's callbacks expand this into
    /// the recovered-tree of `ItemRow`s by calling
    /// `History::items_for(job_row_id)` + streaming chunks from the
    /// chunk store.
    JobPlaceholder { job_row_id: i64 },
}

/// Full virtual filesystem tree backed by the history + chunk store.
#[derive(Debug, Clone)]
pub struct MountTree {
    pub root: MountNode,
    pub layout: MountLayout,
    pub job_count: usize,
}

impl MountTree {
    /// Build the tree from a pre-fetched set of [`JobSummary`] rows.
    /// Callers fetch via `History::search(...)` on the tokio side
    /// and hand the result here; keeping this builder async-agnostic
    /// means the FUSE / WinFsp backends' blocking callbacks don't
    /// need to thread a runtime through the tree layer.
    ///
    /// The chunk store is referenced for future on-demand
    /// materialisation — 33a doesn't read it, just confirms the
    /// caller holds a handle.
    pub fn build(
        jobs: &[JobSummary],
        chunk_store: &ChunkStore,
        layout: MountLayout,
    ) -> Result<Self, MountError> {
        // Keep the reference alive across the build so the returned
        // handle can later borrow it for per-file reads.
        let _chunk_store = chunk_store;

        let mut root = MountNode::directory("");

        for job in jobs {
            let (date, time) = split_started_at(job.started_at_ms);
            let job_label = job_label_from_summary(&job.dst_root, &job.kind);
            let escaped_source = escape_path(&job.src_root);
            let job_row_id = job.row_id;

            if layout.by_date {
                let placeholder = MountNode::job_placeholder(job_label.clone(), job_row_id);
                root.insert_at(&["by-date", &date, &time], placeholder);
            }

            if layout.by_source {
                let placeholder = MountNode::job_placeholder(format!("{date}--{time}"), job_row_id);
                root.insert_at(&["by-source", &escaped_source], placeholder);
            }

            if layout.by_job_id {
                let placeholder = MountNode::job_placeholder(job_label, job_row_id);
                // Job ID bucket mirrors the `jobs.row_id` integer so
                // direct lookup via `by-job-id/<N>` is O(1). Phase
                // 33b can swap this for a UUID once the history
                // schema gets one.
                root.insert_at(&["by-job-id", &job_row_id.to_string()], placeholder);
            }
        }

        Ok(Self {
            root,
            layout,
            job_count: jobs.len(),
        })
    }

    /// Convenience — locate a node by its slash-delimited virtual
    /// path (no leading `/`). Returns `None` for any missing
    /// component.
    pub fn lookup(&self, virtual_path: &str) -> Option<&MountNode> {
        let mut current = &self.root;
        if virtual_path.is_empty() {
            return Some(current);
        }
        for segment in virtual_path.split('/') {
            current = current.children.get(segment)?;
        }
        Some(current)
    }
}

/// Split a millis-since-epoch timestamp into `(YYYY-MM-DD, HH-MM-SS)`
/// in UTC. Pure-Rust implementation so the crate doesn't pull in
/// `chrono` just for this (the history DB already stores the raw
/// millis; this file renders them for the virtual-FS path segments).
fn split_started_at(started_at_ms: i64) -> (String, String) {
    let secs = started_at_ms.div_euclid(1000);
    // Breaks down seconds since 1970-01-01 into date + time of day.
    let days = secs.div_euclid(86_400);
    let sod = secs.rem_euclid(86_400);
    let hh = sod / 3_600;
    let mm = (sod % 3_600) / 60;
    let ss = sod % 60;

    // Civil-from-days algorithm (Howard Hinnant's, MIT-licensed).
    let z = days + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 }.div_euclid(146_097);
    let doe = (z - era * 146_097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = y + if m <= 2 { 1 } else { 0 };

    (
        format!("{y:04}-{m:02}-{d:02}"),
        format!("{hh:02}-{mm:02}-{ss:02}"),
    )
}

/// Turn a source root into a filesystem-safe single segment:
/// `C:\Users\miken\Desktop` → `C--Users-miken-Desktop`.
fn escape_path(src_root: &std::path::Path) -> String {
    let raw = src_root.to_string_lossy();
    let mut out = String::with_capacity(raw.len());
    let mut prev_sep = false;
    for ch in raw.chars() {
        match ch {
            ':' => {
                out.push('-');
                out.push('-');
                prev_sep = true;
            }
            '\\' | '/' => {
                if !prev_sep {
                    out.push('-');
                    prev_sep = true;
                }
            }
            _ => {
                out.push(ch);
                prev_sep = false;
            }
        }
    }
    // Trim any leading / trailing separators.
    out.trim_matches('-').to_owned()
}

/// Derive a short label for a job node from its destination root
/// plus kind. Falls back to the kind when the dst_root is empty or
/// root-like.
fn job_label_from_summary(dst_root: &std::path::Path, kind: &str) -> String {
    let name = dst_root
        .file_name()
        .and_then(|os| os.to_str())
        .map(|s| s.to_owned())
        .unwrap_or_default();
    if name.is_empty() {
        kind.to_owned()
    } else {
        format!("{name}-{kind}")
    }
}

/// Used by tests that want to construct a tree without a live
/// history DB — seeds the same branches the real builder would.
#[cfg(test)]
pub(crate) fn build_from_rows(
    rows: &[(i64, PathBuf, PathBuf, i64, String)],
    layout: MountLayout,
) -> MountTree {
    let mut root = MountNode::directory("");
    for (row_id, src_root, dst_root, started_at_ms, kind) in rows {
        let (date, time) = split_started_at(*started_at_ms);
        let job_label = job_label_from_summary(dst_root, kind);
        let escaped_source = escape_path(src_root);

        if layout.by_date {
            let placeholder = MountNode::job_placeholder(job_label.clone(), *row_id);
            root.insert_at(&["by-date", &date, &time], placeholder);
        }
        if layout.by_source {
            let placeholder = MountNode::job_placeholder(format!("{date}--{time}"), *row_id);
            root.insert_at(&["by-source", &escaped_source], placeholder);
        }
        if layout.by_job_id {
            let placeholder = MountNode::job_placeholder(job_label, *row_id);
            root.insert_at(&["by-job-id", &row_id.to_string()], placeholder);
        }
    }
    MountTree {
        root,
        layout,
        job_count: rows.len(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_started_at_known_epochs() {
        // 2026-04-21 14:32:15 UTC -> 1 776 011 535_000 ms.
        let (d, t) = split_started_at(1_776_781_935_000);
        assert_eq!(d, "2026-04-21");
        assert_eq!(t, "14-32-15");

        // Epoch itself.
        let (d0, t0) = split_started_at(0);
        assert_eq!(d0, "1970-01-01");
        assert_eq!(t0, "00-00-00");
    }

    #[test]
    fn escape_path_windows_drive() {
        let escaped = escape_path(&PathBuf::from(r"C:\Users\miken\Desktop"));
        assert_eq!(escaped, "C--Users-miken-Desktop");
    }

    #[test]
    fn escape_path_unix_root() {
        let escaped = escape_path(&PathBuf::from("/home/miken/docs"));
        assert_eq!(escaped, "home-miken-docs");
    }

    #[test]
    fn job_label_prefers_basename_with_kind_suffix() {
        assert_eq!(
            job_label_from_summary(&PathBuf::from("/tmp/archive"), "copy"),
            "archive-copy",
        );
    }

    #[test]
    fn build_from_rows_emits_three_views() {
        let rows = vec![(
            1,
            PathBuf::from(r"C:\src"),
            PathBuf::from(r"C:\dst\archive"),
            1_776_781_935_000_i64,
            "copy".to_owned(),
        )];
        let tree = build_from_rows(&rows, MountLayout::all());
        assert_eq!(tree.job_count, 1);
        assert!(
            tree.lookup("by-date/2026-04-21/14-32-15/archive-copy")
                .is_some()
        );
        assert!(tree.lookup("by-source/C--src").is_some());
        assert!(tree.lookup("by-job-id/1/archive-copy").is_some());
    }

    #[test]
    fn lookup_root_returns_root() {
        let tree = build_from_rows(&[], MountLayout::all());
        let found = tree.lookup("").expect("root present");
        assert_eq!(found.kind, NodeKind::Directory);
    }

    #[test]
    fn lookup_missing_returns_none() {
        let tree = build_from_rows(&[], MountLayout::all());
        assert!(tree.lookup("by-date/9999-99-99").is_none());
    }

    #[test]
    fn layout_partial_suppresses_branches() {
        let rows = vec![(
            1,
            PathBuf::from(r"C:\src"),
            PathBuf::from(r"C:\dst\archive"),
            1_776_781_935_000_i64,
            "copy".to_owned(),
        )];
        let only_by_date = MountLayout {
            by_date: true,
            by_source: false,
            by_job_id: false,
        };
        let tree = build_from_rows(&rows, only_by_date);
        assert!(tree.lookup("by-date").is_some());
        assert!(tree.lookup("by-source").is_none());
        assert!(tree.lookup("by-job-id").is_none());
    }

    #[test]
    fn descendant_count_walks_full_tree() {
        let rows = vec![
            (
                1,
                PathBuf::from("/a"),
                PathBuf::from("/dst/x"),
                1_776_781_935_000,
                "copy".into(),
            ),
            (
                2,
                PathBuf::from("/b"),
                PathBuf::from("/dst/y"),
                1_776_781_936_000,
                "copy".into(),
            ),
        ];
        let tree = build_from_rows(&rows, MountLayout::all());
        // by-date + 1 date dir + 2 time dirs + 2 job leaves
        // + by-source + 2 source dirs + 2 timestamp leaves
        // + by-job-id + 2 id dirs + 2 job leaves
        // = 16
        assert_eq!(tree.root.descendant_count(), 16);
    }
}
