app-name = Copy That v1.25.0
# MT
window-title = Copy That v1.25.0
# MT
shred-ssd-advisory = تحذير: هذا الهدف موجود على قرص SSD. لا توفر عمليات الكتابة المتعددة تطهيرًا موثوقًا لذاكرة الفلاش لأن موازنة التآكل والتخصيص الإضافي ينقلان البيانات بعيدًا عن عنوان الكتلة المنطقي. بالنسبة لوسائط الحالة الصلبة، يُفضَّل استخدام ATA SECURE ERASE أو NVMe Format مع المحو الآمن أو التشفير الكامل للقرص مع التخلص من المفتاح.

# MT
state-idle = في وضع الخمول
# MT
state-copying = جارٍ النسخ
# MT
state-verifying = جارٍ التحقق
# MT
state-paused = متوقف مؤقتًا
# MT
state-error = خطأ

# MT
state-pending = في قائمة الانتظار
# MT
state-running = قيد التشغيل
# MT
state-cancelled = ملغى
# MT
state-succeeded = مكتمل
# MT
state-failed = فشل

# MT
action-pause = إيقاف مؤقت
# MT
action-resume = استئناف
# MT
action-cancel = إلغاء
# MT
action-pause-all = إيقاف كل المهام مؤقتًا
# MT
action-resume-all = استئناف كل المهام
# MT
action-cancel-all = إلغاء كل المهام
# MT
action-close = إغلاق
# MT
action-reveal = عرض في المجلد

# MT
menu-pause = إيقاف مؤقت
# MT
menu-resume = استئناف
# MT
menu-cancel = إلغاء
# MT
menu-remove = إزالة من قائمة الانتظار
# MT
menu-reveal-source = عرض المصدر في المجلد
# MT
menu-reveal-destination = عرض الوجهة في المجلد

# MT
header-eta-label = الوقت المتبقي المقدَّر
# MT
header-toolbar-label = عناصر التحكم العامة

# MT
footer-queued = مهام نشطة
# MT
footer-total-bytes = قيد التنفيذ
# MT
footer-errors = أخطاء
# MT
footer-history = السجل

# MT
empty-title = أسقط ملفات أو مجلدات للنسخ
# MT
empty-hint = اسحب العناصر إلى النافذة. سنطلب منك وجهة ثم ننشئ مهمة لكل مصدر.
# MT
empty-region-label = قائمة المهام

# MT
details-drawer-label = تفاصيل المهمة
# MT
details-source = المصدر
# MT
details-destination = الوجهة
# MT
details-state = الحالة
# MT
details-bytes = البايتات
# MT
details-files = الملفات
# MT
details-speed = السرعة
# MT
details-eta = الوقت المتبقي
# MT
details-error = خطأ

# MT
drop-dialog-title = نقل العناصر المُسقَطة
# MT
drop-dialog-subtitle = { $count } عنصر جاهز للنقل. اختر مجلد الوجهة للبدء.
# MT
drop-dialog-mode = العملية
# MT
drop-dialog-copy = نسخ
# MT
drop-dialog-move = نقل
# MT
drop-dialog-pick-destination = اختر الوجهة
# MT
drop-dialog-change-destination = تغيير الوجهة
# MT
drop-dialog-start-copy = بدء النسخ
# MT
drop-dialog-start-move = بدء النقل

# MT
eta-calculating = جارٍ الحساب…
# MT
eta-unknown = غير معروف

# MT
toast-job-done = اكتمل النقل
# MT
toast-copy-queued = تم إضافة النسخ إلى قائمة الانتظار
# MT
toast-move-queued = تم إضافة النقل إلى قائمة الانتظار
# MT — Phase 8 toast messages
toast-error-resolved = تم حل الخطأ
# MT
toast-collision-resolved = تم حل التعارض
# MT
toast-elevated-unavailable = إعادة المحاولة بصلاحيات مرتفعة ستصل في المرحلة 17 — غير متاحة بعد
toast-clipboard-files-detected = ملفات في الحافظة — اضغط على اختصار اللصق للنسخ عبر Copy That
toast-clipboard-no-files = لا توجد ملفات في الحافظة للصقها
# MT
toast-error-log-exported = تم تصدير سجل الأخطاء

# MT — Error modal
error-modal-title = فشل أحد عمليات النقل
# MT
error-modal-retry = إعادة المحاولة
# MT
error-modal-retry-elevated = إعادة المحاولة بصلاحيات مرتفعة
# MT
error-modal-skip = تخطّي
# MT
error-modal-skip-all-kind = تخطّي كل الأخطاء من هذا النوع
# MT
error-modal-abort = إلغاء الكل
# MT
error-modal-path-label = المسار
# MT
error-modal-code-label = الرمز
error-drawer-pending-count = المزيد من الأخطاء في الانتظار
error-drawer-toggle = طيّ أو توسيع

# MT — Error-kind labels
err-not-found = الملف غير موجود
# MT
err-permission-denied = تم رفض الإذن
# MT
err-disk-full = قرص الوجهة ممتلئ
# MT
err-interrupted = تمت مقاطعة العملية
# MT
err-verify-failed = فشل التحقق بعد النسخ
# MT
err-path-escape = تم رفض المسار — يحتوي على مقاطع الدليل الأصلي (..) أو بايتات غير مسموح بها
# MT
err-path-invalid-encoding = Path rejected — string contains invalid UTF-8 / replacement characters
# MT
err-io-other = خطأ إدخال/إخراج غير معروف
err-sparseness-mismatch = لا يمكن الحفاظ على تخطيط الملف المتناثر في الوجهة  # MT

# MT — Collision modal
collision-modal-title = الملف موجود بالفعل
# MT
collision-modal-overwrite = استبدال
# MT
collision-modal-overwrite-if-newer = استبدال إذا كان أحدث
# MT
collision-modal-skip = تخطّي
# MT
collision-modal-keep-both = الاحتفاظ بكليهما
# MT
collision-modal-rename = إعادة التسمية…
# MT
collision-modal-apply-to-all = تطبيق على الكل
# MT
collision-modal-source = المصدر
# MT
collision-modal-destination = الوجهة
# MT
collision-modal-size = الحجم
# MT
collision-modal-modified = آخر تعديل
# MT
collision-modal-hash-check = تجزئة سريعة (SHA-256)
# MT
collision-modal-rename-placeholder = اسم الملف الجديد
# MT
collision-modal-confirm-rename = إعادة التسمية

# MT — Error log drawer
error-log-title = سجل الأخطاء
# MT
error-log-empty = لا توجد أخطاء مسجَّلة
# MT
error-log-export-csv = تصدير CSV
# MT
error-log-export-txt = تصدير نص
# MT
error-log-clear = مسح السجل
# MT
error-log-col-time = الوقت
# MT
error-log-col-job = المهمة
# MT
error-log-col-path = المسار
# MT
error-log-col-code = الرمز
# MT
error-log-col-message = الرسالة
# MT
error-log-col-resolution = الحل

# MT — History drawer (Phase 9)
history-title = السجل
# MT
history-empty = لم تُسجَّل أي مهام بعد
# MT
history-unavailable = سجل النسخ غير متاح. تعذّر على التطبيق فتح مخزن SQLite عند بدء التشغيل.
# MT
history-filter-any = الكل
# MT
history-filter-kind = النوع
# MT
history-filter-status = الحالة
# MT
history-filter-text = بحث
# MT
history-refresh = تحديث
# MT
history-export-csv = تصدير CSV
# MT
history-purge-30 = حذف أقدم من 30 يومًا
# MT
history-rerun = إعادة التشغيل
# MT
history-detail-open = تفاصيل
# MT
history-detail-title = تفاصيل المهمة
# MT
history-detail-empty = لا توجد عناصر مسجَّلة
# MT
history-col-date = التاريخ
# MT
history-col-kind = النوع
# MT
history-col-src = المصدر
# MT
history-col-dst = الوجهة
# MT
history-col-files = الملفات
# MT
history-col-size = الحجم
# MT
history-col-status = الحالة
# MT
history-col-duration = المدة
# MT
history-col-error = الخطأ

# MT
toast-history-exported = تم تصدير السجل
# MT
toast-history-rerun-queued = تمت إضافة إعادة التشغيل إلى قائمة الانتظار

# MT — Totals drawer (Phase 10)
footer-totals = الإجماليات
# MT
totals-title = الإجماليات
# MT
totals-loading = جارٍ تحميل الإجماليات…
# MT
totals-card-bytes = إجمالي البايتات المنسوخة
# MT
totals-card-files = الملفات
# MT
totals-card-jobs = المهام
# MT
totals-card-avg-rate = متوسط السرعة
# MT
totals-errors = أخطاء
# MT
totals-spark-title = آخر 30 يومًا
# MT
totals-kinds-title = حسب النوع
# MT
totals-saved-title = الوقت الموفَّر (تقديري)
# MT
totals-saved-note = تقديري مقارنةً بنسخة مرجعية بنفس الحمولة باستخدام مدير ملفات قياسي.
# MT
totals-reset = إعادة تعيين الإحصاءات
# MT
totals-reset-confirm = سيؤدي ذلك إلى حذف كل المهام والعناصر المخزَّنة. هل تريد المتابعة؟
# MT
totals-reset-confirm-yes = نعم، إعادة تعيين
# MT
toast-totals-reset = تمت إعادة تعيين الإحصاءات

# MT — Phase 11a additions
header-language-label = اللغة
# MT
header-language-title = تغيير اللغة

# MT
kind-copy = نسخ
# MT
kind-move = نقل
# MT
kind-delete = حذف
# MT
kind-secure-delete = حذف آمن

# MT
status-running = قيد التشغيل
# MT
status-succeeded = ناجح
# MT
status-failed = فشل
# MT
status-cancelled = ملغى
# MT
status-ok = موافق
# MT
status-skipped = تم التخطّي

# MT
history-search-placeholder = /المسار
# MT
toast-history-purged = تم حذف { $count } مهمة أقدم من 30 يومًا

# MT
err-source-required = يجب تحديد مسار مصدر واحد على الأقل.
# MT
err-destination-empty = مسار الوجهة فارغ.
# MT
err-source-empty = مسار المصدر فارغ.

# MT
duration-lt-1s = < 1 ث
# MT
duration-ms = { $ms } مل.ث
# MT
duration-seconds = { $s } ث
# MT
duration-minutes-seconds = { $m } د { $s } ث
# MT
duration-hours-minutes = { $h } س { $m } د
# MT
duration-zero = 0 ث

# MT
rate-unit-per-second = { $size }/ث

# MT — Phase 11b Settings modal
settings-title = الإعدادات
# MT
settings-tab-general = عام
# MT
settings-tab-appearance = المظهر
# MT
settings-section-language = اللغة
# MT
settings-phase-12-hint = ستضاف المزيد من الإعدادات (السمة، إعدادات النقل الافتراضية، خوارزمية التحقق، الملفات الشخصية) في المرحلة 12.

# MT — Phase 12 Settings window
settings-loading = جارٍ تحميل الإعدادات…
# MT
settings-tab-transfer = النقل
# MT
settings-tab-shell = الصدفة
# MT
settings-tab-secure-delete = الحذف الآمن
# MT
settings-tab-advanced = متقدم
# MT
settings-tab-profiles = الملفات الشخصية

# MT
settings-section-theme = السمة
# MT
settings-theme-auto = تلقائي
# MT
settings-theme-light = فاتح
# MT
settings-theme-dark = داكن
# MT
settings-start-with-os = تشغيل عند بدء النظام
# MT
settings-single-instance = مثيل واحد فقط قيد التشغيل
# MT
settings-minimize-to-tray = التصغير إلى صينية النظام عند الإغلاق
settings-error-display-mode = نمط عرض الخطأ
settings-error-display-modal = نافذة مشروطة (يحظر التطبيق)
settings-error-display-drawer = لوحة جانبية (غير حاجبة)
settings-error-display-mode-hint = النافذة المشروطة تُوقف قائمة الانتظار حتى تقرر. اللوحة الجانبية تُبقي القائمة تعمل وتسمح بمعالجة الأخطاء في الزاوية.
settings-paste-shortcut = لصق الملفات عبر اختصار عام
settings-paste-shortcut-combo = تركيبة الاختصار
settings-paste-shortcut-hint = اضغط على هذه التركيبة في أي مكان في النظام للصق الملفات المنسوخة من Explorer / Finder / Files عبر Copy That. يتم حل CmdOrCtrl إلى Cmd على macOS و Ctrl على Windows / Linux.
settings-clipboard-watcher = مراقبة الحافظة للملفات المنسوخة
settings-clipboard-watcher-hint = يعرض إشعارًا عند ظهور عناوين URL للملفات في الحافظة، ملمحًا أنه يمكنك اللصق عبر Copy That. يستطلع كل 500 مللي ثانية عند التمكين.

# MT
settings-buffer-size = حجم المخزن المؤقت
# MT
settings-verify = التحقق بعد النسخ
# MT
settings-verify-off = معطل
# MT
settings-concurrency = التزامن
# MT
settings-concurrency-auto = تلقائي
# MT
settings-reflink = Reflink / المسارات السريعة
# MT
settings-reflink-prefer = تفضيل
# MT
settings-reflink-avoid = تجنب reflink
# MT
settings-reflink-disabled = استخدم محرك async دائمًا
# MT
settings-fsync-on-close = مزامنة على القرص عند الإغلاق (أبطأ، أكثر أمانًا)
# MT
settings-preserve-timestamps = الاحتفاظ بطوابع الوقت
# MT
settings-preserve-permissions = الاحتفاظ بالأذونات
# MT
settings-preserve-acls = الاحتفاظ بقوائم التحكم في الوصول (المرحلة 14)
settings-preserve-sparseness = الاحتفاظ بالملفات المتناثرة  # MT
settings-preserve-sparseness-hint = انسخ فقط النطاقات المخصصة للملفات المتناثرة (أقراص الجهاز الظاهري، ملفات قاعدة البيانات) حتى يظل حجم الوجهة على القرص هو نفسه حجم المصدر.  # MT

# MT
settings-context-menu = تفعيل عناصر قائمة السياق
# MT
settings-intercept-copy = اعتراض معالج النسخ الافتراضي (Windows)
# MT
settings-intercept-copy-hint = عند التفعيل، يمر Ctrl+C / Ctrl+V في Explorer عبر Copy That. التسجيل في المرحلة 14.
# MT
settings-notify-completion = إشعار عند اكتمال المهمة

# MT
settings-shred-method = طريقة الإتلاف الافتراضية
# MT
settings-shred-zero = صفر (مرور واحد)
# MT
settings-shred-random = عشوائي (مرور واحد)
# MT
settings-shred-dod3 = DoD 5220.22-M (3 مرور)
# MT
settings-shred-dod7 = DoD 5220.22-M (7 مرور)
# MT
settings-shred-gutmann = Gutmann (35 مرور)
# MT
settings-shred-nist = NIST 800-88
# MT
settings-shred-confirm-twice = طلب تأكيد مزدوج قبل الإتلاف

# MT
settings-log-level = مستوى السجل
# MT
settings-log-off = معطل
# MT
settings-telemetry = القياسات عن بُعد
# MT
settings-telemetry-never = أبدًا — لا يُرسَل أي بيانات عند أي مستوى للسجل
# MT
settings-error-policy = سياسة الخطأ الافتراضية
# MT
settings-error-policy-ask = السؤال
# MT
settings-error-policy-skip = تخطّي
# MT
settings-error-policy-retry = إعادة المحاولة مع تأخير
# MT
settings-error-policy-abort = إلغاء عند أول خطأ
# MT
settings-history-retention = مدة الاحتفاظ بالسجل (أيام)
# MT
settings-history-retention-hint = 0 = الاحتفاظ دائمًا. أي قيمة أخرى تحذف تلقائيًا المهام القديمة عند بدء التشغيل.
# MT
settings-database-path = مسار قاعدة البيانات
# MT
settings-database-path-default = (افتراضي — مجلد بيانات النظام)
# MT
settings-reset-all = إعادة التعيين إلى الافتراضيات
# MT
settings-reset-confirm = إعادة تعيين كل التفضيلات؟ لن تتأثر الملفات الشخصية.

# MT
settings-profiles-hint = احفظ الإعدادات الحالية باسم؛ حمِّلها لاحقًا للتبديل دون لمس كل عنصر تحكم على حدة.
# MT
settings-profile-name-placeholder = اسم الملف الشخصي
# MT
settings-profile-save = حفظ
# MT
settings-profile-import = استيراد…
# MT
settings-profile-load = تحميل
# MT
settings-profile-export = تصدير…
# MT
settings-profile-delete = حذف
# MT
settings-profile-empty = لا توجد ملفات شخصية محفوظة بعد.
# MT
settings-profile-import-prompt = اسم الملف الشخصي المستورد:

# MT
toast-settings-reset = تمت إعادة تعيين الإعدادات
# MT
toast-profile-saved = تم حفظ الملف الشخصي
# MT
toast-profile-loaded = تم تحميل الملف الشخصي
# MT
toast-profile-exported = تم تصدير الملف الشخصي
# MT
toast-profile-imported = تم استيراد الملف الشخصي

# Phase 13d — activity feed + header picker buttons
action-add-files = إضافة ملفات
action-add-folders = إضافة مجلدات
activity-title = النشاط
activity-clear = مسح قائمة النشاط
activity-empty = لا يوجد نشاط للملفات بعد.
activity-after-done = عند الانتهاء:
activity-keep-open = إبقاء التطبيق مفتوحًا
activity-close-app = إغلاق التطبيق
activity-shutdown = إيقاف تشغيل الحاسوب
activity-logoff = تسجيل الخروج
activity-sleep = سكون

# Phase 14 — preflight free-space dialog
preflight-block-title = لا توجد مساحة كافية في الوجهة
preflight-warn-title = مساحة قليلة في الوجهة
preflight-unknown-title = تعذر تحديد المساحة الفارغة
preflight-unknown-body = المصدر كبير جدًا بحيث لا يمكن قياسه بسرعة أو أن محرك الوجهة لم يستجب. يمكنك المتابعة؛ سيوقف المحرك النسخ بأمان عند نفاد المساحة.
preflight-required = مطلوب
preflight-free = فارغ
preflight-reserve = محجوز
preflight-shortfall = عجز
preflight-continue = متابعة على أي حال
collision-modal-overwrite-older = استبدال الأقدم فقط

# Phase 14e — subset picker
preflight-pick-subset = اختر ما تنسخه…
subset-title = اختر المصادر المراد نسخها
subset-subtitle = لا تتسع المجموعة الكاملة في الوجهة. ضع علامة على ما تريد نسخه؛ يبقى الباقي.
subset-loading = جاري قياس الأحجام…
subset-too-large = أكبر من أن يُحصى
subset-budget = متاح
subset-remaining = متبقي
subset-confirm = نسخ المحدد
history-rerun-hint = أعد تشغيل هذا النسخ — يعيد فحص كل ملف في شجرة المصدر
history-clear-all = مسح الكل
history-clear-all-confirm = انقر مرة أخرى للتأكيد
history-clear-all-hint = حذف جميع صفوف السجل. يتطلب نقرة ثانية للتأكيد.
toast-history-cleared = تم مسح السجل ({ $count } صفوف أُزيلت)

# Phase 15 — source-list ordering
drop-dialog-sort-label = الترتيب:
sort-custom = مخصص
sort-name-asc = الاسم أ ← ي (الملفات أولًا)
sort-name-desc = الاسم ي ← أ (الملفات أولًا)
sort-size-asc = الحجم الأصغر أولًا (الملفات أولًا)
sort-size-desc = الحجم الأكبر أولًا (الملفات أولًا)
sort-reorder = إعادة ترتيب
sort-move-top = نقل إلى الأعلى
sort-move-up = تحريك للأعلى
sort-move-down = تحريك للأسفل
sort-move-bottom = نقل إلى الأسفل
sort-name-asc-simple = الاسم أ ← ي
sort-name-desc-simple = الاسم ي ← أ
sort-size-asc-simple = الأصغر حجمًا أولًا
sort-size-desc-simple = الأكبر حجمًا أولًا
activity-sort-locked = الترتيب معطل أثناء تشغيل النسخ. أوقف مؤقتًا أو انتظر حتى ينتهي ثم غيّر الترتيب.
drop-dialog-collision-label = إذا كان الملف موجودًا:
collision-policy-keep-both = الإبقاء على كليهما (إعادة تسمية النسخة الجديدة إلى _2 و_3 و…)
collision-policy-skip = تخطي النسخة الجديدة
collision-policy-overwrite = استبدال الملف الموجود
collision-policy-overwrite-if-newer = الاستبدال فقط إذا كان أحدث
collision-policy-prompt = السؤال في كل مرة
drop-dialog-busy-checking = جارٍ فحص المساحة الفارغة…
drop-dialog-busy-enumerating = جارٍ إحصاء الملفات…
drop-dialog-busy-starting = جارٍ بدء النسخ…
toast-enumeration-deferred = شجرة المصدر كبيرة — تخطّي قائمة الملفات المسبقة؛ ستظهر الصفوف أثناء معالجة المحرك.

# Phase 14a — enumeration-time filters
# MT
settings-tab-filters = المرشحات
# MT
settings-filters-hint = يتخطى الملفات أثناء العدّ فلا يفتحها المحرك أصلاً. "تضمين" يُطبّق على الملفات فقط؛ "استبعاد" يقلّم المجلدات المطابقة أيضًا.
# MT
settings-filters-enabled = تفعيل المرشحات لنسخ الأشجار
# MT
settings-filters-include-globs = أنماط التضمين
# MT
settings-filters-include-globs-placeholder = **/*.txt
# MT
settings-filters-include-globs-hint = نمط واحد في كل سطر. إن لم تكن فارغة، يجب أن يطابق الملف نمطًا واحدًا على الأقل. تُجتاز المجلدات دائمًا.
# MT
settings-filters-exclude-globs = أنماط الاستبعاد
# MT
settings-filters-exclude-globs-placeholder = **/node_modules
# MT
settings-filters-exclude-globs-hint = نمط واحد في كل سطر. تطابقات المجلدات تقلّم الشجرة الفرعية بأكملها؛ الملفات المطابقة تُتخطى.
# MT
settings-filters-size-range = نطاق حجم الملف
# MT
settings-filters-min-size-bytes = الحد الأدنى (بايت، فارغ = بلا حد)
# MT
settings-filters-max-size-bytes = الحد الأقصى (بايت، فارغ = بلا حد)
# MT
settings-filters-date-range = نطاق تاريخ التعديل
# MT
settings-filters-min-mtime = تم تعديله في أو بعد
# MT
settings-filters-max-mtime = تم تعديله في أو قبل
# MT
settings-filters-attributes = السمات
# MT
settings-filters-skip-hidden = تخطّي الملفات / المجلدات المخفية
# MT
settings-filters-skip-system = تخطّي ملفات النظام (Windows فقط)
# MT
settings-filters-skip-readonly = تخطّي الملفات للقراءة فقط

# Phase 15 — auto-update
# MT
settings-tab-updater = التحديثات
# MT
settings-updater-hint = يتحقق Copy That من التحديثات الموقَّعة مرة واحدة يوميًا كحد أقصى. تُثبَّت التحديثات عند إنهاء التطبيق في المرة التالية.
# MT
settings-updater-auto-check = التحقق من التحديثات عند التشغيل
# MT
settings-updater-channel = قناة الإصدار
# MT
settings-updater-channel-stable = مستقر
# MT
settings-updater-channel-beta = تجريبي (قبل الإصدار)
# MT
settings-updater-last-check = آخر فحص
# MT
settings-updater-last-never = أبدًا
# MT
settings-updater-check-now = تحقَّق من التحديثات الآن
# MT
settings-updater-checking = جارٍ التحقق…
# MT
settings-updater-available = تحديث متاح
# MT
settings-updater-up-to-date = أنت تستخدم أحدث إصدار.
# MT
settings-updater-dismiss = تخطَّ هذا الإصدار
# MT
settings-updater-dismissed = تم التخطي
# MT
toast-update-available = يتوفر إصدار أحدث
# MT
toast-update-up-to-date = أنت بالفعل على أحدث إصدار

# Phase 19a — disk-backed file enumeration (TeraCopy-compatible scan DB)
# MT
scan-progress-title = جارٍ المسح…
# MT
scan-progress-stats = { $files } ملفات · { $bytes } حتى الآن
# MT
scan-pause-button = إيقاف المسح مؤقتًا
# MT
scan-resume-button = استئناف المسح
# MT
scan-cancel-button = إلغاء المسح
# MT
scan-cancel-confirm = إلغاء المسح وتجاهل التقدم؟
# MT
scan-db-header = قاعدة بيانات المسح
# MT
scan-db-hint = قاعدة بيانات مسح على القرص لمهام متعددة الملايين من الملفات.
# MT
advanced-scan-hash-during = حساب المجاميع الاختبارية أثناء المسح
# MT
advanced-scan-db-path = موقع قاعدة بيانات المسح
# MT
advanced-scan-retention-days = حذف عمليات المسح المكتملة تلقائيًا بعد (أيام)
# MT
advanced-scan-max-keep = الحد الأقصى لعدد قواعد بيانات المسح المحتفظ بها

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
sparse-not-supported-title = الوجهة تملأ الملفات المتناثرة  # MT
sparse-not-supported-body = { $dst_fs } لا يدعم الملفات المتناثرة. تمت كتابة الثقوب الموجودة في المصدر كأصفار، لذا فإن حجم الوجهة على القرص أكبر.  # MT
sparse-warning-densified = تم الحفاظ على تخطيط الملف المتناثر: تم نسخ النطاقات المخصصة فقط.  # MT
sparse-warning-mismatch = عدم تطابق تخطيط الملف المتناثر — قد تكون الوجهة أكبر من المتوقع.  # MT

# Phase 24 — security-metadata preservation. MT-flagged drafts;
# the authoritative English source lives in locales/en/copythat.ftl.
settings-preserve-security-metadata = الحفاظ على بيانات الأمان الوصفية  # MT
settings-preserve-security-metadata-hint = التقاط وإعادة تطبيق تدفقات البيانات الوصفية خارج النطاق (NTFS ADS / xattrs / قوائم ACL / سياقات SELinux / إمكانيات ملف Linux / تفرعات موارد macOS) في كل عملية نسخ.  # MT
settings-preserve-motw = الحفاظ على علامة الويب (تنزيل من الإنترنت)  # MT
settings-preserve-motw-hint = حرج للأمان. يستخدم SmartScreen وOffice Protected View هذا التدفق للتحذير من الملفات التي تم تنزيلها من الإنترنت. تعطيله يسمح للملف القابل للتنفيذ بالتخلص من علامته على نسخة وتجاوز ضمانات نظام التشغيل.  # MT
settings-preserve-posix-acls = الحفاظ على قوائم POSIX ACL والسمات الموسعة  # MT
settings-preserve-posix-acls-hint = نقل سمات user.* / system.* / trusted.* وقوائم التحكم في وصول POSIX عبر النسخ.  # MT
settings-preserve-selinux = الحفاظ على سياقات SELinux  # MT
settings-preserve-selinux-hint = نقل تسمية security.selinux عبر النسخ بحيث يمكن للبرامج التي تعمل ضمن سياسات MAC الوصول إلى الملف.  # MT
settings-preserve-resource-forks = الحفاظ على تفرعات موارد macOS ومعلومات Finder  # MT
settings-preserve-resource-forks-hint = نقل تفرع الموارد القديم وFinderInfo (علامات الألوان، بيانات Carbon الوصفية) عبر النسخ.  # MT
settings-appledouble-fallback = استخدام ملف AppleDouble الجانبي على أنظمة الملفات غير المتوافقة  # MT
meta-translated-to-appledouble = تم تخزين البيانات الوصفية الأجنبية في ملف AppleDouble الجانبي (._{ $ext })  # MT

# Phase 25 — two-way sync with vector-clock conflict detection.
# MT-flagged drafts; the authoritative English source lives in
# locales/en/copythat.ftl.
footer-sync = المزامنة  # MT
sync-drawer-title = مزامنة ثنائية الاتجاه  # MT
sync-drawer-hint = حافظ على تزامن مجلدين دون عمليات استبدال صامتة. تظهر التعديلات المتزامنة كنزاعات يمكنك حلها.  # MT
sync-add-pair = إضافة زوج  # MT
sync-add-cancel = إلغاء  # MT
sync-refresh = تحديث  # MT
sync-add-save = حفظ الزوج  # MT
sync-add-saving = جارٍ الحفظ…  # MT
sync-add-missing-fields = التسمية والمسار الأيسر والمسار الأيمن مطلوبة جميعًا.  # MT
sync-remove-confirm = إزالة زوج المزامنة هذا؟ يتم الاحتفاظ بقاعدة بيانات الحالة؛ لا يتم المساس بالمجلدات.  # MT
sync-field-label = التسمية  # MT
sync-field-label-placeholder = على سبيل المثال: المستندات ↔ NAS  # MT
sync-field-left = المجلد الأيسر  # MT
sync-field-left-placeholder = اختر أو الصق مسارًا مطلقًا  # MT
sync-field-right = المجلد الأيمن  # MT
sync-field-right-placeholder = اختر أو الصق مسارًا مطلقًا  # MT
sync-field-mode = الوضع  # MT
sync-mode-two-way = ثنائي الاتجاه  # MT
sync-mode-mirror-left-to-right = مرآة (يسار → يمين)  # MT
sync-mode-mirror-right-to-left = مرآة (يمين → يسار)  # MT
sync-mode-contribute-left-to-right = مساهمة (يسار → يمين، بدون حذف)  # MT
sync-no-pairs = لم يتم تكوين أي أزواج مزامنة بعد. انقر فوق "إضافة زوج" للبدء.  # MT
sync-loading = تحميل الأزواج المكونة…  # MT
sync-never-run = لم يتم التشغيل  # MT
sync-running = قيد التشغيل  # MT
sync-run-now = تشغيل الآن  # MT
sync-cancel = إلغاء  # MT
sync-remove-pair = إزالة  # MT
sync-view-conflicts = عرض النزاعات ({ $count })  # MT
sync-conflicts-heading = النزاعات  # MT
sync-no-conflicts = لا توجد نزاعات من آخر تشغيل.  # MT
sync-winner = الفائز  # MT
sync-side-left-to-right = يسار  # MT
sync-side-right-to-left = يمين  # MT
sync-conflict-kind-concurrent-write = تعديل متزامن  # MT
sync-conflict-kind-delete-edit = حذف ↔ تعديل  # MT
sync-conflict-kind-add-add = أضاف كلا الجانبين  # MT
sync-conflict-kind-corrupt-equal = تباعد المحتوى دون كتابة جديدة  # MT
sync-resolve-keep-left = الاحتفاظ باليسار  # MT
sync-resolve-keep-right = الاحتفاظ باليمين  # MT
sync-resolve-keep-both = الاحتفاظ بكليهما  # MT
sync-resolve-three-way = حل عبر الدمج الثلاثي  # MT
sync-resolve-phase-53-tooltip = الدمج الثلاثي التفاعلي للملفات غير النصية يأتي في المرحلة 53.  # MT
sync-error-prefix = خطأ في المزامنة  # MT

# Phase 26 — real-time mirror watcher. MT-flagged drafts;
# the authoritative English source lives in locales/en/copythat.ftl.
live-mirror-start = بدء المرآة الحية  # MT
live-mirror-stop = إيقاف المرآة الحية  # MT
live-mirror-watching = مراقبة  # MT
live-mirror-toggle-hint = إعادة المزامنة تلقائيًا عند كل تغيير في نظام الملفات تم اكتشافه. يستخدم خيطًا واحدًا في الخلفية لكل زوج نشط.  # MT
watch-event-prefix = تغيير ملف  # MT
watch-overflow-recovered = تجاوز مخزن المراقب المؤقت؛ إعادة التعداد للاسترداد  # MT

# Phase 27 — content-defined chunk store. MT-flagged drafts;
# the authoritative English source lives in locales/en/copythat.ftl.
chunk-store-section = مخزن الأجزاء  # MT
chunk-store-enable = تمكين مخزن الأجزاء (استئناف التغييرات + إزالة التكرار)  # MT
chunk-store-enable-hint = يقسم كل ملف منسوخ حسب المحتوى (FastCDC) ويخزن الأجزاء بمعالجة المحتوى. تعيد المحاولات كتابة الأجزاء المتغيرة فقط؛ الملفات ذات المحتوى المشترك تزيل التكرار تلقائيًا.  # MT
chunk-store-location = موقع مخزن الأجزاء  # MT
chunk-store-max-size = الحد الأقصى لحجم مخزن الأجزاء  # MT
chunk-store-prune = تنظيف الأجزاء الأقدم من (أيام)  # MT
chunk-store-savings = تم توفير { $gib } جيجابايت عبر إزالة تكرار الأجزاء  # MT
chunk-store-disk-usage = يستخدم { $size } عبر { $chunks } جزءًا  # MT

# Phase 28 — tray-resident Drop Stack. MT-flagged drafts;
# the authoritative English source lives in locales/en/copythat.ftl.
dropstack-window-title = Drop Stack  # MT
dropstack-tray-open = Drop Stack  # MT
dropstack-empty-title = مكدس الإسقاط فارغ  # MT
dropstack-empty-hint = اسحب الملفات هنا من المستكشف أو انقر بزر الماوس الأيمن على صف المهمة لإضافته.  # MT
dropstack-add-to-stack = إضافة إلى مكدس الإسقاط  # MT
dropstack-copy-all-to = نسخ الكل إلى…  # MT
dropstack-move-all-to = نقل الكل إلى…  # MT
dropstack-clear = مسح المكدس  # MT
dropstack-remove-row = إزالة من المكدس  # MT
dropstack-path-missing-toast = تم إسقاط { $path } — الملف لم يعد موجودًا.  # MT
dropstack-always-on-top = إبقاء مكدس الإسقاط دائمًا في المقدمة  # MT
dropstack-show-tray-icon = إظهار أيقونة علبة Copy That  # MT
dropstack-open-on-start = فتح مكدس الإسقاط تلقائيًا عند بدء التطبيق  # MT
dropstack-count = { $count } مسار  # MT

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
