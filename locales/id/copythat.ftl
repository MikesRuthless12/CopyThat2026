app-name = Copy That 2026
# MT
window-title = Copy That 2026
# MT
shred-ssd-advisory = Peringatan: target ini berada pada SSD. Penimpaan berkali-kali tidak dapat membersihkan memori flash secara andal karena wear-leveling dan overprovisioning memindahkan data keluar dari alamat blok logis. Untuk media solid-state, lebih baik gunakan ATA SECURE ERASE, NVMe Format dengan Secure Erase, atau enkripsi disk penuh dengan kunci yang kemudian dibuang.

# MT
state-idle = Diam
# MT
state-copying = Menyalin
# MT
state-verifying = Memverifikasi
# MT
state-paused = Dijeda
# MT
state-error = Galat

# MT
state-pending = Dalam antrean
# MT
state-running = Berjalan
# MT
state-cancelled = Dibatalkan
# MT
state-succeeded = Selesai
# MT
state-failed = Gagal

# MT
action-pause = Jeda
# MT
action-resume = Lanjutkan
# MT
action-cancel = Batal
# MT
action-pause-all = Jeda semua pekerjaan
# MT
action-resume-all = Lanjutkan semua pekerjaan
# MT
action-cancel-all = Batalkan semua pekerjaan
# MT
action-close = Tutup
# MT
action-reveal = Tampilkan di folder

# MT
menu-pause = Jeda
# MT
menu-resume = Lanjutkan
# MT
menu-cancel = Batal
# MT
menu-remove = Hapus dari antrean
# MT
menu-reveal-source = Tampilkan sumber di folder
# MT
menu-reveal-destination = Tampilkan tujuan di folder

# MT
header-eta-label = Perkiraan sisa waktu
# MT
header-toolbar-label = Kontrol global

# MT
footer-queued = pekerjaan aktif
# MT
footer-total-bytes = sedang berjalan
# MT
footer-errors = galat
# MT
footer-history = Riwayat

# MT
empty-title = Jatuhkan berkas atau folder untuk disalin
# MT
empty-hint = Seret item ke jendela. Kami akan menanyakan tujuan, lalu menambahkan satu pekerjaan per sumber.
# MT
empty-region-label = Daftar pekerjaan

# MT
details-drawer-label = Detail pekerjaan
# MT
details-source = Sumber
# MT
details-destination = Tujuan
# MT
details-state = Status
# MT
details-bytes = Bita
# MT
details-files = Berkas
# MT
details-speed = Kecepatan
# MT
details-eta = Sisa waktu
# MT
details-error = Galat

# MT
drop-dialog-title = Transfer item yang dijatuhkan
# MT
drop-dialog-subtitle = { $count } item siap ditransfer. Pilih folder tujuan untuk mulai.
# MT
drop-dialog-mode = Operasi
# MT
drop-dialog-copy = Salin
# MT
drop-dialog-move = Pindahkan
# MT
drop-dialog-pick-destination = Pilih tujuan
# MT
drop-dialog-change-destination = Ganti tujuan
# MT
drop-dialog-start-copy = Mulai menyalin
# MT
drop-dialog-start-move = Mulai memindahkan

# MT
eta-calculating = menghitung…
# MT
eta-unknown = tidak diketahui

# MT
toast-job-done = Transfer selesai
# MT
toast-copy-queued = Salinan dalam antrean
# MT
toast-move-queued = Perpindahan dalam antrean
# Phase 8 additions — MT placeholders; review before 1.0.

# MT — Toast messages
toast-error-resolved = Error resolved
toast-collision-resolved = Collision resolved
toast-elevated-unavailable = Elevated retry lands in Phase 17 — not available yet
toast-error-log-exported = Error log exported

# MT — Error modal
error-modal-title = A transfer failed
error-modal-retry = Retry
error-modal-retry-elevated = Retry with elevated permissions
error-modal-skip = Skip
error-modal-skip-all-kind = Skip all errors of this kind
error-modal-abort = Abort all
error-modal-path-label = Path
error-modal-code-label = Code

# MT — Error-kind labels
err-not-found = File not found
err-permission-denied = Permission denied
err-disk-full = Destination disk is full
err-interrupted = Operation interrupted
err-verify-failed = Post-copy verification failed
err-io-other = Unknown I/O error

# MT — Collision modal
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

# MT — Error log drawer
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

# MT — History drawer (Phase 9)
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

# MT — Phase 9 toasts
toast-history-exported = History exported
toast-history-rerun-queued = Re-run queued
