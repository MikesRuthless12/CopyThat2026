app-name = Freally File Manager v0.19.85
window-title = Freally File Manager v0.19.85
shred-ssd-advisory = Waarschuwing: dit doel staat op een SSD. Meerdere overschrijfdoorgangen wissen flashgeheugen niet betrouwbaar, omdat wear-leveling en over-provisioning gegevens onder het logische blokadres vandaan verplaatsen. Gebruik voor solid-state media liever ATA SECURE ERASE, NVMe Format met Secure Erase of volledige-schijfversleuteling met een weggegooide sleutel.

# Global aggregate states (header pill)
state-idle = Inactief
state-copying = Bezig met kopiëren
state-verifying = Bezig met verifiëren
state-paused = Gepauzeerd
state-error = Fout

# Per-job states (row badge)
state-pending = In wachtrij
state-running = Actief
state-cancelled = Geannuleerd
state-succeeded = Voltooid
state-failed = Mislukt

# Actions
action-pause = Pauzeren
action-resume = Hervatten
action-cancel = Annuleren
action-pause-all = Alle taken pauzeren
action-resume-all = Alle taken hervatten
action-cancel-all = Alle taken annuleren
action-close = Sluiten
action-reveal = Tonen in map
action-add-files = Bestanden toevoegen
action-add-folders = Mappen toevoegen

# Phase 13d — activity feed
activity-title = Activiteit
activity-clear = Activiteitenlijst wissen
activity-empty = Nog geen bestandsactiviteit.
activity-after-done = Wanneer klaar:
activity-keep-open = App geopend houden
activity-close-app = App sluiten
activity-shutdown = Pc afsluiten
activity-logoff = Afmelden
activity-sleep = Slaapstand

# Phase 14 — preflight free-space dialog
preflight-block-title = Onvoldoende ruimte op de bestemming
preflight-warn-title = Weinig ruimte op de bestemming
preflight-unknown-title = Vrije ruimte kon niet worden bepaald
preflight-unknown-body = De bron is te groot om snel te meten of het bestemmingsvolume reageerde niet. Je kunt doorgaan; de ruimtebewaking van de engine stopt het kopiëren netjes als de ruimte opraakt.
preflight-required = Vereist
preflight-free = Vrij
preflight-reserve = Reserve
preflight-shortfall = Tekort
preflight-continue = Toch doorgaan
preflight-pick-subset = Selecteer wat je wilt kopiëren…
collision-modal-overwrite-older = Alleen oudere overschrijven

# Phase 14e — subset picker
subset-title = Kies welke bronnen je wilt kopiëren
subset-subtitle = De volledige selectie past niet op de bestemming. Vink de items aan die je wilt kopiëren; de rest blijft achter.
subset-loading = Groottes meten…
subset-too-large = te groot om te tellen
subset-budget = Beschikbaar
subset-remaining = Resterend
subset-confirm = Selectie kopiëren
history-rerun-hint = Deze kopie opnieuw uitvoeren — scant elk bestand in de bronstructuur opnieuw
history-clear-all = Alles wissen
history-clear-all-confirm = Klik nogmaals om te bevestigen
history-clear-all-hint = Verwijder elke geschiedenisregel. Vereist een tweede klik ter bevestiging.
toast-history-cleared = Geschiedenis gewist ({ $count } regels verwijderd)

# Phase 15 — source-list ordering in the drop staging dialog
drop-dialog-sort-label = Volgorde:
sort-custom = Aangepast
sort-name-asc = Naam A → Z (bestanden eerst)
sort-name-desc = Naam Z → A (bestanden eerst)
sort-size-asc = Grootte kleinste eerst (bestanden eerst)
sort-size-desc = Grootte grootste eerst (bestanden eerst)
sort-reorder = Herordenen
sort-move-top = Naar boven verplaatsen
sort-move-up = Omhoog verplaatsen
sort-move-down = Omlaag verplaatsen
sort-move-bottom = Naar onder verplaatsen

# Phase 16 — sort preset names for the Activity list (shorter
# wording than the DropStagingDialog variants; the Activity list
# is files-only so the "(files first)" suffix isn't relevant).
sort-name-asc-simple = Naam A → Z
sort-name-desc-simple = Naam Z → A
sort-size-asc-simple = Grootte kleinste eerst
sort-size-desc-simple = Grootte grootste eerst
activity-sort-locked = Sorteren is uitgeschakeld terwijl een kopie loopt. Pauzeer of wacht tot deze klaar is en wijzig dan de volgorde.

# Phase 17 — collision-policy picker in the DropStagingDialog
drop-dialog-collision-label = Als een bestand al bestaat:
collision-policy-keep-both = Beide behouden (nieuwe kopie hernoemen naar _2, _3, …)
collision-policy-skip = Nieuwe kopie overslaan
collision-policy-overwrite = Bestaand bestand overschrijven
collision-policy-overwrite-if-newer = Alleen overschrijven als nieuwer
collision-policy-prompt = Elke keer vragen

# Phase 18 — progress labels on the DropStagingDialog Start button
drop-dialog-busy-checking = Vrije ruimte controleren…
drop-dialog-busy-enumerating = Bestanden tellen…
drop-dialog-busy-starting = Kopiëren starten…
toast-enumeration-deferred = Bronstructuur is groot — vooraf opstellen van bestandslijst wordt overgeslagen; regels verschijnen terwijl de engine ze verwerkt.

# Context menu (per-row right-click)
menu-pause = Pauzeren
menu-resume = Hervatten
menu-cancel = Annuleren
menu-remove = Uit wachtrij verwijderen
menu-reveal-source = Bron tonen in map
menu-reveal-destination = Bestemming tonen in map

# Header / toolbar
header-eta-label = Geschatte resterende tijd
header-toolbar-label = Algemene besturing

# Footer
footer-queued = actieve taken
footer-total-bytes = onderweg
footer-errors = fouten
footer-history = Geschiedenis

# Empty state
empty-title = Sleep bestanden of mappen hierheen om te kopiëren
empty-hint = Sleep items op het venster. We vragen om een bestemming en plaatsen dan één taak per bron in de wachtrij.
empty-region-label = Takenlijst

# Details drawer
details-drawer-label = Taakdetails
details-source = Bron
details-destination = Bestemming
details-state = Status
details-bytes = Bytes
details-files = Bestanden
details-speed = Snelheid
details-eta = Resterende tijd
details-error = Fout

# Drop dialog
drop-dialog-title = Gesleepte items overdragen
drop-dialog-subtitle = { $count } item(s) klaar om over te dragen. Kies een bestemmingsmap om te beginnen.
drop-dialog-mode = Bewerking
drop-dialog-copy = Kopiëren
drop-dialog-move = Verplaatsen
drop-dialog-pick-destination = Bestemming kiezen
drop-dialog-change-destination = Bestemming wijzigen
drop-dialog-start-copy = Kopiëren starten
drop-dialog-start-move = Verplaatsen starten

# ETA placeholders
eta-calculating = berekenen…
eta-unknown = onbekend

# Toast messages
toast-job-done = Overdracht voltooid
toast-copy-queued = Kopie in wachtrij
toast-move-queued = Verplaatsing in wachtrij
toast-error-resolved = Fout opgelost
toast-collision-resolved = Conflict opgelost
toast-elevated-unavailable = Opnieuw proberen met verhoogde rechten komt in fase 17 — nog niet beschikbaar
toast-clipboard-files-detected = Bestanden op klembord — druk op je plaksneltoets om te kopiëren via Freally File Manager
toast-clipboard-no-files = Klembord bevat geen bestanden om te plakken
toast-error-log-exported = Foutenlogboek geëxporteerd

# Error modal (Phase 8)
error-modal-title = Een overdracht is mislukt
error-modal-retry = Opnieuw proberen
error-modal-retry-elevated = Opnieuw proberen met verhoogde rechten
error-modal-skip = Overslaan
error-modal-skip-all-kind = Alle fouten van dit soort overslaan
error-modal-abort = Alles afbreken
error-modal-path-label = Pad
error-modal-code-label = Code
error-drawer-pending-count = Meer fouten in wachtrij
error-drawer-toggle = Inklappen of uitklappen

# Error-kind labels (Phase 8). Source of truth — engine maps each
# `CopyErrorKind` to one of these keys via `localized_key()`.
err-not-found = Bestand niet gevonden
err-permission-denied = Toegang geweigerd
err-disk-full = Bestemmingsschijf is vol
err-interrupted = Bewerking onderbroken
err-verify-failed = Verificatie na kopiëren mislukt
err-path-escape = Pad geweigerd — bevat segmenten naar de bovenliggende map (..) of ongeldige bytes
err-path-invalid-encoding = Pad geweigerd — tekenreeks bevat ongeldige UTF-8 / vervangingstekens
err-helper-invalid-json = Bevoegde helper ontving onjuist gevormde JSON; dit verzoek wordt genegeerd
err-helper-grant-out-of-band = GrantCapabilities moet door de run-loop van de helper worden afgehandeld, niet door de stateless handler
err-randomness-unavailable = Generator voor willekeurige getallen van het besturingssysteem is mislukt; kan geen sessie-id aanmaken
err-sparseness-mismatch = Sparse-indeling kon niet worden behouden op de bestemming
err-io-other = Onbekende I/O-fout

# Collision modal (Phase 8)
collision-modal-title = Bestand bestaat al
collision-modal-overwrite = Overschrijven
collision-modal-overwrite-if-newer = Overschrijven als nieuwer
collision-modal-skip = Overslaan
collision-modal-keep-both = Beide behouden
collision-modal-rename = Hernoemen…
collision-modal-apply-to-all = Op alles toepassen
collision-modal-source = Bron
collision-modal-destination = Bestemming
collision-modal-size = Grootte
collision-modal-modified = Gewijzigd
collision-modal-hash-check = Snelle hash (SHA-256)
collision-modal-hash-computing = Berekenen…
collision-modal-hash-identical = Identiek
collision-modal-hash-different = Verschillend
collision-modal-rename-placeholder = Nieuwe bestandsnaam
collision-modal-confirm-rename = Hernoemen

# Error log drawer (Phase 8)
error-log-title = Foutenlogboek
error-log-empty = Geen fouten gelogd
error-log-export-csv = CSV exporteren
error-log-export-txt = Tekst exporteren
error-log-clear = Logboek wissen
error-log-col-time = Tijd
error-log-col-job = Taak
error-log-col-path = Pad
error-log-col-code = Code
error-log-col-message = Bericht
error-log-col-resolution = Oplossing

# History drawer (Phase 9)
history-title = Geschiedenis
history-empty = Nog geen taken vastgelegd
history-unavailable = Kopieergeschiedenis is niet beschikbaar. De app kon de SQLite-opslag bij het opstarten niet openen.
history-filter-any = alle
history-filter-kind = Soort
history-filter-status = Status
history-filter-text = Zoeken
history-refresh = Vernieuwen
history-export-csv = CSV exporteren
history-purge-30 = Ouder dan 30 dagen wissen
history-rerun = Opnieuw uitvoeren
history-detail-open = Details
history-detail-title = Taakdetails
history-detail-empty = Geen items vastgelegd
history-col-date = Datum
history-col-kind = Soort
history-col-src = Bron
history-col-dst = Bestemming
history-col-files = Bestanden
history-col-size = Grootte
history-col-status = Status
history-col-duration = Duur
history-col-error = Fout
toast-history-exported = Geschiedenis geëxporteerd
toast-history-rerun-queued = Opnieuw uitvoeren in wachtrij

# Totals drawer (Phase 10)
footer-totals = Totalen
totals-title = Totalen
totals-loading = Totalen laden…
totals-card-bytes = Totaal aantal gekopieerde bytes
totals-card-files = Bestanden
totals-card-jobs = Taken
totals-card-avg-rate = Gemiddelde doorvoer
totals-errors = fouten
totals-spark-title = Laatste 30 dagen
totals-kinds-title = Op soort
totals-saved-title = Bespaarde tijd (geschat)
totals-saved-note = Geschat ten opzichte van een standaard bestandsbeheerder die dezelfde werklast kopieert.
totals-reset = Statistieken resetten
totals-reset-confirm = Dit verwijdert elke opgeslagen taak en elk item. Doorgaan?
totals-reset-confirm-yes = Ja, resetten
toast-totals-reset = Statistieken gereset

# Phase 11a — i18n core: surface remaining user-visible strings so
# every string on a main-window screen flows through Fluent.

# Header language switcher (temporary placement — folds into
# Settings → General in Phase 12).
header-language-label = Taal
header-language-title = Taal wijzigen

# Job-kind labels. History filter, Totals breakdown, and history
# rows all map wire-format `kind` strings to these labels.
kind-copy = Kopiëren
kind-move = Verplaatsen
kind-delete = Verwijderen
kind-secure-delete = Veilig verwijderen

# History status labels. Distinct from `state-*` because the
# history wire format uses plain `running`/`succeeded`/... rather
# than the live `JobState` enum the queue exposes.
status-running = Actief
status-succeeded = Geslaagd
status-failed = Mislukt
status-cancelled = Geannuleerd
# Per-item status (not job-level): `ok` / `skipped` are only
# reachable from the history detail view.
status-ok = OK
status-skipped = Overgeslagen

# History drawer: search field placeholder + purge toast.
history-search-placeholder = /pad
toast-history-purged = { $count } taken ouder dan 30 dagen gewist

# User-facing command-layer validation errors. The Rust side
# returns these keys when input is missing; the toast layer looks
# them up rather than showing the raw English.
err-source-required = Er is ten minste één bronpad vereist.
err-destination-empty = Bestemmingspad is leeg.
err-source-empty = Bronpad is leeg.

# Localised duration formatting for the Totals drawer and ETA
# fields. `{ $ms }`, `{ $s }`, `{ $m }`, `{ $h }` are integer
# placeables — the formatter passes pre-computed values in.
duration-lt-1s = < 1s
duration-ms = { $ms } ms
duration-seconds = { $s }s
duration-minutes-seconds = { $m }m { $s }s
duration-hours-minutes = { $h }u { $m }m
duration-zero = 0s

# Rate unit. Appended to a formatted byte size. Some languages
# render this with a leading space ("Ko/s"); keep it translatable
# even though the SI-derived "/s" is near-universal.
rate-unit-per-second = { $size }/s

# Phase 11b — Settings modal skeleton. Phase 12 expanded this into a
# full six-tab preferences window; the `settings-phase-12-hint` key
# is retired in favour of `settings-tab-profiles` + concrete labels.
settings-title = Instellingen
settings-tab-general = Algemeen
settings-tab-appearance = Weergave
settings-section-language = Taal
settings-phase-12-hint = Meer instellingen (thema, standaardwaarden voor overdracht, verificatiealgoritme, profielen) komen in fase 12.

# Phase 12 — full Settings window keys. Grouped by tab so the
# human-review pass can pick one section at a time. Every field
# label and every option label that the user will actually read
# has its own key; dynamic wire values (enum discriminants like
# "auto" / "prefer") stay untranslated on the wire.

settings-loading = Instellingen laden…
settings-tab-transfer = Overdracht
settings-tab-filters = Filters
settings-tab-shell = Shell
settings-tab-secure-delete = Veilig verwijderen
settings-tab-advanced = Geavanceerd
settings-tab-updater = Updates
settings-tab-profiles = Profielen

# General tab additions
settings-section-theme = Thema
settings-theme-auto = Automatisch
settings-theme-light = Licht
settings-theme-dark = Donker
settings-start-with-os = Starten bij opstarten van systeem
settings-single-instance = Eén actieve instantie
settings-minimize-to-tray = Bij sluiten minimaliseren naar systeemvak
settings-error-display-mode = Stijl van foutmelding
settings-error-display-modal = Modaal (blokkeert de app)
settings-error-display-drawer = Lade (blokkeert niet)
settings-error-display-mode-hint = Modaal stopt de wachtrij totdat je beslist. De lade houdt de wachtrij in beweging en laat je fouten in de hoek afhandelen.
settings-paste-shortcut = Bestanden plakken via globale sneltoets
settings-paste-shortcut-combo = Sneltoetscombinatie
settings-paste-shortcut-hint = Druk deze combinatie ergens op je systeem in om bestanden te plakken die zijn gekopieerd uit Explorer / Finder / Files via Freally File Manager. CmdOrCtrl wordt Cmd op macOS, Ctrl op Windows / Linux.
settings-clipboard-watcher = Klembord bewaken op gekopieerde bestanden
settings-clipboard-watcher-hint = Toon een melding wanneer bestands-URL's op het klembord verschijnen, met de hint dat je via Freally File Manager kunt plakken. Controleert elke 500 ms wanneer ingeschakeld.

# Transfer tab
settings-buffer-size = Buffergrootte
settings-verify = Verifiëren na kopiëren
settings-verify-off = Uit
settings-concurrency = Gelijktijdigheid
settings-concurrency-auto = Automatisch
settings-reflink = Reflink / snelle paden
settings-reflink-prefer = Voorkeur
settings-reflink-avoid = Reflink vermijden
settings-reflink-disabled = Altijd async-engine gebruiken
settings-fsync-on-close = Bij sluiten naar schijf synchroniseren (trager, veiliger)
settings-preserve-timestamps = Tijdstempels behouden
settings-preserve-permissions = Rechten behouden
settings-preserve-acls = ACL's behouden (fase 14)
settings-preserve-sparseness = Sparse-bestanden behouden
settings-preserve-sparseness-hint = Kopieer alleen de toegewezen extents van sparse-bestanden (vm-schijven, databasebestanden) zodat de bestemming op schijf dezelfde grootte houdt als de bron.
settings-force-parallel-chunks = Parallelle multi-chunk kopie (alleen RAID / arrays)
settings-force-parallel-chunks-hint = Splitst elke grote kopie in gelijktijdige chunks. Helpt alleen bij gestripte/RAID/netwerkbestemmingen; VERTRAAGT een enkele SSD/NVMe (-25% tot -76%). Laat uit tenzij de bestemming een array met meerdere schijven is.

# Shell tab
settings-context-menu = Items in shell-contextmenu inschakelen
settings-intercept-copy = Standaard kopieerafhandeling onderscheppen (Windows)
settings-intercept-copy-hint = Wanneer ingeschakeld, lopen Ctrl+C / Ctrl+V van Explorer via Freally File Manager. Registratie komt in fase 14.
settings-notify-completion = Melden bij voltooiing van taak

# Secure delete tab
settings-shred-method = Standaard shred-methode
settings-shred-zero = Nul (1 doorgang)
settings-shred-random = Willekeurig (1 doorgang)
settings-shred-dod3 = DoD 5220.22-M (3 doorgangen)
settings-shred-dod7 = DoD 5220.22-M (7 doorgangen)
settings-shred-gutmann = Gutmann (35 doorgangen)
settings-shred-nist = NIST 800-88
settings-shred-confirm-twice = Dubbele bevestiging vereisen voor het shredden

# Advanced tab
settings-log-level = Logniveau
settings-log-off = Uit
settings-telemetry = Telemetrie
settings-telemetry-never = Nooit — geen verzending naar huis op enig logniveau
settings-error-policy = Standaard foutbeleid
settings-error-policy-ask = Vragen
settings-error-policy-skip = Overslaan
settings-error-policy-retry = Opnieuw proberen met wachttijd
settings-error-policy-abort = Afbreken bij eerste fout
settings-history-retention = Geschiedenisbewaring (dagen)
settings-history-retention-hint = 0 = voor altijd bewaren. Elke andere waarde wist oudere taken automatisch bij het opstarten.
settings-database-path = Databasepad
settings-database-path-default = (standaard — gegevensmap van besturingssysteem)
settings-reset-all = Terugzetten naar standaard
settings-reset-confirm = Elke voorkeur terugzetten naar de standaard? Profielen blijven ongewijzigd.

# Profiles tab
settings-profiles-hint = Sla de huidige instellingen op onder een naam; laad deze later om terug te schakelen zonder afzonderlijke knoppen aan te raken.
settings-profile-name-placeholder = Profielnaam
settings-profile-save = Opslaan
settings-profile-import = Importeren…
settings-profile-load = Laden
settings-profile-export = Exporteren…
settings-profile-delete = Verwijderen
settings-profile-empty = Nog geen profielen opgeslagen.
settings-profile-import-prompt = Naam voor het geïmporteerde profiel:

# Toasts driven by Phase 12 profile actions
toast-settings-reset = Instellingen gereset
toast-profile-saved = Profiel opgeslagen
toast-profile-loaded = Profiel geladen
toast-profile-exported = Profiel geëxporteerd
toast-profile-imported = Profiel geïmporteerd

# Phase 14a — enumeration-time filters
settings-filters-hint = Sla bestanden over tijdens het opstellen, zodat de engine ze nooit opent. Insluitingen gelden alleen voor bestanden; uitsluitingen snoeien ook overeenkomende mappen.
settings-filters-enabled = Filters inschakelen voor structuurkopieën
settings-filters-include-globs = Insluitings-globs
settings-filters-include-globs-placeholder = **/*.txt
settings-filters-include-globs-hint = Eén glob per regel. Wanneer niet leeg, moet een bestand op minstens één insluiting passen om te overleven. Mappen worden altijd betreden.
settings-filters-exclude-globs = Uitsluitings-globs
settings-filters-exclude-globs-placeholder = **/node_modules
settings-filters-exclude-globs-hint = Eén glob per regel. Bij overeenkomst wordt de hele substructuur van een map gesnoeid; overeenkomende bestanden worden overgeslagen.
settings-filters-size-range = Bereik bestandsgrootte
settings-filters-min-size-bytes = Minimale grootte (bytes, leeg = geen ondergrens)
settings-filters-max-size-bytes = Maximale grootte (bytes, leeg = geen bovengrens)
settings-filters-date-range = Bereik wijzigingstijd
settings-filters-min-mtime = Gewijzigd op of na
settings-filters-max-mtime = Gewijzigd op of voor
settings-filters-attributes = Attribuutbits
settings-filters-skip-hidden = Verborgen bestanden / mappen overslaan
settings-filters-skip-system = Systeembestanden overslaan (alleen Windows)
settings-filters-skip-readonly = Alleen-lezen bestanden overslaan

# Phase 15 — auto-update
settings-updater-hint = Freally File Manager controleert maximaal eenmaal per dag op ondertekende updates. Updates worden geïnstalleerd bij het volgende afsluiten van de app.
settings-updater-auto-check = Bij opstarten controleren op updates
settings-updater-channel = Releasekanaal
settings-updater-channel-stable = Stabiel
settings-updater-channel-beta = Beta (pre-release)
settings-updater-last-check = Laatst gecontroleerd
settings-updater-last-never = Nooit
settings-updater-check-now = Nu controleren op updates
settings-updater-checking = Controleren…
settings-updater-available = Update beschikbaar
settings-updater-up-to-date = Je gebruikt de nieuwste release.
settings-updater-dismiss = Deze versie overslaan
settings-updater-dismissed = Overgeslagen
toast-update-available = Er is een nieuwere versie beschikbaar
toast-update-up-to-date = Je gebruikt al de nieuwste versie

# Phase 19a — disk-backed file enumeration (TeraCopy-compatible scan DB)
scan-progress-title = Scannen…
scan-progress-stats = { $files } bestanden · { $bytes } tot nu toe
scan-pause-button = Scan pauzeren
scan-resume-button = Scan hervatten
scan-cancel-button = Scan annuleren
scan-cancel-confirm = Scan annuleren en voortgang verwerpen?
scan-db-header = Scandatabase
scan-db-hint = Scandatabase op schijf voor taken met miljoenen bestanden.
advanced-scan-hash-during = Controlesommen berekenen tijdens scan
advanced-scan-db-path = Locatie van scandatabase
advanced-scan-retention-days = Voltooide scans automatisch verwijderen na (dagen)
advanced-scan-max-keep = Maximaal aantal te bewaren scandatabases

# Phase 19b — filesystem-snapshot source for locked files.
settings-on-locked = Wanneer een bestand vergrendeld is
settings-on-locked-ask = De eerste keer vragen
settings-on-locked-retry = Kort opnieuw proberen en daarna de fout tonen
settings-on-locked-skip = Het vergrendelde bestand overslaan
settings-on-locked-snapshot = Een bestandssysteem-snapshot gebruiken
settings-on-locked-hint = Voorkom "bestand in gebruik door een ander proces"-fouten. Freally File Manager maakt een snapshot van het bronvolume (VSS op Windows, ZFS/Btrfs op Linux, APFS op macOS) en leest uit de snapshotkopie.
snapshot-prompt-title = Dit bestand wordt gebruikt door een ander proces
snapshot-prompt-body = Een ander programma heeft { $path } geopend met exclusieve schrijftoegang. Kies hoe Freally File Manager dit en soortgelijke bestanden op hetzelfde volume moet afhandelen.
snapshot-source-active = 📷 Lezen uit { $kind }-snapshot van { $volume }
snapshot-create-failed = Kon geen snapshot van het bronvolume maken
snapshot-vss-needs-elevation = Lezen uit een VSS-snapshot vereist beheerdersrechten. Freally File Manager vraagt je dit toe te staan.
snapshot-cleanup-failed = De snapshothelper meldde een opschoonfout — er kan een achtergebleven schaduwkopie op het volume staan.

# Phase 20 — durable resume journal.
resume-prompt-title = Eerdere overdrachten hervatten?
resume-prompt-body = Freally File Manager heeft { $count } onvoltooide overdracht(en) van een vorige sessie gedetecteerd. Kies wat je met elke wilt doen.
resume-prompt-resume = Hervatten
resume-prompt-resume-all = Alles hervatten
resume-discard-one = Niet hervatten
resume-discard-all = Alles verwerpen
resume-aborted-hash-mismatch = De eerste { $offset } bytes van de bestemming komen niet overeen met de bron — opnieuw beginnen vanaf het begin.
settings-auto-resume = Onderbroken taken automatisch hervatten zonder vragen
settings-auto-resume-hint = Sla de hervatprompt bij het opstarten over en zet stilletjes elke onvoltooide taak opnieuw in de wachtrij. Standaard uit.

# Phase 21 — bandwidth shaping (GCRA token bucket + schedule + auto-throttle).
settings-tab-network = Netwerk
settings-network-hint = Beperk je overdrachtssnelheid zodat de rest van het netwerk bruikbaar blijft. Pas dit globaal toe, volg een dagschema of reageer automatisch op gemeten wifi-, batterij- of mobiele verbindingen.
settings-network-mode = Bandbreedtelimiet
settings-network-mode-off = Uit (geen limiet)
settings-network-mode-fixed = Vaste waarde
settings-network-mode-schedule = Schema gebruiken
settings-network-cap-mbps = Limiet (MB/s)
settings-network-schedule = Schema (rclone-formaat)
settings-network-schedule-hint = Door spaties gescheiden HH:MM,snelheid-grenzen plus optionele Mon-Fri,snelheid-dagregels. Snelheden: 512k, 10M, 2G, off, unlimited. Voorbeeld: 08:00,512k 18:00,10M Sat-Sun,unlimited.
settings-network-auto-header = Automatisch beperken
settings-network-auto-metered = Op gemeten wifi
settings-network-auto-battery = Op batterij
settings-network-auto-cellular = Op mobiel netwerk
settings-network-auto-unchanged = Niet overschrijven
settings-network-auto-pause = Overdrachten pauzeren
settings-network-auto-cap = Beperken tot vaste waarde
shape-badge-paused = gepauzeerd
shape-badge-tooltip = Bandbreedtelimiet actief — klik om Instellingen → Netwerk te openen
shape-badge-source-schedule = gepland
shape-badge-source-metered = gemeten
shape-badge-source-battery = op batterij
shape-badge-source-cellular = mobiel
shape-badge-source-settings = actief
shape-error-schedule-invalid = Schemaformaat is ongeldig: { $message }

# Phase 22 — aggregate conflict dialog v2 (thumbnails, per-pattern
# rules, and reusable conflict profiles). Every key below is user-
# visible text in the `ConflictBatchModal.svelte` component.
conflict-batch-title = { $count } bestandsconflicten in { $jobname }
conflict-batch-state-pending = In behandeling
conflict-batch-state-resolved = Opgelost
conflict-batch-action-overwrite = Overschrijven
conflict-batch-action-skip = Overslaan
conflict-batch-action-keep-both = Beide behouden
conflict-batch-action-newer-wins = Nieuwere wint
conflict-batch-action-larger-wins = Grotere wint
conflict-batch-bulk-apply-selected = Toepassen op selectie
conflict-batch-bulk-apply-extension = Toepassen op alle van deze extensie
conflict-batch-bulk-apply-glob = Toepassen op overeenkomende glob…
conflict-batch-bulk-apply-remaining = Toepassen op alle resterende
conflict-batch-bulk-glob-placeholder = bijv. **/*.tmp
conflict-batch-save-profile = Deze regels opslaan als profiel…
conflict-batch-profile-placeholder = Profielnaam
conflict-batch-matched-rule = via regel '{ $rule }' → { $action }
conflict-batch-empty = Alle conflicten opgelost
conflict-batch-source-vs-destination = Bron vs. bestemming
conflict-batch-source-label = Bron
conflict-batch-destination-label = Bestemming
conflict-batch-size-label = Grootte
conflict-batch-modified-label = Gewijzigd
conflict-batch-close = Sluiten
conflict-batch-profile-saved = Conflictprofiel opgeslagen

# Phase 23 — sparse-file preservation. The toast fires once per
# destination volume when the filesystem can't preserve holes; the
# warning line is surfaced in the job detail drawer so the user knows
# the dst is larger on disk than the source was.
sparse-not-supported-title = Bestemming vult sparse-bestanden op
sparse-not-supported-body = { $dst_fs } ondersteunt geen sparse-bestanden. Gaten in de bron zijn als nullen weggeschreven, dus de bestemming is groter op schijf.
sparse-warning-densified = Sparse-indeling behouden: alleen toegewezen extents zijn gekopieerd.
sparse-warning-mismatch = Verschil in sparse-indeling — bestemming kan groter zijn dan verwacht.

# Phase 24 — security-metadata preservation. The Mark-of-the-Web
# (Zone.Identifier ADS) toggle is security-sensitive: turning it off
# lets a downloaded executable shed its SmartScreen / Office Protected
# View flag on copy, which is why the tooltip carries an explicit
# warning. AppleDouble fallback emits `._<filename>` sidecars on
# destination filesystems that can't hold the foreign metadata.
settings-preserve-security-metadata = Beveiligingsmetadata behouden
settings-preserve-security-metadata-hint = Leg bij elke kopie out-of-band metadatastromen vast en pas ze opnieuw toe (NTFS ADS / xattrs / POSIX-ACL's / SELinux-contexten / Linux-bestandscapabilities / macOS-resource forks).
settings-preserve-motw = Mark-of-the-Web behouden (vlag voor gedownload-van-internet)
settings-preserve-motw-hint = Cruciaal voor de beveiliging. SmartScreen en Office Protected View gebruiken deze stroom om te waarschuwen voor van internet gedownloade bestanden. Uitschakelen laat een gedownload uitvoerbaar bestand zijn herkomstmarkering bij het kopiëren afwerpen en de beveiligingen van het besturingssysteem omzeilen.
settings-preserve-posix-acls = POSIX-ACL's en uitgebreide attributen behouden
settings-preserve-posix-acls-hint = Neem user.* / system.* / trusted.* xattrs en POSIX-toegangsbeheerlijsten mee over de kopie.
settings-preserve-selinux = SELinux-contexten behouden
settings-preserve-selinux-hint = Neem het security.selinux-label mee over de kopie zodat daemons onder MAC-beleid het bestand kunnen blijven openen.
settings-preserve-resource-forks = macOS-resource forks en Finder-info behouden
settings-preserve-resource-forks-hint = Neem de oude resource fork en FinderInfo (kleurlabels, Carbon-metadata) mee over de kopie.
settings-appledouble-fallback = AppleDouble-sidecar gebruiken op incompatibele bestandssystemen
meta-translated-to-appledouble = Vreemde metadata opgeslagen in AppleDouble-sidecar (._{ $ext })

# Phase 25 — two-way sync with vector-clock conflict detection.
# The drawer lists configured sync pairs; each pair runs an
# independent reconciliation round against a per-pair `.copythat-sync.db`
# state store. Concurrent edits from a common ancestor surface as
# conflicts rather than silent overwrites; the losing side's content
# is preserved as `name.sync-conflict-YYYYMMDD-HHMMSS-<host>.ext`.
footer-sync = Synchroniseren
sync-drawer-title = Tweerichtingssynchronisatie
sync-drawer-hint = Houd twee mappen gesynchroniseerd zonder stille overschrijvingen. Gelijktijdige wijzigingen verschijnen als conflicten die je kunt oplossen.
sync-add-pair = Paar toevoegen
sync-add-cancel = Annuleren
sync-refresh = Vernieuwen
sync-add-save = Paar opslaan
sync-add-saving = Opslaan…
sync-add-missing-fields = Label, linkerpad en rechterpad zijn allemaal vereist.
sync-remove-confirm = Dit synchronisatiepaar verwijderen? De statusdatabase blijft behouden; de mappen blijven ongemoeid.
sync-field-label = Label
sync-field-label-placeholder = bijv. Documenten ↔ NAS
sync-field-left = Linkermap
sync-field-left-placeholder = Kies of plak een absoluut pad
sync-field-right = Rechtermap
sync-field-right-placeholder = Kies of plak een absoluut pad
sync-field-mode = Modus
sync-mode-two-way = Tweerichtings
sync-mode-mirror-left-to-right = Spiegelen (links → rechts)
sync-mode-mirror-right-to-left = Spiegelen (rechts → links)
sync-mode-contribute-left-to-right = Bijdragen (links → rechts, geen verwijderingen)
sync-no-pairs = Nog geen synchronisatieparen geconfigureerd. Klik op "Paar toevoegen" om te beginnen.
sync-loading = Geconfigureerde paren laden…
sync-never-run = Nooit uitgevoerd
sync-running = Actief
sync-run-now = Nu uitvoeren
sync-cancel = Annuleren
sync-remove-pair = Verwijderen
sync-view-conflicts = Conflicten bekijken ({ $count })
sync-conflicts-heading = Conflicten
sync-no-conflicts = Geen conflicten bij de laatste uitvoering.
sync-winner = Winnaar
sync-side-left-to-right = links
sync-side-right-to-left = rechts
sync-conflict-kind-concurrent-write = Gelijktijdige wijziging
sync-conflict-kind-delete-edit = Verwijderen ↔ wijzigen
sync-conflict-kind-add-add = Beide kanten toegevoegd
sync-conflict-kind-corrupt-equal = Inhoud is uiteengelopen zonder nieuwe schrijfactie
sync-resolve-keep-left = Links behouden
sync-resolve-keep-right = Rechts behouden
sync-resolve-keep-both = Beide behouden
sync-resolve-three-way = Oplossen via 3-weg-samenvoeging
sync-resolve-phase-53-tooltip = Interactieve 3-weg-samenvoeging voor niet-tekstbestanden komt in fase 53.
sync-error-prefix = Synchronisatiefout

# Phase 26 — real-time mirror watcher. "Live mirror" starts a
# filesystem watcher on the pair's left side; every debounced event
# triggers a re-sync. The watcher filters vim swap files / Office
# lock files / atomic-save temp names so one logical save becomes
# exactly one sync round.
live-mirror-start = Live-spiegeling starten
live-mirror-stop = Live-spiegeling stoppen
live-mirror-watching = Bewaken
live-mirror-toggle-hint = Automatisch opnieuw synchroniseren bij elke gedetecteerde wijziging in het bestandssysteem. Eén achtergrondthread per actief paar.
watch-event-prefix = Bestandswijziging
watch-overflow-recovered = Buffer van bewaker liep over; opnieuw opstellen om te herstellen

# Phase 27 — content-defined chunk store. Enables delta-resume (a
# retry only re-writes chunks that actually changed) and same-job
# dedup (files sharing content blocks store those blocks once). The
# store is disk-backed under `<data-dir>/chunks/` by default and is
# the foundation for the Phase 49–51 moonshot repository phases.
chunk-store-section = Chunkopslag
chunk-store-enable = Chunkopslag inschakelen (delta-hervatten en dedup)
chunk-store-enable-hint = Splitst elk gekopieerd bestand op inhoud (FastCDC) en slaat chunks inhoudsgeadresseerd op. Nieuwe pogingen herschrijven alleen gewijzigde chunks; bestanden met gedeelde inhoud worden automatisch gededupliceerd.
chunk-store-location = Locatie van chunkopslag
chunk-store-max-size = Maximale grootte van chunkopslag
chunk-store-prune = Chunks ouder dan (dagen) snoeien
chunk-store-savings = { $gib } GiB bespaard via chunk-dedup
chunk-store-disk-usage = { $size } in gebruik over { $chunks } chunks

# Phase 28 — tray-resident Drop Stack. The stack is a persistent list
# of paths gathered from multiple sources (Explorer drag, main-window
# context menu, CLI, drag onto the Drop Stack window) that the user
# can dispatch to a destination in one go.
dropstack-window-title = Drop Stack
dropstack-tray-open = Drop Stack
dropstack-empty-title = Drop Stack is leeg
dropstack-empty-hint = Sleep hier bestanden naartoe vanuit Explorer of klik met de rechtermuisknop op een taakregel om die toe te voegen.
dropstack-add-to-stack = Toevoegen aan Drop Stack
dropstack-copy-all-to = Alles kopiëren naar…
dropstack-move-all-to = Alles verplaatsen naar…
dropstack-clear = Stapel wissen
dropstack-remove-row = Uit stapel verwijderen
dropstack-path-missing-toast = { $path } gesleept — het bestand bestaat niet meer.
dropstack-always-on-top = Drop Stack altijd op de voorgrond houden
dropstack-show-tray-icon = Het Freally File Manager-systeemvakpictogram tonen
dropstack-open-on-start = Drop Stack automatisch openen bij starten van app
dropstack-count = { $count } pad

# Phase 29 — spring-loaded folders + native DnD polish. The Settings
# → General tab carries the knobs; DropTarget / DestinationPicker
# Svelte components use the dropzone-* keys at runtime.
settings-dnd-heading = Slepen en neerzetten
settings-dnd-spring-load = Mappen tijdens slepen automatisch openen
settings-dnd-spring-delay = Vertraging voor automatisch openen (ms)
settings-dnd-thumbnails = Sleepminiaturen tonen
settings-dnd-invalid-highlight = Ongeldige sleepdoelen markeren
dropzone-invalid-title = Geen geldig sleepdoel
dropzone-invalid-readonly = Bestemming is alleen-lezen
dropzone-picker-title = Kies een bestemming
dropzone-picker-up = Omhoog
dropzone-picker-path = Huidig pad
dropzone-picker-root = Hoofdmappen
dropzone-picker-use-this = Deze map gebruiken
dropzone-picker-empty = Geen submappen
dropzone-picker-cancel = Annuleren

# Phase 30 — cross-platform path translation. Settings → Transfer
# exposes these under a "Cross-platform compatibility" subsection.
translate-heading = Platformoverschrijdende compatibiliteit
translate-unicode-label = Unicode-normalisatie
translate-unicode-auto = Bestemming automatisch detecteren
translate-unicode-windows = NFC (Windows / Linux)
translate-unicode-macos = Ongewijzigd laten (macOS / APFS)
translate-line-endings-label = Regeleinden vertalen voor tekstbestanden
translate-line-endings-allowlist = Tekstbestandsextensies
reserved-name-label = Afhandeling van gereserveerde namen in Windows
reserved-name-suffix = "_" toevoegen (CON.txt → CON_.txt)
reserved-name-reject = Weigeren en waarschuwen
long-path-label = Windows long-path-voorvoegsel (\\?\) gebruiken bij meer dan 260 tekens
long-path-hint = Sommige netwerkshares en oude tools ondersteunen de \\?\-naamruimte niet.

# Phase 31 — power-aware copying. Settings → Power & State tab + the
# header badge that renders "⏸ Paused — Zoom call detected" when the
# runner's power subscriber has paused or capped due to a policy match.
power-heading = Energie en status
power-enabled = Energiebewuste regels inschakelen
power-battery-label = Op batterij
power-metered-label = Op gemeten wifi
power-cellular-label = Op mobiel netwerk
power-presentation-label = Tijdens presenteren (Zoom / Teams / Keynote)
power-fullscreen-label = Wanneer een app op volledig scherm staat
power-thermal-label = Wanneer de CPU thermisch wordt beperkt
power-rule-continue = Doorgaan op volle snelheid
power-rule-pause = Alle taken pauzeren
power-rule-cap = Bandbreedte beperken
power-rule-cap-percent = Beperken tot een percentage van de huidige snelheid
power-reason-on-battery = op batterij
power-reason-metered-network = gemeten netwerk
power-reason-cellular-network = mobiel netwerk
power-reason-presenting = presentatiemodus
power-reason-fullscreen = app op volledig scherm
power-reason-thermal-throttling = CPU wordt beperkt

# Phase 32 — cloud backend matrix via OpenDAL. Settings → Remotes
# tab + the Add-backend wizard that writes one entry per remote into
# the keychain-backed credential store.
remote-heading = Externe backends
remote-add = Backend toevoegen
remote-list-empty = Geen externe backends geconfigureerd
remote-test = Verbinding testen
remote-test-success = Verbinding geslaagd
remote-test-failed = Verbinding mislukt
remote-remove = Backend verwijderen
remote-name-label = Weergavenaam
remote-kind-label = Backendtype
remote-save = Backend opslaan
remote-cancel = Annuleren
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
backend-local-fs = Lokaal bestandssysteem
cloud-config-bucket = Bucket
cloud-config-region = Regio
cloud-config-endpoint = Endpoint-URL
cloud-config-root = Hoofdpad
cloud-error-invalid-config = Backendconfiguratie is ongeldig
cloud-error-network = Netwerkfout bij contact met backend
cloud-error-not-found = Object niet gevonden op het opgevraagde pad
cloud-error-permission = Toegang geweigerd door externe backend
cloud-error-keychain = Toegang tot sleutelhanger van besturingssysteem mislukt
settings-tab-remotes = Externe opslag
settings-tab-mobile = Mobiel

# Phase 33 — mount Freally File Manager's chunk store + history archive as a
# read-only filesystem (FUSE / WinFsp). Surfaced by the History tab
# context menu's "Mount snapshot" action + the Settings → Advanced
# "Mount latest on launch" toggle.
mount-heading = Snapshot koppelen
mount-action-mount = Snapshot koppelen
mount-action-unmount = Ontkoppelen
mount-status-mounted = Gekoppeld op { $path }
mount-error-unsafe-mountpoint = Pad van koppelpunt is onveilig
mount-error-mountpoint-not-empty = Koppelpunt moet een lege map zijn
mount-error-backend-unavailable = Koppelbackend is niet beschikbaar op dit systeem
mount-error-archive-read = Lezen van archief mislukt
mount-picker-title = Kies map voor koppelpunt
mount-toast-mounted = Snapshot gekoppeld op { $path }
mount-toast-unmounted = Snapshot ontkoppeld
mount-toast-failed = Koppelen mislukt: { $reason }
settings-mount-heading = Snapshots koppelen
settings-mount-hint = Stel het geschiedenisarchief beschikbaar als een alleen-lezen bestandssysteem. Fase 33b verbindt de runnerstroom; de kernel-FUSE/WinFsp-backends komen in fase 33c.
settings-mount-on-launch = De nieuwste snapshot koppelen bij starten
settings-mount-on-launch-path = Pad van koppelpunt
settings-mount-on-launch-path-placeholder = bijv. C:\Mounts\copythat

# Phase 34 — enterprise-grade audit log export + WORM (write-once-read-
# many) tamper-resistance. Settings → Advanced → Audit log collects the
# format + destination + rotation + WORM toggle; runtime records job and
# file events into the tamper-evident chain-hashed sink.
settings-audit-heading = Auditlogboek
settings-audit-hint = Alleen-toevoegen, manipulatiebestendig logboek van elke taak- en bestandsgebeurtenis. Formaten zijn onder meer CSV, JSON-lines, RFC 5424 Syslog, ArcSight CEF en QRadar LEEF.
settings-audit-enable = Auditlogboek inschakelen
settings-audit-format = Logformaat
settings-audit-format-json-lines = JSON lines (aanbevolen standaard)
settings-audit-format-csv = CSV (geschikt voor spreadsheets)
settings-audit-format-syslog = Syslog (RFC 5424)
settings-audit-format-cef = CEF (ArcSight)
settings-audit-format-leef = LEEF 2.0 (IBM QRadar)
settings-audit-file-path = Pad van logbestand
settings-audit-file-path-placeholder = bijv. C:\ProgramData\CopyThat\audit.log
settings-audit-max-size = Roteren na (bytes, 0 = nooit)
settings-audit-worm = WORM-modus inschakelen (write-once-read-many)
settings-audit-worm-hint = Past de alleen-toevoegen-vlag van het platform toe (Linux chattr +a, macOS chflags uappnd, alleen-lezen-attribuut van Windows) na elke aanmaak of rotatie. Zelfs een beheerder moet de vlag expliciet wissen om het logboek in te korten.
settings-audit-test-write = Testschrijfactie
settings-audit-verify-chain = Keten verifiëren
toast-audit-test-write-ok = Testschrijfactie van auditlogboek geslaagd
toast-audit-verify-ok = Auditketen intact geverifieerd
toast-audit-verify-failed = Verificatie van auditketen meldde verschillen

# Phase 35 — destination encryption via age (X25519 / passphrase /
# SSH) + on-the-fly zstd compression with a per-extension deny
# heuristic. Surfaced through Settings → Transfer → Encryption +
# Compression; engine short-circuits to the crypt pipeline when
# either stage is active.
settings-crypt-heading = Versleuteling en compressie
settings-crypt-hint = Transformeer bestandsinhoud voordat deze op de bestemming belandt. Versleuteling gebruikt het age-formaat; compressie gebruikt zstd en kan reeds gecomprimeerde media op extensie overslaan.
settings-crypt-encryption-mode = Versleuteling
settings-crypt-encryption-off = Uit
settings-crypt-encryption-passphrase = Wachtwoordzin (vragen bij start van kopie)
settings-crypt-encryption-recipients = Ontvangerssleutels uit bestand
settings-crypt-encryption-hint = Wachtwoordzinnen blijven alleen in het geheugen voor de duur van de kopie. Ontvangersbestanden bevatten één age1… of ssh- publieke sleutel per regel.
settings-crypt-recipients-file = Pad van ontvangersbestand
settings-crypt-recipients-file-placeholder = bijv. C:\Users\me\recipients.txt
settings-crypt-compression-mode = Compressie
settings-crypt-compression-off = Uit
settings-crypt-compression-always = Altijd
settings-crypt-compression-smart = Slim (reeds gecomprimeerde media overslaan)
settings-crypt-compression-hint = Slimme modus slaat jpg, mp4, zip, 7z en soortgelijke formaten over die geen baat hebben bij zstd. De modus Altijd comprimeert elk bestand op het gekozen niveau.
settings-crypt-compression-level = zstd-niveau (1-22)
settings-crypt-compression-level-hint = Lagere getallen zijn sneller; hogere getallen comprimeren sterker. Niveau 3 komt overeen met de CLI-standaard van zstd.
compress-footer-savings = 💾 { $original } → { $compressed } ({ $percent }% bespaard)
compress-savings-toast = { $percent }% gecomprimeerd ({ $bytes } bespaard)
crypt-toast-recipients-loaded = { $count } versleutelingsontvangers geladen
crypt-toast-recipients-error = Laden van ontvangers mislukt: { $reason }
crypt-toast-passphrase-required = Versleuteling heeft een wachtwoordzin nodig voordat de kopie start
crypt-toast-passphrase-set = Wachtwoordzin voor versleuteling vastgelegd
crypt-footer-encrypted-badge = 🔒 Versleuteld (age)
crypt-footer-compressed-badge = 📦 Gecomprimeerd (zstd)

# Phase 36 — copythat CLI surface. Documented exit codes + the
# user-facing strings for plan/apply/verify/config. Help text stays
# in English (engineering accessibility) but error / info / status
# strings are localized so a localized desktop installation surfaces
# CLI errors in the same language as the GUI.
cli-help-tagline = Freally File Manager CLI — byte-exacte bestandskopie, synchronisatie, verificatie en audit voor CI/CD-pijplijnen.
cli-help-exit-codes = Exitcodes: 0 succes, 1 fout, 2 in behandeling, 3 conflict, 4 verificatie mislukt, 5 net, 6 rechten, 7 schijf vol, 8 annuleren, 9 config.
cli-error-bad-args = copy/move vereist ten minste één bron en een bestemming
cli-error-unknown-algo = Onbekend verificatiealgoritme: { $algo }
cli-error-missing-spec = --spec is vereist voor plan/apply
cli-error-spec-parse = Parsen van jobspec { $path } mislukt: { $reason }
cli-error-spec-empty-sources = Bronlijst van jobspec is leeg
cli-info-shape-recorded = Bandbreedte-shape "{ $rate }" vastgelegd; handhaving loopt via copythat-shape
cli-info-stub-deferred = { $command } staat klaar voor de vervolgbedrading van fase 36
cli-plan-summary = Plan: { $actions } actie(s), { $bytes } byte(s); { $already_done } al aanwezig
cli-plan-pending = Plan meldt openstaande acties; voer opnieuw uit met `apply` om uit te voeren
cli-plan-already-done = Plan meldt dat er niets te doen is (idempotent)
cli-apply-success = Toepassen voltooid zonder fouten
cli-apply-failed = Toepassen voltooid met een of meer fouten
cli-verify-ok = Verificatie ok: { $algo } { $digest }
cli-verify-failed = Verificatie MISLUKT voor { $path } ({ $algo })
cli-config-set = { $key } = { $value } ingesteld
cli-config-reset = { $key } teruggezet naar standaard
cli-config-unknown-key = Onbekende configuratiesleutel: { $key }
cli-completions-emitted = Shell-completions voor { $shell } afgedrukt naar stdout

# Phase 37 — desktop-side mobile companion. Settings → Mobile panel
# strings + the SAS-confirmation modal + push-notification toasts.
# The actual mobile UI lives in the Phase 37 follow-up Tauri Mobile
# target; the phone displays its own localized SAS prompt.
settings-mobile-heading = Mobiele metgezel
settings-mobile-hint = Koppel een iPhone of Android-telefoon om de geschiedenis te doorbladeren, opgeslagen profielen en jobspecs uit fase 36 te starten en meldingen bij voltooiing te ontvangen.
settings-mobile-pair-toggle = Nieuwe koppelingen toestaan
settings-mobile-pair-active = Koppelserver actief — scan de QR met de Freally File Manager mobiele app
settings-mobile-pair-button = Koppelen starten
settings-mobile-revoke-button = Intrekken
settings-mobile-no-pairings = Nog geen gekoppelde apparaten
settings-mobile-pair-port = Bindpoort (0 = kies een vrije)
pair-sas-prompt = Beide schermen moeten dezelfde vier emoji's tonen. Tik op Komt overeen als ze gelijk zijn.
pair-sas-confirm = Komt overeen
pair-sas-reject = Komt niet overeen — annuleren
pair-toast-success = Gekoppeld met { $device }
pair-toast-failed = Koppelen mislukt: { $reason }
push-toast-sent = Pushmelding verzonden naar { $device }
push-toast-failed = Pushmelding naar { $device } mislukt: { $reason }

# Phase 38 — aggregate destination dedup + reflink fallback ladder.
# Settings → Transfer → Dedup panel + per-job-row badges (⚡
# Reflinked / 🔗 Hardlinked / 🧩 Chunk-shared / 📋 Copied) +
# pre-pass dedup-scan modal.
settings-dedup-heading = Bestemmingsdedup
settings-dedup-hint = Wanneer de bron en bestemming een volume delen, kan Freally File Manager bestanden op bestandssysteemniveau klonen in plaats van bytes te kopiëren. Reflink is direct + veilig; hardlink is sneller maar beide namen delen dan hun status.
settings-dedup-mode-auto = Automatische ladder (reflink → hardlink → chunk → kopie)
settings-dedup-mode-reflink-only = Alleen reflink
settings-dedup-mode-hardlink-aggressive = Agressief (reflink + hardlink, zelfs bij beschrijfbare bestanden)
settings-dedup-mode-off = Uitgeschakeld (altijd byte-kopie)
settings-dedup-hardlink-policy = Hardlink-beleid
settings-dedup-prescan = Bestemmingsstructuur vooraf scannen op dubbele inhoud
dedup-badge-reflinked = ⚡ Reflinked
dedup-badge-hardlinked = 🔗 Hardlinked
dedup-badge-chunk-shared = 🧩 Chunk-gedeeld
dedup-badge-copied = 📋 Gekopieerd
phase42-paranoid-verify-label = Paranoïde verificatie
phase42-paranoid-verify-hint = Wist de gecachete pagina's van de bestemming en leest opnieuw van schijf om leugens van de schrijfcache en stille corruptie te betrappen. Ongeveer 50% trager dan de standaardverificatie; standaard uit.
phase42-sharing-violation-retries-label = Aantal nieuwe pogingen bij vergrendelde bronbestanden
phase42-sharing-violation-retries-hint = Hoe vaak opnieuw proberen wanneer een ander proces het bronbestand met een exclusieve vergrendeling open houdt. De wachttijd verdubbelt bij elke poging (standaard 50 ms / 100 ms / 200 ms). Standaard 3, overeenkomend met Robocopy /R:3.
phase42-cloud-placeholder-warning = { $name } is een OneDrive-bestand dat alleen in de cloud staat. Het kopiëren start een download — tot { $size } over je netwerkverbinding.
phase42-defender-exclusion-hint = Voeg voor maximale kopieersnelheid de bestemmingsmap toe aan de Microsoft Defender-uitsluitingen vóór bulkoverdrachten. Zie docs/PERFORMANCE_TUNING.md.

# Phase 39 — Browser-accessible recovery UI. Settings → Advanced
# exposes these strings; the recovery server itself renders askama
# templates that always read English — these eight keys are the
# Settings prose the user actually reads in their preferred locale.
settings-recovery-heading = Herstel-web-UI
settings-recovery-enable = Herstel-web-UI inschakelen
settings-recovery-bind-address = Bindadres
settings-recovery-port = Poort (0 = kies een vrije)
settings-recovery-show-url = URL & token tonen
settings-recovery-rotate-token = Token roteren
settings-recovery-allow-non-loopback = Niet-loopback-binding toestaan
settings-recovery-non-loopback-warning = WAARSCHUWING: het inschakelen van een niet-loopback-binding stelt de herstel-UI bloot aan je lokale netwerk. Iedereen die het token achterhaalt, kan je bestandsgeschiedenis doorbladeren en bestanden downloaden. Zet er TLS of een reverse proxy voor als het LAN niet vertrouwd is.

# Phase 40 — SMB compression negotiation + cloud-VM offload helper.
# 6 SMB keys (header badge + Settings prose) + 6 cloud-offload keys
# (Remotes tab wizard for cross-cloud copy templates).
smb-compress-badge = 🗜 SMB-compressie: { $algo }
smb-compress-badge-tooltip = Netwerkverkeer naar deze bestemming wordt onderweg gecomprimeerd (SMB 3.1.1).
smb-compress-toast-saved = { $bytes } bespaard over het netwerk
smb-compress-algo-unknown = onbekend algoritme
settings-smb-compress-heading = SMB-netwerkcompressie
settings-smb-compress-hint = Onderhandel automatisch over SMB 3.1.1-verkeercompressie op UNC-bestemmingen. Gratis winst op trage verbindingen; genegeerd bij lokale bestemmingen.
cloud-offload-heading = Cloud-vm-offloadhelper
cloud-offload-hint = Bij rechtstreeks kopiëren tussen twee clouds genereer je een implementatiesjabloon die de kopie uitvoert vanaf een kleine tijdelijke vm in de cloud — bytes raken nooit het netwerk van je laptop.
cloud-offload-render-button = Sjabloon genereren
cloud-offload-copy-clipboard = Kopiëren naar klembord
cloud-offload-template-format = Sjabloonformaat
cloud-offload-self-destruct-warning = De vm wordt automatisch uitgeschakeld na { $minutes } minuten — bevestig de IAM-rol + regio voordat je implementeert.

# Phase 41 — animated before/after tree-diff preview. The `Preview
# changes` modal renders the rolled-up plan before the engine starts
# work; 14 keys cover the title, the summary header / counts, the
# row-reason labels, and the two action buttons.
preview-modal-title = Wijzigingen bekijken
preview-summary-header = Wat er gaat gebeuren
preview-category-additions = { $count } toevoegingen
preview-category-replacements = { $count } vervangingen
preview-category-skips = { $count } overgeslagen
preview-category-conflicts = { $count } conflicten
preview-category-unchanged = { $count } ongewijzigd
preview-bytes-to-transfer = { $bytes } over te dragen
preview-reason-source-newer = Bron is nieuwer
preview-reason-dest-newer = Bestemming is nieuwer — wordt overgeslagen
preview-reason-content-different = Inhoud verschilt
preview-reason-identical = Identiek aan bron
preview-button-run = Plan uitvoeren
preview-button-reduce = Mijn plan verkleinen…

# Phase 42 — perceptual-hash visual-similarity dedup. Eight keys cover
# the pre-copy "looks visually identical" warning + the Settings panel.
perceptual-warn-title = Ziet er visueel identiek uit
perceptual-warn-body = { $name } op de bestemming lijkt overeen te komen met de bronafbeelding. Toch doorgaan met kopiëren?
perceptual-warn-keep-both = Beide behouden
perceptual-warn-skip = Dit bestand overslaan
perceptual-warn-overwrite = Toch overschrijven
perceptual-settings-heading = Visuele-gelijkenisdedup
perceptual-settings-hint = Detecteer visueel identieke afbeeldingen op de bestemming voordat ze worden overschreven. De hash is perceptueel (herkent dezelfde afbeelding opnieuw opgeslagen in een ander formaat), niet byte-exact.
perceptual-settings-threshold-label = Waarschuwingsdrempel (lager = strenger overeenkomen)

# Phase 42 Part B — per-file rolling versions (Time Machine for any
# destination). 8 keys cover the version-list panel + retention picker.
version-list-heading = Vorige versies
version-list-empty = Geen eerdere versies van dit bestand
version-list-restore = Deze versie herstellen
version-retention-heading = Vorige versies behouden bij overschrijven
version-retention-none = Elke versie voor altijd behouden
version-retention-last-n = Laatste { $n } versies behouden
version-retention-older-than-days = Versies ouder dan { $days } dagen verwijderen
version-retention-gfs = Per uur { $h } · per dag { $d } · per week { $w } · per maand { $m }

# Phase 43 — forensic chain-of-custody manifests + BLAKE3 verified
# streaming. 14 keys cover the Settings → Provenance panel
# (heading, hint, toggles, TSA URL, signing-key management) plus
# the post-job manifest notification + the verify command's two
# headline result lines.
provenance-settings-heading = Forensische chain-of-custody
provenance-settings-hint = Onderteken elke kopieertaak met een BLAKE3 + ed25519-manifest. Reviewers kunnen de bestemmingsstructuur later opnieuw hashen en bewijzen dat er sinds de kopie geen byte is gewijzigd.
provenance-settings-enable-default = Elke nieuwe taak standaard ondertekenen
provenance-settings-show-after-job = Manifest tonen na elke voltooide taak
provenance-settings-tsa-url-label = Standaard-URL van RFC 3161-tijdstempelautoriteit
provenance-settings-tsa-url-hint = Optioneel. Indien ingesteld, dragen manifesten een gratis TSA-tijdstempel die bewijst dat de bytes op dit tijdstip bestonden. Laat leeg om over te slaan.
provenance-settings-keys-heading = Ondertekeningssleutels
provenance-settings-keys-generate = Nieuwe sleutel genereren
provenance-settings-keys-import = Sleutel importeren…
provenance-settings-keys-export = Publieke sleutel exporteren…
provenance-job-completed-title = Provenance-manifest opgeslagen
provenance-job-completed-body = { $count } bestanden ondertekend → { $path }
provenance-verify-clean = Manifest geldig voor { $count } bestanden; handtekening { $sig }; merkle-root OK.
provenance-verify-tampered = Manifest ONGELDIG — { $tampered } gemanipuleerd, { $missing } ontbreekt.
# Phase 43 post-review hardening — toast text for the Settings →
# Provenance buttons whose Tauri IPC has not yet landed.
provenance-action-staged = Fase 43 — de IPC-bedrading voor deze actie komt in een vervolgcommit.

# Phase 44 — SSD-aware whole-drive sanitize (NVMe Sanitize / OPAL
# Crypto Erase) + the per-file shred refusal on copy-on-write
# filesystems. 16 keys cover the new "Drive sanitize" Settings
# subsection and the localized error messages.
sanitize-heading = Volledige schijf veilig wissen
sanitize-hint = NVMe Sanitize, OPAL Crypto Erase en ATA Secure Erase wissen een flashschijf op firmwareniveau in milliseconden. Overschrijven per bestand is zinloos op flash — meerdere shred-doorgangen verslijten alleen de NAND. Gebruik dit voor een echte purge.
sanitize-pick-device = Kies de schijf om te wissen
sanitize-mode-label = Wismethode
sanitize-mode-nvme-format = NVMe Format (met secure erase)
sanitize-mode-nvme-sanitize-block = NVMe Sanitize — Block Erase (traag, elke cel)
sanitize-mode-nvme-sanitize-crypto = NVMe Sanitize — Crypto Erase (direct)
sanitize-mode-ata-secure-erase = ATA Secure Erase (oudere SATA-SSD's)
sanitize-mode-opal-crypto-erase = TCG OPAL Crypto Erase (zelfversleutelende schijven)
sanitize-mode-apfs-crypto-erase = APFS Crypto Erase (FileVault-sleutel roteren, alleen macOS)
sanitize-confirm-1 = Dit vernietigt ELKE byte op { $device }. Dit kan niet ongedaan worden gemaakt.
sanitize-confirm-2 = Ik begrijp dat alle partities, alle bestanden en alle snapshots op { $device } permanent onleesbaar worden.
sanitize-confirm-3 = Typ de modelnaam van de schijf om door te gaan: { $model }
sanitize-running = { $device } wordt gewist ({ $mode }) — dit kan van milliseconden (crypto erase) tot tientallen minuten (block erase) duren. Schakel niet uit.
sanitize-completed = Wissen voltooid — { $device } is nu leeg.
ssd-honest-shred-meaningless = Shred per bestand op een copy-on-write-bestandssysteem (Btrfs / ZFS / APFS) kan de onderliggende blokken niet bereiken. Gebruik in plaats daarvan het veilig wissen van de hele schijf plus rotatie van de volledige-schijfversleutelingssleutel.
ssd-honest-advisory = Dit bestand staat op flash. Overschrijven per bestand kost NAND-slijtage en garandeert NIET dat de oorspronkelijke cellen onherstelbaar zijn. Wis voor gevoelige gegevens de hele schijf.

# Phase 44.1f post-review — placeholder toast for SanitizeTab
# buttons whose Tauri IPC has not yet landed.
sanitize-action-staged = Fase 44.1 — de IPC-bedrading voor deze actie komt in een vervolgcommit.

# Phase 45.3 — named-queue tab strip (Subfeature A). Tabs surface
# once the QueueRegistry holds at least one queue; the synthesised
# default tab keeps legacy single-queue jobs reachable.
queue-tab-default = Standaard
queue-tab-empty-state = Takenwachtrijen
queue-badge-tooltip = Openstaande en actieve taken in deze wachtrij

# Phase 45.4 — drag-progress-merge (Subfeature B). Drag a queue tab
# onto another to merge their job lists. The default tab is neither
# draggable nor a drop target; only registry queues participate.
queue-drag-hint = Sleep op een andere wachtrij om samen te voegen
queue-merge-confirm = Neerzetten om samen te voegen
queue-merge-toast = Wachtrijen samengevoegd

# Phase 45.5 — F2-queue UX (Subfeature C). F2 toggles
# `auto_enqueue_next` so every fresh enqueue piles into the running
# queue rather than spawning a parallel one. Status pill renders in
# the Footer; pulsing dot renders on the running tab.
queue-f2-active-hint = F2-modus: elke nieuwe taak belandt in deze wachtrij
queue-f2-toggled-on = F2-wachtrijmodus AAN — nieuwe taken sluiten aan bij de actieve wachtrij
queue-f2-toggled-off = F2-wachtrijmodus UIT — nieuwe taken starten parallelle wachtrijen
queue-f2-status-bar = F2-wachtrijmodus: AAN

# Phase 45.6 — tray destination targets (Subfeature D). Pinned
# destinations appear in the OS tray menu; clicking one arms it as
# the active drop target so the next file drop bypasses the
# DropStagingDialog. Settings → General hosts the list editor.
tray-target-section-title = Systeemvakbestemmingen
tray-target-section-hint = Vastgezette bestemmingen verschijnen in het systeemvakmenu. Klik er een aan om die als volgende sleepdoel scherp te zetten.
tray-target-empty = Nog geen systeemvakbestemmingen vastgezet.
tray-target-remove = Verwijderen
tray-target-add-label = Label
tray-target-add-path = Pad of backend-URI
tray-target-add = Toevoegen
tray-target-armed-toast = Zet je volgende bestand neer om het naar { $label } te sturen
tray-target-active-pill = → { $label }

# Phase 45.7 follow-up — pinned-destination validation errors. The
# `queue_pin_destination` IPC returns these Fluent keys verbatim
# when input fails the IPC-boundary checks; the toast layer renders
# them via `t(...)` (Toast.svelte detects kebab-case-lowercase and
# routes through the locale table). Phase 17e's `err-path-escape` /
# `err-destination-empty` cover the path-traversal + empty cases for
# `queue_route_job`; the keys below are pin-specific.
err-pinned-destination-label-empty = Label van systeemvakbestemming mag niet leeg zijn.
err-pinned-destination-path-empty = Pad van systeemvakbestemming mag niet leeg zijn.
err-pinned-destination-label-too-long = Label van systeemvakbestemming is te lang (max. 64 tekens).
err-pinned-destination-path-too-long = Pad van systeemvakbestemming is te lang (max. 1024 tekens).
err-pinned-destination-label-invalid = Label van systeemvakbestemming bevat tekens die niet zijn toegestaan (regeleinde, return of NUL).
err-pinned-destination-path-invalid = Pad van systeemvakbestemming bevat tekens die niet zijn toegestaan (regeleinde, return of NUL).
err-pinned-destination-too-many = Je hebt de limiet van 50 systeemvakbestemmingen bereikt. Verwijder er een om een andere toe te voegen.

# Phase 46.6 — Settings → Plugins tab. The IPC layer in
# `apps/copythat-ui/src-tauri/src/plugin_commands.rs` enumerates the
# per-user plugin store under `<config_dir>/plugins/`; the
# `PluginsTab.svelte` component renders these strings against the
# returned manifest + grant state.
settings-tab-plugins = Plug-ins
plugin-heading = Plug-ins
plugin-hint = Sandboxed WASM-plug-ins breiden Freally File Manager uit met aangepaste hooks. Elke plug-in draait onder CPU- en geheugenlimieten per aanroep en ziet alleen de hostcapabilities die je toekent.
plugin-list-empty = Nog geen plug-ins geïnstalleerd.
plugin-enabled = Ingeschakeld
plugin-disabled = Uitgeschakeld
plugin-hooks = Hooks
plugin-capabilities = Capabilities
plugin-no-capabilities = (geen)
plugin-directory = Locatie
plugin-install-from-file = Installeren vanuit bestand…
plugin-install-from-url = Installeren vanuit URL…
plugin-url-wasm = WASM-URL
plugin-url-manifest = Manifest-URL
plugin-url-hash = BLAKE3-hash
plugin-url-preview = Voorbeeld
plugin-url-confirm = Installatie bevestigen

# Phase 31b — power-policy settings (Power tab).
settings-tab-power = Energie
settings-power-hint = Beperk of pauzeer kopieën op basis van energiestatus: accu, gedoseerd/mobiel netwerk, presentatie/volledig scherm of thermische CPU-throttling.
settings-power-enabled = Energiebewuste beperking inschakelen
settings-power-battery = Op accu
settings-power-metered = Op een gedoseerd netwerk
settings-power-cellular = Op mobiel netwerk
settings-power-presentation = Tijdens presenteren
settings-power-fullscreen = In volledig scherm
settings-power-thermal = Bij thermische throttling
settings-power-continue = Doorgaan
settings-power-pause = Pauzeren
err-server-not-implemented = De servermodus is nog niet beschikbaar.
err-webhook-not-implemented = Webhook-bezorging is nog niet beschikbaar.

# Phase 47 — "why is this slow?" diagnostics (bottleneck badge + tooltip).
bottleneck-source-io = Bron I/O
bottleneck-dest-io = Bestemming I/O
bottleneck-network = Netwerk
bottleneck-antivirus = Antivirus
bottleneck-cpu = CPU
bottleneck-thermal = Thermisch
bottleneck-unknown = Onbekend
diag-aria = Knelpunt: { $cause }
diag-tooltip = Beperkt door { $cause } · { $rate }
diag-spark-aria = Doorvoer van de afgelopen minuut
diag-keeping-up = Houdt bij
diag-label = Diagnostiek

# Phase 48 — server mode + observability (Settings → Server).
settings-tab-server = Server
server-hint = Voer Freally File Manager uit als headless bestandsserver. Kies de protocollen om beschikbaar te stellen, stel het adres en de te delen map in en vereis optioneel authenticatie.
server-protocols = Protocollen
server-bind-addr = Bind-adres
server-root = Gedeelde map
server-readonly = Alleen-lezen (uploads en verwijderingen weigeren)
server-auth-mode = Authenticatie
server-auth-none = Geen
server-auth-bearer = Bearer-token
server-auth-basic = Basis (gebruikersnaam + wachtwoord)
server-auth-token = Token
server-auth-user = Gebruikersnaam
server-auth-password = Wachtwoord
otel-endpoint = OpenTelemetry-eindpunt
webhook-section = Webhooks
webhook-url = Webhook-URL
webhook-add = Webhook toevoegen
webhook-remove = Verwijderen
webhook-empty = Geen webhooks geconfigureerd.
webhook-pushover-token = Pushover-token
webhook-pushover-user = Pushover-gebruiker
server-start = Server starten
server-stop = Server stoppen
server-status-running = Actief op { $addr }
server-status-stopped = Gestopt
server-metrics-url = Statistieken
err-server-no-protocols = Selecteer ten minste één protocol voordat u de server start.
err-server-bind = Kan het serveradres niet binden. Mogelijk is het al in gebruik.

# Library drawer (Phase 49) — unified content-addressed repository view.
footer-library = Bibliotheek
library-title = Bibliotheek
library-loading = Repository laden…
library-unavailable = Repository niet beschikbaar
library-tab-live = Live
library-tab-snapshots = Snapshots
library-tab-versions = Versies
library-hero-savings = { $effective } effectieve gegevens geleverd · { $pct } bespaard
library-hero-empty = { $chunks } chunks opgeslagen — nog geen snapshots
library-stat-stored = Op schijf opgeslagen
library-stat-effective = Effectieve gegevens
library-stat-snapshots = Snapshots
library-stat-chunks = Unieke chunks
library-snapshot-empty = Nog geen snapshots
library-snapshot-files = { $n } bestanden
library-version-path-ph = Bestemmingspad…
library-version-load = Versies tonen
library-version-empty = Geen versies voor dit pad
repo-kind-copy = Kopie
repo-kind-sync = Synchronisatie
repo-kind-version = Versie
repo-kind-backup = Back-up
# Phase 49o — snapshot diff / compare.
library-tab-compare = Vergelijken
repo-change-added = Toegevoegd
repo-change-removed = Verwijderd
repo-change-modified = Gewijzigd
repo-change-unchanged = Geen wijzigingen
repo-diff-summary = { $added } toegevoegd · { $removed } verwijderd · { $modified } gewijzigd
repo-diff-bytes-added = { $bytes } nieuw
repo-diff-pick-two = Kies twee snapshots om te vergelijken
# Phase 49r — statistics / reports.
library-tab-reports = Rapporten
report-growth-title = Opslaggroei
report-by-kind-title = Op type
report-top-files-title = Topbestanden
report-dedup-ratio = { $pct }% gededupliceerd
report-export = Rapport exporteren
report-exported = Rapport opgeslagen in { $path }
report-file-versions = { $n } versies
# Phase 49p — pinning / prune.
repo-pin = Vastmaken
repo-unpin = Losmaken
repo-pinned-badge = Vastgemaakt
repo-prune-title = Opschonen
repo-prune-keep-last = Nieuwste behouden
repo-prune-removed = { $n } snapshots opgeschoond
repo-prune-none = Niets op te schonen

# Phase 49c — back-upbronnen.
library-tab-sources = Bronnen
backup-add-source = Bron toevoegen…
backup-source-path-ph = Map om te back-uppen…
backup-exclude-ph = Uitsluitingspatronen (komma-gescheiden)
backup-now = Nu back-uppen
backup-remove = Verwijderen
backup-empty = Nog geen back-upbronnen
backup-never-run = Nooit geback-upt
backup-last-run = Laatste back-up { $when }
backup-running = Back-uppen… { $files } bestanden
backup-toast-started = { $label } back-uppen…
backup-toast-completed = { $label } geback-upt: { $files } bestanden
backup-toast-failed = Back-up van { $label } mislukt: { $reason }
# Phase 49e — per-source retention + prune.
backup-retention = Bewaarbeleid
backup-retention-keep-all = Alles behouden
backup-retention-last = Laatste { $n } behouden
backup-retention-days = Ouder dan { $days } dagen
backup-retention-gfs = GFS-rotatie
backup-prune-now = Nu opschonen
backup-prune-none = Niets op te schonen
backup-prune-result = { $removed } momentopnames verwijderd · { $bytes } vrijgemaakt
# Phase 49f — per-source scheduling.
backup-schedule = Planning
backup-schedule-manual = Handmatig
backup-schedule-hourly = Elk uur
backup-schedule-daily = Dagelijks
backup-schedule-weekly = Wekelijks
backup-next-run = Volgende uitvoering { $when }
backup-not-scheduled = Niet gepland
# Phase 49g — source filters.
backup-include-ph = Insluitings-globs (door komma's gescheiden)
backup-skip-hidden = Verborgen overslaan
# Phase 49q — notifications.
notify-title = Meldingen
notify-on-success = Bij succes
notify-on-failure = Bij mislukking
notify-test = Test verzenden
notify-test-sent = Test verzonden naar { $n } bestemming(en)

# Phase 49d — herstelbrowser.
restore-browse = Herstellen…
restore-title = Herstellen vanuit momentopname
restore-select-all = Alles selecteren
restore-dest = Herstellen naar
restore-confirm = { $n } bestanden herstellen
restore-empty = Deze momentopname bevat geen bestanden
restore-conflict-body = { $count } geselecteerde bestanden bestaan al op de bestemming.
restore-conflict-overwrite = Overschrijven
restore-conflict-skip = Bestaande overslaan
restore-conflict-keep-both = Beide behouden
restore-toast-done = { $restored } hersteld, { $skipped } overgeslagen
restore-toast-failed = Herstellen mislukt: { $reason }
snapshot-forget = Vergeten
snapshot-forget-toast = Momentopname vergeten — voer Ruimte vrijmaken uit om deze vrij te geven
library-reclaim = Ruimte vrijmaken
# Phase 49i — full compaction.
library-compact = Volledige compactie
library-compact-started = Compactie gestart — bekijk Taken
# Phase 49h — compression.
library-stat-compression = Bespaard door compressie
storage-compression = Compressie
storage-compression-off = Uit
storage-compression-auto = Automatisch (niet-comprimeerbare overslaan)
storage-compression-always = Altijd
storage-compression-restart = Wordt bij volgende start toegepast
# Phase 49j — tasks & progress center.
footer-tasks = Taken
tasks-title = Taken
tasks-empty = Nog geen taken
tasks-running = Actief
tasks-recent = Recent
tasks-cancel = Annuleren
task-state-running = Actief
task-state-completed = Voltooid
task-state-failed = Mislukt
task-state-cancelled = Geannuleerd
# Phase 49k — repository setup/connect wizard.
repo-wizard-title = Repository verbinden
repo-wizard-create-tab = Nieuw aanmaken
repo-wizard-connect-tab = Bestaande verbinden
repo-field-name = Naam
repo-field-path = Locatie
repo-field-password = Wachtwoordzin (optioneel)
repo-action-create = Aanmaken
repo-action-connect = Verbinden
repo-action-browse = Bladeren…
repo-switcher-label = Repository
repo-action-forget = Vergeten
repo-action-change-pass = Wachtwoordzin wijzigen…
repo-password-old = Huidige wachtwoordzin
repo-password-new = Nieuwe wachtwoordzin
repo-error-exists = Op deze locatie bestaat al een repository
repo-error-not-found = Geen repository gevonden op deze locatie
repo-error-bad-pass = Onjuiste wachtwoordzin
repo-note-no-encryption = De wachtwoordzin regelt alleen de toegang; versleuteling in rust komt in een latere versie
repo-confirm-forget = "{ $name }" uit de lijst verwijderen? Je gegevens blijven op de schijf.
repo-toast-created = Repository "{ $name }" aangemaakt
repo-toast-connected = Verbonden met "{ $name }"
repo-toast-pass-changed = Wachtwoordzin bijgewerkt
# Phase 49l — Sources dashboard.
library-tab-overview = Overzicht
library-source-empty = Nog geen bronnen
library-source-unknown = (niet-opgegeven bron)
library-source-snapshots = { $n } momentopnamen
library-source-latest = Laatste { $when }
# Phase 49n — verify & repair.
repo-action-verify = Verifiëren
repo-action-verify-deep = Verifiëren (alle gegevens lezen)
repo-action-repair = Repareren…
repo-verify-clean = { $files } bestanden / { $chunks } blokken geverifieerd — geen schade
repo-verify-damaged = { $missing } ontbreekt, { $corrupt } beschadigde blokken
repo-repair-confirm = { $n } momentopnamen verwijderen die niet meer hersteld kunnen worden?
repo-repair-removed = { $n } beschadigde momentopnamen verwijderd
repo-repair-none = Niets te repareren — de repository is schoon
repo-gc-done = { $bytes } vrijgemaakt ({ $chunks } blokken)
restore-toast-partial = { $restored } hersteld, { $skipped } overgeslagen, { $failed } mislukt
