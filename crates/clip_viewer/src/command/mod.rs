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
mod key_settings_ops;
mod library;
mod new_stack;
mod next_clip;
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
pub(crate) use crop_ops::クロップ枠の操作;
pub(crate) use ffmpeg_ops::FFmpegの操作;
pub(crate) use file_drop::一度に落としたファイル;
pub(crate) use file_ops::ファイルの操作;
pub(crate) use key_settings_ops::キーの設定の操作;
pub(crate) use library::ライブラリの操作;
pub(crate) use output_ops::出力の操作;
pub(crate) use playback_ops::{再生の操作, 送る向き};
pub(crate) use span_edit::{区間の端, 端のずらし方};

/// 応答とは、画面が発する操作を、画面の区画ごとの操作の列挙で束ねたもののことである(キーの設定はキーの一覧のダイアログの操作)。
/// 画面は描画の間にこの値を集めるだけで、状態へ当てるのは描画の後の `操作の適用係::適用する` である。
/// 重ね合わせの作業場へ移るは、ヘッダーの「同時再生」が発し、スタックの作業場の状態へは当てない。配線(`app/workspace.rs`)が先に受け取って作業場を移すため、
/// `操作の適用係::適用する` へは届かない(届いたら到達したらバグとして止める)。参照: _doc/設計/同時再生.md 3-2
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
    キーの設定(キーの設定の操作),
    重ね合わせの作業場へ移る,
}
