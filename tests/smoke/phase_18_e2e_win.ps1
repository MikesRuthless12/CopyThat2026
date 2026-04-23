# Phase 18 end-to-end smoke wrapper (Windows). See the Linux
# wrapper's header for the full contract — the Rust test body is
# platform-agnostic.
$ErrorActionPreference = 'Stop'
$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
Push-Location (Join-Path $scriptDir '..\..')
try {
    if ($args -contains '--full') {
        $env:COPYTHAT_PHASE18_FULL = '1'
        Write-Host "[phase 18] --full: 10 000 files, expect minutes."
    }
    & cargo test -p copythat-ui --test phase_18_e2e -- --nocapture
    if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
} finally {
    Pop-Location
}
