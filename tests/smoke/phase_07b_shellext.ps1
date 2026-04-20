<#
.SYNOPSIS
    Phase 7b smoke test -- Windows COM DLL layer.

.DESCRIPTION
    Verifies the compiled copythat-shellext.dll is a valid COM
    in-proc server without actually registering it against a user
    profile. The test:

    1. Builds the DLL in release mode via `cargo build -p copythat-shellext --release`.
    2. Resolves the produced DLL path.
    3. LoadLibraries the DLL via Win32 P/Invoke.
    4. Calls GetProcAddress for the four COM entry points
       regsvr32 + the COM runtime require:
           - DllGetClassObject
           - DllCanUnloadNow
           - DllRegisterServer
           - DllUnregisterServer
    5. FreeLibrary the DLL.
    6. Reports PASS / FAIL and exits with a matching code.

    No registry writes. No process spawns. No Explorer integration.
    The live end-to-end smoke (actually register the DLL, invoke
    "Copy with Copy That" on a temp file via [Shell.Application],
    observe the running app enqueue the job) stays manual for 0.x
    because registering a COM DLL is an invasive operation and the
    CI Windows runner doesn't have a user-session Explorer to
    drive. When Phase 16 packaging wires up the MSI installer, the
    install-time hook exercises the live path.

.NOTES
    Exit 0 on success; non-zero on any failure.
    Run from the repo root:
        pwsh tests/smoke/phase_07b_shellext.ps1
#>

$ErrorActionPreference = 'Stop'

# Resolve the repo root as the parent of tests/.
$RepoRoot = Resolve-Path (Join-Path $PSScriptRoot '..\..')
Set-Location $RepoRoot

Write-Host "==> Building copythat-shellext (release)..."
& cargo build -p copythat-shellext --release
if ($LASTEXITCODE -ne 0) {
    Write-Error "cargo build failed with exit code $LASTEXITCODE"
    exit 1
}

$DllPath = Join-Path $RepoRoot 'target\release\copythat_shellext.dll'
if (-not (Test-Path $DllPath)) {
    Write-Error "DLL not found at $DllPath"
    exit 2
}
$dllInfo = Get-Item $DllPath
Write-Host ("==> DLL built: {0} ({1:N0} bytes)" -f $dllInfo.FullName, $dllInfo.Length)

# --- Resolve the four exports via P/Invoke ------------------------------
#
# Win32: LoadLibraryW + GetProcAddress + FreeLibrary.
# We don't call the exports (that would trigger registration / COM
# activation); we just prove they resolve, which is what `regsvr32`
# will need at install time.

$SourceCode = @"
using System;
using System.Runtime.InteropServices;

public static class Loader {
    [DllImport("kernel32.dll", SetLastError = true, CharSet = CharSet.Unicode)]
    public static extern IntPtr LoadLibraryW(string lpFileName);

    [DllImport("kernel32.dll", SetLastError = true)]
    public static extern IntPtr GetProcAddress(IntPtr hModule, [MarshalAs(UnmanagedType.LPStr)] string lpProcName);

    [DllImport("kernel32.dll", SetLastError = true)]
    [return: MarshalAs(UnmanagedType.Bool)]
    public static extern bool FreeLibrary(IntPtr hModule);
}
"@

Add-Type -TypeDefinition $SourceCode -Language CSharp | Out-Null

$handle = [Loader]::LoadLibraryW($DllPath)
if ($handle -eq [IntPtr]::Zero) {
    $err = [Runtime.InteropServices.Marshal]::GetLastWin32Error()
    Write-Error ("LoadLibraryW failed (Win32 error {0})" -f $err)
    exit 3
}
Write-Host ("==> LoadLibraryW ok (HMODULE 0x{0:X})" -f $handle.ToInt64())

$Exports = @(
    'DllGetClassObject',
    'DllCanUnloadNow',
    'DllRegisterServer',
    'DllUnregisterServer'
)

$failed = $false
foreach ($export in $Exports) {
    $addr = [Loader]::GetProcAddress($handle, $export)
    if ($addr -eq [IntPtr]::Zero) {
        Write-Host ("    MISS  {0}" -f $export) -ForegroundColor Red
        $failed = $true
    } else {
        Write-Host ("    OK    {0}  (0x{1:X})" -f $export, $addr.ToInt64())
    }
}

[void][Loader]::FreeLibrary($handle)

if ($failed) {
    Write-Error '==> FAIL -- one or more COM exports missing.'
    exit 4
}

Write-Host '==> PASS -- DLL exposes all four COM entry points.'
exit 0
