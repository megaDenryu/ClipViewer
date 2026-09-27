//! 書き出す設定から第4版の設定ファイルへの変換。書き出しは常に第4版の形で行う。
//! ブラウザ版と双方向で読み書きするため、書き出す形は移植元 `スタック設定をエクスポートする` と同じ項目・同じ順である。

use super::v4::{
    アプリケーション名, 無限ループの表記, 版の表記, 第4版のクリップ, 第4版のクロップ,
    第4版のトリガー, 第4版の繰り返し, 第4版の設定ファイル,
};
use super::{書き出し日時, 書き出す設定};
use crate::clip::クリップ;
use crate::repeat::リピート設定;

impl 第4版の設定ファイル {
    /// 書き出す設定から作る。期待する動画名が無ければ空文字列を書く(移植元と同じ)。
    pub(crate) fn 最新から作る(
        設定: &書き出す設定<'_>, 日時: &書き出し日時
    ) -> Self {
        let 期待する動画の名前 = 設定
            .動画のパス
            .期待する動画()
            .map_or_else(String::new, |名前| 名前.文字列().to_string());
        Self {
            application: アプリケーション名.to_string(),
            version: Some(版の表記.to_string()),
            export_date: Some(日時.文字列().to_string()),
            expected_video_name: Some(期待する動画の名前),
            expected_video_path: Some(設定.動画のパス.正規化したパス().文字列().to_string()),
            modifiers: 設定
                .スタック
                .スタック()
                .一覧()
                .iter()
                .map(第4版のクリップ::最新から作る)
                .collect(),
        }
    }
}

impl 第4版のクリップ {
    fn 最新から作る(クリップ: &クリップ) -> Self {
        Self {
            id: Some(クリップ.識別子().文字列().to_string()),
            name: Some(クリップ.名前.文字列().to_string()),
            active: Some(クリップ.有効か),
            start: Some(クリップ.区間.開始().秒数()),
            end: Some(クリップ.区間.終了().秒数()),
            repeat: Some(match クリップ.リピート {
                リピート設定::回数指定(回数) => {
                    第4版の繰り返し::回数(serde_json::Number::from(回数.回数()))
                }
                リピート設定::無限ループ => {
                    第4版の繰り返し::文字列(無限ループの表記.to_string())
                }
            }),
            crop: Some(第4版のクロップ {
                x: Some(クリップ.クロップ.左端().数値()),
                y: Some(クリップ.クロップ.上端().数値()),
                w: Some(クリップ.クロップ.幅().数値()),
                h: Some(クリップ.クロップ.高さ().数値()),
            }),
            trigger_event: Some(第4版のトリガー::from(クリップ.トリガー)),
        }
    }
}
