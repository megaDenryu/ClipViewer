//! ライブラリのファイルの本文が読めないときに、理由を型で返す試験。
#![allow(clippy::expect_used)]

mod support;

use clip_domain::*;
use support::library_text::{本文, 読む};

#[test]
fn 壊れていない本文は読める() {
    let 項目 = 読む(&本文(&[])).expect("読める");
    assert_eq!(項目.並び().一覧().len(), 1);
    assert_eq!(項目.動画のパス().文字列(), "C:/v.mp4");
    assert_eq!(
        項目.更新した日時(),
        ライブラリの日時::紀元からのミリ秒で作成する(2).expect("日時")
    );
}

#[test]
fn 構文の誤りと形式の名前と版の違いを分けて返す() {
    assert!(matches!(
        読む("{壊れた"),
        Err(
            ライブラリのファイルの読み込みエラー::JSONとして解析できない(
                _
            )
        )
    ));
    assert_eq!(
        読む(&本文(&[("ClipViewer.library", "ModifierVideoStack")])),
        Err(
            ライブラリのファイルの読み込みエラー::形式の名前が違う(Some(
                "ModifierVideoStack".into()
            ))
        )
    );
    assert_eq!(
        読む(&本文(&[(r#""version":1"#, r#""version":2"#)])),
        Err(ライブラリのファイルの読み込みエラー::版が新しすぎる(2))
    );
    assert_eq!(
        読む(&本文(&[(r#""version":1"#, r#""version":0"#)])),
        Err(
            ライブラリのファイルの読み込みエラー::対応していない版(
                Some(0)
            )
        )
    );
    assert_eq!(
        読む(&本文(&[(r#""version":1,"#, "")])),
        Err(ライブラリのファイルの読み込みエラー::対応していない版(None))
    );
}

#[test]
fn 欠けた項目と型の違う項目は形式が不正である() {
    for 置き換え in [
        (r#""name":"一","#, ""),
        (r#""active":true"#, r#""active":"yes""#),
    ] {
        assert!(
            matches!(
                読む(&本文(&[置き換え])),
                Err(ライブラリのファイルの読み込みエラー::形式が不正(_))
            ),
            "{置き換え:?}"
        );
    }
}
