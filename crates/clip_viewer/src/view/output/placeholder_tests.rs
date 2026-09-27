//! 動画の無いシアターの案内の試験。案内を1フレーム描き、「編集へ戻る」のボタンの文字と Escape の説明が画面に出ることと、
//! 動画の開き方の文が、シアターの構えでは見えない左のサイドバーを指さず、編集の構えでは今のままであることを確かめる。
//! 動画の無い案内の全体をシアターの構えで描き、「左の「動画を開く」か」を含む文が出ないことも確かめる。

use eframe::egui;
use sengen_egui::ノード;
use video_source::FFmpegが見つからないエラー;

use super::placeholder::{
    シアターから編集へ戻る案内, 動画の無い案内, 動画の開き方の文
};
use crate::command::応答;
use crate::state::{
    FFmpegの状況, アプリの状態, 入力中のフォルダ, 画面の構え, 音の出力の状況
};
use crate::thumbnail_feed::一覧のサムネイル;

/// 動画を開いていない、起動したときの状態。
fn 動画の無い状態(構え: 画面の構え) -> アプリの状態 {
    let ffmpegの状況 = FFmpegの状況::見つからない {
        理由: FFmpegが見つからないエラー {
            見つからない道具: Vec::new(),
            探した場所: Vec::new(),
        },
        入力中のフォルダ: 入力中のフォルダ::default(),
    };
    let 音の出力 = 音の出力の状況::使えない(audio_output::音声出力のエラー::装置が無い);
    let サムネイル =
        一覧のサムネイル::キャッシュを使わずに作る(egui::Context::default());
    let mut 状態 = アプリの状態::起動時(ffmpegの状況, 音の出力, サムネイル);
    状態.出力.構え = 構え;
    状態
}

/// 木を1フレーム描き、描いた文字を全部集める。
fn 描いた文字(木: &dyn Fn() -> ノード<応答>) -> Vec<String> {
    let eguiの本体 = egui::Context::default();
    let 出力 = eguiの本体.run(egui::RawInput::default(), |eguiの本体| {
        egui::CentralPanel::default().show(eguiの本体, |ui| {
            let _ = 木().描画して集める(ui);
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
    let 文字 = 描いた文字(&シアターから編集へ戻る案内);
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

#[test]
fn シアターの構えで描いた動画の無い案内は左の動画を開くを指さない() {
    let 状態 = 動画の無い状態(画面の構え::シアター);
    let 文字 = 描いた文字(&|| 動画の無い案内(&状態));
    assert!(
        !文字.iter().any(|文| 文.contains("左の「動画を開く」か")),
        "{文字:?}"
    );
    assert!(
        文字
            .iter()
            .any(|文| 文 == "ウインドウへ動画をドラッグ＆ドロップすると開けます。"),
        "{文字:?}"
    );
}
