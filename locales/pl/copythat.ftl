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
# MT — Phase 8 toast messages
toast-error-resolved = Błąd rozwiązany
# MT
toast-collision-resolved = Konflikt rozwiązany
# MT
toast-elevated-unavailable = Ponowienie z podwyższonymi uprawnieniami pojawi się w fazie 17 — jeszcze niedostępne
# MT
toast-error-log-exported = Dziennik błędów wyeksportowany

# MT — Error modal
error-modal-title = Transfer nie powiódł się
# MT
error-modal-retry = Ponów
# MT
error-modal-retry-elevated = Ponów z podwyższonymi uprawnieniami
# MT
error-modal-skip = Pomiń
# MT
error-modal-skip-all-kind = Pomiń wszystkie błędy tego rodzaju
# MT
error-modal-abort = Przerwij wszystko
# MT
error-modal-path-label = Ścieżka
# MT
error-modal-code-label = Kod

# MT — Error-kind labels
err-not-found = Nie znaleziono pliku
# MT
err-permission-denied = Odmowa dostępu
# MT
err-disk-full = Dysk docelowy jest pełny
# MT
err-interrupted = Operacja przerwana
# MT
err-verify-failed = Weryfikacja po kopiowaniu nieudana
# MT
err-io-other = Nieznany błąd we/wy

# MT — Collision modal
collision-modal-title = Plik już istnieje
# MT
collision-modal-overwrite = Zastąp
# MT
collision-modal-overwrite-if-newer = Zastąp, jeśli nowszy
# MT
collision-modal-skip = Pomiń
# MT
collision-modal-keep-both = Zachowaj oba
# MT
collision-modal-rename = Zmień nazwę…
# MT
collision-modal-apply-to-all = Zastosuj do wszystkich
# MT
collision-modal-source = Źródło
# MT
collision-modal-destination = Cel
# MT
collision-modal-size = Rozmiar
# MT
collision-modal-modified = Zmodyfikowano
# MT
collision-modal-hash-check = Szybki skrót (SHA-256)
# MT
collision-modal-rename-placeholder = Nowa nazwa pliku
# MT
collision-modal-confirm-rename = Zmień nazwę

# MT — Error log drawer
error-log-title = Dziennik błędów
# MT
error-log-empty = Brak zarejestrowanych błędów
# MT
error-log-export-csv = Eksportuj CSV
# MT
error-log-export-txt = Eksportuj tekst
# MT
error-log-clear = Wyczyść dziennik
# MT
error-log-col-time = Czas
# MT
error-log-col-job = Zadanie
# MT
error-log-col-path = Ścieżka
# MT
error-log-col-code = Kod
# MT
error-log-col-message = Wiadomość
# MT
error-log-col-resolution = Rozwiązanie

# MT — History drawer (Phase 9)
history-title = Historia
# MT
history-empty = Brak zarejestrowanych zadań
# MT
history-unavailable = Historia kopiowania jest niedostępna. Aplikacja nie mogła otworzyć magazynu SQLite przy starcie.
# MT
history-filter-any = dowolny
# MT
history-filter-kind = Typ
# MT
history-filter-status = Stan
# MT
history-filter-text = Szukaj
# MT
history-refresh = Odśwież
# MT
history-export-csv = Eksportuj CSV
# MT
history-purge-30 = Usuń starsze niż 30 dni
# MT
history-rerun = Uruchom ponownie
# MT
history-detail-open = Szczegóły
# MT
history-detail-title = Szczegóły zadania
# MT
history-detail-empty = Brak zarejestrowanych elementów
# MT
history-col-date = Data
# MT
history-col-kind = Typ
# MT
history-col-src = Źródło
# MT
history-col-dst = Cel
# MT
history-col-files = Pliki
# MT
history-col-size = Rozmiar
# MT
history-col-status = Stan
# MT
history-col-duration = Czas trwania
# MT
history-col-error = Błąd

# MT
toast-history-exported = Historia wyeksportowana
# MT
toast-history-rerun-queued = Ponowne uruchomienie w kolejce

# MT — Totals drawer (Phase 10)
footer-totals = Podsumowanie
# MT
totals-title = Podsumowanie
# MT
totals-loading = Ładowanie podsumowania…
# MT
totals-card-bytes = Łącznie skopiowane bajty
# MT
totals-card-files = Pliki
# MT
totals-card-jobs = Zadania
# MT
totals-card-avg-rate = Średnia przepustowość
# MT
totals-errors = błędy
# MT
totals-spark-title = Ostatnie 30 dni
# MT
totals-kinds-title = Według typu
# MT
totals-saved-title = Zaoszczędzony czas (szacowany)
# MT
totals-saved-note = Szacowany w porównaniu z kopią odniesienia tego samego zadania za pomocą standardowego menedżera plików.
# MT
totals-reset = Resetuj statystyki
# MT
totals-reset-confirm = To usuwa wszystkie zapisane zadania i elementy. Kontynuować?
# MT
totals-reset-confirm-yes = Tak, resetuj
# MT
toast-totals-reset = Statystyki zresetowane

# MT — Phase 11a additions
header-language-label = Język
# MT
header-language-title = Zmień język

# MT
kind-copy = Kopiowanie
# MT
kind-move = Przenoszenie
# MT
kind-delete = Usuwanie
# MT
kind-secure-delete = Bezpieczne usuwanie

# MT
status-running = W toku
# MT
status-succeeded = Powodzenie
# MT
status-failed = Niepowodzenie
# MT
status-cancelled = Anulowano
# MT
status-ok = OK
# MT
status-skipped = Pominięto

# MT
history-search-placeholder = /ścieżka
# MT
toast-history-purged = Usunięto { $count } zadań starszych niż 30 dni

# MT
err-source-required = Wymagana jest co najmniej jedna ścieżka źródłowa.
# MT
err-destination-empty = Ścieżka docelowa jest pusta.
# MT
err-source-empty = Ścieżka źródłowa jest pusta.

# MT
duration-lt-1s = < 1 s
# MT
duration-ms = { $ms } ms
# MT
duration-seconds = { $s } s
# MT
duration-minutes-seconds = { $m } min { $s } s
# MT
duration-hours-minutes = { $h } godz. { $m } min
# MT
duration-zero = 0 s

# MT
rate-unit-per-second = { $size }/s

# MT — Phase 11b Settings modal
settings-title = Ustawienia
# MT
settings-tab-general = Ogólne
# MT
settings-tab-appearance = Wygląd
# MT
settings-section-language = Język
# MT
settings-phase-12-hint = Więcej ustawień (motyw, domyślne transferu, algorytm weryfikacji, profile) pojawi się w fazie 12.
