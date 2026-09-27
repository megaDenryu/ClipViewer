//! 受け渡しの往復の試験。受け口を試験の中で立て、送り手から頼みが届くことと、合言葉の違う頼みを受け付けないことを確かめる。
#![allow(clippy::expect_used)]

use std::time::Duration;

use eframe::egui;

use super::guide::合言葉;
use super::test_support::{一時の案内ファイル, 動画を開く頼み};
use super::{
    受け口の案内ファイル, 起動の受け口, 起動の頼み, 起動の頼みの送り手
};

#[test]
fn 送り手から渡した頼みが受け口に届く() {
    let (フォルダ, 案内ファイル) = 一時の案内ファイル("往復");
    let 受け口 = 起動の受け口::開く(&案内ファイル)
        .expect("待ち始められる")
        .受け取り始める(egui::Context::default())
        .expect("受け取り始められる");
    let 送り手 = 起動の頼みの送り手::作成する(案内ファイル, Duration::from_secs(3));
    for 頼み in [
        動画を開く頼み(r"C:\動画\日本語の名前.mp4", 1),
        起動の頼み::前に出る,
    ] {
        送り手.送る(&頼み).expect("渡せる");
        assert_eq!(受け口.届いた頼みを受け取る(), vec![頼み]);
    }
    drop(受け口);
    let _ = std::fs::remove_dir_all(&フォルダ);
}

#[test]
fn 合言葉の違う頼みは受け付けない() {
    let (フォルダ, 案内ファイル) = 一時の案内ファイル("合言葉");
    let 受け口 = 起動の受け口::開く(&案内ファイル)
        .expect("待ち始められる")
        .受け取り始める(egui::Context::default())
        .expect("受け取り始められる");
    let mut 案内 = 案内ファイル.読む().expect("読める");
    案内.合言葉 = 合言葉::作る();
    let 偽の案内ファイル =
        受け口の案内ファイル::作成する(フォルダ.join("wrong.json"));
    偽の案内ファイル.書く(&案内).expect("書ける");
    let 送り手 =
        起動の頼みの送り手::作成する(偽の案内ファイル, Duration::from_millis(300));
    assert!(送り手.送る(&起動の頼み::前に出る).is_err());
    assert!(受け口.届いた頼みを受け取る().is_empty());
    drop(受け口);
    let _ = std::fs::remove_dir_all(&フォルダ);
}
