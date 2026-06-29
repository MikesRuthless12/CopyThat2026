//! `copythat migrate` / `copythat export` — Phase 50 CDR-0 cross-tool
//! migration entry points.
//!
//! `migrate <from-tool> <src> <dst>` imports another tool's repository
//! INTO a CDR-0 repository. Only `cdr` is implemented today; `restic` /
//! `borg` / `kopia` print exactly what a full importer needs (see
//! `docs/spec/CDR-0.md §11`) and exit non-zero rather than guessing —
//! a wrong-but-successful import would corrupt the migration.
//!
//! `export <cdr> <to-tool> <dst>` is the inverse; writing another tool's
//! on-disk format is not yet implemented.

use std::sync::Arc;

use copythat_chunk::{RepoFormat, migrate};

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
    let pw = args
        .password
        .clone()
        .or_else(|| std::env::var("RESTIC_PASSWORD").ok());
    match migrate(from, &args.src, &args.dst, pw.as_deref()) {
        Ok(report) => {
            let _ = writer.human(&format!(
                "Migrated {} snapshot(s) / {} file(s) into the CDR-0 repository at {}",
                report.snapshots,
                report.files,
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
    let _ = writer.human(&format!(
        "Exporting a CDR-0 repository to a {tool} repository is not yet implemented: it \
         requires {tool}'s pack/index encoder + encryption (see docs/spec/CDR-0.md §11). To \
         copy or re-home a CDR-0 repository, use `copythat migrate cdr <src> <dst>`.",
        tool = to.as_str(),
    ));
    ExitCode::GenericError
}
