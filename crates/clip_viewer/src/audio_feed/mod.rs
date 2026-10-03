//! 音の供給。読み込んだ動画の音(音の倉庫と音の流し読み)を持ち、先読みの並びを倉庫へ渡し、今の区間の音の出どころを選ぶ。
//! 動画ごとの部分(動画の音の供給元)と流し読みの部分(開き直しをまたぐ音の流し読み)を分け、重ね合わせの作業場も同じ2つを動画ごとと行ごとに持つ。
//! 映像の供給(`video_feed`)を知らない(`video_feed` の読み込んだ動画が動画の音を持つため、逆向きに知ると循環する)。
//! 音声出力装置へ渡す再生の指示は、状態から `state/sound_instruction.rs` が組み立てる。参照: _doc/設計/画面.md 判断10

mod source;
mod stream_pair;
mod stream_slot;
mod supply;
mod supply_stream;
mod video_sound;

pub(crate) use source::動画の音の供給元;
pub(crate) use stream_pair::開き直しをまたぐ音の流し読み;
pub(crate) use supply::音の供給;
pub(crate) use supply_stream::音の鳴らし方;
pub(crate) use video_sound::{
    動画の音, 音が無い様子の文, 音を鳴らせない理由, 鳴らせない様子の文
};
