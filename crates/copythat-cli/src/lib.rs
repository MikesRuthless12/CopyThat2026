//! `copythat-cli` — Phase 36 command-line surface for Copy That.
//!
//! Goal: a `copythat` binary suitable for CI/CD pipelines, automation
//! scripts, and headless servers. Stable JSON-Lines output (`--json`),
//! declarative TOML job specs with idempotent `plan` / `apply`
//! semantics, and a fixed set of nine documented exit codes that
//! callers can branch on without parsing `stderr`.
//!
//! # Subcommands
//!
//! | Command           | Status                                   |
//! | ----------------- | ---------------------------------------- |
//! | `copy`            | wired to `copythat_core::copy_file/copy_tree` |
//! | `move`            | wired to `copythat_core::move_file/move_tree` |
//! | `verify`          | wired to `copythat_hash::hash_file_async`     |
//! | `version`         | JSON-serialisable build metadata              |
//! | `config`          | `copythat_settings` get / set / reset         |
//! | `plan` / `apply`  | TOML jobspec → action list → idempotent run   |
//! | `history`         | `copythat_history` read-side reporter         |
//! | `completions`     | `clap_complete` shell-script emitter          |
//! | `sync` / `shred` / `stack` / `remote` / `mount` / `audit` | accept-and-stub  |
//!
//! Stubbed subcommands accept their flags so scripts written against
//! the v1 surface keep parsing, but they exit with a clearly-labelled
//! "feature staged for IPC; CLI follow-up" JSON event and exit code 1.
//! Each stub is one phase away from full wiring; the CLI surface is
//! frozen at Phase 36 so later phases only fill bodies in.
//!
//! # Exit codes (stable)
//!
//! See [`ExitCode`]. Documented in `--help` output and in the public
//! README so CI scripts can pin against numeric values.
//!
//! # JSON output schema
//!
//! Every line on `stdout` (when `--json` is set) is a single
//! JSON object with at minimum:
//!
//! ```json
//! {"ts":"2026-04-25T14:32:15Z","kind":"job.progress","job_id":"…",
//!  "bytes_done":12345,"bytes_total":98765,"rate_bps":1234567}
//! ```
//!
//! See [`output::JsonEvent`] for the full event taxonomy.

#![forbid(unsafe_code)]

use std::process::ExitCode as ProcessExitCode;

pub mod commands;
pub mod jobspec;
pub mod output;
pub mod schedule;
pub mod volume_watch;

mod cli;
mod runtime;

pub use cli::{Cli, Cmd, GlobalArgs};
pub use jobspec::{JobSpec, JobSpecError, PlanReport, plan_jobspec};
pub use output::{JsonEvent, JsonEventKind, OutputMode};

/// Stable exit codes documented in `--help`. CI scripts pin against
/// the numeric values; renaming variants is allowed but the discriminant
/// must not move.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ExitCode {
    Success = 0,
    GenericError = 1,
    PendingActions = 2,
    CollisionsUnresolved = 3,
    VerifyFailed = 4,
    NetworkUnreachable = 5,
    PermissionDenied = 6,
    DiskFull = 7,
    UserCanceled = 8,
    ConfigInvalid = 9,
}

impl ExitCode {
    /// Numeric byte the CLI returns to its parent process.
    pub fn as_u8(self) -> u8 {
        self as u8
    }

    /// `std::process::ExitCode` equivalent for `main()` return values.
    pub fn as_process(self) -> ProcessExitCode {
        ProcessExitCode::from(self as u8)
    }
}

/// Parse `std::env::args_os()` and dispatch. Used by the `copythat`
/// binary; tests use [`run_from`] with a custom argv vector.
pub fn run_from_argv() -> ProcessExitCode {
    let args: Vec<std::ffi::OsString> = std::env::args_os().collect();
    run_from(args).as_process()
}

/// Test-friendly entry point. Parses the supplied argv (including
/// `argv[0]`), dispatches, and returns the resolved [`ExitCode`].
///
/// Errors during parsing exit with [`ExitCode::ConfigInvalid`] and
/// print a clap-formatted error to `stderr`.
pub fn run_from(args: Vec<std::ffi::OsString>) -> ExitCode {
    use clap::Parser;

    let parsed = match Cli::try_parse_from(&args) {
        Ok(cli) => cli,
        Err(e) => {
            // `clap` already routes `--help` / `--version` to stdout
            // and treats them as success; everything else (including
            // unknown flags + missing args) exits ConfigInvalid so
            // callers can branch on a stable code.
            let _ = e.print();
            return match e.kind() {
                clap::error::ErrorKind::DisplayHelp
                | clap::error::ErrorKind::DisplayVersion
                | clap::error::ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand => {
                    ExitCode::Success
                }
                _ => ExitCode::ConfigInvalid,
            };
        }
    };

    runtime::dispatch(parsed)
}
