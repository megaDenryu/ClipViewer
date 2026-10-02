//! 画面の試験の道具。動画を開いていない起動したときの状態を作り、木を egui に描かせて、描いた文字の範囲を探し、押した応答を集める。
#![allow(clippy::expect_used)]

use eframe::egui;
use sengen_egui::ノード;
use video_source::FFmpegが見つからないエラー;

use crate::command::応答;
use crate::state::{
    FFmpegの状況, アプリの状態, 入力中のフォルダ, 画面の構え, 音の出力の状況
};
use crate::thumbnail_feed::一覧のサムネイル;

/// 動画を開いていない、起動したときの状態を、その構えにしたもの。
pub(super) fn 動画の無い状態(構え: 画面の構え) -> アプリの状態 {
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

/// 画面の大きさ(論理画素)を決めた入力に事象を足して1フレーム描き、発した応答と出力を返す。
pub(super) fn 大きさを決めて描く(
    eguiの本体: &egui::Context,
    画面の大きさ: egui::Vec2,
    事象: Vec<egui::Event>,
    木: &dyn Fn() -> ノード<応答>,
) -> (Vec<応答>, egui::FullOutput) {
    let 入力 = egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, 画面の大きさ)),
        events: 事象,
        ..Default::default()
    };
    let mut 集まり = Vec::new();
    let 出力 = eguiの本体.run(入力, |eguiの本体| {
        egui::CentralPanel::default().show(eguiの本体, |ui| {
            集まり = 木().描画して集める(ui);
        });
    });
    (集まり, 出力)
}

/// 出力の中で、その文字をそのまま描いた範囲。無ければ試験を失敗させる。
pub(super) fn 描いた文字の範囲(出力: &egui::FullOutput, 文字: &str) -> egui::Rect {
    出力
        .shapes
        .iter()
        .find_map(|切り抜いた形| match &切り抜いた形.shape {
            egui::Shape::Text(文字の形) if 文字の形.galley.text() == 文字 => {
                Some(文字の形.galley.rect.translate(文字の形.pos.to_vec2()))
            }
            _ => None,
        })
        .expect("その文字を描いていない")
}

/// 位置へポインタを動かして押して放し、その間に発した応答を返す。
pub(super) fn 押して集める(
    eguiの本体: &egui::Context,
    画面の大きさ: egui::Vec2,
    位置: egui::Pos2,
    木: &dyn Fn() -> ノード<応答>,
) -> Vec<応答> {
    let ボタン = |押した| egui::Event::PointerButton {
        pos: 位置,
        button: egui::PointerButton::Primary,
        pressed: 押した,
        modifiers: egui::Modifiers::NONE,
    };
    let 押す = vec![egui::Event::PointerMoved(位置), ボタン(true)];
    let mut 集まり = 大きさを決めて描く(eguiの本体, 画面の大きさ, 押す, 木).0;
    集まり.extend(大きさを決めて描く(eguiの本体, 画面の大きさ, vec![ボタン(false)], 木).0);
    集まり
}
