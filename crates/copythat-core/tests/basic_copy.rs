//! Byte-exact correctness for a variety of sizes.

mod common;

use common::{has_variant, read_all, run_copy, write_random};
use copythat_core::{CopyEvent, CopyOptions};
use tempfile::tempdir;

#[tokio::test]
async fn empty_file_copies_zero_bytes() {
    let dir = tempdir().unwrap();
    let src = dir.path().join("empty.bin");
    let dst = dir.path().join("empty.out");
    std::fs::write(&src, b"").unwrap();

    let (report, events) = run_copy(&src, &dst, CopyOptions::default()).await;
    let report = report.unwrap();
    assert_eq!(report.bytes, 0);
    assert_eq!(std::fs::metadata(&dst).unwrap().len(), 0);
    assert!(has_variant(&events, |e| matches!(
        e,
        CopyEvent::Started { .. }
    )));
    assert!(has_variant(&events, |e| matches!(
        e,
        CopyEvent::Completed { .. }
    )));
}

#[tokio::test]
async fn single_byte_roundtrip() {
    let dir = tempdir().unwrap();
    let src = dir.path().join("one.bin");
    let dst = dir.path().join("one.out");
    std::fs::write(&src, b"Z").unwrap();

    let (report, _events) = run_copy(&src, &dst, CopyOptions::default()).await;
    let report = report.unwrap();
    assert_eq!(report.bytes, 1);
    assert_eq!(read_all(&dst), b"Z");
}

#[tokio::test]
async fn one_mib_roundtrip_is_byte_exact() {
    let dir = tempdir().unwrap();
    let src = dir.path().join("1m.bin");
    let dst = dir.path().join("1m.out");
    let expected = write_random(&src, 1024 * 1024, 42);

    let (report, events) = run_copy(&src, &dst, CopyOptions::default()).await;
    let report = report.unwrap();
    assert_eq!(report.bytes as usize, expected.len());
    assert_eq!(read_all(&dst), expected);
    // 1 MiB should trigger at least one Progress at the 16 KiB + 50 ms
    // cadence (actually: Progress may or may not fire on a very fast
    // SSD because the whole copy beats the 50 ms gate; we only assert
    // the lifecycle events fired).
    assert!(has_variant(&events, |e| matches!(
        e,
        CopyEvent::Started { .. }
    )));
    assert!(has_variant(&events, |e| matches!(
        e,
        CopyEvent::Completed { .. }
    )));
}

#[tokio::test]
async fn custom_small_buffer_still_produces_byte_exact_copy() {
    let dir = tempdir().unwrap();
    let src = dir.path().join("odd.bin");
    let dst = dir.path().join("odd.out");
    // 200 KiB — not a power-of-two-buffer multiple.
    let expected = write_random(&src, 200 * 1024, 7);

    let opts = CopyOptions {
        buffer_size: 64 * 1024,
        ..Default::default()
    };
    let (report, _events) = run_copy(&src, &dst, opts).await;
    let report = report.unwrap();
    assert_eq!(report.bytes as usize, expected.len());
    assert_eq!(read_all(&dst), expected);
}

#[cfg(feature = "slow-tests")]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn one_gib_slow_roundtrip() {
    let dir = tempdir().unwrap();
    let src = dir.path().join("1g.bin");
    let dst = dir.path().join("1g.out");
    let size: usize = 1024 * 1024 * 1024;
    // Use a pattern, not pure random, so writing + verifying stays under the
    // time budget on modest CI hardware. We still assert byte equality below.
    let chunk: Vec<u8> = (0u32..(64 * 1024u32)).map(|i| (i & 0xff) as u8).collect();
    {
        use std::io::Write;
        let mut f = std::fs::File::create(&src).unwrap();
        let mut written = 0;
        while written < size {
            f.write_all(&chunk).unwrap();
            written += chunk.len();
        }
    }

    let (report, _events) = run_copy(&src, &dst, CopyOptions::default()).await;
    let report = report.unwrap();
    assert_eq!(report.bytes as usize, size);

    // Stream compare — don't hold both in memory.
    use std::io::Read;
    let mut a = std::fs::File::open(&src).unwrap();
    let mut b = std::fs::File::open(&dst).unwrap();
    let mut ba = vec![0u8; 1024 * 1024];
    let mut bb = vec![0u8; 1024 * 1024];
    loop {
        let na = a.read(&mut ba).unwrap();
        let nb = b.read(&mut bb).unwrap();
        assert_eq!(na, nb);
        if na == 0 {
            break;
        }
        assert_eq!(&ba[..na], &bb[..nb]);
    }
}
