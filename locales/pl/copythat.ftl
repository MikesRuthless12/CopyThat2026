app-name = Copy That v1.0.0
# MT
window-title = Copy That v1.0.0
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
err-path-invalid-encoding = Path rejected — string contains invalid UTF-8 / replacement characters
# MT
err-helper-invalid-json = Privileged helper received malformed JSON; ignoring this request
err-helper-grant-out-of-band = GrantCapabilities must be handled by the helper run-loop, not the stateless handler
err-randomness-unavailable = OS random-number generator failed; cannot mint a session id
# MT
err-io-other = Nieznany błąd we/wy
err-sparseness-mismatch = Nie udało się zachować układu rozrzedzonego w miejscu docelowym  # MT

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
settings-preserve-sparseness = Zachowaj pliki rozrzedzone  # MT
settings-preserve-sparseness-hint = Kopiuj tylko przydzielone zakresy plików rozrzedzonych (dyski VM, pliki baz danych), aby rozmiar na dysku w miejscu docelowym był taki sam jak w źródle.  # MT

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

# Phase 19a — disk-backed file enumeration (TeraCopy-compatible scan DB)
# MT
scan-progress-title = Skanowanie…
# MT
scan-progress-stats = { $files } plików · { $bytes } do tej pory
# MT
scan-pause-button = Wstrzymaj skanowanie
# MT
scan-resume-button = Wznów skanowanie
# MT
scan-cancel-button = Anuluj skanowanie
# MT
scan-cancel-confirm = Anulować skanowanie i odrzucić postęp?
# MT
scan-db-header = Baza danych skanowania
# MT
scan-db-hint = Baza danych skanowania na dysku dla zadań z milionami plików.
# MT
advanced-scan-hash-during = Obliczaj sumy kontrolne podczas skanowania
# MT
advanced-scan-db-path = Lokalizacja bazy danych skanowania
# MT
advanced-scan-retention-days = Automatycznie usuwaj ukończone skany po (dniach)
# MT
advanced-scan-max-keep = Maksymalna liczba przechowywanych baz danych skanowania

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
sparse-not-supported-title = Miejsce docelowe wypełnia pliki rozrzedzone  # MT
sparse-not-supported-body = { $dst_fs } nie obsługuje plików rozrzedzonych. Dziury w źródle zostały zapisane jako zera, więc miejsce docelowe jest większe na dysku.  # MT
sparse-warning-densified = Zachowano układ rozrzedzony: skopiowano tylko przydzielone zakresy.  # MT
sparse-warning-mismatch = Niezgodność układu rozrzedzonego — miejsce docelowe może być większe niż oczekiwano.  # MT

# Phase 24 — security-metadata preservation. MT-flagged drafts;
# the authoritative English source lives in locales/en/copythat.ftl.
settings-preserve-security-metadata = Zachowaj metadane bezpieczeństwa  # MT
settings-preserve-security-metadata-hint = Przechwyć i ponownie zastosuj strumienie metadanych poza pasmem (NTFS ADS / xattrs / listy ACL POSIX / konteksty SELinux / uprawnienia plików Linux / forki zasobów macOS) przy każdej kopii.  # MT
settings-preserve-motw = Zachowaj Mark-of-the-Web (flagę pobrania z internetu)  # MT
settings-preserve-motw-hint = Krytyczne dla bezpieczeństwa. SmartScreen i Office Protected View używają tego strumienia do ostrzegania o plikach pobranych z internetu. Wyłączenie pozwala pobranemu programowi pozbyć się znacznika pochodzenia podczas kopiowania i obejść zabezpieczenia systemu operacyjnego.  # MT
settings-preserve-posix-acls = Zachowaj listy ACL POSIX i atrybuty rozszerzone  # MT
settings-preserve-posix-acls-hint = Przenieś atrybuty user.* / system.* / trusted.* xattrs i listy kontroli dostępu POSIX podczas kopiowania.  # MT
settings-preserve-selinux = Zachowaj konteksty SELinux  # MT
settings-preserve-selinux-hint = Przenieś etykietę security.selinux podczas kopiowania, aby demony działające pod politykami MAC nadal miały dostęp do pliku.  # MT
settings-preserve-resource-forks = Zachowaj forki zasobów macOS i informacje Findera  # MT
settings-preserve-resource-forks-hint = Przenieś starszy fork zasobów i FinderInfo (tagi kolorów, metadane Carbon) podczas kopiowania.  # MT
settings-appledouble-fallback = Użyj towarzyszącego pliku AppleDouble na niezgodnych systemach plików  # MT
meta-translated-to-appledouble = Obce metadane zapisane w towarzyszącym pliku AppleDouble (._{ $ext })  # MT

# Phase 25 — two-way sync with vector-clock conflict detection.
# MT-flagged drafts; the authoritative English source lives in
# locales/en/copythat.ftl.
footer-sync = Sync  # MT
sync-drawer-title = Synchronizacja dwukierunkowa  # MT
sync-drawer-hint = Utrzymuj dwa foldery zsynchronizowane bez cichych nadpisań. Równoczesne edycje pojawiają się jako konflikty do rozwiązania.  # MT
sync-add-pair = Dodaj parę  # MT
sync-add-cancel = Anuluj  # MT
sync-refresh = Odśwież  # MT
sync-add-save = Zapisz parę  # MT
sync-add-saving = Zapisywanie…  # MT
sync-add-missing-fields = Etykieta, ścieżka lewa i ścieżka prawa są wszystkie wymagane.  # MT
sync-remove-confirm = Usunąć tę parę synchronizacji? Baza danych stanu jest zachowywana; foldery pozostają nietknięte.  # MT
sync-field-label = Etykieta  # MT
sync-field-label-placeholder = np. Dokumenty ↔ NAS  # MT
sync-field-left = Lewy folder  # MT
sync-field-left-placeholder = Wybierz lub wklej ścieżkę absolutną  # MT
sync-field-right = Prawy folder  # MT
sync-field-right-placeholder = Wybierz lub wklej ścieżkę absolutną  # MT
sync-field-mode = Tryb  # MT
sync-mode-two-way = Dwukierunkowy  # MT
sync-mode-mirror-left-to-right = Lustro (lewo → prawo)  # MT
sync-mode-mirror-right-to-left = Lustro (prawo → lewo)  # MT
sync-mode-contribute-left-to-right = Wkład (lewo → prawo, bez usuwania)  # MT
sync-no-pairs = Brak skonfigurowanych par synchronizacji. Kliknij "Dodaj parę", aby rozpocząć.  # MT
sync-loading = Ładowanie skonfigurowanych par…  # MT
sync-never-run = Nigdy nie uruchomiono  # MT
sync-running = Uruchomione  # MT
sync-run-now = Uruchom teraz  # MT
sync-cancel = Anuluj  # MT
sync-remove-pair = Usuń  # MT
sync-view-conflicts = Zobacz konflikty ({ $count })  # MT
sync-conflicts-heading = Konflikty  # MT
sync-no-conflicts = Brak konfliktów z ostatniego uruchomienia.  # MT
sync-winner = Zwycięzca  # MT
sync-side-left-to-right = lewy  # MT
sync-side-right-to-left = prawy  # MT
sync-conflict-kind-concurrent-write = Równoczesna edycja  # MT
sync-conflict-kind-delete-edit = Usuń ↔ edytuj  # MT
sync-conflict-kind-add-add = Obie strony dodały  # MT
sync-conflict-kind-corrupt-equal = Zawartość rozeszła się bez nowego zapisu  # MT
sync-resolve-keep-left = Zachowaj lewy  # MT
sync-resolve-keep-right = Zachowaj prawy  # MT
sync-resolve-keep-both = Zachowaj oba  # MT
sync-resolve-three-way = Rozwiąż przez scalanie trójdrożne  # MT
sync-resolve-phase-53-tooltip = Interaktywne scalanie trójdrożne dla plików nietekstowych pojawi się w fazie 53.  # MT
sync-error-prefix = Błąd synchronizacji  # MT

# Phase 26 — real-time mirror watcher. MT-flagged drafts;
# the authoritative English source lives in locales/en/copythat.ftl.
live-mirror-start = Uruchom lustro na żywo  # MT
live-mirror-stop = Zatrzymaj lustro na żywo  # MT
live-mirror-watching = Obserwowanie  # MT
live-mirror-toggle-hint = Automatyczne ponowne zsynchronizowanie przy każdej wykrytej zmianie w systemie plików. Jeden wątek w tle na aktywną parę.  # MT
watch-event-prefix = Zmiana pliku  # MT
watch-overflow-recovered = Bufor obserwatora przepełniony; ponowne wyliczanie w celu odzyskania  # MT

# Phase 27 — content-defined chunk store. MT-flagged drafts;
# the authoritative English source lives in locales/en/copythat.ftl.
chunk-store-section = Magazyn fragmentów  # MT
chunk-store-enable = Włącz magazyn fragmentów (wznawianie delta i deduplikacja)  # MT
chunk-store-enable-hint = Dzieli każdy kopiowany plik według treści (FastCDC) i przechowuje fragmenty adresowane treścią. Ponowne próby przepisują tylko zmienione fragmenty; pliki z udostępnioną treścią są automatycznie deduplikowane.  # MT
chunk-store-location = Lokalizacja magazynu fragmentów  # MT
chunk-store-max-size = Maksymalny rozmiar magazynu fragmentów  # MT
chunk-store-prune = Usuń fragmenty starsze niż (dni)  # MT
chunk-store-savings = Zaoszczędzono { $gib } GiB dzięki deduplikacji fragmentów  # MT
chunk-store-disk-usage = Używa { $size } w { $chunks } fragmentach  # MT

# Phase 28 — tray-resident Drop Stack. MT-flagged drafts;
# the authoritative English source lives in locales/en/copythat.ftl.
dropstack-window-title = Drop Stack  # MT
dropstack-tray-open = Drop Stack  # MT
dropstack-empty-title = Drop Stack jest pusty  # MT
dropstack-empty-hint = Przeciągnij pliki tutaj z Eksploratora lub kliknij prawym przyciskiem myszy wiersz zadania, aby go dodać.  # MT
dropstack-add-to-stack = Dodaj do Drop Stack  # MT
dropstack-copy-all-to = Skopiuj wszystko do…  # MT
dropstack-move-all-to = Przenieś wszystko do…  # MT
dropstack-clear = Wyczyść stos  # MT
dropstack-remove-row = Usuń ze stosu  # MT
dropstack-path-missing-toast = Usunięto { $path } — plik już nie istnieje.  # MT
dropstack-always-on-top = Trzymaj Drop Stack zawsze na wierzchu  # MT
dropstack-show-tray-icon = Pokaż ikonę Copy That w obszarze powiadomień  # MT
dropstack-open-on-start = Otwórz Drop Stack automatycznie przy starcie aplikacji  # MT
dropstack-count = { $count } ścieżka  # MT

# Phase 29 — spring-loaded folders + native DnD polish.
settings-dnd-heading = Drag and drop  # MT
settings-dnd-spring-load = Spring-load folders while dragging  # MT
settings-dnd-spring-delay = Spring-load delay (ms)  # MT
settings-dnd-thumbnails = Show drag thumbnails  # MT
settings-dnd-invalid-highlight = Highlight invalid drop targets  # MT
dropzone-invalid-title = Not a valid drop target  # MT
dropzone-invalid-readonly = Destination is read-only  # MT
dropzone-picker-title = Choose a destination  # MT
dropzone-picker-up = Up  # MT
dropzone-picker-path = Current path  # MT
dropzone-picker-root = Roots  # MT
dropzone-picker-use-this = Use this folder  # MT
dropzone-picker-empty = No subfolders  # MT
dropzone-picker-cancel = Cancel  # MT

# Phase 30 — cross-platform path translation.
translate-heading = Cross-platform compatibility  # MT
translate-unicode-label = Unicode normalization  # MT
translate-unicode-auto = Auto-detect destination  # MT
translate-unicode-windows = NFC (Windows / Linux)  # MT
translate-unicode-macos = Leave as-is (macOS / APFS)  # MT
translate-line-endings-label = Translate line endings for text files  # MT
translate-line-endings-allowlist = Text file extensions  # MT
reserved-name-label = Windows reserved-name handling  # MT
reserved-name-suffix = Append "_" (CON.txt → CON_.txt)  # MT
reserved-name-reject = Reject and warn  # MT
long-path-label = Use Windows long-path prefix (\?\) when over 260 chars  # MT
long-path-hint = Some network shares and legacy tools don't honor the \?\ namespace.  # MT

# Phase 31 — power-aware copying.
power-heading = Power & State  # MT
power-enabled = Enable power-aware rules  # MT
power-battery-label = On battery  # MT
power-metered-label = On metered Wi-Fi  # MT
power-cellular-label = On cellular  # MT
power-presentation-label = When presenting (Zoom / Teams / Keynote)  # MT
power-fullscreen-label = When an app is fullscreen  # MT
power-thermal-label = When CPU is thermal-throttling  # MT
power-rule-continue = Continue at full speed  # MT
power-rule-pause = Pause all jobs  # MT
power-rule-cap = Cap bandwidth  # MT
power-rule-cap-percent = Cap to a percent of current rate  # MT
power-reason-on-battery = on battery  # MT
power-reason-metered-network = metered network  # MT
power-reason-cellular-network = cellular network  # MT
power-reason-presenting = presentation mode  # MT
power-reason-fullscreen = fullscreen app  # MT
power-reason-thermal-throttling = CPU is throttling  # MT

# Phase 32 — cloud backend matrix via OpenDAL.
remote-heading = Remote backends  # MT
remote-add = Add backend  # MT
remote-list-empty = No remote backends configured  # MT
remote-test = Test connection  # MT
remote-test-success = Connection successful  # MT
remote-test-failed = Connection failed  # MT
remote-remove = Remove backend  # MT
remote-name-label = Display name  # MT
remote-kind-label = Backend type  # MT
remote-save = Save backend  # MT
remote-cancel = Cancel  # MT
backend-s3 = Amazon S3  # MT
backend-r2 = Cloudflare R2  # MT
backend-b2 = Backblaze B2  # MT
backend-azure-blob = Azure Blob Storage  # MT
backend-gcs = Google Cloud Storage  # MT
backend-onedrive = OneDrive  # MT
backend-google-drive = Google Drive  # MT
backend-dropbox = Dropbox  # MT
backend-webdav = WebDAV  # MT
backend-sftp = SFTP  # MT
backend-ftp = FTP  # MT
backend-local-fs = Local filesystem  # MT
cloud-config-bucket = Bucket  # MT
cloud-config-region = Region  # MT
cloud-config-endpoint = Endpoint URL  # MT
cloud-config-root = Root path  # MT
cloud-error-invalid-config = Backend configuration is invalid  # MT
cloud-error-network = Network error contacting backend  # MT
cloud-error-not-found = Object not found at the requested path  # MT
cloud-error-permission = Permission denied by remote backend  # MT
cloud-error-keychain = OS keychain access failed  # MT
settings-tab-remotes = Remotes  # MT
settings-tab-mobile = Mobile  # MT

# Phase 33 — mount as read-only filesystem.
mount-heading = Mount snapshot  # MT
mount-action-mount = Mount snapshot  # MT
mount-action-unmount = Unmount  # MT
mount-status-mounted = Mounted at { $path }  # MT
mount-error-unsafe-mountpoint = Mountpoint path is unsafe  # MT
mount-error-mountpoint-not-empty = Mountpoint must be an empty directory  # MT
mount-error-backend-unavailable = Mount backend is not available on this system  # MT
mount-error-archive-read = Archive read failed  # MT
mount-picker-title = Pick mountpoint directory  # MT
mount-toast-mounted = Snapshot mounted at { $path }  # MT
mount-toast-unmounted = Snapshot unmounted  # MT
mount-toast-failed = Mount failed: { $reason }  # MT
settings-mount-heading = Mount snapshots  # MT
settings-mount-hint = Expose the history archive as a read-only filesystem. Phase 33b wires the runner flow; the kernel FUSE/WinFsp backends land in Phase 33c.  # MT
settings-mount-on-launch = Mount the latest snapshot on launch  # MT
settings-mount-on-launch-path = Mountpoint path  # MT
settings-mount-on-launch-path-placeholder = e.g. C:\Mounts\copythat  # MT

# Phase 34 — enterprise-grade audit log export + WORM (write-once-read-
# many) tamper-resistance. Settings → Advanced → Audit log collects the
# format + destination + rotation + WORM toggle; runtime records job and
# file events into the tamper-evident chain-hashed sink.
settings-audit-heading = Audit log  # MT
settings-audit-hint = Append-only tamper-evident log of every job and file event. Formats include CSV, JSON-lines, RFC 5424 Syslog, ArcSight CEF, and QRadar LEEF.  # MT
settings-audit-enable = Enable audit logging  # MT
settings-audit-format = Log format  # MT
settings-audit-format-json-lines = JSON lines (recommended default)  # MT
settings-audit-format-csv = CSV (spreadsheet-friendly)  # MT
settings-audit-format-syslog = Syslog (RFC 5424)  # MT
settings-audit-format-cef = CEF (ArcSight)  # MT
settings-audit-format-leef = LEEF 2.0 (IBM QRadar)  # MT
settings-audit-file-path = Log file path  # MT
settings-audit-file-path-placeholder = e.g. C:\ProgramData\CopyThat\audit.log  # MT
settings-audit-max-size = Rotate after (bytes, 0 = never)  # MT
settings-audit-worm = Enable WORM mode (write-once-read-many)  # MT
settings-audit-worm-hint = Applies the platform's append-only flag (Linux chattr +a, macOS chflags uappnd, Windows read-only attribute) after every create or rotation. Even an administrator must explicitly clear the flag to truncate the log.  # MT
settings-audit-test-write = Test write  # MT
settings-audit-verify-chain = Verify chain  # MT
toast-audit-test-write-ok = Audit log test write succeeded  # MT
toast-audit-verify-ok = Audit chain verified intact  # MT
toast-audit-verify-failed = Audit chain verification reported mismatches  # MT

# Phase 35 — destination encryption via age (X25519 / passphrase /
# SSH) + on-the-fly zstd compression with a per-extension deny
# heuristic. Surfaced through Settings → Transfer → Encryption +
# Compression; engine short-circuits to the crypt pipeline when
# either stage is active.
settings-crypt-heading = Encryption & compression  # MT
settings-crypt-hint = Transform file contents before they land at the destination. Encryption uses the age format; compression uses zstd and can skip already-compressed media by extension.  # MT
settings-crypt-encryption-mode = Encryption  # MT
settings-crypt-encryption-off = Off  # MT
settings-crypt-encryption-passphrase = Passphrase (prompt at copy start)  # MT
settings-crypt-encryption-recipients = Recipient keys from file  # MT
settings-crypt-encryption-hint = Passphrases are held only in memory for the duration of the copy. Recipient files list one age1… or ssh- public key per line.  # MT
settings-crypt-recipients-file = Recipients file path  # MT
settings-crypt-recipients-file-placeholder = e.g. C:\Users\me\recipients.txt  # MT
settings-crypt-compression-mode = Compression  # MT
settings-crypt-compression-off = Off  # MT
settings-crypt-compression-always = Always  # MT
settings-crypt-compression-smart = Smart (skip already-compressed media)  # MT
settings-crypt-compression-hint = Smart mode skips jpg, mp4, zip, 7z and similar formats that don't benefit from zstd. Always mode compresses every file at the chosen level.  # MT
settings-crypt-compression-level = zstd level (1-22)  # MT
settings-crypt-compression-level-hint = Lower numbers are faster; higher numbers compress harder. Level 3 matches zstd's CLI default.  # MT
compress-footer-savings = 💾 { $original } → { $compressed } ({ $percent }% saved)  # MT
compress-savings-toast = Compressed { $percent }% ({ $bytes } saved)  # MT
crypt-toast-recipients-loaded = Loaded { $count } encryption recipients  # MT
crypt-toast-recipients-error = Failed to load recipients: { $reason }  # MT
crypt-toast-passphrase-required = Encryption needs a passphrase before the copy starts  # MT
crypt-toast-passphrase-set = Encryption passphrase captured  # MT
crypt-footer-encrypted-badge = 🔒 Encrypted (age)  # MT
crypt-footer-compressed-badge = 📦 Compressed (zstd)  # MT

# Phase 36 — copythat CLI. MT-flagged English strings pending human
# translation; tracked in docs/I18N_TODO.md.
cli-help-tagline = Copy That CLI — byte-exact file copy, sync, verify and audit for CI/CD pipelines.  # MT
cli-help-exit-codes = Exit codes: 0 success, 1 error, 2 pending, 3 collision, 4 verify-fail, 5 net, 6 perm, 7 disk-full, 8 cancel, 9 config.  # MT
cli-error-bad-args = copy/move requires at least one source and a destination  # MT
cli-error-unknown-algo = Unknown verify algorithm: { $algo }  # MT
cli-error-missing-spec = --spec is required for plan/apply  # MT
cli-error-spec-parse = Failed to parse jobspec { $path }: { $reason }  # MT
cli-error-spec-empty-sources = Jobspec source list is empty  # MT
cli-info-shape-recorded = Bandwidth shape "{ $rate }" recorded; enforcement is plumbed via copythat-shape  # MT
cli-info-stub-deferred = { $command } is staged for the Phase 36 follow-up wiring  # MT
cli-plan-summary = Plan: { $actions } action(s), { $bytes } byte(s); { $already_done } already in place  # MT
cli-plan-pending = Plan reports pending actions; rerun with `apply` to execute  # MT
cli-plan-already-done = Plan reports nothing to do (idempotent)  # MT
cli-apply-success = Apply finished without errors  # MT
cli-apply-failed = Apply finished with one or more errors  # MT
cli-verify-ok = Verify ok: { $algo } { $digest }  # MT
cli-verify-failed = Verify FAILED for { $path } ({ $algo })  # MT
cli-config-set = Set { $key } = { $value }  # MT
cli-config-reset = Reset { $key } to default  # MT
cli-config-unknown-key = Unknown config key: { $key }  # MT
cli-completions-emitted = Shell completions for { $shell } printed to stdout  # MT

# Phase 37 — desktop-side mobile companion. MT-flagged English
# strings pending human translation; tracked in docs/I18N_TODO.md.
settings-mobile-heading = Mobile companion  # MT
settings-mobile-hint = Pair an iPhone or Android phone to browse history, kick off saved profiles and Phase 36 jobspecs, and receive completion notifications.  # MT
settings-mobile-pair-toggle = Allow new pairings  # MT
settings-mobile-pair-active = Pair-server active — scan the QR with the Copy That mobile app  # MT
settings-mobile-pair-button = Start pairing  # MT
settings-mobile-revoke-button = Revoke  # MT
settings-mobile-no-pairings = No paired devices yet  # MT
settings-mobile-pair-port = Bind port (0 = pick a free one)  # MT
pair-sas-prompt = Both screens should show the same four emojis. Tap Match if they agree.  # MT
pair-sas-confirm = Match  # MT
pair-sas-reject = Mismatch — cancel  # MT
pair-toast-success = Paired with { $device }  # MT
pair-toast-failed = Pairing failed: { $reason }  # MT
push-toast-sent = Push sent to { $device }  # MT
push-toast-failed = Push to { $device } failed: { $reason }  # MT

# Phase 38 — destination dedup + reflink ladder. MT-flagged
# English strings pending human translation; tracked in
# docs/I18N_TODO.md.
settings-dedup-heading = Destination dedup  # MT
settings-dedup-hint = When the source and destination share a volume, Copy That can clone files at the filesystem level instead of copying bytes. Reflink is instant + safe; hardlink is faster but both names share state.  # MT
settings-dedup-mode-auto = Auto ladder (reflink → hardlink → chunk → copy)  # MT
settings-dedup-mode-reflink-only = Reflink only  # MT
settings-dedup-mode-hardlink-aggressive = Aggressive (reflink + hardlink even on writable files)  # MT
settings-dedup-mode-off = Disabled (always byte-copy)  # MT
settings-dedup-hardlink-policy = Hardlink policy  # MT
settings-dedup-prescan = Pre-scan destination tree for duplicate content  # MT
dedup-badge-reflinked = ⚡ Reflinked  # MT
dedup-badge-hardlinked = 🔗 Hardlinked  # MT
dedup-badge-chunk-shared = 🧩 Chunk-shared  # MT
dedup-badge-copied = 📋 Copied  # MT
phase42-paranoid-verify-label = Weryfikacja paranoiczna
phase42-paranoid-verify-hint = Odrzuca buforowane strony miejsca docelowego i ponownie odczytuje z dysku, aby wykryć kłamstwa pamięci podręcznej zapisu oraz cichą korupcję danych. Około 50 % wolniejsza niż domyślna weryfikacja; domyślnie wyłączona.
phase42-sharing-violation-retries-label = Próby ponowienia dla zablokowanych plików źródłowych
phase42-sharing-violation-retries-hint = Ile razy ponowić próbę, gdy inny proces utrzymuje plik źródłowy otwarty z blokadą wyłączną. Czas oczekiwania podwaja się przy każdej próbie (domyślnie 50 ms / 100 ms / 200 ms). Domyślnie: 3, zgodnie z Robocopy /R:3.
phase42-cloud-placeholder-warning = { $name } to plik OneDrive dostępny wyłącznie w chmurze. Skopiowanie go wywoła pobieranie — nawet { $size } przez połączenie sieciowe.
phase42-defender-exclusion-hint = Aby uzyskać maksymalną przepustowość kopiowania, przed transferami zbiorczymi dodaj folder docelowy do wykluczeń programu Microsoft Defender. Zobacz docs/PERFORMANCE_TUNING.md.

# Phase 39 — Browser-accessible recovery UI.  # MT
settings-recovery-heading = Recovery web UI  # MT
settings-recovery-enable = Enable recovery web UI  # MT
settings-recovery-bind-address = Bind address  # MT
settings-recovery-port = Port (0 = pick a free one)  # MT
settings-recovery-show-url = Show URL & token  # MT
settings-recovery-rotate-token = Rotate token  # MT
settings-recovery-allow-non-loopback = Allow non-loopback bind  # MT
settings-recovery-non-loopback-warning = WARNING: enabling a non-loopback bind exposes the recovery UI to your local network. Anyone who learns the token can browse your file history and download files. Front it with TLS or a reverse proxy if the LAN is untrusted.  # MT
