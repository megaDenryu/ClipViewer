//! ライブラリの一覧の行の並び。仮想縦スクロールで見えている行だけを組む部品は2つの作業場の画面が共有する `library_common::view` が持ち、
//! ここはスタックの行の材料と、見えている行を知らせる応答を渡す。

use std::rc::Rc;

use clip_library::ライブラリの一覧;
use sengen_egui::ノード;

use super::row_source::行の材料;
use crate::command::{ライブラリの操作, 応答};
use crate::library_common::view::一覧の行の並び as 共有の行の並び;
use crate::state::library::見えている行の範囲;
use crate::state::アプリの状態;

pub(super) fn 一覧の行の並び(
    状態: &アプリの状態,
    一覧: &Rc<ライブラリの一覧>,
) -> ノード<応答> {
    共有の行の並び(
        "ライブラリの一覧",
        行の材料::状態から作る(状態, 一覧),
        見えている行を知らせる,
    )
}

fn 見えている行を知らせる(範囲: 見えている行の範囲) -> 応答 {
    応答::ライブラリ(ライブラリの操作::一覧の見えている行を知らせる(範囲))
}
