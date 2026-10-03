//! 開いている重ね合わせ。重ね合わせと、その再生の状況と、映像の供給を作れたかと、開いている重ね合わせの音(`sound/`)を持ち、
//! 毎フレームこのフレームで映すものを1回だけ求めて映像と音へ渡す。再生と停止・全体ループ・再生の位置を動かす操作は `playback_ops.rs`、画面が読む口は `read.rs` に置く。
//! 置き方の編集(`state/placement/`)と掴んでいるもの(`state/grab.rs`)も持ち、置き方の操作は `placement_ops.rs`・`placement_drag.rs`、読む口は `placement_read.rs` に置く。
//! タイムラインの操作(塊のドラッグ・行の選択と削除と追加)は `timeline_drag.rs`・`timeline_rows.rs`、読む口は `timeline_read.rs` に、
//! 位置の線のドラッグと掴んでいるものから決める流し読みの開き直し方・音の様子・安全網は `grab_ops.rs` に、編集の履歴へ積むことと取り消しとやり直しは `history_ops.rs` に置く。

mod grab_ops;
mod history_ops;
mod library_relation;
mod placement_drag;
mod placement_ops;
mod placement_read;
mod playback_ops;
mod read;
mod timeline_drag;
mod timeline_read;
mod timeline_rows;

pub(crate) use placement_read::重ねる枠;

use std::time::Instant;

use audio_output::行ごとの再生の指示;
use clip_domain::{重ね合わせ, 音量};

use crate::overlay::feed::{
    このフレームで映すもの, 重ね合わせの映像の供給を作れたか
};
use crate::overlay::state::grab::掴んでいるもの;
use crate::overlay::state::history::重ね合わせの編集の履歴;
use crate::overlay::state::library::重ね合わせとライブラリの関係;
use crate::overlay::state::placement::置き方の編集;
use crate::overlay::state::playback::重ね合わせの再生の状況;
use crate::overlay::state::sound::開いている重ね合わせの音;

/// 開いている重ね合わせとは、重ね合わせの作業場が開いている重ね合わせと、その再生の状況と、その映像の供給を作れたかと、
/// 開いている重ね合わせの音と、置き方の編集と、掴んでいるもの(ドラッグの間の仮の値)と、ライブラリとの関係(未登録か登録済みか)と、重ね合わせの編集の履歴の組のことである。
/// 供給は重ね合わせを開くときに作り、重ね合わせと一緒に捨てる。ライブラリとの関係と編集の履歴も重ね合わせと一緒に生まれて一緒に消える。
pub(crate) struct 開いている重ね合わせ {
    重ね合わせ: 重ね合わせ,
    再生: 重ね合わせの再生の状況,
    映像: 重ね合わせの映像の供給を作れたか,
    音: 開いている重ね合わせの音,
    編集: 置き方の編集,
    掴んでいるもの: 掴んでいるもの,
    ライブラリとの関係: 重ね合わせとライブラリの関係,
    履歴: 重ね合わせの編集の履歴,
}

impl 開いている重ね合わせ {
    pub(crate) fn 先頭で止まって開く(
        重ね合わせ: 重ね合わせ,
        映像: 重ね合わせの映像の供給を作れたか,
        音: 開いている重ね合わせの音,
        ライブラリとの関係: 重ね合わせとライブラリの関係,
    ) -> Self {
        let 再生 = 重ね合わせの再生の状況::起動時();
        let 履歴 = 重ね合わせの編集の履歴::今の値から始める(&重ね合わせ);
        Self {
            重ね合わせ,
            再生,
            映像,
            音,
            編集: 置き方の編集::開いた直後(),
            掴んでいるもの: 掴んでいるもの::無し,
            ライブラリとの関係,
            履歴,
        }
    }

    /// 毎フレーム呼ぶ。前のフレームで当てた操作による重ね合わせの変化を編集の履歴へ積み、時計を進め、進めた位置でこのフレームで映すものを1回だけ求め、行ごとに映すコマを載せ、行ごとの音を整え、行ごとの再生の指示を組み立てて返す。
    pub(crate) fn フレームを進める(
        &mut self,
        今: Instant,
        全体の鳴らす音量: 音量,
    ) -> 行ごとの再生の指示 {
        self.履歴.値を見る(&self.重ね合わせ);
        self.再生.時計を進める(今, &self.重ね合わせ);
        let 映すもの = このフレームで映すもの::求める(
            &self.重ね合わせ,
            self.再生.映すものを求める条件(),
        );
        let 開き直し = self.流し読みの開き直し();
        self.映像.行ごとのコマを載せる(&映すもの, 開き直し, 今);
        self.音.行ごとの溜める並びと流し読みを整える(
            &映すもの,
            &self.映像,
            開き直し,
            今,
        );
        let 様子 = self.音の再生の様子();
        let 指示 = self.音.行ごとの再生の指示を組み立てる(
            &映すもの,
            様子,
            今,
            全体の鳴らす音量,
        );
        self.コマが載った置いたばかりの置いたクリップを外す();
        指示
    }

    /// 2本目の流れが途中で使えなくなったときに、音の流し読みと音の倉庫の読み込みを止める。
    pub(crate) fn 出力が使えないため音を止める(&mut self) {
        self.音.出力が使えないため止める();
    }
}
