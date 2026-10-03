//! decode-load の引数の解釈の試験。省いた値の既定と、解釈できない引数を理由付きの失敗にすることを確かめる。
#![allow(clippy::expect_used)]

use std::path::Path;

use super::arguments::{測る動画, 測定の指定};

fn 引数を解釈する(並び: &[&str]) -> Result<測定の指定, String> {
    let 引数一覧: Vec<String> = 並び.iter().map(ToString::to_string).collect();
    測定の指定::引数から解釈する(&引数一覧)
}

fn ffmpegの数の値(指定: &測定の指定) -> Vec<u32> {
    指定.ffmpegの数の並び.iter().map(|数| 数.値()).collect()
}

#[test]
fn 引数が無ければ合成画像を30秒作って1本2本4本8本で測る() {
    let 指定 = 引数を解釈する(&[]).expect("解釈できる");
    assert_eq!(指定.動画, 測る動画::合成画像で作る);
    assert_eq!(ffmpegの数の値(&指定), [1, 2, 4, 8]);
    assert_eq!(指定.読む長さ.値(), 30);
}

#[test]
fn 動画のパスとオプションを順不同で受け取る() {
    let 指定 = 引数を解釈する(&["--seconds", "10", "動画.mp4", "--streams", "1, 3,16"])
        .expect("解釈できる");
    let 測る動画::与えられた動画(パス) = &指定.動画 else {
        panic!("与えられた動画になっていない: {:?}", 指定.動画);
    };
    assert_eq!(パス.パス(), Path::new("動画.mp4"));
    assert_eq!(ffmpegの数の値(&指定), [1, 3, 16]);
    assert_eq!(指定.読む長さ.値(), 10);
}

#[test]
fn 解釈できない引数は理由を付けて失敗にする() {
    let 失敗の理由を求める = |並び: &[&str]| 引数を解釈する(並び).expect_err("失敗する");
    assert!(失敗の理由を求める(&["--streams"]).contains("値が無い"));
    assert!(失敗の理由を求める(&["--streams", "0"]).contains("「0」"));
    assert!(失敗の理由を求める(&["--streams", "1,,2"]).contains("「」"));
    assert!(失敗の理由を求める(&["--streams", "65"]).contains("「65」"));
    assert!(失敗の理由を求める(&["--seconds", "0"]).contains("「0」"));
    assert!(失敗の理由を求める(&["--seconds", "601"]).contains("「601」"));
    assert!(失敗の理由を求める(&["--fast"]).contains("「--fast」"));
    assert!(失敗の理由を求める(&["甲.mp4", "乙.mp4"]).contains("1つだけ"));
}
