//! 操作コマンドの層。画面が発する操作を応答として列挙し、状態へ適用する。参照: _doc/設計/画面.md 判断1

mod applier;
mod band_commit;
mod band_ops;
mod clip_add;
mod clip_arrange;
mod clip_edit;
mod clip_ops;
mod clue_ops;
mod crop_ops;
mod ffmpeg_ops;
mod file_drop;
mod file_ops;
mod id_issuer;
mod library;
mod output_ops;
mod playback_ops;
mod position_target;
mod previous_clip;
mod relation_end;
mod seek;
mod single_play;
mod span_edit;
mod stack_export;
mod video_opener;
mod video_ops;

#[cfg(test)]
mod tests;

pub(crate) use applier::操作の適用係;
pub(crate) use band_commit::主ボタンの様子;
pub(crate) use band_ops::{クリップの区間の帯の操作, 区間の帯の段階};
pub(crate) use clip_arrange::動かす向き;
pub(crate) use clip_edit::クリップの編集;
pub(crate) use clip_ops::クリップの操作;
pub(crate) use clue_ops::手がかりの操作;
pub(crate) use crop_ops::{クロップ枠の掴む所, クロップ枠の操作};
pub(crate) use ffmpeg_ops::FFmpegの操作;
pub(crate) use file_drop::落としたファイル;
pub(crate) use file_ops::ファイルの操作;
pub(crate) use library::ライブラリの操作;
pub(crate) use output_ops::出力の操作;
pub(crate) use playback_ops::{再生の操作, 送る向き};
pub(crate) use span_edit::{区間の端, 端のずらし方};

/// 応答とは、画面が発する操作を、画面の区画ごとの操作の列挙で束ねたもののことである。
/// 画面は描画の間にこの値を集めるだけで、状態へ当てるのは描画の後の `操作の適用係::適用する` である。
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum 応答 {
    ファイル(ファイルの操作),
    手がかり(手がかりの操作),
    クリップ(クリップの操作),
    クロップ枠(クロップ枠の操作),
    出力(出力の操作),
    再生(再生の操作),
    FFmpeg(FFmpegの操作),
    ライブラリ(ライブラリの操作),
}
