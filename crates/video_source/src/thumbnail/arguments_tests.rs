//! サムネイルの引数の試験。動画の情報が調べた映像の流れを再生と同じ規則のシークとフィルタで読み、クロップを割合の式で渡し、はみ出すクロップ範囲は寄せ、撮り方の大きさへ収め、JPEG を1枚だけ標準出力へ出させる。
#![allow(clippy::expect_used)]

use std::ffi::OsString;

use clip_domain::{
    クロップ範囲, サムネイルの大きさ, サムネイルの撮り方, 入力された動画パス, 動画上の秒,
};

use super::arguments::サムネイルの引数を並べる;
use crate::video_info::動画の情報;

fn 撮り方を作る(
    時刻: f64, クロップ: クロップ範囲
) -> サムネイルの撮り方 {
    サムネイルの撮り方::作成する(
        入力された動画パス::作成する(r"D:\動画\a.mp4".into()).正規化する(),
        動画上の秒::作成する(時刻).expect("時刻"),
        クロップ,
        サムネイルの大きさ::作成する(192, 108).expect("大きさ"),
    )
    .expect("撮り方")
}

/// 30コマ/秒で、読む映像の流れが2番(添付の画像や音の後ろ)にある動画の情報。
fn 動画() -> 動画の情報 {
    let パス = 入力された動画パス::作成する(r"D:\動画\a.mp4".into()).正規化する();
    let 本文 = r#"{"streams":[{"index":2,"codec_type":"video","width":640,"height":360,"avg_frame_rate":"30/1","duration":"60"}]}"#;
    動画の情報::調査の出力から作る(パス, 本文).expect("動画の情報")
}

fn 文字列にする(引数: Vec<OsString>) -> Vec<String> {
    引数
        .into_iter()
        .map(|一つ| 一つ.to_string_lossy().into_owned())
        .collect()
}

#[test]
fn 時刻へシークしてクロップを掛けて大きさへ収めた画像を一枚だけ出させる() {
    let クロップ =
        クロップ範囲::各値を範囲へ収めて作成する(10.0, 20.0, 50.0, 25.0);
    let 引数 = 文字列にする(サムネイルの引数を並べる(
        &撮り方を作る(12.5, クロップ),
        &動画(),
    ));
    let 位置 = |探す: &str| 引数.iter().position(|一つ| 一つ == 探す);
    assert_eq!(
        引数[位置("-ss").expect("ある") + 1],
        "12.483333",
        "12.5秒を含む375番のコマの半コマ手前へシークする"
    );
    assert!(位置("-ss") < 位置("-i"), "-ss は -i の前に置く");
    assert_eq!(
        引数[位置("-vf").expect("ある") + 1],
        r"setpts=PTS+0.000000/TB,fps=30/1:round=down:start_time=0,crop=w=iw*0.5:h=ih*0.25:x=min(iw*0.1\,iw-ow):y=min(ih*0.2\,ih-oh),scale=w=192:h=108:force_original_aspect_ratio=decrease,format=yuvj420p|yuvj422p|yuvj444p"
    );
    assert_eq!(引数[位置("-frames:v").expect("ある") + 1], "1");
    assert_eq!(
        引数[位置("-map").expect("ある") + 1],
        "0:2",
        "動画の情報が調べた映像の流れを読む"
    );
    assert!(引数.ends_with(&["-f", "image2pipe", "-c:v", "mjpeg", "-"].map(String::from)));
}
