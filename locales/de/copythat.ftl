app-name = Copy That v1.0.0
# MT
window-title = Copy That v1.0.0
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
