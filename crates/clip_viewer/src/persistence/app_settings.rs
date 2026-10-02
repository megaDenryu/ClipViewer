//! アプリの設定の保管場所。FFmpeg を置いたフォルダなどのアプリの設定を `%APPDATA%\ClipViewer\settings.json` に保存し、次に起動したときに読む。
//! settings.json は版を持ち、版ごとの型(`app_settings/v0.rs`・`app_settings/v1.rs`)から最新のアプリの設定へ変換して読む。
//! 保存は、読み直す→変える→一時ファイルへ書き切る→置き換える、の順に行う。参照: _doc/設計/ライブラリ.md「settings.json の形式」

mod broken_move;
mod error;
mod file;
mod format;
mod rewrite;
mod settings;
mod v0;
mod v1;
mod v1_keys;
mod v1_notation;
mod v1_viewer;
mod v1_window;
mod viewer;

#[cfg(test)]
mod broken_move_tests;
#[cfg(test)]
mod broken_tests;
#[cfg(test)]
mod item_name_tests;
#[cfg(test)]
mod keys_tests;
#[cfg(test)]
mod test_support;
#[cfg(test)]
mod tests;
#[cfg(test)]
mod text_tests;
#[cfg(test)]
mod viewer_aspect_tests;
#[cfg(test)]
mod viewer_tests;

use std::path::PathBuf;
use std::time::SystemTime;

use video_source::{FFmpegの置き場所の設定, FFmpegを置いたフォルダ};

pub(crate) use error::アプリの設定のエラー;
use file::設定のファイル;
pub(crate) use rewrite::アプリの設定の保存の結果;

/// アプリの設定の保管場所とは、アプリの設定のファイルを置くフォルダか、それが決まらないことの区別のことである。
/// 決まらないときも起動は止めず、読むと未設定になり、保存すると失敗を返す。
#[derive(Debug, Clone)]
pub(crate) enum アプリの設定の保管場所 {
    フォルダ(PathBuf),
    無い,
}

impl アプリの設定の保管場所 {
    /// アプリのデータを置くフォルダ(Windows では %APPDATA%)の下の ClipViewer フォルダにする。無ければ保管場所は無い。
    pub(crate) fn アプリのデータのフォルダから決める(
        フォルダ: Option<PathBuf>,
    ) -> Self {
        フォルダ.map_or(Self::無い, |フォルダ| {
            Self::フォルダ(フォルダ.join("ClipViewer"))
        })
    }

    /// 保存した FFmpeg の置き場所を読む。保管場所かファイルが無ければ未設定である。
    pub(crate) fn ffmpegの置き場所を読む(
        &self,
    ) -> Result<FFmpegの置き場所の設定, アプリの設定のエラー> {
        match self.設定のファイル() {
            Some(ファイル) => Ok(ファイル.読む()?.ffmpegの置き場所().clone()),
            None => Ok(FFmpegの置き場所の設定::未設定),
        }
    }

    /// FFmpeg を置いたフォルダを保存する。今のファイルを読み直して置き場所だけを変えて書くため、ほかの項目を消さない。
    /// 今のファイルが壊れていれば、`今` の日時とプロセスの番号を名前に入れた別の名前へ移してから書き、移したことを結果で返す。
    /// 移した後で書けなかったときは、移し先のパスを理由の文に入れる。
    /// 新しすぎる版・別の形式のファイルは書き換えずに失敗を返す。
    pub(crate) fn ffmpegの置き場所を保存する(
        &self,
        置き場所: &FFmpegを置いたフォルダ,
        今: SystemTime,
    ) -> Result<アプリの設定の保存の結果, アプリの設定のエラー> {
        let ファイル = self
            .設定のファイル()
            .ok_or(アプリの設定のエラー::保管場所が無い)?;
        let (変える前, 結果) = ファイル
            .書き直す前に読み直す(今)?
            .変える前の設定と保存の結果に分ける();
        ファイル
            .書き切って置き換える(変える前.ffmpegの置き場所を変えた(
                FFmpegの置き場所の設定::設定済み(置き場所.clone()),
            ))
            .map_err(|エラー| 結果.書けなかったときのエラーにする(エラー))?;
        Ok(結果)
    }

    /// アプリの設定のファイル(`%APPDATA%\ClipViewer\settings.json`)。保管場所が無ければ無い。
    fn 設定のファイル(&self) -> Option<設定のファイル> {
        match self {
            Self::フォルダ(フォルダ) => {
                Some(設定のファイル::フォルダに置く(フォルダ))
            }
            Self::無い => None,
        }
    }
}
