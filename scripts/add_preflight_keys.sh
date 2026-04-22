#!/usr/bin/env bash
# Phase 14 — preflight + collision-modal-overwrite-older key drop.

set -euo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"

append() {
  local locale="$1"; shift
  local file="$root/locales/$locale/copythat.ftl"
  cat >> "$file" <<EOF

# Phase 14 — preflight free-space dialog
preflight-block-title = $1
preflight-warn-title = $2
preflight-unknown-title = $3
preflight-unknown-body = $4
preflight-required = $5
preflight-free = $6
preflight-reserve = $7
preflight-shortfall = $8
preflight-continue = $9
collision-modal-overwrite-older = ${10}
EOF
}

append ar "لا توجد مساحة كافية في الوجهة" "مساحة قليلة في الوجهة" "تعذر تحديد المساحة الفارغة" "المصدر كبير جدًا بحيث لا يمكن قياسه بسرعة أو أن محرك الوجهة لم يستجب. يمكنك المتابعة؛ سيوقف المحرك النسخ بأمان عند نفاد المساحة." "مطلوب" "فارغ" "محجوز" "عجز" "متابعة على أي حال" "استبدال الأقدم فقط"
append de "Nicht genug Speicher am Ziel" "Wenig Speicher am Ziel" "Freier Speicher nicht ermittelbar" "Die Quelle ist zu groß, um schnell gemessen zu werden, oder das Ziellaufwerk antwortet nicht. Sie können fortfahren; die Schutzfunktion der Engine stoppt den Kopiervorgang sauber, falls der Platz ausgeht." "Benötigt" "Frei" "Reserve" "Fehlbetrag" "Trotzdem fortfahren" "Nur ältere überschreiben"
append es "No hay espacio suficiente en el destino" "Poco espacio en el destino" "No se pudo determinar el espacio libre" "El origen es demasiado grande para medirlo rápidamente o el volumen de destino no respondió. Puedes continuar; el protector del motor detendrá la copia limpiamente si se queda sin espacio." "Necesario" "Libre" "Reserva" "Déficit" "Continuar igualmente" "Sobrescribir solo los más antiguos"
append fr "Espace insuffisant sur la destination" "Espace faible sur la destination" "Espace libre indéterminé" "La source est trop volumineuse pour être mesurée rapidement ou le volume de destination n'a pas répondu. Vous pouvez continuer ; le garde-fou du moteur arrêtera proprement la copie si l'espace vient à manquer." "Requis" "Libre" "Réserve" "Déficit" "Continuer quand même" "Écraser uniquement les plus anciens"
append hi "गंतव्य पर पर्याप्त स्थान नहीं है" "गंतव्य पर कम स्थान है" "खाली स्थान निर्धारित नहीं हो सका" "स्रोत को शीघ्रता से मापने के लिए बहुत बड़ा है या गंतव्य वॉल्यूम ने उत्तर नहीं दिया। आप जारी रख सकते हैं; स्थान समाप्त होने पर इंजन कॉपी को साफ़ तरीके से रोक देगा।" "आवश्यक" "खाली" "आरक्षित" "कमी" "फिर भी जारी रखें" "केवल पुराने अधिलेखित करें"
append id "Ruang tidak cukup di tujuan" "Ruang sedikit di tujuan" "Tidak dapat menentukan ruang kosong" "Sumber terlalu besar untuk diukur cepat atau volume tujuan tidak merespons. Anda dapat melanjutkan; mesin akan menghentikan penyalinan dengan rapi jika ruang habis." "Dibutuhkan" "Kosong" "Cadangan" "Kekurangan" "Lanjutkan saja" "Timpa hanya yang lebih lama"
append it "Spazio insufficiente nella destinazione" "Spazio ridotto nella destinazione" "Impossibile determinare lo spazio libero" "La sorgente è troppo grande per essere misurata rapidamente o il volume di destinazione non ha risposto. Puoi continuare; il motore fermerà la copia in modo pulito se lo spazio si esaurisce." "Richiesto" "Libero" "Riserva" "Deficit" "Continua comunque" "Sovrascrivi solo i più vecchi"
append ja "コピー先に十分な空き容量がありません" "コピー先の空き容量が少なくなっています" "空き容量を判別できません" "コピー元のサイズが大きすぎて短時間で測定できないか、コピー先ボリュームが応答しませんでした。続行できます。空き容量がなくなった場合は、エンジンがコピーを安全に停止します。" "必要" "空き" "予約" "不足" "それでも続行" "古いファイルのみ上書き"
append ko "대상에 공간이 부족합니다" "대상의 공간이 부족합니다" "여유 공간을 확인할 수 없습니다" "원본이 너무 커서 빠르게 측정할 수 없거나 대상 볼륨이 응답하지 않았습니다. 계속할 수 있으며, 공간이 부족해지면 엔진이 복사를 깔끔하게 중단합니다." "필요" "남음" "예약" "부족" "그래도 계속" "오래된 파일만 덮어쓰기"
append nl "Onvoldoende ruimte op de bestemming" "Weinig ruimte op de bestemming" "Vrije ruimte niet te bepalen" "De bron is te groot om snel te meten of het doelvolume reageerde niet. Je kunt doorgaan; de beveiliging van de engine stopt het kopiëren netjes als de ruimte opraakt." "Vereist" "Vrij" "Reserve" "Tekort" "Toch doorgaan" "Alleen oudere overschrijven"
append pl "Za mało miejsca w miejscu docelowym" "Mało miejsca w miejscu docelowym" "Nie można ustalić wolnego miejsca" "Źródło jest za duże, by je szybko zmierzyć, lub wolumin docelowy nie odpowiedział. Możesz kontynuować; zabezpieczenie silnika czysto zatrzyma kopiowanie, jeśli zabraknie miejsca." "Wymagane" "Wolne" "Rezerwa" "Niedobór" "Kontynuuj mimo to" "Nadpisz tylko starsze"
append pt-BR "Espaço insuficiente no destino" "Pouco espaço no destino" "Não foi possível determinar o espaço livre" "A origem é grande demais para ser medida rapidamente ou o volume de destino não respondeu. Você pode continuar; o limitador do mecanismo interromperá a cópia com segurança se o espaço acabar." "Necessário" "Livre" "Reserva" "Déficit" "Continuar mesmo assim" "Sobrescrever só os mais antigos"
append ru "На целевом томе недостаточно места" "Мало места на целевом томе" "Не удалось определить свободное место" "Источник слишком велик для быстрого измерения, или целевой том не ответил. Вы можете продолжить; защита движка корректно остановит копирование, если место закончится." "Требуется" "Свободно" "Резерв" "Недостача" "Всё равно продолжить" "Перезаписать только более старые"
append tr "Hedefte yeterli alan yok" "Hedefte az alan var" "Boş alan belirlenemedi" "Kaynak hızlı ölçülemeyecek kadar büyük veya hedef disk yanıt vermedi. Devam edebilirsiniz; alan biterse motor kopyalamayı temiz bir şekilde durdurur." "Gerekli" "Boş" "Rezerv" "Eksik" "Yine de devam et" "Yalnızca eskileri üzerine yaz"
append uk "На цільовому томі недостатньо місця" "Мало місця на цільовому томі" "Не вдалося визначити вільне місце" "Джерело задовелике, щоб швидко виміряти, або цільовий том не відповів. Ви можете продовжити; захист рушія акуратно зупинить копіювання, якщо закінчиться місце." "Потрібно" "Вільно" "Резерв" "Нестача" "Все одно продовжити" "Перезаписувати лише старіші"
append vi "Không đủ dung lượng tại đích" "Ít dung lượng tại đích" "Không thể xác định dung lượng trống" "Nguồn quá lớn để đo nhanh hoặc đích không phản hồi. Bạn có thể tiếp tục; hệ thống sẽ dừng sao chép an toàn nếu hết dung lượng." "Cần" "Trống" "Dự trữ" "Thiếu" "Vẫn tiếp tục" "Chỉ ghi đè tệp cũ hơn"
append zh-CN "目标位置空间不足" "目标位置空间较低" "无法确定可用空间" "源过大无法快速测量，或目标卷未响应。您可以继续；如果空间用完，引擎会干净地停止复制。" "需要" "可用" "保留" "缺口" "仍然继续" "仅覆盖较旧的文件"

echo "Added preflight + overwrite-older keys to 17 locales."
