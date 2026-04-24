app-name = Copy That v1.25.0
# MT
window-title = Copy That v1.25.0
# MT
shred-ssd-advisory = 경고: 이 대상은 SSD에 있습니다. 웨어 레벨링과 오버프로비저닝이 논리 블록 주소 아래의 데이터를 이동시키므로, 여러 번의 덮어쓰기로는 플래시 메모리를 안정적으로 말소할 수 없습니다. 솔리드 스테이트 매체에서는 ATA SECURE ERASE, NVMe Format(보안 삭제 포함), 또는 키를 파기한 전체 디스크 암호화를 우선적으로 사용하십시오.

# MT
state-idle = 대기 중
# MT
state-copying = 복사 중
# MT
state-verifying = 검증 중
# MT
state-paused = 일시 중지됨
# MT
state-error = 오류

# MT
state-pending = 대기열
# MT
state-running = 실행 중
# MT
state-cancelled = 취소됨
# MT
state-succeeded = 완료
# MT
state-failed = 실패

# MT
action-pause = 일시 중지
# MT
action-resume = 재개
# MT
action-cancel = 취소
# MT
action-pause-all = 모든 작업 일시 중지
# MT
action-resume-all = 모든 작업 재개
# MT
action-cancel-all = 모든 작업 취소
# MT
action-close = 닫기
# MT
action-reveal = 폴더에서 보기

# MT
menu-pause = 일시 중지
# MT
menu-resume = 재개
# MT
menu-cancel = 취소
# MT
menu-remove = 대기열에서 제거
# MT
menu-reveal-source = 원본을 폴더에서 보기
# MT
menu-reveal-destination = 대상을 폴더에서 보기

# MT
header-eta-label = 예상 남은 시간
# MT
header-toolbar-label = 전역 컨트롤

# MT
footer-queued = 개의 활성 작업
# MT
footer-total-bytes = 진행 중
# MT
footer-errors = 개의 오류
# MT
footer-history = 기록

# MT
empty-title = 복사할 파일이나 폴더를 놓으세요
# MT
empty-hint = 창에 항목을 끌어다 놓으세요. 대상 폴더를 물어본 뒤 원본마다 하나의 작업을 대기열에 추가합니다.
# MT
empty-region-label = 작업 목록

# MT
details-drawer-label = 작업 세부 정보
# MT
details-source = 원본
# MT
details-destination = 대상
# MT
details-state = 상태
# MT
details-bytes = 바이트
# MT
details-files = 파일
# MT
details-speed = 속도
# MT
details-eta = 남은 시간
# MT
details-error = 오류

# MT
drop-dialog-title = 드롭한 항목 전송
# MT
drop-dialog-subtitle = 전송할 준비가 된 항목 { $count }개. 시작하려면 대상 폴더를 선택하세요.
# MT
drop-dialog-mode = 작업
# MT
drop-dialog-copy = 복사
# MT
drop-dialog-move = 이동
# MT
drop-dialog-pick-destination = 대상 선택
# MT
drop-dialog-change-destination = 대상 변경
# MT
drop-dialog-start-copy = 복사 시작
# MT
drop-dialog-start-move = 이동 시작

# MT
eta-calculating = 계산 중…
# MT
eta-unknown = 알 수 없음

# MT
toast-job-done = 전송 완료
# MT
toast-copy-queued = 복사가 대기열에 추가됨
# MT
toast-move-queued = 이동이 대기열에 추가됨
# MT — Phase 8 toast messages
toast-error-resolved = 오류가 해결되었습니다
# MT
toast-collision-resolved = 충돌이 해결되었습니다
# MT
toast-elevated-unavailable = 상승 권한으로 재시도 기능은 17단계에서 제공됩니다 — 아직 사용할 수 없음
toast-clipboard-files-detected = 클립보드에 파일 있음 — 붙여넣기 단축키를 눌러 Copy That으로 복사
toast-clipboard-no-files = 클립보드에 붙여넣을 파일이 없음
# MT
toast-error-log-exported = 오류 로그를 내보냈습니다

# MT — Error modal
error-modal-title = 전송이 실패했습니다
# MT
error-modal-retry = 다시 시도
# MT
error-modal-retry-elevated = 상승 권한으로 다시 시도
# MT
error-modal-skip = 건너뛰기
# MT
error-modal-skip-all-kind = 이 종류의 오류를 모두 건너뛰기
# MT
error-modal-abort = 모두 중단
# MT
error-modal-path-label = 경로
# MT
error-modal-code-label = 코드
error-drawer-pending-count = 대기 중인 추가 오류
error-drawer-toggle = 접기 또는 펼치기

# MT — Error-kind labels
err-not-found = 파일을 찾을 수 없습니다
# MT
err-permission-denied = 권한이 거부되었습니다
# MT
err-disk-full = 대상 디스크가 가득 찼습니다
# MT
err-interrupted = 작업이 중단되었습니다
# MT
err-verify-failed = 복사 후 확인 실패
# MT
err-path-escape = 경로 거부됨 — 상위 디렉터리(..) 세그먼트 또는 잘못된 바이트 포함
# MT
err-io-other = 알 수 없는 I/O 오류
err-sparseness-mismatch = 대상에서 스파스 레이아웃을 보존할 수 없습니다  # MT

# MT — Collision modal
collision-modal-title = 파일이 이미 있습니다
# MT
collision-modal-overwrite = 덮어쓰기
# MT
collision-modal-overwrite-if-newer = 더 새 파일이면 덮어쓰기
# MT
collision-modal-skip = 건너뛰기
# MT
collision-modal-keep-both = 둘 다 유지
# MT
collision-modal-rename = 이름 바꾸기…
# MT
collision-modal-apply-to-all = 모두 적용
# MT
collision-modal-source = 원본
# MT
collision-modal-destination = 대상
# MT
collision-modal-size = 크기
# MT
collision-modal-modified = 수정됨
# MT
collision-modal-hash-check = 빠른 해시 (SHA-256)
# MT
collision-modal-rename-placeholder = 새 파일 이름
# MT
collision-modal-confirm-rename = 이름 바꾸기

# MT — Error log drawer
error-log-title = 오류 로그
# MT
error-log-empty = 기록된 오류가 없습니다
# MT
error-log-export-csv = CSV 내보내기
# MT
error-log-export-txt = 텍스트 내보내기
# MT
error-log-clear = 로그 지우기
# MT
error-log-col-time = 시간
# MT
error-log-col-job = 작업
# MT
error-log-col-path = 경로
# MT
error-log-col-code = 코드
# MT
error-log-col-message = 메시지
# MT
error-log-col-resolution = 해결

# MT — History drawer (Phase 9)
history-title = 기록
# MT
history-empty = 기록된 작업이 없습니다
# MT
history-unavailable = 복사 기록을 사용할 수 없습니다. 앱이 시작 시 SQLite 저장소를 열지 못했습니다.
# MT
history-filter-any = 모두
# MT
history-filter-kind = 종류
# MT
history-filter-status = 상태
# MT
history-filter-text = 검색
# MT
history-refresh = 새로 고침
# MT
history-export-csv = CSV 내보내기
# MT
history-purge-30 = 30일 이전 삭제
# MT
history-rerun = 다시 실행
# MT
history-detail-open = 세부 정보
# MT
history-detail-title = 작업 세부 정보
# MT
history-detail-empty = 기록된 항목이 없습니다
# MT
history-col-date = 날짜
# MT
history-col-kind = 종류
# MT
history-col-src = 원본
# MT
history-col-dst = 대상
# MT
history-col-files = 파일
# MT
history-col-size = 크기
# MT
history-col-status = 상태
# MT
history-col-duration = 소요 시간
# MT
history-col-error = 오류

# MT
toast-history-exported = 기록을 내보냈습니다
# MT
toast-history-rerun-queued = 다시 실행을 대기열에 추가했습니다

# MT — Totals drawer (Phase 10)
footer-totals = 합계
# MT
totals-title = 합계
# MT
totals-loading = 합계 불러오는 중…
# MT
totals-card-bytes = 총 복사 바이트
# MT
totals-card-files = 파일
# MT
totals-card-jobs = 작업
# MT
totals-card-avg-rate = 평균 처리량
# MT
totals-errors = 오류
# MT
totals-spark-title = 지난 30일
# MT
totals-kinds-title = 종류별
# MT
totals-saved-title = 절약된 시간 (추정)
# MT
totals-saved-note = 표준 파일 관리자로 동일한 작업을 복사한 기준과 비교한 추정치입니다.
# MT
totals-reset = 통계 재설정
# MT
totals-reset-confirm = 저장된 모든 작업과 항목이 삭제됩니다. 계속하시겠습니까?
# MT
totals-reset-confirm-yes = 예, 재설정
# MT
toast-totals-reset = 통계가 재설정되었습니다

# MT — Phase 11a additions
header-language-label = 언어
# MT
header-language-title = 언어 변경

# MT
kind-copy = 복사
# MT
kind-move = 이동
# MT
kind-delete = 삭제
# MT
kind-secure-delete = 안전하게 삭제

# MT
status-running = 실행 중
# MT
status-succeeded = 성공
# MT
status-failed = 실패
# MT
status-cancelled = 취소됨
# MT
status-ok = 확인
# MT
status-skipped = 건너뜀

# MT
history-search-placeholder = /경로
# MT
toast-history-purged = 30일보다 오래된 작업 { $count }개 삭제됨

# MT
err-source-required = 하나 이상의 원본 경로가 필요합니다.
# MT
err-destination-empty = 대상 경로가 비어 있습니다.
# MT
err-source-empty = 원본 경로가 비어 있습니다.

# MT
duration-lt-1s = < 1초
# MT
duration-ms = { $ms } ms
# MT
duration-seconds = { $s }초
# MT
duration-minutes-seconds = { $m }분 { $s }초
# MT
duration-hours-minutes = { $h }시간 { $m }분
# MT
duration-zero = 0초

# MT
rate-unit-per-second = { $size }/초

# MT — Phase 11b Settings modal
settings-title = 설정
# MT
settings-tab-general = 일반
# MT
settings-tab-appearance = 모양
# MT
settings-section-language = 언어
# MT
settings-phase-12-hint = 더 많은 설정(테마, 전송 기본값, 확인 알고리즘, 프로필)은 12단계에서 추가됩니다.

# MT — Phase 12 Settings window
settings-loading = 설정 불러오는 중…
# MT
settings-tab-transfer = 전송
# MT
settings-tab-shell = 셸
# MT
settings-tab-secure-delete = 안전하게 삭제
# MT
settings-tab-advanced = 고급
# MT
settings-tab-profiles = 프로필

# MT
settings-section-theme = 테마
# MT
settings-theme-auto = 자동
# MT
settings-theme-light = 라이트
# MT
settings-theme-dark = 다크
# MT
settings-start-with-os = 시스템 시작 시 실행
# MT
settings-single-instance = 단일 인스턴스 실행
# MT
settings-minimize-to-tray = 닫을 때 트레이로 최소화
settings-error-display-mode = 오류 프롬프트 스타일
settings-error-display-modal = 모달 (앱 차단)
settings-error-display-drawer = 드로어 (비차단)
settings-error-display-mode-hint = 모달은 결정할 때까지 큐를 중지합니다. 드로어는 큐를 계속 실행하고 모서리에서 오류를 분류할 수 있습니다.
settings-paste-shortcut = 전역 단축키로 파일 붙여넣기
settings-paste-shortcut-combo = 단축키 조합
settings-paste-shortcut-hint = 시스템 어디에서나 이 조합을 누르면 Explorer / Finder / Files에서 복사한 파일을 Copy That을 통해 붙여넣을 수 있습니다. CmdOrCtrl은 macOS에서는 Cmd, Windows / Linux에서는 Ctrl로 해석됩니다.
settings-clipboard-watcher = 복사된 파일에 대해 클립보드 감시
settings-clipboard-watcher-hint = 파일 URL이 클립보드에 나타날 때 토스트를 표시하여 Copy That으로 붙여넣을 수 있음을 알립니다. 활성화된 동안 500 ms마다 폴링합니다.

# MT
settings-buffer-size = 버퍼 크기
# MT
settings-verify = 복사 후 확인
# MT
settings-verify-off = 끄기
# MT
settings-concurrency = 동시성
# MT
settings-concurrency-auto = 자동
# MT
settings-reflink = Reflink / 빠른 경로
# MT
settings-reflink-prefer = 선호
# MT
settings-reflink-avoid = reflink 피하기
# MT
settings-reflink-disabled = 항상 비동기 엔진 사용
# MT
settings-fsync-on-close = 닫을 때 디스크에 동기화 (느림, 안전함)
# MT
settings-preserve-timestamps = 타임스탬프 유지
# MT
settings-preserve-permissions = 권한 유지
# MT
settings-preserve-acls = ACL 유지 (14단계)
settings-preserve-sparseness = 스파스 파일 보존  # MT
settings-preserve-sparseness-hint = 스파스 파일(VM 디스크, 데이터베이스 파일)의 할당된 범위만 복사하여 대상이 원본과 동일한 디스크 크기를 유지합니다.  # MT

# MT
settings-context-menu = 셸 컨텍스트 메뉴 항목 사용
# MT
settings-intercept-copy = 기본 복사 처리기 가로채기 (Windows)
# MT
settings-intercept-copy-hint = 켜면 Explorer의 Ctrl+C / Ctrl+V가 Copy That을 거칩니다. 등록은 14단계.
# MT
settings-notify-completion = 작업 완료 시 알림

# MT
settings-shred-method = 기본 파쇄 방법
# MT
settings-shred-zero = 영 (1회)
# MT
settings-shred-random = 무작위 (1회)
# MT
settings-shred-dod3 = DoD 5220.22-M (3회)
# MT
settings-shred-dod7 = DoD 5220.22-M (7회)
# MT
settings-shred-gutmann = Gutmann (35회)
# MT
settings-shred-nist = NIST 800-88
# MT
settings-shred-confirm-twice = 파쇄 전 이중 확인 요구

# MT
settings-log-level = 로그 수준
# MT
settings-log-off = 끄기
# MT
settings-telemetry = 원격 측정
# MT
settings-telemetry-never = 사용 안 함 — 어떤 로그 수준에서도 데이터 전송 없음
# MT
settings-error-policy = 기본 오류 정책
# MT
settings-error-policy-ask = 물어보기
# MT
settings-error-policy-skip = 건너뛰기
# MT
settings-error-policy-retry = 대기 후 재시도
# MT
settings-error-policy-abort = 첫 오류에 중단
# MT
settings-history-retention = 기록 보존 (일)
# MT
settings-history-retention-hint = 0 = 영구 보관. 다른 값은 시작 시 오래된 작업을 자동 정리합니다.
# MT
settings-database-path = 데이터베이스 경로
# MT
settings-database-path-default = (기본값 — OS 데이터 디렉터리)
# MT
settings-reset-all = 기본값으로 재설정
# MT
settings-reset-confirm = 모든 기본 설정을 재설정하시겠습니까? 프로필은 영향받지 않습니다.

# MT
settings-profiles-hint = 현재 설정을 이름으로 저장하세요; 나중에 불러오면 개별 조정 없이 전환할 수 있습니다.
# MT
settings-profile-name-placeholder = 프로필 이름
# MT
settings-profile-save = 저장
# MT
settings-profile-import = 가져오기…
# MT
settings-profile-load = 불러오기
# MT
settings-profile-export = 내보내기…
# MT
settings-profile-delete = 삭제
# MT
settings-profile-empty = 저장된 프로필이 없습니다.
# MT
settings-profile-import-prompt = 가져올 프로필의 이름:

# MT
toast-settings-reset = 설정이 재설정되었습니다
# MT
toast-profile-saved = 프로필이 저장되었습니다
# MT
toast-profile-loaded = 프로필을 불러왔습니다
# MT
toast-profile-exported = 프로필을 내보냈습니다
# MT
toast-profile-imported = 프로필을 가져왔습니다

# Phase 13d — activity feed + header picker buttons
action-add-files = 파일 추가
action-add-folders = 폴더 추가
activity-title = 활동
activity-clear = 활동 목록 지우기
activity-empty = 아직 파일 활동이 없습니다.
activity-after-done = 완료 시:
activity-keep-open = 앱 열어두기
activity-close-app = 앱 닫기
activity-shutdown = PC 종료
activity-logoff = 로그오프
activity-sleep = 절전 모드

# Phase 14 — preflight free-space dialog
preflight-block-title = 대상에 공간이 부족합니다
preflight-warn-title = 대상의 공간이 부족합니다
preflight-unknown-title = 여유 공간을 확인할 수 없습니다
preflight-unknown-body = 원본이 너무 커서 빠르게 측정할 수 없거나 대상 볼륨이 응답하지 않았습니다. 계속할 수 있으며, 공간이 부족해지면 엔진이 복사를 깔끔하게 중단합니다.
preflight-required = 필요
preflight-free = 남음
preflight-reserve = 예약
preflight-shortfall = 부족
preflight-continue = 그래도 계속
collision-modal-overwrite-older = 오래된 파일만 덮어쓰기

# Phase 14e — subset picker
preflight-pick-subset = 복사할 항목 선택…
subset-title = 복사할 원본 선택
subset-subtitle = 전체 선택이 대상에 맞지 않습니다. 복사할 항목을 선택하면 나머지는 복사되지 않습니다.
subset-loading = 크기 측정 중…
subset-too-large = 너무 커서 셀 수 없음
subset-budget = 사용 가능
subset-remaining = 남음
subset-confirm = 선택 복사
history-rerun-hint = 이 복사 재실행 — 원본 트리의 모든 파일을 다시 스캔
history-clear-all = 전체 지우기
history-clear-all-confirm = 확인하려면 다시 클릭
history-clear-all-hint = 모든 기록 행을 삭제합니다. 두 번째 클릭으로 확인합니다.
toast-history-cleared = 기록이 지워졌습니다 ({ $count } 행 제거됨)

# Phase 15 — source-list ordering
drop-dialog-sort-label = 정렬:
sort-custom = 사용자 지정
sort-name-asc = 이름 A → Z (파일 먼저)
sort-name-desc = 이름 Z → A (파일 먼저)
sort-size-asc = 크기 작은 순 (파일 먼저)
sort-size-desc = 크기 큰 순 (파일 먼저)
sort-reorder = 재정렬
sort-move-top = 맨 위로 이동
sort-move-up = 위로
sort-move-down = 아래로
sort-move-bottom = 맨 아래로 이동
sort-name-asc-simple = 이름 A → Z
sort-name-desc-simple = 이름 Z → A
sort-size-asc-simple = 작은 순
sort-size-desc-simple = 큰 순
activity-sort-locked = 복사가 실행 중일 때는 정렬이 비활성화됩니다. 일시중지하거나 완료를 기다린 후 순서를 변경하세요.
drop-dialog-collision-label = 파일이 이미 존재하는 경우:
collision-policy-keep-both = 둘 다 유지 (새 복사본의 이름을 _2, _3 … 로 변경)
collision-policy-skip = 새 복사본 건너뛰기
collision-policy-overwrite = 기존 파일 덮어쓰기
collision-policy-overwrite-if-newer = 새로운 경우에만 덮어쓰기
collision-policy-prompt = 매번 묻기
drop-dialog-busy-checking = 여유 공간 확인 중…
drop-dialog-busy-enumerating = 파일 수 계산 중…
drop-dialog-busy-starting = 복사 시작 중…
toast-enumeration-deferred = 원본 트리가 큽니다 — 사전 목록을 생략합니다. 엔진이 처리하는 대로 행이 나타납니다.

# Phase 14a — enumeration-time filters
# MT
settings-tab-filters = 필터
# MT
settings-filters-hint = 열거 시점에서 파일을 건너뛰어 엔진이 아예 열지 않게 합니다. 포함은 파일에만 적용되고, 제외는 디렉터리도 가지치기합니다.
# MT
settings-filters-enabled = 트리 복사에 필터 사용
# MT
settings-filters-include-globs = 포함 글롭
# MT
settings-filters-include-globs-placeholder = **/*.txt
# MT
settings-filters-include-globs-hint = 한 줄에 하나. 비어 있지 않으면 파일은 최소 하나와 일치해야 합니다. 디렉터리는 항상 내려갑니다.
# MT
settings-filters-exclude-globs = 제외 글롭
# MT
settings-filters-exclude-globs-placeholder = **/node_modules
# MT
settings-filters-exclude-globs-hint = 한 줄에 하나. 디렉터리에 일치하면 서브트리 전체를 가지치기하고, 파일에 일치하면 건너뜁니다.
# MT
settings-filters-size-range = 파일 크기 범위
# MT
settings-filters-min-size-bytes = 최소 크기(바이트, 빈 값 = 하한 없음)
# MT
settings-filters-max-size-bytes = 최대 크기(바이트, 빈 값 = 상한 없음)
# MT
settings-filters-date-range = 수정 시간 범위
# MT
settings-filters-min-mtime = 이 날짜 이후 수정됨
# MT
settings-filters-max-mtime = 이 날짜 이전 수정됨
# MT
settings-filters-attributes = 속성 비트
# MT
settings-filters-skip-hidden = 숨김 파일 / 폴더 건너뛰기
# MT
settings-filters-skip-system = 시스템 파일 건너뛰기(Windows 전용)
# MT
settings-filters-skip-readonly = 읽기 전용 파일 건너뛰기

# Phase 15 — auto-update
# MT
settings-tab-updater = 업데이트
# MT
settings-updater-hint = Copy That는 하루에 한 번 서명된 업데이트를 확인합니다. 다음 앱 종료 시 업데이트가 설치됩니다.
# MT
settings-updater-auto-check = 실행 시 업데이트 확인
# MT
settings-updater-channel = 릴리스 채널
# MT
settings-updater-channel-stable = 안정 버전
# MT
settings-updater-channel-beta = 베타 (프리릴리스)
# MT
settings-updater-last-check = 마지막 확인
# MT
settings-updater-last-never = 없음
# MT
settings-updater-check-now = 지금 업데이트 확인
# MT
settings-updater-checking = 확인 중…
# MT
settings-updater-available = 업데이트 사용 가능
# MT
settings-updater-up-to-date = 최신 릴리스를 사용 중입니다.
# MT
settings-updater-dismiss = 이 버전 건너뛰기
# MT
settings-updater-dismissed = 건너뜀
# MT
toast-update-available = 새 버전이 사용 가능합니다
# MT
toast-update-up-to-date = 이미 최신 버전입니다

# Phase 19a — disk-backed file enumeration (TeraCopy-compatible scan DB)
# MT
scan-progress-title = 스캔 중…
# MT
scan-progress-stats = { $files } 파일 · 지금까지 { $bytes }
# MT
scan-pause-button = 스캔 일시 정지
# MT
scan-resume-button = 스캔 재개
# MT
scan-cancel-button = 스캔 취소
# MT
scan-cancel-confirm = 스캔을 취소하고 진행 상황을 버리시겠습니까?
# MT
scan-db-header = 스캔 데이터베이스
# MT
scan-db-hint = 수백만 개 파일 작업을 위한 디스크 기반 스캔 데이터베이스.
# MT
advanced-scan-hash-during = 스캔 중 체크섬 계산
# MT
advanced-scan-db-path = 스캔 데이터베이스 위치
# MT
advanced-scan-retention-days = 완료된 스캔 자동 삭제 기간(일)
# MT
advanced-scan-max-keep = 보관할 최대 스캔 데이터베이스 수

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
sparse-not-supported-title = 대상이 스파스 파일을 채움  # MT
sparse-not-supported-body = { $dst_fs }는 스파스 파일을 지원하지 않습니다. 원본의 구멍이 0으로 기록되어 대상이 디스크에서 더 큽니다.  # MT
sparse-warning-densified = 스파스 레이아웃 보존: 할당된 범위만 복사되었습니다.  # MT
sparse-warning-mismatch = 스파스 레이아웃 불일치 — 대상이 예상보다 클 수 있습니다.  # MT

# Phase 24 — security-metadata preservation. MT-flagged drafts;
# the authoritative English source lives in locales/en/copythat.ftl.
settings-preserve-security-metadata = 보안 메타데이터 보존  # MT
settings-preserve-security-metadata-hint = 모든 복사에서 대역 외 메타데이터 스트림(NTFS ADS / xattrs / POSIX ACL / SELinux 컨텍스트 / Linux 파일 기능 / macOS 리소스 포크)을 캡처하고 다시 적용합니다.  # MT
settings-preserve-motw = Mark-of-the-Web(인터넷에서 다운로드 플래그) 보존  # MT
settings-preserve-motw-hint = 보안에 매우 중요. SmartScreen 및 Office Protected View는 이 스트림을 사용하여 인터넷에서 다운로드한 파일에 대해 경고합니다. 비활성화하면 다운로드한 실행 파일이 복사 시 출처 표식을 잃고 운영 체제 보호를 우회할 수 있습니다.  # MT
settings-preserve-posix-acls = POSIX ACL 및 확장 속성 보존  # MT
settings-preserve-posix-acls-hint = 복사 시 user.* / system.* / trusted.* xattrs 및 POSIX 액세스 제어 목록을 전달합니다.  # MT
settings-preserve-selinux = SELinux 컨텍스트 보존  # MT
settings-preserve-selinux-hint = MAC 정책하에서 실행되는 데몬이 파일에 계속 액세스할 수 있도록 복사 시 security.selinux 레이블을 전달합니다.  # MT
settings-preserve-resource-forks = macOS 리소스 포크 및 Finder 정보 보존  # MT
settings-preserve-resource-forks-hint = 복사 시 레거시 리소스 포크 및 FinderInfo(색상 태그, Carbon 메타데이터)를 전달합니다.  # MT
settings-appledouble-fallback = 호환되지 않는 파일 시스템에서 AppleDouble 사이드카 사용  # MT
meta-translated-to-appledouble = 외래 메타데이터가 AppleDouble 사이드카에 저장됨 (._{ $ext })  # MT

# Phase 25 — two-way sync with vector-clock conflict detection.
# MT-flagged drafts; the authoritative English source lives in
# locales/en/copythat.ftl.
footer-sync = 동기화  # MT
sync-drawer-title = 양방향 동기화  # MT
sync-drawer-hint = 두 폴더를 조용한 덮어쓰기 없이 동기화합니다. 동시 편집은 해결 가능한 충돌로 표시됩니다.  # MT
sync-add-pair = 쌍 추가  # MT
sync-add-cancel = 취소  # MT
sync-refresh = 새로 고침  # MT
sync-add-save = 쌍 저장  # MT
sync-add-saving = 저장 중…  # MT
sync-add-missing-fields = 레이블, 왼쪽 경로, 오른쪽 경로는 모두 필수입니다.  # MT
sync-remove-confirm = 이 동기화 쌍을 제거하시겠습니까? 상태 데이터베이스는 보존되며 폴더는 영향을 받지 않습니다.  # MT
sync-field-label = 레이블  # MT
sync-field-label-placeholder = 예: 문서 ↔ NAS  # MT
sync-field-left = 왼쪽 폴더  # MT
sync-field-left-placeholder = 절대 경로를 선택하거나 붙여넣기  # MT
sync-field-right = 오른쪽 폴더  # MT
sync-field-right-placeholder = 절대 경로를 선택하거나 붙여넣기  # MT
sync-field-mode = 모드  # MT
sync-mode-two-way = 양방향  # MT
sync-mode-mirror-left-to-right = 미러 (왼쪽 → 오른쪽)  # MT
sync-mode-mirror-right-to-left = 미러 (오른쪽 → 왼쪽)  # MT
sync-mode-contribute-left-to-right = 기여 (왼쪽 → 오른쪽, 삭제 없음)  # MT
sync-no-pairs = 아직 구성된 동기화 쌍이 없습니다. "쌍 추가"를 클릭하여 시작하세요.  # MT
sync-loading = 구성된 쌍 로드 중…  # MT
sync-never-run = 실행된 적 없음  # MT
sync-running = 실행 중  # MT
sync-run-now = 지금 실행  # MT
sync-cancel = 취소  # MT
sync-remove-pair = 제거  # MT
sync-view-conflicts = 충돌 보기 ({ $count })  # MT
sync-conflicts-heading = 충돌  # MT
sync-no-conflicts = 마지막 실행의 충돌 없음.  # MT
sync-winner = 승자  # MT
sync-side-left-to-right = 왼쪽  # MT
sync-side-right-to-left = 오른쪽  # MT
sync-conflict-kind-concurrent-write = 동시 편집  # MT
sync-conflict-kind-delete-edit = 삭제 ↔ 편집  # MT
sync-conflict-kind-add-add = 양쪽 모두 추가함  # MT
sync-conflict-kind-corrupt-equal = 새로 쓰기 없이 내용이 분기됨  # MT
sync-resolve-keep-left = 왼쪽 유지  # MT
sync-resolve-keep-right = 오른쪽 유지  # MT
sync-resolve-keep-both = 둘 다 유지  # MT
sync-resolve-three-way = 3-way 병합으로 해결  # MT
sync-resolve-phase-53-tooltip = 비텍스트 파일을 위한 대화형 3-way 병합은 53단계에 제공됩니다.  # MT
sync-error-prefix = 동기화 오류  # MT

# Phase 26 — real-time mirror watcher. MT-flagged drafts;
# the authoritative English source lives in locales/en/copythat.ftl.
live-mirror-start = 실시간 미러 시작  # MT
live-mirror-stop = 실시간 미러 중지  # MT
live-mirror-watching = 감시 중  # MT
live-mirror-toggle-hint = 감지된 모든 파일 시스템 변경 시 자동으로 다시 동기화합니다. 활성 쌍당 하나의 백그라운드 스레드.  # MT
watch-event-prefix = 파일 변경  # MT
watch-overflow-recovered = 감시자 버퍼 오버플로; 복구를 위해 재열거 중  # MT

# Phase 27 — content-defined chunk store. MT-flagged drafts;
# the authoritative English source lives in locales/en/copythat.ftl.
chunk-store-section = 청크 저장소  # MT
chunk-store-enable = 청크 저장소 활성화 (델타 재개 및 중복 제거)  # MT
chunk-store-enable-hint = 복사되는 모든 파일을 콘텐츠별로 분할 (FastCDC)하고 청크를 콘텐츠 주소 지정 방식으로 저장합니다. 재시도는 변경된 청크만 다시 씁니다. 공유 콘텐츠가 있는 파일은 자동으로 중복 제거됩니다.  # MT
chunk-store-location = 청크 저장소 위치  # MT
chunk-store-max-size = 청크 저장소 최대 크기  # MT
chunk-store-prune = 다음 일수보다 오래된 청크 정리 (일)  # MT
chunk-store-savings = 청크 중복 제거를 통해 { $gib } GiB 절약됨  # MT
chunk-store-disk-usage = { $chunks }개 청크에 { $size } 사용 중  # MT

# Phase 28 — tray-resident Drop Stack. MT-flagged drafts;
# the authoritative English source lives in locales/en/copythat.ftl.
dropstack-window-title = 드롭 스택  # MT
dropstack-tray-open = 드롭 스택  # MT
dropstack-empty-title = 드롭 스택이 비어 있습니다  # MT
dropstack-empty-hint = 탐색기에서 파일을 여기로 끌거나 작업 행을 마우스 오른쪽 버튼으로 클릭하여 추가하세요.  # MT
dropstack-add-to-stack = 드롭 스택에 추가  # MT
dropstack-copy-all-to = 모두 복사…  # MT
dropstack-move-all-to = 모두 이동…  # MT
dropstack-clear = 스택 지우기  # MT
dropstack-remove-row = 스택에서 제거  # MT
dropstack-path-missing-toast = { $path }을(를) 제거했습니다 — 파일이 더 이상 존재하지 않습니다.  # MT
dropstack-always-on-top = 드롭 스택을 항상 위에 유지  # MT
dropstack-show-tray-icon = Copy That 트레이 아이콘 표시  # MT
dropstack-open-on-start = 앱 시작 시 드롭 스택 자동 열기  # MT
dropstack-count = { $count }개 경로  # MT

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
settings-tab-remotes = Remotes  # MT

# Phase 33 — mount as read-only filesystem.
mount-heading = Mount snapshot  # MT
mount-action-mount = Mount snapshot  # MT
mount-action-unmount = Unmount  # MT
mount-status-mounted = Mounted at { $path }  # MT
mount-error-unsafe-mountpoint = Mountpoint path is unsafe  # MT
mount-error-mountpoint-not-empty = Mountpoint must be an empty directory  # MT
mount-error-backend-unavailable = Mount backend is not available on this system  # MT
mount-error-archive-read = Archive read failed  # MT
mount-picker-title = Pick mountpoint directory  # MT
mount-toast-mounted = Snapshot mounted at { $path }  # MT
mount-toast-unmounted = Snapshot unmounted  # MT
mount-toast-failed = Mount failed: { $reason }  # MT
settings-mount-heading = Mount snapshots  # MT
settings-mount-hint = Expose the history archive as a read-only filesystem. Phase 33b wires the runner flow; the kernel FUSE/WinFsp backends land in Phase 33c.  # MT
settings-mount-on-launch = Mount the latest snapshot on launch  # MT
settings-mount-on-launch-path = Mountpoint path  # MT
settings-mount-on-launch-path-placeholder = e.g. C:\Mounts\copythat  # MT

# Phase 34 — enterprise-grade audit log export + WORM (write-once-read-
# many) tamper-resistance. Settings → Advanced → Audit log collects the
# format + destination + rotation + WORM toggle; runtime records job and
# file events into the tamper-evident chain-hashed sink.
settings-audit-heading = Audit log  # MT
settings-audit-hint = Append-only tamper-evident log of every job and file event. Formats include CSV, JSON-lines, RFC 5424 Syslog, ArcSight CEF, and QRadar LEEF.  # MT
settings-audit-enable = Enable audit logging  # MT
settings-audit-format = Log format  # MT
settings-audit-format-json-lines = JSON lines (recommended default)  # MT
settings-audit-format-csv = CSV (spreadsheet-friendly)  # MT
settings-audit-format-syslog = Syslog (RFC 5424)  # MT
settings-audit-format-cef = CEF (ArcSight)  # MT
settings-audit-format-leef = LEEF 2.0 (IBM QRadar)  # MT
settings-audit-file-path = Log file path  # MT
settings-audit-file-path-placeholder = e.g. C:\ProgramData\CopyThat\audit.log  # MT
settings-audit-max-size = Rotate after (bytes, 0 = never)  # MT
settings-audit-worm = Enable WORM mode (write-once-read-many)  # MT
settings-audit-worm-hint = Applies the platform's append-only flag (Linux chattr +a, macOS chflags uappnd, Windows read-only attribute) after every create or rotation. Even an administrator must explicitly clear the flag to truncate the log.  # MT
settings-audit-test-write = Test write  # MT
settings-audit-verify-chain = Verify chain  # MT
toast-audit-test-write-ok = Audit log test write succeeded  # MT
toast-audit-verify-ok = Audit chain verified intact  # MT
toast-audit-verify-failed = Audit chain verification reported mismatches  # MT

# Phase 35 — destination encryption via age (X25519 / passphrase /
# SSH) + on-the-fly zstd compression with a per-extension deny
# heuristic. Surfaced through Settings → Transfer → Encryption +
# Compression; engine short-circuits to the crypt pipeline when
# either stage is active.
settings-crypt-heading = Encryption & compression  # MT
settings-crypt-hint = Transform file contents before they land at the destination. Encryption uses the age format; compression uses zstd and can skip already-compressed media by extension.  # MT
settings-crypt-encryption-mode = Encryption  # MT
settings-crypt-encryption-off = Off  # MT
settings-crypt-encryption-passphrase = Passphrase (prompt at copy start)  # MT
settings-crypt-encryption-recipients = Recipient keys from file  # MT
settings-crypt-encryption-hint = Passphrases are held only in memory for the duration of the copy. Recipient files list one age1… or ssh- public key per line.  # MT
settings-crypt-recipients-file = Recipients file path  # MT
settings-crypt-recipients-file-placeholder = e.g. C:\Users\me\recipients.txt  # MT
settings-crypt-compression-mode = Compression  # MT
settings-crypt-compression-off = Off  # MT
settings-crypt-compression-always = Always  # MT
settings-crypt-compression-smart = Smart (skip already-compressed media)  # MT
settings-crypt-compression-hint = Smart mode skips jpg, mp4, zip, 7z and similar formats that don't benefit from zstd. Always mode compresses every file at the chosen level.  # MT
settings-crypt-compression-level = zstd level (1-22)  # MT
settings-crypt-compression-level-hint = Lower numbers are faster; higher numbers compress harder. Level 3 matches zstd's CLI default.  # MT
compress-footer-savings = 💾 { $original } → { $compressed } ({ $percent }% saved)  # MT
compress-savings-toast = Compressed { $percent }% ({ $bytes } saved)  # MT
crypt-toast-recipients-loaded = Loaded { $count } encryption recipients  # MT
crypt-toast-recipients-error = Failed to load recipients: { $reason }  # MT
crypt-toast-passphrase-required = Encryption needs a passphrase before the copy starts  # MT
crypt-toast-passphrase-set = Encryption passphrase captured  # MT
crypt-footer-encrypted-badge = 🔒 Encrypted (age)  # MT
crypt-footer-compressed-badge = 📦 Compressed (zstd)  # MT
