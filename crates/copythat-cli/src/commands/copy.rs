//! `copythat copy` and `copythat move`.
//!
//! Both routes share argument parsing (`CopyArgs`); the boolean
//! `is_move` flag in `run` selects the move-vs-copy entry point in
//! `copythat_core`. The CLI surface accepts N+1 paths (last is
//! destination) and dispatches per-source through the engine.

use std::path::{Path, PathBuf};
use std::sync::Arc;

use copythat_core::{
    CopyControl, CopyEvent, CopyOptions, MoveOptions, TreeOptions, copy_file, copy_tree, move_file,
    move_tree,
};
use tokio::sync::mpsc;

use crate::ExitCode;
use crate::cli::{CopyArgs, GlobalArgs};
use crate::output::{JsonEventKind, OutputWriter};

pub(crate) async fn run(
    _global: &GlobalArgs,
    args: CopyArgs,
    writer: Arc<OutputWriter>,
    is_move: bool,
) -> ExitCode {
    if args.paths.len() < 2 {
        let _ = writer.emit(JsonEventKind::Error {
            message: "copy/move requires at least one source and a destination".into(),
            code: ExitCode::ConfigInvalid.as_u8(),
        });
        return ExitCode::ConfigInvalid;
    }
    let mut paths = args.paths;
    let dst_root = paths.pop().expect("paths.len() >= 2");
    let sources = paths;

    if let Some(rate) = &args.shape {
        let _ = writer.emit(JsonEventKind::Info {
            message: format!(
                "shape `{rate}` is parsed but enforcement is wired through copythat_shape; \
                 CLI follow-up will plumb it into CopyOptions"
            ),
        });
        let _ = writer.human(&format!("(info) bandwidth shape `{rate}` recorded"));
    }

    if !dst_root_ok(&dst_root, sources.len() > 1) {
        let _ = writer.emit(JsonEventKind::Error {
            message: format!(
                "destination `{}` does not exist or is not a directory for multi-source {} job",
                dst_root.display(),
                if is_move { "move" } else { "copy" }
            ),
            code: ExitCode::ConfigInvalid.as_u8(),
        });
        return ExitCode::ConfigInvalid;
    }

    let mut last_status = ExitCode::Success;

    for src in &sources {
        let job_id = generate_job_id(src);
        let dst = pick_destination(src, &dst_root);
        let kind_str = if is_move { "move" } else { "copy" };

        let _ = writer.emit(JsonEventKind::JobStarted {
            job_id: job_id.clone(),
            src: src.display().to_string(),
            dst: dst.display().to_string(),
            operation: kind_str.into(),
        });
        let _ = writer.human(&format!(
            "{kind_str}: {} -> {}",
            src.display(),
            dst.display()
        ));

        let mut copy_opts = CopyOptions {
            fail_if_exists: args.fail_if_exists,
            follow_symlinks: args.follow_symlinks,
            ..CopyOptions::default()
        };

        if let Some(algo_name) = &args.verify {
            match algo_name.parse::<copythat_hash::HashAlgorithm>() {
                Ok(algo) => {
                    copy_opts.verify = Some(algo.verifier());
                }
                Err(_) => {
                    let _ = writer.emit(JsonEventKind::Error {
                        message: format!("unknown verify algorithm `{algo_name}`"),
                        code: ExitCode::ConfigInvalid.as_u8(),
                    });
                    return ExitCode::ConfigInvalid;
                }
            }
        }

        let (tx, mut rx) = mpsc::channel::<CopyEvent>(64);
        let ctrl = CopyControl::new();
        let writer_clone = writer.clone();
        let job_id_clone = job_id.clone();
        let event_pump = tokio::spawn(async move {
            while let Some(evt) = rx.recv().await {
                pump_event(&writer_clone, &job_id_clone, evt);
            }
        });

        let src_path = src.clone();
        let dst_path = dst.clone();
        let result: Result<(), copythat_core::CopyError> = if src_path.is_dir() {
            if is_move {
                let mv = MoveOptions {
                    copy: copy_opts.clone(),
                    ..MoveOptions::default()
                };
                move_tree(&src_path, &dst_path, mv, ctrl, tx)
                    .await
                    .map(|_| ())
            } else {
                let tree_opts = TreeOptions {
                    file: copy_opts.clone(),
                    ..TreeOptions::default()
                };
                copy_tree(&src_path, &dst_path, tree_opts, ctrl, tx)
                    .await
                    .map(|_| ())
            }
        } else if is_move {
            let mv = MoveOptions {
                copy: copy_opts.clone(),
                ..MoveOptions::default()
            };
            move_file(&src_path, &dst_path, mv, ctrl, tx)
                .await
                .map(|_| ())
        } else {
            copy_file(&src_path, &dst_path, copy_opts, ctrl, tx)
                .await
                .map(|_| ())
        };
        let _ = event_pump.await;

        match result {
            Ok(()) => {
                let _ = writer.emit(JsonEventKind::JobCompleted {
                    job_id,
                    bytes: 0,
                    files: 1,
                    duration_ms: 0,
                });
                let _ = writer.human(&format!(
                    "{kind_str} done: {} -> {}",
                    src.display(),
                    dst.display()
                ));
            }
            Err(e) => {
                let exit = classify_engine_error(&e);
                let _ = writer.emit(JsonEventKind::JobFailed {
                    job_id,
                    reason: e.to_string(),
                });
                let _ = writer.human(&format!("{kind_str} failed: {e}"));
                if exit != ExitCode::Success {
                    last_status = exit;
                }
            }
        }
    }

    last_status
}

fn pump_event(writer: &OutputWriter, job_id: &str, evt: CopyEvent) {
    match evt {
        CopyEvent::Progress { bytes, total, .. } => {
            let _ = writer.emit(JsonEventKind::JobProgress {
                job_id: job_id.into(),
                bytes_done: bytes,
                bytes_total: total,
                rate_bps: 0,
            });
        }
        CopyEvent::TreeProgress {
            bytes_done,
            bytes_total,
            ..
        } => {
            let _ = writer.emit(JsonEventKind::JobProgress {
                job_id: job_id.into(),
                bytes_done,
                bytes_total,
                rate_bps: 0,
            });
        }
        CopyEvent::FileError { ref err } | CopyEvent::Failed { ref err } => {
            let _ = writer.emit(JsonEventKind::Error {
                message: err.to_string(),
                code: ExitCode::GenericError.as_u8(),
            });
        }
        CopyEvent::VerifyFailed {
            algorithm,
            src_hex,
            dst_hex,
        } => {
            let _ = writer.emit(JsonEventKind::VerifyFailed {
                path: String::new(),
                algo: algorithm.into(),
                expected: Some(src_hex),
                actual: dst_hex,
            });
        }
        _ => {}
    }
}

fn classify_engine_error(e: &copythat_core::CopyError) -> ExitCode {
    use copythat_core::CopyErrorKind as K;
    match e.kind {
        K::VerifyFailed => ExitCode::VerifyFailed,
        K::PermissionDenied => ExitCode::PermissionDenied,
        K::DiskFull => ExitCode::DiskFull,
        K::Interrupted => ExitCode::UserCanceled,
        _ => ExitCode::GenericError,
    }
}

fn dst_root_ok(dst: &Path, multi_source: bool) -> bool {
    if dst.exists() {
        return true;
    }
    if multi_source {
        return false;
    }
    dst.parent()
        .map(|p| p.as_os_str().is_empty() || p.exists())
        .unwrap_or(true)
}

fn pick_destination(src: &Path, dst_root: &Path) -> PathBuf {
    if dst_root.is_dir() {
        if let Some(name) = src.file_name() {
            return dst_root.join(name);
        }
    }
    dst_root.to_path_buf()
}

fn generate_job_id(src: &Path) -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    format!(
        "cli-{}-{nonce:x}",
        src.file_name().and_then(|n| n.to_str()).unwrap_or("job")
    )
}
