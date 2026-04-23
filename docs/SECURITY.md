# Security policy

Copy That v1.25.0 is in early development (Phase 0 — scaffold). The rules below
apply from day one even though the engine isn't implemented yet.

## Reporting a vulnerability

Please open a private security advisory on the GitHub repo
(`Security` tab → `Report a vulnerability`) rather than filing a public issue.
We will acknowledge within 5 business days and aim for a fix within 30 days
for High/Critical findings.

Do not include exploit details in public issues, pull requests, or commit
messages until a patched release is available.

## Supported versions

| Version | Supported           |
| ------- | ------------------- |
| `main`  | Yes (pre-release)   |

A formal support window will be defined in Phase 18 with the 1.0 release.

## Threat model (Phase 0)

The Phase 0 binary is a Tauri 2.x shell with no engine wiring and no IPC
commands beyond Tauri defaults. The realistic attack surface today is:

1. The webview rendering the static placeholder HTML.
2. The Tauri runtime itself (kept up to date — see dependency policy).
3. The build / CI pipeline (GitHub Actions on a private repo).

Concrete user-data risks (file paths, contents, hashes) appear from Phase 1
onward. Each later phase will extend this document.

## Dependency policy

- Every transitive dependency must satisfy `deny.toml`. The allowlist is
  permissive-only: MIT, Apache-2.0, BSD-2-Clause, BSD-3-Clause, ISC, CC0,
  Unlicense, Unicode-DFS-2016, Zlib, MPL-2.0. GPL / AGPL / SSPL / BUSL /
  CC-BY-NC and any other non-permissive license fails the CI build.
- `cargo deny check` runs on every push and pull request.
- `cargo audit` will be added in Phase 17 to catch RustSec advisories.
- `cargo vet` will be added in Phase 17 to require trust audits before
  upgrading dependencies.

## Code-execution boundaries (target, post-Phase 17)

- The main UI process never runs elevated. Operations that require
  elevation (secure-delete on protected files, shell-extension install)
  are dispatched to a separate `copythat-helper` binary that prompts the
  OS-native elevation flow (UAC / `sudo` / `polkit`). **Status:** deferred
  to Phase 17d — see `docs/SECURITY_BACKLOG.md`.
- All file paths received over IPC are canonicalised and validated for
  directory traversal and symlink races (`openat` / `O_NOFOLLOW` on Unix,
  long-path-aware Win32 calls on Windows). **Status (Phase 17a):** lexical
  `..`-traversal + NUL-byte rejection ships now in
  `copythat_core::safety::validate_path_no_traversal`, wired into
  `copy_file` / `copy_tree` / `move_file` / `move_tree` plus the IPC
  (`start_copy` / `start_move`) and CLI (`--enqueue`) entry points.
  Symlink-race / `O_NOFOLLOW` hardening is deferred to Phase 17c.
- File contents are never logged. Paths and hashes may be logged.

## Phase 17a — path-safety bar (shipped)

The IPC, CLI, and engine entry points all run a lexical guard
([`copythat_core::safety::validate_path_no_traversal`]) on every
caller-supplied path before any filesystem call. The guard rejects:

- Any `..` component (`Component::ParentDir`) — even one is enough to
  refuse, no path "simplification" rewrites.
- Any embedded NUL byte (POSIX `\0`, Windows wide-`u16` `0`).
- Empty strings.

A rejected path surfaces as a typed `CopyErrorKind::PathEscape` with
the new `err-path-escape` Fluent key, present in all 18 locales. No
filesystem operation runs, no partial destination is left behind, and
no history row is written.

Why lexical, not `canonicalize`-based: `std::fs::canonicalize` requires
the path to exist *right now* (which a destination doesn't) and
introduces a TOCTOU window between the check and the open. Walking
`Path::components` is filesystem-free and equally effective for
parent-traversal — the Phase 17c hardening pass uses `O_NOFOLLOW` on
Linux/macOS and `FILE_FLAG_OPEN_REPARSE_POINT` on Windows to close
the symlink-race gap on top of this lexical bar.

The Phase 17a smoke test
(`cargo test -p copythat-core --test phase_17_security`) crafts the
exact `foo/../../../etc/passwd` payload from the Phase 17 prompt and
asserts six independent rejection paths: helper, engine entry,
absolute-prefix variant, tree entry, source-side symmetry, and
locale-key resolution.

## Phase 19b — VSS privilege boundary (shipped)

Phase 19b adds read-from-snapshot fallback for locked files. The
threat-model impact is narrow but worth calling out explicitly:

- **Windows VSS requires Administrator.** The main Copy That process
  does not request elevation for itself. When the user opts into
  `LockedFilePolicy::Snapshot` and VSS is the chosen backend, the
  main process spawns a sibling binary `copythat-helper-vss.exe` via
  `Start-Process -Verb RunAs`, which triggers the OS-native UAC
  consent dialog. User consent is the only path to the elevated
  surface; the main Copy That binary never holds an Administrator
  token and never runs COM / WMI that requires one. Denial of the
  UAC prompt surfaces as a typed `SnapshotError::UacDenied` and the
  file falls through to the next `LockedFilePolicy`. This matches
  the Phase 17 privilege-separation rules.
- **JSON-RPC wire format is simple by design.** The helper accepts
  exactly four request kinds (`hello` / `create` / `release` /
  `shutdown`) carried as newline-delimited JSON over two named
  pipes created by the main process *before* Start-Process runs.
  The helper refuses mismatched protocol versions (`hello` carries
  a `version` field; a mismatch returns `ok: false` and the main
  process re-spawns). The helper tracks every shadow-copy ID it
  mints and best-effort-releases them on EOF, so a crashed main
  process cannot leak shadows for longer than the helper's own
  lifetime.
- **No new network surface.** Every snapshot backend is a local OS
  primitive (`vssadmin`/WMI / `zfs` / `btrfs` / `tmutil`). Copy That
  does not phone home to take or release a snapshot.
- **File contents flow through the snapshot path unchanged.** The
  engine reads `lease.translated` instead of the live source, hashes
  / verifies / writes / destroys using the same code paths it uses
  for any other source file. BLAKE3 verify (Phase 3) still runs
  post-copy and still catches a snapshot-side corruption.
- **Locale strings are translatable placeholders only.** The six
  `snapshot-*` Fluent keys cover UI prose; none carry user data or
  format structured logs. No user-data leak via i18n.

The privilege boundary is asserted by the engine's type system:
`CopyOptions::snapshot_hook` is the only path to a snapshot, the
trait requires `Send + Sync`, and the lease holds the RAII guard for
the full copy. A future phase that relaxes these invariants must
revisit this threat-model entry.

## Phase 20 — resume journal (shipped)

Phase 20 adds a redb-backed durable journal at
`<data-dir>/copythat-journal.redb`. Threat-model deltas:

- **No new network surface.** redb is a single-file embedded KV
  store; the journal never phones home. It sits next to the
  Phase 9 history DB and inherits the same per-OS data-dir
  permissions (`%LOCALAPPDATA%` on Windows, `~/.local/share` on
  Linux, `~/Library/Application Support` on macOS).
- **No new privilege escalation.** Both writes (during a copy) and
  reads (at app start) run in the unprivileged main process. The
  journal does not need elevated rights at any point — including
  the resume-time prefix re-hash, which only reads the destination
  the engine was already authorised to write.
- **Resume forgery is bounded by BLAKE3.** A malicious actor with
  write access to the destination could *replace* the partial dst
  bytes between the crash and the next launch. The engine's
  `decide_resume` re-hashes the dst's first `offset` bytes and
  compares against the `src_hash_at_offset` stored in the journal
  (which is fsync'd at the time of the original checkpoint). On
  mismatch the engine emits `CopyEvent::ResumeAborted
  { reason: "prefix-hash-mismatch" }` and falls back to a full
  restart from byte 0 — the corrupt prefix is never silently
  trusted. BLAKE3 is the same primitive used by Phase 3's verify
  pipeline.
- **No journal data leaks file contents.** The journal stores
  paths (already user-visible in the queue and history),
  monotonic sequence numbers, and 32-byte BLAKE3 digests of
  prefix bytes — never the bytes themselves. A stolen
  `copythat-journal.redb` reveals what was being copied where, on
  the same scale as the existing `history.db`. No additional
  classification is needed.
- **Journal corruption falls through cleanly.** Codec errors,
  redb commit failures, or I/O at app start surface as a typed
  `JournalError` and the runner skips checkpointing for that
  session (the engine's `journal: None` path is the same as the
  pre-Phase-20 behaviour). The user sees no resume modal; the
  copy still works. A follow-up phase will add a "journal
  unhealthy" toast so the user can manually recover.

## Phase 21 — bandwidth shaping (shipped)

Phase 21 adds a GCRA token bucket on the engine's byte-by-byte
read loop plus a user-editable schedule / auto-throttle table in
Settings → Network. Threat-model deltas:

- **No new persistent surface.** Shape state lives in RAM
  (`AppState::shape: Arc<Shape>`); only the user's configuration
  writes to disk, via the existing `settings.toml` round-trip.
  Settings are still `#[serde(default)]`-gated — an older binary
  loading the Phase 21 file silently drops the `[network]` table
  and keeps running.
- **No new network calls.** Despite the name, "network settings"
  here means the user's local link class (metered / battery /
  cellular); Copy That still never phones home. The auto-throttle
  probes in `copythat_shape::auto` are stubbed to
  `Unmetered` / `PluggedIn` in Phase 21 and will use OS-native
  APIs only (`INetworkCostManager` on Windows,
  `NWPathMonitor` on macOS, NetworkManager DBus + `battery` on
  Linux) when the per-OS bridges land — no HTTP / DNS / anything
  external.
- **Shape parser rejects malformed input.** `Schedule::parse`
  surfaces typed errors (`MissingComma`, `InvalidKey`,
  `InvalidRate`, `TimeOutOfRange`, `UnknownDay`) so a user who
  types `25:00,512k` or `Foo-Sun,10M` gets inline feedback in the
  textarea rather than a panic or silent cap-of-zero. The
  `validate_schedule_spec` IPC lints before persisting.
- **Shape cannot corrupt a transfer.** `Shape::permit` is a pure
  delay primitive — it blocks the engine's read loop but never
  touches the bytes. BLAKE3 verify still fires at the same cadence
  whether a shape is attached or not.
- **Paused shape is not a DoS risk.** A `Shape::set_rate(Some(0))`
  path makes `permit` block indefinitely, which in turn pauses
  the copy. The copy's `CopyControl::cancel()` remains responsive
  (the engine checks `is_cancelled` at the top of each loop turn
  before calling `permit`), so the user can always abort a shape-
  paused job from the UI.
- **No data leaked via rate timing.** Unlike an encryption
  side-channel, the shape's delay is user-configured and
  unrelated to file contents. An observer of the transfer wall
  time learns nothing beyond the configured cap.

## Phase 24 — security metadata preservation (shipped)

The Phase 24 metadata pass captures and replays out-of-band
streams (NTFS ADS / Linux + macOS xattrs / POSIX ACLs / SELinux /
Linux file capabilities / macOS resource forks) so security-
sensitive flags survive a copy. The threat model:

- **Mark-of-the-Web preserves SmartScreen / Office Protected View
  warnings.** A downloaded `.exe` / `.docx` carries a Windows
  `Zone.Identifier` ADS that SmartScreen and Office Protected View
  read to decide whether to warn the user before execution. A
  copy that drops MOTW silently turns "downloaded from the
  internet" into "trusted local file" — the engine's default
  behaviour now preserves the stream so the OS-level warning
  fires at the destination too. The Settings → Transfer →
  "Preserve Mark-of-the-Web" toggle defaults ON; the UI tooltip
  carries an explicit warning that disabling it is dangerous.
- **POSIX ACLs and SELinux contexts survive the trip.** Linux
  daemons running under MAC policies (`unconfined_u:...:s0`) need
  the `security.selinux` label to remain accurate after a copy or
  they lose access to the destination. Same for POSIX
  `system.posix_acl_*` entries — a copy that drops them silently
  widens or narrows access. The default-on toggles preserve both;
  per-stream policy flags let the user opt out per scenario.
- **Linux file capabilities make the trip.** The
  `security.capability` xattr carries cap_net_admin / cap_setuid
  /etc on `setcap`-enabled binaries. Dropping it on copy turns a
  privileged helper into a non-functional one — Phase 24 keeps
  it by default.
- **macOS resource forks + Finder color tags survive.** The
  legacy `..namedfork/rsrc` stream and `com.apple.FinderInfo`
  xattr carry Carbon metadata + Finder color tags; older
  workflows still depend on them.
- **AppleDouble fallback is a fidelity feature, not a leak.**
  Cross-FS destinations (SMB / FAT / exFAT / ext4) that cannot
  hold the foreign metadata get an `._<filename>` AppleDouble v2
  sidecar carrying the unsupported streams. The sidecar contains
  exactly the metadata the source had — no new attack surface;
  the same bytes the user was already storing.
  `MetaPolicy::appledouble_fallback` is the master toggle; a user
  who prefers to lose the metadata rather than write a sidecar
  flips it off.
- **Capture and apply both run in `spawn_blocking` workers.** The
  underlying syscalls (`FindFirstStreamW`, `getxattr`, `setxattr`,
  `..namedfork/rsrc` open) are blocking; the engine isolates them
  off the tokio scheduler so a slow xattr table on a network
  share can't block the copy loop.
- **Metadata apply failures are non-fatal.** The byte copy
  finishes first; the apply pass runs after timestamps and
  permissions and downgrades per-stream errors into
  `MetaApplyOutcome::partial_failures`. A `setxattr` permission-
  denied on the destination never aborts a copy that already
  succeeded.
- **The `xattr` crate is MIT/Apache-2.0**, dual-licensed, and
  re-uses libc for the underlying syscalls. No new transitive
  dependencies on Windows. No new network calls.

## Build hardening (target, post-Phase 17)

- Stack probes enabled.
- Windows: Control Flow Guard (`/guard:cf`).
- macOS arm64: PAC + BTI.
- Linux: full RELRO + BIND_NOW.
- No `unsafe` code without an explicit `// SAFETY:` comment that the
  reviewer signed off on.

## What we will not do

- Telemetry. The app does not phone home. Any future opt-in telemetry will
  be disabled by default and clearly disclosed in Settings.
- Network calls during normal copy operations.
- Auto-update without explicit user consent (Phase 15 will be opt-in by
  default for the install action; checking is on by default but throttled
  to once per 24 h).
