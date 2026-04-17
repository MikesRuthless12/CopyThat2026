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
