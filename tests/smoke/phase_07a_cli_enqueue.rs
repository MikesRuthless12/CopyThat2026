//! Phase 7a smoke test.
//!
//! Exercises the shell-integration cross-platform plumbing without
//! booting the webview: the CLI parser, the parsed-action shape, and
//! the path the shell dispatcher walks to put paths into the queue.
//!
//! Layering:
//! 1. Parse a realistic argv (same shape shell extensions will emit)
//!    and assert the resulting `CliAction::Enqueue` carries the right
//!    verb, paths, and destination.
//! 2. Walk the `shell::destination_for` helper with every source —
//!    shell extensions may pass multiple paths and each has to land
//!    under the chosen root with its own basename.
//! 3. Drive a real `copy_file` against that destination, mirroring
//!    what the live app's runner does, and assert the destination
//!    file shows up byte-exact within two seconds.
//!
//! Tauri is not required: we call `copythat_ui_lib::cli` and
//! `copythat_ui_lib::shell::destination_for` directly and use
//! `copythat_core::copy_file` to drive the copy. The integration
//! tests in `apps/copythat-ui/src-tauri/tests/command_layer.rs` use
//! the same approach — the shell layer only adds AppHandle emits on
//! top, which is covered by the unit tests in `cli.rs` and `shell.rs`.

use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use copythat_core::{CopyOptions, JobKind, Queue, copy_file};
use copythat_ui_lib::cli::{self, CliAction, EnqueueVerb};
use copythat_ui_lib::shell::destination_for;
use tempfile::tempdir;
use tokio::sync::mpsc;

/// Argv parity check. A shell extension invokes the binary with
/// exactly this shape; if parsing fails or returns the wrong variant,
/// nothing downstream will queue a job.
#[test]
fn parses_copy_enqueue_argv() {
    let argv = os(&[
        "copythat",
        "--enqueue",
        "copy",
        "/source/a.bin",
        "/source/b.bin",
    ]);
    let action = cli::parse_args(argv).expect("parse");
    let CliAction::Enqueue(eq) = action else {
        panic!("expected Enqueue variant, got {action:?}");
    };
    assert_eq!(eq.verb, EnqueueVerb::Copy);
    assert_eq!(eq.paths.len(), 2);
    assert!(eq.destination.is_none());
}

/// Move verb exposes the same shape as copy — the dispatcher only
/// branches on this when translating to `JobKind`.
#[test]
fn parses_move_enqueue_with_destination() {
    let argv = os(&[
        "copythat",
        "--enqueue",
        "move",
        "/source/c.bin",
        "--destination",
        "/dst",
    ]);
    let CliAction::Enqueue(eq) = cli::parse_args(argv).expect("parse") else {
        panic!();
    };
    assert_eq!(eq.verb, EnqueueVerb::Move);
    assert_eq!(eq.paths, vec![PathBuf::from("/source/c.bin")]);
    assert_eq!(eq.destination, Some(PathBuf::from("/dst")));
}

/// `--` terminates flag parsing — important because shell extensions
/// on Linux pass paths via `%F` which may include names beginning
/// with `--`.
#[test]
fn double_dash_preserves_paths_that_look_like_flags() {
    let argv = os(&[
        "copythat",
        "--enqueue",
        "copy",
        "--",
        "--weird.bin",
        "/ok.bin",
    ]);
    let CliAction::Enqueue(eq) = cli::parse_args(argv).expect("parse") else {
        panic!();
    };
    assert_eq!(
        eq.paths,
        vec![PathBuf::from("--weird.bin"), PathBuf::from("/ok.bin")]
    );
}

/// Help + version exit paths. If these stop working, every CLI user
/// gets a stack trace instead of a usage line.
#[test]
fn help_and_version_recognised() {
    assert_eq!(
        cli::parse_args(os(&["copythat", "--help"])).unwrap(),
        CliAction::PrintHelp
    );
    assert_eq!(
        cli::parse_args(os(&["copythat", "--version"])).unwrap(),
        CliAction::PrintVersion
    );
}

/// End-to-end: walk the same code path `shell::dispatch_cli_action`
/// takes after dispatching a parsed enqueue — compute the per-source
/// destination and execute the copy. Asserts the file landed within
/// the two-second budget the phase spec calls for.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn enqueue_with_destination_drives_a_real_copy() {
    let tmp = tempdir().expect("tempdir");
    let src = tmp.path().join("payload.bin");
    let dst_root = tmp.path().join("out");
    std::fs::create_dir_all(&dst_root).unwrap();

    // 4 KiB is enough to prove the copy pipeline executed without
    // slowing CI; Phase 1 already covers the large-file path.
    let payload: Vec<u8> = (0..4096).map(|i| (i & 0xff) as u8).collect();
    std::fs::write(&src, &payload).unwrap();

    // Parse the exact argv shell extensions will generate.
    let argv = vec![
        OsString::from("copythat"),
        OsString::from("--enqueue"),
        OsString::from("copy"),
        OsString::from(&src),
        OsString::from("--destination"),
        OsString::from(&dst_root),
    ];
    let action = cli::parse_args(argv).expect("parse");
    let CliAction::Enqueue(eq) = action else {
        panic!("expected Enqueue");
    };
    assert_eq!(eq.paths.len(), 1);
    let dst_root = eq.destination.as_deref().expect("destination");

    // Mirror the dispatcher's per-source destination rule and drive
    // one copy through `copy_file` + a real Queue. A live app would
    // spawn `runner::run_job`; we call the engine directly since the
    // runner requires a Tauri AppHandle.
    let src_path = eq.paths[0].clone();
    let dst_path = destination_for(&src_path, dst_root);

    let q = Queue::new();
    let (id, ctrl) = q.add(JobKind::Copy, src_path.clone(), Some(dst_path.clone()));
    q.start(id);

    let (tx, mut rx) = mpsc::channel(64);
    // Drop events — the smoke test cares about the bytes landing, not
    // the event stream (covered by the Phase 1 + 5 tests already).
    tokio::spawn(async move { while rx.recv().await.is_some() {} });

    let opts = CopyOptions::default();
    let deadline = Instant::now() + Duration::from_secs(2);
    let report = tokio::time::timeout_at(deadline.into(), async move {
        copy_file(&src_path, &dst_path, opts, ctrl, tx).await
    })
    .await
    .expect("copy did not complete within 2 s")
    .expect("copy returned error");

    q.mark_completed(id);

    assert_eq!(report.bytes as usize, payload.len());
    assert!(dst_path_exists(&dst_root.join("payload.bin")));
    let round_trip = std::fs::read(dst_root.join("payload.bin")).unwrap();
    assert_eq!(round_trip, payload);
}

fn dst_path_exists(p: &Path) -> bool {
    p.exists()
}

fn os(argv: &[&str]) -> Vec<OsString> {
    argv.iter().map(|s| OsString::from(*s)).collect()
}
