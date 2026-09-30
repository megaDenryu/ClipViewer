//! ClipViewer の起動の部分。落ちたときの記録を取り付け、起動の手順を決め、ウインドウを作り、テーマと日本語フォントを適用し、
//! 毎フレームの手順を画面の殻(`screen_shell.rs`)として eframe へ渡す。素の eframe と egui を呼ぶのは、本ファイルと画面の殻だけである。
//! 参照: _doc/設計/画面.md
//! 起動: リポジトリのルートで `cargo xtask run`

#![forbid(unsafe_code)]
// release のビルドではコンソールのウインドウを出さない。開発のビルド(cargo xtask run)では標準エラーを見るため出す。
// 起動の途中の失敗は、標準エラーの代わりに startup_notice の口で OS のメッセージボックスへ出す。
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod audio_feed;
mod command;
mod crash_record;
mod launch;
mod persistence;
mod screen_shell;
mod startup_notice;
mod state;
mod stream_rules;
mod thumbnail_feed;
mod video_feed;
mod view;
mod viewer_settings;

#[cfg(test)]
mod main_tests;

use std::process::ExitCode;

use eframe::egui;
use sengen_egui::日本語フォントの候補;

/// OS に入っている日本語フォントを読み込んで足す。どれも読めなければ試した置き場所を利用者へ見せ、起動は続ける。
/// 日本語の字形が出ないだけで操作はできるため、起動を止めるより画面を出す方を選ぶ。
fn 日本語フォントを設定する(eguiの本体: &egui::Context) {
    if let Err(失敗) = 日本語フォントの候補::標準で入っている候補()
        .最初に読めたものを設定する(eguiの本体)
    {
        startup_notice::画面の外で見せる知らせ::作成する(format!(
            "{失敗}。日本語が表示されない場合は OS へ日本語フォントを導入する"
        ))
        .利用者へ見せる();
    }
}

/// ウインドウのアイコンにする画像(256×256 の PNG)。題名の帯と Alt+Tab に出す。実行ファイルのアイコンは build.rs が埋め込む。
const ウインドウのアイコンの画像: &[u8] = include_bytes!("../../../assets/icon/ClipViewer-256.png");

/// ウインドウのアイコンを画素へ直す。埋め込んだ画像を読めなければ(ビルドの誤りであり、試験で確かめている)警告の記録へ書き、アイコンなしで起動する。
fn ウインドウのアイコン() -> Option<egui::IconData> {
    eframe::icon_data::from_png_bytes(ウインドウのアイコンの画像)
        .inspect_err(|原因| log::warn!("ウインドウのアイコンの画像を読めない: {原因}"))
        .ok()
}

fn main() -> ExitCode {
    // 起動の引数は、エクスプローラーの「このアプリで開く」とダブルクリックが渡す動画のパスである(参照: _doc/設計/画面.md 判断13)。
    let 環境 = app::起動時の環境::今のプロセスから読む();
    let 記録の置き場所 = crash_record::記録の置き場所::ローカルのアプリのデータのフォルダから決める(
        環境.ローカルのアプリのデータのフォルダ.clone(),
    );
    記録の置き場所.落ちたときの記録を取り付ける();
    let 準備 = app::起動の手順::作成する(環境).決める();
    // 警告の記録(warnings.log を移して書き始める)は、ライブラリの錠を持つアプリだけが取り付ける。読み取り専用で起動した
    // 2つ目のアプリが、1つ目の書いている warnings.log を前回の記録へ移さないためである。読み取り専用のアプリの警告は残さない。
    if 準備.ライブラリの錠を持つか() {
        記録の置き場所.警告の記録を取り付ける();
    }
    let ウインドウ = 準備.ウインドウの記憶();
    let ウインドウの作り方 = match ウインドウのアイコン() {
        Some(アイコン) => egui::ViewportBuilder::default().with_icon(アイコン),
        None => egui::ViewportBuilder::default(),
    };
    let 選択肢 = eframe::NativeOptions {
        viewport: ウインドウの作り方
            .with_inner_size(ウインドウ.大きさ.論理画素の組().eguiへ渡す値())
            .with_min_inner_size(
                viewer_settings::ウインドウの大きさ::最小
                    .論理画素の組()
                    .eguiへ渡す値(),
            )
            .with_maximized(ウインドウ.最大化 == viewer_settings::最大化の様子::最大化している)
            .with_title(app::ウインドウの題名),
        centered: true,
        ..Default::default()
    };
    let 結果 = eframe::run_native(
        "ClipViewer",
        選択肢,
        Box::new(|ウインドウを作るときの情報| {
            view::styles::画面のテーマ.適用する(&ウインドウを作るときの情報.egui_ctx);
            日本語フォントを設定する(&ウインドウを作るときの情報.egui_ctx);
            let ビューアー = app::クリップビューアー::組み立てる(
                ウインドウを作るときの情報.egui_ctx.clone(),
                準備,
            );
            Ok(Box::new(screen_shell::画面の殻::作成する(
                ビューアー,
            )))
        }),
    );
    match 結果 {
        Ok(()) => ExitCode::SUCCESS,
        Err(原因) => {
            startup_notice::画面の外で見せる知らせ::作成する(format!(
                "ClipViewer の起動に失敗した: {原因}"
            ))
            .利用者へ見せる();
            ExitCode::FAILURE
        }
    }
}
