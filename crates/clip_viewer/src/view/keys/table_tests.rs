//! キーの割り当ての表の試験。同じキーの組を2行に置いていないことを確かめる。
//! SengenEgui のキー操作は最初に一致した行が押下を消費するため、2行目は決して発しない。

use std::collections::HashSet;

use super::table::キーの割り当ての一覧;

#[test]
fn 同じキーの組を2行に置いていない() {
    let 一覧 = キーの割り当ての一覧();
    let mut 見た組 = HashSet::new();
    for 割り当て in &一覧 {
        assert!(見た組.insert(割り当て.組), "{} が2行ある", 割り当て.表記);
    }
}
