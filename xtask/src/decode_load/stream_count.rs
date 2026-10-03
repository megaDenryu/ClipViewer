//! 同時に起動するffmpegの数の型と、`--streams` の表記の読み方。

use std::fmt;
use std::ops::Range;

/// `--streams` を省いたときに測る数の並び。設計文書 5-4 の表と同じ並びである。
const 既定の同時に起動する数の並び: [u32; 4] = [1, 2, 4, 8];
/// 同時に起動する ffmpeg の数の上限。論理 CPU の数を大きく超える数は測る意味が無く、ffmpeg を起動しすぎないためである。
const 同時に起動する数の上限: u32 = 64;

/// 同時に起動するffmpegの数とは、同じ動画を同時に読み切る ffmpeg のプロセスの数(1以上、上限以下)のことである。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct 同時に起動するffmpegの数(u32);

impl 同時に起動するffmpegの数 {
    /// 数から作成する。0か上限を超えるなら作らない。
    pub fn 作成する(数: u32) -> Option<Self> {
        (1..=同時に起動する数の上限)
            .contains(&数)
            .then_some(Self(数))
    }

    /// `--streams` を省いたときに測る数の並び(1,2,4,8)。
    pub fn 既定の並び() -> Vec<Self> {
        既定の同時に起動する数の並び.map(Self).to_vec()
    }

    /// `1,2,4,8` のようにカンマで区切った表記を読む。1から上限までの整数でない要素があれば、その要素を挙げて失敗にする。
    pub fn 並びを表記から読む(表記: &str) -> Result<Vec<Self>, String> {
        表記
            .split(',')
            .map(|一つ| {
                一つ.trim().parse::<u32>().ok().and_then(Self::作成する).ok_or_else(|| {
                    format!("--streams の「{一つ}」は 1 から {同時に起動する数の上限} までの整数でない")
                })
            })
            .collect()
    }

    /// 起動する1本ごとに1つずつ数える番号の範囲(0から数の手前まで)。
    pub fn 起動する番号の範囲(self) -> Range<u32> {
        0..self.0
    }

    /// 速さを本数で割り掛けするときの小数。
    pub fn 小数として(self) -> f64 {
        f64::from(self.0)
    }
}

impl fmt::Display for 同時に起動するffmpegの数 {
    fn fmt(&self, 書き先: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(書き先, "{}", self.0)
    }
}
