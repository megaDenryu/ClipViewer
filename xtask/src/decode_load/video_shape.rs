//! 測る動画の形(元の寸法とコマの速さ)を ffprobe で調べ、アプリと同じ規則で読み出すコマの寸法を決める。

use std::path::Path;

use crate::ffmpeg_location::FFmpegの置き場所;

/// 読み出すコマの長辺の上限(画素)。video_source の `長辺の上限::既定` と同じ値である。
const 長辺の上限: u32 = 1280;
/// RGBA の1画素のバイト数。
const 一画素のバイト数: u64 = 4;

/// 画素の寸法とは、コマの幅と高さを画素の数(1以上)で表したもののことである。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct 画素の寸法 {
    pub 幅: u32,
    pub 高さ: u32,
}

impl 画素の寸法 {
    /// 縦横比を保ち、長辺が上限(1280画素)に収まるよう縮めた寸法。収まっていれば拡大しない。短辺は四捨五入する。
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
    pub fn 一コマのバイト数(self) -> u64 {
        u64::from(self.幅) * u64::from(self.高さ) * 一画素のバイト数
    }
}

/// コマの速さとは、1秒あたりのコマの数を分数(分子/分母。どちらも1以上)で表したもののことである。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct コマの速さ {
    pub 分子: u32,
    pub 分母: u32,
}

impl コマの速さ {
    /// 1秒あたりのコマの数。
    pub fn 毎秒のコマ数(self) -> f64 {
        f64::from(self.分子) / f64::from(self.分母)
    }

    /// 人が読む1秒あたりのコマの数の表記。割り切れるなら整数で、割り切れなければ小数2桁で書く。
    pub fn 毎秒のコマ数の表記(self) -> String {
        if self.分子.is_multiple_of(self.分母) {
            (self.分子 / self.分母).to_string()
        } else {
            format!("{:.2}", self.毎秒のコマ数())
        }
    }

    /// fps フィルタへ渡す分数の表記。
    pub fn 分数の表記(self) -> String {
        format!("{}/{}", self.分子, self.分母)
    }
}

/// 動画の形とは、測る動画の最初の映像の流れの元の寸法とコマの速さ(ffprobe の平均)の組のことである。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct 動画の形 {
    pub 元の寸法: 画素の寸法,
    pub コマの速さ: コマの速さ,
}

impl 動画の形 {
    /// ffprobe で動画の最初の映像の流れを調べる。
    pub fn 調べる(
        ffmpegの置き場所: &FFmpegの置き場所, 動画: &Path
    ) -> Result<Self, String> {
        let 出力 = ffmpegの置き場所
            .ffprobeの命令()
            .args(["-v", "error", "-select_streams", "v:0", "-show_entries"])
            .args(["stream=width,height,avg_frame_rate", "-of", "csv=p=0"])
            .arg(動画)
            .output()
            .map_err(|原因| format!("ffprobe の起動に失敗した: {原因}"))?;
        if !出力.status.success() {
            let 理由 = String::from_utf8_lossy(&出力.stderr);
            return Err(format!(
                "ffprobe が {} を調べられなかった: {理由}",
                動画.display()
            ));
        }
        Self::ffprobeの出力から読む(&String::from_utf8_lossy(&出力.stdout))
    }

    /// ffprobe の `幅,高さ,分子/分母` の1行を読む。読めない出力は理由を付けて失敗にする。
    pub fn ffprobeの出力から読む(出力: &str) -> Result<Self, String> {
        let 行 = 出力.lines().next().unwrap_or_default().trim();
        let 読めないと伝える =
            || format!("ffprobe の出力「{行}」から幅・高さ・コマの速さを読めない");
        let 欄: Vec<&str> = 行.split(',').collect();
        let [幅, 高さ, 速さ] = 欄.as_slice() else {
            return Err(読めないと伝える());
        };
        let (分子, 分母) = 速さ.split_once('/').ok_or_else(読めないと伝える)?;
        let 一以上の整数として読む = |表記: &str| {
            表記
                .parse::<u32>()
                .ok()
                .filter(|値| *値 > 0)
                .ok_or_else(読めないと伝える)
        };
        Ok(Self {
            元の寸法: 画素の寸法 {
                幅: 一以上の整数として読む(幅)?,
                高さ: 一以上の整数として読む(高さ)?,
            },
            コマの速さ: コマの速さ {
                分子: 一以上の整数として読む(分子)?,
                分母: 一以上の整数として読む(分母)?,
            },
        })
    }
}
