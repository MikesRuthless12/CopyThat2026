# Copy That v1.0.0

A lightweight, cross-platform, async, byte-exact file/folder copier in Rust —
matching every feature of TeraCopy and pushing past it, while staying as fast
as (or faster than) Explorer / Finder / `cp` / `rsync` for typical desktop
workloads.

![Stack](https://img.shields.io/badge/stack-Rust%20%2B%20Tauri%202.x%20%2B%20Svelte%205-blue)
![Platforms](https://img.shields.io/badge/platforms-Windows%20%7C%20macOS%20%7C%20Linux-success)
![Languages](https://img.shields.io/badge/languages-18-brightgreen)
![License](https://img.shields.io/badge/license-All%20Rights%20Reserved-red)

## Features

### Copy engine

- **Byte-exact async copy** with selectable verification (CRC32, MD5, SHA-1, SHA-256, SHA-512, xxHash3-64, xxHash3-128, BLAKE3).
- **OS-native fast paths** dispatched per platform: `CopyFileExW` (Windows, with `COPY_FILE_NO_BUFFERING` for files ≥ 256 MiB), `copyfile(3)` with `COPYFILE_ALL` (macOS), `copy_file_range(2)` + `sendfile(2)` fallback (Linux).
- **Reflink clones** on supported filesystems (Btrfs, XFS with reflink, ZFS, bcachefs, APFS, ReFS / Dev Drive) via `reflink-copy`, with a same-volume guard that skips the syscall on cross-volume copies.
- **Parallel multi-chunk** copy for large files (default 4 chunks at matched memory budget). Opt-in via `COPYTHAT_PARALLEL_CHUNKS`; see [`docs/BENCHMARKS.md`](docs/BENCHMARKS.md).
- **Dynamic buffer sizing** — 64 KiB → 4 MiB picked per file by `CopyOptions::buffer_size_for_file` from the Phase 13b sweep, which confirmed 1 MiB as the mid-band optimum.
- **Streaming tree enumeration** — walker feeds 100 k-entry `Plan` chunks through a bounded channel while copies run in parallel. **Tree size is bounded only by the destination volume, not by RAM** — drives with tens of millions of files work the same as ten-file folders.
- **Concurrency control** tuned per-destination with an HDD heuristic that clamps to a single worker on rotational media.

### Resilience

- **Locked-file copy on Windows** — opens the source with `FILE_SHARE_READ | WRITE | DELETE` plus an exponential-backoff retry on `ERROR_SHARING_VIOLATION`, so in-use log files, loaded DLLs, and Office documents copy without admin.
- **Symlink preserve + graceful degrade** — clones symlinks when the process has privilege (or Windows Developer Mode is on); falls back to copying the target contents as a regular file so nothing is lost.
- **Access-denied tolerance** — `$RECYCLE.BIN\S-1-5-*`, `System Volume Information`, `Config.Msi`, etc. are skipped and counted, not fatal.
- **Collision policy** picked up-front in the drop dialog: *Keep both* (`foo.txt` → `foo_2.txt`, `foo_3.txt`, …), *Skip*, *Overwrite*, *Overwrite only if newer*, *Ask each time*.
- **Error policy** per-job: *Ask*, *Skip*, *Abort*, or *Retry N times*.
- **Error prompt style** — blocking modal or non-blocking corner drawer; user-configurable in Settings → General.
- **Pause / resume / cancel** on every job, plus pause-all / resume-all / cancel-all.
- **Reserve free space on destination** — optional minimum, preflight-warned before the first byte moves.
- **Enumeration-time filters** — include / exclude globs (`**/*.txt`, `**/node_modules`), size range, modification-time range, and attribute bits (hidden, system, read-only) are applied inside the walker so the engine never even opens the files you didn't want. Exclude globs that match a directory prune the whole subtree; include globs gate files only, so a `**/*.txt` filter still descends every folder to find the text files inside. Configured per-install under Settings → Filters.

### Interface

- **Tauri 2.x + Svelte 5** single-window shell; dense default theme, light / dark / system.
- **Activity panel** with one row per file the engine touches: live circular ring, `src → dst` truncated, `bytes_done / total`, and icons for pending / in-flight / done / error / folder. Caps at 250 000 rows for stable performance on 4 GB systems.
- **Sort + reorder** the pre-copy source list by name or size, asc or desc, with `Home` / `End` / `Ctrl+↑↓` / `Shift+↑↓` keyboard shortcuts.
- **Files-first grouping** — sorts always put files ahead of folders, enforced even through manual drag-reorders.
- **Preflight free-space check** with a subset-picker modal when the full set won't fit.
- **History** (SQLite): every job logs kind, source, destination, verify algorithm, size, status, duration; filter by kind / status / text; export CSV; clear-all with two-step confirm; **Rerun** any past job with one click.
- **Lifetime Totals drawer** — bytes, files, jobs, errors, by-kind breakdown, 30-day sparkline.
- **Per-file live counter**, verbose compact ETA (`3h 10m 38s`), average MiB/s — header never says "calculating…" when idle.
- **When-done action** — keep app open, close app, log off, shut down, sleep.
- **System tray** with minimize-to-tray-on-close option.
- **Drag-and-drop and picker** entry points: drag onto the window, or use *Add files* / *Add folders* in the header.
- **Shell integration** — Windows IExplorerCommand, macOS Finder Sync, Linux `.desktop` / KDE ServiceMenu / Thunar UCA all route into the same queue via `copythat --enqueue <verb> <paths…> [--destination <dst>]`.
- **Global paste hotkey** — press Ctrl+Shift+V (Windows / Linux) or Cmd+Shift+V (macOS) anywhere on the system to paste files copied from Explorer / Finder / Files through Copy That's staging dialog. Combo is user-configurable; the shortcut can be toggled off entirely.
- **Clipboard watcher** (opt-in) — surfaces a toast when file URLs land on the OS clipboard, hinting the paste hotkey is ready. Polls every 500 ms while enabled; silent when off.
- **Secure delete** — single-pass zero / random, DoD 3-pass, DoD 7-pass, Gutmann 35-pass, NIST 800-88. SSD-aware (skips multi-pass overwrites on SSDs by default — use TRIM instead).
- **Read-through-snapshot for locked files** — when another process holds the source open for exclusive write, Copy That can pull a read-only filesystem snapshot (VSS on Windows, ZFS / Btrfs on Linux, APFS local snapshot on macOS) and copy from there instead of surfacing the "file in use" error. Opt-in via Settings → Transfer → "When a file is locked". The Windows path spawns a sibling `copythat-helper-vss.exe` via UAC so the main app never needs elevation of its own.
- **Crash / reboot resume** — every 50 ms the copy engine fsync's the destination and writes a checkpoint to a redb-backed journal at `<data-dir>/copythat-journal.redb` (carrying the running BLAKE3 of consumed source bytes). On the next launch Copy That detects unfinished jobs, BLAKE3-verifies the partial destination's prefix, and seeks both source and destination past the verified offset — power-cut at 96 % of a 2 TB transfer no longer means starting over. Toggle "Auto-resume interrupted jobs without prompting" in Settings → General to skip the resume modal.
- **Bandwidth shaping** — Settings → Network lets you cap transfers at a fixed MB/s, follow a daily rclone-style schedule (`08:00,512k 18:00,10M Sat-Sun,unlimited`), or react automatically to metered Wi-Fi / battery / cellular. Cap applies globally across every in-flight job; a minute-tick poller re-evaluates the schedule so a 09:00 boundary change hot-swaps the live rate without touching running jobs. Header badge (🔻 30 MB/s · scheduled) shows the active cap — click it to open the Network tab. GCRA token bucket via the `governor` crate is accurate to a handful of milliseconds across the full range (1 KB/s → storage-native ceiling).
- **Sparse file preservation** — a 100 GB VM disk with 1 MB of real data stays 1 MB on disk at the destination. Copy That detects allocated extents via `SEEK_HOLE` / `SEEK_DATA` on Linux / macOS and `FSCTL_QUERY_ALLOCATED_RANGES` on Windows, marks the destination sparse (NTFS `FSCTL_SET_SPARSE`; automatic on APFS / ext4 / Btrfs / XFS / ZFS / ReFS), and seeks-copies only the allocated ranges so holes are preserved by omission. Toggle in Settings → Transfer → *Preserve sparse files* (default on); destinations that can't preserve sparseness (exFAT / FAT32 / HFS+) raise a one-shot "destination fills sparse files" toast and densify.
- **Security metadata preservation** — out-of-band metadata streams round-trip across every copy: Windows NTFS Alternate Data Streams (including the **Mark-of-the-Web** `Zone.Identifier` that SmartScreen and Office Protected View key off — a downloaded `.exe` keeps its origin warning after the copy), Linux extended attributes + POSIX ACLs + SELinux contexts + file capabilities (`security.capability`), macOS xattrs (Gatekeeper `com.apple.quarantine`, Spotlight `kMDItemWhereFroms`, Finder color tags) + the legacy resource fork at `<file>/..namedfork/rsrc`. Cross-FS destinations that can't hold the foreign metadata (SMB / FAT / exFAT / ext4 receiving Mac data) automatically fall through to an `._<filename>` AppleDouble v2 sidecar so the data survives the trip. Settings → Transfer → "Security metadata" gives per-stream toggles; the MOTW toggle carries an explicit security warning since disabling it lets a downloaded executable shed its origin marker on copy.
- **Two-way sync with conflict detection** — configure a sync pair (two folders on any filesystem) and Copy That reconciles them without silent overwrites. Per-file vector clocks persisted in `<left>/.copythat-sync.db` (redb) detect three cases: one side is an ancestor of the other (propagate), both diverged concurrently from a common ancestor (conflict), or vectors match but content diverged (corrupt). Concurrent edits are never auto-resolved destructively — the losing side's content is preserved in place as `name.sync-conflict-YYYYMMDD-HHMMSS-<host>.ext`, and both versions surface in the Sync drawer for per-file resolution (Keep left / Keep right / Keep both). Four modes: two-way, mirror (either direction), and contribute (one-way upload that never deletes on the receiver). Moves model as delete+add until the three-tree state machine lands in Phase 52.
- **Real-time mirror (live sync)** — toggle "Start live mirror" on any sync pair and Copy That watches the left-side folder (ReadDirectoryChangesW on Windows, FSEvents on macOS, inotify on Linux) and re-syncs on every debounced change. Edge cases that normally produce spurious sync rounds — vim's `:wq` dance (5 inotify events, 1 logical save), Word's `~$lock` + temp-rename dance, atomic writes like `write-to-tmp → fsync → rename-over` — all collapse to a single `Modified(file.ext)` event by the watcher's filter + atomic-save tracker + priority-ordered debounce queue (Removed > Renamed > Modified > Created). Windows ReadDirectoryChangesW buffer overflows trigger a recovery rescan so no change is silently dropped; macOS FSEvents directory-coalesce signals are resolved by synchronous enumeration. One background thread per active pair, surfaced in the UI as a green pulsing dot + "Watching" badge.

### Internationalisation

- **18 languages** shipped in full: Arabic, Chinese (Simplified), Dutch, English, French, German, Hindi, Indonesian, Italian, Japanese, Korean, Polish, Portuguese (Brazil), Russian, Spanish, Turkish, Ukrainian, Vietnamese.
- **Fluent**-based (`.ftl`) key system with `xtask i18n-lint` enforcing key parity + syntax.
- **RTL support** (Arabic) with `dir="rtl"` on the root and icon mirroring.
- **Locale-driven formatting**: `Intl.NumberFormat` for bytes / percent / counts, `Intl.DisplayNames` for language pickers (each language renders in its own endonym + in the active UI locale).
- **Reactive translation switching** — changes apply instantly across every modal, drawer, and badge without reload.

### Cloud backends (Phase 32)

- **12 backends via OpenDAL + one custom SFTP path**: Amazon S3, Cloudflare R2, Backblaze B2, Azure Blob, Google Cloud Storage, OneDrive, Google Drive, Dropbox, WebDAV, FTP, LocalFs — plus SFTP via a pure-Rust MIT-licensed `russh` / `russh-sftp` backend (avoids OpenDAL's GPL-blocked `services-sftp`).
- **Settings → Remotes tab** for add / remove / test-connection; secrets live in the OS keychain (Apple Keychain / Windows Credential Manager / Secret Service / kwallet) under `copythat-cloud/<backend-name>`, never in `settings.toml`.
- **Engine-level cloud destinations**: `copy_file` routes through `CopyOptions::cloud_sink` when the destination is remote, streaming the source in `buffer_size` chunks through `opendal::Operator::writer()` with multipart upload handled transparently (S3-class backends). Progress events fire per-chunk with running byte counts.
- **Verify-on-remote** with four algorithms (BLAKE3, SHA-256, MD5, round-trip). The server-side checksum fast-path (ETag on S3-class, `content-md5` on Azure Blob) skips the round-trip fetch when the backend exposes a comparable hash.
- **OAuth device-code flow** for Microsoft Graph + Google Drive + Dropbox, plus an **RFC 7636 PKCE browser-redirect flow** for providers where device-code isn't available. Refresh-token flow preserves the old refresh_token when providers (MS Graph) omit it on refresh. Loopback HTTP receiver on `127.0.0.1:<auto-port>` catches the authorize redirect.
- **SFTP key forms**: password (default), unencrypted OpenSSH private key (`KEY\n<body>`), encrypted OpenSSH private key (`KEY_ENC\n<passphrase>\n<body>`). `known_hosts` pinning with both plain + hashed `|1|salt|hmac` entries (HMAC-SHA1 host matching). Connection pooling via an `Arc<AsyncMutex<PooledSession>>` so concurrent put / get calls reuse a live SSH channel and re-handshake lazily on error.
- **Resume journal awareness**: `JobRecord::remote_backend_name` distinguishes cloud-dst from local-dst at startup-resume time; cross-process MPU resume is intentionally not handled (Copy That's remote-destination model is idempotent re-copy).

### Mount the archive (Phase 33)

- **Cross-platform read-only mount** of the Copy That snapshot history: `by-date/YYYY-MM-DD/HH-MM-SS/<job>` + `by-source/<escaped-src-root>/<timestamp>` + `by-job-id/<row-id>/<job>`. The mount exposes historical copies as plain folders — `cat` / Explorer / Finder all work.
- **FUSE** on Linux/macOS behind `--features fuse` (pulls `fuser` 0.15, MIT). Real `fuser::Filesystem` trait impl with `lookup` / `getattr` / `readdir` / `read` consulting `TreeInodeMap` + the Phase 27 chunk store.
- **WinFsp** on Windows behind `--features winfsp` (pulls `winfsp_wrs` 0.4, MIT — replaces the GPL-blocked `winfsp-sys`). Real `winfsp_wrs::FileSystemInterface` impl overriding `get_volume_info`, `get_security_by_name`, `open`, `close`, `read`, `read_directory`, `get_file_info`. Default SDDL (`D:P(A;;GR;;;WD)`) grants everyone generic-read.
- **Chunk-streaming reads**: `read` callbacks walk each item's Phase 27 manifest by cursor, fetching only chunks that overlap `[offset, offset+size)` — multi-GB snapshots don't spike memory.
- **History context menu**: *Mount snapshot* on any successful history row opens a directory picker + dispatches the mount IPC; *Settings → Advanced* has a *Mount latest on launch* toggle.
- **USER ACTION** to validate real kernel mounts: run `cargo test -p copythat-mount --features fuse` on Linux/macOS, or `cargo test -p copythat-mount --features winfsp` on Windows (requires the WinFsp driver + LLVM libclang installed — see [`docs/ROADMAP.md`](docs/ROADMAP.md) Phase 33g for the exact `winget` commands).

### Enterprise audit log (Phase 34)

- **Eight typed events** — `JobStarted`, `JobCompleted`, `FileCopied`, `FileFailed`, `CollisionResolved`, `SettingsChanged`, `LoginEvent`, `UnauthorizedAccess` — are written into an append-only log sink every time the engine hits a user-visible transition.
- **Five wire formats** ship simultaneously: **CSV** (eight-column stable header for spreadsheet forensics), **JSON-lines** (default, pipe-friendly), **Syslog RFC 5424** (structured-data block `[copythat@32473 jobId="..." user="..."]`), **ArcSight CEF v0** (`CEF:0|CopyThat|CopyThat|<ver>|<sig>|<name>|<sev>|<ext>`), and **IBM QRadar LEEF 2.0** (tab-separated extension, `LEEF:2.0|CopyThat|CopyThat|<ver>|<eventID>|<ext>`).
- **Tamper-evident chain**: every record embeds `prev_hash = BLAKE3(previous_record_bytes)` as a hex column; one edit anywhere in the file cascades into every subsequent record's chain step. The Settings → *Verify chain* button + the Phase 36 CLI's `copythat audit verify <log>` catch single-byte tampering.
- **WORM (write-once-read-many)** is an opt-in Settings toggle that applies the platform's append-only primitive after every create / rotate: Linux `FS_IOC_SETFLAGS | FS_APPEND_FL` (requires CAP_LINUX_IMMUTABLE), macOS `chflags(UF_APPEND)`, Windows `FILE_ATTRIBUTE_READONLY` (the richer deny-write ACE path is a Phase 36 follow-up).
- **Rotation** rolls the log to `<path>.1` at a byte threshold (default 10 MiB, slider in Settings); the rollover re-applies the WORM attribute to the fresh file.
- **Tracing fan-out**: `tracing_subscriber::Layer` captures any `target = "copythat::audit"` log from the rest of the workspace into the sink so ad-hoc `warn!` calls auditors care about land in the same file.
- **Settings → Advanced → Audit log** surfaces the enable toggle, format picker, file path, rotation threshold, WORM toggle, and *Test write* + *Verify chain* buttons. The sink hot-swaps on every `update_settings` without restart.

### Encryption + on-the-fly compression (Phase 35)

- **Destination encryption via [age](https://age-encryption.org/)** — passphrase, X25519 (`age1…`), or SSH ed25519 / RSA recipients. Output is bit-for-bit compatible with the upstream `rage` CLI's binary format, so a future user can decrypt with the official `rage` binary without Copy That in the loop.
- **On-the-fly zstd compression** with three modes: *Off* (default), *Always* (every file at the chosen level), *Smart* (every file *except* a built-in 38-extension deny list covering jpg / mp4 / zip / 7z / pdf / msi / iso / etc. — re-compressing already-compressed media wastes CPU and can grow the file). User can append extra extensions to the Smart deny list.
- **zstd level 1–22 slider** in Settings → Transfer → Compression. Default is 3 (zstd's CLI default and the workspace's "fast + useful" pick); higher numbers compress harder at progressively-larger CPU cost.
- **Live compression metrics**: every file's `CopyEvent::CompressionSavings { ratio, bytes_saved }` event powers a footer "💾 256 MiB → 84 MiB (67% saved)" badge against the running tree totals.
- **Engine pipeline** (when either stage is active): `src bytes → optional zstd encoder → optional age encryptor → dst`. The pipeline runs on a `spawn_blocking` worker so age + zstd (both sync-first libraries) compose cleanly with the async copy engine. Fast paths + verify + sparse + chunk-store + journal are auto-bypassed because byte-level invariants don't hold against a transformed destination.
- **Decryption is symmetric**: `copythat_crypt::decrypted_reader` accepts a passphrase, X25519 secret key, or SSH private key — same `Identity` bag, same age-format input. The runner's auto-decrypt-on-copy-FROM-`.age` flow lands in a Phase 35 follow-up.
- **Settings → Transfer → Encryption + Compression** carries the mode pickers, recipients-file path, and the level slider. The passphrase modal flow (collect at copy-start, hold in `secrecy::SecretString` for the duration of the run) is deferred to a Phase 35 follow-up — today the runner falls back to plain copy when the user selects `passphrase` mode and logs the reason.

### Mobile companion (Phase 37)

- **PWA over PeerJS WebRTC.** The phone-side app at [`apps/copythat-mobile/`](apps/copythat-mobile/) is a Vite + Svelte 5 Progressive Web App. Users open the deployed URL in their phone browser, scan the desktop's pairing QR with the system camera, and tap "Add to Home Screen" — the manifest reuses the desktop's icon so the home-screen launcher matches the desktop tray. **No App Store gate.**
- **Pairing protocol.** Settings → Mobile shows a `cthat-pair://<peer-id>?sas=<base32-256-bit>` QR. The PWA scans it, opens a PeerJS data channel against the desktop's stable peer-id, and both sides derive the same four-emoji SAS short-authentication-string from `SHA-256(seed ‖ desktop_pubkey ‖ phone_pubkey)`. The user confirms the four glyphs match; pairing commits.
- **Always-on availability.** "Always connect to Mobile App" toggle (Settings → Mobile) registers the desktop's persisted peer-id with the PeerJS broker on every launch, so paired phones can connect anytime the desktop is running. Auto-connect requires at least one paired device — the runner shows the install QR + "pair a phone first" prompt instead of registering when no pairing exists.
- **Full remote control surface.** The PWA drives every active job (pause / resume / cancel), resolves collisions (overwrite / overwrite-all / skip / skip-all / rename / keep-both), kicks off saved profiles + history reruns, fires secure-delete jobs, and shows live globals (percentage to two decimals, files-done / files-total, rate, per-operation tally) plus a streaming "files being processed" panel that scrolls individual filenames as the desktop walks the tree.
- **Loading-state lock.** When the desktop is enumerating files (after a Re-run / Start Copy / etc.), all PWA control buttons disable until the desktop emits `JobReady` so the user can't fire conflicting commands during the load phase.
- **Keep desktop awake.** PWA toggle that asks the desktop to inhibit screensaver / sleep while the connection is live — `SetThreadExecutionState` on Windows, `IOPMAssertionCreateWithName` on macOS, `org.freedesktop.ScreenSaver.Inhibit` on Linux. The OS-level wake-lock glue lives in `copythat-platform` (Phase 37 follow-up); the IPC + setting are wired today.
- **Synchronized state.** Pause / cancel / resume that originates on the desktop UI flows back to the PWA via `JobStateChanged` streaming events; the inverse path uses standard request / `Ok` reply. Desktop exit emits `ServerShuttingDown` so the PWA shows an explicit "Desktop exited — reconnect when Copy That is running again" screen instead of a generic disconnect.
- **Push notifications.** APNs ES256 JWT + FCM RS256 JWT signers in `copythat_mobile::notify` for completion notifications when the data channel is asleep. Real provider-token signing is wired today; the runner reads the credentials out of the `MobileSettings` blob (the keychain migration lands in a Phase 37 follow-up).
- **Exit button.** Phone-side button cleanly disconnects PeerJS, sends `Goodbye`, and clears the in-memory session so a closed tab can't accidentally leak a control link.
- **Broker supply-chain audit.** The Phase 42 fix-swarm verified `peerjs@^1.5.5` is actively maintained (1.5.5 / Jun 2025 / MIT) and ruled out a replacement; the documented mitigation is to self-host the broker rather than rely on the public `0.peerjs.com` default. Full evaluation + four-candidate comparison in [`docs/PEERJS_REPLACEMENT_PLAN.md`](docs/PEERJS_REPLACEMENT_PLAN.md).

### `copythat` CLI (Phase 36)

- **`copythat <SUBCOMMAND>`** — a real command-line interface suitable for CI/CD pipelines, automation scripts, and headless servers. Top-level commands: `copy`, `move`, `sync`, `shred`, `verify`, `history`, `stack`, `remote`, `mount`, `audit`, `plan`, `apply`, `version`, `config`, and `completions`.
- **Stable JSON-Lines output** via `--json` — one tagged JSON object per line on `stdout` with twelve canonical event kinds (`job_started` / `job_progress` / `job_completed` / `job_failed` / `plan_action` / `plan_summary` / `version` / `config_value` / `verify_ok` / `verify_failed` / `info` / `error`). Every line carries a UTC `ts` field, so `jq -r 'select(.kind=="job_progress")' < log.ndjson` pipes work out of the box.
- **Nine documented exit codes** — `0` success, `1` generic error, `2` pending actions (plan), `3` collisions unresolved, `4` verify failed, `5` network unreachable, `6` permission denied, `7` disk full, `8` user canceled, `9` config invalid. Surfaced as a `#[repr(u8)]` enum so the numeric values cannot drift between releases.
- **Declarative TOML jobspec** drives `plan` (no-mutation; reports the action list and exits 2 with pending) and `apply` (runs the same plan; idempotent — re-runs on a finished tree exit 0 with zero new actions). Spec layout: `[job] kind / source / destination / verify / shape / preserve / collisions`, plus `[retry]` and an optional `[schedule]`.
- **Shell completions** for bash / zsh / fish / pwsh / elvish via `copythat completions <SHELL>`. Redirect `stdout` to the shell's per-user completion location; the CLI itself never writes files.
- **Stub commands for cross-cutting features** (sync / shred / stack / remote / mount / audit) accept the same flag surface the GUI uses so scripts written today don't break when the wiring lands in a follow-up phase. Each stub exits `1` and emits a clearly-labelled `cli-info-stub-deferred` JSON event.

### SMB compression + cloud-VM offload helper (Phase 40)

- **SMB 3.1.1 traffic compression badge** — when the destination is a UNC path on Windows, Copy That requests `COPY_FILE_REQUEST_COMPRESSED_TRAFFIC` on every `CopyFileExW` / `CopyFile2` invocation and surfaces a teal "🗜 SMB compress: \<algo\>" pill in the header bar while the job runs. The flag is a *request* to the kernel — the SMB 3.1.1 server independently decides per-share whether to compress, and Windows does not surface the negotiated chained-compression algorithm (`XPRESS_LZ77` / `XPRESS_HUFFMAN` / `LZNT1`) to user mode, so today's badge labels a successful flag-pass rather than a confirmed compressed wire. Free win on slow remote links; ignored on local destinations.
- **Cloud-VM offload helper wizard** in Settings → Remotes lets the user pair two configured backends (e.g. an S3 source + an Azure Blob destination), pick a deployment-template format (cloud-init / AWS Terraform / Azure ARM / GCP Deployment Manager), tune the knob set (job_name / region / instance_size / iam_role / self_destruct_minutes), and render a deployment template they paste into their cloud's console (or `terraform apply`). Templates run the copy on a tiny ephemeral VM *inside* the cloud network so bytes never touch the user's laptop. **Templates never embed cloud credentials** — the deployed VM relies on IAM-role / managed-identity / service-account access the user provisions out-of-band; the renderer assumes nothing about credential plumbing. Every template ships a `shutdown -h +N` watchdog (clamped at 1 minute minimum) so a botched copy doesn't accrue cost. The Azure ARM `customData` field is base64-encoded inline so the deployment boots cloud-init correctly without a manual encode step. User-supplied label fields (job_name / region / iam_role / etc.) are sanitised against an `[A-Za-z0-9._/@:+-]` allowlist before substitution as defense-in-depth against shell / HCL / YAML / JSON metacharacters.
- **Probe API**: `smb_compression_state(dst_path)` IPC command returns `{ supported, algorithm }` for an arbitrary destination so a future Settings-level "test connection" UX can preview the badge before queueing the copy.

### Pre-flight tree-diff preview (Phase 41)

- **`compute_tree_diff(src, dst, opts)`** — the engine walks the source tree, stat's the destination side per file, and classifies every pair into Addition / Replacement / Skip / Conflict / Unchanged. Mtime compare uses a 1-second cross-FS rounding tolerance so two files differing only by NTFS-vs-ext4 timestamp granularity classify as Same rather than spurious replacements.
- **TreeDiffPreview modal** in the UI shows the rolled-up plan before the engine starts work: counts per category, bytes-to-transfer running total, expandable per-row lists in their respective category colours (green=add, yellow=replace, grey=skip, red=conflict, dim=unchanged). The Run button is gated on `hasBlockingConflicts` so the operator must either reduce the plan or resolve conflicts before the run can start — no silent data loss.
- **Three classifications surface non-obvious cases**: `ForceOverwriteOlder` (orange row — user opted into "always overwrite" so we'll replace even when the destination's mtime is newer), `KindMismatch` (file-vs-directory clash that the engine refuses to resolve silently), and `Ambiguous` (sizes + mtimes both differ within the cross-FS tolerance, requiring operator input). The full virtualized side-by-side trees + 1.5 s post-Run morph animation land in a follow-up — the engine + IPC + locale layer is stable enough that the animation pass can ship independently.

### Performance

- **Beats every measured competitor on Windows 11 NVMe** (10 GiB · same-volume copy):

  | Tool | MiB/s | vs CopyThat CLI |
  | --- | ---: | ---: |
  | **CopyThat CLI / engine** | **2429** | — |
  | **CopyThat UI (Phase 40)** | **2198** | −9 % |
  | Robocopy | 1305 | **−46 %** |
  | FastCopy | 1006 | **−59 %** |
  | cmd copy | 940-1147 | **−53-61 %** |
  | TeraCopy | 855 | **−65 %** |

  CopyThat's UI also beats every competitor by 68-157 % on the same workload. Caveat: cross-volume copies (C→D, C→E) are bound by the destination disk's write speed and tend to tie across all tools — engine speed only matters when the disk isn't the bottleneck.

- **`PlatformFastCopyHook` everywhere** — every entry point (CLI, UI start_copy, UI rerun, CLI shell-extension enqueue) attaches the hook so reflink + `CopyFileExW` + the Phase 38 dedup ladder are the default fast paths.
- **Phase 40 named-pipe broker** — `copythat-ui --enqueue` invocations forward argv to the running first instance via named pipe in **110 ms** instead of booting a fresh ~5 second Tauri runtime per call.
- **Phase 41 cross-volume auto-engage** — `is_cross_volume(src, dst)` automatically routes large cross-volume copies through the overlapped-IOCP pipeline (Phase 42 made this **topology-aware** via `IOCTL_STORAGE_QUERY_PROPERTY`: NVMe / SATA SSD destinations get `NO_BUFFERING` on; HDD / USB / SMB stay cached).
- **Phase 42 attribute probe + sparse-aware CopyFile2** — every copy starts with a `GetFileAttributesExW` snapshot. Sparse sources on Win11 22H2+ route through a new `CopyFile2` path that engages `COPY_FILE_ENABLE_SPARSE_COPY` so unallocated zero ranges are preserved natively. OneDrive cloud-only files (`RECALL_ON_DATA_ACCESS`), reparse points, and EFS-encrypted files are surfaced for downstream policy.
- **Phase 43 `NO_BUFFERING` threshold raised to 16 GiB floor** — Phase 42's 1 GiB adaptive cap made CopyThat correctly waiting-for-durability look ~30 % slower than `cmd copy` on the bench (cmd's buffered writes return before bytes hit platter). Phase 43 changed the formula to `max(free_phys_ram, 16 GiB)` so the 1–16 GiB band stays cache-friendly and matches competitor wall-clock numbers; truly huge files still engage `COPY_FILE_NO_BUFFERING` because the page cache cannot help. Durability override per-job via `fsync_on_close = true` / `--verify <algo>` / `paranoid_verify`. Env override (`COPYTHAT_NO_BUFFERING_THRESHOLD_MB`) still wins over both.
- **Phase 43 `--quiet` perf mode** — passing `--quiet` to the CLI flips `CopyOptions::disable_progress_callback`; the platform layer then passes `NULL` for `CopyFileExW`'s `lpProgressRoutine` (and equivalent on `CopyFile2`), eliminating the per-kernel-chunk thread-boundary crossing. Measurable on multi-GiB copies. Caveat: in-flight cancel mid-copy is suppressed (Ctrl-C still kills the process).
- **Phase 43 NTFS reflink-probe skip** — destination FS probed once via `GetVolumeInformationW` (replacing a 100-300 ms `Get-Volume` PowerShell shell-out per copy with a 1-2 µs Win32 call); pure-NTFS dests skip the `try_reflink` round-trip entirely (no reflink syscall to attempt on NTFS).
- **Phase 43 single-threaded tokio CLI runtime** — switched from `multi_thread(2)` to `current_thread`. Heavy work runs on `spawn_blocking` either way; the second worker was pure startup cost (~5-10 ms) on the small-file copies where startup dominates wall time.
- **Phase 42 SMB compressed-traffic flag** — `COPY_FILE_REQUEST_COMPRESSED_TRAFFIC` auto-OR'd onto UNC-path destinations. Free win on slow remote links via SMB v3.1.1 traffic compression negotiation.
- **Phase 42 paranoid verify mode** — `CopyOptions::paranoid_verify` (off by default) drops the destination's page-cache pages before re-reading for hash compare. The only verify mode that catches write-cache lying / silent disk corruption / FS-driver write-path bugs.
- **Phase 42 configurable retry knobs** — `sharing_violation_retries` and `sharing_violation_base_delay_ms` (defaults `3` / `50` ms) match Robocopy `/R:n /W:s` parity. Was a hard-coded 3 × 50 ms.
- **Phase 42 hardlink scaffolding** — `HardlinkSet` data structure + native `CreateHardLinkW` / `std::fs::hard_link` plumbing. Library consumers can preserve hardlink sets today; engine tree-walk integration is Phase 43.
- **Phase 42 OpenZFS 2.2.x corruption warning** — one-shot stderr warning (suppressible via `COPYTHAT_SUPPRESS_ZFS_WARNING=1`) if the host runs OpenZFS 2.2.0-2.2.6 with `zfs_bclone_enabled=1` (openzfs/zfs#15526 data-corruption bug). Reflink path stays active.
- **Phase 42 fix-swarm** — 15 parallel agents addressed every CRITICAL/HIGH/MEDIUM finding from a 10-agent review-swarm pass: HMAC-authenticated named-pipe broker, mobile pairing nonce challenge-response, IOCP loop generation-counter + cancel-drain hardening, CopyFile2 HRESULT facility-7 check, EncryptionSink explicit-finish enforcement, cloud PUT atomicity, audit chain-hash de-circularization, and ~75 more.
- **1 MiB** is the measured optimum buffer size on the default `CopyFileExW` path; all other sizes regressed in the Phase 13b sweep — see [`docs/BENCHMARKS.md`](docs/BENCHMARKS.md).
- **Head-to-head methodology + per-scenario numbers** live in [`COMPETITOR-TEST.md`](COMPETITOR-TEST.md) at the repo root (256 MiB + 10 GiB workloads across same-volume, cross-NTFS, external-SSD destinations).
- **Cross-volume reflink guard** avoids a pointless syscall on copies that can't possibly reflink (different volume IDs). Phase 42 added the `Win11 24H2 + ReFS` skip — on those targets `CopyFileExW` itself fires the block-clone IOCTL natively.
- **Criterion benches** live at `crates/copythat-core/benches/copy_bench.rs`: `single_huge_file`, `buffer_size_sweep`, `many_small_files` (10 KiB / 100 KiB / 1 MiB / 10 MiB mix), `mixed_tree` (10 KiB → 250 MiB).
- **Power-user env-var tunables** documented in [`docs/PERFORMANCE_TUNING.md`](docs/PERFORMANCE_TUNING.md): `COPYTHAT_PARALLEL_CHUNKS`, `COPYTHAT_OVERLAPPED_IO`, `COPYTHAT_OVERLAPPED_BUFFER_KB`, `COPYTHAT_OVERLAPPED_SLOTS`, `COPYTHAT_OVERLAPPED_NO_BUFFERING`, `COPYTHAT_NO_BUFFERING_THRESHOLD_MB`, `COPYTHAT_SKIP_ZERO_FILL` (admin-only), `COPYTHAT_DISABLE_AUTO_OVERLAPPED`.
- **Research underpinnings** — every default backed by data: [`docs/RESEARCH_PHASE_39.md`](docs/RESEARCH_PHASE_39.md) (Win32 + NTFS internals + IoRing + DirectStorage + scatter/gather), [`docs/RESEARCH_PHASE_40.md`](docs/RESEARCH_PHASE_40.md) (UI-bypass + Win32-skip evaluation), [`docs/RESEARCH_PHASE_42.md`](docs/RESEARCH_PHASE_42.md) (270-source swarm deep dive + 21-item gap audit; the basis for the Phase 42 work above).

## Targets

- Windows 11+ (build 22000+) — Win10 dropped in Phase 42 (Microsoft EOL October 2025)
- macOS 12+ (Monterey and later)
- Linux (Ubuntu 22.04+, Fedora 38+, Arch, …)

## Stack

| Concern        | Choice                                  |
| -------------- | --------------------------------------- |
| Language       | Rust (stable, edition 2024, MSRV 1.85)  |
| Async runtime  | `tokio` (added Phase 1)                 |
| GUI shell      | Tauri 2.x + Svelte 5 + TypeScript + Vite |
| Verify hashes  | CRC32 / MD5 / SHA-1/256/512 / xxHash3 / BLAKE3 |
| Persistence    | `rusqlite` (bundled SQLite)             |
| i18n           | Fluent (`.ftl`), 18 locales             |
| Packaging      | `tauri bundle` (MSI / NSIS / DMG / AppImage / deb / rpm) |
| License        | All Rights Reserved (proprietary; source-visible) |

Every dependency is permissively licensed. `cargo deny check` runs in CI and
fails the build if any transitive dependency falls outside the allowlist
(MIT / Apache-2.0 / BSD-2/3-Clause / ISC / CC0 / Unlicense /
Unicode-DFS-2016 / Zlib / MPL-2.0).

## Repository layout

```
CopyThat2026/
├── crates/
│   ├── copythat-core/           # async copy engine + streaming tree walk
│   ├── copythat-hash/           # verify hashes
│   ├── copythat-secure-delete/  # multi-pass shredding
│   ├── copythat-history/        # SQLite history
│   ├── copythat-platform/       # OS fast paths + shell hooks
│   ├── copythat-settings/       # TOML settings + JSON profile store
│   └── copythat-i18n/           # Fluent loader
├── apps/copythat-ui/            # Tauri 2.x + Svelte 5 shell
├── xtask/                       # workspace automation + benches
├── locales/<code>/copythat.ftl  # 18 Fluent locale files
├── tests/smoke/                 # per-phase smoke tests
└── docs/                        # architecture, changelog, benchmarks, roadmap
```

## Building

Prerequisites:

- Rust toolchain (stable, ≥ 1.85). Install with [rustup](https://rustup.rs).
- Node.js 20+ and `pnpm` 9+. Install pnpm with `npm i -g pnpm` or via
  [`corepack`](https://nodejs.org/api/corepack.html).
- Platform Tauri prerequisites:
  [docs.tauri.app/start/prerequisites/](https://v2.tauri.app/start/prerequisites/).

Workspace build:

```sh
cargo build --all
```

Tauri debug build:

```sh
cd apps/copythat-ui
pnpm install
pnpm tauri build --debug
```

Release build (no installer bundle):

```sh
cd apps/copythat-ui
pnpm tauri build --no-bundle
```

Lint Fluent key parity across all 18 locales:

```sh
cargo run -p xtask -- i18n-lint
```

Run Criterion benches:

```sh
# Full-size run; several minutes on a normal SSD
cargo bench -p copythat-core --bench copy_bench

# CI-scaled run, finishes in ~60 s
cargo run -p xtask --release -- bench-ci

# Head-to-head against any competitor copiers on PATH
cargo run -p xtask --release -- bench-vs
```

Per-phase smoke tests (run them individually):

```sh
# Phase 0 — scaffold
bash tests/smoke/phase_00_scaffold.sh

# Phase 1 — 100 MiB async round-trip through copy_file
cargo test -p copythat-core --test phase_01_core_copy -- --nocapture

# Phase 2 — 500-file tree copy + move
cargo test -p copythat-core --test phase_02_tree_queue -- --nocapture

# Phase 3 — 500 MiB copy+verify across all 8 hash algorithms
cargo test -p copythat-hash --test phase_03_verify --release -- --nocapture

# Phase 4 — 10 MiB shred across every ShredMethod
cargo test -p copythat-secure-delete --test phase_04_shred -- --nocapture

# Phase 5 — Tauri shell end-to-end
pwsh tests/smoke/phase_05_ui.ps1     # Windows
bash tests/smoke/phase_05_ui.sh      # macOS / Linux

# Phase 6 — sparse-file round-trip through the platform fast paths
cargo test -p copythat-platform --test phase_06_fast_paths -- --nocapture

# Phase 13 — throughput floor (20 MiB/s on a 100 MiB single-file copy)
cargo test -p copythat-ui --test phase_13_bench -- --nocapture

# Phase 16 — free-first packaging tripwire (no paid signing, free runners only)
cargo test -p copythat-ui --test phase_16_package -- --nocapture

# Phase 17a — red-team `..` traversal rejection at every trust boundary
cargo test -p copythat-core --test phase_17_security -- --nocapture

# Phase 18 — end-to-end: seed → copy_tree(BLAKE3) → history → CSV → shred_tree
cargo test -p copythat-ui --test phase_18_e2e -- --nocapture
# Scale up the phase 18 run to the literal Phase 18 brief (10 000 files):
COPYTHAT_PHASE18_FULL=1 cargo test -p copythat-ui --test phase_18_e2e --release

# Phase 23 — 100 MiB sparse copy: byte-exact + dst allocated <= 8 MiB
cargo test -p copythat-platform --test phase_23_sparse -- --nocapture
```

## Installing

Installable artifacts for every supported platform are produced by a
tag-triggered workflow (`.github/workflows/release.yml`) that runs
exclusively on free GitHub-hosted runners. Tag a commit with `v*.*.*`
and the release job uploads:

| Platform | Artifact                    | Install command / channel                                    |
| -------- | --------------------------- | ------------------------------------------------------------ |
| Windows  | MSI + NSIS (`.exe`)          | Double-click, or `winget install CopyThat.CopyThat` (after the winget-pkgs PR merges) |
| macOS    | `.app` + `.dmg` (x64 + arm64)| Right-click → Open on first launch, or `brew install --cask copythat` |
| Linux    | AppImage + `.deb` + `.rpm`   | `sudo apt install ./copythat-ui_<ver>_amd64.deb`, or Flathub / AUR |

Everything above ships **without paying for code-signing**:

- Windows installers are unsigned. SmartScreen shows a one-time
  warning for the first ~100 downloads, then Microsoft's reputation
  service clears the binary. Winget's SHA-256 verification gives a
  tamper-proof install path in the meantime.
- macOS bundles are **ad-hoc signed** (`codesign -s -`). Gatekeeper
  asks you to right-click → Open on first launch; subsequent launches
  are silent. Homebrew cask auto-clears the quarantine flag.
- Linux artifacts are GPG-signed only if the maintainers have set the
  optional `GPG_SIGNING_KEY` repository secret; otherwise they ship
  unsigned and distros verify via the AppImage hash + apt/rpm trust.

The free-first guarantee is enforced by
[`tests/smoke/phase_16_package.rs`](tests/smoke/phase_16_package.rs),
which fails the build if the release workflow ever imports a
paid-signing service outside a commented-out upgrade block. The paid
upgrade paths (Azure Trusted Signing ~$10/mo, Apple Developer $99/yr,
MiniSign updater signing) are captured in a maintainer-local
`docs/SIGNING_UPGRADE.md` planning doc; the commented-out
`windows-sign:` and `macos-notarize:` job blocks at the bottom of
`.github/workflows/release.yml` mirror the exact wiring.

Package channel manifests:

- [`packaging/windows/winget/`](packaging/windows/winget/) —
  microsoft/winget-pkgs submission triple.
- [`packaging/windows/chocolatey/`](packaging/windows/chocolatey/) —
  Chocolatey community repo.
- [`packaging/macos/homebrew-cask/`](packaging/macos/homebrew-cask/) —
  Homebrew cask Ruby definition.
- [`packaging/linux/flatpak/`](packaging/linux/flatpak/) — Flathub
  manifest + AppStream appdata.
- [`packaging/linux/aur/`](packaging/linux/aur/) — AUR `PKGBUILD`.

## License

**All Rights Reserved.** Copyright (c) 2026 Mike Weaver. See [`LICENSE`](LICENSE)
for the full terms.

This repository is publicly visible for reference and review only. No license
or permission is granted to use, copy, modify, distribute, or sell the code,
in whole or in part, without the express prior written permission of the
copyright holder. Workspace dependencies remain under their respective
permissive licenses (MIT / Apache-2.0 / BSD / ISC / CC0 / Unlicense /
Unicode-DFS / Zlib / MPL-2.0 / BSL-1.0); the `cargo deny` policy enforces
that allowlist as the dependency tree evolves.
