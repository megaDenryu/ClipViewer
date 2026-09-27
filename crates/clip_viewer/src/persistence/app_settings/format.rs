//! アプリの設定と settings.json の本文の変換。読み込みは見出し(format と version)で版を判別し、版に合う型で読んで最新へ変換する。
//! 版を足すときは、版に合う型で読む選択肢をここへ足す。参照: _doc/設計/ライブラリ.md「形式の版を上げる手順」

use serde::Deserialize;

use super::error::{書き換えない理由, 設定の本文のエラー};
use super::settings::アプリの設定;
use super::v0::第0版の設定;
use super::v1::{形式の名前, 第1版の番号, 第1版の設定};

/// このアプリが知っている最新の版の番号。これより大きい版は、このアプリより新しい ClipViewer が書いたファイルである。
const 最新の版の番号: u64 = 第1版の番号;

/// 版の見出しとは、中身を読む前に確かめる format と version の2項目のことである。
#[derive(Deserialize)]
struct 版の見出し {
    format: Option<String>,
    version: Option<u64>,
}

/// 設定のファイルの本文とは、settings.json の中身(JSONの文字列)そのもののことである。
pub(super) struct 設定のファイルの本文(String);

impl 設定のファイルの本文 {
    pub(super) fn 作成する(本文: String) -> Self {
        Self(本文)
    }

    /// 本文の文字列を返す。ファイルへ書く境界で使う。
    pub(super) fn 文字列(&self) -> &str {
        &self.0
    }

    /// 最新のアプリの設定として読む。format と version の両方が無ければ第0版として読む。
    pub(super) fn 最新の設定として読む(
        &self,
    ) -> Result<アプリの設定, 設定の本文のエラー> {
        let 見出し: 版の見出し = serde_json::from_str(&self.0)?;
        match (見出し.format, 見出し.version) {
            (None, None) => Ok(serde_json::from_str::<第0版の設定>(&self.0)?.最新へ変換する()),
            (Some(名前), 版) if 名前 == 形式の名前 => self.版を選んで読む(版),
            (名前, _) => Err(書き換えない理由::形式の名前が違う(名前).into()),
        }
    }

    fn 版を選んで読む(
        &self,
        版: Option<u64>,
    ) -> Result<アプリの設定, 設定の本文のエラー> {
        match 版 {
            Some(第1版の番号) => {
                Ok(serde_json::from_str::<第1版の設定>(&self.0)?.最新へ変換する())
            }
            Some(版) if 版 > 最新の版の番号 => {
                Err(書き換えない理由::版が新しすぎる(版).into())
            }
            その他 => Err(書き換えない理由::対応していない版(その他).into()),
        }
    }

    /// 最新の版で、人が読める形(字下げ付き)の本文へ書き出す。
    pub(super) fn 設定から書き出す(
        設定: アプリの設定
    ) -> Result<Self, serde_json::Error> {
        serde_json::to_string_pretty(&第1版の設定::最新から作る(設定)).map(Self)
    }
}
