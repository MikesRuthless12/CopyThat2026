//! `copythat provenance <verify|keygen> …` — Phase 43.
//!
//! `verify` re-hashes each file the manifest references and prints
//! "Manifest valid for N files; tampered: M (paths…)." mirroring the
//! Phase 43 spec. `keygen` writes a fresh ed25519 PKCS#8 PEM private
//! key (and optionally a SubjectPublicKeyInfo PEM public key) so the
//! UI's "Manage signing keys" panel has a CLI counterpart for
//! headless setup.

use std::path::Path;
use std::sync::Arc;

use copythat_provenance::{
    VerificationOutcome, generate_signing_key, signing_key_to_pem, verify_manifest,
    verifying_key_from_pem, verifying_key_to_pem,
};

use crate::ExitCode;
use crate::cli::{
    GlobalArgs, ProvenanceAction, ProvenanceArgs, ProvenanceKeygenArgs, ProvenanceVerifyArgs,
};
use crate::output::{JsonEventKind, OutputWriter};

pub(crate) async fn run(
    global: &GlobalArgs,
    args: ProvenanceArgs,
    writer: Arc<OutputWriter>,
) -> ExitCode {
    match args.action {
        ProvenanceAction::Verify(v) => verify(global, v, writer).await,
        ProvenanceAction::Keygen(k) => keygen(global, k, writer).await,
    }
}

async fn verify(
    _global: &GlobalArgs,
    args: ProvenanceVerifyArgs,
    writer: Arc<OutputWriter>,
) -> ExitCode {
    let trusted_key_bytes = match args.trusted_key.as_ref() {
        Some(path) => match load_trusted_key(path) {
            Ok(bytes) => Some(bytes),
            Err(e) => {
                let _ = writer.emit(JsonEventKind::Error {
                    message: format!("trusted-key load failed: {e}"),
                    code: ExitCode::ConfigInvalid.as_u8(),
                });
                return ExitCode::ConfigInvalid;
            }
        },
        None => None,
    };

    let report = match verify_manifest(&args.manifest, trusted_key_bytes.as_ref()) {
        Ok(r) => r,
        Err(e) => {
            let _ = writer.emit(JsonEventKind::Error {
                message: format!("verify failed: {e}"),
                code: ExitCode::ConfigInvalid.as_u8(),
            });
            return ExitCode::ConfigInvalid;
        }
    };

    if report.all_clean() {
        let _ = writer.emit(JsonEventKind::Info {
            message: format!(
                "Manifest valid for {} files; signature {}; merkle root OK.",
                report.ok_count,
                match report.signature_ok {
                    Some(true) => "VALID",
                    Some(false) => "INVALID",
                    None => "ABSENT",
                }
            ),
        });
        ExitCode::Success
    } else {
        // Phase 43 re-review (H-1, H-2) — collect tampered AND
        // missing paths, both clamped at 32 entries with `take(32)`
        // (no cloning a million-element vec just to truncate the
        // first 32). Build the human-readable summary from the full
        // tampered_paths_full vec separately.
        let tampered_paths_full: Vec<String> = report
            .per_file
            .iter()
            .filter_map(|(path, outcome)| match outcome {
                VerificationOutcome::Tampered { .. } => Some(path.display().to_string()),
                _ => None,
            })
            .collect();
        let missing_paths_full: Vec<String> = report
            .per_file
            .iter()
            .filter_map(|(path, outcome)| match outcome {
                VerificationOutcome::Missing { .. } => Some(path.display().to_string()),
                _ => None,
            })
            .collect();
        let tampered_for_event: Vec<String> =
            tampered_paths_full.iter().take(32).cloned().collect();
        let missing_for_event: Vec<String> = missing_paths_full.iter().take(32).cloned().collect();
        let _ = writer.emit(JsonEventKind::ProvenanceVerifyFailed {
            manifest: args.manifest.display().to_string(),
            ok_count: report.ok_count as u64,
            tampered_count: report.tampered_count as u64,
            missing_count: report.missing_count as u64,
            merkle_root_ok: report.merkle_root_ok,
            signature_ok: report.signature_ok,
            timestamp_ok: report.timestamp_ok,
            tampered_paths: tampered_for_event,
            missing_paths: missing_for_event,
        });
        let tampered_paths = tampered_paths_full;
        // Also emit the human-readable summary as Info for the
        // default text mode (the typed event is redundant under
        // OutputMode::Human; the writer drops it).
        let _ = writer.emit(JsonEventKind::Info {
            message: format!(
                "Manifest valid for {} files; tampered: {} ({}); missing: {}; signature {}; merkle {}.",
                report.ok_count,
                report.tampered_count,
                if tampered_paths.is_empty() {
                    "—".into()
                } else {
                    tampered_paths.join(", ")
                },
                report.missing_count,
                match report.signature_ok {
                    Some(true) => "VALID",
                    Some(false) => "INVALID",
                    None => "ABSENT",
                },
                if report.merkle_root_ok { "OK" } else { "MISMATCH" },
            ),
        });
        ExitCode::VerifyFailed
    }
}

async fn keygen(
    _global: &GlobalArgs,
    args: ProvenanceKeygenArgs,
    writer: Arc<OutputWriter>,
) -> ExitCode {
    let sk = generate_signing_key();
    let priv_pem = match signing_key_to_pem(&sk) {
        Ok(s) => s,
        Err(e) => {
            let _ = writer.emit(JsonEventKind::Error {
                message: format!("private-key encode failed: {e}"),
                code: ExitCode::GenericError.as_u8(),
            });
            return ExitCode::GenericError;
        }
    };
    if let Err(e) = std::fs::write(&args.out, &priv_pem) {
        let _ = writer.emit(JsonEventKind::Error {
            message: format!("write {}: {e}", args.out.display()),
            code: ExitCode::PermissionDenied.as_u8(),
        });
        return ExitCode::PermissionDenied;
    }

    let pub_pem = match verifying_key_to_pem(&sk.verifying_key()) {
        Ok(s) => s,
        Err(e) => {
            let _ = writer.emit(JsonEventKind::Error {
                message: format!("public-key encode failed: {e}"),
                code: ExitCode::GenericError.as_u8(),
            });
            return ExitCode::GenericError;
        }
    };

    if args.write_public {
        let pub_path = args.out.with_extension(
            args.out
                .extension()
                .map(|e| {
                    let mut s = e.to_os_string();
                    s.push(".pub");
                    s
                })
                .unwrap_or_else(|| "pub".into()),
        );
        if let Err(e) = std::fs::write(&pub_path, &pub_pem) {
            let _ = writer.emit(JsonEventKind::Error {
                message: format!("write public key {}: {e}", pub_path.display()),
                code: ExitCode::PermissionDenied.as_u8(),
            });
            return ExitCode::PermissionDenied;
        }
    } else {
        // Print to stdout in human mode; in JSON mode, emit a config
        // event with the pem under `value`.
        match writer.mode() {
            crate::output::OutputMode::Json => {
                let _ = writer.emit(JsonEventKind::ConfigValue {
                    key: "provenance.public_key_pem".into(),
                    value: serde_json::Value::String(pub_pem),
                });
            }
            crate::output::OutputMode::Human => {
                println!("{pub_pem}");
            }
            crate::output::OutputMode::Quiet => {}
        }
    }

    ExitCode::Success
}

fn load_trusted_key(path: &Path) -> Result<[u8; 32], String> {
    let pem = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
    let vk = verifying_key_from_pem(&pem).map_err(|e| e.to_string())?;
    Ok(vk.to_bytes())
}
