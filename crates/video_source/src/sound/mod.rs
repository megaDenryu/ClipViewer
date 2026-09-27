//! 音の読み出し。FFmpeg で区間の音を PCM(f32le・2ch・出力装置のサンプリング周波数)として読み、
//! 音の倉庫へ溜めるか、流し読みで流れてくる音へ書き足す。参照: _doc/設計/アーキテクチャ.md 判断8

mod launch;
mod pcm_reader;
mod plan;
mod read_stop;
mod reader_sound;
mod rows;
mod spec;
mod status;
mod store;
mod store_shared;
mod store_worker;
mod stream;
mod stream_outcome;
mod stream_reader;

#[cfg(test)]
mod pcm_reader_tests;
#[cfg(test)]
mod rows_tests;

pub use plan::{音のメモリの上限, 音の溜め方};
pub use status::音の区間の状況;
pub use store::音の倉庫;
pub use stream::音の流し読み;
pub use stream_outcome::音の流し読みの状態;
