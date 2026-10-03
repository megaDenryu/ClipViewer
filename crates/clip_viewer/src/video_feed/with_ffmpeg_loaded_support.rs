//! コマの載せ先の結合試験の道具。試験動画を作り、その動画のコマの供給元と、載せるコマの範囲を決めたコマの載せ先を開く。
#![allow(clippy::expect_used)]

use std::time::Instant;

use clip_domain::{動画上の区間, 正規化した動画パス};
use eframe::egui;
use video_source::{メモリの上限, 動画の読み手};

use super::frame_loaded::コマを載せたか;
use super::frame_source::動画のコマの供給元;
use super::frame_target::コマの載せ先;
use super::load_range::載せるコマの範囲;
use super::video_texture::テクスチャの名前;
use super::with_ffmpeg_support::{実行ファイルを探す, 求め, 試験動画を作る};
use crate::stream_rules::流し読みの開き直し;

/// 試験動画の供給元と載せ先とは、試験動画(4秒、30コマ/秒)の動画のコマの供給元と、コマの載せ先と、動画ファイルのパスの組のことである。落とすとファイルを消す。
pub(super) struct 試験動画の供給元と載せ先 {
    供給元: 動画のコマの供給元,
    pub(super) 載せ先: コマの載せ先,
    パス: 正規化した動画パス,
}

impl 試験動画の供給元と載せ先 {
    /// 名前ごとに別のファイルへ試験動画を作り、載せるコマの範囲を決めた載せ先で開く。
    pub(super) fn 試験動画を作って開く(
        名前: &str, 範囲: 載せるコマの範囲
    ) -> Self {
        let 実行ファイル = 実行ファイルを探す();
        let パス = 試験動画を作る(&実行ファイル, 名前);
        let 供給元 = 動画のコマの供給元::動画の情報を取って作る(
            動画の読み手::作成する(実行ファイル),
            &パス,
            メモリの上限::既定,
        )
        .expect("動画を開ける");
        let 載せ先 = コマの載せ先::登録して作る(
            &egui::Context::default(),
            テクスチャの名前::作成する("試験の映像"),
            範囲,
        )
        .expect("テクスチャを作れる");
        Self {
            供給元,
            載せ先,
            パス,
        }
    }

    /// 区間を持たない求めで、位置のコマを流し読みから載せる。
    pub(super) fn 位置のコマを載せる(&mut self, 位置: f64) -> コマを載せたか {
        self.区間の位置のコマを載せる(位置, None)
    }

    /// 区間を添えた求めで、位置のコマを載せる。区間を溜めていないため流し読みから載せる。
    pub(super) fn 区間の位置のコマを載せる(
        &mut self,
        位置: f64,
        区間: Option<動画上の区間>,
    ) -> コマを載せたか {
        self.載せ先.コマを載せる(
            &mut self.供給元,
            &求め(位置, 区間),
            流し読みの開き直し::してよい,
            Instant::now(),
        )
    }
}

impl Drop for 試験動画の供給元と載せ先 {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(self.パス.文字列());
    }
}
