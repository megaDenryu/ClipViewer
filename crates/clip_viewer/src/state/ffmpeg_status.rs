//! FFmpeg の状況。画面が FFmpeg の見つかった場所か、見つからない理由と入力中のフォルダを出すために持つ。

use std::path::PathBuf;

use video_source::{
    FFmpegが見つからないエラー, FFmpegの実行ファイル, FFmpegを置いたフォルダ
};

/// 入力中のフォルダとは、FFmpeg の置き場所の入力欄に利用者が書いている途中の文字列のことである。
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct 入力中のフォルダ(String);

impl 入力中のフォルダ {
    pub(crate) fn 作成する(文字列: String) -> Self {
        Self(文字列)
    }

    /// 入力欄へ書き戻す文字列。
    pub(crate) fn 文字列(&self) -> &str {
        &self.0
    }

    /// 前後の空白を除いた入力をフォルダとして読む。空なら無い。
    pub(crate) fn フォルダとして読む(&self) -> Option<FFmpegを置いたフォルダ> {
        let 中身 = self.0.trim();
        (!中身.is_empty())
            .then(|| FFmpegを置いたフォルダ::作成する(PathBuf::from(中身)))
    }
}

/// FFmpegの状況とは、ffmpeg と ffprobe が見つかったか、見つからずに置き場所の入力を待っているかの区別のことである。
#[derive(Debug, Clone)]
pub(crate) enum FFmpegの状況 {
    見つかった(FFmpegの実行ファイル),
    見つからない {
        理由: FFmpegが見つからないエラー,
        入力中のフォルダ: 入力中のフォルダ,
    },
}
