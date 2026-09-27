//! FFmpeg の実行ファイルが見つからないときのエラー。

use std::fmt;
use std::path::{Path, PathBuf};

/// FFmpegの道具とは、このクレートが呼ぶ FFmpeg の実行ファイルの区別のことである。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FFmpegの道具 {
    /// 動画をデコードする ffmpeg。
    変換,
    /// 動画の情報を調べる ffprobe。
    調査,
}

impl FFmpegの道具 {
    /// 実行ファイルのファイル名。Windows では拡張子 .exe が付く。
    pub fn ファイル名(self) -> String {
        let 名前 = match self {
            Self::変換 => "ffmpeg",
            Self::調査 => "ffprobe",
        };
        format!("{名前}{}", std::env::consts::EXE_SUFFIX)
    }
}

/// 探した場所とは、実行ファイルを探したフォルダと、それをどこから知ったかの組のことである。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum 探した場所 {
    /// アプリの設定に書いたフォルダ。
    設定の場所(PathBuf),
    /// 環境変数 PATH に並んだフォルダ。
    PATHの中(PathBuf),
}

impl 探した場所 {
    /// 探したフォルダ。
    pub fn フォルダ(&self) -> &Path {
        match self {
            Self::設定の場所(フォルダ) | Self::PATHの中(フォルダ) => フォルダ,
        }
    }
}

impl fmt::Display for 探した場所 {
    fn fmt(&self, 書き先: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::設定の場所(フォルダ) => {
                write!(書き先, "設定の場所 {}", フォルダ.display())
            }
            Self::PATHの中(フォルダ) => write!(書き先, "PATH の中 {}", フォルダ.display()),
        }
    }
}

/// FFmpegが見つからないエラーとは、設定の場所と PATH のどちらにも必要な実行ファイルが無かったことである。
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("{}。探した場所: {}", 見つからないものの説明(.見つからない道具), 探した場所の一覧(.探した場所))]
pub struct FFmpegが見つからないエラー {
    /// どの場所にも無かった実行ファイル。空なら、それぞれはあるが同じフォルダにそろう場所が無い。
    pub 見つからない道具: Vec<FFmpegの道具>,
    /// 探した順のフォルダ。設定が未設定で PATH も空なら空である。
    pub 探した場所: Vec<探した場所>,
}

fn 見つからないものの説明(道具: &[FFmpegの道具]) -> String {
    if 道具.is_empty() {
        return "ffmpeg と ffprobe が同じフォルダにそろう場所が無い".to_string();
    }
    let 名前 = 道具
        .iter()
        .map(|一つ| 一つ.ファイル名())
        .collect::<Vec<_>>();
    format!("{} が見つからない", 名前.join(" と "))
}

fn 探した場所の一覧(場所: &[探した場所]) -> String {
    if 場所.is_empty() {
        return "無し(設定の場所が未設定で、PATH も空である)".to_string();
    }
    場所
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(" / ")
}
