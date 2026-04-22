#!/usr/bin/env bash
# Phase 16 smoke test wrapper (macOS). Same Rust test body as the
# Linux wrapper — the tripwire is filesystem-only and doesn't need
# platform branching. CI uses this on the macos-latest leg.
set -euo pipefail
cd "$(dirname "$0")/../.."
exec cargo test -p copythat-ui --test phase_16_package -- --nocapture "$@"
