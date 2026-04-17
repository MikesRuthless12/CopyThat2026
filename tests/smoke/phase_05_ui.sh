#!/usr/bin/env bash
# Phase 5 smoke test — macOS / Linux.
#
# Proves the Tauri shell layer is production-buildable end-to-end:
#   1. `pnpm install` resolves with the lockfile clean.
#   2. `pnpm check` (svelte-check) finds zero errors and zero warnings.
#   3. `pnpm build` produces a Vite bundle.
#   4. The Rust command layer's integration tests (driven against
#      the real `copythat-core` engine, minus the webview) pass —
#      this covers `start_copy` / pause / resume / cancel round-trips
#      plus the IPC DTO and i18n loader.
#   5. `xtask i18n-lint` confirms 18-locale key parity.
#
# Exits non-zero on any failure.
#
# A full tauri-driver / Playwright E2E that drags a folder onto the
# webview lives in Phase 8's error-handling pass — it depends on
# the collision modal and retry dialogs that phase introduces, and
# on a stable tauri-driver install across Windows/macOS/Linux. The
# command-layer test here covers the same contract (enqueue →
# progress events → completion) without booting the webview.
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
UI_DIR="$REPO_ROOT/apps/copythat-ui"

step() { printf '\n\033[1;36m==> %s\033[0m\n' "$*"; }

step "Phase 5 smoke: pnpm install --frozen-lockfile"
( cd "$UI_DIR" && pnpm install --frozen-lockfile )

step "Phase 5 smoke: pnpm check (svelte-check)"
( cd "$UI_DIR" && pnpm check )

step "Phase 5 smoke: pnpm build (vite)"
( cd "$UI_DIR" && pnpm build )

step "Phase 5 smoke: cargo test -p copythat-ui --lib (Rust command layer)"
( cd "$REPO_ROOT" && cargo test -p copythat-ui --lib )

step "Phase 5 smoke: cargo test -p copythat-ui --test command_layer (integration)"
( cd "$REPO_ROOT" && cargo test -p copythat-ui --test command_layer )

step "Phase 5 smoke: cargo run -p xtask -- i18n-lint"
( cd "$REPO_ROOT" && cargo run -p xtask --quiet -- i18n-lint )

echo
echo "Phase 5 smoke: PASS"
