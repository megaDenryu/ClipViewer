//! 検査の結果と、見つけたこと(禁じた参照か、調べられなかったファイル)と、その表示の文。

use std::fmt;
use std::path::PathBuf;

use super::module_path::禁じた参照;
use super::tokens::解析できない理由;

/// 見つけたこととは、検査が報告する、禁じた参照か、調べられなかったファイルのことである。
pub enum 見つけたこと {
    禁じた参照(PathBuf, 禁じた参照),
    解析できない(PathBuf, 解析できない理由),
}

/// 検査の結果とは、調べたファイルの数と、見つけたことの並びの組のことである。
pub struct 検査の結果 {
    pub 調べたファイルの数: usize,
    pub 見つけたことの並び: Vec<見つけたこと>,
}

impl fmt::Display for 見つけたこと {
    fn fmt(&self, 書き込み先: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::禁じた参照(ファイル, 参照) => write!(
                書き込み先,
                "{}:{}: スタックの作業場の {} を使っている",
                ファイル.display(),
                参照.行,
                参照.crateルートからのパス
            ),
            Self::解析できない(ファイル, 理由) => write!(
                書き込み先,
                "{}:{}: 調べられない: {}",
                ファイル.display(),
                理由.行,
                理由.説明
            ),
        }
    }
}
