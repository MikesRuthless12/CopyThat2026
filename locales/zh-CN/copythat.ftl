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
