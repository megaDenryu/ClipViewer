//! 重ね合わせの作業場の層。重ね合わせを作り、直し、再生するための作業場の状態(`state/`)・応答(`command/`)・画面(`view/`)・映像の供給(`feed/`)を持つ。
//! 注意: この下ではスタックの作業場の `crate::state`・`crate::command`・`crate::view` を使わない(`cargo xtask check-overlay-deps` が検査する)。
//! 参照: _doc/設計/同時再生.md 1節(作業場の定義)・3-2・3-3

mod command;
mod feed;
mod placed_id_issuer;
mod state;
mod view;
mod workspace;

#[cfg(test)]
pub(crate) mod arrange_test_support;
#[cfg(test)]
mod arrange_tests;
#[cfg(test)]
mod placed_id_issuer_tests;
#[cfg(test)]
pub(crate) mod test_support;
#[cfg(test)]
mod tests;
#[cfg(test)]
mod with_ffmpeg_feed_tests;

pub(crate) use command::重ね合わせの作業場の応答;
#[cfg(test)]
pub(crate) use command::{落とされたファイルの知らせ, 重ね合わせの操作};
pub(crate) use state::{
    今のスタックの値, 見たスタックの保存, 開いている動画の値
};
#[cfg(test)]
pub(crate) use view::重ね合わせの作業場が受け取るキーの組;
pub(crate) use workspace::重ね合わせの作業場;
