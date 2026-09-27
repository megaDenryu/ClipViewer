//! 窓を開いているときのキーの試験。キーの一覧の窓とライブラリの窓を開いている間は、再生のキーが効かず、Escape は窓を閉じる。

use eframe::egui;

use super::key_support::キーを押して集める;
use super::クリップを並べた状態;
use crate::command::{ライブラリの操作, 出力の操作, 応答};
use crate::state::library::ライブラリの窓;
use crate::state::キーの一覧の窓;

const 無し: egui::Modifiers = egui::Modifiers::NONE;

#[test]
fn キーの一覧を開いている間は開いたキーとエスケープで閉じ_空白キーは効かない() {
    let mut 状態 = クリップを並べた状態(Vec::new());
    状態.キーの一覧 = キーの一覧の窓::開いている;
    assert_eq!(
        キーを押して集める(&状態, egui::Key::F1, 無し),
        [応答::出力(出力の操作::キーの一覧を切り替える)]
    );
    assert_eq!(
        キーを押して集める(&状態, egui::Key::Escape, 無し),
        [応答::出力(出力の操作::キーの一覧を閉じる)]
    );
    assert!(キーを押して集める(&状態, egui::Key::Space, 無し).is_empty());
    assert_eq!(
        キーを押して集める(&状態, egui::Key::F11, 無し),
        [応答::出力(出力の操作::全画面を切り替える)]
    );
}

#[test]
fn ライブラリの窓を開いている間はエスケープで窓を閉じ_キーの一覧と空白キーは効かない() {
    let mut 状態 = クリップを並べた状態(Vec::new());
    状態.ライブラリ.窓 = ライブラリの窓::保存できずに続ける確認(
        "試験".to_string(),
        crate::state::library::関係を終える操作::動画を解除する,
    );
    assert_eq!(
        キーを押して集める(&状態, egui::Key::Escape, 無し),
        [応答::ライブラリ(ライブラリの操作::窓を閉じる)]
    );
    assert!(キーを押して集める(&状態, egui::Key::F1, 無し).is_empty());
    assert!(キーを押して集める(&状態, egui::Key::Space, 無し).is_empty());
}
