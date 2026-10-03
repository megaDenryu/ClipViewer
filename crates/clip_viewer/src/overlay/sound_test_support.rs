//! 重ね合わせの作業場の音の試験の道具。音声出力装置の無い音の出力の状況とその作業場を作ることと、置いたクリップの識別子を作ることと、行ごとの再生の指示から行の指示を取り出すことと、
//! 作業場の知らせの文を読むことと、音付きの試験動画を作ることを受け持つ。
#![allow(clippy::expect_used)]

use crate::overlay::state::library::重ね合わせのライブラリの状態;
use std::process::Command;
use std::time::Instant;

use audio_output::{再生の指示, 行ごとの再生の指示, 音声出力のエラー};
use audio_pcm::音の出どころ;
use clip_domain::{
    入力された動画パス, 正規化した動画パス, 置いたクリップの識別子, 行の番号
};

use super::state::重ね合わせの音の出力の状況;
use super::workspace::重ね合わせの作業場;
use crate::video_feed::with_ffmpeg_support::実行ファイルを探す;

/// 2本目の流れを開けなかった音の出力の状況。試験は音声出力装置を開かない。
pub(crate) fn 装置の無い音の出力を作る() -> 重ね合わせの音の出力の状況 {
    重ね合わせの音の出力の状況::使えない(音声出力のエラー::装置が無い)
}

/// 2本目の流れを開けなかった、重ね合わせを開いていない作業場。
pub(crate) fn 装置の無い作業場を作る() -> 重ね合わせの作業場 {
    音の出力で作業場を作る(eframe::egui::Context::default(), 装置の無い音の出力を作る())
}

/// 画面描画の共有状態と音の出力の状況を渡して、重ね合わせを開いていない作業場を作る。重ね合わせのライブラリは置き場所が無い(本物の置き場所を使わない)。
pub(crate) fn 音の出力で作業場を作る(
    画面描画の共有状態: eframe::egui::Context,
    音の出力: 重ね合わせの音の出力の状況,
) -> 重ね合わせの作業場 {
    重ね合わせの作業場::開いていない作業場(
        画面描画の共有状態,
        音の出力,
        重ね合わせのライブラリの状態::置き場所の無い試験の状態(),
    )
}

pub(crate) fn 置いたクリップの識別子を作る(
    文字列: &str,
) -> 置いたクリップの識別子 {
    置いたクリップの識別子::文字列から作成する(文字列.to_string()).expect("空でない")
}

/// 行の番号の行の指示を、黙る指示と入れ替えて取り出す(行ごとの再生の指示は行の指示を読む口を持たないため)。
pub(crate) fn 行の指示を取り出す(
    指示: &mut 行ごとの再生の指示,
    番号: usize,
) -> 再生の指示 {
    指示
        .行の指示を置き換える(
            行の番号::番号から作成する(番号),
            再生の指示::黙る(Instant::now()),
        )
        .expect("行の数の上限の内")
}

/// 作業場の知らせの文(古い順)。文は知らせの回と一緒に整形してあるため、試験は文を含むかで確かめる。
pub(crate) fn 知らせの文(作業場: &重ね合わせの作業場) -> Vec<String> {
    作業場
        .状態()
        .並べる知らせ()
        .iter()
        .map(|通知| format!("{通知:?}"))
        .collect()
}

/// 音付きの試験動画のファイル。落とすとファイルを消す。
pub(crate) struct 音付きの試験動画(pub(crate) 正規化した動画パス);

impl 音付きの試験動画 {
    /// 4秒(30コマ/秒、320×180)の映像に 440Hz の正弦波の音を付けた試験動画を一時フォルダへ作る。FFmpeg が要る。
    pub(crate) fn 作る(名前: &str) -> Self {
        let パス = std::env::temp_dir().join(format!(
            "clip_viewer_重ね合わせの音の試験_{名前}_{}.mp4",
            std::process::id()
        ));
        let 状態 = Command::new(実行ファイルを探す().変換のパス())
            .args(["-y", "-v", "error", "-f", "lavfi"])
            .args(["-i", "testsrc2=size=320x180:rate=30:duration=4"])
            .args(["-f", "lavfi", "-i", "sine=frequency=440:duration=4"])
            .args(["-c:a", "aac", "-c:v", "libx264", "-pix_fmt", "yuv420p"])
            .arg(&パス)
            .status()
            .expect("ffmpeg を起動できない");
        assert!(状態.success(), "試験動画を作れない");
        Self(入力された動画パス::作成する(パス.display().to_string()).正規化する())
    }
}

impl Drop for 音付きの試験動画 {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(self.0.文字列());
    }
}

/// 行の指示の区間の出どころが溜めた音か。
pub(crate) fn 溜めた音か(指示: &再生の指示) -> bool {
    指示
        .区間
        .as_ref()
        .is_some_and(|区間| matches!(区間.出どころ, Some(音の出どころ::溜めた音(_))))
}
