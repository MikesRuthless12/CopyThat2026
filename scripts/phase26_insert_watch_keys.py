#!/usr/bin/env python3
"""Phase 26 — insert real-time mirror watcher Fluent keys into the 17
non-English locales. MT-flagged drafts matching the Standing Per-Phase
Rules. The 6 keys live as a single block at the end of each locale;
the English source in locales/en/copythat.ftl is authoritative."""

from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
LOCALES_DIR = ROOT / "locales"

# 6 keys × 17 locales. MT-flagged; human review tracked in
# docs/I18N_TODO.md.
TRANSLATIONS = {
    "ar": {
        "start": "بدء المرآة الحية",
        "stop": "إيقاف المرآة الحية",
        "watching": "مراقبة",
        "hint": "إعادة المزامنة تلقائيًا عند كل تغيير في نظام الملفات تم اكتشافه. يستخدم خيطًا واحدًا في الخلفية لكل زوج نشط.",
        "evt_prefix": "تغيير ملف",
        "overflow": "تجاوز مخزن المراقب المؤقت؛ إعادة التعداد للاسترداد",
    },
    "de": {
        "start": "Live-Mirror starten",
        "stop": "Live-Mirror stoppen",
        "watching": "Wird überwacht",
        "hint": "Automatisch bei jeder erkannten Dateisystemänderung erneut synchronisieren. Ein Hintergrund-Thread pro aktivem Paar.",
        "evt_prefix": "Dateiänderung",
        "overflow": "Watcher-Puffer übergelaufen; zur Wiederherstellung wird neu aufgelistet",
    },
    "es": {
        "start": "Iniciar espejo en vivo",
        "stop": "Detener espejo en vivo",
        "watching": "Vigilando",
        "hint": "Resincroniza automáticamente con cada cambio del sistema de archivos detectado. Un hilo en segundo plano por par activo.",
        "evt_prefix": "Cambio de archivo",
        "overflow": "Búfer del observador desbordado; reenumerando para recuperar",
    },
    "fr": {
        "start": "Démarrer le miroir en direct",
        "stop": "Arrêter le miroir en direct",
        "watching": "Surveillance",
        "hint": "Resynchronise automatiquement à chaque changement du système de fichiers détecté. Un thread d'arrière-plan par paire active.",
        "evt_prefix": "Changement de fichier",
        "overflow": "Débordement du tampon du surveillant ; réénumération pour récupérer",
    },
    "hi": {
        "start": "लाइव मिरर प्रारंभ करें",
        "stop": "लाइव मिरर बंद करें",
        "watching": "निगरानी",
        "hint": "हर पहचाने गए फ़ाइल सिस्टम परिवर्तन पर स्वचालित रूप से पुनः-सिंक करें। प्रति सक्रिय जोड़ी एक पृष्ठभूमि थ्रेड।",
        "evt_prefix": "फ़ाइल परिवर्तन",
        "overflow": "मॉनिटर बफ़र अतिप्रवाहित हुआ; पुनर्प्राप्ति के लिए पुनः-सूचीकरण",
    },
    "id": {
        "start": "Mulai cermin langsung",
        "stop": "Hentikan cermin langsung",
        "watching": "Mengawasi",
        "hint": "Sinkronisasi ulang secara otomatis pada setiap perubahan sistem berkas yang terdeteksi. Satu thread latar per pasangan aktif.",
        "evt_prefix": "Perubahan berkas",
        "overflow": "Buffer pengawas meluap; mengenumerasi ulang untuk memulihkan",
    },
    "it": {
        "start": "Avvia mirror live",
        "stop": "Ferma mirror live",
        "watching": "Osservazione",
        "hint": "Risincronizza automaticamente ad ogni modifica del file system rilevata. Un thread in background per ogni coppia attiva.",
        "evt_prefix": "Modifica file",
        "overflow": "Buffer dell'osservatore in overflow; rienumerando per recuperare",
    },
    "ja": {
        "start": "ライブミラーを開始",
        "stop": "ライブミラーを停止",
        "watching": "監視中",
        "hint": "検出されたすべてのファイルシステム変更で自動的に再同期します。アクティブなペアごとに 1 つのバックグラウンド スレッド。",
        "evt_prefix": "ファイルの変更",
        "overflow": "ウォッチャー バッファがオーバーフローしました; 回復のために再列挙中",
    },
    "ko": {
        "start": "실시간 미러 시작",
        "stop": "실시간 미러 중지",
        "watching": "감시 중",
        "hint": "감지된 모든 파일 시스템 변경 시 자동으로 다시 동기화합니다. 활성 쌍당 하나의 백그라운드 스레드.",
        "evt_prefix": "파일 변경",
        "overflow": "감시자 버퍼 오버플로; 복구를 위해 재열거 중",
    },
    "nl": {
        "start": "Live mirror starten",
        "stop": "Live mirror stoppen",
        "watching": "Kijken",
        "hint": "Synchroniseer automatisch opnieuw bij elke gedetecteerde wijziging in het bestandssysteem. Eén achtergrondthread per actief paar.",
        "evt_prefix": "Bestandswijziging",
        "overflow": "Buffer van waarnemer liep over; opnieuw opsommen om te herstellen",
    },
    "pl": {
        "start": "Uruchom lustro na żywo",
        "stop": "Zatrzymaj lustro na żywo",
        "watching": "Obserwowanie",
        "hint": "Automatyczne ponowne zsynchronizowanie przy każdej wykrytej zmianie w systemie plików. Jeden wątek w tle na aktywną parę.",
        "evt_prefix": "Zmiana pliku",
        "overflow": "Bufor obserwatora przepełniony; ponowne wyliczanie w celu odzyskania",
    },
    "pt-BR": {
        "start": "Iniciar espelho ao vivo",
        "stop": "Parar espelho ao vivo",
        "watching": "Observando",
        "hint": "Ressincroniza automaticamente a cada alteração detectada no sistema de arquivos. Uma thread em segundo plano por par ativo.",
        "evt_prefix": "Alteração de arquivo",
        "overflow": "Buffer do observador transbordou; reenumerando para recuperar",
    },
    "ru": {
        "start": "Запустить живое зеркало",
        "stop": "Остановить живое зеркало",
        "watching": "Наблюдение",
        "hint": "Автоматическая повторная синхронизация при каждом обнаруженном изменении файловой системы. Один фоновый поток на активную пару.",
        "evt_prefix": "Изменение файла",
        "overflow": "Буфер наблюдателя переполнен; повторное перечисление для восстановления",
    },
    "tr": {
        "start": "Canlı yansıtmayı başlat",
        "stop": "Canlı yansıtmayı durdur",
        "watching": "İzleniyor",
        "hint": "Algılanan her dosya sistemi değişikliğinde otomatik olarak yeniden eşitleme. Etkin çift başına bir arka plan iş parçacığı.",
        "evt_prefix": "Dosya değişikliği",
        "overflow": "İzleyici arabelleği taştı; kurtarmak için yeniden numaralandırılıyor",
    },
    "uk": {
        "start": "Запустити живе дзеркало",
        "stop": "Зупинити живе дзеркало",
        "watching": "Спостереження",
        "hint": "Автоматична повторна синхронізація при кожній виявленій зміні файлової системи. Один фоновий потік на активну пару.",
        "evt_prefix": "Зміна файлу",
        "overflow": "Буфер спостерігача переповнений; повторне перелічення для відновлення",
    },
    "vi": {
        "start": "Bắt đầu gương trực tiếp",
        "stop": "Dừng gương trực tiếp",
        "watching": "Đang theo dõi",
        "hint": "Tự động đồng bộ lại trên mọi thay đổi hệ thống tệp được phát hiện. Một luồng nền cho mỗi cặp hoạt động.",
        "evt_prefix": "Thay đổi tệp",
        "overflow": "Bộ đệm theo dõi bị tràn; liệt kê lại để khôi phục",
    },
    "zh-CN": {
        "start": "启动实时镜像",
        "stop": "停止实时镜像",
        "watching": "监视中",
        "hint": "在每次检测到文件系统更改时自动重新同步。每个活动对一个后台线程。",
        "evt_prefix": "文件更改",
        "overflow": "监视器缓冲区溢出;正在重新枚举以恢复",
    },
}


def build_block(t: dict) -> str:
    return (
        "\n"
        "# Phase 26 — real-time mirror watcher. MT-flagged drafts;\n"
        "# the authoritative English source lives in locales/en/copythat.ftl.\n"
        f"live-mirror-start = {t['start']}  # MT\n"
        f"live-mirror-stop = {t['stop']}  # MT\n"
        f"live-mirror-watching = {t['watching']}  # MT\n"
        f"live-mirror-toggle-hint = {t['hint']}  # MT\n"
        f"watch-event-prefix = {t['evt_prefix']}  # MT\n"
        f"watch-overflow-recovered = {t['overflow']}  # MT\n"
    )


def patch_locale(locale: str, t: dict) -> None:
    path = LOCALES_DIR / locale / "copythat.ftl"
    text = path.read_text(encoding="utf-8")
    if "live-mirror-watching" in text:
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
