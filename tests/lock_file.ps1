# Phase 17 follow-up - locked-file fixture for VSS testing.
#
# Creates `tests\vss-test-locked.bin` (or the path you pass in
# via -Path), opens it with FileShare.None, and holds the handle
# until you press Ctrl+C. While the script is running, any other
# process reading that file gets ERROR_SHARING_VIOLATION (0x20)
# - the exact condition the engine's snapshot fallback is meant
# to recover from.
#
# Usage:
#   PowerShell -ExecutionPolicy Bypass -File tests\lock_file.ps1
#   # or with a custom path:
#   PowerShell -ExecutionPolicy Bypass -File tests\lock_file.ps1 -Path C:\path\to\file.bin
#
# Run this in one shell, leave it open, then in a second shell
# run the smoke harness (`tests\vss_com_smoke.ps1`) or trigger
# CopyThat's snapshot fallback against the locked file directly.

[CmdletBinding()]
param(
    [string]$Path = '',
    [int]$SizeBytes = 4096
)

$ErrorActionPreference = 'Stop'

# `$PSScriptRoot` is empty when PowerShell evaluates the param-
# block default in some 5.1 invocation paths (running via
# `powershell -File ...`). Compute the script-relative default in
# the body where every variable is reliably populated.
if ([string]::IsNullOrEmpty($Path)) {
    $scriptDir = if ($PSScriptRoot) {
        $PSScriptRoot
    } elseif ($MyInvocation.MyCommand.Path) {
        Split-Path -Parent $MyInvocation.MyCommand.Path
    } else {
        (Get-Location).Path
    }
    $Path = Join-Path $scriptDir 'vss-test-locked.bin'
}

# Seed the file with deterministic content (a repeating 0..255 pattern)
# so the smoke harness can verify byte-exact reads off the shadow.
$pattern = [byte[]]::new($SizeBytes)
for ($i = 0; $i -lt $SizeBytes; $i++) { $pattern[$i] = ($i % 256) }
[System.IO.File]::WriteAllBytes($Path, $pattern)
Write-Host "[lock_file] wrote $SizeBytes bytes to $Path"

# Open with FileShare.None - any other process attempting to
# Read or Write fails with ERROR_SHARING_VIOLATION until this
# script releases the handle (Ctrl+C).
$fs = [System.IO.File]::Open($Path, 'Open', 'ReadWrite', 'None')
Write-Host "[lock_file] holding exclusive ReadWrite/None lock on $Path"
Write-Host "[lock_file] in another shell:"
Write-Host "[lock_file]   Get-Content '$Path'   # expect: sharing violation"
Write-Host "[lock_file]   pwsh tests\vss_com_smoke.ps1 -LockedFilePath '$Path'"
Write-Host "[lock_file] press Ctrl+C to release the lock + exit"

try {
    while ($true) {
        Start-Sleep -Seconds 60
    }
} finally {
    $fs.Close()
    $fs.Dispose()
    Write-Host "[lock_file] released lock; file remains at $Path"
}
