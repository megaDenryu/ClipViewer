//! 開いているスタック。今のクリップの並びが、ライブラリに登録していないものか、登録済みのスタックかを持つ。
//! 登録済みのスタックの保存の追跡は `registered_stack.rs` が持つ。
//! 参照: _doc/設計/ライブラリ.md 判断5

use super::registered_stack::登録済みのスタック;
use clip_domain::スタックの識別子;

/// 開いているスタックとは、今のクリップの並びがライブラリに登録していないものか、登録済みのスタックかの区別のことである。
#[derive(Debug, Default)]
#[expect(
    clippy::large_enum_variant,
    reason = "アプリの状態に1つしか持たないため、選択肢の大きさの差は問題にならない"
)]
pub(crate) enum 開いているスタック {
    #[default]
    未登録,
    登録済み(登録済みのスタック),
}

impl 開いているスタック {
    /// 登録済みで、識別子が同じなら、その登録済みのスタック。
    pub(crate) fn 識別子が同じなら(
        &mut self,
        識別子: &スタックの識別子,
    ) -> Option<&mut 登録済みのスタック> {
        match self {
            Self::登録済み(登録済み) if 登録済み.識別子() == 識別子 => {
                Some(登録済み)
            }
            _ => None,
        }
    }

    /// 開いている登録済みのスタックの識別子。未登録なら無い。
    pub(crate) fn 識別子(&self) -> Option<&スタックの識別子> {
        match self {
            Self::登録済み(登録済み) => Some(登録済み.識別子()),
            Self::未登録 => None,
        }
    }
}
