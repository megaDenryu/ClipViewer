//! 見る側の設定の読み書きの試験。見本(`fixtures/settings_v1_viewer.json`)を読んだ値と書き出した本文、見る側の項目の無い古い第1版の読み方、
//! 知らない値と範囲の外の値の扱い、保存がほかの項目を消さないことと、壊れたファイルと新しすぎる版に書かないことを確かめる。
#![allow(clippy::expect_used)]

use audio_pcm::音量;
use clip_domain::{アスペクト比設定, 再生速度};

use super::format::設定のファイルの本文;
use super::test_support::{
    ファイルを置く, フォルダの中のファイル名, 一時の保管場所, 後片付け, 見本のキーの割り当て,
    読み直す,
};
use super::text_tests::第1版の見本;
use crate::viewer_settings::{
    ウインドウの大きさ, ウインドウの記憶, 左右の向き, 最大化の様子, 表示サイズ, 見る側の設定,
};

pub(super) const 見る側の見本: &str = include_str!("fixtures/settings_v1_viewer.json");

pub(super) fn 見本の見る側の設定() -> 見る側の設定 {
    見る側の設定 {
        ウインドウ: ウインドウの記憶 {
            大きさ: ウインドウの大きさ::幅と高さから作る(1440.0, 810.0).expect("大きさ"),
            最大化: 最大化の様子::最大化している,
        },
        音量: 音量::範囲へ収めて作る(0.25),
        速度: 再生速度::作成する(0.75).expect("速度"),
        左右: 左右の向き::反転,
        アスペクト比: アスペクト比設定::クロップ連動,
        表示サイズ: 表示サイズ::余白なし,
        キー: 見本のキーの割り当て(),
    }
}

#[test]
fn 見る側の見本を読むと期待値になり_期待値を書くと見本と1バイトも違わない() {
    let 読んだ = 設定のファイルの本文::作成する(見る側の見本.to_string())
        .最新の設定として読む()
        .expect("読める");
    assert_eq!(読んだ.見る側の設定(), 見本の見る側の設定());
    let 書いた = 設定のファイルの本文::設定から書き出す(読んだ).expect("書ける");
    assert_eq!(書いた.文字列().as_bytes(), 見る側の見本.as_bytes());
}

#[test]
fn 見る側の項目の無い古い第1版は既定の設定として読む() {
    let 読んだ = 設定のファイルの本文::作成する(第1版の見本.to_string())
        .最新の設定として読む()
        .expect("読める");
    assert_eq!(読んだ.見る側の設定(), 見る側の設定::既定);
}

#[test]
fn 知らない表記はその項目だけ既定にし_範囲の外の音量と速度は範囲へ収め_ほかの項目は読む() {
    let 本文 = r#"{"format":"ClipViewer.settings","version":1,"aspectRatio":"21:9","displaySize":"standard","volume":3.0,"playbackSpeed":9.0,"windowSize":{"width":0.0,"height":600.0}}"#;
    let 読んだ = 設定のファイルの本文::作成する(本文.to_string())
        .最新の設定として読む()
        .expect("壊れたとみなさない")
        .見る側の設定();
    assert_eq!(読んだ.アスペクト比, 見る側の設定::既定.アスペクト比);
    assert_eq!(読んだ.表示サイズ, 表示サイズ::標準);
    assert_eq!(読んだ.音量, 音量::範囲へ収めて作る(1.0));
    assert_eq!(読んだ.速度.倍率(), 2.0, "つまみの範囲の端へ収める");
    assert_eq!(読んだ.ウインドウ.大きさ, ウインドウの大きさ::最初);
}

#[test]
fn 見る側の設定の保存はffmpegの置き場所と知らない項目を消さない() {
    let (保管場所, パス) = 一時の保管場所("見る側の保存");
    ファイルを置く(
        &パス,
        r#"{"format":"ClipViewer.settings","version":1,"ffmpegFolder":"D:/ff","futureItem":7}"#,
    );
    保管場所
        .見る側の設定を保存する(見本の見る側の設定())
        .expect("書ける");
    let 書いた = 読み直す(&パス);
    assert_eq!(書いた["ffmpegFolder"], serde_json::json!("D:/ff"));
    assert_eq!(書いた["futureItem"], serde_json::json!(7));
    assert_eq!(
        保管場所.見る側の設定を読む().expect("読める"),
        見本の見る側の設定()
    );
    後片付け(&パス);
}

#[test]
fn 壊れたファイルと新しすぎる版には見る側の設定を書かず_壊れたファイルを移さない() {
    for (名前, 本文) in [
        ("見る側_壊れた", r#"{"ffmpegFolder":"D:/ff"#),
        (
            "見る側_新しすぎる",
            r#"{"format":"ClipViewer.settings","version":2}"#,
        ),
    ] {
        let (保管場所, パス) = 一時の保管場所(名前);
        ファイルを置く(&パス, 本文);
        assert!(保管場所.見る側の設定を保存する(見る側の設定::既定).is_err());
        assert_eq!(std::fs::read_to_string(&パス).expect("読める"), 本文);
        assert_eq!(
            フォルダの中のファイル名(&パス),
            vec!["settings.json".to_string()]
        );
        後片付け(&パス);
    }
}
