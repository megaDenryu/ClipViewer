//! settings.json の本文の読み書きの試験。第1版の見本(`fixtures/settings_v1.json`)は第1版で書いたファイルそのものであり、以後書き換えない。
//! 書き出しが見本と1バイトも違わないことは、第1版が最新の版である間だけ確かめる。参照: _doc/設計/ライブラリ.md「形式の版を上げる手順」
#![allow(clippy::expect_used)]

use std::path::PathBuf;

use video_source::{FFmpegの置き場所の設定, FFmpegを置いたフォルダ};

use super::format::設定のファイルの本文;
use super::settings::アプリの設定;

pub(super) const 第1版の見本: &str = include_str!("fixtures/settings_v1.json");

/// 見本の ffmpegFolder に書いたフォルダ。
pub(super) fn 見本の置き場所() -> FFmpegを置いたフォルダ {
    FFmpegを置いたフォルダ::作成する(PathBuf::from(r"C:\ツール\ffmpeg 7.1\bin"))
}

#[test]
fn 第1版の見本を読むと期待値になり_期待値を書くと見本と1バイトも違わない() {
    let 期待値 = アプリの設定::何も無い().ffmpegの置き場所を変えた(
        FFmpegの置き場所の設定::設定済み(見本の置き場所()),
    );
    let 読んだ = 設定のファイルの本文::作成する(第1版の見本.to_string())
        .最新の設定として読む()
        .expect("読める");
    assert_eq!(読んだ, 期待値);
    let 書いた = 設定のファイルの本文::設定から書き出す(期待値).expect("書ける");
    assert_eq!(書いた.文字列().as_bytes(), 第1版の見本.as_bytes());
}

#[test]
fn 形式の名前が違うか版の番号が不正なら読めない() {
    for 本文 in [
        r#"{"format":"ModifierVideoStack","version":1}"#,
        r#"{"version":1,"ffmpegFolder":"C:\\x"}"#,
        r#"{"format":"ClipViewer.settings","version":0}"#,
        r#"{"format":"ClipViewer.settings"}"#,
        r#"{"format":"ClipViewer.settings","version":"1"}"#,
    ] {
        assert!(
            設定のファイルの本文::作成する(本文.to_string())
                .最新の設定として読む()
                .is_err(),
            "{本文}"
        );
    }
}
