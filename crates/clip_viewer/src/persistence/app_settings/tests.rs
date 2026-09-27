//! settings.json の保存の流れの試験。第0版を第1版で書き直すこと、同じ版の知らない項目を消さないこと、
//! 読めない版のファイルを書き換えないことを、一時フォルダの settings.json で確かめる。
#![allow(clippy::expect_used)]

use std::path::PathBuf;

use video_source::{FFmpegの置き場所の設定, FFmpegを置いたフォルダ};

use super::test_support::{
    ファイルを置く, フォルダの中のファイル名, 一時の保管場所, 後片付け, 試験の日時, 読み直す,
};
use super::text_tests::{第1版の見本, 見本の置き場所};

#[test]
fn 版の無い第0版を読み_保存すると第1版で書く() {
    let (保管場所, パス) = 一時の保管場所("第0版");
    ファイルを置く(&パス, r#"{"ffmpegFolder":"C:\\ffmpeg\\bin"}"#);
    let 第0版のフォルダ =
        FFmpegを置いたフォルダ::作成する(PathBuf::from(r"C:\ffmpeg\bin"));
    assert_eq!(
        保管場所.ffmpegの置き場所を読む().expect("読める"),
        FFmpegの置き場所の設定::設定済み(第0版のフォルダ)
    );
    保管場所
        .ffmpegの置き場所を保存する(&見本の置き場所(), 試験の日時())
        .expect("保存できる");
    assert_eq!(std::fs::read_to_string(&パス).expect("読める"), 第1版の見本);
    assert_eq!(
        フォルダの中のファイル名(&パス),
        vec!["settings.json".to_string()]
    );
    後片付け(&パス);
}

#[test]
fn 同じ版の知らない項目は保存しても消さない() {
    let (保管場所, パス) = 一時の保管場所("知らない項目");
    ファイルを置く(
        &パス,
        r#"{"format":"ClipViewer.settings","version":1,"futureList":[800,600],"futureNumber":0.5}"#,
    );
    保管場所
        .ffmpegの置き場所を保存する(&見本の置き場所(), 試験の日時())
        .expect("保存できる");
    let 書いた = 読み直す(&パス);
    assert_eq!(書いた["futureList"], serde_json::json!([800, 600]));
    assert_eq!(書いた["futureNumber"], serde_json::json!(0.5));
    assert_eq!(
        書いた["ffmpegFolder"],
        serde_json::json!(r"C:\ツール\ffmpeg 7.1\bin")
    );
    後片付け(&パス);
}

#[test]
fn 新しすぎる版は読めないと返し_保存しても書き換えない() {
    let (保管場所, パス) = 一時の保管場所("新しすぎる版");
    let 新しい版 = r#"{"format":"ClipViewer.settings","version":2,"ffmpegFolder":"D:\\新"}"#;
    ファイルを置く(&パス, 新しい版);
    let 理由 = 保管場所
        .ffmpegの置き場所を読む()
        .expect_err("読めない")
        .to_string();
    assert!(理由.contains("新しい ClipViewer で保存された"), "{理由}");
    assert!(理由.contains("更新すれば読める"), "{理由}");
    assert!(理由.contains("書き換えない"), "{理由}");
    assert!(
        保管場所
            .ffmpegの置き場所を保存する(&見本の置き場所(), 試験の日時())
            .is_err()
    );
    assert_eq!(std::fs::read_to_string(&パス).expect("読める"), 新しい版);
    後片付け(&パス);
}

#[test]
fn 形式の名前が違うか読めない版の番号なら保存しても書き換えない() {
    for (名前, 本文) in [
        (
            "形式の名前",
            r#"{"format":"ModifierVideoStack","version":1}"#,
        ),
        (
            "版の番号",
            r#"{"format":"ClipViewer.settings","version":0}"#,
        ),
    ] {
        let (保管場所, パス) = 一時の保管場所(名前);
        ファイルを置く(&パス, 本文);
        let 理由 = 保管場所
            .ffmpegの置き場所を保存する(&見本の置き場所(), 試験の日時())
            .expect_err("書き換えない")
            .to_string();
        assert!(理由.contains("書き換えない"), "{理由}");
        assert_eq!(std::fs::read_to_string(&パス).expect("読める"), 本文);
        assert_eq!(
            フォルダの中のファイル名(&パス),
            vec!["settings.json".to_string()]
        );
        後片付け(&パス);
    }
}
