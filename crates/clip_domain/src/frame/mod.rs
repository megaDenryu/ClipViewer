//! 百分率で表した枠のドラッグによる移動と、四隅のつまみによる大きさの変更。基準(元の動画・重ねる画面)に依らない規則であり、
//! クロップ範囲と映す矩形が共に使う。掴む所の語彙は `grab.rs`、移動量は `movement.rs`、枠と移動と大きさの変更は `rect.rs`、
//! 範囲へ収める前の矩形の工程は `resize.rs`、縦横比を保つ大きさの変更は `aspect.rs` に置く。
//! 移植元 `ClipViewerAppクロップギズモハンドラ.ts` の `クロップ操作を適用する` と `リサイズを適用する` の移植である。

mod aspect;
mod grab;
mod movement;
mod rect;
mod resize;

pub use grab::{四隅のつまみ, 枠の掴む所};
pub use movement::枠の移動量;
pub(crate) use rect::百分率の枠;

/// 枠の幅と高さの最小値(5%)の数値。クロップ範囲と映す矩形で同じである。
pub(crate) const 枠の最小の大きさ: f64 = 5.0;
