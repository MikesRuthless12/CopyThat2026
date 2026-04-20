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
# MT — Phase 8 toast messages
toast-error-resolved = Errore risolto
# MT
toast-collision-resolved = Conflitto risolto
# MT
toast-elevated-unavailable = Il riprova con permessi elevati arriva nella fase 17 — non ancora disponibile
# MT
toast-error-log-exported = Registro errori esportato

# MT — Error modal
error-modal-title = Un trasferimento è fallito
# MT
error-modal-retry = Riprova
# MT
error-modal-retry-elevated = Riprova con permessi elevati
# MT
error-modal-skip = Salta
# MT
error-modal-skip-all-kind = Salta tutti gli errori di questo tipo
# MT
error-modal-abort = Interrompi tutto
# MT
error-modal-path-label = Percorso
# MT
error-modal-code-label = Codice

# MT — Error-kind labels
err-not-found = File non trovato
# MT
err-permission-denied = Autorizzazione negata
# MT
err-disk-full = Il disco di destinazione è pieno
# MT
err-interrupted = Operazione interrotta
# MT
err-verify-failed = Verifica post-copia fallita
# MT
err-io-other = Errore I/O sconosciuto

# MT — Collision modal
collision-modal-title = Il file esiste già
# MT
collision-modal-overwrite = Sovrascrivi
# MT
collision-modal-overwrite-if-newer = Sovrascrivi se più recente
# MT
collision-modal-skip = Salta
# MT
collision-modal-keep-both = Conserva entrambi
# MT
collision-modal-rename = Rinomina…
# MT
collision-modal-apply-to-all = Applica a tutti
# MT
collision-modal-source = Origine
# MT
collision-modal-destination = Destinazione
# MT
collision-modal-size = Dimensione
# MT
collision-modal-modified = Modificato
# MT
collision-modal-hash-check = Hash rapido (SHA-256)
# MT
collision-modal-rename-placeholder = Nuovo nome file
# MT
collision-modal-confirm-rename = Rinomina

# MT — Error log drawer
error-log-title = Registro errori
# MT
error-log-empty = Nessun errore registrato
# MT
error-log-export-csv = Esporta CSV
# MT
error-log-export-txt = Esporta testo
# MT
error-log-clear = Cancella registro
# MT
error-log-col-time = Ora
# MT
error-log-col-job = Lavoro
# MT
error-log-col-path = Percorso
# MT
error-log-col-code = Codice
# MT
error-log-col-message = Messaggio
# MT
error-log-col-resolution = Risoluzione

# MT — History drawer (Phase 9)
history-title = Cronologia
# MT
history-empty = Nessun lavoro registrato
# MT
history-unavailable = La cronologia delle copie non è disponibile. L'app non è riuscita ad aprire l'archivio SQLite all'avvio.
# MT
history-filter-any = qualsiasi
# MT
history-filter-kind = Tipo
# MT
history-filter-status = Stato
# MT
history-filter-text = Cerca
# MT
history-refresh = Aggiorna
# MT
history-export-csv = Esporta CSV
# MT
history-purge-30 = Elimina > 30 giorni
# MT
history-rerun = Riesegui
# MT
history-detail-open = Dettagli
# MT
history-detail-title = Dettagli lavoro
# MT
history-detail-empty = Nessun elemento registrato
# MT
history-col-date = Data
# MT
history-col-kind = Tipo
# MT
history-col-src = Origine
# MT
history-col-dst = Destinazione
# MT
history-col-files = File
# MT
history-col-size = Dimensione
# MT
history-col-status = Stato
# MT
history-col-duration = Durata
# MT
history-col-error = Errore

# MT
toast-history-exported = Cronologia esportata
# MT
toast-history-rerun-queued = Riesecuzione in coda

# MT — Totals drawer (Phase 10)
footer-totals = Totali
# MT
totals-title = Totali
# MT
totals-loading = Caricamento totali…
# MT
totals-card-bytes = Byte totali copiati
# MT
totals-card-files = File
# MT
totals-card-jobs = Lavori
# MT
totals-card-avg-rate = Throughput medio
# MT
totals-errors = errori
# MT
totals-spark-title = Ultimi 30 giorni
# MT
totals-kinds-title = Per tipo
# MT
totals-saved-title = Tempo risparmiato (stimato)
# MT
totals-saved-note = Stimato rispetto a una copia di riferimento dello stesso carico con un file manager standard.
# MT
totals-reset = Ripristina statistiche
# MT
totals-reset-confirm = Elimina tutti i lavori e gli elementi memorizzati. Continuare?
# MT
totals-reset-confirm-yes = Sì, ripristina
# MT
toast-totals-reset = Statistiche azzerate

# MT — Phase 11a additions
header-language-label = Lingua
# MT
header-language-title = Cambia lingua

# MT
kind-copy = Copia
# MT
kind-move = Sposta
# MT
kind-delete = Elimina
# MT
kind-secure-delete = Eliminazione sicura

# MT
status-running = In esecuzione
# MT
status-succeeded = Completato
# MT
status-failed = Fallito
# MT
status-cancelled = Annullato
# MT
status-ok = OK
# MT
status-skipped = Saltato

# MT
history-search-placeholder = /percorso
# MT
toast-history-purged = Eliminati { $count } lavori più vecchi di 30 giorni

# MT
err-source-required = È richiesto almeno un percorso di origine.
# MT
err-destination-empty = Il percorso di destinazione è vuoto.
# MT
err-source-empty = Il percorso di origine è vuoto.

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
