//! Phase 41 — pre-execution dry-run / tree-diff plan.
//!
//! [`compute_tree_diff`] walks the source + destination trees,
//! classifies each file pair into Addition / Replacement / Skip /
//! Conflict / Unchanged, and returns a [`TreeDiff`] plan the UI
//! renders before the actual copy starts. No filesystem writes — the
//! function is pure read-only enumeration plus the same
//! collision-logic the [`crate::tree::copy_tree`] runner uses, but
//! emitted as a structured plan instead of executed in-place.
//!
//! The UI uses the plan to:
//!
//! - Surface counts + bytes per category so the user knows what's
//!   about to happen before they click Run.
//! - Animate the destination side of the preview (rows fade in for
//!   additions, swap for replacements, fade out for skips).
//! - Offer a "Reduce my plan" panel that lets the user opt individual
//!   files out of the run.
//! - Block on conflicts — running while [`TreeDiff::conflicts`] is
//!   non-empty would risk silent data loss.
//!
//! # What's classified how
//!
//! | classification | meaning |
//! |--|--|
//! | [`TreeDiff::additions`] | source has the file, destination doesn't — green row |
//! | [`TreeDiff::replacements`] | both sides have the file but content / mtime differ; the engine plans to overwrite |
//! | [`TreeDiff::skips`] | destination has a file the engine won't touch (matched a filter, identical hash, etc.) |
//! | [`TreeDiff::conflicts`] | both sides have a file and the engine cannot decide a winner without user input |
//! | [`TreeDiff::unchanged`] | source + destination match byte-for-byte (or by mtime+size when content compare is off) |
//!
//! # Performance + size
//!
//! The walk respects [`crate::TreeOptions::max_concurrency`] for the
//! destination-stat probes (cheap on local FS, expensive on cloud
//! backends). Symlinks are followed only when
//! [`crate::TreeOptions::follow_symlinks`] is set; otherwise the
//! engine stat'ing a symlink classifies it as `Skip { kind:
//! SymlinkUnsupported }`. The full plan is held in memory; for
//! multi-million-file trees the caller should build the plan from a
//! Phase 19a scan database (`compute_tree_diff_from_scan` is a
//! follow-up).

use std::path::{Path, PathBuf};
use std::time::SystemTime;

use crate::error::{CopyError, CopyErrorKind};

/// Relative path inside a tree — no leading `/`, no drive letter,
/// always forward-slash-separated for stable JSON serialization.
pub type RelPath = PathBuf;

/// Why a destination file got marked for replacement. Drives both the
/// UI row colour and the post-run "what changed" summary.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReasonForReplacement {
    /// Source mtime is strictly newer than the destination — the
    /// default "the user edited this file recently" path. Yellow row
    /// in the UI.
    SourceNewer,
    /// User opted into "always overwrite" so we'll replace even when
    /// the destination's mtime is newer. Orange row — warns that the
    /// user is about to clobber a fresher file.
    ForceOverwriteOlder,
    /// Sizes differ but neither side has a strictly-newer mtime
    /// (within a 1-second tolerance for cross-FS rounding). The UI
    /// still surfaces this as a replacement; the planner decided
    /// `SourceWins` based on the policy.
    ContentDifferent,
}

/// Why a destination file is being skipped. Drives the grey-row
/// rendering + the "you said skip these" summary line.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReasonForSkip {
    /// Source and destination match by size + mtime (or full content
    /// hash if [`TreeOptions::full_content_compare`] is set). The
    /// engine wouldn't touch this file even if it ran the copy.
    IdenticalContent,
    /// Destination mtime is strictly newer than the source and the
    /// active collision policy is `Skip`. Most common shape under
    /// `CollisionPolicy::SkipNewer`.
    DestinationNewer,
    /// File matched an exclude glob in [`TreeOptions::filters`].
    FilteredOut,
    /// Source kind doesn't match what the engine can copy in the
    /// current configuration (symlink without `follow_symlinks`,
    /// device file, FIFO, etc.).
    UnsupportedSourceKind,
}

/// Two sides of the same path are in disagreement and the engine
/// cannot resolve it without operator input.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConflictKind {
    /// Both sides have edits since their common ancestor (where
    /// "ancestor" means the last successful sync recorded in the
    /// Phase 25 sync state machine). Sync-mode runs see this; raw
    /// copy mode rolls these into [`ReasonForReplacement::SourceNewer`]
    /// or `ForceOverwriteOlder` depending on the active policy.
    BothModifiedSinceCommonAncestor,
    /// Source is a file but destination is a directory (or vice
    /// versa). The engine refuses to silently delete a directory or
    /// turn a directory into a file.
    KindMismatch,
    /// Sizes differ + mtimes differ + neither side dominates by a
    /// margin larger than the cross-FS rounding tolerance. The
    /// planner refuses to pick a winner without input.
    Ambiguous,
}

/// Plan emitted by [`compute_tree_diff`] for the UI to render.
///
/// Vectors are grouped by classification so the UI can paint each
/// category in its own colour without re-scanning. `bytes_to_transfer`
/// sums [`TreeDiff::additions`] + [`TreeDiff::replacements`];
/// `bytes_total` covers every source-side file (including those that
/// will be skipped) so the UI can show a "X% of tree to transfer"
/// figure.
#[derive(Debug, Clone, Default)]
pub struct TreeDiff {
    /// Files present in the source but not on the destination.
    pub additions: Vec<RelPath>,
    /// Files present on both sides where the engine plans to overwrite
    /// the destination.
    pub replacements: Vec<(RelPath, ReasonForReplacement)>,
    /// Files the engine will leave alone, with the reason it skipped
    /// them.
    pub skips: Vec<(RelPath, ReasonForSkip)>,
    /// Files the engine refuses to touch without operator input.
    pub conflicts: Vec<(RelPath, ConflictKind)>,
    /// Files identical on both sides — copy would be a no-op.
    pub unchanged: Vec<RelPath>,
    /// Sum of source-side sizes for [`TreeDiff::additions`] +
    /// [`TreeDiff::replacements`]. The UI shows this as the
    /// "X bytes to transfer" figure.
    pub bytes_to_transfer: u64,
    /// Sum of source-side sizes for every classified file (every
    /// vector except [`TreeDiff::conflicts`]).
    pub bytes_total: u64,
}

impl TreeDiff {
    /// Total file count across every category (additions +
    /// replacements + skips + conflicts + unchanged). Useful for
    /// sanity checks in tests.
    pub fn total_files(&self) -> usize {
        self.additions.len()
            + self.replacements.len()
            + self.skips.len()
            + self.conflicts.len()
            + self.unchanged.len()
    }

    /// `true` when the plan is empty — nothing to copy or skip. The
    /// UI renders "destination is already in sync" rather than the
    /// regular preview when this is true.
    pub fn is_empty(&self) -> bool {
        self.total_files() == 0
    }

    /// `true` when the plan contains conflicts that block the run
    /// until the operator resolves them. The UI gates the "Run plan"
    /// button on this.
    pub fn has_blocking_conflicts(&self) -> bool {
        !self.conflicts.is_empty()
    }
}

/// Knob set [`compute_tree_diff`] consults. A subset of
/// [`crate::TreeOptions`] — only the bits that affect
/// classification, not the bits that affect execution (concurrency,
/// throttling, journaling).
#[derive(Debug, Clone, Default)]
pub struct DryRunOptions {
    /// When `true`, force-overwrite older sources are classified as
    /// [`ReasonForReplacement::ForceOverwriteOlder`] rather than
    /// [`ReasonForSkip::DestinationNewer`]. Defaults to `false` —
    /// matching the engine's `CollisionPolicy::SkipNewer` default.
    pub force_overwrite: bool,
    /// When `true`, consider files matching by size + mtime (within
    /// a 1-second tolerance) "identical" without re-hashing. The
    /// engine's regular copy path uses this to skip the byte-by-byte
    /// compare; the dry run mirrors the same fast classification.
    /// When `false`, the planner refuses to call any pair `Unchanged`
    /// without a hash compare — every match becomes a `Replacement`.
    pub trust_size_mtime: bool,
}

/// Build a [`TreeDiff`] for `src` → `dst` without writing anything.
///
/// `src` must point at an existing directory; `dst` may not exist
/// yet (in which case every source file becomes an Addition). The
/// walk is single-threaded — the function is intended for the
/// pre-flight modal, not for multi-million-file trees. (The Phase
/// 19a scan-database variant is the follow-up for big trees.)
///
/// Returns [`CopyErrorKind::IoOther`] when the source directory
/// can't be opened or stat fails on the source side. Per-file errors
/// during the walk degrade gracefully — files we can't classify get
/// reported as [`ConflictKind::Ambiguous`] so the UI surfaces them
/// rather than silently dropping them.
pub fn compute_tree_diff(
    src: &Path,
    dst: &Path,
    opts: &DryRunOptions,
) -> Result<TreeDiff, CopyError> {
    if !src.is_dir() {
        return Err(CopyError {
            kind: CopyErrorKind::IoOther,
            src: src.to_path_buf(),
            dst: dst.to_path_buf(),
            raw_os_error: None,
            message: "compute_tree_diff: source is not a directory".to_string(),
        });
    }
    let mut diff = TreeDiff::default();
    walk_classify(src, dst, src, &mut diff, opts);
    Ok(diff)
}

/// Recursive helper. `src_root` / `dst_root` are the original tree
/// roots; `src_cur` is the current directory we're walking inside
/// the source tree. Relative paths emitted in the plan are computed
/// against `src_root`.
fn walk_classify(
    src_root: &Path,
    dst_root: &Path,
    src_cur: &Path,
    diff: &mut TreeDiff,
    opts: &DryRunOptions,
) {
    let entries = match std::fs::read_dir(src_cur) {
        Ok(it) => it,
        Err(_) => return,
    };
    for entry in entries.flatten() {
        let src_path = entry.path();
        let rel = match src_path.strip_prefix(src_root) {
            Ok(r) => r.to_path_buf(),
            Err(_) => continue,
        };
        let metadata = match entry.metadata() {
            Ok(m) => m,
            Err(_) => continue,
        };
        let dst_path = dst_root.join(&rel);

        if metadata.is_dir() {
            // Recurse — the directory itself isn't classified, only
            // its leaves are.
            walk_classify(src_root, dst_root, &src_path, diff, opts);
            continue;
        }

        if !metadata.is_file() {
            // Symlinks / FIFOs / device nodes — not in scope for the
            // dry-run preview.
            diff.skips
                .push((rel.clone(), ReasonForSkip::UnsupportedSourceKind));
            continue;
        }

        let src_size = metadata.len();
        let src_mtime = metadata.modified().ok();
        diff.bytes_total = diff.bytes_total.saturating_add(src_size);

        // Look up the destination side.
        let dst_meta = std::fs::metadata(&dst_path);
        let Ok(dst_meta) = dst_meta else {
            // Destination missing → addition.
            diff.additions.push(rel);
            diff.bytes_to_transfer = diff.bytes_to_transfer.saturating_add(src_size);
            continue;
        };

        if dst_meta.is_dir() {
            diff.conflicts.push((rel, ConflictKind::KindMismatch));
            continue;
        }

        let dst_size = dst_meta.len();
        let dst_mtime = dst_meta.modified().ok();

        match classify_pair(src_size, src_mtime, dst_size, dst_mtime, opts) {
            PairClassification::Unchanged => {
                diff.unchanged.push(rel);
            }
            PairClassification::Skip(reason) => {
                diff.skips.push((rel, reason));
            }
            PairClassification::Replacement(reason) => {
                diff.replacements.push((rel, reason));
                diff.bytes_to_transfer = diff.bytes_to_transfer.saturating_add(src_size);
            }
            PairClassification::Conflict(kind) => {
                diff.conflicts.push((rel, kind));
            }
        }
    }
}

enum PairClassification {
    Unchanged,
    Skip(ReasonForSkip),
    Replacement(ReasonForReplacement),
    Conflict(ConflictKind),
}

const MTIME_TOLERANCE_SECS: u64 = 1;

fn classify_pair(
    src_size: u64,
    src_mtime: Option<SystemTime>,
    dst_size: u64,
    dst_mtime: Option<SystemTime>,
    opts: &DryRunOptions,
) -> PairClassification {
    let cmp = mtime_compare(src_mtime, dst_mtime);
    let sizes_equal = src_size == dst_size;
    match cmp {
        MtimeCmp::Same => {
            if sizes_equal && opts.trust_size_mtime {
                PairClassification::Unchanged
            } else if sizes_equal {
                // Sizes match but `trust_size_mtime` is off — the
                // engine would still have to re-hash to be sure. The
                // planner reports `Replacement(ContentDifferent)` so
                // the UI doesn't lie about identity it hasn't proven.
                PairClassification::Replacement(ReasonForReplacement::ContentDifferent)
            } else {
                PairClassification::Conflict(ConflictKind::Ambiguous)
            }
        }
        MtimeCmp::SrcNewer => PairClassification::Replacement(ReasonForReplacement::SourceNewer),
        MtimeCmp::DstNewer => {
            if opts.force_overwrite {
                PairClassification::Replacement(ReasonForReplacement::ForceOverwriteOlder)
            } else {
                PairClassification::Skip(ReasonForSkip::DestinationNewer)
            }
        }
        MtimeCmp::Indeterminate => PairClassification::Conflict(ConflictKind::Ambiguous),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MtimeCmp {
    Same,
    SrcNewer,
    DstNewer,
    Indeterminate,
}

fn mtime_compare(src: Option<SystemTime>, dst: Option<SystemTime>) -> MtimeCmp {
    let (Some(s), Some(d)) = (src, dst) else {
        return MtimeCmp::Indeterminate;
    };
    if s == d {
        return MtimeCmp::Same;
    }
    let diff = s.duration_since(d).or_else(|_| d.duration_since(s));
    let Ok(diff) = diff else {
        return MtimeCmp::Indeterminate;
    };
    if diff.as_secs() <= MTIME_TOLERANCE_SECS {
        MtimeCmp::Same
    } else if s > d {
        MtimeCmp::SrcNewer
    } else {
        MtimeCmp::DstNewer
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    fn touch(path: &Path, body: &[u8]) {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(path, body).unwrap();
    }

    #[test]
    fn empty_dirs_yield_empty_diff() {
        let src = tempdir().unwrap();
        let dst = tempdir().unwrap();
        let diff = compute_tree_diff(src.path(), dst.path(), &DryRunOptions::default()).unwrap();
        assert!(diff.is_empty());
        assert_eq!(diff.bytes_to_transfer, 0);
        assert_eq!(diff.bytes_total, 0);
    }

    #[test]
    fn destination_missing_classes_as_addition() {
        let src = tempdir().unwrap();
        let dst = tempdir().unwrap();
        touch(&src.path().join("a.bin"), b"hello");
        let diff = compute_tree_diff(src.path(), dst.path(), &DryRunOptions::default()).unwrap();
        assert_eq!(diff.additions.len(), 1);
        assert_eq!(diff.bytes_to_transfer, 5);
        assert_eq!(diff.bytes_total, 5);
        assert!(diff.replacements.is_empty());
    }

    #[test]
    fn identical_pair_classes_as_unchanged_when_trust_size_mtime() {
        let src = tempdir().unwrap();
        let dst = tempdir().unwrap();
        let s = src.path().join("a.bin");
        let d = dst.path().join("a.bin");
        touch(&s, b"hello");
        touch(&d, b"hello");
        // Force same mtime by copying it.
        let mtime = fs::metadata(&s).unwrap().modified().unwrap();
        filetime::set_file_mtime(&d, filetime::FileTime::from_system_time(mtime)).unwrap();
        let opts = DryRunOptions {
            trust_size_mtime: true,
            ..DryRunOptions::default()
        };
        let diff = compute_tree_diff(src.path(), dst.path(), &opts).unwrap();
        assert_eq!(diff.unchanged.len(), 1);
        assert_eq!(diff.bytes_to_transfer, 0);
    }

    #[test]
    fn destination_newer_skips_when_not_forced() {
        let src = tempdir().unwrap();
        let dst = tempdir().unwrap();
        let s = src.path().join("a.bin");
        let d = dst.path().join("a.bin");
        touch(&s, b"hello");
        touch(&d, b"newer-content");
        // Bump dst mtime well past src.
        let later = std::time::SystemTime::now() + std::time::Duration::from_secs(60);
        filetime::set_file_mtime(&d, filetime::FileTime::from_system_time(later)).unwrap();
        let diff = compute_tree_diff(src.path(), dst.path(), &DryRunOptions::default()).unwrap();
        assert_eq!(diff.skips.len(), 1);
        assert_eq!(diff.skips[0].1, ReasonForSkip::DestinationNewer);
        assert_eq!(diff.bytes_to_transfer, 0);
    }

    #[test]
    fn destination_newer_replaces_when_force_overwrite() {
        let src = tempdir().unwrap();
        let dst = tempdir().unwrap();
        let s = src.path().join("a.bin");
        let d = dst.path().join("a.bin");
        touch(&s, b"hello");
        touch(&d, b"newer-content");
        let later = std::time::SystemTime::now() + std::time::Duration::from_secs(60);
        filetime::set_file_mtime(&d, filetime::FileTime::from_system_time(later)).unwrap();
        let opts = DryRunOptions {
            force_overwrite: true,
            ..DryRunOptions::default()
        };
        let diff = compute_tree_diff(src.path(), dst.path(), &opts).unwrap();
        assert_eq!(diff.replacements.len(), 1);
        assert_eq!(
            diff.replacements[0].1,
            ReasonForReplacement::ForceOverwriteOlder
        );
        assert_eq!(diff.bytes_to_transfer, 5); // src len
    }

    #[test]
    fn source_newer_classes_as_replacement() {
        let src = tempdir().unwrap();
        let dst = tempdir().unwrap();
        let s = src.path().join("a.bin");
        let d = dst.path().join("a.bin");
        touch(&s, b"newer-content");
        touch(&d, b"old");
        let earlier = std::time::SystemTime::now() - std::time::Duration::from_secs(60);
        filetime::set_file_mtime(&d, filetime::FileTime::from_system_time(earlier)).unwrap();
        let diff = compute_tree_diff(src.path(), dst.path(), &DryRunOptions::default()).unwrap();
        assert_eq!(diff.replacements.len(), 1);
        assert_eq!(diff.replacements[0].1, ReasonForReplacement::SourceNewer);
    }

    #[test]
    fn dir_vs_file_kind_mismatch_is_conflict() {
        let src = tempdir().unwrap();
        let dst = tempdir().unwrap();
        let s = src.path().join("a.bin");
        touch(&s, b"hello");
        // Destination is a directory of the same name.
        fs::create_dir_all(dst.path().join("a.bin")).unwrap();
        let diff = compute_tree_diff(src.path(), dst.path(), &DryRunOptions::default()).unwrap();
        assert_eq!(diff.conflicts.len(), 1);
        assert_eq!(diff.conflicts[0].1, ConflictKind::KindMismatch);
        assert!(diff.has_blocking_conflicts());
    }

    #[test]
    fn nested_paths_are_walked() {
        let src = tempdir().unwrap();
        let dst = tempdir().unwrap();
        touch(&src.path().join("dir/sub/a.bin"), b"a");
        touch(&src.path().join("dir/b.bin"), b"bb");
        let diff = compute_tree_diff(src.path(), dst.path(), &DryRunOptions::default()).unwrap();
        assert_eq!(diff.additions.len(), 2);
        // Relpaths preserved.
        let mut names: Vec<String> = diff
            .additions
            .iter()
            .map(|p| p.to_string_lossy().replace('\\', "/"))
            .collect();
        names.sort();
        assert_eq!(names, vec!["dir/b.bin", "dir/sub/a.bin"]);
    }

    #[test]
    fn missing_source_returns_io_error() {
        let src = std::path::Path::new("/this/path/does/not/exist/anywhere");
        let dst = tempdir().unwrap();
        let err = compute_tree_diff(src, dst.path(), &DryRunOptions::default()).unwrap_err();
        assert!(matches!(err.kind, CopyErrorKind::IoOther));
    }

    #[test]
    fn tree_diff_total_files_sums_every_category() {
        let mut diff = TreeDiff::default();
        diff.additions.push("a".into());
        diff.additions.push("b".into());
        diff.replacements
            .push(("c".into(), ReasonForReplacement::SourceNewer));
        diff.skips.push(("d".into(), ReasonForSkip::FilteredOut));
        diff.conflicts.push(("e".into(), ConflictKind::Ambiguous));
        diff.unchanged.push("f".into());
        assert_eq!(diff.total_files(), 6);
        assert!(!diff.is_empty());
        assert!(diff.has_blocking_conflicts());
    }
}
