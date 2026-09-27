//! スタックのライブラリの状態。ライブラリの接続、開いているスタック(未登録か登録済みか)、並べた一覧、開いているダイアログ、アプリの閉じ方を持つ。
//! 参照: _doc/設計/ライブラリ.md

mod change_watch;
mod connection;
mod connection_wait;
mod dialog;
mod list_view;
mod open_stack;
mod registered_stack;
mod save_failure;
mod save_purpose;
mod save_retry;
mod save_status;
mod shown_list;
mod shown_rows;
mod usability;
mod visible_rows;

#[cfg(test)]
mod change_watch_tests;
#[cfg(test)]
mod list_view_tests;

pub(crate) use change_watch::見た結果;
pub(crate) use connection::{ライブラリの接続, ライブラリを使えない理由};
pub(crate) use dialog::{
    ライブラリのダイアログ, 並びを捨てる理由, 入力中の名前, 登録の後にすること,
    開いたときの添え書き, 関係を終える操作,
};
pub(crate) use list_view::{並べ替え方, 絞り込みの語, 見せる行};
pub(crate) use open_stack::開いているスタック;
pub(crate) use registered_stack::登録済みのスタック;
pub(crate) use save_purpose::保存の目的;
pub(crate) use save_status::保存の段階;
pub(crate) use shown_list::{並べた一覧, 最後に読んだ一覧};
pub(crate) use usability::ライブラリの使える様子;
pub(crate) use visible_rows::見えている行の範囲;

use std::time::Duration;

use clip_domain::{クリップ, スタックの名前, スタックの識別子};

use crate::thumbnail_feed::一覧のサムネイル;

/// アプリの閉じ方とは、ウインドウを閉じる要求を受けたときに保存を確かめてから閉じるか、利用者が変更を捨てると決めたので確かめずに閉じるかの区別のことである。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum アプリの閉じ方 {
    #[default]
    確かめてから閉じる,
    確かめずに閉じる,
}

/// ライブラリの状態とは、スタックのライブラリについて画面が読む値と、ライブラリの接続と、一覧のサムネイルと、前のフレームで一覧に見えていた行の組のことである。
/// `見えている行` は、画面が発した応答の適用で置き、次のフレームの `サムネイルを進める` が取り出す(画面は状態を書き換えないため)。
/// 項目どうしに不変条件は無く、各項目の型がそれぞれの不変条件を持つ。
/// `次に並びを見るまでの時間` は、毎フレームの `ライブラリの書き込みを進める` がフレームの時刻から決め、画面が描き直しの予約に使う
/// (画面は時計を読まないため)。
pub(crate) struct ライブラリの状態 {
    pub(crate) 接続: ライブラリの接続,
    pub(crate) 開いている: 開いているスタック,
    pub(crate) 一覧: 並べた一覧,
    pub(crate) ダイアログ: ライブラリのダイアログ,
    pub(crate) 閉じ方: アプリの閉じ方,
    pub(crate) 次に並びを見るまでの時間: Option<Duration>,
    pub(crate) サムネイル: 一覧のサムネイル,
    pub(crate) 見えている行: Option<見えている行の範囲>,
}

impl ライブラリの状態 {
    /// 接続と一覧のサムネイルから作る。
    pub(crate) fn 接続から作る(
        接続: ライブラリの接続, サムネイル: 一覧のサムネイル
    ) -> Self {
        Self {
            接続,
            開いている: 開いているスタック::default(),
            一覧: 並べた一覧::default(),
            ダイアログ: ライブラリのダイアログ::default(),
            閉じ方: アプリの閉じ方::default(),
            次に並びを見るまでの時間: None,
            サムネイル,
            見えている行: None,
        }
    }

    /// 登録済みのスタックの並びがファイルに書けていないか。
    pub(crate) fn 保存されていないか(&self, 今の並び: &[クリップ]) -> bool {
        match &self.開いている {
            開いているスタック::登録済み(登録済み) => {
                登録済み.保存の様子().保存されていないか(今の並び)
            }
            開いているスタック::未登録 => false,
        }
    }

    /// 識別子のスタックの名前。最後に読んだ一覧から探し、無ければ開いているスタックの名前を使う。
    pub(crate) fn 一覧の名前(
        &self,
        識別子: &スタックの識別子,
    ) -> Option<スタックの名前> {
        self.一覧.名前(識別子).or_else(|| match &self.開いている {
            開いているスタック::登録済み(登録済み) if 登録済み.識別子() == 識別子 => {
                Some(登録済み.名前().clone())
            }
            _ => None,
        })
    }
}
