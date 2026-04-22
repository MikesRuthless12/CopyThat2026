app-name = Copy That 2026
# MT
window-title = Copy That 2026
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
