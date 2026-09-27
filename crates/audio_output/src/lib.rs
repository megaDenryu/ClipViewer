//! 音声出力の層。音声出力装置(cpal)を開く境界と、装置が音を求めるたびに次の標本を作る音の再生器を持つ。
//!
//! 画面のスレッドは毎フレーム `再生の指示` を組み立てて `音声出力::指示を渡す` で渡す。音声のスレッドは指示に従い、
//! 音の再生器で溜めた音や流れてくる音から標本を作る。音の再生器は装置から切り離した純粋な計算であり、装置なしで試験できる。
//! 参照: _doc/設計/アーキテクチャ.md 判断8

#![warn(missing_docs)]

mod device;
mod drift;
mod instruction;
mod player;
mod ramp;
mod reading;
mod retired;
mod section;

#[cfg(test)]
mod tests;

pub use device::{音声出力, 音声出力のエラー};
pub use instruction::{位置を飛ばした回数, 再生の指示, 再生の様子};
pub use player::音の再生器;
pub use reading::{線形補間で読む, 速さを変える読み方};
pub use retired::退いた出どころの置き場;
pub use section::{終わりで続く音, 音の区間};
