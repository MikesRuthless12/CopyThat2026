#!/usr/bin/env bash
# Phase 14 — "clear all history" button translation drop.

set -euo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"

append() {
  local locale="$1"; shift
  local file="$root/locales/$locale/copythat.ftl"
  cat >> "$file" <<EOF
history-clear-all = $1
history-clear-all-confirm = $2
history-clear-all-hint = $3
toast-history-cleared = $4
EOF
}

append ar "مسح الكل" "انقر مرة أخرى للتأكيد" "حذف جميع صفوف السجل. يتطلب نقرة ثانية للتأكيد." "تم مسح السجل ({ \$count } صفوف أُزيلت)"
append de "Alles löschen" "Zum Bestätigen erneut klicken" "Alle Verlaufszeilen löschen. Ein zweiter Klick bestätigt." "Verlauf gelöscht ({ \$count } Zeilen entfernt)"
append es "Borrar todo" "Haz clic de nuevo para confirmar" "Elimina todas las filas del historial. Requiere un segundo clic para confirmar." "Historial borrado ({ \$count } filas eliminadas)"
append fr "Tout effacer" "Cliquez à nouveau pour confirmer" "Supprime toutes les lignes de l'historique. Un deuxième clic confirme." "Historique effacé ({ \$count } lignes supprimées)"
append hi "सब हटाएँ" "पुष्टि के लिए फिर क्लिक करें" "हर इतिहास पंक्ति हटाएँ। पुष्टि के लिए दूसरा क्लिक आवश्यक है।" "इतिहास साफ़ किया गया ({ \$count } पंक्तियाँ हटाई गईं)"
append id "Hapus semua" "Klik lagi untuk konfirmasi" "Menghapus setiap baris riwayat. Butuh klik kedua untuk konfirmasi." "Riwayat dibersihkan ({ \$count } baris dihapus)"
append it "Cancella tutto" "Fai di nuovo clic per confermare" "Elimina ogni riga della cronologia. Richiede un secondo clic per confermare." "Cronologia cancellata ({ \$count } righe rimosse)"
append ja "すべて消去" "もう一度クリックして確認" "すべての履歴行を削除します。確認のためもう一度クリックが必要です。" "履歴を消去しました ({ \$count } 行削除)"
append ko "전체 지우기" "확인하려면 다시 클릭" "모든 기록 행을 삭제합니다. 두 번째 클릭으로 확인합니다." "기록이 지워졌습니다 ({ \$count } 행 제거됨)"
append nl "Alles wissen" "Klik opnieuw om te bevestigen" "Verwijdert elke geschiedenisrij. Een tweede klik bevestigt." "Geschiedenis gewist ({ \$count } rijen verwijderd)"
append pl "Wyczyść wszystko" "Kliknij ponownie, aby potwierdzić" "Usuwa każdy wiersz historii. Drugie kliknięcie potwierdza." "Historia wyczyszczona ({ \$count } wierszy usunięto)"
append pt-BR "Limpar tudo" "Clique novamente para confirmar" "Exclui todas as linhas do histórico. Requer um segundo clique para confirmar." "Histórico limpo ({ \$count } linhas removidas)"
append ru "Очистить всё" "Нажмите ещё раз для подтверждения" "Удаляет все строки истории. Требуется второе нажатие для подтверждения." "История очищена ({ \$count } строк удалено)"
append tr "Tümünü temizle" "Onaylamak için tekrar tıkla" "Tüm geçmiş satırlarını siler. Onaylamak için ikinci bir tıklama gerekir." "Geçmiş temizlendi ({ \$count } satır kaldırıldı)"
append uk "Очистити все" "Натисніть ще раз для підтвердження" "Видаляє всі рядки історії. Потрібне друге натискання для підтвердження." "Історію очищено ({ \$count } рядків видалено)"
append vi "Xoá tất cả" "Nhấp lần nữa để xác nhận" "Xoá mọi dòng lịch sử. Cần nhấp lần hai để xác nhận." "Đã xoá lịch sử (đã xoá { \$count } dòng)"
append zh-CN "全部清除" "再次点击以确认" "删除每一行历史记录。需要第二次点击确认。" "已清除历史（删除 { \$count } 行）"

echo "Clear-all keys added to 17 locales."
