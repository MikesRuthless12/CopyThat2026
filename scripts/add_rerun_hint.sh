#!/usr/bin/env bash
# Phase 14 — rerun tooltip hint translation drop.

set -euo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"

append() {
  local locale="$1"; shift
  local file="$root/locales/$locale/copythat.ftl"
  cat >> "$file" <<EOF
history-rerun-hint = $1
EOF
}

append ar "أعد تشغيل هذا النسخ — يعيد فحص كل ملف في شجرة المصدر"
append de "Diesen Kopiervorgang erneut ausführen — scannt alle Dateien im Quellbaum"
append es "Volver a ejecutar esta copia — vuelve a escanear todos los archivos del origen"
append fr "Relancer cette copie — analyse à nouveau tous les fichiers de la source"
append hi "इस कॉपी को फिर से चलाएँ — स्रोत ट्री में हर फ़ाइल को फिर से स्कैन करता है"
append id "Jalankan ulang salinan ini — memindai ulang setiap file di pohon sumber"
append it "Riesegui questa copia — ripete la scansione di ogni file nell'origine"
append ja "このコピーを再実行 — ソースツリー内のすべてのファイルを再スキャン"
append ko "이 복사 재실행 — 원본 트리의 모든 파일을 다시 스캔"
append nl "Deze kopie opnieuw uitvoeren — scant alle bestanden in de bronboom opnieuw"
append pl "Uruchom tę kopię ponownie — ponownie skanuje każdy plik w drzewie źródłowym"
append pt-BR "Executar esta cópia novamente — reexamina cada arquivo na árvore de origem"
append ru "Повторить эту копию — заново сканирует все файлы в дереве источника"
append tr "Bu kopyayı yeniden çalıştır — kaynak ağacındaki her dosyayı yeniden tarar"
append uk "Повторити це копіювання — знову сканує всі файли в дереві джерела"
append vi "Chạy lại bản sao này — quét lại mọi tệp trong cây nguồn"
append zh-CN "重新运行此复制 — 重新扫描源树中的每个文件"

echo "history-rerun-hint added to 17 locales."
