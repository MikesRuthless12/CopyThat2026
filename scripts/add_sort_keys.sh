#!/usr/bin/env bash
# Phase 15 — source-list sort + reorder translation drop.

set -euo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"

append() {
  local locale="$1"; shift
  local file="$root/locales/$locale/copythat.ftl"
  cat >> "$file" <<EOF

# Phase 15 — source-list ordering
drop-dialog-sort-label = $1
sort-custom = $2
sort-name-asc = $3
sort-name-desc = $4
sort-size-asc = $5
sort-size-desc = $6
sort-reorder = $7
sort-move-top = $8
sort-move-up = $9
sort-move-down = ${10}
sort-move-bottom = ${11}
EOF
}

append ar "الترتيب:" "مخصص" "الاسم أ ← ي (الملفات أولًا)" "الاسم ي ← أ (الملفات أولًا)" "الحجم الأصغر أولًا (الملفات أولًا)" "الحجم الأكبر أولًا (الملفات أولًا)" "إعادة ترتيب" "نقل إلى الأعلى" "تحريك للأعلى" "تحريك للأسفل" "نقل إلى الأسفل"
append de "Reihenfolge:" "Benutzerdefiniert" "Name A → Z (Dateien zuerst)" "Name Z → A (Dateien zuerst)" "Größe aufsteigend (Dateien zuerst)" "Größe absteigend (Dateien zuerst)" "Neu anordnen" "Nach ganz oben" "Nach oben" "Nach unten" "Nach ganz unten"
append es "Orden:" "Personalizado" "Nombre A → Z (archivos primero)" "Nombre Z → A (archivos primero)" "Tamaño ascendente (archivos primero)" "Tamaño descendente (archivos primero)" "Reordenar" "Mover al principio" "Subir" "Bajar" "Mover al final"
append fr "Ordre :" "Personnalisé" "Nom A → Z (fichiers d'abord)" "Nom Z → A (fichiers d'abord)" "Taille croissante (fichiers d'abord)" "Taille décroissante (fichiers d'abord)" "Réorganiser" "Mettre tout en haut" "Monter" "Descendre" "Mettre tout en bas"
append hi "क्रम:" "कस्टम" "नाम A → Z (पहले फ़ाइलें)" "नाम Z → A (पहले फ़ाइलें)" "आकार छोटा से बड़ा (पहले फ़ाइलें)" "आकार बड़ा से छोटा (पहले फ़ाइलें)" "पुनः व्यवस्थित करें" "सबसे ऊपर ले जाएँ" "ऊपर ले जाएँ" "नीचे ले जाएँ" "सबसे नीचे ले जाएँ"
append id "Urutan:" "Khusus" "Nama A → Z (file dulu)" "Nama Z → A (file dulu)" "Ukuran terkecil dulu (file dulu)" "Ukuran terbesar dulu (file dulu)" "Susun ulang" "Pindah ke atas" "Naik" "Turun" "Pindah ke bawah"
append it "Ordinamento:" "Personalizzato" "Nome A → Z (prima i file)" "Nome Z → A (prima i file)" "Dimensione crescente (prima i file)" "Dimensione decrescente (prima i file)" "Riordina" "Sposta in alto" "Su" "Giù" "Sposta in basso"
append ja "並び順:" "カスタム" "名前 A → Z（ファイルを先に）" "名前 Z → A（ファイルを先に）" "サイズ 小 → 大（ファイルを先に）" "サイズ 大 → 小（ファイルを先に）" "並べ替え" "最上部へ移動" "上へ移動" "下へ移動" "最下部へ移動"
append ko "정렬:" "사용자 지정" "이름 A → Z (파일 먼저)" "이름 Z → A (파일 먼저)" "크기 작은 순 (파일 먼저)" "크기 큰 순 (파일 먼저)" "재정렬" "맨 위로 이동" "위로" "아래로" "맨 아래로 이동"
append nl "Volgorde:" "Aangepast" "Naam A → Z (bestanden eerst)" "Naam Z → A (bestanden eerst)" "Grootte klein naar groot (bestanden eerst)" "Grootte groot naar klein (bestanden eerst)" "Herschikken" "Naar boven" "Omhoog" "Omlaag" "Naar beneden"
append pl "Kolejność:" "Własna" "Nazwa A → Z (najpierw pliki)" "Nazwa Z → A (najpierw pliki)" "Rozmiar rosnąco (najpierw pliki)" "Rozmiar malejąco (najpierw pliki)" "Zmień kolejność" "Na górę" "W górę" "W dół" "Na dół"
append pt-BR "Ordem:" "Personalizado" "Nome A → Z (arquivos primeiro)" "Nome Z → A (arquivos primeiro)" "Tamanho crescente (arquivos primeiro)" "Tamanho decrescente (arquivos primeiro)" "Reordenar" "Mover para o topo" "Para cima" "Para baixo" "Mover para o fim"
append ru "Порядок:" "Свой" "Имя А → Я (файлы сначала)" "Имя Я → А (файлы сначала)" "Размер от меньшего (файлы сначала)" "Размер от большего (файлы сначала)" "Переупорядочить" "В начало" "Вверх" "Вниз" "В конец"
append tr "Sıralama:" "Özel" "Ad A → Z (önce dosyalar)" "Ad Z → A (önce dosyalar)" "Boyut küçükten büyüğe (önce dosyalar)" "Boyut büyükten küçüğe (önce dosyalar)" "Yeniden sırala" "En üste taşı" "Yukarı" "Aşağı" "En alta taşı"
append uk "Порядок:" "Власний" "Ім'я А → Я (файли першими)" "Ім'я Я → А (файли першими)" "Розмір від меншого (файли першими)" "Розмір від більшого (файли першими)" "Переупорядкувати" "Нагору" "Вгору" "Вниз" "Донизу"
append vi "Thứ tự:" "Tùy chỉnh" "Tên A → Z (tệp trước)" "Tên Z → A (tệp trước)" "Kích thước nhỏ đến lớn (tệp trước)" "Kích thước lớn đến nhỏ (tệp trước)" "Sắp xếp lại" "Chuyển lên đầu" "Lên" "Xuống" "Chuyển xuống cuối"
append zh-CN "排序：" "自定义" "名称 A → Z（文件优先）" "名称 Z → A（文件优先）" "大小从小到大（文件优先）" "大小从大到小（文件优先）" "重新排序" "移到最上" "上移" "下移" "移到最下"

echo "Sort keys added to 17 locales."
