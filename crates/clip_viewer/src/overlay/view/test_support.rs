//! 重ね合わせの作業場の画面の試験の道具。木を egui に描かせて発した応答と出力を返すことと、描いた文字の範囲と、テクスチャを貼った形を探すことと、
//! 位置を押して応答を集めることを受け持つ。スタックの作業場の同じ道具(`view/test_support.rs`)は、重ね合わせの作業場の層から使えないため別に持つ。
#![allow(clippy::expect_used)]

use eframe::egui;
use sengen_egui::ノード;

use crate::overlay::command::重ね合わせの作業場の応答;

pub(super) const 画面の大きさ: egui::Vec2 = egui::vec2(1200.0, 700.0);

/// 画面の大きさを決めた入力に事象を足して1フレーム描き、発した応答と出力を返す。
pub(super) fn 描いて集める(
    画面描画の共有状態: &egui::Context,
    事象: Vec<egui::Event>,
    木: &dyn Fn() -> ノード<重ね合わせの作業場の応答>,
) -> (Vec<重ね合わせの作業場の応答>, egui::FullOutput) {
    let 入力 = egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, 画面の大きさ)),
        events: 事象,
        ..Default::default()
    };
    let mut 集まり = Vec::new();
    let 出力 = 画面描画の共有状態.run(入力, |画面描画の共有状態| {
        egui::CentralPanel::default().show(画面描画の共有状態, |ui| {
            集まり = 木().描画して集める(ui);
        });
    });
    (集まり, 出力)
}

/// 出力の中で、その文字をそのまま描いた範囲。描いていなければ無い。
pub(super) fn 描いた文字の範囲(
    出力: &egui::FullOutput, 文字: &str
) -> Option<egui::Rect> {
    出力
        .shapes
        .iter()
        .find_map(|切り抜いた形| match &切り抜いた形.shape {
            egui::Shape::Text(文字の形) if 文字の形.galley.text() == 文字 => {
                Some(文字の形.galley.rect.translate(文字の形.pos.to_vec2()))
            }
            _ => None,
        })
}

/// 出力の中で、黒で塗った最初の矩形(重ねる画面の地)。描いていなければ無い。
pub(super) fn 黒く塗った矩形(出力: &egui::FullOutput) -> Option<egui::Rect> {
    出力
        .shapes
        .iter()
        .find_map(|切り抜いた形| match &切り抜いた形.shape {
            egui::Shape::Rect(四角) if 四角.fill == egui::Color32::BLACK => Some(四角.rect),
            _ => None,
        })
}

/// テクスチャを貼った形(フォントの字の形を除く)の、描いた順の矩形と、テクスチャのうち描いた範囲(uv)。
pub(super) fn テクスチャを貼った形(
    出力: &egui::FullOutput,
) -> Vec<(egui::Rect, egui::Rect)> {
    出力
        .shapes
        .iter()
        .filter_map(|切り抜いた形| match &切り抜いた形.shape {
            egui::Shape::Rect(四角) => 四角.brush.as_ref().map(|塗り方| (四角.rect, 塗り方.uv)),
            _ => None,
        })
        .collect()
}

/// 位置へポインタを動かして押して放し、その間に発した応答を返す。
pub(super) fn 押して集める(
    画面描画の共有状態: &egui::Context,
    位置: egui::Pos2,
    木: &dyn Fn() -> ノード<重ね合わせの作業場の応答>,
) -> Vec<重ね合わせの作業場の応答> {
    let ボタン = |押した| egui::Event::PointerButton {
        pos: 位置,
        button: egui::PointerButton::Primary,
        pressed: 押した,
        modifiers: egui::Modifiers::NONE,
    };
    let 押す = vec![egui::Event::PointerMoved(位置), ボタン(true)];
    let mut 集まり = 描いて集める(画面描画の共有状態, 押す, 木).0;
    集まり.extend(描いて集める(画面描画の共有状態, vec![ボタン(false)], 木).0);
    集まり
}
