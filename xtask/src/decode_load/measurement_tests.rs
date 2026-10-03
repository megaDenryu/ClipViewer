//! 一回の測定から速さと倍率を求めることの試験。
#![allow(clippy::expect_used)]

use std::time::Duration;

use super::byte_count::バイト数;
use super::elapsed::読み切る時間;
use super::frame_count::コマ数;
use super::frame_rate::コマの速さ;
use super::measurement::一回の測定;
use super::stream_count::同時に起動するffmpegの数;

fn 一回の測定を作る(
    ffmpegの数: u32, ミリ秒: u64, コマの枚数: u32
) -> 一回の測定 {
    一回の測定::作成する(
        同時に起動するffmpegの数::作成する(ffmpegの数).expect("作れる"),
        読み切る時間::作成する(Duration::from_millis(ミリ秒)).expect("作れる"),
        コマ数::作成する(コマの枚数).expect("作れる"),
    )
}

#[test]
fn 合計と1本あたりの速さと倍率を求める() {
    let 四本 = 一回の測定を作る(4, 9_000, 900);
    assert_eq!(四本.合計の速さ().to_string(), "約400コマ/秒");
    assert_eq!(四本.一本あたりの速さ().to_string(), "約100コマ/秒");
    let 再生の速さ = コマの速さ::作成する(30, 1).expect("作れる");
    let 倍率 = 四本.一本あたりの速さ().再生の速さに対する倍率(再生の速さ);
    assert_eq!(倍率.to_string(), "約3.3倍");
}

#[test]
fn 時間とコマ数は0を作らない() {
    assert!(読み切る時間::作成する(Duration::ZERO).is_none());
    assert!(コマ数::作成する(0).is_none());
}

#[test]
fn 読んだバイト数を1コマの大きさで割り切れるときだけコマ数にする() {
    let 一コマ = バイト数::画素の数から求める(2, 2, 4);
    let 三コマ = バイト数::ゼロ.読んだ分を足す(32).読んだ分を足す(16);
    assert_eq!(三コマ.割り切ってコマ数を求める(一コマ), コマ数::作成する(3));
    assert_eq!(
        バイト数::ゼロ
            .読んだ分を足す(47)
            .割り切ってコマ数を求める(一コマ),
        None
    );
    assert_eq!(バイト数::ゼロ.割り切ってコマ数を求める(一コマ), None);
}
