app-name = Copy That v0.19.84
# MT
window-title = Copy That v0.19.84
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
err-path-invalid-encoding = Path rejected — string contains invalid UTF-8 / replacement characters
# MT
err-helper-invalid-json = Privileged helper received malformed JSON; ignoring this request
err-helper-grant-out-of-band = GrantCapabilities must be handled by the helper run-loop, not the stateless handler
err-randomness-unavailable = OS random-number generator failed; cannot mint a session id
# MT
err-io-other = Kesalahan I/O tidak dikenal
err-sparseness-mismatch = Tata letak sparse tidak dapat dipertahankan di tujuan  # MT

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
settings-preserve-sparseness = Pertahankan file sparse  # MT
settings-preserve-sparseness-hint = Salin hanya rentang yang dialokasikan dari file sparse (disk VM, file database) sehingga ukuran tujuan di disk tetap sama dengan sumber.  # MT

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

# Phase 23 — sparse-file preservation. MT-flagged drafts; the
# authoritative English source lives in locales/en/copythat.ftl.
sparse-not-supported-title = Tujuan mengisi file sparse  # MT
sparse-not-supported-body = { $dst_fs } tidak mendukung file sparse. Lubang di sumber ditulis sebagai nol, sehingga tujuan lebih besar di disk.  # MT
sparse-warning-densified = Tata letak sparse dipertahankan: hanya rentang yang dialokasikan yang disalin.  # MT
sparse-warning-mismatch = Ketidakcocokan tata letak sparse — tujuan mungkin lebih besar dari yang diharapkan.  # MT

# Phase 24 — security-metadata preservation. MT-flagged drafts;
# the authoritative English source lives in locales/en/copythat.ftl.
settings-preserve-security-metadata = Pertahankan metadata keamanan  # MT
settings-preserve-security-metadata-hint = Tangkap dan terapkan ulang aliran metadata di luar pita (NTFS ADS / xattrs / ACL POSIX / konteks SELinux / kapabilitas file Linux / fork sumber daya macOS) pada setiap salinan.  # MT
settings-preserve-motw = Pertahankan Mark-of-the-Web (penanda diunduh-dari-internet)  # MT
settings-preserve-motw-hint = Penting untuk keamanan. SmartScreen dan Office Protected View menggunakan aliran ini untuk memperingatkan tentang file yang diunduh dari internet. Menonaktifkannya memungkinkan executable yang diunduh kehilangan penanda asalnya saat disalin dan melewati pengamanan sistem operasi.  # MT
settings-preserve-posix-acls = Pertahankan ACL POSIX dan atribut yang diperluas  # MT
settings-preserve-posix-acls-hint = Bawa xattrs user.* / system.* / trusted.* dan daftar kontrol akses POSIX selama penyalinan.  # MT
settings-preserve-selinux = Pertahankan konteks SELinux  # MT
settings-preserve-selinux-hint = Bawa label security.selinux selama penyalinan agar daemon yang berjalan di bawah kebijakan MAC tetap dapat mengakses file.  # MT
settings-preserve-resource-forks = Pertahankan fork sumber daya macOS dan info Finder  # MT
settings-preserve-resource-forks-hint = Bawa fork sumber daya warisan dan FinderInfo (tag warna, metadata Carbon) selama penyalinan.  # MT
settings-appledouble-fallback = Gunakan sidecar AppleDouble pada sistem file yang tidak kompatibel  # MT
meta-translated-to-appledouble = Metadata asing disimpan di sidecar AppleDouble (._{ $ext })  # MT

# Phase 25 — two-way sync with vector-clock conflict detection.
# MT-flagged drafts; the authoritative English source lives in
# locales/en/copythat.ftl.
footer-sync = Sinkron  # MT
sync-drawer-title = Sinkronisasi dua arah  # MT
sync-drawer-hint = Jaga dua folder tetap sinkron tanpa penimpaan diam-diam. Pengeditan bersamaan muncul sebagai konflik yang dapat Anda selesaikan.  # MT
sync-add-pair = Tambah pasangan  # MT
sync-add-cancel = Batal  # MT
sync-refresh = Segarkan  # MT
sync-add-save = Simpan pasangan  # MT
sync-add-saving = Menyimpan…  # MT
sync-add-missing-fields = Label, jalur kiri, dan jalur kanan semuanya wajib.  # MT
sync-remove-confirm = Hapus pasangan sinkron ini? Basis data status dipertahankan; folder tidak tersentuh.  # MT
sync-field-label = Label  # MT
sync-field-label-placeholder = mis. Dokumen ↔ NAS  # MT
sync-field-left = Folder kiri  # MT
sync-field-left-placeholder = Pilih atau tempel jalur absolut  # MT
sync-field-right = Folder kanan  # MT
sync-field-right-placeholder = Pilih atau tempel jalur absolut  # MT
sync-field-mode = Mode  # MT
sync-mode-two-way = Dua arah  # MT
sync-mode-mirror-left-to-right = Cermin (kiri → kanan)  # MT
sync-mode-mirror-right-to-left = Cermin (kanan → kiri)  # MT
sync-mode-contribute-left-to-right = Kontribusi (kiri → kanan, tanpa hapus)  # MT
sync-no-pairs = Belum ada pasangan sinkron yang dikonfigurasi. Klik "Tambah pasangan" untuk memulai.  # MT
sync-loading = Memuat pasangan yang dikonfigurasi…  # MT
sync-never-run = Tidak pernah dijalankan  # MT
sync-running = Berjalan  # MT
sync-run-now = Jalankan sekarang  # MT
sync-cancel = Batal  # MT
sync-remove-pair = Hapus  # MT
sync-view-conflicts = Lihat konflik ({ $count })  # MT
sync-conflicts-heading = Konflik  # MT
sync-no-conflicts = Tidak ada konflik dari jalannya terakhir.  # MT
sync-winner = Pemenang  # MT
sync-side-left-to-right = kiri  # MT
sync-side-right-to-left = kanan  # MT
sync-conflict-kind-concurrent-write = Pengeditan bersamaan  # MT
sync-conflict-kind-delete-edit = Hapus ↔ sunting  # MT
sync-conflict-kind-add-add = Kedua sisi menambahkan  # MT
sync-conflict-kind-corrupt-equal = Konten menyimpang tanpa tulisan baru  # MT
sync-resolve-keep-left = Pertahankan kiri  # MT
sync-resolve-keep-right = Pertahankan kanan  # MT
sync-resolve-keep-both = Pertahankan keduanya  # MT
sync-resolve-three-way = Selesaikan melalui penggabungan 3-arah  # MT
sync-resolve-phase-53-tooltip = Penggabungan 3-arah interaktif untuk berkas non-teks hadir di Fase 53.  # MT
sync-error-prefix = Kesalahan sinkron  # MT

# Phase 26 — real-time mirror watcher. MT-flagged drafts;
# the authoritative English source lives in locales/en/copythat.ftl.
live-mirror-start = Mulai cermin langsung  # MT
live-mirror-stop = Hentikan cermin langsung  # MT
live-mirror-watching = Mengawasi  # MT
live-mirror-toggle-hint = Sinkronisasi ulang secara otomatis pada setiap perubahan sistem berkas yang terdeteksi. Satu thread latar per pasangan aktif.  # MT
watch-event-prefix = Perubahan berkas  # MT
watch-overflow-recovered = Buffer pengawas meluap; mengenumerasi ulang untuk memulihkan  # MT

# Phase 27 — content-defined chunk store. MT-flagged drafts;
# the authoritative English source lives in locales/en/copythat.ftl.
chunk-store-section = Penyimpanan chunk  # MT
chunk-store-enable = Aktifkan penyimpanan chunk (delta-resume dan deduplikasi)  # MT
chunk-store-enable-hint = Membagi setiap berkas yang disalin berdasarkan konten (FastCDC) dan menyimpan chunk dengan pengalamatan konten. Percobaan ulang hanya menulis ulang chunk yang berubah; berkas dengan konten bersama diduplikasi secara otomatis.  # MT
chunk-store-location = Lokasi penyimpanan chunk  # MT
chunk-store-max-size = Ukuran maksimum penyimpanan chunk  # MT
chunk-store-prune = Pangkas chunk yang lebih lama dari (hari)  # MT
chunk-store-savings = Hemat { $gib } GiB melalui deduplikasi chunk  # MT
chunk-store-disk-usage = Menggunakan { $size } dalam { $chunks } chunk  # MT

# Phase 28 — tray-resident Drop Stack. MT-flagged drafts;
# the authoritative English source lives in locales/en/copythat.ftl.
dropstack-window-title = Drop Stack  # MT
dropstack-tray-open = Drop Stack  # MT
dropstack-empty-title = Drop Stack kosong  # MT
dropstack-empty-hint = Seret file ke sini dari Penjelajah atau klik kanan baris pekerjaan untuk menambahkannya.  # MT
dropstack-add-to-stack = Tambahkan ke Drop Stack  # MT
dropstack-copy-all-to = Salin semua ke…  # MT
dropstack-move-all-to = Pindahkan semua ke…  # MT
dropstack-clear = Bersihkan tumpukan  # MT
dropstack-remove-row = Hapus dari tumpukan  # MT
dropstack-path-missing-toast = { $path } dihapus — file tidak ada lagi.  # MT
dropstack-always-on-top = Selalu tampilkan Drop Stack di atas  # MT
dropstack-show-tray-icon = Tampilkan ikon baki Copy That  # MT
dropstack-open-on-start = Buka Drop Stack secara otomatis saat aplikasi mulai  # MT
dropstack-count = { $count } jalur  # MT

# Phase 29 — spring-loaded folders + native DnD polish.
settings-dnd-heading = Drag and drop  # MT
settings-dnd-spring-load = Spring-load folders while dragging  # MT
settings-dnd-spring-delay = Spring-load delay (ms)  # MT
settings-dnd-thumbnails = Show drag thumbnails  # MT
settings-dnd-invalid-highlight = Highlight invalid drop targets  # MT
dropzone-invalid-title = Not a valid drop target  # MT
dropzone-invalid-readonly = Destination is read-only  # MT
dropzone-picker-title = Choose a destination  # MT
dropzone-picker-up = Up  # MT
dropzone-picker-path = Current path  # MT
dropzone-picker-root = Roots  # MT
dropzone-picker-use-this = Use this folder  # MT
dropzone-picker-empty = No subfolders  # MT
dropzone-picker-cancel = Cancel  # MT

# Phase 30 — cross-platform path translation.
translate-heading = Cross-platform compatibility  # MT
translate-unicode-label = Unicode normalization  # MT
translate-unicode-auto = Auto-detect destination  # MT
translate-unicode-windows = NFC (Windows / Linux)  # MT
translate-unicode-macos = Leave as-is (macOS / APFS)  # MT
translate-line-endings-label = Translate line endings for text files  # MT
translate-line-endings-allowlist = Text file extensions  # MT
reserved-name-label = Windows reserved-name handling  # MT
reserved-name-suffix = Append "_" (CON.txt → CON_.txt)  # MT
reserved-name-reject = Reject and warn  # MT
long-path-label = Use Windows long-path prefix (\?\) when over 260 chars  # MT
long-path-hint = Some network shares and legacy tools don't honor the \?\ namespace.  # MT

# Phase 31 — power-aware copying.
power-heading = Power & State  # MT
power-enabled = Enable power-aware rules  # MT
power-battery-label = On battery  # MT
power-metered-label = On metered Wi-Fi  # MT
power-cellular-label = On cellular  # MT
power-presentation-label = When presenting (Zoom / Teams / Keynote)  # MT
power-fullscreen-label = When an app is fullscreen  # MT
power-thermal-label = When CPU is thermal-throttling  # MT
power-rule-continue = Continue at full speed  # MT
power-rule-pause = Pause all jobs  # MT
power-rule-cap = Cap bandwidth  # MT
power-rule-cap-percent = Cap to a percent of current rate  # MT
power-reason-on-battery = on battery  # MT
power-reason-metered-network = metered network  # MT
power-reason-cellular-network = cellular network  # MT
power-reason-presenting = presentation mode  # MT
power-reason-fullscreen = fullscreen app  # MT
power-reason-thermal-throttling = CPU is throttling  # MT

# Phase 32 — cloud backend matrix via OpenDAL.
remote-heading = Remote backends  # MT
remote-add = Add backend  # MT
remote-list-empty = No remote backends configured  # MT
remote-test = Test connection  # MT
remote-test-success = Connection successful  # MT
remote-test-failed = Connection failed  # MT
remote-remove = Remove backend  # MT
remote-name-label = Display name  # MT
remote-kind-label = Backend type  # MT
remote-save = Save backend  # MT
remote-cancel = Cancel  # MT
backend-s3 = Amazon S3  # MT
backend-r2 = Cloudflare R2  # MT
backend-b2 = Backblaze B2  # MT
backend-azure-blob = Azure Blob Storage  # MT
backend-gcs = Google Cloud Storage  # MT
backend-onedrive = OneDrive  # MT
backend-google-drive = Google Drive  # MT
backend-dropbox = Dropbox  # MT
backend-webdav = WebDAV  # MT
backend-sftp = SFTP  # MT
backend-ftp = FTP  # MT
backend-local-fs = Local filesystem  # MT
cloud-config-bucket = Bucket  # MT
cloud-config-region = Region  # MT
cloud-config-endpoint = Endpoint URL  # MT
cloud-config-root = Root path  # MT
cloud-error-invalid-config = Backend configuration is invalid  # MT
cloud-error-network = Network error contacting backend  # MT
cloud-error-not-found = Object not found at the requested path  # MT
cloud-error-permission = Permission denied by remote backend  # MT
cloud-error-keychain = OS keychain access failed  # MT
settings-tab-remotes = Remotes  # MT
settings-tab-mobile = Mobile  # MT

# Phase 33 — mount as read-only filesystem.
mount-heading = Mount snapshot  # MT
mount-action-mount = Mount snapshot  # MT
mount-action-unmount = Unmount  # MT
mount-status-mounted = Mounted at { $path }  # MT
mount-error-unsafe-mountpoint = Mountpoint path is unsafe  # MT
mount-error-mountpoint-not-empty = Mountpoint must be an empty directory  # MT
mount-error-backend-unavailable = Mount backend is not available on this system  # MT
mount-error-archive-read = Archive read failed  # MT
mount-picker-title = Pick mountpoint directory  # MT
mount-toast-mounted = Snapshot mounted at { $path }  # MT
mount-toast-unmounted = Snapshot unmounted  # MT
mount-toast-failed = Mount failed: { $reason }  # MT
settings-mount-heading = Mount snapshots  # MT
settings-mount-hint = Expose the history archive as a read-only filesystem. Phase 33b wires the runner flow; the kernel FUSE/WinFsp backends land in Phase 33c.  # MT
settings-mount-on-launch = Mount the latest snapshot on launch  # MT
settings-mount-on-launch-path = Mountpoint path  # MT
settings-mount-on-launch-path-placeholder = e.g. C:\Mounts\copythat  # MT

# Phase 34 — enterprise-grade audit log export + WORM (write-once-read-
# many) tamper-resistance. Settings → Advanced → Audit log collects the
# format + destination + rotation + WORM toggle; runtime records job and
# file events into the tamper-evident chain-hashed sink.
settings-audit-heading = Audit log  # MT
settings-audit-hint = Append-only tamper-evident log of every job and file event. Formats include CSV, JSON-lines, RFC 5424 Syslog, ArcSight CEF, and QRadar LEEF.  # MT
settings-audit-enable = Enable audit logging  # MT
settings-audit-format = Log format  # MT
settings-audit-format-json-lines = JSON lines (recommended default)  # MT
settings-audit-format-csv = CSV (spreadsheet-friendly)  # MT
settings-audit-format-syslog = Syslog (RFC 5424)  # MT
settings-audit-format-cef = CEF (ArcSight)  # MT
settings-audit-format-leef = LEEF 2.0 (IBM QRadar)  # MT
settings-audit-file-path = Log file path  # MT
settings-audit-file-path-placeholder = e.g. C:\ProgramData\CopyThat\audit.log  # MT
settings-audit-max-size = Rotate after (bytes, 0 = never)  # MT
settings-audit-worm = Enable WORM mode (write-once-read-many)  # MT
settings-audit-worm-hint = Applies the platform's append-only flag (Linux chattr +a, macOS chflags uappnd, Windows read-only attribute) after every create or rotation. Even an administrator must explicitly clear the flag to truncate the log.  # MT
settings-audit-test-write = Test write  # MT
settings-audit-verify-chain = Verify chain  # MT
toast-audit-test-write-ok = Audit log test write succeeded  # MT
toast-audit-verify-ok = Audit chain verified intact  # MT
toast-audit-verify-failed = Audit chain verification reported mismatches  # MT

# Phase 35 — destination encryption via age (X25519 / passphrase /
# SSH) + on-the-fly zstd compression with a per-extension deny
# heuristic. Surfaced through Settings → Transfer → Encryption +
# Compression; engine short-circuits to the crypt pipeline when
# either stage is active.
settings-crypt-heading = Encryption & compression  # MT
settings-crypt-hint = Transform file contents before they land at the destination. Encryption uses the age format; compression uses zstd and can skip already-compressed media by extension.  # MT
settings-crypt-encryption-mode = Encryption  # MT
settings-crypt-encryption-off = Off  # MT
settings-crypt-encryption-passphrase = Passphrase (prompt at copy start)  # MT
settings-crypt-encryption-recipients = Recipient keys from file  # MT
settings-crypt-encryption-hint = Passphrases are held only in memory for the duration of the copy. Recipient files list one age1… or ssh- public key per line.  # MT
settings-crypt-recipients-file = Recipients file path  # MT
settings-crypt-recipients-file-placeholder = e.g. C:\Users\me\recipients.txt  # MT
settings-crypt-compression-mode = Compression  # MT
settings-crypt-compression-off = Off  # MT
settings-crypt-compression-always = Always  # MT
settings-crypt-compression-smart = Smart (skip already-compressed media)  # MT
settings-crypt-compression-hint = Smart mode skips jpg, mp4, zip, 7z and similar formats that don't benefit from zstd. Always mode compresses every file at the chosen level.  # MT
settings-crypt-compression-level = zstd level (1-22)  # MT
settings-crypt-compression-level-hint = Lower numbers are faster; higher numbers compress harder. Level 3 matches zstd's CLI default.  # MT
compress-footer-savings = 💾 { $original } → { $compressed } ({ $percent }% saved)  # MT
compress-savings-toast = Compressed { $percent }% ({ $bytes } saved)  # MT
crypt-toast-recipients-loaded = Loaded { $count } encryption recipients  # MT
crypt-toast-recipients-error = Failed to load recipients: { $reason }  # MT
crypt-toast-passphrase-required = Encryption needs a passphrase before the copy starts  # MT
crypt-toast-passphrase-set = Encryption passphrase captured  # MT
crypt-footer-encrypted-badge = 🔒 Encrypted (age)  # MT
crypt-footer-compressed-badge = 📦 Compressed (zstd)  # MT

# Phase 36 — copythat CLI. MT-flagged English strings pending human
# translation; tracked in docs/I18N_TODO.md.
cli-help-tagline = Copy That CLI — byte-exact file copy, sync, verify and audit for CI/CD pipelines.  # MT
cli-help-exit-codes = Exit codes: 0 success, 1 error, 2 pending, 3 collision, 4 verify-fail, 5 net, 6 perm, 7 disk-full, 8 cancel, 9 config.  # MT
cli-error-bad-args = copy/move requires at least one source and a destination  # MT
cli-error-unknown-algo = Unknown verify algorithm: { $algo }  # MT
cli-error-missing-spec = --spec is required for plan/apply  # MT
cli-error-spec-parse = Failed to parse jobspec { $path }: { $reason }  # MT
cli-error-spec-empty-sources = Jobspec source list is empty  # MT
cli-info-shape-recorded = Bandwidth shape "{ $rate }" recorded; enforcement is plumbed via copythat-shape  # MT
cli-info-stub-deferred = { $command } is staged for the Phase 36 follow-up wiring  # MT
cli-plan-summary = Plan: { $actions } action(s), { $bytes } byte(s); { $already_done } already in place  # MT
cli-plan-pending = Plan reports pending actions; rerun with `apply` to execute  # MT
cli-plan-already-done = Plan reports nothing to do (idempotent)  # MT
cli-apply-success = Apply finished without errors  # MT
cli-apply-failed = Apply finished with one or more errors  # MT
cli-verify-ok = Verify ok: { $algo } { $digest }  # MT
cli-verify-failed = Verify FAILED for { $path } ({ $algo })  # MT
cli-config-set = Set { $key } = { $value }  # MT
cli-config-reset = Reset { $key } to default  # MT
cli-config-unknown-key = Unknown config key: { $key }  # MT
cli-completions-emitted = Shell completions for { $shell } printed to stdout  # MT

# Phase 37 — desktop-side mobile companion. MT-flagged English
# strings pending human translation; tracked in docs/I18N_TODO.md.
settings-mobile-heading = Mobile companion  # MT
settings-mobile-hint = Pair an iPhone or Android phone to browse history, kick off saved profiles and Phase 36 jobspecs, and receive completion notifications.  # MT
settings-mobile-pair-toggle = Allow new pairings  # MT
settings-mobile-pair-active = Pair-server active — scan the QR with the Copy That mobile app  # MT
settings-mobile-pair-button = Start pairing  # MT
settings-mobile-revoke-button = Revoke  # MT
settings-mobile-no-pairings = No paired devices yet  # MT
settings-mobile-pair-port = Bind port (0 = pick a free one)  # MT
pair-sas-prompt = Both screens should show the same four emojis. Tap Match if they agree.  # MT
pair-sas-confirm = Match  # MT
pair-sas-reject = Mismatch — cancel  # MT
pair-toast-success = Paired with { $device }  # MT
pair-toast-failed = Pairing failed: { $reason }  # MT
push-toast-sent = Push sent to { $device }  # MT
push-toast-failed = Push to { $device } failed: { $reason }  # MT

# Phase 38 — destination dedup + reflink ladder. MT-flagged
# English strings pending human translation; tracked in
# docs/I18N_TODO.md.
settings-dedup-heading = Destination dedup  # MT
settings-dedup-hint = When the source and destination share a volume, Copy That can clone files at the filesystem level instead of copying bytes. Reflink is instant + safe; hardlink is faster but both names share state.  # MT
settings-dedup-mode-auto = Auto ladder (reflink → hardlink → chunk → copy)  # MT
settings-dedup-mode-reflink-only = Reflink only  # MT
settings-dedup-mode-hardlink-aggressive = Aggressive (reflink + hardlink even on writable files)  # MT
settings-dedup-mode-off = Disabled (always byte-copy)  # MT
settings-dedup-hardlink-policy = Hardlink policy  # MT
settings-dedup-prescan = Pre-scan destination tree for duplicate content  # MT
dedup-badge-reflinked = ⚡ Reflinked  # MT
dedup-badge-hardlinked = 🔗 Hardlinked  # MT
dedup-badge-chunk-shared = 🧩 Chunk-shared  # MT
dedup-badge-copied = 📋 Copied  # MT
phase42-paranoid-verify-label = Verifikasi paranoid
phase42-paranoid-verify-hint = Membuang halaman cache tujuan dan membaca ulang dari disk untuk menangkap kebohongan cache tulis dan korupsi diam-diam. Sekitar 50% lebih lambat daripada verifikasi default; nonaktif secara default.
phase42-sharing-violation-retries-label = Upaya percobaan ulang pada file sumber yang terkunci
phase42-sharing-violation-retries-hint = Berapa kali mencoba ulang ketika proses lain menahan file sumber terbuka dengan kunci eksklusif. Jeda berlipat ganda pada setiap percobaan (50 md / 100 md / 200 md secara default). Default: 3, sesuai Robocopy /R:3.
phase42-cloud-placeholder-warning = { $name } adalah berkas OneDrive yang hanya tersedia di cloud. Menyalinnya akan memicu unduhan — hingga { $size } melalui koneksi jaringan Anda.
phase42-defender-exclusion-hint = Untuk throughput penyalinan maksimum, tambahkan folder tujuan ke pengecualian Microsoft Defender sebelum transfer massal. Lihat docs/PERFORMANCE_TUNING.md.

# Phase 39 — Browser-accessible recovery UI.  # MT
settings-recovery-heading = Recovery web UI  # MT
settings-recovery-enable = Enable recovery web UI  # MT
settings-recovery-bind-address = Bind address  # MT
settings-recovery-port = Port (0 = pick a free one)  # MT
settings-recovery-show-url = Show URL & token  # MT
settings-recovery-rotate-token = Rotate token  # MT
settings-recovery-allow-non-loopback = Allow non-loopback bind  # MT
settings-recovery-non-loopback-warning = WARNING: enabling a non-loopback bind exposes the recovery UI to your local network. Anyone who learns the token can browse your file history and download files. Front it with TLS or a reverse proxy if the LAN is untrusted.  # MT

# Phase 40 — SMB compression negotiation + cloud-VM offload helper.  # MT
smb-compress-badge = 🗜 SMB compress: { $algo }  # MT
smb-compress-badge-tooltip = Network traffic to this destination is being compressed in transit (SMB 3.1.1).  # MT
smb-compress-toast-saved = Saved { $bytes } over the network  # MT
smb-compress-algo-unknown = unknown algorithm  # MT
settings-smb-compress-heading = SMB network compression  # MT
settings-smb-compress-hint = Automatically negotiate SMB 3.1.1 traffic compression on UNC destinations. Free win on slow links; ignored on local destinations.  # MT
cloud-offload-heading = Cloud-VM offload helper  # MT
cloud-offload-hint = When copying directly between two clouds, render a deployment template that runs the copy from a tiny ephemeral VM in the cloud — bytes never touch your laptop's network.  # MT
cloud-offload-render-button = Render template  # MT
cloud-offload-copy-clipboard = Copy to clipboard  # MT
cloud-offload-template-format = Template format  # MT
cloud-offload-self-destruct-warning = The VM auto-shuts down after { $minutes } minutes — confirm IAM role + region before deploying.  # MT

# Phase 41 — animated before/after tree-diff preview.  # MT
preview-modal-title = Preview changes  # MT
preview-summary-header = What will happen  # MT
preview-category-additions = { $count } additions  # MT
preview-category-replacements = { $count } replacements  # MT
preview-category-skips = { $count } skipped  # MT
preview-category-conflicts = { $count } conflicts  # MT
preview-category-unchanged = { $count } unchanged  # MT
preview-bytes-to-transfer = { $bytes } to transfer  # MT
preview-reason-source-newer = Source is newer  # MT
preview-reason-dest-newer = Destination is newer — will skip  # MT
preview-reason-content-different = Content differs  # MT
preview-reason-identical = Identical to source  # MT
preview-button-run = Run plan  # MT
preview-button-reduce = Reduce my plan…  # MT

# Phase 42 — perceptual-hash visual-similarity dedup.  # MT
perceptual-warn-title = Looks visually identical  # MT
perceptual-warn-body = { $name } at the destination appears to match the source picture. Continue copying anyway?  # MT
perceptual-warn-keep-both = Keep both  # MT
perceptual-warn-skip = Skip this file  # MT
perceptual-warn-overwrite = Overwrite anyway  # MT
perceptual-settings-heading = Visual-similarity dedup  # MT
perceptual-settings-hint = Detect visually identical images at the destination before they're overwritten. Hash is perceptual (recognises the same picture re-saved as a different format), not byte-exact.  # MT
perceptual-settings-threshold-label = Warn threshold (lower = stricter match)  # MT

# Phase 42 Part B — per-file rolling versions.  # MT
version-list-heading = Previous versions  # MT
version-list-empty = No prior versions of this file  # MT
version-list-restore = Restore this version  # MT
version-retention-heading = Keep previous versions on overwrite  # MT
version-retention-none = Keep every version forever  # MT
version-retention-last-n = Keep last { $n } versions  # MT
version-retention-older-than-days = Drop versions older than { $days } days  # MT
version-retention-gfs = Hourly { $h } · daily { $d } · weekly { $w } · monthly { $m }  # MT

# Phase 43 — forensic chain-of-custody manifests.  # MT
provenance-settings-heading = Forensic chain-of-custody  # MT
provenance-settings-hint = Sign every copy job with a BLAKE3 + ed25519 manifest. Reviewers can re-hash the destination tree later and prove no byte changed since the copy.  # MT
provenance-settings-enable-default = Sign every new job by default  # MT
provenance-settings-show-after-job = Show manifest after each completed job  # MT
provenance-settings-tsa-url-label = Default RFC 3161 timestamp authority URL  # MT
provenance-settings-tsa-url-hint = Optional. When set, manifests carry a free TSA timestamp proving the bytes existed at this point in time. Leave empty to skip.  # MT
provenance-settings-keys-heading = Signing keys  # MT
provenance-settings-keys-generate = Generate new key  # MT
provenance-settings-keys-import = Import key…  # MT
provenance-settings-keys-export = Export public key…  # MT
provenance-job-completed-title = Provenance manifest saved  # MT
provenance-job-completed-body = { $count } files signed → { $path }  # MT
provenance-verify-clean = Manifest valid for { $count } files; signature { $sig }; merkle root OK.  # MT
provenance-verify-tampered = Manifest INVALID — { $tampered } tampered, { $missing } missing.  # MT
provenance-action-staged = Phase 43 — wiring the IPC for this action lands in a follow-up commit.  # MT

# Phase 44 — SSD-aware whole-drive sanitize.  # MT
sanitize-heading = Whole-drive secure sanitize  # MT
sanitize-hint = NVMe Sanitize, OPAL Crypto Erase, and ATA Secure Erase wipe a flash drive at the firmware layer in milliseconds. Per-file overwrite is meaningless on flash — multi-pass shred only burns NAND. Use this for actual purge.  # MT
sanitize-pick-device = Choose the drive to sanitize  # MT
sanitize-mode-label = Sanitization method  # MT
sanitize-mode-nvme-format = NVMe Format (with secure erase)  # MT
sanitize-mode-nvme-sanitize-block = NVMe Sanitize — Block Erase (slow, every cell)  # MT
sanitize-mode-nvme-sanitize-crypto = NVMe Sanitize — Crypto Erase (instant)  # MT
sanitize-mode-ata-secure-erase = ATA Secure Erase (legacy SATA SSDs)  # MT
sanitize-mode-opal-crypto-erase = TCG OPAL Crypto Erase (Self-Encrypting Drives)  # MT
sanitize-mode-apfs-crypto-erase = APFS Crypto Erase (rotate FileVault key, macOS only)  # MT
sanitize-confirm-1 = This destroys EVERY byte on { $device }. There is no undo.  # MT
sanitize-confirm-2 = I understand that all partitions, all files, and all snapshots on { $device } will be permanently unreadable.  # MT
sanitize-confirm-3 = Type the drive's model name to proceed: { $model }  # MT
sanitize-running = Sanitizing { $device } ({ $mode }) — this can take from milliseconds (crypto erase) to tens of minutes (block erase). Do not power down.  # MT
sanitize-completed = Sanitize complete — { $device } is now blank.  # MT
ssd-honest-shred-meaningless = Per-file shred on a copy-on-write filesystem (Btrfs / ZFS / APFS) cannot reach the underlying blocks. Use whole-drive sanitize plus full-disk-encryption key rotation instead.  # MT
ssd-honest-advisory = This file lives on flash. Per-file overwrite costs NAND wear and does NOT guarantee the original cells are unrecoverable. For sensitive data, sanitize the whole drive.  # MT

# Phase 44.1f.  # MT
sanitize-action-staged = Phase 44.1 — wiring the IPC for this action lands in a follow-up commit.  # MT

# Phase 45.3 — named-queue tab strip.  # MT
queue-tab-default = Default  # MT
queue-tab-empty-state = Job queues  # MT
queue-badge-tooltip = Pending and running jobs in this queue  # MT

# Phase 45.4 — drag-progress-merge.  # MT
queue-drag-hint = Drag onto another queue to merge  # MT
queue-merge-confirm = Drop to merge  # MT
queue-merge-toast = Queues merged  # MT

# Phase 45.5 — F2-queue UX.  # MT
queue-f2-active-hint = F2 mode: every new enqueue lands in this queue  # MT
queue-f2-toggled-on = F2 queue mode ON — new enqueues join the running queue  # MT
queue-f2-toggled-off = F2 queue mode OFF — new enqueues spawn parallel queues  # MT
queue-f2-status-bar = F2 queue mode: ON  # MT

# Phase 45.6 — tray destination targets.  # MT
tray-target-section-title = Tray destinations  # MT
tray-target-section-hint = Pinned destinations appear in the tray menu. Click one to arm it as the next drop target.  # MT
tray-target-empty = No tray destinations pinned yet.  # MT
tray-target-remove = Remove  # MT
tray-target-add-label = Label  # MT
tray-target-add-path = Path or backend URI  # MT
tray-target-add = Add  # MT
tray-target-armed-toast = Drop your next file to send it to { $label }  # MT
tray-target-active-pill = → { $label }  # MT

# Phase 45.7 follow-up — pinned-destination validation errors.  # MT
err-pinned-destination-label-empty = Tray destination label can't be empty.  # MT
err-pinned-destination-path-empty = Tray destination path can't be empty.  # MT
err-pinned-destination-label-too-long = Tray destination label is too long (max 64 characters).  # MT
err-pinned-destination-path-too-long = Tray destination path is too long (max 1024 characters).  # MT
err-pinned-destination-label-invalid = Tray destination label contains characters that aren't allowed (newline, return, or NUL).  # MT
err-pinned-destination-path-invalid = Tray destination path contains characters that aren't allowed (newline, return, or NUL).  # MT
err-pinned-destination-too-many = You've reached the limit of 50 tray destinations. Remove one to add another.  # MT
