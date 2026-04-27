# Phase 42 head-to-head benchmark

Generated 2026-04-26 23:27 from `target/bench-phase42.json`.
Run via `pwsh -File scripts/bench-phase42.ps1` then re-render via `pwsh -File scripts/bench-render.ps1`.

**Host**: MIKEDOUBLEYOU (Microsoft Windows 11 Home)
**Total bench duration**: 1452.2 s

## Hardware

These numbers are interpreted relative to the test rig - especially relevant for the cross-volume C: -> D: / C: -> E: scenarios where the destination drive's sustained-write ceiling caps every tool. If your hardware is faster, throughput will scale up; if slower, the ranking *between* tools should still hold.

- **CPU**: 13th Gen Intel(R) Core(TM) i9-13900HX (24C/32T @ 2.20 GHz)
- **RAM**: 15.7 GB

| Drive | Model | Media | Bus | Filesystem | Size | Free |
|---|---|---|---|---|---:|---:|
| **C:** | KXG80ZNV1T02 KIOXIA | SSD | NVMe | NTFS | 952.7 GB | 411.8 GB |
| **D:** | WD easystore 2647 | HDD | USB | NTFS | 3726 GB | 672 GB |
| **E:** | Samsung PSSD T7 | SSD | USB | exFAT | 1863 GB | 427.9 GB |

## Methodology

- **COLD**: a fresh source file is created on disk before *every* iteration. Source bytes have just been written and may or may not be in the OS page cache; this models real-world copy-of-a-just-saved-file.
- **WARM**: the source file is created once + read end-to-end to warm the page cache, then the same file is copied N times. Models repeated copies of an already-cached source.
- For each tool × workload × cache-state, we report the **median wall-clock** across N iterations.
- Same-volume copy (C: -> C:) on the test host. Cross-volume copies are disk-bound and tend to tie across all tools - the engine speed only matters when the disk isn't the bottleneck.

## Tools

| Tool | Invocation |
|---|---|
| **CopyThat** | `copythat copy <src> <dst>` (release binary) |
| **RoboCopy** | `robocopy <srcDir> <dstDir> <name> /NFL /NDL /NJH /NJS /NP /R:0 /W:0` |
| **cmd copy** | `cmd /C copy /Y <src> <dst>` |
| **TeraCopy** | `TeraCopy.exe Copy <src> <dstDir>\\ /Close /SkipAll` |
| **FastCopy** | `FastCopy.exe /cmd=force_copy /no_ui /auto_close /to=<dstDir>\\ <src>` |

---

## Workload: 256MB

Single file, 256 MB. **5 iterations** per cell, median reported.

### Throughput summary

| Rank | Tool | COLD median (ms) | COLD MiB/s | WARM median (ms) | WARM MiB/s |
|---:|---|---:|---:|---:|---:|
| 🥇 1 | RoboCopy | 63 | **4063.5** | 60 | **4266.7** |
| 🥈 2 | **CopyThat** | 63 | **4063.5** | 63 | **4063.5** |
| 🥉 3 | CmdCopy | 79 | **3240.5** | 73 | **3506.8** |
| 4 | FastCopy | 239 | **1071.1** | 222 | **1153.2** |
| 5 | TeraCopy | 1244 | **205.8** | 1244 | **205.8** |

### Visual -- COLD cache

```
  RoboCopy   ████████████████████████████████████████   4,063.5 MiB/s
  CopyThat   ████████████████████████████████████████   4,063.5 MiB/s
  CmdCopy    ████████████████████████████████░░░░░░░░   3,240.5 MiB/s
  FastCopy   ███████████░░░░░░░░░░░░░░░░░░░░░░░░░░░░░   1,071.1 MiB/s
  TeraCopy   ██░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░     205.8 MiB/s
```

### Visual -- WARM cache

```
  RoboCopy   ████████████████████████████████████████   4,266.7 MiB/s
  CopyThat   ██████████████████████████████████████░░   4,063.5 MiB/s
  CmdCopy    █████████████████████████████████░░░░░░░   3,506.8 MiB/s
  FastCopy   ███████████░░░░░░░░░░░░░░░░░░░░░░░░░░░░░   1,153.2 MiB/s
  TeraCopy   ██░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░     205.8 MiB/s
```

### CopyThat vs competitors

| Competitor | COLD: CopyThat is | WARM: CopyThat is |
|---|:---:|:---:|
| CmdCopy | **+25.4% faster** | **+15.9% faster** |
| RoboCopy | tied | -4.8% slower |
| FastCopy | **+279.4% faster** | **+252.4% faster** |
| TeraCopy | **+1874.5% faster** | **+1874.5% faster** |

### Full pairwise matrix (COLD MiB/s; row vs column = how much faster row is)

| | CmdCopy | RoboCopy | FastCopy | TeraCopy | CopyThat |
|---|---|---|---|---|---|
| **CmdCopy** | -- | -20.3% | +202.5% | +1474.6% | -20.3% |
| **RoboCopy** | +25.4% | -- | +279.4% | +1874.5% | 0% |
| **FastCopy** | -66.9% | -73.6% | -- | +420.5% | -73.6% |
| **TeraCopy** | -93.6% | -94.9% | -80.8% | -- | -94.9% |
| **CopyThat** | +25.4% | 0% | +279.4% | +1874.5% | -- |

### Per-iteration seconds (each individual copy)

| Tool | COLD iter seconds | COLD sum | WARM iter seconds | WARM sum |
|---|---|---:|---|---:|
| CmdCopy | 0.097 s, 0.079 s, 0.066 s, 0.081 s, 0.063 s | **0.386 s** | 0.073 s, 0.071 s, 0.075 s, 0.071 s, 0.080 s | **0.370 s** |
| RoboCopy | 0.063 s, 0.063 s, 0.063 s, 0.063 s, 0.079 s | **0.331 s** | 0.058 s, 0.084 s, 0.056 s, 0.063 s, 0.060 s | **0.321 s** |
| FastCopy | 0.237 s, 0.253 s, 0.239 s, 0.269 s, 0.238 s | **1.236 s** | 0.223 s, 0.222 s, 0.222 s, 0.232 s, 0.220 s | **1.119 s** |
| TeraCopy | 1.323 s, 1.238 s, 1.467 s, 1.243 s, 1.244 s | **6.515 s** | 1.453 s, 1.244 s, 1.234 s, 1.224 s, 1.481 s | **6.636 s** |
| CopyThat | 0.072 s, 0.063 s, 0.063 s, 0.063 s, 0.070 s | **0.331 s** | 0.063 s, 0.059 s, 0.064 s, 0.064 s, 0.061 s | **0.311 s** |

<details><summary>Raw iteration timings (ms)</summary>

| Tool | COLD all iters | WARM all iters |
|---|---|---|
| CmdCopy | 97, 79, 66, 81, 63 | 73, 71, 75, 71, 80 |
| RoboCopy | 63, 63, 63, 63, 79 | 58, 84, 56, 63, 60 |
| FastCopy | 237, 253, 239, 269, 238 | 223, 222, 222, 232, 220 |
| TeraCopy | 1323, 1238, 1467, 1243, 1244 | 1453, 1244, 1234, 1224, 1481 |
| CopyThat | 72, 63, 63, 63, 70 | 63, 59, 64, 64, 61 |

</details>

---

## Workload: 512MB

Single file, 512 MB. **4 iterations** per cell, median reported.

### Throughput summary

| Rank | Tool | COLD median (ms) | COLD MiB/s | WARM median (ms) | WARM MiB/s |
|---:|---|---:|---:|---:|---:|
| 🥇 1 | **CopyThat** | 126 | **4063.5** | 123 | **4162.6** |
| 🥈 2 | CmdCopy | 126 | **4063.5** | 126 | **4063.5** |
| 🥉 3 | RoboCopy | 135 | **3792.6** | 126 | **4063.5** |
| 4 | FastCopy | 393 | **1302.8** | 381 | **1343.8** |
| 5 | TeraCopy | 1304 | **392.6** | 1353 | **378.4** |

### Visual -- COLD cache

```
  CopyThat   ████████████████████████████████████████   4,063.5 MiB/s
  CmdCopy    ████████████████████████████████████████   4,063.5 MiB/s
  RoboCopy   █████████████████████████████████████░░░   3,792.6 MiB/s
  FastCopy   █████████████░░░░░░░░░░░░░░░░░░░░░░░░░░░   1,302.8 MiB/s
  TeraCopy   ████░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░     392.6 MiB/s
```

### Visual -- WARM cache

```
  CopyThat   ████████████████████████████████████████   4,162.6 MiB/s
  CmdCopy    ███████████████████████████████████████░   4,063.5 MiB/s
  RoboCopy   ███████████████████████████████████████░   4,063.5 MiB/s
  FastCopy   █████████████░░░░░░░░░░░░░░░░░░░░░░░░░░░   1,343.8 MiB/s
  TeraCopy   ████░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░     378.4 MiB/s
```

### CopyThat vs competitors

| Competitor | COLD: CopyThat is | WARM: CopyThat is |
|---|:---:|:---:|
| CmdCopy | tied | **+2.4% faster** |
| RoboCopy | **+7.1% faster** | **+2.4% faster** |
| FastCopy | **+211.9% faster** | **+209.8% faster** |
| TeraCopy | **+935% faster** | **+1000.1% faster** |

### Full pairwise matrix (COLD MiB/s; row vs column = how much faster row is)

| | CmdCopy | RoboCopy | FastCopy | TeraCopy | CopyThat |
|---|---|---|---|---|---|
| **CmdCopy** | -- | +7.1% | +211.9% | +935% | 0% |
| **RoboCopy** | -6.7% | -- | +191.1% | +866% | -6.7% |
| **FastCopy** | -67.9% | -65.6% | -- | +231.8% | -67.9% |
| **TeraCopy** | -90.3% | -89.6% | -69.9% | -- | -90.3% |
| **CopyThat** | 0% | +7.1% | +211.9% | +935% | -- |

### Per-iteration seconds (each individual copy)

| Tool | COLD iter seconds | COLD sum | WARM iter seconds | WARM sum |
|---|---|---:|---|---:|
| CmdCopy | 0.142 s, 0.111 s, 0.126 s, 0.111 s | **0.490 s** | 0.126 s, 0.173 s, 0.110 s, 0.121 s | **0.530 s** |
| RoboCopy | 0.135 s, 0.126 s, 0.142 s, 0.111 s | **0.514 s** | 0.127 s, 0.126 s, 0.110 s, 0.111 s | **0.474 s** |
| FastCopy | 0.421 s, 0.393 s, 0.365 s, 0.359 s | **1.538 s** | 0.381 s, 0.348 s, 0.333 s, 0.396 s | **1.458 s** |
| TeraCopy | 1.304 s, 1.319 s, 1.293 s, 1.297 s | **5.213 s** | 1.294 s, 1.353 s, 1.365 s, 1.304 s | **5.316 s** |
| CopyThat | 0.111 s, 0.114 s, 0.126 s, 0.130 s | **0.481 s** | 0.123 s, 0.117 s, 0.125 s, 0.116 s | **0.481 s** |

<details><summary>Raw iteration timings (ms)</summary>

| Tool | COLD all iters | WARM all iters |
|---|---|---|
| CmdCopy | 142, 111, 126, 111 | 126, 173, 110, 121 |
| RoboCopy | 135, 126, 142, 111 | 127, 126, 110, 111 |
| FastCopy | 421, 393, 365, 359 | 381, 348, 333, 396 |
| TeraCopy | 1304, 1319, 1293, 1297 | 1294, 1353, 1365, 1304 |
| CopyThat | 111, 114, 126, 130 | 123, 117, 125, 116 |

</details>

---

## Workload: mixed-100

100 files, 100 KB - 150 MB log-distributed (~1719 MB total). **3 iterations** per cell, median reported.

### Throughput summary

| Rank | Tool | COLD median (ms) | COLD MiB/s | WARM median (ms) | WARM MiB/s |
|---:|---|---:|---:|---:|---:|
| 🥇 1 | **CopyThat** | 442 | **3889** | 490 | **3508.1** |
| 🥈 2 | RoboCopy | 487 | **3529.7** | 802 | **2143.3** |
| 🥉 3 | CmdCopy | 554 | **3102.8** | 692 | **2484** |
| 4 | FastCopy | 1182 | **1454.3** | 1128 | **1523.9** |
| 5 | TeraCopy | 1792 | **959.2** | 2366 | **726.5** |

### Visual -- COLD cache

```
  CopyThat   ████████████████████████████████████████   3,889.0 MiB/s
  RoboCopy   ████████████████████████████████████░░░░   3,529.7 MiB/s
  CmdCopy    ████████████████████████████████░░░░░░░░   3,102.8 MiB/s
  FastCopy   ███████████████░░░░░░░░░░░░░░░░░░░░░░░░░   1,454.3 MiB/s
  TeraCopy   ██████████░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░     959.2 MiB/s
```

### Visual -- WARM cache

```
  CopyThat   ████████████████████████████████████████   3,508.1 MiB/s
  CmdCopy    ████████████████████████████░░░░░░░░░░░░   2,484.0 MiB/s
  RoboCopy   ████████████████████████░░░░░░░░░░░░░░░░   2,143.3 MiB/s
  FastCopy   █████████████████░░░░░░░░░░░░░░░░░░░░░░░   1,523.9 MiB/s
  TeraCopy   ████████░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░     726.5 MiB/s
```

### CopyThat vs competitors

| Competitor | COLD: CopyThat is | WARM: CopyThat is |
|---|:---:|:---:|
| CmdCopy | **+25.3% faster** | **+41.2% faster** |
| RoboCopy | **+10.2% faster** | **+63.7% faster** |
| FastCopy | **+167.4% faster** | **+130.2% faster** |
| TeraCopy | **+305.4% faster** | **+382.9% faster** |

### Full pairwise matrix (COLD MiB/s; row vs column = how much faster row is)

| | CmdCopy | RoboCopy | FastCopy | TeraCopy | CopyThat |
|---|---|---|---|---|---|
| **CmdCopy** | -- | -12.1% | +113.4% | +223.5% | -20.2% |
| **RoboCopy** | +13.8% | -- | +142.7% | +268% | -9.2% |
| **FastCopy** | -53.1% | -58.8% | -- | +51.6% | -62.6% |
| **TeraCopy** | -69.1% | -72.8% | -34% | -- | -75.3% |
| **CopyThat** | +25.3% | +10.2% | +167.4% | +305.4% | -- |

### Per-iteration seconds (each individual copy)

| Tool | COLD iter seconds | COLD sum | WARM iter seconds | WARM sum |
|---|---|---:|---|---:|
| CmdCopy | 0.583 s, 0.492 s, 0.554 s | **1.629 s** | 0.568 s, 0.692 s, 0.840 s | **2.100 s** |
| RoboCopy | 0.533 s, 0.487 s, 0.478 s | **1.498 s** | 0.615 s, 0.802 s, 0.861 s | **2.278 s** |
| FastCopy | 1.133 s, 1.182 s, 1.196 s | **3.511 s** | 1.170 s, 1.128 s, 1.101 s | **3.399 s** |
| TeraCopy | 1.918 s, 1.792 s, 1.747 s | **5.457 s** | 1.887 s, 2.396 s, 2.366 s | **6.649 s** |
| CopyThat | 0.459 s, 0.442 s, 0.395 s | **1.296 s** | 0.490 s, 0.459 s, 0.570 s | **1.519 s** |

<details><summary>Raw iteration timings (ms)</summary>

| Tool | COLD all iters | WARM all iters |
|---|---|---|
| CmdCopy | 583, 492, 554 | 568, 692, 840 |
| RoboCopy | 533, 487, 478 | 615, 802, 861 |
| FastCopy | 1133, 1182, 1196 | 1170, 1128, 1101 |
| TeraCopy | 1918, 1792, 1747 | 1887, 2396, 2366 |
| CopyThat | 459, 442, 395 | 490, 459, 570 |

</details>

---

## Workload: 10GB

Single file, 10240 MB. **2 iterations** per cell, median reported.

### Throughput summary

| Rank | Tool | COLD median (ms) | COLD MiB/s | WARM median (ms) | WARM MiB/s |
|---:|---|---:|---:|---:|---:|
| 🥇 1 | CmdCopy | 4756 | **2153.1** | 20797 | **492.4** |
| 🥈 2 | RoboCopy | 4849 | **2111.8** | 20930 | **489.2** |
| 🥉 3 | **CopyThat** | 4975 | **2058.3** | 7040 | **1454.5** |
| 4 | FastCopy | 7605 | **1346.5** | 9495 | **1078.5** |
| 5 | TeraCopy | 8993 | **1138.7** | 11503 | **890.2** |

### Visual -- COLD cache

```
  CmdCopy    ████████████████████████████████████████   2,153.1 MiB/s
  RoboCopy   ███████████████████████████████████████░   2,111.8 MiB/s
  CopyThat   ██████████████████████████████████████░░   2,058.3 MiB/s
  FastCopy   █████████████████████████░░░░░░░░░░░░░░░   1,346.5 MiB/s
  TeraCopy   █████████████████████░░░░░░░░░░░░░░░░░░░   1,138.7 MiB/s
```

### Visual -- WARM cache

```
  CopyThat   ████████████████████████████████████████   1,454.5 MiB/s
  FastCopy   ██████████████████████████████░░░░░░░░░░   1,078.5 MiB/s
  TeraCopy   ████████████████████████░░░░░░░░░░░░░░░░     890.2 MiB/s
  CmdCopy    ██████████████░░░░░░░░░░░░░░░░░░░░░░░░░░     492.4 MiB/s
  RoboCopy   █████████████░░░░░░░░░░░░░░░░░░░░░░░░░░░     489.2 MiB/s
```

### CopyThat vs competitors

| Competitor | COLD: CopyThat is | WARM: CopyThat is |
|---|:---:|:---:|
| CmdCopy | -4.4% slower | **+195.4% faster** |
| RoboCopy | -2.5% slower | **+197.3% faster** |
| FastCopy | **+52.9% faster** | **+34.9% faster** |
| TeraCopy | **+80.8% faster** | **+63.4% faster** |

### Full pairwise matrix (COLD MiB/s; row vs column = how much faster row is)

| | CmdCopy | RoboCopy | FastCopy | TeraCopy | CopyThat |
|---|---|---|---|---|---|
| **CmdCopy** | -- | +2% | +59.9% | +89.1% | +4.6% |
| **RoboCopy** | -1.9% | -- | +56.8% | +85.5% | +2.6% |
| **FastCopy** | -37.5% | -36.2% | -- | +18.2% | -34.6% |
| **TeraCopy** | -47.1% | -46.1% | -15.4% | -- | -44.7% |
| **CopyThat** | -4.4% | -2.5% | +52.9% | +80.8% | -- |

### Per-iteration seconds (each individual copy)

| Tool | COLD iter seconds | COLD sum | WARM iter seconds | WARM sum |
|---|---|---:|---|---:|
| CmdCopy | 4.756 s, 4.688 s | **9.444 s** | 5.068 s, 20.797 s | **25.865 s** |
| RoboCopy | 4.849 s, 4.800 s | **9.649 s** | 4.704 s, 20.930 s | **25.634 s** |
| FastCopy | 7.605 s, 7.215 s | **14.820 s** | 7.919 s, 9.495 s | **17.414 s** |
| TeraCopy | 8.992 s, 8.993 s | **17.985 s** | 6.928 s, 11.503 s | **18.431 s** |
| CopyThat | 4.975 s, 4.948 s | **9.923 s** | 5.374 s, 7.040 s | **12.414 s** |

<details><summary>Raw iteration timings (ms)</summary>

| Tool | COLD all iters | WARM all iters |
|---|---|---|
| CmdCopy | 4756, 4688 | 5068, 20797 |
| RoboCopy | 4849, 4800 | 4704, 20930 |
| FastCopy | 7605, 7215 | 7919, 9495 |
| TeraCopy | 8992, 8993 | 6928, 11503 |
| CopyThat | 4975, 4948 | 5374, 7040 |

</details>

---

## Cross-workload summary -- CopyThat throughput vs the field

| Workload | CopyThat COLD MiB/s | Field-best COLD MiB/s | Δ vs best | CopyThat WARM MiB/s | Field-best WARM MiB/s | Δ vs best |
|---|---:|---:|---:|---:|---:|---:|
| 256MB | **4063.5** | 4063.5 (RoboCopy) | tied | **4063.5** | 4266.7 (RoboCopy) | -4.8% (best: RoboCopy) |
| 512MB | **4063.5** | 4063.5 (CopyThat) | 🥇 best | **4162.6** | 4162.6 (CopyThat) | 🥇 best |
| mixed-100 | **3889** | 3889 (CopyThat) | 🥇 best | **3508.1** | 3508.1 (CopyThat) | 🥇 best |
| 10GB | **2058.3** | 2153.1 (CmdCopy) | -4.4% (best: CmdCopy) | **1454.5** | 1454.5 (CopyThat) | 🥇 best |

---

## Phase 13c parallel-chunks A/B (do not flip without re-bench)

The Phase 13c parallel multi-chunk copy path is **default-off** for a reason.
Phase 13c re-bench on Windows 11 NVMe at 10 GiB measured a **regression**:

- C -> C: single-stream CopyFileExW 1080 MiB/s -> parallel-4 **809 MiB/s (-25%)**
- C -> E: single-stream CopyFileExW 328 MiB/s -> parallel-4 **80 MiB/s (-76%)**

Modern NVMe + Windows 11's CopyFileExW already saturates the per-device queue with a single stream; splitting into 4 streams adds per-chunk seek + handle-open overhead and contends for the same hardware queue.
Opt-in via COPYTHAT_PARALLEL_CHUNKS=<N> if your topology (RAID array, NVMe-over-fabric, distributed FS) might benefit; do not promote the path to default without a fresh A/B.

