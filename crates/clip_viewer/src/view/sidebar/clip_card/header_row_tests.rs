//! カードの見出しの行の試験。クリップの始まりへ再生の位置を移すボタンの文字が、4行目の見出し「開始:」と紛れない「先頭へ」であることを確かめる。

use clip_domain::{クリップ, クリップ名, クリップ識別子};
use eframe::egui;

use super::見出しの行;

/// 見出しの行を1フレーム描き、描いた文字を全部集める。
fn 描いた文字() -> Vec<String> {
    let 識別子 = クリップ識別子::文字列から作成する("甲".to_string())
        .unwrap_or_else(|不正| panic!("識別子が不正: {不正}"));
    let クリップ =
        クリップ::既定値で作成する(識別子, クリップ名::作成する("甲".to_string()));
    let 画面描画の共有状態 = egui::Context::default();
    let 出力 = 画面描画の共有状態.run(egui::RawInput::default(), |画面描画の共有状態| {
        egui::CentralPanel::default().show(画面描画の共有状態, |ui| {
            let _ = 見出しの行(&クリップ, false).描画して集める(ui);
        });
    });
    出力
        .shapes
        .iter()
        .filter_map(|切り抜いた形| match &切り抜いた形.shape {
            egui::Shape::Text(文字の形) => Some(文字の形.galley.text().to_string()),
            _ => None,
        })
        .collect()
}

#[test]
fn 先頭へ移るボタンの文字は先頭へである() {
    let 文字 = 描いた文字();
    assert!(文字.iter().any(|文| 文 == "先頭へ"), "{文字:?}");
    assert!(!文字.iter().any(|文| 文 == "開始"), "{文字:?}");
}
