//! スタックの保存の目的。登録と取り込みは、書き込みを終えた知らせを受けてから通知するために、接続の控え(`library_common::save::知らせる保存の目的の覚え`)へ覚えておく。

use clip_domain::スタックの名前;

use crate::library_common::save::保存の目的の区別;

/// 保存の目的とは、スタックの保存を頼んだ理由の区別のことである。登録と取り込みは、書き込みを終えた知らせを受けてから利用者へ知らせる。
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum 保存の目的 {
    自動保存,
    登録(スタックの名前),
    取り込み(スタックの名前),
}

impl 保存の目的の区別 for 保存の目的 {
    fn 自動保存() -> Self {
        Self::自動保存
    }

    fn 自動保存か(&self) -> bool {
        *self == Self::自動保存
    }
}
