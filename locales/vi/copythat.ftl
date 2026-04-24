app-name = Copy That v1.25.0
# MT
window-title = Copy That v1.25.0
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
err-sparseness-mismatch = Không thể duy trì bố cục thưa tại đích  # MT

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
settings-preserve-sparseness = Bảo toàn tệp thưa  # MT
settings-preserve-sparseness-hint = Chỉ sao chép các phạm vi được cấp phát của tệp thưa (đĩa VM, tệp cơ sở dữ liệu) để kích thước trên đĩa tại đích giữ nguyên bằng với nguồn.  # MT

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

# Phase 19a — disk-backed file enumeration (TeraCopy-compatible scan DB)
# MT
scan-progress-title = Đang quét…
# MT
scan-progress-stats = { $files } tệp · { $bytes } đến nay
# MT
scan-pause-button = Tạm dừng quét
# MT
scan-resume-button = Tiếp tục quét
# MT
scan-cancel-button = Huỷ quét
# MT
scan-cancel-confirm = Huỷ quét và loại bỏ tiến độ?
# MT
scan-db-header = Cơ sở dữ liệu quét
# MT
scan-db-hint = Cơ sở dữ liệu quét trên đĩa cho các tác vụ có hàng triệu tệp.
# MT
advanced-scan-hash-during = Tính tổng kiểm tra trong khi quét
# MT
advanced-scan-db-path = Vị trí cơ sở dữ liệu quét
# MT
advanced-scan-retention-days = Tự động xoá các lượt quét đã hoàn tất sau (ngày)
# MT
advanced-scan-max-keep = Số cơ sở dữ liệu quét tối đa cần giữ

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
sparse-not-supported-title = Đích lấp đầy tệp thưa  # MT
sparse-not-supported-body = { $dst_fs } không hỗ trợ tệp thưa. Các lỗ trong nguồn đã được ghi dưới dạng số 0, do đó đích lớn hơn trên đĩa.  # MT
sparse-warning-densified = Bố cục thưa được bảo toàn: chỉ các phạm vi được cấp phát đã được sao chép.  # MT
sparse-warning-mismatch = Không khớp bố cục thưa — đích có thể lớn hơn mong đợi.  # MT

# Phase 24 — security-metadata preservation. MT-flagged drafts;
# the authoritative English source lives in locales/en/copythat.ftl.
settings-preserve-security-metadata = Bảo toàn siêu dữ liệu bảo mật  # MT
settings-preserve-security-metadata-hint = Bắt và áp dụng lại các luồng siêu dữ liệu ngoài băng (NTFS ADS / xattrs / ACL POSIX / ngữ cảnh SELinux / khả năng tệp Linux / nhánh tài nguyên macOS) trên mỗi bản sao.  # MT
settings-preserve-motw = Bảo toàn Mark-of-the-Web (cờ tải xuống từ internet)  # MT
settings-preserve-motw-hint = Quan trọng cho bảo mật. SmartScreen và Office Protected View sử dụng luồng này để cảnh báo về các tệp đã tải xuống từ internet. Vô hiệu hóa cho phép một tệp thực thi đã tải xuống mất dấu nguồn gốc khi sao chép và bỏ qua các biện pháp bảo vệ của hệ điều hành.  # MT
settings-preserve-posix-acls = Bảo toàn ACL POSIX và thuộc tính mở rộng  # MT
settings-preserve-posix-acls-hint = Mang theo các xattrs user.* / system.* / trusted.* và danh sách kiểm soát truy cập POSIX trong quá trình sao chép.  # MT
settings-preserve-selinux = Bảo toàn ngữ cảnh SELinux  # MT
settings-preserve-selinux-hint = Mang theo nhãn security.selinux trong quá trình sao chép để các tiến trình nền chạy dưới chính sách MAC vẫn có thể truy cập tệp.  # MT
settings-preserve-resource-forks = Bảo toàn nhánh tài nguyên macOS và thông tin Finder  # MT
settings-preserve-resource-forks-hint = Mang theo nhánh tài nguyên kế thừa và FinderInfo (thẻ màu, siêu dữ liệu Carbon) trong quá trình sao chép.  # MT
settings-appledouble-fallback = Sử dụng tệp đi kèm AppleDouble trên hệ thống tệp không tương thích  # MT
meta-translated-to-appledouble = Siêu dữ liệu nước ngoài được lưu trữ trong tệp đi kèm AppleDouble (._{ $ext })  # MT

# Phase 25 — two-way sync with vector-clock conflict detection.
# MT-flagged drafts; the authoritative English source lives in
# locales/en/copythat.ftl.
footer-sync = Đồng bộ  # MT
sync-drawer-title = Đồng bộ hai chiều  # MT
sync-drawer-hint = Giữ hai thư mục đồng bộ mà không ghi đè âm thầm. Các chỉnh sửa đồng thời xuất hiện dưới dạng xung đột có thể giải quyết.  # MT
sync-add-pair = Thêm cặp  # MT
sync-add-cancel = Hủy  # MT
sync-refresh = Làm mới  # MT
sync-add-save = Lưu cặp  # MT
sync-add-saving = Đang lưu…  # MT
sync-add-missing-fields = Nhãn, đường dẫn trái và đường dẫn phải đều bắt buộc.  # MT
sync-remove-confirm = Xóa cặp đồng bộ này? Cơ sở dữ liệu trạng thái được giữ lại; các thư mục không bị ảnh hưởng.  # MT
sync-field-label = Nhãn  # MT
sync-field-label-placeholder = vd. Tài liệu ↔ NAS  # MT
sync-field-left = Thư mục trái  # MT
sync-field-left-placeholder = Chọn hoặc dán đường dẫn tuyệt đối  # MT
sync-field-right = Thư mục phải  # MT
sync-field-right-placeholder = Chọn hoặc dán đường dẫn tuyệt đối  # MT
sync-field-mode = Chế độ  # MT
sync-mode-two-way = Hai chiều  # MT
sync-mode-mirror-left-to-right = Gương (trái → phải)  # MT
sync-mode-mirror-right-to-left = Gương (phải → trái)  # MT
sync-mode-contribute-left-to-right = Đóng góp (trái → phải, không xóa)  # MT
sync-no-pairs = Chưa có cặp đồng bộ nào được cấu hình. Nhấp "Thêm cặp" để bắt đầu.  # MT
sync-loading = Đang tải các cặp đã cấu hình…  # MT
sync-never-run = Chưa bao giờ chạy  # MT
sync-running = Đang chạy  # MT
sync-run-now = Chạy ngay  # MT
sync-cancel = Hủy  # MT
sync-remove-pair = Xóa  # MT
sync-view-conflicts = Xem xung đột ({ $count })  # MT
sync-conflicts-heading = Xung đột  # MT
sync-no-conflicts = Không có xung đột từ lần chạy cuối.  # MT
sync-winner = Người thắng  # MT
sync-side-left-to-right = trái  # MT
sync-side-right-to-left = phải  # MT
sync-conflict-kind-concurrent-write = Chỉnh sửa đồng thời  # MT
sync-conflict-kind-delete-edit = Xóa ↔ sửa  # MT
sync-conflict-kind-add-add = Cả hai bên đều thêm  # MT
sync-conflict-kind-corrupt-equal = Nội dung đã phân kỳ mà không có ghi mới  # MT
sync-resolve-keep-left = Giữ trái  # MT
sync-resolve-keep-right = Giữ phải  # MT
sync-resolve-keep-both = Giữ cả hai  # MT
sync-resolve-three-way = Giải quyết qua hợp nhất 3 chiều  # MT
sync-resolve-phase-53-tooltip = Hợp nhất 3 chiều tương tác cho các tệp không phải văn bản sẽ có trong Giai đoạn 53.  # MT
sync-error-prefix = Lỗi đồng bộ  # MT

# Phase 26 — real-time mirror watcher. MT-flagged drafts;
# the authoritative English source lives in locales/en/copythat.ftl.
live-mirror-start = Bắt đầu gương trực tiếp  # MT
live-mirror-stop = Dừng gương trực tiếp  # MT
live-mirror-watching = Đang theo dõi  # MT
live-mirror-toggle-hint = Tự động đồng bộ lại trên mọi thay đổi hệ thống tệp được phát hiện. Một luồng nền cho mỗi cặp hoạt động.  # MT
watch-event-prefix = Thay đổi tệp  # MT
watch-overflow-recovered = Bộ đệm theo dõi bị tràn; liệt kê lại để khôi phục  # MT

# Phase 27 — content-defined chunk store. MT-flagged drafts;
# the authoritative English source lives in locales/en/copythat.ftl.
chunk-store-section = Kho phân đoạn  # MT
chunk-store-enable = Kích hoạt kho phân đoạn (tiếp tục delta và loại bỏ trùng lặp)  # MT
chunk-store-enable-hint = Phân chia mỗi tệp được sao chép theo nội dung (FastCDC) và lưu trữ các phân đoạn được địa chỉ hóa theo nội dung. Các lần thử lại chỉ ghi lại các phân đoạn đã thay đổi; các tệp có nội dung chung được loại bỏ trùng lặp tự động.  # MT
chunk-store-location = Vị trí kho phân đoạn  # MT
chunk-store-max-size = Kích thước tối đa của kho phân đoạn  # MT
chunk-store-prune = Xóa các phân đoạn cũ hơn (ngày)  # MT
chunk-store-savings = Đã tiết kiệm { $gib } GiB nhờ loại bỏ trùng lặp phân đoạn  # MT
chunk-store-disk-usage = Đang sử dụng { $size } trong { $chunks } phân đoạn  # MT

# Phase 28 — tray-resident Drop Stack. MT-flagged drafts;
# the authoritative English source lives in locales/en/copythat.ftl.
dropstack-window-title = Drop Stack  # MT
dropstack-tray-open = Drop Stack  # MT
dropstack-empty-title = Drop Stack trống  # MT
dropstack-empty-hint = Kéo tệp vào đây từ Explorer hoặc nhấp chuột phải vào một dòng công việc để thêm.  # MT
dropstack-add-to-stack = Thêm vào Drop Stack  # MT
dropstack-copy-all-to = Sao chép tất cả đến…  # MT
dropstack-move-all-to = Di chuyển tất cả đến…  # MT
dropstack-clear = Xóa ngăn xếp  # MT
dropstack-remove-row = Xóa khỏi ngăn xếp  # MT
dropstack-path-missing-toast = Đã xóa { $path } — tệp không còn tồn tại.  # MT
dropstack-always-on-top = Luôn giữ Drop Stack ở trên cùng  # MT
dropstack-show-tray-icon = Hiển thị biểu tượng khay Copy That  # MT
dropstack-open-on-start = Tự động mở Drop Stack khi ứng dụng khởi động  # MT
dropstack-count = { $count } đường dẫn  # MT

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
