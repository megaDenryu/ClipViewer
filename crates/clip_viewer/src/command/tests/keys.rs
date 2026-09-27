//! 編集のキーの試験。画面を1フレーム描いてキーの押下を渡し、編集の構えでは Ctrl+Z・Ctrl+Y・Ctrl+Shift+Z・I・O が
//! 取り消し・やり直し・区間の端を今の位置にする応答を発し、ライブラリの構えでは発しないことを確かめる。

use eframe::egui;

use super::key_support::キーを押して集める;
use super::クリップを並べた状態;
use crate::command::{クリップの操作, 区間の端, 応答};
use crate::state::{アプリの状態, 画面の構え};

/// キーを1つ押して画面を1フレーム描き、発したクリップの操作を返す。
fn クリップの操作を集める(
    状態: &アプリの状態,
    キー: egui::Key,
    修飾キー: egui::Modifiers,
) -> Vec<クリップの操作> {
    キーを押して集める(状態, キー, 修飾キー)
        .into_iter()
        .filter_map(|応答| match 応答 {
            応答::クリップ(操作) => Some(操作),
            _ => None,
        })
        .collect()
}

#[test]
fn 編集の構えでは取り消しとやり直しと今の位置のキーが効く() {
    let 状態 = クリップを並べた状態(Vec::new());
    let コマンド = egui::Modifiers::COMMAND;
    let コマンドとシフト = egui::Modifiers::COMMAND | egui::Modifiers::SHIFT;
    assert_eq!(
        クリップの操作を集める(&状態, egui::Key::Z, コマンド),
        [クリップの操作::編集を取り消す]
    );
    assert_eq!(
        クリップの操作を集める(&状態, egui::Key::Y, コマンド),
        [クリップの操作::編集をやり直す]
    );
    assert_eq!(
        クリップの操作を集める(&状態, egui::Key::Z, コマンドとシフト),
        [クリップの操作::編集をやり直す]
    );
    assert_eq!(
        クリップの操作を集める(&状態, egui::Key::I, egui::Modifiers::NONE),
        [クリップの操作::選択中の区間の端を今の位置にする(区間の端::開始)]
    );
    assert_eq!(
        クリップの操作を集める(&状態, egui::Key::O, egui::Modifiers::NONE),
        [クリップの操作::選択中の区間の端を今の位置にする(区間の端::終了)]
    );
}

#[test]
fn ライブラリとシアターの構えでは編集のキーが効かない() {
    for 構え in [画面の構え::ライブラリ, 画面の構え::シアター] {
        let mut 状態 = クリップを並べた状態(Vec::new());
        状態.出力.構え = 構え;
        assert!(クリップの操作を集める(&状態, egui::Key::Z, egui::Modifiers::COMMAND).is_empty());
        assert!(クリップの操作を集める(&状態, egui::Key::I, egui::Modifiers::NONE).is_empty());
    }
}
