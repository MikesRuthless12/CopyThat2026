app-name = Copy That v0.19.84
window-title = Copy That v0.19.84
shred-ssd-advisory = Attenzione: questa destinazione si trova su un SSD. Le sovrascritture multi-passaggio non sanificano in modo affidabile la memoria flash, perché il wear-leveling e l'over-provisioning spostano i dati al di fuori dell'indirizzo del blocco logico. Per i supporti a stato solido, preferisci ATA SECURE ERASE, NVMe Format con Secure Erase oppure la crittografia dell'intero disco con eliminazione della chiave.

# Global aggregate states (header pill)
state-idle = Inattivo
state-copying = Copia in corso
state-verifying = Verifica in corso
state-paused = In pausa
state-error = Errore

# Per-job states (row badge)
state-pending = In coda
state-running = In esecuzione
state-cancelled = Annullato
state-succeeded = Completato
state-failed = Non riuscito

# Actions
action-pause = Pausa
action-resume = Riprendi
action-cancel = Annulla
action-pause-all = Metti in pausa tutti i processi
action-resume-all = Riprendi tutti i processi
action-cancel-all = Annulla tutti i processi
action-close = Chiudi
action-reveal = Mostra nella cartella
action-add-files = Aggiungi file
action-add-folders = Aggiungi cartelle

# Phase 13d — activity feed
activity-title = Attività
activity-clear = Cancella elenco attività
activity-empty = Nessuna attività sui file al momento.
activity-after-done = Al termine:
activity-keep-open = Mantieni l'app aperta
activity-close-app = Chiudi l'app
activity-shutdown = Spegni il PC
activity-logoff = Disconnetti
activity-sleep = Sospendi

# Phase 14 — preflight free-space dialog
preflight-block-title = Spazio insufficiente nella destinazione
preflight-warn-title = Spazio ridotto nella destinazione
preflight-unknown-title = Impossibile determinare lo spazio libero
preflight-unknown-body = L'origine è troppo grande per essere misurata rapidamente oppure il volume di destinazione non ha risposto. Puoi continuare; la protezione dello spazio del motore interromperà la copia in modo pulito se lo spazio si esaurisce.
preflight-required = Necessario
preflight-free = Libero
preflight-reserve = Riserva
preflight-shortfall = Mancante
preflight-continue = Continua comunque
preflight-pick-subset = Scegli cosa copiare…
collision-modal-overwrite-older = Sovrascrivi solo i più vecchi

# Phase 14e — subset picker
subset-title = Scegli quali origini copiare
subset-subtitle = L'intera selezione non entra nella destinazione. Seleziona gli elementi da copiare; gli altri restano dove sono.
subset-loading = Misurazione delle dimensioni…
subset-too-large = troppo grande da conteggiare
subset-budget = Disponibile
subset-remaining = Rimanente
subset-confirm = Copia la selezione
history-rerun-hint = Riesegui questa copia — riscansiona ogni file nell'albero di origine
history-clear-all = Cancella tutto
history-clear-all-confirm = Fai di nuovo clic per confermare
history-clear-all-hint = Elimina ogni riga della cronologia. Richiede un secondo clic per confermare.
toast-history-cleared = Cronologia cancellata ({ $count } righe rimosse)

# Phase 15 — source-list ordering in the drop staging dialog
drop-dialog-sort-label = Ordine:
sort-custom = Personalizzato
sort-name-asc = Nome A → Z (prima i file)
sort-name-desc = Nome Z → A (prima i file)
sort-size-asc = Dimensione crescente (prima i file)
sort-size-desc = Dimensione decrescente (prima i file)
sort-reorder = Riordina
sort-move-top = Sposta all'inizio
sort-move-up = Sposta su
sort-move-down = Sposta giù
sort-move-bottom = Sposta alla fine

# Phase 16 — sort preset names for the Activity list (shorter
# wording than the DropStagingDialog variants; the Activity list
# is files-only so the "(files first)" suffix isn't relevant).
sort-name-asc-simple = Nome A → Z
sort-name-desc-simple = Nome Z → A
sort-size-asc-simple = Dimensione crescente
sort-size-desc-simple = Dimensione decrescente
activity-sort-locked = L'ordinamento è disabilitato durante una copia. Metti in pausa o attendi il termine, poi modifica l'ordine.

# Phase 17 — collision-policy picker in the DropStagingDialog
drop-dialog-collision-label = Se un file esiste già:
collision-policy-keep-both = Mantieni entrambi (rinomina la nuova copia in _2, _3, …)
collision-policy-skip = Salta la nuova copia
collision-policy-overwrite = Sovrascrivi il file esistente
collision-policy-overwrite-if-newer = Sovrascrivi solo se più recente
collision-policy-prompt = Chiedi ogni volta

# Phase 18 — progress labels on the DropStagingDialog Start button
drop-dialog-busy-checking = Verifica dello spazio libero…
drop-dialog-busy-enumerating = Conteggio dei file…
drop-dialog-busy-starting = Avvio della copia…
toast-enumeration-deferred = L'albero di origine è grande — l'elenco file iniziale verrà saltato; le righe appariranno man mano che il motore le elabora.

# Context menu (per-row right-click)
menu-pause = Pausa
menu-resume = Riprendi
menu-cancel = Annulla
menu-remove = Rimuovi dalla coda
menu-reveal-source = Mostra origine nella cartella
menu-reveal-destination = Mostra destinazione nella cartella

# Header / toolbar
header-eta-label = Tempo rimanente stimato
header-toolbar-label = Controlli globali

# Footer
footer-queued = processi attivi
footer-total-bytes = in trasferimento
footer-errors = errori
footer-history = Cronologia

# Empty state
empty-title = Trascina file o cartelle da copiare
empty-hint = Trascina gli elementi sulla finestra. Ti chiederemo una destinazione, poi metteremo in coda un processo per ogni origine.
empty-region-label = Elenco processi

# Details drawer
details-drawer-label = Dettagli del processo
details-source = Origine
details-destination = Destinazione
details-state = Stato
details-bytes = Byte
details-files = File
details-speed = Velocità
details-eta = Tempo stimato
details-error = Errore

# Drop dialog
drop-dialog-title = Trasferisci elementi trascinati
drop-dialog-subtitle = { $count } elemento/i pronto/i per il trasferimento. Scegli una cartella di destinazione per iniziare.
drop-dialog-mode = Operazione
drop-dialog-copy = Copia
drop-dialog-move = Sposta
drop-dialog-pick-destination = Scegli destinazione
drop-dialog-change-destination = Cambia destinazione
drop-dialog-start-copy = Avvia la copia
drop-dialog-start-move = Avvia lo spostamento

# ETA placeholders
eta-calculating = calcolo in corso…
eta-unknown = sconosciuto

# Toast messages
toast-job-done = Trasferimento completato
toast-copy-queued = Copia in coda
toast-move-queued = Spostamento in coda
toast-error-resolved = Errore risolto
toast-collision-resolved = Conflitto risolto
toast-elevated-unavailable = Il nuovo tentativo con privilegi elevati arriverà nella Phase 17 — non ancora disponibile
toast-clipboard-files-detected = File negli appunti — premi la scorciatoia di incolla per copiarli con Copy That
toast-clipboard-no-files = Negli appunti non ci sono file da incollare
toast-error-log-exported = Registro errori esportato

# Error modal (Phase 8)
error-modal-title = Un trasferimento non è riuscito
error-modal-retry = Riprova
error-modal-retry-elevated = Riprova con privilegi elevati
error-modal-skip = Salta
error-modal-skip-all-kind = Salta tutti gli errori di questo tipo
error-modal-abort = Interrompi tutto
error-modal-path-label = Percorso
error-modal-code-label = Codice
error-drawer-pending-count = Altri errori in attesa
error-drawer-toggle = Comprimi o espandi

# Error-kind labels (Phase 8). Source of truth — engine maps each
# `CopyErrorKind` to one of these keys via `localized_key()`.
err-not-found = File non trovato
err-permission-denied = Autorizzazione negata
err-disk-full = Il disco di destinazione è pieno
err-interrupted = Operazione interrotta
err-verify-failed = Verifica post-copia non riuscita
err-path-escape = Percorso rifiutato — contiene segmenti di directory superiore (..) o byte non validi
err-path-invalid-encoding = Percorso rifiutato — la stringa contiene UTF-8 non valido / caratteri di sostituzione
err-helper-invalid-json = L'helper con privilegi ha ricevuto JSON non valido; richiesta ignorata
err-helper-grant-out-of-band = GrantCapabilities deve essere gestito dal run-loop dell'helper, non dal gestore stateless
err-randomness-unavailable = Il generatore di numeri casuali del sistema operativo non ha funzionato; impossibile generare un ID di sessione
err-sparseness-mismatch = Impossibile preservare il layout sparso nella destinazione
err-io-other = Errore di I/O sconosciuto

# Collision modal (Phase 8)
collision-modal-title = Il file esiste già
collision-modal-overwrite = Sovrascrivi
collision-modal-overwrite-if-newer = Sovrascrivi se più recente
collision-modal-skip = Salta
collision-modal-keep-both = Mantieni entrambi
collision-modal-rename = Rinomina…
collision-modal-apply-to-all = Applica a tutti
collision-modal-source = Origine
collision-modal-destination = Destinazione
collision-modal-size = Dimensione
collision-modal-modified = Modificato
collision-modal-hash-check = Hash rapido (SHA-256)
collision-modal-hash-computing = Calcolo in corso…
collision-modal-hash-identical = Identici
collision-modal-hash-different = Diversi
collision-modal-rename-placeholder = Nuovo nome file
collision-modal-confirm-rename = Rinomina

# Error log drawer (Phase 8)
error-log-title = Registro errori
error-log-empty = Nessun errore registrato
error-log-export-csv = Esporta CSV
error-log-export-txt = Esporta testo
error-log-clear = Cancella registro
error-log-col-time = Ora
error-log-col-job = Processo
error-log-col-path = Percorso
error-log-col-code = Codice
error-log-col-message = Messaggio
error-log-col-resolution = Risoluzione

# History drawer (Phase 9)
history-title = Cronologia
history-empty = Nessun processo registrato al momento
history-unavailable = La cronologia delle copie non è disponibile. L'app non è riuscita ad aprire l'archivio SQLite all'avvio.
history-filter-any = qualsiasi
history-filter-kind = Tipo
history-filter-status = Stato
history-filter-text = Cerca
history-refresh = Aggiorna
history-export-csv = Esporta CSV
history-purge-30 = Elimina > 30 giorni
history-rerun = Riesegui
history-detail-open = Dettagli
history-detail-title = Dettagli del processo
history-detail-empty = Nessun elemento registrato
history-col-date = Data
history-col-kind = Tipo
history-col-src = Origine
history-col-dst = Destinazione
history-col-files = File
history-col-size = Dimensione
history-col-status = Stato
history-col-duration = Durata
history-col-error = Errore
toast-history-exported = Cronologia esportata
toast-history-rerun-queued = Riesecuzione in coda

# Totals drawer (Phase 10)
footer-totals = Totali
totals-title = Totali
totals-loading = Caricamento dei totali…
totals-card-bytes = Byte totali copiati
totals-card-files = File
totals-card-jobs = Processi
totals-card-avg-rate = Velocità media
totals-errors = errori
totals-spark-title = Ultimi 30 giorni
totals-kinds-title = Per tipo
totals-saved-title = Tempo risparmiato (stimato)
totals-saved-note = Stima rispetto a una copia dello stesso carico con un file manager di base.
totals-reset = Reimposta statistiche
totals-reset-confirm = Questa operazione elimina ogni processo ed elemento memorizzato. Continuare?
totals-reset-confirm-yes = Sì, reimposta
toast-totals-reset = Statistiche reimpostate

# Phase 11a — i18n core: surface remaining user-visible strings so
# every string on a main-window screen flows through Fluent.

# Header language switcher (temporary placement — folds into
# Settings → General in Phase 12).
header-language-label = Lingua
header-language-title = Cambia lingua

# Job-kind labels. History filter, Totals breakdown, and history
# rows all map wire-format `kind` strings to these labels.
kind-copy = Copia
kind-move = Sposta
kind-delete = Elimina
kind-secure-delete = Eliminazione sicura

# History status labels. Distinct from `state-*` because the
# history wire format uses plain `running`/`succeeded`/... rather
# than the live `JobState` enum the queue exposes.
status-running = In esecuzione
status-succeeded = Riuscito
status-failed = Non riuscito
status-cancelled = Annullato
# Per-item status (not job-level): `ok` / `skipped` are only
# reachable from the history detail view.
status-ok = OK
status-skipped = Saltato

# History drawer: search field placeholder + purge toast.
history-search-placeholder = /percorso
toast-history-purged = Eliminati { $count } processi più vecchi di 30 giorni

# User-facing command-layer validation errors. The Rust side
# returns these keys when input is missing; the toast layer looks
# them up rather than showing the raw English.
err-source-required = È richiesto almeno un percorso di origine.
err-destination-empty = Il percorso di destinazione è vuoto.
err-source-empty = Il percorso di origine è vuoto.

# Localised duration formatting for the Totals drawer and ETA
# fields. `{ $ms }`, `{ $s }`, `{ $m }`, `{ $h }` are integer
# placeables — the formatter passes pre-computed values in.
duration-lt-1s = < 1s
duration-ms = { $ms } ms
duration-seconds = { $s }s
duration-minutes-seconds = { $m }m { $s }s
duration-hours-minutes = { $h }h { $m }m
duration-zero = 0s

# Rate unit. Appended to a formatted byte size. Some languages
# render this with a leading space ("Ko/s"); keep it translatable
# even though the SI-derived "/s" is near-universal.
rate-unit-per-second = { $size }/s

# Phase 11b — Settings modal skeleton. Phase 12 expanded this into a
# full six-tab preferences window; the `settings-phase-12-hint` key
# is retired in favour of `settings-tab-profiles` + concrete labels.
settings-title = Impostazioni
settings-tab-general = Generale
settings-tab-appearance = Aspetto
settings-section-language = Lingua
settings-phase-12-hint = Altre impostazioni (tema, valori predefiniti di trasferimento, algoritmo di verifica, profili) arriveranno nella Phase 12.

# Phase 12 — full Settings window keys. Grouped by tab so the
# human-review pass can pick one section at a time. Every field
# label and every option label that the user will actually read
# has its own key; dynamic wire values (enum discriminants like
# "auto" / "prefer") stay untranslated on the wire.

settings-loading = Caricamento delle impostazioni…
settings-tab-transfer = Trasferimento
settings-tab-filters = Filtri
settings-tab-shell = Shell
settings-tab-secure-delete = Eliminazione sicura
settings-tab-advanced = Avanzate
settings-tab-updater = Aggiornamenti
settings-tab-profiles = Profili

# General tab additions
settings-section-theme = Tema
settings-theme-auto = Automatico
settings-theme-light = Chiaro
settings-theme-dark = Scuro
settings-start-with-os = Avvia all'accensione del sistema
settings-single-instance = Singola istanza in esecuzione
settings-minimize-to-tray = Riduci nella barra delle applicazioni alla chiusura
settings-error-display-mode = Stile della richiesta di errore
settings-error-display-modal = Modale (blocca l'app)
settings-error-display-drawer = Pannello (non bloccante)
settings-error-display-mode-hint = La modalità modale arresta la coda finché non decidi. Il pannello mantiene la coda attiva e ti permette di gestire gli errori nell'angolo.
settings-paste-shortcut = Incolla file tramite scorciatoia globale
settings-paste-shortcut-combo = Combinazione di tasti
settings-paste-shortcut-hint = Premi questa combinazione ovunque sul sistema per incollare i file copiati da Explorer / Finder / Files tramite Copy That. CmdOrCtrl corrisponde a Cmd su macOS e a Ctrl su Windows / Linux.
settings-clipboard-watcher = Monitora gli appunti per i file copiati
settings-clipboard-watcher-hint = Mostra un avviso quando negli appunti compaiono URL di file, suggerendo che puoi incollarli tramite Copy That. Controlla ogni 500 ms quando è attivo.

# Transfer tab
settings-buffer-size = Dimensione del buffer
settings-verify = Verifica dopo la copia
settings-verify-off = Disattivata
settings-concurrency = Concorrenza
settings-concurrency-auto = Automatica
settings-reflink = Reflink / percorsi rapidi
settings-reflink-prefer = Preferisci
settings-reflink-avoid = Evita reflink
settings-reflink-disabled = Usa sempre il motore asincrono
settings-fsync-on-close = Sincronizza su disco alla chiusura (più lento, più sicuro)
settings-preserve-timestamps = Mantieni i timestamp
settings-preserve-permissions = Mantieni le autorizzazioni
settings-preserve-acls = Mantieni gli ACL (Phase 14)
settings-preserve-sparseness = Mantieni i file sparsi
settings-preserve-sparseness-hint = Copia solo gli extent allocati dei file sparsi (dischi di VM, file di database) in modo che la destinazione mantenga su disco la stessa dimensione dell'origine.
settings-force-parallel-chunks = Copia parallela multi-blocco (solo RAID / array)
settings-force-parallel-chunks-hint = Divide ogni copia di grandi dimensioni in blocchi concorrenti. Aiuta solo destinazioni in striping/RAID/rete; RALLENTA un singolo SSD/NVMe (-25% a -76%). Lascia disattivato a meno che la destinazione non sia un array multidisco.

# Shell tab
settings-context-menu = Abilita le voci del menu contestuale della shell
settings-intercept-copy = Intercetta il gestore di copia predefinito (Windows)
settings-intercept-copy-hint = Quando è attivo, Ctrl+C / Ctrl+V di Explorer passa attraverso Copy That. La registrazione arriverà nella Phase 14.
settings-notify-completion = Avvisa al completamento del processo

# Secure delete tab
settings-shred-method = Metodo di triturazione predefinito
settings-shred-zero = Zero (1 passaggio)
settings-shred-random = Casuale (1 passaggio)
settings-shred-dod3 = DoD 5220.22-M (3 passaggi)
settings-shred-dod7 = DoD 5220.22-M (7 passaggi)
settings-shred-gutmann = Gutmann (35 passaggi)
settings-shred-nist = NIST 800-88
settings-shred-confirm-twice = Richiedi doppia conferma prima della triturazione

# Advanced tab
settings-log-level = Livello di log
settings-log-off = Disattivato
settings-telemetry = Telemetria
settings-telemetry-never = Mai — nessun invio di dati a nessun livello di log
settings-error-policy = Criterio di errore predefinito
settings-error-policy-ask = Chiedi
settings-error-policy-skip = Salta
settings-error-policy-retry = Riprova con backoff
settings-error-policy-abort = Interrompi al primo errore
settings-history-retention = Conservazione cronologia (giorni)
settings-history-retention-hint = 0 = conserva per sempre. Qualsiasi altro valore elimina automaticamente i processi più vecchi all'avvio.
settings-database-path = Percorso del database
settings-database-path-default = (predefinito — directory dati del sistema operativo)
settings-reset-all = Ripristina i valori predefiniti
settings-reset-confirm = Ripristinare ogni preferenza al valore predefinito? I profili non vengono modificati.

# Profiles tab
settings-profiles-hint = Salva le impostazioni attuali con un nome; caricalo in seguito per ripristinarle senza toccare le singole opzioni.
settings-profile-name-placeholder = Nome profilo
settings-profile-save = Salva
settings-profile-import = Importa…
settings-profile-load = Carica
settings-profile-export = Esporta…
settings-profile-delete = Elimina
settings-profile-empty = Nessun profilo salvato al momento.
settings-profile-import-prompt = Nome per il profilo importato:

# Toasts driven by Phase 12 profile actions
toast-settings-reset = Impostazioni reimpostate
toast-profile-saved = Profilo salvato
toast-profile-loaded = Profilo caricato
toast-profile-exported = Profilo esportato
toast-profile-imported = Profilo importato

# Phase 14a — enumeration-time filters
settings-filters-hint = Salta i file in fase di enumerazione, così il motore non li apre nemmeno. Le inclusioni si applicano solo ai file; le esclusioni eliminano anche le directory corrispondenti.
settings-filters-enabled = Abilita i filtri per le copie ad albero
settings-filters-include-globs = Glob di inclusione
settings-filters-include-globs-placeholder = **/*.txt
settings-filters-include-globs-hint = Un glob per riga. Quando non è vuoto, un file deve corrispondere ad almeno un'inclusione per essere mantenuto. Nelle directory si entra sempre.
settings-filters-exclude-globs = Glob di esclusione
settings-filters-exclude-globs-placeholder = **/node_modules
settings-filters-exclude-globs-hint = Un glob per riga. Per le directory, le corrispondenze eliminano l'intero sottoalbero; i file corrispondenti vengono saltati.
settings-filters-size-range = Intervallo di dimensione dei file
settings-filters-min-size-bytes = Dimensione minima (byte, vuoto = nessun minimo)
settings-filters-max-size-bytes = Dimensione massima (byte, vuoto = nessun massimo)
settings-filters-date-range = Intervallo dell'ora di modifica
settings-filters-min-mtime = Modificato a partire da
settings-filters-max-mtime = Modificato fino a
settings-filters-attributes = Bit di attributo
settings-filters-skip-hidden = Salta i file / le cartelle nascosti
settings-filters-skip-system = Salta i file di sistema (solo Windows)
settings-filters-skip-readonly = Salta i file di sola lettura

# Phase 15 — auto-update
settings-updater-hint = Copy That verifica gli aggiornamenti firmati al massimo una volta al giorno. Gli aggiornamenti si installano alla successiva chiusura dell'app.
settings-updater-auto-check = Verifica aggiornamenti all'avvio
settings-updater-channel = Canale di rilascio
settings-updater-channel-stable = Stabile
settings-updater-channel-beta = Beta (pre-rilascio)
settings-updater-last-check = Ultima verifica
settings-updater-last-never = Mai
settings-updater-check-now = Verifica aggiornamenti ora
settings-updater-checking = Verifica in corso…
settings-updater-available = Aggiornamento disponibile
settings-updater-up-to-date = Stai usando l'ultima versione.
settings-updater-dismiss = Ignora questa versione
settings-updater-dismissed = Ignorato
toast-update-available = È disponibile una versione più recente
toast-update-up-to-date = Stai già usando l'ultima versione

# Phase 19a — disk-backed file enumeration (TeraCopy-compatible scan DB)
scan-progress-title = Scansione in corso…
scan-progress-stats = { $files } file · { $bytes } finora
scan-pause-button = Metti in pausa la scansione
scan-resume-button = Riprendi la scansione
scan-cancel-button = Annulla la scansione
scan-cancel-confirm = Annullare la scansione e scartare i progressi?
scan-db-header = Database di scansione
scan-db-hint = Database di scansione su disco per processi con milioni di file.
advanced-scan-hash-during = Calcola i checksum durante la scansione
advanced-scan-db-path = Posizione del database di scansione
advanced-scan-retention-days = Elimina automaticamente le scansioni completate dopo (giorni)
advanced-scan-max-keep = Numero massimo di database di scansione da conservare

# Phase 19b — filesystem-snapshot source for locked files.
settings-on-locked = Quando un file è bloccato
settings-on-locked-ask = Chiedi la prima volta
settings-on-locked-retry = Riprova brevemente, poi mostra l'errore
settings-on-locked-skip = Salta il file bloccato
settings-on-locked-snapshot = Usa uno snapshot del filesystem
settings-on-locked-hint = Elimina gli errori "file in uso da un altro processo". Copy That crea uno snapshot del volume di origine (VSS su Windows, ZFS/Btrfs su Linux, APFS su macOS) e legge dalla copia snapshot.
snapshot-prompt-title = Questo file è in uso da un altro processo
snapshot-prompt-body = Un altro programma ha { $path } aperto in scrittura esclusiva. Scegli come Copy That deve gestire questo file e altri simili sullo stesso volume.
snapshot-source-active = 📷 Lettura dallo snapshot { $kind } di { $volume }
snapshot-create-failed = Impossibile creare uno snapshot del volume di origine
snapshot-vss-needs-elevation = La lettura da uno snapshot VSS richiede autorizzazioni di amministratore. Copy That ti chiederà di consentirla.
snapshot-cleanup-failed = L'helper degli snapshot ha segnalato un errore di pulizia — una copia shadow residua potrebbe rimanere sul volume.

# Phase 20 — durable resume journal.
resume-prompt-title = Riprendere i trasferimenti precedenti?
resume-prompt-body = Copy That ha rilevato { $count } trasferimento/i incompleto/i da una sessione precedente. Scegli cosa fare per ciascuno.
resume-prompt-resume = Riprendi
resume-prompt-resume-all = Riprendi tutto
resume-discard-one = Non riprendere
resume-discard-all = Scarta tutto
resume-aborted-hash-mismatch = I primi { $offset } byte della destinazione non corrispondono all'origine — riavvio dall'inizio.
settings-auto-resume = Riprendi automaticamente i processi interrotti senza chiedere
settings-auto-resume-hint = Salta la richiesta di ripresa all'avvio e rimetti silenziosamente in coda ogni processo incompleto. Disattivato per impostazione predefinita.

# Phase 21 — bandwidth shaping (GCRA token bucket + schedule + auto-throttle).
settings-tab-network = Rete
settings-network-hint = Limita la velocità di trasferimento per mantenere utilizzabile il resto della rete. Applicalo globalmente, segui una pianificazione giornaliera o reagisci automaticamente a connessioni Wi-Fi a consumo / batteria / cellulari.
settings-network-mode = Limite di banda
settings-network-mode-off = Disattivato (nessun limite)
settings-network-mode-fixed = Valore fisso
settings-network-mode-schedule = Usa pianificazione
settings-network-cap-mbps = Limite (MB/s)
settings-network-schedule = Pianificazione (formato rclone)
settings-network-schedule-hint = Limiti HH:MM,velocità separati da spazi più regole giornaliere opzionali Mon-Fri,velocità. Velocità: 512k, 10M, 2G, off, unlimited. Esempio: 08:00,512k 18:00,10M Sat-Sun,unlimited.
settings-network-auto-header = Limitazione automatica
settings-network-auto-metered = Su Wi-Fi a consumo
settings-network-auto-battery = A batteria
settings-network-auto-cellular = Su rete cellulare
settings-network-auto-unchanged = Non modificare
settings-network-auto-pause = Metti in pausa i trasferimenti
settings-network-auto-cap = Limita a un valore fisso
shape-badge-paused = in pausa
shape-badge-tooltip = Limite di banda attivo — fai clic per aprire Impostazioni → Rete
shape-badge-source-schedule = pianificato
shape-badge-source-metered = a consumo
shape-badge-source-battery = a batteria
shape-badge-source-cellular = cellulare
shape-badge-source-settings = attivo
shape-error-schedule-invalid = Il formato della pianificazione non è valido: { $message }

# Phase 22 — aggregate conflict dialog v2 (thumbnails, per-pattern
# rules, and reusable conflict profiles). Every key below is user-
# visible text in the `ConflictBatchModal.svelte` component.
conflict-batch-title = { $count } conflitti di file in { $jobname }
conflict-batch-state-pending = In attesa
conflict-batch-state-resolved = Risolto
conflict-batch-action-overwrite = Sovrascrivi
conflict-batch-action-skip = Salta
conflict-batch-action-keep-both = Mantieni entrambi
conflict-batch-action-newer-wins = Vince il più recente
conflict-batch-action-larger-wins = Vince il più grande
conflict-batch-bulk-apply-selected = Applica ai selezionati
conflict-batch-bulk-apply-extension = Applica a tutti quelli con questa estensione
conflict-batch-bulk-apply-glob = Applica al glob corrispondente…
conflict-batch-bulk-apply-remaining = Applica a tutti i rimanenti
conflict-batch-bulk-glob-placeholder = es. **/*.tmp
conflict-batch-save-profile = Salva queste regole come profilo…
conflict-batch-profile-placeholder = Nome profilo
conflict-batch-matched-rule = tramite la regola '{ $rule }' → { $action }
conflict-batch-empty = Tutti i conflitti risolti
conflict-batch-source-vs-destination = Origine vs. destinazione
conflict-batch-source-label = Origine
conflict-batch-destination-label = Destinazione
conflict-batch-size-label = Dimensione
conflict-batch-modified-label = Modificato
conflict-batch-close = Chiudi
conflict-batch-profile-saved = Profilo di conflitto salvato

# Phase 23 — sparse-file preservation. The toast fires once per
# destination volume when the filesystem can't preserve holes; the
# warning line is surfaced in the job detail drawer so the user knows
# the dst is larger on disk than the source was.
sparse-not-supported-title = La destinazione riempie i file sparsi
sparse-not-supported-body = { $dst_fs } non supporta i file sparsi. I vuoti nell'origine sono stati scritti come zeri, quindi la destinazione occupa più spazio su disco.
sparse-warning-densified = Layout sparso mantenuto: sono stati copiati solo gli extent allocati.
sparse-warning-mismatch = Discrepanza nel layout sparso — la destinazione potrebbe essere più grande del previsto.

# Phase 24 — security-metadata preservation. The Mark-of-the-Web
# (Zone.Identifier ADS) toggle is security-sensitive: turning it off
# lets a downloaded executable shed its SmartScreen / Office Protected
# View flag on copy, which is why the tooltip carries an explicit
# warning. AppleDouble fallback emits `._<filename>` sidecars on
# destination filesystems that can't hold the foreign metadata.
settings-preserve-security-metadata = Mantieni i metadati di sicurezza
settings-preserve-security-metadata-hint = Acquisisci e riapplica i flussi di metadati fuori banda (NTFS ADS / xattr / ACL POSIX / contesti SELinux / capability dei file Linux / resource fork di macOS) a ogni copia.
settings-preserve-motw = Mantieni il Mark-of-the-Web (flag di provenienza da Internet)
settings-preserve-motw-hint = Fondamentale per la sicurezza. SmartScreen e Office Protected View usano questo flusso per avvisare sui file scaricati da Internet. Disattivarlo permette a un eseguibile scaricato di perdere il proprio marcatore di origine durante la copia ed eludere le protezioni del sistema operativo.
settings-preserve-posix-acls = Mantieni gli ACL POSIX e gli attributi estesi
settings-preserve-posix-acls-hint = Trasferisci gli xattr user.* / system.* / trusted.* e le liste di controllo accessi POSIX durante la copia.
settings-preserve-selinux = Mantieni i contesti SELinux
settings-preserve-selinux-hint = Trasferisci l'etichetta security.selinux durante la copia, così i daemon che girano sotto criteri MAC possono comunque accedere al file.
settings-preserve-resource-forks = Mantieni i resource fork e le informazioni del Finder di macOS
settings-preserve-resource-forks-hint = Trasferisci durante la copia il resource fork legacy e le FinderInfo (etichette colore, metadati Carbon).
settings-appledouble-fallback = Usa il sidecar AppleDouble sui filesystem incompatibili
meta-translated-to-appledouble = Metadati esterni memorizzati nel sidecar AppleDouble (._{ $ext })

# Phase 25 — two-way sync with vector-clock conflict detection.
# The drawer lists configured sync pairs; each pair runs an
# independent reconciliation round against a per-pair `.copythat-sync.db`
# state store. Concurrent edits from a common ancestor surface as
# conflicts rather than silent overwrites; the losing side's content
# is preserved as `name.sync-conflict-YYYYMMDD-HHMMSS-<host>.ext`.
footer-sync = Sincronizzazione
sync-drawer-title = Sincronizzazione bidirezionale
sync-drawer-hint = Mantieni due cartelle sincronizzate senza sovrascritture silenziose. Le modifiche simultanee emergono come conflitti che puoi risolvere.
sync-add-pair = Aggiungi coppia
sync-add-cancel = Annulla
sync-refresh = Aggiorna
sync-add-save = Salva coppia
sync-add-saving = Salvataggio…
sync-add-missing-fields = Etichetta, percorso sinistro e percorso destro sono tutti obbligatori.
sync-remove-confirm = Rimuovere questa coppia di sincronizzazione? Il database di stato viene mantenuto; le cartelle restano intatte.
sync-field-label = Etichetta
sync-field-label-placeholder = es. Documenti ↔ NAS
sync-field-left = Cartella sinistra
sync-field-left-placeholder = Scegli o incolla un percorso assoluto
sync-field-right = Cartella destra
sync-field-right-placeholder = Scegli o incolla un percorso assoluto
sync-field-mode = Modalità
sync-mode-two-way = Bidirezionale
sync-mode-mirror-left-to-right = Mirror (sinistra → destra)
sync-mode-mirror-right-to-left = Mirror (destra → sinistra)
sync-mode-contribute-left-to-right = Contribuisci (sinistra → destra, nessuna eliminazione)
sync-no-pairs = Nessuna coppia di sincronizzazione configurata. Fai clic su "Aggiungi coppia" per iniziare.
sync-loading = Caricamento delle coppie configurate…
sync-never-run = Mai eseguita
sync-running = In esecuzione
sync-run-now = Esegui ora
sync-cancel = Annulla
sync-remove-pair = Rimuovi
sync-view-conflicts = Visualizza conflitti ({ $count })
sync-conflicts-heading = Conflitti
sync-no-conflicts = Nessun conflitto dall'ultima esecuzione.
sync-winner = Vincitore
sync-side-left-to-right = sinistra
sync-side-right-to-left = destra
sync-conflict-kind-concurrent-write = Modifica simultanea
sync-conflict-kind-delete-edit = Eliminazione ↔ modifica
sync-conflict-kind-add-add = Aggiunta da entrambi i lati
sync-conflict-kind-corrupt-equal = Il contenuto è divergente senza una nuova scrittura
sync-resolve-keep-left = Mantieni sinistra
sync-resolve-keep-right = Mantieni destra
sync-resolve-keep-both = Mantieni entrambe
sync-resolve-three-way = Risolvi con merge a 3 vie
sync-resolve-phase-53-tooltip = Il merge interattivo a 3 vie per i file non testuali arriverà nella Phase 53.
sync-error-prefix = Errore di sincronizzazione

# Phase 26 — real-time mirror watcher. "Live mirror" starts a
# filesystem watcher on the pair's left side; every debounced event
# triggers a re-sync. The watcher filters vim swap files / Office
# lock files / atomic-save temp names so one logical save becomes
# exactly one sync round.
live-mirror-start = Avvia mirror in tempo reale
live-mirror-stop = Arresta mirror in tempo reale
live-mirror-watching = Monitoraggio
live-mirror-toggle-hint = Sincronizza automaticamente a ogni modifica del filesystem rilevata. Un thread in background per ogni coppia attiva.
watch-event-prefix = Modifica file
watch-overflow-recovered = Il buffer del watcher è andato in overflow; nuova enumerazione per il recupero

# Phase 27 — content-defined chunk store. Enables delta-resume (a
# retry only re-writes chunks that actually changed) and same-job
# dedup (files sharing content blocks store those blocks once). The
# store is disk-backed under `<data-dir>/chunks/` by default and is
# the foundation for the Phase 49–51 moonshot repository phases.
chunk-store-section = Archivio chunk
chunk-store-enable = Abilita l'archivio chunk (delta-resume e dedup)
chunk-store-enable-hint = Suddivide ogni file copiato per contenuto (FastCDC) e memorizza i chunk con indirizzamento per contenuto. I nuovi tentativi riscrivono solo i chunk modificati; i file con contenuti condivisi vengono deduplicati automaticamente.
chunk-store-location = Posizione dell'archivio chunk
chunk-store-max-size = Dimensione massima dell'archivio chunk
chunk-store-prune = Elimina i chunk più vecchi di (giorni)
chunk-store-savings = Risparmiati { $gib } GiB grazie alla dedup dei chunk
chunk-store-disk-usage = In uso { $size } su { $chunks } chunk

# Phase 28 — tray-resident Drop Stack. The stack is a persistent list
# of paths gathered from multiple sources (Explorer drag, main-window
# context menu, CLI, drag onto the Drop Stack window) that the user
# can dispatch to a destination in one go.
dropstack-window-title = Drop Stack
dropstack-tray-open = Drop Stack
dropstack-empty-title = Drop Stack è vuoto
dropstack-empty-hint = Trascina qui i file da Explorer o fai clic con il tasto destro su una riga di processo per aggiungerla.
dropstack-add-to-stack = Aggiungi a Drop Stack
dropstack-copy-all-to = Copia tutto in…
dropstack-move-all-to = Sposta tutto in…
dropstack-clear = Svuota lo stack
dropstack-remove-row = Rimuovi dallo stack
dropstack-path-missing-toast = { $path } trascinato — il file non esiste più.
dropstack-always-on-top = Mantieni Drop Stack sempre in primo piano
dropstack-show-tray-icon = Mostra l'icona di Copy That nella barra delle applicazioni
dropstack-open-on-start = Apri automaticamente Drop Stack all'avvio dell'app
dropstack-count = { $count } percorso

# Phase 29 — spring-loaded folders + native DnD polish. The Settings
# → General tab carries the knobs; DropTarget / DestinationPicker
# Svelte components use the dropzone-* keys at runtime.
settings-dnd-heading = Trascina e rilascia
settings-dnd-spring-load = Apri le cartelle al passaggio durante il trascinamento
settings-dnd-spring-delay = Ritardo di apricartelle (ms)
settings-dnd-thumbnails = Mostra le anteprime durante il trascinamento
settings-dnd-invalid-highlight = Evidenzia le destinazioni di rilascio non valide
dropzone-invalid-title = Non è una destinazione di rilascio valida
dropzone-invalid-readonly = La destinazione è di sola lettura
dropzone-picker-title = Scegli una destinazione
dropzone-picker-up = Su
dropzone-picker-path = Percorso corrente
dropzone-picker-root = Radici
dropzone-picker-use-this = Usa questa cartella
dropzone-picker-empty = Nessuna sottocartella
dropzone-picker-cancel = Annulla

# Phase 30 — cross-platform path translation. Settings → Transfer
# exposes these under a "Cross-platform compatibility" subsection.
translate-heading = Compatibilità multipiattaforma
translate-unicode-label = Normalizzazione Unicode
translate-unicode-auto = Rileva automaticamente la destinazione
translate-unicode-windows = NFC (Windows / Linux)
translate-unicode-macos = Lascia invariato (macOS / APFS)
translate-line-endings-label = Converti i fine riga per i file di testo
translate-line-endings-allowlist = Estensioni dei file di testo
reserved-name-label = Gestione dei nomi riservati di Windows
reserved-name-suffix = Aggiungi "_" (CON.txt → CON_.txt)
reserved-name-reject = Rifiuta e avvisa
long-path-label = Usa il prefisso per percorsi lunghi di Windows (\\?\) oltre 260 caratteri
long-path-hint = Alcune condivisioni di rete e strumenti legacy non rispettano lo spazio dei nomi \\?\.

# Phase 31 — power-aware copying. Settings → Power & State tab + the
# header badge that renders "⏸ Paused — Zoom call detected" when the
# runner's power subscriber has paused or capped due to a policy match.
power-heading = Alimentazione e stato
power-enabled = Abilita le regole basate sull'alimentazione
power-battery-label = A batteria
power-metered-label = Su Wi-Fi a consumo
power-cellular-label = Su rete cellulare
power-presentation-label = Durante una presentazione (Zoom / Teams / Keynote)
power-fullscreen-label = Quando un'app è a schermo intero
power-thermal-label = Quando la CPU è in throttling termico
power-rule-continue = Continua a piena velocità
power-rule-pause = Metti in pausa tutti i processi
power-rule-cap = Limita la banda
power-rule-cap-percent = Limita a una percentuale della velocità attuale
power-reason-on-battery = a batteria
power-reason-metered-network = rete a consumo
power-reason-cellular-network = rete cellulare
power-reason-presenting = modalità presentazione
power-reason-fullscreen = app a schermo intero
power-reason-thermal-throttling = la CPU è in throttling

# Phase 32 — cloud backend matrix via OpenDAL. Settings → Remotes
# tab + the Add-backend wizard that writes one entry per remote into
# the keychain-backed credential store.
remote-heading = Backend remoti
remote-add = Aggiungi backend
remote-list-empty = Nessun backend remoto configurato
remote-test = Prova la connessione
remote-test-success = Connessione riuscita
remote-test-failed = Connessione non riuscita
remote-remove = Rimuovi backend
remote-name-label = Nome visualizzato
remote-kind-label = Tipo di backend
remote-save = Salva backend
remote-cancel = Annulla
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
backend-local-fs = Filesystem locale
cloud-config-bucket = Bucket
cloud-config-region = Regione
cloud-config-endpoint = URL dell'endpoint
cloud-config-root = Percorso radice
cloud-error-invalid-config = La configurazione del backend non è valida
cloud-error-network = Errore di rete nel contattare il backend
cloud-error-not-found = Oggetto non trovato nel percorso richiesto
cloud-error-permission = Autorizzazione negata dal backend remoto
cloud-error-keychain = Accesso al portachiavi del sistema operativo non riuscito
settings-tab-remotes = Backend remoti
settings-tab-mobile = Mobile

# Phase 33 — mount Copy That's chunk store + history archive as a
# read-only filesystem (FUSE / WinFsp). Surfaced by the History tab
# context menu's "Mount snapshot" action + the Settings → Advanced
# "Mount latest on launch" toggle.
mount-heading = Monta snapshot
mount-action-mount = Monta snapshot
mount-action-unmount = Smonta
mount-status-mounted = Montato in { $path }
mount-error-unsafe-mountpoint = Il percorso del punto di montaggio non è sicuro
mount-error-mountpoint-not-empty = Il punto di montaggio deve essere una directory vuota
mount-error-backend-unavailable = Il backend di montaggio non è disponibile su questo sistema
mount-error-archive-read = Lettura dell'archivio non riuscita
mount-picker-title = Scegli la directory del punto di montaggio
mount-toast-mounted = Snapshot montato in { $path }
mount-toast-unmounted = Snapshot smontato
mount-toast-failed = Montaggio non riuscito: { $reason }
settings-mount-heading = Monta snapshot
settings-mount-hint = Esponi l'archivio della cronologia come filesystem di sola lettura. La Phase 33b collega il flusso del runner; i backend FUSE/WinFsp del kernel arriveranno nella Phase 33c.
settings-mount-on-launch = Monta l'ultimo snapshot all'avvio
settings-mount-on-launch-path = Percorso del punto di montaggio
settings-mount-on-launch-path-placeholder = es. C:\Mounts\copythat

# Phase 34 — enterprise-grade audit log export + WORM (write-once-read-
# many) tamper-resistance. Settings → Advanced → Audit log collects the
# format + destination + rotation + WORM toggle; runtime records job and
# file events into the tamper-evident chain-hashed sink.
settings-audit-heading = Registro di controllo
settings-audit-hint = Registro a sola aggiunta e a prova di manomissione di ogni evento di processo e di file. I formati includono CSV, JSON-lines, Syslog RFC 5424, ArcSight CEF e QRadar LEEF.
settings-audit-enable = Abilita la registrazione di controllo
settings-audit-format = Formato del registro
settings-audit-format-json-lines = JSON lines (predefinito consigliato)
settings-audit-format-csv = CSV (compatibile con i fogli di calcolo)
settings-audit-format-syslog = Syslog (RFC 5424)
settings-audit-format-cef = CEF (ArcSight)
settings-audit-format-leef = LEEF 2.0 (IBM QRadar)
settings-audit-file-path = Percorso del file di registro
settings-audit-file-path-placeholder = es. C:\ProgramData\CopyThat\audit.log
settings-audit-max-size = Ruota dopo (byte, 0 = mai)
settings-audit-worm = Abilita la modalità WORM (write-once-read-many)
settings-audit-worm-hint = Applica il flag a sola aggiunta della piattaforma (Linux chattr +a, macOS chflags uappnd, attributo di sola lettura di Windows) dopo ogni creazione o rotazione. Persino un amministratore deve rimuovere esplicitamente il flag per troncare il registro.
settings-audit-test-write = Scrittura di prova
settings-audit-verify-chain = Verifica la catena
toast-audit-test-write-ok = Scrittura di prova del registro di controllo riuscita
toast-audit-verify-ok = Catena di controllo verificata e integra
toast-audit-verify-failed = La verifica della catena di controllo ha segnalato delle discrepanze

# Phase 35 — destination encryption via age (X25519 / passphrase /
# SSH) + on-the-fly zstd compression with a per-extension deny
# heuristic. Surfaced through Settings → Transfer → Encryption +
# Compression; engine short-circuits to the crypt pipeline when
# either stage is active.
settings-crypt-heading = Crittografia e compressione
settings-crypt-hint = Trasforma il contenuto dei file prima che arrivi a destinazione. La crittografia usa il formato age; la compressione usa zstd e può saltare i media già compressi in base all'estensione.
settings-crypt-encryption-mode = Crittografia
settings-crypt-encryption-off = Disattivata
settings-crypt-encryption-passphrase = Passphrase (richiesta all'avvio della copia)
settings-crypt-encryption-recipients = Chiavi dei destinatari da file
settings-crypt-encryption-hint = Le passphrase restano in memoria solo per la durata della copia. I file dei destinatari elencano una chiave pubblica age1… o ssh- per riga.
settings-crypt-recipients-file = Percorso del file dei destinatari
settings-crypt-recipients-file-placeholder = es. C:\Users\me\recipients.txt
settings-crypt-compression-mode = Compressione
settings-crypt-compression-off = Disattivata
settings-crypt-compression-always = Sempre
settings-crypt-compression-smart = Intelligente (salta i media già compressi)
settings-crypt-compression-hint = La modalità intelligente salta jpg, mp4, zip, 7z e formati simili che non traggono vantaggio da zstd. La modalità Sempre comprime ogni file al livello scelto.
settings-crypt-compression-level = Livello zstd (1-22)
settings-crypt-compression-level-hint = I numeri più bassi sono più veloci; quelli più alti comprimono di più. Il livello 3 corrisponde al valore predefinito della CLI di zstd.
compress-footer-savings = 💾 { $original } → { $compressed } ({ $percent }% risparmiato)
compress-savings-toast = Compresso del { $percent }% ({ $bytes } risparmiati)
crypt-toast-recipients-loaded = Caricati { $count } destinatari di crittografia
crypt-toast-recipients-error = Caricamento dei destinatari non riuscito: { $reason }
crypt-toast-passphrase-required = La crittografia richiede una passphrase prima dell'avvio della copia
crypt-toast-passphrase-set = Passphrase di crittografia acquisita
crypt-footer-encrypted-badge = 🔒 Crittografato (age)
crypt-footer-compressed-badge = 📦 Compresso (zstd)

# Phase 36 — copythat CLI surface. Documented exit codes + the
# user-facing strings for plan/apply/verify/config. Help text stays
# in English (engineering accessibility) but error / info / status
# strings are localized so a localized desktop installation surfaces
# CLI errors in the same language as the GUI.
cli-help-tagline = Copy That CLI — copia di file byte-esatta, sincronizzazione, verifica e controllo per pipeline CI/CD.
cli-help-exit-codes = Codici di uscita: 0 successo, 1 errore, 2 in attesa, 3 conflitto, 4 verifica fallita, 5 rete, 6 autorizzazioni, 7 disco pieno, 8 annullamento, 9 configurazione.
cli-error-bad-args = copy/move richiede almeno un'origine e una destinazione
cli-error-unknown-algo = Algoritmo di verifica sconosciuto: { $algo }
cli-error-missing-spec = --spec è obbligatorio per plan/apply
cli-error-spec-parse = Analisi del jobspec { $path } non riuscita: { $reason }
cli-error-spec-empty-sources = L'elenco delle origini del jobspec è vuoto
cli-info-shape-recorded = Limite di banda "{ $rate }" registrato; l'applicazione avviene tramite copythat-shape
cli-info-stub-deferred = { $command } è in attesa del collegamento successivo della Phase 36
cli-plan-summary = Piano: { $actions } azione/i, { $bytes } byte; { $already_done } già a posto
cli-plan-pending = Il piano segnala azioni in sospeso; riesegui con `apply` per eseguirle
cli-plan-already-done = Il piano segnala che non c'è nulla da fare (idempotente)
cli-apply-success = Apply terminato senza errori
cli-apply-failed = Apply terminato con uno o più errori
cli-verify-ok = Verifica riuscita: { $algo } { $digest }
cli-verify-failed = Verifica NON RIUSCITA per { $path } ({ $algo })
cli-config-set = Impostato { $key } = { $value }
cli-config-reset = { $key } reimpostato al valore predefinito
cli-config-unknown-key = Chiave di configurazione sconosciuta: { $key }
cli-completions-emitted = Completamenti della shell per { $shell } stampati su stdout

# Phase 37 — desktop-side mobile companion. Settings → Mobile panel
# strings + the SAS-confirmation modal + push-notification toasts.
# The actual mobile UI lives in the Phase 37 follow-up Tauri Mobile
# target; the phone displays its own localized SAS prompt.
settings-mobile-heading = Companion mobile
settings-mobile-hint = Associa un iPhone o un telefono Android per sfogliare la cronologia, avviare profili salvati e jobspec della Phase 36 e ricevere notifiche di completamento.
settings-mobile-pair-toggle = Consenti nuove associazioni
settings-mobile-pair-active = Server di associazione attivo — scansiona il QR con l'app mobile di Copy That
settings-mobile-pair-button = Avvia associazione
settings-mobile-revoke-button = Revoca
settings-mobile-no-pairings = Nessun dispositivo associato al momento
settings-mobile-pair-port = Porta di binding (0 = scegline una libera)
pair-sas-prompt = Entrambe le schermate dovrebbero mostrare le stesse quattro emoji. Tocca Corrispondono se sono uguali.
pair-sas-confirm = Corrispondono
pair-sas-reject = Non corrispondono — annulla
pair-toast-success = Associato con { $device }
pair-toast-failed = Associazione non riuscita: { $reason }
push-toast-sent = Notifica push inviata a { $device }
push-toast-failed = Notifica push a { $device } non riuscita: { $reason }

# Phase 38 — aggregate destination dedup + reflink fallback ladder.
# Settings → Transfer → Dedup panel + per-job-row badges (⚡
# Reflinked / 🔗 Hardlinked / 🧩 Chunk-shared / 📋 Copied) +
# pre-pass dedup-scan modal.
settings-dedup-heading = Dedup della destinazione
settings-dedup-hint = Quando origine e destinazione condividono un volume, Copy That può clonare i file a livello di filesystem invece di copiare i byte. Il reflink è istantaneo e sicuro; l'hardlink è più veloce ma entrambi i nomi condividono lo stato.
settings-dedup-mode-auto = Scala automatica (reflink → hardlink → chunk → copia)
settings-dedup-mode-reflink-only = Solo reflink
settings-dedup-mode-hardlink-aggressive = Aggressiva (reflink + hardlink anche sui file scrivibili)
settings-dedup-mode-off = Disabilitata (sempre copia byte a byte)
settings-dedup-hardlink-policy = Criterio degli hardlink
settings-dedup-prescan = Pre-scansiona l'albero di destinazione per contenuti duplicati
dedup-badge-reflinked = ⚡ Reflink eseguito
dedup-badge-hardlinked = 🔗 Hardlink eseguito
dedup-badge-chunk-shared = 🧩 Chunk condivisi
dedup-badge-copied = 📋 Copiato
phase42-paranoid-verify-label = Verifica paranoica
phase42-paranoid-verify-hint = Scarta le pagine in cache della destinazione e rilegge dal disco per individuare le bugie della cache di scrittura e la corruzione silenziosa. Circa il 50% più lenta della verifica predefinita; disattivata per impostazione predefinita.
phase42-sharing-violation-retries-label = Tentativi sui file di origine bloccati
phase42-sharing-violation-retries-hint = Quante volte riprovare quando un altro processo mantiene il file di origine aperto con un blocco esclusivo. Il backoff raddoppia a ogni tentativo (50 ms / 100 ms / 200 ms per impostazione predefinita). Predefinito 3, come Robocopy /R:3.
phase42-cloud-placeholder-warning = { $name } è un file OneDrive solo cloud. Copiarlo avvierà un download — fino a { $size } sulla tua connessione di rete.
phase42-defender-exclusion-hint = Per la massima velocità di copia, aggiungi la cartella di destinazione alle esclusioni di Microsoft Defender prima dei trasferimenti di massa. Vedi docs/PERFORMANCE_TUNING.md.

# Phase 39 — Browser-accessible recovery UI. Settings → Advanced
# exposes these strings; the recovery server itself renders askama
# templates that always read English — these eight keys are the
# Settings prose the user actually reads in their preferred locale.
settings-recovery-heading = Interfaccia web di ripristino
settings-recovery-enable = Abilita l'interfaccia web di ripristino
settings-recovery-bind-address = Indirizzo di binding
settings-recovery-port = Porta (0 = scegline una libera)
settings-recovery-show-url = Mostra URL e token
settings-recovery-rotate-token = Ruota il token
settings-recovery-allow-non-loopback = Consenti il binding non-loopback
settings-recovery-non-loopback-warning = ATTENZIONE: abilitare un binding non-loopback espone l'interfaccia di ripristino alla tua rete locale. Chiunque conosca il token può sfogliare la cronologia dei file e scaricarli. Proteggila con TLS o un reverse proxy se la LAN non è attendibile.

# Phase 40 — SMB compression negotiation + cloud-VM offload helper.
# 6 SMB keys (header badge + Settings prose) + 6 cloud-offload keys
# (Remotes tab wizard for cross-cloud copy templates).
smb-compress-badge = 🗜 Compressione SMB: { $algo }
smb-compress-badge-tooltip = Il traffico di rete verso questa destinazione viene compresso durante il transito (SMB 3.1.1).
smb-compress-toast-saved = Risparmiati { $bytes } sulla rete
smb-compress-algo-unknown = algoritmo sconosciuto
settings-smb-compress-heading = Compressione di rete SMB
settings-smb-compress-hint = Negozia automaticamente la compressione del traffico SMB 3.1.1 sulle destinazioni UNC. Vantaggio gratuito sui collegamenti lenti; ignorato sulle destinazioni locali.
cloud-offload-heading = Helper di offload su VM cloud
cloud-offload-hint = Quando copi direttamente tra due cloud, genera un modello di distribuzione che esegue la copia da una piccola VM effimera nel cloud — i byte non toccano mai la rete del tuo laptop.
cloud-offload-render-button = Genera modello
cloud-offload-copy-clipboard = Copia negli appunti
cloud-offload-template-format = Formato del modello
cloud-offload-self-destruct-warning = La VM si spegne automaticamente dopo { $minutes } minuti — conferma il ruolo IAM e la regione prima della distribuzione.

# Phase 41 — animated before/after tree-diff preview. The `Preview
# changes` modal renders the rolled-up plan before the engine starts
# work; 14 keys cover the title, the summary header / counts, the
# row-reason labels, and the two action buttons.
preview-modal-title = Anteprima delle modifiche
preview-summary-header = Cosa accadrà
preview-category-additions = { $count } aggiunte
preview-category-replacements = { $count } sostituzioni
preview-category-skips = { $count } saltati
preview-category-conflicts = { $count } conflitti
preview-category-unchanged = { $count } invariati
preview-bytes-to-transfer = { $bytes } da trasferire
preview-reason-source-newer = L'origine è più recente
preview-reason-dest-newer = La destinazione è più recente — verrà saltata
preview-reason-content-different = Il contenuto è diverso
preview-reason-identical = Identico all'origine
preview-button-run = Esegui il piano
preview-button-reduce = Riduci il mio piano…

# Phase 42 — perceptual-hash visual-similarity dedup. Eight keys cover
# the pre-copy "looks visually identical" warning + the Settings panel.
perceptual-warn-title = Sembra visivamente identico
perceptual-warn-body = { $name } nella destinazione sembra corrispondere all'immagine di origine. Continuare comunque la copia?
perceptual-warn-keep-both = Mantieni entrambi
perceptual-warn-skip = Salta questo file
perceptual-warn-overwrite = Sovrascrivi comunque
perceptual-settings-heading = Dedup per somiglianza visiva
perceptual-settings-hint = Rileva le immagini visivamente identiche nella destinazione prima che vengano sovrascritte. L'hash è percettivo (riconosce la stessa immagine risalvata in un formato diverso), non byte-esatto.
perceptual-settings-threshold-label = Soglia di avviso (più bassa = corrispondenza più rigida)

# Phase 42 Part B — per-file rolling versions (Time Machine for any
# destination). 8 keys cover the version-list panel + retention picker.
version-list-heading = Versioni precedenti
version-list-empty = Nessuna versione precedente di questo file
version-list-restore = Ripristina questa versione
version-retention-heading = Conserva le versioni precedenti in caso di sovrascrittura
version-retention-none = Conserva ogni versione per sempre
version-retention-last-n = Conserva le ultime { $n } versioni
version-retention-older-than-days = Elimina le versioni più vecchie di { $days } giorni
version-retention-gfs = Oraria { $h } · giornaliera { $d } · settimanale { $w } · mensile { $m }

# Phase 43 — forensic chain-of-custody manifests + BLAKE3 verified
# streaming. 14 keys cover the Settings → Provenance panel
# (heading, hint, toggles, TSA URL, signing-key management) plus
# the post-job manifest notification + the verify command's two
# headline result lines.
provenance-settings-heading = Catena di custodia forense
provenance-settings-hint = Firma ogni processo di copia con un manifest BLAKE3 + ed25519. I revisori possono in seguito ricalcolare l'hash dell'albero di destinazione e dimostrare che nessun byte è cambiato dalla copia.
provenance-settings-enable-default = Firma ogni nuovo processo per impostazione predefinita
provenance-settings-show-after-job = Mostra il manifest dopo ogni processo completato
provenance-settings-tsa-url-label = URL predefinito dell'autorità di marcatura temporale RFC 3161
provenance-settings-tsa-url-hint = Facoltativo. Quando è impostato, i manifest contengono una marcatura temporale TSA gratuita che dimostra l'esistenza dei byte in quel momento. Lascia vuoto per saltare.
provenance-settings-keys-heading = Chiavi di firma
provenance-settings-keys-generate = Genera nuova chiave
provenance-settings-keys-import = Importa chiave…
provenance-settings-keys-export = Esporta chiave pubblica…
provenance-job-completed-title = Manifest di provenienza salvato
provenance-job-completed-body = { $count } file firmati → { $path }
provenance-verify-clean = Manifest valido per { $count } file; firma { $sig }; radice merkle OK.
provenance-verify-tampered = Manifest NON VALIDO — { $tampered } manomessi, { $missing } mancanti.
# Phase 43 post-review hardening — toast text for the Settings →
# Provenance buttons whose Tauri IPC has not yet landed.
provenance-action-staged = Phase 43 — il collegamento dell'IPC per questa azione arriverà in un commit successivo.

# Phase 44 — SSD-aware whole-drive sanitize (NVMe Sanitize / OPAL
# Crypto Erase) + the per-file shred refusal on copy-on-write
# filesystems. 16 keys cover the new "Drive sanitize" Settings
# subsection and the localized error messages.
sanitize-heading = Sanificazione sicura dell'intero disco
sanitize-hint = NVMe Sanitize, OPAL Crypto Erase e ATA Secure Erase cancellano un'unità flash a livello firmware in millisecondi. La sovrascrittura per file è inutile su flash — la triturazione multi-passaggio non fa che consumare la NAND. Usa questo per una cancellazione effettiva.
sanitize-pick-device = Scegli l'unità da sanificare
sanitize-mode-label = Metodo di sanificazione
sanitize-mode-nvme-format = NVMe Format (con secure erase)
sanitize-mode-nvme-sanitize-block = NVMe Sanitize — Block Erase (lento, ogni cella)
sanitize-mode-nvme-sanitize-crypto = NVMe Sanitize — Crypto Erase (istantaneo)
sanitize-mode-ata-secure-erase = ATA Secure Erase (SSD SATA legacy)
sanitize-mode-opal-crypto-erase = TCG OPAL Crypto Erase (unità autocrittografanti)
sanitize-mode-apfs-crypto-erase = APFS Crypto Erase (ruota la chiave FileVault, solo macOS)
sanitize-confirm-1 = Questa operazione distrugge OGNI byte su { $device }. Non è reversibile.
sanitize-confirm-2 = Comprendo che tutte le partizioni, tutti i file e tutti gli snapshot su { $device } diventeranno definitivamente illeggibili.
sanitize-confirm-3 = Digita il nome del modello dell'unità per procedere: { $model }
sanitize-running = Sanificazione di { $device } ({ $mode }) — può richiedere da millisecondi (crypto erase) a decine di minuti (block erase). Non spegnere.
sanitize-completed = Sanificazione completata — { $device } è ora vuota.
ssd-honest-shred-meaningless = La triturazione per file su un filesystem copy-on-write (Btrfs / ZFS / APFS) non può raggiungere i blocchi sottostanti. Usa invece la sanificazione dell'intero disco più la rotazione della chiave di crittografia dell'intero disco.
ssd-honest-advisory = Questo file si trova su flash. La sovrascrittura per file consuma la NAND e NON garantisce che le celle originali siano irrecuperabili. Per i dati sensibili, sanifica l'intero disco.

# Phase 44.1f post-review — placeholder toast for SanitizeTab
# buttons whose Tauri IPC has not yet landed.
sanitize-action-staged = Phase 44.1 — il collegamento dell'IPC per questa azione arriverà in un commit successivo.

# Phase 45.3 — named-queue tab strip (Subfeature A). Tabs surface
# once the QueueRegistry holds at least one queue; the synthesised
# default tab keeps legacy single-queue jobs reachable.
queue-tab-default = Predefinita
queue-tab-empty-state = Code dei processi
queue-badge-tooltip = Processi in attesa e in esecuzione in questa coda

# Phase 45.4 — drag-progress-merge (Subfeature B). Drag a queue tab
# onto another to merge their job lists. The default tab is neither
# draggable nor a drop target; only registry queues participate.
queue-drag-hint = Trascina su un'altra coda per unirle
queue-merge-confirm = Rilascia per unire
queue-merge-toast = Code unite

# Phase 45.5 — F2-queue UX (Subfeature C). F2 toggles
# `auto_enqueue_next` so every fresh enqueue piles into the running
# queue rather than spawning a parallel one. Status pill renders in
# the Footer; pulsing dot renders on the running tab.
queue-f2-active-hint = Modalità F2: ogni nuovo accodamento finisce in questa coda
queue-f2-toggled-on = Modalità coda F2 ATTIVA — i nuovi accodamenti si uniscono alla coda in esecuzione
queue-f2-toggled-off = Modalità coda F2 DISATTIVA — i nuovi accodamenti creano code parallele
queue-f2-status-bar = Modalità coda F2: ATTIVA

# Phase 45.6 — tray destination targets (Subfeature D). Pinned
# destinations appear in the OS tray menu; clicking one arms it as
# the active drop target so the next file drop bypasses the
# DropStagingDialog. Settings → General hosts the list editor.
tray-target-section-title = Destinazioni nella barra delle applicazioni
tray-target-section-hint = Le destinazioni aggiunte compaiono nel menu della barra delle applicazioni. Fai clic su una per impostarla come prossima destinazione di rilascio.
tray-target-empty = Nessuna destinazione aggiunta alla barra delle applicazioni.
tray-target-remove = Rimuovi
tray-target-add-label = Etichetta
tray-target-add-path = Percorso o URI del backend
tray-target-add = Aggiungi
tray-target-armed-toast = Rilascia il prossimo file per inviarlo a { $label }
tray-target-active-pill = → { $label }

# Phase 45.7 follow-up — pinned-destination validation errors. The
# `queue_pin_destination` IPC returns these Fluent keys verbatim
# when input fails the IPC-boundary checks; the toast layer renders
# them via `t(...)` (Toast.svelte detects kebab-case-lowercase and
# routes through the locale table). Phase 17e's `err-path-escape` /
# `err-destination-empty` cover the path-traversal + empty cases for
# `queue_route_job`; the keys below are pin-specific.
err-pinned-destination-label-empty = L'etichetta della destinazione nella barra delle applicazioni non può essere vuota.
err-pinned-destination-path-empty = Il percorso della destinazione nella barra delle applicazioni non può essere vuoto.
err-pinned-destination-label-too-long = L'etichetta della destinazione nella barra delle applicazioni è troppo lunga (massimo 64 caratteri).
err-pinned-destination-path-too-long = Il percorso della destinazione nella barra delle applicazioni è troppo lungo (massimo 1024 caratteri).
err-pinned-destination-label-invalid = L'etichetta della destinazione nella barra delle applicazioni contiene caratteri non consentiti (a capo, ritorno o NUL).
err-pinned-destination-path-invalid = Il percorso della destinazione nella barra delle applicazioni contiene caratteri non consentiti (a capo, ritorno o NUL).
err-pinned-destination-too-many = Hai raggiunto il limite di 50 destinazioni nella barra delle applicazioni. Rimuovine una per aggiungerne un'altra.

# Phase 46.6 — Settings → Plugins tab. The IPC layer in
# `apps/copythat-ui/src-tauri/src/plugin_commands.rs` enumerates the
# per-user plugin store under `<config_dir>/plugins/`; the
# `PluginsTab.svelte` component renders these strings against the
# returned manifest + grant state.
settings-tab-plugins = Plugin
plugin-heading = Plugin
plugin-hint = I plugin WASM in sandbox estendono Copy That con hook personalizzati. Ogni plugin viene eseguito con limiti di CPU e memoria per chiamata e vede solo le capacità host che gli concedi.
plugin-list-empty = Nessun plugin installato al momento.
plugin-enabled = Abilitato
plugin-disabled = Disabilitato
plugin-hooks = Hook
plugin-capabilities = Capacità
plugin-no-capabilities = (nessuna)
plugin-directory = Posizione
plugin-install-from-file = Installa da file…
plugin-install-from-url = Installa da URL…
plugin-url-wasm = URL WASM
plugin-url-manifest = URL del manifest
plugin-url-hash = Hash BLAKE3
plugin-url-preview = Anteprima
plugin-url-confirm = Conferma installazione

# Phase 31b — power-policy settings (Power tab).
settings-tab-power = Alimentazione
settings-power-hint = Limita o sospendi le copie in base all'alimentazione: batteria, rete a consumo/cellulare, presentazione/schermo intero o throttling termico della CPU.
settings-power-enabled = Abilita la limitazione in base all'alimentazione
settings-power-battery = A batteria
settings-power-metered = Su rete a consumo
settings-power-cellular = Su rete cellulare
settings-power-presentation = Durante una presentazione
settings-power-fullscreen = A schermo intero
settings-power-thermal = Con throttling termico
settings-power-continue = Continua
settings-power-pause = Sospendi
err-server-not-implemented = La modalità server non è ancora disponibile.
err-webhook-not-implemented = La consegna dei webhook non è ancora disponibile.

# Phase 47 — "why is this slow?" diagnostics (bottleneck badge + tooltip).
bottleneck-source-io = Origine I/O
bottleneck-dest-io = Destinazione I/O
bottleneck-network = Rete
bottleneck-antivirus = Antivirus
bottleneck-cpu = CPU
bottleneck-thermal = Termico
bottleneck-unknown = Sconosciuto
diag-aria = Collo di bottiglia: { $cause }
diag-tooltip = Limitato da { $cause } · { $rate }
diag-spark-aria = Velocità nell'ultimo minuto
diag-keeping-up = Al passo
diag-label = Diagnostica
