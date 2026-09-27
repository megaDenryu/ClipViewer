//! ffmpeg へ音を読ませる指定と、その引数の組み立て。

use std::ffi::OsString;

use audio_pcm::{サンプリング周波数, 動画上の標本位置, 標本数};
use clip_domain::正規化した動画パス;

/// 読む標本数とは、音のデコードで読む標本の数を決めているかの区別のことである。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum 読む標本数 {
    /// 指定の数まで読む。区間を溜めるときに使う。
    数(標本数),
    /// 動画の終わりまで読む。流し読みで使う。
    終わりまで,
}

/// 音のデコードの指定とは、ffmpeg に読ませる動画・最初の標本の位置・読む標本数・出させる周波数の組のことである。
/// 出力のi番目の標本は、動画上の標本位置「先頭 + i」に当たる。
#[derive(Debug, Clone)]
pub(crate) struct 音のデコードの指定 {
    pub(crate) 動画: 正規化した動画パス,
    pub(crate) 先頭: 動画上の標本位置,
    pub(crate) 読む数: 読む標本数,
    pub(crate) 周波数: サンプリング周波数,
}

impl 音のデコードの指定 {
    /// ffmpeg へ渡す引数を並べる。-ss を -i の前に置く。音はどのフレームからでも復号でき、
    /// ffmpeg は -ss より前の標本を切り捨てるため、出力の最初の標本は先頭の位置に当たる。
    /// 最初の音の流れだけを、2チャンネル・出力装置の周波数・f32 のリトルエンディアンで標準出力へ出させる。
    /// 数を決めて読むときは、秒の丸めで足りなくならないよう10ミリ秒だけ長く読ませ、読む側が数で切る。
    pub(crate) fn 引数を並べる(&self) -> Vec<OsString> {
        let 開始の秒 = self.周波数.標本位置の時刻(self.先頭).秒数();
        let mut 引数: Vec<OsString> = ["-hide_banner", "-nostdin", "-loglevel", "error", "-ss"]
            .into_iter()
            .map(OsString::from)
            .collect();
        引数.push(format!("{開始の秒:.6}").into());
        if let 読む標本数::数(数) = self.読む数 {
            let 読む秒 = self
                .周波数
                .標本数の長さ(数.足す(余分に読む数(self.周波数)))
                .秒数();
            引数.extend(["-t".into(), format!("{読む秒:.6}").into()]);
        }
        引数.extend(["-i".into(), OsString::from(self.動画.文字列())]);
        引数.extend(["-vn", "-sn", "-dn", "-map", "0:a:0", "-ac", "2", "-ar"].map(OsString::from));
        引数.push(self.周波数.毎秒の標本数().to_string().into());
        引数.extend(["-f", "f32le", "-"].map(OsString::from));
        引数
    }
}

/// 数を決めて読むときに余分に読ませる標本の数(10ミリ秒分)。
fn 余分に読む数(周波数: サンプリング周波数) -> 標本数 {
    標本数::作成する(周波数.毎秒の標本数() / 100)
}
