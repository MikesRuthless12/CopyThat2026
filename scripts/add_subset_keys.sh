#!/usr/bin/env bash
# Phase 14e — subset picker + "pick which to copy" preflight action.

set -euo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"

append() {
  local locale="$1"; shift
  local file="$root/locales/$locale/copythat.ftl"
  cat >> "$file" <<EOF

# Phase 14e — subset picker
preflight-pick-subset = $1
subset-title = $2
subset-subtitle = $3
subset-loading = $4
subset-too-large = $5
subset-budget = $6
subset-remaining = $7
subset-confirm = $8
EOF
}

append ar "اختر ما تنسخه…" "اختر المصادر المراد نسخها" "لا تتسع المجموعة الكاملة في الوجهة. ضع علامة على ما تريد نسخه؛ يبقى الباقي." "جاري قياس الأحجام…" "أكبر من أن يُحصى" "متاح" "متبقي" "نسخ المحدد"
append de "Auswählen, was kopiert wird…" "Zu kopierende Quellen auswählen" "Die komplette Auswahl passt nicht auf das Ziel. Wähle aus, was kopiert werden soll; der Rest bleibt zurück." "Größen werden gemessen…" "zu groß zum Zählen" "Verfügbar" "Verbleibend" "Auswahl kopieren"
append es "Elegir qué copiar…" "Elige qué fuentes copiar" "La selección completa no cabe en el destino. Marca lo que quieras copiar; el resto se queda." "Midiendo tamaños…" "demasiado grande para contar" "Disponible" "Restante" "Copiar selección"
append fr "Choisir ce qui sera copié…" "Sélectionnez les sources à copier" "La sélection complète ne rentre pas sur la destination. Cochez ce que vous voulez copier ; le reste est ignoré." "Mesure des tailles…" "trop volumineux à compter" "Disponible" "Restant" "Copier la sélection"
append hi "चुनें कि क्या कॉपी करना है…" "कॉपी करने के लिए स्रोत चुनें" "पूरी सूची गंतव्य पर फिट नहीं होगी। जो चाहते हैं वह चुनें; बाकी छोड़ दिया जाएगा।" "आकार मापा जा रहा है…" "गिनने के लिए बहुत बड़ा" "उपलब्ध" "शेष" "चयन कॉपी करें"
append id "Pilih yang akan disalin…" "Pilih sumber untuk disalin" "Pilihan lengkap tidak muat di tujuan. Centang yang ingin disalin; sisanya tidak disalin." "Mengukur ukuran…" "terlalu besar untuk dihitung" "Tersedia" "Tersisa" "Salin pilihan"
append it "Scegli cosa copiare…" "Scegli le sorgenti da copiare" "La selezione completa non entra nella destinazione. Spunta gli elementi da copiare; il resto resta." "Misura delle dimensioni…" "troppo grande da contare" "Disponibile" "Rimanente" "Copia selezione"
append ja "コピー対象を選択…" "コピーするソースを選択" "すべての選択はコピー先に収まりません。コピーするものにチェックを入れてください。残りはスキップされます。" "サイズを測定中…" "大きすぎて計測できない" "利用可能" "残り" "選択をコピー"
append ko "복사할 항목 선택…" "복사할 원본 선택" "전체 선택이 대상에 맞지 않습니다. 복사할 항목을 선택하면 나머지는 복사되지 않습니다." "크기 측정 중…" "너무 커서 셀 수 없음" "사용 가능" "남음" "선택 복사"
append nl "Kies wat gekopieerd wordt…" "Kies de bronnen om te kopiëren" "De volledige selectie past niet op de bestemming. Vink aan wat je wilt kopiëren; de rest blijft achter." "Groottes meten…" "te groot om te tellen" "Beschikbaar" "Resterend" "Selectie kopiëren"
append pl "Wybierz, co skopiować…" "Wybierz źródła do skopiowania" "Cała selekcja nie zmieści się na miejscu docelowym. Zaznacz to, co chcesz skopiować; reszta zostanie pominięta." "Mierzenie rozmiarów…" "za duże, by policzyć" "Dostępne" "Pozostało" "Skopiuj wybór"
append pt-BR "Escolher o que copiar…" "Escolha quais fontes copiar" "A seleção completa não cabe no destino. Marque o que deseja copiar; o restante fica para trás." "Medindo tamanhos…" "grande demais para contar" "Disponível" "Restante" "Copiar seleção"
append ru "Выбрать, что копировать…" "Выберите источники для копирования" "Полный набор не помещается в пункте назначения. Отметьте то, что хотите скопировать; остальное останется." "Измерение размеров…" "слишком велико для подсчёта" "Доступно" "Осталось" "Копировать выбор"
append tr "Hangisinin kopyalanacağını seç…" "Kopyalanacak kaynakları seçin" "Seçimin tamamı hedefe sığmıyor. Kopyalamak istediklerinizi işaretleyin; geri kalanlar atlanır." "Boyutlar ölçülüyor…" "sayılamayacak kadar büyük" "Kullanılabilir" "Kalan" "Seçileni kopyala"
append uk "Вибрати, що копіювати…" "Виберіть джерела для копіювання" "Повний набір не вміщується в призначенні. Позначте те, що хочете скопіювати; решта залишиться." "Вимірювання розмірів…" "завеликий для підрахунку" "Доступно" "Залишилось" "Скопіювати вибране"
append vi "Chọn mục cần sao chép…" "Chọn nguồn để sao chép" "Toàn bộ lựa chọn không vừa tại đích. Đánh dấu những mục cần sao chép; phần còn lại sẽ bị bỏ qua." "Đang đo dung lượng…" "quá lớn để đếm" "Khả dụng" "Còn lại" "Sao chép mục đã chọn"
append zh-CN "选择要复制的内容…" "选择要复制的源" "整体选择无法装入目标。勾选要复制的项目；其余将保留。" "正在测量大小…" "过大无法计数" "可用" "剩余" "复制所选"

echo "Added subset picker keys to 17 locales."
