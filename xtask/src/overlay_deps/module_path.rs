//! モジュールパスと、ソースに書かれたパスを クレートルートからのパスへ直すこと。`crate`・`super`・`self` で始まるパスだけを crate の中のパスとして直す。
//! ほかの名前で始まるパスは、外のクレートか今のモジュールの中の名前であり、クレートルートのモジュールを指さない(ファイルが クレートルートでないため)。

use super::forbidden_reference::{禁じた参照, 禁じた理由};
use super::reason::解析できない理由;
use super::written_path::{ソースに書かれたパス, パスの終わり方};

/// モジュールパスとは、クレートルートから、あるファイルまたはその中の `mod 名前 { … }` までのモジュールの名前の並びのことである。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct モジュールパス(Vec<String>);

impl モジュールパス {
    /// `src` から見たファイルの位置(`overlay/view/mod.rs` を `["overlay", "view", "mod.rs"]` と並べたもの)から作る。
    /// `mod.rs` はそのフォルダのモジュールであり、ほかのファイルはファイル名から拡張子を除いた名前のモジュールである。
    pub fn ファイルの位置から作る(位置: &[String]) -> Self {
        let mut 名前の並び: Vec<String> = 位置.to_vec();
        match 名前の並び.pop() {
            Some(ファイル名) if ファイル名 != "mod.rs" => {
                名前の並び.push(ファイル名.trim_end_matches(".rs").to_string())
            }
            _ => {}
        }
        Self(名前の並び)
    }

    /// ファイルの中の `mod 名前 {` へ入る。
    pub fn 中のモジュールへ入る(&mut self, 名前: String) {
        self.0.push(名前);
    }

    /// ファイルの中の `mod 名前 { … }` の閉じの `}` で出る。
    pub fn 中のモジュールから出る(&mut self) {
        self.0.pop();
    }

    /// ソースに書かれたパスをこのモジュールパスから クレートルートのパスへ直し、禁じた書き方なら禁じた参照を返す。
    /// 禁じた書き方は、禁じたモジュール(検査する層ごとに違う)を指すことと、クレートルートを `*` で全部取り込むことと、クレートルートに `as` で別名を付けることである。
    /// `super` が クレートルートより上を指すパスは、直せないため解析できない理由を返す。
    pub fn 禁じた参照なら返す(
        &self,
        パス: &ソースに書かれたパス,
        禁じたモジュール: &[&str],
    ) -> Result<Option<禁じた参照>, 解析できない理由> {
        let Some(直した) = self.クレートルートから直す(パス)? else {
            return Ok(None);
        };
        let 理由 = match (直した.first(), &パス.終わり方) {
            (Some(先頭), _) if 禁じたモジュール.contains(&先頭.as_str()) => {
                禁じた理由::禁じたモジュールを指す
            }
            (None, パスの終わり方::全部を取り込む) => {
                禁じた理由::クレートルートの全部を取り込む
            }
            (None, パスの終わり方::別名を付ける(_)) => {
                禁じた理由::クレートルートに別名を付ける
            }
            _ => return Ok(None),
        };
        let 書き方 = std::iter::once("crate".to_string())
            .chain(直した)
            .collect::<Vec<_>>()
            .join("::");
        let クレートルートからのパス = match &パス.終わり方 {
            パスの終わり方::名前で終わる => 書き方,
            パスの終わり方::全部を取り込む => format!("{書き方}::*"),
            パスの終わり方::別名を付ける(別名) => format!("{書き方} as {別名}"),
        };
        Ok(Some(禁じた参照 {
            行: パス.行,
            クレートルートからのパス,
            理由,
        }))
    }

    /// `crate`・`super`・`self` で始まるパスを、クレートルートからの名前の並びへ直す。それ以外で始まるパスは無い。
    fn クレートルートから直す(
        &self,
        パス: &ソースに書かれたパス,
    ) -> Result<Option<Vec<String>>, 解析できない理由> {
        let 先頭 = パス.名前の並び.first().map(String::as_str);
        let mut 直した = match 先頭 {
            Some("crate") => Vec::new(),
            Some("super" | "self") => self.0.clone(),
            _ => return Ok(None),
        };
        for 名前 in パス
            .名前の並び
            .iter()
            .skip(usize::from(先頭 == Some("crate")))
        {
            match 名前.as_str() {
                "super" => {
                    直した.pop().ok_or_else(|| {
                        解析できない理由::作成する(
                            パス.行,
                            "super が クレートルートより上を指す",
                        )
                    })?;
                }
                "self" => {}
                _ => 直した.push(名前.clone()),
            }
        }
        Ok(Some(直した))
    }
}
