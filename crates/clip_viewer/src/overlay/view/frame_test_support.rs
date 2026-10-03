//! 重ねる画面の枠の試験の道具。押す・動かす・放すの事象を1フレームずつ描かせて、発した応答を集めることを受け持つ。
#![allow(clippy::expect_used)]

use eframe::egui;

use super::screen_press_support::試験の全体の音量;
use super::test_support::{描いて集める, 黒く塗った矩形};
use crate::overlay::command::重ね合わせの作業場の応答;
use crate::overlay::state::今のスタックから並べる状況;
use crate::overlay::workspace::重ね合わせの作業場;

/// 1フレームずつ、押す・動かす・放すの事象を描かせて、発した応答をつなげて返す。
pub(super) fn ドラッグして応答を集める(
    作業場: &重ね合わせの作業場,
    始め: egui::Pos2,
    終わり: egui::Pos2,
) -> (egui::Rect, Vec<重ね合わせの作業場の応答>) {
    let 画面描画の共有状態 = egui::Context::default();
    let 木 = || {
        super::画面(
            作業場.状態(),
            今のスタックから並べる状況::並べられる,
            None,
            試験の全体の音量,
            None,
        )
    };
    let (_, 出力) = 描いて集める(&画面描画の共有状態, Vec::new(), &木);
    let 地 = 黒く塗った矩形(&出力).expect("重ねる画面を描いた");
    let ボタン = |位置, 押した| egui::Event::PointerButton {
        pos: 位置,
        button: egui::PointerButton::Primary,
        pressed: 押した,
        modifiers: egui::Modifiers::NONE,
    };
    let 始め = 始め + 地.min.to_vec2();
    let 終わり = 終わり + 地.min.to_vec2();
    let 間 = 始め + (終わり - 始め) * 0.5;
    let 事象の並び = [
        vec![egui::Event::PointerMoved(始め), ボタン(始め, true)],
        vec![egui::Event::PointerMoved(間)],
        vec![egui::Event::PointerMoved(終わり)],
        vec![ボタン(終わり, false)],
    ];
    let 集まり = 事象の並び
        .into_iter()
        .flat_map(|事象| 描いて集める(&画面描画の共有状態, 事象, &木).0)
        .collect();
    (地, 集まり)
}
