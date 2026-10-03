//! 重ね合わせの作業場の画面。重ね合わせの作業場の状態を借りて1フレーム分の画面を組み、操作を重ね合わせの作業場の応答として発する。
//! 区画は、上にヘッダー、下に再生の操作とタイムライン(重ね合わせを開いている間)、左にサイドバー(重ね合わせを開いている間の置くクリップと置いたクリップと、選んでいる置いたクリップの設定と音)、中央に出力(重ねる画面か、
//! 重ね合わせを開いていない間の案内)を置く。
//! SengenEgui の口だけで組み、素の egui を呼ばない。参照: _doc/設計/同時再生.md 2-2・2-3・2-4・2-6・2-8・3-3

mod conversion;
mod frame_conversion;
mod header;
mod invisible;
mod keys;
mod layout;
mod not_open;
mod output_area;
mod placed_sound;
mod playback_row;
mod rearrange_dialog;
mod screen;
mod sidebar;
mod sidebar_placed;
mod timeline;
mod timeline_conversion;
mod timeline_row_actions;

#[cfg(test)]
mod frame_conversion_tests;
#[cfg(test)]
mod frame_test_support;
#[cfg(test)]
mod frame_tests;
#[cfg(test)]
mod header_tests;
#[cfg(test)]
mod not_open_tests;
#[cfg(test)]
mod playback_row_tests;
#[cfg(test)]
mod rearrange_dialog_tests;
#[cfg(test)]
mod screen_press_support;
#[cfg(test)]
mod screen_tests;
#[cfg(test)]
mod sidebar_test_support;
#[cfg(test)]
mod sidebar_tests;
#[cfg(test)]
mod sidebar_width_tests;
#[cfg(test)]
mod sound_tests;
#[cfg(test)]
mod test_support;
#[cfg(test)]
mod timeline_conversion_tests;
#[cfg(test)]
mod timeline_tests;

#[cfg(test)]
pub(crate) use keys::重ね合わせの作業場が受け取るキーの組;

pub(crate) use layout::画面;
