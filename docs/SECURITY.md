# Security policy

Copy That 2026 is in early development (Phase 0 — scaffold). The rules below
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
