//! ライブラリのファイル第1版の、ファイル上の形そのままの型。
//! フィールド名はファイル形式が決めている英語の名前であり、翻訳しない。第1版はこのアプリだけが書くため、欠けた項目を許さない。
//! 知らない項目も許さない(`deny_unknown_fields`)。項目を足した新しい版のファイルを読んで、自動保存で足した項目を消さないためである。
//! 項目を足すときは版を上げる。参照: _doc/設計/ライブラリ.md「形式の版を上げる手順」

use serde::{Deserialize, Serialize};

/// format に入る、ライブラリのファイルであることを示す名前。
pub(crate) const 形式の名前: &str = "ClipViewer.library";

/// 第1版の version に入る版の番号。
pub(crate) const 第1版の番号: u64 = 1;

/// 版の見出しとは、中身を読む前に確かめる format と version の2項目のことである。
#[derive(Deserialize)]
pub(crate) struct 版の見出し {
    pub format: Option<String>,
    pub version: Option<u64>,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct 第1版のファイル {
    pub format: String,
    pub version: u64,
    pub identifier: String,
    pub name: String,
    #[serde(rename = "videoPath")]
    pub video_path: String,
    #[serde(rename = "createdAtMs")]
    pub created_at_ms: u64,
    #[serde(rename = "updatedAtMs")]
    pub updated_at_ms: u64,
    pub clips: Vec<第1版のクリップ>,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct 第1版のクリップ {
    pub id: String,
    pub name: String,
    pub active: bool,
    pub start: f64,
    pub end: f64,
    pub repeat: 第1版の繰り返し,
    pub crop: 第1版のクロップ,
    pub trigger: 第1版のトリガー,
}

/// 第1版の繰り返しとは、repeat に入る「回数の整数」または「"infinite"」のことである。
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub(crate) enum 第1版の繰り返し {
    回数(u32),
    文字列(String),
}

/// 第1版の repeat で無限ループを表す表記。
pub(crate) const 無限ループの表記: &str = "infinite";

/// 第1版のクロップとは、crop に入る x・y・w・h(元の動画に対する百分率)の組のことである。
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct 第1版のクロップ {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub(crate) enum 第1版のトリガー {
    #[serde(rename = "none")]
    自動進行,
    #[serde(rename = "Enter")]
    Enterキー待ち,
    #[serde(rename = "Space")]
    Spaceキー待ち,
    #[serde(rename = "Click")]
    クリック待ち,
}
