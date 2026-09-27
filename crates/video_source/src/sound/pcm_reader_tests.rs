//! 左右の値の読み手の試験。1回の読み取りで足す標本の数が上限を超えないこと(音の倉庫が確保する容量の前提)と、
//! 1標本の途中で切れたバイトを次の読み取りへ持ち越すこと。

#![allow(clippy::expect_used)]

use std::io::{Cursor, Read};

use super::pcm_reader::{
    一回に足す標本数の上限, 左右の値の読み手, 読んだ結果
};

/// 1回の読み取りで、決まったバイト数までしか返さない入力。
struct 少しずつ返す入力 {
    中身: Cursor<Vec<u8>>,
    一回の長さ: usize,
}

impl Read for 少しずつ返す入力 {
    fn read(&mut self, 書き先: &mut [u8]) -> std::io::Result<usize> {
        let 長さ = 書き先.len().min(self.一回の長さ);
        self.中身.read(&mut 書き先[..長さ])
    }
}

#[test]
fn 一回に足す標本の数は上限を超えず端数のバイトを持ち越す() {
    let 標本の数 = 10_000;
    let バイト: Vec<u8> = (0..標本の数)
        .flat_map(|番号: u16| {
            let 値 = f32::from(番号).to_le_bytes();
            [値, 値].concat()
        })
        .collect();
    let 入力 = 少しずつ返す入力 {
        中身: Cursor::new(バイト),
        一回の長さ: 16 * 1024 + 3,
    };
    let mut 読み手 = 左右の値の読み手::作成する(入力);
    let mut 書き先 = Vec::new();
    loop {
        let 前の長さ = 書き先.len();
        match 読み手.次の塊を読む(&mut 書き先).expect("読める") {
            読んだ結果::読んだ => {
                assert!(書き先.len() - 前の長さ <= 一回に足す標本数の上限)
            }
            読んだ結果::終わった => break,
        }
    }
    assert_eq!(書き先.len(), usize::from(標本の数));
    assert!(
        書き先
            .iter()
            .enumerate()
            .all(|(番号, 値)| f64::from(値[0]) == f64::from(u16::try_from(番号).expect("番号")))
    );
}
