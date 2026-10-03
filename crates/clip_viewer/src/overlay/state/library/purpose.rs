//! 重ね合わせの保存の目的。登録は、書き込みを終えた知らせを受けてから知らせるために、接続の控えへ覚えておく。

use clip_domain::重ね合わせの名前;

use crate::library_common::save::保存の目的の区別;

/// 重ね合わせの保存の目的とは、重ね合わせの保存を頼んだ理由の区別のことである。登録は、書き込みを終えた知らせを受けてから利用者へ知らせる。
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum 重ね合わせの保存の目的 {
    自動保存,
    登録(重ね合わせの名前),
}

impl 保存の目的の区別 for 重ね合わせの保存の目的 {
    fn 自動保存() -> Self {
        Self::自動保存
    }

    fn 自動保存か(&self) -> bool {
        *self == Self::自動保存
    }
}
