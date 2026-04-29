app-name = Copy That v0.19.84
# MT
window-title = Copy That v0.19.84
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
err-path-invalid-encoding = Path rejected — string contains invalid UTF-8 / replacement characters
# MT
err-helper-invalid-json = Privileged helper received malformed JSON; ignoring this request
err-helper-grant-out-of-band = GrantCapabilities must be handled by the helper run-loop, not the stateless handler
err-randomness-unavailable = OS random-number generator failed; cannot mint a session id
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
phase42-paranoid-verify-label = Vérification paranoïaque
phase42-paranoid-verify-hint = Vide les pages mises en cache de la destination et relit depuis le disque pour détecter les mensonges du cache d'écriture et la corruption silencieuse. Environ 50 % plus lent que la vérification par défaut ; désactivé par défaut.
phase42-sharing-violation-retries-label = Tentatives de relance sur les fichiers source verrouillés
phase42-sharing-violation-retries-hint = Nombre de tentatives lorsqu'un autre processus maintient le fichier source ouvert avec un verrou exclusif. Le délai double à chaque tentative (50 ms / 100 ms / 200 ms par défaut). Valeur par défaut : 3, comme Robocopy /R:3.
phase42-cloud-placeholder-warning = { $name } est un fichier OneDrive uniquement dans le cloud. Le copier déclenchera un téléchargement — jusqu'à { $size } via votre connexion réseau.
phase42-defender-exclusion-hint = Pour un débit de copie maximal, ajoutez le dossier de destination aux exclusions de Microsoft Defender avant les transferts en masse. Voir docs/PERFORMANCE_TUNING.md.

# Phase 39 — Browser-accessible recovery UI.  # MT
settings-recovery-heading = Recovery web UI  # MT
settings-recovery-enable = Enable recovery web UI  # MT
settings-recovery-bind-address = Bind address  # MT
settings-recovery-port = Port (0 = pick a free one)  # MT
settings-recovery-show-url = Show URL & token  # MT
settings-recovery-rotate-token = Rotate token  # MT
settings-recovery-allow-non-loopback = Allow non-loopback bind  # MT
settings-recovery-non-loopback-warning = WARNING: enabling a non-loopback bind exposes the recovery UI to your local network. Anyone who learns the token can browse your file history and download files. Front it with TLS or a reverse proxy if the LAN is untrusted.  # MT

# Phase 40 — SMB compression negotiation + cloud-VM offload helper.  # MT
smb-compress-badge = 🗜 SMB compress: { $algo }  # MT
smb-compress-badge-tooltip = Network traffic to this destination is being compressed in transit (SMB 3.1.1).  # MT
smb-compress-toast-saved = Saved { $bytes } over the network  # MT
smb-compress-algo-unknown = unknown algorithm  # MT
settings-smb-compress-heading = SMB network compression  # MT
settings-smb-compress-hint = Automatically negotiate SMB 3.1.1 traffic compression on UNC destinations. Free win on slow links; ignored on local destinations.  # MT
cloud-offload-heading = Cloud-VM offload helper  # MT
cloud-offload-hint = When copying directly between two clouds, render a deployment template that runs the copy from a tiny ephemeral VM in the cloud — bytes never touch your laptop's network.  # MT
cloud-offload-render-button = Render template  # MT
cloud-offload-copy-clipboard = Copy to clipboard  # MT
cloud-offload-template-format = Template format  # MT
cloud-offload-self-destruct-warning = The VM auto-shuts down after { $minutes } minutes — confirm IAM role + region before deploying.  # MT

# Phase 41 — animated before/after tree-diff preview.  # MT
preview-modal-title = Preview changes  # MT
preview-summary-header = What will happen  # MT
preview-category-additions = { $count } additions  # MT
preview-category-replacements = { $count } replacements  # MT
preview-category-skips = { $count } skipped  # MT
preview-category-conflicts = { $count } conflicts  # MT
preview-category-unchanged = { $count } unchanged  # MT
preview-bytes-to-transfer = { $bytes } to transfer  # MT
preview-reason-source-newer = Source is newer  # MT
preview-reason-dest-newer = Destination is newer — will skip  # MT
preview-reason-content-different = Content differs  # MT
preview-reason-identical = Identical to source  # MT
preview-button-run = Run plan  # MT
preview-button-reduce = Reduce my plan…  # MT

# Phase 42 — perceptual-hash visual-similarity dedup.  # MT
perceptual-warn-title = Looks visually identical  # MT
perceptual-warn-body = { $name } at the destination appears to match the source picture. Continue copying anyway?  # MT
perceptual-warn-keep-both = Keep both  # MT
perceptual-warn-skip = Skip this file  # MT
perceptual-warn-overwrite = Overwrite anyway  # MT
perceptual-settings-heading = Visual-similarity dedup  # MT
perceptual-settings-hint = Detect visually identical images at the destination before they're overwritten. Hash is perceptual (recognises the same picture re-saved as a different format), not byte-exact.  # MT
perceptual-settings-threshold-label = Warn threshold (lower = stricter match)  # MT

# Phase 42 Part B — per-file rolling versions.  # MT
version-list-heading = Previous versions  # MT
version-list-empty = No prior versions of this file  # MT
version-list-restore = Restore this version  # MT
version-retention-heading = Keep previous versions on overwrite  # MT
version-retention-none = Keep every version forever  # MT
version-retention-last-n = Keep last { $n } versions  # MT
version-retention-older-than-days = Drop versions older than { $days } days  # MT
version-retention-gfs = Hourly { $h } · daily { $d } · weekly { $w } · monthly { $m }  # MT

# Phase 43 — forensic chain-of-custody manifests.  # MT
provenance-settings-heading = Forensic chain-of-custody  # MT
provenance-settings-hint = Sign every copy job with a BLAKE3 + ed25519 manifest. Reviewers can re-hash the destination tree later and prove no byte changed since the copy.  # MT
provenance-settings-enable-default = Sign every new job by default  # MT
provenance-settings-show-after-job = Show manifest after each completed job  # MT
provenance-settings-tsa-url-label = Default RFC 3161 timestamp authority URL  # MT
provenance-settings-tsa-url-hint = Optional. When set, manifests carry a free TSA timestamp proving the bytes existed at this point in time. Leave empty to skip.  # MT
provenance-settings-keys-heading = Signing keys  # MT
provenance-settings-keys-generate = Generate new key  # MT
provenance-settings-keys-import = Import key…  # MT
provenance-settings-keys-export = Export public key…  # MT
provenance-job-completed-title = Provenance manifest saved  # MT
provenance-job-completed-body = { $count } files signed → { $path }  # MT
provenance-verify-clean = Manifest valid for { $count } files; signature { $sig }; merkle root OK.  # MT
provenance-verify-tampered = Manifest INVALID — { $tampered } tampered, { $missing } missing.  # MT
provenance-action-staged = Phase 43 — wiring the IPC for this action lands in a follow-up commit.  # MT

# Phase 44 — SSD-aware whole-drive sanitize.  # MT
sanitize-heading = Whole-drive secure sanitize  # MT
sanitize-hint = NVMe Sanitize, OPAL Crypto Erase, and ATA Secure Erase wipe a flash drive at the firmware layer in milliseconds. Per-file overwrite is meaningless on flash — multi-pass shred only burns NAND. Use this for actual purge.  # MT
sanitize-pick-device = Choose the drive to sanitize  # MT
sanitize-mode-label = Sanitization method  # MT
sanitize-mode-nvme-format = NVMe Format (with secure erase)  # MT
sanitize-mode-nvme-sanitize-block = NVMe Sanitize — Block Erase (slow, every cell)  # MT
sanitize-mode-nvme-sanitize-crypto = NVMe Sanitize — Crypto Erase (instant)  # MT
sanitize-mode-ata-secure-erase = ATA Secure Erase (legacy SATA SSDs)  # MT
sanitize-mode-opal-crypto-erase = TCG OPAL Crypto Erase (Self-Encrypting Drives)  # MT
sanitize-confirm-1 = This destroys EVERY byte on { $device }. There is no undo.  # MT
sanitize-confirm-2 = I understand that all partitions, all files, and all snapshots on { $device } will be permanently unreadable.  # MT
sanitize-confirm-3 = Type the drive's model name to proceed: { $model }  # MT
sanitize-running = Sanitizing { $device } ({ $mode }) — this can take from milliseconds (crypto erase) to tens of minutes (block erase). Do not power down.  # MT
sanitize-completed = Sanitize complete — { $device } is now blank.  # MT
ssd-honest-shred-meaningless = Per-file shred on a copy-on-write filesystem (Btrfs / ZFS / APFS) cannot reach the underlying blocks. Use whole-drive sanitize plus full-disk-encryption key rotation instead.  # MT
ssd-honest-advisory = This file lives on flash. Per-file overwrite costs NAND wear and does NOT guarantee the original cells are unrecoverable. For sensitive data, sanitize the whole drive.  # MT
