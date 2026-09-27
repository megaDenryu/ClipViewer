//! settings.json 第1版のウインドウの項目(windowSize と windowMaximized)の、ファイル上の形とウインドウの記憶との両方向の変換。
//! フィールド名はファイル形式が決めている英語の名前であり、翻訳しない。参照: _doc/設計/ライブラリ.md「settings.json の形式」

use serde::{Deserialize, Serialize};

use crate::viewer_settings::{
    ウインドウの大きさ, ウインドウの記憶, 最大化の様子
};

/// 第1版のウインドウの大きさとは、windowSize に入る、最大化していないときのウインドウの中身の幅と高さ(論理画素)のことである。
#[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
pub(super) struct 第1版のウインドウの大きさ {
    width: f64,
    height: f64,
}

impl 第1版のウインドウの大きさ {
    /// windowSize と windowMaximized をウインドウの記憶へ変換する。大きさが無いか正の数でなければ最初の大きさにする。
    pub(super) fn ウインドウの記憶へ変換する(
        大きさ: Option<Self>,
        最大化: Option<bool>,
    ) -> ウインドウの記憶 {
        ウインドウの記憶 {
            大きさ: 大きさ
                .and_then(|大きさ| {
                    ウインドウの大きさ::幅と高さから作る(
                        大きさ.width,
                        大きさ.height,
                    )
                })
                .unwrap_or(ウインドウの大きさ::最初),
            最大化: match 最大化 {
                Some(true) => 最大化の様子::最大化している,
                Some(false) | None => 最大化の様子::最大化していない,
            },
        }
    }

    /// ウインドウの記憶から、windowSize と windowMaximized の値を作る。
    pub(super) fn ウインドウの記憶から作る(
        記憶: ウインドウの記憶
    ) -> (Self, bool) {
        let (width, height) = 記憶.大きさ.幅と高さ();
        (
            Self { width, height },
            記憶.最大化 == 最大化の様子::最大化している,
        )
    }
}
