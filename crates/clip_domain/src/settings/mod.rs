//! 設定ファイル(JSON)の読み書き。ブラウザ版 ClipViewer の v4.0 形式と双方向で互換である。
//! 版ごとの型(v4.rs)を置き、読み込みは版を判別して最新のスタック設定へ変換し、書き出しは第4版の形で行う。

mod error;
mod export_date;
mod v4;
mod v4_read;
mod v4_trigger;
mod v4_write;
mod write;

pub(crate) use error::JSONの誤りの種類;
pub use error::{
    JSONの不備, クリップの値の不備, 設定ファイルの書き出しエラー, 設定ファイルの読み込みエラー,
};
pub use export_date::{書き出し日時, 書き出し日時エラー};
pub use write::書き出す設定;

use crate::clip_id::クリップ識別子の発行元;
use crate::stack::クリップスタック;
use crate::video_path::{動画ファイル名, 正規化した動画パス};
use v4::{
    アプリケーション名, 版の表記, 版の見出し, 第4版の設定ファイル
};

/// スタック設定とは、設定ファイル1つから読み込んだ内容、すなわちクリップスタックと、それを作った動画の手がかりの組のことである。
#[derive(Debug, Clone, PartialEq)]
pub struct スタック設定 {
    /// クリップの並びと選択。読み込み直後は先頭のクリップが選択されている。
    pub スタック: クリップスタック,
    /// スタックを作ったときの動画のファイル名。
    pub 期待する動画: Option<動画ファイル名>,
    /// スタックを作ったときの動画のパス(読み込みの時点で正規化済み。検証はしていない)。
    pub 期待する動画のパス: Option<正規化した動画パス>,
}

/// 設定ファイルの本文とは、設定ファイルに書かれたJSONの文字列そのもののことである。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct 設定ファイルの本文(String);

impl 設定ファイルの本文 {
    /// ファイルから読んだ文字列から作成する。
    pub fn 作成する(本文: String) -> Self {
        Self(本文)
    }

    /// 本文の文字列を返す。ファイルへ書く境界で使う。
    pub fn 文字列(&self) -> &str {
        &self.0
    }
}

impl スタック設定 {
    /// 設定ファイルの本文から読み込む。識別子が欠けたクリップには発行元が新しい識別子を発行する。
    /// version が無いファイルは第4版として読む(移植元は version を確かめていなかったため)。
    pub fn 本文から読み込む(
        本文: &設定ファイルの本文,
        発行元: &mut impl クリップ識別子の発行元,
    ) -> Result<Self, 設定ファイルの読み込みエラー> {
        let 見出し: 版の見出し = serde_json::from_str(&本文.0)
            .map_err(設定ファイルの読み込みエラー::解析のエラーから作る)?;
        if 見出し.application.as_deref() != Some(アプリケーション名) {
            return Err(
                設定ファイルの読み込みエラー::アプリケーション名が違う(
                    見出し.application,
                ),
            );
        }
        match 見出し.version.as_deref() {
            None | Some(版の表記) => {
                serde_json::from_str::<第4版の設定ファイル>(&本文.0)
                    .map_err(設定ファイルの読み込みエラー::解析のエラーから作る)?
                    .最新へ変換する(発行元)
            }
            Some(その他の版) => Err(
                設定ファイルの読み込みエラー::対応していない版(
                    その他の版.to_string(),
                ),
            ),
        }
    }
}
