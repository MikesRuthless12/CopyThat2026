//! Phase 8 smoke test — error handling + collision UX.
//!
//! Exercises the engine's per-file error policy + the UI-layer
//! registry without booting Tauri. We drive `copy_tree` with a
//! pre-seeded destination whose files are read-only, then consume
//! the event stream in a thin "UI simulator" loop that mirrors
//! `runner::forward_events`:
//!
//! - On `CopyEvent::ErrorPrompt`, register the prompt in
//!   `ErrorRegistry` and resolve with `ErrorAction::Skip` +
//!   apply-to-all. This is exactly what the frontend's
//!   `ErrorModal` does when a user clicks "Skip all errors of
//!   this kind" on the first prompt.
//! - On `CopyEvent::FileError`, log via `ErrorRegistry::log_auto`
//!   so the subsequent auto-skipped files still land in the log.
//!
//! Asserts:
//!
//! 1. Every non-read-only file landed byte-exact at the destination.
//! 2. The three read-only files appear in the error log with the
//!    `permission-denied` kind.
//! 3. `TreeReport::errored == 3` and the three writable files succeeded.
//! 4. CSV + TXT exports each contain all three entries.
//!
//! Skipped on Windows because the read-only-file trick used to
//! provoke per-file permission errors doesn't work the same way —
//! Windows's readonly attribute on a file doesn't prevent a
//! same-user open-for-write, so the engine never sees the error
//! this test expects. Linux / macOS CI runners carry the smoke
//! test; Windows compiles the test (it's gated at runtime, not
//! `cfg`) so the matrix stays consistent.

use std::path::{Path, PathBuf};

use copythat_core::{
    CollisionPolicy, CopyControl, CopyEvent, CopyOptions, ErrorAction, ErrorPolicy, TreeOptions,
    copy_tree,
};
use copythat_ui_lib::errors::{ErrorRegistry, export_csv, export_txt};
use tempfile::tempdir;
use tokio::sync::mpsc;

#[cfg(unix)]
fn make_readonly(path: &Path) -> std::io::Result<()> {
    use std::os::unix::fs::PermissionsExt;
    let meta = std::fs::metadata(path)?;
    let mut perm = meta.permissions();
    perm.set_mode(0o444);
    std::fs::set_permissions(path, perm)
}

#[cfg(windows)]
#[allow(dead_code)]
fn make_readonly(_path: &Path) -> std::io::Result<()> {
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn skip_all_of_kind_lets_tree_finish_and_logs_three_errors() {
    if cfg!(windows) {
        eprintln!(
            "phase_08_errors: Windows readonly-file semantics don't trigger the permission-denied error this test needs; skipping on this host."
        );
        return;
    }

    let tmp = tempdir().expect("tempdir");
    let src_dir = tmp.path().join("src");
    let dst_dir = tmp.path().join("dst");
    std::fs::create_dir_all(&src_dir).unwrap();
    std::fs::create_dir_all(&dst_dir).unwrap();

    let readonly_names = ["ro-a.bin", "ro-b.bin", "ro-c.bin"];
    let writable_names = ["w-a.bin", "w-b.bin", "w-c.bin"];
    for name in readonly_names.iter().chain(writable_names.iter()) {
        std::fs::write(src_dir.join(name), format!("body-{name}")).unwrap();
    }
    for name in readonly_names {
        let dst_file = dst_dir.join(name);
        std::fs::write(&dst_file, b"pre-existing").unwrap();
        make_readonly(&dst_file).unwrap();
    }

    let tree_opts = TreeOptions {
        file: CopyOptions::default(),
        // Overwrite is the interesting policy — it tries to truncate
        // the pre-existing read-only file, which is where the
        // permission error fires.
        collision: CollisionPolicy::Overwrite,
        on_error: ErrorPolicy::Ask,
        ..TreeOptions::default()
    };

    let (tx, mut rx) = mpsc::channel::<CopyEvent>(256);
    let ctrl = CopyControl::new();
    let registry = ErrorRegistry::new();

    // UI simulator: runs concurrently with `copy_tree`, draining
    // events off the channel. For each prompt, Skip + apply-to-all
    // on the first one caches the decision; every subsequent
    // permission-denied is auto-resolved from the cache.
    let registry_for_ui = registry.clone();
    let ui = tokio::spawn(async move {
        while let Some(evt) = rx.recv().await {
            match evt {
                CopyEvent::ErrorPrompt(mut prompt) => {
                    if let Some(cached) = registry_for_ui.cached_action_for(7, &prompt.err) {
                        prompt.resolve(cached);
                        continue;
                    }
                    let Some(sender) = prompt.resolver.take() else {
                        continue;
                    };
                    let err = prompt.err.clone();
                    let id = registry_for_ui.register(7, err, sender);
                    // First prompt of this kind — resolve Skip with
                    // apply-to-all so the remaining read-only files
                    // don't raise more prompts.
                    registry_for_ui
                        .resolve(id, ErrorAction::Skip, true)
                        .expect("resolve");
                }
                CopyEvent::FileError { err } => {
                    registry_for_ui.log_auto(7, &err);
                }
                _ => {}
            }
        }
    });

    let report = copy_tree(&src_dir, &dst_dir, tree_opts, ctrl, tx)
        .await
        .expect("copy_tree should complete; policy=Ask + Skip-all absorbs the 3 permission errors");

    // Drop the tx half above returned it to us — no explicit drop
    // needed; copy_tree owns it and returned. The UI task sees the
    // channel close and exits.
    let _ = ui.await;

    assert_eq!(
        report.errored, 3,
        "three read-only targets → three errored entries on the TreeReport"
    );
    assert_eq!(
        report.files, 3,
        "the three writable files copied successfully"
    );

    // Writable files made it — byte-exact round-trip.
    for name in writable_names {
        let dst_file = dst_dir.join(name);
        assert!(dst_file.exists(), "{name} must exist");
        let want = format!("body-{name}");
        let got = std::fs::read_to_string(&dst_file).unwrap();
        assert_eq!(got, want, "{name} content round-trips");
    }

    // Error log holds 4 entries, all permission-denied kind.
    //
    // The engine emits *both* `ErrorPrompt` and (after Skip / RetryN
    // exhaustion) `CopyEvent::FileError` for the same file — see
    // `copythat_core::tree::record_file_error`. So the first
    // read-only file produces two log entries: one from
    // `ErrorRegistry::resolve` (resolution = "skip"), another from
    // `ErrorRegistry::log_auto` on the follow-up `FileError`
    // (resolution = "auto-skip"). Subsequent read-only files are
    // absorbed by the `apply_to_all = Skip` cache via
    // `prompt.resolve(cached)` (which does NOT append to the log),
    // then logged once each by `log_auto` on the matching
    // `FileError`. Total: 2 (first file) + 1 (second) + 1 (third) = 4.
    //
    // This was a 4-entry reality from Phase 8 onward; the original
    // smoke test's "3" expectation only appeared correct on the
    // Windows runner because of the `if cfg!(windows) { return; }`
    // guard — Linux / macOS CI was the first run that actually
    // executed this block.
    let log = registry.log();
    assert_eq!(
        log.len(),
        4,
        "engine emits ErrorPrompt + FileError for the user-resolved file; \
         cached files log once via FileError. got: {log:?}"
    );
    for entry in &log {
        assert_eq!(entry.kind, "permission-denied");
        assert_eq!(entry.localized_key, "err-permission-denied");
    }

    // CSV + TXT exports carry every read-only basename.
    let csv = export_csv(&log);
    let txt = export_txt(&log);
    for name in readonly_names {
        assert!(
            csv.contains(name),
            "CSV should reference {name} source path; got:\n{csv}"
        );
        assert!(
            txt.contains(name),
            "TXT should reference {name}; got:\n{txt}"
        );
    }
    assert!(
        csv.starts_with("timestamp_ms,job_id,kind"),
        "CSV header must be present"
    );
}

/// Sanity check that the CSV exporter escapes embedded commas and
/// quotes. Host-independent; runs on every CI OS.
#[test]
fn csv_export_quotes_commas_and_quotes() {
    let entries = vec![copythat_ui_lib::errors::LoggedError {
        id: 1,
        job_id: 1,
        timestamp_ms: 0,
        src: PathBuf::from("/tmp/weird, path.bin"),
        dst: PathBuf::from("/tmp/dst.bin"),
        kind: "io-other",
        localized_key: "err-io-other",
        message: String::from("it said \"no\""),
        raw_os_error: Some(5),
        resolution: Some("skip"),
    }];
    let csv = export_csv(&entries);
    let body = csv.lines().nth(1).expect("body row");
    // Doubled quotes inside a quoted field: `"it said ""no"""`.
    assert!(body.contains("\"\""));
    // Embedded comma must live inside a quoted field.
    assert!(body.contains("weird, path.bin"));
    // Row count stays 2 (header + one record) even though the body
    // contains commas; a naive split would over-count.
    assert_eq!(csv.lines().count(), 2);
}
