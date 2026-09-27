//! 永続化境界。設定ファイル(JSON)とアプリの設定の読み書き、ファイルダイアログ、FFmpeg とスタックのライブラリとサムネイルのキャッシュの置き場所の決め方を持つ。
//! アプリの状態を知らず、読み書きした値を返すだけにする(状態へ当てるのは `command`)。参照: _doc/設計/画面.md

mod app_settings;
mod export_name;
mod ffmpeg_place;
mod launch_place;
mod library_place;
mod stack_file;
mod thumbnail_place;

#[cfg(test)]
mod tests;

pub(crate) use app_settings::{
    アプリの設定の保存の結果, アプリの設定の保管場所
};
pub(crate) use export_name::設定ファイルの既定の名前;
pub(crate) use ffmpeg_place::FFmpegの置き場所の候補;
pub(crate) use stack_file::{ファイルの窓口, 設定ファイルのパス};
pub(crate) use thumbnail_place::サムネイルのキャッシュを決める;
