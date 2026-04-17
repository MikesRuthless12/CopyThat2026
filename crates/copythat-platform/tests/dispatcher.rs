//! Phase 6 integration tests for `copythat-platform`.
//!
//! Covers the dispatcher: each strategy fires when it should, the
//! strategy enum is reported correctly, fallback runs when reflink and
//! native both decline, and the `PlatformFastCopyHook` plugs into
//! `copythat_core::copy_file` end-to-end.
//!
//! On Windows runners we expect `CopyFileExW` (or reflink on a Dev
//! Drive). On macOS, `Copyfile` (or reflink on APFS — usually). On
//! Linux runners with a Btrfs loop device, `Reflink`. All other Linux
//! runners default to `CopyFileRange` (or `Sendfile` for files <2 GiB).

use std::path::Path;
use std::sync::Arc;

use copythat_core::{
    CopyControl, CopyEvent, CopyOptions, CopyStrategy, FastCopyHook, FastCopyHookOutcome, copy_file,
};
use copythat_platform::{ChosenStrategy, PlatformFastCopyHook, fast_copy, recommend_concurrency};
use tempfile::tempdir;
use tokio::sync::mpsc;

const SMALL: usize = 1024 * 1024; // 1 MiB
const MEDIUM: usize = 16 * 1024 * 1024; // 16 MiB

fn seeded(path: &Path, size: usize, seed: u8) {
    let mut buf = Vec::with_capacity(size);
    for i in 0..size {
        buf.push((i as u8).wrapping_add(seed));
    }
    std::fs::write(path, &buf).expect("seed file");
}

async fn drain(mut rx: mpsc::Receiver<CopyEvent>) -> Vec<CopyEvent> {
    let mut out = Vec::new();
    while let Some(evt) = rx.recv().await {
        out.push(evt);
    }
    out
}

fn expected_native_strategy() -> ChosenStrategy {
    if cfg!(target_os = "windows") {
        ChosenStrategy::CopyFileExW
    } else if cfg!(target_os = "macos") {
        ChosenStrategy::Copyfile
    } else if cfg!(target_os = "linux") {
        ChosenStrategy::CopyFileRange
    } else {
        ChosenStrategy::AsyncFallback
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn dispatcher_picks_a_fast_strategy() {
    let dir = tempdir().unwrap();
    let src = dir.path().join("src.bin");
    let dst = dir.path().join("dst.bin");
    seeded(&src, SMALL, 0x37);

    let (tx, rx) = mpsc::channel::<CopyEvent>(1024);
    let opts = CopyOptions::default();
    let outcome = fast_copy(&src, &dst, opts, CopyControl::new(), tx)
        .await
        .expect("fast_copy");
    let _events = drain(rx).await;

    // We accept any path other than AsyncFallback here — reflink is the
    // happy case on Btrfs / APFS / Dev Drive, native is the happy case
    // everywhere else. AsyncFallback is allowed only if the runner is
    // an exotic OS we don't have a backend for (caught by cfg above).
    assert_eq!(outcome.bytes, SMALL as u64);
    if cfg!(any(
        target_os = "windows",
        target_os = "macos",
        target_os = "linux"
    )) {
        assert_ne!(
            outcome.strategy,
            ChosenStrategy::AsyncFallback,
            "expected reflink or native on supported OS, got AsyncFallback"
        );
    }

    // Content matches.
    let src_bytes = std::fs::read(&src).unwrap();
    let dst_bytes = std::fs::read(&dst).unwrap();
    assert_eq!(src_bytes, dst_bytes);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn always_async_returns_async_fallback() {
    let dir = tempdir().unwrap();
    let src = dir.path().join("src.bin");
    let dst = dir.path().join("dst.bin");
    seeded(&src, SMALL, 0x42);

    let (tx, rx) = mpsc::channel::<CopyEvent>(1024);
    let opts = CopyOptions {
        strategy: CopyStrategy::AlwaysAsync,
        ..CopyOptions::default()
    };
    let outcome = fast_copy(&src, &dst, opts, CopyControl::new(), tx)
        .await
        .expect("fast_copy");
    let _ = drain(rx).await;

    assert_eq!(outcome.strategy, ChosenStrategy::AsyncFallback);
    assert_eq!(outcome.bytes, SMALL as u64);

    let src_bytes = std::fs::read(&src).unwrap();
    let dst_bytes = std::fs::read(&dst).unwrap();
    assert_eq!(src_bytes, dst_bytes);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn no_reflink_strategy_skips_reflink() {
    let dir = tempdir().unwrap();
    let src = dir.path().join("src.bin");
    let dst = dir.path().join("dst.bin");
    seeded(&src, SMALL, 0x77);

    let (tx, rx) = mpsc::channel::<CopyEvent>(1024);
    let opts = CopyOptions {
        strategy: CopyStrategy::NoReflink,
        ..CopyOptions::default()
    };
    let outcome = fast_copy(&src, &dst, opts, CopyControl::new(), tx)
        .await
        .expect("fast_copy");
    let _ = drain(rx).await;

    // NoReflink must never report Reflink as the chosen strategy.
    assert_ne!(outcome.strategy, ChosenStrategy::Reflink);
    assert_eq!(outcome.bytes, SMALL as u64);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn native_path_is_invoked_on_supported_os() {
    // This test confirms the OS-native strategy is actually reached.
    // We use NoReflink to skip the COW fast path so the dispatcher is
    // forced into the OS-native branch.
    let dir = tempdir().unwrap();
    let src = dir.path().join("src.bin");
    let dst = dir.path().join("dst.bin");
    seeded(&src, MEDIUM, 0x91);

    let (tx, rx) = mpsc::channel::<CopyEvent>(1024);
    let opts = CopyOptions {
        strategy: CopyStrategy::NoReflink,
        ..CopyOptions::default()
    };
    let outcome = fast_copy(&src, &dst, opts, CopyControl::new(), tx)
        .await
        .expect("fast_copy");
    let _ = drain(rx).await;

    let expected = expected_native_strategy();
    if expected != ChosenStrategy::AsyncFallback {
        // On Linux, sendfile is also acceptable; the dispatcher prefers
        // copy_file_range and falls through only on EXDEV/EINVAL.
        let acceptable = matches!(
            outcome.strategy,
            ChosenStrategy::CopyFileExW
                | ChosenStrategy::Copyfile
                | ChosenStrategy::CopyFileRange
                | ChosenStrategy::Sendfile
        );
        assert!(
            acceptable,
            "expected an OS-native strategy, got {:?}",
            outcome.strategy
        );
    }
    assert_eq!(outcome.bytes, MEDIUM as u64);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn hook_integrates_with_copy_file() {
    let dir = tempdir().unwrap();
    let src = dir.path().join("src.bin");
    let dst = dir.path().join("dst.bin");
    seeded(&src, MEDIUM, 0xAB);

    let (tx, rx) = mpsc::channel::<CopyEvent>(1024);
    let hook: Arc<dyn FastCopyHook> = Arc::new(PlatformFastCopyHook);
    let opts = CopyOptions {
        fast_copy_hook: Some(hook),
        ..CopyOptions::default()
    };
    let report = copy_file(&src, &dst, opts, CopyControl::new(), tx)
        .await
        .expect("copy_file via hook");
    let _ = drain(rx).await;

    assert_eq!(report.bytes, MEDIUM as u64);
    let src_bytes = std::fs::read(&src).unwrap();
    let dst_bytes = std::fs::read(&dst).unwrap();
    assert_eq!(src_bytes, dst_bytes);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn hook_returns_done_so_engine_does_not_double_copy() {
    // If the hook returned NotSupported the engine would run its own
    // loop and overwrite the destination. Ensure the bytes-on-disk
    // count matches the source size exactly (not 2× because of a
    // double-copy bug).
    let dir = tempdir().unwrap();
    let src = dir.path().join("src.bin");
    let dst = dir.path().join("dst.bin");
    seeded(&src, MEDIUM, 0xCD);

    let (tx, rx) = mpsc::channel::<CopyEvent>(1024);
    let hook = PlatformFastCopyHook;
    let opts = CopyOptions {
        fast_copy_hook: Some(Arc::new(hook)),
        ..CopyOptions::default()
    };
    let _ = copy_file(&src, &dst, opts, CopyControl::new(), tx)
        .await
        .unwrap();
    let _ = drain(rx).await;

    let dst_meta = std::fs::metadata(&dst).unwrap();
    assert_eq!(dst_meta.len(), MEDIUM as u64);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn verify_options_bypass_the_hook() {
    use copythat_core::Verifier;

    // When verify is set, the engine should skip the hook entirely
    // (the hook bypasses bytes through the kernel — there's nothing to
    // hash from a CopyFileExW call). We confirm by installing a hook
    // that would panic if invoked.

    #[derive(Debug)]
    struct PanickingHook;
    impl FastCopyHook for PanickingHook {
        fn try_copy<'a>(
            &'a self,
            _src: std::path::PathBuf,
            _dst: std::path::PathBuf,
            _opts: CopyOptions,
            _ctrl: CopyControl,
            _events: mpsc::Sender<CopyEvent>,
        ) -> std::pin::Pin<
            Box<
                dyn std::future::Future<
                        Output = Result<FastCopyHookOutcome, copythat_core::CopyError>,
                    > + Send
                    + 'a,
            >,
        > {
            Box::pin(async { panic!("hook should not have been called") })
        }
    }

    let dir = tempdir().unwrap();
    let src = dir.path().join("src.bin");
    let dst = dir.path().join("dst.bin");
    seeded(&src, SMALL, 0x05);

    // Use a no-op verifier — copythat-hash would normally provide one.
    // We define a trivial hasher inline so this test doesn't depend on
    // copythat-hash.
    #[derive(Default)]
    struct NopHasher(u64);
    impl copythat_core::Hasher for NopHasher {
        fn name(&self) -> &'static str {
            "nop"
        }
        fn update(&mut self, buf: &[u8]) {
            self.0 = self.0.wrapping_add(buf.len() as u64);
        }
        fn finalize(self: Box<Self>) -> Vec<u8> {
            self.0.to_be_bytes().to_vec()
        }
    }
    let verifier = Verifier::new("nop", || Box::new(NopHasher::default()));

    let (tx, rx) = mpsc::channel::<CopyEvent>(1024);
    let opts = CopyOptions {
        verify: Some(verifier),
        fast_copy_hook: Some(Arc::new(PanickingHook)),
        ..CopyOptions::default()
    };
    let _ = copy_file(&src, &dst, opts, CopyControl::new(), tx)
        .await
        .expect("copy_file with verify but bypassed hook");
    let _ = drain(rx).await;
}

#[test]
fn recommend_concurrency_clamps_for_unknown_paths() {
    let n = recommend_concurrency(Path::new("."), Path::new("."), 8);
    // Unknown probe answers are treated as SSD; default keeps `requested`.
    // On hosts where the probe answers Some(false) (rotational), we get 1.
    assert!(n == 1 || n == 8, "unexpected concurrency: {n}");
}
