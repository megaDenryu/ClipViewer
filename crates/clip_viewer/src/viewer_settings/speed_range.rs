//! 速度のつまみの範囲。画面のつまみと、settings.json から読んだ速度を収める範囲の両方がこの範囲を使う。

use std::ops::RangeInclusive;

use clip_domain::再生速度;

/// 速度のつまみで選べる倍率の範囲(移植元と同じ0.25〜2倍)。
pub(crate) const 速度のつまみの範囲: RangeInclusive<f64> = 0.25..=2.0;

/// 速度のつまみの刻み。
pub(crate) const 速度のつまみの刻み: f64 = 0.05;

/// 倍率をつまみの範囲へ収めた速度。範囲の外なら範囲の端に収める。非数と無限は無い(収める先が決まらないため)。
/// settings.json を手で書き換えた値を、つまみで選べない速度のまま使わないためである。
pub(crate) fn つまみの範囲へ収めた速度(倍率: f64) -> Option<再生速度> {
    if !倍率.is_finite() {
        return None;
    }
    let 収めた = 倍率.clamp(*速度のつまみの範囲.start(), *速度のつまみの範囲.end());
    再生速度::作成する(収めた).ok()
}
