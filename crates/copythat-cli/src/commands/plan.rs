//! `copythat plan --spec <jobspec.toml>` and `copythat apply ...`.
//!
//! `plan` reads the spec, computes the action list via `jobspec::plan_jobspec`,
//! prints each action as a JSON event, and exits **2** if any actions
//! are pending — exit **0** if everything is already done.
//!
//! `apply` runs the same plan; pending actions are routed through
//! `commands::copy::run` so the wiring is identical to `copythat copy`.
//! Re-running on a finished tree exits **0** (idempotency).

use std::sync::Arc;

use copythat_core::{CopyControl, CopyEvent, CopyOptions, copy_file};
use tokio::sync::mpsc;

use crate::ExitCode;
use crate::cli::{GlobalArgs, PlanArgs};
use crate::jobspec::{JobKind, JobSpec, PlannedActionKind, plan_jobspec};
use crate::output::{JsonEventKind, OutputWriter};

pub(crate) async fn run(
    _global: &GlobalArgs,
    args: PlanArgs,
    writer: Arc<OutputWriter>,
    apply: bool,
) -> ExitCode {
    let spec = match JobSpec::load(&args.spec) {
        Ok(s) => s,
        Err(e) => {
            let _ = writer.emit(JsonEventKind::Error {
                message: e.to_string(),
                code: ExitCode::ConfigInvalid.as_u8(),
            });
            return ExitCode::ConfigInvalid;
        }
    };

    let report = plan_jobspec(&spec);

    for action in &report.actions {
        let _ = writer.emit(JsonEventKind::PlanAction {
            action: serde_plain(action.action),
            src: Some(action.src.display().to_string()),
            dst: Some(action.dst.display().to_string()),
            bytes: action.bytes,
            note: action.note.clone(),
        });
        let _ = writer.human(&format!(
            "{:>8}  {} -> {}",
            serde_plain(action.action),
            action.src.display(),
            action.dst.display()
        ));
    }
    let _ = writer.emit(JsonEventKind::PlanSummary {
        actions: report.pending_count(),
        bytes: report.bytes,
        already_done: report.already_done,
    });
    let _ = writer.human(&format!(
        "summary: {} action(s), {} byte(s); {} already in place",
        report.pending_count(),
        report.bytes,
        report.already_done,
    ));

    if !apply {
        return if report.actions.is_empty() {
            ExitCode::Success
        } else {
            ExitCode::PendingActions
        };
    }

    if report.actions.is_empty() {
        return ExitCode::Success;
    }

    if !matches!(spec.job.kind, JobKind::Copy | JobKind::Move | JobKind::Sync) {
        let _ = writer.emit(JsonEventKind::Error {
            message: format!(
                "apply currently runs copy/move/sync jobspecs only — `{}` is staged for a follow-up phase",
                spec.job.kind.as_str()
            ),
            code: ExitCode::GenericError.as_u8(),
        });
        return ExitCode::GenericError;
    }

    let mut last_status = ExitCode::Success;
    for action in report.actions {
        if !matches!(
            action.action,
            PlannedActionKind::Copy | PlannedActionKind::Replace
        ) {
            continue;
        }
        if let Some(parent) = action.dst.parent() {
            if !parent.as_os_str().is_empty() {
                if let Err(e) = std::fs::create_dir_all(parent) {
                    let _ = writer.emit(JsonEventKind::Error {
                        message: format!("create_dir_all({}): {e}", parent.display()),
                        code: ExitCode::GenericError.as_u8(),
                    });
                    last_status = ExitCode::GenericError;
                    continue;
                }
            }
        }

        let opts = CopyOptions::default();
        let (tx, mut rx) = mpsc::channel::<CopyEvent>(64);
        let ctrl = CopyControl::new();
        let writer_clone = writer.clone();
        let pump = tokio::spawn(async move {
            while let Some(_evt) = rx.recv().await {
                let _ = &writer_clone;
            }
        });
        let result = copy_file(&action.src, &action.dst, opts, ctrl, tx).await;
        let _ = pump.await;

        match result {
            Ok(_) => {
                let _ = writer.emit(JsonEventKind::JobCompleted {
                    job_id: format!(
                        "apply-{}",
                        action
                            .src
                            .file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or("file")
                    ),
                    bytes: action.bytes.unwrap_or(0),
                    files: 1,
                    duration_ms: 0,
                });
            }
            Err(e) => {
                let _ = writer.emit(JsonEventKind::JobFailed {
                    job_id: action.src.display().to_string(),
                    reason: e.to_string(),
                });
                last_status = ExitCode::GenericError;
            }
        }
    }

    last_status
}

fn serde_plain(kind: PlannedActionKind) -> String {
    match kind {
        PlannedActionKind::Copy => "copy".into(),
        PlannedActionKind::Replace => "replace".into(),
        PlannedActionKind::Skip => "skip".into(),
        PlannedActionKind::Verify => "verify".into(),
        PlannedActionKind::Shred => "shred".into(),
    }
}
