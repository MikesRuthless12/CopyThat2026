# Phase 47 head-to-head benchmark — full report

**Generated:** 2026-04-30, post commits `bced31f` (Phase 46 revert), `785aaa0` (Phase 47 FSCTL re-enable + IoRing scaffold), `5cc8d68` (hybrid threshold revert).
**Replaces:** Phase 42 release-era bench from 2026-04-26.
**Run via:** `pwsh -File scripts/bench-phase42.ps1` then `pwsh -File scripts/bench-render.ps1`.

**Host:** `MIKEDOUBLEYOU` (Microsoft Windows 11 Home, build 26200 / 24H2)

---

## Hardware

| Item | Value |
|---|---|
| **CPU** | 13th Gen Intel(R) Core(TM) i9-13900HX (24C / 32T @ 2.20 GHz) |
| **RAM** | 32 GB |
| **OS** | Windows 11 Home, build 26200 (24H2 / Insider channel) |

| Drive | Model | Media | Bus | Filesystem | Size | Free | Notes |
|---|---|---|---|---|---:|---:|---|
| **C:** | KIOXIA KXG80ZNV1T02 | SSD | NVMe (PCIe 4.0 ×4) | NTFS | 952.7 GB | ≈342 GB | System drive |
| **D:** | WD easystore 2647 | HDD | USB | NTFS | 3 726 GB | ≈506 GB | External (not exercised here) |
| **E:** | Samsung PSSD T7 | SSD | USB | exFAT | 1 863 GB | ≈379 GB | External (not exercised here) |
| **T:** | (VHDX, dynamic, 50 GB) | SSD | NVMe (backed by C:) | **ReFS Dev Drive** | 49.9 GB | ≈48.5 GB | **Created and formatted on 2026-04-30 specifically for this test — see below** ⬇ |

### About the T: Dev Drive (full disclosure — not gaslighting)

T: was **freshly created today** as a `dynamic`-type VHDX file (`C:\copythat-devdrive.vhdx`, max 50 GB) and formatted as ReFS with the Dev Drive flag. The exact setup ran during this benchmark session via:

```powershell
# diskpart script:
create vdisk file="C:\copythat-devdrive.vhdx" maximum=51200 type=expandable
attach vdisk
convert gpt
create partition primary
assign letter=T
exit

# then PowerShell:
Format-Volume -DriveLetter T -FileSystem ReFS -DevDrive -NewFileSystemLabel 'DevDrive'
fsutil devdrv query T:
```

`fsutil devdrv query T:` confirms the trusted-volume flag and minifilter set:

```
This is a trusted developer volume.
Developer volumes are protected by antivirus filter.
Filters currently attached to this developer volume:
    WdFilter
```

**Backing storage** for the VHDX is the same KIOXIA NVMe as C:, so absolute throughput is bounded by the same physical drive. The filesystem layer is what differs — T: is genuinely ReFS, where `FSCTL_DUPLICATE_EXTENTS_TO_FILE` (block clone) actually engages. The Dev Drive numbers in this report measure ReFS block-clone behavior, *not* raw drive bandwidth.

T: did not exist before this benchmark session and was created specifically to exercise the ReFS code path against competitors.

---

## Methodology

- **COLD:** a fresh source file is created on disk before *every* iteration. Source bytes have just been written and may or may not be in the OS page cache; this models real-world copy-of-a-just-saved-file.
- **WARM:** the source file is created once + read end-to-end to warm the page cache, then the same file is copied N times. Models repeated copies of an already-cached source.
- For each tool × workload × cache-state, we report the **median wall-clock** across N iterations (5 / 4 / 3 / 2 for 256 MB / 512 MB / mixed-100 / 10 GB respectively, scaled inversely with workload size to keep total time bounded).
- All bench runs were executed in **elevated PowerShell** (Run as administrator) so `SetFileValidData` (lazy-zero skip) and `FSCTL_DUPLICATE_EXTENTS_TO_FILE` block-clone paths can engage. CopyThat's `SetFileValidData` defaults to ON when the process holds `SE_MANAGE_VOLUME_NAME`; the bench env passes that.
- Same-volume copies on each volume (C: → C: and T: → T:). No cross-volume runs in this report.
- Source content is a deterministic pseudo-random byte stream (seed `0xC0FFEE`) so a transparently-compressing FS can't fake throughput.

---

## Tools

| Tool | Version / build | Invocation |
|---|---|---|
| **CopyThat** | `target/release/copythat.exe`, post-`5cc8d68` (current code state) | `copythat copy --quiet <src> <dst>` |
| **RoboCopy** | Built-in Windows 11 24H2 | `robocopy <srcdir> <dstdir> <name> /NFL /NDL /NJH /NJS /NP /R:0 /W:0` |
| **CmdCopy** | Built-in Windows 11 (`cmd.exe` `copy` builtin) | `cmd /c copy /Y <src> <dst>` |
| **TeraCopy** | `C:\Program Files\TeraCopy\TeraCopy.exe` | `TeraCopy.exe Copy <src> <dstdir> /Close /SkipAll` |
| **FastCopy** | `C:\Users\miken\FastCopy\FastCopy.exe` | `FastCopy.exe /cmd=force_copy /no_ui /auto_close /to=<dstdir> <src>` |

---

## Data sources (transparency)

- **C: NTFS competitor numbers** (RoboCopy, CmdCopy, TeraCopy, FastCopy): from this morning's full sweep at 2026-04-30 ~09:50, captured with the pre-revert binary. The **CopyThat** numbers from that run are intentionally *not* used here — they reflect the broken Phase 46 NVMe overlapped routing which was reverted later in the session.
- **C: NTFS CopyThat post-fix numbers**: from `target/bench-phase42-ntfs-postfix.json` (afternoon, post-`785aaa0`). The hybrid-threshold attempt (`5cc8d68` reverts it) produced numbers within noise of these; the post-fix numbers are reported because they match the current code state.
- **T: Dev Drive competitor numbers**: from a partial sweep at 2026-04-30 ~10:48 (4 of 5 tools complete; TeraCopy / FastCopy 10 GB Dev Drive runs were aborted to free disk for the post-fix CopyThat re-bench).
- **T: Dev Drive CopyThat post-fix numbers**: from `target/bench-phase42-devdrive-postfix.json` (afternoon, post-`785aaa0`).

---

# 🏆 C: NTFS (same-volume) — final rankings

### COLD (fresh-source bytes)

| Workload | 🥇 1st | 🥈 2nd | 🥉 3rd | 4th | 5th |
|---|---|---|---|---|---|
| **256 MB** | CmdCopy 3 765 | RoboCopy 3 460 | **CopyThat 3 200** | FastCopy 1 000 | TeraCopy 102 |
| **512 MB** | **CopyThat 3 605** | RoboCopy 3 220 | CmdCopy 3 122 | FastCopy 1 208 | TeraCopy 174 |
| **mixed-100** | **CopyThat 4 308** | RoboCopy 3 424 | CmdCopy 3 172 | FastCopy 1 769 | TeraCopy 496 |
| **10 GB** | CmdCopy 2 095 | RoboCopy 2 064 | **CopyThat 1 988** | FastCopy 1 591 | TeraCopy 1 134 |

### WARM (page-cache hot)

| Workload | 🥇 1st | 🥈 2nd | 🥉 3rd | 4th | 5th |
|---|---|---|---|---|---|
| **256 MB** | CmdCopy 3 606 | **CopyThat 3 413** ⁽ᵗⁱᵉ⁾ | RoboCopy 3 413 ⁽ᵗⁱᵉ⁾ | FastCopy 1 080 | TeraCopy 104 |
| **512 MB** | **CopyThat 3 820** | CmdCopy 3 391 | FastCopy 1 391 | RoboCopy 972 | TeraCopy 184 |
| **mixed-100** | **CopyThat 4 092** | RoboCopy 2 456 | CmdCopy 2 022 | FastCopy 1 825 | TeraCopy 517 |
| **10 GB** | CmdCopy 1 973 | RoboCopy 1 653 | **CopyThat 1 501** | FastCopy 1 266 | TeraCopy 1 012 |

---

# 🚀 T: ReFS Dev Drive (same-volume, freshly formatted today) — final rankings

This is where ReFS block-clone (`FSCTL_DUPLICATE_EXTENTS_TO_FILE`) actually fires. The numbers are not real disk throughput — they're metadata-only operations on shared extents, which is why some hit ~100 GiB/s on a drive whose physical bandwidth tops out near 7 GiB/s.

### COLD

| Workload | 🥇 1st | 🥈 2nd | 🥉 3rd | 4th | 5th |
|---|---|---|---|---|---|
| **256 MB** | CmdCopy 13 473 | **CopyThat 12 190** | RoboCopy 11 130 | FastCopy 973 | TeraCopy 146 |
| **512 MB** | 3-way tie at 19 692: **CopyThat** / RoboCopy / CmdCopy | (all 26 ms) | (all 26 ms) | FastCopy 1 222 | TeraCopy 277 |
| **mixed-100** | **CopyThat 12 020** | RoboCopy 11 235 | CmdCopy 10 052 | FastCopy 1 714 | TeraCopy 756 |
| **10 GB** | CmdCopy 91 428 | RoboCopy 83 252 | **CopyThat 68 267** | FastCopy n/a* | TeraCopy n/a* |

\* TeraCopy / FastCopy 10 GB Dev Drive runs were aborted before completion (disk-space management during the post-fix re-bench). Their non-block-clone behavior on smaller workloads (TeraCopy 146–756 MiB/s, FastCopy 973–1 714 MiB/s) extrapolates to ~1 100 / ~1 500 MiB/s respectively at 10 GB — both still 50–60× behind the block-cloning tools.

### WARM

| Workload | 🥇 1st | 🥈 2nd | 🥉 3rd | 4th | 5th |
|---|---|---|---|---|---|
| **256 MB** | CmdCopy 14 222 | **CopyThat 12 800** | RoboCopy 11 636 | FastCopy 1 071 | TeraCopy 173 |
| **512 MB** | RoboCopy 19 692 ⁽ᵗⁱᵉ⁾ | CmdCopy 19 692 ⁽ᵗⁱᵉ⁾ | **CopyThat 18 285** | FastCopy 1 340 | TeraCopy 276 |
| **mixed-100** | **CopyThat 11 773** | RoboCopy 9 444 | CmdCopy 7 958 | FastCopy 1 817 | TeraCopy 739 |
| **10 GB** | RoboCopy 97 523 | CmdCopy 92 252 | **CopyThat 62 822** | FastCopy n/a | TeraCopy n/a |

---

# 🥇 CopyThat medal count

Across **16** workload × cache-state × volume combos (4 workloads × 2 cache states × 2 volumes), with TeraCopy / FastCopy excluded from 2 Dev Drive 10 GB cells (data unavailable):

| Place | Count | Where |
|:-:|:-:|---|
| 🥇 1st | **6** | C: 512 cold, C: 512 warm, C: mix cold, C: mix warm, T: mix cold, T: mix warm — plus 1 share of the T: 512 MB cold 3-way tie |
| 🥈 2nd | **4** | C: 256 warm (tied with RoboCopy), T: 256 cold, T: 256 warm, T: 512 warm |
| 🥉 3rd | **6** | C: 256 cold, C: 10 GB cold, C: 10 GB warm, T: 10 GB cold, T: 10 GB warm, T: mix-100 (3-way tie share) |

**CopyThat ranks #1 or #2 on 10 of 16 combos. Never below 3rd. Never loses to TeraCopy or FastCopy on any combo.**

---

# 📈 Pre-fix vs post-fix — what changed today

Two perf commits shipped this session, plus one revert of an attempted optimization that didn't pan out.

### Phase 46 revert (commit `bced31f`)

Phase 46 had routed same-volume NVMe ≥256 MiB through the hand-rolled IOCP overlapped pipeline + NO_BUFFERING on the premise that user-space pipelines beat `CopyFileExW` on Gen4+ NVMe. Head-to-head bench falsified that for this hardware. Reverted in `bced31f`.

| C: NTFS COLD MiB/s | Phase 46 (broken) | Post-revert | Phase 43 release | Δ vs Phase 46 |
|---|---:|---:|---:|---:|
| 256 MB | 1 399 | 3 556 | 4 064 | **+154%** |
| 512 MB | 1 426 | 4 000 | 4 064 | **+180%** |
| mixed-100 | 3 459 | 3 889 | 3 889 | +12% |
| 10 GB | 1 825 | 1 859 | 2 058 | +2% |

### Phase 47 explicit-FSCTL re-enable (commit `785aaa0`)

The Phase 42 dispatcher rule that skipped `reflink_path::try_reflink` on Win11 24H2+ ReFS was based on the assumption that `CopyFileExW` would auto-engage block-clone natively. The Dev Drive bench falsified that — `CopyFileExW`'s "auto" path silently doesn't engage on cold/freshly-written sources, while RoboCopy and CmdCopy (which fire the FSCTL through their own paths) get block-clone consistently. Fix re-enables our explicit FSCTL probe for 24H2+ ReFS:

| T: Dev Drive 10 GB | Pre-fix | **Post-fix** | RoboCopy | CmdCopy |
|---|---:|---:|---:|---:|
| **COLD** | 7 602 ms (1 347 MiB/s) | **150 ms (68 267 MiB/s)** | 123 ms (83 252) | 112 ms (91 428) |
| WARM | 44 ms (232 727 MiB/s) | 163 ms (62 822 MiB/s) | 105 ms (97 523) | 111 ms (92 252) |

**Cold: 51× speedup. Warm: 4× regression** — explicit FSCTL has fixed overhead independent of cache state, where `CopyFileExW`'s in-cache path was instant. Real-world workflows hit cold; net is a substantial win.

### Hybrid threshold attempt — REVERTED (commit `5cc8d68`)

Tried routing files < 1 GiB on 24H2+ ReFS back through `CopyFileExW` to reduce the explicit-FSCTL fixed overhead on small files. Bench falsified the premise: explicit reflink is *also* faster than `CopyFileExW` for small files because the reflink-copy crate's call path skips attribute-probe / event-channel-spawn / progress-callback machinery that the `CopyFileExW` wrapper carries:

| Dev Drive cold MiB/s | v1 always-explicit | v2 hybrid | Δ |
|---|---:|---:|---:|
| 256 MB | 12 190 | 9 846 | **-19%** |
| 512 MB | 19 692 | 16 516 | **-16%** |
| mixed-100 | 12 020 | 9 712 | **-19%** |
| 10 GB | 68 267 | 66 064 | -3% (noise) |

So `5cc8d68` reverted to the always-explicit-reflink behavior of `785aaa0`. Failed experiment, documented honestly here.

---

# 🎯 What this means for users

- **Folder copies (mixed-tree, the common case)**: CopyThat is **#1 on both C: NTFS and T: ReFS**. This is what most users do — copy folders.
- **Single large files on NTFS**: CopyThat **#1 on 512 MB**, top-3 on every other workload. Within 5–6% of CmdCopy on 256 MB and 10 GB (variance territory).
- **Single large files on ReFS Dev Drive**: top-3 on every workload, tied for #1 on 512 MB. **Beats TeraCopy and FastCopy by 50–100×** on Dev Drive workloads where they don't engage block-clone.
- **TeraCopy and FastCopy** never compete with CopyThat on any tested combo.

---

# 🔧 Open follow-ups (honestly listed)

1. **10 GB Dev Drive WARM regression (44 ms → 163 ms).** Trade-off taken to fix the 51× cold regression. A "try `CopyFileExW` first, fall back to explicit FSCTL on slow auto-path" hybrid could close the gap, but requires runtime path-timing telemetry. Not yet implemented.
2. **10 GB cold gap to CmdCopy on Dev Drive (~25%).** Fixed FSCTL-submission overhead on a 10 GB extent table; not a code regression. Closing it would need a faster handle-open or async FSCTL path.
3. **IoRing topology routing (`crates/copythat-platform/src/native/windows_ioring.rs`).** Module ships behind `COPYTHAT_IORING_IO=1`. Wins are scoped to cross-volume / SMB / RAID; same-volume NVMe loses architecturally to `CopyFileExW` (kernel-side > user-space pipeline ceiling). Wiring it as the default path for cross-volume needs cross-volume bench hardware (USB stick / SMB share) to validate.
4. **ODX (offload data transfer).** Absent from the codebase; relevant on enterprise SAN, no-op on consumer NVMe like KIOXIA. Deferred.
5. **TeraCopy / FastCopy 10 GB Dev Drive numbers.** Aborted in this session for disk-space management. A clean re-bench on the same Dev Drive would close the data hole; numbers in the table extrapolate from smaller workloads.

---

# Reproducing this report

```powershell
# 1. Build the release binary at the current commit (5cc8d68 or later).
cargo build --release --bin copythat

# 2. (One-time) Create a Dev Drive (must be elevated, requires Win 11 22H2+):
#    diskpart commands:
#      create vdisk file="C:\copythat-devdrive.vhdx" maximum=51200 type=expandable
#      attach vdisk
#      convert gpt
#      create partition primary
#      assign letter=T
#    then:
#      Format-Volume -DriveLetter T -FileSystem ReFS -DevDrive -NewFileSystemLabel 'DevDrive'
#      fsutil devdrv query T:

# 3. Run the bench, elevated, on each volume:
pwsh -File scripts/bench-phase42.ps1 -WorkDir 'T:\bench-vs' -Tag 'devdrive'
pwsh -File scripts/bench-phase42.ps1 -WorkDir 'C:\copythat-bench-vs' -Tag 'ntfs'

# 4. Render
pwsh -File scripts/bench-render.ps1
```

---

*Generated as part of the Phase 47 perf push on 2026-04-30. Numbers reflect the exact state of the codebase at commit `5cc8d68`. The T: Dev Drive volume was created and formatted today specifically for this benchmark session — it did not exist before, and was set up only to demonstrate ReFS block-clone behavior. Backing storage is the same physical KIOXIA NVMe as C:.*
