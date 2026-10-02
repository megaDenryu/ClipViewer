//! 起動の頼みの試験の道具。FFmpeg も音声出力装置も無いクリップビューアーを作ることと、
//! 通知に出ている最新の文面を読み出すことを受け持つ。
#![allow(clippy::expect_used)]

use eframe::egui;
use video_source::{FFmpegが見つからないエラー, 実行ファイルの検索パス};

use super::front_workspace::前に出ている作業場;
use super::sound_sender::音の送り手;
use super::viewer_settings_save::{見る側の設定の保存係, 設定の書き方};
use super::クリップビューアー;
use crate::command::操作の適用係;
use crate::persistence::アプリの設定の保管場所;
use crate::state::{
    FFmpegの状況, アプリの状態, 入力中のフォルダ, 音の出力の状況
};
use crate::thumbnail_feed::一覧のサムネイル;
use crate::viewer_settings::見る側の設定;

/// FFmpeg も音声出力装置も無く、動画を開いていないクリップビューアー。
pub(super) fn 試験のビューアー() -> クリップビューアー {
    let ffmpegの状況 = FFmpegの状況::見つからない {
        理由: FFmpegが見つからないエラー {
            見つからない道具: Vec::new(),
            探した場所: Vec::new(),
        },
        入力中のフォルダ: 入力中のフォルダ::default(),
    };
    let 音の出力 = 音の出力の状況::使えない(audio_output::音声出力のエラー::装置が無い);
    let 本体 = egui::Context::default();
    let サムネイル = 一覧のサムネイル::キャッシュを使わずに作る(本体.clone());
    クリップビューアー {
        状態: アプリの状態::起動時(ffmpegの状況, 音の出力, サムネイル),
        適用係: 操作の適用係::作成する(
            アプリの設定の保管場所::無い,
            実行ファイルの検索パス::作成する(Vec::new()),
            本体,
        ),
        音の送り手: 音の送り手::装置なし,
        起動の受け口: None,
        設定の保存係: 見る側の設定の保存係::作成する(
            アプリの設定の保管場所::無い,
            設定の書き方::書かない,
            見る側の設定::既定,
        ),
        前に出ている作業場: 前に出ている作業場::起動時(),
    }
}

/// 通知に出ている最新の文面。まだ出していなければ空の文字列。
pub(super) fn 通知の文(ビューアー: &クリップビューアー) -> String {
    ビューアー
        .状態
        .通知
        .最新()
        .map(|(_, 文面)| 文面.to_string())
        .unwrap_or_default()
}
