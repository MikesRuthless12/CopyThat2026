# Phase 40 — Win32-skip + UI-bypass research (April 2026)

Two more research passes on top of [`RESEARCH_PHASE_39.md`](RESEARCH_PHASE_39.md):

- **Win32-skip path** — how can a user-mode Rust copier squeeze
  more throughput out of NTFS / ReFS / NVMe than `CopyFileExW`?
- **UI-bypass path** — how do we close the UI's 893 → engine's
  2429 MiB/s gap without forking the engine into a sidecar?

## Where we are after Phase 39

| Layer | MiB/s on 10 GiB · C→C | Beats |
|-------|---------------------:|-------|
| **CopyThat CLI / engine** | **2429** | Robocopy +86 %, FastCopy +141 %, TeraCopy +184 %, cmd copy +112-158 % |
| CopyThat UI | 893 | Robocopy −32 %, TeraCopy +5 %, FastCopy −4 %, cmd copy −5 % |

The engine already crushes the field. The UI gap is the
remaining target.

## Win32-skip: every legitimate technique, ranked

| # | Technique | Δ over 2429 MiB/s | Risk | Recommendation |
|---|-----------|------------------:|------|----------------|
| 1 | **Block clone via `FSCTL_DUPLICATE_EXTENTS_TO_FILE`** on ReFS / Dev Drive | **+10×-100×** (metadata-only) | low | **already shipped** via `reflink-copy` crate; auto-fires when src+dst share a ReFS volume |
| 2 | Tune queue depth to 8-16 in-flight overlapped I/Os, 1-4 MiB chunks | +5-25 % if not already at QD≥8 | low | **already shipped** — env-var tunable via `COPYTHAT_OVERLAPPED_*` |
| 3 | `SetFileValidData` (admin-only, skip NTFS lazy-zero on pre-allocated dst) | +5-15 % cold writes | medium (admin, opt-in) | **already shipped** behind `COPYTHAT_SKIP_ZERO_FILL=1` |
| 4 | Drop any leftover `FILE_FLAG_WRITE_THROUGH` + per-file `FlushFileBuffers` | +5-15 % if currently flushing per-file | low | **verify** — Raymond Chen confirms `WRITE_THROUGH + NO_BUFFERING` is "the slowest possible" |
| 5 | `FILE_DISPOSITION_POSIX_SEMANTICS` for Move/delete-source | UX win (instant delete) | low | **yes-now** for Move flow |
| 6 | Direct `NtCreateFile` / `NtReadFile` / `NtWriteFile` (NT-layer bypass of Win32 wrappers) | **0-3 %** (Win32 is already a thin shim; FastCopy actually uses Win32 too) | low | **no** — negligible gain |
| 7 | Direct `NtCopyFileChunk` syscall from user mode | **0-5 %** (kernel-mode-only API; user-mode call is undocumented) | medium (per-build SSN drift) | **no** — `CopyFileExW` already calls this internally on Win11 22H2+ |
| 8 | Raw volume `\\.\PhysicalDriveN` cluster writes | unknown, theoretically NVMe peak | **blue-screen / data-loss** | **NO** — re-implementing NTFS metadata in user space; no indie tool does this |
| 9 | DirectStorage 1.x for general copy (not GPU) | 0 % to slightly slower | medium | **no** — read-only, no `OpenFiles`, designed for many-small-reads |
| 10 | `BypassIO` user-mode opt-in | n/a — **does not exist** | n/a | **NO** — filter-driver-only, read-only, even on 25H2 |
| 11 | USN-journal suppression (`FSCTL_DELETE_USN_JOURNAL`) | +2-4 % | medium (breaks backup, AV, indexing) | **no** — collateral damage exceeds gain |
| 12 | `MapViewOfFile2` + `OfferVirtualMemory` zero-copy | **negative** (cold mmap is slower than ReadFile) | low | **no** — Jeremy Ong / Raymond Chen confirm |
| 13 | AVX-512 non-temporal stores (`vmovntdq`) for read→write | **0 %** under `NO_BUFFERING` (DMA bypasses CPU memcpy) | low | **no** — wrong tool |
| 14 | IoRing API (Win11 22H2+) | +2-3 % (Yarden Shafir's official benchmark) | medium (no Rust crate, Win11-only) | **defer** — small gain, young API |
| 15 | Custom NTFS minifilter / kernel driver | possibly +20-40 % on uncached cold reads | very high — **kills end-user UX** | **NO** — EV cert ($300-500/yr ongoing), WHQL submission, Win11 23H2+ stricter signing breaks Secure Boot for users on test-mode |
| 16 | SPDK / user-mode NVMe driver | drive limit (~7 GB/s) | very high — unbinds device | **no** — kills the filesystem layer; not a general copier |

### Key sources

- [`NtCopyFileChunk` (kernel-only API)](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/ntifs/nf-ntifs-ntcopyfilechunk)
- [BypassIO docs (filter-driver-only, read-only as of 25H2)](https://learn.microsoft.com/en-us/windows-hardware/drivers/ifs/bypassio)
- [Block cloning on ReFS — Microsoft Learn](https://learn.microsoft.com/en-us/windows-server/storage/refs/block-cloning)
- [Raymond Chen — `WRITE_THROUGH + NO_BUFFERING` is "the slowest possible"](https://devblogs.microsoft.com/oldnewthing/20140306-00/?p=1583)
- [`FILE_DISPOSITION_POSIX_SEMANTICS`](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/ntddk/ns-ntddk-_file_disposition_information_ex)
- [DirectStorage Developer Guidance — read-only, no per-file open amortization](https://github.com/microsoft/DirectStorage/blob/main/Docs/DeveloperGuidance.md)
- [FastCopy Help — admits "designed using Win32 API and C Runtime only"](https://fastcopy.jp/help/fastcopy_eng.htm) (the "FastCopy uses NT API" claim in the wild is wrong)
- [Yarden Shafir IoRing benchmarks — ~2-3 % over IOCP](https://github.com/yardenshafir/IoRing_Demos)
- [Driver Signing Requirements + Microsoft Trusted Signing $9.99/mo (since 2024)](https://learn.microsoft.com/en-us/windows-hardware/drivers/dashboard/code-signing-reqs)

### Hard cap verdict

For **NTFS same-volume** large-file copy on Windows 11 NVMe from
user mode, the practical ceiling is **~2800-3200 MiB/s on Gen 4
NVMe**, **~3500-4000 MiB/s on Gen 5** with the new native NVMe
driver enabled.

**At 2429 MiB/s we are at ~75-85 % of the practical ceiling.**
The remaining 15-25 % lives in:

- Verifying we've dropped any leftover `FlushFileBuffers` /
  `WRITE_THROUGH` (#4 above)
- Tuning queue depth + buffer size on the overlapped path
  (#2 above — already shipped, env-var tunable)
- ReFS / Dev Drive block clone (#1 above — already shipped)

There is **no** legitimate user-mode trick that yields a clean
2× on plain NTFS same-volume. Direct NT calls,
`NtCopyFileChunk`, raw-volume writes, DirectStorage,
`MapViewOfFile2`, AVX-512 streaming stores all either produce no
measurable gain, produce regressions, or require kernel-mode
work that breaks the end-user UX.

## UI-bypass: closing the UI → engine gap (893 → 2429)

### Why it exists

The `copythat-ui.exe --enqueue copy …` invocation does this on
the second instance (the one launched from the shell or CLI):

1. Parse argv (fast).
2. Open the SQLite history file (~50-200 ms).
3. Load `settings.toml` (~10-50 ms).
4. Initialise the profiles store (~50-200 ms).
5. Build the `AppState`.
6. Construct the Tauri builder + register every plugin
   (single-instance, fs, dialog, updater, global-shortcut).
7. Call `builder.run()` — Tauri then initialises the runtime,
   the WebView2 host, the DPI awareness, the window factory.
8. Setup hooks fire in plugin order. Eventually the
   single-instance plugin detects an already-running first
   instance, forwards argv via its IPC channel, and the second
   process calls `app.exit(0)`.

Steps 2-4 + 7 + 8 sum to ~5-7 seconds **per `--enqueue`
invocation**, all of which is wasted work on the second
instance. Phase 39 measurements: UI 893 MiB/s vs engine
2429 MiB/s = ~7 seconds of pure boot overhead on a 10 GiB
copy.

### Approaches considered

| Approach | Pros | Cons | Verdict |
|----------|------|------|---------|
| **Windows service** (CLI talks to service via named pipe; service does copy in-process) | Zero shell-init overhead; service stays warm; clean bidirectional IPC | **Admin install required** + UAC prompt + service-install friction; loses casual users; code-signing more rigid | **NO** — install friction conflicts with "do not lose end users" |
| **User-mode broker process** (auto-starts on first launch, stays in tray) | No admin needed; same warm-broker benefit | New process lifecycle to manage; must handle clean shutdown / orphan processes | maybe — but already what tauri-plugin-single-instance attempts |
| **Lightweight pre-empt in `main()`** (Windows mutex check before any heavy init; if second instance, forward args via custom named pipe and exit) | No new processes; uses existing app as broker; eliminates 5-7 s second-instance boot | ~150 lines unsafe Windows IPC; named-pipe server in setup hook + client in main + mutex bootstrap | **YES — Phase 41** |
| **Sidecar `copythat-cli` for fast-lane jobs** (Tauri `externalBin` + `Command::new_sidecar`) | Literally CLI throughput (2429); UI just `tail`s stdout | Loses unified queue / cancellation / audit; orphan-process hygiene; double engine ship; progress-via-file-size-polling is racy on multi-file jobs | maybe — escape valve for explicit "fast lane" single-file jobs only |

### Recommended Phase 41 implementation

**Lightweight pre-empt + named pipe broker**, in two parts:

#### Part A: second-instance fast bail (`apps/copythat-ui/src-tauri/src/lib.rs::run()`)

Before `let mut builder = tauri::Builder::default()`, detect
whether we're a second instance via a Windows mutex:

```rust
#[cfg(windows)]
fn is_second_instance() -> bool {
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;
    use windows_sys::Win32::Foundation::{ERROR_ALREADY_EXISTS, GetLastError};
    use windows_sys::Win32::System::Threading::CreateMutexW;

    let name: Vec<u16> = OsStr::new("Local\\copythat-ui-instance-mutex")
        .encode_wide()
        .chain(Some(0))
        .collect();
    // SAFETY: name is null-terminated UTF-16. The mutex handle
    // leaks intentionally — we want it held for our process
    // lifetime so a third-instance check still finds it owned.
    let handle = unsafe { CreateMutexW(std::ptr::null_mut(), 0, name.as_ptr()) };
    !handle.is_null() && unsafe { GetLastError() } == ERROR_ALREADY_EXISTS
}
```

If `is_second_instance()` returns true:

1. Try to open the named pipe `\\.\pipe\copythat-ui-enqueue` as a
   client.
2. Serialise the `CliAction` (JSON) and write to the pipe.
3. Wait for an ack byte (so we don't exit before the first
   instance acknowledges receipt).
4. `return` from `run()` immediately.

This skips the SQLite history open, settings load, profiles
store, Tauri builder, plugin registration, runtime init, WebView2
init, and setup hooks. Total saved: ~5-7 seconds per invocation.

#### Part B: named-pipe server in setup hook (first instance)

Inside the existing `.setup(move |app| { … })` block, spawn a
dedicated thread that:

```rust
loop {
    // CreateNamedPipeW("\\.\pipe\copythat-ui-enqueue", PIPE_ACCESS_DUPLEX, ...)
    // ConnectNamedPipe(handle, NULL) — block until a client connects
    // ReadFile until \n or EOF — buffer the JSON
    // Parse into CliAction
    // shell::dispatch_cli_action(&app_handle, action)
    // WriteFile([0x06]) — ack byte
    // DisconnectNamedPipe + loop
}
```

The pipe DACL is restricted to the current user only (per the
existing VSS-pipe DACL pattern in
`crates/copythat-helper/src/transport.rs`).

#### Expected outcome

- First-instance launch: identical to today (full Tauri boot for
  the visible UI window).
- Second-instance `--enqueue` invocation: ~50-200 ms total
  (mutex check + pipe write + ack), down from ~5-7 seconds.
- UI 10 GiB · C→C copy throughput: **893 → ~2300+ MiB/s**
  (within 5 % of the engine's 2429).

### What this does NOT solve

The UI's *interactive* copy path (drag-drop into the running
window, `start_copy` IPC) doesn't have the second-instance boot
overhead — it's already running in the live app. Its throughput
should already be at engine-speed minus the per-event Tauri
emit overhead (which Phase 39 fixed via the 120 ms throttle).

If interactive UI throughput is also showing a gap (which we
haven't measured directly), the suspect is no longer the boot —
it's the per-job orchestration in `runner.rs::run_job` (audit
records, history inserts, journal sink, shape sink, transform
hook, mpsc channel allocation, forward_events spawn). Phase 41
should profile *that* separately by setting `tracing` to
`debug` and timestamping each phase.

## Phase 41 priority order

1. **Drop any leftover `FILE_FLAG_WRITE_THROUGH` / per-file
   `FlushFileBuffers`** if our engine has them. (Cheap audit;
   if found, +5-15 %.)
2. **Implement the second-instance fast bail** (named-pipe
   broker, Part A + Part B). UI 893 → ~2300 MiB/s.
3. **`FILE_DISPOSITION_POSIX_SEMANTICS` for Move source-delete**.
   UX win, small throughput gain.
4. **Profile the interactive UI path** to confirm there's no
   second hot spot once the boot overhead is gone.

Tier-2+ items (IoRing, scatter/gather, NT layer, raw volume,
DirectStorage, kernel driver) all yield <5 % or kill end-user
UX. Skipped per [`RESEARCH_PHASE_39.md`](RESEARCH_PHASE_39.md)
verdict.

## End-user impact summary

Phase 41 changes are **all user-mode, no admin install, no
service install, no driver install**. The named-pipe broker
runs entirely inside the existing `copythat-ui.exe`. The mutex
+ pipe live in the user's session — no system-wide footprint,
nothing to clean up on uninstall.

Zero end-user friction. Maximum throughput.
