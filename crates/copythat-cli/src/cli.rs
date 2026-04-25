//! Clap-derived argument tree.
//!
//! Anything user-facing about the CLI surface lives here. The
//! subcommand variants are paired 1:1 with `commands::*` modules that
//! implement them.

use std::path::PathBuf;

use clap::{Args, Parser, Subcommand, ValueEnum};

/// `copythat <SUBCOMMAND>` — top-level entry point.
///
/// Documented exit codes (stable across releases):
///
/// | Code | Meaning                                  |
/// | ---- | ---------------------------------------- |
/// | 0    | success / no-op                          |
/// | 1    | generic error (see `stderr`)             |
/// | 2    | pending actions (`plan` only)            |
/// | 3    | collisions unresolved                    |
/// | 4    | verify failed                            |
/// | 5    | network unreachable                      |
/// | 6    | permission denied                        |
/// | 7    | disk full                                |
/// | 8    | user canceled                            |
/// | 9    | config invalid                           |
#[derive(Parser, Debug)]
#[command(
    name = "copythat",
    bin_name = "copythat",
    about = "Copy That CLI — byte-exact file copy, sync, verify and audit for CI/CD pipelines.",
    version,
    propagate_version = true,
    after_help = "Exit codes: 0 success, 1 error, 2 pending, 3 collision, 4 verify-fail, 5 net, 6 perm, 7 disk-full, 8 cancel, 9 config."
)]
pub struct Cli {
    #[command(flatten)]
    pub global: GlobalArgs,

    #[command(subcommand)]
    pub command: Cmd,
}

/// Global flags that apply to every subcommand. Threaded through
/// dispatch as a borrowed reference; nothing here mutates engine state
/// directly.
#[derive(Args, Debug, Clone)]
pub struct GlobalArgs {
    /// Emit JSON-Lines on `stdout` instead of human progress text.
    #[arg(long, global = true)]
    pub json: bool,

    /// Suppress progress; only the exit code is meaningful.
    #[arg(long, global = true, conflicts_with = "json")]
    pub quiet: bool,

    /// Disable ANSI colours regardless of `stdout` TTY detection.
    #[arg(long, global = true)]
    pub no_color: bool,

    /// Tracing verbosity for the bundled engine. `error` is quiet,
    /// `trace` is firehose-level.
    #[arg(long, global = true, value_enum, default_value_t = LogLevel::Warn)]
    pub log_level: LogLevel,

    /// Path to a `copythat.toml` config override. Falls back to
    /// `<config-dir>/CopyThat/copythat.toml` when omitted.
    #[arg(long, global = true, value_name = "PATH")]
    pub config: Option<PathBuf>,

    /// Profile name for routing settings (per-machine vs per-user).
    /// Reserved for the Phase 12 settings router; today this only
    /// affects the in-memory copy of `Settings::profile`.
    #[arg(long, global = true, value_name = "NAME")]
    pub profile: Option<String>,
}

/// Trace level passed via `--log-level`.
#[derive(Debug, Clone, Copy, ValueEnum, Default)]
pub enum LogLevel {
    Error,
    #[default]
    Warn,
    Info,
    Debug,
    Trace,
}

/// All `copythat <…>` subcommands.
#[derive(Subcommand, Debug)]
pub enum Cmd {
    /// Copy one or more sources to a destination.
    Copy(CopyArgs),
    /// Move (rename across volumes) one or more sources to a destination.
    Move(CopyArgs),
    /// Sync two trees (default mode = two-way conflict-aware merge).
    Sync(SyncArgs),
    /// Securely erase a file or tree.
    Shred(ShredArgs),
    /// Hash a file with the chosen algorithm; optionally compare against a sidecar.
    Verify(VerifyArgs),
    /// Print prior-job history from the local SQLite store.
    History(HistoryArgs),
    /// Drop-Stack management (Phase 28).
    Stack(StackArgs),
    /// Cloud-remote management (Phase 32).
    Remote(RemoteArgs),
    /// Mount a virtual filesystem view of an in-flight job (Phase 33).
    Mount(MountArgs),
    /// Audit-log inspection (Phase 34).
    Audit(AuditArgs),
    /// Read a TOML jobspec and report the actions a matching `apply` would take.
    Plan(PlanArgs),
    /// Read a TOML jobspec and execute it. Idempotent — re-runs are no-ops.
    Apply(PlanArgs),
    /// Phase 14d — render a per-OS scheduler stanza from a jobspec
    /// containing a `[schedule]` block. Prints the stanza + the
    /// suggested install path; the user pastes it into their
    /// schtasks / launchd / systemd config.
    Schedule(ScheduleArgs),
    /// Print version metadata.
    Version(VersionArgs),
    /// Get / set / reset values in the persistent settings file.
    Config(ConfigArgs),
    /// Emit a shell-completion script for bash / zsh / fish / pwsh.
    Completions(CompletionsArgs),
}

/// `copy` and `move` share a flag surface — same `CopyArgs` struct.
#[derive(Args, Debug, Clone)]
pub struct CopyArgs {
    /// One or more source paths. Last positional argument is the destination.
    #[arg(value_name = "PATHS", required = true, num_args = 2..)]
    pub paths: Vec<PathBuf>,

    /// Hash algorithm for post-copy verification. Off by default.
    #[arg(long, value_name = "ALGO")]
    pub verify: Option<String>,

    /// Bandwidth shape, e.g. `10MB/s`. Off by default.
    #[arg(long, value_name = "RATE")]
    pub shape: Option<String>,

    /// If true, refuse to overwrite an existing destination file.
    #[arg(long)]
    pub fail_if_exists: bool,

    /// Follow symlinks instead of cloning them.
    #[arg(long)]
    pub follow_symlinks: bool,
}

#[derive(Args, Debug, Clone)]
pub struct SyncArgs {
    pub left: PathBuf,
    pub right: PathBuf,
    /// Sync mode. Defaults to `two-way`.
    #[arg(long, value_name = "MODE", default_value = "two-way")]
    pub mode: String,
}

#[derive(Args, Debug, Clone)]
pub struct ShredArgs {
    pub path: PathBuf,
    /// Pattern method. Defaults to a single random pass.
    #[arg(long, value_name = "METHOD", default_value = "random-1")]
    pub method: String,
}

#[derive(Args, Debug, Clone)]
pub struct VerifyArgs {
    pub path: PathBuf,
    /// Hash algorithm name (e.g. `blake3`, `sha256`).
    #[arg(long, value_name = "ALGO", default_value = "blake3")]
    pub algo: String,
    /// Optional sidecar file with the expected digest.
    #[arg(long, value_name = "SIDECAR")]
    pub against: Option<PathBuf>,
}

#[derive(Args, Debug, Clone)]
pub struct HistoryArgs {
    /// Substring filter on the job's source path.
    #[arg(long)]
    pub filter: Option<String>,
    /// Maximum rows to emit.
    #[arg(long, default_value_t = 50)]
    pub limit: u32,
}

#[derive(Subcommand, Debug, Clone)]
pub enum StackOp {
    /// Append a path to the current Drop Stack.
    Add { path: PathBuf },
    /// List the current Drop Stack contents.
    List,
    /// Empty the Drop Stack.
    Clear,
    /// Copy every Drop Stack entry to a destination root.
    CopyTo { dst: PathBuf },
}

#[derive(Args, Debug, Clone)]
pub struct StackArgs {
    #[command(subcommand)]
    pub op: StackOp,
}

#[derive(Subcommand, Debug, Clone)]
pub enum RemoteOp {
    /// Register a new remote.
    Add { name: String, url: String },
    /// List configured remotes.
    List,
    /// Remove a configured remote.
    Remove { name: String },
    /// Test connectivity to a remote.
    Test { name: String },
}

#[derive(Args, Debug, Clone)]
pub struct RemoteArgs {
    #[command(subcommand)]
    pub op: RemoteOp,
}

#[derive(Args, Debug, Clone)]
pub struct MountArgs {
    pub mountpoint: PathBuf,
    /// Optional job UUID to mount. Defaults to the most recent job.
    #[arg(long)]
    pub job: Option<String>,
}

#[derive(Subcommand, Debug, Clone)]
pub enum AuditOp {
    /// Verify the BLAKE3 chain of an audit log file.
    Verify { log_file: PathBuf },
}

#[derive(Args, Debug, Clone)]
pub struct AuditArgs {
    #[command(subcommand)]
    pub op: AuditOp,
}

#[derive(Args, Debug, Clone)]
pub struct PlanArgs {
    /// Path to the jobspec TOML file.
    #[arg(long, value_name = "PATH", required = true)]
    pub spec: PathBuf,
}

/// `copythat schedule --spec <PATH> [--host <linux|macos|windows>]`.
///
/// Defaults `--host` to the current host; the override exists so a
/// user on Linux can still render a Windows schtasks stanza for an
/// offline NAS, etc.
#[derive(Args, Debug, Clone)]
pub struct ScheduleArgs {
    /// Path to the jobspec TOML file. Must declare a `[schedule]`
    /// block; sources + destination must be absolute.
    #[arg(long, value_name = "PATH", required = true)]
    pub spec: PathBuf,

    /// Override the host OS the stanza is rendered for. Defaults to
    /// the current platform.
    #[arg(long, value_enum, value_name = "OS")]
    pub host: Option<ScheduleHostKind>,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum ScheduleHostKind {
    Windows,
    MacOs,
    Linux,
}

#[derive(Args, Debug, Clone)]
pub struct VersionArgs {}

#[derive(Subcommand, Debug, Clone)]
pub enum ConfigOp {
    /// Print a single config key (dotted path, e.g. `transfer.buffer_size`).
    Get { key: String },
    /// Set a config key. Value is parsed as TOML scalar.
    Set { key: String, value: String },
    /// Reset a key to its default. Pass `--all` to reset everything.
    Reset {
        #[arg(long)]
        all: bool,
        key: Option<String>,
    },
}

#[derive(Args, Debug, Clone)]
pub struct ConfigArgs {
    #[command(subcommand)]
    pub op: ConfigOp,
}

#[derive(Args, Debug, Clone)]
pub struct CompletionsArgs {
    /// Target shell.
    #[arg(value_enum)]
    pub shell: ShellKind,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum ShellKind {
    Bash,
    Zsh,
    Fish,
    /// PowerShell (Windows + cross-platform pwsh).
    PowerShell,
    /// Elvish — included because clap_complete supports it.
    Elvish,
}

impl ShellKind {
    pub(crate) fn as_clap(self) -> clap_complete::Shell {
        match self {
            ShellKind::Bash => clap_complete::Shell::Bash,
            ShellKind::Zsh => clap_complete::Shell::Zsh,
            ShellKind::Fish => clap_complete::Shell::Fish,
            ShellKind::PowerShell => clap_complete::Shell::PowerShell,
            ShellKind::Elvish => clap_complete::Shell::Elvish,
        }
    }
}
