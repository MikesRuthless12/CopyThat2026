app-name = Copy That 2026
# MT
window-title = Copy That 2026
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
err-io-other = خطأ إدخال/إخراج غير معروف

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
