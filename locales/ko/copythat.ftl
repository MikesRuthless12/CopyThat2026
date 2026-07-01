app-name = Freally File Manager v0.19.85
window-title = Freally File Manager v0.19.85
shred-ssd-advisory = 경고: 이 대상은 SSD에 있습니다. 웨어 레벨링과 오버 프로비저닝으로 인해 데이터가 논리 블록 주소 밖으로 이동하기 때문에 다중 패스 덮어쓰기로는 플래시 메모리를 안정적으로 소거할 수 없습니다. 솔리드 스테이트 미디어에는 ATA SECURE ERASE, Secure Erase를 사용한 NVMe Format, 또는 키를 폐기하는 전체 디스크 암호화를 사용하세요.

# Global aggregate states (header pill)
state-idle = 대기 중
state-copying = 복사 중
state-verifying = 검증 중
state-paused = 일시 중지됨
state-error = 오류

# Per-job states (row badge)
state-pending = 대기열에 있음
state-running = 실행 중
state-cancelled = 취소됨
state-succeeded = 완료
state-failed = 실패

# Actions
action-pause = 일시 중지
action-resume = 다시 시작
action-cancel = 취소
action-pause-all = 모든 작업 일시 중지
action-resume-all = 모든 작업 다시 시작
action-cancel-all = 모든 작업 취소
action-close = 닫기
action-reveal = 폴더에서 표시
action-add-files = 파일 추가
action-add-folders = 폴더 추가

# Phase 13d — activity feed
activity-title = 활동
activity-clear = 활동 목록 지우기
activity-empty = 아직 파일 활동이 없습니다.
activity-after-done = 완료 시:
activity-keep-open = 앱 열어 두기
activity-close-app = 앱 닫기
activity-shutdown = PC 종료
activity-logoff = 로그오프
activity-sleep = 절전

# Phase 14 — preflight free-space dialog
preflight-block-title = 대상에 공간이 부족합니다
preflight-warn-title = 대상의 공간이 적습니다
preflight-unknown-title = 사용 가능한 공간을 확인할 수 없습니다
preflight-unknown-body = 원본이 너무 커서 빠르게 크기를 측정할 수 없거나 대상 볼륨이 응답하지 않았습니다. 계속 진행할 수 있으며, 공간이 부족해지면 엔진의 공간 보호 기능이 복사를 정상적으로 중단합니다.
preflight-required = 필요한 공간
preflight-free = 사용 가능
preflight-reserve = 예약
preflight-shortfall = 부족분
preflight-continue = 그래도 계속
preflight-pick-subset = 복사할 항목 선택…
collision-modal-overwrite-older = 오래된 것만 덮어쓰기

# Phase 14e — subset picker
subset-title = 복사할 원본 선택
subset-subtitle = 전체 선택 항목이 대상에 맞지 않습니다. 복사할 항목을 선택하세요. 나머지는 그대로 남습니다.
subset-loading = 크기 측정 중…
subset-too-large = 너무 커서 계산 불가
subset-budget = 사용 가능
subset-remaining = 남은 용량
subset-confirm = 선택 항목 복사
history-rerun-hint = 이 복사를 다시 실행합니다 — 원본 트리의 모든 파일을 다시 스캔합니다
history-clear-all = 모두 지우기
history-clear-all-confirm = 확인하려면 다시 클릭
history-clear-all-hint = 모든 기록 행을 삭제합니다. 확인을 위해 한 번 더 클릭해야 합니다.
toast-history-cleared = 기록을 지웠습니다 ({ $count }개 행 제거됨)

# Phase 15 — source-list ordering in the drop staging dialog
drop-dialog-sort-label = 정렬:
sort-custom = 사용자 지정
sort-name-asc = 이름 A → Z (파일 먼저)
sort-name-desc = 이름 Z → A (파일 먼저)
sort-size-asc = 크기 작은 것 먼저 (파일 먼저)
sort-size-desc = 크기 큰 것 먼저 (파일 먼저)
sort-reorder = 순서 변경
sort-move-top = 맨 위로 이동
sort-move-up = 위로 이동
sort-move-down = 아래로 이동
sort-move-bottom = 맨 아래로 이동

# Phase 16 — sort preset names for the Activity list (shorter
# wording than the DropStagingDialog variants; the Activity list
# is files-only so the "(files first)" suffix isn't relevant).
sort-name-asc-simple = 이름 A → Z
sort-name-desc-simple = 이름 Z → A
sort-size-asc-simple = 크기 작은 것 먼저
sort-size-desc-simple = 크기 큰 것 먼저
activity-sort-locked = 복사가 실행되는 동안에는 정렬을 사용할 수 없습니다. 일시 중지하거나 완료될 때까지 기다린 후 순서를 변경하세요.

# Phase 17 — collision-policy picker in the DropStagingDialog
drop-dialog-collision-label = 파일이 이미 있는 경우:
collision-policy-keep-both = 둘 다 유지 (새 복사본 이름을 _2, _3, …으로 변경)
collision-policy-skip = 새 복사본 건너뛰기
collision-policy-overwrite = 기존 파일 덮어쓰기
collision-policy-overwrite-if-newer = 더 새로운 경우에만 덮어쓰기
collision-policy-prompt = 매번 묻기

# Phase 18 — progress labels on the DropStagingDialog Start button
drop-dialog-busy-checking = 사용 가능한 공간 확인 중…
drop-dialog-busy-enumerating = 파일 수 계산 중…
drop-dialog-busy-starting = 복사 시작 중…
toast-enumeration-deferred = 원본 트리가 큽니다 — 사전 파일 목록 작성을 건너뜁니다. 엔진이 처리하는 대로 행이 표시됩니다.

# Context menu (per-row right-click)
menu-pause = 일시 중지
menu-resume = 다시 시작
menu-cancel = 취소
menu-remove = 대기열에서 제거
menu-reveal-source = 원본을 폴더에서 표시
menu-reveal-destination = 대상을 폴더에서 표시

# Header / toolbar
header-eta-label = 예상 남은 시간
header-toolbar-label = 전역 컨트롤

# Footer
footer-queued = 활성 작업
footer-total-bytes = 진행 중
footer-errors = 오류
footer-history = 기록

# Empty state
empty-title = 복사할 파일 또는 폴더를 끌어다 놓으세요
empty-hint = 항목을 창으로 끌어다 놓으세요. 대상을 물어본 다음 원본마다 작업을 하나씩 대기열에 추가합니다.
empty-region-label = 작업 목록

# Details drawer
details-drawer-label = 작업 세부 정보
details-source = 원본
details-destination = 대상
details-state = 상태
details-bytes = 바이트
details-files = 파일
details-speed = 속도
details-eta = 예상 시간
details-error = 오류

# Drop dialog
drop-dialog-title = 끌어다 놓은 항목 전송
drop-dialog-subtitle = { $count }개 항목을 전송할 준비가 되었습니다. 대상 폴더를 선택하여 시작하세요.
drop-dialog-mode = 작업
drop-dialog-copy = 복사
drop-dialog-move = 이동
drop-dialog-pick-destination = 대상 선택
drop-dialog-change-destination = 대상 변경
drop-dialog-start-copy = 복사 시작
drop-dialog-start-move = 이동 시작

# ETA placeholders
eta-calculating = 계산 중…
eta-unknown = 알 수 없음

# Toast messages
toast-job-done = 전송 완료
toast-copy-queued = 복사가 대기열에 추가됨
toast-move-queued = 이동이 대기열에 추가됨
toast-error-resolved = 오류 해결됨
toast-collision-resolved = 충돌 해결됨
toast-elevated-unavailable = 권한 상승 재시도는 Phase 17에 추가됩니다 — 아직 사용할 수 없습니다
toast-clipboard-files-detected = 클립보드에 파일이 있습니다 — 붙여넣기 단축키를 눌러 Freally File Manager으로 복사하세요
toast-clipboard-no-files = 클립보드에 붙여넣을 파일이 없습니다
toast-error-log-exported = 오류 로그를 내보냈습니다

# Error modal (Phase 8)
error-modal-title = 전송에 실패했습니다
error-modal-retry = 다시 시도
error-modal-retry-elevated = 상승된 권한으로 다시 시도
error-modal-skip = 건너뛰기
error-modal-skip-all-kind = 이 종류의 오류 모두 건너뛰기
error-modal-abort = 모두 중단
error-modal-path-label = 경로
error-modal-code-label = 코드
error-drawer-pending-count = 대기 중인 오류 더 있음
error-drawer-toggle = 접거나 펼치기

# Error-kind labels (Phase 8). Source of truth — engine maps each
# `CopyErrorKind` to one of these keys via `localized_key()`.
err-not-found = 파일을 찾을 수 없음
err-permission-denied = 권한이 거부됨
err-disk-full = 대상 디스크가 가득 참
err-interrupted = 작업이 중단됨
err-verify-failed = 복사 후 검증 실패
err-path-escape = 경로 거부됨 — 상위 디렉터리(..) 세그먼트 또는 잘못된 바이트가 포함됨
err-path-invalid-encoding = 경로 거부됨 — 문자열에 잘못된 UTF-8 / 대체 문자가 포함됨
err-helper-invalid-json = 권한 있는 도우미가 잘못된 형식의 JSON을 받았습니다. 이 요청을 무시합니다
err-helper-grant-out-of-band = GrantCapabilities는 무상태 핸들러가 아니라 도우미 실행 루프에서 처리해야 합니다
err-randomness-unavailable = OS 난수 생성기에 실패했습니다. 세션 ID를 생성할 수 없습니다
err-sparseness-mismatch = 대상에서 희소 레이아웃을 보존할 수 없습니다
err-io-other = 알 수 없는 I/O 오류

# Collision modal (Phase 8)
collision-modal-title = 파일이 이미 있습니다
collision-modal-overwrite = 덮어쓰기
collision-modal-overwrite-if-newer = 더 새로우면 덮어쓰기
collision-modal-skip = 건너뛰기
collision-modal-keep-both = 둘 다 유지
collision-modal-rename = 이름 변경…
collision-modal-apply-to-all = 모두 적용
collision-modal-source = 원본
collision-modal-destination = 대상
collision-modal-size = 크기
collision-modal-modified = 수정한 날짜
collision-modal-hash-check = 빠른 해시 (SHA-256)
collision-modal-hash-computing = 계산 중…
collision-modal-hash-identical = 동일
collision-modal-hash-different = 다름
collision-modal-rename-placeholder = 새 파일 이름
collision-modal-confirm-rename = 이름 변경

# Error log drawer (Phase 8)
error-log-title = 오류 로그
error-log-empty = 기록된 오류가 없습니다
error-log-export-csv = CSV 내보내기
error-log-export-txt = 텍스트 내보내기
error-log-clear = 로그 지우기
error-log-col-time = 시간
error-log-col-job = 작업
error-log-col-path = 경로
error-log-col-code = 코드
error-log-col-message = 메시지
error-log-col-resolution = 해결 방법

# History drawer (Phase 9)
history-title = 기록
history-empty = 아직 기록된 작업이 없습니다
history-unavailable = 복사 기록을 사용할 수 없습니다. 시작 시 앱이 SQLite 저장소를 열지 못했습니다.
history-filter-any = 전체
history-filter-kind = 종류
history-filter-status = 상태
history-filter-text = 검색
history-refresh = 새로 고침
history-export-csv = CSV 내보내기
history-purge-30 = 30일 초과 정리
history-rerun = 다시 실행
history-detail-open = 세부 정보
history-detail-title = 작업 세부 정보
history-detail-empty = 기록된 항목이 없습니다
history-col-date = 날짜
history-col-kind = 종류
history-col-src = 원본
history-col-dst = 대상
history-col-files = 파일
history-col-size = 크기
history-col-status = 상태
history-col-duration = 소요 시간
history-col-error = 오류
toast-history-exported = 기록을 내보냈습니다
toast-history-rerun-queued = 다시 실행이 대기열에 추가됨

# Totals drawer (Phase 10)
footer-totals = 합계
totals-title = 합계
totals-loading = 합계 불러오는 중…
totals-card-bytes = 복사한 총 바이트
totals-card-files = 파일
totals-card-jobs = 작업
totals-card-avg-rate = 평균 처리량
totals-errors = 오류
totals-spark-title = 최근 30일
totals-kinds-title = 종류별
totals-saved-title = 절약한 시간 (예상)
totals-saved-note = 동일한 작업량을 기준 파일 관리자로 복사한 경우와 비교한 예상치입니다.
totals-reset = 통계 초기화
totals-reset-confirm = 저장된 모든 작업과 항목이 삭제됩니다. 계속하시겠습니까?
totals-reset-confirm-yes = 예, 초기화합니다
toast-totals-reset = 통계 초기화됨

# Phase 11a — i18n core: surface remaining user-visible strings so
# every string on a main-window screen flows through Fluent.

# Header language switcher (temporary placement — folds into
# Settings → General in Phase 12).
header-language-label = 언어
header-language-title = 언어 변경

# Job-kind labels. History filter, Totals breakdown, and history
# rows all map wire-format `kind` strings to these labels.
kind-copy = 복사
kind-move = 이동
kind-delete = 삭제
kind-secure-delete = 보안 삭제

# History status labels. Distinct from `state-*` because the
# history wire format uses plain `running`/`succeeded`/... rather
# than the live `JobState` enum the queue exposes.
status-running = 실행 중
status-succeeded = 성공
status-failed = 실패
status-cancelled = 취소됨
# Per-item status (not job-level): `ok` / `skipped` are only
# reachable from the history detail view.
status-ok = 정상
status-skipped = 건너뜀

# History drawer: search field placeholder + purge toast.
history-search-placeholder = /path
toast-history-purged = 30일이 지난 작업 { $count }개를 정리했습니다

# User-facing command-layer validation errors. The Rust side
# returns these keys when input is missing; the toast layer looks
# them up rather than showing the raw English.
err-source-required = 원본 경로가 하나 이상 필요합니다.
err-destination-empty = 대상 경로가 비어 있습니다.
err-source-empty = 원본 경로가 비어 있습니다.

# Localised duration formatting for the Totals drawer and ETA
# fields. `{ $ms }`, `{ $s }`, `{ $m }`, `{ $h }` are integer
# placeables — the formatter passes pre-computed values in.
duration-lt-1s = < 1초
duration-ms = { $ms }밀리초
duration-seconds = { $s }초
duration-minutes-seconds = { $m }분 { $s }초
duration-hours-minutes = { $h }시간 { $m }분
duration-zero = 0초

# Rate unit. Appended to a formatted byte size. Some languages
# render this with a leading space ("Ko/s"); keep it translatable
# even though the SI-derived "/s" is near-universal.
rate-unit-per-second = { $size }/s

# Phase 11b — Settings modal skeleton. Phase 12 expanded this into a
# full six-tab preferences window; the `settings-phase-12-hint` key
# is retired in favour of `settings-tab-profiles` + concrete labels.
settings-title = 설정
settings-tab-general = 일반
settings-tab-appearance = 모양
settings-section-language = 언어
settings-phase-12-hint = 추가 설정(테마, 전송 기본값, 검증 알고리즘, 프로필)은 Phase 12에 추가됩니다.

# Phase 12 — full Settings window keys. Grouped by tab so the
# human-review pass can pick one section at a time. Every field
# label and every option label that the user will actually read
# has its own key; dynamic wire values (enum discriminants like
# "auto" / "prefer") stay untranslated on the wire.

settings-loading = 설정 불러오는 중…
settings-tab-transfer = 전송
settings-tab-filters = 필터
settings-tab-shell = 셸
settings-tab-secure-delete = 보안 삭제
settings-tab-advanced = 고급
settings-tab-updater = 업데이트
settings-tab-profiles = 프로필

# General tab additions
settings-section-theme = 테마
settings-theme-auto = 자동
settings-theme-light = 라이트
settings-theme-dark = 다크
settings-start-with-os = 시스템 시작 시 실행
settings-single-instance = 단일 실행 인스턴스
settings-minimize-to-tray = 닫을 때 트레이로 최소화
settings-error-display-mode = 오류 알림 스타일
settings-error-display-modal = 모달 (앱 차단)
settings-error-display-drawer = 서랍 (차단하지 않음)
settings-error-display-mode-hint = 모달은 결정할 때까지 대기열을 멈춥니다. 서랍은 대기열을 계속 진행하면서 화면 구석에서 오류를 처리할 수 있게 합니다.
settings-paste-shortcut = 전역 단축키로 파일 붙여넣기
settings-paste-shortcut-combo = 단축키 조합
settings-paste-shortcut-hint = 시스템 어디에서나 이 조합을 눌러 Explorer / Finder / Files에서 복사한 파일을 Freally File Manager으로 붙여넣으세요. CmdOrCtrl은 macOS에서는 Cmd로, Windows / Linux에서는 Ctrl로 처리됩니다.
settings-clipboard-watcher = 복사된 파일이 있는지 클립보드 감시
settings-clipboard-watcher-hint = 클립보드에 파일 URL이 나타나면 Freally File Manager으로 붙여넣을 수 있음을 알리는 알림을 표시합니다. 사용 설정 시 500밀리초마다 폴링합니다.

# Transfer tab
settings-buffer-size = 버퍼 크기
settings-verify = 복사 후 검증
settings-verify-off = 끄기
settings-concurrency = 동시 실행
settings-concurrency-auto = 자동
settings-reflink = Reflink / 빠른 경로
settings-reflink-prefer = 우선 사용
settings-reflink-avoid = reflink 피하기
settings-reflink-disabled = 항상 비동기 엔진 사용
settings-fsync-on-close = 닫을 때 디스크에 동기화 (느리지만 더 안전함)
settings-preserve-timestamps = 타임스탬프 보존
settings-preserve-permissions = 권한 보존
settings-preserve-acls = ACL 보존 (Phase 14)
settings-preserve-sparseness = 희소 파일 보존
settings-preserve-sparseness-hint = 희소 파일(VM 디스크, 데이터베이스 파일)의 할당된 익스텐트만 복사하여 대상의 디스크상 크기가 원본과 동일하게 유지되도록 합니다.
settings-force-parallel-chunks = 병렬 멀티 청크 복사 (RAID/어레이 전용)
settings-force-parallel-chunks-hint = 큰 복사를 동시 청크로 분할합니다. 스트라이프/RAID/네트워크 대상에서만 도움이 되며 단일 SSD/NVMe는 느려집니다(-25%~-76%). 대상이 다중 디스크 어레이가 아니면 꺼 두세요.

# Shell tab
settings-context-menu = 셸 컨텍스트 메뉴 항목 사용
settings-intercept-copy = 기본 복사 처리기 가로채기 (Windows)
settings-intercept-copy-hint = 켜면 Explorer의 Ctrl+C / Ctrl+V가 Freally File Manager을 통해 처리됩니다. 등록 기능은 Phase 14에 추가됩니다.
settings-notify-completion = 작업 완료 시 알림

# Secure delete tab
settings-shred-method = 기본 분쇄 방법
settings-shred-zero = 0으로 채우기 (1회 패스)
settings-shred-random = 무작위 (1회 패스)
settings-shred-dod3 = DoD 5220.22-M (3회 패스)
settings-shred-dod7 = DoD 5220.22-M (7회 패스)
settings-shred-gutmann = Gutmann (35회 패스)
settings-shred-nist = NIST 800-88
settings-shred-confirm-twice = 분쇄 전에 이중 확인 요구

# Advanced tab
settings-log-level = 로그 수준
settings-log-off = 끄기
settings-telemetry = 텔레메트리
settings-telemetry-never = 사용 안 함 — 어떤 로그 수준에서도 데이터를 전송하지 않음
settings-error-policy = 기본 오류 정책
settings-error-policy-ask = 묻기
settings-error-policy-skip = 건너뛰기
settings-error-policy-retry = 백오프하며 다시 시도
settings-error-policy-abort = 첫 실패 시 중단
settings-history-retention = 기록 보존 기간 (일)
settings-history-retention-hint = 0 = 영구 보존. 다른 값을 지정하면 시작 시 오래된 작업을 자동으로 정리합니다.
settings-database-path = 데이터베이스 경로
settings-database-path-default = (기본값 — OS 데이터 디렉터리)
settings-reset-all = 기본값으로 초기화
settings-reset-confirm = 모든 환경 설정을 기본값으로 초기화하시겠습니까? 프로필은 영향을 받지 않습니다.

# Profiles tab
settings-profiles-hint = 현재 설정을 이름을 지정해 저장하세요. 나중에 불러오면 개별 항목을 건드리지 않고 되돌릴 수 있습니다.
settings-profile-name-placeholder = 프로필 이름
settings-profile-save = 저장
settings-profile-import = 가져오기…
settings-profile-load = 불러오기
settings-profile-export = 내보내기…
settings-profile-delete = 삭제
settings-profile-empty = 아직 저장된 프로필이 없습니다.
settings-profile-import-prompt = 가져온 프로필의 이름:

# Toasts driven by Phase 12 profile actions
toast-settings-reset = 설정 초기화됨
toast-profile-saved = 프로필 저장됨
toast-profile-loaded = 프로필 불러옴
toast-profile-exported = 프로필 내보냄
toast-profile-imported = 프로필 가져옴

# Phase 14a — enumeration-time filters
settings-filters-hint = 열거 시점에 파일을 건너뛰어 엔진이 아예 열지 않도록 합니다. 포함 규칙은 파일에만 적용되고, 제외 규칙은 일치하는 디렉터리도 잘라냅니다.
settings-filters-enabled = 트리 복사에 필터 사용
settings-filters-include-globs = 포함 글로브
settings-filters-include-globs-placeholder = **/*.txt
settings-filters-include-globs-hint = 한 줄에 글로브 하나씩 입력하세요. 비어 있지 않으면 파일이 살아남으려면 하나 이상의 포함 규칙과 일치해야 합니다. 디렉터리에는 항상 들어갑니다.
settings-filters-exclude-globs = 제외 글로브
settings-filters-exclude-globs-placeholder = **/node_modules
settings-filters-exclude-globs-hint = 한 줄에 글로브 하나씩 입력하세요. 디렉터리는 일치하면 하위 트리 전체가 잘리고, 일치하는 파일은 건너뜁니다.
settings-filters-size-range = 파일 크기 범위
settings-filters-min-size-bytes = 최소 크기 (바이트, 비워두면 하한 없음)
settings-filters-max-size-bytes = 최대 크기 (바이트, 비워두면 상한 없음)
settings-filters-date-range = 수정 시간 범위
settings-filters-min-mtime = 수정 시점 이후
settings-filters-max-mtime = 수정 시점 이전
settings-filters-attributes = 속성 비트
settings-filters-skip-hidden = 숨김 파일 / 폴더 건너뛰기
settings-filters-skip-system = 시스템 파일 건너뛰기 (Windows 전용)
settings-filters-skip-readonly = 읽기 전용 파일 건너뛰기

# Phase 15 — auto-update
settings-updater-hint = Freally File Manager은 서명된 업데이트를 하루에 최대 한 번 확인합니다. 업데이트는 다음에 앱을 종료할 때 설치됩니다.
settings-updater-auto-check = 실행 시 업데이트 확인
settings-updater-channel = 릴리스 채널
settings-updater-channel-stable = 안정 버전
settings-updater-channel-beta = 베타 (사전 출시)
settings-updater-last-check = 마지막 확인
settings-updater-last-never = 안 함
settings-updater-check-now = 지금 업데이트 확인
settings-updater-checking = 확인 중…
settings-updater-available = 업데이트 사용 가능
settings-updater-up-to-date = 최신 릴리스를 사용 중입니다.
settings-updater-dismiss = 이 버전 건너뛰기
settings-updater-dismissed = 건너뜀
toast-update-available = 더 새로운 버전을 사용할 수 있습니다
toast-update-up-to-date = 이미 최신 버전을 사용 중입니다

# Phase 19a — disk-backed file enumeration (TeraCopy-compatible scan DB)
scan-progress-title = 스캔 중…
scan-progress-stats = { $files }개 파일 · 지금까지 { $bytes }
scan-pause-button = 스캔 일시 중지
scan-resume-button = 스캔 다시 시작
scan-cancel-button = 스캔 취소
scan-cancel-confirm = 스캔을 취소하고 진행 상황을 버리시겠습니까?
scan-db-header = 스캔 데이터베이스
scan-db-hint = 수백만 개 파일 작업을 위한 디스크 기반 스캔 데이터베이스입니다.
advanced-scan-hash-during = 스캔 중 체크섬 계산
advanced-scan-db-path = 스캔 데이터베이스 위치
advanced-scan-retention-days = 완료된 스캔 자동 삭제 기간 (일)
advanced-scan-max-keep = 보관할 최대 스캔 데이터베이스 수

# Phase 19b — filesystem-snapshot source for locked files.
settings-on-locked = 파일이 잠겨 있을 때
settings-on-locked-ask = 처음 한 번 묻기
settings-on-locked-retry = 잠시 다시 시도한 후 오류 표시
settings-on-locked-skip = 잠긴 파일 건너뛰기
settings-on-locked-snapshot = 파일 시스템 스냅샷 사용
settings-on-locked-hint = "다른 프로세스가 파일을 사용 중" 오류를 없앱니다. Freally File Manager이 원본 볼륨의 스냅샷(Windows에서는 VSS, Linux에서는 ZFS/Btrfs, macOS에서는 APFS)을 만들어 스냅샷 복사본에서 읽습니다.
snapshot-prompt-title = 이 파일은 다른 프로세스에서 사용 중입니다
snapshot-prompt-body = 다른 프로그램이 { $path }을(를) 독점 쓰기로 열어 두었습니다. 같은 볼륨의 이 파일 및 유사한 파일을 Freally File Manager이 어떻게 처리할지 선택하세요.
snapshot-source-active = 📷 { $volume }의 { $kind } 스냅샷에서 읽는 중
snapshot-create-failed = 원본 볼륨의 스냅샷을 만들 수 없습니다
snapshot-vss-needs-elevation = VSS 스냅샷에서 읽으려면 관리자 권한이 필요합니다. Freally File Manager이 허용을 요청합니다.
snapshot-cleanup-failed = 스냅샷 도우미가 정리 실패를 보고했습니다 — 볼륨에 남은 섀도 복사본이 남아 있을 수 있습니다.

# Phase 20 — durable resume journal.
resume-prompt-title = 이전 전송을 다시 시작하시겠습니까?
resume-prompt-body = Freally File Manager이 이전 세션에서 완료되지 않은 전송 { $count }개를 감지했습니다. 각각에 대해 어떻게 할지 선택하세요.
resume-prompt-resume = 다시 시작
resume-prompt-resume-all = 모두 다시 시작
resume-discard-one = 다시 시작 안 함
resume-discard-all = 모두 버리기
resume-aborted-hash-mismatch = 대상의 처음 { $offset }바이트가 원본과 일치하지 않습니다 — 처음부터 다시 시작합니다.
settings-auto-resume = 묻지 않고 중단된 작업 자동으로 다시 시작
settings-auto-resume-hint = 시작 시 다시 시작 프롬프트를 건너뛰고 완료되지 않은 모든 작업을 조용히 다시 대기열에 추가합니다. 기본적으로 꺼져 있습니다.

# Phase 21 — bandwidth shaping (GCRA token bucket + schedule + auto-throttle).
settings-tab-network = 네트워크
settings-network-hint = 전송 속도를 제한하여 나머지 네트워크를 계속 사용할 수 있게 하세요. 전역으로 적용하거나, 일일 일정을 따르거나, 종량제 Wi-Fi / 배터리 / 셀룰러 연결에 자동으로 반응하도록 할 수 있습니다.
settings-network-mode = 대역폭 제한
settings-network-mode-off = 끄기 (제한 없음)
settings-network-mode-fixed = 고정값
settings-network-mode-schedule = 일정 사용
settings-network-cap-mbps = 제한 (MB/s)
settings-network-schedule = 일정 (rclone 형식)
settings-network-schedule-hint = 공백으로 구분된 HH:MM,rate 경계와 선택적 Mon-Fri,rate 요일 규칙을 입력하세요. 속도: 512k, 10M, 2G, off, unlimited. 예: 08:00,512k 18:00,10M Sat-Sun,unlimited.
settings-network-auto-header = 자동 속도 제한
settings-network-auto-metered = 종량제 Wi-Fi에서
settings-network-auto-battery = 배터리에서
settings-network-auto-cellular = 셀룰러에서
settings-network-auto-unchanged = 재정의 안 함
settings-network-auto-pause = 전송 일시 중지
settings-network-auto-cap = 고정값으로 제한
shape-badge-paused = 일시 중지됨
shape-badge-tooltip = 대역폭 제한 활성 — 클릭하여 설정 → 네트워크 열기
shape-badge-source-schedule = 예약됨
shape-badge-source-metered = 종량제
shape-badge-source-battery = 배터리 사용 중
shape-badge-source-cellular = 셀룰러
shape-badge-source-settings = 활성
shape-error-schedule-invalid = 일정 형식이 유효하지 않습니다: { $message }

# Phase 22 — aggregate conflict dialog v2 (thumbnails, per-pattern
# rules, and reusable conflict profiles). Every key below is user-
# visible text in the `ConflictBatchModal.svelte` component.
conflict-batch-title = { $jobname }에 파일 충돌 { $count }개
conflict-batch-state-pending = 대기 중
conflict-batch-state-resolved = 해결됨
conflict-batch-action-overwrite = 덮어쓰기
conflict-batch-action-skip = 건너뛰기
conflict-batch-action-keep-both = 둘 다 유지
conflict-batch-action-newer-wins = 더 새로운 것 우선
conflict-batch-action-larger-wins = 더 큰 것 우선
conflict-batch-bulk-apply-selected = 선택 항목에 적용
conflict-batch-bulk-apply-extension = 이 확장자 전체에 적용
conflict-batch-bulk-apply-glob = 일치하는 글로브에 적용…
conflict-batch-bulk-apply-remaining = 남은 항목 전체에 적용
conflict-batch-bulk-glob-placeholder = 예: **/*.tmp
conflict-batch-save-profile = 이 규칙을 프로필로 저장…
conflict-batch-profile-placeholder = 프로필 이름
conflict-batch-matched-rule = '{ $rule }' 규칙으로 → { $action }
conflict-batch-empty = 모든 충돌이 해결됨
conflict-batch-source-vs-destination = 원본 vs. 대상
conflict-batch-source-label = 원본
conflict-batch-destination-label = 대상
conflict-batch-size-label = 크기
conflict-batch-modified-label = 수정한 날짜
conflict-batch-close = 닫기
conflict-batch-profile-saved = 충돌 프로필 저장됨

# Phase 23 — sparse-file preservation. The toast fires once per
# destination volume when the filesystem can't preserve holes; the
# warning line is surfaced in the job detail drawer so the user knows
# the dst is larger on disk than the source was.
sparse-not-supported-title = 대상이 희소 파일을 채웁니다
sparse-not-supported-body = { $dst_fs }은(는) 희소 파일을 지원하지 않습니다. 원본의 빈 공간이 0으로 기록되어 대상이 디스크상에서 더 큽니다.
sparse-warning-densified = 희소 레이아웃 보존됨: 할당된 익스텐트만 복사되었습니다.
sparse-warning-mismatch = 희소 레이아웃 불일치 — 대상이 예상보다 클 수 있습니다.

# Phase 24 — security-metadata preservation. The Mark-of-the-Web
# (Zone.Identifier ADS) toggle is security-sensitive: turning it off
# lets a downloaded executable shed its SmartScreen / Office Protected
# View flag on copy, which is why the tooltip carries an explicit
# warning. AppleDouble fallback emits `._<filename>` sidecars on
# destination filesystems that can't hold the foreign metadata.
settings-preserve-security-metadata = 보안 메타데이터 보존
settings-preserve-security-metadata-hint = 모든 복사 시 대역 외 메타데이터 스트림(NTFS ADS / xattr / POSIX ACL / SELinux 컨텍스트 / Linux 파일 기능 / macOS 리소스 포크)을 캡처하여 다시 적용합니다.
settings-preserve-motw = Mark-of-the-Web 보존 (인터넷에서 다운로드함 플래그)
settings-preserve-motw-hint = 보안에 중요합니다. SmartScreen과 Office Protected View는 이 스트림을 사용해 인터넷에서 다운로드한 파일에 대해 경고합니다. 비활성화하면 다운로드한 실행 파일이 복사 시 출처 표시를 잃고 운영 체제 보호 기능을 우회할 수 있습니다.
settings-preserve-posix-acls = POSIX ACL 및 확장 속성 보존
settings-preserve-posix-acls-hint = 복사 시 user.* / system.* / trusted.* xattr와 POSIX 액세스 제어 목록을 함께 전달합니다.
settings-preserve-selinux = SELinux 컨텍스트 보존
settings-preserve-selinux-hint = 복사 시 security.selinux 레이블을 함께 전달하여 MAC 정책에서 실행되는 데몬이 계속 파일에 액세스할 수 있도록 합니다.
settings-preserve-resource-forks = macOS 리소스 포크 및 Finder 정보 보존
settings-preserve-resource-forks-hint = 복사 시 레거시 리소스 포크와 FinderInfo(색상 태그, Carbon 메타데이터)를 함께 전달합니다.
settings-appledouble-fallback = 호환되지 않는 파일 시스템에서 AppleDouble 사이드카 사용
meta-translated-to-appledouble = 외부 메타데이터를 AppleDouble 사이드카에 저장함 (._{ $ext })

# Phase 25 — two-way sync with vector-clock conflict detection.
# The drawer lists configured sync pairs; each pair runs an
# independent reconciliation round against a per-pair `.copythat-sync.db`
# state store. Concurrent edits from a common ancestor surface as
# conflicts rather than silent overwrites; the losing side's content
# is preserved as `name.sync-conflict-YYYYMMDD-HHMMSS-<host>.ext`.
footer-sync = 동기화
sync-drawer-title = 양방향 동기화
sync-drawer-hint = 자동 덮어쓰기 없이 두 폴더를 동기화 상태로 유지하세요. 동시에 편집한 내용은 해결할 수 있는 충돌로 표시됩니다.
sync-add-pair = 쌍 추가
sync-add-cancel = 취소
sync-refresh = 새로 고침
sync-add-save = 쌍 저장
sync-add-saving = 저장 중…
sync-add-missing-fields = 레이블, 왼쪽 경로, 오른쪽 경로가 모두 필요합니다.
sync-remove-confirm = 이 동기화 쌍을 제거하시겠습니까? 상태 데이터베이스는 보존되며 폴더는 그대로 유지됩니다.
sync-field-label = 레이블
sync-field-label-placeholder = 예: 문서 ↔ NAS
sync-field-left = 왼쪽 폴더
sync-field-left-placeholder = 절대 경로를 선택하거나 붙여넣으세요
sync-field-right = 오른쪽 폴더
sync-field-right-placeholder = 절대 경로를 선택하거나 붙여넣으세요
sync-field-mode = 모드
sync-mode-two-way = 양방향
sync-mode-mirror-left-to-right = 미러 (왼쪽 → 오른쪽)
sync-mode-mirror-right-to-left = 미러 (오른쪽 → 왼쪽)
sync-mode-contribute-left-to-right = 기여 (왼쪽 → 오른쪽, 삭제 없음)
sync-no-pairs = 아직 구성된 동기화 쌍이 없습니다. "쌍 추가"를 클릭하여 시작하세요.
sync-loading = 구성된 쌍 불러오는 중…
sync-never-run = 실행한 적 없음
sync-running = 실행 중
sync-run-now = 지금 실행
sync-cancel = 취소
sync-remove-pair = 제거
sync-view-conflicts = 충돌 보기 ({ $count })
sync-conflicts-heading = 충돌
sync-no-conflicts = 마지막 실행에서 충돌이 없습니다.
sync-winner = 우선 항목
sync-side-left-to-right = 왼쪽
sync-side-right-to-left = 오른쪽
sync-conflict-kind-concurrent-write = 동시 편집
sync-conflict-kind-delete-edit = 삭제 ↔ 편집
sync-conflict-kind-add-add = 양쪽에서 추가됨
sync-conflict-kind-corrupt-equal = 새 쓰기 없이 콘텐츠가 갈라짐
sync-resolve-keep-left = 왼쪽 유지
sync-resolve-keep-right = 오른쪽 유지
sync-resolve-keep-both = 둘 다 유지
sync-resolve-three-way = 3방향 병합으로 해결
sync-resolve-phase-53-tooltip = 텍스트가 아닌 파일에 대한 대화형 3방향 병합은 Phase 53에 추가됩니다.
sync-error-prefix = 동기화 오류

# Phase 26 — real-time mirror watcher. "Live mirror" starts a
# filesystem watcher on the pair's left side; every debounced event
# triggers a re-sync. The watcher filters vim swap files / Office
# lock files / atomic-save temp names so one logical save becomes
# exactly one sync round.
live-mirror-start = 라이브 미러 시작
live-mirror-stop = 라이브 미러 중지
live-mirror-watching = 감시 중
live-mirror-toggle-hint = 감지된 모든 파일 시스템 변경 시 자동으로 다시 동기화합니다. 활성 쌍마다 백그라운드 스레드 하나가 실행됩니다.
watch-event-prefix = 파일 변경
watch-overflow-recovered = 감시 버퍼가 넘쳤습니다. 복구를 위해 다시 열거합니다

# Phase 27 — content-defined chunk store. Enables delta-resume (a
# retry only re-writes chunks that actually changed) and same-job
# dedup (files sharing content blocks store those blocks once). The
# store is disk-backed under `<data-dir>/chunks/` by default and is
# the foundation for the Phase 49–51 moonshot repository phases.
chunk-store-section = 청크 저장소
chunk-store-enable = 청크 저장소 사용 (델타 재개 및 중복 제거)
chunk-store-enable-hint = 복사하는 모든 파일을 콘텐츠(FastCDC)로 분할하여 콘텐츠 주소 방식으로 청크를 저장합니다. 다시 시도하면 변경된 청크만 다시 기록하고, 콘텐츠를 공유하는 파일은 자동으로 중복 제거됩니다.
chunk-store-location = 청크 저장소 위치
chunk-store-max-size = 최대 청크 저장소 크기
chunk-store-prune = 청크 정리 기간 (일)
chunk-store-savings = 청크 중복 제거로 { $gib } GiB 절약
chunk-store-disk-usage = 청크 { $chunks }개에 걸쳐 { $size } 사용 중

# Phase 28 — tray-resident Drop Stack. The stack is a persistent list
# of paths gathered from multiple sources (Explorer drag, main-window
# context menu, CLI, drag onto the Drop Stack window) that the user
# can dispatch to a destination in one go.
dropstack-window-title = Drop Stack
dropstack-tray-open = Drop Stack
dropstack-empty-title = Drop Stack이 비어 있습니다
dropstack-empty-hint = Explorer에서 파일을 여기로 끌어다 놓거나 작업 행을 마우스 오른쪽 버튼으로 클릭하여 추가하세요.
dropstack-add-to-stack = Drop Stack에 추가
dropstack-copy-all-to = 모두 복사할 위치…
dropstack-move-all-to = 모두 이동할 위치…
dropstack-clear = 스택 지우기
dropstack-remove-row = 스택에서 제거
dropstack-path-missing-toast = { $path }을(를) 끌어다 놓음 — 파일이 더 이상 존재하지 않습니다.
dropstack-always-on-top = Drop Stack을 항상 위에 유지
dropstack-show-tray-icon = Freally File Manager 트레이 아이콘 표시
dropstack-open-on-start = 앱 시작 시 Drop Stack 자동으로 열기
dropstack-count = { $count }개 경로

# Phase 29 — spring-loaded folders + native DnD polish. The Settings
# → General tab carries the knobs; DropTarget / DestinationPicker
# Svelte components use the dropzone-* keys at runtime.
settings-dnd-heading = 끌어서 놓기
settings-dnd-spring-load = 끄는 동안 폴더 자동 열기
settings-dnd-spring-delay = 자동 열기 지연 (밀리초)
settings-dnd-thumbnails = 끌기 썸네일 표시
settings-dnd-invalid-highlight = 잘못된 놓기 대상 강조 표시
dropzone-invalid-title = 유효한 놓기 대상이 아닙니다
dropzone-invalid-readonly = 대상이 읽기 전용입니다
dropzone-picker-title = 대상 선택
dropzone-picker-up = 위로
dropzone-picker-path = 현재 경로
dropzone-picker-root = 루트
dropzone-picker-use-this = 이 폴더 사용
dropzone-picker-empty = 하위 폴더 없음
dropzone-picker-cancel = 취소

# Phase 30 — cross-platform path translation. Settings → Transfer
# exposes these under a "Cross-platform compatibility" subsection.
translate-heading = 크로스 플랫폼 호환성
translate-unicode-label = 유니코드 정규화
translate-unicode-auto = 대상 자동 감지
translate-unicode-windows = NFC (Windows / Linux)
translate-unicode-macos = 그대로 두기 (macOS / APFS)
translate-line-endings-label = 텍스트 파일의 줄 끝 변환
translate-line-endings-allowlist = 텍스트 파일 확장자
reserved-name-label = Windows 예약 이름 처리
reserved-name-suffix = "_" 추가 (CON.txt → CON_.txt)
reserved-name-reject = 거부하고 경고
long-path-label = 260자를 초과하면 Windows 긴 경로 접두사(\\?\) 사용
long-path-hint = 일부 네트워크 공유와 레거시 도구는 \\?\ 네임스페이스를 따르지 않습니다.

# Phase 31 — power-aware copying. Settings → Power & State tab + the
# header badge that renders "⏸ Paused — Zoom call detected" when the
# runner's power subscriber has paused or capped due to a policy match.
power-heading = 전원 및 상태
power-enabled = 전원 인식 규칙 사용
power-battery-label = 배터리에서
power-metered-label = 종량제 Wi-Fi에서
power-cellular-label = 셀룰러에서
power-presentation-label = 발표 중일 때 (Zoom / Teams / Keynote)
power-fullscreen-label = 앱이 전체 화면일 때
power-thermal-label = CPU가 열로 인해 제한될 때
power-rule-continue = 최고 속도로 계속
power-rule-pause = 모든 작업 일시 중지
power-rule-cap = 대역폭 제한
power-rule-cap-percent = 현재 속도의 백분율로 제한
power-reason-on-battery = 배터리 사용 중
power-reason-metered-network = 종량제 네트워크
power-reason-cellular-network = 셀룰러 네트워크
power-reason-presenting = 발표 모드
power-reason-fullscreen = 전체 화면 앱
power-reason-thermal-throttling = CPU 제한 중

# Phase 32 — cloud backend matrix via OpenDAL. Settings → Remotes
# tab + the Add-backend wizard that writes one entry per remote into
# the keychain-backed credential store.
remote-heading = 원격 백엔드
remote-add = 백엔드 추가
remote-list-empty = 구성된 원격 백엔드가 없습니다
remote-test = 연결 테스트
remote-test-success = 연결 성공
remote-test-failed = 연결 실패
remote-remove = 백엔드 제거
remote-name-label = 표시 이름
remote-kind-label = 백엔드 유형
remote-save = 백엔드 저장
remote-cancel = 취소
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
backend-local-fs = 로컬 파일 시스템
cloud-config-bucket = 버킷
cloud-config-region = 리전
cloud-config-endpoint = 엔드포인트 URL
cloud-config-root = 루트 경로
cloud-error-invalid-config = 백엔드 구성이 올바르지 않습니다
cloud-error-network = 백엔드에 연결하는 중 네트워크 오류 발생
cloud-error-not-found = 요청한 경로에서 객체를 찾을 수 없습니다
cloud-error-permission = 원격 백엔드에서 권한이 거부되었습니다
cloud-error-keychain = OS 키체인 액세스에 실패했습니다
settings-tab-remotes = 원격
settings-tab-mobile = 모바일

# Phase 33 — mount Freally File Manager's chunk store + history archive as a
# read-only filesystem (FUSE / WinFsp). Surfaced by the History tab
# context menu's "Mount snapshot" action + the Settings → Advanced
# "Mount latest on launch" toggle.
mount-heading = 스냅샷 마운트
mount-action-mount = 스냅샷 마운트
mount-action-unmount = 마운트 해제
mount-status-mounted = { $path }에 마운트됨
mount-error-unsafe-mountpoint = 마운트 지점 경로가 안전하지 않습니다
mount-error-mountpoint-not-empty = 마운트 지점은 빈 디렉터리여야 합니다
mount-error-backend-unavailable = 이 시스템에서 마운트 백엔드를 사용할 수 없습니다
mount-error-archive-read = 아카이브 읽기에 실패했습니다
mount-picker-title = 마운트 지점 디렉터리 선택
mount-toast-mounted = { $path }에 스냅샷이 마운트됨
mount-toast-unmounted = 스냅샷 마운트 해제됨
mount-toast-failed = 마운트 실패: { $reason }
settings-mount-heading = 스냅샷 마운트
settings-mount-hint = 기록 아카이브를 읽기 전용 파일 시스템으로 노출합니다. Phase 33b에서 실행 흐름을 연결하고, 커널 FUSE/WinFsp 백엔드는 Phase 33c에 추가됩니다.
settings-mount-on-launch = 실행 시 최신 스냅샷 마운트
settings-mount-on-launch-path = 마운트 지점 경로
settings-mount-on-launch-path-placeholder = 예: C:\Mounts\copythat

# Phase 34 — enterprise-grade audit log export + WORM (write-once-read-
# many) tamper-resistance. Settings → Advanced → Audit log collects the
# format + destination + rotation + WORM toggle; runtime records job and
# file events into the tamper-evident chain-hashed sink.
settings-audit-heading = 감사 로그
settings-audit-hint = 모든 작업 및 파일 이벤트에 대한 추가 전용 변조 감지 로그입니다. 형식에는 CSV, JSON-lines, RFC 5424 Syslog, ArcSight CEF, QRadar LEEF가 포함됩니다.
settings-audit-enable = 감사 로깅 사용
settings-audit-format = 로그 형식
settings-audit-format-json-lines = JSON lines (권장 기본값)
settings-audit-format-csv = CSV (스프레드시트 친화적)
settings-audit-format-syslog = Syslog (RFC 5424)
settings-audit-format-cef = CEF (ArcSight)
settings-audit-format-leef = LEEF 2.0 (IBM QRadar)
settings-audit-file-path = 로그 파일 경로
settings-audit-file-path-placeholder = 예: C:\ProgramData\CopyThat\audit.log
settings-audit-max-size = 회전 기준 크기 (바이트, 0 = 안 함)
settings-audit-worm = WORM 모드 사용 (write-once-read-many)
settings-audit-worm-hint = 생성 또는 회전할 때마다 플랫폼의 추가 전용 플래그(Linux chattr +a, macOS chflags uappnd, Windows 읽기 전용 속성)를 적용합니다. 관리자라도 로그를 잘라내려면 플래그를 명시적으로 해제해야 합니다.
settings-audit-test-write = 테스트 쓰기
settings-audit-verify-chain = 체인 검증
toast-audit-test-write-ok = 감사 로그 테스트 쓰기 성공
toast-audit-verify-ok = 감사 체인이 손상 없이 검증됨
toast-audit-verify-failed = 감사 체인 검증에서 불일치가 보고됨

# Phase 35 — destination encryption via age (X25519 / passphrase /
# SSH) + on-the-fly zstd compression with a per-extension deny
# heuristic. Surfaced through Settings → Transfer → Encryption +
# Compression; engine short-circuits to the crypt pipeline when
# either stage is active.
settings-crypt-heading = 암호화 및 압축
settings-crypt-hint = 파일 콘텐츠를 대상에 도달하기 전에 변환합니다. 암호화는 age 형식을 사용하고, 압축은 zstd를 사용하며 확장자별로 이미 압축된 미디어를 건너뛸 수 있습니다.
settings-crypt-encryption-mode = 암호화
settings-crypt-encryption-off = 끄기
settings-crypt-encryption-passphrase = 암호 (복사 시작 시 입력)
settings-crypt-encryption-recipients = 파일에서 수신자 키 사용
settings-crypt-encryption-hint = 암호는 복사가 진행되는 동안에만 메모리에 보관됩니다. 수신자 파일에는 한 줄에 age1… 또는 ssh- 공개 키를 하나씩 나열합니다.
settings-crypt-recipients-file = 수신자 파일 경로
settings-crypt-recipients-file-placeholder = 예: C:\Users\me\recipients.txt
settings-crypt-compression-mode = 압축
settings-crypt-compression-off = 끄기
settings-crypt-compression-always = 항상
settings-crypt-compression-smart = 스마트 (이미 압축된 미디어 건너뛰기)
settings-crypt-compression-hint = 스마트 모드는 zstd로 이점이 없는 jpg, mp4, zip, 7z 및 유사한 형식을 건너뜁니다. 항상 모드는 선택한 수준으로 모든 파일을 압축합니다.
settings-crypt-compression-level = zstd 수준 (1-22)
settings-crypt-compression-level-hint = 숫자가 낮을수록 빠르고, 높을수록 더 강하게 압축합니다. 수준 3은 zstd의 CLI 기본값과 같습니다.
compress-footer-savings = 💾 { $original } → { $compressed } ({ $percent }% 절약)
compress-savings-toast = { $percent }% 압축 ({ $bytes } 절약)
crypt-toast-recipients-loaded = 암호화 수신자 { $count }명을 불러옴
crypt-toast-recipients-error = 수신자를 불러오지 못함: { $reason }
crypt-toast-passphrase-required = 복사를 시작하기 전에 암호화에 암호가 필요합니다
crypt-toast-passphrase-set = 암호화 암호가 입력됨
crypt-footer-encrypted-badge = 🔒 암호화됨 (age)
crypt-footer-compressed-badge = 📦 압축됨 (zstd)

# Phase 36 — copythat CLI surface. Documented exit codes + the
# user-facing strings for plan/apply/verify/config. Help text stays
# in English (engineering accessibility) but error / info / status
# strings are localized so a localized desktop installation surfaces
# CLI errors in the same language as the GUI.
cli-help-tagline = Freally File Manager CLI — CI/CD 파이프라인을 위한 바이트 단위 정확한 파일 복사, 동기화, 검증 및 감사.
cli-help-exit-codes = 종료 코드: 0 성공, 1 오류, 2 대기, 3 충돌, 4 검증 실패, 5 네트워크, 6 권한, 7 디스크 가득 참, 8 취소, 9 구성.
cli-error-bad-args = copy/move에는 원본이 하나 이상과 대상이 필요합니다
cli-error-unknown-algo = 알 수 없는 검증 알고리즘: { $algo }
cli-error-missing-spec = plan/apply에는 --spec이 필요합니다
cli-error-spec-parse = jobspec { $path } 구문 분석 실패: { $reason }
cli-error-spec-empty-sources = Jobspec 원본 목록이 비어 있습니다
cli-info-shape-recorded = 대역폭 셰이프 "{ $rate }" 기록됨; 적용은 copythat-shape을 통해 연결됩니다
cli-info-stub-deferred = { $command }은(는) Phase 36 후속 연결 작업으로 준비되어 있습니다
cli-plan-summary = 계획: 작업 { $actions }개, { $bytes }바이트; { $already_done }개는 이미 처리됨
cli-plan-pending = 계획에 대기 중인 작업이 보고됨; `apply`로 다시 실행하여 실행하세요
cli-plan-already-done = 계획에 할 작업이 없음 (멱등성)
cli-apply-success = 적용이 오류 없이 완료됨
cli-apply-failed = 적용이 하나 이상의 오류와 함께 완료됨
cli-verify-ok = 검증 정상: { $algo } { $digest }
cli-verify-failed = { $path } 검증 실패 ({ $algo })
cli-config-set = { $key } = { $value } 설정함
cli-config-reset = { $key }을(를) 기본값으로 초기화함
cli-config-unknown-key = 알 수 없는 구성 키: { $key }
cli-completions-emitted = { $shell }의 셸 자동 완성이 stdout으로 출력됨

# Phase 37 — desktop-side mobile companion. Settings → Mobile panel
# strings + the SAS-confirmation modal + push-notification toasts.
# The actual mobile UI lives in the Phase 37 follow-up Tauri Mobile
# target; the phone displays its own localized SAS prompt.
settings-mobile-heading = 모바일 컴패니언
settings-mobile-hint = iPhone 또는 Android 휴대폰을 페어링하여 기록을 탐색하고, 저장된 프로필과 Phase 36 jobspec을 시작하고, 완료 알림을 받으세요.
settings-mobile-pair-toggle = 새 페어링 허용
settings-mobile-pair-active = 페어링 서버 활성 — Freally File Manager 모바일 앱으로 QR을 스캔하세요
settings-mobile-pair-button = 페어링 시작
settings-mobile-revoke-button = 취소
settings-mobile-no-pairings = 아직 페어링된 기기가 없습니다
settings-mobile-pair-port = 바인딩 포트 (0 = 빈 포트 선택)
pair-sas-prompt = 두 화면에 같은 네 개의 이모지가 표시되어야 합니다. 일치하면 일치를 누르세요.
pair-sas-confirm = 일치
pair-sas-reject = 불일치 — 취소
pair-toast-success = { $device }과(와) 페어링됨
pair-toast-failed = 페어링 실패: { $reason }
push-toast-sent = { $device }에 푸시 보냄
push-toast-failed = { $device }에 푸시 실패: { $reason }

# Phase 38 — aggregate destination dedup + reflink fallback ladder.
# Settings → Transfer → Dedup panel + per-job-row badges (⚡
# Reflinked / 🔗 Hardlinked / 🧩 Chunk-shared / 📋 Copied) +
# pre-pass dedup-scan modal.
settings-dedup-heading = 대상 중복 제거
settings-dedup-hint = 원본과 대상이 볼륨을 공유하면 Freally File Manager은 바이트를 복사하는 대신 파일 시스템 수준에서 파일을 복제할 수 있습니다. Reflink는 즉각적이고 안전하며, 하드링크는 더 빠르지만 두 이름이 상태를 공유합니다.
settings-dedup-mode-auto = 자동 단계 (reflink → 하드링크 → 청크 → 복사)
settings-dedup-mode-reflink-only = reflink만
settings-dedup-mode-hardlink-aggressive = 적극적 (쓰기 가능한 파일에서도 reflink + 하드링크)
settings-dedup-mode-off = 비활성화 (항상 바이트 복사)
settings-dedup-hardlink-policy = 하드링크 정책
settings-dedup-prescan = 중복 콘텐츠에 대해 대상 트리 사전 스캔
dedup-badge-reflinked = ⚡ Reflink됨
dedup-badge-hardlinked = 🔗 하드링크됨
dedup-badge-chunk-shared = 🧩 청크 공유됨
dedup-badge-copied = 📋 복사됨
phase42-paranoid-verify-label = 편집증적 검증
phase42-paranoid-verify-hint = 대상의 캐시된 페이지를 버리고 디스크에서 다시 읽어 쓰기 캐시의 거짓과 무음 손상을 잡아냅니다. 기본 검증보다 약 50% 느리며 기본적으로 꺼져 있습니다.
phase42-sharing-violation-retries-label = 잠긴 원본 파일에 대한 재시도 횟수
phase42-sharing-violation-retries-hint = 다른 프로세스가 원본 파일을 독점 잠금으로 열고 있을 때 다시 시도할 횟수입니다. 백오프는 시도할 때마다 두 배가 됩니다(기본값 50밀리초 / 100밀리초 / 200밀리초). 기본값 3으로, Robocopy /R:3과 동일합니다.
phase42-cloud-placeholder-warning = { $name }은(는) 클라우드 전용 OneDrive 파일입니다. 복사하면 다운로드가 시작되어 네트워크 연결을 통해 최대 { $size }이(가) 전송됩니다.
phase42-defender-exclusion-hint = 복사 처리량을 최대화하려면 대량 전송 전에 대상 폴더를 Microsoft Defender 제외 항목에 추가하세요. docs/PERFORMANCE_TUNING.md를 참조하세요.

# Phase 39 — Browser-accessible recovery UI. Settings → Advanced
# exposes these strings; the recovery server itself renders askama
# templates that always read English — these eight keys are the
# Settings prose the user actually reads in their preferred locale.
settings-recovery-heading = 복구 웹 UI
settings-recovery-enable = 복구 웹 UI 사용
settings-recovery-bind-address = 바인딩 주소
settings-recovery-port = 포트 (0 = 빈 포트 선택)
settings-recovery-show-url = URL 및 토큰 표시
settings-recovery-rotate-token = 토큰 교체
settings-recovery-allow-non-loopback = 루프백이 아닌 바인딩 허용
settings-recovery-non-loopback-warning = 경고: 루프백이 아닌 바인딩을 사용하면 복구 UI가 로컬 네트워크에 노출됩니다. 토큰을 알게 된 누구나 파일 기록을 탐색하고 파일을 다운로드할 수 있습니다. LAN을 신뢰할 수 없는 경우 TLS나 리버스 프록시로 앞단을 보호하세요.

# Phase 40 — SMB compression negotiation + cloud-VM offload helper.
# 6 SMB keys (header badge + Settings prose) + 6 cloud-offload keys
# (Remotes tab wizard for cross-cloud copy templates).
smb-compress-badge = 🗜 SMB 압축: { $algo }
smb-compress-badge-tooltip = 이 대상으로의 네트워크 트래픽이 전송 중 압축되고 있습니다 (SMB 3.1.1).
smb-compress-toast-saved = 네트워크에서 { $bytes } 절약
smb-compress-algo-unknown = 알 수 없는 알고리즘
settings-smb-compress-heading = SMB 네트워크 압축
settings-smb-compress-hint = UNC 대상에서 SMB 3.1.1 트래픽 압축을 자동으로 협상합니다. 느린 링크에서는 무료 이득이며, 로컬 대상에서는 무시됩니다.
cloud-offload-heading = 클라우드 VM 오프로드 도우미
cloud-offload-hint = 두 클라우드 사이를 직접 복사할 때, 클라우드의 작은 임시 VM에서 복사를 실행하는 배포 템플릿을 생성합니다 — 바이트가 노트북 네트워크를 거치지 않습니다.
cloud-offload-render-button = 템플릿 생성
cloud-offload-copy-clipboard = 클립보드에 복사
cloud-offload-template-format = 템플릿 형식
cloud-offload-self-destruct-warning = VM은 { $minutes }분 후 자동으로 종료됩니다 — 배포 전에 IAM 역할 + 리전을 확인하세요.

# Phase 41 — animated before/after tree-diff preview. The `Preview
# changes` modal renders the rolled-up plan before the engine starts
# work; 14 keys cover the title, the summary header / counts, the
# row-reason labels, and the two action buttons.
preview-modal-title = 변경 사항 미리 보기
preview-summary-header = 무엇이 일어날지
preview-category-additions = 추가 { $count }개
preview-category-replacements = 교체 { $count }개
preview-category-skips = 건너뜀 { $count }개
preview-category-conflicts = 충돌 { $count }개
preview-category-unchanged = 변경 없음 { $count }개
preview-bytes-to-transfer = 전송할 { $bytes }
preview-reason-source-newer = 원본이 더 새로움
preview-reason-dest-newer = 대상이 더 새로움 — 건너뜁니다
preview-reason-content-different = 콘텐츠가 다름
preview-reason-identical = 원본과 동일
preview-button-run = 계획 실행
preview-button-reduce = 계획 줄이기…

# Phase 42 — perceptual-hash visual-similarity dedup. Eight keys cover
# the pre-copy "looks visually identical" warning + the Settings panel.
perceptual-warn-title = 시각적으로 동일해 보임
perceptual-warn-body = 대상의 { $name }이(가) 원본 사진과 일치하는 것으로 보입니다. 그래도 계속 복사하시겠습니까?
perceptual-warn-keep-both = 둘 다 유지
perceptual-warn-skip = 이 파일 건너뛰기
perceptual-warn-overwrite = 그래도 덮어쓰기
perceptual-settings-heading = 시각적 유사성 중복 제거
perceptual-settings-hint = 대상의 이미지가 덮어쓰이기 전에 시각적으로 동일한 이미지를 감지합니다. 해시는 바이트 단위가 아니라 지각적(같은 사진을 다른 형식으로 다시 저장한 경우도 인식)입니다.
perceptual-settings-threshold-label = 경고 임곗값 (낮을수록 엄격한 일치)

# Phase 42 Part B — per-file rolling versions (Time Machine for any
# destination). 8 keys cover the version-list panel + retention picker.
version-list-heading = 이전 버전
version-list-empty = 이 파일의 이전 버전이 없습니다
version-list-restore = 이 버전 복원
version-retention-heading = 덮어쓸 때 이전 버전 유지
version-retention-none = 모든 버전 영구 유지
version-retention-last-n = 최근 { $n }개 버전 유지
version-retention-older-than-days = { $days }일이 지난 버전 삭제
version-retention-gfs = 매시간 { $h }개 · 매일 { $d }개 · 매주 { $w }개 · 매월 { $m }개

# Phase 43 — forensic chain-of-custody manifests + BLAKE3 verified
# streaming. 14 keys cover the Settings → Provenance panel
# (heading, hint, toggles, TSA URL, signing-key management) plus
# the post-job manifest notification + the verify command's two
# headline result lines.
provenance-settings-heading = 포렌식 보관 연속성
provenance-settings-hint = 모든 복사 작업에 BLAKE3 + ed25519 매니페스트로 서명합니다. 검토자는 나중에 대상 트리를 다시 해시하여 복사 이후 단 한 바이트도 변경되지 않았음을 증명할 수 있습니다.
provenance-settings-enable-default = 기본적으로 모든 새 작업에 서명
provenance-settings-show-after-job = 완료된 작업마다 매니페스트 표시
provenance-settings-tsa-url-label = 기본 RFC 3161 타임스탬프 기관 URL
provenance-settings-tsa-url-hint = 선택 사항입니다. 설정하면 매니페스트에 해당 시점에 바이트가 존재했음을 증명하는 무료 TSA 타임스탬프가 포함됩니다. 건너뛰려면 비워 두세요.
provenance-settings-keys-heading = 서명 키
provenance-settings-keys-generate = 새 키 생성
provenance-settings-keys-import = 키 가져오기…
provenance-settings-keys-export = 공개 키 내보내기…
provenance-job-completed-title = 출처 매니페스트 저장됨
provenance-job-completed-body = 파일 { $count }개 서명됨 → { $path }
provenance-verify-clean = 파일 { $count }개에 대한 매니페스트 유효; 서명 { $sig }; 머클 루트 정상.
provenance-verify-tampered = 매니페스트 무효 — 변조 { $tampered }개, 누락 { $missing }개.
# Phase 43 post-review hardening — toast text for the Settings →
# Provenance buttons whose Tauri IPC has not yet landed.
provenance-action-staged = Phase 43 — 이 작업에 대한 IPC 연결은 후속 커밋에 추가됩니다.

# Phase 44 — SSD-aware whole-drive sanitize (NVMe Sanitize / OPAL
# Crypto Erase) + the per-file shred refusal on copy-on-write
# filesystems. 16 keys cover the new "Drive sanitize" Settings
# subsection and the localized error messages.
sanitize-heading = 전체 드라이브 보안 소거
sanitize-hint = NVMe Sanitize, OPAL Crypto Erase, ATA Secure Erase는 펌웨어 계층에서 플래시 드라이브를 밀리초 안에 지웁니다. 파일별 덮어쓰기는 플래시에서 무의미합니다 — 다중 패스 분쇄는 NAND만 소모할 뿐입니다. 실제 소거에는 이것을 사용하세요.
sanitize-pick-device = 소거할 드라이브 선택
sanitize-mode-label = 소거 방법
sanitize-mode-nvme-format = NVMe Format (보안 소거 포함)
sanitize-mode-nvme-sanitize-block = NVMe Sanitize — Block Erase (느림, 모든 셀)
sanitize-mode-nvme-sanitize-crypto = NVMe Sanitize — Crypto Erase (즉시)
sanitize-mode-ata-secure-erase = ATA Secure Erase (레거시 SATA SSD)
sanitize-mode-opal-crypto-erase = TCG OPAL Crypto Erase (자체 암호화 드라이브)
sanitize-mode-apfs-crypto-erase = APFS Crypto Erase (FileVault 키 교체, macOS 전용)
sanitize-confirm-1 = 이 작업은 { $device }의 모든 바이트를 파괴합니다. 되돌릴 수 없습니다.
sanitize-confirm-2 = { $device }의 모든 파티션, 모든 파일, 모든 스냅샷이 영구적으로 읽을 수 없게 됨을 이해합니다.
sanitize-confirm-3 = 진행하려면 드라이브의 모델 이름을 입력하세요: { $model }
sanitize-running = { $device } 소거 중 ({ $mode }) — 밀리초(크립토 소거)에서 수십 분(블록 소거)까지 걸릴 수 있습니다. 전원을 끄지 마세요.
sanitize-completed = 소거 완료 — { $device }이(가) 이제 비어 있습니다.
ssd-honest-shred-meaningless = 쓰기 시 복사 파일 시스템(Btrfs / ZFS / APFS)에서의 파일별 분쇄는 기본 블록에 도달할 수 없습니다. 대신 전체 드라이브 소거와 전체 디스크 암호화 키 교체를 사용하세요.
ssd-honest-advisory = 이 파일은 플래시에 있습니다. 파일별 덮어쓰기는 NAND 마모를 유발하며 원본 셀을 복구할 수 없음을 보장하지 않습니다. 민감한 데이터의 경우 전체 드라이브를 소거하세요.

# Phase 44.1f post-review — placeholder toast for SanitizeTab
# buttons whose Tauri IPC has not yet landed.
sanitize-action-staged = Phase 44.1 — 이 작업에 대한 IPC 연결은 후속 커밋에 추가됩니다.

# Phase 45.3 — named-queue tab strip (Subfeature A). Tabs surface
# once the QueueRegistry holds at least one queue; the synthesised
# default tab keeps legacy single-queue jobs reachable.
queue-tab-default = 기본
queue-tab-empty-state = 작업 대기열
queue-badge-tooltip = 이 대기열의 대기 중 및 실행 중인 작업

# Phase 45.4 — drag-progress-merge (Subfeature B). Drag a queue tab
# onto another to merge their job lists. The default tab is neither
# draggable nor a drop target; only registry queues participate.
queue-drag-hint = 다른 대기열로 끌어다 놓아 병합
queue-merge-confirm = 놓아서 병합
queue-merge-toast = 대기열 병합됨

# Phase 45.5 — F2-queue UX (Subfeature C). F2 toggles
# `auto_enqueue_next` so every fresh enqueue piles into the running
# queue rather than spawning a parallel one. Status pill renders in
# the Footer; pulsing dot renders on the running tab.
queue-f2-active-hint = F2 모드: 모든 새 대기 추가가 이 대기열에 들어갑니다
queue-f2-toggled-on = F2 대기열 모드 켜짐 — 새 대기 추가가 실행 중인 대기열에 합류합니다
queue-f2-toggled-off = F2 대기열 모드 꺼짐 — 새 대기 추가가 병렬 대기열을 생성합니다
queue-f2-status-bar = F2 대기열 모드: 켜짐

# Phase 45.6 — tray destination targets (Subfeature D). Pinned
# destinations appear in the OS tray menu; clicking one arms it as
# the active drop target so the next file drop bypasses the
# DropStagingDialog. Settings → General hosts the list editor.
tray-target-section-title = 트레이 대상
tray-target-section-hint = 고정된 대상이 트레이 메뉴에 나타납니다. 하나를 클릭하면 다음 놓기 대상으로 설정됩니다.
tray-target-empty = 아직 고정된 트레이 대상이 없습니다.
tray-target-remove = 제거
tray-target-add-label = 레이블
tray-target-add-path = 경로 또는 백엔드 URI
tray-target-add = 추가
tray-target-armed-toast = 다음 파일을 놓아 { $label }(으)로 보내세요
tray-target-active-pill = → { $label }

# Phase 45.7 follow-up — pinned-destination validation errors. The
# `queue_pin_destination` IPC returns these Fluent keys verbatim
# when input fails the IPC-boundary checks; the toast layer renders
# them via `t(...)` (Toast.svelte detects kebab-case-lowercase and
# routes through the locale table). Phase 17e's `err-path-escape` /
# `err-destination-empty` cover the path-traversal + empty cases for
# `queue_route_job`; the keys below are pin-specific.
err-pinned-destination-label-empty = 트레이 대상 레이블은 비워 둘 수 없습니다.
err-pinned-destination-path-empty = 트레이 대상 경로는 비워 둘 수 없습니다.
err-pinned-destination-label-too-long = 트레이 대상 레이블이 너무 깁니다 (최대 64자).
err-pinned-destination-path-too-long = 트레이 대상 경로가 너무 깁니다 (최대 1024자).
err-pinned-destination-label-invalid = 트레이 대상 레이블에 허용되지 않는 문자(줄 바꿈, 캐리지 리턴, NUL)가 포함되어 있습니다.
err-pinned-destination-path-invalid = 트레이 대상 경로에 허용되지 않는 문자(줄 바꿈, 캐리지 리턴, NUL)가 포함되어 있습니다.
err-pinned-destination-too-many = 트레이 대상 50개 한도에 도달했습니다. 하나를 제거하면 다른 것을 추가할 수 있습니다.

# 라이브러리 드로어(단계 49) — 콘텐츠 주소 지정 저장소 통합 보기.
footer-library = 라이브러리
library-title = 라이브러리
library-loading = 저장소 불러오는 중…
library-unavailable = 저장소를 사용할 수 없습니다
library-tab-live = 현재
library-tab-snapshots = 스냅샷
library-tab-versions = 버전
library-hero-savings = 실효 { $effective } 제공 · { $pct } 절약
library-hero-empty = 청크 { $chunks } 개 저장됨 — 아직 스냅샷 없음
library-stat-stored = 디스크 사용량
library-stat-effective = 실효 데이터
library-stat-snapshots = 스냅샷
library-stat-chunks = 고유 청크
library-snapshot-empty = 아직 스냅샷이 없습니다
library-snapshot-files = { $n } 개 파일
library-version-path-ph = 대상 경로…
library-version-load = 버전 표시
library-version-empty = 이 경로에 대한 버전이 없습니다
repo-kind-copy = 복사
repo-kind-sync = 동기화
repo-kind-version = 버전 관리
repo-kind-backup = 백업
# Phase 49o — snapshot diff / compare.
library-tab-compare = 비교
repo-change-added = 추가됨
repo-change-removed = 제거됨
repo-change-modified = 수정됨
repo-change-unchanged = 변경 없음
repo-diff-summary = { $added }개 추가 · { $removed }개 제거 · { $modified }개 수정
repo-diff-bytes-added = 신규 { $bytes }
repo-diff-pick-two = 비교할 스냅샷 두 개 선택
# Phase 49r — statistics / reports.
library-tab-reports = 보고서
report-growth-title = 저장소 증가
report-by-kind-title = 종류별
report-top-files-title = 상위 파일
report-dedup-ratio = { $pct }% 중복 제거됨
report-export = 보고서 내보내기
report-exported = 보고서를 { $path }에 저장함
report-file-versions = { $n }개 버전
# Phase 49p — pinning / prune.
repo-pin = 고정
repo-unpin = 고정 해제
repo-pinned-badge = 고정됨
repo-prune-title = 정리
repo-prune-keep-last = 최신 항목 유지
repo-prune-removed = 스냅샷 { $n }개 정리됨
repo-prune-none = 정리할 항목 없음

# Phase 46.6 — Settings → Plugins tab. The IPC layer in
# `apps/copythat-ui/src-tauri/src/plugin_commands.rs` enumerates the
# per-user plugin store under `<config_dir>/plugins/`; the
# `PluginsTab.svelte` component renders these strings against the
# returned manifest + grant state.
settings-tab-plugins = 플러그인
plugin-heading = 플러그인
plugin-hint = 샌드박스 처리된 WASM 플러그인이 사용자 지정 후크로 Freally File Manager을 확장합니다. 각 플러그인은 호출별 CPU 및 메모리 제한에서 실행되며 사용자가 부여한 호스트 기능만 볼 수 있습니다.
plugin-list-empty = 아직 설치된 플러그인이 없습니다.
plugin-enabled = 사용
plugin-disabled = 사용 안 함
plugin-hooks = 후크
plugin-capabilities = 기능
plugin-no-capabilities = (없음)
plugin-directory = 위치
plugin-install-from-file = 파일에서 설치…
plugin-install-from-url = URL에서 설치…
plugin-url-wasm = WASM URL
plugin-url-manifest = 매니페스트 URL
plugin-url-hash = BLAKE3 해시
plugin-url-preview = 미리 보기
plugin-url-confirm = 설치 확인

# Phase 31b — power-policy settings (Power tab).
settings-tab-power = 전원
settings-power-hint = 전원 상태에 따라 복사를 제한하거나 일시중지합니다: 배터리, 종량제/셀룰러 네트워크, 프레젠테이션/전체 화면 또는 CPU 열 스로틀링.
settings-power-enabled = 전원 기반 제한 사용
settings-power-battery = 배터리 사용 시
settings-power-metered = 종량제 네트워크에서
settings-power-cellular = 셀룰러에서
settings-power-presentation = 프레젠테이션 중
settings-power-fullscreen = 전체 화면일 때
settings-power-thermal = 열 스로틀링 시
settings-power-continue = 계속
settings-power-pause = 일시중지
err-server-not-implemented = 서버 모드는 아직 사용할 수 없습니다.
err-webhook-not-implemented = 웹훅 전송은 아직 사용할 수 없습니다.

# Phase 47 — "why is this slow?" diagnostics (bottleneck badge + tooltip).
bottleneck-source-io = 원본 I/O
bottleneck-dest-io = 대상 I/O
bottleneck-network = 네트워크
bottleneck-antivirus = 백신
bottleneck-cpu = CPU
bottleneck-thermal = 발열
bottleneck-unknown = 알 수 없음
diag-aria = 병목: { $cause }
diag-tooltip = { $cause }에 의해 제한 · { $rate }
diag-spark-aria = 지난 1분간 처리량
diag-keeping-up = 정상 속도
diag-label = 진단

# Phase 48 — server mode + observability (Settings → Server).
settings-tab-server = 서버
server-hint = Freally File Manager를 헤드리스 파일 서버로 실행합니다. 노출할 프로토콜을 선택하고 주소와 제공할 폴더를 설정하며 선택적으로 인증을 요구합니다.
server-protocols = 프로토콜
server-bind-addr = 바인드 주소
server-root = 제공 폴더
server-readonly = 읽기 전용(업로드 및 삭제 거부)
server-auth-mode = 인증
server-auth-none = 없음
server-auth-bearer = 베어러 토큰
server-auth-basic = 기본(사용자 이름 + 비밀번호)
server-auth-token = 토큰
server-auth-user = 사용자 이름
server-auth-password = 비밀번호
otel-endpoint = OpenTelemetry 엔드포인트
webhook-section = Webhook
webhook-url = Webhook URL
webhook-add = Webhook 추가
webhook-remove = 제거
webhook-empty = 구성된 webhook이 없습니다.
webhook-pushover-token = Pushover 토큰
webhook-pushover-user = Pushover 사용자
server-start = 서버 시작
server-stop = 서버 중지
server-status-running = { $addr }에서 실행 중
server-status-stopped = 중지됨
server-metrics-url = 메트릭
err-server-no-protocols = 서버를 시작하기 전에 프로토콜을 하나 이상 선택하세요.
err-server-bind = 서버 주소를 바인드할 수 없습니다. 이미 사용 중일 수 있습니다.

# Phase 49c — 백업 소스.
library-tab-sources = 소스
backup-add-source = 소스 추가…
backup-source-path-ph = 백업할 폴더…
backup-exclude-ph = 제외 글로브(쉼표로 구분)
backup-now = 지금 백업
backup-remove = 제거
backup-empty = 아직 백업 소스가 없습니다
backup-never-run = 백업한 적 없음
backup-last-run = 마지막 백업 { $when }
backup-running = 백업 중… { $files }개 파일
backup-toast-started = { $label } 백업 중…
backup-toast-completed = { $label } 백업 완료: { $files }개 파일
backup-toast-failed = { $label } 백업 실패: { $reason }
# Phase 49e — per-source retention + prune.
backup-retention = 보존
backup-retention-keep-all = 모두 보관
backup-retention-last = 최근 { $n }개 보관
backup-retention-days = { $days }일보다 오래됨
backup-retention-gfs = GFS 순환
backup-prune-now = 지금 정리
backup-prune-none = 정리할 항목 없음
backup-prune-result = { $removed }개 스냅샷 제거됨 · { $bytes } 확보
# Phase 49f — per-source scheduling.
backup-schedule = 예약
backup-schedule-manual = 수동
backup-schedule-hourly = 매시간
backup-schedule-daily = 매일
backup-schedule-weekly = 매주
backup-next-run = 다음 실행 { $when }
backup-not-scheduled = 예약 안 됨
# Phase 49g — source filters.
backup-include-ph = 포함 글로브 (쉼표로 구분)
backup-skip-hidden = 숨김 건너뛰기
# Phase 49q — notifications.
notify-title = 알림
notify-on-success = 성공 시
notify-on-failure = 실패 시
notify-test = 테스트 전송
notify-test-sent = { $n }개 대상으로 테스트를 보냈습니다

# Phase 49d — 복원 브라우저.
restore-browse = 복원…
restore-title = 스냅샷에서 복원
restore-select-all = 모두 선택
restore-dest = 복원 위치
restore-confirm = { $n }개 파일 복원
restore-empty = 이 스냅샷에 파일이 없습니다
restore-conflict-body = 선택한 { $count }개 파일이 대상에 이미 있습니다.
restore-conflict-overwrite = 덮어쓰기
restore-conflict-skip = 기존 건너뛰기
restore-conflict-keep-both = 둘 다 유지
restore-toast-done = { $restored }개 복원, { $skipped }개 건너뜀
restore-toast-failed = 복원 실패: { $reason }
snapshot-forget = 잊기
snapshot-forget-toast = 스냅샷을 잊었습니다 — “공간 회수”를 실행하여 비우세요
library-reclaim = 공간 회수
# Phase 49i — full compaction.
library-compact = 전체 컴팩션
library-compact-started = 컴팩션을 시작했습니다 — 작업을 확인하세요
# Phase 49h — compression.
library-stat-compression = 압축 절약량
storage-compression = 압축
storage-compression-off = 끄기
storage-compression-auto = 자동 (압축 불가 데이터 건너뛰기)
storage-compression-always = 항상
storage-compression-restart = 다음 실행 시 적용
# Phase 49j — tasks & progress center.
footer-tasks = 작업
tasks-title = 작업
tasks-empty = 아직 작업이 없습니다
tasks-running = 실행 중
tasks-recent = 최근
tasks-cancel = 취소
task-state-running = 실행 중
task-state-completed = 완료
task-state-failed = 실패
task-state-cancelled = 취소됨
# Phase 49k — repository setup/connect wizard.
repo-wizard-title = 저장소 연결
repo-wizard-create-tab = 새로 만들기
repo-wizard-connect-tab = 기존 연결
repo-field-name = 이름
repo-field-path = 위치
repo-field-password = 암호 (선택 사항)
repo-action-create = 만들기
repo-action-connect = 연결
repo-action-browse = 찾아보기…
repo-switcher-label = 저장소
repo-action-forget = 연결 해제
repo-action-change-pass = 암호 변경…
repo-password-old = 현재 암호
repo-password-new = 새 암호
repo-error-exists = 이 위치에 이미 저장소가 있습니다
repo-error-not-found = 이 위치에서 저장소를 찾을 수 없습니다
repo-error-bad-pass = 암호가 올바르지 않습니다
repo-note-no-encryption = 암호는 접근만 제어합니다. 저장 데이터 암호화는 향후 릴리스에서 제공됩니다
repo-confirm-forget = "{ $name }"을(를) 목록에서 제거하시겠습니까? 데이터는 디스크에 그대로 남아 있습니다.
repo-toast-created = 저장소 "{ $name }" 생성됨
repo-toast-connected = "{ $name }"에 연결됨
repo-toast-pass-changed = 암호가 업데이트됨
# Phase 49l — Sources dashboard.
library-tab-overview = 개요
library-source-empty = 아직 원본이 없습니다
library-source-unknown = (지정되지 않은 원본)
library-source-snapshots = 스냅샷 { $n }개
library-source-latest = 최신 { $when }
# Phase 49n — verify & repair.
repo-action-verify = 검증
repo-action-verify-deep = 검증(모든 데이터 읽기)
repo-action-repair = 복구…
repo-verify-clean = { $files } 파일 / { $chunks } 청크 검증 — 손상 없음
repo-verify-damaged = 누락 { $missing }개, 손상된 청크 { $corrupt }개
repo-repair-confirm = 더 이상 복원할 수 없는 스냅샷 { $n }개를 제거할까요?
repo-repair-removed = 손상된 스냅샷 { $n }개를 제거했습니다
repo-repair-none = 복구할 항목이 없습니다 — 저장소가 정상입니다
repo-gc-done = { $bytes } 회수됨 ({ $chunks } 청크)
restore-toast-partial = { $restored }개 복원, { $skipped }개 건너뜀, { $failed }개 실패
