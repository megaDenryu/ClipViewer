//! 依存の向きの検査の本文の試験の道具。`src` から見たファイルの位置に本文を書いたときの、重ね合わせの作業場の層の禁じた参照の書き方の並びを返す。
#![allow(clippy::expect_used)]

use super::layer::重ね合わせの作業場の層;
use super::module_path::モジュールパス;
use super::paths::禁じた参照を探す;
use super::tokens::字句に分ける;

/// `src` から見たファイルの位置(`/` 区切り)のファイルに本文を書いたときの、禁じた参照のクレートルートからのパスの並び。解析できなければその説明。
pub(super) fn 本文の禁じた参照を調べる(
    位置: &str,
    本文: &str,
) -> Result<Vec<String>, String> {
    let 位置の並び: Vec<String> = 位置.split('/').map(str::to_string).collect();
    let 今のモジュールパス = モジュールパス::ファイルの位置から作る(&位置の並び);
    字句に分ける(本文)
        .and_then(|並び| {
            禁じた参照を探す(
                &並び,
                &今のモジュールパス,
                重ね合わせの作業場の層.禁じたモジュール,
            )
        })
        .map(|参照の並び| {
            参照の並び
                .into_iter()
                .map(|参照| 参照.クレートルートからのパス)
                .collect()
        })
        .map_err(|理由| 理由.説明)
}

/// 解析できるはずの本文の、禁じた参照の書き方の並び。
pub(super) fn 解析できる本文の禁じた参照を調べる(
    位置: &str,
    本文: &str,
) -> Vec<String> {
    本文の禁じた参照を調べる(位置, 本文).expect("解析できる")
}
