//! `copythat schedule --spec <PATH>` — Phase 14d.
//!
//! Reads a TOML jobspec carrying a `[schedule]` block, renders it
//! into the OS-native scheduler stanza via [`crate::schedule`], and
//! prints both the body and the suggested install path. This
//! command **does not write to disk** — installing the rendered
//! stanza is the user's deliberate next step (paste into
//! `schtasks /Create`, drop into `~/Library/LaunchAgents/`, or
//! `systemctl --user daemon-reload`).
//!
//! Exit codes:
//! - `0` — render succeeded.
//! - `2` — a Phase 17a path-safety guard rejected one of the
//!   paths embedded in the jobspec. The CLI emits an
//!   `error.path-rejected` event with the offending path.
//! - `9` — `[schedule]` block missing or unsupported, paths not
//!   absolute, or job kind not in `{copy, move, sync}`.

use std::sync::Arc;

use crate::ExitCode;
use crate::cli::{GlobalArgs, ScheduleArgs, ScheduleHostKind};
use crate::jobspec::JobSpec;
use crate::output::{JsonEventKind, OutputWriter};
use crate::schedule::{ScheduleError, ScheduleHostOs, render_schedule};

pub(crate) async fn run(
    _global: &GlobalArgs,
    args: ScheduleArgs,
    writer: Arc<OutputWriter>,
) -> ExitCode {
    let spec = match JobSpec::load(&args.spec) {
        Ok(s) => s,
        Err(e) => {
            let _ = writer.emit(JsonEventKind::Error {
                message: e.to_string(),
                code: ExitCode::ConfigInvalid.as_u8(),
            });
            return ExitCode::ConfigInvalid;
        }
    };

    let host = args
        .host
        .map(host_from_kind)
        .unwrap_or_else(default_host_os);

    let render = match render_schedule(&spec, &args.spec, host) {
        Ok(r) => r,
        Err(e) => {
            let exit = match &e {
                ScheduleError::PathRejected { .. } => ExitCode::PendingActions,
                _ => ExitCode::ConfigInvalid,
            };
            let _ = writer.emit(JsonEventKind::Error {
                message: e.to_string(),
                code: exit.as_u8(),
            });
            return exit;
        }
    };

    let _ = writer.emit(JsonEventKind::Info {
        message: format!(
            "Rendered {} stanza for schedule {} (suggested path: {})",
            host_label(render.host_os),
            render.schedule.label(),
            render.suggested_install_path.display(),
        ),
    });
    let _ = writer.human(&format!(
        "# Suggested install path: {}",
        render.suggested_install_path.display()
    ));
    let _ = writer.human(&render.body);

    ExitCode::Success
}

fn host_from_kind(kind: ScheduleHostKind) -> ScheduleHostOs {
    match kind {
        ScheduleHostKind::Windows => ScheduleHostOs::Windows,
        ScheduleHostKind::MacOs => ScheduleHostOs::MacOs,
        ScheduleHostKind::Linux => ScheduleHostOs::Linux,
    }
}

fn default_host_os() -> ScheduleHostOs {
    if cfg!(target_os = "windows") {
        ScheduleHostOs::Windows
    } else if cfg!(target_os = "macos") {
        ScheduleHostOs::MacOs
    } else {
        // Every other Unix lands on the systemd-user path. Users on
        // BSD / Solaris can override with `--host linux` and adapt
        // the OnCalendar= line by hand.
        ScheduleHostOs::Linux
    }
}

fn host_label(host: ScheduleHostOs) -> &'static str {
    match host {
        ScheduleHostOs::Windows => "Windows schtasks",
        ScheduleHostOs::MacOs => "macOS launchd",
        ScheduleHostOs::Linux => "Linux systemd --user",
    }
}
