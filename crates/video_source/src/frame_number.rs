//! コマ番号とコマ数の型。

/// 動画上のコマ番号とは、一定のコマの速さでそろえた動画の先頭のコマを0番とする、コマの通し番号のことである。
/// 番号nのコマは、動画上の n÷コマの速さ 秒から次のコマまでの間に表示される。
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct 動画上のコマ番号(u32);

/// コマ数とは、コマの枚数のことである。
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct コマ数(u32);

impl 動画上のコマ番号 {
    /// 動画の先頭のコマ(0番)。
    pub const 先頭: Self = Self(0);

    /// 番号から作成する。
    pub fn 作成する(番号: u32) -> Self {
        Self(番号)
    }

    /// 番号の値を返す。
    pub fn 値(self) -> u32 {
        self.0
    }

    /// 起点のコマからこのコマまでに何コマ進んだかを求める。起点より前なら0とする。
    pub fn 起点から進んだコマ数(self, 起点: Self) -> コマ数 {
        コマ数(self.0.saturating_sub(起点.0))
    }

    /// 指定のコマ数だけ後ろのコマ番号を求める。
    pub fn 進める(self, 進むコマ数: コマ数) -> Self {
        Self(self.0.saturating_add(進むコマ数.0))
    }
}

impl コマ数 {
    /// 0コマ。
    pub const ゼロ: Self = Self(0);

    /// 枚数から作成する。
    pub fn 作成する(枚数: u32) -> Self {
        Self(枚数)
    }

    /// 枚数の値を返す。
    pub fn 値(self) -> u32 {
        self.0
    }

    /// 0コマか。
    pub fn ゼロか(self) -> bool {
        self.0 == 0
    }

    /// 1コマ増やす。
    pub(crate) fn 一つ増やす(self) -> Self {
        Self(self.0.saturating_add(1))
    }
}
