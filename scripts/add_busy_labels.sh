#!/usr/bin/env bash
set -euo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
append() {
  local locale="$1"; shift
  cat >> "$root/locales/$locale/copythat.ftl" <<EOF
drop-dialog-busy-checking = $1
drop-dialog-busy-enumerating = $2
drop-dialog-busy-starting = $3
toast-enumeration-deferred = $4
EOF
}
append ar "جارٍ فحص المساحة الفارغة…" "جارٍ إحصاء الملفات…" "جارٍ بدء النسخ…" "شجرة المصدر كبيرة — تخطّي قائمة الملفات المسبقة؛ ستظهر الصفوف أثناء معالجة المحرك."
append de "Freier Speicher wird geprüft…" "Dateien werden gezählt…" "Kopiervorgang wird gestartet…" "Der Quellbaum ist groß — Vorabliste wird übersprungen; Zeilen erscheinen, sobald die Engine sie verarbeitet."
append es "Comprobando espacio libre…" "Contando archivos…" "Iniciando copia…" "El árbol de origen es grande — se omite la lista previa; las filas aparecerán a medida que el motor las procese."
append fr "Vérification de l'espace libre…" "Comptage des fichiers…" "Démarrage de la copie…" "L'arborescence source est volumineuse — liste préalable ignorée ; les lignes apparaîtront au fur et à mesure du traitement."
append hi "खाली स्थान की जाँच…" "फ़ाइलें गिनी जा रही हैं…" "कॉपी आरंभ हो रही है…" "स्रोत ट्री बड़ा है — अग्रिम सूची छोड़ दी गई; जैसे-जैसे इंजन काम करेगा पंक्तियाँ दिखेंगी।"
append id "Memeriksa ruang kosong…" "Menghitung file…" "Memulai penyalinan…" "Pohon sumber besar — melewati daftar awal; baris akan muncul seiring mesin memprosesnya."
append it "Verifica spazio libero…" "Conteggio file…" "Avvio della copia…" "L'albero di origine è grande — lista preliminare saltata; le righe appariranno mentre il motore le elabora."
append ja "空き容量を確認中…" "ファイルを数えています…" "コピーを開始しています…" "ソースツリーが大きいため事前リストを省略しました。エンジンが処理するごとに行が表示されます。"
append ko "여유 공간 확인 중…" "파일 수 계산 중…" "복사 시작 중…" "원본 트리가 큽니다 — 사전 목록을 생략합니다. 엔진이 처리하는 대로 행이 나타납니다."
append nl "Vrije ruimte controleren…" "Bestanden tellen…" "Kopie starten…" "Bronstructuur is groot — voorbereide lijst overgeslagen; regels verschijnen terwijl de engine ze verwerkt."
append pl "Sprawdzanie wolnego miejsca…" "Liczenie plików…" "Uruchamianie kopiowania…" "Drzewo źródłowe jest duże — pomijanie listy wstępnej; wiersze pojawią się w trakcie przetwarzania."
append pt-BR "Verificando espaço livre…" "Contando arquivos…" "Iniciando cópia…" "A árvore de origem é grande — lista prévia ignorada; linhas aparecerão conforme o mecanismo processar."
append ru "Проверка свободного места…" "Подсчёт файлов…" "Запуск копирования…" "Исходное дерево большое — предварительный список пропущен; строки появятся по мере работы движка."
append tr "Boş alan kontrol ediliyor…" "Dosyalar sayılıyor…" "Kopyalama başlatılıyor…" "Kaynak ağaç büyük — ön liste atlanıyor; satırlar motor işlediğinde görünecek."
append uk "Перевірка вільного місця…" "Підрахунок файлів…" "Запуск копіювання…" "Дерево джерела велике — пропускаємо попередній список; рядки з'являться під час обробки."
append vi "Đang kiểm tra dung lượng trống…" "Đang đếm tệp…" "Đang bắt đầu sao chép…" "Cây nguồn lớn — bỏ qua danh sách trước; các dòng sẽ xuất hiện khi bộ máy xử lý."
append zh-CN "正在检查可用空间…" "正在统计文件…" "正在开始复制…" "源树较大 — 跳过预列表；引擎处理时行会陆续显示。"
echo "Busy labels added to 17 locales."
