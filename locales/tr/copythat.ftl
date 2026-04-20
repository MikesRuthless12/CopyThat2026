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
# MT — Phase 8 toast messages
toast-error-resolved = Hata çözüldü
# MT
toast-collision-resolved = Çakışma çözüldü
# MT
toast-elevated-unavailable = Yükseltilmiş izinle yeniden deneme 17. aşamada gelecek — henüz kullanılamıyor
# MT
toast-error-log-exported = Hata günlüğü dışa aktarıldı

# MT — Error modal
error-modal-title = Bir aktarım başarısız oldu
# MT
error-modal-retry = Yeniden dene
# MT
error-modal-retry-elevated = Yükseltilmiş izinle yeniden dene
# MT
error-modal-skip = Atla
# MT
error-modal-skip-all-kind = Bu türdeki tüm hataları atla
# MT
error-modal-abort = Tümünü iptal et
# MT
error-modal-path-label = Yol
# MT
error-modal-code-label = Kod

# MT — Error-kind labels
err-not-found = Dosya bulunamadı
# MT
err-permission-denied = İzin reddedildi
# MT
err-disk-full = Hedef disk dolu
# MT
err-interrupted = İşlem kesildi
# MT
err-verify-failed = Kopya sonrası doğrulama başarısız
# MT
err-io-other = Bilinmeyen G/Ç hatası

# MT — Collision modal
collision-modal-title = Dosya zaten var
# MT
collision-modal-overwrite = Üzerine yaz
# MT
collision-modal-overwrite-if-newer = Daha yeniyse üzerine yaz
# MT
collision-modal-skip = Atla
# MT
collision-modal-keep-both = İkisini de sakla
# MT
collision-modal-rename = Yeniden adlandır…
# MT
collision-modal-apply-to-all = Tümüne uygula
# MT
collision-modal-source = Kaynak
# MT
collision-modal-destination = Hedef
# MT
collision-modal-size = Boyut
# MT
collision-modal-modified = Değiştirilme
# MT
collision-modal-hash-check = Hızlı özet (SHA-256)
# MT
collision-modal-rename-placeholder = Yeni dosya adı
# MT
collision-modal-confirm-rename = Yeniden adlandır

# MT — Error log drawer
error-log-title = Hata günlüğü
# MT
error-log-empty = Kaydedilmiş hata yok
# MT
error-log-export-csv = CSV dışa aktar
# MT
error-log-export-txt = Metin dışa aktar
# MT
error-log-clear = Günlüğü temizle
# MT
error-log-col-time = Zaman
# MT
error-log-col-job = İş
# MT
error-log-col-path = Yol
# MT
error-log-col-code = Kod
# MT
error-log-col-message = Mesaj
# MT
error-log-col-resolution = Çözüm

# MT — History drawer (Phase 9)
history-title = Geçmiş
# MT
history-empty = Henüz kaydedilmiş iş yok
# MT
history-unavailable = Kopyalama geçmişi kullanılamıyor. Uygulama başlangıçta SQLite deposunu açamadı.
# MT
history-filter-any = herhangi
# MT
history-filter-kind = Tür
# MT
history-filter-status = Durum
# MT
history-filter-text = Ara
# MT
history-refresh = Yenile
# MT
history-export-csv = CSV dışa aktar
# MT
history-purge-30 = 30 günden eskiyi sil
# MT
history-rerun = Yeniden çalıştır
# MT
history-detail-open = Ayrıntılar
# MT
history-detail-title = İş ayrıntıları
# MT
history-detail-empty = Kaydedilmiş öğe yok
# MT
history-col-date = Tarih
# MT
history-col-kind = Tür
# MT
history-col-src = Kaynak
# MT
history-col-dst = Hedef
# MT
history-col-files = Dosyalar
# MT
history-col-size = Boyut
# MT
history-col-status = Durum
# MT
history-col-duration = Süre
# MT
history-col-error = Hata

# MT
toast-history-exported = Geçmiş dışa aktarıldı
# MT
toast-history-rerun-queued = Yeniden çalıştırma kuyruğa alındı

# MT — Totals drawer (Phase 10)
footer-totals = Toplamlar
# MT
totals-title = Toplamlar
# MT
totals-loading = Toplamlar yükleniyor…
# MT
totals-card-bytes = Toplam kopyalanan bayt
# MT
totals-card-files = Dosyalar
# MT
totals-card-jobs = İşler
# MT
totals-card-avg-rate = Ortalama hız
# MT
totals-errors = hatalar
# MT
totals-spark-title = Son 30 gün
# MT
totals-kinds-title = Türe göre
# MT
totals-saved-title = Kazanılan süre (tahmini)
# MT
totals-saved-note = Aynı iş yükünü standart dosya yöneticisiyle kopyalamaya kıyasla tahmini değerdir.
# MT
totals-reset = İstatistikleri sıfırla
# MT
totals-reset-confirm = Bu, depolanan tüm iş ve öğeleri siler. Devam edilsin mi?
# MT
totals-reset-confirm-yes = Evet, sıfırla
# MT
toast-totals-reset = İstatistikler sıfırlandı

# MT — Phase 11a additions
header-language-label = Dil
# MT
header-language-title = Dili değiştir

# MT
kind-copy = Kopyala
# MT
kind-move = Taşı
# MT
kind-delete = Sil
# MT
kind-secure-delete = Güvenli sil

# MT
status-running = Çalışıyor
# MT
status-succeeded = Başarılı
# MT
status-failed = Başarısız
# MT
status-cancelled = İptal edildi
# MT
status-ok = Tamam
# MT
status-skipped = Atlandı

# MT
history-search-placeholder = /yol
# MT
toast-history-purged = 30 günden eski { $count } iş silindi

# MT
err-source-required = En az bir kaynak yolu gerekli.
# MT
err-destination-empty = Hedef yolu boş.
# MT
err-source-empty = Kaynak yolu boş.

# MT
duration-lt-1s = < 1 sn
# MT
duration-ms = { $ms } ms
# MT
duration-seconds = { $s } sn
# MT
duration-minutes-seconds = { $m } dk { $s } sn
# MT
duration-hours-minutes = { $h } sa { $m } dk
# MT
duration-zero = 0 sn

# MT
rate-unit-per-second = { $size }/sn

# MT — Phase 11b Settings modal
settings-title = Ayarlar
# MT
settings-tab-general = Genel
# MT
settings-tab-appearance = Görünüm
# MT
settings-section-language = Dil
# MT
settings-phase-12-hint = Daha fazla ayar (tema, aktarım varsayılanları, doğrulama algoritması, profiller) 12. aşamada gelecek.
