//! 型契約の層。アプリの状態と、状態から導く値と、1フレームごとに状態を進める手順を持つ。
//! 画面は `&アプリの状態` を読んで組み、書き換えは応答(`command`)を通す。参照: _doc/設計/画面.md

mod app_close;
mod band_drag;
mod band_values;
mod band_vocabulary;
mod crop_drag;
mod derived;
mod ffmpeg_status;
mod frame_step;
mod history;
mod key_list;
pub(crate) mod library;
mod library_notice;
mod library_request;
mod library_save;
mod library_thumbnail;
mod notice;
mod open_dialog;
mod output_settings;
mod playback;
mod relation_check;
mod shown_frame;
mod sound_instruction;
mod sound_output;
mod speed_steps;
mod stack_history;
mod stack_replace;
mod startup;
mod video_clue;
mod view_stance;
mod viewer_preference;
mod volume;
mod window;
mod window_vocabulary;
mod workspace_leave;

#[cfg(test)]
mod band_values_tests;
#[cfg(test)]
mod notice_tests;
#[cfg(test)]
mod window_tests;
#[cfg(test)]
mod with_ffmpeg_band_tests;
#[cfg(test)]
mod with_ffmpeg_sound_support;
#[cfg(test)]
mod with_ffmpeg_sound_tests;

pub(crate) use crate::viewer_settings::{左右の向き, 表示サイズ};
pub(crate) use crate::viewer_settings::{速度のつまみの刻み, 速度のつまみの範囲};
pub(crate) use app_close::閉じる要求への答え;
pub(crate) use audio_pcm::音量;
pub(crate) use band_drag::区間の帯のドラッグ;
pub(crate) use band_values::カードの区間の帯に描く値;
pub(crate) use band_vocabulary::{
    区間の帯で掴んだもの, 区間の帯に置いた値, 区間の帯のドラッグの結末, 区間の帯の種類,
};
pub(crate) use crop_drag::クロップ枠の掴み始め;
pub(crate) use ffmpeg_status::{FFmpegの状況, 入力中のフォルダ};
pub(crate) use history::{並びの出どころ, 続けて変える値, 編集の履歴};
pub(crate) use key_list::キーの一覧のダイアログ;
pub(crate) use notice::通知;
pub(crate) use output_settings::{出力の設定, 画面の構え};
pub(crate) use playback::{シークの様子, 再生の状況};
pub(crate) use relation_check::確かめた結果;
pub(crate) use sound_output::音の出力の状況;
pub(crate) use speed_steps::速度を変える向き;
pub(crate) use video_clue::動画の手がかり;
pub(crate) use volume::消音の様子;
pub(crate) use window::ウインドウの状態;
pub(crate) use window_vocabulary::{
    ウインドウの形, ウインドウの様子, ウインドウへの頼み, 全画面の様子,
};

use clip_domain::クリップスタック;

use crate::video_feed::読み込んだ動画;
use crate::viewer_settings::キーの割り当て;

/// アプリの状態とは、画面が読むすべての値の組のことである。
/// 不変条件: クロップ枠の掴み始めと区間の帯のドラッグは並びの中のクリップを指し、開いているスタックは並びの出どころであり、
/// 編集の履歴は並びを置き換えたときからの並びの変化を持つ。このため並びを丸ごと置き換えるときは `並びを置き換える` を通して、
/// これらを揃えて変える。並びの中のクリップを変える操作とほかの項目は、各項目の型がそれぞれの不変条件を持つため、型の外から直接書き換えてよい。
pub(crate) struct アプリの状態 {
    pub(crate) 並び: クリップスタック,
    pub(crate) 再生: 再生の状況,
    pub(crate) 出力: 出力の設定,
    pub(crate) 手がかり: 動画の手がかり,
    pub(crate) 動画: Option<読み込んだ動画>,
    pub(crate) 通知: 通知,
    pub(crate) 掴み始め: Option<クロップ枠の掴み始め>,
    pub(crate) 区間の帯のドラッグ: Option<区間の帯のドラッグ>,
    pub(crate) 履歴: 編集の履歴,
    pub(crate) ffmpegの状況: FFmpegの状況,
    pub(crate) 音の出力: 音の出力の状況,
    pub(crate) ライブラリ: library::ライブラリの状態,
    pub(crate) キーの一覧: キーの一覧のダイアログ,
    pub(crate) キーの割り当て: キーの割り当て,
    pub(crate) ウインドウ: ウインドウの状態,
    pub(crate) 見る側の設定を保存するまでの時間: Option<std::time::Duration>, // 配線の保存係が毎フレーム置き、画面が描き直しの予約に使う
}
