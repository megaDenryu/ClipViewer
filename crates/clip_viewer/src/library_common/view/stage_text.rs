//! ヘッダーに出す保存の段階の文字と、一覧の行に出す更新日時の文。スタックと重ね合わせが共に使う。

use std::fmt;

use chrono::{DateTime, Local};
use clip_domain::ライブラリの日時;
use sengen_egui::{ノード, 文字表示};

use crate::library_common::save::保存の段階;
use crate::styles;

/// 開いている登録済みの保存物の保存の段階の文字。失敗だけを不備の装飾で出す。
pub(crate) fn 保存の段階の文字<応答: 'static, 操作エラー: fmt::Display + fmt::Debug>(
    段階: 保存の段階<'_, 操作エラー>,
) -> ノード<応答> {
    let (段階, 装飾) = match 段階 {
        保存の段階::保存済み => ("保存済み".to_string(), styles::補足),
        保存の段階::変更を保存待ち => ("変更を保存待ち".to_string(), styles::補足),
        保存の段階::保存中 => ("保存中".to_string(), styles::補足),
        保存の段階::保存に失敗(理由) => {
            (format!("保存に失敗: {理由}"), styles::不備の文)
        }
    };
    文字表示(段階).装飾(装飾).into()
}

/// この計算機の時間帯での「2026-09-26 14:05」の形。
pub(crate) fn 日時の表示(日時: ライブラリの日時) -> String {
    DateTime::<Local>::from(日時.時刻にする())
        .format("%Y-%m-%d %H:%M")
        .to_string()
}
