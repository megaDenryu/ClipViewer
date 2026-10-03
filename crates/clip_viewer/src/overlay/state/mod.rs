//! 重ね合わせの作業場の状態。重ね合わせを開いているか(開いていればその重ね合わせと再生の状況と映像と音の供給とライブラリとの関係)と、重ね合わせの作業場の知らせと、
//! 置かなかったクリップの報告と、知らせ済みのスタックの保存の失敗と、重ね合わせの音の出力の状況と、重ね合わせのライブラリの状態(`library/`)を持つ。
//! 開いている重ね合わせの置き方の編集は `placement/` と `opened/placement_*.rs` に置き、画面と操作は `state::placement::…` の道筋で読む。音の側(音の出力の状況・開いている重ね合わせの音・行ごとの再生の指示の組み立て・音の口)は `sound/` に置く。
//! 画面は状態を読む口(`read.rs` と、開いている重ね合わせの重ねる画面に描く行の並び)だけを読み、
//! 書き換えは応答(`overlay/command`)と、作業場(`overlay/workspace/`)の口を通す。参照: _doc/設計/同時再生.md 3-3

mod arrange_report;
mod drawn_rows;
pub(crate) mod grab;
pub(crate) mod library;
mod notice;
mod opened;
mod opening;
pub(crate) mod placement;
mod playback;
mod read;
mod sound;
mod stack_save_failure;
mod stack_source;
pub(crate) mod timeline;
mod whole_volume;

#[cfg(test)]
mod stack_save_failure_tests;
#[cfg(test)]
mod stack_source_tests;

pub(crate) use arrange_report::置かなかったクリップの報告;
pub(crate) use drawn_rows::重ねる画面に描く行;
pub(crate) use notice::重ね合わせの作業場の知らせ;
pub(crate) use opened::開いている重ね合わせ;
pub(crate) use opening::重ね合わせの開き方;
pub(crate) use sound::重ね合わせの音の出力の状況;
pub(crate) use stack_save_failure::{
    スタックの保存の観測結果, 知らせ済みのスタックの保存の失敗
};
pub(crate) use stack_source::{
    今のスタックから並べる状況, 今のスタックの値, 開いている動画の値
};
pub(crate) use whole_volume::全体の音量;

use std::time::Instant;

use audio_output::行ごとの再生の指示;
use clip_domain::音量;

/// 重ね合わせの作業場の状態とは、重ね合わせの開き方と、重ね合わせの作業場の知らせと、置かなかったクリップの報告と、知らせ済みのスタックの保存の失敗と、
/// 重ね合わせの音の出力の状況と、重ね合わせのライブラリの状態の組のことである。
/// 知らせと報告と音の出力の状況は、重ね合わせを開いているかにかかわらず持つ(置けるクリップが1つも無いときは開かずに報告する)ため、開き方の外に持つ。
pub(crate) struct 重ね合わせの作業場の状態 {
    pub(in crate::overlay) 開き方: 重ね合わせの開き方,
    pub(in crate::overlay) 知らせ: 重ね合わせの作業場の知らせ,
    置かなかったクリップの報告: 置かなかったクリップの報告,
    知らせ済みのスタックの保存の失敗: 知らせ済みのスタックの保存の失敗,
    pub(super) 音の出力: 重ね合わせの音の出力の状況,
    pub(super) ライブラリ: library::重ね合わせのライブラリの状態,
}

impl 重ね合わせの作業場の状態 {
    /// 初めて重ね合わせの作業場へ移ったときの状態。重ね合わせを開いておらず、知らせも報告も無い。音の出力の状況は、配線が2本目の流れを開いた結果である。
    /// 重ね合わせのライブラリの状態は、配線が起動のときに錠を試した重ね合わせのライブラリから作る。
    pub(crate) fn 開いていない(
        音の出力: 重ね合わせの音の出力の状況,
        ライブラリ: library::重ね合わせのライブラリの状態,
    ) -> Self {
        Self {
            開き方: 重ね合わせの開き方::開いていない,
            知らせ: 重ね合わせの作業場の知らせ::default(),
            置かなかったクリップの報告:
                置かなかったクリップの報告::default(),
            知らせ済みのスタックの保存の失敗:
                知らせ済みのスタックの保存の失敗::default(),
            音の出力,
            ライブラリ,
        }
    }

    /// 開いている重ね合わせ。開いていなければ無い。
    pub(crate) fn 開いている重ね合わせ(&self) -> Option<&開いている重ね合わせ> {
        self.開き方.開いている重ね合わせ()
    }

    /// 毎フレーム呼ぶ。重ね合わせを開いていれば、時計を進め(再生している間だけ進む)、行ごとに映すコマを載せ、行ごとの音を整え、
    /// 行ごとの再生の指示を返す。開いていなければ、どの行も黙る指示を返す。
    pub(crate) fn フレームを進める(
        &mut self,
        今: Instant,
        全体の鳴らす音量: 音量,
    ) -> 行ごとの再生の指示 {
        match self.開き方.開いている重ね合わせを書き換える() {
            Some(開いている) => 開いている.フレームを進める(今, 全体の鳴らす音量),
            None => 行ごとの再生の指示::黙る(今),
        }
    }

    /// 作業場を控える前に呼ぶ。開いている重ね合わせの再生を止め、行ごとの映像と音の流し読みを閉じて ffmpeg を止める。重ね合わせと再生の位置と載せたコマは残す。
    /// 登録済みの重ね合わせに書けていない変更があれば、落ち着くのを待たずに保存を頼む(スタックの作業場を控えるときと同じ。同時再生.md 2-1)。
    pub(crate) fn 控える前に再生を止めて保存を頼む(&mut self) {
        if let Some(開いている) = self.開き方.開いている重ね合わせを書き換える()
        {
            開いている.控える前に再生を止めて流し読みを閉じる();
        }
        self.書けていない重ね合わせの保存を頼む();
    }
}
