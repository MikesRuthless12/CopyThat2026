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

use copythat_core::{CopyOptions, JobKind, Verifier};
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
pub fn enqueue_jobs(
    app: &AppHandle,
    state: &AppState,
    kind: JobKind,
    sources: Vec<PathBuf>,
    dst_root: &Path,
    copy_opts: CopyOptions,
    verifier: Option<Verifier>,
) -> Vec<u64> {
    let mut ids = Vec::with_capacity(sources.len());
    for src in sources {
        if src.as_os_str().is_empty() {
            continue;
        }
        let dst = destination_for(&src, dst_root);

        let (id, ctrl) = state.queue.add(kind, src.clone(), Some(dst.clone()));
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
        };
        tokio::spawn(async move {
            run_job(run).await;
        });
        ids.push(id.as_u64());
    }
    ids
}

/// Compose the destination path for one source. Each source lands
/// under `dst_root` with its own basename, so a multi-item enqueue
/// doesn't overwrite entries onto each other.
pub fn destination_for(src: &Path, dst_root: &Path) -> PathBuf {
    let name = src
        .file_name()
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("copy"));
    dst_root.join(name)
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

    if let Some(dst_root) = args.destination {
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
    fn destination_for_handles_trailing_slash_basename_gone() {
        // Path with no file-name (e.g. "/", ".") falls back to "copy".
        let dst = destination_for(Path::new("/"), Path::new("/dst"));
        assert_eq!(dst, PathBuf::from("/dst/copy"));
    }
}
