//! 出力の画面の左右の向きの試験。左右を反転すると、編集の構えの出力でもシアターでも、映像の uv の左端と右端が入れ替わる。
//! 描く部分(クロップ)を指定しているときは、その部分の中で入れ替わる。

use eframe::egui;
use sengen_egui::画素の組;

use super::screen_test_support::試験の材料の選び方;
use crate::state::{左右の向き, 画面の構え};

/// 左端0.25・右端0.5の描く部分で、構えと左右の向きの材料を組んで1フレーム描き、映像の uv を返す。
fn 描いた映像のuv(構え: 画面の構え, 左右: 左右の向き) -> egui::Rect {
    let 画面描画の共有状態 = egui::Context::default();
    let 材料 = 試験の材料の選び方 {
        構え,
        左右,
        待ちの案内: None,
    }
    .材料を作る(&画面描画の共有状態);
    let 出力 = 画面描画の共有状態.run(egui::RawInput::default(), |画面描画の共有状態| {
        egui::CentralPanel::default().show(画面描画の共有状態, |ui| {
            let _ = 材料.組む(画素の組(800.0, 600.0)).描画して集める(ui);
        });
    });
    出力
        .shapes
        .iter()
        .find_map(|切り抜いた形| match &切り抜いた形.shape {
            egui::Shape::Rect(矩形) => 矩形.brush.as_ref().map(|塗り方| 塗り方.uv),
            _ => None,
        })
        .unwrap_or_else(|| panic!("映像を貼った矩形が無い"))
}

#[test]
fn 左右を反転すると編集の構えでもシアターでも描く部分の中で左右を入れ替える() {
    for 構え in [画面の構え::編集, 画面の構え::シアター] {
        let そのまま = 描いた映像のuv(構え, 左右の向き::そのまま);
        assert_eq!((そのまま.min.x, そのまま.max.x), (0.25, 0.5), "{構え:?}");
        let 反転 = 描いた映像のuv(構え, 左右の向き::反転);
        assert_eq!((反転.min.x, 反転.max.x), (0.5, 0.25), "{構え:?}");
    }
}
