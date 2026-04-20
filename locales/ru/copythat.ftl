app-name = Copy That 2026
# MT
window-title = Copy That 2026
# MT
shred-ssd-advisory = Внимание: целевой объект находится на SSD. Многократная перезапись ненадёжно очищает флеш-память, поскольку выравнивание износа и резервная ёмкость смещают данные относительно логического адреса блока. Для твердотельных накопителей предпочтительнее использовать ATA SECURE ERASE, NVMe Format с защищённым стиранием или полнодисковое шифрование с последующим уничтожением ключа.

# MT
state-idle = Простой
# MT
state-copying = Копирование
# MT
state-verifying = Проверка
# MT
state-paused = Приостановлено
# MT
state-error = Ошибка

# MT
state-pending = В очереди
# MT
state-running = Выполняется
# MT
state-cancelled = Отменено
# MT
state-succeeded = Готово
# MT
state-failed = Сбой

# MT
action-pause = Приостановить
# MT
action-resume = Возобновить
# MT
action-cancel = Отменить
# MT
action-pause-all = Приостановить все задачи
# MT
action-resume-all = Возобновить все задачи
# MT
action-cancel-all = Отменить все задачи
# MT
action-close = Закрыть
# MT
action-reveal = Показать в папке

# MT
menu-pause = Приостановить
# MT
menu-resume = Возобновить
# MT
menu-cancel = Отменить
# MT
menu-remove = Удалить из очереди
# MT
menu-reveal-source = Показать источник в папке
# MT
menu-reveal-destination = Показать назначение в папке

# MT
header-eta-label = Оставшееся время
# MT
header-toolbar-label = Общие элементы управления

# MT
footer-queued = активных задач
# MT
footer-total-bytes = в работе
# MT
footer-errors = ошибок
# MT
footer-history = История

# MT
empty-title = Перетащите файлы или папки для копирования
# MT
empty-hint = Перетащите элементы в окно. Мы спросим пункт назначения и добавим по одной задаче на каждый источник.
# MT
empty-region-label = Список задач

# MT
details-drawer-label = Сведения о задаче
# MT
details-source = Источник
# MT
details-destination = Назначение
# MT
details-state = Состояние
# MT
details-bytes = Байты
# MT
details-files = Файлы
# MT
details-speed = Скорость
# MT
details-eta = Оставшееся время
# MT
details-error = Ошибка

# MT
drop-dialog-title = Передать перетащенные элементы
# MT
drop-dialog-subtitle = { $count } элемент(ов) готов(ы) к передаче. Выберите папку назначения, чтобы начать.
# MT
drop-dialog-mode = Операция
# MT
drop-dialog-copy = Копировать
# MT
drop-dialog-move = Переместить
# MT
drop-dialog-pick-destination = Выбрать назначение
# MT
drop-dialog-change-destination = Изменить назначение
# MT
drop-dialog-start-copy = Начать копирование
# MT
drop-dialog-start-move = Начать перемещение

# MT
eta-calculating = вычисление…
# MT
eta-unknown = неизвестно

# MT
toast-job-done = Передача завершена
# MT
toast-copy-queued = Копирование поставлено в очередь
# MT
toast-move-queued = Перемещение поставлено в очередь
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
