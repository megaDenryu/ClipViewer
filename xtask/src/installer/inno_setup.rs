//! Inno Setup のコンパイラ(ISCC.exe)を探し、インストーラーの定義ファイルから setup.exe を組み立てる。

use std::path::{Path, PathBuf};
use std::process::Command;

/// 探すときに試す、インストール先のフォルダの環境変数。Inno Setup 6 を全利用者向けに入れると前の2つ、
/// 利用者ごとに入れる(winget の既定)と最後の1つの下の `Inno Setup 6` に入る。
const 入れた先の環境変数: [(&str, &str); 3] = [
    ("ProgramFiles(x86)", "Inno Setup 6"),
    ("ProgramFiles", "Inno Setup 6"),
    ("LOCALAPPDATA", r"Programs\Inno Setup 6"),
];

/// InnoSetupのコンパイラとは、見つかった ISCC.exe(Inno Setup のコマンド行のコンパイラ)の場所のことである。
pub struct InnoSetupのコンパイラ(PathBuf);

/// インストーラーの材料とは、インストーラーの定義ファイルへ外から渡す値の組のことである。
pub struct インストーラーの材料<'a> {
    pub 定義ファイル: &'a Path,
    pub 版: &'a str,
    pub 数字だけの版: &'a str,
    pub アプリの実行ファイル: &'a Path,
    pub ライセンス: &'a Path,
    pub ライセンス表示: &'a Path,
    pub アイコン: &'a Path,
    pub 置き場所: &'a Path,
}

impl InnoSetupのコンパイラ {
    /// PATH → Inno Setup 6 の既定のインストール先の順に ISCC.exe を探す。見つからなければ入れる手順を返す。
    pub fn 探す() -> Result<Self, String> {
        let 検索パス = std::env::var_os("PATH").unwrap_or_default();
        let 既定の入れた先 = 入れた先の環境変数
            .iter()
            .filter_map(|(変数, 下のフォルダ)| {
                std::env::var_os(変数).map(|上| PathBuf::from(上).join(下のフォルダ))
            });
        std::env::split_paths(&検索パス)
            .chain(既定の入れた先)
            .map(|フォルダ| フォルダ.join("ISCC.exe"))
            .find(|候補| 候補.is_file())
            .map(Self)
            .ok_or_else(|| {
                "Inno Setup 6 のコンパイラ(ISCC.exe)が PATH にも既定のインストール先にも無い。\
                 `winget install JRSoftware.InnoSetup` で入れてから実行し直す"
                    .to_string()
            })
    }

    /// 定義ファイルへ版・実行ファイル・ライセンスの2つのファイル・アイコン・出力先を渡して setup.exe を組み立てる。
    pub fn インストーラーを組み立てる(
        &self,
        材料: &インストーラーの材料,
    ) -> Result<(), String> {
        println!("> {} {}", self.0.display(), 材料.定義ファイル.display());
        let 終了状態 = Command::new(&self.0)
            .arg(format!("/DAppVersion={}", 材料.版))
            .arg(format!("/DNumericVersion={}", 材料.数字だけの版))
            .arg(format!(
                "/DSourceExe={}",
                材料.アプリの実行ファイル.display()
            ))
            .arg(format!("/DLicenseFile={}", 材料.ライセンス.display()))
            .arg(format!("/DNoticesFile={}", 材料.ライセンス表示.display()))
            .arg(format!("/DIconFile={}", 材料.アイコン.display()))
            .arg(format!("/DOutputDir={}", 材料.置き場所.display()))
            .arg(材料.定義ファイル)
            .status()
            .map_err(|原因| format!("ISCC.exe の起動に失敗した: {原因}"))?;
        if 終了状態.success() {
            Ok(())
        } else {
            Err(format!("ISCC.exe が失敗した ({終了状態})"))
        }
    }
}
