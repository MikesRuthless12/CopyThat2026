# Phase 17 follow-up - IVssBackupComponents COM port smoke harness.
#
# What it does:
#  1. Confirms VSS service + provider are up.
#  2. Builds the snapshot crate with `--features vss-com`.
#  3. Runs the cargo test that actually drives
#     `IVssBackupComponents::DoSnapshotSet` against a real volume,
#     reading back the device path it produced.
#  4. (Optional) If you pass `-LockedFilePath`, it reads the
#     locked file from the shadow and verifies the bytes match
#     the pattern `lock_file.ps1` writes.
#
# Run this AS ADMINISTRATOR. VSS calls fail with E_ACCESSDENIED
# without admin.

[CmdletBinding()]
param(
    [string]$Volume = 'C:\',
    [string]$LockedFilePath = '',
    [switch]$SkipBuild
)

$ErrorActionPreference = 'Stop'

function Step($msg) {
    Write-Host "==> $msg" -ForegroundColor Cyan
}
function Note($msg) {
    Write-Host "    $msg" -ForegroundColor DarkGray
}

# Verify admin
Step "verifying Administrator privileges"
$admin = ([Security.Principal.WindowsPrincipal] `
            [Security.Principal.WindowsIdentity]::GetCurrent() `
        ).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
if (-not $admin) {
    Write-Error "Run this script in an elevated (admin) PowerShell."
}
Note "admin: ok"

# Verify VSS service running
Step "checking VSS service"
$svc = Get-Service -Name VSS -ErrorAction SilentlyContinue
if (-not $svc) {
    Write-Error "VSS service not found. Reinstall Windows. Just kidding - but VSS is missing."
}
if ($svc.Status -ne 'Running') {
    Note "VSS is $($svc.Status); starting it"
    Start-Service VSS
}
Note "VSS service: $((Get-Service VSS).Status)"

# Show existing shadows so the user has context
Step "current VSS shadow inventory (pre-test)"
& vssadmin list shadows /for=$Volume 2>&1 | Out-String | ForEach-Object { Note $_.TrimEnd() }

# Resolve the repo root robustly. `$PSScriptRoot` can be empty in
# some PowerShell 5.1 invocation paths (`powershell -File ...`),
# so fall back to `$MyInvocation.MyCommand.Path` and finally to
# the current location.
$scriptDir = if ($PSScriptRoot) {
    $PSScriptRoot
} elseif ($MyInvocation.MyCommand.Path) {
    Split-Path -Parent $MyInvocation.MyCommand.Path
} else {
    (Get-Location).Path
}
$repoRoot = Split-Path -Parent $scriptDir
if (-not $SkipBuild) {
    Step "cargo check -p copythat-snapshot --features vss-com"
    Push-Location $repoRoot
    try {
        & cargo check -p copythat-snapshot --features vss-com --tests
        if ($LASTEXITCODE -ne 0) { Write-Error "cargo check failed" }
    } finally {
        Pop-Location
    }
}

# Run the ignored COM round-trip test
Step "cargo test --features vss-com vss_com_create_release_round_trip --ignored"
Push-Location $repoRoot
try {
    $env:COPYTHAT_VSS_TEST_VOLUME = $Volume
    & cargo test -p copythat-snapshot --features vss-com `
        vss_com_create_release_round_trip -- --ignored --nocapture
    $rc = $LASTEXITCODE
    Remove-Item Env:COPYTHAT_VSS_TEST_VOLUME -ErrorAction SilentlyContinue
    if ($rc -ne 0) { Write-Error "VSS COM round-trip test failed (rc=$rc)" }
} finally {
    Pop-Location
}
Note "round-trip test: passed"

# Optional locked-file copy-via-shadow probe.
#
# End-to-end demonstration of the COM port + GLOBALROOT-path read +
# write-to-destination. While `lock_file.ps1` holds an exclusive lock
# on `LockedFilePath`, the ignored test `vss_com_copy_locked_file_via_shadow`:
#  1. confirms direct read fails with ERROR_SHARING_VIOLATION (32),
#  2. mints a shadow and reads the file off
#     `\\?\GLOBALROOT\Device\HarddiskVolumeShadowCopyN\<rest>`,
#  3. writes the bytes into `tests\vss-test-restored\` (a sibling
#     folder under the repo's `tests/` root),
#  4. verifies the bytes match the deterministic pattern
#     `lock_file.ps1` wrote (0..255 repeating).
if ($LockedFilePath) {
    Step "locked-file copy-via-shadow probe: $LockedFilePath"
    if (-not (Test-Path $LockedFilePath)) {
        Write-Error "LockedFilePath does not exist: $LockedFilePath"
    }

    # Surface lock state to the user before kicking off cargo. The
    # cargo test asserts the same condition, but seeing it print here
    # makes a misconfigured run obvious without sifting through Rust
    # output.
    Note "verifying $LockedFilePath is currently locked..."
    try {
        $bytes = [System.IO.File]::ReadAllBytes($LockedFilePath)
        $len = $bytes.Length
        $warnMsg = "expected sharing violation but read succeeded $len bytes. Is lock_file.ps1 running?"
        Write-Warning $warnMsg
    } catch [System.IO.IOException] {
        $msg = $_.Exception.Message
        Note "sharing violation confirmed: $msg"
    }

    $destDir = Join-Path $repoRoot 'tests\vss-test-restored'
    Note "destination dir: $destDir"
    Step "cargo test --features vss-com vss_com_copy_locked_file_via_shadow --ignored"
    Push-Location $repoRoot
    try {
        $env:COPYTHAT_VSS_LOCKED_FILE_PATH = $LockedFilePath
        $env:COPYTHAT_VSS_DEST_DIR = $destDir
        & cargo test -p copythat-snapshot --features vss-com `
            vss_com_copy_locked_file_via_shadow -- --ignored --nocapture
        $rc = $LASTEXITCODE
        Remove-Item Env:COPYTHAT_VSS_LOCKED_FILE_PATH -ErrorAction SilentlyContinue
        Remove-Item Env:COPYTHAT_VSS_DEST_DIR -ErrorAction SilentlyContinue
        if ($rc -ne 0) { Write-Error "locked-file copy-via-shadow probe failed (rc=$rc)" }
    } finally {
        Pop-Location
    }

    # Echo the artefact path so the user can inspect / hash / diff.
    $copyPath = Join-Path $destDir (Split-Path -Leaf $LockedFilePath)
    if (Test-Path $copyPath) {
        $info = Get-Item $copyPath
        $sizeMsg = "copied artefact: $($info.FullName) ($($info.Length) bytes)"
        Note $sizeMsg
    } else {
        Write-Warning "expected copy at $copyPath but it is not present"
    }
}

Step "done"
