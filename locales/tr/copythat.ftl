app-name = Freally File Manager v0.19.85
window-title = Freally File Manager v0.19.85
shred-ssd-advisory = Uyarı: bu hedef bir SSD üzerinde bulunuyor. Çok geçişli üzerine yazmalar flash belleği güvenilir şekilde temizlemez; çünkü aşınma dengeleme ve fazla yer ayırma, verileri mantıksal blok adresinin altından taşır. Katı hal ortamları için ATA SECURE ERASE, Güvenli Silmeli NVMe Format veya anahtarı imha edilen tam disk şifrelemesini tercih edin.

# Global aggregate states (header pill)
state-idle = Boşta
state-copying = Kopyalanıyor
state-verifying = Doğrulanıyor
state-paused = Duraklatıldı
state-error = Hata

# Per-job states (row badge)
state-pending = Kuyrukta
state-running = Çalışıyor
state-cancelled = İptal edildi
state-succeeded = Tamamlandı
state-failed = Başarısız

# Actions
action-pause = Duraklat
action-resume = Devam et
action-cancel = İptal
action-pause-all = Tüm işleri duraklat
action-resume-all = Tüm işleri sürdür
action-cancel-all = Tüm işleri iptal et
action-close = Kapat
action-reveal = Klasörde göster
action-add-files = Dosya ekle
action-add-folders = Klasör ekle

# Phase 13d — activity feed
activity-title = Etkinlik
activity-clear = Etkinlik listesini temizle
activity-empty = Henüz dosya etkinliği yok.
activity-after-done = Tamamlandığında:
activity-keep-open = Uygulamayı açık tut
activity-close-app = Uygulamayı kapat
activity-shutdown = Bilgisayarı kapat
activity-logoff = Oturumu kapat
activity-sleep = Uyku

# Phase 14 — preflight free-space dialog
preflight-block-title = Hedefte yeterli alan yok
preflight-warn-title = Hedefte alan az
preflight-unknown-title = Boş alan belirlenemedi
preflight-unknown-body = Kaynak, boyutunu hızlıca ölçmek için fazla büyük ya da hedef birim yanıt vermedi. Devam edebilirsiniz; alan yetmezse motorun alan koruması kopyalamayı düzgün şekilde durdurur.
preflight-required = Gerekli
preflight-free = Boş
preflight-reserve = Ayrılan
preflight-shortfall = Eksik
preflight-continue = Yine de devam et
preflight-pick-subset = Kopyalanacakları seç…
collision-modal-overwrite-older = Yalnızca eskileri üzerine yaz

# Phase 14e — subset picker
subset-title = Kopyalanacak kaynakları seçin
subset-subtitle = Seçimin tamamı hedefe sığmıyor. Kopyalamak istediğiniz öğeleri işaretleyin; geri kalanı atlanır.
subset-loading = Boyutlar ölçülüyor…
subset-too-large = saymak için çok büyük
subset-budget = Kullanılabilir
subset-remaining = Kalan
subset-confirm = Seçimi kopyala
history-rerun-hint = Bu kopyalamayı yeniden çalıştır — kaynak ağacındaki her dosyayı yeniden tarar
history-clear-all = Tümünü temizle
history-clear-all-confirm = Onaylamak için tekrar tıklayın
history-clear-all-hint = Tüm geçmiş satırlarını sil. Onay için ikinci bir tıklama gerekir.
toast-history-cleared = Geçmiş temizlendi ({ $count } satır kaldırıldı)

# Phase 15 — source-list ordering in the drop staging dialog
drop-dialog-sort-label = Sıralama:
sort-custom = Özel
sort-name-asc = Ad A → Z (önce dosyalar)
sort-name-desc = Ad Z → A (önce dosyalar)
sort-size-asc = Boyut küçükten büyüğe (önce dosyalar)
sort-size-desc = Boyut büyükten küçüğe (önce dosyalar)
sort-reorder = Yeniden sırala
sort-move-top = En üste taşı
sort-move-up = Yukarı taşı
sort-move-down = Aşağı taşı
sort-move-bottom = En alta taşı

# Phase 16 — sort preset names for the Activity list (shorter
# wording than the DropStagingDialog variants; the Activity list
# is files-only so the "(files first)" suffix isn't relevant).
sort-name-asc-simple = Ad A → Z
sort-name-desc-simple = Ad Z → A
sort-size-asc-simple = Boyut küçükten büyüğe
sort-size-desc-simple = Boyut büyükten küçüğe
activity-sort-locked = Kopyalama sürerken sıralama devre dışıdır. Duraklatın ya da bitmesini bekleyip sırayı değiştirin.

# Phase 17 — collision-policy picker in the DropStagingDialog
drop-dialog-collision-label = Bir dosya zaten varsa:
collision-policy-keep-both = İkisini de tut (yeni kopyayı _2, _3, … olarak yeniden adlandır)
collision-policy-skip = Yeni kopyayı atla
collision-policy-overwrite = Mevcut dosyanın üzerine yaz
collision-policy-overwrite-if-newer = Yalnızca daha yeniyse üzerine yaz
collision-policy-prompt = Her seferinde sor

# Phase 18 — progress labels on the DropStagingDialog Start button
drop-dialog-busy-checking = Boş alan denetleniyor…
drop-dialog-busy-enumerating = Dosyalar sayılıyor…
drop-dialog-busy-starting = Kopyalama başlatılıyor…
toast-enumeration-deferred = Kaynak ağaç büyük — önceden dosya listesi atlanıyor; satırlar motor onları işledikçe görünecek.

# Context menu (per-row right-click)
menu-pause = Duraklat
menu-resume = Devam et
menu-cancel = İptal
menu-remove = Kuyruktan kaldır
menu-reveal-source = Kaynağı klasörde göster
menu-reveal-destination = Hedefi klasörde göster

# Header / toolbar
header-eta-label = Tahmini kalan süre
header-toolbar-label = Genel denetimler

# Footer
footer-queued = etkin iş
footer-total-bytes = aktarımda
footer-errors = hata
footer-history = Geçmiş

# Empty state
empty-title = Kopyalamak için dosya veya klasör bırakın
empty-hint = Öğeleri pencereye sürükleyin. Bir hedef soracağız, ardından her kaynak için bir iş kuyruğa alacağız.
empty-region-label = İş listesi

# Details drawer
details-drawer-label = İş ayrıntıları
details-source = Kaynak
details-destination = Hedef
details-state = Durum
details-bytes = Bayt
details-files = Dosya
details-speed = Hız
details-eta = Kalan süre
details-error = Hata

# Drop dialog
drop-dialog-title = Bırakılan öğeleri aktar
drop-dialog-subtitle = { $count } öğe aktarıma hazır. Başlamak için bir hedef klasör seçin.
drop-dialog-mode = İşlem
drop-dialog-copy = Kopyala
drop-dialog-move = Taşı
drop-dialog-pick-destination = Hedef seç
drop-dialog-change-destination = Hedefi değiştir
drop-dialog-start-copy = Kopyalamayı başlat
drop-dialog-start-move = Taşımayı başlat

# ETA placeholders
eta-calculating = hesaplanıyor…
eta-unknown = bilinmiyor

# Toast messages
toast-job-done = Aktarım tamamlandı
toast-copy-queued = Kopyalama kuyruğa alındı
toast-move-queued = Taşıma kuyruğa alındı
toast-error-resolved = Hata çözüldü
toast-collision-resolved = Çakışma çözüldü
toast-elevated-unavailable = Yükseltilmiş yeniden deneme Aşama 17'de gelecek — henüz kullanılamıyor
toast-clipboard-files-detected = Panoda dosyalar var — Freally File Manager ile kopyalamak için yapıştırma kısayolunuza basın
toast-clipboard-no-files = Panoda yapıştırılacak dosya yok
toast-error-log-exported = Hata günlüğü dışa aktarıldı

# Error modal (Phase 8)
error-modal-title = Bir aktarım başarısız oldu
error-modal-retry = Yeniden dene
error-modal-retry-elevated = Yükseltilmiş izinlerle yeniden dene
error-modal-skip = Atla
error-modal-skip-all-kind = Bu türdeki tüm hataları atla
error-modal-abort = Tümünü durdur
error-modal-path-label = Yol
error-modal-code-label = Kod
error-drawer-pending-count = Bekleyen başka hatalar var
error-drawer-toggle = Daralt veya genişlet

# Error-kind labels (Phase 8). Source of truth — engine maps each
# `CopyErrorKind` to one of these keys via `localized_key()`.
err-not-found = Dosya bulunamadı
err-permission-denied = İzin reddedildi
err-disk-full = Hedef disk dolu
err-interrupted = İşlem kesintiye uğradı
err-verify-failed = Kopyalama sonrası doğrulama başarısız oldu
err-path-escape = Yol reddedildi — üst dizin (..) bölümleri veya geçersiz baytlar içeriyor
err-path-invalid-encoding = Yol reddedildi — dize geçersiz UTF-8 / değiştirme karakterleri içeriyor
err-helper-invalid-json = Ayrıcalıklı yardımcı hatalı biçimli JSON aldı; bu istek yoksayılıyor
err-helper-grant-out-of-band = GrantCapabilities, durumsuz işleyici tarafından değil yardımcı çalıştırma döngüsü tarafından işlenmelidir
err-randomness-unavailable = İşletim sistemi rastgele sayı üreteci başarısız oldu; bir oturum kimliği üretilemiyor
err-sparseness-mismatch = Seyrek düzen hedefte korunamadı
err-io-other = Bilinmeyen G/Ç hatası

# Collision modal (Phase 8)
collision-modal-title = Dosya zaten var
collision-modal-overwrite = Üzerine yaz
collision-modal-overwrite-if-newer = Daha yeniyse üzerine yaz
collision-modal-skip = Atla
collision-modal-keep-both = İkisini de tut
collision-modal-rename = Yeniden adlandır…
collision-modal-apply-to-all = Tümüne uygula
collision-modal-source = Kaynak
collision-modal-destination = Hedef
collision-modal-size = Boyut
collision-modal-modified = Değiştirilme
collision-modal-hash-check = Hızlı karma (SHA-256)
collision-modal-hash-computing = Hesaplanıyor…
collision-modal-hash-identical = Aynı
collision-modal-hash-different = Farklı
collision-modal-rename-placeholder = Yeni dosya adı
collision-modal-confirm-rename = Yeniden adlandır

# Error log drawer (Phase 8)
error-log-title = Hata günlüğü
error-log-empty = Günlüğe alınmış hata yok
error-log-export-csv = CSV dışa aktar
error-log-export-txt = Metin dışa aktar
error-log-clear = Günlüğü temizle
error-log-col-time = Zaman
error-log-col-job = İş
error-log-col-path = Yol
error-log-col-code = Kod
error-log-col-message = İleti
error-log-col-resolution = Çözüm

# History drawer (Phase 9)
history-title = Geçmiş
history-empty = Henüz kaydedilmiş iş yok
history-unavailable = Kopyalama geçmişi kullanılamıyor. Uygulama başlangıçta SQLite deposunu açamadı.
history-filter-any = herhangi
history-filter-kind = Tür
history-filter-status = Durum
history-filter-text = Ara
history-refresh = Yenile
history-export-csv = CSV dışa aktar
history-purge-30 = 30 günden eskileri temizle
history-rerun = Yeniden çalıştır
history-detail-open = Ayrıntılar
history-detail-title = İş ayrıntıları
history-detail-empty = Kaydedilmiş öğe yok
history-col-date = Tarih
history-col-kind = Tür
history-col-src = Kaynak
history-col-dst = Hedef
history-col-files = Dosya
history-col-size = Boyut
history-col-status = Durum
history-col-duration = Süre
history-col-error = Hata
toast-history-exported = Geçmiş dışa aktarıldı
toast-history-rerun-queued = Yeniden çalıştırma kuyruğa alındı

# Totals drawer (Phase 10)
footer-totals = Toplamlar
totals-title = Toplamlar
totals-loading = Toplamlar yükleniyor…
totals-card-bytes = Kopyalanan toplam bayt
totals-card-files = Dosya
totals-card-jobs = İş
totals-card-avg-rate = Ortalama verim
totals-errors = hata
totals-spark-title = Son 30 gün
totals-kinds-title = Türe göre
totals-saved-title = Kazanılan süre (tahmini)
totals-saved-note = Aynı iş yükünün temel bir dosya yöneticisi kopyalamasına kıyasla tahmini.
totals-reset = İstatistikleri sıfırla
totals-reset-confirm = Bu, kaydedilmiş her işi ve öğeyi siler. Devam edilsin mi?
totals-reset-confirm-yes = Evet, sıfırla
toast-totals-reset = İstatistikler sıfırlandı

# Phase 11a — i18n core: surface remaining user-visible strings so
# every string on a main-window screen flows through Fluent.

# Header language switcher (temporary placement — folds into
# Settings → General in Phase 12).
header-language-label = Dil
header-language-title = Dili değiştir

# Job-kind labels. History filter, Totals breakdown, and history
# rows all map wire-format `kind` strings to these labels.
kind-copy = Kopyala
kind-move = Taşı
kind-delete = Sil
kind-secure-delete = Güvenli silme

# History status labels. Distinct from `state-*` because the
# history wire format uses plain `running`/`succeeded`/... rather
# than the live `JobState` enum the queue exposes.
status-running = Çalışıyor
status-succeeded = Başarılı
status-failed = Başarısız
status-cancelled = İptal edildi
# Per-item status (not job-level): `ok` / `skipped` are only
# reachable from the history detail view.
status-ok = Tamam
status-skipped = Atlandı

# History drawer: search field placeholder + purge toast.
history-search-placeholder = /yol
toast-history-purged = 30 günden eski { $count } iş temizlendi

# User-facing command-layer validation errors. The Rust side
# returns these keys when input is missing; the toast layer looks
# them up rather than showing the raw English.
err-source-required = En az bir kaynak yolu gereklidir.
err-destination-empty = Hedef yolu boş.
err-source-empty = Kaynak yolu boş.

# Localised duration formatting for the Totals drawer and ETA
# fields. `{ $ms }`, `{ $s }`, `{ $m }`, `{ $h }` are integer
# placeables — the formatter passes pre-computed values in.
duration-lt-1s = < 1 sn
duration-ms = { $ms } ms
duration-seconds = { $s } sn
duration-minutes-seconds = { $m } dk { $s } sn
duration-hours-minutes = { $h } sa { $m } dk
duration-zero = 0 sn

# Rate unit. Appended to a formatted byte size. Some languages
# render this with a leading space ("Ko/s"); keep it translatable
# even though the SI-derived "/s" is near-universal.
rate-unit-per-second = { $size }/sn

# Phase 11b — Settings modal skeleton. Phase 12 expanded this into a
# full six-tab preferences window; the `settings-phase-12-hint` key
# is retired in favour of `settings-tab-profiles` + concrete labels.
settings-title = Ayarlar
settings-tab-general = Genel
settings-tab-appearance = Görünüm
settings-section-language = Dil
settings-phase-12-hint = Daha fazla ayar (tema, aktarım varsayılanları, doğrulama algoritması, profiller) Aşama 12'de gelir.

# Phase 12 — full Settings window keys. Grouped by tab so the
# human-review pass can pick one section at a time. Every field
# label and every option label that the user will actually read
# has its own key; dynamic wire values (enum discriminants like
# "auto" / "prefer") stay untranslated on the wire.

settings-loading = Ayarlar yükleniyor…
settings-tab-transfer = Aktarım
settings-tab-filters = Filtreler
settings-tab-shell = Kabuk
settings-tab-secure-delete = Güvenli silme
settings-tab-advanced = Gelişmiş
settings-tab-updater = Güncellemeler
settings-tab-profiles = Profiller

# General tab additions
settings-section-theme = Tema
settings-theme-auto = Otomatik
settings-theme-light = Açık
settings-theme-dark = Koyu
settings-start-with-os = Sistem başlangıcında çalıştır
settings-single-instance = Tek çalışan örnek
settings-minimize-to-tray = Kapatınca sistem tepsisine küçült
settings-error-display-mode = Hata istemi biçimi
settings-error-display-modal = Kalıcı pencere (uygulamayı engeller)
settings-error-display-drawer = Çekmece (engellemez)
settings-error-display-mode-hint = Kalıcı pencere, siz karar verene kadar kuyruğu durdurur. Çekmece kuyruğu sürdürür ve hataları köşede ayıklamanıza olanak tanır.
settings-paste-shortcut = Dosyaları genel kısayolla yapıştır
settings-paste-shortcut-combo = Kısayol birleşimi
settings-paste-shortcut-hint = Explorer / Finder / Files'tan kopyalanan dosyaları Freally File Manager ile yapıştırmak için bu birleşime sisteminizin herhangi bir yerinde basın. CmdOrCtrl, macOS'ta Cmd, Windows / Linux'ta Ctrl olarak çözümlenir.
settings-clipboard-watcher = Kopyalanan dosyalar için panoyu izle
settings-clipboard-watcher-hint = Panoda dosya URL'leri belirdiğinde, Freally File Manager ile yapıştırabileceğinizi belirten bir bildirim göster. Etkinken her 500 ms'de bir yoklar.

# Transfer tab
settings-buffer-size = Arabellek boyutu
settings-verify = Kopyalamadan sonra doğrula
settings-verify-off = Kapalı
settings-concurrency = Eşzamanlılık
settings-concurrency-auto = Otomatik
settings-reflink = Reflink / hızlı yollar
settings-reflink-prefer = Tercih et
settings-reflink-avoid = Reflink'ten kaçın
settings-reflink-disabled = Her zaman eşzamansız motoru kullan
settings-fsync-on-close = Kapatırken diske eşitle (daha yavaş, daha güvenli)
settings-preserve-timestamps = Zaman damgalarını koru
settings-preserve-permissions = İzinleri koru
settings-preserve-acls = ACL'leri koru (Aşama 14)
settings-preserve-sparseness = Seyrek dosyaları koru
settings-preserve-sparseness-hint = Yalnızca seyrek dosyaların (VM diskleri, veritabanı dosyaları) ayrılmış bölümlerini kopyala; böylece hedef, diskte kaynakla aynı boyutta kalır.
settings-force-parallel-chunks = Paralel çok parçalı kopyalama (yalnızca RAID / diziler)
settings-force-parallel-chunks-hint = Her büyük kopyalamayı eşzamanlı parçalara böler. Yalnızca şeritli/RAID/ağ hedeflerinde yardımcı olur; tek bir SSD/NVMe'yi YAVAŞLATIR (-%25 ila -%76). Hedef çok diskli bir dizi değilse kapalı bırakın.

# Shell tab
settings-context-menu = Kabuk bağlam menüsü girişlerini etkinleştir
settings-intercept-copy = Varsayılan kopyalama işleyicisini yakala (Windows)
settings-intercept-copy-hint = Açıkken Explorer'ın Ctrl+C / Ctrl+V işlemleri Freally File Manager üzerinden yönlendirilir. Kayıt Aşama 14'te gelir.
settings-notify-completion = İş tamamlandığında bildir

# Secure delete tab
settings-shred-method = Varsayılan parçalama yöntemi
settings-shred-zero = Sıfır (1 geçiş)
settings-shred-random = Rastgele (1 geçiş)
settings-shred-dod3 = DoD 5220.22-M (3 geçiş)
settings-shred-dod7 = DoD 5220.22-M (7 geçiş)
settings-shred-gutmann = Gutmann (35 geçiş)
settings-shred-nist = NIST 800-88
settings-shred-confirm-twice = Parçalamadan önce çift onay iste

# Advanced tab
settings-log-level = Günlük düzeyi
settings-log-off = Kapalı
settings-telemetry = Telemetri
settings-telemetry-never = Asla — hiçbir günlük düzeyinde sunucuya veri gönderilmez
settings-error-policy = Varsayılan hata ilkesi
settings-error-policy-ask = Sor
settings-error-policy-skip = Atla
settings-error-policy-retry = Geri çekilmeyle yeniden dene
settings-error-policy-abort = İlk hatada durdur
settings-history-retention = Geçmiş saklama süresi (gün)
settings-history-retention-hint = 0 = sonsuza dek tut. Diğer değerler, başlangıçta eski işleri otomatik temizler.
settings-database-path = Veritabanı yolu
settings-database-path-default = (varsayılan — işletim sistemi veri dizini)
settings-reset-all = Varsayılanlara sıfırla
settings-reset-confirm = Her tercih varsayılanına sıfırlansın mı? Profiller etkilenmez.

# Profiles tab
settings-profiles-hint = Geçerli ayarları bir adla kaydedin; tek tek ayarlara dokunmadan geri dönmek için sonra yükleyin.
settings-profile-name-placeholder = Profil adı
settings-profile-save = Kaydet
settings-profile-import = İçe aktar…
settings-profile-load = Yükle
settings-profile-export = Dışa aktar…
settings-profile-delete = Sil
settings-profile-empty = Henüz kaydedilmiş profil yok.
settings-profile-import-prompt = İçe aktarılan profilin adı:

# Toasts driven by Phase 12 profile actions
toast-settings-reset = Ayarlar sıfırlandı
toast-profile-saved = Profil kaydedildi
toast-profile-loaded = Profil yüklendi
toast-profile-exported = Profil dışa aktarıldı
toast-profile-imported = Profil içe aktarıldı

# Phase 14a — enumeration-time filters
settings-filters-hint = Motor onları hiç açmasın diye dosyaları sıralama anında atlayın. Dahil etmeler yalnızca dosyalara uygulanır; hariç tutmalar eşleşen dizinleri de budar.
settings-filters-enabled = Ağaç kopyalamaları için filtreleri etkinleştir
settings-filters-include-globs = Dahil etme glob'ları
settings-filters-include-globs-placeholder = **/*.txt
settings-filters-include-globs-hint = Satır başına bir glob. Boş değilken bir dosyanın hayatta kalması için en az bir dahil etmeyle eşleşmesi gerekir. Dizinlerin içine her zaman inilir.
settings-filters-exclude-globs = Hariç tutma glob'ları
settings-filters-exclude-globs-placeholder = **/node_modules
settings-filters-exclude-globs-hint = Satır başına bir glob. Eşleşmeler dizinler için tüm alt ağacı budar; eşleşen dosyalar atlanır.
settings-filters-size-range = Dosya boyutu aralığı
settings-filters-min-size-bytes = En küçük boyut (bayt, boş = alt sınır yok)
settings-filters-max-size-bytes = En büyük boyut (bayt, boş = üst sınır yok)
settings-filters-date-range = Değiştirilme zamanı aralığı
settings-filters-min-mtime = Şu tarihte veya sonrasında değiştirilmiş
settings-filters-max-mtime = Şu tarihte veya öncesinde değiştirilmiş
settings-filters-attributes = Öznitelik bitleri
settings-filters-skip-hidden = Gizli dosyaları / klasörleri atla
settings-filters-skip-system = Sistem dosyalarını atla (yalnızca Windows)
settings-filters-skip-readonly = Salt okunur dosyaları atla

# Phase 15 — auto-update
settings-updater-hint = Freally File Manager, imzalı güncellemeleri günde en fazla bir kez denetler. Güncellemeler uygulamadan sonraki çıkışta yüklenir.
settings-updater-auto-check = Başlangıçta güncellemeleri denetle
settings-updater-channel = Sürüm kanalı
settings-updater-channel-stable = Kararlı
settings-updater-channel-beta = Beta (ön sürüm)
settings-updater-last-check = Son denetlenme
settings-updater-last-never = Hiçbir zaman
settings-updater-check-now = Şimdi güncellemeleri denetle
settings-updater-checking = Denetleniyor…
settings-updater-available = Güncelleme var
settings-updater-up-to-date = En son sürümü çalıştırıyorsunuz.
settings-updater-dismiss = Bu sürümü atla
settings-updater-dismissed = Atlandı
toast-update-available = Daha yeni bir sürüm var
toast-update-up-to-date = Zaten en son sürümdesiniz

# Phase 19a — disk-backed file enumeration (TeraCopy-compatible scan DB)
scan-progress-title = Taranıyor…
scan-progress-stats = { $files } dosya · şu ana dek { $bytes }
scan-pause-button = Taramayı duraklat
scan-resume-button = Taramayı sürdür
scan-cancel-button = Taramayı iptal et
scan-cancel-confirm = Tarama iptal edilip ilerleme atılsın mı?
scan-db-header = Tarama veritabanı
scan-db-hint = Milyonlarca dosyalı işler için disk üzerinde tarama veritabanı.
advanced-scan-hash-during = Tarama sırasında sağlama toplamlarını hesapla
advanced-scan-db-path = Tarama veritabanı konumu
advanced-scan-retention-days = Tamamlanan taramaları şu süre sonra otomatik sil (gün)
advanced-scan-max-keep = Tutulacak en fazla tarama veritabanı sayısı

# Phase 19b — filesystem-snapshot source for locked files.
settings-on-locked = Bir dosya kilitliyken
settings-on-locked-ask = İlk seferinde sor
settings-on-locked-retry = Kısa süre yeniden dene, sonra hatayı bildir
settings-on-locked-skip = Kilitli dosyayı atla
settings-on-locked-snapshot = Bir dosya sistemi anlık görüntüsü kullan
settings-on-locked-hint = "Dosya başka bir işlem tarafından kullanılıyor" hatalarını ortadan kaldırır. Freally File Manager, kaynak birimin anlık görüntüsünü alır (Windows'ta VSS, Linux'ta ZFS/Btrfs, macOS'ta APFS) ve anlık görüntü kopyasından okur.
snapshot-prompt-title = Bu dosya başka bir işlem tarafından kullanılıyor
snapshot-prompt-body = Başka bir program { $path } dosyasını özel yazma için açık tutuyor. Freally File Manager'in bu ve aynı birimdeki benzer dosyaları nasıl ele alacağını seçin.
snapshot-source-active = 📷 { $volume } biriminin { $kind } anlık görüntüsünden okunuyor
snapshot-create-failed = Kaynak birimin anlık görüntüsü oluşturulamadı
snapshot-vss-needs-elevation = Bir VSS anlık görüntüsünden okumak Yönetici izni gerektirir. Freally File Manager sizden buna izin vermenizi isteyecek.
snapshot-cleanup-failed = Anlık görüntü yardımcısı bir temizleme hatası bildirdi — birimde kalıntı bir gölge kopya kalmış olabilir.

# Phase 20 — durable resume journal.
resume-prompt-title = Önceki aktarımlar sürdürülsün mü?
resume-prompt-body = Freally File Manager, önceki bir oturumdan { $count } tamamlanmamış aktarım algıladı. Her biri için ne yapılacağını seçin.
resume-prompt-resume = Devam et
resume-prompt-resume-all = Tümünü sürdür
resume-discard-one = Devam etme
resume-discard-all = Tümünü at
resume-aborted-hash-mismatch = Hedefin ilk { $offset } baytı kaynakla eşleşmiyor — baştan başlatılıyor.
settings-auto-resume = Kesintiye uğrayan işleri sormadan otomatik sürdür
settings-auto-resume-hint = Başlangıçtaki sürdürme istemini atla ve her tamamlanmamış işi sessizce yeniden kuyruğa al. Varsayılan olarak kapalı.

# Phase 21 — bandwidth shaping (GCRA token bucket + schedule + auto-throttle).
settings-tab-network = Ağ
settings-network-hint = Ağın geri kalanını kullanılabilir tutmak için aktarım hızınızı sınırlayın. Genel olarak uygulayın, günlük bir programı izleyin ya da sınırlı Wi-Fi / pil / hücresel bağlantılara otomatik tepki verin.
settings-network-mode = Bant genişliği sınırı
settings-network-mode-off = Kapalı (sınır yok)
settings-network-mode-fixed = Sabit değer
settings-network-mode-schedule = Program kullan
settings-network-cap-mbps = Sınır (MB/sn)
settings-network-schedule = Program (rclone biçimi)
settings-network-schedule-hint = Boşlukla ayrılmış HH:MM,hız sınırları ve isteğe bağlı Mon-Fri,hız gün kuralları. Hızlar: 512k, 10M, 2G, off, unlimited. Örnek: 08:00,512k 18:00,10M Sat-Sun,unlimited.
settings-network-auto-header = Otomatik kısma
settings-network-auto-metered = Sınırlı Wi-Fi'de
settings-network-auto-battery = Pildeyken
settings-network-auto-cellular = Hücreseldeyken
settings-network-auto-unchanged = Geçersiz kılma
settings-network-auto-pause = Aktarımları duraklat
settings-network-auto-cap = Sabit değere sınırla
shape-badge-paused = duraklatıldı
shape-badge-tooltip = Bant genişliği sınırı etkin — Ayarlar → Ağ'ı açmak için tıklayın
shape-badge-source-schedule = programlı
shape-badge-source-metered = sınırlı
shape-badge-source-battery = pildeyken
shape-badge-source-cellular = hücresel
shape-badge-source-settings = etkin
shape-error-schedule-invalid = Program biçimi geçerli değil: { $message }

# Phase 22 — aggregate conflict dialog v2 (thumbnails, per-pattern
# rules, and reusable conflict profiles). Every key below is user-
# visible text in the `ConflictBatchModal.svelte` component.
conflict-batch-title = { $jobname } içinde { $count } dosya çakışması
conflict-batch-state-pending = Bekliyor
conflict-batch-state-resolved = Çözüldü
conflict-batch-action-overwrite = Üzerine yaz
conflict-batch-action-skip = Atla
conflict-batch-action-keep-both = İkisini de tut
conflict-batch-action-newer-wins = Yeni olan kazanır
conflict-batch-action-larger-wins = Büyük olan kazanır
conflict-batch-bulk-apply-selected = Seçilenlere uygula
conflict-batch-bulk-apply-extension = Bu uzantının tümüne uygula
conflict-batch-bulk-apply-glob = Eşleşen glob'a uygula…
conflict-batch-bulk-apply-remaining = Kalan tümüne uygula
conflict-batch-bulk-glob-placeholder = ör. **/*.tmp
conflict-batch-save-profile = Bu kuralları profil olarak kaydet…
conflict-batch-profile-placeholder = Profil adı
conflict-batch-matched-rule = '{ $rule }' kuralı yoluyla → { $action }
conflict-batch-empty = Tüm çakışmalar çözüldü
conflict-batch-source-vs-destination = Kaynak ile hedef
conflict-batch-source-label = Kaynak
conflict-batch-destination-label = Hedef
conflict-batch-size-label = Boyut
conflict-batch-modified-label = Değiştirilme
conflict-batch-close = Kapat
conflict-batch-profile-saved = Çakışma profili kaydedildi

# Phase 23 — sparse-file preservation. The toast fires once per
# destination volume when the filesystem can't preserve holes; the
# warning line is surfaced in the job detail drawer so the user knows
# the dst is larger on disk than the source was.
sparse-not-supported-title = Hedef, seyrek dosyaları dolduruyor
sparse-not-supported-body = { $dst_fs } seyrek dosyaları desteklemiyor. Kaynaktaki boşluklar sıfır olarak yazıldı; bu nedenle hedef diskte daha büyük.
sparse-warning-densified = Seyrek düzen korundu: yalnızca ayrılmış bölümler kopyalandı.
sparse-warning-mismatch = Seyrek düzen uyuşmazlığı — hedef beklenenden büyük olabilir.

# Phase 24 — security-metadata preservation. The Mark-of-the-Web
# (Zone.Identifier ADS) toggle is security-sensitive: turning it off
# lets a downloaded executable shed its SmartScreen / Office Protected
# View flag on copy, which is why the tooltip carries an explicit
# warning. AppleDouble fallback emits `._<filename>` sidecars on
# destination filesystems that can't hold the foreign metadata.
settings-preserve-security-metadata = Güvenlik meta verilerini koru
settings-preserve-security-metadata-hint = Her kopyalamada bant dışı meta veri akışlarını (NTFS ADS / xattr'lar / POSIX ACL'leri / SELinux bağlamları / Linux dosya yetenekleri / macOS kaynak çatalları) yakala ve yeniden uygula.
settings-preserve-motw = Mark-of-the-Web'i koru (internetten indirildi işareti)
settings-preserve-motw-hint = Güvenlik için kritik. SmartScreen ve Office Korumalı Görünüm, internetten indirilen dosyalar hakkında uyarmak için bu akışı kullanır. Devre dışı bırakmak, indirilen bir yürütülebilir dosyanın kopyalanırken köken işaretini bırakmasına ve işletim sistemi korumalarını aşmasına izin verir.
settings-preserve-posix-acls = POSIX ACL'lerini ve genişletilmiş öznitelikleri koru
settings-preserve-posix-acls-hint = user.* / system.* / trusted.* xattr'larını ve POSIX erişim denetim listelerini kopyalama boyunca taşı.
settings-preserve-selinux = SELinux bağlamlarını koru
settings-preserve-selinux-hint = security.selinux etiketini kopyalama boyunca taşı; böylece MAC ilkeleri altında çalışan arka plan hizmetleri dosyaya yine erişebilir.
settings-preserve-resource-forks = macOS kaynak çatallarını ve Finder bilgisini koru
settings-preserve-resource-forks-hint = Eski kaynak çatalını ve FinderInfo'yu (renk etiketleri, Carbon meta verisi) kopyalama boyunca taşı.
settings-appledouble-fallback = Uyumsuz dosya sistemlerinde AppleDouble yan dosyası kullan
meta-translated-to-appledouble = Yabancı meta veri AppleDouble yan dosyasında saklandı (._{ $ext })

# Phase 25 — two-way sync with vector-clock conflict detection.
# The drawer lists configured sync pairs; each pair runs an
# independent reconciliation round against a per-pair `.copythat-sync.db`
# state store. Concurrent edits from a common ancestor surface as
# conflicts rather than silent overwrites; the losing side's content
# is preserved as `name.sync-conflict-YYYYMMDD-HHMMSS-<host>.ext`.
footer-sync = Eşitleme
sync-drawer-title = İki yönlü eşitleme
sync-drawer-hint = İki klasörü sessiz üzerine yazmalar olmadan eşitleyin. Eşzamanlı düzenlemeler, çözebileceğiniz çakışmalar olarak ortaya çıkar.
sync-add-pair = Çift ekle
sync-add-cancel = İptal
sync-refresh = Yenile
sync-add-save = Çifti kaydet
sync-add-saving = Kaydediliyor…
sync-add-missing-fields = Etiket, sol yol ve sağ yolun tümü gereklidir.
sync-remove-confirm = Bu eşitleme çifti kaldırılsın mı? Durum veritabanı korunur; klasörlere dokunulmaz.
sync-field-label = Etiket
sync-field-label-placeholder = ör. Belgeler ↔ NAS
sync-field-left = Sol klasör
sync-field-left-placeholder = Mutlak bir yol seçin veya yapıştırın
sync-field-right = Sağ klasör
sync-field-right-placeholder = Mutlak bir yol seçin veya yapıştırın
sync-field-mode = Mod
sync-mode-two-way = İki yönlü
sync-mode-mirror-left-to-right = Yansıt (sol → sağ)
sync-mode-mirror-right-to-left = Yansıt (sağ → sol)
sync-mode-contribute-left-to-right = Katkıda bulun (sol → sağ, silme yok)
sync-no-pairs = Henüz yapılandırılmış eşitleme çifti yok. Başlamak için "Çift ekle"ye tıklayın.
sync-loading = Yapılandırılmış çiftler yükleniyor…
sync-never-run = Hiç çalıştırılmadı
sync-running = Çalışıyor
sync-run-now = Şimdi çalıştır
sync-cancel = İptal
sync-remove-pair = Kaldır
sync-view-conflicts = Çakışmaları görüntüle ({ $count })
sync-conflicts-heading = Çakışmalar
sync-no-conflicts = Son çalıştırmadan çakışma yok.
sync-winner = Kazanan
sync-side-left-to-right = sol
sync-side-right-to-left = sağ
sync-conflict-kind-concurrent-write = Eşzamanlı düzenleme
sync-conflict-kind-delete-edit = Silme ↔ düzenleme
sync-conflict-kind-add-add = Her iki taraf da ekledi
sync-conflict-kind-corrupt-equal = İçerik yeni bir yazma olmadan ayrıştı
sync-resolve-keep-left = Solu tut
sync-resolve-keep-right = Sağı tut
sync-resolve-keep-both = İkisini de tut
sync-resolve-three-way = 3 yönlü birleştirmeyle çöz
sync-resolve-phase-53-tooltip = Metin dışı dosyalar için etkileşimli 3 yönlü birleştirme Aşama 53'te gelir.
sync-error-prefix = Eşitleme hatası

# Phase 26 — real-time mirror watcher. "Live mirror" starts a
# filesystem watcher on the pair's left side; every debounced event
# triggers a re-sync. The watcher filters vim swap files / Office
# lock files / atomic-save temp names so one logical save becomes
# exactly one sync round.
live-mirror-start = Canlı yansıtmayı başlat
live-mirror-stop = Canlı yansıtmayı durdur
live-mirror-watching = İzleniyor
live-mirror-toggle-hint = Algılanan her dosya sistemi değişikliğinde otomatik yeniden eşitle. Etkin çift başına bir arka plan iş parçacığı.
watch-event-prefix = Dosya değişikliği
watch-overflow-recovered = İzleyici arabelleği taştı; kurtarmak için yeniden sıralanıyor

# Phase 27 — content-defined chunk store. Enables delta-resume (a
# retry only re-writes chunks that actually changed) and same-job
# dedup (files sharing content blocks store those blocks once). The
# store is disk-backed under `<data-dir>/chunks/` by default and is
# the foundation for the Phase 49–51 moonshot repository phases.
chunk-store-section = Yığın deposu
chunk-store-enable = Yığın deposunu etkinleştir (delta sürdürme ve yinelenenleri ayıklama)
chunk-store-enable-hint = Kopyalanan her dosyayı içeriğine göre (FastCDC) böler ve yığınları içerik adresli depolar. Yeniden denemeler yalnızca değişen yığınları yeniden yazar; içerik paylaşan dosyalar otomatik olarak yinelenenleri ayıklar.
chunk-store-location = Yığın deposu konumu
chunk-store-max-size = En fazla yığın deposu boyutu
chunk-store-prune = Şu süreden eski yığınları buda (gün)
chunk-store-savings = Yığın yinelenenleri ayıklama ile { $gib } GiB kazanıldı
chunk-store-disk-usage = { $chunks } yığın genelinde { $size } kullanılıyor

# Phase 28 — tray-resident Drop Stack. The stack is a persistent list
# of paths gathered from multiple sources (Explorer drag, main-window
# context menu, CLI, drag onto the Drop Stack window) that the user
# can dispatch to a destination in one go.
dropstack-window-title = Drop Stack
dropstack-tray-open = Drop Stack
dropstack-empty-title = Drop Stack boş
dropstack-empty-hint = Buraya Explorer'dan dosya sürükleyin ya da eklemek için bir iş satırına sağ tıklayın.
dropstack-add-to-stack = Drop Stack'e ekle
dropstack-copy-all-to = Tümünü şuraya kopyala…
dropstack-move-all-to = Tümünü şuraya taşı…
dropstack-clear = Yığını temizle
dropstack-remove-row = Yığından kaldır
dropstack-path-missing-toast = { $path } bırakıldı — dosya artık yok.
dropstack-always-on-top = Drop Stack'i her zaman üstte tut
dropstack-show-tray-icon = Freally File Manager tepsi simgesini göster
dropstack-open-on-start = Uygulama başlangıcında Drop Stack'i otomatik aç
dropstack-count = { $count } yol

# Phase 29 — spring-loaded folders + native DnD polish. The Settings
# → General tab carries the knobs; DropTarget / DestinationPicker
# Svelte components use the dropzone-* keys at runtime.
settings-dnd-heading = Sürükle ve bırak
settings-dnd-spring-load = Sürüklerken klasörleri otomatik aç
settings-dnd-spring-delay = Otomatik açma gecikmesi (ms)
settings-dnd-thumbnails = Sürükleme küçük resimlerini göster
settings-dnd-invalid-highlight = Geçersiz bırakma hedeflerini vurgula
dropzone-invalid-title = Geçerli bir bırakma hedefi değil
dropzone-invalid-readonly = Hedef salt okunur
dropzone-picker-title = Bir hedef seçin
dropzone-picker-up = Yukarı
dropzone-picker-path = Geçerli yol
dropzone-picker-root = Kökler
dropzone-picker-use-this = Bu klasörü kullan
dropzone-picker-empty = Alt klasör yok
dropzone-picker-cancel = İptal

# Phase 30 — cross-platform path translation. Settings → Transfer
# exposes these under a "Cross-platform compatibility" subsection.
translate-heading = Platformlar arası uyumluluk
translate-unicode-label = Unicode normalleştirme
translate-unicode-auto = Hedefi otomatik algıla
translate-unicode-windows = NFC (Windows / Linux)
translate-unicode-macos = Olduğu gibi bırak (macOS / APFS)
translate-line-endings-label = Metin dosyaları için satır sonlarını çevir
translate-line-endings-allowlist = Metin dosyası uzantıları
reserved-name-label = Windows ayrılmış ad işleme
reserved-name-suffix = "_" ekle (CON.txt → CON_.txt)
reserved-name-reject = Reddet ve uyar
long-path-label = 260 karakteri aşınca Windows uzun yol önekini (\\?\) kullan
long-path-hint = Bazı ağ paylaşımları ve eski araçlar \\?\ ad alanını desteklemez.

# Phase 31 — power-aware copying. Settings → Power & State tab + the
# header badge that renders "⏸ Paused — Zoom call detected" when the
# runner's power subscriber has paused or capped due to a policy match.
power-heading = Güç ve Durum
power-enabled = Güce duyarlı kuralları etkinleştir
power-battery-label = Pildeyken
power-metered-label = Sınırlı Wi-Fi'de
power-cellular-label = Hücreseldeyken
power-presentation-label = Sunum yaparken (Zoom / Teams / Keynote)
power-fullscreen-label = Bir uygulama tam ekrandayken
power-thermal-label = CPU ısı kısıtlaması yaparken
power-rule-continue = Tam hızda devam et
power-rule-pause = Tüm işleri duraklat
power-rule-cap = Bant genişliğini sınırla
power-rule-cap-percent = Geçerli hızın bir yüzdesine sınırla
power-reason-on-battery = pildeyken
power-reason-metered-network = sınırlı ağ
power-reason-cellular-network = hücresel ağ
power-reason-presenting = sunum modu
power-reason-fullscreen = tam ekran uygulaması
power-reason-thermal-throttling = CPU kısıtlama yapıyor

# Phase 32 — cloud backend matrix via OpenDAL. Settings → Remotes
# tab + the Add-backend wizard that writes one entry per remote into
# the keychain-backed credential store.
remote-heading = Uzak arka uçlar
remote-add = Arka uç ekle
remote-list-empty = Yapılandırılmış uzak arka uç yok
remote-test = Bağlantıyı sına
remote-test-success = Bağlantı başarılı
remote-test-failed = Bağlantı başarısız
remote-remove = Arka ucu kaldır
remote-name-label = Görünen ad
remote-kind-label = Arka uç türü
remote-save = Arka ucu kaydet
remote-cancel = İptal
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
backend-local-fs = Yerel dosya sistemi
cloud-config-bucket = Kova
cloud-config-region = Bölge
cloud-config-endpoint = Uç nokta URL'si
cloud-config-root = Kök yol
cloud-error-invalid-config = Arka uç yapılandırması geçersiz
cloud-error-network = Arka uca bağlanırken ağ hatası
cloud-error-not-found = İstenen yolda nesne bulunamadı
cloud-error-permission = Uzak arka uç izni reddetti
cloud-error-keychain = İşletim sistemi anahtar zinciri erişimi başarısız oldu
settings-tab-remotes = Uzak konumlar
settings-tab-mobile = Mobil

# Phase 33 — mount Freally File Manager's chunk store + history archive as a
# read-only filesystem (FUSE / WinFsp). Surfaced by the History tab
# context menu's "Mount snapshot" action + the Settings → Advanced
# "Mount latest on launch" toggle.
mount-heading = Anlık görüntü bağla
mount-action-mount = Anlık görüntü bağla
mount-action-unmount = Bağlantıyı kaldır
mount-status-mounted = { $path } konumunda bağlandı
mount-error-unsafe-mountpoint = Bağlama noktası yolu güvenli değil
mount-error-mountpoint-not-empty = Bağlama noktası boş bir dizin olmalıdır
mount-error-backend-unavailable = Bağlama arka ucu bu sistemde kullanılamıyor
mount-error-archive-read = Arşiv okuması başarısız oldu
mount-picker-title = Bağlama noktası dizinini seçin
mount-toast-mounted = Anlık görüntü { $path } konumunda bağlandı
mount-toast-unmounted = Anlık görüntü bağlantısı kaldırıldı
mount-toast-failed = Bağlama başarısız: { $reason }
settings-mount-heading = Anlık görüntüleri bağla
settings-mount-hint = Geçmiş arşivini salt okunur bir dosya sistemi olarak sun. Aşama 33b çalıştırma akışını bağlar; çekirdek FUSE/WinFsp arka uçları Aşama 33c'de gelir.
settings-mount-on-launch = En son anlık görüntüyü başlangıçta bağla
settings-mount-on-launch-path = Bağlama noktası yolu
settings-mount-on-launch-path-placeholder = ör. C:\Mounts\copythat

# Phase 34 — enterprise-grade audit log export + WORM (write-once-read-
# many) tamper-resistance. Settings → Advanced → Audit log collects the
# format + destination + rotation + WORM toggle; runtime records job and
# file events into the tamper-evident chain-hashed sink.
settings-audit-heading = Denetim günlüğü
settings-audit-hint = Her iş ve dosya olayının yalnızca eklemeli, kurcalamaya karşı kanıtlı günlüğü. Biçimler CSV, JSON satırları, RFC 5424 Syslog, ArcSight CEF ve QRadar LEEF içerir.
settings-audit-enable = Denetim günlüğünü etkinleştir
settings-audit-format = Günlük biçimi
settings-audit-format-json-lines = JSON satırları (önerilen varsayılan)
settings-audit-format-csv = CSV (elektronik tabloya uygun)
settings-audit-format-syslog = Syslog (RFC 5424)
settings-audit-format-cef = CEF (ArcSight)
settings-audit-format-leef = LEEF 2.0 (IBM QRadar)
settings-audit-file-path = Günlük dosyası yolu
settings-audit-file-path-placeholder = ör. C:\ProgramData\CopyThat\audit.log
settings-audit-max-size = Şu boyuttan sonra döndür (bayt, 0 = asla)
settings-audit-worm = WORM modunu etkinleştir (bir kez yaz, çok kez oku)
settings-audit-worm-hint = Her oluşturma veya döndürmeden sonra platformun yalnızca eklemeli bayrağını uygular (Linux chattr +a, macOS chflags uappnd, Windows salt okunur özniteliği). Bir yönetici bile günlüğü kısaltmak için bayrağı açıkça temizlemelidir.
settings-audit-test-write = Yazmayı sına
settings-audit-verify-chain = Zinciri doğrula
toast-audit-test-write-ok = Denetim günlüğü yazma sınaması başarılı oldu
toast-audit-verify-ok = Denetim zinciri sağlam olarak doğrulandı
toast-audit-verify-failed = Denetim zinciri doğrulaması uyuşmazlık bildirdi

# Phase 35 — destination encryption via age (X25519 / passphrase /
# SSH) + on-the-fly zstd compression with a per-extension deny
# heuristic. Surfaced through Settings → Transfer → Encryption +
# Compression; engine short-circuits to the crypt pipeline when
# either stage is active.
settings-crypt-heading = Şifreleme ve sıkıştırma
settings-crypt-hint = Dosya içeriklerini hedefe ulaşmadan önce dönüştürün. Şifreleme age biçimini kullanır; sıkıştırma zstd kullanır ve uzantıya göre zaten sıkıştırılmış medyayı atlayabilir.
settings-crypt-encryption-mode = Şifreleme
settings-crypt-encryption-off = Kapalı
settings-crypt-encryption-passphrase = Parola (kopyalama başında sorulur)
settings-crypt-encryption-recipients = Dosyadan alıcı anahtarları
settings-crypt-encryption-hint = Parolalar yalnızca kopyalama süresince bellekte tutulur. Alıcı dosyaları satır başına bir age1… veya ssh- ortak anahtarı listeler.
settings-crypt-recipients-file = Alıcılar dosyası yolu
settings-crypt-recipients-file-placeholder = ör. C:\Users\me\recipients.txt
settings-crypt-compression-mode = Sıkıştırma
settings-crypt-compression-off = Kapalı
settings-crypt-compression-always = Her zaman
settings-crypt-compression-smart = Akıllı (zaten sıkıştırılmış medyayı atla)
settings-crypt-compression-hint = Akıllı mod, zstd'den yarar görmeyen jpg, mp4, zip, 7z ve benzeri biçimleri atlar. Her zaman modu her dosyayı seçilen düzeyde sıkıştırır.
settings-crypt-compression-level = zstd düzeyi (1-22)
settings-crypt-compression-level-hint = Düşük sayılar daha hızlıdır; yüksek sayılar daha sıkı sıkıştırır. Düzey 3, zstd'nin CLI varsayılanıyla eşleşir.
compress-footer-savings = 💾 { $original } → { $compressed } (%{ $percent } kazanç)
compress-savings-toast = %{ $percent } sıkıştırıldı ({ $bytes } kazanıldı)
crypt-toast-recipients-loaded = { $count } şifreleme alıcısı yüklendi
crypt-toast-recipients-error = Alıcılar yüklenemedi: { $reason }
crypt-toast-passphrase-required = Şifreleme, kopyalama başlamadan önce bir parola gerektirir
crypt-toast-passphrase-set = Şifreleme parolası yakalandı
crypt-footer-encrypted-badge = 🔒 Şifrelendi (age)
crypt-footer-compressed-badge = 📦 Sıkıştırıldı (zstd)

# Phase 36 — copythat CLI surface. Documented exit codes + the
# user-facing strings for plan/apply/verify/config. Help text stays
# in English (engineering accessibility) but error / info / status
# strings are localized so a localized desktop installation surfaces
# CLI errors in the same language as the GUI.
cli-help-tagline = Freally File Manager CLI — CI/CD ardışık düzenleri için bayt düzeyinde tam dosya kopyalama, eşitleme, doğrulama ve denetim.
cli-help-exit-codes = Çıkış kodları: 0 başarı, 1 hata, 2 bekliyor, 3 çakışma, 4 doğrulama-başarısız, 5 ağ, 6 izin, 7 disk-dolu, 8 iptal, 9 yapılandırma.
cli-error-bad-args = copy/move en az bir kaynak ve bir hedef gerektirir
cli-error-unknown-algo = Bilinmeyen doğrulama algoritması: { $algo }
cli-error-missing-spec = plan/apply için --spec gereklidir
cli-error-spec-parse = jobspec { $path } ayrıştırılamadı: { $reason }
cli-error-spec-empty-sources = Jobspec kaynak listesi boş
cli-info-shape-recorded = Bant genişliği şekli "{ $rate }" kaydedildi; uygulama copythat-shape aracılığıyla bağlanır
cli-info-stub-deferred = { $command } Aşama 36 takip bağlantısı için hazırlandı
cli-plan-summary = Plan: { $actions } eylem, { $bytes } bayt; { $already_done } zaten yerinde
cli-plan-pending = Plan bekleyen eylemler bildiriyor; yürütmek için `apply` ile yeniden çalıştırın
cli-plan-already-done = Plan yapılacak bir şey olmadığını bildiriyor (etkisiz)
cli-apply-success = Apply hatasız tamamlandı
cli-apply-failed = Apply bir veya daha fazla hatayla tamamlandı
cli-verify-ok = Doğrulama tamam: { $algo } { $digest }
cli-verify-failed = { $path } için doğrulama BAŞARISIZ ({ $algo })
cli-config-set = { $key } = { $value } ayarlandı
cli-config-reset = { $key } varsayılana sıfırlandı
cli-config-unknown-key = Bilinmeyen yapılandırma anahtarı: { $key }
cli-completions-emitted = { $shell } için kabuk tamamlamaları stdout'a yazdırıldı

# Phase 37 — desktop-side mobile companion. Settings → Mobile panel
# strings + the SAS-confirmation modal + push-notification toasts.
# The actual mobile UI lives in the Phase 37 follow-up Tauri Mobile
# target; the phone displays its own localized SAS prompt.
settings-mobile-heading = Mobil yardımcı
settings-mobile-hint = Geçmişe göz atmak, kaydedilmiş profilleri ve Aşama 36 jobspec'lerini başlatmak ve tamamlanma bildirimleri almak için bir iPhone veya Android telefon eşleştirin.
settings-mobile-pair-toggle = Yeni eşleştirmelere izin ver
settings-mobile-pair-active = Eşleştirme sunucusu etkin — QR'yi Freally File Manager mobil uygulamasıyla tarayın
settings-mobile-pair-button = Eşleştirmeyi başlat
settings-mobile-revoke-button = İptal et
settings-mobile-no-pairings = Henüz eşleştirilmiş cihaz yok
settings-mobile-pair-port = Bağlama bağlantı noktası (0 = boş olanı seç)
pair-sas-prompt = Her iki ekran da aynı dört emojiyi göstermeli. Aynılarsa Eşleşiyor'a dokunun.
pair-sas-confirm = Eşleşiyor
pair-sas-reject = Uyuşmuyor — iptal et
pair-toast-success = { $device } ile eşleştirildi
pair-toast-failed = Eşleştirme başarısız: { $reason }
push-toast-sent = { $device } cihazına push gönderildi
push-toast-failed = { $device } cihazına push başarısız: { $reason }

# Phase 38 — aggregate destination dedup + reflink fallback ladder.
# Settings → Transfer → Dedup panel + per-job-row badges (⚡
# Reflinked / 🔗 Hardlinked / 🧩 Chunk-shared / 📋 Copied) +
# pre-pass dedup-scan modal.
settings-dedup-heading = Hedef yinelenenleri ayıklama
settings-dedup-hint = Kaynak ve hedef aynı birimi paylaştığında Freally File Manager, baytları kopyalamak yerine dosyaları dosya sistemi düzeyinde klonlayabilir. Reflink anlık ve güvenlidir; hardlink daha hızlıdır ama her iki ad da durumu paylaşır.
settings-dedup-mode-auto = Otomatik merdiven (reflink → hardlink → yığın → kopya)
settings-dedup-mode-reflink-only = Yalnızca reflink
settings-dedup-mode-hardlink-aggressive = Agresif (yazılabilir dosyalarda bile reflink + hardlink)
settings-dedup-mode-off = Devre dışı (her zaman bayt kopyası)
settings-dedup-hardlink-policy = Hardlink ilkesi
settings-dedup-prescan = Yinelenen içerik için hedef ağacını önceden tara
dedup-badge-reflinked = ⚡ Reflink yapıldı
dedup-badge-hardlinked = 🔗 Hardlink yapıldı
dedup-badge-chunk-shared = 🧩 Yığın paylaşımlı
dedup-badge-copied = 📋 Kopyalandı
phase42-paranoid-verify-label = Paranoyak doğrulama
phase42-paranoid-verify-hint = Hedefin önbelleğe alınmış sayfalarını atar ve yazma önbelleği yalanlarını ve sessiz bozulmayı yakalamak için diskten yeniden okur. Varsayılan doğrulamadan yaklaşık %50 daha yavaştır; varsayılan olarak kapalı.
phase42-sharing-violation-retries-label = Kilitli kaynak dosyalarda yeniden deneme sayısı
phase42-sharing-violation-retries-hint = Başka bir işlem kaynak dosyayı özel kilitle açık tuttuğunda kaç kez yeniden denenecek. Geri çekilme her denemede iki katına çıkar (varsayılan olarak 50 ms / 100 ms / 200 ms). Varsayılan 3, Robocopy /R:3 ile eşleşir.
phase42-cloud-placeholder-warning = { $name } yalnızca bulutta bulunan bir OneDrive dosyasıdır. Kopyalamak bir indirmeyi tetikler — ağ bağlantınız üzerinden { $size } kadar.
phase42-defender-exclusion-hint = En yüksek kopyalama verimi için toplu aktarımlardan önce hedef klasörü Microsoft Defender hariç tutmalarına ekleyin. docs/PERFORMANCE_TUNING.md belgesine bakın.

# Phase 39 — Browser-accessible recovery UI. Settings → Advanced
# exposes these strings; the recovery server itself renders askama
# templates that always read English — these eight keys are the
# Settings prose the user actually reads in their preferred locale.
settings-recovery-heading = Kurtarma web arayüzü
settings-recovery-enable = Kurtarma web arayüzünü etkinleştir
settings-recovery-bind-address = Bağlama adresi
settings-recovery-port = Bağlantı noktası (0 = boş olanı seç)
settings-recovery-show-url = URL ve belirteci göster
settings-recovery-rotate-token = Belirteci döndür
settings-recovery-allow-non-loopback = Geri döngü dışı bağlamaya izin ver
settings-recovery-non-loopback-warning = UYARI: geri döngü dışı bağlamayı etkinleştirmek kurtarma arayüzünü yerel ağınıza açar. Belirteci öğrenen herkes dosya geçmişinize göz atıp dosya indirebilir. LAN güvenilir değilse önüne TLS veya ters proxy koyun.

# Phase 40 — SMB compression negotiation + cloud-VM offload helper.
# 6 SMB keys (header badge + Settings prose) + 6 cloud-offload keys
# (Remotes tab wizard for cross-cloud copy templates).
smb-compress-badge = 🗜 SMB sıkıştırma: { $algo }
smb-compress-badge-tooltip = Bu hedefe giden ağ trafiği aktarım sırasında sıkıştırılıyor (SMB 3.1.1).
smb-compress-toast-saved = Ağ üzerinden { $bytes } kazanıldı
smb-compress-algo-unknown = bilinmeyen algoritma
settings-smb-compress-heading = SMB ağ sıkıştırması
settings-smb-compress-hint = UNC hedeflerinde SMB 3.1.1 trafik sıkıştırmasını otomatik olarak müzakere et. Yavaş bağlantılarda bedava kazanç; yerel hedeflerde yoksayılır.
cloud-offload-heading = Bulut VM boşaltma yardımcısı
cloud-offload-hint = Doğrudan iki bulut arasında kopyalarken, kopyalamayı bulutta küçük bir geçici VM'den çalıştıran bir dağıtım şablonu oluştur — baytlar dizüstü bilgisayarınızın ağına hiç dokunmaz.
cloud-offload-render-button = Şablonu oluştur
cloud-offload-copy-clipboard = Panoya kopyala
cloud-offload-template-format = Şablon biçimi
cloud-offload-self-destruct-warning = VM, { $minutes } dakika sonra otomatik kapanır — dağıtmadan önce IAM rolünü ve bölgeyi doğrulayın.

# Phase 41 — animated before/after tree-diff preview. The `Preview
# changes` modal renders the rolled-up plan before the engine starts
# work; 14 keys cover the title, the summary header / counts, the
# row-reason labels, and the two action buttons.
preview-modal-title = Değişiklikleri önizle
preview-summary-header = Ne olacak
preview-category-additions = { $count } ekleme
preview-category-replacements = { $count } değiştirme
preview-category-skips = { $count } atlandı
preview-category-conflicts = { $count } çakışma
preview-category-unchanged = { $count } değişmedi
preview-bytes-to-transfer = aktarılacak { $bytes }
preview-reason-source-newer = Kaynak daha yeni
preview-reason-dest-newer = Hedef daha yeni — atlanacak
preview-reason-content-different = İçerik farklı
preview-reason-identical = Kaynakla aynı
preview-button-run = Planı çalıştır
preview-button-reduce = Planımı küçült…

# Phase 42 — perceptual-hash visual-similarity dedup. Eight keys cover
# the pre-copy "looks visually identical" warning + the Settings panel.
perceptual-warn-title = Görsel olarak aynı görünüyor
perceptual-warn-body = Hedefteki { $name }, kaynak resimle eşleşiyor gibi görünüyor. Yine de kopyalamaya devam edilsin mi?
perceptual-warn-keep-both = İkisini de tut
perceptual-warn-skip = Bu dosyayı atla
perceptual-warn-overwrite = Yine de üzerine yaz
perceptual-settings-heading = Görsel benzerlik yinelenenleri ayıklama
perceptual-settings-hint = Hedefteki görsel olarak aynı görüntüleri, üzerlerine yazılmadan önce algıla. Karma, bayt düzeyinde değil algısaldır (aynı resmi farklı bir biçimde yeniden kaydedilmiş olarak tanır).
perceptual-settings-threshold-label = Uyarı eşiği (düşük = daha sıkı eşleşme)

# Phase 42 Part B — per-file rolling versions (Time Machine for any
# destination). 8 keys cover the version-list panel + retention picker.
version-list-heading = Önceki sürümler
version-list-empty = Bu dosyanın önceki sürümü yok
version-list-restore = Bu sürümü geri yükle
version-retention-heading = Üzerine yazınca önceki sürümleri tut
version-retention-none = Her sürümü sonsuza dek tut
version-retention-last-n = Son { $n } sürümü tut
version-retention-older-than-days = { $days } günden eski sürümleri at
version-retention-gfs = Saatlik { $h } · günlük { $d } · haftalık { $w } · aylık { $m }

# Phase 43 — forensic chain-of-custody manifests + BLAKE3 verified
# streaming. 14 keys cover the Settings → Provenance panel
# (heading, hint, toggles, TSA URL, signing-key management) plus
# the post-job manifest notification + the verify command's two
# headline result lines.
provenance-settings-heading = Adli vesayet zinciri
provenance-settings-hint = Her kopyalama işini BLAKE3 + ed25519 bildirimiyle imzalayın. İncelemeciler hedef ağacı sonra yeniden karmalayıp kopyalamadan bu yana hiçbir baytın değişmediğini kanıtlayabilir.
provenance-settings-enable-default = Her yeni işi varsayılan olarak imzala
provenance-settings-show-after-job = Tamamlanan her işin ardından bildirimi göster
provenance-settings-tsa-url-label = Varsayılan RFC 3161 zaman damgası yetkilisi URL'si
provenance-settings-tsa-url-hint = İsteğe bağlı. Ayarlandığında bildirimler, baytların bu zaman noktasında var olduğunu kanıtlayan ücretsiz bir TSA zaman damgası taşır. Atlamak için boş bırakın.
provenance-settings-keys-heading = İmzalama anahtarları
provenance-settings-keys-generate = Yeni anahtar oluştur
provenance-settings-keys-import = Anahtar içe aktar…
provenance-settings-keys-export = Ortak anahtarı dışa aktar…
provenance-job-completed-title = Köken bildirimi kaydedildi
provenance-job-completed-body = { $count } dosya imzalandı → { $path }
provenance-verify-clean = Bildirim { $count } dosya için geçerli; imza { $sig }; merkle kökü tamam.
provenance-verify-tampered = Bildirim GEÇERSİZ — { $tampered } kurcalanmış, { $missing } eksik.
# Phase 43 post-review hardening — toast text for the Settings →
# Provenance buttons whose Tauri IPC has not yet landed.
provenance-action-staged = Aşama 43 — bu eylem için IPC bağlantısı bir takip işlemesinde gelir.

# Phase 44 — SSD-aware whole-drive sanitize (NVMe Sanitize / OPAL
# Crypto Erase) + the per-file shred refusal on copy-on-write
# filesystems. 16 keys cover the new "Drive sanitize" Settings
# subsection and the localized error messages.
sanitize-heading = Tüm sürücüyü güvenli temizleme
sanitize-hint = NVMe Sanitize, OPAL Crypto Erase ve ATA Secure Erase, bir flash sürücüyü milisaniyeler içinde donanım yazılımı katmanında siler. Dosya başına üzerine yazma flash'ta anlamsızdır — çok geçişli parçalama yalnızca NAND'ı yıpratır. Gerçek temizleme için bunu kullanın.
sanitize-pick-device = Temizlenecek sürücüyü seçin
sanitize-mode-label = Temizleme yöntemi
sanitize-mode-nvme-format = NVMe Format (güvenli silmeli)
sanitize-mode-nvme-sanitize-block = NVMe Sanitize — Blok Silme (yavaş, her hücre)
sanitize-mode-nvme-sanitize-crypto = NVMe Sanitize — Crypto Erase (anlık)
sanitize-mode-ata-secure-erase = ATA Secure Erase (eski SATA SSD'leri)
sanitize-mode-opal-crypto-erase = TCG OPAL Crypto Erase (Kendinden Şifrelemeli Sürücüler)
sanitize-mode-apfs-crypto-erase = APFS Crypto Erase (FileVault anahtarını döndür, yalnızca macOS)
sanitize-confirm-1 = Bu, { $device } üzerindeki HER baytı yok eder. Geri alınamaz.
sanitize-confirm-2 = { $device } üzerindeki tüm bölümlerin, tüm dosyaların ve tüm anlık görüntülerin kalıcı olarak okunamaz hâle geleceğini anlıyorum.
sanitize-confirm-3 = Devam etmek için sürücünün model adını yazın: { $model }
sanitize-running = { $device } temizleniyor ({ $mode }) — bu, milisaniyelerden (crypto erase) onlarca dakikaya (blok silme) kadar sürebilir. Gücü kapatmayın.
sanitize-completed = Temizleme tamamlandı — { $device } artık boş.
ssd-honest-shred-meaningless = Bir kopyala-yaz dosya sisteminde (Btrfs / ZFS / APFS) dosya başına parçalama, alttaki bloklara ulaşamaz. Bunun yerine tüm sürücüyü güvenli temizleyin ve tam disk şifreleme anahtarını döndürün.
ssd-honest-advisory = Bu dosya flash üzerinde bulunuyor. Dosya başına üzerine yazma NAND yıpranmasına yol açar ve özgün hücrelerin kurtarılamaz olduğunu GARANTİ ETMEZ. Hassas veriler için tüm sürücüyü güvenli temizleyin.

# Phase 44.1f post-review — placeholder toast for SanitizeTab
# buttons whose Tauri IPC has not yet landed.
sanitize-action-staged = Aşama 44.1 — bu eylem için IPC bağlantısı bir takip işlemesinde gelir.

# Phase 45.3 — named-queue tab strip (Subfeature A). Tabs surface
# once the QueueRegistry holds at least one queue; the synthesised
# default tab keeps legacy single-queue jobs reachable.
queue-tab-default = Varsayılan
queue-tab-empty-state = İş kuyrukları
queue-badge-tooltip = Bu kuyruktaki bekleyen ve çalışan işler

# Phase 45.4 — drag-progress-merge (Subfeature B). Drag a queue tab
# onto another to merge their job lists. The default tab is neither
# draggable nor a drop target; only registry queues participate.
queue-drag-hint = Birleştirmek için başka bir kuyruğun üzerine sürükleyin
queue-merge-confirm = Birleştirmek için bırakın
queue-merge-toast = Kuyruklar birleştirildi

# Phase 45.5 — F2-queue UX (Subfeature C). F2 toggles
# `auto_enqueue_next` so every fresh enqueue piles into the running
# queue rather than spawning a parallel one. Status pill renders in
# the Footer; pulsing dot renders on the running tab.
queue-f2-active-hint = F2 modu: her yeni kuyruğa alma bu kuyruğa düşer
queue-f2-toggled-on = F2 kuyruk modu AÇIK — yeni kuyruğa almalar çalışan kuyruğa katılır
queue-f2-toggled-off = F2 kuyruk modu KAPALI — yeni kuyruğa almalar paralel kuyruklar oluşturur
queue-f2-status-bar = F2 kuyruk modu: AÇIK

# Phase 45.6 — tray destination targets (Subfeature D). Pinned
# destinations appear in the OS tray menu; clicking one arms it as
# the active drop target so the next file drop bypasses the
# DropStagingDialog. Settings → General hosts the list editor.
tray-target-section-title = Tepsi hedefleri
tray-target-section-hint = Sabitlenmiş hedefler tepsi menüsünde görünür. Bir sonraki bırakma hedefi olarak hazırlamak için birine tıklayın.
tray-target-empty = Henüz sabitlenmiş tepsi hedefi yok.
tray-target-remove = Kaldır
tray-target-add-label = Etiket
tray-target-add-path = Yol veya arka uç URI'si
tray-target-add = Ekle
tray-target-armed-toast = Bir sonraki dosyanızı { $label } hedefine göndermek için bırakın
tray-target-active-pill = → { $label }

# Phase 45.7 follow-up — pinned-destination validation errors. The
# `queue_pin_destination` IPC returns these Fluent keys verbatim
# when input fails the IPC-boundary checks; the toast layer renders
# them via `t(...)` (Toast.svelte detects kebab-case-lowercase and
# routes through the locale table). Phase 17e's `err-path-escape` /
# `err-destination-empty` cover the path-traversal + empty cases for
# `queue_route_job`; the keys below are pin-specific.
err-pinned-destination-label-empty = Tepsi hedefi etiketi boş olamaz.
err-pinned-destination-path-empty = Tepsi hedefi yolu boş olamaz.
err-pinned-destination-label-too-long = Tepsi hedefi etiketi çok uzun (en fazla 64 karakter).
err-pinned-destination-path-too-long = Tepsi hedefi yolu çok uzun (en fazla 1024 karakter).
err-pinned-destination-label-invalid = Tepsi hedefi etiketi izin verilmeyen karakterler içeriyor (satır sonu, satır başı veya NUL).
err-pinned-destination-path-invalid = Tepsi hedefi yolu izin verilmeyen karakterler içeriyor (satır sonu, satır başı veya NUL).
err-pinned-destination-too-many = 50 tepsi hedefi sınırına ulaştınız. Başka bir tane eklemek için birini kaldırın.

# Phase 46.6 — Settings → Plugins tab. The IPC layer in
# `apps/copythat-ui/src-tauri/src/plugin_commands.rs` enumerates the
# per-user plugin store under `<config_dir>/plugins/`; the
# `PluginsTab.svelte` component renders these strings against the
# returned manifest + grant state.
settings-tab-plugins = Eklentiler
plugin-heading = Eklentiler
plugin-hint = Yalıtılmış WASM eklentileri Freally File Manager'i özel kancalarla genişletir. Her eklenti çağrı başına CPU ve bellek sınırları altında çalışır ve yalnızca verdiğiniz ana bilgisayar yeteneklerini görür.
plugin-list-empty = Henüz yüklü eklenti yok.
plugin-enabled = Etkin
plugin-disabled = Devre dışı
plugin-hooks = Kancalar
plugin-capabilities = Yetenekler
plugin-no-capabilities = (yok)
plugin-directory = Konum
plugin-install-from-file = Dosyadan yükle…
plugin-install-from-url = URL'den yükle…
plugin-url-wasm = WASM URL'si
plugin-url-manifest = Bildirim URL'si
plugin-url-hash = BLAKE3 karması
plugin-url-preview = Önizleme
plugin-url-confirm = Yüklemeyi onayla

# Phase 31b — power-policy settings (Power tab).
settings-tab-power = Güç
settings-power-hint = Kopyalamayı güç durumuna göre kısıtla veya duraklat: pil, sayaçlı/hücresel ağ, sunum/tam ekran veya CPU termal kısıtlaması.
settings-power-enabled = Güce duyarlı kısıtlamayı etkinleştir
settings-power-battery = Pildeyken
settings-power-metered = Sayaçlı ağda
settings-power-cellular = Hücresel ağda
settings-power-presentation = Sunum sırasında
settings-power-fullscreen = Tam ekranda
settings-power-thermal = Termal kısıtlamada
settings-power-continue = Devam et
settings-power-pause = Duraklat
err-server-not-implemented = Sunucu modu henüz kullanılamıyor.
err-webhook-not-implemented = Webhook teslimi henüz kullanılamıyor.

# Phase 47 — "why is this slow?" diagnostics (bottleneck badge + tooltip).
bottleneck-source-io = Kaynak I/O
bottleneck-dest-io = Hedef I/O
bottleneck-network = Ağ
bottleneck-antivirus = Antivirüs
bottleneck-cpu = CPU
bottleneck-thermal = Termal
bottleneck-unknown = Bilinmiyor
diag-aria = Darboğaz: { $cause }
diag-tooltip = { $cause } ile sınırlı · { $rate }
diag-spark-aria = Son bir dakikadaki aktarım hızı
diag-keeping-up = Yetişiyor
diag-label = Tanılama

# Phase 48 — server mode + observability (Settings → Server).
settings-tab-server = Sunucu
server-hint = Freally File Manager'i arayüzsüz bir dosya sunucusu olarak çalıştırın. Sunulacak protokolleri seçin, adresi ve sunulacak klasörü ayarlayın ve isteğe bağlı olarak kimlik doğrulaması isteyin.
server-protocols = Protokoller
server-bind-addr = Bağlama adresi
server-root = Sunulan klasör
server-readonly = Salt okunur (yüklemeleri ve silmeleri reddet)
server-auth-mode = Kimlik doğrulama
server-auth-none = Yok
server-auth-bearer = Bearer belirteci
server-auth-basic = Temel (kullanıcı adı + parola)
server-auth-token = Belirteç
server-auth-user = Kullanıcı adı
server-auth-password = Parola
otel-endpoint = OpenTelemetry uç noktası
webhook-section = Web kancaları
webhook-url = Web kancası URL'si
webhook-add = Web kancası ekle
webhook-remove = Kaldır
webhook-empty = Yapılandırılmış web kancası yok.
webhook-pushover-token = Pushover belirteci
webhook-pushover-user = Pushover kullanıcısı
server-start = Sunucuyu başlat
server-stop = Sunucuyu durdur
server-status-running = { $addr } üzerinde çalışıyor
server-status-stopped = Durduruldu
server-metrics-url = Metrikler
err-server-no-protocols = Sunucuyu başlatmadan önce en az bir protokol seçin.
err-server-bind = Sunucu adresi bağlanamadı. Zaten kullanımda olabilir.

# Kitaplık çekmecesi (Phase 49) — birleşik içerik adresli depo görünümü.
footer-library = Kitaplık
library-title = Kitaplık
library-loading = Depo yükleniyor…
library-unavailable = Depo kullanılamıyor
library-tab-live = Canlı
library-tab-snapshots = Anlık görüntüler
library-tab-versions = Sürümler
library-hero-savings = { $effective } etkin veri sunuluyor · { $pct } tasarruf
library-hero-empty = { $chunks } yığın depolandı — henüz anlık görüntü yok
library-stat-stored = Diskte saklanan
library-stat-effective = Etkin veri
library-stat-snapshots = Anlık görüntüler
library-stat-chunks = Benzersiz yığınlar
library-snapshot-empty = Henüz anlık görüntü yok
library-snapshot-files = { $n } dosya
library-version-path-ph = Hedef yol…
library-version-load = Sürümleri göster
library-version-empty = Bu yol için sürüm yok
repo-kind-copy = Kopya
repo-kind-sync = Eşitleme
repo-kind-version = Sürüm
repo-kind-backup = Yedek
# Phase 49o — snapshot diff / compare.
library-tab-compare = Karşılaştır
repo-change-added = Eklendi
repo-change-removed = Kaldırıldı
repo-change-modified = Değiştirildi
repo-change-unchanged = Değişiklik yok
repo-diff-summary = { $added } eklendi · { $removed } kaldırıldı · { $modified } değiştirildi
repo-diff-bytes-added = { $bytes } yeni
repo-diff-pick-two = Karşılaştırmak için iki anlık görüntü seçin
# Phase 49r — statistics / reports.
library-tab-reports = Raporlar
report-growth-title = Depolama büyümesi
report-by-kind-title = Türe göre
report-top-files-title = Öne çıkan dosyalar
report-dedup-ratio = %{ $pct } tekilleştirildi
report-export = Raporu dışa aktar
report-exported = Rapor { $path } konumuna kaydedildi
report-file-versions = { $n } sürüm
# Phase 49p — pinning / prune.
repo-pin = Sabitle
repo-unpin = Sabitlemeyi kaldır
repo-pinned-badge = Sabitlendi
repo-prune-title = Buda
repo-prune-keep-last = En yenileri tut
repo-prune-removed = { $n } anlık görüntü budandı
repo-prune-none = Budanacak bir şey yok

# Phase 49c — yedekleme kaynakları.
library-tab-sources = Kaynaklar
backup-add-source = Kaynak ekle…
backup-source-path-ph = Yedeklenecek klasör…
backup-exclude-ph = Hariç tutma kalıpları (virgülle ayrılmış)
backup-now = Şimdi yedekle
backup-remove = Kaldır
backup-empty = Henüz yedekleme kaynağı yok
backup-never-run = Hiç yedeklenmedi
backup-last-run = Son yedekleme { $when }
backup-running = Yedekleniyor… { $files } dosya
backup-toast-started = { $label } yedekleniyor…
backup-toast-completed = { $label } yedeklendi: { $files } dosya
backup-toast-failed = { $label } yedeklemesi başarısız: { $reason }
# Phase 49e — per-source retention + prune.
backup-retention = Saklama
backup-retention-keep-all = Tümünü tut
backup-retention-last = Son { $n } tanesini tut
backup-retention-days = { $days } günden eski
backup-retention-gfs = GFS döngüsü
backup-prune-now = Şimdi buda
backup-prune-none = Budanacak bir şey yok
backup-prune-result = { $removed } anlık görüntü kaldırıldı · { $bytes } geri kazanıldı
# Phase 49f — per-source scheduling.
backup-schedule = Zamanlama
backup-schedule-manual = Manuel
backup-schedule-hourly = Saatlik
backup-schedule-daily = Günlük
backup-schedule-weekly = Haftalık
backup-next-run = Sonraki çalıştırma { $when }
backup-not-scheduled = Zamanlanmadı
# Phase 49g — source filters.
backup-include-ph = Dahil etme glob'ları (virgülle ayrılmış)
backup-skip-hidden = Gizlileri atla
# Phase 49q — notifications.
notify-title = Bildirimler
notify-on-success = Başarılı olduğunda
notify-on-failure = Başarısız olduğunda
notify-test = Test gönder
notify-test-sent = { $n } hedefe test gönderildi

# Phase 49d — geri yükleme tarayıcısı.
restore-browse = Geri yükle…
restore-title = Anlık görüntüden geri yükle
restore-select-all = Tümünü seç
restore-dest = Şuraya geri yükle
restore-confirm = { $n } dosyayı geri yükle
restore-empty = Bu anlık görüntüde dosya yok
restore-conflict-body = Seçilen { $count } dosya hedefte zaten var.
restore-conflict-overwrite = Üzerine yaz
restore-conflict-skip = Var olanları atla
restore-conflict-keep-both = İkisini de tut
restore-toast-done = { $restored } geri yüklendi, { $skipped } atlandı
restore-toast-failed = Geri yükleme başarısız: { $reason }
snapshot-forget = Unut
snapshot-forget-toast = Anlık görüntü unutuldu — boşaltmak için Alanı geri kazan'ı çalıştırın
library-reclaim = Alanı geri kazan
# Phase 49i — full compaction.
library-compact = Tam birleştirme
library-compact-started = Birleştirme başlatıldı — Görevler'i izleyin
# Phase 49h — compression.
library-stat-compression = Sıkıştırmayla kazanılan
storage-compression = Sıkıştırma
storage-compression-off = Kapalı
storage-compression-auto = Otomatik (sıkıştırılamayanları atla)
storage-compression-always = Her zaman
storage-compression-restart = Sonraki başlatmada uygulanır
# Phase 49j — tasks & progress center.
footer-tasks = Görevler
tasks-title = Görevler
tasks-empty = Henüz görev yok
tasks-running = Çalışıyor
tasks-recent = Son
tasks-cancel = İptal
task-state-running = Çalışıyor
task-state-completed = Tamamlandı
task-state-failed = Başarısız
task-state-cancelled = İptal edildi
# Phase 49k — repository setup/connect wizard.
repo-wizard-title = Depoya bağlan
repo-wizard-create-tab = Yeni oluştur
repo-wizard-connect-tab = Mevcuda bağlan
repo-field-name = Ad
repo-field-path = Konum
repo-field-password = Parola (isteğe bağlı)
repo-action-create = Oluştur
repo-action-connect = Bağlan
repo-action-browse = Gözat…
repo-switcher-label = Depo
repo-action-forget = Unut
repo-action-change-pass = Parolayı değiştir…
repo-password-old = Mevcut parola
repo-password-new = Yeni parola
repo-error-exists = Bu konumda zaten bir depo var
repo-error-not-found = Bu konumda depo bulunamadı
repo-error-bad-pass = Yanlış parola
repo-note-no-encryption = Parola yalnızca erişimi denetler; durağan veri şifrelemesi sonraki bir sürümde gelecek
repo-confirm-forget = "{ $name }" listeden kaldırılsın mı? Verileriniz diskte kalır.
repo-toast-created = "{ $name }" deposu oluşturuldu
repo-toast-connected = "{ $name }" deposuna bağlanıldı
repo-toast-pass-changed = Parola güncellendi
# Phase 49l — Sources dashboard.
library-tab-overview = Genel bakış
library-source-empty = Henüz kaynak yok
library-source-unknown = (belirtilmemiş kaynak)
library-source-snapshots = { $n } anlık görüntü
library-source-latest = En son { $when }
# Phase 49n — verify & repair.
repo-action-verify = Doğrula
repo-action-verify-deep = Doğrula (tüm verileri oku)
repo-action-repair = Onar…
repo-verify-clean = { $files } dosya / { $chunks } yığın doğrulandı — hasar yok
repo-verify-damaged = { $missing } eksik, { $corrupt } bozuk yığın
repo-repair-confirm = Artık geri yüklenemeyen { $n } anlık görüntü kaldırılsın mı?
repo-repair-removed = { $n } hasarlı anlık görüntü kaldırıldı
repo-repair-none = Onarılacak bir şey yok — depo temiz
repo-gc-done = { $bytes } geri kazanıldı ({ $chunks } yığın)
restore-toast-partial = { $restored } geri yüklendi, { $skipped } atlandı, { $failed } başarısız
