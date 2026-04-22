# Phase 16 smoke test wrapper (Windows). Same Rust test body as the
# Linux wrapper — the tripwire is filesystem-only and doesn't need
# platform branching. CI uses this on the windows-latest leg.
$ErrorActionPreference = 'Stop'
$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
Push-Location (Join-Path $scriptDir '..\..')
try {
    & cargo test -p copythat-ui --test phase_16_package -- --nocapture @args
    if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
} finally {
    Pop-Location
}
