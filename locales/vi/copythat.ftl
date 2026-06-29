app-name = Copy That v0.19.84
window-title = Copy That v0.19.84
shred-ssd-advisory = Cảnh báo: ổ đích này là SSD. Ghi đè nhiều lượt không thể xóa sạch bộ nhớ flash một cách đáng tin cậy vì cơ chế cân bằng hao mòn và dự phòng dung lượng đẩy dữ liệu ra khỏi địa chỉ khối logic. Với ổ thể rắn, hãy dùng ATA SECURE ERASE, NVMe Format kèm Secure Erase, hoặc mã hóa toàn ổ rồi hủy khóa.

# Global aggregate states (header pill)
state-idle = Rảnh
state-copying = Đang sao chép
state-verifying = Đang xác minh
state-paused = Tạm dừng
state-error = Lỗi

# Per-job states (row badge)
state-pending = Trong hàng đợi
state-running = Đang chạy
state-cancelled = Đã hủy
state-succeeded = Xong
state-failed = Thất bại

# Actions
action-pause = Tạm dừng
action-resume = Tiếp tục
action-cancel = Hủy
action-pause-all = Tạm dừng mọi tác vụ
action-resume-all = Tiếp tục mọi tác vụ
action-cancel-all = Hủy mọi tác vụ
action-close = Đóng
action-reveal = Hiện trong thư mục
action-add-files = Thêm tệp
action-add-folders = Thêm thư mục

# Phase 13d — activity feed
activity-title = Hoạt động
activity-clear = Xóa danh sách hoạt động
activity-empty = Chưa có hoạt động tệp nào.
activity-after-done = Khi hoàn tất:
activity-keep-open = Giữ ứng dụng mở
activity-close-app = Đóng ứng dụng
activity-shutdown = Tắt máy tính
activity-logoff = Đăng xuất
activity-sleep = Ngủ

# Phase 14 — preflight free-space dialog
preflight-block-title = Không đủ dung lượng tại ổ đích
preflight-warn-title = Dung lượng ổ đích sắp hết
preflight-unknown-title = Không xác định được dung lượng trống
preflight-unknown-body = Nguồn quá lớn để tính nhanh kích thước hoặc ổ đích không phản hồi. Bạn vẫn có thể tiếp tục; bộ bảo vệ dung lượng của engine sẽ dừng việc sao chép một cách gọn gàng nếu hết chỗ.
preflight-required = Cần
preflight-free = Trống
preflight-reserve = Dự trữ
preflight-shortfall = Thiếu hụt
preflight-continue = Vẫn tiếp tục
preflight-pick-subset = Chọn mục cần sao chép…
collision-modal-overwrite-older = Chỉ ghi đè tệp cũ hơn

# Phase 14e — subset picker
subset-title = Chọn nguồn cần sao chép
subset-subtitle = Toàn bộ lựa chọn không vừa với ổ đích. Đánh dấu các mục bạn muốn sao chép; phần còn lại sẽ được giữ nguyên.
subset-loading = Đang đo kích thước…
subset-too-large = quá lớn để đếm
subset-budget = Khả dụng
subset-remaining = Còn lại
subset-confirm = Sao chép lựa chọn
history-rerun-hint = Chạy lại lần sao chép này — quét lại mọi tệp trong cây nguồn
history-clear-all = Xóa tất cả
history-clear-all-confirm = Nhấp lần nữa để xác nhận
history-clear-all-hint = Xóa mọi dòng lịch sử. Cần nhấp lần thứ hai để xác nhận.
toast-history-cleared = Đã xóa lịch sử (đã xóa { $count } dòng)

# Phase 15 — source-list ordering in the drop staging dialog
drop-dialog-sort-label = Thứ tự:
sort-custom = Tùy chỉnh
sort-name-asc = Tên A → Z (tệp trước)
sort-name-desc = Tên Z → A (tệp trước)
sort-size-asc = Nhỏ nhất trước (tệp trước)
sort-size-desc = Lớn nhất trước (tệp trước)
sort-reorder = Sắp xếp lại
sort-move-top = Đưa lên đầu
sort-move-up = Di chuyển lên
sort-move-down = Di chuyển xuống
sort-move-bottom = Đưa xuống cuối

# Phase 16 — sort preset names for the Activity list (shorter
# wording than the DropStagingDialog variants; the Activity list
# is files-only so the "(files first)" suffix isn't relevant).
sort-name-asc-simple = Tên A → Z
sort-name-desc-simple = Tên Z → A
sort-size-asc-simple = Nhỏ nhất trước
sort-size-desc-simple = Lớn nhất trước
activity-sort-locked = Không thể sắp xếp khi đang sao chép. Hãy tạm dừng hoặc chờ hoàn tất rồi đổi thứ tự.

# Phase 17 — collision-policy picker in the DropStagingDialog
drop-dialog-collision-label = Nếu tệp đã tồn tại:
collision-policy-keep-both = Giữ cả hai (đổi tên bản mới thành _2, _3, …)
collision-policy-skip = Bỏ qua bản mới
collision-policy-overwrite = Ghi đè tệp hiện có
collision-policy-overwrite-if-newer = Chỉ ghi đè nếu mới hơn
collision-policy-prompt = Hỏi mỗi lần

# Phase 18 — progress labels on the DropStagingDialog Start button
drop-dialog-busy-checking = Đang kiểm tra dung lượng trống…
drop-dialog-busy-enumerating = Đang đếm tệp…
drop-dialog-busy-starting = Đang bắt đầu sao chép…
toast-enumeration-deferred = Cây nguồn rất lớn — bỏ qua việc liệt kê tệp trước; các dòng sẽ xuất hiện khi engine xử lý dần.

# Context menu (per-row right-click)
menu-pause = Tạm dừng
menu-resume = Tiếp tục
menu-cancel = Hủy
menu-remove = Gỡ khỏi hàng đợi
menu-reveal-source = Hiện nguồn trong thư mục
menu-reveal-destination = Hiện đích trong thư mục

# Header / toolbar
header-eta-label = Thời gian còn lại ước tính
header-toolbar-label = Điều khiển toàn cục

# Footer
footer-queued = tác vụ đang hoạt động
footer-total-bytes = đang xử lý
footer-errors = lỗi
footer-history = Lịch sử

# Empty state
empty-title = Thả tệp hoặc thư mục để sao chép
empty-hint = Kéo các mục vào cửa sổ. Chúng tôi sẽ hỏi nơi đích, rồi tạo một tác vụ cho mỗi nguồn.
empty-region-label = Danh sách tác vụ

# Details drawer
details-drawer-label = Chi tiết tác vụ
details-source = Nguồn
details-destination = Đích
details-state = Trạng thái
details-bytes = Byte
details-files = Tệp
details-speed = Tốc độ
details-eta = Thời gian còn lại
details-error = Lỗi

# Drop dialog
drop-dialog-title = Chuyển các mục đã thả
drop-dialog-subtitle = { $count } mục sẵn sàng để chuyển. Chọn thư mục đích để bắt đầu.
drop-dialog-mode = Thao tác
drop-dialog-copy = Sao chép
drop-dialog-move = Di chuyển
drop-dialog-pick-destination = Chọn đích
drop-dialog-change-destination = Đổi đích
drop-dialog-start-copy = Bắt đầu sao chép
drop-dialog-start-move = Bắt đầu di chuyển

# ETA placeholders
eta-calculating = đang tính…
eta-unknown = không rõ

# Toast messages
toast-job-done = Đã hoàn tất chuyển
toast-copy-queued = Đã đưa sao chép vào hàng đợi
toast-move-queued = Đã đưa di chuyển vào hàng đợi
toast-error-resolved = Đã xử lý lỗi
toast-collision-resolved = Đã xử lý xung đột
toast-elevated-unavailable = Thử lại với quyền nâng cao sẽ có ở Phase 17 — chưa khả dụng
toast-clipboard-files-detected = Có tệp trong bộ nhớ tạm — nhấn phím dán để sao chép qua Copy That
toast-clipboard-no-files = Bộ nhớ tạm không có tệp để dán
toast-error-log-exported = Đã xuất nhật ký lỗi

# Error modal (Phase 8)
error-modal-title = Một lượt chuyển đã thất bại
error-modal-retry = Thử lại
error-modal-retry-elevated = Thử lại với quyền nâng cao
error-modal-skip = Bỏ qua
error-modal-skip-all-kind = Bỏ qua mọi lỗi loại này
error-modal-abort = Hủy tất cả
error-modal-path-label = Đường dẫn
error-modal-code-label = Mã
error-drawer-pending-count = Còn lỗi đang chờ
error-drawer-toggle = Thu gọn hoặc mở rộng

# Error-kind labels (Phase 8). Source of truth — engine maps each
# `CopyErrorKind` to one of these keys via `localized_key()`.
err-not-found = Không tìm thấy tệp
err-permission-denied = Bị từ chối quyền truy cập
err-disk-full = Ổ đĩa đích đã đầy
err-interrupted = Thao tác bị gián đoạn
err-verify-failed = Xác minh sau sao chép thất bại
err-path-escape = Đường dẫn bị từ chối — chứa đoạn thư mục cha (..) hoặc byte không hợp lệ
err-path-invalid-encoding = Đường dẫn bị từ chối — chuỗi chứa UTF-8 không hợp lệ / ký tự thay thế
err-helper-invalid-json = Trình trợ giúp đặc quyền nhận JSON sai định dạng; bỏ qua yêu cầu này
err-helper-grant-out-of-band = GrantCapabilities phải do vòng lặp chạy của trình trợ giúp xử lý, không phải trình xử lý không trạng thái
err-randomness-unavailable = Bộ tạo số ngẫu nhiên của hệ điều hành thất bại; không thể tạo ID phiên
err-sparseness-mismatch = Không thể giữ bố cục thưa trên ổ đích
err-io-other = Lỗi I/O không xác định

# Collision modal (Phase 8)
collision-modal-title = Tệp đã tồn tại
collision-modal-overwrite = Ghi đè
collision-modal-overwrite-if-newer = Ghi đè nếu mới hơn
collision-modal-skip = Bỏ qua
collision-modal-keep-both = Giữ cả hai
collision-modal-rename = Đổi tên…
collision-modal-apply-to-all = Áp dụng cho tất cả
collision-modal-source = Nguồn
collision-modal-destination = Đích
collision-modal-size = Kích thước
collision-modal-modified = Sửa đổi
collision-modal-hash-check = Băm nhanh (SHA-256)
collision-modal-hash-computing = Đang băm…
collision-modal-hash-identical = Giống nhau
collision-modal-hash-different = Khác nhau
collision-modal-rename-placeholder = Tên tệp mới
collision-modal-confirm-rename = Đổi tên

# Error log drawer (Phase 8)
error-log-title = Nhật ký lỗi
error-log-empty = Chưa ghi lỗi nào
error-log-export-csv = Xuất CSV
error-log-export-txt = Xuất văn bản
error-log-clear = Xóa nhật ký
error-log-col-time = Thời gian
error-log-col-job = Tác vụ
error-log-col-path = Đường dẫn
error-log-col-code = Mã
error-log-col-message = Thông báo
error-log-col-resolution = Cách xử lý

# History drawer (Phase 9)
history-title = Lịch sử
history-empty = Chưa ghi tác vụ nào
history-unavailable = Lịch sử sao chép không khả dụng. Ứng dụng không thể mở kho SQLite khi khởi động.
history-filter-any = bất kỳ
history-filter-kind = Loại
history-filter-status = Trạng thái
history-filter-text = Tìm kiếm
history-refresh = Làm mới
history-export-csv = Xuất CSV
history-purge-30 = Dọn các mục > 30 ngày
history-rerun = Chạy lại
history-detail-open = Chi tiết
history-detail-title = Chi tiết tác vụ
history-detail-empty = Không có mục nào được ghi
history-col-date = Ngày
history-col-kind = Loại
history-col-src = Nguồn
history-col-dst = Đích
history-col-files = Tệp
history-col-size = Kích thước
history-col-status = Trạng thái
history-col-duration = Thời lượng
history-col-error = Lỗi
toast-history-exported = Đã xuất lịch sử
toast-history-rerun-queued = Đã đưa lần chạy lại vào hàng đợi

# Totals drawer (Phase 10)
footer-totals = Tổng cộng
totals-title = Tổng cộng
totals-loading = Đang tải số liệu tổng…
totals-card-bytes = Tổng số byte đã sao chép
totals-card-files = Tệp
totals-card-jobs = Tác vụ
totals-card-avg-rate = Thông lượng trung bình
totals-errors = lỗi
totals-spark-title = 30 ngày qua
totals-kinds-title = Theo loại
totals-saved-title = Thời gian tiết kiệm (ước tính)
totals-saved-note = Ước tính so với việc sao chép cùng khối lượng bằng trình quản lý tệp thông thường.
totals-reset = Đặt lại thống kê
totals-reset-confirm = Thao tác này xóa mọi tác vụ và mục đã lưu. Tiếp tục?
totals-reset-confirm-yes = Có, đặt lại
toast-totals-reset = Đã đặt lại thống kê

# Phase 11a — i18n core: surface remaining user-visible strings so
# every string on a main-window screen flows through Fluent.

# Header language switcher (temporary placement — folds into
# Settings → General in Phase 12).
header-language-label = Ngôn ngữ
header-language-title = Đổi ngôn ngữ

# Job-kind labels. History filter, Totals breakdown, and history
# rows all map wire-format `kind` strings to these labels.
kind-copy = Sao chép
kind-move = Di chuyển
kind-delete = Xóa
kind-secure-delete = Xóa an toàn

# History status labels. Distinct from `state-*` because the
# history wire format uses plain `running`/`succeeded`/... rather
# than the live `JobState` enum the queue exposes.
status-running = Đang chạy
status-succeeded = Thành công
status-failed = Thất bại
status-cancelled = Đã hủy
# Per-item status (not job-level): `ok` / `skipped` are only
# reachable from the history detail view.
status-ok = OK
status-skipped = Đã bỏ qua

# History drawer: search field placeholder + purge toast.
history-search-placeholder = /đường-dẫn
toast-history-purged = Đã dọn { $count } tác vụ cũ hơn 30 ngày

# User-facing command-layer validation errors. The Rust side
# returns these keys when input is missing; the toast layer looks
# them up rather than showing the raw English.
err-source-required = Cần ít nhất một đường dẫn nguồn.
err-destination-empty = Đường dẫn đích đang trống.
err-source-empty = Đường dẫn nguồn đang trống.

# Localised duration formatting for the Totals drawer and ETA
# fields. `{ $ms }`, `{ $s }`, `{ $m }`, `{ $h }` are integer
# placeables — the formatter passes pre-computed values in.
duration-lt-1s = < 1 giây
duration-ms = { $ms } mili giây
duration-seconds = { $s } giây
duration-minutes-seconds = { $m } phút { $s } giây
duration-hours-minutes = { $h } giờ { $m } phút
duration-zero = 0 giây

# Rate unit. Appended to a formatted byte size. Some languages
# render this with a leading space ("Ko/s"); keep it translatable
# even though the SI-derived "/s" is near-universal.
rate-unit-per-second = { $size }/giây

# Phase 11b — Settings modal skeleton. Phase 12 expanded this into a
# full six-tab preferences window; the `settings-phase-12-hint` key
# is retired in favour of `settings-tab-profiles` + concrete labels.
settings-title = Cài đặt
settings-tab-general = Chung
settings-tab-appearance = Giao diện
settings-section-language = Ngôn ngữ
settings-phase-12-hint = Thêm cài đặt (chủ đề, mặc định chuyển, thuật toán xác minh, hồ sơ) sẽ có ở Phase 12.

# Phase 12 — full Settings window keys. Grouped by tab so the
# human-review pass can pick one section at a time. Every field
# label and every option label that the user will actually read
# has its own key; dynamic wire values (enum discriminants like
# "auto" / "prefer") stay untranslated on the wire.

settings-loading = Đang tải cài đặt…
settings-tab-transfer = Chuyển
settings-tab-filters = Bộ lọc
settings-tab-shell = Shell
settings-tab-secure-delete = Xóa an toàn
settings-tab-advanced = Nâng cao
settings-tab-updater = Cập nhật
settings-tab-profiles = Hồ sơ

# General tab additions
settings-section-theme = Chủ đề
settings-theme-auto = Tự động
settings-theme-light = Sáng
settings-theme-dark = Tối
settings-start-with-os = Khởi chạy khi mở máy
settings-single-instance = Một phiên bản chạy duy nhất
settings-minimize-to-tray = Thu nhỏ xuống khay khi đóng
settings-error-display-mode = Kiểu hiển thị lỗi
settings-error-display-modal = Hộp thoại (chặn ứng dụng)
settings-error-display-drawer = Ngăn kéo (không chặn)
settings-error-display-mode-hint = Hộp thoại dừng hàng đợi cho đến khi bạn quyết định. Ngăn kéo giữ hàng đợi chạy và cho phép bạn xử lý lỗi ở góc màn hình.
settings-paste-shortcut = Dán tệp bằng phím tắt toàn cục
settings-paste-shortcut-combo = Tổ hợp phím tắt
settings-paste-shortcut-hint = Nhấn tổ hợp này ở bất kỳ đâu trên hệ thống để dán các tệp đã sao chép từ Explorer / Finder / Files qua Copy That. CmdOrCtrl tương ứng với Cmd trên macOS, Ctrl trên Windows / Linux.
settings-clipboard-watcher = Theo dõi bộ nhớ tạm cho các tệp đã sao chép
settings-clipboard-watcher-hint = Hiển thị thông báo khi có URL tệp xuất hiện trong bộ nhớ tạm, gợi ý rằng bạn có thể dán qua Copy That. Kiểm tra mỗi 500 ms khi bật.

# Transfer tab
settings-buffer-size = Kích thước bộ đệm
settings-verify = Xác minh sau khi sao chép
settings-verify-off = Tắt
settings-concurrency = Số luồng đồng thời
settings-concurrency-auto = Tự động
settings-reflink = Reflink / đường dẫn nhanh
settings-reflink-prefer = Ưu tiên
settings-reflink-avoid = Tránh reflink
settings-reflink-disabled = Luôn dùng engine bất đồng bộ
settings-fsync-on-close = Đồng bộ xuống đĩa khi đóng (chậm hơn, an toàn hơn)
settings-preserve-timestamps = Giữ dấu thời gian
settings-preserve-permissions = Giữ quyền truy cập
settings-preserve-acls = Giữ ACL (Phase 14)
settings-preserve-sparseness = Giữ tệp thưa
settings-preserve-sparseness-hint = Chỉ sao chép các vùng đã cấp phát của tệp thưa (đĩa máy ảo, tệp cơ sở dữ liệu) để ổ đích giữ cùng kích thước trên đĩa như nguồn.
settings-force-parallel-chunks = Sao chép song song nhiều khối (chỉ RAID / mảng đĩa)
settings-force-parallel-chunks-hint = Chia mỗi lần sao chép lớn thành các khối đồng thời. Chỉ có lợi cho đích phân dải/RAID/mạng; LÀM CHẬM một SSD/NVMe đơn (-25% đến -76%). Hãy tắt trừ khi đích là mảng nhiều đĩa.

# Shell tab
settings-context-menu = Bật mục menu ngữ cảnh của shell
settings-intercept-copy = Chặn trình xử lý sao chép mặc định (Windows)
settings-intercept-copy-hint = Khi bật, Ctrl+C / Ctrl+V của Explorer sẽ đi qua Copy That. Việc đăng ký sẽ có ở Phase 14.
settings-notify-completion = Thông báo khi hoàn tất tác vụ

# Secure delete tab
settings-shred-method = Phương pháp hủy mặc định
settings-shred-zero = Số không (1 lượt)
settings-shred-random = Ngẫu nhiên (1 lượt)
settings-shred-dod3 = DoD 5220.22-M (3 lượt)
settings-shred-dod7 = DoD 5220.22-M (7 lượt)
settings-shred-gutmann = Gutmann (35 lượt)
settings-shred-nist = NIST 800-88
settings-shred-confirm-twice = Yêu cầu xác nhận hai lần trước khi hủy

# Advanced tab
settings-log-level = Cấp độ nhật ký
settings-log-off = Tắt
settings-telemetry = Đo lường từ xa
settings-telemetry-never = Không bao giờ — không gửi dữ liệu về ở bất kỳ cấp độ nhật ký nào
settings-error-policy = Chính sách lỗi mặc định
settings-error-policy-ask = Hỏi
settings-error-policy-skip = Bỏ qua
settings-error-policy-retry = Thử lại với độ trễ tăng dần
settings-error-policy-abort = Hủy ngay khi gặp lỗi đầu tiên
settings-history-retention = Thời gian lưu lịch sử (ngày)
settings-history-retention-hint = 0 = giữ mãi mãi. Mọi giá trị khác sẽ tự động dọn các tác vụ cũ khi khởi động.
settings-database-path = Đường dẫn cơ sở dữ liệu
settings-database-path-default = (mặc định — thư mục dữ liệu của hệ điều hành)
settings-reset-all = Đặt lại về mặc định
settings-reset-confirm = Đặt lại mọi tùy chọn về mặc định? Các hồ sơ không bị ảnh hưởng.

# Profiles tab
settings-profiles-hint = Lưu cài đặt hiện tại dưới một tên; tải lại sau để khôi phục mà không cần chỉnh từng tùy chọn.
settings-profile-name-placeholder = Tên hồ sơ
settings-profile-save = Lưu
settings-profile-import = Nhập…
settings-profile-load = Tải
settings-profile-export = Xuất…
settings-profile-delete = Xóa
settings-profile-empty = Chưa lưu hồ sơ nào.
settings-profile-import-prompt = Tên cho hồ sơ được nhập:

# Toasts driven by Phase 12 profile actions
toast-settings-reset = Đã đặt lại cài đặt
toast-profile-saved = Đã lưu hồ sơ
toast-profile-loaded = Đã tải hồ sơ
toast-profile-exported = Đã xuất hồ sơ
toast-profile-imported = Đã nhập hồ sơ

# Phase 14a — enumeration-time filters
settings-filters-hint = Bỏ qua tệp ngay khi liệt kê để engine không bao giờ mở chúng. Các mục "bao gồm" chỉ áp dụng cho tệp; các mục "loại trừ" còn cắt bỏ cả thư mục khớp.
settings-filters-enabled = Bật bộ lọc cho việc sao chép cây thư mục
settings-filters-include-globs = Mẫu bao gồm (glob)
settings-filters-include-globs-placeholder = **/*.txt
settings-filters-include-globs-hint = Mỗi dòng một mẫu glob. Khi không trống, tệp phải khớp ít nhất một mẫu bao gồm mới được giữ lại. Luôn duyệt vào các thư mục.
settings-filters-exclude-globs = Mẫu loại trừ (glob)
settings-filters-exclude-globs-placeholder = **/node_modules
settings-filters-exclude-globs-hint = Mỗi dòng một mẫu glob. Khớp sẽ cắt bỏ toàn bộ cây con với thư mục; các tệp khớp sẽ bị bỏ qua.
settings-filters-size-range = Khoảng kích thước tệp
settings-filters-min-size-bytes = Kích thước tối thiểu (byte, để trống = không giới hạn dưới)
settings-filters-max-size-bytes = Kích thước tối đa (byte, để trống = không giới hạn trên)
settings-filters-date-range = Khoảng thời gian sửa đổi
settings-filters-min-mtime = Sửa đổi vào hoặc sau
settings-filters-max-mtime = Sửa đổi vào hoặc trước
settings-filters-attributes = Bit thuộc tính
settings-filters-skip-hidden = Bỏ qua tệp / thư mục ẩn
settings-filters-skip-system = Bỏ qua tệp hệ thống (chỉ Windows)
settings-filters-skip-readonly = Bỏ qua tệp chỉ đọc

# Phase 15 — auto-update
settings-updater-hint = Copy That kiểm tra bản cập nhật đã ký tối đa một lần mỗi ngày. Cập nhật cài đặt vào lần thoát ứng dụng tiếp theo.
settings-updater-auto-check = Kiểm tra cập nhật khi khởi chạy
settings-updater-channel = Kênh phát hành
settings-updater-channel-stable = Ổn định
settings-updater-channel-beta = Beta (bản thử nghiệm)
settings-updater-last-check = Kiểm tra lần cuối
settings-updater-last-never = Chưa bao giờ
settings-updater-check-now = Kiểm tra cập nhật ngay
settings-updater-checking = Đang kiểm tra…
settings-updater-available = Có bản cập nhật
settings-updater-up-to-date = Bạn đang dùng bản phát hành mới nhất.
settings-updater-dismiss = Bỏ qua phiên bản này
settings-updater-dismissed = Đã bỏ qua
toast-update-available = Đã có phiên bản mới hơn
toast-update-up-to-date = Bạn đã dùng phiên bản mới nhất

# Phase 19a — disk-backed file enumeration (TeraCopy-compatible scan DB)
scan-progress-title = Đang quét…
scan-progress-stats = { $files } tệp · { $bytes } đến giờ
scan-pause-button = Tạm dừng quét
scan-resume-button = Tiếp tục quét
scan-cancel-button = Hủy quét
scan-cancel-confirm = Hủy quét và bỏ tiến trình?
scan-db-header = Cơ sở dữ liệu quét
scan-db-hint = Cơ sở dữ liệu quét trên đĩa cho các tác vụ hàng triệu tệp.
advanced-scan-hash-during = Tính checksum trong khi quét
advanced-scan-db-path = Vị trí cơ sở dữ liệu quét
advanced-scan-retention-days = Tự xóa các lần quét đã xong sau (ngày)
advanced-scan-max-keep = Số cơ sở dữ liệu quét tối đa được giữ

# Phase 19b — filesystem-snapshot source for locked files.
settings-on-locked = Khi một tệp bị khóa
settings-on-locked-ask = Hỏi vào lần đầu
settings-on-locked-retry = Thử lại trong giây lát, rồi báo lỗi
settings-on-locked-skip = Bỏ qua tệp bị khóa
settings-on-locked-snapshot = Dùng ảnh chụp hệ thống tệp
settings-on-locked-hint = Loại bỏ lỗi "tệp đang được tiến trình khác sử dụng". Copy That chụp ảnh ổ nguồn (VSS trên Windows, ZFS/Btrfs trên Linux, APFS trên macOS) và đọc từ bản ảnh chụp.
snapshot-prompt-title = Tệp này đang được một tiến trình khác sử dụng
snapshot-prompt-body = Một chương trình khác đang mở { $path } để ghi độc quyền. Hãy chọn cách Copy That xử lý tệp này và các tệp tương tự trên cùng ổ đĩa.
snapshot-source-active = 📷 Đang đọc từ ảnh chụp { $kind } của { $volume }
snapshot-create-failed = Không thể tạo ảnh chụp của ổ nguồn
snapshot-vss-needs-elevation = Đọc từ ảnh chụp VSS cần quyền Administrator. Copy That sẽ xin phép bạn cho phép.
snapshot-cleanup-failed = Trình trợ giúp ảnh chụp báo lỗi dọn dẹp — một bản sao bóng còn sót có thể vẫn còn trên ổ đĩa.

# Phase 20 — durable resume journal.
resume-prompt-title = Tiếp tục các lượt chuyển trước đó?
resume-prompt-body = Copy That phát hiện { $count } lượt chuyển chưa hoàn tất từ phiên trước. Hãy chọn việc cần làm với từng lượt.
resume-prompt-resume = Tiếp tục
resume-prompt-resume-all = Tiếp tục tất cả
resume-discard-one = Không tiếp tục
resume-discard-all = Bỏ tất cả
resume-aborted-hash-mismatch = { $offset } byte đầu của đích không khớp với nguồn — bắt đầu lại từ đầu.
settings-auto-resume = Tự động tiếp tục các tác vụ bị gián đoạn mà không hỏi
settings-auto-resume-hint = Bỏ qua lời nhắc tiếp tục khi khởi động và lặng lẽ đưa lại mọi tác vụ chưa hoàn tất vào hàng đợi. Mặc định tắt.

# Phase 21 — bandwidth shaping (GCRA token bucket + schedule + auto-throttle).
settings-tab-network = Mạng
settings-network-hint = Giới hạn tốc độ chuyển để phần còn lại của mạng vẫn dùng được. Áp dụng toàn cục, theo lịch hằng ngày, hoặc tự động phản ứng với Wi-Fi giới hạn / pin / kết nối di động.
settings-network-mode = Giới hạn băng thông
settings-network-mode-off = Tắt (không giới hạn)
settings-network-mode-fixed = Giá trị cố định
settings-network-mode-schedule = Dùng lịch
settings-network-cap-mbps = Giới hạn (MB/giây)
settings-network-schedule = Lịch (định dạng rclone)
settings-network-schedule-hint = Các mốc HH:MM,tốc-độ cách nhau bằng khoảng trắng cùng tùy chọn quy tắc theo ngày Mon-Fri,tốc-độ. Tốc độ: 512k, 10M, 2G, off, unlimited. Ví dụ: 08:00,512k 18:00,10M Sat-Sun,unlimited.
settings-network-auto-header = Tự động điều tiết
settings-network-auto-metered = Trên Wi-Fi giới hạn
settings-network-auto-battery = Khi dùng pin
settings-network-auto-cellular = Trên mạng di động
settings-network-auto-unchanged = Không ghi đè
settings-network-auto-pause = Tạm dừng các lượt chuyển
settings-network-auto-cap = Giới hạn ở giá trị cố định
shape-badge-paused = đã tạm dừng
shape-badge-tooltip = Giới hạn băng thông đang bật — nhấp để mở Cài đặt → Mạng
shape-badge-source-schedule = theo lịch
shape-badge-source-metered = giới hạn
shape-badge-source-battery = dùng pin
shape-badge-source-cellular = di động
shape-badge-source-settings = đang bật
shape-error-schedule-invalid = Định dạng lịch không hợp lệ: { $message }

# Phase 22 — aggregate conflict dialog v2 (thumbnails, per-pattern
# rules, and reusable conflict profiles). Every key below is user-
# visible text in the `ConflictBatchModal.svelte` component.
conflict-batch-title = { $count } xung đột tệp trong { $jobname }
conflict-batch-state-pending = Đang chờ
conflict-batch-state-resolved = Đã xử lý
conflict-batch-action-overwrite = Ghi đè
conflict-batch-action-skip = Bỏ qua
conflict-batch-action-keep-both = Giữ cả hai
conflict-batch-action-newer-wins = Bản mới hơn thắng
conflict-batch-action-larger-wins = Bản lớn hơn thắng
conflict-batch-bulk-apply-selected = Áp dụng cho mục đã chọn
conflict-batch-bulk-apply-extension = Áp dụng cho mọi tệp cùng phần mở rộng này
conflict-batch-bulk-apply-glob = Áp dụng cho glob khớp…
conflict-batch-bulk-apply-remaining = Áp dụng cho tất cả còn lại
conflict-batch-bulk-glob-placeholder = ví dụ **/*.tmp
conflict-batch-save-profile = Lưu các quy tắc này thành hồ sơ…
conflict-batch-profile-placeholder = Tên hồ sơ
conflict-batch-matched-rule = qua quy tắc '{ $rule }' → { $action }
conflict-batch-empty = Đã xử lý mọi xung đột
conflict-batch-source-vs-destination = Nguồn vs. đích
conflict-batch-source-label = Nguồn
conflict-batch-destination-label = Đích
conflict-batch-size-label = Kích thước
conflict-batch-modified-label = Sửa đổi
conflict-batch-close = Đóng
conflict-batch-profile-saved = Đã lưu hồ sơ xung đột

# Phase 23 — sparse-file preservation. The toast fires once per
# destination volume when the filesystem can't preserve holes; the
# warning line is surfaced in the job detail drawer so the user knows
# the dst is larger on disk than the source was.
sparse-not-supported-title = Đích lấp đầy tệp thưa
sparse-not-supported-body = { $dst_fs } không hỗ trợ tệp thưa. Các lỗ trống trong nguồn được ghi ra dưới dạng số không, nên đích chiếm nhiều dung lượng đĩa hơn.
sparse-warning-densified = Đã giữ bố cục thưa: chỉ sao chép các vùng đã cấp phát.
sparse-warning-mismatch = Bố cục thưa không khớp — đích có thể lớn hơn dự kiến.

# Phase 24 — security-metadata preservation. The Mark-of-the-Web
# (Zone.Identifier ADS) toggle is security-sensitive: turning it off
# lets a downloaded executable shed its SmartScreen / Office Protected
# View flag on copy, which is why the tooltip carries an explicit
# warning. AppleDouble fallback emits `._<filename>` sidecars on
# destination filesystems that can't hold the foreign metadata.
settings-preserve-security-metadata = Giữ siêu dữ liệu bảo mật
settings-preserve-security-metadata-hint = Ghi lại và áp dụng lại các luồng siêu dữ liệu ngoài luồng (NTFS ADS / xattr / POSIX ACL / ngữ cảnh SELinux / khả năng tệp Linux / resource fork macOS) cho mỗi lần sao chép.
settings-preserve-motw = Giữ Mark-of-the-Web (cờ tải từ internet)
settings-preserve-motw-hint = Quan trọng cho bảo mật. SmartScreen và Office Protected View dùng luồng này để cảnh báo về các tệp tải từ internet. Tắt nó cho phép tệp thực thi đã tải bỏ dấu nguồn gốc khi sao chép và vượt qua các biện pháp bảo vệ của hệ điều hành.
settings-preserve-posix-acls = Giữ POSIX ACL và thuộc tính mở rộng
settings-preserve-posix-acls-hint = Mang theo xattr user.* / system.* / trusted.* và danh sách kiểm soát truy cập POSIX qua lần sao chép.
settings-preserve-selinux = Giữ ngữ cảnh SELinux
settings-preserve-selinux-hint = Mang theo nhãn security.selinux qua lần sao chép để các tiến trình chạy dưới chính sách MAC vẫn truy cập được tệp.
settings-preserve-resource-forks = Giữ resource fork và thông tin Finder của macOS
settings-preserve-resource-forks-hint = Mang theo resource fork cũ và FinderInfo (thẻ màu, siêu dữ liệu Carbon) qua lần sao chép.
settings-appledouble-fallback = Dùng tệp đồng hành AppleDouble trên hệ thống tệp không tương thích
meta-translated-to-appledouble = Siêu dữ liệu ngoài đã lưu trong tệp đồng hành AppleDouble (._{ $ext })

# Phase 25 — two-way sync with vector-clock conflict detection.
# The drawer lists configured sync pairs; each pair runs an
# independent reconciliation round against a per-pair `.copythat-sync.db`
# state store. Concurrent edits from a common ancestor surface as
# conflicts rather than silent overwrites; the losing side's content
# is preserved as `name.sync-conflict-YYYYMMDD-HHMMSS-<host>.ext`.
footer-sync = Đồng bộ
sync-drawer-title = Đồng bộ hai chiều
sync-drawer-hint = Giữ hai thư mục đồng bộ mà không ghi đè ngầm. Các chỉnh sửa đồng thời sẽ hiện thành xung đột để bạn xử lý.
sync-add-pair = Thêm cặp
sync-add-cancel = Hủy
sync-refresh = Làm mới
sync-add-save = Lưu cặp
sync-add-saving = Đang lưu…
sync-add-missing-fields = Nhãn, đường dẫn trái và đường dẫn phải đều bắt buộc.
sync-remove-confirm = Gỡ cặp đồng bộ này? Cơ sở dữ liệu trạng thái được giữ lại; các thư mục không bị động đến.
sync-field-label = Nhãn
sync-field-label-placeholder = ví dụ Tài liệu ↔ NAS
sync-field-left = Thư mục trái
sync-field-left-placeholder = Chọn hoặc dán một đường dẫn tuyệt đối
sync-field-right = Thư mục phải
sync-field-right-placeholder = Chọn hoặc dán một đường dẫn tuyệt đối
sync-field-mode = Chế độ
sync-mode-two-way = Hai chiều
sync-mode-mirror-left-to-right = Phản chiếu (trái → phải)
sync-mode-mirror-right-to-left = Phản chiếu (phải → trái)
sync-mode-contribute-left-to-right = Đóng góp (trái → phải, không xóa)
sync-no-pairs = Chưa cấu hình cặp đồng bộ nào. Nhấp "Thêm cặp" để bắt đầu.
sync-loading = Đang tải các cặp đã cấu hình…
sync-never-run = Chưa chạy lần nào
sync-running = Đang chạy
sync-run-now = Chạy ngay
sync-cancel = Hủy
sync-remove-pair = Gỡ
sync-view-conflicts = Xem xung đột ({ $count })
sync-conflicts-heading = Xung đột
sync-no-conflicts = Không có xung đột từ lần chạy gần nhất.
sync-winner = Bên thắng
sync-side-left-to-right = trái
sync-side-right-to-left = phải
sync-conflict-kind-concurrent-write = Chỉnh sửa đồng thời
sync-conflict-kind-delete-edit = Xóa ↔ chỉnh sửa
sync-conflict-kind-add-add = Cả hai bên đều thêm
sync-conflict-kind-corrupt-equal = Nội dung phân kỳ mà không có lần ghi mới
sync-resolve-keep-left = Giữ bên trái
sync-resolve-keep-right = Giữ bên phải
sync-resolve-keep-both = Giữ cả hai
sync-resolve-three-way = Xử lý bằng hợp nhất 3 chiều
sync-resolve-phase-53-tooltip = Hợp nhất 3 chiều tương tác cho tệp không phải văn bản sẽ có ở Phase 53.
sync-error-prefix = Lỗi đồng bộ

# Phase 26 — real-time mirror watcher. "Live mirror" starts a
# filesystem watcher on the pair's left side; every debounced event
# triggers a re-sync. The watcher filters vim swap files / Office
# lock files / atomic-save temp names so one logical save becomes
# exactly one sync round.
live-mirror-start = Bắt đầu phản chiếu trực tiếp
live-mirror-stop = Dừng phản chiếu trực tiếp
live-mirror-watching = Đang theo dõi
live-mirror-toggle-hint = Tự động đồng bộ lại trên mỗi thay đổi hệ thống tệp được phát hiện. Mỗi cặp đang hoạt động có một luồng nền.
watch-event-prefix = Thay đổi tệp
watch-overflow-recovered = Bộ đệm theo dõi bị tràn; đang liệt kê lại để khôi phục

# Phase 27 — content-defined chunk store. Enables delta-resume (a
# retry only re-writes chunks that actually changed) and same-job
# dedup (files sharing content blocks store those blocks once). The
# store is disk-backed under `<data-dir>/chunks/` by default and is
# the foundation for the Phase 49–51 moonshot repository phases.
chunk-store-section = Kho khối
chunk-store-enable = Bật kho khối (tiếp tục theo delta và khử trùng lặp)
chunk-store-enable-hint = Chia mỗi tệp đã sao chép theo nội dung (FastCDC) và lưu các khối theo địa chỉ nội dung. Lần thử lại chỉ ghi lại các khối đã thay đổi; các tệp có nội dung chung sẽ tự động khử trùng lặp.
chunk-store-location = Vị trí kho khối
chunk-store-max-size = Kích thước kho khối tối đa
chunk-store-prune = Dọn các khối cũ hơn (ngày)
chunk-store-savings = Đã tiết kiệm { $gib } GiB nhờ khử trùng lặp khối
chunk-store-disk-usage = Đang dùng { $size } trên { $chunks } khối

# Phase 28 — tray-resident Drop Stack. The stack is a persistent list
# of paths gathered from multiple sources (Explorer drag, main-window
# context menu, CLI, drag onto the Drop Stack window) that the user
# can dispatch to a destination in one go.
dropstack-window-title = Drop Stack
dropstack-tray-open = Drop Stack
dropstack-empty-title = Drop Stack đang trống
dropstack-empty-hint = Kéo tệp vào đây từ Explorer hoặc nhấp chuột phải vào một dòng tác vụ để thêm.
dropstack-add-to-stack = Thêm vào Drop Stack
dropstack-copy-all-to = Sao chép tất cả tới…
dropstack-move-all-to = Di chuyển tất cả tới…
dropstack-clear = Xóa stack
dropstack-remove-row = Gỡ khỏi stack
dropstack-path-missing-toast = Đã thả { $path } — tệp không còn tồn tại.
dropstack-always-on-top = Luôn giữ Drop Stack trên cùng
dropstack-show-tray-icon = Hiển thị biểu tượng khay Copy That
dropstack-open-on-start = Tự động mở Drop Stack khi khởi động ứng dụng
dropstack-count = { $count } đường dẫn

# Phase 29 — spring-loaded folders + native DnD polish. The Settings
# → General tab carries the knobs; DropTarget / DestinationPicker
# Svelte components use the dropzone-* keys at runtime.
settings-dnd-heading = Kéo và thả
settings-dnd-spring-load = Tự mở thư mục khi đang kéo
settings-dnd-spring-delay = Độ trễ tự mở thư mục (ms)
settings-dnd-thumbnails = Hiển thị ảnh thu nhỏ khi kéo
settings-dnd-invalid-highlight = Làm nổi bật đích thả không hợp lệ
dropzone-invalid-title = Không phải đích thả hợp lệ
dropzone-invalid-readonly = Đích chỉ đọc
dropzone-picker-title = Chọn một đích
dropzone-picker-up = Lên
dropzone-picker-path = Đường dẫn hiện tại
dropzone-picker-root = Gốc
dropzone-picker-use-this = Dùng thư mục này
dropzone-picker-empty = Không có thư mục con
dropzone-picker-cancel = Hủy

# Phase 30 — cross-platform path translation. Settings → Transfer
# exposes these under a "Cross-platform compatibility" subsection.
translate-heading = Tương thích đa nền tảng
translate-unicode-label = Chuẩn hóa Unicode
translate-unicode-auto = Tự phát hiện đích
translate-unicode-windows = NFC (Windows / Linux)
translate-unicode-macos = Giữ nguyên (macOS / APFS)
translate-line-endings-label = Chuyển ký tự xuống dòng cho tệp văn bản
translate-line-endings-allowlist = Phần mở rộng tệp văn bản
reserved-name-label = Xử lý tên dành riêng của Windows
reserved-name-suffix = Thêm "_" (CON.txt → CON_.txt)
reserved-name-reject = Từ chối và cảnh báo
long-path-label = Dùng tiền tố đường dẫn dài của Windows (\\?\) khi vượt 260 ký tự
long-path-hint = Một số thư mục dùng chung qua mạng và công cụ cũ không hỗ trợ không gian tên \\?\.

# Phase 31 — power-aware copying. Settings → Power & State tab + the
# header badge that renders "⏸ Paused — Zoom call detected" when the
# runner's power subscriber has paused or capped due to a policy match.
power-heading = Nguồn & Trạng thái
power-enabled = Bật các quy tắc nhận biết nguồn điện
power-battery-label = Khi dùng pin
power-metered-label = Trên Wi-Fi giới hạn
power-cellular-label = Trên mạng di động
power-presentation-label = Khi đang trình bày (Zoom / Teams / Keynote)
power-fullscreen-label = Khi một ứng dụng đang toàn màn hình
power-thermal-label = Khi CPU bị giảm tốc do nhiệt
power-rule-continue = Tiếp tục ở tốc độ tối đa
power-rule-pause = Tạm dừng mọi tác vụ
power-rule-cap = Giới hạn băng thông
power-rule-cap-percent = Giới hạn ở một phần trăm tốc độ hiện tại
power-reason-on-battery = dùng pin
power-reason-metered-network = mạng giới hạn
power-reason-cellular-network = mạng di động
power-reason-presenting = chế độ trình bày
power-reason-fullscreen = ứng dụng toàn màn hình
power-reason-thermal-throttling = CPU đang giảm tốc

# Phase 32 — cloud backend matrix via OpenDAL. Settings → Remotes
# tab + the Add-backend wizard that writes one entry per remote into
# the keychain-backed credential store.
remote-heading = Backend từ xa
remote-add = Thêm backend
remote-list-empty = Chưa cấu hình backend từ xa nào
remote-test = Kiểm tra kết nối
remote-test-success = Kết nối thành công
remote-test-failed = Kết nối thất bại
remote-remove = Gỡ backend
remote-name-label = Tên hiển thị
remote-kind-label = Loại backend
remote-save = Lưu backend
remote-cancel = Hủy
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
backend-local-fs = Hệ thống tệp cục bộ
cloud-config-bucket = Bucket
cloud-config-region = Vùng
cloud-config-endpoint = URL điểm cuối
cloud-config-root = Đường dẫn gốc
cloud-error-invalid-config = Cấu hình backend không hợp lệ
cloud-error-network = Lỗi mạng khi liên hệ backend
cloud-error-not-found = Không tìm thấy đối tượng tại đường dẫn yêu cầu
cloud-error-permission = Backend từ xa từ chối quyền truy cập
cloud-error-keychain = Truy cập keychain của hệ điều hành thất bại
settings-tab-remotes = Backend từ xa
settings-tab-mobile = Di động

# Phase 33 — mount Copy That's chunk store + history archive as a
# read-only filesystem (FUSE / WinFsp). Surfaced by the History tab
# context menu's "Mount snapshot" action + the Settings → Advanced
# "Mount latest on launch" toggle.
mount-heading = Gắn ảnh chụp
mount-action-mount = Gắn ảnh chụp
mount-action-unmount = Tháo gắn
mount-status-mounted = Đã gắn tại { $path }
mount-error-unsafe-mountpoint = Đường dẫn điểm gắn không an toàn
mount-error-mountpoint-not-empty = Điểm gắn phải là một thư mục trống
mount-error-backend-unavailable = Backend gắn không khả dụng trên hệ thống này
mount-error-archive-read = Đọc kho lưu trữ thất bại
mount-picker-title = Chọn thư mục điểm gắn
mount-toast-mounted = Đã gắn ảnh chụp tại { $path }
mount-toast-unmounted = Đã tháo gắn ảnh chụp
mount-toast-failed = Gắn thất bại: { $reason }
settings-mount-heading = Gắn ảnh chụp
settings-mount-hint = Hiển thị kho lưu trữ lịch sử dưới dạng hệ thống tệp chỉ đọc. Phase 33b kết nối luồng chạy; các backend nhân FUSE/WinFsp sẽ có ở Phase 33c.
settings-mount-on-launch = Gắn ảnh chụp mới nhất khi khởi chạy
settings-mount-on-launch-path = Đường dẫn điểm gắn
settings-mount-on-launch-path-placeholder = ví dụ C:\Mounts\copythat

# Phase 34 — enterprise-grade audit log export + WORM (write-once-read-
# many) tamper-resistance. Settings → Advanced → Audit log collects the
# format + destination + rotation + WORM toggle; runtime records job and
# file events into the tamper-evident chain-hashed sink.
settings-audit-heading = Nhật ký kiểm toán
settings-audit-hint = Nhật ký chỉ ghi thêm, chống giả mạo, cho mọi sự kiện tác vụ và tệp. Các định dạng gồm CSV, JSON-lines, Syslog RFC 5424, ArcSight CEF và QRadar LEEF.
settings-audit-enable = Bật ghi nhật ký kiểm toán
settings-audit-format = Định dạng nhật ký
settings-audit-format-json-lines = JSON lines (mặc định khuyến nghị)
settings-audit-format-csv = CSV (thân thiện với bảng tính)
settings-audit-format-syslog = Syslog (RFC 5424)
settings-audit-format-cef = CEF (ArcSight)
settings-audit-format-leef = LEEF 2.0 (IBM QRadar)
settings-audit-file-path = Đường dẫn tệp nhật ký
settings-audit-file-path-placeholder = ví dụ C:\ProgramData\CopyThat\audit.log
settings-audit-max-size = Xoay vòng sau (byte, 0 = không bao giờ)
settings-audit-worm = Bật chế độ WORM (ghi một lần, đọc nhiều lần)
settings-audit-worm-hint = Áp dụng cờ chỉ-ghi-thêm của nền tảng (Linux chattr +a, macOS chflags uappnd, thuộc tính chỉ đọc của Windows) sau mỗi lần tạo hoặc xoay vòng. Ngay cả quản trị viên cũng phải xóa cờ một cách rõ ràng mới cắt ngắn được nhật ký.
settings-audit-test-write = Kiểm tra ghi
settings-audit-verify-chain = Xác minh chuỗi
toast-audit-test-write-ok = Kiểm tra ghi nhật ký kiểm toán thành công
toast-audit-verify-ok = Đã xác minh chuỗi kiểm toán nguyên vẹn
toast-audit-verify-failed = Xác minh chuỗi kiểm toán báo có sai lệch

# Phase 35 — destination encryption via age (X25519 / passphrase /
# SSH) + on-the-fly zstd compression with a per-extension deny
# heuristic. Surfaced through Settings → Transfer → Encryption +
# Compression; engine short-circuits to the crypt pipeline when
# either stage is active.
settings-crypt-heading = Mã hóa & nén
settings-crypt-hint = Biến đổi nội dung tệp trước khi tới đích. Mã hóa dùng định dạng age; nén dùng zstd và có thể bỏ qua các tệp media đã nén theo phần mở rộng.
settings-crypt-encryption-mode = Mã hóa
settings-crypt-encryption-off = Tắt
settings-crypt-encryption-passphrase = Cụm mật khẩu (hỏi khi bắt đầu sao chép)
settings-crypt-encryption-recipients = Khóa người nhận từ tệp
settings-crypt-encryption-hint = Cụm mật khẩu chỉ được giữ trong bộ nhớ trong suốt quá trình sao chép. Tệp người nhận liệt kê mỗi dòng một khóa công khai age1… hoặc ssh-.
settings-crypt-recipients-file = Đường dẫn tệp người nhận
settings-crypt-recipients-file-placeholder = ví dụ C:\Users\me\recipients.txt
settings-crypt-compression-mode = Nén
settings-crypt-compression-off = Tắt
settings-crypt-compression-always = Luôn nén
settings-crypt-compression-smart = Thông minh (bỏ qua media đã nén)
settings-crypt-compression-hint = Chế độ thông minh bỏ qua jpg, mp4, zip, 7z và các định dạng tương tự không hưởng lợi từ zstd. Chế độ luôn nén sẽ nén mọi tệp ở mức đã chọn.
settings-crypt-compression-level = Mức zstd (1-22)
settings-crypt-compression-level-hint = Số thấp hơn nhanh hơn; số cao hơn nén mạnh hơn. Mức 3 tương ứng với mặc định CLI của zstd.
compress-footer-savings = 💾 { $original } → { $compressed } (tiết kiệm { $percent }%)
compress-savings-toast = Đã nén { $percent }% (tiết kiệm { $bytes })
crypt-toast-recipients-loaded = Đã tải { $count } người nhận mã hóa
crypt-toast-recipients-error = Tải người nhận thất bại: { $reason }
crypt-toast-passphrase-required = Mã hóa cần cụm mật khẩu trước khi bắt đầu sao chép
crypt-toast-passphrase-set = Đã ghi nhận cụm mật khẩu mã hóa
crypt-footer-encrypted-badge = 🔒 Đã mã hóa (age)
crypt-footer-compressed-badge = 📦 Đã nén (zstd)

# Phase 36 — copythat CLI surface. Documented exit codes + the
# user-facing strings for plan/apply/verify/config. Help text stays
# in English (engineering accessibility) but error / info / status
# strings are localized so a localized desktop installation surfaces
# CLI errors in the same language as the GUI.
cli-help-tagline = Copy That CLI — sao chép, đồng bộ, xác minh và kiểm toán tệp chính xác từng byte cho pipeline CI/CD.
cli-help-exit-codes = Mã thoát: 0 thành công, 1 lỗi, 2 đang chờ, 3 xung đột, 4 xác minh thất bại, 5 mạng, 6 quyền, 7 đĩa đầy, 8 hủy, 9 cấu hình.
cli-error-bad-args = copy/move cần ít nhất một nguồn và một đích
cli-error-unknown-algo = Thuật toán xác minh không xác định: { $algo }
cli-error-missing-spec = --spec là bắt buộc cho plan/apply
cli-error-spec-parse = Phân tích jobspec { $path } thất bại: { $reason }
cli-error-spec-empty-sources = Danh sách nguồn của jobspec trống
cli-info-shape-recorded = Đã ghi nhận hình băng thông "{ $rate }"; việc thực thi được nối qua copythat-shape
cli-info-stub-deferred = { $command } đã được chuẩn bị cho việc nối tiếp ở Phase 36
cli-plan-summary = Kế hoạch: { $actions } hành động, { $bytes } byte; { $already_done } đã sẵn sàng
cli-plan-pending = Kế hoạch báo có hành động đang chờ; chạy lại với `apply` để thực thi
cli-plan-already-done = Kế hoạch báo không có gì để làm (bất biến)
cli-apply-success = Apply hoàn tất không lỗi
cli-apply-failed = Apply hoàn tất với một hoặc nhiều lỗi
cli-verify-ok = Xác minh ổn: { $algo } { $digest }
cli-verify-failed = Xác minh THẤT BẠI cho { $path } ({ $algo })
cli-config-set = Đặt { $key } = { $value }
cli-config-reset = Đặt lại { $key } về mặc định
cli-config-unknown-key = Khóa cấu hình không xác định: { $key }
cli-completions-emitted = Bản hoàn thiện shell cho { $shell } đã in ra stdout

# Phase 37 — desktop-side mobile companion. Settings → Mobile panel
# strings + the SAS-confirmation modal + push-notification toasts.
# The actual mobile UI lives in the Phase 37 follow-up Tauri Mobile
# target; the phone displays its own localized SAS prompt.
settings-mobile-heading = Ứng dụng đồng hành di động
settings-mobile-hint = Ghép nối iPhone hoặc điện thoại Android để duyệt lịch sử, khởi chạy các hồ sơ và jobspec Phase 36 đã lưu, và nhận thông báo hoàn tất.
settings-mobile-pair-toggle = Cho phép ghép nối mới
settings-mobile-pair-active = Máy chủ ghép nối đang chạy — quét mã QR bằng ứng dụng di động Copy That
settings-mobile-pair-button = Bắt đầu ghép nối
settings-mobile-revoke-button = Thu hồi
settings-mobile-no-pairings = Chưa có thiết bị nào được ghép nối
settings-mobile-pair-port = Cổng liên kết (0 = chọn cổng trống)
pair-sas-prompt = Cả hai màn hình phải hiển thị cùng bốn emoji. Chạm Khớp nếu chúng trùng nhau.
pair-sas-confirm = Khớp
pair-sas-reject = Không khớp — hủy
pair-toast-success = Đã ghép nối với { $device }
pair-toast-failed = Ghép nối thất bại: { $reason }
push-toast-sent = Đã gửi thông báo đẩy tới { $device }
push-toast-failed = Gửi thông báo đẩy tới { $device } thất bại: { $reason }

# Phase 38 — aggregate destination dedup + reflink fallback ladder.
# Settings → Transfer → Dedup panel + per-job-row badges (⚡
# Reflinked / 🔗 Hardlinked / 🧩 Chunk-shared / 📋 Copied) +
# pre-pass dedup-scan modal.
settings-dedup-heading = Khử trùng lặp tại đích
settings-dedup-hint = Khi nguồn và đích dùng chung một ổ đĩa, Copy That có thể nhân bản tệp ở cấp hệ thống tệp thay vì sao chép từng byte. Reflink tức thì + an toàn; hardlink nhanh hơn nhưng cả hai tên dùng chung trạng thái.
settings-dedup-mode-auto = Thang tự động (reflink → hardlink → khối → sao chép)
settings-dedup-mode-reflink-only = Chỉ reflink
settings-dedup-mode-hardlink-aggressive = Quyết liệt (reflink + hardlink cả trên tệp ghi được)
settings-dedup-mode-off = Tắt (luôn sao chép từng byte)
settings-dedup-hardlink-policy = Chính sách hardlink
settings-dedup-prescan = Quét trước cây đích để tìm nội dung trùng lặp
dedup-badge-reflinked = ⚡ Đã reflink
dedup-badge-hardlinked = 🔗 Đã hardlink
dedup-badge-chunk-shared = 🧩 Dùng chung khối
dedup-badge-copied = 📋 Đã sao chép
phase42-paranoid-verify-label = Xác minh kỹ lưỡng
phase42-paranoid-verify-hint = Loại bỏ các trang đã lưu trong bộ đệm của đích và đọc lại từ đĩa để bắt lỗi nói dối của bộ đệm ghi và hỏng dữ liệu ngầm. Chậm hơn khoảng 50% so với xác minh mặc định; mặc định tắt.
phase42-sharing-violation-retries-label = Số lần thử lại với tệp nguồn bị khóa
phase42-sharing-violation-retries-hint = Số lần thử lại khi một tiến trình khác đang giữ tệp nguồn mở với khóa độc quyền. Độ trễ tăng gấp đôi mỗi lần (mặc định 50 ms / 100 ms / 200 ms). Mặc định 3, khớp với Robocopy /R:3.
phase42-cloud-placeholder-warning = { $name } là tệp OneDrive chỉ trên đám mây. Sao chép nó sẽ kích hoạt tải về — lên đến { $size } qua kết nối mạng của bạn.
phase42-defender-exclusion-hint = Để có thông lượng sao chép tối đa, hãy thêm thư mục đích vào danh sách loại trừ của Microsoft Defender trước khi chuyển hàng loạt. Xem docs/PERFORMANCE_TUNING.md.

# Phase 39 — Browser-accessible recovery UI. Settings → Advanced
# exposes these strings; the recovery server itself renders askama
# templates that always read English — these eight keys are the
# Settings prose the user actually reads in their preferred locale.
settings-recovery-heading = Giao diện web khôi phục
settings-recovery-enable = Bật giao diện web khôi phục
settings-recovery-bind-address = Địa chỉ liên kết
settings-recovery-port = Cổng (0 = chọn cổng trống)
settings-recovery-show-url = Hiện URL & token
settings-recovery-rotate-token = Xoay vòng token
settings-recovery-allow-non-loopback = Cho phép liên kết ngoài loopback
settings-recovery-non-loopback-warning = CẢNH BÁO: bật liên kết ngoài loopback sẽ phơi bày giao diện khôi phục ra mạng nội bộ của bạn. Bất kỳ ai biết token đều có thể duyệt lịch sử tệp và tải tệp về. Hãy che chắn bằng TLS hoặc reverse proxy nếu mạng LAN không đáng tin.

# Phase 40 — SMB compression negotiation + cloud-VM offload helper.
# 6 SMB keys (header badge + Settings prose) + 6 cloud-offload keys
# (Remotes tab wizard for cross-cloud copy templates).
smb-compress-badge = 🗜 Nén SMB: { $algo }
smb-compress-badge-tooltip = Lưu lượng mạng tới đích này đang được nén khi truyền (SMB 3.1.1).
smb-compress-toast-saved = Đã tiết kiệm { $bytes } qua mạng
smb-compress-algo-unknown = thuật toán không xác định
settings-smb-compress-heading = Nén mạng SMB
settings-smb-compress-hint = Tự động thương lượng nén lưu lượng SMB 3.1.1 trên các đích UNC. Lợi ích miễn phí trên đường truyền chậm; bỏ qua với đích cục bộ.
cloud-offload-heading = Trình trợ giúp giảm tải VM đám mây
cloud-offload-hint = Khi sao chép trực tiếp giữa hai đám mây, hãy tạo một mẫu triển khai chạy việc sao chép từ một VM tạm thời nhỏ trên đám mây — dữ liệu không bao giờ đi qua mạng của máy bạn.
cloud-offload-render-button = Tạo mẫu
cloud-offload-copy-clipboard = Sao chép vào bộ nhớ tạm
cloud-offload-template-format = Định dạng mẫu
cloud-offload-self-destruct-warning = VM tự tắt sau { $minutes } phút — xác nhận vai trò IAM + vùng trước khi triển khai.

# Phase 41 — animated before/after tree-diff preview. The `Preview
# changes` modal renders the rolled-up plan before the engine starts
# work; 14 keys cover the title, the summary header / counts, the
# row-reason labels, and the two action buttons.
preview-modal-title = Xem trước thay đổi
preview-summary-header = Điều sẽ xảy ra
preview-category-additions = { $count } mục thêm
preview-category-replacements = { $count } mục thay thế
preview-category-skips = { $count } mục bỏ qua
preview-category-conflicts = { $count } xung đột
preview-category-unchanged = { $count } không đổi
preview-bytes-to-transfer = { $bytes } cần chuyển
preview-reason-source-newer = Nguồn mới hơn
preview-reason-dest-newer = Đích mới hơn — sẽ bỏ qua
preview-reason-content-different = Nội dung khác nhau
preview-reason-identical = Giống hệt nguồn
preview-button-run = Chạy kế hoạch
preview-button-reduce = Giảm bớt kế hoạch…

# Phase 42 — perceptual-hash visual-similarity dedup. Eight keys cover
# the pre-copy "looks visually identical" warning + the Settings panel.
perceptual-warn-title = Trông giống hệt về thị giác
perceptual-warn-body = { $name } tại đích trông trùng khớp với ảnh nguồn. Vẫn tiếp tục sao chép?
perceptual-warn-keep-both = Giữ cả hai
perceptual-warn-skip = Bỏ qua tệp này
perceptual-warn-overwrite = Vẫn ghi đè
perceptual-settings-heading = Khử trùng lặp theo độ giống thị giác
perceptual-settings-hint = Phát hiện các ảnh giống hệt về thị giác tại đích trước khi chúng bị ghi đè. Băm theo cảm nhận (nhận ra cùng một ảnh được lưu lại ở định dạng khác), không phải chính xác từng byte.
perceptual-settings-threshold-label = Ngưỡng cảnh báo (thấp hơn = khớp chặt hơn)

# Phase 42 Part B — per-file rolling versions (Time Machine for any
# destination). 8 keys cover the version-list panel + retention picker.
version-list-heading = Phiên bản trước
version-list-empty = Không có phiên bản trước của tệp này
version-list-restore = Khôi phục phiên bản này
version-retention-heading = Giữ các phiên bản trước khi ghi đè
version-retention-none = Giữ mọi phiên bản mãi mãi
version-retention-last-n = Giữ { $n } phiên bản gần nhất
version-retention-older-than-days = Bỏ các phiên bản cũ hơn { $days } ngày
version-retention-gfs = Hằng giờ { $h } · hằng ngày { $d } · hằng tuần { $w } · hằng tháng { $m }

# Phase 43 — forensic chain-of-custody manifests + BLAKE3 verified
# streaming. 14 keys cover the Settings → Provenance panel
# (heading, hint, toggles, TSA URL, signing-key management) plus
# the post-job manifest notification + the verify command's two
# headline result lines.
provenance-settings-heading = Chuỗi giám sát pháp y
provenance-settings-hint = Ký mỗi tác vụ sao chép bằng manifest BLAKE3 + ed25519. Người xét duyệt có thể băm lại cây đích sau này và chứng minh không byte nào thay đổi kể từ khi sao chép.
provenance-settings-enable-default = Ký mọi tác vụ mới theo mặc định
provenance-settings-show-after-job = Hiển thị manifest sau mỗi tác vụ hoàn tất
provenance-settings-tsa-url-label = URL cơ quan đóng dấu thời gian RFC 3161 mặc định
provenance-settings-tsa-url-hint = Tùy chọn. Khi đặt, manifest mang một dấu thời gian TSA miễn phí chứng minh dữ liệu đã tồn tại tại thời điểm này. Để trống để bỏ qua.
provenance-settings-keys-heading = Khóa ký
provenance-settings-keys-generate = Tạo khóa mới
provenance-settings-keys-import = Nhập khóa…
provenance-settings-keys-export = Xuất khóa công khai…
provenance-job-completed-title = Đã lưu manifest nguồn gốc
provenance-job-completed-body = { $count } tệp đã ký → { $path }
provenance-verify-clean = Manifest hợp lệ cho { $count } tệp; chữ ký { $sig }; gốc merkle OK.
provenance-verify-tampered = Manifest KHÔNG HỢP LỆ — { $tampered } bị giả mạo, { $missing } bị thiếu.
# Phase 43 post-review hardening — toast text for the Settings →
# Provenance buttons whose Tauri IPC has not yet landed.
provenance-action-staged = Phase 43 — việc nối IPC cho hành động này sẽ có ở một commit tiếp theo.

# Phase 44 — SSD-aware whole-drive sanitize (NVMe Sanitize / OPAL
# Crypto Erase) + the per-file shred refusal on copy-on-write
# filesystems. 16 keys cover the new "Drive sanitize" Settings
# subsection and the localized error messages.
sanitize-heading = Xóa sạch an toàn toàn ổ đĩa
sanitize-hint = NVMe Sanitize, OPAL Crypto Erase và ATA Secure Erase xóa sạch ổ flash ở lớp firmware trong vài mili giây. Ghi đè từng tệp là vô nghĩa trên flash — hủy nhiều lượt chỉ làm hao mòn NAND. Hãy dùng cách này để xóa sạch thực sự.
sanitize-pick-device = Chọn ổ đĩa cần xóa sạch
sanitize-mode-label = Phương pháp xóa sạch
sanitize-mode-nvme-format = NVMe Format (kèm secure erase)
sanitize-mode-nvme-sanitize-block = NVMe Sanitize — Block Erase (chậm, mọi ô nhớ)
sanitize-mode-nvme-sanitize-crypto = NVMe Sanitize — Crypto Erase (tức thì)
sanitize-mode-ata-secure-erase = ATA Secure Erase (SSD SATA cũ)
sanitize-mode-opal-crypto-erase = TCG OPAL Crypto Erase (ổ tự mã hóa)
sanitize-mode-apfs-crypto-erase = APFS Crypto Erase (xoay khóa FileVault, chỉ macOS)
sanitize-confirm-1 = Thao tác này hủy MỌI byte trên { $device }. Không thể hoàn tác.
sanitize-confirm-2 = Tôi hiểu rằng mọi phân vùng, mọi tệp và mọi ảnh chụp trên { $device } sẽ vĩnh viễn không thể đọc được.
sanitize-confirm-3 = Nhập tên model của ổ đĩa để tiếp tục: { $model }
sanitize-running = Đang xóa sạch { $device } ({ $mode }) — việc này có thể mất từ vài mili giây (crypto erase) đến hàng chục phút (block erase). Đừng tắt nguồn.
sanitize-completed = Xóa sạch hoàn tất — { $device } giờ đã trống.
ssd-honest-shred-meaningless = Hủy từng tệp trên hệ thống tệp sao-chép-khi-ghi (Btrfs / ZFS / APFS) không thể chạm tới các khối bên dưới. Hãy dùng xóa sạch toàn ổ đĩa cộng với xoay khóa mã hóa toàn ổ thay thế.
ssd-honest-advisory = Tệp này nằm trên flash. Ghi đè từng tệp gây hao mòn NAND và KHÔNG đảm bảo các ô nhớ gốc là không thể khôi phục. Với dữ liệu nhạy cảm, hãy xóa sạch toàn bộ ổ đĩa.

# Phase 44.1f post-review — placeholder toast for SanitizeTab
# buttons whose Tauri IPC has not yet landed.
sanitize-action-staged = Phase 44.1 — việc nối IPC cho hành động này sẽ có ở một commit tiếp theo.

# Phase 45.3 — named-queue tab strip (Subfeature A). Tabs surface
# once the QueueRegistry holds at least one queue; the synthesised
# default tab keeps legacy single-queue jobs reachable.
queue-tab-default = Mặc định
queue-tab-empty-state = Hàng đợi tác vụ
queue-badge-tooltip = Các tác vụ đang chờ và đang chạy trong hàng đợi này

# Phase 45.4 — drag-progress-merge (Subfeature B). Drag a queue tab
# onto another to merge their job lists. The default tab is neither
# draggable nor a drop target; only registry queues participate.
queue-drag-hint = Kéo lên một hàng đợi khác để hợp nhất
queue-merge-confirm = Thả để hợp nhất
queue-merge-toast = Đã hợp nhất hàng đợi

# Phase 45.5 — F2-queue UX (Subfeature C). F2 toggles
# `auto_enqueue_next` so every fresh enqueue piles into the running
# queue rather than spawning a parallel one. Status pill renders in
# the Footer; pulsing dot renders on the running tab.
queue-f2-active-hint = Chế độ F2: mọi tác vụ mới đưa vào đều rơi vào hàng đợi này
queue-f2-toggled-on = Chế độ hàng đợi F2 BẬT — các tác vụ mới gia nhập hàng đợi đang chạy
queue-f2-toggled-off = Chế độ hàng đợi F2 TẮT — các tác vụ mới sinh ra hàng đợi song song
queue-f2-status-bar = Chế độ hàng đợi F2: BẬT

# Phase 45.6 — tray destination targets (Subfeature D). Pinned
# destinations appear in the OS tray menu; clicking one arms it as
# the active drop target so the next file drop bypasses the
# DropStagingDialog. Settings → General hosts the list editor.
tray-target-section-title = Đích trên khay
tray-target-section-hint = Các đích đã ghim xuất hiện trong menu khay. Nhấp vào một mục để đặt nó làm đích thả tiếp theo.
tray-target-empty = Chưa ghim đích nào trên khay.
tray-target-remove = Gỡ
tray-target-add-label = Nhãn
tray-target-add-path = Đường dẫn hoặc URI backend
tray-target-add = Thêm
tray-target-armed-toast = Thả tệp tiếp theo để gửi tới { $label }
tray-target-active-pill = → { $label }

# Phase 45.7 follow-up — pinned-destination validation errors. The
# `queue_pin_destination` IPC returns these Fluent keys verbatim
# when input fails the IPC-boundary checks; the toast layer renders
# them via `t(...)` (Toast.svelte detects kebab-case-lowercase and
# routes through the locale table). Phase 17e's `err-path-escape` /
# `err-destination-empty` cover the path-traversal + empty cases for
# `queue_route_job`; the keys below are pin-specific.
err-pinned-destination-label-empty = Nhãn đích trên khay không được để trống.
err-pinned-destination-path-empty = Đường dẫn đích trên khay không được để trống.
err-pinned-destination-label-too-long = Nhãn đích trên khay quá dài (tối đa 64 ký tự).
err-pinned-destination-path-too-long = Đường dẫn đích trên khay quá dài (tối đa 1024 ký tự).
err-pinned-destination-label-invalid = Nhãn đích trên khay chứa ký tự không được phép (xuống dòng, về đầu dòng, hoặc NUL).
err-pinned-destination-path-invalid = Đường dẫn đích trên khay chứa ký tự không được phép (xuống dòng, về đầu dòng, hoặc NUL).
err-pinned-destination-too-many = Bạn đã đạt giới hạn 50 đích trên khay. Hãy gỡ một mục để thêm mục khác.

# Phase 46.6 — Settings → Plugins tab. The IPC layer in
# `apps/copythat-ui/src-tauri/src/plugin_commands.rs` enumerates the
# per-user plugin store under `<config_dir>/plugins/`; the
# `PluginsTab.svelte` component renders these strings against the
# returned manifest + grant state.
settings-tab-plugins = Plugin
plugin-heading = Plugin
plugin-hint = Các plugin WASM trong hộp cát mở rộng Copy That bằng các hook tùy chỉnh. Mỗi plugin chạy dưới giới hạn CPU và bộ nhớ cho từng lần gọi và chỉ thấy các năng lực máy chủ mà bạn cấp cho nó.
plugin-list-empty = Chưa cài plugin nào.
plugin-enabled = Đã bật
plugin-disabled = Đã tắt
plugin-hooks = Hook
plugin-capabilities = Năng lực
plugin-no-capabilities = (không có)
plugin-directory = Vị trí
plugin-install-from-file = Cài từ tệp…
plugin-install-from-url = Cài từ URL…
plugin-url-wasm = URL WASM
plugin-url-manifest = URL manifest
plugin-url-hash = Băm BLAKE3
plugin-url-preview = Xem trước
plugin-url-confirm = Xác nhận cài đặt

# Phase 31b — power-policy settings (Power tab).
settings-tab-power = Nguồn điện
settings-power-hint = Giới hạn hoặc tạm dừng sao chép theo trạng thái nguồn: pin, mạng tính theo dung lượng/di động, khi trình chiếu/toàn màn hình, hoặc khi CPU bị giảm tốc do nhiệt.
settings-power-enabled = Bật điều tiết theo nguồn điện
settings-power-battery = Khi dùng pin
settings-power-metered = Trên mạng tính theo dung lượng
settings-power-cellular = Trên mạng di động
settings-power-presentation = Khi đang trình chiếu
settings-power-fullscreen = Khi toàn màn hình
settings-power-thermal = Khi giảm tốc do nhiệt
settings-power-continue = Tiếp tục
settings-power-pause = Tạm dừng
err-server-not-implemented = Chế độ máy chủ chưa khả dụng.
err-webhook-not-implemented = Việc gửi webhook chưa khả dụng.

# Phase 47 — "why is this slow?" diagnostics (bottleneck badge + tooltip).
bottleneck-source-io = Nguồn I/O
bottleneck-dest-io = Đích I/O
bottleneck-network = Mạng
bottleneck-antivirus = Diệt vi-rút
bottleneck-cpu = CPU
bottleneck-thermal = Nhiệt
bottleneck-unknown = Không xác định
diag-aria = Điểm nghẽn: { $cause }
diag-tooltip = Bị giới hạn bởi { $cause } · { $rate }
diag-spark-aria = Thông lượng trong phút vừa qua
diag-keeping-up = Đang theo kịp
diag-label = Chẩn đoán

# Phase 48 — server mode + observability (Settings → Server).
settings-tab-server = Máy chủ
server-hint = Chạy Copy That như một máy chủ tệp không giao diện. Chọn các giao thức cần công khai, đặt địa chỉ và thư mục cần phục vụ, và tùy chọn yêu cầu xác thực.
server-protocols = Giao thức
server-bind-addr = Địa chỉ liên kết
server-root = Thư mục phục vụ
server-readonly = Chỉ đọc (từ chối tải lên và xóa)
server-auth-mode = Xác thực
server-auth-none = Không
server-auth-bearer = Token Bearer
server-auth-basic = Cơ bản (tên người dùng + mật khẩu)
server-auth-token = Token
server-auth-user = Tên người dùng
server-auth-password = Mật khẩu
otel-endpoint = Điểm cuối OpenTelemetry
webhook-section = Webhook
webhook-url = URL webhook
webhook-add = Thêm webhook
webhook-remove = Xóa
webhook-empty = Chưa cấu hình webhook nào.
webhook-pushover-token = Token Pushover
webhook-pushover-user = Người dùng Pushover
server-start = Khởi động máy chủ
server-stop = Dừng máy chủ
server-status-running = Đang chạy trên { $addr }
server-status-stopped = Đã dừng
server-metrics-url = Số liệu
err-server-no-protocols = Chọn ít nhất một giao thức trước khi khởi động máy chủ.
err-server-bind = Không thể liên kết địa chỉ máy chủ. Địa chỉ này có thể đang được sử dụng.
