# Phase 42 — Cargo.lock Hygiene + Duplicate Dependency Audit

This document captures the workspace-wide audit of duplicate
dependency versions performed after the wave-1 fix swarm landed
its security and correctness commits. The wave-1 swarm added the
following direct dependencies that needed to be screened for
gratuitous version skew:

| Crate added | Where added | Wave-1 commit |
|-------------|-------------|---------------|
| `hmac = "0.12"` | `apps/copythat-ui/src-tauri` (cfg-windows) | `8cff232 fix(broker): per-message HMAC auth` |
| `getrandom = "0.2"` | `apps/copythat-ui/src-tauri` (cfg-windows) | `8cff232 fix(broker): per-message HMAC auth` |
| `subtle = "2"` | `crates/copythat-audit` | `d1d28b6 fix(crypto+audit): constant-time compare` |
| `hmac = "0.12"` | `crates/copythat-mobile` | `ef60a42 fix(mobile): pairing nonce challenge` |

None of these introduced fresh major-version skew with the existing
workspace tree:

- **`hmac 0.12`** is the stable RustCrypto line. It already shipped
  in `copythat-cloud` (Phase 32i, HMAC-SHA1 for OpenSSH known_hosts
  hashed entries). The wave-1 additions in `copythat-mobile` and
  `copythat-ui` (Tauri) join that resolution. The other `hmac` line
  in the lockfile (`v0.13.0`) is a pre-release dragged in by
  `internal-russh-forked-ssh-key`'s RC stack — see "Necessary skew"
  below.
- **`getrandom 0.2`** in the Tauri broker code joins the existing
  `getrandom 0.2.17` line that `rand_core 0.6.4`, `ring 0.17.14`,
  and `russh` already pull. Bumping to `0.3` would force a code
  change (the API was renamed `getrandom::getrandom` → `fill`) and
  would not eliminate the `0.2` resolution from the lockfile, since
  `rand_core 0.6.4` (used by RSA, age, dalek, x25519-dalek, etc.)
  pins it transitively. Net dup count is unchanged.
- **`subtle = "2"`** in `copythat-audit` joins the existing `subtle
  v2.x` resolution shared by `age`, `chacha20poly1305`, dalek, and
  the entire RustCrypto stack. There is no `subtle 3.x` to skew
  against; this is a clean unification.
- **`compio` (optional, gated)** — the wave-1 swarm did not actually
  add this crate; the task's brief was speculative. No-op.

## Direct duplicate eliminated by this audit

| Crate | Before (runtime + dev-deps) | After | Action |
|-------|------------------------------|-------|--------|
| `toml` | 3 resolutions (`0.5.11` + `0.8.2` + `0.9.12+spec-1.1.0`) | 2 resolutions (`0.5.11` + `0.9.12+spec-1.1.0`) | Bumped `toml = "0.8"` → `"0.9"` in `copythat-cli`, `copythat-cloud` (dev-dep), `copythat-mobile` (dev-dep), `apps/copythat-ui/src-tauri`. The 4 callers used only the `to_string` / `from_str` / `Value` API which is source-compatible across the bump. `copythat-settings` already pinned `0.9` on the Phase-37 settings rewrite. |
| `toml_datetime` | `0.6.3` + `0.7.5+spec-1.1.0` | `0.7.5+spec-1.1.0` | Cascading removal — `toml_datetime 0.6` was tied to `toml 0.8`. |

Note: `toml 0.8.2` still appears in `Cargo.lock` because `system-deps
6.2.2` (a Linux-only **build-dep** dragged in by `gtk-sys` for Tauri's
GTK shell) requires it. `cargo tree --duplicates` excludes
build-deps by default, so the duplicate doesn't show up in the
audit metric. There is no opportunity to remove it without an
upstream `system-deps` release (tracked at
<https://github.com/gtk-rs/gtk-rs-core>). It costs zero runtime
bytes — it's only present during the GTK build script.

## Necessary skew — duplicates left in the lockfile

Each row below documents a duplicate that **cannot** be unified
without an upstream change or an unacceptable downgrade. The audit
reviewed these and confirms they are tracked, not gratuitous.

### RustCrypto stable + RC stacks (russh)

`russh 0.60` and its sibling `internal-russh-forked-ssh-key` pull
the **release-candidate** RustCrypto stack
(`elliptic-curve 0.14.0-rc.31`, `pkcs5 0.8.0-rc.13`, `pkcs8
0.11.0-rc.11`, `ed25519 3.0.0-rc.4`, `ecdsa 0.17.0-rc.17`, `aes-gcm
0.11.0-rc.3`, `aead 0.6.0-rc.10`, `signature 3.0.0-rc.10`, `sha1
0.11.0`, `sha2 0.11.0`, `digest 0.11.2`, `cipher 0.5.1`, `inout
0.2.2`, `block-buffer 0.12.0`, `block-padding 0.4.2`, `der 0.8.0`,
`spki 0.8.0`, `pem-rfc7468 1.0.0`, `cpufeatures 0.3.0`, `crypto-
common 0.2.1`, `aes 0.9.0`, `cbc 0.2.0`, `ctr 0.10.0`, `salsa20
0.11.0`, `scrypt 0.12.0`, `pbkdf2 0.13.0`, `hkdf 0.13.0`, `hmac
0.13.0`, `chacha20 0.10.0`, `polyval 0.7.1`, `ghash 0.6.0`,
`universal-hash 0.6.1`, `ed25519-dalek 3.0.0-pre.6`, `curve25519-
dalek 5.0.0-pre.6`).

Every other workspace consumer (`age 0.11`, `aes-gcm 0.10.3`,
`copythat-crypt`, `copythat-cloud` direct, `copythat-mobile`,
`reqsign`, `rsa 0.9`, `ssh-cipher 0.2`) is on the **stable** 0.x
line. Bumping the stable callers to RC is unsafe (RC API drift
without a stable guarantee) and downgrading russh would lose the
2024-edition-compatible upstream fixes. **Resolution: live with the
two ladders until russh ships against released versions of the
RustCrypto crates.** Tracked upstream at
<https://github.com/Eugeny/russh>.

### `windows-link 0.1.3` vs `0.2.1` (and the `windows v0.61` vs `v0.62` ecosystems)

The `windows v0.61.x` ecosystem (used by `tauri 2.10.3`, all
`tauri-plugin-*`, `tao 0.34.8`, `wry 0.54.4`, `tray-icon 0.21.3`,
`muda 0.17.2`, `notify 8.2.0`, `keyring 3.6.3`) holds `windows-link
0.1.3`. The newer `windows v0.62.2` ecosystem (used by `chrono
0.4.44`, `gethostname 1.1.0`, `parking_lot_core 0.9.12`, `clap
4.6.1`, `tokio 1.52.1`, `directories 6.0.0`, `rustix 1.1.4`,
`rustls-platform-verifier 0.6.2`, `walkdir 2.5.0`) holds
`windows-link 0.2.1`. Ditto for `windows-core`, `windows-sys`,
`windows-targets`, `windows-numerics`, `windows-future`, `windows-
result`, `windows-strings`, `windows-threading`, `windows-implement`,
`windows-interface`, `windows-collections`, `windows_x86_64_msvc`.
**Resolution: cannot be unified without a Tauri major-version bump
(or a tokio/chrono downgrade). Will resolve when Tauri 2.11+ adopts
the `windows v0.62` line, expected post-2026.05.**

The third resolution `windows v0.57.0` is held by `russh-cryptovec
0.59.0` — even older. Same constraint as above.

### `getrandom 0.1.16` + `0.2.17` + `0.3.4` + `0.4.2`

- `0.1.16` — held by `rand 0.7.3`, used only at compile-time by
  `phf_generator 0.8.0` → `phf_codegen 0.8.0` (build-dep of
  `selectors 0.24.0` → `kuchikiki 0.8.8-speedreader` →
  `tauri-utils`). Build-dep only; no runtime cost.
- `0.2.17` — pinned by `rand_core 0.6.4`, `ring 0.17.14`, the wave-
  1 broker auth (cfg-windows), and `const-random-macro 0.1.16`.
  Not removable without ripping out RSA + age + dalek + ring, which
  is the entire crypto floor.
- `0.3.4` — our preferred line: `copythat-cloud`, `copythat-mobile`,
  `copythat-snapshot`, `copythat-helper`, plus `tauri 2.10.3` and
  `rand_core 0.9.5`.
- `0.4.2` — held by `crypto-bigint 0.7.3` (RC, dragged in by
  `russh`), `tempfile 3.27.0`, `uuid 1.23.1`, `rand 0.10.1`. Bumping
  `tempfile` and `uuid` won't help (they already use 0.4); the
  resolution belongs to the same russh-RC family above.

**Resolution: leave as-is. Each line is required by an upstream
crate we cannot upgrade away.**

### `rand 0.7.3` + `0.8.5` + `0.9.4` + `0.10.1`

- `0.7.3` — `phf_generator 0.8.0` build-dep only.
- `0.8.5` — `age`, `age-core`, `num-bigint-dig` (via `rsa`),
  `pageant`, `reqsign`, `rand_core 0.6.4` (transitive default).
- `0.9.4` — `copythat-secure-delete` (direct, Phase-25 SDelete
  passes), `governor 0.10.4` (via `copythat-shape`).
- `0.10.1` — `russh 0.60.1`, `internal-russh-num-bigint`, the RC
  RustCrypto stack.

Each of `rand_core 0.5.1` + `0.6.4` + `0.9.5` + `0.10.1` and
`rand_chacha 0.2.2` + `0.3.1` + `0.9.0` mirrors the same skew.
Cannot be unified without bumping `age 0.11` (no released `0.12`
yet — upstream tracker open) and breaking `rsa`'s
`num-bigint-dig` floor.

### `phf 0.8` + `0.10.1` + `0.11.3` (+ `phf_codegen`, `phf_generator`, `phf_macros`, `phf_shared`, `siphasher`)

The Tauri ecosystem uses `phf 0.11.3` (via `markup5ever 0.14.1` /
`tauri-utils`). The `cssparser 0.29.6` ecosystem (transitive from
`kuchikiki` / `selectors`) uses `phf 0.8.0` and `0.10.1`. All
build-time only; no runtime impact. Cannot be unified without
forking `cssparser`.

### `bitflags 1.3.2` + `2.11.1`

`bitflags 1.3.2` is held by `png 0.17.16` (via `ico 0.5.0` →
`tauri-codegen`) and `selectors 0.24.0` (transitive). All build-
time. The remainder of the workspace is on `2.11.1`. Not removable.

### `base64 0.21.7` + `0.22.1`

`0.21.7` is held by `age 0.11.3` and `age-core 0.11.0`. The rest
of the workspace pins `0.22.1`. Pre-emptive bump requires `age
0.12` (not yet released). Tracked upstream.

### `quick-xml 0.37.5` + `0.38.4`

`0.37.5` is held by `reqsign 0.16.5`; `0.38.4` is held by `opendal
0.54.1`. Both belong to the OpenDAL family — `reqsign` is the
crate OpenDAL itself uses for AWS-SigV4 signing. Cannot be unified
without an `opendal` patch release that also bumps `reqsign`.

### `serde v1.0.228` (×2) + `serde_core v1.0.228` (×2) + `serde_json v1.0.149` (×2) + `serde_spanned 0.6.9` + `1.1.1` + `smallvec 1.15.1` (×2) + `stable_deref_trait 1.2.1` (×2) + `typenum 1.19.0` (×2) + `crypto-common 0.1.7` (×2) + `digest 0.10.7` (×2) + `sha2 0.10.9` (×2) + `serde_json v1.0.149` (×2)

These all show twice in `cargo tree --duplicates` despite being the
**same version**. Cargo flags them because the feature sets differ
between consumers (e.g., `serde` with `derive,std,alloc` vs `serde`
with only `std`). The lockfile holds **one** entry per
(crate, version, source) triple — these are not actual disk
duplicates. **No action required.**

### `i18n-embed v0.15.4` (×2) + `unic-langid 0.9.6` (×2) + `unic-langid-impl 0.9.6` (×2) + `tauri-utils v2.8.3` (×2) + `log v0.4.29` (×2) + `event-listener` + `chacha20`

Same-version-different-features class. `i18n-embed 0.15.4` is
listed twice because some consumers enable the `desktop-requester`
feature and others don't, but cargo unifies the build. Not a true
duplicate.

### `thiserror 1.0.69` + `2.0.18`

`thiserror 1.x` is held by `i18n-config 0.4.8`, `i18n-embed`,
`fluent-syntax 0.11.1`, `pageant 0.2.0`, `json-patch 3.0.1` —
upstream fluent / i18n stack still on the v1 line. Every
workspace crate uses `thiserror = "2"`. Cannot be unified without
forking the fluent stack.

### Misc transitive duplicates

- `event-listener 2.5.3` (`async-channel 1.9.0` only) + `5.4.1`
  (everything else). Tracked at <https://github.com/smol-rs/async-channel>.
- `indexmap 1.9.3` + `2.14.0` — the 1.x line is held by
  `serde_json` (still maintains a 1.x-compat path) and `find-crate`;
  cannot be removed.
- `hashbrown 0.12.3` + `0.14.5` + `0.16.1` + `0.17.0` —
  transitive ladder from `indexmap 1.9.3` (0.12) to `dashmap 6.x`
  (0.16) to `tokio` (0.17). Each consumer pins a different hash
  table API.
- `winnow 0.5.40` + `0.7.15` + `1.0.1` — `0.5` is held by
  `toml_edit 0.20.2` (transitive via tauri-utils' `cargo_toml`);
  `0.7` by `toml 0.9.x`; `1.0` by `serde_spanned 1.1.1`.
- `untrusted 0.7.1` + `0.9.0` — `0.7` is held by `webpki 0.22`
  (transitive from older TLS code); `0.9` by `ring 0.17.14`.
- `syn 1.0.109` + `2.0.117` — `1.x` is held by `phf_macros 0.10.0`
  + `cssparser` build-deps. The rest of the workspace is on `2.x`.
- `cpufeatures 0.2.17` + `0.3.0` — RC RustCrypto skew.
- `rustc-hash 1.1.0` + `2.1.2` — `1.x` is held by `fluent-bundle
  0.15.3` (i18n stack). `2.x` everywhere else.
- `self_cell 0.10.3` + `1.2.2` — `0.10` is held by `fluent-bundle`;
  `1.2` everywhere else.

## Spot-check matrix (cargo check + targeted tests after the bump)

| Command | Result |
|---------|--------|
| `cargo check --workspace` | OK (warnings only — pre-existing) |
| `cargo check -p copythat-cli` | OK |
| `cargo check -p copythat-cloud --tests` | OK |
| `cargo check -p copythat-mobile --tests` | OK |
| `cargo check -p copythat-ui` | OK |
| `cargo test -p copythat-platform --lib` | 42 passed / 0 failed |

## Net change

| Metric | Before audit | After audit |
|--------|--------------|-------------|
| Total `cargo tree --duplicates` rows | 197 | 191 |
| `toml` resolutions | 3 (`0.5.11` + `0.8.2` + `0.9.x`) | 2 (`0.5.11` + `0.9.x`) |
| `toml_datetime` resolutions | 2 | 1 |
| Wave-1 introduced major-version skew | 0 (verified) | 0 |

The remaining duplicates are all upstream-bound — they belong to
distinct ecosystems (russh-RC vs RustCrypto-stable, Tauri-`windows
0.61` vs tokio-`windows 0.62`, age-`base64 0.21` vs everything-`0.22`,
i18n-stack-`thiserror 1` vs workspace-`2`, OpenDAL-internal
`quick-xml 0.37` vs `0.38`, etc.) and resolving any of them
requires an upstream release we do not control.

When upstream tracking moves (russh ships against released
RustCrypto, Tauri 2.11 adopts `windows 0.62`, age 0.12 lands), the
hygiene pass should be repeated.
