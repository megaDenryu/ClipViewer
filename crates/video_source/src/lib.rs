//! FFmpeg を外部プロセスとして呼ぶ境界の層。動画の情報の取得、コマの倉庫(区間のコマを裏のスレッドでデコードして RAM に溜める)、
//! 流し読み(溜めない区間と元動画の再生)を持つ。画面にも、クリップや再生の規則にも依存しない。
//!
//! 使い方の流れ: `FFmpegの実行ファイル::探す` で ffmpeg と ffprobe を見つけ、`動画の読み手` を作る。
//! 動画を読み込んだら `動画の情報を取る` で調べ、`コマの倉庫を開く` で倉庫を作る。音があるかは `動画の情報::音の有無` で確かめ、
//! `音の倉庫を開く` と `音の流し読みを開く` で読む。
//! ライブラリの一覧に出すスタックの顔は `サムネイルを作る` で JPEG にする(先に ffprobe で動画の情報を調べ、期限を過ぎるか `取り消しの合図` で取り消されたら ffprobe も ffmpeg も止める。続けて撮るなら `撮影に使う動画の情報の覚え` で動画の情報を使い回す)。
//! 参照: _doc/設計/アーキテクチャ.md 判断1・判断2

#![warn(missing_docs)]

mod bytes;
mod estimate;
mod frame;
mod frame_number;
mod frame_rate;
#[cfg(test)]
mod frame_rate_tests;
mod frame_size;
mod locate;
mod long_side;
mod probe;
mod process;
mod read_position;
mod reader;
mod sound;
mod store;
mod stream;
mod thumbnail;
mod video_info;

#[cfg(test)]
mod video_info_tests;

pub use bytes::{バイト数, メモリの上限};
pub use estimate::区間の見積もり;
pub use frame::{コマ, 番号付きのコマ};
pub use frame_number::{コマ数, 動画上のコマ番号};
pub use frame_rate::コマの速さ;
pub use frame_size::コマの寸法;
pub use locate::{
    FFmpegが見つからないエラー, FFmpegの実行ファイル, FFmpegの置き場所の設定, FFmpegの道具,
    FFmpegを置いたフォルダ, 実行ファイルの検索パス, 探した場所,
};
pub use long_side::長辺の上限;
pub use probe::{動画の情報の取得エラー, 読めない項目, 音の有無};
pub use process::{デコードの失敗, 取り消しの合図, 標準エラーの末尾};
pub use reader::動画の読み手;
pub use sound::{
    音のメモリの上限, 音の倉庫, 音の区間の状況, 音の流し読み, 音の流し読みの状態, 音の溜め方,
};
pub use store::{
    コマの倉庫, 依頼の状況, 保持の優先順, 倉庫を開くエラー, 受け付けない理由, 受付の札,
    溜める依頼の結果, 溜め終えた内訳,
};
pub use stream::{流し読み, 流し読みのコマ, 流し読みの状態};
pub use thumbnail::{サムネイルの撮影の失敗, 撮影に使う動画の情報の覚え};
pub use video_info::動画の情報;
