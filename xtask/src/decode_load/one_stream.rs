//! 1本の ffmpeg の読み方。アプリの流し読み(crates/video_source/src/process/spec.rs の `デコードの指定::引数を並べる`)と同じ形で、
//! fps フィルタでコマの速さをそろえ、長辺1280画素へ縮め、RGBA の生データで標準出力へ出す。
//! アプリと違い、先頭から読むため -ss を置かず、読む長さを -t で限る。

use std::ffi::OsString;
use std::path::Path;

use super::read_length::読む秒数;
use super::video_shape::動画の形;

/// 一本の読み方とは、1本の ffmpeg が読む動画と、その動画の形と、先頭から読む長さの組のことである。
pub struct 一本の読み方<'a> {
    pub 動画: &'a Path,
    pub 形: 動画の形,
    pub 読む長さ: 読む秒数,
}

impl 一本の読み方<'_> {
    /// ffmpeg へ渡す引数を並べる。
    pub fn 引数を並べる(&self) -> Vec<OsString> {
        let 出力の寸法 = self.形.元の寸法.長辺を上限へ縮める();
        let フィルタ = format!(
            "fps={}:round=down:start_time=0,scale={}:{}",
            self.形.コマの速さ.分数の表記(),
            出力の寸法.幅,
            出力の寸法.高さ
        );
        let mut 引数: Vec<OsString> = ["-hide_banner", "-nostdin", "-loglevel", "error", "-t"]
            .map(OsString::from)
            .to_vec();
        引数.push(self.読む長さ.値().to_string().into());
        引数.extend(["-i".into(), self.動画.as_os_str().to_owned()]);
        引数.extend(["-map", "0:v:0", "-an", "-sn", "-dn", "-vf"].map(OsString::from));
        引数.push(フィルタ.into());
        引数.extend(["-f", "rawvideo", "-pix_fmt", "rgba", "-"].map(OsString::from));
        引数
    }

    /// 読み出す1コマのバイト数。
    pub fn 一コマのバイト数(&self) -> u64 {
        self.形.元の寸法.長辺を上限へ縮める().一コマのバイト数()
    }
}
