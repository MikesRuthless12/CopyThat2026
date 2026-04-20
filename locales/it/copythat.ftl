app-name = Copy That 2026
# MT
window-title = Copy That 2026
# MT
shred-ssd-advisory = Avviso: questo bersaglio si trova su un SSD. Le riscritture multiple non sanificano in modo affidabile la memoria flash perché il wear-leveling e l'over-provisioning spostano i dati fuori dall'indirizzo logico del blocco. Per i supporti a stato solido, preferire ATA SECURE ERASE, NVMe Format con Secure Erase o la crittografia dell'intero disco con successiva distruzione della chiave.

# MT
state-idle = Inattivo
# MT
state-copying = Copia in corso
# MT
state-verifying = Verifica
# MT
state-paused = In pausa
# MT
state-error = Errore

# MT
state-pending = In coda
# MT
state-running = In esecuzione
# MT
state-cancelled = Annullato
# MT
state-succeeded = Completato
# MT
state-failed = Fallito

# MT
action-pause = Pausa
# MT
action-resume = Riprendi
# MT
action-cancel = Annulla
# MT
action-pause-all = Metti in pausa tutti i lavori
# MT
action-resume-all = Riprendi tutti i lavori
# MT
action-cancel-all = Annulla tutti i lavori
# MT
action-close = Chiudi
# MT
action-reveal = Mostra nella cartella

# MT
menu-pause = Pausa
# MT
menu-resume = Riprendi
# MT
menu-cancel = Annulla
# MT
menu-remove = Rimuovi dalla coda
# MT
menu-reveal-source = Mostra origine nella cartella
# MT
menu-reveal-destination = Mostra destinazione nella cartella

# MT
header-eta-label = Tempo residuo stimato
# MT
header-toolbar-label = Controlli globali

# MT
footer-queued = lavori attivi
# MT
footer-total-bytes = in corso
# MT
footer-errors = errori
# MT
footer-history = Cronologia

# MT
empty-title = Rilasciare file o cartelle da copiare
# MT
empty-hint = Trascina elementi sulla finestra. Ti chiederemo la destinazione e poi aggiungeremo un lavoro per ogni origine.
# MT
empty-region-label = Elenco dei lavori

# MT
details-drawer-label = Dettagli del lavoro
# MT
details-source = Origine
# MT
details-destination = Destinazione
# MT
details-state = Stato
# MT
details-bytes = Byte
# MT
details-files = File
# MT
details-speed = Velocità
# MT
details-eta = Tempo residuo
# MT
details-error = Errore

# MT
drop-dialog-title = Trasferire gli elementi rilasciati
# MT
drop-dialog-subtitle = { $count } elemento/i pronti al trasferimento. Scegli una cartella di destinazione per iniziare.
# MT
drop-dialog-mode = Operazione
# MT
drop-dialog-copy = Copia
# MT
drop-dialog-move = Sposta
# MT
drop-dialog-pick-destination = Scegli destinazione
# MT
drop-dialog-change-destination = Cambia destinazione
# MT
drop-dialog-start-copy = Avvia copia
# MT
drop-dialog-start-move = Avvia spostamento

# MT
eta-calculating = calcolo in corso…
# MT
eta-unknown = sconosciuto

# MT
toast-job-done = Trasferimento completato
# MT
toast-copy-queued = Copia in coda
# MT
toast-move-queued = Spostamento in coda
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
