app-name = Copy That 2026
# MT
window-title = Copy That 2026
# MT
shred-ssd-advisory = Cảnh báo: mục tiêu này nằm trên một ổ SSD. Ghi đè nhiều lần không thể làm sạch bộ nhớ flash một cách đáng tin cậy vì kỹ thuật cân bằng hao mòn (wear-leveling) và không gian dự phòng (overprovisioning) sẽ di chuyển dữ liệu ra khỏi địa chỉ khối logic. Đối với thiết bị trạng thái rắn, hãy ưu tiên ATA SECURE ERASE, NVMe Format kèm Secure Erase, hoặc mã hóa toàn ổ đĩa rồi hủy khóa.

# MT
state-idle = Không hoạt động
# MT
state-copying = Đang sao chép
# MT
state-verifying = Đang xác minh
# MT
state-paused = Tạm dừng
# MT
state-error = Lỗi

# MT
state-pending = Trong hàng đợi
# MT
state-running = Đang chạy
# MT
state-cancelled = Đã hủy
# MT
state-succeeded = Hoàn tất
# MT
state-failed = Thất bại

# MT
action-pause = Tạm dừng
# MT
action-resume = Tiếp tục
# MT
action-cancel = Hủy
# MT
action-pause-all = Tạm dừng tất cả công việc
# MT
action-resume-all = Tiếp tục tất cả công việc
# MT
action-cancel-all = Hủy tất cả công việc
# MT
action-close = Đóng
# MT
action-reveal = Hiển thị trong thư mục

# MT
menu-pause = Tạm dừng
# MT
menu-resume = Tiếp tục
# MT
menu-cancel = Hủy
# MT
menu-remove = Xóa khỏi hàng đợi
# MT
menu-reveal-source = Hiển thị nguồn trong thư mục
# MT
menu-reveal-destination = Hiển thị đích trong thư mục

# MT
header-eta-label = Thời gian còn lại ước tính
# MT
header-toolbar-label = Điều khiển chung

# MT
footer-queued = công việc đang hoạt động
# MT
footer-total-bytes = đang xử lý
# MT
footer-errors = lỗi
# MT
footer-history = Lịch sử

# MT
empty-title = Thả tệp hoặc thư mục để sao chép
# MT
empty-hint = Kéo các mục vào cửa sổ. Chúng tôi sẽ yêu cầu thư mục đích rồi thêm một công việc cho mỗi nguồn.
# MT
empty-region-label = Danh sách công việc

# MT
details-drawer-label = Chi tiết công việc
# MT
details-source = Nguồn
# MT
details-destination = Đích
# MT
details-state = Trạng thái
# MT
details-bytes = Byte
# MT
details-files = Tệp
# MT
details-speed = Tốc độ
# MT
details-eta = Thời gian còn lại
# MT
details-error = Lỗi

# MT
drop-dialog-title = Chuyển các mục đã thả
# MT
drop-dialog-subtitle = { $count } mục sẵn sàng để chuyển. Chọn một thư mục đích để bắt đầu.
# MT
drop-dialog-mode = Thao tác
# MT
drop-dialog-copy = Sao chép
# MT
drop-dialog-move = Di chuyển
# MT
drop-dialog-pick-destination = Chọn đích
# MT
drop-dialog-change-destination = Đổi đích
# MT
drop-dialog-start-copy = Bắt đầu sao chép
# MT
drop-dialog-start-move = Bắt đầu di chuyển

# MT
eta-calculating = đang tính…
# MT
eta-unknown = không rõ

# MT
toast-job-done = Đã hoàn tất chuyển
# MT
toast-copy-queued = Đã thêm sao chép vào hàng đợi
# MT
toast-move-queued = Đã thêm di chuyển vào hàng đợi
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
