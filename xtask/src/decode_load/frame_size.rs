//! 画素の寸法の型と、アプリと同じ規則で読み出すコマの寸法を決める計算。

use std::fmt;

use super::byte_count::バイト数;

/// 読み出すコマの長辺の上限(画素)。video_source の `長辺の上限::既定` と同じ値である。
const 長辺の上限: u32 = 1280;
/// RGBA の1画素のバイト数。
const 一画素のバイト数: u32 = 4;

/// 画素の寸法とは、コマの幅と高さを画素の数(どちらも1以上)で表したもののことである。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct 画素の寸法 {
    幅: u32,
    高さ: u32,
}

impl 画素の寸法 {
    /// 幅と高さから作成する。どちらかが0なら作らない。
    pub fn 作成する(幅: u32, 高さ: u32) -> Option<Self> {
        (幅 > 0 && 高さ > 0).then_some(Self { 幅, 高さ })
    }

    /// 幅と高さを入れ替えた寸法。90度・270度の回転の情報を持つ動画の、表示される向きの寸法を求めるときに使う。
    pub fn 縦横を入れ替える(self) -> Self {
        Self {
            幅: self.高さ,
            高さ: self.幅,
        }
    }

    /// 縦横比を保ち、長辺が上限(1280画素)に収まるよう縮めた寸法。収まっていれば拡大しない。短辺は四捨五入し、1画素を下回らない。
    /// 参照: 丸めの規則は crates/video_source/src/frame_size.rs の `コマの寸法::長辺の上限へ縮める` と同じである。
    pub fn 長辺を上限へ縮める(self) -> Self {
        let 長辺 = u64::from(self.幅.max(self.高さ));
        if 長辺 <= u64::from(長辺の上限) {
            return self;
        }
        let 縮める = |辺: u32| {
            let 縮めた辺 = (u64::from(辺) * u64::from(長辺の上限) * 2 + 長辺) / (長辺 * 2);
            u32::try_from(縮めた辺).unwrap_or(長辺の上限).max(1)
        };
        Self {
            幅: 縮める(self.幅),
            高さ: 縮める(self.高さ),
        }
    }

    /// RGBA で読み出した1コマのバイト数。
    pub fn 一コマのバイト数(self) -> バイト数 {
        バイト数::画素の数から求める(self.幅, self.高さ, 一画素のバイト数)
    }

    /// ffmpeg の scale フィルタへ渡す「幅:高さ」の指定。
    pub fn 拡大縮小の指定(self) -> String {
        format!("{}:{}", self.幅, self.高さ)
    }
}

impl fmt::Display for 画素の寸法 {
    fn fmt(&self, 書き先: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(書き先, "{}x{}", self.幅, self.高さ)
    }
}
