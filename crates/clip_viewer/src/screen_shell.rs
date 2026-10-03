//! 画面の殻。eframe が毎フレーム呼び出す所(`eframe::App` の実装)であり、クリップビューアーの手順を順に呼び、
//! egui が知らせたウインドウの様子(全画面・最大化・大きさ・画面の大きさ)を配線へ渡し、配線が決めたウインドウへの指示の並びを egui のウインドウの命令へ写す。ウインドウを閉じる要求は、ライブラリの保存を確かめてから通す
//! (書けなければ閉じるのをやめて確かめるダイアログを出す。ウインドウへの指示の判断は `app/close.rs`。参照: _doc/設計/ライブラリ.md 判断5)。
//! `cargo xtask frame-time` が起動したときだけ、フレームの時間の計測(`app/frame_time_measure.rs`)を持ち、毎フレームの最後に eframe が知らせた1フレームの時間を渡す。
//! egui のウインドウの様子と命令への写しは `screen_shell/window_io.rs` に置く。参照: _doc/設計/画面.md、同時再生.md 5-4

use std::time::Instant;

use eframe::egui;

mod window_io;

use window_io::{ウインドウの様子を読む, 指示を送る};

use crate::app;

/// 画面の殻とは、eframe が毎フレーム呼び出す所であり、クリップビューアーの手順を順に呼ぶだけのもののことである。
/// フレームの時間の計測は、`cargo xtask frame-time` が起動したときだけある。
pub(crate) struct 画面の殻 {
    ビューアー: app::クリップビューアー,
    計測: Option<app::フレームの時間の計測>,
}

impl 画面の殻 {
    /// 計測の頼みがあれば(`cargo xtask frame-time` が起動したとき)、フレームの時間の計測を持つ。
    pub(crate) fn 作成する(
        ビューアー: app::クリップビューアー,
        計測の頼み: Option<crate::frame_time::フレームの時間を測る頼み>,
    ) -> Self {
        let 計測 = 計測の頼み.map(app::フレームの時間の計測::頼みから始める);
        Self {
            ビューアー, 計測
        }
    }
}

impl eframe::App for 画面の殻 {
    fn update(&mut self, 画面描画の共有状態: &egui::Context, 枠: &mut eframe::Frame) {
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
        if let Some(計測) = &mut self.計測
            && 計測.フレームを数える(&mut self.ビューアー, 今, 枠.info().cpu_usage)
                == app::計測の続き::ウインドウを閉じる
        {
            指示を送る(画面描画の共有状態, app::ウインドウへの指示::閉じる);
        }
    }
}
