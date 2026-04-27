# Performance tuning

Copy That ships sensible defaults that match Phase 13b benchmarks
on a typical Windows 11 NVMe machine. If you have unusual hardware
(slower drive, RAID array, NVMe-over-fabric, very large RAM, etc.),
the env vars below let you A/B-test alternative paths without
recompiling.

All env vars are read once per process (cached via `OnceLock`), so
set them before launching `copythat-ui.exe` or invoking the
`copythat` CLI.

## Quick recipes

**Default — already very fast on most hardware:**
```
copythat copy <src> <dst>
```
Uses the parallel-chunk path with 4 concurrent workers and 1 MiB
buffers per chunk. Hits ~1657 MiB/s on a 990 PRO 10 GiB same-volume
copy.

**NVMe Gen 4/5 with deep queues (saturate the device):**
```
COPYTHAT_PARALLEL_CHUNKS=8 copythat copy <src> <dst>
```
Pushes the engine's read+write pipeline depth from 4 → 8 in-flight
operations. Per AnandTech, sequential I/O on a Samsung 990 PRO
saturates around QD16-32 — `8` is the per-chunk default in the
parallel path; combined with the chunk's internal 1 MiB blocks
that's already 8 in-flight 1 MiB requests.

**Slow disk / spinning HDD:**
```
COPYTHAT_PARALLEL_CHUNKS=1 copythat copy <src> <dst>
```
Forces single-stream copy. On rotational media, parallel reads
fight for the head and regress (Phase 13c data showed −76 % on a
USB-attached HDD). The default already auto-clamps to 1 worker on
detected rotational media via `helpers::recommend_concurrency`,
but you can force it explicitly.

**Admin user, willing to skip NTFS lazy-zero (faster, advisory):**
```
COPYTHAT_SKIP_ZERO_FILL=1 copythat copy <src> <dst>
```
After pre-allocating the destination, calls `SetFileValidData` so
NTFS skips its lazy-zero pass over the pre-allocated extent.
Requires `SE_MANAGE_VOLUME_NAME` privilege (admin). The pre-
allocation extent contains whatever the disk's prior data was;
Copy That's worker writes overwrite every byte, so the only risk
is a Copy That crash mid-copy briefly leaving uninitialised
clusters readable to a process running as the same user. If you're
not comfortable with that trade-off, leave this off.

## Full env-var reference

### `COPYTHAT_PARALLEL_CHUNKS=<N>`

Number of in-flight read+write chunks for the parallel-chunk path.

- `0` or `1` — disable; use single-stream `CopyFileExW`
- `2..=16` — clamped; `4` is the Phase 13b default
- Unset — uses `4`

Each chunk owns its own pair of file handles + its own ~1 MiB
buffer (calculated as `CopyOptions::buffer_size_for_file / N`,
floored at 64 KiB).

### `COPYTHAT_OVERLAPPED_IO=<bool>`

Switches the engine onto the experimental overlapped-I/O fast path
(`copythat-platform/src/native/windows_overlapped.rs`) for files
≥256 MiB. Uses `FILE_FLAG_OVERLAPPED | FILE_FLAG_NO_BUFFERING` and
an IOCP loop with N in-flight buffers.

- `1` / `true` / `on` — enable
- everything else (default) — disable, use parallel-chunk path

On the test rig the parallel-chunk path edges this by 1.3 %; we
keep overlapped as a fallback for hardware where it may win
(RAID-0, NVMe-over-fabric, distributed FS).

### `COPYTHAT_OVERLAPPED_BUFFER_KB=<N>`

When the overlapped path is engaged, override the per-slot buffer
size. Default 1024 (1 MiB). Common values: 256, 1024, 4096, 8192.

### `COPYTHAT_OVERLAPPED_SLOTS=<N>`

When the overlapped path is engaged, override the in-flight slot
count. Default 4. Clamped to `1..=64`. Larger values can saturate
deeper NVMe queues; smaller values reduce memory pressure.

### `COPYTHAT_OVERLAPPED_NO_BUFFERING=<bool>`

When the overlapped path is engaged, control whether
`FILE_FLAG_NO_BUFFERING` is set on src/dst handles.

- `0` / `false` / `off` — drop the flag, use cached I/O. Faster on
  workloads that fit in OS page cache.
- everything else (default) — keep the flag, direct DMA. Faster on
  workloads larger than RAM.

### `COPYTHAT_NO_BUFFERING_THRESHOLD_MB=<N>`

For the **default** `CopyFileExW` path, override the file-size
threshold for setting `COPY_FILE_NO_BUFFERING`. Default 256 MiB —
files smaller than this use the buffered path (cache hits dominate),
files at-or-above use unbuffered.

### `COPYTHAT_SKIP_ZERO_FILL=<bool>`

After pre-allocating dst via `SetEndOfFile` (parallel-chunk and
overlapped paths), call `SetFileValidData` to skip NTFS's lazy
zero-fill of the unwritten extent. **Requires admin** —
specifically the `SE_MANAGE_VOLUME_NAME` privilege.

- `1` / `true` / `on` — attempt SetFileValidData (best-effort:
  silently no-ops on `ERROR_PRIVILEGE_NOT_HELD`)
- everything else (default) — do not attempt; NTFS lazy-zeros on
  writes (slightly slower)

Phase 39 research showed this is +5-15 % on cold writes on a fast
NVMe. Off by default because of the security implication: an admin
opting into this acknowledges that the pre-allocated extent
briefly contains whatever bytes were on those clusters before.

### `COPYTHAT_BENCH_VS_SIZE_MB=<N>`

For `xtask bench-vs` only — workload size in MiB. Default 256.
Override to `10240` for the canonical 10 GiB run that matches
`COMPETITOR-TEST.md`.

### `COPYTHAT_BENCH_VS_DST=<path>`

For `xtask bench-vs` only — override the destination directory.
Useful for testing cross-volume scenarios (`D:\bench-dst`,
`E:\bench-dst`).

### `COPYTHAT_PARALLEL_BUDGET_BYTES=<N>`

Override the total memory budget for the parallel-chunk path. The
per-chunk buffer is `budget / num_chunks`, floored at 64 KiB. Used
mainly for A/B testing buffer sizes against fixed memory.

### `COPYTHAT_SUPPRESS_ZFS_WARNING=<bool>`

Silences the one-shot ZFS-version warning that the reflink path
emits when the destination filesystem is ZFS but the host may be
running a pre-OpenZFS-2.2 release without `clone_range` support.
The warning is informational — copies still succeed by falling back
to byte-copy — but it can become repetitive in scripted workflows
that already know the dataset version.

- `1` — suppress the warning (silent)
- everything else (default) — emit the warning at most once per
  process to stderr

Useful for CI runners and automated test harnesses where the noise
buries real diagnostic output. Doesn't affect any other warning
surface.

## Verifying the path you're on

`xtask bench-vs` reports the chosen strategy in its output line:

- `CopyFileExW` — single-stream path
- `Parallel-N-chunks` — parallel-chunk path with N workers
- `Overlapped-N-slots-MMiB` — overlapped path with N slots × M MiB
- `Reflink` — block-clone fast path on ReFS / Dev Drive

## Hardware-specific recommendations

| Hardware | Recommended overrides |
|----------|-----------------------|
| Default NVMe Gen 3/4, ~16-64 GB RAM | (none — defaults are tuned for this) |
| NVMe Gen 5, deep queues, ≥64 GB RAM | `COPYTHAT_PARALLEL_CHUNKS=8` |
| ReFS / Win 11 Dev Drive | (none — block clone fires automatically) |
| RAID-0 array, multiple spindles | `COPYTHAT_OVERLAPPED_IO=1 COPYTHAT_OVERLAPPED_SLOTS=8` |
| External USB HDD / spinning media | `COPYTHAT_PARALLEL_CHUNKS=1` (or trust auto-detect) |
| Admin + privacy-OK + max throughput | add `COPYTHAT_SKIP_ZERO_FILL=1` |

## Phase 39 research

For the full research underlying these defaults, see
[`docs/RESEARCH_PHASE_39.md`](RESEARCH_PHASE_39.md). TL;DR: on
plain NTFS we're at ~25-30 % of the user-mode ceiling; on ReFS /
Dev Drive we're metadata-only. The remaining wins are mainly in
queue-depth tuning + zero-fill skipping; everything else
(scatter/gather, memory-mapped, IoRing, DirectStorage,
kernel-mode drivers) is either marginal, the wrong tool, or not
viable for an indie distribution.

## Microsoft Defender / antivirus exclusions (Phase 42)

**Defender real-time scanning double-scans every byte of every
copy** (once on read, once on write). On bulk copy workloads this is
frequently the dominant slowdown — sometimes more than the disk
itself. The Phase 42 swarm research traced ~30-50 % throughput
recovery on large workloads after adding the destination tree as a
Defender path exclusion.

**Copy That will never disable AV silently.** This is a manual,
opt-in tuning step for users who have explicitly decided the
workload is from a trusted source.

### How to add a path exclusion (Windows 11)

1. Open **Windows Security** (Start → "Windows Security").
2. Go to **Virus & threat protection** → **Manage settings**.
3. Scroll to **Exclusions** → **Add or remove exclusions**.
4. Click **Add an exclusion** → **Folder**, then pick the
   destination folder for the copy (e.g. your `D:\Backups\`).
5. The exclusion is effective immediately.

Or via PowerShell (admin):
```powershell
Add-MpPreference -ExclusionPath "D:\Backups"
```

To remove it after the copy:
```powershell
Remove-MpPreference -ExclusionPath "D:\Backups"
```

### When to use this

- ✅ Bulk copying media archives, build outputs, VM disks, or
  backups from a local source you trust.
- ✅ Restoring from a known-good local backup.
- ❌ Copying anything you downloaded today.
- ❌ Copying from a network share whose contents you didn't put
  there.

For temporary exclusions you can wrap a single copy session, the
PowerShell add/remove pair above is the right tool. Don't leave
permanent exclusions on directories where untrusted files might
land.

### Other AV products

The same principle applies to ESET, Bitdefender, Norton, Sophos,
McAfee, Kaspersky, etc. — every behaviour-monitoring AV does
on-access scans. Add the destination directory to the product's
"trusted folders" / "scan exclusions" list before bulk copies, and
remove the exclusion afterward.

## Phase 42 — Win11+ baseline

CopyThat 1.25.0 onward targets **Windows 11+ only** (build 22000+).
Win10 was end-of-life October 2025. Several runtime-detected paths
ride on the new floor:

- **`COPY_FILE_REQUEST_COMPRESSED_TRAFFIC`** — engaged automatically
  when the destination is a UNC path (`\\server\share`). Free win
  on slow remote links via SMB v3.1.1 traffic compression.
- **Win11 24H2 native block cloning inside `CopyFileExW`** —
  on same-volume ReFS / Dev Drive copies, the OS itself fires
  `FSCTL_DUPLICATE_EXTENTS_TO_FILE` and the copy becomes a
  metadata-only operation (~94 % time savings on 1 GB files per
  Microsoft's own benchmarks).
- **Adaptive `COPY_FILE_NO_BUFFERING` threshold** — the cutoff is
  now `max(256 MiB, min(2 GiB, free_phys_ram / 4))` instead of a
  static 256 MiB. On RAM-constrained hosts the unbuffered path
  engages earlier (avoiding SuperFetch standby-list pollution); on
  RAM-rich hosts it caps at 2 GiB so a 64 GiB host doesn't try to
  buffer a 16 GiB file.
- **Storage topology probe** — `IOCTL_STORAGE_QUERY_PROPERTY` is
  used at copy start to detect bus type (NVMe / SATA / USB / RAID
  / iSCSI / VHDX) and seek penalty (HDD vs SSD), with results
  cached per-volume.

See [`docs/RESEARCH_PHASE_42.md`](RESEARCH_PHASE_42.md) for the
full research deep-dive (~270 sources across 10 specialist
research agents) and the gap-list audit.
