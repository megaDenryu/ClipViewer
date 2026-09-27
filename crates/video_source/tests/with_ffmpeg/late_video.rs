//! 映像の流れが入れ物の始まりより遅れて始まる試験動画(音が先に始まるもの)と、その枠に描かれているべきコマの規則。

use std::process::Command;

use video_source::動画の情報;

use crate::common::{実行ファイルを探す, 読み手};
use crate::numbered_videos::番号を描くフィルタ;
use crate::test_videos::動画を作る;

/// 30コマ/秒の映像を0.5秒遅らせ、音を0秒から始めた mkv を作って調べ、映像の流れの始まりの時刻(秒)と組にして返す。
pub fn 映像の遅れた動画を開く(名前: &str) -> (動画の情報, f64) {
    let 映像 = format!("color=c=black:s=320x180:r=30:d=3,format=yuv420p,{番号を描くフィルタ}");
    let 引数 = [
        "-itsoffset",
        "0.5",
        "-f",
        "lavfi",
        "-i",
        &映像,
        "-f",
        "lavfi",
        "-i",
        "sine=d=3.5",
        "-map",
        "0",
        "-map",
        "1",
        "-c:v",
        "libx264",
        "-crf",
        "5",
        "-g",
        "300",
        "-c:a",
        "aac",
    ];
    let パス = 動画を作る(名前, &引数);
    let 出力 = Command::new(実行ファイルを探す().調査のパス())
        .args([
            "-v",
            "error",
            "-select_streams",
            "v:0",
            "-show_entries",
            "stream=start_time",
        ])
        .args(["-of", "csv=p=0"])
        .arg(パス.文字列())
        .output()
        .expect("ffprobe を起動できない");
    let 始まり: f64 = String::from_utf8_lossy(&出力.stdout)
        .trim()
        .parse()
        .expect("映像の始まり");
    assert!(
        始まり > 0.4,
        "試験の前提: 映像が入れ物の始まりより遅れて始まる({始まり}秒)"
    );
    (
        読み手()
            .動画の情報を取る(&パス)
            .expect("動画の情報を取れない"),
        始まり,
    )
}

/// 番号の枠の真ん中の時刻に映っているべきコマの番号。映像が始まる前の枠は、最初のコマで埋まる。
/// 映像のコマの時刻は枠の刻みとずれているため、丸めの向きの違いで1つずれることを許す。
pub fn 合っているか(番号: u32, 描かれた: u32, 始まり: f64) -> bool {
    let 真ん中 = (f64::from(番号) + 0.5) / 30.0;
    if 真ん中 < 始まり {
        return 描かれた == 0;
    }
    ((真ん中 - 始まり) * 30.0 - f64::from(描かれた)).abs() <= 1.0
}
