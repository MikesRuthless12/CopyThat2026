# Copy That 2026

A lightweight, cross-platform, async, byte-exact file/folder copier in Rust —
matching every feature of TeraCopy and pushing past it, while staying as fast
as (or faster than) Explorer / Finder / `cp` / `rsync` for typical desktop
workloads.

> **Status:** Phase 11a — i18n core + MT drafts + RTL + ICU formatting.
> Every user-visible string in the app now flows through Fluent
> (167 keys across 18 locales). All 17 non-English locales ship
> full MT drafts (Phase 4/5/8/9/10 translations plus the 20
> Phase 11a additions), each line marked `# MT` for human review.
> RTL is wired for Arabic (`dir="rtl"` on `<html>`), the temporary
> header language-switcher hot-swaps locales without a restart
> (Phase 12's Settings window will host the permanent control),
> `Intl.NumberFormat` renders in the active locale for byte /
> percent / number output, and `formatEta` routes every unit word
> through Fluent. `xtask i18n-lint` now also verifies literal-key
> coverage from source and Fluent syntax (duplicate-key reject),
> in addition to the existing key-parity check. Per-locale
> pixelmatch visual-regression is deferred to Phase 18 polish.
>
> _Previous status: Phase 6 — platform-specific fast paths._
> `crates/copythat-platform` now ships a `fast_copy(src, dst, opts,
> ctrl, events)` dispatcher that attempts, in order: reflink (Linux
> Btrfs / XFS-with-reflink / ZFS / bcachefs, macOS APFS, Windows
> ReFS / Dev Drive via the `reflink-copy` crate); the OS-native
> accelerated path (`CopyFileExW` with `COPY_FILE_NO_BUFFERING` for
> files ≥256 MiB on Windows, `copyfile(3)` with `COPYFILE_ALL` on
> macOS, `copy_file_range(2)` with a `sendfile(2)` fallback for files
> <2 GiB on Linux); and finally the Phase 1
> [`copythat_core::copy_file`] async loop. The dispatcher honours
> `CopyOptions::strategy` (`Auto` / `AlwaysAsync` / `AlwaysFast` /
> `NoReflink`) and reports back which strategy actually moved the
> bytes via `FastCopyOutcome::strategy`. New helpers — `is_ssd(path)`,
> `filesystem_name(path)`, `supports_reflink(path)`, and
> `recommend_concurrency(src, dst, requested)` (clamps to 1 when
> either side is on rotational media to avoid HDD seek thrash). The
> seam into `copythat-core` is a `FastCopyHook` trait carried on
> `CopyOptions`; drop a `PlatformFastCopyHook` in and every per-file
> `copy_file` (and therefore every leaf of `copy_tree`) routes through
> the OS acceleration path. The hook is bypassed when `verify` is
> set, since the verify pipeline relies on hashing the source bytes
> during the write loop. Smoke test: Windows runner copies a 64 MiB
> sparse file via `CopyFileExW` at ~2.6 GiB/s. The next phase
> (Phase 7) wires the engine into the OS shell context menus and
> drag-and-drop.

## Targets

- Windows 10+
- macOS 12+ (Monterey and later)
- Linux (Ubuntu 22.04+, Fedora 38+, Arch, ...)

## Stack

| Concern        | Choice                                  |
| -------------- | --------------------------------------- |
| Language       | Rust (stable, edition 2024, MSRV 1.85)  |
| Async runtime  | `tokio` (added Phase 1)                 |
| GUI shell      | Tauri 2.x + Svelte 5 + TypeScript + Vite |
| Verify hashes  | CRC32 / MD5 / SHA-1/256/512 / xxHash3 / BLAKE3 |
| Persistence    | `rusqlite` (bundled SQLite)             |
| i18n           | Fluent (`.ftl`), 18 locales             |
| Packaging      | `tauri bundle` (MSI / NSIS / DMG / AppImage / deb / rpm) |
| License        | All Rights Reserved (proprietary; source-visible) |

Every dependency is permissively licensed. `cargo deny check` runs in CI and
fails the build if any transitive dependency falls outside the allowlist
(MIT / Apache-2.0 / BSD-2/3-Clause / ISC / CC0 / Unlicense /
Unicode-DFS-2016 / Zlib / MPL-2.0).

## Repository layout

```
CopyThat2026/
├── crates/
│   ├── copythat-core/           # async copy engine
│   ├── copythat-hash/           # verify hashes
│   ├── copythat-secure-delete/  # multi-pass shredding
│   ├── copythat-history/        # SQLite history
│   ├── copythat-platform/       # OS fast paths + shell hooks
│   └── copythat-i18n/           # Fluent loader
├── apps/copythat-ui/            # Tauri 2.x + Svelte 5 shell
├── xtask/                       # workspace automation
├── locales/<code>/copythat.ftl  # 18 Fluent locale files
├── tests/smoke/                 # per-phase smoke tests
└── docs/                        # architecture, changelog, roadmap, ...
```

## Building

Prerequisites:

- Rust toolchain (stable, ≥ 1.85). Install with [rustup](https://rustup.rs).
- Node.js 20+ and `pnpm` 9+. Install pnpm with `npm i -g pnpm` or via
  [`corepack`](https://nodejs.org/api/corepack.html).
- Platform Tauri prerequisites:
  [docs.tauri.app/start/prerequisites/](https://v2.tauri.app/start/prerequisites/).

Workspace build:

```sh
cargo build --all
```

Tauri debug build:

```sh
cd apps/copythat-ui
pnpm install
pnpm tauri build --debug
```

Lint Fluent key parity across all 18 locales:

```sh
cargo run -p xtask -- i18n-lint
```

Phase 0 smoke test (runs both):

```sh
bash tests/smoke/phase_00_scaffold.sh
```

Phase 1 smoke test (100 MiB async round-trip through `copy_file`):

```sh
cargo test -p copythat-core --test phase_01_core_copy -- --nocapture
```

Phase 2 smoke test (500-file tree copy + move):

```sh
cargo test -p copythat-core --test phase_02_tree_queue -- --nocapture
```

Phase 3 smoke test (500 MiB copy+verify across all 8 hash algorithms):

```sh
cargo test -p copythat-hash --test phase_03_verify --release -- --nocapture
```

Phase 4 smoke test (10 MiB shred across every `ShredMethod`):

```sh
cargo test -p copythat-secure-delete --test phase_04_shred -- --nocapture
```

Phase 5 smoke test (Tauri shell end-to-end: pnpm check + vite build +
`copythat-ui` unit & integration tests + i18n-lint):

```sh
# Windows
pwsh tests/smoke/phase_05_ui.ps1
# macOS / Linux
bash tests/smoke/phase_05_ui.sh
```

Phase 6 smoke test (sparse-file round-trip through the platform fast
paths; logs the chosen strategy — `reflink` / `CopyFileExW` /
`copyfile` / `copy_file_range` / `sendfile` / `async-fallback`):

```sh
# Default 64 MiB; set COPYTHAT_PHASE_06_FULL=1 for the 2 GiB run.
cargo test -p copythat-platform --test phase_06_fast_paths -- --nocapture
```

## License

**All Rights Reserved.** Copyright (c) 2026 Mike Weaver. See [`LICENSE`](LICENSE)
for the full terms.

This repository is publicly visible for reference and review only. No license
or permission is granted to use, copy, modify, distribute, or sell the code,
in whole or in part, without the express prior written permission of the
copyright holder. Workspace dependencies remain under their respective
permissive licenses (MIT / Apache-2.0 / BSD / ISC / CC0 / Unlicense /
Unicode-DFS / Zlib / MPL-2.0 / BSL-1.0); the `cargo deny` policy enforces
that allowlist as the dependency tree evolves.
