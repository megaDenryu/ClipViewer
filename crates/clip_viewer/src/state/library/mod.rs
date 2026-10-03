//! スタックのライブラリの状態。ライブラリの接続、開いているスタック(未登録か登録済みか)、並べた一覧、開いているダイアログ、アプリの閉じ方を持つ。
//! 参照: _doc/設計/ライブラリ.md

mod connection_wait;
mod dialog;
mod open_stack;
mod registered_stack;
mod save_purpose;

pub(crate) use crate::library_common::list::{
    並べ替え方, 絞り込みの語, 見えている行の範囲, 見せる行,
};
pub(crate) use crate::library_common::save::{
    アプリの閉じ方, ライブラリを使えない理由, 入力中の名前, 見た結果,
};
pub(crate) use connection_wait::{ライブラリの使える様子, ライブラリの接続};
pub(crate) use dialog::{
    ライブラリのダイアログ, 並びを捨てる理由, 登録の後にすること, 開いたときの添え書き,
    関係を終える操作,
};
pub(crate) use open_stack::開いているスタック;
pub(crate) use registered_stack::登録済みのスタック;
pub(crate) use save_purpose::保存の目的;

/// 保存の段階とは、開いている登録済みのスタックの保存の段階のことである。
pub(crate) type 保存の段階<'様子> =
    crate::library_common::save::保存の段階<'様子, ライブラリの操作エラー>;

/// 並べた一覧とは、スタックのライブラリの一覧を並べた一覧のことである。
pub(crate) type 並べた一覧 = crate::library_common::list::並べた一覧<ライブラリの一覧>;

/// 最後に読んだ一覧とは、スタックのライブラリの最後に読んだ一覧のことである。
pub(crate) type 最後に読んだ一覧 =
    crate::library_common::list::最後に読んだ一覧<ライブラリの一覧>;

use std::time::Duration;

use clip_domain::{クリップ, スタックの名前, スタックの識別子};
use clip_library::{ライブラリの一覧, ライブラリの操作エラー};

use crate::thumbnail_feed::一覧のサムネイル;

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
    pub(crate) サムネイル: 一覧のサムネイル<スタックの識別子>,
    pub(crate) 見えている行: Option<見えている行の範囲>,
}

impl ライブラリの状態 {
    /// 接続と一覧のサムネイルから作る。
    pub(crate) fn 接続から作る(
        接続: ライブラリの接続,
        サムネイル: 一覧のサムネイル<スタックの識別子>,
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
