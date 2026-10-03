//! 読む秒数の試験。範囲の検査と、`--seconds` の表記の読み方と、ffmpeg へ渡す表記を確かめる。
#![allow(clippy::expect_used)]

use super::read_length::読む秒数;

#[test]
fn 一から上限までの秒数だけを作る() {
    assert!(読む秒数::作成する(0).is_none());
    assert!(読む秒数::作成する(600).is_some());
    assert!(読む秒数::作成する(601).is_none());
    assert_eq!(読む秒数::既定.to_string(), "30秒");
}

#[test]
fn 表記を読みffmpegへ渡す秒の表記に戻す() {
    let 秒数 = 読む秒数::表記から読む("12").expect("読める");
    assert_eq!(秒数.ffmpegへ渡す秒の表記(), "12");
    let 失敗の理由 = 読む秒数::表記から読む("0").expect_err("失敗する");
    assert!(失敗の理由.contains("「0」"));
}
