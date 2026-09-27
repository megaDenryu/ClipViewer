//! 標本の数と、動画上の標本位置の型。
//! 位置と数は u32 で持つ。48kHz で約24時間、192kHz で約6時間まで数えられ、動画1本の長さには足りる。

/// 標本数とは、標本の個数のことである。
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
#[repr(transparent)]
pub struct 標本数(u32);

/// 動画上の標本位置とは、動画の先頭の標本を0番とする、出力装置のサンプリング周波数で数えた標本の通し番号のことである。
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
#[repr(transparent)]
pub struct 動画上の標本位置(u32);

impl 標本数 {
    /// 0個。
    pub const ゼロ: Self = Self(0);

    /// 個数から作成する。
    pub fn 作成する(個数: u32) -> Self {
        Self(個数)
    }

    /// 並びの長さから作成する。u32 に収まらない長さは最大に飽和させる。
    pub fn 並びの長さから作る(長さ: usize) -> Self {
        Self(u32::try_from(長さ).unwrap_or(u32::MAX))
    }

    /// 個数の値を返す。
    pub fn 値(self) -> u32 {
        self.0
    }

    /// 小数の標本数として返す。読み位置との計算に使う。
    pub fn 小数の値(self) -> f64 {
        f64::from(self.0)
    }

    /// 2つの数を足す。上限を超えたら飽和させる。
    pub fn 足す(self, 他方: Self) -> Self {
        Self(self.0.saturating_add(他方.0))
    }

    /// 並びの添字として使う長さ。
    pub fn 並びの長さ(self) -> usize {
        usize::try_from(self.0).unwrap_or(usize::MAX)
    }
}

impl 動画上の標本位置 {
    /// 動画の先頭(0番)。
    pub const 先頭: Self = Self(0);

    /// 番号から作成する。
    pub fn 作成する(番号: u32) -> Self {
        Self(番号)
    }

    /// 番号の値を返す。
    pub fn 値(self) -> u32 {
        self.0
    }

    /// 指定の数だけ後ろの位置を求める。
    pub fn 進める(self, 数: 標本数) -> Self {
        Self(self.0.saturating_add(数.値()))
    }

    /// 指定の数だけ前の位置を求める。先頭より前には戻らない。
    pub fn 戻す(self, 数: 標本数) -> Self {
        Self(self.0.saturating_sub(数.値()))
    }

    /// 起点からこの位置までの標本数を求める。起点より前なら0とする。
    pub fn 起点からの標本数(self, 起点: Self) -> 標本数 {
        標本数::作成する(self.0.saturating_sub(起点.0))
    }

    /// 小数の位置として返す。読み位置との計算に使う。
    pub fn 小数の値(self) -> f64 {
        f64::from(self.0)
    }
}
