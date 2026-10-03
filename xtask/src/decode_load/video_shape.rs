//! 測る動画の形を ffprobe で調べる。アプリと同じく、添付の画像を除いた最初の映像の流れを使う。

use super::frame_rate::コマの速さ;
use super::frame_size::画素の寸法;
use super::probed_stream::調べた流れ;
use super::video_file::測る動画のファイル;
use crate::ffmpeg_location::FFmpegの置き場所;

/// 動画の形とは、測る動画の再生する映像の流れの、番号と表示される向きの寸法とコマの速さの組のことである。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct 動画の形 {
    映像の流れの番号: u32,
    表示される寸法: 画素の寸法,
    コマの速さ: コマの速さ,
}

impl 動画の形 {
    /// ffprobe で動画の流れを調べる。
    /// 注意: `-show_entries` に side_data を頼むと、ffprobe はパケットの副データまで読みに行き、大きな動画で終わらなくなる。
    /// そのためアプリと同じく `-show_streams` で流れの全項目を出させ、使う項目だけを読む。
    pub fn 調べる(
        ffmpegの置き場所: &FFmpegの置き場所,
        動画: &測る動画のファイル,
    ) -> Result<Self, String> {
        let 出力 = ffmpegの置き場所
            .ffprobeの命令()
            .args(["-v", "error", "-show_streams", "-of", "flat"])
            .arg(動画.引数の表記())
            .output()
            .map_err(|原因| format!("ffprobe の起動に失敗した: {原因}"))?;
        if !出力.status.success() {
            let 理由 = String::from_utf8_lossy(&出力.stderr);
            return Err(format!("ffprobe が {動画} を調べられなかった: {理由}"));
        }
        Self::ffprobeの出力から読む(&String::from_utf8_lossy(&出力.stdout))
    }

    /// ffprobe の flat の出力から、添付の画像を除いた最初の映像の流れを選んで読む。読めない値があれば、その項目を挙げて失敗にする。
    pub fn ffprobeの出力から読む(出力: &str) -> Result<Self, String> {
        let 並び = 調べた流れ::flatの出力から並びを読む(出力)?;
        let 映像 = 並び
            .iter()
            .find(|流れ| 流れ.再生する映像か())
            .ok_or_else(|| "再生する映像の流れが無い".to_owned())?;
        let 読めないと伝える = |項目: &str| format!("映像の流れの{項目}を読めない");
        Ok(Self {
            映像の流れの番号: 映像
                .番号()
                .ok_or_else(|| 読めないと伝える("番号"))?,
            表示される寸法: 映像
                .表示される寸法()
                .ok_or_else(|| 読めないと伝える("寸法"))?,
            コマの速さ: 映像
                .コマの速さを選ぶ()
                .ok_or_else(|| 読めないと伝える("コマの速さ"))?,
        })
    }

    /// ffmpeg の -map へ渡す、最初の入力のこの映像の流れの指定。
    pub fn 読む流れの指定(&self) -> String {
        format!("0:{}", self.映像の流れの番号)
    }

    /// 表示される向きの寸法。
    pub fn 表示される寸法(&self) -> 画素の寸法 {
        self.表示される寸法
    }

    /// アプリと同じ規則で縮めた、読み出すコマの寸法。
    pub fn 出力の寸法(&self) -> 画素の寸法 {
        self.表示される寸法.長辺を上限へ縮める()
    }

    /// 動画のコマの速さ。
    pub fn コマの速さ(&self) -> コマの速さ {
        self.コマの速さ
    }
}
