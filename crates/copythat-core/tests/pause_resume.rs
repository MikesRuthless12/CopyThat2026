//! Pause / resume / cancel behaviour.

mod common;

use std::time::Duration;

use common::{sleep, write_random};
use copythat_core::{CopyControl, CopyEvent, CopyOptions, copy_file};
use tempfile::tempdir;
use tokio::sync::mpsc;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn pause_halts_progress_and_resume_completes() {
    let dir = tempdir().unwrap();
    let src = dir.path().join("p.bin");
    let dst = dir.path().join("p.out");
    // Large enough that the copy is still running when we arm the
    // pause on the first Progress event. 128 MiB at a conservative
    // 500 MB/s is ~250 ms of wall-clock.
    let expected = write_random(&src, 128 * 1024 * 1024, 1);

    let (tx, mut rx) = mpsc::channel::<CopyEvent>(512);
    let ctrl = CopyControl::new();
    let ctrl_steer = ctrl.clone();

    let opts = CopyOptions {
        buffer_size: 64 * 1024,
        ..Default::default()
    };
    let src_task = src.clone();
    let dst_task = dst.clone();
    let task = tokio::spawn(async move { copy_file(&src_task, &dst_task, opts, ctrl, tx).await });

    let mut saw_progress = false;
    let mut saw_paused = false;
    let mut saw_resumed = false;
    let mut saw_completed = false;
    let mut steered = false;

    while let Some(evt) = rx.recv().await {
        match evt {
            CopyEvent::Progress { .. } => {
                saw_progress = true;
                if !steered {
                    steered = true;
                    // Pause off the event thread so we don't block the
                    // receiver; resume after a short hold.
                    let c = ctrl_steer.clone();
                    tokio::spawn(async move {
                        c.pause();
                        sleep(Duration::from_millis(80)).await;
                        c.resume();
                    });
                }
            }
            CopyEvent::Paused => saw_paused = true,
            CopyEvent::Resumed => saw_resumed = true,
            CopyEvent::Completed { .. } => {
                saw_completed = true;
                break;
            }
            CopyEvent::Failed { err } => panic!("copy failed: {err}"),
            _ => {}
        }
    }

    let report = task.await.unwrap().unwrap();
    assert_eq!(report.bytes as usize, expected.len());
    assert_eq!(std::fs::read(&dst).unwrap(), expected);
    assert!(saw_progress, "engine never emitted Progress");
    assert!(saw_paused, "engine never emitted Paused");
    assert!(saw_resumed, "engine never emitted Resumed");
    assert!(saw_completed);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn cancel_aborts_and_removes_partial_by_default() {
    let dir = tempdir().unwrap();
    let src = dir.path().join("c.bin");
    let dst = dir.path().join("c.out");
    write_random(&src, 4 * 1024 * 1024, 2);

    let (tx, mut rx) = mpsc::channel::<CopyEvent>(512);
    let ctrl = CopyControl::new();
    // Arm the cancel BEFORE the engine starts — guarantees the first
    // loop iteration sees `is_cancelled`. Sidesteps the race where a
    // small file copy finishes before we can schedule the cancel.
    ctrl.cancel();

    let opts = CopyOptions {
        buffer_size: 64 * 1024,
        ..Default::default()
    };

    let src_task = src.clone();
    let dst_task = dst.clone();
    let task = tokio::spawn(async move { copy_file(&src_task, &dst_task, opts, ctrl, tx).await });

    let mut saw_failed = false;
    while let Some(evt) = rx.recv().await {
        if let CopyEvent::Failed { err } = &evt {
            assert!(err.is_cancelled(), "expected cancellation error, got {err}");
            saw_failed = true;
        }
    }

    let result = task.await.unwrap();
    let err = result.expect_err("copy must not succeed after cancel");
    assert!(err.is_cancelled());
    assert!(saw_failed);
    assert!(
        !dst.exists(),
        "partial destination should have been removed"
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn cancel_with_keep_partial_leaves_destination() {
    let dir = tempdir().unwrap();
    let src = dir.path().join("k.bin");
    let dst = dir.path().join("k.out");
    write_random(&src, 4 * 1024 * 1024, 3);

    let (tx, mut rx) = mpsc::channel::<CopyEvent>(512);
    let ctrl = CopyControl::new();
    ctrl.cancel();

    let opts = CopyOptions {
        buffer_size: 64 * 1024,
        keep_partial: true,
        ..Default::default()
    };
    let src_task = src.clone();
    let dst_task = dst.clone();
    let task = tokio::spawn(async move { copy_file(&src_task, &dst_task, opts, ctrl, tx).await });

    while rx.recv().await.is_some() {}
    let err = task.await.unwrap().expect_err("must error");
    assert!(err.is_cancelled());
    assert!(
        dst.exists(),
        "keep_partial was set; destination should remain"
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn cancel_midflight_stops_a_large_copy() {
    // Stronger assertion than the pre-armed cancel: spawn the copy,
    // let it run long enough to emit progress, then cancel and verify
    // we don't ship the full payload.
    let dir = tempdir().unwrap();
    let src = dir.path().join("big.bin");
    let dst = dir.path().join("big.out");
    let src_len = 128 * 1024 * 1024;
    write_random(&src, src_len, 4);

    let (tx, mut rx) = mpsc::channel::<CopyEvent>(512);
    let ctrl = CopyControl::new();
    let ctrl_steer = ctrl.clone();

    let opts = CopyOptions {
        buffer_size: 64 * 1024,
        ..Default::default()
    };
    let src_task = src.clone();
    let dst_task = dst.clone();
    let task = tokio::spawn(async move { copy_file(&src_task, &dst_task, opts, ctrl, tx).await });

    // Started fires before any bytes copy — schedule a small delay off
    // of it so we're guaranteed to be mid-flight.
    let mut steered = false;
    while let Some(evt) = rx.recv().await {
        if matches!(evt, CopyEvent::Started { .. }) && !steered {
            steered = true;
            let c = ctrl_steer.clone();
            tokio::spawn(async move {
                sleep(Duration::from_millis(5)).await;
                c.cancel();
            });
        }
    }

    let err = task.await.unwrap().expect_err("must error");
    assert!(err.is_cancelled());
    assert!(!dst.exists(), "dst should have been cleaned up on cancel");
}
