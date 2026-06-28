app-name = Copy That v0.19.84
window-title = Copy That v0.19.84
shred-ssd-advisory = चेतावनी: यह लक्ष्य एक SSD पर मौजूद है। मल्टी-पास ओवरराइट फ़्लैश मेमोरी को विश्वसनीय रूप से साफ़ नहीं करते क्योंकि वियर-लेवलिंग और ओवर-प्रोविज़निंग डेटा को लॉजिकल ब्लॉक एड्रेस के नीचे से हटा देते हैं। सॉलिड-स्टेट मीडिया के लिए ATA SECURE ERASE, NVMe Format with Secure Erase, या डिस्कार्डेड की के साथ फ़ुल-डिस्क एन्क्रिप्शन को प्राथमिकता दें।

# Global aggregate states (header pill)
state-idle = निष्क्रिय
state-copying = कॉपी हो रहा है
state-verifying = सत्यापित हो रहा है
state-paused = रोका गया
state-error = त्रुटि

# Per-job states (row badge)
state-pending = कतार में
state-running = चल रहा है
state-cancelled = रद्द किया गया
state-succeeded = पूर्ण
state-failed = विफल

# Actions
action-pause = रोकें
action-resume = जारी रखें
action-cancel = रद्द करें
action-pause-all = सभी कार्य रोकें
action-resume-all = सभी कार्य जारी रखें
action-cancel-all = सभी कार्य रद्द करें
action-close = बंद करें
action-reveal = फ़ोल्डर में दिखाएँ
action-add-files = फ़ाइलें जोड़ें
action-add-folders = फ़ोल्डर जोड़ें

# Phase 13d — activity feed
activity-title = गतिविधि
activity-clear = गतिविधि सूची साफ़ करें
activity-empty = अभी तक कोई फ़ाइल गतिविधि नहीं।
activity-after-done = पूर्ण होने पर:
activity-keep-open = ऐप खुला रखें
activity-close-app = ऐप बंद करें
activity-shutdown = PC बंद करें
activity-logoff = लॉग ऑफ़ करें
activity-sleep = स्लीप

# Phase 14 — preflight free-space dialog
preflight-block-title = गंतव्य पर पर्याप्त जगह नहीं है
preflight-warn-title = गंतव्य पर कम जगह है
preflight-unknown-title = खाली जगह का पता नहीं लगाया जा सका
preflight-unknown-body = स्रोत इतना बड़ा है कि उसका आकार जल्दी नहीं निकाला जा सका, या गंतव्य वॉल्यूम ने प्रतिक्रिया नहीं दी। आप जारी रख सकते हैं; जगह खत्म होने पर इंजन का स्पेस गार्ड कॉपी को सफ़ाई से रोक देगा।
preflight-required = आवश्यक
preflight-free = खाली
preflight-reserve = आरक्षित
preflight-shortfall = कमी
preflight-continue = फिर भी जारी रखें
preflight-pick-subset = चुनें कि क्या कॉपी करना है…
collision-modal-overwrite-older = केवल पुराने ओवरराइट करें

# Phase 14e — subset picker
subset-title = चुनें कि कौन से स्रोत कॉपी करने हैं
subset-subtitle = पूरा चयन गंतव्य पर नहीं समाता। जिन आइटम को कॉपी करना है उन्हें टिक करें; बाकी पीछे रह जाएँगे।
subset-loading = आकार मापे जा रहे हैं…
subset-too-large = गिनने के लिए बहुत बड़ा
subset-budget = उपलब्ध
subset-remaining = शेष
subset-confirm = चयन कॉपी करें
history-rerun-hint = इस कॉपी को फिर से चलाएँ — स्रोत ट्री की हर फ़ाइल को दोबारा स्कैन करता है
history-clear-all = सभी साफ़ करें
history-clear-all-confirm = पुष्टि के लिए फिर से क्लिक करें
history-clear-all-hint = इतिहास की हर पंक्ति हटाएँ। पुष्टि के लिए दूसरी क्लिक आवश्यक है।
toast-history-cleared = इतिहास साफ़ किया गया ({ $count } पंक्तियाँ हटाई गईं)

# Phase 15 — source-list ordering in the drop staging dialog
drop-dialog-sort-label = क्रम:
sort-custom = कस्टम
sort-name-asc = नाम A → Z (फ़ाइलें पहले)
sort-name-desc = नाम Z → A (फ़ाइलें पहले)
sort-size-asc = आकार छोटा पहले (फ़ाइलें पहले)
sort-size-desc = आकार बड़ा पहले (फ़ाइलें पहले)
sort-reorder = क्रम बदलें
sort-move-top = सबसे ऊपर ले जाएँ
sort-move-up = ऊपर ले जाएँ
sort-move-down = नीचे ले जाएँ
sort-move-bottom = सबसे नीचे ले जाएँ

# Phase 16 — sort preset names for the Activity list (shorter
# wording than the DropStagingDialog variants; the Activity list
# is files-only so the "(files first)" suffix isn't relevant).
sort-name-asc-simple = नाम A → Z
sort-name-desc-simple = नाम Z → A
sort-size-asc-simple = आकार छोटा पहले
sort-size-desc-simple = आकार बड़ा पहले
activity-sort-locked = कॉपी चलने के दौरान क्रमबद्ध करना अक्षम है। रोकें या पूर्ण होने तक प्रतीक्षा करें, फिर क्रम बदलें।

# Phase 17 — collision-policy picker in the DropStagingDialog
drop-dialog-collision-label = यदि कोई फ़ाइल पहले से मौजूद है:
collision-policy-keep-both = दोनों रखें (नई कॉपी का नाम बदलकर _2, _3, …)
collision-policy-skip = नई कॉपी छोड़ें
collision-policy-overwrite = मौजूदा फ़ाइल ओवरराइट करें
collision-policy-overwrite-if-newer = केवल तभी ओवरराइट करें जब नई हो
collision-policy-prompt = हर बार पूछें

# Phase 18 — progress labels on the DropStagingDialog Start button
drop-dialog-busy-checking = खाली जगह जाँची जा रही है…
drop-dialog-busy-enumerating = फ़ाइलें गिनी जा रही हैं…
drop-dialog-busy-starting = कॉपी शुरू हो रही है…
toast-enumeration-deferred = स्रोत ट्री बड़ा है — पहले से फ़ाइल सूची नहीं बनाई जा रही; इंजन जैसे-जैसे आगे बढ़ेगा पंक्तियाँ दिखाई देंगी।

# Context menu (per-row right-click)
menu-pause = रोकें
menu-resume = जारी रखें
menu-cancel = रद्द करें
menu-remove = कतार से हटाएँ
menu-reveal-source = स्रोत को फ़ोल्डर में दिखाएँ
menu-reveal-destination = गंतव्य को फ़ोल्डर में दिखाएँ

# Header / toolbar
header-eta-label = अनुमानित शेष समय
header-toolbar-label = ग्लोबल नियंत्रण

# Footer
footer-queued = सक्रिय कार्य
footer-total-bytes = प्रगति पर
footer-errors = त्रुटियाँ
footer-history = इतिहास

# Empty state
empty-title = कॉपी करने के लिए फ़ाइलें या फ़ोल्डर छोड़ें
empty-hint = आइटम को विंडो पर खींचें। हम एक गंतव्य पूछेंगे, फिर प्रति स्रोत एक कार्य कतार में लगाएँगे।
empty-region-label = कार्य सूची

# Details drawer
details-drawer-label = कार्य विवरण
details-source = स्रोत
details-destination = गंतव्य
details-state = स्थिति
details-bytes = बाइट्स
details-files = फ़ाइलें
details-speed = गति
details-eta = ETA
details-error = त्रुटि

# Drop dialog
drop-dialog-title = छोड़े गए आइटम स्थानांतरित करें
drop-dialog-subtitle = { $count } आइटम स्थानांतरण के लिए तैयार। शुरू करने के लिए गंतव्य फ़ोल्डर चुनें।
drop-dialog-mode = ऑपरेशन
drop-dialog-copy = कॉपी करें
drop-dialog-move = स्थानांतरित करें
drop-dialog-pick-destination = गंतव्य चुनें
drop-dialog-change-destination = गंतव्य बदलें
drop-dialog-start-copy = कॉपी शुरू करें
drop-dialog-start-move = स्थानांतरण शुरू करें

# ETA placeholders
eta-calculating = गणना हो रही है…
eta-unknown = अज्ञात

# Toast messages
toast-job-done = स्थानांतरण पूर्ण हुआ
toast-copy-queued = कॉपी कतार में लगाई गई
toast-move-queued = स्थानांतरण कतार में लगाया गया
toast-error-resolved = त्रुटि हल हुई
toast-collision-resolved = टकराव हल हुआ
toast-elevated-unavailable = एलिवेटेड पुनः प्रयास Phase 17 में आएगा — अभी उपलब्ध नहीं
toast-clipboard-files-detected = क्लिपबोर्ड पर फ़ाइलें — Copy That के ज़रिए कॉपी करने के लिए अपना पेस्ट शॉर्टकट दबाएँ
toast-clipboard-no-files = क्लिपबोर्ड पर पेस्ट करने के लिए कोई फ़ाइल नहीं है
toast-error-log-exported = त्रुटि लॉग निर्यात किया गया

# Error modal (Phase 8)
error-modal-title = एक स्थानांतरण विफल हुआ
error-modal-retry = पुनः प्रयास करें
error-modal-retry-elevated = एलिवेटेड अनुमतियों के साथ पुनः प्रयास करें
error-modal-skip = छोड़ें
error-modal-skip-all-kind = इस प्रकार की सभी त्रुटियाँ छोड़ें
error-modal-abort = सभी रद्द करें
error-modal-path-label = पथ
error-modal-code-label = कोड
error-drawer-pending-count = और त्रुटियाँ प्रतीक्षारत
error-drawer-toggle = समेटें या विस्तृत करें

# Error-kind labels (Phase 8). Source of truth — engine maps each
# `CopyErrorKind` to one of these keys via `localized_key()`.
err-not-found = फ़ाइल नहीं मिली
err-permission-denied = अनुमति अस्वीकृत
err-disk-full = गंतव्य डिस्क भरी हुई है
err-interrupted = ऑपरेशन बाधित हुआ
err-verify-failed = कॉपी के बाद सत्यापन विफल हुआ
err-path-escape = पथ अस्वीकृत — इसमें पैरेंट-डायरेक्टरी (..) सेगमेंट या अवैध बाइट्स हैं
err-path-invalid-encoding = पथ अस्वीकृत — स्ट्रिंग में अमान्य UTF-8 / प्रतिस्थापन वर्ण हैं
err-helper-invalid-json = विशेषाधिकार प्राप्त हेल्पर को विकृत JSON मिला; इस अनुरोध को अनदेखा किया जा रहा है
err-helper-grant-out-of-band = GrantCapabilities को हेल्पर रन-लूप द्वारा संभाला जाना चाहिए, स्टेटलेस हैंडलर द्वारा नहीं
err-randomness-unavailable = OS रैंडम-नंबर जनरेटर विफल हुआ; सत्र आईडी नहीं बनाई जा सकती
err-sparseness-mismatch = गंतव्य पर स्पार्स लेआउट संरक्षित नहीं किया जा सका
err-io-other = अज्ञात I/O त्रुटि

# Collision modal (Phase 8)
collision-modal-title = फ़ाइल पहले से मौजूद है
collision-modal-overwrite = ओवरराइट करें
collision-modal-overwrite-if-newer = नई हो तो ओवरराइट करें
collision-modal-skip = छोड़ें
collision-modal-keep-both = दोनों रखें
collision-modal-rename = नाम बदलें…
collision-modal-apply-to-all = सभी पर लागू करें
collision-modal-source = स्रोत
collision-modal-destination = गंतव्य
collision-modal-size = आकार
collision-modal-modified = संशोधित
collision-modal-hash-check = त्वरित हैश (SHA-256)
collision-modal-hash-computing = हैश हो रहा है…
collision-modal-hash-identical = समान
collision-modal-hash-different = भिन्न
collision-modal-rename-placeholder = नया फ़ाइलनाम
collision-modal-confirm-rename = नाम बदलें

# Error log drawer (Phase 8)
error-log-title = त्रुटि लॉग
error-log-empty = कोई त्रुटि लॉग नहीं हुई
error-log-export-csv = CSV निर्यात करें
error-log-export-txt = टेक्स्ट निर्यात करें
error-log-clear = लॉग साफ़ करें
error-log-col-time = समय
error-log-col-job = कार्य
error-log-col-path = पथ
error-log-col-code = कोड
error-log-col-message = संदेश
error-log-col-resolution = समाधान

# History drawer (Phase 9)
history-title = इतिहास
history-empty = अभी तक कोई कार्य दर्ज नहीं हुआ
history-unavailable = कॉपी इतिहास उपलब्ध नहीं है। ऐप स्टार्टअप पर SQLite स्टोर नहीं खोल सका।
history-filter-any = कोई भी
history-filter-kind = प्रकार
history-filter-status = स्थिति
history-filter-text = खोजें
history-refresh = रिफ़्रेश करें
history-export-csv = CSV निर्यात करें
history-purge-30 = > 30 दिन पुराने हटाएँ
history-rerun = फिर से चलाएँ
history-detail-open = विवरण
history-detail-title = कार्य विवरण
history-detail-empty = कोई आइटम दर्ज नहीं हुआ
history-col-date = तारीख़
history-col-kind = प्रकार
history-col-src = स्रोत
history-col-dst = गंतव्य
history-col-files = फ़ाइलें
history-col-size = आकार
history-col-status = स्थिति
history-col-duration = अवधि
history-col-error = त्रुटि
toast-history-exported = इतिहास निर्यात किया गया
toast-history-rerun-queued = पुनः चलाना कतार में लगाया गया

# Totals drawer (Phase 10)
footer-totals = कुल योग
totals-title = कुल योग
totals-loading = कुल योग लोड हो रहे हैं…
totals-card-bytes = कुल बाइट्स कॉपी किए गए
totals-card-files = फ़ाइलें
totals-card-jobs = कार्य
totals-card-avg-rate = औसत थ्रूपुट
totals-errors = त्रुटियाँ
totals-spark-title = पिछले 30 दिन
totals-kinds-title = प्रकार के अनुसार
totals-saved-title = बचाया गया समय (अनुमानित)
totals-saved-note = उसी कार्यभार की एक बेसलाइन फ़ाइल-मैनेजर कॉपी की तुलना में अनुमानित।
totals-reset = आँकड़े रीसेट करें
totals-reset-confirm = यह हर संग्रहित कार्य और आइटम को हटा देता है। जारी रखें?
totals-reset-confirm-yes = हाँ, रीसेट करें
toast-totals-reset = आँकड़े रीसेट किए गए

# Phase 11a — i18n core: surface remaining user-visible strings so
# every string on a main-window screen flows through Fluent.

# Header language switcher (temporary placement — folds into
# Settings → General in Phase 12).
header-language-label = भाषा
header-language-title = भाषा बदलें

# Job-kind labels. History filter, Totals breakdown, and history
# rows all map wire-format `kind` strings to these labels.
kind-copy = कॉपी
kind-move = स्थानांतरण
kind-delete = हटाएँ
kind-secure-delete = सुरक्षित रूप से हटाएँ

# History status labels. Distinct from `state-*` because the
# history wire format uses plain `running`/`succeeded`/... rather
# than the live `JobState` enum the queue exposes.
status-running = चल रहा है
status-succeeded = सफल
status-failed = विफल
status-cancelled = रद्द किया गया
# Per-item status (not job-level): `ok` / `skipped` are only
# reachable from the history detail view.
status-ok = ठीक
status-skipped = छोड़ा गया

# History drawer: search field placeholder + purge toast.
history-search-placeholder = /path
toast-history-purged = 30 दिन से पुराने { $count } कार्य हटाए गए

# User-facing command-layer validation errors. The Rust side
# returns these keys when input is missing; the toast layer looks
# them up rather than showing the raw English.
err-source-required = कम से कम एक स्रोत पथ आवश्यक है।
err-destination-empty = गंतव्य पथ खाली है।
err-source-empty = स्रोत पथ खाली है।

# Localised duration formatting for the Totals drawer and ETA
# fields. `{ $ms }`, `{ $s }`, `{ $m }`, `{ $h }` are integer
# placeables — the formatter passes pre-computed values in.
duration-lt-1s = < 1s
duration-ms = { $ms } ms
duration-seconds = { $s }s
duration-minutes-seconds = { $m }m { $s }s
duration-hours-minutes = { $h }h { $m }m
duration-zero = 0s

# Rate unit. Appended to a formatted byte size. Some languages
# render this with a leading space ("Ko/s"); keep it translatable
# even though the SI-derived "/s" is near-universal.
rate-unit-per-second = { $size }/s

# Phase 11b — Settings modal skeleton. Phase 12 expanded this into a
# full six-tab preferences window; the `settings-phase-12-hint` key
# is retired in favour of `settings-tab-profiles` + concrete labels.
settings-title = सेटिंग्स
settings-tab-general = सामान्य
settings-tab-appearance = दिखावट
settings-section-language = भाषा
settings-phase-12-hint = और सेटिंग्स (थीम, स्थानांतरण डिफ़ॉल्ट, सत्यापन एल्गोरिदम, प्रोफ़ाइल) Phase 12 में आएँगी।

# Phase 12 — full Settings window keys. Grouped by tab so the
# human-review pass can pick one section at a time. Every field
# label and every option label that the user will actually read
# has its own key; dynamic wire values (enum discriminants like
# "auto" / "prefer") stay untranslated on the wire.

settings-loading = सेटिंग्स लोड हो रही हैं…
settings-tab-transfer = स्थानांतरण
settings-tab-filters = फ़िल्टर
settings-tab-shell = शेल
settings-tab-secure-delete = सुरक्षित रूप से हटाना
settings-tab-advanced = उन्नत
settings-tab-updater = अपडेट
settings-tab-profiles = प्रोफ़ाइल

# General tab additions
settings-section-theme = थीम
settings-theme-auto = स्वतः
settings-theme-light = हल्की
settings-theme-dark = गहरी
settings-start-with-os = सिस्टम स्टार्टअप पर लॉन्च करें
settings-single-instance = एकल चालू इंस्टेंस
settings-minimize-to-tray = बंद करने पर ट्रे में मिनिमाइज़ करें
settings-error-display-mode = त्रुटि प्रॉम्प्ट शैली
settings-error-display-modal = मोडल (ऐप को रोकता है)
settings-error-display-drawer = ड्रॉअर (बिना रुकावट)
settings-error-display-mode-hint = मोडल आपके निर्णय लेने तक कतार रोक देता है। ड्रॉअर कतार को चलता रखता है और आपको कोने में त्रुटियों को सुलझाने देता है।
settings-paste-shortcut = ग्लोबल शॉर्टकट के ज़रिए फ़ाइलें पेस्ट करें
settings-paste-shortcut-combo = शॉर्टकट कॉम्बो
settings-paste-shortcut-hint = Explorer / Finder / Files से कॉपी की गई फ़ाइलों को Copy That के ज़रिए पेस्ट करने के लिए अपने सिस्टम पर कहीं भी यह कॉम्बो दबाएँ। CmdOrCtrl macOS पर Cmd और Windows / Linux पर Ctrl में बदल जाता है।
settings-clipboard-watcher = कॉपी की गई फ़ाइलों के लिए क्लिपबोर्ड पर नज़र रखें
settings-clipboard-watcher-hint = जब क्लिपबोर्ड पर फ़ाइल URL दिखें तो एक टोस्ट दिखाएँ, यह संकेत देते हुए कि आप Copy That के ज़रिए पेस्ट कर सकते हैं। सक्षम रहने पर हर 500 ms पर जाँच करता है।

# Transfer tab
settings-buffer-size = बफ़र आकार
settings-verify = कॉपी के बाद सत्यापित करें
settings-verify-off = बंद
settings-concurrency = समवर्तीयता
settings-concurrency-auto = स्वतः
settings-reflink = Reflink / तेज़ पथ
settings-reflink-prefer = प्राथमिकता दें
settings-reflink-avoid = reflink से बचें
settings-reflink-disabled = हमेशा async इंजन का उपयोग करें
settings-fsync-on-close = बंद करने पर डिस्क पर सिंक करें (धीमा, सुरक्षित)
settings-preserve-timestamps = टाइमस्टैम्प संरक्षित करें
settings-preserve-permissions = अनुमतियाँ संरक्षित करें
settings-preserve-acls = ACLs संरक्षित करें (Phase 14)
settings-preserve-sparseness = स्पार्स फ़ाइलें संरक्षित करें
settings-preserve-sparseness-hint = स्पार्स फ़ाइलों (VM डिस्क, डेटाबेस फ़ाइलें) के केवल आवंटित एक्सटेंट कॉपी करें ताकि गंतव्य का ऑन-डिस्क आकार स्रोत के समान रहे।

# Shell tab
settings-context-menu = शेल कॉन्टेक्स्ट मेनू प्रविष्टियाँ सक्षम करें
settings-intercept-copy = डिफ़ॉल्ट कॉपी हैंडलर इंटरसेप्ट करें (Windows)
settings-intercept-copy-hint = चालू होने पर, Explorer का Ctrl+C / Ctrl+V Copy That के ज़रिए होकर गुज़रता है। पंजीकरण Phase 14 में आएगा।
settings-notify-completion = कार्य पूर्ण होने पर सूचित करें

# Secure delete tab
settings-shred-method = डिफ़ॉल्ट श्रेड विधि
settings-shred-zero = शून्य (1 पास)
settings-shred-random = रैंडम (1 पास)
settings-shred-dod3 = DoD 5220.22-M (3 पास)
settings-shred-dod7 = DoD 5220.22-M (7 पास)
settings-shred-gutmann = Gutmann (35 पास)
settings-shred-nist = NIST 800-88
settings-shred-confirm-twice = श्रेड करने से पहले दोहरी पुष्टि आवश्यक करें

# Advanced tab
settings-log-level = लॉग स्तर
settings-log-off = बंद
settings-telemetry = टेलीमेट्री
settings-telemetry-never = कभी नहीं — किसी भी लॉग स्तर पर कोई फ़ोन-होम नहीं
settings-error-policy = डिफ़ॉल्ट त्रुटि नीति
settings-error-policy-ask = पूछें
settings-error-policy-skip = छोड़ें
settings-error-policy-retry = बैकऑफ़ के साथ पुनः प्रयास करें
settings-error-policy-abort = पहली विफलता पर रद्द करें
settings-history-retention = इतिहास प्रतिधारण (दिन)
settings-history-retention-hint = 0 = हमेशा रखें। कोई अन्य मान स्टार्टअप पर पुराने कार्यों को स्वतः हटा देता है।
settings-database-path = डेटाबेस पथ
settings-database-path-default = (डिफ़ॉल्ट — OS डेटा डायरेक्टरी)
settings-reset-all = डिफ़ॉल्ट पर रीसेट करें
settings-reset-confirm = हर वरीयता को उसके डिफ़ॉल्ट पर रीसेट करें? प्रोफ़ाइल अप्रभावित रहती हैं।

# Profiles tab
settings-profiles-hint = वर्तमान सेटिंग्स को एक नाम के तहत सहेजें; अलग-अलग नॉब को छुए बिना वापस लौटने के लिए बाद में इसे लोड करें।
settings-profile-name-placeholder = प्रोफ़ाइल नाम
settings-profile-save = सहेजें
settings-profile-import = आयात करें…
settings-profile-load = लोड करें
settings-profile-export = निर्यात करें…
settings-profile-delete = हटाएँ
settings-profile-empty = अभी तक कोई प्रोफ़ाइल सहेजी नहीं गई।
settings-profile-import-prompt = आयातित प्रोफ़ाइल का नाम:

# Toasts driven by Phase 12 profile actions
toast-settings-reset = सेटिंग्स रीसेट की गईं
toast-profile-saved = प्रोफ़ाइल सहेजी गई
toast-profile-loaded = प्रोफ़ाइल लोड की गई
toast-profile-exported = प्रोफ़ाइल निर्यात की गई
toast-profile-imported = प्रोफ़ाइल आयात की गई

# Phase 14a — enumeration-time filters
settings-filters-hint = गणना के समय फ़ाइलें छोड़ें ताकि इंजन उन्हें कभी खोले ही नहीं। इन्क्लूड केवल फ़ाइलों पर लागू होते हैं; एक्सक्लूड मिलते-जुलते डायरेक्टरी को भी काट देते हैं।
settings-filters-enabled = ट्री कॉपी के लिए फ़िल्टर सक्षम करें
settings-filters-include-globs = इन्क्लूड ग्लोब्स
settings-filters-include-globs-placeholder = **/*.txt
settings-filters-include-globs-hint = प्रति पंक्ति एक ग्लोब। गैर-खाली होने पर, बचे रहने के लिए फ़ाइल को कम से कम एक इन्क्लूड से मिलना ज़रूरी है। डायरेक्टरी में हमेशा प्रवेश किया जाता है।
settings-filters-exclude-globs = एक्सक्लूड ग्लोब्स
settings-filters-exclude-globs-placeholder = **/node_modules
settings-filters-exclude-globs-hint = प्रति पंक्ति एक ग्लोब। मेल खाने पर डायरेक्टरी के पूरे सबट्री को काट देता है; मेल खाने वाली फ़ाइलें छोड़ दी जाती हैं।
settings-filters-size-range = फ़ाइल आकार सीमा
settings-filters-min-size-bytes = न्यूनतम आकार (बाइट्स, खाली = कोई न्यूनतम नहीं)
settings-filters-max-size-bytes = अधिकतम आकार (बाइट्स, खाली = कोई अधिकतम नहीं)
settings-filters-date-range = संशोधन समय सीमा
settings-filters-min-mtime = इस दिन या उसके बाद संशोधित
settings-filters-max-mtime = इस दिन या उससे पहले संशोधित
settings-filters-attributes = एट्रिब्यूट बिट्स
settings-filters-skip-hidden = छिपी हुई फ़ाइलें / फ़ोल्डर छोड़ें
settings-filters-skip-system = सिस्टम फ़ाइलें छोड़ें (केवल Windows)
settings-filters-skip-readonly = केवल-पढ़ने योग्य फ़ाइलें छोड़ें

# Phase 15 — auto-update
settings-updater-hint = Copy That दिन में अधिकतम एक बार साइन किए गए अपडेट की जाँच करता है। अपडेट अगली बार ऐप बंद होने पर इंस्टॉल होते हैं।
settings-updater-auto-check = लॉन्च पर अपडेट की जाँच करें
settings-updater-channel = रिलीज़ चैनल
settings-updater-channel-stable = स्थिर
settings-updater-channel-beta = बीटा (प्री-रिलीज़)
settings-updater-last-check = अंतिम जाँच
settings-updater-last-never = कभी नहीं
settings-updater-check-now = अभी अपडेट की जाँच करें
settings-updater-checking = जाँच हो रही है…
settings-updater-available = अपडेट उपलब्ध है
settings-updater-up-to-date = आप नवीनतम रिलीज़ चला रहे हैं।
settings-updater-dismiss = इस संस्करण को छोड़ें
settings-updater-dismissed = छोड़ा गया
toast-update-available = एक नया संस्करण उपलब्ध है
toast-update-up-to-date = आप पहले से नवीनतम संस्करण पर हैं

# Phase 19a — disk-backed file enumeration (TeraCopy-compatible scan DB)
scan-progress-title = स्कैन हो रहा है…
scan-progress-stats = { $files } फ़ाइलें · अब तक { $bytes }
scan-pause-button = स्कैन रोकें
scan-resume-button = स्कैन जारी रखें
scan-cancel-button = स्कैन रद्द करें
scan-cancel-confirm = स्कैन रद्द करें और प्रगति त्यागें?
scan-db-header = स्कैन डेटाबेस
scan-db-hint = कई-मिलियन-फ़ाइल कार्यों के लिए ऑन-डिस्क स्कैन डेटाबेस।
advanced-scan-hash-during = स्कैन के दौरान चेकसम की गणना करें
advanced-scan-db-path = स्कैन डेटाबेस स्थान
advanced-scan-retention-days = पूर्ण किए गए स्कैन इतने दिनों बाद स्वतः हटाएँ (दिन)
advanced-scan-max-keep = रखने के लिए अधिकतम स्कैन डेटाबेस

# Phase 19b — filesystem-snapshot source for locked files.
settings-on-locked = जब कोई फ़ाइल लॉक हो
settings-on-locked-ask = पहली बार पूछें
settings-on-locked-retry = कुछ देर पुनः प्रयास करें, फिर त्रुटि दिखाएँ
settings-on-locked-skip = लॉक की गई फ़ाइल छोड़ें
settings-on-locked-snapshot = फ़ाइलसिस्टम स्नैपशॉट का उपयोग करें
settings-on-locked-hint = "फ़ाइल किसी अन्य प्रोसेस द्वारा उपयोग में है" त्रुटियों को समाप्त करें। Copy That स्रोत वॉल्यूम का स्नैपशॉट लेता है (Windows पर VSS, Linux पर ZFS/Btrfs, macOS पर APFS) और स्नैपशॉट कॉपी से पढ़ता है।
snapshot-prompt-title = यह फ़ाइल किसी अन्य प्रोसेस द्वारा उपयोग में है
snapshot-prompt-body = एक अन्य प्रोग्राम ने { $path } को एक्सक्लूसिव राइट के लिए खोल रखा है। चुनें कि Copy That को इस तथा उसी वॉल्यूम पर ऐसी ही फ़ाइलों को कैसे संभालना चाहिए।
snapshot-source-active = 📷 { $volume } के { $kind } स्नैपशॉट से पढ़ा जा रहा है
snapshot-create-failed = स्रोत वॉल्यूम का स्नैपशॉट नहीं बनाया जा सका
snapshot-vss-needs-elevation = VSS स्नैपशॉट से पढ़ने के लिए Administrator अनुमति आवश्यक है। Copy That आपसे इसकी अनुमति माँगेगा।
snapshot-cleanup-failed = स्नैपशॉट हेल्पर ने सफ़ाई विफलता की सूचना दी — वॉल्यूम पर एक बची हुई शैडो कॉपी रह सकती है।

# Phase 20 — durable resume journal.
resume-prompt-title = पिछले स्थानांतरण जारी रखें?
resume-prompt-body = Copy That ने पिछले सत्र से { $count } अधूरे स्थानांतरण पाए। प्रत्येक के साथ क्या करना है चुनें।
resume-prompt-resume = जारी रखें
resume-prompt-resume-all = सभी जारी रखें
resume-discard-one = जारी न रखें
resume-discard-all = सभी त्यागें
resume-aborted-hash-mismatch = गंतव्य के पहले { $offset } बाइट्स स्रोत से मेल नहीं खाते — शुरुआत से फिर से आरंभ कर रहे हैं।
settings-auto-resume = बिना पूछे बाधित कार्य स्वतः जारी रखें
settings-auto-resume-hint = स्टार्टअप पर रिज़्यूम प्रॉम्प्ट छोड़ें और हर अधूरे कार्य को चुपचाप फिर से कतार में लगाएँ। डिफ़ॉल्ट रूप से बंद।

# Phase 21 — bandwidth shaping (GCRA token bucket + schedule + auto-throttle).
settings-tab-network = नेटवर्क
settings-network-hint = बाकी नेटवर्क को उपयोग योग्य रखने के लिए अपनी स्थानांतरण दर सीमित करें। इसे ग्लोबल रूप से लागू करें, दैनिक शेड्यूल का पालन करें, या मीटर्ड Wi-Fi / बैटरी / सेल्युलर कनेक्शन पर स्वतः प्रतिक्रिया दें।
settings-network-mode = बैंडविड्थ सीमा
settings-network-mode-off = बंद (कोई सीमा नहीं)
settings-network-mode-fixed = निश्चित मान
settings-network-mode-schedule = शेड्यूल का उपयोग करें
settings-network-cap-mbps = सीमा (MB/s)
settings-network-schedule = शेड्यूल (rclone प्रारूप)
settings-network-schedule-hint = स्पेस से अलग किए गए HH:MM,rate सीमांक और वैकल्पिक Mon-Fri,rate दिन नियम। दरें: 512k, 10M, 2G, off, unlimited। उदाहरण: 08:00,512k 18:00,10M Sat-Sun,unlimited।
settings-network-auto-header = स्वतः-थ्रॉटल
settings-network-auto-metered = मीटर्ड Wi-Fi पर
settings-network-auto-battery = बैटरी पर
settings-network-auto-cellular = सेल्युलर पर
settings-network-auto-unchanged = ओवरराइड न करें
settings-network-auto-pause = स्थानांतरण रोकें
settings-network-auto-cap = निश्चित मान तक सीमित करें
shape-badge-paused = रोका गया
shape-badge-tooltip = बैंडविड्थ सीमा सक्रिय — Settings → Network खोलने के लिए क्लिक करें
shape-badge-source-schedule = शेड्यूल्ड
shape-badge-source-metered = मीटर्ड
shape-badge-source-battery = बैटरी पर
shape-badge-source-cellular = सेल्युलर
shape-badge-source-settings = सक्रिय
shape-error-schedule-invalid = शेड्यूल प्रारूप मान्य नहीं है: { $message }

# Phase 22 — aggregate conflict dialog v2 (thumbnails, per-pattern
# rules, and reusable conflict profiles). Every key below is user-
# visible text in the `ConflictBatchModal.svelte` component.
conflict-batch-title = { $jobname } में { $count } फ़ाइल टकराव
conflict-batch-state-pending = प्रतीक्षारत
conflict-batch-state-resolved = हल किया गया
conflict-batch-action-overwrite = ओवरराइट करें
conflict-batch-action-skip = छोड़ें
conflict-batch-action-keep-both = दोनों रखें
conflict-batch-action-newer-wins = नई जीतती है
conflict-batch-action-larger-wins = बड़ी जीतती है
conflict-batch-bulk-apply-selected = चयनित पर लागू करें
conflict-batch-bulk-apply-extension = इस एक्सटेंशन की सभी पर लागू करें
conflict-batch-bulk-apply-glob = मेल खाने वाले ग्लोब पर लागू करें…
conflict-batch-bulk-apply-remaining = सभी शेष पर लागू करें
conflict-batch-bulk-glob-placeholder = उदा. **/*.tmp
conflict-batch-save-profile = इन नियमों को प्रोफ़ाइल के रूप में सहेजें…
conflict-batch-profile-placeholder = प्रोफ़ाइल नाम
conflict-batch-matched-rule = नियम '{ $rule }' के ज़रिए → { $action }
conflict-batch-empty = सभी टकराव हल हो गए
conflict-batch-source-vs-destination = स्रोत बनाम गंतव्य
conflict-batch-source-label = स्रोत
conflict-batch-destination-label = गंतव्य
conflict-batch-size-label = आकार
conflict-batch-modified-label = संशोधित
conflict-batch-close = बंद करें
conflict-batch-profile-saved = टकराव प्रोफ़ाइल सहेजी गई

# Phase 23 — sparse-file preservation. The toast fires once per
# destination volume when the filesystem can't preserve holes; the
# warning line is surfaced in the job detail drawer so the user knows
# the dst is larger on disk than the source was.
sparse-not-supported-title = गंतव्य स्पार्स फ़ाइलों को भर देता है
sparse-not-supported-body = { $dst_fs } स्पार्स फ़ाइलों का समर्थन नहीं करता। स्रोत के होल्स शून्य के रूप में लिखे गए, इसलिए गंतव्य डिस्क पर बड़ा है।
sparse-warning-densified = स्पार्स लेआउट संरक्षित: केवल आवंटित एक्सटेंट कॉपी किए गए।
sparse-warning-mismatch = स्पार्स लेआउट बेमेल — गंतव्य अपेक्षा से बड़ा हो सकता है।

# Phase 24 — security-metadata preservation. The Mark-of-the-Web
# (Zone.Identifier ADS) toggle is security-sensitive: turning it off
# lets a downloaded executable shed its SmartScreen / Office Protected
# View flag on copy, which is why the tooltip carries an explicit
# warning. AppleDouble fallback emits `._<filename>` sidecars on
# destination filesystems that can't hold the foreign metadata.
settings-preserve-security-metadata = सुरक्षा मेटाडेटा संरक्षित करें
settings-preserve-security-metadata-hint = हर कॉपी पर आउट-ऑफ़-बैंड मेटाडेटा स्ट्रीम (NTFS ADS / xattrs / POSIX ACLs / SELinux कॉन्टेक्स्ट / Linux फ़ाइल कैपेबिलिटीज़ / macOS रिसोर्स फ़ोर्क्स) कैप्चर करें और दोबारा लागू करें।
settings-preserve-motw = Mark-of-the-Web संरक्षित करें (इंटरनेट-से-डाउनलोड किया गया फ़्लैग)
settings-preserve-motw-hint = सुरक्षा के लिए अत्यंत महत्वपूर्ण। SmartScreen और Office Protected View इंटरनेट से डाउनलोड की गई फ़ाइलों के बारे में चेतावनी देने के लिए इस स्ट्रीम का उपयोग करते हैं। इसे अक्षम करने से डाउनलोड किया गया executable कॉपी पर अपना मूल चिह्न खो देता है और ऑपरेटिंग-सिस्टम सुरक्षा उपायों को दरकिनार कर देता है।
settings-preserve-posix-acls = POSIX ACLs और एक्सटेंडेड एट्रिब्यूट संरक्षित करें
settings-preserve-posix-acls-hint = कॉपी के दौरान user.* / system.* / trusted.* xattrs और POSIX एक्सेस-कंट्रोल सूचियाँ साथ ले जाएँ।
settings-preserve-selinux = SELinux कॉन्टेक्स्ट संरक्षित करें
settings-preserve-selinux-hint = कॉपी के दौरान security.selinux लेबल साथ ले जाएँ ताकि MAC नीतियों के तहत चल रहे डेमॉन अभी भी फ़ाइल तक पहुँच सकें।
settings-preserve-resource-forks = macOS रिसोर्स फ़ोर्क्स और Finder जानकारी संरक्षित करें
settings-preserve-resource-forks-hint = कॉपी के दौरान लीगेसी रिसोर्स फ़ोर्क और FinderInfo (रंग टैग, Carbon मेटाडेटा) साथ ले जाएँ।
settings-appledouble-fallback = असंगत फ़ाइलसिस्टम पर AppleDouble साइडकार का उपयोग करें
meta-translated-to-appledouble = विदेशी मेटाडेटा AppleDouble साइडकार में संग्रहित (._{ $ext })

# Phase 25 — two-way sync with vector-clock conflict detection.
# The drawer lists configured sync pairs; each pair runs an
# independent reconciliation round against a per-pair `.copythat-sync.db`
# state store. Concurrent edits from a common ancestor surface as
# conflicts rather than silent overwrites; the losing side's content
# is preserved as `name.sync-conflict-YYYYMMDD-HHMMSS-<host>.ext`.
footer-sync = सिंक
sync-drawer-title = दोतरफ़ा सिंक
sync-drawer-hint = दो फ़ोल्डरों को बिना चुपचाप ओवरराइट किए सिंक रखें। एक साथ हुए संपादन ऐसे टकराव के रूप में दिखते हैं जिन्हें आप हल कर सकते हैं।
sync-add-pair = जोड़ी जोड़ें
sync-add-cancel = रद्द करें
sync-refresh = रिफ़्रेश करें
sync-add-save = जोड़ी सहेजें
sync-add-saving = सहेजा जा रहा है…
sync-add-missing-fields = लेबल, बायाँ पथ, और दायाँ पथ सभी आवश्यक हैं।
sync-remove-confirm = यह सिंक जोड़ी हटाएँ? स्थिति डेटाबेस संरक्षित रहता है; फ़ोल्डर अछूते रहते हैं।
sync-field-label = लेबल
sync-field-label-placeholder = उदा. Documents ↔ NAS
sync-field-left = बायाँ फ़ोल्डर
sync-field-left-placeholder = कोई पूर्ण पथ चुनें या पेस्ट करें
sync-field-right = दायाँ फ़ोल्डर
sync-field-right-placeholder = कोई पूर्ण पथ चुनें या पेस्ट करें
sync-field-mode = मोड
sync-mode-two-way = दोतरफ़ा
sync-mode-mirror-left-to-right = मिरर (बायाँ → दायाँ)
sync-mode-mirror-right-to-left = मिरर (दायाँ → बायाँ)
sync-mode-contribute-left-to-right = योगदान (बायाँ → दायाँ, कोई विलोपन नहीं)
sync-no-pairs = अभी तक कोई सिंक जोड़ी कॉन्फ़िगर नहीं की गई। शुरू करने के लिए "जोड़ी जोड़ें" पर क्लिक करें।
sync-loading = कॉन्फ़िगर की गई जोड़ियाँ लोड हो रही हैं…
sync-never-run = कभी नहीं चली
sync-running = चल रही है
sync-run-now = अभी चलाएँ
sync-cancel = रद्द करें
sync-remove-pair = हटाएँ
sync-view-conflicts = टकराव देखें ({ $count })
sync-conflicts-heading = टकराव
sync-no-conflicts = पिछली बार चलने से कोई टकराव नहीं।
sync-winner = विजेता
sync-side-left-to-right = बायाँ
sync-side-right-to-left = दायाँ
sync-conflict-kind-concurrent-write = एक साथ हुआ संपादन
sync-conflict-kind-delete-edit = विलोपन ↔ संपादन
sync-conflict-kind-add-add = दोनों ओर जोड़ा गया
sync-conflict-kind-corrupt-equal = बिना नए राइट के सामग्री भिन्न हुई
sync-resolve-keep-left = बायाँ रखें
sync-resolve-keep-right = दायाँ रखें
sync-resolve-keep-both = दोनों रखें
sync-resolve-three-way = 3-तरफ़ा मर्ज के ज़रिए हल करें
sync-resolve-phase-53-tooltip = गैर-टेक्स्ट फ़ाइलों के लिए इंटरैक्टिव 3-तरफ़ा मर्ज Phase 53 में आएगा।
sync-error-prefix = सिंक त्रुटि

# Phase 26 — real-time mirror watcher. "Live mirror" starts a
# filesystem watcher on the pair's left side; every debounced event
# triggers a re-sync. The watcher filters vim swap files / Office
# lock files / atomic-save temp names so one logical save becomes
# exactly one sync round.
live-mirror-start = लाइव मिरर शुरू करें
live-mirror-stop = लाइव मिरर रोकें
live-mirror-watching = नज़र रखी जा रही है
live-mirror-toggle-hint = हर पहचाने गए फ़ाइलसिस्टम बदलाव पर स्वतः फिर से सिंक करें। प्रति सक्रिय जोड़ी एक बैकग्राउंड थ्रेड।
watch-event-prefix = फ़ाइल बदलाव
watch-overflow-recovered = वॉचर बफ़र ओवरफ़्लो हो गया; पुनः प्राप्ति के लिए फिर से गणना की जा रही है

# Phase 27 — content-defined chunk store. Enables delta-resume (a
# retry only re-writes chunks that actually changed) and same-job
# dedup (files sharing content blocks store those blocks once). The
# store is disk-backed under `<data-dir>/chunks/` by default and is
# the foundation for the Phase 49–51 moonshot repository phases.
chunk-store-section = चंक स्टोर
chunk-store-enable = चंक स्टोर सक्षम करें (डेल्टा-रिज़्यूम और डीडुप)
chunk-store-enable-hint = हर कॉपी की गई फ़ाइल को सामग्री के अनुसार विभाजित करता है (FastCDC) और चंक्स को कंटेंट-एड्रेस्ड संग्रहित करता है। पुनः प्रयास केवल बदले गए चंक्स फिर से लिखते हैं; साझा सामग्री वाली फ़ाइलें स्वतः डीडुप हो जाती हैं।
chunk-store-location = चंक स्टोर स्थान
chunk-store-max-size = अधिकतम चंक स्टोर आकार
chunk-store-prune = इतने दिनों से पुराने चंक्स हटाएँ (दिन)
chunk-store-savings = चंक डीडुप के ज़रिए { $gib } GiB बचाए गए
chunk-store-disk-usage = { $chunks } चंक्स में { $size } का उपयोग

# Phase 28 — tray-resident Drop Stack. The stack is a persistent list
# of paths gathered from multiple sources (Explorer drag, main-window
# context menu, CLI, drag onto the Drop Stack window) that the user
# can dispatch to a destination in one go.
dropstack-window-title = Drop Stack
dropstack-tray-open = Drop Stack
dropstack-empty-title = Drop Stack खाली है
dropstack-empty-hint = Explorer से फ़ाइलें यहाँ खींचें या किसी कार्य पंक्ति पर राइट-क्लिक करके उसे जोड़ें।
dropstack-add-to-stack = Drop Stack में जोड़ें
dropstack-copy-all-to = सभी को यहाँ कॉपी करें…
dropstack-move-all-to = सभी को यहाँ स्थानांतरित करें…
dropstack-clear = स्टैक साफ़ करें
dropstack-remove-row = स्टैक से हटाएँ
dropstack-path-missing-toast = { $path } छोड़ा गया — फ़ाइल अब मौजूद नहीं है।
dropstack-always-on-top = Drop Stack को हमेशा सबसे ऊपर रखें
dropstack-show-tray-icon = Copy That ट्रे आइकन दिखाएँ
dropstack-open-on-start = ऐप शुरू होने पर Drop Stack स्वतः खोलें
dropstack-count = { $count } पथ

# Phase 29 — spring-loaded folders + native DnD polish. The Settings
# → General tab carries the knobs; DropTarget / DestinationPicker
# Svelte components use the dropzone-* keys at runtime.
settings-dnd-heading = ड्रैग और ड्रॉप
settings-dnd-spring-load = खींचते समय फ़ोल्डर स्प्रिंग-लोड करें
settings-dnd-spring-delay = स्प्रिंग-लोड विलंब (ms)
settings-dnd-thumbnails = ड्रैग थंबनेल दिखाएँ
settings-dnd-invalid-highlight = अमान्य ड्रॉप लक्ष्यों को हाइलाइट करें
dropzone-invalid-title = यह मान्य ड्रॉप लक्ष्य नहीं है
dropzone-invalid-readonly = गंतव्य केवल-पढ़ने योग्य है
dropzone-picker-title = एक गंतव्य चुनें
dropzone-picker-up = ऊपर
dropzone-picker-path = वर्तमान पथ
dropzone-picker-root = रूट्स
dropzone-picker-use-this = यह फ़ोल्डर उपयोग करें
dropzone-picker-empty = कोई सबफ़ोल्डर नहीं
dropzone-picker-cancel = रद्द करें

# Phase 30 — cross-platform path translation. Settings → Transfer
# exposes these under a "Cross-platform compatibility" subsection.
translate-heading = क्रॉस-प्लेटफ़ॉर्म संगतता
translate-unicode-label = Unicode नॉर्मलाइज़ेशन
translate-unicode-auto = गंतव्य स्वतः पहचानें
translate-unicode-windows = NFC (Windows / Linux)
translate-unicode-macos = जैसा है वैसा रखें (macOS / APFS)
translate-line-endings-label = टेक्स्ट फ़ाइलों के लिए लाइन एंडिंग बदलें
translate-line-endings-allowlist = टेक्स्ट फ़ाइल एक्सटेंशन
reserved-name-label = Windows आरक्षित-नाम प्रबंधन
reserved-name-suffix = "_" जोड़ें (CON.txt → CON_.txt)
reserved-name-reject = अस्वीकार करें और चेतावनी दें
long-path-label = 260 वर्णों से अधिक होने पर Windows लॉन्ग-पथ प्रीफ़िक्स (\\?\) का उपयोग करें
long-path-hint = कुछ नेटवर्क शेयर और लीगेसी टूल \\?\ नेमस्पेस का सम्मान नहीं करते।

# Phase 31 — power-aware copying. Settings → Power & State tab + the
# header badge that renders "⏸ Paused — Zoom call detected" when the
# runner's power subscriber has paused or capped due to a policy match.
power-heading = पावर और स्थिति
power-enabled = पावर-सजग नियम सक्षम करें
power-battery-label = बैटरी पर
power-metered-label = मीटर्ड Wi-Fi पर
power-cellular-label = सेल्युलर पर
power-presentation-label = प्रस्तुति देते समय (Zoom / Teams / Keynote)
power-fullscreen-label = जब कोई ऐप फ़ुलस्क्रीन हो
power-thermal-label = जब CPU थर्मल-थ्रॉटलिंग कर रहा हो
power-rule-continue = पूरी गति से जारी रखें
power-rule-pause = सभी कार्य रोकें
power-rule-cap = बैंडविड्थ सीमित करें
power-rule-cap-percent = वर्तमान दर के एक प्रतिशत तक सीमित करें
power-reason-on-battery = बैटरी पर
power-reason-metered-network = मीटर्ड नेटवर्क
power-reason-cellular-network = सेल्युलर नेटवर्क
power-reason-presenting = प्रस्तुति मोड
power-reason-fullscreen = फ़ुलस्क्रीन ऐप
power-reason-thermal-throttling = CPU थ्रॉटलिंग कर रहा है

# Phase 32 — cloud backend matrix via OpenDAL. Settings → Remotes
# tab + the Add-backend wizard that writes one entry per remote into
# the keychain-backed credential store.
remote-heading = रिमोट बैकएंड
remote-add = बैकएंड जोड़ें
remote-list-empty = कोई रिमोट बैकएंड कॉन्फ़िगर नहीं किया गया
remote-test = कनेक्शन जाँचें
remote-test-success = कनेक्शन सफल
remote-test-failed = कनेक्शन विफल
remote-remove = बैकएंड हटाएँ
remote-name-label = प्रदर्शन नाम
remote-kind-label = बैकएंड प्रकार
remote-save = बैकएंड सहेजें
remote-cancel = रद्द करें
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
backend-local-fs = स्थानीय फ़ाइलसिस्टम
cloud-config-bucket = बकेट
cloud-config-region = रीजन
cloud-config-endpoint = एंडपॉइंट URL
cloud-config-root = रूट पथ
cloud-error-invalid-config = बैकएंड कॉन्फ़िगरेशन अमान्य है
cloud-error-network = बैकएंड से संपर्क करते समय नेटवर्क त्रुटि
cloud-error-not-found = अनुरोधित पथ पर ऑब्जेक्ट नहीं मिला
cloud-error-permission = रिमोट बैकएंड द्वारा अनुमति अस्वीकृत
cloud-error-keychain = OS कीचेन एक्सेस विफल
settings-tab-remotes = रिमोट्स
settings-tab-mobile = मोबाइल

# Phase 33 — mount Copy That's chunk store + history archive as a
# read-only filesystem (FUSE / WinFsp). Surfaced by the History tab
# context menu's "Mount snapshot" action + the Settings → Advanced
# "Mount latest on launch" toggle.
mount-heading = स्नैपशॉट माउंट करें
mount-action-mount = स्नैपशॉट माउंट करें
mount-action-unmount = अनमाउंट करें
mount-status-mounted = { $path } पर माउंट किया गया
mount-error-unsafe-mountpoint = माउंटपॉइंट पथ असुरक्षित है
mount-error-mountpoint-not-empty = माउंटपॉइंट एक खाली डायरेक्टरी होनी चाहिए
mount-error-backend-unavailable = इस सिस्टम पर माउंट बैकएंड उपलब्ध नहीं है
mount-error-archive-read = आर्काइव पढ़ना विफल हुआ
mount-picker-title = माउंटपॉइंट डायरेक्टरी चुनें
mount-toast-mounted = स्नैपशॉट { $path } पर माउंट किया गया
mount-toast-unmounted = स्नैपशॉट अनमाउंट किया गया
mount-toast-failed = माउंट विफल: { $reason }
settings-mount-heading = स्नैपशॉट माउंट करें
settings-mount-hint = इतिहास आर्काइव को केवल-पढ़ने योग्य फ़ाइलसिस्टम के रूप में उजागर करें। Phase 33b रनर फ़्लो जोड़ता है; कर्नेल FUSE/WinFsp बैकएंड Phase 33c में आते हैं।
settings-mount-on-launch = लॉन्च पर नवीनतम स्नैपशॉट माउंट करें
settings-mount-on-launch-path = माउंटपॉइंट पथ
settings-mount-on-launch-path-placeholder = उदा. C:\Mounts\copythat

# Phase 34 — enterprise-grade audit log export + WORM (write-once-read-
# many) tamper-resistance. Settings → Advanced → Audit log collects the
# format + destination + rotation + WORM toggle; runtime records job and
# file events into the tamper-evident chain-hashed sink.
settings-audit-heading = ऑडिट लॉग
settings-audit-hint = हर कार्य और फ़ाइल इवेंट का केवल-जोड़ने योग्य छेड़छाड़-स्पष्ट लॉग। प्रारूपों में CSV, JSON-lines, RFC 5424 Syslog, ArcSight CEF, और QRadar LEEF शामिल हैं।
settings-audit-enable = ऑडिट लॉगिंग सक्षम करें
settings-audit-format = लॉग प्रारूप
settings-audit-format-json-lines = JSON lines (अनुशंसित डिफ़ॉल्ट)
settings-audit-format-csv = CSV (स्प्रेडशीट-अनुकूल)
settings-audit-format-syslog = Syslog (RFC 5424)
settings-audit-format-cef = CEF (ArcSight)
settings-audit-format-leef = LEEF 2.0 (IBM QRadar)
settings-audit-file-path = लॉग फ़ाइल पथ
settings-audit-file-path-placeholder = उदा. C:\ProgramData\CopyThat\audit.log
settings-audit-max-size = इतने बाद रोटेट करें (बाइट्स, 0 = कभी नहीं)
settings-audit-worm = WORM मोड सक्षम करें (write-once-read-many)
settings-audit-worm-hint = हर निर्माण या रोटेशन के बाद प्लेटफ़ॉर्म का केवल-जोड़ने योग्य फ़्लैग लागू करता है (Linux chattr +a, macOS chflags uappnd, Windows केवल-पढ़ने योग्य एट्रिब्यूट)। लॉग को छोटा करने के लिए एक एडमिनिस्ट्रेटर को भी फ़्लैग स्पष्ट रूप से हटाना ज़रूरी है।
settings-audit-test-write = परीक्षण राइट
settings-audit-verify-chain = चेन सत्यापित करें
toast-audit-test-write-ok = ऑडिट लॉग परीक्षण राइट सफल रहा
toast-audit-verify-ok = ऑडिट चेन अक्षुण्ण सत्यापित हुई
toast-audit-verify-failed = ऑडिट चेन सत्यापन में बेमेल पाए गए

# Phase 35 — destination encryption via age (X25519 / passphrase /
# SSH) + on-the-fly zstd compression with a per-extension deny
# heuristic. Surfaced through Settings → Transfer → Encryption +
# Compression; engine short-circuits to the crypt pipeline when
# either stage is active.
settings-crypt-heading = एन्क्रिप्शन और कम्प्रेशन
settings-crypt-hint = फ़ाइल सामग्री को गंतव्य पर पहुँचने से पहले रूपांतरित करें। एन्क्रिप्शन age प्रारूप का उपयोग करता है; कम्प्रेशन zstd का उपयोग करता है और एक्सटेंशन के आधार पर पहले से कम्प्रेस किए गए मीडिया को छोड़ सकता है।
settings-crypt-encryption-mode = एन्क्रिप्शन
settings-crypt-encryption-off = बंद
settings-crypt-encryption-passphrase = पासफ़्रेज़ (कॉपी शुरू होने पर पूछें)
settings-crypt-encryption-recipients = फ़ाइल से प्राप्तकर्ता कुंजियाँ
settings-crypt-encryption-hint = पासफ़्रेज़ केवल कॉपी की अवधि के लिए मेमोरी में रखे जाते हैं। प्राप्तकर्ता फ़ाइलें प्रति पंक्ति एक age1… या ssh- सार्वजनिक कुंजी सूचीबद्ध करती हैं।
settings-crypt-recipients-file = प्राप्तकर्ता फ़ाइल पथ
settings-crypt-recipients-file-placeholder = उदा. C:\Users\me\recipients.txt
settings-crypt-compression-mode = कम्प्रेशन
settings-crypt-compression-off = बंद
settings-crypt-compression-always = हमेशा
settings-crypt-compression-smart = स्मार्ट (पहले से कम्प्रेस किया गया मीडिया छोड़ें)
settings-crypt-compression-hint = स्मार्ट मोड jpg, mp4, zip, 7z और इसी तरह के प्रारूपों को छोड़ देता है जिन्हें zstd से लाभ नहीं होता। हमेशा मोड हर फ़ाइल को चुने गए स्तर पर कम्प्रेस करता है।
settings-crypt-compression-level = zstd स्तर (1-22)
settings-crypt-compression-level-hint = कम संख्या तेज़ है; अधिक संख्या ज़्यादा कम्प्रेस करती है। स्तर 3 zstd के CLI डिफ़ॉल्ट से मेल खाता है।
compress-footer-savings = 💾 { $original } → { $compressed } ({ $percent }% बचाया)
compress-savings-toast = { $percent }% कम्प्रेस किया ({ $bytes } बचाया)
crypt-toast-recipients-loaded = { $count } एन्क्रिप्शन प्राप्तकर्ता लोड किए गए
crypt-toast-recipients-error = प्राप्तकर्ता लोड करने में विफल: { $reason }
crypt-toast-passphrase-required = कॉपी शुरू होने से पहले एन्क्रिप्शन को एक पासफ़्रेज़ की आवश्यकता है
crypt-toast-passphrase-set = एन्क्रिप्शन पासफ़्रेज़ कैप्चर किया गया
crypt-footer-encrypted-badge = 🔒 एन्क्रिप्टेड (age)
crypt-footer-compressed-badge = 📦 कम्प्रेस्ड (zstd)

# Phase 36 — copythat CLI surface. Documented exit codes + the
# user-facing strings for plan/apply/verify/config. Help text stays
# in English (engineering accessibility) but error / info / status
# strings are localized so a localized desktop installation surfaces
# CLI errors in the same language as the GUI.
cli-help-tagline = Copy That CLI — CI/CD पाइपलाइनों के लिए बाइट-एग्ज़ैक्ट फ़ाइल कॉपी, सिंक, सत्यापन और ऑडिट।
cli-help-exit-codes = एग्ज़िट कोड: 0 success, 1 error, 2 pending, 3 collision, 4 verify-fail, 5 net, 6 perm, 7 disk-full, 8 cancel, 9 config।
cli-error-bad-args = copy/move के लिए कम से कम एक स्रोत और एक गंतव्य आवश्यक है
cli-error-unknown-algo = अज्ञात सत्यापन एल्गोरिदम: { $algo }
cli-error-missing-spec = plan/apply के लिए --spec आवश्यक है
cli-error-spec-parse = jobspec { $path } को पार्स करने में विफल: { $reason }
cli-error-spec-empty-sources = Jobspec स्रोत सूची खाली है
cli-info-shape-recorded = बैंडविड्थ शेप "{ $rate }" दर्ज किया गया; प्रवर्तन copythat-shape के ज़रिए जुड़ा हुआ है
cli-info-stub-deferred = { $command } Phase 36 के फ़ॉलो-अप वायरिंग के लिए स्टेज किया गया है
cli-plan-summary = योजना: { $actions } क्रिया(एँ), { $bytes } बाइट(्स); { $already_done } पहले से मौजूद
cli-plan-pending = योजना लंबित क्रियाओं की रिपोर्ट करती है; निष्पादित करने के लिए `apply` के साथ फिर से चलाएँ
cli-plan-already-done = योजना कुछ करने को नहीं बताती (idempotent)
cli-apply-success = Apply बिना त्रुटियों के पूर्ण हुआ
cli-apply-failed = Apply एक या अधिक त्रुटियों के साथ पूर्ण हुआ
cli-verify-ok = सत्यापन ठीक: { $algo } { $digest }
cli-verify-failed = { $path } ({ $algo }) के लिए सत्यापन विफल
cli-config-set = { $key } = { $value } सेट किया गया
cli-config-reset = { $key } को डिफ़ॉल्ट पर रीसेट किया गया
cli-config-unknown-key = अज्ञात कॉन्फ़िग कुंजी: { $key }
cli-completions-emitted = { $shell } के लिए शेल कम्प्लीशन stdout पर प्रिंट किए गए

# Phase 37 — desktop-side mobile companion. Settings → Mobile panel
# strings + the SAS-confirmation modal + push-notification toasts.
# The actual mobile UI lives in the Phase 37 follow-up Tauri Mobile
# target; the phone displays its own localized SAS prompt.
settings-mobile-heading = मोबाइल साथी
settings-mobile-hint = इतिहास ब्राउज़ करने, सहेजी गई प्रोफ़ाइल और Phase 36 jobspecs शुरू करने, और पूर्णता सूचनाएँ प्राप्त करने के लिए एक iPhone या Android फ़ोन जोड़ें।
settings-mobile-pair-toggle = नई जोड़ियाँ अनुमति दें
settings-mobile-pair-active = पेयर-सर्वर सक्रिय — Copy That मोबाइल ऐप से QR स्कैन करें
settings-mobile-pair-button = पेयरिंग शुरू करें
settings-mobile-revoke-button = रद्द करें
settings-mobile-no-pairings = अभी तक कोई जोड़ा गया डिवाइस नहीं
settings-mobile-pair-port = बाइंड पोर्ट (0 = कोई खाली चुनें)
pair-sas-prompt = दोनों स्क्रीन पर एक जैसे चार इमोजी दिखने चाहिए। यदि वे मेल खाते हैं तो Match टैप करें।
pair-sas-confirm = मैच
pair-sas-reject = बेमेल — रद्द करें
pair-toast-success = { $device } के साथ जोड़ा गया
pair-toast-failed = पेयरिंग विफल: { $reason }
push-toast-sent = { $device } को पुश भेजा गया
push-toast-failed = { $device } को पुश विफल: { $reason }

# Phase 38 — aggregate destination dedup + reflink fallback ladder.
# Settings → Transfer → Dedup panel + per-job-row badges (⚡
# Reflinked / 🔗 Hardlinked / 🧩 Chunk-shared / 📋 Copied) +
# pre-pass dedup-scan modal.
settings-dedup-heading = गंतव्य डीडुप
settings-dedup-hint = जब स्रोत और गंतव्य एक वॉल्यूम साझा करते हैं, तो Copy That बाइट्स कॉपी करने के बजाय फ़ाइलसिस्टम स्तर पर फ़ाइलों को क्लोन कर सकता है। Reflink तुरंत और सुरक्षित है; hardlink तेज़ है पर दोनों नाम स्थिति साझा करते हैं।
settings-dedup-mode-auto = ऑटो सीढ़ी (reflink → hardlink → chunk → copy)
settings-dedup-mode-reflink-only = केवल reflink
settings-dedup-mode-hardlink-aggressive = आक्रामक (राइट योग्य फ़ाइलों पर भी reflink + hardlink)
settings-dedup-mode-off = अक्षम (हमेशा बाइट-कॉपी)
settings-dedup-hardlink-policy = Hardlink नीति
settings-dedup-prescan = डुप्लिकेट सामग्री के लिए गंतव्य ट्री को पहले से स्कैन करें
dedup-badge-reflinked = ⚡ Reflinked
dedup-badge-hardlinked = 🔗 Hardlinked
dedup-badge-chunk-shared = 🧩 Chunk-shared
dedup-badge-copied = 📋 Copied
phase42-paranoid-verify-label = पैरानॉयड सत्यापन
phase42-paranoid-verify-hint = राइट-कैश के झूठ और मौन भ्रष्टाचार को पकड़ने के लिए गंतव्य के कैश किए गए पेज हटा देता है और डिस्क से फिर से पढ़ता है। डिफ़ॉल्ट सत्यापन से लगभग 50% धीमा; डिफ़ॉल्ट रूप से बंद।
phase42-sharing-violation-retries-label = लॉक की गई स्रोत फ़ाइलों पर पुनः प्रयासों की संख्या
phase42-sharing-violation-retries-hint = जब कोई अन्य प्रोसेस स्रोत फ़ाइल को एक्सक्लूसिव लॉक के साथ खुला रखे हो तो कितनी बार पुनः प्रयास करना है। बैकऑफ़ हर प्रयास पर दोगुना होता है (डिफ़ॉल्ट रूप से 50 ms / 100 ms / 200 ms)। डिफ़ॉल्ट 3, Robocopy /R:3 से मेल खाता है।
phase42-cloud-placeholder-warning = { $name } एक क्लाउड-ओनली OneDrive फ़ाइल है। इसे कॉपी करने से डाउनलोड शुरू होगा — आपके नेटवर्क कनेक्शन पर { $size } तक।
phase42-defender-exclusion-hint = अधिकतम कॉपी थ्रूपुट के लिए, बल्क स्थानांतरण से पहले गंतव्य फ़ोल्डर को Microsoft Defender बहिष्करणों में जोड़ें। docs/PERFORMANCE_TUNING.md देखें।

# Phase 39 — Browser-accessible recovery UI. Settings → Advanced
# exposes these strings; the recovery server itself renders askama
# templates that always read English — these eight keys are the
# Settings prose the user actually reads in their preferred locale.
settings-recovery-heading = रिकवरी वेब UI
settings-recovery-enable = रिकवरी वेब UI सक्षम करें
settings-recovery-bind-address = बाइंड पता
settings-recovery-port = पोर्ट (0 = कोई खाली चुनें)
settings-recovery-show-url = URL और टोकन दिखाएँ
settings-recovery-rotate-token = टोकन रोटेट करें
settings-recovery-allow-non-loopback = नॉन-लूपबैक बाइंड की अनुमति दें
settings-recovery-non-loopback-warning = चेतावनी: नॉन-लूपबैक बाइंड सक्षम करने से रिकवरी UI आपके स्थानीय नेटवर्क के सामने उजागर हो जाता है। जो भी टोकन जान लेता है वह आपका फ़ाइल इतिहास ब्राउज़ कर सकता है और फ़ाइलें डाउनलोड कर सकता है। यदि LAN अविश्वसनीय है तो इसे TLS या रिवर्स प्रॉक्सी के पीछे रखें।

# Phase 40 — SMB compression negotiation + cloud-VM offload helper.
# 6 SMB keys (header badge + Settings prose) + 6 cloud-offload keys
# (Remotes tab wizard for cross-cloud copy templates).
smb-compress-badge = 🗜 SMB compress: { $algo }
smb-compress-badge-tooltip = इस गंतव्य पर नेटवर्क ट्रैफ़िक ट्रांज़िट में कम्प्रेस किया जा रहा है (SMB 3.1.1)।
smb-compress-toast-saved = नेटवर्क पर { $bytes } बचाए गए
smb-compress-algo-unknown = अज्ञात एल्गोरिदम
settings-smb-compress-heading = SMB नेटवर्क कम्प्रेशन
settings-smb-compress-hint = UNC गंतव्यों पर SMB 3.1.1 ट्रैफ़िक कम्प्रेशन स्वतः नेगोशिएट करें। धीमे लिंक पर मुफ़्त लाभ; स्थानीय गंतव्यों पर अनदेखा किया जाता है।
cloud-offload-heading = क्लाउड-VM ऑफ़लोड हेल्पर
cloud-offload-hint = जब सीधे दो क्लाउड के बीच कॉपी करते हैं, तो एक डिप्लॉयमेंट टेम्प्लेट रेंडर करें जो क्लाउड में एक छोटे अल्पकालिक VM से कॉपी चलाता है — बाइट्स आपके लैपटॉप के नेटवर्क को कभी नहीं छूते।
cloud-offload-render-button = टेम्प्लेट रेंडर करें
cloud-offload-copy-clipboard = क्लिपबोर्ड पर कॉपी करें
cloud-offload-template-format = टेम्प्लेट प्रारूप
cloud-offload-self-destruct-warning = VM { $minutes } मिनट बाद स्वतः बंद हो जाता है — डिप्लॉय करने से पहले IAM रोल + रीजन की पुष्टि करें।

# Phase 41 — animated before/after tree-diff preview. The `Preview
# changes` modal renders the rolled-up plan before the engine starts
# work; 14 keys cover the title, the summary header / counts, the
# row-reason labels, and the two action buttons.
preview-modal-title = बदलावों का पूर्वावलोकन
preview-summary-header = क्या होगा
preview-category-additions = { $count } जोड़
preview-category-replacements = { $count } प्रतिस्थापन
preview-category-skips = { $count } छोड़े गए
preview-category-conflicts = { $count } टकराव
preview-category-unchanged = { $count } अपरिवर्तित
preview-bytes-to-transfer = स्थानांतरित करने के लिए { $bytes }
preview-reason-source-newer = स्रोत नया है
preview-reason-dest-newer = गंतव्य नया है — छोड़ा जाएगा
preview-reason-content-different = सामग्री भिन्न है
preview-reason-identical = स्रोत के समान
preview-button-run = योजना चलाएँ
preview-button-reduce = मेरी योजना घटाएँ…

# Phase 42 — perceptual-hash visual-similarity dedup. Eight keys cover
# the pre-copy "looks visually identical" warning + the Settings panel.
perceptual-warn-title = देखने में एक जैसी लगती है
perceptual-warn-body = गंतव्य पर { $name } स्रोत चित्र से मेल खाती प्रतीत होती है। फिर भी कॉपी करना जारी रखें?
perceptual-warn-keep-both = दोनों रखें
perceptual-warn-skip = यह फ़ाइल छोड़ें
perceptual-warn-overwrite = फिर भी ओवरराइट करें
perceptual-settings-heading = विज़ुअल-समानता डीडुप
perceptual-settings-hint = गंतव्य पर ओवरराइट होने से पहले देखने में एक जैसी छवियाँ पहचानें। हैश परसेप्चुअल है (वही चित्र किसी अन्य प्रारूप में फिर से सहेजा गया हो तो भी पहचानता है), बाइट-एग्ज़ैक्ट नहीं।
perceptual-settings-threshold-label = चेतावनी सीमा (कम = सख्त मेल)

# Phase 42 Part B — per-file rolling versions (Time Machine for any
# destination). 8 keys cover the version-list panel + retention picker.
version-list-heading = पिछले संस्करण
version-list-empty = इस फ़ाइल का कोई पुराना संस्करण नहीं
version-list-restore = यह संस्करण पुनर्स्थापित करें
version-retention-heading = ओवरराइट पर पिछले संस्करण रखें
version-retention-none = हर संस्करण हमेशा रखें
version-retention-last-n = अंतिम { $n } संस्करण रखें
version-retention-older-than-days = { $days } दिन से पुराने संस्करण हटाएँ
version-retention-gfs = प्रति घंटा { $h } · दैनिक { $d } · साप्ताहिक { $w } · मासिक { $m }

# Phase 43 — forensic chain-of-custody manifests + BLAKE3 verified
# streaming. 14 keys cover the Settings → Provenance panel
# (heading, hint, toggles, TSA URL, signing-key management) plus
# the post-job manifest notification + the verify command's two
# headline result lines.
provenance-settings-heading = फ़ोरेंसिक चेन-ऑफ़-कस्टडी
provenance-settings-hint = हर कॉपी कार्य को एक BLAKE3 + ed25519 मैनिफ़ेस्ट से साइन करें। समीक्षक बाद में गंतव्य ट्री को फिर से हैश कर सकते हैं और साबित कर सकते हैं कि कॉपी के बाद से कोई बाइट नहीं बदला।
provenance-settings-enable-default = डिफ़ॉल्ट रूप से हर नए कार्य को साइन करें
provenance-settings-show-after-job = हर पूर्ण किए गए कार्य के बाद मैनिफ़ेस्ट दिखाएँ
provenance-settings-tsa-url-label = डिफ़ॉल्ट RFC 3161 टाइमस्टैम्प अथॉरिटी URL
provenance-settings-tsa-url-hint = वैकल्पिक। सेट होने पर, मैनिफ़ेस्ट एक मुफ़्त TSA टाइमस्टैम्प रखते हैं जो साबित करता है कि बाइट्स इस समय बिंदु पर मौजूद थे। छोड़ने के लिए खाली रखें।
provenance-settings-keys-heading = साइनिंग कुंजियाँ
provenance-settings-keys-generate = नई कुंजी बनाएँ
provenance-settings-keys-import = कुंजी आयात करें…
provenance-settings-keys-export = सार्वजनिक कुंजी निर्यात करें…
provenance-job-completed-title = प्रोवेनेंस मैनिफ़ेस्ट सहेजा गया
provenance-job-completed-body = { $count } फ़ाइलें साइन की गईं → { $path }
provenance-verify-clean = { $count } फ़ाइलों के लिए मैनिफ़ेस्ट मान्य; हस्ताक्षर { $sig }; merkle रूट ठीक।
provenance-verify-tampered = मैनिफ़ेस्ट अमान्य — { $tampered } से छेड़छाड़ हुई, { $missing } गायब।
# Phase 43 post-review hardening — toast text for the Settings →
# Provenance buttons whose Tauri IPC has not yet landed.
provenance-action-staged = Phase 43 — इस क्रिया के लिए IPC वायरिंग एक फ़ॉलो-अप कमिट में आएगी।

# Phase 44 — SSD-aware whole-drive sanitize (NVMe Sanitize / OPAL
# Crypto Erase) + the per-file shred refusal on copy-on-write
# filesystems. 16 keys cover the new "Drive sanitize" Settings
# subsection and the localized error messages.
sanitize-heading = पूरी-ड्राइव सुरक्षित सैनिटाइज़
sanitize-hint = NVMe Sanitize, OPAL Crypto Erase, और ATA Secure Erase एक फ़्लैश ड्राइव को फ़र्मवेयर परत पर मिलीसेकंड में मिटा देते हैं। फ़्लैश पर प्रति-फ़ाइल ओवरराइट निरर्थक है — मल्टी-पास श्रेड केवल NAND जलाता है। वास्तविक पर्ज के लिए इसका उपयोग करें।
sanitize-pick-device = सैनिटाइज़ करने के लिए ड्राइव चुनें
sanitize-mode-label = सैनिटाइज़ेशन विधि
sanitize-mode-nvme-format = NVMe Format (सुरक्षित इरेज़ के साथ)
sanitize-mode-nvme-sanitize-block = NVMe Sanitize — Block Erase (धीमा, हर सेल)
sanitize-mode-nvme-sanitize-crypto = NVMe Sanitize — Crypto Erase (तुरंत)
sanitize-mode-ata-secure-erase = ATA Secure Erase (लीगेसी SATA SSDs)
sanitize-mode-opal-crypto-erase = TCG OPAL Crypto Erase (सेल्फ-एन्क्रिप्टिंग ड्राइव)
sanitize-mode-apfs-crypto-erase = APFS Crypto Erase (FileVault कुंजी रोटेट करें, केवल macOS)
sanitize-confirm-1 = यह { $device } पर हर बाइट नष्ट कर देता है। इसे वापस नहीं किया जा सकता।
sanitize-confirm-2 = मैं समझता हूँ कि { $device } पर सभी पार्टिशन, सभी फ़ाइलें, और सभी स्नैपशॉट स्थायी रूप से अपठनीय हो जाएँगे।
sanitize-confirm-3 = आगे बढ़ने के लिए ड्राइव का मॉडल नाम टाइप करें: { $model }
sanitize-running = { $device } ({ $mode }) सैनिटाइज़ हो रहा है — इसमें मिलीसेकंड (क्रिप्टो इरेज़) से लेकर दसियों मिनट (ब्लॉक इरेज़) तक लग सकते हैं। पावर बंद न करें।
sanitize-completed = सैनिटाइज़ पूर्ण — { $device } अब खाली है।
ssd-honest-shred-meaningless = कॉपी-ऑन-राइट फ़ाइलसिस्टम (Btrfs / ZFS / APFS) पर प्रति-फ़ाइल श्रेड अंतर्निहित ब्लॉक्स तक नहीं पहुँच सकता। इसके बजाय पूरी-ड्राइव सैनिटाइज़ और फ़ुल-डिस्क-एन्क्रिप्शन कुंजी रोटेशन का उपयोग करें।
ssd-honest-advisory = यह फ़ाइल फ़्लैश पर मौजूद है। प्रति-फ़ाइल ओवरराइट NAND घिसाव की कीमत लेता है और यह गारंटी नहीं देता कि मूल सेल अपुनर्प्राप्य हैं। संवेदनशील डेटा के लिए, पूरी ड्राइव सैनिटाइज़ करें।

# Phase 44.1f post-review — placeholder toast for SanitizeTab
# buttons whose Tauri IPC has not yet landed.
sanitize-action-staged = Phase 44.1 — इस क्रिया के लिए IPC वायरिंग एक फ़ॉलो-अप कमिट में आएगी।

# Phase 45.3 — named-queue tab strip (Subfeature A). Tabs surface
# once the QueueRegistry holds at least one queue; the synthesised
# default tab keeps legacy single-queue jobs reachable.
queue-tab-default = डिफ़ॉल्ट
queue-tab-empty-state = कार्य कतारें
queue-badge-tooltip = इस कतार में लंबित और चल रहे कार्य

# Phase 45.4 — drag-progress-merge (Subfeature B). Drag a queue tab
# onto another to merge their job lists. The default tab is neither
# draggable nor a drop target; only registry queues participate.
queue-drag-hint = मर्ज करने के लिए किसी अन्य कतार पर खींचें
queue-merge-confirm = मर्ज करने के लिए छोड़ें
queue-merge-toast = कतारें मर्ज की गईं

# Phase 45.5 — F2-queue UX (Subfeature C). F2 toggles
# `auto_enqueue_next` so every fresh enqueue piles into the running
# queue rather than spawning a parallel one. Status pill renders in
# the Footer; pulsing dot renders on the running tab.
queue-f2-active-hint = F2 मोड: हर नया एनक्यू इसी कतार में आता है
queue-f2-toggled-on = F2 कतार मोड चालू — नए एनक्यू चल रही कतार में जुड़ते हैं
queue-f2-toggled-off = F2 कतार मोड बंद — नए एनक्यू समानांतर कतारें बनाते हैं
queue-f2-status-bar = F2 कतार मोड: चालू

# Phase 45.6 — tray destination targets (Subfeature D). Pinned
# destinations appear in the OS tray menu; clicking one arms it as
# the active drop target so the next file drop bypasses the
# DropStagingDialog. Settings → General hosts the list editor.
tray-target-section-title = ट्रे गंतव्य
tray-target-section-hint = पिन किए गए गंतव्य ट्रे मेनू में दिखते हैं। अगले ड्रॉप लक्ष्य के रूप में आर्म करने के लिए किसी एक पर क्लिक करें।
tray-target-empty = अभी तक कोई ट्रे गंतव्य पिन नहीं किया गया।
tray-target-remove = हटाएँ
tray-target-add-label = लेबल
tray-target-add-path = पथ या बैकएंड URI
tray-target-add = जोड़ें
tray-target-armed-toast = अपनी अगली फ़ाइल छोड़ें ताकि वह { $label } को भेजी जाए
tray-target-active-pill = → { $label }

# Phase 45.7 follow-up — pinned-destination validation errors. The
# `queue_pin_destination` IPC returns these Fluent keys verbatim
# when input fails the IPC-boundary checks; the toast layer renders
# them via `t(...)` (Toast.svelte detects kebab-case-lowercase and
# routes through the locale table). Phase 17e's `err-path-escape` /
# `err-destination-empty` cover the path-traversal + empty cases for
# `queue_route_job`; the keys below are pin-specific.
err-pinned-destination-label-empty = ट्रे गंतव्य लेबल खाली नहीं हो सकता।
err-pinned-destination-path-empty = ट्रे गंतव्य पथ खाली नहीं हो सकता।
err-pinned-destination-label-too-long = ट्रे गंतव्य लेबल बहुत लंबा है (अधिकतम 64 वर्ण)।
err-pinned-destination-path-too-long = ट्रे गंतव्य पथ बहुत लंबा है (अधिकतम 1024 वर्ण)।
err-pinned-destination-label-invalid = ट्रे गंतव्य लेबल में ऐसे वर्ण हैं जिनकी अनुमति नहीं है (न्यूलाइन, रिटर्न, या NUL)।
err-pinned-destination-path-invalid = ट्रे गंतव्य पथ में ऐसे वर्ण हैं जिनकी अनुमति नहीं है (न्यूलाइन, रिटर्न, या NUL)।
err-pinned-destination-too-many = आप 50 ट्रे गंतव्यों की सीमा तक पहुँच गए हैं। दूसरा जोड़ने के लिए किसी एक को हटाएँ।

# Phase 46.6 — Settings → Plugins tab. The IPC layer in
# `apps/copythat-ui/src-tauri/src/plugin_commands.rs` enumerates the
# per-user plugin store under `<config_dir>/plugins/`; the
# `PluginsTab.svelte` component renders these strings against the
# returned manifest + grant state.
settings-tab-plugins = प्लगइन
plugin-heading = प्लगइन
plugin-hint = सैंडबॉक्स्ड WASM प्लगइन Copy That को कस्टम हुक के साथ विस्तृत करते हैं। हर प्लगइन प्रति-कॉल CPU और मेमोरी सीमाओं के तहत चलता है और केवल वही होस्ट क्षमताएँ देखता है जो आप उसे देते हैं।
plugin-list-empty = अभी तक कोई प्लगइन इंस्टॉल नहीं किया गया।
plugin-enabled = सक्षम
plugin-disabled = अक्षम
plugin-hooks = हुक
plugin-capabilities = क्षमताएँ
plugin-no-capabilities = (कोई नहीं)
plugin-directory = स्थान
plugin-install-from-file = फ़ाइल से इंस्टॉल करें…
plugin-install-from-url = URL से इंस्टॉल करें…
plugin-url-wasm = WASM URL
plugin-url-manifest = मैनिफ़ेस्ट URL
plugin-url-hash = BLAKE3 हैश
plugin-url-preview = पूर्वावलोकन
plugin-url-confirm = इंस्टॉल की पुष्टि करें
