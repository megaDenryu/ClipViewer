//! 作業場を移すことの試験の道具のうち、画面の殻と同じ順にクリップビューアーの1フレームを進めること。キーを押した入力で画面を描き、発した応答を適用する。

use std::time::Instant;

use eframe::egui;

use super::クリップビューアー;
use crate::command::主ボタンの様子;

/// 新しい egui の本体で、何も押さずに1フレーム進めてから、キーを1つ押した入力で1フレーム進める。
pub(super) fn キーを押して適用する(
    ビューアー: &mut クリップビューアー,
    キー: egui::Key,
    修飾キー: egui::Modifiers,
) {
    let 本体 = egui::Context::default();
    一フレーム進める(ビューアー, &本体, Vec::new(), 修飾キー);
    一フレーム進める(
        ビューアー,
        &本体,
        vec![キーの押下(キー, 修飾キー)],
        修飾キー,
    );
}

/// 何も押さずに1フレーム進める。毎フレームの手順が状態へ当てる値(操作の区切り等)を、キーを押す前にそろえるために使う。
pub(super) fn 何も押さずに一フレーム進める(
    ビューアー: &mut クリップビューアー
) {
    一フレーム進める(
        ビューアー,
        &egui::Context::default(),
        Vec::new(),
        egui::Modifiers::NONE,
    );
}

/// 画面の殻と同じ順に1フレームを進める。状態を進め、前に出ている作業場の画面を事象を入力にして描き、発した応答を適用する。
fn 一フレーム進める(
    ビューアー: &mut クリップビューアー,
    本体: &egui::Context,
    事象: Vec<egui::Event>,
    修飾キー: egui::Modifiers,
) {
    ビューアー.フレームを進める(Instant::now());
    let 入力 = egui::RawInput {
        events: 事象,
        modifiers: 修飾キー,
        ..Default::default()
    };
    let mut 集まり = Vec::new();
    let _ = 本体.run(入力, |本体| {
        egui::CentralPanel::default().show(本体, |ui| {
            集まり = ビューアー.画面().描画して集める(ui);
        });
    });
    let _ = ビューアー.応答を適用する(集まり, 主ボタンの様子::押していない);
}

fn キーの押下(キー: egui::Key, 修飾キー: egui::Modifiers) -> egui::Event {
    egui::Event::Key {
        key: キー,
        physical_key: None,
        pressed: true,
        repeat: false,
        modifiers: 修飾キー,
    }
}
