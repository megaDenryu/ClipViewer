//! 置き方の編集に関わる状態の側の部品。置き方の編集(`edit.rs`)と、置き方の手直しの有無(`hand_fix.rs`)と、重ねる画面に描く置いたクリップの選び方(`draw_choice.rs`)と、映す矩形のドラッグ(`drag.rs`)と、ドラッグの動きの語彙(`drag_motion.rs`)と、
//! 置くクリップの出どころ(`place_source.rs`)と、今のスタックのクリップを置くこと(`place_from_stack.rs`)を置く。
//! 画面と操作はこのモジュールの道筋(`state::placement::…`)から読む。開いている重ね合わせの項目に触れる操作は `opened/placement_*.rs` に置く。
//! 参照: _doc/設計/同時再生.md 2-2・2-4

mod drag;
mod drag_motion;
mod draw_choice;
#[cfg(test)]
mod draw_choice_tests;
mod edit;
mod fresh;
mod hand_fix;
mod place_from_stack;
mod place_source;
mod selection;

pub(crate) use super::opened::重ねる枠;
pub(crate) use drag::映す矩形のドラッグ;
pub(crate) use drag_motion::{映す矩形のドラッグの動き, 隅の縦横比の扱い};
pub(crate) use edit::置き方の編集;
pub(crate) use place_from_stack::{
    動画が違うため置けない文, 動画を開いていないため置けない文
};
pub(crate) use place_source::{
    置くクリップ, 置くクリップの一覧, 置くクリップの動画
};
