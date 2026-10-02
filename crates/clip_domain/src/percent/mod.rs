//! 幅・高さに対する百分率の型。何の幅・高さを100とするかを、基準を表す型引数で区別する。
//! クロップ範囲は元の動画を、映す矩形は重ねる画面を基準にするため、取り違えると型が通らない。全体を等分する計算は子の `division.rs` に置く。

mod division;

pub use division::等分した区間;

use std::marker::PhantomData;

/// 元の動画に対するとは、元の動画の幅または高さを100とする基準のことである。クロップ範囲とクロップ枠のドラッグの量が使う。
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum 元の動画に対する {}

/// 百分率とは、基準の幅または高さを100とした、0以上100以下の割合のことである。基準は型引数が決める。
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct 百分率<基準> {
    数値: f64,
    基準: PhantomData<基準>,
}

impl<基準> 百分率<基準> {
    /// 0%。
    pub const ゼロ: Self = Self::検査済みの数値から作る(0.0);
    /// 100%。
    pub const 全体: Self = Self::検査済みの数値から作る(100.0);

    const fn 検査済みの数値から作る(数値: f64) -> Self {
        Self {
            数値,
            基準: PhantomData,
        }
    }

    /// 数値を0以上100以下へ収めて作成する。非数は0、無限大は近い側の端にする。
    pub fn 範囲へ収める(数値: f64) -> Self {
        Self::下限と上限の間へ収める(数値, 0.0, 100.0)
    }

    /// 数値を下限以上・上限以下へ収めて作成する。下限と上限は0以上100以下であること。
    /// 注意: f64::max は非数を捨てて他方を返すため、非数は下限になる。
    pub(crate) fn 下限と上限の間へ収める(数値: f64, 下限: f64, 上限: f64) -> Self {
        Self::検査済みの数値から作る(数値.max(下限).min(上限).clamp(0.0, 100.0))
    }

    /// 百分率の数値(0〜100)を返す。表示や描画へ渡す境界で使う。
    pub fn 数値(self) -> f64 {
        self.数値
    }

    /// 0以上1以下の割合を返す。描画の切り出し座標へ渡す境界で使う。
    pub fn 割合(self) -> f64 {
        self.数値 / 100.0
    }
}

/// 百分率の差分とは、ドラッグした量を、基準の幅または高さに対する百分率で表したもののことである。
/// 正負どちらも取る。基準は型引数が決める。
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct 百分率の差分<基準> {
    数値: f64,
    基準: PhantomData<基準>,
}

/// 百分率の差分エラーとは、差分の数値が有限でないことである。
#[derive(Debug, Clone, Copy, PartialEq, thiserror::Error)]
#[error("百分率の差分が有限でない")]
pub struct 百分率の差分エラー;

impl<基準> 百分率の差分<基準> {
    /// 数値から作成する。有限でない値を拒む。
    pub fn 作成する(数値: f64) -> Result<Self, 百分率の差分エラー> {
        if 数値.is_finite() {
            Ok(Self {
                数値,
                基準: PhantomData,
            })
        } else {
            Err(百分率の差分エラー)
        }
    }

    pub(crate) fn 数値(self) -> f64 {
        self.数値
    }
}

impl<基準> Default for 百分率の差分<基準> {
    fn default() -> Self {
        Self {
            数値: 0.0,
            基準: PhantomData,
        }
    }
}
