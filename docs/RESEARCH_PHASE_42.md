# Phase 42 — File-copy state-of-the-art deep dive (2026-04-26)

A **swarm-research deep dive** spanning ~270 sources across 10 specialist
agents, plus a codebase audit cross-referencing every research finding
against the shipping engine. Replaces / supplements
`RESEARCH_PHASE_39.md` and `RESEARCH_PHASE_40.md`.

The full per-agent findings (with sources cited) live at
`target/research-phase-42-scratch.md`. This document is the synthesis +
gap list.

---

## TL;DR

Phase 13c-final + Phase 38–41 puts the engine **at parity or ahead of
TeraCopy / FastCopy / RoboCopy / cmd copy on every disk-bound scenario
we measured**. The 2026 research pass found **no architectural change**
that would meaningfully move that needle on consumer single-NVMe — the
Phase 38 "single-stream `CopyFileExW` is the optimum default" verdict is
research-confirmed for a **second** independent pass.

What the new research *does* surface is a **gap list of narrow
features** competitors ship that we don't. They are not throughput
deltas on our current bench matrix — they are correctness, fidelity,
edge-case, and topology-detection wins that matter on real-world user
hardware (RAID, SMB, OneDrive, Dev Drive, hardlink-heavy trees, etc.).

**Audit result:** of 21 items checked against the codebase,
**6 DONE, 6 PARTIAL, 9 MISSING**. Most MISSING items are scope-bounded
and shippable individually; none are blocking the "we beat them all"
claim on the disk-bound benches.

---

## What's already optimal — research-confirmed-CONFIRM (do not change)

The following 2026 swarm pass independently re-confirms each of these:

- **Allocation-free `CopyFileExW` callback writing an atomic** + Tokio
  polling task. Per-event cost: 1-3 ns vs ~200 ns for `mpsc::send`. At
  ~110 k callbacks/sec on Gen5 NVMe, the difference is ~22 ms wasted per
  10 GB copy avoided.
- **256 MiB `NO_BUFFERING_THRESHOLD` default** matches Microsoft's xcopy
  /J empirical guidance and our own Phase 13b regression test.
- **`reflink-copy` ladder + cross-volume short-circuit** via `volume_id`
  — the right shape on every platform we surveyed (Linux
  `copy_file_range` + FICLONE; macOS `clonefile` + `copyfile(3)`;
  Windows `FSCTL_DUPLICATE_EXTENTS_TO_FILE`).
- **4-stage dispatcher** (try_reflink → OS-native → AlwaysFast →
  async byte-pump) is correct.
- **Default `COPYTHAT_PARALLEL_CHUNKS=off`** for consumer single-NVMe.
  Phase 13c regression measurements (-25% C→C, -76% C→E) are the
  predictable outcome — Windows NT cache manager already pipelines at
  QD≈8 via `CcReadAhead`/`CcLazyWrite`.
- **USB → force N=1 / serial** path. Confirmed by FastCopy's same-disk
  serial mode + our -76% measurement.
- **Don't Unicode-normalize paths** — NTFS treats names as opaque
  WCHARs. Normalization breaks roundtripping.
- **`SetFileValidData` behind a feature flag, default OFF** (Phase 39
  design). The function leaks raw cluster contents and Microsoft now
  documents it as a security footgun.
- **Sharing-violation retry/backoff** mirrors Robocopy `/R:n /W:s` —
  `engine.rs:1739-1821` already implements 3-retry exponential backoff
  on `ERROR_SHARING_VIOLATION`.

## Confirmed dead-ends — do not pursue

Each of these was explicitly evaluated and ruled out by at least one
research agent, often two:

| Dead-end | Why |
|---|---|
| Memory-mapped I/O for sequential copy | Page-fault overhead; can't combine with `NO_BUFFERING`; modern `CopyFile2` uses overlapped IOCP, not MMF |
| `tokio::fs::copy` for the copy path | It's just `spawn_blocking` internally — strictly worse than calling `std::fs::copy` once |
| `compio` wrapping `CopyFileExW` | `compio` is a runtime, not a copy primitive — adds nothing on the syscall path |
| `CopyFileTransacted*` | TxF deprecated since Win10 |
| `TransmitFile` for LAN | Superseded by SMB compression + `CopyFile2` offload |
| `ReadFileScatter` / `WriteFileGather` | Only wins on many small page-aligned buffers; for sequential whole-file copy with one big preallocated buffer it's a wash |
| DirectStorage / GDeflate / GPU memcpy | Read-only, NVMe→GPU game-asset path. Not a copy primitive |
| WOF-compressed-file preservation across copy | No public API copies `WofCompressedData` ADS atomically |
| Preserving NTFS dedup across copy | Microsoft KB explicitly warns Robocopy can corrupt the Chunk Store |
| `MEM_LARGE_PAGES` for copy buffers | No measurable benefit; requires `SeLockMemoryPrivilege` |
| Disabling SysMain to "speed up copies" | Modern memory manager handles cache pressure; disabling hurts app launch |
| `FILE_FLAG_RANDOM_ACCESS` for any copy | Disables prefetch with no upside |
| `SetSystemFileCacheSize` to clear cache between copies | Microsoft KB explicitly warns against |
| `IORING_OP_COPY_FILE` / IORING-based copy | Op doesn't exist; IORING is read-optimized; ~2-3% over IOCP for read-heavy workloads only |
| Smart App Control concerns | Doesn't affect file-copy syscalls |
| Cross-volume reflink | Physically impossible (extents live in one allocation pool) |
| Chunking single file across parallel readers on same physical device | Contends on per-device queue; Phase 13c -25% / -76% measured regressions confirm |
| Always-on verify-after-copy | Doubles destination IO; unacceptable as default |
| SHA-1 / MD5 / SHA-512 as default verify | Cripple Gen5 NVMe throughput at 2-3 GB/s |
| Windows 11 24H2 same-volume ReFS explicit clone code | OS already does it inside `CopyFileExW` natively (KB5034848+) |

---

## Gap list — 21 items audited, 9 MISSING / 6 PARTIAL / 6 DONE

Sorted by **research-impact × audit-status**. The expected wins on our
current bench matrix are mostly small (we already beat competitors on
disk-bound scenarios); the value here is **correctness, fidelity, and
real-world topology coverage** — not synthetic-bench numbers.

### MISSING (9)

| # | Gap | Source | Notes |
|---|---|---|---|
| 3 | **Paranoid verify mode** — re-open dest with `FILE_FLAG_NO_BUFFERING` after `FlushFileBuffers` and re-hash | Hash agent | Only mode that catches write-cache lying / silent dst corruption / FS bugs |
| 4 | **Pre-copy attribute probe** (`GetFileAttributesExW`) — fork on `RECALL_ON_DATA_ACCESS`, `REPARSE_POINT`, `SPARSE_FILE`, `COMPRESSED`, `ENCRYPTED` | Hardware/edge agent | Gates OneDrive cloud, sparse, encrypted handling. Currently pathways exist but aren't auto-engaged from attribute detection |
| 6 | **Hardlink set detection** during scan + `CreateHardLinkW` on dest (group by `(VolumeSerialNumber, FileIndex)`) | Hardware/edge + competitor agents | Multi-link sets currently become independent files. Robocopy `/COPYALL` preserves, FastCopy `/linkdest` preserves |
| 9 | **`COPY_FILE_REQUEST_COMPRESSED_TRAFFIC` flag** for SMB UNC dests | Win32 + NTFS agents | Win10 1903+; free win on slow links; incompatible with SMB Direct/RDMA (skip for those) |
| 10 | **`FILE_SKIP_COMPLETION_PORT_ON_SUCCESS`** on Phase-41 IOCP-bound handles in `windows_overlapped.rs` | Win32 + Async agents | Skips a syscall when I/O completes inline. Measurable on cached reads |
| 12 | **OpenZFS 2.2.0-2.2.6 detection** → emit warning / NoReflink hint | Reflink agent | `zfs_bclone_enabled=1` on these versions has data-corruption bug (#15526). Recommended ON only for 2.3+ |
| 13 | **`STORAGE_ACCESS_ALIGNMENT_DESCRIPTOR.BytesPerPhysicalSector`** per-volume probe | Cache + Buffer agents | `IOCTL_STORAGE_QUERY_PROPERTY`. Current code doesn't hardcode 4096 (overlapped uses 1 MiB which is naturally aligned), but doesn't probe either — fragile on 8 KiB-sector enterprise SSDs |
| 15 | **`compio` for Phase-41 manual overlapped path** | Async agent | Replace hand-rolled IOCP completion logic with maintained runtime. No perf change vs `CopyFileExW`; pure code-simplification win |
| 19 | **Defender exclusion guidance** in `docs/PERFORMANCE_TUNING.md` | Hardware/edge agent | Defender double-scans copies (read + write) — often dominant slowdown. Document the path-exclusion workflow as opt-in |

### PARTIAL (6)

| # | Gap | Status | Notes |
|---|---|---|---|
| 1 | **Migrate `CopyFileExW` → `CopyFile2`** | Engine still on `CopyFileExW` with one flag (`COPY_FILE_NO_BUFFERING`) — `windows.rs:292`. `CopyFile2` adds NO_OFFLOAD / SKIP_ALTERNATE_STREAMS / ENABLE_SPARSE_COPY / DISABLE_PRE_ALLOCATION / RESUME_FROM_PAUSE. Win11 25H2 routes everything through `CopyFile2` internally already, but explicit migration unlocks the new flags |
| 5 | **Sparse preservation in default path** | `sparse.rs` exists and is correct, but the default `CopyFileExW` path bypasses it. Sparse is currently a separate early-branch in `engine.rs:121-179` — needs to be auto-engaged from the attribute probe (item #4) |
| 8 | **Adaptive `NO_BUFFERING_THRESHOLD`** | `windows.rs:58` is static 256 MiB with `COPYTHAT_NO_BUFFERING_THRESHOLD_MB` env override. Adaptive formula `max(256 MiB, min(2 GiB, free_phys_ram / 4))` would cut over earlier on 8 GiB hosts and prevent SuperFetch standby-list pollution |
| 16 | **Auto-engage parallel-N** via `IOCTL_STORAGE_QUERY_PROPERTY` BusType + UNC SMB detection | Cross-volume detection works (`is_cross_volume()`); BusType / UNC / RAID / Storage Spaces detection absent. Parallel still env-var opt-in. Heuristic: SMB / RAID / cloud-block / Storage Spaces with ≥2 columns + file ≥256 MB → suggest N=4-8 |
| 17 | **HDD destinations → 4 MiB buffer**, USB → QD ≤ 4 | `recommend_concurrency()` probes `is_ssd()` only. Detected-class buffer/QD sizing absent. Per Buffer-sizing agent's table: NVMe Gen3 256 KiB / Gen4 512 KiB / Gen5 1 MiB / SATA SSD 256 KiB / HDD 1-4 MiB / USB 512 KiB QD≤4 / exFAT ≥ cluster |
| 21 | **Block cloning auto-engagement** for ReFS / Dev Drive | `reflink_path.rs` calls `FSCTL_DUPLICATE_EXTENTS_TO_FILE` via `reflink-copy` crate; tried first for all same-volume copies. Win11 24H2+ already does this inside `CopyFileExW` natively — partial duplication is harmless. Could feature-gate to skip the extra `CreateFile` probe on detected 24H2+Dev Drive |

### DONE (6)

| # | Item | Evidence |
|---|---|---|
| 2 | xxHash3 + BLAKE3 in verify menu | `copythat-hash/src/algorithm.rs:20-31` — XxHash3_64, XxHash3_128, Blake3 all present; `VerifyChoice` enum exposes them |
| 7 | ADS on directories via `FindFirstStreamW` | `copythat-platform/src/meta.rs:1-30` documents `FindFirstStreamW`/`FindNextStreamW`; `copythat-core/src/meta.rs` has `NtfsStream` for per-file/dir ADS capture |
| 11 | `EXDEV` cross-volume reflink fall-through | `reflink_path.rs:61-86` `is_unsupported()` catches non-propagatable errors; EXDEV correctly falls through to next strategy. Could be made explicit (low effort) but functionally DONE |
| 14 | Tauri 2.0 with event progress channel | `apps/copythat-ui/src-tauri/Cargo.toml` `tauri = "2"`; `commands.rs:15-16` uses `Emitter` trait. Migrating to `Channel<T>` would be a 5-10× per-event speedup for the hot path — minor follow-up |
| 18 | Long path `\\?\` support | `windows.rs:340-343` `wide()` encodes via `OsStr::encode_wide()`. Rust `PathBuf` → `CreateFileW` correctly handles long paths. Could add explicit `\\?\` prefixing for paths ≥260 wchars to be paranoid, but default works |
| 20 | Sharing-violation retry/backoff | `engine.rs:1739-1821` `open_src_with_retry()` — `ERROR_SHARING_VIOLATION = 32`, exponential backoff 50/100/200 ms over 3 attempts. Mirrors Robocopy `/R:3 /W:1` (slightly faster cadence) |

---

## Recommended action order (if implementing the gap list)

Grouped by ROI for typical user workloads. **None of these are
prerequisites for the head-to-head benchmarks** — they are real-world
fidelity wins, not throughput moves.

### Tier 1 — small, high-fidelity wins (recommended)

1. **#9 `COPY_FILE_REQUEST_COMPRESSED_TRAFFIC`** for SMB UNC dests —
   one-line flag bump on detected UNC paths, free win on slow links.
2. **#10 `FILE_SKIP_COMPLETION_PORT_ON_SUCCESS`** in Phase-41 overlapped
   path — one `SetFileCompletionNotificationModes` call per handle.
3. **#11 explicit `EXDEV` mapping** in `is_unsupported` — robustness
   only, no behaviour change.
4. **#19 Defender exclusion guidance** — pure docs, no code.
5. **#20 → make sharing-violation retry knobs configurable** (already
   DONE behaviorally; expose via settings).

### Tier 2 — correctness/fidelity gaps that competitors ship

6. **#4 Pre-copy attribute probe** (`GetFileAttributesExW`) — gates
   #5/sparse, #6/hardlink, OneDrive cloud handling, EFS roundtrip.
   Should land first; everything else hangs off it.
7. **#5 Sparse preservation auto-engagement** — wire `sparse.rs` into
   the post-probe routing so `SPARSE_FILE` source files use it
   automatically.
8. **#6 Hardlink set detection + `CreateHardLinkW`** — Robocopy
   `/COPYALL` and FastCopy `/linkdest` both preserve. Currently a real
   user-visible gap on dev / build trees.
9. **#3 Paranoid-verify mode** — opt-in policy; only mode that catches
   write-cache lying.

### Tier 3 — topology / hardware tuning

10. **#13 `IOCTL_STORAGE_QUERY_PROPERTY`** per-volume probe → cache by
    GUID. Drives #17 (HDD/USB buffer/QD) and #16 (auto-parallel
    detection).
11. **#17 Media-class-aware buffer/QD sizing** — table from Buffer
    agent.
12. **#16 Auto-engage parallel-N on SMB/RAID/cloud-block** —
    suggest-not-force; env-var override stays.
13. **#8 Adaptive `NO_BUFFERING_THRESHOLD`** — `max(256 MiB, min(2 GiB,
    free_phys / 4))`. Phase 13d open work item gets resolved.
14. **#21 Skip extra reflink probe on Win11 24H2 + Dev Drive** —
    micro-optimization.

### Tier 4 — code-quality / future-proofing

15. **#1 Migrate to `CopyFile2`** — bigger refactor, low immediate ROI
    (Win11 routes `CopyFileExW` through it anyway). Worth doing for
    flag-set unlock + future-proofing.
16. **#15 `compio` for Phase-41 overlapped path** — code simplification,
    no perf change.
17. **#12 OpenZFS 2.2.x version warning** — one runtime check + doc
    note.

---

## Sources

The 10 research agents cited **~270 distinct sources** across the
research pass; the full per-agent source lists with one-line takeaways
live at `target/research-phase-42-scratch.md`. Highlights:

- **Microsoft Learn** — CopyFileExW, CopyFile2, COPYFILE2_EXTENDED_PARAMETERS, CREATEFILE2_EXTENDED_PARAMETERS, File Buffering, File Caching, Cache Manager Routines, Block Cloning (Win32 + ReFS), FSCTL_DUPLICATE_EXTENTS_TO_FILE(_EX), FSCTL_QUERY_ALLOCATED_RANGES, FSCTL_SET_SPARSE, IOCTL_STORAGE_QUERY_PROPERTY + STORAGE_*_DESCRIPTOR variants, ODX (`FSCTL_OFFLOAD_READ`/`WRITE`) + Compatibility Cookbook, robocopy + xcopy reference, SMB Multichannel + Compression + Direct, Cloud File API, Reparse Points + Tags, Symbolic Link Effects on FS Functions, Hard Links and Junctions, Maximum File Path Limitation, IORING_OP_CODE + BuildIoRing*, Dev Drive setup, ReFS overview, Data Deduplication interop, Storage Spaces / S2D plan volumes
- **Devblogs / Old New Thing (Raymond Chen)** — Sequential vs Random Access flags, NO_BUFFERING+WRITE_THROUGH interaction, FlushFileBuffers performance, CopyFile zero-fill, Path Normalization (Jeremy Kuhne)
- **Engineering at Microsoft** — Copy-on-Write performance and debugging on Dev Drive
- **NVM Express Inc.** — Base Spec 2.0 + 2.1
- **SNIA** — SSD Performance Test Specification 2.0.2, NVMe-oF
- **AnandTech** — SSD Queue Depth Mythology (Tallis 2018), Samsung 990 Pro / SN850X queue-depth scaling, Raptor Lake hash throughput
- **ServeTheHome / Storage Review / TechPowerUp / Phoronix** — Crucial T705 Gen5, Solidigm P5520, Kioxia CD8, 4×NVMe RAID 0 scaling, GNU coreutils parallel-copy patches
- **Yarden Shafir / windows-internals.com** — IORING evolution, write/flush ops in 22H2, IoRing_Demos benchmarks
- **Niall Douglas — LLFIO** design notes on overlapped CopyFileEx
- **Apriorit** — IOCP programming
- **Microsoft Press / Richter** — Sync/async I/O canonical reference
- **Tokio source + Alice Ryhl + Aaron Turon (Ringbahn) + matklad** — Async runtime cost model
- **compio-rs / mio / monoio / glommio** — Async-FS landscape
- **Tauri 2.0** — `Channel<T>` ipc docs
- **Cyan4973 (xxHash) + BLAKE3 paper (Aumasson 2020) + Intel SHA Extensions / PCLMULQDQ papers + RustCrypto** — Hash/verify throughput
- **Code Sector (TeraCopy v3/v4 RC2 + blog), Shirouzu Hiroaki (FastCopy 5.11.2 help + GitHub source), Microsoft Robocopy team, Ghisler (Total Commander), GPSoftware (Directory Opus FAQ), alphaonex86 (Ultracopier), kevinwu1024 (ExtremeCopy)** — Competitor internals
- **Forensics: kraftkennedy / stark4n6** — TeraCopy SQLite job model
- **Backblaze Drive Stats 2024** — silent corruption discussion + sequential benchmarks
- **Russinovich / Solomon / Ionescu — Windows Internals 7th ed.** — NTFS cache manager + Mm/Cc subsystems
- **kernel.org / man7.org / btrfs.readthedocs.io / OpenZFS GitHub (issues #15526 #15728 #15345 + PR #15050) / RFC 7862 NFSv4.2 / Apple developer (`man copyfile`) / eclecticlight.co (APFS clones + sparse files) / wadetregaskis.com (CoW APFS)** — Cross-platform reflink landscape
- **Helge Klein** — EFS+CopyFileEx
- **Tom's Hardware / pureinfotech / deploymentresearch / windowsforum** — Win11 24H2 ReFS 94% gain, SMB compression
- **Rust crates** — `reflink-copy`, `copy_on_write`, `xxhash-rust`, `blake3`, `crc32fast`, `sha1`, `sha2`, `md-5`

---

## Windows 11+ baseline — Phase 42 onward

**As of Phase 42 (2026-04-26), CopyThat2026 targets Windows 11+ only
(minimum build 22000, Win11 21H2, Oct 2021).** Windows 10 is end-of-life
(October 2025) and is dropped from the support matrix. This unblocks
adoption of features that were previously deferred for Win10
compatibility.

Per-item OS posture under the new baseline:

| # | Item | Min OS | Gate strategy |
|---|---|---|---|
| 1 | `CopyFile2` migration | Win8+ | ✅ Universal on Win11+. No gating. |
| 3 | Paranoid verify (`NO_BUFFERING` re-read + `FlushFileBuffers`) | Win11+ | ✅ No gating. |
| 4 | `GetFileAttributesExW` attribute probe | Win11+ | ✅ No gating. |
| 5 | Sparse preservation auto-engage | Win11+ | ✅ FSCTLs universal. |
| 6 | Hardlink set detection + `CreateHardLinkW` | Win11+ | ✅ No gating. |
| 7 | Directory ADS via `FindFirstStreamW` (DONE) | Win11+ | ✅ |
| 9 | `COPY_FILE_REQUEST_COMPRESSED_TRAFFIC` | Win10 1903+ | ✅ **Always satisfied on Win11+.** No version gate needed; only conditional on detected SMB UNC dest. |
| 10 | `FILE_SKIP_COMPLETION_PORT_ON_SUCCESS` | Vista+ | ✅ No gating. |
| 11 | Explicit `EXDEV` mapping | All | ✅ No gating. |
| 12 | OpenZFS 2.2.x detection | All | ✅ Linux-only path. |
| 13 | `IOCTL_STORAGE_QUERY_PROPERTY` per-volume probe | Win11+ | ✅ No gating. |
| 14 | Tauri 2.0 `Channel<T>` for progress | Tauri 2 / Win10 1809+ | ✅ |
| 15 | `compio` for Phase-41 overlapped path | Win11+ | ✅ Pure-Rust runtime. |
| 16 | Auto-engage parallel-N detection | Win11+ | ✅ |
| 17 | Media-class buffer/QD sizing | Win11+ | ✅ |
| 19 | Defender exclusion guidance | Win11+ | ✅ Pure docs. |
| 20 | Sharing-violation retry knobs | All | ✅ |
| 21 | Skip extra reflink probe on Win11 24H2 + Dev Drive | **Win11 24H2+** | ⚠️ `RtlGetVersion ≥ 26100` runtime gate; pre-24H2 path runs unchanged. |
| **NEW** | `COPY_FILE_ENABLE_SPARSE_COPY` flag in `CopyFile2` | **Win11 22H2+** | ⚠️ `RtlGetVersion ≥ 22621` runtime gate. Now in-scope under Win11+ baseline; pairs with item #5. Pre-22H2 falls back to manual `FSCTL_QUERY_ALLOCATED_RANGES` + range copy. |

**The OS-version detection helper (task #8) is still required** — even
on a Win11+ baseline we have to runtime-distinguish 21H2 / 22H2 / 24H2
to gate `COPY_FILE_ENABLE_SPARSE_COPY` and the 24H2 reflink-skip
optimization. Just not for "is this Win10".

**Fallback rule of thumb**: any flag introduced after Win11 21H2
(build 22000) must be passed via a single
`if os_supports(flag) { add_flag(); }` helper that defaults to "off"
on detection failure. We never bubble an "unsupported flag" error to
the user.

**What this unlocks vs. the prior Win10+ posture:**

- **`COPY_FILE_ENABLE_SPARSE_COPY` (Win11 22H2+)** — added as a new
  scope item, paired with sparse auto-engage (item #5). Lets
  `CopyFile2` preserve sparseness natively on supported builds without
  manual range-copy.
- **IORING experimental backend** — research classified DEAD-END for
  the hot path (only ~2-3% over IOCP, no `IORING_OP_COPY_FILE`). Stays
  DEAD-END; mentioned for completeness.
- **Cleaner code paths** — no Win10-specific fallbacks needed in items
  #9 (SMB compressed traffic) and several others. Simplifies the
  matrix.

---

## Verdict

The engine is **architecturally complete and competitive** as of
Phase 41. The 21-item gap list is **scope-bounded** and shippable
incrementally — each item is independent, none depend on a
non-shipping OS feature, and none compromise the existing CONFIRM
guarantees. **No items in the gap list would be expected to flip a
result on the disk-bound bench matrix** (256 MiB / 10 GiB × C→C/D/E),
which is why the head-to-head benchmark can run against the current
HEAD as a valid Phase-42 baseline.

The outstanding strategic decision is whether to land Tier 1+2 items
(items 3, 4, 5, 6, 9, 10, 11, 19, 20-knobs) **before** the
head-to-head benchmark — which would tighten the "we beat them all"
claim to include hardlink/sparse/cloud/SMB-compression workloads, not
just the four scenarios already measured.

---

## Deferred items — design notes for Phase 43

### Item: HardlinkSet integration into `copy_tree`

The Phase 42 work added `copythat_platform::hardlink_set::HardlinkSet`
with:

- `HardlinkSet::identify(src) -> Option<LinkIdentity>` — probes the
  per-platform `(volume, file-id)` triple.
- `HardlinkSet::dispatch(src, dst) -> io::Result<bool>` — looks up
  the ledger and, on a hit, calls `CreateHardLinkW` / `link(2)` and
  returns `Ok(true)`. On a miss, `Ok(false)` (caller byte-copies).
- `HardlinkSet::record(ident, dst)` — caller invokes after a
  successful byte copy to remember the canonical destination for
  subsequent members of the set.

The unit-tested helper is shippable. **What's missing is the wiring
into `copy_tree_inner` at `crates/copythat-core/src/tree.rs:777`** —
the per-file dispatch site that today goes straight to
`attempt_copy_with_policy`.

#### Why this is not a one-line insertion

The blocker is a **dependency cycle**: `copythat-platform` already
depends on `copythat-core`, so `core::tree` cannot directly call
`platform::HardlinkSet` without inverting one of the two dep edges.
Two viable architectures:

##### Option A — Move `HardlinkSet` into `copythat-core`

Pros: single crate, no trait indirection, `tree.rs` calls
`HardlinkSet::dispatch` directly.

Cons: the per-platform `identity_impl` (`#[cfg(target_os = "windows")]`
NTFS file-index probe via `GetFileInformationByHandle` +
`#[cfg(unix)]` `st_dev/st_ino` via `MetadataExt`) lives in `platform`
today by design. Moving it to `core` adds the Win32 + libc edge
that `core` currently avoids.

##### Option B — Trait hook on `TreeOptions` (recommended)

Add a trait that the runner — which has both `core` AND `platform`
in scope — can implement against `platform::HardlinkSet`:

```rust
// in copythat-core/src/options.rs
pub trait TreeFileHook: Send + Sync {
    /// Called immediately before each per-file copy. Return:
    /// - `Ok(HookOutcome::Skip)` — engine treats the file as
    ///   already done (e.g. hardlink-set hit; `dst` is now linked
    ///   to a prior member). Engine emits a `FileCopied` event
    ///   with `bytes = 0`.
    /// - `Ok(HookOutcome::Continue)` — engine performs the byte
    ///   copy as usual.
    /// - `Err(io::Error)` — surfaced as a `CopyError::IoOther`
    ///   and routed through the `on_error` policy.
    fn before_file(&self, src: &Path, dst: &Path) -> io::Result<HookOutcome>;

    /// Called immediately after a successful byte copy. Lets the
    /// hook record state (e.g. `HardlinkSet::record`) for
    /// subsequent files. Errors here are logged but do NOT fail
    /// the file — the bytes already landed; bookkeeping failures
    /// just disable future hardlink hits.
    fn after_copy(&self, src: &Path, dst: &Path);
}

pub enum HookOutcome {
    Skip,
    Continue,
}

// in TreeOptions
pub file_hook: Option<Arc<dyn TreeFileHook>>,
```

Wiring in `tree.rs:777` becomes:

```rust
EntryKind::File => {
    if let Some(hook) = &opts_file_hook {
        match hook.before_file(&entry.src, &dst_final) {
            Ok(HookOutcome::Skip) => Ok(FileOutcome::Done(0)),
            Ok(HookOutcome::Continue) => attempt_copy_with_policy(...).await,
            Err(e) => /* on_error_task routing */,
        }
    } else {
        attempt_copy_with_policy(...).await
    }
}
```

The runner (apps/copythat-ui or `copythat-cloud-runner`) constructs
an `Arc<HardlinkSetHook>` (a thin newtype around
`platform::HardlinkSet` implementing `TreeFileHook`) per tree-copy
job and threads it through `TreeOptions::file_hook`. This keeps
`core` free of the `platform` dep edge and lets future hooks (e.g.
"throttle on cloud-target backpressure", "consult dedup
catalogue") plug in without further engine changes.

#### Acceptance for Phase 43 landing

1. `copythat-core` defines `TreeFileHook` + `HookOutcome` and adds
   `file_hook: Option<Arc<dyn TreeFileHook>>` to `TreeOptions`.
2. `tree.rs::copy_tree_inner` calls `hook.before_file` /
   `hook.after_copy` at the dispatch site.
3. `copythat-platform` adds an `impl TreeFileHook for HardlinkSet`
   newtype (or directly on `HardlinkSet`).
4. Tauri runner wires `TreeOptions::file_hook = Some(hook)` when
   the user opts into hardlink preservation.
5. New regression test in `crates/copythat-core/tests/` exercises
   the hook (using a stub impl) so the trait + dispatch site stay
   covered without pulling in `platform`.
