//! 変更の見張り。登録済みの保存物の比べる値(スタックのクリップの並び・重ね合わせ)が、最後に保存を頼んだ値から変わり、落ち着いたかを毎フレーム判定する。
//! 参照: _doc/設計/ライブラリ.md 判断5・判断11

use std::borrow::Borrow;
use std::fmt;
use std::time::{Duration, Instant};

/// 値が変わらなくなってから保存を頼むまでの待ち時間。数値入力やボタンを続けて押している間に書き込みを繰り返さないためである。
pub(crate) const 落ち着くまでの時間: Duration = Duration::from_millis(500);

/// 変更の見張りとは、最後に保存を頼んだ値と今の値が違うときに、今の値を最後に見た時刻とともに覚えておくもののことである。
#[derive(Default)]
pub(crate) enum 変更の見張り<比べる値: ?Sized + ToOwned> {
    #[default]
    変わっていない,
    落ち着くのを待っている {
        見た値: 比べる値::Owned,
        見た時刻: Instant,
    },
}

impl<比べる値: ?Sized + ToOwned> fmt::Debug for 変更の見張り<比べる値> {
    /// 見た値は出さず、待っているかと見た時刻だけを出す(長い並びを試験の失敗の表示に並べないため)。
    fn fmt(&self, 書き込み先: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::変わっていない => 書き込み先.write_str("変わっていない"),
            Self::落ち着くのを待っている { 見た時刻, .. } => {
                write!(書き込み先, "落ち着くのを待っている({見た時刻:?})")
            }
        }
    }
}

/// 見た結果とは、変更の見張りが1フレームで今の値を見て出した判定の区別のことである。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum 保存するかの判定 {
    保存しない,
    保存する,
}

impl<比べる値: ?Sized + PartialEq + ToOwned> 変更の見張り<比べる値> {
    /// 落ち着くのを待っているなら、落ち着くまでの残りの時間。変わっていなければ無い。
    pub(crate) fn 落ち着くまでの残り(&self, 今: Instant) -> Option<Duration> {
        match self {
            Self::変わっていない => None,
            Self::落ち着くのを待っている { 見た時刻, .. } => {
                Some((*見た時刻 + 落ち着くまでの時間).saturating_duration_since(今))
            }
        }
    }

    /// 今の値を見る。保存を頼んだ値と違い、落ち着くまでの時間のあいだ変わらなければ保存すると判定する。
    pub(crate) fn 変更を見る(
        &mut self,
        今の値: &比べる値,
        保存を頼んだ値: &比べる値,
        今: Instant,
    ) -> 保存するかの判定 {
        if 今の値 == 保存を頼んだ値 {
            *self = Self::変わっていない;
            return 保存するかの判定::保存しない;
        }
        if let Self::落ち着くのを待っている {
            見た値, 見た時刻
        } = self
            && (*見た値).borrow() == 今の値
        {
            if 今.saturating_duration_since(*見た時刻) < 落ち着くまでの時間 {
                return 保存するかの判定::保存しない;
            }
            *self = Self::変わっていない;
            return 保存するかの判定::保存する;
        }
        *self = Self::落ち着くのを待っている {
            見た値: 今の値.to_owned(),
            見た時刻: 今,
        };
        保存するかの判定::保存しない
    }
}
