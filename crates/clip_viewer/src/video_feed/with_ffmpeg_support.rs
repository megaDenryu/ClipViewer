//! 映像の供給の結合試験の道具。FFmpeg を探すこと、試験動画を作って開くこと、求めを作ること、条件を待つこと。
#![allow(clippy::expect_used)]

use std::path::PathBuf;
use std::process::Command;
use std::time::{Duration, Instant};

use clip_domain::{
    クロップ範囲, 入力された動画パス, 動画上の区間, 時刻, 正規化した動画パス
};
use eframe::egui;
use video_source::{
    FFmpegの実行ファイル, FFmpegの置き場所の設定, FFmpegを置いたフォルダ, 動画の読み手,
    実行ファイルの検索パス,
};

use super::frame_request::表示するコマの求め;
use super::loaded_video::読み込んだ動画;

const 待つ期限: Duration = Duration::from_secs(30);

/// 環境変数 CLIPVIEWER_FFMPEG_DIR → PATH の順に探す(verify が渡す場所と同じ)。
pub(crate) fn 実行ファイルを探す() -> FFmpegの実行ファイル {
    let 設定 = std::env::var_os("CLIPVIEWER_FFMPEG_DIR").map_or(
        FFmpegの置き場所の設定::未設定,
        |フォルダ| {
            FFmpegの置き場所の設定::設定済み(FFmpegを置いたフォルダ::作成する(
                PathBuf::from(フォルダ),
            ))
        },
    );
    FFmpegの実行ファイル::探す(&設定, &実行ファイルの検索パス::環境変数から読む())
        .expect("FFmpeg が見つからない")
}

/// 4秒(30コマ/秒、320×180)の試験動画を一時フォルダへ作る。
pub(crate) fn 試験動画を作る(
    実行ファイル: &FFmpegの実行ファイル,
    名前: &str,
) -> 正規化した動画パス {
    let パス = std::env::temp_dir().join(format!(
        "clip_viewer_供給の試験_{名前}_{}.mp4",
        std::process::id()
    ));
    let 状態 = Command::new(実行ファイル.変換のパス())
        .args([
            "-y",
            "-v",
            "error",
            "-f",
            "lavfi",
            "-i",
            "testsrc2=size=320x180:rate=30:duration=4",
        ])
        .args(["-c:v", "libx264", "-pix_fmt", "yuv420p"])
        .arg(&パス)
        .status()
        .expect("ffmpeg を起動できない");
    assert!(状態.success(), "試験動画を作れない");
    入力された動画パス::作成する(パス.display().to_string()).正規化する()
}

/// 試験の動画とは、試験動画を開いた読み込んだ動画と、その動画ファイルのパスの組のことである。落とすとファイルを消す。
pub(super) struct 試験の動画 {
    pub(super) 動画: 読み込んだ動画,
    パス: 正規化した動画パス,
}

impl 試験の動画 {
    /// 名前ごとに別のファイルへ試験動画(4秒、30コマ/秒、320×180)を作って開く。試験を並べて走らせても重ならない。
    pub(super) fn 作って開く(名前: &str) -> Self {
        let 実行ファイル = 実行ファイルを探す();
        let パス = 試験動画を作る(&実行ファイル, 名前);
        let 読み手 = 動画の読み手::作成する(実行ファイル);
        let 動画 = 読み込んだ動画::開く(読み手, &パス, &egui::Context::default(), None)
            .expect("動画を開ける");
        Self { 動画, パス }
    }
}

impl Drop for 試験の動画 {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(self.パス.文字列());
    }
}

pub(crate) fn 秒(値: f64) -> clip_domain::動画上の秒 {
    時刻::作成する(値).expect("時刻")
}

pub(super) fn 求め(
    位置: f64, 区間: Option<動画上の区間>
) -> 表示するコマの求め {
    表示するコマの求め {
        動画上の位置: 秒(位置),
        区間,
        クロップ: クロップ範囲::全体,
    }
}

pub(crate) fn 成り立つまで待つ(説明: &str, mut 条件: impl FnMut() -> bool) {
    let 始め = Instant::now();
    while !条件() {
        assert!(始め.elapsed() < 待つ期限, "{説明}が期限内に成り立たない");
        std::thread::sleep(Duration::from_millis(10));
    }
}
