$ErrorActionPreference = 'Stop'
$packageName = 'copythat'
$softwareName = 'Copy That v1.25.0*'
$args = @{
    PackageName    = $packageName
    FileType       = 'msi'
    SilentArgs     = '/qn /norestart'
    ValidExitCodes = @(0, 1605, 1614, 1641, 3010)
}
$key = Get-UninstallRegistryKey -SoftwareName $softwareName
if ($key.Count -eq 1) {
    $args.File = $key[0].UninstallString
    Uninstall-ChocolateyPackage @args
} elseif ($key.Count -eq 0) {
    Write-Warning "$packageName is not installed."
} else {
    Write-Warning "Multiple matches for $softwareName; skipping automatic uninstall."
}
