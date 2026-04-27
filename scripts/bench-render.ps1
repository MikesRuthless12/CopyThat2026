# Phase 42 bench renderer -- reads target/bench-phase42.json (produced
# by bench-phase42.ps1) and emits:
#   - docs/BENCHMARKS_PHASE_42.md  (table + ASCII bars + % faster matrix)
#   - target/bench-phase42.html    (SVG bar charts, opens in any browser)
#
# Re-runnable independently: if you re-render after editing the JSON
# or change the renderer, the bench data is not re-collected.
#
# Run: pwsh -File scripts/bench-render.ps1

$ErrorActionPreference = 'Stop'
$RepoRoot   = (Resolve-Path "$PSScriptRoot/..").Path
$ResultJson = Join-Path $RepoRoot "target\bench-phase42.json"
$ResultMd   = Join-Path $RepoRoot "docs\BENCHMARKS_PHASE_42.md"
$ResultHtml = Join-Path $RepoRoot "target\bench-phase42.html"

if (-not (Test-Path $ResultJson)) {
    throw "no JSON found at $ResultJson -- run bench-phase42.ps1 first"
}

$report = Get-Content $ResultJson -Raw | ConvertFrom-Json

# ----- Helpers ----------------------------------------------------------

function Bar-Ascii {
    param([double]$value, [double]$max, [int]$width = 40)
    $filled = if ($max -le 0) { 0 } else { [int]([Math]::Round($value / $max * $width)) }
    $empty = $width - $filled
    return ('█' * $filled) + ('░' * $empty)
}

function Pct-Faster {
    # "tool A is N% faster than B" means A throughput is (1 + N/100) * B.
    # If A:1000 MiB/s, B:500 MiB/s -> A is 100% faster (2x).
    param([double]$a, [double]$b)
    if ($b -le 0) { return 0 }
    return [Math]::Round(($a - $b) / $b * 100, 1)
}

function Workload-MiB {
    param($wl)
    if ($wl.bytes) { return [Math]::Round($wl.bytes / 1MB, 0) }
    # mixed tree: derive total from any tool's reported bytes
    $any = $wl.results.PSObject.Properties.Value | Select-Object -First 1
    if ($any -and $any.cold_bytes) {
        return [Math]::Round($any.cold_bytes / 1MB, 0)
    }
    return 0
}

# ----- Markdown report --------------------------------------------------

$md = @()
$md += "# Phase 42 head-to-head benchmark"
$md += ""
$md += "Generated $(Get-Date -Format 'yyyy-MM-dd HH:mm') from `target/bench-phase42.json`."
$md += "Run via `pwsh -File scripts/bench-phase42.ps1` then re-render via `pwsh -File scripts/bench-render.ps1`."
$md += ""
$md += "**Host**: $($report.host) ($($report.os))"
$md += "**Total bench duration**: $([Math]::Round($report.duration_ms / 1000.0, 1)) s"
$md += ""
$md += "## Methodology"
$md += ""
$md += "- **COLD**: a fresh source file is created on disk before *every* iteration. Source bytes have just been written and may or may not be in the OS page cache; this models real-world copy-of-a-just-saved-file."
$md += "- **WARM**: the source file is created once + read end-to-end to warm the page cache, then the same file is copied N times. Models repeated copies of an already-cached source."
$md += "- For each tool × workload × cache-state, we report the **median wall-clock** across N iterations."
$md += "- Same-volume copy (C: -> C:) on the test host. Cross-volume copies are disk-bound and tend to tie across all tools - the engine speed only matters when the disk isn't the bottleneck."
$md += ""
$md += "## Tools"
$md += ""
$md += "| Tool | Invocation |"
$md += "|---|---|"
$md += "| **CopyThat** | ``copythat copy <src> <dst>`` (release binary) |"
$md += "| **RoboCopy** | ``robocopy <srcDir> <dstDir> <name> /NFL /NDL /NJH /NJS /NP /R:0 /W:0`` |"
$md += "| **cmd copy** | ``cmd /C copy /Y <src> <dst>`` |"
$md += "| **TeraCopy** | ``TeraCopy.exe Copy <src> <dstDir>\\ /Close /SkipAll`` |"
$md += "| **FastCopy** | ``FastCopy.exe /cmd=force_copy /no_ui /auto_close /to=<dstDir>\\ <src>`` |"
$md += ""

foreach ($wl in $report.workloads) {
    $md += "---"
    $md += ""
    $md += "## Workload: $($wl.label)"
    $md += ""
    if ($wl.bytes) {
        $md += "Single file, $([Math]::Round($wl.bytes / 1MB, 0)) MB. **$($wl.iterations) iterations** per cell, median reported."
    } else {
        $md += "100 files, 100 KB - 150 MB log-distributed (~$(Workload-MiB $wl) MB total). **$($wl.iterations) iterations** per cell, median reported."
    }
    $md += ""

    # Build rows
    $tools = @($wl.results.PSObject.Properties.Name)
    $rows = @()
    foreach ($t in $tools) {
        $r = $wl.results.$t
        $rows += [pscustomobject]@{
            Tool = $t
            ColdMs = $r.cold_median_ms
            ColdMib = $r.cold_throughput_MiBps
            WarmMs = $r.warm_median_ms
            WarmMib = $r.warm_throughput_MiBps
            ColdAll = ($r.cold_times_ms -join ', ')
            WarmAll = ($r.warm_times_ms -join ', ')
        }
    }

    # Find max throughput per cache state for ranking + bars
    $maxColdMib = ($rows | Measure-Object -Property ColdMib -Maximum).Maximum
    $maxWarmMib = ($rows | Measure-Object -Property WarmMib -Maximum).Maximum

    # Sort by COLD throughput desc for the headline table
    $sorted = $rows | Sort-Object -Property ColdMib -Descending

    $md += "### Throughput summary"
    $md += ""
    $md += "| Rank | Tool | COLD median (ms) | COLD MiB/s | WARM median (ms) | WARM MiB/s |"
    $md += "|---:|---|---:|---:|---:|---:|"
    $rank = 1
    foreach ($r in $sorted) {
        $tool = $r.Tool
        $marker = if ($tool -eq 'CopyThat') { "**$tool**" } else { $tool }
        $crown = if ($rank -eq 1) { "🥇 " } elseif ($rank -eq 2) { "🥈 " } elseif ($rank -eq 3) { "🥉 " } else { "" }
        $md += "| $crown$rank | $marker | $($r.ColdMs) | **$($r.ColdMib)** | $($r.WarmMs) | **$($r.WarmMib)** |"
        $rank++
    }
    $md += ""

    # ASCII bar chart -- COLD
    $md += "### Visual -- COLD cache"
    $md += ""
    $md += '```'
    foreach ($r in $sorted) {
        $bar = Bar-Ascii $r.ColdMib $maxColdMib 40
        $md += ('  {0,-10} {1}  {2,8:N1} MiB/s' -f $r.Tool, $bar, $r.ColdMib)
    }
    $md += '```'
    $md += ""

    # ASCII bar chart -- WARM
    $sortedWarm = $rows | Sort-Object -Property WarmMib -Descending
    $md += "### Visual -- WARM cache"
    $md += ""
    $md += '```'
    foreach ($r in $sortedWarm) {
        $bar = Bar-Ascii $r.WarmMib $maxWarmMib 40
        $md += ('  {0,-10} {1}  {2,8:N1} MiB/s' -f $r.Tool, $bar, $r.WarmMib)
    }
    $md += '```'
    $md += ""

    # Percentages -- CopyThat vs each competitor
    $self = $rows | Where-Object { $_.Tool -eq 'CopyThat' }
    if ($self) {
        $md += "### CopyThat vs competitors"
        $md += ""
        $md += "| Competitor | COLD: CopyThat is | WARM: CopyThat is |"
        $md += "|---|:---:|:---:|"
        foreach ($r in ($rows | Where-Object { $_.Tool -ne 'CopyThat' })) {
            $pcCold = Pct-Faster $self.ColdMib $r.ColdMib
            $pcWarm = Pct-Faster $self.WarmMib $r.WarmMib
            $coldStr = if ($pcCold -gt 0) { "**+$pcCold% faster**" } elseif ($pcCold -lt 0) { "$pcCold% slower" } else { "tied" }
            $warmStr = if ($pcWarm -gt 0) { "**+$pcWarm% faster**" } elseif ($pcWarm -lt 0) { "$pcWarm% slower" } else { "tied" }
            $md += "| $($r.Tool) | $coldStr | $warmStr |"
        }
        $md += ""
    }

    # Full pairwise % matrix (cold)
    $md += "### Full pairwise matrix (COLD MiB/s; row vs column = how much faster row is)"
    $md += ""
    $headers = @($rows.Tool)
    $hdr = "| | " + ($headers -join " | ") + " |"
    $sep = "|---|" + ((@("---") * $headers.Count) -join "|") + "|"
    $md += $hdr
    $md += $sep
    foreach ($r in $rows) {
        $cells = @($r.Tool)
        foreach ($c in $rows) {
            if ($r.Tool -eq $c.Tool) {
                $cells += "--"
            } else {
                $p = Pct-Faster $r.ColdMib $c.ColdMib
                if ($p -gt 0) { $cells += "+$p%" }
                elseif ($p -lt 0) { $cells += "$p%" }
                else { $cells += "0%" }
            }
        }
        $md += "| **" + ($cells[0]) + "** | " + (($cells[1..($cells.Count-1)]) -join " | ") + " |"
    }
    $md += ""

    # Raw iteration data
    $md += "<details><summary>Raw iteration timings (ms)</summary>"
    $md += ""
    $md += "| Tool | COLD all iters | WARM all iters |"
    $md += "|---|---|---|"
    foreach ($r in $rows) {
        $md += "| $($r.Tool) | $($r.ColdAll) | $($r.WarmAll) |"
    }
    $md += ""
    $md += "</details>"
    $md += ""
}

# ----- Cross-workload summary -----
$md += "---"
$md += ""
$md += "## Cross-workload summary -- CopyThat throughput vs the field"
$md += ""
$md += "| Workload | CopyThat COLD MiB/s | Field-best COLD MiB/s | Δ vs best | CopyThat WARM MiB/s | Field-best WARM MiB/s | Δ vs best |"
$md += "|---|---:|---:|---:|---:|---:|---:|"
foreach ($wl in $report.workloads) {
    $rows = @()
    foreach ($t in $wl.results.PSObject.Properties.Name) {
        $r = $wl.results.$t
        $rows += [pscustomobject]@{
            Tool = $t
            ColdMib = $r.cold_throughput_MiBps
            WarmMib = $r.warm_throughput_MiBps
        }
    }
    $self = $rows | Where-Object { $_.Tool -eq 'CopyThat' }
    $bestColdRow = $rows | Sort-Object -Property ColdMib -Descending | Select-Object -First 1
    $bestWarmRow = $rows | Sort-Object -Property WarmMib -Descending | Select-Object -First 1
    if (-not $self) { continue }

    $coldDelta = if ($self.Tool -eq $bestColdRow.Tool) { "🥇 best" } else {
        $p = Pct-Faster $self.ColdMib $bestColdRow.ColdMib
        if ($p -lt 0) { "$p% (best: $($bestColdRow.Tool))" } else { "tied" }
    }
    $warmDelta = if ($self.Tool -eq $bestWarmRow.Tool) { "🥇 best" } else {
        $p = Pct-Faster $self.WarmMib $bestWarmRow.WarmMib
        if ($p -lt 0) { "$p% (best: $($bestWarmRow.Tool))" } else { "tied" }
    }
    $md += "| $($wl.label) | **$($self.ColdMib)** | $($bestColdRow.ColdMib) ($($bestColdRow.Tool)) | $coldDelta | **$($self.WarmMib)** | $($bestWarmRow.WarmMib) ($($bestWarmRow.Tool)) | $warmDelta |"
}
$md += ""

New-Item -ItemType Directory -Path (Split-Path $ResultMd -Parent) -Force | Out-Null
$md -join [Environment]::NewLine | Set-Content -Path $ResultMd -Encoding UTF8
Write-Host "wrote markdown -> $ResultMd"

# ----- HTML report ------------------------------------------------------

$colors = @{
    CopyThat = '#2563eb'  # blue -- our app, headline
    RoboCopy = '#10b981'  # green
    CmdCopy  = '#f59e0b'  # amber
    TeraCopy = '#ef4444'  # red
    FastCopy = '#8b5cf6'  # purple
}

function Html-BarChart {
    param($wl, [string]$mode)  # mode: 'cold' or 'warm'
    $rows = @()
    foreach ($t in $wl.results.PSObject.Properties.Name) {
        $r = $wl.results.$t
        $mib = if ($mode -eq 'cold') { $r.cold_throughput_MiBps } else { $r.warm_throughput_MiBps }
        $rows += [pscustomobject]@{ Tool = $t; Mib = $mib }
    }
    $rows = $rows | Sort-Object -Property Mib -Descending
    $maxMib = ($rows | Measure-Object -Property Mib -Maximum).Maximum
    if ($maxMib -le 0) { $maxMib = 1 }

    $html = @()
    $html += '<div class="chart">'
    foreach ($r in $rows) {
        $pct = [Math]::Round($r.Mib / $maxMib * 100, 1)
        $color = $colors[$r.Tool]
        if (-not $color) { $color = '#888' }
        $highlight = if ($r.Tool -eq 'CopyThat') { ' chart-self' } else { '' }
        $html += "  <div class='bar-row$highlight'>"
        $html += "    <div class='bar-label'>$($r.Tool)</div>"
        $html += "    <div class='bar-track'><div class='bar-fill' style='width:$pct%; background:$color'></div></div>"
        $html += ("    <div class='bar-value'>{0:N1} MiB/s</div>" -f $r.Mib)
        $html += '  </div>'
    }
    $html += '</div>'
    return ($html -join [Environment]::NewLine)
}

$html = @()
$html += '<!DOCTYPE html>'
$html += '<html lang="en"><head><meta charset="utf-8">'
$html += '<title>CopyThat -- Phase 42 head-to-head benchmark</title>'
$html += '<style>'
$html += '  body { font-family: -apple-system, Segoe UI, Roboto, sans-serif; background: #0f172a; color: #e2e8f0; max-width: 1100px; margin: 0 auto; padding: 32px; }'
$html += '  h1 { color: #60a5fa; }'
$html += '  h2 { color: #93c5fd; margin-top: 36px; border-bottom: 1px solid #334155; padding-bottom: 6px; }'
$html += '  h3 { color: #cbd5e1; margin-top: 24px; }'
$html += '  .meta { color: #94a3b8; font-size: 14px; }'
$html += '  .chart { margin: 16px 0; }'
$html += '  .bar-row { display: flex; align-items: center; gap: 12px; margin: 6px 0; }'
$html += '  .bar-row.chart-self .bar-label { color: #60a5fa; font-weight: bold; }'
$html += '  .bar-label { width: 110px; font-family: monospace; }'
$html += '  .bar-track { flex: 1; background: #1e293b; height: 22px; border-radius: 4px; overflow: hidden; }'
$html += '  .bar-fill { height: 100%; transition: width 0.5s ease; }'
$html += '  .bar-value { width: 130px; text-align: right; font-family: monospace; color: #f8fafc; }'
$html += '  .pair-table, .summary-table { border-collapse: collapse; width: 100%; margin: 12px 0; font-size: 14px; }'
$html += '  .pair-table th, .pair-table td, .summary-table th, .summary-table td { border: 1px solid #334155; padding: 6px 10px; text-align: right; }'
$html += '  .pair-table th, .summary-table th { background: #1e293b; }'
$html += '  .pair-table td:first-child, .summary-table td:first-child { text-align: left; font-weight: bold; color: #93c5fd; }'
$html += '  .pos { color: #4ade80; }'
$html += '  .neg { color: #f87171; }'
$html += '  .self-row td:first-child { color: #60a5fa; }'
$html += '  .grid { display: grid; grid-template-columns: 1fr 1fr; gap: 24px; }'
$html += '  @media (max-width: 800px) { .grid { grid-template-columns: 1fr; } }'
$html += '</style></head><body>'
$html += '<h1>CopyThat -- Phase 42 head-to-head benchmark</h1>'
$html += "<div class='meta'>Host: $($report.host) ($($report.os)) -- Generated $(Get-Date -Format 'yyyy-MM-dd HH:mm') -- Total bench duration $([Math]::Round($report.duration_ms / 1000.0, 1)) s</div>"
$html += '<p>COLD = fresh source file before every iteration. WARM = same source file across iterations (page-cache hit after #1). Median across N iterations.</p>'

foreach ($wl in $report.workloads) {
    $html += "<h2>Workload: $($wl.label)</h2>"
    if ($wl.bytes) {
        $html += "<div class='meta'>Single file, $([Math]::Round($wl.bytes / 1MB, 0)) MB · $($wl.iterations) iterations per cell · median reported</div>"
    } else {
        $html += "<div class='meta'>100 files, 100 KB - 150 MB log-distributed (~$(Workload-MiB $wl) MB total) · $($wl.iterations) iterations per cell · median reported</div>"
    }

    $html += "<div class='grid'>"
    $html += "  <div><h3>COLD cache</h3>"
    $html += (Html-BarChart $wl 'cold')
    $html += "  </div>"
    $html += "  <div><h3>WARM cache</h3>"
    $html += (Html-BarChart $wl 'warm')
    $html += "  </div>"
    $html += "</div>"

    # Pairwise % matrix
    $rows = @()
    foreach ($t in $wl.results.PSObject.Properties.Name) {
        $r = $wl.results.$t
        $rows += [pscustomobject]@{ Tool = $t; ColdMib = $r.cold_throughput_MiBps; WarmMib = $r.warm_throughput_MiBps }
    }
    $html += "<h3>Pairwise &mdash; row is N% faster than column (COLD MiB/s)</h3>"
    $html += "<table class='pair-table'><thead><tr><th></th>"
    foreach ($c in $rows) { $html += "<th>$($c.Tool)</th>" }
    $html += "</tr></thead><tbody>"
    foreach ($r in $rows) {
        $cls = if ($r.Tool -eq 'CopyThat') { ' class="self-row"' } else { '' }
        $html += "<tr$cls><td>$($r.Tool)</td>"
        foreach ($c in $rows) {
            if ($r.Tool -eq $c.Tool) { $html += "<td>--</td>"; continue }
            $p = Pct-Faster $r.ColdMib $c.ColdMib
            $cell = if ($p -gt 0) { "<span class='pos'>+$p%</span>" } elseif ($p -lt 0) { "<span class='neg'>$p%</span>" } else { "0%" }
            $html += "<td>$cell</td>"
        }
        $html += "</tr>"
    }
    $html += "</tbody></table>"
}

# Cross-workload summary
$html += "<h2>Cross-workload summary &mdash; CopyThat vs field-best</h2>"
$html += "<table class='summary-table'><thead><tr><th>Workload</th><th>CopyThat COLD</th><th>Field-best COLD</th><th>&Delta;</th><th>CopyThat WARM</th><th>Field-best WARM</th><th>&Delta;</th></tr></thead><tbody>"
foreach ($wl in $report.workloads) {
    $rows = @()
    foreach ($t in $wl.results.PSObject.Properties.Name) {
        $r = $wl.results.$t
        $rows += [pscustomobject]@{ Tool = $t; ColdMib = $r.cold_throughput_MiBps; WarmMib = $r.warm_throughput_MiBps }
    }
    $self = $rows | Where-Object { $_.Tool -eq 'CopyThat' }
    if (-not $self) { continue }
    $bestC = $rows | Sort-Object -Property ColdMib -Descending | Select-Object -First 1
    $bestW = $rows | Sort-Object -Property WarmMib -Descending | Select-Object -First 1
    $coldDelta = if ($self.Tool -eq $bestC.Tool) { "<span class='pos'>🥇 best</span>" } else {
        $p = Pct-Faster $self.ColdMib $bestC.ColdMib
        "<span class='neg'>$p% (best: $($bestC.Tool))</span>"
    }
    $warmDelta = if ($self.Tool -eq $bestW.Tool) { "<span class='pos'>🥇 best</span>" } else {
        $p = Pct-Faster $self.WarmMib $bestW.WarmMib
        "<span class='neg'>$p% (best: $($bestW.Tool))</span>"
    }
    $html += "<tr><td>$($wl.label)</td><td>$($self.ColdMib) MiB/s</td><td>$($bestC.ColdMib) ($($bestC.Tool))</td><td>$coldDelta</td><td>$($self.WarmMib) MiB/s</td><td>$($bestW.WarmMib) ($($bestW.Tool))</td><td>$warmDelta</td></tr>"
}
$html += "</tbody></table>"

$html += "</body></html>"
New-Item -ItemType Directory -Path (Split-Path $ResultHtml -Parent) -Force | Out-Null
$html -join [Environment]::NewLine | Set-Content -Path $ResultHtml -Encoding UTF8
Write-Host "wrote html -> $ResultHtml"
Write-Host ""
Write-Host "Open the HTML in a browser:"
Write-Host "  start $ResultHtml"
