//! 結合試験の試験動画を ffmpeg の lavfi で target 配下へ作る。

use std::path::PathBuf;
use std::process::Command;

use clip_domain::{入力された動画パス, 正規化した動画パス};

use crate::common::実行ファイルを探す;

/// 赤・緑・青を1秒ずつ(30コマ/秒、320×180)つないだ試験動画を作る。キーフレームは先頭だけにし、
/// 区間の途中へのシークがキーフレームからのデコードを伴うようにする。x264 は場面の切り替わりにキーフレームを
/// 足すため、`-sc_threshold 0` で止め、作った後に ffprobe で確かめる。
pub fn 色を並べた動画を作る(名前: &str) -> 正規化した動画パス {
    let 色 = |名前: &str| format!("color=c={名前}:s=320x180:r=30:d=1");
    let フィルタ = format!(
        "{}[赤];{}[緑];{}[青];[赤][緑][青]concat=n=3:v=1:a=0,format=yuv420p",
        色("red"),
        色("green"),
        色("blue")
    );
    let 引数 = [
        "-filter_complex",
        &フィルタ,
        "-c:v",
        "libx264",
        "-g",
        "300",
        "-sc_threshold",
        "0",
    ];
    let パス = 動画を作る(名前, &引数);
    assert_eq!(
        キーフレームの時刻(&パス),
        ["0.000000"],
        "試験の前提: キーフレームは先頭だけである"
    );
    パス
}

/// ffprobe でキーフレームの表示時刻を並べる。ffprobe が失敗したら、試験の前提ではなく ffprobe の失敗として落とす。
/// `pkt_pts_time` は FFmpeg 5 以降の ffprobe で出ないため、`best_effort_timestamp_time` を使う。
/// 出力は1行に値だけを書く形(`default=nw=1:nk=1`)にする。csv の形は版で行の形が変わる(9.0 は行末にカンマを足し、
/// 4.4 は空の行を挟む)ためである。
fn キーフレームの時刻(パス: &正規化した動画パス) -> Vec<String> {
    let 引数 = [
        "-v",
        "error",
        "-select_streams",
        "v:0",
        "-skip_frame",
        "nokey",
    ];
    let 出力 = Command::new(実行ファイルを探す().調査のパス())
        .args(引数)
        .args([
            "-show_entries",
            "frame=best_effort_timestamp_time",
            "-of",
            "default=nw=1:nk=1",
        ])
        .arg(パス.文字列())
        .output()
        .expect("ffprobe を起動できない");
    assert!(
        出力.status.success(),
        "キーフレームを調べる ffprobe が失敗した({}): {}",
        出力.status,
        String::from_utf8_lossy(&出力.stderr)
    );
    let 本文 = String::from_utf8_lossy(&出力.stdout);
    本文
        .lines()
        .map(str::trim)
        .filter(|行| !行.is_empty())
        .map(str::to_string)
        .collect()
}

/// デコードに時間のかかる長い試験動画(640×360、30コマ/秒、20秒)を作る。取り消しと後始末の試験に使う。
pub fn 長い動画を作る(名前: &str) -> 正規化した動画パス {
    let 入力 = "testsrc2=size=640x360:rate=30:duration=20";
    動画を作る(
        名前,
        &[
            "-f",
            "lavfi",
            "-i",
            入力,
            "-c:v",
            "libx264",
            "-preset",
            "ultrafast",
            "-pix_fmt",
            "yuv420p",
        ],
    )
}

pub fn 動画を作る(名前: &str, 引数: &[&str]) -> 正規化した動画パス {
    let 出力 = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(名前);
    let 状態 = Command::new(実行ファイルを探す().変換のパス())
        .args(["-hide_banner", "-loglevel", "error", "-y"])
        .args(引数)
        .arg(&出力)
        .status()
        .expect("ffmpeg を起動できない");
    assert!(状態.success(), "試験動画を作れない");
    入力された動画パス::作成する(出力.to_string_lossy().into_owned()).正規化する()
}
