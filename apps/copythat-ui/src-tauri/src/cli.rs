//! Command-line interface for shell integration.
//!
//! Parses the argv handed to the app binary by shell-extension hosts
//! (Phase 7b Windows COM DLL, Phase 7c macOS Finder Sync Extension,
//! the Phase 7a Linux `.desktop` / ServiceMenu / UCA files). Two
//! entry paths:
//!
//! - No CLI args → normal GUI launch.
//! - `--enqueue <verb> <paths…> [--destination <dst>]` → route the
//!   paths into the job queue. When the app is already running, the
//!   `tauri-plugin-single-instance` plugin forwards argv to the live
//!   instance and this process exits; when it isn't, we parse on
//!   first boot and dispatch from `.setup()`.
//!
//! The CLI intentionally stays minimal and stable: shell extensions
//! are host-of-record for the argv, and changing the flag names means
//! re-shipping those extensions. New flags append, never replace.

use std::ffi::OsString;
use std::path::PathBuf;

/// What the app should do on launch, as derived from argv.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CliAction {
    /// Launch the GUI with no pre-queued work.
    Run,
    /// Dispatch a shell-integration enqueue request.
    Enqueue(EnqueueArgs),
    /// Print help and exit.
    PrintHelp,
    /// Print version and exit.
    PrintVersion,
}

/// Which shell verb was invoked.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnqueueVerb {
    Copy,
    Move,
}

impl EnqueueVerb {
    /// Stable wire name — mirrors `ipc::job_kind_name` so the
    /// frontend can branch on the same strings it already uses for
    /// `JobKind`.
    pub fn as_str(self) -> &'static str {
        match self {
            EnqueueVerb::Copy => "copy",
            EnqueueVerb::Move => "move",
        }
    }
}

/// Parsed `--enqueue` payload.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnqueueArgs {
    pub verb: EnqueueVerb,
    pub paths: Vec<PathBuf>,
    /// Optional. When present, the job runs non-interactively and
    /// skips the drop-staging dialog — this is the scripted-use path.
    /// When absent, the app emits a `shell-enqueue` event and the
    /// frontend reuses its drop-staging flow to pick a destination.
    pub destination: Option<PathBuf>,
}

/// Errors encountered while parsing CLI arguments. User-facing
/// messages land on stderr; they are intentionally not localised
/// (CLI output is a scripting / developer surface, not UI).
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum CliError {
    #[error("--enqueue requires a verb (copy or move)")]
    MissingVerb,
    #[error("unknown enqueue verb: {0}")]
    UnknownVerb(String),
    #[error("--enqueue requires at least one path")]
    NoPaths,
    #[error("unknown argument: {0}")]
    Unknown(String),
    #[error("--destination requires a path")]
    MissingDestination,
}

/// Short CLI help text printed by `--help` / `-h`. Not localised; see
/// [`CliError`] note above.
pub const HELP: &str = "\
Copy That 2026 — shell-integration CLI

Usage:
    copythat                                  Launch the GUI
    copythat --enqueue copy <paths…>          Queue a copy job per path
    copythat --enqueue move <paths…>          Queue a move job per path
                                              (optionally: --destination <dst>)
    copythat --help | -h
    copythat --version | -V
";

/// Current crate version. Reported by `--version`.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Parse argv into a [`CliAction`].
///
/// `argv` is the full process argv including `argv[0]`; the first
/// element is always skipped. Non-UTF-8 arguments are accepted (we
/// round-trip via `PathBuf`) but flag names must be valid UTF-8.
pub fn parse_args(argv: Vec<OsString>) -> Result<CliAction, CliError> {
    let mut iter = argv.into_iter();
    // Skip argv[0] — the binary path.
    let _ = iter.next();

    let mut enqueue: Option<EnqueueArgs> = None;
    let mut help = false;
    let mut version = false;

    while let Some(raw) = iter.next() {
        let Some(flag) = raw.to_str() else {
            // A non-UTF-8 argument outside of path position is an
            // error — all flag names are ASCII.
            return Err(CliError::Unknown(raw.to_string_lossy().into_owned()));
        };
        match flag {
            "--help" | "-h" => help = true,
            "--version" | "-V" => version = true,
            "--enqueue" => {
                let verb_raw = iter.next().ok_or(CliError::MissingVerb)?;
                let verb = match verb_raw.to_str() {
                    Some("copy") => EnqueueVerb::Copy,
                    Some("move") => EnqueueVerb::Move,
                    Some(other) => return Err(CliError::UnknownVerb(other.to_string())),
                    None => {
                        return Err(CliError::UnknownVerb(
                            verb_raw.to_string_lossy().into_owned(),
                        ));
                    }
                };
                let mut paths: Vec<PathBuf> = Vec::new();
                let mut destination: Option<PathBuf> = None;
                while let Some(next) = iter.next() {
                    // Only inspect strings when it *looks* like a
                    // flag; otherwise treat as a path to preserve
                    // non-UTF-8 filenames byte-for-byte.
                    if let Some(s) = next.to_str() {
                        match s {
                            "--destination" | "-d" => {
                                let dst = iter.next().ok_or(CliError::MissingDestination)?;
                                destination = Some(PathBuf::from(dst));
                                continue;
                            }
                            "--" => {
                                for rest in iter.by_ref() {
                                    paths.push(PathBuf::from(rest));
                                }
                                break;
                            }
                            other if other.starts_with("--") => {
                                return Err(CliError::Unknown(other.to_string()));
                            }
                            _ => {}
                        }
                    }
                    paths.push(PathBuf::from(next));
                }
                if paths.is_empty() {
                    return Err(CliError::NoPaths);
                }
                enqueue = Some(EnqueueArgs {
                    verb,
                    paths,
                    destination,
                });
            }
            other => return Err(CliError::Unknown(other.to_string())),
        }
    }

    if help {
        return Ok(CliAction::PrintHelp);
    }
    if version {
        return Ok(CliAction::PrintVersion);
    }
    if let Some(eq) = enqueue {
        return Ok(CliAction::Enqueue(eq));
    }
    Ok(CliAction::Run)
}

/// Convenience for tests and for `std::env::args_os()` at runtime.
pub fn parse_args_iter<I, S>(argv: I) -> Result<CliAction, CliError>
where
    I: IntoIterator<Item = S>,
    S: Into<OsString>,
{
    parse_args(argv.into_iter().map(Into::into).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args(v: &[&str]) -> Vec<OsString> {
        v.iter().map(|s| OsString::from(*s)).collect()
    }

    #[test]
    fn bare_launch_returns_run() {
        assert_eq!(parse_args(args(&["copythat"])).unwrap(), CliAction::Run);
    }

    #[test]
    fn help_flag() {
        assert_eq!(
            parse_args(args(&["copythat", "--help"])).unwrap(),
            CliAction::PrintHelp
        );
        assert_eq!(
            parse_args(args(&["copythat", "-h"])).unwrap(),
            CliAction::PrintHelp
        );
    }

    #[test]
    fn version_flag() {
        assert_eq!(
            parse_args(args(&["copythat", "--version"])).unwrap(),
            CliAction::PrintVersion
        );
    }

    #[test]
    fn enqueue_copy_single_path() {
        let a = parse_args(args(&["copythat", "--enqueue", "copy", "/src/a"])).unwrap();
        assert_eq!(
            a,
            CliAction::Enqueue(EnqueueArgs {
                verb: EnqueueVerb::Copy,
                paths: vec![PathBuf::from("/src/a")],
                destination: None,
            })
        );
    }

    #[test]
    fn enqueue_move_multiple_paths() {
        let a = parse_args(args(&["copythat", "--enqueue", "move", "/a", "/b", "/c"])).unwrap();
        let CliAction::Enqueue(eq) = a else {
            panic!("expected Enqueue");
        };
        assert_eq!(eq.verb, EnqueueVerb::Move);
        assert_eq!(eq.paths.len(), 3);
        assert_eq!(eq.paths[0], PathBuf::from("/a"));
        assert_eq!(eq.paths[2], PathBuf::from("/c"));
    }

    #[test]
    fn enqueue_with_destination_flag() {
        let a = parse_args(args(&[
            "copythat",
            "--enqueue",
            "copy",
            "/a",
            "--destination",
            "/dst",
            "/b",
        ]))
        .unwrap();
        let CliAction::Enqueue(eq) = a else {
            panic!();
        };
        assert_eq!(eq.paths, vec![PathBuf::from("/a"), PathBuf::from("/b")]);
        assert_eq!(eq.destination, Some(PathBuf::from("/dst")));
    }

    #[test]
    fn enqueue_double_dash_terminates_flags() {
        // After `--`, everything is a path even if it starts with --.
        let a = parse_args(args(&[
            "copythat",
            "--enqueue",
            "copy",
            "--",
            "--weird",
            "/b",
        ]))
        .unwrap();
        let CliAction::Enqueue(eq) = a else {
            panic!();
        };
        assert_eq!(
            eq.paths,
            vec![PathBuf::from("--weird"), PathBuf::from("/b")]
        );
    }

    #[test]
    fn missing_verb_errors() {
        assert_eq!(
            parse_args(args(&["copythat", "--enqueue"])).unwrap_err(),
            CliError::MissingVerb
        );
    }

    #[test]
    fn unknown_verb_errors() {
        assert_eq!(
            parse_args(args(&["copythat", "--enqueue", "nuke", "/a"])).unwrap_err(),
            CliError::UnknownVerb("nuke".to_string())
        );
    }

    #[test]
    fn no_paths_errors() {
        // `--` on its own with no paths after — but we swallow the --
        // before checking emptiness.
        assert_eq!(
            parse_args(args(&["copythat", "--enqueue", "copy"])).unwrap_err(),
            CliError::NoPaths
        );
    }

    #[test]
    fn missing_destination_value_errors() {
        assert_eq!(
            parse_args(args(&[
                "copythat",
                "--enqueue",
                "copy",
                "/a",
                "--destination"
            ]))
            .unwrap_err(),
            CliError::MissingDestination
        );
    }

    #[test]
    fn unknown_top_level_flag_errors() {
        assert_eq!(
            parse_args(args(&["copythat", "--nope"])).unwrap_err(),
            CliError::Unknown("--nope".to_string())
        );
    }

    #[test]
    fn verb_str_round_trips() {
        assert_eq!(EnqueueVerb::Copy.as_str(), "copy");
        assert_eq!(EnqueueVerb::Move.as_str(), "move");
    }
}
