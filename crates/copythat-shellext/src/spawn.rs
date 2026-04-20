//! Argv composition + detached spawn helpers for the shell extension.
//!
//! The `Invoke` path of each `IExplorerCommand` wraps this module:
//! take the selected shell paths, build `copythat --enqueue <verb>
//! -- <paths…>`, and start the child in a new process group so
//! Explorer does not block on the app window.
//!
//! The argv composition is kept pure-Rust + host-testable so the
//! shape can be validated on every CI runner without needing a live
//! Windows session. The actual `CreateProcessW` call sits behind
//! `#[cfg(windows)]`.

use std::ffi::OsString;

use crate::consts::HOST_BIN;

/// Shell verb being invoked.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Verb {
    Copy,
    Move,
}

impl Verb {
    /// Wire name handed to `copythat --enqueue`.
    pub fn as_str(self) -> &'static str {
        match self {
            Verb::Copy => "copy",
            Verb::Move => "move",
        }
    }
}

/// Compose the argv for the shell-extension-triggered `copythat`
/// invocation.
///
/// Layout:
/// ```text
/// argv[0] = "copythat"
/// argv[1] = "--enqueue"
/// argv[2] = "copy" | "move"
/// argv[3] = "--"              (flag terminator — see below)
/// argv[4..] = <paths>
/// ```
///
/// The `--` terminator is deliberate: selected items in Explorer can
/// have names that begin with `--` (a user could rename a file to
/// `--help.txt`), and we do not want that to flip the Phase 7a
/// parser into thinking the name is a flag.
pub fn build_argv(verb: Verb, paths: &[OsString]) -> Vec<OsString> {
    let mut argv = Vec::with_capacity(paths.len() + 4);
    argv.push(OsString::from(HOST_BIN));
    argv.push(OsString::from("--enqueue"));
    argv.push(OsString::from(verb.as_str()));
    argv.push(OsString::from("--"));
    argv.extend(paths.iter().cloned());
    argv
}

/// Spawn `copythat` detached with the given verb + paths. The
/// child inherits nothing from Explorer: no stdin / stdout / stderr,
/// fresh console, fresh process group. Explorer's COM server thread
/// returns from `Invoke` immediately.
#[cfg(windows)]
pub fn spawn_detached(verb: Verb, paths: &[OsString]) -> std::io::Result<()> {
    use std::os::windows::process::CommandExt;
    use std::process::{Command, Stdio};

    // CREATE_NO_WINDOW suppresses the transient console; DETACHED_PROCESS
    // cuts the child from Explorer's console group. CREATE_NEW_PROCESS_GROUP
    // ensures a subsequent Ctrl-C on Explorer does not cascade.
    const CREATE_NO_WINDOW: u32 = 0x0800_0000;
    const DETACHED_PROCESS: u32 = 0x0000_0008;
    const CREATE_NEW_PROCESS_GROUP: u32 = 0x0000_0200;

    let argv = build_argv(verb, paths);
    // argv[0] is the binary; std::process::Command takes it separately.
    let mut cmd = Command::new(&argv[0]);
    cmd.args(&argv[1..]);
    cmd.stdin(Stdio::null());
    cmd.stdout(Stdio::null());
    cmd.stderr(Stdio::null());
    cmd.creation_flags(CREATE_NO_WINDOW | DETACHED_PROCESS | CREATE_NEW_PROCESS_GROUP);
    cmd.spawn()?;
    Ok(())
}

/// Non-Windows stub — the shell extension never actually runs off
/// Windows, but we keep the symbol so the `com` module can compile
/// against a single import in the CI test builds.
#[cfg(not(windows))]
pub fn spawn_detached(_verb: Verb, _paths: &[OsString]) -> std::io::Result<()> {
    Err(std::io::Error::new(
        std::io::ErrorKind::Unsupported,
        "copythat-shellext: spawn_detached is Windows-only",
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::OsString;

    fn os(s: &str) -> OsString {
        OsString::from(s)
    }

    #[test]
    fn build_argv_copy_wraps_paths_after_double_dash() {
        let paths = vec![os(r"C:\a\b.txt"), os(r"C:\c\d.txt")];
        let argv = build_argv(Verb::Copy, &paths);
        assert_eq!(
            argv,
            vec![
                os("copythat"),
                os("--enqueue"),
                os("copy"),
                os("--"),
                os(r"C:\a\b.txt"),
                os(r"C:\c\d.txt"),
            ]
        );
    }

    #[test]
    fn build_argv_move_uses_move_verb() {
        let paths = vec![os(r"C:\tmp")];
        let argv = build_argv(Verb::Move, &paths);
        assert_eq!(argv[2], os("move"));
    }

    #[test]
    fn build_argv_preserves_weird_names_under_double_dash() {
        let paths = vec![os(r"--weird.txt")];
        let argv = build_argv(Verb::Copy, &paths);
        // The `--` flag-terminator must come *before* the weird path.
        let dash_idx = argv.iter().position(|x| x == &os("--")).unwrap();
        let weird_idx = argv.iter().position(|x| x == &os("--weird.txt")).unwrap();
        assert!(dash_idx < weird_idx);
    }

    #[test]
    fn build_argv_empty_paths_still_valid_shape() {
        let argv = build_argv(Verb::Copy, &[]);
        assert_eq!(
            argv,
            vec![os("copythat"), os("--enqueue"), os("copy"), os("--")]
        );
    }

    #[test]
    fn verb_str_round_trip() {
        assert_eq!(Verb::Copy.as_str(), "copy");
        assert_eq!(Verb::Move.as_str(), "move");
    }
}
