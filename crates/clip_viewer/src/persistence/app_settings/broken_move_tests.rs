//! 壊れた settings.json の退避(`broken_move.rs`)の試験。移し先の名前は先に移した写しを上書きせず、
//! 移した後で書けなかったときは理由の文に移し先を入れる。
#![allow(clippy::expect_used)]

use super::test_support::{
    ファイルを置く, フォルダの中のファイル名, 一時の保管場所, 後片付け, 試験の日時,
};
use super::text_tests::見本の置き場所;
use super::アプリの設定のエラー;

#[test]
fn 同じ日時に2回移しても先に移した写しを上書きしない() {
    let (保管場所, パス) = 一時の保管場所("2回移す");
    for 壊れた本文 in ["{1回目", "{2回目"] {
        ファイルを置く(&パス, 壊れた本文);
        保管場所
            .ffmpegの置き場所を保存する(&見本の置き場所(), 試験の日時())
            .expect("移して書ける");
    }
    let 移した本文: Vec<String> = フォルダの中のファイル名(&パス)
        .iter()
        .filter(|名前| 名前.starts_with("settings.json.broken-"))
        .map(|名前| std::fs::read_to_string(パス.with_file_name(名前)).expect("読める"))
        .collect();
    assert_eq!(移した本文.len(), 2, "{移した本文:?}");
    assert!(移した本文.contains(&"{1回目".to_string()), "{移した本文:?}");
    assert!(移した本文.contains(&"{2回目".to_string()), "{移した本文:?}");
    後片付け(&パス);
}

#[test]
fn 移した後で書けなければ理由の文に移し先を入れる() {
    let (保管場所, パス) = 一時の保管場所("移した後で書けない");
    ファイルを置く(&パス, "{壊れた");
    let 書きかけ = パス.with_file_name(format!("settings.json.{}.tmp", std::process::id()));
    std::fs::create_dir(&書きかけ).expect("書きかけの名前でフォルダを作れる");
    let エラー = 保管場所
        .ffmpegの置き場所を保存する(&見本の置き場所(), 試験の日時())
        .expect_err("書けない");
    let アプリの設定のエラー::移した後で書けない { 移し先, .. } = &エラー
    else {
        panic!("移した後で書けないでない: {エラー}");
    };
    assert!(エラー.to_string().contains(移し先.as_str()), "{エラー}");
    assert_eq!(
        std::fs::read_to_string(移し先).expect("写しが残る"),
        "{壊れた"
    );
    後片付け(&パス);
}
