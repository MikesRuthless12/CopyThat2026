app-name = Copy That 2026
# MT
window-title = Copy That 2026
# MT
shred-ssd-advisory = 警告: この対象は SSD 上にあります。ウェアレベリングとオーバープロビジョニングにより論理ブロックアドレスの下からデータが移動されるため、多重上書きはフラッシュメモリを確実にサニタイズできません。ソリッドステートメディアには、ATA SECURE ERASE、NVMe Format（セキュア消去付き）、または鍵を破棄したフルディスク暗号化を優先してください。

# MT
state-idle = 待機中
# MT
state-copying = コピー中
# MT
state-verifying = 検証中
# MT
state-paused = 一時停止
# MT
state-error = エラー

# MT
state-pending = キュー
# MT
state-running = 実行中
# MT
state-cancelled = キャンセル済み
# MT
state-succeeded = 完了
# MT
state-failed = 失敗

# MT
action-pause = 一時停止
# MT
action-resume = 再開
# MT
action-cancel = キャンセル
# MT
action-pause-all = すべてのジョブを一時停止
# MT
action-resume-all = すべてのジョブを再開
# MT
action-cancel-all = すべてのジョブをキャンセル
# MT
action-close = 閉じる
# MT
action-reveal = フォルダーで表示

# MT
menu-pause = 一時停止
# MT
menu-resume = 再開
# MT
menu-cancel = キャンセル
# MT
menu-remove = キューから削除
# MT
menu-reveal-source = ソースをフォルダーで表示
# MT
menu-reveal-destination = 宛先をフォルダーで表示

# MT
header-eta-label = 推定残り時間
# MT
header-toolbar-label = グローバルコントロール

# MT
footer-queued = 件のアクティブジョブ
# MT
footer-total-bytes = 実行中
# MT
footer-errors = 件のエラー
# MT
footer-history = 履歴

# MT
empty-title = コピーするファイルまたはフォルダーをドロップ
# MT
empty-hint = ウィンドウに項目をドラッグしてください。宛先を尋ねた後、各ソースごとに 1 件のジョブをキューに追加します。
# MT
empty-region-label = ジョブリスト

# MT
details-drawer-label = ジョブの詳細
# MT
details-source = ソース
# MT
details-destination = 宛先
# MT
details-state = 状態
# MT
details-bytes = バイト
# MT
details-files = ファイル
# MT
details-speed = 速度
# MT
details-eta = 残り時間
# MT
details-error = エラー

# MT
drop-dialog-title = ドロップした項目を転送
# MT
drop-dialog-subtitle = { $count } 件の項目が転送可能です。開始するには宛先フォルダーを選択してください。
# MT
drop-dialog-mode = 操作
# MT
drop-dialog-copy = コピー
# MT
drop-dialog-move = 移動
# MT
drop-dialog-pick-destination = 宛先を選択
# MT
drop-dialog-change-destination = 宛先を変更
# MT
drop-dialog-start-copy = コピーを開始
# MT
drop-dialog-start-move = 移動を開始

# MT
eta-calculating = 計算中…
# MT
eta-unknown = 不明

# MT
toast-job-done = 転送が完了しました
# MT
toast-copy-queued = コピーをキューに追加しました
# MT
toast-move-queued = 移動をキューに追加しました
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
