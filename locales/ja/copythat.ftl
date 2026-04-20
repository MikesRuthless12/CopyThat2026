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
# MT — Phase 8 toast messages
toast-error-resolved = エラーを解決しました
# MT
toast-collision-resolved = 競合を解決しました
# MT
toast-elevated-unavailable = 昇格権限での再試行は Phase 17 で提供されます — まだ利用できません
# MT
toast-error-log-exported = エラーログをエクスポートしました

# MT — Error modal
error-modal-title = 転送が失敗しました
# MT
error-modal-retry = 再試行
# MT
error-modal-retry-elevated = 昇格権限で再試行
# MT
error-modal-skip = スキップ
# MT
error-modal-skip-all-kind = この種類のエラーをすべてスキップ
# MT
error-modal-abort = すべて中止
# MT
error-modal-path-label = パス
# MT
error-modal-code-label = コード

# MT — Error-kind labels
err-not-found = ファイルが見つかりません
# MT
err-permission-denied = アクセスが拒否されました
# MT
err-disk-full = 転送先ディスクに空き容量がありません
# MT
err-interrupted = 操作が中断されました
# MT
err-verify-failed = コピー後の検証に失敗しました
# MT
err-io-other = 不明な I/O エラー

# MT — Collision modal
collision-modal-title = ファイルが既に存在します
# MT
collision-modal-overwrite = 上書き
# MT
collision-modal-overwrite-if-newer = 新しい場合は上書き
# MT
collision-modal-skip = スキップ
# MT
collision-modal-keep-both = 両方を保持
# MT
collision-modal-rename = 名前を変更…
# MT
collision-modal-apply-to-all = すべてに適用
# MT
collision-modal-source = ソース
# MT
collision-modal-destination = 宛先
# MT
collision-modal-size = サイズ
# MT
collision-modal-modified = 更新日時
# MT
collision-modal-hash-check = 簡易ハッシュ (SHA-256)
# MT
collision-modal-rename-placeholder = 新しいファイル名
# MT
collision-modal-confirm-rename = 名前を変更

# MT — Error log drawer
error-log-title = エラーログ
# MT
error-log-empty = エラーは記録されていません
# MT
error-log-export-csv = CSV をエクスポート
# MT
error-log-export-txt = テキストをエクスポート
# MT
error-log-clear = ログをクリア
# MT
error-log-col-time = 時刻
# MT
error-log-col-job = ジョブ
# MT
error-log-col-path = パス
# MT
error-log-col-code = コード
# MT
error-log-col-message = メッセージ
# MT
error-log-col-resolution = 解決方法

# MT — History drawer (Phase 9)
history-title = 履歴
# MT
history-empty = 記録されたジョブはありません
# MT
history-unavailable = コピー履歴は利用できません。アプリ起動時に SQLite ストアを開けませんでした。
# MT
history-filter-any = すべて
# MT
history-filter-kind = 種類
# MT
history-filter-status = 状態
# MT
history-filter-text = 検索
# MT
history-refresh = 更新
# MT
history-export-csv = CSV をエクスポート
# MT
history-purge-30 = 30 日より古いものを削除
# MT
history-rerun = 再実行
# MT
history-detail-open = 詳細
# MT
history-detail-title = ジョブの詳細
# MT
history-detail-empty = 記録された項目はありません
# MT
history-col-date = 日付
# MT
history-col-kind = 種類
# MT
history-col-src = ソース
# MT
history-col-dst = 宛先
# MT
history-col-files = ファイル
# MT
history-col-size = サイズ
# MT
history-col-status = 状態
# MT
history-col-duration = 所要時間
# MT
history-col-error = エラー

# MT
toast-history-exported = 履歴をエクスポートしました
# MT
toast-history-rerun-queued = 再実行をキューに追加しました

# MT — Totals drawer (Phase 10)
footer-totals = 合計
# MT
totals-title = 合計
# MT
totals-loading = 合計を読み込み中…
# MT
totals-card-bytes = 総コピーバイト数
# MT
totals-card-files = ファイル
# MT
totals-card-jobs = ジョブ
# MT
totals-card-avg-rate = 平均スループット
# MT
totals-errors = エラー
# MT
totals-spark-title = 直近 30 日
# MT
totals-kinds-title = 種類別
# MT
totals-saved-title = 節約時間 (推定)
# MT
totals-saved-note = 標準のファイルマネージャーで同じ作業をコピーした場合との推定比較です。
# MT
totals-reset = 統計をリセット
# MT
totals-reset-confirm = 保存されているすべてのジョブと項目が削除されます。続行しますか?
# MT
totals-reset-confirm-yes = はい、リセット
# MT
toast-totals-reset = 統計をリセットしました

# MT — Phase 11a additions
header-language-label = 言語
# MT
header-language-title = 言語を変更

# MT
kind-copy = コピー
# MT
kind-move = 移動
# MT
kind-delete = 削除
# MT
kind-secure-delete = セキュア削除

# MT
status-running = 実行中
# MT
status-succeeded = 成功
# MT
status-failed = 失敗
# MT
status-cancelled = キャンセル
# MT
status-ok = OK
# MT
status-skipped = スキップ

# MT
history-search-placeholder = /パス
# MT
toast-history-purged = 30 日より古い { $count } 件のジョブを削除しました

# MT
err-source-required = 少なくとも 1 つのソースパスが必要です。
# MT
err-destination-empty = 宛先のパスが空です。
# MT
err-source-empty = ソースのパスが空です。

# MT
duration-lt-1s = < 1 秒
# MT
duration-ms = { $ms } ミリ秒
# MT
duration-seconds = { $s } 秒
# MT
duration-minutes-seconds = { $m } 分 { $s } 秒
# MT
duration-hours-minutes = { $h } 時間 { $m } 分
# MT
duration-zero = 0 秒

# MT
rate-unit-per-second = { $size }/秒
