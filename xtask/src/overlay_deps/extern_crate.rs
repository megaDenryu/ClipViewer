//! `extern crate` 宣言の読み方。`extern crate self as 別名;` はクレートルートに別名を付け、別名からスタックの作業場のモジュールを
//! 名指さずに使えてしまうため、禁じた参照にする。外のクレートの `extern crate 名前 (as 別名);` は調べずに読み飛ばす。

use super::forbidden_reference::{禁じた参照, 禁じた理由};
use super::token::{字句, 字句の種類};

/// 読んだextern宣言とは、`extern` から読み終えた次の字句の位置と、`extern crate self` なら禁じた参照の組のことである。
pub struct 読んだextern宣言 {
    pub 次の位置: usize,
    pub 禁じた参照: Option<禁じた参照>,
}

fn 並びの位置の名前(並び: &[字句], 位置: usize) -> Option<&str> {
    match 字句::並びの位置の種類(並び, 位置) {
        Some(字句の種類::名前(名前)) => Some(名前.as_str()),
        _ => None,
    }
}

/// 位置の `extern` から読む。`extern crate` でなければ(`extern "C" fn` 等)、`extern` だけを読んで禁じた参照は無い。
pub fn extern宣言を読む(並び: &[字句], 位置: usize) -> 読んだextern宣言 {
    let 行 = 並び.get(位置).map_or(0, |字句| 字句.行);
    let (Some("crate"), Some(クレート)) = (
        並びの位置の名前(並び, 位置 + 1),
        並びの位置の名前(並び, 位置 + 2),
    ) else {
        return 読んだextern宣言 {
            次の位置: 位置 + 1,
            禁じた参照: None,
        };
    };
    let 別名 = (並びの位置の名前(並び, 位置 + 3) == Some("as"))
        .then(|| 並びの位置の名前(並び, 位置 + 4))
        .flatten();
    let 次の位置 = 位置 + if 別名.is_some() { 5 } else { 3 };
    let 禁じた参照 = (クレート == "self").then(|| 禁じた参照 {
        行,
        クレートルートからのパス: match 別名 {
            Some(別名) => format!("extern crate self as {別名}"),
            None => "extern crate self".to_string(),
        },
        理由: 禁じた理由::クレートルートに別名を付ける,
    });
    読んだextern宣言 {
        次の位置,
        禁じた参照,
    }
}
