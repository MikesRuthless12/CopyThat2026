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
# MT — Phase 8 toast messages
toast-error-resolved = Помилку усунено
# MT
toast-collision-resolved = Конфлікт розв'язано
# MT
toast-elevated-unavailable = Повтор із підвищеними правами з'явиться на етапі 17 — поки недоступно
toast-clipboard-files-detected = Файли в буфері обміну — натисніть скорочення вставки, щоб скопіювати через Copy That
toast-clipboard-no-files = У буфері обміну немає файлів для вставки
# MT
toast-error-log-exported = Журнал помилок експортовано

# MT — Error modal
error-modal-title = Передача не вдалася
# MT
error-modal-retry = Повторити
# MT
error-modal-retry-elevated = Повторити з підвищеними правами
# MT
error-modal-skip = Пропустити
# MT
error-modal-skip-all-kind = Пропускати всі помилки цього типу
# MT
error-modal-abort = Перервати все
# MT
error-modal-path-label = Шлях
# MT
error-modal-code-label = Код
error-drawer-pending-count = Ще помилки очікують
error-drawer-toggle = Згорнути або розгорнути

# MT — Error-kind labels
err-not-found = Файл не знайдено
# MT
err-permission-denied = Доступ заборонено
# MT
err-disk-full = Цільовий диск переповнений
# MT
err-interrupted = Операцію перервано
# MT
err-verify-failed = Перевірка після копіювання не вдалася
# MT
err-io-other = Невідома помилка вводу/виводу

# MT — Collision modal
collision-modal-title = Файл уже існує
# MT
collision-modal-overwrite = Перезаписати
# MT
collision-modal-overwrite-if-newer = Перезаписати, якщо новіший
# MT
collision-modal-skip = Пропустити
# MT
collision-modal-keep-both = Зберегти обидва
# MT
collision-modal-rename = Перейменувати…
# MT
collision-modal-apply-to-all = Застосувати до всіх
# MT
collision-modal-source = Джерело
# MT
collision-modal-destination = Призначення
# MT
collision-modal-size = Розмір
# MT
collision-modal-modified = Змінено
# MT
collision-modal-hash-check = Швидкий хеш (SHA-256)
# MT
collision-modal-rename-placeholder = Нове ім'я файлу
# MT
collision-modal-confirm-rename = Перейменувати

# MT — Error log drawer
error-log-title = Журнал помилок
# MT
error-log-empty = Помилки не зареєстровано
# MT
error-log-export-csv = Експортувати CSV
# MT
error-log-export-txt = Експортувати текст
# MT
error-log-clear = Очистити журнал
# MT
error-log-col-time = Час
# MT
error-log-col-job = Завдання
# MT
error-log-col-path = Шлях
# MT
error-log-col-code = Код
# MT
error-log-col-message = Повідомлення
# MT
error-log-col-resolution = Розв'язання

# MT — History drawer (Phase 9)
history-title = Історія
# MT
history-empty = Завдання ще не зареєстровано
# MT
history-unavailable = Історія копіювання недоступна. Не вдалося відкрити сховище SQLite під час запуску.
# MT
history-filter-any = будь-який
# MT
history-filter-kind = Тип
# MT
history-filter-status = Стан
# MT
history-filter-text = Пошук
# MT
history-refresh = Оновити
# MT
history-export-csv = Експортувати CSV
# MT
history-purge-30 = Видалити старші за 30 днів
# MT
history-rerun = Повторити
# MT
history-detail-open = Деталі
# MT
history-detail-title = Деталі завдання
# MT
history-detail-empty = Немає зареєстрованих елементів
# MT
history-col-date = Дата
# MT
history-col-kind = Тип
# MT
history-col-src = Джерело
# MT
history-col-dst = Призначення
# MT
history-col-files = Файли
# MT
history-col-size = Розмір
# MT
history-col-status = Стан
# MT
history-col-duration = Тривалість
# MT
history-col-error = Помилка

# MT
toast-history-exported = Історію експортовано
# MT
toast-history-rerun-queued = Повторне виконання поставлено в чергу

# MT — Totals drawer (Phase 10)
footer-totals = Підсумки
# MT
totals-title = Підсумки
# MT
totals-loading = Завантаження підсумків…
# MT
totals-card-bytes = Усього скопійованих байтів
# MT
totals-card-files = Файли
# MT
totals-card-jobs = Завдання
# MT
totals-card-avg-rate = Середня швидкість
# MT
totals-errors = помилки
# MT
totals-spark-title = Останні 30 днів
# MT
totals-kinds-title = За типом
# MT
totals-saved-title = Зекономлений час (оцінка)
# MT
totals-saved-note = Оцінка відносно копіювання того ж навантаження стандартним файловим менеджером.
# MT
totals-reset = Скинути статистику
# MT
totals-reset-confirm = Це видалить усі збережені завдання та елементи. Продовжити?
# MT
totals-reset-confirm-yes = Так, скинути
# MT
toast-totals-reset = Статистику скинуто

# MT — Phase 11a additions
header-language-label = Мова
# MT
header-language-title = Змінити мову

# MT
kind-copy = Копіювання
# MT
kind-move = Переміщення
# MT
kind-delete = Видалення
# MT
kind-secure-delete = Безпечне видалення

# MT
status-running = Виконується
# MT
status-succeeded = Успішно
# MT
status-failed = Невдача
# MT
status-cancelled = Скасовано
# MT
status-ok = ОК
# MT
status-skipped = Пропущено

# MT
history-search-placeholder = /шлях
# MT
toast-history-purged = Видалено { $count } завдань старших за 30 днів

# MT
err-source-required = Потрібен принаймні один шлях джерела.
# MT
err-destination-empty = Шлях призначення порожній.
# MT
err-source-empty = Шлях джерела порожній.

# MT
duration-lt-1s = < 1 с
# MT
duration-ms = { $ms } мс
# MT
duration-seconds = { $s } с
# MT
duration-minutes-seconds = { $m } хв { $s } с
# MT
duration-hours-minutes = { $h } год { $m } хв
# MT
duration-zero = 0 с

# MT
rate-unit-per-second = { $size }/с

# MT — Phase 11b Settings modal
settings-title = Налаштування
# MT
settings-tab-general = Загальні
# MT
settings-tab-appearance = Вигляд
# MT
settings-section-language = Мова
# MT
settings-phase-12-hint = Інші налаштування (тема, типові параметри передачі, алгоритм перевірки, профілі) з'являться на етапі 12.

# MT — Phase 12 Settings window
settings-loading = Завантаження налаштувань…
# MT
settings-tab-transfer = Передача
# MT
settings-tab-shell = Оболонка
# MT
settings-tab-secure-delete = Безпечне видалення
# MT
settings-tab-advanced = Додатково
# MT
settings-tab-profiles = Профілі

# MT
settings-section-theme = Тема
# MT
settings-theme-auto = Автоматично
# MT
settings-theme-light = Світла
# MT
settings-theme-dark = Темна
# MT
settings-start-with-os = Запускати при старті системи
# MT
settings-single-instance = Один запущений екземпляр
# MT
settings-minimize-to-tray = Згортати у трей під час закриття
settings-error-display-mode = Стиль сповіщення про помилку
settings-error-display-modal = Модальне вікно (блокує застосунок)
settings-error-display-drawer = Бічна панель (неблокуюча)
settings-error-display-mode-hint = Модальне вікно зупиняє чергу, поки ви не вирішите. Бічна панель тримає чергу активною та дозволяє сортувати помилки в кутку.
settings-paste-shortcut = Вставляти файли через глобальне скорочення
settings-paste-shortcut-combo = Комбінація скорочення
settings-paste-shortcut-hint = Натисніть цю комбінацію будь-де в системі, щоб вставити файли, скопійовані з Провідника / Finder / Файлів, через Copy That. CmdOrCtrl вирішується як Cmd на macOS та Ctrl на Windows / Linux.
settings-clipboard-watcher = Спостерігати за буфером обміну для скопійованих файлів
settings-clipboard-watcher-hint = Показує сповіщення, коли URL файлів з'являються в буфері обміну, підказуючи, що можна вставити через Copy That. Опитує кожні 500 мс при увімкненні.

# MT
settings-buffer-size = Розмір буфера
# MT
settings-verify = Перевіряти після копіювання
# MT
settings-verify-off = Вимкнено
# MT
settings-concurrency = Паралельність
# MT
settings-concurrency-auto = Авто
# MT
settings-reflink = Reflink / швидкі шляхи
# MT
settings-reflink-prefer = Надавати перевагу
# MT
settings-reflink-avoid = Уникати reflink
# MT
settings-reflink-disabled = Завжди використовувати async-рушій
# MT
settings-fsync-on-close = Синхронізувати на диск під час закриття (повільніше, надійніше)
# MT
settings-preserve-timestamps = Зберігати мітки часу
# MT
settings-preserve-permissions = Зберігати права доступу
# MT
settings-preserve-acls = Зберігати ACL (етап 14)

# MT
settings-context-menu = Увімкнути пункти контекстного меню оболонки
# MT
settings-intercept-copy = Перехоплювати стандартний обробник копіювання (Windows)
# MT
settings-intercept-copy-hint = Коли увімкнено, Ctrl+C / Ctrl+V у Провіднику йде через Copy That. Реєстрація на етапі 14.
# MT
settings-notify-completion = Сповіщати про завершення завдання

# MT
settings-shred-method = Типовий метод затирання
# MT
settings-shred-zero = Нулі (1 прохід)
# MT
settings-shred-random = Випадкові (1 прохід)
# MT
settings-shred-dod3 = DoD 5220.22-M (3 проходи)
# MT
settings-shred-dod7 = DoD 5220.22-M (7 проходів)
# MT
settings-shred-gutmann = Ґутманна (35 проходів)
# MT
settings-shred-nist = NIST 800-88
# MT
settings-shred-confirm-twice = Вимагати подвійне підтвердження перед затиранням

# MT
settings-log-level = Рівень журналу
# MT
settings-log-off = Вимкнено
# MT
settings-telemetry = Телеметрія
# MT
settings-telemetry-never = Ніколи — дані не надсилаються на жодному рівні
# MT
settings-error-policy = Типова політика помилок
# MT
settings-error-policy-ask = Питати
# MT
settings-error-policy-skip = Пропускати
# MT
settings-error-policy-retry = Повторити із затримкою
# MT
settings-error-policy-abort = Перервати при першій помилці
# MT
settings-history-retention = Зберігання історії (дні)
# MT
settings-history-retention-hint = 0 = зберігати завжди. Будь-яке інше значення автоматично видаляє старі завдання при запуску.
# MT
settings-database-path = Шлях до бази даних
# MT
settings-database-path-default = (типовий — каталог даних ОС)
# MT
settings-reset-all = Скинути до типових
# MT
settings-reset-confirm = Скинути всі налаштування? Профілі залишаються недоторканими.

# MT
settings-profiles-hint = Збережіть поточні налаштування під іменем; пізніше завантажте, щоб перемикатися без зміни окремих параметрів.
# MT
settings-profile-name-placeholder = Назва профілю
# MT
settings-profile-save = Зберегти
# MT
settings-profile-import = Імпорт…
# MT
settings-profile-load = Завантажити
# MT
settings-profile-export = Експорт…
# MT
settings-profile-delete = Видалити
# MT
settings-profile-empty = Збережених профілів немає.
# MT
settings-profile-import-prompt = Назва для імпортованого профілю:

# MT
toast-settings-reset = Налаштування скинуто
# MT
toast-profile-saved = Профіль збережено
# MT
toast-profile-loaded = Профіль завантажено
# MT
toast-profile-exported = Профіль експортовано
# MT
toast-profile-imported = Профіль імпортовано

# Phase 13d — activity feed + header picker buttons
action-add-files = Додати файли
action-add-folders = Додати папки
activity-title = Активність
activity-clear = Очистити список активності
activity-empty = Поки немає файлової активності.
activity-after-done = Після завершення:
activity-keep-open = Залишити застосунок відкритим
activity-close-app = Закрити застосунок
activity-shutdown = Вимкнути ПК
activity-logoff = Вийти з системи
activity-sleep = Сплячий режим

# Phase 14 — preflight free-space dialog
preflight-block-title = На цільовому томі недостатньо місця
preflight-warn-title = Мало місця на цільовому томі
preflight-unknown-title = Не вдалося визначити вільне місце
preflight-unknown-body = Джерело задовелике, щоб швидко виміряти, або цільовий том не відповів. Ви можете продовжити; захист рушія акуратно зупинить копіювання, якщо закінчиться місце.
preflight-required = Потрібно
preflight-free = Вільно
preflight-reserve = Резерв
preflight-shortfall = Нестача
preflight-continue = Все одно продовжити
collision-modal-overwrite-older = Перезаписувати лише старіші

# Phase 14e — subset picker
preflight-pick-subset = Вибрати, що копіювати…
subset-title = Виберіть джерела для копіювання
subset-subtitle = Повний набір не вміщується в призначенні. Позначте те, що хочете скопіювати; решта залишиться.
subset-loading = Вимірювання розмірів…
subset-too-large = завеликий для підрахунку
subset-budget = Доступно
subset-remaining = Залишилось
subset-confirm = Скопіювати вибране
history-rerun-hint = Повторити це копіювання — знову сканує всі файли в дереві джерела
history-clear-all = Очистити все
history-clear-all-confirm = Натисніть ще раз для підтвердження
history-clear-all-hint = Видаляє всі рядки історії. Потрібне друге натискання для підтвердження.
toast-history-cleared = Історію очищено ({ $count } рядків видалено)

# Phase 15 — source-list ordering
drop-dialog-sort-label = Порядок:
sort-custom = Власний
sort-name-asc = Ім'я А → Я (файли першими)
sort-name-desc = Ім'я Я → А (файли першими)
sort-size-asc = Розмір від меншого (файли першими)
sort-size-desc = Розмір від більшого (файли першими)
sort-reorder = Переупорядкувати
sort-move-top = Нагору
sort-move-up = Вгору
sort-move-down = Вниз
sort-move-bottom = Донизу
sort-name-asc-simple = Ім'я А → Я
sort-name-desc-simple = Ім'я Я → А
sort-size-asc-simple = Менші першими
sort-size-desc-simple = Більші першими
activity-sort-locked = Сортування вимкнено під час копіювання. Призупиніть або дочекайтеся завершення, потім змініть порядок.
drop-dialog-collision-label = Якщо файл уже існує:
collision-policy-keep-both = Зберегти обидва (перейменувати нову копію у _2, _3 …)
collision-policy-skip = Пропустити нову копію
collision-policy-overwrite = Перезаписати існуючий файл
collision-policy-overwrite-if-newer = Перезаписувати лише якщо новіший
collision-policy-prompt = Питати щоразу
drop-dialog-busy-checking = Перевірка вільного місця…
drop-dialog-busy-enumerating = Підрахунок файлів…
drop-dialog-busy-starting = Запуск копіювання…
toast-enumeration-deferred = Дерево джерела велике — пропускаємо попередній список; рядки з'являться під час обробки.

# Phase 14a — enumeration-time filters
# MT
settings-tab-filters = Фільтри
# MT
settings-filters-hint = Пропускає файли на етапі переліку, тож рушій їх навіть не відкриває. «Включити» діє лише на файли; «Виключити» також обрізає відповідні каталоги.
# MT
settings-filters-enabled = Увімкнути фільтри для копіювання дерева
# MT
settings-filters-include-globs = Глоби включення
# MT
settings-filters-include-globs-placeholder = **/*.txt
# MT
settings-filters-include-globs-hint = Один глоб на рядок. Якщо не порожньо, файл має збігтися щонайменше з одним. Каталоги завжди обходяться.
# MT
settings-filters-exclude-globs = Глоби виключення
# MT
settings-filters-exclude-globs-placeholder = **/node_modules
# MT
settings-filters-exclude-globs-hint = Один глоб на рядок. Збіги обрізають усе піддерево для каталогів; відповідні файли пропускаються.
# MT
settings-filters-size-range = Діапазон розміру файлу
# MT
settings-filters-min-size-bytes = Мін. розмір (байти, порожньо = без нижньої межі)
# MT
settings-filters-max-size-bytes = Макс. розмір (байти, порожньо = без верхньої межі)
# MT
settings-filters-date-range = Діапазон часу зміни
# MT
settings-filters-min-mtime = Змінено від
# MT
settings-filters-max-mtime = Змінено до
# MT
settings-filters-attributes = Атрибути
# MT
settings-filters-skip-hidden = Пропускати приховані файли / теки
# MT
settings-filters-skip-system = Пропускати системні файли (лише Windows)
# MT
settings-filters-skip-readonly = Пропускати файли лише для читання

# Phase 15 — auto-update
# MT
settings-tab-updater = Оновлення
# MT
settings-updater-hint = Copy That перевіряє підписані оновлення не частіше одного разу на добу. Оновлення встановлюються під час наступного виходу з програми.
# MT
settings-updater-auto-check = Перевіряти оновлення під час запуску
# MT
settings-updater-channel = Канал випуску
# MT
settings-updater-channel-stable = Стабільний
# MT
settings-updater-channel-beta = Бета (передвипуск)
# MT
settings-updater-last-check = Остання перевірка
# MT
settings-updater-last-never = Ніколи
# MT
settings-updater-check-now = Перевірити оновлення зараз
# MT
settings-updater-checking = Перевірка…
# MT
settings-updater-available = Доступне оновлення
# MT
settings-updater-up-to-date = Ви використовуєте останній випуск.
# MT
settings-updater-dismiss = Пропустити цю версію
# MT
settings-updater-dismissed = Пропущено
# MT
toast-update-available = Доступна нова версія
# MT
toast-update-up-to-date = У вас уже остання версія
