app-name = Copy That v0.19.84
window-title = Copy That v0.19.84
shred-ssd-advisory = Peringatan: target ini berada di SSD. Penimpaan multi-lintasan tidak dapat membersihkan memori flash secara andal karena wear-leveling dan over-provisioning memindahkan data keluar dari alamat blok logis. Untuk media solid-state, gunakan ATA SECURE ERASE, NVMe Format dengan Secure Erase, atau enkripsi seluruh disk dengan kunci yang dibuang.

# Global aggregate states (header pill)
state-idle = Diam
state-copying = Menyalin
state-verifying = Memverifikasi
state-paused = Dijeda
state-error = Galat

# Per-job states (row badge)
state-pending = Antre
state-running = Berjalan
state-cancelled = Dibatalkan
state-succeeded = Selesai
state-failed = Gagal

# Actions
action-pause = Jeda
action-resume = Lanjutkan
action-cancel = Batal
action-pause-all = Jeda semua tugas
action-resume-all = Lanjutkan semua tugas
action-cancel-all = Batalkan semua tugas
action-close = Tutup
action-reveal = Tampilkan di folder
action-add-files = Tambah berkas
action-add-folders = Tambah folder

# Phase 13d — activity feed
activity-title = Aktivitas
activity-clear = Bersihkan daftar aktivitas
activity-empty = Belum ada aktivitas berkas.
activity-after-done = Saat selesai:
activity-keep-open = Biarkan aplikasi terbuka
activity-close-app = Tutup aplikasi
activity-shutdown = Matikan PC
activity-logoff = Keluar
activity-sleep = Tidur

# Phase 14 — preflight free-space dialog
preflight-block-title = Ruang tujuan tidak cukup
preflight-warn-title = Ruang tujuan menipis
preflight-unknown-title = Tidak dapat menentukan ruang kosong
preflight-unknown-body = Sumber terlalu besar untuk diukur dengan cepat atau volume tujuan tidak merespons. Anda dapat melanjutkan; penjaga ruang mesin akan menghentikan penyalinan dengan rapi jika kehabisan ruang.
preflight-required = Diperlukan
preflight-free = Kosong
preflight-reserve = Cadangan
preflight-shortfall = Kekurangan
preflight-continue = Tetap lanjutkan
preflight-pick-subset = Pilih yang akan disalin…
collision-modal-overwrite-older = Timpa yang lebih lama saja

# Phase 14e — subset picker
subset-title = Pilih sumber yang akan disalin
subset-subtitle = Seluruh pilihan tidak muat di tujuan. Centang item yang ingin Anda salin; sisanya akan ditinggalkan.
subset-loading = Mengukur ukuran…
subset-too-large = terlalu besar untuk dihitung
subset-budget = Tersedia
subset-remaining = Tersisa
subset-confirm = Salin pilihan
history-rerun-hint = Jalankan ulang penyalinan ini — memindai ulang setiap berkas di pohon sumber
history-clear-all = Bersihkan semua
history-clear-all-confirm = Klik lagi untuk konfirmasi
history-clear-all-hint = Hapus setiap baris riwayat. Memerlukan klik kedua untuk konfirmasi.
toast-history-cleared = Riwayat dibersihkan ({ $count } baris dihapus)

# Phase 15 — source-list ordering in the drop staging dialog
drop-dialog-sort-label = Urutan:
sort-custom = Khusus
sort-name-asc = Nama A → Z (berkas dulu)
sort-name-desc = Nama Z → A (berkas dulu)
sort-size-asc = Ukuran terkecil dulu (berkas dulu)
sort-size-desc = Ukuran terbesar dulu (berkas dulu)
sort-reorder = Atur ulang
sort-move-top = Pindah ke atas
sort-move-up = Naik
sort-move-down = Turun
sort-move-bottom = Pindah ke bawah

# Phase 16 — sort preset names for the Activity list (shorter
# wording than the DropStagingDialog variants; the Activity list
# is files-only so the "(files first)" suffix isn't relevant).
sort-name-asc-simple = Nama A → Z
sort-name-desc-simple = Nama Z → A
sort-size-asc-simple = Ukuran terkecil dulu
sort-size-desc-simple = Ukuran terbesar dulu
activity-sort-locked = Pengurutan dinonaktifkan saat penyalinan berjalan. Jeda atau tunggu hingga selesai, lalu ubah urutannya.

# Phase 17 — collision-policy picker in the DropStagingDialog
drop-dialog-collision-label = Jika berkas sudah ada:
collision-policy-keep-both = Simpan keduanya (ganti nama salinan baru menjadi _2, _3, …)
collision-policy-skip = Lewati salinan baru
collision-policy-overwrite = Timpa berkas yang ada
collision-policy-overwrite-if-newer = Timpa hanya jika lebih baru
collision-policy-prompt = Tanya setiap kali

# Phase 18 — progress labels on the DropStagingDialog Start button
drop-dialog-busy-checking = Memeriksa ruang kosong…
drop-dialog-busy-enumerating = Menghitung berkas…
drop-dialog-busy-starting = Memulai penyalinan…
toast-enumeration-deferred = Pohon sumber besar — melewati daftar berkas di awal; baris akan muncul seiring mesin memprosesnya.

# Context menu (per-row right-click)
menu-pause = Jeda
menu-resume = Lanjutkan
menu-cancel = Batal
menu-remove = Hapus dari antrean
menu-reveal-source = Tampilkan sumber di folder
menu-reveal-destination = Tampilkan tujuan di folder

# Header / toolbar
header-eta-label = Perkiraan waktu tersisa
header-toolbar-label = Kontrol global

# Footer
footer-queued = tugas aktif
footer-total-bytes = sedang berlangsung
footer-errors = galat
footer-history = Riwayat

# Empty state
empty-title = Letakkan berkas atau folder untuk disalin
empty-hint = Seret item ke jendela. Kami akan menanyakan tujuan, lalu mengantrekan satu tugas per sumber.
empty-region-label = Daftar tugas

# Details drawer
details-drawer-label = Detail tugas
details-source = Sumber
details-destination = Tujuan
details-state = Status
details-bytes = Bita
details-files = Berkas
details-speed = Kecepatan
details-eta = ETA
details-error = Galat

# Drop dialog
drop-dialog-title = Transfer item yang dijatuhkan
drop-dialog-subtitle = { $count } item siap ditransfer. Pilih folder tujuan untuk memulai.
drop-dialog-mode = Operasi
drop-dialog-copy = Salin
drop-dialog-move = Pindah
drop-dialog-pick-destination = Pilih tujuan
drop-dialog-change-destination = Ubah tujuan
drop-dialog-start-copy = Mulai menyalin
drop-dialog-start-move = Mulai memindahkan

# ETA placeholders
eta-calculating = menghitung…
eta-unknown = tidak diketahui

# Toast messages
toast-job-done = Transfer selesai
toast-copy-queued = Penyalinan diantrekan
toast-move-queued = Pemindahan diantrekan
toast-error-resolved = Galat teratasi
toast-collision-resolved = Tabrakan teratasi
toast-elevated-unavailable = Coba ulang dengan hak istimewa hadir di Fase 17 — belum tersedia
toast-clipboard-files-detected = Ada berkas di papan klip — tekan pintasan tempel untuk menyalin lewat Copy That
toast-clipboard-no-files = Papan klip tidak punya berkas untuk ditempel
toast-error-log-exported = Log galat diekspor

# Error modal (Phase 8)
error-modal-title = Sebuah transfer gagal
error-modal-retry = Coba ulang
error-modal-retry-elevated = Coba ulang dengan izin yang ditinggikan
error-modal-skip = Lewati
error-modal-skip-all-kind = Lewati semua galat jenis ini
error-modal-abort = Batalkan semua
error-modal-path-label = Jalur
error-modal-code-label = Kode
error-drawer-pending-count = Galat lain menunggu
error-drawer-toggle = Ciutkan atau perluas

# Error-kind labels (Phase 8). Source of truth — engine maps each
# `CopyErrorKind` to one of these keys via `localized_key()`.
err-not-found = Berkas tidak ditemukan
err-permission-denied = Izin ditolak
err-disk-full = Disk tujuan penuh
err-interrupted = Operasi terganggu
err-verify-failed = Verifikasi pascasalin gagal
err-path-escape = Jalur ditolak — berisi segmen direktori induk (..) atau bita ilegal
err-path-invalid-encoding = Jalur ditolak — string berisi UTF-8 tidak valid / karakter pengganti
err-helper-invalid-json = Helper berhak istimewa menerima JSON yang rusak; permintaan ini diabaikan
err-helper-grant-out-of-band = GrantCapabilities harus ditangani oleh run-loop helper, bukan handler tanpa keadaan
err-randomness-unavailable = Pembangkit angka acak OS gagal; tidak dapat membuat id sesi
err-sparseness-mismatch = Tata letak sparse tidak dapat dipertahankan di tujuan
err-io-other = Galat I/O tidak diketahui

# Collision modal (Phase 8)
collision-modal-title = Berkas sudah ada
collision-modal-overwrite = Timpa
collision-modal-overwrite-if-newer = Timpa jika lebih baru
collision-modal-skip = Lewati
collision-modal-keep-both = Simpan keduanya
collision-modal-rename = Ganti nama…
collision-modal-apply-to-all = Terapkan ke semua
collision-modal-source = Sumber
collision-modal-destination = Tujuan
collision-modal-size = Ukuran
collision-modal-modified = Diubah
collision-modal-hash-check = Hash cepat (SHA-256)
collision-modal-hash-computing = Menghitung…
collision-modal-hash-identical = Identik
collision-modal-hash-different = Berbeda
collision-modal-rename-placeholder = Nama berkas baru
collision-modal-confirm-rename = Ganti nama

# Error log drawer (Phase 8)
error-log-title = Log galat
error-log-empty = Tidak ada galat tercatat
error-log-export-csv = Ekspor CSV
error-log-export-txt = Ekspor teks
error-log-clear = Bersihkan log
error-log-col-time = Waktu
error-log-col-job = Tugas
error-log-col-path = Jalur
error-log-col-code = Kode
error-log-col-message = Pesan
error-log-col-resolution = Resolusi

# History drawer (Phase 9)
history-title = Riwayat
history-empty = Belum ada tugas tercatat
history-unavailable = Riwayat penyalinan tidak tersedia. Aplikasi tidak dapat membuka penyimpanan SQLite saat memulai.
history-filter-any = apa saja
history-filter-kind = Jenis
history-filter-status = Status
history-filter-text = Cari
history-refresh = Segarkan
history-export-csv = Ekspor CSV
history-purge-30 = Hapus > 30 hari
history-rerun = Jalankan ulang
history-detail-open = Detail
history-detail-title = Detail tugas
history-detail-empty = Tidak ada item tercatat
history-col-date = Tanggal
history-col-kind = Jenis
history-col-src = Sumber
history-col-dst = Tujuan
history-col-files = Berkas
history-col-size = Ukuran
history-col-status = Status
history-col-duration = Durasi
history-col-error = Galat
toast-history-exported = Riwayat diekspor
toast-history-rerun-queued = Jalankan ulang diantrekan

# Totals drawer (Phase 10)
footer-totals = Total
totals-title = Total
totals-loading = Memuat total…
totals-card-bytes = Total bita disalin
totals-card-files = Berkas
totals-card-jobs = Tugas
totals-card-avg-rate = Throughput rata-rata
totals-errors = galat
totals-spark-title = 30 hari terakhir
totals-kinds-title = Menurut jenis
totals-saved-title = Waktu dihemat (perkiraan)
totals-saved-note = Perkiraan dibanding penyalinan baku oleh pengelola berkas untuk beban kerja yang sama.
totals-reset = Atur ulang statistik
totals-reset-confirm = Ini menghapus setiap tugas dan item tersimpan. Lanjutkan?
totals-reset-confirm-yes = Ya, atur ulang
toast-totals-reset = Statistik diatur ulang

# Phase 11a — i18n core: surface remaining user-visible strings so
# every string on a main-window screen flows through Fluent.

# Header language switcher (temporary placement — folds into
# Settings → General in Phase 12).
header-language-label = Bahasa
header-language-title = Ubah bahasa

# Job-kind labels. History filter, Totals breakdown, and history
# rows all map wire-format `kind` strings to these labels.
kind-copy = Salin
kind-move = Pindah
kind-delete = Hapus
kind-secure-delete = Hapus aman

# History status labels. Distinct from `state-*` because the
# history wire format uses plain `running`/`succeeded`/... rather
# than the live `JobState` enum the queue exposes.
status-running = Berjalan
status-succeeded = Berhasil
status-failed = Gagal
status-cancelled = Dibatalkan
# Per-item status (not job-level): `ok` / `skipped` are only
# reachable from the history detail view.
status-ok = OK
status-skipped = Dilewati

# History drawer: search field placeholder + purge toast.
history-search-placeholder = /jalur
toast-history-purged = Menghapus { $count } tugas yang lebih lama dari 30 hari

# User-facing command-layer validation errors. The Rust side
# returns these keys when input is missing; the toast layer looks
# them up rather than showing the raw English.
err-source-required = Setidaknya satu jalur sumber diperlukan.
err-destination-empty = Jalur tujuan kosong.
err-source-empty = Jalur sumber kosong.

# Localised duration formatting for the Totals drawer and ETA
# fields. `{ $ms }`, `{ $s }`, `{ $m }`, `{ $h }` are integer
# placeables — the formatter passes pre-computed values in.
duration-lt-1s = < 1d
duration-ms = { $ms } md
duration-seconds = { $s }d
duration-minutes-seconds = { $m }mnt { $s }d
duration-hours-minutes = { $h }j { $m }mnt
duration-zero = 0d

# Rate unit. Appended to a formatted byte size. Some languages
# render this with a leading space ("Ko/s"); keep it translatable
# even though the SI-derived "/s" is near-universal.
rate-unit-per-second = { $size }/d

# Phase 11b — Settings modal skeleton. Phase 12 expanded this into a
# full six-tab preferences window; the `settings-phase-12-hint` key
# is retired in favour of `settings-tab-profiles` + concrete labels.
settings-title = Pengaturan
settings-tab-general = Umum
settings-tab-appearance = Tampilan
settings-section-language = Bahasa
settings-phase-12-hint = Lebih banyak pengaturan (tema, default transfer, algoritma verifikasi, profil) hadir di Fase 12.

# Phase 12 — full Settings window keys. Grouped by tab so the
# human-review pass can pick one section at a time. Every field
# label and every option label that the user will actually read
# has its own key; dynamic wire values (enum discriminants like
# "auto" / "prefer") stay untranslated on the wire.

settings-loading = Memuat pengaturan…
settings-tab-transfer = Transfer
settings-tab-filters = Filter
settings-tab-shell = Shell
settings-tab-secure-delete = Hapus aman
settings-tab-advanced = Lanjutan
settings-tab-updater = Pembaruan
settings-tab-profiles = Profil

# General tab additions
settings-section-theme = Tema
settings-theme-auto = Otomatis
settings-theme-light = Terang
settings-theme-dark = Gelap
settings-start-with-os = Jalankan saat sistem dinyalakan
settings-single-instance = Satu instans berjalan
settings-minimize-to-tray = Kecilkan ke baki saat ditutup
settings-error-display-mode = Gaya prompt galat
settings-error-display-modal = Modal (memblokir aplikasi)
settings-error-display-drawer = Laci (tidak memblokir)
settings-error-display-mode-hint = Modal menghentikan antrean sampai Anda memutuskan. Laci membuat antrean tetap berjalan dan membiarkan Anda memilah galat di pojok.
settings-paste-shortcut = Tempel berkas lewat pintasan global
settings-paste-shortcut-combo = Kombinasi pintasan
settings-paste-shortcut-hint = Tekan kombinasi ini di mana pun pada sistem Anda untuk menempel berkas yang disalin dari Explorer / Finder / Files lewat Copy That. CmdOrCtrl menjadi Cmd di macOS, Ctrl di Windows / Linux.
settings-clipboard-watcher = Pantau papan klip untuk berkas yang disalin
settings-clipboard-watcher-hint = Tampilkan toast saat URL berkas muncul di papan klip, mengisyaratkan Anda bisa menempel lewat Copy That. Memantau setiap 500 md saat aktif.

# Transfer tab
settings-buffer-size = Ukuran buffer
settings-verify = Verifikasi setelah menyalin
settings-verify-off = Mati
settings-concurrency = Konkurensi
settings-concurrency-auto = Otomatis
settings-reflink = Reflink / jalur cepat
settings-reflink-prefer = Utamakan
settings-reflink-avoid = Hindari reflink
settings-reflink-disabled = Selalu gunakan mesin asinkron
settings-fsync-on-close = Sinkronkan ke disk saat menutup (lebih lambat, lebih aman)
settings-preserve-timestamps = Pertahankan stempel waktu
settings-preserve-permissions = Pertahankan izin
settings-preserve-acls = Pertahankan ACL (Fase 14)
settings-preserve-sparseness = Pertahankan berkas sparse
settings-preserve-sparseness-hint = Salin hanya extent yang teralokasi dari berkas sparse (disk VM, berkas basis data) agar ukuran tujuan di disk tetap sama dengan sumber.
settings-force-parallel-chunks = Salinan multi-chunk paralel (hanya RAID / larik)
settings-force-parallel-chunks-hint = Membagi setiap salinan besar menjadi potongan serentak. Hanya membantu tujuan striped/RAID/jaringan; MEMPERLAMBAT satu SSD/NVMe (-25% hingga -76%). Biarkan nonaktif kecuali tujuan Anda adalah larik multi-disk.

# Shell tab
settings-context-menu = Aktifkan entri menu konteks shell
settings-intercept-copy = Cegat penangan salin bawaan (Windows)
settings-intercept-copy-hint = Saat aktif, Ctrl+C / Ctrl+V di Explorer dialihkan lewat Copy That. Pendaftaran hadir di Fase 14.
settings-notify-completion = Beri tahu saat tugas selesai

# Secure delete tab
settings-shred-method = Metode shred default
settings-shred-zero = Nol (1 lintasan)
settings-shred-random = Acak (1 lintasan)
settings-shred-dod3 = DoD 5220.22-M (3 lintasan)
settings-shred-dod7 = DoD 5220.22-M (7 lintasan)
settings-shred-gutmann = Gutmann (35 lintasan)
settings-shred-nist = NIST 800-88
settings-shred-confirm-twice = Wajibkan konfirmasi ganda sebelum shred

# Advanced tab
settings-log-level = Tingkat log
settings-log-off = Mati
settings-telemetry = Telemetri
settings-telemetry-never = Tidak pernah — tanpa phone-home pada tingkat log apa pun
settings-error-policy = Kebijakan galat default
settings-error-policy-ask = Tanya
settings-error-policy-skip = Lewati
settings-error-policy-retry = Coba ulang dengan backoff
settings-error-policy-abort = Batalkan pada kegagalan pertama
settings-history-retention = Retensi riwayat (hari)
settings-history-retention-hint = 0 = simpan selamanya. Nilai lain otomatis menghapus tugas lama saat memulai.
settings-database-path = Jalur basis data
settings-database-path-default = (default — direktori data OS)
settings-reset-all = Atur ulang ke default
settings-reset-confirm = Atur ulang setiap preferensi ke defaultnya? Profil tidak terpengaruh.

# Profiles tab
settings-profiles-hint = Simpan pengaturan saat ini dengan sebuah nama; muat nanti untuk beralih kembali tanpa menyentuh setelan satu per satu.
settings-profile-name-placeholder = Nama profil
settings-profile-save = Simpan
settings-profile-import = Impor…
settings-profile-load = Muat
settings-profile-export = Ekspor…
settings-profile-delete = Hapus
settings-profile-empty = Belum ada profil tersimpan.
settings-profile-import-prompt = Nama untuk profil yang diimpor:

# Toasts driven by Phase 12 profile actions
toast-settings-reset = Pengaturan diatur ulang
toast-profile-saved = Profil disimpan
toast-profile-loaded = Profil dimuat
toast-profile-exported = Profil diekspor
toast-profile-imported = Profil diimpor

# Phase 14a — enumeration-time filters
settings-filters-hint = Lewati berkas saat enumerasi agar mesin tidak pernah membukanya. Sertakan hanya berlaku untuk berkas; pengecualian juga memangkas direktori yang cocok.
settings-filters-enabled = Aktifkan filter untuk penyalinan pohon
settings-filters-include-globs = Sertakan glob
settings-filters-include-globs-placeholder = **/*.txt
settings-filters-include-globs-hint = Satu glob per baris. Saat tidak kosong, sebuah berkas harus cocok dengan setidaknya satu penyertaan untuk bertahan. Direktori selalu ditelusuri.
settings-filters-exclude-globs = Kecualikan glob
settings-filters-exclude-globs-placeholder = **/node_modules
settings-filters-exclude-globs-hint = Satu glob per baris. Kecocokan memangkas seluruh subpohon untuk direktori; berkas yang cocok dilewati.
settings-filters-size-range = Rentang ukuran berkas
settings-filters-min-size-bytes = Ukuran minimum (bita, kosong = tanpa batas bawah)
settings-filters-max-size-bytes = Ukuran maksimum (bita, kosong = tanpa batas atas)
settings-filters-date-range = Rentang waktu modifikasi
settings-filters-min-mtime = Diubah pada atau setelah
settings-filters-max-mtime = Diubah pada atau sebelum
settings-filters-attributes = Bit atribut
settings-filters-skip-hidden = Lewati berkas / folder tersembunyi
settings-filters-skip-system = Lewati berkas sistem (Windows saja)
settings-filters-skip-readonly = Lewati berkas hanya-baca

# Phase 15 — auto-update
settings-updater-hint = Copy That memeriksa pembaruan bertanda tangan paling banyak sekali sehari. Pembaruan terpasang saat aplikasi berikutnya ditutup.
settings-updater-auto-check = Periksa pembaruan saat diluncurkan
settings-updater-channel = Kanal rilis
settings-updater-channel-stable = Stabil
settings-updater-channel-beta = Beta (pra-rilis)
settings-updater-last-check = Terakhir diperiksa
settings-updater-last-never = Tidak pernah
settings-updater-check-now = Periksa pembaruan sekarang
settings-updater-checking = Memeriksa…
settings-updater-available = Pembaruan tersedia
settings-updater-up-to-date = Anda menjalankan rilis terbaru.
settings-updater-dismiss = Lewati versi ini
settings-updater-dismissed = Dilewati
toast-update-available = Versi yang lebih baru tersedia
toast-update-up-to-date = Anda sudah memakai versi terbaru

# Phase 19a — disk-backed file enumeration (TeraCopy-compatible scan DB)
scan-progress-title = Memindai…
scan-progress-stats = { $files } berkas · { $bytes } sejauh ini
scan-pause-button = Jeda pemindaian
scan-resume-button = Lanjutkan pemindaian
scan-cancel-button = Batalkan pemindaian
scan-cancel-confirm = Batalkan pemindaian dan buang progres?
scan-db-header = Basis data pemindaian
scan-db-hint = Basis data pemindaian di disk untuk tugas berisi jutaan berkas.
advanced-scan-hash-during = Hitung checksum saat memindai
advanced-scan-db-path = Lokasi basis data pemindaian
advanced-scan-retention-days = Hapus otomatis pemindaian selesai setelah (hari)
advanced-scan-max-keep = Maksimum basis data pemindaian yang disimpan

# Phase 19b — filesystem-snapshot source for locked files.
settings-on-locked = Saat sebuah berkas terkunci
settings-on-locked-ask = Tanya pertama kali
settings-on-locked-retry = Coba ulang sebentar, lalu munculkan galat
settings-on-locked-skip = Lewati berkas yang terkunci
settings-on-locked-snapshot = Gunakan snapshot sistem berkas
settings-on-locked-hint = Hilangkan galat "berkas sedang digunakan oleh proses lain". Copy That membuat snapshot volume sumber (VSS di Windows, ZFS/Btrfs di Linux, APFS di macOS) dan membaca dari salinan snapshot.
snapshot-prompt-title = Berkas ini sedang digunakan oleh proses lain
snapshot-prompt-body = Program lain membuka { $path } untuk penulisan eksklusif. Pilih cara Copy That menangani berkas ini dan yang serupa di volume yang sama.
snapshot-source-active = 📷 Membaca dari snapshot { $kind } pada { $volume }
snapshot-create-failed = Tidak dapat membuat snapshot volume sumber
snapshot-vss-needs-elevation = Membaca dari snapshot VSS memerlukan izin Administrator. Copy That akan meminta Anda mengizinkannya.
snapshot-cleanup-failed = Helper snapshot melaporkan kegagalan pembersihan — salinan bayangan sisa mungkin masih ada di volume.

# Phase 20 — durable resume journal.
resume-prompt-title = Lanjutkan transfer sebelumnya?
resume-prompt-body = Copy That mendeteksi { $count } transfer yang belum selesai dari sesi sebelumnya. Pilih tindakan untuk masing-masing.
resume-prompt-resume = Lanjutkan
resume-prompt-resume-all = Lanjutkan semua
resume-discard-one = Jangan lanjutkan
resume-discard-all = Buang semua
resume-aborted-hash-mismatch = { $offset } bita pertama di tujuan tidak cocok dengan sumber — memulai ulang dari awal.
settings-auto-resume = Lanjutkan otomatis tugas yang terganggu tanpa bertanya
settings-auto-resume-hint = Lewati prompt lanjutan saat memulai dan diam-diam mengantrekan ulang setiap tugas yang belum selesai. Mati secara default.

# Phase 21 — bandwidth shaping (GCRA token bucket + schedule + auto-throttle).
settings-tab-network = Jaringan
settings-network-hint = Batasi laju transfer agar sisa jaringan tetap dapat digunakan. Terapkan secara global, ikuti jadwal harian, atau bereaksi otomatis terhadap Wi-Fi terukur / baterai / koneksi seluler.
settings-network-mode = Batas bandwidth
settings-network-mode-off = Mati (tanpa batas)
settings-network-mode-fixed = Nilai tetap
settings-network-mode-schedule = Gunakan jadwal
settings-network-cap-mbps = Batas (MB/d)
settings-network-schedule = Jadwal (format rclone)
settings-network-schedule-hint = Batas HH:MM,laju dipisah spasi ditambah aturan hari Mon-Fri,laju opsional. Laju: 512k, 10M, 2G, off, unlimited. Contoh: 08:00,512k 18:00,10M Sat-Sun,unlimited.
settings-network-auto-header = Throttle otomatis
settings-network-auto-metered = Pada Wi-Fi terukur
settings-network-auto-battery = Pada baterai
settings-network-auto-cellular = Pada seluler
settings-network-auto-unchanged = Jangan timpa
settings-network-auto-pause = Jeda transfer
settings-network-auto-cap = Batasi ke nilai tetap
shape-badge-paused = dijeda
shape-badge-tooltip = Batas bandwidth aktif — klik untuk membuka Pengaturan → Jaringan
shape-badge-source-schedule = terjadwal
shape-badge-source-metered = terukur
shape-badge-source-battery = pada baterai
shape-badge-source-cellular = seluler
shape-badge-source-settings = aktif
shape-error-schedule-invalid = Format jadwal tidak valid: { $message }

# Phase 22 — aggregate conflict dialog v2 (thumbnails, per-pattern
# rules, and reusable conflict profiles). Every key below is user-
# visible text in the `ConflictBatchModal.svelte` component.
conflict-batch-title = { $count } konflik berkas di { $jobname }
conflict-batch-state-pending = Tertunda
conflict-batch-state-resolved = Teratasi
conflict-batch-action-overwrite = Timpa
conflict-batch-action-skip = Lewati
conflict-batch-action-keep-both = Simpan keduanya
conflict-batch-action-newer-wins = Yang lebih baru menang
conflict-batch-action-larger-wins = Yang lebih besar menang
conflict-batch-bulk-apply-selected = Terapkan ke yang dipilih
conflict-batch-bulk-apply-extension = Terapkan ke semua ekstensi ini
conflict-batch-bulk-apply-glob = Terapkan ke glob yang cocok…
conflict-batch-bulk-apply-remaining = Terapkan ke semua sisanya
conflict-batch-bulk-glob-placeholder = mis. **/*.tmp
conflict-batch-save-profile = Simpan aturan ini sebagai profil…
conflict-batch-profile-placeholder = Nama profil
conflict-batch-matched-rule = lewat aturan '{ $rule }' → { $action }
conflict-batch-empty = Semua konflik teratasi
conflict-batch-source-vs-destination = Sumber vs. tujuan
conflict-batch-source-label = Sumber
conflict-batch-destination-label = Tujuan
conflict-batch-size-label = Ukuran
conflict-batch-modified-label = Diubah
conflict-batch-close = Tutup
conflict-batch-profile-saved = Profil konflik disimpan

# Phase 23 — sparse-file preservation. The toast fires once per
# destination volume when the filesystem can't preserve holes; the
# warning line is surfaced in the job detail drawer so the user knows
# the dst is larger on disk than the source was.
sparse-not-supported-title = Tujuan mengisi penuh berkas sparse
sparse-not-supported-body = { $dst_fs } tidak mendukung berkas sparse. Lubang pada sumber ditulis sebagai nol, sehingga tujuan lebih besar di disk.
sparse-warning-densified = Tata letak sparse dipertahankan: hanya extent teralokasi yang disalin.
sparse-warning-mismatch = Ketidakcocokan tata letak sparse — tujuan mungkin lebih besar dari yang diharapkan.

# Phase 24 — security-metadata preservation. The Mark-of-the-Web
# (Zone.Identifier ADS) toggle is security-sensitive: turning it off
# lets a downloaded executable shed its SmartScreen / Office Protected
# View flag on copy, which is why the tooltip carries an explicit
# warning. AppleDouble fallback emits `._<filename>` sidecars on
# destination filesystems that can't hold the foreign metadata.
settings-preserve-security-metadata = Pertahankan metadata keamanan
settings-preserve-security-metadata-hint = Tangkap dan terapkan ulang aliran metadata out-of-band (NTFS ADS / xattr / POSIX ACL / konteks SELinux / kapabilitas berkas Linux / resource fork macOS) di setiap penyalinan.
settings-preserve-motw = Pertahankan Mark-of-the-Web (tanda diunduh-dari-internet)
settings-preserve-motw-hint = Penting untuk keamanan. SmartScreen dan Office Protected View memakai aliran ini untuk memperingatkan berkas yang diunduh dari internet. Menonaktifkannya membuat eksekutabel unduhan kehilangan penanda asalnya saat menyalin dan melewati pengaman sistem operasi.
settings-preserve-posix-acls = Pertahankan POSIX ACL dan atribut diperluas
settings-preserve-posix-acls-hint = Bawa xattr user.* / system.* / trusted.* dan daftar kontrol akses POSIX melintasi penyalinan.
settings-preserve-selinux = Pertahankan konteks SELinux
settings-preserve-selinux-hint = Bawa label security.selinux melintasi penyalinan agar daemon yang berjalan di bawah kebijakan MAC tetap dapat mengakses berkas.
settings-preserve-resource-forks = Pertahankan resource fork macOS dan info Finder
settings-preserve-resource-forks-hint = Bawa resource fork warisan dan FinderInfo (tag warna, metadata Carbon) melintasi penyalinan.
settings-appledouble-fallback = Gunakan sidecar AppleDouble pada sistem berkas yang tidak kompatibel
meta-translated-to-appledouble = Metadata asing disimpan dalam sidecar AppleDouble (._{ $ext })

# Phase 25 — two-way sync with vector-clock conflict detection.
# The drawer lists configured sync pairs; each pair runs an
# independent reconciliation round against a per-pair `.copythat-sync.db`
# state store. Concurrent edits from a common ancestor surface as
# conflicts rather than silent overwrites; the losing side's content
# is preserved as `name.sync-conflict-YYYYMMDD-HHMMSS-<host>.ext`.
footer-sync = Sinkron
sync-drawer-title = Sinkronisasi dua arah
sync-drawer-hint = Jaga dua folder tetap sinkron tanpa penimpaan diam-diam. Suntingan bersamaan muncul sebagai konflik yang dapat Anda selesaikan.
sync-add-pair = Tambah pasangan
sync-add-cancel = Batal
sync-refresh = Segarkan
sync-add-save = Simpan pasangan
sync-add-saving = Menyimpan…
sync-add-missing-fields = Label, jalur kiri, dan jalur kanan semuanya wajib diisi.
sync-remove-confirm = Hapus pasangan sinkron ini? Basis data status dipertahankan; folder tidak tersentuh.
sync-field-label = Label
sync-field-label-placeholder = mis. Dokumen ↔ NAS
sync-field-left = Folder kiri
sync-field-left-placeholder = Pilih atau tempel jalur absolut
sync-field-right = Folder kanan
sync-field-right-placeholder = Pilih atau tempel jalur absolut
sync-field-mode = Mode
sync-mode-two-way = Dua arah
sync-mode-mirror-left-to-right = Cermin (kiri → kanan)
sync-mode-mirror-right-to-left = Cermin (kanan → kiri)
sync-mode-contribute-left-to-right = Kontribusi (kiri → kanan, tanpa hapus)
sync-no-pairs = Belum ada pasangan sinkron yang dikonfigurasi. Klik "Tambah pasangan" untuk memulai.
sync-loading = Memuat pasangan yang dikonfigurasi…
sync-never-run = Belum pernah dijalankan
sync-running = Berjalan
sync-run-now = Jalankan sekarang
sync-cancel = Batal
sync-remove-pair = Hapus
sync-view-conflicts = Lihat konflik ({ $count })
sync-conflicts-heading = Konflik
sync-no-conflicts = Tidak ada konflik dari proses terakhir.
sync-winner = Pemenang
sync-side-left-to-right = kiri
sync-side-right-to-left = kanan
sync-conflict-kind-concurrent-write = Suntingan bersamaan
sync-conflict-kind-delete-edit = Hapus ↔ sunting
sync-conflict-kind-add-add = Kedua sisi menambahkan
sync-conflict-kind-corrupt-equal = Konten menyimpang tanpa penulisan baru
sync-resolve-keep-left = Simpan kiri
sync-resolve-keep-right = Simpan kanan
sync-resolve-keep-both = Simpan keduanya
sync-resolve-three-way = Selesaikan via penggabungan 3 arah
sync-resolve-phase-53-tooltip = Penggabungan 3 arah interaktif untuk berkas non-teks hadir di Fase 53.
sync-error-prefix = Galat sinkron

# Phase 26 — real-time mirror watcher. "Live mirror" starts a
# filesystem watcher on the pair's left side; every debounced event
# triggers a re-sync. The watcher filters vim swap files / Office
# lock files / atomic-save temp names so one logical save becomes
# exactly one sync round.
live-mirror-start = Mulai cermin langsung
live-mirror-stop = Hentikan cermin langsung
live-mirror-watching = Memantau
live-mirror-toggle-hint = Sinkronkan ulang otomatis pada setiap perubahan sistem berkas yang terdeteksi. Satu utas latar per pasangan aktif.
watch-event-prefix = Perubahan berkas
watch-overflow-recovered = Buffer pemantau meluap; enumerasi ulang untuk memulihkan

# Phase 27 — content-defined chunk store. Enables delta-resume (a
# retry only re-writes chunks that actually changed) and same-job
# dedup (files sharing content blocks store those blocks once). The
# store is disk-backed under `<data-dir>/chunks/` by default and is
# the foundation for the Phase 49–51 moonshot repository phases.
chunk-store-section = Penyimpanan chunk
chunk-store-enable = Aktifkan penyimpanan chunk (delta-resume dan dedup)
chunk-store-enable-hint = Memecah setiap berkas tersalin menurut konten (FastCDC) dan menyimpan chunk dengan pengalamatan konten. Percobaan ulang hanya menulis ulang chunk yang berubah; berkas dengan konten bersama otomatis ter-dedup.
chunk-store-location = Lokasi penyimpanan chunk
chunk-store-max-size = Ukuran maksimum penyimpanan chunk
chunk-store-prune = Pangkas chunk yang lebih lama dari (hari)
chunk-store-savings = Hemat { $gib } GiB lewat dedup chunk
chunk-store-disk-usage = Memakai { $size } di { $chunks } chunk

# Phase 28 — tray-resident Drop Stack. The stack is a persistent list
# of paths gathered from multiple sources (Explorer drag, main-window
# context menu, CLI, drag onto the Drop Stack window) that the user
# can dispatch to a destination in one go.
dropstack-window-title = Drop Stack
dropstack-tray-open = Drop Stack
dropstack-empty-title = Drop Stack kosong
dropstack-empty-hint = Seret berkas ke sini dari Explorer atau klik kanan baris tugas untuk menambahkannya.
dropstack-add-to-stack = Tambah ke Drop Stack
dropstack-copy-all-to = Salin semua ke…
dropstack-move-all-to = Pindahkan semua ke…
dropstack-clear = Bersihkan tumpukan
dropstack-remove-row = Hapus dari tumpukan
dropstack-path-missing-toast = { $path } dijatuhkan — berkas tidak ada lagi.
dropstack-always-on-top = Jaga Drop Stack selalu di atas
dropstack-show-tray-icon = Tampilkan ikon baki Copy That
dropstack-open-on-start = Buka Drop Stack otomatis saat aplikasi dimulai
dropstack-count = { $count } jalur

# Phase 29 — spring-loaded folders + native DnD polish. The Settings
# → General tab carries the knobs; DropTarget / DestinationPicker
# Svelte components use the dropzone-* keys at runtime.
settings-dnd-heading = Seret dan jatuhkan
settings-dnd-spring-load = Buka folder otomatis saat menyeret
settings-dnd-spring-delay = Jeda buka otomatis (md)
settings-dnd-thumbnails = Tampilkan thumbnail seret
settings-dnd-invalid-highlight = Sorot target jatuh yang tidak valid
dropzone-invalid-title = Bukan target jatuh yang valid
dropzone-invalid-readonly = Tujuan hanya-baca
dropzone-picker-title = Pilih tujuan
dropzone-picker-up = Naik
dropzone-picker-path = Jalur saat ini
dropzone-picker-root = Akar
dropzone-picker-use-this = Gunakan folder ini
dropzone-picker-empty = Tidak ada subfolder
dropzone-picker-cancel = Batal

# Phase 30 — cross-platform path translation. Settings → Transfer
# exposes these under a "Cross-platform compatibility" subsection.
translate-heading = Kompatibilitas lintas platform
translate-unicode-label = Normalisasi Unicode
translate-unicode-auto = Deteksi tujuan otomatis
translate-unicode-windows = NFC (Windows / Linux)
translate-unicode-macos = Biarkan apa adanya (macOS / APFS)
translate-line-endings-label = Terjemahkan akhir baris untuk berkas teks
translate-line-endings-allowlist = Ekstensi berkas teks
reserved-name-label = Penanganan nama cadangan Windows
reserved-name-suffix = Tambahkan "_" (CON.txt → CON_.txt)
reserved-name-reject = Tolak dan peringatkan
long-path-label = Gunakan awalan jalur panjang Windows (\\?\) saat lebih dari 260 karakter
long-path-hint = Beberapa berbagi jaringan dan alat lawas tidak menghormati ruang nama \\?\.

# Phase 31 — power-aware copying. Settings → Power & State tab + the
# header badge that renders "⏸ Paused — Zoom call detected" when the
# runner's power subscriber has paused or capped due to a policy match.
power-heading = Daya & Status
power-enabled = Aktifkan aturan sadar-daya
power-battery-label = Pada baterai
power-metered-label = Pada Wi-Fi terukur
power-cellular-label = Pada seluler
power-presentation-label = Saat presentasi (Zoom / Teams / Keynote)
power-fullscreen-label = Saat sebuah aplikasi layar penuh
power-thermal-label = Saat CPU melakukan thermal-throttling
power-rule-continue = Lanjutkan dengan kecepatan penuh
power-rule-pause = Jeda semua tugas
power-rule-cap = Batasi bandwidth
power-rule-cap-percent = Batasi ke persentase dari laju saat ini
power-reason-on-battery = pada baterai
power-reason-metered-network = jaringan terukur
power-reason-cellular-network = jaringan seluler
power-reason-presenting = mode presentasi
power-reason-fullscreen = aplikasi layar penuh
power-reason-thermal-throttling = CPU melakukan throttling

# Phase 32 — cloud backend matrix via OpenDAL. Settings → Remotes
# tab + the Add-backend wizard that writes one entry per remote into
# the keychain-backed credential store.
remote-heading = Backend jarak jauh
remote-add = Tambah backend
remote-list-empty = Tidak ada backend jarak jauh yang dikonfigurasi
remote-test = Uji koneksi
remote-test-success = Koneksi berhasil
remote-test-failed = Koneksi gagal
remote-remove = Hapus backend
remote-name-label = Nama tampilan
remote-kind-label = Jenis backend
remote-save = Simpan backend
remote-cancel = Batal
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
backend-local-fs = Sistem berkas lokal
cloud-config-bucket = Bucket
cloud-config-region = Wilayah
cloud-config-endpoint = URL endpoint
cloud-config-root = Jalur akar
cloud-error-invalid-config = Konfigurasi backend tidak valid
cloud-error-network = Galat jaringan saat menghubungi backend
cloud-error-not-found = Objek tidak ditemukan di jalur yang diminta
cloud-error-permission = Izin ditolak oleh backend jarak jauh
cloud-error-keychain = Akses keychain OS gagal
settings-tab-remotes = Jarak jauh
settings-tab-mobile = Seluler

# Phase 33 — mount Copy That's chunk store + history archive as a
# read-only filesystem (FUSE / WinFsp). Surfaced by the History tab
# context menu's "Mount snapshot" action + the Settings → Advanced
# "Mount latest on launch" toggle.
mount-heading = Pasang snapshot
mount-action-mount = Pasang snapshot
mount-action-unmount = Lepas pasang
mount-status-mounted = Terpasang di { $path }
mount-error-unsafe-mountpoint = Jalur titik pasang tidak aman
mount-error-mountpoint-not-empty = Titik pasang harus berupa direktori kosong
mount-error-backend-unavailable = Backend pemasangan tidak tersedia di sistem ini
mount-error-archive-read = Pembacaan arsip gagal
mount-picker-title = Pilih direktori titik pasang
mount-toast-mounted = Snapshot terpasang di { $path }
mount-toast-unmounted = Snapshot dilepas
mount-toast-failed = Pemasangan gagal: { $reason }
settings-mount-heading = Pasang snapshot
settings-mount-hint = Ekspos arsip riwayat sebagai sistem berkas hanya-baca. Fase 33b menyambungkan alur runner; backend kernel FUSE/WinFsp hadir di Fase 33c.
settings-mount-on-launch = Pasang snapshot terbaru saat diluncurkan
settings-mount-on-launch-path = Jalur titik pasang
settings-mount-on-launch-path-placeholder = mis. C:\Mounts\copythat

# Phase 34 — enterprise-grade audit log export + WORM (write-once-read-
# many) tamper-resistance. Settings → Advanced → Audit log collects the
# format + destination + rotation + WORM toggle; runtime records job and
# file events into the tamper-evident chain-hashed sink.
settings-audit-heading = Log audit
settings-audit-hint = Log tahan-rusak hanya-tambah dari setiap peristiwa tugas dan berkas. Format meliputi CSV, JSON-lines, Syslog RFC 5424, ArcSight CEF, dan QRadar LEEF.
settings-audit-enable = Aktifkan pencatatan audit
settings-audit-format = Format log
settings-audit-format-json-lines = JSON lines (default yang disarankan)
settings-audit-format-csv = CSV (ramah lembar kerja)
settings-audit-format-syslog = Syslog (RFC 5424)
settings-audit-format-cef = CEF (ArcSight)
settings-audit-format-leef = LEEF 2.0 (IBM QRadar)
settings-audit-file-path = Jalur berkas log
settings-audit-file-path-placeholder = mis. C:\ProgramData\CopyThat\audit.log
settings-audit-max-size = Putar setelah (bita, 0 = tidak pernah)
settings-audit-worm = Aktifkan mode WORM (write-once-read-many)
settings-audit-worm-hint = Menerapkan flag hanya-tambah platform (Linux chattr +a, macOS chflags uappnd, atribut hanya-baca Windows) setelah setiap pembuatan atau pemutaran. Bahkan administrator harus secara eksplisit menghapus flag untuk memangkas log.
settings-audit-test-write = Uji tulis
settings-audit-verify-chain = Verifikasi rantai
toast-audit-test-write-ok = Uji tulis log audit berhasil
toast-audit-verify-ok = Rantai audit terverifikasi utuh
toast-audit-verify-failed = Verifikasi rantai audit melaporkan ketidakcocokan

# Phase 35 — destination encryption via age (X25519 / passphrase /
# SSH) + on-the-fly zstd compression with a per-extension deny
# heuristic. Surfaced through Settings → Transfer → Encryption +
# Compression; engine short-circuits to the crypt pipeline when
# either stage is active.
settings-crypt-heading = Enkripsi & kompresi
settings-crypt-hint = Ubah isi berkas sebelum tiba di tujuan. Enkripsi memakai format age; kompresi memakai zstd dan dapat melewati media yang sudah terkompresi menurut ekstensi.
settings-crypt-encryption-mode = Enkripsi
settings-crypt-encryption-off = Mati
settings-crypt-encryption-passphrase = Frasa sandi (tanya saat penyalinan dimulai)
settings-crypt-encryption-recipients = Kunci penerima dari berkas
settings-crypt-encryption-hint = Frasa sandi hanya disimpan di memori selama penyalinan berlangsung. Berkas penerima mencantumkan satu kunci publik age1… atau ssh- per baris.
settings-crypt-recipients-file = Jalur berkas penerima
settings-crypt-recipients-file-placeholder = mis. C:\Users\me\recipients.txt
settings-crypt-compression-mode = Kompresi
settings-crypt-compression-off = Mati
settings-crypt-compression-always = Selalu
settings-crypt-compression-smart = Cerdas (lewati media yang sudah terkompresi)
settings-crypt-compression-hint = Mode cerdas melewati jpg, mp4, zip, 7z dan format serupa yang tidak diuntungkan oleh zstd. Mode selalu mengompresi setiap berkas pada tingkat yang dipilih.
settings-crypt-compression-level = Tingkat zstd (1-22)
settings-crypt-compression-level-hint = Angka lebih rendah lebih cepat; angka lebih tinggi mengompresi lebih kuat. Tingkat 3 menyamai default CLI zstd.
compress-footer-savings = 💾 { $original } → { $compressed } ({ $percent }% dihemat)
compress-savings-toast = Terkompresi { $percent }% ({ $bytes } dihemat)
crypt-toast-recipients-loaded = Memuat { $count } penerima enkripsi
crypt-toast-recipients-error = Gagal memuat penerima: { $reason }
crypt-toast-passphrase-required = Enkripsi memerlukan frasa sandi sebelum penyalinan dimulai
crypt-toast-passphrase-set = Frasa sandi enkripsi ditangkap
crypt-footer-encrypted-badge = 🔒 Terenkripsi (age)
crypt-footer-compressed-badge = 📦 Terkompresi (zstd)

# Phase 36 — copythat CLI surface. Documented exit codes + the
# user-facing strings for plan/apply/verify/config. Help text stays
# in English (engineering accessibility) but error / info / status
# strings are localized so a localized desktop installation surfaces
# CLI errors in the same language as the GUI.
cli-help-tagline = Copy That CLI — penyalinan, sinkronisasi, verifikasi, dan audit berkas byte-exact untuk pipeline CI/CD.
cli-help-exit-codes = Kode keluar: 0 berhasil, 1 galat, 2 tertunda, 3 tabrakan, 4 verify-fail, 5 net, 6 perm, 7 disk-full, 8 batal, 9 config.
cli-error-bad-args = copy/move memerlukan setidaknya satu sumber dan satu tujuan
cli-error-unknown-algo = Algoritma verifikasi tidak diketahui: { $algo }
cli-error-missing-spec = --spec diperlukan untuk plan/apply
cli-error-spec-parse = Gagal mengurai jobspec { $path }: { $reason }
cli-error-spec-empty-sources = Daftar sumber jobspec kosong
cli-info-shape-recorded = Bentuk bandwidth "{ $rate }" tercatat; penegakan disalurkan via copythat-shape
cli-info-stub-deferred = { $command } disiapkan untuk penyambungan lanjutan Fase 36
cli-plan-summary = Rencana: { $actions } tindakan, { $bytes } bita; { $already_done } sudah pada tempatnya
cli-plan-pending = Rencana melaporkan tindakan tertunda; jalankan ulang dengan `apply` untuk mengeksekusi
cli-plan-already-done = Rencana melaporkan tidak ada yang perlu dilakukan (idempoten)
cli-apply-success = Apply selesai tanpa galat
cli-apply-failed = Apply selesai dengan satu galat atau lebih
cli-verify-ok = Verifikasi ok: { $algo } { $digest }
cli-verify-failed = Verifikasi GAGAL untuk { $path } ({ $algo })
cli-config-set = Atur { $key } = { $value }
cli-config-reset = Atur ulang { $key } ke default
cli-config-unknown-key = Kunci config tidak diketahui: { $key }
cli-completions-emitted = Penyelesaian shell untuk { $shell } dicetak ke stdout

# Phase 37 — desktop-side mobile companion. Settings → Mobile panel
# strings + the SAS-confirmation modal + push-notification toasts.
# The actual mobile UI lives in the Phase 37 follow-up Tauri Mobile
# target; the phone displays its own localized SAS prompt.
settings-mobile-heading = Pendamping seluler
settings-mobile-hint = Pasangkan iPhone atau ponsel Android untuk menelusuri riwayat, memulai profil tersimpan dan jobspec Fase 36, serta menerima notifikasi penyelesaian.
settings-mobile-pair-toggle = Izinkan pemasangan baru
settings-mobile-pair-active = Server pasangan aktif — pindai QR dengan aplikasi seluler Copy That
settings-mobile-pair-button = Mulai pemasangan
settings-mobile-revoke-button = Cabut
settings-mobile-no-pairings = Belum ada perangkat berpasangan
settings-mobile-pair-port = Port pengikatan (0 = pilih yang bebas)
pair-sas-prompt = Kedua layar harus menampilkan empat emoji yang sama. Ketuk Cocok jika sesuai.
pair-sas-confirm = Cocok
pair-sas-reject = Tidak cocok — batalkan
pair-toast-success = Berpasangan dengan { $device }
pair-toast-failed = Pemasangan gagal: { $reason }
push-toast-sent = Push terkirim ke { $device }
push-toast-failed = Push ke { $device } gagal: { $reason }

# Phase 38 — aggregate destination dedup + reflink fallback ladder.
# Settings → Transfer → Dedup panel + per-job-row badges (⚡
# Reflinked / 🔗 Hardlinked / 🧩 Chunk-shared / 📋 Copied) +
# pre-pass dedup-scan modal.
settings-dedup-heading = Dedup tujuan
settings-dedup-hint = Saat sumber dan tujuan berbagi volume, Copy That dapat mengklona berkas di tingkat sistem berkas alih-alih menyalin bita. Reflink instan + aman; hardlink lebih cepat tetapi kedua nama berbagi keadaan.
settings-dedup-mode-auto = Tangga otomatis (reflink → hardlink → chunk → salin)
settings-dedup-mode-reflink-only = Reflink saja
settings-dedup-mode-hardlink-aggressive = Agresif (reflink + hardlink bahkan pada berkas yang dapat ditulis)
settings-dedup-mode-off = Dinonaktifkan (selalu salin bita)
settings-dedup-hardlink-policy = Kebijakan hardlink
settings-dedup-prescan = Pra-pindai pohon tujuan untuk konten duplikat
dedup-badge-reflinked = ⚡ Reflinked
dedup-badge-hardlinked = 🔗 Hardlinked
dedup-badge-chunk-shared = 🧩 Chunk-shared
dedup-badge-copied = 📋 Disalin
phase42-paranoid-verify-label = Verifikasi paranoid
phase42-paranoid-verify-hint = Membuang halaman cache tujuan dan membaca ulang dari disk untuk menangkap kebohongan write-cache dan korupsi diam-diam. Sekitar 50% lebih lambat dari verifikasi default; mati secara default.
phase42-sharing-violation-retries-label = Upaya coba ulang pada berkas sumber terkunci
phase42-sharing-violation-retries-hint = Berapa kali mencoba ulang saat proses lain memegang berkas sumber terbuka dengan kunci eksklusif. Backoff menggandakan tiap upaya (50 md / 100 md / 200 md secara default). Default 3, menyamai Robocopy /R:3.
phase42-cloud-placeholder-warning = { $name } adalah berkas OneDrive khusus-cloud. Menyalinnya akan memicu unduhan — hingga { $size } melalui koneksi jaringan Anda.
phase42-defender-exclusion-hint = Untuk throughput penyalinan maksimum, tambahkan folder tujuan ke pengecualian Microsoft Defender sebelum transfer massal. Lihat docs/PERFORMANCE_TUNING.md.

# Phase 39 — Browser-accessible recovery UI. Settings → Advanced
# exposes these strings; the recovery server itself renders askama
# templates that always read English — these eight keys are the
# Settings prose the user actually reads in their preferred locale.
settings-recovery-heading = UI web pemulihan
settings-recovery-enable = Aktifkan UI web pemulihan
settings-recovery-bind-address = Alamat pengikatan
settings-recovery-port = Port (0 = pilih yang bebas)
settings-recovery-show-url = Tampilkan URL & token
settings-recovery-rotate-token = Putar token
settings-recovery-allow-non-loopback = Izinkan pengikatan non-loopback
settings-recovery-non-loopback-warning = PERINGATAN: mengaktifkan pengikatan non-loopback mengekspos UI pemulihan ke jaringan lokal Anda. Siapa pun yang mengetahui token dapat menelusuri riwayat berkas dan mengunduh berkas. Lapisi dengan TLS atau reverse proxy jika LAN tidak tepercaya.

# Phase 40 — SMB compression negotiation + cloud-VM offload helper.
# 6 SMB keys (header badge + Settings prose) + 6 cloud-offload keys
# (Remotes tab wizard for cross-cloud copy templates).
smb-compress-badge = 🗜 Kompres SMB: { $algo }
smb-compress-badge-tooltip = Lalu lintas jaringan ke tujuan ini sedang dikompresi saat transit (SMB 3.1.1).
smb-compress-toast-saved = Hemat { $bytes } melalui jaringan
smb-compress-algo-unknown = algoritma tidak diketahui
settings-smb-compress-heading = Kompresi jaringan SMB
settings-smb-compress-hint = Otomatis menegosiasikan kompresi lalu lintas SMB 3.1.1 pada tujuan UNC. Keuntungan gratis di tautan lambat; diabaikan pada tujuan lokal.
cloud-offload-heading = Helper offload Cloud-VM
cloud-offload-hint = Saat menyalin langsung antara dua cloud, render templat penyebaran yang menjalankan penyalinan dari VM efemeral mungil di cloud — bita tidak pernah menyentuh jaringan laptop Anda.
cloud-offload-render-button = Render templat
cloud-offload-copy-clipboard = Salin ke papan klip
cloud-offload-template-format = Format templat
cloud-offload-self-destruct-warning = VM mati otomatis setelah { $minutes } menit — konfirmasi peran IAM + wilayah sebelum menyebarkan.

# Phase 41 — animated before/after tree-diff preview. The `Preview
# changes` modal renders the rolled-up plan before the engine starts
# work; 14 keys cover the title, the summary header / counts, the
# row-reason labels, and the two action buttons.
preview-modal-title = Pratinjau perubahan
preview-summary-header = Apa yang akan terjadi
preview-category-additions = { $count } penambahan
preview-category-replacements = { $count } penggantian
preview-category-skips = { $count } dilewati
preview-category-conflicts = { $count } konflik
preview-category-unchanged = { $count } tidak berubah
preview-bytes-to-transfer = { $bytes } untuk ditransfer
preview-reason-source-newer = Sumber lebih baru
preview-reason-dest-newer = Tujuan lebih baru — akan dilewati
preview-reason-content-different = Konten berbeda
preview-reason-identical = Identik dengan sumber
preview-button-run = Jalankan rencana
preview-button-reduce = Kurangi rencana saya…

# Phase 42 — perceptual-hash visual-similarity dedup. Eight keys cover
# the pre-copy "looks visually identical" warning + the Settings panel.
perceptual-warn-title = Tampak identik secara visual
perceptual-warn-body = { $name } di tujuan tampak cocok dengan gambar sumber. Tetap lanjutkan menyalin?
perceptual-warn-keep-both = Simpan keduanya
perceptual-warn-skip = Lewati berkas ini
perceptual-warn-overwrite = Tetap timpa
perceptual-settings-heading = Dedup kemiripan visual
perceptual-settings-hint = Deteksi gambar identik secara visual di tujuan sebelum ditimpa. Hash bersifat perseptual (mengenali gambar yang sama disimpan ulang dalam format berbeda), bukan byte-exact.
perceptual-settings-threshold-label = Ambang peringatan (lebih rendah = kecocokan lebih ketat)

# Phase 42 Part B — per-file rolling versions (Time Machine for any
# destination). 8 keys cover the version-list panel + retention picker.
version-list-heading = Versi sebelumnya
version-list-empty = Tidak ada versi terdahulu dari berkas ini
version-list-restore = Pulihkan versi ini
version-retention-heading = Simpan versi sebelumnya saat menimpa
version-retention-none = Simpan setiap versi selamanya
version-retention-last-n = Simpan { $n } versi terakhir
version-retention-older-than-days = Buang versi yang lebih lama dari { $days } hari
version-retention-gfs = Per jam { $h } · harian { $d } · mingguan { $w } · bulanan { $m }

# Phase 43 — forensic chain-of-custody manifests + BLAKE3 verified
# streaming. 14 keys cover the Settings → Provenance panel
# (heading, hint, toggles, TSA URL, signing-key management) plus
# the post-job manifest notification + the verify command's two
# headline result lines.
provenance-settings-heading = Rantai pengawasan forensik
provenance-settings-hint = Tanda tangani setiap tugas penyalinan dengan manifes BLAKE3 + ed25519. Peninjau dapat menghitung ulang hash pohon tujuan nanti dan membuktikan tidak ada bita yang berubah sejak penyalinan.
provenance-settings-enable-default = Tanda tangani setiap tugas baru secara default
provenance-settings-show-after-job = Tampilkan manifes setelah setiap tugas selesai
provenance-settings-tsa-url-label = URL otoritas stempel waktu RFC 3161 default
provenance-settings-tsa-url-hint = Opsional. Saat diatur, manifes membawa stempel waktu TSA gratis yang membuktikan bita ada pada titik waktu ini. Kosongkan untuk melewati.
provenance-settings-keys-heading = Kunci penanda tangan
provenance-settings-keys-generate = Buat kunci baru
provenance-settings-keys-import = Impor kunci…
provenance-settings-keys-export = Ekspor kunci publik…
provenance-job-completed-title = Manifes provenans disimpan
provenance-job-completed-body = { $count } berkas ditandatangani → { $path }
provenance-verify-clean = Manifes valid untuk { $count } berkas; tanda tangan { $sig }; akar merkle OK.
provenance-verify-tampered = Manifes TIDAK VALID — { $tampered } dirusak, { $missing } hilang.
# Phase 43 post-review hardening — toast text for the Settings →
# Provenance buttons whose Tauri IPC has not yet landed.
provenance-action-staged = Fase 43 — penyambungan IPC untuk tindakan ini hadir di commit lanjutan.

# Phase 44 — SSD-aware whole-drive sanitize (NVMe Sanitize / OPAL
# Crypto Erase) + the per-file shred refusal on copy-on-write
# filesystems. 16 keys cover the new "Drive sanitize" Settings
# subsection and the localized error messages.
sanitize-heading = Sanitasi aman seluruh drive
sanitize-hint = NVMe Sanitize, OPAL Crypto Erase, dan ATA Secure Erase menghapus drive flash di lapisan firmware dalam milidetik. Penimpaan per berkas tidak berarti pada flash — shred multi-lintasan hanya membakar NAND. Gunakan ini untuk pembersihan tuntas.
sanitize-pick-device = Pilih drive yang akan disanitasi
sanitize-mode-label = Metode sanitasi
sanitize-mode-nvme-format = NVMe Format (dengan secure erase)
sanitize-mode-nvme-sanitize-block = NVMe Sanitize — Block Erase (lambat, setiap sel)
sanitize-mode-nvme-sanitize-crypto = NVMe Sanitize — Crypto Erase (instan)
sanitize-mode-ata-secure-erase = ATA Secure Erase (SSD SATA lawas)
sanitize-mode-opal-crypto-erase = TCG OPAL Crypto Erase (Self-Encrypting Drives)
sanitize-mode-apfs-crypto-erase = APFS Crypto Erase (putar kunci FileVault, macOS saja)
sanitize-confirm-1 = Ini menghancurkan SETIAP bita di { $device }. Tidak ada pembatalan.
sanitize-confirm-2 = Saya memahami bahwa semua partisi, semua berkas, dan semua snapshot di { $device } akan tidak terbaca secara permanen.
sanitize-confirm-3 = Ketik nama model drive untuk melanjutkan: { $model }
sanitize-running = Menyanitasi { $device } ({ $mode }) — ini dapat memakan waktu dari milidetik (crypto erase) hingga puluhan menit (block erase). Jangan matikan daya.
sanitize-completed = Sanitasi selesai — { $device } sekarang kosong.
ssd-honest-shred-meaningless = Shred per berkas pada sistem berkas copy-on-write (Btrfs / ZFS / APFS) tidak dapat menjangkau blok yang mendasarinya. Gunakan sanitasi seluruh drive plus rotasi kunci enkripsi seluruh disk sebagai gantinya.
ssd-honest-advisory = Berkas ini berada di flash. Penimpaan per berkas menghabiskan keausan NAND dan TIDAK menjamin sel asli tidak dapat dipulihkan. Untuk data sensitif, sanitasi seluruh drive.

# Phase 44.1f post-review — placeholder toast for SanitizeTab
# buttons whose Tauri IPC has not yet landed.
sanitize-action-staged = Fase 44.1 — penyambungan IPC untuk tindakan ini hadir di commit lanjutan.

# Phase 45.3 — named-queue tab strip (Subfeature A). Tabs surface
# once the QueueRegistry holds at least one queue; the synthesised
# default tab keeps legacy single-queue jobs reachable.
queue-tab-default = Default
queue-tab-empty-state = Antrean tugas
queue-badge-tooltip = Tugas tertunda dan berjalan di antrean ini

# Phase 45.4 — drag-progress-merge (Subfeature B). Drag a queue tab
# onto another to merge their job lists. The default tab is neither
# draggable nor a drop target; only registry queues participate.
queue-drag-hint = Seret ke antrean lain untuk menggabungkan
queue-merge-confirm = Jatuhkan untuk menggabungkan
queue-merge-toast = Antrean digabungkan

# Phase 45.5 — F2-queue UX (Subfeature C). F2 toggles
# `auto_enqueue_next` so every fresh enqueue piles into the running
# queue rather than spawning a parallel one. Status pill renders in
# the Footer; pulsing dot renders on the running tab.
queue-f2-active-hint = Mode F2: setiap antrean baru masuk ke antrean ini
queue-f2-toggled-on = Mode antrean F2 AKTIF — antrean baru bergabung ke antrean berjalan
queue-f2-toggled-off = Mode antrean F2 NONAKTIF — antrean baru memunculkan antrean paralel
queue-f2-status-bar = Mode antrean F2: AKTIF

# Phase 45.6 — tray destination targets (Subfeature D). Pinned
# destinations appear in the OS tray menu; clicking one arms it as
# the active drop target so the next file drop bypasses the
# DropStagingDialog. Settings → General hosts the list editor.
tray-target-section-title = Tujuan baki
tray-target-section-hint = Tujuan yang disematkan muncul di menu baki. Klik salah satu untuk menjadikannya target jatuh berikutnya.
tray-target-empty = Belum ada tujuan baki yang disematkan.
tray-target-remove = Hapus
tray-target-add-label = Label
tray-target-add-path = Jalur atau URI backend
tray-target-add = Tambah
tray-target-armed-toast = Jatuhkan berkas berikutnya untuk mengirimnya ke { $label }
tray-target-active-pill = → { $label }

# Phase 45.7 follow-up — pinned-destination validation errors. The
# `queue_pin_destination` IPC returns these Fluent keys verbatim
# when input fails the IPC-boundary checks; the toast layer renders
# them via `t(...)` (Toast.svelte detects kebab-case-lowercase and
# routes through the locale table). Phase 17e's `err-path-escape` /
# `err-destination-empty` cover the path-traversal + empty cases for
# `queue_route_job`; the keys below are pin-specific.
err-pinned-destination-label-empty = Label tujuan baki tidak boleh kosong.
err-pinned-destination-path-empty = Jalur tujuan baki tidak boleh kosong.
err-pinned-destination-label-too-long = Label tujuan baki terlalu panjang (maks 64 karakter).
err-pinned-destination-path-too-long = Jalur tujuan baki terlalu panjang (maks 1024 karakter).
err-pinned-destination-label-invalid = Label tujuan baki berisi karakter yang tidak diizinkan (baris baru, return, atau NUL).
err-pinned-destination-path-invalid = Jalur tujuan baki berisi karakter yang tidak diizinkan (baris baru, return, atau NUL).
err-pinned-destination-too-many = Anda telah mencapai batas 50 tujuan baki. Hapus satu untuk menambah yang lain.

# Phase 46.6 — Settings → Plugins tab. The IPC layer in
# `apps/copythat-ui/src-tauri/src/plugin_commands.rs` enumerates the
# per-user plugin store under `<config_dir>/plugins/`; the
# `PluginsTab.svelte` component renders these strings against the
# returned manifest + grant state.
settings-tab-plugins = Plugin
plugin-heading = Plugin
plugin-hint = Plugin WASM tersandbox memperluas Copy That dengan hook khusus. Setiap plugin berjalan di bawah batas CPU dan memori per panggilan dan hanya melihat kapabilitas host yang Anda berikan.
plugin-list-empty = Belum ada plugin terpasang.
plugin-enabled = Aktif
plugin-disabled = Nonaktif
plugin-hooks = Hook
plugin-capabilities = Kapabilitas
plugin-no-capabilities = (tidak ada)
plugin-directory = Lokasi
plugin-install-from-file = Pasang dari berkas…
plugin-install-from-url = Pasang dari URL…
plugin-url-wasm = URL WASM
plugin-url-manifest = URL manifes
plugin-url-hash = Hash BLAKE3
plugin-url-preview = Pratinjau
plugin-url-confirm = Konfirmasi pemasangan

# Phase 31b — power-policy settings (Power tab).
settings-tab-power = Daya
settings-power-hint = Batasi atau jeda penyalinan berdasarkan status daya: baterai, jaringan berkuota/seluler, saat presentasi/layar penuh, atau saat CPU throttling termal.
settings-power-enabled = Aktifkan pembatasan berbasis daya
settings-power-battery = Saat memakai baterai
settings-power-metered = Di jaringan berkuota
settings-power-cellular = Di jaringan seluler
settings-power-presentation = Saat presentasi
settings-power-fullscreen = Saat layar penuh
settings-power-thermal = Saat throttling termal
settings-power-continue = Lanjutkan
settings-power-pause = Jeda
err-server-not-implemented = Mode server belum tersedia.
err-webhook-not-implemented = Pengiriman webhook belum tersedia.

# Phase 47 — "why is this slow?" diagnostics (bottleneck badge + tooltip).
bottleneck-source-io = Sumber I/O
bottleneck-dest-io = Tujuan I/O
bottleneck-network = Jaringan
bottleneck-antivirus = Antivirus
bottleneck-cpu = CPU
bottleneck-thermal = Termal
bottleneck-unknown = Tidak diketahui
diag-aria = Hambatan: { $cause }
diag-tooltip = Dibatasi oleh { $cause } · { $rate }
diag-spark-aria = Throughput dalam satu menit terakhir
diag-keeping-up = Mengimbangi
diag-label = Diagnostik

# Phase 48 — server mode + observability (Settings → Server).
settings-tab-server = Server
server-hint = Jalankan Copy That sebagai server berkas tanpa antarmuka. Pilih protokol yang akan diekspos, atur alamat dan folder yang akan dilayani, dan secara opsional wajibkan autentikasi.
server-protocols = Protokol
server-bind-addr = Alamat bind
server-root = Folder yang dilayani
server-readonly = Hanya-baca (tolak unggahan dan penghapusan)
server-auth-mode = Autentikasi
server-auth-none = Tidak ada
server-auth-bearer = Token Bearer
server-auth-basic = Dasar (nama pengguna + kata sandi)
server-auth-token = Token
server-auth-user = Nama pengguna
server-auth-password = Kata sandi
otel-endpoint = Endpoint OpenTelemetry
webhook-section = Webhook
webhook-url = URL webhook
webhook-add = Tambah webhook
webhook-remove = Hapus
webhook-empty = Tidak ada webhook yang dikonfigurasi.
webhook-pushover-token = Token Pushover
webhook-pushover-user = Pengguna Pushover
server-start = Mulai server
server-stop = Hentikan server
server-status-running = Berjalan di { $addr }
server-status-stopped = Dihentikan
server-metrics-url = Metrik
err-server-no-protocols = Pilih setidaknya satu protokol sebelum memulai server.
err-server-bind = Tidak dapat mengikat alamat server. Mungkin sudah digunakan.

# Laci Pustaka (Phase 49) — tampilan repositori beralamat konten terpadu.
footer-library = Pustaka
library-title = Pustaka
library-loading = Memuat repositori…
library-unavailable = Repositori tidak tersedia
library-tab-live = Langsung
library-tab-snapshots = Snapshot
library-tab-versions = Versi
library-hero-savings = Menyajikan { $effective } efektif · { $pct } dihemat
library-hero-empty = { $chunks } chunk tersimpan — belum ada snapshot
library-stat-stored = Tersimpan di disk
library-stat-effective = Data efektif
library-stat-snapshots = Snapshot
library-stat-chunks = Chunk unik
library-snapshot-empty = Belum ada snapshot
library-snapshot-files = { $n } berkas
library-version-path-ph = Jalur tujuan…
library-version-load = Tampilkan versi
library-version-empty = Tidak ada versi untuk jalur ini
repo-kind-copy = Salin
repo-kind-sync = Sinkron
repo-kind-version = Versi
repo-kind-backup = Cadangan
# Phase 49o — snapshot diff / compare.
library-tab-compare = Bandingkan
repo-change-added = Ditambahkan
repo-change-removed = Dihapus
repo-change-modified = Diubah
repo-change-unchanged = Tidak ada perubahan
repo-diff-summary = { $added } ditambahkan · { $removed } dihapus · { $modified } diubah
repo-diff-bytes-added = { $bytes } baru
repo-diff-pick-two = Pilih dua snapshot untuk dibandingkan
# Phase 49r — statistics / reports.
library-tab-reports = Laporan
report-growth-title = Pertumbuhan penyimpanan
report-by-kind-title = Menurut jenis
report-top-files-title = File teratas
report-dedup-ratio = { $pct }% terdeduplikasi
report-export = Ekspor laporan
report-exported = Laporan disimpan ke { $path }
report-file-versions = { $n } versi
# Phase 49p — pinning / prune.
repo-pin = Sematkan
repo-unpin = Lepas sematan
repo-pinned-badge = Tersemat
repo-prune-title = Pangkas
repo-prune-keep-last = Simpan terbaru
repo-prune-removed = { $n } snapshot dipangkas
repo-prune-none = Tidak ada yang dipangkas

# Phase 49c — sumber cadangan.
library-tab-sources = Sumber
backup-add-source = Tambah sumber…
backup-source-path-ph = Folder untuk dicadangkan…
backup-exclude-ph = Glob pengecualian (dipisahkan koma)
backup-now = Cadangkan sekarang
backup-remove = Hapus
backup-empty = Belum ada sumber cadangan
backup-never-run = Belum pernah dicadangkan
backup-last-run = Pencadangan terakhir { $when }
backup-running = Mencadangkan… { $files } berkas
backup-toast-started = Mencadangkan { $label }…
backup-toast-completed = { $label } dicadangkan: { $files } berkas
backup-toast-failed = Pencadangan { $label } gagal: { $reason }
# Phase 49e — per-source retention + prune.
backup-retention = Retensi
backup-retention-keep-all = Simpan semua
backup-retention-last = Simpan { $n } terakhir
backup-retention-days = Lebih lama dari { $days } hari
backup-retention-gfs = Rotasi GFS
backup-prune-now = Pangkas sekarang
backup-prune-none = Tidak ada yang dipangkas
backup-prune-result = { $removed } snapshot dihapus · { $bytes } dipulihkan
# Phase 49f — per-source scheduling.
backup-schedule = Jadwal
backup-schedule-manual = Manual
backup-schedule-hourly = Setiap jam
backup-schedule-daily = Harian
backup-schedule-weekly = Mingguan
backup-next-run = Eksekusi berikutnya { $when }
backup-not-scheduled = Tidak dijadwalkan
# Phase 49g — source filters.
backup-include-ph = Sertakan glob (dipisah koma)
backup-skip-hidden = Lewati tersembunyi
# Phase 49q — notifications.
notify-title = Notifikasi
notify-on-success = Saat berhasil
notify-on-failure = Saat gagal
notify-test = Kirim tes
notify-test-sent = Tes dikirim ke { $n } tujuan

# Phase 49d — penjelajah pemulihan.
restore-browse = Pulihkan…
restore-title = Pulihkan dari snapshot
restore-select-all = Pilih semua
restore-dest = Pulihkan ke
restore-confirm = Pulihkan { $n } berkas
restore-empty = Snapshot ini tidak memiliki berkas
restore-conflict-body = { $count } berkas terpilih sudah ada di tujuan.
restore-conflict-overwrite = Timpa
restore-conflict-skip = Lewati yang ada
restore-conflict-keep-both = Simpan keduanya
restore-toast-done = { $restored } dipulihkan, { $skipped } dilewati
restore-toast-failed = Pemulihan gagal: { $reason }
snapshot-forget = Lupakan
snapshot-forget-toast = Snapshot dilupakan — jalankan Klaim ulang ruang untuk membebaskannya
library-reclaim = Klaim ulang ruang
# Phase 49i — full compaction.
library-compact = Pemadatan penuh
library-compact-started = Pemadatan dimulai — lihat Tugas
# Phase 49h — compression.
library-stat-compression = Dihemat oleh kompresi
storage-compression = Kompresi
storage-compression-off = Mati
storage-compression-auto = Otomatis (lewati yang tidak dapat dikompresi)
storage-compression-always = Selalu
storage-compression-restart = Berlaku saat peluncuran berikutnya
# Phase 49j — tasks & progress center.
footer-tasks = Tugas
tasks-title = Tugas
tasks-empty = Belum ada tugas
tasks-running = Berjalan
tasks-recent = Terbaru
tasks-cancel = Batal
task-state-running = Berjalan
task-state-completed = Selesai
task-state-failed = Gagal
task-state-cancelled = Dibatalkan
# Phase 49k — repository setup/connect wizard.
repo-wizard-title = Hubungkan repositori
repo-wizard-create-tab = Buat baru
repo-wizard-connect-tab = Hubungkan yang ada
repo-field-name = Nama
repo-field-path = Lokasi
repo-field-password = Frasa sandi (opsional)
repo-action-create = Buat
repo-action-connect = Hubungkan
repo-action-browse = Telusuri…
repo-switcher-label = Repositori
repo-action-forget = Lupakan
repo-action-change-pass = Ubah frasa sandi…
repo-password-old = Frasa sandi saat ini
repo-password-new = Frasa sandi baru
repo-error-exists = Sudah ada repositori di lokasi ini
repo-error-not-found = Tidak ada repositori yang ditemukan di lokasi ini
repo-error-bad-pass = Frasa sandi salah
repo-note-no-encryption = Frasa sandi hanya mengatur akses; enkripsi data tersimpan akan hadir di rilis berikutnya
repo-confirm-forget = Hapus "{ $name }" dari daftar? Data Anda tetap ada di disk.
repo-toast-created = Repositori "{ $name }" dibuat
repo-toast-connected = Terhubung ke "{ $name }"
repo-toast-pass-changed = Frasa sandi diperbarui
# Phase 49l — Sources dashboard.
library-tab-overview = Ikhtisar
library-source-empty = Belum ada sumber
library-source-unknown = (sumber tidak ditentukan)
library-source-snapshots = { $n } snapshot
library-source-latest = Terbaru { $when }
# Phase 49n — verify & repair.
repo-action-verify = Verifikasi
repo-action-verify-deep = Verifikasi (baca semua data)
repo-action-repair = Perbaiki…
repo-verify-clean = { $files } berkas / { $chunks } potongan diverifikasi — tidak ada kerusakan
repo-verify-damaged = { $missing } hilang, { $corrupt } potongan rusak
repo-repair-confirm = Hapus { $n } snapshot yang tidak dapat dipulihkan lagi?
repo-repair-removed = { $n } snapshot rusak dihapus
repo-repair-none = Tidak ada yang perlu diperbaiki — repositori bersih
repo-gc-done = { $bytes } diklaim ulang ({ $chunks } potongan)
restore-toast-partial = { $restored } dipulihkan, { $skipped } dilewati, { $failed } gagal
