//! 設定ファイルの読み込みと書き出しで起きるエラー。
//! 公開の型に serde_json の型を出さず、JSONの不備は位置と理由を自前の型で持つ(serde_json を settings の中に閉じるため)。

use crate::duration::時間の値エラー;
use crate::stack_error::クリップスタックの操作エラー;
use crate::video_span::区間エラー;

/// JSONの不備とは、JSONの本文のどこがどう読めなかったかを、1から数えた行と列と理由の文で表したもののことである。
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("{行}行{列}列: {理由}")]
pub struct JSONの不備 {
    /// 1から数えた行。
    pub 行: usize,
    /// 1から数えた列。
    pub 列: usize,
    /// 読めなかった理由。
    pub 理由: String,
}

/// 設定ファイルの読み込みエラーとは、設定ファイルの本文からスタック設定を作れなかった理由のことである。
#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum 設定ファイルの読み込みエラー {
    /// 本文がJSONとして解析できない。
    #[error("JSON解析に失敗: {0}")]
    JSONとして解析できない(JSONの不備),
    /// application がこのアプリの名前でない。
    #[error("不正な設定ファイル形式です(application が ModifierVideoStack でない: {0:?})")]
    アプリケーション名が違う(Option<String>),
    /// version がこのアプリの読める版でない。
    #[error("対応していない設定ファイルの版です: {0}")]
    対応していない版(String),
    /// 項目の型や表記が第4版の形式に合わない。
    #[error("不正な設定ファイル形式です: {0}")]
    形式が不正(JSONの不備),
    /// 指定の番号(0から数える)のクリップの値が成立しない。
    #[error("{番号}番目のクリップが不正です: {理由}")]
    クリップが不正 {
        /// 0から数えたクリップの番号。
        番号: usize,
        /// 成立しない理由。
        理由: クリップの値の不備,
    },
    /// クリップの一覧からスタックを作れない(識別子の重複)。
    #[error("クリップの一覧が不正です: {0}")]
    スタックを作れない(クリップスタックの操作エラー),
}

/// JSONの誤りの種類とは、JSONの構文として読めなかったか、JSONとしては読めたが項目の型や表記が形式に合わなかったかの区別のことである。
pub(crate) enum JSONの誤りの種類 {
    構文(JSONの不備),
    形(JSONの不備),
}

impl JSONの誤りの種類 {
    /// serde_json の解析のエラーから、位置と理由を写して種類を分ける。
    pub(crate) fn 解析のエラーから作る(エラー: &serde_json::Error) -> Self {
        let 不備 = JSONの不備 {
            行: エラー.line(),
            列: エラー.column(),
            理由: エラー.to_string(),
        };
        match エラー.classify() {
            serde_json::error::Category::Data => Self::形(不備),
            _ => Self::構文(不備),
        }
    }
}

impl 設定ファイルの読み込みエラー {
    /// serde_json の解析のエラーを、構文の誤りなら「解析できない」、型や表記の誤りなら「形式が不正」へ分けて変換する。
    pub(crate) fn 解析のエラーから作る(エラー: serde_json::Error) -> Self {
        match JSONの誤りの種類::解析のエラーから作る(&エラー) {
            JSONの誤りの種類::形(不備) => Self::形式が不正(不備),
            JSONの誤りの種類::構文(不備) => Self::JSONとして解析できない(不備),
        }
    }
}

/// クリップの値の不備とは、設定ファイルのクリップの値がドメインの型として成立しない理由のことである。
#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum クリップの値の不備 {
    /// start か end が時刻として成立しない。
    #[error("時刻が不正: {0}")]
    時刻(#[from] 時間の値エラー),
    /// start が end より後ろにある。
    #[error("{0}")]
    区間(#[from] 区間エラー),
    /// repeat が数値でも "infinite" でもない。
    #[error("repeat が数値でも \"infinite\" でもない: {0}")]
    繰り返し(String),
}

/// 設定ファイルの書き出しエラーとは、書き出す設定をJSONの本文へ変換できなかった理由のことである。
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("設定ファイルの書き出しに失敗: {理由}")]
pub struct 設定ファイルの書き出しエラー {
    /// 変換できなかった理由。
    pub 理由: String,
}
