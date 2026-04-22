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

## Deferred (open)

### 17b — Dependency + supply-chain audit

- Add `cargo audit` to CI (RustSec advisories — fail on High/Critical).
- Add `cargo vet` with the Mozilla + Google + Embark trust imports;
  require vet-clean for tagged releases.
- Pin direct dependencies in `Cargo.toml` via the workspace resolver
  and record the policy in `docs/SECURITY.md`.
- Decision: include `cargo deny advisories` (already runs on every
  push via the existing `cargo-deny` job) as the floor; `cargo audit`
  is added as a richer second pass that surfaces yanks + maintainer
  warnings the deny job can miss.

### 17c — Symlink-race / TOCTOU hardening

- Switch the engine's `tokio::fs::File::open` calls to
  `OpenOptions::custom_flags(libc::O_NOFOLLOW)` on Linux/macOS so a
  symlink swapped in mid-copy doesn't redirect to a victim file.
- On Windows use `FILE_FLAG_OPEN_REPARSE_POINT` plus
  `GetFinalPathNameByHandleW` to verify the resolved path is still
  inside the user-chosen staging root.
- Add `safety::is_within_root` (already drafted in Phase 17a) as the
  post-resolution boundary check; pair it with a regression smoke
  test that races a symlink swap against the open.

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

### 17e — IPC argument audit + canonicalisation

- Walk every Tauri `#[tauri::command]` and confirm the path-typed
  arguments pass through `validate_path_no_traversal` (Phase 17a) AND
  a per-command capability check.
- Reject non-UTF-8 paths on POSIX; on Windows accept WTF-16 via
  `OsStr` and convert lossily only for log lines.
- Add a typed `IpcError` enum so the frontend never receives an
  ad-hoc `String` message that conceals the underlying classification.

### 17f — Logging & content scrubbing audit

- `tracing` filter rule that drops any field named `body` / `bytes` /
  `chunk` / `password` regardless of level.
- Hash sidecars (`*.sha256` / `*.b3` etc., shipped in Phase 3) must
  use job-root-relative paths only — confirm with a smoke test that
  asserts no absolute path leaks into the on-disk format.
- Move `eprintln!` debug calls in the IPC layer behind a
  `tracing::debug!` macro so production builds don't surface user
  paths on stderr.

### 17g — Binary hardening flags

- Cargo: keep `panic = "abort"` (already set in workspace
  `[profile.release]`); add `lto = "fat"` for release.
- Linux build script: pass `-C link-args=-Wl,-z,now -Wl,-z,relro`.
- Windows: rely on the MSVC toolchain's default `/guard:cf`; add a
  reminder in `docs/SIGNING_UPGRADE.md` that the upgrade to a paid
  cert also unlocks `/INTEGRITYCHECK`.
- macOS arm64: PAC + BTI are automatic with the current Apple linker
  on the GitHub-hosted runners; add a smoke test that asserts the
  produced binary has the expected `LC_VERSION_MIN_MACOSX` entries.

### 17h — `/security-review` skill pass

- Run `/security-review` (Claude Code skill) against the workspace
  and triage every High/Critical into a fix-forward PR; log Medium /
  Low findings as new entries in this file under the relevant
  sub-phase.
- The Phase 17 prompt's first workstream is precisely this; deferred
  to once 17b–17g land so the skill has a more representative
  attack surface to review.

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
