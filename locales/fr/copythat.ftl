app-name = Copy That 2026
# MT
window-title = Copy That 2026
# MT
shred-ssd-advisory = Avertissement : cette cible se trouve sur un SSD. La réécriture en plusieurs passes ne nettoie pas fiablement la mémoire flash, car le nivellement de l'usure et le sur-provisionnement déplacent les données hors de l'adresse de bloc logique. Pour les supports SSD, préférez ATA SECURE ERASE, NVMe Format avec effacement sécurisé, ou le chiffrement intégral du disque avec destruction de la clé.

# MT
state-idle = Inactif
# MT
state-copying = Copie en cours
# MT
state-verifying = Vérification
# MT
state-paused = En pause
# MT
state-error = Erreur

# MT
state-pending = En file d'attente
# MT
state-running = En cours
# MT
state-cancelled = Annulé
# MT
state-succeeded = Terminé
# MT
state-failed = Échec

# MT
action-pause = Pause
# MT
action-resume = Reprendre
# MT
action-cancel = Annuler
# MT
action-pause-all = Mettre en pause toutes les tâches
# MT
action-resume-all = Reprendre toutes les tâches
# MT
action-cancel-all = Annuler toutes les tâches
# MT
action-close = Fermer
# MT
action-reveal = Afficher dans le dossier

# MT
menu-pause = Pause
# MT
menu-resume = Reprendre
# MT
menu-cancel = Annuler
# MT
menu-remove = Retirer de la file
# MT
menu-reveal-source = Afficher la source dans le dossier
# MT
menu-reveal-destination = Afficher la destination dans le dossier

# MT
header-eta-label = Temps restant estimé
# MT
header-toolbar-label = Commandes globales

# MT
footer-queued = tâches actives
# MT
footer-total-bytes = en cours
# MT
footer-errors = erreurs
# MT
footer-history = Historique

# MT
empty-title = Déposer des fichiers ou des dossiers à copier
# MT
empty-hint = Faites glisser des éléments sur la fenêtre. Nous demanderons une destination puis ajouterons une tâche par source.
# MT
empty-region-label = Liste des tâches

# MT
details-drawer-label = Détails de la tâche
# MT
details-source = Source
# MT
details-destination = Destination
# MT
details-state = État
# MT
details-bytes = Octets
# MT
details-files = Fichiers
# MT
details-speed = Vitesse
# MT
details-eta = Temps restant
# MT
details-error = Erreur

# MT
drop-dialog-title = Transférer les éléments déposés
# MT
drop-dialog-subtitle = { $count } élément(s) prêt(s) à être transférés. Choisissez un dossier de destination pour commencer.
# MT
drop-dialog-mode = Opération
# MT
drop-dialog-copy = Copier
# MT
drop-dialog-move = Déplacer
# MT
drop-dialog-pick-destination = Choisir la destination
# MT
drop-dialog-change-destination = Modifier la destination
# MT
drop-dialog-start-copy = Démarrer la copie
# MT
drop-dialog-start-move = Démarrer le déplacement

# MT
eta-calculating = calcul en cours…
# MT
eta-unknown = inconnu

# MT
toast-job-done = Transfert terminé
# MT
toast-copy-queued = Copie mise en file d'attente
# MT
toast-move-queued = Déplacement mis en file d'attente
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

# MT — Totals drawer (Phase 10)
footer-totals = Totals
totals-title = Totals
totals-loading = Loading totals…
totals-card-bytes = Total bytes copied
totals-card-files = Files
totals-card-jobs = Jobs
totals-card-avg-rate = Average throughput
totals-errors = errors
totals-spark-title = Last 30 days
totals-kinds-title = By kind
totals-saved-title = Time saved (estimated)
totals-saved-note = Estimated vs a baseline file-manager copy of the same workload.
totals-reset = Reset statistics
totals-reset-confirm = This deletes every stored job and item. Continue?
totals-reset-confirm-yes = Yes, reset
toast-totals-reset = Statistics reset
