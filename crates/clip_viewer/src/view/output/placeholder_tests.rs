//! 動画の無いシアターの案内の試験。案内を1フレーム描き、「編集へ戻る」のボタンの文字と Escape の説明が画面に出ることと、
//! 動画の開き方の文が、シアターの構えでは見えない左のサイドバーを指さず、編集の構えでは今のままであることを確かめる。

use eframe::egui;

use super::placeholder::{シアターから編集へ戻る案内, 動画の開き方の文};
use crate::state::画面の構え;

/// 案内を1フレーム描き、描いた文字を全部集める。
fn 描いた文字() -> Vec<String> {
    let eguiの本体 = egui::Context::default();
    let 出力 = eguiの本体.run(egui::RawInput::default(), |eguiの本体| {
        egui::CentralPanel::default().show(eguiの本体, |ui| {
            let _ = シアターから編集へ戻る案内().描画して集める(ui);
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
fn 動画の無いシアターの案内は編集へ戻るのボタンとescapeの説明を出す() {
    let 文字 = 描いた文字();
    assert!(文字.iter().any(|文| 文 == "編集へ戻る"), "{文字:?}");
    assert!(文字.iter().any(|文| 文.contains("Escape キー")), "{文字:?}");
}

#[test]
fn シアターの構えの動画の開き方の文は見えている部品だけを指す() {
    assert_eq!(
        動画の開き方の文(画面の構え::シアター),
        "ウインドウへ動画をドラッグ＆ドロップすると開けます。"
    );
}

#[test]
fn 編集の構えの動画の開き方の文は左の動画を開くを指す() {
    assert_eq!(
        動画の開き方の文(画面の構え::編集),
        "左の「動画を開く」か、ウインドウへ動画をドラッグ＆ドロップして、シーケンス再生を開始します。"
    );
}
