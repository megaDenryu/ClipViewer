//! FFmpeg の結合試験が開く試験動画を作る。ffmpeg の試験用の映像源から、既定で2秒の動画を一時フォルダへ書き出す。
#![allow(clippy::expect_used)]

use std::process::Command;

use clip_domain::入力された動画パス;

use video_source::動画の読み手;

use super::library_support::試験のライブラリ;
use crate::state::アプリの状態;
use crate::video_feed::with_ffmpeg_support::実行ファイルを探す;

/// 2秒の試験動画「練習動画.mp4」を、試験の名前ごとに重ならない一時フォルダへ作り、そのパスの入力を返す。
/// 試験は並んで走るため、フォルダを共有すると、ある試験が読んでいる動画を別の試験が書き直してしまう。
pub(super) fn 試験動画を作る(試験の名前: &str) -> 入力された動画パス {
    名前を付けて試験動画を作る(試験の名前, "練習動画.mp4")
}

/// 2秒の試験動画を、試験の名前ごとに重ならない一時フォルダへファイル名を付けて作り、そのパスの入力を返す。
pub(super) fn 名前を付けて試験動画を作る(
    試験の名前: &str,
    ファイル名: &str,
) -> 入力された動画パス {
    長さを決めて試験動画を作る(試験の名前, ファイル名, 2.0)
}

/// 秒数の長さの試験動画を、試験の名前ごとに重ならない一時フォルダへファイル名を付けて作り、そのパスの入力を返す。
pub(super) fn 長さを決めて試験動画を作る(
    試験の名前: &str,
    ファイル名: &str,
    秒数: f64,
) -> 入力された動画パス {
    let フォルダ = std::env::temp_dir().join(format!(
        "clip_viewer_ライブラリ動画_{試験の名前}_{}",
        std::process::id()
    ));
    std::fs::create_dir_all(&フォルダ).expect("フォルダを作れる");
    let パス = フォルダ.join(ファイル名);
    let 状態 = Command::new(実行ファイルを探す().変換のパス())
        .args([
            "-y",
            "-v",
            "error",
            "-f",
            "lavfi",
            "-i",
            &format!("testsrc2=size=320x180:rate=30:duration={秒数}"),
        ])
        .args(["-c:v", "libx264", "-pix_fmt", "yuv420p"])
        .arg(&パス)
        .status()
        .expect("ffmpeg を起動できない");
    assert!(状態.success(), "試験動画を作れない");
    入力された動画パス::作成する(パス.display().to_string())
}

/// FFmpeg で動画を開ける適用係を持つ試験のライブラリと、そのライブラリへ接続した空の状態。
pub(super) fn 読み手のある試験(
    名前: &str,
) -> (試験のライブラリ, アプリの状態) {
    let mut 試験 = 試験のライブラリ::作る(名前);
    試験
        .適用係
        .開き手
        .読み手を持たせる(動画の読み手::作成する(実行ファイルを探す()));
    let 状態 = 試験.接続した状態(Vec::new());
    (試験, 状態)
}
