app-name = Copy That 2026
# MT
window-title = Copy That 2026
# MT
shred-ssd-advisory = चेतावनी: यह लक्ष्य SSD पर है। वियर-लेवलिंग और ओवर-प्रोविजनिंग लॉजिकल ब्लॉक पते के नीचे से डेटा हटा देते हैं, इसलिए बहु-पास ओवरराइट फ्लैश मेमोरी को विश्वसनीय रूप से साफ नहीं करते। सॉलिड-स्टेट मीडिया के लिए ATA SECURE ERASE, NVMe फॉर्मैट विद सिक्योर इरेज़, या त्यागी गई कुंजी वाले फुल-डिस्क एन्क्रिप्शन को प्राथमिकता दें।

# MT
state-idle = निष्क्रिय
# MT
state-copying = कॉपी हो रहा है
# MT
state-verifying = सत्यापन हो रहा है
# MT
state-paused = रोका गया
# MT
state-error = त्रुटि

# MT
state-pending = कतार में
# MT
state-running = चल रहा है
# MT
state-cancelled = रद्द किया गया
# MT
state-succeeded = पूर्ण
# MT
state-failed = विफल

# MT
action-pause = रोकें
# MT
action-resume = जारी रखें
# MT
action-cancel = रद्द करें
# MT
action-pause-all = सभी कार्य रोकें
# MT
action-resume-all = सभी कार्य जारी रखें
# MT
action-cancel-all = सभी कार्य रद्द करें
# MT
action-close = बंद करें
# MT
action-reveal = फ़ोल्डर में दिखाएँ

# MT
menu-pause = रोकें
# MT
menu-resume = जारी रखें
# MT
menu-cancel = रद्द करें
# MT
menu-remove = कतार से हटाएँ
# MT
menu-reveal-source = स्रोत फ़ोल्डर में दिखाएँ
# MT
menu-reveal-destination = गंतव्य फ़ोल्डर में दिखाएँ

# MT
header-eta-label = अनुमानित शेष समय
# MT
header-toolbar-label = वैश्विक नियंत्रण

# MT
footer-queued = सक्रिय कार्य
# MT
footer-total-bytes = प्रगति पर
# MT
footer-errors = त्रुटियाँ
# MT
footer-history = इतिहास

# MT
empty-title = कॉपी करने के लिए फ़ाइलें या फ़ोल्डर छोड़ें
# MT
empty-hint = विंडो पर आइटम खींचें। हम गंतव्य पूछेंगे, फिर प्रत्येक स्रोत के लिए एक कार्य जोड़ेंगे।
# MT
empty-region-label = कार्य सूची

# MT
details-drawer-label = कार्य विवरण
# MT
details-source = स्रोत
# MT
details-destination = गंतव्य
# MT
details-state = स्थिति
# MT
details-bytes = बाइट्स
# MT
details-files = फ़ाइलें
# MT
details-speed = गति
# MT
details-eta = शेष समय
# MT
details-error = त्रुटि

# MT
drop-dialog-title = छोड़े गए आइटम स्थानांतरित करें
# MT
drop-dialog-subtitle = { $count } आइटम स्थानांतरण के लिए तैयार। प्रारंभ करने के लिए गंतव्य फ़ोल्डर चुनें।
# MT
drop-dialog-mode = ऑपरेशन
# MT
drop-dialog-copy = कॉपी
# MT
drop-dialog-move = मूव
# MT
drop-dialog-pick-destination = गंतव्य चुनें
# MT
drop-dialog-change-destination = गंतव्य बदलें
# MT
drop-dialog-start-copy = कॉपी शुरू करें
# MT
drop-dialog-start-move = मूव शुरू करें

# MT
eta-calculating = गणना हो रही है…
# MT
eta-unknown = अज्ञात

# MT
toast-job-done = स्थानांतरण पूरा हुआ
# MT
toast-copy-queued = कॉपी कतारबद्ध
# MT
toast-move-queued = मूव कतारबद्ध
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
