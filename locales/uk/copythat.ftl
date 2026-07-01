app-name = Freally File Manager v0.19.85
window-title = Freally File Manager v0.19.85
shred-ssd-advisory = Попередження: цей об'єкт розташований на SSD. Багатопрохідне перезаписування ненадійно очищає флеш-пам'ять, оскільки вирівнювання зносу та надмірне резервування переміщують дані з-під логічної адреси блоку. Для твердотільних носіїв надавайте перевагу ATA SECURE ERASE, NVMe Format із Secure Erase або повному шифруванню диска з відкинутим ключем.

# Global aggregate states (header pill)
state-idle = Очікування
state-copying = Копіювання
state-verifying = Перевірка
state-paused = Призупинено
state-error = Помилка

# Per-job states (row badge)
state-pending = У черзі
state-running = Виконується
state-cancelled = Скасовано
state-succeeded = Готово
state-failed = Збій

# Actions
action-pause = Призупинити
action-resume = Відновити
action-cancel = Скасувати
action-pause-all = Призупинити всі завдання
action-resume-all = Відновити всі завдання
action-cancel-all = Скасувати всі завдання
action-close = Закрити
action-reveal = Показати в папці
action-add-files = Додати файли
action-add-folders = Додати папки

# Phase 13d — activity feed
activity-title = Активність
activity-clear = Очистити список активності
activity-empty = Поки що немає активності з файлами.
activity-after-done = Після завершення:
activity-keep-open = Залишити застосунок відкритим
activity-close-app = Закрити застосунок
activity-shutdown = Вимкнути ПК
activity-logoff = Вийти з системи
activity-sleep = Сон

# Phase 14 — preflight free-space dialog
preflight-block-title = Недостатньо місця на призначенні
preflight-warn-title = Мало місця на призначенні
preflight-unknown-title = Не вдалося визначити вільне місце
preflight-unknown-body = Джерело надто велике, щоб швидко оцінити розмір, або том призначення не відповів. Можна продовжити; захист місця рушія акуратно зупинить копіювання, якщо місце закінчиться.
preflight-required = Потрібно
preflight-free = Вільно
preflight-reserve = Резерв
preflight-shortfall = Нестача
preflight-continue = Усе одно продовжити
preflight-pick-subset = Вибрати, що копіювати…
collision-modal-overwrite-older = Перезаписати лише старіші

# Phase 14e — subset picker
subset-title = Виберіть джерела для копіювання
subset-subtitle = Повний вибір не вміщається на призначенні. Позначте елементи, які хочете скопіювати; решта залишиться.
subset-loading = Вимірювання розмірів…
subset-too-large = завеликий для підрахунку
subset-budget = Доступно
subset-remaining = Залишилось
subset-confirm = Копіювати вибране
history-rerun-hint = Повторити це копіювання — повторно скануються всі файли в дереві джерела
history-clear-all = Очистити все
history-clear-all-confirm = Клацніть ще раз для підтвердження
history-clear-all-hint = Видалити кожен рядок історії. Потрібне повторне клацання для підтвердження.
toast-history-cleared = Історію очищено (видалено рядків: { $count })

# Phase 15 — source-list ordering in the drop staging dialog
drop-dialog-sort-label = Порядок:
sort-custom = Власний
sort-name-asc = Назва А → Я (спочатку файли)
sort-name-desc = Назва Я → А (спочатку файли)
sort-size-asc = Розмір від меншого (спочатку файли)
sort-size-desc = Розмір від більшого (спочатку файли)
sort-reorder = Змінити порядок
sort-move-top = Перемістити на початок
sort-move-up = Перемістити вгору
sort-move-down = Перемістити вниз
sort-move-bottom = Перемістити в кінець

# Phase 16 — sort preset names for the Activity list (shorter
# wording than the DropStagingDialog variants; the Activity list
# is files-only so the "(files first)" suffix isn't relevant).
sort-name-asc-simple = Назва А → Я
sort-name-desc-simple = Назва Я → А
sort-size-asc-simple = Розмір від меншого
sort-size-desc-simple = Розмір від більшого
activity-sort-locked = Сортування вимкнено під час копіювання. Призупиніть або дочекайтеся завершення, потім змініть порядок.

# Phase 17 — collision-policy picker in the DropStagingDialog
drop-dialog-collision-label = Якщо файл уже існує:
collision-policy-keep-both = Зберегти обидва (перейменувати нову копію на _2, _3, …)
collision-policy-skip = Пропустити нову копію
collision-policy-overwrite = Перезаписати наявний файл
collision-policy-overwrite-if-newer = Перезаписати лише якщо новіший
collision-policy-prompt = Питати щоразу

# Phase 18 — progress labels on the DropStagingDialog Start button
drop-dialog-busy-checking = Перевірка вільного місця…
drop-dialog-busy-enumerating = Підрахунок файлів…
drop-dialog-busy-starting = Початок копіювання…
toast-enumeration-deferred = Дерево джерела велике — попередній список файлів пропущено; рядки з'являтимуться в міру обробки рушієм.

# Context menu (per-row right-click)
menu-pause = Призупинити
menu-resume = Відновити
menu-cancel = Скасувати
menu-remove = Вилучити з черги
menu-reveal-source = Показати джерело в папці
menu-reveal-destination = Показати призначення в папці

# Header / toolbar
header-eta-label = Орієнтовний час, що залишився
header-toolbar-label = Глобальні елементи керування

# Footer
footer-queued = активних завдань
footer-total-bytes = у процесі
footer-errors = помилок
footer-history = Історія

# Empty state
empty-title = Перетягніть файли або папки для копіювання
empty-hint = Перетягніть елементи у вікно. Ми запитаємо призначення, потім поставимо в чергу одне завдання на кожне джерело.
empty-region-label = Список завдань

# Details drawer
details-drawer-label = Деталі завдання
details-source = Джерело
details-destination = Призначення
details-state = Стан
details-bytes = Байтів
details-files = Файлів
details-speed = Швидкість
details-eta = Час до завершення
details-error = Помилка

# Drop dialog
drop-dialog-title = Перенести перетягнуті елементи
drop-dialog-subtitle = { $count } елемент(ів) готові до перенесення. Виберіть папку призначення, щоб почати.
drop-dialog-mode = Операція
drop-dialog-copy = Копіювати
drop-dialog-move = Перемістити
drop-dialog-pick-destination = Вибрати призначення
drop-dialog-change-destination = Змінити призначення
drop-dialog-start-copy = Почати копіювання
drop-dialog-start-move = Почати переміщення

# ETA placeholders
eta-calculating = обчислення…
eta-unknown = невідомо

# Toast messages
toast-job-done = Перенесення завершено
toast-copy-queued = Копіювання в черзі
toast-move-queued = Переміщення в черзі
toast-error-resolved = Помилку усунено
toast-collision-resolved = Конфлікт усунено
toast-elevated-unavailable = Повтор із підвищеними правами з'явиться у Phase 17 — поки що недоступно
toast-clipboard-files-detected = Файли в буфері обміну — натисніть комбінацію вставлення, щоб скопіювати через Freally File Manager
toast-clipboard-no-files = У буфері обміну немає файлів для вставлення
toast-error-log-exported = Журнал помилок експортовано

# Error modal (Phase 8)
error-modal-title = Перенесення не вдалося
error-modal-retry = Повторити
error-modal-retry-elevated = Повторити з підвищеними правами
error-modal-skip = Пропустити
error-modal-skip-all-kind = Пропустити всі помилки цього типу
error-modal-abort = Перервати все
error-modal-path-label = Шлях
error-modal-code-label = Код
error-drawer-pending-count = Очікують інші помилки
error-drawer-toggle = Згорнути або розгорнути

# Error-kind labels (Phase 8). Source of truth — engine maps each
# `CopyErrorKind` to one of these keys via `localized_key()`.
err-not-found = Файл не знайдено
err-permission-denied = Доступ заборонено
err-disk-full = Диск призначення заповнений
err-interrupted = Операцію перервано
err-verify-failed = Перевірка після копіювання не вдалася
err-path-escape = Шлях відхилено — містить сегменти батьківського каталогу (..) або недопустимі байти
err-path-invalid-encoding = Шлях відхилено — рядок містить недопустимий UTF-8 / символи заміни
err-helper-invalid-json = Привілейований помічник отримав некоректний JSON; запит проігноровано
err-helper-grant-out-of-band = GrantCapabilities має оброблятися циклом виконання помічника, а не stateless-обробником
err-randomness-unavailable = Генератор випадкових чисел ОС дав збій; неможливо створити ідентифікатор сеансу
err-sparseness-mismatch = Не вдалося зберегти розріджену структуру на призначенні
err-io-other = Невідома помилка вводу-виводу

# Collision modal (Phase 8)
collision-modal-title = Файл уже існує
collision-modal-overwrite = Перезаписати
collision-modal-overwrite-if-newer = Перезаписати якщо новіший
collision-modal-skip = Пропустити
collision-modal-keep-both = Зберегти обидва
collision-modal-rename = Перейменувати…
collision-modal-apply-to-all = Застосувати до всіх
collision-modal-source = Джерело
collision-modal-destination = Призначення
collision-modal-size = Розмір
collision-modal-modified = Змінено
collision-modal-hash-check = Швидкий хеш (SHA-256)
collision-modal-hash-computing = Обчислення…
collision-modal-hash-identical = Ідентичні
collision-modal-hash-different = Відрізняються
collision-modal-rename-placeholder = Нова назва файлу
collision-modal-confirm-rename = Перейменувати

# Error log drawer (Phase 8)
error-log-title = Журнал помилок
error-log-empty = Помилок не зареєстровано
error-log-export-csv = Експорт CSV
error-log-export-txt = Експорт тексту
error-log-clear = Очистити журнал
error-log-col-time = Час
error-log-col-job = Завдання
error-log-col-path = Шлях
error-log-col-code = Код
error-log-col-message = Повідомлення
error-log-col-resolution = Розв'язання

# History drawer (Phase 9)
history-title = Історія
history-empty = Поки що немає записаних завдань
history-unavailable = Історія копіювання недоступна. Застосунок не зміг відкрити сховище SQLite під час запуску.
history-filter-any = будь-який
history-filter-kind = Тип
history-filter-status = Статус
history-filter-text = Пошук
history-refresh = Оновити
history-export-csv = Експорт CSV
history-purge-30 = Очистити > 30 днів
history-rerun = Повторити
history-detail-open = Деталі
history-detail-title = Деталі завдання
history-detail-empty = Немає записаних елементів
history-col-date = Дата
history-col-kind = Тип
history-col-src = Джерело
history-col-dst = Призначення
history-col-files = Файлів
history-col-size = Розмір
history-col-status = Статус
history-col-duration = Тривалість
history-col-error = Помилка
toast-history-exported = Історію експортовано
toast-history-rerun-queued = Повтор поставлено в чергу

# Totals drawer (Phase 10)
footer-totals = Підсумки
totals-title = Підсумки
totals-loading = Завантаження підсумків…
totals-card-bytes = Усього скопійовано байтів
totals-card-files = Файлів
totals-card-jobs = Завдань
totals-card-avg-rate = Середня пропускна здатність
totals-errors = помилок
totals-spark-title = Останні 30 днів
totals-kinds-title = За типом
totals-saved-title = Зекономлено часу (орієнтовно)
totals-saved-note = Орієнтовно порівняно з базовим копіюванням того ж обсягу у файловому менеджері.
totals-reset = Скинути статистику
totals-reset-confirm = Це видалить усі збережені завдання й елементи. Продовжити?
totals-reset-confirm-yes = Так, скинути
toast-totals-reset = Статистику скинуто

# Phase 11a — i18n core: surface remaining user-visible strings so
# every string on a main-window screen flows through Fluent.

# Header language switcher (temporary placement — folds into
# Settings → General in Phase 12).
header-language-label = Мова
header-language-title = Змінити мову

# Job-kind labels. History filter, Totals breakdown, and history
# rows all map wire-format `kind` strings to these labels.
kind-copy = Копіювання
kind-move = Переміщення
kind-delete = Видалення
kind-secure-delete = Безпечне видалення

# History status labels. Distinct from `state-*` because the
# history wire format uses plain `running`/`succeeded`/... rather
# than the live `JobState` enum the queue exposes.
status-running = Виконується
status-succeeded = Успішно
status-failed = Збій
status-cancelled = Скасовано
# Per-item status (not job-level): `ok` / `skipped` are only
# reachable from the history detail view.
status-ok = OK
status-skipped = Пропущено

# History drawer: search field placeholder + purge toast.
history-search-placeholder = /шлях
toast-history-purged = Очищено завдань: { $count } старших за 30 днів

# User-facing command-layer validation errors. The Rust side
# returns these keys when input is missing; the toast layer looks
# them up rather than showing the raw English.
err-source-required = Потрібен щонайменше один шлях джерела.
err-destination-empty = Шлях призначення порожній.
err-source-empty = Шлях джерела порожній.

# Localised duration formatting for the Totals drawer and ETA
# fields. `{ $ms }`, `{ $s }`, `{ $m }`, `{ $h }` are integer
# placeables — the formatter passes pre-computed values in.
duration-lt-1s = < 1 с
duration-ms = { $ms } мс
duration-seconds = { $s } с
duration-minutes-seconds = { $m } хв { $s } с
duration-hours-minutes = { $h } год { $m } хв
duration-zero = 0 с

# Rate unit. Appended to a formatted byte size. Some languages
# render this with a leading space ("Ko/s"); keep it translatable
# even though the SI-derived "/s" is near-universal.
rate-unit-per-second = { $size }/с

# Phase 11b — Settings modal skeleton. Phase 12 expanded this into a
# full six-tab preferences window; the `settings-phase-12-hint` key
# is retired in favour of `settings-tab-profiles` + concrete labels.
settings-title = Налаштування
settings-tab-general = Загальні
settings-tab-appearance = Вигляд
settings-section-language = Мова
settings-phase-12-hint = Більше налаштувань (тема, типові параметри перенесення, алгоритм перевірки, профілі) з'явиться у Phase 12.

# Phase 12 — full Settings window keys. Grouped by tab so the
# human-review pass can pick one section at a time. Every field
# label and every option label that the user will actually read
# has its own key; dynamic wire values (enum discriminants like
# "auto" / "prefer") stay untranslated on the wire.

settings-loading = Завантаження налаштувань…
settings-tab-transfer = Перенесення
settings-tab-filters = Фільтри
settings-tab-shell = Оболонка
settings-tab-secure-delete = Безпечне видалення
settings-tab-advanced = Додатково
settings-tab-updater = Оновлення
settings-tab-profiles = Профілі

# General tab additions
settings-section-theme = Тема
settings-theme-auto = Авто
settings-theme-light = Світла
settings-theme-dark = Темна
settings-start-with-os = Запускати під час старту системи
settings-single-instance = Єдиний запущений екземпляр
settings-minimize-to-tray = Згортати в трей під час закриття
settings-error-display-mode = Стиль сповіщення про помилки
settings-error-display-modal = Модальне (блокує застосунок)
settings-error-display-drawer = Панель (не блокує)
settings-error-display-mode-hint = Модальне зупиняє чергу, доки ви не вирішите. Панель не зупиняє чергу й дає змогу розбирати помилки в кутку.
settings-paste-shortcut = Вставляти файли глобальною комбінацією
settings-paste-shortcut-combo = Комбінація клавіш
settings-paste-shortcut-hint = Натисніть цю комбінацію будь-де в системі, щоб вставити файли, скопійовані з Explorer / Finder / Files, через Freally File Manager. CmdOrCtrl відповідає Cmd на macOS, Ctrl на Windows / Linux.
settings-clipboard-watcher = Стежити за буфером обміну на копійовані файли
settings-clipboard-watcher-hint = Показувати сповіщення, коли в буфері обміну з'являються URL-адреси файлів, натякаючи, що можна вставити через Freally File Manager. Опитує кожні 500 мс, коли ввімкнено.

# Transfer tab
settings-buffer-size = Розмір буфера
settings-verify = Перевіряти після копіювання
settings-verify-off = Вимкнено
settings-concurrency = Паралелізм
settings-concurrency-auto = Авто
settings-reflink = Reflink / швидкі шляхи
settings-reflink-prefer = Надавати перевагу
settings-reflink-avoid = Уникати reflink
settings-reflink-disabled = Завжди використовувати асинхронний рушій
settings-fsync-on-close = Синхронізувати на диск під час закриття (повільніше, безпечніше)
settings-preserve-timestamps = Зберігати позначки часу
settings-preserve-permissions = Зберігати дозволи
settings-preserve-acls = Зберігати ACL (Phase 14)
settings-preserve-sparseness = Зберігати розріджені файли
settings-preserve-sparseness-hint = Копіювати лише виділені екстенти розріджених файлів (диски ВМ, файли баз даних), щоб призначення мало той самий розмір на диску, що й джерело.
settings-force-parallel-chunks = Паралельне багатоблокове копіювання (лише RAID / масиви)
settings-force-parallel-chunks-hint = Розбиває кожне велике копіювання на паралельні блоки. Корисно лише для черезсмугових/RAID/мережевих призначень; СПОВІЛЬНЮЄ одиночний SSD/NVMe (-25%…-76%). Залиште вимкненим, якщо призначення не є масивом із кількох дисків.

# Shell tab
settings-context-menu = Увімкнути пункти контекстного меню оболонки
settings-intercept-copy = Перехоплювати типовий обробник копіювання (Windows)
settings-intercept-copy-hint = Коли ввімкнено, Ctrl+C / Ctrl+V у Explorer проходять через Freally File Manager. Реєстрація з'явиться у Phase 14.
settings-notify-completion = Сповіщати про завершення завдання

# Secure delete tab
settings-shred-method = Типовий метод знищення
settings-shred-zero = Нулі (1 прохід)
settings-shred-random = Випадкові дані (1 прохід)
settings-shred-dod3 = DoD 5220.22-M (3 проходи)
settings-shred-dod7 = DoD 5220.22-M (7 проходів)
settings-shred-gutmann = Gutmann (35 проходів)
settings-shred-nist = NIST 800-88
settings-shred-confirm-twice = Вимагати подвійне підтвердження перед знищенням

# Advanced tab
settings-log-level = Рівень журналювання
settings-log-off = Вимкнено
settings-telemetry = Телеметрія
settings-telemetry-never = Ніколи — жодного зв'язку із сервером за будь-якого рівня журналювання
settings-error-policy = Типова політика помилок
settings-error-policy-ask = Питати
settings-error-policy-skip = Пропускати
settings-error-policy-retry = Повторювати з відкатом
settings-error-policy-abort = Переривати за першого збою
settings-history-retention = Зберігання історії (днів)
settings-history-retention-hint = 0 = зберігати назавжди. Будь-яке інше значення автоматично очищає старіші завдання під час запуску.
settings-database-path = Шлях до бази даних
settings-database-path-default = (типово — каталог даних ОС)
settings-reset-all = Скинути до типових
settings-reset-confirm = Скинути всі налаштування до типових? Профілі не зачіпаються.

# Profiles tab
settings-profiles-hint = Збережіть поточні налаштування під назвою; завантажте їх пізніше, щоб повернутися, не торкаючись окремих параметрів.
settings-profile-name-placeholder = Назва профілю
settings-profile-save = Зберегти
settings-profile-import = Імпорт…
settings-profile-load = Завантажити
settings-profile-export = Експорт…
settings-profile-delete = Видалити
settings-profile-empty = Поки що немає збережених профілів.
settings-profile-import-prompt = Назва для імпортованого профілю:

# Toasts driven by Phase 12 profile actions
toast-settings-reset = Налаштування скинуто
toast-profile-saved = Профіль збережено
toast-profile-loaded = Профіль завантажено
toast-profile-exported = Профіль експортовано
toast-profile-imported = Профіль імпортовано

# Phase 14a — enumeration-time filters
settings-filters-hint = Пропускайте файли під час переліку, щоб рушій навіть не відкривав їх. Включення стосуються лише файлів; виключення також відсікають відповідні каталоги.
settings-filters-enabled = Увімкнути фільтри для копіювання дерев
settings-filters-include-globs = Включити шаблони
settings-filters-include-globs-placeholder = **/*.txt
settings-filters-include-globs-hint = Один шаблон на рядок. Коли непорожньо, файл має відповідати хоча б одному включенню, щоб уціліти. У каталоги завжди заходимо.
settings-filters-exclude-globs = Виключити шаблони
settings-filters-exclude-globs-placeholder = **/node_modules
settings-filters-exclude-globs-hint = Один шаблон на рядок. Збіги відсікають усе піддерево для каталогів; відповідні файли пропускаються.
settings-filters-size-range = Діапазон розміру файлу
settings-filters-min-size-bytes = Мінімальний розмір (байтів, порожньо = без нижньої межі)
settings-filters-max-size-bytes = Максимальний розмір (байтів, порожньо = без верхньої межі)
settings-filters-date-range = Діапазон часу зміни
settings-filters-min-mtime = Змінено в цей день або пізніше
settings-filters-max-mtime = Змінено в цей день або раніше
settings-filters-attributes = Біти атрибутів
settings-filters-skip-hidden = Пропускати приховані файли / папки
settings-filters-skip-system = Пропускати системні файли (лише Windows)
settings-filters-skip-readonly = Пропускати файли лише для читання

# Phase 15 — auto-update
settings-updater-hint = Freally File Manager перевіряє підписані оновлення не частіше ніж раз на день. Оновлення встановлюються під час наступного виходу із застосунку.
settings-updater-auto-check = Перевіряти оновлення під час запуску
settings-updater-channel = Канал випуску
settings-updater-channel-stable = Стабільний
settings-updater-channel-beta = Бета (попередній випуск)
settings-updater-last-check = Остання перевірка
settings-updater-last-never = Ніколи
settings-updater-check-now = Перевірити оновлення зараз
settings-updater-checking = Перевірка…
settings-updater-available = Доступне оновлення
settings-updater-up-to-date = У вас найновіший випуск.
settings-updater-dismiss = Пропустити цю версію
settings-updater-dismissed = Пропущено
toast-update-available = Доступна новіша версія
toast-update-up-to-date = У вас уже найновіша версія

# Phase 19a — disk-backed file enumeration (TeraCopy-compatible scan DB)
scan-progress-title = Сканування…
scan-progress-stats = { $files } файлів · { $bytes } опрацьовано
scan-pause-button = Призупинити сканування
scan-resume-button = Відновити сканування
scan-cancel-button = Скасувати сканування
scan-cancel-confirm = Скасувати сканування та відкинути поступ?
scan-db-header = База даних сканування
scan-db-hint = Дискова база даних сканування для завдань на мільйони файлів.
advanced-scan-hash-during = Обчислювати контрольні суми під час сканування
advanced-scan-db-path = Розташування бази даних сканування
advanced-scan-retention-days = Автоматично видаляти завершені сканування через (днів)
advanced-scan-max-keep = Максимум баз даних сканування для зберігання

# Phase 19b — filesystem-snapshot source for locked files.
settings-on-locked = Коли файл заблоковано
settings-on-locked-ask = Питати першого разу
settings-on-locked-retry = Коротко повторити, потім показати помилку
settings-on-locked-skip = Пропустити заблокований файл
settings-on-locked-snapshot = Використати знімок файлової системи
settings-on-locked-hint = Усуває помилки «файл використовується іншим процесом». Freally File Manager робить знімок тому джерела (VSS на Windows, ZFS/Btrfs на Linux, APFS на macOS) і читає з копії знімка.
snapshot-prompt-title = Цей файл використовується іншим процесом
snapshot-prompt-body = Інша програма тримає { $path } відкритим для ексклюзивного запису. Виберіть, як Freally File Manager має поводитися з цим та подібними файлами на тому ж томі.
snapshot-source-active = 📷 Читання зі знімка { $kind } тому { $volume }
snapshot-create-failed = Не вдалося створити знімок тому джерела
snapshot-vss-needs-elevation = Читання зі знімка VSS потребує прав адміністратора. Freally File Manager попросить дозвіл.
snapshot-cleanup-failed = Помічник знімків повідомив про збій очищення — на томі може залишитися тіньова копія.

# Phase 20 — durable resume journal.
resume-prompt-title = Відновити попередні перенесення?
resume-prompt-body = Freally File Manager виявив { $count } незавершене(их) перенесення з попереднього сеансу. Виберіть, що робити з кожним.
resume-prompt-resume = Відновити
resume-prompt-resume-all = Відновити всі
resume-discard-one = Не відновлювати
resume-discard-all = Відкинути всі
resume-aborted-hash-mismatch = Перші { $offset } байтів призначення не збігаються з джерелом — починаємо спочатку.
settings-auto-resume = Автоматично відновлювати перервані завдання без запиту
settings-auto-resume-hint = Пропускати запит про відновлення під час запуску й тихо ставити в чергу кожне незавершене завдання. Типово вимкнено.

# Phase 21 — bandwidth shaping (GCRA token bucket + schedule + auto-throttle).
settings-tab-network = Мережа
settings-network-hint = Обмежте швидкість перенесення, щоб решта мережі залишалася придатною для використання. Застосовуйте глобально, дотримуйтеся щоденного розкладу або реагуйте автоматично на лімітований Wi-Fi / батарею / мобільне з'єднання.
settings-network-mode = Обмеження пропускної здатності
settings-network-mode-off = Вимкнено (без обмеження)
settings-network-mode-fixed = Фіксоване значення
settings-network-mode-schedule = За розкладом
settings-network-cap-mbps = Обмеження (МБ/с)
settings-network-schedule = Розклад (формат rclone)
settings-network-schedule-hint = Розділені пробілами межі HH:MM,швидкість плюс необов'язкові правила днів Mon-Fri,швидкість. Швидкості: 512k, 10M, 2G, off, unlimited. Приклад: 08:00,512k 18:00,10M Sat-Sun,unlimited.
settings-network-auto-header = Автообмеження
settings-network-auto-metered = На лімітованому Wi-Fi
settings-network-auto-battery = Від батареї
settings-network-auto-cellular = На мобільному з'єднанні
settings-network-auto-unchanged = Не перевизначати
settings-network-auto-pause = Призупинити перенесення
settings-network-auto-cap = Обмежити до фіксованого значення
shape-badge-paused = призупинено
shape-badge-tooltip = Обмеження пропускної здатності активне — клацніть, щоб відкрити Налаштування → Мережа
shape-badge-source-schedule = за розкладом
shape-badge-source-metered = лімітоване
shape-badge-source-battery = від батареї
shape-badge-source-cellular = мобільне
shape-badge-source-settings = активне
shape-error-schedule-invalid = Формат розкладу недійсний: { $message }

# Phase 22 — aggregate conflict dialog v2 (thumbnails, per-pattern
# rules, and reusable conflict profiles). Every key below is user-
# visible text in the `ConflictBatchModal.svelte` component.
conflict-batch-title = { $count } конфліктів файлів у { $jobname }
conflict-batch-state-pending = Очікує
conflict-batch-state-resolved = Розв'язано
conflict-batch-action-overwrite = Перезаписати
conflict-batch-action-skip = Пропустити
conflict-batch-action-keep-both = Зберегти обидва
conflict-batch-action-newer-wins = Перемагає новіший
conflict-batch-action-larger-wins = Перемагає більший
conflict-batch-bulk-apply-selected = Застосувати до вибраних
conflict-batch-bulk-apply-extension = Застосувати до всіх із цим розширенням
conflict-batch-bulk-apply-glob = Застосувати за шаблоном…
conflict-batch-bulk-apply-remaining = Застосувати до всіх решти
conflict-batch-bulk-glob-placeholder = напр. **/*.tmp
conflict-batch-save-profile = Зберегти ці правила як профіль…
conflict-batch-profile-placeholder = Назва профілю
conflict-batch-matched-rule = за правилом '{ $rule }' → { $action }
conflict-batch-empty = Усі конфлікти розв'язано
conflict-batch-source-vs-destination = Джерело проти призначення
conflict-batch-source-label = Джерело
conflict-batch-destination-label = Призначення
conflict-batch-size-label = Розмір
conflict-batch-modified-label = Змінено
conflict-batch-close = Закрити
conflict-batch-profile-saved = Профіль конфліктів збережено

# Phase 23 — sparse-file preservation. The toast fires once per
# destination volume when the filesystem can't preserve holes; the
# warning line is surfaced in the job detail drawer so the user knows
# the dst is larger on disk than the source was.
sparse-not-supported-title = Призначення заповнює розріджені файли
sparse-not-supported-body = { $dst_fs } не підтримує розріджені файли. Порожнини в джерелі записано нулями, тому призначення займає більше місця на диску.
sparse-warning-densified = Розріджену структуру збережено: скопійовано лише виділені екстенти.
sparse-warning-mismatch = Невідповідність розрідженої структури — призначення може бути більшим, ніж очікувалося.

# Phase 24 — security-metadata preservation. The Mark-of-the-Web
# (Zone.Identifier ADS) toggle is security-sensitive: turning it off
# lets a downloaded executable shed its SmartScreen / Office Protected
# View flag on copy, which is why the tooltip carries an explicit
# warning. AppleDouble fallback emits `._<filename>` sidecars on
# destination filesystems that can't hold the foreign metadata.
settings-preserve-security-metadata = Зберігати метадані безпеки
settings-preserve-security-metadata-hint = Захоплювати й повторно застосовувати позасмугові потоки метаданих (NTFS ADS / xattrs / POSIX ACL / контексти SELinux / можливості файлів Linux / гілки ресурсів macOS) під час кожного копіювання.
settings-preserve-motw = Зберігати Mark-of-the-Web (позначку «завантажено з інтернету»)
settings-preserve-motw-hint = Критично для безпеки. SmartScreen та Office Protected View використовують цей потік, щоб попереджати про файли, завантажені з інтернету. Вимкнення дає змогу завантаженому виконуваному файлу скинути позначку походження під час копіювання й обійти захист операційної системи.
settings-preserve-posix-acls = Зберігати POSIX ACL та розширені атрибути
settings-preserve-posix-acls-hint = Переносити xattrs user.* / system.* / trusted.* та списки контролю доступу POSIX під час копіювання.
settings-preserve-selinux = Зберігати контексти SELinux
settings-preserve-selinux-hint = Переносити мітку security.selinux під час копіювання, щоб демони під політиками MAC і далі мали доступ до файлу.
settings-preserve-resource-forks = Зберігати гілки ресурсів macOS та інформацію Finder
settings-preserve-resource-forks-hint = Переносити застарілу гілку ресурсів та FinderInfo (кольорові теги, метадані Carbon) під час копіювання.
settings-appledouble-fallback = Використовувати супутній файл AppleDouble на несумісних файлових системах
meta-translated-to-appledouble = Сторонні метадані збережено в супутньому файлі AppleDouble (._{ $ext })

# Phase 25 — two-way sync with vector-clock conflict detection.
# The drawer lists configured sync pairs; each pair runs an
# independent reconciliation round against a per-pair `.copythat-sync.db`
# state store. Concurrent edits from a common ancestor surface as
# conflicts rather than silent overwrites; the losing side's content
# is preserved as `name.sync-conflict-YYYYMMDD-HHMMSS-<host>.ext`.
footer-sync = Синхронізація
sync-drawer-title = Двостороння синхронізація
sync-drawer-hint = Тримайте дві папки синхронізованими без тихих перезаписів. Одночасні зміни постають як конфлікти, які можна розв'язати.
sync-add-pair = Додати пару
sync-add-cancel = Скасувати
sync-refresh = Оновити
sync-add-save = Зберегти пару
sync-add-saving = Збереження…
sync-add-missing-fields = Мітка, лівий шлях та правий шлях обов'язкові.
sync-remove-confirm = Вилучити цю пару синхронізації? База даних стану зберігається; папки не зачіпаються.
sync-field-label = Мітка
sync-field-label-placeholder = напр. Документи ↔ NAS
sync-field-left = Ліва папка
sync-field-left-placeholder = Виберіть або вставте абсолютний шлях
sync-field-right = Права папка
sync-field-right-placeholder = Виберіть або вставте абсолютний шлях
sync-field-mode = Режим
sync-mode-two-way = Двосторонній
sync-mode-mirror-left-to-right = Дзеркало (ліворуч → праворуч)
sync-mode-mirror-right-to-left = Дзеркало (праворуч → ліворуч)
sync-mode-contribute-left-to-right = Внесок (ліворуч → праворуч, без видалень)
sync-no-pairs = Поки що не налаштовано пар синхронізації. Клацніть «Додати пару», щоб почати.
sync-loading = Завантаження налаштованих пар…
sync-never-run = Ніколи не запускалося
sync-running = Виконується
sync-run-now = Запустити зараз
sync-cancel = Скасувати
sync-remove-pair = Вилучити
sync-view-conflicts = Переглянути конфлікти ({ $count })
sync-conflicts-heading = Конфлікти
sync-no-conflicts = Немає конфліктів з останнього запуску.
sync-winner = Переможець
sync-side-left-to-right = ліворуч
sync-side-right-to-left = праворуч
sync-conflict-kind-concurrent-write = Одночасна зміна
sync-conflict-kind-delete-edit = Видалення ↔ зміна
sync-conflict-kind-add-add = Додано з обох сторін
sync-conflict-kind-corrupt-equal = Вміст розійшовся без нового запису
sync-resolve-keep-left = Зберегти лівий
sync-resolve-keep-right = Зберегти правий
sync-resolve-keep-both = Зберегти обидва
sync-resolve-three-way = Розв'язати через тристороннє злиття
sync-resolve-phase-53-tooltip = Інтерактивне тристороннє злиття для нетекстових файлів з'явиться у Phase 53.
sync-error-prefix = Помилка синхронізації

# Phase 26 — real-time mirror watcher. "Live mirror" starts a
# filesystem watcher on the pair's left side; every debounced event
# triggers a re-sync. The watcher filters vim swap files / Office
# lock files / atomic-save temp names so one logical save becomes
# exactly one sync round.
live-mirror-start = Запустити живе дзеркало
live-mirror-stop = Зупинити живе дзеркало
live-mirror-watching = Спостереження
live-mirror-toggle-hint = Автоматично пересинхронізовувати за кожної виявленої зміни у файловій системі. Один фоновий потік на активну пару.
watch-event-prefix = Зміна файлу
watch-overflow-recovered = Буфер спостерігача переповнився; повторний перелік для відновлення

# Phase 27 — content-defined chunk store. Enables delta-resume (a
# retry only re-writes chunks that actually changed) and same-job
# dedup (files sharing content blocks store those blocks once). The
# store is disk-backed under `<data-dir>/chunks/` by default and is
# the foundation for the Phase 49–51 moonshot repository phases.
chunk-store-section = Сховище фрагментів
chunk-store-enable = Увімкнути сховище фрагментів (дельта-відновлення й дедуплікація)
chunk-store-enable-hint = Розбиває кожен скопійований файл за вмістом (FastCDC) і зберігає фрагменти з адресацією за вмістом. Повтори переписують лише змінені фрагменти; файли зі спільним вмістом дедуплікуються автоматично.
chunk-store-location = Розташування сховища фрагментів
chunk-store-max-size = Максимальний розмір сховища фрагментів
chunk-store-prune = Видаляти фрагменти, старші за (днів)
chunk-store-savings = Зекономлено { $gib } ГіБ завдяки дедуплікації фрагментів
chunk-store-disk-usage = Використано { $size } у { $chunks } фрагментах

# Phase 28 — tray-resident Drop Stack. The stack is a persistent list
# of paths gathered from multiple sources (Explorer drag, main-window
# context menu, CLI, drag onto the Drop Stack window) that the user
# can dispatch to a destination in one go.
dropstack-window-title = Drop Stack
dropstack-tray-open = Drop Stack
dropstack-empty-title = Drop Stack порожній
dropstack-empty-hint = Перетягніть сюди файли з Explorer або клацніть правою кнопкою рядок завдання, щоб додати його.
dropstack-add-to-stack = Додати до Drop Stack
dropstack-copy-all-to = Копіювати все до…
dropstack-move-all-to = Перемістити все до…
dropstack-clear = Очистити стек
dropstack-remove-row = Вилучити зі стека
dropstack-path-missing-toast = Перетягнуто { $path } — файл більше не існує.
dropstack-always-on-top = Тримати Drop Stack завжди зверху
dropstack-show-tray-icon = Показувати піктограму Freally File Manager у треї
dropstack-open-on-start = Автоматично відкривати Drop Stack під час запуску застосунку
dropstack-count = { $count } шлях

# Phase 29 — spring-loaded folders + native DnD polish. The Settings
# → General tab carries the knobs; DropTarget / DestinationPicker
# Svelte components use the dropzone-* keys at runtime.
settings-dnd-heading = Перетягування
settings-dnd-spring-load = Автовідкривання папок під час перетягування
settings-dnd-spring-delay = Затримка автовідкривання (мс)
settings-dnd-thumbnails = Показувати ескізи під час перетягування
settings-dnd-invalid-highlight = Підсвічувати недопустимі цілі скидання
dropzone-invalid-title = Недопустима ціль скидання
dropzone-invalid-readonly = Призначення лише для читання
dropzone-picker-title = Виберіть призначення
dropzone-picker-up = Угору
dropzone-picker-path = Поточний шлях
dropzone-picker-root = Корені
dropzone-picker-use-this = Використати цю папку
dropzone-picker-empty = Немає підпапок
dropzone-picker-cancel = Скасувати

# Phase 30 — cross-platform path translation. Settings → Transfer
# exposes these under a "Cross-platform compatibility" subsection.
translate-heading = Кросплатформна сумісність
translate-unicode-label = Нормалізація Unicode
translate-unicode-auto = Автовизначення призначення
translate-unicode-windows = NFC (Windows / Linux)
translate-unicode-macos = Залишити як є (macOS / APFS)
translate-line-endings-label = Перетворювати закінчення рядків для текстових файлів
translate-line-endings-allowlist = Розширення текстових файлів
reserved-name-label = Обробка зарезервованих імен Windows
reserved-name-suffix = Додавати "_" (CON.txt → CON_.txt)
reserved-name-reject = Відхиляти й попереджати
long-path-label = Використовувати префікс довгих шляхів Windows (\\?\) понад 260 символів
long-path-hint = Деякі мережеві ресурси та застарілі інструменти не підтримують простір імен \\?\.

# Phase 31 — power-aware copying. Settings → Power & State tab + the
# header badge that renders "⏸ Paused — Zoom call detected" when the
# runner's power subscriber has paused or capped due to a policy match.
power-heading = Живлення та стан
power-enabled = Увімкнути правила з урахуванням живлення
power-battery-label = Від батареї
power-metered-label = На лімітованому Wi-Fi
power-cellular-label = На мобільному з'єднанні
power-presentation-label = Під час презентації (Zoom / Teams / Keynote)
power-fullscreen-label = Коли застосунок у повноекранному режимі
power-thermal-label = Коли ЦП троттлиться через нагрів
power-rule-continue = Продовжувати на повній швидкості
power-rule-pause = Призупинити всі завдання
power-rule-cap = Обмежити пропускну здатність
power-rule-cap-percent = Обмежити до відсотка поточної швидкості
power-reason-on-battery = від батареї
power-reason-metered-network = лімітована мережа
power-reason-cellular-network = мобільна мережа
power-reason-presenting = режим презентації
power-reason-fullscreen = повноекранний застосунок
power-reason-thermal-throttling = ЦП троттлиться

# Phase 32 — cloud backend matrix via OpenDAL. Settings → Remotes
# tab + the Add-backend wizard that writes one entry per remote into
# the keychain-backed credential store.
remote-heading = Віддалені бекенди
remote-add = Додати бекенд
remote-list-empty = Не налаштовано віддалених бекендів
remote-test = Перевірити з'єднання
remote-test-success = З'єднання успішне
remote-test-failed = З'єднання не вдалося
remote-remove = Вилучити бекенд
remote-name-label = Відображувана назва
remote-kind-label = Тип бекенда
remote-save = Зберегти бекенд
remote-cancel = Скасувати
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
backend-local-fs = Локальна файлова система
cloud-config-bucket = Сегмент
cloud-config-region = Регіон
cloud-config-endpoint = URL-адреса кінцевої точки
cloud-config-root = Кореневий шлях
cloud-error-invalid-config = Конфігурація бекенда недійсна
cloud-error-network = Мережева помилка під час звернення до бекенда
cloud-error-not-found = Об'єкт не знайдено за вказаним шляхом
cloud-error-permission = Доступ заборонено віддаленим бекендом
cloud-error-keychain = Не вдалося отримати доступ до сховища ключів ОС
settings-tab-remotes = Віддалені
settings-tab-mobile = Мобільний

# Phase 33 — mount Freally File Manager's chunk store + history archive as a
# read-only filesystem (FUSE / WinFsp). Surfaced by the History tab
# context menu's "Mount snapshot" action + the Settings → Advanced
# "Mount latest on launch" toggle.
mount-heading = Монтувати знімок
mount-action-mount = Монтувати знімок
mount-action-unmount = Демонтувати
mount-status-mounted = Змонтовано в { $path }
mount-error-unsafe-mountpoint = Шлях точки монтування небезпечний
mount-error-mountpoint-not-empty = Точка монтування має бути порожнім каталогом
mount-error-backend-unavailable = Бекенд монтування недоступний у цій системі
mount-error-archive-read = Не вдалося прочитати архів
mount-picker-title = Виберіть каталог точки монтування
mount-toast-mounted = Знімок змонтовано в { $path }
mount-toast-unmounted = Знімок демонтовано
mount-toast-failed = Монтування не вдалося: { $reason }
settings-mount-heading = Монтувати знімки
settings-mount-hint = Надати доступ до архіву історії як до файлової системи лише для читання. Phase 33b підключає потік виконання; ядрові бекенди FUSE/WinFsp з'являться у Phase 33c.
settings-mount-on-launch = Монтувати найновіший знімок під час запуску
settings-mount-on-launch-path = Шлях точки монтування
settings-mount-on-launch-path-placeholder = напр. C:\Mounts\copythat

# Phase 34 — enterprise-grade audit log export + WORM (write-once-read-
# many) tamper-resistance. Settings → Advanced → Audit log collects the
# format + destination + rotation + WORM toggle; runtime records job and
# file events into the tamper-evident chain-hashed sink.
settings-audit-heading = Журнал аудиту
settings-audit-hint = Журнал лише для додавання, захищений від підробки, для кожної події завдань і файлів. Формати: CSV, JSON-lines, RFC 5424 Syslog, ArcSight CEF та QRadar LEEF.
settings-audit-enable = Увімкнути журналювання аудиту
settings-audit-format = Формат журналу
settings-audit-format-json-lines = JSON lines (рекомендовано типово)
settings-audit-format-csv = CSV (зручно для таблиць)
settings-audit-format-syslog = Syslog (RFC 5424)
settings-audit-format-cef = CEF (ArcSight)
settings-audit-format-leef = LEEF 2.0 (IBM QRadar)
settings-audit-file-path = Шлях до файлу журналу
settings-audit-file-path-placeholder = напр. C:\ProgramData\CopyThat\audit.log
settings-audit-max-size = Ротація після (байтів, 0 = ніколи)
settings-audit-worm = Увімкнути режим WORM (write-once-read-many)
settings-audit-worm-hint = Застосовує прапор платформи «лише для додавання» (Linux chattr +a, macOS chflags uappnd, атрибут «лише для читання» Windows) після кожного створення чи ротації. Навіть адміністратор має явно зняти прапор, щоб скоротити журнал.
settings-audit-test-write = Тестовий запис
settings-audit-verify-chain = Перевірити ланцюжок
toast-audit-test-write-ok = Тестовий запис у журнал аудиту успішний
toast-audit-verify-ok = Ланцюжок аудиту перевірено, цілісність збережена
toast-audit-verify-failed = Перевірка ланцюжка аудиту виявила невідповідності

# Phase 35 — destination encryption via age (X25519 / passphrase /
# SSH) + on-the-fly zstd compression with a per-extension deny
# heuristic. Surfaced through Settings → Transfer → Encryption +
# Compression; engine short-circuits to the crypt pipeline when
# either stage is active.
settings-crypt-heading = Шифрування та стиснення
settings-crypt-hint = Перетворюйте вміст файлів, перш ніж вони потраплять у призначення. Шифрування використовує формат age; стиснення використовує zstd і може пропускати вже стиснені медіа за розширенням.
settings-crypt-encryption-mode = Шифрування
settings-crypt-encryption-off = Вимкнено
settings-crypt-encryption-passphrase = Парольна фраза (запит на початку копіювання)
settings-crypt-encryption-recipients = Ключі одержувачів з файлу
settings-crypt-encryption-hint = Парольні фрази зберігаються лише в пам'яті на час копіювання. Файли одержувачів містять по одному відкритому ключу age1… або ssh- на рядок.
settings-crypt-recipients-file = Шлях до файлу одержувачів
settings-crypt-recipients-file-placeholder = напр. C:\Users\me\recipients.txt
settings-crypt-compression-mode = Стиснення
settings-crypt-compression-off = Вимкнено
settings-crypt-compression-always = Завжди
settings-crypt-compression-smart = Розумне (пропускати вже стиснені медіа)
settings-crypt-compression-hint = Розумний режим пропускає jpg, mp4, zip, 7z та подібні формати, яким zstd не дає переваг. Режим «Завжди» стискає кожен файл на вибраному рівні.
settings-crypt-compression-level = Рівень zstd (1-22)
settings-crypt-compression-level-hint = Менші числа швидші; більші стискають сильніше. Рівень 3 відповідає типовому значенню CLI zstd.
compress-footer-savings = 💾 { $original } → { $compressed } (зекономлено { $percent }%)
compress-savings-toast = Стиснено на { $percent }% (зекономлено { $bytes })
crypt-toast-recipients-loaded = Завантажено одержувачів шифрування: { $count }
crypt-toast-recipients-error = Не вдалося завантажити одержувачів: { $reason }
crypt-toast-passphrase-required = Шифрування потребує парольної фрази перед початком копіювання
crypt-toast-passphrase-set = Парольну фразу шифрування збережено
crypt-footer-encrypted-badge = 🔒 Зашифровано (age)
crypt-footer-compressed-badge = 📦 Стиснено (zstd)

# Phase 36 — copythat CLI surface. Documented exit codes + the
# user-facing strings for plan/apply/verify/config. Help text stays
# in English (engineering accessibility) but error / info / status
# strings are localized so a localized desktop installation surfaces
# CLI errors in the same language as the GUI.
cli-help-tagline = Freally File Manager CLI — побайтово точне копіювання, синхронізація, перевірка та аудит файлів для конвеєрів CI/CD.
cli-help-exit-codes = Коди виходу: 0 успіх, 1 помилка, 2 очікування, 3 конфлікт, 4 збій перевірки, 5 мережа, 6 права, 7 диск повний, 8 скасування, 9 конфігурація.
cli-error-bad-args = copy/move потребує щонайменше одного джерела й призначення
cli-error-unknown-algo = Невідомий алгоритм перевірки: { $algo }
cli-error-missing-spec = --spec обов'язковий для plan/apply
cli-error-spec-parse = Не вдалося розібрати jobspec { $path }: { $reason }
cli-error-spec-empty-sources = Список джерел jobspec порожній
cli-info-shape-recorded = Профіль пропускної здатності "{ $rate }" записано; застосування підключається через copythat-shape
cli-info-stub-deferred = { $command } заплановано для подальшого підключення у Phase 36
cli-plan-summary = План: { $actions } дій, { $bytes } байтів; { $already_done } уже на місці
cli-plan-pending = План повідомляє про очікувані дії; запустіть знову з `apply` для виконання
cli-plan-already-done = План повідомляє, що робити нічого (ідемпотентно)
cli-apply-success = Apply завершено без помилок
cli-apply-failed = Apply завершено з однією або кількома помилками
cli-verify-ok = Перевірка успішна: { $algo } { $digest }
cli-verify-failed = Перевірка НЕ ВДАЛАСЯ для { $path } ({ $algo })
cli-config-set = Установлено { $key } = { $value }
cli-config-reset = Скинуто { $key } до типового
cli-config-unknown-key = Невідомий ключ конфігурації: { $key }
cli-completions-emitted = Доповнення оболонки для { $shell } виведено в stdout

# Phase 37 — desktop-side mobile companion. Settings → Mobile panel
# strings + the SAS-confirmation modal + push-notification toasts.
# The actual mobile UI lives in the Phase 37 follow-up Tauri Mobile
# target; the phone displays its own localized SAS prompt.
settings-mobile-heading = Мобільний компаньйон
settings-mobile-hint = Підключіть iPhone або Android-телефон, щоб переглядати історію, запускати збережені профілі та jobspec з Phase 36 й отримувати сповіщення про завершення.
settings-mobile-pair-toggle = Дозволити нові підключення
settings-mobile-pair-active = Сервер підключення активний — відскануйте QR за допомогою мобільного застосунку Freally File Manager
settings-mobile-pair-button = Почати підключення
settings-mobile-revoke-button = Відкликати
settings-mobile-no-pairings = Поки що немає підключених пристроїв
settings-mobile-pair-port = Порт прив'язки (0 = вибрати вільний)
pair-sas-prompt = На обох екранах мають відображатися однакові чотири емодзі. Торкніться «Збігається», якщо вони однакові.
pair-sas-confirm = Збігається
pair-sas-reject = Не збігається — скасувати
pair-toast-success = Підключено до { $device }
pair-toast-failed = Підключення не вдалося: { $reason }
push-toast-sent = Push надіслано на { $device }
push-toast-failed = Push на { $device } не вдалося: { $reason }

# Phase 38 — aggregate destination dedup + reflink fallback ladder.
# Settings → Transfer → Dedup panel + per-job-row badges (⚡
# Reflinked / 🔗 Hardlinked / 🧩 Chunk-shared / 📋 Copied) +
# pre-pass dedup-scan modal.
settings-dedup-heading = Дедуплікація призначення
settings-dedup-hint = Коли джерело й призначення на одному томі, Freally File Manager може клонувати файли на рівні файлової системи замість копіювання байтів. Reflink миттєвий і безпечний; hardlink швидший, але обидві назви спільно використовують стан.
settings-dedup-mode-auto = Авто-драбина (reflink → hardlink → фрагмент → копія)
settings-dedup-mode-reflink-only = Лише reflink
settings-dedup-mode-hardlink-aggressive = Агресивно (reflink + hardlink навіть для записуваних файлів)
settings-dedup-mode-off = Вимкнено (завжди побайтова копія)
settings-dedup-hardlink-policy = Політика hardlink
settings-dedup-prescan = Попередньо сканувати дерево призначення на дублікати вмісту
dedup-badge-reflinked = ⚡ Reflink
dedup-badge-hardlinked = 🔗 Hardlink
dedup-badge-chunk-shared = 🧩 Спільні фрагменти
dedup-badge-copied = 📋 Скопійовано
phase42-paranoid-verify-label = Параноїдальна перевірка
phase42-paranoid-verify-hint = Скидає кешовані сторінки призначення й перечитує з диска, щоб виявити брехню кешу запису та тиху пошкодженість. Приблизно на 50% повільніше за типову перевірку; типово вимкнено.
phase42-sharing-violation-retries-label = Спроби повтору для заблокованих файлів джерела
phase42-sharing-violation-retries-hint = Скільки разів повторювати, коли інший процес тримає файл джерела відкритим з ексклюзивним блокуванням. Відкат подвоюється з кожною спробою (типово 50 мс / 100 мс / 200 мс). Типово 3, як у Robocopy /R:3.
phase42-cloud-placeholder-warning = { $name } — це хмарний файл OneDrive. Його копіювання запустить завантаження — до { $size } через ваше мережеве з'єднання.
phase42-defender-exclusion-hint = Для максимальної пропускної здатності копіювання додайте папку призначення до винятків Microsoft Defender перед масовими перенесеннями. Див. docs/PERFORMANCE_TUNING.md.

# Phase 39 — Browser-accessible recovery UI. Settings → Advanced
# exposes these strings; the recovery server itself renders askama
# templates that always read English — these eight keys are the
# Settings prose the user actually reads in their preferred locale.
settings-recovery-heading = Вебінтерфейс відновлення
settings-recovery-enable = Увімкнути вебінтерфейс відновлення
settings-recovery-bind-address = Адреса прив'язки
settings-recovery-port = Порт (0 = вибрати вільний)
settings-recovery-show-url = Показати URL і токен
settings-recovery-rotate-token = Змінити токен
settings-recovery-allow-non-loopback = Дозволити прив'язку поза loopback
settings-recovery-non-loopback-warning = ПОПЕРЕДЖЕННЯ: увімкнення прив'язки поза loopback відкриває вебінтерфейс відновлення для локальної мережі. Будь-хто, хто дізнається токен, зможе переглядати історію файлів і завантажувати файли. Закрийте його через TLS або зворотний проксі, якщо локальна мережа недовірена.

# Phase 40 — SMB compression negotiation + cloud-VM offload helper.
# 6 SMB keys (header badge + Settings prose) + 6 cloud-offload keys
# (Remotes tab wizard for cross-cloud copy templates).
smb-compress-badge = 🗜 SMB-стиснення: { $algo }
smb-compress-badge-tooltip = Мережевий трафік до цього призначення стискається під час передавання (SMB 3.1.1).
smb-compress-toast-saved = Зекономлено { $bytes } у мережі
smb-compress-algo-unknown = невідомий алгоритм
settings-smb-compress-heading = Мережеве стиснення SMB
settings-smb-compress-hint = Автоматично узгоджувати стиснення трафіку SMB 3.1.1 на призначеннях UNC. Безкоштовна перевага на повільних каналах; ігнорується для локальних призначень.
cloud-offload-heading = Помічник розвантаження на хмарну ВМ
cloud-offload-hint = Під час копіювання безпосередньо між двома хмарами згенеруйте шаблон розгортання, що виконує копіювання з крихітної тимчасової ВМ у хмарі — байти ніколи не торкаються мережі вашого ноутбука.
cloud-offload-render-button = Згенерувати шаблон
cloud-offload-copy-clipboard = Копіювати в буфер обміну
cloud-offload-template-format = Формат шаблону
cloud-offload-self-destruct-warning = ВМ автоматично вимикається через { $minutes } хв — підтвердьте роль IAM і регіон перед розгортанням.

# Phase 41 — animated before/after tree-diff preview. The `Preview
# changes` modal renders the rolled-up plan before the engine starts
# work; 14 keys cover the title, the summary header / counts, the
# row-reason labels, and the two action buttons.
preview-modal-title = Попередній перегляд змін
preview-summary-header = Що станеться
preview-category-additions = { $count } додавань
preview-category-replacements = { $count } замін
preview-category-skips = { $count } пропущено
preview-category-conflicts = { $count } конфліктів
preview-category-unchanged = { $count } без змін
preview-bytes-to-transfer = { $bytes } до перенесення
preview-reason-source-newer = Джерело новіше
preview-reason-dest-newer = Призначення новіше — буде пропущено
preview-reason-content-different = Вміст відрізняється
preview-reason-identical = Ідентично джерелу
preview-button-run = Виконати план
preview-button-reduce = Скоротити мій план…

# Phase 42 — perceptual-hash visual-similarity dedup. Eight keys cover
# the pre-copy "looks visually identical" warning + the Settings panel.
perceptual-warn-title = Виглядає візуально ідентичним
perceptual-warn-body = { $name } у призначенні, схоже, збігається із зображенням джерела. Усе одно продовжити копіювання?
perceptual-warn-keep-both = Зберегти обидва
perceptual-warn-skip = Пропустити цей файл
perceptual-warn-overwrite = Усе одно перезаписати
perceptual-settings-heading = Дедуплікація за візуальною схожістю
perceptual-settings-hint = Виявляти візуально ідентичні зображення в призначенні перед їх перезаписом. Хеш перцептивний (розпізнає те саме зображення, пересохранене в іншому форматі), а не побайтово точний.
perceptual-settings-threshold-label = Поріг попередження (нижче = суворіший збіг)

# Phase 42 Part B — per-file rolling versions (Time Machine for any
# destination). 8 keys cover the version-list panel + retention picker.
version-list-heading = Попередні версії
version-list-empty = Немає попередніх версій цього файлу
version-list-restore = Відновити цю версію
version-retention-heading = Зберігати попередні версії під час перезапису
version-retention-none = Зберігати кожну версію назавжди
version-retention-last-n = Зберігати останні { $n } версій
version-retention-older-than-days = Видаляти версії, старші за { $days } днів
version-retention-gfs = Щогодинно { $h } · щодня { $d } · щотижня { $w } · щомісяця { $m }

# Phase 43 — forensic chain-of-custody manifests + BLAKE3 verified
# streaming. 14 keys cover the Settings → Provenance panel
# (heading, hint, toggles, TSA URL, signing-key management) plus
# the post-job manifest notification + the verify command's two
# headline result lines.
provenance-settings-heading = Криміналістичний ланцюжок зберігання
provenance-settings-hint = Підписуйте кожне завдання копіювання маніфестом BLAKE3 + ed25519. Рецензенти можуть пізніше повторно хешувати дерево призначення й довести, що жоден байт не змінився після копіювання.
provenance-settings-enable-default = Типово підписувати кожне нове завдання
provenance-settings-show-after-job = Показувати маніфест після кожного завершеного завдання
provenance-settings-tsa-url-label = Типова URL-адреса служби позначок часу RFC 3161
provenance-settings-tsa-url-hint = Необов'язково. Коли задано, маніфести містять безкоштовну позначку часу TSA, що доводить існування байтів у цей момент часу. Залиште порожнім, щоб пропустити.
provenance-settings-keys-heading = Ключі підпису
provenance-settings-keys-generate = Згенерувати новий ключ
provenance-settings-keys-import = Імпортувати ключ…
provenance-settings-keys-export = Експортувати відкритий ключ…
provenance-job-completed-title = Маніфест походження збережено
provenance-job-completed-body = Підписано файлів: { $count } → { $path }
provenance-verify-clean = Маніфест дійсний для { $count } файлів; підпис { $sig }; корінь merkle OK.
provenance-verify-tampered = Маніфест НЕДІЙСНИЙ — { $tampered } підроблено, { $missing } відсутні.
# Phase 43 post-review hardening — toast text for the Settings →
# Provenance buttons whose Tauri IPC has not yet landed.
provenance-action-staged = Phase 43 — підключення IPC для цієї дії з'явиться в подальшому коміті.

# Phase 44 — SSD-aware whole-drive sanitize (NVMe Sanitize / OPAL
# Crypto Erase) + the per-file shred refusal on copy-on-write
# filesystems. 16 keys cover the new "Drive sanitize" Settings
# subsection and the localized error messages.
sanitize-heading = Безпечне очищення всього диска
sanitize-hint = NVMe Sanitize, OPAL Crypto Erase та ATA Secure Erase стирають флеш-диск на рівні прошивки за мілісекунди. Перезапис кожного файлу безглуздий на флеші — багатопрохідне знищення лише зношує NAND. Використовуйте це для справжнього очищення.
sanitize-pick-device = Виберіть диск для очищення
sanitize-mode-label = Метод очищення
sanitize-mode-nvme-format = NVMe Format (з безпечним стиранням)
sanitize-mode-nvme-sanitize-block = NVMe Sanitize — Block Erase (повільно, кожна комірка)
sanitize-mode-nvme-sanitize-crypto = NVMe Sanitize — Crypto Erase (миттєво)
sanitize-mode-ata-secure-erase = ATA Secure Erase (застарілі SATA SSD)
sanitize-mode-opal-crypto-erase = TCG OPAL Crypto Erase (самошифрувальні диски)
sanitize-mode-apfs-crypto-erase = APFS Crypto Erase (зміна ключа FileVault, лише macOS)
sanitize-confirm-1 = Це знищить КОЖЕН байт на { $device }. Скасувати неможливо.
sanitize-confirm-2 = Я розумію, що всі розділи, усі файли та всі знімки на { $device } стануть назавжди нечитабельними.
sanitize-confirm-3 = Введіть назву моделі диска, щоб продовжити: { $model }
sanitize-running = Очищення { $device } ({ $mode }) — це може тривати від мілісекунд (crypto erase) до десятків хвилин (block erase). Не вимикайте живлення.
sanitize-completed = Очищення завершено — { $device } тепер порожній.
ssd-honest-shred-meaningless = Перезапис кожного файлу на файловій системі copy-on-write (Btrfs / ZFS / APFS) не може дістатися до базових блоків. Натомість використовуйте очищення всього диска плюс зміну ключа повного шифрування диска.
ssd-honest-advisory = Цей файл розташований на флеші. Перезапис кожного файлу зношує NAND і НЕ гарантує, що оригінальні комірки неможливо відновити. Для конфіденційних даних очистіть весь диск.

# Phase 44.1f post-review — placeholder toast for SanitizeTab
# buttons whose Tauri IPC has not yet landed.
sanitize-action-staged = Phase 44.1 — підключення IPC для цієї дії з'явиться в подальшому коміті.

# Phase 45.3 — named-queue tab strip (Subfeature A). Tabs surface
# once the QueueRegistry holds at least one queue; the synthesised
# default tab keeps legacy single-queue jobs reachable.
queue-tab-default = Типова
queue-tab-empty-state = Черги завдань
queue-badge-tooltip = Очікувані та виконувані завдання в цій черзі

# Phase 45.4 — drag-progress-merge (Subfeature B). Drag a queue tab
# onto another to merge their job lists. The default tab is neither
# draggable nor a drop target; only registry queues participate.
queue-drag-hint = Перетягніть на іншу чергу для злиття
queue-merge-confirm = Скиньте для злиття
queue-merge-toast = Черги об'єднано

# Phase 45.5 — F2-queue UX (Subfeature C). F2 toggles
# `auto_enqueue_next` so every fresh enqueue piles into the running
# queue rather than spawning a parallel one. Status pill renders in
# the Footer; pulsing dot renders on the running tab.
queue-f2-active-hint = Режим F2: кожне нове завдання потрапляє в цю чергу
queue-f2-toggled-on = Режим черги F2 УВІМКНЕНО — нові завдання приєднуються до поточної черги
queue-f2-toggled-off = Режим черги F2 ВИМКНЕНО — нові завдання створюють паралельні черги
queue-f2-status-bar = Режим черги F2: УВІМКНЕНО

# Phase 45.6 — tray destination targets (Subfeature D). Pinned
# destinations appear in the OS tray menu; clicking one arms it as
# the active drop target so the next file drop bypasses the
# DropStagingDialog. Settings → General hosts the list editor.
tray-target-section-title = Призначення в треї
tray-target-section-hint = Закріплені призначення з'являються в меню трея. Клацніть одне, щоб зробити його наступною ціллю скидання.
tray-target-empty = Поки що не закріплено призначень у треї.
tray-target-remove = Вилучити
tray-target-add-label = Мітка
tray-target-add-path = Шлях або URI бекенда
tray-target-add = Додати
tray-target-armed-toast = Скиньте наступний файл, щоб надіслати його до { $label }
tray-target-active-pill = → { $label }

# Phase 45.7 follow-up — pinned-destination validation errors. The
# `queue_pin_destination` IPC returns these Fluent keys verbatim
# when input fails the IPC-boundary checks; the toast layer renders
# them via `t(...)` (Toast.svelte detects kebab-case-lowercase and
# routes through the locale table). Phase 17e's `err-path-escape` /
# `err-destination-empty` cover the path-traversal + empty cases for
# `queue_route_job`; the keys below are pin-specific.
err-pinned-destination-label-empty = Мітка призначення в треї не може бути порожньою.
err-pinned-destination-path-empty = Шлях призначення в треї не може бути порожнім.
err-pinned-destination-label-too-long = Мітка призначення в треї надто довга (макс. 64 символи).
err-pinned-destination-path-too-long = Шлях призначення в треї надто довгий (макс. 1024 символи).
err-pinned-destination-label-invalid = Мітка призначення в треї містить недопустимі символи (новий рядок, повернення каретки або NUL).
err-pinned-destination-path-invalid = Шлях призначення в треї містить недопустимі символи (новий рядок, повернення каретки або NUL).
err-pinned-destination-too-many = Ви досягли межі в 50 призначень у треї. Вилучіть одне, щоб додати інше.

# Phase 46.6 — Settings → Plugins tab. The IPC layer in
# `apps/copythat-ui/src-tauri/src/plugin_commands.rs` enumerates the
# per-user plugin store under `<config_dir>/plugins/`; the
# `PluginsTab.svelte` component renders these strings against the
# returned manifest + grant state.
settings-tab-plugins = Плагіни
plugin-heading = Плагіни
plugin-hint = Ізольовані WASM-плагіни розширюють Freally File Manager власними хуками. Кожен плагін працює в межах обмежень ЦП і пам'яті на виклик і бачить лише надані вами можливості хоста.
plugin-list-empty = Поки що не встановлено плагінів.
plugin-enabled = Увімкнено
plugin-disabled = Вимкнено
plugin-hooks = Хуки
plugin-capabilities = Можливості
plugin-no-capabilities = (немає)
plugin-directory = Розташування
plugin-install-from-file = Установити з файлу…
plugin-install-from-url = Установити з URL…
plugin-url-wasm = WASM URL
plugin-url-manifest = URL маніфесту
plugin-url-hash = Хеш BLAKE3
plugin-url-preview = Попередній перегляд
plugin-url-confirm = Підтвердити встановлення

# Phase 31b — power-policy settings (Power tab).
settings-tab-power = Живлення
settings-power-hint = Обмежувати або призупиняти копіювання залежно від живлення: батарея, лімітна/мобільна мережа, презентація/повний екран або тепловий тротлінг ЦП.
settings-power-enabled = Увімкнути обмеження за живленням
settings-power-battery = Від батареї
settings-power-metered = У лімітній мережі
settings-power-cellular = У мобільній мережі
settings-power-presentation = Під час презентації
settings-power-fullscreen = У повноекранному режимі
settings-power-thermal = При тепловому тротлінгу
settings-power-continue = Продовжити
settings-power-pause = Пауза
err-server-not-implemented = Режим сервера ще недоступний.
err-webhook-not-implemented = Доставка вебхуків ще недоступна.

# Phase 47 — "why is this slow?" diagnostics (bottleneck badge + tooltip).
bottleneck-source-io = Джерело I/O
bottleneck-dest-io = Призначення I/O
bottleneck-network = Мережа
bottleneck-antivirus = Антивірус
bottleneck-cpu = CPU
bottleneck-thermal = Перегрів
bottleneck-unknown = Невідомо
diag-aria = Вузьке місце: { $cause }
diag-tooltip = Обмежено: { $cause } · { $rate }
diag-spark-aria = Пропускна здатність за останню хвилину
diag-keeping-up = Встигає
diag-label = Діагностика

# Phase 48 — server mode + observability (Settings → Server).
settings-tab-server = Сервер
server-hint = Запустіть Freally File Manager як файловий сервер без інтерфейсу. Виберіть протоколи для надання, задайте адресу та теку для роздавання й за потреби увімкніть автентифікацію.
server-protocols = Протоколи
server-bind-addr = Адреса прив'язки
server-root = Тека, що надається
server-readonly = Лише читання (забороняти завантаження та видалення)
server-auth-mode = Автентифікація
server-auth-none = Немає
server-auth-bearer = Токен Bearer
server-auth-basic = Базова (ім'я користувача + пароль)
server-auth-token = Токен
server-auth-user = Ім'я користувача
server-auth-password = Пароль
otel-endpoint = Кінцева точка OpenTelemetry
webhook-section = Webhooks
webhook-url = URL вебхука
webhook-add = Додати вебхук
webhook-remove = Вилучити
webhook-empty = Вебхуки не налаштовані.
webhook-pushover-token = Токен Pushover
webhook-pushover-user = Користувач Pushover
server-start = Запустити сервер
server-stop = Зупинити сервер
server-status-running = Працює на { $addr }
server-status-stopped = Зупинено
server-metrics-url = Метрики
err-server-no-protocols = Виберіть принаймні один протокол перед запуском сервера.
err-server-bind = Не вдалося прив'язати адресу сервера. Можливо, вона вже використовується.

# Library drawer (Phase 49) — unified content-addressed repository view.
footer-library = Бібліотека
library-title = Бібліотека
library-loading = Завантаження репозиторію…
library-unavailable = Репозиторій недоступний
library-tab-live = Наживо
library-tab-snapshots = Знімки
library-tab-versions = Версії
library-hero-savings = надано { $effective } ефективних даних · { $pct } заощаджено
library-hero-empty = { $chunks } фрагментів збережено — ще немає знімків
library-stat-stored = Збережено на диску
library-stat-effective = Ефективні дані
library-stat-snapshots = Знімки
library-stat-chunks = Унікальні фрагменти
library-snapshot-empty = Ще немає знімків
library-snapshot-files = { $n } файлів
library-version-path-ph = Шлях призначення…
library-version-load = Показати версії
library-version-empty = Немає версій для цього шляху
repo-kind-copy = Копія
repo-kind-sync = Синхронізація
repo-kind-version = Версія
repo-kind-backup = Резервна копія
# Phase 49o — snapshot diff / compare.
library-tab-compare = Порівняти
repo-change-added = Додано
repo-change-removed = Видалено
repo-change-modified = Змінено
repo-change-unchanged = Без змін
repo-diff-summary = { $added } додано · { $removed } видалено · { $modified } змінено
repo-diff-bytes-added = { $bytes } нових
repo-diff-pick-two = Виберіть два знімки для порівняння
# Phase 49r — statistics / reports.
library-tab-reports = Звіти
report-growth-title = Зростання сховища
report-by-kind-title = За типом
report-top-files-title = Топ файлів
report-dedup-ratio = Дедупльовано { $pct }%
report-export = Експорт звіту
report-exported = Звіт збережено в { $path }
report-file-versions = { $n } версій
# Phase 49p — pinning / prune.
repo-pin = Закріпити
repo-unpin = Відкріпити
repo-pinned-badge = Закріплено
repo-prune-title = Очистити
repo-prune-keep-last = Залишити найновіші
repo-prune-removed = Очищено знімків: { $n }
repo-prune-none = Немає чого очищати

# Phase 49c — джерела резервних копій.
library-tab-sources = Джерела
backup-add-source = Додати джерело…
backup-source-path-ph = Папка для резервної копії…
backup-exclude-ph = Шаблони виключення (через кому)
backup-now = Створити копію зараз
backup-remove = Вилучити
backup-empty = Ще немає джерел резервних копій
backup-never-run = Резервну копію не створювали
backup-last-run = Остання копія { $when }
backup-running = Створення копії… { $files } файлів
backup-toast-started = Створення копії { $label }…
backup-toast-completed = { $label } скопійовано: { $files } файлів
backup-toast-failed = Не вдалося створити копію { $label }: { $reason }
# Phase 49e — per-source retention + prune.
backup-retention = Зберігання
backup-retention-keep-all = Зберігати все
backup-retention-last = Зберігати останні { $n }
backup-retention-days = Старші за { $days } днів
backup-retention-gfs = Ротація GFS
backup-prune-now = Очистити зараз
backup-prune-none = Немає чого очищати
backup-prune-result = Видалено { $removed } знімків · звільнено { $bytes }
# Phase 49f — per-source scheduling.
backup-schedule = Розклад
backup-schedule-manual = Вручну
backup-schedule-hourly = Щогодини
backup-schedule-daily = Щоденно
backup-schedule-weekly = Щотижня
backup-next-run = Наступний запуск { $when }
backup-not-scheduled = Не заплановано
# Phase 49g — source filters.
backup-include-ph = Включити шаблони (через кому)
backup-skip-hidden = Пропускати приховані
# Phase 49q — notifications.
notify-title = Сповіщення
notify-on-success = У разі успіху
notify-on-failure = У разі помилки
notify-test = Надіслати тест
notify-test-sent = Тест надіслано до { $n } призначень

# Phase 49d — браузер відновлення.
restore-browse = Відновити…
restore-title = Відновити зі знімка
restore-select-all = Вибрати все
restore-dest = Відновити до
restore-confirm = Відновити { $n } файлів
restore-empty = Цей знімок не містить файлів
restore-conflict-body = { $count } вибраних файлів уже існують у місці призначення.
restore-conflict-overwrite = Перезаписати
restore-conflict-skip = Пропустити наявні
restore-conflict-keep-both = Зберегти обидва
restore-toast-done = Відновлено { $restored }, пропущено { $skipped }
restore-toast-failed = Помилка відновлення: { $reason }
snapshot-forget = Забути
snapshot-forget-toast = Знімок забуто — запустіть «Звільнити місце», щоб очистити
library-reclaim = Звільнити місце
# Phase 49i — full compaction.
library-compact = Повне ущільнення
library-compact-started = Ущільнення розпочато — дивіться Завдання
# Phase 49h — compression.
library-stat-compression = Зекономлено стисненням
storage-compression = Стиснення
storage-compression-off = Вимкнено
storage-compression-auto = Авто (пропускати нестисливі)
storage-compression-always = Завжди
storage-compression-restart = Застосується під час наступного запуску
# Phase 49j — tasks & progress center.
footer-tasks = Завдання
tasks-title = Завдання
tasks-empty = Поки немає завдань
tasks-running = Виконується
tasks-recent = Нещодавні
tasks-cancel = Скасувати
task-state-running = Виконується
task-state-completed = Завершено
task-state-failed = Збій
task-state-cancelled = Скасовано
# Phase 49k — repository setup/connect wizard.
repo-wizard-title = Підключити репозиторій
repo-wizard-create-tab = Створити новий
repo-wizard-connect-tab = Підключити наявний
repo-field-name = Назва
repo-field-path = Розташування
repo-field-password = Парольна фраза (необов'язково)
repo-action-create = Створити
repo-action-connect = Підключити
repo-action-browse = Огляд…
repo-switcher-label = Репозиторій
repo-action-forget = Забути
repo-action-change-pass = Змінити парольну фразу…
repo-password-old = Поточна парольна фраза
repo-password-new = Нова парольна фраза
repo-error-exists = У цьому розташуванні вже існує репозиторій
repo-error-not-found = У цьому розташуванні репозиторій не знайдено
repo-error-bad-pass = Неправильна парольна фраза
repo-note-no-encryption = Парольна фраза керує лише доступом; шифрування даних у стані спокою з'явиться в наступному випуску
repo-confirm-forget = Вилучити "{ $name }" зі списку? Ваші дані залишаться на диску.
repo-toast-created = Репозиторій "{ $name }" створено
repo-toast-connected = Підключено до "{ $name }"
repo-toast-pass-changed = Парольну фразу оновлено
# Phase 49l — Sources dashboard.
library-tab-overview = Огляд
library-source-empty = Поки немає джерел
library-source-unknown = (джерело не вказано)
library-source-snapshots = { $n } знімків
library-source-latest = Останній { $when }
# Phase 49n — verify & repair.
repo-action-verify = Перевірити
repo-action-verify-deep = Перевірити (читати всі дані)
repo-action-repair = Виправити…
repo-verify-clean = Перевірено { $files } файлів / { $chunks } блоків — пошкоджень немає
repo-verify-damaged = { $missing } відсутні, { $corrupt } пошкоджених блоків
repo-repair-confirm = Вилучити { $n } знімків, які більше не можна відновити?
repo-repair-removed = Вилучено { $n } пошкоджених знімків
repo-repair-none = Немає що виправляти — репозиторій чистий
repo-gc-done = Звільнено { $bytes } ({ $chunks } блоків)
restore-toast-partial = Відновлено { $restored }, пропущено { $skipped }, не вдалося { $failed }
