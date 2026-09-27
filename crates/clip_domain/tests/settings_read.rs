//! 設定ファイルの読み込みの試験。ブラウザ版が書き出した形の読み込みと、欠けた項目の既定値の補完。
#![allow(clippy::expect_used)]

mod support;

use clip_domain::*;
use support::連番の発行元;

fn 読み込む(
    本文: &str,
) -> Result<スタック設定, 設定ファイルの読み込みエラー> {
    let 本文 = 設定ファイルの本文::作成する(本文.to_string());
    スタック設定::本文から読み込む(&本文, &mut 連番の発行元::default())
}

/// ブラウザ版の書き出しとは、ブラウザ版 ClipViewer が JSON.stringify(設定, null, 2) で書き出す形を写した設定ファイルのことである。
/// 注意: この fixture はブラウザ版が実際に書き出したファイルではなく、移植元の型と書き出し処理を読んで手で書いたものである。
const ブラウザ版の書き出し: &str = include_str!("fixtures/browser_v4.json");

#[test]
fn ブラウザ版が書き出した設定ファイルを読める() {
    let 設定 = 読み込む(ブラウザ版の書き出し).expect("読める");
    assert_eq!(設定.期待する動画.expect("ある").文字列(), "sample.mp4");
    assert_eq!(
        設定.期待する動画のパス.expect("ある").文字列(),
        "C:\\videos\\sample.mp4"
    );
    let [一つ目, 二つ目] = 設定.スタック.一覧() else {
        panic!("2つのクリップがある");
    };
    assert_eq!(一つ目.識別子().文字列(), "mod-1725000000000-1234");
    assert_eq!(一つ目.区間.開始().秒数(), 1.5);
    assert_eq!(
        一つ目.リピート,
        リピート設定::回数指定(リピート回数::作成する(3).expect("範囲内"))
    );
    assert_eq!(一つ目.クロップ.高さ().数値(), 75.25);
    assert_eq!(一つ目.トリガー, トリガー::Enterキー待ち);
    assert!(!二つ目.有効か);
    assert_eq!(二つ目.リピート, リピート設定::無限ループ);
    assert_eq!(二つ目.トリガー, トリガー::クリック待ち);
    assert_eq!(設定.スタック.選択中のクリップ(), Some(一つ目));
}

#[test]
fn 欠けた項目は移植元と同じ既定値で補う() {
    let 設定 = 読み込む(r#"{"application":"ModifierVideoStack","modifiers":[{"id":""},{"name":"","crop":{"x":10}}]}"#)
        .expect("読める");
    let [一つ目, 二つ目] = 設定.スタック.一覧() else {
        panic!("2つのクリップがある");
    };
    assert_eq!(一つ目.識別子().文字列(), "発行-1");
    assert_eq!(二つ目.識別子().文字列(), "発行-2");
    assert_eq!(一つ目.名前.文字列(), "Unnamed Clip");
    assert!(一つ目.有効か);
    assert_eq!(
        (一つ目.区間.開始().秒数(), 一つ目.区間.終了().秒数()),
        (0.0, 5.0)
    );
    assert_eq!(一つ目.リピート, リピート設定::回数指定(リピート回数::一回));
    assert_eq!(一つ目.クロップ, クロップ範囲::全体);
    assert_eq!(一つ目.トリガー, トリガー::自動進行);
    assert_eq!(
        二つ目.クロップ,
        クロップ範囲::各値を範囲へ収めて作成する(10.0, 0.0, 100.0, 100.0)
    );
    assert_eq!(設定.期待する動画, None);
    assert_eq!(設定.期待する動画のパス, None);
}

#[test]
fn 回数とクロップは範囲へ収め期待する動画のパスは正規化する() {
    let 設定 = 読み込む(
        r#"{"application":"ModifierVideoStack","expectedVideoPath":"  \"C:\\a\\b.mp4\" ",
            "modifiers":[{"repeat":2.7,"crop":{"x":-5,"y":120,"w":2,"h":300}},{"repeat":500}]}"#,
    )
    .expect("読める");
    let [一つ目, 二つ目] = 設定.スタック.一覧() else {
        panic!("2つのクリップがある");
    };
    assert_eq!(
        一つ目.リピート,
        リピート設定::回数指定(リピート回数::作成する(2).expect("範囲内"))
    );
    assert_eq!(
        二つ目.リピート,
        リピート設定::回数指定(リピート回数::作成する(100).expect("範囲内"))
    );
    assert_eq!(
        一つ目.クロップ,
        クロップ範囲::各値を範囲へ収めて作成する(0.0, 100.0, 5.0, 100.0)
    );
    assert_eq!(
        設定.期待する動画のパス.expect("ある").文字列(),
        "C:\\a\\b.mp4"
    );
}
