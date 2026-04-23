app-name = Copy That v1.0.0
# MT
window-title = Copy That v1.0.0
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
toast-clipboard-files-detected = File negli appunti — premi la scorciatoia di incolla per copiare tramite Copy That
toast-clipboard-no-files = Gli appunti non contengono file da incollare
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
error-drawer-pending-count = Altri errori in attesa
error-drawer-toggle = Comprimi o espandi

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
err-path-escape = Percorso rifiutato — contiene segmenti di cartella superiore (..) o byte non validi
# MT
err-io-other = Errore I/O sconosciuto
err-sparseness-mismatch = Layout sparso non preservato sulla destinazione  # MT

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

# MT — Phase 11b Settings modal
settings-title = Impostazioni
# MT
settings-tab-general = Generale
# MT
settings-tab-appearance = Aspetto
# MT
settings-section-language = Lingua
# MT
settings-phase-12-hint = Altre impostazioni (tema, valori predefiniti di trasferimento, algoritmo di verifica, profili) arriveranno nella fase 12.

# MT — Phase 12 Settings window
settings-loading = Caricamento impostazioni…
# MT
settings-tab-transfer = Trasferimento
# MT
settings-tab-shell = Shell
# MT
settings-tab-secure-delete = Eliminazione sicura
# MT
settings-tab-advanced = Avanzate
# MT
settings-tab-profiles = Profili

# MT
settings-section-theme = Tema
# MT
settings-theme-auto = Automatico
# MT
settings-theme-light = Chiaro
# MT
settings-theme-dark = Scuro
# MT
settings-start-with-os = Avvia con il sistema
# MT
settings-single-instance = Istanza singola
# MT
settings-minimize-to-tray = Riduci nella barra delle applicazioni alla chiusura
settings-error-display-mode = Stile avviso di errore
settings-error-display-modal = Modale (blocca l'app)
settings-error-display-drawer = Pannello laterale (non bloccante)
settings-error-display-mode-hint = La modale ferma la coda finché non decidi. Il pannello laterale mantiene la coda attiva e ti lascia gestire gli errori nell'angolo.
settings-paste-shortcut = Incolla file tramite scorciatoia globale
settings-paste-shortcut-combo = Combinazione tasti
settings-paste-shortcut-hint = Premi questa combinazione ovunque nel sistema per incollare file copiati da Esplora risorse / Finder / File tramite Copy That. CmdOrCtrl si risolve in Cmd su macOS e Ctrl su Windows / Linux.
settings-clipboard-watcher = Controlla gli appunti per file copiati
settings-clipboard-watcher-hint = Mostra un avviso quando URL di file appaiono negli appunti, suggerendo che puoi incollare tramite Copy That. Controlla ogni 500 ms quando attivo.

# MT
settings-buffer-size = Dimensione del buffer
# MT
settings-verify = Verifica dopo la copia
# MT
settings-verify-off = Disattivato
# MT
settings-concurrency = Concorrenza
# MT
settings-concurrency-auto = Automatica
# MT
settings-reflink = Reflink / percorsi rapidi
# MT
settings-reflink-prefer = Preferisci
# MT
settings-reflink-avoid = Evita reflink
# MT
settings-reflink-disabled = Usa sempre il motore asincrono
# MT
settings-fsync-on-close = Sincronizza sul disco alla chiusura (più lento, più sicuro)
# MT
settings-preserve-timestamps = Conserva i timestamp
# MT
settings-preserve-permissions = Conserva i permessi
# MT
settings-preserve-acls = Conserva gli ACL (fase 14)
settings-preserve-sparseness = Preserva i file sparsi  # MT
settings-preserve-sparseness-hint = Copia solo le estensioni allocate dei file sparsi (dischi VM, file di database) in modo che la destinazione mantenga la stessa dimensione su disco dell'origine.  # MT

# MT
settings-context-menu = Abilita le voci del menu contestuale
# MT
settings-intercept-copy = Intercetta il gestore di copia predefinito (Windows)
# MT
settings-intercept-copy-hint = Quando attivo, Ctrl+C / Ctrl+V nell'Esplora risorse passa per Copy That. Registrazione nella fase 14.
# MT
settings-notify-completion = Notifica al completamento del lavoro

# MT
settings-shred-method = Metodo di distruzione predefinito
# MT
settings-shred-zero = Zero (1 passaggio)
# MT
settings-shred-random = Casuale (1 passaggio)
# MT
settings-shred-dod3 = DoD 5220.22-M (3 passaggi)
# MT
settings-shred-dod7 = DoD 5220.22-M (7 passaggi)
# MT
settings-shred-gutmann = Gutmann (35 passaggi)
# MT
settings-shred-nist = NIST 800-88
# MT
settings-shred-confirm-twice = Richiedi doppia conferma prima della distruzione

# MT
settings-log-level = Livello di log
# MT
settings-log-off = Disattivato
# MT
settings-telemetry = Telemetria
# MT
settings-telemetry-never = Mai — nessun invio di dati a qualsiasi livello di log
# MT
settings-error-policy = Politica di errore predefinita
# MT
settings-error-policy-ask = Chiedi
# MT
settings-error-policy-skip = Salta
# MT
settings-error-policy-retry = Riprova con backoff
# MT
settings-error-policy-abort = Annulla al primo errore
# MT
settings-history-retention = Conservazione cronologia (giorni)
# MT
settings-history-retention-hint = 0 = conserva per sempre. Qualsiasi altro valore elimina automaticamente i lavori più vecchi all'avvio.
# MT
settings-database-path = Percorso del database
# MT
settings-database-path-default = (predefinito — directory dati del sistema)
# MT
settings-reset-all = Ripristina impostazioni predefinite
# MT
settings-reset-confirm = Ripristinare tutte le preferenze? I profili non vengono toccati.

# MT
settings-profiles-hint = Salva le impostazioni attuali con un nome; caricale in seguito per cambiare senza toccare i singoli controlli.
# MT
settings-profile-name-placeholder = Nome del profilo
# MT
settings-profile-save = Salva
# MT
settings-profile-import = Importa…
# MT
settings-profile-load = Carica
# MT
settings-profile-export = Esporta…
# MT
settings-profile-delete = Elimina
# MT
settings-profile-empty = Nessun profilo salvato.
# MT
settings-profile-import-prompt = Nome per il profilo importato:

# MT
toast-settings-reset = Impostazioni ripristinate
# MT
toast-profile-saved = Profilo salvato
# MT
toast-profile-loaded = Profilo caricato
# MT
toast-profile-exported = Profilo esportato
# MT
toast-profile-imported = Profilo importato

# Phase 13d — activity feed + header picker buttons
action-add-files = Aggiungi file
action-add-folders = Aggiungi cartelle
activity-title = Attività
activity-clear = Svuota elenco attività
activity-empty = Nessuna attività file.
activity-after-done = Al termine:
activity-keep-open = Mantieni l'app aperta
activity-close-app = Chiudi app
activity-shutdown = Spegni PC
activity-logoff = Disconnetti
activity-sleep = Sospendi

# Phase 14 — preflight free-space dialog
preflight-block-title = Spazio insufficiente nella destinazione
preflight-warn-title = Spazio ridotto nella destinazione
preflight-unknown-title = Impossibile determinare lo spazio libero
preflight-unknown-body = La sorgente è troppo grande per essere misurata rapidamente o il volume di destinazione non ha risposto. Puoi continuare; il motore fermerà la copia in modo pulito se lo spazio si esaurisce.
preflight-required = Richiesto
preflight-free = Libero
preflight-reserve = Riserva
preflight-shortfall = Deficit
preflight-continue = Continua comunque
collision-modal-overwrite-older = Sovrascrivi solo i più vecchi

# Phase 14e — subset picker
preflight-pick-subset = Scegli cosa copiare…
subset-title = Scegli le sorgenti da copiare
subset-subtitle = La selezione completa non entra nella destinazione. Spunta gli elementi da copiare; il resto resta.
subset-loading = Misura delle dimensioni…
subset-too-large = troppo grande da contare
subset-budget = Disponibile
subset-remaining = Rimanente
subset-confirm = Copia selezione
history-rerun-hint = Riesegui questa copia — ripete la scansione di ogni file nell'origine
history-clear-all = Cancella tutto
history-clear-all-confirm = Fai di nuovo clic per confermare
history-clear-all-hint = Elimina ogni riga della cronologia. Richiede un secondo clic per confermare.
toast-history-cleared = Cronologia cancellata ({ $count } righe rimosse)

# Phase 15 — source-list ordering
drop-dialog-sort-label = Ordinamento:
sort-custom = Personalizzato
sort-name-asc = Nome A → Z (prima i file)
sort-name-desc = Nome Z → A (prima i file)
sort-size-asc = Dimensione crescente (prima i file)
sort-size-desc = Dimensione decrescente (prima i file)
sort-reorder = Riordina
sort-move-top = Sposta in alto
sort-move-up = Su
sort-move-down = Giù
sort-move-bottom = Sposta in basso
sort-name-asc-simple = Nome A → Z
sort-name-desc-simple = Nome Z → A
sort-size-asc-simple = Più piccoli prima
sort-size-desc-simple = Più grandi prima
activity-sort-locked = L'ordinamento è disabilitato durante una copia. Metti in pausa o attendi la fine, poi cambia l'ordine.
drop-dialog-collision-label = Se un file esiste già:
collision-policy-keep-both = Mantieni entrambi (rinomina la nuova copia in _2, _3, …)
collision-policy-skip = Salta la nuova copia
collision-policy-overwrite = Sovrascrivi il file esistente
collision-policy-overwrite-if-newer = Sovrascrivi solo se più nuovo
collision-policy-prompt = Chiedi ogni volta
drop-dialog-busy-checking = Verifica spazio libero…
drop-dialog-busy-enumerating = Conteggio file…
drop-dialog-busy-starting = Avvio della copia…
toast-enumeration-deferred = L'albero di origine è grande — lista preliminare saltata; le righe appariranno mentre il motore le elabora.

# Phase 14a — enumeration-time filters
# MT
settings-tab-filters = Filtri
# MT
settings-filters-hint = Salta i file durante l'enumerazione così il motore non li apre nemmeno. Gli inclusi si applicano solo ai file; gli esclusi potano anche le cartelle corrispondenti.
# MT
settings-filters-enabled = Abilita i filtri per le copie di alberi
# MT
settings-filters-include-globs = Glob di inclusione
# MT
settings-filters-include-globs-placeholder = **/*.txt
# MT
settings-filters-include-globs-hint = Un glob per riga. Se non vuoto, un file deve corrispondere ad almeno uno. Le cartelle sono sempre attraversate.
# MT
settings-filters-exclude-globs = Glob di esclusione
# MT
settings-filters-exclude-globs-placeholder = **/node_modules
# MT
settings-filters-exclude-globs-hint = Un glob per riga. Le corrispondenze potano l'intero sottoalbero per le cartelle; i file corrispondenti vengono saltati.
# MT
settings-filters-size-range = Intervallo di dimensione
# MT
settings-filters-min-size-bytes = Dimensione minima (byte, vuoto = nessun limite inferiore)
# MT
settings-filters-max-size-bytes = Dimensione massima (byte, vuoto = nessun limite superiore)
# MT
settings-filters-date-range = Intervallo data di modifica
# MT
settings-filters-min-mtime = Modificato a partire dal
# MT
settings-filters-max-mtime = Modificato fino al
# MT
settings-filters-attributes = Attributi
# MT
settings-filters-skip-hidden = Salta file / cartelle nascosti
# MT
settings-filters-skip-system = Salta file di sistema (solo Windows)
# MT
settings-filters-skip-readonly = Salta file di sola lettura

# Phase 15 — auto-update
# MT
settings-tab-updater = Aggiornamenti
# MT
settings-updater-hint = Copy That cerca aggiornamenti firmati al massimo una volta al giorno. Gli aggiornamenti si installano alla prossima chiusura dell'app.
# MT
settings-updater-auto-check = Cerca aggiornamenti all'avvio
# MT
settings-updater-channel = Canale di rilascio
# MT
settings-updater-channel-stable = Stabile
# MT
settings-updater-channel-beta = Beta (pre-rilascio)
# MT
settings-updater-last-check = Ultimo controllo
# MT
settings-updater-last-never = Mai
# MT
settings-updater-check-now = Controlla aggiornamenti ora
# MT
settings-updater-checking = Verifica in corso…
# MT
settings-updater-available = Aggiornamento disponibile
# MT
settings-updater-up-to-date = Stai usando l'ultima versione.
# MT
settings-updater-dismiss = Salta questa versione
# MT
settings-updater-dismissed = Saltata
# MT
toast-update-available = È disponibile una versione più recente
# MT
toast-update-up-to-date = Sei già alla versione più recente

# Phase 19a — disk-backed file enumeration (TeraCopy-compatible scan DB)
# MT
scan-progress-title = Scansione in corso…
# MT
scan-progress-stats = { $files } file · { $bytes } finora
# MT
scan-pause-button = Sospendi scansione
# MT
scan-resume-button = Riprendi scansione
# MT
scan-cancel-button = Annulla scansione
# MT
scan-cancel-confirm = Annullare la scansione e scartare i progressi?
# MT
scan-db-header = Database di scansione
# MT
scan-db-hint = Database di scansione su disco per operazioni con milioni di file.
# MT
advanced-scan-hash-during = Calcola i checksum durante la scansione
# MT
advanced-scan-db-path = Posizione del database di scansione
# MT
advanced-scan-retention-days = Elimina automaticamente le scansioni completate dopo (giorni)
# MT
advanced-scan-max-keep = Numero massimo di database di scansione da conservare

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
sparse-not-supported-title = La destinazione riempie i file sparsi  # MT
sparse-not-supported-body = { $dst_fs } non supporta i file sparsi. I buchi nell'origine sono stati scritti come zeri, quindi la destinazione è più grande su disco.  # MT
sparse-warning-densified = Layout sparso preservato: sono state copiate solo le estensioni allocate.  # MT
sparse-warning-mismatch = Discordanza layout sparso — la destinazione potrebbe essere più grande del previsto.  # MT
