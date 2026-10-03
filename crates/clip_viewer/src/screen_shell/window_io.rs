//! 画面の殻と egui のウインドウの境界の写し。egui が知らせたウインドウの様子(全画面・最大化・最小化・ウインドウの中身の大きさ・画面の大きさ)を配線の型へ写すことと、
//! 配線が決めたウインドウへの指示を egui のウインドウの命令へ写して送ることを持つ。参照: _doc/設計/画面.md 判断16・判断17

use eframe::egui;

use crate::viewer_settings::{ウインドウの大きさ, 画面の大きさ};
use crate::{app, state};

/// egui が知らせたウインドウの様子を読む。egui がまだ知らせていない項目は、全画面でない・最大化していない・大きさが分からないとみなす。
pub(super) fn ウインドウの様子を読む(
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
pub(super) fn 指示を送る(
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
