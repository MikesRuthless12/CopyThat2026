app-name = Copy That 2026
window-title = Copy That 2026
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
