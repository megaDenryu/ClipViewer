//! 再生のキーの試験。編集とシアターの構えで、Space・K・Enter・PageUp・PageDown・左右・Shift+左右・[・]・M・Ctrl+S・F1・
//! F11 が表のとおりの応答を1つずつ発し、ライブラリの構えでは再生のキーが効かないことを確かめる。

use clip_domain::進める操作;
use eframe::egui;

use super::key_support::キーを押して集める;
use super::クリップを並べた状態;
use crate::command::{
    ライブラリの操作, 再生の操作, 出力の操作, 応答, 送る向き
};
use crate::state::{画面の構え, 速度を変える向き};

/// 押すキーと、発してほしい応答の組。
fn 再生のキーと応答() -> Vec<(egui::Key, egui::Modifiers, 応答)> {
    let 無し = egui::Modifiers::NONE;
    let シフト = egui::Modifiers::SHIFT;
    let 再生 = 応答::再生;
    let 出力 = 応答::出力;
    vec![
        (egui::Key::Space, 無し, 再生(再生の操作::再生を切り替える)),
        (egui::Key::K, 無し, 再生(再生の操作::再生を切り替える)),
        (
            egui::Key::Enter,
            無し,
            出力(出力の操作::次へ進める(進める操作::Enterキー)),
        ),
        (
            egui::Key::PageUp,
            無し,
            出力(出力の操作::前のクリップへ戻る),
        ),
        (
            egui::Key::PageDown,
            無し,
            出力(出力の操作::次へ進める(進める操作::次へボタン)),
        ),
        (
            egui::Key::ArrowLeft,
            無し,
            再生(再生の操作::コマ送り(送る向き::戻す)),
        ),
        (
            egui::Key::ArrowRight,
            シフト,
            再生(再生の操作::五秒ずらす(送る向き::進める)),
        ),
        (
            egui::Key::ArrowLeft,
            シフト,
            再生(再生の操作::五秒ずらす(送る向き::戻す)),
        ),
        (
            egui::Key::OpenBracket,
            無し,
            再生(再生の操作::速度を一段変える(
                速度を変える向き::下げる,
            )),
        ),
        (
            egui::Key::CloseBracket,
            無し,
            再生(再生の操作::速度を一段変える(
                速度を変える向き::上げる,
            )),
        ),
        (egui::Key::M, 無し, 再生(再生の操作::消音を切り替える)),
        (
            egui::Key::S,
            egui::Modifiers::COMMAND,
            応答::ライブラリ(ライブラリの操作::今すぐ保存する),
        ),
        (
            egui::Key::F1,
            無し,
            出力(出力の操作::キーの一覧を切り替える),
        ),
    ]
}

#[test]
fn 編集とシアターの構えでは再生のキーが表のとおりの応答を1つだけ発する() {
    for 構え in [画面の構え::編集, 画面の構え::シアター] {
        let mut 状態 = クリップを並べた状態(Vec::new());
        状態.出力.構え = 構え;
        for (キー, 修飾キー, 期待) in 再生のキーと応答() {
            assert_eq!(
                キーを押して集める(&状態, キー, 修飾キー),
                [期待],
                "{構え:?} の {キー:?}"
            );
        }
    }
}

#[test]
fn ライブラリの構えでは再生のキーが効かず_キーの一覧と全画面のキーだけが効く() {
    let mut 状態 = クリップを並べた状態(Vec::new());
    状態.出力.構え = 画面の構え::ライブラリ;
    for (キー, 修飾キー, 期待) in 再生のキーと応答() {
        let 集めた = キーを押して集める(&状態, キー, 修飾キー);
        if matches!(キー, egui::Key::F1 | egui::Key::F11) {
            assert_eq!(集めた, [期待]);
        } else {
            assert!(!集めた.contains(&期待), "{キー:?} が効いた: {集めた:?}");
        }
    }
}
