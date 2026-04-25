//! `copythat completions <SHELL>` — emits the per-shell completion
//! script via `clap_complete::generate`.
//!
//! The user redirects stdout into the shell's completion location;
//! the CLI itself never writes to disk. Always exits Success.

use std::io;

use clap::CommandFactory;

use crate::ExitCode;
use crate::cli::{Cli, CompletionsArgs};

pub(crate) async fn run(args: CompletionsArgs) -> ExitCode {
    let mut cmd = Cli::command();
    let bin = cmd.get_name().to_string();
    clap_complete::generate(args.shell.as_clap(), &mut cmd, bin, &mut io::stdout());
    ExitCode::Success
}
