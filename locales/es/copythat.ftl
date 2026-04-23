app-name = Copy That v1.0.0
# MT
window-title = Copy That v1.0.0
# MT
shred-ssd-advisory = Advertencia: este destino se encuentra en un SSD. Las sobrescrituras de varias pasadas no desinfectan de forma fiable la memoria flash porque el nivelado de desgaste y el sobreaprovisionamiento mueven los datos fuera de la dirección lógica de bloque. Para medios de estado sólido, prefiera ATA SECURE ERASE, NVMe Format con borrado seguro o cifrado de disco completo con una clave descartada.

# MT
state-idle = Inactivo
# MT
state-copying = Copiando
# MT
state-verifying = Verificando
# MT
state-paused = En pausa
# MT
state-error = Error

# MT
state-pending = En cola
# MT
state-running = En curso
# MT
state-cancelled = Cancelado
# MT
state-succeeded = Completado
# MT
state-failed = Fallido

# MT
action-pause = Pausar
# MT
action-resume = Reanudar
# MT
action-cancel = Cancelar
# MT
action-pause-all = Pausar todas las tareas
# MT
action-resume-all = Reanudar todas las tareas
# MT
action-cancel-all = Cancelar todas las tareas
# MT
action-close = Cerrar
# MT
action-reveal = Mostrar en la carpeta

# MT
menu-pause = Pausar
# MT
menu-resume = Reanudar
# MT
menu-cancel = Cancelar
# MT
menu-remove = Quitar de la cola
# MT
menu-reveal-source = Mostrar origen en la carpeta
# MT
menu-reveal-destination = Mostrar destino en la carpeta

# MT
header-eta-label = Tiempo restante estimado
# MT
header-toolbar-label = Controles globales

# MT
footer-queued = tareas activas
# MT
footer-total-bytes = en curso
# MT
footer-errors = errores
# MT
footer-history = Historial

# MT
empty-title = Suelte archivos o carpetas para copiar
# MT
empty-hint = Arrastre elementos a la ventana. Le pediremos un destino y luego crearemos una tarea por origen.
# MT
empty-region-label = Lista de tareas

# MT
details-drawer-label = Detalles de la tarea
# MT
details-source = Origen
# MT
details-destination = Destino
# MT
details-state = Estado
# MT
details-bytes = Bytes
# MT
details-files = Archivos
# MT
details-speed = Velocidad
# MT
details-eta = Tiempo restante
# MT
details-error = Error

# MT
drop-dialog-title = Transferir elementos soltados
# MT
drop-dialog-subtitle = { $count } elemento(s) listos para transferir. Elija una carpeta de destino para comenzar.
# MT
drop-dialog-mode = Operación
# MT
drop-dialog-copy = Copiar
# MT
drop-dialog-move = Mover
# MT
drop-dialog-pick-destination = Elegir destino
# MT
drop-dialog-change-destination = Cambiar destino
# MT
drop-dialog-start-copy = Iniciar copia
# MT
drop-dialog-start-move = Iniciar movimiento

# MT
eta-calculating = calculando…
# MT
eta-unknown = desconocido

# MT
toast-job-done = Transferencia completada
# MT
toast-copy-queued = Copia en cola
# MT
toast-move-queued = Movimiento en cola
# MT
toast-error-resolved = Error resuelto
# MT
toast-collision-resolved = Conflicto resuelto
# MT
toast-elevated-unavailable = Reintento con permisos elevados llegará en la fase 17 — aún no disponible
toast-clipboard-files-detected = Archivos en el portapapeles — pulsa tu atajo de pegado para copiar con Copy That
toast-clipboard-no-files = El portapapeles no tiene archivos para pegar
# MT
toast-error-log-exported = Registro de errores exportado

# MT — Error modal
error-modal-title = Una transferencia falló
# MT
error-modal-retry = Reintentar
# MT
error-modal-retry-elevated = Reintentar con permisos elevados
# MT
error-modal-skip = Omitir
# MT
error-modal-skip-all-kind = Omitir todos los errores de este tipo
# MT
error-modal-abort = Cancelar todo
# MT
error-modal-path-label = Ruta
# MT
error-modal-code-label = Código
error-drawer-pending-count = Más errores en espera
error-drawer-toggle = Contraer o expandir

# MT — Error-kind labels
err-not-found = Archivo no encontrado
# MT
err-permission-denied = Permiso denegado
# MT
err-disk-full = El disco de destino está lleno
# MT
err-interrupted = Operación interrumpida
# MT
err-verify-failed = Verificación posterior a la copia fallida
# MT
err-path-escape = Ruta rechazada — contiene segmentos de directorio padre (..) o bytes no válidos
# MT
err-io-other = Error de E/S desconocido
err-sparseness-mismatch = No se pudo preservar el diseño disperso en el destino  # MT

# MT — Collision modal
collision-modal-title = El archivo ya existe
# MT
collision-modal-overwrite = Sobrescribir
# MT
collision-modal-overwrite-if-newer = Sobrescribir si es más reciente
# MT
collision-modal-skip = Omitir
# MT
collision-modal-keep-both = Conservar ambos
# MT
collision-modal-rename = Renombrar…
# MT
collision-modal-apply-to-all = Aplicar a todos
# MT
collision-modal-source = Origen
# MT
collision-modal-destination = Destino
# MT
collision-modal-size = Tamaño
# MT
collision-modal-modified = Modificado
# MT
collision-modal-hash-check = Hash rápido (SHA-256)
# MT
collision-modal-rename-placeholder = Nuevo nombre de archivo
# MT
collision-modal-confirm-rename = Renombrar

# MT — Error log drawer
error-log-title = Registro de errores
# MT
error-log-empty = No hay errores registrados
# MT
error-log-export-csv = Exportar CSV
# MT
error-log-export-txt = Exportar texto
# MT
error-log-clear = Borrar registro
# MT
error-log-col-time = Hora
# MT
error-log-col-job = Tarea
# MT
error-log-col-path = Ruta
# MT
error-log-col-code = Código
# MT
error-log-col-message = Mensaje
# MT
error-log-col-resolution = Resolución

# MT — History drawer (Phase 9)
history-title = Historial
# MT
history-empty = Aún no hay tareas registradas
# MT
history-unavailable = El historial de copias no está disponible. La aplicación no pudo abrir el almacén SQLite al iniciar.
# MT
history-filter-any = cualquiera
# MT
history-filter-kind = Tipo
# MT
history-filter-status = Estado
# MT
history-filter-text = Buscar
# MT
history-refresh = Actualizar
# MT
history-export-csv = Exportar CSV
# MT
history-purge-30 = Eliminar > 30 días
# MT
history-rerun = Repetir
# MT
history-detail-open = Detalles
# MT
history-detail-title = Detalles de la tarea
# MT
history-detail-empty = No hay elementos registrados
# MT
history-col-date = Fecha
# MT
history-col-kind = Tipo
# MT
history-col-src = Origen
# MT
history-col-dst = Destino
# MT
history-col-files = Archivos
# MT
history-col-size = Tamaño
# MT
history-col-status = Estado
# MT
history-col-duration = Duración
# MT
history-col-error = Error

# MT
toast-history-exported = Historial exportado
# MT
toast-history-rerun-queued = Repetición en cola

# MT — Totals drawer (Phase 10)
footer-totals = Totales
# MT
totals-title = Totales
# MT
totals-loading = Cargando totales…
# MT
totals-card-bytes = Bytes totales copiados
# MT
totals-card-files = Archivos
# MT
totals-card-jobs = Tareas
# MT
totals-card-avg-rate = Rendimiento promedio
# MT
totals-errors = errores
# MT
totals-spark-title = Últimos 30 días
# MT
totals-kinds-title = Por tipo
# MT
totals-saved-title = Tiempo ahorrado (estimado)
# MT
totals-saved-note = Estimado en comparación con una copia de referencia del mismo trabajo con un administrador de archivos estándar.
# MT
totals-reset = Restablecer estadísticas
# MT
totals-reset-confirm = Esto eliminará todas las tareas y elementos almacenados. ¿Continuar?
# MT
totals-reset-confirm-yes = Sí, restablecer
# MT
toast-totals-reset = Estadísticas restablecidas

# MT — Phase 11a additions
header-language-label = Idioma
# MT
header-language-title = Cambiar idioma

# MT
kind-copy = Copiar
# MT
kind-move = Mover
# MT
kind-delete = Eliminar
# MT
kind-secure-delete = Borrado seguro

# MT
status-running = En curso
# MT
status-succeeded = Completado
# MT
status-failed = Fallido
# MT
status-cancelled = Cancelado
# MT
status-ok = OK
# MT
status-skipped = Omitido

# MT
history-search-placeholder = /ruta
# MT
toast-history-purged = Se eliminaron { $count } tareas de más de 30 días

# MT
err-source-required = Se requiere al menos una ruta de origen.
# MT
err-destination-empty = La ruta de destino está vacía.
# MT
err-source-empty = La ruta de origen está vacía.

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
settings-title = Configuración
# MT
settings-tab-general = General
# MT
settings-tab-appearance = Apariencia
# MT
settings-section-language = Idioma
# MT
settings-phase-12-hint = Más opciones (tema, predeterminados de transferencia, algoritmo de verificación, perfiles) llegan en la fase 12.

# MT — Phase 12 Settings window
settings-loading = Cargando configuración…
# MT
settings-tab-transfer = Transferencia
# MT
settings-tab-shell = Shell
# MT
settings-tab-secure-delete = Borrado seguro
# MT
settings-tab-advanced = Avanzado
# MT
settings-tab-profiles = Perfiles

# MT
settings-section-theme = Tema
# MT
settings-theme-auto = Automático
# MT
settings-theme-light = Claro
# MT
settings-theme-dark = Oscuro
# MT
settings-start-with-os = Iniciar con el sistema
# MT
settings-single-instance = Instancia única en ejecución
# MT
settings-minimize-to-tray = Minimizar a bandeja al cerrar
settings-error-display-mode = Estilo de aviso de error
settings-error-display-modal = Modal (bloquea la app)
settings-error-display-drawer = Panel lateral (no bloqueante)
settings-error-display-mode-hint = El modal detiene la cola hasta que decidas. El panel lateral mantiene la cola en marcha y permite gestionar los errores en la esquina.
settings-paste-shortcut = Pegar archivos con atajo global
settings-paste-shortcut-combo = Combinación del atajo
settings-paste-shortcut-hint = Pulsa esta combinación en cualquier parte del sistema para pegar archivos copiados desde el Explorador / Finder / Archivos con Copy That. CmdOrCtrl se resuelve como Cmd en macOS y Ctrl en Windows / Linux.
settings-clipboard-watcher = Vigilar el portapapeles para archivos copiados
settings-clipboard-watcher-hint = Muestra un aviso cuando aparecen URLs de archivos en el portapapeles, indicando que puedes pegar con Copy That. Sondea cada 500 ms mientras está activo.

# MT
settings-buffer-size = Tamaño del búfer
# MT
settings-verify = Verificar tras la copia
# MT
settings-verify-off = Desactivado
# MT
settings-concurrency = Concurrencia
# MT
settings-concurrency-auto = Automática
# MT
settings-reflink = Reflink / rutas rápidas
# MT
settings-reflink-prefer = Preferir
# MT
settings-reflink-avoid = Evitar reflink
# MT
settings-reflink-disabled = Usar siempre el motor asíncrono
# MT
settings-fsync-on-close = Sincronizar al disco al cerrar (más lento, más seguro)
# MT
settings-preserve-timestamps = Conservar marcas de tiempo
# MT
settings-preserve-permissions = Conservar permisos
# MT
settings-preserve-acls = Conservar ACL (Fase 14)
settings-preserve-sparseness = Preservar archivos dispersos  # MT
settings-preserve-sparseness-hint = Copie solo las extensiones asignadas de los archivos dispersos (discos de VM, archivos de base de datos) para que el tamaño en disco del destino sea igual al origen.  # MT

# MT
settings-context-menu = Activar entradas del menú contextual
# MT
settings-intercept-copy = Interceptar el gestor de copia predeterminado (Windows)
# MT
settings-intercept-copy-hint = Cuando está activo, Ctrl+C / Ctrl+V en el Explorador pasa por Copy That. Registro en la fase 14.
# MT
settings-notify-completion = Notificar al finalizar la tarea

# MT
settings-shred-method = Método de triturado predeterminado
# MT
settings-shred-zero = Cero (1 pasada)
# MT
settings-shred-random = Aleatorio (1 pasada)
# MT
settings-shred-dod3 = DoD 5220.22-M (3 pasadas)
# MT
settings-shred-dod7 = DoD 5220.22-M (7 pasadas)
# MT
settings-shred-gutmann = Gutmann (35 pasadas)
# MT
settings-shred-nist = NIST 800-88
# MT
settings-shred-confirm-twice = Requerir doble confirmación antes de triturar

# MT
settings-log-level = Nivel de registro
# MT
settings-log-off = Desactivado
# MT
settings-telemetry = Telemetría
# MT
settings-telemetry-never = Nunca — sin envío de datos en ningún nivel
# MT
settings-error-policy = Política de errores predeterminada
# MT
settings-error-policy-ask = Preguntar
# MT
settings-error-policy-skip = Omitir
# MT
settings-error-policy-retry = Reintentar con espera
# MT
settings-error-policy-abort = Cancelar al primer error
# MT
settings-history-retention = Retención del historial (días)
# MT
settings-history-retention-hint = 0 = conservar para siempre. Cualquier otro valor purga tareas antiguas al inicio.
# MT
settings-database-path = Ruta de la base de datos
# MT
settings-database-path-default = (predeterminada — directorio de datos del SO)
# MT
settings-reset-all = Restablecer a valores predeterminados
# MT
settings-reset-confirm = ¿Restablecer todas las preferencias? Los perfiles no se modifican.

# MT
settings-profiles-hint = Guarde la configuración actual con un nombre; cárguela después para alternar sin tocar cada opción.
# MT
settings-profile-name-placeholder = Nombre del perfil
# MT
settings-profile-save = Guardar
# MT
settings-profile-import = Importar…
# MT
settings-profile-load = Cargar
# MT
settings-profile-export = Exportar…
# MT
settings-profile-delete = Eliminar
# MT
settings-profile-empty = Aún no hay perfiles guardados.
# MT
settings-profile-import-prompt = Nombre para el perfil importado:

# MT
toast-settings-reset = Configuración restablecida
# MT
toast-profile-saved = Perfil guardado
# MT
toast-profile-loaded = Perfil cargado
# MT
toast-profile-exported = Perfil exportado
# MT
toast-profile-imported = Perfil importado

# Phase 13d — activity feed + header picker buttons
action-add-files = Añadir archivos
action-add-folders = Añadir carpetas
activity-title = Actividad
activity-clear = Borrar lista de actividad
activity-empty = Aún no hay actividad.
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
preflight-unknown-body = El origen es demasiado grande para medirlo rápidamente o el volumen de destino no respondió. Puedes continuar; el protector del motor detendrá la copia limpiamente si se queda sin espacio.
preflight-required = Necesario
preflight-free = Libre
preflight-reserve = Reserva
preflight-shortfall = Déficit
preflight-continue = Continuar igualmente
collision-modal-overwrite-older = Sobrescribir solo los más antiguos

# Phase 14e — subset picker
preflight-pick-subset = Elegir qué copiar…
subset-title = Elige qué fuentes copiar
subset-subtitle = La selección completa no cabe en el destino. Marca lo que quieras copiar; el resto se queda.
subset-loading = Midiendo tamaños…
subset-too-large = demasiado grande para contar
subset-budget = Disponible
subset-remaining = Restante
subset-confirm = Copiar selección
history-rerun-hint = Volver a ejecutar esta copia — vuelve a escanear todos los archivos del origen
history-clear-all = Borrar todo
history-clear-all-confirm = Haz clic de nuevo para confirmar
history-clear-all-hint = Elimina todas las filas del historial. Requiere un segundo clic para confirmar.
toast-history-cleared = Historial borrado ({ $count } filas eliminadas)

# Phase 15 — source-list ordering
drop-dialog-sort-label = Orden:
sort-custom = Personalizado
sort-name-asc = Nombre A → Z (archivos primero)
sort-name-desc = Nombre Z → A (archivos primero)
sort-size-asc = Tamaño ascendente (archivos primero)
sort-size-desc = Tamaño descendente (archivos primero)
sort-reorder = Reordenar
sort-move-top = Mover al principio
sort-move-up = Subir
sort-move-down = Bajar
sort-move-bottom = Mover al final
sort-name-asc-simple = Nombre A → Z
sort-name-desc-simple = Nombre Z → A
sort-size-asc-simple = Más pequeños primero
sort-size-desc-simple = Más grandes primero
activity-sort-locked = La ordenación está desactivada mientras se copia. Pausa o espera a que termine, luego cambia el orden.
drop-dialog-collision-label = Si el archivo ya existe:
collision-policy-keep-both = Conservar ambos (renombrar la nueva copia a _2, _3 …)
collision-policy-skip = Omitir la nueva copia
collision-policy-overwrite = Sobrescribir el archivo existente
collision-policy-overwrite-if-newer = Sobrescribir solo si es más nuevo
collision-policy-prompt = Preguntar cada vez
drop-dialog-busy-checking = Comprobando espacio libre…
drop-dialog-busy-enumerating = Contando archivos…
drop-dialog-busy-starting = Iniciando copia…
toast-enumeration-deferred = El árbol de origen es grande — se omite la lista previa; las filas aparecerán a medida que el motor las procese.

# Phase 14a — enumeration-time filters
# MT
settings-tab-filters = Filtros
# MT
settings-filters-hint = Omite archivos al enumerar para que el motor ni siquiera los abra. "Incluir" se aplica solo a archivos; "Excluir" también poda los directorios coincidentes.
# MT
settings-filters-enabled = Activar filtros en copias de árbol
# MT
settings-filters-include-globs = Globs de inclusión
# MT
settings-filters-include-globs-placeholder = **/*.txt
# MT
settings-filters-include-globs-hint = Un glob por línea. Si hay alguno, el archivo debe coincidir con al menos uno. Los directorios siempre se recorren.
# MT
settings-filters-exclude-globs = Globs de exclusión
# MT
settings-filters-exclude-globs-placeholder = **/node_modules
# MT
settings-filters-exclude-globs-hint = Un glob por línea. Las coincidencias podan todo el subárbol en directorios; los archivos coincidentes se omiten.
# MT
settings-filters-size-range = Rango de tamaño de archivo
# MT
settings-filters-min-size-bytes = Tamaño mínimo (bytes, vacío = sin mínimo)
# MT
settings-filters-max-size-bytes = Tamaño máximo (bytes, vacío = sin máximo)
# MT
settings-filters-date-range = Rango de fecha de modificación
# MT
settings-filters-min-mtime = Modificado a partir de
# MT
settings-filters-max-mtime = Modificado hasta
# MT
settings-filters-attributes = Atributos
# MT
settings-filters-skip-hidden = Omitir archivos / carpetas ocultos
# MT
settings-filters-skip-system = Omitir archivos de sistema (solo Windows)
# MT
settings-filters-skip-readonly = Omitir archivos de solo lectura

# Phase 15 — auto-update
# MT
settings-tab-updater = Actualizaciones
# MT
settings-updater-hint = Copy That busca actualizaciones firmadas como máximo una vez al día. Las actualizaciones se instalan al salir de la app.
# MT
settings-updater-auto-check = Buscar actualizaciones al iniciar
# MT
settings-updater-channel = Canal de publicación
# MT
settings-updater-channel-stable = Estable
# MT
settings-updater-channel-beta = Beta (preliminar)
# MT
settings-updater-last-check = Última comprobación
# MT
settings-updater-last-never = Nunca
# MT
settings-updater-check-now = Buscar actualizaciones ahora
# MT
settings-updater-checking = Buscando…
# MT
settings-updater-available = Actualización disponible
# MT
settings-updater-up-to-date = Estás usando la última versión.
# MT
settings-updater-dismiss = Omitir esta versión
# MT
settings-updater-dismissed = Omitida
# MT
toast-update-available = Hay una versión más reciente disponible
# MT
toast-update-up-to-date = Ya tienes la última versión

# Phase 19a — disk-backed file enumeration (TeraCopy-compatible scan DB)
# MT
scan-progress-title = Escaneando…
# MT
scan-progress-stats = { $files } archivos · { $bytes } hasta ahora
# MT
scan-pause-button = Pausar escaneo
# MT
scan-resume-button = Reanudar escaneo
# MT
scan-cancel-button = Cancelar escaneo
# MT
scan-cancel-confirm = ¿Cancelar escaneo y descartar el progreso?
# MT
scan-db-header = Base de datos de escaneo
# MT
scan-db-hint = Base de datos de escaneo en disco para trabajos con millones de archivos.
# MT
advanced-scan-hash-during = Calcular sumas de verificación durante el escaneo
# MT
advanced-scan-db-path = Ubicación de la base de datos de escaneo
# MT
advanced-scan-retention-days = Borrar escaneos completados automáticamente tras (días)
# MT
advanced-scan-max-keep = Máximo de bases de datos de escaneo a conservar

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
sparse-not-supported-title = El destino rellena los archivos dispersos  # MT
sparse-not-supported-body = { $dst_fs } no admite archivos dispersos. Los huecos del origen se escribieron como ceros, por lo que el destino ocupa más espacio en disco.  # MT
sparse-warning-densified = Diseño disperso preservado: solo se copiaron las extensiones asignadas.  # MT
sparse-warning-mismatch = Desajuste de diseño disperso — el destino puede ser mayor de lo esperado.  # MT

# Phase 24 — security-metadata preservation. MT-flagged drafts;
# the authoritative English source lives in locales/en/copythat.ftl.
settings-preserve-security-metadata = Preservar metadatos de seguridad  # MT
settings-preserve-security-metadata-hint = Capture y vuelva a aplicar flujos de metadatos fuera de banda (NTFS ADS / xattrs / ACL POSIX / contextos SELinux / capacidades de archivo Linux / bifurcaciones de recursos macOS) en cada copia.  # MT
settings-preserve-motw = Preservar Marca de la Web (indicador de descarga de internet)  # MT
settings-preserve-motw-hint = Crítico para la seguridad. SmartScreen y Office Protected View usan este flujo para advertir sobre archivos descargados de internet. Desactivarlo permite que un ejecutable descargado pierda su marca de origen al copiar y omita las protecciones del sistema operativo.  # MT
settings-preserve-posix-acls = Preservar ACL POSIX y atributos extendidos  # MT
settings-preserve-posix-acls-hint = Traslade los xattrs user.* / system.* / trusted.* y las listas de control de acceso POSIX en la copia.  # MT
settings-preserve-selinux = Preservar contextos SELinux  # MT
settings-preserve-selinux-hint = Traslade la etiqueta security.selinux en la copia para que los demonios bajo políticas MAC puedan seguir accediendo al archivo.  # MT
settings-preserve-resource-forks = Preservar bifurcaciones de recursos y Finder info de macOS  # MT
settings-preserve-resource-forks-hint = Traslade la bifurcación de recursos heredada y FinderInfo (etiquetas de color, metadatos Carbon) en la copia.  # MT
settings-appledouble-fallback = Usar archivo lateral AppleDouble en sistemas de archivos incompatibles  # MT
meta-translated-to-appledouble = Metadatos foráneos guardados en archivo lateral AppleDouble (._{ $ext })  # MT

# Phase 25 — two-way sync with vector-clock conflict detection.
# MT-flagged drafts; the authoritative English source lives in
# locales/en/copythat.ftl.
footer-sync = Sinc  # MT
sync-drawer-title = Sincronización bidireccional  # MT
sync-drawer-hint = Mantenga dos carpetas sincronizadas sin sobrescrituras silenciosas. Las ediciones concurrentes aparecen como conflictos resolubles.  # MT
sync-add-pair = Añadir par  # MT
sync-add-cancel = Cancelar  # MT
sync-refresh = Actualizar  # MT
sync-add-save = Guardar par  # MT
sync-add-saving = Guardando…  # MT
sync-add-missing-fields = La etiqueta, la ruta izquierda y la ruta derecha son todas obligatorias.  # MT
sync-remove-confirm = ¿Eliminar este par de sincronización? Se conserva la base de datos de estado; las carpetas permanecen intactas.  # MT
sync-field-label = Etiqueta  # MT
sync-field-label-placeholder = p. ej. Documentos ↔ NAS  # MT
sync-field-left = Carpeta izquierda  # MT
sync-field-left-placeholder = Elija o pegue una ruta absoluta  # MT
sync-field-right = Carpeta derecha  # MT
sync-field-right-placeholder = Elija o pegue una ruta absoluta  # MT
sync-field-mode = Modo  # MT
sync-mode-two-way = Bidireccional  # MT
sync-mode-mirror-left-to-right = Espejo (izquierda → derecha)  # MT
sync-mode-mirror-right-to-left = Espejo (derecha → izquierda)  # MT
sync-mode-contribute-left-to-right = Contribuir (izquierda → derecha, sin borrados)  # MT
sync-no-pairs = Aún no hay pares de sincronización configurados. Haga clic en "Añadir par" para empezar.  # MT
sync-loading = Cargando pares configurados…  # MT
sync-never-run = Nunca ejecutado  # MT
sync-running = Ejecutando  # MT
sync-run-now = Ejecutar ahora  # MT
sync-cancel = Cancelar  # MT
sync-remove-pair = Eliminar  # MT
sync-view-conflicts = Ver conflictos ({ $count })  # MT
sync-conflicts-heading = Conflictos  # MT
sync-no-conflicts = Sin conflictos de la última ejecución.  # MT
sync-winner = Ganador  # MT
sync-side-left-to-right = izquierda  # MT
sync-side-right-to-left = derecha  # MT
sync-conflict-kind-concurrent-write = Edición concurrente  # MT
sync-conflict-kind-delete-edit = Eliminar ↔ editar  # MT
sync-conflict-kind-add-add = Ambos lados añadieron  # MT
sync-conflict-kind-corrupt-equal = El contenido divergió sin una nueva escritura  # MT
sync-resolve-keep-left = Conservar izquierda  # MT
sync-resolve-keep-right = Conservar derecha  # MT
sync-resolve-keep-both = Conservar ambos  # MT
sync-resolve-three-way = Resolver mediante fusión de 3 vías  # MT
sync-resolve-phase-53-tooltip = La fusión interactiva de 3 vías para archivos no textuales llega en la Fase 53.  # MT
sync-error-prefix = Error de sincronización  # MT

# Phase 26 — real-time mirror watcher. MT-flagged drafts;
# the authoritative English source lives in locales/en/copythat.ftl.
live-mirror-start = Iniciar espejo en vivo  # MT
live-mirror-stop = Detener espejo en vivo  # MT
live-mirror-watching = Vigilando  # MT
live-mirror-toggle-hint = Resincroniza automáticamente con cada cambio del sistema de archivos detectado. Un hilo en segundo plano por par activo.  # MT
watch-event-prefix = Cambio de archivo  # MT
watch-overflow-recovered = Búfer del observador desbordado; reenumerando para recuperar  # MT

# Phase 27 — content-defined chunk store. MT-flagged drafts;
# the authoritative English source lives in locales/en/copythat.ftl.
chunk-store-section = Almacén de fragmentos  # MT
chunk-store-enable = Activar almacén de fragmentos (reanudación delta y deduplicación)  # MT
chunk-store-enable-hint = Divide cada archivo copiado por contenido (FastCDC) y almacena fragmentos direccionados por contenido. Los reintentos solo reescriben los fragmentos modificados; los archivos con contenido compartido se deduplican automáticamente.  # MT
chunk-store-location = Ubicación del almacén de fragmentos  # MT
chunk-store-max-size = Tamaño máximo del almacén de fragmentos  # MT
chunk-store-prune = Eliminar fragmentos más antiguos que (días)  # MT
chunk-store-savings = Ahorrados { $gib } GiB mediante deduplicación de fragmentos  # MT
chunk-store-disk-usage = Usando { $size } en { $chunks } fragmentos  # MT

# Phase 28 — tray-resident Drop Stack. MT-flagged drafts;
# the authoritative English source lives in locales/en/copythat.ftl.
dropstack-window-title = Pila de arrastre  # MT
dropstack-tray-open = Pila de arrastre  # MT
dropstack-empty-title = La pila de arrastre está vacía  # MT
dropstack-empty-hint = Arrastra archivos aquí desde el Explorador o haz clic derecho en una fila de trabajo para añadirla.  # MT
dropstack-add-to-stack = Añadir a la pila de arrastre  # MT
dropstack-copy-all-to = Copiar todo a…  # MT
dropstack-move-all-to = Mover todo a…  # MT
dropstack-clear = Vaciar pila  # MT
dropstack-remove-row = Quitar de la pila  # MT
dropstack-path-missing-toast = Se quitó { $path } — el archivo ya no existe.  # MT
dropstack-always-on-top = Mantener la pila de arrastre siempre en primer plano  # MT
dropstack-show-tray-icon = Mostrar el icono de Copy That en la bandeja  # MT
dropstack-open-on-start = Abrir la pila de arrastre al iniciar la aplicación  # MT
dropstack-count = { $count } ruta  # MT
