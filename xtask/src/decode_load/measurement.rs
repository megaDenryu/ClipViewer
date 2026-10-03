//! 1回の測定の結果(n 本を同時に読み切った時間とコマ数)と、そこから求める速さ。

use super::elapsed::読み切る時間;
use super::frame_count::コマ数;
use super::read_speed::読み出しの速さ;
use super::stream_count::同時に起動するffmpegの数;

/// 一回の測定とは、同じ動画を n 本の ffmpeg で同時に読み切ったときの、ffmpegの数と読み切る時間と1本が読んだコマ数の組のことである。
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct 一回の測定 {
    ffmpegの数: 同時に起動するffmpegの数,
    読み切る時間: 読み切る時間,
    一本あたりのコマ数: コマ数,
}

impl 一回の測定 {
    /// 測った値から作成する。時間とコマ数は型が0を含まないため、速さを必ず求められる。
    pub fn 作成する(
        ffmpegの数: 同時に起動するffmpegの数,
        読み切る時間: 読み切る時間,
        一本あたりのコマ数: コマ数,
    ) -> Self {
        Self {
            ffmpegの数,
            読み切る時間,
            一本あたりのコマ数,
        }
    }

    /// 同時に起動した ffmpeg の数。
    pub fn ffmpegの数(&self) -> 同時に起動するffmpegの数 {
        self.ffmpegの数
    }

    /// すべてを読み切るまでの時間。
    pub fn 読み切る時間(&self) -> 読み切る時間 {
        self.読み切る時間
    }

    /// 1本が読んだコマ数。
    pub fn 一本あたりのコマ数(&self) -> コマ数 {
        self.一本あたりのコマ数
    }

    /// n 本が読んだコマの合計を、読み切る時間で割った速さ。
    pub fn 合計の速さ(&self) -> 読み出しの速さ {
        self.一本あたりの速さ().ffmpegの数を掛ける(self.ffmpegの数)
    }

    /// 1本あたりの速さ。1本が読んだコマ数を、読み切る時間で割ったものである。
    pub fn 一本あたりの速さ(&self) -> 読み出しの速さ {
        読み出しの速さ::コマ数と時間から求める(
            self.一本あたりのコマ数,
            self.読み切る時間,
        )
    }
}
