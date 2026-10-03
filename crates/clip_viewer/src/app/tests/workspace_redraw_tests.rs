//! 重ね合わせが前の間の描き直しの予約の試験。重ね合わせが前のフレームはスタックの作業場の画面を組まないため、
//! スタックの作業場が待っている時刻(並びの保存・見る側の設定の保存)の描き直しを、重ね合わせの作業場の画面が予約することを確かめる。
//! 参照: _doc/設計/同時再生.md 3-2 の触れる所の表の描き直しの予約の行

use std::time::Instant;

use std::time::Duration;

use eframe::egui;

use super::super::クリップビューアー;
use super::instruction_receiver_test_support::{控えへ開く, 渡した指示の控え};
use super::launch_requests_test_support::試験のビューアーを二本目の流れの手立てで作る;
use super::workspace_test_support::スタックの作業場の値を既定から変えて再生させる;

/// クリップビューアーの画面を2フレーム描き、2フレーム目が egui に頼んだ描き直しまでの時間を返す。
fn 画面を描いて描き直すまでの時間を読む(
    ビューアー: &クリップビューアー,
) -> Duration {
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
    // 注意: 2本目の流れを開けなかった知らせは、消える時刻に描き直しを予約する。この試験はスタックの保存待ちの予約を見るため、開ける流れで作る。
    let mut ビューアー = スタックの作業場の値を既定から変えて再生させる(
        試験のビューアーを二本目の流れの手立てで作る(Box::new(控えへ開く(
            渡した指示の控え::空(),
        ))),
    );
    ビューアー.重ね合わせの作業場へ移る(Instant::now());
    assert_eq!(
        画面を描いて描き直すまでの時間を読む(&ビューアー),
        Duration::MAX
    );
    ビューアー.状態.見る側の設定を保存するまでの時間 = Some(Duration::from_millis(700));
    // egui は頼んだ時間から1フレーム分(約17ミリ秒)を引いて知らせるため、範囲で確かめる。
    let 時間 = 画面を描いて描き直すまでの時間を読む(&ビューアー);
    assert!(
        Duration::from_millis(600) < 時間 && 時間 <= Duration::from_millis(700),
        "{時間:?}"
    );
}
