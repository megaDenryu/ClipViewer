; 動画のファイルとの関連付けの登録。ClipViewer.iss が #include で取り込む。
; エクスプローラーの「プログラムから開く」に ClipViewer を出し、「既定のアプリ」で動画の既定に選べるようにする。
; 既定のアプリは奪わない。UserChoice(利用者が選んだ既定の記録)には触れず、選べる状態にするだけである。
; ルートの HKA は、利用者ごとのインストールでは HKCU、全利用者向け(管理者)のインストールでは HKLM を指し、インストール先と揃う。
; アンインストールでは、下の uninsdeletekey・uninsdeletevalue の指定どおりに、ここで書いたキーと値だけを消す。
; 注意: ProgID の名(ClipViewer.Video)は利用者の UserChoice から参照されうるため、公開した後に変えてはならない。

#define ProgId "ClipViewer.Video"
#define OpenCommand '""{app}\clip_viewer.exe"" ""%1""'
; 注意: この一覧は crates/clip_viewer/src/persistence/video_extension.rs の一覧(「動画を開く」のファイルダイアログが出す種類)と同じに保つ。試験が食い違いを見つける。
; .ts は MPEG-2 の転送用の形式でもあるが、TypeScript のソースの拡張子と重なるため登録しない。
#dim VideoExtensions[12] {".mp4", ".m4v", ".mkv", ".webm", ".mov", ".avi", ".wmv", ".flv", ".mpg", ".mpeg", ".m2ts", ".mts"}

[Registry]
Root: HKA; Subkey: "Software\Classes\{#ProgId}"; ValueType: string; ValueName: ""; ValueData: "動画ファイル (ClipViewer)"; Flags: uninsdeletekey
; 関連付けた動画のファイルのアイコンは、clip_viewer.exe に埋め込んだ最初のアイコン(番号0。build.rs が埋め込むアプリのアイコン)にする。
Root: HKA; Subkey: "Software\Classes\{#ProgId}\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\clip_viewer.exe,0"
Root: HKA; Subkey: "Software\Classes\{#ProgId}\shell\open\command"; ValueType: string; ValueName: ""; ValueData: "{#OpenCommand}"

Root: HKA; Subkey: "Software\Classes\Applications\clip_viewer.exe"; ValueType: string; ValueName: "FriendlyAppName"; ValueData: "ClipViewer"; Flags: uninsdeletekey
Root: HKA; Subkey: "Software\Classes\Applications\clip_viewer.exe\shell\open\command"; ValueType: string; ValueName: ""; ValueData: "{#OpenCommand}"

Root: HKA; Subkey: "Software\ClipViewer"; Flags: uninsdeletekey
Root: HKA; Subkey: "Software\ClipViewer\Capabilities"; ValueType: string; ValueName: "ApplicationName"; ValueData: "ClipViewer"
Root: HKA; Subkey: "Software\ClipViewer\Capabilities"; ValueType: string; ValueName: "ApplicationDescription"; ValueData: "動画から切り出したクリップを並べて繰り返し再生する"
Root: HKA; Subkey: "Software\RegisteredApplications"; ValueType: string; ValueName: "ClipViewer"; ValueData: "Software\ClipViewer\Capabilities"; Flags: uninsdeletevalue

#sub RegisterVideoExtension
Root: HKA; Subkey: "Software\Classes\{#VideoExtensions[i]}\OpenWithProgids"; ValueType: string; ValueName: "{#ProgId}"; ValueData: ""; Flags: uninsdeletevalue
Root: HKA; Subkey: "Software\Classes\Applications\clip_viewer.exe\SupportedTypes"; ValueType: string; ValueName: "{#VideoExtensions[i]}"; ValueData: ""
Root: HKA; Subkey: "Software\ClipViewer\Capabilities\FileAssociations"; ValueType: string; ValueName: "{#VideoExtensions[i]}"; ValueData: "{#ProgId}"
#endsub
#define i
#for {i = 0; i < DimOf(VideoExtensions); i++} RegisterVideoExtension
