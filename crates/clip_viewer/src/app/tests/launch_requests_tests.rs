//! 起動の頼みの適用の試験。起動の引数の頼みと、起動の受け口に届いた頼みを、ウインドウへ落とした動画と同じく確かめてから開こうとすることを確かめる。
//! FFmpeg を見つけていない状態で当てるため、開こうとした結果は「FFmpeg が無い」の通知で見分ける。案内のファイルは一時フォルダに置く。
#![allow(clippy::expect_used)]

use std::time::Instant;

use std::ffi::OsString;
use std::time::Duration;

use eframe::egui;

use super::super::launch_requests::起動の頼みの届き方;
use super::launch_requests_test_support::{試験のビューアー, 通知の文};
use crate::launch::{
    受け口の案内ファイル, 起動の受け口, 起動の頼み, 起動の頼みの送り手
};

#[test]
fn 起動の引数の動画を開こうとし_起動の知らせは書き足し_開けなければ開かなかった数は書き足さない() {
    let mut ビューアー = 試験のビューアー();
    let 引数 = [r"C:\a.mp4", r"C:\b.mp4", r"C:\c.mp4"].map(OsString::from);
    ビューアー.起動の受け口で受け取り始めて頼みを当てる(
        None,
        egui::Context::default(),
        起動の頼み::引数から読む(引数),
        vec!["起動の知らせ".to_string()],
    );
    let 文面 = 通知の文(&ビューアー);
    assert!(
        文面.starts_with("FFmpeg が見つからないため動画を開けない"),
        "{文面}"
    );
    assert!(文面.contains("起動の知らせ"), "{文面}");
    assert!(!文面.contains("最初の1つだけ"), "{文面}");
}

#[test]
fn 受け口に届いた頼みを当てて届いたと返す() {
    let フォルダ = std::env::temp_dir().join(format!(
        "clip_viewer_起動の頼みの適用_{}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&フォルダ);
    let 案内ファイル = 受け口の案内ファイル::作成する(フォルダ.join("launch.json"));
    let mut ビューアー = 試験のビューアー();
    let 受け口 = 起動の受け口::開く(&案内ファイル).expect("待ち始められる");
    ビューアー.起動の受け口で受け取り始めて頼みを当てる(
        Some(受け口),
        egui::Context::default(),
        起動の頼み::前に出る,
        Vec::new(),
    );
    assert_eq!(
        ビューアー.届いた起動の頼みを適用する(Instant::now()),
        起動の頼みの届き方::届いていない
    );
    let 頼み = 起動の頼み::引数から読む([OsString::from(r"C:\届いた.mp4")]);
    起動の頼みの送り手::作成する(案内ファイル, Duration::from_secs(3))
        .送る(&頼み)
        .expect("渡せる");
    assert_eq!(
        ビューアー.届いた起動の頼みを適用する(Instant::now()),
        起動の頼みの届き方::届いた
    );
    let 文面 = 通知の文(&ビューアー);
    assert!(
        文面.starts_with("FFmpeg が見つからないため動画を開けない"),
        "{文面}"
    );
    drop(ビューアー);
    let _ = std::fs::remove_dir_all(&フォルダ);
}
