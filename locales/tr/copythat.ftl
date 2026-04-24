app-name = Copy That v1.25.0
# MT
window-title = Copy That v1.25.0
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
toast-clipboard-files-detected = Panoda dosyalar var — Copy That ile kopyalamak için yapıştırma kısayoluna basın
toast-clipboard-no-files = Panoda yapıştırılacak dosya yok
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
error-drawer-pending-count = Daha fazla hata bekliyor
error-drawer-toggle = Daralt veya genişlet

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
err-path-escape = Yol reddedildi — üst dizin (..) parçaları veya geçersiz baytlar içeriyor
# MT
err-io-other = Bilinmeyen G/Ç hatası
err-sparseness-mismatch = Hedefte seyrek düzen korunamadı  # MT

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

# MT — Phase 12 Settings window
settings-loading = Ayarlar yükleniyor…
# MT
settings-tab-transfer = Aktarım
# MT
settings-tab-shell = Kabuk
# MT
settings-tab-secure-delete = Güvenli silme
# MT
settings-tab-advanced = Gelişmiş
# MT
settings-tab-profiles = Profiller

# MT
settings-section-theme = Tema
# MT
settings-theme-auto = Otomatik
# MT
settings-theme-light = Açık
# MT
settings-theme-dark = Koyu
# MT
settings-start-with-os = Sistem başlangıcında çalıştır
# MT
settings-single-instance = Tek çalışan örnek
# MT
settings-minimize-to-tray = Kapatırken sistem tepsisine küçült
settings-error-display-mode = Hata istemi stili
settings-error-display-modal = Modal (uygulamayı engeller)
settings-error-display-drawer = Çekmece (engellemeyen)
settings-error-display-mode-hint = Modal, siz karar verene kadar kuyruğu durdurur. Çekmece kuyruğu çalışır durumda tutar ve hataları köşede ayıklamanıza izin verir.
settings-paste-shortcut = Global kısayolla dosyaları yapıştır
settings-paste-shortcut-combo = Kısayol kombinasyonu
settings-paste-shortcut-hint = Explorer / Finder / Dosyalar'dan kopyalanan dosyaları Copy That üzerinden yapıştırmak için sisteminizde herhangi bir yerde bu kombinasyona basın. CmdOrCtrl, macOS'ta Cmd'ye ve Windows / Linux'ta Ctrl'ye çözümlenir.
settings-clipboard-watcher = Kopyalanan dosyalar için panoyu izle
settings-clipboard-watcher-hint = Dosya URL'leri panoda göründüğünde bir bildirim gösterir ve Copy That ile yapıştırabileceğinizi belirtir. Etkinken her 500 ms'de bir sorgular.

# MT
settings-buffer-size = Arabellek boyutu
# MT
settings-verify = Kopyalama sonrası doğrula
# MT
settings-verify-off = Kapalı
# MT
settings-concurrency = Eşzamanlılık
# MT
settings-concurrency-auto = Otomatik
# MT
settings-reflink = Reflink / hızlı yollar
# MT
settings-reflink-prefer = Tercih et
# MT
settings-reflink-avoid = Reflinkten kaçın
# MT
settings-reflink-disabled = Her zaman async motoru kullan
# MT
settings-fsync-on-close = Kapanışta diske senkronize et (daha yavaş, daha güvenli)
# MT
settings-preserve-timestamps = Zaman damgalarını koru
# MT
settings-preserve-permissions = İzinleri koru
# MT
settings-preserve-acls = ACL'leri koru (14. aşama)
settings-preserve-sparseness = Seyrek dosyaları koru  # MT
settings-preserve-sparseness-hint = Seyrek dosyaların (VM diskleri, veritabanı dosyaları) yalnızca ayrılmış kapsamlarını kopyalayın; böylece hedefin diskteki boyutu kaynakla aynı kalır.  # MT

# MT
settings-context-menu = Bağlam menüsü girdilerini etkinleştir
# MT
settings-intercept-copy = Varsayılan kopya işleyicisini yakala (Windows)
# MT
settings-intercept-copy-hint = Açıkken Explorer'da Ctrl+C / Ctrl+V Copy That üzerinden çalışır. Kayıt 14. aşamada.
# MT
settings-notify-completion = İş tamamlandığında bildir

# MT
settings-shred-method = Varsayılan silme yöntemi
# MT
settings-shred-zero = Sıfır (1 geçiş)
# MT
settings-shred-random = Rastgele (1 geçiş)
# MT
settings-shred-dod3 = DoD 5220.22-M (3 geçiş)
# MT
settings-shred-dod7 = DoD 5220.22-M (7 geçiş)
# MT
settings-shred-gutmann = Gutmann (35 geçiş)
# MT
settings-shred-nist = NIST 800-88
# MT
settings-shred-confirm-twice = Silmeden önce çift onay iste

# MT
settings-log-level = Günlük düzeyi
# MT
settings-log-off = Kapalı
# MT
settings-telemetry = Telemetri
# MT
settings-telemetry-never = Asla — hiçbir günlük düzeyinde veri gönderilmez
# MT
settings-error-policy = Varsayılan hata ilkesi
# MT
settings-error-policy-ask = Sor
# MT
settings-error-policy-skip = Atla
# MT
settings-error-policy-retry = Bekleme ile yeniden dene
# MT
settings-error-policy-abort = İlk hatada iptal et
# MT
settings-history-retention = Geçmiş saklama (gün)
# MT
settings-history-retention-hint = 0 = süresiz tut. Başka herhangi bir değer başlangıçta eski işleri otomatik olarak temizler.
# MT
settings-database-path = Veritabanı yolu
# MT
settings-database-path-default = (varsayılan — OS veri dizini)
# MT
settings-reset-all = Varsayılanlara sıfırla
# MT
settings-reset-confirm = Tüm tercihler sıfırlansın mı? Profiller etkilenmez.

# MT
settings-profiles-hint = Mevcut ayarları bir ad altında kaydedin; daha sonra ayrı ayrı tekli düğmeleri değiştirmeden geri yüklemek için yükleyin.
# MT
settings-profile-name-placeholder = Profil adı
# MT
settings-profile-save = Kaydet
# MT
settings-profile-import = İçe aktar…
# MT
settings-profile-load = Yükle
# MT
settings-profile-export = Dışa aktar…
# MT
settings-profile-delete = Sil
# MT
settings-profile-empty = Henüz kaydedilmiş profil yok.
# MT
settings-profile-import-prompt = İçe aktarılan profil için ad:

# MT
toast-settings-reset = Ayarlar sıfırlandı
# MT
toast-profile-saved = Profil kaydedildi
# MT
toast-profile-loaded = Profil yüklendi
# MT
toast-profile-exported = Profil dışa aktarıldı
# MT
toast-profile-imported = Profil içe aktarıldı

# Phase 13d — activity feed + header picker buttons
action-add-files = Dosya ekle
action-add-folders = Klasör ekle
activity-title = Etkinlik
activity-clear = Etkinlik listesini temizle
activity-empty = Henüz dosya etkinliği yok.
activity-after-done = Bittiğinde:
activity-keep-open = Uygulamayı açık tut
activity-close-app = Uygulamayı kapat
activity-shutdown = Bilgisayarı kapat
activity-logoff = Oturumu kapat
activity-sleep = Uyku moduna al

# Phase 14 — preflight free-space dialog
preflight-block-title = Hedefte yeterli alan yok
preflight-warn-title = Hedefte az alan var
preflight-unknown-title = Boş alan belirlenemedi
preflight-unknown-body = Kaynak hızlı ölçülemeyecek kadar büyük veya hedef disk yanıt vermedi. Devam edebilirsiniz; alan biterse motor kopyalamayı temiz bir şekilde durdurur.
preflight-required = Gerekli
preflight-free = Boş
preflight-reserve = Rezerv
preflight-shortfall = Eksik
preflight-continue = Yine de devam et
collision-modal-overwrite-older = Yalnızca eskileri üzerine yaz

# Phase 14e — subset picker
preflight-pick-subset = Hangisinin kopyalanacağını seç…
subset-title = Kopyalanacak kaynakları seçin
subset-subtitle = Seçimin tamamı hedefe sığmıyor. Kopyalamak istediklerinizi işaretleyin; geri kalanlar atlanır.
subset-loading = Boyutlar ölçülüyor…
subset-too-large = sayılamayacak kadar büyük
subset-budget = Kullanılabilir
subset-remaining = Kalan
subset-confirm = Seçileni kopyala
history-rerun-hint = Bu kopyayı yeniden çalıştır — kaynak ağacındaki her dosyayı yeniden tarar
history-clear-all = Tümünü temizle
history-clear-all-confirm = Onaylamak için tekrar tıkla
history-clear-all-hint = Tüm geçmiş satırlarını siler. Onaylamak için ikinci bir tıklama gerekir.
toast-history-cleared = Geçmiş temizlendi ({ $count } satır kaldırıldı)

# Phase 15 — source-list ordering
drop-dialog-sort-label = Sıralama:
sort-custom = Özel
sort-name-asc = Ad A → Z (önce dosyalar)
sort-name-desc = Ad Z → A (önce dosyalar)
sort-size-asc = Boyut küçükten büyüğe (önce dosyalar)
sort-size-desc = Boyut büyükten küçüğe (önce dosyalar)
sort-reorder = Yeniden sırala
sort-move-top = En üste taşı
sort-move-up = Yukarı
sort-move-down = Aşağı
sort-move-bottom = En alta taşı
sort-name-asc-simple = Ad A → Z
sort-name-desc-simple = Ad Z → A
sort-size-asc-simple = Küçükten büyüğe
sort-size-desc-simple = Büyükten küçüğe
activity-sort-locked = Kopyalama sırasında sıralama devre dışıdır. Duraklatın veya bitmesini bekleyin, sonra sırayı değiştirin.
drop-dialog-collision-label = Dosya zaten varsa:
collision-policy-keep-both = İkisini de tut (yeni kopyayı _2, _3 … olarak yeniden adlandır)
collision-policy-skip = Yeni kopyayı atla
collision-policy-overwrite = Mevcut dosyanın üzerine yaz
collision-policy-overwrite-if-newer = Yalnızca daha yeniyse üzerine yaz
collision-policy-prompt = Her seferinde sor
drop-dialog-busy-checking = Boş alan kontrol ediliyor…
drop-dialog-busy-enumerating = Dosyalar sayılıyor…
drop-dialog-busy-starting = Kopyalama başlatılıyor…
toast-enumeration-deferred = Kaynak ağaç büyük — ön liste atlanıyor; satırlar motor işlediğinde görünecek.

# Phase 14a — enumeration-time filters
# MT
settings-tab-filters = Filtreler
# MT
settings-filters-hint = Dosyaları sıralama sırasında atlayarak motorun onları hiç açmamasını sağlar. Dahil etme yalnızca dosyalara uygulanır; hariç tutma eşleşen klasörleri de budar.
# MT
settings-filters-enabled = Ağaç kopyaları için filtreleri etkinleştir
# MT
settings-filters-include-globs = Dahil etme globları
# MT
settings-filters-include-globs-placeholder = **/*.txt
# MT
settings-filters-include-globs-hint = Satır başına bir glob. Boş değilse dosyanın en az biriyle eşleşmesi gerekir. Klasörlere daima girilir.
# MT
settings-filters-exclude-globs = Hariç tutma globları
# MT
settings-filters-exclude-globs-placeholder = **/node_modules
# MT
settings-filters-exclude-globs-hint = Satır başına bir glob. Klasör eşleşmeleri tüm alt ağacı budar; eşleşen dosyalar atlanır.
# MT
settings-filters-size-range = Dosya boyutu aralığı
# MT
settings-filters-min-size-bytes = Minimum boyut (bayt, boş = alt sınır yok)
# MT
settings-filters-max-size-bytes = Maksimum boyut (bayt, boş = üst sınır yok)
# MT
settings-filters-date-range = Değişiklik tarihi aralığı
# MT
settings-filters-min-mtime = Şu tarihte veya sonrası
# MT
settings-filters-max-mtime = Şu tarihte veya öncesi
# MT
settings-filters-attributes = Öznitelikler
# MT
settings-filters-skip-hidden = Gizli dosyaları / klasörleri atla
# MT
settings-filters-skip-system = Sistem dosyalarını atla (yalnızca Windows)
# MT
settings-filters-skip-readonly = Salt okunur dosyaları atla

# Phase 15 — auto-update
# MT
settings-tab-updater = Güncellemeler
# MT
settings-updater-hint = Copy That, imzalı güncellemeleri günde en fazla bir kez kontrol eder. Güncellemeler bir sonraki uygulama çıkışında yüklenir.
# MT
settings-updater-auto-check = Başlangıçta güncellemeleri kontrol et
# MT
settings-updater-channel = Yayın kanalı
# MT
settings-updater-channel-stable = Kararlı
# MT
settings-updater-channel-beta = Beta (ön sürüm)
# MT
settings-updater-last-check = Son kontrol
# MT
settings-updater-last-never = Asla
# MT
settings-updater-check-now = Güncellemeleri şimdi kontrol et
# MT
settings-updater-checking = Kontrol ediliyor…
# MT
settings-updater-available = Güncelleme mevcut
# MT
settings-updater-up-to-date = En son sürümü kullanıyorsunuz.
# MT
settings-updater-dismiss = Bu sürümü atla
# MT
settings-updater-dismissed = Atlandı
# MT
toast-update-available = Yeni bir sürüm mevcut
# MT
toast-update-up-to-date = Zaten en son sürümdesiniz

# Phase 19a — disk-backed file enumeration (TeraCopy-compatible scan DB)
# MT
scan-progress-title = Taranıyor…
# MT
scan-progress-stats = { $files } dosya · şu ana kadar { $bytes }
# MT
scan-pause-button = Taramayı duraklat
# MT
scan-resume-button = Taramaya devam et
# MT
scan-cancel-button = Taramayı iptal et
# MT
scan-cancel-confirm = Taramayı iptal etmek ve ilerlemeyi atmak ister misiniz?
# MT
scan-db-header = Tarama veritabanı
# MT
scan-db-hint = Milyonlarca dosya içeren işler için disk tabanlı tarama veritabanı.
# MT
advanced-scan-hash-during = Tarama sırasında sağlama toplamı hesapla
# MT
advanced-scan-db-path = Tarama veritabanı konumu
# MT
advanced-scan-retention-days = Tamamlanan taramaları otomatik sil (gün)
# MT
advanced-scan-max-keep = Saklanacak en fazla tarama veritabanı sayısı

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
sparse-not-supported-title = Hedef seyrek dosyaları dolduruyor  # MT
sparse-not-supported-body = { $dst_fs } seyrek dosyaları desteklemiyor. Kaynaktaki boşluklar sıfır olarak yazıldı, bu nedenle hedef diskte daha büyük.  # MT
sparse-warning-densified = Seyrek düzen korundu: yalnızca ayrılmış kapsamlar kopyalandı.  # MT
sparse-warning-mismatch = Seyrek düzen uyuşmazlığı — hedef beklenenden büyük olabilir.  # MT

# Phase 24 — security-metadata preservation. MT-flagged drafts;
# the authoritative English source lives in locales/en/copythat.ftl.
settings-preserve-security-metadata = Güvenlik meta verilerini koru  # MT
settings-preserve-security-metadata-hint = Her kopyalamada bant dışı meta veri akışlarını (NTFS ADS / xattrs / POSIX ACL'leri / SELinux bağlamları / Linux dosya yetenekleri / macOS kaynak çatalları) yakalayın ve yeniden uygulayın.  # MT
settings-preserve-motw = Mark-of-the-Web'i (internetten-indirildi bayrağını) koru  # MT
settings-preserve-motw-hint = Güvenlik için kritik. SmartScreen ve Office Protected View, internetten indirilen dosyalar hakkında uyarmak için bu akışı kullanır. Devre dışı bırakmak, indirilen bir çalıştırılabilir dosyanın kopyalama sırasında kaynak işaretini düşürmesine ve işletim sistemi güvenlik önlemlerini atlamasına olanak tanır.  # MT
settings-preserve-posix-acls = POSIX ACL'leri ve genişletilmiş öznitelikleri koru  # MT
settings-preserve-posix-acls-hint = Kopyalama sırasında user.* / system.* / trusted.* xattrs ve POSIX erişim kontrol listelerini taşı.  # MT
settings-preserve-selinux = SELinux bağlamlarını koru  # MT
settings-preserve-selinux-hint = MAC politikaları altında çalışan arka plan programlarının dosyaya erişmeye devam edebilmesi için kopyalama sırasında security.selinux etiketini taşı.  # MT
settings-preserve-resource-forks = macOS kaynak çatallarını ve Finder bilgilerini koru  # MT
settings-preserve-resource-forks-hint = Kopyalama sırasında eski kaynak çatalını ve FinderInfo'yu (renk etiketleri, Carbon meta verileri) taşı.  # MT
settings-appledouble-fallback = Uyumsuz dosya sistemlerinde AppleDouble yan dosyası kullan  # MT
meta-translated-to-appledouble = Yabancı meta veriler AppleDouble yan dosyasında saklandı (._{ $ext })  # MT

# Phase 25 — two-way sync with vector-clock conflict detection.
# MT-flagged drafts; the authoritative English source lives in
# locales/en/copythat.ftl.
footer-sync = Eşzm  # MT
sync-drawer-title = Çift yönlü eşitleme  # MT
sync-drawer-hint = İki klasörü sessiz üzerine yazmalar olmadan eşitleyin. Eşzamanlı düzenlemeler çözülebilir çatışmalar olarak görünür.  # MT
sync-add-pair = Çift ekle  # MT
sync-add-cancel = İptal  # MT
sync-refresh = Yenile  # MT
sync-add-save = Çifti kaydet  # MT
sync-add-saving = Kaydediliyor…  # MT
sync-add-missing-fields = Etiket, sol yol ve sağ yol hepsi gereklidir.  # MT
sync-remove-confirm = Bu eşitleme çiftini kaldır? Durum veritabanı korunur; klasörlere dokunulmaz.  # MT
sync-field-label = Etiket  # MT
sync-field-label-placeholder = örn. Belgeler ↔ NAS  # MT
sync-field-left = Sol klasör  # MT
sync-field-left-placeholder = Mutlak bir yol seçin veya yapıştırın  # MT
sync-field-right = Sağ klasör  # MT
sync-field-right-placeholder = Mutlak bir yol seçin veya yapıştırın  # MT
sync-field-mode = Mod  # MT
sync-mode-two-way = Çift yönlü  # MT
sync-mode-mirror-left-to-right = Ayna (sol → sağ)  # MT
sync-mode-mirror-right-to-left = Ayna (sağ → sol)  # MT
sync-mode-contribute-left-to-right = Katkı (sol → sağ, silme yok)  # MT
sync-no-pairs = Henüz yapılandırılmış eşitleme çifti yok. Başlamak için "Çift ekle" düğmesine tıklayın.  # MT
sync-loading = Yapılandırılmış çiftler yükleniyor…  # MT
sync-never-run = Hiç çalıştırılmadı  # MT
sync-running = Çalışıyor  # MT
sync-run-now = Şimdi çalıştır  # MT
sync-cancel = İptal  # MT
sync-remove-pair = Kaldır  # MT
sync-view-conflicts = Çatışmaları görüntüle ({ $count })  # MT
sync-conflicts-heading = Çatışmalar  # MT
sync-no-conflicts = Son çalıştırmadan çatışma yok.  # MT
sync-winner = Kazanan  # MT
sync-side-left-to-right = sol  # MT
sync-side-right-to-left = sağ  # MT
sync-conflict-kind-concurrent-write = Eşzamanlı düzenleme  # MT
sync-conflict-kind-delete-edit = Sil ↔ düzenle  # MT
sync-conflict-kind-add-add = Her iki taraf da ekledi  # MT
sync-conflict-kind-corrupt-equal = İçerik yeni bir yazma olmadan ayrıldı  # MT
sync-resolve-keep-left = Solu tut  # MT
sync-resolve-keep-right = Sağı tut  # MT
sync-resolve-keep-both = İkisini de tut  # MT
sync-resolve-three-way = 3-yönlü birleştirme ile çöz  # MT
sync-resolve-phase-53-tooltip = Metin dışı dosyalar için etkileşimli 3-yönlü birleştirme Faz 53'te gelir.  # MT
sync-error-prefix = Eşitleme hatası  # MT

# Phase 26 — real-time mirror watcher. MT-flagged drafts;
# the authoritative English source lives in locales/en/copythat.ftl.
live-mirror-start = Canlı yansıtmayı başlat  # MT
live-mirror-stop = Canlı yansıtmayı durdur  # MT
live-mirror-watching = İzleniyor  # MT
live-mirror-toggle-hint = Algılanan her dosya sistemi değişikliğinde otomatik olarak yeniden eşitleme. Etkin çift başına bir arka plan iş parçacığı.  # MT
watch-event-prefix = Dosya değişikliği  # MT
watch-overflow-recovered = İzleyici arabelleği taştı; kurtarmak için yeniden numaralandırılıyor  # MT

# Phase 27 — content-defined chunk store. MT-flagged drafts;
# the authoritative English source lives in locales/en/copythat.ftl.
chunk-store-section = Parça deposu  # MT
chunk-store-enable = Parça deposunu etkinleştir (delta devam ve tekilleştirme)  # MT
chunk-store-enable-hint = Her kopyalanan dosyayı içeriğe göre böler (FastCDC) ve parçaları içerik-adresli olarak depolar. Yeniden denemeler yalnızca değişen parçaları yeniden yazar; paylaşılan içeriğe sahip dosyalar otomatik olarak tekilleştirilir.  # MT
chunk-store-location = Parça deposu konumu  # MT
chunk-store-max-size = Maksimum parça deposu boyutu  # MT
chunk-store-prune = Şundan eski parçaları temizle (gün)  # MT
chunk-store-savings = Parça tekilleştirme ile { $gib } GiB tasarruf edildi  # MT
chunk-store-disk-usage = { $chunks } parçada { $size } kullanılıyor  # MT

# Phase 28 — tray-resident Drop Stack. MT-flagged drafts;
# the authoritative English source lives in locales/en/copythat.ftl.
dropstack-window-title = Drop Stack  # MT
dropstack-tray-open = Drop Stack  # MT
dropstack-empty-title = Drop Stack boş  # MT
dropstack-empty-hint = Dosyaları Gezgin'den buraya sürükleyin veya bir iş satırına sağ tıklayarak ekleyin.  # MT
dropstack-add-to-stack = Drop Stack'e ekle  # MT
dropstack-copy-all-to = Hepsini kopyala…  # MT
dropstack-move-all-to = Hepsini taşı…  # MT
dropstack-clear = Yığını temizle  # MT
dropstack-remove-row = Yığından çıkar  # MT
dropstack-path-missing-toast = { $path } kaldırıldı — dosya artık yok.  # MT
dropstack-always-on-top = Drop Stack'i her zaman en üstte tut  # MT
dropstack-show-tray-icon = Copy That tepsi simgesini göster  # MT
dropstack-open-on-start = Uygulama başlangıcında Drop Stack'i otomatik aç  # MT
dropstack-count = { $count } yol  # MT

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
