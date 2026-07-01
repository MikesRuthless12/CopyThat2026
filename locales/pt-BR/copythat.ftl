app-name = Copy That v0.19.84
window-title = Copy That v0.19.84
shred-ssd-advisory = Aviso: este destino está em um SSD. Sobrescritas em várias passagens não higienizam a memória flash de forma confiável, pois o nivelamento de desgaste e o provisionamento extra movem os dados para fora do endereço de bloco lógico. Para mídia de estado sólido, prefira ATA SECURE ERASE, NVMe Format com Secure Erase ou criptografia de disco inteiro com descarte da chave.

# Global aggregate states (header pill)
state-idle = Ocioso
state-copying = Copiando
state-verifying = Verificando
state-paused = Pausado
state-error = Erro

# Per-job states (row badge)
state-pending = Na fila
state-running = Em execução
state-cancelled = Cancelado
state-succeeded = Concluído
state-failed = Falhou

# Actions
action-pause = Pausar
action-resume = Retomar
action-cancel = Cancelar
action-pause-all = Pausar todas as tarefas
action-resume-all = Retomar todas as tarefas
action-cancel-all = Cancelar todas as tarefas
action-close = Fechar
action-reveal = Mostrar na pasta
action-add-files = Adicionar arquivos
action-add-folders = Adicionar pastas

# Phase 13d — activity feed
activity-title = Atividade
activity-clear = Limpar lista de atividades
activity-empty = Nenhuma atividade de arquivo ainda.
activity-after-done = Ao concluir:
activity-keep-open = Manter o app aberto
activity-close-app = Fechar o app
activity-shutdown = Desligar o PC
activity-logoff = Encerrar sessão
activity-sleep = Suspender

# Phase 14 — preflight free-space dialog
preflight-block-title = Espaço insuficiente no destino
preflight-warn-title = Pouco espaço no destino
preflight-unknown-title = Não foi possível determinar o espaço livre
preflight-unknown-body = A origem é grande demais para medir rapidamente ou o volume de destino não respondeu. Você pode continuar; a proteção de espaço do mecanismo interromperá a cópia de forma limpa caso fique sem espaço.
preflight-required = Necessário
preflight-free = Livre
preflight-reserve = Reserva
preflight-shortfall = Déficit
preflight-continue = Continuar mesmo assim
preflight-pick-subset = Escolher o que copiar…
collision-modal-overwrite-older = Sobrescrever apenas os mais antigos

# Phase 14e — subset picker
subset-title = Escolha quais origens copiar
subset-subtitle = A seleção completa não cabe no destino. Marque os itens que deseja copiar; os demais ficam de fora.
subset-loading = Medindo tamanhos…
subset-too-large = grande demais para contar
subset-budget = Disponível
subset-remaining = Restante
subset-confirm = Copiar seleção
history-rerun-hint = Refaz esta cópia — examina novamente cada arquivo na árvore de origem
history-clear-all = Limpar tudo
history-clear-all-confirm = Clique de novo para confirmar
history-clear-all-hint = Exclui todas as linhas do histórico. Requer um segundo clique para confirmar.
toast-history-cleared = Histórico limpo ({ $count } linhas removidas)

# Phase 15 — source-list ordering in the drop staging dialog
drop-dialog-sort-label = Ordem:
sort-custom = Personalizada
sort-name-asc = Nome A → Z (arquivos primeiro)
sort-name-desc = Nome Z → A (arquivos primeiro)
sort-size-asc = Menor tamanho primeiro (arquivos primeiro)
sort-size-desc = Maior tamanho primeiro (arquivos primeiro)
sort-reorder = Reordenar
sort-move-top = Mover para o topo
sort-move-up = Mover para cima
sort-move-down = Mover para baixo
sort-move-bottom = Mover para o fim

# Phase 16 — sort preset names for the Activity list (shorter
# wording than the DropStagingDialog variants; the Activity list
# is files-only so the "(files first)" suffix isn't relevant).
sort-name-asc-simple = Nome A → Z
sort-name-desc-simple = Nome Z → A
sort-size-asc-simple = Menor tamanho primeiro
sort-size-desc-simple = Maior tamanho primeiro
activity-sort-locked = A ordenação fica desativada enquanto uma cópia está em execução. Pause ou aguarde a conclusão e então altere a ordem.

# Phase 17 — collision-policy picker in the DropStagingDialog
drop-dialog-collision-label = Se um arquivo já existir:
collision-policy-keep-both = Manter ambos (renomear a nova cópia para _2, _3, …)
collision-policy-skip = Ignorar a nova cópia
collision-policy-overwrite = Sobrescrever o arquivo existente
collision-policy-overwrite-if-newer = Sobrescrever apenas se for mais recente
collision-policy-prompt = Perguntar sempre

# Phase 18 — progress labels on the DropStagingDialog Start button
drop-dialog-busy-checking = Verificando espaço livre…
drop-dialog-busy-enumerating = Contando arquivos…
drop-dialog-busy-starting = Iniciando cópia…
toast-enumeration-deferred = A árvore de origem é grande — pulando a lista prévia de arquivos; as linhas aparecerão conforme o mecanismo as processa.

# Context menu (per-row right-click)
menu-pause = Pausar
menu-resume = Retomar
menu-cancel = Cancelar
menu-remove = Remover da fila
menu-reveal-source = Mostrar origem na pasta
menu-reveal-destination = Mostrar destino na pasta

# Header / toolbar
header-eta-label = Tempo restante estimado
header-toolbar-label = Controles globais

# Footer
footer-queued = tarefas ativas
footer-total-bytes = em andamento
footer-errors = erros
footer-history = Histórico

# Empty state
empty-title = Solte arquivos ou pastas para copiar
empty-hint = Arraste itens para a janela. Pediremos um destino e enfileiraremos uma tarefa por origem.
empty-region-label = Lista de tarefas

# Details drawer
details-drawer-label = Detalhes da tarefa
details-source = Origem
details-destination = Destino
details-state = Estado
details-bytes = Bytes
details-files = Arquivos
details-speed = Velocidade
details-eta = Tempo restante
details-error = Erro

# Drop dialog
drop-dialog-title = Transferir itens soltos
drop-dialog-subtitle = { $count } item(ns) pronto(s) para transferir. Escolha uma pasta de destino para começar.
drop-dialog-mode = Operação
drop-dialog-copy = Copiar
drop-dialog-move = Mover
drop-dialog-pick-destination = Escolher destino
drop-dialog-change-destination = Alterar destino
drop-dialog-start-copy = Iniciar cópia
drop-dialog-start-move = Iniciar movimentação

# ETA placeholders
eta-calculating = calculando…
eta-unknown = desconhecido

# Toast messages
toast-job-done = Transferência concluída
toast-copy-queued = Cópia enfileirada
toast-move-queued = Movimentação enfileirada
toast-error-resolved = Erro resolvido
toast-collision-resolved = Conflito resolvido
toast-elevated-unavailable = A nova tentativa com elevação chega na Phase 17 — ainda não disponível
toast-clipboard-files-detected = Arquivos na área de transferência — pressione o atalho de colar para copiar via Copy That
toast-clipboard-no-files = A área de transferência não tem arquivos para colar
toast-error-log-exported = Registro de erros exportado

# Error modal (Phase 8)
error-modal-title = Uma transferência falhou
error-modal-retry = Tentar de novo
error-modal-retry-elevated = Tentar de novo com permissões elevadas
error-modal-skip = Ignorar
error-modal-skip-all-kind = Ignorar todos os erros deste tipo
error-modal-abort = Abortar tudo
error-modal-path-label = Caminho
error-modal-code-label = Código
error-drawer-pending-count = Mais erros aguardando
error-drawer-toggle = Recolher ou expandir

# Error-kind labels (Phase 8). Source of truth — engine maps each
# `CopyErrorKind` to one of these keys via `localized_key()`.
err-not-found = Arquivo não encontrado
err-permission-denied = Permissão negada
err-disk-full = O disco de destino está cheio
err-interrupted = Operação interrompida
err-verify-failed = Falha na verificação pós-cópia
err-path-escape = Caminho rejeitado — contém segmentos de diretório pai (..) ou bytes ilegais
err-path-invalid-encoding = Caminho rejeitado — a string contém UTF-8 inválido / caracteres de substituição
err-helper-invalid-json = O auxiliar privilegiado recebeu JSON malformado; ignorando esta solicitação
err-helper-grant-out-of-band = GrantCapabilities deve ser tratado pelo loop de execução do auxiliar, não pelo manipulador sem estado
err-randomness-unavailable = Falha no gerador de números aleatórios do SO; não é possível gerar um id de sessão
err-sparseness-mismatch = Não foi possível preservar o layout esparso no destino
err-io-other = Erro de E/S desconhecido

# Collision modal (Phase 8)
collision-modal-title = O arquivo já existe
collision-modal-overwrite = Sobrescrever
collision-modal-overwrite-if-newer = Sobrescrever se for mais recente
collision-modal-skip = Ignorar
collision-modal-keep-both = Manter ambos
collision-modal-rename = Renomear…
collision-modal-apply-to-all = Aplicar a todos
collision-modal-source = Origem
collision-modal-destination = Destino
collision-modal-size = Tamanho
collision-modal-modified = Modificado
collision-modal-hash-check = Hash rápido (SHA-256)
collision-modal-hash-computing = Calculando…
collision-modal-hash-identical = Idênticos
collision-modal-hash-different = Diferentes
collision-modal-rename-placeholder = Novo nome de arquivo
collision-modal-confirm-rename = Renomear

# Error log drawer (Phase 8)
error-log-title = Registro de erros
error-log-empty = Nenhum erro registrado
error-log-export-csv = Exportar CSV
error-log-export-txt = Exportar texto
error-log-clear = Limpar registro
error-log-col-time = Hora
error-log-col-job = Tarefa
error-log-col-path = Caminho
error-log-col-code = Código
error-log-col-message = Mensagem
error-log-col-resolution = Resolução

# History drawer (Phase 9)
history-title = Histórico
history-empty = Nenhuma tarefa registrada ainda
history-unavailable = O histórico de cópias não está disponível. O app não conseguiu abrir o repositório SQLite na inicialização.
history-filter-any = qualquer
history-filter-kind = Tipo
history-filter-status = Status
history-filter-text = Pesquisar
history-refresh = Atualizar
history-export-csv = Exportar CSV
history-purge-30 = Limpar > 30 dias
history-rerun = Refazer
history-detail-open = Detalhes
history-detail-title = Detalhes da tarefa
history-detail-empty = Nenhum item registrado
history-col-date = Data
history-col-kind = Tipo
history-col-src = Origem
history-col-dst = Destino
history-col-files = Arquivos
history-col-size = Tamanho
history-col-status = Status
history-col-duration = Duração
history-col-error = Erro
toast-history-exported = Histórico exportado
toast-history-rerun-queued = Reexecução enfileirada

# Totals drawer (Phase 10)
footer-totals = Totais
totals-title = Totais
totals-loading = Carregando totais…
totals-card-bytes = Total de bytes copiados
totals-card-files = Arquivos
totals-card-jobs = Tarefas
totals-card-avg-rate = Taxa média de transferência
totals-errors = erros
totals-spark-title = Últimos 30 dias
totals-kinds-title = Por tipo
totals-saved-title = Tempo economizado (estimado)
totals-saved-note = Estimativa em relação a uma cópia padrão de gerenciador de arquivos da mesma carga.
totals-reset = Redefinir estatísticas
totals-reset-confirm = Isso exclui todas as tarefas e itens armazenados. Continuar?
totals-reset-confirm-yes = Sim, redefinir
toast-totals-reset = Estatísticas redefinidas

# Phase 11a — i18n core: surface remaining user-visible strings so
# every string on a main-window screen flows through Fluent.

# Header language switcher (temporary placement — folds into
# Settings → General in Phase 12).
header-language-label = Idioma
header-language-title = Alterar idioma

# Job-kind labels. History filter, Totals breakdown, and history
# rows all map wire-format `kind` strings to these labels.
kind-copy = Copiar
kind-move = Mover
kind-delete = Excluir
kind-secure-delete = Exclusão segura

# History status labels. Distinct from `state-*` because the
# history wire format uses plain `running`/`succeeded`/... rather
# than the live `JobState` enum the queue exposes.
status-running = Em execução
status-succeeded = Concluído
status-failed = Falhou
status-cancelled = Cancelado
# Per-item status (not job-level): `ok` / `skipped` are only
# reachable from the history detail view.
status-ok = OK
status-skipped = Ignorado

# History drawer: search field placeholder + purge toast.
history-search-placeholder = /caminho
toast-history-purged = { $count } tarefas com mais de 30 dias foram removidas

# User-facing command-layer validation errors. The Rust side
# returns these keys when input is missing; the toast layer looks
# them up rather than showing the raw English.
err-source-required = É necessário pelo menos um caminho de origem.
err-destination-empty = O caminho de destino está vazio.
err-source-empty = O caminho de origem está vazio.

# Localised duration formatting for the Totals drawer and ETA
# fields. `{ $ms }`, `{ $s }`, `{ $m }`, `{ $h }` are integer
# placeables — the formatter passes pre-computed values in.
duration-lt-1s = < 1s
duration-ms = { $ms } ms
duration-seconds = { $s }s
duration-minutes-seconds = { $m }min { $s }s
duration-hours-minutes = { $h }h { $m }min
duration-zero = 0s

# Rate unit. Appended to a formatted byte size. Some languages
# render this with a leading space ("Ko/s"); keep it translatable
# even though the SI-derived "/s" is near-universal.
rate-unit-per-second = { $size }/s

# Phase 11b — Settings modal skeleton. Phase 12 expanded this into a
# full six-tab preferences window; the `settings-phase-12-hint` key
# is retired in favour of `settings-tab-profiles` + concrete labels.
settings-title = Configurações
settings-tab-general = Geral
settings-tab-appearance = Aparência
settings-section-language = Idioma
settings-phase-12-hint = Mais configurações (tema, padrões de transferência, algoritmo de verificação, perfis) chegam na Phase 12.

# Phase 12 — full Settings window keys. Grouped by tab so the
# human-review pass can pick one section at a time. Every field
# label and every option label that the user will actually read
# has its own key; dynamic wire values (enum discriminants like
# "auto" / "prefer") stay untranslated on the wire.

settings-loading = Carregando configurações…
settings-tab-transfer = Transferência
settings-tab-filters = Filtros
settings-tab-shell = Shell
settings-tab-secure-delete = Exclusão segura
settings-tab-advanced = Avançado
settings-tab-updater = Atualizações
settings-tab-profiles = Perfis

# General tab additions
settings-section-theme = Tema
settings-theme-auto = Automático
settings-theme-light = Claro
settings-theme-dark = Escuro
settings-start-with-os = Iniciar com o sistema
settings-single-instance = Instância única em execução
settings-minimize-to-tray = Minimizar para a bandeja ao fechar
settings-error-display-mode = Estilo do aviso de erro
settings-error-display-modal = Modal (bloqueia o app)
settings-error-display-drawer = Gaveta (não bloqueia)
settings-error-display-mode-hint = O modal interrompe a fila até você decidir. A gaveta mantém a fila em andamento e permite triar os erros no canto.
settings-paste-shortcut = Colar arquivos via atalho global
settings-paste-shortcut-combo = Combinação de atalho
settings-paste-shortcut-hint = Pressione esta combinação em qualquer lugar do sistema para colar arquivos copiados do Explorer / Finder / Files via Copy That. CmdOrCtrl corresponde a Cmd no macOS e Ctrl no Windows / Linux.
settings-clipboard-watcher = Monitorar a área de transferência por arquivos copiados
settings-clipboard-watcher-hint = Mostra um aviso quando URLs de arquivo aparecem na área de transferência, indicando que você pode colar via Copy That. Verifica a cada 500 ms quando ativado.

# Transfer tab
settings-buffer-size = Tamanho do buffer
settings-verify = Verificar após copiar
settings-verify-off = Desativado
settings-concurrency = Simultaneidade
settings-concurrency-auto = Automático
settings-reflink = Reflink / caminhos rápidos
settings-reflink-prefer = Preferir
settings-reflink-avoid = Evitar reflink
settings-reflink-disabled = Sempre usar o mecanismo assíncrono
settings-fsync-on-close = Sincronizar com o disco ao fechar (mais lento, mais seguro)
settings-preserve-timestamps = Preservar carimbos de data/hora
settings-preserve-permissions = Preservar permissões
settings-preserve-acls = Preservar ACLs (Phase 14)
settings-preserve-sparseness = Preservar arquivos esparsos
settings-preserve-sparseness-hint = Copia apenas as extensões alocadas de arquivos esparsos (discos de VM, arquivos de banco de dados) para que o destino mantenha o mesmo tamanho em disco da origem.
settings-force-parallel-chunks = Cópia paralela em múltiplos blocos (apenas RAID / arrays)
settings-force-parallel-chunks-hint = Divide cada cópia grande em blocos simultâneos. Só ajuda em destinos em faixa/RAID/rede; deixa mais LENTO um único SSD/NVMe (-25% a -76%). Mantenha desligado a menos que o destino seja um array de vários discos.

# Shell tab
settings-context-menu = Ativar entradas no menu de contexto do shell
settings-intercept-copy = Interceptar o manipulador de cópia padrão (Windows)
settings-intercept-copy-hint = Quando ativado, o Ctrl+C / Ctrl+V do Explorer passa pelo Copy That. O registro chega na Phase 14.
settings-notify-completion = Notificar ao concluir a tarefa

# Secure delete tab
settings-shred-method = Método de trituração padrão
settings-shred-zero = Zero (1 passagem)
settings-shred-random = Aleatório (1 passagem)
settings-shred-dod3 = DoD 5220.22-M (3 passagens)
settings-shred-dod7 = DoD 5220.22-M (7 passagens)
settings-shred-gutmann = Gutmann (35 passagens)
settings-shred-nist = NIST 800-88
settings-shred-confirm-twice = Exigir dupla confirmação antes de triturar

# Advanced tab
settings-log-level = Nível de log
settings-log-off = Desativado
settings-telemetry = Telemetria
settings-telemetry-never = Nunca — sem envio de dados em nenhum nível de log
settings-error-policy = Política de erro padrão
settings-error-policy-ask = Perguntar
settings-error-policy-skip = Ignorar
settings-error-policy-retry = Tentar de novo com espera progressiva
settings-error-policy-abort = Abortar na primeira falha
settings-history-retention = Retenção do histórico (dias)
settings-history-retention-hint = 0 = manter para sempre. Qualquer outro valor remove automaticamente as tarefas mais antigas na inicialização.
settings-database-path = Caminho do banco de dados
settings-database-path-default = (padrão — diretório de dados do SO)
settings-reset-all = Restaurar padrões
settings-reset-confirm = Restaurar todas as preferências para o padrão? Os perfis não são afetados.

# Profiles tab
settings-profiles-hint = Salve as configurações atuais com um nome; carregue-o depois para voltar a elas sem mexer em cada ajuste.
settings-profile-name-placeholder = Nome do perfil
settings-profile-save = Salvar
settings-profile-import = Importar…
settings-profile-load = Carregar
settings-profile-export = Exportar…
settings-profile-delete = Excluir
settings-profile-empty = Nenhum perfil salvo ainda.
settings-profile-import-prompt = Nome para o perfil importado:

# Toasts driven by Phase 12 profile actions
toast-settings-reset = Configurações redefinidas
toast-profile-saved = Perfil salvo
toast-profile-loaded = Perfil carregado
toast-profile-exported = Perfil exportado
toast-profile-imported = Perfil importado

# Phase 14a — enumeration-time filters
settings-filters-hint = Pula arquivos no momento da enumeração para que o mecanismo nem os abra. As inclusões se aplicam apenas a arquivos; as exclusões também removem diretórios correspondentes.
settings-filters-enabled = Ativar filtros para cópias de árvore
settings-filters-include-globs = Globs de inclusão
settings-filters-include-globs-placeholder = **/*.txt
settings-filters-include-globs-hint = Um glob por linha. Quando preenchido, um arquivo deve corresponder a pelo menos uma inclusão para ser mantido. Os diretórios são sempre percorridos.
settings-filters-exclude-globs = Globs de exclusão
settings-filters-exclude-globs-placeholder = **/node_modules
settings-filters-exclude-globs-hint = Um glob por linha. As correspondências removem toda a subárvore no caso de diretórios; arquivos correspondentes são ignorados.
settings-filters-size-range = Intervalo de tamanho de arquivo
settings-filters-min-size-bytes = Tamanho mínimo (bytes, em branco = sem mínimo)
settings-filters-max-size-bytes = Tamanho máximo (bytes, em branco = sem máximo)
settings-filters-date-range = Intervalo de data de modificação
settings-filters-min-mtime = Modificado em ou depois de
settings-filters-max-mtime = Modificado em ou antes de
settings-filters-attributes = Bits de atributo
settings-filters-skip-hidden = Ignorar arquivos / pastas ocultos
settings-filters-skip-system = Ignorar arquivos de sistema (apenas Windows)
settings-filters-skip-readonly = Ignorar arquivos somente leitura

# Phase 15 — auto-update
settings-updater-hint = O Copy That verifica atualizações assinadas no máximo uma vez por dia. As atualizações são instaladas ao fechar o app.
settings-updater-auto-check = Verificar atualizações ao iniciar
settings-updater-channel = Canal de lançamento
settings-updater-channel-stable = Estável
settings-updater-channel-beta = Beta (pré-lançamento)
settings-updater-last-check = Última verificação
settings-updater-last-never = Nunca
settings-updater-check-now = Verificar atualizações agora
settings-updater-checking = Verificando…
settings-updater-available = Atualização disponível
settings-updater-up-to-date = Você está usando a versão mais recente.
settings-updater-dismiss = Ignorar esta versão
settings-updater-dismissed = Ignorada
toast-update-available = Uma versão mais recente está disponível
toast-update-up-to-date = Você já está na versão mais recente

# Phase 19a — disk-backed file enumeration (TeraCopy-compatible scan DB)
scan-progress-title = Examinando…
scan-progress-stats = { $files } arquivos · { $bytes } até agora
scan-pause-button = Pausar exame
scan-resume-button = Retomar exame
scan-cancel-button = Cancelar exame
scan-cancel-confirm = Cancelar o exame e descartar o progresso?
scan-db-header = Banco de dados de exame
scan-db-hint = Banco de dados de exame em disco para tarefas com milhões de arquivos.
advanced-scan-hash-during = Calcular somas de verificação durante o exame
advanced-scan-db-path = Local do banco de dados de exame
advanced-scan-retention-days = Excluir automaticamente exames concluídos após (dias)
advanced-scan-max-keep = Máximo de bancos de dados de exame a manter

# Phase 19b — filesystem-snapshot source for locked files.
settings-on-locked = Quando um arquivo está bloqueado
settings-on-locked-ask = Perguntar na primeira vez
settings-on-locked-retry = Tentar brevemente e então exibir o erro
settings-on-locked-skip = Ignorar o arquivo bloqueado
settings-on-locked-snapshot = Usar um instantâneo do sistema de arquivos
settings-on-locked-hint = Elimina os erros "arquivo em uso por outro processo". O Copy That cria um instantâneo do volume de origem (VSS no Windows, ZFS/Btrfs no Linux, APFS no macOS) e lê da cópia do instantâneo.
snapshot-prompt-title = Este arquivo está em uso por outro processo
snapshot-prompt-body = Outro programa mantém { $path } aberto para escrita exclusiva. Escolha como o Copy That deve tratar este e arquivos semelhantes no mesmo volume.
snapshot-source-active = 📷 Lendo do instantâneo { $kind } de { $volume }
snapshot-create-failed = Não foi possível criar um instantâneo do volume de origem
snapshot-vss-needs-elevation = A leitura de um instantâneo VSS requer permissão de Administrador. O Copy That pedirá para você autorizar.
snapshot-cleanup-failed = O auxiliar de instantâneos relatou uma falha de limpeza — uma cópia de sombra residual pode permanecer no volume.

# Phase 20 — durable resume journal.
resume-prompt-title = Retomar transferências anteriores?
resume-prompt-body = O Copy That detectou { $count } transferência(s) inacabada(s) de uma sessão anterior. Escolha o que fazer com cada uma.
resume-prompt-resume = Retomar
resume-prompt-resume-all = Retomar todas
resume-discard-one = Não retomar
resume-discard-all = Descartar todas
resume-aborted-hash-mismatch = Os primeiros { $offset } bytes do destino não correspondem à origem — reiniciando do começo.
settings-auto-resume = Retomar automaticamente tarefas interrompidas sem perguntar
settings-auto-resume-hint = Pula o aviso de retomada na inicialização e reenfileira silenciosamente cada tarefa inacabada. Desativado por padrão.

# Phase 21 — bandwidth shaping (GCRA token bucket + schedule + auto-throttle).
settings-tab-network = Rede
settings-network-hint = Limite sua taxa de transferência para manter o restante da rede utilizável. Aplique globalmente, siga um cronograma diário ou reaja automaticamente a conexões Wi-Fi limitadas / bateria / celular.
settings-network-mode = Limite de banda
settings-network-mode-off = Desativado (sem limite)
settings-network-mode-fixed = Valor fixo
settings-network-mode-schedule = Usar cronograma
settings-network-cap-mbps = Limite (MB/s)
settings-network-schedule = Cronograma (formato rclone)
settings-network-schedule-hint = Limites HH:MM,taxa separados por espaços mais regras opcionais de dia Mon-Fri,taxa. Taxas: 512k, 10M, 2G, off, unlimited. Exemplo: 08:00,512k 18:00,10M Sat-Sun,unlimited.
settings-network-auto-header = Limitação automática
settings-network-auto-metered = Em Wi-Fi limitado
settings-network-auto-battery = Na bateria
settings-network-auto-cellular = Em rede celular
settings-network-auto-unchanged = Não substituir
settings-network-auto-pause = Pausar transferências
settings-network-auto-cap = Limitar a um valor fixo
shape-badge-paused = pausado
shape-badge-tooltip = Limite de banda ativo — clique para abrir Configurações → Rede
shape-badge-source-schedule = agendado
shape-badge-source-metered = limitado
shape-badge-source-battery = na bateria
shape-badge-source-cellular = celular
shape-badge-source-settings = ativo
shape-error-schedule-invalid = O formato do cronograma não é válido: { $message }

# Phase 22 — aggregate conflict dialog v2 (thumbnails, per-pattern
# rules, and reusable conflict profiles). Every key below is user-
# visible text in the `ConflictBatchModal.svelte` component.
conflict-batch-title = { $count } conflitos de arquivo em { $jobname }
conflict-batch-state-pending = Pendente
conflict-batch-state-resolved = Resolvido
conflict-batch-action-overwrite = Sobrescrever
conflict-batch-action-skip = Ignorar
conflict-batch-action-keep-both = Manter ambos
conflict-batch-action-newer-wins = O mais recente vence
conflict-batch-action-larger-wins = O maior vence
conflict-batch-bulk-apply-selected = Aplicar aos selecionados
conflict-batch-bulk-apply-extension = Aplicar a todos desta extensão
conflict-batch-bulk-apply-glob = Aplicar ao glob correspondente…
conflict-batch-bulk-apply-remaining = Aplicar a todos os restantes
conflict-batch-bulk-glob-placeholder = ex.: **/*.tmp
conflict-batch-save-profile = Salvar estas regras como perfil…
conflict-batch-profile-placeholder = Nome do perfil
conflict-batch-matched-rule = via regra '{ $rule }' → { $action }
conflict-batch-empty = Todos os conflitos resolvidos
conflict-batch-source-vs-destination = Origem vs. destino
conflict-batch-source-label = Origem
conflict-batch-destination-label = Destino
conflict-batch-size-label = Tamanho
conflict-batch-modified-label = Modificado
conflict-batch-close = Fechar
conflict-batch-profile-saved = Perfil de conflito salvo

# Phase 23 — sparse-file preservation. The toast fires once per
# destination volume when the filesystem can't preserve holes; the
# warning line is surfaced in the job detail drawer so the user knows
# the dst is larger on disk than the source was.
sparse-not-supported-title = O destino preenche arquivos esparsos
sparse-not-supported-body = { $dst_fs } não oferece suporte a arquivos esparsos. As lacunas na origem foram gravadas como zeros, então o destino ocupa mais espaço em disco.
sparse-warning-densified = Layout esparso preservado: apenas as extensões alocadas foram copiadas.
sparse-warning-mismatch = Incompatibilidade de layout esparso — o destino pode ser maior do que o esperado.

# Phase 24 — security-metadata preservation. The Mark-of-the-Web
# (Zone.Identifier ADS) toggle is security-sensitive: turning it off
# lets a downloaded executable shed its SmartScreen / Office Protected
# View flag on copy, which is why the tooltip carries an explicit
# warning. AppleDouble fallback emits `._<filename>` sidecars on
# destination filesystems that can't hold the foreign metadata.
settings-preserve-security-metadata = Preservar metadados de segurança
settings-preserve-security-metadata-hint = Captura e reaplica fluxos de metadados fora de banda (NTFS ADS / xattrs / ACLs POSIX / contextos SELinux / capacidades de arquivo do Linux / forks de recursos do macOS) em cada cópia.
settings-preserve-motw = Preservar Mark-of-the-Web (sinalizador de baixado-da-internet)
settings-preserve-motw-hint = Essencial para a segurança. O SmartScreen e o Office Protected View usam este fluxo para avisar sobre arquivos baixados da internet. Desativar permite que um executável baixado perca seu marcador de origem ao copiar e contorne as proteções do sistema operacional.
settings-preserve-posix-acls = Preservar ACLs POSIX e atributos estendidos
settings-preserve-posix-acls-hint = Transfere xattrs user.* / system.* / trusted.* e listas de controle de acesso POSIX na cópia.
settings-preserve-selinux = Preservar contextos SELinux
settings-preserve-selinux-hint = Transfere o rótulo security.selinux na cópia para que daemons sob políticas MAC ainda possam acessar o arquivo.
settings-preserve-resource-forks = Preservar forks de recursos e informações do Finder do macOS
settings-preserve-resource-forks-hint = Transfere o fork de recursos legado e o FinderInfo (etiquetas de cor, metadados Carbon) na cópia.
settings-appledouble-fallback = Usar arquivo auxiliar AppleDouble em sistemas de arquivos incompatíveis
meta-translated-to-appledouble = Metadados externos armazenados no arquivo auxiliar AppleDouble (._{ $ext })

# Phase 25 — two-way sync with vector-clock conflict detection.
# The drawer lists configured sync pairs; each pair runs an
# independent reconciliation round against a per-pair `.copythat-sync.db`
# state store. Concurrent edits from a common ancestor surface as
# conflicts rather than silent overwrites; the losing side's content
# is preserved as `name.sync-conflict-YYYYMMDD-HHMMSS-<host>.ext`.
footer-sync = Sincronizar
sync-drawer-title = Sincronização bidirecional
sync-drawer-hint = Mantenha duas pastas sincronizadas sem sobrescritas silenciosas. Edições simultâneas surgem como conflitos que você pode resolver.
sync-add-pair = Adicionar par
sync-add-cancel = Cancelar
sync-refresh = Atualizar
sync-add-save = Salvar par
sync-add-saving = Salvando…
sync-add-missing-fields = Rótulo, caminho esquerdo e caminho direito são todos obrigatórios.
sync-remove-confirm = Remover este par de sincronização? O banco de dados de estado é preservado; as pastas permanecem intactas.
sync-field-label = Rótulo
sync-field-label-placeholder = ex.: Documentos ↔ NAS
sync-field-left = Pasta esquerda
sync-field-left-placeholder = Escolha ou cole um caminho absoluto
sync-field-right = Pasta direita
sync-field-right-placeholder = Escolha ou cole um caminho absoluto
sync-field-mode = Modo
sync-mode-two-way = Bidirecional
sync-mode-mirror-left-to-right = Espelho (esquerda → direita)
sync-mode-mirror-right-to-left = Espelho (direita → esquerda)
sync-mode-contribute-left-to-right = Contribuir (esquerda → direita, sem exclusões)
sync-no-pairs = Nenhum par de sincronização configurado ainda. Clique em "Adicionar par" para começar.
sync-loading = Carregando pares configurados…
sync-never-run = Nunca executado
sync-running = Em execução
sync-run-now = Executar agora
sync-cancel = Cancelar
sync-remove-pair = Remover
sync-view-conflicts = Ver conflitos ({ $count })
sync-conflicts-heading = Conflitos
sync-no-conflicts = Nenhum conflito na última execução.
sync-winner = Vencedor
sync-side-left-to-right = esquerda
sync-side-right-to-left = direita
sync-conflict-kind-concurrent-write = Edição simultânea
sync-conflict-kind-delete-edit = Excluir ↔ editar
sync-conflict-kind-add-add = Ambos os lados adicionaram
sync-conflict-kind-corrupt-equal = O conteúdo divergiu sem uma nova gravação
sync-resolve-keep-left = Manter o esquerdo
sync-resolve-keep-right = Manter o direito
sync-resolve-keep-both = Manter ambos
sync-resolve-three-way = Resolver via mesclagem de 3 vias
sync-resolve-phase-53-tooltip = A mesclagem interativa de 3 vias para arquivos não textuais chega na Phase 53.
sync-error-prefix = Erro de sincronização

# Phase 26 — real-time mirror watcher. "Live mirror" starts a
# filesystem watcher on the pair's left side; every debounced event
# triggers a re-sync. The watcher filters vim swap files / Office
# lock files / atomic-save temp names so one logical save becomes
# exactly one sync round.
live-mirror-start = Iniciar espelho ao vivo
live-mirror-stop = Parar espelho ao vivo
live-mirror-watching = Monitorando
live-mirror-toggle-hint = Sincroniza novamente de forma automática a cada alteração detectada no sistema de arquivos. Uma thread em segundo plano por par ativo.
watch-event-prefix = Alteração de arquivo
watch-overflow-recovered = O buffer do monitor transbordou; reenumerando para recuperar

# Phase 27 — content-defined chunk store. Enables delta-resume (a
# retry only re-writes chunks that actually changed) and same-job
# dedup (files sharing content blocks store those blocks once). The
# store is disk-backed under `<data-dir>/chunks/` by default and is
# the foundation for the Phase 49–51 moonshot repository phases.
chunk-store-section = Repositório de blocos
chunk-store-enable = Ativar repositório de blocos (retomada delta e dedup)
chunk-store-enable-hint = Divide cada arquivo copiado por conteúdo (FastCDC) e armazena os blocos por endereçamento de conteúdo. As novas tentativas só regravam os blocos alterados; arquivos com conteúdo compartilhado fazem dedup automaticamente.
chunk-store-location = Local do repositório de blocos
chunk-store-max-size = Tamanho máximo do repositório de blocos
chunk-store-prune = Remover blocos com mais de (dias)
chunk-store-savings = { $gib } GiB economizados via dedup de blocos
chunk-store-disk-usage = Usando { $size } em { $chunks } blocos

# Phase 28 — tray-resident Drop Stack. The stack is a persistent list
# of paths gathered from multiple sources (Explorer drag, main-window
# context menu, CLI, drag onto the Drop Stack window) that the user
# can dispatch to a destination in one go.
dropstack-window-title = Drop Stack
dropstack-tray-open = Drop Stack
dropstack-empty-title = O Drop Stack está vazio
dropstack-empty-hint = Arraste arquivos para cá do Explorer ou clique com o botão direito em uma linha de tarefa para adicioná-la.
dropstack-add-to-stack = Adicionar ao Drop Stack
dropstack-copy-all-to = Copiar tudo para…
dropstack-move-all-to = Mover tudo para…
dropstack-clear = Limpar pilha
dropstack-remove-row = Remover da pilha
dropstack-path-missing-toast = { $path } solto — o arquivo não existe mais.
dropstack-always-on-top = Manter o Drop Stack sempre no topo
dropstack-show-tray-icon = Mostrar o ícone do Copy That na bandeja
dropstack-open-on-start = Abrir o Drop Stack automaticamente ao iniciar o app
dropstack-count = { $count } caminho

# Phase 29 — spring-loaded folders + native DnD polish. The Settings
# → General tab carries the knobs; DropTarget / DestinationPicker
# Svelte components use the dropzone-* keys at runtime.
settings-dnd-heading = Arrastar e soltar
settings-dnd-spring-load = Abrir pastas automaticamente ao arrastar sobre elas
settings-dnd-spring-delay = Atraso da abertura automática (ms)
settings-dnd-thumbnails = Mostrar miniaturas ao arrastar
settings-dnd-invalid-highlight = Destacar destinos de soltura inválidos
dropzone-invalid-title = Não é um destino de soltura válido
dropzone-invalid-readonly = O destino é somente leitura
dropzone-picker-title = Escolha um destino
dropzone-picker-up = Acima
dropzone-picker-path = Caminho atual
dropzone-picker-root = Raízes
dropzone-picker-use-this = Usar esta pasta
dropzone-picker-empty = Nenhuma subpasta
dropzone-picker-cancel = Cancelar

# Phase 30 — cross-platform path translation. Settings → Transfer
# exposes these under a "Cross-platform compatibility" subsection.
translate-heading = Compatibilidade entre plataformas
translate-unicode-label = Normalização Unicode
translate-unicode-auto = Detectar destino automaticamente
translate-unicode-windows = NFC (Windows / Linux)
translate-unicode-macos = Manter como está (macOS / APFS)
translate-line-endings-label = Converter quebras de linha para arquivos de texto
translate-line-endings-allowlist = Extensões de arquivo de texto
reserved-name-label = Tratamento de nomes reservados do Windows
reserved-name-suffix = Acrescentar "_" (CON.txt → CON_.txt)
reserved-name-reject = Rejeitar e avisar
long-path-label = Usar o prefixo de caminho longo do Windows (\\?\) quando acima de 260 caracteres
long-path-hint = Alguns compartilhamentos de rede e ferramentas legadas não respeitam o namespace \\?\.

# Phase 31 — power-aware copying. Settings → Power & State tab + the
# header badge that renders "⏸ Paused — Zoom call detected" when the
# runner's power subscriber has paused or capped due to a policy match.
power-heading = Energia e estado
power-enabled = Ativar regras com reconhecimento de energia
power-battery-label = Na bateria
power-metered-label = Em Wi-Fi limitado
power-cellular-label = Em rede celular
power-presentation-label = Ao apresentar (Zoom / Teams / Keynote)
power-fullscreen-label = Quando um app está em tela cheia
power-thermal-label = Quando a CPU está com limitação térmica
power-rule-continue = Continuar em velocidade máxima
power-rule-pause = Pausar todas as tarefas
power-rule-cap = Limitar a banda
power-rule-cap-percent = Limitar a uma porcentagem da taxa atual
power-reason-on-battery = na bateria
power-reason-metered-network = rede limitada
power-reason-cellular-network = rede celular
power-reason-presenting = modo de apresentação
power-reason-fullscreen = app em tela cheia
power-reason-thermal-throttling = a CPU está com limitação

# Phase 32 — cloud backend matrix via OpenDAL. Settings → Remotes
# tab + the Add-backend wizard that writes one entry per remote into
# the keychain-backed credential store.
remote-heading = Backends remotos
remote-add = Adicionar backend
remote-list-empty = Nenhum backend remoto configurado
remote-test = Testar conexão
remote-test-success = Conexão bem-sucedida
remote-test-failed = Falha na conexão
remote-remove = Remover backend
remote-name-label = Nome de exibição
remote-kind-label = Tipo de backend
remote-save = Salvar backend
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
backend-local-fs = Sistema de arquivos local
cloud-config-bucket = Bucket
cloud-config-region = Região
cloud-config-endpoint = URL do endpoint
cloud-config-root = Caminho raiz
cloud-error-invalid-config = A configuração do backend é inválida
cloud-error-network = Erro de rede ao contatar o backend
cloud-error-not-found = Objeto não encontrado no caminho solicitado
cloud-error-permission = Permissão negada pelo backend remoto
cloud-error-keychain = Falha ao acessar o chaveiro do SO
settings-tab-remotes = Remotos
settings-tab-mobile = Celular

# Phase 33 — mount Copy That's chunk store + history archive as a
# read-only filesystem (FUSE / WinFsp). Surfaced by the History tab
# context menu's "Mount snapshot" action + the Settings → Advanced
# "Mount latest on launch" toggle.
mount-heading = Montar instantâneo
mount-action-mount = Montar instantâneo
mount-action-unmount = Desmontar
mount-status-mounted = Montado em { $path }
mount-error-unsafe-mountpoint = O caminho do ponto de montagem não é seguro
mount-error-mountpoint-not-empty = O ponto de montagem deve ser um diretório vazio
mount-error-backend-unavailable = O backend de montagem não está disponível neste sistema
mount-error-archive-read = Falha ao ler o arquivo morto
mount-picker-title = Escolha o diretório do ponto de montagem
mount-toast-mounted = Instantâneo montado em { $path }
mount-toast-unmounted = Instantâneo desmontado
mount-toast-failed = Falha na montagem: { $reason }
settings-mount-heading = Montar instantâneos
settings-mount-hint = Expõe o arquivo de histórico como um sistema de arquivos somente leitura. A Phase 33b conecta o fluxo do executor; os backends FUSE/WinFsp do kernel chegam na Phase 33c.
settings-mount-on-launch = Montar o instantâneo mais recente ao iniciar
settings-mount-on-launch-path = Caminho do ponto de montagem
settings-mount-on-launch-path-placeholder = ex.: C:\Mounts\copythat

# Phase 34 — enterprise-grade audit log export + WORM (write-once-read-
# many) tamper-resistance. Settings → Advanced → Audit log collects the
# format + destination + rotation + WORM toggle; runtime records job and
# file events into the tamper-evident chain-hashed sink.
settings-audit-heading = Registro de auditoria
settings-audit-hint = Registro somente de acréscimo e à prova de adulteração de cada evento de tarefa e arquivo. Os formatos incluem CSV, JSON-lines, Syslog RFC 5424, ArcSight CEF e QRadar LEEF.
settings-audit-enable = Ativar registro de auditoria
settings-audit-format = Formato do registro
settings-audit-format-json-lines = JSON lines (padrão recomendado)
settings-audit-format-csv = CSV (compatível com planilhas)
settings-audit-format-syslog = Syslog (RFC 5424)
settings-audit-format-cef = CEF (ArcSight)
settings-audit-format-leef = LEEF 2.0 (IBM QRadar)
settings-audit-file-path = Caminho do arquivo de registro
settings-audit-file-path-placeholder = ex.: C:\ProgramData\CopyThat\audit.log
settings-audit-max-size = Rotacionar após (bytes, 0 = nunca)
settings-audit-worm = Ativar modo WORM (gravar uma vez, ler muitas)
settings-audit-worm-hint = Aplica o sinalizador somente de acréscimo da plataforma (chattr +a no Linux, chflags uappnd no macOS, atributo somente leitura no Windows) após cada criação ou rotação. Até um administrador deve limpar explicitamente o sinalizador para truncar o registro.
settings-audit-test-write = Gravação de teste
settings-audit-verify-chain = Verificar cadeia
toast-audit-test-write-ok = Gravação de teste do registro de auditoria bem-sucedida
toast-audit-verify-ok = Cadeia de auditoria verificada e intacta
toast-audit-verify-failed = A verificação da cadeia de auditoria relatou divergências

# Phase 35 — destination encryption via age (X25519 / passphrase /
# SSH) + on-the-fly zstd compression with a per-extension deny
# heuristic. Surfaced through Settings → Transfer → Encryption +
# Compression; engine short-circuits to the crypt pipeline when
# either stage is active.
settings-crypt-heading = Criptografia e compactação
settings-crypt-hint = Transforme o conteúdo dos arquivos antes que cheguem ao destino. A criptografia usa o formato age; a compactação usa zstd e pode pular mídia já compactada por extensão.
settings-crypt-encryption-mode = Criptografia
settings-crypt-encryption-off = Desativada
settings-crypt-encryption-passphrase = Frase secreta (perguntar no início da cópia)
settings-crypt-encryption-recipients = Chaves de destinatários a partir de arquivo
settings-crypt-encryption-hint = As frases secretas são mantidas apenas na memória durante a cópia. Os arquivos de destinatários listam uma chave pública age1… ou ssh- por linha.
settings-crypt-recipients-file = Caminho do arquivo de destinatários
settings-crypt-recipients-file-placeholder = ex.: C:\Users\me\recipients.txt
settings-crypt-compression-mode = Compactação
settings-crypt-compression-off = Desativada
settings-crypt-compression-always = Sempre
settings-crypt-compression-smart = Inteligente (pular mídia já compactada)
settings-crypt-compression-hint = O modo inteligente pula jpg, mp4, zip, 7z e formatos semelhantes que não se beneficiam do zstd. O modo Sempre compacta cada arquivo no nível escolhido.
settings-crypt-compression-level = Nível zstd (1-22)
settings-crypt-compression-level-hint = Números menores são mais rápidos; números maiores compactam mais. O nível 3 corresponde ao padrão da CLI do zstd.
compress-footer-savings = 💾 { $original } → { $compressed } ({ $percent }% economizado)
compress-savings-toast = Compactado em { $percent }% ({ $bytes } economizados)
crypt-toast-recipients-loaded = { $count } destinatários de criptografia carregados
crypt-toast-recipients-error = Falha ao carregar destinatários: { $reason }
crypt-toast-passphrase-required = A criptografia precisa de uma frase secreta antes de a cópia começar
crypt-toast-passphrase-set = Frase secreta de criptografia capturada
crypt-footer-encrypted-badge = 🔒 Criptografado (age)
crypt-footer-compressed-badge = 📦 Compactado (zstd)

# Phase 36 — copythat CLI surface. Documented exit codes + the
# user-facing strings for plan/apply/verify/config. Help text stays
# in English (engineering accessibility) but error / info / status
# strings are localized so a localized desktop installation surfaces
# CLI errors in the same language as the GUI.
cli-help-tagline = Copy That CLI — cópia de arquivos byte a byte exata, sincronização, verificação e auditoria para pipelines de CI/CD.
cli-help-exit-codes = Exit codes: 0 success, 1 error, 2 pending, 3 collision, 4 verify-fail, 5 net, 6 perm, 7 disk-full, 8 cancel, 9 config.
cli-error-bad-args = copy/move requer pelo menos uma origem e um destino
cli-error-unknown-algo = Algoritmo de verificação desconhecido: { $algo }
cli-error-missing-spec = --spec é obrigatório para plan/apply
cli-error-spec-parse = Falha ao analisar o jobspec { $path }: { $reason }
cli-error-spec-empty-sources = A lista de origens do jobspec está vazia
cli-info-shape-recorded = Limite de banda "{ $rate }" registrado; a aplicação é feita via copythat-shape
cli-info-stub-deferred = { $command } está preparado para a conexão de acompanhamento da Phase 36
cli-plan-summary = Plano: { $actions } ação(ões), { $bytes } byte(s); { $already_done } já em vigor
cli-plan-pending = O plano relata ações pendentes; execute novamente com `apply` para concluir
cli-plan-already-done = O plano não relata nada a fazer (idempotente)
cli-apply-success = Apply concluído sem erros
cli-apply-failed = Apply concluído com um ou mais erros
cli-verify-ok = Verificação ok: { $algo } { $digest }
cli-verify-failed = Verificação FALHOU para { $path } ({ $algo })
cli-config-set = Definir { $key } = { $value }
cli-config-reset = Redefinir { $key } para o padrão
cli-config-unknown-key = Chave de configuração desconhecida: { $key }
cli-completions-emitted = Conclusões de shell para { $shell } impressas no stdout

# Phase 37 — desktop-side mobile companion. Settings → Mobile panel
# strings + the SAS-confirmation modal + push-notification toasts.
# The actual mobile UI lives in the Phase 37 follow-up Tauri Mobile
# target; the phone displays its own localized SAS prompt.
settings-mobile-heading = Companheiro móvel
settings-mobile-hint = Pareie um iPhone ou um celular Android para navegar pelo histórico, iniciar perfis salvos e jobspecs da Phase 36 e receber notificações de conclusão.
settings-mobile-pair-toggle = Permitir novos pareamentos
settings-mobile-pair-active = Servidor de pareamento ativo — escaneie o QR com o app móvel do Copy That
settings-mobile-pair-button = Iniciar pareamento
settings-mobile-revoke-button = Revogar
settings-mobile-no-pairings = Nenhum dispositivo pareado ainda
settings-mobile-pair-port = Porta de vinculação (0 = escolher uma livre)
pair-sas-prompt = Ambas as telas devem mostrar os mesmos quatro emojis. Toque em Combina se forem iguais.
pair-sas-confirm = Combina
pair-sas-reject = Não combina — cancelar
pair-toast-success = Pareado com { $device }
pair-toast-failed = Falha no pareamento: { $reason }
push-toast-sent = Notificação enviada para { $device }
push-toast-failed = Falha ao enviar notificação para { $device }: { $reason }

# Phase 38 — aggregate destination dedup + reflink fallback ladder.
# Settings → Transfer → Dedup panel + per-job-row badges (⚡
# Reflinked / 🔗 Hardlinked / 🧩 Chunk-shared / 📋 Copied) +
# pre-pass dedup-scan modal.
settings-dedup-heading = Dedup de destino
settings-dedup-hint = Quando a origem e o destino compartilham um volume, o Copy That pode clonar arquivos no nível do sistema de arquivos em vez de copiar bytes. O reflink é instantâneo e seguro; o hardlink é mais rápido, mas os dois nomes compartilham estado.
settings-dedup-mode-auto = Escala automática (reflink → hardlink → bloco → cópia)
settings-dedup-mode-reflink-only = Apenas reflink
settings-dedup-mode-hardlink-aggressive = Agressivo (reflink + hardlink mesmo em arquivos graváveis)
settings-dedup-mode-off = Desativado (sempre copiar bytes)
settings-dedup-hardlink-policy = Política de hardlink
settings-dedup-prescan = Examinar previamente a árvore de destino em busca de conteúdo duplicado
dedup-badge-reflinked = ⚡ Reflink
dedup-badge-hardlinked = 🔗 Hardlink
dedup-badge-chunk-shared = 🧩 Bloco compartilhado
dedup-badge-copied = 📋 Copiado
phase42-paranoid-verify-label = Verificação paranoica
phase42-paranoid-verify-hint = Descarta as páginas em cache do destino e relê do disco para detectar mentiras do cache de gravação e corrupção silenciosa. Cerca de 50% mais lento que a verificação padrão; desativado por padrão.
phase42-sharing-violation-retries-label = Tentativas em arquivos de origem bloqueados
phase42-sharing-violation-retries-hint = Quantas vezes tentar de novo quando outro processo mantém o arquivo de origem aberto com bloqueio exclusivo. A espera dobra a cada tentativa (50 ms / 100 ms / 200 ms por padrão). Padrão 3, igual ao Robocopy /R:3.
phase42-cloud-placeholder-warning = { $name } é um arquivo do OneDrive somente na nuvem. Copiá-lo acionará um download — até { $size } pela sua conexão de rede.
phase42-defender-exclusion-hint = Para a máxima taxa de cópia, adicione a pasta de destino às exclusões do Microsoft Defender antes de transferências em massa. Consulte docs/PERFORMANCE_TUNING.md.

# Phase 39 — Browser-accessible recovery UI. Settings → Advanced
# exposes these strings; the recovery server itself renders askama
# templates that always read English — these eight keys are the
# Settings prose the user actually reads in their preferred locale.
settings-recovery-heading = Interface web de recuperação
settings-recovery-enable = Ativar interface web de recuperação
settings-recovery-bind-address = Endereço de vinculação
settings-recovery-port = Porta (0 = escolher uma livre)
settings-recovery-show-url = Mostrar URL e token
settings-recovery-rotate-token = Rotacionar token
settings-recovery-allow-non-loopback = Permitir vinculação fora de loopback
settings-recovery-non-loopback-warning = AVISO: ativar uma vinculação fora de loopback expõe a interface de recuperação à sua rede local. Qualquer pessoa que descubra o token pode navegar pelo seu histórico de arquivos e baixá-los. Coloque TLS ou um proxy reverso à frente se a LAN não for confiável.

# Phase 40 — SMB compression negotiation + cloud-VM offload helper.
# 6 SMB keys (header badge + Settings prose) + 6 cloud-offload keys
# (Remotes tab wizard for cross-cloud copy templates).
smb-compress-badge = 🗜 Compactação SMB: { $algo }
smb-compress-badge-tooltip = O tráfego de rede para este destino está sendo compactado em trânsito (SMB 3.1.1).
smb-compress-toast-saved = { $bytes } economizados pela rede
smb-compress-algo-unknown = algoritmo desconhecido
settings-smb-compress-heading = Compactação de rede SMB
settings-smb-compress-hint = Negocia automaticamente a compactação de tráfego SMB 3.1.1 em destinos UNC. Ganho gratuito em links lentos; ignorado em destinos locais.
cloud-offload-heading = Auxiliar de offload de VM na nuvem
cloud-offload-hint = Ao copiar diretamente entre duas nuvens, gere um modelo de implantação que executa a cópia a partir de uma pequena VM efêmera na nuvem — os bytes nunca passam pela rede do seu laptop.
cloud-offload-render-button = Gerar modelo
cloud-offload-copy-clipboard = Copiar para a área de transferência
cloud-offload-template-format = Formato do modelo
cloud-offload-self-destruct-warning = A VM se desliga automaticamente após { $minutes } minutos — confirme a função IAM e a região antes de implantar.

# Phase 41 — animated before/after tree-diff preview. The `Preview
# changes` modal renders the rolled-up plan before the engine starts
# work; 14 keys cover the title, the summary header / counts, the
# row-reason labels, and the two action buttons.
preview-modal-title = Visualizar alterações
preview-summary-header = O que vai acontecer
preview-category-additions = { $count } adições
preview-category-replacements = { $count } substituições
preview-category-skips = { $count } ignorados
preview-category-conflicts = { $count } conflitos
preview-category-unchanged = { $count } inalterados
preview-bytes-to-transfer = { $bytes } a transferir
preview-reason-source-newer = A origem é mais recente
preview-reason-dest-newer = O destino é mais recente — será ignorado
preview-reason-content-different = O conteúdo difere
preview-reason-identical = Idêntico à origem
preview-button-run = Executar plano
preview-button-reduce = Reduzir meu plano…

# Phase 42 — perceptual-hash visual-similarity dedup. Eight keys cover
# the pre-copy "looks visually identical" warning + the Settings panel.
perceptual-warn-title = Parece visualmente idêntico
perceptual-warn-body = { $name } no destino parece corresponder à imagem de origem. Continuar copiando mesmo assim?
perceptual-warn-keep-both = Manter ambos
perceptual-warn-skip = Ignorar este arquivo
perceptual-warn-overwrite = Sobrescrever mesmo assim
perceptual-settings-heading = Dedup por similaridade visual
perceptual-settings-hint = Detecta imagens visualmente idênticas no destino antes de serem sobrescritas. O hash é perceptual (reconhece a mesma imagem salva novamente em outro formato), não byte a byte.
perceptual-settings-threshold-label = Limite de aviso (menor = correspondência mais rígida)

# Phase 42 Part B — per-file rolling versions (Time Machine for any
# destination). 8 keys cover the version-list panel + retention picker.
version-list-heading = Versões anteriores
version-list-empty = Nenhuma versão anterior deste arquivo
version-list-restore = Restaurar esta versão
version-retention-heading = Manter versões anteriores ao sobrescrever
version-retention-none = Manter todas as versões para sempre
version-retention-last-n = Manter as últimas { $n } versões
version-retention-older-than-days = Descartar versões com mais de { $days } dias
version-retention-gfs = A cada hora { $h } · diário { $d } · semanal { $w } · mensal { $m }

# Phase 43 — forensic chain-of-custody manifests + BLAKE3 verified
# streaming. 14 keys cover the Settings → Provenance panel
# (heading, hint, toggles, TSA URL, signing-key management) plus
# the post-job manifest notification + the verify command's two
# headline result lines.
provenance-settings-heading = Cadeia de custódia forense
provenance-settings-hint = Assina cada tarefa de cópia com um manifesto BLAKE3 + ed25519. Os revisores podem refazer o hash da árvore de destino depois e provar que nenhum byte mudou desde a cópia.
provenance-settings-enable-default = Assinar cada nova tarefa por padrão
provenance-settings-show-after-job = Mostrar o manifesto após cada tarefa concluída
provenance-settings-tsa-url-label = URL padrão da autoridade de carimbo de tempo RFC 3161
provenance-settings-tsa-url-hint = Opcional. Quando definido, os manifestos carregam um carimbo de tempo TSA gratuito provando que os bytes existiam neste momento. Deixe vazio para pular.
provenance-settings-keys-heading = Chaves de assinatura
provenance-settings-keys-generate = Gerar nova chave
provenance-settings-keys-import = Importar chave…
provenance-settings-keys-export = Exportar chave pública…
provenance-job-completed-title = Manifesto de proveniência salvo
provenance-job-completed-body = { $count } arquivos assinados → { $path }
provenance-verify-clean = Manifesto válido para { $count } arquivos; assinatura { $sig }; raiz merkle OK.
provenance-verify-tampered = Manifesto INVÁLIDO — { $tampered } adulterados, { $missing } ausentes.
# Phase 43 post-review hardening — toast text for the Settings →
# Provenance buttons whose Tauri IPC has not yet landed.
provenance-action-staged = Phase 43 — a conexão do IPC para esta ação chega em um commit de acompanhamento.

# Phase 44 — SSD-aware whole-drive sanitize (NVMe Sanitize / OPAL
# Crypto Erase) + the per-file shred refusal on copy-on-write
# filesystems. 16 keys cover the new "Drive sanitize" Settings
# subsection and the localized error messages.
sanitize-heading = Higienização segura de unidade inteira
sanitize-hint = NVMe Sanitize, OPAL Crypto Erase e ATA Secure Erase apagam uma unidade flash na camada de firmware em milissegundos. Sobrescrever por arquivo é inútil em flash — a trituração em várias passagens só desgasta a NAND. Use isto para uma purga real.
sanitize-pick-device = Escolha a unidade a higienizar
sanitize-mode-label = Método de higienização
sanitize-mode-nvme-format = NVMe Format (com secure erase)
sanitize-mode-nvme-sanitize-block = NVMe Sanitize — Block Erase (lento, cada célula)
sanitize-mode-nvme-sanitize-crypto = NVMe Sanitize — Crypto Erase (instantâneo)
sanitize-mode-ata-secure-erase = ATA Secure Erase (SSDs SATA legados)
sanitize-mode-opal-crypto-erase = TCG OPAL Crypto Erase (unidades autocriptografadas)
sanitize-mode-apfs-crypto-erase = APFS Crypto Erase (rotacionar a chave FileVault, apenas macOS)
sanitize-confirm-1 = Isto destrói CADA byte em { $device }. Não há como desfazer.
sanitize-confirm-2 = Entendo que todas as partições, todos os arquivos e todos os instantâneos em { $device } ficarão permanentemente ilegíveis.
sanitize-confirm-3 = Digite o nome do modelo da unidade para prosseguir: { $model }
sanitize-running = Higienizando { $device } ({ $mode }) — isto pode levar de milissegundos (crypto erase) a dezenas de minutos (block erase). Não desligue.
sanitize-completed = Higienização concluída — { $device } agora está em branco.
ssd-honest-shred-meaningless = A trituração por arquivo em um sistema de arquivos copy-on-write (Btrfs / ZFS / APFS) não consegue alcançar os blocos subjacentes. Use a higienização da unidade inteira junto com a rotação da chave de criptografia de disco inteiro.
ssd-honest-advisory = Este arquivo está em flash. Sobrescrever por arquivo desgasta a NAND e NÃO garante que as células originais sejam irrecuperáveis. Para dados sensíveis, higienize a unidade inteira.

# Phase 44.1f post-review — placeholder toast for SanitizeTab
# buttons whose Tauri IPC has not yet landed.
sanitize-action-staged = Phase 44.1 — a conexão do IPC para esta ação chega em um commit de acompanhamento.

# Phase 45.3 — named-queue tab strip (Subfeature A). Tabs surface
# once the QueueRegistry holds at least one queue; the synthesised
# default tab keeps legacy single-queue jobs reachable.
queue-tab-default = Padrão
queue-tab-empty-state = Filas de tarefas
queue-badge-tooltip = Tarefas pendentes e em execução nesta fila

# Phase 45.4 — drag-progress-merge (Subfeature B). Drag a queue tab
# onto another to merge their job lists. The default tab is neither
# draggable nor a drop target; only registry queues participate.
queue-drag-hint = Arraste sobre outra fila para mesclar
queue-merge-confirm = Solte para mesclar
queue-merge-toast = Filas mescladas

# Phase 45.5 — F2-queue UX (Subfeature C). F2 toggles
# `auto_enqueue_next` so every fresh enqueue piles into the running
# queue rather than spawning a parallel one. Status pill renders in
# the Footer; pulsing dot renders on the running tab.
queue-f2-active-hint = Modo F2: cada novo enfileiramento cai nesta fila
queue-f2-toggled-on = Modo de fila F2 ATIVADO — novos enfileiramentos entram na fila em execução
queue-f2-toggled-off = Modo de fila F2 DESATIVADO — novos enfileiramentos geram filas paralelas
queue-f2-status-bar = Modo de fila F2: ATIVADO

# Phase 45.6 — tray destination targets (Subfeature D). Pinned
# destinations appear in the OS tray menu; clicking one arms it as
# the active drop target so the next file drop bypasses the
# DropStagingDialog. Settings → General hosts the list editor.
tray-target-section-title = Destinos da bandeja
tray-target-section-hint = Os destinos fixados aparecem no menu da bandeja. Clique em um para armá-lo como o próximo destino de soltura.
tray-target-empty = Nenhum destino da bandeja fixado ainda.
tray-target-remove = Remover
tray-target-add-label = Rótulo
tray-target-add-path = Caminho ou URI de backend
tray-target-add = Adicionar
tray-target-armed-toast = Solte seu próximo arquivo para enviá-lo a { $label }
tray-target-active-pill = → { $label }

# Phase 45.7 follow-up — pinned-destination validation errors. The
# `queue_pin_destination` IPC returns these Fluent keys verbatim
# when input fails the IPC-boundary checks; the toast layer renders
# them via `t(...)` (Toast.svelte detects kebab-case-lowercase and
# routes through the locale table). Phase 17e's `err-path-escape` /
# `err-destination-empty` cover the path-traversal + empty cases for
# `queue_route_job`; the keys below are pin-specific.
err-pinned-destination-label-empty = O rótulo do destino da bandeja não pode ficar vazio.
err-pinned-destination-path-empty = O caminho do destino da bandeja não pode ficar vazio.
err-pinned-destination-label-too-long = O rótulo do destino da bandeja é longo demais (máx. 64 caracteres).
err-pinned-destination-path-too-long = O caminho do destino da bandeja é longo demais (máx. 1024 caracteres).
err-pinned-destination-label-invalid = O rótulo do destino da bandeja contém caracteres não permitidos (nova linha, retorno ou NUL).
err-pinned-destination-path-invalid = O caminho do destino da bandeja contém caracteres não permitidos (nova linha, retorno ou NUL).
err-pinned-destination-too-many = Você atingiu o limite de 50 destinos da bandeja. Remova um para adicionar outro.

# Phase 46.6 — Settings → Plugins tab. The IPC layer in
# `apps/copythat-ui/src-tauri/src/plugin_commands.rs` enumerates the
# per-user plugin store under `<config_dir>/plugins/`; the
# `PluginsTab.svelte` component renders these strings against the
# returned manifest + grant state.
settings-tab-plugins = Plugins
plugin-heading = Plugins
plugin-hint = Plugins WASM em sandbox estendem o Copy That com hooks personalizados. Cada plugin é executado sob limites de CPU e memória por chamada e só enxerga as capacidades do host que você conceder.
plugin-list-empty = Nenhum plugin instalado ainda.
plugin-enabled = Ativado
plugin-disabled = Desativado
plugin-hooks = Hooks
plugin-capabilities = Capacidades
plugin-no-capabilities = (nenhuma)
plugin-directory = Local
plugin-install-from-file = Instalar a partir de arquivo…
plugin-install-from-url = Instalar a partir de URL…
plugin-url-wasm = URL do WASM
plugin-url-manifest = URL do manifesto
plugin-url-hash = Hash BLAKE3
plugin-url-preview = Visualizar
plugin-url-confirm = Confirmar instalação

# Phase 31b — power-policy settings (Power tab).
settings-tab-power = Energia
settings-power-hint = Limite ou pause cópias conforme a energia: bateria, rede limitada/celular, apresentação/tela cheia ou limitação térmica da CPU.
settings-power-enabled = Ativar limitação por energia
settings-power-battery = Na bateria
settings-power-metered = Em rede limitada
settings-power-cellular = Em rede celular
settings-power-presentation = Durante apresentação
settings-power-fullscreen = Em tela cheia
settings-power-thermal = Em limitação térmica
settings-power-continue = Continuar
settings-power-pause = Pausar
err-server-not-implemented = O modo servidor ainda não está disponível.
err-webhook-not-implemented = A entrega de webhooks ainda não está disponível.

# Phase 47 — "why is this slow?" diagnostics (bottleneck badge + tooltip).
bottleneck-source-io = Origem I/O
bottleneck-dest-io = Destino I/O
bottleneck-network = Rede
bottleneck-antivirus = Antivírus
bottleneck-cpu = CPU
bottleneck-thermal = Térmico
bottleneck-unknown = Desconhecido
diag-aria = Gargalo: { $cause }
diag-tooltip = Limitado por { $cause } · { $rate }
diag-spark-aria = Taxa de transferência no último minuto
diag-keeping-up = Acompanhando
diag-label = Diagnóstico

# Phase 48 — server mode + observability (Settings → Server).
settings-tab-server = Servidor
server-hint = Execute o Copy That como um servidor de arquivos sem interface. Escolha os protocolos a expor, defina o endereço e a pasta a servir e, opcionalmente, exija autenticação.
server-protocols = Protocolos
server-bind-addr = Endereço de vínculo
server-root = Pasta servida
server-readonly = Somente leitura (recusar envios e exclusões)
server-auth-mode = Autenticação
server-auth-none = Nenhuma
server-auth-bearer = Token Bearer
server-auth-basic = Básica (usuário + senha)
server-auth-token = Token
server-auth-user = Usuário
server-auth-password = Senha
otel-endpoint = Endpoint do OpenTelemetry
webhook-section = Webhooks
webhook-url = URL do webhook
webhook-add = Adicionar webhook
webhook-remove = Remover
webhook-empty = Nenhum webhook configurado.
webhook-pushover-token = Token do Pushover
webhook-pushover-user = Usuário do Pushover
server-start = Iniciar servidor
server-stop = Parar servidor
server-status-running = Em execução em { $addr }
server-status-stopped = Parado
server-metrics-url = Métricas
err-server-no-protocols = Selecione pelo menos um protocolo antes de iniciar o servidor.
err-server-bind = Não foi possível vincular o endereço do servidor. Ele pode já estar em uso.

# Library drawer (Phase 49) — unified content-addressed repository view.
footer-library = Biblioteca
library-title = Biblioteca
library-loading = Carregando repositório…
library-unavailable = Repositório indisponível
library-tab-live = Ao vivo
library-tab-snapshots = Instantâneos
library-tab-versions = Versões
library-hero-savings = servindo { $effective } efetivos · { $pct } economizado
library-hero-empty = { $chunks } blocos armazenados — ainda sem instantâneos
library-stat-stored = Armazenado em disco
library-stat-effective = Dados efetivos
library-stat-snapshots = Instantâneos
library-stat-chunks = Blocos distintos
library-snapshot-empty = Ainda sem instantâneos
library-snapshot-files = { $n } arquivos
library-version-path-ph = Caminho de destino…
library-version-load = Mostrar versões
library-version-empty = Nenhuma versão para este caminho
repo-kind-copy = Cópia
repo-kind-sync = Sincronização
repo-kind-version = Versão
repo-kind-backup = Backup
# Phase 49o — snapshot diff / compare.
library-tab-compare = Comparar
repo-change-added = Adicionado
repo-change-removed = Removido
repo-change-modified = Modificado
repo-change-unchanged = Sem alterações
repo-diff-summary = { $added } adicionados · { $removed } removidos · { $modified } modificados
repo-diff-bytes-added = { $bytes } novos
repo-diff-pick-two = Escolha dois instantâneos para comparar
# Phase 49r — statistics / reports.
library-tab-reports = Relatórios
report-growth-title = Crescimento do armazenamento
report-by-kind-title = Por tipo
report-top-files-title = Principais arquivos
report-dedup-ratio = { $pct }% deduplicado
report-export = Exportar relatório
report-exported = Relatório salvo em { $path }
report-file-versions = { $n } versões
# Phase 49p — pinning / prune.
repo-pin = Fixar
repo-unpin = Desafixar
repo-pinned-badge = Fixado
repo-prune-title = Limpar
repo-prune-keep-last = Manter os mais recentes
repo-prune-removed = { $n } instantâneos removidos
repo-prune-none = Nada para limpar

# Phase 49c — fontes de backup.
library-tab-sources = Fontes
backup-add-source = Adicionar fonte…
backup-source-path-ph = Pasta para backup…
backup-exclude-ph = Globs de exclusão (separados por vírgula)
backup-now = Fazer backup agora
backup-remove = Remover
backup-empty = Ainda não há fontes de backup
backup-never-run = Nunca teve backup
backup-last-run = Último backup { $when }
backup-running = Fazendo backup… { $files } arquivos
backup-toast-started = Fazendo backup de { $label }…
backup-toast-completed = Backup de { $label } concluído: { $files } arquivos
backup-toast-failed = Falha no backup de { $label }: { $reason }
# Phase 49e — per-source retention + prune.
backup-retention = Retenção
backup-retention-keep-all = Manter tudo
backup-retention-last = Manter os últimos { $n }
backup-retention-days = Anteriores a { $days } dias
backup-retention-gfs = Rotação GFS
backup-prune-now = Limpar agora
backup-prune-none = Nada para limpar
backup-prune-result = { $removed } instantâneos removidos · { $bytes } liberados
# Phase 49f — per-source scheduling.
backup-schedule = Agendamento
backup-schedule-manual = Manual
backup-schedule-hourly = A cada hora
backup-schedule-daily = Diariamente
backup-schedule-weekly = Semanalmente
backup-next-run = Próxima execução { $when }
backup-not-scheduled = Não agendado
# Phase 49g — source filters.
backup-include-ph = Globs de inclusão (separados por vírgula)
backup-skip-hidden = Ignorar ocultos
# Phase 49q — notifications.
notify-title = Notificações
notify-on-success = Em caso de sucesso
notify-on-failure = Em caso de falha
notify-test = Enviar teste
notify-test-sent = Teste enviado para { $n } destino(s)

# Phase 49d — navegador de restauração.
restore-browse = Restaurar…
restore-title = Restaurar do instantâneo
restore-select-all = Selecionar tudo
restore-dest = Restaurar em
restore-confirm = Restaurar { $n } arquivos
restore-empty = Este instantâneo não tem arquivos
restore-conflict-body = { $count } arquivos selecionados já existem no destino.
restore-conflict-overwrite = Sobrescrever
restore-conflict-skip = Ignorar existentes
restore-conflict-keep-both = Manter ambos
restore-toast-done = Restaurados { $restored }, ignorados { $skipped }
restore-toast-failed = Falha na restauração: { $reason }
snapshot-forget = Esquecer
snapshot-forget-toast = Instantâneo esquecido — use Recuperar espaço para liberá-lo
library-reclaim = Recuperar espaço
# Phase 49i — full compaction.
library-compact = Compactação completa
library-compact-started = Compactação iniciada — acompanhe Tarefas
# Phase 49h — compression.
library-stat-compression = Economizado com compactação
storage-compression = Compactação
storage-compression-off = Desativada
storage-compression-auto = Automático (pular incompressíveis)
storage-compression-always = Sempre
storage-compression-restart = Aplica-se na próxima inicialização
# Phase 49j — tasks & progress center.
footer-tasks = Tarefas
tasks-title = Tarefas
tasks-empty = Ainda não há tarefas
tasks-running = Em execução
tasks-recent = Recentes
tasks-cancel = Cancelar
task-state-running = Em execução
task-state-completed = Concluído
task-state-failed = Falhou
task-state-cancelled = Cancelado
# Phase 49k — repository setup/connect wizard.
repo-wizard-title = Conectar repositório
repo-wizard-create-tab = Criar novo
repo-wizard-connect-tab = Conectar existente
repo-field-name = Nome
repo-field-path = Local
repo-field-password = Frase secreta (opcional)
repo-action-create = Criar
repo-action-connect = Conectar
repo-action-browse = Procurar…
repo-switcher-label = Repositório
repo-action-forget = Esquecer
repo-action-change-pass = Alterar frase secreta…
repo-password-old = Frase secreta atual
repo-password-new = Nova frase secreta
repo-error-exists = Já existe um repositório neste local
repo-error-not-found = Nenhum repositório encontrado neste local
repo-error-bad-pass = Frase secreta incorreta
repo-note-no-encryption = A frase secreta apenas controla o acesso; a criptografia em repouso chegará em uma versão futura
repo-confirm-forget = Remover "{ $name }" da lista? Seus dados permanecem no disco.
repo-toast-created = Repositório "{ $name }" criado
repo-toast-connected = Conectado a "{ $name }"
repo-toast-pass-changed = Frase secreta atualizada
# Phase 49l — Sources dashboard.
library-tab-overview = Visão geral
library-source-empty = Ainda não há origens
library-source-unknown = (origem não especificada)
library-source-snapshots = { $n } instantâneos
library-source-latest = Mais recente { $when }
# Phase 49n — verify & repair.
repo-action-verify = Verificar
repo-action-verify-deep = Verificar (ler todos os dados)
repo-action-repair = Reparar…
repo-verify-clean = { $files } arquivos / { $chunks } blocos verificados — sem danos
repo-verify-damaged = { $missing } ausentes, { $corrupt } blocos corrompidos
repo-repair-confirm = Remover { $n } instantâneos que não podem mais ser restaurados?
repo-repair-removed = { $n } instantâneos danificados removidos
repo-repair-none = Nada a reparar — o repositório está íntegro
repo-gc-done = Recuperado { $bytes } ({ $chunks } blocos)
restore-toast-partial = Restaurados { $restored }, ignorados { $skipped }, falharam { $failed }
