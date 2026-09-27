//! FFmpeg の実行ファイルを探す。参照: _doc/設計/アーキテクチャ.md 判断2

mod error;
mod executables;
mod place;

pub use error::{FFmpegが見つからないエラー, FFmpegの道具, 探した場所};
pub use executables::FFmpegの実行ファイル;
pub use place::{
    FFmpegの置き場所の設定, FFmpegを置いたフォルダ, 実行ファイルの検索パス
};
