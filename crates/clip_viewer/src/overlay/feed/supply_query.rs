//! 重ね合わせの映像の供給への問い合わせ。重ね合わせの状態が開けなかった動画を知らせへ出すときと、重ねる画面が行のテクスチャを描くとき(同時再生.md 5-2 の7)と、
//! 結合試験が動画ごとの台帳と開けない理由を確かめるときに読む口。

use clip_domain::行の番号;

use super::supply::重ね合わせの映像の供給;
use super::video_status::重ね合わせの動画を開けたか;
use crate::video_feed::コマの載せ先;

impl 重ね合わせの映像の供給 {
    /// 開けなかった動画ごとの理由の文(使う動画の表の順)。
    pub(super) fn 開けなかった動画の理由の文(&self) -> Vec<String> {
        self.動画
            .iter()
            .filter_map(重ね合わせの動画を開けたか::開けない理由)
            .map(ToString::to_string)
            .collect()
    }

    /// 行の番号の行のコマの載せ先。同時に重ねられる行の数より後ろの番号なら無い。
    #[cfg_attr(
        not(test),
        expect(
            dead_code,
            reason = "第8段階で重ねる画面に行のテクスチャを描くときに読む(同時再生.md 5-2 の7)。それまでは試験からだけ読む(同時再生.md 6節)"
        )
    )]
    pub(crate) fn 行のコマの載せ先(
        &self, 番号: 行の番号
    ) -> Option<&コマの載せ先> {
        self.行ごとのコマの載せ先.get(番号.番号())
    }

    /// 動画の番号の動画を開けたか。結合試験で、台帳と開けない理由を確かめるために使う。
    #[cfg(test)]
    pub(crate) fn 動画を開けたか(
        &self,
        番号: clip_domain::動画の番号,
    ) -> Option<&重ね合わせの動画を開けたか> {
        self.動画.get(番号.番号())
    }
}
