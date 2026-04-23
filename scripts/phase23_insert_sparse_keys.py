#!/usr/bin/env python3
"""Phase 23 — insert sparse-file Fluent keys into the 17 non-English
locales. MT-flagged drafts matching the Standing Per-Phase Rules."""

from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
LOCALES_DIR = ROOT / "locales"

# Per-locale MT drafts. English lives in the authoritative file and
# is not regenerated here.
TRANSLATIONS = {
    "ar": {
        "err_mismatch": "لا يمكن الحفاظ على تخطيط الملف المتناثر في الوجهة",
        "settings_label": "الاحتفاظ بالملفات المتناثرة",
        "settings_hint": "انسخ فقط النطاقات المخصصة للملفات المتناثرة (أقراص الجهاز الظاهري، ملفات قاعدة البيانات) حتى يظل حجم الوجهة على القرص هو نفسه حجم المصدر.",
        "toast_title": "الوجهة تملأ الملفات المتناثرة",
        "toast_body": "{ $dst_fs } لا يدعم الملفات المتناثرة. تمت كتابة الثقوب الموجودة في المصدر كأصفار، لذا فإن حجم الوجهة على القرص أكبر.",
        "warn_densified": "تم الحفاظ على تخطيط الملف المتناثر: تم نسخ النطاقات المخصصة فقط.",
        "warn_mismatch": "عدم تطابق تخطيط الملف المتناثر — قد تكون الوجهة أكبر من المتوقع.",
    },
    "de": {
        "err_mismatch": "Sparse-Layout konnte am Ziel nicht beibehalten werden",
        "settings_label": "Sparse-Dateien bewahren",
        "settings_hint": "Bei Sparse-Dateien (VM-Disks, Datenbankdateien) werden nur die zugewiesenen Bereiche kopiert, sodass das Ziel dieselbe Größe auf dem Datenträger wie die Quelle behält.",
        "toast_title": "Ziel füllt Sparse-Dateien aus",
        "toast_body": "{ $dst_fs } unterstützt keine Sparse-Dateien. Löcher in der Quelle wurden als Nullen geschrieben, daher ist das Ziel auf dem Datenträger größer.",
        "warn_densified": "Sparse-Layout beibehalten: nur zugewiesene Bereiche wurden kopiert.",
        "warn_mismatch": "Sparse-Layout stimmt nicht überein — Ziel könnte größer als erwartet sein.",
    },
    "es": {
        "err_mismatch": "No se pudo preservar el diseño disperso en el destino",
        "settings_label": "Preservar archivos dispersos",
        "settings_hint": "Copie solo las extensiones asignadas de los archivos dispersos (discos de VM, archivos de base de datos) para que el tamaño en disco del destino sea igual al origen.",
        "toast_title": "El destino rellena los archivos dispersos",
        "toast_body": "{ $dst_fs } no admite archivos dispersos. Los huecos del origen se escribieron como ceros, por lo que el destino ocupa más espacio en disco.",
        "warn_densified": "Diseño disperso preservado: solo se copiaron las extensiones asignadas.",
        "warn_mismatch": "Desajuste de diseño disperso — el destino puede ser mayor de lo esperado.",
    },
    "fr": {
        "err_mismatch": "La disposition clairsemée n'a pas pu être préservée sur la destination",
        "settings_label": "Préserver les fichiers clairsemés",
        "settings_hint": "Copier uniquement les étendues allouées des fichiers clairsemés (disques de VM, fichiers de base de données) pour que la destination conserve la même taille sur disque que la source.",
        "toast_title": "La destination remplit les fichiers clairsemés",
        "toast_body": "{ $dst_fs } ne prend pas en charge les fichiers clairsemés. Les trous de la source ont été écrits sous forme de zéros, donc la destination est plus grande sur disque.",
        "warn_densified": "Disposition clairsemée préservée : seules les étendues allouées ont été copiées.",
        "warn_mismatch": "Incompatibilité de disposition clairsemée — la destination peut être plus grande que prévu.",
    },
    "hi": {
        "err_mismatch": "गंतव्य पर स्पार्स लेआउट संरक्षित नहीं किया जा सका",
        "settings_label": "स्पार्स फ़ाइलें संरक्षित करें",
        "settings_hint": "स्पार्स फ़ाइलों (VM डिस्क, डेटाबेस फ़ाइलें) के केवल आवंटित विस्तार को कॉपी करें ताकि गंतव्य डिस्क पर स्रोत के समान आकार बना रहे।",
        "toast_title": "गंतव्य स्पार्स फ़ाइलें भरता है",
        "toast_body": "{ $dst_fs } स्पार्स फ़ाइलों का समर्थन नहीं करता। स्रोत में छेदों को शून्य के रूप में लिखा गया, इसलिए गंतव्य डिस्क पर बड़ा है।",
        "warn_densified": "स्पार्स लेआउट संरक्षित: केवल आवंटित विस्तार कॉपी किए गए।",
        "warn_mismatch": "स्पार्स लेआउट मेल नहीं — गंतव्य अपेक्षा से बड़ा हो सकता है।",
    },
    "id": {
        "err_mismatch": "Tata letak sparse tidak dapat dipertahankan di tujuan",
        "settings_label": "Pertahankan file sparse",
        "settings_hint": "Salin hanya rentang yang dialokasikan dari file sparse (disk VM, file database) sehingga ukuran tujuan di disk tetap sama dengan sumber.",
        "toast_title": "Tujuan mengisi file sparse",
        "toast_body": "{ $dst_fs } tidak mendukung file sparse. Lubang di sumber ditulis sebagai nol, sehingga tujuan lebih besar di disk.",
        "warn_densified": "Tata letak sparse dipertahankan: hanya rentang yang dialokasikan yang disalin.",
        "warn_mismatch": "Ketidakcocokan tata letak sparse — tujuan mungkin lebih besar dari yang diharapkan.",
    },
    "it": {
        "err_mismatch": "Layout sparso non preservato sulla destinazione",
        "settings_label": "Preserva i file sparsi",
        "settings_hint": "Copia solo le estensioni allocate dei file sparsi (dischi VM, file di database) in modo che la destinazione mantenga la stessa dimensione su disco dell'origine.",
        "toast_title": "La destinazione riempie i file sparsi",
        "toast_body": "{ $dst_fs } non supporta i file sparsi. I buchi nell'origine sono stati scritti come zeri, quindi la destinazione è più grande su disco.",
        "warn_densified": "Layout sparso preservato: sono state copiate solo le estensioni allocate.",
        "warn_mismatch": "Discordanza layout sparso — la destinazione potrebbe essere più grande del previsto.",
    },
    "ja": {
        "err_mismatch": "宛先でスパースレイアウトを保持できませんでした",
        "settings_label": "スパースファイルを保持",
        "settings_hint": "スパースファイル (VM ディスク、データベースファイル) の割り当て済み範囲のみをコピーし、宛先がソースと同じディスク上のサイズを維持します。",
        "toast_title": "宛先がスパースファイルを埋めます",
        "toast_body": "{ $dst_fs } はスパースファイルをサポートしていません。ソースの穴はゼロとして書き込まれ、宛先はディスク上で大きくなります。",
        "warn_densified": "スパースレイアウトを保持: 割り当て済み範囲のみコピーされました。",
        "warn_mismatch": "スパースレイアウトの不一致 — 宛先が予想より大きくなる可能性があります。",
    },
    "ko": {
        "err_mismatch": "대상에서 스파스 레이아웃을 보존할 수 없습니다",
        "settings_label": "스파스 파일 보존",
        "settings_hint": "스파스 파일(VM 디스크, 데이터베이스 파일)의 할당된 범위만 복사하여 대상이 원본과 동일한 디스크 크기를 유지합니다.",
        "toast_title": "대상이 스파스 파일을 채움",
        "toast_body": "{ $dst_fs }는 스파스 파일을 지원하지 않습니다. 원본의 구멍이 0으로 기록되어 대상이 디스크에서 더 큽니다.",
        "warn_densified": "스파스 레이아웃 보존: 할당된 범위만 복사되었습니다.",
        "warn_mismatch": "스파스 레이아웃 불일치 — 대상이 예상보다 클 수 있습니다.",
    },
    "nl": {
        "err_mismatch": "Sparse-indeling kon niet behouden blijven op bestemming",
        "settings_label": "Sparse-bestanden behouden",
        "settings_hint": "Kopieer alleen de toegewezen gebieden van sparse-bestanden (VM-schijven, databasebestanden) zodat de bestemming dezelfde grootte op schijf behoudt als de bron.",
        "toast_title": "Bestemming vult sparse-bestanden",
        "toast_body": "{ $dst_fs } ondersteunt geen sparse-bestanden. Gaten in de bron zijn als nullen geschreven, dus de bestemming is groter op schijf.",
        "warn_densified": "Sparse-indeling behouden: alleen toegewezen gebieden zijn gekopieerd.",
        "warn_mismatch": "Sparse-indeling komt niet overeen — bestemming kan groter zijn dan verwacht.",
    },
    "pl": {
        "err_mismatch": "Nie udało się zachować układu rozrzedzonego w miejscu docelowym",
        "settings_label": "Zachowaj pliki rozrzedzone",
        "settings_hint": "Kopiuj tylko przydzielone zakresy plików rozrzedzonych (dyski VM, pliki baz danych), aby rozmiar na dysku w miejscu docelowym był taki sam jak w źródle.",
        "toast_title": "Miejsce docelowe wypełnia pliki rozrzedzone",
        "toast_body": "{ $dst_fs } nie obsługuje plików rozrzedzonych. Dziury w źródle zostały zapisane jako zera, więc miejsce docelowe jest większe na dysku.",
        "warn_densified": "Zachowano układ rozrzedzony: skopiowano tylko przydzielone zakresy.",
        "warn_mismatch": "Niezgodność układu rozrzedzonego — miejsce docelowe może być większe niż oczekiwano.",
    },
    "pt-BR": {
        "err_mismatch": "Não foi possível preservar layout esparso no destino",
        "settings_label": "Preservar arquivos esparsos",
        "settings_hint": "Copie apenas as extensões alocadas de arquivos esparsos (discos de VM, arquivos de banco de dados) para que o tamanho em disco do destino permaneça igual ao da origem.",
        "toast_title": "Destino preenche arquivos esparsos",
        "toast_body": "{ $dst_fs } não suporta arquivos esparsos. Buracos na origem foram gravados como zeros, portanto o destino ocupa mais espaço em disco.",
        "warn_densified": "Layout esparso preservado: apenas as extensões alocadas foram copiadas.",
        "warn_mismatch": "Incompatibilidade de layout esparso — destino pode ser maior que o esperado.",
    },
    "ru": {
        "err_mismatch": "Не удалось сохранить разреженную структуру в месте назначения",
        "settings_label": "Сохранять разреженные файлы",
        "settings_hint": "Копировать только выделенные области разреженных файлов (диски виртуальных машин, файлы баз данных), чтобы размер на диске в месте назначения оставался таким же, как у источника.",
        "toast_title": "Место назначения заполняет разреженные файлы",
        "toast_body": "{ $dst_fs } не поддерживает разреженные файлы. Пропуски в источнике были записаны нулями, поэтому место назначения занимает больше места на диске.",
        "warn_densified": "Разреженная структура сохранена: скопированы только выделенные области.",
        "warn_mismatch": "Несоответствие разреженной структуры — место назначения может быть больше ожидаемого.",
    },
    "tr": {
        "err_mismatch": "Hedefte seyrek düzen korunamadı",
        "settings_label": "Seyrek dosyaları koru",
        "settings_hint": "Seyrek dosyaların (VM diskleri, veritabanı dosyaları) yalnızca ayrılmış kapsamlarını kopyalayın; böylece hedefin diskteki boyutu kaynakla aynı kalır.",
        "toast_title": "Hedef seyrek dosyaları dolduruyor",
        "toast_body": "{ $dst_fs } seyrek dosyaları desteklemiyor. Kaynaktaki boşluklar sıfır olarak yazıldı, bu nedenle hedef diskte daha büyük.",
        "warn_densified": "Seyrek düzen korundu: yalnızca ayrılmış kapsamlar kopyalandı.",
        "warn_mismatch": "Seyrek düzen uyuşmazlığı — hedef beklenenden büyük olabilir.",
    },
    "uk": {
        "err_mismatch": "Не вдалося зберегти розріджений макет у цільовому",
        "settings_label": "Зберігати розріджені файли",
        "settings_hint": "Копіювати лише виділені діапазони розріджених файлів (диски віртуальних машин, файли баз даних), щоб розмір на диску в цільовому залишався таким самим, як у джерелі.",
        "toast_title": "Цільове місце заповнює розріджені файли",
        "toast_body": "{ $dst_fs } не підтримує розріджені файли. Отвори в джерелі були записані як нулі, тому цільове місце більше на диску.",
        "warn_densified": "Розріджений макет збережено: скопійовано лише виділені діапазони.",
        "warn_mismatch": "Невідповідність розрідженого макета — цільове місце може бути більшим за очікуване.",
    },
    "vi": {
        "err_mismatch": "Không thể duy trì bố cục thưa tại đích",
        "settings_label": "Bảo toàn tệp thưa",
        "settings_hint": "Chỉ sao chép các phạm vi được cấp phát của tệp thưa (đĩa VM, tệp cơ sở dữ liệu) để kích thước trên đĩa tại đích giữ nguyên bằng với nguồn.",
        "toast_title": "Đích lấp đầy tệp thưa",
        "toast_body": "{ $dst_fs } không hỗ trợ tệp thưa. Các lỗ trong nguồn đã được ghi dưới dạng số 0, do đó đích lớn hơn trên đĩa.",
        "warn_densified": "Bố cục thưa được bảo toàn: chỉ các phạm vi được cấp phát đã được sao chép.",
        "warn_mismatch": "Không khớp bố cục thưa — đích có thể lớn hơn mong đợi.",
    },
    "zh-CN": {
        "err_mismatch": "无法在目标保留稀疏布局",
        "settings_label": "保留稀疏文件",
        "settings_hint": "仅复制稀疏文件(VM 磁盘、数据库文件)的已分配区段,以便目标在磁盘上的大小与源相同。",
        "toast_title": "目标填充稀疏文件",
        "toast_body": "{ $dst_fs } 不支持稀疏文件。源中的空洞被写入为零,因此目标在磁盘上更大。",
        "warn_densified": "已保留稀疏布局:仅复制了已分配的区段。",
        "warn_mismatch": "稀疏布局不匹配——目标可能比预期更大。",
    },
}


def insert_block_after(text: str, anchor_prefix: str, block: str) -> str:
    """Insert `block` (which ends with a newline) immediately after the
    line whose content starts with `anchor_prefix`. Raises ValueError if
    the anchor is not found."""
    lines = text.split("\n")
    for i, line in enumerate(lines):
        if line.startswith(anchor_prefix):
            lines.insert(i + 1, block.rstrip("\n"))
            return "\n".join(lines)
    raise ValueError(f"anchor not found: {anchor_prefix}")


def append_block(text: str, block: str) -> str:
    if not text.endswith("\n"):
        text += "\n"
    if not block.endswith("\n"):
        block += "\n"
    return text + block


def build_err_line(t: dict) -> str:
    return f"err-sparseness-mismatch = {t['err_mismatch']}  # MT"


def build_settings_block(t: dict) -> str:
    return (
        f"settings-preserve-sparseness = {t['settings_label']}  # MT\n"
        f"settings-preserve-sparseness-hint = {t['settings_hint']}  # MT"
    )


def build_tail_block(t: dict) -> str:
    return (
        "\n"
        "# Phase 23 — sparse-file preservation. MT-flagged drafts; the\n"
        "# authoritative English source lives in locales/en/copythat.ftl.\n"
        f"sparse-not-supported-title = {t['toast_title']}  # MT\n"
        f"sparse-not-supported-body = {t['toast_body']}  # MT\n"
        f"sparse-warning-densified = {t['warn_densified']}  # MT\n"
        f"sparse-warning-mismatch = {t['warn_mismatch']}  # MT\n"
    )


def patch_locale(locale: str, t: dict) -> None:
    path = LOCALES_DIR / locale / "copythat.ftl"
    text = path.read_text(encoding="utf-8")
    # Skip if already patched.
    if "sparse-not-supported-title" in text:
        print(f"{locale}: already patched, skipping")
        return
    text = insert_block_after(text, "err-io-other =", build_err_line(t))
    text = insert_block_after(text, "settings-preserve-acls =", build_settings_block(t))
    text = append_block(text, build_tail_block(t))
    path.write_text(text, encoding="utf-8")
    print(f"{locale}: patched")


def main() -> None:
    for locale, t in TRANSLATIONS.items():
        patch_locale(locale, t)


if __name__ == "__main__":
    main()
