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
action-pause-all = Pausar todos los trabajos
# MT
action-resume-all = Reanudar todos los trabajos
# MT
action-cancel-all = Cancelar todos los trabajos
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
header-eta-label = Tiempo estimado restante
# MT
header-toolbar-label = Controles globales

# MT
footer-queued = trabajos activos
# MT
footer-total-bytes = en curso
# MT
footer-errors = errores
# MT
footer-history = Historial

# MT
empty-title = Suelte archivos o carpetas para copiar
# MT
empty-hint = Arrastre elementos a la ventana. Le preguntaremos el destino y luego agregaremos un trabajo por cada origen.
# MT
empty-region-label = Lista de trabajos

# MT
details-drawer-label = Detalles del trabajo
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
drop-dialog-subtitle = { $count } elementos listos para transferir. Elija una carpeta de destino para empezar.
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
drop-dialog-start-copy = Comenzar a copiar
# MT
drop-dialog-start-move = Comenzar a mover

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
# Phase 8 additions — MT placeholders; review before 1.0.

# MT — Toast messages
toast-error-resolved = Error resolved
toast-collision-resolved = Collision resolved
toast-elevated-unavailable = Elevated retry lands in Phase 17 — not available yet
toast-error-log-exported = Error log exported

# MT — Error modal
error-modal-title = A transfer failed
error-modal-retry = Retry
error-modal-retry-elevated = Retry with elevated permissions
error-modal-skip = Skip
error-modal-skip-all-kind = Skip all errors of this kind
error-modal-abort = Abort all
error-modal-path-label = Path
error-modal-code-label = Code

# MT — Error-kind labels
err-not-found = File not found
err-permission-denied = Permission denied
err-disk-full = Destination disk is full
err-interrupted = Operation interrupted
err-verify-failed = Post-copy verification failed
err-io-other = Unknown I/O error

# MT — Collision modal
collision-modal-title = File already exists
collision-modal-overwrite = Overwrite
collision-modal-overwrite-if-newer = Overwrite if newer
collision-modal-skip = Skip
collision-modal-keep-both = Keep both
collision-modal-rename = Rename…
collision-modal-apply-to-all = Apply to all
collision-modal-source = Source
collision-modal-destination = Destination
collision-modal-size = Size
collision-modal-modified = Modified
collision-modal-hash-check = Quick hash (SHA-256)
collision-modal-rename-placeholder = New filename
collision-modal-confirm-rename = Rename

# MT — Error log drawer
error-log-title = Error log
error-log-empty = No errors logged
error-log-export-csv = Export CSV
error-log-export-txt = Export text
error-log-clear = Clear log
error-log-col-time = Time
error-log-col-job = Job
error-log-col-path = Path
error-log-col-code = Code
error-log-col-message = Message
error-log-col-resolution = Resolution
