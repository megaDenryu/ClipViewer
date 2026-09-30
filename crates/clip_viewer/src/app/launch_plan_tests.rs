//! 起動の手順の試験。同じ一時フォルダで複数のアプリが独立して起動し、ライブラリの書き込みだけが限られることを確かめる。
//! 利用者の %APPDATA% のライブラリと受け口を使わない。
#![allow(clippy::expect_used)]

use std::ffi::OsString;
use std::path::{Path, PathBuf};

use eframe::egui;
use video_source::実行ファイルの検索パス;

use super::environment::起動時の環境;
use super::launch_plan::起動の手順;
use super::launch_preparation::起動の準備;
use crate::launch::起動の頼み;

fn 一時フォルダ(名前: &str) -> PathBuf {
    let フォルダ = std::env::temp_dir().join(format!(
        "clip_viewer_起動の手順_{名前}_{}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&フォルダ);
    フォルダ
}

fn 決める(アプリのデータ: &Path, 引数: &[&str]) -> 起動の準備 {
    起動の手順::作成する(起動時の環境 {
        アプリのデータのフォルダ: Some(アプリのデータ.to_path_buf()),
        ローカルのアプリのデータのフォルダ: None,
        ffmpegのフォルダ: None,
        検索パス: 実行ファイルの検索パス::作成する(Vec::new()),
        起動の引数: 引数.iter().map(OsString::from).collect(),
    })
    .決める()
}

#[test]
fn 複数の起動はそれぞれ動画を持ち_受け渡さず_書けるアプリが終われば次の起動が書ける() {
    let フォルダ = 一時フォルダ("複数");
    let 一つ目 = 決める(&フォルダ, &[r"C:\一つ目.mp4"]);
    assert!(
        一つ目
            .ライブラリ
            .as_ref()
            .is_some_and(|試した| 試した.書けるか())
    );
    let 受け取っている = 一つ目
        .受け口
        .expect("1つ目は受け口を開く")
        .受け取り始める(egui::Context::default())
        .expect("受け取り始められる");
    let 二つ目 = 決める(&フォルダ, &[r"C:\二つ目.mp4"]);
    let 三つ目 = 決める(&フォルダ, &[]);
    assert!(!二つ目.ライブラリの錠を持つか());
    assert!(!三つ目.ライブラリの錠を持つか());
    assert!(二つ目.受け口.is_none() && 三つ目.受け口.is_none());
    assert!(二つ目.知らせ.is_empty() && 三つ目.知らせ.is_empty());
    assert_eq!(
        一つ目.頼み,
        起動の頼み::引数から読む([OsString::from(r"C:\一つ目.mp4")])
    );
    assert_eq!(
        二つ目.頼み,
        起動の頼み::引数から読む([OsString::from(r"C:\二つ目.mp4")])
    );
    assert_eq!(三つ目.頼み, 起動の頼み::前に出る);
    assert!(受け取っている.届いた頼みを受け取る().is_empty());
    drop(受け取っている);
    drop(一つ目.ライブラリ);
    let 次 = 決める(&フォルダ, &[]);
    assert!(
        次.ライブラリ
            .as_ref()
            .is_some_and(|試した| 試した.書けるか())
    );
    assert!(次.受け口.is_some() && 次.知らせ.is_empty());
    drop((次, 二つ目, 三つ目));
    let _ = std::fs::remove_dir_all(&フォルダ);
}

#[test]
fn 一つ目が受け取り始めていなくても二つ目は自分の動画で起動する() {
    let フォルダ = 一時フォルダ("応答しない");
    let 一つ目 = 決める(&フォルダ, &[]);
    let 二つ目 = 決める(&フォルダ, &[r"C:\二つ目.mp4"]);
    assert!(
        二つ目
            .ライブラリ
            .as_ref()
            .is_some_and(|試した| 試した.別のアプリが持っているか())
    );
    assert!(二つ目.受け口.is_none());
    assert!(二つ目.知らせ.is_empty());
    assert_eq!(
        二つ目.頼み,
        起動の頼み::引数から読む([OsString::from(r"C:\二つ目.mp4")])
    );
    drop((一つ目, 二つ目));
    let _ = std::fs::remove_dir_all(&フォルダ);
}
