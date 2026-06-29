app-name = Copy That v0.19.84
window-title = Copy That v0.19.84
shred-ssd-advisory = Warnung: Dieses Ziel liegt auf einer SSD. Mehrfaches Überschreiben säubert Flash-Speicher nicht zuverlässig, da Wear-Leveling und Over-Provisioning die Daten unter der logischen Blockadresse wegbewegen. Für Solid-State-Medien nutze stattdessen ATA SECURE ERASE, NVMe Format mit Secure Erase oder eine vollständige Festplattenverschlüsselung mit verworfenem Schlüssel.

# Global aggregate states (header pill)
state-idle = Bereit
state-copying = Kopieren
state-verifying = Prüfen
state-paused = Pausiert
state-error = Fehler

# Per-job states (row badge)
state-pending = In Warteschlange
state-running = Läuft
state-cancelled = Abgebrochen
state-succeeded = Fertig
state-failed = Fehlgeschlagen

# Actions
action-pause = Pausieren
action-resume = Fortsetzen
action-cancel = Abbrechen
action-pause-all = Alle Aufträge pausieren
action-resume-all = Alle Aufträge fortsetzen
action-cancel-all = Alle Aufträge abbrechen
action-close = Schließen
action-reveal = Im Ordner anzeigen
action-add-files = Dateien hinzufügen
action-add-folders = Ordner hinzufügen

# Phase 13d — activity feed
activity-title = Aktivität
activity-clear = Aktivitätsliste leeren
activity-empty = Noch keine Dateiaktivität.
activity-after-done = Wenn fertig:
activity-keep-open = App geöffnet lassen
activity-close-app = App schließen
activity-shutdown = PC herunterfahren
activity-logoff = Abmelden
activity-sleep = Energiesparmodus

# Phase 14 — preflight free-space dialog
preflight-block-title = Nicht genügend Speicherplatz am Ziel
preflight-warn-title = Wenig Speicherplatz am Ziel
preflight-unknown-title = Freier Speicherplatz konnte nicht ermittelt werden
preflight-unknown-body = Die Quelle ist zu groß, um ihre Größe schnell zu bestimmen, oder das Ziellaufwerk hat nicht reagiert. Du kannst fortfahren; die Speicherplatzüberwachung der Engine stoppt den Kopiervorgang sauber, falls der Platz ausgeht.
preflight-required = Erforderlich
preflight-free = Frei
preflight-reserve = Reserve
preflight-shortfall = Fehlbetrag
preflight-continue = Trotzdem fortfahren
preflight-pick-subset = Auswahl zum Kopieren treffen…
collision-modal-overwrite-older = Nur ältere überschreiben

# Phase 14e — subset picker
subset-title = Quellen zum Kopieren auswählen
subset-subtitle = Die vollständige Auswahl passt nicht auf das Ziel. Wähle die Elemente aus, die du kopieren möchtest; der Rest bleibt zurück.
subset-loading = Größen werden ermittelt…
subset-too-large = zu groß zum Zählen
subset-budget = Verfügbar
subset-remaining = Verbleibend
subset-confirm = Auswahl kopieren
history-rerun-hint = Diesen Kopiervorgang erneut ausführen — scannt jede Datei im Quellverzeichnis neu
history-clear-all = Alle löschen
history-clear-all-confirm = Zum Bestätigen erneut klicken
history-clear-all-hint = Jede Verlaufszeile löschen. Erfordert einen zweiten Klick zur Bestätigung.
toast-history-cleared = Verlauf geleert ({ $count } Zeilen entfernt)

# Phase 15 — source-list ordering in the drop staging dialog
drop-dialog-sort-label = Reihenfolge:
sort-custom = Benutzerdefiniert
sort-name-asc = Name A → Z (Dateien zuerst)
sort-name-desc = Name Z → A (Dateien zuerst)
sort-size-asc = Größe aufsteigend (Dateien zuerst)
sort-size-desc = Größe absteigend (Dateien zuerst)
sort-reorder = Neu anordnen
sort-move-top = Ganz nach oben
sort-move-up = Nach oben
sort-move-down = Nach unten
sort-move-bottom = Ganz nach unten

# Phase 16 — sort preset names for the Activity list (shorter
# wording than the DropStagingDialog variants; the Activity list
# is files-only so the "(files first)" suffix isn't relevant).
sort-name-asc-simple = Name A → Z
sort-name-desc-simple = Name Z → A
sort-size-asc-simple = Größe aufsteigend
sort-size-desc-simple = Größe absteigend
activity-sort-locked = Sortieren ist deaktiviert, während ein Kopiervorgang läuft. Pausiere oder warte, bis er fertig ist, und ändere dann die Reihenfolge.

# Phase 17 — collision-policy picker in the DropStagingDialog
drop-dialog-collision-label = Wenn eine Datei bereits existiert:
collision-policy-keep-both = Beide behalten (neue Kopie zu _2, _3, … umbenennen)
collision-policy-skip = Neue Kopie überspringen
collision-policy-overwrite = Vorhandene Datei überschreiben
collision-policy-overwrite-if-newer = Nur überschreiben, wenn neuer
collision-policy-prompt = Jedes Mal fragen

# Phase 18 — progress labels on the DropStagingDialog Start button
drop-dialog-busy-checking = Freier Speicherplatz wird geprüft…
drop-dialog-busy-enumerating = Dateien werden gezählt…
drop-dialog-busy-starting = Kopiervorgang wird gestartet…
toast-enumeration-deferred = Der Quellbaum ist groß — die vorab erstellte Dateiliste wird übersprungen; Zeilen erscheinen, sobald die Engine sie abarbeitet.

# Context menu (per-row right-click)
menu-pause = Pausieren
menu-resume = Fortsetzen
menu-cancel = Abbrechen
menu-remove = Aus Warteschlange entfernen
menu-reveal-source = Quelle im Ordner anzeigen
menu-reveal-destination = Ziel im Ordner anzeigen

# Header / toolbar
header-eta-label = Geschätzte verbleibende Zeit
header-toolbar-label = Globale Steuerung

# Footer
footer-queued = aktive Aufträge
footer-total-bytes = in Bearbeitung
footer-errors = Fehler
footer-history = Verlauf

# Empty state
empty-title = Dateien oder Ordner zum Kopieren ablegen
empty-hint = Ziehe Elemente auf das Fenster. Wir fragen nach einem Ziel und stellen dann pro Quelle einen Auftrag in die Warteschlange.
empty-region-label = Auftragsliste

# Details drawer
details-drawer-label = Auftragsdetails
details-source = Quelle
details-destination = Ziel
details-state = Status
details-bytes = Bytes
details-files = Dateien
details-speed = Geschwindigkeit
details-eta = Restzeit
details-error = Fehler

# Drop dialog
drop-dialog-title = Abgelegte Elemente übertragen
drop-dialog-subtitle = { $count } Element(e) bereit zur Übertragung. Wähle einen Zielordner, um zu beginnen.
drop-dialog-mode = Vorgang
drop-dialog-copy = Kopieren
drop-dialog-move = Verschieben
drop-dialog-pick-destination = Ziel wählen
drop-dialog-change-destination = Ziel ändern
drop-dialog-start-copy = Kopieren starten
drop-dialog-start-move = Verschieben starten

# ETA placeholders
eta-calculating = wird berechnet…
eta-unknown = unbekannt

# Toast messages
toast-job-done = Übertragung abgeschlossen
toast-copy-queued = Kopiervorgang in Warteschlange
toast-move-queued = Verschieben in Warteschlange
toast-error-resolved = Fehler behoben
toast-collision-resolved = Konflikt gelöst
toast-elevated-unavailable = Wiederholung mit erhöhten Rechten kommt in Phase 17 — noch nicht verfügbar
toast-clipboard-files-detected = Dateien in der Zwischenablage — drücke dein Einfügen-Kürzel, um mit Copy That zu kopieren
toast-clipboard-no-files = In der Zwischenablage sind keine Dateien zum Einfügen
toast-error-log-exported = Fehlerprotokoll exportiert

# Error modal (Phase 8)
error-modal-title = Eine Übertragung ist fehlgeschlagen
error-modal-retry = Wiederholen
error-modal-retry-elevated = Mit erhöhten Rechten wiederholen
error-modal-skip = Überspringen
error-modal-skip-all-kind = Alle Fehler dieser Art überspringen
error-modal-abort = Alle abbrechen
error-modal-path-label = Pfad
error-modal-code-label = Code
error-drawer-pending-count = Weitere Fehler warten
error-drawer-toggle = Ein- oder ausklappen

# Error-kind labels (Phase 8). Source of truth — engine maps each
# `CopyErrorKind` to one of these keys via `localized_key()`.
err-not-found = Datei nicht gefunden
err-permission-denied = Zugriff verweigert
err-disk-full = Ziellaufwerk ist voll
err-interrupted = Vorgang unterbrochen
err-verify-failed = Prüfung nach dem Kopieren fehlgeschlagen
err-path-escape = Pfad abgelehnt — enthält übergeordnete Verzeichnissegmente (..) oder unzulässige Bytes
err-path-invalid-encoding = Pfad abgelehnt — Zeichenkette enthält ungültiges UTF-8 / Ersatzzeichen
err-helper-invalid-json = Privilegierter Helfer hat fehlerhaftes JSON erhalten; diese Anfrage wird ignoriert
err-helper-grant-out-of-band = GrantCapabilities muss von der Helfer-Run-Loop verarbeitet werden, nicht vom zustandslosen Handler
err-randomness-unavailable = Zufallszahlengenerator des Betriebssystems fehlgeschlagen; es kann keine Sitzungs-ID erzeugt werden
err-sparseness-mismatch = Sparse-Layout konnte am Ziel nicht erhalten werden
err-io-other = Unbekannter E/A-Fehler

# Collision modal (Phase 8)
collision-modal-title = Datei existiert bereits
collision-modal-overwrite = Überschreiben
collision-modal-overwrite-if-newer = Überschreiben, wenn neuer
collision-modal-skip = Überspringen
collision-modal-keep-both = Beide behalten
collision-modal-rename = Umbenennen…
collision-modal-apply-to-all = Auf alle anwenden
collision-modal-source = Quelle
collision-modal-destination = Ziel
collision-modal-size = Größe
collision-modal-modified = Geändert
collision-modal-hash-check = Schnelle Prüfsumme (SHA-256)
collision-modal-hash-computing = Wird berechnet…
collision-modal-hash-identical = Identisch
collision-modal-hash-different = Unterschiedlich
collision-modal-rename-placeholder = Neuer Dateiname
collision-modal-confirm-rename = Umbenennen

# Error log drawer (Phase 8)
error-log-title = Fehlerprotokoll
error-log-empty = Keine Fehler protokolliert
error-log-export-csv = CSV exportieren
error-log-export-txt = Text exportieren
error-log-clear = Protokoll leeren
error-log-col-time = Zeit
error-log-col-job = Auftrag
error-log-col-path = Pfad
error-log-col-code = Code
error-log-col-message = Meldung
error-log-col-resolution = Lösung

# History drawer (Phase 9)
history-title = Verlauf
history-empty = Noch keine Aufträge erfasst
history-unavailable = Der Kopierverlauf ist nicht verfügbar. Die App konnte den SQLite-Speicher beim Start nicht öffnen.
history-filter-any = beliebig
history-filter-kind = Art
history-filter-status = Status
history-filter-text = Suchen
history-refresh = Aktualisieren
history-export-csv = CSV exportieren
history-purge-30 = > 30 Tage bereinigen
history-rerun = Erneut ausführen
history-detail-open = Details
history-detail-title = Auftragsdetails
history-detail-empty = Keine Elemente erfasst
history-col-date = Datum
history-col-kind = Art
history-col-src = Quelle
history-col-dst = Ziel
history-col-files = Dateien
history-col-size = Größe
history-col-status = Status
history-col-duration = Dauer
history-col-error = Fehler
toast-history-exported = Verlauf exportiert
toast-history-rerun-queued = Erneute Ausführung in Warteschlange

# Totals drawer (Phase 10)
footer-totals = Summen
totals-title = Summen
totals-loading = Summen werden geladen…
totals-card-bytes = Insgesamt kopierte Bytes
totals-card-files = Dateien
totals-card-jobs = Aufträge
totals-card-avg-rate = Durchschnittlicher Durchsatz
totals-errors = Fehler
totals-spark-title = Letzte 30 Tage
totals-kinds-title = Nach Art
totals-saved-title = Gesparte Zeit (geschätzt)
totals-saved-note = Geschätzt im Vergleich zu einem Standard-Dateimanager bei gleicher Arbeitslast.
totals-reset = Statistik zurücksetzen
totals-reset-confirm = Dies löscht jeden gespeicherten Auftrag und jedes Element. Fortfahren?
totals-reset-confirm-yes = Ja, zurücksetzen
toast-totals-reset = Statistik zurückgesetzt

# Phase 11a — i18n core: surface remaining user-visible strings so
# every string on a main-window screen flows through Fluent.

# Header language switcher (temporary placement — folds into
# Settings → General in Phase 12).
header-language-label = Sprache
header-language-title = Sprache ändern

# Job-kind labels. History filter, Totals breakdown, and history
# rows all map wire-format `kind` strings to these labels.
kind-copy = Kopieren
kind-move = Verschieben
kind-delete = Löschen
kind-secure-delete = Sicher löschen

# History status labels. Distinct from `state-*` because the
# history wire format uses plain `running`/`succeeded`/... rather
# than the live `JobState` enum the queue exposes.
status-running = Läuft
status-succeeded = Erfolgreich
status-failed = Fehlgeschlagen
status-cancelled = Abgebrochen
# Per-item status (not job-level): `ok` / `skipped` are only
# reachable from the history detail view.
status-ok = OK
status-skipped = Übersprungen

# History drawer: search field placeholder + purge toast.
history-search-placeholder = /pfad
toast-history-purged = { $count } Aufträge älter als 30 Tage bereinigt

# User-facing command-layer validation errors. The Rust side
# returns these keys when input is missing; the toast layer looks
# them up rather than showing the raw English.
err-source-required = Mindestens ein Quellpfad ist erforderlich.
err-destination-empty = Der Zielpfad ist leer.
err-source-empty = Der Quellpfad ist leer.

# Localised duration formatting for the Totals drawer and ETA
# fields. `{ $ms }`, `{ $s }`, `{ $m }`, `{ $h }` are integer
# placeables — the formatter passes pre-computed values in.
duration-lt-1s = < 1 s
duration-ms = { $ms } ms
duration-seconds = { $s } s
duration-minutes-seconds = { $m } min { $s } s
duration-hours-minutes = { $h } h { $m } min
duration-zero = 0 s

# Rate unit. Appended to a formatted byte size. Some languages
# render this with a leading space ("Ko/s"); keep it translatable
# even though the SI-derived "/s" is near-universal.
rate-unit-per-second = { $size }/s

# Phase 11b — Settings modal skeleton. Phase 12 expanded this into a
# full six-tab preferences window; the `settings-phase-12-hint` key
# is retired in favour of `settings-tab-profiles` + concrete labels.
settings-title = Einstellungen
settings-tab-general = Allgemein
settings-tab-appearance = Darstellung
settings-section-language = Sprache
settings-phase-12-hint = Weitere Einstellungen (Design, Übertragungsstandards, Prüfalgorithmus, Profile) kommen in Phase 12.

# Phase 12 — full Settings window keys. Grouped by tab so the
# human-review pass can pick one section at a time. Every field
# label and every option label that the user will actually read
# has its own key; dynamic wire values (enum discriminants like
# "auto" / "prefer") stay untranslated on the wire.

settings-loading = Einstellungen werden geladen…
settings-tab-transfer = Übertragung
settings-tab-filters = Filter
settings-tab-shell = Shell
settings-tab-secure-delete = Sicher löschen
settings-tab-advanced = Erweitert
settings-tab-updater = Updates
settings-tab-profiles = Profile

# General tab additions
settings-section-theme = Design
settings-theme-auto = Automatisch
settings-theme-light = Hell
settings-theme-dark = Dunkel
settings-start-with-os = Beim Systemstart starten
settings-single-instance = Nur eine laufende Instanz
settings-minimize-to-tray = Beim Schließen in den Infobereich minimieren
settings-error-display-mode = Stil der Fehlermeldung
settings-error-display-modal = Modal (blockiert die App)
settings-error-display-drawer = Seitenleiste (nicht blockierend)
settings-error-display-mode-hint = Modal stoppt die Warteschlange, bis du entscheidest. Die Seitenleiste hält die Warteschlange in Bewegung und lässt dich Fehler in der Ecke abarbeiten.
settings-paste-shortcut = Dateien per globalem Tastenkürzel einfügen
settings-paste-shortcut-combo = Tastenkombination
settings-paste-shortcut-hint = Drücke diese Kombination überall auf deinem System, um aus Explorer / Finder / Files kopierte Dateien per Copy That einzufügen. CmdOrCtrl entspricht Cmd unter macOS, Ctrl unter Windows / Linux.
settings-clipboard-watcher = Zwischenablage auf kopierte Dateien überwachen
settings-clipboard-watcher-hint = Zeigt eine Benachrichtigung, wenn Datei-URLs in der Zwischenablage erscheinen, mit dem Hinweis, dass du per Copy That einfügen kannst. Prüft alle 500 ms, solange aktiviert.

# Transfer tab
settings-buffer-size = Puffergröße
settings-verify = Nach dem Kopieren prüfen
settings-verify-off = Aus
settings-concurrency = Parallelität
settings-concurrency-auto = Automatisch
settings-reflink = Reflink / schnelle Pfade
settings-reflink-prefer = Bevorzugen
settings-reflink-avoid = Reflink vermeiden
settings-reflink-disabled = Immer asynchrone Engine verwenden
settings-fsync-on-close = Beim Schließen auf Datenträger synchronisieren (langsamer, sicherer)
settings-preserve-timestamps = Zeitstempel beibehalten
settings-preserve-permissions = Berechtigungen beibehalten
settings-preserve-acls = ACLs beibehalten (Phase 14)
settings-preserve-sparseness = Sparse-Dateien beibehalten
settings-preserve-sparseness-hint = Kopiert nur die belegten Bereiche von Sparse-Dateien (VM-Festplatten, Datenbankdateien), sodass das Ziel dieselbe Größe auf dem Datenträger behält wie die Quelle.
settings-force-parallel-chunks = Paralleles Mehrfach-Chunk-Kopieren (nur RAID / Arrays)
settings-force-parallel-chunks-hint = Teilt jede große Kopie in gleichzeitige Chunks auf. Hilft nur bei Striped-/RAID-/Netzwerkzielen; verlangsamt eine einzelne SSD/NVMe (-25% bis -76%). Lassen Sie es aus, sofern das Ziel kein Array aus mehreren Datenträgern ist.

# Shell tab
settings-context-menu = Einträge im Shell-Kontextmenü aktivieren
settings-intercept-copy = Standard-Kopierhandler abfangen (Windows)
settings-intercept-copy-hint = Wenn aktiviert, werden Strg+C / Strg+V im Explorer über Copy That geleitet. Die Registrierung kommt in Phase 14.
settings-notify-completion = Bei Auftragsabschluss benachrichtigen

# Secure delete tab
settings-shred-method = Standard-Schreddermethode
settings-shred-zero = Nullen (1 Durchgang)
settings-shred-random = Zufällig (1 Durchgang)
settings-shred-dod3 = DoD 5220.22-M (3 Durchgänge)
settings-shred-dod7 = DoD 5220.22-M (7 Durchgänge)
settings-shred-gutmann = Gutmann (35 Durchgänge)
settings-shred-nist = NIST 800-88
settings-shred-confirm-twice = Vor dem Schreddern doppelte Bestätigung verlangen

# Advanced tab
settings-log-level = Protokollebene
settings-log-off = Aus
settings-telemetry = Telemetrie
settings-telemetry-never = Niemals — kein Phone-Home auf keiner Protokollebene
settings-error-policy = Standard-Fehlerverhalten
settings-error-policy-ask = Fragen
settings-error-policy-skip = Überspringen
settings-error-policy-retry = Mit Wartezeit wiederholen
settings-error-policy-abort = Beim ersten Fehler abbrechen
settings-history-retention = Verlauf aufbewahren (Tage)
settings-history-retention-hint = 0 = für immer behalten. Jeder andere Wert bereinigt ältere Aufträge automatisch beim Start.
settings-database-path = Datenbankpfad
settings-database-path-default = (Standard — Datenverzeichnis des Betriebssystems)
settings-reset-all = Auf Standard zurücksetzen
settings-reset-confirm = Jede Einstellung auf ihren Standard zurücksetzen? Profile sind nicht betroffen.

# Profiles tab
settings-profiles-hint = Speichere die aktuellen Einstellungen unter einem Namen; lade sie später, um zurückzuwechseln, ohne einzelne Optionen anzupassen.
settings-profile-name-placeholder = Profilname
settings-profile-save = Speichern
settings-profile-import = Importieren…
settings-profile-load = Laden
settings-profile-export = Exportieren…
settings-profile-delete = Löschen
settings-profile-empty = Noch keine Profile gespeichert.
settings-profile-import-prompt = Name für das importierte Profil:

# Toasts driven by Phase 12 profile actions
toast-settings-reset = Einstellungen zurückgesetzt
toast-profile-saved = Profil gespeichert
toast-profile-loaded = Profil geladen
toast-profile-exported = Profil exportiert
toast-profile-imported = Profil importiert

# Phase 14a — enumeration-time filters
settings-filters-hint = Überspringt Dateien beim Einlesen, sodass die Engine sie gar nicht erst öffnet. Includes gelten nur für Dateien; Excludes entfernen auch passende Verzeichnisse.
settings-filters-enabled = Filter für Baumkopien aktivieren
settings-filters-include-globs = Include-Globs
settings-filters-include-globs-placeholder = **/*.txt
settings-filters-include-globs-hint = Ein Glob pro Zeile. Wenn nicht leer, muss eine Datei mindestens einem Include entsprechen, um erhalten zu bleiben. In Verzeichnisse wird immer hinabgestiegen.
settings-filters-exclude-globs = Exclude-Globs
settings-filters-exclude-globs-placeholder = **/node_modules
settings-filters-exclude-globs-hint = Ein Glob pro Zeile. Treffer entfernen bei Verzeichnissen den gesamten Unterbaum; passende Dateien werden übersprungen.
settings-filters-size-range = Dateigrößenbereich
settings-filters-min-size-bytes = Mindestgröße (Bytes, leer = keine Untergrenze)
settings-filters-max-size-bytes = Maximalgröße (Bytes, leer = keine Obergrenze)
settings-filters-date-range = Änderungszeitraum
settings-filters-min-mtime = Geändert am oder nach
settings-filters-max-mtime = Geändert am oder vor
settings-filters-attributes = Attributbits
settings-filters-skip-hidden = Versteckte Dateien / Ordner überspringen
settings-filters-skip-system = Systemdateien überspringen (nur Windows)
settings-filters-skip-readonly = Schreibgeschützte Dateien überspringen

# Phase 15 — auto-update
settings-updater-hint = Copy That sucht höchstens einmal täglich nach signierten Updates. Updates werden beim nächsten Beenden der App installiert.
settings-updater-auto-check = Beim Start nach Updates suchen
settings-updater-channel = Release-Kanal
settings-updater-channel-stable = Stabil
settings-updater-channel-beta = Beta (Vorabversion)
settings-updater-last-check = Zuletzt geprüft
settings-updater-last-never = Niemals
settings-updater-check-now = Jetzt nach Updates suchen
settings-updater-checking = Wird geprüft…
settings-updater-available = Update verfügbar
settings-updater-up-to-date = Du nutzt die neueste Version.
settings-updater-dismiss = Diese Version überspringen
settings-updater-dismissed = Übersprungen
toast-update-available = Eine neuere Version ist verfügbar
toast-update-up-to-date = Du nutzt bereits die neueste Version

# Phase 19a — disk-backed file enumeration (TeraCopy-compatible scan DB)
scan-progress-title = Wird gescannt…
scan-progress-stats = { $files } Dateien · { $bytes } bisher
scan-pause-button = Scan pausieren
scan-resume-button = Scan fortsetzen
scan-cancel-button = Scan abbrechen
scan-cancel-confirm = Scan abbrechen und Fortschritt verwerfen?
scan-db-header = Scan-Datenbank
scan-db-hint = Datenträgerbasierte Scan-Datenbank für Aufträge mit mehreren Millionen Dateien.
advanced-scan-hash-during = Prüfsummen während des Scans berechnen
advanced-scan-db-path = Speicherort der Scan-Datenbank
advanced-scan-retention-days = Abgeschlossene Scans automatisch löschen nach (Tagen)
advanced-scan-max-keep = Maximale Anzahl aufzubewahrender Scan-Datenbanken

# Phase 19b — filesystem-snapshot source for locked files.
settings-on-locked = Wenn eine Datei gesperrt ist
settings-on-locked-ask = Beim ersten Mal fragen
settings-on-locked-retry = Kurz wiederholen, dann den Fehler anzeigen
settings-on-locked-skip = Gesperrte Datei überspringen
settings-on-locked-snapshot = Dateisystem-Snapshot verwenden
settings-on-locked-hint = Beseitigt Fehler "Datei wird von einem anderen Prozess verwendet". Copy That erstellt einen Snapshot des Quelllaufwerks (VSS unter Windows, ZFS/Btrfs unter Linux, APFS unter macOS) und liest aus der Snapshot-Kopie.
snapshot-prompt-title = Diese Datei wird von einem anderen Prozess verwendet
snapshot-prompt-body = Ein anderes Programm hat { $path } exklusiv zum Schreiben geöffnet. Wähle, wie Copy That diese und ähnliche Dateien auf demselben Laufwerk behandeln soll.
snapshot-source-active = 📷 Liest aus { $kind }-Snapshot von { $volume }
snapshot-create-failed = Es konnte kein Snapshot des Quelllaufwerks erstellt werden
snapshot-vss-needs-elevation = Das Lesen aus einem VSS-Snapshot erfordert Administratorrechte. Copy That wird dich um Erlaubnis bitten.
snapshot-cleanup-failed = Der Snapshot-Helfer hat einen Aufräumfehler gemeldet — eine übrig gebliebene Schattenkopie könnte auf dem Laufwerk verbleiben.

# Phase 20 — durable resume journal.
resume-prompt-title = Vorherige Übertragungen fortsetzen?
resume-prompt-body = Copy That hat { $count } unvollständige Übertragung(en) aus einer vorherigen Sitzung erkannt. Wähle, was mit jeder geschehen soll.
resume-prompt-resume = Fortsetzen
resume-prompt-resume-all = Alle fortsetzen
resume-discard-one = Nicht fortsetzen
resume-discard-all = Alle verwerfen
resume-aborted-hash-mismatch = Die ersten { $offset } Bytes des Ziels stimmen nicht mit der Quelle überein — der Vorgang wird von vorn begonnen.
settings-auto-resume = Unterbrochene Aufträge ohne Nachfrage automatisch fortsetzen
settings-auto-resume-hint = Überspringt die Fortsetzungsabfrage beim Start und reiht jeden unvollständigen Auftrag stillschweigend wieder ein. Standardmäßig aus.

# Phase 21 — bandwidth shaping (GCRA token bucket + schedule + auto-throttle).
settings-tab-network = Netzwerk
settings-network-hint = Begrenze deine Übertragungsrate, damit der Rest des Netzwerks nutzbar bleibt. Wende sie global an, folge einem Tagesplan oder reagiere automatisch auf getaktete WLAN- / Akku- / Mobilfunkverbindungen.
settings-network-mode = Bandbreitenbegrenzung
settings-network-mode-off = Aus (keine Begrenzung)
settings-network-mode-fixed = Fester Wert
settings-network-mode-schedule = Zeitplan verwenden
settings-network-cap-mbps = Begrenzung (MB/s)
settings-network-schedule = Zeitplan (rclone-Format)
settings-network-schedule-hint = Durch Leerzeichen getrennte HH:MM,Rate-Grenzen plus optionale Mon-Fri,Rate-Tagesregeln. Raten: 512k, 10M, 2G, off, unlimited. Beispiel: 08:00,512k 18:00,10M Sat-Sun,unlimited.
settings-network-auto-header = Automatische Drosselung
settings-network-auto-metered = Bei getaktetem WLAN
settings-network-auto-battery = Im Akkubetrieb
settings-network-auto-cellular = Bei Mobilfunk
settings-network-auto-unchanged = Nicht überschreiben
settings-network-auto-pause = Übertragungen pausieren
settings-network-auto-cap = Auf festen Wert begrenzen
shape-badge-paused = pausiert
shape-badge-tooltip = Bandbreitenbegrenzung aktiv — klicken, um Einstellungen → Netzwerk zu öffnen
shape-badge-source-schedule = geplant
shape-badge-source-metered = getaktet
shape-badge-source-battery = im Akkubetrieb
shape-badge-source-cellular = Mobilfunk
shape-badge-source-settings = aktiv
shape-error-schedule-invalid = Zeitplanformat ist ungültig: { $message }

# Phase 22 — aggregate conflict dialog v2 (thumbnails, per-pattern
# rules, and reusable conflict profiles). Every key below is user-
# visible text in the `ConflictBatchModal.svelte` component.
conflict-batch-title = { $count } Dateikonflikte in { $jobname }
conflict-batch-state-pending = Ausstehend
conflict-batch-state-resolved = Gelöst
conflict-batch-action-overwrite = Überschreiben
conflict-batch-action-skip = Überspringen
conflict-batch-action-keep-both = Beide behalten
conflict-batch-action-newer-wins = Neuere gewinnt
conflict-batch-action-larger-wins = Größere gewinnt
conflict-batch-bulk-apply-selected = Auf Auswahl anwenden
conflict-batch-bulk-apply-extension = Auf alle dieser Erweiterung anwenden
conflict-batch-bulk-apply-glob = Auf passende Glob anwenden…
conflict-batch-bulk-apply-remaining = Auf alle übrigen anwenden
conflict-batch-bulk-glob-placeholder = z. B. **/*.tmp
conflict-batch-save-profile = Diese Regeln als Profil speichern…
conflict-batch-profile-placeholder = Profilname
conflict-batch-matched-rule = über Regel '{ $rule }' → { $action }
conflict-batch-empty = Alle Konflikte gelöst
conflict-batch-source-vs-destination = Quelle vs. Ziel
conflict-batch-source-label = Quelle
conflict-batch-destination-label = Ziel
conflict-batch-size-label = Größe
conflict-batch-modified-label = Geändert
conflict-batch-close = Schließen
conflict-batch-profile-saved = Konfliktprofil gespeichert

# Phase 23 — sparse-file preservation. The toast fires once per
# destination volume when the filesystem can't preserve holes; the
# warning line is surfaced in the job detail drawer so the user knows
# the dst is larger on disk than the source was.
sparse-not-supported-title = Ziel füllt Sparse-Dateien auf
sparse-not-supported-body = { $dst_fs } unterstützt keine Sparse-Dateien. Lücken in der Quelle wurden als Nullen geschrieben, daher belegt das Ziel mehr Platz auf dem Datenträger.
sparse-warning-densified = Sparse-Layout erhalten: Es wurden nur belegte Bereiche kopiert.
sparse-warning-mismatch = Abweichung im Sparse-Layout — das Ziel könnte größer als erwartet sein.

# Phase 24 — security-metadata preservation. The Mark-of-the-Web
# (Zone.Identifier ADS) toggle is security-sensitive: turning it off
# lets a downloaded executable shed its SmartScreen / Office Protected
# View flag on copy, which is why the tooltip carries an explicit
# warning. AppleDouble fallback emits `._<filename>` sidecars on
# destination filesystems that can't hold the foreign metadata.
settings-preserve-security-metadata = Sicherheitsmetadaten beibehalten
settings-preserve-security-metadata-hint = Erfasst und überträgt bei jedem Kopiervorgang Out-of-Band-Metadatenströme (NTFS ADS / xattrs / POSIX-ACLs / SELinux-Kontexte / Linux-Dateifähigkeiten / macOS-Resource-Forks) erneut.
settings-preserve-motw = Mark-of-the-Web beibehalten (Aus-dem-Internet-heruntergeladen-Markierung)
settings-preserve-motw-hint = Sicherheitskritisch. SmartScreen und die geschützte Office-Ansicht nutzen diesen Stream, um vor aus dem Internet heruntergeladenen Dateien zu warnen. Beim Deaktivieren kann eine heruntergeladene ausführbare Datei ihre Herkunftsmarkierung beim Kopieren ablegen und Schutzmechanismen des Betriebssystems umgehen.
settings-preserve-posix-acls = POSIX-ACLs und erweiterte Attribute beibehalten
settings-preserve-posix-acls-hint = Überträgt user.* / system.* / trusted.* xattrs und POSIX-Zugriffssteuerungslisten beim Kopieren.
settings-preserve-selinux = SELinux-Kontexte beibehalten
settings-preserve-selinux-hint = Überträgt die security.selinux-Kennzeichnung beim Kopieren, damit unter MAC-Richtlinien laufende Daemons weiterhin auf die Datei zugreifen können.
settings-preserve-resource-forks = macOS-Resource-Forks und Finder-Infos beibehalten
settings-preserve-resource-forks-hint = Überträgt den klassischen Resource-Fork und FinderInfo (Farbmarkierungen, Carbon-Metadaten) beim Kopieren.
settings-appledouble-fallback = AppleDouble-Sidecar auf inkompatiblen Dateisystemen verwenden
meta-translated-to-appledouble = Fremde Metadaten in AppleDouble-Sidecar gespeichert (._{ $ext })

# Phase 25 — two-way sync with vector-clock conflict detection.
# The drawer lists configured sync pairs; each pair runs an
# independent reconciliation round against a per-pair `.copythat-sync.db`
# state store. Concurrent edits from a common ancestor surface as
# conflicts rather than silent overwrites; the losing side's content
# is preserved as `name.sync-conflict-YYYYMMDD-HHMMSS-<host>.ext`.
footer-sync = Sync
sync-drawer-title = Zwei-Wege-Synchronisierung
sync-drawer-hint = Halte zwei Ordner synchron, ohne stille Überschreibungen. Gleichzeitige Änderungen erscheinen als Konflikte, die du lösen kannst.
sync-add-pair = Paar hinzufügen
sync-add-cancel = Abbrechen
sync-refresh = Aktualisieren
sync-add-save = Paar speichern
sync-add-saving = Wird gespeichert…
sync-add-missing-fields = Bezeichnung, linker Pfad und rechter Pfad sind alle erforderlich.
sync-remove-confirm = Dieses Sync-Paar entfernen? Die Statusdatenbank bleibt erhalten; die Ordner bleiben unverändert.
sync-field-label = Bezeichnung
sync-field-label-placeholder = z. B. Dokumente ↔ NAS
sync-field-left = Linker Ordner
sync-field-left-placeholder = Absoluten Pfad auswählen oder einfügen
sync-field-right = Rechter Ordner
sync-field-right-placeholder = Absoluten Pfad auswählen oder einfügen
sync-field-mode = Modus
sync-mode-two-way = Zwei-Wege
sync-mode-mirror-left-to-right = Spiegeln (links → rechts)
sync-mode-mirror-right-to-left = Spiegeln (rechts → links)
sync-mode-contribute-left-to-right = Beitragen (links → rechts, keine Löschungen)
sync-no-pairs = Noch keine Sync-Paare konfiguriert. Klicke auf "Paar hinzufügen", um zu beginnen.
sync-loading = Konfigurierte Paare werden geladen…
sync-never-run = Nie ausgeführt
sync-running = Läuft
sync-run-now = Jetzt ausführen
sync-cancel = Abbrechen
sync-remove-pair = Entfernen
sync-view-conflicts = Konflikte ansehen ({ $count })
sync-conflicts-heading = Konflikte
sync-no-conflicts = Keine Konflikte aus dem letzten Durchlauf.
sync-winner = Gewinner
sync-side-left-to-right = links
sync-side-right-to-left = rechts
sync-conflict-kind-concurrent-write = Gleichzeitige Änderung
sync-conflict-kind-delete-edit = Löschen ↔ Bearbeiten
sync-conflict-kind-add-add = Beide Seiten hinzugefügt
sync-conflict-kind-corrupt-equal = Inhalt abweichend ohne neue Schreiboperation
sync-resolve-keep-left = Links behalten
sync-resolve-keep-right = Rechts behalten
sync-resolve-keep-both = Beide behalten
sync-resolve-three-way = Per 3-Wege-Zusammenführung lösen
sync-resolve-phase-53-tooltip = Interaktive 3-Wege-Zusammenführung für Nicht-Textdateien kommt in Phase 53.
sync-error-prefix = Sync-Fehler

# Phase 26 — real-time mirror watcher. "Live mirror" starts a
# filesystem watcher on the pair's left side; every debounced event
# triggers a re-sync. The watcher filters vim swap files / Office
# lock files / atomic-save temp names so one logical save becomes
# exactly one sync round.
live-mirror-start = Live-Spiegelung starten
live-mirror-stop = Live-Spiegelung stoppen
live-mirror-watching = Wird überwacht
live-mirror-toggle-hint = Automatisch bei jeder erkannten Dateisystemänderung neu synchronisieren. Ein Hintergrund-Thread pro aktivem Paar.
watch-event-prefix = Dateiänderung
watch-overflow-recovered = Watcher-Puffer übergelaufen; erneutes Einlesen zur Wiederherstellung

# Phase 27 — content-defined chunk store. Enables delta-resume (a
# retry only re-writes chunks that actually changed) and same-job
# dedup (files sharing content blocks store those blocks once). The
# store is disk-backed under `<data-dir>/chunks/` by default and is
# the foundation for the Phase 49–51 moonshot repository phases.
chunk-store-section = Chunk-Speicher
chunk-store-enable = Chunk-Speicher aktivieren (Delta-Fortsetzung und Dedup)
chunk-store-enable-hint = Teilt jede kopierte Datei nach Inhalt auf (FastCDC) und speichert Chunks inhaltsadressiert. Wiederholungen schreiben nur geänderte Chunks neu; Dateien mit gemeinsamen Inhalten werden automatisch dedupliziert.
chunk-store-location = Speicherort des Chunk-Speichers
chunk-store-max-size = Maximale Größe des Chunk-Speichers
chunk-store-prune = Chunks bereinigen, die älter sind als (Tage)
chunk-store-savings = { $gib } GiB durch Chunk-Dedup gespart
chunk-store-disk-usage = { $size } über { $chunks } Chunks belegt

# Phase 28 — tray-resident Drop Stack. The stack is a persistent list
# of paths gathered from multiple sources (Explorer drag, main-window
# context menu, CLI, drag onto the Drop Stack window) that the user
# can dispatch to a destination in one go.
dropstack-window-title = Drop Stack
dropstack-tray-open = Drop Stack
dropstack-empty-title = Drop Stack ist leer
dropstack-empty-hint = Ziehe Dateien aus dem Explorer hierher oder klicke mit der rechten Maustaste auf eine Auftragszeile, um sie hinzuzufügen.
dropstack-add-to-stack = Zum Drop Stack hinzufügen
dropstack-copy-all-to = Alle kopieren nach…
dropstack-move-all-to = Alle verschieben nach…
dropstack-clear = Stack leeren
dropstack-remove-row = Aus Stack entfernen
dropstack-path-missing-toast = { $path } abgelegt — die Datei existiert nicht mehr.
dropstack-always-on-top = Drop Stack immer im Vordergrund halten
dropstack-show-tray-icon = Das Copy That-Infobereichssymbol anzeigen
dropstack-open-on-start = Drop Stack beim App-Start automatisch öffnen
dropstack-count = { $count } Pfad

# Phase 29 — spring-loaded folders + native DnD polish. The Settings
# → General tab carries the knobs; DropTarget / DestinationPicker
# Svelte components use the dropzone-* keys at runtime.
settings-dnd-heading = Ziehen und Ablegen
settings-dnd-spring-load = Ordner beim Ziehen automatisch öffnen
settings-dnd-spring-delay = Verzögerung beim automatischen Öffnen (ms)
settings-dnd-thumbnails = Vorschaubilder beim Ziehen anzeigen
settings-dnd-invalid-highlight = Ungültige Ablageziele hervorheben
dropzone-invalid-title = Kein gültiges Ablageziel
dropzone-invalid-readonly = Ziel ist schreibgeschützt
dropzone-picker-title = Ziel auswählen
dropzone-picker-up = Nach oben
dropzone-picker-path = Aktueller Pfad
dropzone-picker-root = Stammverzeichnisse
dropzone-picker-use-this = Diesen Ordner verwenden
dropzone-picker-empty = Keine Unterordner
dropzone-picker-cancel = Abbrechen

# Phase 30 — cross-platform path translation. Settings → Transfer
# exposes these under a "Cross-platform compatibility" subsection.
translate-heading = Plattformübergreifende Kompatibilität
translate-unicode-label = Unicode-Normalisierung
translate-unicode-auto = Ziel automatisch erkennen
translate-unicode-windows = NFC (Windows / Linux)
translate-unicode-macos = Unverändert lassen (macOS / APFS)
translate-line-endings-label = Zeilenenden für Textdateien umwandeln
translate-line-endings-allowlist = Textdatei-Erweiterungen
reserved-name-label = Behandlung von reservierten Windows-Namen
reserved-name-suffix = "_" anhängen (CON.txt → CON_.txt)
reserved-name-reject = Ablehnen und warnen
long-path-label = Windows-Langpfad-Präfix (\\?\) bei über 260 Zeichen verwenden
long-path-hint = Manche Netzwerkfreigaben und ältere Tools beachten den \\?\-Namensraum nicht.

# Phase 31 — power-aware copying. Settings → Power & State tab + the
# header badge that renders "⏸ Paused — Zoom call detected" when the
# runner's power subscriber has paused or capped due to a policy match.
power-heading = Energie & Status
power-enabled = Energiebewusste Regeln aktivieren
power-battery-label = Im Akkubetrieb
power-metered-label = Bei getaktetem WLAN
power-cellular-label = Bei Mobilfunk
power-presentation-label = Bei Präsentationen (Zoom / Teams / Keynote)
power-fullscreen-label = Wenn eine App im Vollbild ist
power-thermal-label = Wenn die CPU thermisch drosselt
power-rule-continue = Mit voller Geschwindigkeit fortfahren
power-rule-pause = Alle Aufträge pausieren
power-rule-cap = Bandbreite begrenzen
power-rule-cap-percent = Auf einen Prozentsatz der aktuellen Rate begrenzen
power-reason-on-battery = im Akkubetrieb
power-reason-metered-network = getaktetes Netzwerk
power-reason-cellular-network = Mobilfunknetz
power-reason-presenting = Präsentationsmodus
power-reason-fullscreen = Vollbild-App
power-reason-thermal-throttling = CPU drosselt

# Phase 32 — cloud backend matrix via OpenDAL. Settings → Remotes
# tab + the Add-backend wizard that writes one entry per remote into
# the keychain-backed credential store.
remote-heading = Remote-Backends
remote-add = Backend hinzufügen
remote-list-empty = Keine Remote-Backends konfiguriert
remote-test = Verbindung testen
remote-test-success = Verbindung erfolgreich
remote-test-failed = Verbindung fehlgeschlagen
remote-remove = Backend entfernen
remote-name-label = Anzeigename
remote-kind-label = Backend-Typ
remote-save = Backend speichern
remote-cancel = Abbrechen
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
backend-local-fs = Lokales Dateisystem
cloud-config-bucket = Bucket
cloud-config-region = Region
cloud-config-endpoint = Endpunkt-URL
cloud-config-root = Stammpfad
cloud-error-invalid-config = Backend-Konfiguration ist ungültig
cloud-error-network = Netzwerkfehler beim Kontaktieren des Backends
cloud-error-not-found = Objekt am angeforderten Pfad nicht gefunden
cloud-error-permission = Zugriff vom Remote-Backend verweigert
cloud-error-keychain = Zugriff auf den Schlüsselbund des Betriebssystems fehlgeschlagen
settings-tab-remotes = Remotes
settings-tab-mobile = Mobil

# Phase 33 — mount Copy That's chunk store + history archive as a
# read-only filesystem (FUSE / WinFsp). Surfaced by the History tab
# context menu's "Mount snapshot" action + the Settings → Advanced
# "Mount latest on launch" toggle.
mount-heading = Snapshot einbinden
mount-action-mount = Snapshot einbinden
mount-action-unmount = Aushängen
mount-status-mounted = Eingebunden unter { $path }
mount-error-unsafe-mountpoint = Einbindungspfad ist unsicher
mount-error-mountpoint-not-empty = Der Einbindungspunkt muss ein leeres Verzeichnis sein
mount-error-backend-unavailable = Das Einbindungs-Backend ist auf diesem System nicht verfügbar
mount-error-archive-read = Lesen des Archivs fehlgeschlagen
mount-picker-title = Verzeichnis für Einbindungspunkt auswählen
mount-toast-mounted = Snapshot eingebunden unter { $path }
mount-toast-unmounted = Snapshot ausgehängt
mount-toast-failed = Einbinden fehlgeschlagen: { $reason }
settings-mount-heading = Snapshots einbinden
settings-mount-hint = Stellt das Verlaufsarchiv als schreibgeschütztes Dateisystem bereit. Phase 33b verdrahtet den Runner-Ablauf; die FUSE/WinFsp-Kernel-Backends kommen in Phase 33c.
settings-mount-on-launch = Den neuesten Snapshot beim Start einbinden
settings-mount-on-launch-path = Einbindungspfad
settings-mount-on-launch-path-placeholder = z. B. C:\Mounts\copythat

# Phase 34 — enterprise-grade audit log export + WORM (write-once-read-
# many) tamper-resistance. Settings → Advanced → Audit log collects the
# format + destination + rotation + WORM toggle; runtime records job and
# file events into the tamper-evident chain-hashed sink.
settings-audit-heading = Audit-Protokoll
settings-audit-hint = Manipulationssicheres Protokoll im Anhängemodus für jedes Auftrags- und Dateiereignis. Formate umfassen CSV, JSON-Lines, RFC 5424 Syslog, ArcSight CEF und QRadar LEEF.
settings-audit-enable = Audit-Protokollierung aktivieren
settings-audit-format = Protokollformat
settings-audit-format-json-lines = JSON-Lines (empfohlener Standard)
settings-audit-format-csv = CSV (tabellenkalkulationsfreundlich)
settings-audit-format-syslog = Syslog (RFC 5424)
settings-audit-format-cef = CEF (ArcSight)
settings-audit-format-leef = LEEF 2.0 (IBM QRadar)
settings-audit-file-path = Pfad der Protokolldatei
settings-audit-file-path-placeholder = z. B. C:\ProgramData\CopyThat\audit.log
settings-audit-max-size = Rotieren nach (Bytes, 0 = nie)
settings-audit-worm = WORM-Modus aktivieren (write-once-read-many)
settings-audit-worm-hint = Wendet nach jedem Erstellen oder Rotieren das Nur-Anhängen-Flag der Plattform an (Linux chattr +a, macOS chflags uappnd, Windows-Schreibschutzattribut). Selbst ein Administrator muss das Flag ausdrücklich aufheben, um das Protokoll zu kürzen.
settings-audit-test-write = Testschreibung
settings-audit-verify-chain = Kette prüfen
toast-audit-test-write-ok = Testschreibung des Audit-Protokolls erfolgreich
toast-audit-verify-ok = Audit-Kette als unversehrt bestätigt
toast-audit-verify-failed = Prüfung der Audit-Kette meldete Abweichungen

# Phase 35 — destination encryption via age (X25519 / passphrase /
# SSH) + on-the-fly zstd compression with a per-extension deny
# heuristic. Surfaced through Settings → Transfer → Encryption +
# Compression; engine short-circuits to the crypt pipeline when
# either stage is active.
settings-crypt-heading = Verschlüsselung & Komprimierung
settings-crypt-hint = Wandelt Dateiinhalte um, bevor sie am Ziel ankommen. Die Verschlüsselung nutzt das age-Format; die Komprimierung nutzt zstd und kann bereits komprimierte Medien anhand der Erweiterung überspringen.
settings-crypt-encryption-mode = Verschlüsselung
settings-crypt-encryption-off = Aus
settings-crypt-encryption-passphrase = Passphrase (Abfrage beim Kopierstart)
settings-crypt-encryption-recipients = Empfängerschlüssel aus Datei
settings-crypt-encryption-hint = Passphrasen werden nur für die Dauer des Kopiervorgangs im Speicher gehalten. Empfängerdateien listen einen age1…- oder ssh-Public-Key pro Zeile auf.
settings-crypt-recipients-file = Pfad der Empfängerdatei
settings-crypt-recipients-file-placeholder = z. B. C:\Users\me\recipients.txt
settings-crypt-compression-mode = Komprimierung
settings-crypt-compression-off = Aus
settings-crypt-compression-always = Immer
settings-crypt-compression-smart = Intelligent (bereits komprimierte Medien überspringen)
settings-crypt-compression-hint = Der intelligente Modus überspringt jpg, mp4, zip, 7z und ähnliche Formate, die von zstd nicht profitieren. Der Immer-Modus komprimiert jede Datei mit der gewählten Stufe.
settings-crypt-compression-level = zstd-Stufe (1-22)
settings-crypt-compression-level-hint = Niedrigere Zahlen sind schneller; höhere Zahlen komprimieren stärker. Stufe 3 entspricht dem CLI-Standard von zstd.
compress-footer-savings = 💾 { $original } → { $compressed } ({ $percent } % gespart)
compress-savings-toast = { $percent } % komprimiert ({ $bytes } gespart)
crypt-toast-recipients-loaded = { $count } Verschlüsselungsempfänger geladen
crypt-toast-recipients-error = Empfänger konnten nicht geladen werden: { $reason }
crypt-toast-passphrase-required = Die Verschlüsselung benötigt eine Passphrase, bevor der Kopiervorgang startet
crypt-toast-passphrase-set = Verschlüsselungspassphrase erfasst
crypt-footer-encrypted-badge = 🔒 Verschlüsselt (age)
crypt-footer-compressed-badge = 📦 Komprimiert (zstd)

# Phase 36 — copythat CLI surface. Documented exit codes + the
# user-facing strings for plan/apply/verify/config. Help text stays
# in English (engineering accessibility) but error / info / status
# strings are localized so a localized desktop installation surfaces
# CLI errors in the same language as the GUI.
cli-help-tagline = Copy That CLI — bytegenaues Kopieren, Synchronisieren, Prüfen und Auditieren von Dateien für CI/CD-Pipelines.
cli-help-exit-codes = Exit-Codes: 0 Erfolg, 1 Fehler, 2 ausstehend, 3 Konflikt, 4 Prüfung fehlgeschlagen, 5 Netz, 6 Berechtigung, 7 Datenträger voll, 8 Abbruch, 9 Konfiguration.
cli-error-bad-args = copy/move erfordert mindestens eine Quelle und ein Ziel
cli-error-unknown-algo = Unbekannter Prüfalgorithmus: { $algo }
cli-error-missing-spec = --spec ist für plan/apply erforderlich
cli-error-spec-parse = Jobspec { $path } konnte nicht geparst werden: { $reason }
cli-error-spec-empty-sources = Quellliste der Jobspec ist leer
cli-info-shape-recorded = Bandbreitenform "{ $rate }" aufgezeichnet; die Durchsetzung erfolgt über copythat-shape
cli-info-stub-deferred = { $command } ist für die Folgeverdrahtung in Phase 36 vorgemerkt
cli-plan-summary = Plan: { $actions } Aktion(en), { $bytes } Byte(s); { $already_done } bereits vorhanden
cli-plan-pending = Plan meldet ausstehende Aktionen; mit `apply` erneut ausführen, um sie auszuführen
cli-plan-already-done = Plan meldet, dass nichts zu tun ist (idempotent)
cli-apply-success = Apply ohne Fehler abgeschlossen
cli-apply-failed = Apply mit einem oder mehreren Fehlern abgeschlossen
cli-verify-ok = Prüfung ok: { $algo } { $digest }
cli-verify-failed = Prüfung FEHLGESCHLAGEN für { $path } ({ $algo })
cli-config-set = { $key } = { $value } gesetzt
cli-config-reset = { $key } auf Standard zurückgesetzt
cli-config-unknown-key = Unbekannter Konfigurationsschlüssel: { $key }
cli-completions-emitted = Shell-Vervollständigungen für { $shell } nach stdout ausgegeben

# Phase 37 — desktop-side mobile companion. Settings → Mobile panel
# strings + the SAS-confirmation modal + push-notification toasts.
# The actual mobile UI lives in the Phase 37 follow-up Tauri Mobile
# target; the phone displays its own localized SAS prompt.
settings-mobile-heading = Mobiler Begleiter
settings-mobile-hint = Koppele ein iPhone oder Android-Telefon, um den Verlauf zu durchsuchen, gespeicherte Profile und Phase-36-Jobspecs zu starten und Abschlussbenachrichtigungen zu erhalten.
settings-mobile-pair-toggle = Neue Kopplungen zulassen
settings-mobile-pair-active = Kopplungsserver aktiv — scanne den QR-Code mit der Copy That-Mobil-App
settings-mobile-pair-button = Kopplung starten
settings-mobile-revoke-button = Widerrufen
settings-mobile-no-pairings = Noch keine gekoppelten Geräte
settings-mobile-pair-port = Bindungsport (0 = einen freien wählen)
pair-sas-prompt = Beide Bildschirme sollten dieselben vier Emojis zeigen. Tippe auf Übereinstimmung, wenn sie übereinstimmen.
pair-sas-confirm = Übereinstimmung
pair-sas-reject = Keine Übereinstimmung — abbrechen
pair-toast-success = Mit { $device } gekoppelt
pair-toast-failed = Kopplung fehlgeschlagen: { $reason }
push-toast-sent = Push an { $device } gesendet
push-toast-failed = Push an { $device } fehlgeschlagen: { $reason }

# Phase 38 — aggregate destination dedup + reflink fallback ladder.
# Settings → Transfer → Dedup panel + per-job-row badges (⚡
# Reflinked / 🔗 Hardlinked / 🧩 Chunk-shared / 📋 Copied) +
# pre-pass dedup-scan modal.
settings-dedup-heading = Ziel-Deduplizierung
settings-dedup-hint = Wenn Quelle und Ziel sich ein Laufwerk teilen, kann Copy That Dateien auf Dateisystemebene klonen, statt Bytes zu kopieren. Reflink ist sofort + sicher; Hardlink ist schneller, aber beide Namen teilen sich den Zustand.
settings-dedup-mode-auto = Automatische Stufenfolge (Reflink → Hardlink → Chunk → Kopie)
settings-dedup-mode-reflink-only = Nur Reflink
settings-dedup-mode-hardlink-aggressive = Aggressiv (Reflink + Hardlink auch bei beschreibbaren Dateien)
settings-dedup-mode-off = Deaktiviert (immer Byte-Kopie)
settings-dedup-hardlink-policy = Hardlink-Richtlinie
settings-dedup-prescan = Zielbaum vorab auf doppelte Inhalte scannen
dedup-badge-reflinked = ⚡ Reflink
dedup-badge-hardlinked = 🔗 Hardlink
dedup-badge-chunk-shared = 🧩 Chunk-geteilt
dedup-badge-copied = 📋 Kopiert
phase42-paranoid-verify-label = Paranoide Prüfung
phase42-paranoid-verify-hint = Verwirft die zwischengespeicherten Seiten des Ziels und liest erneut vom Datenträger, um Lügen des Schreibcaches und stille Beschädigungen zu erkennen. Etwa 50 % langsamer als die Standardprüfung; standardmäßig aus.
phase42-sharing-violation-retries-label = Wiederholungsversuche bei gesperrten Quelldateien
phase42-sharing-violation-retries-hint = Wie oft wiederholt wird, wenn ein anderer Prozess die Quelldatei mit einer exklusiven Sperre offen hält. Die Wartezeit verdoppelt sich bei jedem Versuch (standardmäßig 50 ms / 100 ms / 200 ms). Standard 3, entsprechend Robocopy /R:3.
phase42-cloud-placeholder-warning = { $name } ist eine reine Cloud-OneDrive-Datei. Beim Kopieren wird ein Download ausgelöst — bis zu { $size } über deine Netzwerkverbindung.
phase42-defender-exclusion-hint = Für maximalen Kopierdurchsatz füge den Zielordner vor Massenübertragungen den Microsoft-Defender-Ausnahmen hinzu. Siehe docs/PERFORMANCE_TUNING.md.

# Phase 39 — Browser-accessible recovery UI. Settings → Advanced
# exposes these strings; the recovery server itself renders askama
# templates that always read English — these eight keys are the
# Settings prose the user actually reads in their preferred locale.
settings-recovery-heading = Web-UI zur Wiederherstellung
settings-recovery-enable = Web-UI zur Wiederherstellung aktivieren
settings-recovery-bind-address = Bindungsadresse
settings-recovery-port = Port (0 = einen freien wählen)
settings-recovery-show-url = URL & Token anzeigen
settings-recovery-rotate-token = Token rotieren
settings-recovery-allow-non-loopback = Nicht-Loopback-Bindung zulassen
settings-recovery-non-loopback-warning = WARNUNG: Das Aktivieren einer Nicht-Loopback-Bindung stellt die Wiederherstellungs-UI deinem lokalen Netzwerk zur Verfügung. Jeder, der das Token erfährt, kann deinen Dateiverlauf durchsuchen und Dateien herunterladen. Stelle ihr TLS oder einen Reverse-Proxy voran, falls das LAN nicht vertrauenswürdig ist.

# Phase 40 — SMB compression negotiation + cloud-VM offload helper.
# 6 SMB keys (header badge + Settings prose) + 6 cloud-offload keys
# (Remotes tab wizard for cross-cloud copy templates).
smb-compress-badge = 🗜 SMB-Komprimierung: { $algo }
smb-compress-badge-tooltip = Der Netzwerkverkehr zu diesem Ziel wird während der Übertragung komprimiert (SMB 3.1.1).
smb-compress-toast-saved = { $bytes } über das Netzwerk gespart
smb-compress-algo-unknown = unbekannter Algorithmus
settings-smb-compress-heading = SMB-Netzwerkkomprimierung
settings-smb-compress-hint = Verhandelt automatisch SMB-3.1.1-Verkehrskomprimierung bei UNC-Zielen. Kostenloser Vorteil bei langsamen Verbindungen; bei lokalen Zielen ignoriert.
cloud-offload-heading = Cloud-VM-Auslagerungshelfer
cloud-offload-hint = Beim direkten Kopieren zwischen zwei Clouds wird eine Bereitstellungsvorlage erzeugt, die den Kopiervorgang von einer winzigen kurzlebigen VM in der Cloud ausführt — die Bytes berühren nie das Netzwerk deines Laptops.
cloud-offload-render-button = Vorlage erzeugen
cloud-offload-copy-clipboard = In die Zwischenablage kopieren
cloud-offload-template-format = Vorlagenformat
cloud-offload-self-destruct-warning = Die VM fährt nach { $minutes } Minuten automatisch herunter — bestätige IAM-Rolle + Region vor der Bereitstellung.

# Phase 41 — animated before/after tree-diff preview. The `Preview
# changes` modal renders the rolled-up plan before the engine starts
# work; 14 keys cover the title, the summary header / counts, the
# row-reason labels, and the two action buttons.
preview-modal-title = Änderungen in der Vorschau
preview-summary-header = Was geschehen wird
preview-category-additions = { $count } Hinzufügungen
preview-category-replacements = { $count } Ersetzungen
preview-category-skips = { $count } übersprungen
preview-category-conflicts = { $count } Konflikte
preview-category-unchanged = { $count } unverändert
preview-bytes-to-transfer = { $bytes } zu übertragen
preview-reason-source-newer = Quelle ist neuer
preview-reason-dest-newer = Ziel ist neuer — wird übersprungen
preview-reason-content-different = Inhalt unterscheidet sich
preview-reason-identical = Identisch mit der Quelle
preview-button-run = Plan ausführen
preview-button-reduce = Meinen Plan reduzieren…

# Phase 42 — perceptual-hash visual-similarity dedup. Eight keys cover
# the pre-copy "looks visually identical" warning + the Settings panel.
perceptual-warn-title = Sieht visuell identisch aus
perceptual-warn-body = { $name } am Ziel scheint mit dem Quellbild übereinzustimmen. Trotzdem weiterkopieren?
perceptual-warn-keep-both = Beide behalten
perceptual-warn-skip = Diese Datei überspringen
perceptual-warn-overwrite = Trotzdem überschreiben
perceptual-settings-heading = Deduplizierung nach visueller Ähnlichkeit
perceptual-settings-hint = Erkennt visuell identische Bilder am Ziel, bevor sie überschrieben werden. Der Hash ist perzeptuell (erkennt dasselbe Bild, das in einem anderen Format neu gespeichert wurde), nicht bytegenau.
perceptual-settings-threshold-label = Warnschwelle (niedriger = strengere Übereinstimmung)

# Phase 42 Part B — per-file rolling versions (Time Machine for any
# destination). 8 keys cover the version-list panel + retention picker.
version-list-heading = Vorherige Versionen
version-list-empty = Keine früheren Versionen dieser Datei
version-list-restore = Diese Version wiederherstellen
version-retention-heading = Vorherige Versionen beim Überschreiben behalten
version-retention-none = Jede Version für immer behalten
version-retention-last-n = Letzte { $n } Versionen behalten
version-retention-older-than-days = Versionen löschen, die älter sind als { $days } Tage
version-retention-gfs = Stündlich { $h } · täglich { $d } · wöchentlich { $w } · monatlich { $m }

# Phase 43 — forensic chain-of-custody manifests + BLAKE3 verified
# streaming. 14 keys cover the Settings → Provenance panel
# (heading, hint, toggles, TSA URL, signing-key management) plus
# the post-job manifest notification + the verify command's two
# headline result lines.
provenance-settings-heading = Forensische Lückenlose-Beweiskette
provenance-settings-hint = Signiere jeden Kopierauftrag mit einem BLAKE3 + ed25519-Manifest. Prüfer können den Zielbaum später erneut hashen und nachweisen, dass sich seit dem Kopiervorgang kein Byte geändert hat.
provenance-settings-enable-default = Jeden neuen Auftrag standardmäßig signieren
provenance-settings-show-after-job = Manifest nach jedem abgeschlossenen Auftrag anzeigen
provenance-settings-tsa-url-label = Standard-URL der RFC-3161-Zeitstempelstelle
provenance-settings-tsa-url-hint = Optional. Wenn gesetzt, tragen Manifeste einen kostenlosen TSA-Zeitstempel, der nachweist, dass die Bytes zu diesem Zeitpunkt existierten. Leer lassen, um zu überspringen.
provenance-settings-keys-heading = Signaturschlüssel
provenance-settings-keys-generate = Neuen Schlüssel erzeugen
provenance-settings-keys-import = Schlüssel importieren…
provenance-settings-keys-export = Öffentlichen Schlüssel exportieren…
provenance-job-completed-title = Provenienz-Manifest gespeichert
provenance-job-completed-body = { $count } Dateien signiert → { $path }
provenance-verify-clean = Manifest gültig für { $count } Dateien; Signatur { $sig }; Merkle-Wurzel OK.
provenance-verify-tampered = Manifest UNGÜLTIG — { $tampered } manipuliert, { $missing } fehlend.
# Phase 43 post-review hardening — toast text for the Settings →
# Provenance buttons whose Tauri IPC has not yet landed.
provenance-action-staged = Phase 43 — die IPC-Verdrahtung für diese Aktion kommt in einem Folge-Commit.

# Phase 44 — SSD-aware whole-drive sanitize (NVMe Sanitize / OPAL
# Crypto Erase) + the per-file shred refusal on copy-on-write
# filesystems. 16 keys cover the new "Drive sanitize" Settings
# subsection and the localized error messages.
sanitize-heading = Sichere Bereinigung des gesamten Laufwerks
sanitize-hint = NVMe Sanitize, OPAL Crypto Erase und ATA Secure Erase löschen ein Flash-Laufwerk in Millisekunden auf Firmware-Ebene. Das Überschreiben einzelner Dateien ist auf Flash bedeutungslos — mehrfaches Schreddern verbraucht nur NAND. Nutze dies für eine echte Bereinigung.
sanitize-pick-device = Zu bereinigendes Laufwerk auswählen
sanitize-mode-label = Bereinigungsmethode
sanitize-mode-nvme-format = NVMe Format (mit Secure Erase)
sanitize-mode-nvme-sanitize-block = NVMe Sanitize — Block Erase (langsam, jede Zelle)
sanitize-mode-nvme-sanitize-crypto = NVMe Sanitize — Crypto Erase (sofort)
sanitize-mode-ata-secure-erase = ATA Secure Erase (ältere SATA-SSDs)
sanitize-mode-opal-crypto-erase = TCG OPAL Crypto Erase (selbstverschlüsselnde Laufwerke)
sanitize-mode-apfs-crypto-erase = APFS Crypto Erase (FileVault-Schlüssel rotieren, nur macOS)
sanitize-confirm-1 = Dies zerstört JEDES Byte auf { $device }. Es gibt kein Rückgängig.
sanitize-confirm-2 = Ich verstehe, dass alle Partitionen, alle Dateien und alle Snapshots auf { $device } dauerhaft unlesbar werden.
sanitize-confirm-3 = Gib den Modellnamen des Laufwerks ein, um fortzufahren: { $model }
sanitize-running = { $device } wird bereinigt ({ $mode }) — dies kann von Millisekunden (Crypto Erase) bis zu mehreren zehn Minuten (Block Erase) dauern. Nicht ausschalten.
sanitize-completed = Bereinigung abgeschlossen — { $device } ist jetzt leer.
ssd-honest-shred-meaningless = Das Schreddern einzelner Dateien auf einem Copy-on-Write-Dateisystem (Btrfs / ZFS / APFS) kann die zugrunde liegenden Blöcke nicht erreichen. Nutze stattdessen die Bereinigung des gesamten Laufwerks plus Rotation des Schlüssels der vollständigen Festplattenverschlüsselung.
ssd-honest-advisory = Diese Datei liegt auf Flash. Das Überschreiben einzelner Dateien verursacht NAND-Verschleiß und garantiert NICHT, dass die ursprünglichen Zellen unwiederherstellbar sind. Für sensible Daten bereinige das gesamte Laufwerk.

# Phase 44.1f post-review — placeholder toast for SanitizeTab
# buttons whose Tauri IPC has not yet landed.
sanitize-action-staged = Phase 44.1 — die IPC-Verdrahtung für diese Aktion kommt in einem Folge-Commit.

# Phase 45.3 — named-queue tab strip (Subfeature A). Tabs surface
# once the QueueRegistry holds at least one queue; the synthesised
# default tab keeps legacy single-queue jobs reachable.
queue-tab-default = Standard
queue-tab-empty-state = Auftragswarteschlangen
queue-badge-tooltip = Ausstehende und laufende Aufträge in dieser Warteschlange

# Phase 45.4 — drag-progress-merge (Subfeature B). Drag a queue tab
# onto another to merge their job lists. The default tab is neither
# draggable nor a drop target; only registry queues participate.
queue-drag-hint = Auf eine andere Warteschlange ziehen, um zusammenzuführen
queue-merge-confirm = Zum Zusammenführen ablegen
queue-merge-toast = Warteschlangen zusammengeführt

# Phase 45.5 — F2-queue UX (Subfeature C). F2 toggles
# `auto_enqueue_next` so every fresh enqueue piles into the running
# queue rather than spawning a parallel one. Status pill renders in
# the Footer; pulsing dot renders on the running tab.
queue-f2-active-hint = F2-Modus: Jede neue Einreihung landet in dieser Warteschlange
queue-f2-toggled-on = F2-Warteschlangenmodus EIN — neue Einreihungen schließen sich der laufenden Warteschlange an
queue-f2-toggled-off = F2-Warteschlangenmodus AUS — neue Einreihungen erzeugen parallele Warteschlangen
queue-f2-status-bar = F2-Warteschlangenmodus: EIN

# Phase 45.6 — tray destination targets (Subfeature D). Pinned
# destinations appear in the OS tray menu; clicking one arms it as
# the active drop target so the next file drop bypasses the
# DropStagingDialog. Settings → General hosts the list editor.
tray-target-section-title = Infobereich-Ziele
tray-target-section-hint = Angeheftete Ziele erscheinen im Infobereichsmenü. Klicke eines an, um es als nächstes Ablageziel scharfzuschalten.
tray-target-empty = Noch keine Infobereich-Ziele angeheftet.
tray-target-remove = Entfernen
tray-target-add-label = Bezeichnung
tray-target-add-path = Pfad oder Backend-URI
tray-target-add = Hinzufügen
tray-target-armed-toast = Lege deine nächste Datei ab, um sie an { $label } zu senden
tray-target-active-pill = → { $label }

# Phase 45.7 follow-up — pinned-destination validation errors. The
# `queue_pin_destination` IPC returns these Fluent keys verbatim
# when input fails the IPC-boundary checks; the toast layer renders
# them via `t(...)` (Toast.svelte detects kebab-case-lowercase and
# routes through the locale table). Phase 17e's `err-path-escape` /
# `err-destination-empty` cover the path-traversal + empty cases for
# `queue_route_job`; the keys below are pin-specific.
err-pinned-destination-label-empty = Die Bezeichnung des Infobereich-Ziels darf nicht leer sein.
err-pinned-destination-path-empty = Der Pfad des Infobereich-Ziels darf nicht leer sein.
err-pinned-destination-label-too-long = Die Bezeichnung des Infobereich-Ziels ist zu lang (max. 64 Zeichen).
err-pinned-destination-path-too-long = Der Pfad des Infobereich-Ziels ist zu lang (max. 1024 Zeichen).
err-pinned-destination-label-invalid = Die Bezeichnung des Infobereich-Ziels enthält nicht erlaubte Zeichen (Zeilenumbruch, Wagenrücklauf oder NUL).
err-pinned-destination-path-invalid = Der Pfad des Infobereich-Ziels enthält nicht erlaubte Zeichen (Zeilenumbruch, Wagenrücklauf oder NUL).
err-pinned-destination-too-many = Du hast das Limit von 50 Infobereich-Zielen erreicht. Entferne eines, um ein weiteres hinzuzufügen.

# Phase 46.6 — Settings → Plugins tab. The IPC layer in
# `apps/copythat-ui/src-tauri/src/plugin_commands.rs` enumerates the
# per-user plugin store under `<config_dir>/plugins/`; the
# `PluginsTab.svelte` component renders these strings against the
# returned manifest + grant state.
settings-tab-plugins = Plugins
plugin-heading = Plugins
plugin-hint = In der Sandbox laufende WASM-Plugins erweitern Copy That um eigene Hooks. Jedes Plugin läuft unter CPU- und Speicherlimits pro Aufruf und sieht nur die Host-Fähigkeiten, die du ihm gewährst.
plugin-list-empty = Noch keine Plugins installiert.
plugin-enabled = Aktiviert
plugin-disabled = Deaktiviert
plugin-hooks = Hooks
plugin-capabilities = Fähigkeiten
plugin-no-capabilities = (keine)
plugin-directory = Speicherort
plugin-install-from-file = Aus Datei installieren…
plugin-install-from-url = Aus URL installieren…
plugin-url-wasm = WASM-URL
plugin-url-manifest = Manifest-URL
plugin-url-hash = BLAKE3-Hash
plugin-url-preview = Vorschau
plugin-url-confirm = Installation bestätigen

# Phase 31b — power-policy settings (Power tab).
settings-tab-power = Energie
settings-power-hint = Kopien je nach Energiezustand drosseln oder pausieren: Akku, getaktetes/Mobilfunknetz, Präsentation/Vollbild oder thermische CPU-Drosselung.
settings-power-enabled = Energieabhängige Drosselung aktivieren
settings-power-battery = Im Akkubetrieb
settings-power-metered = In getaktetem Netzwerk
settings-power-cellular = Im Mobilfunknetz
settings-power-presentation = Während einer Präsentation
settings-power-fullscreen = Im Vollbild
settings-power-thermal = Bei thermischer Drosselung
settings-power-continue = Fortsetzen
settings-power-pause = Pausieren
err-server-not-implemented = Der Servermodus ist noch nicht verfügbar.
err-webhook-not-implemented = Webhook-Zustellung ist noch nicht verfügbar.

# Phase 47 — "why is this slow?" diagnostics (bottleneck badge + tooltip).
bottleneck-source-io = Quelle I/O
bottleneck-dest-io = Ziel I/O
bottleneck-network = Netzwerk
bottleneck-antivirus = Antivirus
bottleneck-cpu = CPU
bottleneck-thermal = Thermisch
bottleneck-unknown = Unbekannt
diag-aria = Engpass: { $cause }
diag-tooltip = Begrenzt durch { $cause } · { $rate }
diag-spark-aria = Durchsatz der letzten Minute
diag-keeping-up = Hält Schritt
diag-label = Diagnose

# Phase 48 — server mode + observability (Settings → Server).
settings-tab-server = Server
server-hint = Führen Sie Copy That als headless Dateiserver aus. Wählen Sie die bereitzustellenden Protokolle, legen Sie Adresse und freizugebenden Ordner fest und verlangen Sie optional eine Authentifizierung.
server-protocols = Protokolle
server-bind-addr = Bind-Adresse
server-root = Freigegebener Ordner
server-readonly = Schreibgeschützt (Uploads und Löschungen ablehnen)
server-auth-mode = Authentifizierung
server-auth-none = Keine
server-auth-bearer = Bearer-Token
server-auth-basic = Basic (Benutzername + Passwort)
server-auth-token = Token
server-auth-user = Benutzername
server-auth-password = Passwort
otel-endpoint = OpenTelemetry-Endpunkt
webhook-section = Webhooks
webhook-url = Webhook-URL
webhook-add = Webhook hinzufügen
webhook-remove = Entfernen
webhook-empty = Keine Webhooks konfiguriert.
webhook-pushover-token = Pushover-Token
webhook-pushover-user = Pushover-Benutzer
server-start = Server starten
server-stop = Server stoppen
server-status-running = Läuft auf { $addr }
server-status-stopped = Gestoppt
server-metrics-url = Metriken
err-server-no-protocols = Wählen Sie mindestens ein Protokoll, bevor Sie den Server starten.
err-server-bind = Die Serveradresse konnte nicht gebunden werden. Sie ist möglicherweise bereits belegt.
