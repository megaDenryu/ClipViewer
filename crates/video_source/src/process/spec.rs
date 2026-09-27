//! ffmpeg へのデコードの指定と、その引数の組み立て。

use std::ffi::OsString;

use clip_domain::{動画上の秒, 正規化した動画パス};

use crate::estimate::区間の見積もり;
use crate::frame_number::{コマ数, 動画上のコマ番号};
use crate::frame_rate::コマの速さ;
use crate::frame_size::コマの寸法;
use crate::long_side::長辺の上限;
use crate::probe::流れの番号;
use crate::read_position::コマを読む位置;
use crate::video_info::動画の情報;

/// 読むコマ数とは、デコードで読むコマの枚数を決めているかの区別のことである。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum 読むコマ数 {
    /// 指定の枚数まで読む。区間を溜めるときに使う。
    枚数(コマ数),
    /// 動画の終わりまで読む。流し読みで使う。
    終わりまで,
}

/// デコードの指定とは、ffmpeg に読ませる動画と映像の流れ・先頭のコマ・枚数・出力の寸法・コマの速さの組のことである。
/// 可変フレームレートの動画も、fps フィルタでコマの速さ(ffprobe の平均)を一定にそろえてから出させる。
/// これにより、出力のi番目のコマは動画上のコマ番号「先頭のコマ + i」に当たる(丸めの置き方と隙間の埋め方は `コマを読む位置`)。
#[derive(Debug, Clone)]
pub(crate) struct デコードの指定 {
    pub(crate) 動画: 正規化した動画パス,
    pub(crate) 映像の流れ: 流れの番号,
    pub(crate) 先頭のコマ: 動画上のコマ番号,
    pub(crate) 読む枚数: 読むコマ数,
    pub(crate) 出力の寸法: コマの寸法,
    pub(crate) コマの速さ: コマの速さ,
}

impl デコードの指定 {
    /// 見積もった区間を読む指定を作る。
    pub(crate) fn 区間を読む(
        動画: &動画の情報, 見積もり: &区間の見積もり
    ) -> Self {
        Self {
            動画: 動画.パス().clone(),
            映像の流れ: 動画.映像の流れ(),
            先頭のコマ: 見積もり.先頭のコマ(),
            読む枚数: 読むコマ数::枚数(見積もり.コマ数()),
            出力の寸法: 見積もり.寸法(),
            コマの速さ: 動画.コマの速さ(),
        }
    }

    /// 開始の時刻を含むコマから動画の終わりまでを流し読みする指定を作る。
    pub(crate) fn 終わりまで流し読む(
        動画: &動画の情報,
        開始: 動画上の秒,
        上限: 長辺の上限,
    ) -> Self {
        Self {
            動画: 動画.パス().clone(),
            映像の流れ: 動画.映像の流れ(),
            先頭のコマ: 動画.コマの速さ().時刻を含むコマ番号(開始),
            読む枚数: 読むコマ数::終わりまで,
            出力の寸法: 動画.元の寸法().長辺の上限へ縮める(上限),
            コマの速さ: 動画.コマの速さ(),
        }
    }

    /// ffmpeg へ渡す引数を並べる。-ss を -i の前に置き、キーフレームから目当ての位置までをデコードして捨てさせる。
    pub(crate) fn 引数を並べる(&self) -> Vec<OsString> {
        let 読む位置 = コマを読む位置::求める(self.コマの速さ, self.先頭のコマ);
        let mut 引数: Vec<OsString> = ["-hide_banner", "-nostdin", "-loglevel", "error", "-ss"]
            .into_iter()
            .map(OsString::from)
            .collect();
        引数.push(読む位置.シーク位置の表記().into());
        if let 読むコマ数::枚数(枚数) = self.読む枚数 {
            let 読む長さ = self.コマの速さ.コマ数の長さ(枚数.一つ増やす());
            引数.extend(["-t".into(), format!("{:.6}", 読む長さ.秒数()).into()]);
        }
        引数.extend(["-i".into(), OsString::from(self.動画.文字列())]);
        let フィルタ = format!(
            "{},scale={}",
            読む位置.コマをそろえるフィルタ(),
            self.出力の寸法.拡大縮小の指定()
        );
        引数.extend(["-map".into(), self.映像の流れ.読む流れの指定().into()]);
        引数.extend(["-an", "-sn", "-dn", "-vf"].map(OsString::from));
        引数.push(フィルタ.into());
        if let 読むコマ数::枚数(枚数) = self.読む枚数 {
            引数.extend(["-frames:v".into(), 枚数.値().to_string().into()]);
        }
        引数.extend(["-f", "rawvideo", "-pix_fmt", "rgba", "-"].map(OsString::from));
        引数
    }
}
