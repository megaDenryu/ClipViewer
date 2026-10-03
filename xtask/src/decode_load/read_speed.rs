//! 読み出しの速さの型。コマの速さ(動画の性質)と区別し、ffmpeg が実際に読み出せた速さ(測った値)を表す。

use std::fmt;

use super::elapsed::読み切る時間;
use super::frame_count::コマ数;
use super::frame_rate::コマの速さ;
use super::playback_ratio::再生の速さに対する倍率;
use super::stream_count::同時に起動するffmpegの数;

/// 読み出しの速さとは、ffmpeg が実際に1秒あたりに読み出したコマの数(測った値)のことである。
/// 動画が再生のときに1秒あたりに表示するコマの数(`コマの速さ`)とは別の量である。
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct 読み出しの速さ(f64);

impl 読み出しの速さ {
    /// 読んだコマ数を、読み切る時間で割る。
    pub fn コマ数と時間から求める(
        コマ数: コマ数, 時間: 読み切る時間
    ) -> Self {
        Self(コマ数.小数として() / 時間.秒数())
    }

    /// n 本の ffmpeg が同じ速さで読んだときの合計の速さ。
    pub fn ffmpegの数を掛ける(self, 数: 同時に起動するffmpegの数) -> Self {
        Self(self.0 * 数.小数として())
    }

    /// 動画の再生の速さの何倍で読み出せたか。
    pub fn 再生の速さに対する倍率(
        self,
        再生の速さ: コマの速さ,
    ) -> 再生の速さに対する倍率 {
        再生の速さに対する倍率::速さの比から作る(
            self.0 / 再生の速さ.一秒あたりのコマ数(),
        )
    }
}

impl fmt::Display for 読み出しの速さ {
    fn fmt(&self, 書き先: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(書き先, "約{:.0}コマ/秒", self.0)
    }
}
