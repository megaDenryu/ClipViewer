//! 左のサイドバーの試験の道具。乙だけの置くクリップの一覧を作ることと、置くクリップの一覧を渡して画面を描き、文字を押して発した応答を集めることを受け持つ。
#![allow(clippy::expect_used)]

use clip_domain::{クリップ名, クリップ識別子};
use eframe::egui;

use super::screen_press_support::試験の全体の音量;
use super::test_support::{押して集める, 描いた文字の範囲, 描いて集める};
use crate::overlay::command::重ね合わせの作業場の応答;
use crate::overlay::state::placement::{
    置くクリップ, 置くクリップの一覧, 置くクリップの動画
};
use crate::overlay::state::今のスタックから並べる状況;
use crate::overlay::workspace::重ね合わせの作業場;

pub(super) fn 乙だけの置くクリップの一覧を作る(
    動画: 置くクリップの動画,
) -> 置くクリップの一覧 {
    置くクリップの一覧 {
        並び: vec![置くクリップ {
            識別子: クリップ識別子::文字列から作成する("乙".to_string()).expect("識別子"),
            名前: クリップ名::作成する("乙".to_string()),
        }],
        動画,
    }
}

/// 置くクリップの一覧を渡して作業場の画面を1回描いてから、その文字の真ん中を押して発した応答を返す。描いていなければ無い。
pub(super) fn サイドバーを描いて文字を押す(
    作業場: &重ね合わせの作業場,
    一覧: &置くクリップの一覧,
    文字: &str,
) -> Option<Vec<重ね合わせの作業場の応答>> {
    let 画面描画の共有状態 = egui::Context::default();
    let 木 = || {
        super::画面(
            作業場.状態(),
            今のスタックから並べる状況::並べられる,
            Some(一覧),
            試験の全体の音量,
            None,
        )
    };
    let (_, 出力) = 描いて集める(&画面描画の共有状態, Vec::new(), &木);
    let 位置 = 描いた文字の範囲(&出力, 文字)?.center();
    Some(押して集める(&画面描画の共有状態, 位置, &木))
}
