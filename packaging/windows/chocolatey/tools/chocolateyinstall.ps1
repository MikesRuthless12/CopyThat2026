# Chocolatey install script. Downloads the tagged MSI from GitHub
# Releases and runs it silently. `checksum64` below is a placeholder;
# the publish helper (run at tag time) rewrites it with the real
# SHA-256 of the MSI it just built.
$ErrorActionPreference = 'Stop'
$packageName = 'copythat'
$version$version     = '1.25.0'
$url64       = "https://github.com/MikesRuthless12/CopyThat2026/releases/download/v$version/CopyThat_${version}_x64_en-US.msi"
$checksum64  = '0000000000000000000000000000000000000000000000000000000000000000'

$args = @{
    PackageName    = $packageName
    FileType       = 'msi'
    Url64bit       = $url64
    Checksum64     = $checksum64
    ChecksumType64 = 'sha256'
    SilentArgs     = '/qn /norestart'
    ValidExitCodes = @(0, 3010, 1641)
}

Install-ChocolateyPackage @args
