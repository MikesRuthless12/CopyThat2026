#!/usr/bin/env bash
set -euo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
append() {
  local locale="$1"; shift
  cat >> "$root/locales/$locale/copythat.ftl" <<EOF
drop-dialog-collision-label = $1
collision-policy-keep-both = $2
collision-policy-skip = $3
collision-policy-overwrite = $4
collision-policy-overwrite-if-newer = $5
collision-policy-prompt = $6
EOF
}
append ar "إذا كان الملف موجودًا:" "الإبقاء على كليهما (إعادة تسمية النسخة الجديدة إلى _2 و_3 و…)" "تخطي النسخة الجديدة" "استبدال الملف الموجود" "الاستبدال فقط إذا كان أحدث" "السؤال في كل مرة"
append de "Wenn eine Datei bereits existiert:" "Beide behalten (neue Kopie zu _2, _3 … umbenennen)" "Neue Kopie überspringen" "Vorhandene Datei überschreiben" "Nur überschreiben, wenn neuer" "Jedes Mal fragen"
append es "Si el archivo ya existe:" "Conservar ambos (renombrar la nueva copia a _2, _3 …)" "Omitir la nueva copia" "Sobrescribir el archivo existente" "Sobrescribir solo si es más nuevo" "Preguntar cada vez"
append fr "Si un fichier existe déjà :" "Conserver les deux (renommer la nouvelle copie en _2, _3 …)" "Ignorer la nouvelle copie" "Écraser le fichier existant" "Écraser uniquement si plus récent" "Demander à chaque fois"
append hi "अगर फ़ाइल पहले से मौजूद है:" "दोनों रखें (नई प्रति का नाम बदलकर _2, _3, … करें)" "नई प्रति छोड़ दें" "मौजूदा फ़ाइल पर लिखें" "केवल नई होने पर अधिलेखित करें" "हर बार पूछें"
append id "Jika file sudah ada:" "Simpan keduanya (ganti nama salinan baru menjadi _2, _3, …)" "Lewati salinan baru" "Timpa file yang ada" "Timpa hanya jika lebih baru" "Tanyakan setiap kali"
append it "Se un file esiste già:" "Mantieni entrambi (rinomina la nuova copia in _2, _3, …)" "Salta la nuova copia" "Sovrascrivi il file esistente" "Sovrascrivi solo se più nuovo" "Chiedi ogni volta"
append ja "ファイルが既に存在する場合:" "両方を保持 (新しいコピーを _2、_3 … にリネーム)" "新しいコピーをスキップ" "既存のファイルを上書き" "新しいときのみ上書き" "毎回確認"
append ko "파일이 이미 존재하는 경우:" "둘 다 유지 (새 복사본의 이름을 _2, _3 … 로 변경)" "새 복사본 건너뛰기" "기존 파일 덮어쓰기" "새로운 경우에만 덮어쓰기" "매번 묻기"
append nl "Als een bestand al bestaat:" "Beide behouden (nieuwe kopie hernoemen naar _2, _3, …)" "Nieuwe kopie overslaan" "Bestaand bestand overschrijven" "Alleen overschrijven als nieuwer" "Elke keer vragen"
append pl "Jeśli plik już istnieje:" "Zachowaj oba (zmień nazwę nowej kopii na _2, _3 …)" "Pomiń nową kopię" "Nadpisz istniejący plik" "Nadpisz tylko, jeśli nowszy" "Pytaj za każdym razem"
append pt-BR "Se um arquivo já existir:" "Manter os dois (renomear a nova cópia para _2, _3, …)" "Ignorar a nova cópia" "Sobrescrever o arquivo existente" "Sobrescrever apenas se for mais novo" "Perguntar sempre"
append ru "Если файл уже существует:" "Сохранить оба (переименовать новую копию в _2, _3 …)" "Пропустить новую копию" "Перезаписать существующий файл" "Перезаписывать только если новее" "Спрашивать каждый раз"
append tr "Dosya zaten varsa:" "İkisini de tut (yeni kopyayı _2, _3 … olarak yeniden adlandır)" "Yeni kopyayı atla" "Mevcut dosyanın üzerine yaz" "Yalnızca daha yeniyse üzerine yaz" "Her seferinde sor"
append uk "Якщо файл уже існує:" "Зберегти обидва (перейменувати нову копію у _2, _3 …)" "Пропустити нову копію" "Перезаписати існуючий файл" "Перезаписувати лише якщо новіший" "Питати щоразу"
append vi "Nếu tệp đã tồn tại:" "Giữ cả hai (đổi tên bản sao mới thành _2, _3, …)" "Bỏ qua bản sao mới" "Ghi đè tệp hiện có" "Chỉ ghi đè nếu mới hơn" "Hỏi mỗi lần"
append zh-CN "如果文件已存在:" "同时保留两者（将新副本重命名为 _2、_3 …）" "跳过新副本" "覆盖现有文件" "仅在较新时覆盖" "每次询问"
echo "Collision policy keys added to 17 locales."
