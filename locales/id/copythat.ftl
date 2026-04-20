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
# MT — Phase 8 toast messages
toast-error-resolved = Kesalahan teratasi
# MT
toast-collision-resolved = Konflik teratasi
# MT
toast-elevated-unavailable = Coba lagi dengan izin tinggi akan hadir di Fase 17 — belum tersedia
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
