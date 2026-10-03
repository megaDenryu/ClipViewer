//! 測ったフレームの時間。測っている間のフレームごとに、eframe が知らせた1フレームの時間と前のフレームからの間隔を溜め、
//! 数・平均・中央値・95百分位・最大のまとめの文にする。参照: _doc/設計/同時再生.md 5-4

use std::time::Duration;

/// 測ったフレームの時間とは、測っている間のフレームごとの1フレームの時間の並びと、前のフレームからの間隔の並びの組のことである。
/// 1フレームの時間は eframe の `Frame::info().cpu_usage` であり、入力の処理から、画面を組み、描く形を作り、テクスチャを GPU へ載せて描く命令を出すまでの時間である。
/// 垂直同期の待ち(画面の切り替えを待つ時間)は含まない。前のフレームからの間隔はそれを含む。
#[derive(Debug, Default)]
pub(crate) struct 測ったフレームの時間 {
    一フレームの時間: Vec<Duration>,
    間隔: Vec<Duration>,
}

/// 時間の並びのまとめとは、並びの数と、平均と、中央値と、95百分位(小さい方から数えて95%目の値)と、最大の組のことである。
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct 時間の並びのまとめ {
    pub(crate) 数: usize,
    pub(crate) 平均: Duration,
    pub(crate) 中央値: Duration,
    pub(crate) 九十五百分位: Duration,
    pub(crate) 最大: Duration,
}

impl 測ったフレームの時間 {
    /// 1フレーム分を足す。eframe が1フレームの時間をまだ知らせていなければ、間隔だけを足す。
    pub(crate) fn 足す(&mut self, 一フレームの時間: Option<Duration>, 間隔: Duration) {
        self.一フレームの時間.extend(一フレームの時間);
        self.間隔.push(間隔);
    }

    /// 1フレームの時間と間隔のまとめの文(2行)。
    pub(crate) fn まとめの文(&self) -> String {
        format!(
            "1フレームの時間(GPU へ載せる時間を含み、垂直同期の待ちを含まない): {}\n前のフレームからの間隔: {}",
            まとめを文にする(時間の並びのまとめ::並びから求める(
                &self.一フレームの時間
            )),
            まとめを文にする(時間の並びのまとめ::並びから求める(
                &self.間隔
            )),
        )
    }
}

impl 時間の並びのまとめ {
    /// 並びのまとめを求める。並びが空なら無い。
    pub(crate) fn 並びから求める(並び: &[Duration]) -> Option<Self> {
        let mut 並べた = 並び.to_vec();
        並べた.sort_unstable();
        let 最大 = *並べた.last()?;
        let 数 = 並べた.len();
        let 合計: Duration = 並べた.iter().sum();
        Some(Self {
            数,
            平均: 合計 / u32::try_from(数).ok()?,
            中央値: 並べた[(数 - 1) / 2],
            九十五百分位: 並べた[(数 * 95).div_ceil(100) - 1],
            最大,
        })
    }
}

fn まとめを文にする(まとめ: Option<時間の並びのまとめ>) -> String {
    let ミリ秒 = |時間: Duration| 時間.as_secs_f64() * 1000.0;
    まとめ.map_or_else(
        || "測れなかった(0フレーム)".to_string(),
        |まとめ| {
            format!(
                "{}フレーム 平均 {:.2}ミリ秒 中央値 {:.2}ミリ秒 95百分位 {:.2}ミリ秒 最大 {:.2}ミリ秒",
                まとめ.数,
                ミリ秒(まとめ.平均),
                ミリ秒(まとめ.中央値),
                ミリ秒(まとめ.九十五百分位),
                ミリ秒(まとめ.最大),
            )
        },
    )
}
