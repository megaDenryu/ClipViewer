//! 画面の殻。eframe が毎フレーム呼び出す所(`eframe::App` の実装)であり、クリップビューアーの手順を順に呼び、
//! egui が知らせたウインドウの様子(全画面・最大化・大きさ・画面の大きさ)を配線へ渡し、配線が決めたウインドウへの指示の並びを egui のウインドウの命令へ写す。ウインドウを閉じる要求は、ライブラリの保存を確かめてから通す
//! (書けなければ閉じるのをやめて確かめるダイアログを出す。ウインドウへの指示の判断は `app/close.rs`。参照: _doc/設計/ライブラリ.md 判断5)。
//! 参照: _doc/設計/画面.md

use std::time::Instant;

use eframe::egui;

use crate::viewer_settings::{ウインドウの大きさ, 画面の大きさ};
use crate::{app, state};

/// 画面の殻とは、eframe が毎フレーム呼び出す所であり、クリップビューアーの手順を順に呼ぶだけのもののことである。
pub(crate) struct 画面の殻 {
    ビューアー: app::クリップビューアー,
}

impl 画面の殻 {
    pub(crate) fn 作成する(ビューアー: app::クリップビューアー) -> Self {
        Self { ビューアー }
    }
}

impl eframe::App for 画面の殻 {
    fn update(&mut self, 画面描画の共有状態: &egui::Context, _枠: &mut eframe::Frame) {
        // 注意: 時刻はフレームの手順の最初に1回だけ読み、閉じる要求の確かめ・フレームの手順・応答の適用へ同じ値を渡す(画面.md 判断7)。
        let 今 = Instant::now();
        let 閉じる要求への答え = 画面描画の共有状態
            .input(|入力| 入力.viewport().close_requested())
            .then(|| self.ビューアー.閉じる要求を確かめる(今));
        self.ビューアー
            .ウインドウの様子を知らせる(ウインドウの様子を読む(画面描画の共有状態));
        self.ビューアー.フレームを進める(今);
        let mut 応答一覧 = Vec::new();
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE)
            .show(画面描画の共有状態, |ui| {
                応答一覧 = self.ビューアー.画面().描画して集める(ui)
            });
        let 主ボタン = if 画面描画の共有状態.input(|入力| 入力.pointer.primary_down())
        {
            crate::primary_button::主ボタンの様子::押している
        } else {
            crate::primary_button::主ボタンの様子::押していない
        };
        let 届き方 = self.ビューアー.応答を適用する(応答一覧, 主ボタン, 今);
        for 指示 in self
            .ビューアー
            .ウインドウへの指示の並び(閉じる要求への答え, 届き方)
        {
            指示を送る(画面描画の共有状態, 指示);
        }
    }
}

/// egui が知らせたウインドウの様子を読む。egui がまだ知らせていない項目は、全画面でない・最大化していない・大きさが分からないとみなす。
fn ウインドウの様子を読む(
    画面描画の共有状態: &egui::Context,
) -> state::ウインドウの様子 {
    画面描画の共有状態.input(|入力| {
        let ウインドウ = 入力.viewport();
        let 全画面か = match ウインドウ.fullscreen {
            Some(true) => state::全画面の様子::全画面,
            Some(false) | None => state::全画面の様子::全画面でない,
        };
        let 形 = match (ウインドウ.minimized, ウインドウ.maximized) {
            (Some(true), _) => state::ウインドウの形::最小化している,
            (_, Some(true)) => state::ウインドウの形::最大化している,
            _ => state::ウインドウの形::普通(ウインドウ.inner_rect.and_then(|矩形| {
                ウインドウの大きさ::幅と高さから作る(
                    f64::from(矩形.width()),
                    f64::from(矩形.height()),
                )
            })),
        };
        let 画面 = ウインドウ.monitor_size.and_then(|大きさ| {
            画面の大きさ::幅と高さから作る(f64::from(大きさ.x), f64::from(大きさ.y))
        });
        state::ウインドウの様子 {
            全画面: 全画面か,
            形,
            画面,
        }
    })
}

/// 配線が決めたウインドウへの指示を、egui のウインドウの命令へ写して送る。
fn 指示を送る(
    画面描画の共有状態: &egui::Context, 指示: app::ウインドウへの指示
) {
    let 送る = |命令| 画面描画の共有状態.send_viewport_cmd(命令);
    match 指示 {
        app::ウインドウへの指示::閉じるのを取り消す => {
            送る(egui::ViewportCommand::CancelClose)
        }
        app::ウインドウへの指示::閉じる => 送る(egui::ViewportCommand::Close),
        app::ウインドウへの指示::前に出す => {
            送る(egui::ViewportCommand::Minimized(false));
            送る(egui::ViewportCommand::Focus);
        }
        app::ウインドウへの指示::全画面にする(様子) => 送る(
            egui::ViewportCommand::Fullscreen(様子 == state::全画面の様子::全画面),
        ),
        app::ウインドウへの指示::大きさを変える(大きさ) => 送る(
            egui::ViewportCommand::InnerSize(大きさ.論理画素の組().eguiへ渡す値()),
        ),
    }
}
