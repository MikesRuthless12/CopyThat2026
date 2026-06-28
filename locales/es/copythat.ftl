app-name = Copy That v0.19.84
window-title = Copy That v0.19.84
shred-ssd-advisory = Advertencia: este destino está en un SSD. Las sobrescrituras de varias pasadas no sanean de forma fiable la memoria flash, porque el nivelado de desgaste y el sobreaprovisionamiento mueven los datos fuera de la dirección de bloque lógica. Para medios de estado sólido, prefiere ATA SECURE ERASE, NVMe Format con borrado seguro o el cifrado de disco completo con descarte de la clave.

# Global aggregate states (header pill)
state-idle = Inactivo
state-copying = Copiando
state-verifying = Verificando
state-paused = En pausa
state-error = Error

# Per-job states (row badge)
state-pending = En cola
state-running = En curso
state-cancelled = Cancelado
state-succeeded = Listo
state-failed = Fallido

# Actions
action-pause = Pausar
action-resume = Reanudar
action-cancel = Cancelar
action-pause-all = Pausar todos los trabajos
action-resume-all = Reanudar todos los trabajos
action-cancel-all = Cancelar todos los trabajos
action-close = Cerrar
action-reveal = Mostrar en la carpeta
action-add-files = Añadir archivos
action-add-folders = Añadir carpetas

# Phase 13d — activity feed
activity-title = Actividad
activity-clear = Borrar la lista de actividad
activity-empty = Aún no hay actividad de archivos.
activity-after-done = Al terminar:
activity-keep-open = Mantener la app abierta
activity-close-app = Cerrar la app
activity-shutdown = Apagar el PC
activity-logoff = Cerrar sesión
activity-sleep = Suspender

# Phase 14 — preflight free-space dialog
preflight-block-title = No hay espacio suficiente en el destino
preflight-warn-title = Poco espacio en el destino
preflight-unknown-title = No se pudo determinar el espacio libre
preflight-unknown-body = El origen es demasiado grande para medirlo con rapidez o el volumen de destino no respondió. Puedes continuar; el control de espacio del motor detendrá la copia de forma limpia si se queda sin sitio.
preflight-required = Necesario
preflight-free = Libre
preflight-reserve = Reserva
preflight-shortfall = Déficit
preflight-continue = Continuar de todos modos
preflight-pick-subset = Elegir qué copiar…
collision-modal-overwrite-older = Sobrescribir solo los más antiguos

# Phase 14e — subset picker
subset-title = Elige qué orígenes copiar
subset-subtitle = La selección completa no cabe en el destino. Marca los elementos que quieras copiar; el resto se quedará.
subset-loading = Midiendo tamaños…
subset-too-large = demasiado grande para contar
subset-budget = Disponible
subset-remaining = Restante
subset-confirm = Copiar selección
history-rerun-hint = Vuelve a ejecutar esta copia — reanaliza cada archivo del árbol de origen
history-clear-all = Borrar todo
history-clear-all-confirm = Haz clic de nuevo para confirmar
history-clear-all-hint = Elimina todas las filas del historial. Requiere un segundo clic para confirmar.
toast-history-cleared = Historial borrado ({ $count } filas eliminadas)

# Phase 15 — source-list ordering in the drop staging dialog
drop-dialog-sort-label = Orden:
sort-custom = Personalizado
sort-name-asc = Nombre A → Z (archivos primero)
sort-name-desc = Nombre Z → A (archivos primero)
sort-size-asc = Tamaño de menor a mayor (archivos primero)
sort-size-desc = Tamaño de mayor a menor (archivos primero)
sort-reorder = Reordenar
sort-move-top = Mover al principio
sort-move-up = Subir
sort-move-down = Bajar
sort-move-bottom = Mover al final

# Phase 16 — sort preset names for the Activity list (shorter
# wording than the DropStagingDialog variants; the Activity list
# is files-only so the "(files first)" suffix isn't relevant).
sort-name-asc-simple = Nombre A → Z
sort-name-desc-simple = Nombre Z → A
sort-size-asc-simple = Tamaño de menor a mayor
sort-size-desc-simple = Tamaño de mayor a menor
activity-sort-locked = La ordenación está deshabilitada mientras se ejecuta una copia. Pausa o espera a que termine y luego cambia el orden.

# Phase 17 — collision-policy picker in the DropStagingDialog
drop-dialog-collision-label = Si un archivo ya existe:
collision-policy-keep-both = Conservar ambos (renombrar la copia nueva a _2, _3, …)
collision-policy-skip = Omitir la copia nueva
collision-policy-overwrite = Sobrescribir el archivo existente
collision-policy-overwrite-if-newer = Sobrescribir solo si es más reciente
collision-policy-prompt = Preguntar siempre

# Phase 18 — progress labels on the DropStagingDialog Start button
drop-dialog-busy-checking = Comprobando el espacio libre…
drop-dialog-busy-enumerating = Contando archivos…
drop-dialog-busy-starting = Iniciando la copia…
toast-enumeration-deferred = El árbol de origen es grande — se omite la lista previa de archivos; las filas irán apareciendo a medida que el motor las procesa.

# Context menu (per-row right-click)
menu-pause = Pausar
menu-resume = Reanudar
menu-cancel = Cancelar
menu-remove = Quitar de la cola
menu-reveal-source = Mostrar el origen en la carpeta
menu-reveal-destination = Mostrar el destino en la carpeta

# Header / toolbar
header-eta-label = Tiempo restante estimado
header-toolbar-label = Controles globales

# Footer
footer-queued = trabajos activos
footer-total-bytes = en curso
footer-errors = errores
footer-history = Historial

# Empty state
empty-title = Suelta archivos o carpetas para copiar
empty-hint = Arrastra elementos a la ventana. Te pediremos un destino y luego pondremos en cola un trabajo por cada origen.
empty-region-label = Lista de trabajos

# Details drawer
details-drawer-label = Detalles del trabajo
details-source = Origen
details-destination = Destino
details-state = Estado
details-bytes = Bytes
details-files = Archivos
details-speed = Velocidad
details-eta = Tiempo restante
details-error = Error

# Drop dialog
drop-dialog-title = Transferir los elementos soltados
drop-dialog-subtitle = { $count } elemento(s) listos para transferir. Elige una carpeta de destino para empezar.
drop-dialog-mode = Operación
drop-dialog-copy = Copiar
drop-dialog-move = Mover
drop-dialog-pick-destination = Elegir destino
drop-dialog-change-destination = Cambiar destino
drop-dialog-start-copy = Empezar a copiar
drop-dialog-start-move = Empezar a mover

# ETA placeholders
eta-calculating = calculando…
eta-unknown = desconocido

# Toast messages
toast-job-done = Transferencia completada
toast-copy-queued = Copia en cola
toast-move-queued = Movimiento en cola
toast-error-resolved = Error resuelto
toast-collision-resolved = Conflicto resuelto
toast-elevated-unavailable = El reintento con privilegios llega en la Fase 17 — aún no está disponible
toast-clipboard-files-detected = Hay archivos en el portapapeles — pulsa tu atajo de pegar para copiarlos con Copy That
toast-clipboard-no-files = El portapapeles no tiene archivos que pegar
toast-error-log-exported = Registro de errores exportado

# Error modal (Phase 8)
error-modal-title = Una transferencia falló
error-modal-retry = Reintentar
error-modal-retry-elevated = Reintentar con permisos elevados
error-modal-skip = Omitir
error-modal-skip-all-kind = Omitir todos los errores de este tipo
error-modal-abort = Anular todo
error-modal-path-label = Ruta
error-modal-code-label = Código
error-drawer-pending-count = Hay más errores en espera
error-drawer-toggle = Contraer o expandir

# Error-kind labels (Phase 8). Source of truth — engine maps each
# `CopyErrorKind` to one of these keys via `localized_key()`.
err-not-found = Archivo no encontrado
err-permission-denied = Permiso denegado
err-disk-full = El disco de destino está lleno
err-interrupted = Operación interrumpida
err-verify-failed = La verificación posterior a la copia falló
err-path-escape = Ruta rechazada — contiene segmentos de directorio superior (..) o bytes no válidos
err-path-invalid-encoding = Ruta rechazada — la cadena contiene UTF-8 no válido o caracteres de reemplazo
err-helper-invalid-json = El asistente con privilegios recibió un JSON con formato incorrecto; se ignora esta solicitud
err-helper-grant-out-of-band = GrantCapabilities debe gestionarlo el bucle de ejecución del asistente, no el manejador sin estado
err-randomness-unavailable = El generador de números aleatorios del sistema falló; no se puede crear un id de sesión
err-sparseness-mismatch = No se pudo conservar la estructura dispersa en el destino
err-io-other = Error de E/S desconocido

# Collision modal (Phase 8)
collision-modal-title = El archivo ya existe
collision-modal-overwrite = Sobrescribir
collision-modal-overwrite-if-newer = Sobrescribir si es más reciente
collision-modal-skip = Omitir
collision-modal-keep-both = Conservar ambos
collision-modal-rename = Renombrar…
collision-modal-apply-to-all = Aplicar a todos
collision-modal-source = Origen
collision-modal-destination = Destino
collision-modal-size = Tamaño
collision-modal-modified = Modificado
collision-modal-hash-check = Hash rápido (SHA-256)
collision-modal-hash-computing = Calculando…
collision-modal-hash-identical = Idénticos
collision-modal-hash-different = Diferentes
collision-modal-rename-placeholder = Nuevo nombre de archivo
collision-modal-confirm-rename = Renombrar

# Error log drawer (Phase 8)
error-log-title = Registro de errores
error-log-empty = No hay errores registrados
error-log-export-csv = Exportar CSV
error-log-export-txt = Exportar texto
error-log-clear = Borrar registro
error-log-col-time = Hora
error-log-col-job = Trabajo
error-log-col-path = Ruta
error-log-col-code = Código
error-log-col-message = Mensaje
error-log-col-resolution = Resolución

# History drawer (Phase 9)
history-title = Historial
history-empty = Aún no hay trabajos registrados
history-unavailable = El historial de copias no está disponible. La app no pudo abrir el almacén SQLite al iniciarse.
history-filter-any = cualquiera
history-filter-kind = Tipo
history-filter-status = Estado
history-filter-text = Buscar
history-refresh = Actualizar
history-export-csv = Exportar CSV
history-purge-30 = Purgar > 30 días
history-rerun = Volver a ejecutar
history-detail-open = Detalles
history-detail-title = Detalles del trabajo
history-detail-empty = No hay elementos registrados
history-col-date = Fecha
history-col-kind = Tipo
history-col-src = Origen
history-col-dst = Destino
history-col-files = Archivos
history-col-size = Tamaño
history-col-status = Estado
history-col-duration = Duración
history-col-error = Error
toast-history-exported = Historial exportado
toast-history-rerun-queued = Reejecución en cola

# Totals drawer (Phase 10)
footer-totals = Totales
totals-title = Totales
totals-loading = Cargando totales…
totals-card-bytes = Total de bytes copiados
totals-card-files = Archivos
totals-card-jobs = Trabajos
totals-card-avg-rate = Rendimiento medio
totals-errors = errores
totals-spark-title = Últimos 30 días
totals-kinds-title = Por tipo
totals-saved-title = Tiempo ahorrado (estimado)
totals-saved-note = Estimación frente a una copia con un gestor de archivos convencional para la misma carga.
totals-reset = Restablecer estadísticas
totals-reset-confirm = Esto elimina todos los trabajos y elementos almacenados. ¿Continuar?
totals-reset-confirm-yes = Sí, restablecer
toast-totals-reset = Estadísticas restablecidas

# Phase 11a — i18n core: surface remaining user-visible strings so
# every string on a main-window screen flows through Fluent.

# Header language switcher (temporary placement — folds into
# Settings → General in Phase 12).
header-language-label = Idioma
header-language-title = Cambiar idioma

# Job-kind labels. History filter, Totals breakdown, and history
# rows all map wire-format `kind` strings to these labels.
kind-copy = Copiar
kind-move = Mover
kind-delete = Eliminar
kind-secure-delete = Eliminación segura

# History status labels. Distinct from `state-*` because the
# history wire format uses plain `running`/`succeeded`/... rather
# than the live `JobState` enum the queue exposes.
status-running = En curso
status-succeeded = Completado
status-failed = Fallido
status-cancelled = Cancelado
# Per-item status (not job-level): `ok` / `skipped` are only
# reachable from the history detail view.
status-ok = OK
status-skipped = Omitido

# History drawer: search field placeholder + purge toast.
history-search-placeholder = /ruta
toast-history-purged = { $count } trabajos de más de 30 días purgados

# User-facing command-layer validation errors. The Rust side
# returns these keys when input is missing; the toast layer looks
# them up rather than showing the raw English.
err-source-required = Se requiere al menos una ruta de origen.
err-destination-empty = La ruta de destino está vacía.
err-source-empty = La ruta de origen está vacía.

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
settings-title = Configuración
settings-tab-general = General
settings-tab-appearance = Apariencia
settings-section-language = Idioma
settings-phase-12-hint = En la Fase 12 llegan más ajustes (tema, valores predeterminados de transferencia, algoritmo de verificación, perfiles).

# Phase 12 — full Settings window keys. Grouped by tab so the
# human-review pass can pick one section at a time. Every field
# label and every option label that the user will actually read
# has its own key; dynamic wire values (enum discriminants like
# "auto" / "prefer") stay untranslated on the wire.

settings-loading = Cargando configuración…
settings-tab-transfer = Transferencia
settings-tab-filters = Filtros
settings-tab-shell = Shell
settings-tab-secure-delete = Eliminación segura
settings-tab-advanced = Avanzado
settings-tab-updater = Actualizaciones
settings-tab-profiles = Perfiles

# General tab additions
settings-section-theme = Tema
settings-theme-auto = Automático
settings-theme-light = Claro
settings-theme-dark = Oscuro
settings-start-with-os = Iniciar al arrancar el sistema
settings-single-instance = Una sola instancia en ejecución
settings-minimize-to-tray = Minimizar a la bandeja al cerrar
settings-error-display-mode = Estilo de aviso de errores
settings-error-display-modal = Ventana modal (bloquea la app)
settings-error-display-drawer = Panel lateral (no bloqueante)
settings-error-display-mode-hint = La ventana modal detiene la cola hasta que decidas. El panel lateral mantiene la cola en marcha y te permite gestionar los errores en una esquina.
settings-paste-shortcut = Pegar archivos con un atajo global
settings-paste-shortcut-combo = Combinación de teclas
settings-paste-shortcut-hint = Pulsa esta combinación en cualquier parte del sistema para pegar archivos copiados desde Explorer / Finder / Files con Copy That. CmdOrCtrl equivale a Cmd en macOS y a Ctrl en Windows / Linux.
settings-clipboard-watcher = Vigilar el portapapeles en busca de archivos copiados
settings-clipboard-watcher-hint = Muestra un aviso cuando aparecen URL de archivos en el portapapeles, sugiriendo que puedes pegarlos con Copy That. Consulta cada 500 ms mientras está activado.

# Transfer tab
settings-buffer-size = Tamaño del búfer
settings-verify = Verificar después de copiar
settings-verify-off = Desactivado
settings-concurrency = Concurrencia
settings-concurrency-auto = Automática
settings-reflink = Reflink / rutas rápidas
settings-reflink-prefer = Preferir
settings-reflink-avoid = Evitar reflink
settings-reflink-disabled = Usar siempre el motor asíncrono
settings-fsync-on-close = Sincronizar con el disco al cerrar (más lento, más seguro)
settings-preserve-timestamps = Conservar marcas de tiempo
settings-preserve-permissions = Conservar permisos
settings-preserve-acls = Conservar ACL (Fase 14)
settings-preserve-sparseness = Conservar archivos dispersos
settings-preserve-sparseness-hint = Copia solo las extensiones asignadas de los archivos dispersos (discos de VM, archivos de bases de datos) para que el destino ocupe en disco lo mismo que el origen.
settings-force-parallel-chunks = Copia paralela en múltiples fragmentos (solo RAID / matrices)
settings-force-parallel-chunks-hint = Divide cada copia grande en fragmentos concurrentes. Solo ayuda en destinos en franjas/RAID/red; RALENTIZA un único SSD/NVMe (-25% a -76%). Déjalo desactivado salvo que el destino sea una matriz de varios discos.

# Shell tab
settings-context-menu = Habilitar las entradas del menú contextual del shell
settings-intercept-copy = Interceptar el controlador de copia predeterminado (Windows)
settings-intercept-copy-hint = Cuando está activado, el Ctrl+C / Ctrl+V de Explorer pasa por Copy That. El registro llega en la Fase 14.
settings-notify-completion = Notificar al completar el trabajo

# Secure delete tab
settings-shred-method = Método de borrado predeterminado
settings-shred-zero = Ceros (1 pasada)
settings-shred-random = Aleatorio (1 pasada)
settings-shred-dod3 = DoD 5220.22-M (3 pasadas)
settings-shred-dod7 = DoD 5220.22-M (7 pasadas)
settings-shred-gutmann = Gutmann (35 pasadas)
settings-shred-nist = NIST 800-88
settings-shred-confirm-twice = Exigir doble confirmación antes de borrar

# Advanced tab
settings-log-level = Nivel de registro
settings-log-off = Desactivado
settings-telemetry = Telemetría
settings-telemetry-never = Nunca — sin envío de datos en ningún nivel de registro
settings-error-policy = Política de errores predeterminada
settings-error-policy-ask = Preguntar
settings-error-policy-skip = Omitir
settings-error-policy-retry = Reintentar con espera progresiva
settings-error-policy-abort = Anular al primer fallo
settings-history-retention = Retención del historial (días)
settings-history-retention-hint = 0 = conservar para siempre. Cualquier otro valor purga automáticamente los trabajos más antiguos al iniciar.
settings-database-path = Ruta de la base de datos
settings-database-path-default = (predeterminada — directorio de datos del sistema)
settings-reset-all = Restablecer valores predeterminados
settings-reset-confirm = ¿Restablecer todas las preferencias a sus valores predeterminados? Los perfiles no se ven afectados.

# Profiles tab
settings-profiles-hint = Guarda la configuración actual con un nombre; cárgala más tarde para volver a ella sin tocar cada ajuste por separado.
settings-profile-name-placeholder = Nombre del perfil
settings-profile-save = Guardar
settings-profile-import = Importar…
settings-profile-load = Cargar
settings-profile-export = Exportar…
settings-profile-delete = Eliminar
settings-profile-empty = Aún no hay perfiles guardados.
settings-profile-import-prompt = Nombre para el perfil importado:

# Toasts driven by Phase 12 profile actions
toast-settings-reset = Configuración restablecida
toast-profile-saved = Perfil guardado
toast-profile-loaded = Perfil cargado
toast-profile-exported = Perfil exportado
toast-profile-imported = Perfil importado

# Phase 14a — enumeration-time filters
settings-filters-hint = Omite archivos en el momento de enumerarlos para que el motor ni siquiera los abra. Las inclusiones se aplican solo a archivos; las exclusiones también recortan los directorios coincidentes.
settings-filters-enabled = Habilitar filtros para copias de árboles
settings-filters-include-globs = Globs de inclusión
settings-filters-include-globs-placeholder = **/*.txt
settings-filters-include-globs-hint = Un glob por línea. Cuando no está vacío, un archivo debe coincidir con al menos una inclusión para conservarse. Siempre se desciende a los directorios.
settings-filters-exclude-globs = Globs de exclusión
settings-filters-exclude-globs-placeholder = **/node_modules
settings-filters-exclude-globs-hint = Un glob por línea. Las coincidencias recortan todo el subárbol en los directorios; los archivos coincidentes se omiten.
settings-filters-size-range = Rango de tamaño de archivo
settings-filters-min-size-bytes = Tamaño mínimo (bytes, en blanco = sin mínimo)
settings-filters-max-size-bytes = Tamaño máximo (bytes, en blanco = sin máximo)
settings-filters-date-range = Rango de fecha de modificación
settings-filters-min-mtime = Modificado el o después de
settings-filters-max-mtime = Modificado el o antes de
settings-filters-attributes = Bits de atributo
settings-filters-skip-hidden = Omitir archivos y carpetas ocultos
settings-filters-skip-system = Omitir archivos del sistema (solo Windows)
settings-filters-skip-readonly = Omitir archivos de solo lectura

# Phase 15 — auto-update
settings-updater-hint = Copy That busca actualizaciones firmadas como mucho una vez al día. Las actualizaciones se instalan al cerrar la app.
settings-updater-auto-check = Buscar actualizaciones al iniciar
settings-updater-channel = Canal de versiones
settings-updater-channel-stable = Estable
settings-updater-channel-beta = Beta (versión preliminar)
settings-updater-last-check = Última comprobación
settings-updater-last-never = Nunca
settings-updater-check-now = Buscar actualizaciones ahora
settings-updater-checking = Comprobando…
settings-updater-available = Actualización disponible
settings-updater-up-to-date = Tienes la versión más reciente.
settings-updater-dismiss = Omitir esta versión
settings-updater-dismissed = Omitida
toast-update-available = Hay una versión más reciente disponible
toast-update-up-to-date = Ya tienes la versión más reciente

# Phase 19a — disk-backed file enumeration (TeraCopy-compatible scan DB)
scan-progress-title = Analizando…
scan-progress-stats = { $files } archivos · { $bytes } hasta ahora
scan-pause-button = Pausar análisis
scan-resume-button = Reanudar análisis
scan-cancel-button = Cancelar análisis
scan-cancel-confirm = ¿Cancelar el análisis y descartar el progreso?
scan-db-header = Base de datos de análisis
scan-db-hint = Base de datos de análisis en disco para trabajos de millones de archivos.
advanced-scan-hash-during = Calcular sumas de verificación durante el análisis
advanced-scan-db-path = Ubicación de la base de datos de análisis
advanced-scan-retention-days = Eliminar automáticamente los análisis completados tras (días)
advanced-scan-max-keep = Máximo de bases de datos de análisis que conservar

# Phase 19b — filesystem-snapshot source for locked files.
settings-on-locked = Cuando un archivo está bloqueado
settings-on-locked-ask = Preguntar la primera vez
settings-on-locked-retry = Reintentar brevemente y luego mostrar el error
settings-on-locked-skip = Omitir el archivo bloqueado
settings-on-locked-snapshot = Usar una instantánea del sistema de archivos
settings-on-locked-hint = Elimina los errores de "archivo en uso por otro proceso". Copy That toma una instantánea del volumen de origen (VSS en Windows, ZFS/Btrfs en Linux, APFS en macOS) y lee desde la copia de la instantánea.
snapshot-prompt-title = Este archivo está en uso por otro proceso
snapshot-prompt-body = Otro programa tiene { $path } abierto con escritura exclusiva. Elige cómo debe gestionar Copy That este y otros archivos similares del mismo volumen.
snapshot-source-active = 📷 Leyendo desde la instantánea { $kind } de { $volume }
snapshot-create-failed = No se pudo crear una instantánea del volumen de origen
snapshot-vss-needs-elevation = Leer desde una instantánea VSS requiere permisos de administrador. Copy That te pedirá que lo autorices.
snapshot-cleanup-failed = El asistente de instantáneas informó de un fallo de limpieza — puede quedar una copia sombra residual en el volumen.

# Phase 20 — durable resume journal.
resume-prompt-title = ¿Reanudar las transferencias anteriores?
resume-prompt-body = Copy That detectó { $count } transferencia(s) sin terminar de una sesión anterior. Elige qué hacer con cada una.
resume-prompt-resume = Reanudar
resume-prompt-resume-all = Reanudar todas
resume-discard-one = No reanudar
resume-discard-all = Descartar todas
resume-aborted-hash-mismatch = Los primeros { $offset } bytes del destino no coinciden con el origen — reiniciando desde el principio.
settings-auto-resume = Reanudar automáticamente los trabajos interrumpidos sin preguntar
settings-auto-resume-hint = Omite el aviso de reanudación al iniciar y vuelve a poner en cola en silencio cada trabajo sin terminar. Desactivado de forma predeterminada.

# Phase 21 — bandwidth shaping (GCRA token bucket + schedule + auto-throttle).
settings-tab-network = Red
settings-network-hint = Limita la velocidad de transferencia para que el resto de la red siga siendo usable. Aplícalo globalmente, sigue un horario diario o reacciona automáticamente a conexiones Wi-Fi de uso medido, batería o móviles.
settings-network-mode = Límite de ancho de banda
settings-network-mode-off = Desactivado (sin límite)
settings-network-mode-fixed = Valor fijo
settings-network-mode-schedule = Usar horario
settings-network-cap-mbps = Límite (MB/s)
settings-network-schedule = Horario (formato rclone)
settings-network-schedule-hint = Límites HH:MM,tasa separados por espacios más reglas de día opcionales Lun-Vie,tasa. Tasas: 512k, 10M, 2G, off, unlimited. Ejemplo: 08:00,512k 18:00,10M Sat-Sun,unlimited.
settings-network-auto-header = Limitación automática
settings-network-auto-metered = En Wi-Fi de uso medido
settings-network-auto-battery = Con batería
settings-network-auto-cellular = En red móvil
settings-network-auto-unchanged = No anular
settings-network-auto-pause = Pausar transferencias
settings-network-auto-cap = Limitar a un valor fijo
shape-badge-paused = en pausa
shape-badge-tooltip = Límite de ancho de banda activo — haz clic para abrir Configuración → Red
shape-badge-source-schedule = programado
shape-badge-source-metered = uso medido
shape-badge-source-battery = con batería
shape-badge-source-cellular = red móvil
shape-badge-source-settings = activo
shape-error-schedule-invalid = El formato del horario no es válido: { $message }

# Phase 22 — aggregate conflict dialog v2 (thumbnails, per-pattern
# rules, and reusable conflict profiles). Every key below is user-
# visible text in the `ConflictBatchModal.svelte` component.
conflict-batch-title = { $count } conflictos de archivos en { $jobname }
conflict-batch-state-pending = Pendiente
conflict-batch-state-resolved = Resuelto
conflict-batch-action-overwrite = Sobrescribir
conflict-batch-action-skip = Omitir
conflict-batch-action-keep-both = Conservar ambos
conflict-batch-action-newer-wins = Gana el más reciente
conflict-batch-action-larger-wins = Gana el más grande
conflict-batch-bulk-apply-selected = Aplicar a los seleccionados
conflict-batch-bulk-apply-extension = Aplicar a todos los de esta extensión
conflict-batch-bulk-apply-glob = Aplicar a los que coincidan con el glob…
conflict-batch-bulk-apply-remaining = Aplicar a todos los restantes
conflict-batch-bulk-glob-placeholder = p. ej. **/*.tmp
conflict-batch-save-profile = Guardar estas reglas como perfil…
conflict-batch-profile-placeholder = Nombre del perfil
conflict-batch-matched-rule = mediante la regla '{ $rule }' → { $action }
conflict-batch-empty = Todos los conflictos resueltos
conflict-batch-source-vs-destination = Origen frente a destino
conflict-batch-source-label = Origen
conflict-batch-destination-label = Destino
conflict-batch-size-label = Tamaño
conflict-batch-modified-label = Modificado
conflict-batch-close = Cerrar
conflict-batch-profile-saved = Perfil de conflictos guardado

# Phase 23 — sparse-file preservation. The toast fires once per
# destination volume when the filesystem can't preserve holes; the
# warning line is surfaced in the job detail drawer so the user knows
# the dst is larger on disk than the source was.
sparse-not-supported-title = El destino rellena los archivos dispersos
sparse-not-supported-body = { $dst_fs } no admite archivos dispersos. Los huecos del origen se escribieron como ceros, así que el destino ocupa más en disco.
sparse-warning-densified = Estructura dispersa conservada: solo se copiaron las extensiones asignadas.
sparse-warning-mismatch = Discrepancia en la estructura dispersa — el destino puede ser más grande de lo esperado.

# Phase 24 — security-metadata preservation. The Mark-of-the-Web
# (Zone.Identifier ADS) toggle is security-sensitive: turning it off
# lets a downloaded executable shed its SmartScreen / Office Protected
# View flag on copy, which is why the tooltip carries an explicit
# warning. AppleDouble fallback emits `._<filename>` sidecars on
# destination filesystems that can't hold the foreign metadata.
settings-preserve-security-metadata = Conservar metadatos de seguridad
settings-preserve-security-metadata-hint = Captura y vuelve a aplicar los flujos de metadatos fuera de banda (NTFS ADS / xattrs / ACL POSIX / contextos SELinux / capacidades de archivo de Linux / bifurcaciones de recursos de macOS) en cada copia.
settings-preserve-motw = Conservar la Marca de la Web (indicador de descargado de internet)
settings-preserve-motw-hint = Crítico para la seguridad. SmartScreen y la Vista protegida de Office usan este flujo para advertir sobre archivos descargados de internet. Desactivarlo permite que un ejecutable descargado pierda su marca de origen al copiarlo y eluda las protecciones del sistema operativo.
settings-preserve-posix-acls = Conservar ACL POSIX y atributos extendidos
settings-preserve-posix-acls-hint = Transfiere los xattrs user.* / system.* / trusted.* y las listas de control de acceso POSIX en la copia.
settings-preserve-selinux = Conservar contextos SELinux
settings-preserve-selinux-hint = Transfiere la etiqueta security.selinux en la copia para que los demonios que se ejecutan bajo políticas MAC sigan pudiendo acceder al archivo.
settings-preserve-resource-forks = Conservar bifurcaciones de recursos y datos del Finder de macOS
settings-preserve-resource-forks-hint = Transfiere la bifurcación de recursos heredada y FinderInfo (etiquetas de color, metadatos de Carbon) en la copia.
settings-appledouble-fallback = Usar archivos adjuntos AppleDouble en sistemas de archivos incompatibles
meta-translated-to-appledouble = Metadatos externos almacenados en un archivo adjunto AppleDouble (._{ $ext })

# Phase 25 — two-way sync with vector-clock conflict detection.
# The drawer lists configured sync pairs; each pair runs an
# independent reconciliation round against a per-pair `.copythat-sync.db`
# state store. Concurrent edits from a common ancestor surface as
# conflicts rather than silent overwrites; the losing side's content
# is preserved as `name.sync-conflict-YYYYMMDD-HHMMSS-<host>.ext`.
footer-sync = Sincronización
sync-drawer-title = Sincronización bidireccional
sync-drawer-hint = Mantén dos carpetas sincronizadas sin sobrescrituras silenciosas. Las ediciones simultáneas aparecen como conflictos que puedes resolver.
sync-add-pair = Añadir par
sync-add-cancel = Cancelar
sync-refresh = Actualizar
sync-add-save = Guardar par
sync-add-saving = Guardando…
sync-add-missing-fields = La etiqueta, la ruta izquierda y la ruta derecha son obligatorias.
sync-remove-confirm = ¿Quitar este par de sincronización? La base de datos de estado se conserva; las carpetas no se tocan.
sync-field-label = Etiqueta
sync-field-label-placeholder = p. ej. Documentos ↔ NAS
sync-field-left = Carpeta izquierda
sync-field-left-placeholder = Elige o pega una ruta absoluta
sync-field-right = Carpeta derecha
sync-field-right-placeholder = Elige o pega una ruta absoluta
sync-field-mode = Modo
sync-mode-two-way = Bidireccional
sync-mode-mirror-left-to-right = Reflejar (izquierda → derecha)
sync-mode-mirror-right-to-left = Reflejar (derecha → izquierda)
sync-mode-contribute-left-to-right = Aportar (izquierda → derecha, sin eliminar)
sync-no-pairs = Aún no hay pares de sincronización configurados. Haz clic en "Añadir par" para empezar.
sync-loading = Cargando los pares configurados…
sync-never-run = Nunca ejecutado
sync-running = En curso
sync-run-now = Ejecutar ahora
sync-cancel = Cancelar
sync-remove-pair = Quitar
sync-view-conflicts = Ver conflictos ({ $count })
sync-conflicts-heading = Conflictos
sync-no-conflicts = No hay conflictos de la última ejecución.
sync-winner = Ganador
sync-side-left-to-right = izquierda
sync-side-right-to-left = derecha
sync-conflict-kind-concurrent-write = Edición simultánea
sync-conflict-kind-delete-edit = Eliminar ↔ editar
sync-conflict-kind-add-add = Añadido en ambos lados
sync-conflict-kind-corrupt-equal = El contenido divergió sin una nueva escritura
sync-resolve-keep-left = Conservar la izquierda
sync-resolve-keep-right = Conservar la derecha
sync-resolve-keep-both = Conservar ambos
sync-resolve-three-way = Resolver con una fusión a tres bandas
sync-resolve-phase-53-tooltip = La fusión interactiva a tres bandas para archivos no de texto llega en la Fase 53.
sync-error-prefix = Error de sincronización

# Phase 26 — real-time mirror watcher. "Live mirror" starts a
# filesystem watcher on the pair's left side; every debounced event
# triggers a re-sync. The watcher filters vim swap files / Office
# lock files / atomic-save temp names so one logical save becomes
# exactly one sync round.
live-mirror-start = Iniciar reflejo en vivo
live-mirror-stop = Detener reflejo en vivo
live-mirror-watching = Vigilando
live-mirror-toggle-hint = Vuelve a sincronizar automáticamente en cada cambio detectado en el sistema de archivos. Un hilo en segundo plano por cada par activo.
watch-event-prefix = Cambio de archivo
watch-overflow-recovered = El búfer del vigilante se desbordó; reenumerando para recuperarse

# Phase 27 — content-defined chunk store. Enables delta-resume (a
# retry only re-writes chunks that actually changed) and same-job
# dedup (files sharing content blocks store those blocks once). The
# store is disk-backed under `<data-dir>/chunks/` by default and is
# the foundation for the Phase 49–51 moonshot repository phases.
chunk-store-section = Almacén de fragmentos
chunk-store-enable = Habilitar el almacén de fragmentos (reanudación delta y deduplicación)
chunk-store-enable-hint = Divide cada archivo copiado por contenido (FastCDC) y almacena los fragmentos por dirección de contenido. Los reintentos solo reescriben los fragmentos cambiados; los archivos con contenido compartido se deduplican automáticamente.
chunk-store-location = Ubicación del almacén de fragmentos
chunk-store-max-size = Tamaño máximo del almacén de fragmentos
chunk-store-prune = Eliminar fragmentos de más de (días)
chunk-store-savings = { $gib } GiB ahorrados mediante deduplicación de fragmentos
chunk-store-disk-usage = Usando { $size } en { $chunks } fragmentos

# Phase 28 — tray-resident Drop Stack. The stack is a persistent list
# of paths gathered from multiple sources (Explorer drag, main-window
# context menu, CLI, drag onto the Drop Stack window) that the user
# can dispatch to a destination in one go.
dropstack-window-title = Pila de elementos
dropstack-tray-open = Pila de elementos
dropstack-empty-title = La pila de elementos está vacía
dropstack-empty-hint = Arrastra archivos aquí desde Explorer o haz clic derecho en la fila de un trabajo para añadirlo.
dropstack-add-to-stack = Añadir a la pila de elementos
dropstack-copy-all-to = Copiar todo en…
dropstack-move-all-to = Mover todo a…
dropstack-clear = Vaciar la pila
dropstack-remove-row = Quitar de la pila
dropstack-path-missing-toast = { $path } soltado — el archivo ya no existe.
dropstack-always-on-top = Mantener la pila de elementos siempre visible
dropstack-show-tray-icon = Mostrar el icono de Copy That en la bandeja
dropstack-open-on-start = Abrir la pila de elementos automáticamente al iniciar la app
dropstack-count = { $count } ruta

# Phase 29 — spring-loaded folders + native DnD polish. The Settings
# → General tab carries the knobs; DropTarget / DestinationPicker
# Svelte components use the dropzone-* keys at runtime.
settings-dnd-heading = Arrastrar y soltar
settings-dnd-spring-load = Abrir carpetas al pasar por encima mientras se arrastra
settings-dnd-spring-delay = Retardo de apertura automática (ms)
settings-dnd-thumbnails = Mostrar miniaturas al arrastrar
settings-dnd-invalid-highlight = Resaltar los destinos no válidos
dropzone-invalid-title = No es un destino válido
dropzone-invalid-readonly = El destino es de solo lectura
dropzone-picker-title = Elige un destino
dropzone-picker-up = Subir
dropzone-picker-path = Ruta actual
dropzone-picker-root = Raíces
dropzone-picker-use-this = Usar esta carpeta
dropzone-picker-empty = No hay subcarpetas
dropzone-picker-cancel = Cancelar

# Phase 30 — cross-platform path translation. Settings → Transfer
# exposes these under a "Cross-platform compatibility" subsection.
translate-heading = Compatibilidad multiplataforma
translate-unicode-label = Normalización Unicode
translate-unicode-auto = Detectar el destino automáticamente
translate-unicode-windows = NFC (Windows / Linux)
translate-unicode-macos = Dejar tal cual (macOS / APFS)
translate-line-endings-label = Convertir los finales de línea de los archivos de texto
translate-line-endings-allowlist = Extensiones de archivos de texto
reserved-name-label = Tratamiento de nombres reservados de Windows
reserved-name-suffix = Añadir "_" (CON.txt → CON_.txt)
reserved-name-reject = Rechazar y advertir
long-path-label = Usar el prefijo de ruta larga de Windows (\\?\) cuando supere los 260 caracteres
long-path-hint = Algunos recursos compartidos de red y herramientas antiguas no respetan el espacio de nombres \\?\.

# Phase 31 — power-aware copying. Settings → Power & State tab + the
# header badge that renders "⏸ Paused — Zoom call detected" when the
# runner's power subscriber has paused or capped due to a policy match.
power-heading = Energía y estado
power-enabled = Habilitar reglas según la energía
power-battery-label = Con batería
power-metered-label = En Wi-Fi de uso medido
power-cellular-label = En red móvil
power-presentation-label = Al presentar (Zoom / Teams / Keynote)
power-fullscreen-label = Cuando una app está en pantalla completa
power-thermal-label = Cuando la CPU está limitada por temperatura
power-rule-continue = Continuar a toda velocidad
power-rule-pause = Pausar todos los trabajos
power-rule-cap = Limitar el ancho de banda
power-rule-cap-percent = Limitar a un porcentaje de la velocidad actual
power-reason-on-battery = con batería
power-reason-metered-network = red de uso medido
power-reason-cellular-network = red móvil
power-reason-presenting = modo de presentación
power-reason-fullscreen = app en pantalla completa
power-reason-thermal-throttling = la CPU está limitada

# Phase 32 — cloud backend matrix via OpenDAL. Settings → Remotes
# tab + the Add-backend wizard that writes one entry per remote into
# the keychain-backed credential store.
remote-heading = Backends remotos
remote-add = Añadir backend
remote-list-empty = No hay backends remotos configurados
remote-test = Probar conexión
remote-test-success = Conexión correcta
remote-test-failed = La conexión falló
remote-remove = Quitar backend
remote-name-label = Nombre visible
remote-kind-label = Tipo de backend
remote-save = Guardar backend
remote-cancel = Cancelar
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
backend-local-fs = Sistema de archivos local
cloud-config-bucket = Bucket
cloud-config-region = Región
cloud-config-endpoint = URL del endpoint
cloud-config-root = Ruta raíz
cloud-error-invalid-config = La configuración del backend no es válida
cloud-error-network = Error de red al contactar con el backend
cloud-error-not-found = No se encontró el objeto en la ruta solicitada
cloud-error-permission = Permiso denegado por el backend remoto
cloud-error-keychain = Falló el acceso al llavero del sistema
settings-tab-remotes = Remotos
settings-tab-mobile = Móvil

# Phase 33 — mount Copy That's chunk store + history archive as a
# read-only filesystem (FUSE / WinFsp). Surfaced by the History tab
# context menu's "Mount snapshot" action + the Settings → Advanced
# "Mount latest on launch" toggle.
mount-heading = Montar instantánea
mount-action-mount = Montar instantánea
mount-action-unmount = Desmontar
mount-status-mounted = Montada en { $path }
mount-error-unsafe-mountpoint = La ruta del punto de montaje no es segura
mount-error-mountpoint-not-empty = El punto de montaje debe ser un directorio vacío
mount-error-backend-unavailable = El backend de montaje no está disponible en este sistema
mount-error-archive-read = Falló la lectura del archivo de almacenamiento
mount-picker-title = Elige el directorio del punto de montaje
mount-toast-mounted = Instantánea montada en { $path }
mount-toast-unmounted = Instantánea desmontada
mount-toast-failed = El montaje falló: { $reason }
settings-mount-heading = Montar instantáneas
settings-mount-hint = Expón el archivo de almacenamiento del historial como un sistema de archivos de solo lectura. La Fase 33b conecta el flujo del ejecutor; los backends de kernel FUSE/WinFsp llegan en la Fase 33c.
settings-mount-on-launch = Montar la última instantánea al iniciar
settings-mount-on-launch-path = Ruta del punto de montaje
settings-mount-on-launch-path-placeholder = p. ej. C:\Mounts\copythat

# Phase 34 — enterprise-grade audit log export + WORM (write-once-read-
# many) tamper-resistance. Settings → Advanced → Audit log collects the
# format + destination + rotation + WORM toggle; runtime records job and
# file events into the tamper-evident chain-hashed sink.
settings-audit-heading = Registro de auditoría
settings-audit-hint = Registro de solo anexado y a prueba de manipulaciones de cada evento de trabajo y archivo. Los formatos incluyen CSV, líneas JSON, Syslog RFC 5424, ArcSight CEF y QRadar LEEF.
settings-audit-enable = Habilitar el registro de auditoría
settings-audit-format = Formato del registro
settings-audit-format-json-lines = Líneas JSON (predeterminado recomendado)
settings-audit-format-csv = CSV (compatible con hojas de cálculo)
settings-audit-format-syslog = Syslog (RFC 5424)
settings-audit-format-cef = CEF (ArcSight)
settings-audit-format-leef = LEEF 2.0 (IBM QRadar)
settings-audit-file-path = Ruta del archivo de registro
settings-audit-file-path-placeholder = p. ej. C:\ProgramData\CopyThat\audit.log
settings-audit-max-size = Rotar tras (bytes, 0 = nunca)
settings-audit-worm = Habilitar el modo WORM (escribir una vez, leer muchas)
settings-audit-worm-hint = Aplica el indicador de solo anexado de la plataforma (Linux chattr +a, macOS chflags uappnd, atributo de solo lectura de Windows) tras cada creación o rotación. Incluso un administrador debe borrar explícitamente el indicador para truncar el registro.
settings-audit-test-write = Escritura de prueba
settings-audit-verify-chain = Verificar cadena
toast-audit-test-write-ok = La escritura de prueba del registro de auditoría se realizó correctamente
toast-audit-verify-ok = La cadena de auditoría se verificó intacta
toast-audit-verify-failed = La verificación de la cadena de auditoría informó de discrepancias

# Phase 35 — destination encryption via age (X25519 / passphrase /
# SSH) + on-the-fly zstd compression with a per-extension deny
# heuristic. Surfaced through Settings → Transfer → Encryption +
# Compression; engine short-circuits to the crypt pipeline when
# either stage is active.
settings-crypt-heading = Cifrado y compresión
settings-crypt-hint = Transforma el contenido de los archivos antes de que lleguen al destino. El cifrado usa el formato age; la compresión usa zstd y puede omitir medios ya comprimidos según la extensión.
settings-crypt-encryption-mode = Cifrado
settings-crypt-encryption-off = Desactivado
settings-crypt-encryption-passphrase = Frase de contraseña (preguntar al iniciar la copia)
settings-crypt-encryption-recipients = Claves de destinatarios desde un archivo
settings-crypt-encryption-hint = Las frases de contraseña se conservan solo en memoria durante la copia. Los archivos de destinatarios indican una clave pública age1… o ssh- por línea.
settings-crypt-recipients-file = Ruta del archivo de destinatarios
settings-crypt-recipients-file-placeholder = p. ej. C:\Users\me\recipients.txt
settings-crypt-compression-mode = Compresión
settings-crypt-compression-off = Desactivada
settings-crypt-compression-always = Siempre
settings-crypt-compression-smart = Inteligente (omitir medios ya comprimidos)
settings-crypt-compression-hint = El modo inteligente omite jpg, mp4, zip, 7z y formatos similares que no se benefician de zstd. El modo Siempre comprime cada archivo al nivel elegido.
settings-crypt-compression-level = Nivel de zstd (1-22)
settings-crypt-compression-level-hint = Los números más bajos son más rápidos; los más altos comprimen más. El nivel 3 coincide con el predeterminado de la CLI de zstd.
compress-footer-savings = 💾 { $original } → { $compressed } ({ $percent }% ahorrado)
compress-savings-toast = Comprimido un { $percent }% ({ $bytes } ahorrados)
crypt-toast-recipients-loaded = { $count } destinatarios de cifrado cargados
crypt-toast-recipients-error = No se pudieron cargar los destinatarios: { $reason }
crypt-toast-passphrase-required = El cifrado necesita una frase de contraseña antes de empezar la copia
crypt-toast-passphrase-set = Frase de contraseña de cifrado capturada
crypt-footer-encrypted-badge = 🔒 Cifrado (age)
crypt-footer-compressed-badge = 📦 Comprimido (zstd)

# Phase 36 — copythat CLI surface. Documented exit codes + the
# user-facing strings for plan/apply/verify/config. Help text stays
# in English (engineering accessibility) but error / info / status
# strings are localized so a localized desktop installation surfaces
# CLI errors in the same language as the GUI.
cli-help-tagline = Copy That CLI — copia, sincronización, verificación y auditoría de archivos byte a byte para pipelines de CI/CD.
cli-help-exit-codes = Códigos de salida: 0 éxito, 1 error, 2 pendiente, 3 conflicto, 4 fallo de verificación, 5 red, 6 permisos, 7 disco lleno, 8 cancelado, 9 configuración.
cli-error-bad-args = copy/move requiere al menos un origen y un destino
cli-error-unknown-algo = Algoritmo de verificación desconocido: { $algo }
cli-error-missing-spec = --spec es obligatorio para plan/apply
cli-error-spec-parse = No se pudo analizar el jobspec { $path }: { $reason }
cli-error-spec-empty-sources = La lista de orígenes del jobspec está vacía
cli-info-shape-recorded = Forma de ancho de banda "{ $rate }" registrada; la aplicación se realiza mediante copythat-shape
cli-info-stub-deferred = { $command } queda preparado para la conexión posterior de la Fase 36
cli-plan-summary = Plan: { $actions } acción(es), { $bytes } byte(s); { $already_done } ya en su sitio
cli-plan-pending = El plan informa de acciones pendientes; vuelve a ejecutar con `apply` para llevarlas a cabo
cli-plan-already-done = El plan informa de que no hay nada que hacer (idempotente)
cli-apply-success = Apply terminó sin errores
cli-apply-failed = Apply terminó con uno o más errores
cli-verify-ok = Verificación correcta: { $algo } { $digest }
cli-verify-failed = Verificación FALLIDA para { $path } ({ $algo })
cli-config-set = { $key } = { $value } establecido
cli-config-reset = { $key } restablecido al valor predeterminado
cli-config-unknown-key = Clave de configuración desconocida: { $key }
cli-completions-emitted = Autocompletado del shell para { $shell } impreso en stdout

# Phase 37 — desktop-side mobile companion. Settings → Mobile panel
# strings + the SAS-confirmation modal + push-notification toasts.
# The actual mobile UI lives in the Phase 37 follow-up Tauri Mobile
# target; the phone displays its own localized SAS prompt.
settings-mobile-heading = Compañero móvil
settings-mobile-hint = Empareja un iPhone o un Android para consultar el historial, lanzar perfiles guardados y jobspecs de la Fase 36, y recibir notificaciones al completar.
settings-mobile-pair-toggle = Permitir nuevos emparejamientos
settings-mobile-pair-active = Servidor de emparejamiento activo — escanea el QR con la app móvil de Copy That
settings-mobile-pair-button = Iniciar emparejamiento
settings-mobile-revoke-button = Revocar
settings-mobile-no-pairings = Aún no hay dispositivos emparejados
settings-mobile-pair-port = Puerto de enlace (0 = elegir uno libre)
pair-sas-prompt = Ambas pantallas deberían mostrar los mismos cuatro emojis. Toca Coincide si concuerdan.
pair-sas-confirm = Coincide
pair-sas-reject = No coincide — cancelar
pair-toast-success = Emparejado con { $device }
pair-toast-failed = El emparejamiento falló: { $reason }
push-toast-sent = Notificación enviada a { $device }
push-toast-failed = La notificación a { $device } falló: { $reason }

# Phase 38 — aggregate destination dedup + reflink fallback ladder.
# Settings → Transfer → Dedup panel + per-job-row badges (⚡
# Reflinked / 🔗 Hardlinked / 🧩 Chunk-shared / 📋 Copied) +
# pre-pass dedup-scan modal.
settings-dedup-heading = Deduplicación en destino
settings-dedup-hint = Cuando el origen y el destino comparten volumen, Copy That puede clonar archivos a nivel del sistema de archivos en lugar de copiar bytes. El reflink es instantáneo y seguro; el enlace duro es más rápido, pero ambos nombres comparten estado.
settings-dedup-mode-auto = Escala automática (reflink → enlace duro → fragmento → copia)
settings-dedup-mode-reflink-only = Solo reflink
settings-dedup-mode-hardlink-aggressive = Agresivo (reflink + enlace duro incluso en archivos modificables)
settings-dedup-mode-off = Deshabilitado (siempre copiar bytes)
settings-dedup-hardlink-policy = Política de enlaces duros
settings-dedup-prescan = Analizar previamente el árbol de destino en busca de contenido duplicado
dedup-badge-reflinked = ⚡ Con reflink
dedup-badge-hardlinked = 🔗 Con enlace duro
dedup-badge-chunk-shared = 🧩 Fragmento compartido
dedup-badge-copied = 📋 Copiado
phase42-paranoid-verify-label = Verificación paranoica
phase42-paranoid-verify-hint = Descarta las páginas en caché del destino y vuelve a leer desde el disco para detectar mentiras de la caché de escritura y corrupción silenciosa. Es aproximadamente un 50 % más lenta que la verificación predeterminada; desactivada de forma predeterminada.
phase42-sharing-violation-retries-label = Intentos de reintento en archivos de origen bloqueados
phase42-sharing-violation-retries-hint = Cuántas veces reintentar cuando otro proceso mantiene el archivo de origen abierto con un bloqueo exclusivo. La espera se duplica en cada intento (50 ms / 100 ms / 200 ms de forma predeterminada). Predeterminado 3, igual que Robocopy /R:3.
phase42-cloud-placeholder-warning = { $name } es un archivo de OneDrive solo en la nube. Copiarlo iniciará una descarga — hasta { $size } por tu conexión de red.
phase42-defender-exclusion-hint = Para obtener el máximo rendimiento de copia, añade la carpeta de destino a las exclusiones de Microsoft Defender antes de las transferencias masivas. Consulta docs/PERFORMANCE_TUNING.md.

# Phase 39 — Browser-accessible recovery UI. Settings → Advanced
# exposes these strings; the recovery server itself renders askama
# templates that always read English — these eight keys are the
# Settings prose the user actually reads in their preferred locale.
settings-recovery-heading = Interfaz web de recuperación
settings-recovery-enable = Habilitar la interfaz web de recuperación
settings-recovery-bind-address = Dirección de enlace
settings-recovery-port = Puerto (0 = elegir uno libre)
settings-recovery-show-url = Mostrar URL y token
settings-recovery-rotate-token = Rotar token
settings-recovery-allow-non-loopback = Permitir enlace que no sea de bucle invertido
settings-recovery-non-loopback-warning = ADVERTENCIA: habilitar un enlace que no sea de bucle invertido expone la interfaz de recuperación a tu red local. Cualquiera que conozca el token podrá explorar tu historial de archivos y descargar archivos. Protégela con TLS o un proxy inverso si la red local no es de confianza.

# Phase 40 — SMB compression negotiation + cloud-VM offload helper.
# 6 SMB keys (header badge + Settings prose) + 6 cloud-offload keys
# (Remotes tab wizard for cross-cloud copy templates).
smb-compress-badge = 🗜 Compresión SMB: { $algo }
smb-compress-badge-tooltip = El tráfico de red hacia este destino se está comprimiendo en tránsito (SMB 3.1.1).
smb-compress-toast-saved = { $bytes } ahorrados en la red
smb-compress-algo-unknown = algoritmo desconocido
settings-smb-compress-heading = Compresión de red SMB
settings-smb-compress-hint = Negocia automáticamente la compresión de tráfico SMB 3.1.1 en destinos UNC. Una mejora gratuita en enlaces lentos; se ignora en destinos locales.
cloud-offload-heading = Asistente de descarga en VM en la nube
cloud-offload-hint = Cuando copias directamente entre dos nubes, genera una plantilla de despliegue que ejecuta la copia desde una VM efímera diminuta en la nube — los bytes nunca pasan por la red de tu portátil.
cloud-offload-render-button = Generar plantilla
cloud-offload-copy-clipboard = Copiar al portapapeles
cloud-offload-template-format = Formato de plantilla
cloud-offload-self-destruct-warning = La VM se apaga automáticamente tras { $minutes } minutos — confirma el rol IAM y la región antes de desplegar.

# Phase 41 — animated before/after tree-diff preview. The `Preview
# changes` modal renders the rolled-up plan before the engine starts
# work; 14 keys cover the title, the summary header / counts, the
# row-reason labels, and the two action buttons.
preview-modal-title = Vista previa de cambios
preview-summary-header = Qué va a ocurrir
preview-category-additions = { $count } adiciones
preview-category-replacements = { $count } reemplazos
preview-category-skips = { $count } omitidos
preview-category-conflicts = { $count } conflictos
preview-category-unchanged = { $count } sin cambios
preview-bytes-to-transfer = { $bytes } por transferir
preview-reason-source-newer = El origen es más reciente
preview-reason-dest-newer = El destino es más reciente — se omitirá
preview-reason-content-different = El contenido difiere
preview-reason-identical = Idéntico al origen
preview-button-run = Ejecutar el plan
preview-button-reduce = Reducir mi plan…

# Phase 42 — perceptual-hash visual-similarity dedup. Eight keys cover
# the pre-copy "looks visually identical" warning + the Settings panel.
perceptual-warn-title = Parece visualmente idéntico
perceptual-warn-body = { $name } en el destino parece coincidir con la imagen de origen. ¿Continuar copiando de todos modos?
perceptual-warn-keep-both = Conservar ambos
perceptual-warn-skip = Omitir este archivo
perceptual-warn-overwrite = Sobrescribir de todos modos
perceptual-settings-heading = Deduplicación por similitud visual
perceptual-settings-hint = Detecta imágenes visualmente idénticas en el destino antes de que se sobrescriban. El hash es perceptual (reconoce la misma imagen guardada de nuevo en otro formato), no byte a byte.
perceptual-settings-threshold-label = Umbral de aviso (más bajo = coincidencia más estricta)

# Phase 42 Part B — per-file rolling versions (Time Machine for any
# destination). 8 keys cover the version-list panel + retention picker.
version-list-heading = Versiones anteriores
version-list-empty = No hay versiones anteriores de este archivo
version-list-restore = Restaurar esta versión
version-retention-heading = Conservar versiones anteriores al sobrescribir
version-retention-none = Conservar todas las versiones para siempre
version-retention-last-n = Conservar las últimas { $n } versiones
version-retention-older-than-days = Descartar versiones de más de { $days } días
version-retention-gfs = Cada hora { $h } · diaria { $d } · semanal { $w } · mensual { $m }

# Phase 43 — forensic chain-of-custody manifests + BLAKE3 verified
# streaming. 14 keys cover the Settings → Provenance panel
# (heading, hint, toggles, TSA URL, signing-key management) plus
# the post-job manifest notification + the verify command's two
# headline result lines.
provenance-settings-heading = Cadena de custodia forense
provenance-settings-hint = Firma cada trabajo de copia con un manifiesto BLAKE3 + ed25519. Los revisores pueden volver a calcular el hash del árbol de destino más tarde y demostrar que no cambió ningún byte desde la copia.
provenance-settings-enable-default = Firmar cada nuevo trabajo de forma predeterminada
provenance-settings-show-after-job = Mostrar el manifiesto tras cada trabajo completado
provenance-settings-tsa-url-label = URL predeterminada de la autoridad de sellado de tiempo RFC 3161
provenance-settings-tsa-url-hint = Opcional. Cuando se establece, los manifiestos llevan un sello de tiempo TSA gratuito que demuestra que los bytes existían en ese momento. Déjalo vacío para omitirlo.
provenance-settings-keys-heading = Claves de firma
provenance-settings-keys-generate = Generar nueva clave
provenance-settings-keys-import = Importar clave…
provenance-settings-keys-export = Exportar clave pública…
provenance-job-completed-title = Manifiesto de procedencia guardado
provenance-job-completed-body = { $count } archivos firmados → { $path }
provenance-verify-clean = Manifiesto válido para { $count } archivos; firma { $sig }; raíz de Merkle correcta.
provenance-verify-tampered = Manifiesto NO VÁLIDO — { $tampered } manipulados, { $missing } faltan.
# Phase 43 post-review hardening — toast text for the Settings →
# Provenance buttons whose Tauri IPC has not yet landed.
provenance-action-staged = Fase 43 — la conexión de la IPC para esta acción llega en un commit posterior.

# Phase 44 — SSD-aware whole-drive sanitize (NVMe Sanitize / OPAL
# Crypto Erase) + the per-file shred refusal on copy-on-write
# filesystems. 16 keys cover the new "Drive sanitize" Settings
# subsection and the localized error messages.
sanitize-heading = Saneamiento seguro de toda la unidad
sanitize-hint = NVMe Sanitize, OPAL Crypto Erase y ATA Secure Erase borran una unidad flash en la capa del firmware en milisegundos. La sobrescritura por archivo no tiene sentido en flash — el borrado de varias pasadas solo desgasta la NAND. Usa esto para un purgado real.
sanitize-pick-device = Elige la unidad que sanear
sanitize-mode-label = Método de saneamiento
sanitize-mode-nvme-format = NVMe Format (con borrado seguro)
sanitize-mode-nvme-sanitize-block = NVMe Sanitize — Block Erase (lento, todas las celdas)
sanitize-mode-nvme-sanitize-crypto = NVMe Sanitize — Crypto Erase (instantáneo)
sanitize-mode-ata-secure-erase = ATA Secure Erase (SSD SATA antiguos)
sanitize-mode-opal-crypto-erase = TCG OPAL Crypto Erase (unidades con autocifrado)
sanitize-mode-apfs-crypto-erase = APFS Crypto Erase (rotar la clave de FileVault, solo macOS)
sanitize-confirm-1 = Esto destruye TODOS los bytes de { $device }. No hay deshacer.
sanitize-confirm-2 = Entiendo que todas las particiones, todos los archivos y todas las instantáneas de { $device } quedarán ilegibles de forma permanente.
sanitize-confirm-3 = Escribe el nombre del modelo de la unidad para continuar: { $model }
sanitize-running = Saneando { $device } ({ $mode }) — esto puede tardar desde milisegundos (crypto erase) hasta decenas de minutos (block erase). No apagues el equipo.
sanitize-completed = Saneamiento completado — { $device } ahora está en blanco.
ssd-honest-shred-meaningless = La sobrescritura por archivo en un sistema de archivos de copia sobre escritura (Btrfs / ZFS / APFS) no puede llegar a los bloques subyacentes. Usa el saneamiento de toda la unidad y la rotación de la clave de cifrado de disco completo en su lugar.
ssd-honest-advisory = Este archivo está en flash. La sobrescritura por archivo desgasta la NAND y NO garantiza que las celdas originales sean irrecuperables. Para datos sensibles, sanea toda la unidad.

# Phase 44.1f post-review — placeholder toast for SanitizeTab
# buttons whose Tauri IPC has not yet landed.
sanitize-action-staged = Fase 44.1 — la conexión de la IPC para esta acción llega en un commit posterior.

# Phase 45.3 — named-queue tab strip (Subfeature A). Tabs surface
# once the QueueRegistry holds at least one queue; the synthesised
# default tab keeps legacy single-queue jobs reachable.
queue-tab-default = Predeterminada
queue-tab-empty-state = Colas de trabajos
queue-badge-tooltip = Trabajos pendientes y en curso en esta cola

# Phase 45.4 — drag-progress-merge (Subfeature B). Drag a queue tab
# onto another to merge their job lists. The default tab is neither
# draggable nor a drop target; only registry queues participate.
queue-drag-hint = Arrastra sobre otra cola para fusionarlas
queue-merge-confirm = Suelta para fusionar
queue-merge-toast = Colas fusionadas

# Phase 45.5 — F2-queue UX (Subfeature C). F2 toggles
# `auto_enqueue_next` so every fresh enqueue piles into the running
# queue rather than spawning a parallel one. Status pill renders in
# the Footer; pulsing dot renders on the running tab.
queue-f2-active-hint = Modo F2: cada nuevo encolado va a parar a esta cola
queue-f2-toggled-on = Modo de cola F2 ACTIVADO — los nuevos encolados se unen a la cola en curso
queue-f2-toggled-off = Modo de cola F2 DESACTIVADO — los nuevos encolados generan colas paralelas
queue-f2-status-bar = Modo de cola F2: ACTIVADO

# Phase 45.6 — tray destination targets (Subfeature D). Pinned
# destinations appear in the OS tray menu; clicking one arms it as
# the active drop target so the next file drop bypasses the
# DropStagingDialog. Settings → General hosts the list editor.
tray-target-section-title = Destinos de la bandeja
tray-target-section-hint = Los destinos fijados aparecen en el menú de la bandeja. Haz clic en uno para activarlo como próximo destino.
tray-target-empty = Aún no hay destinos de la bandeja fijados.
tray-target-remove = Quitar
tray-target-add-label = Etiqueta
tray-target-add-path = Ruta o URI de backend
tray-target-add = Añadir
tray-target-armed-toast = Suelta tu próximo archivo para enviarlo a { $label }
tray-target-active-pill = → { $label }

# Phase 45.7 follow-up — pinned-destination validation errors. The
# `queue_pin_destination` IPC returns these Fluent keys verbatim
# when input fails the IPC-boundary checks; the toast layer renders
# them via `t(...)` (Toast.svelte detects kebab-case-lowercase and
# routes through the locale table). Phase 17e's `err-path-escape` /
# `err-destination-empty` cover the path-traversal + empty cases for
# `queue_route_job`; the keys below are pin-specific.
err-pinned-destination-label-empty = La etiqueta del destino de la bandeja no puede estar vacía.
err-pinned-destination-path-empty = La ruta del destino de la bandeja no puede estar vacía.
err-pinned-destination-label-too-long = La etiqueta del destino de la bandeja es demasiado larga (máx. 64 caracteres).
err-pinned-destination-path-too-long = La ruta del destino de la bandeja es demasiado larga (máx. 1024 caracteres).
err-pinned-destination-label-invalid = La etiqueta del destino de la bandeja contiene caracteres no permitidos (salto de línea, retorno o NUL).
err-pinned-destination-path-invalid = La ruta del destino de la bandeja contiene caracteres no permitidos (salto de línea, retorno o NUL).
err-pinned-destination-too-many = Has alcanzado el límite de 50 destinos de la bandeja. Quita uno para añadir otro.

# Phase 46.6 — Settings → Plugins tab. The IPC layer in
# `apps/copythat-ui/src-tauri/src/plugin_commands.rs` enumerates the
# per-user plugin store under `<config_dir>/plugins/`; the
# `PluginsTab.svelte` component renders these strings against the
# returned manifest + grant state.
settings-tab-plugins = Complementos
plugin-heading = Complementos
plugin-hint = Los complementos WASM en espacio aislado amplían Copy That con enlaces personalizados. Cada complemento se ejecuta con límites de CPU y memoria por llamada y solo ve las capacidades del host que le concedas.
plugin-list-empty = Aún no hay complementos instalados.
plugin-enabled = Habilitado
plugin-disabled = Deshabilitado
plugin-hooks = Enlaces
plugin-capabilities = Capacidades
plugin-no-capabilities = (ninguna)
plugin-directory = Ubicación
plugin-install-from-file = Instalar desde archivo…
plugin-install-from-url = Instalar desde URL…
plugin-url-wasm = URL del WASM
plugin-url-manifest = URL del manifiesto
plugin-url-hash = Hash BLAKE3
plugin-url-preview = Vista previa
plugin-url-confirm = Confirmar instalación
