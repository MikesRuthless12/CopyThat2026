app-name = Copy That v0.19.84
window-title = Copy That v0.19.84
shred-ssd-advisory = Внимание: этот объект расположен на SSD. Многопроходная перезапись не обеспечивает надёжную очистку флеш-памяти, поскольку выравнивание износа и резервирование перемещают данные из-под логического адреса блока. Для твердотельных накопителей используйте ATA SECURE ERASE, NVMe Format с безопасным стиранием или полнодисковое шифрование с уничтожением ключа.

# Global aggregate states (header pill)
state-idle = Простой
state-copying = Копирование
state-verifying = Проверка
state-paused = Приостановлено
state-error = Ошибка

# Per-job states (row badge)
state-pending = В очереди
state-running = Выполняется
state-cancelled = Отменено
state-succeeded = Готово
state-failed = Сбой

# Actions
action-pause = Пауза
action-resume = Продолжить
action-cancel = Отмена
action-pause-all = Приостановить все задания
action-resume-all = Возобновить все задания
action-cancel-all = Отменить все задания
action-close = Закрыть
action-reveal = Показать в папке
action-add-files = Добавить файлы
action-add-folders = Добавить папки

# Phase 13d — activity feed
activity-title = Активность
activity-clear = Очистить список активности
activity-empty = Пока нет активности по файлам.
activity-after-done = По завершении:
activity-keep-open = Оставить приложение открытым
activity-close-app = Закрыть приложение
activity-shutdown = Выключить ПК
activity-logoff = Выйти из системы
activity-sleep = Спящий режим

# Phase 14 — preflight free-space dialog
preflight-block-title = Недостаточно места в месте назначения
preflight-warn-title = Мало места в месте назначения
preflight-unknown-title = Не удалось определить свободное место
preflight-unknown-body = Источник слишком велик для быстрого подсчёта размера, либо том назначения не ответил. Можно продолжить — защита по свободному месту корректно остановит копирование, если место закончится.
preflight-required = Требуется
preflight-free = Свободно
preflight-reserve = Резерв
preflight-shortfall = Нехватка
preflight-continue = Всё равно продолжить
preflight-pick-subset = Выбрать, что копировать…
collision-modal-overwrite-older = Перезаписывать только старые

# Phase 14e — subset picker
subset-title = Выберите источники для копирования
subset-subtitle = Полный набор не помещается в месте назначения. Отметьте элементы для копирования; остальные останутся на месте.
subset-loading = Измерение размеров…
subset-too-large = слишком велик для подсчёта
subset-budget = Доступно
subset-remaining = Осталось
subset-confirm = Копировать выбранное
history-rerun-hint = Повторить копирование — повторно сканирует каждый файл в дереве источника
history-clear-all = Очистить всё
history-clear-all-confirm = Нажмите ещё раз для подтверждения
history-clear-all-hint = Удаляет все записи истории. Требуется повторное нажатие для подтверждения.
toast-history-cleared = История очищена (удалено записей: { $count })

# Phase 15 — source-list ordering in the drop staging dialog
drop-dialog-sort-label = Порядок:
sort-custom = Произвольный
sort-name-asc = Имя А → Я (сначала файлы)
sort-name-desc = Имя Я → А (сначала файлы)
sort-size-asc = Сначала меньшие (сначала файлы)
sort-size-desc = Сначала большие (сначала файлы)
sort-reorder = Изменить порядок
sort-move-top = В начало
sort-move-up = Вверх
sort-move-down = Вниз
sort-move-bottom = В конец

# Phase 16 — sort preset names for the Activity list (shorter
# wording than the DropStagingDialog variants; the Activity list
# is files-only so the "(files first)" suffix isn't relevant).
sort-name-asc-simple = Имя А → Я
sort-name-desc-simple = Имя Я → А
sort-size-asc-simple = Сначала меньшие
sort-size-desc-simple = Сначала большие
activity-sort-locked = Сортировка отключена во время копирования. Приостановите или дождитесь завершения, затем измените порядок.

# Phase 17 — collision-policy picker in the DropStagingDialog
drop-dialog-collision-label = Если файл уже существует:
collision-policy-keep-both = Сохранить оба (переименовать новую копию в _2, _3, …)
collision-policy-skip = Пропустить новую копию
collision-policy-overwrite = Перезаписать существующий файл
collision-policy-overwrite-if-newer = Перезаписывать только если новее
collision-policy-prompt = Спрашивать каждый раз

# Phase 18 — progress labels on the DropStagingDialog Start button
drop-dialog-busy-checking = Проверка свободного места…
drop-dialog-busy-enumerating = Подсчёт файлов…
drop-dialog-busy-starting = Запуск копирования…
toast-enumeration-deferred = Дерево источника велико — предварительный список файлов пропущен; строки будут появляться по мере обработки.

# Context menu (per-row right-click)
menu-pause = Пауза
menu-resume = Продолжить
menu-cancel = Отмена
menu-remove = Удалить из очереди
menu-reveal-source = Показать источник в папке
menu-reveal-destination = Показать назначение в папке

# Header / toolbar
header-eta-label = Оставшееся время
header-toolbar-label = Глобальные элементы управления

# Footer
footer-queued = активных заданий
footer-total-bytes = в процессе
footer-errors = ошибок
footer-history = История

# Empty state
empty-title = Перетащите файлы или папки для копирования
empty-hint = Перетащите элементы в окно. Мы запросим место назначения и создадим по одному заданию на каждый источник.
empty-region-label = Список заданий

# Details drawer
details-drawer-label = Сведения о задании
details-source = Источник
details-destination = Назначение
details-state = Состояние
details-bytes = Байты
details-files = Файлы
details-speed = Скорость
details-eta = Осталось
details-error = Ошибка

# Drop dialog
drop-dialog-title = Передача перетащенных элементов
drop-dialog-subtitle = Готово к передаче элементов: { $count }. Выберите папку назначения, чтобы начать.
drop-dialog-mode = Операция
drop-dialog-copy = Копировать
drop-dialog-move = Переместить
drop-dialog-pick-destination = Выбрать назначение
drop-dialog-change-destination = Изменить назначение
drop-dialog-start-copy = Начать копирование
drop-dialog-start-move = Начать перемещение

# ETA placeholders
eta-calculating = вычисление…
eta-unknown = неизвестно

# Toast messages
toast-job-done = Передача завершена
toast-copy-queued = Копирование добавлено в очередь
toast-move-queued = Перемещение добавлено в очередь
toast-error-resolved = Ошибка устранена
toast-collision-resolved = Конфликт разрешён
toast-elevated-unavailable = Повтор с повышенными правами появится в Phase 17 — пока недоступно
toast-clipboard-files-detected = Файлы в буфере обмена — нажмите сочетание вставки, чтобы скопировать через Copy That
toast-clipboard-no-files = В буфере обмена нет файлов для вставки
toast-error-log-exported = Журнал ошибок экспортирован

# Error modal (Phase 8)
error-modal-title = Сбой передачи
error-modal-retry = Повторить
error-modal-retry-elevated = Повторить с повышенными правами
error-modal-skip = Пропустить
error-modal-skip-all-kind = Пропускать все ошибки этого типа
error-modal-abort = Прервать всё
error-modal-path-label = Путь
error-modal-code-label = Код
error-drawer-pending-count = Ещё ошибки ожидают
error-drawer-toggle = Свернуть или развернуть

# Error-kind labels (Phase 8). Source of truth — engine maps each
# `CopyErrorKind` to one of these keys via `localized_key()`.
err-not-found = Файл не найден
err-permission-denied = Доступ запрещён
err-disk-full = Диск назначения заполнен
err-interrupted = Операция прервана
err-verify-failed = Проверка после копирования не пройдена
err-path-escape = Путь отклонён — содержит сегменты родительского каталога (..) или недопустимые байты
err-path-invalid-encoding = Путь отклонён — строка содержит недопустимый UTF-8 / символы замены
err-helper-invalid-json = Привилегированный помощник получил некорректный JSON; запрос проигнорирован
err-helper-grant-out-of-band = GrantCapabilities должен обрабатываться циклом выполнения помощника, а не stateless-обработчиком
err-randomness-unavailable = Сбой генератора случайных чисел ОС; не удаётся создать идентификатор сессии
err-sparseness-mismatch = Не удалось сохранить разреженную структуру в месте назначения
err-io-other = Неизвестная ошибка ввода-вывода

# Collision modal (Phase 8)
collision-modal-title = Файл уже существует
collision-modal-overwrite = Перезаписать
collision-modal-overwrite-if-newer = Перезаписать если новее
collision-modal-skip = Пропустить
collision-modal-keep-both = Сохранить оба
collision-modal-rename = Переименовать…
collision-modal-apply-to-all = Применить ко всем
collision-modal-source = Источник
collision-modal-destination = Назначение
collision-modal-size = Размер
collision-modal-modified = Изменён
collision-modal-hash-check = Быстрый хеш (SHA-256)
collision-modal-hash-computing = Вычисление…
collision-modal-hash-identical = Идентичны
collision-modal-hash-different = Различаются
collision-modal-rename-placeholder = Новое имя файла
collision-modal-confirm-rename = Переименовать

# Error log drawer (Phase 8)
error-log-title = Журнал ошибок
error-log-empty = Ошибок не зафиксировано
error-log-export-csv = Экспорт CSV
error-log-export-txt = Экспорт в текст
error-log-clear = Очистить журнал
error-log-col-time = Время
error-log-col-job = Задание
error-log-col-path = Путь
error-log-col-code = Код
error-log-col-message = Сообщение
error-log-col-resolution = Решение

# History drawer (Phase 9)
history-title = История
history-empty = Заданий пока не записано
history-unavailable = История копирования недоступна. Приложению не удалось открыть хранилище SQLite при запуске.
history-filter-any = любой
history-filter-kind = Тип
history-filter-status = Статус
history-filter-text = Поиск
history-refresh = Обновить
history-export-csv = Экспорт CSV
history-purge-30 = Удалить старше 30 дней
history-rerun = Повторить
history-detail-open = Сведения
history-detail-title = Сведения о задании
history-detail-empty = Элементы не записаны
history-col-date = Дата
history-col-kind = Тип
history-col-src = Источник
history-col-dst = Назначение
history-col-files = Файлы
history-col-size = Размер
history-col-status = Статус
history-col-duration = Длительность
history-col-error = Ошибка
toast-history-exported = История экспортирована
toast-history-rerun-queued = Повтор добавлен в очередь

# Totals drawer (Phase 10)
footer-totals = Итоги
totals-title = Итоги
totals-loading = Загрузка итогов…
totals-card-bytes = Всего скопировано байт
totals-card-files = Файлы
totals-card-jobs = Задания
totals-card-avg-rate = Средняя пропускная способность
totals-errors = ошибок
totals-spark-title = Последние 30 дней
totals-kinds-title = По типу
totals-saved-title = Сэкономлено времени (оценка)
totals-saved-note = Оценка относительно базового копирования тем же файловым менеджером для той же нагрузки.
totals-reset = Сбросить статистику
totals-reset-confirm = Это удалит все сохранённые задания и элементы. Продолжить?
totals-reset-confirm-yes = Да, сбросить
toast-totals-reset = Статистика сброшена

# Phase 11a — i18n core: surface remaining user-visible strings so
# every string on a main-window screen flows through Fluent.

# Header language switcher (temporary placement — folds into
# Settings → General in Phase 12).
header-language-label = Язык
header-language-title = Сменить язык

# Job-kind labels. History filter, Totals breakdown, and history
# rows all map wire-format `kind` strings to these labels.
kind-copy = Копирование
kind-move = Перемещение
kind-delete = Удаление
kind-secure-delete = Безопасное удаление

# History status labels. Distinct from `state-*` because the
# history wire format uses plain `running`/`succeeded`/... rather
# than the live `JobState` enum the queue exposes.
status-running = Выполняется
status-succeeded = Успешно
status-failed = Сбой
status-cancelled = Отменено
# Per-item status (not job-level): `ok` / `skipped` are only
# reachable from the history detail view.
status-ok = OK
status-skipped = Пропущено

# History drawer: search field placeholder + purge toast.
history-search-placeholder = /путь
toast-history-purged = Удалено заданий старше 30 дней: { $count }

# User-facing command-layer validation errors. The Rust side
# returns these keys when input is missing; the toast layer looks
# them up rather than showing the raw English.
err-source-required = Требуется хотя бы один путь источника.
err-destination-empty = Путь назначения пуст.
err-source-empty = Путь источника пуст.

# Localised duration formatting for the Totals drawer and ETA
# fields. `{ $ms }`, `{ $s }`, `{ $m }`, `{ $h }` are integer
# placeables — the formatter passes pre-computed values in.
duration-lt-1s = < 1 с
duration-ms = { $ms } мс
duration-seconds = { $s } с
duration-minutes-seconds = { $m } мин { $s } с
duration-hours-minutes = { $h } ч { $m } мин
duration-zero = 0 с

# Rate unit. Appended to a formatted byte size. Some languages
# render this with a leading space ("Ko/s"); keep it translatable
# even though the SI-derived "/s" is near-universal.
rate-unit-per-second = { $size }/с

# Phase 11b — Settings modal skeleton. Phase 12 expanded this into a
# full six-tab preferences window; the `settings-phase-12-hint` key
# is retired in favour of `settings-tab-profiles` + concrete labels.
settings-title = Настройки
settings-tab-general = Общие
settings-tab-appearance = Оформление
settings-section-language = Язык
settings-phase-12-hint = Дополнительные настройки (тема, параметры передачи по умолчанию, алгоритм проверки, профили) появятся в Phase 12.

# Phase 12 — full Settings window keys. Grouped by tab so the
# human-review pass can pick one section at a time. Every field
# label and every option label that the user will actually read
# has its own key; dynamic wire values (enum discriminants like
# "auto" / "prefer") stay untranslated on the wire.

settings-loading = Загрузка настроек…
settings-tab-transfer = Передача
settings-tab-filters = Фильтры
settings-tab-shell = Оболочка
settings-tab-secure-delete = Безопасное удаление
settings-tab-advanced = Дополнительно
settings-tab-updater = Обновления
settings-tab-profiles = Профили

# General tab additions
settings-section-theme = Тема
settings-theme-auto = Авто
settings-theme-light = Светлая
settings-theme-dark = Тёмная
settings-start-with-os = Запускать при старте системы
settings-single-instance = Один работающий экземпляр
settings-minimize-to-tray = Сворачивать в трей при закрытии
settings-error-display-mode = Стиль запроса об ошибках
settings-error-display-modal = Модальное окно (блокирует приложение)
settings-error-display-drawer = Панель (не блокирует)
settings-error-display-mode-hint = Модальное окно останавливает очередь до вашего решения. Панель не останавливает очередь и позволяет разбирать ошибки в углу.
settings-paste-shortcut = Вставлять файлы по глобальному сочетанию
settings-paste-shortcut-combo = Сочетание клавиш
settings-paste-shortcut-hint = Нажмите это сочетание в любом месте системы, чтобы вставить файлы, скопированные из Explorer / Finder / Files, через Copy That. CmdOrCtrl соответствует Cmd на macOS и Ctrl на Windows / Linux.
settings-clipboard-watcher = Отслеживать скопированные файлы в буфере обмена
settings-clipboard-watcher-hint = Показывать уведомление, когда в буфере обмена появляются URL файлов, подсказывая, что их можно вставить через Copy That. Опрос каждые 500 мс, когда включено.

# Transfer tab
settings-buffer-size = Размер буфера
settings-verify = Проверять после копирования
settings-verify-off = Выкл.
settings-concurrency = Параллелизм
settings-concurrency-auto = Авто
settings-reflink = Reflink / быстрые пути
settings-reflink-prefer = Предпочитать
settings-reflink-avoid = Избегать reflink
settings-reflink-disabled = Всегда использовать асинхронный движок
settings-fsync-on-close = Сбрасывать на диск при закрытии (медленнее, надёжнее)
settings-preserve-timestamps = Сохранять временные метки
settings-preserve-permissions = Сохранять разрешения
settings-preserve-acls = Сохранять списки ACL (Phase 14)
settings-preserve-sparseness = Сохранять разреженные файлы
settings-preserve-sparseness-hint = Копировать только выделенные блоки разреженных файлов (диски ВМ, файлы баз данных), чтобы размер на диске в месте назначения совпадал с источником.
settings-force-parallel-chunks = Параллельное копирование частями (только RAID / массивы)
settings-force-parallel-chunks-hint = Разбивает каждое крупное копирование на параллельные части. Полезно только для чередующихся/RAID/сетевых назначений; ЗАМЕДЛЯЕТ одиночный SSD/NVMe (-25%…-76%). Оставьте выключенным, если назначение не является массивом из нескольких дисков.

# Shell tab
settings-context-menu = Включить пункты контекстного меню оболочки
settings-intercept-copy = Перехватывать обработчик копирования по умолчанию (Windows)
settings-intercept-copy-hint = Когда включено, Ctrl+C / Ctrl+V в Explorer проходят через Copy That. Регистрация появится в Phase 14.
settings-notify-completion = Уведомлять о завершении задания

# Secure delete tab
settings-shred-method = Метод уничтожения по умолчанию
settings-shred-zero = Нули (1 проход)
settings-shred-random = Случайные данные (1 проход)
settings-shred-dod3 = DoD 5220.22-M (3 прохода)
settings-shred-dod7 = DoD 5220.22-M (7 проходов)
settings-shred-gutmann = Gutmann (35 проходов)
settings-shred-nist = NIST 800-88
settings-shred-confirm-twice = Требовать двойное подтверждение перед уничтожением

# Advanced tab
settings-log-level = Уровень журналирования
settings-log-off = Выкл.
settings-telemetry = Телеметрия
settings-telemetry-never = Никогда — никаких отправок данных ни при каком уровне журналирования
settings-error-policy = Политика ошибок по умолчанию
settings-error-policy-ask = Спрашивать
settings-error-policy-skip = Пропускать
settings-error-policy-retry = Повтор с задержкой
settings-error-policy-abort = Прервать при первом сбое
settings-history-retention = Хранение истории (дни)
settings-history-retention-hint = 0 = хранить вечно. Любое другое значение автоматически удаляет старые задания при запуске.
settings-database-path = Путь к базе данных
settings-database-path-default = (по умолчанию — каталог данных ОС)
settings-reset-all = Сбросить к значениям по умолчанию
settings-reset-confirm = Сбросить все настройки к значениям по умолчанию? Профили не затрагиваются.

# Profiles tab
settings-profiles-hint = Сохраните текущие настройки под именем; позже загрузите его, чтобы вернуться без изменения отдельных параметров.
settings-profile-name-placeholder = Имя профиля
settings-profile-save = Сохранить
settings-profile-import = Импорт…
settings-profile-load = Загрузить
settings-profile-export = Экспорт…
settings-profile-delete = Удалить
settings-profile-empty = Профили пока не сохранены.
settings-profile-import-prompt = Имя для импортированного профиля:

# Toasts driven by Phase 12 profile actions
toast-settings-reset = Настройки сброшены
toast-profile-saved = Профиль сохранён
toast-profile-loaded = Профиль загружен
toast-profile-exported = Профиль экспортирован
toast-profile-imported = Профиль импортирован

# Phase 14a — enumeration-time filters
settings-filters-hint = Пропускать файлы на этапе перечисления, чтобы движок даже не открывал их. Включения применяются только к файлам; исключения также отсекают совпадающие каталоги.
settings-filters-enabled = Включить фильтры для копирования деревьев
settings-filters-include-globs = Включающие шаблоны
settings-filters-include-globs-placeholder = **/*.txt
settings-filters-include-globs-hint = По одному шаблону на строку. Если список не пуст, файл должен совпасть хотя бы с одним включением, чтобы остаться. В каталоги всегда выполняется заход.
settings-filters-exclude-globs = Исключающие шаблоны
settings-filters-exclude-globs-placeholder = **/node_modules
settings-filters-exclude-globs-hint = По одному шаблону на строку. Совпадение отсекает всё поддерево для каталогов; совпавшие файлы пропускаются.
settings-filters-size-range = Диапазон размера файла
settings-filters-min-size-bytes = Минимальный размер (байты, пусто = без нижней границы)
settings-filters-max-size-bytes = Максимальный размер (байты, пусто = без верхней границы)
settings-filters-date-range = Диапазон времени изменения
settings-filters-min-mtime = Изменён не ранее
settings-filters-max-mtime = Изменён не позднее
settings-filters-attributes = Биты атрибутов
settings-filters-skip-hidden = Пропускать скрытые файлы / папки
settings-filters-skip-system = Пропускать системные файлы (только Windows)
settings-filters-skip-readonly = Пропускать файлы только для чтения

# Phase 15 — auto-update
settings-updater-hint = Copy That проверяет наличие подписанных обновлений не чаще раза в день. Обновления устанавливаются при следующем закрытии приложения.
settings-updater-auto-check = Проверять обновления при запуске
settings-updater-channel = Канал выпусков
settings-updater-channel-stable = Стабильный
settings-updater-channel-beta = Бета (предварительный выпуск)
settings-updater-last-check = Последняя проверка
settings-updater-last-never = Никогда
settings-updater-check-now = Проверить обновления сейчас
settings-updater-checking = Проверка…
settings-updater-available = Доступно обновление
settings-updater-up-to-date = У вас установлена последняя версия.
settings-updater-dismiss = Пропустить эту версию
settings-updater-dismissed = Пропущено
toast-update-available = Доступна более новая версия
toast-update-up-to-date = У вас уже последняя версия

# Phase 19a — disk-backed file enumeration (TeraCopy-compatible scan DB)
scan-progress-title = Сканирование…
scan-progress-stats = { $files } файлов · { $bytes } обработано
scan-pause-button = Приостановить сканирование
scan-resume-button = Возобновить сканирование
scan-cancel-button = Отменить сканирование
scan-cancel-confirm = Отменить сканирование и сбросить прогресс?
scan-db-header = База данных сканирования
scan-db-hint = Дисковая база данных сканирования для заданий с миллионами файлов.
advanced-scan-hash-during = Вычислять контрольные суммы во время сканирования
advanced-scan-db-path = Расположение базы данных сканирования
advanced-scan-retention-days = Автоудаление завершённых сканирований через (дни)
advanced-scan-max-keep = Максимум хранимых баз данных сканирования

# Phase 19b — filesystem-snapshot source for locked files.
settings-on-locked = Когда файл заблокирован
settings-on-locked-ask = Спросить в первый раз
settings-on-locked-retry = Кратко повторить, затем показать ошибку
settings-on-locked-skip = Пропустить заблокированный файл
settings-on-locked-snapshot = Использовать снимок файловой системы
settings-on-locked-hint = Устраняет ошибки «файл используется другим процессом». Copy That создаёт снимок тома источника (VSS в Windows, ZFS/Btrfs в Linux, APFS в macOS) и читает из копии снимка.
snapshot-prompt-title = Этот файл используется другим процессом
snapshot-prompt-body = Другая программа открыла { $path } для монопольной записи. Выберите, как Copy That должен обрабатывать этот и подобные файлы на том же томе.
snapshot-source-active = 📷 Чтение из снимка { $kind } тома { $volume }
snapshot-create-failed = Не удалось создать снимок тома источника
snapshot-vss-needs-elevation = Чтение из снимка VSS требует прав администратора. Copy That запросит разрешение.
snapshot-cleanup-failed = Помощник снимков сообщил о сбое очистки — на томе может остаться неудалённая теневая копия.

# Phase 20 — durable resume journal.
resume-prompt-title = Возобновить предыдущие передачи?
resume-prompt-body = Copy That обнаружил незавершённые передачи из предыдущего сеанса: { $count }. Выберите, что делать с каждой.
resume-prompt-resume = Возобновить
resume-prompt-resume-all = Возобновить все
resume-discard-one = Не возобновлять
resume-discard-all = Отклонить все
resume-aborted-hash-mismatch = Первые { $offset } байт в месте назначения не совпадают с источником — перезапуск с начала.
settings-auto-resume = Автоматически возобновлять прерванные задания без запроса
settings-auto-resume-hint = Пропускать запрос на возобновление при запуске и молча повторно ставить в очередь все незавершённые задания. По умолчанию выключено.

# Phase 21 — bandwidth shaping (GCRA token bucket + schedule + auto-throttle).
settings-tab-network = Сеть
settings-network-hint = Ограничьте скорость передачи, чтобы остальная сеть оставалась пригодной для работы. Применяйте глобально, по дневному расписанию или автоматически реагируйте на лимитированный Wi-Fi / батарею / сотовые подключения.
settings-network-mode = Ограничение полосы
settings-network-mode-off = Выкл. (без ограничения)
settings-network-mode-fixed = Фиксированное значение
settings-network-mode-schedule = Использовать расписание
settings-network-cap-mbps = Предел (МБ/с)
settings-network-schedule = Расписание (формат rclone)
settings-network-schedule-hint = Разделённые пробелами границы HH:MM,скорость плюс необязательные правила дней Mon-Fri,скорость. Скорости: 512k, 10M, 2G, off, unlimited. Пример: 08:00,512k 18:00,10M Sat-Sun,unlimited.
settings-network-auto-header = Автоограничение
settings-network-auto-metered = На лимитированном Wi-Fi
settings-network-auto-battery = От батареи
settings-network-auto-cellular = На сотовой сети
settings-network-auto-unchanged = Не переопределять
settings-network-auto-pause = Приостанавливать передачи
settings-network-auto-cap = Ограничить фиксированным значением
shape-badge-paused = приостановлено
shape-badge-tooltip = Ограничение полосы активно — нажмите, чтобы открыть Настройки → Сеть
shape-badge-source-schedule = по расписанию
shape-badge-source-metered = лимитировано
shape-badge-source-battery = от батареи
shape-badge-source-cellular = сотовая сеть
shape-badge-source-settings = активно
shape-error-schedule-invalid = Неверный формат расписания: { $message }

# Phase 22 — aggregate conflict dialog v2 (thumbnails, per-pattern
# rules, and reusable conflict profiles). Every key below is user-
# visible text in the `ConflictBatchModal.svelte` component.
conflict-batch-title = Конфликтов файлов: { $count } в { $jobname }
conflict-batch-state-pending = В ожидании
conflict-batch-state-resolved = Разрешено
conflict-batch-action-overwrite = Перезаписать
conflict-batch-action-skip = Пропустить
conflict-batch-action-keep-both = Сохранить оба
conflict-batch-action-newer-wins = Побеждает новее
conflict-batch-action-larger-wins = Побеждает больше
conflict-batch-bulk-apply-selected = Применить к выбранным
conflict-batch-bulk-apply-extension = Применить ко всем с этим расширением
conflict-batch-bulk-apply-glob = Применить к совпадающим по шаблону…
conflict-batch-bulk-apply-remaining = Применить ко всем оставшимся
conflict-batch-bulk-glob-placeholder = напр. **/*.tmp
conflict-batch-save-profile = Сохранить эти правила как профиль…
conflict-batch-profile-placeholder = Имя профиля
conflict-batch-matched-rule = по правилу «{ $rule }» → { $action }
conflict-batch-empty = Все конфликты разрешены
conflict-batch-source-vs-destination = Источник и назначение
conflict-batch-source-label = Источник
conflict-batch-destination-label = Назначение
conflict-batch-size-label = Размер
conflict-batch-modified-label = Изменён
conflict-batch-close = Закрыть
conflict-batch-profile-saved = Профиль конфликтов сохранён

# Phase 23 — sparse-file preservation. The toast fires once per
# destination volume when the filesystem can't preserve holes; the
# warning line is surfaced in the job detail drawer so the user knows
# the dst is larger on disk than the source was.
sparse-not-supported-title = Назначение заполняет разреженные файлы
sparse-not-supported-body = { $dst_fs } не поддерживает разреженные файлы. Пустоты в источнике записаны нулями, поэтому назначение занимает больше места на диске.
sparse-warning-densified = Разреженная структура сохранена: скопированы только выделенные блоки.
sparse-warning-mismatch = Несовпадение разреженной структуры — назначение может быть больше ожидаемого.

# Phase 24 — security-metadata preservation. The Mark-of-the-Web
# (Zone.Identifier ADS) toggle is security-sensitive: turning it off
# lets a downloaded executable shed its SmartScreen / Office Protected
# View flag on copy, which is why the tooltip carries an explicit
# warning. AppleDouble fallback emits `._<filename>` sidecars on
# destination filesystems that can't hold the foreign metadata.
settings-preserve-security-metadata = Сохранять метаданные безопасности
settings-preserve-security-metadata-hint = Захватывать и повторно применять внешние потоки метаданных (NTFS ADS / xattrs / POSIX ACL / контексты SELinux / возможности файлов Linux / ресурсные форки macOS) при каждом копировании.
settings-preserve-motw = Сохранять Mark-of-the-Web (флаг «загружено из интернета»)
settings-preserve-motw-hint = Критично для безопасности. SmartScreen и Office Protected View используют этот поток для предупреждения о файлах, загруженных из интернета. Отключение позволяет загруженному исполняемому файлу потерять метку происхождения при копировании и обойти защиту операционной системы.
settings-preserve-posix-acls = Сохранять POSIX ACL и расширенные атрибуты
settings-preserve-posix-acls-hint = Переносить xattrs user.* / system.* / trusted.* и списки управления доступом POSIX при копировании.
settings-preserve-selinux = Сохранять контексты SELinux
settings-preserve-selinux-hint = Переносить метку security.selinux при копировании, чтобы демоны под политиками MAC по-прежнему имели доступ к файлу.
settings-preserve-resource-forks = Сохранять ресурсные форки macOS и сведения Finder
settings-preserve-resource-forks-hint = Переносить устаревший ресурсный форк и FinderInfo (цветовые метки, метаданные Carbon) при копировании.
settings-appledouble-fallback = Использовать вспомогательный файл AppleDouble на несовместимых файловых системах
meta-translated-to-appledouble = Внешние метаданные сохранены во вспомогательном файле AppleDouble (._{ $ext })

# Phase 25 — two-way sync with vector-clock conflict detection.
# The drawer lists configured sync pairs; each pair runs an
# independent reconciliation round against a per-pair `.copythat-sync.db`
# state store. Concurrent edits from a common ancestor surface as
# conflicts rather than silent overwrites; the losing side's content
# is preserved as `name.sync-conflict-YYYYMMDD-HHMMSS-<host>.ext`.
footer-sync = Синхронизация
sync-drawer-title = Двусторонняя синхронизация
sync-drawer-hint = Держите две папки синхронизированными без молчаливых перезаписей. Одновременные правки проявляются как конфликты, которые вы можете разрешить.
sync-add-pair = Добавить пару
sync-add-cancel = Отмена
sync-refresh = Обновить
sync-add-save = Сохранить пару
sync-add-saving = Сохранение…
sync-add-missing-fields = Метка, левый путь и правый путь обязательны.
sync-remove-confirm = Удалить эту пару синхронизации? База состояния сохраняется; папки не затрагиваются.
sync-field-label = Метка
sync-field-label-placeholder = напр. Документы ↔ NAS
sync-field-left = Левая папка
sync-field-left-placeholder = Выберите или вставьте абсолютный путь
sync-field-right = Правая папка
sync-field-right-placeholder = Выберите или вставьте абсолютный путь
sync-field-mode = Режим
sync-mode-two-way = Двусторонний
sync-mode-mirror-left-to-right = Зеркало (левая → правая)
sync-mode-mirror-right-to-left = Зеркало (правая → левая)
sync-mode-contribute-left-to-right = Дополнение (левая → правая, без удалений)
sync-no-pairs = Пары синхронизации пока не настроены. Нажмите «Добавить пару», чтобы начать.
sync-loading = Загрузка настроенных пар…
sync-never-run = Никогда не запускалась
sync-running = Выполняется
sync-run-now = Запустить сейчас
sync-cancel = Отмена
sync-remove-pair = Удалить
sync-view-conflicts = Просмотр конфликтов ({ $count })
sync-conflicts-heading = Конфликты
sync-no-conflicts = Нет конфликтов из последнего запуска.
sync-winner = Победитель
sync-side-left-to-right = левая
sync-side-right-to-left = правая
sync-conflict-kind-concurrent-write = Одновременная правка
sync-conflict-kind-delete-edit = Удаление ↔ правка
sync-conflict-kind-add-add = Добавлено с обеих сторон
sync-conflict-kind-corrupt-equal = Содержимое разошлось без новой записи
sync-resolve-keep-left = Оставить левую
sync-resolve-keep-right = Оставить правую
sync-resolve-keep-both = Оставить обе
sync-resolve-three-way = Разрешить трёхсторонним слиянием
sync-resolve-phase-53-tooltip = Интерактивное трёхстороннее слияние для нетекстовых файлов появится в Phase 53.
sync-error-prefix = Ошибка синхронизации

# Phase 26 — real-time mirror watcher. "Live mirror" starts a
# filesystem watcher on the pair's left side; every debounced event
# triggers a re-sync. The watcher filters vim swap files / Office
# lock files / atomic-save temp names so one logical save becomes
# exactly one sync round.
live-mirror-start = Запустить живое зеркало
live-mirror-stop = Остановить живое зеркало
live-mirror-watching = Наблюдение
live-mirror-toggle-hint = Автоматически пересинхронизировать при каждом обнаруженном изменении файловой системы. По одному фоновому потоку на активную пару.
watch-event-prefix = Изменение файла
watch-overflow-recovered = Буфер наблюдателя переполнился; повторное перечисление для восстановления

# Phase 27 — content-defined chunk store. Enables delta-resume (a
# retry only re-writes chunks that actually changed) and same-job
# dedup (files sharing content blocks store those blocks once). The
# store is disk-backed under `<data-dir>/chunks/` by default and is
# the foundation for the Phase 49–51 moonshot repository phases.
chunk-store-section = Хранилище блоков
chunk-store-enable = Включить хранилище блоков (дельта-возобновление и дедупликация)
chunk-store-enable-hint = Разбивает каждый скопированный файл по содержимому (FastCDC) и хранит блоки с адресацией по содержимому. Повторы перезаписывают только изменённые блоки; файлы с общим содержимым дедуплицируются автоматически.
chunk-store-location = Расположение хранилища блоков
chunk-store-max-size = Максимальный размер хранилища блоков
chunk-store-prune = Удалять блоки старше (дни)
chunk-store-savings = Сэкономлено { $gib } ГиБ за счёт дедупликации блоков
chunk-store-disk-usage = Используется { $size } в { $chunks } блоках

# Phase 28 — tray-resident Drop Stack. The stack is a persistent list
# of paths gathered from multiple sources (Explorer drag, main-window
# context menu, CLI, drag onto the Drop Stack window) that the user
# can dispatch to a destination in one go.
dropstack-window-title = Drop Stack
dropstack-tray-open = Drop Stack
dropstack-empty-title = Drop Stack пуст
dropstack-empty-hint = Перетащите сюда файлы из Explorer или щёлкните правой кнопкой по строке задания, чтобы добавить.
dropstack-add-to-stack = Добавить в Drop Stack
dropstack-copy-all-to = Скопировать всё в…
dropstack-move-all-to = Переместить всё в…
dropstack-clear = Очистить стек
dropstack-remove-row = Удалить из стека
dropstack-path-missing-toast = { $path } перетащено — файл больше не существует.
dropstack-always-on-top = Держать Drop Stack поверх всех окон
dropstack-show-tray-icon = Показывать значок Copy That в трее
dropstack-open-on-start = Открывать Drop Stack автоматически при запуске приложения
dropstack-count = { $count } путей

# Phase 29 — spring-loaded folders + native DnD polish. The Settings
# → General tab carries the knobs; DropTarget / DestinationPicker
# Svelte components use the dropzone-* keys at runtime.
settings-dnd-heading = Перетаскивание
settings-dnd-spring-load = Раскрывать папки при перетаскивании
settings-dnd-spring-delay = Задержка раскрытия (мс)
settings-dnd-thumbnails = Показывать эскизы при перетаскивании
settings-dnd-invalid-highlight = Подсвечивать недопустимые цели для сброса
dropzone-invalid-title = Недопустимая цель для сброса
dropzone-invalid-readonly = Назначение доступно только для чтения
dropzone-picker-title = Выберите назначение
dropzone-picker-up = Вверх
dropzone-picker-path = Текущий путь
dropzone-picker-root = Корни
dropzone-picker-use-this = Использовать эту папку
dropzone-picker-empty = Нет вложенных папок
dropzone-picker-cancel = Отмена

# Phase 30 — cross-platform path translation. Settings → Transfer
# exposes these under a "Cross-platform compatibility" subsection.
translate-heading = Кроссплатформенная совместимость
translate-unicode-label = Нормализация Unicode
translate-unicode-auto = Автоопределение назначения
translate-unicode-windows = NFC (Windows / Linux)
translate-unicode-macos = Оставить как есть (macOS / APFS)
translate-line-endings-label = Преобразовывать переводы строк для текстовых файлов
translate-line-endings-allowlist = Расширения текстовых файлов
reserved-name-label = Обработка зарезервированных имён Windows
reserved-name-suffix = Добавлять «_» (CON.txt → CON_.txt)
reserved-name-reject = Отклонять и предупреждать
long-path-label = Использовать префикс длинных путей Windows (\\?\) при длине более 260 символов
long-path-hint = Некоторые сетевые ресурсы и устаревшие инструменты не поддерживают пространство имён \\?\.

# Phase 31 — power-aware copying. Settings → Power & State tab + the
# header badge that renders "⏸ Paused — Zoom call detected" when the
# runner's power subscriber has paused or capped due to a policy match.
power-heading = Питание и состояние
power-enabled = Включить правила учёта питания
power-battery-label = От батареи
power-metered-label = На лимитированном Wi-Fi
power-cellular-label = На сотовой сети
power-presentation-label = Во время презентации (Zoom / Teams / Keynote)
power-fullscreen-label = Когда приложение в полноэкранном режиме
power-thermal-label = Когда ЦП троттлит из-за нагрева
power-rule-continue = Продолжать на полной скорости
power-rule-pause = Приостановить все задания
power-rule-cap = Ограничить полосу
power-rule-cap-percent = Ограничить процентом от текущей скорости
power-reason-on-battery = от батареи
power-reason-metered-network = лимитированная сеть
power-reason-cellular-network = сотовая сеть
power-reason-presenting = режим презентации
power-reason-fullscreen = полноэкранное приложение
power-reason-thermal-throttling = ЦП троттлит

# Phase 32 — cloud backend matrix via OpenDAL. Settings → Remotes
# tab + the Add-backend wizard that writes one entry per remote into
# the keychain-backed credential store.
remote-heading = Удалённые бэкенды
remote-add = Добавить бэкенд
remote-list-empty = Удалённые бэкенды не настроены
remote-test = Проверить подключение
remote-test-success = Подключение успешно
remote-test-failed = Сбой подключения
remote-remove = Удалить бэкенд
remote-name-label = Отображаемое имя
remote-kind-label = Тип бэкенда
remote-save = Сохранить бэкенд
remote-cancel = Отмена
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
backend-local-fs = Локальная файловая система
cloud-config-bucket = Контейнер
cloud-config-region = Регион
cloud-config-endpoint = URL конечной точки
cloud-config-root = Корневой путь
cloud-error-invalid-config = Конфигурация бэкенда недопустима
cloud-error-network = Сетевая ошибка при обращении к бэкенду
cloud-error-not-found = Объект не найден по запрошенному пути
cloud-error-permission = Доступ отклонён удалённым бэкендом
cloud-error-keychain = Сбой доступа к связке ключей ОС
settings-tab-remotes = Удалённые
settings-tab-mobile = Мобильные

# Phase 33 — mount Copy That's chunk store + history archive as a
# read-only filesystem (FUSE / WinFsp). Surfaced by the History tab
# context menu's "Mount snapshot" action + the Settings → Advanced
# "Mount latest on launch" toggle.
mount-heading = Монтировать снимок
mount-action-mount = Монтировать снимок
mount-action-unmount = Размонтировать
mount-status-mounted = Смонтировано в { $path }
mount-error-unsafe-mountpoint = Путь точки монтирования небезопасен
mount-error-mountpoint-not-empty = Точка монтирования должна быть пустым каталогом
mount-error-backend-unavailable = Бэкенд монтирования недоступен в этой системе
mount-error-archive-read = Сбой чтения архива
mount-picker-title = Выберите каталог точки монтирования
mount-toast-mounted = Снимок смонтирован в { $path }
mount-toast-unmounted = Снимок размонтирован
mount-toast-failed = Сбой монтирования: { $reason }
settings-mount-heading = Монтирование снимков
settings-mount-hint = Предоставлять архив истории как файловую систему только для чтения. Phase 33b связывает поток выполнения; ядерные бэкенды FUSE/WinFsp появятся в Phase 33c.
settings-mount-on-launch = Монтировать последний снимок при запуске
settings-mount-on-launch-path = Путь точки монтирования
settings-mount-on-launch-path-placeholder = напр. C:\Mounts\copythat

# Phase 34 — enterprise-grade audit log export + WORM (write-once-read-
# many) tamper-resistance. Settings → Advanced → Audit log collects the
# format + destination + rotation + WORM toggle; runtime records job and
# file events into the tamper-evident chain-hashed sink.
settings-audit-heading = Журнал аудита
settings-audit-hint = Журнал событий каждого задания и файла, доступный только для добавления и защищённый от подделки. Форматы: CSV, JSON-строки, Syslog RFC 5424, ArcSight CEF и QRadar LEEF.
settings-audit-enable = Включить журналирование аудита
settings-audit-format = Формат журнала
settings-audit-format-json-lines = JSON-строки (рекомендуется по умолчанию)
settings-audit-format-csv = CSV (удобно для таблиц)
settings-audit-format-syslog = Syslog (RFC 5424)
settings-audit-format-cef = CEF (ArcSight)
settings-audit-format-leef = LEEF 2.0 (IBM QRadar)
settings-audit-file-path = Путь к файлу журнала
settings-audit-file-path-placeholder = напр. C:\ProgramData\CopyThat\audit.log
settings-audit-max-size = Ротация после (байты, 0 = никогда)
settings-audit-worm = Включить режим WORM (запись один раз, чтение многократно)
settings-audit-worm-hint = Применяет флаг «только для добавления» платформы (Linux chattr +a, macOS chflags uappnd, атрибут «только для чтения» в Windows) после каждого создания или ротации. Даже администратор должен явно снять флаг, чтобы усечь журнал.
settings-audit-test-write = Тестовая запись
settings-audit-verify-chain = Проверить цепочку
toast-audit-test-write-ok = Тестовая запись в журнал аудита успешна
toast-audit-verify-ok = Цепочка аудита проверена, цела
toast-audit-verify-failed = Проверка цепочки аудита выявила несоответствия

# Phase 35 — destination encryption via age (X25519 / passphrase /
# SSH) + on-the-fly zstd compression with a per-extension deny
# heuristic. Surfaced through Settings → Transfer → Encryption +
# Compression; engine short-circuits to the crypt pipeline when
# either stage is active.
settings-crypt-heading = Шифрование и сжатие
settings-crypt-hint = Преобразовывать содержимое файлов до их записи в место назначения. Шифрование использует формат age; сжатие использует zstd и может пропускать уже сжатые медиафайлы по расширению.
settings-crypt-encryption-mode = Шифрование
settings-crypt-encryption-off = Выкл.
settings-crypt-encryption-passphrase = Пароль (запрос при начале копирования)
settings-crypt-encryption-recipients = Ключи получателей из файла
settings-crypt-encryption-hint = Пароли хранятся только в памяти на время копирования. Файлы получателей содержат по одному публичному ключу age1… или ssh- на строку.
settings-crypt-recipients-file = Путь к файлу получателей
settings-crypt-recipients-file-placeholder = напр. C:\Users\me\recipients.txt
settings-crypt-compression-mode = Сжатие
settings-crypt-compression-off = Выкл.
settings-crypt-compression-always = Всегда
settings-crypt-compression-smart = Умное (пропускать уже сжатые медиафайлы)
settings-crypt-compression-hint = Умный режим пропускает jpg, mp4, zip, 7z и подобные форматы, которым zstd не помогает. Режим «Всегда» сжимает каждый файл на выбранном уровне.
settings-crypt-compression-level = Уровень zstd (1-22)
settings-crypt-compression-level-hint = Меньшие числа быстрее; большие сжимают сильнее. Уровень 3 соответствует значению по умолчанию в CLI zstd.
compress-footer-savings = 💾 { $original } → { $compressed } (сэкономлено { $percent }%)
compress-savings-toast = Сжато на { $percent }% (сэкономлено { $bytes })
crypt-toast-recipients-loaded = Загружено получателей шифрования: { $count }
crypt-toast-recipients-error = Не удалось загрузить получателей: { $reason }
crypt-toast-passphrase-required = Для шифрования нужен пароль до начала копирования
crypt-toast-passphrase-set = Пароль шифрования получен
crypt-footer-encrypted-badge = 🔒 Зашифровано (age)
crypt-footer-compressed-badge = 📦 Сжато (zstd)

# Phase 36 — copythat CLI surface. Documented exit codes + the
# user-facing strings for plan/apply/verify/config. Help text stays
# in English (engineering accessibility) but error / info / status
# strings are localized so a localized desktop installation surfaces
# CLI errors in the same language as the GUI.
cli-help-tagline = Copy That CLI — байт-в-байт копирование, синхронизация, проверка и аудит файлов для конвейеров CI/CD.
cli-help-exit-codes = Коды выхода: 0 успех, 1 ошибка, 2 в ожидании, 3 конфликт, 4 сбой проверки, 5 сеть, 6 права, 7 диск заполнен, 8 отмена, 9 конфигурация.
cli-error-bad-args = copy/move требует хотя бы один источник и назначение
cli-error-unknown-algo = Неизвестный алгоритм проверки: { $algo }
cli-error-missing-spec = --spec обязателен для plan/apply
cli-error-spec-parse = Не удалось разобрать jobspec { $path }: { $reason }
cli-error-spec-empty-sources = Список источников jobspec пуст
cli-info-shape-recorded = Ограничение полосы «{ $rate }» записано; применение выполняется через copythat-shape
cli-info-stub-deferred = { $command } подготовлено для дополнительной привязки в рамках Phase 36
cli-plan-summary = План: действий { $actions }, байт { $bytes }; уже на месте { $already_done }
cli-plan-pending = План сообщает об ожидающих действиях; перезапустите с `apply` для выполнения
cli-plan-already-done = План сообщает, что делать нечего (идемпотентно)
cli-apply-success = Apply завершён без ошибок
cli-apply-failed = Apply завершён с одной или несколькими ошибками
cli-verify-ok = Проверка успешна: { $algo } { $digest }
cli-verify-failed = Проверка НЕ ПРОЙДЕНА для { $path } ({ $algo })
cli-config-set = Установлено { $key } = { $value }
cli-config-reset = { $key } сброшено к значению по умолчанию
cli-config-unknown-key = Неизвестный ключ конфигурации: { $key }
cli-completions-emitted = Автодополнения оболочки для { $shell } выведены в stdout

# Phase 37 — desktop-side mobile companion. Settings → Mobile panel
# strings + the SAS-confirmation modal + push-notification toasts.
# The actual mobile UI lives in the Phase 37 follow-up Tauri Mobile
# target; the phone displays its own localized SAS prompt.
settings-mobile-heading = Мобильный спутник
settings-mobile-hint = Подключите iPhone или телефон Android, чтобы просматривать историю, запускать сохранённые профили и jobspec-задания Phase 36 и получать уведомления о завершении.
settings-mobile-pair-toggle = Разрешить новые сопряжения
settings-mobile-pair-active = Сервер сопряжения активен — отсканируйте QR-код в мобильном приложении Copy That
settings-mobile-pair-button = Начать сопряжение
settings-mobile-revoke-button = Отозвать
settings-mobile-no-pairings = Сопряжённых устройств пока нет
settings-mobile-pair-port = Порт привязки (0 = выбрать свободный)
pair-sas-prompt = На обоих экранах должны отображаться одинаковые четыре эмодзи. Нажмите «Совпадает», если они одинаковы.
pair-sas-confirm = Совпадает
pair-sas-reject = Не совпадает — отмена
pair-toast-success = Сопряжено с { $device }
pair-toast-failed = Сбой сопряжения: { $reason }
push-toast-sent = Push отправлен на { $device }
push-toast-failed = Сбой push на { $device }: { $reason }

# Phase 38 — aggregate destination dedup + reflink fallback ladder.
# Settings → Transfer → Dedup panel + per-job-row badges (⚡
# Reflinked / 🔗 Hardlinked / 🧩 Chunk-shared / 📋 Copied) +
# pre-pass dedup-scan modal.
settings-dedup-heading = Дедупликация в месте назначения
settings-dedup-hint = Когда источник и назначение находятся на одном томе, Copy That может клонировать файлы на уровне файловой системы вместо копирования байтов. Reflink мгновенный и безопасный; жёсткая ссылка быстрее, но оба имени разделяют состояние.
settings-dedup-mode-auto = Авто-лестница (reflink → жёсткая ссылка → блок → копия)
settings-dedup-mode-reflink-only = Только reflink
settings-dedup-mode-hardlink-aggressive = Агрессивно (reflink + жёсткая ссылка даже для записываемых файлов)
settings-dedup-mode-off = Отключено (всегда побайтовое копирование)
settings-dedup-hardlink-policy = Политика жёстких ссылок
settings-dedup-prescan = Предварительно сканировать дерево назначения на дублирующееся содержимое
dedup-badge-reflinked = ⚡ Reflink
dedup-badge-hardlinked = 🔗 Жёсткая ссылка
dedup-badge-chunk-shared = 🧩 Общий блок
dedup-badge-copied = 📋 Скопировано
phase42-paranoid-verify-label = Параноидальная проверка
phase42-paranoid-verify-hint = Сбрасывает кэшированные страницы назначения и перечитывает с диска, чтобы выявить ложь кэша записи и тихие повреждения. Примерно на 50% медленнее обычной проверки; по умолчанию выключено.
phase42-sharing-violation-retries-label = Число повторов для заблокированных файлов источника
phase42-sharing-violation-retries-hint = Сколько раз повторять, когда другой процесс удерживает файл источника открытым с монопольной блокировкой. Задержка удваивается с каждой попыткой (по умолчанию 50 мс / 100 мс / 200 мс). По умолчанию 3, как Robocopy /R:3.
phase42-cloud-placeholder-warning = { $name } — это облачный файл OneDrive. Его копирование вызовет загрузку — до { $size } через ваше сетевое подключение.
phase42-defender-exclusion-hint = Для максимальной скорости копирования добавьте папку назначения в исключения Microsoft Defender перед массовыми передачами. См. docs/PERFORMANCE_TUNING.md.

# Phase 39 — Browser-accessible recovery UI. Settings → Advanced
# exposes these strings; the recovery server itself renders askama
# templates that always read English — these eight keys are the
# Settings prose the user actually reads in their preferred locale.
settings-recovery-heading = Веб-интерфейс восстановления
settings-recovery-enable = Включить веб-интерфейс восстановления
settings-recovery-bind-address = Адрес привязки
settings-recovery-port = Порт (0 = выбрать свободный)
settings-recovery-show-url = Показать URL и токен
settings-recovery-rotate-token = Сменить токен
settings-recovery-allow-non-loopback = Разрешить привязку не к loopback
settings-recovery-non-loopback-warning = ВНИМАНИЕ: включение привязки не к loopback открывает интерфейс восстановления вашей локальной сети. Любой, кто узнает токен, сможет просматривать историю файлов и скачивать файлы. Используйте TLS или обратный прокси, если локальная сеть ненадёжна.

# Phase 40 — SMB compression negotiation + cloud-VM offload helper.
# 6 SMB keys (header badge + Settings prose) + 6 cloud-offload keys
# (Remotes tab wizard for cross-cloud copy templates).
smb-compress-badge = 🗜 Сжатие SMB: { $algo }
smb-compress-badge-tooltip = Сетевой трафик к этому назначению сжимается при передаче (SMB 3.1.1).
smb-compress-toast-saved = Сэкономлено { $bytes } в сети
smb-compress-algo-unknown = неизвестный алгоритм
settings-smb-compress-heading = Сетевое сжатие SMB
settings-smb-compress-hint = Автоматически согласовывать сжатие трафика SMB 3.1.1 для UNC-назначений. Бесплатный выигрыш на медленных каналах; игнорируется для локальных назначений.
cloud-offload-heading = Помощник разгрузки на облачную ВМ
cloud-offload-hint = При прямом копировании между двумя облаками сформируйте шаблон развёртывания, запускающий копирование с крошечной временной ВМ в облаке — байты никогда не касаются сети вашего ноутбука.
cloud-offload-render-button = Сформировать шаблон
cloud-offload-copy-clipboard = Копировать в буфер обмена
cloud-offload-template-format = Формат шаблона
cloud-offload-self-destruct-warning = ВМ автоматически отключается через { $minutes } минут — подтвердите роль IAM и регион перед развёртыванием.

# Phase 41 — animated before/after tree-diff preview. The `Preview
# changes` modal renders the rolled-up plan before the engine starts
# work; 14 keys cover the title, the summary header / counts, the
# row-reason labels, and the two action buttons.
preview-modal-title = Предпросмотр изменений
preview-summary-header = Что произойдёт
preview-category-additions = добавлений: { $count }
preview-category-replacements = замен: { $count }
preview-category-skips = пропущено: { $count }
preview-category-conflicts = конфликтов: { $count }
preview-category-unchanged = без изменений: { $count }
preview-bytes-to-transfer = к передаче: { $bytes }
preview-reason-source-newer = Источник новее
preview-reason-dest-newer = Назначение новее — будет пропущено
preview-reason-content-different = Содержимое отличается
preview-reason-identical = Идентично источнику
preview-button-run = Выполнить план
preview-button-reduce = Сократить мой план…

# Phase 42 — perceptual-hash visual-similarity dedup. Eight keys cover
# the pre-copy "looks visually identical" warning + the Settings panel.
perceptual-warn-title = Выглядит визуально идентично
perceptual-warn-body = { $name } в месте назначения, похоже, совпадает с изображением источника. Всё равно продолжить копирование?
perceptual-warn-keep-both = Сохранить оба
perceptual-warn-skip = Пропустить этот файл
perceptual-warn-overwrite = Всё равно перезаписать
perceptual-settings-heading = Дедупликация по визуальному сходству
perceptual-settings-hint = Обнаруживать визуально идентичные изображения в месте назначения до перезаписи. Хеш перцептивный (распознаёт то же изображение, пересохранённое в другом формате), а не побайтовый.
perceptual-settings-threshold-label = Порог предупреждения (ниже = строже совпадение)

# Phase 42 Part B — per-file rolling versions (Time Machine for any
# destination). 8 keys cover the version-list panel + retention picker.
version-list-heading = Предыдущие версии
version-list-empty = Прежних версий этого файла нет
version-list-restore = Восстановить эту версию
version-retention-heading = Хранить предыдущие версии при перезаписи
version-retention-none = Хранить все версии вечно
version-retention-last-n = Хранить последние { $n } версий
version-retention-older-than-days = Удалять версии старше { $days } дней
version-retention-gfs = Ежечасно { $h } · ежедневно { $d } · еженедельно { $w } · ежемесячно { $m }

# Phase 43 — forensic chain-of-custody manifests + BLAKE3 verified
# streaming. 14 keys cover the Settings → Provenance panel
# (heading, hint, toggles, TSA URL, signing-key management) plus
# the post-job manifest notification + the verify command's two
# headline result lines.
provenance-settings-heading = Криминалистическая цепочка ответственности
provenance-settings-hint = Подписывать каждое задание копирования манифестом BLAKE3 + ed25519. Проверяющие смогут позже повторно хешировать дерево назначения и доказать, что ни один байт не изменился с момента копирования.
provenance-settings-enable-default = Подписывать каждое новое задание по умолчанию
provenance-settings-show-after-job = Показывать манифест после каждого завершённого задания
provenance-settings-tsa-url-label = URL службы меток времени RFC 3161 по умолчанию
provenance-settings-tsa-url-hint = Необязательно. Когда задано, манифесты несут бесплатную метку времени TSA, доказывающую существование байтов в данный момент времени. Оставьте пустым, чтобы пропустить.
provenance-settings-keys-heading = Ключи подписи
provenance-settings-keys-generate = Создать новый ключ
provenance-settings-keys-import = Импорт ключа…
provenance-settings-keys-export = Экспорт публичного ключа…
provenance-job-completed-title = Манифест происхождения сохранён
provenance-job-completed-body = Подписано файлов: { $count } → { $path }
provenance-verify-clean = Манифест действителен для { $count } файлов; подпись { $sig }; корень Меркла в порядке.
provenance-verify-tampered = Манифест НЕДЕЙСТВИТЕЛЕН — подделано { $tampered }, отсутствует { $missing }.
# Phase 43 post-review hardening — toast text for the Settings →
# Provenance buttons whose Tauri IPC has not yet landed.
provenance-action-staged = Phase 43 — привязка IPC для этого действия появится в следующем коммите.

# Phase 44 — SSD-aware whole-drive sanitize (NVMe Sanitize / OPAL
# Crypto Erase) + the per-file shred refusal on copy-on-write
# filesystems. 16 keys cover the new "Drive sanitize" Settings
# subsection and the localized error messages.
sanitize-heading = Безопасная очистка всего накопителя
sanitize-hint = NVMe Sanitize, OPAL Crypto Erase и ATA Secure Erase стирают флеш-накопитель на уровне прошивки за миллисекунды. Перезапись отдельных файлов бессмысленна на флеш-памяти — многопроходное уничтожение лишь изнашивает NAND. Используйте это для реальной очистки.
sanitize-pick-device = Выберите накопитель для очистки
sanitize-mode-label = Метод очистки
sanitize-mode-nvme-format = NVMe Format (с безопасным стиранием)
sanitize-mode-nvme-sanitize-block = NVMe Sanitize — Block Erase (медленно, каждая ячейка)
sanitize-mode-nvme-sanitize-crypto = NVMe Sanitize — Crypto Erase (мгновенно)
sanitize-mode-ata-secure-erase = ATA Secure Erase (устаревшие SATA SSD)
sanitize-mode-opal-crypto-erase = TCG OPAL Crypto Erase (самошифрующиеся накопители)
sanitize-mode-apfs-crypto-erase = APFS Crypto Erase (смена ключа FileVault, только macOS)
sanitize-confirm-1 = Это уничтожит КАЖДЫЙ байт на { $device }. Отмены не будет.
sanitize-confirm-2 = Я понимаю, что все разделы, все файлы и все снимки на { $device } станут навсегда нечитаемыми.
sanitize-confirm-3 = Введите название модели накопителя, чтобы продолжить: { $model }
sanitize-running = Очистка { $device } ({ $mode }) — это может занять от миллисекунд (crypto erase) до десятков минут (block erase). Не отключайте питание.
sanitize-completed = Очистка завершена — { $device } теперь пуст.
ssd-honest-shred-meaningless = Уничтожение отдельных файлов на файловой системе copy-on-write (Btrfs / ZFS / APFS) не достигает базовых блоков. Используйте очистку всего накопителя и смену ключа полнодискового шифрования.
ssd-honest-advisory = Этот файл расположен на флеш-памяти. Перезапись отдельных файлов изнашивает NAND и НЕ гарантирует невосстановимость исходных ячеек. Для конфиденциальных данных очистите весь накопитель.

# Phase 44.1f post-review — placeholder toast for SanitizeTab
# buttons whose Tauri IPC has not yet landed.
sanitize-action-staged = Phase 44.1 — привязка IPC для этого действия появится в следующем коммите.

# Phase 45.3 — named-queue tab strip (Subfeature A). Tabs surface
# once the QueueRegistry holds at least one queue; the synthesised
# default tab keeps legacy single-queue jobs reachable.
queue-tab-default = По умолчанию
queue-tab-empty-state = Очереди заданий
queue-badge-tooltip = Ожидающие и выполняющиеся задания в этой очереди

# Phase 45.4 — drag-progress-merge (Subfeature B). Drag a queue tab
# onto another to merge their job lists. The default tab is neither
# draggable nor a drop target; only registry queues participate.
queue-drag-hint = Перетащите на другую очередь для объединения
queue-merge-confirm = Отпустите для объединения
queue-merge-toast = Очереди объединены

# Phase 45.5 — F2-queue UX (Subfeature C). F2 toggles
# `auto_enqueue_next` so every fresh enqueue piles into the running
# queue rather than spawning a parallel one. Status pill renders in
# the Footer; pulsing dot renders on the running tab.
queue-f2-active-hint = Режим F2: каждое новое задание попадает в эту очередь
queue-f2-toggled-on = Режим очереди F2 ВКЛ — новые задания присоединяются к выполняющейся очереди
queue-f2-toggled-off = Режим очереди F2 ВЫКЛ — новые задания создают параллельные очереди
queue-f2-status-bar = Режим очереди F2: ВКЛ

# Phase 45.6 — tray destination targets (Subfeature D). Pinned
# destinations appear in the OS tray menu; clicking one arms it as
# the active drop target so the next file drop bypasses the
# DropStagingDialog. Settings → General hosts the list editor.
tray-target-section-title = Назначения в трее
tray-target-section-hint = Закреплённые назначения появляются в меню трея. Нажмите одно, чтобы назначить его следующей целью для сброса.
tray-target-empty = Назначения в трее пока не закреплены.
tray-target-remove = Удалить
tray-target-add-label = Метка
tray-target-add-path = Путь или URI бэкенда
tray-target-add = Добавить
tray-target-armed-toast = Перетащите следующий файл, чтобы отправить его в { $label }
tray-target-active-pill = → { $label }

# Phase 45.7 follow-up — pinned-destination validation errors. The
# `queue_pin_destination` IPC returns these Fluent keys verbatim
# when input fails the IPC-boundary checks; the toast layer renders
# them via `t(...)` (Toast.svelte detects kebab-case-lowercase and
# routes through the locale table). Phase 17e's `err-path-escape` /
# `err-destination-empty` cover the path-traversal + empty cases for
# `queue_route_job`; the keys below are pin-specific.
err-pinned-destination-label-empty = Метка назначения в трее не может быть пустой.
err-pinned-destination-path-empty = Путь назначения в трее не может быть пустым.
err-pinned-destination-label-too-long = Метка назначения в трее слишком длинная (максимум 64 символа).
err-pinned-destination-path-too-long = Путь назначения в трее слишком длинный (максимум 1024 символа).
err-pinned-destination-label-invalid = Метка назначения в трее содержит недопустимые символы (перевод строки, возврат каретки или NUL).
err-pinned-destination-path-invalid = Путь назначения в трее содержит недопустимые символы (перевод строки, возврат каретки или NUL).
err-pinned-destination-too-many = Достигнут предел в 50 назначений в трее. Удалите одно, чтобы добавить другое.

# Панель библиотеки (этап 49) — единое представление репозитория с адресацией по содержимому.
footer-library = Библиотека
library-title = Библиотека
library-loading = Загрузка репозитория…
library-unavailable = Репозиторий недоступен
library-tab-live = Текущие
library-tab-snapshots = Снимки
library-tab-versions = Версии
library-hero-savings = доступно { $effective } полезных данных · сэкономлено { $pct }
library-hero-empty = сохранено { $chunks } фрагментов — снимков пока нет
library-stat-stored = Занято на диске
library-stat-effective = Полезные данные
library-stat-snapshots = Снимки
library-stat-chunks = Уникальные фрагменты
library-snapshot-empty = Снимков пока нет
library-snapshot-files = { $n } файлов
library-version-path-ph = Путь назначения…
library-version-load = Показать версии
library-version-empty = Для этого пути нет версий
repo-kind-copy = Копирование
repo-kind-sync = Синхронизация
repo-kind-version = Версионирование
repo-kind-backup = Резервное копирование
# Phase 49o — snapshot diff / compare.
library-tab-compare = Сравнить
repo-change-added = Добавлено
repo-change-removed = Удалено
repo-change-modified = Изменено
repo-change-unchanged = Без изменений
repo-diff-summary = { $added } добавлено · { $removed } удалено · { $modified } изменено
repo-diff-bytes-added = { $bytes } новых
repo-diff-pick-two = Выберите два снимка для сравнения
# Phase 49r — statistics / reports.
library-tab-reports = Отчёты
report-growth-title = Рост хранилища
report-by-kind-title = По типу
report-top-files-title = Топ файлов
report-dedup-ratio = Дедуплицировано { $pct }%
report-export = Экспорт отчёта
report-exported = Отчёт сохранён в { $path }
report-file-versions = { $n } версий
# Phase 49p — pinning / prune.
repo-pin = Закрепить
repo-unpin = Открепить
repo-pinned-badge = Закреплено
repo-prune-title = Очистить
repo-prune-keep-last = Оставить новейшие
repo-prune-removed = Очищено снимков: { $n }
repo-prune-none = Очищать нечего

# Phase 46.6 — Settings → Plugins tab. The IPC layer in
# `apps/copythat-ui/src-tauri/src/plugin_commands.rs` enumerates the
# per-user plugin store under `<config_dir>/plugins/`; the
# `PluginsTab.svelte` component renders these strings against the
# returned manifest + grant state.
settings-tab-plugins = Плагины
plugin-heading = Плагины
plugin-hint = Изолированные WASM-плагины расширяют Copy That пользовательскими хуками. Каждый плагин работает с ограничениями по ЦП и памяти на вызов и видит только те возможности хоста, которые вы ему предоставите.
plugin-list-empty = Плагины пока не установлены.
plugin-enabled = Включён
plugin-disabled = Отключён
plugin-hooks = Хуки
plugin-capabilities = Возможности
plugin-no-capabilities = (нет)
plugin-directory = Расположение
plugin-install-from-file = Установить из файла…
plugin-install-from-url = Установить из URL…
plugin-url-wasm = URL WASM
plugin-url-manifest = URL манифеста
plugin-url-hash = Хеш BLAKE3
plugin-url-preview = Предпросмотр
plugin-url-confirm = Подтвердить установку

# Phase 31b — power-policy settings (Power tab).
settings-tab-power = Питание
settings-power-hint = Ограничивать или приостанавливать копирование по состоянию питания: батарея, лимитная/сотовая сеть, презентация/полный экран или тепловой троттлинг ЦП.
settings-power-enabled = Включить регулировку по питанию
settings-power-battery = От батареи
settings-power-metered = В лимитной сети
settings-power-cellular = В сотовой сети
settings-power-presentation = Во время презентации
settings-power-fullscreen = В полноэкранном режиме
settings-power-thermal = При тепловом троттлинге
settings-power-continue = Продолжить
settings-power-pause = Пауза
err-server-not-implemented = Режим сервера пока недоступен.
err-webhook-not-implemented = Доставка вебхуков пока недоступна.

# Phase 47 — "why is this slow?" diagnostics (bottleneck badge + tooltip).
bottleneck-source-io = Источник I/O
bottleneck-dest-io = Назначение I/O
bottleneck-network = Сеть
bottleneck-antivirus = Антивирус
bottleneck-cpu = CPU
bottleneck-thermal = Перегрев
bottleneck-unknown = Неизвестно
diag-aria = Узкое место: { $cause }
diag-tooltip = Ограничено: { $cause } · { $rate }
diag-spark-aria = Пропускная способность за последнюю минуту
diag-keeping-up = Справляется
diag-label = Диагностика

# Phase 48 — server mode + observability (Settings → Server).
settings-tab-server = Сервер
server-hint = Запустите Copy That как файловый сервер без интерфейса. Выберите протоколы для публикации, задайте адрес и папку для раздачи и при необходимости включите аутентификацию.
server-protocols = Протоколы
server-bind-addr = Адрес привязки
server-root = Раздаваемая папка
server-readonly = Только чтение (запрет загрузки и удаления)
server-auth-mode = Аутентификация
server-auth-none = Нет
server-auth-bearer = Токен Bearer
server-auth-basic = Базовая (имя пользователя + пароль)
server-auth-token = Токен
server-auth-user = Имя пользователя
server-auth-password = Пароль
otel-endpoint = Конечная точка OpenTelemetry
webhook-section = Webhooks
webhook-url = URL вебхука
webhook-add = Добавить вебхук
webhook-remove = Удалить
webhook-empty = Вебхуки не настроены.
webhook-pushover-token = Токен Pushover
webhook-pushover-user = Пользователь Pushover
server-start = Запустить сервер
server-stop = Остановить сервер
server-status-running = Работает на { $addr }
server-status-stopped = Остановлен
server-metrics-url = Метрики
err-server-no-protocols = Выберите хотя бы один протокол перед запуском сервера.
err-server-bind = Не удалось привязать адрес сервера. Возможно, он уже используется.

# Phase 49c — источники резервного копирования.
library-tab-sources = Источники
backup-add-source = Добавить источник…
backup-source-path-ph = Папка для резервного копирования…
backup-exclude-ph = Шаблоны исключения (через запятую)
backup-now = Создать резервную копию
backup-remove = Удалить
backup-empty = Источников резервного копирования пока нет
backup-never-run = Резервная копия не создавалась
backup-last-run = Последняя копия { $when }
backup-running = Резервное копирование… { $files } файлов
backup-toast-started = Резервное копирование { $label }…
backup-toast-completed = { $label } скопировано: { $files } файлов
backup-toast-failed = Сбой резервного копирования { $label }: { $reason }
# Phase 49e — per-source retention + prune.
backup-retention = Хранение
backup-retention-keep-all = Хранить все
backup-retention-last = Хранить последние { $n }
backup-retention-days = Старше { $days } дней
backup-retention-gfs = Ротация GFS
backup-prune-now = Очистить сейчас
backup-prune-none = Нечего очищать
backup-prune-result = Удалено { $removed } снимков · освобождено { $bytes }
# Phase 49f — per-source scheduling.
backup-schedule = Расписание
backup-schedule-manual = Вручную
backup-schedule-hourly = Ежечасно
backup-schedule-daily = Ежедневно
backup-schedule-weekly = Еженедельно
backup-next-run = Следующий запуск { $when }
backup-not-scheduled = Не запланировано
# Phase 49g — source filters.
backup-include-ph = Включающие шаблоны (через запятую)
backup-skip-hidden = Пропускать скрытые
# Phase 49q — notifications.
notify-title = Уведомления
notify-on-success = При успехе
notify-on-failure = При сбое
notify-test = Отправить тест
notify-test-sent = Тест отправлен на { $n } назначений

# Phase 49d — обозреватель восстановления.
restore-browse = Восстановить…
restore-title = Восстановить из снимка
restore-select-all = Выбрать все
restore-dest = Восстановить в
restore-confirm = Восстановить { $n } файлов
restore-empty = В этом снимке нет файлов
restore-conflict-body = { $count } выбранных файлов уже существуют в месте назначения.
restore-conflict-overwrite = Перезаписать
restore-conflict-skip = Пропустить существующие
restore-conflict-keep-both = Сохранить оба
restore-toast-done = Восстановлено { $restored }, пропущено { $skipped }
restore-toast-failed = Сбой восстановления: { $reason }
snapshot-forget = Забыть
snapshot-forget-toast = Снимок забыт — запустите «Освободить место», чтобы очистить
library-reclaim = Освободить место
# Phase 49i — full compaction.
library-compact = Полное уплотнение
library-compact-started = Уплотнение начато — смотрите Задачи
# Phase 49h — compression.
library-stat-compression = Сэкономлено сжатием
storage-compression = Сжатие
storage-compression-off = Выкл.
storage-compression-auto = Авто (пропускать несжимаемые)
storage-compression-always = Всегда
storage-compression-restart = Применяется при следующем запуске
# Phase 49j — tasks & progress center.
footer-tasks = Задачи
tasks-title = Задачи
tasks-empty = Пока нет задач
tasks-running = Выполняется
tasks-recent = Недавние
tasks-cancel = Отмена
task-state-running = Выполняется
task-state-completed = Завершено
task-state-failed = Сбой
task-state-cancelled = Отменено
# Phase 49k — repository setup/connect wizard.
repo-wizard-title = Подключить репозиторий
repo-wizard-create-tab = Создать новый
repo-wizard-connect-tab = Подключить существующий
repo-field-name = Имя
repo-field-path = Расположение
repo-field-password = Пароль (необязательно)
repo-action-create = Создать
repo-action-connect = Подключить
repo-action-browse = Обзор…
repo-switcher-label = Репозиторий
repo-action-forget = Забыть
repo-action-change-pass = Изменить пароль…
repo-password-old = Текущий пароль
repo-password-new = Новый пароль
repo-error-exists = В этом расположении уже существует репозиторий
repo-error-not-found = В этом расположении репозиторий не найден
repo-error-bad-pass = Неверный пароль
repo-note-no-encryption = Пароль управляет только доступом; шифрование данных в покое появится в одном из следующих выпусков
repo-confirm-forget = Удалить "{ $name }" из списка? Ваши данные останутся на диске.
repo-toast-created = Репозиторий "{ $name }" создан
repo-toast-connected = Подключено к "{ $name }"
repo-toast-pass-changed = Пароль обновлён
# Phase 49l — Sources dashboard.
library-tab-overview = Обзор
library-source-empty = Пока нет источников
library-source-unknown = (источник не указан)
library-source-snapshots = { $n } снимков
library-source-latest = Последний { $when }
# Phase 49n — verify & repair.
repo-action-verify = Проверить
repo-action-verify-deep = Проверить (читать все данные)
repo-action-repair = Исправить…
repo-verify-clean = Проверено { $files } файлов / { $chunks } блоков — повреждений нет
repo-verify-damaged = { $missing } отсутствует, { $corrupt } повреждённых блоков
repo-repair-confirm = Удалить { $n } снимков, которые больше нельзя восстановить?
repo-repair-removed = Удалено { $n } повреждённых снимков
repo-repair-none = Исправлять нечего — репозиторий чист
repo-gc-done = Освобождено { $bytes } ({ $chunks } блоков)
restore-toast-partial = Восстановлено { $restored }, пропущено { $skipped }, не удалось { $failed }
