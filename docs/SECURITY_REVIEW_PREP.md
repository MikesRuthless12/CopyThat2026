
# Phase 17h — `/security-review` prep

The `/security-review` Claude Code skill is **user-triggered and
billable** — the harness can't fire it from a non-interactive
session. This file is the briefing the next manual run should read
before starting, so the agents focus on the right surfaces.

## Scope

Run `/security-review` against the `feat/phase-38` branch (or
whatever branch is current at review time). The recent
Phase 38-followup-2 + Phase 38-followup-3 commits widened the
attack surface in seven specific places — each gets its own
section below.

## What changed in Phase 38-followup-2 (7 sub-phases shipped)

### 1. Phase 17b — cargo audit + cargo vet on every push

**Files touched.** `.github/workflows/ci.yml`,
`docs/SECURITY.md`, `supply-chain/config.toml`,
`supply-chain/audits.toml`, `supply-chain/imports.lock`.

**What review should focus on.**
- Does the `cargo audit --ignore` list mirror `deny.toml`'s
  `[advisories]` block exactly? Drift in either direction would
  either mask findings or block CI on something deny is silently
  allowing.
- Are the imported audit feeds (Mozilla / Google / Embark / Bytecode
  Alliance / Zcash) the right set for our threat model? An audit
  feed signed by a key we don't control is a supply-chain risk on
  its own.
- The Phase 17b smoke (`tests/smoke/phase_17b_supply_chain.rs`) is
  the tripwire — does it cover every drift path?

### 2. Phase 17c — symlink-race / TOCTOU hardening

**Files touched.** `crates/copythat-core/src/safety.rs`,
`crates/copythat-core/src/engine.rs`,
`crates/copythat-core/src/lib.rs`.

**What review should focus on.**
- The Unix `O_NOFOLLOW` value is `0x20000` on Linux glibc/musl
  and `0x100` on Apple / BSD libc. Confirm the per-OS branches
  in `safety::no_follow_open_flags()` are correct on every
  platform we ship (`cfg!(target_os = "ios")` is asserted to
  match macOS).
- The Windows `FILE_FLAG_OPEN_REPARSE_POINT` is `0x00200000`.
  Confirm the engine's `open_src_with_retry` actually applies the
  flag through `OpenOptionsExt::custom_flags`.
- `is_no_follow_rejection` recognises ELOOP (Linux 40, Apple 62)
  and Windows `ERROR_CANT_ACCESS_FILE` (1920) +
  `ERROR_INVALID_FUNCTION` (1). Are we sure no other error code
  would surface here on a swap?
- The Phase 17c smoke covers compile-time constants + a Unix-gated
  engine race regression. Anything missing?

### 3. Phase 17d — privilege-separated helper

**Files touched.** New crate `crates/copythat-helper/`,
`apps/copythat-ui/src-tauri/src/commands.rs::retry_elevated`.

**What review should focus on.**
- The protocol is newline-delimited JSON over stdin/stdout. The
  `MessageVisitor` in `copythat-audit::layer` is the closest
  precedent. Is the helper's parse-error recovery (return a
  `Response::Failed` rather than tearing down the connection)
  the right policy?
- The helper's `path_to_validate` covers `ElevatedRetry::src` +
  `HardwareErase::device`. Does it miss any path-typed field
  the capability gate also needs to check?
- `Capability::required_for` returns `None` for `Hello` and
  `Shutdown` — both are lifecycle. Confirm an attacker can't use
  Hello to leak side-channel information (we return the same
  response shape for matched + mismatched protocol versions, only
  the discriminant changes).
- The helper's `bin/helper.rs` reads from stdin / writes to
  stdout. The caller in `copythat-ui` runs the helper in-process
  via `handle_request` today; the actual UAC / sudo / polkit
  spawn (`Start-Process -Verb RunAs` for Windows) is a Phase 17d
  body fill. Confirm: does the in-process path leak any data the
  caller would normally not see, given that the helper now has
  no privilege separation?
- `generate_pipe_name` uses 256 bits from `getrandom`. If
  `getrandom` fails (rare; sandboxed contexts), we currently
  surface `io::Error::other("getrandom failed")`. Confirm: is
  that an acceptable failure mode, or should we fall back to a
  weaker PRNG?

### 4. Phase 17e — IPC argument audit + canonicalisation

**Files touched.** New `apps/copythat-ui/src-tauri/src/ipc_safety.rs`,
sweep through `apps/copythat-ui/src-tauri/src/commands.rs`.

**What review should focus on.**
- Every path-typed `#[tauri::command]` was audited; the smoke
  test (`phase_17e_ipc_audit`) walks the file and trips the build
  on drift. Are there path-typed args in *other* IPC modules
  (`audit_commands.rs`, `cloud_commands.rs`, `mobile_commands.rs`,
  `mount_commands.rs`, `scan_commands.rs`, `crypt_commands.rs`,
  `sync_commands.rs`) that the sweep didn't reach?
- The `IpcError::InvalidEncoding` variant rejects strings
  containing U+FFFD. Confirm: is the U+FFFD check at the right
  layer? A path containing a real `?` is fine; a WTF-16
  conversion that landed U+FFFD is not. Edge case: a user pastes
  text containing U+FFFD on purpose (testing).
- The new Fluent key `err-path-invalid-encoding` is in all 18
  locales (706 keys total). Confirm parity via `xtask i18n-lint`.

### 5. Phase 17f — logging + content scrubbing

**Files touched.** `crates/copythat-audit/src/layer.rs`,
`crates/copythat-hash/src/sidecar.rs`,
`apps/copythat-ui/src-tauri/src/commands.rs` (eprintln sweep).

**What review should focus on.**
- The `is_sensitive_field` allowlist is conservative —
  `body / bytes / chunk / password / passphrase / secret / token /
  api_key / api-key`. Is any sensitive-named field missing?
  Specific concerns: `private_key`, `auth`, `bearer`,
  `session_id`, `client_secret`. (We deliberately did not redact
  `path` because paths are user-visible in the queue + history;
  redacting them would break the audit.)
- `validate_sidecar_relpath` rejects `Component::ParentDir`,
  `Component::Prefix`, `Component::RootDir`. Does it miss any
  shape that would let an absolute path through (e.g. on Windows
  a path like `\foo` without a drive letter)?
- The `eprintln!` audit sweep in `commands.rs` migrated path-
  bearing diagnostics to `tracing::debug!`. Confirm the
  Phase 17f smoke's tripwire (`ipc_layer_does_not_eprintln_user_paths_in_release_builds`)
  covers every path-typed kw substitution.

### 6. Phase 17g — binary hardening flags

**Files touched.** `Cargo.toml` (workspace `[profile.release]`),
`crates/copythat-cli/build.rs`,
`apps/copythat-ui/src-tauri/build.rs`,
`docs/SECURITY.md`.

**What review should focus on.**
- `lto = "fat"` materially slows release builds (~3× link time
  on this host). Confirm the threat-model benefit (tighter
  cross-crate dead-code prune, fewer reachable gadgets)
  outweighs the build-time cost.
- The Linux build scripts emit `-z relro -z now -z noexecstack`.
  Confirm: are any of these no-ops on the toolchain we target
  (Rust nightly might already pass them)?
- Windows MSVC `/guard:cf` is automatic. Confirm: is there a
  way to verify the produced binary actually carries the CFG
  bit set (e.g. `dumpbin /headers`)?
- macOS arm64 PAC + BTI are automatic on Xcode 14+. Confirm:
  the GitHub-hosted runner is on Xcode 14+ (it is, per the
  `release.yml` `runs-on: macos-latest`).

### 7. Phase 14d — scheduled jobs

**Files touched.** New `crates/copythat-cli/src/schedule.rs`,
new `crates/copythat-cli/src/commands/schedule.rs`, `cli.rs`,
`runtime.rs`.

**What review should focus on.**
- The renderer escapes paths via `quote_for_schtasks` + `quote_for_shell`.
  Confirm: does the schtasks escape correctly handle a path
  containing both `"` and `&` (PowerShell special)? Does the
  shell escape correctly handle `'` inside a single-quoted string?
- The renderer requires absolute paths for src + dst. Confirm:
  is there a relative-path-with-symlink shape that would still
  render successfully?

### 8. Phase 14f — queue-while-locked watcher

**Files touched.** New `crates/copythat-cli/src/volume_watch.rs`.

**What review should focus on.**
- The watcher polls `Path::is_dir()` every `poll_interval`.
  Confirm: does it ever follow a symlink and report a dangling
  link as "reachable"?
- The watcher passes Phase 17a's lexical guard at construction.
  Is the cancellation flag (`AtomicBool`) the right shape, or
  should it be a `tokio::sync::Notify` for prompt teardown?

## What changed in Phase 38-followup-3 (this commit)

### 1. Phase 17d — helper crate scaffold (continues)

Same as #3 above. The crate landed in -2 follow-up; -3 wires the
`retry_elevated` IPC into it.

### 2. Phase 8 partials

**Files touched.** `apps/copythat-ui/src-tauri/src/commands.rs`
(`retry_elevated` wiring + `quick_hash_for_collision`).

**What review should focus on.**
- `quick_hash_for_collision` reads the file on the tokio blocking
  pool via `copythat_hash::hash_file_async`. Confirm: does the
  collision modal validate the path through the IPC gate before
  asking? (It does — `validate_ipc_path` runs first.)
- `retry_elevated` calls `state.errors.pending_paths(id)` which
  PEEKS at the pending error without consuming it. Is the peek
  the right choice (vs consume + re-register on capability
  failure)?

### 3. Phase 31b — real OS power probes

**Files touched.** New `crates/copythat-platform/src/presence.rs`,
`crates/copythat-power/src/source.rs`.

**What review should focus on.**
- The Windows `SHQueryUserNotificationState` call lives in
  `copythat-platform` (the only crate where unsafe is allowed).
  Confirm: the `// SAFETY:` comment captures the kernel's
  contract correctly — we pass a stack-local `i32` pointer and
  check the HRESULT before reading.
- The Linux DBus probe uses `zbus::blocking`. Confirm: does the
  blocking variant block forever on a stuck DBus session, or does
  zbus's connect carry an internal timeout?
- macOS keeps a stub. Confirm: are we OK shipping the macOS
  default behaviour (presentation-defaults-to-Pause but the probe
  always says "not presenting")?

### 4. Phase 13c — parallel-chunk gating

**Files touched.** `COMPETITOR-TEST.md` (research summary
appended), new `tests/smoke/phase_13c_parallel.rs`.

**What review should focus on.**
- The competitor research used eight authoritative sources
  (Microsoft Learn, Apple Developer, kernel.org, etc.). Confirm:
  no major file-copy primitive missed?
- The verdict is "ship as-is, single-stream wins". Confirm: the
  bench numbers in COMPETITOR-TEST.md match what the bench-vs
  harness actually produces.

### 5. Phase 7 partials

**Files touched.** New `tests/smoke/phase_07_partials.rs`.

**What review should focus on.**
- The shellext registry tripwire pins the CLSID strings to
  literal hex constants. Does the test pass on every host
  (the `linux_nautilus_extension_has_required_python_shape`
  assertion checks for the right gi typelib pin)?

## Defensive smokes summary

Every Phase 38-followup-2 + -3 sub-phase ships a smoke test that
fires on drift. The full table:

| Sub-phase | Smoke test | Cases |
| --- | --- | ---: |
| 17b | `phase_17b_supply_chain` | 5 |
| 17c | `phase_17c_symlink_race` | 4 + 1 Unix |
| 17d | `phase_17d_helper` | 11 |
| 17e | `phase_17e_ipc_audit` | 8 |
| 17f | `phase_17f_logging_scrub` | 7 |
| 17g | `phase_17g_hardening` | 5 |
| 14d | `phase_14d_schedule` | 10 |
| 14f | `phase_14f_queue_locked` | 5 |
| 13c | `phase_13c_parallel` | 7 |
| 31b | `phase_31b_real_probes` | 4 |
| 7 partials | `phase_07_partials` | 8 |
| 8 partials | `phase_08_partials` | 5 |

A `/security-review` agent reviewing these surfaces should run
each smoke first to confirm nothing has drifted between the
commit and the review.

## What `/security-review` should NOT spend time on

These were already reviewed extensively in earlier phases or are
out of scope for this branch:

- Phase 0–17a — covered in earlier reviews; no changes since.
- Phase 6 (platform fast paths) — Phase 6 already passed a
  `/security-review`; the only change since is the Phase 13c
  parallel-path gating, which is opt-in env-var gated and never
  default-enabled.
- Phase 19a–19b (scan DB, VSS) — no changes since their
  individual `/security-review` passes.
- Phase 25–29 (sync, watch, chunk store, drop stack, DnD) — no
  changes.
- Phase 32 (cloud) / Phase 33 (mount) / Phase 34 (audit) /
  Phase 35 (crypt) / Phase 36 (CLI) / Phase 37 (mobile) — covered
  by their own per-phase reviews; the only changes here are the
  Phase 17e IPC gate and the Phase 17f logging scrub, both of
  which strengthen rather than relax the trust boundary.

## After the review

1. Resolve every High/Critical finding before tagging the next
   release.
2. Log Medium / Low findings under the appropriate sub-phase
   bullet in `docs/SECURITY_BACKLOG.md`.
3. Re-run all twelve smoke tests in the table above; confirm
   green.
4. Update this prep doc's "What changed" sections to reflect the
   next batch.
