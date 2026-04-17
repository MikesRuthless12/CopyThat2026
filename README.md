# Copy That 2026

A lightweight, cross-platform, async, byte-exact file/folder copier in Rust —
matching every feature of TeraCopy and pushing past it, while staying as fast
as (or faster than) Explorer / Finder / `cp` / `rsync` for typical desktop
workloads.

> **Status:** Phase 2 — tree operations + in-memory queue.
> `copythat-core` now does `copy_file`, `copy_tree`, `move_file`, and
> `move_tree` with pause / resume / cancel, throttled progress events,
> six collision policies (Skip / Overwrite / OverwriteIfNewer /
> KeepBoth / Rename / Prompt), and a broadcast-backed job queue. No
> GUI wiring yet; that's Phase 5.

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
| License        | MIT **or** Apache-2.0, your choice      |

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

## Roadmap

See [`docs/ROADMAP.md`](docs/ROADMAP.md). Eighteen ordered phases. Each phase
ships under [Standing Per-Phase Rules](CopyThat2026-Build-Prompts-Guide.md):
docs + i18n + smoke test + green build + Conventional-Commits commit.

## License

Licensed under either of [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE)
at your option. See [`LICENSE`](LICENSE) for the dual-license note.
