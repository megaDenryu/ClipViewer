//! 1本の読み方の試験。ffmpeg へ渡す引数がアプリの流し読みと同じ形(fps フィルタ・長辺1280画素・RGBA の生データ)になることを確かめる。
#![allow(clippy::expect_used)]

use std::path::Path;

use super::one_stream::一本の読み方;
use super::read_length::読む秒数;
use super::video_shape::動画の形;

#[test]
fn 先頭から指定の秒数をアプリと同じ形で読む引数を並べる() {
    let 読む長さ = 読む秒数::表記から読む("12").expect("読める");
    let 読み方 = 一本の読み方 {
        動画: Path::new("動画.mp4"),
        形: 動画の形::ffprobeの出力から読む("3840,2160,60000/1001").expect("読める"),
        読む長さ,
    };
    let 並び: Vec<String> = 読み方
        .引数を並べる()
        .iter()
        .map(|一つ| 一つ.to_string_lossy().into_owned())
        .collect();
    assert_eq!(
        並び,
        [
            "-hide_banner",
            "-nostdin",
            "-loglevel",
            "error",
            "-t",
            "12",
            "-i",
            "動画.mp4",
            "-map",
            "0:v:0",
            "-an",
            "-sn",
            "-dn",
            "-vf",
            "fps=60000/1001:round=down:start_time=0,scale=1280:720",
            "-f",
            "rawvideo",
            "-pix_fmt",
            "rgba",
            "-",
        ]
    );
    assert_eq!(読み方.一コマのバイト数(), 1280 * 720 * 4);
}
