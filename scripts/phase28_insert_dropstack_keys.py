#!/usr/bin/env python3
"""Phase 28 — insert tray-resident Drop Stack Fluent keys into the
17 non-English locales. MT-flagged drafts matching the Standing
Per-Phase Rules. The 14 keys live as a single block at the end of
each locale; the English source in locales/en/copythat.ftl is
authoritative."""

from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
LOCALES_DIR = ROOT / "locales"

# 14 keys × 17 locales. MT-flagged; human review tracked in
# docs/I18N_TODO.md.
TRANSLATIONS = {
    "ar": {
        "title": "Drop Stack",
        "tray": "Drop Stack",
        "empty_title": "مكدس الإسقاط فارغ",
        "empty_hint": "اسحب الملفات هنا من المستكشف أو انقر بزر الماوس الأيمن على صف المهمة لإضافته.",
        "add": "إضافة إلى مكدس الإسقاط",
        "copy_all": "نسخ الكل إلى…",
        "move_all": "نقل الكل إلى…",
        "clear": "مسح المكدس",
        "remove": "إزالة من المكدس",
        "missing": "تم إسقاط { $path } — الملف لم يعد موجودًا.",
        "aot": "إبقاء مكدس الإسقاط دائمًا في المقدمة",
        "tray_show": "إظهار أيقونة علبة Copy That",
        "open_on_start": "فتح مكدس الإسقاط تلقائيًا عند بدء التطبيق",
        "count": "{ $count } مسار",
    },
    "de": {
        "title": "Drop Stack",
        "tray": "Drop Stack",
        "empty_title": "Drop Stack ist leer",
        "empty_hint": "Ziehen Sie Dateien hierher aus dem Explorer oder klicken Sie mit der rechten Maustaste auf eine Auftragszeile, um sie hinzuzufügen.",
        "add": "Zum Drop Stack hinzufügen",
        "copy_all": "Alles kopieren nach…",
        "move_all": "Alles verschieben nach…",
        "clear": "Stapel leeren",
        "remove": "Aus dem Stapel entfernen",
        "missing": "{ $path } entfernt — die Datei existiert nicht mehr.",
        "aot": "Drop Stack immer im Vordergrund halten",
        "tray_show": "Copy That-Symbol im Infobereich anzeigen",
        "open_on_start": "Drop Stack beim App-Start automatisch öffnen",
        "count": "{ $count } Pfad",
    },
    "es": {
        "title": "Pila de arrastre",
        "tray": "Pila de arrastre",
        "empty_title": "La pila de arrastre está vacía",
        "empty_hint": "Arrastra archivos aquí desde el Explorador o haz clic derecho en una fila de trabajo para añadirla.",
        "add": "Añadir a la pila de arrastre",
        "copy_all": "Copiar todo a…",
        "move_all": "Mover todo a…",
        "clear": "Vaciar pila",
        "remove": "Quitar de la pila",
        "missing": "Se quitó { $path } — el archivo ya no existe.",
        "aot": "Mantener la pila de arrastre siempre en primer plano",
        "tray_show": "Mostrar el icono de Copy That en la bandeja",
        "open_on_start": "Abrir la pila de arrastre al iniciar la aplicación",
        "count": "{ $count } ruta",
    },
    "fr": {
        "title": "Pile de glisser",
        "tray": "Pile de glisser",
        "empty_title": "La pile de glisser est vide",
        "empty_hint": "Faites glisser des fichiers ici depuis l'Explorateur ou cliquez-droit sur une ligne de tâche pour l'ajouter.",
        "add": "Ajouter à la pile de glisser",
        "copy_all": "Tout copier vers…",
        "move_all": "Tout déplacer vers…",
        "clear": "Vider la pile",
        "remove": "Retirer de la pile",
        "missing": "{ $path } retiré — le fichier n'existe plus.",
        "aot": "Garder la pile de glisser toujours au premier plan",
        "tray_show": "Afficher l'icône Copy That dans la barre des tâches",
        "open_on_start": "Ouvrir la pile de glisser au démarrage",
        "count": "{ $count } chemin",
    },
    "hi": {
        "title": "ड्रॉप स्टैक",
        "tray": "ड्रॉप स्टैक",
        "empty_title": "ड्रॉप स्टैक खाली है",
        "empty_hint": "एक्सप्लोरर से यहां फ़ाइलें खींचें या जॉब पंक्ति पर दाएं क्लिक करके जोड़ें।",
        "add": "ड्रॉप स्टैक में जोड़ें",
        "copy_all": "सभी को कॉपी करें…",
        "move_all": "सभी को स्थानांतरित करें…",
        "clear": "स्टैक साफ़ करें",
        "remove": "स्टैक से हटाएं",
        "missing": "{ $path } हटाया गया — फ़ाइल अब मौजूद नहीं है।",
        "aot": "ड्रॉप स्टैक को हमेशा शीर्ष पर रखें",
        "tray_show": "Copy That ट्रे आइकन दिखाएं",
        "open_on_start": "ऐप स्टार्ट पर ड्रॉप स्टैक स्वचालित रूप से खोलें",
        "count": "{ $count } पथ",
    },
    "id": {
        "title": "Drop Stack",
        "tray": "Drop Stack",
        "empty_title": "Drop Stack kosong",
        "empty_hint": "Seret file ke sini dari Penjelajah atau klik kanan baris pekerjaan untuk menambahkannya.",
        "add": "Tambahkan ke Drop Stack",
        "copy_all": "Salin semua ke…",
        "move_all": "Pindahkan semua ke…",
        "clear": "Bersihkan tumpukan",
        "remove": "Hapus dari tumpukan",
        "missing": "{ $path } dihapus — file tidak ada lagi.",
        "aot": "Selalu tampilkan Drop Stack di atas",
        "tray_show": "Tampilkan ikon baki Copy That",
        "open_on_start": "Buka Drop Stack secara otomatis saat aplikasi mulai",
        "count": "{ $count } jalur",
    },
    "it": {
        "title": "Pila di trascinamento",
        "tray": "Pila di trascinamento",
        "empty_title": "La pila di trascinamento è vuota",
        "empty_hint": "Trascina i file qui da Esplora risorse o fai clic con il tasto destro su una riga di lavoro per aggiungerla.",
        "add": "Aggiungi alla pila di trascinamento",
        "copy_all": "Copia tutto in…",
        "move_all": "Sposta tutto in…",
        "clear": "Svuota pila",
        "remove": "Rimuovi dalla pila",
        "missing": "{ $path } rimosso — il file non esiste più.",
        "aot": "Mantieni la pila di trascinamento sempre in primo piano",
        "tray_show": "Mostra l'icona di Copy That nell'area di notifica",
        "open_on_start": "Apri la pila di trascinamento all'avvio dell'app",
        "count": "{ $count } percorso",
    },
    "ja": {
        "title": "ドロップスタック",
        "tray": "ドロップスタック",
        "empty_title": "ドロップスタックは空です",
        "empty_hint": "エクスプローラーからファイルをここにドラッグするか、ジョブ行を右クリックして追加します。",
        "add": "ドロップスタックに追加",
        "copy_all": "すべてをコピー…",
        "move_all": "すべてを移動…",
        "clear": "スタックをクリア",
        "remove": "スタックから削除",
        "missing": "{ $path } を削除しました — ファイルは存在しません。",
        "aot": "ドロップスタックを常に最前面に表示",
        "tray_show": "Copy That のトレイアイコンを表示",
        "open_on_start": "アプリ起動時にドロップスタックを自動的に開く",
        "count": "{ $count } 個のパス",
    },
    "ko": {
        "title": "드롭 스택",
        "tray": "드롭 스택",
        "empty_title": "드롭 스택이 비어 있습니다",
        "empty_hint": "탐색기에서 파일을 여기로 끌거나 작업 행을 마우스 오른쪽 버튼으로 클릭하여 추가하세요.",
        "add": "드롭 스택에 추가",
        "copy_all": "모두 복사…",
        "move_all": "모두 이동…",
        "clear": "스택 지우기",
        "remove": "스택에서 제거",
        "missing": "{ $path }을(를) 제거했습니다 — 파일이 더 이상 존재하지 않습니다.",
        "aot": "드롭 스택을 항상 위에 유지",
        "tray_show": "Copy That 트레이 아이콘 표시",
        "open_on_start": "앱 시작 시 드롭 스택 자동 열기",
        "count": "{ $count }개 경로",
    },
    "nl": {
        "title": "Drop Stack",
        "tray": "Drop Stack",
        "empty_title": "Drop Stack is leeg",
        "empty_hint": "Sleep bestanden hier vanuit Verkenner of klik met de rechtermuisknop op een taakrij om deze toe te voegen.",
        "add": "Toevoegen aan Drop Stack",
        "copy_all": "Alles kopiëren naar…",
        "move_all": "Alles verplaatsen naar…",
        "clear": "Stack wissen",
        "remove": "Uit stack verwijderen",
        "missing": "{ $path } verwijderd — het bestand bestaat niet meer.",
        "aot": "Drop Stack altijd op voorgrond houden",
        "tray_show": "Copy That-systeemvakpictogram weergeven",
        "open_on_start": "Drop Stack automatisch openen bij app-start",
        "count": "{ $count } pad",
    },
    "pl": {
        "title": "Drop Stack",
        "tray": "Drop Stack",
        "empty_title": "Drop Stack jest pusty",
        "empty_hint": "Przeciągnij pliki tutaj z Eksploratora lub kliknij prawym przyciskiem myszy wiersz zadania, aby go dodać.",
        "add": "Dodaj do Drop Stack",
        "copy_all": "Skopiuj wszystko do…",
        "move_all": "Przenieś wszystko do…",
        "clear": "Wyczyść stos",
        "remove": "Usuń ze stosu",
        "missing": "Usunięto { $path } — plik już nie istnieje.",
        "aot": "Trzymaj Drop Stack zawsze na wierzchu",
        "tray_show": "Pokaż ikonę Copy That w obszarze powiadomień",
        "open_on_start": "Otwórz Drop Stack automatycznie przy starcie aplikacji",
        "count": "{ $count } ścieżka",
    },
    "pt-BR": {
        "title": "Pilha de arraste",
        "tray": "Pilha de arraste",
        "empty_title": "A pilha de arraste está vazia",
        "empty_hint": "Arraste arquivos aqui do Explorador ou clique com o botão direito em uma linha de trabalho para adicioná-la.",
        "add": "Adicionar à pilha de arraste",
        "copy_all": "Copiar tudo para…",
        "move_all": "Mover tudo para…",
        "clear": "Limpar pilha",
        "remove": "Remover da pilha",
        "missing": "{ $path } removido — o arquivo não existe mais.",
        "aot": "Manter a pilha de arraste sempre no topo",
        "tray_show": "Mostrar o ícone do Copy That na bandeja",
        "open_on_start": "Abrir a pilha de arraste automaticamente ao iniciar o app",
        "count": "{ $count } caminho",
    },
    "ru": {
        "title": "Drop Stack",
        "tray": "Drop Stack",
        "empty_title": "Drop Stack пуст",
        "empty_hint": "Перетащите файлы сюда из Проводника или щелкните правой кнопкой мыши по строке задания, чтобы добавить её.",
        "add": "Добавить в Drop Stack",
        "copy_all": "Скопировать всё в…",
        "move_all": "Переместить всё в…",
        "clear": "Очистить стек",
        "remove": "Удалить из стека",
        "missing": "{ $path } удалён — файл больше не существует.",
        "aot": "Всегда держать Drop Stack поверх других окон",
        "tray_show": "Показывать значок Copy That в области уведомлений",
        "open_on_start": "Открывать Drop Stack автоматически при запуске приложения",
        "count": "{ $count } путь",
    },
    "tr": {
        "title": "Drop Stack",
        "tray": "Drop Stack",
        "empty_title": "Drop Stack boş",
        "empty_hint": "Dosyaları Gezgin'den buraya sürükleyin veya bir iş satırına sağ tıklayarak ekleyin.",
        "add": "Drop Stack'e ekle",
        "copy_all": "Hepsini kopyala…",
        "move_all": "Hepsini taşı…",
        "clear": "Yığını temizle",
        "remove": "Yığından çıkar",
        "missing": "{ $path } kaldırıldı — dosya artık yok.",
        "aot": "Drop Stack'i her zaman en üstte tut",
        "tray_show": "Copy That tepsi simgesini göster",
        "open_on_start": "Uygulama başlangıcında Drop Stack'i otomatik aç",
        "count": "{ $count } yol",
    },
    "uk": {
        "title": "Drop Stack",
        "tray": "Drop Stack",
        "empty_title": "Drop Stack порожній",
        "empty_hint": "Перетягніть файли сюди з Провідника або клацніть правою кнопкою миші по рядку завдання, щоб додати його.",
        "add": "Додати до Drop Stack",
        "copy_all": "Копіювати все в…",
        "move_all": "Перемістити все в…",
        "clear": "Очистити стек",
        "remove": "Видалити зі стеку",
        "missing": "{ $path } видалено — файл більше не існує.",
        "aot": "Завжди тримати Drop Stack зверху",
        "tray_show": "Показувати значок Copy That в області сповіщень",
        "open_on_start": "Автоматично відкривати Drop Stack під час запуску програми",
        "count": "{ $count } шлях",
    },
    "vi": {
        "title": "Drop Stack",
        "tray": "Drop Stack",
        "empty_title": "Drop Stack trống",
        "empty_hint": "Kéo tệp vào đây từ Explorer hoặc nhấp chuột phải vào một dòng công việc để thêm.",
        "add": "Thêm vào Drop Stack",
        "copy_all": "Sao chép tất cả đến…",
        "move_all": "Di chuyển tất cả đến…",
        "clear": "Xóa ngăn xếp",
        "remove": "Xóa khỏi ngăn xếp",
        "missing": "Đã xóa { $path } — tệp không còn tồn tại.",
        "aot": "Luôn giữ Drop Stack ở trên cùng",
        "tray_show": "Hiển thị biểu tượng khay Copy That",
        "open_on_start": "Tự động mở Drop Stack khi ứng dụng khởi động",
        "count": "{ $count } đường dẫn",
    },
    "zh-CN": {
        "title": "拖放堆栈",
        "tray": "拖放堆栈",
        "empty_title": "拖放堆栈为空",
        "empty_hint": "从资源管理器拖动文件到此处,或右键单击作业行以添加。",
        "add": "添加到拖放堆栈",
        "copy_all": "全部复制到…",
        "move_all": "全部移动到…",
        "clear": "清空堆栈",
        "remove": "从堆栈移除",
        "missing": "已移除 { $path } — 文件不再存在。",
        "aot": "拖放堆栈始终置顶",
        "tray_show": "显示 Copy That 托盘图标",
        "open_on_start": "应用启动时自动打开拖放堆栈",
        "count": "{ $count } 路径",
    },
}


def build_block(t: dict) -> str:
    return (
        "\n"
        "# Phase 28 — tray-resident Drop Stack. MT-flagged drafts;\n"
        "# the authoritative English source lives in locales/en/copythat.ftl.\n"
        f"dropstack-window-title = {t['title']}  # MT\n"
        f"dropstack-tray-open = {t['tray']}  # MT\n"
        f"dropstack-empty-title = {t['empty_title']}  # MT\n"
        f"dropstack-empty-hint = {t['empty_hint']}  # MT\n"
        f"dropstack-add-to-stack = {t['add']}  # MT\n"
        f"dropstack-copy-all-to = {t['copy_all']}  # MT\n"
        f"dropstack-move-all-to = {t['move_all']}  # MT\n"
        f"dropstack-clear = {t['clear']}  # MT\n"
        f"dropstack-remove-row = {t['remove']}  # MT\n"
        f"dropstack-path-missing-toast = {t['missing']}  # MT\n"
        f"dropstack-always-on-top = {t['aot']}  # MT\n"
        f"dropstack-show-tray-icon = {t['tray_show']}  # MT\n"
        f"dropstack-open-on-start = {t['open_on_start']}  # MT\n"
        f"dropstack-count = {t['count']}  # MT\n"
    )


def patch_locale(locale: str, t: dict) -> None:
    path = LOCALES_DIR / locale / "copythat.ftl"
    text = path.read_text(encoding="utf-8")
    if "dropstack-window-title" in text:
        print(f"{locale}: already patched, skipping")
        return
    if not text.endswith("\n"):
        text += "\n"
    text += build_block(t)
    path.write_text(text, encoding="utf-8")
    print(f"{locale}: patched")


def main() -> None:
    for locale, t in TRANSLATIONS.items():
        patch_locale(locale, t)


if __name__ == "__main__":
    main()
