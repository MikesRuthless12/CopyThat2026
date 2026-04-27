app-name = Copy That v1.25.0
# MT
window-title = Copy That v1.25.0
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
err-path-invalid-encoding = Path rejected — string contains invalid UTF-8 / replacement characters
# MT
err-helper-invalid-json = Privileged helper received malformed JSON; ignoring this request
err-helper-grant-out-of-band = GrantCapabilities must be handled by the helper run-loop, not the stateless handler
err-randomness-unavailable = OS random-number generator failed; cannot mint a session id
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

# Phase 25 — two-way sync with vector-clock conflict detection.
# MT-flagged drafts; the authoritative English source lives in
# locales/en/copythat.ftl.
footer-sync = Sync  # MT
sync-drawer-title = Tweewegsynchronisatie  # MT
sync-drawer-hint = Houd twee mappen gesynchroniseerd zonder stille overschrijvingen. Gelijktijdige bewerkingen verschijnen als oplosbare conflicten.  # MT
sync-add-pair = Paar toevoegen  # MT
sync-add-cancel = Annuleren  # MT
sync-refresh = Vernieuwen  # MT
sync-add-save = Paar opslaan  # MT
sync-add-saving = Opslaan…  # MT
sync-add-missing-fields = Label, linkerpad en rechterpad zijn allemaal vereist.  # MT
sync-remove-confirm = Dit synchronisatiepaar verwijderen? De toestandsdatabase blijft behouden; de mappen blijven onaangetast.  # MT
sync-field-label = Label  # MT
sync-field-label-placeholder = bijv. Documenten ↔ NAS  # MT
sync-field-left = Linkermap  # MT
sync-field-left-placeholder = Kies of plak een absoluut pad  # MT
sync-field-right = Rechtermap  # MT
sync-field-right-placeholder = Kies of plak een absoluut pad  # MT
sync-field-mode = Modus  # MT
sync-mode-two-way = Tweeweg  # MT
sync-mode-mirror-left-to-right = Spiegel (links → rechts)  # MT
sync-mode-mirror-right-to-left = Spiegel (rechts → links)  # MT
sync-mode-contribute-left-to-right = Bijdragen (links → rechts, geen verwijderingen)  # MT
sync-no-pairs = Nog geen synchronisatieparen geconfigureerd. Klik op "Paar toevoegen" om te beginnen.  # MT
sync-loading = Geconfigureerde paren laden…  # MT
sync-never-run = Nooit uitgevoerd  # MT
sync-running = Actief  # MT
sync-run-now = Nu uitvoeren  # MT
sync-cancel = Annuleren  # MT
sync-remove-pair = Verwijderen  # MT
sync-view-conflicts = Conflicten bekijken ({ $count })  # MT
sync-conflicts-heading = Conflicten  # MT
sync-no-conflicts = Geen conflicten van de laatste run.  # MT
sync-winner = Winnaar  # MT
sync-side-left-to-right = links  # MT
sync-side-right-to-left = rechts  # MT
sync-conflict-kind-concurrent-write = Gelijktijdige bewerking  # MT
sync-conflict-kind-delete-edit = Verwijderen ↔ bewerken  # MT
sync-conflict-kind-add-add = Beide kanten hebben toegevoegd  # MT
sync-conflict-kind-corrupt-equal = Inhoud is divergiert zonder nieuwe schrijfactie  # MT
sync-resolve-keep-left = Links behouden  # MT
sync-resolve-keep-right = Rechts behouden  # MT
sync-resolve-keep-both = Beide behouden  # MT
sync-resolve-three-way = Oplossen via 3-wegs samenvoegen  # MT
sync-resolve-phase-53-tooltip = Interactieve 3-wegs samenvoeging voor niet-tekstbestanden komt in fase 53.  # MT
sync-error-prefix = Synchronisatiefout  # MT

# Phase 26 — real-time mirror watcher. MT-flagged drafts;
# the authoritative English source lives in locales/en/copythat.ftl.
live-mirror-start = Live mirror starten  # MT
live-mirror-stop = Live mirror stoppen  # MT
live-mirror-watching = Kijken  # MT
live-mirror-toggle-hint = Synchroniseer automatisch opnieuw bij elke gedetecteerde wijziging in het bestandssysteem. Eén achtergrondthread per actief paar.  # MT
watch-event-prefix = Bestandswijziging  # MT
watch-overflow-recovered = Buffer van waarnemer liep over; opnieuw opsommen om te herstellen  # MT

# Phase 27 — content-defined chunk store. MT-flagged drafts;
# the authoritative English source lives in locales/en/copythat.ftl.
chunk-store-section = Chunk-opslag  # MT
chunk-store-enable = Chunk-opslag inschakelen (delta-hervatting en deduplicatie)  # MT
chunk-store-enable-hint = Splitst elk gekopieerd bestand op inhoud (FastCDC) en slaat chunks inhoudsgeadresseerd op. Herhalingen herschrijven alleen gewijzigde chunks; bestanden met gedeelde inhoud worden automatisch gededupliceerd.  # MT
chunk-store-location = Locatie van chunk-opslag  # MT
chunk-store-max-size = Maximale grootte van chunk-opslag  # MT
chunk-store-prune = Chunks ouder dan (dagen) opruimen  # MT
chunk-store-savings = { $gib } GiB bespaard via chunk-deduplicatie  # MT
chunk-store-disk-usage = Gebruikt { $size } over { $chunks } chunks  # MT

# Phase 28 — tray-resident Drop Stack. MT-flagged drafts;
# the authoritative English source lives in locales/en/copythat.ftl.
dropstack-window-title = Drop Stack  # MT
dropstack-tray-open = Drop Stack  # MT
dropstack-empty-title = Drop Stack is leeg  # MT
dropstack-empty-hint = Sleep bestanden hier vanuit Verkenner of klik met de rechtermuisknop op een taakrij om deze toe te voegen.  # MT
dropstack-add-to-stack = Toevoegen aan Drop Stack  # MT
dropstack-copy-all-to = Alles kopiëren naar…  # MT
dropstack-move-all-to = Alles verplaatsen naar…  # MT
dropstack-clear = Stack wissen  # MT
dropstack-remove-row = Uit stack verwijderen  # MT
dropstack-path-missing-toast = { $path } verwijderd — het bestand bestaat niet meer.  # MT
dropstack-always-on-top = Drop Stack altijd op voorgrond houden  # MT
dropstack-show-tray-icon = Copy That-systeemvakpictogram weergeven  # MT
dropstack-open-on-start = Drop Stack automatisch openen bij app-start  # MT
dropstack-count = { $count } pad  # MT

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
phase42-paranoid-verify-label = Paranoid verify
phase42-paranoid-verify-hint = Drops the destination's cached pages and re-reads from disk to catch write-cache lies and silent corruption. About 50% slower than the default verify; off by default.
phase42-sharing-violation-retries-label = Retry attempts on locked source files
phase42-sharing-violation-retries-hint = How many times to retry when another process is holding the source file open with an exclusive lock. Backoff doubles each attempt (50 ms / 100 ms / 200 ms by default). Default 3, matching Robocopy /R:3.
phase42-cloud-placeholder-warning = { $name } is a cloud-only OneDrive file. Copying it will trigger a download — up to { $size } over your network connection.
phase42-defender-exclusion-hint = For maximum copy throughput, add the destination folder to Microsoft Defender exclusions before bulk transfers. See docs/PERFORMANCE_TUNING.md.
