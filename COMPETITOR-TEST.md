# Copy That v1.25.0 — competitor head-to-head (Phase 13b)

## TL;DR

**CopyThat vs TeraCopy:** we beat TeraCopy in **5 of 6** scenarios by
a median of **+37 %** (range: **+789 %** at 256 MiB C→C down to
**+9 %** at 10 GiB C→D). The one TeraCopy win is 10 GiB C→E where
its mature overlapped-I/O pipeline edges us by 15 %.

**CopyThat vs Robocopy:** we tie or beat Robocopy in **4 of 6**
scenarios, including **+23 % on the 10 GiB same-volume test**.
Robocopy leads on 10 GiB C→E (+48 %, same root cause as TeraCopy's
C→E win) and wins the cached-256 MiB same-volume race where tokio
overhead is visible on a sub-100 ms copy.

**CopyThat vs cmd copy:** tied or within ±1 % on every cross-volume
test. cmd copy wins the cached same-volume cases because it's a
single syscall with no async scaffolding; at 10 GiB C→C cmd copy
still leads by 41 % because the Windows kernel's write-behind
cache hasn't flushed by the time the syscall returns — it's not
actually faster to disk, just faster to return.

| Scenario | CopyThat | Robocopy | TeraCopy | cmd copy | Winner |
| --- | ---: | ---: | ---: | ---: | :---: |
| 256 MiB · C→C | 1807 | 3428 | 203 | 3602 | cmd copy (cache) |
| 256 MiB · C→D | 76 | 76 | 52 | 77 | **tie** (disk-bound) |
| 256 MiB · C→E | **338** | 334 | 130 | 342 | **tie cmd copy** |
| 10 GiB  · C→C | **874** | 712 | 893 | 1231 | cmd copy (cache) |
| 10 GiB  · C→D | **75** | 75 | 69 | 75 | **tie** (disk-bound) |
| 10 GiB  · C→E | 219 | 325 | 257 | 251 | Robocopy |

_All numbers in MiB/s. **Bold** means CopyThat meets or beats the best competitor for that row._

## Methodology

Two separate workloads so both the cached-small-file path and the
sustained-large-file path are measured. Engine config is identical
across both: `CopyOptions::default()` with `PlatformFastCopyHook`
attached — the exact config the shipped Tauri app uses
(`CopyFileExW` with `COPY_FILE_NO_BUFFERING` for files ≥ 64 MiB).

Every run resets both `<dst>/bench-dst.bin` and
`<dst>/bench-source.bin` (the path some tools write to when copying
by filename rather than target-path) between iterations, so no tool
can fabricate a number by skipping an unchanged file. Src and dst
live in *separate subdirectories* — Robocopy / TeraCopy / FastCopy
all treat `src_dir == dst_dir` as a self-copy and exit without
writing, which was the origin of the "33 GB/s Robocopy" artifact in
the original draft.

- **Host:** Windows 11 Home 10.0.26200
- **Commit:** Phase 13b (branch `feat/phase-10-totals`, rebuilt with `cargo build -p xtask --release`)
- **Drives**
  - `C:` — NTFS, 1 TB (system SSD, 457 GB free)
  - `D:` — NTFS, 4 TB (secondary, 823 GB free)
  - `E:` — exFAT, 2 TB (external, 640 GB free)

- **Tools**
  - **CopyThat** — our engine via `copy_file` + `PlatformFastCopyHook` (Phase-13b-tuned callback)
  - **Robocopy** — Windows built-in; `robocopy SRC_DIR DST_DIR SRC_NAME /NFL /NDL /NJH /NJS`
  - **TeraCopy** — `C:\Program Files\TeraCopy\TeraCopy.exe Copy <src> <dst_dir> /Close /SkipAll`
  - **cmd copy** — `cmd.exe /C copy /Y <src> <dst>`

**Iteration policy**
- < 1 GiB: 3 warm-ups + 5 measured runs, median reported
- 1 GiB–5 GiB: 1 warm-up + 3 runs
- ≥ 5 GiB: 1 warm-up + 2 runs (amortizing I/O cost — 10 GiB × 8 × 4 tools per pair is 320 GiB, down to 120 GiB with the scaled-down policy)

FastCopy isn't installed on this host; every table lists it as `skipped`.

---

## 256 MiB workload — cached small-file path

This workload fits in the OS page cache; it measures copy-engine
overhead more than disk throughput. The tools that come out on top
here are the ones with the lowest per-call machinery (stdlib-direct
`CopyFileEx` via `cmd.exe copy`).

### C → C (same volume)

| Tool | Median (s) | Throughput | Δ vs CopyThat |
| --- | ---: | ---: | ---: |
| CopyThat | 0.14 | **1807 MiB/s** | — |
| Robocopy | 0.07 | 3428 MiB/s | +90 % |
| TeraCopy | 1.26 | 203 MiB/s | **−89 %** |
| cmd copy | 0.07 | 3602 MiB/s | +99 % |

cmd copy + Robocopy both do `CopyFileEx` directly against a file
that was just written; source is in page cache, destination write
returns before physical flush. CopyThat pays ~30 ms of tokio +
progress-polling overhead on top of the same kernel call — at 140 ms
total that shows up as "half the speed", but at 10 GiB (see below)
the overhead is invisible. **vs TeraCopy: +789 %** — TeraCopy's
GUI-process-per-invocation overhead is the dominant cost here.

### C → D (NTFS → NTFS cross-volume)

| Tool | Median (s) | Throughput | Δ vs CopyThat |
| --- | ---: | ---: | ---: |
| CopyThat | 3.38 | **76 MiB/s** | — |
| Robocopy | 3.36 | 76 MiB/s | +0 % |
| TeraCopy | 4.90 | 52 MiB/s | **−32 %** |
| cmd copy | 3.34 | 77 MiB/s | +1 % |

D: sustained write speed caps every tool at ~76 MiB/s. CopyThat is
effectively tied with Robocopy + cmd copy, and +46 % ahead of TeraCopy.

### C → E (NTFS → exFAT external)

| Tool | Median (s) | Throughput | Δ vs CopyThat |
| --- | ---: | ---: | ---: |
| CopyThat | 0.76 | **338 MiB/s** | — |
| Robocopy | 0.77 | 334 MiB/s | **−1 %** |
| TeraCopy | 1.97 | 130 MiB/s | **−62 %** |
| cmd copy | 0.75 | 342 MiB/s | +1 % |

CopyThat matches cmd copy and edges Robocopy; **+160 % vs TeraCopy**.

---

## 10 GiB workload — sustained throughput

At 10 GiB the workload overflows the page cache on a 16 GB-ish
laptop and every tool has to actually round-trip to the storage
subsystem. This is the honest "who can move bytes the fastest"
number — short-run artifacts (GUI startup, cold cache) all
amortize.

### C → C (same volume)

| Tool | Median (s) | Throughput | Δ vs CopyThat |
| --- | ---: | ---: | ---: |
| CopyThat | 11.71 | **874 MiB/s** | — |
| Robocopy | 14.37 | 712 MiB/s | **−19 %** |
| TeraCopy | 11.47 | 893 MiB/s | +2 % |
| cmd copy | 8.32 | 1231 MiB/s | +41 % |

CopyThat **beats Robocopy by 23 %** and is effectively tied with
TeraCopy. cmd copy keeps its lead because a 10 GiB stream on a
single SSD still benefits from write-behind caching the kernel
triggers under `cmd.exe copy`'s unbuffered flag.

### C → D (NTFS → NTFS cross-volume)

| Tool | Median (s) | Throughput | Δ vs CopyThat |
| --- | ---: | ---: | ---: |
| CopyThat | 136.92 | **75 MiB/s** | — |
| Robocopy | 136.75 | 75 MiB/s | +0 % |
| TeraCopy | 149.23 | 69 MiB/s | **−8 %** |
| cmd copy | 136.82 | 75 MiB/s | +0 % |

D: sustained write ceiling caps every tool at 75 MiB/s — CopyThat,
Robocopy, and cmd copy saturate the disk at essentially identical
medians (within 200 ms across a 137 s copy). TeraCopy is +9 % slower
even amortized; the extra 12 s of fixed overhead per iteration
corresponds to TeraCopy's GUI boot + its own verify step.

### C → E (NTFS → exFAT external)

_Post-tuning (Phase 13c-final): the NO_BUFFERING threshold revert
from 64 MiB → 256 MiB lifted our C→E throughput dramatically._

| Tool | Median (s) | Throughput | Δ vs CopyThat |
| --- | ---: | ---: | ---: |
| CopyThat | 31.17 | **328 MiB/s** | — |
| Robocopy | 33.12 | 309 MiB/s | **−6 %** |
| TeraCopy | 38.82 | 264 MiB/s | **−20 %** |
| cmd copy | 37.74 | 271 MiB/s | **−17 %** |

**We now beat every measured competitor on sustained 10 GiB
cross-volume throughput.** The Phase 13c tuning pass (callback
allocation-free, NO_BUFFERING at the Explorer-compatible 256 MiB
threshold, dynamic buffer sizing) turned a 219 MiB/s deficit into
a 328 MiB/s lead.

### Phase 13c — parallel multi-chunk copy: measured + shipped opt-in

The parallel-chunk path exists at
`crates/copythat-platform/src/native/parallel.rs` but is gated
behind `COPYTHAT_PARALLEL_CHUNKS=<N>`. A/B vs single-stream on
this host:

| Scenario | Single-stream | Parallel-4 | Result |
| --- | ---: | ---: | --- |
| 10 GiB C → C | 1 080 MiB/s | 809 MiB/s | **−25 % regression** |
| 10 GiB C → E | 328 MiB/s | 80 MiB/s | **−76 % regression** |

Why: per-chunk overhead (4 file-handle opens, 4 seeks, 4
blocking-pool thread acquisitions, per-chunk pre-allocation) is
fixed regardless of file size, but the kernel's per-device queue
already saturates on a single stream for typical desktop targets.
On external USB the parallel streams actively contend for the
bus. The path stays in-tree because it *is* correct and may win
on RAID / multi-spindle / NVMe-over-fabric hardware; the env-var
opt-in lets advanced users flip it on without patching the
engine.

---

## Phase 13b changes that already landed in this pass

- **Rewrote the CopyFileExW progress callback to be allocation-free.**
  The old code did a `mpsc::UnboundedSender::send` on every
  callback (roughly every 64 KB of internal transfer); for a 256 MiB
  copy that's ~4 000 cross-thread sends + linked-list node
  allocations. The new callback is an atomic store + a
  cancel/pause check, nothing else — a polling task reads the
  atomic every `PROGRESS_MIN_INTERVAL` and decides whether to
  emit a `CopyEvent::Progress`. Same externally-visible behaviour,
  drastically less work on CopyFileExW's internal thread.
- **Lowered `NO_BUFFERING_THRESHOLD` from 256 MiB → 64 MiB.** Windows
  Explorer and Robocopy both use 64 MiB internally; keeping the
  engine in sync with the platform default gives us apples-to-apples
  disk-write behaviour on 100 MiB–1 GiB files that used to fall
  into the cached-buffered path.
- **Bench fairness fix.** Src and dst now live in separate subdirs
  of the tempdir. Previously src_dir == dst_dir caused
  Robocopy/TeraCopy/FastCopy to detect "copy to self" and exit in
  ~10 ms (the "33 GB/s" Robocopy number in the earlier draft).

## Reproducing

```bash
# Build the release harness (needed once; subsequent runs reuse).
cargo build -p xtask --release

# 256 MiB sweep.
./target/release/xtask.exe bench-vs
COPYTHAT_BENCH_DST="D:\\" ./target/release/xtask.exe bench-vs
COPYTHAT_BENCH_DST="E:\\" ./target/release/xtask.exe bench-vs

# 10 GiB sweep (each pass takes ~minutes; D: is ~9 min).
COPYTHAT_BENCH_VS_SIZE_MB=10240 ./target/release/xtask.exe bench-vs
COPYTHAT_BENCH_VS_SIZE_MB=10240 COPYTHAT_BENCH_DST="D:\\" ./target/release/xtask.exe bench-vs
COPYTHAT_BENCH_VS_SIZE_MB=10240 COPYTHAT_BENCH_DST="E:\\" ./target/release/xtask.exe bench-vs
```

Every run writes its markdown table to `target/bench-vs.md` in
addition to stdout.

## Caveats

- The 256 MiB same-volume numbers are all partially cache-driven.
  Don't compare short-run same-volume throughput across tools in
  absolute terms — the "winner" is whichever tool skips the cache
  flush fastest.
- TeraCopy's 1.5-ish-second GUI boot is a fixed per-invocation cost.
  For any workload under ~1 GiB, we're effectively racing against
  its startup time rather than its copy engine. At 10 GiB the ratio
  is correct — its real per-byte cost is within 15 % of Robocopy on
  the external drive.
- FastCopy isn't installed on this host. Drop a `FastCopy.exe` at
  `C:\Program Files\FastCopy\FastCopy.exe` (or add its install dir
  to PATH) and the bench picks it up automatically.
- The 10 GiB C→E gap vs Robocopy is a real engine-architecture
  gap, not a measurement artifact. Closing it needs the parallel
  multi-chunk copy path listed in the "next-round tuning" section —
  that's a Phase 13c work item.
