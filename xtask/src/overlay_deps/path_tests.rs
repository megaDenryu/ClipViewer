//! 依存の向きの検査の試験のうち、1つのファイルの本文の試験。パスの書き方(use の木・式のパス・super・self・中のモジュール)ごとに
//! 禁じた参照を見つけ、コメントと文字列の中は見ず、解析できない入力を黙って外さずに理由を返すことを確かめる。
#![allow(clippy::expect_used, clippy::unwrap_used)]

use super::path_test_support::{
    本文の禁じた参照を調べる, 解析できる本文の禁じた参照を調べる
};

#[test]
fn crateから書いたuseと式のパスを見つける() {
    let 本文 = "use crate::state::アプリの状態;\nfn 甲() { let _ = crate::view::画面; }\nuse crate::command;";
    assert_eq!(
        本文の禁じた参照を調べる("overlay/mod.rs", 本文).unwrap(),
        [
            "crate::state::アプリの状態",
            "crate::view::画面",
            "crate::command"
        ]
    );
}

#[test]
fn 波括弧でまとめたuseの中の禁じたものだけを見つける() {
    let 本文 = "use crate::{overlay::x, view::{y, z}, state, video_feed::台帳};";
    assert_eq!(
        本文の禁じた参照を調べる("overlay/workspace.rs", 本文).unwrap(),
        ["crate::view::y", "crate::view::z", "crate::state"]
    );
}

#[test]
fn superとselfはファイルのモジュールの位置から直す() {
    assert_eq!(
        解析できる本文の禁じた参照を調べる(
            "overlay/mod.rs",
            "use super::view::画面;"
        ),
        ["crate::view::画面"]
    );
    assert!(
        解析できる本文の禁じた参照を調べる(
            "overlay/workspace.rs",
            "use super::view; use super::state::状態;"
        )
        .is_empty()
    );
    assert!(
        解析できる本文の禁じた参照を調べる(
            "overlay/view/mod.rs",
            "use super::command::操作;"
        )
        .is_empty()
    );
    assert_eq!(
        解析できる本文の禁じた参照を調べる(
            "overlay/view/mod.rs",
            "use super::super::command::応答;"
        ),
        ["crate::command::応答"]
    );
    assert!(
        解析できる本文の禁じた参照を調べる(
            "overlay/mod.rs",
            "use self::state::状態;"
        )
        .is_empty()
    );
}

#[test]
fn 中のモジュールの中ではその名前をモジュールパスに足して直す() {
    let 本文 = "mod 試験 { use super::state::状態; use super::super::view; }\nuse super::command;";
    assert_eq!(
        解析できる本文の禁じた参照を調べる("overlay/mod.rs", 本文),
        ["crate::view", "crate::command"]
    );
}

#[test]
fn コメントと文字列と文字の中は見ず_寿命の名前と外のクレートのパスは見つけない() {
    let 本文 = r###"
        // use crate::state;
        /* crate::view /* 入れ子 */ crate::command */
        const 文: &str = "crate::state::\" crate::view";
        const 生: &str = r#"crate::state "# ;
        fn 甲<'a>(x: &'a str) -> char { let _ = std::time::Instant::now(); '\'' }
        fn 乙() -> char { '"' }
    "###;
    assert!(解析できる本文の禁じた参照を調べる("overlay/mod.rs", 本文).is_empty());
}

#[test]
fn 解析できない入力は黙って外さずに理由を返す() {
    assert!(本文の禁じた参照を調べる("overlay/mod.rs", "/* 閉じない").is_err());
    assert!(本文の禁じた参照を調べる("overlay/mod.rs", "const 文: &str = \"閉じない;").is_err());
    assert!(本文の禁じた参照を調べる("overlay/mod.rs", "use crate::{state, view;").is_err());
    assert!(本文の禁じた参照を調べる("overlay/mod.rs", "use super::super::state;").is_err());
    assert!(本文の禁じた参照を調べる("overlay/mod.rs", "fn 甲() {} }").is_err());
}
