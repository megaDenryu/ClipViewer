//! バイト数と、溜めるコマに使ってよいメモリの上限の型。

use crate::frame_number::コマ数;

/// バイト数とは、メモリ上の大きさをバイトの数で表したもののことである。
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
#[repr(transparent)]
pub struct バイト数(u64);

impl バイト数 {
    /// 0バイト。
    pub const ゼロ: Self = Self(0);

    /// バイトの数から作成する。
    pub const fn 作成する(値: u64) -> Self {
        Self(値)
    }

    /// バイトの数を返す。表示の境界で使う。
    pub fn 値(self) -> u64 {
        self.0
    }

    /// 2つの大きさを足す。上限を超えたら飽和させる。
    pub fn 足す(self, 他方: Self) -> Self {
        Self(self.0.saturating_add(他方.0))
    }

    /// 大きさを差し引く。負になるなら0とする。
    pub fn 差し引く(self, 他方: Self) -> Self {
        Self(self.0.saturating_sub(他方.0))
    }

    /// 1コマの大きさをコマ数倍して、そのコマ数のコマが占める大きさを求める。上限を超えたら飽和させる。
    pub(crate) fn コマ数倍にする(self, 枚数: コマ数) -> Self {
        Self(self.0.saturating_mul(u64::from(枚数.値())))
    }
}

/// メモリの上限とは、コマの倉庫が溜めるコマに使ってよいメモリの大きさの上限のことである。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct メモリの上限(バイト数);

impl メモリの上限 {
    /// 既定の上限(2,000,000,000バイト)。1280×720・30コマ/秒で約18秒分が収まる。
    pub const 既定: Self = Self(バイト数(2_000_000_000));

    /// 大きさから作成する。
    pub fn 作成する(大きさ: バイト数) -> Self {
        Self(大きさ)
    }

    /// 上限の大きさを返す。
    pub fn 大きさ(self) -> バイト数 {
        self.0
    }

    /// その大きさが上限に収まるか。
    pub fn 収まるか(self, 大きさ: バイト数) -> bool {
        大きさ <= self.0
    }
}
