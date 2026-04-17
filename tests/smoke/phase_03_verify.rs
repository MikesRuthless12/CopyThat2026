//! Phase 3 smoke test.
//!
//! Creates a 500 MB random source file, then for every supported
//! `HashAlgorithm` runs a `copy_file` with `CopyOptions::verify`
//! pointing at that algorithm. The test asserts:
//!
//! 1. The copy succeeds (`CopyReport` comes back, no `Failed` event).
//! 2. The verify pass completes and emits `VerifyCompleted`.
//! 3. `src_hex == dst_hex` in the `VerifyCompleted` event.
//! 4. The destination file matches the source byte-for-byte.
//! 5. The digest emitted by our `hash_file_async` against the
//!    destination matches the digest emitted by `sha256sum` / `b3sum`
//!    if those CLIs are installed on the host — warn-only, the smoke
//!    still passes when the tools are missing so CI doesn't flake on
//!    minimal runners.
//!
//! The file is written once and reused; each algorithm copies it into
//! a fresh destination so we don't cross-contaminate verify state.
//!
//! Runtime: ~30–60 s on a warm NVMe, longer on spinning media. 500 MB
//! is deliberately a bit heavier than Phase 1's 100 MB to exercise
//! multi-second verify passes and the progress-event throttle.

use std::path::Path;
use std::process::Command;
use std::time::Instant;

use copythat_core::{CopyControl, CopyEvent, CopyOptions, copy_file};
use copythat_hash::{HashAlgorithm, HashEvent, hash_file_async};
use rand::{RngCore, SeedableRng};
use tempfile::tempdir;
use tokio::sync::mpsc;

const SIZE: usize = 500 * 1024 * 1024; // 500 MiB

fn generate_source(path: &Path) {
    let mut rng = rand::rngs::StdRng::seed_from_u64(0x005A_1EC0_91AB);
    // Write in 16 MiB chunks to keep the working set bounded while the
    // on-disk payload grows to 500 MiB.
    let mut file = std::fs::File::create(path).unwrap();
    let mut buf = vec![0u8; 16 * 1024 * 1024];
    let mut remaining = SIZE;
    while remaining > 0 {
        let n = remaining.min(buf.len());
        rng.fill_bytes(&mut buf[..n]);
        std::io::Write::write_all(&mut file, &buf[..n]).unwrap();
        remaining -= n;
    }
    std::io::Write::flush(&mut file).unwrap();
}

async fn hash_with_our_pipeline(path: &Path, algo: HashAlgorithm) -> String {
    let (tx, _rx) = mpsc::channel::<HashEvent>(64);
    let report = hash_file_async(path, algo, CopyControl::new(), tx)
        .await
        .unwrap();
    report.hex()
}

/// Try to run an external hasher CLI. Returns `Some(hex)` when the
/// tool is available *and* we could parse its output; `None` when
/// missing or weird. Never panics — the smoke is warn-only on the
/// cross-check leg.
fn os_hash(bin: &str, args: &[&str], path: &Path) -> Option<String> {
    let mut cmd = Command::new(bin);
    cmd.args(args);
    cmd.arg(path);
    let out = cmd.output().ok()?;
    if !out.status.success() {
        return None;
    }
    let text = String::from_utf8(out.stdout).ok()?;
    // `sha256sum`, `sha1sum`, `md5sum`, `b3sum` all print `<hex>  <path>`.
    let hex = text.split_whitespace().next()?.to_ascii_lowercase();
    if hex.chars().all(|c| c.is_ascii_hexdigit()) && !hex.is_empty() {
        Some(hex)
    } else {
        None
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn phase_03_smoke_500mib_verify_each_algorithm() {
    let dir = tempdir().unwrap();
    let src = dir.path().join("phase03.src");
    let t0 = Instant::now();
    generate_source(&src);
    eprintln!(
        "PHASE03 SMOKE: wrote 500 MiB source in {:.2} s",
        t0.elapsed().as_secs_f64()
    );

    let mut warnings: Vec<String> = Vec::new();

    for algo in HashAlgorithm::ALL {
        let dst = dir.path().join(format!("phase03.{}.out", algo.name()));

        let (tx, mut rx) = mpsc::channel::<CopyEvent>(4096);
        let ctrl = CopyControl::new();
        let src_path = src.clone();
        let dst_path = dst.clone();
        let opts = CopyOptions {
            verify: Some(algo.verifier()),
            ..CopyOptions::default()
        };
        let started = Instant::now();
        let task =
            tokio::spawn(async move { copy_file(&src_path, &dst_path, opts, ctrl, tx).await });

        let mut saw_verify_started = false;
        let mut verify_report: Option<(String, String)> = None;
        let mut saw_failed = false;
        while let Some(evt) = rx.recv().await {
            match evt {
                CopyEvent::VerifyStarted { .. } => saw_verify_started = true,
                CopyEvent::VerifyCompleted {
                    src_hex, dst_hex, ..
                } => {
                    verify_report = Some((src_hex, dst_hex));
                }
                CopyEvent::VerifyFailed { .. } | CopyEvent::Failed { .. } => saw_failed = true,
                _ => {}
            }
        }
        let report = task
            .await
            .unwrap()
            .unwrap_or_else(|e| panic!("{algo}: {e}"));
        let wall = started.elapsed();

        assert!(!saw_failed, "{algo}: unexpected failure event");
        assert!(saw_verify_started, "{algo}: no VerifyStarted");
        let (src_hex, dst_hex) =
            verify_report.unwrap_or_else(|| panic!("{algo}: no VerifyCompleted emitted"));
        assert_eq!(src_hex, dst_hex, "{algo}: src != dst hash");
        assert_eq!(report.bytes as usize, SIZE, "{algo}: byte count mismatch");

        // Bytewise check — loads 1 GB (src + dst) but only per-algorithm
        // and only once each; the smoke is already heavy by design.
        let src_bytes = std::fs::read(&src).unwrap();
        let dst_bytes = std::fs::read(&dst).unwrap();
        assert_eq!(
            src_bytes, dst_bytes,
            "{algo}: destination differs byte-wise"
        );

        // Independent hash through `hash_file_async`, cross-checked
        // against an external tool when available.
        let our_hex = hash_with_our_pipeline(&dst, *algo).await;
        assert_eq!(
            our_hex, dst_hex,
            "{algo}: hash_file_async disagrees with engine verify hash"
        );

        let os_pair = match algo {
            HashAlgorithm::Sha256 => Some(("sha256sum", vec![])),
            HashAlgorithm::Sha1 => Some(("sha1sum", vec![])),
            HashAlgorithm::Sha512 => Some(("sha512sum", vec![])),
            HashAlgorithm::Md5 => Some(("md5sum", vec![])),
            HashAlgorithm::Blake3 => Some(("b3sum", vec!["--no-names"])),
            _ => None,
        };
        if let Some((bin, args)) = os_pair {
            match os_hash(bin, &args, &dst) {
                Some(os) => {
                    if os == our_hex {
                        eprintln!("PHASE03 SMOKE: {algo} cross-check against `{bin}` PASS");
                    } else {
                        warnings.push(format!(
                            "{algo}: `{bin}` reported {os} but we reported {our_hex}"
                        ));
                    }
                }
                None => {
                    eprintln!("PHASE03 SMOKE: {algo} cross-check skipped — `{bin}` not available");
                }
            }
        }

        let mib = SIZE as f64 / (1024.0 * 1024.0);
        let secs = wall.as_secs_f64().max(1e-9);
        eprintln!(
            "PHASE03 SMOKE: {algo} copy+verify {mib:.1} MiB in {:.2} s ({:.1} MiB/s)",
            secs,
            mib / secs
        );

        // Drop the per-algorithm destination early so we don't bloat
        // disk usage across the loop.
        let _ = std::fs::remove_file(&dst);
    }

    if !warnings.is_empty() {
        panic!(
            "PHASE03 SMOKE: {} cross-check failure(s): {}",
            warnings.len(),
            warnings.join("; ")
        );
    }
}
