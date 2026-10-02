//! 重ね合わせの供給の、動画を開けないときの結合試験。使う動画の表の動画を開けないときは、供給は作り、その動画の様子に理由を持ち、
//! その動画の置いたクリップの行は何も映さないことを確かめる。ffprobe を起動するため FFmpeg が要る。
//! FFmpeg が要るため `#[ignore]` にしてあり、`cargo xtask verify` が FFmpeg を見つけたときだけ `--ignored` で流す。
#![allow(clippy::expect_used)]

use std::time::Instant;

use clip_domain::{
    入力された動画パス, 動画の番号, 時刻, 末尾での振る舞い, 行の番号
};
use eframe::egui;
use video_source::{メモリの上限, 動画の読み手};

use super::row_showing::行の映し具合;
use super::show_condition::映すものを求める条件;
use super::status::重ね合わせの映像の様子;
use crate::overlay::test_support::{置き方, 置き方を並べた重ね合わせ};
use crate::video_feed::with_ffmpeg_support::実行ファイルを探す;
use crate::video_feed::動画を開けない理由;

#[test]
#[ignore = "FFmpeg が要る。cargo xtask verify が FFmpeg を見つけたときだけ流す"]
fn 開けない動画は_その理由を動画の様子に持ち_その動画の行は何も映さない() {
    let 読み手 = 動画の読み手::作成する(実行ファイルを探す());
    let 無い動画 = 入力された動画パス::作成する(
        std::env::temp_dir()
            .join("clip_viewer_重ね合わせの供給の試験_無い動画.mp4")
            .display()
            .to_string(),
    )
    .正規化する();
    let 重ね合わせ = 置き方を並べた重ね合わせ(
        無い動画,
        4.0,
        &[&[置き方 {
            開始: 0.5,
            終了: 1.5,
            始まり: 0.0,
        }]],
    );
    let mut 映像 = 重ね合わせの映像の様子::開く(
        Some(&読み手),
        重ね合わせ.動画の表(),
        メモリの上限::既定,
        &egui::Context::default(),
    );
    let 知らせの文 = 映像.開けなかった知らせの文();
    assert_eq!(知らせの文.len(), 1, "開けなかった動画の理由を1つ知らせる");
    let 条件 = 映すものを求める条件 {
        時刻: 時刻::先頭,
        全体の末尾: 末尾での振る舞い::止まる,
    };
    映像.供給する(&重ね合わせ, 条件, Instant::now());
    let 重ね合わせの映像の様子::供給している(供給) = &映像 else {
        panic!("動画を開けなくても供給は作れる");
    };
    let 様子 = 供給
        .動画の様子(動画の番号::番号から作成する(0))
        .expect("表にある");
    assert!(matches!(
        様子.開けない理由(),
        Some(動画を開けない理由::情報を取れない(_))
    ));
    let 行 = 供給
        .行(行の番号::番号から作成する(0))
        .expect("行の数の上限の内");
    assert_eq!(行.映し具合(), 行の映し具合::映していない);
}
