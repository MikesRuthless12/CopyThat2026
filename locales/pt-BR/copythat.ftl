app-name = Copy That v1.0.0
# MT
window-title = Copy That v1.0.0
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
toast-clipboard-files-detected = Arquivos na área de transferência — pressione seu atalho de colar para copiar via Copy That
toast-clipboard-no-files = A área de transferência não tem arquivos para colar
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
error-drawer-pending-count = Mais erros aguardando
error-drawer-toggle = Recolher ou expandir

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
err-path-escape = Caminho rejeitado — contém segmentos de diretório pai (..) ou bytes ilegais
# MT
err-io-other = Erro de E/S desconhecido
err-sparseness-mismatch = Não foi possível preservar layout esparso no destino  # MT

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

# MT — Phase 11b Settings modal
settings-title = Configurações
# MT
settings-tab-general = Geral
# MT
settings-tab-appearance = Aparência
# MT
settings-section-language = Idioma
# MT
settings-phase-12-hint = Mais configurações (tema, padrões de transferência, algoritmo de verificação, perfis) chegarão na fase 12.

# MT — Phase 12 Settings window
settings-loading = Carregando configurações…
# MT
settings-tab-transfer = Transferência
# MT
settings-tab-shell = Shell
# MT
settings-tab-secure-delete = Exclusão segura
# MT
settings-tab-advanced = Avançado
# MT
settings-tab-profiles = Perfis

# MT
settings-section-theme = Tema
# MT
settings-theme-auto = Automático
# MT
settings-theme-light = Claro
# MT
settings-theme-dark = Escuro
# MT
settings-start-with-os = Iniciar com o sistema
# MT
settings-single-instance = Instância única em execução
# MT
settings-minimize-to-tray = Minimizar para a bandeja ao fechar
settings-error-display-mode = Estilo de aviso de erro
settings-error-display-modal = Modal (bloqueia o app)
settings-error-display-drawer = Painel lateral (não bloqueante)
settings-error-display-mode-hint = O modal interrompe a fila até você decidir. O painel lateral mantém a fila em andamento e permite lidar com os erros no canto.
settings-paste-shortcut = Colar arquivos via atalho global
settings-paste-shortcut-combo = Combinação de teclas
settings-paste-shortcut-hint = Pressione esta combinação em qualquer lugar do sistema para colar arquivos copiados do Explorer / Finder / Arquivos via Copy That. CmdOrCtrl resolve para Cmd no macOS e Ctrl no Windows / Linux.
settings-clipboard-watcher = Monitorar a área de transferência para arquivos copiados
settings-clipboard-watcher-hint = Mostra um aviso quando URLs de arquivo aparecem na área de transferência, sugerindo colar via Copy That. Consulta a cada 500 ms quando ativo.

# MT
settings-buffer-size = Tamanho do buffer
# MT
settings-verify = Verificar após copiar
# MT
settings-verify-off = Desativado
# MT
settings-concurrency = Concorrência
# MT
settings-concurrency-auto = Automática
# MT
settings-reflink = Reflink / caminhos rápidos
# MT
settings-reflink-prefer = Preferir
# MT
settings-reflink-avoid = Evitar reflink
# MT
settings-reflink-disabled = Sempre usar o motor assíncrono
# MT
settings-fsync-on-close = Sincronizar com o disco ao fechar (mais lento, mais seguro)
# MT
settings-preserve-timestamps = Preservar carimbos de data/hora
# MT
settings-preserve-permissions = Preservar permissões
# MT
settings-preserve-acls = Preservar ACLs (Fase 14)
settings-preserve-sparseness = Preservar arquivos esparsos  # MT
settings-preserve-sparseness-hint = Copie apenas as extensões alocadas de arquivos esparsos (discos de VM, arquivos de banco de dados) para que o tamanho em disco do destino permaneça igual ao da origem.  # MT

# MT
settings-context-menu = Habilitar entradas do menu de contexto
# MT
settings-intercept-copy = Interceptar gerenciador de cópia padrão (Windows)
# MT
settings-intercept-copy-hint = Quando ativo, Ctrl+C / Ctrl+V no Explorer passa pelo Copy That. Registro na Fase 14.
# MT
settings-notify-completion = Notificar ao concluir tarefa

# MT
settings-shred-method = Método de destruição padrão
# MT
settings-shred-zero = Zero (1 passagem)
# MT
settings-shred-random = Aleatório (1 passagem)
# MT
settings-shred-dod3 = DoD 5220.22-M (3 passagens)
# MT
settings-shred-dod7 = DoD 5220.22-M (7 passagens)
# MT
settings-shred-gutmann = Gutmann (35 passagens)
# MT
settings-shred-nist = NIST 800-88
# MT
settings-shred-confirm-twice = Exigir dupla confirmação antes da destruição

# MT
settings-log-level = Nível de log
# MT
settings-log-off = Desativado
# MT
settings-telemetry = Telemetria
# MT
settings-telemetry-never = Nunca — sem envio de dados em nenhum nível
# MT
settings-error-policy = Política de erro padrão
# MT
settings-error-policy-ask = Perguntar
# MT
settings-error-policy-skip = Ignorar
# MT
settings-error-policy-retry = Repetir com espera
# MT
settings-error-policy-abort = Cancelar ao primeiro erro
# MT
settings-history-retention = Retenção do histórico (dias)
# MT
settings-history-retention-hint = 0 = manter para sempre. Qualquer outro valor remove tarefas antigas na inicialização.
# MT
settings-database-path = Caminho do banco de dados
# MT
settings-database-path-default = (padrão — diretório de dados do SO)
# MT
settings-reset-all = Restaurar padrões
# MT
settings-reset-confirm = Redefinir todas as preferências? Os perfis não são afetados.

# MT
settings-profiles-hint = Salve as configurações atuais com um nome; carregue depois para alternar sem mexer nos controles individuais.
# MT
settings-profile-name-placeholder = Nome do perfil
# MT
settings-profile-save = Salvar
# MT
settings-profile-import = Importar…
# MT
settings-profile-load = Carregar
# MT
settings-profile-export = Exportar…
# MT
settings-profile-delete = Excluir
# MT
settings-profile-empty = Nenhum perfil salvo.
# MT
settings-profile-import-prompt = Nome para o perfil importado:

# MT
toast-settings-reset = Configurações redefinidas
# MT
toast-profile-saved = Perfil salvo
# MT
toast-profile-loaded = Perfil carregado
# MT
toast-profile-exported = Perfil exportado
# MT
toast-profile-imported = Perfil importado

# Phase 13d — activity feed + header picker buttons
action-add-files = Adicionar arquivos
action-add-folders = Adicionar pastas
activity-title = Atividade
activity-clear = Limpar lista de atividade
activity-empty = Ainda não há atividade.
activity-after-done = Ao concluir:
activity-keep-open = Manter o app aberto
activity-close-app = Fechar o app
activity-shutdown = Desligar o PC
activity-logoff = Sair da sessão
activity-sleep = Suspender

# Phase 14 — preflight free-space dialog
preflight-block-title = Espaço insuficiente no destino
preflight-warn-title = Pouco espaço no destino
preflight-unknown-title = Não foi possível determinar o espaço livre
preflight-unknown-body = A origem é grande demais para ser medida rapidamente ou o volume de destino não respondeu. Você pode continuar; o limitador do mecanismo interromperá a cópia com segurança se o espaço acabar.
preflight-required = Necessário
preflight-free = Livre
preflight-reserve = Reserva
preflight-shortfall = Déficit
preflight-continue = Continuar mesmo assim
collision-modal-overwrite-older = Sobrescrever só os mais antigos

# Phase 14e — subset picker
preflight-pick-subset = Escolher o que copiar…
subset-title = Escolha quais fontes copiar
subset-subtitle = A seleção completa não cabe no destino. Marque o que deseja copiar; o restante fica para trás.
subset-loading = Medindo tamanhos…
subset-too-large = grande demais para contar
subset-budget = Disponível
subset-remaining = Restante
subset-confirm = Copiar seleção
history-rerun-hint = Executar esta cópia novamente — reexamina cada arquivo na árvore de origem
history-clear-all = Limpar tudo
history-clear-all-confirm = Clique novamente para confirmar
history-clear-all-hint = Exclui todas as linhas do histórico. Requer um segundo clique para confirmar.
toast-history-cleared = Histórico limpo ({ $count } linhas removidas)

# Phase 15 — source-list ordering
drop-dialog-sort-label = Ordem:
sort-custom = Personalizado
sort-name-asc = Nome A → Z (arquivos primeiro)
sort-name-desc = Nome Z → A (arquivos primeiro)
sort-size-asc = Tamanho crescente (arquivos primeiro)
sort-size-desc = Tamanho decrescente (arquivos primeiro)
sort-reorder = Reordenar
sort-move-top = Mover para o topo
sort-move-up = Para cima
sort-move-down = Para baixo
sort-move-bottom = Mover para o fim
sort-name-asc-simple = Nome A → Z
sort-name-desc-simple = Nome Z → A
sort-size-asc-simple = Menores primeiro
sort-size-desc-simple = Maiores primeiro
activity-sort-locked = A ordenação está desativada enquanto uma cópia está em andamento. Pause ou aguarde terminar, depois mude a ordem.
drop-dialog-collision-label = Se um arquivo já existir:
collision-policy-keep-both = Manter os dois (renomear a nova cópia para _2, _3, …)
collision-policy-skip = Ignorar a nova cópia
collision-policy-overwrite = Sobrescrever o arquivo existente
collision-policy-overwrite-if-newer = Sobrescrever apenas se for mais novo
collision-policy-prompt = Perguntar sempre
drop-dialog-busy-checking = Verificando espaço livre…
drop-dialog-busy-enumerating = Contando arquivos…
drop-dialog-busy-starting = Iniciando cópia…
toast-enumeration-deferred = A árvore de origem é grande — lista prévia ignorada; linhas aparecerão conforme o mecanismo processar.

# Phase 14a — enumeration-time filters
# MT
settings-tab-filters = Filtros
# MT
settings-filters-hint = Pula arquivos na enumeração para que o motor nem os abra. Incluir aplica-se a arquivos apenas; Excluir também poda diretórios correspondentes.
# MT
settings-filters-enabled = Ativar filtros em cópias de árvore
# MT
settings-filters-include-globs = Globs de inclusão
# MT
settings-filters-include-globs-placeholder = **/*.txt
# MT
settings-filters-include-globs-hint = Um glob por linha. Quando preenchido, o arquivo deve corresponder a pelo menos um. Diretórios sempre são percorridos.
# MT
settings-filters-exclude-globs = Globs de exclusão
# MT
settings-filters-exclude-globs-placeholder = **/node_modules
# MT
settings-filters-exclude-globs-hint = Um glob por linha. Correspondências podam toda a subárvore para diretórios; arquivos correspondentes são pulados.
# MT
settings-filters-size-range = Intervalo de tamanho de arquivo
# MT
settings-filters-min-size-bytes = Tamanho mínimo (bytes, vazio = sem mínimo)
# MT
settings-filters-max-size-bytes = Tamanho máximo (bytes, vazio = sem máximo)
# MT
settings-filters-date-range = Intervalo da data de modificação
# MT
settings-filters-min-mtime = Modificado a partir de
# MT
settings-filters-max-mtime = Modificado até
# MT
settings-filters-attributes = Atributos
# MT
settings-filters-skip-hidden = Pular arquivos / pastas ocultos
# MT
settings-filters-skip-system = Pular arquivos de sistema (apenas Windows)
# MT
settings-filters-skip-readonly = Pular arquivos somente leitura

# Phase 15 — auto-update
# MT
settings-tab-updater = Atualizações
# MT
settings-updater-hint = O Copy That verifica atualizações assinadas no máximo uma vez por dia. As atualizações são instaladas ao sair do app.
# MT
settings-updater-auto-check = Verificar atualizações ao iniciar
# MT
settings-updater-channel = Canal de lançamento
# MT
settings-updater-channel-stable = Estável
# MT
settings-updater-channel-beta = Beta (pré-lançamento)
# MT
settings-updater-last-check = Última verificação
# MT
settings-updater-last-never = Nunca
# MT
settings-updater-check-now = Verificar atualizações agora
# MT
settings-updater-checking = Verificando…
# MT
settings-updater-available = Atualização disponível
# MT
settings-updater-up-to-date = Você está usando a versão mais recente.
# MT
settings-updater-dismiss = Ignorar esta versão
# MT
settings-updater-dismissed = Ignorada
# MT
toast-update-available = Uma versão mais recente está disponível
# MT
toast-update-up-to-date = Você já está na versão mais recente

# Phase 19a — disk-backed file enumeration (TeraCopy-compatible scan DB)
# MT
scan-progress-title = Escaneando…
# MT
scan-progress-stats = { $files } arquivos · { $bytes } até agora
# MT
scan-pause-button = Pausar escaneamento
# MT
scan-resume-button = Retomar escaneamento
# MT
scan-cancel-button = Cancelar escaneamento
# MT
scan-cancel-confirm = Cancelar escaneamento e descartar progresso?
# MT
scan-db-header = Banco de dados de escaneamento
# MT
scan-db-hint = Banco de dados de escaneamento em disco para trabalhos com milhões de arquivos.
# MT
advanced-scan-hash-during = Calcular somas de verificação durante o escaneamento
# MT
advanced-scan-db-path = Local do banco de dados de escaneamento
# MT
advanced-scan-retention-days = Excluir escaneamentos concluídos automaticamente após (dias)
# MT
advanced-scan-max-keep = Número máximo de bancos de dados de escaneamento a manter

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
sparse-not-supported-title = Destino preenche arquivos esparsos  # MT
sparse-not-supported-body = { $dst_fs } não suporta arquivos esparsos. Buracos na origem foram gravados como zeros, portanto o destino ocupa mais espaço em disco.  # MT
sparse-warning-densified = Layout esparso preservado: apenas as extensões alocadas foram copiadas.  # MT
sparse-warning-mismatch = Incompatibilidade de layout esparso — destino pode ser maior que o esperado.  # MT
