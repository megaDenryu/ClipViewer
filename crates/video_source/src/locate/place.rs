//! FFmpeg の実行ファイルを探す場所の型。

use std::path::{Path, PathBuf};

/// FFmpegを置いたフォルダとは、ffmpeg と ffprobe の実行ファイルが入っているとアプリの設定に書いたフォルダのことである。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FFmpegを置いたフォルダ(PathBuf);

impl FFmpegを置いたフォルダ {
    /// フォルダのパスから作成する。
    pub fn 作成する(フォルダ: PathBuf) -> Self {
        Self(フォルダ)
    }

    /// フォルダのパス。
    pub fn パス(&self) -> &Path {
        &self.0
    }
}

/// FFmpegの置き場所の設定とは、アプリの設定に FFmpeg を置いたフォルダが書かれているかの区別のことである。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FFmpegの置き場所の設定 {
    /// 設定に書かれていない。PATH だけを探す。
    未設定,
    /// 設定に書かれている。PATH より先に探す。
    設定済み(FFmpegを置いたフォルダ),
}

/// 実行ファイルの検索パスとは、環境変数 PATH に並んだフォルダの列のことである。
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct 実行ファイルの検索パス(Vec<PathBuf>);

impl 実行ファイルの検索パス {
    /// フォルダの列から作成する。
    pub fn 作成する(フォルダの列: Vec<PathBuf>) -> Self {
        Self(フォルダの列)
    }

    /// 今のプロセスの環境変数 PATH から読む。環境変数を読む境界はここ1箇所である。
    pub fn 環境変数から読む() -> Self {
        let 値 = std::env::var_os("PATH").unwrap_or_default();
        Self(std::env::split_paths(&値).collect())
    }

    /// 並んだフォルダを順に返す。
    pub(crate) fn フォルダの列(&self) -> &[PathBuf] {
        &self.0
    }
}
