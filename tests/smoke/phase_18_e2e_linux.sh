#!/usr/bin/env bash
# Phase 18 end-to-end smoke wrapper (Linux).
#
# Drives `tests/smoke/phase_18_e2e.rs`. Default 50 files (~60 s);
# pass `--full` to scale to 10 000 files for the manual release
# rehearsal (takes minutes, not seconds — reserved for
# docs/RELEASE_CHECKLIST.md).
#
# `install` / `uninstall` are NOT exercised here — those operate
# on the tagged artifacts produced by .github/workflows/release.yml
# and require a clean user profile on each OS. See the manual
# checklist entries in docs/RELEASE_CHECKLIST.md.
set -euo pipefail
cd "$(dirname "$0")/../.."

if [[ "${1:-}" == "--full" ]]; then
    export COPYTHAT_PHASE18_FULL=1
    echo "[phase 18] --full: 10 000 files, expect minutes."
fi

exec cargo test -p copythat-ui --test phase_18_e2e -- --nocapture
