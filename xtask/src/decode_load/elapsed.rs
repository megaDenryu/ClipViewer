//! 読み切る時間の型。測った時間(std の Duration)を受け取る窓口である。

use std::fmt;
use std::time::Duration;

/// 読み切る時間とは、同時に起動した ffmpeg の1本目を起動する直前から、すべてが読み終わるまでの時間(0より長い)のことである。
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct 読み切る時間(Duration);

impl 読み切る時間 {
    /// 測った時間から作成する。0なら速さを求められないため作らない。
    pub fn 作成する(時間: Duration) -> Option<Self> {
        (!時間.is_zero()).then_some(Self(時間))
    }

    /// 速さを求めるときの秒数。
    pub fn 秒数(self) -> f64 {
        self.0.as_secs_f64()
    }
}

impl fmt::Display for 読み切る時間 {
    fn fmt(&self, 書き先: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(書き先, "{:.1}秒", self.秒数())
    }
}
