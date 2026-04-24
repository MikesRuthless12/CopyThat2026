app-name = Copy That v1.25.0
# MT
window-title = Copy That v1.25.0
# MT
shred-ssd-advisory = 警告：此目标位于固态硬盘（SSD）上。由于磨损均衡和预留空间会将数据移出逻辑块地址，多次覆写无法可靠地清除闪存数据。对于固态介质，请优先使用 ATA SECURE ERASE、NVMe 安全擦除格式化，或使用已丢弃密钥的全盘加密。

# MT
state-idle = 空闲
# MT
state-copying = 正在复制
# MT
state-verifying = 正在校验
# MT
state-paused = 已暂停
# MT
state-error = 错误

# MT
state-pending = 队列中
# MT
state-running = 进行中
# MT
state-cancelled = 已取消
# MT
state-succeeded = 已完成
# MT
state-failed = 失败

# MT
action-pause = 暂停
# MT
action-resume = 继续
# MT
action-cancel = 取消
# MT
action-pause-all = 暂停所有任务
# MT
action-resume-all = 继续所有任务
# MT
action-cancel-all = 取消所有任务
# MT
action-close = 关闭
# MT
action-reveal = 在文件夹中显示

# MT
menu-pause = 暂停
# MT
menu-resume = 继续
# MT
menu-cancel = 取消
# MT
menu-remove = 从队列中移除
# MT
menu-reveal-source = 显示源文件位置
# MT
menu-reveal-destination = 显示目标位置

# MT
header-eta-label = 预计剩余时间
# MT
header-toolbar-label = 全局控制

# MT
footer-queued = 个活动任务
# MT
footer-total-bytes = 进行中
# MT
footer-errors = 个错误
# MT
footer-history = 历史记录

# MT
empty-title = 拖放文件或文件夹以开始复制
# MT
empty-hint = 将项目拖到窗口上。我们会请您选择目标文件夹，然后为每个源创建一个任务。
# MT
empty-region-label = 任务列表

# MT
details-drawer-label = 任务详情
# MT
details-source = 源
# MT
details-destination = 目标
# MT
details-state = 状态
# MT
details-bytes = 字节数
# MT
details-files = 文件数
# MT
details-speed = 速度
# MT
details-eta = 剩余时间
# MT
details-error = 错误

# MT
drop-dialog-title = 传输拖入的项目
# MT
drop-dialog-subtitle = 有 { $count } 个项目可传输。请选择目标文件夹以开始。
# MT
drop-dialog-mode = 操作
# MT
drop-dialog-copy = 复制
# MT
drop-dialog-move = 移动
# MT
drop-dialog-pick-destination = 选择目标
# MT
drop-dialog-change-destination = 更改目标
# MT
drop-dialog-start-copy = 开始复制
# MT
drop-dialog-start-move = 开始移动

# MT
eta-calculating = 计算中…
# MT
eta-unknown = 未知

# MT
toast-job-done = 传输完成
# MT
toast-copy-queued = 已加入复制队列
# MT
toast-move-queued = 已加入移动队列
# MT — Phase 8 toast messages
toast-error-resolved = 错误已解决
# MT
toast-collision-resolved = 冲突已解决
# MT
toast-elevated-unavailable = 以提升权限重试将在第 17 阶段提供 — 目前尚不可用
toast-clipboard-files-detected = 剪贴板中有文件 — 按粘贴快捷键以通过 Copy That 复制
toast-clipboard-no-files = 剪贴板中没有可粘贴的文件
# MT
toast-error-log-exported = 错误日志已导出

# MT — Error modal
error-modal-title = 传输失败
# MT
error-modal-retry = 重试
# MT
error-modal-retry-elevated = 以提升权限重试
# MT
error-modal-skip = 跳过
# MT
error-modal-skip-all-kind = 跳过所有此类错误
# MT
error-modal-abort = 全部中止
# MT
error-modal-path-label = 路径
# MT
error-modal-code-label = 代码
error-drawer-pending-count = 更多错误等待中
error-drawer-toggle = 折叠或展开

# MT — Error-kind labels
err-not-found = 文件未找到
# MT
err-permission-denied = 权限被拒绝
# MT
err-disk-full = 目标磁盘已满
# MT
err-interrupted = 操作被中断
# MT
err-verify-failed = 复制后校验失败
# MT
err-path-escape = 路径被拒绝 — 包含父目录（..）段或非法字节
# MT
err-io-other = 未知 I/O 错误
err-sparseness-mismatch = 无法在目标保留稀疏布局  # MT

# MT — Collision modal
collision-modal-title = 文件已存在
# MT
collision-modal-overwrite = 覆盖
# MT
collision-modal-overwrite-if-newer = 较新时覆盖
# MT
collision-modal-skip = 跳过
# MT
collision-modal-keep-both = 两者都保留
# MT
collision-modal-rename = 重命名…
# MT
collision-modal-apply-to-all = 应用到全部
# MT
collision-modal-source = 源
# MT
collision-modal-destination = 目标
# MT
collision-modal-size = 大小
# MT
collision-modal-modified = 修改时间
# MT
collision-modal-hash-check = 快速哈希 (SHA-256)
# MT
collision-modal-rename-placeholder = 新文件名
# MT
collision-modal-confirm-rename = 重命名

# MT — Error log drawer
error-log-title = 错误日志
# MT
error-log-empty = 没有记录的错误
# MT
error-log-export-csv = 导出 CSV
# MT
error-log-export-txt = 导出文本
# MT
error-log-clear = 清空日志
# MT
error-log-col-time = 时间
# MT
error-log-col-job = 任务
# MT
error-log-col-path = 路径
# MT
error-log-col-code = 代码
# MT
error-log-col-message = 消息
# MT
error-log-col-resolution = 解决方式

# MT — History drawer (Phase 9)
history-title = 历史记录
# MT
history-empty = 尚无任务记录
# MT
history-unavailable = 复制历史不可用。启动时应用未能打开 SQLite 存储。
# MT
history-filter-any = 任何
# MT
history-filter-kind = 类型
# MT
history-filter-status = 状态
# MT
history-filter-text = 搜索
# MT
history-refresh = 刷新
# MT
history-export-csv = 导出 CSV
# MT
history-purge-30 = 清除超过 30 天
# MT
history-rerun = 重新执行
# MT
history-detail-open = 详情
# MT
history-detail-title = 任务详情
# MT
history-detail-empty = 没有记录的项目
# MT
history-col-date = 日期
# MT
history-col-kind = 类型
# MT
history-col-src = 源
# MT
history-col-dst = 目标
# MT
history-col-files = 文件
# MT
history-col-size = 大小
# MT
history-col-status = 状态
# MT
history-col-duration = 持续时间
# MT
history-col-error = 错误

# MT
toast-history-exported = 历史已导出
# MT
toast-history-rerun-queued = 重新执行已加入队列

# MT — Totals drawer (Phase 10)
footer-totals = 总计
# MT
totals-title = 总计
# MT
totals-loading = 正在加载总计…
# MT
totals-card-bytes = 累计复制字节数
# MT
totals-card-files = 文件
# MT
totals-card-jobs = 任务
# MT
totals-card-avg-rate = 平均吞吐量
# MT
totals-errors = 错误
# MT
totals-spark-title = 最近 30 天
# MT
totals-kinds-title = 按类型
# MT
totals-saved-title = 节省时间 (估算)
# MT
totals-saved-note = 与标准文件管理器以相同工作负载复制作为基准的估算比较。
# MT
totals-reset = 重置统计
# MT
totals-reset-confirm = 这将删除所有已存储的任务和项目。继续吗?
# MT
totals-reset-confirm-yes = 是,重置
# MT
toast-totals-reset = 统计已重置

# MT — Phase 11a additions
header-language-label = 语言
# MT
header-language-title = 更改语言

# MT
kind-copy = 复制
# MT
kind-move = 移动
# MT
kind-delete = 删除
# MT
kind-secure-delete = 安全删除

# MT
status-running = 进行中
# MT
status-succeeded = 成功
# MT
status-failed = 失败
# MT
status-cancelled = 已取消
# MT
status-ok = 确定
# MT
status-skipped = 已跳过

# MT
history-search-placeholder = /路径
# MT
toast-history-purged = 已清除 { $count } 个超过 30 天的任务

# MT
err-source-required = 至少需要一个源路径。
# MT
err-destination-empty = 目标路径为空。
# MT
err-source-empty = 源路径为空。

# MT
duration-lt-1s = < 1 秒
# MT
duration-ms = { $ms } 毫秒
# MT
duration-seconds = { $s } 秒
# MT
duration-minutes-seconds = { $m } 分 { $s } 秒
# MT
duration-hours-minutes = { $h } 小时 { $m } 分
# MT
duration-zero = 0 秒

# MT
rate-unit-per-second = { $size }/秒

# MT — Phase 11b Settings modal
settings-title = 设置
# MT
settings-tab-general = 常规
# MT
settings-tab-appearance = 外观
# MT
settings-section-language = 语言
# MT
settings-phase-12-hint = 更多设置(主题、传输默认值、验证算法、配置文件)将在第 12 阶段推出。

# MT — Phase 12 Settings window
settings-loading = 正在加载设置…
# MT
settings-tab-transfer = 传输
# MT
settings-tab-shell = 外壳
# MT
settings-tab-secure-delete = 安全删除
# MT
settings-tab-advanced = 高级
# MT
settings-tab-profiles = 配置文件

# MT
settings-section-theme = 主题
# MT
settings-theme-auto = 自动
# MT
settings-theme-light = 浅色
# MT
settings-theme-dark = 深色
# MT
settings-start-with-os = 开机启动
# MT
settings-single-instance = 单实例运行
# MT
settings-minimize-to-tray = 关闭时最小化到托盘
settings-error-display-mode = 错误提示样式
settings-error-display-modal = 模态(阻止应用)
settings-error-display-drawer = 抽屉(非阻塞)
settings-error-display-mode-hint = 模态会停止队列直到你决定。抽屉让队列继续运行,可在角落中分类处理错误。
settings-paste-shortcut = 通过全局快捷键粘贴文件
settings-paste-shortcut-combo = 快捷键组合
settings-paste-shortcut-hint = 在系统任意位置按下此组合,可通过 Copy That 粘贴从资源管理器 / 访达 / 文件复制的文件。CmdOrCtrl 在 macOS 上解析为 Cmd,在 Windows / Linux 上解析为 Ctrl。
settings-clipboard-watcher = 监视剪贴板中的复制文件
settings-clipboard-watcher-hint = 当文件 URL 出现在剪贴板时显示通知,提示你可以通过 Copy That 粘贴。启用时每 500 毫秒轮询一次。

# MT
settings-buffer-size = 缓冲区大小
# MT
settings-verify = 复制后校验
# MT
settings-verify-off = 关闭
# MT
settings-concurrency = 并发
# MT
settings-concurrency-auto = 自动
# MT
settings-reflink = Reflink / 快速路径
# MT
settings-reflink-prefer = 优先
# MT
settings-reflink-avoid = 避免 reflink
# MT
settings-reflink-disabled = 始终使用异步引擎
# MT
settings-fsync-on-close = 关闭时同步到磁盘(较慢,更安全)
# MT
settings-preserve-timestamps = 保留时间戳
# MT
settings-preserve-permissions = 保留权限
# MT
settings-preserve-acls = 保留 ACL(第 14 阶段)
settings-preserve-sparseness = 保留稀疏文件  # MT
settings-preserve-sparseness-hint = 仅复制稀疏文件(VM 磁盘、数据库文件)的已分配区段,以便目标在磁盘上的大小与源相同。  # MT

# MT
settings-context-menu = 启用外壳上下文菜单项
# MT
settings-intercept-copy = 拦截默认复制处理程序(Windows)
# MT
settings-intercept-copy-hint = 启用后,资源管理器的 Ctrl+C / Ctrl+V 通过 Copy That。注册在第 14 阶段。
# MT
settings-notify-completion = 任务完成时通知

# MT
settings-shred-method = 默认粉碎方法
# MT
settings-shred-zero = 零(1 遍)
# MT
settings-shred-random = 随机(1 遍)
# MT
settings-shred-dod3 = DoD 5220.22-M(3 遍)
# MT
settings-shred-dod7 = DoD 5220.22-M(7 遍)
# MT
settings-shred-gutmann = Gutmann(35 遍)
# MT
settings-shred-nist = NIST 800-88
# MT
settings-shred-confirm-twice = 粉碎前要求双重确认

# MT
settings-log-level = 日志级别
# MT
settings-log-off = 关闭
# MT
settings-telemetry = 遥测
# MT
settings-telemetry-never = 从不 — 任何日志级别下都不发送数据
# MT
settings-error-policy = 默认错误策略
# MT
settings-error-policy-ask = 询问
# MT
settings-error-policy-skip = 跳过
# MT
settings-error-policy-retry = 带退避重试
# MT
settings-error-policy-abort = 首次错误时中止
# MT
settings-history-retention = 历史保留(天)
# MT
settings-history-retention-hint = 0 = 永久保留。其他值会在启动时自动清理旧任务。
# MT
settings-database-path = 数据库路径
# MT
settings-database-path-default = (默认 — 系统数据目录)
# MT
settings-reset-all = 重置为默认
# MT
settings-reset-confirm = 重置所有偏好设置?配置文件不受影响。

# MT
settings-profiles-hint = 将当前设置另存为命名配置;稍后加载,无需逐项调整即可切换。
# MT
settings-profile-name-placeholder = 配置文件名称
# MT
settings-profile-save = 保存
# MT
settings-profile-import = 导入…
# MT
settings-profile-load = 加载
# MT
settings-profile-export = 导出…
# MT
settings-profile-delete = 删除
# MT
settings-profile-empty = 尚未保存任何配置文件。
# MT
settings-profile-import-prompt = 导入配置文件的名称:

# MT
toast-settings-reset = 设置已重置
# MT
toast-profile-saved = 配置文件已保存
# MT
toast-profile-loaded = 配置文件已加载
# MT
toast-profile-exported = 配置文件已导出
# MT
toast-profile-imported = 配置文件已导入

# Phase 13d — activity feed + header picker buttons
action-add-files = 添加文件
action-add-folders = 添加文件夹
activity-title = 活动
activity-clear = 清除活动列表
activity-empty = 暂无文件活动。
activity-after-done = 完成后:
activity-keep-open = 保持应用打开
activity-close-app = 关闭应用
activity-shutdown = 关机
activity-logoff = 注销
activity-sleep = 睡眠

# Phase 14 — preflight free-space dialog
preflight-block-title = 目标位置空间不足
preflight-warn-title = 目标位置空间较低
preflight-unknown-title = 无法确定可用空间
preflight-unknown-body = 源过大无法快速测量，或目标卷未响应。您可以继续；如果空间用完，引擎会干净地停止复制。
preflight-required = 需要
preflight-free = 可用
preflight-reserve = 保留
preflight-shortfall = 缺口
preflight-continue = 仍然继续
collision-modal-overwrite-older = 仅覆盖较旧的文件

# Phase 14e — subset picker
preflight-pick-subset = 选择要复制的内容…
subset-title = 选择要复制的源
subset-subtitle = 整体选择无法装入目标。勾选要复制的项目；其余将保留。
subset-loading = 正在测量大小…
subset-too-large = 过大无法计数
subset-budget = 可用
subset-remaining = 剩余
subset-confirm = 复制所选
history-rerun-hint = 重新运行此复制 — 重新扫描源树中的每个文件
history-clear-all = 全部清除
history-clear-all-confirm = 再次点击以确认
history-clear-all-hint = 删除每一行历史记录。需要第二次点击确认。
toast-history-cleared = 已清除历史（删除 { $count } 行）

# Phase 15 — source-list ordering
drop-dialog-sort-label = 排序：
sort-custom = 自定义
sort-name-asc = 名称 A → Z（文件优先）
sort-name-desc = 名称 Z → A（文件优先）
sort-size-asc = 大小从小到大（文件优先）
sort-size-desc = 大小从大到小（文件优先）
sort-reorder = 重新排序
sort-move-top = 移到最上
sort-move-up = 上移
sort-move-down = 下移
sort-move-bottom = 移到最下
sort-name-asc-simple = 名称 A → Z
sort-name-desc-simple = 名称 Z → A
sort-size-asc-simple = 从小到大
sort-size-desc-simple = 从大到小
activity-sort-locked = 复制进行中时禁用排序。暂停或等待完成后再更改顺序。
drop-dialog-collision-label = 如果文件已存在:
collision-policy-keep-both = 同时保留两者（将新副本重命名为 _2、_3 …）
collision-policy-skip = 跳过新副本
collision-policy-overwrite = 覆盖现有文件
collision-policy-overwrite-if-newer = 仅在较新时覆盖
collision-policy-prompt = 每次询问
drop-dialog-busy-checking = 正在检查可用空间…
drop-dialog-busy-enumerating = 正在统计文件…
drop-dialog-busy-starting = 正在开始复制…
toast-enumeration-deferred = 源树较大 — 跳过预列表；引擎处理时行会陆续显示。

# Phase 14a — enumeration-time filters
# MT
settings-tab-filters = 过滤器
# MT
settings-filters-hint = 在枚举时跳过文件,引擎不会打开它们。包含仅作用于文件;排除也会剪除匹配的目录。
# MT
settings-filters-enabled = 为树复制启用过滤器
# MT
settings-filters-include-globs = 包含通配符
# MT
settings-filters-include-globs-placeholder = **/*.txt
# MT
settings-filters-include-globs-hint = 每行一个。非空时文件必须至少匹配其一。目录始终会被遍历。
# MT
settings-filters-exclude-globs = 排除通配符
# MT
settings-filters-exclude-globs-placeholder = **/node_modules
# MT
settings-filters-exclude-globs-hint = 每行一个。匹配会剪除目录的整个子树;匹配的文件会被跳过。
# MT
settings-filters-size-range = 文件大小范围
# MT
settings-filters-min-size-bytes = 最小大小(字节,留空 = 无下限)
# MT
settings-filters-max-size-bytes = 最大大小(字节,留空 = 无上限)
# MT
settings-filters-date-range = 修改时间范围
# MT
settings-filters-min-mtime = 修改于此日期之后
# MT
settings-filters-max-mtime = 修改于此日期之前
# MT
settings-filters-attributes = 属性位
# MT
settings-filters-skip-hidden = 跳过隐藏的文件 / 文件夹
# MT
settings-filters-skip-system = 跳过系统文件(仅 Windows)
# MT
settings-filters-skip-readonly = 跳过只读文件

# Phase 15 — auto-update
# MT
settings-tab-updater = 更新
# MT
settings-updater-hint = Copy That 每天最多检查一次签名更新。更新将在下次退出应用时安装。
# MT
settings-updater-auto-check = 启动时检查更新
# MT
settings-updater-channel = 发布通道
# MT
settings-updater-channel-stable = 稳定版
# MT
settings-updater-channel-beta = 测试版(预发布)
# MT
settings-updater-last-check = 上次检查
# MT
settings-updater-last-never = 从未
# MT
settings-updater-check-now = 立即检查更新
# MT
settings-updater-checking = 检查中…
# MT
settings-updater-available = 有可用更新
# MT
settings-updater-up-to-date = 你正在使用最新版本。
# MT
settings-updater-dismiss = 跳过此版本
# MT
settings-updater-dismissed = 已跳过
# MT
toast-update-available = 有新版本可用
# MT
toast-update-up-to-date = 你已是最新版本

# Phase 19a — disk-backed file enumeration (TeraCopy-compatible scan DB)
# MT
scan-progress-title = 正在扫描…
# MT
scan-progress-stats = { $files } 个文件 · 至今 { $bytes }
# MT
scan-pause-button = 暂停扫描
# MT
scan-resume-button = 恢复扫描
# MT
scan-cancel-button = 取消扫描
# MT
scan-cancel-confirm = 取消扫描并放弃进度？
# MT
scan-db-header = 扫描数据库
# MT
scan-db-hint = 用于数百万文件任务的磁盘扫描数据库。
# MT
advanced-scan-hash-during = 扫描时计算校验和
# MT
advanced-scan-db-path = 扫描数据库位置
# MT
advanced-scan-retention-days = 完成的扫描自动删除（天）
# MT
advanced-scan-max-keep = 保留的最大扫描数据库数

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
sparse-not-supported-title = 目标填充稀疏文件  # MT
sparse-not-supported-body = { $dst_fs } 不支持稀疏文件。源中的空洞被写入为零,因此目标在磁盘上更大。  # MT
sparse-warning-densified = 已保留稀疏布局:仅复制了已分配的区段。  # MT
sparse-warning-mismatch = 稀疏布局不匹配——目标可能比预期更大。  # MT

# Phase 24 — security-metadata preservation. MT-flagged drafts;
# the authoritative English source lives in locales/en/copythat.ftl.
settings-preserve-security-metadata = 保留安全元数据  # MT
settings-preserve-security-metadata-hint = 在每次复制时捕获并重新应用带外元数据流(NTFS ADS / xattrs / POSIX ACL / SELinux 上下文 / Linux 文件能力 / macOS 资源分支)。  # MT
settings-preserve-motw = 保留网络标记(从互联网下载标志)  # MT
settings-preserve-motw-hint = 对安全至关重要。SmartScreen 和 Office Protected View 使用此流来警告从互联网下载的文件。禁用会让下载的可执行文件在复制时失去其来源标记,并绕过操作系统的安全防护。  # MT
settings-preserve-posix-acls = 保留 POSIX ACL 和扩展属性  # MT
settings-preserve-posix-acls-hint = 在复制过程中携带 user.* / system.* / trusted.* xattrs 和 POSIX 访问控制列表。  # MT
settings-preserve-selinux = 保留 SELinux 上下文  # MT
settings-preserve-selinux-hint = 在复制过程中携带 security.selinux 标签,以便在 MAC 策略下运行的守护进程仍能访问该文件。  # MT
settings-preserve-resource-forks = 保留 macOS 资源分支和 Finder 信息  # MT
settings-preserve-resource-forks-hint = 在复制过程中携带遗留的资源分支和 FinderInfo(颜色标签、Carbon 元数据)。  # MT
settings-appledouble-fallback = 在不兼容的文件系统上使用 AppleDouble 附属文件  # MT
meta-translated-to-appledouble = 外部元数据已存储在 AppleDouble 附属文件中 (._{ $ext })  # MT

# Phase 25 — two-way sync with vector-clock conflict detection.
# MT-flagged drafts; the authoritative English source lives in
# locales/en/copythat.ftl.
footer-sync = 同步  # MT
sync-drawer-title = 双向同步  # MT
sync-drawer-hint = 在不静默覆盖的情况下保持两个文件夹同步。并发编辑显示为可解决的冲突。  # MT
sync-add-pair = 添加对  # MT
sync-add-cancel = 取消  # MT
sync-refresh = 刷新  # MT
sync-add-save = 保存对  # MT
sync-add-saving = 保存中…  # MT
sync-add-missing-fields = 标签、左侧路径和右侧路径都是必需的。  # MT
sync-remove-confirm = 删除此同步对?状态数据库被保留;文件夹未受影响。  # MT
sync-field-label = 标签  # MT
sync-field-label-placeholder = 例如 文档 ↔ NAS  # MT
sync-field-left = 左侧文件夹  # MT
sync-field-left-placeholder = 选择或粘贴绝对路径  # MT
sync-field-right = 右侧文件夹  # MT
sync-field-right-placeholder = 选择或粘贴绝对路径  # MT
sync-field-mode = 模式  # MT
sync-mode-two-way = 双向  # MT
sync-mode-mirror-left-to-right = 镜像 (左 → 右)  # MT
sync-mode-mirror-right-to-left = 镜像 (右 → 左)  # MT
sync-mode-contribute-left-to-right = 贡献 (左 → 右,无删除)  # MT
sync-no-pairs = 尚未配置同步对。点击"添加对"开始。  # MT
sync-loading = 正在加载已配置的对…  # MT
sync-never-run = 从未运行  # MT
sync-running = 运行中  # MT
sync-run-now = 立即运行  # MT
sync-cancel = 取消  # MT
sync-remove-pair = 删除  # MT
sync-view-conflicts = 查看冲突 ({ $count })  # MT
sync-conflicts-heading = 冲突  # MT
sync-no-conflicts = 上次运行没有冲突。  # MT
sync-winner = 胜者  # MT
sync-side-left-to-right = 左  # MT
sync-side-right-to-left = 右  # MT
sync-conflict-kind-concurrent-write = 并发编辑  # MT
sync-conflict-kind-delete-edit = 删除 ↔ 编辑  # MT
sync-conflict-kind-add-add = 双方都添加了  # MT
sync-conflict-kind-corrupt-equal = 内容在没有新写入的情况下发生分歧  # MT
sync-resolve-keep-left = 保留左侧  # MT
sync-resolve-keep-right = 保留右侧  # MT
sync-resolve-keep-both = 保留两者  # MT
sync-resolve-three-way = 通过 3-way 合并解决  # MT
sync-resolve-phase-53-tooltip = 非文本文件的交互式 3-way 合并将在第 53 阶段提供。  # MT
sync-error-prefix = 同步错误  # MT

# Phase 26 — real-time mirror watcher. MT-flagged drafts;
# the authoritative English source lives in locales/en/copythat.ftl.
live-mirror-start = 启动实时镜像  # MT
live-mirror-stop = 停止实时镜像  # MT
live-mirror-watching = 监视中  # MT
live-mirror-toggle-hint = 在每次检测到文件系统更改时自动重新同步。每个活动对一个后台线程。  # MT
watch-event-prefix = 文件更改  # MT
watch-overflow-recovered = 监视器缓冲区溢出;正在重新枚举以恢复  # MT

# Phase 27 — content-defined chunk store. MT-flagged drafts;
# the authoritative English source lives in locales/en/copythat.ftl.
chunk-store-section = 分块存储  # MT
chunk-store-enable = 启用分块存储 (增量恢复和去重)  # MT
chunk-store-enable-hint = 按内容 (FastCDC) 拆分每个复制的文件并以内容寻址方式存储分块。重试仅重写已更改的分块;具有共享内容的文件自动去重。  # MT
chunk-store-location = 分块存储位置  # MT
chunk-store-max-size = 最大分块存储大小  # MT
chunk-store-prune = 清除早于 (天) 的分块  # MT
chunk-store-savings = 通过分块去重节省了 { $gib } GiB  # MT
chunk-store-disk-usage = 在 { $chunks } 个分块中使用 { $size }  # MT

# Phase 28 — tray-resident Drop Stack. MT-flagged drafts;
# the authoritative English source lives in locales/en/copythat.ftl.
dropstack-window-title = 拖放堆栈  # MT
dropstack-tray-open = 拖放堆栈  # MT
dropstack-empty-title = 拖放堆栈为空  # MT
dropstack-empty-hint = 从资源管理器拖动文件到此处,或右键单击作业行以添加。  # MT
dropstack-add-to-stack = 添加到拖放堆栈  # MT
dropstack-copy-all-to = 全部复制到…  # MT
dropstack-move-all-to = 全部移动到…  # MT
dropstack-clear = 清空堆栈  # MT
dropstack-remove-row = 从堆栈移除  # MT
dropstack-path-missing-toast = 已移除 { $path } — 文件不再存在。  # MT
dropstack-always-on-top = 拖放堆栈始终置顶  # MT
dropstack-show-tray-icon = 显示 Copy That 托盘图标  # MT
dropstack-open-on-start = 应用启动时自动打开拖放堆栈  # MT
dropstack-count = { $count } 路径  # MT

# Phase 29 — spring-loaded folders + native DnD polish.
settings-dnd-heading = Drag and drop  # MT
settings-dnd-spring-load = Spring-load folders while dragging  # MT
settings-dnd-spring-delay = Spring-load delay (ms)  # MT
settings-dnd-thumbnails = Show drag thumbnails  # MT
settings-dnd-invalid-highlight = Highlight invalid drop targets  # MT
dropzone-invalid-title = Not a valid drop target  # MT
dropzone-invalid-readonly = Destination is read-only  # MT
dropzone-picker-title = Choose a destination  # MT
dropzone-picker-up = Up  # MT
dropzone-picker-path = Current path  # MT
dropzone-picker-root = Roots  # MT
dropzone-picker-use-this = Use this folder  # MT
dropzone-picker-empty = No subfolders  # MT
dropzone-picker-cancel = Cancel  # MT

# Phase 30 — cross-platform path translation.
translate-heading = Cross-platform compatibility  # MT
translate-unicode-label = Unicode normalization  # MT
translate-unicode-auto = Auto-detect destination  # MT
translate-unicode-windows = NFC (Windows / Linux)  # MT
translate-unicode-macos = Leave as-is (macOS / APFS)  # MT
translate-line-endings-label = Translate line endings for text files  # MT
translate-line-endings-allowlist = Text file extensions  # MT
reserved-name-label = Windows reserved-name handling  # MT
reserved-name-suffix = Append "_" (CON.txt → CON_.txt)  # MT
reserved-name-reject = Reject and warn  # MT
long-path-label = Use Windows long-path prefix (\?\) when over 260 chars  # MT
long-path-hint = Some network shares and legacy tools don't honor the \?\ namespace.  # MT

# Phase 31 — power-aware copying.
power-heading = Power & State  # MT
power-enabled = Enable power-aware rules  # MT
power-battery-label = On battery  # MT
power-metered-label = On metered Wi-Fi  # MT
power-cellular-label = On cellular  # MT
power-presentation-label = When presenting (Zoom / Teams / Keynote)  # MT
power-fullscreen-label = When an app is fullscreen  # MT
power-thermal-label = When CPU is thermal-throttling  # MT
power-rule-continue = Continue at full speed  # MT
power-rule-pause = Pause all jobs  # MT
power-rule-cap = Cap bandwidth  # MT
power-rule-cap-percent = Cap to a percent of current rate  # MT
power-reason-on-battery = on battery  # MT
power-reason-metered-network = metered network  # MT
power-reason-cellular-network = cellular network  # MT
power-reason-presenting = presentation mode  # MT
power-reason-fullscreen = fullscreen app  # MT
power-reason-thermal-throttling = CPU is throttling  # MT

# Phase 32 — cloud backend matrix via OpenDAL.
remote-heading = Remote backends  # MT
remote-add = Add backend  # MT
remote-list-empty = No remote backends configured  # MT
remote-test = Test connection  # MT
remote-test-success = Connection successful  # MT
remote-test-failed = Connection failed  # MT
remote-remove = Remove backend  # MT
remote-name-label = Display name  # MT
remote-kind-label = Backend type  # MT
remote-save = Save backend  # MT
remote-cancel = Cancel  # MT
backend-s3 = Amazon S3  # MT
backend-r2 = Cloudflare R2  # MT
backend-b2 = Backblaze B2  # MT
backend-azure-blob = Azure Blob Storage  # MT
backend-gcs = Google Cloud Storage  # MT
backend-onedrive = OneDrive  # MT
backend-google-drive = Google Drive  # MT
backend-dropbox = Dropbox  # MT
backend-webdav = WebDAV  # MT
backend-sftp = SFTP  # MT
backend-ftp = FTP  # MT
backend-local-fs = Local filesystem  # MT
cloud-config-bucket = Bucket  # MT
cloud-config-region = Region  # MT
cloud-config-endpoint = Endpoint URL  # MT
cloud-config-root = Root path  # MT
cloud-error-invalid-config = Backend configuration is invalid  # MT
cloud-error-network = Network error contacting backend  # MT
cloud-error-not-found = Object not found at the requested path  # MT
cloud-error-permission = Permission denied by remote backend  # MT
cloud-error-keychain = OS keychain access failed  # MT
settings-tab-remotes = Remotes  # MT

# Phase 33 — mount as read-only filesystem.
mount-heading = Mount snapshot  # MT
mount-action-mount = Mount snapshot  # MT
mount-action-unmount = Unmount  # MT
mount-status-mounted = Mounted at { $path }  # MT
mount-error-unsafe-mountpoint = Mountpoint path is unsafe  # MT
mount-error-mountpoint-not-empty = Mountpoint must be an empty directory  # MT
mount-error-backend-unavailable = Mount backend is not available on this system  # MT
mount-error-archive-read = Archive read failed  # MT
mount-picker-title = Pick mountpoint directory  # MT
mount-toast-mounted = Snapshot mounted at { $path }  # MT
mount-toast-unmounted = Snapshot unmounted  # MT
mount-toast-failed = Mount failed: { $reason }  # MT
settings-mount-heading = Mount snapshots  # MT
settings-mount-hint = Expose the history archive as a read-only filesystem. Phase 33b wires the runner flow; the kernel FUSE/WinFsp backends land in Phase 33c.  # MT
settings-mount-on-launch = Mount the latest snapshot on launch  # MT
settings-mount-on-launch-path = Mountpoint path  # MT
settings-mount-on-launch-path-placeholder = e.g. C:\Mounts\copythat  # MT
