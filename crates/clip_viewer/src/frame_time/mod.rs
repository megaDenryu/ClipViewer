//! 1フレームの時間の計測。`cargo xtask frame-time` が起動したアプリの中で、重ね合わせの8行を映して再生している間の1フレームの時間(GPU へ載せる時間を含む)を測り、
//! まとめの文を結果のファイルへ書く。頼みの値(`request.rs`)と、測った時間とそのまとめ(`record.rs`)を持つ。アプリを動かして測る手順は配線(`app/frame_time_*.rs`)が持つ。
//! 参照: _doc/設計/同時再生.md 5-4

mod record;
mod request;

#[cfg(test)]
mod record_tests;

pub(crate) use record::測ったフレームの時間;
pub(crate) use request::{
    フレームの時間を測る頼み, 動画を待つ上限, 測る前に待つ時間, 測る長さ
};
