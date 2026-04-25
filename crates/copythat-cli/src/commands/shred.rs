//! `copythat shred <path>` — staged for IPC plumbing.
//!
//! The pattern-based secure delete engine lives in
//! `copythat-secure-delete`. This stub accepts the flags surface so
//! scripts written today keep parsing; the wiring is one phase away.

use std::sync::Arc;

use crate::ExitCode;
use crate::cli::{GlobalArgs, ShredArgs};
use crate::output::{JsonEventKind, OutputWriter};

pub(crate) async fn run(
    _global: &GlobalArgs,
    args: ShredArgs,
    writer: Arc<OutputWriter>,
) -> ExitCode {
    let _ = writer.emit(JsonEventKind::Info {
        message: format!(
            "shred method `{}` parsed for `{}`; CLI follow-up will plumb \
             `copythat_secure_delete::shred_path`.",
            args.method,
            args.path.display(),
        ),
    });
    let _ = writer.human(&format!(
        "shred ({}) {}: scheduled — wiring lands in a follow-up phase.",
        args.method,
        args.path.display(),
    ));
    ExitCode::GenericError
}
