//! フォーカスがあるときのキーの試験。フォーカスを持つボタンは Space で押されず、Space は再生と停止だけを発する。
//! 入力欄に文字を打っている間は Space を入力欄に任せ、入力欄の編集を Escape でやめた回の Escape も入力欄に任せる。

use eframe::egui;

use super::key_support::{キーの押下, 事象を渡して描く};
use super::クリップを並べた状態;
use crate::command::{再生の操作, 出力の操作, 応答};
use crate::state::アプリの状態;

const 無し: egui::Modifiers = egui::Modifiers::NONE;

/// Tab でフォーカスを部品へ移した egui の本体。移った先があることを確かめて返す。
fn フォーカスを移した本体(状態: &アプリの状態) -> egui::Context {
    let eguiの本体 = egui::Context::default();
    let _ = 事象を渡して描く(&eguiの本体, 状態, Vec::new(), 無し);
    let _ = 事象を渡して描く(
        &eguiの本体,
        状態,
        vec![キーの押下(egui::Key::Tab, 無し)],
        無し,
    );
    assert!(
        eguiの本体.memory(|記憶| 記憶.focused()).is_some(),
        "Tab でフォーカスが移らなかった"
    );
    eguiの本体
}

#[test]
fn フォーカスを持つボタンは空白キーで押されず_空白キーは再生と停止だけを発する() {
    let 状態 = クリップを並べた状態(Vec::new());
    let eguiの本体 = フォーカスを移した本体(&状態);
    let mut 集めた = 事象を渡して描く(
        &eguiの本体,
        &状態,
        vec![キーの押下(egui::Key::Space, 無し)],
        無し,
    );
    集めた.extend(事象を渡して描く(
        &eguiの本体,
        &状態,
        Vec::new(),
        無し,
    ));
    assert_eq!(集めた, [応答::再生(再生の操作::再生を切り替える)]);
}

/// 文字を打つ部品(テキスト編集)にフォーカスが移るまで Tab を押し続けた egui の本体。移らなければ試験を失敗させる。
pub(super) fn 入力欄へフォーカスを移した本体(
    状態: &アプリの状態
) -> egui::Context {
    let eguiの本体 = egui::Context::default();
    let _ = 事象を渡して描く(&eguiの本体, 状態, Vec::new(), 無し);
    let 入力欄か = |本体: &egui::Context| {
        本体
            .memory(|記憶| 記憶.focused())
            .is_some_and(|識別子| egui::text_edit::TextEditState::load(本体, 識別子).is_some())
    };
    for _ in 0..100 {
        if 入力欄か(&eguiの本体) {
            return eguiの本体;
        }
        let _ = 事象を渡して描く(
            &eguiの本体,
            状態,
            vec![キーの押下(egui::Key::Tab, 無し)],
            無し,
        );
    }
    panic!("Tab で入力欄へフォーカスが移らなかった");
}

#[test]
fn 入力欄に文字を打っている間は空白キーを入力欄に任せる() {
    let 状態 = クリップを並べた状態(Vec::new());
    let eguiの本体 = 入力欄へフォーカスを移した本体(&状態);
    let mut 集めた = 事象を渡して描く(
        &eguiの本体,
        &状態,
        vec![キーの押下(egui::Key::Space, 無し)],
        無し,
    );
    集めた.extend(事象を渡して描く(
        &eguiの本体,
        &状態,
        Vec::new(),
        無し,
    ));
    assert!(
        !集めた.contains(&応答::再生(再生の操作::再生を切り替える)),
        "{集めた:?}"
    );
}

#[test]
fn 入力欄の編集をエスケープでやめた回は全画面もシアターも抜けない() {
    let 状態 = クリップを並べた状態(Vec::new());
    let eguiの本体 = 入力欄へフォーカスを移した本体(&状態);
    let 集めた = 事象を渡して描く(
        &eguiの本体,
        &状態,
        vec![キーの押下(egui::Key::Escape, 無し)],
        無し,
    );
    assert!(
        !集めた.contains(&応答::出力(出力の操作::全画面かシアターを抜ける)),
        "{集めた:?}"
    );
    let _ = 事象を渡して描く(&eguiの本体, &状態, Vec::new(), 無し);
    let 次の回 = 事象を渡して描く(
        &eguiの本体,
        &状態,
        vec![キーの押下(egui::Key::Escape, 無し)],
        無し,
    );
    assert_eq!(次の回, [応答::出力(出力の操作::全画面かシアターを抜ける)]);
}
