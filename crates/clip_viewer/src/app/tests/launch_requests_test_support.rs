//! 起動の頼みの試験の道具。FFmpeg も音声出力装置も無いクリップビューアー(2本目の流れも開けなかったものとして持ち、装置を開かない)を作ることと、
//! 通知に出ている最新の文面を読み出すことを受け持つ。
#![allow(clippy::expect_used)]

use eframe::egui;
use video_source::{FFmpegが見つからないエラー, 実行ファイルの検索パス};

use super::super::front_workspace::前に出ている作業場;
use super::super::instruction_receiver::二本目の流れを開く手立て;
use super::super::overlay_side::重ね合わせの側の作り方;
use super::super::sound_sender::音の送り手;
use super::super::viewer_settings_save::{見る側の設定の保存係, 設定の書き方};
use super::super::クリップビューアー;
use super::instruction_receiver_test_support::開けなかったものとして開く;
use crate::command::操作の適用係;
use crate::persistence::アプリの設定の保管場所;
use crate::state::{
    FFmpegの状況, アプリの状態, 入力中のフォルダ, 音の出力の状況
};
use crate::thumbnail_feed::一覧のサムネイル;
use crate::viewer_settings::見る側の設定;

/// FFmpeg も音声出力装置も無く、動画を開いていないクリップビューアー。2本目の流れは、本番と同じ経路で開けなかったものとして作る。
pub(in crate::app) fn 試験のビューアー() -> クリップビューアー {
    試験のビューアーを二本目の流れの手立てで作る(
        Box::new(開けなかったものとして開く(
            audio_output::音声出力のエラー::装置が無い,
        )),
    )
}

/// FFmpeg も音声出力装置も無く、動画を開いていないクリップビューアー。2本目の流れは渡した手立てで開く。
pub(in crate::app) fn 試験のビューアーを二本目の流れの手立てで作る(
    流れを開く手立て: Box<dyn 二本目の流れを開く手立て>,
) -> クリップビューアー {
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
            本体.clone(),
        ),
        音の送り手: 音の送り手::装置なし,
        起動の受け口: None,
        設定の保存係: 見る側の設定の保存係::作成する(
            アプリの設定の保管場所::無い,
            設定の書き方::書かない,
            見る側の設定::既定,
        ),
        前に出ている作業場: 前に出ている作業場::起動時(
            重ね合わせの側の作り方 {
                画面描画の共有状態: 本体,
                流れを開く手立て,
            },
        ),
    }
}

/// 通知に出ている最新の文面。まだ出していなければ空の文字列。
pub(in crate::app) fn 通知の文(ビューアー: &クリップビューアー) -> String {
    ビューアー
        .状態
        .通知
        .最新()
        .map(|(_, 文面)| 文面.to_string())
        .unwrap_or_default()
}
