//! タイムラインの編集に関わる状態の側の部品。塊のドラッグとその動きの語彙(`block_drag.rs`)と、タイムラインに描く塊と行を読む値の型(`drawn_blocks.rs`)を置く。
//! 開いている重ね合わせの項目に触れる操作は `opened/timeline_*.rs` に置く。参照: _doc/設計/同時再生.md 2-6

mod block_drag;
mod drawn_blocks;

pub(crate) use block_drag::{塊のドラッグ, 塊のドラッグの動き};
pub(crate) use drawn_blocks::{
    タイムラインに描く塊, 選んでいる行の番号と空か
};
