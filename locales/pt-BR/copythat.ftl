app-name = Copy That 2026
# MT
window-title = Copy That 2026
# MT
shred-ssd-advisory = Aviso: este destino está em um SSD. Sobrescrever em múltiplas passagens não higieniza de forma confiável a memória flash porque o wear-leveling e o overprovisioning movem os dados para fora do endereço lógico do bloco. Para mídia de estado sólido, prefira ATA SECURE ERASE, NVMe Format com Secure Erase ou criptografia de disco completo com uma chave descartada.

# MT
state-idle = Ocioso
# MT
state-copying = Copiando
# MT
state-verifying = Verificando
# MT
state-paused = Pausado
# MT
state-error = Erro

# MT
state-pending = Na fila
# MT
state-running = Em execução
# MT
state-cancelled = Cancelado
# MT
state-succeeded = Concluído
# MT
state-failed = Falhou

# MT
action-pause = Pausar
# MT
action-resume = Retomar
# MT
action-cancel = Cancelar
# MT
action-pause-all = Pausar todas as tarefas
# MT
action-resume-all = Retomar todas as tarefas
# MT
action-cancel-all = Cancelar todas as tarefas
# MT
action-close = Fechar
# MT
action-reveal = Mostrar na pasta

# MT
menu-pause = Pausar
# MT
menu-resume = Retomar
# MT
menu-cancel = Cancelar
# MT
menu-remove = Remover da fila
# MT
menu-reveal-source = Mostrar origem na pasta
# MT
menu-reveal-destination = Mostrar destino na pasta

# MT
header-eta-label = Tempo restante estimado
# MT
header-toolbar-label = Controles globais

# MT
footer-queued = tarefas ativas
# MT
footer-total-bytes = em andamento
# MT
footer-errors = erros
# MT
footer-history = Histórico

# MT
empty-title = Solte arquivos ou pastas para copiar
# MT
empty-hint = Arraste itens para a janela. Pediremos um destino e depois enfileiraremos uma tarefa por origem.
# MT
empty-region-label = Lista de tarefas

# MT
details-drawer-label = Detalhes da tarefa
# MT
details-source = Origem
# MT
details-destination = Destino
# MT
details-state = Estado
# MT
details-bytes = Bytes
# MT
details-files = Arquivos
# MT
details-speed = Velocidade
# MT
details-eta = Tempo restante
# MT
details-error = Erro

# MT
drop-dialog-title = Transferir itens soltos
# MT
drop-dialog-subtitle = { $count } item(ns) pronto(s) para transferir. Escolha uma pasta de destino para começar.
# MT
drop-dialog-mode = Operação
# MT
drop-dialog-copy = Copiar
# MT
drop-dialog-move = Mover
# MT
drop-dialog-pick-destination = Escolher destino
# MT
drop-dialog-change-destination = Alterar destino
# MT
drop-dialog-start-copy = Iniciar cópia
# MT
drop-dialog-start-move = Iniciar movimentação

# MT
eta-calculating = calculando…
# MT
eta-unknown = desconhecido

# MT
toast-job-done = Transferência concluída
# MT
toast-copy-queued = Cópia enfileirada
# MT
toast-move-queued = Movimentação enfileirada
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

# MT — History drawer (Phase 9)
history-title = History
history-empty = No jobs recorded yet
history-unavailable = Copy history isn't available. The app couldn't open the SQLite store at startup.
history-filter-any = any
history-filter-kind = Kind
history-filter-status = Status
history-filter-text = Search
history-refresh = Refresh
history-export-csv = Export CSV
history-purge-30 = Purge > 30 days
history-rerun = Re-run
history-detail-open = Details
history-detail-title = Job details
history-detail-empty = No items recorded
history-col-date = Date
history-col-kind = Kind
history-col-src = Source
history-col-dst = Destination
history-col-files = Files
history-col-size = Size
history-col-status = Status
history-col-duration = Duration
history-col-error = Error

# MT — Phase 9 toasts
toast-history-exported = History exported
toast-history-rerun-queued = Re-run queued

# MT — Totals drawer (Phase 10)
footer-totals = Totals
totals-title = Totals
totals-loading = Loading totals…
totals-card-bytes = Total bytes copied
totals-card-files = Files
totals-card-jobs = Jobs
totals-card-avg-rate = Average throughput
totals-errors = errors
totals-spark-title = Last 30 days
totals-kinds-title = By kind
totals-saved-title = Time saved (estimated)
totals-saved-note = Estimated vs a baseline file-manager copy of the same workload.
totals-reset = Reset statistics
totals-reset-confirm = This deletes every stored job and item. Continue?
totals-reset-confirm-yes = Yes, reset
toast-totals-reset = Statistics reset
