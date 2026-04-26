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

    let shape_sink = if let Some(rate) = &args.shape {
        match parse_byte_rate(rate) {
            Ok(bps) => {
                let _ = writer.human(&format!("(info) bandwidth shape `{rate}` ({bps} B/s)"));
                let shape = std::sync::Arc::new(copythat_shape::Shape::new(Some(
                    copythat_shape::ByteRate::new(bps),
                )));
                Some(
                    std::sync::Arc::new(copythat_shape::CopyThatShapeSink::new(shape))
                        as std::sync::Arc<dyn copythat_core::ShapeSink>,
                )
            }
            Err(e) => {
                let _ = writer.emit(JsonEventKind::Error {
                    message: format!("unknown shape rate `{rate}`: {e}"),
                    code: ExitCode::ConfigInvalid.as_u8(),
                });
                return ExitCode::ConfigInvalid;
            }
        }
    } else {
        None
    };

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

        // Phase 39 — attach the platform fast-copy hook so the CLI
        // exercises the same `CopyFileExW` + parallel-chunk + reflink
        // path the GUI does. Without this, the CLI silently fell
        // through to the pure-Rust async loop, which made bench-vs
        // numbers diverge from real-world UI numbers and hid the
        // CLI/UI gap that Phase 39 set out to close.
        let fast_copy_hook: std::sync::Arc<dyn copythat_core::FastCopyHook> =
            std::sync::Arc::new(copythat_platform::PlatformFastCopyHook);
        let mut copy_opts = CopyOptions {
            fail_if_exists: args.fail_if_exists,
            follow_symlinks: args.follow_symlinks,
            shape: shape_sink.clone(),
            fast_copy_hook: Some(fast_copy_hook),
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
        // Aggregate per-job totals from the event stream so the
        // emitted `JobCompleted` payload carries real numbers
        // instead of the placeholders `bytes:0, files:1, duration_ms:0`
        // every machine consumer was reading. The atomics let the
        // pump task own the increment side and the runner own the
        // read side after `event_pump.await`.
        let bytes_acc = std::sync::Arc::new(std::sync::atomic::AtomicU64::new(0));
        let files_acc = std::sync::Arc::new(std::sync::atomic::AtomicU64::new(0));
        let errors_acc = std::sync::Arc::new(std::sync::atomic::AtomicU64::new(0));
        let bytes_for_pump = bytes_acc.clone();
        let files_for_pump = files_acc.clone();
        let errors_for_pump = errors_acc.clone();
        let job_started_at = std::time::Instant::now();
        let event_pump = tokio::spawn(async move {
            while let Some(evt) = rx.recv().await {
                pump_event(
                    &writer_clone,
                    &job_id_clone,
                    evt,
                    &bytes_for_pump,
                    &files_for_pump,
                    &errors_for_pump,
                );
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
                let bytes = bytes_acc.load(std::sync::atomic::Ordering::Relaxed);
                let files = files_acc.load(std::sync::atomic::Ordering::Relaxed).max(1);
                let errors = errors_acc.load(std::sync::atomic::Ordering::Relaxed);
                let duration_ms = job_started_at.elapsed().as_millis() as u64;
                let _ = writer.emit(JsonEventKind::JobCompleted {
                    job_id,
                    bytes,
                    files,
                    duration_ms,
                });
                let _ = writer.human(&format!(
                    "{kind_str} done: {} -> {}",
                    src.display(),
                    dst.display()
                ));
                // A tree job that absorbed per-file FileError events
                // (under the engine's Skip / RetryN policy) returns
                // `Ok(())` overall — but reporting exit 0 in that case
                // hides real failures from CI pipelines / scripts.
                // Surface as `GenericError` when at least one file
                // failed.
                if errors > 0 && last_status == ExitCode::Success {
                    last_status = ExitCode::GenericError;
                }
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

fn pump_event(
    writer: &OutputWriter,
    job_id: &str,
    evt: CopyEvent,
    bytes_acc: &std::sync::atomic::AtomicU64,
    files_acc: &std::sync::atomic::AtomicU64,
    errors_acc: &std::sync::atomic::AtomicU64,
) {
    use std::sync::atomic::Ordering::Relaxed;
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
            files_done,
            ..
        } => {
            // Track running totals. We use `max` rather than `+=`
            // because TreeProgress emits cumulative totals, not
            // deltas — the latest value wins.
            bytes_acc.fetch_max(bytes_done, Relaxed);
            files_acc.fetch_max(files_done, Relaxed);
            let _ = writer.emit(JsonEventKind::JobProgress {
                job_id: job_id.into(),
                bytes_done,
                bytes_total,
                rate_bps: 0,
            });
        }
        CopyEvent::Completed { bytes, .. } => {
            // Single-file copies don't fire TreeProgress; capture
            // the final byte count + bump the file counter so the
            // JobCompleted payload reports `bytes:N, files:1` as
            // documented.
            bytes_acc.fetch_max(bytes, Relaxed);
            files_acc.fetch_add(1, Relaxed);
        }
        CopyEvent::TreeCompleted { bytes, files, .. } => {
            bytes_acc.fetch_max(bytes, Relaxed);
            files_acc.fetch_max(files, Relaxed);
        }
        CopyEvent::FileError { ref err } | CopyEvent::Failed { ref err } => {
            errors_acc.fetch_add(1, Relaxed);
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

/// Parse a human-friendly rate string into bytes-per-second.
/// Accepts plain integers (`512` = 512 B/s), `K` / `M` / `G`
/// suffixes (decimal: 10⁶), `KiB` / `MiB` / `GiB` suffixes
/// (binary: 2²⁰), and an optional trailing `/s` for clarity.
fn parse_byte_rate(s: &str) -> Result<u64, String> {
    let trimmed = s.trim().trim_end_matches("/s").trim();
    if trimmed.is_empty() {
        return Err("empty rate".into());
    }
    let (num_str, mul) = if let Some(p) = trimmed.strip_suffix("GiB") {
        (p.trim(), 1u64 << 30)
    } else if let Some(p) = trimmed.strip_suffix("MiB") {
        (p.trim(), 1u64 << 20)
    } else if let Some(p) = trimmed.strip_suffix("KiB") {
        (p.trim(), 1u64 << 10)
    } else if let Some(p) = trimmed.strip_suffix("GB") {
        (p.trim(), 1_000_000_000u64)
    } else if let Some(p) = trimmed.strip_suffix("MB") {
        (p.trim(), 1_000_000u64)
    } else if let Some(p) = trimmed.strip_suffix("KB") {
        (p.trim(), 1_000u64)
    } else if let Some(p) = trimmed.strip_suffix('G') {
        (p.trim(), 1_000_000_000u64)
    } else if let Some(p) = trimmed.strip_suffix('M') {
        (p.trim(), 1_000_000u64)
    } else if let Some(p) = trimmed.strip_suffix('K') {
        (p.trim(), 1_000u64)
    } else if let Some(p) = trimmed.strip_suffix('B') {
        (p.trim(), 1u64)
    } else {
        (trimmed, 1u64)
    };
    let n: u64 = num_str
        .parse()
        .map_err(|e| format!("not an integer: {num_str:?} ({e})"))?;
    n.checked_mul(mul)
        .ok_or_else(|| format!("rate overflow: {n} * {mul}"))
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
