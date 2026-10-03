//! 重ね合わせの作業場のキーの試験。Space・Ctrl+Z・Ctrl+Y・F11 が対応する応答を発することと、重ね合わせのダイアログを開いている間は F11 のほかが発しないことを、
//! 画面の木を描いて確かめる。参照: _doc/設計/同時再生.md 2-8、画面.md 判断23
#![allow(clippy::expect_used)]

use clip_domain::クリップスタック;
use eframe::egui;

use super::test_support::描いて集める;
use crate::overlay::arrange_test_support::{値, 練習の動画};
use crate::overlay::command::{
    一覧とダイアログの操作, 編集の履歴の操作, 重ね合わせのライブラリの操作,
    重ね合わせの作業場の応答, 重ね合わせの操作,
};
use crate::overlay::library_test_ops::ライブラリの操作を当てる;
use crate::overlay::library_test_support::一時のライブラリ;
use crate::overlay::test_support::五秒の重ね合わせ;
use crate::overlay::workspace::重ね合わせの作業場;

fn キーの押下(キー: egui::Key, 修飾キー: egui::Modifiers) -> egui::Event {
    egui::Event::Key {
        key: キー,
        physical_key: None,
        pressed: true,
        repeat: false,
        modifiers: 修飾キー,
    }
}

/// 作業場の画面を、キーを1つ押した入力で描き、発した応答を返す。
fn キーを押して集める(
    作業場: &重ね合わせの作業場,
    キー: egui::Key,
    修飾キー: egui::Modifiers,
) -> Vec<重ね合わせの作業場の応答> {
    let スタック = クリップスタック::default();
    let パス = 練習の動画();
    let 本体 = egui::Context::default();
    描いて集める(
        &本体,
        vec![キーの押下(キー, 修飾キー)],
        &|| 作業場.画面(値(&スタック, &パス, 60.0), None),
    )
    .0
}

fn 押した組と応答() -> [(egui::Key, egui::Modifiers, 重ね合わせの作業場の応答); 4] {
    [
        (
            egui::Key::Space,
            egui::Modifiers::NONE,
            重ね合わせの作業場の応答::操作(重ね合わせの操作::再生を切り替える),
        ),
        (
            egui::Key::Z,
            egui::Modifiers::COMMAND,
            編集の履歴の操作::編集を取り消す.応答にする(),
        ),
        (
            egui::Key::Y,
            egui::Modifiers::COMMAND,
            編集の履歴の操作::編集をやり直す.応答にする(),
        ),
        (
            egui::Key::F11,
            egui::Modifiers::NONE,
            重ね合わせの作業場の応答::全画面を切り替える,
        ),
    ]
}

#[test]
fn space_ctrl_z_ctrl_y_f11はそれぞれの応答を発する() {
    let 一時 = 一時のライブラリ::作る("キーの応答");
    let 組 = 一時.錠を試す();
    let mut 作業場 = 一時.作業場を作る(組.重ね合わせ);
    作業場.重ね合わせを開く(五秒の重ね合わせ(), None);
    for (キー, 修飾キー, 応答) in 押した組と応答() {
        assert_eq!(
            キーを押して集める(&作業場, キー, 修飾キー),
            [応答],
            "{キー:?}"
        );
    }
    drop((作業場, 組.スタック));
}

#[test]
fn ダイアログを開いている間はf11のほかのキーは発しない() {
    let 一時 = 一時のライブラリ::作る("ダイアログのキー");
    let 組 = 一時.錠を試す();
    let mut 作業場 = 一時.作業場を作る(組.重ね合わせ);
    作業場.重ね合わせを開く(五秒の重ね合わせ(), None);
    ライブラリの操作を当てる(
        &mut 作業場,
        重ね合わせのライブラリの操作::一覧とダイアログ(
            一覧とダイアログの操作::登録のダイアログを開く,
        ),
    );
    assert!(作業場.状態().ダイアログを開いているか());
    for (キー, 修飾キー, 応答) in 押した組と応答() {
        let 発した = キーを押して集める(&作業場, キー, 修飾キー);
        let 全画面のキーか =
            応答 == 重ね合わせの作業場の応答::全画面を切り替える;
        assert_eq!(
            発した.contains(&応答),
            全画面のキーか,
            "{キー:?} {発した:?}"
        );
    }
    drop((作業場, 組.スタック));
}
