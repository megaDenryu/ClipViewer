//! 流れの番号と音の有無の型。

/// 流れの番号とは、動画のファイルの中の流れ(映像・音・字幕など)に、ファイルの中の順に0から振った通し番号のことである。
/// ffprobe の index であり、ffmpeg へ `-map 0:<番号>` で渡すと、ffprobe で調べた流れと同じ流れを読ませられる。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub(crate) struct 流れの番号(u32);

impl 流れの番号 {
    pub(crate) fn 作成する(番号: u32) -> Self {
        Self(番号)
    }

    /// ffmpeg の -map へ渡す、最初の入力のこの流れの指定。
    pub(crate) fn 読む流れの指定(self) -> String {
        format!("0:{}", self.0)
    }
}

/// 音の有無とは、動画のファイルに音の流れ(オーディオストリーム)があるかの区別のことである。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum 音の有無 {
    /// 音がある。最初の音の流れを読む。
    ある,
    /// 音が無い。映像だけを再生する。
    無い,
}
