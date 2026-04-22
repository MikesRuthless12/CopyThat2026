#!/usr/bin/env bash
# Phase 18 end-to-end smoke wrapper (macOS). Same body as the Linux
# wrapper — the underlying Rust test is platform-agnostic.
set -euo pipefail
cd "$(dirname "$0")/../.."

if [[ "${1:-}" == "--full" ]]; then
    export COPYTHAT_PHASE18_FULL=1
    echo "[phase 18] --full: 10 000 files, expect minutes."
fi

exec cargo test -p copythat-ui --test phase_18_e2e -- --nocapture
