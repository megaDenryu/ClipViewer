//! 測る動画の出どころ。引数から決まり、測る前に実在するファイルへ変える。

use std::path::PathBuf;

use super::read_length::読む秒数;
use super::synthetic::合成画像の動画;
use super::video_file::測る動画のファイル;
use crate::ffmpeg_location::FFmpegの置き場所;

/// 測る動画の出どころとは、測る対象の動画を、合成画像から作るか、引数で渡されたファイルを使うかの区別のことである。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum 測る動画の出どころ {
    /// ffmpeg の合成画像(testsrc2)から一時フォルダに作る。
    合成画像で作る,
    /// 利用者が引数で渡したファイルを使う。
    与えられたファイル(PathBuf),
}

impl 測る動画の出どころ {
    /// 測る動画のファイルを用意する。合成画像なら読む長さの分だけ一時フォルダに作る。
    pub fn ファイルを用意する(
        &self,
        ffmpegの置き場所: &FFmpegの置き場所,
        読む長さ: 読む秒数,
    ) -> Result<測る動画のファイル, String> {
        match self {
            Self::合成画像で作る => Ok(測る動画のファイル::合成画像(
                合成画像の動画::作る(ffmpegの置き場所, 読む長さ)?,
            )),
            Self::与えられたファイル(パス) => Ok(
                測る動画のファイル::与えられたファイル(パス.clone()),
            ),
        }
    }
}
