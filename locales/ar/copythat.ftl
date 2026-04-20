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
