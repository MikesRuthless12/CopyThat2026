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
toast-clipboard-files-detected = Có tệp trên clipboard — nhấn phím tắt dán để sao chép qua Copy That
toast-clipboard-no-files = Clipboard không có tệp nào để dán
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
error-drawer-pending-count = Thêm lỗi đang chờ
error-drawer-toggle = Thu gọn hoặc mở rộng

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
err-path-escape = Đường dẫn bị từ chối — chứa đoạn thư mục cha (..) hoặc byte không hợp lệ
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

# MT — Phase 12 Settings window
settings-loading = Đang tải cài đặt…
# MT
settings-tab-transfer = Truyền tải
# MT
settings-tab-shell = Shell
# MT
settings-tab-secure-delete = Xóa an toàn
# MT
settings-tab-advanced = Nâng cao
# MT
settings-tab-profiles = Hồ sơ

# MT
settings-section-theme = Chủ đề
# MT
settings-theme-auto = Tự động
# MT
settings-theme-light = Sáng
# MT
settings-theme-dark = Tối
# MT
settings-start-with-os = Khởi chạy cùng hệ thống
# MT
settings-single-instance = Chỉ một phiên chạy
# MT
settings-minimize-to-tray = Thu nhỏ vào khay khi đóng
settings-error-display-mode = Kiểu thông báo lỗi
settings-error-display-modal = Hộp thoại (chặn ứng dụng)
settings-error-display-drawer = Ngăn kéo (không chặn)
settings-error-display-mode-hint = Hộp thoại dừng hàng đợi cho đến khi bạn quyết định. Ngăn kéo giữ hàng đợi chạy và cho phép bạn xử lý lỗi ở góc.
settings-paste-shortcut = Dán tệp qua phím tắt toàn hệ thống
settings-paste-shortcut-combo = Tổ hợp phím tắt
settings-paste-shortcut-hint = Nhấn tổ hợp này ở bất cứ đâu trên hệ thống để dán các tệp được sao chép từ Explorer / Finder / Files qua Copy That. CmdOrCtrl chuyển thành Cmd trên macOS và Ctrl trên Windows / Linux.
settings-clipboard-watcher = Theo dõi clipboard để phát hiện tệp được sao chép
settings-clipboard-watcher-hint = Hiển thị thông báo khi URL tệp xuất hiện trên clipboard, gợi ý rằng bạn có thể dán qua Copy That. Thăm dò mỗi 500 ms khi được bật.

# MT
settings-buffer-size = Kích thước bộ đệm
# MT
settings-verify = Xác minh sau khi sao chép
# MT
settings-verify-off = Tắt
# MT
settings-concurrency = Đồng thời
# MT
settings-concurrency-auto = Tự động
# MT
settings-reflink = Reflink / đường dẫn nhanh
# MT
settings-reflink-prefer = Ưu tiên
# MT
settings-reflink-avoid = Tránh reflink
# MT
settings-reflink-disabled = Luôn dùng engine bất đồng bộ
# MT
settings-fsync-on-close = Đồng bộ đĩa khi đóng (chậm hơn, an toàn hơn)
# MT
settings-preserve-timestamps = Giữ dấu thời gian
# MT
settings-preserve-permissions = Giữ quyền
# MT
settings-preserve-acls = Giữ ACL (Giai đoạn 14)

# MT
settings-context-menu = Bật các mục menu ngữ cảnh shell
# MT
settings-intercept-copy = Chặn trình xử lý sao chép mặc định (Windows)
# MT
settings-intercept-copy-hint = Khi bật, Ctrl+C / Ctrl+V trong Explorer đi qua Copy That. Đăng ký ở Giai đoạn 14.
# MT
settings-notify-completion = Thông báo khi hoàn thành tác vụ

# MT
settings-shred-method = Phương thức hủy mặc định
# MT
settings-shred-zero = Số không (1 lượt)
# MT
settings-shred-random = Ngẫu nhiên (1 lượt)
# MT
settings-shred-dod3 = DoD 5220.22-M (3 lượt)
# MT
settings-shred-dod7 = DoD 5220.22-M (7 lượt)
# MT
settings-shred-gutmann = Gutmann (35 lượt)
# MT
settings-shred-nist = NIST 800-88
# MT
settings-shred-confirm-twice = Yêu cầu xác nhận kép trước khi hủy

# MT
settings-log-level = Mức nhật ký
# MT
settings-log-off = Tắt
# MT
settings-telemetry = Thu thập dữ liệu
# MT
settings-telemetry-never = Không bao giờ — không gửi dữ liệu ở bất kỳ mức nhật ký nào
# MT
settings-error-policy = Chính sách lỗi mặc định
# MT
settings-error-policy-ask = Hỏi
# MT
settings-error-policy-skip = Bỏ qua
# MT
settings-error-policy-retry = Thử lại với chờ
# MT
settings-error-policy-abort = Hủy khi có lỗi đầu tiên
# MT
settings-history-retention = Lưu giữ lịch sử (ngày)
# MT
settings-history-retention-hint = 0 = giữ mãi. Bất kỳ giá trị nào khác sẽ tự động xóa tác vụ cũ khi khởi động.
# MT
settings-database-path = Đường dẫn cơ sở dữ liệu
# MT
settings-database-path-default = (mặc định — thư mục dữ liệu hệ điều hành)
# MT
settings-reset-all = Đặt lại về mặc định
# MT
settings-reset-confirm = Đặt lại mọi tùy chọn? Hồ sơ không bị ảnh hưởng.

# MT
settings-profiles-hint = Lưu cài đặt hiện tại dưới một tên; tải lại sau để chuyển đổi mà không cần chạm từng nút.
# MT
settings-profile-name-placeholder = Tên hồ sơ
# MT
settings-profile-save = Lưu
# MT
settings-profile-import = Nhập…
# MT
settings-profile-load = Tải
# MT
settings-profile-export = Xuất…
# MT
settings-profile-delete = Xóa
# MT
settings-profile-empty = Chưa có hồ sơ nào được lưu.
# MT
settings-profile-import-prompt = Tên cho hồ sơ nhập:

# MT
toast-settings-reset = Cài đặt đã đặt lại
# MT
toast-profile-saved = Đã lưu hồ sơ
# MT
toast-profile-loaded = Đã tải hồ sơ
# MT
toast-profile-exported = Đã xuất hồ sơ
# MT
toast-profile-imported = Đã nhập hồ sơ

# Phase 13d — activity feed + header picker buttons
action-add-files = Thêm tệp
action-add-folders = Thêm thư mục
activity-title = Hoạt động
activity-clear = Xoá danh sách hoạt động
activity-empty = Chưa có hoạt động tệp.
activity-after-done = Khi hoàn tất:
activity-keep-open = Giữ ứng dụng mở
activity-close-app = Đóng ứng dụng
activity-shutdown = Tắt máy
activity-logoff = Đăng xuất
activity-sleep = Ngủ

# Phase 14 — preflight free-space dialog
preflight-block-title = Không đủ dung lượng tại đích
preflight-warn-title = Ít dung lượng tại đích
preflight-unknown-title = Không thể xác định dung lượng trống
preflight-unknown-body = Nguồn quá lớn để đo nhanh hoặc đích không phản hồi. Bạn có thể tiếp tục; hệ thống sẽ dừng sao chép an toàn nếu hết dung lượng.
preflight-required = Cần
preflight-free = Trống
preflight-reserve = Dự trữ
preflight-shortfall = Thiếu
preflight-continue = Vẫn tiếp tục
collision-modal-overwrite-older = Chỉ ghi đè tệp cũ hơn

# Phase 14e — subset picker
preflight-pick-subset = Chọn mục cần sao chép…
subset-title = Chọn nguồn để sao chép
subset-subtitle = Toàn bộ lựa chọn không vừa tại đích. Đánh dấu những mục cần sao chép; phần còn lại sẽ bị bỏ qua.
subset-loading = Đang đo dung lượng…
subset-too-large = quá lớn để đếm
subset-budget = Khả dụng
subset-remaining = Còn lại
subset-confirm = Sao chép mục đã chọn
history-rerun-hint = Chạy lại bản sao này — quét lại mọi tệp trong cây nguồn
history-clear-all = Xoá tất cả
history-clear-all-confirm = Nhấp lần nữa để xác nhận
history-clear-all-hint = Xoá mọi dòng lịch sử. Cần nhấp lần hai để xác nhận.
toast-history-cleared = Đã xoá lịch sử (đã xoá { $count } dòng)

# Phase 15 — source-list ordering
drop-dialog-sort-label = Thứ tự:
sort-custom = Tùy chỉnh
sort-name-asc = Tên A → Z (tệp trước)
sort-name-desc = Tên Z → A (tệp trước)
sort-size-asc = Kích thước nhỏ đến lớn (tệp trước)
sort-size-desc = Kích thước lớn đến nhỏ (tệp trước)
sort-reorder = Sắp xếp lại
sort-move-top = Chuyển lên đầu
sort-move-up = Lên
sort-move-down = Xuống
sort-move-bottom = Chuyển xuống cuối
sort-name-asc-simple = Tên A → Z
sort-name-desc-simple = Tên Z → A
sort-size-asc-simple = Nhỏ nhất trước
sort-size-desc-simple = Lớn nhất trước
activity-sort-locked = Không thể sắp xếp khi đang sao chép. Tạm dừng hoặc đợi hoàn tất rồi thay đổi thứ tự.
drop-dialog-collision-label = Nếu tệp đã tồn tại:
collision-policy-keep-both = Giữ cả hai (đổi tên bản sao mới thành _2, _3, …)
collision-policy-skip = Bỏ qua bản sao mới
collision-policy-overwrite = Ghi đè tệp hiện có
collision-policy-overwrite-if-newer = Chỉ ghi đè nếu mới hơn
collision-policy-prompt = Hỏi mỗi lần
drop-dialog-busy-checking = Đang kiểm tra dung lượng trống…
drop-dialog-busy-enumerating = Đang đếm tệp…
drop-dialog-busy-starting = Đang bắt đầu sao chép…
toast-enumeration-deferred = Cây nguồn lớn — bỏ qua danh sách trước; các dòng sẽ xuất hiện khi bộ máy xử lý.

# Phase 14a — enumeration-time filters
# MT
settings-tab-filters = Bộ lọc
# MT
settings-filters-hint = Bỏ qua các tệp ngay khi liệt kê để bộ máy không phải mở chúng. Bao gồm chỉ áp dụng cho tệp; Loại trừ cũng cắt bỏ các thư mục khớp.
# MT
settings-filters-enabled = Bật bộ lọc cho sao chép cây
# MT
settings-filters-include-globs = Glob bao gồm
# MT
settings-filters-include-globs-placeholder = **/*.txt
# MT
settings-filters-include-globs-hint = Mỗi dòng một glob. Nếu không rỗng, tệp phải khớp ít nhất một. Luôn duyệt qua thư mục.
# MT
settings-filters-exclude-globs = Glob loại trừ
# MT
settings-filters-exclude-globs-placeholder = **/node_modules
# MT
settings-filters-exclude-globs-hint = Mỗi dòng một glob. Khớp sẽ cắt bỏ toàn bộ cây con đối với thư mục; tệp khớp sẽ bị bỏ qua.
# MT
settings-filters-size-range = Khoảng kích thước tệp
# MT
settings-filters-min-size-bytes = Kích thước tối thiểu (byte, trống = không giới hạn)
# MT
settings-filters-max-size-bytes = Kích thước tối đa (byte, trống = không giới hạn)
# MT
settings-filters-date-range = Khoảng thời gian sửa đổi
# MT
settings-filters-min-mtime = Sửa vào hoặc sau ngày
# MT
settings-filters-max-mtime = Sửa vào hoặc trước ngày
# MT
settings-filters-attributes = Thuộc tính
# MT
settings-filters-skip-hidden = Bỏ qua tệp / thư mục ẩn
# MT
settings-filters-skip-system = Bỏ qua tệp hệ thống (chỉ Windows)
# MT
settings-filters-skip-readonly = Bỏ qua tệp chỉ đọc

# Phase 15 — auto-update
# MT
settings-tab-updater = Cập nhật
# MT
settings-updater-hint = Copy That kiểm tra các bản cập nhật đã ký tối đa một lần mỗi ngày. Các bản cập nhật được cài đặt vào lần thoát ứng dụng tiếp theo.
# MT
settings-updater-auto-check = Kiểm tra cập nhật khi khởi động
# MT
settings-updater-channel = Kênh phát hành
# MT
settings-updater-channel-stable = Ổn định
# MT
settings-updater-channel-beta = Beta (phát hành trước)
# MT
settings-updater-last-check = Lần kiểm tra cuối
# MT
settings-updater-last-never = Không bao giờ
# MT
settings-updater-check-now = Kiểm tra cập nhật ngay
# MT
settings-updater-checking = Đang kiểm tra…
# MT
settings-updater-available = Có bản cập nhật
# MT
settings-updater-up-to-date = Bạn đang chạy phiên bản mới nhất.
# MT
settings-updater-dismiss = Bỏ qua phiên bản này
# MT
settings-updater-dismissed = Đã bỏ qua
# MT
toast-update-available = Có phiên bản mới hơn
# MT
toast-update-up-to-date = Bạn đã ở phiên bản mới nhất
