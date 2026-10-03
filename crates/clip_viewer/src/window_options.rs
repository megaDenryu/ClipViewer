//! ウインドウの作り方。settings.json が覚えているウインドウの大きさと最大化と、最小の大きさと、版を添えた題名と、埋め込んだアイコンから、
//! eframe へ渡すウインドウの選択肢を組み立てる。起動の部分(`main.rs`)が、ウインドウを作る前に1回だけ呼ぶ。参照: _doc/設計/画面.md 2節・判断17

use eframe::egui;

use crate::app;
use crate::viewer_settings::{
    ウインドウの大きさ, ウインドウの記憶, 最大化の様子
};

/// ウインドウのアイコンにする画像(256×256 の PNG)。題名の帯と Alt+Tab に出す。実行ファイルのアイコンは build.rs が埋め込む。
const ウインドウのアイコンの画像: &[u8] = include_bytes!("../../../assets/icon/ClipViewer-256.png");

/// ウインドウのアイコンを画素へ直す。埋め込んだ画像を読めなければ(ビルドの誤りであり、試験で確かめている)警告の記録へ書き、アイコンなしで起動する。
pub(crate) fn ウインドウのアイコン() -> Option<egui::IconData> {
    eframe::icon_data::from_png_bytes(ウインドウのアイコンの画像)
        .inspect_err(|原因| log::warn!("ウインドウのアイコンの画像を読めない: {原因}"))
        .ok()
}

/// 覚えているウインドウの大きさと最大化で、画面の中央に作るウインドウの選択肢。
pub(crate) fn ウインドウの選択肢(
    ウインドウ: ウインドウの記憶
) -> eframe::NativeOptions {
    let ウインドウの作り方 = match ウインドウのアイコン() {
        Some(アイコン) => egui::ViewportBuilder::default().with_icon(アイコン),
        None => egui::ViewportBuilder::default(),
    };
    eframe::NativeOptions {
        viewport: ウインドウの作り方
            .with_inner_size(ウインドウ.大きさ.論理画素の組().eguiへ渡す値())
            .with_min_inner_size(ウインドウの大きさ::最小.論理画素の組().eguiへ渡す値())
            .with_maximized(ウインドウ.最大化 == 最大化の様子::最大化している)
            .with_title(app::ウインドウの題名),
        centered: true,
        ..Default::default()
    }
}
