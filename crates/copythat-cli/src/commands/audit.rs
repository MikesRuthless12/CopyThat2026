//! `copythat audit verify <log-file>` — staged for IPC plumbing.
//!
//! The chain-verification primitive (`copythat_audit::verify_chain`)
//! is already a pure function and could be wired here directly. To
//! keep the Phase 36 dependency footprint minimal we route the
//! command through the GUI's IPC for now; full CLI wiring lands in
//! the Phase 36 follow-up that imports `copythat-audit` against this
//! crate.

use std::sync::Arc;

use crate::ExitCode;
use crate::cli::{AuditArgs, AuditOp, GlobalArgs};
use crate::output::{JsonEventKind, OutputWriter};

pub(crate) async fn run(
    _global: &GlobalArgs,
    args: AuditArgs,
    writer: Arc<OutputWriter>,
) -> ExitCode {
    let summary = match args.op {
        AuditOp::Verify { log_file } => format!("audit-verify `{}`", log_file.display()),
    };
    let _ = writer.emit(JsonEventKind::Info {
        message: format!(
            "{summary} parsed; CLI follow-up will plumb \
             `copythat_audit::verify_chain` through the same exit-code surface as `verify`."
        ),
    });
    let _ = writer.human(&format!(
        "{summary}: scheduled — wiring lands in a follow-up phase."
    ));
    ExitCode::GenericError
}
