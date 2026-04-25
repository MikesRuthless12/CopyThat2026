//! `copythat verify <path> --algo <ALGO> [--against <SIDECAR>]`.
//!
//! Streams the file through `copythat_hash::hash_file_async`. When a
//! sidecar is supplied, the digest is compared against the first hex
//! token on the line that matches the file's basename; mismatches
//! exit `ExitCode::VerifyFailed` (4).

use std::sync::Arc;

use copythat_core::CopyControl;
use copythat_hash::{HashAlgorithm, HashEvent, hash_file_async};
use tokio::sync::mpsc;

use crate::ExitCode;
use crate::cli::{GlobalArgs, VerifyArgs};
use crate::output::{JsonEventKind, OutputWriter};

pub(crate) async fn run(
    _global: &GlobalArgs,
    args: VerifyArgs,
    writer: Arc<OutputWriter>,
) -> ExitCode {
    let algo: HashAlgorithm = match args.algo.parse() {
        Ok(a) => a,
        Err(_) => {
            let _ = writer.emit(JsonEventKind::Error {
                message: format!("unknown verify algorithm `{}`", args.algo),
                code: ExitCode::ConfigInvalid.as_u8(),
            });
            return ExitCode::ConfigInvalid;
        }
    };

    let (tx, mut rx) = mpsc::channel::<HashEvent>(64);
    let ctrl = CopyControl::new();
    let pump = tokio::spawn(async move { while rx.recv().await.is_some() {} });

    let report = match hash_file_async(&args.path, algo, ctrl, tx).await {
        Ok(r) => r,
        Err(e) => {
            let _ = writer.emit(JsonEventKind::Error {
                message: e.to_string(),
                code: ExitCode::GenericError.as_u8(),
            });
            return ExitCode::GenericError;
        }
    };
    let _ = pump.await;

    let actual_hex = bytes_to_hex(&report.digest);

    if let Some(sidecar) = &args.against {
        let expected = match read_sidecar_for_path(sidecar, &args.path) {
            Ok(Some(s)) => s,
            Ok(None) => {
                let _ = writer.emit(JsonEventKind::Error {
                    message: format!(
                        "sidecar `{}` does not contain an entry for `{}`",
                        sidecar.display(),
                        args.path.display()
                    ),
                    code: ExitCode::ConfigInvalid.as_u8(),
                });
                return ExitCode::ConfigInvalid;
            }
            Err(e) => {
                let _ = writer.emit(JsonEventKind::Error {
                    message: format!("read sidecar `{}`: {e}", sidecar.display()),
                    code: ExitCode::ConfigInvalid.as_u8(),
                });
                return ExitCode::ConfigInvalid;
            }
        };

        if expected.eq_ignore_ascii_case(&actual_hex) {
            let _ = writer.emit(JsonEventKind::VerifyOk {
                path: args.path.display().to_string(),
                algo: algo.name().into(),
                digest: actual_hex.clone(),
            });
            let _ = writer.human(&format!("verify ok: {} {}", algo.name(), actual_hex));
            ExitCode::Success
        } else {
            let _ = writer.emit(JsonEventKind::VerifyFailed {
                path: args.path.display().to_string(),
                algo: algo.name().into(),
                expected: Some(expected),
                actual: actual_hex,
            });
            let _ = writer.human(&format!("verify FAILED: {}", algo.name()));
            ExitCode::VerifyFailed
        }
    } else {
        let _ = writer.emit(JsonEventKind::VerifyOk {
            path: args.path.display().to_string(),
            algo: algo.name().into(),
            digest: actual_hex.clone(),
        });
        let _ = writer.human(&format!("{} {}", algo.name(), actual_hex));
        ExitCode::Success
    }
}

fn bytes_to_hex(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        use std::fmt::Write;
        let _ = write!(&mut s, "{b:02x}");
    }
    s
}

/// Find the line whose final whitespace-separated token matches the
/// target file's basename and return the leading hex digest. Matches
/// the TeraCopy / GNU `*sum` sidecar format: `<hex>  <relpath>`.
fn read_sidecar_for_path(
    sidecar: &std::path::Path,
    target: &std::path::Path,
) -> std::io::Result<Option<String>> {
    let body = std::fs::read_to_string(sidecar)?;
    let target_name = target
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or_default();
    for line in body.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        // Match either `<hex> <name>` or `<hex>  <name>` (single or
        // double space; binary marker `*` accepted before the name).
        let mut parts = line.splitn(2, char::is_whitespace);
        let hex = parts.next().unwrap_or_default();
        let rest = parts.next().unwrap_or_default().trim_start();
        let rest = rest.strip_prefix('*').unwrap_or(rest);
        if rest == target_name
            || std::path::Path::new(rest)
                .file_name()
                .and_then(|n| n.to_str())
                == Some(target_name)
        {
            return Ok(Some(hex.to_string()));
        }
    }
    Ok(None)
}
