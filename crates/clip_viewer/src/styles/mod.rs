//! 画面の装飾。テーマと名前付きのスタイルをここへ集める。2つの作業場の画面が共有する(スタックの作業場の画面は `view::styles` の再公開から読む)。配色は移植元の style.css.ts(濃い青灰の地と琥珀色の強調)に寄せる。
//! 配色の定数(palette)、区画の地(area)、文字と部品の装飾(parts)の3つに分け、呼び出し側は `styles::名前` で参照する。

mod area;
mod palette;
mod parts;

pub(crate) use area::*;
pub(crate) use parts::*;

use sengen_egui::{テーマ, 明暗, 画素の組, 角丸画素};

pub(crate) const 画面のテーマ: テーマ = テーマ {
    強調色: Some(palette::琥珀),
    地の色: Some(palette::左ペインの地),
    文字色: Some(palette::文字の色),
    部品の面の色: Some(palette::線と部品の面の色),
    強調色の上の文字色: Some(palette::最も濃い地),
    部品の間隔: Some(画素の組(8.0, 6.0)),
    ボタンの内余白: Some(画素の組(8.0, 3.0)),
    部品の角丸: Some(角丸画素(4)),
    ..テーマ::基調から作る(明暗::濃色)
};
