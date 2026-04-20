app-name = Copy That 2026
# MT
window-title = Copy That 2026
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
err-io-other = 알 수 없는 I/O 오류

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
