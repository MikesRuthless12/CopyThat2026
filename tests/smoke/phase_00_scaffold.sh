#!/usr/bin/env bash
# Phase 0 smoke test — proves the empty scaffold compiles end-to-end:
#   1. `cargo build --all` for the entire workspace.
#   2. `pnpm tauri build --debug` for the Tauri shell.
# Exits non-zero on any failure.
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

step() { printf '\n\033[1;36m==> %s\033[0m\n' "$*"; }

step "Phase 0 smoke: cargo build --all"
( cd "$REPO_ROOT" && cargo build --all )

step "Phase 0 smoke: pnpm install --frozen-lockfile (apps/copythat-ui)"
( cd "$REPO_ROOT/apps/copythat-ui" && pnpm install --frozen-lockfile )

step "Phase 0 smoke: pnpm tauri build --debug (apps/copythat-ui)"
( cd "$REPO_ROOT/apps/copythat-ui" && pnpm tauri build --debug )

step "Phase 0 smoke: PASS"
