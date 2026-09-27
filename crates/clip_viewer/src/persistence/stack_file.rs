//! ファイルの窓口。設定ファイルと動画をファイルダイアログで選ばせ、設定ファイルの本文を読み書きする。
//! ダイアログは利用者が閉じるまで画面のスレッドを止める(rfd の同期の口)。応答を適用する段で呼ぶので描画は止まらない。

use std::io;
use std::path::PathBuf;
use std::time::SystemTime;

use clip_domain::{
    入力された動画パス, 書き出し日時, 書き出し日時エラー, 設定ファイルの本文
};

use super::export_name::設定ファイルの既定の名前;

/// 設定ファイルのパスとは、利用者がダイアログで選んだ設定ファイル(JSON)のパスのことである。
#[derive(Debug, Clone)]
pub(crate) struct 設定ファイルのパス(PathBuf);

impl 設定ファイルのパス {
    /// 窓へ落とされたファイルのパスから作る。拡張子で設定ファイルと見分けるのは呼ぶ側である。
    pub(crate) fn 落とされたファイルから作る(パス: PathBuf) -> Self {
        Self(パス)
    }

    /// ファイル名から拡張子を除いた部分。ライブラリへ取り込むときの名前の元にする。
    pub(crate) fn 拡張子を除いた名前(&self) -> String {
        self.0
            .file_stem()
            .map_or_else(String::new, |幹| 幹.to_string_lossy().into_owned())
    }
}

/// 動画として選べるファイルの拡張子。FFmpeg が読める主な形式である。
const 動画の拡張子: [&str; 8] = ["mp4", "mov", "mkv", "webm", "avi", "m4v", "wmv", "flv"];

/// ファイルの窓口とは、ファイルダイアログとファイルの読み書きの境界であり、最後にダイアログで使ったフォルダを覚える。
#[derive(Debug, Default)]
pub(crate) struct ファイルの窓口 {
    最後に使ったフォルダ: Option<PathBuf>,
}

impl ファイルの窓口 {
    pub(crate) fn 設定ファイルを選ぶ(&mut self) -> Option<設定ファイルのパス> {
        let パス = self
            .ダイアログを作る("設定ファイル(JSON)を開く")
            .add_filter("JSON", &["json"])
            .pick_file()?;
        self.フォルダを覚える(&パス);
        Some(設定ファイルのパス(パス))
    }

    pub(crate) fn 設定ファイルの保存先を選ぶ(
        &mut self,
        既定の名前: &設定ファイルの既定の名前,
    ) -> Option<設定ファイルのパス> {
        let パス = self
            .ダイアログを作る("設定ファイル(JSON)を保存する")
            .add_filter("JSON", &["json"])
            .set_file_name(既定の名前.文字列())
            .save_file()?;
        self.フォルダを覚える(&パス);
        Some(設定ファイルのパス(パス))
    }

    pub(crate) fn 動画を選ぶ(&mut self) -> Option<入力された動画パス> {
        let パス = self
            .ダイアログを作る("動画を開く")
            .add_filter("動画", &動画の拡張子)
            .pick_file()?;
        self.フォルダを覚える(&パス);
        Some(入力された動画パス::作成する(
            パス.to_string_lossy().into_owned(),
        ))
    }

    pub(crate) fn 設定ファイルを読む(
        &self,
        パス: &設定ファイルのパス,
    ) -> io::Result<設定ファイルの本文> {
        std::fs::read_to_string(&パス.0).map(設定ファイルの本文::作成する)
    }

    pub(crate) fn 設定ファイルを書く(
        &self,
        パス: &設定ファイルのパス,
        本文: &設定ファイルの本文,
    ) -> io::Result<()> {
        std::fs::write(&パス.0, 本文.文字列())
    }

    /// 今の時刻の書き出し日時。時計を読む境界はここ1箇所である。
    pub(crate) fn 今の書き出し日時(
        &self,
    ) -> Result<書き出し日時, 書き出し日時エラー> {
        書き出し日時::時刻から作成する(SystemTime::now())
    }

    fn ダイアログを作る(&self, 題: &str) -> rfd::FileDialog {
        let ダイアログ = rfd::FileDialog::new().set_title(題);
        match &self.最後に使ったフォルダ {
            Some(フォルダ) => ダイアログ.set_directory(フォルダ),
            None => ダイアログ,
        }
    }

    fn フォルダを覚える(&mut self, 選んだパス: &std::path::Path) {
        self.最後に使ったフォルダ = 選んだパス.parent().map(std::path::Path::to_path_buf);
    }
}
