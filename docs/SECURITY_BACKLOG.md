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
