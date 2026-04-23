app-name = Copy That v1.0.0
# MT
window-title = Copy That v1.0.0
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
# MT — Phase 8 toast messages
toast-error-resolved = Erreur résolue
# MT
toast-collision-resolved = Conflit résolu
# MT
toast-elevated-unavailable = La nouvelle tentative avec droits élevés arrive en phase 17 — pas encore disponible
toast-clipboard-files-detected = Fichiers dans le presse-papiers — appuyez sur votre raccourci de collage pour copier via Copy That
toast-clipboard-no-files = Le presse-papiers ne contient aucun fichier à coller
# MT
toast-error-log-exported = Journal des erreurs exporté

# MT — Error modal
error-modal-title = Un transfert a échoué
# MT
error-modal-retry = Réessayer
# MT
error-modal-retry-elevated = Réessayer avec des droits élevés
# MT
error-modal-skip = Ignorer
# MT
error-modal-skip-all-kind = Ignorer toutes les erreurs de ce type
# MT
error-modal-abort = Tout annuler
# MT
error-modal-path-label = Chemin
# MT
error-modal-code-label = Code
error-drawer-pending-count = Autres erreurs en attente
error-drawer-toggle = Réduire ou développer

# MT — Error-kind labels
err-not-found = Fichier introuvable
# MT
err-permission-denied = Autorisation refusée
# MT
err-disk-full = Le disque de destination est plein
# MT
err-interrupted = Opération interrompue
# MT
err-verify-failed = Vérification après copie échouée
# MT
err-path-escape = Chemin rejeté — contient des segments de répertoire parent (..) ou des octets illégaux
# MT
err-io-other = Erreur d'E/S inconnue
err-sparseness-mismatch = La disposition clairsemée n'a pas pu être préservée sur la destination  # MT

# MT — Collision modal
collision-modal-title = Le fichier existe déjà
# MT
collision-modal-overwrite = Écraser
# MT
collision-modal-overwrite-if-newer = Écraser si plus récent
# MT
collision-modal-skip = Ignorer
# MT
collision-modal-keep-both = Conserver les deux
# MT
collision-modal-rename = Renommer…
# MT
collision-modal-apply-to-all = Appliquer à tous
# MT
collision-modal-source = Source
# MT
collision-modal-destination = Destination
# MT
collision-modal-size = Taille
# MT
collision-modal-modified = Modifié
# MT
collision-modal-hash-check = Empreinte rapide (SHA-256)
# MT
collision-modal-rename-placeholder = Nouveau nom de fichier
# MT
collision-modal-confirm-rename = Renommer

# MT — Error log drawer
error-log-title = Journal des erreurs
# MT
error-log-empty = Aucune erreur consignée
# MT
error-log-export-csv = Exporter CSV
# MT
error-log-export-txt = Exporter en texte
# MT
error-log-clear = Vider le journal
# MT
error-log-col-time = Heure
# MT
error-log-col-job = Tâche
# MT
error-log-col-path = Chemin
# MT
error-log-col-code = Code
# MT
error-log-col-message = Message
# MT
error-log-col-resolution = Résolution

# MT — History drawer (Phase 9)
history-title = Historique
# MT
history-empty = Aucune tâche enregistrée pour le moment
# MT
history-unavailable = L'historique des copies n'est pas disponible. L'application n'a pas pu ouvrir le magasin SQLite au démarrage.
# MT
history-filter-any = tous
# MT
history-filter-kind = Type
# MT
history-filter-status = État
# MT
history-filter-text = Rechercher
# MT
history-refresh = Actualiser
# MT
history-export-csv = Exporter CSV
# MT
history-purge-30 = Purger > 30 jours
# MT
history-rerun = Relancer
# MT
history-detail-open = Détails
# MT
history-detail-title = Détails de la tâche
# MT
history-detail-empty = Aucun élément enregistré
# MT
history-col-date = Date
# MT
history-col-kind = Type
# MT
history-col-src = Source
# MT
history-col-dst = Destination
# MT
history-col-files = Fichiers
# MT
history-col-size = Taille
# MT
history-col-status = État
# MT
history-col-duration = Durée
# MT
history-col-error = Erreur

# MT
toast-history-exported = Historique exporté
# MT
toast-history-rerun-queued = Relance en file d'attente

# MT — Totals drawer (Phase 10)
footer-totals = Totaux
# MT
totals-title = Totaux
# MT
totals-loading = Chargement des totaux…
# MT
totals-card-bytes = Octets copiés au total
# MT
totals-card-files = Fichiers
# MT
totals-card-jobs = Tâches
# MT
totals-card-avg-rate = Débit moyen
# MT
totals-errors = erreurs
# MT
totals-spark-title = 30 derniers jours
# MT
totals-kinds-title = Par type
# MT
totals-saved-title = Temps gagné (estimé)
# MT
totals-saved-note = Estimé par rapport à une copie de référence du même contenu avec un gestionnaire de fichiers standard.
# MT
totals-reset = Réinitialiser les statistiques
# MT
totals-reset-confirm = Ceci supprime toutes les tâches et éléments stockés. Continuer ?
# MT
totals-reset-confirm-yes = Oui, réinitialiser
# MT
toast-totals-reset = Statistiques réinitialisées

# MT — Phase 11a additions
header-language-label = Langue
# MT
header-language-title = Changer de langue

# MT
kind-copy = Copier
# MT
kind-move = Déplacer
# MT
kind-delete = Supprimer
# MT
kind-secure-delete = Suppression sécurisée

# MT
status-running = En cours
# MT
status-succeeded = Terminé
# MT
status-failed = Échec
# MT
status-cancelled = Annulé
# MT
status-ok = OK
# MT
status-skipped = Ignoré

# MT
history-search-placeholder = /chemin
# MT
toast-history-purged = { $count } tâches de plus de 30 jours supprimées

# MT
err-source-required = Au moins un chemin source est requis.
# MT
err-destination-empty = Le chemin de destination est vide.
# MT
err-source-empty = Le chemin source est vide.

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
settings-title = Paramètres
# MT
settings-tab-general = Général
# MT
settings-tab-appearance = Apparence
# MT
settings-section-language = Langue
# MT
settings-phase-12-hint = D'autres paramètres (thème, valeurs par défaut de transfert, algorithme de vérification, profils) arrivent en phase 12.

# MT — Phase 12 Settings window
settings-loading = Chargement des paramètres…
# MT
settings-tab-transfer = Transfert
# MT
settings-tab-shell = Shell
# MT
settings-tab-secure-delete = Suppression sécurisée
# MT
settings-tab-advanced = Avancé
# MT
settings-tab-profiles = Profils

# MT
settings-section-theme = Thème
# MT
settings-theme-auto = Automatique
# MT
settings-theme-light = Clair
# MT
settings-theme-dark = Sombre
# MT
settings-start-with-os = Lancer au démarrage du système
# MT
settings-single-instance = Instance unique
# MT
settings-minimize-to-tray = Réduire dans la zone de notification à la fermeture
settings-error-display-mode = Style d'invite d'erreur
settings-error-display-modal = Modale (bloque l'application)
settings-error-display-drawer = Panneau latéral (non bloquant)
settings-error-display-mode-hint = La modale suspend la file d'attente jusqu'à votre décision. Le panneau latéral maintient la file active et permet de trier les erreurs dans le coin.
settings-paste-shortcut = Coller des fichiers via un raccourci global
settings-paste-shortcut-combo = Combinaison du raccourci
settings-paste-shortcut-hint = Appuyez sur cette combinaison n'importe où dans le système pour coller des fichiers copiés depuis l'Explorateur / Finder / Fichiers via Copy That. CmdOrCtrl se résout en Cmd sur macOS et en Ctrl sur Windows / Linux.
settings-clipboard-watcher = Surveiller le presse-papiers pour les fichiers copiés
settings-clipboard-watcher-hint = Affiche une notification lorsque des URLs de fichiers apparaissent dans le presse-papiers, suggérant que vous pouvez coller via Copy That. Scrute toutes les 500 ms lorsque activé.

# MT
settings-buffer-size = Taille du tampon
# MT
settings-verify = Vérifier après la copie
# MT
settings-verify-off = Désactivé
# MT
settings-concurrency = Concurrence
# MT
settings-concurrency-auto = Automatique
# MT
settings-reflink = Reflink / chemins rapides
# MT
settings-reflink-prefer = Préférer
# MT
settings-reflink-avoid = Éviter reflink
# MT
settings-reflink-disabled = Toujours utiliser le moteur asynchrone
# MT
settings-fsync-on-close = Synchroniser sur le disque à la fermeture (plus lent, plus sûr)
# MT
settings-preserve-timestamps = Conserver les horodatages
# MT
settings-preserve-permissions = Conserver les permissions
# MT
settings-preserve-acls = Conserver les ACL (phase 14)
settings-preserve-sparseness = Préserver les fichiers clairsemés  # MT
settings-preserve-sparseness-hint = Copier uniquement les étendues allouées des fichiers clairsemés (disques de VM, fichiers de base de données) pour que la destination conserve la même taille sur disque que la source.  # MT

# MT
settings-context-menu = Activer les entrées du menu contextuel
# MT
settings-intercept-copy = Intercepter le gestionnaire de copie par défaut (Windows)
# MT
settings-intercept-copy-hint = Quand activé, Ctrl+C / Ctrl+V dans l'Explorateur passe par Copy That. Enregistrement en phase 14.
# MT
settings-notify-completion = Notifier à la fin de la tâche

# MT
settings-shred-method = Méthode de destruction par défaut
# MT
settings-shred-zero = Zéro (1 passe)
# MT
settings-shred-random = Aléatoire (1 passe)
# MT
settings-shred-dod3 = DoD 5220.22-M (3 passes)
# MT
settings-shred-dod7 = DoD 5220.22-M (7 passes)
# MT
settings-shred-gutmann = Gutmann (35 passes)
# MT
settings-shred-nist = NIST 800-88
# MT
settings-shred-confirm-twice = Exiger une double confirmation avant destruction

# MT
settings-log-level = Niveau de journalisation
# MT
settings-log-off = Désactivé
# MT
settings-telemetry = Télémétrie
# MT
settings-telemetry-never = Jamais — aucun envoi de données quel que soit le niveau
# MT
settings-error-policy = Politique d'erreur par défaut
# MT
settings-error-policy-ask = Demander
# MT
settings-error-policy-skip = Ignorer
# MT
settings-error-policy-retry = Réessayer avec délai
# MT
settings-error-policy-abort = Annuler à la première erreur
# MT
settings-history-retention = Conservation de l'historique (jours)
# MT
settings-history-retention-hint = 0 = conserver indéfiniment. Toute autre valeur purge les anciennes tâches au démarrage.
# MT
settings-database-path = Chemin de la base de données
# MT
settings-database-path-default = (par défaut — répertoire de données du système)
# MT
settings-reset-all = Réinitialiser les valeurs par défaut
# MT
settings-reset-confirm = Réinitialiser toutes les préférences ? Les profils ne sont pas affectés.

# MT
settings-profiles-hint = Enregistrez les paramètres actuels sous un nom ; rechargez-les pour basculer sans toucher à chaque réglage.
# MT
settings-profile-name-placeholder = Nom du profil
# MT
settings-profile-save = Enregistrer
# MT
settings-profile-import = Importer…
# MT
settings-profile-load = Charger
# MT
settings-profile-export = Exporter…
# MT
settings-profile-delete = Supprimer
# MT
settings-profile-empty = Aucun profil enregistré.
# MT
settings-profile-import-prompt = Nom pour le profil importé :

# MT
toast-settings-reset = Paramètres réinitialisés
# MT
toast-profile-saved = Profil enregistré
# MT
toast-profile-loaded = Profil chargé
# MT
toast-profile-exported = Profil exporté
# MT
toast-profile-imported = Profil importé

# Phase 13d — activity feed + header picker buttons
action-add-files = Ajouter des fichiers
action-add-folders = Ajouter des dossiers
activity-title = Activité
activity-clear = Vider la liste d'activité
activity-empty = Aucune activité pour le moment.
activity-after-done = Une fois terminé :
activity-keep-open = Garder l'application ouverte
activity-close-app = Quitter l'application
activity-shutdown = Éteindre le PC
activity-logoff = Se déconnecter
activity-sleep = Mettre en veille

# Phase 14 — preflight free-space dialog
preflight-block-title = Espace insuffisant sur la destination
preflight-warn-title = Espace faible sur la destination
preflight-unknown-title = Espace libre indéterminé
preflight-unknown-body = La source est trop volumineuse pour être mesurée rapidement ou le volume de destination n'a pas répondu. Vous pouvez continuer ; le garde-fou du moteur arrêtera proprement la copie si l'espace vient à manquer.
preflight-required = Requis
preflight-free = Libre
preflight-reserve = Réserve
preflight-shortfall = Déficit
preflight-continue = Continuer quand même
collision-modal-overwrite-older = Écraser uniquement les plus anciens

# Phase 14e — subset picker
preflight-pick-subset = Choisir ce qui sera copié…
subset-title = Sélectionnez les sources à copier
subset-subtitle = La sélection complète ne rentre pas sur la destination. Cochez ce que vous voulez copier ; le reste est ignoré.
subset-loading = Mesure des tailles…
subset-too-large = trop volumineux à compter
subset-budget = Disponible
subset-remaining = Restant
subset-confirm = Copier la sélection
history-rerun-hint = Relancer cette copie — analyse à nouveau tous les fichiers de la source
history-clear-all = Tout effacer
history-clear-all-confirm = Cliquez à nouveau pour confirmer
history-clear-all-hint = Supprime toutes les lignes de l'historique. Un deuxième clic confirme.
toast-history-cleared = Historique effacé ({ $count } lignes supprimées)

# Phase 15 — source-list ordering
drop-dialog-sort-label = Ordre :
sort-custom = Personnalisé
sort-name-asc = Nom A → Z (fichiers d'abord)
sort-name-desc = Nom Z → A (fichiers d'abord)
sort-size-asc = Taille croissante (fichiers d'abord)
sort-size-desc = Taille décroissante (fichiers d'abord)
sort-reorder = Réorganiser
sort-move-top = Mettre tout en haut
sort-move-up = Monter
sort-move-down = Descendre
sort-move-bottom = Mettre tout en bas
sort-name-asc-simple = Nom A → Z
sort-name-desc-simple = Nom Z → A
sort-size-asc-simple = Plus petits d'abord
sort-size-desc-simple = Plus grands d'abord
activity-sort-locked = Le tri est désactivé pendant qu'une copie est en cours. Mets en pause ou attends la fin, puis change l'ordre.
drop-dialog-collision-label = Si un fichier existe déjà :
collision-policy-keep-both = Conserver les deux (renommer la nouvelle copie en _2, _3 …)
collision-policy-skip = Ignorer la nouvelle copie
collision-policy-overwrite = Écraser le fichier existant
collision-policy-overwrite-if-newer = Écraser uniquement si plus récent
collision-policy-prompt = Demander à chaque fois
drop-dialog-busy-checking = Vérification de l'espace libre…
drop-dialog-busy-enumerating = Comptage des fichiers…
drop-dialog-busy-starting = Démarrage de la copie…
toast-enumeration-deferred = L'arborescence source est volumineuse — liste préalable ignorée ; les lignes apparaîtront au fur et à mesure du traitement.

# Phase 14a — enumeration-time filters
# MT
settings-tab-filters = Filtres
# MT
settings-filters-hint = Ignore les fichiers lors de l'énumération, avant même que le moteur ne les ouvre. Les inclusions s'appliquent aux fichiers seuls ; les exclusions élaguent aussi les dossiers correspondants.
# MT
settings-filters-enabled = Activer les filtres pour les copies d'arborescence
# MT
settings-filters-include-globs = Globs d'inclusion
# MT
settings-filters-include-globs-placeholder = **/*.txt
# MT
settings-filters-include-globs-hint = Un glob par ligne. Si non vide, un fichier doit correspondre à au moins un. Les dossiers sont toujours parcourus.
# MT
settings-filters-exclude-globs = Globs d'exclusion
# MT
settings-filters-exclude-globs-placeholder = **/node_modules
# MT
settings-filters-exclude-globs-hint = Un glob par ligne. Les correspondances élaguent toute la sous-arborescence pour les dossiers ; les fichiers correspondants sont ignorés.
# MT
settings-filters-size-range = Plage de taille de fichier
# MT
settings-filters-min-size-bytes = Taille minimale (octets, vide = aucune limite basse)
# MT
settings-filters-max-size-bytes = Taille maximale (octets, vide = aucune limite haute)
# MT
settings-filters-date-range = Plage de date de modification
# MT
settings-filters-min-mtime = Modifié à partir du
# MT
settings-filters-max-mtime = Modifié jusqu'au
# MT
settings-filters-attributes = Attributs
# MT
settings-filters-skip-hidden = Ignorer les fichiers / dossiers cachés
# MT
settings-filters-skip-system = Ignorer les fichiers système (Windows uniquement)
# MT
settings-filters-skip-readonly = Ignorer les fichiers en lecture seule

# Phase 15 — auto-update
# MT
settings-tab-updater = Mises à jour
# MT
settings-updater-hint = Copy That recherche des mises à jour signées une fois par jour au maximum. Les mises à jour s'installent à la prochaine fermeture de l'application.
# MT
settings-updater-auto-check = Rechercher les mises à jour au lancement
# MT
settings-updater-channel = Canal de publication
# MT
settings-updater-channel-stable = Stable
# MT
settings-updater-channel-beta = Bêta (préversion)
# MT
settings-updater-last-check = Dernière vérification
# MT
settings-updater-last-never = Jamais
# MT
settings-updater-check-now = Rechercher des mises à jour maintenant
# MT
settings-updater-checking = Vérification…
# MT
settings-updater-available = Mise à jour disponible
# MT
settings-updater-up-to-date = Vous utilisez la dernière version.
# MT
settings-updater-dismiss = Ignorer cette version
# MT
settings-updater-dismissed = Ignorée
# MT
toast-update-available = Une version plus récente est disponible
# MT
toast-update-up-to-date = Vous êtes déjà à la dernière version

# Phase 19a — disk-backed file enumeration (TeraCopy-compatible scan DB)
# MT
scan-progress-title = Analyse en cours…
# MT
scan-progress-stats = { $files } fichiers · { $bytes } à ce stade
# MT
scan-pause-button = Mettre l'analyse en pause
# MT
scan-resume-button = Reprendre l'analyse
# MT
scan-cancel-button = Annuler l'analyse
# MT
scan-cancel-confirm = Annuler l'analyse et abandonner la progression ?
# MT
scan-db-header = Base de données d'analyse
# MT
scan-db-hint = Base de données d'analyse sur disque pour les tâches de plusieurs millions de fichiers.
# MT
advanced-scan-hash-during = Calculer les sommes de contrôle pendant l'analyse
# MT
advanced-scan-db-path = Emplacement de la base de données d'analyse
# MT
advanced-scan-retention-days = Supprimer automatiquement les analyses terminées après (jours)
# MT
advanced-scan-max-keep = Nombre maximal de bases de données d'analyse à conserver

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
sparse-not-supported-title = La destination remplit les fichiers clairsemés  # MT
sparse-not-supported-body = { $dst_fs } ne prend pas en charge les fichiers clairsemés. Les trous de la source ont été écrits sous forme de zéros, donc la destination est plus grande sur disque.  # MT
sparse-warning-densified = Disposition clairsemée préservée : seules les étendues allouées ont été copiées.  # MT
sparse-warning-mismatch = Incompatibilité de disposition clairsemée — la destination peut être plus grande que prévu.  # MT

# Phase 24 — security-metadata preservation. MT-flagged drafts;
# the authoritative English source lives in locales/en/copythat.ftl.
settings-preserve-security-metadata = Préserver les métadonnées de sécurité  # MT
settings-preserve-security-metadata-hint = Capturer et réappliquer les flux de métadonnées hors-bande (NTFS ADS / xattrs / ACL POSIX / contextes SELinux / capacités de fichier Linux / forks de ressources macOS) à chaque copie.  # MT
settings-preserve-motw = Préserver la Marque du Web (indicateur de téléchargement Internet)  # MT
settings-preserve-motw-hint = Critique pour la sécurité. SmartScreen et Office Protected View utilisent ce flux pour avertir des fichiers téléchargés depuis Internet. Désactiver permet à un exécutable téléchargé de perdre son marqueur d'origine lors de la copie et de contourner les protections du système d'exploitation.  # MT
settings-preserve-posix-acls = Préserver les ACL POSIX et attributs étendus  # MT
settings-preserve-posix-acls-hint = Transporter les xattrs user.* / system.* / trusted.* et les listes de contrôle d'accès POSIX lors de la copie.  # MT
settings-preserve-selinux = Préserver les contextes SELinux  # MT
settings-preserve-selinux-hint = Transporter l'étiquette security.selinux lors de la copie pour que les démons sous politiques MAC puissent accéder au fichier.  # MT
settings-preserve-resource-forks = Préserver les forks de ressources macOS et Finder info  # MT
settings-preserve-resource-forks-hint = Transporter le fork de ressources hérité et FinderInfo (étiquettes de couleur, métadonnées Carbon) lors de la copie.  # MT
settings-appledouble-fallback = Utiliser un fichier annexe AppleDouble sur les systèmes de fichiers incompatibles  # MT
meta-translated-to-appledouble = Métadonnées étrangères stockées dans le fichier annexe AppleDouble (._{ $ext })  # MT

# Phase 25 — two-way sync with vector-clock conflict detection.
# MT-flagged drafts; the authoritative English source lives in
# locales/en/copythat.ftl.
footer-sync = Sync  # MT
sync-drawer-title = Synchronisation bidirectionnelle  # MT
sync-drawer-hint = Gardez deux dossiers synchronisés sans écrasements silencieux. Les modifications concurrentes apparaissent comme conflits résolubles.  # MT
sync-add-pair = Ajouter une paire  # MT
sync-add-cancel = Annuler  # MT
sync-refresh = Actualiser  # MT
sync-add-save = Enregistrer la paire  # MT
sync-add-saving = Enregistrement…  # MT
sync-add-missing-fields = L'étiquette, le chemin gauche et le chemin droit sont tous obligatoires.  # MT
sync-remove-confirm = Supprimer cette paire de synchronisation ? La base de données d'état est conservée ; les dossiers restent intacts.  # MT
sync-field-label = Étiquette  # MT
sync-field-label-placeholder = p. ex. Documents ↔ NAS  # MT
sync-field-left = Dossier gauche  # MT
sync-field-left-placeholder = Choisissez ou collez un chemin absolu  # MT
sync-field-right = Dossier droit  # MT
sync-field-right-placeholder = Choisissez ou collez un chemin absolu  # MT
sync-field-mode = Mode  # MT
sync-mode-two-way = Bidirectionnel  # MT
sync-mode-mirror-left-to-right = Miroir (gauche → droite)  # MT
sync-mode-mirror-right-to-left = Miroir (droite → gauche)  # MT
sync-mode-contribute-left-to-right = Contribuer (gauche → droite, sans suppression)  # MT
sync-no-pairs = Aucune paire de synchronisation configurée. Cliquez sur "Ajouter une paire" pour commencer.  # MT
sync-loading = Chargement des paires configurées…  # MT
sync-never-run = Jamais exécuté  # MT
sync-running = En cours  # MT
sync-run-now = Exécuter maintenant  # MT
sync-cancel = Annuler  # MT
sync-remove-pair = Supprimer  # MT
sync-view-conflicts = Voir les conflits ({ $count })  # MT
sync-conflicts-heading = Conflits  # MT
sync-no-conflicts = Aucun conflit lors de la dernière exécution.  # MT
sync-winner = Vainqueur  # MT
sync-side-left-to-right = gauche  # MT
sync-side-right-to-left = droite  # MT
sync-conflict-kind-concurrent-write = Édition concurrente  # MT
sync-conflict-kind-delete-edit = Suppression ↔ édition  # MT
sync-conflict-kind-add-add = Les deux côtés ont ajouté  # MT
sync-conflict-kind-corrupt-equal = Le contenu a divergé sans nouvelle écriture  # MT
sync-resolve-keep-left = Conserver gauche  # MT
sync-resolve-keep-right = Conserver droite  # MT
sync-resolve-keep-both = Conserver les deux  # MT
sync-resolve-three-way = Résoudre via fusion à 3 voies  # MT
sync-resolve-phase-53-tooltip = La fusion interactive à 3 voies pour les fichiers non textuels arrive en Phase 53.  # MT
sync-error-prefix = Erreur de synchronisation  # MT

# Phase 26 — real-time mirror watcher. MT-flagged drafts;
# the authoritative English source lives in locales/en/copythat.ftl.
live-mirror-start = Démarrer le miroir en direct  # MT
live-mirror-stop = Arrêter le miroir en direct  # MT
live-mirror-watching = Surveillance  # MT
live-mirror-toggle-hint = Resynchronise automatiquement à chaque changement du système de fichiers détecté. Un thread d'arrière-plan par paire active.  # MT
watch-event-prefix = Changement de fichier  # MT
watch-overflow-recovered = Débordement du tampon du surveillant ; réénumération pour récupérer  # MT

# Phase 27 — content-defined chunk store. MT-flagged drafts;
# the authoritative English source lives in locales/en/copythat.ftl.
chunk-store-section = Magasin de blocs  # MT
chunk-store-enable = Activer le magasin de blocs (reprise delta et déduplication)  # MT
chunk-store-enable-hint = Divise chaque fichier copié par contenu (FastCDC) et stocke les blocs par adresse de contenu. Les tentatives de reprise ne réécrivent que les blocs modifiés ; les fichiers avec du contenu partagé sont dédupliqués automatiquement.  # MT
chunk-store-location = Emplacement du magasin de blocs  # MT
chunk-store-max-size = Taille maximale du magasin de blocs  # MT
chunk-store-prune = Élaguer les blocs plus anciens que (jours)  # MT
chunk-store-savings = Économisé { $gib } Gio via la déduplication de blocs  # MT
chunk-store-disk-usage = Utilise { $size } sur { $chunks } blocs  # MT

# Phase 28 — tray-resident Drop Stack. MT-flagged drafts;
# the authoritative English source lives in locales/en/copythat.ftl.
dropstack-window-title = Pile de glisser  # MT
dropstack-tray-open = Pile de glisser  # MT
dropstack-empty-title = La pile de glisser est vide  # MT
dropstack-empty-hint = Faites glisser des fichiers ici depuis l'Explorateur ou cliquez-droit sur une ligne de tâche pour l'ajouter.  # MT
dropstack-add-to-stack = Ajouter à la pile de glisser  # MT
dropstack-copy-all-to = Tout copier vers…  # MT
dropstack-move-all-to = Tout déplacer vers…  # MT
dropstack-clear = Vider la pile  # MT
dropstack-remove-row = Retirer de la pile  # MT
dropstack-path-missing-toast = { $path } retiré — le fichier n'existe plus.  # MT
dropstack-always-on-top = Garder la pile de glisser toujours au premier plan  # MT
dropstack-show-tray-icon = Afficher l'icône Copy That dans la barre des tâches  # MT
dropstack-open-on-start = Ouvrir la pile de glisser au démarrage  # MT
dropstack-count = { $count } chemin  # MT
