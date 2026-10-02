//! 映像の供給。読み込んだ動画を持ち、先読みの依頼と、表示するコマを溜めたコマか流し読みから取り出してテクスチャへ載せることを受け持つ。
//! 1本の動画のコマを出す動画のコマの供給元と、表示するコマを載せるコマの載せ先は、スタックの作業場の読み込んだ動画と重ね合わせの供給
//! (`overlay/feed/`)が共に使う。重ね合わせの供給は、ほかに表示するコマの求めと動画を開けない理由の値の型を使い、`読み込んだ動画` を使わない。
//! 参照: _doc/設計/画面.md 判断2〜判断5、_doc/設計/同時再生.md 3-3

mod frame_request;
mod frame_source;
mod frame_target;
mod frame_target_query;
mod frame_target_stream;
mod ledger;
mod ledger_query;
mod ledger_record;
mod ledger_state;
mod loaded_frame_source;
mod loaded_video;
mod open_failure;
mod prefetch_order;
mod stream_slot;
mod texture_key;
mod video_texture;

#[cfg(test)]
mod ledger_retry_tests;
#[cfg(test)]
mod ledger_test_support;
#[cfg(test)]
mod ledger_tests;
#[cfg(test)]
mod open_failure_tests;
#[cfg(test)]
mod prefetch_order_tests;
#[cfg(test)]
mod pure_tests;
#[cfg(test)]
mod stream_position_tests;
#[cfg(test)]
mod with_ffmpeg_stream_tests;
#[cfg(test)]
pub(crate) mod with_ffmpeg_support;
#[cfg(test)]
mod with_ffmpeg_tests;

pub(crate) use frame_request::{コマの出どころ, 表示するコマの求め};
pub(crate) use frame_source::動画のコマの供給元;
pub(crate) use frame_target::コマの載せ先;
#[cfg(test)]
pub(crate) use ledger::依頼の台帳;
pub(crate) use ledger_state::{依頼の結末, 依頼の進み具合, 行の状態};
#[cfg(test)]
pub(crate) use loaded_frame_source::載せているコマの出どころ;
pub(crate) use loaded_video::読み込んだ動画;
pub(crate) use open_failure::{動画を開く経路, 動画を開けない理由};
pub(crate) use prefetch_order::先読みの並び;
pub(crate) use video_texture::テクスチャの名前;
