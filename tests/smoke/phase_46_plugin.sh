#!/usr/bin/env bash
# Phase 46 smoke test — sandboxed WASM plugin runtime end-to-end.
#
# Phase 46 spans a new `copythat-plugin` crate (host runtime), four
# sample plugins under `apps/copythat-ui/plugins/`, an `xtask
# build-sample-plugins` orchestrator, and the Settings → Plugins UI in
# the Tauri shell. Each sub-phase already carries focused tests; the
# wrap-phase smoke chains them so a single command exercises every
# slice the user-visible feature touches:
#
#   - 46.1 type surface + 46.2 wasmtime engine + 46.3 sandbox budgets
#       -> crates/copythat-plugin tests:
#            scaffold        (public API drift guard)
#            wasm_runtime    (WAT round-trip + fuel/memory/wall budgets
#                             + 46.7 pre-read clamp regression)
#            manifest        (TOML schema validation)
#   - 46.4 capability gate
#       -> wasm_runtime + sample_plugins exercise CapabilityGrant
#   - 46.5 sample plugins (organize-by-exif, notify-discord,
#                          notify-ntfy, dedup-warning)
#       -> sample_plugins (skips when the WASM artifacts aren't built;
#                          to exercise the bodies, run
#                          `cargo run -p xtask -- build-sample-plugins`
#                          first)
#   - 46.6 Settings → Plugins UI (IPC + on-disk store)
#       -> tests/smoke/phase_46_plugin_ui (8 cases incl. i18n parity
#                                          over 17 keys × 18 locales)
#       -> svelte-check + vite build (the PluginsTab.svelte slice)
#   - 46.7 wrap (this script)
#       -> orchestrates the above end-to-end
#
# Exits non-zero on any failure. CI runs the same chain via
# `xtask build-sample-plugins && bash tests/smoke/phase_46_plugin.sh`
# so the sample-plugin bodies execute alongside the host runtime
# tests; without the xtask step the sample_plugins suite skips per
# its `skip_unless_built!` guard and the rest of the chain still
# passes.
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
UI_DIR="$REPO_ROOT/apps/copythat-ui"

step() { printf '\n\033[1;36m==> %s\033[0m\n' "$*"; }

step "Phase 46 smoke: cargo test -p copythat-plugin --test scaffold"
( cd "$REPO_ROOT" && cargo test -p copythat-plugin --test scaffold )

step "Phase 46 smoke: cargo test -p copythat-plugin --test manifest"
( cd "$REPO_ROOT" && cargo test -p copythat-plugin --test manifest )

step "Phase 46 smoke: cargo test -p copythat-plugin --test wasm_runtime"
( cd "$REPO_ROOT" && cargo test -p copythat-plugin --test wasm_runtime )

step "Phase 46 smoke: cargo test -p copythat-plugin --test sample_plugins"
( cd "$REPO_ROOT" && cargo test -p copythat-plugin --test sample_plugins )

step "Phase 46 smoke: cargo test -p copythat-ui --test phase_46_plugin_ui"
( cd "$REPO_ROOT" && cargo test -p copythat-ui --test phase_46_plugin_ui )

step "Phase 46 smoke: cargo check -p copythat-ui (Tauri runtime build)"
( cd "$REPO_ROOT" && cargo check -p copythat-ui )

step "Phase 46 smoke: pnpm check (svelte-check)"
( cd "$UI_DIR" && pnpm check )

step "Phase 46 smoke: pnpm build (vite)"
( cd "$UI_DIR" && pnpm build )

echo
echo "Phase 46 smoke: PASS"
