//! コマの速さの型。動画の性質としての再生の速さを表し、測った `読み出しの速さ` とは区別する。

/// コマの速さとは、動画が再生のときに1秒あたりに表示するコマの数(動画の性質)を、分子÷分母の分数で表したもののことである。
/// 不変条件: 分子と分母はどちらも1以上である。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct コマの速さ {
    分子: u32,
    分母: u32,
}

impl コマの速さ {
    /// 分子と分母から作成する。どちらかが0なら作らない。
    pub fn 作成する(分子: u32, 分母: u32) -> Option<Self> {
        (分子 > 0 && 分母 > 0).then_some(Self { 分子, 分母 })
    }

    /// ffprobe の「30000/1001」または「30」の表記から読む。0/0 のような読めない表記なら無い。
    /// 参照: crates/video_source/src/frame_rate.rs の `コマの速さ::分数の表記から読む` と同じ読み方である。
    pub fn 分数の表記から読む(表記: &str) -> Option<Self> {
        let (分子, 分母) = 表記.split_once('/').unwrap_or((表記, "1"));
        Self::作成する(分子.trim().parse().ok()?, 分母.trim().parse().ok()?)
    }

    /// 1秒あたりのコマの数。
    pub fn 一秒あたりのコマ数(self) -> f64 {
        f64::from(self.分子) / f64::from(self.分母)
    }

    /// 人が読む1秒あたりのコマの数の表記。割り切れるなら整数で、割り切れなければ小数2桁で書く。
    pub fn 一秒あたりのコマ数の表記(self) -> String {
        if self.分子.is_multiple_of(self.分母) {
            (self.分子 / self.分母).to_string()
        } else {
            format!("{:.2}", self.一秒あたりのコマ数())
        }
    }

    /// fps フィルタへ渡す分数の表記。
    pub fn 分数の表記(self) -> String {
        format!("{}/{}", self.分子, self.分母)
    }
}
