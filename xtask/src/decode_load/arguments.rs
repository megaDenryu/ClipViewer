//! `decode-load` のコマンド行引数の解釈: 測る動画の出どころ・同時に起動するffmpegの数の並び・読む長さ。

use std::path::PathBuf;

use super::read_length::読む秒数;
use super::stream_count::同時に起動するffmpegの数;
use super::video_origin::測る動画の出どころ;

/// 測定の指定とは、decode-load の引数から決まる、測る動画の出どころと同時に起動するffmpegの数の並びと読む長さの組のことである。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct 測定の指定 {
    pub 動画の出どころ: 測る動画の出どころ,
    pub ffmpegの数の並び: Vec<同時に起動するffmpegの数>,
    pub 読む長さ: 読む秒数,
}

impl 測定の指定 {
    /// `[<動画のパス>] [--streams <数,...>] [--seconds <秒数>]` を解釈する。解釈できない引数は、理由を付けて失敗にする。
    pub fn 引数から解釈する(引数一覧: &[String]) -> Result<Self, String> {
        let mut 指定 = Self {
            動画の出どころ: 測る動画の出どころ::合成画像で作る,
            ffmpegの数の並び: 同時に起動するffmpegの数::既定の並び(),
            読む長さ: 読む秒数::既定,
        };
        let mut 残り = 引数一覧.iter();
        while let Some(引数) = 残り.next() {
            match 引数.as_str() {
                "--streams" => {
                    let 表記 = オプションの値を取り出す(&mut 残り, 引数)?;
                    指定.ffmpegの数の並び =
                        同時に起動するffmpegの数::並びを表記から読む(表記)?;
                }
                "--seconds" => {
                    let 表記 = オプションの値を取り出す(&mut 残り, 引数)?;
                    指定.読む長さ = 読む秒数::表記から読む(表記)?;
                }
                名前 if 名前.starts_with("--") => {
                    return Err(format!("不明なオプション「{名前}」"));
                }
                パス => 指定.動画のパスを受け取る(パス)?,
            }
        }
        Ok(指定)
    }

    // 動画のパスは1つだけ受け取る。2つ目が来たら、両方を挙げて失敗にする。
    fn 動画のパスを受け取る(&mut self, パス: &str) -> Result<(), String> {
        if let 測る動画の出どころ::与えられたファイル(前) = &self.動画の出どころ
        {
            let 前 = 前.display();
            return Err(format!("動画のパスは1つだけ渡せる(「{前}」と「{パス}」)"));
        }
        self.動画の出どころ =
            測る動画の出どころ::与えられたファイル(PathBuf::from(パス));
        Ok(())
    }
}

fn オプションの値を取り出す<'a>(
    残り: &mut std::slice::Iter<'a, String>,
    オプション: &str,
) -> Result<&'a str, String> {
    残り
        .next()
        .map(String::as_str)
        .ok_or_else(|| format!("{オプション} の後に値が無い"))
}
