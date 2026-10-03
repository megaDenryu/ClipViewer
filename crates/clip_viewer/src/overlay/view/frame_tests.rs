//! 重ねる画面の上の映す矩形の枠の試験。枠の内側をドラッグすると選んで掴み始め、内側の移動を発して放すことと、押しただけでも選ぶことと、何も無い所を押すと選びを外すことと、
//! egui に描かせて確かめる。変換の規則の試験は `frame_conversion_tests.rs` に置く。
#![allow(clippy::expect_used)]

use clip_domain::枠の掴む所;
use eframe::egui;

use super::screen_press_support::試験の全体の音量;
use super::test_support::{描いて集める, 黒く塗った矩形};
use crate::overlay::command::{
    映す矩形の枠の操作, 置き方の操作, 重ね合わせの作業場の応答, 重ね合わせの操作,
};
use crate::overlay::placement_test_support::{
    置いたクリップの識別子を作る, 開いた作業場
};
use crate::overlay::state::今のスタックから並べる状況;
use crate::overlay::workspace::重ね合わせの作業場;

/// 1フレームずつ、押す・動かす・放すの事象を描かせて、発した応答をつなげて返す。
fn ドラッグして応答を集める(
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

#[test]
fn 枠の内側をドラッグすると選んで掴み始め_内側の移動を発して放す() {
    let 作業場 = 開いた作業場();
    let (_, 集まり) = ドラッグして応答を集める(
        &作業場,
        egui::pos2(200.0, 100.0),
        egui::pos2(260.0, 100.0),
    );
    assert_eq!(
        集まり[..2],
        [
            置き方の操作::置いたクリップを選ぶ(
                置いたクリップの識別子を作る("置-1")
            )
            .応答にする(),
            置き方の操作::映す矩形の枠(映す矩形の枠の操作::掴み始めた(
                置いたクリップの識別子を作る("置-1")
            ))
            .応答にする(),
        ]
    );
    let 放した = 集まり.last().expect("放したを発した");
    let 重ね合わせの作業場の応答::操作(重ね合わせの操作::置き方(
        置き方の操作::映す矩形の枠(映す矩形の枠の操作::放した(動き)),
    )) = 放した
    else {
        panic!("最後は放した: {放した:?}");
    };
    assert_eq!(動き.掴む所, 枠の掴む所::内側);
    assert!(動き.移動量.横 > clip_domain::百分率の差分::default());
}

#[test]
fn 何も無い所を押すと選びを外す() {
    let 作業場 = 開いた作業場();
    let (_, 集まり) = ドラッグして応答を集める(
        &作業場,
        egui::pos2(-4.0, -4.0),
        egui::pos2(-4.0, 30.0),
    );
    assert_eq!(集まり, [置き方の操作::選びを外す.応答にする()]);
}

#[test]
fn 枠を押しただけでも選び_当てても置き方は変わらない() {
    let mut 作業場 = 開いた作業場();
    let (_, 集まり) = ドラッグして応答を集める(
        &作業場,
        egui::pos2(200.0, 100.0),
        egui::pos2(200.0, 100.0),
    );
    for 応答 in 集まり {
        let 重ね合わせの作業場の応答::操作(操作) = 応答 else {
            panic!("重ね合わせの作業場の中の操作だけを発する: {応答:?}");
        };
        作業場.操作を適用する(操作);
    }
    let 開いている = 作業場.状態().開いている重ね合わせ().expect("開いている");
    assert!(
        開いている
            .置いたクリップを選んでいるか(&置いたクリップの識別子を作る("置-1"))
    );
    assert!(!開いている.置き方を手で直したか());
}
