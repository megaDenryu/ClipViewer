//! settings.json の項目の名前の試験。第0版に第1版と同じ名前の項目があっても同じ名前を2回書かないことと、
//! 第1版が書く項目の名前の一覧が、全項目を書いた見本の項目の名前と一致することを確かめる。
#![allow(clippy::expect_used)]

use super::test_support::{ファイルを置く, 一時の保管場所, 後片付け};
use super::viewer_tests::{見る側の見本, 見本の見る側の設定};

#[test]
fn 第0版に第1版と同じ名前の項目があっても_書いたファイルに同じ名前の項目は1回しか出ない() {
    let (保管場所, パス) = 一時の保管場所("第0版の同じ名前");
    ファイルを置く(
        &パス,
        r#"{"ffmpegFolder":"D:/ff","volume":"古い値","windowMaximized":"古い値","keepMe":1}"#,
    );
    保管場所
        .見る側の設定を保存する(見本の見る側の設定())
        .expect("書ける");
    let 本文 = std::fs::read_to_string(&パス).expect("読める");
    for 名前 in ["\"volume\"", "\"windowMaximized\"", "\"ffmpegFolder\""] {
        assert_eq!(本文.matches(名前).count(), 1, "{名前}: {本文}");
    }
    assert!(本文.contains("\"keepMe\""), "{本文}");
    assert_eq!(
        保管場所.見る側の設定を読む().expect("読める"),
        見本の見る側の設定()
    );
    後片付け(&パス);
}

#[test]
fn 第1版が書く項目の名前は_全項目を書いたファイルの項目の名前と一致する() {
    let 全項目: serde_json::Value = serde_json::from_str(見る側の見本).expect("JSON");
    let mut 書いた名前: Vec<&str> = 全項目
        .as_object()
        .expect("オブジェクト")
        .keys()
        .map(String::as_str)
        .collect();
    let mut 知っている名前 = super::v1::第1版が書く項目の名前.to_vec();
    書いた名前.sort_unstable();
    知っている名前.sort_unstable();
    assert_eq!(書いた名前, 知っている名前);
}
