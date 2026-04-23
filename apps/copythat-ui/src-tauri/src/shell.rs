//! Shell-integration bridge.
//!
//! The CLI path (`copythat --enqueue <verb> <paths…>`) and the
//! IPC path (the `start_copy` / `start_move` commands invoked by the
//! Svelte frontend) share this module's [`enqueue_jobs`] helper so
//! they land identical jobs in the queue — same ids, same progress
//! events, same terminal states.
//!
//! [`dispatch_cli_action`] is the entry point [`crate::lib::run`] and
//! the single-instance plugin callback both use to route an already-
//! parsed [`cli::CliAction`] into the running app.

use std::path::{Path, PathBuf};

use copythat_core::{
    CollisionPolicy, CopyOptions, ErrorPolicy, FilterSet, JobKind, Verifier,
    validate_path_no_traversal,
};
use tauri::{AppHandle, Emitter, Manager};

use crate::cli::{CliAction, EnqueueArgs, EnqueueVerb};
use crate::ipc::{EVENT_SHELL_ENQUEUE, JobDto, ShellEnqueueDto};
use crate::runner::{RunJob, emit_job_added, run_job};
use crate::state::AppState;

/// Shared enqueue body. Used by both the Tauri command layer and the
/// CLI dispatcher; the two call sites funnel through here so the
/// queue only learns about a job one way.
///
/// Returns the list of newly allocated job ids. The caller is
/// responsible for reporting them upstream (HTTP-style for commands,
/// log-only for CLI).
#[allow(clippy::too_many_arguments)]
pub fn enqueue_jobs(
    app: &AppHandle,
    state: &AppState,
    kind: JobKind,
    sources: Vec<PathBuf>,
    dst_root: &Path,
    copy_opts: CopyOptions,
    verifier: Option<Verifier>,
    collision_policy: CollisionPolicy,
    error_policy: ErrorPolicy,
    tree_concurrency: Option<usize>,
    filters: Option<FilterSet>,
) -> Vec<u64> {
    let mut ids = Vec::with_capacity(sources.len());
    for src in sources {
        if src.as_os_str().is_empty() {
            continue;
        }
        let dst = destination_for(&src, dst_root);

        let (id, ctrl) = state.queue.add(kind, src.clone(), Some(dst.clone()));
        // Phase 22 — seed per-job collision rules from the user's
        // currently-active ConflictProfile. Later appends (via the
        // `add_conflict_rule` IPC) extend this list; the runner
        // consults it before raising any collision prompt.
        {
            let live = state.settings_snapshot();
            if let Some(profile) = live.conflict_profiles.active_profile() {
                state.collisions.seed_rules(id.as_u64(), profile.clone());
            }
        }
        let snapshot = state
            .queue
            .get(id)
            .expect("just-added job must be in queue");
        emit_job_added(app, JobDto::from_job(&snapshot));

        let run = RunJob {
            app: app.clone(),
            state: state.clone(),
            id,
            kind,
            src,
            dst: Some(dst),
            ctrl,
            verifier: verifier.clone(),
            copy_opts: copy_opts.clone(),
            collision_policy: collision_policy.clone(),
            error_policy,
            tree_concurrency,
            filters: filters.clone(),
        };
        // `tauri::async_runtime::spawn` uses the runtime Tauri itself
        // manages, so this call site works from both the #[tauri::command]
        // path (which is already inside a tokio context) and the CLI
        // setup-hook path (which is not). A bare `tokio::spawn` panics
        // with "no reactor running" when invoked from setup.
        tauri::async_runtime::spawn(async move {
            run_job(run).await;
        });
        ids.push(id.as_u64());
    }
    ids
}

/// Compose the destination path for one source. Each source lands
/// under `dst_root` with its own basename, so a multi-item enqueue
/// doesn't overwrite entries onto each other.
///
/// Drive roots (`C:\`, `D:\`, …) have no `file_name()`, so we fall
/// back to the drive letter itself. That way copying the whole of
/// `C:\` into `D:\Dest\` lands as `D:\Dest\C\` rather than silently
/// dumping the drive's contents into `D:\Dest\` and colliding with
/// anything already there. Non-Windows roots (`/`) fall back to
/// `"root"`.
pub fn destination_for(src: &Path, dst_root: &Path) -> PathBuf {
    if let Some(name) = src.file_name() {
        return dst_root.join(name);
    }
    let fallback = drive_letter_folder(src).unwrap_or_else(|| "root".to_string());
    dst_root.join(fallback)
}

/// Extract `"C"` from `C:\`, `"D"` from `D:\`, `"Photos"` from
/// `\\fileserver\Photos\`, etc. Returns `None` when the path
/// doesn't start with any recognisable prefix (Unix `/` and bare
/// relative paths take the `"root"` fallback above).
fn drive_letter_folder(src: &Path) -> Option<String> {
    use std::path::Component;
    let first = src.components().next()?;
    let Component::Prefix(prefix) = first else {
        return None;
    };
    #[cfg(windows)]
    {
        use std::path::Prefix;
        match prefix.kind() {
            Prefix::Disk(letter) | Prefix::VerbatimDisk(letter) => {
                Some((letter as char).to_string())
            }
            Prefix::UNC(_, share) | Prefix::VerbatimUNC(_, share) => {
                let s = share.to_string_lossy().into_owned();
                if s.is_empty() { None } else { Some(s) }
            }
            _ => None,
        }
    }
    #[cfg(not(windows))]
    {
        let raw = prefix.as_os_str().to_string_lossy();
        let cleaned = raw.trim_end_matches(':').to_string();
        if cleaned.is_empty() {
            None
        } else {
            Some(cleaned)
        }
    }
}

/// Dispatch a parsed CLI action against a running app.
///
/// Called twice in the app lifecycle:
/// - from the setup hook of the first (real) launch, when the user
///   invoked the binary with `--enqueue`;
/// - from the `tauri-plugin-single-instance` callback, when a second
///   invocation came in while the first was still running.
///
/// Behaviour:
/// - `Run` → bring the main window to front.
/// - `PrintHelp` / `PrintVersion` → echo to stdout (the primary
///   already handled these before Tauri started, but a second
///   invocation might still be routed here).
/// - `Enqueue { destination: Some(_) }` → push jobs directly to the
///   queue.
/// - `Enqueue { destination: None }` → emit a `shell-enqueue` event
///   for the frontend's drop-staging dialog.
pub fn dispatch_cli_action(app: &AppHandle, action: CliAction) {
    bring_to_front(app);
    match action {
        CliAction::Run => {}
        CliAction::PrintHelp => println!("{}", crate::cli::HELP),
        CliAction::PrintVersion => println!("copythat {}", crate::cli::VERSION),
        CliAction::Enqueue(args) => dispatch_enqueue(app, args),
    }
}

fn dispatch_enqueue(app: &AppHandle, args: EnqueueArgs) {
    let kind = match args.verb {
        EnqueueVerb::Copy => JobKind::Copy,
        EnqueueVerb::Move => JobKind::Move,
    };

    // Phase 17a — the CLI path bypasses `start_copy` / `start_move`,
    // so we run the same lexical safety guard here. A shell-extension
    // COM DLL or scripted pipeline is just as valid an attacker as
    // the webview; rejecting before `enqueue_jobs` runs keeps the
    // job out of the history log + queue entirely.
    for p in &args.paths {
        if let Err(e) = validate_path_no_traversal(p) {
            eprintln!("[cli enqueue] rejected source `{}`: {e}", p.display());
            return;
        }
    }

    if let Some(dst_root) = args.destination {
        if let Err(e) = validate_path_no_traversal(&dst_root) {
            eprintln!(
                "[cli enqueue] rejected destination `{}`: {e}",
                dst_root.display()
            );
            return;
        }
        // Non-interactive: skip the staging dialog and drop straight
        // into the queue. Used by scripted pipelines and tests.
        let state = app.state::<AppState>().inner().clone();
        let _ = enqueue_jobs(
            app,
            &state,
            kind,
            args.paths,
            &dst_root,
            CopyOptions::default(),
            None,
            // Scripted enqueue inherits the engine default (Skip on
            // collision, Abort on error) to stay deterministic; an
            // interactive caller overrides via the commands layer.
            CollisionPolicy::default(),
            ErrorPolicy::default(),
            // Phase 13c — scripted enqueue uses the engine default
            // concurrency; the interactive commands layer threads in
            // `Settings.transfer.concurrency` via `resolve_concurrency`.
            None,
            // Phase 14a — scripted / CLI enqueue does NOT apply the
            // persisted filter set. The CLI is meant for "copy this
            // exact list of paths" and a surprise `skip-hidden`
            // leaking from Settings would silently drop files the
            // user explicitly requested. The interactive commands
            // layer applies filters instead.
            None,
        );
    } else {
        // Interactive: hand the paths to the frontend; it reuses the
        // drop-staging modal to pick a destination, then calls back
        // through `start_copy` / `start_move`.
        let dto = ShellEnqueueDto {
            verb: args.verb.as_str(),
            paths: args
                .paths
                .iter()
                .map(|p| p.to_string_lossy().into_owned())
                .collect(),
        };
        let _ = app.emit(EVENT_SHELL_ENQUEUE, dto);
    }
}

fn bring_to_front(app: &AppHandle) {
    if let Some(win) = app.get_webview_window("main") {
        let _ = win.show();
        let _ = win.unminimize();
        let _ = win.set_focus();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn destination_for_appends_basename() {
        let dst = destination_for(Path::new("/a/b/c.txt"), Path::new("/dst"));
        assert_eq!(dst, PathBuf::from("/dst/c.txt"));
    }

    #[test]
    fn destination_for_handles_trailing_slash_unix_root() {
        // Unix `/` has no `file_name()` and no drive prefix, so the
        // fallback is the "root" sentinel.
        let dst = destination_for(Path::new("/"), Path::new("/dst"));
        assert_eq!(dst, PathBuf::from("/dst/root"));
    }

    #[cfg(windows)]
    #[test]
    fn destination_for_windows_drive_root_uses_letter() {
        let dst = destination_for(Path::new(r"C:\"), Path::new(r"D:\Dest"));
        assert_eq!(dst, PathBuf::from(r"D:\Dest\C"));
    }

    #[cfg(windows)]
    #[test]
    fn destination_for_windows_drive_subfolder_keeps_basename() {
        // A non-root path under a drive still gets its file name as
        // usual — `C:\Music` → `D:\Dest\Music`.
        let dst = destination_for(Path::new(r"C:\Music"), Path::new(r"D:\Dest"));
        assert_eq!(dst, PathBuf::from(r"D:\Dest\Music"));
    }

    #[cfg(windows)]
    #[test]
    fn destination_for_windows_unc_share_root_uses_share_name() {
        // `\\server\Photos\` has no `file_name()`; fall back to the
        // share name so copying a whole SMB share lands as
        // `D:\Dest\Photos\` rather than merging into `D:\Dest\`.
        let dst = destination_for(Path::new(r"\\fileserver\Photos\"), Path::new(r"D:\Dest"));
        assert_eq!(dst, PathBuf::from(r"D:\Dest\Photos"));
    }

    #[cfg(windows)]
    #[test]
    fn destination_for_windows_unc_file_keeps_basename() {
        // `\\server\Photos\vacation\IMG_001.jpg` → `D:\Dest\IMG_001.jpg`.
        let src = Path::new(r"\\fileserver\Photos\vacation\IMG_001.jpg");
        let dst = destination_for(src, Path::new(r"D:\Dest"));
        assert_eq!(dst, PathBuf::from(r"D:\Dest\IMG_001.jpg"));
    }
}
