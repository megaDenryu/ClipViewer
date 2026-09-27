//! 壊れた settings.json の試験。JSON として読めないか形が不正なファイルは、保存の前に `settings.json.broken-<日時>-<プロセスの番号>` へ移して
//! 第1版で書き、移したことを保存の結果で返す。起動時の読み込みは理由を返すだけで、ファイルに触れない。
#![allow(clippy::expect_used)]

use super::test_support::{
    ファイルを置く, フォルダの中のファイル名, 一時の保管場所, 後片付け, 試験の日時,
};
use super::text_tests::{第1版の見本, 見本の置き場所};
use super::{アプリの設定のエラー, アプリの設定の保存の結果};

#[test]
fn 壊れた設定は読めないと返し_読むだけではファイルに触れない() {
    let (保管場所, パス) = 一時の保管場所("壊れた設定を読む");
    ファイルを置く(&パス, r#"{"ffmpegFolder":"C:\\ff"#);
    let エラー = 保管場所.ffmpegの置き場所を読む().expect_err("読めない");
    assert!(
        matches!(エラー, アプリの設定のエラー::壊れている { .. }),
        "{エラー}"
    );
    let 文 = エラー.to_string();
    assert!(文.contains(&パス.display().to_string()), "{文}");
    assert!(
        文.contains("このファイルを消すか直せば、次の起動から読める"),
        "{文}"
    );
    assert!(文.contains("FFmpeg の置き場所を保存するときには"), "{文}");
    assert_eq!(
        フォルダの中のファイル名(&パス),
        vec!["settings.json".to_string()]
    );
    後片付け(&パス);
}

#[test]
fn 途中まで書いた設定と形が不正な設定は移してから第1版で書く() {
    for (名前, 壊れた本文) in [
        ("途中まで", r#"{"ffmpegFolder":"C:\\ff"#),
        (
            "形が不正",
            r#"{"format":"ClipViewer.settings","version":1,"ffmpegFolder":3}"#,
        ),
    ] {
        let (保管場所, パス) = 一時の保管場所(名前);
        ファイルを置く(&パス, 壊れた本文);
        let 結果 = 保管場所
            .ffmpegの置き場所を保存する(&見本の置き場所(), 試験の日時())
            .expect("移して書ける");
        let アプリの設定の保存の結果::壊れたファイルを移して書いた(
            移し先,
        ) = 結果
        else {
            panic!("移したことを返していない: {結果:?}");
        };
        let ファイル名 = フォルダの中のファイル名(&パス);
        assert_eq!(ファイル名.len(), 2, "{ファイル名:?}");
        assert!(
            ファイル名[1].starts_with("settings.json.broken-"),
            "{ファイル名:?}"
        );
        assert!(移し先.to_string().ends_with(&ファイル名[1]), "{移し先}");
        let 移した本文 =
            std::fs::read_to_string(パス.with_file_name(&ファイル名[1])).expect("読める");
        assert_eq!(移した本文, 壊れた本文);
        assert_eq!(std::fs::read_to_string(&パス).expect("読める"), 第1版の見本);
        後片付け(&パス);
    }
}
