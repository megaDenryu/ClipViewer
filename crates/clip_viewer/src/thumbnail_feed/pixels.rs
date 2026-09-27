//! 戻した画素。サムネイルの JPEG を、テクスチャへ載せる赤・緑・青・不透明度の画素の並びへ戻す。
//! 戻すのは裏のスレッドで行い、テクスチャへの登録だけを画面のスレッドで行う。JPEG は一覧の枠の2倍の大きさで撮るため、戻した画素をそのまま載せる。
//! 参照: _doc/設計/ライブラリ.md 判断10(JPEG を image クレートで戻す理由)

use clip_domain::サムネイルの画像;
use eframe::egui;
use sengen_egui::{
    差し替えられるテクスチャ, 拡大縮小の仕方, 画素の並び, 画素の並びの不正, 画素数の寸法,
};

/// 画素へ戻せない理由とは、サムネイルの JPEG を画素へ戻せなかった理由(image クレートの復号の失敗)のことである。
#[derive(Debug, thiserror::Error)]
#[error("サムネイルの画像を読めない: {0}")]
pub(crate) struct 画素へ戻せない理由(#[from] image::ImageError);

/// 戻した画素とは、サムネイルの JPEG を戻した、1画素につき赤・緑・青・不透明度の4バイトの並びと、その寸法の組のことである。
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct 戻した画素 {
    寸法: 画素数の寸法,
    並び: Vec<u8>,
}

impl 戻した画素 {
    /// JPEG のバイト列を画素へ戻す。JPEG として読めなければ理由を返す。
    pub(crate) fn サムネイルの画像から戻す(
        画像: &サムネイルの画像,
    ) -> Result<Self, 画素へ戻せない理由> {
        let 読んだ =
            image::load_from_memory_with_format(画像.バイト列(), image::ImageFormat::Jpeg)?
                .to_rgba8();
        Ok(Self {
            寸法: 画素数の寸法::生成する(読んだ.width(), 読んだ.height()),
            並び: 読んだ.into_raw(),
        })
    }

    /// 画素の寸法。試験で中身を確かめるために使う。
    #[cfg(test)]
    pub(crate) fn 寸法(&self) -> 画素数の寸法 {
        self.寸法
    }

    /// 赤・緑・青・不透明度の並び。試験で中身を確かめるために使う。
    #[cfg(test)]
    pub(crate) fn 並び(&self) -> &[u8] {
        &self.並び
    }

    /// egui へ登録してテクスチャにする。サムネイルは写真なので、隣の画素と混ぜて拡大縮小する。
    pub(crate) fn テクスチャにする(
        &self,
        eguiの本体: &egui::Context,
        名前: String,
    ) -> Result<差し替えられるテクスチャ, 画素の並びの不正> {
        let 画素 = 画素の並び::赤緑青と不透明度のバイト列から作る(
            self.寸法,
            &self.並び,
        )?;
        Ok(差し替えられるテクスチャ::登録して作る(
            eguiの本体,
            名前,
            画素,
            拡大縮小の仕方::隣の画素と混ぜる,
        ))
    }
}
