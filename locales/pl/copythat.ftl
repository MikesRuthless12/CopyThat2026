app-name = Copy That v0.19.84
window-title = Copy That v0.19.84
shred-ssd-advisory = Ostrzeżenie: ten cel znajduje się na dysku SSD. Wielokrotne nadpisywanie nie czyści wiarygodnie pamięci flash, ponieważ wyrównywanie zużycia i nadmiarowa pojemność przenoszą dane spod logicznego adresu bloku. W przypadku nośników półprzewodnikowych użyj ATA SECURE ERASE, NVMe Format z bezpiecznym kasowaniem lub szyfrowania całego dysku z usunięciem klucza.

# Global aggregate states (header pill)
state-idle = Bezczynny
state-copying = Kopiowanie
state-verifying = Weryfikacja
state-paused = Wstrzymano
state-error = Błąd

# Per-job states (row badge)
state-pending = W kolejce
state-running = W toku
state-cancelled = Anulowano
state-succeeded = Gotowe
state-failed = Niepowodzenie

# Actions
action-pause = Wstrzymaj
action-resume = Wznów
action-cancel = Anuluj
action-pause-all = Wstrzymaj wszystkie zadania
action-resume-all = Wznów wszystkie zadania
action-cancel-all = Anuluj wszystkie zadania
action-close = Zamknij
action-reveal = Pokaż w folderze
action-add-files = Dodaj pliki
action-add-folders = Dodaj foldery

# Phase 13d — activity feed
activity-title = Aktywność
activity-clear = Wyczyść listę aktywności
activity-empty = Brak aktywności plików.
activity-after-done = Po zakończeniu:
activity-keep-open = Pozostaw aplikację otwartą
activity-close-app = Zamknij aplikację
activity-shutdown = Wyłącz komputer
activity-logoff = Wyloguj
activity-sleep = Uśpij

# Phase 14 — preflight free-space dialog
preflight-block-title = Za mało miejsca w lokalizacji docelowej
preflight-warn-title = Mało miejsca w lokalizacji docelowej
preflight-unknown-title = Nie można ustalić ilości wolnego miejsca
preflight-unknown-body = Źródło jest zbyt duże, aby szybko oszacować jego rozmiar, lub wolumin docelowy nie odpowiedział. Możesz kontynuować; zabezpieczenie miejsca w silniku zatrzyma kopiowanie w sposób czysty, jeśli zabraknie miejsca.
preflight-required = Wymagane
preflight-free = Wolne
preflight-reserve = Rezerwa
preflight-shortfall = Niedobór
preflight-continue = Kontynuuj mimo to
preflight-pick-subset = Wybierz, co kopiować…
collision-modal-overwrite-older = Nadpisz tylko starsze

# Phase 14e — subset picker
subset-title = Wybierz źródła do skopiowania
subset-subtitle = Pełny wybór nie zmieści się w lokalizacji docelowej. Zaznacz elementy, które chcesz skopiować; reszta pozostanie.
subset-loading = Mierzenie rozmiarów…
subset-too-large = zbyt duży, aby policzyć
subset-budget = Dostępne
subset-remaining = Pozostało
subset-confirm = Kopiuj wybrane
history-rerun-hint = Uruchom to kopiowanie ponownie — skanuje każdy plik w drzewie źródłowym
history-clear-all = Wyczyść wszystko
history-clear-all-confirm = Kliknij ponownie, aby potwierdzić
history-clear-all-hint = Usuń każdy wiersz historii. Wymaga drugiego kliknięcia w celu potwierdzenia.
toast-history-cleared = Wyczyszczono historię (usunięto wierszy: { $count })

# Phase 15 — source-list ordering in the drop staging dialog
drop-dialog-sort-label = Kolejność:
sort-custom = Niestandardowa
sort-name-asc = Nazwa A → Z (najpierw pliki)
sort-name-desc = Nazwa Z → A (najpierw pliki)
sort-size-asc = Rozmiar od najmniejszego (najpierw pliki)
sort-size-desc = Rozmiar od największego (najpierw pliki)
sort-reorder = Zmień kolejność
sort-move-top = Przenieś na górę
sort-move-up = Przenieś w górę
sort-move-down = Przenieś w dół
sort-move-bottom = Przenieś na dół

# Phase 16 — sort preset names for the Activity list (shorter
# wording than the DropStagingDialog variants; the Activity list
# is files-only so the "(files first)" suffix isn't relevant).
sort-name-asc-simple = Nazwa A → Z
sort-name-desc-simple = Nazwa Z → A
sort-size-asc-simple = Rozmiar od najmniejszego
sort-size-desc-simple = Rozmiar od największego
activity-sort-locked = Sortowanie jest wyłączone podczas kopiowania. Wstrzymaj je lub poczekaj na zakończenie, a następnie zmień kolejność.

# Phase 17 — collision-policy picker in the DropStagingDialog
drop-dialog-collision-label = Jeśli plik już istnieje:
collision-policy-keep-both = Zachowaj oba (zmień nazwę nowej kopii na _2, _3, …)
collision-policy-skip = Pomiń nową kopię
collision-policy-overwrite = Nadpisz istniejący plik
collision-policy-overwrite-if-newer = Nadpisz tylko jeśli nowszy
collision-policy-prompt = Pytaj za każdym razem

# Phase 18 — progress labels on the DropStagingDialog Start button
drop-dialog-busy-checking = Sprawdzanie wolnego miejsca…
drop-dialog-busy-enumerating = Liczenie plików…
drop-dialog-busy-starting = Rozpoczynanie kopiowania…
toast-enumeration-deferred = Drzewo źródłowe jest duże — pomijanie wstępnej listy plików; wiersze pojawią się w miarę przetwarzania ich przez silnik.

# Context menu (per-row right-click)
menu-pause = Wstrzymaj
menu-resume = Wznów
menu-cancel = Anuluj
menu-remove = Usuń z kolejki
menu-reveal-source = Pokaż źródło w folderze
menu-reveal-destination = Pokaż cel w folderze

# Header / toolbar
header-eta-label = Szacowany pozostały czas
header-toolbar-label = Sterowanie globalne

# Footer
footer-queued = aktywne zadania
footer-total-bytes = w trakcie
footer-errors = błędy
footer-history = Historia

# Empty state
empty-title = Upuść pliki lub foldery, aby skopiować
empty-hint = Przeciągnij elementy na okno. Zapytamy o lokalizację docelową, a następnie utworzymy jedno zadanie na każde źródło.
empty-region-label = Lista zadań

# Details drawer
details-drawer-label = Szczegóły zadania
details-source = Źródło
details-destination = Cel
details-state = Stan
details-bytes = Bajty
details-files = Pliki
details-speed = Prędkość
details-eta = Pozostały czas
details-error = Błąd

# Drop dialog
drop-dialog-title = Przenieś upuszczone elementy
drop-dialog-subtitle = Elementy gotowe do przeniesienia: { $count }. Wybierz folder docelowy, aby rozpocząć.
drop-dialog-mode = Operacja
drop-dialog-copy = Kopiuj
drop-dialog-move = Przenieś
drop-dialog-pick-destination = Wybierz cel
drop-dialog-change-destination = Zmień cel
drop-dialog-start-copy = Rozpocznij kopiowanie
drop-dialog-start-move = Rozpocznij przenoszenie

# ETA placeholders
eta-calculating = obliczanie…
eta-unknown = nieznany

# Toast messages
toast-job-done = Przesyłanie zakończone
toast-copy-queued = Dodano kopiowanie do kolejki
toast-move-queued = Dodano przenoszenie do kolejki
toast-error-resolved = Błąd rozwiązany
toast-collision-resolved = Konflikt rozwiązany
toast-elevated-unavailable = Ponowienie z podwyższonymi uprawnieniami pojawi się w fazie 17 — jeszcze niedostępne
toast-clipboard-files-detected = Pliki w schowku — naciśnij skrót wklejania, aby skopiować przez Copy That
toast-clipboard-no-files = Schowek nie zawiera plików do wklejenia
toast-error-log-exported = Wyeksportowano dziennik błędów

# Error modal (Phase 8)
error-modal-title = Przesyłanie nie powiodło się
error-modal-retry = Ponów
error-modal-retry-elevated = Ponów z podwyższonymi uprawnieniami
error-modal-skip = Pomiń
error-modal-skip-all-kind = Pomiń wszystkie błędy tego rodzaju
error-modal-abort = Przerwij wszystko
error-modal-path-label = Ścieżka
error-modal-code-label = Kod
error-drawer-pending-count = Więcej błędów oczekuje
error-drawer-toggle = Zwiń lub rozwiń

# Error-kind labels (Phase 8). Source of truth — engine maps each
# `CopyErrorKind` to one of these keys via `localized_key()`.
err-not-found = Nie znaleziono pliku
err-permission-denied = Odmowa dostępu
err-disk-full = Dysk docelowy jest pełny
err-interrupted = Operacja przerwana
err-verify-failed = Weryfikacja po kopiowaniu nie powiodła się
err-path-escape = Ścieżka odrzucona — zawiera segmenty katalogu nadrzędnego (..) lub niedozwolone bajty
err-path-invalid-encoding = Ścieżka odrzucona — ciąg zawiera nieprawidłowy UTF-8 / znaki zastępcze
err-helper-invalid-json = Uprzywilejowany pomocnik otrzymał nieprawidłowy JSON; żądanie zostaje zignorowane
err-helper-grant-out-of-band = GrantCapabilities musi być obsłużone przez pętlę pomocnika, a nie bezstanowy moduł obsługi
err-randomness-unavailable = Generator liczb losowych systemu zawiódł; nie można utworzyć identyfikatora sesji
err-sparseness-mismatch = Nie udało się zachować układu plików rzadkich w lokalizacji docelowej
err-io-other = Nieznany błąd we/wy

# Collision modal (Phase 8)
collision-modal-title = Plik już istnieje
collision-modal-overwrite = Nadpisz
collision-modal-overwrite-if-newer = Nadpisz jeśli nowszy
collision-modal-skip = Pomiń
collision-modal-keep-both = Zachowaj oba
collision-modal-rename = Zmień nazwę…
collision-modal-apply-to-all = Zastosuj do wszystkich
collision-modal-source = Źródło
collision-modal-destination = Cel
collision-modal-size = Rozmiar
collision-modal-modified = Zmodyfikowano
collision-modal-hash-check = Szybki skrót (SHA-256)
collision-modal-hash-computing = Obliczanie…
collision-modal-hash-identical = Identyczne
collision-modal-hash-different = Różne
collision-modal-rename-placeholder = Nowa nazwa pliku
collision-modal-confirm-rename = Zmień nazwę

# Error log drawer (Phase 8)
error-log-title = Dziennik błędów
error-log-empty = Brak zarejestrowanych błędów
error-log-export-csv = Eksportuj CSV
error-log-export-txt = Eksportuj tekst
error-log-clear = Wyczyść dziennik
error-log-col-time = Czas
error-log-col-job = Zadanie
error-log-col-path = Ścieżka
error-log-col-code = Kod
error-log-col-message = Komunikat
error-log-col-resolution = Rozwiązanie

# History drawer (Phase 9)
history-title = Historia
history-empty = Brak zarejestrowanych zadań
history-unavailable = Historia kopiowania jest niedostępna. Aplikacja nie mogła otworzyć magazynu SQLite przy uruchamianiu.
history-filter-any = dowolny
history-filter-kind = Rodzaj
history-filter-status = Status
history-filter-text = Szukaj
history-refresh = Odśwież
history-export-csv = Eksportuj CSV
history-purge-30 = Usuń starsze niż 30 dni
history-rerun = Uruchom ponownie
history-detail-open = Szczegóły
history-detail-title = Szczegóły zadania
history-detail-empty = Brak zarejestrowanych elementów
history-col-date = Data
history-col-kind = Rodzaj
history-col-src = Źródło
history-col-dst = Cel
history-col-files = Pliki
history-col-size = Rozmiar
history-col-status = Status
history-col-duration = Czas trwania
history-col-error = Błąd
toast-history-exported = Wyeksportowano historię
toast-history-rerun-queued = Dodano ponowne uruchomienie do kolejki

# Totals drawer (Phase 10)
footer-totals = Podsumowanie
totals-title = Podsumowanie
totals-loading = Wczytywanie podsumowania…
totals-card-bytes = Łącznie skopiowanych bajtów
totals-card-files = Pliki
totals-card-jobs = Zadania
totals-card-avg-rate = Średnia przepustowość
totals-errors = błędy
totals-spark-title = Ostatnie 30 dni
totals-kinds-title = Według rodzaju
totals-saved-title = Zaoszczędzony czas (szacunkowo)
totals-saved-note = Szacunek względem kopiowania tego samego zestawu w standardowym menedżerze plików.
totals-reset = Resetuj statystyki
totals-reset-confirm = Spowoduje to usunięcie każdego zapisanego zadania i elementu. Kontynuować?
totals-reset-confirm-yes = Tak, resetuj
toast-totals-reset = Zresetowano statystyki

# Phase 11a — i18n core: surface remaining user-visible strings so
# every string on a main-window screen flows through Fluent.

# Header language switcher (temporary placement — folds into
# Settings → General in Phase 12).
header-language-label = Język
header-language-title = Zmień język

# Job-kind labels. History filter, Totals breakdown, and history
# rows all map wire-format `kind` strings to these labels.
kind-copy = Kopiowanie
kind-move = Przenoszenie
kind-delete = Usuwanie
kind-secure-delete = Bezpieczne usuwanie

# History status labels. Distinct from `state-*` because the
# history wire format uses plain `running`/`succeeded`/... rather
# than the live `JobState` enum the queue exposes.
status-running = W toku
status-succeeded = Powodzenie
status-failed = Niepowodzenie
status-cancelled = Anulowano
# Per-item status (not job-level): `ok` / `skipped` are only
# reachable from the history detail view.
status-ok = OK
status-skipped = Pominięto

# History drawer: search field placeholder + purge toast.
history-search-placeholder = /ścieżka
toast-history-purged = Usunięto zadań starszych niż 30 dni: { $count }

# User-facing command-layer validation errors. The Rust side
# returns these keys when input is missing; the toast layer looks
# them up rather than showing the raw English.
err-source-required = Wymagana jest co najmniej jedna ścieżka źródłowa.
err-destination-empty = Ścieżka docelowa jest pusta.
err-source-empty = Ścieżka źródłowa jest pusta.

# Localised duration formatting for the Totals drawer and ETA
# fields. `{ $ms }`, `{ $s }`, `{ $m }`, `{ $h }` are integer
# placeables — the formatter passes pre-computed values in.
duration-lt-1s = < 1 s
duration-ms = { $ms } ms
duration-seconds = { $s } s
duration-minutes-seconds = { $m } min { $s } s
duration-hours-minutes = { $h } godz { $m } min
duration-zero = 0 s

# Rate unit. Appended to a formatted byte size. Some languages
# render this with a leading space ("Ko/s"); keep it translatable
# even though the SI-derived "/s" is near-universal.
rate-unit-per-second = { $size }/s

# Phase 11b — Settings modal skeleton. Phase 12 expanded this into a
# full six-tab preferences window; the `settings-phase-12-hint` key
# is retired in favour of `settings-tab-profiles` + concrete labels.
settings-title = Ustawienia
settings-tab-general = Ogólne
settings-tab-appearance = Wygląd
settings-section-language = Język
settings-phase-12-hint = Więcej ustawień (motyw, domyślne opcje przesyłania, algorytm weryfikacji, profile) pojawi się w fazie 12.

# Phase 12 — full Settings window keys. Grouped by tab so the
# human-review pass can pick one section at a time. Every field
# label and every option label that the user will actually read
# has its own key; dynamic wire values (enum discriminants like
# "auto" / "prefer") stay untranslated on the wire.

settings-loading = Wczytywanie ustawień…
settings-tab-transfer = Przesyłanie
settings-tab-filters = Filtry
settings-tab-shell = Powłoka
settings-tab-secure-delete = Bezpieczne usuwanie
settings-tab-advanced = Zaawansowane
settings-tab-updater = Aktualizacje
settings-tab-profiles = Profile

# General tab additions
settings-section-theme = Motyw
settings-theme-auto = Automatyczny
settings-theme-light = Jasny
settings-theme-dark = Ciemny
settings-start-with-os = Uruchamiaj przy starcie systemu
settings-single-instance = Pojedyncze uruchomione wystąpienie
settings-minimize-to-tray = Minimalizuj do zasobnika przy zamknięciu
settings-error-display-mode = Styl powiadomień o błędach
settings-error-display-modal = Okno modalne (blokuje aplikację)
settings-error-display-drawer = Panel boczny (nieblokujący)
settings-error-display-mode-hint = Okno modalne zatrzymuje kolejkę do czasu podjęcia decyzji. Panel boczny utrzymuje ruch kolejki i pozwala obsługiwać błędy w rogu.
settings-paste-shortcut = Wklejaj pliki za pomocą globalnego skrótu
settings-paste-shortcut-combo = Kombinacja skrótu
settings-paste-shortcut-hint = Naciśnij tę kombinację w dowolnym miejscu systemu, aby wkleić pliki skopiowane z Explorer / Finder / Files przez Copy That. CmdOrCtrl odpowiada Cmd w systemie macOS oraz Ctrl w systemach Windows / Linux.
settings-clipboard-watcher = Obserwuj schowek pod kątem skopiowanych plików
settings-clipboard-watcher-hint = Pokazuj powiadomienie, gdy w schowku pojawią się adresy URL plików, sugerując możliwość wklejenia przez Copy That. Sprawdza schowek co 500 ms, gdy włączone.

# Transfer tab
settings-buffer-size = Rozmiar bufora
settings-verify = Weryfikuj po skopiowaniu
settings-verify-off = Wyłączone
settings-concurrency = Współbieżność
settings-concurrency-auto = Automatyczna
settings-reflink = Reflink / szybkie ścieżki
settings-reflink-prefer = Preferuj
settings-reflink-avoid = Unikaj reflink
settings-reflink-disabled = Zawsze używaj silnika asynchronicznego
settings-fsync-on-close = Synchronizuj z dyskiem przy zamknięciu (wolniej, bezpieczniej)
settings-preserve-timestamps = Zachowuj znaczniki czasu
settings-preserve-permissions = Zachowuj uprawnienia
settings-preserve-acls = Zachowuj listy ACL (faza 14)
settings-preserve-sparseness = Zachowuj pliki rzadkie
settings-preserve-sparseness-hint = Kopiuj tylko przydzielone obszary plików rzadkich (dyski maszyn wirtualnych, pliki baz danych), aby cel zajmował na dysku tyle samo miejsca co źródło.
settings-force-parallel-chunks = Równoległe kopiowanie wieloblokowe (tylko RAID / macierze)
settings-force-parallel-chunks-hint = Dzieli każdą dużą kopię na równoległe bloki. Pomaga tylko w przypadku celów rozłożonych/RAID/sieciowych; SPOWALNIA pojedynczy dysk SSD/NVMe (-25% do -76%). Pozostaw wyłączone, chyba że celem jest macierz wielodyskowa.

# Shell tab
settings-context-menu = Włącz wpisy menu kontekstowego powłoki
settings-intercept-copy = Przejmuj domyślny mechanizm kopiowania (Windows)
settings-intercept-copy-hint = Po włączeniu Ctrl+C / Ctrl+V w Explorer przechodzi przez Copy That. Rejestracja pojawi się w fazie 14.
settings-notify-completion = Powiadamiaj o zakończeniu zadania

# Secure delete tab
settings-shred-method = Domyślna metoda niszczenia
settings-shred-zero = Zero (1 przebieg)
settings-shred-random = Losowo (1 przebieg)
settings-shred-dod3 = DoD 5220.22-M (3 przebiegi)
settings-shred-dod7 = DoD 5220.22-M (7 przebiegów)
settings-shred-gutmann = Gutmann (35 przebiegów)
settings-shred-nist = NIST 800-88
settings-shred-confirm-twice = Wymagaj podwójnego potwierdzenia przed niszczeniem

# Advanced tab
settings-log-level = Poziom dziennika
settings-log-off = Wyłączone
settings-telemetry = Telemetria
settings-telemetry-never = Nigdy — żadnej komunikacji zwrotnej na żadnym poziomie dziennika
settings-error-policy = Domyślna zasada obsługi błędów
settings-error-policy-ask = Pytaj
settings-error-policy-skip = Pomiń
settings-error-policy-retry = Ponów z odstępem
settings-error-policy-abort = Przerwij przy pierwszym niepowodzeniu
settings-history-retention = Przechowywanie historii (dni)
settings-history-retention-hint = 0 = zachowuj na zawsze. Każda inna wartość automatycznie usuwa starsze zadania przy uruchamianiu.
settings-database-path = Ścieżka bazy danych
settings-database-path-default = (domyślnie — katalog danych systemu)
settings-reset-all = Przywróć ustawienia domyślne
settings-reset-confirm = Przywrócić każdą preferencję do wartości domyślnej? Profile pozostaną bez zmian.

# Profiles tab
settings-profiles-hint = Zapisz bieżące ustawienia pod nazwą; wczytaj je później, aby wrócić bez zmieniania pojedynczych opcji.
settings-profile-name-placeholder = Nazwa profilu
settings-profile-save = Zapisz
settings-profile-import = Importuj…
settings-profile-load = Wczytaj
settings-profile-export = Eksportuj…
settings-profile-delete = Usuń
settings-profile-empty = Nie zapisano jeszcze żadnych profili.
settings-profile-import-prompt = Nazwa importowanego profilu:

# Toasts driven by Phase 12 profile actions
toast-settings-reset = Zresetowano ustawienia
toast-profile-saved = Zapisano profil
toast-profile-loaded = Wczytano profil
toast-profile-exported = Wyeksportowano profil
toast-profile-imported = Zaimportowano profil

# Phase 14a — enumeration-time filters
settings-filters-hint = Pomijaj pliki na etapie wyliczania, aby silnik w ogóle ich nie otwierał. Reguły uwzględniania dotyczą tylko plików; reguły wykluczania przycinają również pasujące katalogi.
settings-filters-enabled = Włącz filtry dla kopiowania drzew
settings-filters-include-globs = Wzorce uwzględniania
settings-filters-include-globs-placeholder = **/*.txt
settings-filters-include-globs-hint = Jeden wzorzec na wiersz. Gdy lista nie jest pusta, plik musi pasować do co najmniej jednego wzorca uwzględniania, aby przetrwać. Do katalogów zawsze następuje wejście.
settings-filters-exclude-globs = Wzorce wykluczania
settings-filters-exclude-globs-placeholder = **/node_modules
settings-filters-exclude-globs-hint = Jeden wzorzec na wiersz. Dopasowania przycinają całe poddrzewo katalogów; pasujące pliki są pomijane.
settings-filters-size-range = Zakres rozmiaru plików
settings-filters-min-size-bytes = Minimalny rozmiar (bajty, puste = brak dolnej granicy)
settings-filters-max-size-bytes = Maksymalny rozmiar (bajty, puste = brak górnej granicy)
settings-filters-date-range = Zakres czasu modyfikacji
settings-filters-min-mtime = Zmodyfikowano w dniu lub po
settings-filters-max-mtime = Zmodyfikowano w dniu lub przed
settings-filters-attributes = Bity atrybutów
settings-filters-skip-hidden = Pomijaj ukryte pliki / foldery
settings-filters-skip-system = Pomijaj pliki systemowe (tylko Windows)
settings-filters-skip-readonly = Pomijaj pliki tylko do odczytu

# Phase 15 — auto-update
settings-updater-hint = Copy That sprawdza dostępność podpisanych aktualizacji najwyżej raz dziennie. Aktualizacje instalują się przy następnym zamknięciu aplikacji.
settings-updater-auto-check = Sprawdzaj aktualizacje przy uruchamianiu
settings-updater-channel = Kanał wydań
settings-updater-channel-stable = Stabilny
settings-updater-channel-beta = Beta (wersja przedpremierowa)
settings-updater-last-check = Ostatnie sprawdzenie
settings-updater-last-never = Nigdy
settings-updater-check-now = Sprawdź aktualizacje teraz
settings-updater-checking = Sprawdzanie…
settings-updater-available = Dostępna aktualizacja
settings-updater-up-to-date = Korzystasz z najnowszego wydania.
settings-updater-dismiss = Pomiń tę wersję
settings-updater-dismissed = Pominięto
toast-update-available = Dostępna jest nowsza wersja
toast-update-up-to-date = Korzystasz już z najnowszej wersji

# Phase 19a — disk-backed file enumeration (TeraCopy-compatible scan DB)
scan-progress-title = Skanowanie…
scan-progress-stats = Plików: { $files } · { $bytes } dotychczas
scan-pause-button = Wstrzymaj skanowanie
scan-resume-button = Wznów skanowanie
scan-cancel-button = Anuluj skanowanie
scan-cancel-confirm = Anulować skanowanie i odrzucić postęp?
scan-db-header = Baza danych skanowania
scan-db-hint = Dyskowa baza danych skanowania dla zadań obejmujących wiele milionów plików.
advanced-scan-hash-during = Obliczaj sumy kontrolne podczas skanowania
advanced-scan-db-path = Lokalizacja bazy danych skanowania
advanced-scan-retention-days = Automatycznie usuwaj ukończone skany po (dni)
advanced-scan-max-keep = Maksymalna liczba przechowywanych baz danych skanowania

# Phase 19b — filesystem-snapshot source for locked files.
settings-on-locked = Gdy plik jest zablokowany
settings-on-locked-ask = Pytaj za pierwszym razem
settings-on-locked-retry = Ponów na krótko, a następnie zgłoś błąd
settings-on-locked-skip = Pomiń zablokowany plik
settings-on-locked-snapshot = Użyj migawki systemu plików
settings-on-locked-hint = Wyeliminuj błędy „plik używany przez inny proces”. Copy That tworzy migawkę woluminu źródłowego (VSS w systemie Windows, ZFS/Btrfs w systemie Linux, APFS w systemie macOS) i odczytuje z kopii migawki.
snapshot-prompt-title = Ten plik jest używany przez inny proces
snapshot-prompt-body = Inny program ma plik { $path } otwarty do zapisu na wyłączność. Wybierz, jak Copy That ma obsłużyć ten i podobne pliki na tym samym woluminie.
snapshot-source-active = 📷 Odczyt z migawki { $kind } woluminu { $volume }
snapshot-create-failed = Nie udało się utworzyć migawki woluminu źródłowego
snapshot-vss-needs-elevation = Odczyt z migawki VSS wymaga uprawnień administratora. Copy That poprosi o ich nadanie.
snapshot-cleanup-failed = Pomocnik migawki zgłosił niepowodzenie czyszczenia — na woluminie może pozostać zaległa kopia w tle.

# Phase 20 — durable resume journal.
resume-prompt-title = Wznowić poprzednie przesyłanie?
resume-prompt-body = Copy That wykrył nieukończone przesyłania z poprzedniej sesji ({ $count }). Wybierz, co zrobić z każdym z nich.
resume-prompt-resume = Wznów
resume-prompt-resume-all = Wznów wszystkie
resume-discard-one = Nie wznawiaj
resume-discard-all = Odrzuć wszystkie
resume-aborted-hash-mismatch = Pierwsze { $offset } bajtów w lokalizacji docelowej nie pasuje do źródła — ponowne uruchamianie od początku.
settings-auto-resume = Automatycznie wznawiaj przerwane zadania bez pytania
settings-auto-resume-hint = Pomijaj pytanie o wznowienie przy uruchamianiu i po cichu ponownie dodawaj każde nieukończone zadanie do kolejki. Domyślnie wyłączone.

# Phase 21 — bandwidth shaping (GCRA token bucket + schedule + auto-throttle).
settings-tab-network = Sieć
settings-network-hint = Ogranicz prędkość przesyłania, aby reszta sieci pozostawała użyteczna. Stosuj globalnie, według harmonogramu dziennego lub automatycznie reaguj na taryfowe połączenia Wi-Fi / baterię / sieć komórkową.
settings-network-mode = Limit przepustowości
settings-network-mode-off = Wyłączony (bez limitu)
settings-network-mode-fixed = Stała wartość
settings-network-mode-schedule = Użyj harmonogramu
settings-network-cap-mbps = Limit (MB/s)
settings-network-schedule = Harmonogram (format rclone)
settings-network-schedule-hint = Granice HH:MM,szybkość rozdzielone spacjami plus opcjonalne reguły dni Mon-Fri,szybkość. Szybkości: 512k, 10M, 2G, off, unlimited. Przykład: 08:00,512k 18:00,10M Sat-Sun,unlimited.
settings-network-auto-header = Automatyczne ograniczanie
settings-network-auto-metered = Przy taryfowym Wi-Fi
settings-network-auto-battery = Na baterii
settings-network-auto-cellular = W sieci komórkowej
settings-network-auto-unchanged = Nie zmieniaj
settings-network-auto-pause = Wstrzymaj przesyłanie
settings-network-auto-cap = Ogranicz do stałej wartości
shape-badge-paused = wstrzymano
shape-badge-tooltip = Limit przepustowości aktywny — kliknij, aby otworzyć Ustawienia → Sieć
shape-badge-source-schedule = zaplanowane
shape-badge-source-metered = taryfowe
shape-badge-source-battery = na baterii
shape-badge-source-cellular = sieć komórkowa
shape-badge-source-settings = aktywny
shape-error-schedule-invalid = Format harmonogramu jest nieprawidłowy: { $message }

# Phase 22 — aggregate conflict dialog v2 (thumbnails, per-pattern
# rules, and reusable conflict profiles). Every key below is user-
# visible text in the `ConflictBatchModal.svelte` component.
conflict-batch-title = Konflikty plików w { $jobname }: { $count }
conflict-batch-state-pending = Oczekujące
conflict-batch-state-resolved = Rozwiązane
conflict-batch-action-overwrite = Nadpisz
conflict-batch-action-skip = Pomiń
conflict-batch-action-keep-both = Zachowaj oba
conflict-batch-action-newer-wins = Wygrywa nowszy
conflict-batch-action-larger-wins = Wygrywa większy
conflict-batch-bulk-apply-selected = Zastosuj do zaznaczonych
conflict-batch-bulk-apply-extension = Zastosuj do wszystkich o tym rozszerzeniu
conflict-batch-bulk-apply-glob = Zastosuj do pasujących do wzorca…
conflict-batch-bulk-apply-remaining = Zastosuj do wszystkich pozostałych
conflict-batch-bulk-glob-placeholder = np. **/*.tmp
conflict-batch-save-profile = Zapisz te reguły jako profil…
conflict-batch-profile-placeholder = Nazwa profilu
conflict-batch-matched-rule = według reguły „{ $rule }” → { $action }
conflict-batch-empty = Wszystkie konflikty rozwiązane
conflict-batch-source-vs-destination = Źródło a cel
conflict-batch-source-label = Źródło
conflict-batch-destination-label = Cel
conflict-batch-size-label = Rozmiar
conflict-batch-modified-label = Zmodyfikowano
conflict-batch-close = Zamknij
conflict-batch-profile-saved = Zapisano profil konfliktów

# Phase 23 — sparse-file preservation. The toast fires once per
# destination volume when the filesystem can't preserve holes; the
# warning line is surfaced in the job detail drawer so the user knows
# the dst is larger on disk than the source was.
sparse-not-supported-title = Cel wypełnia pliki rzadkie
sparse-not-supported-body = { $dst_fs } nie obsługuje plików rzadkich. Puste obszary w źródle zostały zapisane jako zera, więc cel zajmuje na dysku więcej miejsca.
sparse-warning-densified = Zachowano układ plików rzadkich: skopiowano tylko przydzielone obszary.
sparse-warning-mismatch = Niezgodność układu plików rzadkich — cel może być większy niż oczekiwano.

# Phase 24 — security-metadata preservation. The Mark-of-the-Web
# (Zone.Identifier ADS) toggle is security-sensitive: turning it off
# lets a downloaded executable shed its SmartScreen / Office Protected
# View flag on copy, which is why the tooltip carries an explicit
# warning. AppleDouble fallback emits `._<filename>` sidecars on
# destination filesystems that can't hold the foreign metadata.
settings-preserve-security-metadata = Zachowuj metadane zabezpieczeń
settings-preserve-security-metadata-hint = Przechwytuj i ponownie stosuj zewnętrzne strumienie metadanych (NTFS ADS / xattrs / listy POSIX ACL / konteksty SELinux / uprawnienia plików Linux / odgałęzienia zasobów macOS) przy każdym kopiowaniu.
settings-preserve-motw = Zachowuj Mark-of-the-Web (flaga pobrania z internetu)
settings-preserve-motw-hint = Kluczowe dla bezpieczeństwa. SmartScreen oraz Office Protected View używają tego strumienia, aby ostrzegać o plikach pobranych z internetu. Wyłączenie pozwala pobranemu plikowi wykonywalnemu pozbyć się znacznika pochodzenia przy kopiowaniu i ominąć zabezpieczenia systemu operacyjnego.
settings-preserve-posix-acls = Zachowuj listy POSIX ACL i atrybuty rozszerzone
settings-preserve-posix-acls-hint = Przenoś atrybuty rozszerzone user.* / system.* / trusted.* oraz listy kontroli dostępu POSIX wraz z kopiowaniem.
settings-preserve-selinux = Zachowuj konteksty SELinux
settings-preserve-selinux-hint = Przenoś etykietę security.selinux wraz z kopiowaniem, aby usługi działające zgodnie z politykami MAC nadal miały dostęp do pliku.
settings-preserve-resource-forks = Zachowuj odgałęzienia zasobów macOS i informacje Finder
settings-preserve-resource-forks-hint = Przenoś starsze odgałęzienie zasobów i FinderInfo (etykiety kolorów, metadane Carbon) wraz z kopiowaniem.
settings-appledouble-fallback = Używaj plików towarzyszących AppleDouble w niezgodnych systemach plików
meta-translated-to-appledouble = Zewnętrzne metadane zapisane w pliku towarzyszącym AppleDouble (._{ $ext })

# Phase 25 — two-way sync with vector-clock conflict detection.
# The drawer lists configured sync pairs; each pair runs an
# independent reconciliation round against a per-pair `.copythat-sync.db`
# state store. Concurrent edits from a common ancestor surface as
# conflicts rather than silent overwrites; the losing side's content
# is preserved as `name.sync-conflict-YYYYMMDD-HHMMSS-<host>.ext`.
footer-sync = Synchronizacja
sync-drawer-title = Synchronizacja dwukierunkowa
sync-drawer-hint = Utrzymuj dwa foldery zsynchronizowane bez cichego nadpisywania. Równoczesne zmiany ujawniają się jako konflikty, które możesz rozwiązać.
sync-add-pair = Dodaj parę
sync-add-cancel = Anuluj
sync-refresh = Odśwież
sync-add-save = Zapisz parę
sync-add-saving = Zapisywanie…
sync-add-missing-fields = Etykieta, ścieżka lewa i ścieżka prawa są wymagane.
sync-remove-confirm = Usunąć tę parę synchronizacji? Baza danych stanu zostanie zachowana; foldery pozostaną nietknięte.
sync-field-label = Etykieta
sync-field-label-placeholder = np. Dokumenty ↔ NAS
sync-field-left = Lewy folder
sync-field-left-placeholder = Wybierz lub wklej ścieżkę bezwzględną
sync-field-right = Prawy folder
sync-field-right-placeholder = Wybierz lub wklej ścieżkę bezwzględną
sync-field-mode = Tryb
sync-mode-two-way = Dwukierunkowy
sync-mode-mirror-left-to-right = Lustro (lewy → prawy)
sync-mode-mirror-right-to-left = Lustro (prawy → lewy)
sync-mode-contribute-left-to-right = Dodawanie (lewy → prawy, bez usuwania)
sync-no-pairs = Nie skonfigurowano jeszcze żadnych par synchronizacji. Kliknij „Dodaj parę”, aby zacząć.
sync-loading = Wczytywanie skonfigurowanych par…
sync-never-run = Nigdy nie uruchomiono
sync-running = W toku
sync-run-now = Uruchom teraz
sync-cancel = Anuluj
sync-remove-pair = Usuń
sync-view-conflicts = Pokaż konflikty ({ $count })
sync-conflicts-heading = Konflikty
sync-no-conflicts = Brak konfliktów z ostatniego uruchomienia.
sync-winner = Zwycięzca
sync-side-left-to-right = lewy
sync-side-right-to-left = prawy
sync-conflict-kind-concurrent-write = Równoczesna zmiana
sync-conflict-kind-delete-edit = Usunięcie ↔ edycja
sync-conflict-kind-add-add = Dodano po obu stronach
sync-conflict-kind-corrupt-equal = Treść się rozeszła bez nowego zapisu
sync-resolve-keep-left = Zachowaj lewy
sync-resolve-keep-right = Zachowaj prawy
sync-resolve-keep-both = Zachowaj oba
sync-resolve-three-way = Rozwiąż przez scalanie trójstronne
sync-resolve-phase-53-tooltip = Interaktywne scalanie trójstronne plików nietekstowych pojawi się w fazie 53.
sync-error-prefix = Błąd synchronizacji

# Phase 26 — real-time mirror watcher. "Live mirror" starts a
# filesystem watcher on the pair's left side; every debounced event
# triggers a re-sync. The watcher filters vim swap files / Office
# lock files / atomic-save temp names so one logical save becomes
# exactly one sync round.
live-mirror-start = Uruchom lustro na żywo
live-mirror-stop = Zatrzymaj lustro na żywo
live-mirror-watching = Obserwowanie
live-mirror-toggle-hint = Automatycznie synchronizuj ponownie przy każdej wykrytej zmianie systemu plików. Jeden wątek w tle na każdą aktywną parę.
watch-event-prefix = Zmiana pliku
watch-overflow-recovered = Bufor obserwatora przepełniony; ponowne wyliczanie w celu odzyskania

# Phase 27 — content-defined chunk store. Enables delta-resume (a
# retry only re-writes chunks that actually changed) and same-job
# dedup (files sharing content blocks store those blocks once). The
# store is disk-backed under `<data-dir>/chunks/` by default and is
# the foundation for the Phase 49–51 moonshot repository phases.
chunk-store-section = Magazyn fragmentów
chunk-store-enable = Włącz magazyn fragmentów (wznawianie różnicowe i deduplikacja)
chunk-store-enable-hint = Dzieli każdy kopiowany plik według treści (FastCDC) i przechowuje fragmenty adresowane treścią. Ponowienia zapisują tylko zmienione fragmenty; pliki o wspólnej treści są automatycznie deduplikowane.
chunk-store-location = Lokalizacja magazynu fragmentów
chunk-store-max-size = Maksymalny rozmiar magazynu fragmentów
chunk-store-prune = Usuwaj fragmenty starsze niż (dni)
chunk-store-savings = Zaoszczędzono { $gib } GiB dzięki deduplikacji fragmentów
chunk-store-disk-usage = Używane { $size } w { $chunks } fragmentach

# Phase 28 — tray-resident Drop Stack. The stack is a persistent list
# of paths gathered from multiple sources (Explorer drag, main-window
# context menu, CLI, drag onto the Drop Stack window) that the user
# can dispatch to a destination in one go.
dropstack-window-title = Drop Stack
dropstack-tray-open = Drop Stack
dropstack-empty-title = Drop Stack jest pusty
dropstack-empty-hint = Przeciągnij tutaj pliki z Explorer lub kliknij prawym przyciskiem wiersz zadania, aby go dodać.
dropstack-add-to-stack = Dodaj do Drop Stack
dropstack-copy-all-to = Kopiuj wszystko do…
dropstack-move-all-to = Przenieś wszystko do…
dropstack-clear = Wyczyść stos
dropstack-remove-row = Usuń ze stosu
dropstack-path-missing-toast = Upuszczono { $path } — plik już nie istnieje.
dropstack-always-on-top = Utrzymuj Drop Stack zawsze na wierzchu
dropstack-show-tray-icon = Pokazuj ikonę Copy That w zasobniku
dropstack-open-on-start = Otwieraj Drop Stack automatycznie przy starcie aplikacji
dropstack-count = Ścieżek: { $count }

# Phase 29 — spring-loaded folders + native DnD polish. The Settings
# → General tab carries the knobs; DropTarget / DestinationPicker
# Svelte components use the dropzone-* keys at runtime.
settings-dnd-heading = Przeciąganie i upuszczanie
settings-dnd-spring-load = Automatycznie otwieraj foldery podczas przeciągania
settings-dnd-spring-delay = Opóźnienie otwierania folderów (ms)
settings-dnd-thumbnails = Pokazuj miniatury podczas przeciągania
settings-dnd-invalid-highlight = Wyróżniaj nieprawidłowe cele upuszczania
dropzone-invalid-title = To nie jest prawidłowy cel upuszczania
dropzone-invalid-readonly = Cel jest tylko do odczytu
dropzone-picker-title = Wybierz cel
dropzone-picker-up = W górę
dropzone-picker-path = Bieżąca ścieżka
dropzone-picker-root = Katalogi główne
dropzone-picker-use-this = Użyj tego folderu
dropzone-picker-empty = Brak podfolderów
dropzone-picker-cancel = Anuluj

# Phase 30 — cross-platform path translation. Settings → Transfer
# exposes these under a "Cross-platform compatibility" subsection.
translate-heading = Zgodność międzyplatformowa
translate-unicode-label = Normalizacja Unicode
translate-unicode-auto = Automatyczne wykrywanie celu
translate-unicode-windows = NFC (Windows / Linux)
translate-unicode-macos = Pozostaw bez zmian (macOS / APFS)
translate-line-endings-label = Konwertuj końce wierszy w plikach tekstowych
translate-line-endings-allowlist = Rozszerzenia plików tekstowych
reserved-name-label = Obsługa nazw zastrzeżonych w systemie Windows
reserved-name-suffix = Dodaj „_” (CON.txt → CON_.txt)
reserved-name-reject = Odrzuć i ostrzeż
long-path-label = Używaj prefiksu długich ścieżek systemu Windows (\\?\) powyżej 260 znaków
long-path-hint = Niektóre udziały sieciowe i starsze narzędzia nie obsługują przestrzeni nazw \\?\.

# Phase 31 — power-aware copying. Settings → Power & State tab + the
# header badge that renders "⏸ Paused — Zoom call detected" when the
# runner's power subscriber has paused or capped due to a policy match.
power-heading = Zasilanie i stan
power-enabled = Włącz reguły zależne od zasilania
power-battery-label = Na baterii
power-metered-label = Przy taryfowym Wi-Fi
power-cellular-label = W sieci komórkowej
power-presentation-label = Podczas prezentacji (Zoom / Teams / Keynote)
power-fullscreen-label = Gdy aplikacja jest na pełnym ekranie
power-thermal-label = Gdy procesor ogranicza wydajność z powodu temperatury
power-rule-continue = Kontynuuj z pełną prędkością
power-rule-pause = Wstrzymaj wszystkie zadania
power-rule-cap = Ogranicz przepustowość
power-rule-cap-percent = Ogranicz do procentu bieżącej prędkości
power-reason-on-battery = na baterii
power-reason-metered-network = sieć taryfowa
power-reason-cellular-network = sieć komórkowa
power-reason-presenting = tryb prezentacji
power-reason-fullscreen = aplikacja pełnoekranowa
power-reason-thermal-throttling = procesor ogranicza wydajność

# Phase 32 — cloud backend matrix via OpenDAL. Settings → Remotes
# tab + the Add-backend wizard that writes one entry per remote into
# the keychain-backed credential store.
remote-heading = Zaplecza zdalne
remote-add = Dodaj zaplecze
remote-list-empty = Nie skonfigurowano żadnych zapleczy zdalnych
remote-test = Testuj połączenie
remote-test-success = Połączenie udane
remote-test-failed = Połączenie nieudane
remote-remove = Usuń zaplecze
remote-name-label = Nazwa wyświetlana
remote-kind-label = Typ zaplecza
remote-save = Zapisz zaplecze
remote-cancel = Anuluj
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
backend-local-fs = Lokalny system plików
cloud-config-bucket = Zasobnik
cloud-config-region = Region
cloud-config-endpoint = Adres URL punktu końcowego
cloud-config-root = Ścieżka główna
cloud-error-invalid-config = Konfiguracja zaplecza jest nieprawidłowa
cloud-error-network = Błąd sieci podczas łączenia z zapleczem
cloud-error-not-found = Nie znaleziono obiektu w żądanej ścieżce
cloud-error-permission = Zdalne zaplecze odmówiło dostępu
cloud-error-keychain = Dostęp do pęku kluczy systemu nie powiódł się
settings-tab-remotes = Zaplecza zdalne
settings-tab-mobile = Urządzenia mobilne

# Phase 33 — mount Copy That's chunk store + history archive as a
# read-only filesystem (FUSE / WinFsp). Surfaced by the History tab
# context menu's "Mount snapshot" action + the Settings → Advanced
# "Mount latest on launch" toggle.
mount-heading = Podłącz migawkę
mount-action-mount = Podłącz migawkę
mount-action-unmount = Odłącz
mount-status-mounted = Podłączono w { $path }
mount-error-unsafe-mountpoint = Ścieżka punktu podłączenia jest niebezpieczna
mount-error-mountpoint-not-empty = Punkt podłączenia musi być pustym katalogiem
mount-error-backend-unavailable = Zaplecze podłączania jest niedostępne w tym systemie
mount-error-archive-read = Odczyt archiwum nie powiódł się
mount-picker-title = Wybierz katalog punktu podłączenia
mount-toast-mounted = Podłączono migawkę w { $path }
mount-toast-unmounted = Odłączono migawkę
mount-toast-failed = Podłączanie nie powiodło się: { $reason }
settings-mount-heading = Podłączanie migawek
settings-mount-hint = Udostępnij archiwum historii jako system plików tylko do odczytu. Faza 33b okabluje przepływ uruchamiania; jądrowe zaplecza FUSE/WinFsp pojawią się w fazie 33c.
settings-mount-on-launch = Podłączaj najnowszą migawkę przy uruchamianiu
settings-mount-on-launch-path = Ścieżka punktu podłączenia
settings-mount-on-launch-path-placeholder = np. C:\Mounts\copythat

# Phase 34 — enterprise-grade audit log export + WORM (write-once-read-
# many) tamper-resistance. Settings → Advanced → Audit log collects the
# format + destination + rotation + WORM toggle; runtime records job and
# file events into the tamper-evident chain-hashed sink.
settings-audit-heading = Dziennik audytu
settings-audit-hint = Dziennik tylko do dopisywania, odporny na manipulacje, zawierający każde zdarzenie zadania i pliku. Formaty obejmują CSV, JSON-lines, RFC 5424 Syslog, ArcSight CEF oraz QRadar LEEF.
settings-audit-enable = Włącz rejestrowanie audytu
settings-audit-format = Format dziennika
settings-audit-format-json-lines = JSON lines (zalecane domyślnie)
settings-audit-format-csv = CSV (przyjazny arkuszom)
settings-audit-format-syslog = Syslog (RFC 5424)
settings-audit-format-cef = CEF (ArcSight)
settings-audit-format-leef = LEEF 2.0 (IBM QRadar)
settings-audit-file-path = Ścieżka pliku dziennika
settings-audit-file-path-placeholder = np. C:\ProgramData\CopyThat\audit.log
settings-audit-max-size = Rotuj po (bajty, 0 = nigdy)
settings-audit-worm = Włącz tryb WORM (write-once-read-many)
settings-audit-worm-hint = Stosuje flagę tylko do dopisywania danej platformy (Linux chattr +a, macOS chflags uappnd, atrybut tylko do odczytu w systemie Windows) po każdym utworzeniu lub rotacji. Nawet administrator musi jawnie zdjąć flagę, aby skrócić dziennik.
settings-audit-test-write = Testowy zapis
settings-audit-verify-chain = Weryfikuj łańcuch
toast-audit-test-write-ok = Testowy zapis dziennika audytu powiódł się
toast-audit-verify-ok = Łańcuch audytu zweryfikowany jako nienaruszony
toast-audit-verify-failed = Weryfikacja łańcucha audytu zgłosiła niezgodności

# Phase 35 — destination encryption via age (X25519 / passphrase /
# SSH) + on-the-fly zstd compression with a per-extension deny
# heuristic. Surfaced through Settings → Transfer → Encryption +
# Compression; engine short-circuits to the crypt pipeline when
# either stage is active.
settings-crypt-heading = Szyfrowanie i kompresja
settings-crypt-hint = Przekształcaj zawartość plików, zanim trafi do celu. Szyfrowanie korzysta z formatu age; kompresja korzysta z zstd i może pomijać już skompresowane multimedia według rozszerzenia.
settings-crypt-encryption-mode = Szyfrowanie
settings-crypt-encryption-off = Wyłączone
settings-crypt-encryption-passphrase = Hasło (pytanie na początku kopiowania)
settings-crypt-encryption-recipients = Klucze odbiorców z pliku
settings-crypt-encryption-hint = Hasła są przechowywane wyłącznie w pamięci na czas kopiowania. Pliki odbiorców zawierają jeden klucz publiczny age1… lub ssh- w wierszu.
settings-crypt-recipients-file = Ścieżka pliku odbiorców
settings-crypt-recipients-file-placeholder = np. C:\Users\me\recipients.txt
settings-crypt-compression-mode = Kompresja
settings-crypt-compression-off = Wyłączona
settings-crypt-compression-always = Zawsze
settings-crypt-compression-smart = Inteligentna (pomijaj już skompresowane multimedia)
settings-crypt-compression-hint = Tryb inteligentny pomija jpg, mp4, zip, 7z i podobne formaty, które nie zyskują na zstd. Tryb „zawsze” kompresuje każdy plik na wybranym poziomie.
settings-crypt-compression-level = poziom zstd (1-22)
settings-crypt-compression-level-hint = Niższe liczby są szybsze; wyższe kompresują mocniej. Poziom 3 odpowiada domyślnemu w CLI zstd.
compress-footer-savings = 💾 { $original } → { $compressed } (zaoszczędzono { $percent }%)
compress-savings-toast = Skompresowano o { $percent }% (zaoszczędzono { $bytes })
crypt-toast-recipients-loaded = Wczytano odbiorców szyfrowania: { $count }
crypt-toast-recipients-error = Nie udało się wczytać odbiorców: { $reason }
crypt-toast-passphrase-required = Szyfrowanie wymaga hasła przed rozpoczęciem kopiowania
crypt-toast-passphrase-set = Przechwycono hasło szyfrowania
crypt-footer-encrypted-badge = 🔒 Zaszyfrowano (age)
crypt-footer-compressed-badge = 📦 Skompresowano (zstd)

# Phase 36 — copythat CLI surface. Documented exit codes + the
# user-facing strings for plan/apply/verify/config. Help text stays
# in English (engineering accessibility) but error / info / status
# strings are localized so a localized desktop installation surfaces
# CLI errors in the same language as the GUI.
cli-help-tagline = Copy That CLI — kopiowanie, synchronizacja, weryfikacja i audyt plików co do bajta dla potoków CI/CD.
cli-help-exit-codes = Kody wyjścia: 0 sukces, 1 błąd, 2 oczekujące, 3 konflikt, 4 błąd weryfikacji, 5 sieć, 6 uprawnienia, 7 pełny dysk, 8 anulowano, 9 konfiguracja.
cli-error-bad-args = copy/move wymaga co najmniej jednego źródła i celu
cli-error-unknown-algo = Nieznany algorytm weryfikacji: { $algo }
cli-error-missing-spec = --spec jest wymagane dla plan/apply
cli-error-spec-parse = Nie udało się przeanalizować jobspec { $path }: { $reason }
cli-error-spec-empty-sources = Lista źródeł jobspec jest pusta
cli-info-shape-recorded = Zarejestrowano kształt przepustowości „{ $rate }”; egzekwowanie jest okablowane przez copythat-shape
cli-info-stub-deferred = { $command } jest przygotowane do dalszego okablowania w fazie 36
cli-plan-summary = Plan: akcji { $actions }, bajtów { $bytes }; { $already_done } już na miejscu
cli-plan-pending = Plan zgłasza oczekujące akcje; uruchom ponownie z `apply`, aby je wykonać
cli-plan-already-done = Plan zgłasza brak działań (idempotentny)
cli-apply-success = Apply zakończone bez błędów
cli-apply-failed = Apply zakończone z co najmniej jednym błędem
cli-verify-ok = Weryfikacja udana: { $algo } { $digest }
cli-verify-failed = Weryfikacja NIEUDANA dla { $path } ({ $algo })
cli-config-set = Ustawiono { $key } = { $value }
cli-config-reset = Zresetowano { $key } do wartości domyślnej
cli-config-unknown-key = Nieznany klucz konfiguracji: { $key }
cli-completions-emitted = Uzupełnienia powłoki dla { $shell } wypisane do stdout

# Phase 37 — desktop-side mobile companion. Settings → Mobile panel
# strings + the SAS-confirmation modal + push-notification toasts.
# The actual mobile UI lives in the Phase 37 follow-up Tauri Mobile
# target; the phone displays its own localized SAS prompt.
settings-mobile-heading = Towarzysz mobilny
settings-mobile-hint = Sparuj iPhone lub telefon z systemem Android, aby przeglądać historię, uruchamiać zapisane profile i jobspeki z fazy 36 oraz odbierać powiadomienia o zakończeniu.
settings-mobile-pair-toggle = Zezwalaj na nowe parowania
settings-mobile-pair-active = Serwer parowania aktywny — zeskanuj kod QR aplikacją mobilną Copy That
settings-mobile-pair-button = Rozpocznij parowanie
settings-mobile-revoke-button = Cofnij
settings-mobile-no-pairings = Brak sparowanych urządzeń
settings-mobile-pair-port = Port nasłuchu (0 = wybierz wolny)
pair-sas-prompt = Oba ekrany powinny pokazywać te same cztery emoji. Stuknij Zgodne, jeśli się zgadzają.
pair-sas-confirm = Zgodne
pair-sas-reject = Niezgodne — anuluj
pair-toast-success = Sparowano z { $device }
pair-toast-failed = Parowanie nie powiodło się: { $reason }
push-toast-sent = Wysłano powiadomienie do { $device }
push-toast-failed = Powiadomienie do { $device } nie powiodło się: { $reason }

# Phase 38 — aggregate destination dedup + reflink fallback ladder.
# Settings → Transfer → Dedup panel + per-job-row badges (⚡
# Reflinked / 🔗 Hardlinked / 🧩 Chunk-shared / 📋 Copied) +
# pre-pass dedup-scan modal.
settings-dedup-heading = Deduplikacja w celu
settings-dedup-hint = Gdy źródło i cel są na tym samym woluminie, Copy That może klonować pliki na poziomie systemu plików zamiast kopiować bajty. Reflink jest natychmiastowy i bezpieczny; hardlink jest szybszy, ale obie nazwy współdzielą stan.
settings-dedup-mode-auto = Drabina automatyczna (reflink → hardlink → fragment → kopia)
settings-dedup-mode-reflink-only = Tylko reflink
settings-dedup-mode-hardlink-aggressive = Agresywny (reflink + hardlink nawet na plikach zapisywalnych)
settings-dedup-mode-off = Wyłączona (zawsze kopiuj bajty)
settings-dedup-hardlink-policy = Zasada hardlink
settings-dedup-prescan = Wstępnie przeskanuj drzewo docelowe pod kątem zduplikowanej treści
dedup-badge-reflinked = ⚡ Reflink
dedup-badge-hardlinked = 🔗 Hardlink
dedup-badge-chunk-shared = 🧩 Współdzielone fragmenty
dedup-badge-copied = 📋 Skopiowano
phase42-paranoid-verify-label = Weryfikacja paranoiczna
phase42-paranoid-verify-hint = Usuwa buforowane strony celu i ponownie odczytuje z dysku, aby wychwycić kłamstwa bufora zapisu i ciche uszkodzenia. Około 50% wolniejsza niż domyślna weryfikacja; domyślnie wyłączona.
phase42-sharing-violation-retries-label = Liczba ponowień dla zablokowanych plików źródłowych
phase42-sharing-violation-retries-hint = Ile razy ponawiać, gdy inny proces trzyma plik źródłowy otwarty z blokadą na wyłączność. Odstęp podwaja się przy każdej próbie (domyślnie 50 ms / 100 ms / 200 ms). Domyślnie 3, zgodnie z Robocopy /R:3.
phase42-cloud-placeholder-warning = { $name } to plik OneDrive dostępny tylko w chmurze. Skopiowanie go uruchomi pobieranie — do { $size } przez Twoje połączenie sieciowe.
phase42-defender-exclusion-hint = Dla maksymalnej przepustowości kopiowania dodaj folder docelowy do wykluczeń Microsoft Defender przed transferami zbiorczymi. Zobacz docs/PERFORMANCE_TUNING.md.

# Phase 39 — Browser-accessible recovery UI. Settings → Advanced
# exposes these strings; the recovery server itself renders askama
# templates that always read English — these eight keys are the
# Settings prose the user actually reads in their preferred locale.
settings-recovery-heading = Webowy interfejs odzyskiwania
settings-recovery-enable = Włącz webowy interfejs odzyskiwania
settings-recovery-bind-address = Adres nasłuchu
settings-recovery-port = Port (0 = wybierz wolny)
settings-recovery-show-url = Pokaż adres URL i token
settings-recovery-rotate-token = Obróć token
settings-recovery-allow-non-loopback = Zezwalaj na nasłuch poza pętlą zwrotną
settings-recovery-non-loopback-warning = OSTRZEŻENIE: włączenie nasłuchu poza pętlą zwrotną udostępnia interfejs odzyskiwania Twojej sieci lokalnej. Każdy, kto pozna token, może przeglądać historię plików i pobierać pliki. Zabezpiecz go przez TLS lub odwrotny serwer proxy, jeśli sieć LAN jest niezaufana.

# Phase 40 — SMB compression negotiation + cloud-VM offload helper.
# 6 SMB keys (header badge + Settings prose) + 6 cloud-offload keys
# (Remotes tab wizard for cross-cloud copy templates).
smb-compress-badge = 🗜 Kompresja SMB: { $algo }
smb-compress-badge-tooltip = Ruch sieciowy do tego celu jest kompresowany w trakcie przesyłania (SMB 3.1.1).
smb-compress-toast-saved = Zaoszczędzono { $bytes } w sieci
smb-compress-algo-unknown = nieznany algorytm
settings-smb-compress-heading = Kompresja sieciowa SMB
settings-smb-compress-hint = Automatycznie negocjuj kompresję ruchu SMB 3.1.1 na celach UNC. Darmowy zysk na wolnych łączach; pomijana przy celach lokalnych.
cloud-offload-heading = Pomocnik odciążania przez maszynę w chmurze
cloud-offload-hint = Przy kopiowaniu bezpośrednio między dwiema chmurami wygeneruj szablon wdrożenia, który uruchamia kopiowanie z niewielkiej, tymczasowej maszyny wirtualnej w chmurze — bajty nigdy nie przechodzą przez sieć Twojego laptopa.
cloud-offload-render-button = Wygeneruj szablon
cloud-offload-copy-clipboard = Kopiuj do schowka
cloud-offload-template-format = Format szablonu
cloud-offload-self-destruct-warning = Maszyna wirtualna automatycznie wyłączy się po { $minutes } min — potwierdź rolę IAM i region przed wdrożeniem.

# Phase 41 — animated before/after tree-diff preview. The `Preview
# changes` modal renders the rolled-up plan before the engine starts
# work; 14 keys cover the title, the summary header / counts, the
# row-reason labels, and the two action buttons.
preview-modal-title = Podgląd zmian
preview-summary-header = Co się stanie
preview-category-additions = dodań: { $count }
preview-category-replacements = zamian: { $count }
preview-category-skips = pominięć: { $count }
preview-category-conflicts = konfliktów: { $count }
preview-category-unchanged = bez zmian: { $count }
preview-bytes-to-transfer = { $bytes } do przesłania
preview-reason-source-newer = Źródło jest nowsze
preview-reason-dest-newer = Cel jest nowszy — zostanie pominięty
preview-reason-content-different = Treść się różni
preview-reason-identical = Identyczny ze źródłem
preview-button-run = Uruchom plan
preview-button-reduce = Ogranicz mój plan…

# Phase 42 — perceptual-hash visual-similarity dedup. Eight keys cover
# the pre-copy "looks visually identical" warning + the Settings panel.
perceptual-warn-title = Wygląda identycznie wizualnie
perceptual-warn-body = { $name } w celu wydaje się pasować do obrazu źródłowego. Kontynuować kopiowanie mimo to?
perceptual-warn-keep-both = Zachowaj oba
perceptual-warn-skip = Pomiń ten plik
perceptual-warn-overwrite = Nadpisz mimo to
perceptual-settings-heading = Deduplikacja wizualnego podobieństwa
perceptual-settings-hint = Wykrywaj wizualnie identyczne obrazy w celu, zanim zostaną nadpisane. Skrót jest percepcyjny (rozpoznaje ten sam obraz zapisany ponownie w innym formacie), a nie dokładny co do bajta.
perceptual-settings-threshold-label = Próg ostrzeżenia (niżej = ściślejsze dopasowanie)

# Phase 42 Part B — per-file rolling versions (Time Machine for any
# destination). 8 keys cover the version-list panel + retention picker.
version-list-heading = Poprzednie wersje
version-list-empty = Brak wcześniejszych wersji tego pliku
version-list-restore = Przywróć tę wersję
version-retention-heading = Zachowuj poprzednie wersje przy nadpisaniu
version-retention-none = Zachowuj każdą wersję na zawsze
version-retention-last-n = Zachowuj ostatnie wersje: { $n }
version-retention-older-than-days = Usuwaj wersje starsze niż { $days } dni
version-retention-gfs = Co godzinę { $h } · dziennie { $d } · tygodniowo { $w } · miesięcznie { $m }

# Phase 43 — forensic chain-of-custody manifests + BLAKE3 verified
# streaming. 14 keys cover the Settings → Provenance panel
# (heading, hint, toggles, TSA URL, signing-key management) plus
# the post-job manifest notification + the verify command's two
# headline result lines.
provenance-settings-heading = Kryminalistyczny łańcuch nadzoru
provenance-settings-hint = Podpisuj każde zadanie kopiowania manifestem BLAKE3 + ed25519. Recenzenci mogą później ponownie zahaszować drzewo docelowe i udowodnić, że żaden bajt nie zmienił się od czasu kopiowania.
provenance-settings-enable-default = Domyślnie podpisuj każde nowe zadanie
provenance-settings-show-after-job = Pokazuj manifest po każdym ukończonym zadaniu
provenance-settings-tsa-url-label = Domyślny adres URL urzędu znaczników czasu RFC 3161
provenance-settings-tsa-url-hint = Opcjonalnie. Po ustawieniu manifesty zawierają darmowy znacznik czasu TSA potwierdzający, że bajty istniały w danym momencie. Pozostaw puste, aby pominąć.
provenance-settings-keys-heading = Klucze podpisujące
provenance-settings-keys-generate = Wygeneruj nowy klucz
provenance-settings-keys-import = Importuj klucz…
provenance-settings-keys-export = Eksportuj klucz publiczny…
provenance-job-completed-title = Zapisano manifest pochodzenia
provenance-job-completed-body = Podpisano plików: { $count } → { $path }
provenance-verify-clean = Manifest prawidłowy dla plików: { $count }; podpis { $sig }; korzeń Merkle OK.
provenance-verify-tampered = Manifest NIEPRAWIDŁOWY — zmanipulowanych { $tampered }, brakujących { $missing }.
# Phase 43 post-review hardening — toast text for the Settings →
# Provenance buttons whose Tauri IPC has not yet landed.
provenance-action-staged = Faza 43 — okablowanie IPC dla tej akcji pojawi się w kolejnym commicie.

# Phase 44 — SSD-aware whole-drive sanitize (NVMe Sanitize / OPAL
# Crypto Erase) + the per-file shred refusal on copy-on-write
# filesystems. 16 keys cover the new "Drive sanitize" Settings
# subsection and the localized error messages.
sanitize-heading = Bezpieczne czyszczenie całego dysku
sanitize-hint = NVMe Sanitize, OPAL Crypto Erase oraz ATA Secure Erase czyszczą dysk flash na poziomie oprogramowania układowego w milisekundach. Nadpisywanie pojedynczych plików jest bezsensowne na flash — wielokrotne niszczenie tylko zużywa NAND. Użyj tego do faktycznego usunięcia.
sanitize-pick-device = Wybierz dysk do wyczyszczenia
sanitize-mode-label = Metoda czyszczenia
sanitize-mode-nvme-format = NVMe Format (z bezpiecznym kasowaniem)
sanitize-mode-nvme-sanitize-block = NVMe Sanitize — Block Erase (wolne, każda komórka)
sanitize-mode-nvme-sanitize-crypto = NVMe Sanitize — Crypto Erase (natychmiastowe)
sanitize-mode-ata-secure-erase = ATA Secure Erase (starsze dyski SATA SSD)
sanitize-mode-opal-crypto-erase = TCG OPAL Crypto Erase (dyski samoszyfrujące)
sanitize-mode-apfs-crypto-erase = APFS Crypto Erase (rotacja klucza FileVault, tylko macOS)
sanitize-confirm-1 = To niszczy KAŻDY bajt na { $device }. Nie ma cofnięcia.
sanitize-confirm-2 = Rozumiem, że wszystkie partycje, wszystkie pliki i wszystkie migawki na { $device } staną się trwale nieczytelne.
sanitize-confirm-3 = Wpisz nazwę modelu dysku, aby kontynuować: { $model }
sanitize-running = Czyszczenie { $device } ({ $mode }) — może potrwać od milisekund (crypto erase) do dziesiątek minut (block erase). Nie wyłączaj zasilania.
sanitize-completed = Czyszczenie ukończone — { $device } jest teraz pusty.
ssd-honest-shred-meaningless = Niszczenie pojedynczych plików w systemie plików kopiowania przy zapisie (Btrfs / ZFS / APFS) nie może dotrzeć do bloków bazowych. Zamiast tego użyj czyszczenia całego dysku oraz rotacji klucza szyfrowania całego dysku.
ssd-honest-advisory = Ten plik znajduje się na flash. Nadpisywanie pojedynczego pliku zużywa NAND i NIE gwarantuje, że oryginalne komórki będą nieodzyskiwalne. W przypadku danych wrażliwych wyczyść cały dysk.

# Phase 44.1f post-review — placeholder toast for SanitizeTab
# buttons whose Tauri IPC has not yet landed.
sanitize-action-staged = Faza 44.1 — okablowanie IPC dla tej akcji pojawi się w kolejnym commicie.

# Phase 45.3 — named-queue tab strip (Subfeature A). Tabs surface
# once the QueueRegistry holds at least one queue; the synthesised
# default tab keeps legacy single-queue jobs reachable.
queue-tab-default = Domyślna
queue-tab-empty-state = Kolejki zadań
queue-badge-tooltip = Oczekujące i wykonywane zadania w tej kolejce

# Phase 45.4 — drag-progress-merge (Subfeature B). Drag a queue tab
# onto another to merge their job lists. The default tab is neither
# draggable nor a drop target; only registry queues participate.
queue-drag-hint = Przeciągnij na inną kolejkę, aby je scalić
queue-merge-confirm = Upuść, aby scalić
queue-merge-toast = Scalono kolejki

# Phase 45.5 — F2-queue UX (Subfeature C). F2 toggles
# `auto_enqueue_next` so every fresh enqueue piles into the running
# queue rather than spawning a parallel one. Status pill renders in
# the Footer; pulsing dot renders on the running tab.
queue-f2-active-hint = Tryb F2: każde nowe dodanie trafia do tej kolejki
queue-f2-toggled-on = Tryb kolejki F2 WŁ. — nowe dodania dołączają do bieżącej kolejki
queue-f2-toggled-off = Tryb kolejki F2 WYŁ. — nowe dodania tworzą równoległe kolejki
queue-f2-status-bar = Tryb kolejki F2: WŁ.

# Phase 45.6 — tray destination targets (Subfeature D). Pinned
# destinations appear in the OS tray menu; clicking one arms it as
# the active drop target so the next file drop bypasses the
# DropStagingDialog. Settings → General hosts the list editor.
tray-target-section-title = Cele w zasobniku
tray-target-section-hint = Przypięte cele pojawiają się w menu zasobnika. Kliknij jeden, aby uzbroić go jako kolejny cel upuszczania.
tray-target-empty = Nie przypięto jeszcze żadnych celów w zasobniku.
tray-target-remove = Usuń
tray-target-add-label = Etykieta
tray-target-add-path = Ścieżka lub identyfikator URI zaplecza
tray-target-add = Dodaj
tray-target-armed-toast = Upuść kolejny plik, aby wysłać go do { $label }
tray-target-active-pill = → { $label }

# Phase 45.7 follow-up — pinned-destination validation errors. The
# `queue_pin_destination` IPC returns these Fluent keys verbatim
# when input fails the IPC-boundary checks; the toast layer renders
# them via `t(...)` (Toast.svelte detects kebab-case-lowercase and
# routes through the locale table). Phase 17e's `err-path-escape` /
# `err-destination-empty` cover the path-traversal + empty cases for
# `queue_route_job`; the keys below are pin-specific.
err-pinned-destination-label-empty = Etykieta celu w zasobniku nie może być pusta.
err-pinned-destination-path-empty = Ścieżka celu w zasobniku nie może być pusta.
err-pinned-destination-label-too-long = Etykieta celu w zasobniku jest za długa (maks. 64 znaki).
err-pinned-destination-path-too-long = Ścieżka celu w zasobniku jest za długa (maks. 1024 znaki).
err-pinned-destination-label-invalid = Etykieta celu w zasobniku zawiera niedozwolone znaki (nowy wiersz, powrót karetki lub NUL).
err-pinned-destination-path-invalid = Ścieżka celu w zasobniku zawiera niedozwolone znaki (nowy wiersz, powrót karetki lub NUL).
err-pinned-destination-too-many = Osiągnięto limit 50 celów w zasobniku. Usuń jeden, aby dodać kolejny.

# Phase 46.6 — Settings → Plugins tab. The IPC layer in
# `apps/copythat-ui/src-tauri/src/plugin_commands.rs` enumerates the
# per-user plugin store under `<config_dir>/plugins/`; the
# `PluginsTab.svelte` component renders these strings against the
# returned manifest + grant state.
settings-tab-plugins = Wtyczki
plugin-heading = Wtyczki
plugin-hint = Izolowane wtyczki WASM rozszerzają Copy That o niestandardowe zaczepy. Każda wtyczka działa w ramach limitów procesora i pamięci na wywołanie i widzi tylko nadane jej możliwości hosta.
plugin-list-empty = Nie zainstalowano jeszcze żadnych wtyczek.
plugin-enabled = Włączona
plugin-disabled = Wyłączona
plugin-hooks = Zaczepy
plugin-capabilities = Możliwości
plugin-no-capabilities = (brak)
plugin-directory = Lokalizacja
plugin-install-from-file = Zainstaluj z pliku…
plugin-install-from-url = Zainstaluj z adresu URL…
plugin-url-wasm = Adres URL WASM
plugin-url-manifest = Adres URL manifestu
plugin-url-hash = Skrót BLAKE3
plugin-url-preview = Podgląd
plugin-url-confirm = Potwierdź instalację
