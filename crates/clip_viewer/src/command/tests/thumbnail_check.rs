//! サムネイルの試験の道具。行の見せ方を読むことと、ライブラリの構えで毎フレーム見えている行を置きながら裏の結果がそろうまで進めること。
#![allow(clippy::expect_used)]

use std::ops::RangeInclusive;
use std::time::{Duration, Instant};

use crate::library_common::list::見えている行の範囲;
use crate::state::{アプリの状態, 画面の構え};
use clip_domain::スタックの識別子;

use crate::thumbnail_feed::サムネイルの見せ方;

pub(super) fn 見せ方(
    状態: &アプリの状態,
    識別子: &スタックの識別子,
) -> Option<サムネイルの見せ方> {
    状態
        .ライブラリ
        .サムネイル
        .項目の表()
        .get(識別子)
        .map(|項目| 項目.見せ方())
}

pub(super) fn 見せられるか(
    状態: &アプリの状態, 識別子: &スタックの識別子
) -> bool {
    matches!(
        見せ方(状態, 識別子),
        Some(サムネイルの見せ方::見せられる(_))
    )
}

/// テクスチャを載せている行の数。
pub(super) fn 見せられる数(状態: &アプリの状態) -> usize {
    状態
        .ライブラリ
        .サムネイル
        .項目の表()
        .values()
        .filter(|項目| matches!(項目.見せ方(), サムネイルの見せ方::見せられる(_)))
        .count()
}

/// 見せる順の番号の行の識別子。
pub(super) fn 見せる順の識別子(
    状態: &アプリの状態
) -> Vec<スタックの識別子> {
    状態
        .ライブラリ
        .一覧
        .行の並び(None)
        .見せる順
        .into_iter()
        .cloned()
        .collect()
}

/// ライブラリの構えにして、画面が毎フレーム知らせるのと同じく見えている行を置きながら、サムネイルの裏の仕事の返事がそろうまで進める。
pub(super) fn 見えている行で進める(
    状態: &mut アプリの状態,
    見えている: RangeInclusive<usize>,
) {
    状態.出力.構え = 画面の構え::ライブラリ;
    let 始め = Instant::now();
    loop {
        状態.ライブラリ.見えている行 =
            Some(見えている行の範囲::両端の行番号から作る(
                *見えている.start(),
                *見えている.end(),
            ));
        状態.サムネイルを進める();
        if !状態.ライブラリ.サムネイル.返事を待っているか() {
            return;
        }
        assert!(
            始め.elapsed() < Duration::from_secs(30),
            "サムネイルの結果が届かない"
        );
        std::thread::sleep(Duration::from_millis(2));
    }
}
