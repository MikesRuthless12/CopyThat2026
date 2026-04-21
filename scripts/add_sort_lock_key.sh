#!/usr/bin/env bash
set -euo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
append() {
  local locale="$1"; shift
  echo "activity-sort-locked = $1" >> "$root/locales/$locale/copythat.ftl"
}
append ar "الترتيب معطل أثناء تشغيل النسخ. أوقف مؤقتًا أو انتظر حتى ينتهي ثم غيّر الترتيب."
append de "Sortierung ist deaktiviert, während ein Kopiervorgang läuft. Pausiere ihn oder warte bis zum Ende, dann ändere die Reihenfolge."
append es "La ordenación está desactivada mientras se copia. Pausa o espera a que termine, luego cambia el orden."
append fr "Le tri est désactivé pendant qu'une copie est en cours. Mets en pause ou attends la fin, puis change l'ordre."
append hi "कॉपी चल रही हो तो क्रम अक्षम होता है। रोकें या समाप्त होने का इंतज़ार करें, फिर क्रम बदलें।"
append id "Pengurutan dinonaktifkan saat penyalinan berjalan. Jeda atau tunggu hingga selesai, lalu ubah urutan."
append it "L'ordinamento è disabilitato durante una copia. Metti in pausa o attendi la fine, poi cambia l'ordine."
append ja "コピー中は並び替えを無効にしています。一時停止するか終了を待ってから順序を変更してください。"
append ko "복사가 실행 중일 때는 정렬이 비활성화됩니다. 일시중지하거나 완료를 기다린 후 순서를 변경하세요."
append nl "Sorteren is uitgeschakeld terwijl een kopie loopt. Pauzeer of wacht tot het klaar is en wijzig dan de volgorde."
append pl "Sortowanie jest wyłączone podczas trwania kopii. Wstrzymaj lub poczekaj na zakończenie, potem zmień kolejność."
append pt-BR "A ordenação está desativada enquanto uma cópia está em andamento. Pause ou aguarde terminar, depois mude a ordem."
append ru "Сортировка отключена во время копирования. Приостановите или дождитесь окончания, затем измените порядок."
append tr "Kopyalama sırasında sıralama devre dışıdır. Duraklatın veya bitmesini bekleyin, sonra sırayı değiştirin."
append uk "Сортування вимкнено під час копіювання. Призупиніть або дочекайтеся завершення, потім змініть порядок."
append vi "Không thể sắp xếp khi đang sao chép. Tạm dừng hoặc đợi hoàn tất rồi thay đổi thứ tự."
append zh-CN "复制进行中时禁用排序。暂停或等待完成后再更改顺序。"
echo "activity-sort-locked added to 17 locales."
