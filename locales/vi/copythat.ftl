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
# MT — Phase 8 toast messages
toast-error-resolved = Đã giải quyết lỗi
# MT
toast-collision-resolved = Đã giải quyết xung đột
# MT
toast-elevated-unavailable = Thử lại với quyền nâng cao sẽ có ở Giai đoạn 17 — hiện chưa khả dụng
# MT
toast-error-log-exported = Đã xuất nhật ký lỗi

# MT — Error modal
error-modal-title = Một quá trình truyền đã thất bại
# MT
error-modal-retry = Thử lại
# MT
error-modal-retry-elevated = Thử lại với quyền nâng cao
# MT
error-modal-skip = Bỏ qua
# MT
error-modal-skip-all-kind = Bỏ qua mọi lỗi cùng loại
# MT
error-modal-abort = Hủy tất cả
# MT
error-modal-path-label = Đường dẫn
# MT
error-modal-code-label = Mã

# MT — Error-kind labels
err-not-found = Không tìm thấy tệp
# MT
err-permission-denied = Bị từ chối quyền
# MT
err-disk-full = Đĩa đích đã đầy
# MT
err-interrupted = Thao tác bị gián đoạn
# MT
err-verify-failed = Xác minh sau khi sao chép thất bại
# MT
err-io-other = Lỗi I/O không xác định

# MT — Collision modal
collision-modal-title = Tệp đã tồn tại
# MT
collision-modal-overwrite = Ghi đè
# MT
collision-modal-overwrite-if-newer = Ghi đè nếu mới hơn
# MT
collision-modal-skip = Bỏ qua
# MT
collision-modal-keep-both = Giữ cả hai
# MT
collision-modal-rename = Đổi tên…
# MT
collision-modal-apply-to-all = Áp dụng cho tất cả
# MT
collision-modal-source = Nguồn
# MT
collision-modal-destination = Đích
# MT
collision-modal-size = Kích thước
# MT
collision-modal-modified = Đã sửa đổi
# MT
collision-modal-hash-check = Băm nhanh (SHA-256)
# MT
collision-modal-rename-placeholder = Tên tệp mới
# MT
collision-modal-confirm-rename = Đổi tên

# MT — Error log drawer
error-log-title = Nhật ký lỗi
# MT
error-log-empty = Chưa có lỗi nào được ghi
# MT
error-log-export-csv = Xuất CSV
# MT
error-log-export-txt = Xuất văn bản
# MT
error-log-clear = Xóa nhật ký
# MT
error-log-col-time = Thời gian
# MT
error-log-col-job = Tác vụ
# MT
error-log-col-path = Đường dẫn
# MT
error-log-col-code = Mã
# MT
error-log-col-message = Thông điệp
# MT
error-log-col-resolution = Giải quyết

# MT — History drawer (Phase 9)
history-title = Lịch sử
# MT
history-empty = Chưa ghi nhận tác vụ nào
# MT
history-unavailable = Lịch sử sao chép không khả dụng. Ứng dụng không thể mở kho SQLite khi khởi động.
# MT
history-filter-any = tất cả
# MT
history-filter-kind = Loại
# MT
history-filter-status = Trạng thái
# MT
history-filter-text = Tìm kiếm
# MT
history-refresh = Làm mới
# MT
history-export-csv = Xuất CSV
# MT
history-purge-30 = Xóa cũ hơn 30 ngày
# MT
history-rerun = Chạy lại
# MT
history-detail-open = Chi tiết
# MT
history-detail-title = Chi tiết tác vụ
# MT
history-detail-empty = Chưa có mục nào được ghi
# MT
history-col-date = Ngày
# MT
history-col-kind = Loại
# MT
history-col-src = Nguồn
# MT
history-col-dst = Đích
# MT
history-col-files = Tệp
# MT
history-col-size = Kích thước
# MT
history-col-status = Trạng thái
# MT
history-col-duration = Thời lượng
# MT
history-col-error = Lỗi

# MT
toast-history-exported = Đã xuất lịch sử
# MT
toast-history-rerun-queued = Đã thêm chạy lại vào hàng đợi

# MT — Totals drawer (Phase 10)
footer-totals = Tổng
# MT
totals-title = Tổng
# MT
totals-loading = Đang tải tổng…
# MT
totals-card-bytes = Tổng byte đã sao chép
# MT
totals-card-files = Tệp
# MT
totals-card-jobs = Tác vụ
# MT
totals-card-avg-rate = Tốc độ trung bình
# MT
totals-errors = lỗi
# MT
totals-spark-title = 30 ngày qua
# MT
totals-kinds-title = Theo loại
# MT
totals-saved-title = Thời gian tiết kiệm (ước tính)
# MT
totals-saved-note = Ước tính so với bản sao chép tham chiếu cùng khối lượng bằng trình quản lý tệp tiêu chuẩn.
# MT
totals-reset = Đặt lại thống kê
# MT
totals-reset-confirm = Điều này xóa mọi tác vụ và mục đã lưu. Tiếp tục?
# MT
totals-reset-confirm-yes = Có, đặt lại
# MT
toast-totals-reset = Đã đặt lại thống kê

# MT — Phase 11a additions
header-language-label = Ngôn ngữ
# MT
header-language-title = Thay đổi ngôn ngữ

# MT
kind-copy = Sao chép
# MT
kind-move = Di chuyển
# MT
kind-delete = Xóa
# MT
kind-secure-delete = Xóa an toàn

# MT
status-running = Đang chạy
# MT
status-succeeded = Thành công
# MT
status-failed = Thất bại
# MT
status-cancelled = Đã hủy
# MT
status-ok = OK
# MT
status-skipped = Đã bỏ qua

# MT
history-search-placeholder = /đường-dẫn
# MT
toast-history-purged = Đã xóa { $count } tác vụ cũ hơn 30 ngày

# MT
err-source-required = Cần ít nhất một đường dẫn nguồn.
# MT
err-destination-empty = Đường dẫn đích đang trống.
# MT
err-source-empty = Đường dẫn nguồn đang trống.

# MT
duration-lt-1s = < 1 giây
# MT
duration-ms = { $ms } ms
# MT
duration-seconds = { $s } giây
# MT
duration-minutes-seconds = { $m } phút { $s } giây
# MT
duration-hours-minutes = { $h } giờ { $m } phút
# MT
duration-zero = 0 giây

# MT
rate-unit-per-second = { $size }/giây

# MT — Phase 11b Settings modal
settings-title = Cài đặt
# MT
settings-tab-general = Chung
# MT
settings-tab-appearance = Giao diện
# MT
settings-section-language = Ngôn ngữ
# MT
settings-phase-12-hint = Các cài đặt khác (chủ đề, mặc định truyền, thuật toán xác minh, hồ sơ) sẽ đến ở Giai đoạn 12.
