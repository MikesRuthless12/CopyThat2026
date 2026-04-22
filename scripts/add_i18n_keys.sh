#!/usr/bin/env bash
# One-shot translation drop for the Phase 13d activity / header keys.
# Each locale gets the 11 new keys appended. Translations are
# community-level — native-speaker review welcome but the fallback
# (English) was shipping in its place, so these are strictly better.

set -euo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"

append() {
  local locale="$1"; shift
  local file="$root/locales/$locale/copythat.ftl"
  cat >> "$file" <<EOF

# Phase 13d — activity feed + header picker buttons
action-add-files = $1
action-add-folders = $2
activity-title = $3
activity-clear = $4
activity-empty = $5
activity-after-done = $6
activity-keep-open = $7
activity-close-app = $8
activity-shutdown = $9
activity-logoff = ${10}
activity-sleep = ${11}
EOF
}

append ar  "إضافة ملفات"      "إضافة مجلدات"      "النشاط"        "مسح قائمة النشاط"        "لا يوجد نشاط للملفات بعد."    "عند الانتهاء:"        "إبقاء التطبيق مفتوحًا"   "إغلاق التطبيق"  "إيقاف تشغيل الحاسوب"  "تسجيل الخروج"    "سكون"
append de  "Dateien hinzufügen" "Ordner hinzufügen" "Aktivität"     "Aktivitätsliste leeren"  "Noch keine Dateiaktivität." "Nach Abschluss:"      "Anwendung offen lassen"  "App schließen"  "PC herunterfahren"    "Abmelden"       "Energiesparmodus"
append es  "Añadir archivos"   "Añadir carpetas"   "Actividad"     "Borrar lista de actividad" "Aún no hay actividad."       "Al terminar:"         "Mantener la app abierta" "Cerrar la app"  "Apagar el PC"         "Cerrar sesión"  "Suspender"
append fr  "Ajouter des fichiers" "Ajouter des dossiers" "Activité" "Vider la liste d'activité" "Aucune activité pour le moment." "Une fois terminé :" "Garder l'application ouverte" "Quitter l'application" "Éteindre le PC" "Se déconnecter" "Mettre en veille"
append hi  "फ़ाइलें जोड़ें"       "फ़ोल्डर जोड़ें"         "गतिविधि"        "गतिविधि सूची साफ़ करें"      "अभी तक कोई गतिविधि नहीं."      "पूरा होने पर:"          "ऐप खुला रखें"              "ऐप बंद करें"       "पीसी बंद करें"          "लॉग ऑफ़"          "निद्रा"
append id  "Tambah file"       "Tambah folder"     "Aktivitas"     "Bersihkan daftar aktivitas" "Belum ada aktivitas file."  "Saat selesai:"        "Biarkan aplikasi terbuka" "Tutup aplikasi" "Matikan PC"           "Keluar"         "Tidur"
append it  "Aggiungi file"     "Aggiungi cartelle" "Attività"      "Svuota elenco attività"  "Nessuna attività file."      "Al termine:"          "Mantieni l'app aperta"   "Chiudi app"      "Spegni PC"            "Disconnetti"    "Sospendi"
append ja  "ファイルを追加"     "フォルダを追加"     "アクティビティ" "アクティビティを消去"     "まだファイルアクティビティはありません。" "完了時:"       "アプリを開いたままにする"  "アプリを閉じる"  "PC をシャットダウン"   "ログオフ"       "スリープ"
append ko  "파일 추가"          "폴더 추가"          "활동"           "활동 목록 지우기"         "아직 파일 활동이 없습니다."     "완료 시:"             "앱 열어두기"              "앱 닫기"         "PC 종료"              "로그오프"       "절전 모드"
append nl  "Bestanden toevoegen" "Mappen toevoegen" "Activiteit"    "Activiteitenlijst wissen" "Nog geen bestandsactiviteit." "Bij voltooiing:"      "App open laten"          "App sluiten"    "PC afsluiten"         "Afmelden"       "Slaapstand"
append pl  "Dodaj pliki"       "Dodaj foldery"     "Aktywność"     "Wyczyść listę aktywności" "Brak aktywności plików."    "Po zakończeniu:"      "Zostaw aplikację otwartą" "Zamknij aplikację" "Wyłącz PC"         "Wyloguj"        "Uśpij"
append pt-BR "Adicionar arquivos" "Adicionar pastas" "Atividade"   "Limpar lista de atividade" "Ainda não há atividade."  "Ao concluir:"         "Manter o app aberto"     "Fechar o app"   "Desligar o PC"        "Sair da sessão" "Suspender"
append ru  "Добавить файлы"    "Добавить папки"    "Активность"    "Очистить список активности" "Файловой активности пока нет." "По завершении:"  "Оставить приложение открытым" "Закрыть приложение" "Выключить ПК" "Выйти из системы" "Спящий режим"
append tr  "Dosya ekle"         "Klasör ekle"      "Etkinlik"      "Etkinlik listesini temizle" "Henüz dosya etkinliği yok." "Bittiğinde:"          "Uygulamayı açık tut"     "Uygulamayı kapat" "Bilgisayarı kapat" "Oturumu kapat"  "Uyku moduna al"
append uk  "Додати файли"       "Додати папки"     "Активність"    "Очистити список активності" "Поки немає файлової активності." "Після завершення:" "Залишити застосунок відкритим" "Закрити застосунок" "Вимкнути ПК" "Вийти з системи" "Сплячий режим"
append vi  "Thêm tệp"           "Thêm thư mục"     "Hoạt động"     "Xoá danh sách hoạt động" "Chưa có hoạt động tệp."      "Khi hoàn tất:"        "Giữ ứng dụng mở"         "Đóng ứng dụng"  "Tắt máy"              "Đăng xuất"      "Ngủ"
append zh-CN "添加文件"           "添加文件夹"        "活动"           "清除活动列表"             "暂无文件活动。"              "完成后:"              "保持应用打开"            "关闭应用"       "关机"                 "注销"           "睡眠"

echo "Added 11 keys to 17 locales."
