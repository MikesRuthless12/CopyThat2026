app-name = Copy That v1.0.0
# MT
window-title = Copy That v1.0.0
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
# MT — Phase 8 toast messages
toast-error-resolved = Ошибка устранена
# MT
toast-collision-resolved = Конфликт разрешён
# MT
toast-elevated-unavailable = Повтор с повышенными правами появится на этапе 17 — пока недоступно
toast-clipboard-files-detected = Файлы в буфере обмена — нажмите ярлык вставки, чтобы копировать через Copy That
toast-clipboard-no-files = В буфере обмена нет файлов для вставки
# MT
toast-error-log-exported = Журнал ошибок экспортирован

# MT — Error modal
error-modal-title = Передача не удалась
# MT
error-modal-retry = Повторить
# MT
error-modal-retry-elevated = Повторить с повышенными правами
# MT
error-modal-skip = Пропустить
# MT
error-modal-skip-all-kind = Пропустить все ошибки этого типа
# MT
error-modal-abort = Прервать все
# MT
error-modal-path-label = Путь
# MT
error-modal-code-label = Код
error-drawer-pending-count = Ещё ошибки в ожидании
error-drawer-toggle = Свернуть или развернуть

# MT — Error-kind labels
err-not-found = Файл не найден
# MT
err-permission-denied = Доступ запрещён
# MT
err-disk-full = Целевой диск переполнен
# MT
err-interrupted = Операция прервана
# MT
err-verify-failed = Ошибка проверки после копирования
# MT
err-path-escape = Путь отклонён — содержит переходы к родительскому каталогу (..) или недопустимые байты
# MT
err-io-other = Неизвестная ошибка ввода-вывода
err-sparseness-mismatch = Не удалось сохранить разреженную структуру в месте назначения  # MT

# MT — Collision modal
collision-modal-title = Файл уже существует
# MT
collision-modal-overwrite = Заменить
# MT
collision-modal-overwrite-if-newer = Заменить, если новее
# MT
collision-modal-skip = Пропустить
# MT
collision-modal-keep-both = Сохранить оба
# MT
collision-modal-rename = Переименовать…
# MT
collision-modal-apply-to-all = Применить ко всем
# MT
collision-modal-source = Источник
# MT
collision-modal-destination = Назначение
# MT
collision-modal-size = Размер
# MT
collision-modal-modified = Изменён
# MT
collision-modal-hash-check = Быстрый хеш (SHA-256)
# MT
collision-modal-rename-placeholder = Новое имя файла
# MT
collision-modal-confirm-rename = Переименовать

# MT — Error log drawer
error-log-title = Журнал ошибок
# MT
error-log-empty = Ошибки не зарегистрированы
# MT
error-log-export-csv = Экспорт CSV
# MT
error-log-export-txt = Экспорт текста
# MT
error-log-clear = Очистить журнал
# MT
error-log-col-time = Время
# MT
error-log-col-job = Задача
# MT
error-log-col-path = Путь
# MT
error-log-col-code = Код
# MT
error-log-col-message = Сообщение
# MT
error-log-col-resolution = Решение

# MT — History drawer (Phase 9)
history-title = История
# MT
history-empty = Пока задач не зарегистрировано
# MT
history-unavailable = История копирования недоступна. Не удалось открыть хранилище SQLite при запуске.
# MT
history-filter-any = любой
# MT
history-filter-kind = Тип
# MT
history-filter-status = Состояние
# MT
history-filter-text = Поиск
# MT
history-refresh = Обновить
# MT
history-export-csv = Экспорт CSV
# MT
history-purge-30 = Удалить старше 30 дней
# MT
history-rerun = Повторить
# MT
history-detail-open = Подробности
# MT
history-detail-title = Подробности задачи
# MT
history-detail-empty = Нет зарегистрированных элементов
# MT
history-col-date = Дата
# MT
history-col-kind = Тип
# MT
history-col-src = Источник
# MT
history-col-dst = Назначение
# MT
history-col-files = Файлы
# MT
history-col-size = Размер
# MT
history-col-status = Состояние
# MT
history-col-duration = Длительность
# MT
history-col-error = Ошибка

# MT
toast-history-exported = История экспортирована
# MT
toast-history-rerun-queued = Повтор поставлен в очередь

# MT — Totals drawer (Phase 10)
footer-totals = Итого
# MT
totals-title = Итого
# MT
totals-loading = Загрузка итогов…
# MT
totals-card-bytes = Всего скопировано байт
# MT
totals-card-files = Файлы
# MT
totals-card-jobs = Задачи
# MT
totals-card-avg-rate = Средняя пропускная способность
# MT
totals-errors = ошибки
# MT
totals-spark-title = Последние 30 дней
# MT
totals-kinds-title = По типу
# MT
totals-saved-title = Сэкономленное время (оценка)
# MT
totals-saved-note = Оценка по сравнению с эталонным копированием той же нагрузки стандартным файловым менеджером.
# MT
totals-reset = Сбросить статистику
# MT
totals-reset-confirm = Это удалит все сохранённые задачи и элементы. Продолжить?
# MT
totals-reset-confirm-yes = Да, сбросить
# MT
toast-totals-reset = Статистика сброшена

# MT — Phase 11a additions
header-language-label = Язык
# MT
header-language-title = Изменить язык

# MT
kind-copy = Копировать
# MT
kind-move = Переместить
# MT
kind-delete = Удалить
# MT
kind-secure-delete = Безопасное удаление

# MT
status-running = Выполняется
# MT
status-succeeded = Успешно
# MT
status-failed = Не удалось
# MT
status-cancelled = Отменено
# MT
status-ok = ОК
# MT
status-skipped = Пропущено

# MT
history-search-placeholder = /путь
# MT
toast-history-purged = Удалено { $count } задач старше 30 дней

# MT
err-source-required = Требуется хотя бы один путь источника.
# MT
err-destination-empty = Путь назначения пуст.
# MT
err-source-empty = Путь источника пуст.

# MT
duration-lt-1s = < 1 с
# MT
duration-ms = { $ms } мс
# MT
duration-seconds = { $s } с
# MT
duration-minutes-seconds = { $m } мин { $s } с
# MT
duration-hours-minutes = { $h } ч { $m } мин
# MT
duration-zero = 0 с

# MT
rate-unit-per-second = { $size }/с

# MT — Phase 11b Settings modal
settings-title = Настройки
# MT
settings-tab-general = Общие
# MT
settings-tab-appearance = Внешний вид
# MT
settings-section-language = Язык
# MT
settings-phase-12-hint = Другие настройки (тема, параметры передачи по умолчанию, алгоритм проверки, профили) появятся на этапе 12.

# MT — Phase 12 Settings window
settings-loading = Загрузка настроек…
# MT
settings-tab-transfer = Передача
# MT
settings-tab-shell = Оболочка
# MT
settings-tab-secure-delete = Безопасное удаление
# MT
settings-tab-advanced = Дополнительно
# MT
settings-tab-profiles = Профили

# MT
settings-section-theme = Тема
# MT
settings-theme-auto = Автоматически
# MT
settings-theme-light = Светлая
# MT
settings-theme-dark = Тёмная
# MT
settings-start-with-os = Запускать при старте системы
# MT
settings-single-instance = Один запущенный экземпляр
# MT
settings-minimize-to-tray = Сворачивать в трей при закрытии
settings-error-display-mode = Стиль уведомления об ошибке
settings-error-display-modal = Модальное окно (блокирует приложение)
settings-error-display-drawer = Выдвижная панель (не блокирует)
settings-error-display-mode-hint = Модальное окно останавливает очередь, пока вы не решите. Выдвижная панель оставляет очередь работать и позволяет разбирать ошибки в углу.
settings-paste-shortcut = Вставка файлов через глобальное сочетание
settings-paste-shortcut-combo = Сочетание клавиш
settings-paste-shortcut-hint = Нажмите эту комбинацию в любом месте системы, чтобы вставить файлы, скопированные из Проводника / Finder / Файлы через Copy That. CmdOrCtrl — это Cmd на macOS и Ctrl на Windows / Linux.
settings-clipboard-watcher = Следить за буфером обмена для скопированных файлов
settings-clipboard-watcher-hint = Показывает уведомление, когда в буфере обмена появляются URL файлов, подсказывая, что можно вставить через Copy That. Опрашивает каждые 500 мс при включении.

# MT
settings-buffer-size = Размер буфера
# MT
settings-verify = Проверять после копирования
# MT
settings-verify-off = Выключено
# MT
settings-concurrency = Параллельность
# MT
settings-concurrency-auto = Авто
# MT
settings-reflink = Reflink / быстрые пути
# MT
settings-reflink-prefer = Предпочитать
# MT
settings-reflink-avoid = Избегать reflink
# MT
settings-reflink-disabled = Всегда использовать async-движок
# MT
settings-fsync-on-close = Синхронизировать на диск при закрытии (медленнее, надёжнее)
# MT
settings-preserve-timestamps = Сохранять временные метки
# MT
settings-preserve-permissions = Сохранять права доступа
# MT
settings-preserve-acls = Сохранять ACL (этап 14)
settings-preserve-sparseness = Сохранять разреженные файлы  # MT
settings-preserve-sparseness-hint = Копировать только выделенные области разреженных файлов (диски виртуальных машин, файлы баз данных), чтобы размер на диске в месте назначения оставался таким же, как у источника.  # MT

# MT
settings-context-menu = Включить пункты контекстного меню оболочки
# MT
settings-intercept-copy = Перехватывать стандартный обработчик копирования (Windows)
# MT
settings-intercept-copy-hint = Когда включено, Ctrl+C / Ctrl+V в Проводнике идёт через Copy That. Регистрация на этапе 14.
# MT
settings-notify-completion = Уведомлять по завершении задания

# MT
settings-shred-method = Метод затирания по умолчанию
# MT
settings-shred-zero = Нули (1 проход)
# MT
settings-shred-random = Случайные (1 проход)
# MT
settings-shred-dod3 = DoD 5220.22-M (3 прохода)
# MT
settings-shred-dod7 = DoD 5220.22-M (7 проходов)
# MT
settings-shred-gutmann = Гуттманна (35 проходов)
# MT
settings-shred-nist = NIST 800-88
# MT
settings-shred-confirm-twice = Требовать двойное подтверждение перед затиранием

# MT
settings-log-level = Уровень журнала
# MT
settings-log-off = Выключено
# MT
settings-telemetry = Телеметрия
# MT
settings-telemetry-never = Никогда — данные не отправляются ни на одном уровне
# MT
settings-error-policy = Политика ошибок по умолчанию
# MT
settings-error-policy-ask = Спрашивать
# MT
settings-error-policy-skip = Пропускать
# MT
settings-error-policy-retry = Повтор с задержкой
# MT
settings-error-policy-abort = Прервать при первой ошибке
# MT
settings-history-retention = Хранение истории (дней)
# MT
settings-history-retention-hint = 0 = хранить всегда. Любое другое значение автоматически удаляет старые задания при запуске.
# MT
settings-database-path = Путь к базе данных
# MT
settings-database-path-default = (по умолчанию — каталог данных ОС)
# MT
settings-reset-all = Сбросить по умолчанию
# MT
settings-reset-confirm = Сбросить все настройки к значениям по умолчанию? Профили не изменятся.

# MT
settings-profiles-hint = Сохраняйте текущие настройки под именем; позже загружайте их, чтобы переключаться без настройки каждого параметра.
# MT
settings-profile-name-placeholder = Имя профиля
# MT
settings-profile-save = Сохранить
# MT
settings-profile-import = Импорт…
# MT
settings-profile-load = Загрузить
# MT
settings-profile-export = Экспорт…
# MT
settings-profile-delete = Удалить
# MT
settings-profile-empty = Сохранённых профилей нет.
# MT
settings-profile-import-prompt = Имя для импортируемого профиля:

# MT
toast-settings-reset = Настройки сброшены
# MT
toast-profile-saved = Профиль сохранён
# MT
toast-profile-loaded = Профиль загружен
# MT
toast-profile-exported = Профиль экспортирован
# MT
toast-profile-imported = Профиль импортирован

# Phase 13d — activity feed + header picker buttons
action-add-files = Добавить файлы
action-add-folders = Добавить папки
activity-title = Активность
activity-clear = Очистить список активности
activity-empty = Файловой активности пока нет.
activity-after-done = По завершении:
activity-keep-open = Оставить приложение открытым
activity-close-app = Закрыть приложение
activity-shutdown = Выключить ПК
activity-logoff = Выйти из системы
activity-sleep = Спящий режим

# Phase 14 — preflight free-space dialog
preflight-block-title = На целевом томе недостаточно места
preflight-warn-title = Мало места на целевом томе
preflight-unknown-title = Не удалось определить свободное место
preflight-unknown-body = Источник слишком велик для быстрого измерения, или целевой том не ответил. Вы можете продолжить; защита движка корректно остановит копирование, если место закончится.
preflight-required = Требуется
preflight-free = Свободно
preflight-reserve = Резерв
preflight-shortfall = Недостача
preflight-continue = Всё равно продолжить
collision-modal-overwrite-older = Перезаписать только более старые

# Phase 14e — subset picker
preflight-pick-subset = Выбрать, что копировать…
subset-title = Выберите источники для копирования
subset-subtitle = Полный набор не помещается в пункте назначения. Отметьте то, что хотите скопировать; остальное останется.
subset-loading = Измерение размеров…
subset-too-large = слишком велико для подсчёта
subset-budget = Доступно
subset-remaining = Осталось
subset-confirm = Копировать выбор
history-rerun-hint = Повторить эту копию — заново сканирует все файлы в дереве источника
history-clear-all = Очистить всё
history-clear-all-confirm = Нажмите ещё раз для подтверждения
history-clear-all-hint = Удаляет все строки истории. Требуется второе нажатие для подтверждения.
toast-history-cleared = История очищена ({ $count } строк удалено)

# Phase 15 — source-list ordering
drop-dialog-sort-label = Порядок:
sort-custom = Свой
sort-name-asc = Имя А → Я (файлы сначала)
sort-name-desc = Имя Я → А (файлы сначала)
sort-size-asc = Размер от меньшего (файлы сначала)
sort-size-desc = Размер от большего (файлы сначала)
sort-reorder = Переупорядочить
sort-move-top = В начало
sort-move-up = Вверх
sort-move-down = Вниз
sort-move-bottom = В конец
sort-name-asc-simple = Имя А → Я
sort-name-desc-simple = Имя Я → А
sort-size-asc-simple = Меньшие первыми
sort-size-desc-simple = Большие первыми
activity-sort-locked = Сортировка отключена во время копирования. Приостановите или дождитесь окончания, затем измените порядок.
drop-dialog-collision-label = Если файл уже существует:
collision-policy-keep-both = Сохранить оба (переименовать новую копию в _2, _3 …)
collision-policy-skip = Пропустить новую копию
collision-policy-overwrite = Перезаписать существующий файл
collision-policy-overwrite-if-newer = Перезаписывать только если новее
collision-policy-prompt = Спрашивать каждый раз
drop-dialog-busy-checking = Проверка свободного места…
drop-dialog-busy-enumerating = Подсчёт файлов…
drop-dialog-busy-starting = Запуск копирования…
toast-enumeration-deferred = Исходное дерево большое — предварительный список пропущен; строки появятся по мере работы движка.

# Phase 14a — enumeration-time filters
# MT
settings-tab-filters = Фильтры
# MT
settings-filters-hint = Пропускает файлы на этапе перечисления — движок их даже не открывает. Включения действуют только на файлы; исключения также обрезают совпадающие папки.
# MT
settings-filters-enabled = Включить фильтры для копирования дерева
# MT
settings-filters-include-globs = Шаблоны включения
# MT
settings-filters-include-globs-placeholder = **/*.txt
# MT
settings-filters-include-globs-hint = По одному шаблону на строку. Если непусто, файл должен совпасть хотя бы с одним. Папки всегда обходятся.
# MT
settings-filters-exclude-globs = Шаблоны исключения
# MT
settings-filters-exclude-globs-placeholder = **/node_modules
# MT
settings-filters-exclude-globs-hint = По одному шаблону на строку. Совпадения обрезают всё поддерево для папок; совпадающие файлы пропускаются.
# MT
settings-filters-size-range = Диапазон размера файла
# MT
settings-filters-min-size-bytes = Мин. размер (байт, пусто = без нижнего предела)
# MT
settings-filters-max-size-bytes = Макс. размер (байт, пусто = без верхнего предела)
# MT
settings-filters-date-range = Диапазон времени изменения
# MT
settings-filters-min-mtime = Изменено не раньше
# MT
settings-filters-max-mtime = Изменено не позже
# MT
settings-filters-attributes = Атрибуты
# MT
settings-filters-skip-hidden = Пропускать скрытые файлы / папки
# MT
settings-filters-skip-system = Пропускать системные файлы (только Windows)
# MT
settings-filters-skip-readonly = Пропускать файлы только для чтения

# Phase 15 — auto-update
# MT
settings-tab-updater = Обновления
# MT
settings-updater-hint = Copy That проверяет подписанные обновления не чаще одного раза в день. Обновления устанавливаются при следующем выходе из приложения.
# MT
settings-updater-auto-check = Проверять обновления при запуске
# MT
settings-updater-channel = Канал выпуска
# MT
settings-updater-channel-stable = Стабильный
# MT
settings-updater-channel-beta = Бета (предварительный)
# MT
settings-updater-last-check = Последняя проверка
# MT
settings-updater-last-never = Никогда
# MT
settings-updater-check-now = Проверить обновления сейчас
# MT
settings-updater-checking = Проверка…
# MT
settings-updater-available = Доступно обновление
# MT
settings-updater-up-to-date = Вы используете последнюю версию.
# MT
settings-updater-dismiss = Пропустить эту версию
# MT
settings-updater-dismissed = Пропущено
# MT
toast-update-available = Доступна новая версия
# MT
toast-update-up-to-date = У вас уже последняя версия

# Phase 19a — disk-backed file enumeration (TeraCopy-compatible scan DB)
# MT
scan-progress-title = Сканирование…
# MT
scan-progress-stats = { $files } файлов · { $bytes } на данный момент
# MT
scan-pause-button = Приостановить сканирование
# MT
scan-resume-button = Возобновить сканирование
# MT
scan-cancel-button = Отменить сканирование
# MT
scan-cancel-confirm = Отменить сканирование и отбросить прогресс?
# MT
scan-db-header = База данных сканирования
# MT
scan-db-hint = База данных сканирования на диске для задач с миллионами файлов.
# MT
advanced-scan-hash-during = Вычислять контрольные суммы во время сканирования
# MT
advanced-scan-db-path = Расположение базы данных сканирования
# MT
advanced-scan-retention-days = Автоматически удалять завершённые сканирования через (дней)
# MT
advanced-scan-max-keep = Максимальное число сохраняемых баз данных сканирования

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
sparse-not-supported-title = Место назначения заполняет разреженные файлы  # MT
sparse-not-supported-body = { $dst_fs } не поддерживает разреженные файлы. Пропуски в источнике были записаны нулями, поэтому место назначения занимает больше места на диске.  # MT
sparse-warning-densified = Разреженная структура сохранена: скопированы только выделенные области.  # MT
sparse-warning-mismatch = Несоответствие разреженной структуры — место назначения может быть больше ожидаемого.  # MT
