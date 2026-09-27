; ClipViewer の Windows のインストーラーの定義(Inno Setup 6)。
; cargo xtask installer が ISCC.exe へ AppVersion・NumericVersion・SourceExe・LicenseFile・NoticesFile・IconFile・OutputDir の7つを /D で渡して組み立てる。
; AppVersion は版の文字列(将来の 0.2.0-beta.1 のような先行版を含む)であり、NumericVersion はそこから先行版の印を除いた数字だけの版である。
; 実行ファイルの版の情報(VersionInfoVersion)は数字とピリオドだけを受けるため NumericVersion を渡し、利用者に見える版には AppVersion を使う。
; 既定は利用者ごとのインストール(管理者の権限が要らない。%LOCALAPPDATA%\Programs\ClipViewer)であり、
; 開始時の問いで全利用者向け(Program Files)も選べる。FFmpeg は同梱しない。
; 入れるものは clip_viewer.exe・ClipViewer のライセンス(LICENSE.txt)・第三者のライセンス表示(THIRD-PARTY-NOTICES.html)・
; スタートメニューのショートカット・選んだときだけデスクトップのショートカット・
; 動画のファイルとの関連付け(file_association.iss)・アンインストーラーである。
; 注意: AppId はアップグレードとアンインストールで同じアプリだと見分ける鍵であり、公開した後に変えてはならない。

#ifndef AppVersion
  #error AppVersion が渡されていない。cargo xtask installer から組み立てる
#endif
#ifndef NumericVersion
  #error NumericVersion が渡されていない。cargo xtask installer から組み立てる
#endif
#ifndef SourceExe
  #error SourceExe が渡されていない。cargo xtask installer から組み立てる
#endif
#ifndef LicenseFile
  #error LicenseFile が渡されていない。cargo xtask installer から組み立てる
#endif
#ifndef NoticesFile
  #error NoticesFile が渡されていない。cargo xtask installer から組み立てる
#endif
#ifndef IconFile
  #error IconFile が渡されていない。cargo xtask installer から組み立てる
#endif
#ifndef OutputDir
  #error OutputDir が渡されていない。cargo xtask installer から組み立てる
#endif

[Setup]
AppId={{B410CC5F-E6B1-49B9-9CB4-82F32944F6CB}
AppName=ClipViewer
AppVersion={#AppVersion}
AppVerName=ClipViewer {#AppVersion}
AppPublisher=megaDenryu
AppPublisherURL=https://github.com/megaDenryu/ClipViewer
AppSupportURL=https://github.com/megaDenryu/ClipViewer/issues
VersionInfoVersion={#NumericVersion}
VersionInfoTextVersion={#AppVersion}
DefaultDirName={autopf}\ClipViewer
DefaultGroupName=ClipViewer
DisableProgramGroupPage=yes
PrivilegesRequired=lowest
PrivilegesRequiredOverridesAllowed=dialog
ArchitecturesAllowed=x64compatible
ArchitecturesInstallIn64BitMode=x64compatible
; Windows 10 より前の Windows では入れさせない。動作を確かめているのは Windows 10 と 11 だけである。
MinVersion=10.0
OutputDir={#OutputDir}
OutputBaseFilename=ClipViewer-{#AppVersion}-setup
Compression=lzma2/max
SolidCompression=yes
WizardStyle=modern
; setup.exe 自身のアイコン。アプリのアイコン(assets/icon/ClipViewer.ico)と同じにする。
SetupIconFile={#IconFile}
UninstallDisplayName=ClipViewer
; アプリの追加と削除の一覧に出すアイコン。clip_viewer.exe にはビルドのときにアプリのアイコンを埋め込んである(crates/clip_viewer/build.rs)。
UninstallDisplayIcon={app}\clip_viewer.exe
ChangesAssociations=yes

[Languages]
Name: "japanese"; MessagesFile: "compiler:Languages\Japanese.isl"
Name: "english"; MessagesFile: "compiler:Default.isl"

[CustomMessages]
japanese.DeleteUserData=ClipViewer の利用者のデータも削除しますか?%n%n・スタックのライブラリとアプリの設定: %1%n・一覧のサムネイルのキャッシュと、落ちたときの記録と警告の記録: %2%n%n「いいえ」を選ぶと残します。入れ直したときに、そのまま使えます。
english.DeleteUserData=Also delete ClipViewer user data?%n%n- Stack library and app settings: %1%n- Thumbnail cache, crash reports and warning logs: %2%n%nChoose "No" to keep them for a later reinstall.

[Tasks]
Name: "desktopicon"; Description: "{cm:CreateDesktopIcon}"; GroupDescription: "{cm:AdditionalIcons}"; Flags: unchecked

[Files]
Source: "{#SourceExe}"; DestDir: "{app}"; Flags: ignoreversion
Source: "{#LicenseFile}"; DestDir: "{app}"; DestName: "LICENSE.txt"; Flags: ignoreversion
Source: "{#NoticesFile}"; DestDir: "{app}"; DestName: "THIRD-PARTY-NOTICES.html"; Flags: ignoreversion

[Icons]
Name: "{autoprograms}\ClipViewer"; Filename: "{app}\clip_viewer.exe"
Name: "{autodesktop}\ClipViewer"; Filename: "{app}\clip_viewer.exe"; Tasks: desktopicon

#include "file_association.iss"

[Run]
Filename: "{app}\clip_viewer.exe"; Description: "{cm:LaunchProgram,ClipViewer}"; Flags: nowait postinstall skipifsilent

[Code]
{ アンインストールの最後に、利用者のデータを消すかを問う。既定の答えは「いいえ」であり、
  問いを出さない無人のアンインストール(/SILENT 等)でも「いいえ」として残す。
  消す範囲は、アンインストーラーを動かしているアカウントの %APPDATA%\ClipViewer と %LOCALAPPDATA%\ClipViewer だけである。
  全利用者向けのインストールでも、他の利用者のアカウントのデータは消さない。標準の利用者が別の管理者の資格で昇格して
  アンインストールしたときは、その管理者のアカウントのフォルダを指すため、元の利用者のデータは残る。 }
procedure CurUninstallStepChanged(CurUninstallStep: TUninstallStep);
var
  LibraryAndSettings: String;
  CacheAndLogs: String;
begin
  if CurUninstallStep <> usPostUninstall then
    Exit;
  LibraryAndSettings := ExpandConstant('{userappdata}\ClipViewer');
  CacheAndLogs := ExpandConstant('{localappdata}\ClipViewer');
  if SuppressibleMsgBox(FmtMessage(CustomMessage('DeleteUserData'), [LibraryAndSettings, CacheAndLogs]),
       mbConfirmation, MB_YESNO or MB_DEFBUTTON2, IDNO) = IDYES then
  begin
    DelTree(LibraryAndSettings, True, True, True);
    DelTree(CacheAndLogs, True, True, True);
  end;
end;
