//! `copythat remote add|list|remove|test` — staged for IPC plumbing.
//!
//! Remote management currently lives in the GUI's Settings → Remotes
//! panel, backed by `copythat-cloud` + the OS keychain. The CLI
//! surface accepts the flags so scripts written today keep parsing.

use std::sync::Arc;

use crate::ExitCode;
use crate::cli::{GlobalArgs, RemoteArgs, RemoteOp};
use crate::output::{JsonEventKind, OutputWriter};

pub(crate) async fn run(
    _global: &GlobalArgs,
    args: RemoteArgs,
    writer: Arc<OutputWriter>,
) -> ExitCode {
    let summary = match args.op {
        RemoteOp::Add { name, url } => format!("remote-add `{name}` -> `{url}`"),
        RemoteOp::List => "remote-list".to_string(),
        RemoteOp::Remove { name } => format!("remote-remove `{name}`"),
        RemoteOp::Test { name } => format!("remote-test `{name}`"),
    };
    let _ = writer.emit(JsonEventKind::Info {
        message: format!(
            "{summary} parsed; CLI follow-up will plumb \
             `copythat_cloud::Backend` through a stable IPC + bring the keychain integration to the CLI host."
        ),
    });
    let _ = writer.human(&format!(
        "{summary}: scheduled — wiring lands in a follow-up phase."
    ));
    ExitCode::GenericError
}
