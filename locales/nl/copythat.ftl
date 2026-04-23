app-name = Copy That v1.0.0
# MT
window-title = Copy That v1.0.0
# MT
shred-ssd-advisory = Waarschuwing: dit doel staat op een SSD. Meervoudig overschrijven saneert flashgeheugen niet betrouwbaar, omdat wear-leveling en overprovisioning gegevens buiten het logische blokadres verplaatsen. Gebruik bij solid-state media bij voorkeur ATA SECURE ERASE, NVMe Format met Secure Erase of volledige schijfversleuteling waarbij de sleutel wordt vernietigd.

# MT
state-idle = Inactief
# MT
state-copying = Kopiëren
# MT
state-verifying = Controleren
# MT
state-paused = Gepauzeerd
# MT
state-error = Fout

# MT
state-pending = In wachtrij
# MT
state-running = Actief
# MT
state-cancelled = Geannuleerd
# MT
state-succeeded = Voltooid
# MT
state-failed = Mislukt

# MT
action-pause = Pauzeren
# MT
action-resume = Hervatten
# MT
action-cancel = Annuleren
# MT
action-pause-all = Alle taken pauzeren
# MT
action-resume-all = Alle taken hervatten
# MT
action-cancel-all = Alle taken annuleren
# MT
action-close = Sluiten
# MT
action-reveal = Tonen in map

# MT
menu-pause = Pauzeren
# MT
menu-resume = Hervatten
# MT
menu-cancel = Annuleren
# MT
menu-remove = Uit wachtrij verwijderen
# MT
menu-reveal-source = Bron in map tonen
# MT
menu-reveal-destination = Doel in map tonen

# MT
header-eta-label = Geschatte resterende tijd
# MT
header-toolbar-label = Algemene bediening

# MT
footer-queued = actieve taken
# MT
footer-total-bytes = in uitvoering
# MT
footer-errors = fouten
# MT
footer-history = Geschiedenis

# MT
empty-title = Zet bestanden of mappen neer om te kopiëren
# MT
empty-hint = Sleep items naar het venster. We vragen om een doel en maken vervolgens één taak per bron.
# MT
empty-region-label = Takenlijst

# MT
details-drawer-label = Taakdetails
# MT
details-source = Bron
# MT
details-destination = Doel
# MT
details-state = Status
# MT
details-bytes = Bytes
# MT
details-files = Bestanden
# MT
details-speed = Snelheid
# MT
details-eta = Resterende tijd
# MT
details-error = Fout

# MT
drop-dialog-title = Neergezette items overdragen
# MT
drop-dialog-subtitle = { $count } item(s) klaar voor overdracht. Kies een doelmap om te beginnen.
# MT
drop-dialog-mode = Bewerking
# MT
drop-dialog-copy = Kopiëren
# MT
drop-dialog-move = Verplaatsen
# MT
drop-dialog-pick-destination = Doel kiezen
# MT
drop-dialog-change-destination = Doel wijzigen
# MT
drop-dialog-start-copy = Kopiëren starten
# MT
drop-dialog-start-move = Verplaatsen starten

# MT
eta-calculating = berekenen…
# MT
eta-unknown = onbekend

# MT
toast-job-done = Overdracht voltooid
# MT
toast-copy-queued = Kopie in wachtrij
# MT
toast-move-queued = Verplaatsing in wachtrij
# MT — Phase 8 toast messages
toast-error-resolved = Fout opgelost
# MT
toast-collision-resolved = Conflict opgelost
# MT
toast-elevated-unavailable = Opnieuw proberen met verhoogde rechten komt in fase 17 — nog niet beschikbaar
toast-clipboard-files-detected = Bestanden op het klembord — druk op uw plak-sneltoets om te kopiëren via Copy That
toast-clipboard-no-files = Klembord bevat geen bestanden om te plakken
# MT
toast-error-log-exported = Foutenlogboek geëxporteerd

# MT — Error modal
error-modal-title = Een overdracht is mislukt
# MT
error-modal-retry = Opnieuw proberen
# MT
error-modal-retry-elevated = Opnieuw proberen met verhoogde rechten
# MT
error-modal-skip = Overslaan
# MT
error-modal-skip-all-kind = Alle fouten van deze soort overslaan
# MT
error-modal-abort = Alles afbreken
# MT
error-modal-path-label = Pad
# MT
error-modal-code-label = Code
error-drawer-pending-count = Meer fouten wachten
error-drawer-toggle = Inklappen of uitklappen

# MT — Error-kind labels
err-not-found = Bestand niet gevonden
# MT
err-permission-denied = Toestemming geweigerd
# MT
err-disk-full = Doelschijf is vol
# MT
err-interrupted = Bewerking onderbroken
# MT
err-verify-failed = Controle na kopiëren mislukt
# MT
err-path-escape = Pad geweigerd — bevat bovenliggende map-segmenten (..) of ongeldige bytes
# MT
err-io-other = Onbekende I/O-fout
err-sparseness-mismatch = Sparse-indeling kon niet behouden blijven op bestemming  # MT

# MT — Collision modal
collision-modal-title = Bestand bestaat al
# MT
collision-modal-overwrite = Overschrijven
# MT
collision-modal-overwrite-if-newer = Overschrijven als nieuwer
# MT
collision-modal-skip = Overslaan
# MT
collision-modal-keep-both = Beide behouden
# MT
collision-modal-rename = Hernoemen…
# MT
collision-modal-apply-to-all = Op alles toepassen
# MT
collision-modal-source = Bron
# MT
collision-modal-destination = Bestemming
# MT
collision-modal-size = Grootte
# MT
collision-modal-modified = Gewijzigd
# MT
collision-modal-hash-check = Snelle hash (SHA-256)
# MT
collision-modal-rename-placeholder = Nieuwe bestandsnaam
# MT
collision-modal-confirm-rename = Hernoemen

# MT — Error log drawer
error-log-title = Foutenlogboek
# MT
error-log-empty = Geen fouten geregistreerd
# MT
error-log-export-csv = CSV exporteren
# MT
error-log-export-txt = Tekst exporteren
# MT
error-log-clear = Logboek wissen
# MT
error-log-col-time = Tijd
# MT
error-log-col-job = Taak
# MT
error-log-col-path = Pad
# MT
error-log-col-code = Code
# MT
error-log-col-message = Bericht
# MT
error-log-col-resolution = Oplossing

# MT — History drawer (Phase 9)
history-title = Geschiedenis
# MT
history-empty = Nog geen taken geregistreerd
# MT
history-unavailable = Kopieergeschiedenis is niet beschikbaar. De app kon de SQLite-opslag bij het opstarten niet openen.
# MT
history-filter-any = alle
# MT
history-filter-kind = Soort
# MT
history-filter-status = Status
# MT
history-filter-text = Zoeken
# MT
history-refresh = Vernieuwen
# MT
history-export-csv = CSV exporteren
# MT
history-purge-30 = Ouder dan 30 dagen wissen
# MT
history-rerun = Opnieuw uitvoeren
# MT
history-detail-open = Details
# MT
history-detail-title = Taakdetails
# MT
history-detail-empty = Geen items geregistreerd
# MT
history-col-date = Datum
# MT
history-col-kind = Soort
# MT
history-col-src = Bron
# MT
history-col-dst = Bestemming
# MT
history-col-files = Bestanden
# MT
history-col-size = Grootte
# MT
history-col-status = Status
# MT
history-col-duration = Duur
# MT
history-col-error = Fout

# MT
toast-history-exported = Geschiedenis geëxporteerd
# MT
toast-history-rerun-queued = Opnieuw uitvoeren in wachtrij

# MT — Totals drawer (Phase 10)
footer-totals = Totalen
# MT
totals-title = Totalen
# MT
totals-loading = Totalen laden…
# MT
totals-card-bytes = Totaal gekopieerde bytes
# MT
totals-card-files = Bestanden
# MT
totals-card-jobs = Taken
# MT
totals-card-avg-rate = Gemiddelde doorvoer
# MT
totals-errors = fouten
# MT
totals-spark-title = Laatste 30 dagen
# MT
totals-kinds-title = Per soort
# MT
totals-saved-title = Bespaarde tijd (geschat)
# MT
totals-saved-note = Geschat ten opzichte van een referentiekopie van dezelfde werklast met een standaard bestandsbeheerder.
# MT
totals-reset = Statistieken resetten
# MT
totals-reset-confirm = Dit verwijdert alle opgeslagen taken en items. Doorgaan?
# MT
totals-reset-confirm-yes = Ja, resetten
# MT
toast-totals-reset = Statistieken gereset

# MT — Phase 11a additions
header-language-label = Taal
# MT
header-language-title = Taal wijzigen

# MT
kind-copy = Kopiëren
# MT
kind-move = Verplaatsen
# MT
kind-delete = Verwijderen
# MT
kind-secure-delete = Veilig verwijderen

# MT
status-running = Wordt uitgevoerd
# MT
status-succeeded = Geslaagd
# MT
status-failed = Mislukt
# MT
status-cancelled = Geannuleerd
# MT
status-ok = OK
# MT
status-skipped = Overgeslagen

# MT
history-search-placeholder = /pad
# MT
toast-history-purged = { $count } taken ouder dan 30 dagen verwijderd

# MT
err-source-required = Minstens één bronpad is vereist.
# MT
err-destination-empty = Het doelpad is leeg.
# MT
err-source-empty = Het bronpad is leeg.

# MT
duration-lt-1s = < 1 s
# MT
duration-ms = { $ms } ms
# MT
duration-seconds = { $s } s
# MT
duration-minutes-seconds = { $m } min { $s } s
# MT
duration-hours-minutes = { $h } u { $m } min
# MT
duration-zero = 0 s

# MT
rate-unit-per-second = { $size }/s

# MT — Phase 11b Settings modal
settings-title = Instellingen
# MT
settings-tab-general = Algemeen
# MT
settings-tab-appearance = Weergave
# MT
settings-section-language = Taal
# MT
settings-phase-12-hint = Meer instellingen (thema, overdrachtsstandaarden, verificatie-algoritme, profielen) komen in fase 12.

# MT — Phase 12 Settings window
settings-loading = Instellingen laden…
# MT
settings-tab-transfer = Overdracht
# MT
settings-tab-shell = Shell
# MT
settings-tab-secure-delete = Veilig verwijderen
# MT
settings-tab-advanced = Geavanceerd
# MT
settings-tab-profiles = Profielen

# MT
settings-section-theme = Thema
# MT
settings-theme-auto = Automatisch
# MT
settings-theme-light = Licht
# MT
settings-theme-dark = Donker
# MT
settings-start-with-os = Starten bij opstarten systeem
# MT
settings-single-instance = Enkele actieve instantie
# MT
settings-minimize-to-tray = Minimaliseren naar systeemvak bij sluiten
settings-error-display-mode = Stijl foutprompt
settings-error-display-modal = Modaal (blokkeert de app)
settings-error-display-drawer = Zijpaneel (niet-blokkerend)
settings-error-display-mode-hint = Een modaal pauzeert de wachtrij totdat u beslist. Het zijpaneel laat de wachtrij doorlopen en u fouten in de hoek afhandelen.
settings-paste-shortcut = Bestanden plakken via globale sneltoets
settings-paste-shortcut-combo = Sneltoetscombinatie
settings-paste-shortcut-hint = Druk op deze combinatie ergens in het systeem om bestanden die uit Verkenner / Finder / Bestanden zijn gekopieerd, via Copy That te plakken. CmdOrCtrl wordt Cmd op macOS en Ctrl op Windows / Linux.
settings-clipboard-watcher = Klembord in de gaten houden voor gekopieerde bestanden
settings-clipboard-watcher-hint = Toont een melding wanneer bestand-URL's op het klembord verschijnen, met de hint dat u via Copy That kunt plakken. Scant elke 500 ms wanneer ingeschakeld.

# MT
settings-buffer-size = Buffergrootte
# MT
settings-verify = Verifiëren na kopiëren
# MT
settings-verify-off = Uit
# MT
settings-concurrency = Gelijktijdigheid
# MT
settings-concurrency-auto = Automatisch
# MT
settings-reflink = Reflink / snelle paden
# MT
settings-reflink-prefer = Voorkeur
# MT
settings-reflink-avoid = Reflink vermijden
# MT
settings-reflink-disabled = Altijd async-engine gebruiken
# MT
settings-fsync-on-close = Synchroniseren naar schijf bij sluiten (trager, veiliger)
# MT
settings-preserve-timestamps = Tijdstempels behouden
# MT
settings-preserve-permissions = Rechten behouden
# MT
settings-preserve-acls = ACL's behouden (fase 14)
settings-preserve-sparseness = Sparse-bestanden behouden  # MT
settings-preserve-sparseness-hint = Kopieer alleen de toegewezen gebieden van sparse-bestanden (VM-schijven, databasebestanden) zodat de bestemming dezelfde grootte op schijf behoudt als de bron.  # MT

# MT
settings-context-menu = Contextmenu-items inschakelen
# MT
settings-intercept-copy = Standaard kopieerhandler onderscheppen (Windows)
# MT
settings-intercept-copy-hint = Indien aan loopt Ctrl+C / Ctrl+V in Verkenner via Copy That. Registratie in fase 14.
# MT
settings-notify-completion = Melding bij voltooiing taak

# MT
settings-shred-method = Standaard vernietigingsmethode
# MT
settings-shred-zero = Nul (1 passage)
# MT
settings-shred-random = Willekeurig (1 passage)
# MT
settings-shred-dod3 = DoD 5220.22-M (3 passages)
# MT
settings-shred-dod7 = DoD 5220.22-M (7 passages)
# MT
settings-shred-gutmann = Gutmann (35 passages)
# MT
settings-shred-nist = NIST 800-88
# MT
settings-shred-confirm-twice = Dubbele bevestiging vereisen voor vernietiging

# MT
settings-log-level = Logniveau
# MT
settings-log-off = Uit
# MT
settings-telemetry = Telemetrie
# MT
settings-telemetry-never = Nooit — geen data-uitwisseling op welk logniveau dan ook
# MT
settings-error-policy = Standaard foutbeleid
# MT
settings-error-policy-ask = Vragen
# MT
settings-error-policy-skip = Overslaan
# MT
settings-error-policy-retry = Opnieuw met wachttijd
# MT
settings-error-policy-abort = Afbreken bij eerste fout
# MT
settings-history-retention = Geschiedenisbehoud (dagen)
# MT
settings-history-retention-hint = 0 = voor altijd bewaren. Elke andere waarde purgeert oude taken bij opstarten.
# MT
settings-database-path = Databasepad
# MT
settings-database-path-default = (standaard — OS-datamap)
# MT
settings-reset-all = Herstellen naar standaard
# MT
settings-reset-confirm = Alle voorkeuren herstellen? Profielen blijven ongewijzigd.

# MT
settings-profiles-hint = Sla de huidige instellingen op onder een naam; laad later terug zonder individuele regelaars aan te raken.
# MT
settings-profile-name-placeholder = Profielnaam
# MT
settings-profile-save = Opslaan
# MT
settings-profile-import = Importeren…
# MT
settings-profile-load = Laden
# MT
settings-profile-export = Exporteren…
# MT
settings-profile-delete = Verwijderen
# MT
settings-profile-empty = Nog geen profielen opgeslagen.
# MT
settings-profile-import-prompt = Naam voor het geïmporteerde profiel:

# MT
toast-settings-reset = Instellingen hersteld
# MT
toast-profile-saved = Profiel opgeslagen
# MT
toast-profile-loaded = Profiel geladen
# MT
toast-profile-exported = Profiel geëxporteerd
# MT
toast-profile-imported = Profiel geïmporteerd

# Phase 13d — activity feed + header picker buttons
action-add-files = Bestanden toevoegen
action-add-folders = Mappen toevoegen
activity-title = Activiteit
activity-clear = Activiteitenlijst wissen
activity-empty = Nog geen bestandsactiviteit.
activity-after-done = Bij voltooiing:
activity-keep-open = App open laten
activity-close-app = App sluiten
activity-shutdown = PC afsluiten
activity-logoff = Afmelden
activity-sleep = Slaapstand

# Phase 14 — preflight free-space dialog
preflight-block-title = Onvoldoende ruimte op de bestemming
preflight-warn-title = Weinig ruimte op de bestemming
preflight-unknown-title = Vrije ruimte niet te bepalen
preflight-unknown-body = De bron is te groot om snel te meten of het doelvolume reageerde niet. Je kunt doorgaan; de beveiliging van de engine stopt het kopiëren netjes als de ruimte opraakt.
preflight-required = Vereist
preflight-free = Vrij
preflight-reserve = Reserve
preflight-shortfall = Tekort
preflight-continue = Toch doorgaan
collision-modal-overwrite-older = Alleen oudere overschrijven

# Phase 14e — subset picker
preflight-pick-subset = Kies wat gekopieerd wordt…
subset-title = Kies de bronnen om te kopiëren
subset-subtitle = De volledige selectie past niet op de bestemming. Vink aan wat je wilt kopiëren; de rest blijft achter.
subset-loading = Groottes meten…
subset-too-large = te groot om te tellen
subset-budget = Beschikbaar
subset-remaining = Resterend
subset-confirm = Selectie kopiëren
history-rerun-hint = Deze kopie opnieuw uitvoeren — scant alle bestanden in de bronboom opnieuw
history-clear-all = Alles wissen
history-clear-all-confirm = Klik opnieuw om te bevestigen
history-clear-all-hint = Verwijdert elke geschiedenisrij. Een tweede klik bevestigt.
toast-history-cleared = Geschiedenis gewist ({ $count } rijen verwijderd)

# Phase 15 — source-list ordering
drop-dialog-sort-label = Volgorde:
sort-custom = Aangepast
sort-name-asc = Naam A → Z (bestanden eerst)
sort-name-desc = Naam Z → A (bestanden eerst)
sort-size-asc = Grootte klein naar groot (bestanden eerst)
sort-size-desc = Grootte groot naar klein (bestanden eerst)
sort-reorder = Herschikken
sort-move-top = Naar boven
sort-move-up = Omhoog
sort-move-down = Omlaag
sort-move-bottom = Naar beneden
sort-name-asc-simple = Naam A → Z
sort-name-desc-simple = Naam Z → A
sort-size-asc-simple = Kleinste eerst
sort-size-desc-simple = Grootste eerst
activity-sort-locked = Sorteren is uitgeschakeld terwijl een kopie loopt. Pauzeer of wacht tot het klaar is en wijzig dan de volgorde.
drop-dialog-collision-label = Als een bestand al bestaat:
collision-policy-keep-both = Beide behouden (nieuwe kopie hernoemen naar _2, _3, …)
collision-policy-skip = Nieuwe kopie overslaan
collision-policy-overwrite = Bestaand bestand overschrijven
collision-policy-overwrite-if-newer = Alleen overschrijven als nieuwer
collision-policy-prompt = Elke keer vragen
drop-dialog-busy-checking = Vrije ruimte controleren…
drop-dialog-busy-enumerating = Bestanden tellen…
drop-dialog-busy-starting = Kopie starten…
toast-enumeration-deferred = Bronstructuur is groot — voorbereide lijst overgeslagen; regels verschijnen terwijl de engine ze verwerkt.

# Phase 14a — enumeration-time filters
# MT
settings-tab-filters = Filters
# MT
settings-filters-hint = Slaat bestanden over tijdens het opsommen, zodat de engine ze niet eens opent. Insluiten geldt alleen voor bestanden; uitsluiten snoeit ook overeenkomende mappen.
# MT
settings-filters-enabled = Filters inschakelen voor boomkopieën
# MT
settings-filters-include-globs = Insluit-globs
# MT
settings-filters-include-globs-placeholder = **/*.txt
# MT
settings-filters-include-globs-hint = Eén glob per regel. Indien niet leeg moet een bestand met ten minste één overeenkomen. Mappen worden altijd doorlopen.
# MT
settings-filters-exclude-globs = Uitsluit-globs
# MT
settings-filters-exclude-globs-placeholder = **/node_modules
# MT
settings-filters-exclude-globs-hint = Eén glob per regel. Treffers snoeien de hele subboom voor mappen; overeenkomende bestanden worden overgeslagen.
# MT
settings-filters-size-range = Bestandsgroottebereik
# MT
settings-filters-min-size-bytes = Minimum grootte (bytes, leeg = geen ondergrens)
# MT
settings-filters-max-size-bytes = Maximum grootte (bytes, leeg = geen bovengrens)
# MT
settings-filters-date-range = Wijzigingstijdbereik
# MT
settings-filters-min-mtime = Gewijzigd op of na
# MT
settings-filters-max-mtime = Gewijzigd op of voor
# MT
settings-filters-attributes = Attributen
# MT
settings-filters-skip-hidden = Verborgen bestanden / mappen overslaan
# MT
settings-filters-skip-system = Systeembestanden overslaan (alleen Windows)
# MT
settings-filters-skip-readonly = Alleen-lezen bestanden overslaan

# Phase 15 — auto-update
# MT
settings-tab-updater = Updates
# MT
settings-updater-hint = Copy That controleert ondertekende updates maximaal één keer per dag. Updates worden geïnstalleerd bij het volgende afsluiten van de app.
# MT
settings-updater-auto-check = Bij opstarten op updates controleren
# MT
settings-updater-channel = Release-kanaal
# MT
settings-updater-channel-stable = Stabiel
# MT
settings-updater-channel-beta = Beta (pre-release)
# MT
settings-updater-last-check = Laatst gecontroleerd
# MT
settings-updater-last-never = Nooit
# MT
settings-updater-check-now = Nu op updates controleren
# MT
settings-updater-checking = Bezig met controleren…
# MT
settings-updater-available = Update beschikbaar
# MT
settings-updater-up-to-date = Je gebruikt de nieuwste versie.
# MT
settings-updater-dismiss = Deze versie overslaan
# MT
settings-updater-dismissed = Overgeslagen
# MT
toast-update-available = Er is een nieuwere versie beschikbaar
# MT
toast-update-up-to-date = Je hebt al de nieuwste versie

# Phase 19a — disk-backed file enumeration (TeraCopy-compatible scan DB)
# MT
scan-progress-title = Bezig met scannen…
# MT
scan-progress-stats = { $files } bestanden · { $bytes } tot nu toe
# MT
scan-pause-button = Scan pauzeren
# MT
scan-resume-button = Scan hervatten
# MT
scan-cancel-button = Scan annuleren
# MT
scan-cancel-confirm = Scan annuleren en voortgang verwerpen?
# MT
scan-db-header = Scandatabase
# MT
scan-db-hint = Scandatabase op schijf voor taken met miljoenen bestanden.
# MT
advanced-scan-hash-during = Controlegetallen berekenen tijdens scan
# MT
advanced-scan-db-path = Locatie van scandatabase
# MT
advanced-scan-retention-days = Voltooide scans automatisch verwijderen na (dagen)
# MT
advanced-scan-max-keep = Maximum aantal te bewaren scandatabases

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
sparse-not-supported-title = Bestemming vult sparse-bestanden  # MT
sparse-not-supported-body = { $dst_fs } ondersteunt geen sparse-bestanden. Gaten in de bron zijn als nullen geschreven, dus de bestemming is groter op schijf.  # MT
sparse-warning-densified = Sparse-indeling behouden: alleen toegewezen gebieden zijn gekopieerd.  # MT
sparse-warning-mismatch = Sparse-indeling komt niet overeen — bestemming kan groter zijn dan verwacht.  # MT

# Phase 24 — security-metadata preservation. MT-flagged drafts;
# the authoritative English source lives in locales/en/copythat.ftl.
settings-preserve-security-metadata = Beveiligingsmetadata behouden  # MT
settings-preserve-security-metadata-hint = Leg buiten-band-metadatastromen (NTFS ADS / xattrs / POSIX ACL's / SELinux-contexten / Linux-bestandscapabilities / macOS-resourceforks) vast en pas ze opnieuw toe bij elke kopie.  # MT
settings-preserve-motw = Mark-of-the-Web (van-internet-gedownload-vlag) behouden  # MT
settings-preserve-motw-hint = Kritiek voor de beveiliging. SmartScreen en Office Protected View gebruiken deze stroom om te waarschuwen voor van internet gedownloade bestanden. Uitschakelen laat een gedownload uitvoerbaar bestand zijn oorsprongmarker verliezen bij het kopiëren en de beschermingen van het besturingssysteem omzeilen.  # MT
settings-preserve-posix-acls = POSIX ACL's en uitgebreide attributen behouden  # MT
settings-preserve-posix-acls-hint = Draag user.* / system.* / trusted.* xattrs en POSIX-toegangscontrolelijsten over tijdens het kopiëren.  # MT
settings-preserve-selinux = SELinux-contexten behouden  # MT
settings-preserve-selinux-hint = Draag het security.selinux-label over tijdens het kopiëren zodat daemons onder MAC-beleid het bestand kunnen blijven openen.  # MT
settings-preserve-resource-forks = macOS-resourceforks en Finder-info behouden  # MT
settings-preserve-resource-forks-hint = Draag de legacy resource-fork en FinderInfo (kleurtags, Carbon-metadata) over tijdens het kopiëren.  # MT
settings-appledouble-fallback = AppleDouble-sidecar gebruiken op incompatibele bestandssystemen  # MT
meta-translated-to-appledouble = Buitenlandse metadata opgeslagen in AppleDouble-sidecar (._{ $ext })  # MT
