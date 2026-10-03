//! 再生の速さに対する倍率の型。

use std::fmt;

/// 再生の速さに対する倍率とは、読み出しの速さを動画の再生の速さで割った比のことである。1を下回ると流し読みが再生に間に合わない。
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct 再生の速さに対する倍率(f64);

impl 再生の速さに対する倍率 {
    /// 読み出しの速さと再生の速さの比から作る。
    pub fn 速さの比から作る(比: f64) -> Self {
        Self(比)
    }
}

impl fmt::Display for 再生の速さに対する倍率 {
    fn fmt(&self, 書き先: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(書き先, "約{:.1}倍", self.0)
    }
}
