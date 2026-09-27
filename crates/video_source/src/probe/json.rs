//! ffprobe の JSON 出力を読み、動画の長さ・元の寸法・コマの速さ・映像の流れの番号・音の有無を取り出す純粋な計算。

use clip_domain::時間の長さ;

use super::error::{動画の情報の取得エラー, 読めない項目};
use super::json_shape::調査の出力;
use super::stream_index::{流れの番号, 音の有無};
use crate::frame_rate::コマの速さ;
use crate::frame_size::コマの寸法;

/// 調べた値とは、ffprobe の出力から取り出した、動画の長さ・表示される向きの寸法・コマの速さ・映像の流れの番号・音の有無の組のことである。
pub(crate) struct 調べた値 {
    pub(crate) 長さ: 時間の長さ,
    pub(crate) 元の寸法: コマの寸法,
    pub(crate) コマの速さ: コマの速さ,
    pub(crate) 映像の流れ: 流れの番号,
    pub(crate) 音の有無: 音の有無,
}

/// ffprobe の `-show_streams -show_format -of json` の出力を読む。添付の画像を除いた最初の映像の流れを使う。
pub(crate) fn 調査の出力を読む(
    本文: &str,
) -> Result<調べた値, 動画の情報の取得エラー> {
    let 出力: 調査の出力 = serde_json::from_str(本文).map_err(|原因| {
        動画の情報の取得エラー::出力を解釈できない {
            理由: 原因.to_string(),
        }
    })?;
    let 流れ = 出力
        .最初の映像()
        .ok_or(動画の情報の取得エラー::映像が無い)?;
    let 読めない = 動画の情報の取得エラー::項目を読めない;
    let 元の寸法 = 流れ.表示される寸法().ok_or(読めない(読めない項目::寸法))?;
    let コマの速さ = 流れ
        .コマの速さを選ぶ()
        .ok_or(読めない(読めない項目::コマの速さ))?;
    let 映像の流れ = 流れ
        .番号
        .map(流れの番号::作成する)
        .ok_or(読めない(読めない項目::流れの番号))?;
    let 長さ = 出力.長さ().ok_or(読めない(読めない項目::長さ))?;
    let 音の有無 = if 出力.音があるか() {
        音の有無::ある
    } else {
        音の有無::無い
    };
    Ok(調べた値 {
        長さ,
        元の寸法,
        コマの速さ,
        映像の流れ,
        音の有無,
    })
}
