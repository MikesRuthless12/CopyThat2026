# Security Backlog

Phase 17 of the build guide is "Security review & hardening". The full
prompt has eight workstreams; **Phase 17a — path-safety bar** ships in
the same commit as this file. The remaining workstreams stay open as
deferred sub-phases here so we can land them incrementally without
losing track. Each item is sized small enough to ship as its own PR.

## Shipped

- [x] **17a — Path-traversal + NUL-byte rejection at every trust
      boundary.** `copythat_core::safety::validate_path_no_traversal`
      runs lexically (no FS calls, no TOCTOU window) and is invoked
      from `copy_file` / `copy_tree` / `move_file` / `move_tree`, plus
      the Tauri commands (`start_copy` / `start_move`) and the CLI
      `--enqueue` dispatcher. Typed `CopyErrorKind::PathEscape` with
      `err-path-escape` Fluent key in all 18 locales. Smoke test
      `tests/smoke/phase_17_security.rs` (6 cases).

- [x] **17b — Dependency + supply-chain audit.** `cargo-audit` +
      `cargo-vet` CI jobs (`.github/workflows/ci.yml`); audit
      `--ignore` list mirrors `deny.toml`. `supply-chain/config.toml`
      imports the Mozilla / Google / Embark / Bytecode Alliance /
      Zcash audit feeds. Smoke test
      `tests/smoke/phase_17b_supply_chain.rs` (5 cases).

- [x] **17c — Symlink-race / TOCTOU hardening.** Engine source-side
      open path sets `O_NOFOLLOW` (Unix) / `FILE_FLAG_OPEN_REPARSE_POINT`
      (Windows) via `safety::no_follow_open_flags`. New
      `safety::is_no_follow_rejection` classifier so callers can
      distinguish a hardening rejection from generic I/O.
      `safety::is_within_root` retained for jail-style consumers.
      Smoke test `tests/smoke/phase_17c_symlink_race.rs` (4 cases on
      every host + 1 Unix-gated race regression).

- [x] **17e — IPC argument audit + canonicalisation.** New
      `apps/copythat-ui/src-tauri/src/ipc_safety.rs` module — typed
      `IpcError` enum + `validate_ipc_path` / `validate_ipc_paths`
      / `validate_ipc_path_ref` helpers. Every path-typed
      `#[tauri::command]` in `commands.rs` (start_copy / start_move
      / file_icon / reveal_in_folder / destination_free_bytes /
      path_total_bytes / path_metadata / path_sizes_individual /
      enumerate_tree_files / list_directory / drag_out_stage /
      thumbnail_for / error_log_export / history_export_csv /
      export_profile / import_profile) calls the gate. New Fluent
      key `err-path-invalid-encoding` across all 18 locales (706
      keys total). Smoke test
      `tests/smoke/phase_17e_ipc_audit.rs` (8 cases) walks
      commands.rs for drift.

- [x] **17f — Logging & content-scrubbing audit.**
      `copythat-audit::layer::AuditLayer::MessageVisitor` drops
      fields named `body` / `bytes` / `chunk` / `password` /
      `passphrase` / `secret` / `token` / `api_key` /
      `api-key` before they reach the sink (no redacted marker
      either — even the field name is information leakage).
      `copythat-hash::sidecar::validate_sidecar_relpath` rejects
      absolute / `..`-laden entries before writing the sidecar
      file. `eprintln!` calls in the IPC layer migrated to
      `tracing::debug!(target: "copythat::ipc", …)` so production
      builds don't surface user paths on stderr. Smoke test
      `tests/smoke/phase_17f_logging_scrub.rs` (7 cases).

- [x] **17g — Binary hardening flags.** Workspace
      `[profile.release]` upgraded to `lto = "fat"`; kept
      `panic = "abort"` + `codegen-units = 1` + `strip = "symbols"`.
      New `crates/copythat-cli/build.rs` + existing
      `apps/copythat-ui/src-tauri/build.rs` emit
      `-Wl,-z,relro -Wl,-z,now -Wl,-z,noexecstack` on Linux
      targets. Windows `/guard:cf` stays automatic on MSVC;
      macOS arm64 PAC + BTI stay automatic on the Apple linker.
      `docs/SECURITY.md` updated to mark Phase 17g as shipped.
      Smoke test `tests/smoke/phase_17g_hardening.rs` (5 cases).

## Deferred (open)

### 17i — Replace VSS PowerShell shellouts with `IVssBackupComponents` COM

- The current Windows VSS backend (`crates/copythat-snapshot/src/backends/vss.rs`)
  shells to PowerShell + WMI for `Win32_ShadowCopy::Create` and
  `Delete`. Hardening landed in followup-4 / -6 / -7 closes the
  immediate attack surface:
  - Absolute `%SystemRoot%\System32\WindowsPowerShell\v1.0\powershell.exe`
    path (no PATH-hijack at elevated integrity).
  - Both `helper_vss::create_shadow`/`release_shadow` AND the
    in-process `vss::create_in_process`/`release_in_process` paths
    validate `volume == [A-Za-z]:\` and `shadow_id == {GUID}`
    before interpolation.
  - Bad-request rate limiter on the helper.
  - Pipe DACL via `win_pipe_security` + 256-bit random name suffix.
  - Post-handshake capability grant (17j).
  
  Remaining gap: PowerShell process startup is ~300–700 ms per
  shadow-create (perf, not correctness) and the format-string
  interpolation pattern is fragile against future contributor
  edits even with input validation.
- Port to direct `IVssBackupComponents` COM:
  - `CoInitializeEx` + `CoInitializeSecurity`
  - `CreateVssBackupComponents` → returns `IVssBackupComponents*`
  - `InitializeForBackup` + `SetContext(VSS_CTX_BACKUP)`
  - `StartSnapshotSet` → snapshot-set GUID
  - `AddToSnapshotSet(volume_path)` → shadow GUID
  - `DoSnapshotSet` (returns `IVssAsync*`; poll `QueryStatus`
    until `VSS_S_ASYNC_FINISHED`)
  - `GetSnapshotProperties(shadow_id)` → `VSS_SNAPSHOT_PROP` with
    `pwszSnapshotDeviceObject` = `\\?\GLOBALROOT\Device\HarddiskVolumeShadowCopyN`
  - `DeleteSnapshots` for cleanup

**Binding-source survey (followup-7).** The IVssBackupComponents
COM interface is declared in Microsoft's `vsbackup.h`, which is
**not bound** by `windows-sys 0.59` / `0.60.2` / `0.61.2` (the
generated metadata excludes vsbackup) NOR by the higher-level
`windows 0.61.3` / `0.62.2` crates' `Win32::Storage::Vss` modules
— only adjacent VSS structs (`VSS_SNAPSHOT_PROP`,
`VSS_SNAPSHOT_CONTEXT`) and provider interfaces appear. The
options for shipping this:

1. **Hand-write extern bindings** alongside the existing
   PowerShell path in `vss_com.rs`. Roughly 400 lines of vtable
   layouts + GUID constants + struct definitions; high error
   surface without a Windows VSS test environment.
2. **Add `winapi-0.3.9`** as a dep — the older crate ships
   `IVssBackupComponents` in `winapi::um::vsbackup`. winapi-0.3
   is in maintenance mode but proven; ~3 MB compile-time cost.
3. **Wait for `windows-sys` to expose `Win32_Storage_Vss::Backup`**
   — Microsoft's metadata project has been adding more
   interfaces; vsbackup may land in a future minor release.

**Followup-8 — scaffolded behind `vss-com` feature flag.** Took
option 2: `crates/copythat-snapshot/src/backends/vss_com.rs`
ships `create_shadow_via_com` + `release_shadow_via_com` with
the full IVssBackupComponents flow (CreateVssBackupComponents →
InitializeForBackup → SetBackupState → SetContext →
StartSnapshotSet → AddToSnapshotSet → PrepareForBackup [async
poll] → DoSnapshotSet [async poll] → GetSnapshotProperties →
DeleteSnapshots), RAII wrappers for `IVssBackupComponents` /
`IVssAsync` / `VSS_SNAPSHOT_PROP` so every interface pointer
releases on Drop, GUID round-trip helpers matching the WMI
`{XXXXXXXX-...}` shape, and a `#[tokio::test] #[ignore]` smoke
that runs the full create/release cycle when invoked with
`--ignored` from an admin shell on a Windows VSS host. The
companion fixtures `tests/lock_file.ps1` and
`tests/vss_com_smoke.ps1` drive the verification.

Production code still uses the PowerShell path — the feature
flag stays off by default until the Windows VSS test environment
verifies the COM bindings work end-to-end. Flip the call sites
in `backends/vss.rs::{create_in_process, release_in_process,
release_in_process_blocking}` to call `super::vss_com::*` once
the round-trip test passes.

### ~~17j — Helper-argv signing / capability binding~~ (shipped, redesigned)

Originally scoped as HMAC-bound argv signing. Implementation in
followup-7 took a different (better) shape: **post-handshake
capability grant over the (DACL-restricted) pipe**.

Shipped:

- New `Request::GrantCapabilities { capabilities: Vec<Capability> }` /
  `Response::CapabilitiesGranted { granted: Vec<Capability> }`
  wire-protocol additions in `crates/copythat-helper/src/rpc.rs`.
- `bin/helper.rs`'s run-loop maintains `pipe_granted: Vec<Capability>`
  state (starts empty); the legacy `--capabilities=` argv flag is
  retained as the *upper bound* (you can never grant more than
  the spawn argv asked for) but no longer the source of truth for
  the active set.
- Capability checks gate against
  `effective = argv_requested ∩ pipe_granted`. Capability-bearing
  requests received before any `GrantCapabilities` arrives surface
  as `CapabilityDenied`.
- Smoke tests in `bin/helper.rs::tests` cover all three matrix
  cells: deny-before-grant, grant-then-serve, clamp-to-argv when
  the pipe asks for more than argv allowed.

Why the redesign vs. HMAC: the originally-proposed HMAC binding
needed a key both the unprivileged parent and the elevated child
could read, which on Windows means the key sits in either argv
(visible to any same-user process via `Get-Process | Select
CommandLine`) or environment (also same-user-readable). HMAC
without confidentiality of the key isn't a meaningful defence
against the realistic threat (same-user attacker reading argv).

The pipe-handshake design defends against the actual threat
window — argv injection between `Start-Process -Verb RunAs` and
helper-startup. The DACL-restricted pipe (`win_pipe_security`,
followup-6) plus 256-bit random pipe-name suffix (followup-4) is
what limits who can connect; the post-handshake grant means even
if argv is forged, the elevated helper does nothing
capability-bearing until it sees a grant message from the
legitimate caller over the secured pipe.

### 17i — Replace VSS PowerShell shellouts with `IVssBackupComponents` COM

### 17d — Privilege separation (`copythat-helper`)

- New `copythat-helper` binary that holds *only* the elevated paths:
  hardware-erase (NVMe Sanitize / OPAL / ATA Secure Erase — see Phase
  4 `Nist80088Purge` stub), shell-extension install/uninstall (writes
  to HKLM on Windows, `/Library/PreferencePanes` on macOS, system
  `.desktop` files on Linux), and the elevated-retry path the Phase 8
  `error-modal-retry-elevated` button still stubs out.
- The main UI never elevates; it spawns `copythat-helper` via the
  OS-native flow (UAC / `sudo` / `polkit`) and IPCs over a per-launch
  random-named pipe / Unix socket.
- Helper exits as soon as the elevated operation completes. Audit
  every IPC argument with the same lexical safety bar as the main
  app, plus a fresh capability check.
- Phase 19b's `copythat-helper-vss` is the narrow precedent — same
  named-pipe + RAII-release shape, scoped to a single workstream.

### 17h — `/security-review` skill pass

- Run `/security-review` (Claude Code skill) against the workspace
  and triage every High/Critical into a fix-forward PR; log Medium /
  Low findings as new entries in this file under the relevant
  sub-phase.
- User-triggered + billable run; the harness can't fire it
  automatically. With 17b / 17c / 17e / 17f / 17g shipped, the
  attack surface this pass reviews is now closer to its post-1.0
  shape — the next manual run is well-positioned.

## Cross-cutting decisions

- **Backwards compatibility.** `CopyErrorKind::PathEscape` is a new
  variant on a pre-1.0 enum; consumers that match exhaustively need
  to add an arm. The UI handles unknown kinds via `err-io-other` so
  there is no breakage path even if a downstream lags.
- **Performance.** The lexical guard walks `Path::components` once —
  on the order of ~50 ns per IPC call. Negligible vs the
  fast-path-or-tokio-loop that follows.
- **Test discipline.** Every shipped sub-phase carries its own smoke
  test under `tests/smoke/phase_17{a,b,c,…}_*.rs` and gets a row in
  `docs/ROADMAP.md`. Items that don't fit that mould (e.g. the
  `cargo audit` CI gate) are anchored by an assertion in the existing
  `phase_16_package` smoke test instead.
