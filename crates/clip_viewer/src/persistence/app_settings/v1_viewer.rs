//! settings.json 第1版の見る側の設定の項目と、最新との両方向の変換。フィールド名はファイル形式が決めている英語の名前であり、翻訳しない。
//! どの項目も値が無いときは書かない(`skip_serializing_if`)。第1版の見本(見る側の項目を持たない)の書き出しを変えないためである。
//! キーの割り当て(keyBindings)は、見る側の設定を覚えていても、既定と違う操作が無ければ書かない(v1_keys.rs)。
//! 文字列の項目(aspectRatio・displaySize)に知らない値があれば、その項目は覚えていないとみなす(新しいアプリが同じ版に値を足しても、ファイルを壊れたとみなさない)。
//! 参照: _doc/設計/ライブラリ.md「settings.json の形式」

use audio_pcm::音量;
use serde::{Deserialize, Serialize};

use super::settings::覚えた見る側の設定;
use super::v1_keys::第1版のキーの割り当て;
use super::v1_notation::{
    アスペクト比の表記, 表示サイズの表記, 表記から読む, 表記にする
};
use super::v1_window::第1版のウインドウの大きさ;
use crate::viewer_settings::{
    つまみの範囲へ収めた速度, 左右の向き, 見る側の設定
};

/// 第1版の見る側の設定とは、settings.json 第1版のうち見る側の設定の項目の組のことである。第1版の設定へ平らに入れる(flatten)。
#[derive(Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(super) struct 第1版の見る側の設定 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    window_size: Option<第1版のウインドウの大きさ>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    window_maximized: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    volume: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    playback_speed: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    mirror_horizontally: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    aspect_ratio: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    display_size: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    key_bindings: Option<第1版のキーの割り当て>,
}

impl 第1版の見る側の設定 {
    /// 最新の見る側の設定へ変換する。どの項目も無ければ覚えていない。一部の項目だけあれば、無い項目と読めない値を既定で埋める。
    pub(super) fn 見る側の設定へ変換する(self) -> 覚えた見る側の設定 {
        if self == Self::default() {
            return 覚えた見る側の設定::覚えていない;
        }
        let Self {
            window_size,
            window_maximized,
            volume,
            playback_speed,
            mirror_horizontally,
            aspect_ratio,
            display_size,
            key_bindings,
        } = self;
        let 既定 = 見る側の設定::既定;
        覚えた見る側の設定::覚えている(見る側の設定 {
            ウインドウ:
                第1版のウインドウの大きさ::ウインドウの記憶へ変換する(
                    window_size,
                    window_maximized,
                ),
            音量: volume.map_or(既定.音量, 音量::範囲へ収めて作る),
            速度: playback_speed
                .and_then(つまみの範囲へ収めた速度)
                .unwrap_or(既定.速度),
            左右: match mirror_horizontally {
                Some(true) => 左右の向き::反転,
                Some(false) | None => 左右の向き::そのまま,
            },
            アスペクト比: aspect_ratio
                .and_then(|表記| 表記から読む(&アスペクト比の表記, &表記))
                .unwrap_or(既定.アスペクト比),
            表示サイズ: display_size
                .and_then(|表記| 表記から読む(&表示サイズの表記, &表記))
                .unwrap_or(既定.表示サイズ),
            キー: 第1版のキーの割り当て::キーの割り当てへ変換する(
                key_bindings,
            ),
        })
    }

    /// 最新の見る側の設定から作る。覚えていなければ、どの項目も書かない。
    pub(super) fn 見る側の設定から作る(
        覚えた設定: 覚えた見る側の設定
    ) -> Self {
        let 設定 = match 覚えた設定 {
            覚えた見る側の設定::覚えていない => return Self::default(),
            覚えた見る側の設定::覚えている(設定) => 設定,
        };
        let (window_size, window_maximized) =
            第1版のウインドウの大きさ::ウインドウの記憶から作る(
                設定.ウインドウ,
            );
        Self {
            window_size: Some(window_size),
            window_maximized: Some(window_maximized),
            volume: Some(設定.音量.値()),
            playback_speed: Some(設定.速度.倍率()),
            mirror_horizontally: Some(設定.左右 == 左右の向き::反転),
            aspect_ratio: Some(表記にする(&アスペクト比の表記, 設定.アスペクト比)),
            display_size: Some(表記にする(&表示サイズの表記, 設定.表示サイズ)),
            key_bindings: 第1版のキーの割り当て::キーの割り当てから作る(
                設定.キー,
            ),
        }
    }
}
