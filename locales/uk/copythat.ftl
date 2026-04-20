app-name = Copy That 2026
# MT
window-title = Copy That 2026
# MT
shred-ssd-advisory = Увага: ціль розташована на SSD. Багатопрохідне перезаписування не надійно очищує флеш-пам'ять, оскільки вирівнювання зносу та надлишкове виділення пересувають дані поза межі логічної адреси блоку. Для твердотільних носіїв слід віддавати перевагу ATA SECURE ERASE, NVMe Format із безпечним стиранням або повному шифруванню диска з подальшим знищенням ключа.

# MT
state-idle = Простій
# MT
state-copying = Копіювання
# MT
state-verifying = Перевірка
# MT
state-paused = Призупинено
# MT
state-error = Помилка

# MT
state-pending = У черзі
# MT
state-running = Виконується
# MT
state-cancelled = Скасовано
# MT
state-succeeded = Готово
# MT
state-failed = Невдача

# MT
action-pause = Призупинити
# MT
action-resume = Відновити
# MT
action-cancel = Скасувати
# MT
action-pause-all = Призупинити всі завдання
# MT
action-resume-all = Відновити всі завдання
# MT
action-cancel-all = Скасувати всі завдання
# MT
action-close = Закрити
# MT
action-reveal = Показати в теці

# MT
menu-pause = Призупинити
# MT
menu-resume = Відновити
# MT
menu-cancel = Скасувати
# MT
menu-remove = Видалити з черги
# MT
menu-reveal-source = Показати джерело в теці
# MT
menu-reveal-destination = Показати призначення в теці

# MT
header-eta-label = Орієнтовний час, що залишився
# MT
header-toolbar-label = Загальні елементи керування

# MT
footer-queued = активних завдань
# MT
footer-total-bytes = у роботі
# MT
footer-errors = помилок
# MT
footer-history = Історія

# MT
empty-title = Перетягніть файли або теки для копіювання
# MT
empty-hint = Перетягніть елементи у вікно. Ми запитаємо про призначення та додамо по одному завданню на джерело.
# MT
empty-region-label = Список завдань

# MT
details-drawer-label = Подробиці завдання
# MT
details-source = Джерело
# MT
details-destination = Призначення
# MT
details-state = Стан
# MT
details-bytes = Байти
# MT
details-files = Файли
# MT
details-speed = Швидкість
# MT
details-eta = Залишилося
# MT
details-error = Помилка

# MT
drop-dialog-title = Передати перетягнуті елементи
# MT
drop-dialog-subtitle = { $count } елемент(ів) готовий(і) до передавання. Виберіть теку призначення, щоб почати.
# MT
drop-dialog-mode = Операція
# MT
drop-dialog-copy = Копіювати
# MT
drop-dialog-move = Перемістити
# MT
drop-dialog-pick-destination = Вибрати призначення
# MT
drop-dialog-change-destination = Змінити призначення
# MT
drop-dialog-start-copy = Почати копіювання
# MT
drop-dialog-start-move = Почати переміщення

# MT
eta-calculating = обчислення…
# MT
eta-unknown = невідомо

# MT
toast-job-done = Передавання завершено
# MT
toast-copy-queued = Копіювання додано до черги
# MT
toast-move-queued = Переміщення додано до черги
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
