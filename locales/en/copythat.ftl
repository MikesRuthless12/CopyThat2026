app-name = Copy That v0.19.84
window-title = Copy That v0.19.84
shred-ssd-advisory = Warning: this target lives on an SSD. Multi-pass overwrites do not reliably sanitize flash memory because wear-leveling and over-provisioning move data out from under the logical block address. For solid-state media, prefer ATA SECURE ERASE, NVMe Format with Secure Erase, or full-disk encryption with a discarded key.

# Global aggregate states (header pill)
state-idle = Idle
state-copying = Copying
state-verifying = Verifying
state-paused = Paused
state-error = Error

# Per-job states (row badge)
state-pending = Queued
state-running = Running
state-cancelled = Cancelled
state-succeeded = Done
state-failed = Failed

# Actions
action-pause = Pause
action-resume = Resume
action-cancel = Cancel
action-pause-all = Pause all jobs
action-resume-all = Resume all jobs
action-cancel-all = Cancel all jobs
action-close = Close
action-reveal = Show in folder
action-add-files = Add files
action-add-folders = Add folders

# Phase 13d — activity feed
activity-title = Activity
activity-clear = Clear activity list
activity-empty = No file activity yet.
activity-after-done = When done:
activity-keep-open = Keep app open
activity-close-app = Close app
activity-shutdown = Shut down PC
activity-logoff = Log off
activity-sleep = Sleep

# Phase 14 — preflight free-space dialog
preflight-block-title = Not enough space on the destination
preflight-warn-title = Low space on the destination
preflight-unknown-title = Couldn't determine free space
preflight-unknown-body = The source is too large to size up quickly or the destination volume didn't respond. You can continue; the engine's space guard will stop the copy cleanly if it runs out of room.
preflight-required = Required
preflight-free = Free
preflight-reserve = Reserve
preflight-shortfall = Shortfall
preflight-continue = Continue anyway
preflight-pick-subset = Pick which to copy…
collision-modal-overwrite-older = Overwrite older only

# Phase 14e — subset picker
subset-title = Pick which sources to copy
subset-subtitle = The full selection doesn't fit on the destination. Tick the items you want to copy; the rest stay behind.
subset-loading = Measuring sizes…
subset-too-large = too large to count
subset-budget = Available
subset-remaining = Remaining
subset-confirm = Copy selection
history-rerun-hint = Rerun this copy — re-scans every file in the source tree
history-clear-all = Clear all
history-clear-all-confirm = Click again to confirm
history-clear-all-hint = Delete every history row. Requires a second click to confirm.
toast-history-cleared = History cleared ({ $count } rows removed)

# Phase 15 — source-list ordering in the drop staging dialog
drop-dialog-sort-label = Order:
sort-custom = Custom
sort-name-asc = Name A → Z (files first)
sort-name-desc = Name Z → A (files first)
sort-size-asc = Size smallest first (files first)
sort-size-desc = Size largest first (files first)
sort-reorder = Reorder
sort-move-top = Move to top
sort-move-up = Move up
sort-move-down = Move down
sort-move-bottom = Move to bottom

# Phase 16 — sort preset names for the Activity list (shorter
# wording than the DropStagingDialog variants; the Activity list
# is files-only so the "(files first)" suffix isn't relevant).
sort-name-asc-simple = Name A → Z
sort-name-desc-simple = Name Z → A
sort-size-asc-simple = Size smallest first
sort-size-desc-simple = Size largest first
activity-sort-locked = Sorting is disabled while a copy is running. Pause or wait for it to finish, then change the order.

# Phase 17 — collision-policy picker in the DropStagingDialog
drop-dialog-collision-label = If a file already exists:
collision-policy-keep-both = Keep both (rename new copy to _2, _3, …)
collision-policy-skip = Skip the new copy
collision-policy-overwrite = Overwrite the existing file
collision-policy-overwrite-if-newer = Overwrite only if newer
collision-policy-prompt = Ask every time

# Phase 18 — progress labels on the DropStagingDialog Start button
drop-dialog-busy-checking = Checking free space…
drop-dialog-busy-enumerating = Counting files…
drop-dialog-busy-starting = Starting copy…
toast-enumeration-deferred = Source tree is large — skipping up-front file list; rows will appear as the engine works through them.

# Context menu (per-row right-click)
menu-pause = Pause
menu-resume = Resume
menu-cancel = Cancel
menu-remove = Remove from queue
menu-reveal-source = Show source in folder
menu-reveal-destination = Show destination in folder

# Header / toolbar
header-eta-label = Estimated time remaining
header-toolbar-label = Global controls

# Footer
footer-queued = active jobs
footer-total-bytes = in flight
footer-errors = errors
footer-history = History

# Empty state
empty-title = Drop files or folders to copy
empty-hint = Drag items onto the window. We'll ask for a destination, then queue one job per source.
empty-region-label = Job list

# Details drawer
details-drawer-label = Job details
details-source = Source
details-destination = Destination
details-state = State
details-bytes = Bytes
details-files = Files
details-speed = Speed
details-eta = ETA
details-error = Error

# Drop dialog
drop-dialog-title = Transfer dropped items
drop-dialog-subtitle = { $count } item(s) ready to transfer. Pick a destination folder to begin.
drop-dialog-mode = Operation
drop-dialog-copy = Copy
drop-dialog-move = Move
drop-dialog-pick-destination = Pick destination
drop-dialog-change-destination = Change destination
drop-dialog-start-copy = Start copying
drop-dialog-start-move = Start moving

# ETA placeholders
eta-calculating = calculating…
eta-unknown = unknown

# Toast messages
toast-job-done = Transfer completed
toast-copy-queued = Copy queued
toast-move-queued = Move queued
toast-error-resolved = Error resolved
toast-collision-resolved = Collision resolved
toast-elevated-unavailable = Elevated retry lands in Phase 17 — not available yet
toast-clipboard-files-detected = Files on clipboard — press your paste shortcut to copy via Copy That
toast-clipboard-no-files = Clipboard has no files to paste
toast-error-log-exported = Error log exported

# Error modal (Phase 8)
error-modal-title = A transfer failed
error-modal-retry = Retry
error-modal-retry-elevated = Retry with elevated permissions
error-modal-skip = Skip
error-modal-skip-all-kind = Skip all errors of this kind
error-modal-abort = Abort all
error-modal-path-label = Path
error-modal-code-label = Code
error-drawer-pending-count = More errors waiting
error-drawer-toggle = Collapse or expand

# Error-kind labels (Phase 8). Source of truth — engine maps each
# `CopyErrorKind` to one of these keys via `localized_key()`.
err-not-found = File not found
err-permission-denied = Permission denied
err-disk-full = Destination disk is full
err-interrupted = Operation interrupted
err-verify-failed = Post-copy verification failed
err-path-escape = Path rejected — contains parent-directory (..) segments or illegal bytes
err-path-invalid-encoding = Path rejected — string contains invalid UTF-8 / replacement characters
err-helper-invalid-json = Privileged helper received malformed JSON; ignoring this request
err-helper-grant-out-of-band = GrantCapabilities must be handled by the helper run-loop, not the stateless handler
err-randomness-unavailable = OS random-number generator failed; cannot mint a session id
err-sparseness-mismatch = Sparse layout could not be preserved on destination
err-io-other = Unknown I/O error

# Collision modal (Phase 8)
collision-modal-title = File already exists
collision-modal-overwrite = Overwrite
collision-modal-overwrite-if-newer = Overwrite if newer
collision-modal-skip = Skip
collision-modal-keep-both = Keep both
collision-modal-rename = Rename…
collision-modal-apply-to-all = Apply to all
collision-modal-source = Source
collision-modal-destination = Destination
collision-modal-size = Size
collision-modal-modified = Modified
collision-modal-hash-check = Quick hash (SHA-256)
collision-modal-rename-placeholder = New filename
collision-modal-confirm-rename = Rename

# Error log drawer (Phase 8)
error-log-title = Error log
error-log-empty = No errors logged
error-log-export-csv = Export CSV
error-log-export-txt = Export text
error-log-clear = Clear log
error-log-col-time = Time
error-log-col-job = Job
error-log-col-path = Path
error-log-col-code = Code
error-log-col-message = Message
error-log-col-resolution = Resolution

# History drawer (Phase 9)
history-title = History
history-empty = No jobs recorded yet
history-unavailable = Copy history isn't available. The app couldn't open the SQLite store at startup.
history-filter-any = any
history-filter-kind = Kind
history-filter-status = Status
history-filter-text = Search
history-refresh = Refresh
history-export-csv = Export CSV
history-purge-30 = Purge > 30 days
history-rerun = Re-run
history-detail-open = Details
history-detail-title = Job details
history-detail-empty = No items recorded
history-col-date = Date
history-col-kind = Kind
history-col-src = Source
history-col-dst = Destination
history-col-files = Files
history-col-size = Size
history-col-status = Status
history-col-duration = Duration
history-col-error = Error
toast-history-exported = History exported
toast-history-rerun-queued = Re-run queued

# Totals drawer (Phase 10)
footer-totals = Totals
totals-title = Totals
totals-loading = Loading totals…
totals-card-bytes = Total bytes copied
totals-card-files = Files
totals-card-jobs = Jobs
totals-card-avg-rate = Average throughput
totals-errors = errors
totals-spark-title = Last 30 days
totals-kinds-title = By kind
totals-saved-title = Time saved (estimated)
totals-saved-note = Estimated vs a baseline file-manager copy of the same workload.
totals-reset = Reset statistics
totals-reset-confirm = This deletes every stored job and item. Continue?
totals-reset-confirm-yes = Yes, reset
toast-totals-reset = Statistics reset

# Phase 11a — i18n core: surface remaining user-visible strings so
# every string on a main-window screen flows through Fluent.

# Header language switcher (temporary placement — folds into
# Settings → General in Phase 12).
header-language-label = Language
header-language-title = Change language

# Job-kind labels. History filter, Totals breakdown, and history
# rows all map wire-format `kind` strings to these labels.
kind-copy = Copy
kind-move = Move
kind-delete = Delete
kind-secure-delete = Secure delete

# History status labels. Distinct from `state-*` because the
# history wire format uses plain `running`/`succeeded`/... rather
# than the live `JobState` enum the queue exposes.
status-running = Running
status-succeeded = Succeeded
status-failed = Failed
status-cancelled = Cancelled
# Per-item status (not job-level): `ok` / `skipped` are only
# reachable from the history detail view.
status-ok = OK
status-skipped = Skipped

# History drawer: search field placeholder + purge toast.
history-search-placeholder = /path
toast-history-purged = Purged { $count } jobs older than 30 days

# User-facing command-layer validation errors. The Rust side
# returns these keys when input is missing; the toast layer looks
# them up rather than showing the raw English.
err-source-required = At least one source path is required.
err-destination-empty = Destination path is empty.
err-source-empty = Source path is empty.

# Localised duration formatting for the Totals drawer and ETA
# fields. `{ $ms }`, `{ $s }`, `{ $m }`, `{ $h }` are integer
# placeables — the formatter passes pre-computed values in.
duration-lt-1s = < 1s
duration-ms = { $ms } ms
duration-seconds = { $s }s
duration-minutes-seconds = { $m }m { $s }s
duration-hours-minutes = { $h }h { $m }m
duration-zero = 0s

# Rate unit. Appended to a formatted byte size. Some languages
# render this with a leading space ("Ko/s"); keep it translatable
# even though the SI-derived "/s" is near-universal.
rate-unit-per-second = { $size }/s

# Phase 11b — Settings modal skeleton. Phase 12 expanded this into a
# full six-tab preferences window; the `settings-phase-12-hint` key
# is retired in favour of `settings-tab-profiles` + concrete labels.
settings-title = Settings
settings-tab-general = General
settings-tab-appearance = Appearance
settings-section-language = Language
settings-phase-12-hint = More settings (theme, transfer defaults, verify algorithm, profiles) arrive in Phase 12.

# Phase 12 — full Settings window keys. Grouped by tab so the
# human-review pass can pick one section at a time. Every field
# label and every option label that the user will actually read
# has its own key; dynamic wire values (enum discriminants like
# "auto" / "prefer") stay untranslated on the wire.

settings-loading = Loading settings…
settings-tab-transfer = Transfer
settings-tab-filters = Filters
settings-tab-shell = Shell
settings-tab-secure-delete = Secure delete
settings-tab-advanced = Advanced
settings-tab-updater = Updates
settings-tab-profiles = Profiles

# General tab additions
settings-section-theme = Theme
settings-theme-auto = Auto
settings-theme-light = Light
settings-theme-dark = Dark
settings-start-with-os = Launch on system startup
settings-single-instance = Single running instance
settings-minimize-to-tray = Minimize to tray on close
settings-error-display-mode = Error prompt style
settings-error-display-modal = Modal (blocks the app)
settings-error-display-drawer = Drawer (non-blocking)
settings-error-display-mode-hint = Modal stops the queue until you decide. Drawer keeps the queue moving and lets you triage errors in the corner.
settings-paste-shortcut = Paste files via global shortcut
settings-paste-shortcut-combo = Shortcut combo
settings-paste-shortcut-hint = Press this combo anywhere on your system to paste files copied from Explorer / Finder / Files via Copy That. CmdOrCtrl resolves to Cmd on macOS, Ctrl on Windows / Linux.
settings-clipboard-watcher = Watch clipboard for copied files
settings-clipboard-watcher-hint = Show a toast when file URLs appear on the clipboard, hinting you can paste via Copy That. Polls every 500 ms while enabled.

# Transfer tab
settings-buffer-size = Buffer size
settings-verify = Verify after copy
settings-verify-off = Off
settings-concurrency = Concurrency
settings-concurrency-auto = Auto
settings-reflink = Reflink / fast paths
settings-reflink-prefer = Prefer
settings-reflink-avoid = Avoid reflink
settings-reflink-disabled = Always use async engine
settings-fsync-on-close = Sync to disk on close (slower, safer)
settings-preserve-timestamps = Preserve timestamps
settings-preserve-permissions = Preserve permissions
settings-preserve-acls = Preserve ACLs (Phase 14)
settings-preserve-sparseness = Preserve sparse files
settings-preserve-sparseness-hint = Copy only the allocated extents of sparse files (VM disks, database files) so the destination stays the same on-disk size as the source.

# Shell tab
settings-context-menu = Enable shell context menu entries
settings-intercept-copy = Intercept default copy handler (Windows)
settings-intercept-copy-hint = When on, Explorer's Ctrl+C / Ctrl+V routes through Copy That. Registration lands in Phase 14.
settings-notify-completion = Notify on job completion

# Secure delete tab
settings-shred-method = Default shred method
settings-shred-zero = Zero (1 pass)
settings-shred-random = Random (1 pass)
settings-shred-dod3 = DoD 5220.22-M (3 passes)
settings-shred-dod7 = DoD 5220.22-M (7 passes)
settings-shred-gutmann = Gutmann (35 passes)
settings-shred-nist = NIST 800-88
settings-shred-confirm-twice = Require double confirmation before shredding

# Advanced tab
settings-log-level = Log level
settings-log-off = Off
settings-telemetry = Telemetry
settings-telemetry-never = Never — no phone-home at any log level
settings-error-policy = Default error policy
settings-error-policy-ask = Ask
settings-error-policy-skip = Skip
settings-error-policy-retry = Retry with backoff
settings-error-policy-abort = Abort on first failure
settings-history-retention = History retention (days)
settings-history-retention-hint = 0 = keep forever. Any other value auto-purges older jobs on startup.
settings-database-path = Database path
settings-database-path-default = (default — OS data directory)
settings-reset-all = Reset to defaults
settings-reset-confirm = Reset every preference to its default? Profiles are unaffected.

# Profiles tab
settings-profiles-hint = Save the current settings under a name; load it later to flip back without touching individual knobs.
settings-profile-name-placeholder = Profile name
settings-profile-save = Save
settings-profile-import = Import…
settings-profile-load = Load
settings-profile-export = Export…
settings-profile-delete = Delete
settings-profile-empty = No profiles saved yet.
settings-profile-import-prompt = Name for the imported profile:

# Toasts driven by Phase 12 profile actions
toast-settings-reset = Settings reset
toast-profile-saved = Profile saved
toast-profile-loaded = Profile loaded
toast-profile-exported = Profile exported
toast-profile-imported = Profile imported

# Phase 14a — enumeration-time filters
settings-filters-hint = Skip files at enumeration time so the engine never even opens them. Includes apply to files only; excludes also prune matching directories.
settings-filters-enabled = Enable filters for tree copies
settings-filters-include-globs = Include globs
settings-filters-include-globs-placeholder = **/*.txt
settings-filters-include-globs-hint = One glob per line. When non-empty, a file must match at least one include to survive. Directories are always descended into.
settings-filters-exclude-globs = Exclude globs
settings-filters-exclude-globs-placeholder = **/node_modules
settings-filters-exclude-globs-hint = One glob per line. Matches prune the whole subtree for directories; matching files are skipped.
settings-filters-size-range = File size range
settings-filters-min-size-bytes = Minimum size (bytes, blank = no floor)
settings-filters-max-size-bytes = Maximum size (bytes, blank = no ceiling)
settings-filters-date-range = Modification time range
settings-filters-min-mtime = Modified on or after
settings-filters-max-mtime = Modified on or before
settings-filters-attributes = Attribute bits
settings-filters-skip-hidden = Skip hidden files / folders
settings-filters-skip-system = Skip system files (Windows only)
settings-filters-skip-readonly = Skip read-only files

# Phase 15 — auto-update
settings-updater-hint = Copy That checks for signed updates at most once a day. Updates install on the next app quit.
settings-updater-auto-check = Check for updates on launch
settings-updater-channel = Release channel
settings-updater-channel-stable = Stable
settings-updater-channel-beta = Beta (pre-release)
settings-updater-last-check = Last checked
settings-updater-last-never = Never
settings-updater-check-now = Check for updates now
settings-updater-checking = Checking…
settings-updater-available = Update available
settings-updater-up-to-date = You're running the latest release.
settings-updater-dismiss = Skip this version
settings-updater-dismissed = Skipped
toast-update-available = A newer version is available
toast-update-up-to-date = You're already on the latest version

# Phase 19a — disk-backed file enumeration (TeraCopy-compatible scan DB)
scan-progress-title = Scanning…
scan-progress-stats = { $files } files · { $bytes } so far
scan-pause-button = Pause scan
scan-resume-button = Resume scan
scan-cancel-button = Cancel scan
scan-cancel-confirm = Cancel scan and discard progress?
scan-db-header = Scan database
scan-db-hint = On-disk scan database for multi-million-file jobs.
advanced-scan-hash-during = Compute checksums during scan
advanced-scan-db-path = Scan database location
advanced-scan-retention-days = Auto-delete completed scans after (days)
advanced-scan-max-keep = Maximum scan databases to keep

# Phase 19b — filesystem-snapshot source for locked files.
settings-on-locked = When a file is locked
settings-on-locked-ask = Ask the first time
settings-on-locked-retry = Retry briefly, then surface the error
settings-on-locked-skip = Skip the locked file
settings-on-locked-snapshot = Use a filesystem snapshot
settings-on-locked-hint = Eliminate "file in use by another process" errors. Copy That snapshots the source volume (VSS on Windows, ZFS/Btrfs on Linux, APFS on macOS) and reads from the snapshot copy.
snapshot-prompt-title = This file is in use by another process
snapshot-prompt-body = Another program has { $path } open for exclusive write. Choose how Copy That should handle this and similar files on the same volume.
snapshot-source-active = 📷 Reading from { $kind } snapshot of { $volume }
snapshot-create-failed = Could not create a snapshot of the source volume
snapshot-vss-needs-elevation = Reading from a VSS snapshot requires Administrator permission. Copy That will ask you to allow it.
snapshot-cleanup-failed = The snapshot helper reported a cleanup failure — a leftover shadow copy may remain on the volume.

# Phase 20 — durable resume journal.
resume-prompt-title = Resume previous transfers?
resume-prompt-body = Copy That detected { $count } unfinished transfer(s) from a previous session. Choose what to do with each.
resume-prompt-resume = Resume
resume-prompt-resume-all = Resume all
resume-discard-one = Don't resume
resume-discard-all = Discard all
resume-aborted-hash-mismatch = The destination's first { $offset } bytes don't match the source — restarting from the beginning.
settings-auto-resume = Auto-resume interrupted jobs without prompting
settings-auto-resume-hint = Skip the resume prompt at startup and silently re-enqueue every unfinished job. Off by default.

# Phase 21 — bandwidth shaping (GCRA token bucket + schedule + auto-throttle).
settings-tab-network = Network
settings-network-hint = Cap your transfer rate to keep the rest of the network usable. Apply globally, follow a daily schedule, or react automatically to metered Wi-Fi / battery / cellular connections.
settings-network-mode = Bandwidth limit
settings-network-mode-off = Off (no limit)
settings-network-mode-fixed = Fixed value
settings-network-mode-schedule = Use schedule
settings-network-cap-mbps = Cap (MB/s)
settings-network-schedule = Schedule (rclone format)
settings-network-schedule-hint = Whitespace-separated HH:MM,rate boundaries plus optional Mon-Fri,rate day rules. Rates: 512k, 10M, 2G, off, unlimited. Example: 08:00,512k 18:00,10M Sat-Sun,unlimited.
settings-network-auto-header = Auto-throttle
settings-network-auto-metered = On metered Wi-Fi
settings-network-auto-battery = On battery
settings-network-auto-cellular = On cellular
settings-network-auto-unchanged = Don't override
settings-network-auto-pause = Pause transfers
settings-network-auto-cap = Cap to fixed value
shape-badge-paused = paused
shape-badge-tooltip = Bandwidth limit active — click to open Settings → Network
shape-badge-source-schedule = scheduled
shape-badge-source-metered = metered
shape-badge-source-battery = on battery
shape-badge-source-cellular = cellular
shape-badge-source-settings = active
shape-error-schedule-invalid = Schedule format is not valid: { $message }

# Phase 22 — aggregate conflict dialog v2 (thumbnails, per-pattern
# rules, and reusable conflict profiles). Every key below is user-
# visible text in the `ConflictBatchModal.svelte` component.
conflict-batch-title = { $count } file conflicts in { $jobname }
conflict-batch-state-pending = Pending
conflict-batch-state-resolved = Resolved
conflict-batch-action-overwrite = Overwrite
conflict-batch-action-skip = Skip
conflict-batch-action-keep-both = Keep both
conflict-batch-action-newer-wins = Newer wins
conflict-batch-action-larger-wins = Larger wins
conflict-batch-bulk-apply-selected = Apply to selected
conflict-batch-bulk-apply-extension = Apply to all of this extension
conflict-batch-bulk-apply-glob = Apply to matching glob…
conflict-batch-bulk-apply-remaining = Apply to all remaining
conflict-batch-bulk-glob-placeholder = e.g. **/*.tmp
conflict-batch-save-profile = Save these rules as profile…
conflict-batch-profile-placeholder = Profile name
conflict-batch-matched-rule = via rule '{ $rule }' → { $action }
conflict-batch-empty = All conflicts resolved
conflict-batch-source-vs-destination = Source vs. destination
conflict-batch-source-label = Source
conflict-batch-destination-label = Destination
conflict-batch-size-label = Size
conflict-batch-modified-label = Modified
conflict-batch-close = Close
conflict-batch-profile-saved = Conflict profile saved

# Phase 23 — sparse-file preservation. The toast fires once per
# destination volume when the filesystem can't preserve holes; the
# warning line is surfaced in the job detail drawer so the user knows
# the dst is larger on disk than the source was.
sparse-not-supported-title = Destination fills sparse files
sparse-not-supported-body = { $dst_fs } does not support sparse files. Holes in the source were written out as zeros, so the destination is larger on disk.
sparse-warning-densified = Sparse layout preserved: only allocated extents were copied.
sparse-warning-mismatch = Sparse layout mismatch — destination may be larger than expected.

# Phase 24 — security-metadata preservation. The Mark-of-the-Web
# (Zone.Identifier ADS) toggle is security-sensitive: turning it off
# lets a downloaded executable shed its SmartScreen / Office Protected
# View flag on copy, which is why the tooltip carries an explicit
# warning. AppleDouble fallback emits `._<filename>` sidecars on
# destination filesystems that can't hold the foreign metadata.
settings-preserve-security-metadata = Preserve security metadata
settings-preserve-security-metadata-hint = Capture and re-apply out-of-band metadata streams (NTFS ADS / xattrs / POSIX ACLs / SELinux contexts / Linux file capabilities / macOS resource forks) on every copy.
settings-preserve-motw = Preserve Mark-of-the-Web (downloaded-from-internet flag)
settings-preserve-motw-hint = Critical for security. SmartScreen and Office Protected View use this stream to warn about files downloaded from the internet. Disabling lets a downloaded executable shed its origin marker on copy and bypass operating-system safeguards.
settings-preserve-posix-acls = Preserve POSIX ACLs and extended attributes
settings-preserve-posix-acls-hint = Carry user.* / system.* / trusted.* xattrs and POSIX access-control lists across the copy.
settings-preserve-selinux = Preserve SELinux contexts
settings-preserve-selinux-hint = Carry the security.selinux label across the copy so daemons running under MAC policies can still access the file.
settings-preserve-resource-forks = Preserve macOS resource forks and Finder info
settings-preserve-resource-forks-hint = Carry the legacy resource fork and FinderInfo (color tags, Carbon metadata) across the copy.
settings-appledouble-fallback = Use AppleDouble sidecar on incompatible filesystems
meta-translated-to-appledouble = Foreign metadata stored in AppleDouble sidecar (._{ $ext })

# Phase 25 — two-way sync with vector-clock conflict detection.
# The drawer lists configured sync pairs; each pair runs an
# independent reconciliation round against a per-pair `.copythat-sync.db`
# state store. Concurrent edits from a common ancestor surface as
# conflicts rather than silent overwrites; the losing side's content
# is preserved as `name.sync-conflict-YYYYMMDD-HHMMSS-<host>.ext`.
footer-sync = Sync
sync-drawer-title = Two-way sync
sync-drawer-hint = Keep two folders in sync without silent overwrites. Concurrent edits surface as conflicts you can resolve.
sync-add-pair = Add pair
sync-add-cancel = Cancel
sync-refresh = Refresh
sync-add-save = Save pair
sync-add-saving = Saving…
sync-add-missing-fields = Label, left path, and right path are all required.
sync-remove-confirm = Remove this sync pair? The state database is preserved; the folders are untouched.
sync-field-label = Label
sync-field-label-placeholder = e.g. Documents ↔ NAS
sync-field-left = Left folder
sync-field-left-placeholder = Pick or paste an absolute path
sync-field-right = Right folder
sync-field-right-placeholder = Pick or paste an absolute path
sync-field-mode = Mode
sync-mode-two-way = Two-way
sync-mode-mirror-left-to-right = Mirror (left → right)
sync-mode-mirror-right-to-left = Mirror (right → left)
sync-mode-contribute-left-to-right = Contribute (left → right, no deletes)
sync-no-pairs = No sync pairs configured yet. Click "Add pair" to start.
sync-loading = Loading configured pairs…
sync-never-run = Never run
sync-running = Running
sync-run-now = Run now
sync-cancel = Cancel
sync-remove-pair = Remove
sync-view-conflicts = View conflicts ({ $count })
sync-conflicts-heading = Conflicts
sync-no-conflicts = No conflicts from the last run.
sync-winner = Winner
sync-side-left-to-right = left
sync-side-right-to-left = right
sync-conflict-kind-concurrent-write = Concurrent edit
sync-conflict-kind-delete-edit = Delete ↔ edit
sync-conflict-kind-add-add = Both sides added
sync-conflict-kind-corrupt-equal = Content diverged without a new write
sync-resolve-keep-left = Keep left
sync-resolve-keep-right = Keep right
sync-resolve-keep-both = Keep both
sync-resolve-three-way = Resolve via 3-way merge
sync-resolve-phase-53-tooltip = Interactive 3-way merge for non-text files lands in Phase 53.
sync-error-prefix = Sync error

# Phase 26 — real-time mirror watcher. "Live mirror" starts a
# filesystem watcher on the pair's left side; every debounced event
# triggers a re-sync. The watcher filters vim swap files / Office
# lock files / atomic-save temp names so one logical save becomes
# exactly one sync round.
live-mirror-start = Start live mirror
live-mirror-stop = Stop live mirror
live-mirror-watching = Watching
live-mirror-toggle-hint = Re-sync automatically on every detected filesystem change. One background thread per active pair.
watch-event-prefix = File change
watch-overflow-recovered = Watcher buffer overflowed; re-enumerating to recover

# Phase 27 — content-defined chunk store. Enables delta-resume (a
# retry only re-writes chunks that actually changed) and same-job
# dedup (files sharing content blocks store those blocks once). The
# store is disk-backed under `<data-dir>/chunks/` by default and is
# the foundation for the Phase 49–51 moonshot repository phases.
chunk-store-section = Chunk store
chunk-store-enable = Enable chunk store (delta-resume and dedup)
chunk-store-enable-hint = Splits every copied file by content (FastCDC) and stores chunks content-addressed. Retries only re-write changed chunks; files with shared content dedup automatically.
chunk-store-location = Chunk store location
chunk-store-max-size = Maximum chunk store size
chunk-store-prune = Prune chunks older than (days)
chunk-store-savings = Saved { $gib } GiB via chunk dedup
chunk-store-disk-usage = Using { $size } across { $chunks } chunks

# Phase 28 — tray-resident Drop Stack. The stack is a persistent list
# of paths gathered from multiple sources (Explorer drag, main-window
# context menu, CLI, drag onto the Drop Stack window) that the user
# can dispatch to a destination in one go.
dropstack-window-title = Drop Stack
dropstack-tray-open = Drop Stack
dropstack-empty-title = Drop Stack is empty
dropstack-empty-hint = Drag files here from Explorer or right-click a job row to add it.
dropstack-add-to-stack = Add to Drop Stack
dropstack-copy-all-to = Copy all to…
dropstack-move-all-to = Move all to…
dropstack-clear = Clear stack
dropstack-remove-row = Remove from stack
dropstack-path-missing-toast = Dropped { $path } — the file no longer exists.
dropstack-always-on-top = Keep Drop Stack always on top
dropstack-show-tray-icon = Show the Copy That tray icon
dropstack-open-on-start = Open Drop Stack automatically on app start
dropstack-count = { $count } path

# Phase 29 — spring-loaded folders + native DnD polish. The Settings
# → General tab carries the knobs; DropTarget / DestinationPicker
# Svelte components use the dropzone-* keys at runtime.
settings-dnd-heading = Drag and drop
settings-dnd-spring-load = Spring-load folders while dragging
settings-dnd-spring-delay = Spring-load delay (ms)
settings-dnd-thumbnails = Show drag thumbnails
settings-dnd-invalid-highlight = Highlight invalid drop targets
dropzone-invalid-title = Not a valid drop target
dropzone-invalid-readonly = Destination is read-only
dropzone-picker-title = Choose a destination
dropzone-picker-up = Up
dropzone-picker-path = Current path
dropzone-picker-root = Roots
dropzone-picker-use-this = Use this folder
dropzone-picker-empty = No subfolders
dropzone-picker-cancel = Cancel

# Phase 30 — cross-platform path translation. Settings → Transfer
# exposes these under a "Cross-platform compatibility" subsection.
translate-heading = Cross-platform compatibility
translate-unicode-label = Unicode normalization
translate-unicode-auto = Auto-detect destination
translate-unicode-windows = NFC (Windows / Linux)
translate-unicode-macos = Leave as-is (macOS / APFS)
translate-line-endings-label = Translate line endings for text files
translate-line-endings-allowlist = Text file extensions
reserved-name-label = Windows reserved-name handling
reserved-name-suffix = Append "_" (CON.txt → CON_.txt)
reserved-name-reject = Reject and warn
long-path-label = Use Windows long-path prefix (\\?\) when over 260 chars
long-path-hint = Some network shares and legacy tools don't honor the \\?\ namespace.

# Phase 31 — power-aware copying. Settings → Power & State tab + the
# header badge that renders "⏸ Paused — Zoom call detected" when the
# runner's power subscriber has paused or capped due to a policy match.
power-heading = Power & State
power-enabled = Enable power-aware rules
power-battery-label = On battery
power-metered-label = On metered Wi-Fi
power-cellular-label = On cellular
power-presentation-label = When presenting (Zoom / Teams / Keynote)
power-fullscreen-label = When an app is fullscreen
power-thermal-label = When CPU is thermal-throttling
power-rule-continue = Continue at full speed
power-rule-pause = Pause all jobs
power-rule-cap = Cap bandwidth
power-rule-cap-percent = Cap to a percent of current rate
power-reason-on-battery = on battery
power-reason-metered-network = metered network
power-reason-cellular-network = cellular network
power-reason-presenting = presentation mode
power-reason-fullscreen = fullscreen app
power-reason-thermal-throttling = CPU is throttling

# Phase 32 — cloud backend matrix via OpenDAL. Settings → Remotes
# tab + the Add-backend wizard that writes one entry per remote into
# the keychain-backed credential store.
remote-heading = Remote backends
remote-add = Add backend
remote-list-empty = No remote backends configured
remote-test = Test connection
remote-test-success = Connection successful
remote-test-failed = Connection failed
remote-remove = Remove backend
remote-name-label = Display name
remote-kind-label = Backend type
remote-save = Save backend
remote-cancel = Cancel
backend-s3 = Amazon S3
backend-r2 = Cloudflare R2
backend-b2 = Backblaze B2
backend-azure-blob = Azure Blob Storage
backend-gcs = Google Cloud Storage
backend-onedrive = OneDrive
backend-google-drive = Google Drive
backend-dropbox = Dropbox
backend-webdav = WebDAV
backend-sftp = SFTP
backend-ftp = FTP
backend-local-fs = Local filesystem
cloud-config-bucket = Bucket
cloud-config-region = Region
cloud-config-endpoint = Endpoint URL
cloud-config-root = Root path
cloud-error-invalid-config = Backend configuration is invalid
cloud-error-network = Network error contacting backend
cloud-error-not-found = Object not found at the requested path
cloud-error-permission = Permission denied by remote backend
cloud-error-keychain = OS keychain access failed
settings-tab-remotes = Remotes
settings-tab-mobile = Mobile

# Phase 33 — mount Copy That's chunk store + history archive as a
# read-only filesystem (FUSE / WinFsp). Surfaced by the History tab
# context menu's "Mount snapshot" action + the Settings → Advanced
# "Mount latest on launch" toggle.
mount-heading = Mount snapshot
mount-action-mount = Mount snapshot
mount-action-unmount = Unmount
mount-status-mounted = Mounted at { $path }
mount-error-unsafe-mountpoint = Mountpoint path is unsafe
mount-error-mountpoint-not-empty = Mountpoint must be an empty directory
mount-error-backend-unavailable = Mount backend is not available on this system
mount-error-archive-read = Archive read failed
mount-picker-title = Pick mountpoint directory
mount-toast-mounted = Snapshot mounted at { $path }
mount-toast-unmounted = Snapshot unmounted
mount-toast-failed = Mount failed: { $reason }
settings-mount-heading = Mount snapshots
settings-mount-hint = Expose the history archive as a read-only filesystem. Phase 33b wires the runner flow; the kernel FUSE/WinFsp backends land in Phase 33c.
settings-mount-on-launch = Mount the latest snapshot on launch
settings-mount-on-launch-path = Mountpoint path
settings-mount-on-launch-path-placeholder = e.g. C:\Mounts\copythat

# Phase 34 — enterprise-grade audit log export + WORM (write-once-read-
# many) tamper-resistance. Settings → Advanced → Audit log collects the
# format + destination + rotation + WORM toggle; runtime records job and
# file events into the tamper-evident chain-hashed sink.
settings-audit-heading = Audit log
settings-audit-hint = Append-only tamper-evident log of every job and file event. Formats include CSV, JSON-lines, RFC 5424 Syslog, ArcSight CEF, and QRadar LEEF.
settings-audit-enable = Enable audit logging
settings-audit-format = Log format
settings-audit-format-json-lines = JSON lines (recommended default)
settings-audit-format-csv = CSV (spreadsheet-friendly)
settings-audit-format-syslog = Syslog (RFC 5424)
settings-audit-format-cef = CEF (ArcSight)
settings-audit-format-leef = LEEF 2.0 (IBM QRadar)
settings-audit-file-path = Log file path
settings-audit-file-path-placeholder = e.g. C:\ProgramData\CopyThat\audit.log
settings-audit-max-size = Rotate after (bytes, 0 = never)
settings-audit-worm = Enable WORM mode (write-once-read-many)
settings-audit-worm-hint = Applies the platform's append-only flag (Linux chattr +a, macOS chflags uappnd, Windows read-only attribute) after every create or rotation. Even an administrator must explicitly clear the flag to truncate the log.
settings-audit-test-write = Test write
settings-audit-verify-chain = Verify chain
toast-audit-test-write-ok = Audit log test write succeeded
toast-audit-verify-ok = Audit chain verified intact
toast-audit-verify-failed = Audit chain verification reported mismatches

# Phase 35 — destination encryption via age (X25519 / passphrase /
# SSH) + on-the-fly zstd compression with a per-extension deny
# heuristic. Surfaced through Settings → Transfer → Encryption +
# Compression; engine short-circuits to the crypt pipeline when
# either stage is active.
settings-crypt-heading = Encryption & compression
settings-crypt-hint = Transform file contents before they land at the destination. Encryption uses the age format; compression uses zstd and can skip already-compressed media by extension.
settings-crypt-encryption-mode = Encryption
settings-crypt-encryption-off = Off
settings-crypt-encryption-passphrase = Passphrase (prompt at copy start)
settings-crypt-encryption-recipients = Recipient keys from file
settings-crypt-encryption-hint = Passphrases are held only in memory for the duration of the copy. Recipient files list one age1… or ssh- public key per line.
settings-crypt-recipients-file = Recipients file path
settings-crypt-recipients-file-placeholder = e.g. C:\Users\me\recipients.txt
settings-crypt-compression-mode = Compression
settings-crypt-compression-off = Off
settings-crypt-compression-always = Always
settings-crypt-compression-smart = Smart (skip already-compressed media)
settings-crypt-compression-hint = Smart mode skips jpg, mp4, zip, 7z and similar formats that don't benefit from zstd. Always mode compresses every file at the chosen level.
settings-crypt-compression-level = zstd level (1-22)
settings-crypt-compression-level-hint = Lower numbers are faster; higher numbers compress harder. Level 3 matches zstd's CLI default.
compress-footer-savings = 💾 { $original } → { $compressed } ({ $percent }% saved)
compress-savings-toast = Compressed { $percent }% ({ $bytes } saved)
crypt-toast-recipients-loaded = Loaded { $count } encryption recipients
crypt-toast-recipients-error = Failed to load recipients: { $reason }
crypt-toast-passphrase-required = Encryption needs a passphrase before the copy starts
crypt-toast-passphrase-set = Encryption passphrase captured
crypt-footer-encrypted-badge = 🔒 Encrypted (age)
crypt-footer-compressed-badge = 📦 Compressed (zstd)

# Phase 36 — copythat CLI surface. Documented exit codes + the
# user-facing strings for plan/apply/verify/config. Help text stays
# in English (engineering accessibility) but error / info / status
# strings are localized so a localized desktop installation surfaces
# CLI errors in the same language as the GUI.
cli-help-tagline = Copy That CLI — byte-exact file copy, sync, verify and audit for CI/CD pipelines.
cli-help-exit-codes = Exit codes: 0 success, 1 error, 2 pending, 3 collision, 4 verify-fail, 5 net, 6 perm, 7 disk-full, 8 cancel, 9 config.
cli-error-bad-args = copy/move requires at least one source and a destination
cli-error-unknown-algo = Unknown verify algorithm: { $algo }
cli-error-missing-spec = --spec is required for plan/apply
cli-error-spec-parse = Failed to parse jobspec { $path }: { $reason }
cli-error-spec-empty-sources = Jobspec source list is empty
cli-info-shape-recorded = Bandwidth shape "{ $rate }" recorded; enforcement is plumbed via copythat-shape
cli-info-stub-deferred = { $command } is staged for the Phase 36 follow-up wiring
cli-plan-summary = Plan: { $actions } action(s), { $bytes } byte(s); { $already_done } already in place
cli-plan-pending = Plan reports pending actions; rerun with `apply` to execute
cli-plan-already-done = Plan reports nothing to do (idempotent)
cli-apply-success = Apply finished without errors
cli-apply-failed = Apply finished with one or more errors
cli-verify-ok = Verify ok: { $algo } { $digest }
cli-verify-failed = Verify FAILED for { $path } ({ $algo })
cli-config-set = Set { $key } = { $value }
cli-config-reset = Reset { $key } to default
cli-config-unknown-key = Unknown config key: { $key }
cli-completions-emitted = Shell completions for { $shell } printed to stdout

# Phase 37 — desktop-side mobile companion. Settings → Mobile panel
# strings + the SAS-confirmation modal + push-notification toasts.
# The actual mobile UI lives in the Phase 37 follow-up Tauri Mobile
# target; the phone displays its own localized SAS prompt.
settings-mobile-heading = Mobile companion
settings-mobile-hint = Pair an iPhone or Android phone to browse history, kick off saved profiles and Phase 36 jobspecs, and receive completion notifications.
settings-mobile-pair-toggle = Allow new pairings
settings-mobile-pair-active = Pair-server active — scan the QR with the Copy That mobile app
settings-mobile-pair-button = Start pairing
settings-mobile-revoke-button = Revoke
settings-mobile-no-pairings = No paired devices yet
settings-mobile-pair-port = Bind port (0 = pick a free one)
pair-sas-prompt = Both screens should show the same four emojis. Tap Match if they agree.
pair-sas-confirm = Match
pair-sas-reject = Mismatch — cancel
pair-toast-success = Paired with { $device }
pair-toast-failed = Pairing failed: { $reason }
push-toast-sent = Push sent to { $device }
push-toast-failed = Push to { $device } failed: { $reason }

# Phase 38 — aggregate destination dedup + reflink fallback ladder.
# Settings → Transfer → Dedup panel + per-job-row badges (⚡
# Reflinked / 🔗 Hardlinked / 🧩 Chunk-shared / 📋 Copied) +
# pre-pass dedup-scan modal.
settings-dedup-heading = Destination dedup
settings-dedup-hint = When the source and destination share a volume, Copy That can clone files at the filesystem level instead of copying bytes. Reflink is instant + safe; hardlink is faster but both names share state.
settings-dedup-mode-auto = Auto ladder (reflink → hardlink → chunk → copy)
settings-dedup-mode-reflink-only = Reflink only
settings-dedup-mode-hardlink-aggressive = Aggressive (reflink + hardlink even on writable files)
settings-dedup-mode-off = Disabled (always byte-copy)
settings-dedup-hardlink-policy = Hardlink policy
settings-dedup-prescan = Pre-scan destination tree for duplicate content
dedup-badge-reflinked = ⚡ Reflinked
dedup-badge-hardlinked = 🔗 Hardlinked
dedup-badge-chunk-shared = 🧩 Chunk-shared
dedup-badge-copied = 📋 Copied
phase42-paranoid-verify-label = Paranoid verify
phase42-paranoid-verify-hint = Drops the destination's cached pages and re-reads from disk to catch write-cache lies and silent corruption. About 50% slower than the default verify; off by default.
phase42-sharing-violation-retries-label = Retry attempts on locked source files
phase42-sharing-violation-retries-hint = How many times to retry when another process is holding the source file open with an exclusive lock. Backoff doubles each attempt (50 ms / 100 ms / 200 ms by default). Default 3, matching Robocopy /R:3.
phase42-cloud-placeholder-warning = { $name } is a cloud-only OneDrive file. Copying it will trigger a download — up to { $size } over your network connection.
phase42-defender-exclusion-hint = For maximum copy throughput, add the destination folder to Microsoft Defender exclusions before bulk transfers. See docs/PERFORMANCE_TUNING.md.

# Phase 39 — Browser-accessible recovery UI. Settings → Advanced
# exposes these strings; the recovery server itself renders askama
# templates that always read English — these eight keys are the
# Settings prose the user actually reads in their preferred locale.
settings-recovery-heading = Recovery web UI
settings-recovery-enable = Enable recovery web UI
settings-recovery-bind-address = Bind address
settings-recovery-port = Port (0 = pick a free one)
settings-recovery-show-url = Show URL & token
settings-recovery-rotate-token = Rotate token
settings-recovery-allow-non-loopback = Allow non-loopback bind
settings-recovery-non-loopback-warning = WARNING: enabling a non-loopback bind exposes the recovery UI to your local network. Anyone who learns the token can browse your file history and download files. Front it with TLS or a reverse proxy if the LAN is untrusted.

# Phase 40 — SMB compression negotiation + cloud-VM offload helper.
# 6 SMB keys (header badge + Settings prose) + 6 cloud-offload keys
# (Remotes tab wizard for cross-cloud copy templates).
smb-compress-badge = 🗜 SMB compress: { $algo }
smb-compress-badge-tooltip = Network traffic to this destination is being compressed in transit (SMB 3.1.1).
smb-compress-toast-saved = Saved { $bytes } over the network
smb-compress-algo-unknown = unknown algorithm
settings-smb-compress-heading = SMB network compression
settings-smb-compress-hint = Automatically negotiate SMB 3.1.1 traffic compression on UNC destinations. Free win on slow links; ignored on local destinations.
cloud-offload-heading = Cloud-VM offload helper
cloud-offload-hint = When copying directly between two clouds, render a deployment template that runs the copy from a tiny ephemeral VM in the cloud — bytes never touch your laptop's network.
cloud-offload-render-button = Render template
cloud-offload-copy-clipboard = Copy to clipboard
cloud-offload-template-format = Template format
cloud-offload-self-destruct-warning = The VM auto-shuts down after { $minutes } minutes — confirm IAM role + region before deploying.

# Phase 41 — animated before/after tree-diff preview. The `Preview
# changes` modal renders the rolled-up plan before the engine starts
# work; 14 keys cover the title, the summary header / counts, the
# row-reason labels, and the two action buttons.
preview-modal-title = Preview changes
preview-summary-header = What will happen
preview-category-additions = { $count } additions
preview-category-replacements = { $count } replacements
preview-category-skips = { $count } skipped
preview-category-conflicts = { $count } conflicts
preview-category-unchanged = { $count } unchanged
preview-bytes-to-transfer = { $bytes } to transfer
preview-reason-source-newer = Source is newer
preview-reason-dest-newer = Destination is newer — will skip
preview-reason-content-different = Content differs
preview-reason-identical = Identical to source
preview-button-run = Run plan
preview-button-reduce = Reduce my plan…

# Phase 42 — perceptual-hash visual-similarity dedup. Eight keys cover
# the pre-copy "looks visually identical" warning + the Settings panel.
perceptual-warn-title = Looks visually identical
perceptual-warn-body = { $name } at the destination appears to match the source picture. Continue copying anyway?
perceptual-warn-keep-both = Keep both
perceptual-warn-skip = Skip this file
perceptual-warn-overwrite = Overwrite anyway
perceptual-settings-heading = Visual-similarity dedup
perceptual-settings-hint = Detect visually identical images at the destination before they're overwritten. Hash is perceptual (recognises the same picture re-saved as a different format), not byte-exact.
perceptual-settings-threshold-label = Warn threshold (lower = stricter match)

# Phase 42 Part B — per-file rolling versions (Time Machine for any
# destination). 8 keys cover the version-list panel + retention picker.
version-list-heading = Previous versions
version-list-empty = No prior versions of this file
version-list-restore = Restore this version
version-retention-heading = Keep previous versions on overwrite
version-retention-none = Keep every version forever
version-retention-last-n = Keep last { $n } versions
version-retention-older-than-days = Drop versions older than { $days } days
version-retention-gfs = Hourly { $h } · daily { $d } · weekly { $w } · monthly { $m }

# Phase 43 — forensic chain-of-custody manifests + BLAKE3 verified
# streaming. 14 keys cover the Settings → Provenance panel
# (heading, hint, toggles, TSA URL, signing-key management) plus
# the post-job manifest notification + the verify command's two
# headline result lines.
provenance-settings-heading = Forensic chain-of-custody
provenance-settings-hint = Sign every copy job with a BLAKE3 + ed25519 manifest. Reviewers can re-hash the destination tree later and prove no byte changed since the copy.
provenance-settings-enable-default = Sign every new job by default
provenance-settings-show-after-job = Show manifest after each completed job
provenance-settings-tsa-url-label = Default RFC 3161 timestamp authority URL
provenance-settings-tsa-url-hint = Optional. When set, manifests carry a free TSA timestamp proving the bytes existed at this point in time. Leave empty to skip.
provenance-settings-keys-heading = Signing keys
provenance-settings-keys-generate = Generate new key
provenance-settings-keys-import = Import key…
provenance-settings-keys-export = Export public key…
provenance-job-completed-title = Provenance manifest saved
provenance-job-completed-body = { $count } files signed → { $path }
provenance-verify-clean = Manifest valid for { $count } files; signature { $sig }; merkle root OK.
provenance-verify-tampered = Manifest INVALID — { $tampered } tampered, { $missing } missing.
# Phase 43 post-review hardening — toast text for the Settings →
# Provenance buttons whose Tauri IPC has not yet landed.
provenance-action-staged = Phase 43 — wiring the IPC for this action lands in a follow-up commit.

# Phase 44 — SSD-aware whole-drive sanitize (NVMe Sanitize / OPAL
# Crypto Erase) + the per-file shred refusal on copy-on-write
# filesystems. 16 keys cover the new "Drive sanitize" Settings
# subsection and the localized error messages.
sanitize-heading = Whole-drive secure sanitize
sanitize-hint = NVMe Sanitize, OPAL Crypto Erase, and ATA Secure Erase wipe a flash drive at the firmware layer in milliseconds. Per-file overwrite is meaningless on flash — multi-pass shred only burns NAND. Use this for actual purge.
sanitize-pick-device = Choose the drive to sanitize
sanitize-mode-label = Sanitization method
sanitize-mode-nvme-format = NVMe Format (with secure erase)
sanitize-mode-nvme-sanitize-block = NVMe Sanitize — Block Erase (slow, every cell)
sanitize-mode-nvme-sanitize-crypto = NVMe Sanitize — Crypto Erase (instant)
sanitize-mode-ata-secure-erase = ATA Secure Erase (legacy SATA SSDs)
sanitize-mode-opal-crypto-erase = TCG OPAL Crypto Erase (Self-Encrypting Drives)
sanitize-mode-apfs-crypto-erase = APFS Crypto Erase (rotate FileVault key, macOS only)
sanitize-confirm-1 = This destroys EVERY byte on { $device }. There is no undo.
sanitize-confirm-2 = I understand that all partitions, all files, and all snapshots on { $device } will be permanently unreadable.
sanitize-confirm-3 = Type the drive's model name to proceed: { $model }
sanitize-running = Sanitizing { $device } ({ $mode }) — this can take from milliseconds (crypto erase) to tens of minutes (block erase). Do not power down.
sanitize-completed = Sanitize complete — { $device } is now blank.
ssd-honest-shred-meaningless = Per-file shred on a copy-on-write filesystem (Btrfs / ZFS / APFS) cannot reach the underlying blocks. Use whole-drive sanitize plus full-disk-encryption key rotation instead.
ssd-honest-advisory = This file lives on flash. Per-file overwrite costs NAND wear and does NOT guarantee the original cells are unrecoverable. For sensitive data, sanitize the whole drive.

# Phase 44.1f post-review — placeholder toast for SanitizeTab
# buttons whose Tauri IPC has not yet landed.
sanitize-action-staged = Phase 44.1 — wiring the IPC for this action lands in a follow-up commit.

# Phase 45.3 — named-queue tab strip (Subfeature A). Tabs surface
# once the QueueRegistry holds at least one queue; the synthesised
# default tab keeps legacy single-queue jobs reachable.
queue-tab-default = Default
queue-tab-empty-state = Job queues
queue-badge-tooltip = Pending and running jobs in this queue

# Phase 45.4 — drag-progress-merge (Subfeature B). Drag a queue tab
# onto another to merge their job lists. The default tab is neither
# draggable nor a drop target; only registry queues participate.
queue-drag-hint = Drag onto another queue to merge
queue-merge-confirm = Drop to merge
queue-merge-toast = Queues merged

# Phase 45.5 — F2-queue UX (Subfeature C). F2 toggles
# `auto_enqueue_next` so every fresh enqueue piles into the running
# queue rather than spawning a parallel one. Status pill renders in
# the Footer; pulsing dot renders on the running tab.
queue-f2-active-hint = F2 mode: every new enqueue lands in this queue
queue-f2-toggled-on = F2 queue mode ON — new enqueues join the running queue
queue-f2-toggled-off = F2 queue mode OFF — new enqueues spawn parallel queues
queue-f2-status-bar = F2 queue mode: ON

# Phase 45.6 — tray destination targets (Subfeature D). Pinned
# destinations appear in the OS tray menu; clicking one arms it as
# the active drop target so the next file drop bypasses the
# DropStagingDialog. Settings → General hosts the list editor.
tray-target-section-title = Tray destinations
tray-target-section-hint = Pinned destinations appear in the tray menu. Click one to arm it as the next drop target.
tray-target-empty = No tray destinations pinned yet.
tray-target-remove = Remove
tray-target-add-label = Label
tray-target-add-path = Path or backend URI
tray-target-add = Add
tray-target-armed-toast = Drop your next file to send it to { $label }
tray-target-active-pill = → { $label }

# Phase 45.7 follow-up — pinned-destination validation errors. The
# `queue_pin_destination` IPC returns these Fluent keys verbatim
# when input fails the IPC-boundary checks; the toast layer renders
# them via `t(...)` (Toast.svelte detects kebab-case-lowercase and
# routes through the locale table). Phase 17e's `err-path-escape` /
# `err-destination-empty` cover the path-traversal + empty cases for
# `queue_route_job`; the keys below are pin-specific.
err-pinned-destination-label-empty = Tray destination label can't be empty.
err-pinned-destination-path-empty = Tray destination path can't be empty.
err-pinned-destination-label-too-long = Tray destination label is too long (max 64 characters).
err-pinned-destination-path-too-long = Tray destination path is too long (max 1024 characters).
err-pinned-destination-label-invalid = Tray destination label contains characters that aren't allowed (newline, return, or NUL).
err-pinned-destination-path-invalid = Tray destination path contains characters that aren't allowed (newline, return, or NUL).
err-pinned-destination-too-many = You've reached the limit of 50 tray destinations. Remove one to add another.
