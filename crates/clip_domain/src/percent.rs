//! 幅・高さに対する百分率の型。クロップ範囲(元の動画が基準)と映す矩形(重ねる画面が基準)の座標と大きさに使う。

/// 百分率とは、基準の幅または高さを100とした、0以上100以下の割合のことである。
/// 基準は使う側の型が決める。クロップ範囲では元の動画、映す矩形では重ねる画面である。
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct 百分率(f64);

impl 百分率 {
    /// 0%。
    pub const ゼロ: Self = Self(0.0);
    /// 100%。
    pub const 全体: Self = Self(100.0);

    /// 数値を0以上100以下へ収めて作成する。非数は0、無限大は近い側の端にする。
    pub fn 範囲へ収める(数値: f64) -> Self {
        Self::下限と上限の間へ収める(数値, 0.0, 100.0)
    }

    /// 数値を下限以上・上限以下へ収めて作成する。下限と上限は0以上100以下であること。
    /// 注意: f64::max は非数を捨てて他方を返すため、非数は下限になる。
    pub(crate) fn 下限と上限の間へ収める(数値: f64, 下限: f64, 上限: f64) -> Self {
        Self(数値.max(下限).min(上限).clamp(0.0, 100.0))
    }

    /// 百分率の数値(0〜100)を返す。表示や描画へ渡す境界で使う。
    pub fn 数値(self) -> f64 {
        self.0
    }

    /// 0以上1以下の割合を返す。描画の切り出し座標へ渡す境界で使う。
    pub fn 割合(self) -> f64 {
        self.0 / 100.0
    }
}

/// 百分率の差分とは、クロップ枠をドラッグした量を、画像の幅または高さに対する百分率で表したもののことである。
/// 正負どちらも取る。
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
#[repr(transparent)]
pub struct 百分率の差分(f64);

/// 百分率の差分エラーとは、差分の数値が有限でないことである。
#[derive(Debug, Clone, Copy, PartialEq, thiserror::Error)]
#[error("百分率の差分が有限でない")]
pub struct 百分率の差分エラー;

impl 百分率の差分 {
    /// 数値から作成する。有限でない値を拒む。
    pub fn 作成する(数値: f64) -> Result<Self, 百分率の差分エラー> {
        if 数値.is_finite() {
            Ok(Self(数値))
        } else {
            Err(百分率の差分エラー)
        }
    }

    pub(crate) fn 数値(self) -> f64 {
        self.0
    }
}
