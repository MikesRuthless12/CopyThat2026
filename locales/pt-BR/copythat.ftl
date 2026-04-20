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
# MT — Phase 8 toast messages
toast-error-resolved = Erro resolvido
# MT
toast-collision-resolved = Conflito resolvido
# MT
toast-elevated-unavailable = A nova tentativa com permissões elevadas chega na fase 17 — ainda não disponível
# MT
toast-error-log-exported = Registro de erros exportado

# MT — Error modal
error-modal-title = Uma transferência falhou
# MT
error-modal-retry = Tentar novamente
# MT
error-modal-retry-elevated = Tentar novamente com permissões elevadas
# MT
error-modal-skip = Ignorar
# MT
error-modal-skip-all-kind = Ignorar todos os erros desse tipo
# MT
error-modal-abort = Cancelar tudo
# MT
error-modal-path-label = Caminho
# MT
error-modal-code-label = Código

# MT — Error-kind labels
err-not-found = Arquivo não encontrado
# MT
err-permission-denied = Permissão negada
# MT
err-disk-full = O disco de destino está cheio
# MT
err-interrupted = Operação interrompida
# MT
err-verify-failed = Verificação pós-cópia falhou
# MT
err-io-other = Erro de E/S desconhecido

# MT — Collision modal
collision-modal-title = O arquivo já existe
# MT
collision-modal-overwrite = Sobrescrever
# MT
collision-modal-overwrite-if-newer = Sobrescrever se mais recente
# MT
collision-modal-skip = Ignorar
# MT
collision-modal-keep-both = Manter ambos
# MT
collision-modal-rename = Renomear…
# MT
collision-modal-apply-to-all = Aplicar a todos
# MT
collision-modal-source = Origem
# MT
collision-modal-destination = Destino
# MT
collision-modal-size = Tamanho
# MT
collision-modal-modified = Modificado
# MT
collision-modal-hash-check = Hash rápido (SHA-256)
# MT
collision-modal-rename-placeholder = Novo nome do arquivo
# MT
collision-modal-confirm-rename = Renomear

# MT — Error log drawer
error-log-title = Registro de erros
# MT
error-log-empty = Nenhum erro registrado
# MT
error-log-export-csv = Exportar CSV
# MT
error-log-export-txt = Exportar texto
# MT
error-log-clear = Limpar registro
# MT
error-log-col-time = Hora
# MT
error-log-col-job = Tarefa
# MT
error-log-col-path = Caminho
# MT
error-log-col-code = Código
# MT
error-log-col-message = Mensagem
# MT
error-log-col-resolution = Resolução

# MT — History drawer (Phase 9)
history-title = Histórico
# MT
history-empty = Nenhuma tarefa registrada ainda
# MT
history-unavailable = O histórico de cópias não está disponível. O aplicativo não conseguiu abrir o armazenamento SQLite na inicialização.
# MT
history-filter-any = qualquer
# MT
history-filter-kind = Tipo
# MT
history-filter-status = Estado
# MT
history-filter-text = Pesquisar
# MT
history-refresh = Atualizar
# MT
history-export-csv = Exportar CSV
# MT
history-purge-30 = Eliminar > 30 dias
# MT
history-rerun = Executar novamente
# MT
history-detail-open = Detalhes
# MT
history-detail-title = Detalhes da tarefa
# MT
history-detail-empty = Nenhum item registrado
# MT
history-col-date = Data
# MT
history-col-kind = Tipo
# MT
history-col-src = Origem
# MT
history-col-dst = Destino
# MT
history-col-files = Arquivos
# MT
history-col-size = Tamanho
# MT
history-col-status = Estado
# MT
history-col-duration = Duração
# MT
history-col-error = Erro

# MT
toast-history-exported = Histórico exportado
# MT
toast-history-rerun-queued = Nova execução na fila

# MT — Totals drawer (Phase 10)
footer-totals = Totais
# MT
totals-title = Totais
# MT
totals-loading = Carregando totais…
# MT
totals-card-bytes = Total de bytes copiados
# MT
totals-card-files = Arquivos
# MT
totals-card-jobs = Tarefas
# MT
totals-card-avg-rate = Taxa média
# MT
totals-errors = erros
# MT
totals-spark-title = Últimos 30 dias
# MT
totals-kinds-title = Por tipo
# MT
totals-saved-title = Tempo economizado (estimado)
# MT
totals-saved-note = Estimado em comparação com uma cópia de referência da mesma carga em um gerenciador de arquivos padrão.
# MT
totals-reset = Redefinir estatísticas
# MT
totals-reset-confirm = Isso exclui todas as tarefas e itens armazenados. Continuar?
# MT
totals-reset-confirm-yes = Sim, redefinir
# MT
toast-totals-reset = Estatísticas redefinidas

# MT — Phase 11a additions
header-language-label = Idioma
# MT
header-language-title = Alterar idioma

# MT
kind-copy = Copiar
# MT
kind-move = Mover
# MT
kind-delete = Excluir
# MT
kind-secure-delete = Exclusão segura

# MT
status-running = Em execução
# MT
status-succeeded = Concluído
# MT
status-failed = Falhou
# MT
status-cancelled = Cancelado
# MT
status-ok = OK
# MT
status-skipped = Ignorado

# MT
history-search-placeholder = /caminho
# MT
toast-history-purged = { $count } tarefas com mais de 30 dias removidas

# MT
err-source-required = É necessário pelo menos um caminho de origem.
# MT
err-destination-empty = O caminho de destino está vazio.
# MT
err-source-empty = O caminho de origem está vazio.

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
