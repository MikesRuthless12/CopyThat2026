# Phase 39 — File-copy throughput research (April 2026)

Two parallel research passes (Win32/kernel/NTFS internals; hardware
NVMe / DirectStorage / IoRing) covered every major user-mode + kernel
path on Windows 11 NVMe. Goal was to identify everything that could
move the needle vs Robocopy's published 1305-1700 MiB/s on a 10 GiB
same-volume copy.

## TL;DR — what's worth doing

| # | Technique | Expected Δ | Engineering | Status |
|---|-----------|------------|-------------|--------|
| 1 | **Block clone via `FSCTL_DUPLICATE_EXTENTS_TO_FILE`** | 10×-100× on ReFS / Win 11 Dev Drive (metadata-only) | 4-8 h | Phase 39 — **DO** |
| 2 | **`SetFileValidData` after `SetEndOfFile`** (admin opt-in) | +5-15% on cold writes (skips zero-fill) | 2-4 h | Phase 39 — **DO** behind admin/opt-in |
| 3 | **Tune IOCP / overlapped queue depth to 8-32** | +10-40% on Gen 4/5 NVMe (saturates QD16) | 4-8 h | Phase 39 — **DO** (env-var tunables already shipped) |
| 4 | **`SetEndOfFile` pre-allocation** before first write | +2-8% (less fragmentation) | 1-2 h | Phase 38 — already in parallel.rs and overlapped path |
| 5 | **GUI shell overhead reduction** — journal fsync rate, event channel batching | UI 614 → ~1500 MiB/s (close to engine) | 4-8 h | Phase 39 — **DO** (highest user-visible win) |
| 6 | **`std::thread::spawn` for parallel chunks** (vs `spawn_blocking`) | +0-10% (guaranteed dedicated threads) | 2-4 h | Phase 39 — **DO** |
| 7 | **`bench-vs` ordering randomization** | Fairer competitor numbers (currently 30-50% biased against first tool) | 3-6 h | Phase 39 — **DO** |
| 8 | **Switch to `IoRing` (Win11 22H2+)** | +2-3% per Yarden Shafir's official benchmarks | 16-30 h | **defer** (small gain, young API, Win11-only) |
| 9 | **`ReadFileScatter` / `WriteFileGather`** | +0-5% (designed for SQL random I/O, not sequential) | 8-16 h | **skip** (wrong tool) |
| 10 | **Memory-mapped copy** (`MapViewOfFile`) | -10% to 0% (Raymond Chen confirms write-fault read penalty) | 8 h | **skip** |
| 11 | **DirectStorage for general copy** | not applicable | — | **skip** (GPU-target API only) |
| 12 | **Custom NTFS minifilter / kernel driver** | possibly +20-40% on uncached cold reads | 200-500 h, EV cert + WHQL ~$300-500/yr ongoing | **skip** (kills end-user UX, not viable for indie) |
| 13 | **SPDK / user-mode NVMe driver** | drive limit (~7 GB/s) | 200+ h | **skip** (unbinds device from Windows; no NTFS) |
| 14 | **AVX-512 non-temporal stores in user buffer** | irrelevant under `NO_BUFFERING` (DMA bypasses CPU memcpy entirely) | 8-16 h | **skip** |

## Robocopy's actual internals

Public docs + the Robocopy team's archive notes confirm the
implementation is **vanilla Win32**: `CopyFileEx` per file +
`FindFirstFile`/`FindNextFile` for traversal. `/MT[:n]` parallelizes
**across files only** (not within a single file). `/J` is the
unbuffered flag (passes `COPY_FILE_NO_BUFFERING` through). It does
**not** call `SetFileValidData`, **not** issue
`FSCTL_DUPLICATE_EXTENTS`, and **not** split single files. There is
no Russinovich / SysInternals deep-dive — Robocopy was written by
Kevin Allen for the NT4 Resource Kit and absorbed into Vista.

That's important: **our existing parallel-chunk path
(`crates/copythat-platform/src/native/parallel.rs`) already does
things Robocopy does not.** The win we're chasing on plain NTFS is
queue-depth tuning + zero-fill skipping + GUI overhead, not anything
architecturally absent from our engine.

## FastCopy's design (for comparison)

FastCopy bypasses Win32 and calls NT-layer APIs directly
(`NtCreateFile`, etc.), runs read and write on separate threads with
a pipelined buffer queue, uses `NO_BUFFERING`, and picks a per-pair
strategy by whether src/dst share a physical device. This is
essentially what we already do, with the exception of NT-layer calls
— which buy nothing on modern Windows where the NT layer is a thin
shim over the Win32 entry points anyway.

## Hard ceilings

**User-mode Win32 ceiling on NTFS same-volume large file:** the
slower of (NVMe sustained sequential write bandwidth) and (NTFS
metadata + zero-fill cost). On a 990 PRO that's **~6 GB/s with
QD16-32, `NO_BUFFERING`, overlapped**. Our 1657 MiB/s is ~25-30% of
that ceiling — significant headroom from items #2, #3 above.

**Crossing 2× Robocopy on plain NTFS is unrealistic without CoW.**

**On ReFS / Win11 Dev Drive, `FSCTL_DUPLICATE_EXTENTS_TO_FILE` gives
effectively unbounded speedup** vs Robocopy because Robocopy still
ships pre-CoW logic. `CopyFileEx` only auto-CoWs on Win11 24H2+, and
even then the FSCTL is more reliable. On supported volumes block
cloning is **metadata-only** — VCN→LCN map edit, not byte movement.

**Why kernel-mode is not the answer for an indie copy tool:**

- EV signing cert: $250-500/yr ongoing.
- WHQL submission process: weeks of HLK testing per release.
- Win11 23H2+ stricter signing: unsigned drivers need test-mode
  (`bcdedit /set testsigning on` + reboot, breaks Secure Boot +
  BitLocker for the user).
- July 2024 CrowdStrike incident accelerated the kernel-surface
  narrowing — Microsoft is actively reducing what kernel drivers
  can do.
- Microsoft's own `BypassIO` is read-only / NTFS-NVMe-only, callable
  only from filter drivers (not user mode).

For Copy That, **stay user-mode**. The wins are still substantial.

## Surprises from the research

1. **`CopyFileW` itself silently does CoW on Win11 24H2 Dev Drive.**
   If our bench test rig is on Dev Drive, plain `CopyFileExW` may
   already be metadata-only — meaning our 1657 MiB/s number is
   misleadingly LOW (we're benching something the OS short-circuits
   on bandwidth that doesn't actually flow). Worth checking the
   bench's volume type at runtime.

2. **IoRing is only ~2-3% over IOCP** for current Win11 builds. The
   pitch sounds like "Linux io_uring for Windows" but the impl is
   much more conservative — no SQPOLL kernel-thread polling, which
   is where Linux's io_uring win comes from.

3. **Microsoft's own File Explorer is single-threaded** for the
   actual copy. The Task Manager core spread you see is thread
   migration, not parallelism. We were never trying to beat
   Explorer; we're trying to beat Robocopy.

4. **Robocopy single-file `/MT` "speedup" reports in the wild are
   measurement noise** — the docs and architecture are clear that
   `/MT` is across-file only, but blog posts misleadingly imply
   otherwise.

## Sources (consolidated)

### MSDN / learn.microsoft.com
- [CopyFileExW](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-copyfileexw)
- [CopyFile2](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-copyfile2)
- [SetFileValidData](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-setfilevaliddata)
- [FSCTL_DUPLICATE_EXTENTS_TO_FILE](https://learn.microsoft.com/en-us/windows/win32/api/winioctl/ni-winioctl-fsctl_duplicate_extents_to_file)
- [Block Cloning](https://learn.microsoft.com/en-us/windows/win32/fileio/block-cloning)
- [ReFS Block Cloning](https://learn.microsoft.com/en-us/windows-server/storage/refs/block-cloning)
- [BypassIO for filter drivers](https://learn.microsoft.com/en-us/windows-hardware/drivers/ifs/bypassio)
- [BypassIO storage operations](https://learn.microsoft.com/en-us/windows-hardware/drivers/storage/bypassio)
- [ReadFileScatter](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-readfilescatter)
- [WriteFileGather](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-writefilegather)
- [Scatter/gather scheme](https://learn.microsoft.com/en-us/windows/win32/fileio/reading-from-or-writing-to-files-using-a-scatter-gather-scheme)
- [BuildIoRingReadFile](https://learn.microsoft.com/en-us/windows/win32/api/ioringapi/nf-ioringapi-buildioringreadfile)
- [Driver Code Signing Requirements](https://learn.microsoft.com/en-us/windows-hardware/drivers/dashboard/code-signing-reqs)
- [Driver Signing Offerings](https://learn.microsoft.com/en-us/windows-hardware/drivers/dashboard/driver-signing-offerings)
- [Robocopy](https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/robocopy)
- [Set up a Dev Drive](https://learn.microsoft.com/en-us/windows/dev-drive/)
- [PrefetchVirtualMemory](https://learn.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-prefetchvirtualmemory)

### Microsoft DevBlogs
- [Copy-on-Write in Win32 API Early Access](https://devblogs.microsoft.com/engineering-at-microsoft/copy-on-write-in-win32-api-early-access/)
- [Dev Drive and Copy-on-Write for Developer Performance](https://devblogs.microsoft.com/engineering-at-microsoft/dev-drive-and-copy-on-write-for-developer-performance/)
- [Why memory-mapped writes cause unwanted reads — Old New Thing](https://devblogs.microsoft.com/oldnewthing/20241107-00/?p=110486)

### Independent benchmarks + analysis
- [I/O Rings: When One I/O Is Not Enough — Winsider](https://windows-internals.com/i-o-rings-when-one-i-o-operation-is-not-enough/)
- [One Year to I/O Ring: What Changed — Winsider](https://windows-internals.com/one-year-to-i-o-ring-what-changed/)
- [IoRing vs io_uring — Winsider](https://windows-internals.com/ioring-vs-io_uring-a-comparison-of-windows-and-linux-implementations/)
- [yardenshafir/IoRing_Demos](https://github.com/yardenshafir/IoRing_Demos)
- [Samsung 980 PRO QD-saturation — AnandTech](https://www.anandtech.com/show/16087/the-samsung-980-pro-pcie-4-ssd-review/6)
- [Samsung 990 PRO QD32 = 7,485 MB/s — KitGuru](https://www.kitguru.net/components/ssd-drives/simon-crisp/samsung-990-pro-2tb-review/all/1/)
- [DiskSpd issue #118 — queue-depth saturation](https://github.com/microsoft/diskspd/issues/118)
- [Apriorit IOCP programming guide](https://www.apriorit.com/dev-blog/412-win-api-programming-iocp)
- [Robocopy and multithreading — Andy's Tech Blog](https://andys-tech.blog/2018/04/robocopy-and-multithreading-how-fast-is-it/)
- [Windows Memory Mapped File IO — Jeremy Ong](https://www.jeremyong.com/winapi/io/2024/11/03/windows-memory-mapped-file-io/)
- [The NT Insider on scatter/gather — OSR](https://www.osronline.com/article.cfm%5Eid=165.htm)
- [FastCopy implementation details — fastcopy.jp](https://fastcopy.jp/help/fastcopy_eng.htm)
- [Tom's Hardware: 24H2 block cloning 94% time reduction](https://www.tomshardware.com/software/windows/one-of-the-best-features-coming-with-the-windows-2024-update-is-block-cloning-a-former-windows-server-refs-exclusive)
- [SPDK NVMe driver](https://spdk.io/doc/nvme.html)
- [SSL.com Kernel-Mode Code Signing FAQ](https://www.ssl.com/faqs/faq-kernel-mode-code-signing-certificates/)
