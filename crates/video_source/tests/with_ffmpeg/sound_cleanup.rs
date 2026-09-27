//! 音の倉庫の後始末の結合試験。読んでいる ffmpeg が何も出力しないまま固まっていても、倉庫を落とすと待たされずに返ることを確かめる。
//! 倉庫を落とす側が ffmpeg を止めないと、裏のスレッドは出力を待ち続けて終わらず、落とす側はその終わりを待ち続ける。

use std::path::PathBuf;
use std::sync::mpsc;
use std::time::Duration;

use clip_domain::時間の長さ;
use video_source::{音のメモリの上限, 音の溜め方};

use crate::common::{区間, 読み手};
use crate::sound_support::周波数;
use crate::stalling_server::止まる配信元;
use crate::test_videos::動画を作る;

#[test]
#[ignore = "FFmpeg が要る"]
fn 読んでいる途中で固まった音の倉庫を落としても待たされない() {
    let ファイル = 動画を作る(
        "音の倉庫を落とす.mkv",
        &[
            "-f",
            "lavfi",
            "-i",
            "color=c=black:s=16x16:r=1:d=10",
            "-f",
            "lavfi",
            "-i",
            "sine=frequency=440:sample_rate=48000:duration=10",
            "-c:v",
            "libx264",
            "-preset",
            "ultrafast",
            "-c:a",
            "pcm_s16le",
        ],
    );
    let 配信元 = 止まる配信元::起動する(PathBuf::from(ファイル.文字列()));
    let 読み手 = 読み手();
    let 情報 = 読み手
        .動画の情報を取る(&配信元.動画のパス())
        .expect("動画の情報を取れない");
    let 溜め方 = 音の溜め方 {
        周波数: 周波数(),
        開始の前の余白: 時間の長さ::ゼロ,
        上限: 音のメモリの上限::既定,
    };
    let 倉庫 = 読み手
        .音の倉庫を開く(&情報, 溜め方)
        .expect("音の倉庫を開けない");
    倉庫.溜める区間の並びを渡す(&[区間(0.0, 5.0)]);
    配信元.黙るまで待つ();
    let (送り口, 受け口) = mpsc::channel();
    std::thread::spawn(move || {
        drop(倉庫);
        let _ = 送り口.send(());
    });
    assert!(
        受け口.recv_timeout(Duration::from_secs(2)).is_ok(),
        "固まった ffmpeg を止めずに裏のスレッドの終わりを待っている"
    );
}
