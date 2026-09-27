//! 流れてくる音の試験。輪の上書きの規則と、書き手が空きを待つことと、閉じたときに書き手が止まること。

#![allow(clippy::expect_used)]

use std::sync::Arc;
use std::thread;
use std::time::Duration;

use crate::{動画上の標本位置, 書き足した結果, 標本数, 流れてくる音};

fn 位置(番号: u32) -> 動画上の標本位置 {
    動画上の標本位置::作成する(番号)
}

fn 並び(開始: u32, 数: u32) -> Vec<[f32; 2]> {
    (開始..開始 + 数)
        .map(|番号| {
            let 値 = u16::try_from(番号).map_or(0.0, f32::from);
            [値, -値]
        })
        .collect()
}

#[test]
fn 書いた範囲の標本だけを読める() {
    let 音 = 流れてくる音::作成する(位置(1000), 標本数::作成する(8), 標本数::作成する(2));
    assert_eq!(音.書き足す(&並び(0, 5)), 書き足した結果::書き足した);
    assert!(音.標本(位置(999)).is_none());
    assert!(音.標本(位置(1005)).is_none());
    let 標本 = 音.標本(位置(1003)).expect("書いた標本");
    assert!((標本.左 - 3.0).abs() < 1e-9);
}

#[test]
fn 鳴らしている位置より残す長さだけ前までを上書きする() {
    let 音 = 流れてくる音::作成する(位置(0), 標本数::作成する(8), 標本数::作成する(2));
    assert_eq!(音.書き足す(&並び(0, 8)), 書き足した結果::書き足した);
    音.鳴らしている位置を知らせる(位置(5));
    assert_eq!(音.書き足す(&並び(8, 3)), 書き足した結果::書き足した);
    assert_eq!(音.範囲().開始().値(), 3);
    assert!(音.標本(位置(2)).is_none());
    assert!(音.標本(位置(3)).is_some());
    assert!(音.標本(位置(10)).is_some());
}

#[test]
fn 書き手は空きができるまで待ち閉じられたらやめる() {
    let 音 = Arc::new(流れてくる音::作成する(
        位置(0),
        標本数::作成する(4),
        標本数::作成する(0),
    ));
    let 書き手の音 = Arc::clone(&音);
    let 書き手 = thread::spawn(move || 書き手の音.書き足す(&並び(0, 6)));
    thread::sleep(Duration::from_millis(50));
    assert_eq!(音.範囲().終了().値(), 4, "容量まで書いて待っている");
    音.鳴らしている位置を知らせる(位置(2));
    thread::sleep(Duration::from_millis(50));
    let 結果 = 書き手.join().expect("書き手がパニックした");
    assert_eq!(結果, 書き足した結果::書き足した);
    assert_eq!(音.範囲().終了().値(), 6);

    let 書き手の音 = Arc::clone(&音);
    let 書き手 = thread::spawn(move || 書き手の音.書き足す(&並び(6, 10)));
    thread::sleep(Duration::from_millis(50));
    音.閉じる();
    let 結果 = 書き手.join().expect("書き手がパニックした");
    assert_eq!(結果, 書き足した結果::閉じられた);
}
