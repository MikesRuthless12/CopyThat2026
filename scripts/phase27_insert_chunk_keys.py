#!/usr/bin/env python3
"""Phase 27 — insert content-defined chunk store Fluent keys into the
17 non-English locales. MT-flagged drafts matching the Standing
Per-Phase Rules. The 8 keys live as a single block at the end of each
locale; the English source in locales/en/copythat.ftl is
authoritative."""

from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
LOCALES_DIR = ROOT / "locales"

# 8 keys × 17 locales. MT-flagged; human review tracked in
# docs/I18N_TODO.md.
TRANSLATIONS = {
    "ar": {
        "section": "مخزن الأجزاء",
        "enable": "تمكين مخزن الأجزاء (استئناف التغييرات + إزالة التكرار)",
        "hint": "يقسم كل ملف منسوخ حسب المحتوى (FastCDC) ويخزن الأجزاء بمعالجة المحتوى. تعيد المحاولات كتابة الأجزاء المتغيرة فقط؛ الملفات ذات المحتوى المشترك تزيل التكرار تلقائيًا.",
        "location": "موقع مخزن الأجزاء",
        "max_size": "الحد الأقصى لحجم مخزن الأجزاء",
        "prune": "تنظيف الأجزاء الأقدم من (أيام)",
        "savings": "تم توفير { $gib } جيجابايت عبر إزالة تكرار الأجزاء",
        "usage": "يستخدم { $size } عبر { $chunks } جزءًا",
    },
    "de": {
        "section": "Chunk-Speicher",
        "enable": "Chunk-Speicher aktivieren (Delta-Fortsetzung und Deduplizierung)",
        "hint": "Teilt jede kopierte Datei nach Inhalt (FastCDC) und speichert Chunks inhaltsadressiert. Wiederholungen schreiben nur geänderte Chunks neu; Dateien mit gemeinsamen Inhalten werden automatisch dedupliziert.",
        "location": "Chunk-Speicher-Ort",
        "max_size": "Maximale Chunk-Speichergröße",
        "prune": "Chunks bereinigen älter als (Tage)",
        "savings": "{ $gib } GiB durch Chunk-Deduplizierung gespart",
        "usage": "Belegt { $size } in { $chunks } Chunks",
    },
    "es": {
        "section": "Almacén de fragmentos",
        "enable": "Activar almacén de fragmentos (reanudación delta y deduplicación)",
        "hint": "Divide cada archivo copiado por contenido (FastCDC) y almacena fragmentos direccionados por contenido. Los reintentos solo reescriben los fragmentos modificados; los archivos con contenido compartido se deduplican automáticamente.",
        "location": "Ubicación del almacén de fragmentos",
        "max_size": "Tamaño máximo del almacén de fragmentos",
        "prune": "Eliminar fragmentos más antiguos que (días)",
        "savings": "Ahorrados { $gib } GiB mediante deduplicación de fragmentos",
        "usage": "Usando { $size } en { $chunks } fragmentos",
    },
    "fr": {
        "section": "Magasin de blocs",
        "enable": "Activer le magasin de blocs (reprise delta et déduplication)",
        "hint": "Divise chaque fichier copié par contenu (FastCDC) et stocke les blocs par adresse de contenu. Les tentatives de reprise ne réécrivent que les blocs modifiés ; les fichiers avec du contenu partagé sont dédupliqués automatiquement.",
        "location": "Emplacement du magasin de blocs",
        "max_size": "Taille maximale du magasin de blocs",
        "prune": "Élaguer les blocs plus anciens que (jours)",
        "savings": "Économisé { $gib } Gio via la déduplication de blocs",
        "usage": "Utilise { $size } sur { $chunks } blocs",
    },
    "hi": {
        "section": "चंक स्टोर",
        "enable": "चंक स्टोर सक्षम करें (डेल्टा-रिज़्यूम और डीडुप)",
        "hint": "प्रत्येक कॉपी की गई फ़ाइल को सामग्री के अनुसार विभाजित करता है (FastCDC) और चंक्स को सामग्री-एड्रेस्ड के रूप में संग्रहीत करता है। पुनः प्रयास केवल बदले गए चंक्स को फिर से लिखते हैं; साझा सामग्री वाली फ़ाइलें स्वचालित रूप से डीडुप हो जाती हैं।",
        "location": "चंक स्टोर स्थान",
        "max_size": "अधिकतम चंक स्टोर आकार",
        "prune": "इस से पुराने चंक्स को हटाएं (दिन)",
        "savings": "चंक डीडुप के माध्यम से { $gib } GiB बचाया गया",
        "usage": "{ $chunks } चंक्स में { $size } का उपयोग",
    },
    "id": {
        "section": "Penyimpanan chunk",
        "enable": "Aktifkan penyimpanan chunk (delta-resume dan deduplikasi)",
        "hint": "Membagi setiap berkas yang disalin berdasarkan konten (FastCDC) dan menyimpan chunk dengan pengalamatan konten. Percobaan ulang hanya menulis ulang chunk yang berubah; berkas dengan konten bersama diduplikasi secara otomatis.",
        "location": "Lokasi penyimpanan chunk",
        "max_size": "Ukuran maksimum penyimpanan chunk",
        "prune": "Pangkas chunk yang lebih lama dari (hari)",
        "savings": "Hemat { $gib } GiB melalui deduplikasi chunk",
        "usage": "Menggunakan { $size } dalam { $chunks } chunk",
    },
    "it": {
        "section": "Archivio chunk",
        "enable": "Abilita archivio chunk (ripresa delta e deduplicazione)",
        "hint": "Divide ogni file copiato per contenuto (FastCDC) e memorizza i chunk indirizzati per contenuto. I tentativi riscrivono solo i chunk modificati; i file con contenuto condiviso vengono deduplicati automaticamente.",
        "location": "Percorso dell'archivio chunk",
        "max_size": "Dimensione massima dell'archivio chunk",
        "prune": "Elimina chunk più vecchi di (giorni)",
        "savings": "Risparmiati { $gib } GiB tramite deduplicazione chunk",
        "usage": "In uso { $size } in { $chunks } chunk",
    },
    "ja": {
        "section": "チャンクストア",
        "enable": "チャンクストアを有効化 (差分再開と重複排除)",
        "hint": "コピーされるすべてのファイルをコンテンツ (FastCDC) で分割し、コンテンツアドレス指定されたチャンクとして保存します。再試行では変更されたチャンクのみ書き換えられます。コンテンツを共有するファイルは自動的に重複排除されます。",
        "location": "チャンクストアの場所",
        "max_size": "チャンクストアの最大サイズ",
        "prune": "以下の日数より古いチャンクを削除 (日)",
        "savings": "チャンク重複排除により { $gib } GiB を節約",
        "usage": "{ $chunks } 個のチャンクで { $size } を使用中",
    },
    "ko": {
        "section": "청크 저장소",
        "enable": "청크 저장소 활성화 (델타 재개 및 중복 제거)",
        "hint": "복사되는 모든 파일을 콘텐츠별로 분할 (FastCDC)하고 청크를 콘텐츠 주소 지정 방식으로 저장합니다. 재시도는 변경된 청크만 다시 씁니다. 공유 콘텐츠가 있는 파일은 자동으로 중복 제거됩니다.",
        "location": "청크 저장소 위치",
        "max_size": "청크 저장소 최대 크기",
        "prune": "다음 일수보다 오래된 청크 정리 (일)",
        "savings": "청크 중복 제거를 통해 { $gib } GiB 절약됨",
        "usage": "{ $chunks }개 청크에 { $size } 사용 중",
    },
    "nl": {
        "section": "Chunk-opslag",
        "enable": "Chunk-opslag inschakelen (delta-hervatting en deduplicatie)",
        "hint": "Splitst elk gekopieerd bestand op inhoud (FastCDC) en slaat chunks inhoudsgeadresseerd op. Herhalingen herschrijven alleen gewijzigde chunks; bestanden met gedeelde inhoud worden automatisch gededupliceerd.",
        "location": "Locatie van chunk-opslag",
        "max_size": "Maximale grootte van chunk-opslag",
        "prune": "Chunks ouder dan (dagen) opruimen",
        "savings": "{ $gib } GiB bespaard via chunk-deduplicatie",
        "usage": "Gebruikt { $size } over { $chunks } chunks",
    },
    "pl": {
        "section": "Magazyn fragmentów",
        "enable": "Włącz magazyn fragmentów (wznawianie delta i deduplikacja)",
        "hint": "Dzieli każdy kopiowany plik według treści (FastCDC) i przechowuje fragmenty adresowane treścią. Ponowne próby przepisują tylko zmienione fragmenty; pliki z udostępnioną treścią są automatycznie deduplikowane.",
        "location": "Lokalizacja magazynu fragmentów",
        "max_size": "Maksymalny rozmiar magazynu fragmentów",
        "prune": "Usuń fragmenty starsze niż (dni)",
        "savings": "Zaoszczędzono { $gib } GiB dzięki deduplikacji fragmentów",
        "usage": "Używa { $size } w { $chunks } fragmentach",
    },
    "pt-BR": {
        "section": "Armazenamento de blocos",
        "enable": "Ativar armazenamento de blocos (retomada delta e deduplicação)",
        "hint": "Divide cada arquivo copiado por conteúdo (FastCDC) e armazena blocos endereçados por conteúdo. Novas tentativas reescrevem apenas os blocos alterados; arquivos com conteúdo compartilhado são deduplicados automaticamente.",
        "location": "Local do armazenamento de blocos",
        "max_size": "Tamanho máximo do armazenamento de blocos",
        "prune": "Remover blocos mais antigos que (dias)",
        "savings": "Economizou { $gib } GiB via deduplicação de blocos",
        "usage": "Usando { $size } em { $chunks } blocos",
    },
    "ru": {
        "section": "Хранилище фрагментов",
        "enable": "Включить хранилище фрагментов (дельта-возобновление и дедупликация)",
        "hint": "Разделяет каждый копируемый файл по содержимому (FastCDC) и сохраняет фрагменты с адресацией по содержимому. Повторные попытки переписывают только изменённые фрагменты; файлы с общим содержимым автоматически дедуплицируются.",
        "location": "Расположение хранилища фрагментов",
        "max_size": "Максимальный размер хранилища фрагментов",
        "prune": "Удалять фрагменты старше (дней)",
        "savings": "Сэкономлено { $gib } ГиБ благодаря дедупликации фрагментов",
        "usage": "Используется { $size } в { $chunks } фрагментах",
    },
    "tr": {
        "section": "Parça deposu",
        "enable": "Parça deposunu etkinleştir (delta devam ve tekilleştirme)",
        "hint": "Her kopyalanan dosyayı içeriğe göre böler (FastCDC) ve parçaları içerik-adresli olarak depolar. Yeniden denemeler yalnızca değişen parçaları yeniden yazar; paylaşılan içeriğe sahip dosyalar otomatik olarak tekilleştirilir.",
        "location": "Parça deposu konumu",
        "max_size": "Maksimum parça deposu boyutu",
        "prune": "Şundan eski parçaları temizle (gün)",
        "savings": "Parça tekilleştirme ile { $gib } GiB tasarruf edildi",
        "usage": "{ $chunks } parçada { $size } kullanılıyor",
    },
    "uk": {
        "section": "Сховище фрагментів",
        "enable": "Увімкнути сховище фрагментів (дельта-відновлення та дедуплікація)",
        "hint": "Розділяє кожен скопійований файл за вмістом (FastCDC) і зберігає фрагменти з адресацією за вмістом. Повторні спроби перезаписують лише змінені фрагменти; файли зі спільним вмістом автоматично дедуплікуються.",
        "location": "Розташування сховища фрагментів",
        "max_size": "Максимальний розмір сховища фрагментів",
        "prune": "Видаляти фрагменти, старші за (дні)",
        "savings": "Заощаджено { $gib } ГіБ завдяки дедуплікації фрагментів",
        "usage": "Використовується { $size } у { $chunks } фрагментах",
    },
    "vi": {
        "section": "Kho phân đoạn",
        "enable": "Kích hoạt kho phân đoạn (tiếp tục delta và loại bỏ trùng lặp)",
        "hint": "Phân chia mỗi tệp được sao chép theo nội dung (FastCDC) và lưu trữ các phân đoạn được địa chỉ hóa theo nội dung. Các lần thử lại chỉ ghi lại các phân đoạn đã thay đổi; các tệp có nội dung chung được loại bỏ trùng lặp tự động.",
        "location": "Vị trí kho phân đoạn",
        "max_size": "Kích thước tối đa của kho phân đoạn",
        "prune": "Xóa các phân đoạn cũ hơn (ngày)",
        "savings": "Đã tiết kiệm { $gib } GiB nhờ loại bỏ trùng lặp phân đoạn",
        "usage": "Đang sử dụng { $size } trong { $chunks } phân đoạn",
    },
    "zh-CN": {
        "section": "分块存储",
        "enable": "启用分块存储 (增量恢复和去重)",
        "hint": "按内容 (FastCDC) 拆分每个复制的文件并以内容寻址方式存储分块。重试仅重写已更改的分块;具有共享内容的文件自动去重。",
        "location": "分块存储位置",
        "max_size": "最大分块存储大小",
        "prune": "清除早于 (天) 的分块",
        "savings": "通过分块去重节省了 { $gib } GiB",
        "usage": "在 { $chunks } 个分块中使用 { $size }",
    },
}


def build_block(t: dict) -> str:
    return (
        "\n"
        "# Phase 27 — content-defined chunk store. MT-flagged drafts;\n"
        "# the authoritative English source lives in locales/en/copythat.ftl.\n"
        f"chunk-store-section = {t['section']}  # MT\n"
        f"chunk-store-enable = {t['enable']}  # MT\n"
        f"chunk-store-enable-hint = {t['hint']}  # MT\n"
        f"chunk-store-location = {t['location']}  # MT\n"
        f"chunk-store-max-size = {t['max_size']}  # MT\n"
        f"chunk-store-prune = {t['prune']}  # MT\n"
        f"chunk-store-savings = {t['savings']}  # MT\n"
        f"chunk-store-disk-usage = {t['usage']}  # MT\n"
    )


def patch_locale(locale: str, t: dict) -> None:
    path = LOCALES_DIR / locale / "copythat.ftl"
    text = path.read_text(encoding="utf-8")
    if "chunk-store-section" in text:
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
