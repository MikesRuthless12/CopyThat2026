# Merge a CopyThat-only bench run back into the full-run JSON.
#
# Use case: the user reran only CopyThat (`-OnlyTools @('CopyThat')`)
# to refresh its numbers without burning 24 minutes on competitors.
# This script:
#   1. Loads the saved full run from `target/bench-phase42.full.json`
#      (the previous full bench's data, with all 5 tools).
#   2. Loads the partial run from `target/bench-phase42.json` (the
#      most recent CopyThat-only run).
#   3. For each workload in the full run, replaces the CopyThat
#      result with the fresh one. Competitor results stay intact.
#   4. Writes the merged JSON back to `target/bench-phase42.json` so
#      the renderer picks it up.
#
# Run: pwsh -File scripts/bench-merge.ps1

$ErrorActionPreference = 'Stop'
$RepoRoot   = (Resolve-Path "$PSScriptRoot/..").Path
$FullJson   = Join-Path $RepoRoot 'target\bench-phase42.full.json'
$FreshJson  = Join-Path $RepoRoot 'target\bench-phase42.json'

if (-not (Test-Path $FullJson))  { throw "no full JSON at $FullJson" }
if (-not (Test-Path $FreshJson)) { throw "no fresh JSON at $FreshJson" }

$full  = Get-Content $FullJson  -Raw | ConvertFrom-Json
$fresh = Get-Content $FreshJson -Raw | ConvertFrom-Json

Write-Host "full has  $($full.workloads.Count) workloads"
Write-Host "fresh has $($fresh.workloads.Count) workloads"

# Index fresh by label for O(1) lookup
$freshByLabel = @{}
foreach ($w in $fresh.workloads) { $freshByLabel[$w.label] = $w }

foreach ($wlFull in $full.workloads) {
    $label = $wlFull.label
    $wlFresh = $freshByLabel[$label]
    if (-not $wlFresh) {
        Write-Host "  $label : no fresh data, leaving full as-is"
        continue
    }
    # Replace CopyThat result for this workload
    $tools = @($wlFresh.results.PSObject.Properties.Name)
    foreach ($tool in $tools) {
        Write-Host "  $label : updating $tool from fresh run"
        # Add or replace property on $wlFull.results
        if ($wlFull.results.PSObject.Properties[$tool]) {
            $wlFull.results.PSObject.Properties.Remove($tool) | Out-Null
        }
        $wlFull.results | Add-Member -NotePropertyName $tool -NotePropertyValue $wlFresh.results.$tool -Force
    }
}

# Note: keep the original `host`, `os`, `hardware`, `duration_ms` from
# the FULL run. The merge represents "competitor data from full run +
# CopyThat data from fresh run". duration_ms reflects the longer of
# the two; a re-render simply summarizes the merge.
$full | ConvertTo-Json -Depth 12 | Set-Content -Path $FreshJson -Encoding UTF8
Write-Host ""
Write-Host "merged -> $FreshJson"
