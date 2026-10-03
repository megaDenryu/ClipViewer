//! 縦横比を保った寸法の計算。スタックの作業場の出力の寸法(view/output/size.rs)とプレビューの大きさ(view/sidebar/preview.rs)と、
//! 重ね合わせの作業場の重ねる画面の寸法(overlay/view/screen.rs)が使う。
//! 計算はドメインの縦横比と同じ倍精度の小数で行い、結果だけを論理画素へ包む。参照: _doc/設計/画面.md 判断6・判断8

use clip_domain::縦横比;
use sengen_egui::{縦横の論理画素, 論理画素};

const 既定の縦横比: f64 = 16.0 / 9.0; // 縦横比を求められないときに使う比

/// 描く縦横比とは、寸法の計算に使う、幅を高さで割った正の有限な値のことである。
#[derive(Debug, Clone, Copy)]
pub(crate) struct 描く縦横比(f64);

impl 描く縦横比 {
    /// 動画の縦横比から求める。縦横比が無いか0以下なら16:9とみなす(移植元は割り算の結果をそのまま使っていた)。
    pub(crate) fn 求める(縦横比: Option<縦横比>) -> Self {
        let 値 = 縦横比
            .map(縦横比::値)
            .filter(|比| *比 > 0.0 && 比.is_finite())
            .unwrap_or(既定の縦横比);
        Self(値)
    }

    pub(crate) fn 高さに合わせた幅(self, 高さ: f64) -> f64 {
        高さ * self.0
    }

    pub(crate) fn 幅に合わせた高さ(self, 幅: f64) -> f64 {
        幅 / self.0
    }
}

/// 丸める前の論理画素の寸法とは、寸法の計算の途中で使う、論理画素を単位とする幅と高さの組を、論理画素の組へ丸める前の倍精度の小数で表したもののことである。
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct 丸める前の論理画素の寸法 {
    幅: f64,
    高さ: f64,
}

impl 丸める前の論理画素の寸法 {
    pub(crate) const fn 生成する(幅: f64, 高さ: f64) -> Self {
        Self { 幅, 高さ }
    }

    pub(crate) fn 論理画素の組から作る(組: 縦横の論理画素) -> Self {
        Self {
            幅: 組.横.倍精度の小数にする(),
            高さ: 組.縦.倍精度の小数にする(),
        }
    }

    /// 縦横比を保って、この幅と高さの中に収まる最も大きい寸法。
    pub(crate) fn 縦横比を保って収める(self, 比: 描く縦横比) -> Self {
        let 高さに合わせた幅 = 比.高さに合わせた幅(self.高さ);
        if 高さに合わせた幅 <= self.幅 {
            self.高さに合わせる(比)
        } else {
            self.幅に合わせる(比)
        }
    }

    /// 縦横比を保って、この幅と高さを覆う最も小さい寸法。
    pub(crate) fn 縦横比を保って覆う(self, 比: 描く縦横比) -> Self {
        let 高さに合わせた幅 = 比.高さに合わせた幅(self.高さ);
        if 高さに合わせた幅 >= self.幅 {
            self.高さに合わせる(比)
        } else {
            self.幅に合わせる(比)
        }
    }

    /// 幅はそのままにし、高さを縦横比に合わせる。
    pub(crate) fn 幅に合わせる(self, 比: 描く縦横比) -> Self {
        Self {
            幅: self.幅,
            高さ: 比.幅に合わせた高さ(self.幅),
        }
    }

    /// 高さはそのままにし、幅を縦横比に合わせる。
    pub(crate) fn 高さに合わせる(self, 比: 描く縦横比) -> Self {
        Self {
            幅: 比.高さに合わせた幅(self.高さ),
            高さ: self.高さ,
        }
    }

    /// 幅と高さのそれぞれから、相手の幅と高さを差し引く。
    pub(crate) fn 差し引く(self, 相手: Self) -> Self {
        Self::生成する(self.幅 - 相手.幅, self.高さ - 相手.高さ)
    }

    /// 幅と高さのそれぞれを、下限の幅と高さ以上へ揃える。
    pub(crate) fn 下限へ揃える(self, 下限: Self) -> Self {
        Self::生成する(self.幅.max(下限.幅), self.高さ.max(下限.高さ))
    }

    /// 幅と高さの両方に同じ比を掛ける。
    pub(crate) fn 比を掛ける(self, 比: f64) -> Self {
        Self::生成する(self.幅 * 比, self.高さ * 比)
    }

    pub(crate) fn 四捨五入して論理画素の組にする(self) -> 縦横の論理画素 {
        論理画素の組にする(self.幅.round(), self.高さ.round())
    }

    pub(crate) fn 切り捨てて論理画素の組にする(self) -> 縦横の論理画素 {
        論理画素の組にする(self.幅.floor(), self.高さ.floor())
    }
}

fn 論理画素の組にする(幅: f64, 高さ: f64) -> 縦横の論理画素 {
    縦横の論理画素::生成する(
        論理画素::倍精度の小数から生成する(幅),
        論理画素::倍精度の小数から生成する(高さ),
    )
}
