//! 依頼の台帳の試験の共通の道具。
#![allow(clippy::expect_used)]

use std::time::{Duration, Instant};

use clip_domain::{動画上の区間, 時刻};

use super::ledger::落ち着くまでの時間;

pub(super) fn 区間(開始: f64, 終了: f64) -> 動画上の区間 {
    let 秒 = |値| 時刻::作成する(値).expect("時刻を作れない");
    動画上の区間::作成する(秒(開始), 秒(終了)).expect("区間を作れない")
}

pub(super) fn 落ち着いた後(始め: Instant) -> Instant {
    始め + 落ち着くまでの時間 + Duration::from_millis(1)
}
