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
