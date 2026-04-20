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
