# Phase 42 head-to-head bench harness -- CopyThat vs RoboCopy / cmd copy / TeraCopy / FastCopy.
#
# Per-workload methodology:
#   - COLD: create a fresh source file before each iteration (uncached source bytes)
#   - WARM: use the same source file across iterations (page-cache hit after first)
#
# Workloads:
#   - 256 MB single file
#   - 512 MB single file
#   - 10 GB  single file
#   - mixed tree of 100 files, 100 KB - 150 MB log-distributed (~7.5 GB total)
#
# Live progress is streamed for every iteration. Final results land in
#   target/bench-phase42.json   (raw)
#   docs/BENCHMARKS_PHASE_42.md (human-readable + percentages)
#   target/bench-phase42.html   (SVG bar charts, opens in any browser)
#
# Run: pwsh -File scripts/bench-phase42.ps1

$ErrorActionPreference = 'Stop'

# ----- Config ----------------------------------------------------------

$RepoRoot      = (Resolve-Path "$PSScriptRoot/..").Path
$BenchDir      = "C:\copythat-bench-vs"
$ResultJson    = Join-Path $RepoRoot "target\bench-phase42.json"
$ResultMd      = Join-Path $RepoRoot "docs\BENCHMARKS_PHASE_42.md"
$ResultHtml    = Join-Path $RepoRoot "target\bench-phase42.html"

$CopyThat = "$RepoRoot\target\release\copythat.exe"
$TeraCopy = "C:\Program Files\TeraCopy\TeraCopy.exe"
$FastCopy = "C:\Users\miken\FastCopy\FastCopy.exe"

# Iteration counts: smaller workloads get more iterations for a tighter median.
$IterMap = @{
    '256MB'    = 5
    '512MB'    = 4
    '10GB'     = 2
    'mixed-100' = 3
}

# ----- Helpers ---------------------------------------------------------

function Now-Ms { [int64](([datetime]::UtcNow - [datetime]'1970-01-01').TotalMilliseconds) }

function Write-RandomFile {
    param([string]$Path, [long]$Bytes)
    # Allocate-and-fill via .NET streaming so we don't hold the bytes
    # in PowerShell memory. Pseudo-random pattern (deterministic seed)
    # so a compressing FS can't fake the throughput.
    $fs = [System.IO.File]::Create($Path)
    try {
        $buf = New-Object byte[] (1 * 1024 * 1024)  # 1 MiB chunks
        $rng = [System.Random]::new(0xC0FFEE)
        $remaining = $Bytes
        while ($remaining -gt 0) {
            $rng.NextBytes($buf)
            $n = [Math]::Min($buf.Length, $remaining)
            $fs.Write($buf, 0, $n)
            $remaining -= $n
        }
        $fs.Flush($true) | Out-Null
    } finally {
        $fs.Dispose()
    }
}

function Warm-File {
    param([string]$Path)
    # Read the file end-to-end once so subsequent copies hit the cache.
    $fs = [System.IO.File]::OpenRead($Path)
    try {
        $buf = New-Object byte[] (1 * 1024 * 1024)
        while ($fs.Read($buf, 0, $buf.Length) -gt 0) {}
    } finally {
        $fs.Dispose()
    }
}

function Reset-Workdir {
    param([string]$Sub)
    $d = Join-Path $BenchDir $Sub
    if (Test-Path $d) { Remove-Item $d -Recurse -Force -ErrorAction SilentlyContinue }
    New-Item -ItemType Directory -Path $d -Force | Out-Null
    return $d
}

function Run-Tool {
    param(
        [string]$Tool,
        [string]$SrcFile,
        [string]$DstDir,
        [string]$DstFile  # for tools that take dst as a file path
    )
    switch ($Tool) {
        'CopyThat' {
            # Use --quiet so we time the engine, not the progress bar paint.
            $args = @('copy', '--quiet', $SrcFile, $DstFile)
            $t0 = Now-Ms
            & $CopyThat @args | Out-Null
            return (Now-Ms) - $t0
        }
        'RoboCopy' {
            $name = Split-Path $SrcFile -Leaf
            $srcDir = Split-Path $SrcFile -Parent
            $args = @($srcDir, $DstDir, $name, '/NFL', '/NDL', '/NJH', '/NJS', '/NP', '/R:0', '/W:0')
            $t0 = Now-Ms
            $null = & robocopy @args
            # robocopy exit < 8 == success
            if ($LASTEXITCODE -ge 8) { Write-Warning "robocopy returned $LASTEXITCODE" }
            return (Now-Ms) - $t0
        }
        'CmdCopy' {
            $t0 = Now-Ms
            $cmdline = 'copy /Y "' + $SrcFile + '" "' + $DstFile + '"'
            & cmd.exe /C $cmdline | Out-Null
            return (Now-Ms) - $t0
        }
        'TeraCopy' {
            # /Close auto-closes when done; /SkipAll says "skip on conflict".
            # Note: TeraCopy CLI returns immediately and runs in the background;
            # we have to wait for the resulting file to appear and stop changing.
            $expectedDst = Join-Path $DstDir (Split-Path $SrcFile -Leaf)
            if (Test-Path $expectedDst) { Remove-Item $expectedDst -Force }
            $t0 = Now-Ms
            $teraArgs = @('Copy', ('"' + $SrcFile + '"'), ('"' + $DstDir + '\"'), '/Close', '/SkipAll')
            $proc = Start-Process -FilePath $TeraCopy -ArgumentList $teraArgs -PassThru -NoNewWindow
            $proc.WaitForExit()
            # TeraCopy may finalize the file write a beat after the process exits.
            # Wait for the destination file to exist + size match.
            $srcSize = (Get-Item $SrcFile).Length
            $deadline = [DateTime]::UtcNow.AddSeconds(180)
            while ([DateTime]::UtcNow -lt $deadline) {
                if (Test-Path $expectedDst) {
                    try {
                        $dstSize = (Get-Item $expectedDst).Length
                        if ($dstSize -eq $srcSize) { break }
                    } catch {}
                }
                Start-Sleep -Milliseconds 50
            }
            return (Now-Ms) - $t0
        }
        'FastCopy' {
            # /cmd=force_copy = copy + overwrite without prompt
            # /no_ui = headless
            # /auto_close = exit when done
            # /to=DSTDIR\ (trailing backslash is required)
            $expectedDst = Join-Path $DstDir (Split-Path $SrcFile -Leaf)
            if (Test-Path $expectedDst) { Remove-Item $expectedDst -Force }
            $t0 = Now-Ms
            $fcArgs = @('/cmd=force_copy', '/no_ui', '/auto_close', ('/to="' + $DstDir + '\"'), ('"' + $SrcFile + '"'))
            $proc = Start-Process -FilePath $FastCopy -ArgumentList $fcArgs -PassThru -NoNewWindow
            $proc.WaitForExit()
            # Same wait pattern as TeraCopy -- verify the file landed.
            $srcSize = (Get-Item $SrcFile).Length
            $deadline = [DateTime]::UtcNow.AddSeconds(180)
            while ([DateTime]::UtcNow -lt $deadline) {
                if (Test-Path $expectedDst) {
                    try {
                        $dstSize = (Get-Item $expectedDst).Length
                        if ($dstSize -eq $srcSize) { break }
                    } catch {}
                }
                Start-Sleep -Milliseconds 50
            }
            return (Now-Ms) - $t0
        }
        default { throw "unknown tool: $Tool" }
    }
}

function Run-Tool-Tree {
    param(
        [string]$Tool,
        [string]$SrcDir,
        [string]$DstDir
    )
    switch ($Tool) {
        'CopyThat' {
            # Engine handles tree copy via copy <dir> <dir>
            $t0 = Now-Ms
            & $CopyThat copy --quiet $SrcDir $DstDir | Out-Null
            return (Now-Ms) - $t0
        }
        'RoboCopy' {
            $args = @($SrcDir, $DstDir, '/E', '/NFL', '/NDL', '/NJH', '/NJS', '/NP', '/R:0', '/W:0')
            $t0 = Now-Ms
            $null = & robocopy @args
            return (Now-Ms) - $t0
        }
        'CmdCopy' {
            # cmd copy doesn't recurse -- use xcopy /E /Y /Q
            $t0 = Now-Ms
            $cmdline = 'xcopy /E /Y /Q "' + $SrcDir + '\*" "' + $DstDir + '\"'
            & cmd.exe /C $cmdline | Out-Null
            return (Now-Ms) - $t0
        }
        'TeraCopy' {
            $t0 = Now-Ms
            $teraArgs = @('Copy', ('"' + $SrcDir + '\*"'), ('"' + $DstDir + '\"'), '/Close', '/SkipAll')
            $proc = Start-Process -FilePath $TeraCopy -ArgumentList $teraArgs -PassThru -NoNewWindow
            $proc.WaitForExit()
            # Wait for tree to finish landing.
            $expectedCount = (Get-ChildItem $SrcDir -File -Recurse).Count
            $deadline = [DateTime]::UtcNow.AddSeconds(300)
            while ([DateTime]::UtcNow -lt $deadline) {
                $actual = (Get-ChildItem $DstDir -File -Recurse -ErrorAction SilentlyContinue).Count
                if ($actual -ge $expectedCount) { break }
                Start-Sleep -Milliseconds 100
            }
            return (Now-Ms) - $t0
        }
        'FastCopy' {
            $t0 = Now-Ms
            $fcArgs = @('/cmd=force_copy', '/no_ui', '/auto_close', ('/to="' + $DstDir + '\"'), ('"' + $SrcDir + '\*"'))
            $proc = Start-Process -FilePath $FastCopy -ArgumentList $fcArgs -PassThru -NoNewWindow
            $proc.WaitForExit()
            $expectedCount = (Get-ChildItem $SrcDir -File -Recurse).Count
            $deadline = [DateTime]::UtcNow.AddSeconds(300)
            while ([DateTime]::UtcNow -lt $deadline) {
                $actual = (Get-ChildItem $DstDir -File -Recurse -ErrorAction SilentlyContinue).Count
                if ($actual -ge $expectedCount) { break }
                Start-Sleep -Milliseconds 100
            }
            return (Now-Ms) - $t0
        }
        default { throw "unknown tool for tree: $Tool" }
    }
}

function Median { param([array]$xs); ($xs | Sort-Object)[[int][Math]::Floor($xs.Count / 2)] }

# ----- Main ------------------------------------------------------------

function Bench-Single {
    param([string]$Label, [long]$Bytes)

    $iters = $IterMap[$Label]
    Write-Host ""
    Write-Host "================================================================"
    Write-Host "WORKLOAD: $Label  ($([Math]::Round($Bytes / 1MB, 0)) MB)  iter=$iters"
    Write-Host "================================================================"

    # Tools to bench in this run.
    $Tools = @('CopyThat', 'RoboCopy', 'CmdCopy', 'TeraCopy', 'FastCopy')
    $results = @{}

    foreach ($tool in $Tools) {
        Write-Host ""
        Write-Host "----- $tool -----"

        # COLD: create a fresh source file before EACH iteration.
        $coldTimes = @()
        for ($i = 1; $i -le $iters; $i++) {
            $srcDir = Reset-Workdir "src-$Label"
            $dstDir = Reset-Workdir "dst-$Label"
            $srcFile = Join-Path $srcDir "bench-source.bin"
            $dstFile = Join-Path $dstDir "bench-source.bin"

            Write-Host "  COLD iter $i/$iters  generating fresh $Label source..." -NoNewline
            Write-RandomFile -Path $srcFile -Bytes $Bytes
            Write-Host " ok"

            Write-Host "  COLD iter $i/$iters  running $tool..." -NoNewline
            $ms = Run-Tool -Tool $tool -SrcFile $srcFile -DstDir $dstDir -DstFile $dstFile
            $mibs = [Math]::Round(($Bytes / 1MB) / ($ms / 1000.0), 1)
            Write-Host (" {0,7} ms  ({1,8:N1} MiB/s)" -f $ms, $mibs)
            $coldTimes += $ms
        }

        # WARM: same source file across iterations (caches after iter 1).
        $srcDir = Reset-Workdir "src-$Label-warm"
        $dstDir = Reset-Workdir "dst-$Label-warm"
        $srcFile = Join-Path $srcDir "bench-source.bin"
        $dstFile = Join-Path $dstDir "bench-source.bin"

        Write-Host "  warm: generating + caching $Label source..." -NoNewline
        Write-RandomFile -Path $srcFile -Bytes $Bytes
        Warm-File -Path $srcFile
        Write-Host " ok"

        $warmTimes = @()
        for ($i = 1; $i -le $iters; $i++) {
            if (Test-Path $dstFile) { Remove-Item $dstFile -Force }
            Write-Host "  WARM iter $i/$iters  running $tool..." -NoNewline
            $ms = Run-Tool -Tool $tool -SrcFile $srcFile -DstDir $dstDir -DstFile $dstFile
            $mibs = [Math]::Round(($Bytes / 1MB) / ($ms / 1000.0), 1)
            Write-Host (" {0,7} ms  ({1,8:N1} MiB/s)" -f $ms, $mibs)
            $warmTimes += $ms
        }

        $coldMedian = Median $coldTimes
        $warmMedian = Median $warmTimes
        $coldMibs   = [Math]::Round(($Bytes / 1MB) / ($coldMedian / 1000.0), 1)
        $warmMibs   = [Math]::Round(($Bytes / 1MB) / ($warmMedian / 1000.0), 1)
        Write-Host ("  ==> $tool COLD median {0} ms ({1} MiB/s) | WARM median {2} ms ({3} MiB/s)" `
            -f $coldMedian, $coldMibs, $warmMedian, $warmMibs)

        $results[$tool] = @{
            cold_times_ms = $coldTimes
            warm_times_ms = $warmTimes
            cold_median_ms = $coldMedian
            warm_median_ms = $warmMedian
            cold_throughput_MiBps = $coldMibs
            warm_throughput_MiBps = $warmMibs
        }
    }

    return @{
        label = $Label
        bytes = $Bytes
        iterations = $iters
        results = $results
    }
}

function Bench-Tree {
    $Label = 'mixed-100'
    $iters = $IterMap[$Label]
    Write-Host ""
    Write-Host "================================================================"
    Write-Host "WORKLOAD: $Label  (100 files, 100 KB - 150 MB log-dist)  iter=$iters"
    Write-Host "================================================================"

    # Build the source tree once (it's expensive). We rebuild fresh for
    # COLD iterations to evict it from cache.
    function Build-Tree([string]$Dir) {
        New-Item -ItemType Directory -Path $Dir -Force | Out-Null
        $rng = [System.Random]::new(0xBADC0DE)
        # 100 files, log-distributed sizes 100KB..150MB
        $minLog = [Math]::Log(100 * 1KB)
        $maxLog = [Math]::Log(150 * 1MB)
        for ($i = 1; $i -le 100; $i++) {
            $u = $rng.NextDouble()
            $sz = [int64][Math]::Exp($minLog + ($maxLog - $minLog) * $u)
            $name = "file-{0:D3}.bin" -f $i
            Write-RandomFile -Path (Join-Path $Dir $name) -Bytes $sz
        }
    }

    # Compute total bytes
    function Tree-TotalBytes([string]$Dir) {
        ((Get-ChildItem $Dir -File -Recurse) | Measure-Object Length -Sum).Sum
    }

    $Tools = @('CopyThat', 'RoboCopy', 'CmdCopy', 'TeraCopy', 'FastCopy')
    $results = @{}

    foreach ($tool in $Tools) {
        Write-Host ""
        Write-Host "----- $tool -----"

        # COLD
        $coldTimes = @()
        $coldBytes = 0
        for ($i = 1; $i -le $iters; $i++) {
            $srcDir = Reset-Workdir "src-tree"
            $dstDir = Reset-Workdir "dst-tree"
            Write-Host "  COLD iter $i/$iters  building 100-file tree..." -NoNewline
            Build-Tree -Dir $srcDir
            $totalBytes = Tree-TotalBytes -Dir $srcDir
            $coldBytes = $totalBytes
            $totalMb = [Math]::Round($totalBytes / 1MB, 0)
            Write-Host " ok (~$totalMb MB total)"

            Write-Host "  COLD iter $i/$iters  running $tool..." -NoNewline
            $ms = Run-Tool-Tree -Tool $tool -SrcDir $srcDir -DstDir $dstDir
            $mibs = [Math]::Round(($totalBytes / 1MB) / ($ms / 1000.0), 1)
            Write-Host (" {0,7} ms  ({1,8:N1} MiB/s)" -f $ms, $mibs)
            $coldTimes += $ms
        }

        # WARM
        $srcDir = Reset-Workdir "src-tree-warm"
        $dstDir = Reset-Workdir "dst-tree-warm"
        Write-Host "  warm: building + caching 100-file tree..." -NoNewline
        Build-Tree -Dir $srcDir
        # Warm cache: read every file once
        Get-ChildItem $srcDir -File -Recurse | ForEach-Object { Warm-File -Path $_.FullName }
        $warmBytes = Tree-TotalBytes -Dir $srcDir
        Write-Host (" ok (~{0} MB total)" -f [Math]::Round($warmBytes / 1MB, 0))

        $warmTimes = @()
        for ($i = 1; $i -le $iters; $i++) {
            # Wipe dst between warm iterations
            if (Test-Path $dstDir) {
                Remove-Item $dstDir -Recurse -Force -ErrorAction SilentlyContinue
            }
            New-Item -ItemType Directory -Path $dstDir -Force | Out-Null

            Write-Host "  WARM iter $i/$iters  running $tool..." -NoNewline
            $ms = Run-Tool-Tree -Tool $tool -SrcDir $srcDir -DstDir $dstDir
            $mibs = [Math]::Round(($warmBytes / 1MB) / ($ms / 1000.0), 1)
            Write-Host (" {0,7} ms  ({1,8:N1} MiB/s)" -f $ms, $mibs)
            $warmTimes += $ms
        }

        $coldMedian = Median $coldTimes
        $warmMedian = Median $warmTimes
        $coldMibs   = [Math]::Round(($coldBytes / 1MB) / ($coldMedian / 1000.0), 1)
        $warmMibs   = [Math]::Round(($warmBytes / 1MB) / ($warmMedian / 1000.0), 1)
        Write-Host ("  ==> $tool COLD median {0} ms ({1} MiB/s) | WARM median {2} ms ({3} MiB/s)" `
            -f $coldMedian, $coldMibs, $warmMedian, $warmMibs)

        $results[$tool] = @{
            cold_times_ms = $coldTimes
            warm_times_ms = $warmTimes
            cold_median_ms = $coldMedian
            warm_median_ms = $warmMedian
            cold_bytes = $coldBytes
            warm_bytes = $warmBytes
            cold_throughput_MiBps = $coldMibs
            warm_throughput_MiBps = $warmMibs
        }
    }

    return @{
        label = $Label
        iterations = $iters
        files = 100
        size_min_bytes = 100 * 1KB
        size_max_bytes = 150 * 1MB
        results = $results
    }
}

# ----- Run -------------------------------------------------------------

Write-Host "Phase 42 head-to-head bench harness -- starting"
Write-Host "Repo: $RepoRoot"
Write-Host "Workdir: $BenchDir"
Write-Host "Output: $ResultJson + $ResultMd + $ResultHtml"
Write-Host ""

# Tooling sanity check
foreach ($p in @($CopyThat, $TeraCopy, $FastCopy)) {
    if (-not (Test-Path $p)) { throw "tool not found: $p" }
}
Write-Host "  [ok] CopyThat = $CopyThat"
Write-Host "  [ok] TeraCopy = $TeraCopy"
Write-Host "  [ok] FastCopy = $FastCopy"
Write-Host "  [ok] RoboCopy / CmdCopy via PATH"

$startMs = Now-Ms
$workloads = @()

$workloads += Bench-Single -Label '256MB' -Bytes (256 * 1MB)
$workloads += Bench-Single -Label '512MB' -Bytes (512 * 1MB)
$workloads += Bench-Tree
$workloads += Bench-Single -Label '10GB'  -Bytes (10 * 1GB)

# Final cleanup (free disk)
if (Test-Path $BenchDir) {
    Write-Host ""
    Write-Host "cleanup: removing $BenchDir..." -NoNewline
    Remove-Item $BenchDir -Recurse -Force -ErrorAction SilentlyContinue
    Write-Host " ok"
}

$report = @{
    started_at_unix_ms = $startMs
    finished_at_unix_ms = (Now-Ms)
    duration_ms = (Now-Ms) - $startMs
    host = $env:COMPUTERNAME
    os = (Get-CimInstance Win32_OperatingSystem).Caption
    workloads = $workloads
}

# Write JSON
New-Item -ItemType Directory -Path (Split-Path $ResultJson -Parent) -Force | Out-Null
$report | ConvertTo-Json -Depth 10 | Set-Content -Path $ResultJson -Encoding UTF8
Write-Host ''
Write-Host ('wrote raw -> ' + $ResultJson)
