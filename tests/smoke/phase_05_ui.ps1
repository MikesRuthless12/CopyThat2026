# Phase 5 smoke test — Windows PowerShell.
#
# Proves the Tauri shell layer is production-buildable end-to-end:
#   1. `pnpm install` resolves with the lockfile clean.
#   2. `pnpm check` (svelte-check) finds zero errors and zero warnings.
#   3. `pnpm build` produces a Vite bundle.
#   4. The Rust command layer's integration tests (driven against
#      the real `copythat-core` engine, minus the webview) pass —
#      this covers `start_copy` / pause / resume / cancel round-trips
#      plus the IPC DTO and i18n loader.
#   5. The Phase 0 scaffold smoke also passes (Tauri debug build
#      exercises the same Rust binary from top to bottom).
#
# Exits non-zero on any failure.
#
# A full tauri-driver / Playwright E2E that drags a folder onto the
# webview lives in Phase 8's error-handling pass — it depends on
# the collision modal and retry dialogs that phase introduces, and
# on a stable tauri-driver install across Windows/macOS/Linux. The
# command-layer test here covers the same contract (enqueue →
# progress events → completion) without booting the webview.

$ErrorActionPreference = "Stop"

$RepoRoot = Resolve-Path (Join-Path $PSScriptRoot "..\..")
$UiDir = Join-Path $RepoRoot "apps\copythat-ui"

function Step {
    param([string]$Message)
    Write-Host ""
    Write-Host "==> $Message" -ForegroundColor Cyan
}

Push-Location $UiDir
try {
    Step "Phase 5 smoke: pnpm install --frozen-lockfile"
    pnpm install --frozen-lockfile
    if ($LASTEXITCODE -ne 0) { throw "pnpm install failed" }

    Step "Phase 5 smoke: pnpm check (svelte-check)"
    pnpm check
    if ($LASTEXITCODE -ne 0) { throw "svelte-check failed" }

    Step "Phase 5 smoke: pnpm build (vite)"
    pnpm build
    if ($LASTEXITCODE -ne 0) { throw "vite build failed" }
}
finally {
    Pop-Location
}

Push-Location $RepoRoot
try {
    Step "Phase 5 smoke: cargo test -p copythat-ui --lib (Rust command layer)"
    cargo test -p copythat-ui --lib
    if ($LASTEXITCODE -ne 0) { throw "copythat-ui lib tests failed" }

    Step "Phase 5 smoke: cargo test -p copythat-ui --test command_layer (integration)"
    cargo test -p copythat-ui --test command_layer
    if ($LASTEXITCODE -ne 0) { throw "copythat-ui integration tests failed" }

    Step "Phase 5 smoke: cargo run -p xtask -- i18n-lint"
    cargo run -p xtask --quiet -- i18n-lint
    if ($LASTEXITCODE -ne 0) { throw "i18n-lint failed" }
}
finally {
    Pop-Location
}

Write-Host ""
Write-Host "Phase 5 smoke: PASS" -ForegroundColor Green
