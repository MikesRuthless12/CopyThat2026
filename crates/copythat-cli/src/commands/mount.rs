//! `copythat mount <mountpoint> [--job <id>]` — staged for IPC plumbing.
//!
//! The mount-as-filesystem runtime lives in `copythat-mount` (Phase 33)
//! and currently activates from the GUI's History → Mount entry-point.
//! The CLI surface accepts the flags so scripts written today don't
//! break when the wiring lands in a follow-up phase.

use std::sync::Arc;

use crate::ExitCode;
use crate::cli::{GlobalArgs, MountArgs};
use crate::output::{JsonEventKind, OutputWriter};

pub(crate) async fn run(
    _global: &GlobalArgs,
    args: MountArgs,
    writer: Arc<OutputWriter>,
) -> ExitCode {
    let _ = writer.emit(JsonEventKind::Info {
        message: format!(
            "mount `{}`{} parsed; CLI follow-up will plumb \
             `copythat_mount::MountHandle` through a stable IPC.",
            args.mountpoint.display(),
            args.job
                .as_ref()
                .map(|j| format!(" (job={j})"))
                .unwrap_or_default(),
        ),
    });
    let _ = writer.human(&format!(
        "mount {}: scheduled — wiring lands in a follow-up phase.",
        args.mountpoint.display(),
    ));
    ExitCode::GenericError
}
