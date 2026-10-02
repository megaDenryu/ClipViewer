//! 回帰試験(b)。重ね合わせの作業場が前のとき、スタックの作業場のキーを押しても `アプリの状態` が変わらないことを確かめる。
//! 押したキーがスタックの作業場のキーの表へ届く道具であることは、スタックの作業場が前のときに同じ道具で T が構えを変えることで確かめる。
//! 参照: _doc/設計/同時再生.md 3-2

use eframe::egui;

use super::launch_requests::起動の頼みの届き方;
use super::workspace_test_frames::{
    キーを押して適用する, 何も押さずに一フレーム進める
};
use super::workspace_test_snapshot::スタックの作業場の写し;
use super::workspace_test_support::スタックの作業場の値を既定から変えて再生しているビューアー;
use crate::state::画面の構え;

/// スタックの作業場の既定のキーの表から、再生・クリップの移動・編集・構え・ダイアログ・保存・全画面に当たるキーを選んだもの。
fn スタックの作業場のキー() -> Vec<(egui::Key, egui::Modifiers)> {
    let 無し = egui::Modifiers::NONE;
    let シフト = egui::Modifiers::SHIFT;
    let コマンド = egui::Modifiers::COMMAND;
    vec![
        (egui::Key::Space, 無し),
        (egui::Key::K, 無し),
        (egui::Key::Enter, 無し),
        (egui::Key::PageUp, 無し),
        (egui::Key::PageDown, 無し),
        (egui::Key::ArrowLeft, 無し),
        (egui::Key::ArrowRight, 無し),
        (egui::Key::ArrowLeft, シフト),
        (egui::Key::ArrowRight, シフト),
        (egui::Key::ArrowUp, 無し),
        (egui::Key::ArrowDown, 無し),
        (egui::Key::OpenBracket, 無し),
        (egui::Key::CloseBracket, 無し),
        (egui::Key::M, 無し),
        (egui::Key::T, 無し),
        (egui::Key::I, 無し),
        (egui::Key::O, 無し),
        (egui::Key::Y, 無し),
        (egui::Key::F1, 無し),
        (egui::Key::F11, 無し),
        (egui::Key::Escape, 無し),
        (egui::Key::Delete, 無し),
        (egui::Key::Z, コマンド),
        (egui::Key::Y, コマンド),
        (egui::Key::S, コマンド),
    ]
}

#[test]
fn 道具の確かめ_スタックの作業場が前なら構えを切り替えるキーで編集の構えへ戻る() {
    let mut ビューアー =
        スタックの作業場の値を既定から変えて再生しているビューアー();
    assert_eq!(ビューアー.状態.出力.構え, 画面の構え::シアター);
    キーを押して適用する(&mut ビューアー, egui::Key::T, egui::Modifiers::NONE);
    assert_eq!(ビューアー.状態.出力.構え, 画面の構え::編集);
}

#[test]
fn 回帰試験b_重ね合わせの作業場が前のときスタックの作業場のキーを押してもアプリの状態は変わらない()
{
    let mut ビューアー =
        スタックの作業場の値を既定から変えて再生しているビューアー();
    ビューアー.重ね合わせの作業場へ移る();
    何も押さずに一フレーム進める(&mut ビューアー);
    let 移った後 = スタックの作業場の写し::写しを取る(&ビューアー);
    for (キー, 修飾キー) in スタックの作業場のキー() {
        キーを押して適用する(&mut ビューアー, キー, 修飾キー);
        assert_eq!(
            スタックの作業場の写し::写しを取る(&ビューアー),
            移った後,
            "{キー:?} {修飾キー:?}"
        );
        assert!(!ビューアー.状態.再生.再生しているか(), "{キー:?}");
    }
    assert!(
        ビューアー
            .ウインドウへの指示の並び(None, 起動の頼みの届き方::届いていない)
            .is_empty()
    );
}
