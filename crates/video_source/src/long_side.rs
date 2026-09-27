//! 長辺の上限の型。

/// 長辺の上限とは、溜めるコマを表示用に縮めるときの、幅と高さの長い方の画素数の上限のことである。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct 長辺の上限(u32);

impl 長辺の上限 {
    /// 既定の上限(1280画素)。
    pub const 既定: Self = Self(1280);

    /// 画素数から作成する。0なら作らない。
    pub fn 作成する(画素数: u32) -> Option<Self> {
        (画素数 > 0).then_some(Self(画素数))
    }

    /// 画素数を返す。
    pub fn 画素数(self) -> u32 {
        self.0
    }
}
