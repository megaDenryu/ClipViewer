//! 音の供給と再生の指示の結合試験の道具。音付き・音無しの試験動画を作って開くことと、2つのクリップを並べた状態を作ること。
#![allow(clippy::expect_used)]

use std::path::{Path, PathBuf};
use std::process::Command;

use audio_pcm::サンプリング周波数;
use clip_domain::{
    クリップ, クリップスタック, クリップ名, クリップ識別子, リピート回数, リピート設定,
    入力された動画パス, 動画上の区間,
};
use eframe::egui;
use video_source::{FFmpegが見つからないエラー, 動画の読み手};

use super::{FFmpegの状況, アプリの状態, 入力中のフォルダ, 音の出力の状況};
use crate::thumbnail_feed::一覧のサムネイル;
use crate::video_feed::with_ffmpeg_support::{実行ファイルを探す, 秒};
use crate::video_feed::読み込んだ動画;

pub(super) fn 周波数() -> サンプリング周波数 {
    サンプリング周波数::作成する(48_000).expect("周波数")
}

/// 4秒の試験動画を一時フォルダへ作る。音を付けるときは 440Hz の正弦波にする。
pub(super) fn 試験動画を作る(名前: &str, 音を付けるか: bool) -> PathBuf {
    let パス = std::env::temp_dir().join(format!(
        "clip_viewer_音の試験_{名前}_{}.mp4",
        std::process::id()
    ));
    let mut 命令 = Command::new(実行ファイルを探す().変換のパス());
    命令.args([
        "-y",
        "-v",
        "error",
        "-f",
        "lavfi",
        "-i",
        "testsrc2=size=320x180:rate=30:duration=4",
    ]);
    if 音を付けるか {
        命令.args([
            "-f",
            "lavfi",
            "-i",
            "sine=frequency=440:duration=4",
            "-c:a",
            "aac",
        ]);
    }
    let 状態 = 命令
        .args(["-c:v", "libx264", "-pix_fmt", "yuv420p"])
        .arg(&パス)
        .status();
    assert!(
        状態.expect("ffmpeg を起動できない").success(),
        "試験動画を作れない"
    );
    パス
}

pub(super) fn 開く(
    パス: &Path, 周波数: Option<サンプリング周波数>
) -> 読み込んだ動画 {
    let 動画パス = 入力された動画パス::作成する(パス.display().to_string()).正規化する();
    let 読み手 = 動画の読み手::作成する(実行ファイルを探す());
    読み込んだ動画::開く(読み手, &動画パス, &egui::Context::default(), 周波数)
        .expect("動画を開ける")
}

pub(super) fn 区間(開始: f64, 終了: f64) -> 動画上の区間 {
    動画上の区間::作成する(秒(開始), 秒(終了)).expect("区間")
}

/// A[1,2) を2回、B[3,3.5) を1回、どちらも自動進行で並べた状態。止めたままシーケンス再生の先頭にいる。
pub(super) fn 二つのクリップの状態(動画: 読み込んだ動画) -> アプリの状態 {
    let クリップを作る = |名前: &str, 区間, 回数| {
        let 識別子 = クリップ識別子::文字列から作成する(名前.to_string()).expect("識別子");
        let mut クリップ =
            クリップ::既定値で作成する(識別子, クリップ名::作成する(名前.to_string()));
        クリップ.区間 = 区間;
        クリップ.リピート =
            リピート設定::回数指定(リピート回数::作成する(回数).expect("回数"));
        クリップ
    };
    let ffmpegの状況 = FFmpegの状況::見つからない {
        理由: FFmpegが見つからないエラー {
            見つからない道具: Vec::new(),
            探した場所: Vec::new(),
        },
        入力中のフォルダ: 入力中のフォルダ::default(),
    };
    let 音の出力 = 音の出力の状況::使える(周波数());
    let サムネイル =
        一覧のサムネイル::キャッシュを使わずに作る(egui::Context::default());
    let mut 状態 = アプリの状態::起動時(ffmpegの状況, 音の出力, サムネイル);
    状態.並び = クリップスタック::一覧から作成する(vec![
        クリップを作る("A", 区間(1.0, 2.0), 2),
        クリップを作る("B", 区間(3.0, 3.5), 1),
    ])
    .expect("スタック");
    状態.動画 = Some(動画);
    状態
}

pub(super) fn 溜めてあるか(状態: &アプリの状態, 区間: &動画上の区間) -> bool {
    状態
        .動画
        .as_ref()
        .and_then(|動画| 動画.音.供給())
        .and_then(|供給| 供給.溜めた音の出どころ(区間))
        .is_some()
}
