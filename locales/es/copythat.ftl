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
