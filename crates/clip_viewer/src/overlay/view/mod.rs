//! 重ね合わせの作業場の画面。重ね合わせの作業場の状態を借りて1フレーム分の画面を組み、操作を重ね合わせの作業場の応答として発する。
//! 区画は、上にヘッダー、下に再生の操作とタイムライン(重ね合わせを開いている間)、左にサイドバー(重ね合わせを開いている間の置くクリップと置いたクリップと、選んでいる置いたクリップの設定と音)、中央に出力(重ねる画面か、
//! 重ね合わせを開いていない間の案内)を置く。
//! SengenEgui の口だけで組み、素の egui を呼ばない。参照: _doc/設計/同時再生.md 2-2・2-3・2-4・2-6・2-8・3-3

mod conversion;
mod frame_conversion;
mod header;
mod invisible;
mod keys;
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

use std::time::Duration;

use sengen_egui::{
    ノード, パネル, パネルの位置, 一定時間で消える通知の並び, 子, 無し, 縦積み
};

use super::command::重ね合わせの作業場の応答;
use super::state::{
    library_status::重ね合わせのライブラリの様子, placement::置くクリップの一覧,
    今のスタックから並べる状況, 全体の音量, 重ね合わせの作業場の状態,
};

/// 重ね合わせの作業場の画面。今のスタックから並べる状況と、置くクリップの一覧(重ね合わせを開いていなければ無い)と、全体の音量と、
/// スタックの作業場の描き直すまでの時間は、配線がスタックの作業場の状態から求めて渡す。重ね合わせのライブラリの様子は、配線が持つ裏で動く重ね合わせのライブラリから求めて渡す。
/// 重ね合わせが前の間はスタックの作業場の画面を組まないため、スタックの保存待ちの時刻に描き直しを予約する役もこの画面が受け持つ。
/// 左のサイドバーは中央の出力より先に置く(egui は中央のパネルを最後に置き、残りの場所を使わせるため)。
pub(crate) fn 画面(
    状態: &重ね合わせの作業場の状態,
    並べられるか: 今のスタックから並べる状況,
    置くクリップ: Option<&置くクリップの一覧>,
    全体の音量: 全体の音量,
    スタックの作業場の描き直すまでの時間: Option<Duration>,
    ライブラリの様子: 重ね合わせのライブラリの様子<'_>,
) -> ノード<重ね合わせの作業場の応答> {
    let 開いている = 状態.開いている重ね合わせ();
    縦積み(子![
        invisible::ファイルとキーの受け取りと描き直しの予約(
            状態,
            スタックの作業場の描き直すまでの時間
        ),
        パネル(
            "重ね合わせのヘッダー",
            パネルの位置::上,
            子![header::ヘッダー(
                開いている,
                並べられるか,
                ライブラリの様子
            )]
        ),
        パネル(
            "重ね合わせの再生の操作",
            パネルの位置::下,
            子![
                playback_row::再生の操作の行(状態, 全体の音量),
                開いている.map_or_else(無し, timeline::タイムライン),
            ]
        ),
        開いている.map_or_else(無し, |開いている| sidebar::サイドバーのパネル(
            開いている,
            置くクリップ
        )),
        パネル(
            "重ね合わせの出力",
            パネルの位置::中央,
            子![output_area::出力の区画(状態, 並べられるか)]
        ),
        一定時間で消える通知の並び("重ね合わせの作業場の知らせ", 状態.並べる知らせ()),
        rearrange_dialog::並べ直す前の確かめのダイアログ(状態),
    ])
    .into()
}
