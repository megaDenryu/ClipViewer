//! 動画の読み手を貸す口の試験。FFmpeg が見つかる前は貸さず、FFmpeg の置き場所を設定し直した後は新しい置き場所の読み手を貸すことを確かめる。
//! FFmpeg を起動しないため、ffmpeg と ffprobe の名前の空のファイルを置いたフォルダで探す。

use std::path::PathBuf;

use eframe::egui;
use video_source::{
    FFmpegの実行ファイル, FFmpegの置き場所の設定, FFmpegの道具, FFmpegを置いたフォルダ,
    動画の読み手, 実行ファイルの検索パス,
};

use crate::command::操作の適用係;
use crate::persistence::アプリの設定の保管場所;

/// ffmpeg と ffprobe の名前の空のファイルを置いた一時フォルダ。落とすとフォルダを消す。
struct 空の道具を置いたフォルダ(PathBuf);

impl 空の道具を置いたフォルダ {
    fn 作る(名前: &str) -> Self {
        let フォルダ = std::env::temp_dir().join(format!(
            "clip_viewer_読み手を貸す試験_{名前}_{}",
            std::process::id()
        ));
        std::fs::create_dir_all(&フォルダ).expect("フォルダを作れる");
        for 道具 in [FFmpegの道具::変換, FFmpegの道具::調査] {
            std::fs::write(フォルダ.join(道具.ファイル名()), b"").expect("ファイルを置ける");
        }
        Self(フォルダ)
    }

    /// このフォルダの道具から作った読み手。
    fn 読み手(&self) -> 動画の読み手 {
        let 実行ファイル = FFmpegの実行ファイル::探す(
            &self.置き場所の設定(),
            &実行ファイルの検索パス::作成する(Vec::new()),
        )
        .expect("道具を置いた");
        動画の読み手::作成する(実行ファイル)
    }

    fn 置き場所の設定(&self) -> FFmpegの置き場所の設定 {
        FFmpegの置き場所の設定::設定済み(FFmpegを置いたフォルダ::作成する(
            self.0.clone(),
        ))
    }
}

impl Drop for 空の道具を置いたフォルダ {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

#[test]
fn ffmpegが見つかる前は読み手を貸さず_置き場所を設定し直した後は新しい置き場所の読み手を貸す() {
    let mut 適用係 = 操作の適用係::作成する(
        アプリの設定の保管場所::無い,
        実行ファイルの検索パス::作成する(Vec::new()),
        egui::Context::default(),
    );
    assert!(適用係.動画の読み手().is_none());
    let 前の場所 = 空の道具を置いたフォルダ::作る("前");
    let 後の場所 = 空の道具を置いたフォルダ::作る("後");
    適用係.ffmpegを探す(&前の場所.置き場所の設定());
    assert_eq!(適用係.動画の読み手(), Some(&前の場所.読み手()));
    適用係.ffmpegを探す(&後の場所.置き場所の設定());
    assert_eq!(適用係.動画の読み手(), Some(&後の場所.読み手()));
}
