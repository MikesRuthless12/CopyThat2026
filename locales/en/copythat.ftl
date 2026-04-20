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
