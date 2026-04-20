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
# MT — Phase 8 toast messages
toast-error-resolved = Erreur résolue
# MT
toast-collision-resolved = Conflit résolu
# MT
toast-elevated-unavailable = La nouvelle tentative avec droits élevés arrive en phase 17 — pas encore disponible
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
err-io-other = Erreur d'E/S inconnue

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
