//! 変更の見張り。登録済みのスタックのクリップの並びが、最後に保存を頼んだ並びから変わり、落ち着いたかを毎フレーム判定する。
//! 参照: _doc/設計/ライブラリ.md 判断5

use std::time::{Duration, Instant};

use clip_domain::クリップ;

/// 並びが変わらなくなってから保存を頼むまでの待ち時間。数値入力やボタンを続けて押している間に書き込みを繰り返さないためである。
pub(crate) const 落ち着くまでの時間: Duration = Duration::from_millis(500);

/// 変更の見張りとは、最後に保存を頼んだ並びと今の並びが違うときに、今の並びを最後に見た時刻とともに覚えておくもののことである。
#[derive(Debug, Clone, Default)]
pub(crate) enum 変更の見張り {
    #[default]
    変わっていない,
    落ち着くのを待っている {
        見た並び: Vec<クリップ>,
        見た時刻: Instant,
    },
}

/// 見た結果とは、変更の見張りが1フレームで今の並びを見て出した判定の区別のことである。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum 見た結果 {
    保存しない,
    保存する,
}

impl 変更の見張り {
    /// 落ち着くのを待っているなら、落ち着くまでの残りの時間。変わっていなければ無い。
    pub(crate) fn 落ち着くまでの残り(&self, 今: Instant) -> Option<Duration> {
        match self {
            Self::変わっていない => None,
            Self::落ち着くのを待っている { 見た時刻, .. } => {
                Some((*見た時刻 + 落ち着くまでの時間).saturating_duration_since(今))
            }
        }
    }

    /// 今の並びを見る。保存を頼んだ並びと違い、落ち着くまでの時間のあいだ変わらなければ保存すると判定する。
    pub(crate) fn 見る(
        &mut self,
        今の並び: &[クリップ],
        保存を頼んだ並び: &[クリップ],
        今: Instant,
    ) -> 見た結果 {
        if 今の並び == 保存を頼んだ並び {
            *self = Self::変わっていない;
            return 見た結果::保存しない;
        }
        if let Self::落ち着くのを待っている {
            見た並び, 見た時刻
        } = self
            && 見た並び.as_slice() == 今の並び
        {
            if 今.saturating_duration_since(*見た時刻) < 落ち着くまでの時間 {
                return 見た結果::保存しない;
            }
            *self = Self::変わっていない;
            return 見た結果::保存する;
        }
        *self = Self::落ち着くのを待っている {
            見た並び: 今の並び.to_vec(),
            見た時刻: 今,
        };
        見た結果::保存しない
    }
}
