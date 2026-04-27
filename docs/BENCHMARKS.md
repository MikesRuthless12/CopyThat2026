# Copy That v1.0.0 — performance benchmarks

Phase 13 shipped a Criterion-backed bench harness under
`crates/copythat-core/benches/copy_bench.rs` plus three xtask
wrappers (`bench`, `bench-ci`, `bench-vs`). This doc captures the
methodology + the most recent measured numbers on reference
hardware. Phase 13b lands the actual measurements and any
resulting default-value changes (e.g. a bumped `DEFAULT_BUFFER_SIZE`).

## Methodology

All measurements use Criterion's median + IQR over 10 samples with
a 1-second warm-up and 3-second measurement window in CI mode.
Workloads sit inside a `tempfile::tempdir()` on the same volume so
the variance comes from the copy engine, not cross-device DMA.
Override the destination with `COPYTHAT_BENCH_DST=/path/on/other/volume`
to measure cross-device cases.

The benches are *deterministic*: the synthetic payload is a
256-byte rolling pattern (see `write_random_file`), not random
bytes, so a filesystem with transparent compression (ZFS, APFS
under some configs) cannot silently inflate throughput by eliding
the write pass. If you run against a compressing FS, interpret the
numbers as an *upper bound* of what the engine can do on that
stack, not as real-world throughput.

### Workloads

| Name | Full-size | CI-mode (`COPYTHAT_BENCH_CI=1`) |
| --- | --- | --- |
| `single_huge_file` | 1 × 1 GiB | 1 × 100 MiB |
| `buffer_size_sweep` | 5 × 200 MiB (64 KiB / 256 KiB / 1 MiB / 4 MiB / 16 MiB buffers) | 5 × 50 MiB |
| `many_small_files` | 2 000 files cycling 10 KiB / 100 KiB / 1 MiB / 10 MiB | 200 files same cycle |
| `mixed_tree` | 4 subdirs × 12 files, sizes cycling 10 KiB / 100 KiB / 1 MiB / 10 MiB / 50 MiB / 250 MiB | 2 subdirs × 6 files |

Phase 13b-2 rebalanced the mixed-size workloads to exercise every
bucket in `CopyOptions::buffer_size_for_file`: tiny dotfile-sized
entries that default-shrink the buffer, medium-band entries that
run the 1 MiB default verbatim, and 50 – 250 MiB entries that
cross into the 4 MiB large-file band. The old 16 KiB-only shape
was measuring only the tiny bucket.

### CI regression gate

`xtask bench-ci` is wired into the Phase 13 smoke test at
`tests/smoke/phase_13_bench.rs`. The smoke test runs a scaled-down
single-file copy (100 MiB) and asserts a throughput floor of
20 MiB/s — conservative enough to tolerate the slowest shared CI
runner, strict enough to trip a 10× regression. Any change that
drops below 20 MiB/s on stock GitHub runners is almost certainly a
bug (the floor is about 1/10th of what even a spinning-disk laptop
delivers).

The 0.95× TeraCopy and 1.0× `cp` / `ditto` gates from the Phase 13
guide are enforced manually at release time — CI can't
meaningfully compare against external tools whose performance
varies more than our own engine.

## Running locally

```bash
# Full-size run. Takes several minutes on a normal SSD.
cargo bench -p copythat-core --bench copy_bench
# Same thing through xtask (identical output):
cargo run -p xtask --release -- bench

# CI-scaled run. Finishes in ~60 s.
cargo run -p xtask --release -- bench-ci

# Compare against any competitor tools we can detect on PATH
# (Robocopy, TeraCopy, FastCopy on Windows; cp / rsync / ditto
# on Unix). Non-present tools are skipped — the runner never
# fails just because e.g. TeraCopy isn't installed.
cargo run -p xtask --release -- bench-vs

# Larger bench-vs workload:
COPYTHAT_BENCH_VS_SIZE_MB=1024 cargo run -p xtask --release -- bench-vs
```

Outputs go to stdout (Criterion's normal terminal format) plus a
Markdown table at `target/bench-vs.md` for the `bench-vs`
subcommand.

## Results

<!--
  Phase 13b fills the tables below with measured numbers. Baseline
  host: the commit message on the Phase 13b commit names the
  machine + OS + filesystem + kernel / webview-2 / WKWebView / GTK
  versions used. Historical rows are retained (append, don't
  replace) so a future regression shows up as a visible delta.
-->

### Streaming tree enumeration (Phase 14-16 notes)

The engine's `copy_tree` no longer builds a Plan of every file in
memory before the first copy starts. A `spawn_blocking` walker
pushes 100 000-entry `Plan` chunks through a bounded `mpsc` channel
of capacity 2; the async dispatcher `mkdir`s the chunk's dirs +
spawns its file copies under the existing concurrency semaphore
as each chunk arrives. Walker + copies overlap, memory footprint
stays at ~50 MB per chunk regardless of tree size, and the
previous drive-root `tree-too-large` guard was removed — a 100 M
file / 2 PB source is the same workload shape as a 10-file one.

The UI's Activity panel caps at **250 000 rows**; beyond that,
oldest completed rows age out so in-flight rows stay visible.
Engine still copies every file regardless of the UI cap. Other
fixes that landed alongside the streaming refactor:

- `TreeEnumerating` ticks every 500 entries during the walk so
  the UI counter advances visibly during enumeration
- Per-file `Started` / `Completed` events no longer clobber the
  tree-level `files_done` / `bytes_done` (that was the cause of
  the oscillating `0 / N ↔ X / N` counter)
- Walk tolerates `PermissionDenied` on system folders
  (`$RECYCLE.BIN`, `System Volume Information`, …) — skips + logs
  instead of aborting the whole job
- Cross-volume reflink guard skips the reflink syscall when src
  and dst live on different volumes (per-volume-id check)
- Windows source open retries on `ERROR_SHARING_VIOLATION` with
  `FILE_SHARE_READ|WRITE|DELETE`, so locked files (open logs,
  loaded DLLs, Office docs) copy without admin

### Buffer-size sweep (`buffer_size_sweep` group, CI-scale 50 MiB)

Confirms `DEFAULT_BUFFER_SIZE = 1 MiB`. Every non-1-MiB size is
slower, so the default stays put — no change to the `DEFAULT_BUFFER_SIZE`
constant in `copythat-core::options` or the mirror in
`copythat-settings::defaults`.

| Host | 64 KiB | 256 KiB | 1 MiB | 4 MiB | 16 MiB | Winner |
| --- | ---: | ---: | ---: | ---: | ---: | --- |
| Windows 11 · NTFS C:  | 1.56 GiB/s | 2.22 GiB/s | **2.41 GiB/s** | 1.51 GiB/s | 1.27 GiB/s | 1 MiB |

Observations:
- Below 256 KiB, per-syscall overhead dominates.
- Above 1 MiB, CPU cache locality degrades and writes stop
  overlapping with reads inside `BufWriter`; 16 MiB is actually
  ~half the throughput of 1 MiB.

### Head-to-head (`xtask bench-vs`)

Full results live in `COMPETITOR-TEST.md` at the repo root — two
complete tables (256 MiB + 10 GiB) across C→C, C→D, and C→E.
Highlights:

- **CopyThat vs Robocopy**: tie or beat Robocopy in 4 of 6
  scenarios; notably +23 % on 10 GiB C→C (874 MiB/s vs 712 MiB/s).
- **CopyThat vs TeraCopy**: 5 of 6 wins, median +37 % (max +789 %
  at 256 MiB C→C). Lose only on 10 GiB C→E by 15 %.
- **CopyThat vs cmd copy**: tied on every cross-volume test (±1 %).
  cmd copy leads same-volume cached copies because the Windows
  kernel's write-behind cache hasn't flushed by the time the
  `copy` syscall returns — not actually faster to disk, just
  faster to return.

### Phase 13b engine tuning

- **CopyFileExW progress callback is now allocation-free.** Previously
  the per-chunk callback did an `mpsc::UnboundedSender::send` (≈
  4 000 cross-thread sends per 256 MiB copy). The new callback is
  atomic-only (cancel-check → pause-check → atomic-store), and a
  polling task reads the atomic every `PROGRESS_MIN_INTERVAL` for
  emission. Same externally-visible behaviour, measurable win on
  sustained large-file copies.
- **`NO_BUFFERING_THRESHOLD` dropped from 256 MiB → 64 MiB** to
  match Robocopy's internal behaviour. Keeps the engine and the
  competitor it benches against on the same cache-bypass
  boundary, so head-to-head numbers reflect only engine
  differences.
- **Bench fairness fix.** Src and dst now live in separate
  subdirectories of the tempdir. Previously `src_dir == dst_dir`
  caused Robocopy / TeraCopy / FastCopy to treat the "copy" as a
  self-copy and exit in ~10 ms — that's the origin of the
  "33 GB/s Robocopy" number in the initial report draft.

## Tuning notes

- **Default buffer size.** 1 MiB is a sensible floor — below that,
  `tokio::fs` syscall overhead dominates; above 4 MiB, page-cache
  eviction on the destination starts costing more than batched
  writes save. The sweep in `buffer_size_sweep` re-measures this
  on every Phase 13 run; if the observed winner is 4 MiB or
  16 MiB by a ≥ 5 % margin, Phase 13b bumps the constant.
- **Reflink preference.** Default is `Prefer`; `xtask bench` does
  not isolate the reflink path (that's Phase 6 territory —
  `cargo test -p copythat-platform`). `bench-vs`'s copy-to-same-
  volume run exercises reflink on Btrfs / XFS-with-reflink / APFS
  / ReFS implicitly.
- **Concurrency.** `copythat-platform::recommend_concurrency`
  picks a worker count per storage class. The benches use the
  default; a future Phase 13 pass can add a concurrency sweep if
  the per-platform tuning drifts.

## Phase 13b follow-ups

- [ ] Run `xtask bench-ci` on the reference Windows host; fill the
      "Primary workloads" table.
- [ ] Run `xtask bench-vs`; fill the "Head-to-head" table.
- [ ] If `buffer_size_sweep` shows a clear winner ≠ 1 MiB, bump
      `DEFAULT_BUFFER_SIZE` + `copythat-settings::defaults`; record
      the before/after numbers here.
- [ ] Add Linux kernel source as a canonical corpus (deferred —
      requires a reliable mirror + CI bandwidth budget).
