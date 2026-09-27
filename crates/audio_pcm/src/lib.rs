//! 音の値の型と、音を受け渡す入れ物の層。
//!
//! 標本とは、ある1つの時点の左右の音の値の組のことである。本クレートは、サンプリング周波数・標本数・動画上の標本位置・
//! 左右の標本・音量の型と、溜めた音(区間を読み切った不変の音)・流れてくる音(流し読みの裏のスレッドが書き足し、
//! 音声のスレッドが読む入れ物)を持つ。音を作る側(video_source)と鳴らす側(audio_output)は互いを知らず、この型だけで受け渡す。
//! 参照: _doc/設計/アーキテクチャ.md 判断8

#![warn(missing_docs)]

mod difference;
mod flowing;
mod flowing_ring;
mod fractional;
mod fractional_range;
mod position;
mod range;
mod sample_rate;
mod source;
mod stereo;
mod stored;
mod volume;

#[cfg(test)]
mod flowing_tests;
#[cfg(test)]
mod tests;

pub use difference::標本の差;
pub use flowing::{書き足した結果, 流れてくる音};
pub use fractional::小数の標本位置;
pub use fractional_range::小数の標本の範囲;
pub use position::{動画上の標本位置, 標本数};
pub use range::標本の範囲;
pub use sample_rate::サンプリング周波数;
pub use source::音の出どころ;
pub use stereo::{ステレオの標本, 左右の値, 左右の値のバイト数};
pub use stored::溜めた音;
pub use volume::音量;
