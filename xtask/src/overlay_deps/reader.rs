//! 文字の読み手。Rust のソースを1文字ずつ読み、行を数え、名前を読む。コメントと文字列を読み飛ばすことは、この型の公開の口だけを使う
//! `skipping.rs` の型が受け持つ。

use super::reason::解析できない理由;
use super::skipping::コメントと文字列の読み飛ばし;

/// 文字の読み手とは、ソースの文字の並びと、今読んでいる位置と行(1から数える)の組のことである。
pub struct 文字の読み手 {
    文字: Vec<char>,
    位置: usize,
    行: usize,
}

impl 文字の読み手 {
    pub fn 作成する(本文: &str) -> Self {
        Self {
            文字: 本文.chars().collect(),
            位置: 0,
            行: 1,
        }
    }

    /// 今読んでいる行(1から数える)。
    pub fn 今の行(&self) -> usize {
        self.行
    }

    /// 今の位置からずれだけ先の文字。終わりを越えれば無い。
    pub fn 先の文字を見る(&self, ずれ: usize) -> Option<char> {
        self.文字.get(self.位置 + ずれ).copied()
    }

    /// 1文字読んで返す。改行なら行を進める。終わりなら無い。
    pub fn 一文字読み進める(&mut self) -> Option<char> {
        let 文字 = self.先の文字を見る(0)?;
        self.位置 += 1;
        if 文字 == '\n' {
            self.行 += 1;
        }
        Some(文字)
    }

    /// 数だけ文字を読み進める。
    pub fn 文字を読み進める(&mut self, 数: usize) {
        for _ in 0..数 {
            let _ = self.一文字読み進める();
        }
    }

    /// 改行の手前まで読み飛ばす(行コメント)。
    pub fn 行の終わりまで読み飛ばす(&mut self) {
        while self.先の文字を見る(0).is_some_and(|文字| 文字 != '\n') {
            let _ = self.一文字読み進める();
        }
    }

    /// 名前(英字・数字・下線と、日本語のような英字以外の文字)を読む。
    pub fn 名前を読む(&mut self) -> String {
        let mut 名前 = String::new();
        while let Some(文字) = self
            .先の文字を見る(0)
            .filter(|文字| 文字.is_alphanumeric() || *文字 == '_')
        {
            名前.push(文字);
            let _ = self.一文字読み進める();
        }
        名前
    }

    /// 名前を読む。名前が文字列の前置き(`r"`・`r#"`・`b"`・`br"`・`c"`・`cr"`・`b'`)なら、その文字列か文字を読み飛ばして無いを返す。
    /// 生の名前(`r#型`)は `r` と `#` を除いた名前を返す。数(`1.5`・`0x1F`)も名前として返すが、パスの先頭にならないため検査に影響しない。
    pub fn 名前か文字列の前置きを読む(
        &mut self,
    ) -> Result<Option<String>, 解析できない理由> {
        let 名前 = self.名前を読む();
        let 生の前置きか = matches!(名前.as_str(), "r" | "br" | "cr");
        let 井桁の数 = (0..)
            .take_while(|ずれ| self.先の文字を見る(*ずれ) == Some('#'))
            .count();
        match (self.先の文字を見る(井桁の数), 井桁の数) {
            (Some('"'), _) if 生の前置きか => {
                self.文字を読み進める(井桁の数 + 1);
                コメントと文字列の読み飛ばし::作成する(self)
                    .生の文字列を読み飛ばす(井桁の数)?;
                Ok(None)
            }
            (Some('"'), 0) if matches!(名前.as_str(), "b" | "c") => {
                self.文字を読み進める(1);
                コメントと文字列の読み飛ばし::作成する(self).文字列を読み飛ばす()?;
                Ok(None)
            }
            (Some('\''), 0) if 名前 == "b" => {
                self.文字を読み進める(1);
                コメントと文字列の読み飛ばし::作成する(self).文字なら読み飛ばす()?;
                Ok(None)
            }
            (Some(_), 1) if 名前 == "r" => {
                self.文字を読み進める(1);
                Ok(Some(self.名前を読む()))
            }
            _ => Ok(Some(名前)),
        }
    }
}
