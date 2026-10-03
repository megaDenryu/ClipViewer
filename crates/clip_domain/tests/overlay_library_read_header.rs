//! 重ね合わせのファイルの読み込みの試験。見出しの誤り・知らない項目と欠けた項目・縦横比の表記・識別子と名前と日時の不備を、型付きのエラーで拒むことを確かめる。
#![allow(clippy::expect_used)]

mod support;

use clip_domain::*;
use support::overlay_library_text::{本文, 読む};

#[test]
fn 壊していない本文は読める() {
    let 読んだ = 読む(&本文(&[])).expect("読める");
    assert_eq!(読んだ.識別子().文字列(), "overlay-1-1");
    assert_eq!(
        読んだ.重ね合わせ().縦横比(),
        重ねる画面の縦横比::ワイド16対9
    );
}

#[test]
fn 見出しが違う本文を拒む() {
    assert!(matches!(
        読む(&本文(&[("ClipViewer.overlay", "ClipViewer.library")])),
        Err(
            重ね合わせのファイルの読み込みエラー::形式の名前が違う(
                Some(_)
            )
        )
    ));
    assert_eq!(
        読む(&本文(&[(r#""version":1"#, r#""version":2"#)])),
        Err(重ね合わせのファイルの読み込みエラー::版が新しすぎる(2))
    );
    assert_eq!(
        読む(&本文(&[(r#""version":1"#, r#""version":0"#)])),
        Err(
            重ね合わせのファイルの読み込みエラー::対応していない版(
                Some(0)
            )
        )
    );
    assert!(matches!(
        読む("{"),
        Err(
            重ね合わせのファイルの読み込みエラー::JSONとして解析できない(
                _
            )
        )
    ));
}

#[test]
fn 知らない項目と欠けた項目を拒む() {
    for 壊し方 in [
        (r#""muted":false"#, r#""muted":false,"speed":1.0"#),
        (r#""rows":"#, r#""loop":true,"rows":"#),
        (r#""volume":1.0,"#, ""),
        (r#""aspect":"16:9","#, ""),
    ] {
        assert!(
            matches!(
                読む(&本文(&[壊し方])),
                Err(重ね合わせのファイルの読み込みエラー::形式が不正(_))
            ),
            "{壊し方:?}"
        );
    }
}

#[test]
fn 縦横比のクロップ枠に合わせると知らない表記を拒む() {
    assert_eq!(
        読む(&本文(&[(r#""aspect":"16:9""#, r#""aspect":"crop""#)])),
        Err(
            重ね合わせのファイルの読み込みエラー::縦横比が不正(
                重ねる画面の縦横比エラー
            )
        )
    );
    assert!(matches!(
        読む(&本文(&[(r#""aspect":"16:9""#, r#""aspect":"2:1""#)])),
        Err(重ね合わせのファイルの読み込みエラー::形式が不正(_))
    ));
}

#[test]
fn 識別子と名前と日時が成立しない本文を拒む() {
    assert!(matches!(
        読む(&本文(&[("overlay-1-1", "Overlay-1-1")])),
        Err(重ね合わせのファイルの読み込みエラー::識別子が不正(_))
    ));
    assert!(matches!(
        読む(&本文(&[(r#""name":"甲""#, r#""name":" ""#)])),
        Err(重ね合わせのファイルの読み込みエラー::名前が空(_))
    ));
    assert!(matches!(
        読む(&本文(&[(
            r#""updatedAtMs":2"#,
            r#""updatedAtMs":253402300800000"#
        )])),
        Err(重ね合わせのファイルの読み込みエラー::日時が不正(_))
    ));
}
