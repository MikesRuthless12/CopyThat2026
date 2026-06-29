app-name = Copy That v0.19.84
window-title = Copy That v0.19.84
shred-ssd-advisory = تحذير: هذا الهدف موجود على قرص SSD. عمليات الكتابة فوق البيانات متعددة المرات لا تمسح ذاكرة الفلاش بشكل موثوق، لأن موازنة التآكل والتخصيص الزائد ينقلان البيانات بعيدًا عن عنوان الكتلة المنطقي. بالنسبة لوسائط الحالة الصلبة، يُفضّل استخدام ATA SECURE ERASE أو NVMe Format مع المسح الآمن أو التشفير الكامل للقرص مع التخلص من المفتاح.

# Global aggregate states (header pill)
state-idle = خامل
state-copying = جارٍ النسخ
state-verifying = جارٍ التحقق
state-paused = متوقف مؤقتًا
state-error = خطأ

# Per-job states (row badge)
state-pending = في قائمة الانتظار
state-running = قيد التشغيل
state-cancelled = أُلغي
state-succeeded = اكتمل
state-failed = فشل

# Actions
action-pause = إيقاف مؤقت
action-resume = استئناف
action-cancel = إلغاء
action-pause-all = إيقاف كل المهام مؤقتًا
action-resume-all = استئناف كل المهام
action-cancel-all = إلغاء كل المهام
action-close = إغلاق
action-reveal = إظهار في المجلد
action-add-files = إضافة ملفات
action-add-folders = إضافة مجلدات

# Phase 13d — activity feed
activity-title = النشاط
activity-clear = مسح قائمة النشاط
activity-empty = لا يوجد نشاط للملفات بعد.
activity-after-done = عند الانتهاء:
activity-keep-open = إبقاء التطبيق مفتوحًا
activity-close-app = إغلاق التطبيق
activity-shutdown = إيقاف تشغيل الكمبيوتر
activity-logoff = تسجيل الخروج
activity-sleep = وضع السكون

# Phase 14 — preflight free-space dialog
preflight-block-title = لا توجد مساحة كافية في الوجهة
preflight-warn-title = مساحة منخفضة في الوجهة
preflight-unknown-title = تعذّر تحديد المساحة الحرة
preflight-unknown-body = المصدر أكبر من أن يُحتسب حجمه بسرعة أو أن وحدة تخزين الوجهة لم تستجب. يمكنك المتابعة؛ فحارس المساحة في المحرك سيوقف النسخ بشكل سليم إذا نفدت المساحة.
preflight-required = المطلوب
preflight-free = المتاح
preflight-reserve = المحجوز
preflight-shortfall = العجز
preflight-continue = المتابعة على أي حال
preflight-pick-subset = اختر ما تريد نسخه…
collision-modal-overwrite-older = الكتابة فوق الأقدم فقط

# Phase 14e — subset picker
subset-title = اختر المصادر المراد نسخها
subset-subtitle = لا تتسع كل المحددات في الوجهة. حدّد العناصر التي تريد نسخها؛ والباقي يبقى كما هو.
subset-loading = جارٍ قياس الأحجام…
subset-too-large = أكبر من أن يُحتسب
subset-budget = المتاح
subset-remaining = المتبقي
subset-confirm = نسخ المحدد
history-rerun-hint = إعادة تشغيل هذا النسخ — يعيد فحص كل ملف في شجرة المصدر
history-clear-all = مسح الكل
history-clear-all-confirm = انقر مجددًا للتأكيد
history-clear-all-hint = حذف كل صفوف السجل. يتطلب نقرة ثانية للتأكيد.
toast-history-cleared = تم مسح السجل (أُزيل { $count } صف)

# Phase 15 — source-list ordering in the drop staging dialog
drop-dialog-sort-label = الترتيب:
sort-custom = مخصص
sort-name-asc = الاسم أ ← ي (الملفات أولًا)
sort-name-desc = الاسم ي ← أ (الملفات أولًا)
sort-size-asc = الحجم من الأصغر (الملفات أولًا)
sort-size-desc = الحجم من الأكبر (الملفات أولًا)
sort-reorder = إعادة الترتيب
sort-move-top = نقل إلى الأعلى
sort-move-up = نقل لأعلى
sort-move-down = نقل لأسفل
sort-move-bottom = نقل إلى الأسفل

# Phase 16 — sort preset names for the Activity list (shorter
# wording than the DropStagingDialog variants; the Activity list
# is files-only so the "(files first)" suffix isn't relevant).
sort-name-asc-simple = الاسم أ ← ي
sort-name-desc-simple = الاسم ي ← أ
sort-size-asc-simple = الحجم من الأصغر
sort-size-desc-simple = الحجم من الأكبر
activity-sort-locked = الترتيب معطّل أثناء تشغيل عملية نسخ. أوقفها مؤقتًا أو انتظر انتهاءها، ثم غيّر الترتيب.

# Phase 17 — collision-policy picker in the DropStagingDialog
drop-dialog-collision-label = إذا كان الملف موجودًا بالفعل:
collision-policy-keep-both = الاحتفاظ بكليهما (إعادة تسمية النسخة الجديدة إلى ‎_2‎ و‎_3‎ …)
collision-policy-skip = تخطّي النسخة الجديدة
collision-policy-overwrite = الكتابة فوق الملف الموجود
collision-policy-overwrite-if-newer = الكتابة فوقه فقط إذا كان أحدث
collision-policy-prompt = السؤال في كل مرة

# Phase 18 — progress labels on the DropStagingDialog Start button
drop-dialog-busy-checking = جارٍ التحقق من المساحة الحرة…
drop-dialog-busy-enumerating = جارٍ عدّ الملفات…
drop-dialog-busy-starting = جارٍ بدء النسخ…
toast-enumeration-deferred = شجرة المصدر كبيرة — سيتم تخطّي قائمة الملفات المسبقة؛ ستظهر الصفوف تباعًا مع تقدّم المحرك فيها.

# Context menu (per-row right-click)
menu-pause = إيقاف مؤقت
menu-resume = استئناف
menu-cancel = إلغاء
menu-remove = إزالة من قائمة الانتظار
menu-reveal-source = إظهار المصدر في المجلد
menu-reveal-destination = إظهار الوجهة في المجلد

# Header / toolbar
header-eta-label = الوقت المتبقي المقدّر
header-toolbar-label = عناصر التحكم العامة

# Footer
footer-queued = مهام نشطة
footer-total-bytes = قيد النقل
footer-errors = أخطاء
footer-history = السجل

# Empty state
empty-title = أفلِت الملفات أو المجلدات لنسخها
empty-hint = اسحب العناصر إلى النافذة. سنطلب منك تحديد وجهة، ثم نضيف مهمة لكل مصدر إلى قائمة الانتظار.
empty-region-label = قائمة المهام

# Details drawer
details-drawer-label = تفاصيل المهمة
details-source = المصدر
details-destination = الوجهة
details-state = الحالة
details-bytes = البايتات
details-files = الملفات
details-speed = السرعة
details-eta = الوقت المتبقي
details-error = الخطأ

# Drop dialog
drop-dialog-title = نقل العناصر المُفلتة
drop-dialog-subtitle = { $count } عنصر جاهز للنقل. اختر مجلد وجهة للبدء.
drop-dialog-mode = العملية
drop-dialog-copy = نسخ
drop-dialog-move = نقل
drop-dialog-pick-destination = اختيار الوجهة
drop-dialog-change-destination = تغيير الوجهة
drop-dialog-start-copy = بدء النسخ
drop-dialog-start-move = بدء النقل

# ETA placeholders
eta-calculating = جارٍ الحساب…
eta-unknown = غير معروف

# Toast messages
toast-job-done = اكتمل النقل
toast-copy-queued = أُضيف النسخ إلى قائمة الانتظار
toast-move-queued = أُضيف النقل إلى قائمة الانتظار
toast-error-resolved = تم حل الخطأ
toast-collision-resolved = تم حل التعارض
toast-elevated-unavailable = إعادة المحاولة بصلاحيات مرتفعة ستتوفر في المرحلة 17 — غير متاحة بعد
toast-clipboard-files-detected = توجد ملفات في الحافظة — اضغط اختصار اللصق لنسخها عبر Copy That
toast-clipboard-no-files = لا توجد ملفات في الحافظة للصقها
toast-error-log-exported = تم تصدير سجل الأخطاء

# Error modal (Phase 8)
error-modal-title = فشل أحد عمليات النقل
error-modal-retry = إعادة المحاولة
error-modal-retry-elevated = إعادة المحاولة بصلاحيات مرتفعة
error-modal-skip = تخطّي
error-modal-skip-all-kind = تخطّي كل الأخطاء من هذا النوع
error-modal-abort = إيقاف الكل
error-modal-path-label = المسار
error-modal-code-label = الرمز
error-drawer-pending-count = أخطاء أخرى في الانتظار
error-drawer-toggle = طيّ أو توسيع

# Error-kind labels (Phase 8). Source of truth — engine maps each
# `CopyErrorKind` to one of these keys via `localized_key()`.
err-not-found = الملف غير موجود
err-permission-denied = تم رفض الإذن
err-disk-full = قرص الوجهة ممتلئ
err-interrupted = تمت مقاطعة العملية
err-verify-failed = فشل التحقق بعد النسخ
err-path-escape = تم رفض المسار — يحتوي على أجزاء للمجلد الأصل (..) أو بايتات غير مسموح بها
err-path-invalid-encoding = تم رفض المسار — تحتوي السلسلة على ترميز UTF-8 غير صالح / أحرف استبدال
err-helper-invalid-json = استلم المساعد المميّز بيانات JSON مشوّهة؛ يتم تجاهل هذا الطلب
err-helper-grant-out-of-band = يجب معالجة GrantCapabilities عبر حلقة تشغيل المساعد، وليس عبر المعالج عديم الحالة
err-randomness-unavailable = فشل مولّد الأرقام العشوائية لنظام التشغيل؛ تعذّر إنشاء معرّف جلسة
err-sparseness-mismatch = تعذّر الحفاظ على التخطيط المتفرّق في الوجهة
err-io-other = خطأ إدخال/إخراج غير معروف

# Collision modal (Phase 8)
collision-modal-title = الملف موجود بالفعل
collision-modal-overwrite = الكتابة فوقه
collision-modal-overwrite-if-newer = الكتابة فوقه إذا كان أحدث
collision-modal-skip = تخطّي
collision-modal-keep-both = الاحتفاظ بكليهما
collision-modal-rename = إعادة التسمية…
collision-modal-apply-to-all = تطبيق على الكل
collision-modal-source = المصدر
collision-modal-destination = الوجهة
collision-modal-size = الحجم
collision-modal-modified = آخر تعديل
collision-modal-hash-check = تجزئة سريعة (SHA-256)
collision-modal-hash-computing = جارٍ الحساب…
collision-modal-hash-identical = متطابقة
collision-modal-hash-different = مختلفة
collision-modal-rename-placeholder = اسم ملف جديد
collision-modal-confirm-rename = إعادة التسمية

# Error log drawer (Phase 8)
error-log-title = سجل الأخطاء
error-log-empty = لم تُسجَّل أي أخطاء
error-log-export-csv = تصدير CSV
error-log-export-txt = تصدير نص
error-log-clear = مسح السجل
error-log-col-time = الوقت
error-log-col-job = المهمة
error-log-col-path = المسار
error-log-col-code = الرمز
error-log-col-message = الرسالة
error-log-col-resolution = الحل

# History drawer (Phase 9)
history-title = السجل
history-empty = لم تُسجَّل أي مهام بعد
history-unavailable = سجل النسخ غير متاح. تعذّر على التطبيق فتح مخزن SQLite عند بدء التشغيل.
history-filter-any = أي
history-filter-kind = النوع
history-filter-status = الحالة
history-filter-text = بحث
history-refresh = تحديث
history-export-csv = تصدير CSV
history-purge-30 = تطهير ما يزيد عن 30 يومًا
history-rerun = إعادة التشغيل
history-detail-open = التفاصيل
history-detail-title = تفاصيل المهمة
history-detail-empty = لم تُسجَّل أي عناصر
history-col-date = التاريخ
history-col-kind = النوع
history-col-src = المصدر
history-col-dst = الوجهة
history-col-files = الملفات
history-col-size = الحجم
history-col-status = الحالة
history-col-duration = المدة
history-col-error = الخطأ
toast-history-exported = تم تصدير السجل
toast-history-rerun-queued = أُضيفت إعادة التشغيل إلى قائمة الانتظار

# Totals drawer (Phase 10)
footer-totals = الإجماليات
totals-title = الإجماليات
totals-loading = جارٍ تحميل الإجماليات…
totals-card-bytes = إجمالي البايتات المنسوخة
totals-card-files = الملفات
totals-card-jobs = المهام
totals-card-avg-rate = متوسط معدّل النقل
totals-errors = أخطاء
totals-spark-title = آخر 30 يومًا
totals-kinds-title = حسب النوع
totals-saved-title = الوقت الموفَّر (تقديري)
totals-saved-note = تقديري مقارنةً بنسخ مدير ملفات أساسي لنفس حجم العمل.
totals-reset = إعادة تعيين الإحصائيات
totals-reset-confirm = سيؤدي هذا إلى حذف كل المهام والعناصر المخزّنة. هل تريد المتابعة؟
totals-reset-confirm-yes = نعم، أعد التعيين
toast-totals-reset = تمت إعادة تعيين الإحصائيات

# Phase 11a — i18n core: surface remaining user-visible strings so
# every string on a main-window screen flows through Fluent.

# Header language switcher (temporary placement — folds into
# Settings → General in Phase 12).
header-language-label = اللغة
header-language-title = تغيير اللغة

# Job-kind labels. History filter, Totals breakdown, and history
# rows all map wire-format `kind` strings to these labels.
kind-copy = نسخ
kind-move = نقل
kind-delete = حذف
kind-secure-delete = حذف آمن

# History status labels. Distinct from `state-*` because the
# history wire format uses plain `running`/`succeeded`/... rather
# than the live `JobState` enum the queue exposes.
status-running = قيد التشغيل
status-succeeded = نجح
status-failed = فشل
status-cancelled = أُلغي
# Per-item status (not job-level): `ok` / `skipped` are only
# reachable from the history detail view.
status-ok = موافق
status-skipped = تم التخطّي

# History drawer: search field placeholder + purge toast.
history-search-placeholder = /path
toast-history-purged = تم تطهير { $count } مهمة أقدم من 30 يومًا

# User-facing command-layer validation errors. The Rust side
# returns these keys when input is missing; the toast layer looks
# them up rather than showing the raw English.
err-source-required = يجب توفير مسار مصدر واحد على الأقل.
err-destination-empty = مسار الوجهة فارغ.
err-source-empty = مسار المصدر فارغ.

# Localised duration formatting for the Totals drawer and ETA
# fields. `{ $ms }`, `{ $s }`, `{ $m }`, `{ $h }` are integer
# placeables — the formatter passes pre-computed values in.
duration-lt-1s = < ثانية واحدة
duration-ms = { $ms } مل.ث
duration-seconds = { $s } ث
duration-minutes-seconds = { $m } د { $s } ث
duration-hours-minutes = { $h } س { $m } د
duration-zero = 0 ث

# Rate unit. Appended to a formatted byte size. Some languages
# render this with a leading space ("Ko/s"); keep it translatable
# even though the SI-derived "/s" is near-universal.
rate-unit-per-second = { $size }/ث

# Phase 11b — Settings modal skeleton. Phase 12 expanded this into a
# full six-tab preferences window; the `settings-phase-12-hint` key
# is retired in favour of `settings-tab-profiles` + concrete labels.
settings-title = الإعدادات
settings-tab-general = عام
settings-tab-appearance = المظهر
settings-section-language = اللغة
settings-phase-12-hint = ستصل إعدادات إضافية (السمة، إعدادات النقل الافتراضية، خوارزمية التحقق، الملفات الشخصية) في المرحلة 12.

# Phase 12 — full Settings window keys. Grouped by tab so the
# human-review pass can pick one section at a time. Every field
# label and every option label that the user will actually read
# has its own key; dynamic wire values (enum discriminants like
# "auto" / "prefer") stay untranslated on the wire.

settings-loading = جارٍ تحميل الإعدادات…
settings-tab-transfer = النقل
settings-tab-filters = عوامل التصفية
settings-tab-shell = الصدفة
settings-tab-secure-delete = الحذف الآمن
settings-tab-advanced = متقدم
settings-tab-updater = التحديثات
settings-tab-profiles = الملفات الشخصية

# General tab additions
settings-section-theme = السمة
settings-theme-auto = تلقائي
settings-theme-light = فاتح
settings-theme-dark = داكن
settings-start-with-os = التشغيل عند بدء تشغيل النظام
settings-single-instance = نسخة واحدة قيد التشغيل
settings-minimize-to-tray = التصغير إلى علبة النظام عند الإغلاق
settings-error-display-mode = نمط مطالبة الخطأ
settings-error-display-modal = نافذة منبثقة (تحجب التطبيق)
settings-error-display-drawer = درج (غير حاجب)
settings-error-display-mode-hint = النافذة المنبثقة توقف قائمة الانتظار حتى تتخذ قرارًا. الدرج يُبقي قائمة الانتظار تعمل ويتيح لك فرز الأخطاء في الزاوية.
settings-paste-shortcut = لصق الملفات عبر اختصار عام
settings-paste-shortcut-combo = مجموعة الاختصار
settings-paste-shortcut-hint = اضغط هذه المجموعة في أي مكان على نظامك للصق الملفات المنسوخة من Explorer / Finder / Files عبر Copy That. يُترجم CmdOrCtrl إلى Cmd على macOS وCtrl على Windows / Linux.
settings-clipboard-watcher = مراقبة الحافظة بحثًا عن الملفات المنسوخة
settings-clipboard-watcher-hint = إظهار إشعار عند ظهور عناوين ملفات في الحافظة، للتلميح بأنه يمكنك اللصق عبر Copy That. يفحص كل 500 مل.ث أثناء التفعيل.

# Transfer tab
settings-buffer-size = حجم المخزن المؤقت
settings-verify = التحقق بعد النسخ
settings-verify-off = إيقاف
settings-concurrency = التزامن
settings-concurrency-auto = تلقائي
settings-reflink = Reflink / المسارات السريعة
settings-reflink-prefer = تفضيل
settings-reflink-avoid = تجنّب reflink
settings-reflink-disabled = استخدام المحرك غير المتزامن دائمًا
settings-fsync-on-close = المزامنة إلى القرص عند الإغلاق (أبطأ، أكثر أمانًا)
settings-preserve-timestamps = الحفاظ على الطوابع الزمنية
settings-preserve-permissions = الحفاظ على الأذونات
settings-preserve-acls = الحفاظ على قوائم ACL (المرحلة 14)
settings-preserve-sparseness = الحفاظ على الملفات المتفرّقة
settings-preserve-sparseness-hint = نسخ المقاطع المخصّصة فقط من الملفات المتفرّقة (أقراص الأجهزة الافتراضية، ملفات قواعد البيانات) بحيث تبقى الوجهة بنفس الحجم على القرص كالمصدر.
settings-force-parallel-chunks = نسخ متوازٍ متعدد الأجزاء (RAID/المصفوفات فقط)
settings-force-parallel-chunks-hint = يقسّم كل نسخة كبيرة إلى أجزاء متزامنة. يفيد فقط مع وجهات التوزيع/RAID/الشبكة؛ ويُبطئ قرص SSD/NVMe الواحد (-25% إلى -76%). اتركه معطّلاً ما لم تكن الوجهة مصفوفة أقراص متعددة.

# Shell tab
settings-context-menu = تمكين إدخالات قائمة سياق الصدفة
settings-intercept-copy = اعتراض معالج النسخ الافتراضي (Windows)
settings-intercept-copy-hint = عند التفعيل، يمر Ctrl+C / Ctrl+V في Explorer عبر Copy That. سيصل التسجيل في المرحلة 14.
settings-notify-completion = الإشعار عند اكتمال المهمة

# Secure delete tab
settings-shred-method = طريقة التمزيق الافتراضية
settings-shred-zero = صفر (مرور واحد)
settings-shred-random = عشوائي (مرور واحد)
settings-shred-dod3 = DoD 5220.22-M (3 مرّات)
settings-shred-dod7 = DoD 5220.22-M (7 مرّات)
settings-shred-gutmann = Gutmann (35 مرة)
settings-shred-nist = NIST 800-88
settings-shred-confirm-twice = اشتراط تأكيد مزدوج قبل التمزيق

# Advanced tab
settings-log-level = مستوى السجل
settings-log-off = إيقاف
settings-telemetry = القياس عن بُعد
settings-telemetry-never = أبدًا — بلا أي اتصال خارجي عند أي مستوى تسجيل
settings-error-policy = سياسة الأخطاء الافتراضية
settings-error-policy-ask = السؤال
settings-error-policy-skip = التخطّي
settings-error-policy-retry = إعادة المحاولة مع تأخير تصاعدي
settings-error-policy-abort = الإيقاف عند أول فشل
settings-history-retention = الاحتفاظ بالسجل (أيام)
settings-history-retention-hint = 0 = الاحتفاظ إلى الأبد. أي قيمة أخرى تطهّر المهام الأقدم تلقائيًا عند بدء التشغيل.
settings-database-path = مسار قاعدة البيانات
settings-database-path-default = (افتراضي — دليل بيانات نظام التشغيل)
settings-reset-all = إعادة التعيين إلى الإعدادات الافتراضية
settings-reset-confirm = إعادة تعيين كل تفضيل إلى قيمته الافتراضية؟ لن تتأثر الملفات الشخصية.

# Profiles tab
settings-profiles-hint = احفظ الإعدادات الحالية تحت اسم؛ ثم حمّلها لاحقًا للعودة إليها دون لمس كل خيار على حدة.
settings-profile-name-placeholder = اسم الملف الشخصي
settings-profile-save = حفظ
settings-profile-import = استيراد…
settings-profile-load = تحميل
settings-profile-export = تصدير…
settings-profile-delete = حذف
settings-profile-empty = لم يُحفظ أي ملف شخصي بعد.
settings-profile-import-prompt = اسم الملف الشخصي المستورد:

# Toasts driven by Phase 12 profile actions
toast-settings-reset = تمت إعادة تعيين الإعدادات
toast-profile-saved = تم حفظ الملف الشخصي
toast-profile-loaded = تم تحميل الملف الشخصي
toast-profile-exported = تم تصدير الملف الشخصي
toast-profile-imported = تم استيراد الملف الشخصي

# Phase 14a — enumeration-time filters
settings-filters-hint = تخطّي الملفات وقت الإحصاء بحيث لا يفتحها المحرك إطلاقًا. تنطبق عناصر التضمين على الملفات فقط؛ بينما عناصر الاستبعاد تقلّم المجلدات المطابقة أيضًا.
settings-filters-enabled = تمكين عوامل التصفية لنسخ الأشجار
settings-filters-include-globs = أنماط التضمين
settings-filters-include-globs-placeholder = **/*.txt
settings-filters-include-globs-hint = نمط واحد في كل سطر. عند عدم تركها فارغة، يجب أن يطابق الملف نمط تضمين واحدًا على الأقل لينجو. يتم الدخول إلى المجلدات دائمًا.
settings-filters-exclude-globs = أنماط الاستبعاد
settings-filters-exclude-globs-placeholder = **/node_modules
settings-filters-exclude-globs-hint = نمط واحد في كل سطر. تقلّم المطابقات الشجرة الفرعية بأكملها للمجلدات؛ ويتم تخطّي الملفات المطابقة.
settings-filters-size-range = نطاق حجم الملف
settings-filters-min-size-bytes = الحجم الأدنى (بايت، فارغ = بلا حد أدنى)
settings-filters-max-size-bytes = الحجم الأقصى (بايت، فارغ = بلا حد أقصى)
settings-filters-date-range = نطاق وقت التعديل
settings-filters-min-mtime = مُعدّل في أو بعد
settings-filters-max-mtime = مُعدّل في أو قبل
settings-filters-attributes = بتات السمات
settings-filters-skip-hidden = تخطّي الملفات / المجلدات المخفية
settings-filters-skip-system = تخطّي ملفات النظام (Windows فقط)
settings-filters-skip-readonly = تخطّي الملفات للقراءة فقط

# Phase 15 — auto-update
settings-updater-hint = يتحقق Copy That من التحديثات الموقّعة مرة واحدة في اليوم على الأكثر. تُثبَّت التحديثات عند إنهاء التطبيق التالي.
settings-updater-auto-check = التحقق من التحديثات عند التشغيل
settings-updater-channel = قناة الإصدار
settings-updater-channel-stable = مستقر
settings-updater-channel-beta = تجريبي (ما قبل الإصدار)
settings-updater-last-check = آخر تحقق
settings-updater-last-never = أبدًا
settings-updater-check-now = التحقق من التحديثات الآن
settings-updater-checking = جارٍ التحقق…
settings-updater-available = يتوفر تحديث
settings-updater-up-to-date = أنت تشغّل أحدث إصدار.
settings-updater-dismiss = تخطّي هذا الإصدار
settings-updater-dismissed = تم التخطّي
toast-update-available = يتوفر إصدار أحدث
toast-update-up-to-date = أنت بالفعل على أحدث إصدار

# Phase 19a — disk-backed file enumeration (TeraCopy-compatible scan DB)
scan-progress-title = جارٍ الفحص…
scan-progress-stats = { $files } ملف · { $bytes } حتى الآن
scan-pause-button = إيقاف الفحص مؤقتًا
scan-resume-button = استئناف الفحص
scan-cancel-button = إلغاء الفحص
scan-cancel-confirm = إلغاء الفحص وتجاهل التقدّم؟
scan-db-header = قاعدة بيانات الفحص
scan-db-hint = قاعدة بيانات فحص على القرص للمهام التي تضم ملايين الملفات.
advanced-scan-hash-during = حساب المجاميع الاختبارية أثناء الفحص
advanced-scan-db-path = موقع قاعدة بيانات الفحص
advanced-scan-retention-days = حذف عمليات الفحص المكتملة تلقائيًا بعد (أيام)
advanced-scan-max-keep = الحد الأقصى لعدد قواعد بيانات الفحص المحتفظ بها

# Phase 19b — filesystem-snapshot source for locked files.
settings-on-locked = عندما يكون الملف مقفلًا
settings-on-locked-ask = السؤال في المرة الأولى
settings-on-locked-retry = إعادة المحاولة لفترة وجيزة، ثم إظهار الخطأ
settings-on-locked-skip = تخطّي الملف المقفل
settings-on-locked-snapshot = استخدام لقطة لنظام الملفات
settings-on-locked-hint = القضاء على أخطاء "الملف قيد الاستخدام من قبل عملية أخرى". يلتقط Copy That لقطة لوحدة تخزين المصدر (VSS على Windows، وZFS/Btrfs على Linux، وAPFS على macOS) ويقرأ من نسخة اللقطة.
snapshot-prompt-title = هذا الملف قيد الاستخدام من قبل عملية أخرى
snapshot-prompt-body = برنامج آخر يبقي { $path } مفتوحًا للكتابة الحصرية. اختر كيف ينبغي لـ Copy That التعامل مع هذا الملف والملفات المشابهة على نفس وحدة التخزين.
snapshot-source-active = 📷 جارٍ القراءة من لقطة { $kind } لـ { $volume }
snapshot-create-failed = تعذّر إنشاء لقطة لوحدة تخزين المصدر
snapshot-vss-needs-elevation = القراءة من لقطة VSS تتطلب إذن المسؤول. سيطلب منك Copy That السماح بذلك.
snapshot-cleanup-failed = أبلغ مساعد اللقطات عن فشل في التنظيف — قد تبقى نسخة ظل متبقية على وحدة التخزين.

# Phase 20 — durable resume journal.
resume-prompt-title = استئناف عمليات النقل السابقة؟
resume-prompt-body = اكتشف Copy That { $count } عملية نقل غير مكتملة من جلسة سابقة. اختر ما تريد فعله بكل منها.
resume-prompt-resume = استئناف
resume-prompt-resume-all = استئناف الكل
resume-discard-one = عدم الاستئناف
resume-discard-all = تجاهل الكل
resume-aborted-hash-mismatch = أول { $offset } بايت في الوجهة لا تطابق المصدر — إعادة البدء من البداية.
settings-auto-resume = استئناف المهام المُقاطَعة تلقائيًا دون مطالبة
settings-auto-resume-hint = تخطّي مطالبة الاستئناف عند بدء التشغيل وإعادة إضافة كل مهمة غير مكتملة بصمت إلى قائمة الانتظار. مُعطّل افتراضيًا.

# Phase 21 — bandwidth shaping (GCRA token bucket + schedule + auto-throttle).
settings-tab-network = الشبكة
settings-network-hint = حدّ معدّل النقل لإبقاء بقية الشبكة قابلة للاستخدام. طبّقه على نطاق عام، أو اتبع جدولًا يوميًا، أو تفاعل تلقائيًا مع اتصالات Wi-Fi محدودة الاستخدام / البطارية / الخلوية.
settings-network-mode = حدّ النطاق الترددي
settings-network-mode-off = إيقاف (بلا حد)
settings-network-mode-fixed = قيمة ثابتة
settings-network-mode-schedule = استخدام جدول
settings-network-cap-mbps = الحد الأقصى (م.ب/ث)
settings-network-schedule = الجدول (تنسيق rclone)
settings-network-schedule-hint = حدود HH:MM,rate مفصولة بمسافات بيضاء بالإضافة إلى قواعد يومية اختيارية Mon-Fri,rate. المعدّلات: 512k و10M و2G وoff وunlimited. مثال: 08:00,512k 18:00,10M Sat-Sun,unlimited.
settings-network-auto-header = التقييد التلقائي
settings-network-auto-metered = على Wi-Fi محدود الاستخدام
settings-network-auto-battery = على البطارية
settings-network-auto-cellular = على الاتصال الخلوي
settings-network-auto-unchanged = عدم التجاوز
settings-network-auto-pause = إيقاف عمليات النقل مؤقتًا
settings-network-auto-cap = التقييد إلى قيمة ثابتة
shape-badge-paused = متوقف مؤقتًا
shape-badge-tooltip = حدّ النطاق الترددي نشط — انقر لفتح الإعدادات ← الشبكة
shape-badge-source-schedule = مجدول
shape-badge-source-metered = محدود الاستخدام
shape-badge-source-battery = على البطارية
shape-badge-source-cellular = خلوي
shape-badge-source-settings = نشط
shape-error-schedule-invalid = تنسيق الجدول غير صالح: { $message }

# Phase 22 — aggregate conflict dialog v2 (thumbnails, per-pattern
# rules, and reusable conflict profiles). Every key below is user-
# visible text in the `ConflictBatchModal.svelte` component.
conflict-batch-title = { $count } تعارضات ملفات في { $jobname }
conflict-batch-state-pending = معلّق
conflict-batch-state-resolved = تم الحل
conflict-batch-action-overwrite = الكتابة فوقه
conflict-batch-action-skip = تخطّي
conflict-batch-action-keep-both = الاحتفاظ بكليهما
conflict-batch-action-newer-wins = الأحدث يفوز
conflict-batch-action-larger-wins = الأكبر يفوز
conflict-batch-bulk-apply-selected = تطبيق على المحدد
conflict-batch-bulk-apply-extension = تطبيق على كل ما يحمل هذا الامتداد
conflict-batch-bulk-apply-glob = تطبيق على النمط المطابق…
conflict-batch-bulk-apply-remaining = تطبيق على كل المتبقي
conflict-batch-bulk-glob-placeholder = مثل **/*.tmp
conflict-batch-save-profile = حفظ هذه القواعد كملف شخصي…
conflict-batch-profile-placeholder = اسم الملف الشخصي
conflict-batch-matched-rule = عبر القاعدة '{ $rule }' ← { $action }
conflict-batch-empty = تم حل كل التعارضات
conflict-batch-source-vs-destination = المصدر مقابل الوجهة
conflict-batch-source-label = المصدر
conflict-batch-destination-label = الوجهة
conflict-batch-size-label = الحجم
conflict-batch-modified-label = آخر تعديل
conflict-batch-close = إغلاق
conflict-batch-profile-saved = تم حفظ ملف التعارضات الشخصي

# Phase 23 — sparse-file preservation. The toast fires once per
# destination volume when the filesystem can't preserve holes; the
# warning line is surfaced in the job detail drawer so the user knows
# the dst is larger on disk than the source was.
sparse-not-supported-title = الوجهة تملأ الملفات المتفرّقة
sparse-not-supported-body = لا يدعم { $dst_fs } الملفات المتفرّقة. كُتبت الفجوات في المصدر على هيئة أصفار، لذا أصبحت الوجهة أكبر على القرص.
sparse-warning-densified = تم الحفاظ على التخطيط المتفرّق: نُسخت المقاطع المخصّصة فقط.
sparse-warning-mismatch = عدم تطابق في التخطيط المتفرّق — قد تكون الوجهة أكبر من المتوقع.

# Phase 24 — security-metadata preservation. The Mark-of-the-Web
# (Zone.Identifier ADS) toggle is security-sensitive: turning it off
# lets a downloaded executable shed its SmartScreen / Office Protected
# View flag on copy, which is why the tooltip carries an explicit
# warning. AppleDouble fallback emits `._<filename>` sidecars on
# destination filesystems that can't hold the foreign metadata.
settings-preserve-security-metadata = الحفاظ على بيانات الأمان الوصفية
settings-preserve-security-metadata-hint = التقاط تدفقات البيانات الوصفية الخارجية وإعادة تطبيقها (NTFS ADS / xattrs / POSIX ACLs / سياقات SELinux / إمكانات ملفات Linux / تشعّبات موارد macOS) في كل نسخة.
settings-preserve-motw = الحفاظ على Mark-of-the-Web (علامة التنزيل من الإنترنت)
settings-preserve-motw-hint = أساسي للأمان. يستخدم SmartScreen وOffice Protected View هذا التدفق للتحذير من الملفات المنزَّلة من الإنترنت. تعطيله يتيح لملف تنفيذي منزَّل التخلص من علامة مصدره عند النسخ وتجاوز ضمانات نظام التشغيل.
settings-preserve-posix-acls = الحفاظ على قوائم POSIX ACL والسمات الموسّعة
settings-preserve-posix-acls-hint = نقل سمات user.* / system.* / trusted.* الموسّعة وقوائم التحكم في الوصول POSIX عبر النسخ.
settings-preserve-selinux = الحفاظ على سياقات SELinux
settings-preserve-selinux-hint = نقل علامة security.selinux عبر النسخ بحيث تستطيع الخدمات العاملة تحت سياسات MAC الوصول إلى الملف.
settings-preserve-resource-forks = الحفاظ على تشعّبات موارد macOS ومعلومات Finder
settings-preserve-resource-forks-hint = نقل تشعّب الموارد القديم ومعلومات FinderInfo (علامات الألوان، بيانات Carbon الوصفية) عبر النسخ.
settings-appledouble-fallback = استخدام ملف AppleDouble الجانبي على أنظمة الملفات غير المتوافقة
meta-translated-to-appledouble = البيانات الوصفية الخارجية مخزّنة في ملف AppleDouble الجانبي (._{ $ext })

# Phase 25 — two-way sync with vector-clock conflict detection.
# The drawer lists configured sync pairs; each pair runs an
# independent reconciliation round against a per-pair `.copythat-sync.db`
# state store. Concurrent edits from a common ancestor surface as
# conflicts rather than silent overwrites; the losing side's content
# is preserved as `name.sync-conflict-YYYYMMDD-HHMMSS-<host>.ext`.
footer-sync = المزامنة
sync-drawer-title = مزامنة ثنائية الاتجاه
sync-drawer-hint = أبقِ مجلدين متزامنين دون كتابة فوق صامتة. تظهر التعديلات المتزامنة كتعارضات يمكنك حلها.
sync-add-pair = إضافة زوج
sync-add-cancel = إلغاء
sync-refresh = تحديث
sync-add-save = حفظ الزوج
sync-add-saving = جارٍ الحفظ…
sync-add-missing-fields = التسمية والمسار الأيسر والمسار الأيمن كلها مطلوبة.
sync-remove-confirm = إزالة زوج المزامنة هذا؟ يتم الحفاظ على قاعدة بيانات الحالة؛ ولا تُمس المجلدات.
sync-field-label = التسمية
sync-field-label-placeholder = مثل المستندات ↔ NAS
sync-field-left = المجلد الأيسر
sync-field-left-placeholder = اختر أو الصق مسارًا مطلقًا
sync-field-right = المجلد الأيمن
sync-field-right-placeholder = اختر أو الصق مسارًا مطلقًا
sync-field-mode = الوضع
sync-mode-two-way = ثنائي الاتجاه
sync-mode-mirror-left-to-right = نسخ مطابق (الأيسر ← الأيمن)
sync-mode-mirror-right-to-left = نسخ مطابق (الأيمن ← الأيسر)
sync-mode-contribute-left-to-right = إسهام (الأيسر ← الأيمن، بلا حذف)
sync-no-pairs = لم يتم تكوين أي أزواج مزامنة بعد. انقر "إضافة زوج" للبدء.
sync-loading = جارٍ تحميل الأزواج المكوّنة…
sync-never-run = لم يُشغَّل قط
sync-running = قيد التشغيل
sync-run-now = التشغيل الآن
sync-cancel = إلغاء
sync-remove-pair = إزالة
sync-view-conflicts = عرض التعارضات ({ $count })
sync-conflicts-heading = التعارضات
sync-no-conflicts = لا توجد تعارضات من التشغيل الأخير.
sync-winner = الفائز
sync-side-left-to-right = الأيسر
sync-side-right-to-left = الأيمن
sync-conflict-kind-concurrent-write = تعديل متزامن
sync-conflict-kind-delete-edit = حذف ↔ تعديل
sync-conflict-kind-add-add = أُضيف من كلا الجانبين
sync-conflict-kind-corrupt-equal = تباعد المحتوى دون كتابة جديدة
sync-resolve-keep-left = الاحتفاظ بالأيسر
sync-resolve-keep-right = الاحتفاظ بالأيمن
sync-resolve-keep-both = الاحتفاظ بكليهما
sync-resolve-three-way = الحل عبر دمج ثلاثي الاتجاه
sync-resolve-phase-53-tooltip = الدمج الثلاثي التفاعلي للملفات غير النصية سيصل في المرحلة 53.
sync-error-prefix = خطأ في المزامنة

# Phase 26 — real-time mirror watcher. "Live mirror" starts a
# filesystem watcher on the pair's left side; every debounced event
# triggers a re-sync. The watcher filters vim swap files / Office
# lock files / atomic-save temp names so one logical save becomes
# exactly one sync round.
live-mirror-start = بدء النسخ المطابق المباشر
live-mirror-stop = إيقاف النسخ المطابق المباشر
live-mirror-watching = قيد المراقبة
live-mirror-toggle-hint = إعادة المزامنة تلقائيًا عند كل تغيير مكتشف في نظام الملفات. خيط خلفي واحد لكل زوج نشط.
watch-event-prefix = تغيير في الملف
watch-overflow-recovered = فاض مخزن المراقب المؤقت؛ يُعاد الإحصاء للتعافي

# Phase 27 — content-defined chunk store. Enables delta-resume (a
# retry only re-writes chunks that actually changed) and same-job
# dedup (files sharing content blocks store those blocks once). The
# store is disk-backed under `<data-dir>/chunks/` by default and is
# the foundation for the Phase 49–51 moonshot repository phases.
chunk-store-section = مخزن الأجزاء
chunk-store-enable = تمكين مخزن الأجزاء (استئناف تفاضلي وإزالة تكرار)
chunk-store-enable-hint = يقسّم كل ملف منسوخ حسب المحتوى (FastCDC) ويخزّن الأجزاء بعنونة المحتوى. تعيد المحاولات كتابة الأجزاء المتغيّرة فقط؛ والملفات ذات المحتوى المشترك تُزال تكراراتها تلقائيًا.
chunk-store-location = موقع مخزن الأجزاء
chunk-store-max-size = الحجم الأقصى لمخزن الأجزاء
chunk-store-prune = تقليم الأجزاء الأقدم من (أيام)
chunk-store-savings = تم توفير { $gib } غ.ب عبر إزالة تكرار الأجزاء
chunk-store-disk-usage = استخدام { $size } عبر { $chunks } جزء

# Phase 28 — tray-resident Drop Stack. The stack is a persistent list
# of paths gathered from multiple sources (Explorer drag, main-window
# context menu, CLI, drag onto the Drop Stack window) that the user
# can dispatch to a destination in one go.
dropstack-window-title = Drop Stack
dropstack-tray-open = Drop Stack
dropstack-empty-title = Drop Stack فارغ
dropstack-empty-hint = اسحب الملفات إلى هنا من Explorer أو انقر بزر الفأرة الأيمن على صف مهمة لإضافته.
dropstack-add-to-stack = إضافة إلى Drop Stack
dropstack-copy-all-to = نسخ الكل إلى…
dropstack-move-all-to = نقل الكل إلى…
dropstack-clear = مسح المكدّس
dropstack-remove-row = إزالة من المكدّس
dropstack-path-missing-toast = أُفلت { $path } — لم يعد الملف موجودًا.
dropstack-always-on-top = إبقاء Drop Stack في المقدمة دائمًا
dropstack-show-tray-icon = إظهار أيقونة Copy That في علبة النظام
dropstack-open-on-start = فتح Drop Stack تلقائيًا عند بدء التطبيق
dropstack-count = { $count } مسار

# Phase 29 — spring-loaded folders + native DnD polish. The Settings
# → General tab carries the knobs; DropTarget / DestinationPicker
# Svelte components use the dropzone-* keys at runtime.
settings-dnd-heading = السحب والإفلات
settings-dnd-spring-load = فتح المجلدات تلقائيًا أثناء السحب
settings-dnd-spring-delay = تأخير الفتح التلقائي (مل.ث)
settings-dnd-thumbnails = إظهار صور مصغّرة أثناء السحب
settings-dnd-invalid-highlight = إبراز أهداف الإفلات غير الصالحة
dropzone-invalid-title = ليس هدف إفلات صالحًا
dropzone-invalid-readonly = الوجهة للقراءة فقط
dropzone-picker-title = اختر وجهة
dropzone-picker-up = للأعلى
dropzone-picker-path = المسار الحالي
dropzone-picker-root = الجذور
dropzone-picker-use-this = استخدام هذا المجلد
dropzone-picker-empty = لا توجد مجلدات فرعية
dropzone-picker-cancel = إلغاء

# Phase 30 — cross-platform path translation. Settings → Transfer
# exposes these under a "Cross-platform compatibility" subsection.
translate-heading = التوافق بين المنصات
translate-unicode-label = توحيد ترميز Unicode
translate-unicode-auto = الكشف التلقائي عن الوجهة
translate-unicode-windows = NFC (Windows / Linux)
translate-unicode-macos = الإبقاء كما هو (macOS / APFS)
translate-line-endings-label = ترجمة نهايات الأسطر للملفات النصية
translate-line-endings-allowlist = امتدادات الملفات النصية
reserved-name-label = التعامل مع الأسماء المحجوزة في Windows
reserved-name-suffix = إلحاق "_" (CON.txt ← CON_.txt)
reserved-name-reject = الرفض والتحذير
long-path-label = استخدام بادئة المسار الطويل في Windows (\\?\) عند تجاوز 260 حرفًا
long-path-hint = بعض مشاركات الشبكة والأدوات القديمة لا تحترم مساحة الأسماء \\?\.

# Phase 31 — power-aware copying. Settings → Power & State tab + the
# header badge that renders "⏸ Paused — Zoom call detected" when the
# runner's power subscriber has paused or capped due to a policy match.
power-heading = الطاقة والحالة
power-enabled = تمكين القواعد المراعية للطاقة
power-battery-label = على البطارية
power-metered-label = على Wi-Fi محدود الاستخدام
power-cellular-label = على الاتصال الخلوي
power-presentation-label = أثناء العرض التقديمي (Zoom / Teams / Keynote)
power-fullscreen-label = عندما يكون تطبيق ما في وضع ملء الشاشة
power-thermal-label = عند تقييد وحدة المعالجة بسبب الحرارة
power-rule-continue = المتابعة بكامل السرعة
power-rule-pause = إيقاف كل المهام مؤقتًا
power-rule-cap = تقييد النطاق الترددي
power-rule-cap-percent = التقييد إلى نسبة من المعدّل الحالي
power-reason-on-battery = على البطارية
power-reason-metered-network = شبكة محدودة الاستخدام
power-reason-cellular-network = شبكة خلوية
power-reason-presenting = وضع العرض التقديمي
power-reason-fullscreen = تطبيق بملء الشاشة
power-reason-thermal-throttling = وحدة المعالجة تُقيَّد حراريًا

# Phase 32 — cloud backend matrix via OpenDAL. Settings → Remotes
# tab + the Add-backend wizard that writes one entry per remote into
# the keychain-backed credential store.
remote-heading = الواجهات الخلفية البعيدة
remote-add = إضافة واجهة خلفية
remote-list-empty = لم يتم تكوين أي واجهات خلفية بعيدة
remote-test = اختبار الاتصال
remote-test-success = نجح الاتصال
remote-test-failed = فشل الاتصال
remote-remove = إزالة الواجهة الخلفية
remote-name-label = الاسم المعروض
remote-kind-label = نوع الواجهة الخلفية
remote-save = حفظ الواجهة الخلفية
remote-cancel = إلغاء
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
backend-local-fs = نظام الملفات المحلي
cloud-config-bucket = الحاوية
cloud-config-region = المنطقة
cloud-config-endpoint = عنوان URL لنقطة النهاية
cloud-config-root = مسار الجذر
cloud-error-invalid-config = تكوين الواجهة الخلفية غير صالح
cloud-error-network = خطأ شبكة أثناء الاتصال بالواجهة الخلفية
cloud-error-not-found = لم يُعثر على الكائن في المسار المطلوب
cloud-error-permission = رفضت الواجهة الخلفية البعيدة الإذن
cloud-error-keychain = فشل الوصول إلى سلسلة مفاتيح نظام التشغيل
settings-tab-remotes = الأجهزة البعيدة
settings-tab-mobile = الجوال

# Phase 33 — mount Copy That's chunk store + history archive as a
# read-only filesystem (FUSE / WinFsp). Surfaced by the History tab
# context menu's "Mount snapshot" action + the Settings → Advanced
# "Mount latest on launch" toggle.
mount-heading = تركيب اللقطة
mount-action-mount = تركيب اللقطة
mount-action-unmount = إلغاء التركيب
mount-status-mounted = مُركَّبة في { $path }
mount-error-unsafe-mountpoint = مسار نقطة التركيب غير آمن
mount-error-mountpoint-not-empty = يجب أن تكون نقطة التركيب دليلًا فارغًا
mount-error-backend-unavailable = الواجهة الخلفية للتركيب غير متاحة على هذا النظام
mount-error-archive-read = فشلت قراءة الأرشيف
mount-picker-title = اختر دليل نقطة التركيب
mount-toast-mounted = تم تركيب اللقطة في { $path }
mount-toast-unmounted = تم إلغاء تركيب اللقطة
mount-toast-failed = فشل التركيب: { $reason }
settings-mount-heading = تركيب اللقطات
settings-mount-hint = إظهار أرشيف السجل كنظام ملفات للقراءة فقط. تربط المرحلة 33b تدفق المشغّل؛ وتصل الواجهات الخلفية FUSE/WinFsp على مستوى النواة في المرحلة 33c.
settings-mount-on-launch = تركيب أحدث لقطة عند التشغيل
settings-mount-on-launch-path = مسار نقطة التركيب
settings-mount-on-launch-path-placeholder = مثل C:\Mounts\copythat

# Phase 34 — enterprise-grade audit log export + WORM (write-once-read-
# many) tamper-resistance. Settings → Advanced → Audit log collects the
# format + destination + rotation + WORM toggle; runtime records job and
# file events into the tamper-evident chain-hashed sink.
settings-audit-heading = سجل التدقيق
settings-audit-hint = سجل لكل مهمة وحدث ملف للإضافة فقط ويكشف العبث. تشمل التنسيقات CSV وJSON-lines وRFC 5424 Syslog وArcSight CEF وQRadar LEEF.
settings-audit-enable = تمكين تسجيل التدقيق
settings-audit-format = تنسيق السجل
settings-audit-format-json-lines = JSON lines (الافتراضي الموصى به)
settings-audit-format-csv = CSV (مناسب لجداول البيانات)
settings-audit-format-syslog = Syslog (RFC 5424)
settings-audit-format-cef = CEF (ArcSight)
settings-audit-format-leef = LEEF 2.0 (IBM QRadar)
settings-audit-file-path = مسار ملف السجل
settings-audit-file-path-placeholder = مثل C:\ProgramData\CopyThat\audit.log
settings-audit-max-size = التدوير بعد (بايت، 0 = أبدًا)
settings-audit-worm = تمكين وضع WORM (الكتابة مرة والقراءة عدة مرات)
settings-audit-worm-hint = يطبّق علامة الإضافة فقط الخاصة بالمنصة (Linux chattr +a، وmacOS chflags uappnd، وسمة القراءة فقط في Windows) بعد كل إنشاء أو تدوير. حتى المسؤول يجب أن يمسح العلامة صراحةً لاقتطاع السجل.
settings-audit-test-write = اختبار الكتابة
settings-audit-verify-chain = التحقق من السلسلة
toast-audit-test-write-ok = نجح اختبار الكتابة لسجل التدقيق
toast-audit-verify-ok = تم التحقق من سلامة سلسلة التدقيق
toast-audit-verify-failed = أبلغ التحقق من سلسلة التدقيق عن عدم تطابق

# Phase 35 — destination encryption via age (X25519 / passphrase /
# SSH) + on-the-fly zstd compression with a per-extension deny
# heuristic. Surfaced through Settings → Transfer → Encryption +
# Compression; engine short-circuits to the crypt pipeline when
# either stage is active.
settings-crypt-heading = التشفير والضغط
settings-crypt-hint = تحويل محتوى الملفات قبل وصولها إلى الوجهة. يستخدم التشفير تنسيق age؛ ويستخدم الضغط zstd ويمكنه تخطّي الوسائط المضغوطة مسبقًا حسب الامتداد.
settings-crypt-encryption-mode = التشفير
settings-crypt-encryption-off = إيقاف
settings-crypt-encryption-passphrase = عبارة مرور (مطالبة عند بدء النسخ)
settings-crypt-encryption-recipients = مفاتيح المستلمين من ملف
settings-crypt-encryption-hint = تُحفظ عبارات المرور في الذاكرة فقط طوال مدة النسخ. تسرد ملفات المستلمين مفتاحًا عامًا واحدًا من نوع age1… أو ssh- في كل سطر.
settings-crypt-recipients-file = مسار ملف المستلمين
settings-crypt-recipients-file-placeholder = مثل C:\Users\me\recipients.txt
settings-crypt-compression-mode = الضغط
settings-crypt-compression-off = إيقاف
settings-crypt-compression-always = دائمًا
settings-crypt-compression-smart = ذكي (تخطّي الوسائط المضغوطة مسبقًا)
settings-crypt-compression-hint = يتخطّى الوضع الذكي صيغ jpg وmp4 وzip و7z والصيغ المشابهة التي لا تستفيد من zstd. ويضغط الوضع الدائم كل ملف بالمستوى المختار.
settings-crypt-compression-level = مستوى zstd (1-22)
settings-crypt-compression-level-hint = الأرقام الأقل أسرع؛ والأرقام الأعلى تضغط بقوة أكبر. المستوى 3 يطابق الإعداد الافتراضي لواجهة zstd.
compress-footer-savings = 💾 { $original } ← { $compressed } (تم توفير { $percent }%)
compress-savings-toast = تم الضغط بنسبة { $percent }% (تم توفير { $bytes })
crypt-toast-recipients-loaded = تم تحميل { $count } من مستلمي التشفير
crypt-toast-recipients-error = فشل تحميل المستلمين: { $reason }
crypt-toast-passphrase-required = يحتاج التشفير إلى عبارة مرور قبل بدء النسخ
crypt-toast-passphrase-set = تم التقاط عبارة مرور التشفير
crypt-footer-encrypted-badge = 🔒 مشفّر (age)
crypt-footer-compressed-badge = 📦 مضغوط (zstd)

# Phase 36 — copythat CLI surface. Documented exit codes + the
# user-facing strings for plan/apply/verify/config. Help text stays
# in English (engineering accessibility) but error / info / status
# strings are localized so a localized desktop installation surfaces
# CLI errors in the same language as the GUI.
cli-help-tagline = Copy That CLI — نسخ ومزامنة وتحقق وتدقيق دقيق على مستوى البايت للملفات لخطوط أنابيب CI/CD.
cli-help-exit-codes = رموز الخروج: 0 نجاح، 1 خطأ، 2 معلّق، 3 تعارض، 4 فشل تحقق، 5 شبكة، 6 إذن، 7 قرص ممتلئ، 8 إلغاء، 9 تكوين.
cli-error-bad-args = يتطلب copy/move مصدرًا واحدًا على الأقل ووجهة
cli-error-unknown-algo = خوارزمية تحقق غير معروفة: { $algo }
cli-error-missing-spec = ‏--spec مطلوب لـ plan/apply
cli-error-spec-parse = فشل تحليل jobspec ‏{ $path }: { $reason }
cli-error-spec-empty-sources = قائمة مصادر jobspec فارغة
cli-info-shape-recorded = تم تسجيل شكل النطاق الترددي "{ $rate }"؛ يُفرض عبر copythat-shape
cli-info-stub-deferred = { $command } مُجدول لربط متابعة المرحلة 36
cli-plan-summary = الخطة: { $actions } إجراء، { $bytes } بايت؛ { $already_done } جاهز بالفعل
cli-plan-pending = تُبلغ الخطة عن إجراءات معلّقة؛ أعد التشغيل بـ `apply` للتنفيذ
cli-plan-already-done = تُبلغ الخطة بعدم وجود ما يلزم فعله (متكافئ)
cli-apply-success = اكتمل apply دون أخطاء
cli-apply-failed = اكتمل apply مع خطأ واحد أو أكثر
cli-verify-ok = نجح التحقق: { $algo } { $digest }
cli-verify-failed = فشل التحقق لـ { $path } ({ $algo })
cli-config-set = تعيين { $key } = { $value }
cli-config-reset = إعادة تعيين { $key } إلى الافتراضي
cli-config-unknown-key = مفتاح تكوين غير معروف: { $key }
cli-completions-emitted = تمت طباعة إكمالات الصدفة لـ { $shell } إلى stdout

# Phase 37 — desktop-side mobile companion. Settings → Mobile panel
# strings + the SAS-confirmation modal + push-notification toasts.
# The actual mobile UI lives in the Phase 37 follow-up Tauri Mobile
# target; the phone displays its own localized SAS prompt.
settings-mobile-heading = رفيق الجوال
settings-mobile-hint = اقترِن بهاتف iPhone أو Android لتصفّح السجل، وبدء الملفات الشخصية المحفوظة وjobspecs المرحلة 36، وتلقّي إشعارات الاكتمال.
settings-mobile-pair-toggle = السماح بالاقترانات الجديدة
settings-mobile-pair-active = خادم الاقتران نشط — امسح رمز QR بتطبيق Copy That للجوال
settings-mobile-pair-button = بدء الاقتران
settings-mobile-revoke-button = إلغاء
settings-mobile-no-pairings = لا توجد أجهزة مقترنة بعد
settings-mobile-pair-port = منفذ الربط (0 = اختيار منفذ حر)
pair-sas-prompt = يجب أن تُظهر كلتا الشاشتين نفس الرموز التعبيرية الأربعة. اضغط مطابقة إذا تطابقت.
pair-sas-confirm = مطابقة
pair-sas-reject = عدم تطابق — إلغاء
pair-toast-success = تم الاقتران مع { $device }
pair-toast-failed = فشل الاقتران: { $reason }
push-toast-sent = تم إرسال إشعار دفع إلى { $device }
push-toast-failed = فشل إرسال الإشعار إلى { $device }: { $reason }

# Phase 38 — aggregate destination dedup + reflink fallback ladder.
# Settings → Transfer → Dedup panel + per-job-row badges (⚡
# Reflinked / 🔗 Hardlinked / 🧩 Chunk-shared / 📋 Copied) +
# pre-pass dedup-scan modal.
settings-dedup-heading = إزالة تكرار الوجهة
settings-dedup-hint = عندما يتشارك المصدر والوجهة وحدة تخزين، يستطيع Copy That استنساخ الملفات على مستوى نظام الملفات بدلًا من نسخ البايتات. الـ reflink فوري وآمن؛ والـ hardlink أسرع لكن الاسمين يتشاركان الحالة.
settings-dedup-mode-auto = سلّم تلقائي (reflink ← hardlink ← chunk ← نسخ)
settings-dedup-mode-reflink-only = reflink فقط
settings-dedup-mode-hardlink-aggressive = حازم (reflink + hardlink حتى على الملفات القابلة للكتابة)
settings-dedup-mode-off = معطّل (نسخ بايت دائمًا)
settings-dedup-hardlink-policy = سياسة hardlink
settings-dedup-prescan = فحص مسبق لشجرة الوجهة بحثًا عن محتوى مكرّر
dedup-badge-reflinked = ⚡ مرتبط بـ reflink
dedup-badge-hardlinked = 🔗 مرتبط بـ hardlink
dedup-badge-chunk-shared = 🧩 جزء مشترك
dedup-badge-copied = 📋 منسوخ
phase42-paranoid-verify-label = تحقق مفرط
phase42-paranoid-verify-hint = يُسقط الصفحات المخزّنة مؤقتًا للوجهة ويعيد القراءة من القرص لكشف أكاذيب ذاكرة الكتابة والتلف الصامت. أبطأ بنحو 50% من التحقق الافتراضي؛ معطّل افتراضيًا.
phase42-sharing-violation-retries-label = محاولات إعادة المحاولة على ملفات المصدر المقفلة
phase42-sharing-violation-retries-hint = عدد مرات إعادة المحاولة عندما تبقي عملية أخرى ملف المصدر مفتوحًا بقفل حصري. يتضاعف التأخير في كل محاولة (50 مل.ث / 100 مل.ث / 200 مل.ث افتراضيًا). الافتراضي 3، مطابقًا لـ Robocopy /R:3.
phase42-cloud-placeholder-warning = { $name } ملف OneDrive سحابي فقط. سيؤدي نسخه إلى بدء تنزيل — يصل إلى { $size } عبر اتصال شبكتك.
phase42-defender-exclusion-hint = لأقصى معدّل نقل، أضِف مجلد الوجهة إلى استثناءات Microsoft Defender قبل عمليات النقل الكبيرة. راجع docs/PERFORMANCE_TUNING.md.

# Phase 39 — Browser-accessible recovery UI. Settings → Advanced
# exposes these strings; the recovery server itself renders askama
# templates that always read English — these eight keys are the
# Settings prose the user actually reads in their preferred locale.
settings-recovery-heading = واجهة الاسترداد عبر الويب
settings-recovery-enable = تمكين واجهة الاسترداد عبر الويب
settings-recovery-bind-address = عنوان الربط
settings-recovery-port = المنفذ (0 = اختيار منفذ حر)
settings-recovery-show-url = إظهار عنوان URL والرمز المميّز
settings-recovery-rotate-token = تدوير الرمز المميّز
settings-recovery-allow-non-loopback = السماح بالربط على غير loopback
settings-recovery-non-loopback-warning = تحذير: تمكين الربط على غير loopback يعرّض واجهة الاسترداد لشبكتك المحلية. أي شخص يعرف الرمز المميّز يمكنه تصفّح سجل ملفاتك وتنزيل الملفات. ضعها خلف TLS أو وكيل عكسي إذا كانت الشبكة المحلية غير موثوقة.

# Phase 40 — SMB compression negotiation + cloud-VM offload helper.
# 6 SMB keys (header badge + Settings prose) + 6 cloud-offload keys
# (Remotes tab wizard for cross-cloud copy templates).
smb-compress-badge = 🗜 ضغط SMB: { $algo }
smb-compress-badge-tooltip = حركة الشبكة إلى هذه الوجهة يتم ضغطها أثناء النقل (SMB 3.1.1).
smb-compress-toast-saved = تم توفير { $bytes } عبر الشبكة
smb-compress-algo-unknown = خوارزمية غير معروفة
settings-smb-compress-heading = ضغط شبكة SMB
settings-smb-compress-hint = التفاوض تلقائيًا على ضغط حركة SMB 3.1.1 على وجهات UNC. مكسب مجاني على الوصلات البطيئة؛ ويُتجاهل على الوجهات المحلية.
cloud-offload-heading = مساعد التفريغ عبر جهاز افتراضي سحابي
cloud-offload-hint = عند النسخ مباشرةً بين سحابتين، أنشئ قالب نشر يشغّل النسخ من جهاز افتراضي عابر صغير في السحابة — فلا تمر البايتات عبر شبكة حاسوبك المحمول مطلقًا.
cloud-offload-render-button = إنشاء القالب
cloud-offload-copy-clipboard = نسخ إلى الحافظة
cloud-offload-template-format = تنسيق القالب
cloud-offload-self-destruct-warning = يتوقف الجهاز الافتراضي تلقائيًا بعد { $minutes } دقيقة — أكّد دور IAM والمنطقة قبل النشر.

# Phase 41 — animated before/after tree-diff preview. The `Preview
# changes` modal renders the rolled-up plan before the engine starts
# work; 14 keys cover the title, the summary header / counts, the
# row-reason labels, and the two action buttons.
preview-modal-title = معاينة التغييرات
preview-summary-header = ما الذي سيحدث
preview-category-additions = { $count } إضافة
preview-category-replacements = { $count } استبدال
preview-category-skips = { $count } تم تخطّيها
preview-category-conflicts = { $count } تعارض
preview-category-unchanged = { $count } دون تغيير
preview-bytes-to-transfer = { $bytes } للنقل
preview-reason-source-newer = المصدر أحدث
preview-reason-dest-newer = الوجهة أحدث — سيتم التخطّي
preview-reason-content-different = المحتوى مختلف
preview-reason-identical = مطابق للمصدر
preview-button-run = تشغيل الخطة
preview-button-reduce = تقليص خطتي…

# Phase 42 — perceptual-hash visual-similarity dedup. Eight keys cover
# the pre-copy "looks visually identical" warning + the Settings panel.
perceptual-warn-title = يبدو متطابقًا بصريًا
perceptual-warn-body = يبدو أن { $name } في الوجهة يطابق صورة المصدر. هل تريد المتابعة في النسخ على أي حال؟
perceptual-warn-keep-both = الاحتفاظ بكليهما
perceptual-warn-skip = تخطّي هذا الملف
perceptual-warn-overwrite = الكتابة فوقه على أي حال
perceptual-settings-heading = إزالة التكرار حسب التشابه البصري
perceptual-settings-hint = اكتشاف الصور المتطابقة بصريًا في الوجهة قبل الكتابة فوقها. التجزئة إدراكية (تتعرّف على الصورة نفسها وقد أُعيد حفظها بتنسيق مختلف)، وليست دقيقة على مستوى البايت.
perceptual-settings-threshold-label = عتبة التحذير (أقل = مطابقة أكثر صرامة)

# Phase 42 Part B — per-file rolling versions (Time Machine for any
# destination). 8 keys cover the version-list panel + retention picker.
version-list-heading = الإصدارات السابقة
version-list-empty = لا توجد إصدارات سابقة لهذا الملف
version-list-restore = استعادة هذا الإصدار
version-retention-heading = الاحتفاظ بالإصدارات السابقة عند الكتابة فوقها
version-retention-none = الاحتفاظ بكل إصدار إلى الأبد
version-retention-last-n = الاحتفاظ بآخر { $n } إصدار
version-retention-older-than-days = إسقاط الإصدارات الأقدم من { $days } يومًا
version-retention-gfs = كل ساعة { $h } · يوميًا { $d } · أسبوعيًا { $w } · شهريًا { $m }

# Phase 43 — forensic chain-of-custody manifests + BLAKE3 verified
# streaming. 14 keys cover the Settings → Provenance panel
# (heading, hint, toggles, TSA URL, signing-key management) plus
# the post-job manifest notification + the verify command's two
# headline result lines.
provenance-settings-heading = سلسلة عهدة جنائية
provenance-settings-hint = وقّع كل مهمة نسخ ببيان BLAKE3 + ed25519. يستطيع المراجعون إعادة تجزئة شجرة الوجهة لاحقًا وإثبات عدم تغيّر أي بايت منذ النسخ.
provenance-settings-enable-default = توقيع كل مهمة جديدة افتراضيًا
provenance-settings-show-after-job = إظهار البيان بعد كل مهمة مكتملة
provenance-settings-tsa-url-label = عنوان URL الافتراضي لهيئة الطوابع الزمنية RFC 3161
provenance-settings-tsa-url-hint = اختياري. عند التعيين، تحمل البيانات طابعًا زمنيًا مجانيًا من TSA يثبت وجود البايتات في هذه اللحظة الزمنية. اتركه فارغًا للتخطّي.
provenance-settings-keys-heading = مفاتيح التوقيع
provenance-settings-keys-generate = إنشاء مفتاح جديد
provenance-settings-keys-import = استيراد مفتاح…
provenance-settings-keys-export = تصدير المفتاح العام…
provenance-job-completed-title = تم حفظ بيان المصدر
provenance-job-completed-body = تم توقيع { $count } ملف ← { $path }
provenance-verify-clean = البيان صالح لـ { $count } ملف؛ التوقيع { $sig }؛ جذر Merkle سليم.
provenance-verify-tampered = البيان غير صالح — { $tampered } تم العبث بها، { $missing } مفقودة.
# Phase 43 post-review hardening — toast text for the Settings →
# Provenance buttons whose Tauri IPC has not yet landed.
provenance-action-staged = المرحلة 43 — سيصل ربط IPC لهذا الإجراء في التزام لاحق.

# Phase 44 — SSD-aware whole-drive sanitize (NVMe Sanitize / OPAL
# Crypto Erase) + the per-file shred refusal on copy-on-write
# filesystems. 16 keys cover the new "Drive sanitize" Settings
# subsection and the localized error messages.
sanitize-heading = مسح آمن لكامل القرص
sanitize-hint = تمسح NVMe Sanitize وOPAL Crypto Erase وATA Secure Erase قرص فلاش على طبقة البرنامج الثابت خلال أجزاء من الثانية. الكتابة فوق الملف الواحد بلا معنى على الفلاش — والتمزيق متعدد المرات يستهلك NAND فقط. استخدم هذا للمسح الفعلي.
sanitize-pick-device = اختر القرص المراد مسحه
sanitize-mode-label = طريقة المسح الآمن
sanitize-mode-nvme-format = NVMe Format (مع المسح الآمن)
sanitize-mode-nvme-sanitize-block = NVMe Sanitize — مسح بالكتل (بطيء، لكل خلية)
sanitize-mode-nvme-sanitize-crypto = NVMe Sanitize — مسح تشفيري (فوري)
sanitize-mode-ata-secure-erase = ATA Secure Erase (أقراص SATA SSD القديمة)
sanitize-mode-opal-crypto-erase = TCG OPAL Crypto Erase (الأقراص ذاتية التشفير)
sanitize-mode-apfs-crypto-erase = APFS Crypto Erase (تدوير مفتاح FileVault، macOS فقط)
sanitize-confirm-1 = هذا يدمّر كل بايت على { $device }. لا يمكن التراجع.
sanitize-confirm-2 = أُدرك أن كل الأقسام وكل الملفات وكل اللقطات على { $device } ستصبح غير قابلة للقراءة بشكل دائم.
sanitize-confirm-3 = اكتب اسم طراز القرص للمتابعة: { $model }
sanitize-running = جارٍ مسح { $device } ({ $mode }) — قد يستغرق هذا من أجزاء من الثانية (المسح التشفيري) إلى عشرات الدقائق (المسح بالكتل). لا توقف التشغيل.
sanitize-completed = اكتمل المسح الآمن — أصبح { $device } فارغًا الآن.
ssd-honest-shred-meaningless = التمزيق للملف الواحد على نظام ملفات بالكتابة عند النسخ (Btrfs / ZFS / APFS) لا يمكنه الوصول إلى الكتل الأساسية. استخدم بدلًا من ذلك المسح الآمن لكامل القرص مع تدوير مفتاح التشفير الكامل للقرص.
ssd-honest-advisory = هذا الملف موجود على فلاش. الكتابة فوق الملف الواحد تستهلك NAND ولا تضمن أن الخلايا الأصلية غير قابلة للاسترداد. للبيانات الحساسة، امسح كامل القرص بأمان.

# Phase 44.1f post-review — placeholder toast for SanitizeTab
# buttons whose Tauri IPC has not yet landed.
sanitize-action-staged = المرحلة 44.1 — سيصل ربط IPC لهذا الإجراء في التزام لاحق.

# Phase 45.3 — named-queue tab strip (Subfeature A). Tabs surface
# once the QueueRegistry holds at least one queue; the synthesised
# default tab keeps legacy single-queue jobs reachable.
queue-tab-default = افتراضي
queue-tab-empty-state = قوائم انتظار المهام
queue-badge-tooltip = المهام المعلّقة والجارية في قائمة الانتظار هذه

# Phase 45.4 — drag-progress-merge (Subfeature B). Drag a queue tab
# onto another to merge their job lists. The default tab is neither
# draggable nor a drop target; only registry queues participate.
queue-drag-hint = اسحب فوق قائمة انتظار أخرى للدمج
queue-merge-confirm = أفلِت للدمج
queue-merge-toast = تم دمج قوائم الانتظار

# Phase 45.5 — F2-queue UX (Subfeature C). F2 toggles
# `auto_enqueue_next` so every fresh enqueue piles into the running
# queue rather than spawning a parallel one. Status pill renders in
# the Footer; pulsing dot renders on the running tab.
queue-f2-active-hint = وضع F2: كل إضافة جديدة تذهب إلى هذه القائمة
queue-f2-toggled-on = وضع قائمة F2 مفعّل — الإضافات الجديدة تنضم إلى القائمة الجارية
queue-f2-toggled-off = وضع قائمة F2 معطّل — الإضافات الجديدة تنشئ قوائم انتظار متوازية
queue-f2-status-bar = وضع قائمة F2: مفعّل

# Phase 45.6 — tray destination targets (Subfeature D). Pinned
# destinations appear in the OS tray menu; clicking one arms it as
# the active drop target so the next file drop bypasses the
# DropStagingDialog. Settings → General hosts the list editor.
tray-target-section-title = وجهات علبة النظام
tray-target-section-hint = تظهر الوجهات المثبّتة في قائمة علبة النظام. انقر إحداها لتجهيزها كهدف الإفلات التالي.
tray-target-empty = لم تُثبَّت أي وجهات لعلبة النظام بعد.
tray-target-remove = إزالة
tray-target-add-label = التسمية
tray-target-add-path = المسار أو عنوان URI للواجهة الخلفية
tray-target-add = إضافة
tray-target-armed-toast = أفلِت ملفك التالي لإرساله إلى { $label }
tray-target-active-pill = ← { $label }

# Phase 45.7 follow-up — pinned-destination validation errors. The
# `queue_pin_destination` IPC returns these Fluent keys verbatim
# when input fails the IPC-boundary checks; the toast layer renders
# them via `t(...)` (Toast.svelte detects kebab-case-lowercase and
# routes through the locale table). Phase 17e's `err-path-escape` /
# `err-destination-empty` cover the path-traversal + empty cases for
# `queue_route_job`; the keys below are pin-specific.
err-pinned-destination-label-empty = لا يمكن أن تكون تسمية وجهة علبة النظام فارغة.
err-pinned-destination-path-empty = لا يمكن أن يكون مسار وجهة علبة النظام فارغًا.
err-pinned-destination-label-too-long = تسمية وجهة علبة النظام طويلة جدًا (بحد أقصى 64 حرفًا).
err-pinned-destination-path-too-long = مسار وجهة علبة النظام طويل جدًا (بحد أقصى 1024 حرفًا).
err-pinned-destination-label-invalid = تحتوي تسمية وجهة علبة النظام على أحرف غير مسموح بها (سطر جديد أو رجوع أو NUL).
err-pinned-destination-path-invalid = يحتوي مسار وجهة علبة النظام على أحرف غير مسموح بها (سطر جديد أو رجوع أو NUL).
err-pinned-destination-too-many = لقد بلغت حد 50 وجهة لعلبة النظام. أزِل واحدة لإضافة أخرى.

# Phase 46.6 — Settings → Plugins tab. The IPC layer in
# `apps/copythat-ui/src-tauri/src/plugin_commands.rs` enumerates the
# per-user plugin store under `<config_dir>/plugins/`; the
# `PluginsTab.svelte` component renders these strings against the
# returned manifest + grant state.
settings-tab-plugins = الإضافات
plugin-heading = الإضافات
plugin-hint = توسّع إضافات WASM المعزولة Copy That بخطافات مخصّصة. تعمل كل إضافة ضمن حدود لوحدة المعالجة والذاكرة لكل استدعاء، ولا ترى سوى إمكانات المضيف التي تمنحها لها.
plugin-list-empty = لم تُثبَّت أي إضافات بعد.
plugin-enabled = مفعّلة
plugin-disabled = معطّلة
plugin-hooks = الخطافات
plugin-capabilities = الإمكانات
plugin-no-capabilities = (لا شيء)
plugin-directory = الموقع
plugin-install-from-file = التثبيت من ملف…
plugin-install-from-url = التثبيت من عنوان URL…
plugin-url-wasm = عنوان URL لـ WASM
plugin-url-manifest = عنوان URL للبيان
plugin-url-hash = تجزئة BLAKE3
plugin-url-preview = معاينة
plugin-url-confirm = تأكيد التثبيت

# Phase 31b — power-policy settings (Power tab).
settings-tab-power = الطاقة
settings-power-hint = تقييد أو إيقاف النسخ حسب حالة الطاقة: البطارية، الشبكات محدودة الاستهلاك/الخلوية، أثناء العرض/ملء الشاشة، أو عند خفض تردد المعالج حرارياً.
settings-power-enabled = تفعيل التقييد حسب الطاقة
settings-power-battery = على البطارية
settings-power-metered = على شبكة محدودة الاستهلاك
settings-power-cellular = على الشبكة الخلوية
settings-power-presentation = أثناء العرض التقديمي
settings-power-fullscreen = في وضع ملء الشاشة
settings-power-thermal = عند الخفض الحراري
settings-power-continue = متابعة
settings-power-pause = إيقاف مؤقت
err-server-not-implemented = وضع الخادم غير متاح بعد.
err-webhook-not-implemented = تسليم الـ Webhook غير متاح بعد.

# Phase 47 — "why is this slow?" diagnostics (bottleneck badge + tooltip).
bottleneck-source-io = المصدر I/O
bottleneck-dest-io = الوجهة I/O
bottleneck-network = الشبكة
bottleneck-antivirus = مكافح الفيروسات
bottleneck-cpu = CPU
bottleneck-thermal = حراري
bottleneck-unknown = غير معروف
diag-aria = عنق الزجاجة: { $cause }
diag-tooltip = مقيَّد بـ { $cause } · { $rate }
diag-spark-aria = الإنتاجية خلال الدقيقة الأخيرة
diag-keeping-up = مواكب
diag-label = التشخيص
