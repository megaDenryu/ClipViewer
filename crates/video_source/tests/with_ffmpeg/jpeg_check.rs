//! 作った JPEG を調べる道具。JPEG として読めることと寸法と色を、ffprobe と ffmpeg に読ませて確かめる
//! (試験のために画像の復号器を依存へ足さない)。

use std::path::PathBuf;
use std::process::Command;

use crate::common::実行ファイルを探す;

/// JPEG のファイルを ffprobe に読ませ、幅と高さを返す。
pub(crate) fn 寸法を調べる(パス: &PathBuf) -> (u32, u32) {
    let 出力 = Command::new(実行ファイルを探す().調査のパス())
        .args([
            "-v",
            "error",
            "-show_entries",
            "stream=width,height",
            "-of",
            "csv=p=0",
        ])
        .arg(パス)
        .output()
        .expect("ffprobe を起動できる");
    assert!(出力.status.success(), "JPEG として読めない");
    let 本文 = String::from_utf8_lossy(&出力.stdout);
    let (幅, 高さ) = 本文.trim().split_once(',').expect("幅と高さ");
    (幅.parse().expect("幅"), 高さ.parse().expect("高さ"))
}

/// JPEG の最初の画素を ffmpeg で RGBA へ戻す。
pub(crate) fn 最初の画素(パス: &PathBuf) -> Vec<u8> {
    let 出力 = Command::new(実行ファイルを探す().変換のパス())
        .args(["-v", "error", "-i"])
        .arg(パス)
        .args([
            "-vf",
            "format=rgba,crop=1:1:0:0",
            "-f",
            "rawvideo",
            "-pix_fmt",
            "rgba",
            "-",
        ])
        .output()
        .expect("ffmpeg を起動できる");
    assert!(出力.status.success(), "JPEG を画素へ戻せない");
    出力.stdout
}

/// JPEG の全体の画素を ffmpeg で RGBA へ戻す。寸法は `寸法を調べる` で別に調べる。
pub(crate) fn 全体の画素(パス: &PathBuf) -> Vec<u8> {
    let 出力 = Command::new(実行ファイルを探す().変換のパス())
        .args(["-v", "error", "-i"])
        .arg(パス)
        .args(["-f", "rawvideo", "-pix_fmt", "rgba", "-"])
        .output()
        .expect("ffmpeg を起動できる");
    assert!(出力.status.success(), "JPEG を画素へ戻せない");
    出力.stdout
}
