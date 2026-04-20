app-name = Copy That 2026
# MT
window-title = Copy That 2026
# MT
shred-ssd-advisory = Ostrzeżenie: cel znajduje się na dysku SSD. Wielokrotne nadpisywanie nie czyści niezawodnie pamięci flash, ponieważ mechanizmy wear-leveling i overprovisioning przemieszczają dane poza logiczny adres bloku. W przypadku nośników półprzewodnikowych lepiej użyć ATA SECURE ERASE, NVMe Format z Secure Erase lub pełnego szyfrowania dysku z późniejszym zniszczeniem klucza.

# MT
state-idle = Bezczynny
# MT
state-copying = Kopiowanie
# MT
state-verifying = Weryfikacja
# MT
state-paused = Wstrzymane
# MT
state-error = Błąd

# MT
state-pending = W kolejce
# MT
state-running = W toku
# MT
state-cancelled = Anulowano
# MT
state-succeeded = Ukończono
# MT
state-failed = Nieudane

# MT
action-pause = Wstrzymaj
# MT
action-resume = Wznów
# MT
action-cancel = Anuluj
# MT
action-pause-all = Wstrzymaj wszystkie zadania
# MT
action-resume-all = Wznów wszystkie zadania
# MT
action-cancel-all = Anuluj wszystkie zadania
# MT
action-close = Zamknij
# MT
action-reveal = Pokaż w folderze

# MT
menu-pause = Wstrzymaj
# MT
menu-resume = Wznów
# MT
menu-cancel = Anuluj
# MT
menu-remove = Usuń z kolejki
# MT
menu-reveal-source = Pokaż źródło w folderze
# MT
menu-reveal-destination = Pokaż cel w folderze

# MT
header-eta-label = Szacowany pozostały czas
# MT
header-toolbar-label = Ogólne sterowanie

# MT
footer-queued = aktywnych zadań
# MT
footer-total-bytes = w toku
# MT
footer-errors = błędów
# MT
footer-history = Historia

# MT
empty-title = Upuść pliki lub foldery do skopiowania
# MT
empty-hint = Przeciągnij elementy na okno. Poprosimy o cel, a następnie dodamy po jednym zadaniu na źródło.
# MT
empty-region-label = Lista zadań

# MT
details-drawer-label = Szczegóły zadania
# MT
details-source = Źródło
# MT
details-destination = Cel
# MT
details-state = Stan
# MT
details-bytes = Bajty
# MT
details-files = Pliki
# MT
details-speed = Szybkość
# MT
details-eta = Pozostały czas
# MT
details-error = Błąd

# MT
drop-dialog-title = Przenieś upuszczone elementy
# MT
drop-dialog-subtitle = { $count } element(ów) gotowych do przeniesienia. Wybierz folder docelowy, aby rozpocząć.
# MT
drop-dialog-mode = Operacja
# MT
drop-dialog-copy = Kopiuj
# MT
drop-dialog-move = Przenieś
# MT
drop-dialog-pick-destination = Wybierz cel
# MT
drop-dialog-change-destination = Zmień cel
# MT
drop-dialog-start-copy = Rozpocznij kopiowanie
# MT
drop-dialog-start-move = Rozpocznij przenoszenie

# MT
eta-calculating = obliczanie…
# MT
eta-unknown = nieznany

# MT
toast-job-done = Transfer zakończony
# MT
toast-copy-queued = Kopiowanie w kolejce
# MT
toast-move-queued = Przenoszenie w kolejce
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

# MT — Totals drawer (Phase 10)
footer-totals = Totals
totals-title = Totals
totals-loading = Loading totals…
totals-card-bytes = Total bytes copied
totals-card-files = Files
totals-card-jobs = Jobs
totals-card-avg-rate = Average throughput
totals-errors = errors
totals-spark-title = Last 30 days
totals-kinds-title = By kind
totals-saved-title = Time saved (estimated)
totals-saved-note = Estimated vs a baseline file-manager copy of the same workload.
totals-reset = Reset statistics
totals-reset-confirm = This deletes every stored job and item. Continue?
totals-reset-confirm-yes = Yes, reset
toast-totals-reset = Statistics reset
