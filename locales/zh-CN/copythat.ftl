app-name = Copy That v0.19.84
window-title = Copy That v0.19.84
shred-ssd-advisory = 警告：此目标位于 SSD 上。多次覆写无法可靠清除闪存数据，因为磨损均衡和预留空间会将数据从逻辑块地址下移走。对于固态存储介质，请优先使用 ATA SECURE ERASE、带安全擦除的 NVMe Format，或使用全盘加密并丢弃密钥。

# Global aggregate states (header pill)
state-idle = 空闲
state-copying = 复制中
state-verifying = 校验中
state-paused = 已暂停
state-error = 错误

# Per-job states (row badge)
state-pending = 已排队
state-running = 运行中
state-cancelled = 已取消
state-succeeded = 已完成
state-failed = 失败

# Actions
action-pause = 暂停
action-resume = 继续
action-cancel = 取消
action-pause-all = 暂停所有任务
action-resume-all = 继续所有任务
action-cancel-all = 取消所有任务
action-close = 关闭
action-reveal = 在文件夹中显示
action-add-files = 添加文件
action-add-folders = 添加文件夹

# Phase 13d — activity feed
activity-title = 活动
activity-clear = 清空活动列表
activity-empty = 暂无文件活动。
activity-after-done = 完成后：
activity-keep-open = 保持应用打开
activity-close-app = 关闭应用
activity-shutdown = 关闭电脑
activity-logoff = 注销
activity-sleep = 睡眠

# Phase 14 — preflight free-space dialog
preflight-block-title = 目标位置空间不足
preflight-warn-title = 目标位置空间偏低
preflight-unknown-title = 无法确定可用空间
preflight-unknown-body = 源文件过大，无法快速估算大小，或目标卷未响应。你可以继续；如果空间耗尽，引擎的空间保护机制会干净地停止复制。
preflight-required = 所需
preflight-free = 可用
preflight-reserve = 预留
preflight-shortfall = 缺口
preflight-continue = 仍然继续
preflight-pick-subset = 选择要复制的内容…
collision-modal-overwrite-older = 仅覆盖较旧的

# Phase 14e — subset picker
subset-title = 选择要复制的源
subset-subtitle = 全部所选内容无法放入目标位置。勾选你想复制的项目，其余将保留不动。
subset-loading = 正在测量大小…
subset-too-large = 过大，无法统计
subset-budget = 可用
subset-remaining = 剩余
subset-confirm = 复制所选项
history-rerun-hint = 重新运行此次复制 — 会重新扫描源目录树中的每个文件
history-clear-all = 全部清除
history-clear-all-confirm = 再次点击以确认
history-clear-all-hint = 删除每一条历史记录。需要再次点击以确认。
toast-history-cleared = 历史记录已清空（移除了 { $count } 行）

# Phase 15 — source-list ordering in the drop staging dialog
drop-dialog-sort-label = 排序：
sort-custom = 自定义
sort-name-asc = 名称 A → Z（文件优先）
sort-name-desc = 名称 Z → A（文件优先）
sort-size-asc = 大小从小到大（文件优先）
sort-size-desc = 大小从大到小（文件优先）
sort-reorder = 重新排序
sort-move-top = 移到顶部
sort-move-up = 上移
sort-move-down = 下移
sort-move-bottom = 移到底部

# Phase 16 — sort preset names for the Activity list (shorter
# wording than the DropStagingDialog variants; the Activity list
# is files-only so the "(files first)" suffix isn't relevant).
sort-name-asc-simple = 名称 A → Z
sort-name-desc-simple = 名称 Z → A
sort-size-asc-simple = 大小从小到大
sort-size-desc-simple = 大小从大到小
activity-sort-locked = 复制进行时无法排序。请暂停或等待其完成，然后再更改顺序。

# Phase 17 — collision-policy picker in the DropStagingDialog
drop-dialog-collision-label = 如果文件已存在：
collision-policy-keep-both = 两者都保留（将新副本重命名为 _2、_3、…）
collision-policy-skip = 跳过新副本
collision-policy-overwrite = 覆盖已有文件
collision-policy-overwrite-if-newer = 仅当较新时覆盖
collision-policy-prompt = 每次询问

# Phase 18 — progress labels on the DropStagingDialog Start button
drop-dialog-busy-checking = 正在检查可用空间…
drop-dialog-busy-enumerating = 正在统计文件…
drop-dialog-busy-starting = 正在开始复制…
toast-enumeration-deferred = 源目录树较大 — 已跳过预先生成文件列表；引擎处理时会逐行显示。

# Context menu (per-row right-click)
menu-pause = 暂停
menu-resume = 继续
menu-cancel = 取消
menu-remove = 从队列中移除
menu-reveal-source = 在文件夹中显示源
menu-reveal-destination = 在文件夹中显示目标

# Header / toolbar
header-eta-label = 预计剩余时间
header-toolbar-label = 全局控制

# Footer
footer-queued = 个活动任务
footer-total-bytes = 传输中
footer-errors = 个错误
footer-history = 历史记录

# Empty state
empty-title = 拖入文件或文件夹以复制
empty-hint = 将项目拖到窗口上。我们会询问目标位置，然后为每个源排入一个任务。
empty-region-label = 任务列表

# Details drawer
details-drawer-label = 任务详情
details-source = 源
details-destination = 目标
details-state = 状态
details-bytes = 字节
details-files = 文件
details-speed = 速度
details-eta = 预计时间
details-error = 错误

# Drop dialog
drop-dialog-title = 传输拖入的项目
drop-dialog-subtitle = { $count } 个项目已准备好传输。请选择目标文件夹以开始。
drop-dialog-mode = 操作
drop-dialog-copy = 复制
drop-dialog-move = 移动
drop-dialog-pick-destination = 选择目标
drop-dialog-change-destination = 更改目标
drop-dialog-start-copy = 开始复制
drop-dialog-start-move = 开始移动

# ETA placeholders
eta-calculating = 计算中…
eta-unknown = 未知

# Toast messages
toast-job-done = 传输已完成
toast-copy-queued = 复制已排队
toast-move-queued = 移动已排队
toast-error-resolved = 错误已解决
toast-collision-resolved = 冲突已解决
toast-elevated-unavailable = 提权重试将在第 17 阶段推出 — 目前尚不可用
toast-clipboard-files-detected = 剪贴板上有文件 — 按下粘贴快捷键即可通过 Copy That 复制
toast-clipboard-no-files = 剪贴板中没有可粘贴的文件
toast-error-log-exported = 错误日志已导出

# Error modal (Phase 8)
error-modal-title = 一项传输失败
error-modal-retry = 重试
error-modal-retry-elevated = 以提升的权限重试
error-modal-skip = 跳过
error-modal-skip-all-kind = 跳过所有此类错误
error-modal-abort = 全部中止
error-modal-path-label = 路径
error-modal-code-label = 代码
error-drawer-pending-count = 还有更多错误待处理
error-drawer-toggle = 收起或展开

# Error-kind labels (Phase 8). Source of truth — engine maps each
# `CopyErrorKind` to one of these keys via `localized_key()`.
err-not-found = 找不到文件
err-permission-denied = 权限被拒绝
err-disk-full = 目标磁盘已满
err-interrupted = 操作被中断
err-verify-failed = 复制后校验失败
err-path-escape = 路径被拒绝 — 包含上级目录（..）片段或非法字节
err-path-invalid-encoding = 路径被拒绝 — 字符串包含无效的 UTF-8 / 替换字符
err-helper-invalid-json = 特权帮助程序收到格式错误的 JSON；已忽略此请求
err-helper-grant-out-of-band = GrantCapabilities 必须由帮助程序运行循环处理，而非无状态处理程序
err-randomness-unavailable = 操作系统随机数生成器失败；无法生成会话 ID
err-sparseness-mismatch = 无法在目标位置保留稀疏布局
err-io-other = 未知 I/O 错误

# Collision modal (Phase 8)
collision-modal-title = 文件已存在
collision-modal-overwrite = 覆盖
collision-modal-overwrite-if-newer = 较新则覆盖
collision-modal-skip = 跳过
collision-modal-keep-both = 两者都保留
collision-modal-rename = 重命名…
collision-modal-apply-to-all = 应用于全部
collision-modal-source = 源
collision-modal-destination = 目标
collision-modal-size = 大小
collision-modal-modified = 修改时间
collision-modal-hash-check = 快速哈希（SHA-256）
collision-modal-hash-computing = 正在计算…
collision-modal-hash-identical = 相同
collision-modal-hash-different = 不同
collision-modal-rename-placeholder = 新文件名
collision-modal-confirm-rename = 重命名

# Error log drawer (Phase 8)
error-log-title = 错误日志
error-log-empty = 没有记录的错误
error-log-export-csv = 导出 CSV
error-log-export-txt = 导出文本
error-log-clear = 清空日志
error-log-col-time = 时间
error-log-col-job = 任务
error-log-col-path = 路径
error-log-col-code = 代码
error-log-col-message = 消息
error-log-col-resolution = 处理方式

# History drawer (Phase 9)
history-title = 历史记录
history-empty = 尚无任务记录
history-unavailable = 复制历史不可用。应用在启动时无法打开 SQLite 存储。
history-filter-any = 任意
history-filter-kind = 类型
history-filter-status = 状态
history-filter-text = 搜索
history-refresh = 刷新
history-export-csv = 导出 CSV
history-purge-30 = 清除 30 天以前的
history-rerun = 重新运行
history-detail-open = 详情
history-detail-title = 任务详情
history-detail-empty = 没有记录的项目
history-col-date = 日期
history-col-kind = 类型
history-col-src = 源
history-col-dst = 目标
history-col-files = 文件
history-col-size = 大小
history-col-status = 状态
history-col-duration = 时长
history-col-error = 错误
toast-history-exported = 历史记录已导出
toast-history-rerun-queued = 重新运行已排队

# Totals drawer (Phase 10)
footer-totals = 总计
totals-title = 总计
totals-loading = 正在加载总计…
totals-card-bytes = 已复制字节总数
totals-card-files = 文件
totals-card-jobs = 任务
totals-card-avg-rate = 平均吞吐量
totals-errors = 个错误
totals-spark-title = 最近 30 天
totals-kinds-title = 按类型
totals-saved-title = 节省的时间（估算）
totals-saved-note = 相对于以基准文件管理器复制相同工作量的估算值。
totals-reset = 重置统计
totals-reset-confirm = 这将删除所有存储的任务和项目。是否继续？
totals-reset-confirm-yes = 是，重置
toast-totals-reset = 统计已重置

# Phase 11a — i18n core: surface remaining user-visible strings so
# every string on a main-window screen flows through Fluent.

# Header language switcher (temporary placement — folds into
# Settings → General in Phase 12).
header-language-label = 语言
header-language-title = 更改语言

# Job-kind labels. History filter, Totals breakdown, and history
# rows all map wire-format `kind` strings to these labels.
kind-copy = 复制
kind-move = 移动
kind-delete = 删除
kind-secure-delete = 安全删除

# History status labels. Distinct from `state-*` because the
# history wire format uses plain `running`/`succeeded`/... rather
# than the live `JobState` enum the queue exposes.
status-running = 运行中
status-succeeded = 成功
status-failed = 失败
status-cancelled = 已取消
# Per-item status (not job-level): `ok` / `skipped` are only
# reachable from the history detail view.
status-ok = 正常
status-skipped = 已跳过

# History drawer: search field placeholder + purge toast.
history-search-placeholder = /路径
toast-history-purged = 已清除 { $count } 个超过 30 天的任务

# User-facing command-layer validation errors. The Rust side
# returns these keys when input is missing; the toast layer looks
# them up rather than showing the raw English.
err-source-required = 至少需要一个源路径。
err-destination-empty = 目标路径为空。
err-source-empty = 源路径为空。

# Localised duration formatting for the Totals drawer and ETA
# fields. `{ $ms }`, `{ $s }`, `{ $m }`, `{ $h }` are integer
# placeables — the formatter passes pre-computed values in.
duration-lt-1s = < 1 秒
duration-ms = { $ms } 毫秒
duration-seconds = { $s } 秒
duration-minutes-seconds = { $m } 分 { $s } 秒
duration-hours-minutes = { $h } 小时 { $m } 分
duration-zero = 0 秒

# Rate unit. Appended to a formatted byte size. Some languages
# render this with a leading space ("Ko/s"); keep it translatable
# even though the SI-derived "/s" is near-universal.
rate-unit-per-second = { $size }/s

# Phase 11b — Settings modal skeleton. Phase 12 expanded this into a
# full six-tab preferences window; the `settings-phase-12-hint` key
# is retired in favour of `settings-tab-profiles` + concrete labels.
settings-title = 设置
settings-tab-general = 常规
settings-tab-appearance = 外观
settings-section-language = 语言
settings-phase-12-hint = 更多设置（主题、传输默认值、校验算法、配置文件）将在第 12 阶段推出。

# Phase 12 — full Settings window keys. Grouped by tab so the
# human-review pass can pick one section at a time. Every field
# label and every option label that the user will actually read
# has its own key; dynamic wire values (enum discriminants like
# "auto" / "prefer") stay untranslated on the wire.

settings-loading = 正在加载设置…
settings-tab-transfer = 传输
settings-tab-filters = 筛选
settings-tab-shell = Shell
settings-tab-secure-delete = 安全删除
settings-tab-advanced = 高级
settings-tab-updater = 更新
settings-tab-profiles = 配置文件

# General tab additions
settings-section-theme = 主题
settings-theme-auto = 自动
settings-theme-light = 浅色
settings-theme-dark = 深色
settings-start-with-os = 开机时启动
settings-single-instance = 单一运行实例
settings-minimize-to-tray = 关闭时最小化到托盘
settings-error-display-mode = 错误提示样式
settings-error-display-modal = 模态框（阻塞应用）
settings-error-display-drawer = 抽屉（非阻塞）
settings-error-display-mode-hint = 模态框会让队列暂停直到你做出决定。抽屉则让队列继续运行，并允许你在角落里逐一处理错误。
settings-paste-shortcut = 通过全局快捷键粘贴文件
settings-paste-shortcut-combo = 快捷键组合
settings-paste-shortcut-hint = 在系统任意位置按下此组合键，即可通过 Copy That 粘贴从 Explorer / Finder / Files 复制的文件。CmdOrCtrl 在 macOS 上解析为 Cmd，在 Windows / Linux 上解析为 Ctrl。
settings-clipboard-watcher = 监视剪贴板中复制的文件
settings-clipboard-watcher-hint = 当文件 URL 出现在剪贴板上时显示提示，提醒你可通过 Copy That 粘贴。启用时每 500 毫秒轮询一次。

# Transfer tab
settings-buffer-size = 缓冲区大小
settings-verify = 复制后校验
settings-verify-off = 关闭
settings-concurrency = 并发数
settings-concurrency-auto = 自动
settings-reflink = Reflink / 快速路径
settings-reflink-prefer = 优先使用
settings-reflink-avoid = 避免使用 reflink
settings-reflink-disabled = 始终使用异步引擎
settings-fsync-on-close = 关闭时同步到磁盘（更慢，更安全）
settings-preserve-timestamps = 保留时间戳
settings-preserve-permissions = 保留权限
settings-preserve-acls = 保留 ACL（第 14 阶段）
settings-preserve-sparseness = 保留稀疏文件
settings-preserve-sparseness-hint = 仅复制稀疏文件（虚拟机磁盘、数据库文件）已分配的区段，使目标在磁盘上的占用大小与源保持一致。
settings-force-parallel-chunks = 并行多块复制（仅限 RAID/阵列）
settings-force-parallel-chunks-hint = 将每个大文件复制拆分为并发块。仅对条带化/RAID/网络目标有益；会拖慢单个 SSD/NVMe（-25% 至 -76%）。除非目标是多磁盘阵列，否则请保持关闭。

# Shell tab
settings-context-menu = 启用 shell 右键菜单项
settings-intercept-copy = 拦截默认复制处理程序（Windows）
settings-intercept-copy-hint = 启用后，Explorer 的 Ctrl+C / Ctrl+V 会经由 Copy That 处理。注册功能将在第 14 阶段推出。
settings-notify-completion = 任务完成时通知

# Secure delete tab
settings-shred-method = 默认粉碎方法
settings-shred-zero = 清零（1 遍）
settings-shred-random = 随机（1 遍）
settings-shred-dod3 = DoD 5220.22-M（3 遍）
settings-shred-dod7 = DoD 5220.22-M（7 遍）
settings-shred-gutmann = Gutmann（35 遍）
settings-shred-nist = NIST 800-88
settings-shred-confirm-twice = 粉碎前需要双重确认

# Advanced tab
settings-log-level = 日志级别
settings-log-off = 关闭
settings-telemetry = 遥测
settings-telemetry-never = 永不 — 在任何日志级别都不回传
settings-error-policy = 默认错误策略
settings-error-policy-ask = 询问
settings-error-policy-skip = 跳过
settings-error-policy-retry = 退避后重试
settings-error-policy-abort = 首次失败即中止
settings-history-retention = 历史保留天数
settings-history-retention-hint = 0 = 永久保留。任何其他值都会在启动时自动清除较旧的任务。
settings-database-path = 数据库路径
settings-database-path-default = （默认 — 操作系统数据目录）
settings-reset-all = 重置为默认值
settings-reset-confirm = 将每项偏好设置重置为默认值？配置文件不受影响。

# Profiles tab
settings-profiles-hint = 将当前设置以某个名称保存；以后可加载它来快速切回，而无需逐项调整。
settings-profile-name-placeholder = 配置文件名称
settings-profile-save = 保存
settings-profile-import = 导入…
settings-profile-load = 加载
settings-profile-export = 导出…
settings-profile-delete = 删除
settings-profile-empty = 尚未保存任何配置文件。
settings-profile-import-prompt = 为导入的配置文件命名：

# Toasts driven by Phase 12 profile actions
toast-settings-reset = 设置已重置
toast-profile-saved = 配置文件已保存
toast-profile-loaded = 配置文件已加载
toast-profile-exported = 配置文件已导出
toast-profile-imported = 配置文件已导入

# Phase 14a — enumeration-time filters
settings-filters-hint = 在枚举阶段就跳过文件，使引擎根本不会打开它们。包含规则仅作用于文件；排除规则还会修剪匹配的目录。
settings-filters-enabled = 为目录树复制启用筛选
settings-filters-include-globs = 包含通配符
settings-filters-include-globs-placeholder = **/*.txt
settings-filters-include-globs-hint = 每行一个通配符。非空时，文件必须匹配至少一个包含规则才会保留。目录始终会被递归进入。
settings-filters-exclude-globs = 排除通配符
settings-filters-exclude-globs-placeholder = **/node_modules
settings-filters-exclude-globs-hint = 每行一个通配符。对目录而言，匹配会修剪整个子树；匹配的文件会被跳过。
settings-filters-size-range = 文件大小范围
settings-filters-min-size-bytes = 最小大小（字节，留空 = 无下限）
settings-filters-max-size-bytes = 最大大小（字节，留空 = 无上限）
settings-filters-date-range = 修改时间范围
settings-filters-min-mtime = 修改时间不早于
settings-filters-max-mtime = 修改时间不晚于
settings-filters-attributes = 属性位
settings-filters-skip-hidden = 跳过隐藏的文件 / 文件夹
settings-filters-skip-system = 跳过系统文件（仅限 Windows）
settings-filters-skip-readonly = 跳过只读文件

# Phase 15 — auto-update
settings-updater-hint = Copy That 最多每天检查一次签名更新。更新会在下次退出应用时安装。
settings-updater-auto-check = 启动时检查更新
settings-updater-channel = 发布渠道
settings-updater-channel-stable = 稳定版
settings-updater-channel-beta = Beta（预发布）
settings-updater-last-check = 上次检查
settings-updater-last-never = 从未
settings-updater-check-now = 立即检查更新
settings-updater-checking = 正在检查…
settings-updater-available = 有可用更新
settings-updater-up-to-date = 你正在运行最新版本。
settings-updater-dismiss = 跳过此版本
settings-updater-dismissed = 已跳过
toast-update-available = 有更新的版本可用
toast-update-up-to-date = 你已是最新版本

# Phase 19a — disk-backed file enumeration (TeraCopy-compatible scan DB)
scan-progress-title = 正在扫描…
scan-progress-stats = { $files } 个文件 · 已处理 { $bytes }
scan-pause-button = 暂停扫描
scan-resume-button = 继续扫描
scan-cancel-button = 取消扫描
scan-cancel-confirm = 取消扫描并丢弃进度？
scan-db-header = 扫描数据库
scan-db-hint = 用于数百万文件任务的磁盘扫描数据库。
advanced-scan-hash-during = 扫描时计算校验和
advanced-scan-db-path = 扫描数据库位置
advanced-scan-retention-days = 自动删除已完成扫描的天数
advanced-scan-max-keep = 保留的扫描数据库数量上限

# Phase 19b — filesystem-snapshot source for locked files.
settings-on-locked = 当文件被锁定时
settings-on-locked-ask = 首次询问
settings-on-locked-retry = 短暂重试，然后报出错误
settings-on-locked-skip = 跳过被锁定的文件
settings-on-locked-snapshot = 使用文件系统快照
settings-on-locked-hint = 消除“文件正被其他进程使用”的错误。Copy That 会对源卷拍摄快照（Windows 上为 VSS，Linux 上为 ZFS/Btrfs，macOS 上为 APFS），并从快照副本读取。
snapshot-prompt-title = 此文件正被另一个进程使用
snapshot-prompt-body = 另一个程序已以独占写入方式打开 { $path }。请选择 Copy That 应如何处理此文件以及同一卷上的类似文件。
snapshot-source-active = 📷 正在从 { $volume } 的 { $kind } 快照读取
snapshot-create-failed = 无法为源卷创建快照
snapshot-vss-needs-elevation = 从 VSS 快照读取需要管理员权限。Copy That 将请求你的授权。
snapshot-cleanup-failed = 快照帮助程序报告了清理失败 — 卷上可能残留了多余的卷影副本。

# Phase 20 — durable resume journal.
resume-prompt-title = 恢复之前的传输？
resume-prompt-body = Copy That 检测到上一会话中有 { $count } 项未完成的传输。请选择对每一项的处理方式。
resume-prompt-resume = 恢复
resume-prompt-resume-all = 全部恢复
resume-discard-one = 不恢复
resume-discard-all = 全部丢弃
resume-aborted-hash-mismatch = 目标的前 { $offset } 字节与源不匹配 — 将从头开始。
settings-auto-resume = 无需提示，自动恢复被中断的任务
settings-auto-resume-hint = 跳过启动时的恢复提示，静默地重新排入每一项未完成的任务。默认关闭。

# Phase 21 — bandwidth shaping (GCRA token bucket + schedule + auto-throttle).
settings-tab-network = 网络
settings-network-hint = 限制你的传输速率，让网络其余部分保持可用。可全局应用、按每日计划执行，或对计费 Wi-Fi / 电池 / 蜂窝连接自动作出反应。
settings-network-mode = 带宽限制
settings-network-mode-off = 关闭（不限制）
settings-network-mode-fixed = 固定值
settings-network-mode-schedule = 使用计划
settings-network-cap-mbps = 上限（MB/s）
settings-network-schedule = 计划（rclone 格式）
settings-network-schedule-hint = 以空白分隔的 HH:MM,rate 边界，外加可选的 Mon-Fri,rate 日规则。速率：512k、10M、2G、off、unlimited。示例：08:00,512k 18:00,10M Sat-Sun,unlimited。
settings-network-auto-header = 自动限速
settings-network-auto-metered = 在计费 Wi-Fi 上
settings-network-auto-battery = 在电池供电时
settings-network-auto-cellular = 在蜂窝网络上
settings-network-auto-unchanged = 不覆盖
settings-network-auto-pause = 暂停传输
settings-network-auto-cap = 限制为固定值
shape-badge-paused = 已暂停
shape-badge-tooltip = 带宽限制已生效 — 点击以打开 设置 → 网络
shape-badge-source-schedule = 按计划
shape-badge-source-metered = 计费网络
shape-badge-source-battery = 电池供电
shape-badge-source-cellular = 蜂窝网络
shape-badge-source-settings = 已生效
shape-error-schedule-invalid = 计划格式无效：{ $message }

# Phase 22 — aggregate conflict dialog v2 (thumbnails, per-pattern
# rules, and reusable conflict profiles). Every key below is user-
# visible text in the `ConflictBatchModal.svelte` component.
conflict-batch-title = { $jobname } 中有 { $count } 个文件冲突
conflict-batch-state-pending = 待处理
conflict-batch-state-resolved = 已解决
conflict-batch-action-overwrite = 覆盖
conflict-batch-action-skip = 跳过
conflict-batch-action-keep-both = 两者都保留
conflict-batch-action-newer-wins = 较新者优先
conflict-batch-action-larger-wins = 较大者优先
conflict-batch-bulk-apply-selected = 应用于所选项
conflict-batch-bulk-apply-extension = 应用于此扩展名的全部
conflict-batch-bulk-apply-glob = 应用于匹配通配符…
conflict-batch-bulk-apply-remaining = 应用于所有剩余项
conflict-batch-bulk-glob-placeholder = 例如 **/*.tmp
conflict-batch-save-profile = 将这些规则保存为配置文件…
conflict-batch-profile-placeholder = 配置文件名称
conflict-batch-matched-rule = 通过规则“{ $rule }”→ { $action }
conflict-batch-empty = 所有冲突已解决
conflict-batch-source-vs-destination = 源 与 目标对比
conflict-batch-source-label = 源
conflict-batch-destination-label = 目标
conflict-batch-size-label = 大小
conflict-batch-modified-label = 修改时间
conflict-batch-close = 关闭
conflict-batch-profile-saved = 冲突配置文件已保存

# Phase 23 — sparse-file preservation. The toast fires once per
# destination volume when the filesystem can't preserve holes; the
# warning line is surfaced in the job detail drawer so the user knows
# the dst is larger on disk than the source was.
sparse-not-supported-title = 目标会填充稀疏文件
sparse-not-supported-body = { $dst_fs } 不支持稀疏文件。源中的空洞被写为零，因此目标在磁盘上占用更大。
sparse-warning-densified = 已保留稀疏布局：仅复制了已分配的区段。
sparse-warning-mismatch = 稀疏布局不匹配 — 目标在磁盘上的占用可能比预期更大。

# Phase 24 — security-metadata preservation. The Mark-of-the-Web
# (Zone.Identifier ADS) toggle is security-sensitive: turning it off
# lets a downloaded executable shed its SmartScreen / Office Protected
# View flag on copy, which is why the tooltip carries an explicit
# warning. AppleDouble fallback emits `._<filename>` sidecars on
# destination filesystems that can't hold the foreign metadata.
settings-preserve-security-metadata = 保留安全元数据
settings-preserve-security-metadata-hint = 在每次复制时捕获并重新应用带外元数据流（NTFS ADS / xattr / POSIX ACL / SELinux 上下文 / Linux 文件能力 / macOS 资源分支）。
settings-preserve-motw = 保留 Mark-of-the-Web（从互联网下载标记）
settings-preserve-motw-hint = 对安全至关重要。SmartScreen 和 Office 受保护的视图使用此数据流来警示从互联网下载的文件。禁用后，下载的可执行文件可在复制时丢弃其来源标记，从而绕过操作系统的防护措施。
settings-preserve-posix-acls = 保留 POSIX ACL 和扩展属性
settings-preserve-posix-acls-hint = 在复制过程中携带 user.* / system.* / trusted.* xattr 以及 POSIX 访问控制列表。
settings-preserve-selinux = 保留 SELinux 上下文
settings-preserve-selinux-hint = 在复制过程中携带 security.selinux 标签，使在 MAC 策略下运行的守护进程仍能访问该文件。
settings-preserve-resource-forks = 保留 macOS 资源分支和 Finder 信息
settings-preserve-resource-forks-hint = 在复制过程中携带旧式资源分支和 FinderInfo（颜色标签、Carbon 元数据）。
settings-appledouble-fallback = 在不兼容的文件系统上使用 AppleDouble 附属文件
meta-translated-to-appledouble = 外来元数据已存入 AppleDouble 附属文件（._{ $ext }）

# Phase 25 — two-way sync with vector-clock conflict detection.
# The drawer lists configured sync pairs; each pair runs an
# independent reconciliation round against a per-pair `.copythat-sync.db`
# state store. Concurrent edits from a common ancestor surface as
# conflicts rather than silent overwrites; the losing side's content
# is preserved as `name.sync-conflict-YYYYMMDD-HHMMSS-<host>.ext`.
footer-sync = 同步
sync-drawer-title = 双向同步
sync-drawer-hint = 让两个文件夹保持同步，不会静默覆盖。并发编辑会以冲突形式呈现，供你解决。
sync-add-pair = 添加配对
sync-add-cancel = 取消
sync-refresh = 刷新
sync-add-save = 保存配对
sync-add-saving = 正在保存…
sync-add-missing-fields = 标签、左侧路径和右侧路径均为必填项。
sync-remove-confirm = 移除此同步配对？状态数据库会保留；文件夹不受影响。
sync-field-label = 标签
sync-field-label-placeholder = 例如 文档 ↔ NAS
sync-field-left = 左侧文件夹
sync-field-left-placeholder = 选择或粘贴绝对路径
sync-field-right = 右侧文件夹
sync-field-right-placeholder = 选择或粘贴绝对路径
sync-field-mode = 模式
sync-mode-two-way = 双向
sync-mode-mirror-left-to-right = 镜像（左 → 右）
sync-mode-mirror-right-to-left = 镜像（右 → 左）
sync-mode-contribute-left-to-right = 贡献（左 → 右，不删除）
sync-no-pairs = 尚未配置同步配对。点击“添加配对”开始。
sync-loading = 正在加载已配置的配对…
sync-never-run = 从未运行
sync-running = 运行中
sync-run-now = 立即运行
sync-cancel = 取消
sync-remove-pair = 移除
sync-view-conflicts = 查看冲突（{ $count }）
sync-conflicts-heading = 冲突
sync-no-conflicts = 上次运行没有冲突。
sync-winner = 优胜方
sync-side-left-to-right = 左侧
sync-side-right-to-left = 右侧
sync-conflict-kind-concurrent-write = 并发编辑
sync-conflict-kind-delete-edit = 删除 ↔ 编辑
sync-conflict-kind-add-add = 两侧都新增
sync-conflict-kind-corrupt-equal = 内容在无新写入的情况下发生分歧
sync-resolve-keep-left = 保留左侧
sync-resolve-keep-right = 保留右侧
sync-resolve-keep-both = 两者都保留
sync-resolve-three-way = 通过三方合并解决
sync-resolve-phase-53-tooltip = 针对非文本文件的交互式三方合并将在第 53 阶段推出。
sync-error-prefix = 同步错误

# Phase 26 — real-time mirror watcher. "Live mirror" starts a
# filesystem watcher on the pair's left side; every debounced event
# triggers a re-sync. The watcher filters vim swap files / Office
# lock files / atomic-save temp names so one logical save becomes
# exactly one sync round.
live-mirror-start = 启动实时镜像
live-mirror-stop = 停止实时镜像
live-mirror-watching = 监视中
live-mirror-toggle-hint = 在每次检测到文件系统更改时自动重新同步。每个活动配对一个后台线程。
watch-event-prefix = 文件更改
watch-overflow-recovered = 监视缓冲区溢出；正在重新枚举以恢复

# Phase 27 — content-defined chunk store. Enables delta-resume (a
# retry only re-writes chunks that actually changed) and same-job
# dedup (files sharing content blocks store those blocks once). The
# store is disk-backed under `<data-dir>/chunks/` by default and is
# the foundation for the Phase 49–51 moonshot repository phases.
chunk-store-section = 块存储
chunk-store-enable = 启用块存储（增量恢复和去重）
chunk-store-enable-hint = 按内容（FastCDC）拆分每个复制的文件，并以内容寻址方式存储块。重试时只重写更改的块；内容相同的文件会自动去重。
chunk-store-location = 块存储位置
chunk-store-max-size = 块存储最大大小
chunk-store-prune = 清除超过指定天数的块
chunk-store-savings = 通过块去重节省了 { $gib } GiB
chunk-store-disk-usage = 在 { $chunks } 个块中占用 { $size }

# Phase 28 — tray-resident Drop Stack. The stack is a persistent list
# of paths gathered from multiple sources (Explorer drag, main-window
# context menu, CLI, drag onto the Drop Stack window) that the user
# can dispatch to a destination in one go.
dropstack-window-title = Drop Stack
dropstack-tray-open = Drop Stack
dropstack-empty-title = Drop Stack 为空
dropstack-empty-hint = 从 Explorer 拖入文件，或右键点击任务行将其添加。
dropstack-add-to-stack = 添加到 Drop Stack
dropstack-copy-all-to = 全部复制到…
dropstack-move-all-to = 全部移动到…
dropstack-clear = 清空堆栈
dropstack-remove-row = 从堆栈中移除
dropstack-path-missing-toast = 已拖入 { $path } — 该文件已不存在。
dropstack-always-on-top = 让 Drop Stack 始终置顶
dropstack-show-tray-icon = 显示 Copy That 托盘图标
dropstack-open-on-start = 应用启动时自动打开 Drop Stack
dropstack-count = { $count } 个路径

# Phase 29 — spring-loaded folders + native DnD polish. The Settings
# → General tab carries the knobs; DropTarget / DestinationPicker
# Svelte components use the dropzone-* keys at runtime.
settings-dnd-heading = 拖放
settings-dnd-spring-load = 拖动时弹簧式展开文件夹
settings-dnd-spring-delay = 弹簧式展开延迟（毫秒）
settings-dnd-thumbnails = 显示拖动缩略图
settings-dnd-invalid-highlight = 高亮无效的放置目标
dropzone-invalid-title = 不是有效的放置目标
dropzone-invalid-readonly = 目标为只读
dropzone-picker-title = 选择目标
dropzone-picker-up = 上一级
dropzone-picker-path = 当前路径
dropzone-picker-root = 根目录
dropzone-picker-use-this = 使用此文件夹
dropzone-picker-empty = 没有子文件夹
dropzone-picker-cancel = 取消

# Phase 30 — cross-platform path translation. Settings → Transfer
# exposes these under a "Cross-platform compatibility" subsection.
translate-heading = 跨平台兼容性
translate-unicode-label = Unicode 规范化
translate-unicode-auto = 自动检测目标
translate-unicode-windows = NFC（Windows / Linux）
translate-unicode-macos = 保持原样（macOS / APFS）
translate-line-endings-label = 为文本文件转换换行符
translate-line-endings-allowlist = 文本文件扩展名
reserved-name-label = Windows 保留名称处理
reserved-name-suffix = 追加“_”（CON.txt → CON_.txt）
reserved-name-reject = 拒绝并警告
long-path-label = 超过 260 个字符时使用 Windows 长路径前缀（\\?\）
long-path-hint = 某些网络共享和旧工具不支持 \\?\ 命名空间。

# Phase 31 — power-aware copying. Settings → Power & State tab + the
# header badge that renders "⏸ Paused — Zoom call detected" when the
# runner's power subscriber has paused or capped due to a policy match.
power-heading = 电源与状态
power-enabled = 启用电源感知规则
power-battery-label = 电池供电时
power-metered-label = 计费 Wi-Fi 时
power-cellular-label = 蜂窝网络时
power-presentation-label = 演示时（Zoom / Teams / Keynote）
power-fullscreen-label = 有应用全屏时
power-thermal-label = CPU 因过热降频时
power-rule-continue = 以全速继续
power-rule-pause = 暂停所有任务
power-rule-cap = 限制带宽
power-rule-cap-percent = 限制为当前速率的百分比
power-reason-on-battery = 电池供电
power-reason-metered-network = 计费网络
power-reason-cellular-network = 蜂窝网络
power-reason-presenting = 演示模式
power-reason-fullscreen = 全屏应用
power-reason-thermal-throttling = CPU 正在降频

# Phase 32 — cloud backend matrix via OpenDAL. Settings → Remotes
# tab + the Add-backend wizard that writes one entry per remote into
# the keychain-backed credential store.
remote-heading = 远程后端
remote-add = 添加后端
remote-list-empty = 未配置远程后端
remote-test = 测试连接
remote-test-success = 连接成功
remote-test-failed = 连接失败
remote-remove = 移除后端
remote-name-label = 显示名称
remote-kind-label = 后端类型
remote-save = 保存后端
remote-cancel = 取消
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
backend-local-fs = 本地文件系统
cloud-config-bucket = 存储桶
cloud-config-region = 区域
cloud-config-endpoint = 端点 URL
cloud-config-root = 根路径
cloud-error-invalid-config = 后端配置无效
cloud-error-network = 连接后端时发生网络错误
cloud-error-not-found = 在请求的路径上找不到对象
cloud-error-permission = 远程后端拒绝了权限
cloud-error-keychain = 操作系统密钥串访问失败
settings-tab-remotes = 远程
settings-tab-mobile = 移动端

# Phase 33 — mount Copy That's chunk store + history archive as a
# read-only filesystem (FUSE / WinFsp). Surfaced by the History tab
# context menu's "Mount snapshot" action + the Settings → Advanced
# "Mount latest on launch" toggle.
mount-heading = 挂载快照
mount-action-mount = 挂载快照
mount-action-unmount = 卸载
mount-status-mounted = 已挂载于 { $path }
mount-error-unsafe-mountpoint = 挂载点路径不安全
mount-error-mountpoint-not-empty = 挂载点必须是空目录
mount-error-backend-unavailable = 此系统上挂载后端不可用
mount-error-archive-read = 归档读取失败
mount-picker-title = 选择挂载点目录
mount-toast-mounted = 快照已挂载于 { $path }
mount-toast-unmounted = 快照已卸载
mount-toast-failed = 挂载失败：{ $reason }
settings-mount-heading = 挂载快照
settings-mount-hint = 将历史归档作为只读文件系统公开。第 33b 阶段接入运行流程；内核 FUSE/WinFsp 后端将在第 33c 阶段推出。
settings-mount-on-launch = 启动时挂载最新快照
settings-mount-on-launch-path = 挂载点路径
settings-mount-on-launch-path-placeholder = 例如 C:\Mounts\copythat

# Phase 34 — enterprise-grade audit log export + WORM (write-once-read-
# many) tamper-resistance. Settings → Advanced → Audit log collects the
# format + destination + rotation + WORM toggle; runtime records job and
# file events into the tamper-evident chain-hashed sink.
settings-audit-heading = 审计日志
settings-audit-hint = 记录每个任务和文件事件的仅追加防篡改日志。格式包括 CSV、JSON-lines、RFC 5424 Syslog、ArcSight CEF 和 QRadar LEEF。
settings-audit-enable = 启用审计日志
settings-audit-format = 日志格式
settings-audit-format-json-lines = JSON lines（推荐默认）
settings-audit-format-csv = CSV（便于电子表格使用）
settings-audit-format-syslog = Syslog（RFC 5424）
settings-audit-format-cef = CEF（ArcSight）
settings-audit-format-leef = LEEF 2.0（IBM QRadar）
settings-audit-file-path = 日志文件路径
settings-audit-file-path-placeholder = 例如 C:\ProgramData\CopyThat\audit.log
settings-audit-max-size = 达到此大小后轮转（字节，0 = 永不）
settings-audit-worm = 启用 WORM 模式（一次写入多次读取）
settings-audit-worm-hint = 在每次创建或轮转后应用平台的仅追加标志（Linux chattr +a、macOS chflags uappnd、Windows 只读属性）。即使是管理员也必须明确清除该标志才能截断日志。
settings-audit-test-write = 测试写入
settings-audit-verify-chain = 校验链
toast-audit-test-write-ok = 审计日志测试写入成功
toast-audit-verify-ok = 审计链校验完整无误
toast-audit-verify-failed = 审计链校验报告了不匹配

# Phase 35 — destination encryption via age (X25519 / passphrase /
# SSH) + on-the-fly zstd compression with a per-extension deny
# heuristic. Surfaced through Settings → Transfer → Encryption +
# Compression; engine short-circuits to the crypt pipeline when
# either stage is active.
settings-crypt-heading = 加密与压缩
settings-crypt-hint = 在文件内容到达目标之前对其进行转换。加密使用 age 格式；压缩使用 zstd，并可按扩展名跳过已压缩的媒体。
settings-crypt-encryption-mode = 加密
settings-crypt-encryption-off = 关闭
settings-crypt-encryption-passphrase = 口令（在复制开始时提示）
settings-crypt-encryption-recipients = 来自文件的收件人密钥
settings-crypt-encryption-hint = 口令仅在复制期间保存于内存中。收件人文件每行列出一个 age1… 或 ssh- 公钥。
settings-crypt-recipients-file = 收件人文件路径
settings-crypt-recipients-file-placeholder = 例如 C:\Users\me\recipients.txt
settings-crypt-compression-mode = 压缩
settings-crypt-compression-off = 关闭
settings-crypt-compression-always = 始终
settings-crypt-compression-smart = 智能（跳过已压缩的媒体）
settings-crypt-compression-hint = 智能模式会跳过 jpg、mp4、zip、7z 等无法从 zstd 中获益的格式。始终模式会以所选级别压缩每个文件。
settings-crypt-compression-level = zstd 级别（1-22）
settings-crypt-compression-level-hint = 数字越小越快；数字越大压缩越强。级别 3 与 zstd 的 CLI 默认值相同。
compress-footer-savings = 💾 { $original } → { $compressed }（节省 { $percent }%）
compress-savings-toast = 已压缩 { $percent }%（节省 { $bytes }）
crypt-toast-recipients-loaded = 已加载 { $count } 个加密收件人
crypt-toast-recipients-error = 加载收件人失败：{ $reason }
crypt-toast-passphrase-required = 加密需要在复制开始前提供口令
crypt-toast-passphrase-set = 已捕获加密口令
crypt-footer-encrypted-badge = 🔒 已加密（age）
crypt-footer-compressed-badge = 📦 已压缩（zstd）

# Phase 36 — copythat CLI surface. Documented exit codes + the
# user-facing strings for plan/apply/verify/config. Help text stays
# in English (engineering accessibility) but error / info / status
# strings are localized so a localized desktop installation surfaces
# CLI errors in the same language as the GUI.
cli-help-tagline = Copy That CLI — 面向 CI/CD 流水线的字节精确文件复制、同步、校验与审计。
cli-help-exit-codes = 退出代码：0 成功，1 错误，2 待处理，3 冲突，4 校验失败，5 网络，6 权限，7 磁盘已满，8 取消，9 配置。
cli-error-bad-args = copy/move 至少需要一个源和一个目标
cli-error-unknown-algo = 未知的校验算法：{ $algo }
cli-error-missing-spec = plan/apply 需要 --spec
cli-error-spec-parse = 解析 jobspec { $path } 失败：{ $reason }
cli-error-spec-empty-sources = Jobspec 的源列表为空
cli-info-shape-recorded = 带宽整形“{ $rate }”已记录；强制执行通过 copythat-shape 接入
cli-info-stub-deferred = { $command } 已为第 36 阶段的后续接入做好准备
cli-plan-summary = 计划：{ $actions } 项操作，{ $bytes } 字节；{ $already_done } 项已就位
cli-plan-pending = 计划报告有待处理操作；使用 `apply` 重新运行以执行
cli-plan-already-done = 计划报告无需执行任何操作（幂等）
cli-apply-success = 应用完成，无错误
cli-apply-failed = 应用完成，但存在一个或多个错误
cli-verify-ok = 校验通过：{ $algo } { $digest }
cli-verify-failed = { $path } 校验失败（{ $algo }）
cli-config-set = 已设置 { $key } = { $value }
cli-config-reset = 已将 { $key } 重置为默认值
cli-config-unknown-key = 未知的配置键：{ $key }
cli-completions-emitted = { $shell } 的 shell 补全已打印到 stdout

# Phase 37 — desktop-side mobile companion. Settings → Mobile panel
# strings + the SAS-confirmation modal + push-notification toasts.
# The actual mobile UI lives in the Phase 37 follow-up Tauri Mobile
# target; the phone displays its own localized SAS prompt.
settings-mobile-heading = 移动端伴侣
settings-mobile-hint = 配对 iPhone 或 Android 手机，以浏览历史、启动已保存的配置文件和第 36 阶段的 jobspec，并接收完成通知。
settings-mobile-pair-toggle = 允许新配对
settings-mobile-pair-active = 配对服务器已激活 — 用 Copy That 移动应用扫描二维码
settings-mobile-pair-button = 开始配对
settings-mobile-revoke-button = 撤销
settings-mobile-no-pairings = 尚无已配对设备
settings-mobile-pair-port = 绑定端口（0 = 自动选取空闲端口）
pair-sas-prompt = 两个屏幕应显示相同的四个表情符号。若一致，请点击“匹配”。
pair-sas-confirm = 匹配
pair-sas-reject = 不匹配 — 取消
pair-toast-success = 已与 { $device } 配对
pair-toast-failed = 配对失败：{ $reason }
push-toast-sent = 推送已发送至 { $device }
push-toast-failed = 向 { $device } 推送失败：{ $reason }

# Phase 38 — aggregate destination dedup + reflink fallback ladder.
# Settings → Transfer → Dedup panel + per-job-row badges (⚡
# Reflinked / 🔗 Hardlinked / 🧩 Chunk-shared / 📋 Copied) +
# pre-pass dedup-scan modal.
settings-dedup-heading = 目标去重
settings-dedup-hint = 当源与目标共享同一卷时，Copy That 可以在文件系统级别克隆文件，而无需复制字节。Reflink 即时且安全；hardlink 更快，但两个名称会共享同一状态。
settings-dedup-mode-auto = 自动阶梯（reflink → hardlink → 块 → 复制）
settings-dedup-mode-reflink-only = 仅 reflink
settings-dedup-mode-hardlink-aggressive = 激进（即使是可写文件也使用 reflink + hardlink）
settings-dedup-mode-off = 禁用（始终按字节复制）
settings-dedup-hardlink-policy = Hardlink 策略
settings-dedup-prescan = 预先扫描目标目录树中的重复内容
dedup-badge-reflinked = ⚡ 已 Reflink
dedup-badge-hardlinked = 🔗 已硬链接
dedup-badge-chunk-shared = 🧩 块共享
dedup-badge-copied = 📋 已复制
phase42-paranoid-verify-label = 偏执校验
phase42-paranoid-verify-hint = 丢弃目标的缓存页并从磁盘重新读取，以捕获写缓存谎报和静默损坏。比默认校验慢约 50%；默认关闭。
phase42-sharing-violation-retries-label = 对被锁定源文件的重试次数
phase42-sharing-violation-retries-hint = 当另一个进程以独占锁持有源文件打开状态时的重试次数。退避时间每次翻倍（默认 50 毫秒 / 100 毫秒 / 200 毫秒）。默认 3 次，与 Robocopy /R:3 相同。
phase42-cloud-placeholder-warning = { $name } 是仅云端的 OneDrive 文件。复制它将触发下载 — 通过你的网络连接最多 { $size }。
phase42-defender-exclusion-hint = 为获得最大复制吞吐量，请在批量传输前将目标文件夹添加到 Microsoft Defender 排除项。参见 docs/PERFORMANCE_TUNING.md。

# Phase 39 — Browser-accessible recovery UI. Settings → Advanced
# exposes these strings; the recovery server itself renders askama
# templates that always read English — these eight keys are the
# Settings prose the user actually reads in their preferred locale.
settings-recovery-heading = 恢复 Web 界面
settings-recovery-enable = 启用恢复 Web 界面
settings-recovery-bind-address = 绑定地址
settings-recovery-port = 端口（0 = 自动选取空闲端口）
settings-recovery-show-url = 显示 URL 和令牌
settings-recovery-rotate-token = 轮换令牌
settings-recovery-allow-non-loopback = 允许非环回绑定
settings-recovery-non-loopback-warning = 警告：启用非环回绑定会将恢复界面暴露给你的本地网络。任何知道令牌的人都能浏览你的文件历史并下载文件。如果局域网不可信，请在其前面加上 TLS 或反向代理。

# Phase 40 — SMB compression negotiation + cloud-VM offload helper.
# 6 SMB keys (header badge + Settings prose) + 6 cloud-offload keys
# (Remotes tab wizard for cross-cloud copy templates).
smb-compress-badge = 🗜 SMB 压缩：{ $algo }
smb-compress-badge-tooltip = 发往此目标的网络流量正在传输中被压缩（SMB 3.1.1）。
smb-compress-toast-saved = 通过网络节省了 { $bytes }
smb-compress-algo-unknown = 未知算法
settings-smb-compress-heading = SMB 网络压缩
settings-smb-compress-hint = 在 UNC 目标上自动协商 SMB 3.1.1 流量压缩。在慢速链路上是免费收益；在本地目标上会被忽略。
cloud-offload-heading = 云虚拟机卸载帮助程序
cloud-offload-hint = 在两个云之间直接复制时，渲染一个部署模板，让复制从云中一个微型临时虚拟机上运行 — 字节永不经过你笔记本的网络。
cloud-offload-render-button = 渲染模板
cloud-offload-copy-clipboard = 复制到剪贴板
cloud-offload-template-format = 模板格式
cloud-offload-self-destruct-warning = 该虚拟机将在 { $minutes } 分钟后自动关机 — 部署前请确认 IAM 角色和区域。

# Phase 41 — animated before/after tree-diff preview. The `Preview
# changes` modal renders the rolled-up plan before the engine starts
# work; 14 keys cover the title, the summary header / counts, the
# row-reason labels, and the two action buttons.
preview-modal-title = 预览更改
preview-summary-header = 将会发生的操作
preview-category-additions = { $count } 项新增
preview-category-replacements = { $count } 项替换
preview-category-skips = { $count } 项跳过
preview-category-conflicts = { $count } 项冲突
preview-category-unchanged = { $count } 项未更改
preview-bytes-to-transfer = 待传输 { $bytes }
preview-reason-source-newer = 源较新
preview-reason-dest-newer = 目标较新 — 将跳过
preview-reason-content-different = 内容不同
preview-reason-identical = 与源相同
preview-button-run = 运行计划
preview-button-reduce = 缩减我的计划…

# Phase 42 — perceptual-hash visual-similarity dedup. Eight keys cover
# the pre-copy "looks visually identical" warning + the Settings panel.
perceptual-warn-title = 看起来视觉上完全相同
perceptual-warn-body = 目标处的 { $name } 似乎与源图片相符。仍要继续复制吗？
perceptual-warn-keep-both = 两者都保留
perceptual-warn-skip = 跳过此文件
perceptual-warn-overwrite = 仍然覆盖
perceptual-settings-heading = 视觉相似度去重
perceptual-settings-hint = 在目标处的图像被覆盖之前检测视觉上相同的图像。哈希是感知性的（能识别以不同格式重新保存的同一图片），而非字节精确。
perceptual-settings-threshold-label = 警告阈值（越低匹配越严格）

# Phase 42 Part B — per-file rolling versions (Time Machine for any
# destination). 8 keys cover the version-list panel + retention picker.
version-list-heading = 以前的版本
version-list-empty = 此文件没有先前版本
version-list-restore = 还原此版本
version-retention-heading = 覆盖时保留先前版本
version-retention-none = 永久保留每个版本
version-retention-last-n = 保留最近 { $n } 个版本
version-retention-older-than-days = 丢弃超过 { $days } 天的版本
version-retention-gfs = 每小时 { $h } · 每天 { $d } · 每周 { $w } · 每月 { $m }

# Phase 43 — forensic chain-of-custody manifests + BLAKE3 verified
# streaming. 14 keys cover the Settings → Provenance panel
# (heading, hint, toggles, TSA URL, signing-key management) plus
# the post-job manifest notification + the verify command's two
# headline result lines.
provenance-settings-heading = 取证监管链
provenance-settings-hint = 用 BLAKE3 + ed25519 清单为每个复制任务签名。审核人员日后可重新哈希目标目录树，证明复制后没有任何字节发生改变。
provenance-settings-enable-default = 默认为每个新任务签名
provenance-settings-show-after-job = 在每个已完成任务后显示清单
provenance-settings-tsa-url-label = 默认 RFC 3161 时间戳颁发机构 URL
provenance-settings-tsa-url-hint = 可选。设置后，清单会携带免费的 TSA 时间戳，证明这些字节在该时间点已存在。留空则跳过。
provenance-settings-keys-heading = 签名密钥
provenance-settings-keys-generate = 生成新密钥
provenance-settings-keys-import = 导入密钥…
provenance-settings-keys-export = 导出公钥…
provenance-job-completed-title = 来源清单已保存
provenance-job-completed-body = 已为 { $count } 个文件签名 → { $path }
provenance-verify-clean = 清单对 { $count } 个文件有效；签名 { $sig }；merkle 根正常。
provenance-verify-tampered = 清单无效 — { $tampered } 项被篡改，{ $missing } 项缺失。
# Phase 43 post-review hardening — toast text for the Settings →
# Provenance buttons whose Tauri IPC has not yet landed.
provenance-action-staged = 第 43 阶段 — 此操作的 IPC 接入将在后续提交中推出。

# Phase 44 — SSD-aware whole-drive sanitize (NVMe Sanitize / OPAL
# Crypto Erase) + the per-file shred refusal on copy-on-write
# filesystems. 16 keys cover the new "Drive sanitize" Settings
# subsection and the localized error messages.
sanitize-heading = 全盘安全清除
sanitize-hint = NVMe Sanitize、OPAL Crypto Erase 和 ATA Secure Erase 可在固件层以毫秒级清除闪存驱动器。逐文件覆写在闪存上毫无意义 — 多次粉碎只会损耗 NAND。请用此功能进行真正的清除。
sanitize-pick-device = 选择要清除的驱动器
sanitize-mode-label = 清除方法
sanitize-mode-nvme-format = NVMe Format（带安全擦除）
sanitize-mode-nvme-sanitize-block = NVMe Sanitize — 块擦除（慢，逐个单元）
sanitize-mode-nvme-sanitize-crypto = NVMe Sanitize — 加密擦除（即时）
sanitize-mode-ata-secure-erase = ATA Secure Erase（旧式 SATA SSD）
sanitize-mode-opal-crypto-erase = TCG OPAL Crypto Erase（自加密驱动器）
sanitize-mode-apfs-crypto-erase = APFS Crypto Erase（轮换 FileVault 密钥，仅限 macOS）
sanitize-confirm-1 = 这将销毁 { $device } 上的每一个字节。无法撤销。
sanitize-confirm-2 = 我明白 { $device } 上的所有分区、所有文件和所有快照都将永久无法读取。
sanitize-confirm-3 = 输入驱动器的型号名称以继续：{ $model }
sanitize-running = 正在清除 { $device }（{ $mode }）— 这可能耗时从毫秒（加密擦除）到数十分钟（块擦除）不等。请勿断电。
sanitize-completed = 清除完成 — { $device } 现已为空白。
ssd-honest-shred-meaningless = 在写时复制文件系统（Btrfs / ZFS / APFS）上的逐文件粉碎无法触及底层块。请改用全盘清除加上全盘加密密钥轮换。
ssd-honest-advisory = 此文件位于闪存上。逐文件覆写会损耗 NAND，且不保证原始单元无法恢复。对于敏感数据，请清除整个驱动器。

# Phase 44.1f post-review — placeholder toast for SanitizeTab
# buttons whose Tauri IPC has not yet landed.
sanitize-action-staged = 第 44.1 阶段 — 此操作的 IPC 接入将在后续提交中推出。

# Phase 45.3 — named-queue tab strip (Subfeature A). Tabs surface
# once the QueueRegistry holds at least one queue; the synthesised
# default tab keeps legacy single-queue jobs reachable.
queue-tab-default = 默认
queue-tab-empty-state = 任务队列
queue-badge-tooltip = 此队列中待处理和运行中的任务

# Phase 45.4 — drag-progress-merge (Subfeature B). Drag a queue tab
# onto another to merge their job lists. The default tab is neither
# draggable nor a drop target; only registry queues participate.
queue-drag-hint = 拖到另一个队列上以合并
queue-merge-confirm = 放下以合并
queue-merge-toast = 队列已合并

# Phase 45.5 — F2-queue UX (Subfeature C). F2 toggles
# `auto_enqueue_next` so every fresh enqueue piles into the running
# queue rather than spawning a parallel one. Status pill renders in
# the Footer; pulsing dot renders on the running tab.
queue-f2-active-hint = F2 模式：每个新入队任务都会进入此队列
queue-f2-toggled-on = F2 队列模式已开启 — 新入队任务将加入运行中的队列
queue-f2-toggled-off = F2 队列模式已关闭 — 新入队任务将生成并行队列
queue-f2-status-bar = F2 队列模式：开启

# Phase 45.6 — tray destination targets (Subfeature D). Pinned
# destinations appear in the OS tray menu; clicking one arms it as
# the active drop target so the next file drop bypasses the
# DropStagingDialog. Settings → General hosts the list editor.
tray-target-section-title = 托盘目标
tray-target-section-hint = 固定的目标会出现在托盘菜单中。点击其中一个即可将其设为下一个放置目标。
tray-target-empty = 尚未固定任何托盘目标。
tray-target-remove = 移除
tray-target-add-label = 标签
tray-target-add-path = 路径或后端 URI
tray-target-add = 添加
tray-target-armed-toast = 放下你的下一个文件，将其发送到 { $label }
tray-target-active-pill = → { $label }

# Phase 45.7 follow-up — pinned-destination validation errors. The
# `queue_pin_destination` IPC returns these Fluent keys verbatim
# when input fails the IPC-boundary checks; the toast layer renders
# them via `t(...)` (Toast.svelte detects kebab-case-lowercase and
# routes through the locale table). Phase 17e's `err-path-escape` /
# `err-destination-empty` cover the path-traversal + empty cases for
# `queue_route_job`; the keys below are pin-specific.
err-pinned-destination-label-empty = 托盘目标标签不能为空。
err-pinned-destination-path-empty = 托盘目标路径不能为空。
err-pinned-destination-label-too-long = 托盘目标标签过长（最多 64 个字符）。
err-pinned-destination-path-too-long = 托盘目标路径过长（最多 1024 个字符）。
err-pinned-destination-label-invalid = 托盘目标标签包含不允许的字符（换行、回车或 NUL）。
err-pinned-destination-path-invalid = 托盘目标路径包含不允许的字符（换行、回车或 NUL）。
err-pinned-destination-too-many = 你已达到 50 个托盘目标的上限。移除一个才能再添加。

# Phase 46.6 — Settings → Plugins tab. The IPC layer in
# `apps/copythat-ui/src-tauri/src/plugin_commands.rs` enumerates the
# per-user plugin store under `<config_dir>/plugins/`; the
# `PluginsTab.svelte` component renders these strings against the
# returned manifest + grant state.
settings-tab-plugins = 插件
plugin-heading = 插件
plugin-hint = 沙盒化的 WASM 插件以自定义钩子扩展 Copy That。每个插件在每次调用的 CPU 和内存限制下运行，且只能看到你授予它的主机能力。
plugin-list-empty = 尚未安装任何插件。
plugin-enabled = 已启用
plugin-disabled = 已禁用
plugin-hooks = 钩子
plugin-capabilities = 能力
plugin-no-capabilities = （无）
plugin-directory = 位置
plugin-install-from-file = 从文件安装…
plugin-install-from-url = 从 URL 安装…
plugin-url-wasm = WASM URL
plugin-url-manifest = 清单 URL
plugin-url-hash = BLAKE3 哈希
plugin-url-preview = 预览
plugin-url-confirm = 确认安装

# Phase 31b — power-policy settings (Power tab).
settings-tab-power = 电源
settings-power-hint = 根据电源状态限速或暂停复制：电池、按流量计费/蜂窝网络、演示/全屏，或 CPU 热降频时。
settings-power-enabled = 启用电源感知限速
settings-power-battery = 使用电池时
settings-power-metered = 在按流量计费的网络上
settings-power-cellular = 使用蜂窝网络时
settings-power-presentation = 演示时
settings-power-fullscreen = 全屏时
settings-power-thermal = 热降频时
settings-power-continue = 继续
settings-power-pause = 暂停
err-server-not-implemented = 服务器模式尚不可用。
err-webhook-not-implemented = Webhook 投递尚不可用。

# Phase 47 — "why is this slow?" diagnostics (bottleneck badge + tooltip).
bottleneck-source-io = 源 I/O
bottleneck-dest-io = 目标 I/O
bottleneck-network = 网络
bottleneck-antivirus = 杀毒软件
bottleneck-cpu = CPU
bottleneck-thermal = 热降频
bottleneck-unknown = 未知
diag-aria = 瓶颈：{ $cause }
diag-tooltip = 受限于 { $cause } · { $rate }
diag-spark-aria = 最近一分钟的吞吐量
diag-keeping-up = 速度正常
diag-label = 诊断
