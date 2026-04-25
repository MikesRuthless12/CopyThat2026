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

## Phase 34 — audit log export + WORM

- **Tamper-evidence by chain hash.** Every line in the audit log
  includes a `prev_hash` column with the hex-encoded BLAKE3 of the
  previous record's bytes. Modifying any earlier line requires
  recomputing every subsequent chain hash; `verify_chain` catches
  single-byte edits at O(n) with no external state.
- **WORM enforcement is opt-in per-install.** When the user enables
  Settings → Advanced → Audit log → WORM, Copy That applies the
  platform's append-only primitive after each create / rotation:
  `FS_IOC_SETFLAGS | FS_APPEND_FL` on Linux (requires
  CAP_LINUX_IMMUTABLE — surfaced as a clear error when missing),
  `chflags(UF_APPEND)` on macOS (userspace-legal), and
  `FILE_ATTRIBUTE_READONLY` on Windows (the richer deny-write ACE
  path is a Phase 36 follow-up). Attempting to truncate a WORM-
  flagged log fails at the kernel level — even for the Copy That
  process that wrote it.
- **Rotation preserves the prior log.** When `bytes_written ≥
  max_size_bytes` the sink renames the file to `<path>.1`, opens a
  fresh primary, and re-applies WORM to the new file. The rotated
  `.1` retains its own append-only flag so a rollover cannot be
  used as a way to evade WORM on the active file.
- **Sink open is fail-closed.** A WORM apply error at open time
  surfaces `AuditError::WormApply` — the runner leaves the
  registry empty and logs the reason to stderr; no audit records
  are dropped silently on a pretend-sink.
- **No credential material in the log.** The
  `SettingsChanged` record carries *hashes* (SHA-256 of the TOML
  serialisation) of the before + after states, not the content.
  An auditor can see that a setting changed without leaking a
  credential that happens to live in the settings blob.
- **No network writes in Phase 34.** The `syslog_destination`
  field is persisted for the Phase 36 CLI's pipe-to-syslog
  follow-up but today only the file path sink is open.
- **`cargo deny` coverage.** The three new direct deps —
  `csv` 1 (MIT/Apache-2.0), `nix` 0.29 (MIT, Linux-only), and
  `gethostname` 1 (BSD-3-Clause) — are all in the permissive
  allowlist; no exceptions added.

## Phase 35 — destination encryption + on-the-fly compression

- **Encryption is opt-in per install.** A fresh install ships
  `Settings::crypt.encryption_mode = "off"` — no transform runs,
  no extra files appear, and the engine path is byte-for-byte
  identical to pre-Phase-35 behaviour. The user must explicitly
  pick `passphrase` or `recipients` from Settings → Transfer →
  Encryption to enable the pipeline.
- **age format wire-compatibility.** Encrypted destinations are
  bit-for-bit identical to what the upstream `rage` CLI produces
  (`age -r <recipient> <file>` / `age -p <file>`). Decryption
  works without Copy That in the loop — a user who later loses
  this app can still decrypt with the official `rage` binary
  using the same passphrase or X25519 / SSH key.
- **No key material on disk.** Passphrases live in
  `secrecy::SecretString` for the duration of a copy; they're
  never persisted to `settings.toml`. X25519 / SSH recipients are
  read from a user-supplied recipients file path stored in
  Settings — the *path* is persisted, not the key bytes.
  (Identity files for *decryption* will live in `<config-dir>/keys/`
  with file-mode 600 once the import-keys flow lands; today the
  decrypted_reader path takes a pre-loaded `Identity` from
  whichever source the caller chose.)
- **Verify is auto-disabled when transform runs.** A byte-exact
  post-copy hash against an age / zstd destination would always
  mismatch (the destination bytes differ from the source by
  construction). The runner strips any verifier the user
  configured when it attaches the crypt hook so users don't see
  spurious "verify failed" errors. Compression integrity is
  still guaranteed by the format itself: zstd carries a 32-bit
  XXH64 checksum per frame, age writes per-chunk MACs.
- **Compression cannot exfiltrate or alter.** The zstd encoder is
  a pure data-transform — it can't reach the network or read
  files outside what the engine hands it. The deny-extension
  list (jpg / mp4 / zip / pdf / msi / iso / …) is a correctness
  optimisation: re-compressing already-compressed data wastes CPU
  and may *grow* the file slightly, so Smart mode skips them.
- **CRIME / BREACH-style attacks are not in-scope.** The crypt
  pipeline is "compress then encrypt" because that's what produces
  good ratios on plain text + small ciphertext overhead. Compress-
  then-encrypt has well-known leakage characteristics when the
  attacker controls part of the input (CRIME / BREACH against
  TLS); for offline file backups + one-shot copies this is not
  the threat model. A future opt-in flag could disable
  compression for sensitive workloads.
- **`cargo deny` coverage.** The three new direct deps —
  [`age`](https://crates.io/crates/age) 0.11 (MIT/Apache-2.0),
  [`zstd`](https://crates.io/crates/zstd) 0.13 (MIT/Apache-2.0;
  zstd-sys ships under BSD/MIT), and
  [`secrecy`](https://crates.io/crates/secrecy) 0.10
  (MIT/Apache-2.0) — are all in the permissive allowlist; no
  exceptions added. Transitive crypto crates (chacha20poly1305,
  x25519-dalek, scrypt, sha2, hmac, hkdf) are all RustCrypto-
  ecosystem MIT/Apache-2.0.

## Phase 36 — `copythat` CLI surface

The Phase 36 CLI changes the trust boundary in two narrow ways:

- **TOML jobspec parsing.** `copythat plan` and `copythat apply`
  read a user-controlled TOML file. The parser is `toml` 0.8 (the
  same version `copythat-settings` already pins; no new attack
  surface). `JobSpec::validate` enforces semantic invariants the
  TOML grammar can't express: source paths must exist on disk before
  `apply` runs, and the destination's parent must exist (or the
  destination itself must, for already-populated trees). The engine
  re-runs lexical traversal-safety checks on every per-file
  `copy_file` invocation as it has since Phase 17a — passing a
  jobspec that contains `..` components in a source path does not
  bypass the `copythat_core::safety::validate_path_no_traversal`
  guard.

- **Exit-code-driven CI integration.** Nine documented exit codes
  (`0`–`9`) are the contract for downstream automation. They are
  declared as a `#[repr(u8)]` enum so the numeric values cannot
  drift across releases without a deliberate code change. CI scripts
  that branch on exit code are stable across patch versions; a
  semver-major change is the only path where the discriminants can
  move, and any such move requires a `### Changed BREAKING:`
  CHANGELOG entry.

What Phase 36 deliberately does not change: the `--config <PATH>`
override does not bypass the `copythat-settings` TOML schema —
malformed configs reject with `ExitCode::ConfigInvalid` before any
mutation. The `config set <key> <value>` round-trip writes through
`Settings::save_to`'s atomic stage-and-rename, so a partial write
cannot leave the settings file in a half-baked state.

### Inherited transitive advisories ignored in `deny.toml`

Phase 36 documents six pre-existing `cargo deny check advisories`
exceptions that earlier phases (32 / 33 / 35) shipped behind
`[skip ci]` markers without attaching a threat-model note. The new
entries each carry an inline provenance comment plus an explanation
of why the advisory does not materialise in CopyThat's threat model:

- **RUSTSEC-2023-0071** (Marvin Attack on `rsa` 0.9). Inherited
  through `age` (Phase 35) and `reqsign` (Phase 32). The Marvin
  Attack requires an attacker to observe RSA operation timing across
  many requests over the network. CopyThat is a local file-copying
  tool: RSA operations run on the user's own machine, never against
  attacker-influenced data delivered over a network channel. The
  upstream `rsa` advisory text says "local use on a non-compromised
  computer is fine"; that's the regime CopyThat operates in.
- **RUSTSEC-2021-0119** (`nix 0.19` getgrouplist OOB). Inherited
  through `battery 0.7` for Linux power-source enumeration.
  Exploitation requires a malicious /etc/group on the local
  machine, which is the same trust boundary as the file copy
  itself — if the attacker can edit /etc/group they already have
  root and don't need a vulnerability to read your files.
- **RUSTSEC-2025-0026** (`registry 1.3` archived). Inherited through
  `winfsp_wrs_sys`'s build script for the Windows mount path. Not a
  vulnerability — the upstream just archived in favour of
  `windows-registry`.
- **RUSTSEC-2021-0154** (`fuser 0.15` uninitialized memory read).
  Inherited through the FUSE mount path on Linux. The unsoundness
  materialises only in narrow handcrafted-syscall edge cases; the
  bytes the read returns are kernel-supplied FUSE message header
  bytes, which the kernel zero-fills before delivery in normal use.
- **RUSTSEC-2020-0168** (`mach 0.3` archived). Inherited through
  `battery 0.7` on macOS. Replacement candidate is `mach2`; blocked
  on a `battery` upstream version bump.
- **RUSTSEC-2025-0052** (`async-std` discontinued). Inherited
  through `fuser`'s default-feature stack. The runtime swap to
  `tokio` requires a fuser upstream change.

Each entry is tagged for re-audit when its upstream chain ships a
fix. Any new vulnerability advisory (RUSTSEC ID without
`unmaintained`) MUST be evaluated case by case before the entry can
land in `deny.toml`'s ignore list — the policy is documented in the
file's header comment.

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
