//! 読む秒数の型と、`--seconds` の表記の読み方。
//! 秒の系統は2つあり、引数で指定する長さはこの型(整数の秒)、測った時間は `読み切る時間`(std の Duration)が持つ。
//! この型は ffmpeg へ渡す秒の表記への変換の窓口であり、Duration へは変換しない。

use std::fmt;

/// `--seconds` を省いたときの秒数。設計文書 5-4 の表を作ったときの合成画像の長さと同じである。
const 既定の読む秒数: u32 = 30;
/// 読む長さの上限(秒)。合成画像を作る時間と、長い実写の動画を読み切る時間を抑えるためである。
const 読む秒数の上限: u32 = 600;

/// 読む秒数とは、動画の先頭から読む長さを秒で表したもの(1以上、上限以下)のことである。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct 読む秒数(u32);

impl 読む秒数 {
    /// `--seconds` を省いたときの長さ(30秒)。
    pub const 既定: Self = Self(既定の読む秒数);

    /// 秒数から作成する。0か上限を超えるなら作らない。
    pub fn 作成する(秒数: u32) -> Option<Self> {
        (1..=読む秒数の上限).contains(&秒数).then_some(Self(秒数))
    }

    /// 秒数の表記を読む。1から上限までの整数でなければ失敗にする。
    pub fn 表記から読む(表記: &str) -> Result<Self, String> {
        表記
            .parse::<u32>()
            .ok()
            .and_then(Self::作成する)
            .ok_or_else(|| {
                format!("--seconds の「{表記}」は 1 から {読む秒数の上限} までの整数でない")
            })
    }

    /// ffmpeg の -t と lavfi の duration へ渡す秒の表記。
    pub fn ffmpegへ渡す秒の表記(self) -> String {
        self.0.to_string()
    }
}

impl fmt::Display for 読む秒数 {
    fn fmt(&self, 書き先: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(書き先, "{}秒", self.0)
    }
}
