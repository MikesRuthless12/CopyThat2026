app-name = Freally File Manager v0.19.85
window-title = Freally File Manager v0.19.85
shred-ssd-advisory = Attention : cette cible se trouve sur un SSD. Les réécritures multipasses ne nettoient pas la mémoire flash de façon fiable, car le nivellement d'usure et le surprovisionnement déplacent les données hors de l'adresse de bloc logique. Pour les supports SSD, préférez ATA SECURE ERASE, NVMe Format avec Secure Erase ou le chiffrement intégral du disque avec destruction de la clé.

# Global aggregate states (header pill)
state-idle = Inactif
state-copying = Copie en cours
state-verifying = Vérification
state-paused = En pause
state-error = Erreur

# Per-job states (row badge)
state-pending = En file d'attente
state-running = En cours
state-cancelled = Annulé
state-succeeded = Terminé
state-failed = Échec

# Actions
action-pause = Pause
action-resume = Reprendre
action-cancel = Annuler
action-pause-all = Mettre en pause toutes les tâches
action-resume-all = Reprendre toutes les tâches
action-cancel-all = Annuler toutes les tâches
action-close = Fermer
action-reveal = Afficher dans le dossier
action-add-files = Ajouter des fichiers
action-add-folders = Ajouter des dossiers

# Phase 13d — activity feed
activity-title = Activité
activity-clear = Effacer la liste d'activité
activity-empty = Aucune activité de fichier pour l'instant.
activity-after-done = Une fois terminé :
activity-keep-open = Garder l'application ouverte
activity-close-app = Fermer l'application
activity-shutdown = Éteindre le PC
activity-logoff = Se déconnecter
activity-sleep = Mettre en veille

# Phase 14 — preflight free-space dialog
preflight-block-title = Espace insuffisant sur la destination
preflight-warn-title = Espace faible sur la destination
preflight-unknown-title = Impossible de déterminer l'espace libre
preflight-unknown-body = La source est trop volumineuse pour être mesurée rapidement, ou le volume de destination n'a pas répondu. Vous pouvez continuer ; la protection d'espace du moteur arrêtera proprement la copie en cas de manque de place.
preflight-required = Requis
preflight-free = Libre
preflight-reserve = Réserve
preflight-shortfall = Déficit
preflight-continue = Continuer quand même
preflight-pick-subset = Choisir les éléments à copier…
collision-modal-overwrite-older = Remplacer uniquement les plus anciens

# Phase 14e — subset picker
subset-title = Choisir les sources à copier
subset-subtitle = La sélection complète ne tient pas sur la destination. Cochez les éléments à copier ; les autres seront ignorés.
subset-loading = Mesure des tailles…
subset-too-large = trop volumineux à mesurer
subset-budget = Disponible
subset-remaining = Restant
subset-confirm = Copier la sélection
history-rerun-hint = Relancer cette copie — réanalyse chaque fichier de l'arborescence source
history-clear-all = Tout effacer
history-clear-all-confirm = Cliquez à nouveau pour confirmer
history-clear-all-hint = Supprime toutes les lignes de l'historique. Nécessite un second clic pour confirmer.
toast-history-cleared = Historique effacé ({ $count } lignes supprimées)

# Phase 15 — source-list ordering in the drop staging dialog
drop-dialog-sort-label = Ordre :
sort-custom = Personnalisé
sort-name-asc = Nom A → Z (fichiers d'abord)
sort-name-desc = Nom Z → A (fichiers d'abord)
sort-size-asc = Taille croissante (fichiers d'abord)
sort-size-desc = Taille décroissante (fichiers d'abord)
sort-reorder = Réorganiser
sort-move-top = Placer en haut
sort-move-up = Monter
sort-move-down = Descendre
sort-move-bottom = Placer en bas

# Phase 16 — sort preset names for the Activity list (shorter
# wording than the DropStagingDialog variants; the Activity list
# is files-only so the "(files first)" suffix isn't relevant).
sort-name-asc-simple = Nom A → Z
sort-name-desc-simple = Nom Z → A
sort-size-asc-simple = Taille croissante
sort-size-desc-simple = Taille décroissante
activity-sort-locked = Le tri est désactivé pendant une copie. Mettez en pause ou attendez la fin, puis modifiez l'ordre.

# Phase 17 — collision-policy picker in the DropStagingDialog
drop-dialog-collision-label = Si un fichier existe déjà :
collision-policy-keep-both = Conserver les deux (renommer la nouvelle copie en _2, _3, …)
collision-policy-skip = Ignorer la nouvelle copie
collision-policy-overwrite = Remplacer le fichier existant
collision-policy-overwrite-if-newer = Remplacer uniquement si plus récent
collision-policy-prompt = Demander à chaque fois

# Phase 18 — progress labels on the DropStagingDialog Start button
drop-dialog-busy-checking = Vérification de l'espace libre…
drop-dialog-busy-enumerating = Comptage des fichiers…
drop-dialog-busy-starting = Démarrage de la copie…
toast-enumeration-deferred = L'arborescence source est volumineuse — la liste des fichiers est ignorée au départ ; les lignes apparaîtront à mesure que le moteur les traite.

# Context menu (per-row right-click)
menu-pause = Pause
menu-resume = Reprendre
menu-cancel = Annuler
menu-remove = Retirer de la file
menu-reveal-source = Afficher la source dans le dossier
menu-reveal-destination = Afficher la destination dans le dossier

# Header / toolbar
header-eta-label = Temps restant estimé
header-toolbar-label = Commandes globales

# Footer
footer-queued = tâches actives
footer-total-bytes = en transit
footer-errors = erreurs
footer-history = Historique

# Empty state
empty-title = Déposez des fichiers ou des dossiers à copier
empty-hint = Faites glisser des éléments sur la fenêtre. Nous demanderons une destination, puis créerons une tâche par source.
empty-region-label = Liste des tâches

# Details drawer
details-drawer-label = Détails de la tâche
details-source = Source
details-destination = Destination
details-state = État
details-bytes = Octets
details-files = Fichiers
details-speed = Vitesse
details-eta = Temps restant
details-error = Erreur

# Drop dialog
drop-dialog-title = Transférer les éléments déposés
drop-dialog-subtitle = { $count } élément(s) prêt(s) à transférer. Choisissez un dossier de destination pour commencer.
drop-dialog-mode = Opération
drop-dialog-copy = Copier
drop-dialog-move = Déplacer
drop-dialog-pick-destination = Choisir une destination
drop-dialog-change-destination = Changer de destination
drop-dialog-start-copy = Démarrer la copie
drop-dialog-start-move = Démarrer le déplacement

# ETA placeholders
eta-calculating = calcul en cours…
eta-unknown = inconnu

# Toast messages
toast-job-done = Transfert terminé
toast-copy-queued = Copie mise en file
toast-move-queued = Déplacement mis en file
toast-error-resolved = Erreur résolue
toast-collision-resolved = Conflit résolu
toast-elevated-unavailable = La nouvelle tentative avec élévation arrive en Phase 17 — pas encore disponible
toast-clipboard-files-detected = Fichiers dans le presse-papiers — appuyez sur votre raccourci coller pour copier via Freally File Manager
toast-clipboard-no-files = Le presse-papiers ne contient aucun fichier à coller
toast-error-log-exported = Journal des erreurs exporté

# Error modal (Phase 8)
error-modal-title = Un transfert a échoué
error-modal-retry = Réessayer
error-modal-retry-elevated = Réessayer avec des autorisations élevées
error-modal-skip = Ignorer
error-modal-skip-all-kind = Ignorer toutes les erreurs de ce type
error-modal-abort = Tout abandonner
error-modal-path-label = Chemin
error-modal-code-label = Code
error-drawer-pending-count = D'autres erreurs en attente
error-drawer-toggle = Réduire ou développer

# Error-kind labels (Phase 8). Source of truth — engine maps each
# `CopyErrorKind` to one of these keys via `localized_key()`.
err-not-found = Fichier introuvable
err-permission-denied = Autorisation refusée
err-disk-full = Le disque de destination est plein
err-interrupted = Opération interrompue
err-verify-failed = La vérification après copie a échoué
err-path-escape = Chemin rejeté — contient des segments de dossier parent (..) ou des octets illégaux
err-path-invalid-encoding = Chemin rejeté — la chaîne contient des caractères UTF-8 / de remplacement invalides
err-helper-invalid-json = L'assistant privilégié a reçu un JSON malformé ; requête ignorée
err-helper-grant-out-of-band = GrantCapabilities doit être traité par la boucle d'exécution de l'assistant, pas par le gestionnaire sans état
err-randomness-unavailable = Le générateur de nombres aléatoires du système a échoué ; impossible de créer un identifiant de session
err-sparseness-mismatch = La disposition creuse n'a pas pu être préservée sur la destination
err-io-other = Erreur d'E/S inconnue

# Collision modal (Phase 8)
collision-modal-title = Le fichier existe déjà
collision-modal-overwrite = Remplacer
collision-modal-overwrite-if-newer = Remplacer si plus récent
collision-modal-skip = Ignorer
collision-modal-keep-both = Conserver les deux
collision-modal-rename = Renommer…
collision-modal-apply-to-all = Appliquer à tout
collision-modal-source = Source
collision-modal-destination = Destination
collision-modal-size = Taille
collision-modal-modified = Modifié
collision-modal-hash-check = Hachage rapide (SHA-256)
collision-modal-hash-computing = Calcul en cours…
collision-modal-hash-identical = Identiques
collision-modal-hash-different = Différents
collision-modal-rename-placeholder = Nouveau nom de fichier
collision-modal-confirm-rename = Renommer

# Error log drawer (Phase 8)
error-log-title = Journal des erreurs
error-log-empty = Aucune erreur enregistrée
error-log-export-csv = Exporter en CSV
error-log-export-txt = Exporter en texte
error-log-clear = Effacer le journal
error-log-col-time = Heure
error-log-col-job = Tâche
error-log-col-path = Chemin
error-log-col-code = Code
error-log-col-message = Message
error-log-col-resolution = Résolution

# History drawer (Phase 9)
history-title = Historique
history-empty = Aucune tâche enregistrée pour l'instant
history-unavailable = L'historique des copies n'est pas disponible. L'application n'a pas pu ouvrir la base SQLite au démarrage.
history-filter-any = tous
history-filter-kind = Type
history-filter-status = Statut
history-filter-text = Rechercher
history-refresh = Actualiser
history-export-csv = Exporter en CSV
history-purge-30 = Purger > 30 jours
history-rerun = Relancer
history-detail-open = Détails
history-detail-title = Détails de la tâche
history-detail-empty = Aucun élément enregistré
history-col-date = Date
history-col-kind = Type
history-col-src = Source
history-col-dst = Destination
history-col-files = Fichiers
history-col-size = Taille
history-col-status = Statut
history-col-duration = Durée
history-col-error = Erreur
toast-history-exported = Historique exporté
toast-history-rerun-queued = Relance mise en file

# Totals drawer (Phase 10)
footer-totals = Totaux
totals-title = Totaux
totals-loading = Chargement des totaux…
totals-card-bytes = Total d'octets copiés
totals-card-files = Fichiers
totals-card-jobs = Tâches
totals-card-avg-rate = Débit moyen
totals-errors = erreurs
totals-spark-title = 30 derniers jours
totals-kinds-title = Par type
totals-saved-title = Temps gagné (estimé)
totals-saved-note = Estimé par rapport à une copie de référence par le gestionnaire de fichiers pour la même charge.
totals-reset = Réinitialiser les statistiques
totals-reset-confirm = Cette action supprime toutes les tâches et tous les éléments enregistrés. Continuer ?
totals-reset-confirm-yes = Oui, réinitialiser
toast-totals-reset = Statistiques réinitialisées

# Phase 11a — i18n core: surface remaining user-visible strings so
# every string on a main-window screen flows through Fluent.

# Header language switcher (temporary placement — folds into
# Settings → General in Phase 12).
header-language-label = Langue
header-language-title = Changer de langue

# Job-kind labels. History filter, Totals breakdown, and history
# rows all map wire-format `kind` strings to these labels.
kind-copy = Copie
kind-move = Déplacement
kind-delete = Suppression
kind-secure-delete = Suppression sécurisée

# History status labels. Distinct from `state-*` because the
# history wire format uses plain `running`/`succeeded`/... rather
# than the live `JobState` enum the queue exposes.
status-running = En cours
status-succeeded = Réussi
status-failed = Échec
status-cancelled = Annulé
# Per-item status (not job-level): `ok` / `skipped` are only
# reachable from the history detail view.
status-ok = OK
status-skipped = Ignoré

# History drawer: search field placeholder + purge toast.
history-search-placeholder = /chemin
toast-history-purged = { $count } tâches de plus de 30 jours purgées

# User-facing command-layer validation errors. The Rust side
# returns these keys when input is missing; the toast layer looks
# them up rather than showing the raw English.
err-source-required = Au moins un chemin source est requis.
err-destination-empty = Le chemin de destination est vide.
err-source-empty = Le chemin source est vide.

# Localised duration formatting for the Totals drawer and ETA
# fields. `{ $ms }`, `{ $s }`, `{ $m }`, `{ $h }` are integer
# placeables — the formatter passes pre-computed values in.
duration-lt-1s = < 1 s
duration-ms = { $ms } ms
duration-seconds = { $s } s
duration-minutes-seconds = { $m } min { $s } s
duration-hours-minutes = { $h } h { $m } min
duration-zero = 0 s

# Rate unit. Appended to a formatted byte size. Some languages
# render this with a leading space ("Ko/s"); keep it translatable
# even though the SI-derived "/s" is near-universal.
rate-unit-per-second = { $size }/s

# Phase 11b — Settings modal skeleton. Phase 12 expanded this into a
# full six-tab preferences window; the `settings-phase-12-hint` key
# is retired in favour of `settings-tab-profiles` + concrete labels.
settings-title = Paramètres
settings-tab-general = Général
settings-tab-appearance = Apparence
settings-section-language = Langue
settings-phase-12-hint = D'autres paramètres (thème, valeurs par défaut de transfert, algorithme de vérification, profils) arrivent en Phase 12.

# Phase 12 — full Settings window keys. Grouped by tab so the
# human-review pass can pick one section at a time. Every field
# label and every option label that the user will actually read
# has its own key; dynamic wire values (enum discriminants like
# "auto" / "prefer") stay untranslated on the wire.

settings-loading = Chargement des paramètres…
settings-tab-transfer = Transfert
settings-tab-filters = Filtres
settings-tab-shell = Shell
settings-tab-secure-delete = Suppression sécurisée
settings-tab-advanced = Avancé
settings-tab-updater = Mises à jour
settings-tab-profiles = Profils

# General tab additions
settings-section-theme = Thème
settings-theme-auto = Auto
settings-theme-light = Clair
settings-theme-dark = Sombre
settings-start-with-os = Lancer au démarrage du système
settings-single-instance = Instance unique en cours d'exécution
settings-minimize-to-tray = Réduire dans la barre d'état à la fermeture
settings-error-display-mode = Style d'invite d'erreur
settings-error-display-modal = Modale (bloque l'application)
settings-error-display-drawer = Panneau (non bloquant)
settings-error-display-mode-hint = La modale arrête la file jusqu'à votre décision. Le panneau garde la file active et vous permet de trier les erreurs dans le coin.
settings-paste-shortcut = Coller des fichiers via un raccourci global
settings-paste-shortcut-combo = Combinaison de touches
settings-paste-shortcut-hint = Appuyez sur cette combinaison n'importe où sur votre système pour coller des fichiers copiés depuis Explorer / Finder / Files via Freally File Manager. CmdOrCtrl correspond à Cmd sur macOS, Ctrl sur Windows / Linux.
settings-clipboard-watcher = Surveiller le presse-papiers pour les fichiers copiés
settings-clipboard-watcher-hint = Affiche une notification lorsque des URL de fichiers apparaissent dans le presse-papiers, suggérant de les coller via Freally File Manager. Interrogation toutes les 500 ms lorsque l'option est activée.

# Transfer tab
settings-buffer-size = Taille de la mémoire tampon
settings-verify = Vérifier après la copie
settings-verify-off = Désactivé
settings-concurrency = Simultanéité
settings-concurrency-auto = Auto
settings-reflink = Reflink / chemins rapides
settings-reflink-prefer = Préférer
settings-reflink-avoid = Éviter le reflink
settings-reflink-disabled = Toujours utiliser le moteur asynchrone
settings-fsync-on-close = Synchroniser sur le disque à la fermeture (plus lent, plus sûr)
settings-preserve-timestamps = Préserver les horodatages
settings-preserve-permissions = Préserver les autorisations
settings-preserve-acls = Préserver les ACL (Phase 14)
settings-preserve-sparseness = Préserver les fichiers creux
settings-preserve-sparseness-hint = Copie uniquement les extents alloués des fichiers creux (disques de VM, fichiers de base de données) afin que la destination conserve la même taille sur disque que la source.
settings-force-parallel-chunks = Copie parallèle multi-blocs (RAID / matrices uniquement)
settings-force-parallel-chunks-hint = Divise chaque copie volumineuse en blocs simultanés. N'aide que pour les destinations en bandes/RAID/réseau ; RALENTIT un SSD/NVMe unique (-25 % à -76 %). Laissez désactivé sauf si la destination est une matrice multidisque.

# Shell tab
settings-context-menu = Activer les entrées du menu contextuel du shell
settings-intercept-copy = Intercepter le gestionnaire de copie par défaut (Windows)
settings-intercept-copy-hint = Lorsque cette option est activée, Ctrl+C / Ctrl+V d'Explorer passe par Freally File Manager. L'enregistrement arrive en Phase 14.
settings-notify-completion = Notifier à la fin d'une tâche

# Secure delete tab
settings-shred-method = Méthode de destruction par défaut
settings-shred-zero = Zéro (1 passe)
settings-shred-random = Aléatoire (1 passe)
settings-shred-dod3 = DoD 5220.22-M (3 passes)
settings-shred-dod7 = DoD 5220.22-M (7 passes)
settings-shred-gutmann = Gutmann (35 passes)
settings-shred-nist = NIST 800-88
settings-shred-confirm-twice = Exiger une double confirmation avant la destruction

# Advanced tab
settings-log-level = Niveau de journalisation
settings-log-off = Désactivé
settings-telemetry = Télémétrie
settings-telemetry-never = Jamais — aucune transmission, quel que soit le niveau de journalisation
settings-error-policy = Politique d'erreur par défaut
settings-error-policy-ask = Demander
settings-error-policy-skip = Ignorer
settings-error-policy-retry = Réessayer avec temporisation
settings-error-policy-abort = Abandonner au premier échec
settings-history-retention = Conservation de l'historique (jours)
settings-history-retention-hint = 0 = conserver indéfiniment. Toute autre valeur purge automatiquement les tâches plus anciennes au démarrage.
settings-database-path = Chemin de la base de données
settings-database-path-default = (par défaut — dossier de données du système)
settings-reset-all = Rétablir les valeurs par défaut
settings-reset-confirm = Rétablir toutes les préférences à leur valeur par défaut ? Les profils ne sont pas affectés.

# Profiles tab
settings-profiles-hint = Enregistrez les paramètres actuels sous un nom ; chargez-les plus tard pour revenir en arrière sans toucher à chaque réglage.
settings-profile-name-placeholder = Nom du profil
settings-profile-save = Enregistrer
settings-profile-import = Importer…
settings-profile-load = Charger
settings-profile-export = Exporter…
settings-profile-delete = Supprimer
settings-profile-empty = Aucun profil enregistré pour l'instant.
settings-profile-import-prompt = Nom du profil importé :

# Toasts driven by Phase 12 profile actions
toast-settings-reset = Paramètres réinitialisés
toast-profile-saved = Profil enregistré
toast-profile-loaded = Profil chargé
toast-profile-exported = Profil exporté
toast-profile-imported = Profil importé

# Phase 14a — enumeration-time filters
settings-filters-hint = Ignore des fichiers au moment de l'énumération pour que le moteur ne les ouvre même pas. Les inclusions s'appliquent uniquement aux fichiers ; les exclusions élaguent aussi les dossiers correspondants.
settings-filters-enabled = Activer les filtres pour les copies d'arborescence
settings-filters-include-globs = Motifs d'inclusion
settings-filters-include-globs-placeholder = **/*.txt
settings-filters-include-globs-hint = Un motif par ligne. Lorsqu'il n'est pas vide, un fichier doit correspondre à au moins une inclusion pour être conservé. Les dossiers sont toujours parcourus.
settings-filters-exclude-globs = Motifs d'exclusion
settings-filters-exclude-globs-placeholder = **/node_modules
settings-filters-exclude-globs-hint = Un motif par ligne. Pour les dossiers, une correspondance élague tout le sous-arbre ; les fichiers correspondants sont ignorés.
settings-filters-size-range = Plage de taille de fichier
settings-filters-min-size-bytes = Taille minimale (octets, vide = aucun minimum)
settings-filters-max-size-bytes = Taille maximale (octets, vide = aucun maximum)
settings-filters-date-range = Plage de date de modification
settings-filters-min-mtime = Modifié le ou après
settings-filters-max-mtime = Modifié le ou avant
settings-filters-attributes = Bits d'attribut
settings-filters-skip-hidden = Ignorer les fichiers / dossiers cachés
settings-filters-skip-system = Ignorer les fichiers système (Windows uniquement)
settings-filters-skip-readonly = Ignorer les fichiers en lecture seule

# Phase 15 — auto-update
settings-updater-hint = Freally File Manager recherche des mises à jour signées au plus une fois par jour. Les mises à jour s'installent au prochain arrêt de l'application.
settings-updater-auto-check = Rechercher les mises à jour au lancement
settings-updater-channel = Canal de publication
settings-updater-channel-stable = Stable
settings-updater-channel-beta = Bêta (préversion)
settings-updater-last-check = Dernière vérification
settings-updater-last-never = Jamais
settings-updater-check-now = Rechercher les mises à jour maintenant
settings-updater-checking = Vérification…
settings-updater-available = Mise à jour disponible
settings-updater-up-to-date = Vous utilisez la dernière version.
settings-updater-dismiss = Ignorer cette version
settings-updater-dismissed = Ignorée
toast-update-available = Une version plus récente est disponible
toast-update-up-to-date = Vous utilisez déjà la dernière version

# Phase 19a — disk-backed file enumeration (TeraCopy-compatible scan DB)
scan-progress-title = Analyse…
scan-progress-stats = { $files } fichiers · { $bytes } jusqu'à présent
scan-pause-button = Mettre en pause l'analyse
scan-resume-button = Reprendre l'analyse
scan-cancel-button = Annuler l'analyse
scan-cancel-confirm = Annuler l'analyse et abandonner la progression ?
scan-db-header = Base de données d'analyse
scan-db-hint = Base de données d'analyse sur disque pour les tâches de plusieurs millions de fichiers.
advanced-scan-hash-during = Calculer les sommes de contrôle pendant l'analyse
advanced-scan-db-path = Emplacement de la base de données d'analyse
advanced-scan-retention-days = Supprimer automatiquement les analyses terminées après (jours)
advanced-scan-max-keep = Nombre maximal de bases de données d'analyse à conserver

# Phase 19b — filesystem-snapshot source for locked files.
settings-on-locked = Lorsqu'un fichier est verrouillé
settings-on-locked-ask = Demander la première fois
settings-on-locked-retry = Réessayer brièvement, puis signaler l'erreur
settings-on-locked-skip = Ignorer le fichier verrouillé
settings-on-locked-snapshot = Utiliser un instantané du système de fichiers
settings-on-locked-hint = Élimine les erreurs « fichier utilisé par un autre processus ». Freally File Manager crée un instantané du volume source (VSS sous Windows, ZFS/Btrfs sous Linux, APFS sous macOS) et lit à partir de la copie de l'instantané.
snapshot-prompt-title = Ce fichier est utilisé par un autre processus
snapshot-prompt-body = Un autre programme a ouvert { $path } en écriture exclusive. Choisissez comment Freally File Manager doit traiter ce fichier et les fichiers similaires sur le même volume.
snapshot-source-active = 📷 Lecture depuis l'instantané { $kind } de { $volume }
snapshot-create-failed = Impossible de créer un instantané du volume source
snapshot-vss-needs-elevation = La lecture depuis un instantané VSS nécessite des droits d'administrateur. Freally File Manager vous demandera l'autorisation.
snapshot-cleanup-failed = L'assistant d'instantané a signalé un échec de nettoyage — une copie shadow résiduelle peut subsister sur le volume.

# Phase 20 — durable resume journal.
resume-prompt-title = Reprendre les transferts précédents ?
resume-prompt-body = Freally File Manager a détecté { $count } transfert(s) inachevé(s) d'une session précédente. Choisissez quoi faire de chacun.
resume-prompt-resume = Reprendre
resume-prompt-resume-all = Tout reprendre
resume-discard-one = Ne pas reprendre
resume-discard-all = Tout abandonner
resume-aborted-hash-mismatch = Les { $offset } premiers octets de la destination ne correspondent pas à la source — redémarrage depuis le début.
settings-auto-resume = Reprendre automatiquement les tâches interrompues sans demander
settings-auto-resume-hint = Ignore l'invite de reprise au démarrage et remet silencieusement en file chaque tâche inachevée. Désactivé par défaut.

# Phase 21 — bandwidth shaping (GCRA token bucket + schedule + auto-throttle).
settings-tab-network = Réseau
settings-network-hint = Limitez votre débit de transfert pour garder le reste du réseau utilisable. Appliquez-le globalement, suivez un programme quotidien ou réagissez automatiquement aux connexions Wi-Fi à consommation limitée / batterie / cellulaires.
settings-network-mode = Limite de bande passante
settings-network-mode-off = Désactivé (aucune limite)
settings-network-mode-fixed = Valeur fixe
settings-network-mode-schedule = Utiliser un programme
settings-network-cap-mbps = Plafond (Mo/s)
settings-network-schedule = Programme (format rclone)
settings-network-schedule-hint = Limites HH:MM,débit séparées par des espaces, plus des règles de jour facultatives Mon-Fri,débit. Débits : 512k, 10M, 2G, off, unlimited. Exemple : 08:00,512k 18:00,10M Sat-Sun,unlimited.
settings-network-auto-header = Limitation automatique
settings-network-auto-metered = Sur Wi-Fi à consommation limitée
settings-network-auto-battery = Sur batterie
settings-network-auto-cellular = Sur réseau cellulaire
settings-network-auto-unchanged = Ne pas modifier
settings-network-auto-pause = Mettre les transferts en pause
settings-network-auto-cap = Plafonner à une valeur fixe
shape-badge-paused = en pause
shape-badge-tooltip = Limite de bande passante active — cliquez pour ouvrir Paramètres → Réseau
shape-badge-source-schedule = programmé
shape-badge-source-metered = consommation limitée
shape-badge-source-battery = sur batterie
shape-badge-source-cellular = cellulaire
shape-badge-source-settings = actif
shape-error-schedule-invalid = Le format du programme n'est pas valide : { $message }

# Phase 22 — aggregate conflict dialog v2 (thumbnails, per-pattern
# rules, and reusable conflict profiles). Every key below is user-
# visible text in the `ConflictBatchModal.svelte` component.
conflict-batch-title = { $count } conflits de fichiers dans { $jobname }
conflict-batch-state-pending = En attente
conflict-batch-state-resolved = Résolu
conflict-batch-action-overwrite = Remplacer
conflict-batch-action-skip = Ignorer
conflict-batch-action-keep-both = Conserver les deux
conflict-batch-action-newer-wins = Le plus récent l'emporte
conflict-batch-action-larger-wins = Le plus volumineux l'emporte
conflict-batch-bulk-apply-selected = Appliquer à la sélection
conflict-batch-bulk-apply-extension = Appliquer à toute cette extension
conflict-batch-bulk-apply-glob = Appliquer au motif correspondant…
conflict-batch-bulk-apply-remaining = Appliquer à tout le reste
conflict-batch-bulk-glob-placeholder = ex. **/*.tmp
conflict-batch-save-profile = Enregistrer ces règles comme profil…
conflict-batch-profile-placeholder = Nom du profil
conflict-batch-matched-rule = via la règle « { $rule } » → { $action }
conflict-batch-empty = Tous les conflits sont résolus
conflict-batch-source-vs-destination = Source vs destination
conflict-batch-source-label = Source
conflict-batch-destination-label = Destination
conflict-batch-size-label = Taille
conflict-batch-modified-label = Modifié
conflict-batch-close = Fermer
conflict-batch-profile-saved = Profil de conflit enregistré

# Phase 23 — sparse-file preservation. The toast fires once per
# destination volume when the filesystem can't preserve holes; the
# warning line is surfaced in the job detail drawer so the user knows
# the dst is larger on disk than the source was.
sparse-not-supported-title = La destination remplit les fichiers creux
sparse-not-supported-body = { $dst_fs } ne prend pas en charge les fichiers creux. Les trous de la source ont été écrits sous forme de zéros, la destination est donc plus volumineuse sur disque.
sparse-warning-densified = Disposition creuse préservée : seuls les extents alloués ont été copiés.
sparse-warning-mismatch = Incohérence de disposition creuse — la destination peut être plus volumineuse que prévu.

# Phase 24 — security-metadata preservation. The Mark-of-the-Web
# (Zone.Identifier ADS) toggle is security-sensitive: turning it off
# lets a downloaded executable shed its SmartScreen / Office Protected
# View flag on copy, which is why the tooltip carries an explicit
# warning. AppleDouble fallback emits `._<filename>` sidecars on
# destination filesystems that can't hold the foreign metadata.
settings-preserve-security-metadata = Préserver les métadonnées de sécurité
settings-preserve-security-metadata-hint = Capture et réapplique les flux de métadonnées hors bande (NTFS ADS / xattrs / ACL POSIX / contextes SELinux / capacités de fichier Linux / forks de ressources macOS) à chaque copie.
settings-preserve-motw = Préserver le Mark-of-the-Web (indicateur de téléchargement Internet)
settings-preserve-motw-hint = Essentiel pour la sécurité. SmartScreen et Office Protected View utilisent ce flux pour avertir des fichiers téléchargés depuis Internet. Le désactiver permet à un exécutable téléchargé de perdre son marqueur d'origine à la copie et de contourner les protections du système d'exploitation.
settings-preserve-posix-acls = Préserver les ACL POSIX et les attributs étendus
settings-preserve-posix-acls-hint = Transporte les xattrs user.* / system.* / trusted.* et les listes de contrôle d'accès POSIX lors de la copie.
settings-preserve-selinux = Préserver les contextes SELinux
settings-preserve-selinux-hint = Transporte l'étiquette security.selinux lors de la copie afin que les démons exécutés sous des politiques MAC puissent toujours accéder au fichier.
settings-preserve-resource-forks = Préserver les forks de ressources et les infos Finder de macOS
settings-preserve-resource-forks-hint = Transporte l'ancien fork de ressources et FinderInfo (étiquettes de couleur, métadonnées Carbon) lors de la copie.
settings-appledouble-fallback = Utiliser un sidecar AppleDouble sur les systèmes de fichiers incompatibles
meta-translated-to-appledouble = Métadonnées étrangères stockées dans un sidecar AppleDouble (._{ $ext })

# Phase 25 — two-way sync with vector-clock conflict detection.
# The drawer lists configured sync pairs; each pair runs an
# independent reconciliation round against a per-pair `.copythat-sync.db`
# state store. Concurrent edits from a common ancestor surface as
# conflicts rather than silent overwrites; the losing side's content
# is preserved as `name.sync-conflict-YYYYMMDD-HHMMSS-<host>.ext`.
footer-sync = Synchro
sync-drawer-title = Synchronisation bidirectionnelle
sync-drawer-hint = Gardez deux dossiers synchronisés sans écrasements silencieux. Les modifications concurrentes apparaissent comme des conflits que vous pouvez résoudre.
sync-add-pair = Ajouter une paire
sync-add-cancel = Annuler
sync-refresh = Actualiser
sync-add-save = Enregistrer la paire
sync-add-saving = Enregistrement…
sync-add-missing-fields = L'étiquette, le chemin de gauche et le chemin de droite sont tous requis.
sync-remove-confirm = Supprimer cette paire de synchronisation ? La base d'état est conservée ; les dossiers ne sont pas touchés.
sync-field-label = Étiquette
sync-field-label-placeholder = ex. Documents ↔ NAS
sync-field-left = Dossier de gauche
sync-field-left-placeholder = Choisissez ou collez un chemin absolu
sync-field-right = Dossier de droite
sync-field-right-placeholder = Choisissez ou collez un chemin absolu
sync-field-mode = Mode
sync-mode-two-way = Bidirectionnel
sync-mode-mirror-left-to-right = Miroir (gauche → droite)
sync-mode-mirror-right-to-left = Miroir (droite → gauche)
sync-mode-contribute-left-to-right = Contribution (gauche → droite, sans suppression)
sync-no-pairs = Aucune paire de synchronisation configurée pour l'instant. Cliquez sur « Ajouter une paire » pour commencer.
sync-loading = Chargement des paires configurées…
sync-never-run = Jamais exécutée
sync-running = En cours
sync-run-now = Exécuter maintenant
sync-cancel = Annuler
sync-remove-pair = Supprimer
sync-view-conflicts = Voir les conflits ({ $count })
sync-conflicts-heading = Conflits
sync-no-conflicts = Aucun conflit lors de la dernière exécution.
sync-winner = Gagnant
sync-side-left-to-right = gauche
sync-side-right-to-left = droite
sync-conflict-kind-concurrent-write = Modification concurrente
sync-conflict-kind-delete-edit = Suppression ↔ modification
sync-conflict-kind-add-add = Ajouté des deux côtés
sync-conflict-kind-corrupt-equal = Contenu divergent sans nouvelle écriture
sync-resolve-keep-left = Conserver la gauche
sync-resolve-keep-right = Conserver la droite
sync-resolve-keep-both = Conserver les deux
sync-resolve-three-way = Résoudre par fusion à trois voies
sync-resolve-phase-53-tooltip = La fusion interactive à trois voies pour les fichiers non texte arrive en Phase 53.
sync-error-prefix = Erreur de synchronisation

# Phase 26 — real-time mirror watcher. "Live mirror" starts a
# filesystem watcher on the pair's left side; every debounced event
# triggers a re-sync. The watcher filters vim swap files / Office
# lock files / atomic-save temp names so one logical save becomes
# exactly one sync round.
live-mirror-start = Démarrer le miroir en direct
live-mirror-stop = Arrêter le miroir en direct
live-mirror-watching = Surveillance
live-mirror-toggle-hint = Resynchronise automatiquement à chaque changement de système de fichiers détecté. Un thread d'arrière-plan par paire active.
watch-event-prefix = Changement de fichier
watch-overflow-recovered = Le tampon du surveillant a débordé ; réénumération pour récupérer

# Phase 27 — content-defined chunk store. Enables delta-resume (a
# retry only re-writes chunks that actually changed) and same-job
# dedup (files sharing content blocks store those blocks once). The
# store is disk-backed under `<data-dir>/chunks/` by default and is
# the foundation for the Phase 49–51 moonshot repository phases.
chunk-store-section = Magasin de blocs
chunk-store-enable = Activer le magasin de blocs (reprise delta et déduplication)
chunk-store-enable-hint = Divise chaque fichier copié par contenu (FastCDC) et stocke les blocs adressés par contenu. Les nouvelles tentatives ne réécrivent que les blocs modifiés ; les fichiers au contenu partagé sont dédupliqués automatiquement.
chunk-store-location = Emplacement du magasin de blocs
chunk-store-max-size = Taille maximale du magasin de blocs
chunk-store-prune = Élaguer les blocs plus anciens que (jours)
chunk-store-savings = { $gib } Gio économisés grâce à la déduplication de blocs
chunk-store-disk-usage = Utilisation de { $size } sur { $chunks } blocs

# Phase 28 — tray-resident Drop Stack. The stack is a persistent list
# of paths gathered from multiple sources (Explorer drag, main-window
# context menu, CLI, drag onto the Drop Stack window) that the user
# can dispatch to a destination in one go.
dropstack-window-title = Drop Stack
dropstack-tray-open = Drop Stack
dropstack-empty-title = La Drop Stack est vide
dropstack-empty-hint = Faites glisser des fichiers ici depuis Explorer ou faites un clic droit sur une ligne de tâche pour l'ajouter.
dropstack-add-to-stack = Ajouter à la Drop Stack
dropstack-copy-all-to = Tout copier vers…
dropstack-move-all-to = Tout déplacer vers…
dropstack-clear = Vider la pile
dropstack-remove-row = Retirer de la pile
dropstack-path-missing-toast = { $path } déposé — le fichier n'existe plus.
dropstack-always-on-top = Garder la Drop Stack toujours au premier plan
dropstack-show-tray-icon = Afficher l'icône Freally File Manager dans la barre d'état
dropstack-open-on-start = Ouvrir automatiquement la Drop Stack au démarrage de l'application
dropstack-count = { $count } chemin

# Phase 29 — spring-loaded folders + native DnD polish. The Settings
# → General tab carries the knobs; DropTarget / DestinationPicker
# Svelte components use the dropzone-* keys at runtime.
settings-dnd-heading = Glisser-déposer
settings-dnd-spring-load = Ouvrir automatiquement les dossiers pendant le glissement
settings-dnd-spring-delay = Délai d'ouverture automatique (ms)
settings-dnd-thumbnails = Afficher les miniatures de glissement
settings-dnd-invalid-highlight = Mettre en évidence les cibles de dépôt invalides
dropzone-invalid-title = Cible de dépôt non valide
dropzone-invalid-readonly = La destination est en lecture seule
dropzone-picker-title = Choisir une destination
dropzone-picker-up = Remonter
dropzone-picker-path = Chemin actuel
dropzone-picker-root = Racines
dropzone-picker-use-this = Utiliser ce dossier
dropzone-picker-empty = Aucun sous-dossier
dropzone-picker-cancel = Annuler

# Phase 30 — cross-platform path translation. Settings → Transfer
# exposes these under a "Cross-platform compatibility" subsection.
translate-heading = Compatibilité multiplateforme
translate-unicode-label = Normalisation Unicode
translate-unicode-auto = Détecter automatiquement la destination
translate-unicode-windows = NFC (Windows / Linux)
translate-unicode-macos = Laisser tel quel (macOS / APFS)
translate-line-endings-label = Convertir les fins de ligne pour les fichiers texte
translate-line-endings-allowlist = Extensions de fichiers texte
reserved-name-label = Gestion des noms réservés Windows
reserved-name-suffix = Ajouter « _ » (CON.txt → CON_.txt)
reserved-name-reject = Rejeter et avertir
long-path-label = Utiliser le préfixe de chemin long Windows (\\?\) au-delà de 260 caractères
long-path-hint = Certains partages réseau et anciens outils ne respectent pas l'espace de noms \\?\.

# Phase 31 — power-aware copying. Settings → Power & State tab + the
# header badge that renders "⏸ Paused — Zoom call detected" when the
# runner's power subscriber has paused or capped due to a policy match.
power-heading = Alimentation et état
power-enabled = Activer les règles tenant compte de l'alimentation
power-battery-label = Sur batterie
power-metered-label = Sur Wi-Fi à consommation limitée
power-cellular-label = Sur réseau cellulaire
power-presentation-label = Pendant une présentation (Zoom / Teams / Keynote)
power-fullscreen-label = Lorsqu'une application est en plein écran
power-thermal-label = Lorsque le processeur réduit sa fréquence pour cause de chaleur
power-rule-continue = Continuer à pleine vitesse
power-rule-pause = Mettre en pause toutes les tâches
power-rule-cap = Limiter la bande passante
power-rule-cap-percent = Limiter à un pourcentage du débit actuel
power-reason-on-battery = sur batterie
power-reason-metered-network = réseau à consommation limitée
power-reason-cellular-network = réseau cellulaire
power-reason-presenting = mode présentation
power-reason-fullscreen = application en plein écran
power-reason-thermal-throttling = le processeur réduit sa fréquence

# Phase 32 — cloud backend matrix via OpenDAL. Settings → Remotes
# tab + the Add-backend wizard that writes one entry per remote into
# the keychain-backed credential store.
remote-heading = Backends distants
remote-add = Ajouter un backend
remote-list-empty = Aucun backend distant configuré
remote-test = Tester la connexion
remote-test-success = Connexion réussie
remote-test-failed = Échec de la connexion
remote-remove = Supprimer le backend
remote-name-label = Nom d'affichage
remote-kind-label = Type de backend
remote-save = Enregistrer le backend
remote-cancel = Annuler
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
backend-local-fs = Système de fichiers local
cloud-config-bucket = Bucket
cloud-config-region = Région
cloud-config-endpoint = URL de point de terminaison
cloud-config-root = Chemin racine
cloud-error-invalid-config = La configuration du backend n'est pas valide
cloud-error-network = Erreur réseau lors de la connexion au backend
cloud-error-not-found = Objet introuvable au chemin demandé
cloud-error-permission = Autorisation refusée par le backend distant
cloud-error-keychain = Échec de l'accès au trousseau du système
settings-tab-remotes = Backends distants
settings-tab-mobile = Mobile

# Phase 33 — mount Freally File Manager's chunk store + history archive as a
# read-only filesystem (FUSE / WinFsp). Surfaced by the History tab
# context menu's "Mount snapshot" action + the Settings → Advanced
# "Mount latest on launch" toggle.
mount-heading = Monter l'instantané
mount-action-mount = Monter l'instantané
mount-action-unmount = Démonter
mount-status-mounted = Monté sur { $path }
mount-error-unsafe-mountpoint = Le chemin du point de montage n'est pas sûr
mount-error-mountpoint-not-empty = Le point de montage doit être un dossier vide
mount-error-backend-unavailable = Le backend de montage n'est pas disponible sur ce système
mount-error-archive-read = Échec de la lecture de l'archive
mount-picker-title = Choisir le dossier du point de montage
mount-toast-mounted = Instantané monté sur { $path }
mount-toast-unmounted = Instantané démonté
mount-toast-failed = Échec du montage : { $reason }
settings-mount-heading = Monter les instantanés
settings-mount-hint = Expose l'archive d'historique comme système de fichiers en lecture seule. La Phase 33b câble le flux d'exécution ; les backends FUSE/WinFsp du noyau arrivent en Phase 33c.
settings-mount-on-launch = Monter le dernier instantané au lancement
settings-mount-on-launch-path = Chemin du point de montage
settings-mount-on-launch-path-placeholder = ex. C:\Mounts\copythat

# Phase 34 — enterprise-grade audit log export + WORM (write-once-read-
# many) tamper-resistance. Settings → Advanced → Audit log collects the
# format + destination + rotation + WORM toggle; runtime records job and
# file events into the tamper-evident chain-hashed sink.
settings-audit-heading = Journal d'audit
settings-audit-hint = Journal en ajout seul, infalsifiable, de chaque événement de tâche et de fichier. Les formats incluent CSV, JSON-lines, Syslog RFC 5424, ArcSight CEF et QRadar LEEF.
settings-audit-enable = Activer la journalisation d'audit
settings-audit-format = Format du journal
settings-audit-format-json-lines = JSON lines (valeur par défaut recommandée)
settings-audit-format-csv = CSV (compatible tableur)
settings-audit-format-syslog = Syslog (RFC 5424)
settings-audit-format-cef = CEF (ArcSight)
settings-audit-format-leef = LEEF 2.0 (IBM QRadar)
settings-audit-file-path = Chemin du fichier journal
settings-audit-file-path-placeholder = ex. C:\ProgramData\CopyThat\audit.log
settings-audit-max-size = Effectuer une rotation après (octets, 0 = jamais)
settings-audit-worm = Activer le mode WORM (write-once-read-many)
settings-audit-worm-hint = Applique l'indicateur d'ajout seul de la plateforme (Linux chattr +a, macOS chflags uappnd, attribut lecture seule Windows) après chaque création ou rotation. Même un administrateur doit explicitement effacer l'indicateur pour tronquer le journal.
settings-audit-test-write = Test d'écriture
settings-audit-verify-chain = Vérifier la chaîne
toast-audit-test-write-ok = Test d'écriture du journal d'audit réussi
toast-audit-verify-ok = Chaîne d'audit vérifiée et intacte
toast-audit-verify-failed = La vérification de la chaîne d'audit a signalé des incohérences

# Phase 35 — destination encryption via age (X25519 / passphrase /
# SSH) + on-the-fly zstd compression with a per-extension deny
# heuristic. Surfaced through Settings → Transfer → Encryption +
# Compression; engine short-circuits to the crypt pipeline when
# either stage is active.
settings-crypt-heading = Chiffrement et compression
settings-crypt-hint = Transforme le contenu des fichiers avant qu'ils n'atteignent la destination. Le chiffrement utilise le format age ; la compression utilise zstd et peut ignorer les médias déjà compressés selon leur extension.
settings-crypt-encryption-mode = Chiffrement
settings-crypt-encryption-off = Désactivé
settings-crypt-encryption-passphrase = Phrase secrète (demandée au début de la copie)
settings-crypt-encryption-recipients = Clés des destinataires depuis un fichier
settings-crypt-encryption-hint = Les phrases secrètes ne sont conservées en mémoire que pendant la durée de la copie. Les fichiers de destinataires listent une clé publique age1… ou ssh- par ligne.
settings-crypt-recipients-file = Chemin du fichier des destinataires
settings-crypt-recipients-file-placeholder = ex. C:\Users\me\recipients.txt
settings-crypt-compression-mode = Compression
settings-crypt-compression-off = Désactivée
settings-crypt-compression-always = Toujours
settings-crypt-compression-smart = Intelligente (ignorer les médias déjà compressés)
settings-crypt-compression-hint = Le mode intelligent ignore les formats jpg, mp4, zip, 7z et similaires qui ne bénéficient pas de zstd. Le mode Toujours compresse chaque fichier au niveau choisi.
settings-crypt-compression-level = Niveau zstd (1-22)
settings-crypt-compression-level-hint = Les valeurs plus basses sont plus rapides ; les valeurs plus élevées compressent davantage. Le niveau 3 correspond à la valeur par défaut de la CLI de zstd.
compress-footer-savings = 💾 { $original } → { $compressed } ({ $percent } % économisés)
compress-savings-toast = Compressé de { $percent } % ({ $bytes } économisés)
crypt-toast-recipients-loaded = { $count } destinataires de chiffrement chargés
crypt-toast-recipients-error = Échec du chargement des destinataires : { $reason }
crypt-toast-passphrase-required = Le chiffrement nécessite une phrase secrète avant le début de la copie
crypt-toast-passphrase-set = Phrase secrète de chiffrement enregistrée
crypt-footer-encrypted-badge = 🔒 Chiffré (age)
crypt-footer-compressed-badge = 📦 Compressé (zstd)

# Phase 36 — copythat CLI surface. Documented exit codes + the
# user-facing strings for plan/apply/verify/config. Help text stays
# in English (engineering accessibility) but error / info / status
# strings are localized so a localized desktop installation surfaces
# CLI errors in the same language as the GUI.
cli-help-tagline = Freally File Manager CLI — copie, synchronisation, vérification et audit de fichiers octet par octet pour les pipelines CI/CD.
cli-help-exit-codes = Codes de sortie : 0 succès, 1 erreur, 2 en attente, 3 conflit, 4 échec vérification, 5 réseau, 6 autorisation, 7 disque plein, 8 annulation, 9 configuration.
cli-error-bad-args = copy/move nécessite au moins une source et une destination
cli-error-unknown-algo = Algorithme de vérification inconnu : { $algo }
cli-error-missing-spec = --spec est requis pour plan/apply
cli-error-spec-parse = Échec de l'analyse du jobspec { $path } : { $reason }
cli-error-spec-empty-sources = La liste des sources du jobspec est vide
cli-info-shape-recorded = Limite de bande passante « { $rate } » enregistrée ; l'application est gérée via copythat-shape
cli-info-stub-deferred = { $command } est planifié pour le câblage de suivi de la Phase 36
cli-plan-summary = Plan : { $actions } action(s), { $bytes } octet(s) ; { $already_done } déjà en place
cli-plan-pending = Le plan signale des actions en attente ; relancez avec `apply` pour exécuter
cli-plan-already-done = Le plan ne signale rien à faire (idempotent)
cli-apply-success = Application terminée sans erreur
cli-apply-failed = Application terminée avec une ou plusieurs erreurs
cli-verify-ok = Vérification réussie : { $algo } { $digest }
cli-verify-failed = ÉCHEC de la vérification pour { $path } ({ $algo })
cli-config-set = { $key } défini sur { $value }
cli-config-reset = { $key } réinitialisé à la valeur par défaut
cli-config-unknown-key = Clé de configuration inconnue : { $key }
cli-completions-emitted = Complétions de shell pour { $shell } imprimées sur stdout

# Phase 37 — desktop-side mobile companion. Settings → Mobile panel
# strings + the SAS-confirmation modal + push-notification toasts.
# The actual mobile UI lives in the Phase 37 follow-up Tauri Mobile
# target; the phone displays its own localized SAS prompt.
settings-mobile-heading = Compagnon mobile
settings-mobile-hint = Associez un iPhone ou un téléphone Android pour parcourir l'historique, lancer des profils enregistrés et des jobspecs de la Phase 36, et recevoir des notifications de fin.
settings-mobile-pair-toggle = Autoriser les nouvelles associations
settings-mobile-pair-active = Serveur d'association actif — scannez le QR avec l'application mobile Freally File Manager
settings-mobile-pair-button = Démarrer l'association
settings-mobile-revoke-button = Révoquer
settings-mobile-no-pairings = Aucun appareil associé pour l'instant
settings-mobile-pair-port = Port de liaison (0 = en choisir un libre)
pair-sas-prompt = Les deux écrans doivent afficher les mêmes quatre émojis. Touchez Correspond s'ils sont identiques.
pair-sas-confirm = Correspond
pair-sas-reject = Différent — annuler
pair-toast-success = Associé avec { $device }
pair-toast-failed = Échec de l'association : { $reason }
push-toast-sent = Notification envoyée à { $device }
push-toast-failed = Échec de l'envoi de la notification à { $device } : { $reason }

# Phase 38 — aggregate destination dedup + reflink fallback ladder.
# Settings → Transfer → Dedup panel + per-job-row badges (⚡
# Reflinked / 🔗 Hardlinked / 🧩 Chunk-shared / 📋 Copied) +
# pre-pass dedup-scan modal.
settings-dedup-heading = Déduplication de la destination
settings-dedup-hint = Lorsque la source et la destination partagent un volume, Freally File Manager peut cloner les fichiers au niveau du système de fichiers au lieu de copier les octets. Le reflink est instantané et sûr ; le lien physique est plus rapide mais les deux noms partagent leur état.
settings-dedup-mode-auto = Échelle auto (reflink → lien physique → bloc → copie)
settings-dedup-mode-reflink-only = Reflink uniquement
settings-dedup-mode-hardlink-aggressive = Agressif (reflink + lien physique même sur les fichiers modifiables)
settings-dedup-mode-off = Désactivé (toujours copier les octets)
settings-dedup-hardlink-policy = Politique de lien physique
settings-dedup-prescan = Pré-analyser l'arborescence de destination pour le contenu en double
dedup-badge-reflinked = ⚡ Reflink
dedup-badge-hardlinked = 🔗 Lien physique
dedup-badge-chunk-shared = 🧩 Bloc partagé
dedup-badge-copied = 📋 Copié
phase42-paranoid-verify-label = Vérification paranoïaque
phase42-paranoid-verify-hint = Vide les pages en cache de la destination et relit depuis le disque pour détecter les mensonges du cache d'écriture et la corruption silencieuse. Environ 50 % plus lent que la vérification par défaut ; désactivé par défaut.
phase42-sharing-violation-retries-label = Tentatives sur les fichiers source verrouillés
phase42-sharing-violation-retries-hint = Nombre de nouvelles tentatives lorsqu'un autre processus maintient le fichier source ouvert avec un verrou exclusif. La temporisation double à chaque tentative (50 ms / 100 ms / 200 ms par défaut). Par défaut 3, comme Robocopy /R:3.
phase42-cloud-placeholder-warning = { $name } est un fichier OneDrive uniquement dans le cloud. Le copier déclenchera un téléchargement — jusqu'à { $size } via votre connexion réseau.
phase42-defender-exclusion-hint = Pour un débit de copie maximal, ajoutez le dossier de destination aux exclusions de Microsoft Defender avant les transferts en masse. Voir docs/PERFORMANCE_TUNING.md.

# Phase 39 — Browser-accessible recovery UI. Settings → Advanced
# exposes these strings; the recovery server itself renders askama
# templates that always read English — these eight keys are the
# Settings prose the user actually reads in their preferred locale.
settings-recovery-heading = Interface web de récupération
settings-recovery-enable = Activer l'interface web de récupération
settings-recovery-bind-address = Adresse de liaison
settings-recovery-port = Port (0 = en choisir un libre)
settings-recovery-show-url = Afficher l'URL et le jeton
settings-recovery-rotate-token = Renouveler le jeton
settings-recovery-allow-non-loopback = Autoriser la liaison hors boucle locale
settings-recovery-non-loopback-warning = ATTENTION : activer une liaison hors boucle locale expose l'interface de récupération à votre réseau local. Quiconque connaît le jeton peut parcourir votre historique de fichiers et télécharger des fichiers. Protégez-la avec TLS ou un proxy inverse si le réseau local n'est pas de confiance.

# Phase 40 — SMB compression negotiation + cloud-VM offload helper.
# 6 SMB keys (header badge + Settings prose) + 6 cloud-offload keys
# (Remotes tab wizard for cross-cloud copy templates).
smb-compress-badge = 🗜 Compression SMB : { $algo }
smb-compress-badge-tooltip = Le trafic réseau vers cette destination est compressé en transit (SMB 3.1.1).
smb-compress-toast-saved = { $bytes } économisés sur le réseau
smb-compress-algo-unknown = algorithme inconnu
settings-smb-compress-heading = Compression réseau SMB
settings-smb-compress-hint = Négocie automatiquement la compression du trafic SMB 3.1.1 sur les destinations UNC. Gain facile sur les liaisons lentes ; ignoré sur les destinations locales.
cloud-offload-heading = Assistant de délégation cloud-VM
cloud-offload-hint = Lors d'une copie directe entre deux clouds, génère un modèle de déploiement qui exécute la copie depuis une petite VM éphémère dans le cloud — les octets ne touchent jamais le réseau de votre ordinateur portable.
cloud-offload-render-button = Générer le modèle
cloud-offload-copy-clipboard = Copier dans le presse-papiers
cloud-offload-template-format = Format de modèle
cloud-offload-self-destruct-warning = La VM s'arrête automatiquement après { $minutes } minutes — vérifiez le rôle IAM et la région avant de déployer.

# Phase 41 — animated before/after tree-diff preview. The `Preview
# changes` modal renders the rolled-up plan before the engine starts
# work; 14 keys cover the title, the summary header / counts, the
# row-reason labels, and the two action buttons.
preview-modal-title = Aperçu des modifications
preview-summary-header = Ce qui va se passer
preview-category-additions = { $count } ajouts
preview-category-replacements = { $count } remplacements
preview-category-skips = { $count } ignorés
preview-category-conflicts = { $count } conflits
preview-category-unchanged = { $count } inchangés
preview-bytes-to-transfer = { $bytes } à transférer
preview-reason-source-newer = La source est plus récente
preview-reason-dest-newer = La destination est plus récente — sera ignorée
preview-reason-content-different = Le contenu diffère
preview-reason-identical = Identique à la source
preview-button-run = Exécuter le plan
preview-button-reduce = Réduire mon plan…

# Phase 42 — perceptual-hash visual-similarity dedup. Eight keys cover
# the pre-copy "looks visually identical" warning + the Settings panel.
perceptual-warn-title = Semble visuellement identique
perceptual-warn-body = { $name } à la destination semble correspondre à l'image source. Continuer la copie quand même ?
perceptual-warn-keep-both = Conserver les deux
perceptual-warn-skip = Ignorer ce fichier
perceptual-warn-overwrite = Remplacer quand même
perceptual-settings-heading = Déduplication par similarité visuelle
perceptual-settings-hint = Détecte les images visuellement identiques à la destination avant qu'elles ne soient remplacées. Le hachage est perceptuel (reconnaît la même image réenregistrée dans un autre format), pas octet par octet.
perceptual-settings-threshold-label = Seuil d'avertissement (plus bas = correspondance plus stricte)

# Phase 42 Part B — per-file rolling versions (Time Machine for any
# destination). 8 keys cover the version-list panel + retention picker.
version-list-heading = Versions précédentes
version-list-empty = Aucune version antérieure de ce fichier
version-list-restore = Restaurer cette version
version-retention-heading = Conserver les versions précédentes lors d'un remplacement
version-retention-none = Conserver toutes les versions indéfiniment
version-retention-last-n = Conserver les { $n } dernières versions
version-retention-older-than-days = Supprimer les versions plus anciennes que { $days } jours
version-retention-gfs = Toutes les heures { $h } · quotidien { $d } · hebdomadaire { $w } · mensuel { $m }

# Phase 43 — forensic chain-of-custody manifests + BLAKE3 verified
# streaming. 14 keys cover the Settings → Provenance panel
# (heading, hint, toggles, TSA URL, signing-key management) plus
# the post-job manifest notification + the verify command's two
# headline result lines.
provenance-settings-heading = Chaîne de responsabilité forensique
provenance-settings-hint = Signe chaque tâche de copie avec un manifeste BLAKE3 + ed25519. Les vérificateurs peuvent rehacher l'arborescence de destination plus tard et prouver qu'aucun octet n'a changé depuis la copie.
provenance-settings-enable-default = Signer chaque nouvelle tâche par défaut
provenance-settings-show-after-job = Afficher le manifeste après chaque tâche terminée
provenance-settings-tsa-url-label = URL de l'autorité d'horodatage RFC 3161 par défaut
provenance-settings-tsa-url-hint = Facultatif. Lorsqu'elle est définie, les manifestes portent un horodatage TSA gratuit prouvant que les octets existaient à ce moment précis. Laissez vide pour ignorer.
provenance-settings-keys-heading = Clés de signature
provenance-settings-keys-generate = Générer une nouvelle clé
provenance-settings-keys-import = Importer une clé…
provenance-settings-keys-export = Exporter la clé publique…
provenance-job-completed-title = Manifeste de provenance enregistré
provenance-job-completed-body = { $count } fichiers signés → { $path }
provenance-verify-clean = Manifeste valide pour { $count } fichiers ; signature { $sig } ; racine Merkle OK.
provenance-verify-tampered = Manifeste INVALIDE — { $tampered } altérés, { $missing } manquants.
# Phase 43 post-review hardening — toast text for the Settings →
# Provenance buttons whose Tauri IPC has not yet landed.
provenance-action-staged = Phase 43 — le câblage IPC de cette action arrive dans un commit ultérieur.

# Phase 44 — SSD-aware whole-drive sanitize (NVMe Sanitize / OPAL
# Crypto Erase) + the per-file shred refusal on copy-on-write
# filesystems. 16 keys cover the new "Drive sanitize" Settings
# subsection and the localized error messages.
sanitize-heading = Effacement sécurisé du disque entier
sanitize-hint = NVMe Sanitize, OPAL Crypto Erase et ATA Secure Erase effacent un disque flash au niveau du firmware en quelques millisecondes. La réécriture par fichier n'a aucun sens sur la mémoire flash — la destruction multipasses ne fait qu'user le NAND. Utilisez ceci pour une véritable purge.
sanitize-pick-device = Choisir le disque à effacer
sanitize-mode-label = Méthode d'effacement
sanitize-mode-nvme-format = NVMe Format (avec secure erase)
sanitize-mode-nvme-sanitize-block = NVMe Sanitize — Block Erase (lent, chaque cellule)
sanitize-mode-nvme-sanitize-crypto = NVMe Sanitize — Crypto Erase (instantané)
sanitize-mode-ata-secure-erase = ATA Secure Erase (anciens SSD SATA)
sanitize-mode-opal-crypto-erase = TCG OPAL Crypto Erase (disques à chiffrement automatique)
sanitize-mode-apfs-crypto-erase = APFS Crypto Erase (rotation de la clé FileVault, macOS uniquement)
sanitize-confirm-1 = Cette action détruit CHAQUE octet de { $device }. Il n'y a aucune annulation possible.
sanitize-confirm-2 = Je comprends que toutes les partitions, tous les fichiers et tous les instantanés de { $device } deviendront définitivement illisibles.
sanitize-confirm-3 = Saisissez le nom de modèle du disque pour continuer : { $model }
sanitize-running = Effacement de { $device } ({ $mode }) — cela peut prendre de quelques millisecondes (crypto erase) à plusieurs dizaines de minutes (block erase). Ne pas éteindre.
sanitize-completed = Effacement terminé — { $device } est maintenant vide.
ssd-honest-shred-meaningless = La destruction par fichier sur un système de fichiers à copie sur écriture (Btrfs / ZFS / APFS) ne peut pas atteindre les blocs sous-jacents. Utilisez plutôt l'effacement sécurisé du disque entier ainsi que la rotation de la clé de chiffrement intégral du disque.
ssd-honest-advisory = Ce fichier se trouve sur de la mémoire flash. La réécriture par fichier use le NAND et ne garantit PAS que les cellules d'origine soient irrécupérables. Pour les données sensibles, effacez le disque entier.

# Phase 44.1f post-review — placeholder toast for SanitizeTab
# buttons whose Tauri IPC has not yet landed.
sanitize-action-staged = Phase 44.1 — le câblage IPC de cette action arrive dans un commit ultérieur.

# Phase 45.3 — named-queue tab strip (Subfeature A). Tabs surface
# once the QueueRegistry holds at least one queue; the synthesised
# default tab keeps legacy single-queue jobs reachable.
queue-tab-default = Par défaut
queue-tab-empty-state = Files de tâches
queue-badge-tooltip = Tâches en attente et en cours dans cette file

# Phase 45.4 — drag-progress-merge (Subfeature B). Drag a queue tab
# onto another to merge their job lists. The default tab is neither
# draggable nor a drop target; only registry queues participate.
queue-drag-hint = Faites glisser sur une autre file pour fusionner
queue-merge-confirm = Déposez pour fusionner
queue-merge-toast = Files fusionnées

# Phase 45.5 — F2-queue UX (Subfeature C). F2 toggles
# `auto_enqueue_next` so every fresh enqueue piles into the running
# queue rather than spawning a parallel one. Status pill renders in
# the Footer; pulsing dot renders on the running tab.
queue-f2-active-hint = Mode F2 : chaque nouvelle mise en file rejoint cette file
queue-f2-toggled-on = Mode file F2 ACTIVÉ — les nouvelles mises en file rejoignent la file en cours
queue-f2-toggled-off = Mode file F2 DÉSACTIVÉ — les nouvelles mises en file créent des files parallèles
queue-f2-status-bar = Mode file F2 : ACTIVÉ

# Phase 45.6 — tray destination targets (Subfeature D). Pinned
# destinations appear in the OS tray menu; clicking one arms it as
# the active drop target so the next file drop bypasses the
# DropStagingDialog. Settings → General hosts the list editor.
tray-target-section-title = Destinations de la barre d'état
tray-target-section-hint = Les destinations épinglées apparaissent dans le menu de la barre d'état. Cliquez sur l'une d'elles pour l'armer comme prochaine cible de dépôt.
tray-target-empty = Aucune destination épinglée dans la barre d'état pour l'instant.
tray-target-remove = Supprimer
tray-target-add-label = Étiquette
tray-target-add-path = Chemin ou URI de backend
tray-target-add = Ajouter
tray-target-armed-toast = Déposez votre prochain fichier pour l'envoyer vers { $label }
tray-target-active-pill = → { $label }

# Phase 45.7 follow-up — pinned-destination validation errors. The
# `queue_pin_destination` IPC returns these Fluent keys verbatim
# when input fails the IPC-boundary checks; the toast layer renders
# them via `t(...)` (Toast.svelte detects kebab-case-lowercase and
# routes through the locale table). Phase 17e's `err-path-escape` /
# `err-destination-empty` cover the path-traversal + empty cases for
# `queue_route_job`; the keys below are pin-specific.
err-pinned-destination-label-empty = L'étiquette de la destination ne peut pas être vide.
err-pinned-destination-path-empty = Le chemin de la destination ne peut pas être vide.
err-pinned-destination-label-too-long = L'étiquette de la destination est trop longue (64 caractères maximum).
err-pinned-destination-path-too-long = Le chemin de la destination est trop long (1024 caractères maximum).
err-pinned-destination-label-invalid = L'étiquette de la destination contient des caractères non autorisés (saut de ligne, retour chariot ou NUL).
err-pinned-destination-path-invalid = Le chemin de la destination contient des caractères non autorisés (saut de ligne, retour chariot ou NUL).
err-pinned-destination-too-many = Vous avez atteint la limite de 50 destinations dans la barre d'état. Supprimez-en une pour en ajouter une autre.

# Phase 46.6 — Settings → Plugins tab. The IPC layer in
# `apps/copythat-ui/src-tauri/src/plugin_commands.rs` enumerates the
# per-user plugin store under `<config_dir>/plugins/`; the
# `PluginsTab.svelte` component renders these strings against the
# returned manifest + grant state.
settings-tab-plugins = Plugins
plugin-heading = Plugins
plugin-hint = Des plugins WASM en bac à sable étendent Freally File Manager avec des hooks personnalisés. Chaque plugin s'exécute sous des limites de processeur et de mémoire par appel et ne voit que les capacités hôtes que vous lui accordez.
plugin-list-empty = Aucun plugin installé pour l'instant.
plugin-enabled = Activé
plugin-disabled = Désactivé
plugin-hooks = Hooks
plugin-capabilities = Capacités
plugin-no-capabilities = (aucune)
plugin-directory = Emplacement
plugin-install-from-file = Installer depuis un fichier…
plugin-install-from-url = Installer depuis une URL…
plugin-url-wasm = URL WASM
plugin-url-manifest = URL du manifeste
plugin-url-hash = Hachage BLAKE3
plugin-url-preview = Aperçu
plugin-url-confirm = Confirmer l'installation

# Phase 31b — power-policy settings (Power tab).
settings-tab-power = Alimentation
settings-power-hint = Limiter ou suspendre les copies selon l'alimentation : batterie, réseau facturé à l'usage/mobile, présentation/plein écran ou limitation thermique du CPU.
settings-power-enabled = Activer la limitation selon l'alimentation
settings-power-battery = Sur batterie
settings-power-metered = Sur réseau facturé à l'usage
settings-power-cellular = Sur réseau mobile
settings-power-presentation = Pendant une présentation
settings-power-fullscreen = En plein écran
settings-power-thermal = En limitation thermique
settings-power-continue = Continuer
settings-power-pause = Suspendre
err-server-not-implemented = Le mode serveur n'est pas encore disponible.
err-webhook-not-implemented = La distribution des webhooks n'est pas encore disponible.

# Phase 47 — "why is this slow?" diagnostics (bottleneck badge + tooltip).
bottleneck-source-io = Source I/O
bottleneck-dest-io = Destination I/O
bottleneck-network = Réseau
bottleneck-antivirus = Antivirus
bottleneck-cpu = CPU
bottleneck-thermal = Thermique
bottleneck-unknown = Inconnu
diag-aria = Goulot d'étranglement : { $cause }
diag-tooltip = Limité par { $cause } · { $rate }
diag-spark-aria = Débit de la dernière minute
diag-keeping-up = Suit le rythme
diag-label = Diagnostic

# Phase 48 — server mode + observability (Settings → Server).
settings-tab-server = Serveur
server-hint = Exécutez Freally File Manager comme un serveur de fichiers sans interface. Choisissez les protocoles à exposer, définissez l'adresse et le dossier à servir, et exigez éventuellement une authentification.
server-protocols = Protocoles
server-bind-addr = Adresse de liaison
server-root = Dossier servi
server-readonly = Lecture seule (refuser les envois et suppressions)
server-auth-mode = Authentification
server-auth-none = Aucune
server-auth-bearer = Jeton Bearer
server-auth-basic = Basique (utilisateur + mot de passe)
server-auth-token = Jeton
server-auth-user = Nom d'utilisateur
server-auth-password = Mot de passe
otel-endpoint = Point de terminaison OpenTelemetry
webhook-section = Webhooks
webhook-url = URL du webhook
webhook-add = Ajouter un webhook
webhook-remove = Supprimer
webhook-empty = Aucun webhook configuré.
webhook-pushover-token = Jeton Pushover
webhook-pushover-user = Utilisateur Pushover
server-start = Démarrer le serveur
server-stop = Arrêter le serveur
server-status-running = En cours d'exécution sur { $addr }
server-status-stopped = Arrêté
server-metrics-url = Métriques
err-server-no-protocols = Sélectionnez au moins un protocole avant de démarrer le serveur.
err-server-bind = Impossible de lier l'adresse du serveur. Elle est peut-être déjà utilisée.

# Library drawer (Phase 49) — unified content-addressed repository view.
footer-library = Bibliothèque
library-title = Bibliothèque
library-loading = Chargement du dépôt…
library-unavailable = Dépôt indisponible
library-tab-live = En direct
library-tab-snapshots = Instantanés
library-tab-versions = Versions
library-hero-savings = { $effective } effectifs servis · { $pct } économisé
library-hero-empty = { $chunks } blocs stockés — aucun instantané pour l'instant
library-stat-stored = Stocké sur le disque
library-stat-effective = Données effectives
library-stat-snapshots = Instantanés
library-stat-chunks = Blocs distincts
library-snapshot-empty = Aucun instantané pour l'instant
library-snapshot-files = { $n } fichiers
library-version-path-ph = Chemin de destination…
library-version-load = Afficher les versions
library-version-empty = Aucune version pour ce chemin
repo-kind-copy = Copie
repo-kind-sync = Synchronisation
repo-kind-version = Version
repo-kind-backup = Sauvegarde
# Phase 49o — snapshot diff / compare.
library-tab-compare = Comparer
repo-change-added = Ajouté
repo-change-removed = Supprimé
repo-change-modified = Modifié
repo-change-unchanged = Aucune modification
repo-diff-summary = { $added } ajoutés · { $removed } supprimés · { $modified } modifiés
repo-diff-bytes-added = { $bytes } nouveaux
repo-diff-pick-two = Choisissez deux instantanés à comparer
# Phase 49r — statistics / reports.
library-tab-reports = Rapports
report-growth-title = Croissance du stockage
report-by-kind-title = Par type
report-top-files-title = Fichiers principaux
report-dedup-ratio = { $pct }% dédupliqué
report-export = Exporter le rapport
report-exported = Rapport enregistré dans { $path }
report-file-versions = { $n } versions
# Phase 49p — pinning / prune.
repo-pin = Épingler
repo-unpin = Désépingler
repo-pinned-badge = Épinglé
repo-prune-title = Élaguer
repo-prune-keep-last = Conserver les plus récents
repo-prune-removed = { $n } instantanés élagués
repo-prune-none = Rien à élaguer

# Phase 49c — sources de sauvegarde.
library-tab-sources = Sources
backup-add-source = Ajouter une source…
backup-source-path-ph = Dossier à sauvegarder…
backup-exclude-ph = Motifs d'exclusion (séparés par des virgules)
backup-now = Sauvegarder maintenant
backup-remove = Retirer
backup-empty = Aucune source de sauvegarde pour l'instant
backup-never-run = Jamais sauvegardé
backup-last-run = Dernière sauvegarde { $when }
backup-running = Sauvegarde… { $files } fichiers
backup-toast-started = Sauvegarde de { $label }…
backup-toast-completed = { $label } sauvegardé : { $files } fichiers
backup-toast-failed = Échec de la sauvegarde de { $label } : { $reason }
# Phase 49e — per-source retention + prune.
backup-retention = Rétention
backup-retention-keep-all = Tout conserver
backup-retention-last = Conserver les { $n } derniers
backup-retention-days = Plus de { $days } jours
backup-retention-gfs = Rotation GFS
backup-prune-now = Élaguer maintenant
backup-prune-none = Rien à élaguer
backup-prune-result = { $removed } instantanés supprimés · { $bytes } récupérés
# Phase 49f — per-source scheduling.
backup-schedule = Planification
backup-schedule-manual = Manuel
backup-schedule-hourly = Toutes les heures
backup-schedule-daily = Tous les jours
backup-schedule-weekly = Toutes les semaines
backup-next-run = Prochaine exécution { $when }
backup-not-scheduled = Non planifié
# Phase 49g — source filters.
backup-include-ph = Motifs d'inclusion (séparés par des virgules)
backup-skip-hidden = Ignorer les cachés
# Phase 49q — notifications.
notify-title = Notifications
notify-on-success = En cas de succès
notify-on-failure = En cas d'échec
notify-test = Envoyer un test
notify-test-sent = Test envoyé à { $n } destination(s)

# Phase 49d — explorateur de restauration.
restore-browse = Restaurer…
restore-title = Restaurer depuis l'instantané
restore-select-all = Tout sélectionner
restore-dest = Restaurer vers
restore-confirm = Restaurer { $n } fichiers
restore-empty = Cet instantané ne contient aucun fichier
restore-conflict-body = { $count } fichiers sélectionnés existent déjà à destination.
restore-conflict-overwrite = Écraser
restore-conflict-skip = Ignorer les existants
restore-conflict-keep-both = Garder les deux
restore-toast-done = { $restored } restaurés, { $skipped } ignorés
restore-toast-failed = Échec de la restauration : { $reason }
snapshot-forget = Oublier
snapshot-forget-toast = Instantané oublié — lancez « Récupérer de l'espace » pour le libérer
library-reclaim = Récupérer de l'espace
# Phase 49i — full compaction.
library-compact = Compactage complet
library-compact-started = Compactage démarré — voir Tâches
# Phase 49h — compression.
library-stat-compression = Économisé par compression
storage-compression = Compression
storage-compression-off = Désactivée
storage-compression-auto = Auto (ignorer les incompressibles)
storage-compression-always = Toujours
storage-compression-restart = S'applique au prochain lancement
# Phase 49j — tasks & progress center.
footer-tasks = Tâches
tasks-title = Tâches
tasks-empty = Aucune tâche pour l'instant
tasks-running = En cours
tasks-recent = Récentes
tasks-cancel = Annuler
task-state-running = En cours
task-state-completed = Terminé
task-state-failed = Échec
task-state-cancelled = Annulé
# Phase 49k — repository setup/connect wizard.
repo-wizard-title = Connecter un dépôt
repo-wizard-create-tab = Créer un dépôt
repo-wizard-connect-tab = Connecter un dépôt existant
repo-field-name = Nom
repo-field-path = Emplacement
repo-field-password = Phrase secrète (facultative)
repo-action-create = Créer
repo-action-connect = Connecter
repo-action-browse = Parcourir…
repo-switcher-label = Dépôt
repo-action-forget = Oublier
repo-action-change-pass = Changer la phrase secrète…
repo-password-old = Phrase secrète actuelle
repo-password-new = Nouvelle phrase secrète
repo-error-exists = Un dépôt existe déjà à cet emplacement
repo-error-not-found = Aucun dépôt trouvé à cet emplacement
repo-error-bad-pass = Phrase secrète incorrecte
repo-note-no-encryption = La phrase secrète ne contrôle que l'accès ; le chiffrement au repos arrivera dans une version ultérieure
repo-confirm-forget = Retirer "{ $name }" de la liste ? Vos données restent sur le disque.
repo-toast-created = Dépôt "{ $name }" créé
repo-toast-connected = Connecté à "{ $name }"
repo-toast-pass-changed = Phrase secrète mise à jour
# Phase 49l — Sources dashboard.
library-tab-overview = Vue d'ensemble
library-source-empty = Aucune source pour l'instant
library-source-unknown = (source non spécifiée)
library-source-snapshots = { $n } instantanés
library-source-latest = Dernier { $when }
# Phase 49n — verify & repair.
repo-action-verify = Vérifier
repo-action-verify-deep = Vérifier (lire toutes les données)
repo-action-repair = Réparer…
repo-verify-clean = { $files } fichiers / { $chunks } blocs vérifiés — aucun dommage
repo-verify-damaged = { $missing } manquants, { $corrupt } blocs corrompus
repo-repair-confirm = Supprimer { $n } instantanés qui ne peuvent plus être restaurés ?
repo-repair-removed = { $n } instantanés endommagés supprimés
repo-repair-none = Rien à réparer — le dépôt est sain
repo-gc-done = { $bytes } récupérés ({ $chunks } blocs)
restore-toast-partial = { $restored } restaurés, { $skipped } ignorés, { $failed } en échec
