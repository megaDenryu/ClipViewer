//! 重ね合わせの供給。使う動画の表の動画ごとの供給(コマの倉庫と依頼の台帳)と、タイムラインの行ごとの供給(流し読みとテクスチャ)を持ち、
//! 重ね合わせ上の時刻に行ごとに映すコマを、溜めたコマか流し読みから取り出して行のテクスチャへ載せる。
//! スタックの作業場の `video_feed` からは、依頼の台帳と載せ直しの鍵と、表示するコマの求め・コマの出どころ・動画を開けない理由の値の型だけを使い、
//! `読み込んだ動画` を使わない。参照: _doc/設計/同時再生.md 3-3・5-1・5-2

mod prefetch;
mod row;
mod row_query;
mod row_request;
mod row_stream;
mod row_texture;
mod status;
mod supply;
mod video;

#[cfg(test)]
mod prefetch_tests;
#[cfg(test)]
mod with_ffmpeg_open_failure_tests;
#[cfg(test)]
mod with_ffmpeg_stream_tests;
#[cfg(test)]
mod with_ffmpeg_support;
#[cfg(test)]
mod with_ffmpeg_tests;

pub(crate) use status::重ね合わせの映像の様子;
