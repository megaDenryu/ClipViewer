//! 読む秒数の型と、`--seconds` の表記の読み方。

/// 読む長さを省いたときの秒数。設計文書 5-4 の表を作ったときの合成画像の長さと同じである。
const 既定の読む秒数: u32 = 30;
/// 読む長さの上限(秒)。合成画像を作る時間と、長い実写の動画を読み切る時間を抑えるためである。
const 読む秒数の上限: u32 = 600;

/// 読む秒数とは、動画の先頭から読む長さを秒で表したもの(1以上)のことである。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct 読む秒数(u32);

impl 読む秒数 {
    /// `--seconds` を省いたときの長さ(30秒)。
    pub const 既定: Self = Self(既定の読む秒数);

    /// 秒数。
    pub fn 値(self) -> u32 {
        self.0
    }

    /// 秒数の表記を読む。1から上限までの整数でなければ失敗にする。
    pub fn 表記から読む(表記: &str) -> Result<Self, String> {
        表記
            .parse::<u32>()
            .ok()
            .filter(|秒数| (1..=読む秒数の上限).contains(秒数))
            .map(Self)
            .ok_or_else(|| {
                format!("--seconds の「{表記}」は 1 から {読む秒数の上限} までの整数でない")
            })
    }
}
