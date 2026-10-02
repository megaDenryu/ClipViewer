//! 重ね合わせの作業場の層。重ね合わせを作り、直し、再生するための作業場の状態(`state/`)・操作の適用(`command/`)・画面(`view/`)を持つ。
//! 作業場とは、利用者が1つの目的で使う画面の全体であり、自分の状態・操作の適用・キーの表・画面の木・時計を持つもののことである。
//! 注意: このモジュールの下では、スタックの作業場の `crate::state`・`crate::command`・`crate::view` を使わない。スタックの作業場から読むものは、
//! 配線(`app/workspace.rs`)が値として渡す。`cargo xtask check-overlay-deps` がこの決まりを検査する。
//! 参照: _doc/設計/同時再生.md 3-2・3-3

mod command;
mod state;
mod view;
mod workspace;

#[cfg(test)]
mod tests;

pub(crate) use command::{操作の後の作業場, 重ね合わせの操作};
pub(crate) use workspace::重ね合わせの作業場;
