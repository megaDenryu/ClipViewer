//! 画素の寸法と縦横比の型。出力のアスペクト比の計算に使う。

/// 画素の寸法とは、画像の幅と高さを画素の数で表したもののことである。切り出し後の寸法は小数を取りうる。
/// 不変条件: 幅と高さは0以上の有限の数である。
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct 画素の寸法 {
    幅: f64,
    高さ: f64,
}

/// 画素の寸法エラーとは、幅または高さが0以上の有限の数でないことである。
#[derive(Debug, Clone, Copy, PartialEq, thiserror::Error)]
#[error("画素の寸法は0以上の有限の数でなければならない: 幅{幅} 高さ{高さ}")]
pub struct 画素の寸法エラー {
    /// 渡された幅。
    pub 幅: f64,
    /// 渡された高さ。
    pub 高さ: f64,
}

impl 画素の寸法 {
    /// 幅と高さから作成する。
    pub fn 作成する(幅: f64, 高さ: f64) -> Result<Self, 画素の寸法エラー> {
        let 成立するか = |値: f64| 値.is_finite() && 値 >= 0.0;
        if 成立するか(幅) && 成立するか(高さ) {
            Ok(Self { 幅, 高さ })
        } else {
            Err(画素の寸法エラー { 幅, 高さ })
        }
    }

    /// 幅の画素数。
    pub fn 幅(&self) -> f64 {
        self.幅
    }

    /// 高さの画素数。
    pub fn 高さ(&self) -> f64 {
        self.高さ
    }

    /// 幅と高さに0以上1以下の割合を掛けた寸法を求める。
    pub(crate) fn 割合で縮める(&self, 幅の割合: f64, 高さの割合: f64) -> Self {
        Self {
            幅: self.幅 * 幅の割合,
            高さ: self.高さ * 高さの割合,
        }
    }

    /// 縦横比を求める。高さが0なら求められない。
    pub fn 縦横比(&self) -> Option<縦横比> {
        (self.高さ > 0.0).then(|| 縦横比(self.幅 / self.高さ))
    }
}

/// 縦横比とは、幅を高さで割った値のことである。
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct 縦横比(f64);

impl 縦横比 {
    /// 幅と高さの整数比から求める。
    pub(crate) fn 整数比から求める(横: u8, 縦: u8) -> Self {
        Self(f64::from(横) / f64::from(縦))
    }

    /// 幅÷高さの値を返す。描画の寸法の計算へ渡す境界で使う。
    pub fn 値(self) -> f64 {
        self.0
    }
}
