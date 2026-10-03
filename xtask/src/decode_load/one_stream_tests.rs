//! 1本の読み方の試験。ffmpeg へ渡す引数がアプリの流し読みと同じ形(選んだ映像の流れ・fps フィルタ・長辺1280画素・RGBA)になることを確かめる。
#![allow(clippy::expect_used)]

use std::path::PathBuf;

use super::one_stream::一本の読み方;
use super::read_length::読む秒数;
use super::video_file::測る動画のファイル;
use super::video_shape::動画の形;

#[test]
fn 先頭から指定の秒数をアプリと同じ形で読む引数を並べる() {
    let 動画 = 測る動画のファイル::与えられたファイル(PathBuf::from("動画.mp4"));
    let 出力 = "streams.stream.0.index=0\nstreams.stream.0.codec_type=\"audio\"\n\
        streams.stream.1.index=1\nstreams.stream.1.codec_type=\"video\"\nstreams.stream.1.width=3840\n\
        streams.stream.1.height=2160\nstreams.stream.1.avg_frame_rate=\"60000/1001\"\n";
    let 読み方 = 一本の読み方 {
        動画: &動画,
        形: 動画の形::ffprobeの出力から読む(出力).expect("読める"),
        読む長さ: 読む秒数::作成する(12).expect("作れる"),
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
            "0:1",
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
    assert_eq!(読み方.一コマのバイト数().to_string(), "3686400バイト");
}
