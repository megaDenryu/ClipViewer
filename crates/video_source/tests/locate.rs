//! FFmpeg の実行ファイルを探す手順の試験。FFmpeg が無くても流せる。

use std::path::PathBuf;

use video_source::{
    FFmpegの実行ファイル, FFmpegの置き場所の設定, FFmpegの道具, FFmpegを置いたフォルダ,
    実行ファイルの検索パス, 探した場所,
};

#[test]
fn 見つからなければ探した場所を順に並べたエラーを返す() {
    let 設定のフォルダ = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("FFmpegの無いフォルダ");
    let 検索パスのフォルダ = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("PATHの無いフォルダ");
    let 設定 = FFmpegの置き場所の設定::設定済み(FFmpegを置いたフォルダ::作成する(
        設定のフォルダ.clone(),
    ));
    let 検索パス =
        実行ファイルの検索パス::作成する(vec![検索パスのフォルダ.clone()]);
    let Err(エラー) = FFmpegの実行ファイル::探す(&設定, &検索パス) else {
        panic!("見つからないはずである")
    };
    assert_eq!(
        エラー.見つからない道具,
        vec![FFmpegの道具::変換, FFmpegの道具::調査]
    );
    assert_eq!(
        エラー.探した場所,
        vec![
            探した場所::設定の場所(設定のフォルダ),
            探した場所::PATHの中(検索パスのフォルダ)
        ]
    );
    assert!(エラー.to_string().contains("探した場所"));
}

#[test]
fn 設定が未設定で検索パスも空なら探した場所が無いと書く() {
    let 結果 = FFmpegの実行ファイル::探す(
        &FFmpegの置き場所の設定::未設定,
        &実行ファイルの検索パス::default(),
    );
    let Err(エラー) = 結果 else {
        panic!("見つからないはずである")
    };
    assert!(エラー.探した場所.is_empty());
    assert!(エラー.to_string().contains("未設定"));
}

/// 試験用のフォルダを作り、指定の道具の名前の空のファイルを置く。
fn 道具を置いたフォルダ(名前: &str, 道具: &[FFmpegの道具]) -> PathBuf {
    let フォルダ = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(名前);
    let _ = std::fs::remove_dir_all(&フォルダ);
    if std::fs::create_dir_all(&フォルダ).is_err() {
        panic!("試験用のフォルダを作れない");
    }
    for 一つ in 道具 {
        if std::fs::write(フォルダ.join(一つ.ファイル名()), b"").is_err() {
            panic!("試験用のファイルを置けない");
        }
    }
    フォルダ
}

#[test]
fn ffmpegとffprobeは同じフォルダにそろう場所から選ぶ() {
    let 変換だけ = 道具を置いたフォルダ("変換だけ", &[FFmpegの道具::変換]);
    let 調査だけ = 道具を置いたフォルダ("調査だけ", &[FFmpegの道具::調査]);
    let 両方 = 道具を置いたフォルダ("両方", &[FFmpegの道具::変換, FFmpegの道具::調査]);
    let 分かれた検索パス =
        実行ファイルの検索パス::作成する(vec![変換だけ.clone(), 調査だけ.clone()]);
    let Err(エラー) =
        FFmpegの実行ファイル::探す(&FFmpegの置き場所の設定::未設定, &分かれた検索パス)
    else {
        panic!("別々のフォルダの組を選んではならない")
    };
    assert!(エラー.見つからない道具.is_empty());
    assert!(
        エラー
            .to_string()
            .contains("同じフォルダにそろう場所が無い")
    );
    let 検索パス =
        実行ファイルの検索パス::作成する(vec![変換だけ, 調査だけ, 両方.clone()]);
    let Ok(見つけた) =
        FFmpegの実行ファイル::探す(&FFmpegの置き場所の設定::未設定, &検索パス)
    else {
        panic!("両方そろうフォルダがあるのに見つからない")
    };
    assert_eq!(見つけた.変換のパス().parent(), Some(両方.as_path()));
    assert_eq!(見つけた.調査のパス().parent(), Some(両方.as_path()));
}
