//! アプリの設定のファイル(settings.json)の読み書き。書くときは一時ファイルへ書き切ってから置き換え、途中で落ちても元のファイルを壊さない。

use std::io;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use super::error::{アプリの設定のエラー, 設定の本文のエラー};
use super::format::設定のファイルの本文;
use super::rewrite::読み直した設定;
use super::settings::アプリの設定;

/// アプリの設定のファイル名。
pub(super) const 設定のファイル名: &str = "settings.json";

/// 設定のファイルとは、アプリの設定を書く settings.json を置くフォルダを持ち、そのファイルを読み書きするもののことである。
pub(super) struct 設定のファイル {
    pub(super) フォルダ: PathBuf,
}

impl 設定のファイル {
    /// アプリの設定の保管場所のフォルダに置く。
    pub(super) fn フォルダに置く(フォルダ: &Path) -> Self {
        Self {
            フォルダ: フォルダ.to_path_buf(),
        }
    }

    pub(super) fn パス(&self) -> PathBuf {
        self.フォルダ.join(設定のファイル名)
    }

    /// 書きかけのファイルのパス(`settings.json.<プロセスの番号>.tmp`)。settings.json はライブラリの錠の外にあり、
    /// 2つのアプリが同時に保存しうるため、書きかけの名前をプロセスごとに分けて取り合わないようにする。
    fn 書きかけのパス(&self) -> PathBuf {
        self.フォルダ
            .join(format!("{設定のファイル名}.{}.tmp", std::process::id()))
    }

    /// 読む。ファイルが無ければ何も設定していない設定である。
    pub(super) fn 読む(&self) -> Result<アプリの設定, アプリの設定のエラー> {
        let パス = self.パス();
        let 本文 = match std::fs::read_to_string(&パス) {
            Ok(本文) => 設定のファイルの本文::作成する(本文),
            Err(原因) if 原因.kind() == io::ErrorKind::NotFound => {
                return Ok(アプリの設定::何も無い());
            }
            Err(原因) => {
                return Err(アプリの設定のエラー::読み書きできないときのエラーを作る(&パス, 原因));
            }
        };
        let パス = パス.display().to_string();
        本文.最新の設定として読む().map_err(|エラー| match エラー {
            設定の本文のエラー::壊れている(原因) => {
                アプリの設定のエラー::壊れている { パス, 原因 }
            }
            設定の本文のエラー::書き換えない(理由) => {
                アプリの設定のエラー::書き換えない { パス, 理由 }
            }
        })
    }

    /// 書き直す前に読み直す。壊れていれば(JSON として読めないか形が不正なら)別の名前へ移す(`broken_move.rs`)。
    /// 新しすぎる版や別の形式のファイルは移さずに失敗を返す(書き換えてはならないため)。
    pub(super) fn 書き直す前に読み直す(
        &self,
        今: SystemTime,
    ) -> Result<読み直した設定, アプリの設定のエラー> {
        match self.読む() {
            Ok(設定) => Ok(読み直した設定::読めた(設定)),
            Err(アプリの設定のエラー::壊れている { .. }) => {
                self.まだ壊れていれば移す(今)
            }
            Err(その他) => Err(その他),
        }
    }

    /// 最新の版で書く。書きかけのファイルへ書いて記憶装置まで書き切ってから(sync_all)、settings.json を置き換える。
    pub(super) fn 書き切って置き換える(
        &self,
        設定: アプリの設定,
    ) -> Result<(), アプリの設定のエラー> {
        let パス = self.パス();
        let 本文 = 設定のファイルの本文::設定から書き出す(設定).map_err(|原因| {
            アプリの設定のエラー::書き出せない {
                パス: パス.display().to_string(),
                原因,
            }
        })?;
        let 読み書きのエラー =
            アプリの設定のエラー::読み書きできないときのエラーを作る;
        std::fs::create_dir_all(&self.フォルダ)
            .map_err(|原因| 読み書きのエラー(&self.フォルダ, 原因))?;
        let 書きかけ = self.書きかけのパス();
        書きかけのファイルへ書き切る(&書きかけ, 本文.文字列())
            .map_err(|原因| 読み書きのエラー(&書きかけ, 原因))?;
        std::fs::rename(&書きかけ, &パス).map_err(|原因| 読み書きのエラー(&パス, 原因))
    }
}

/// 書きかけのファイルへ本文を書き、OS の書き込みの溜めから記憶装置まで書き切る(sync_all)。
/// 書き切らずに置き換えると、電源が落ちたときに置き換えた後のファイルが空や途中になることがあるためである。
fn 書きかけのファイルへ書き切る(パス: &Path, 本文: &str) -> io::Result<()> {
    let mut ファイル = std::fs::File::create(パス)?;
    io::Write::write_all(&mut ファイル, 本文.as_bytes())?;
    ファイル.sync_all()
}
