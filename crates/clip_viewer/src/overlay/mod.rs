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
mod library_open_tests;
#[cfg(test)]
mod library_read_only_tests;
#[cfg(test)]
mod library_rearrange_tests;
#[cfg(test)]
mod library_save_tests;
#[cfg(test)]
pub(crate) mod library_test_ops;
#[cfg(test)]
pub(crate) mod library_test_support;
#[cfg(test)]
mod place_tests;
#[cfg(test)]
mod placed_id_issuer_tests;
#[cfg(test)]
mod placement_drag_aspect_tests;
#[cfg(test)]
mod placement_drag_tests;
#[cfg(test)]
pub(crate) mod placement_test_support;
#[cfg(test)]
mod placement_tests;
#[cfg(test)]
mod sound_notice_tests;
#[cfg(test)]
pub(crate) mod sound_test_support;
#[cfg(test)]
mod sound_tests;
#[cfg(test)]
pub(crate) mod test_support;
#[cfg(test)]
mod tests;
#[cfg(test)]
mod timeline_continuation_sound_tests;
#[cfg(test)]
mod timeline_drag_discard_tests;
#[cfg(test)]
mod timeline_drag_tests;
#[cfg(test)]
mod timeline_rows_tests;
#[cfg(test)]
mod timeline_seek_tests;
#[cfg(test)]
mod timeline_select_tests;
#[cfg(test)]
pub(crate) mod timeline_test_support;
#[cfg(test)]
mod with_ffmpeg_continuation_support;
#[cfg(test)]
mod with_ffmpeg_continuation_tests;
#[cfg(test)]
mod with_ffmpeg_feed_tests;
#[cfg(test)]
mod with_ffmpeg_place_tests;
#[cfg(test)]
mod with_ffmpeg_seek_drag_tests;
#[cfg(test)]
mod with_ffmpeg_seek_sound_tests;
#[cfg(test)]
mod with_ffmpeg_seek_tests;
#[cfg(test)]
mod with_ffmpeg_sound_tests;

#[cfg(test)]
pub(crate) use command::{置き方の操作, 落とされたファイルの知らせ};
pub(crate) use command::{
    重ね合わせのライブラリの操作, 重ね合わせの作業場の応答, 重ね合わせの操作,
};
pub(crate) use state::library::重ね合わせのライブラリの状態;
pub(crate) use state::{
    スタックの保存の観測結果, 今のスタックの値, 全体の音量, 重ね合わせの音の出力の状況,
    開いている動画の値,
};
#[cfg(test)]
pub(crate) use view::重ね合わせの作業場が受け取るキーの組;
pub(crate) use workspace::{
    スタックの作業場から借りるもの, 重ね合わせの作業場
};
