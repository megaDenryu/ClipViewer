//! 速度のつまみの範囲へ収める規則の試験。

use super::つまみの範囲へ収めた速度;

#[test]
fn 範囲の外は端へ収め_範囲の中はそのまま_非数は無い() {
    let 倍率 = |値| つまみの範囲へ収めた速度(値).map(|速度| 速度.倍率());
    assert_eq!(倍率(0.1), Some(0.25));
    assert_eq!(倍率(-3.0), Some(0.25));
    assert_eq!(倍率(5.0), Some(2.0));
    assert_eq!(倍率(1.3), Some(1.3));
    assert_eq!(倍率(f64::NAN), None);
    assert_eq!(倍率(f64::INFINITY), None);
}
