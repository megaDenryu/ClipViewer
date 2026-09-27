//! 読み込んだ動画。動画を開いたときに一緒に作り、解除したときに一緒に捨てるもの(倉庫・流し読み・テクスチャ・依頼の台帳・動画の音)の組。
//! 参照: _doc/設計/画面.md 判断2

use audio_pcm::サンプリング周波数;
use eframe::egui;
use video_source::{
    コマの倉庫, メモリの上限, 動画の情報, 動画の読み手, 受付の札, 長辺の上限
};

use clip_domain::正規化した動画パス;

use super::frame_request::コマの出どころ;
use super::ledger::依頼の台帳;
use super::open_failure::動画を開けない理由;
use super::stream_slot::開いた流し読み;
use super::video_texture::映像のテクスチャ;
use crate::audio_feed::動画の音;

/// 読み込んだ動画とは、1つの動画に結び付いたコマの倉庫と、流し読みと、映像のテクスチャと、先読みの依頼の台帳と、動画の音の組のことである。
/// 動画の情報は倉庫が保持する。流し読みは溜めたコマで映せない間だけ開く。動画の音は音の供給(`audio_feed`)が扱う。
pub(crate) struct 読み込んだ動画 {
    pub(super) 読み手: 動画の読み手,
    pub(super) 倉庫: コマの倉庫,
    pub(super) 流し読み: Option<開いた流し読み>,
    pub(super) 映像: 映像のテクスチャ,
    pub(super) 台帳: 依頼の台帳<受付の札>,
    pub(super) 出どころ: コマの出どころ,
    pub(super) 載せられない理由: Option<String>,
    pub(super) 流し読みのコマを待っている: bool,
    pub(crate) 音: 動画の音,
}

impl 読み込んだ動画 {
    /// 動画の情報を ffprobe で調べ、既定の長辺の上限とメモリの上限で倉庫を開き、映像のテクスチャを登録する。
    /// 音は出力装置の周波数があれば開く。音を鳴らせなくても動画は開く。
    pub(crate) fn 開く(
        読み手: 動画の読み手,
        パス: &正規化した動画パス,
        eguiの本体: &egui::Context,
        周波数: Option<サンプリング周波数>,
    ) -> Result<Self, 動画を開けない理由> {
        let 情報 = 読み手.動画の情報を取る(パス)?;
        let 音 = 動画の音::開く(&読み手, &情報, 周波数);
        let 倉庫 = 読み手.コマの倉庫を開く(情報, 長辺の上限::既定, メモリの上限::既定)?;
        let 映像 = 映像のテクスチャ::登録して作る(eguiの本体)?;
        Ok(Self {
            読み手,
            倉庫,
            流し読み: None,
            映像,
            台帳: 依頼の台帳::空(),
            出どころ: コマの出どころ::流し読み,
            載せられない理由: None,
            流し読みのコマを待っている: false,
            音,
        })
    }

    pub(crate) fn 情報(&self) -> &動画の情報 {
        self.倉庫.動画()
    }

    pub(crate) fn 映像(&self) -> &映像のテクスチャ {
        &self.映像
    }

    pub(crate) fn 台帳(&self) -> &依頼の台帳<受付の札> {
        &self.台帳
    }

    /// 流し読みで、求めた位置のコマがまだ届いていないか。画面は、この間は入力が無くても描き直す。
    pub(crate) fn 流し読みのコマを待っているか(&self) -> bool {
        self.出どころ == コマの出どころ::流し読み && self.流し読みのコマを待っている
    }

    /// 最後に表示したコマの出どころ。
    pub(crate) fn 出どころ(&self) -> コマの出どころ {
        self.出どころ
    }

    /// 流し読みが読めない理由、またはテクスチャへ載せられなかった理由。
    pub(crate) fn 映せない理由(&self) -> Option<&str> {
        let 流し読みの理由 = match self.出どころ {
            コマの出どころ::流し読み => self
                .流し読み
                .as_ref()
                .and_then(開いた流し読み::読めない理由),
            コマの出どころ::溜めたコマ(_) => None,
        };
        流し読みの理由.or(self.載せられない理由.as_deref())
    }
}
