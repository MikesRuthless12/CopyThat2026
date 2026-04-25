//! Subcommand modules. Each one exposes a single `pub(crate) async fn
//! run(...)` that returns the resolved `ExitCode`.

pub(crate) mod audit;
pub(crate) mod completions;
pub(crate) mod config;
pub(crate) mod copy;
pub(crate) mod history;
pub(crate) mod mount;
pub(crate) mod plan;
pub(crate) mod remote;
pub(crate) mod schedule;
pub(crate) mod shred;
pub(crate) mod stack;
pub(crate) mod sync;
pub(crate) mod verify;
pub(crate) mod version;
