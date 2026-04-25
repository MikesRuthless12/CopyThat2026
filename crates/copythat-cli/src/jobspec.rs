//! Declarative TOML jobspec — Phase 36.
//!
//! Specs describe **what** the user wants to happen (copy a tree;
//! sync two roots; verify a file). `plan` reads the spec and reports
//! the actions a matching `apply` would take, exit 2 means there are
//! pending actions, exit 0 means already-done. `apply` runs the same
//! plan; idempotent re-runs exit 0.

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

/// Top-level jobspec. Mirrors the layout in
/// `CopyThat2026-Build-Prompts-Guide2.md` Phase 36.
///
/// Example:
/// ```toml
/// [job]
/// kind = "copy"
/// source = ["C:/data/q1"]
/// destination = "D:/backups/q1"
/// verify = "blake3"
/// shape = "10MB/s"
///
/// [retry]
/// attempts = 3
/// backoff = "30s"
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobSpec {
    pub job: JobBlock,
    #[serde(default)]
    pub retry: RetryBlock,
    #[serde(default)]
    pub schedule: Option<ScheduleBlock>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobBlock {
    pub kind: JobKind,
    pub source: Vec<PathBuf>,
    pub destination: PathBuf,
    #[serde(default)]
    pub verify: Option<String>,
    #[serde(default)]
    pub shape: Option<String>,
    #[serde(default)]
    pub preserve: PreserveBlock,
    #[serde(default)]
    pub collisions: CollisionBlock,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum JobKind {
    Copy,
    Move,
    Sync,
    Shred,
    Verify,
}

impl JobKind {
    pub fn as_str(self) -> &'static str {
        match self {
            JobKind::Copy => "copy",
            JobKind::Move => "move",
            JobKind::Sync => "sync",
            JobKind::Shred => "shred",
            JobKind::Verify => "verify",
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PreserveBlock {
    #[serde(default)]
    pub security_metadata: bool,
    #[serde(default)]
    pub sparseness: bool,
    #[serde(default)]
    pub timestamps: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CollisionBlock {
    #[serde(default)]
    pub policy: Option<String>,
    #[serde(default)]
    pub profile: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RetryBlock {
    #[serde(default = "RetryBlock::default_attempts")]
    pub attempts: u32,
    #[serde(default = "RetryBlock::default_backoff")]
    pub backoff: String,
}

impl RetryBlock {
    fn default_attempts() -> u32 {
        1
    }
    fn default_backoff() -> String {
        "0s".into()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduleBlock {
    pub cron: String,
}

/// Failure modes for spec parsing. Surfaces through `ExitCode::ConfigInvalid`.
#[derive(Debug, thiserror::Error)]
pub enum JobSpecError {
    #[error("read jobspec {path}: {source}")]
    Read {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("parse jobspec {path}: {source}")]
    Parse {
        path: PathBuf,
        #[source]
        source: Box<toml::de::Error>,
    },
    #[error("jobspec source list is empty")]
    EmptySources,
    #[error("jobspec source `{0}` does not exist")]
    SourceMissing(PathBuf),
    #[error("jobspec destination is not a directory and parent does not exist: {0}")]
    DestinationParentMissing(PathBuf),
}

impl JobSpec {
    /// Parse a TOML jobspec from disk. Errors are surfaced verbatim.
    pub fn load(path: &Path) -> Result<Self, JobSpecError> {
        let buf = std::fs::read_to_string(path).map_err(|source| JobSpecError::Read {
            path: path.to_path_buf(),
            source,
        })?;
        let spec: JobSpec = toml::from_str(&buf).map_err(|source| JobSpecError::Parse {
            path: path.to_path_buf(),
            source: Box::new(source),
        })?;
        spec.validate()?;
        Ok(spec)
    }

    /// Validate semantic invariants the TOML grammar can't express.
    pub fn validate(&self) -> Result<(), JobSpecError> {
        if self.job.source.is_empty() {
            return Err(JobSpecError::EmptySources);
        }
        for s in &self.job.source {
            if !s.exists() {
                return Err(JobSpecError::SourceMissing(s.clone()));
            }
        }
        if let Some(parent) = self.job.destination.parent() {
            if !parent.as_os_str().is_empty() && !parent.exists() && !self.job.destination.exists()
            {
                return Err(JobSpecError::DestinationParentMissing(
                    self.job.destination.clone(),
                ));
            }
        }
        Ok(())
    }
}

/// Outcome of `plan_jobspec` — the per-source list of actions plus an
/// aggregate counter the runtime exits on.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PlanReport {
    pub actions: Vec<PlannedAction>,
    pub already_done: u64,
    pub bytes: u64,
}

impl PlanReport {
    pub fn pending_count(&self) -> u64 {
        self.actions.len() as u64
    }
}

/// One unit of work the runtime would attempt during `apply`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlannedAction {
    pub action: PlannedActionKind,
    pub src: PathBuf,
    pub dst: PathBuf,
    /// Source size in bytes when known (`None` for missing files).
    pub bytes: Option<u64>,
    /// Free-form note for the human / JSON consumer.
    pub note: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PlannedActionKind {
    Copy,
    Replace,
    Skip,
    Verify,
    Shred,
}

/// Compute the action list for a jobspec. **Never** mutates the
/// filesystem — that's what `apply` is for. The diff is intentionally
/// shallow (file-presence + size) so `plan` is fast on huge trees;
/// content-hash comparisons are deferred to `apply` where the
/// existing engine already streams them through verify.
pub fn plan_jobspec(spec: &JobSpec) -> PlanReport {
    let mut report = PlanReport::default();
    let dst_root = &spec.job.destination;

    for src in &spec.job.source {
        match spec.job.kind {
            JobKind::Copy | JobKind::Move => walk_for_copy_or_move(src, dst_root, &mut report),
            JobKind::Verify => walk_for_verify(src, &mut report),
            JobKind::Shred => walk_for_shred(src, &mut report),
            JobKind::Sync => walk_for_sync(src, dst_root, &mut report),
        }
    }

    report
}

fn walk_for_copy_or_move(src: &Path, dst_root: &Path, report: &mut PlanReport) {
    let entries = collect_entries(src);
    for entry in entries {
        let rel = entry.path.strip_prefix(src).unwrap_or(&entry.path);
        let dst = dst_root.join(rel);
        if let Ok(dst_meta) = std::fs::metadata(&dst) {
            if dst_meta.is_file() && dst_meta.len() == entry.size {
                report.already_done += 1;
                continue;
            }
            // Same path exists with a different size → replace.
            report.bytes += entry.size;
            report.actions.push(PlannedAction {
                action: PlannedActionKind::Replace,
                src: entry.path.clone(),
                dst,
                bytes: Some(entry.size),
                note: Some("destination size differs from source".into()),
            });
        } else {
            report.bytes += entry.size;
            report.actions.push(PlannedAction {
                action: PlannedActionKind::Copy,
                src: entry.path.clone(),
                dst,
                bytes: Some(entry.size),
                note: None,
            });
        }
    }
}

fn walk_for_verify(src: &Path, report: &mut PlanReport) {
    for entry in collect_entries(src) {
        report.bytes += entry.size;
        report.actions.push(PlannedAction {
            action: PlannedActionKind::Verify,
            src: entry.path.clone(),
            dst: entry.path,
            bytes: Some(entry.size),
            note: None,
        });
    }
}

fn walk_for_shred(src: &Path, report: &mut PlanReport) {
    for entry in collect_entries(src) {
        report.bytes += entry.size;
        report.actions.push(PlannedAction {
            action: PlannedActionKind::Shred,
            src: entry.path.clone(),
            dst: entry.path,
            bytes: Some(entry.size),
            note: None,
        });
    }
}

fn walk_for_sync(src: &Path, dst_root: &Path, report: &mut PlanReport) {
    walk_for_copy_or_move(src, dst_root, report);
    let entries = collect_entries(dst_root);
    for entry in entries {
        let rel = entry.path.strip_prefix(dst_root).unwrap_or(&entry.path);
        let mirror = src.join(rel);
        if !mirror.exists() {
            report.bytes += entry.size;
            report.actions.push(PlannedAction {
                action: PlannedActionKind::Copy,
                src: entry.path.clone(),
                dst: mirror,
                bytes: Some(entry.size),
                note: Some("right-to-left mirror".into()),
            });
        }
    }
}

struct WalkEntry {
    path: PathBuf,
    size: u64,
}

/// Cheap, allocation-light shallow walk. Doesn't follow symlinks; for
/// the smoke test corpus a recursive `read_dir` is plenty.
fn collect_entries(root: &Path) -> Vec<WalkEntry> {
    let mut out = Vec::new();
    if !root.exists() {
        return out;
    }
    if let Ok(meta) = std::fs::metadata(root) {
        if meta.is_file() {
            out.push(WalkEntry {
                path: root.to_path_buf(),
                size: meta.len(),
            });
            return out;
        }
    }
    let mut stack = vec![root.to_path_buf()];
    while let Some(dir) = stack.pop() {
        let Ok(read) = std::fs::read_dir(&dir) else {
            continue;
        };
        for entry in read.flatten() {
            let path = entry.path();
            let Ok(meta) = entry.metadata() else {
                continue;
            };
            if meta.is_dir() {
                stack.push(path);
            } else if meta.is_file() {
                out.push(WalkEntry {
                    path,
                    size: meta.len(),
                });
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn parse_minimal_spec() {
        let dir = tempdir().unwrap();
        let src = dir.path().join("src");
        std::fs::create_dir(&src).unwrap();
        std::fs::write(src.join("a.txt"), b"hello").unwrap();
        let dst = dir.path().join("dst");
        std::fs::create_dir(&dst).unwrap();

        let spec_path = dir.path().join("job.toml");
        std::fs::write(
            &spec_path,
            format!(
                r#"
[job]
kind = "copy"
source = ["{src}"]
destination = "{dst}"
"#,
                src = src.display().to_string().replace('\\', "/"),
                dst = dst.display().to_string().replace('\\', "/"),
            ),
        )
        .unwrap();

        let spec = JobSpec::load(&spec_path).unwrap();
        assert_eq!(spec.job.kind, JobKind::Copy);
        assert_eq!(spec.job.source.len(), 1);
    }

    #[test]
    fn plan_emits_three_copies_for_three_files() {
        let dir = tempdir().unwrap();
        let src = dir.path().join("src");
        std::fs::create_dir(&src).unwrap();
        for i in 0..3 {
            std::fs::write(src.join(format!("f{i}.bin")), vec![i as u8; 1024]).unwrap();
        }
        let dst = dir.path().join("dst");
        std::fs::create_dir(&dst).unwrap();

        let spec = JobSpec {
            job: JobBlock {
                kind: JobKind::Copy,
                source: vec![src],
                destination: dst,
                verify: None,
                shape: None,
                preserve: PreserveBlock::default(),
                collisions: CollisionBlock::default(),
            },
            retry: RetryBlock::default(),
            schedule: None,
        };

        let plan = plan_jobspec(&spec);
        assert_eq!(plan.actions.len(), 3);
        assert!(
            plan.actions
                .iter()
                .all(|a| a.action == PlannedActionKind::Copy)
        );
    }

    #[test]
    fn plan_idempotent_when_destination_already_matches() {
        let dir = tempdir().unwrap();
        let src = dir.path().join("src");
        std::fs::create_dir(&src).unwrap();
        std::fs::write(src.join("a.txt"), b"hello").unwrap();

        let dst = dir.path().join("dst");
        std::fs::create_dir(&dst).unwrap();
        std::fs::write(dst.join("a.txt"), b"hello").unwrap();

        let spec = JobSpec {
            job: JobBlock {
                kind: JobKind::Copy,
                source: vec![src],
                destination: dst,
                verify: None,
                shape: None,
                preserve: PreserveBlock::default(),
                collisions: CollisionBlock::default(),
            },
            retry: RetryBlock::default(),
            schedule: None,
        };

        let plan = plan_jobspec(&spec);
        assert_eq!(plan.actions.len(), 0);
        assert_eq!(plan.already_done, 1);
    }
}
