//! モジュールパスと、ソースに書かれたパスを crate ルートからのパスへ直すこと。`crate`・`super`・`self` で始まるパスだけを crate の中のパスとして直す。
//! ほかの名前で始まるパスは、外のクレートか今のモジュールの中の名前であり、crate ルートのモジュールを指さない(ファイルが crate ルートでないため)。

use super::tokens::解析できない理由;

/// 重ね合わせの作業場の層が使ってはならない、スタックの作業場のモジュール。
const 禁じたモジュール: [&str; 3] = ["state", "command", "view"];

/// モジュールパスとは、crate ルートから、あるファイルまたはその中の `mod 名前 { … }` までのモジュールの名前の並びのことである。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct モジュールパス(Vec<String>);

/// ソースに書かれたパスとは、ソースに書かれたパスの行と、`::` で区切った名前の並び(`use` の波括弧は展開したもの)の組のことである。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ソースに書かれたパス {
    pub 行: usize,
    pub 名前の並び: Vec<String>,
}

/// 禁じた参照とは、重ね合わせの作業場の層のファイルに書かれた、スタックの作業場のモジュールを指すパスの行と、crate ルートからcrateルートからのパスの組のことである。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct 禁じた参照 {
    pub 行: usize,
    pub crateルートからのパス: String,
}

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

    /// ソースに書かれたパスをこのモジュールパスから crate ルートのパスへ直し、禁じたモジュールを指すなら禁じた参照を返す。
    /// `super` が crate ルートより上を指すパスは、直せないため解析できない理由を返す。
    pub fn 禁じた参照なら返す(
        &self,
        パス: &ソースに書かれたパス,
    ) -> Result<Option<禁じた参照>, 解析できない理由> {
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
                            "super が crate ルートより上を指す",
                        )
                    })?;
                }
                "self" => {}
                _ => 直した.push(名前.clone()),
            }
        }
        Ok(直した
            .first()
            .filter(|先頭| 禁じたモジュール.contains(&先頭.as_str()))
            .map(|_| 禁じた参照 {
                行: パス.行,
                crateルートからのパス: format!("crate::{}", 直した.join("::")),
            }))
    }
}
