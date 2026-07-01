//! `copythat migrate` / `copythat export` — Phase 50 CDR-0 cross-tool
//! migration entry points.
//!
//! `migrate <from-tool> <src> <dst>` imports another tool's repository
//! INTO a CDR-0 repository. `cdr`, `restic`, `borg`, and `kopia` are all
//! implemented (each validated byte-identical against a committed fixture;
//! see `docs/spec/CDR-0.md §11`).
//!
//! `export <cdr> <to-tool> <dst>` is the inverse; writing another tool's
//! on-disk format is not yet implemented.

use std::sync::Arc;

use copythat_chunk::{RepoFormat, export, migrate};

use crate::ExitCode;
use crate::cli::{ExportArgs, GlobalArgs, MigrateArgs};
use crate::output::OutputWriter;

pub(crate) async fn run(
    _global: &GlobalArgs,
    args: MigrateArgs,
    writer: Arc<OutputWriter>,
) -> ExitCode {
    let Some(from) = RepoFormat::parse(&args.from) else {
        let _ = writer.human(&format!(
            "unknown source tool {:?}; expected one of: cdr, restic, borg, kopia",
            args.from
        ));
        return ExitCode::ConfigInvalid;
    };
    let pw = args.password.clone().or_else(|| {
        // Fall back to each tool's conventional passphrase env var.
        let env_var = match from {
            RepoFormat::Restic => "RESTIC_PASSWORD",
            RepoFormat::Borg => "BORG_PASSPHRASE",
            RepoFormat::Kopia => "KOPIA_PASSWORD",
            _ => return None,
        };
        std::env::var(env_var).ok()
    });
    match migrate(from, &args.src, &args.dst, pw.as_deref()) {
        Ok(report) => {
            let skipped = if report.skipped > 0 {
                format!(
                    " ({} non-content node(s) skipped: symlinks / devices / empty dirs)",
                    report.skipped
                )
            } else {
                String::new()
            };
            let _ = writer.human(&format!(
                "Migrated {} snapshot(s) / {} file(s){} into the CDR-0 repository at {}",
                report.snapshots,
                report.files,
                skipped,
                args.dst.display(),
            ));
            ExitCode::Success
        }
        Err(e) => {
            let _ = writer.human(&format!("migrate failed: {e}"));
            ExitCode::GenericError
        }
    }
}

pub(crate) async fn run_export(
    global: &GlobalArgs,
    args: ExportArgs,
    writer: Arc<OutputWriter>,
) -> ExitCode {
    let Some(to) = RepoFormat::parse(&args.to) else {
        let _ = writer.human(&format!(
            "unknown target tool {:?}; expected one of: restic, borg, kopia",
            args.to
        ));
        return ExitCode::ConfigInvalid;
    };
    if to == RepoFormat::Cdr {
        // CDR → CDR is just a copy / re-home; route through `migrate`.
        return run(
            global,
            MigrateArgs {
                from: "cdr".to_string(),
                src: args.src,
                dst: args.dst,
                password: None,
            },
            writer,
        )
        .await;
    }
    let pw = args.password.clone().or_else(|| {
        let env_var = match to {
            RepoFormat::Restic => "RESTIC_PASSWORD",
            RepoFormat::Borg => "BORG_PASSPHRASE",
            RepoFormat::Kopia => "KOPIA_PASSWORD",
            _ => return None,
        };
        std::env::var(env_var).ok()
    });
    match export(to, &args.src, &args.dst, pw.as_deref()) {
        Ok(report) => {
            let _ = writer.human(&format!(
                "Exported {} snapshot(s) / {} file(s) into the {} repository at {}",
                report.snapshots,
                report.files,
                to.as_str(),
                args.dst.display(),
            ));
            ExitCode::Success
        }
        Err(e) => {
            // kopia returns a typed `SourceUnsupported`; restic/borg surface
            // any I/O or crypto failure. Both print a clear line here.
            let _ = writer.human(&format!("export failed: {e}"));
            ExitCode::GenericError
        }
    }
}
