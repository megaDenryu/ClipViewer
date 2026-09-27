//! `verify` の最後の工程: FFmpeg を使う video_source と clip_viewer の結合試験を、FFmpeg が見つかったときだけ流す。
//! 結合試験は `#[ignore]` にしてあり、`cargo test --workspace` では流れない。見つからないときは、
//! 実行しなかったことと理由と手順を表示して、検証列そのものは失敗させない。
//! xtask は video_source に依存しない。video_source がビルドできなくても fmt と clippy を走らせるためである。
//! ここでは流すかどうかだけを決め、実際に使う ffmpeg と ffprobe の組は結合試験の側が video_source の口で探し直す。

use std::ffi::OsString;
use std::path::{Path, PathBuf};

use crate::verify::工程を実行する;

/// 結合試験へ FFmpeg を置いたフォルダを渡す環境変数。video_source の試験も同じ名前を読む。
const 場所を渡す環境変数: &str = "CLIPVIEWER_FFMPEG_DIR";

/// 結合試験の結果とは、FFmpeg の結合試験を流して通ったか、FFmpeg が見つからず流さなかったかの区別のことである。
pub enum 結合試験の結果 {
    流して通った,
    流さなかった,
}

/// 結合試験に渡すFFmpegの場所とは、ffmpeg と ffprobe の実行ファイルが同じフォルダにそろう場所のことである。
struct 結合試験に渡すFFmpegの場所(PathBuf);

impl 結合試験に渡すFFmpegの場所 {
    /// 環境変数 CLIPVIEWER_FFMPEG_DIR → PATH の順に、ffmpeg と ffprobe が同じフォルダにそろう最初の場所を探す。
    /// 参照: 探す順と同じフォルダの規則は crates/video_source/src/locate/executables.rs の `FFmpegの実行ファイル::探す` と同じである。
    fn 探す() -> Option<Self> {
        let 指定 = std::env::var_os(場所を渡す環境変数).map(PathBuf::from);
        let 検索パス = std::env::var_os("PATH").unwrap_or_default();
        指定
            .into_iter()
            .chain(std::env::split_paths(&検索パス))
            .find(|フォルダ| 両方そろうか(フォルダ))
            .map(Self)
    }
}

fn 両方そろうか(フォルダ: &Path) -> bool {
    ["ffmpeg", "ffprobe"].iter().all(|名前| {
        フォルダ
            .join(format!("{名前}{}", std::env::consts::EXE_SUFFIX))
            .is_file()
    })
}

/// FFmpeg が見つかれば結合試験を流す。見つからなければ、実行しなかったことと手順を表示して、流さなかったと返す。
pub fn 結合試験を実行する(
    リポジトリルート: &Path,
) -> Result<結合試験の結果, String> {
    let Some(場所) = 結合試験に渡すFFmpegの場所::探す() else {
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
        場所.0.display()
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
        let 環境変数 = (場所を渡す環境変数, OsString::from(&場所.0));
        工程を実行する(引数, リポジトリルート, Some(環境変数))?;
    }
    Ok(結合試験の結果::流して通った)
}
