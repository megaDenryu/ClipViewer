//! キーの割り当ての試験の道具。egui にキーの押下を渡して画面を1フレーム描き、発した応答を集める。
#![allow(clippy::expect_used)]

use eframe::egui;

use crate::command::応答;
use crate::state::アプリの状態;
use crate::view;

/// 渡した事象を入力にして画面を1フレーム描き、発した応答を返す。同じ egui の本体で続けて描くと、フォーカスがフレームをまたいで残る。
pub(super) fn 事象を渡して描く(
    画面描画の共有状態: &egui::Context,
    状態: &アプリの状態,
    事象: Vec<egui::Event>,
    修飾キー: egui::Modifiers,
) -> Vec<応答> {
    let 入力 = egui::RawInput {
        events: 事象,
        modifiers: 修飾キー,
        ..Default::default()
    };
    let mut 集まり = Vec::new();
    let _ = 画面描画の共有状態.run(入力, |画面描画の共有状態| {
        egui::CentralPanel::default().show(画面描画の共有状態, |ui| {
            集まり = view::画面(状態, None).描画して集める(ui);
        });
    });
    集まり
}

/// キーを押した事象。
pub(super) fn キーの押下(キー: egui::Key, 修飾キー: egui::Modifiers) -> egui::Event {
    egui::Event::Key {
        key: キー,
        physical_key: None,
        pressed: true,
        repeat: false,
        modifiers: 修飾キー,
    }
}

/// 新しい egui の本体に何も押さずに1フレーム描いてから、キーを1つ押した入力で1フレーム描き、発した応答を返す。
/// 先に1フレーム描くのは、egui のダイアログ(モーダル)が一番上のダイアログとして Escape を受け取るのが、描いた次の回からだからである。
pub(super) fn キーを押して集める(
    状態: &アプリの状態,
    キー: egui::Key,
    修飾キー: egui::Modifiers,
) -> Vec<応答> {
    let 画面描画の共有状態 = egui::Context::default();
    let _ = 事象を渡して描く(&画面描画の共有状態, 状態, Vec::new(), egui::Modifiers::NONE);
    事象を渡して描く(
        &画面描画の共有状態,
        状態,
        vec![キーの押下(キー, 修飾キー)],
        修飾キー,
    )
}
