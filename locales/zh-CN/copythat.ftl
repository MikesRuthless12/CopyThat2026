app-name = Copy That 2026
# MT
window-title = Copy That 2026
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
err-io-other = 未知 I/O 错误

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
