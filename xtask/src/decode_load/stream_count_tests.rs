//! 同時に起動するffmpegの数の試験。範囲の検査と、`--streams` の表記の読み方を確かめる。
#![allow(clippy::expect_used)]

use super::stream_count::同時に起動するffmpegの数;

#[test]
fn 一から上限までの数だけを作る() {
    assert!(同時に起動するffmpegの数::作成する(0).is_none());
    assert!(同時に起動するffmpegの数::作成する(1).is_some());
    assert!(同時に起動するffmpegの数::作成する(64).is_some());
    assert!(同時に起動するffmpegの数::作成する(65).is_none());
}

#[test]
fn カンマで区切った表記を空白を除いて読む() {
    let 並び = 同時に起動するffmpegの数::並びを表記から読む("1, 3,16").expect("読める");
    let 表記: Vec<String> = 並び.iter().map(ToString::to_string).collect();
    assert_eq!(表記, ["1", "3", "16"]);
    assert_eq!(並び[1].起動する番号の範囲(), 0..3);
}

#[test]
fn 読めない要素はその要素を挙げて失敗にする() {
    let 失敗の理由を求める =
        |表記: &str| 同時に起動するffmpegの数::並びを表記から読む(表記).expect_err("失敗する");
    assert!(失敗の理由を求める("1,,2").contains("「」"));
    assert!(失敗の理由を求める("65").contains("「65」"));
    assert!(失敗の理由を求める("二").contains("「二」"));
}
