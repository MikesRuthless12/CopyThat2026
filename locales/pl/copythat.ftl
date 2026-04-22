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
toast-clipboard-files-detected = Pliki w schowku — naciśnij skrót wklejania, aby skopiować za pomocą Copy That
toast-clipboard-no-files = Schowek nie zawiera plików do wklejenia
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
error-drawer-pending-count = Więcej błędów czeka
error-drawer-toggle = Zwiń lub rozwiń

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
err-path-escape = Ścieżka odrzucona — zawiera segmenty katalogu nadrzędnego (..) lub niedozwolone bajty
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

# MT — Phase 12 Settings window
settings-loading = Ładowanie ustawień…
# MT
settings-tab-transfer = Transfer
# MT
settings-tab-shell = Powłoka
# MT
settings-tab-secure-delete = Bezpieczne usuwanie
# MT
settings-tab-advanced = Zaawansowane
# MT
settings-tab-profiles = Profile

# MT
settings-section-theme = Motyw
# MT
settings-theme-auto = Automatyczny
# MT
settings-theme-light = Jasny
# MT
settings-theme-dark = Ciemny
# MT
settings-start-with-os = Uruchom z systemem
# MT
settings-single-instance = Pojedyncza aktywna instancja
# MT
settings-minimize-to-tray = Minimalizuj do zasobnika przy zamknięciu
settings-error-display-mode = Styl okna błędu
settings-error-display-modal = Modalne (blokuje aplikację)
settings-error-display-drawer = Panel boczny (nieblokujący)
settings-error-display-mode-hint = Okno modalne wstrzymuje kolejkę do czasu decyzji. Panel boczny utrzymuje kolejkę w ruchu i pozwala zarządzać błędami w rogu.
settings-paste-shortcut = Wklej pliki za pomocą skrótu globalnego
settings-paste-shortcut-combo = Kombinacja skrótu
settings-paste-shortcut-hint = Naciśnij tę kombinację w dowolnym miejscu systemu, aby wkleić pliki skopiowane z Eksploratora / Findera / Plików za pomocą Copy That. CmdOrCtrl to Cmd na macOS i Ctrl na Windows / Linux.
settings-clipboard-watcher = Monitoruj schowek pod kątem skopiowanych plików
settings-clipboard-watcher-hint = Pokazuje powiadomienie, gdy adresy URL plików pojawią się w schowku, wskazując, że możesz wkleić za pomocą Copy That. Odpytuje co 500 ms, gdy włączone.

# MT
settings-buffer-size = Rozmiar bufora
# MT
settings-verify = Weryfikuj po kopiowaniu
# MT
settings-verify-off = Wyłączone
# MT
settings-concurrency = Równoległość
# MT
settings-concurrency-auto = Automatycznie
# MT
settings-reflink = Reflink / szybkie ścieżki
# MT
settings-reflink-prefer = Preferuj
# MT
settings-reflink-avoid = Unikaj reflink
# MT
settings-reflink-disabled = Zawsze używaj silnika async
# MT
settings-fsync-on-close = Synchronizuj na dysk przy zamknięciu (wolniej, bezpieczniej)
# MT
settings-preserve-timestamps = Zachowaj znaczniki czasu
# MT
settings-preserve-permissions = Zachowaj uprawnienia
# MT
settings-preserve-acls = Zachowaj ACL (faza 14)

# MT
settings-context-menu = Włącz wpisy menu kontekstowego
# MT
settings-intercept-copy = Przechwytuj domyślny uchwyt kopiowania (Windows)
# MT
settings-intercept-copy-hint = Gdy aktywne, Ctrl+C / Ctrl+V w Eksploratorze przechodzi przez Copy That. Rejestracja w fazie 14.
# MT
settings-notify-completion = Powiadamiaj po zakończeniu zadania

# MT
settings-shred-method = Domyślna metoda niszczenia
# MT
settings-shred-zero = Zero (1 przebieg)
# MT
settings-shred-random = Losowo (1 przebieg)
# MT
settings-shred-dod3 = DoD 5220.22-M (3 przebiegi)
# MT
settings-shred-dod7 = DoD 5220.22-M (7 przebiegów)
# MT
settings-shred-gutmann = Gutmann (35 przebiegów)
# MT
settings-shred-nist = NIST 800-88
# MT
settings-shred-confirm-twice = Wymagaj podwójnego potwierdzenia przed niszczeniem

# MT
settings-log-level = Poziom logowania
# MT
settings-log-off = Wyłączone
# MT
settings-telemetry = Telemetria
# MT
settings-telemetry-never = Nigdy — brak wysyłania danych na żadnym poziomie logowania
# MT
settings-error-policy = Domyślna polityka błędów
# MT
settings-error-policy-ask = Zapytaj
# MT
settings-error-policy-skip = Pomiń
# MT
settings-error-policy-retry = Ponów z opóźnieniem
# MT
settings-error-policy-abort = Przerwij przy pierwszym błędzie
# MT
settings-history-retention = Przechowywanie historii (dni)
# MT
settings-history-retention-hint = 0 = zachowaj na zawsze. Dowolna inna wartość usuwa stare zadania przy starcie.
# MT
settings-database-path = Ścieżka bazy danych
# MT
settings-database-path-default = (domyślna — katalog danych systemu)
# MT
settings-reset-all = Przywróć domyślne
# MT
settings-reset-confirm = Zresetować wszystkie preferencje? Profile nie zostaną zmienione.

# MT
settings-profiles-hint = Zapisz bieżące ustawienia pod nazwą; później załaduj, aby przełączać się bez dotykania pojedynczych regulatorów.
# MT
settings-profile-name-placeholder = Nazwa profilu
# MT
settings-profile-save = Zapisz
# MT
settings-profile-import = Importuj…
# MT
settings-profile-load = Załaduj
# MT
settings-profile-export = Eksportuj…
# MT
settings-profile-delete = Usuń
# MT
settings-profile-empty = Brak zapisanych profili.
# MT
settings-profile-import-prompt = Nazwa dla importowanego profilu:

# MT
toast-settings-reset = Ustawienia zresetowane
# MT
toast-profile-saved = Profil zapisany
# MT
toast-profile-loaded = Profil załadowany
# MT
toast-profile-exported = Profil wyeksportowany
# MT
toast-profile-imported = Profil zaimportowany

# Phase 13d — activity feed + header picker buttons
action-add-files = Dodaj pliki
action-add-folders = Dodaj foldery
activity-title = Aktywność
activity-clear = Wyczyść listę aktywności
activity-empty = Brak aktywności plików.
activity-after-done = Po zakończeniu:
activity-keep-open = Zostaw aplikację otwartą
activity-close-app = Zamknij aplikację
activity-shutdown = Wyłącz PC
activity-logoff = Wyloguj
activity-sleep = Uśpij

# Phase 14 — preflight free-space dialog
preflight-block-title = Za mało miejsca w miejscu docelowym
preflight-warn-title = Mało miejsca w miejscu docelowym
preflight-unknown-title = Nie można ustalić wolnego miejsca
preflight-unknown-body = Źródło jest za duże, by je szybko zmierzyć, lub wolumin docelowy nie odpowiedział. Możesz kontynuować; zabezpieczenie silnika czysto zatrzyma kopiowanie, jeśli zabraknie miejsca.
preflight-required = Wymagane
preflight-free = Wolne
preflight-reserve = Rezerwa
preflight-shortfall = Niedobór
preflight-continue = Kontynuuj mimo to
collision-modal-overwrite-older = Nadpisz tylko starsze

# Phase 14e — subset picker
preflight-pick-subset = Wybierz, co skopiować…
subset-title = Wybierz źródła do skopiowania
subset-subtitle = Cała selekcja nie zmieści się na miejscu docelowym. Zaznacz to, co chcesz skopiować; reszta zostanie pominięta.
subset-loading = Mierzenie rozmiarów…
subset-too-large = za duże, by policzyć
subset-budget = Dostępne
subset-remaining = Pozostało
subset-confirm = Skopiuj wybór
history-rerun-hint = Uruchom tę kopię ponownie — ponownie skanuje każdy plik w drzewie źródłowym
history-clear-all = Wyczyść wszystko
history-clear-all-confirm = Kliknij ponownie, aby potwierdzić
history-clear-all-hint = Usuwa każdy wiersz historii. Drugie kliknięcie potwierdza.
toast-history-cleared = Historia wyczyszczona ({ $count } wierszy usunięto)

# Phase 15 — source-list ordering
drop-dialog-sort-label = Kolejność:
sort-custom = Własna
sort-name-asc = Nazwa A → Z (najpierw pliki)
sort-name-desc = Nazwa Z → A (najpierw pliki)
sort-size-asc = Rozmiar rosnąco (najpierw pliki)
sort-size-desc = Rozmiar malejąco (najpierw pliki)
sort-reorder = Zmień kolejność
sort-move-top = Na górę
sort-move-up = W górę
sort-move-down = W dół
sort-move-bottom = Na dół
sort-name-asc-simple = Nazwa A → Z
sort-name-desc-simple = Nazwa Z → A
sort-size-asc-simple = Najmniejsze najpierw
sort-size-desc-simple = Największe najpierw
activity-sort-locked = Sortowanie jest wyłączone podczas trwania kopii. Wstrzymaj lub poczekaj na zakończenie, potem zmień kolejność.
drop-dialog-collision-label = Jeśli plik już istnieje:
collision-policy-keep-both = Zachowaj oba (zmień nazwę nowej kopii na _2, _3 …)
collision-policy-skip = Pomiń nową kopię
collision-policy-overwrite = Nadpisz istniejący plik
collision-policy-overwrite-if-newer = Nadpisz tylko, jeśli nowszy
collision-policy-prompt = Pytaj za każdym razem
drop-dialog-busy-checking = Sprawdzanie wolnego miejsca…
drop-dialog-busy-enumerating = Liczenie plików…
drop-dialog-busy-starting = Uruchamianie kopiowania…
toast-enumeration-deferred = Drzewo źródłowe jest duże — pomijanie listy wstępnej; wiersze pojawią się w trakcie przetwarzania.

# Phase 14a — enumeration-time filters
# MT
settings-tab-filters = Filtry
# MT
settings-filters-hint = Pomija pliki na etapie wyliczania, więc silnik ich nawet nie otwiera. „Dołącz" działa tylko na plikach; „Wyklucz" przycina też pasujące foldery.
# MT
settings-filters-enabled = Włącz filtry dla kopii drzew
# MT
settings-filters-include-globs = Globy włączające
# MT
settings-filters-include-globs-placeholder = **/*.txt
# MT
settings-filters-include-globs-hint = Jeden glob na linię. Jeśli nie jest pusty, plik musi pasować do co najmniej jednego. Katalogi są zawsze przeglądane.
# MT
settings-filters-exclude-globs = Globy wykluczające
# MT
settings-filters-exclude-globs-placeholder = **/node_modules
# MT
settings-filters-exclude-globs-hint = Jeden glob na linię. Dopasowania przycinają całe poddrzewo dla folderów; pasujące pliki są pomijane.
# MT
settings-filters-size-range = Zakres rozmiaru pliku
# MT
settings-filters-min-size-bytes = Minimalny rozmiar (bajty, puste = brak dolnej granicy)
# MT
settings-filters-max-size-bytes = Maksymalny rozmiar (bajty, puste = brak górnej granicy)
# MT
settings-filters-date-range = Zakres czasu modyfikacji
# MT
settings-filters-min-mtime = Zmodyfikowano w dniu lub po
# MT
settings-filters-max-mtime = Zmodyfikowano w dniu lub przed
# MT
settings-filters-attributes = Atrybuty
# MT
settings-filters-skip-hidden = Pomijaj ukryte pliki / foldery
# MT
settings-filters-skip-system = Pomijaj pliki systemowe (tylko Windows)
# MT
settings-filters-skip-readonly = Pomijaj pliki tylko do odczytu

# Phase 15 — auto-update
# MT
settings-tab-updater = Aktualizacje
# MT
settings-updater-hint = Copy That sprawdza podpisane aktualizacje maksymalnie raz dziennie. Aktualizacje instalują się przy następnym zamknięciu aplikacji.
# MT
settings-updater-auto-check = Sprawdzaj aktualizacje przy uruchamianiu
# MT
settings-updater-channel = Kanał wydań
# MT
settings-updater-channel-stable = Stabilny
# MT
settings-updater-channel-beta = Beta (przed wydaniem)
# MT
settings-updater-last-check = Ostatnie sprawdzenie
# MT
settings-updater-last-never = Nigdy
# MT
settings-updater-check-now = Sprawdź aktualizacje teraz
# MT
settings-updater-checking = Sprawdzanie…
# MT
settings-updater-available = Dostępna aktualizacja
# MT
settings-updater-up-to-date = Używasz najnowszej wersji.
# MT
settings-updater-dismiss = Pomiń tę wersję
# MT
settings-updater-dismissed = Pominięto
# MT
toast-update-available = Dostępna jest nowsza wersja
# MT
toast-update-up-to-date = Masz już najnowszą wersję
