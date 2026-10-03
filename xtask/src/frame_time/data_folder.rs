//! 測るアプリに渡す一時のデータのフォルダ。アプリのデータ(%APPDATA%)とローカルのアプリのデータ(%LOCALAPPDATA%)をここへ向け、
//! 測る起動が利用者の本物のライブラリ・設定・キャッシュを読み書きしないようにする。値を捨てるとフォルダごと消す。

use std::path::PathBuf;

/// 測るための一時のデータのフォルダとは、OS の一時フォルダの下の、このプロセスだけが使うフォルダのことである。
/// 下に、アプリのデータのフォルダと、ローカルのアプリのデータのフォルダと、結果のファイルを置く。
pub struct 測るための一時のデータのフォルダ(PathBuf);

impl 測るための一時のデータのフォルダ {
    pub fn 作る() -> Result<Self, String> {
        let フォルダ =
            std::env::temp_dir().join(format!("clipviewer-frame-time-{}", std::process::id()));
        for 下 in ["appdata", "localappdata"] {
            std::fs::create_dir_all(フォルダ.join(下)).map_err(|原因| {
                format!("一時フォルダ {} を作れない: {原因}", フォルダ.display())
            })?;
        }
        Ok(Self(フォルダ))
    }

    /// アプリに %APPDATA% として渡すフォルダ。
    pub fn アプリのデータのフォルダ(&self) -> PathBuf {
        self.0.join("appdata")
    }

    /// アプリに %LOCALAPPDATA% として渡すフォルダ。
    pub fn ローカルのアプリのデータのフォルダ(&self) -> PathBuf {
        self.0.join("localappdata")
    }

    /// アプリがまとめの文を書く結果のファイル。
    pub fn 結果のファイル(&self) -> PathBuf {
        self.0.join("frame-time.txt")
    }
}

impl Drop for 測るための一時のデータのフォルダ {
    fn drop(&mut self) {
        if let Err(原因) = std::fs::remove_dir_all(&self.0) {
            eprintln!("一時フォルダ {} を消せなかった: {原因}", self.0.display());
        }
    }
}
