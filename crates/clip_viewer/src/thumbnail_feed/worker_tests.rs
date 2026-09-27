//! 仕事を順に行う裏のスレッドの試験。頼んだ順に結果が届き、スレッドが異常終了したら返事を待つのをやめる。
#![allow(clippy::expect_used)]

use std::time::{Duration, Instant};

use super::worker::{
    仕事を順に行う裏のスレッド, 裏のスレッドで仕事を行う処理
};

/// 数を2倍にし、0を渡されたら異常終了する処理。
struct 二倍にする処理;

impl 裏のスレッドで仕事を行う処理 for 二倍にする処理 {
    type 仕事 = u32;
    type 結果 = u32;

    #[allow(clippy::panic)]
    fn 行う(&mut self, 仕事: u32) -> u32 {
        if 仕事 == 0 {
            panic!("試験のために異常終了する");
        }
        仕事 * 2
    }
}

fn 結果がそろうまで受け取る(
    裏: &mut 仕事を順に行う裏のスレッド<二倍にする処理>,
) -> Vec<u32> {
    let 始め = Instant::now();
    let mut 届いた = Vec::new();
    while 裏.返事を待っている数() > 0 {
        assert!(始め.elapsed() < Duration::from_secs(10), "結果が届かない");
        std::thread::sleep(Duration::from_millis(2));
        届いた.extend(裏.届いた結果を受け取る());
    }
    届いた
}

#[test]
fn 頼んだ順に結果が届く() {
    let mut 裏 = 仕事を順に行う裏のスレッド::起こす(二倍にする処理);
    for 数 in [1, 2, 3] {
        裏.頼む(数).expect("頼める");
    }
    assert_eq!(結果がそろうまで受け取る(&mut 裏), vec![2, 4, 6]);
    assert!(!裏.止まったか());
}

#[test]
fn 異常終了したら返事を待つのをやめ_次の頼みを断る() {
    let mut 裏 = 仕事を順に行う裏のスレッド::起こす(二倍にする処理);
    裏.頼む(0).expect("頼める");
    // 止まったと分かる前なら頼めることも、既に止まっていて断られることもある。どちらでも結果は届かない。
    let _ = 裏.頼む(5);
    let 届いた = 結果がそろうまで受け取る(&mut 裏);
    assert!(届いた.is_empty(), "異常終了した後の仕事は行わない");
    assert!(裏.止まったか());
    assert!(裏.頼む(1).is_err(), "止まったスレッドには頼めない");
}
