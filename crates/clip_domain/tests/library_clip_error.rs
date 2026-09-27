//! ライブラリのファイルのクリップの値と識別子と名前が成立しないときに、理由を型で返す試験。
#![allow(clippy::expect_used)]

mod support;

use clip_domain::*;
use support::library_text::{本文, 読む};

#[test]
fn 識別子と名前とクリップの値の不備を返す() {
    assert!(matches!(
        読む(&本文(&[("stack-1-1", "Stack/1")])),
        Err(ライブラリのファイルの読み込みエラー::識別子が不正(_))
    ));
    assert!(matches!(
        読む(&本文(&[(r#""name":"甲""#, r#""name":" ""#)])),
        Err(ライブラリのファイルの読み込みエラー::名前が空(_))
    ));
    let クリップの不備 = |置き換え: (&str, &str)| match 読む(&本文(&[置き換え])) {
        Err(ライブラリのファイルの読み込みエラー::クリップが不正 {
            番号: 0,
            理由,
        }) => 理由,
        その他 => panic!("クリップの不備でない: {その他:?}"),
    };
    assert!(matches!(
        クリップの不備((r#""id":"c1""#, r#""id":"""#)),
        ライブラリのクリップの不備::識別子(_)
    ));
    assert!(matches!(
        クリップの不備((r#""start":1.0"#, r#""start":3.0"#)),
        ライブラリのクリップの不備::区間(_)
    ));
    assert!(matches!(
        クリップの不備((r#""start":1.0"#, r#""start":-1.0"#)),
        ライブラリのクリップの不備::時刻(_)
    ));
    assert!(matches!(
        クリップの不備((r#""repeat":2"#, r#""repeat":0"#)),
        ライブラリのクリップの不備::繰り返しの回数(_)
    ));
    assert!(matches!(
        クリップの不備((r#""repeat":2"#, r#""repeat":"forever""#)),
        ライブラリのクリップの不備::繰り返しの表記(_)
    ));
}

#[test]
fn クリップの識別子が重複していれば並びを作れない() {
    let 二つ = 本文(&[]).replace(
        r#""trigger":"Space"}]"#,
        r#""trigger":"Space"},{"id":"c1","name":"二","active":true,"start":1.0,"end":2.0,"repeat":1,"crop":{"x":0.0,"y":0.0,"w":100.0,"h":100.0},"trigger":"none"}]"#,
    );
    assert!(matches!(
        読む(&二つ),
        Err(ライブラリのファイルの読み込みエラー::並びを作れない(_))
    ));
}
