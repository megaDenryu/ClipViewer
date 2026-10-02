//! フォーカスを持つボタンのキーの試験。フォーカスを持つボタンは Space で押されず、Space は再生と停止だけを発する。
//! 入力欄にフォーカスがあるときのキーは keys_text_field.rs が確かめる。

use eframe::egui;

use super::key_support::{キーの押下, 事象を渡して描く};
use super::クリップを並べた状態;
use crate::command::{再生の操作, 応答};
use crate::state::アプリの状態;

const 無し: egui::Modifiers = egui::Modifiers::NONE;

/// Tab でフォーカスを部品へ移した egui の本体。移った先があることを確かめて返す。
fn フォーカスを移した本体(状態: &アプリの状態) -> egui::Context {
    let 画面描画の共有状態 = egui::Context::default();
    let _ = 事象を渡して描く(&画面描画の共有状態, 状態, Vec::new(), 無し);
    let _ = 事象を渡して描く(
        &画面描画の共有状態,
        状態,
        vec![キーの押下(egui::Key::Tab, 無し)],
        無し,
    );
    assert!(
        画面描画の共有状態.memory(|記憶| 記憶.focused()).is_some(),
        "Tab でフォーカスが移らなかった"
    );
    画面描画の共有状態
}

#[test]
fn フォーカスを持つボタンは空白キーで押されず_空白キーは再生と停止だけを発する() {
    let 状態 = クリップを並べた状態(Vec::new());
    let 画面描画の共有状態 = フォーカスを移した本体(&状態);
    let mut 集めた = 事象を渡して描く(
        &画面描画の共有状態,
        &状態,
        vec![キーの押下(egui::Key::Space, 無し)],
        無し,
    );
    集めた.extend(事象を渡して描く(
        &画面描画の共有状態,
        &状態,
        Vec::new(),
        無し,
    ));
    assert_eq!(集めた, [応答::再生(再生の操作::再生を切り替える)]);
}
