//! 配線の層。起動の手順を決め、起動の準備からアプリの状態と操作の適用係を組み立て(`assemble.rs`)、1フレームの手順(状態を進める → 画面を組む →
//! 応答を適用する)の各工程を公開する。egui の描画の呼び出しは、起動の部分(main.rs)と画面の殻(screen_shell.rs)が行う。参照: _doc/設計/画面.md

mod assemble;
mod close;
mod environment;
mod launch_plan;
mod launch_preparation;
mod launch_requests;
mod settings_watch;
mod sound_sender;
mod viewer_settings_save;
mod window;
mod workspace;

#[cfg(test)]
mod close_tests;
#[cfg(test)]
mod launch_plan_tests;
#[cfg(test)]
mod launch_requests_test_support;
#[cfg(test)]
mod launch_requests_tests;
#[cfg(test)]
mod launch_window_tests;
#[cfg(test)]
mod settings_watch_tests;
#[cfg(test)]
mod viewer_settings_save_tests;
#[cfg(test)]
mod workspace_front_tests;
#[cfg(test)]
mod workspace_keys_tests;
#[cfg(test)]
mod workspace_redraw_tests;
#[cfg(test)]
mod workspace_test_frames;
#[cfg(test)]
mod workspace_test_snapshot;
#[cfg(test)]
mod workspace_test_support;
#[cfg(test)]
mod workspace_tests;

pub(crate) use close::ウインドウへの指示;
pub(crate) use environment::起動時の環境;
pub(crate) use launch_plan::起動の手順;
pub(crate) use launch_requests::起動の頼みの届き方;
pub(crate) use window::ウインドウの題名;

use std::time::Instant;

use sengen_egui::ノード;

use crate::command::{主ボタンの様子, 操作の適用係};
use crate::launch::受け取っている受け口;
use crate::state::アプリの状態;
use sound_sender::音の送り手;
use viewer_settings_save::見る側の設定の保存係;
use workspace::{作業場の応答, 前に出ている作業場};

/// クリップビューアーとは、アプリの状態と、応答を状態へ適用する係と、音の送り手と、起動の受け口と、見る側の設定の保存係と、前に出ている作業場の組のことである。
/// 状態は係の処理の対象であって依存ではないため、係の中に入れず並べて持つ。音の送り手は音声出力装置を持ち、毎フレーム指示を渡す。
/// 起動の受け口は、ライブラリの錠を取れた1つ目のアプリだけが持ち、2つ目のアプリから届いた頼みを渡す。
/// アプリの状態と操作の適用係はスタックの作業場のものであり、どちらの作業場が前でも常に持つ(参照: _doc/設計/同時再生.md 3-2)。
pub(crate) struct クリップビューアー {
    状態: アプリの状態,
    適用係: 操作の適用係,
    音の送り手: 音の送り手,
    起動の受け口: Option<受け取っている受け口>,
    設定の保存係: 見る側の設定の保存係,
    前に出ている作業場: 前に出ている作業場,
}

impl クリップビューアー {
    /// 画面を組む前に毎フレーム呼ぶ。時計を進め、先読みを整え、表示するコマをテクスチャへ載せ、音の再生の指示を渡し、
    /// ライブラリの知らせを受け取って自動保存を進め、一覧のサムネイルの仕事を進め、見る側の設定が落ち着いていれば書く。
    /// スタックの作業場の手順は、重ね合わせが前のときも今のまま呼ぶ(再生は止めてあるため時計は進まず、保存は今どおり進む)。
    /// 最後に、重ね合わせが前なら重ね合わせの作業場の時計を進める。
    pub(crate) fn フレームを進める(&mut self, 今: Instant) {
        self.状態.フレームを進める(今);
        self.状態.ライブラリの書き込みを進める(今);
        self.状態.サムネイルを進める();
        self.音の送り手.指示を送る(&mut self.状態, 今);
        self.設定の保存係.進める(&mut self.状態, 今);
        self.前に出ている重ね合わせの作業場のフレームを進める(今);
    }

    /// 今の状態から1フレーム分の画面を組む。前に出ている作業場の画面だけを組む。
    pub(crate) fn 画面(&self) -> ノード<作業場の応答> {
        self.前に出ている作業場の画面を組む()
    }

    /// 描画の間に集めた応答を、描画の後で順に状態へ適用する。その後、主ボタンを押していないのに残った区間の帯とクロップ枠のドラッグを捨て、
    /// 起動の受け口に届いた頼みを適用する。頼みが届いたかを返し、起動の部分はそれをウインドウへの指示へ渡す。
    pub(crate) fn 応答を適用する(
        &mut self,
        応答一覧: Vec<作業場の応答>,
        主ボタン: 主ボタンの様子,
    ) -> 起動の頼みの届き方 {
        for 応答 in 応答一覧 {
            self.作業場の応答を適用する(応答);
        }
        self.状態.取り残されたドラッグを捨てる(主ボタン);
        self.届いた起動の頼みを適用する()
    }
}

impl Drop for クリップビューアー {
    /// アプリを閉じるとき、書けていない並びがあれば保存を頼む(閉じる要求の確かめを経ずに落とされたときのため)。
    /// 頼んだ書き込みは、状態が持つライブラリの接続が落とされるときに終えてから閉じる。見る側の設定は、書いたものと違えばここで書く。
    fn drop(&mut self) {
        self.状態.書けていない並びの保存を頼む();
        self.設定の保存係.終わる前に書く(&mut self.状態);
    }
}
