//! 測ったフレームの時間のまとめの試験。数・平均・中央値・95百分位・最大の求め方と、空の並びは測れなかったと書くことを確かめる。
#![allow(clippy::expect_used)]

use std::time::Duration;

use super::record::{時間の並びのまとめ, 測ったフレームの時間};

fn ミリ秒の並び(並び: &[u64]) -> Vec<Duration> {
    並び.iter().copied().map(Duration::from_millis).collect()
}

#[test]
fn 並びのまとめは並べた順の中央値と95百分位と最大を持つ() {
    let まとめ = 時間の並びのまとめ::並びから求める(&ミリ秒の並び(&[
        20, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
    ]))
    .expect("空でない");
    assert_eq!(
        まとめ,
        時間の並びのまとめ {
            数: 20,
            平均: Duration::from_micros(10_500),
            中央値: Duration::from_millis(10),
            九十五百分位: Duration::from_millis(19),
            最大: Duration::from_millis(20),
        }
    );
}

#[test]
fn 一つだけの並びはどのまとめもその値になり_空の並びはまとめが無い() {
    let まとめ = 時間の並びのまとめ::並びから求める(&ミリ秒の並び(&[7])).expect("空でない");
    assert_eq!(まとめ.中央値, Duration::from_millis(7));
    assert_eq!(まとめ.九十五百分位, Duration::from_millis(7));
    assert!(時間の並びのまとめ::並びから求める(&[]).is_none());
}

#[test]
fn 一フレームの時間が知らされなかったフレームは間隔だけを数え_空なら測れなかったと書く() {
    let mut 測った = 測ったフレームの時間::default();
    測った.足す(None, Duration::from_millis(16));
    let 文 = 測った.まとめの文();
    assert!(文.contains("1フレームの時間(GPU へ載せる時間を含み、垂直同期の待ちを含まない): 測れなかった(0フレーム)"), "{文}");
    assert!(
        文.contains("前のフレームからの間隔: 1フレーム 平均 16.00ミリ秒"),
        "{文}"
    );
}
