//! キャッシュの容量。

/// キャッシュの容量とは、キャッシュのフォルダに置くサムネイルのファイルの大きさの合計の上限(バイト数)のことである。
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct キャッシュの容量(u64);

impl キャッシュの容量 {
    /// 既定の上限(50MB)。1枚は数KBなので、約1万件を持てる。
    pub const 既定: Self = Self(50_000_000);

    /// バイト数から作る。
    pub fn バイト数から作る(バイト数: u64) -> Self {
        Self(バイト数)
    }

    /// 合計のバイト数がこの上限を超えるか。
    pub(crate) fn 超えるか(self, 合計のバイト数: u64) -> bool {
        合計のバイト数 > self.0
    }
}
