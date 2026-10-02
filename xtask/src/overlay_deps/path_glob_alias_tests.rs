//! 依存の向きの検査の試験のうち、禁じたモジュールを名指さずに使える書き方(クレートルートを `*` で全部取り込む・クレートルートに `as` で別名を付ける)と、
//! 走査の終わりで閉じていない波括弧を、黙って外さずに報告することを確かめる。
#![allow(clippy::unwrap_used)]

use super::path_test_support::{
    本文の禁じた参照を調べる, 解析できる本文の禁じた参照を調べる
};

#[test]
fn クレートルートを全部取り込む書き方を見つける() {
    assert_eq!(
        解析できる本文の禁じた参照を調べる("overlay/mod.rs", "use crate::*;"),
        ["crate::*"]
    );
    assert_eq!(
        解析できる本文の禁じた参照を調べる("overlay/mod.rs", "use super::*;"),
        ["crate::*"]
    );
    assert_eq!(
        解析できる本文の禁じた参照を調べる(
            "overlay/view/mod.rs",
            "use super::super::*;"
        ),
        ["crate::*"]
    );
    assert_eq!(
        解析できる本文の禁じた参照を調べる("overlay/mod.rs", "use crate::{*};"),
        ["crate::*"]
    );
    assert_eq!(
        解析できる本文の禁じた参照を調べる(
            "overlay/mod.rs",
            "use crate::state::*;"
        ),
        ["crate::state::*"]
    );
}

#[test]
fn 重ね合わせの層の中を全部取り込む書き方は見つけない() {
    assert!(解析できる本文の禁じた参照を調べる("overlay/view/mod.rs", "use super::*;").is_empty());
    assert!(
        解析できる本文の禁じた参照を調べる(
            "overlay/mod.rs",
            "use crate::overlay::*;"
        )
        .is_empty()
    );
}

#[test]
fn クレートルートに別名を付ける書き方を見つける() {
    assert_eq!(
        解析できる本文の禁じた参照を調べる("overlay/mod.rs", "use crate as 根;"),
        ["crate as 根"]
    );
    assert_eq!(
        解析できる本文の禁じた参照を調べる(
            "overlay/mod.rs",
            "use crate::{self as 根};"
        ),
        ["crate as 根"]
    );
    assert_eq!(
        解析できる本文の禁じた参照を調べる("overlay/mod.rs", "use super as 根;"),
        ["crate as 根"]
    );
}

#[test]
fn extern_crate_selfでクレートルートに別名を付ける書き方を見つけ_外のクレートは見つけない() {
    assert_eq!(
        解析できる本文の禁じた参照を調べる(
            "overlay/mod.rs",
            "extern crate self as 根;\nuse 根::state::状態;"
        ),
        ["extern crate self as 根"]
    );
    assert!(
        解析できる本文の禁じた参照を調べる(
            "overlay/mod.rs",
            "extern crate alloc as 割り当て;\nextern \"C\" fn 甲() {}"
        )
        .is_empty()
    );
}

#[test]
fn 重ね合わせの層の中に別名を付ける書き方と式の型の変換は見つけない() {
    let 本文 = "use crate::overlay::state as 状態の層;\nfn 甲(x: u8) -> u32 { x as u32 }";
    assert!(解析できる本文の禁じた参照を調べる("overlay/mod.rs", 本文).is_empty());
}

#[test]
fn 走査の終わりで開きの波括弧が閉じていなければ解析できない理由を返す() {
    assert_eq!(
        本文の禁じた参照を調べる("overlay/mod.rs", "fn 甲() {").unwrap_err(),
        "開きの波括弧が閉じていない"
    );
    assert_eq!(
        本文の禁じた参照を調べる("overlay/mod.rs", "mod 中 { fn 甲() {} ").unwrap_err(),
        "開きの波括弧が閉じていない"
    );
}
