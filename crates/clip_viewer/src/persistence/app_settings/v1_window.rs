//! settings.json 第1版の窓の項目(windowSize と windowMaximized)の、ファイル上の形と窓の記憶との両方向の変換。
//! フィールド名はファイル形式が決めている英語の名前であり、翻訳しない。参照: _doc/設計/ライブラリ.md「settings.json の形式」

use serde::{Deserialize, Serialize};

use crate::viewer_settings::{最大化の様子, 窓の大きさ, 窓の記憶};

/// 第1版の窓の大きさとは、windowSize に入る、最大化していないときの窓の中身の幅と高さ(論理画素)のことである。
#[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
pub(super) struct 第1版の窓の大きさ {
    width: f64,
    height: f64,
}

impl 第1版の窓の大きさ {
    /// windowSize と windowMaximized を窓の記憶へ変換する。大きさが無いか正の数でなければ最初の大きさにする。
    pub(super) fn 窓の記憶へ変換する(
        大きさ: Option<Self>,
        最大化: Option<bool>,
    ) -> 窓の記憶 {
        窓の記憶 {
            大きさ: 大きさ
                .and_then(|大きさ| {
                    窓の大きさ::幅と高さから作る(大きさ.width, 大きさ.height)
                })
                .unwrap_or(窓の大きさ::最初),
            最大化: match 最大化 {
                Some(true) => 最大化の様子::最大化している,
                Some(false) | None => 最大化の様子::最大化していない,
            },
        }
    }

    /// 窓の記憶から、windowSize と windowMaximized の値を作る。
    pub(super) fn 窓の記憶から作る(記憶: 窓の記憶) -> (Self, bool) {
        let (width, height) = 記憶.大きさ.幅と高さ();
        (
            Self { width, height },
            記憶.最大化 == 最大化の様子::最大化している,
        )
    }
}
