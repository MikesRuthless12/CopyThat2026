#!/usr/bin/env python3
"""Phase 24 — insert security-metadata Fluent keys into the 17
non-English locales. MT-flagged drafts matching the Standing
Per-Phase Rules. The 12 keys live as a single block at the end of
each locale; the English source is authoritative."""

from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
LOCALES_DIR = ROOT / "locales"

# 12 keys × 17 locales. MT-flagged; human review tracked in
# docs/I18N_TODO.md.
TRANSLATIONS = {
    "ar": {
        "section": "الحفاظ على بيانات الأمان الوصفية",
        "section_hint": "التقاط وإعادة تطبيق تدفقات البيانات الوصفية خارج النطاق (NTFS ADS / xattrs / قوائم ACL / سياقات SELinux / إمكانيات ملف Linux / تفرعات موارد macOS) في كل عملية نسخ.",
        "motw": "الحفاظ على علامة الويب (تنزيل من الإنترنت)",
        "motw_hint": "حرج للأمان. يستخدم SmartScreen وOffice Protected View هذا التدفق للتحذير من الملفات التي تم تنزيلها من الإنترنت. تعطيله يسمح للملف القابل للتنفيذ بالتخلص من علامته على نسخة وتجاوز ضمانات نظام التشغيل.",
        "posix": "الحفاظ على قوائم POSIX ACL والسمات الموسعة",
        "posix_hint": "نقل سمات user.* / system.* / trusted.* وقوائم التحكم في وصول POSIX عبر النسخ.",
        "selinux": "الحفاظ على سياقات SELinux",
        "selinux_hint": "نقل تسمية security.selinux عبر النسخ بحيث يمكن للبرامج التي تعمل ضمن سياسات MAC الوصول إلى الملف.",
        "rfork": "الحفاظ على تفرعات موارد macOS ومعلومات Finder",
        "rfork_hint": "نقل تفرع الموارد القديم وFinderInfo (علامات الألوان، بيانات Carbon الوصفية) عبر النسخ.",
        "appledouble": "استخدام ملف AppleDouble الجانبي على أنظمة الملفات غير المتوافقة",
        "translated": "تم تخزين البيانات الوصفية الأجنبية في ملف AppleDouble الجانبي (._{ $ext })",
    },
    "de": {
        "section": "Sicherheitsmetadaten bewahren",
        "section_hint": "Out-of-Band-Metadatenströme (NTFS-ADS / xattrs / POSIX-ACLs / SELinux-Kontexte / Linux-Datei-Capabilities / macOS-Ressourcen-Forks) bei jeder Kopie erfassen und erneut anwenden.",
        "motw": "Mark-of-the-Web (Aus-dem-Internet-heruntergeladen-Flag) bewahren",
        "motw_hint": "Sicherheitskritisch. SmartScreen und Office Protected View nutzen diesen Stream, um vor aus dem Internet heruntergeladenen Dateien zu warnen. Deaktivieren erlaubt einer heruntergeladenen Anwendung, ihre Herkunftsmarkierung beim Kopieren abzuwerfen und Betriebssystem-Schutzmaßnahmen zu umgehen.",
        "posix": "POSIX-ACLs und erweiterte Attribute bewahren",
        "posix_hint": "user.* / system.* / trusted.* xattrs und POSIX-Zugriffssteuerungslisten beim Kopieren übernehmen.",
        "selinux": "SELinux-Kontexte bewahren",
        "selinux_hint": "Das security.selinux-Label beim Kopieren übernehmen, damit unter MAC-Richtlinien laufende Daemons weiterhin auf die Datei zugreifen können.",
        "rfork": "macOS-Ressourcen-Forks und Finder-Info bewahren",
        "rfork_hint": "Den Legacy-Ressourcen-Fork und FinderInfo (Farbtags, Carbon-Metadaten) beim Kopieren übernehmen.",
        "appledouble": "AppleDouble-Sidecar auf inkompatiblen Dateisystemen verwenden",
        "translated": "Fremde Metadaten im AppleDouble-Sidecar gespeichert (._{ $ext })",
    },
    "es": {
        "section": "Preservar metadatos de seguridad",
        "section_hint": "Capture y vuelva a aplicar flujos de metadatos fuera de banda (NTFS ADS / xattrs / ACL POSIX / contextos SELinux / capacidades de archivo Linux / bifurcaciones de recursos macOS) en cada copia.",
        "motw": "Preservar Marca de la Web (indicador de descarga de internet)",
        "motw_hint": "Crítico para la seguridad. SmartScreen y Office Protected View usan este flujo para advertir sobre archivos descargados de internet. Desactivarlo permite que un ejecutable descargado pierda su marca de origen al copiar y omita las protecciones del sistema operativo.",
        "posix": "Preservar ACL POSIX y atributos extendidos",
        "posix_hint": "Traslade los xattrs user.* / system.* / trusted.* y las listas de control de acceso POSIX en la copia.",
        "selinux": "Preservar contextos SELinux",
        "selinux_hint": "Traslade la etiqueta security.selinux en la copia para que los demonios bajo políticas MAC puedan seguir accediendo al archivo.",
        "rfork": "Preservar bifurcaciones de recursos y Finder info de macOS",
        "rfork_hint": "Traslade la bifurcación de recursos heredada y FinderInfo (etiquetas de color, metadatos Carbon) en la copia.",
        "appledouble": "Usar archivo lateral AppleDouble en sistemas de archivos incompatibles",
        "translated": "Metadatos foráneos guardados en archivo lateral AppleDouble (._{ $ext })",
    },
    "fr": {
        "section": "Préserver les métadonnées de sécurité",
        "section_hint": "Capturer et réappliquer les flux de métadonnées hors-bande (NTFS ADS / xattrs / ACL POSIX / contextes SELinux / capacités de fichier Linux / forks de ressources macOS) à chaque copie.",
        "motw": "Préserver la Marque du Web (indicateur de téléchargement Internet)",
        "motw_hint": "Critique pour la sécurité. SmartScreen et Office Protected View utilisent ce flux pour avertir des fichiers téléchargés depuis Internet. Désactiver permet à un exécutable téléchargé de perdre son marqueur d'origine lors de la copie et de contourner les protections du système d'exploitation.",
        "posix": "Préserver les ACL POSIX et attributs étendus",
        "posix_hint": "Transporter les xattrs user.* / system.* / trusted.* et les listes de contrôle d'accès POSIX lors de la copie.",
        "selinux": "Préserver les contextes SELinux",
        "selinux_hint": "Transporter l'étiquette security.selinux lors de la copie pour que les démons sous politiques MAC puissent accéder au fichier.",
        "rfork": "Préserver les forks de ressources macOS et Finder info",
        "rfork_hint": "Transporter le fork de ressources hérité et FinderInfo (étiquettes de couleur, métadonnées Carbon) lors de la copie.",
        "appledouble": "Utiliser un fichier annexe AppleDouble sur les systèmes de fichiers incompatibles",
        "translated": "Métadonnées étrangères stockées dans le fichier annexe AppleDouble (._{ $ext })",
    },
    "hi": {
        "section": "सुरक्षा मेटाडेटा संरक्षित करें",
        "section_hint": "हर प्रतिलिपि पर आउट-ऑफ-बैंड मेटाडेटा स्ट्रीम (NTFS ADS / xattrs / POSIX ACLs / SELinux संदर्भ / Linux फ़ाइल क्षमताएँ / macOS संसाधन फोर्क्स) को कैप्चर और पुनः लागू करें।",
        "motw": "मार्क-ऑफ-द-वेब (इंटरनेट-से-डाउनलोड किया गया फ़्लैग) संरक्षित करें",
        "motw_hint": "सुरक्षा के लिए महत्वपूर्ण। SmartScreen और Office Protected View इस स्ट्रीम का उपयोग इंटरनेट से डाउनलोड की गई फ़ाइलों के बारे में चेतावनी देने के लिए करते हैं। इसे अक्षम करने से एक डाउनलोड किया गया निष्पादन योग्य फ़ाइल कॉपी पर अपना मूल चिह्न खो सकता है और ऑपरेटिंग सिस्टम सुरक्षा को बायपास कर सकता है।",
        "posix": "POSIX ACLs और विस्तारित विशेषताएँ संरक्षित करें",
        "posix_hint": "प्रतिलिपि के दौरान user.* / system.* / trusted.* xattrs और POSIX एक्सेस-नियंत्रण सूचियाँ ले जाएँ।",
        "selinux": "SELinux संदर्भ संरक्षित करें",
        "selinux_hint": "प्रतिलिपि के दौरान security.selinux लेबल ले जाएँ ताकि MAC नीतियों के तहत चल रहे डेमॉन फ़ाइल तक पहुँच सकें।",
        "rfork": "macOS संसाधन फोर्क्स और Finder जानकारी संरक्षित करें",
        "rfork_hint": "प्रतिलिपि के दौरान विरासत संसाधन फ़ोर्क और FinderInfo (रंग टैग, Carbon मेटाडेटा) ले जाएँ।",
        "appledouble": "असंगत फ़ाइल सिस्टम पर AppleDouble साइडकार का उपयोग करें",
        "translated": "विदेशी मेटाडेटा AppleDouble साइडकार में संग्रहीत (._{ $ext })",
    },
    "id": {
        "section": "Pertahankan metadata keamanan",
        "section_hint": "Tangkap dan terapkan ulang aliran metadata di luar pita (NTFS ADS / xattrs / ACL POSIX / konteks SELinux / kapabilitas file Linux / fork sumber daya macOS) pada setiap salinan.",
        "motw": "Pertahankan Mark-of-the-Web (penanda diunduh-dari-internet)",
        "motw_hint": "Penting untuk keamanan. SmartScreen dan Office Protected View menggunakan aliran ini untuk memperingatkan tentang file yang diunduh dari internet. Menonaktifkannya memungkinkan executable yang diunduh kehilangan penanda asalnya saat disalin dan melewati pengamanan sistem operasi.",
        "posix": "Pertahankan ACL POSIX dan atribut yang diperluas",
        "posix_hint": "Bawa xattrs user.* / system.* / trusted.* dan daftar kontrol akses POSIX selama penyalinan.",
        "selinux": "Pertahankan konteks SELinux",
        "selinux_hint": "Bawa label security.selinux selama penyalinan agar daemon yang berjalan di bawah kebijakan MAC tetap dapat mengakses file.",
        "rfork": "Pertahankan fork sumber daya macOS dan info Finder",
        "rfork_hint": "Bawa fork sumber daya warisan dan FinderInfo (tag warna, metadata Carbon) selama penyalinan.",
        "appledouble": "Gunakan sidecar AppleDouble pada sistem file yang tidak kompatibel",
        "translated": "Metadata asing disimpan di sidecar AppleDouble (._{ $ext })",
    },
    "it": {
        "section": "Conserva metadati di sicurezza",
        "section_hint": "Acquisisci e riapplica flussi di metadati fuori banda (NTFS ADS / xattrs / ACL POSIX / contesti SELinux / capacità file Linux / fork di risorse macOS) ad ogni copia.",
        "motw": "Conserva Mark-of-the-Web (flag scaricato-da-internet)",
        "motw_hint": "Critico per la sicurezza. SmartScreen e Office Protected View usano questo flusso per avvisare sui file scaricati da internet. Disabilitare consente a un eseguibile scaricato di perdere il proprio marcatore di origine durante la copia e bypassare le protezioni del sistema operativo.",
        "posix": "Conserva ACL POSIX e attributi estesi",
        "posix_hint": "Trasporta xattrs user.* / system.* / trusted.* ed elenchi di controllo accesso POSIX durante la copia.",
        "selinux": "Conserva contesti SELinux",
        "selinux_hint": "Trasporta l'etichetta security.selinux durante la copia in modo che i demoni sotto policy MAC possano ancora accedere al file.",
        "rfork": "Conserva fork di risorse macOS e Finder info",
        "rfork_hint": "Trasporta il fork di risorse legacy e FinderInfo (tag colore, metadati Carbon) durante la copia.",
        "appledouble": "Usa sidecar AppleDouble su filesystem incompatibili",
        "translated": "Metadati esterni archiviati nel sidecar AppleDouble (._{ $ext })",
    },
    "ja": {
        "section": "セキュリティメタデータを保持",
        "section_hint": "コピーごとに帯域外メタデータストリーム(NTFS ADS / xattrs / POSIX ACL / SELinux コンテキスト / Linux ファイル機能 / macOS リソースフォーク)をキャプチャして再適用します。",
        "motw": "Mark-of-the-Web(インターネットからダウンロードフラグ)を保持",
        "motw_hint": "セキュリティ上重要。SmartScreen と Office Protected View はこのストリームを使用してインターネットからダウンロードしたファイルに関する警告を表示します。無効にすると、ダウンロードした実行可能ファイルがコピー時に起源マーカーを失い、オペレーティングシステムの保護を回避できるようになります。",
        "posix": "POSIX ACL と拡張属性を保持",
        "posix_hint": "コピー時に user.* / system.* / trusted.* xattrs と POSIX アクセス制御リストを引き継ぎます。",
        "selinux": "SELinux コンテキストを保持",
        "selinux_hint": "MAC ポリシー下で実行されているデーモンが引き続きファイルにアクセスできるよう、コピー時に security.selinux ラベルを引き継ぎます。",
        "rfork": "macOS リソースフォークと Finder 情報を保持",
        "rfork_hint": "コピー時にレガシーリソースフォークと FinderInfo(カラータグ、Carbon メタデータ)を引き継ぎます。",
        "appledouble": "互換性のないファイルシステムでは AppleDouble サイドカーを使用",
        "translated": "外部メタデータを AppleDouble サイドカーに保存しました (._{ $ext })",
    },
    "ko": {
        "section": "보안 메타데이터 보존",
        "section_hint": "모든 복사에서 대역 외 메타데이터 스트림(NTFS ADS / xattrs / POSIX ACL / SELinux 컨텍스트 / Linux 파일 기능 / macOS 리소스 포크)을 캡처하고 다시 적용합니다.",
        "motw": "Mark-of-the-Web(인터넷에서 다운로드 플래그) 보존",
        "motw_hint": "보안에 매우 중요. SmartScreen 및 Office Protected View는 이 스트림을 사용하여 인터넷에서 다운로드한 파일에 대해 경고합니다. 비활성화하면 다운로드한 실행 파일이 복사 시 출처 표식을 잃고 운영 체제 보호를 우회할 수 있습니다.",
        "posix": "POSIX ACL 및 확장 속성 보존",
        "posix_hint": "복사 시 user.* / system.* / trusted.* xattrs 및 POSIX 액세스 제어 목록을 전달합니다.",
        "selinux": "SELinux 컨텍스트 보존",
        "selinux_hint": "MAC 정책하에서 실행되는 데몬이 파일에 계속 액세스할 수 있도록 복사 시 security.selinux 레이블을 전달합니다.",
        "rfork": "macOS 리소스 포크 및 Finder 정보 보존",
        "rfork_hint": "복사 시 레거시 리소스 포크 및 FinderInfo(색상 태그, Carbon 메타데이터)를 전달합니다.",
        "appledouble": "호환되지 않는 파일 시스템에서 AppleDouble 사이드카 사용",
        "translated": "외래 메타데이터가 AppleDouble 사이드카에 저장됨 (._{ $ext })",
    },
    "nl": {
        "section": "Beveiligingsmetadata behouden",
        "section_hint": "Leg buiten-band-metadatastromen (NTFS ADS / xattrs / POSIX ACL's / SELinux-contexten / Linux-bestandscapabilities / macOS-resourceforks) vast en pas ze opnieuw toe bij elke kopie.",
        "motw": "Mark-of-the-Web (van-internet-gedownload-vlag) behouden",
        "motw_hint": "Kritiek voor de beveiliging. SmartScreen en Office Protected View gebruiken deze stroom om te waarschuwen voor van internet gedownloade bestanden. Uitschakelen laat een gedownload uitvoerbaar bestand zijn oorsprongmarker verliezen bij het kopiëren en de beschermingen van het besturingssysteem omzeilen.",
        "posix": "POSIX ACL's en uitgebreide attributen behouden",
        "posix_hint": "Draag user.* / system.* / trusted.* xattrs en POSIX-toegangscontrolelijsten over tijdens het kopiëren.",
        "selinux": "SELinux-contexten behouden",
        "selinux_hint": "Draag het security.selinux-label over tijdens het kopiëren zodat daemons onder MAC-beleid het bestand kunnen blijven openen.",
        "rfork": "macOS-resourceforks en Finder-info behouden",
        "rfork_hint": "Draag de legacy resource-fork en FinderInfo (kleurtags, Carbon-metadata) over tijdens het kopiëren.",
        "appledouble": "AppleDouble-sidecar gebruiken op incompatibele bestandssystemen",
        "translated": "Buitenlandse metadata opgeslagen in AppleDouble-sidecar (._{ $ext })",
    },
    "pl": {
        "section": "Zachowaj metadane bezpieczeństwa",
        "section_hint": "Przechwyć i ponownie zastosuj strumienie metadanych poza pasmem (NTFS ADS / xattrs / listy ACL POSIX / konteksty SELinux / uprawnienia plików Linux / forki zasobów macOS) przy każdej kopii.",
        "motw": "Zachowaj Mark-of-the-Web (flagę pobrania z internetu)",
        "motw_hint": "Krytyczne dla bezpieczeństwa. SmartScreen i Office Protected View używają tego strumienia do ostrzegania o plikach pobranych z internetu. Wyłączenie pozwala pobranemu programowi pozbyć się znacznika pochodzenia podczas kopiowania i obejść zabezpieczenia systemu operacyjnego.",
        "posix": "Zachowaj listy ACL POSIX i atrybuty rozszerzone",
        "posix_hint": "Przenieś atrybuty user.* / system.* / trusted.* xattrs i listy kontroli dostępu POSIX podczas kopiowania.",
        "selinux": "Zachowaj konteksty SELinux",
        "selinux_hint": "Przenieś etykietę security.selinux podczas kopiowania, aby demony działające pod politykami MAC nadal miały dostęp do pliku.",
        "rfork": "Zachowaj forki zasobów macOS i informacje Findera",
        "rfork_hint": "Przenieś starszy fork zasobów i FinderInfo (tagi kolorów, metadane Carbon) podczas kopiowania.",
        "appledouble": "Użyj towarzyszącego pliku AppleDouble na niezgodnych systemach plików",
        "translated": "Obce metadane zapisane w towarzyszącym pliku AppleDouble (._{ $ext })",
    },
    "pt-BR": {
        "section": "Preservar metadados de segurança",
        "section_hint": "Capture e reaplique fluxos de metadados fora de banda (NTFS ADS / xattrs / ACLs POSIX / contextos SELinux / capacidades de arquivo Linux / forks de recursos macOS) em cada cópia.",
        "motw": "Preservar Marca da Web (flag de download da internet)",
        "motw_hint": "Crítico para a segurança. SmartScreen e Office Protected View usam este fluxo para alertar sobre arquivos baixados da internet. Desativar permite que um executável baixado perca seu marcador de origem ao copiar e contorne as proteções do sistema operacional.",
        "posix": "Preservar ACLs POSIX e atributos estendidos",
        "posix_hint": "Transporte os xattrs user.* / system.* / trusted.* e listas de controle de acesso POSIX durante a cópia.",
        "selinux": "Preservar contextos SELinux",
        "selinux_hint": "Transporte o rótulo security.selinux durante a cópia para que daemons sob políticas MAC possam continuar acessando o arquivo.",
        "rfork": "Preservar forks de recursos macOS e Finder info",
        "rfork_hint": "Transporte o fork de recursos legado e FinderInfo (etiquetas de cor, metadados Carbon) durante a cópia.",
        "appledouble": "Usar arquivo lateral AppleDouble em sistemas de arquivos incompatíveis",
        "translated": "Metadados estrangeiros armazenados em arquivo lateral AppleDouble (._{ $ext })",
    },
    "ru": {
        "section": "Сохранять метаданные безопасности",
        "section_hint": "Захватывайте и применяйте повторно внешние потоки метаданных (NTFS ADS / xattrs / POSIX ACL / контексты SELinux / возможности файлов Linux / форки ресурсов macOS) при каждой копии.",
        "motw": "Сохранять Mark-of-the-Web (флаг загрузки из интернета)",
        "motw_hint": "Критично для безопасности. SmartScreen и Office Protected View используют этот поток для предупреждения о файлах, загруженных из интернета. Отключение позволяет загруженному исполняемому файлу потерять метку происхождения при копировании и обойти защиту операционной системы.",
        "posix": "Сохранять POSIX ACL и расширенные атрибуты",
        "posix_hint": "Переносите xattrs user.* / system.* / trusted.* и списки контроля доступа POSIX при копировании.",
        "selinux": "Сохранять контексты SELinux",
        "selinux_hint": "Переносите метку security.selinux при копировании, чтобы демоны под политиками MAC могли по-прежнему получать доступ к файлу.",
        "rfork": "Сохранять форки ресурсов macOS и информацию Finder",
        "rfork_hint": "Переносите устаревший форк ресурсов и FinderInfo (цветовые теги, метаданные Carbon) при копировании.",
        "appledouble": "Использовать дополнительный файл AppleDouble в несовместимых файловых системах",
        "translated": "Иностранные метаданные сохранены в дополнительном файле AppleDouble (._{ $ext })",
    },
    "tr": {
        "section": "Güvenlik meta verilerini koru",
        "section_hint": "Her kopyalamada bant dışı meta veri akışlarını (NTFS ADS / xattrs / POSIX ACL'leri / SELinux bağlamları / Linux dosya yetenekleri / macOS kaynak çatalları) yakalayın ve yeniden uygulayın.",
        "motw": "Mark-of-the-Web'i (internetten-indirildi bayrağını) koru",
        "motw_hint": "Güvenlik için kritik. SmartScreen ve Office Protected View, internetten indirilen dosyalar hakkında uyarmak için bu akışı kullanır. Devre dışı bırakmak, indirilen bir çalıştırılabilir dosyanın kopyalama sırasında kaynak işaretini düşürmesine ve işletim sistemi güvenlik önlemlerini atlamasına olanak tanır.",
        "posix": "POSIX ACL'leri ve genişletilmiş öznitelikleri koru",
        "posix_hint": "Kopyalama sırasında user.* / system.* / trusted.* xattrs ve POSIX erişim kontrol listelerini taşı.",
        "selinux": "SELinux bağlamlarını koru",
        "selinux_hint": "MAC politikaları altında çalışan arka plan programlarının dosyaya erişmeye devam edebilmesi için kopyalama sırasında security.selinux etiketini taşı.",
        "rfork": "macOS kaynak çatallarını ve Finder bilgilerini koru",
        "rfork_hint": "Kopyalama sırasında eski kaynak çatalını ve FinderInfo'yu (renk etiketleri, Carbon meta verileri) taşı.",
        "appledouble": "Uyumsuz dosya sistemlerinde AppleDouble yan dosyası kullan",
        "translated": "Yabancı meta veriler AppleDouble yan dosyasında saklandı (._{ $ext })",
    },
    "uk": {
        "section": "Зберігати метадані безпеки",
        "section_hint": "Захоплюйте та повторно застосовуйте позасмужні потоки метаданих (NTFS ADS / xattrs / POSIX ACL / контексти SELinux / можливості файлів Linux / форки ресурсів macOS) під час кожної копії.",
        "motw": "Зберігати Mark-of-the-Web (прапорець завантаження з інтернету)",
        "motw_hint": "Критично для безпеки. SmartScreen і Office Protected View використовують цей потік для попередження про файли, завантажені з інтернету. Вимкнення дозволяє завантаженому виконуваному файлу втратити маркер походження під час копіювання та обійти захист операційної системи.",
        "posix": "Зберігати POSIX ACL та розширені атрибути",
        "posix_hint": "Переносити xattrs user.* / system.* / trusted.* і списки керування доступом POSIX під час копіювання.",
        "selinux": "Зберігати контексти SELinux",
        "selinux_hint": "Переносити мітку security.selinux під час копіювання, щоб демони під політиками MAC могли продовжувати доступ до файлу.",
        "rfork": "Зберігати форки ресурсів macOS та інформацію Finder",
        "rfork_hint": "Переносити застарілий форк ресурсів і FinderInfo (кольорові теги, метадані Carbon) під час копіювання.",
        "appledouble": "Використовувати додатковий файл AppleDouble у несумісних файлових системах",
        "translated": "Чужорідні метадані збережено в додатковому файлі AppleDouble (._{ $ext })",
    },
    "vi": {
        "section": "Bảo toàn siêu dữ liệu bảo mật",
        "section_hint": "Bắt và áp dụng lại các luồng siêu dữ liệu ngoài băng (NTFS ADS / xattrs / ACL POSIX / ngữ cảnh SELinux / khả năng tệp Linux / nhánh tài nguyên macOS) trên mỗi bản sao.",
        "motw": "Bảo toàn Mark-of-the-Web (cờ tải xuống từ internet)",
        "motw_hint": "Quan trọng cho bảo mật. SmartScreen và Office Protected View sử dụng luồng này để cảnh báo về các tệp đã tải xuống từ internet. Vô hiệu hóa cho phép một tệp thực thi đã tải xuống mất dấu nguồn gốc khi sao chép và bỏ qua các biện pháp bảo vệ của hệ điều hành.",
        "posix": "Bảo toàn ACL POSIX và thuộc tính mở rộng",
        "posix_hint": "Mang theo các xattrs user.* / system.* / trusted.* và danh sách kiểm soát truy cập POSIX trong quá trình sao chép.",
        "selinux": "Bảo toàn ngữ cảnh SELinux",
        "selinux_hint": "Mang theo nhãn security.selinux trong quá trình sao chép để các tiến trình nền chạy dưới chính sách MAC vẫn có thể truy cập tệp.",
        "rfork": "Bảo toàn nhánh tài nguyên macOS và thông tin Finder",
        "rfork_hint": "Mang theo nhánh tài nguyên kế thừa và FinderInfo (thẻ màu, siêu dữ liệu Carbon) trong quá trình sao chép.",
        "appledouble": "Sử dụng tệp đi kèm AppleDouble trên hệ thống tệp không tương thích",
        "translated": "Siêu dữ liệu nước ngoài được lưu trữ trong tệp đi kèm AppleDouble (._{ $ext })",
    },
    "zh-CN": {
        "section": "保留安全元数据",
        "section_hint": "在每次复制时捕获并重新应用带外元数据流(NTFS ADS / xattrs / POSIX ACL / SELinux 上下文 / Linux 文件能力 / macOS 资源分支)。",
        "motw": "保留网络标记(从互联网下载标志)",
        "motw_hint": "对安全至关重要。SmartScreen 和 Office Protected View 使用此流来警告从互联网下载的文件。禁用会让下载的可执行文件在复制时失去其来源标记,并绕过操作系统的安全防护。",
        "posix": "保留 POSIX ACL 和扩展属性",
        "posix_hint": "在复制过程中携带 user.* / system.* / trusted.* xattrs 和 POSIX 访问控制列表。",
        "selinux": "保留 SELinux 上下文",
        "selinux_hint": "在复制过程中携带 security.selinux 标签,以便在 MAC 策略下运行的守护进程仍能访问该文件。",
        "rfork": "保留 macOS 资源分支和 Finder 信息",
        "rfork_hint": "在复制过程中携带遗留的资源分支和 FinderInfo(颜色标签、Carbon 元数据)。",
        "appledouble": "在不兼容的文件系统上使用 AppleDouble 附属文件",
        "translated": "外部元数据已存储在 AppleDouble 附属文件中 (._{ $ext })",
    },
}


def build_block(t: dict) -> str:
    return (
        "\n"
        "# Phase 24 — security-metadata preservation. MT-flagged drafts;\n"
        "# the authoritative English source lives in locales/en/copythat.ftl.\n"
        f"settings-preserve-security-metadata = {t['section']}  # MT\n"
        f"settings-preserve-security-metadata-hint = {t['section_hint']}  # MT\n"
        f"settings-preserve-motw = {t['motw']}  # MT\n"
        f"settings-preserve-motw-hint = {t['motw_hint']}  # MT\n"
        f"settings-preserve-posix-acls = {t['posix']}  # MT\n"
        f"settings-preserve-posix-acls-hint = {t['posix_hint']}  # MT\n"
        f"settings-preserve-selinux = {t['selinux']}  # MT\n"
        f"settings-preserve-selinux-hint = {t['selinux_hint']}  # MT\n"
        f"settings-preserve-resource-forks = {t['rfork']}  # MT\n"
        f"settings-preserve-resource-forks-hint = {t['rfork_hint']}  # MT\n"
        f"settings-appledouble-fallback = {t['appledouble']}  # MT\n"
        f"meta-translated-to-appledouble = {t['translated']}  # MT\n"
    )


def patch_locale(locale: str, t: dict) -> None:
    path = LOCALES_DIR / locale / "copythat.ftl"
    text = path.read_text(encoding="utf-8")
    if "settings-preserve-security-metadata" in text:
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
