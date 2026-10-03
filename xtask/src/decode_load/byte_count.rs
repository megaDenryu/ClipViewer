//! バイト数の型。ffmpeg の標準出力から読んだ量と、1コマの大きさを表す。

use std::fmt;

use super::frame_count::コマ数;

/// バイト数とは、データの大きさをバイトの数で表したもののことである。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct バイト数(u64);

impl バイト数 {
    /// 何も読んでいない大きさ。
    pub const ゼロ: Self = Self(0);

    /// 幅と高さと1画素のバイト数から、1コマの大きさを求める。
    pub fn 画素の数から求める(
        幅: u32, 高さ: u32, 一画素のバイト数: u32
    ) -> Self {
        Self(u64::from(幅) * u64::from(高さ) * u64::from(一画素のバイト数))
    }

    /// 1回の読み出しで読んだ大きさを足す。
    pub fn 読んだ分を足す(self, 読んだ: usize) -> Self {
        Self(
            self.0
                .saturating_add(u64::try_from(読んだ).unwrap_or(u64::MAX)),
        )
    }

    /// 1コマの大きさで割り切って、何コマ分かを求める。割り切れないとき・0コマのとき・数え切れないほど多いときは無い。
    pub fn 割り切ってコマ数を求める(self, 一コマ: Self) -> Option<コマ数> {
        if 一コマ.0 == 0 || !self.0.is_multiple_of(一コマ.0) {
            return None;
        }
        u32::try_from(self.0 / 一コマ.0)
            .ok()
            .and_then(コマ数::作成する)
    }
}

impl fmt::Display for バイト数 {
    fn fmt(&self, 書き先: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(書き先, "{}バイト", self.0)
    }
}
