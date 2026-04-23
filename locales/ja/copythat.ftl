app-name = Copy That v1.0.0
# MT
window-title = Copy That v1.0.0
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
toast-clipboard-files-detected = クリップボードにファイル — 貼り付けショートカットを押して Copy That でコピー
toast-clipboard-no-files = クリップボードに貼り付けるファイルがありません
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
error-drawer-pending-count = 他のエラーが待機中
error-drawer-toggle = 折りたたむ / 展開する

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
err-path-escape = パスは拒否されました — 親ディレクトリ（..）セグメントまたは無効なバイトが含まれています
# MT
err-io-other = 不明な I/O エラー
err-sparseness-mismatch = 宛先でスパースレイアウトを保持できませんでした  # MT

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

# MT — Phase 11b Settings modal
settings-title = 設定
# MT
settings-tab-general = 一般
# MT
settings-tab-appearance = 外観
# MT
settings-section-language = 言語
# MT
settings-phase-12-hint = 他の設定 (テーマ、転送デフォルト、検証アルゴリズム、プロファイル) は Phase 12 で追加されます。

# MT — Phase 12 Settings window
settings-loading = 設定を読み込み中…
# MT
settings-tab-transfer = 転送
# MT
settings-tab-shell = シェル
# MT
settings-tab-secure-delete = セキュア削除
# MT
settings-tab-advanced = 詳細
# MT
settings-tab-profiles = プロファイル

# MT
settings-section-theme = テーマ
# MT
settings-theme-auto = 自動
# MT
settings-theme-light = ライト
# MT
settings-theme-dark = ダーク
# MT
settings-start-with-os = システム起動時に起動
# MT
settings-single-instance = インスタンスを 1 つだけ実行
# MT
settings-minimize-to-tray = 閉じるときにトレイに最小化
settings-error-display-mode = エラー通知スタイル
settings-error-display-modal = モーダル (アプリをブロック)
settings-error-display-drawer = ドロワー (非ブロック)
settings-error-display-mode-hint = モーダルは判断するまでキューを停止します。ドロワーはキューを継続させ、隅でエラーを整理できます。
settings-paste-shortcut = グローバルショートカットでファイルを貼り付け
settings-paste-shortcut-combo = ショートカットの組み合わせ
settings-paste-shortcut-hint = システムのどこででもこの組み合わせを押すと、Explorer / Finder / ファイルからコピーしたファイルを Copy That 経由で貼り付けます。CmdOrCtrl は macOS では Cmd、Windows / Linux では Ctrl に解決されます。
settings-clipboard-watcher = コピーされたファイルのクリップボードを監視
settings-clipboard-watcher-hint = ファイル URL がクリップボードに現れたときにトーストを表示し、Copy That で貼り付けられることを示します。有効時は 500 ms ごとにポーリング。

# MT
settings-buffer-size = バッファサイズ
# MT
settings-verify = コピー後に検証
# MT
settings-verify-off = オフ
# MT
settings-concurrency = 並行処理
# MT
settings-concurrency-auto = 自動
# MT
settings-reflink = Reflink / 高速パス
# MT
settings-reflink-prefer = 優先する
# MT
settings-reflink-avoid = reflink を避ける
# MT
settings-reflink-disabled = 常に非同期エンジンを使用
# MT
settings-fsync-on-close = 閉じるときにディスクへ同期 (低速、安全)
# MT
settings-preserve-timestamps = タイムスタンプを保持
# MT
settings-preserve-permissions = パーミッションを保持
# MT
settings-preserve-acls = ACL を保持 (Phase 14)
settings-preserve-sparseness = スパースファイルを保持  # MT
settings-preserve-sparseness-hint = スパースファイル (VM ディスク、データベースファイル) の割り当て済み範囲のみをコピーし、宛先がソースと同じディスク上のサイズを維持します。  # MT

# MT
settings-context-menu = シェルのコンテキストメニュー項目を有効化
# MT
settings-intercept-copy = 既定のコピーハンドラを横取り (Windows)
# MT
settings-intercept-copy-hint = オン時、Explorer の Ctrl+C / Ctrl+V は Copy That を経由します。登録は Phase 14。
# MT
settings-notify-completion = ジョブ完了時に通知

# MT
settings-shred-method = 既定の抹消方式
# MT
settings-shred-zero = ゼロ (1 回)
# MT
settings-shred-random = ランダム (1 回)
# MT
settings-shred-dod3 = DoD 5220.22-M (3 回)
# MT
settings-shred-dod7 = DoD 5220.22-M (7 回)
# MT
settings-shred-gutmann = Gutmann (35 回)
# MT
settings-shred-nist = NIST 800-88
# MT
settings-shred-confirm-twice = 抹消前に二重確認を要求

# MT
settings-log-level = ログレベル
# MT
settings-log-off = オフ
# MT
settings-telemetry = テレメトリ
# MT
settings-telemetry-never = 行わない — ログレベルに関わらずデータ送信なし
# MT
settings-error-policy = 既定のエラーポリシー
# MT
settings-error-policy-ask = 確認
# MT
settings-error-policy-skip = スキップ
# MT
settings-error-policy-retry = バックオフ付きで再試行
# MT
settings-error-policy-abort = 最初の失敗で中止
# MT
settings-history-retention = 履歴の保持期間 (日)
# MT
settings-history-retention-hint = 0 = 永久に保持。それ以外の値は起動時に古いジョブを自動削除します。
# MT
settings-database-path = データベースのパス
# MT
settings-database-path-default = (既定 — OS データディレクトリ)
# MT
settings-reset-all = 既定にリセット
# MT
settings-reset-confirm = すべての設定を既定にリセットしますか? プロファイルは変更されません。

# MT
settings-profiles-hint = 現在の設定に名前を付けて保存し、後で読み込んで個別のつまみを触らずに切り替えます。
# MT
settings-profile-name-placeholder = プロファイル名
# MT
settings-profile-save = 保存
# MT
settings-profile-import = インポート…
# MT
settings-profile-load = 読み込み
# MT
settings-profile-export = エクスポート…
# MT
settings-profile-delete = 削除
# MT
settings-profile-empty = 保存されたプロファイルはありません。
# MT
settings-profile-import-prompt = インポートされるプロファイルの名前:

# MT
toast-settings-reset = 設定をリセットしました
# MT
toast-profile-saved = プロファイルを保存しました
# MT
toast-profile-loaded = プロファイルを読み込みました
# MT
toast-profile-exported = プロファイルをエクスポートしました
# MT
toast-profile-imported = プロファイルをインポートしました

# Phase 13d — activity feed + header picker buttons
action-add-files = ファイルを追加
action-add-folders = フォルダを追加
activity-title = アクティビティ
activity-clear = アクティビティを消去
activity-empty = まだファイルアクティビティはありません。
activity-after-done = 完了時:
activity-keep-open = アプリを開いたままにする
activity-close-app = アプリを閉じる
activity-shutdown = PC をシャットダウン
activity-logoff = ログオフ
activity-sleep = スリープ

# Phase 14 — preflight free-space dialog
preflight-block-title = コピー先に十分な空き容量がありません
preflight-warn-title = コピー先の空き容量が少なくなっています
preflight-unknown-title = 空き容量を判別できません
preflight-unknown-body = コピー元のサイズが大きすぎて短時間で測定できないか、コピー先ボリュームが応答しませんでした。続行できます。空き容量がなくなった場合は、エンジンがコピーを安全に停止します。
preflight-required = 必要
preflight-free = 空き
preflight-reserve = 予約
preflight-shortfall = 不足
preflight-continue = それでも続行
collision-modal-overwrite-older = 古いファイルのみ上書き

# Phase 14e — subset picker
preflight-pick-subset = コピー対象を選択…
subset-title = コピーするソースを選択
subset-subtitle = すべての選択はコピー先に収まりません。コピーするものにチェックを入れてください。残りはスキップされます。
subset-loading = サイズを測定中…
subset-too-large = 大きすぎて計測できない
subset-budget = 利用可能
subset-remaining = 残り
subset-confirm = 選択をコピー
history-rerun-hint = このコピーを再実行 — ソースツリー内のすべてのファイルを再スキャン
history-clear-all = すべて消去
history-clear-all-confirm = もう一度クリックして確認
history-clear-all-hint = すべての履歴行を削除します。確認のためもう一度クリックが必要です。
toast-history-cleared = 履歴を消去しました ({ $count } 行削除)

# Phase 15 — source-list ordering
drop-dialog-sort-label = 並び順:
sort-custom = カスタム
sort-name-asc = 名前 A → Z（ファイルを先に）
sort-name-desc = 名前 Z → A（ファイルを先に）
sort-size-asc = サイズ 小 → 大（ファイルを先に）
sort-size-desc = サイズ 大 → 小（ファイルを先に）
sort-reorder = 並べ替え
sort-move-top = 最上部へ移動
sort-move-up = 上へ移動
sort-move-down = 下へ移動
sort-move-bottom = 最下部へ移動
sort-name-asc-simple = 名前 A → Z
sort-name-desc-simple = 名前 Z → A
sort-size-asc-simple = 小さい順
sort-size-desc-simple = 大きい順
activity-sort-locked = コピー中は並び替えを無効にしています。一時停止するか終了を待ってから順序を変更してください。
drop-dialog-collision-label = ファイルが既に存在する場合:
collision-policy-keep-both = 両方を保持 (新しいコピーを _2、_3 … にリネーム)
collision-policy-skip = 新しいコピーをスキップ
collision-policy-overwrite = 既存のファイルを上書き
collision-policy-overwrite-if-newer = 新しいときのみ上書き
collision-policy-prompt = 毎回確認
drop-dialog-busy-checking = 空き容量を確認中…
drop-dialog-busy-enumerating = ファイルを数えています…
drop-dialog-busy-starting = コピーを開始しています…
toast-enumeration-deferred = ソースツリーが大きいため事前リストを省略しました。エンジンが処理するごとに行が表示されます。

# Phase 14a — enumeration-time filters
# MT
settings-tab-filters = フィルター
# MT
settings-filters-hint = 列挙時にファイルをスキップし、エンジンが開くことすらしません。含めるはファイルのみ、除外はディレクトリも剪定します。
# MT
settings-filters-enabled = ツリーコピー時にフィルターを有効化
# MT
settings-filters-include-globs = 含めるグロブ
# MT
settings-filters-include-globs-placeholder = **/*.txt
# MT
settings-filters-include-globs-hint = 1 行につき 1 つ。指定があるときは、ファイルは少なくとも 1 つに一致する必要があります。ディレクトリは常に降下します。
# MT
settings-filters-exclude-globs = 除外グロブ
# MT
settings-filters-exclude-globs-placeholder = **/node_modules
# MT
settings-filters-exclude-globs-hint = 1 行につき 1 つ。ディレクトリに一致するとサブツリー全体が剪定され、ファイルに一致するとスキップされます。
# MT
settings-filters-size-range = ファイルサイズ範囲
# MT
settings-filters-min-size-bytes = 最小サイズ(バイト、空白 = 下限なし)
# MT
settings-filters-max-size-bytes = 最大サイズ(バイト、空白 = 上限なし)
# MT
settings-filters-date-range = 更新日時の範囲
# MT
settings-filters-min-mtime = 更新日がこれ以降
# MT
settings-filters-max-mtime = 更新日がこれ以前
# MT
settings-filters-attributes = 属性
# MT
settings-filters-skip-hidden = 隠しファイル / フォルダーをスキップ
# MT
settings-filters-skip-system = システムファイルをスキップ(Windows のみ)
# MT
settings-filters-skip-readonly = 読み取り専用ファイルをスキップ

# Phase 15 — auto-update
# MT
settings-tab-updater = アップデート
# MT
settings-updater-hint = Copy That は1日1回まで署名付きアップデートを確認します。アップデートは次回のアプリ終了時にインストールされます。
# MT
settings-updater-auto-check = 起動時にアップデートを確認する
# MT
settings-updater-channel = リリースチャンネル
# MT
settings-updater-channel-stable = 安定版
# MT
settings-updater-channel-beta = ベータ(プレリリース)
# MT
settings-updater-last-check = 最後の確認
# MT
settings-updater-last-never = 未確認
# MT
settings-updater-check-now = 今すぐアップデートを確認
# MT
settings-updater-checking = 確認中…
# MT
settings-updater-available = アップデートが利用可能
# MT
settings-updater-up-to-date = 最新リリースを使用しています。
# MT
settings-updater-dismiss = このバージョンをスキップ
# MT
settings-updater-dismissed = スキップ済み
# MT
toast-update-available = 新しいバージョンが利用可能です
# MT
toast-update-up-to-date = 既に最新バージョンです

# Phase 19a — disk-backed file enumeration (TeraCopy-compatible scan DB)
# MT
scan-progress-title = スキャン中…
# MT
scan-progress-stats = { $files } ファイル · これまでに { $bytes }
# MT
scan-pause-button = スキャンを一時停止
# MT
scan-resume-button = スキャンを再開
# MT
scan-cancel-button = スキャンをキャンセル
# MT
scan-cancel-confirm = スキャンをキャンセルして進行状況を破棄しますか？
# MT
scan-db-header = スキャン データベース
# MT
scan-db-hint = 数百万ファイル規模のジョブ向けのディスク上スキャン データベース。
# MT
advanced-scan-hash-during = スキャン中にチェックサムを計算
# MT
advanced-scan-db-path = スキャン データベースの場所
# MT
advanced-scan-retention-days = 完了したスキャンを自動削除（日数）
# MT
advanced-scan-max-keep = 保持するスキャン データベースの最大数

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
sparse-not-supported-title = 宛先がスパースファイルを埋めます  # MT
sparse-not-supported-body = { $dst_fs } はスパースファイルをサポートしていません。ソースの穴はゼロとして書き込まれ、宛先はディスク上で大きくなります。  # MT
sparse-warning-densified = スパースレイアウトを保持: 割り当て済み範囲のみコピーされました。  # MT
sparse-warning-mismatch = スパースレイアウトの不一致 — 宛先が予想より大きくなる可能性があります。  # MT
