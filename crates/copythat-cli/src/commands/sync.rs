//! `copythat sync <left> <right>` — staged for IPC plumbing.
//!
//! The two-way conflict-aware sync engine ships in `copythat-sync`
//! (Phase 25). The CLI surface accepts the same flags the GUI uses
//! and emits a clearly-labelled `info` JSON event so scripts written
//! against this surface today don't break when the wiring lands in
//! a follow-up phase.

use std::sync::Arc;

use crate::ExitCode;
use crate::cli::{GlobalArgs, SyncArgs};
use crate::output::{JsonEventKind, OutputWriter};

pub(crate) async fn run(
    _global: &GlobalArgs,
    args: SyncArgs,
    writer: Arc<OutputWriter>,
) -> ExitCode {
    let _ = writer.emit(JsonEventKind::Info {
        message: format!(
            "sync mode `{}` parsed for {} <-> {}; CLI follow-up will plumb \
             `copythat_sync::run_sync` through the same JobBatch surface as the GUI runner.",
            args.mode,
            args.left.display(),
            args.right.display(),
        ),
    });
    let _ = writer.human(&format!(
        "sync ({}) {} <-> {}: scheduled — invoke from the GUI for the first cut.",
        args.mode,
        args.left.display(),
        args.right.display(),
    ));
    ExitCode::GenericError
}
