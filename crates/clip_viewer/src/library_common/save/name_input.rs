//! 入力中の名前。名前を付けて登録するダイアログ(スタック・重ね合わせ)と名前を変えるダイアログの入力欄に、今書かれている文字列を持つ。

use clip_domain::{スタックの名前, 重ね合わせの名前};

/// 入力から作る名前とは、入力欄の文字列から、前後の空白を除いて空でない名前を作れる型(スタックの名前・重ね合わせの名前)の性質のことである。
pub(crate) trait 入力から作る名前: Sized {
    fn 入力から作る(文字列: String) -> Option<Self>;
}

impl 入力から作る名前 for スタックの名前 {
    fn 入力から作る(文字列: String) -> Option<Self> {
        Self::作成する(文字列).ok()
    }
}

impl 入力から作る名前 for 重ね合わせの名前 {
    fn 入力から作る(文字列: String) -> Option<Self> {
        Self::作成する(文字列).ok()
    }
}

/// 入力中の名前とは、ダイアログの名前の入力欄に今書かれている文字列のことである。決定したときに名前として検査する。
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct 入力中の名前(String);

impl 入力中の名前 {
    pub(crate) fn 作成する(文字列: String) -> Self {
        Self(文字列)
    }

    /// 入力欄へ出す文字列。
    pub(crate) fn 文字列(&self) -> &str {
        &self.0
    }

    /// 名前として読む。前後の空白だけなら無い。
    pub(crate) fn 名前として読む<名前: 入力から作る名前>(&self) -> Option<名前> {
        名前::入力から作る(self.0.clone())
    }
}
