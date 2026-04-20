app-name = Copy That 2026
# MT
window-title = Copy That 2026
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
err-io-other = Unbekannter E/A-Fehler

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
