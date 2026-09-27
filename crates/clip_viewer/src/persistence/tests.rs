//! 永続化境界の試験。アプリの設定は一時ディレクトリへ読み書きする。
#![allow(clippy::expect_used)]

use std::path::PathBuf;

use clip_domain::動画ファイル名;
use video_source::{FFmpegの置き場所の設定, FFmpegを置いたフォルダ};

use super::{
    FFmpegの置き場所の候補, アプリの設定の保管場所, 設定ファイルの既定の名前
};

/// 試験ごとに重ならない一時フォルダ。試験の名前とプロセス番号で分ける。
fn 一時フォルダ(名前: &str) -> PathBuf {
    let フォルダ =
        std::env::temp_dir().join(format!("clip_viewer_試験_{名前}_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&フォルダ);
    フォルダ
}

#[test]
fn 保存したffmpegの置き場所を読み直せる() {
    let フォルダ = 一時フォルダ("保存");
    let 保管場所 =
        アプリの設定の保管場所::アプリのデータのフォルダから決める(
            Some(フォルダ.clone()),
        );
    assert_eq!(
        保管場所.ffmpegの置き場所を読む().expect("読める"),
        FFmpegの置き場所の設定::未設定
    );
    let 置き場所 = FFmpegを置いたフォルダ::作成する(PathBuf::from(r"C:\ffmpeg\bin"));
    保管場所
        .ffmpegの置き場所を保存する(&置き場所, std::time::SystemTime::now())
        .expect("保存できる");
    assert_eq!(
        保管場所.ffmpegの置き場所を読む().expect("読める"),
        FFmpegの置き場所の設定::設定済み(置き場所)
    );
    let _ = std::fs::remove_dir_all(&フォルダ);
}

#[test]
fn 形の壊れた設定は読めないと返し_保管場所が無ければ保存は失敗する() {
    let フォルダ = 一時フォルダ("壊れた");
    let 保管場所 =
        アプリの設定の保管場所::アプリのデータのフォルダから決める(
            Some(フォルダ.clone()),
        );
    std::fs::create_dir_all(フォルダ.join("ClipViewer")).expect("フォルダを作れる");
    std::fs::write(フォルダ.join("ClipViewer").join("settings.json"), "{壊れた").expect("書ける");
    assert!(保管場所.ffmpegの置き場所を読む().is_err());
    let 保管場所が無い =
        アプリの設定の保管場所::アプリのデータのフォルダから決める(
            None,
        );
    let 置き場所 = FFmpegを置いたフォルダ::作成する(PathBuf::from("x"));
    assert!(
        保管場所が無い
            .ffmpegの置き場所を保存する(&置き場所, std::time::SystemTime::now())
            .is_err()
    );
    let _ = std::fs::remove_dir_all(&フォルダ);
}

#[test]
fn 環境変数のフォルダを保存した置き場所より先に使う() {
    let 環境 = FFmpegを置いたフォルダ::作成する(PathBuf::from("環境"));
    let 保存 = FFmpegの置き場所の設定::設定済み(FFmpegを置いたフォルダ::作成する(
        PathBuf::from("保存"),
    ));
    let 候補 = FFmpegの置き場所の候補 {
        環境変数のフォルダ: Some(環境.clone()),
        保存した置き場所: 保存.clone(),
    };
    assert_eq!(候補.使う置き場所(), FFmpegの置き場所の設定::設定済み(環境));
    let 候補 = FFmpegの置き場所の候補 {
        環境変数のフォルダ: None,
        保存した置き場所: 保存.clone(),
    };
    assert_eq!(候補.使う置き場所(), 保存);
}

#[test]
fn 設定ファイルの既定の名前は動画の名前の最初の点より前から作る() {
    let 名前 = 動画ファイル名::作成する("会議.2026.mp4".into()).expect("名前");
    assert_eq!(
        設定ファイルの既定の名前::動画から作る(Some(&名前)).文字列(),
        "会議_clip_stack_config.json"
    );
    assert_eq!(
        設定ファイルの既定の名前::動画から作る(None).文字列(),
        "stack_clip_stack_config.json"
    );
}
