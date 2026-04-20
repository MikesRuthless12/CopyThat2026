app-name = Copy That 2026
# MT
window-title = Copy That 2026
# MT
shred-ssd-advisory = Uyarı: bu hedef bir SSD üzerindedir. Yıpranma dengeleme ve aşırı tahsis, verileri mantıksal blok adresinin dışına taşıdığından, çok geçişli üzerine yazma işlemleri flash belleği güvenilir biçimde temizleyemez. Katı hal ortamları için ATA SECURE ERASE, Güvenli Silme ile NVMe Format veya anahtarı imha edilmiş tam disk şifrelemesini tercih edin.

# MT
state-idle = Boşta
# MT
state-copying = Kopyalanıyor
# MT
state-verifying = Doğrulanıyor
# MT
state-paused = Duraklatıldı
# MT
state-error = Hata

# MT
state-pending = Kuyrukta
# MT
state-running = Çalışıyor
# MT
state-cancelled = İptal edildi
# MT
state-succeeded = Tamamlandı
# MT
state-failed = Başarısız

# MT
action-pause = Duraklat
# MT
action-resume = Devam et
# MT
action-cancel = İptal et
# MT
action-pause-all = Tüm işleri duraklat
# MT
action-resume-all = Tüm işlere devam et
# MT
action-cancel-all = Tüm işleri iptal et
# MT
action-close = Kapat
# MT
action-reveal = Klasörde göster

# MT
menu-pause = Duraklat
# MT
menu-resume = Devam et
# MT
menu-cancel = İptal et
# MT
menu-remove = Kuyruktan kaldır
# MT
menu-reveal-source = Kaynağı klasörde göster
# MT
menu-reveal-destination = Hedefi klasörde göster

# MT
header-eta-label = Tahmini kalan süre
# MT
header-toolbar-label = Genel denetimler

# MT
footer-queued = etkin iş
# MT
footer-total-bytes = devam ediyor
# MT
footer-errors = hata
# MT
footer-history = Geçmiş

# MT
empty-title = Kopyalanacak dosya veya klasörleri bırakın
# MT
empty-hint = Öğeleri pencereye sürükleyin. Hedef soracağız, ardından her kaynak için bir iş kuyruğa ekleyeceğiz.
# MT
empty-region-label = İş listesi

# MT
details-drawer-label = İş ayrıntıları
# MT
details-source = Kaynak
# MT
details-destination = Hedef
# MT
details-state = Durum
# MT
details-bytes = Bayt
# MT
details-files = Dosyalar
# MT
details-speed = Hız
# MT
details-eta = Kalan süre
# MT
details-error = Hata

# MT
drop-dialog-title = Bırakılan öğeleri aktar
# MT
drop-dialog-subtitle = Aktarıma hazır { $count } öğe. Başlamak için bir hedef klasör seçin.
# MT
drop-dialog-mode = İşlem
# MT
drop-dialog-copy = Kopyala
# MT
drop-dialog-move = Taşı
# MT
drop-dialog-pick-destination = Hedef seç
# MT
drop-dialog-change-destination = Hedefi değiştir
# MT
drop-dialog-start-copy = Kopyalamaya başla
# MT
drop-dialog-start-move = Taşımaya başla

# MT
eta-calculating = hesaplanıyor…
# MT
eta-unknown = bilinmiyor

# MT
toast-job-done = Aktarım tamamlandı
# MT
toast-copy-queued = Kopyalama kuyruğa alındı
# MT
toast-move-queued = Taşıma kuyruğa alındı
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

# MT — Totals drawer (Phase 10)
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
