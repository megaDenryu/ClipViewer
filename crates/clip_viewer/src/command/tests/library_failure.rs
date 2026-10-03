//! 保存に失敗した状態を作る試験の道具。スタックのファイルを排他で開いて置き換えられなくする(Windows だけで流す)。
#![cfg(windows)]
#![allow(clippy::expect_used)]

use std::os::windows::fs::OpenOptionsExt;
use std::time::{Duration, Instant};

use clip_domain::{スタックの識別子, トリガー};

use super::library_check::終えるまで待って当てる;
use super::library_support::試験のライブラリ;
use super::{クリップを作る, 編集する};
use crate::command::クリップの編集;
use crate::state::library::保存の段階;
use crate::state::アプリの状態;

/// 甲のクリップを登録済みにし、スタックのファイルを排他で開いたまま甲の名前を変えて、落ち着いた後の保存を失敗させる。
pub(super) fn 保存に失敗した状態(
    試験: &試験のライブラリ,
) -> (アプリの状態, スタックの識別子, std::fs::File, Instant) {
    let mut 状態 = 試験.接続した状態(vec![クリップを作る(
        "甲",
        0.0,
        1.0,
        トリガー::自動進行,
    )]);
    let 識別子 = 試験.登録済みにする(&mut 状態, "stack-1-1", "練習");
    let パス = 試験.フォルダ.パス().join("stack-1-1.json");
    let 排他 = std::fs::OpenOptions::new()
        .read(true)
        .share_mode(0)
        .open(パス)
        .expect("開ける");
    編集する(&mut 状態, クリップの編集::名前を変える("変えた".into()));
    let 始め = Instant::now();
    状態.ライブラリの書き込みを進める(始め);
    状態.ライブラリの書き込みを進める(始め + Duration::from_millis(600));
    終えるまで待って当てる(&mut 状態);
    (状態, 識別子, 排他, 始め)
}

pub(super) fn 段階(状態: &アプリの状態) -> Option<保存の段階<'_>> {
    状態.開いているスタックの保存の段階()
}
