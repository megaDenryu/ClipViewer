//! 動画パスの正規化と検証の試験。
#![allow(clippy::expect_used)]

use clip_domain::*;

fn 入力(文字列: &str) -> 入力された動画パス {
    入力された動画パス::作成する(文字列.to_string())
}

fn 不備(文字列: &str, 期待: Option<&str>) -> Option<動画パスの不備> {
    let 期待 = 期待.and_then(|名前| 動画ファイル名::作成する(名前.to_string()));
    match 入力(文字列).検証する(期待.as_ref()) {
        動画パスの検証結果::有効(_) => None,
        動画パスの検証結果::不正 { 不備, .. } => Some(不備),
    }
}

#[test]
fn 正規化は前後の空白と対になった引用符を取り除く() {
    assert_eq!(入力("  C:\\a.mp4 ").正規化する().文字列(), "C:\\a.mp4");
    assert_eq!(入力(" \" C:\\a.mp4 \" ").正規化する().文字列(), "C:\\a.mp4");
    assert_eq!(入力("'C:\\a.mp4'").正規化する().文字列(), "C:\\a.mp4");
    assert_eq!(入力("\"C:\\a.mp4'").正規化する().文字列(), "\"C:\\a.mp4'");
    assert_eq!(入力("\"").正規化する().文字列(), "\"");
}

#[test]
fn 有効なパスはドライブ文字の形と共有フォルダの形である() {
    assert_eq!(不備("C:\\videos\\a.mp4", None), None);
    assert_eq!(不備("c:/videos/a.mp4", None), None);
    assert_eq!(不備("\\\\server\\share\\a.mp4", None), None);
}

#[test]
fn 不正なパスは理由を返し正規化したパスも返す() {
    assert_eq!(不備("  ", None), Some(動画パスの不備::空である));
    let 片側 = 不備("\"C:\\a.mp4", None);
    assert_eq!(片側, Some(動画パスの不備::引用符が釣り合っていない));
    assert_eq!(
        不備("videos\\a.mp4", None),
        Some(動画パスの不備::絶対パスでない)
    );
    assert_eq!(
        不備("\\\\server", None),
        Some(動画パスの不備::絶対パスでない)
    );
    assert_eq!(不備("C:a.mp4", None), Some(動画パスの不備::絶対パスでない));
    let 結果 = 入力(" 'relative.mp4' ").検証する(None);
    let 動画パスの検証結果::不正 {
        正規化したパス,
    ..
    } = 結果
    else {
        panic!("不正のはず");
    };
    assert_eq!(正規化したパス.文字列(), "relative.mp4");
}

#[test]
fn 期待する動画名とは大文字小文字を区別せずに照合する() {
    assert_eq!(不備("C:\\v\\Sample.MP4", Some("sample.mp4")), None);
    assert_eq!(不備("C:/v/sample.mp4", Some("sample.mp4")), None);
    let 期待 = 動画ファイル名::作成する("other.mp4".to_string()).expect("空でない");
    let 不一致 = 不備("C:\\v\\sample.mp4", Some("other.mp4"));
    assert_eq!(
        不一致,
        Some(動画パスの不備::ファイル名が一致しない(
            期待
        ))
    );
    let 文言 = 動画パスの不備::空である.to_string();
    assert_eq!(文言, "動画のPC内絶対パスを入力してください");
}
