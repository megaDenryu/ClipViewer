//! ライブラリのファイルの読み込みの守りの試験。知らない項目を持つファイル・新しすぎる版・表せない日時を、
//! 黙って読み替えたり異常終了したりせずに、読めない理由として返すことを確かめる。参照: _doc/設計/ライブラリ.md 判断4
#![allow(clippy::expect_used)]

mod support;

use clip_domain::*;
use support::library_text::{本文, 読む};

#[test]
fn 新しすぎる版の理由は更新すれば開けることを示す() {
    let 理由 = 読む(&本文(&[(r#""version":1"#, r#""version":7"#)]))
        .expect_err("読めない")
        .to_string();
    assert!(理由.contains("新しい ClipViewer"), "{理由}");
    assert!(理由.contains("更新すれば開ける"), "{理由}");
}

#[test]
fn 第1版は知らない項目を許さない() {
    for 置き換え in [
        (
            r#""name":"甲","#,
            r#""name":"甲","tags":["新しい版の項目"],"#,
        ),
        (r#""name":"一","#, r#""name":"一","speed":2.0,"#),
        (r#""h":100.0}"#, r#""h":100.0,"angle":90.0}"#),
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

#[test]
fn 日時が9999年より後なら異常終了せずに読めないと返す() {
    for 大きすぎる in ["253402300800000", "18446744073709551615"] {
        assert!(
            matches!(
                読む(&本文(&[(
                    r#""createdAtMs":1"#,
                    &format!(r#""createdAtMs":{大きすぎる}"#)
                )])),
                Err(ライブラリのファイルの読み込みエラー::日時が不正(_))
            ),
            "{大きすぎる}"
        );
    }
    let 最も遅い = 読む(&本文(&[(
        r#""updatedAtMs":2"#,
        r#""updatedAtMs":253402300799999"#,
    )]))
    .expect("9999年の終わりまでは読める");
    assert!(最も遅い.更新した日時().時刻にする() > std::time::SystemTime::UNIX_EPOCH);
}
