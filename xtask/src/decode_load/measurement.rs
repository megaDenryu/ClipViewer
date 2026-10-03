//! 1回の測定の結果(n本を同時に読み切った時間とコマ数)と、そこから求める速さ。

use std::time::Duration;

use super::stream_count::同時に起動するffmpegの数;
use super::video_shape::コマの速さ;

/// 読み出しの速さとは、1秒あたりに読み出したコマの数のことである。
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct 読み出しの速さ(f64);

impl 読み出しの速さ {
    /// 1秒あたりのコマの数。
    pub fn 値(self) -> f64 {
        self.0
    }

    /// 動画の再生の速さ(コマの速さ)の何倍で読み出せたか。
    pub fn 再生の速さに対する倍率(self, 再生の速さ: コマの速さ) -> f64 {
        self.0 / 再生の速さ.毎秒のコマ数()
    }
}

/// 一回の測定とは、同じ動画を n 本の ffmpeg で同時に読み切ったときの、ffmpegの数と読み切る時間と1本が読んだコマ数の組のことである。
/// 読み切る時間は、すべての ffmpeg を起動する直前から、すべてが終わるまでの時間である。
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct 一回の測定 {
    ffmpegの数: 同時に起動するffmpegの数,
    読み切る時間: Duration,
    一本あたりのコマ数: u32,
}

impl 一回の測定 {
    /// 測った値から作る。時間かコマ数が0なら速さを求められないため、理由を付けて失敗にする。
    pub fn 作る(
        ffmpegの数: 同時に起動するffmpegの数,
        読み切る時間: Duration,
        一本あたりのコマ数: u32,
    ) -> Result<Self, String> {
        if 読み切る時間.is_zero() || 一本あたりのコマ数 == 0 {
            return Err(format!(
                "{}本の測定で速さを求められない(時間 {読み切る時間:?}、1本あたり {一本あたりのコマ数}コマ)",
                ffmpegの数.値()
            ));
        }
        Ok(Self {
            ffmpegの数,
            読み切る時間,
            一本あたりのコマ数,
        })
    }

    /// 同時に読んだffmpegの数。
    pub fn ffmpegの数(&self) -> 同時に起動するffmpegの数 {
        self.ffmpegの数
    }

    /// すべてを読み切るまでの時間。
    pub fn 読み切る時間(&self) -> Duration {
        self.読み切る時間
    }

    /// 1本が読んだコマ数。
    pub fn 一本あたりのコマ数(&self) -> u32 {
        self.一本あたりのコマ数
    }

    /// n 本が読んだコマの合計を、読み切る時間で割った速さ。
    pub fn 合計の速さ(&self) -> 読み出しの速さ {
        let 合計 = f64::from(self.一本あたりのコマ数) * f64::from(self.ffmpegの数.値());
        読み出しの速さ(合計 / self.読み切る時間.as_secs_f64())
    }

    /// 1本あたりの速さ。合計の速さをffmpegの数で割ったものである。
    pub fn 一本あたりの速さ(&self) -> 読み出しの速さ {
        読み出しの速さ(self.合計の速さ().0 / f64::from(self.ffmpegの数.値()))
    }
}
