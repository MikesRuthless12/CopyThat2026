#!/usr/bin/env bash
# Phase 45 smoke test — drag-progress-merge + named queues + F2 +
# tray destination targets.
#
# Phase 45 ships entirely on top of the existing copythat-core /
# copythat-ui workspace — no new crates — so the smoke is a focused
# orchestration of the per-sub-phase tests + a clean svelte-check +
# vite build pass. Each sub-phase already carries its own targeted
# test:
#
#   - 45.1 backend data model    -> crates/copythat-core/tests/queue_registry.rs (8/8)
#   - 45.2 Tauri IPC layer       -> apps/copythat-ui/src-tauri/tests/queue_commands.rs
#                                   (10/10 from 45.2 + the unpin case added in 45.6)
#   - 45.3 UI tabs + queueId     -> command_layer.rs (incl. JobDto split + locale
#                                   parity over the 19 new Phase-45 Fluent keys)
#   - 45.4 drag-merge UI         -> svelte-check + vite build (frontend-only slice)
#   - 45.5 F2-mode UX            -> queue_commands::set_f2_mode_flips_registry_flag
#                                   covers the IPC half; pulse class is manual
#   - 45.6 tray destinations     -> queue_commands::unpin_destination_removes_match
#                                   _and_is_idempotent + tray menu rebuild path
#
# Exits non-zero on any failure.
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
UI_DIR="$REPO_ROOT/apps/copythat-ui"

step() { printf '\n\033[1;36m==> %s\033[0m\n' "$*"; }

step "Phase 45 smoke: cargo test -p copythat-core --test queue_registry"
( cd "$REPO_ROOT" && cargo test -p copythat-core --test queue_registry )

step "Phase 45 smoke: cargo test -p copythat-ui --test queue_commands"
( cd "$REPO_ROOT" && cargo test -p copythat-ui --test queue_commands )

step "Phase 45 smoke: cargo test -p copythat-ui --test command_layer"
( cd "$REPO_ROOT" && cargo test -p copythat-ui --test command_layer )

step "Phase 45 smoke: cargo check -p copythat-ui (Tauri runtime build)"
( cd "$REPO_ROOT" && cargo check -p copythat-ui )

step "Phase 45 smoke: pnpm check (svelte-check)"
( cd "$UI_DIR" && pnpm check )

step "Phase 45 smoke: pnpm build (vite)"
( cd "$UI_DIR" && pnpm build )

echo
echo "Phase 45 smoke: PASS"
