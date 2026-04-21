Write-Host "--- Recent WER reports (any app, last 2 h) ---"
$locs = @(
  "$env:LocalAppData\CrashDumps",
  "$env:LocalAppData\Microsoft\Windows\WER\ReportArchive",
  "$env:LocalAppData\Microsoft\Windows\WER\ReportQueue"
)
$cutoff = (Get-Date).AddHours(-2)
foreach ($loc in $locs) {
  if (Test-Path $loc) {
    Write-Host ""
    Write-Host "## $loc"
    Get-ChildItem -Path $loc -Recurse -ErrorAction SilentlyContinue |
      Where-Object { $_.LastWriteTime -gt $cutoff } |
      Sort-Object LastWriteTime -Descending |
      Select-Object -First 12 Name, Length, LastWriteTime, FullName |
      Format-Table -AutoSize
  }
}
Write-Host ""
Write-Host "--- Recent Application Errors (any app, last 1 h) ---"
$cutoff2 = (Get-Date).AddHours(-1)
Get-WinEvent -LogName Application -MaxEvents 800 -ErrorAction SilentlyContinue |
  Where-Object { $_.TimeCreated -gt $cutoff2 -and $_.LevelDisplayName -eq 'Error' } |
  Select-Object -First 10 TimeCreated, ProviderName, Id, Message |
  Format-List
