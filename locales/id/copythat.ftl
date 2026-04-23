app-name = Copy That v1.0.0
# MT
window-title = Copy That v1.0.0
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
# MT — Phase 8 toast messages
toast-error-resolved = Kesalahan teratasi
# MT
toast-collision-resolved = Konflik teratasi
# MT
toast-elevated-unavailable = Coba lagi dengan izin tinggi akan hadir di Fase 17 — belum tersedia
toast-clipboard-files-detected = Berkas di papan klip — tekan pintasan tempel untuk menyalin via Copy That
toast-clipboard-no-files = Tidak ada berkas di papan klip untuk ditempel
# MT
toast-error-log-exported = Log kesalahan diekspor

# MT — Error modal
error-modal-title = Sebuah transfer gagal
# MT
error-modal-retry = Coba lagi
# MT
error-modal-retry-elevated = Coba lagi dengan izin tinggi
# MT
error-modal-skip = Lewati
# MT
error-modal-skip-all-kind = Lewati semua kesalahan jenis ini
# MT
error-modal-abort = Batalkan semua
# MT
error-modal-path-label = Jalur
# MT
error-modal-code-label = Kode
error-drawer-pending-count = Kesalahan lain menunggu
error-drawer-toggle = Ciutkan atau perluas

# MT — Error-kind labels
err-not-found = File tidak ditemukan
# MT
err-permission-denied = Izin ditolak
# MT
err-disk-full = Disk tujuan penuh
# MT
err-interrupted = Operasi terputus
# MT
err-verify-failed = Verifikasi pasca-salin gagal
# MT
err-path-escape = Jalur ditolak — berisi segmen direktori induk (..) atau byte ilegal
# MT
err-io-other = Kesalahan I/O tidak dikenal

# MT — Collision modal
collision-modal-title = File sudah ada
# MT
collision-modal-overwrite = Timpa
# MT
collision-modal-overwrite-if-newer = Timpa jika lebih baru
# MT
collision-modal-skip = Lewati
# MT
collision-modal-keep-both = Simpan keduanya
# MT
collision-modal-rename = Ubah nama…
# MT
collision-modal-apply-to-all = Terapkan ke semua
# MT
collision-modal-source = Sumber
# MT
collision-modal-destination = Tujuan
# MT
collision-modal-size = Ukuran
# MT
collision-modal-modified = Diubah
# MT
collision-modal-hash-check = Hash cepat (SHA-256)
# MT
collision-modal-rename-placeholder = Nama file baru
# MT
collision-modal-confirm-rename = Ubah nama

# MT — Error log drawer
error-log-title = Log kesalahan
# MT
error-log-empty = Tidak ada kesalahan tercatat
# MT
error-log-export-csv = Ekspor CSV
# MT
error-log-export-txt = Ekspor teks
# MT
error-log-clear = Bersihkan log
# MT
error-log-col-time = Waktu
# MT
error-log-col-job = Pekerjaan
# MT
error-log-col-path = Jalur
# MT
error-log-col-code = Kode
# MT
error-log-col-message = Pesan
# MT
error-log-col-resolution = Penyelesaian

# MT — History drawer (Phase 9)
history-title = Riwayat
# MT
history-empty = Belum ada pekerjaan tercatat
# MT
history-unavailable = Riwayat salin tidak tersedia. Aplikasi tidak dapat membuka penyimpanan SQLite saat mulai.
# MT
history-filter-any = semua
# MT
history-filter-kind = Jenis
# MT
history-filter-status = Status
# MT
history-filter-text = Cari
# MT
history-refresh = Segarkan
# MT
history-export-csv = Ekspor CSV
# MT
history-purge-30 = Hapus lebih dari 30 hari
# MT
history-rerun = Jalankan ulang
# MT
history-detail-open = Detail
# MT
history-detail-title = Detail pekerjaan
# MT
history-detail-empty = Tidak ada item tercatat
# MT
history-col-date = Tanggal
# MT
history-col-kind = Jenis
# MT
history-col-src = Sumber
# MT
history-col-dst = Tujuan
# MT
history-col-files = File
# MT
history-col-size = Ukuran
# MT
history-col-status = Status
# MT
history-col-duration = Durasi
# MT
history-col-error = Kesalahan

# MT
toast-history-exported = Riwayat diekspor
# MT
toast-history-rerun-queued = Jalankan ulang masuk antrean

# MT — Totals drawer (Phase 10)
footer-totals = Total
# MT
totals-title = Total
# MT
totals-loading = Memuat total…
# MT
totals-card-bytes = Total byte disalin
# MT
totals-card-files = File
# MT
totals-card-jobs = Pekerjaan
# MT
totals-card-avg-rate = Throughput rata-rata
# MT
totals-errors = kesalahan
# MT
totals-spark-title = 30 hari terakhir
# MT
totals-kinds-title = Menurut jenis
# MT
totals-saved-title = Waktu dihemat (perkiraan)
# MT
totals-saved-note = Perkiraan dibandingkan dengan salinan referensi beban yang sama menggunakan pengelola file standar.
# MT
totals-reset = Atur ulang statistik
# MT
totals-reset-confirm = Ini menghapus semua pekerjaan dan item tersimpan. Lanjutkan?
# MT
totals-reset-confirm-yes = Ya, atur ulang
# MT
toast-totals-reset = Statistik diatur ulang

# MT — Phase 11a additions
header-language-label = Bahasa
# MT
header-language-title = Ubah bahasa

# MT
kind-copy = Salin
# MT
kind-move = Pindahkan
# MT
kind-delete = Hapus
# MT
kind-secure-delete = Hapus aman

# MT
status-running = Berjalan
# MT
status-succeeded = Berhasil
# MT
status-failed = Gagal
# MT
status-cancelled = Dibatalkan
# MT
status-ok = OK
# MT
status-skipped = Dilewati

# MT
history-search-placeholder = /jalur
# MT
toast-history-purged = { $count } pekerjaan lebih dari 30 hari dihapus

# MT
err-source-required = Setidaknya satu jalur sumber diperlukan.
# MT
err-destination-empty = Jalur tujuan kosong.
# MT
err-source-empty = Jalur sumber kosong.

# MT
duration-lt-1s = < 1 d
# MT
duration-ms = { $ms } md
# MT
duration-seconds = { $s } d
# MT
duration-minutes-seconds = { $m } mnt { $s } d
# MT
duration-hours-minutes = { $h } j { $m } mnt
# MT
duration-zero = 0 d

# MT
rate-unit-per-second = { $size }/d

# MT — Phase 11b Settings modal
settings-title = Pengaturan
# MT
settings-tab-general = Umum
# MT
settings-tab-appearance = Tampilan
# MT
settings-section-language = Bahasa
# MT
settings-phase-12-hint = Pengaturan lain (tema, default transfer, algoritma verifikasi, profil) akan hadir di Fase 12.

# MT — Phase 12 Settings window
settings-loading = Memuat pengaturan…
# MT
settings-tab-transfer = Transfer
# MT
settings-tab-shell = Shell
# MT
settings-tab-secure-delete = Hapus aman
# MT
settings-tab-advanced = Lanjutan
# MT
settings-tab-profiles = Profil

# MT
settings-section-theme = Tema
# MT
settings-theme-auto = Otomatis
# MT
settings-theme-light = Terang
# MT
settings-theme-dark = Gelap
# MT
settings-start-with-os = Jalankan saat sistem dimulai
# MT
settings-single-instance = Satu instansi berjalan
# MT
settings-minimize-to-tray = Minimalkan ke baki saat ditutup
settings-error-display-mode = Gaya peringatan kesalahan
settings-error-display-modal = Modal (memblokir aplikasi)
settings-error-display-drawer = Panel samping (tidak memblokir)
settings-error-display-mode-hint = Modal menghentikan antrean sampai Anda memutuskan. Panel samping menjaga antrean tetap berjalan dan memungkinkan Anda menangani kesalahan di sudut.
settings-paste-shortcut = Tempel berkas via pintasan global
settings-paste-shortcut-combo = Kombinasi pintasan
settings-paste-shortcut-hint = Tekan kombinasi ini di mana saja di sistem untuk menempelkan berkas yang disalin dari Explorer / Finder / Files melalui Copy That. CmdOrCtrl menjadi Cmd di macOS dan Ctrl di Windows / Linux.
settings-clipboard-watcher = Awasi papan klip untuk berkas yang disalin
settings-clipboard-watcher-hint = Menampilkan pemberitahuan saat URL berkas muncul di papan klip, mengisyaratkan Anda bisa menempel via Copy That. Memeriksa setiap 500 ms saat aktif.

# MT
settings-buffer-size = Ukuran buffer
# MT
settings-verify = Verifikasi setelah salin
# MT
settings-verify-off = Mati
# MT
settings-concurrency = Konkurensi
# MT
settings-concurrency-auto = Otomatis
# MT
settings-reflink = Reflink / jalur cepat
# MT
settings-reflink-prefer = Utamakan
# MT
settings-reflink-avoid = Hindari reflink
# MT
settings-reflink-disabled = Selalu pakai mesin async
# MT
settings-fsync-on-close = Sinkron ke disk saat ditutup (lebih lambat, lebih aman)
# MT
settings-preserve-timestamps = Pertahankan stempel waktu
# MT
settings-preserve-permissions = Pertahankan izin
# MT
settings-preserve-acls = Pertahankan ACL (Fase 14)

# MT
settings-context-menu = Aktifkan entri menu konteks shell
# MT
settings-intercept-copy = Cegat penangan salin default (Windows)
# MT
settings-intercept-copy-hint = Saat aktif, Ctrl+C / Ctrl+V di Explorer melalui Copy That. Pendaftaran di Fase 14.
# MT
settings-notify-completion = Beritahu saat pekerjaan selesai

# MT
settings-shred-method = Metode penghancuran default
# MT
settings-shred-zero = Nol (1 lintasan)
# MT
settings-shred-random = Acak (1 lintasan)
# MT
settings-shred-dod3 = DoD 5220.22-M (3 lintasan)
# MT
settings-shred-dod7 = DoD 5220.22-M (7 lintasan)
# MT
settings-shred-gutmann = Gutmann (35 lintasan)
# MT
settings-shred-nist = NIST 800-88
# MT
settings-shred-confirm-twice = Butuh konfirmasi ganda sebelum menghancurkan

# MT
settings-log-level = Level log
# MT
settings-log-off = Mati
# MT
settings-telemetry = Telemetri
# MT
settings-telemetry-never = Tidak pernah — tidak ada pengiriman data di level log manapun
# MT
settings-error-policy = Kebijakan error default
# MT
settings-error-policy-ask = Tanya
# MT
settings-error-policy-skip = Lewati
# MT
settings-error-policy-retry = Coba lagi dengan jeda
# MT
settings-error-policy-abort = Batalkan pada error pertama
# MT
settings-history-retention = Retensi riwayat (hari)
# MT
settings-history-retention-hint = 0 = simpan selamanya. Nilai lain menghapus otomatis pekerjaan lama saat mulai.
# MT
settings-database-path = Jalur basis data
# MT
settings-database-path-default = (default — direktori data OS)
# MT
settings-reset-all = Reset ke default
# MT
settings-reset-confirm = Reset semua preferensi? Profil tidak terpengaruh.

# MT
settings-profiles-hint = Simpan pengaturan saat ini di bawah nama; muat nanti untuk beralih tanpa menyentuh kontrol individual.
# MT
settings-profile-name-placeholder = Nama profil
# MT
settings-profile-save = Simpan
# MT
settings-profile-import = Impor…
# MT
settings-profile-load = Muat
# MT
settings-profile-export = Ekspor…
# MT
settings-profile-delete = Hapus
# MT
settings-profile-empty = Belum ada profil yang tersimpan.
# MT
settings-profile-import-prompt = Nama untuk profil yang diimpor:

# MT
toast-settings-reset = Pengaturan direset
# MT
toast-profile-saved = Profil disimpan
# MT
toast-profile-loaded = Profil dimuat
# MT
toast-profile-exported = Profil diekspor
# MT
toast-profile-imported = Profil diimpor

# Phase 13d — activity feed + header picker buttons
action-add-files = Tambah file
action-add-folders = Tambah folder
activity-title = Aktivitas
activity-clear = Bersihkan daftar aktivitas
activity-empty = Belum ada aktivitas file.
activity-after-done = Saat selesai:
activity-keep-open = Biarkan aplikasi terbuka
activity-close-app = Tutup aplikasi
activity-shutdown = Matikan PC
activity-logoff = Keluar
activity-sleep = Tidur

# Phase 14 — preflight free-space dialog
preflight-block-title = Ruang tidak cukup di tujuan
preflight-warn-title = Ruang sedikit di tujuan
preflight-unknown-title = Tidak dapat menentukan ruang kosong
preflight-unknown-body = Sumber terlalu besar untuk diukur cepat atau volume tujuan tidak merespons. Anda dapat melanjutkan; mesin akan menghentikan penyalinan dengan rapi jika ruang habis.
preflight-required = Dibutuhkan
preflight-free = Kosong
preflight-reserve = Cadangan
preflight-shortfall = Kekurangan
preflight-continue = Lanjutkan saja
collision-modal-overwrite-older = Timpa hanya yang lebih lama

# Phase 14e — subset picker
preflight-pick-subset = Pilih yang akan disalin…
subset-title = Pilih sumber untuk disalin
subset-subtitle = Pilihan lengkap tidak muat di tujuan. Centang yang ingin disalin; sisanya tidak disalin.
subset-loading = Mengukur ukuran…
subset-too-large = terlalu besar untuk dihitung
subset-budget = Tersedia
subset-remaining = Tersisa
subset-confirm = Salin pilihan
history-rerun-hint = Jalankan ulang salinan ini — memindai ulang setiap file di pohon sumber
history-clear-all = Hapus semua
history-clear-all-confirm = Klik lagi untuk konfirmasi
history-clear-all-hint = Menghapus setiap baris riwayat. Butuh klik kedua untuk konfirmasi.
toast-history-cleared = Riwayat dibersihkan ({ $count } baris dihapus)

# Phase 15 — source-list ordering
drop-dialog-sort-label = Urutan:
sort-custom = Khusus
sort-name-asc = Nama A → Z (file dulu)
sort-name-desc = Nama Z → A (file dulu)
sort-size-asc = Ukuran terkecil dulu (file dulu)
sort-size-desc = Ukuran terbesar dulu (file dulu)
sort-reorder = Susun ulang
sort-move-top = Pindah ke atas
sort-move-up = Naik
sort-move-down = Turun
sort-move-bottom = Pindah ke bawah
sort-name-asc-simple = Nama A → Z
sort-name-desc-simple = Nama Z → A
sort-size-asc-simple = Terkecil dulu
sort-size-desc-simple = Terbesar dulu
activity-sort-locked = Pengurutan dinonaktifkan saat penyalinan berjalan. Jeda atau tunggu hingga selesai, lalu ubah urutan.
drop-dialog-collision-label = Jika file sudah ada:
collision-policy-keep-both = Simpan keduanya (ganti nama salinan baru menjadi _2, _3, …)
collision-policy-skip = Lewati salinan baru
collision-policy-overwrite = Timpa file yang ada
collision-policy-overwrite-if-newer = Timpa hanya jika lebih baru
collision-policy-prompt = Tanyakan setiap kali
drop-dialog-busy-checking = Memeriksa ruang kosong…
drop-dialog-busy-enumerating = Menghitung file…
drop-dialog-busy-starting = Memulai penyalinan…
toast-enumeration-deferred = Pohon sumber besar — melewati daftar awal; baris akan muncul seiring mesin memprosesnya.

# Phase 14a — enumeration-time filters
# MT
settings-tab-filters = Filter
# MT
settings-filters-hint = Melewati berkas saat enumerasi sehingga mesin bahkan tidak membukanya. Sertakan hanya berlaku untuk berkas; Kecualikan juga memangkas direktori yang cocok.
# MT
settings-filters-enabled = Aktifkan filter untuk salinan pohon
# MT
settings-filters-include-globs = Glob penyertaan
# MT
settings-filters-include-globs-placeholder = **/*.txt
# MT
settings-filters-include-globs-hint = Satu glob per baris. Jika tidak kosong, berkas harus cocok dengan setidaknya satu. Direktori selalu ditelusuri.
# MT
settings-filters-exclude-globs = Glob pengecualian
# MT
settings-filters-exclude-globs-placeholder = **/node_modules
# MT
settings-filters-exclude-globs-hint = Satu glob per baris. Kecocokan memangkas seluruh subpohon untuk direktori; berkas yang cocok dilewati.
# MT
settings-filters-size-range = Rentang ukuran berkas
# MT
settings-filters-min-size-bytes = Ukuran minimum (byte, kosong = tanpa batas bawah)
# MT
settings-filters-max-size-bytes = Ukuran maksimum (byte, kosong = tanpa batas atas)
# MT
settings-filters-date-range = Rentang waktu modifikasi
# MT
settings-filters-min-mtime = Dimodifikasi pada atau setelah
# MT
settings-filters-max-mtime = Dimodifikasi pada atau sebelum
# MT
settings-filters-attributes = Atribut
# MT
settings-filters-skip-hidden = Lewati berkas / folder tersembunyi
# MT
settings-filters-skip-system = Lewati berkas sistem (hanya Windows)
# MT
settings-filters-skip-readonly = Lewati berkas baca-saja

# Phase 15 — auto-update
# MT
settings-tab-updater = Pembaruan
# MT
settings-updater-hint = Copy That memeriksa pembaruan yang ditandatangani maksimal sekali sehari. Pembaruan dipasang saat aplikasi ditutup berikutnya.
# MT
settings-updater-auto-check = Periksa pembaruan saat mulai
# MT
settings-updater-channel = Saluran rilis
# MT
settings-updater-channel-stable = Stabil
# MT
settings-updater-channel-beta = Beta (pra-rilis)
# MT
settings-updater-last-check = Pemeriksaan terakhir
# MT
settings-updater-last-never = Tidak pernah
# MT
settings-updater-check-now = Periksa pembaruan sekarang
# MT
settings-updater-checking = Memeriksa…
# MT
settings-updater-available = Pembaruan tersedia
# MT
settings-updater-up-to-date = Anda menggunakan rilis terbaru.
# MT
settings-updater-dismiss = Lewati versi ini
# MT
settings-updater-dismissed = Dilewati
# MT
toast-update-available = Versi yang lebih baru tersedia
# MT
toast-update-up-to-date = Anda sudah menggunakan versi terbaru

# Phase 19a — disk-backed file enumeration (TeraCopy-compatible scan DB)
# MT
scan-progress-title = Memindai…
# MT
scan-progress-stats = { $files } file · { $bytes } sejauh ini
# MT
scan-pause-button = Jeda pemindaian
# MT
scan-resume-button = Lanjutkan pemindaian
# MT
scan-cancel-button = Batalkan pemindaian
# MT
scan-cancel-confirm = Batalkan pemindaian dan buang kemajuan?
# MT
scan-db-header = Basis data pemindaian
# MT
scan-db-hint = Basis data pemindaian di disk untuk tugas berukuran jutaan file.
# MT
advanced-scan-hash-during = Hitung checksum selama pemindaian
# MT
advanced-scan-db-path = Lokasi basis data pemindaian
# MT
advanced-scan-retention-days = Hapus pemindaian selesai otomatis setelah (hari)
# MT
advanced-scan-max-keep = Jumlah maksimum basis data pemindaian yang disimpan

# Phase 19b — filesystem-snapshot source for locked files.
# MT
settings-on-locked = When a file is locked
# MT
settings-on-locked-ask = Ask the first time
# MT
settings-on-locked-retry = Retry briefly, then surface the error
# MT
settings-on-locked-skip = Skip the locked file
# MT
settings-on-locked-snapshot = Use a filesystem snapshot
# MT
settings-on-locked-hint = Eliminate "file in use by another process" errors. Copy That snapshots the source volume (VSS on Windows, ZFS/Btrfs on Linux, APFS on macOS) and reads from the snapshot copy.
# MT
snapshot-prompt-title = This file is in use by another process
# MT
snapshot-prompt-body = Another program has { $path } open for exclusive write. Choose how Copy That should handle this and similar files on the same volume.
# MT
snapshot-source-active = 📷 Reading from { $kind } snapshot of { $volume }
# MT
snapshot-create-failed = Could not create a snapshot of the source volume
# MT
snapshot-vss-needs-elevation = Reading from a VSS snapshot requires Administrator permission. Copy That will ask you to allow it.
# MT
snapshot-cleanup-failed = The snapshot helper reported a cleanup failure — a leftover shadow copy may remain on the volume.

# Phase 20 — durable resume journal.
# MT
resume-prompt-title = Resume previous transfers?
# MT
resume-prompt-body = Copy That detected { $count } unfinished transfer(s) from a previous session. Choose what to do with each.
# MT
resume-prompt-resume = Resume
# MT
resume-prompt-resume-all = Resume all
# MT
resume-discard-one = Don't resume
# MT
resume-discard-all = Discard all
# MT
resume-aborted-hash-mismatch = The destination's first { $offset } bytes don't match the source — restarting from the beginning.
# MT
settings-auto-resume = Auto-resume interrupted jobs without prompting
# MT
settings-auto-resume-hint = Skip the resume prompt at startup and silently re-enqueue every unfinished job. Off by default.

# Phase 21 — bandwidth shaping (GCRA token bucket + schedule + auto-throttle).
# MT
settings-tab-network = Network
# MT
settings-network-hint = Cap your transfer rate to keep the rest of the network usable. Apply globally, follow a daily schedule, or react automatically to metered Wi-Fi / battery / cellular connections.
# MT
settings-network-mode = Bandwidth limit
# MT
settings-network-mode-off = Off (no limit)
# MT
settings-network-mode-fixed = Fixed value
# MT
settings-network-mode-schedule = Use schedule
# MT
settings-network-cap-mbps = Cap (MB/s)
# MT
settings-network-schedule = Schedule (rclone format)
# MT
settings-network-schedule-hint = Whitespace-separated HH:MM,rate boundaries plus optional Mon-Fri,rate day rules. Rates: 512k, 10M, 2G, off, unlimited. Example: 08:00,512k 18:00,10M Sat-Sun,unlimited.
# MT
settings-network-auto-header = Auto-throttle
# MT
settings-network-auto-metered = On metered Wi-Fi
# MT
settings-network-auto-battery = On battery
# MT
settings-network-auto-cellular = On cellular
# MT
settings-network-auto-unchanged = Don't override
# MT
settings-network-auto-pause = Pause transfers
# MT
settings-network-auto-cap = Cap to fixed value
# MT
shape-badge-paused = paused
# MT
shape-badge-tooltip = Bandwidth limit active — click to open Settings → Network
# MT
shape-badge-source-schedule = scheduled
# MT
shape-badge-source-metered = metered
# MT
shape-badge-source-battery = on battery
# MT
shape-badge-source-cellular = cellular
# MT
shape-badge-source-settings = active
# MT
shape-error-schedule-invalid = Schedule format is not valid: { $message }

# Phase 22 — aggregate conflict dialog v2 (thumbnails, per-pattern
# rules, and reusable conflict profiles). Every key below is user-
# visible text in the `ConflictBatchModal.svelte` component.
# MT
conflict-batch-title = { $count } file conflicts in { $jobname }
# MT
conflict-batch-state-pending = Pending
# MT
conflict-batch-state-resolved = Resolved
# MT
conflict-batch-action-overwrite = Overwrite
# MT
conflict-batch-action-skip = Skip
# MT
conflict-batch-action-keep-both = Keep both
# MT
conflict-batch-action-newer-wins = Newer wins
# MT
conflict-batch-action-larger-wins = Larger wins
# MT
conflict-batch-bulk-apply-selected = Apply to selected
# MT
conflict-batch-bulk-apply-extension = Apply to all of this extension
# MT
conflict-batch-bulk-apply-glob = Apply to matching glob…
# MT
conflict-batch-bulk-apply-remaining = Apply to all remaining
# MT
conflict-batch-bulk-glob-placeholder = e.g. **/*.tmp
# MT
conflict-batch-save-profile = Save these rules as profile…
# MT
conflict-batch-profile-placeholder = Profile name
# MT
conflict-batch-matched-rule = via rule '{ $rule }' → { $action }
# MT
conflict-batch-empty = All conflicts resolved
# MT
conflict-batch-source-vs-destination = Source vs. destination
# MT
conflict-batch-source-label = Source
# MT
conflict-batch-destination-label = Destination
# MT
conflict-batch-size-label = Size
# MT
conflict-batch-modified-label = Modified
# MT
conflict-batch-close = Close
# MT
conflict-batch-profile-saved = Conflict profile saved
