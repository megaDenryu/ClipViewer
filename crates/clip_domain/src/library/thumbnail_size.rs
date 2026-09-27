//! サムネイルの大きさ。スタックの顔を縦横比を保って収める枠の画素数であり、撮り方の一部として撮る画像を決める。
//! 参照: _doc/設計/ライブラリ.md 判断10

use std::num::NonZeroU32;

/// サムネイルの大きさとは、スタックの顔を縦横比を保ったまま収める枠の、幅と高さの画素数のことである。
/// 顔はこの枠に接するまで拡大か縮小する(小さいクロップは拡大する)。幅と高さはどちらも1以上である。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct サムネイルの大きさ {
    幅: NonZeroU32,
    高さ: NonZeroU32,
}

impl サムネイルの大きさ {
    /// 幅と高さの画素数から作る。どちらかが0なら作らない。定数の式でも使える。
    pub const fn 作成する(幅: u32, 高さ: u32) -> Option<Self> {
        match (NonZeroU32::new(幅), NonZeroU32::new(高さ)) {
            (Some(幅), Some(高さ)) => Some(Self { 幅, 高さ }),
            _ => None,
        }
    }

    /// 枠の幅の画素数。
    pub fn 幅(self) -> u32 {
        self.幅.get()
    }

    /// 枠の高さの画素数。
    pub fn 高さ(self) -> u32 {
        self.高さ.get()
    }
}
