//! 重ね合わせのファイル第1版の、ファイル上の形そのままの型。
//! フィールド名はファイル形式が決めている英語の名前であり、翻訳しない。第1版はこのアプリだけが書くため、欠けた項目を許さない。
//! 知らない項目も許さない(`deny_unknown_fields`)。項目を足した新しい版のファイルを読んで、自動保存で足した項目を消さないためである。
//! 項目を足すときは版を上げる。参照: _doc/設計/同時再生.md 4-2、_doc/設計/ライブラリ.md「形式の版を上げる手順」

use serde::{Deserialize, Serialize};

/// format に入る、重ね合わせのファイルであることを示す名前。
pub(crate) const 重ね合わせの形式の名前: &str = "ClipViewer.overlay";

/// 重ね合わせのファイルの第1版の version に入る版の番号。
pub(crate) const 重ね合わせの第1版の番号: u64 = 1;

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct 重ね合わせの第1版のファイル {
    pub format: String,
    pub version: u64,
    pub identifier: String,
    pub name: String,
    #[serde(rename = "createdAtMs")]
    pub created_at_ms: u64,
    #[serde(rename = "updatedAtMs")]
    pub updated_at_ms: u64,
    pub aspect: 重ね合わせの第1版の縦横比,
    pub videos: Vec<String>,
    pub rows: Vec<重ね合わせの第1版の行>,
}

/// 重ね合わせの第1版の行とは、rows の1つの要素であり、奥から手前の順の1つの行に置いた置いたクリップの並び(items)のことである。
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct 重ね合わせの第1版の行 {
    pub items: Vec<重ね合わせの第1版の置いたクリップ>,
}

/// 重ね合わせの第1版の置いたクリップとは、items の1つの要素のことである。video は使う動画の表の番号、at は重ね合わせ上の始まりの秒である。
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct 重ね合わせの第1版の置いたクリップ {
    pub id: String,
    pub name: String,
    pub video: usize,
    pub start: f64,
    pub end: f64,
    pub repeat: u32,
    pub crop: 重ね合わせの第1版の矩形,
    pub at: f64,
    pub rect: 重ね合わせの第1版の矩形,
    pub volume: f64,
    pub muted: bool,
}

/// 重ね合わせの第1版の矩形とは、crop(元の動画に対する百分率)と rect(重ねる画面に対する百分率)に入る x・y・w・h の組のことである。
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct 重ね合わせの第1版の矩形 {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}

/// 重ね合わせの第1版の縦横比とは、aspect に入る表記(settings.json の aspectRatio と同じ)のことである。crop は読めるが、重ね合わせでは拒む。
#[derive(Serialize, Deserialize, Clone, Copy)]
pub(crate) enum 重ね合わせの第1版の縦横比 {
    #[serde(rename = "16:9")]
    ワイド16対9,
    #[serde(rename = "4:3")]
    スタンダード4対3,
    #[serde(rename = "1:1")]
    正方形,
    #[serde(rename = "9:16")]
    縦型9対16,
    #[serde(rename = "crop")]
    クロップ枠に合わせる,
}
