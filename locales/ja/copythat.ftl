app-name = Copy That v0.19.84
window-title = Copy That v0.19.84
shred-ssd-advisory = 警告: この対象は SSD 上にあります。ウェアレベリングとオーバープロビジョニングによりデータが論理ブロックアドレスの下から移動するため、複数回の上書きではフラッシュメモリを確実に消去できません。ソリッドステートメディアには、ATA SECURE ERASE、NVMe Format（Secure Erase 付き）、または鍵を破棄した全ディスク暗号化を推奨します。

# Global aggregate states (header pill)
state-idle = アイドル
state-copying = コピー中
state-verifying = 検証中
state-paused = 一時停止中
state-error = エラー

# Per-job states (row badge)
state-pending = 待機中
state-running = 実行中
state-cancelled = キャンセル済み
state-succeeded = 完了
state-failed = 失敗

# Actions
action-pause = 一時停止
action-resume = 再開
action-cancel = キャンセル
action-pause-all = すべてのジョブを一時停止
action-resume-all = すべてのジョブを再開
action-cancel-all = すべてのジョブをキャンセル
action-close = 閉じる
action-reveal = フォルダーに表示
action-add-files = ファイルを追加
action-add-folders = フォルダーを追加

# Phase 13d — activity feed
activity-title = アクティビティ
activity-clear = アクティビティ一覧をクリア
activity-empty = ファイルアクティビティはまだありません。
activity-after-done = 完了したら:
activity-keep-open = アプリを開いたままにする
activity-close-app = アプリを終了
activity-shutdown = PC をシャットダウン
activity-logoff = ログオフ
activity-sleep = スリープ

# Phase 14 — preflight free-space dialog
preflight-block-title = コピー先の空き容量が足りません
preflight-warn-title = コピー先の空き容量が少なくなっています
preflight-unknown-title = 空き容量を確認できませんでした
preflight-unknown-body = ソースが大きすぎて素早くサイズを測定できないか、コピー先のボリュームが応答しませんでした。続行できます。容量が不足した場合、エンジンの容量ガードがコピーを安全に停止します。
preflight-required = 必要容量
preflight-free = 空き容量
preflight-reserve = 予約容量
preflight-shortfall = 不足分
preflight-continue = それでも続行
preflight-pick-subset = コピー対象を選択…
collision-modal-overwrite-older = 古いファイルのみ上書き

# Phase 14e — subset picker
subset-title = コピーするソースを選択
subset-subtitle = 選択したすべてはコピー先に収まりません。コピーする項目にチェックを入れてください。残りはそのまま残ります。
subset-loading = サイズを測定中…
subset-too-large = 大きすぎて計測不可
subset-budget = 利用可能
subset-remaining = 残り
subset-confirm = 選択項目をコピー
history-rerun-hint = このコピーを再実行 — ソースツリー内のすべてのファイルを再スキャンします
history-clear-all = すべてクリア
history-clear-all-confirm = もう一度クリックして確定
history-clear-all-hint = すべての履歴行を削除します。確定するにはもう一度クリックが必要です。
toast-history-cleared = 履歴をクリアしました（{ $count } 行を削除）

# Phase 15 — source-list ordering in the drop staging dialog
drop-dialog-sort-label = 並び順:
sort-custom = カスタム
sort-name-asc = 名前 A → Z（ファイルを先に）
sort-name-desc = 名前 Z → A（ファイルを先に）
sort-size-asc = サイズの小さい順（ファイルを先に）
sort-size-desc = サイズの大きい順（ファイルを先に）
sort-reorder = 並べ替え
sort-move-top = 先頭へ移動
sort-move-up = 上へ移動
sort-move-down = 下へ移動
sort-move-bottom = 末尾へ移動

# Phase 16 — sort preset names for the Activity list (shorter
# wording than the DropStagingDialog variants; the Activity list
# is files-only so the "(files first)" suffix isn't relevant).
sort-name-asc-simple = 名前 A → Z
sort-name-desc-simple = 名前 Z → A
sort-size-asc-simple = サイズの小さい順
sort-size-desc-simple = サイズの大きい順
activity-sort-locked = コピーの実行中は並べ替えできません。一時停止するか完了を待ってから順序を変更してください。

# Phase 17 — collision-policy picker in the DropStagingDialog
drop-dialog-collision-label = ファイルが既に存在する場合:
collision-policy-keep-both = 両方を保持（新しいコピーを _2、_3… にリネーム）
collision-policy-skip = 新しいコピーをスキップ
collision-policy-overwrite = 既存のファイルを上書き
collision-policy-overwrite-if-newer = 新しい場合のみ上書き
collision-policy-prompt = 毎回確認する

# Phase 18 — progress labels on the DropStagingDialog Start button
drop-dialog-busy-checking = 空き容量を確認中…
drop-dialog-busy-enumerating = ファイルを集計中…
drop-dialog-busy-starting = コピーを開始中…
toast-enumeration-deferred = ソースツリーが大きいため、事前のファイル一覧をスキップします。エンジンが処理を進めるにつれて行が表示されます。

# Context menu (per-row right-click)
menu-pause = 一時停止
menu-resume = 再開
menu-cancel = キャンセル
menu-remove = キューから削除
menu-reveal-source = ソースをフォルダーに表示
menu-reveal-destination = コピー先をフォルダーに表示

# Header / toolbar
header-eta-label = 推定残り時間
header-toolbar-label = グローバルコントロール

# Footer
footer-queued = 実行中のジョブ
footer-total-bytes = 転送中
footer-errors = エラー
footer-history = 履歴

# Empty state
empty-title = ファイルまたはフォルダーをドロップしてコピー
empty-hint = 項目をウィンドウにドラッグしてください。コピー先を確認した後、ソースごとに 1 つのジョブをキューに追加します。
empty-region-label = ジョブ一覧

# Details drawer
details-drawer-label = ジョブの詳細
details-source = ソース
details-destination = コピー先
details-state = 状態
details-bytes = バイト数
details-files = ファイル数
details-speed = 速度
details-eta = 残り時間
details-error = エラー

# Drop dialog
drop-dialog-title = ドロップした項目を転送
drop-dialog-subtitle = { $count } 件の項目を転送する準備ができました。コピー先フォルダーを選択して開始してください。
drop-dialog-mode = 操作
drop-dialog-copy = コピー
drop-dialog-move = 移動
drop-dialog-pick-destination = コピー先を選択
drop-dialog-change-destination = コピー先を変更
drop-dialog-start-copy = コピーを開始
drop-dialog-start-move = 移動を開始

# ETA placeholders
eta-calculating = 計算中…
eta-unknown = 不明

# Toast messages
toast-job-done = 転送が完了しました
toast-copy-queued = コピーをキューに追加しました
toast-move-queued = 移動をキューに追加しました
toast-error-resolved = エラーを解決しました
toast-collision-resolved = 競合を解決しました
toast-elevated-unavailable = 昇格してのリトライは Phase 17 で実装予定です — まだ利用できません
toast-clipboard-files-detected = クリップボードにファイルがあります — 貼り付けショートカットを押すと Copy That でコピーできます
toast-clipboard-no-files = クリップボードに貼り付けるファイルがありません
toast-error-log-exported = エラーログをエクスポートしました

# Error modal (Phase 8)
error-modal-title = 転送に失敗しました
error-modal-retry = リトライ
error-modal-retry-elevated = 昇格した権限でリトライ
error-modal-skip = スキップ
error-modal-skip-all-kind = この種類のエラーをすべてスキップ
error-modal-abort = すべて中止
error-modal-path-label = パス
error-modal-code-label = コード
error-drawer-pending-count = 他にもエラーが待機中
error-drawer-toggle = 折りたたみ／展開

# Error-kind labels (Phase 8). Source of truth — engine maps each
# `CopyErrorKind` to one of these keys via `localized_key()`.
err-not-found = ファイルが見つかりません
err-permission-denied = アクセスが拒否されました
err-disk-full = コピー先のディスクがいっぱいです
err-interrupted = 操作が中断されました
err-verify-failed = コピー後の検証に失敗しました
err-path-escape = パスが拒否されました — 親ディレクトリ（..）セグメントまたは不正なバイトが含まれています
err-path-invalid-encoding = パスが拒否されました — 文字列に無効な UTF-8 / 置換文字が含まれています
err-helper-invalid-json = 特権ヘルパーが不正な形式の JSON を受信しました。この要求を無視します
err-helper-grant-out-of-band = GrantCapabilities はステートレスハンドラーではなくヘルパーの run-loop で処理する必要があります
err-randomness-unavailable = OS の乱数生成器が失敗しました。セッション ID を生成できません
err-sparseness-mismatch = コピー先でスパースレイアウトを保持できませんでした
err-io-other = 不明な I/O エラー

# Collision modal (Phase 8)
collision-modal-title = ファイルが既に存在します
collision-modal-overwrite = 上書き
collision-modal-overwrite-if-newer = 新しい場合は上書き
collision-modal-skip = スキップ
collision-modal-keep-both = 両方を保持
collision-modal-rename = リネーム…
collision-modal-apply-to-all = すべてに適用
collision-modal-source = ソース
collision-modal-destination = コピー先
collision-modal-size = サイズ
collision-modal-modified = 更新日時
collision-modal-hash-check = クイックハッシュ（SHA-256）
collision-modal-hash-computing = 計算中…
collision-modal-hash-identical = 一致
collision-modal-hash-different = 不一致
collision-modal-rename-placeholder = 新しいファイル名
collision-modal-confirm-rename = リネーム

# Error log drawer (Phase 8)
error-log-title = エラーログ
error-log-empty = 記録されたエラーはありません
error-log-export-csv = CSV をエクスポート
error-log-export-txt = テキストをエクスポート
error-log-clear = ログをクリア
error-log-col-time = 時刻
error-log-col-job = ジョブ
error-log-col-path = パス
error-log-col-code = コード
error-log-col-message = メッセージ
error-log-col-resolution = 解決方法

# History drawer (Phase 9)
history-title = 履歴
history-empty = 記録されたジョブはまだありません
history-unavailable = コピー履歴を利用できません。起動時に SQLite ストアを開けませんでした。
history-filter-any = すべて
history-filter-kind = 種類
history-filter-status = ステータス
history-filter-text = 検索
history-refresh = 更新
history-export-csv = CSV をエクスポート
history-purge-30 = 30 日以上前を削除
history-rerun = 再実行
history-detail-open = 詳細
history-detail-title = ジョブの詳細
history-detail-empty = 記録された項目はありません
history-col-date = 日付
history-col-kind = 種類
history-col-src = ソース
history-col-dst = コピー先
history-col-files = ファイル数
history-col-size = サイズ
history-col-status = ステータス
history-col-duration = 所要時間
history-col-error = エラー
toast-history-exported = 履歴をエクスポートしました
toast-history-rerun-queued = 再実行をキューに追加しました

# Totals drawer (Phase 10)
footer-totals = 合計
totals-title = 合計
totals-loading = 合計を読み込み中…
totals-card-bytes = コピーした合計バイト数
totals-card-files = ファイル数
totals-card-jobs = ジョブ数
totals-card-avg-rate = 平均スループット
totals-errors = エラー
totals-spark-title = 過去 30 日間
totals-kinds-title = 種類別
totals-saved-title = 節約時間（推定）
totals-saved-note = 同じ作業を標準のファイルマネージャーでコピーした場合との推定比較です。
totals-reset = 統計をリセット
totals-reset-confirm = 保存されたすべてのジョブと項目を削除します。続行しますか？
totals-reset-confirm-yes = はい、リセットします
toast-totals-reset = 統計をリセットしました

# Phase 11a — i18n core: surface remaining user-visible strings so
# every string on a main-window screen flows through Fluent.

# Header language switcher (temporary placement — folds into
# Settings → General in Phase 12).
header-language-label = 言語
header-language-title = 言語を変更

# Job-kind labels. History filter, Totals breakdown, and history
# rows all map wire-format `kind` strings to these labels.
kind-copy = コピー
kind-move = 移動
kind-delete = 削除
kind-secure-delete = 安全な削除

# History status labels. Distinct from `state-*` because the
# history wire format uses plain `running`/`succeeded`/... rather
# than the live `JobState` enum the queue exposes.
status-running = 実行中
status-succeeded = 成功
status-failed = 失敗
status-cancelled = キャンセル済み
# Per-item status (not job-level): `ok` / `skipped` are only
# reachable from the history detail view.
status-ok = OK
status-skipped = スキップ済み

# History drawer: search field placeholder + purge toast.
history-search-placeholder = /path
toast-history-purged = 30 日以上前の { $count } 件のジョブを削除しました

# User-facing command-layer validation errors. The Rust side
# returns these keys when input is missing; the toast layer looks
# them up rather than showing the raw English.
err-source-required = ソースパスが少なくとも 1 つ必要です。
err-destination-empty = コピー先のパスが空です。
err-source-empty = ソースパスが空です。

# Localised duration formatting for the Totals drawer and ETA
# fields. `{ $ms }`, `{ $s }`, `{ $m }`, `{ $h }` are integer
# placeables — the formatter passes pre-computed values in.
duration-lt-1s = < 1秒
duration-ms = { $ms } ミリ秒
duration-seconds = { $s }秒
duration-minutes-seconds = { $m }分 { $s }秒
duration-hours-minutes = { $h }時間 { $m }分
duration-zero = 0秒

# Rate unit. Appended to a formatted byte size. Some languages
# render this with a leading space ("Ko/s"); keep it translatable
# even though the SI-derived "/s" is near-universal.
rate-unit-per-second = { $size }/s

# Phase 11b — Settings modal skeleton. Phase 12 expanded this into a
# full six-tab preferences window; the `settings-phase-12-hint` key
# is retired in favour of `settings-tab-profiles` + concrete labels.
settings-title = 設定
settings-tab-general = 一般
settings-tab-appearance = 外観
settings-section-language = 言語
settings-phase-12-hint = その他の設定（テーマ、転送のデフォルト、検証アルゴリズム、プロファイル）は Phase 12 で追加されます。

# Phase 12 — full Settings window keys. Grouped by tab so the
# human-review pass can pick one section at a time. Every field
# label and every option label that the user will actually read
# has its own key; dynamic wire values (enum discriminants like
# "auto" / "prefer") stay untranslated on the wire.

settings-loading = 設定を読み込み中…
settings-tab-transfer = 転送
settings-tab-filters = フィルター
settings-tab-shell = シェル
settings-tab-secure-delete = 安全な削除
settings-tab-advanced = 詳細設定
settings-tab-updater = 更新
settings-tab-profiles = プロファイル

# General tab additions
settings-section-theme = テーマ
settings-theme-auto = 自動
settings-theme-light = ライト
settings-theme-dark = ダーク
settings-start-with-os = システム起動時に自動起動
settings-single-instance = 単一インスタンスで実行
settings-minimize-to-tray = 閉じるときにトレイへ最小化
settings-error-display-mode = エラー表示のスタイル
settings-error-display-modal = モーダル（アプリをブロック）
settings-error-display-drawer = ドロワー（ブロックなし）
settings-error-display-mode-hint = モーダルは判断するまでキューを停止します。ドロワーはキューを動かし続け、隅でエラーを処理できます。
settings-paste-shortcut = グローバルショートカットでファイルを貼り付け
settings-paste-shortcut-combo = ショートカットの組み合わせ
settings-paste-shortcut-hint = この組み合わせをシステム上のどこでも押すと、Explorer / Finder / Files でコピーしたファイルを Copy That で貼り付けられます。CmdOrCtrl は macOS では Cmd、Windows / Linux では Ctrl に対応します。
settings-clipboard-watcher = コピーされたファイルをクリップボードで監視
settings-clipboard-watcher-hint = ファイルの URL がクリップボードに表示されたときにトーストを表示し、Copy That で貼り付けられることを知らせます。有効な間は 500 ミリ秒ごとにポーリングします。

# Transfer tab
settings-buffer-size = バッファサイズ
settings-verify = コピー後に検証
settings-verify-off = オフ
settings-concurrency = 並列実行数
settings-concurrency-auto = 自動
settings-reflink = Reflink / 高速パス
settings-reflink-prefer = 優先
settings-reflink-avoid = Reflink を回避
settings-reflink-disabled = 常に非同期エンジンを使用
settings-fsync-on-close = 閉じるときにディスクへ同期（低速・安全）
settings-preserve-timestamps = タイムスタンプを保持
settings-preserve-permissions = アクセス権を保持
settings-preserve-acls = ACL を保持（Phase 14）
settings-preserve-sparseness = スパースファイルを保持
settings-preserve-sparseness-hint = スパースファイル（VM ディスク、データベースファイル）の割り当て済みエクステントのみをコピーし、コピー先のディスク上サイズがソースと同じになるようにします。
settings-force-parallel-chunks = 並列マルチチャンクコピー（RAID/アレイのみ）
settings-force-parallel-chunks-hint = 大きなコピーを並行チャンクに分割します。ストライプ/RAID/ネットワーク宛先でのみ有効で、単一の SSD/NVMe では遅くなります（-25%〜-76%）。宛先が複数ディスクのアレイでない限りオフのままにしてください。

# Shell tab
settings-context-menu = シェルのコンテキストメニュー項目を有効化
settings-intercept-copy = 既定のコピーハンドラーをインターセプト（Windows）
settings-intercept-copy-hint = 有効にすると、Explorer の Ctrl+C / Ctrl+V が Copy That を経由します。登録は Phase 14 で実装されます。
settings-notify-completion = ジョブ完了時に通知

# Secure delete tab
settings-shred-method = 既定のシュレッド方式
settings-shred-zero = ゼロ（1 パス）
settings-shred-random = ランダム（1 パス）
settings-shred-dod3 = DoD 5220.22-M（3 パス）
settings-shred-dod7 = DoD 5220.22-M（7 パス）
settings-shred-gutmann = Gutmann（35 パス）
settings-shred-nist = NIST 800-88
settings-shred-confirm-twice = シュレッド前に二重確認を要求

# Advanced tab
settings-log-level = ログレベル
settings-log-off = オフ
settings-telemetry = テレメトリ
settings-telemetry-never = なし — どのログレベルでも外部送信しません
settings-error-policy = 既定のエラーポリシー
settings-error-policy-ask = 確認する
settings-error-policy-skip = スキップ
settings-error-policy-retry = バックオフ付きでリトライ
settings-error-policy-abort = 最初の失敗で中止
settings-history-retention = 履歴の保持期間（日数）
settings-history-retention-hint = 0 = 無期限に保持。それ以外の値では起動時に古いジョブを自動削除します。
settings-database-path = データベースのパス
settings-database-path-default = （既定 — OS のデータディレクトリ）
settings-reset-all = 既定値にリセット
settings-reset-confirm = すべての設定を既定値にリセットしますか？プロファイルには影響しません。

# Profiles tab
settings-profiles-hint = 現在の設定に名前を付けて保存し、後で読み込めば個々の項目を触らずに元に戻せます。
settings-profile-name-placeholder = プロファイル名
settings-profile-save = 保存
settings-profile-import = インポート…
settings-profile-load = 読み込み
settings-profile-export = エクスポート…
settings-profile-delete = 削除
settings-profile-empty = 保存されたプロファイルはまだありません。
settings-profile-import-prompt = インポートするプロファイルの名前:

# Toasts driven by Phase 12 profile actions
toast-settings-reset = 設定をリセットしました
toast-profile-saved = プロファイルを保存しました
toast-profile-loaded = プロファイルを読み込みました
toast-profile-exported = プロファイルをエクスポートしました
toast-profile-imported = プロファイルをインポートしました

# Phase 14a — enumeration-time filters
settings-filters-hint = 列挙時にファイルをスキップして、エンジンが一切開かないようにします。include はファイルのみに適用され、exclude は一致するディレクトリも除外します。
settings-filters-enabled = ツリーコピーでフィルターを有効化
settings-filters-include-globs = 含めるグロブ
settings-filters-include-globs-placeholder = **/*.txt
settings-filters-include-globs-hint = 1 行に 1 つのグロブを記述します。空でない場合、ファイルは少なくとも 1 つの include に一致しないと残りません。ディレクトリには常に再帰的に入ります。
settings-filters-exclude-globs = 除外するグロブ
settings-filters-exclude-globs-placeholder = **/node_modules
settings-filters-exclude-globs-hint = 1 行に 1 つのグロブを記述します。ディレクトリに一致するとサブツリー全体を除外し、ファイルに一致するとスキップします。
settings-filters-size-range = ファイルサイズの範囲
settings-filters-min-size-bytes = 最小サイズ（バイト、空白 = 下限なし）
settings-filters-max-size-bytes = 最大サイズ（バイト、空白 = 上限なし）
settings-filters-date-range = 更新日時の範囲
settings-filters-min-mtime = この日時以降に更新
settings-filters-max-mtime = この日時以前に更新
settings-filters-attributes = 属性ビット
settings-filters-skip-hidden = 隠しファイル／フォルダーをスキップ
settings-filters-skip-system = システムファイルをスキップ（Windows のみ）
settings-filters-skip-readonly = 読み取り専用ファイルをスキップ

# Phase 15 — auto-update
settings-updater-hint = Copy That は署名済みの更新を 1 日に最大 1 回確認します。更新は次回のアプリ終了時にインストールされます。
settings-updater-auto-check = 起動時に更新を確認
settings-updater-channel = リリースチャンネル
settings-updater-channel-stable = 安定版
settings-updater-channel-beta = ベータ版（プレリリース）
settings-updater-last-check = 最終確認
settings-updater-last-never = なし
settings-updater-check-now = 今すぐ更新を確認
settings-updater-checking = 確認中…
settings-updater-available = 更新があります
settings-updater-up-to-date = 最新のリリースを使用しています。
settings-updater-dismiss = このバージョンをスキップ
settings-updater-dismissed = スキップ済み
toast-update-available = 新しいバージョンが利用可能です
toast-update-up-to-date = 既に最新バージョンです

# Phase 19a — disk-backed file enumeration (TeraCopy-compatible scan DB)
scan-progress-title = スキャン中…
scan-progress-stats = { $files } ファイル · これまでに { $bytes }
scan-pause-button = スキャンを一時停止
scan-resume-button = スキャンを再開
scan-cancel-button = スキャンをキャンセル
scan-cancel-confirm = スキャンをキャンセルして進捗を破棄しますか？
scan-db-header = スキャンデータベース
scan-db-hint = 数百万ファイル規模のジョブ向けのディスク上スキャンデータベース。
advanced-scan-hash-during = スキャン中にチェックサムを計算
advanced-scan-db-path = スキャンデータベースの場所
advanced-scan-retention-days = 完了したスキャンを自動削除するまでの日数
advanced-scan-max-keep = 保持するスキャンデータベースの最大数

# Phase 19b — filesystem-snapshot source for locked files.
settings-on-locked = ファイルがロックされている場合
settings-on-locked-ask = 初回に確認する
settings-on-locked-retry = 短時間リトライした後、エラーを表示
settings-on-locked-skip = ロックされたファイルをスキップ
settings-on-locked-snapshot = ファイルシステムのスナップショットを使用
settings-on-locked-hint = 「他のプロセスがファイルを使用中」エラーを解消します。Copy That がソースボリュームのスナップショットを作成し（Windows では VSS、Linux では ZFS/Btrfs、macOS では APFS）、スナップショットのコピーから読み取ります。
snapshot-prompt-title = このファイルは他のプロセスが使用中です
snapshot-prompt-body = 別のプログラムが { $path } を排他書き込みで開いています。Copy That がこのファイルおよび同じボリューム上の同様のファイルをどう扱うかを選択してください。
snapshot-source-active = 📷 { $volume } の { $kind } スナップショットから読み取り中
snapshot-create-failed = ソースボリュームのスナップショットを作成できませんでした
snapshot-vss-needs-elevation = VSS スナップショットからの読み取りには管理者権限が必要です。Copy That が許可を求めます。
snapshot-cleanup-failed = スナップショットヘルパーがクリーンアップの失敗を報告しました — 残留したシャドウコピーがボリュームに残っている可能性があります。

# Phase 20 — durable resume journal.
resume-prompt-title = 前回の転送を再開しますか？
resume-prompt-body = Copy That は前回のセッションから { $count } 件の未完了の転送を検出しました。それぞれの処理を選択してください。
resume-prompt-resume = 再開
resume-prompt-resume-all = すべて再開
resume-discard-one = 再開しない
resume-discard-all = すべて破棄
resume-aborted-hash-mismatch = コピー先の先頭 { $offset } バイトがソースと一致しません — 最初からやり直します。
settings-auto-resume = 確認なしで中断したジョブを自動再開
settings-auto-resume-hint = 起動時の再開確認をスキップし、未完了のジョブをすべて自動的にキューへ再追加します。既定ではオフです。

# Phase 21 — bandwidth shaping (GCRA token bucket + schedule + auto-throttle).
settings-tab-network = ネットワーク
settings-network-hint = 転送速度を制限して、ネットワークの残りを使えるように保ちます。全体に適用したり、日次スケジュールに従ったり、従量制 Wi-Fi / バッテリー / モバイル通信に自動的に反応させたりできます。
settings-network-mode = 帯域制限
settings-network-mode-off = オフ（制限なし）
settings-network-mode-fixed = 固定値
settings-network-mode-schedule = スケジュールを使用
settings-network-cap-mbps = 上限（MB/s）
settings-network-schedule = スケジュール（rclone 形式）
settings-network-schedule-hint = 空白区切りの HH:MM,rate の境界に加えて、任意で Mon-Fri,rate の曜日ルールを指定します。レート: 512k、10M、2G、off、unlimited。例: 08:00,512k 18:00,10M Sat-Sun,unlimited。
settings-network-auto-header = 自動スロットリング
settings-network-auto-metered = 従量制 Wi-Fi のとき
settings-network-auto-battery = バッテリー駆動のとき
settings-network-auto-cellular = モバイル通信のとき
settings-network-auto-unchanged = 上書きしない
settings-network-auto-pause = 転送を一時停止
settings-network-auto-cap = 固定値に制限
shape-badge-paused = 一時停止中
shape-badge-tooltip = 帯域制限が有効です — クリックして設定 → ネットワークを開く
shape-badge-source-schedule = スケジュール
shape-badge-source-metered = 従量制
shape-badge-source-battery = バッテリー駆動
shape-badge-source-cellular = モバイル通信
shape-badge-source-settings = 有効
shape-error-schedule-invalid = スケジュール形式が無効です: { $message }

# Phase 22 — aggregate conflict dialog v2 (thumbnails, per-pattern
# rules, and reusable conflict profiles). Every key below is user-
# visible text in the `ConflictBatchModal.svelte` component.
conflict-batch-title = { $jobname } 内の { $count } 件のファイル競合
conflict-batch-state-pending = 保留中
conflict-batch-state-resolved = 解決済み
conflict-batch-action-overwrite = 上書き
conflict-batch-action-skip = スキップ
conflict-batch-action-keep-both = 両方を保持
conflict-batch-action-newer-wins = 新しい方を優先
conflict-batch-action-larger-wins = 大きい方を優先
conflict-batch-bulk-apply-selected = 選択項目に適用
conflict-batch-bulk-apply-extension = この拡張子すべてに適用
conflict-batch-bulk-apply-glob = 一致するグロブに適用…
conflict-batch-bulk-apply-remaining = 残りすべてに適用
conflict-batch-bulk-glob-placeholder = 例: **/*.tmp
conflict-batch-save-profile = これらのルールをプロファイルとして保存…
conflict-batch-profile-placeholder = プロファイル名
conflict-batch-matched-rule = ルール '{ $rule }' により → { $action }
conflict-batch-empty = すべての競合を解決しました
conflict-batch-source-vs-destination = ソース vs. コピー先
conflict-batch-source-label = ソース
conflict-batch-destination-label = コピー先
conflict-batch-size-label = サイズ
conflict-batch-modified-label = 更新日時
conflict-batch-close = 閉じる
conflict-batch-profile-saved = 競合プロファイルを保存しました

# Phase 23 — sparse-file preservation. The toast fires once per
# destination volume when the filesystem can't preserve holes; the
# warning line is surfaced in the job detail drawer so the user knows
# the dst is larger on disk than the source was.
sparse-not-supported-title = コピー先がスパースファイルを埋めます
sparse-not-supported-body = { $dst_fs } はスパースファイルをサポートしていません。ソース内の穴がゼロとして書き出されたため、コピー先はディスク上で大きくなっています。
sparse-warning-densified = スパースレイアウトを保持しました: 割り当て済みエクステントのみがコピーされました。
sparse-warning-mismatch = スパースレイアウトの不一致 — コピー先が予想より大きくなっている可能性があります。

# Phase 24 — security-metadata preservation. The Mark-of-the-Web
# (Zone.Identifier ADS) toggle is security-sensitive: turning it off
# lets a downloaded executable shed its SmartScreen / Office Protected
# View flag on copy, which is why the tooltip carries an explicit
# warning. AppleDouble fallback emits `._<filename>` sidecars on
# destination filesystems that can't hold the foreign metadata.
settings-preserve-security-metadata = セキュリティメタデータを保持
settings-preserve-security-metadata-hint = コピーのたびに帯域外メタデータストリーム（NTFS ADS / xattr / POSIX ACL / SELinux コンテキスト / Linux ファイルケーパビリティ / macOS リソースフォーク）を取得して再適用します。
settings-preserve-motw = Mark-of-the-Web（インターネットからダウンロードのフラグ）を保持
settings-preserve-motw-hint = セキュリティに重要です。SmartScreen と Office 保護ビューはこのストリームを使ってインターネットからダウンロードされたファイルを警告します。無効にすると、ダウンロードされた実行ファイルがコピー時に出所のマーカーを失い、OS の保護を回避できてしまいます。
settings-preserve-posix-acls = POSIX ACL と拡張属性を保持
settings-preserve-posix-acls-hint = user.* / system.* / trusted.* の xattr と POSIX アクセス制御リストをコピーに引き継ぎます。
settings-preserve-selinux = SELinux コンテキストを保持
settings-preserve-selinux-hint = security.selinux ラベルをコピーに引き継ぎ、MAC ポリシー下で動作するデーモンが引き続きファイルにアクセスできるようにします。
settings-preserve-resource-forks = macOS のリソースフォークと Finder 情報を保持
settings-preserve-resource-forks-hint = 従来のリソースフォークと FinderInfo（カラータグ、Carbon メタデータ）をコピーに引き継ぎます。
settings-appledouble-fallback = 互換性のないファイルシステムでは AppleDouble サイドカーを使用
meta-translated-to-appledouble = 外部メタデータを AppleDouble サイドカー（._{ $ext }）に保存しました

# Phase 25 — two-way sync with vector-clock conflict detection.
# The drawer lists configured sync pairs; each pair runs an
# independent reconciliation round against a per-pair `.copythat-sync.db`
# state store. Concurrent edits from a common ancestor surface as
# conflicts rather than silent overwrites; the losing side's content
# is preserved as `name.sync-conflict-YYYYMMDD-HHMMSS-<host>.ext`.
footer-sync = 同期
sync-drawer-title = 双方向同期
sync-drawer-hint = 2 つのフォルダーを暗黙の上書きなしで同期します。同時編集は解決可能な競合として表示されます。
sync-add-pair = ペアを追加
sync-add-cancel = キャンセル
sync-refresh = 更新
sync-add-save = ペアを保存
sync-add-saving = 保存中…
sync-add-missing-fields = ラベル、左のパス、右のパスはすべて必須です。
sync-remove-confirm = この同期ペアを削除しますか？状態データベースは保持され、フォルダーには影響しません。
sync-field-label = ラベル
sync-field-label-placeholder = 例: ドキュメント ↔ NAS
sync-field-left = 左のフォルダー
sync-field-left-placeholder = 絶対パスを選択または貼り付け
sync-field-right = 右のフォルダー
sync-field-right-placeholder = 絶対パスを選択または貼り付け
sync-field-mode = モード
sync-mode-two-way = 双方向
sync-mode-mirror-left-to-right = ミラー（左 → 右）
sync-mode-mirror-right-to-left = ミラー（右 → 左）
sync-mode-contribute-left-to-right = 追加のみ（左 → 右、削除なし）
sync-no-pairs = 同期ペアはまだ設定されていません。「ペアを追加」をクリックして開始してください。
sync-loading = 設定済みのペアを読み込み中…
sync-never-run = 未実行
sync-running = 実行中
sync-run-now = 今すぐ実行
sync-cancel = キャンセル
sync-remove-pair = 削除
sync-view-conflicts = 競合を表示（{ $count }）
sync-conflicts-heading = 競合
sync-no-conflicts = 前回の実行で競合はありませんでした。
sync-winner = 優先された方
sync-side-left-to-right = 左
sync-side-right-to-left = 右
sync-conflict-kind-concurrent-write = 同時編集
sync-conflict-kind-delete-edit = 削除 ↔ 編集
sync-conflict-kind-add-add = 両側で追加
sync-conflict-kind-corrupt-equal = 新しい書き込みなしに内容が分岐
sync-resolve-keep-left = 左を保持
sync-resolve-keep-right = 右を保持
sync-resolve-keep-both = 両方を保持
sync-resolve-three-way = 3 ウェイマージで解決
sync-resolve-phase-53-tooltip = テキスト以外のファイルのインタラクティブな 3 ウェイマージは Phase 53 で実装されます。
sync-error-prefix = 同期エラー

# Phase 26 — real-time mirror watcher. "Live mirror" starts a
# filesystem watcher on the pair's left side; every debounced event
# triggers a re-sync. The watcher filters vim swap files / Office
# lock files / atomic-save temp names so one logical save becomes
# exactly one sync round.
live-mirror-start = ライブミラーを開始
live-mirror-stop = ライブミラーを停止
live-mirror-watching = 監視中
live-mirror-toggle-hint = 検出されたファイルシステムの変更ごとに自動で再同期します。アクティブなペアごとにバックグラウンドスレッドが 1 つ動作します。
watch-event-prefix = ファイルの変更
watch-overflow-recovered = ウォッチャーのバッファがオーバーフローしました。回復のため再列挙しています

# Phase 27 — content-defined chunk store. Enables delta-resume (a
# retry only re-writes chunks that actually changed) and same-job
# dedup (files sharing content blocks store those blocks once). The
# store is disk-backed under `<data-dir>/chunks/` by default and is
# the foundation for the Phase 49–51 moonshot repository phases.
chunk-store-section = チャンクストア
chunk-store-enable = チャンクストアを有効化（差分再開と重複排除）
chunk-store-enable-hint = コピーするすべてのファイルを内容で分割し（FastCDC）、チャンクを内容アドレスで保存します。リトライでは変更されたチャンクのみを書き直し、内容を共有するファイルは自動的に重複排除されます。
chunk-store-location = チャンクストアの場所
chunk-store-max-size = チャンクストアの最大サイズ
chunk-store-prune = 指定日数より古いチャンクを削除
chunk-store-savings = チャンク重複排除により { $gib } GiB を節約
chunk-store-disk-usage = { $chunks } 個のチャンクで { $size } を使用中

# Phase 28 — tray-resident Drop Stack. The stack is a persistent list
# of paths gathered from multiple sources (Explorer drag, main-window
# context menu, CLI, drag onto the Drop Stack window) that the user
# can dispatch to a destination in one go.
dropstack-window-title = Drop Stack
dropstack-tray-open = Drop Stack
dropstack-empty-title = Drop Stack は空です
dropstack-empty-hint = Explorer からここにファイルをドラッグするか、ジョブ行を右クリックして追加してください。
dropstack-add-to-stack = Drop Stack に追加
dropstack-copy-all-to = すべてを次の場所にコピー…
dropstack-move-all-to = すべてを次の場所に移動…
dropstack-clear = スタックをクリア
dropstack-remove-row = スタックから削除
dropstack-path-missing-toast = { $path } をドロップしました — このファイルはもう存在しません。
dropstack-always-on-top = Drop Stack を常に最前面に表示
dropstack-show-tray-icon = Copy That のトレイアイコンを表示
dropstack-open-on-start = アプリ起動時に Drop Stack を自動で開く
dropstack-count = { $count } 件のパス

# Phase 29 — spring-loaded folders + native DnD polish. The Settings
# → General tab carries the knobs; DropTarget / DestinationPicker
# Svelte components use the dropzone-* keys at runtime.
settings-dnd-heading = ドラッグ＆ドロップ
settings-dnd-spring-load = ドラッグ中にフォルダーをスプリングロード
settings-dnd-spring-delay = スプリングロードの遅延（ミリ秒）
settings-dnd-thumbnails = ドラッグ時にサムネイルを表示
settings-dnd-invalid-highlight = 無効なドロップ先をハイライト
dropzone-invalid-title = 有効なドロップ先ではありません
dropzone-invalid-readonly = コピー先が読み取り専用です
dropzone-picker-title = コピー先を選択
dropzone-picker-up = 上へ
dropzone-picker-path = 現在のパス
dropzone-picker-root = ルート
dropzone-picker-use-this = このフォルダーを使用
dropzone-picker-empty = サブフォルダーがありません
dropzone-picker-cancel = キャンセル

# Phase 30 — cross-platform path translation. Settings → Transfer
# exposes these under a "Cross-platform compatibility" subsection.
translate-heading = クロスプラットフォーム互換性
translate-unicode-label = Unicode 正規化
translate-unicode-auto = コピー先を自動検出
translate-unicode-windows = NFC（Windows / Linux）
translate-unicode-macos = そのまま（macOS / APFS）
translate-line-endings-label = テキストファイルの改行コードを変換
translate-line-endings-allowlist = テキストファイルの拡張子
reserved-name-label = Windows 予約名の処理
reserved-name-suffix = 「_」を付加（CON.txt → CON_.txt）
reserved-name-reject = 拒否して警告
long-path-label = 260 文字を超える場合に Windows ロングパスのプレフィックス（\\?\）を使用
long-path-hint = 一部のネットワーク共有や従来のツールは \\?\ 名前空間に対応していません。

# Phase 31 — power-aware copying. Settings → Power & State tab + the
# header badge that renders "⏸ Paused — Zoom call detected" when the
# runner's power subscriber has paused or capped due to a policy match.
power-heading = 電源と状態
power-enabled = 電源対応ルールを有効化
power-battery-label = バッテリー駆動のとき
power-metered-label = 従量制 Wi-Fi のとき
power-cellular-label = モバイル通信のとき
power-presentation-label = プレゼン中（Zoom / Teams / Keynote）
power-fullscreen-label = アプリが全画面のとき
power-thermal-label = CPU がサーマルスロットリング中のとき
power-rule-continue = 全速力で続行
power-rule-pause = すべてのジョブを一時停止
power-rule-cap = 帯域を制限
power-rule-cap-percent = 現在のレートの割合に制限
power-reason-on-battery = バッテリー駆動
power-reason-metered-network = 従量制ネットワーク
power-reason-cellular-network = モバイルネットワーク
power-reason-presenting = プレゼンモード
power-reason-fullscreen = 全画面アプリ
power-reason-thermal-throttling = CPU がスロットリング中

# Phase 32 — cloud backend matrix via OpenDAL. Settings → Remotes
# tab + the Add-backend wizard that writes one entry per remote into
# the keychain-backed credential store.
remote-heading = リモートバックエンド
remote-add = バックエンドを追加
remote-list-empty = リモートバックエンドが設定されていません
remote-test = 接続をテスト
remote-test-success = 接続に成功しました
remote-test-failed = 接続に失敗しました
remote-remove = バックエンドを削除
remote-name-label = 表示名
remote-kind-label = バックエンドの種類
remote-save = バックエンドを保存
remote-cancel = キャンセル
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
backend-local-fs = ローカルファイルシステム
cloud-config-bucket = バケット
cloud-config-region = リージョン
cloud-config-endpoint = エンドポイント URL
cloud-config-root = ルートパス
cloud-error-invalid-config = バックエンドの構成が無効です
cloud-error-network = バックエンドへの接続中にネットワークエラーが発生しました
cloud-error-not-found = 指定されたパスにオブジェクトが見つかりません
cloud-error-permission = リモートバックエンドによってアクセスが拒否されました
cloud-error-keychain = OS キーチェーンへのアクセスに失敗しました
settings-tab-remotes = リモート
settings-tab-mobile = モバイル

# Phase 33 — mount Copy That's chunk store + history archive as a
# read-only filesystem (FUSE / WinFsp). Surfaced by the History tab
# context menu's "Mount snapshot" action + the Settings → Advanced
# "Mount latest on launch" toggle.
mount-heading = スナップショットをマウント
mount-action-mount = スナップショットをマウント
mount-action-unmount = アンマウント
mount-status-mounted = { $path } にマウント済み
mount-error-unsafe-mountpoint = マウントポイントのパスが安全ではありません
mount-error-mountpoint-not-empty = マウントポイントは空のディレクトリである必要があります
mount-error-backend-unavailable = このシステムではマウントバックエンドを利用できません
mount-error-archive-read = アーカイブの読み取りに失敗しました
mount-picker-title = マウントポイントのディレクトリを選択
mount-toast-mounted = スナップショットを { $path } にマウントしました
mount-toast-unmounted = スナップショットをアンマウントしました
mount-toast-failed = マウントに失敗しました: { $reason }
settings-mount-heading = スナップショットのマウント
settings-mount-hint = 履歴アーカイブを読み取り専用ファイルシステムとして公開します。Phase 33b でランナーのフローを接続し、カーネルの FUSE/WinFsp バックエンドは Phase 33c で実装されます。
settings-mount-on-launch = 起動時に最新のスナップショットをマウント
settings-mount-on-launch-path = マウントポイントのパス
settings-mount-on-launch-path-placeholder = 例: C:\Mounts\copythat

# Phase 34 — enterprise-grade audit log export + WORM (write-once-read-
# many) tamper-resistance. Settings → Advanced → Audit log collects the
# format + destination + rotation + WORM toggle; runtime records job and
# file events into the tamper-evident chain-hashed sink.
settings-audit-heading = 監査ログ
settings-audit-hint = すべてのジョブとファイルイベントの追記専用で改ざん検知可能なログです。形式には CSV、JSON Lines、RFC 5424 Syslog、ArcSight CEF、QRadar LEEF があります。
settings-audit-enable = 監査ログを有効化
settings-audit-format = ログ形式
settings-audit-format-json-lines = JSON Lines（推奨の既定）
settings-audit-format-csv = CSV（スプレッドシート向け）
settings-audit-format-syslog = Syslog（RFC 5424）
settings-audit-format-cef = CEF（ArcSight）
settings-audit-format-leef = LEEF 2.0（IBM QRadar）
settings-audit-file-path = ログファイルのパス
settings-audit-file-path-placeholder = 例: C:\ProgramData\CopyThat\audit.log
settings-audit-max-size = この容量で切り替え（バイト、0 = なし）
settings-audit-worm = WORM モードを有効化（write-once-read-many）
settings-audit-worm-hint = 作成またはローテーションのたびに、プラットフォームの追記専用フラグ（Linux の chattr +a、macOS の chflags uappnd、Windows の読み取り専用属性）を適用します。管理者であっても、ログを切り詰めるにはフラグを明示的に解除する必要があります。
settings-audit-test-write = テスト書き込み
settings-audit-verify-chain = チェーンを検証
toast-audit-test-write-ok = 監査ログのテスト書き込みに成功しました
toast-audit-verify-ok = 監査チェーンが無傷であることを確認しました
toast-audit-verify-failed = 監査チェーンの検証で不一致が報告されました

# Phase 35 — destination encryption via age (X25519 / passphrase /
# SSH) + on-the-fly zstd compression with a per-extension deny
# heuristic. Surfaced through Settings → Transfer → Encryption +
# Compression; engine short-circuits to the crypt pipeline when
# either stage is active.
settings-crypt-heading = 暗号化と圧縮
settings-crypt-hint = ファイルの内容がコピー先に届く前に変換します。暗号化には age 形式を使用し、圧縮には zstd を使用して、拡張子により既に圧縮済みのメディアをスキップできます。
settings-crypt-encryption-mode = 暗号化
settings-crypt-encryption-off = オフ
settings-crypt-encryption-passphrase = パスフレーズ（コピー開始時に入力）
settings-crypt-encryption-recipients = ファイルから受信者鍵を読み込み
settings-crypt-encryption-hint = パスフレーズはコピーの間だけメモリに保持されます。受信者ファイルには 1 行に 1 つの age1… または ssh- 公開鍵を記述します。
settings-crypt-recipients-file = 受信者ファイルのパス
settings-crypt-recipients-file-placeholder = 例: C:\Users\me\recipients.txt
settings-crypt-compression-mode = 圧縮
settings-crypt-compression-off = オフ
settings-crypt-compression-always = 常に
settings-crypt-compression-smart = スマート（圧縮済みメディアをスキップ）
settings-crypt-compression-hint = スマートモードは jpg、mp4、zip、7z など zstd の恩恵を受けない形式をスキップします。常にモードはすべてのファイルを選択したレベルで圧縮します。
settings-crypt-compression-level = zstd レベル（1〜22）
settings-crypt-compression-level-hint = 数値が小さいほど高速で、大きいほど強く圧縮します。レベル 3 は zstd の CLI 既定値に一致します。
compress-footer-savings = 💾 { $original } → { $compressed }（{ $percent }% 節約）
compress-savings-toast = { $percent }% 圧縮しました（{ $bytes } 節約）
crypt-toast-recipients-loaded = { $count } 件の暗号化受信者を読み込みました
crypt-toast-recipients-error = 受信者の読み込みに失敗しました: { $reason }
crypt-toast-passphrase-required = コピー開始前に暗号化のパスフレーズが必要です
crypt-toast-passphrase-set = 暗号化パスフレーズを取得しました
crypt-footer-encrypted-badge = 🔒 暗号化済み（age）
crypt-footer-compressed-badge = 📦 圧縮済み（zstd）

# Phase 36 — copythat CLI surface. Documented exit codes + the
# user-facing strings for plan/apply/verify/config. Help text stays
# in English (engineering accessibility) but error / info / status
# strings are localized so a localized desktop installation surfaces
# CLI errors in the same language as the GUI.
cli-help-tagline = Copy That CLI — CI/CD パイプライン向けのバイト単位で正確なファイルコピー、同期、検証、監査。
cli-help-exit-codes = 終了コード: 0 成功、1 エラー、2 保留、3 競合、4 検証失敗、5 ネットワーク、6 権限、7 ディスク満杯、8 キャンセル、9 構成。
cli-error-bad-args = copy/move には少なくとも 1 つのソースとコピー先が必要です
cli-error-unknown-algo = 不明な検証アルゴリズム: { $algo }
cli-error-missing-spec = plan/apply には --spec が必要です
cli-error-spec-parse = jobspec { $path } の解析に失敗しました: { $reason }
cli-error-spec-empty-sources = jobspec のソース一覧が空です
cli-info-shape-recorded = 帯域シェイプ "{ $rate }" を記録しました。適用は copythat-shape を通じて行われます
cli-info-stub-deferred = { $command } は Phase 36 のフォローアップ接続に向けて準備中です
cli-plan-summary = プラン: { $actions } 件のアクション、{ $bytes } バイト。{ $already_done } は既に配置済み
cli-plan-pending = プランは保留中のアクションを報告しています。`apply` で再実行して実行してください
cli-plan-already-done = プランは実行すべき内容がないと報告しています（冪等）
cli-apply-success = apply はエラーなく完了しました
cli-apply-failed = apply は 1 つ以上のエラーで完了しました
cli-verify-ok = 検証 ok: { $algo } { $digest }
cli-verify-failed = { $path } の検証に失敗しました（{ $algo }）
cli-config-set = { $key } = { $value } を設定しました
cli-config-reset = { $key } を既定値にリセットしました
cli-config-unknown-key = 不明な構成キー: { $key }
cli-completions-emitted = { $shell } 用のシェル補完を stdout に出力しました

# Phase 37 — desktop-side mobile companion. Settings → Mobile panel
# strings + the SAS-confirmation modal + push-notification toasts.
# The actual mobile UI lives in the Phase 37 follow-up Tauri Mobile
# target; the phone displays its own localized SAS prompt.
settings-mobile-heading = モバイルコンパニオン
settings-mobile-hint = iPhone または Android スマートフォンをペアリングして、履歴の閲覧、保存済みプロファイルや Phase 36 の jobspec の実行、完了通知の受信ができます。
settings-mobile-pair-toggle = 新しいペアリングを許可
settings-mobile-pair-active = ペアサーバーが稼働中 — Copy That モバイルアプリで QR をスキャンしてください
settings-mobile-pair-button = ペアリングを開始
settings-mobile-revoke-button = 取り消し
settings-mobile-no-pairings = ペアリング済みのデバイスはまだありません
settings-mobile-pair-port = バインドポート（0 = 空きポートを選択）
pair-sas-prompt = 両方の画面に同じ 4 つの絵文字が表示されているはずです。一致していれば「一致」をタップしてください。
pair-sas-confirm = 一致
pair-sas-reject = 不一致 — キャンセル
pair-toast-success = { $device } とペアリングしました
pair-toast-failed = ペアリングに失敗しました: { $reason }
push-toast-sent = { $device } にプッシュを送信しました
push-toast-failed = { $device } へのプッシュに失敗しました: { $reason }

# Phase 38 — aggregate destination dedup + reflink fallback ladder.
# Settings → Transfer → Dedup panel + per-job-row badges (⚡
# Reflinked / 🔗 Hardlinked / 🧩 Chunk-shared / 📋 Copied) +
# pre-pass dedup-scan modal.
settings-dedup-heading = コピー先の重複排除
settings-dedup-hint = ソースとコピー先が同じボリュームを共有している場合、Copy That はバイトをコピーする代わりにファイルシステムレベルでファイルをクローンできます。Reflink は即時かつ安全です。ハードリンクはより高速ですが、両方の名前が状態を共有します。
settings-dedup-mode-auto = 自動ラダー（reflink → ハードリンク → チャンク → コピー）
settings-dedup-mode-reflink-only = Reflink のみ
settings-dedup-mode-hardlink-aggressive = アグレッシブ（書き込み可能なファイルでも reflink + ハードリンク）
settings-dedup-mode-off = 無効（常にバイトコピー）
settings-dedup-hardlink-policy = ハードリンクポリシー
settings-dedup-prescan = コピー先ツリーを事前スキャンして重複した内容を検出
dedup-badge-reflinked = ⚡ Reflink 済み
dedup-badge-hardlinked = 🔗 ハードリンク済み
dedup-badge-chunk-shared = 🧩 チャンク共有
dedup-badge-copied = 📋 コピー済み
phase42-paranoid-verify-label = パラノイド検証
phase42-paranoid-verify-hint = コピー先のキャッシュページを破棄してディスクから再読み取りし、書き込みキャッシュの偽りや暗黙の破損を検出します。既定の検証より約 50% 遅くなります。既定ではオフです。
phase42-sharing-violation-retries-label = ロックされたソースファイルへのリトライ回数
phase42-sharing-violation-retries-hint = 他のプロセスがソースファイルを排他ロックで開いている場合に何回リトライするかを指定します。バックオフは試行ごとに倍になります（既定で 50 ミリ秒 / 100 ミリ秒 / 200 ミリ秒）。既定値は 3 で、Robocopy /R:3 に一致します。
phase42-cloud-placeholder-warning = { $name } はクラウド専用の OneDrive ファイルです。コピーするとダウンロードが発生し、ネットワーク接続経由で最大 { $size } になります。
phase42-defender-exclusion-hint = コピーのスループットを最大化するには、一括転送の前にコピー先フォルダーを Microsoft Defender の除外に追加してください。docs/PERFORMANCE_TUNING.md を参照してください。

# Phase 39 — Browser-accessible recovery UI. Settings → Advanced
# exposes these strings; the recovery server itself renders askama
# templates that always read English — these eight keys are the
# Settings prose the user actually reads in their preferred locale.
settings-recovery-heading = リカバリ Web UI
settings-recovery-enable = リカバリ Web UI を有効化
settings-recovery-bind-address = バインドアドレス
settings-recovery-port = ポート（0 = 空きポートを選択）
settings-recovery-show-url = URL とトークンを表示
settings-recovery-rotate-token = トークンを更新
settings-recovery-allow-non-loopback = 非ループバックバインドを許可
settings-recovery-non-loopback-warning = 警告: 非ループバックバインドを有効にすると、リカバリ UI がローカルネットワークに公開されます。トークンを知る者は誰でもファイル履歴を閲覧してファイルをダウンロードできます。LAN が信頼できない場合は、TLS またはリバースプロキシを前段に配置してください。

# Phase 40 — SMB compression negotiation + cloud-VM offload helper.
# 6 SMB keys (header badge + Settings prose) + 6 cloud-offload keys
# (Remotes tab wizard for cross-cloud copy templates).
smb-compress-badge = 🗜 SMB 圧縮: { $algo }
smb-compress-badge-tooltip = このコピー先へのネットワークトラフィックは転送中に圧縮されています（SMB 3.1.1）。
smb-compress-toast-saved = ネットワーク経由で { $bytes } を節約しました
smb-compress-algo-unknown = 不明なアルゴリズム
settings-smb-compress-heading = SMB ネットワーク圧縮
settings-smb-compress-hint = UNC コピー先で SMB 3.1.1 のトラフィック圧縮を自動的にネゴシエートします。低速回線では効果があり、ローカルコピー先では無視されます。
cloud-offload-heading = クラウド VM オフロードヘルパー
cloud-offload-hint = 2 つのクラウド間で直接コピーする際に、クラウド内の小さな一時 VM からコピーを実行するデプロイテンプレートを生成します — バイトがラップトップのネットワークに触れることはありません。
cloud-offload-render-button = テンプレートを生成
cloud-offload-copy-clipboard = クリップボードにコピー
cloud-offload-template-format = テンプレート形式
cloud-offload-self-destruct-warning = VM は { $minutes } 分後に自動でシャットダウンします — デプロイ前に IAM ロールとリージョンを確認してください。

# Phase 41 — animated before/after tree-diff preview. The `Preview
# changes` modal renders the rolled-up plan before the engine starts
# work; 14 keys cover the title, the summary header / counts, the
# row-reason labels, and the two action buttons.
preview-modal-title = 変更をプレビュー
preview-summary-header = 何が起こるか
preview-category-additions = { $count } 件の追加
preview-category-replacements = { $count } 件の置換
preview-category-skips = { $count } 件のスキップ
preview-category-conflicts = { $count } 件の競合
preview-category-unchanged = { $count } 件の変更なし
preview-bytes-to-transfer = 転送する { $bytes }
preview-reason-source-newer = ソースの方が新しい
preview-reason-dest-newer = コピー先の方が新しい — スキップします
preview-reason-content-different = 内容が異なる
preview-reason-identical = ソースと同一
preview-button-run = プランを実行
preview-button-reduce = プランを減らす…

# Phase 42 — perceptual-hash visual-similarity dedup. Eight keys cover
# the pre-copy "looks visually identical" warning + the Settings panel.
perceptual-warn-title = 見た目が同一のようです
perceptual-warn-body = コピー先の { $name } はソースの画像と一致するようです。それでもコピーを続行しますか？
perceptual-warn-keep-both = 両方を保持
perceptual-warn-skip = このファイルをスキップ
perceptual-warn-overwrite = それでも上書き
perceptual-settings-heading = 視覚的類似性による重複排除
perceptual-settings-hint = コピー先の視覚的に同一な画像を上書き前に検出します。ハッシュは知覚的（同じ画像を別の形式で再保存しても認識）で、バイト単位の正確さではありません。
perceptual-settings-threshold-label = 警告のしきい値（低いほど厳密な一致）

# Phase 42 Part B — per-file rolling versions (Time Machine for any
# destination). 8 keys cover the version-list panel + retention picker.
version-list-heading = 以前のバージョン
version-list-empty = このファイルの以前のバージョンはありません
version-list-restore = このバージョンを復元
version-retention-heading = 上書き時に以前のバージョンを保持
version-retention-none = すべてのバージョンを無期限に保持
version-retention-last-n = 直近 { $n } 個のバージョンを保持
version-retention-older-than-days = { $days } 日より古いバージョンを破棄
version-retention-gfs = 時間別 { $h } · 日別 { $d } · 週別 { $w } · 月別 { $m }

# Phase 43 — forensic chain-of-custody manifests + BLAKE3 verified
# streaming. 14 keys cover the Settings → Provenance panel
# (heading, hint, toggles, TSA URL, signing-key management) plus
# the post-job manifest notification + the verify command's two
# headline result lines.
provenance-settings-heading = フォレンジック証拠保全
provenance-settings-hint = すべてのコピージョブに BLAKE3 + ed25519 のマニフェストで署名します。レビュー担当者は後でコピー先ツリーを再ハッシュし、コピー以降 1 バイトも変更されていないことを証明できます。
provenance-settings-enable-default = 既定ですべての新しいジョブに署名
provenance-settings-show-after-job = 完了したジョブごとにマニフェストを表示
provenance-settings-tsa-url-label = 既定の RFC 3161 タイムスタンプ局 URL
provenance-settings-tsa-url-hint = 任意。設定すると、マニフェストに無料の TSA タイムスタンプが付き、その時点でバイトが存在していたことを証明します。スキップする場合は空のままにしてください。
provenance-settings-keys-heading = 署名鍵
provenance-settings-keys-generate = 新しい鍵を生成
provenance-settings-keys-import = 鍵をインポート…
provenance-settings-keys-export = 公開鍵をエクスポート…
provenance-job-completed-title = 証拠保全マニフェストを保存しました
provenance-job-completed-body = { $count } 件のファイルに署名 → { $path }
provenance-verify-clean = { $count } 件のファイルでマニフェストが有効。署名 { $sig }、Merkle ルート OK。
provenance-verify-tampered = マニフェストが無効 — { $tampered } 件が改ざん、{ $missing } 件が欠落。
# Phase 43 post-review hardening — toast text for the Settings →
# Provenance buttons whose Tauri IPC has not yet landed.
provenance-action-staged = Phase 43 — このアクションの IPC 接続はフォローアップのコミットで実装されます。

# Phase 44 — SSD-aware whole-drive sanitize (NVMe Sanitize / OPAL
# Crypto Erase) + the per-file shred refusal on copy-on-write
# filesystems. 16 keys cover the new "Drive sanitize" Settings
# subsection and the localized error messages.
sanitize-heading = ドライブ全体の安全なサニタイズ
sanitize-hint = NVMe Sanitize、OPAL Crypto Erase、ATA Secure Erase は、フラッシュドライブをファームウェア層でミリ秒単位で消去します。ファイル単位の上書きはフラッシュでは無意味で、複数回のシュレッドは NAND を消耗させるだけです。実際のパージにはこちらを使用してください。
sanitize-pick-device = サニタイズするドライブを選択
sanitize-mode-label = サニタイズ方式
sanitize-mode-nvme-format = NVMe Format（Secure Erase 付き）
sanitize-mode-nvme-sanitize-block = NVMe Sanitize — Block Erase（低速、全セル）
sanitize-mode-nvme-sanitize-crypto = NVMe Sanitize — Crypto Erase（即時）
sanitize-mode-ata-secure-erase = ATA Secure Erase（従来の SATA SSD）
sanitize-mode-opal-crypto-erase = TCG OPAL Crypto Erase（自己暗号化ドライブ）
sanitize-mode-apfs-crypto-erase = APFS Crypto Erase（FileVault 鍵をローテーション、macOS のみ）
sanitize-confirm-1 = これは { $device } 上のすべてのバイトを破壊します。元に戻すことはできません。
sanitize-confirm-2 = { $device } 上のすべてのパーティション、すべてのファイル、すべてのスナップショットが永久に読み取り不能になることを理解しています。
sanitize-confirm-3 = 続行するにはドライブのモデル名を入力してください: { $model }
sanitize-running = { $device }（{ $mode }）をサニタイズ中 — これにはミリ秒（crypto erase）から数十分（block erase）かかることがあります。電源を切らないでください。
sanitize-completed = サニタイズ完了 — { $device } は空になりました。
ssd-honest-shred-meaningless = コピーオンライトファイルシステム（Btrfs / ZFS / APFS）上のファイル単位のシュレッドは、基盤となるブロックに届きません。代わりにドライブ全体のサニタイズと全ディスク暗号化の鍵ローテーションを使用してください。
ssd-honest-advisory = このファイルはフラッシュ上にあります。ファイル単位の上書きは NAND を消耗させ、元のセルが復元不能であることを保証しません。機密データには、ドライブ全体をサニタイズしてください。

# Phase 44.1f post-review — placeholder toast for SanitizeTab
# buttons whose Tauri IPC has not yet landed.
sanitize-action-staged = Phase 44.1 — このアクションの IPC 接続はフォローアップのコミットで実装されます。

# Phase 45.3 — named-queue tab strip (Subfeature A). Tabs surface
# once the QueueRegistry holds at least one queue; the synthesised
# default tab keeps legacy single-queue jobs reachable.
queue-tab-default = 既定
queue-tab-empty-state = ジョブキュー
queue-badge-tooltip = このキュー内の待機中および実行中のジョブ

# Phase 45.4 — drag-progress-merge (Subfeature B). Drag a queue tab
# onto another to merge their job lists. The default tab is neither
# draggable nor a drop target; only registry queues participate.
queue-drag-hint = 別のキューにドラッグして結合
queue-merge-confirm = ドロップして結合
queue-merge-toast = キューを結合しました

# Phase 45.5 — F2-queue UX (Subfeature C). F2 toggles
# `auto_enqueue_next` so every fresh enqueue piles into the running
# queue rather than spawning a parallel one. Status pill renders in
# the Footer; pulsing dot renders on the running tab.
queue-f2-active-hint = F2 モード: 新しいエンキューはすべてこのキューに入ります
queue-f2-toggled-on = F2 キューモード オン — 新しいエンキューは実行中のキューに加わります
queue-f2-toggled-off = F2 キューモード オフ — 新しいエンキューは並列キューを生成します
queue-f2-status-bar = F2 キューモード: オン

# Phase 45.6 — tray destination targets (Subfeature D). Pinned
# destinations appear in the OS tray menu; clicking one arms it as
# the active drop target so the next file drop bypasses the
# DropStagingDialog. Settings → General hosts the list editor.
tray-target-section-title = トレイのコピー先
tray-target-section-hint = ピン留めしたコピー先がトレイメニューに表示されます。クリックすると次のドロップ先として設定されます。
tray-target-empty = ピン留めされたトレイのコピー先はまだありません。
tray-target-remove = 削除
tray-target-add-label = ラベル
tray-target-add-path = パスまたはバックエンド URI
tray-target-add = 追加
tray-target-armed-toast = 次のファイルをドロップして { $label } に送信してください
tray-target-active-pill = → { $label }

# Phase 45.7 follow-up — pinned-destination validation errors. The
# `queue_pin_destination` IPC returns these Fluent keys verbatim
# when input fails the IPC-boundary checks; the toast layer renders
# them via `t(...)` (Toast.svelte detects kebab-case-lowercase and
# routes through the locale table). Phase 17e's `err-path-escape` /
# `err-destination-empty` cover the path-traversal + empty cases for
# `queue_route_job`; the keys below are pin-specific.
err-pinned-destination-label-empty = トレイのコピー先ラベルは空にできません。
err-pinned-destination-path-empty = トレイのコピー先パスは空にできません。
err-pinned-destination-label-too-long = トレイのコピー先ラベルが長すぎます（最大 64 文字）。
err-pinned-destination-path-too-long = トレイのコピー先パスが長すぎます（最大 1024 文字）。
err-pinned-destination-label-invalid = トレイのコピー先ラベルに使用できない文字（改行、復帰、または NUL）が含まれています。
err-pinned-destination-path-invalid = トレイのコピー先パスに使用できない文字（改行、復帰、または NUL）が含まれています。
err-pinned-destination-too-many = トレイのコピー先は上限の 50 件に達しました。追加するには 1 つ削除してください。

# Phase 46.6 — Settings → Plugins tab. The IPC layer in
# `apps/copythat-ui/src-tauri/src/plugin_commands.rs` enumerates the
# per-user plugin store under `<config_dir>/plugins/`; the
# `PluginsTab.svelte` component renders these strings against the
# returned manifest + grant state.
settings-tab-plugins = プラグイン
plugin-heading = プラグイン
plugin-hint = サンドボックス化された WASM プラグインがカスタムフックで Copy That を拡張します。各プラグインは呼び出しごとの CPU とメモリの制限下で動作し、付与したホスト機能のみを参照できます。
plugin-list-empty = インストール済みのプラグインはまだありません。
plugin-enabled = 有効
plugin-disabled = 無効
plugin-hooks = フック
plugin-capabilities = 機能
plugin-no-capabilities = （なし）
plugin-directory = 場所
plugin-install-from-file = ファイルからインストール…
plugin-install-from-url = URL からインストール…
plugin-url-wasm = WASM URL
plugin-url-manifest = マニフェスト URL
plugin-url-hash = BLAKE3 ハッシュ
plugin-url-preview = プレビュー
plugin-url-confirm = インストールを確定

# Phase 31b — power-policy settings (Power tab).
settings-tab-power = 電源
settings-power-hint = 電源状態に応じてコピーを制限/一時停止します：バッテリー、従量制/モバイル回線、プレゼン/全画面、または CPU のサーマルスロットリング時。
settings-power-enabled = 電源に応じた制限を有効化
settings-power-battery = バッテリー駆動時
settings-power-metered = 従量制ネットワーク時
settings-power-cellular = モバイル回線時
settings-power-presentation = プレゼン中
settings-power-fullscreen = 全画面時
settings-power-thermal = サーマルスロットリング時
settings-power-continue = 続行
settings-power-pause = 一時停止
err-server-not-implemented = サーバーモードはまだ利用できません。
err-webhook-not-implemented = Webhook の配信はまだ利用できません。

# Phase 47 — "why is this slow?" diagnostics (bottleneck badge + tooltip).
bottleneck-source-io = コピー元 I/O
bottleneck-dest-io = コピー先 I/O
bottleneck-network = ネットワーク
bottleneck-antivirus = ウイルス対策
bottleneck-cpu = CPU
bottleneck-thermal = サーマル
bottleneck-unknown = 不明
diag-aria = ボトルネック: { $cause }
diag-tooltip = { $cause } により制限 · { $rate }
diag-spark-aria = 直近1分間のスループット
diag-keeping-up = 順調
diag-label = 診断
