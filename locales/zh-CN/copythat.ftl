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
