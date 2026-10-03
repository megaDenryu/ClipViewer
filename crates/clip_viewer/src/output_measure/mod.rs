//! 出力の寸法と描く部分。スタックの作業場の画面(`view/`)と重ね合わせの作業場の画面(`overlay/view/`)が共有する、
//! 縦横比を保った寸法の計算と、出力の欄の余白の内側と、クロップ範囲を描く部分にする変換と、範囲枠の操作の値の変換を置く。
//! 2つの作業場の画面はお互いを知らないため、どちらにも属さない置き場にする(`redraw_interval.rs` と同じ扱い)。

mod aspect_fit;
mod crop_part;
mod frame_operation;
mod output_area;

#[cfg(test)]
mod aspect_fit_tests;

pub(crate) use aspect_fit::{丸める前の論理画素の寸法, 描く縦横比};
pub(crate) use crop_part::クロップを描く部分にする;
pub(crate) use frame_operation::{
    割合の差を枠の移動量にする, 掴んだ部分を掴む所にする
};
