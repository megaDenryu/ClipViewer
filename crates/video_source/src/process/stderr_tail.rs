//! ffmpeg の標準エラー出力を読み続け、末尾だけを残す。

use std::fmt;
use std::io::{self, Read};
use std::process::ChildStderr;

use super::thread::終わりを待つスレッド;

/// 残す末尾のバイト数。失敗の理由を示すには末尾の数行で足りる。
const 残す長さ: usize = 4096;

/// 標準エラーの末尾とは、ffmpeg が標準エラー出力へ書いた内容の最後の部分のことである。失敗の理由を示すために使う。
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct 標準エラーの末尾(String);

impl 標準エラーの末尾 {
    /// 末尾の文字列。
    pub fn 文字列(&self) -> &str {
        &self.0
    }

    /// 読み終えた標準エラー出力の全体から、末尾だけを残して作る。
    pub(crate) fn 出力の全体から作る(出力: &[u8]) -> Self {
        let 始まり = 出力.len().saturating_sub(残す長さ);
        Self(String::from_utf8_lossy(&出力[始まり..]).into_owned())
    }
}

impl fmt::Display for 標準エラーの末尾 {
    fn fmt(&self, 書き先: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(書き先, "標準エラーの末尾: {}", self.0.trim())
    }
}

/// 標準エラーの収集とは、標準エラー出力を裏のスレッドで読み続ける仕組みのことである。
/// 読み続けるのは、パイプが詰まって ffmpeg が止まることを防ぐためである。
pub(crate) struct 標準エラーの収集(終わりを待つスレッド<標準エラーの末尾>);

impl 標準エラーの収集 {
    /// 標準エラー出力を読むスレッドを起動する。
    pub(crate) fn 起動する(出力: ChildStderr) -> io::Result<Self> {
        終わりを待つスレッド::起動する("ffmpeg の標準エラーを読む", move || {
            末尾まで読み続ける(出力)
        })
        .map(Self)
    }

    /// 出力が閉じるまで待ち、末尾を受け取る。
    pub(crate) fn 読み終えるまで待つ(&mut self) -> 標準エラーの末尾 {
        self.0.終わるまで待つ().unwrap_or_else(|| {
            標準エラーの末尾("標準エラー出力を読むスレッドが異常終了した".to_string())
        })
    }
}

fn 末尾まで読み続ける(mut 出力: ChildStderr) -> 標準エラーの末尾 {
    let mut 溜め: Vec<u8> = Vec::new();
    let mut 一回分 = [0_u8; 1024];
    loop {
        match 出力.read(&mut 一回分) {
            Ok(0) => break,
            Ok(読んだ長さ) => 溜め.extend_from_slice(&一回分[..読んだ長さ]),
            Err(原因) if 原因.kind() == io::ErrorKind::Interrupted => continue,
            Err(_) => break,
        }
        if 溜め.len() > 残す長さ * 2 {
            溜め.drain(..溜め.len() - 残す長さ);
        }
    }
    標準エラーの末尾::出力の全体から作る(&溜め)
}
