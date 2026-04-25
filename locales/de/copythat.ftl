app-name = Copy That v1.25.0
# MT
window-title = Copy That v1.25.0
# MT
shred-ssd-advisory = Warnung: Das Ziel liegt auf einer SSD. Mehrfaches Überschreiben bereinigt den Flash-Speicher nicht zuverlässig, da Wear-Leveling und Overprovisioning die Daten aus der logischen Blockadresse herausschieben. Für Solid-State-Medien sind ATA SECURE ERASE, NVMe Format mit Secure Erase oder eine vollverschlüsselte Festplatte mit anschließend verworfenem Schlüssel vorzuziehen.

# MT
state-idle = Leerlauf
# MT
state-copying = Kopieren
# MT
state-verifying = Prüfen
# MT
state-paused = Pausiert
# MT
state-error = Fehler

# MT
state-pending = In Warteschlange
# MT
state-running = Läuft
# MT
state-cancelled = Abgebrochen
# MT
state-succeeded = Fertig
# MT
state-failed = Fehlgeschlagen

# MT
action-pause = Pausieren
# MT
action-resume = Fortsetzen
# MT
action-cancel = Abbrechen
# MT
action-pause-all = Alle Aufgaben pausieren
# MT
action-resume-all = Alle Aufgaben fortsetzen
# MT
action-cancel-all = Alle Aufgaben abbrechen
# MT
action-close = Schließen
# MT
action-reveal = Im Ordner anzeigen

# MT
menu-pause = Pausieren
# MT
menu-resume = Fortsetzen
# MT
menu-cancel = Abbrechen
# MT
menu-remove = Aus Warteschlange entfernen
# MT
menu-reveal-source = Quelle im Ordner anzeigen
# MT
menu-reveal-destination = Ziel im Ordner anzeigen

# MT
header-eta-label = Geschätzte Restzeit
# MT
header-toolbar-label = Globale Steuerung

# MT
footer-queued = aktive Aufgaben
# MT
footer-total-bytes = in Bearbeitung
# MT
footer-errors = Fehler
# MT
footer-history = Verlauf

# MT
empty-title = Dateien oder Ordner zum Kopieren ablegen
# MT
empty-hint = Ziehen Sie Elemente auf das Fenster. Wir fragen nach einem Ziel und erstellen dann eine Aufgabe pro Quelle.
# MT
empty-region-label = Aufgabenliste

# MT
details-drawer-label = Aufgabendetails
# MT
details-source = Quelle
# MT
details-destination = Ziel
# MT
details-state = Zustand
# MT
details-bytes = Bytes
# MT
details-files = Dateien
# MT
details-speed = Geschwindigkeit
# MT
details-eta = Restzeit
# MT
details-error = Fehler

# MT
drop-dialog-title = Abgelegte Elemente übertragen
# MT
drop-dialog-subtitle = { $count } Element(e) bereit zur Übertragung. Wählen Sie einen Zielordner, um zu beginnen.
# MT
drop-dialog-mode = Vorgang
# MT
drop-dialog-copy = Kopieren
# MT
drop-dialog-move = Verschieben
# MT
drop-dialog-pick-destination = Ziel auswählen
# MT
drop-dialog-change-destination = Ziel ändern
# MT
drop-dialog-start-copy = Kopieren starten
# MT
drop-dialog-start-move = Verschieben starten

# MT
eta-calculating = wird berechnet…
# MT
eta-unknown = unbekannt

# MT
toast-job-done = Übertragung abgeschlossen
# MT
toast-copy-queued = Kopieren in Warteschlange
# MT
toast-move-queued = Verschieben in Warteschlange
# MT
toast-error-resolved = Fehler behoben
# MT
toast-collision-resolved = Konflikt gelöst
# MT
toast-elevated-unavailable = Wiederholung mit erhöhten Rechten kommt in Phase 17 — noch nicht verfügbar
toast-clipboard-files-detected = Dateien in der Zwischenablage — Tastenkürzel drücken, um über Copy That zu kopieren
toast-clipboard-no-files = Zwischenablage enthält keine Dateien zum Einfügen
# MT
toast-error-log-exported = Fehlerprotokoll exportiert

# MT — Error modal
error-modal-title = Eine Übertragung ist fehlgeschlagen
# MT
error-modal-retry = Wiederholen
# MT
error-modal-retry-elevated = Mit erhöhten Rechten wiederholen
# MT
error-modal-skip = Überspringen
# MT
error-modal-skip-all-kind = Alle Fehler dieser Art überspringen
# MT
error-modal-abort = Alle abbrechen
# MT
error-modal-path-label = Pfad
# MT
error-modal-code-label = Code
error-drawer-pending-count = Weitere Fehler warten
error-drawer-toggle = Einklappen oder ausklappen

# MT — Error-kind labels
err-not-found = Datei nicht gefunden
# MT
err-permission-denied = Zugriff verweigert
# MT
err-disk-full = Ziellaufwerk ist voll
# MT
err-interrupted = Vorgang unterbrochen
# MT
err-verify-failed = Prüfung nach Kopie fehlgeschlagen
# MT
err-path-escape = Pfad abgelehnt — enthält übergeordnete Verzeichnissegmente (..) oder ungültige Bytes
# MT
err-path-invalid-encoding = Path rejected — string contains invalid UTF-8 / replacement characters
# MT
err-io-other = Unbekannter E/A-Fehler
err-sparseness-mismatch = Sparse-Layout konnte am Ziel nicht beibehalten werden  # MT

# MT — Collision modal
collision-modal-title = Datei existiert bereits
# MT
collision-modal-overwrite = Überschreiben
# MT
collision-modal-overwrite-if-newer = Überschreiben wenn neuer
# MT
collision-modal-skip = Überspringen
# MT
collision-modal-keep-both = Beide behalten
# MT
collision-modal-rename = Umbenennen…
# MT
collision-modal-apply-to-all = Auf alle anwenden
# MT
collision-modal-source = Quelle
# MT
collision-modal-destination = Ziel
# MT
collision-modal-size = Größe
# MT
collision-modal-modified = Geändert
# MT
collision-modal-hash-check = Schnelle Prüfsumme (SHA-256)
# MT
collision-modal-rename-placeholder = Neuer Dateiname
# MT
collision-modal-confirm-rename = Umbenennen

# MT — Error log drawer
error-log-title = Fehlerprotokoll
# MT
error-log-empty = Keine Fehler protokolliert
# MT
error-log-export-csv = CSV exportieren
# MT
error-log-export-txt = Text exportieren
# MT
error-log-clear = Protokoll leeren
# MT
error-log-col-time = Zeit
# MT
error-log-col-job = Aufgabe
# MT
error-log-col-path = Pfad
# MT
error-log-col-code = Code
# MT
error-log-col-message = Meldung
# MT
error-log-col-resolution = Lösung

# MT — History drawer (Phase 9)
history-title = Verlauf
# MT
history-empty = Noch keine Aufgaben aufgezeichnet
# MT
history-unavailable = Der Kopierverlauf ist nicht verfügbar. Die App konnte den SQLite-Speicher beim Start nicht öffnen.
# MT
history-filter-any = alle
# MT
history-filter-kind = Art
# MT
history-filter-status = Status
# MT
history-filter-text = Suchen
# MT
history-refresh = Aktualisieren
# MT
history-export-csv = CSV exportieren
# MT
history-purge-30 = Älter als 30 Tage löschen
# MT
history-rerun = Erneut ausführen
# MT
history-detail-open = Details
# MT
history-detail-title = Aufgabendetails
# MT
history-detail-empty = Keine Einträge aufgezeichnet
# MT
history-col-date = Datum
# MT
history-col-kind = Art
# MT
history-col-src = Quelle
# MT
history-col-dst = Ziel
# MT
history-col-files = Dateien
# MT
history-col-size = Größe
# MT
history-col-status = Status
# MT
history-col-duration = Dauer
# MT
history-col-error = Fehler

# MT
toast-history-exported = Verlauf exportiert
# MT
toast-history-rerun-queued = Erneute Ausführung in Warteschlange

# MT — Totals drawer (Phase 10)
footer-totals = Gesamt
# MT
totals-title = Gesamtwerte
# MT
totals-loading = Lade Gesamtwerte…
# MT
totals-card-bytes = Kopierte Bytes insgesamt
# MT
totals-card-files = Dateien
# MT
totals-card-jobs = Aufgaben
# MT
totals-card-avg-rate = Durchschnittlicher Durchsatz
# MT
totals-errors = Fehler
# MT
totals-spark-title = Letzte 30 Tage
# MT
totals-kinds-title = Nach Art
# MT
totals-saved-title = Eingesparte Zeit (geschätzt)
# MT
totals-saved-note = Geschätzt gegenüber einem Standard-Dateimanager-Kopiervorgang derselben Daten.
# MT
totals-reset = Statistik zurücksetzen
# MT
totals-reset-confirm = Dadurch werden alle gespeicherten Aufgaben und Einträge gelöscht. Fortfahren?
# MT
totals-reset-confirm-yes = Ja, zurücksetzen
# MT
toast-totals-reset = Statistik zurückgesetzt

# MT — Phase 11a additions
header-language-label = Sprache
# MT
header-language-title = Sprache ändern

# MT
kind-copy = Kopieren
# MT
kind-move = Verschieben
# MT
kind-delete = Löschen
# MT
kind-secure-delete = Sicher löschen

# MT
status-running = Läuft
# MT
status-succeeded = Erfolgreich
# MT
status-failed = Fehlgeschlagen
# MT
status-cancelled = Abgebrochen
# MT
status-ok = OK
# MT
status-skipped = Übersprungen

# MT
history-search-placeholder = /pfad
# MT
toast-history-purged = { $count } Aufgaben älter als 30 Tage gelöscht

# MT
err-source-required = Mindestens ein Quellpfad ist erforderlich.
# MT
err-destination-empty = Der Zielpfad ist leer.
# MT
err-source-empty = Der Quellpfad ist leer.

# MT
duration-lt-1s = < 1 s
# MT
duration-ms = { $ms } ms
# MT
duration-seconds = { $s } s
# MT
duration-minutes-seconds = { $m } min { $s } s
# MT
duration-hours-minutes = { $h } h { $m } min
# MT
duration-zero = 0 s

# MT
rate-unit-per-second = { $size }/s

# MT — Phase 11b Settings modal
settings-title = Einstellungen
# MT
settings-tab-general = Allgemein
# MT
settings-tab-appearance = Darstellung
# MT
settings-section-language = Sprache
# MT
settings-phase-12-hint = Weitere Einstellungen (Design, Übertragungs-Standards, Prüfalgorithmus, Profile) kommen in Phase 12.

# MT — Phase 12 Settings window
settings-loading = Einstellungen werden geladen…
# MT
settings-tab-transfer = Übertragung
# MT
settings-tab-shell = Shell
# MT
settings-tab-secure-delete = Sicher löschen
# MT
settings-tab-advanced = Erweitert
# MT
settings-tab-profiles = Profile

# MT
settings-section-theme = Design
# MT
settings-theme-auto = Automatisch
# MT
settings-theme-light = Hell
# MT
settings-theme-dark = Dunkel
# MT
settings-start-with-os = Beim Systemstart starten
# MT
settings-single-instance = Einzelne laufende Instanz
# MT
settings-minimize-to-tray = Beim Schließen in den Tray minimieren
settings-error-display-mode = Fehler-Dialogstil
settings-error-display-modal = Modal (blockiert die App)
settings-error-display-drawer = Seitenleiste (nicht blockierend)
settings-error-display-mode-hint = Das Modal hält die Warteschlange an, bis Sie entscheiden. Die Seitenleiste hält die Warteschlange am Laufen und lässt Sie Fehler in der Ecke abarbeiten.
settings-paste-shortcut = Dateien per globalem Tastenkürzel einfügen
settings-paste-shortcut-combo = Tastenkombination
settings-paste-shortcut-hint = Drücken Sie diese Kombination an beliebiger Stelle im System, um aus Explorer / Finder / Dateien kopierte Dateien über Copy That einzufügen. CmdOrCtrl entspricht Cmd auf macOS und Strg auf Windows / Linux.
settings-clipboard-watcher = Zwischenablage auf kopierte Dateien überwachen
settings-clipboard-watcher-hint = Zeigt eine Einblendung, wenn Datei-URLs in der Zwischenablage erscheinen, und weist darauf hin, dass Sie über Copy That einfügen können. Prüft alle 500 ms, solange aktiv.

# MT
settings-buffer-size = Puffergröße
# MT
settings-verify = Nach Kopie prüfen
# MT
settings-verify-off = Aus
# MT
settings-concurrency = Parallelität
# MT
settings-concurrency-auto = Automatisch
# MT
settings-reflink = Reflink / schnelle Pfade
# MT
settings-reflink-prefer = Bevorzugen
# MT
settings-reflink-avoid = Reflink vermeiden
# MT
settings-reflink-disabled = Immer Async-Engine verwenden
# MT
settings-fsync-on-close = Beim Schließen auf Festplatte synchronisieren (langsamer, sicherer)
# MT
settings-preserve-timestamps = Zeitstempel beibehalten
# MT
settings-preserve-permissions = Berechtigungen beibehalten
# MT
settings-preserve-acls = ACLs beibehalten (Phase 14)
settings-preserve-sparseness = Sparse-Dateien bewahren  # MT
settings-preserve-sparseness-hint = Bei Sparse-Dateien (VM-Disks, Datenbankdateien) werden nur die zugewiesenen Bereiche kopiert, sodass das Ziel dieselbe Größe auf dem Datenträger wie die Quelle behält.  # MT

# MT
settings-context-menu = Shell-Kontextmenü-Einträge aktivieren
# MT
settings-intercept-copy = Standard-Kopier-Handler abfangen (Windows)
# MT
settings-intercept-copy-hint = Wenn aktiv, wird Strg+C / Strg+V im Explorer über Copy That abgewickelt. Registrierung in Phase 14.
# MT
settings-notify-completion = Bei Auftragsende benachrichtigen

# MT
settings-shred-method = Standard-Shred-Methode
# MT
settings-shred-zero = Null (1 Durchgang)
# MT
settings-shred-random = Zufall (1 Durchgang)
# MT
settings-shred-dod3 = DoD 5220.22-M (3 Durchgänge)
# MT
settings-shred-dod7 = DoD 5220.22-M (7 Durchgänge)
# MT
settings-shred-gutmann = Gutmann (35 Durchgänge)
# MT
settings-shred-nist = NIST 800-88
# MT
settings-shred-confirm-twice = Vor dem Schreddern doppelte Bestätigung verlangen

# MT
settings-log-level = Log-Level
# MT
settings-log-off = Aus
# MT
settings-telemetry = Telemetrie
# MT
settings-telemetry-never = Niemals — keine Datenübertragung bei keinem Log-Level
# MT
settings-error-policy = Standard-Fehlerrichtlinie
# MT
settings-error-policy-ask = Fragen
# MT
settings-error-policy-skip = Überspringen
# MT
settings-error-policy-retry = Mit Backoff wiederholen
# MT
settings-error-policy-abort = Bei erstem Fehler abbrechen
# MT
settings-history-retention = Verlaufsaufbewahrung (Tage)
# MT
settings-history-retention-hint = 0 = für immer behalten. Jeder andere Wert löscht ältere Aufträge beim Start automatisch.
# MT
settings-database-path = Datenbankpfad
# MT
settings-database-path-default = (Standard — OS-Datenverzeichnis)
# MT
settings-reset-all = Auf Standards zurücksetzen
# MT
settings-reset-confirm = Alle Einstellungen auf Standards zurücksetzen? Profile bleiben unberührt.

# MT
settings-profiles-hint = Aktuelle Einstellungen unter einem Namen speichern; später laden, um ohne Anpassung einzelner Regler umzuschalten.
# MT
settings-profile-name-placeholder = Profilname
# MT
settings-profile-save = Speichern
# MT
settings-profile-import = Importieren…
# MT
settings-profile-load = Laden
# MT
settings-profile-export = Exportieren…
# MT
settings-profile-delete = Löschen
# MT
settings-profile-empty = Noch keine Profile gespeichert.
# MT
settings-profile-import-prompt = Name für das importierte Profil:

# MT
toast-settings-reset = Einstellungen zurückgesetzt
# MT
toast-profile-saved = Profil gespeichert
# MT
toast-profile-loaded = Profil geladen
# MT
toast-profile-exported = Profil exportiert
# MT
toast-profile-imported = Profil importiert

# Phase 13d — activity feed + header picker buttons
action-add-files = Dateien hinzufügen
action-add-folders = Ordner hinzufügen
activity-title = Aktivität
activity-clear = Aktivitätsliste leeren
activity-empty = Noch keine Dateiaktivität.
activity-after-done = Nach Abschluss:
activity-keep-open = Anwendung offen lassen
activity-close-app = App schließen
activity-shutdown = PC herunterfahren
activity-logoff = Abmelden
activity-sleep = Energiesparmodus

# Phase 14 — preflight free-space dialog
preflight-block-title = Nicht genug Speicher am Ziel
preflight-warn-title = Wenig Speicher am Ziel
preflight-unknown-title = Freier Speicher nicht ermittelbar
preflight-unknown-body = Die Quelle ist zu groß, um schnell gemessen zu werden, oder das Ziellaufwerk antwortet nicht. Sie können fortfahren; die Schutzfunktion der Engine stoppt den Kopiervorgang sauber, falls der Platz ausgeht.
preflight-required = Benötigt
preflight-free = Frei
preflight-reserve = Reserve
preflight-shortfall = Fehlbetrag
preflight-continue = Trotzdem fortfahren
collision-modal-overwrite-older = Nur ältere überschreiben

# Phase 14e — subset picker
preflight-pick-subset = Auswählen, was kopiert wird…
subset-title = Zu kopierende Quellen auswählen
subset-subtitle = Die komplette Auswahl passt nicht auf das Ziel. Wähle aus, was kopiert werden soll; der Rest bleibt zurück.
subset-loading = Größen werden gemessen…
subset-too-large = zu groß zum Zählen
subset-budget = Verfügbar
subset-remaining = Verbleibend
subset-confirm = Auswahl kopieren
history-rerun-hint = Diesen Kopiervorgang erneut ausführen — scannt alle Dateien im Quellbaum
history-clear-all = Alles löschen
history-clear-all-confirm = Zum Bestätigen erneut klicken
history-clear-all-hint = Alle Verlaufszeilen löschen. Ein zweiter Klick bestätigt.
toast-history-cleared = Verlauf gelöscht ({ $count } Zeilen entfernt)

# Phase 15 — source-list ordering
drop-dialog-sort-label = Reihenfolge:
sort-custom = Benutzerdefiniert
sort-name-asc = Name A → Z (Dateien zuerst)
sort-name-desc = Name Z → A (Dateien zuerst)
sort-size-asc = Größe aufsteigend (Dateien zuerst)
sort-size-desc = Größe absteigend (Dateien zuerst)
sort-reorder = Neu anordnen
sort-move-top = Nach ganz oben
sort-move-up = Nach oben
sort-move-down = Nach unten
sort-move-bottom = Nach ganz unten
sort-name-asc-simple = Name A → Z
sort-name-desc-simple = Name Z → A
sort-size-asc-simple = Kleinste zuerst
sort-size-desc-simple = Größte zuerst
activity-sort-locked = Sortierung ist deaktiviert, während ein Kopiervorgang läuft. Pausiere ihn oder warte bis zum Ende, dann ändere die Reihenfolge.
drop-dialog-collision-label = Wenn eine Datei bereits existiert:
collision-policy-keep-both = Beide behalten (neue Kopie zu _2, _3 … umbenennen)
collision-policy-skip = Neue Kopie überspringen
collision-policy-overwrite = Vorhandene Datei überschreiben
collision-policy-overwrite-if-newer = Nur überschreiben, wenn neuer
collision-policy-prompt = Jedes Mal fragen
drop-dialog-busy-checking = Freier Speicher wird geprüft…
drop-dialog-busy-enumerating = Dateien werden gezählt…
drop-dialog-busy-starting = Kopiervorgang wird gestartet…
toast-enumeration-deferred = Der Quellbaum ist groß — Vorabliste wird übersprungen; Zeilen erscheinen, sobald die Engine sie verarbeitet.

# Phase 14a — enumeration-time filters
# MT
settings-tab-filters = Filter
# MT
settings-filters-hint = Überspringt Dateien bereits beim Aufzählen, bevor die Engine sie öffnet. Einschließen gilt nur für Dateien; Ausschließen bereinigt auch passende Ordner.
# MT
settings-filters-enabled = Filter für Baumkopien aktivieren
# MT
settings-filters-include-globs = Einschluss-Globs
# MT
settings-filters-include-globs-placeholder = **/*.txt
# MT
settings-filters-include-globs-hint = Ein Glob pro Zeile. Wenn vorhanden, muss eine Datei mindestens einem entsprechen. Ordner werden stets durchlaufen.
# MT
settings-filters-exclude-globs = Ausschluss-Globs
# MT
settings-filters-exclude-globs-placeholder = **/node_modules
# MT
settings-filters-exclude-globs-hint = Ein Glob pro Zeile. Treffer schneiden den gesamten Unterbaum für Ordner ab; passende Dateien werden übersprungen.
# MT
settings-filters-size-range = Dateigrößenbereich
# MT
settings-filters-min-size-bytes = Mindestgröße (Bytes, leer = keine Untergrenze)
# MT
settings-filters-max-size-bytes = Höchstgröße (Bytes, leer = keine Obergrenze)
# MT
settings-filters-date-range = Änderungszeitraum
# MT
settings-filters-min-mtime = Geändert ab
# MT
settings-filters-max-mtime = Geändert bis
# MT
settings-filters-attributes = Attribute
# MT
settings-filters-skip-hidden = Versteckte Dateien / Ordner überspringen
# MT
settings-filters-skip-system = Systemdateien überspringen (nur Windows)
# MT
settings-filters-skip-readonly = Schreibgeschützte Dateien überspringen

# Phase 15 — auto-update
# MT
settings-tab-updater = Updates
# MT
settings-updater-hint = Copy That prüft höchstens einmal am Tag auf signierte Updates. Updates werden beim nächsten Beenden der App installiert.
# MT
settings-updater-auto-check = Beim Start nach Updates suchen
# MT
settings-updater-channel = Veröffentlichungskanal
# MT
settings-updater-channel-stable = Stabil
# MT
settings-updater-channel-beta = Beta (Vorabversion)
# MT
settings-updater-last-check = Letzte Prüfung
# MT
settings-updater-last-never = Nie
# MT
settings-updater-check-now = Jetzt nach Updates suchen
# MT
settings-updater-checking = Wird geprüft…
# MT
settings-updater-available = Update verfügbar
# MT
settings-updater-up-to-date = Du verwendest die neueste Version.
# MT
settings-updater-dismiss = Diese Version überspringen
# MT
settings-updater-dismissed = Übersprungen
# MT
toast-update-available = Eine neuere Version ist verfügbar
# MT
toast-update-up-to-date = Du hast bereits die neueste Version

# Phase 19a — disk-backed file enumeration (TeraCopy-compatible scan DB)
# MT
scan-progress-title = Scan läuft…
# MT
scan-progress-stats = { $files } Dateien · { $bytes } bisher
# MT
scan-pause-button = Scan anhalten
# MT
scan-resume-button = Scan fortsetzen
# MT
scan-cancel-button = Scan abbrechen
# MT
scan-cancel-confirm = Scan abbrechen und Fortschritt verwerfen?
# MT
scan-db-header = Scandatenbank
# MT
scan-db-hint = Scandatenbank auf Festplatte für Aufträge mit Millionen von Dateien.
# MT
advanced-scan-hash-during = Prüfsummen während des Scans berechnen
# MT
advanced-scan-db-path = Speicherort der Scandatenbank
# MT
advanced-scan-retention-days = Abgeschlossene Scans automatisch löschen nach (Tage)
# MT
advanced-scan-max-keep = Maximale Anzahl behaltener Scandatenbanken

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
sparse-not-supported-title = Ziel füllt Sparse-Dateien aus  # MT
sparse-not-supported-body = { $dst_fs } unterstützt keine Sparse-Dateien. Löcher in der Quelle wurden als Nullen geschrieben, daher ist das Ziel auf dem Datenträger größer.  # MT
sparse-warning-densified = Sparse-Layout beibehalten: nur zugewiesene Bereiche wurden kopiert.  # MT
sparse-warning-mismatch = Sparse-Layout stimmt nicht überein — Ziel könnte größer als erwartet sein.  # MT

# Phase 24 — security-metadata preservation. MT-flagged drafts;
# the authoritative English source lives in locales/en/copythat.ftl.
settings-preserve-security-metadata = Sicherheitsmetadaten bewahren  # MT
settings-preserve-security-metadata-hint = Out-of-Band-Metadatenströme (NTFS-ADS / xattrs / POSIX-ACLs / SELinux-Kontexte / Linux-Datei-Capabilities / macOS-Ressourcen-Forks) bei jeder Kopie erfassen und erneut anwenden.  # MT
settings-preserve-motw = Mark-of-the-Web (Aus-dem-Internet-heruntergeladen-Flag) bewahren  # MT
settings-preserve-motw-hint = Sicherheitskritisch. SmartScreen und Office Protected View nutzen diesen Stream, um vor aus dem Internet heruntergeladenen Dateien zu warnen. Deaktivieren erlaubt einer heruntergeladenen Anwendung, ihre Herkunftsmarkierung beim Kopieren abzuwerfen und Betriebssystem-Schutzmaßnahmen zu umgehen.  # MT
settings-preserve-posix-acls = POSIX-ACLs und erweiterte Attribute bewahren  # MT
settings-preserve-posix-acls-hint = user.* / system.* / trusted.* xattrs und POSIX-Zugriffssteuerungslisten beim Kopieren übernehmen.  # MT
settings-preserve-selinux = SELinux-Kontexte bewahren  # MT
settings-preserve-selinux-hint = Das security.selinux-Label beim Kopieren übernehmen, damit unter MAC-Richtlinien laufende Daemons weiterhin auf die Datei zugreifen können.  # MT
settings-preserve-resource-forks = macOS-Ressourcen-Forks und Finder-Info bewahren  # MT
settings-preserve-resource-forks-hint = Den Legacy-Ressourcen-Fork und FinderInfo (Farbtags, Carbon-Metadaten) beim Kopieren übernehmen.  # MT
settings-appledouble-fallback = AppleDouble-Sidecar auf inkompatiblen Dateisystemen verwenden  # MT
meta-translated-to-appledouble = Fremde Metadaten im AppleDouble-Sidecar gespeichert (._{ $ext })  # MT

# Phase 25 — two-way sync with vector-clock conflict detection.
# MT-flagged drafts; the authoritative English source lives in
# locales/en/copythat.ftl.
footer-sync = Sync  # MT
sync-drawer-title = Zwei-Wege-Synchronisation  # MT
sync-drawer-hint = Halten Sie zwei Ordner ohne stille Überschreibungen synchron. Gleichzeitige Bearbeitungen erscheinen als auflösbare Konflikte.  # MT
sync-add-pair = Paar hinzufügen  # MT
sync-add-cancel = Abbrechen  # MT
sync-refresh = Aktualisieren  # MT
sync-add-save = Paar speichern  # MT
sync-add-saving = Speichern…  # MT
sync-add-missing-fields = Bezeichnung, linker Pfad und rechter Pfad sind alle erforderlich.  # MT
sync-remove-confirm = Dieses Sync-Paar entfernen? Die Zustandsdatenbank bleibt erhalten; die Ordner bleiben unberührt.  # MT
sync-field-label = Bezeichnung  # MT
sync-field-label-placeholder = z.B. Dokumente ↔ NAS  # MT
sync-field-left = Linker Ordner  # MT
sync-field-left-placeholder = Absoluten Pfad wählen oder einfügen  # MT
sync-field-right = Rechter Ordner  # MT
sync-field-right-placeholder = Absoluten Pfad wählen oder einfügen  # MT
sync-field-mode = Modus  # MT
sync-mode-two-way = Zwei-Wege  # MT
sync-mode-mirror-left-to-right = Spiegel (links → rechts)  # MT
sync-mode-mirror-right-to-left = Spiegel (rechts → links)  # MT
sync-mode-contribute-left-to-right = Beitragen (links → rechts, keine Löschungen)  # MT
sync-no-pairs = Noch keine Sync-Paare konfiguriert. Klicken Sie auf "Paar hinzufügen", um zu beginnen.  # MT
sync-loading = Konfigurierte Paare werden geladen…  # MT
sync-never-run = Nie ausgeführt  # MT
sync-running = Läuft  # MT
sync-run-now = Jetzt ausführen  # MT
sync-cancel = Abbrechen  # MT
sync-remove-pair = Entfernen  # MT
sync-view-conflicts = Konflikte ansehen ({ $count })  # MT
sync-conflicts-heading = Konflikte  # MT
sync-no-conflicts = Keine Konflikte aus dem letzten Lauf.  # MT
sync-winner = Sieger  # MT
sync-side-left-to-right = links  # MT
sync-side-right-to-left = rechts  # MT
sync-conflict-kind-concurrent-write = Gleichzeitige Bearbeitung  # MT
sync-conflict-kind-delete-edit = Löschen ↔ Bearbeiten  # MT
sync-conflict-kind-add-add = Beide Seiten hinzugefügt  # MT
sync-conflict-kind-corrupt-equal = Inhalt divergierte ohne neue Schreibung  # MT
sync-resolve-keep-left = Links behalten  # MT
sync-resolve-keep-right = Rechts behalten  # MT
sync-resolve-keep-both = Beide behalten  # MT
sync-resolve-three-way = Über 3-Wege-Merge auflösen  # MT
sync-resolve-phase-53-tooltip = Interaktiver 3-Wege-Merge für Nicht-Text-Dateien landet in Phase 53.  # MT
sync-error-prefix = Sync-Fehler  # MT

# Phase 26 — real-time mirror watcher. MT-flagged drafts;
# the authoritative English source lives in locales/en/copythat.ftl.
live-mirror-start = Live-Mirror starten  # MT
live-mirror-stop = Live-Mirror stoppen  # MT
live-mirror-watching = Wird überwacht  # MT
live-mirror-toggle-hint = Automatisch bei jeder erkannten Dateisystemänderung erneut synchronisieren. Ein Hintergrund-Thread pro aktivem Paar.  # MT
watch-event-prefix = Dateiänderung  # MT
watch-overflow-recovered = Watcher-Puffer übergelaufen; zur Wiederherstellung wird neu aufgelistet  # MT

# Phase 27 — content-defined chunk store. MT-flagged drafts;
# the authoritative English source lives in locales/en/copythat.ftl.
chunk-store-section = Chunk-Speicher  # MT
chunk-store-enable = Chunk-Speicher aktivieren (Delta-Fortsetzung und Deduplizierung)  # MT
chunk-store-enable-hint = Teilt jede kopierte Datei nach Inhalt (FastCDC) und speichert Chunks inhaltsadressiert. Wiederholungen schreiben nur geänderte Chunks neu; Dateien mit gemeinsamen Inhalten werden automatisch dedupliziert.  # MT
chunk-store-location = Chunk-Speicher-Ort  # MT
chunk-store-max-size = Maximale Chunk-Speichergröße  # MT
chunk-store-prune = Chunks bereinigen älter als (Tage)  # MT
chunk-store-savings = { $gib } GiB durch Chunk-Deduplizierung gespart  # MT
chunk-store-disk-usage = Belegt { $size } in { $chunks } Chunks  # MT

# Phase 28 — tray-resident Drop Stack. MT-flagged drafts;
# the authoritative English source lives in locales/en/copythat.ftl.
dropstack-window-title = Drop Stack  # MT
dropstack-tray-open = Drop Stack  # MT
dropstack-empty-title = Drop Stack ist leer  # MT
dropstack-empty-hint = Ziehen Sie Dateien hierher aus dem Explorer oder klicken Sie mit der rechten Maustaste auf eine Auftragszeile, um sie hinzuzufügen.  # MT
dropstack-add-to-stack = Zum Drop Stack hinzufügen  # MT
dropstack-copy-all-to = Alles kopieren nach…  # MT
dropstack-move-all-to = Alles verschieben nach…  # MT
dropstack-clear = Stapel leeren  # MT
dropstack-remove-row = Aus dem Stapel entfernen  # MT
dropstack-path-missing-toast = { $path } entfernt — die Datei existiert nicht mehr.  # MT
dropstack-always-on-top = Drop Stack immer im Vordergrund halten  # MT
dropstack-show-tray-icon = Copy That-Symbol im Infobereich anzeigen  # MT
dropstack-open-on-start = Drop Stack beim App-Start automatisch öffnen  # MT
dropstack-count = { $count } Pfad  # MT

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
