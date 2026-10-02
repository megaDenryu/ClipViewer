//! 重ね合わせが前の間の描き直しの予約の試験。重ね合わせが前のフレームはスタックの作業場の画面を組まないため、
//! スタックの作業場が待っている時刻(並びの保存・見る側の設定の保存)の描き直しを、重ね合わせの作業場の画面が予約することを確かめる。
//! 参照: _doc/設計/同時再生.md 3-2 の触れる所の表の描き直しの予約の行

use std::time::Duration;

use eframe::egui;

use super::workspace_test_support::手を加えたビューアー;
use super::クリップビューアー;

/// クリップビューアーの画面を2フレーム描き、2フレーム目が egui に頼んだ描き直しまでの時間を返す。
fn 描き直しまでの時間(ビューアー: &クリップビューアー) -> Duration {
    let 本体 = egui::Context::default();
    let mut 時間 = Duration::MAX;
    for _ in 0..2 {
        let 出力 = 本体.run(egui::RawInput::default(), |本体| {
            egui::CentralPanel::default().show(本体, |ui| {
                let _ = ビューアー.画面().描画して集める(ui);
            });
        });
        時間 = 出力
            .viewport_output
            .get(&egui::ViewportId::ROOT)
            .map_or(Duration::MAX, |出力| 出力.repaint_delay);
    }
    時間
}

#[test]
fn 重ね合わせが前の間も_スタックの作業場の保存を待つ時刻に描き直しを予約する() {
    let mut ビューアー = 手を加えたビューアー();
    ビューアー.重ね合わせの作業場へ移る();
    assert_eq!(描き直しまでの時間(&ビューアー), Duration::MAX);
    ビューアー.状態.見る側の設定を保存するまでの時間 = Some(Duration::from_millis(700));
    // egui は頼んだ時間から1フレーム分(約17ミリ秒)を引いて知らせるため、範囲で確かめる。
    let 時間 = 描き直しまでの時間(&ビューアー);
    assert!(
        Duration::from_millis(600) < 時間 && 時間 <= Duration::from_millis(700),
        "{時間:?}"
    );
}
