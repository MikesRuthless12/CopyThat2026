//! Phase 41 — Tauri IPC command for the pre-execution tree-diff
//! preview. Wraps [`copythat_core::dryrun::compute_tree_diff`] and
//! returns a JSON-serialisable [`TreeDiffDto`] the Svelte preview
//! modal renders.
//!
//! The dry-run is single-threaded and reads only the source/destination
//! file metadata — no chunk hashing, no IO writes. For multi-million-
//! file trees the Phase 19a scan-database variant is the follow-up
//! (`compute_tree_diff_from_scan` lands later).

use std::path::PathBuf;

use copythat_core::dryrun::{
    self, ConflictKind, DryRunOptions, ReasonForReplacement, ReasonForSkip,
};
use serde::{Deserialize, Serialize};

/// Wire shape for [`copythat_core::dryrun::TreeDiff`]. Vectors are
/// grouped per category so the Svelte panel can paint them in their
/// own colour without re-scanning.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TreeDiffDto {
    pub additions: Vec<String>,
    pub replacements: Vec<ReplacementRowDto>,
    pub skips: Vec<SkipRowDto>,
    pub conflicts: Vec<ConflictRowDto>,
    pub unchanged: Vec<String>,
    pub bytes_to_transfer: u64,
    pub bytes_total: u64,
    /// Aggregate count across every category, mirroring
    /// `TreeDiff::total_files`. Saves the frontend from summing.
    pub total_files: usize,
    /// `true` when [`TreeDiffDto::conflicts`] is non-empty. The
    /// preview modal grays the "Run plan" button on this so the
    /// operator has to either reduce the plan or resolve conflicts
    /// before the run can start.
    pub has_blocking_conflicts: bool,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplacementRowDto {
    pub rel_path: String,
    /// Stable wire string: `"source-newer"` / `"force-overwrite-older"`
    /// / `"content-different"`.
    pub reason: &'static str,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SkipRowDto {
    pub rel_path: String,
    /// Stable wire string: `"identical-content"` / `"destination-newer"`
    /// / `"filtered-out"` / `"unsupported-source-kind"`.
    pub reason: &'static str,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConflictRowDto {
    pub rel_path: String,
    /// Stable wire string: `"both-modified-since-common-ancestor"` /
    /// `"kind-mismatch"` / `"ambiguous"`.
    pub kind: &'static str,
}

/// Knob set the Svelte modal hands across the IPC boundary. Mirrors
/// [`copythat_core::dryrun::DryRunOptions`] in camelCase.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DryRunOptionsDto {
    pub force_overwrite: bool,
    pub trust_size_mtime: bool,
}

impl From<DryRunOptionsDto> for DryRunOptions {
    fn from(dto: DryRunOptionsDto) -> Self {
        DryRunOptions {
            force_overwrite: dto.force_overwrite,
            trust_size_mtime: dto.trust_size_mtime,
        }
    }
}

fn replacement_wire(r: ReasonForReplacement) -> &'static str {
    match r {
        ReasonForReplacement::SourceNewer => "source-newer",
        ReasonForReplacement::ForceOverwriteOlder => "force-overwrite-older",
        ReasonForReplacement::ContentDifferent => "content-different",
    }
}

fn skip_wire(r: ReasonForSkip) -> &'static str {
    match r {
        ReasonForSkip::IdenticalContent => "identical-content",
        ReasonForSkip::DestinationNewer => "destination-newer",
        ReasonForSkip::FilteredOut => "filtered-out",
        ReasonForSkip::UnsupportedSourceKind => "unsupported-source-kind",
    }
}

fn conflict_wire(k: ConflictKind) -> &'static str {
    match k {
        ConflictKind::BothModifiedSinceCommonAncestor => "both-modified-since-common-ancestor",
        ConflictKind::KindMismatch => "kind-mismatch",
        ConflictKind::Ambiguous => "ambiguous",
    }
}

fn rel_to_string(p: PathBuf) -> String {
    p.to_string_lossy().replace('\\', "/")
}

/// `compute_tree_diff(src, dst, opts)` — render the pre-flight plan
/// the preview modal shows. Single-threaded; small to medium trees
/// (the Settings → "Show preview before running copies" toggle gates
/// this call on a sane size threshold so multi-million-file jobs
/// don't lock the UI).
#[tauri::command]
pub async fn compute_tree_diff(
    src: String,
    dst: String,
    opts: DryRunOptionsDto,
) -> Result<TreeDiffDto, String> {
    let src_path = std::path::PathBuf::from(src);
    let dst_path = std::path::PathBuf::from(dst);
    let opts: DryRunOptions = opts.into();
    let diff =
        tokio::task::spawn_blocking(move || dryrun::compute_tree_diff(&src_path, &dst_path, &opts))
            .await
            .map_err(|e| format!("dry-run worker panicked: {e}"))?
            .map_err(|e| format!("compute_tree_diff: {e}"))?;

    let total_files = diff.total_files();
    let has_blocking_conflicts = diff.has_blocking_conflicts();

    Ok(TreeDiffDto {
        additions: diff.additions.into_iter().map(rel_to_string).collect(),
        replacements: diff
            .replacements
            .into_iter()
            .map(|(p, r)| ReplacementRowDto {
                rel_path: rel_to_string(p),
                reason: replacement_wire(r),
            })
            .collect(),
        skips: diff
            .skips
            .into_iter()
            .map(|(p, r)| SkipRowDto {
                rel_path: rel_to_string(p),
                reason: skip_wire(r),
            })
            .collect(),
        conflicts: diff
            .conflicts
            .into_iter()
            .map(|(p, k)| ConflictRowDto {
                rel_path: rel_to_string(p),
                kind: conflict_wire(k),
            })
            .collect(),
        unchanged: diff.unchanged.into_iter().map(rel_to_string).collect(),
        bytes_to_transfer: diff.bytes_to_transfer,
        bytes_total: diff.bytes_total,
        total_files,
        has_blocking_conflicts,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn replacement_wire_strings_are_stable() {
        assert_eq!(
            replacement_wire(ReasonForReplacement::SourceNewer),
            "source-newer"
        );
        assert_eq!(
            replacement_wire(ReasonForReplacement::ForceOverwriteOlder),
            "force-overwrite-older"
        );
        assert_eq!(
            replacement_wire(ReasonForReplacement::ContentDifferent),
            "content-different"
        );
    }

    #[test]
    fn skip_wire_strings_are_stable() {
        assert_eq!(
            skip_wire(ReasonForSkip::IdenticalContent),
            "identical-content"
        );
        assert_eq!(
            skip_wire(ReasonForSkip::DestinationNewer),
            "destination-newer"
        );
        assert_eq!(skip_wire(ReasonForSkip::FilteredOut), "filtered-out");
        assert_eq!(
            skip_wire(ReasonForSkip::UnsupportedSourceKind),
            "unsupported-source-kind"
        );
    }

    #[test]
    fn conflict_wire_strings_are_stable() {
        assert_eq!(conflict_wire(ConflictKind::KindMismatch), "kind-mismatch");
        assert_eq!(conflict_wire(ConflictKind::Ambiguous), "ambiguous");
        assert_eq!(
            conflict_wire(ConflictKind::BothModifiedSinceCommonAncestor),
            "both-modified-since-common-ancestor"
        );
    }

    #[tokio::test]
    async fn empty_dirs_yield_empty_dto() {
        let src = tempfile::tempdir().unwrap();
        let dst = tempfile::tempdir().unwrap();
        let dto = compute_tree_diff(
            src.path().display().to_string(),
            dst.path().display().to_string(),
            DryRunOptionsDto::default(),
        )
        .await
        .expect("ok");
        assert_eq!(dto.total_files, 0);
        assert!(!dto.has_blocking_conflicts);
        assert_eq!(dto.bytes_to_transfer, 0);
    }

    #[tokio::test]
    async fn rel_paths_use_forward_slash_in_dto() {
        let src = tempfile::tempdir().unwrap();
        let dst = tempfile::tempdir().unwrap();
        let nested = src.path().join("dir").join("nested.bin");
        fs::create_dir_all(nested.parent().unwrap()).unwrap();
        fs::write(&nested, b"hi").unwrap();
        let dto = compute_tree_diff(
            src.path().display().to_string(),
            dst.path().display().to_string(),
            DryRunOptionsDto::default(),
        )
        .await
        .expect("ok");
        assert_eq!(dto.additions.len(), 1);
        // Wire format always uses `/` separators, even on Windows.
        assert!(dto.additions[0].contains('/'));
        assert!(!dto.additions[0].contains('\\'));
    }
}
