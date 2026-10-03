//! コマ数の型。1本の ffmpeg が読み出したコマの枚数を表す。

use std::fmt;

/// コマ数とは、読み出したコマの枚数(1以上)のことである。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct コマ数(u32);

impl コマ数 {
    /// 枚数から作成する。0なら速さを求められないため作らない。
    pub fn 作成する(枚数: u32) -> Option<Self> {
        (枚数 > 0).then_some(Self(枚数))
    }

    /// 速さを求めるときの小数。
    pub fn 小数として(self) -> f64 {
        f64::from(self.0)
    }
}

impl fmt::Display for コマ数 {
    fn fmt(&self, 書き先: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(書き先, "{}コマ", self.0)
    }
}
