//! 標本の差の型。2つの位置の間の、標本単位の符号付きの長さである。

use std::ops::{Add, Neg, Sub};

use clip_domain::再生速度;

use crate::position::標本数;

/// 標本の差とは、2つの小数の標本位置の間の、標本単位の符号付きの長さのことである。区間の終わりまでの残り・映像とのずれ・
/// 重ねる長さのように、位置でなく長さを表す値に使う。
/// 不変条件: 有限の数である。
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
#[repr(transparent)]
pub struct 標本の差(f64);

impl 標本の差 {
    /// 0。
    pub const ゼロ: Self = Self(0.0);

    /// 小数から作成する。非数と無限大は0に収める。
    pub fn 収めて作る(値: f64) -> Self {
        if 値.is_finite() {
            Self(値)
        } else {
            Self(0.0)
        }
    }

    /// 標本数を長さとして作る。
    pub fn 標本数から作る(数: 標本数) -> Self {
        Self(数.小数の値())
    }

    /// 値。
    pub fn 値(self) -> f64 {
        self.0
    }

    /// 符号を除いた大きさ。
    pub fn 大きさ(self) -> Self {
        Self(self.0.abs())
    }

    /// 小さい方。
    pub fn 小さい方(self, 他方: Self) -> Self {
        Self(self.0.min(他方.0))
    }

    /// 全体に対するこの長さの割合。全体が0なら0である。重ね合わせの重みに使う。
    pub fn 全体に対する割合(self, 全体: Self) -> f64 {
        if 全体.0 == 0.0 {
            0.0
        } else {
            self.0 / 全体.0
        }
    }

    /// 速度の倍率を掛けた長さ。実時間の経過を、その速度で進む読み位置の長さへ換算するときに使う。
    pub fn 速度を掛ける(self, 速度: 再生速度) -> Self {
        Self::収めて作る(self.0 * 速度.倍率())
    }

    /// 半分の長さ。
    pub fn 半分(self) -> Self {
        Self(self.0 / 2.0)
    }

    /// 全体の長さで割った余り(0以上、全体未満)。全体が0以下なら0である。
    pub fn 全体で割った余り(self, 全体: Self) -> Self {
        if 全体.0 <= 0.0 {
            Self(0.0)
        } else {
            Self(self.0.rem_euclid(全体.0))
        }
    }
}

impl Add for 標本の差 {
    type Output = Self;
    fn add(self, 他方: Self) -> Self {
        Self(self.0 + 他方.0)
    }
}

impl Sub for 標本の差 {
    type Output = Self;
    fn sub(self, 他方: Self) -> Self {
        Self(self.0 - 他方.0)
    }
}

impl Neg for 標本の差 {
    type Output = Self;
    fn neg(self) -> Self {
        Self(-self.0)
    }
}
