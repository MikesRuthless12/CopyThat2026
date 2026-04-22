#!/usr/bin/env bash
# Phase 16 smoke test wrapper (Linux).
#
# Drives `tests/smoke/phase_16_package.rs` — a filesystem-only
# tripwire that asserts the free-first packaging path stays free.
# CI uses the same invocation on the ubuntu-latest matrix leg.
set -euo pipefail
cd "$(dirname "$0")/../.."
exec cargo test -p copythat-ui --test phase_16_package -- --nocapture "$@"
