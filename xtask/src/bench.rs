//! Phase 13 bench runners driven by `xtask bench`, `xtask bench-ci`,
//! and `xtask bench-vs`.
//!
//! The first two shell out to `cargo bench -p copythat-core --bench
//! copy_bench` with the same argv Criterion expects, optionally
//! setting the `COPYTHAT_BENCH_CI=1` env var so each workload scales
//! down to its CI-friendly size. We don't re-implement Criterion's
//! harness here — a subprocess wrapper keeps the output format
//! identical to `cargo bench`, and the 10 %-regression gate in
//! `docs/BENCHMARKS.md` references the same line noise.
//!
//! `bench-vs` is different: it times a fixed 1 GiB workload against
//! our engine *and* whatever competitor copy tools it can find on
//! PATH (Robocopy / TeraCopy / FastCopy on Windows; `cp` / `rsync` /
//! `ditto` on Unix). Tools that aren't installed are listed as
//! "skipped"; the runner only fails if it cannot time our own
//! engine. Results print as a stable markdown table so the output
//! pastes directly into `docs/BENCHMARKS.md`.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use crate::repo_root;

/// Run `cargo bench -p copythat-core --bench copy_bench [args...]`.
///
/// When `ci` is true, set `COPYTHAT_BENCH_CI=1` so the bench file
/// picks up its scaled-down workload constants. Also pass Criterion's
/// `--warm-up-time 1 --measurement-time 3 --sample-size 10` in CI
/// mode so the total run finishes under a couple of minutes.
pub fn run(ci: bool) -> Result<(), String> {
    let root = repo_root().ok_or("could not locate repo root")?;
    let mut cmd = Command::new(cargo_bin());
    cmd.current_dir(&root)
        .arg("bench")
        .arg("-p")
        .arg("copythat-core")
        .arg("--bench")
        .arg("copy_bench")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());
    if ci {
        cmd.env("COPYTHAT_BENCH_CI", "1");
        // Forward a conservative Criterion arg block so CI runs
        // finish in tens of seconds rather than minutes.
        cmd.arg("--")
            .arg("--warm-up-time")
            .arg("1")
            .arg("--measurement-time")
            .arg("3")
            .arg("--sample-size")
            .arg("10")
            .arg("--noplot");
    }
    let status = cmd
        .status()
        .map_err(|e| format!("spawn cargo bench: {e}"))?;
    if !status.success() {
        return Err(format!("cargo bench exited with {status}"));
    }
    Ok(())
}

/// Scripted head-to-head run. Writes a ~256 MiB synthetic source
/// file into a tempdir, then times our engine + each detected
/// competitor copying that file to a sibling path inside the same
/// tempdir (so we're benchmarking the write path, not cross-device
/// DMA). 3 warm-ups + 5 timed runs per tool; we report the median.
///
/// The workload is small enough to finish in ~60 s even on spinning
/// media. For a proper 1 GiB–10 GiB run the user bumps
/// `COPYTHAT_BENCH_VS_SIZE_MB=10240` before invoking xtask.
pub fn run_vs_with(secure_cleanup: bool) -> Result<(), String> {
    let root = repo_root().ok_or("could not locate repo root")?;
    let size_mb = std::env::var("COPYTHAT_BENCH_VS_SIZE_MB")
        .ok()
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(256);
    let size_bytes = size_mb * 1024 * 1024;

    // Blow away lingering bench artefacts from previous runs
    // across C:, D:, E: so every measurement sees a cold
    // destination. `tempfile::tempdir()` cleans on drop for C:
    // but the user-specified override paths on D:\ and E:\ stay
    // around unless we wipe them explicitly. A second pass at the
    // end (after the bench) keeps the volumes tidy for the next
    // invocation.
    purge_bench_residue();

    // Fair-comparison fix: put src and dst in *separate* subdirs.
    // Robocopy / TeraCopy / FastCopy all identify "copy onto self"
    // when src_dir == dst_dir and skip the copy entirely (exit in
    // ~10 ms, which looked like "33 GB/s" in earlier runs). Splitting
    // into `src/` and `dst/` subdirectories forces every tool to
    // actually move bytes even on the same-volume case.
    let tmp = tempfile::tempdir().map_err(|e| format!("tempdir: {e}"))?;
    let src_dir = tmp.path().join("src");
    std::fs::create_dir_all(&src_dir).map_err(|e| format!("create {}: {e}", src_dir.display()))?;
    let src = src_dir.join("bench-source.bin");

    // Destination override — lets us benchmark a cross-volume copy
    // by setting `COPYTHAT_BENCH_DST=D:\path\to\workdir` before
    // running. Falls back to a `dst/` subdir inside the same
    // tempdir (same-volume baseline) when unset.
    let (_dst_tmp, dst_dir) = match std::env::var("COPYTHAT_BENCH_DST") {
        Ok(raw) => {
            let dir = PathBuf::from(raw).join("copythat-bench-vs");
            std::fs::create_dir_all(&dir).map_err(|e| format!("create {}: {e}", dir.display()))?;
            // No TempDir guard — the user-chosen location is their
            // responsibility to clean up.
            (None, dir)
        }
        Err(_) => {
            let dir = tmp.path().join("dst");
            std::fs::create_dir_all(&dir).map_err(|e| format!("create {}: {e}", dir.display()))?;
            (Some(&tmp), dir)
        }
    };
    let dst = dst_dir.join("bench-dst.bin");
    eprintln!(
        "bench-vs: src={} dst={} workload={} MiB",
        src.display(),
        dst.display(),
        size_mb
    );
    write_synthetic(&src, size_bytes)?;

    let mut rows: Vec<Row> = Vec::new();

    // --- Our engine ---
    // xtask now links copythat-core directly so we can time the
    // async loop head-to-head. 3 warm-ups + 5 measured runs; we
    // report the median wall-clock elapsed, same shape as the
    // competitor runners below.
    rows.push(time_engine(&src, &dst, size_bytes));

    // --- Competitor matrix ---
    for candidate in competitors() {
        rows.push(time_tool(
            candidate.label,
            Some(candidate),
            || Err("unused".into()), // `time_tool` handles tool-discovery itself.
            &src,
            &dst,
            size_bytes,
        ));
    }

    print_markdown(&rows, size_bytes);

    // Write a copy of the table next to the repo so the user can
    // paste it into docs/BENCHMARKS.md manually.
    let report = root.join("target").join("bench-vs.md");
    fs::create_dir_all(report.parent().unwrap()).ok();
    fs::write(&report, format_markdown(&rows, size_bytes))
        .map_err(|e| format!("write report: {e}"))?;
    eprintln!("wrote report → {}", report.display());

    if secure_cleanup {
        // Shred the leftover dst file with the same DoD-3 overwrite
        // the UI offers. Uses `copythat_secure_delete::shred_file`
        // on the Rust side (same engine the app ships) so a run of
        // `bench-vs --secure-cleanup` also doubles as a smoke test
        // of the shredder against a real 10 GiB file.
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .map_err(|e| format!("tokio runtime: {e}"))?;
        let dst_for_shred = dst.clone();
        let shred = rt.block_on(async move { secure_shred_dst(&dst_for_shred).await });
        if let Err(e) = shred {
            eprintln!("bench-vs: secure cleanup failed ({e}); falling back to plain rm");
        } else {
            eprintln!("bench-vs: secure-deleted {}", dst.display());
        }
    }

    // Cleanup pass: remove destination residue so back-to-back
    // `bench-vs` invocations don't stack 10 GiB files on the
    // user's volumes. The `tempfile::tempdir()` drop at the end
    // of this function handles the C: side automatically.
    purge_bench_residue();
    Ok(())
}

/// Phase 13c — DoD-3-pass shred via `copythat-secure-delete`. Used
/// by `bench-vs --secure-cleanup` to wipe the bench dst file after
/// a run. Takes ~30 s on a 10 GiB file on an external USB drive
/// (three sequential writes through the bus); only invoked on
/// explicit opt-in so default bench runs stay fast.
async fn secure_shred_dst(dst: &Path) -> Result<(), String> {
    use copythat_core::CopyControl;
    use copythat_secure_delete::{ShredMethod, shred_file};
    use tokio::sync::mpsc;

    if !dst.exists() {
        return Ok(());
    }
    let ctrl = CopyControl::new();
    let (tx, mut rx) = mpsc::channel(64);
    let drain = tokio::spawn(async move { while rx.recv().await.is_some() {} });
    shred_file(dst, ShredMethod::DoD3Pass, ctrl, tx)
        .await
        .map_err(|e| format!("shred_file: {e}"))?;
    drain.abort();
    Ok(())
}

/// Delete any leftover `copythat-bench-vs/` directories on C:, D:,
/// and E: (whichever exist). The name is predictable — every run
/// creates one under whichever drive is the destination — so
/// we never touch unrelated user data. Missing directories are
/// silently ignored; each is a non-fatal best-effort clean.
fn purge_bench_residue() {
    let candidates: &[&str] = if cfg!(windows) {
        &[
            r"C:\copythat-bench-vs",
            r"D:\copythat-bench-vs",
            r"E:\copythat-bench-vs",
        ]
    } else {
        &["/tmp/copythat-bench-vs"]
    };
    for path in candidates {
        let p = Path::new(path);
        if p.exists() {
            match fs::remove_dir_all(p) {
                Ok(()) => eprintln!("bench-vs: cleaned {}", p.display()),
                Err(e) => eprintln!("bench-vs: cleanup {} failed ({e}); ignoring", p.display()),
            }
        }
    }
}

// ---------------------------------------------------------------------
// Competitor detection + timing
// ---------------------------------------------------------------------

/// Synthetic source-file writer. Deterministic 256-byte rolling
/// pattern — same shape as the Criterion bench's helper, so a
/// compressing filesystem (ZFS, APFS, some Btrfs configs) can't
/// silently inflate our numbers by skipping the write pass.
fn write_synthetic(path: &Path, size: usize) -> Result<(), String> {
    use std::io::Write;
    let pattern: [u8; 256] = std::array::from_fn(|i| i as u8);
    let mut f =
        std::fs::File::create(path).map_err(|e| format!("create {}: {e}", path.display()))?;
    let mut remaining = size;
    while remaining > 0 {
        let n = remaining.min(pattern.len());
        f.write_all(&pattern[..n])
            .map_err(|e| format!("write: {e}"))?;
        remaining -= n;
    }
    f.sync_all().ok();
    Ok(())
}

struct Competitor {
    label: &'static str,
    /// Argv template. `{SRC}` and `{DST}` are substituted before
    /// exec; `{SRC_DIR}` and `{SRC_NAME}` also available for tools
    /// that want the pieces separately (Robocopy).
    argv: &'static [&'static str],
    /// Windows-specific fallback install locations tried in order
    /// when `argv[0]` is not on PATH. Lets the bench pick up a
    /// default-installed TeraCopy / FastCopy without the user
    /// needing to fiddle with the `PATH` env var. First match wins
    /// and replaces `argv[0]` for the run.
    fallback_paths: &'static [&'static str],
}

#[cfg(windows)]
fn competitors() -> Vec<Competitor> {
    vec![
        // Robocopy copies a single file by naming its parent dir +
        // filename separately; we wrap it to match our src/dst shape.
        Competitor {
            label: "Robocopy",
            argv: &[
                "robocopy",
                "{SRC_DIR}",
                "{DST_DIR}",
                "{SRC_NAME}",
                "/NFL",
                "/NDL",
                "/NJH",
                "/NJS",
            ],
            fallback_paths: &[],
        },
        // TeraCopy.exe — GUI-first, but runs a foreground copy when
        // invoked as `TeraCopy.exe Copy <src> <dst_dir> /Close /SkipAll`.
        // `/Close` makes the window auto-dismiss when the copy
        // completes so `Command::status()` actually blocks until
        // the whole copy is done. `/SkipAll` suppresses the
        // "destination exists" prompt (we remove dst before each
        // iteration, but TeraCopy still prompts on some releases).
        Competitor {
            label: "TeraCopy",
            argv: &[
                "teracopy",
                "Copy",
                "{SRC}",
                "{DST_DIR}",
                "/Close",
                "/SkipAll",
            ],
            fallback_paths: &[
                r"C:\Program Files\TeraCopy\TeraCopy.exe",
                r"C:\Program Files (x86)\TeraCopy\TeraCopy.exe",
            ],
        },
        // FastCopy CLI.
        Competitor {
            label: "FastCopy",
            argv: &[
                "fastcopy",
                "/cmd=diff",
                "/auto_close",
                "{SRC}",
                "/to={DST_DIR}",
            ],
            fallback_paths: &[
                r"C:\Program Files\FastCopy\FastCopy.exe",
                r"C:\Program Files (x86)\FastCopy\FastCopy.exe",
            ],
        },
        // Windows built-in `copy` via cmd.exe.
        Competitor {
            label: "cmd copy",
            argv: &["cmd.exe", "/C", "copy", "/Y", "{SRC}", "{DST}"],
            fallback_paths: &[],
        },
    ]
}

#[cfg(all(unix, not(target_os = "macos")))]
fn competitors() -> Vec<Competitor> {
    vec![
        Competitor {
            label: "cp",
            argv: &["cp", "{SRC}", "{DST}"],
            fallback_paths: &[],
        },
        Competitor {
            label: "rsync",
            argv: &["rsync", "--inplace", "{SRC}", "{DST}"],
            fallback_paths: &[],
        },
    ]
}

#[cfg(target_os = "macos")]
fn competitors() -> Vec<Competitor> {
    vec![
        Competitor {
            label: "cp",
            argv: &["cp", "{SRC}", "{DST}"],
            fallback_paths: &[],
        },
        Competitor {
            label: "ditto",
            argv: &["ditto", "{SRC}", "{DST}"],
            fallback_paths: &[],
        },
        Competitor {
            label: "rsync",
            argv: &["rsync", "--inplace", "{SRC}", "{DST}"],
            fallback_paths: &[],
        },
    ]
}

/// Execution outcome for one competitor.
struct Row {
    label: String,
    /// Median elapsed wall-clock. `None` means the tool wasn't on
    /// PATH or the driver returned an error.
    median: Option<Duration>,
    note: String,
}

/// Time our own async engine with `CopyOptions::default()`. Mirrors
/// the 3-warmup-5-sample pattern used for competitor CLIs so the
/// numbers land on the same axis. `CopyControl` and the event
/// channel stay alive for the whole iteration — we drain events on
/// a background task so the `mpsc::Sender`'s backpressure doesn't
/// stall the engine.
fn time_engine(src: &Path, dst: &Path, size_bytes: usize) -> Row {
    use std::sync::Arc;

    use copythat_core::{CopyControl, CopyOptions, copy_file};
    use copythat_platform::PlatformFastCopyHook;

    let Ok(rt) = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
    else {
        return Row {
            label: "CopyThat".into(),
            median: None,
            note: "tokio runtime build failed".into(),
        };
    };

    // Mirror the shipped Tauri config: the platform fast-path hook
    // attached. Without this, we're racing a pure-async byte loop
    // against Windows's kernel-tuned CopyFileEx — which is the
    // same asymmetry users would never see in the real app.
    let hook: Arc<dyn copythat_core::FastCopyHook> = Arc::new(PlatformFastCopyHook);
    let base_opts = CopyOptions {
        fast_copy_hook: Some(hook.clone()),
        ..Default::default()
    };

    let (warmups, runs) = if size_bytes >= 5 * 1024 * 1024 * 1024 {
        (1usize, 2usize)
    } else if size_bytes >= 1024 * 1024 * 1024 {
        (1usize, 3usize)
    } else {
        (3usize, 5usize)
    };
    let mut samples: Vec<Duration> = Vec::with_capacity(runs);
    for i in 0..(warmups + runs) {
        let _ = std::fs::remove_file(dst);
        let opts = base_opts.clone();
        let t0 = Instant::now();
        let outcome = rt.block_on(async {
            let ctrl = CopyControl::new();
            let (tx, mut rx) = tokio::sync::mpsc::channel(32);
            let drain = tokio::spawn(async move { while rx.recv().await.is_some() {} });
            let r = copy_file(src, dst, opts, ctrl, tx).await;
            drain.abort();
            r
        });
        let elapsed = t0.elapsed();
        if outcome.is_err() {
            return Row {
                label: "CopyThat".into(),
                median: None,
                note: format!("copy_file: {:?}", outcome.err().map(|e| e.message)),
            };
        }
        if i >= warmups {
            samples.push(elapsed);
        }
    }
    samples.sort();
    let median = samples.get(samples.len() / 2).copied();
    let note = median
        .map(|d| format!("{:>6.0} MiB/s", mb_per_sec(size_bytes, d)))
        .unwrap_or_else(|| "no samples".into());
    Row {
        label: "CopyThat".into(),
        median,
        note,
    }
}

fn time_tool(
    label: &str,
    competitor: Option<Competitor>,
    _fallback: impl FnOnce() -> Result<Duration, String>,
    src: &Path,
    dst: &Path,
    size_bytes: usize,
) -> Row {
    let Some(comp) = competitor else {
        // Rust-side engine placeholder (see comment in run_vs).
        return Row {
            label: label.into(),
            median: None,
            note: "see `xtask bench` output".into(),
        };
    };

    // Resolve the executable path. Prefer the PATH hit; fall back
    // to the competitor's list of known install locations so a
    // GUI-installed TeraCopy / FastCopy still participates in the
    // bench without the user editing PATH.
    let exe = comp.argv[0];
    let resolved_exe = if on_path(exe) {
        exe.to_string()
    } else {
        let found = comp.fallback_paths.iter().find(|p| Path::new(p).exists());
        match found {
            Some(p) => p.to_string(),
            None => {
                return Row {
                    label: label.into(),
                    median: None,
                    note: format!("{exe} not on PATH — skipped"),
                };
            }
        }
    };

    // Scale iterations with workload size so a 10 GiB test doesn't
    // do 40 GiB of I/O per competitor. Small workloads can afford
    // the 3+5 classic shape; ≥ 1 GiB drops to 1+3 (4 passes); ≥ 5 GiB
    // drops to 1+2 (3 passes). Still enough for a stable median.
    let (warmups, runs) = if size_bytes >= 5 * 1024 * 1024 * 1024 {
        (1usize, 2usize)
    } else if size_bytes >= 1024 * 1024 * 1024 {
        (1usize, 3usize)
    } else {
        (3usize, 5usize)
    };
    let mut samples: Vec<Duration> = Vec::with_capacity(runs);
    // Some tools (Robocopy, TeraCopy, FastCopy) copy *into a
    // directory* using the source's filename — they create
    // `DST_DIR/SRC_NAME` rather than honouring our `DST` path
    // verbatim. Clean up that sibling path too, otherwise
    // iterations 2+ detect the file is already there and skip the
    // copy, fabricating "27 GiB/s" numbers.
    //
    // Guard: only clean `alt_dst` if it's actually a *different*
    // path from both `src` and `dst`. When src + dst share a dir
    // (the same-volume same-tempdir baseline) `alt_dst` aliases
    // the source itself, and deleting it between iterations
    // sabotages the copy.
    let alt_dst = dst
        .parent()
        .and_then(|d| src.file_name().map(|n| d.join(n)))
        .filter(|p| p != dst && p != src);
    for i in 0..(warmups + runs) {
        // Reset destination each iteration so every run times the
        // write path, not an in-place truncate-and-skip.
        let _ = fs::remove_file(dst);
        if let Some(alt) = &alt_dst {
            let _ = fs::remove_file(alt);
        }
        let mut args = substitute_argv(comp.argv, src, dst);
        // `args[0]` came from argv's bare name (e.g. `teracopy`);
        // swap in the PATH-resolved absolute path so `Command::new`
        // finds it even when the bare name isn't on PATH.
        args[0] = resolved_exe.clone();
        let t0 = Instant::now();
        let status = Command::new(&args[0])
            .args(&args[1..])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
        let elapsed = t0.elapsed();
        match status {
            Ok(s) if s.success() => {
                if i >= warmups {
                    samples.push(elapsed);
                }
            }
            // Robocopy uses exit codes 0..=7 for "successful /
            // informational"; anything above is a real failure.
            Ok(s) if label == "Robocopy" => {
                if let Some(code) = s.code()
                    && code <= 7
                {
                    if i >= warmups {
                        samples.push(elapsed);
                    }
                } else {
                    return Row {
                        label: label.into(),
                        median: None,
                        note: format!("{exe} exited with {s}"),
                    };
                }
            }
            Ok(s) => {
                return Row {
                    label: label.into(),
                    median: None,
                    note: format!("{exe} exited with {s}"),
                };
            }
            Err(e) => {
                return Row {
                    label: label.into(),
                    median: None,
                    note: format!("{exe} spawn: {e}"),
                };
            }
        }
    }

    samples.sort();
    let median = samples.get(samples.len() / 2).copied();
    let mb_s = median.map(|d| mb_per_sec(size_bytes, d));
    let note = mb_s
        .map(|v| format!("{v:>6.0} MiB/s"))
        .unwrap_or_else(|| "no samples".into());
    Row {
        label: label.into(),
        median,
        note,
    }
}

fn substitute_argv(template: &[&str], src: &Path, dst: &Path) -> Vec<String> {
    let src_dir = src
        .parent()
        .map(|p| p.to_string_lossy().into_owned())
        .unwrap_or_default();
    let src_name = src
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_default();
    let dst_dir = dst
        .parent()
        .map(|p| p.to_string_lossy().into_owned())
        .unwrap_or_default();
    template
        .iter()
        .map(|s| {
            s.replace("{SRC_DIR}", &src_dir)
                .replace("{SRC_NAME}", &src_name)
                .replace("{DST_DIR}", &dst_dir)
                .replace("{SRC}", &src.to_string_lossy())
                .replace("{DST}", &dst.to_string_lossy())
        })
        .collect()
}

fn on_path(exe: &str) -> bool {
    let lookup = if cfg!(windows) { "where" } else { "which" };
    Command::new(lookup)
        .arg(exe)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

fn mb_per_sec(bytes: usize, elapsed: Duration) -> f64 {
    let secs = elapsed.as_secs_f64().max(1e-9);
    (bytes as f64) / (1024.0 * 1024.0) / secs
}

fn cargo_bin() -> String {
    std::env::var("CARGO").unwrap_or_else(|_| "cargo".into())
}

// ---------------------------------------------------------------------
// Reporting
// ---------------------------------------------------------------------

fn format_markdown(rows: &[Row], size_bytes: usize) -> String {
    let size_mb = size_bytes / (1024 * 1024);
    let mut out = String::new();
    out.push_str(&format!(
        "### `xtask bench-vs` ({size_mb} MiB workload)\n\n"
    ));
    out.push_str("| Tool | Median (s) | Throughput | Note |\n");
    out.push_str("| --- | ---: | ---: | --- |\n");
    for r in rows {
        let secs = r
            .median
            .map(|d| format!("{:.2}", d.as_secs_f64()))
            .unwrap_or_else(|| "—".into());
        let tput = r
            .median
            .map(|d| format!("{:.0} MiB/s", mb_per_sec(size_bytes, d)))
            .unwrap_or_else(|| "—".into());
        out.push_str(&format!(
            "| {} | {} | {} | {} |\n",
            r.label, secs, tput, r.note
        ));
    }
    out.push('\n');
    out
}

fn print_markdown(rows: &[Row], size_bytes: usize) {
    println!("{}", format_markdown(rows, size_bytes));
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn substitute_fills_every_placeholder() {
        let src = PathBuf::from("/tmp/foo/bar.bin");
        let dst = PathBuf::from("/other/baz.bin");
        let out = substitute_argv(
            &[
                "tool",
                "{SRC_DIR}",
                "{SRC_NAME}",
                "{SRC}",
                "{DST_DIR}",
                "{DST}",
            ],
            &src,
            &dst,
        );
        assert_eq!(out.len(), 6);
        assert!(out[1].ends_with("foo"));
        assert_eq!(out[2], "bar.bin");
        assert!(out[3].ends_with("bar.bin"));
        assert!(out[4].ends_with("other"));
        assert!(out[5].ends_with("baz.bin"));
    }

    #[test]
    fn mb_per_sec_is_positive() {
        assert!(mb_per_sec(1024 * 1024, Duration::from_secs(1)) > 0.99);
        assert!(mb_per_sec(1024 * 1024, Duration::from_secs(1)) < 1.01);
    }
}
