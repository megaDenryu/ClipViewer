//! ClipViewer のドメイン層。画面にも動画のデコードにも依存しない、クリップ再生の型と規則を持つ。
//!
//! クリップとは元の動画の区間を切り出したものであり、クリップスタックとはクリップを再生順に並べたものである。
//! 本クレートは、クリップと時間・クロップ・繰り返し・進む条件の型、タイムライン上の位置で再生するものを求める処理、
//! 設定ファイルの読み書き、スタックのライブラリの項目と形式、動画パスの検証、クロップ枠の移動と大きさの変更の計算、
//! 同時再生で再生する重ね合わせの型と操作を持つ。
//! 参照: _doc/設計/アーキテクチャ.md

#![warn(missing_docs)]

mod aspect;
mod clip;
mod clip_ending;
mod clip_id;
mod crop;
mod duration;
mod frame;
mod library;
mod overlay;
mod percent;
mod pixel;
mod play_mode;
mod playback_clock;
mod playback_next;
mod playback_position;
mod playback_previous;
mod repeat;
mod rounding;
mod settings;
mod span_edit;
mod span_end;
mod span_place;
mod stack;
mod stack_error;
mod stack_layout;
mod stack_navigation;
mod stack_non_empty;
mod stack_previous;
mod time;
mod timeline;
mod timeline_result;
mod trigger;
mod video_path;
mod video_path_check;
mod video_span;
mod volume;

pub use aspect::アスペクト比設定;
pub use clip::{クリップ, クリップ名};
pub use clip_id::{
    クリップ識別子, クリップ識別子の発行元, 発行時刻, 空の識別子エラー, 識別子の乱数,
};
pub use crop::{クロップの移動量, クロップ範囲};
pub use duration::{時間の値エラー, 時間の長さ};
pub use frame::{四隅のつまみ, 枠の掴む所, 枠の移動量};
pub use library::{
    サムネイルの大きさ, サムネイルの撮り方, サムネイルの画像, スタックの名前, スタックの識別子,
    スタックの識別子の不備, ライブラリのクリップの不備, ライブラリのスタック,
    ライブラリのファイルの書き出しエラー, ライブラリのファイルの本文,
    ライブラリのファイルの読み込みエラー, ライブラリの日時, ライブラリの日時エラー,
    ライブラリへ取り込めない理由, 空のスタックの名前エラー,
};
pub use overlay::*;
pub use percent::{
    元の動画に対する, 百分率, 百分率の差分, 百分率の差分エラー, 等分した区間
};
pub use pixel::{画素の寸法, 画素の寸法エラー, 縦横比};
pub use play_mode::再生モード;
pub use playback_clock::{
    再生速度, 再生速度エラー, 時計を進めた結果, 時計を進める条件, 末尾での振る舞い,
};
pub use playback_next::{次へ進めた結果, 進めない理由};
pub use playback_position::再生位置;
pub use playback_previous::前へ戻した結果;
pub use repeat::{
    リピート回数, リピート回数エラー, リピート設定, 周回番号
};
pub use rounding::{
    境目を許容して切り上げた整数, 境目を許容して切り捨てた整数, 小数を切り上げた整数,
    小数を切り捨てた整数, 小数を四捨五入した整数,
};
pub use settings::{
    JSONの不備, クリップの値の不備, スタック設定, 書き出し日時, 書き出し日時エラー, 書き出す設定,
    設定ファイルの書き出しエラー, 設定ファイルの本文, 設定ファイルの読み込みエラー,
};
pub use span_edit::区間の編集の決まり;
pub use span_end::区間の終わりの行き先;
pub use stack::クリップスタック;
pub use stack_error::クリップスタックの操作エラー;
pub use stack_layout::開始位置付きのクリップ;
pub use stack_navigation::次のクリップの探索結果;
pub use stack_non_empty::空でないクリップスタック;
pub use stack_previous::前のクリップの探索結果;
pub use time::{
    タイムライン上, タイムライン上の秒, 動画上, 動画上の秒, 時刻
};
pub use timeline_result::{
    タイムライン上の位置で再生するもの, タイムライン上の位置の再生の中身, 周回の状況,
    表示するコマの位置,
};
pub use trigger::{トリガー, 進める操作};
pub use video_path::{
    入力された動画パス, 動画ファイル名, 正規化した動画パス
};
pub use video_path_check::{
    動画パスの不備, 動画パスの検証結果, 検証済みの動画パス
};
pub use video_span::{動画上の区間, 区間エラー};
pub use volume::音量;
