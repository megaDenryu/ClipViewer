//! クリップの繰り返しの設定と、繰り返しの何周目かを表す型。

use crate::duration::時間の長さ;
use crate::rounding::小数を切り捨てた整数;

/// リピート回数とは、クリップの区間を続けて再生する回数のことであり、1以上100以下である。
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct リピート回数(u8);

/// リピート回数エラーとは、回数が1以上100以下の範囲に無いことである。
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[error("リピート回数は1以上100以下でなければならない: {0}")]
pub struct リピート回数エラー(pub u32);

impl リピート回数 {
    /// 1回。
    pub const 一回: Self = Self(1);
    const 上限: u8 = 100;

    /// 回数から作成する。範囲外の回数を拒む。
    pub fn 作成する(回数: u32) -> Result<Self, リピート回数エラー> {
        u8::try_from(回数)
            .ok()
            .filter(|回数| (1..=Self::上限).contains(回数))
            .map(Self)
            .ok_or(リピート回数エラー(回数))
    }

    /// 小数を含む数値を切り捨て、1以上100以下へ収めて作成する。
    /// 移植元の `回数指定(回数)` と同じ規則であり、設定ファイルの読み込みで使う。
    pub fn 切り捨てて範囲へ収める(数値: f64) -> Self {
        let 回数 = 小数を切り捨てた整数(数値).clamp(1, u32::from(Self::上限));
        Self(u8::try_from(回数).unwrap_or(Self::上限))
    }

    /// 回数を返す。表示や設定ファイルへ渡す境界で使う。
    pub fn 回数(self) -> u8 {
        self.0
    }

    /// 1回増やす。上限では増えない。
    pub fn 増やす(self) -> Self {
        Self(self.0.saturating_add(1).min(Self::上限))
    }

    /// 1回減らす。1回より少なくはならない。
    pub fn 減らす(self) -> Self {
        Self(self.0.saturating_sub(1).max(1))
    }

    /// 一周の長さをこの回数だけ繰り返した長さを求める。
    pub fn リピートした長さ(self, 一周: 時間の長さ) -> 時間の長さ {
        時間の長さ::検査済みの秒数から作る(一周.秒数() * f64::from(self.0))
    }

    /// この周回が最後の周回か。
    pub fn 最後の周回か(self, 周回: 周回番号) -> bool {
        u16::from(周回.0) + 1 == u16::from(self.0)
    }
}

/// リピート設定とは、クリップの区間を何回繰り返してから次へ進むかの設定のことである。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum リピート設定 {
    /// 指定の回数だけ繰り返す。
    回数指定(リピート回数),
    /// 利用者の操作で次へ進むまで繰り返す。
    無限ループ,
}

/// 周回番号とは、クリップの区間を繰り返している何周目かを0から数えた番号のことである。
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct 周回番号(u8);

impl 周回番号 {
    /// 最初の周回(0周目)。
    pub const 最初: Self = Self(0);

    /// 経過÷一周の商を切り捨てて周回番号にする。255周を超える商は255周に収める。
    /// 注意: 回数指定の占有時間は最大100周分なので、通常の探索で上限に届くことはない。
    pub(crate) fn 商から求める(商: f64) -> Self {
        Self(u8::try_from(小数を切り捨てた整数(商)).unwrap_or(u8::MAX))
    }

    /// 1から数えた周回の順番を返す。表示の「(2/3)」の2に当たる。
    pub fn 一から数えた順番(self) -> u16 {
        u16::from(self.0) + 1
    }
}
