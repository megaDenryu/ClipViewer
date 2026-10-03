//! 重ね合わせの作業場の状態。重ね合わせを開いているか(開いていればその重ね合わせと再生の状況と映像の供給を作れたか)と、重ね合わせの作業場の知らせと、
//! 置かなかったクリップの報告と、知らせ済みのスタックの保存の失敗と、並べ直す前の確かめを持つ。開いている重ね合わせの置き方の編集は `placement*.rs` に置く。
//! 画面は状態を読む口(`read.rs` と、開いている重ね合わせの重ねる画面に描く行の並び)だけを読み、
//! 書き換えは応答(`overlay/command`)と、作業場(`overlay/workspace/`)の口を通す。参照: _doc/設計/同時再生.md 3-3

mod arrange_report;
mod drawn_rows;
mod notice;
mod opened;
mod opening;
mod place_from_stack;
mod place_source;
mod placement;
mod placement_drag;
mod placement_ops;
mod placement_read;
mod playback;
mod read;
mod rearrange_confirm;
mod stack_save_failure;
mod stack_source;

#[cfg(test)]
mod stack_save_failure_tests;
#[cfg(test)]
mod stack_source_tests;

pub(crate) use arrange_report::置かなかったクリップの報告;
pub(crate) use drawn_rows::重ねる画面に描く行;
pub(crate) use notice::重ね合わせの作業場の知らせ;
pub(crate) use opened::開いている重ね合わせ;
pub(crate) use opening::重ね合わせの開き方;
pub(crate) use place_from_stack::{
    動画が違うため置けない文, 動画を開いていないため置けない文
};
pub(crate) use place_source::{
    置くクリップ, 置くクリップの一覧, 置くクリップの動画
};
pub(crate) use placement::{
    映す矩形のドラッグの動き, 映す矩形の掴む所, 隅の縦横比の扱い
};
pub(crate) use placement_read::重ねる枠;
pub(crate) use stack_save_failure::{
    スタックの保存の観測結果, 知らせ済みのスタックの保存の失敗
};
pub(crate) use stack_source::{
    今のスタックから並べる状況, 今のスタックの値, 開いている動画の値
};

use std::time::Instant;

use clip_domain::重ね合わせ;

use crate::overlay::feed::重ね合わせの映像の供給を作れたか;
use rearrange_confirm::並べ直す前の確かめ;

/// 重ね合わせの作業場の状態とは、重ね合わせの開き方と、重ね合わせの作業場の知らせと、置かなかったクリップの報告と、知らせ済みのスタックの保存の失敗と、並べ直す前の確かめの組のことである。
/// 知らせと報告は、重ね合わせを開いているかにかかわらず出す(置けるクリップが1つも無いときは開かずに報告する)ため、開き方の外に持つ。
pub(crate) struct 重ね合わせの作業場の状態 {
    pub(in crate::overlay) 開き方: 重ね合わせの開き方,
    pub(in crate::overlay) 知らせ: 重ね合わせの作業場の知らせ,
    置かなかったクリップの報告: 置かなかったクリップの報告,
    知らせ済みのスタックの保存の失敗: 知らせ済みのスタックの保存の失敗,
    並べ直す前の確かめ: 並べ直す前の確かめ,
}

impl 重ね合わせの作業場の状態 {
    /// 初めて重ね合わせの作業場へ移ったときの状態。重ね合わせを開いておらず、知らせも報告も無い。
    pub(crate) fn 開いていない() -> Self {
        Self {
            開き方: 重ね合わせの開き方::開いていない,
            知らせ: 重ね合わせの作業場の知らせ::default(),
            置かなかったクリップの報告:
                置かなかったクリップの報告::default(),
            知らせ済みのスタックの保存の失敗:
                知らせ済みのスタックの保存の失敗::default(),
            並べ直す前の確かめ: 並べ直す前の確かめ::出していない,
        }
    }

    /// 重ね合わせを、その映像の供給を作れたかと一緒に開く。先頭で止まった再生の状況で始める。映像の供給を作れなかったときと開けなかった動画があるときは、その理由を知らせる。
    pub(crate) fn 重ね合わせを開く(
        &mut self,
        重ね合わせ: 重ね合わせ,
        映像: 重ね合わせの映像の供給を作れたか,
    ) {
        for 文 in 映像.開けなかった知らせの文() {
            self.知らせ.出す(文);
        }
        self.開き方 = 重ね合わせの開き方::開いている(
            開いている重ね合わせ::先頭で止まって開く(重ね合わせ, 映像),
        );
    }

    /// 開いている重ね合わせ。開いていなければ無い。
    pub(crate) fn 開いている重ね合わせ(&self) -> Option<&開いている重ね合わせ> {
        self.開き方.開いている重ね合わせ()
    }

    /// 毎フレーム呼ぶ。重ね合わせを開いていて再生している間だけ、時計を実時間の経過で進める。
    pub(crate) fn 時計を進める(&mut self, 今: Instant) {
        if let Some(開いている) = self.開き方.開いている重ね合わせを書き換える()
        {
            開いている.時計を進める(今);
        }
    }

    /// 毎フレーム、時計を進めた後に呼ぶ。重ね合わせを開いていれば、再生の位置に行ごとに映すコマを載せる。
    pub(crate) fn 行ごとのコマを載せる(&mut self, 今: Instant) {
        if let Some(開いている) = self.開き方.開いている重ね合わせを書き換える()
        {
            開いている.行ごとのコマを載せる(今);
        }
    }

    /// 作業場を控える前に呼ぶ。開いている重ね合わせの再生を止め、行ごとの流し読みを閉じて ffmpeg を止める。重ね合わせと再生の位置と載せたコマは残す。
    pub(crate) fn 控える前に再生を止める(&mut self) {
        if let Some(開いている) = self.開き方.開いている重ね合わせを書き換える()
        {
            開いている.控える前に再生を止めて流し読みを閉じる();
        }
    }

    /// 配線が毎フレームスタックの保存の観測結果を受け取り、前に知らせた理由と違う失敗なら知らせに出す。
    pub(crate) fn スタックの保存の失敗を知らせる(
        &mut self,
        観測結果: スタックの保存の観測結果,
    ) {
        if let Some(文) = self
            .知らせ済みのスタックの保存の失敗
            .覚えて新しい失敗なら知らせの文を返す(観測結果)
        {
            self.知らせ.出す(文);
        }
    }
}
