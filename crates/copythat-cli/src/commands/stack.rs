//! `copythat stack add|list|clear|copy-to` — staged for IPC plumbing.
//!
//! The Drop Stack itself lives in the GUI runner's persistent state
//! (Phase 28). This stub accepts the flag surface so scripts written
//! today don't break when the IPC bridge lands in a follow-up phase.

use std::sync::Arc;

use crate::ExitCode;
use crate::cli::{GlobalArgs, StackArgs, StackOp};
use crate::output::{JsonEventKind, OutputWriter};

pub(crate) async fn run(
    _global: &GlobalArgs,
    args: StackArgs,
    writer: Arc<OutputWriter>,
) -> ExitCode {
    let summary = match args.op {
        StackOp::Add { path } => format!("stack-add `{}`", path.display()),
        StackOp::List => "stack-list".to_string(),
        StackOp::Clear => "stack-clear".to_string(),
        StackOp::CopyTo { dst } => format!("stack-copy-to `{}`", dst.display()),
    };
    let _ = writer.emit(JsonEventKind::Info {
        message: format!(
            "{summary} parsed; CLI follow-up will plumb \
             `apps/copythat-ui/src-tauri::DropStackState` through a stable IPC."
        ),
    });
    let _ = writer.human(&format!(
        "{summary}: scheduled — wiring lands in a follow-up phase."
    ));
    ExitCode::GenericError
}
