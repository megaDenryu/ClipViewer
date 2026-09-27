//! 設定ファイルの不正な形式を型付きのエラーで拒む試験。
#![allow(clippy::expect_used)]

mod support;

use clip_domain::*;
use support::連番の発行元;

fn 読み込む(
    本文: &str,
) -> Result<スタック設定, 設定ファイルの読み込みエラー> {
    let 本文 = 設定ファイルの本文::作成する(本文.to_string());
    スタック設定::本文から読み込む(&本文, &mut 連番の発行元::default())
}

fn 見出し付き(クリップの一覧: &str) -> String {
    format!(
        r#"{{"application":"ModifierVideoStack","version":"4.0","modifiers":{クリップの一覧}}}"#
    )
}

#[test]
fn 不正な形式は型付きのエラーで拒む() {
    use 設定ファイルの読み込みエラー as エラー;
    assert!(matches!(
        読み込む("{"),
        Err(エラー::JSONとして解析できない(_))
    ));
    assert!(matches!(読み込む("[]"), Err(エラー::形式が不正(_))));
    assert!(matches!(
        読み込む(r#"{"application":"Other","modifiers":[]}"#),
        Err(エラー::アプリケーション名が違う(_))
    ));
    let 未知の版 = r#"{"application":"ModifierVideoStack","version":"5.0","modifiers":[]}"#;
    assert!(matches!(読み込む(未知の版), Err(エラー::対応していない版(版)) if 版 == "5.0"));
    assert!(matches!(
        読み込む(r#"{"application":"ModifierVideoStack"}"#),
        Err(エラー::形式が不正(_))
    ));
    assert!(matches!(
        読み込む(&見出し付き(r#"[{"triggerEvent":"Foo"}]"#)),
        Err(エラー::形式が不正(_))
    ));
    assert!(matches!(
        読み込む(&見出し付き(r#"[{"start":"abc"}]"#)),
        Err(エラー::形式が不正(_))
    ));
    let 繰り返しが不正 = 読み込む(&見出し付き(r#"[{},{"repeat":"forever"}]"#));
    assert!(matches!(
        繰り返しが不正,
        Err(エラー::クリップが不正 {
            番号: 1,
            理由: クリップの値の不備::繰り返し(_)
        })
    ));
    let 区間が逆 = 読み込む(&見出し付き(r#"[{"start":6,"end":5}]"#));
    assert!(matches!(
        区間が逆,
        Err(エラー::クリップが不正 {
            番号: 0,
            理由: クリップの値の不備::区間(_)
        })
    ));
    let 負の時刻 = 読み込む(&見出し付き(r#"[{"start":-1}]"#));
    assert!(matches!(
        負の時刻,
        Err(エラー::クリップが不正 {
            理由: クリップの値の不備::時刻(_),
            ..
        })
    ));
    let 重複 = 読み込む(&見出し付き(r#"[{"id":"x"},{"id":"x"}]"#));
    assert!(matches!(重複, Err(エラー::スタックを作れない(_))));
}

#[test]
fn 読めなかった箇所は1から数えた行と列を持つ() {
    let 構文の誤り = 読み込む("{\n  \"application\": ,\n}");
    let Err(設定ファイルの読み込みエラー::JSONとして解析できない(不備)) = 構文の誤り
    else {
        panic!("構文の誤りのはず: {構文の誤り:?}");
    };
    assert_eq!((不備.行, 不備.列), (2, 18));
    let 型の誤り = 読み込む(
        "{\"application\":\"ModifierVideoStack\",\n\"modifiers\":[{\"start\":\"abc\"}]}",
    );
    let Err(設定ファイルの読み込みエラー::形式が不正(不備)) = 型の誤り
    else {
        panic!("形式の誤りのはず: {型の誤り:?}");
    };
    assert_eq!((不備.行, 不備.列), (2, 27));
}
