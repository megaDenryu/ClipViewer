//! `verify` の最後の工程: FFmpeg を使う video_source と clip_viewer の結合試験を、FFmpeg が見つかったときだけ流す。
//! 結合試験は `#[ignore]` にしてあり、`cargo test --workspace` では流れない。見つからないときは、
//! 実行しなかったことと理由と手順を表示して、検証列そのものは失敗させない。
//! xtask は video_source に依存しない。video_source がビルドできなくても fmt と clippy を走らせるためである。
//! ここでは流すかどうかだけを決め、実際に使う ffmpeg と ffprobe の組は結合試験の側が video_source の口で探し直す。

use std::ffi::OsString;
use std::path::Path;

use crate::ffmpeg_location::{FFmpegの置き場所, 場所を渡す環境変数};
use crate::verify::工程を実行する;

/// 結合試験の結果とは、FFmpeg の結合試験を流して通ったか、FFmpeg が見つからず流さなかったかの区別のことである。
pub enum 結合試験の結果 {
    流して通った,
    流さなかった,
}

/// FFmpeg が見つかれば結合試験を流す。見つからなければ、実行しなかったことと手順を表示して、流さなかったと返す。
pub fn 結合試験を実行する(
    リポジトリルート: &Path,
) -> Result<結合試験の結果, String> {
    let Some(場所) = FFmpegの置き場所::探す() else {
        println!(
            "FFmpeg の結合試験を実行しなかった: ffmpeg と ffprobe が同じフォルダにそろう場所が、環境変数 {場所を渡す環境変数} にも PATH にも無い。"
        );
        println!(
            r"  流すには、環境変数 {場所を渡す環境変数} に FFmpeg を置いたフォルダ(例: C:\ffmpeg\bin)を入れるか、そのフォルダを PATH へ足してから cargo xtask verify を実行し直す。"
        );
        return Ok(結合試験の結果::流さなかった);
    };
    println!(
        "FFmpeg の結合試験を流す(FFmpeg の場所: {})",
        場所.フォルダ().display()
    );
    let 映像の境界の引数: &[&str] = &[
        "test",
        "--package",
        "video_source",
        "--test",
        "with_ffmpeg",
        "--",
        "--ignored",
    ];
    let アプリの引数: &[&str] = &["test", "--package", "clip_viewer", "--", "--ignored"];
    for 引数 in [映像の境界の引数, アプリの引数] {
        let 環境変数 = (場所を渡す環境変数, OsString::from(場所.フォルダ()));
        工程を実行する(引数, リポジトリルート, Some(環境変数))?;
    }
    Ok(結合試験の結果::流して通った)
}
