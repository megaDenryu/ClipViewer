//! 重ね合わせの供給。使う動画の表の動画ごとの動画のコマの供給元(読み手・コマの倉庫・依頼の台帳)と、タイムラインの行ごとのコマの載せ先(流し読みとテクスチャ)を持ち、
//! 重ね合わせ上の時刻に行ごとに映すコマを、溜めたコマか流し読みから取り出して行のテクスチャへ載せる。
//! 供給元と載せ先の型と、表示するコマの求め・動画を開けない理由の値の型は、スタックの作業場の `video_feed` と共有し、
//! `読み込んだ動画` を使わない。音の側(重ね合わせの音の供給)も同じ形で、動画ごとの動画の音の供給元と行ごとの音の流し読みを
//! スタックの作業場の `audio_feed` と共有する。参照: _doc/設計/同時再生.md 3-3・5-1・5-2・5-3

mod frame_targets;
mod prefetch;
mod row_request;
mod show_condition;
mod sound_query;
mod sound_supply;
mod sound_video;
mod status;
mod supply;
mod supply_failure;
mod supply_query;
mod video_status;

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

pub(crate) use frame_targets::このフレームで映すもの;
pub(crate) use show_condition::映すものを求める条件;
pub(crate) use sound_supply::重ね合わせの音の供給;
pub(crate) use status::重ね合わせの映像の供給を作れたか;
#[cfg(test)]
pub(crate) use supply_failure::映像の供給を作れない理由;
