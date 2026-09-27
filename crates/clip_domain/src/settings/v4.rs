//! 設定ファイル第4版(ブラウザ版 ClipViewer の v4.0 形式)の、ファイル上の形そのままの型。
//! フィールド名はファイル形式が決めている英語の名前であり、翻訳しない。
//! 読み込みで欠けてよい項目はすべて Option で受け、既定値の補完は最新への変換(v4_read.rs)が行う。

use serde::{Deserialize, Serialize};

/// 第4版の application に入る、このアプリの設定ファイルであることを示す名前。
pub(crate) const アプリケーション名: &str = "ModifierVideoStack";

/// 第4版の version に入る版の表記。
pub(crate) const 版の表記: &str = "4.0";

/// 版の見出しとは、中身を読む前に確かめる application と version の2項目のことである。
#[derive(Deserialize)]
pub(crate) struct 版の見出し {
    pub application: Option<String>,
    pub version: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct 第4版の設定ファイル {
    pub application: String,
    pub version: Option<String>,
    #[serde(rename = "exportDate")]
    pub export_date: Option<String>,
    #[serde(rename = "expectedVideoName")]
    pub expected_video_name: Option<String>,
    #[serde(rename = "expectedVideoPath")]
    pub expected_video_path: Option<String>,
    pub modifiers: Vec<第4版のクリップ>,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct 第4版のクリップ {
    pub id: Option<String>,
    pub name: Option<String>,
    pub active: Option<bool>,
    pub start: Option<f64>,
    pub end: Option<f64>,
    pub repeat: Option<第4版の繰り返し>,
    pub crop: Option<第4版のクロップ>,
    #[serde(rename = "triggerEvent")]
    pub trigger_event: Option<第4版のトリガー>,
}

/// 第4版の繰り返しとは、repeat に入る「回数の数値」または「"infinite"」のことである。
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub(crate) enum 第4版の繰り返し {
    回数(serde_json::Number),
    文字列(String),
}

/// 第4版の repeat で無限ループを表す表記。
pub(crate) const 無限ループの表記: &str = "infinite";

/// 第4版のクロップとは、crop に入る x・y・w・h の組のことである。
/// 移植元は欠けた項目にJavaScriptの既定引数(x=0, y=0, w=100, h=100)が効くため、各項目を Option で受ける。
#[derive(Serialize, Deserialize)]
pub(crate) struct 第4版のクロップ {
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub w: Option<f64>,
    pub h: Option<f64>,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub(crate) enum 第4版のトリガー {
    #[serde(rename = "none")]
    自動進行,
    #[serde(rename = "Enter")]
    Enterキー待ち,
    #[serde(rename = "Space")]
    Spaceキー待ち,
    #[serde(rename = "Click")]
    クリック待ち,
}
