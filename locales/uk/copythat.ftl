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
