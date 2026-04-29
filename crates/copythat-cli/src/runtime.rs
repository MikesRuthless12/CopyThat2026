//! Top-level dispatch — maps `Cli` → command module → `ExitCode`.
//!
//! Every subcommand runs inside a single tokio runtime built here. The
//! commands themselves use the shared [`crate::output::OutputWriter`]
//! to emit their human / JSON output.

use std::sync::Arc;

use crate::ExitCode;
use crate::cli::{Cli, Cmd};
use crate::commands;
use crate::output::{OutputMode, OutputWriter};

/// Build a single-threaded tokio runtime with the engine's default
/// IO + time features enabled.
///
/// Phase 43 — switched from `new_multi_thread().worker_threads(2)` to
/// `new_current_thread()`. The CLI is short-lived and copies a single
/// file (or one tree) per invocation; the engine's heavy work runs on
/// `spawn_blocking` threads from the dedicated blocking pool, which
/// is independent of the worker-thread count. Dropping the second
/// worker thread saves ~5–10 ms of startup per invocation — visible
/// on small-file copies where the CLI overhead dominates wall time.
/// `enable_all()` keeps `spawn_blocking`, timers, and async fs intact.
fn build_runtime() -> tokio::runtime::Runtime {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("tokio runtime build")
}

pub(crate) fn dispatch(cli: Cli) -> ExitCode {
    let mode = OutputMode::from_global(cli.global.json, cli.global.quiet);
    let writer = Arc::new(OutputWriter::stdout(mode));

    let rt = build_runtime();
    let _enter = rt.enter();
    rt.block_on(async move {
        match cli.command {
            Cmd::Copy(args) => commands::copy::run(&cli.global, args, writer.clone(), false).await,
            Cmd::Move(args) => commands::copy::run(&cli.global, args, writer.clone(), true).await,
            Cmd::Sync(args) => commands::sync::run(&cli.global, args, writer.clone()).await,
            Cmd::Shred(args) => commands::shred::run(&cli.global, args, writer.clone()).await,
            Cmd::Verify(args) => commands::verify::run(&cli.global, args, writer.clone()).await,
            Cmd::History(args) => commands::history::run(&cli.global, args, writer.clone()).await,
            Cmd::Stack(args) => commands::stack::run(&cli.global, args, writer.clone()).await,
            Cmd::Remote(args) => commands::remote::run(&cli.global, args, writer.clone()).await,
            Cmd::Mount(args) => commands::mount::run(&cli.global, args, writer.clone()).await,
            Cmd::Audit(args) => commands::audit::run(&cli.global, args, writer.clone()).await,
            Cmd::Plan(args) => commands::plan::run(&cli.global, args, writer.clone(), false).await,
            Cmd::Apply(args) => commands::plan::run(&cli.global, args, writer.clone(), true).await,
            Cmd::Schedule(args) => commands::schedule::run(&cli.global, args, writer.clone()).await,
            Cmd::Version(_) => commands::version::run(&cli.global, writer.clone()).await,
            Cmd::Config(args) => commands::config::run(&cli.global, args, writer.clone()).await,
            Cmd::Provenance(args) => {
                commands::provenance::run(&cli.global, args, writer.clone()).await
            }
            Cmd::Completions(args) => commands::completions::run(args).await,
        }
    })
}
